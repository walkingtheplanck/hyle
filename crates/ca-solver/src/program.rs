//! Compiled solver programs derived from interpreted blueprints.

use hyle_ca_interface::semantics::{cell_rng, ResolvedBlueprint};
use hyle_ca_interface::{
    AttributeAssignment, AttributeComparison, AttributeValue, Cell, CellModel, Condition,
    CountComparison, RuleEffect, WeightComparison,
};

use crate::Neighborhood;

pub(crate) struct CompiledProgram<C: Cell + Eq> {
    rules: Vec<CompiledRule<C>>,
}

pub(crate) struct Evaluation<C: Cell + Eq> {
    pub(crate) effect: RuleEffect<C>,
    pub(crate) attribute_updates: Vec<CompiledAttributeAssignment>,
}

impl<C: Cell + Eq> CompiledProgram<C> {
    pub(crate) fn from_blueprint(blueprint: &ResolvedBlueprint<C>) -> Self
    where
        C: CellModel,
    {
        let neighborhoods = blueprint.neighborhoods();
        let rules = blueprint
            .rules()
            .iter()
            .map(|rule| {
                let neighborhood = neighborhoods[rule.neighborhood].neighborhood();
                CompiledRule {
                    when: rule.when,
                    condition: rule
                        .condition
                        .as_ref()
                        .map(|condition| compile_condition(condition, blueprint)),
                    attribute_updates: rule
                        .attribute_updates
                        .iter()
                        .map(|update| compile_attribute_assignment(update, blueprint))
                        .collect(),
                    effect: rule.effect,
                    neighborhood: Neighborhood::new(neighborhood.samples()),
                }
            })
            .collect();

        Self { rules }
    }

    pub(crate) fn evaluate(
        &mut self,
        center: C,
        pos: [i32; 3],
        step: u32,
        seed: u64,
        sample: impl Fn(i32, i32, i32) -> C,
        read_attribute: impl Fn(usize) -> AttributeValue,
    ) -> Option<Evaluation<C>> {
        for rule in &mut self.rules {
            if let Some(evaluation) =
                rule.evaluate(center, pos, step, seed, &sample, &read_attribute)
            {
                return Some(evaluation);
            }
        }

        None
    }
}

struct CompiledRule<C: Cell + Eq> {
    when: C,
    condition: Option<CompiledCondition<C>>,
    attribute_updates: Vec<CompiledAttributeAssignment>,
    effect: RuleEffect<C>,
    neighborhood: Neighborhood<C>,
}

#[derive(Clone)]
pub(crate) struct CompiledAttributeAssignment {
    pub(crate) attribute: usize,
    pub(crate) value: AttributeValue,
}

#[derive(Clone)]
enum CompiledCondition<C: Cell + Eq> {
    NeighborCount {
        state: C,
        comparison: CountComparison,
    },
    NeighborWeightedSum {
        state: C,
        comparison: WeightComparison,
    },
    RandomChance {
        stream: u32,
        one_in: u32,
    },
    Attribute {
        attribute: usize,
        comparison: AttributeComparison,
    },
    And(Vec<CompiledCondition<C>>),
    Or(Vec<CompiledCondition<C>>),
    Not(Box<CompiledCondition<C>>),
}

impl<C: Cell + Eq> CompiledRule<C> {
    fn evaluate(
        &mut self,
        center: C,
        pos: [i32; 3],
        step: u32,
        seed: u64,
        sample: &impl Fn(i32, i32, i32) -> C,
        read_attribute: &impl Fn(usize) -> AttributeValue,
    ) -> Option<Evaluation<C>> {
        if center != self.when {
            return None;
        }

        if let Some(condition) = &self.condition {
            self.neighborhood.fill(center, pos, sample);
            if !evaluate_condition(
                condition,
                &self.neighborhood,
                pos,
                step,
                seed,
                read_attribute,
            ) {
                return None;
            }
        }

        Some(Evaluation {
            effect: self.effect,
            attribute_updates: self.attribute_updates.clone(),
        })
    }
}

fn compile_condition<C: Cell + Eq + CellModel>(
    condition: &Condition<C>,
    blueprint: &ResolvedBlueprint<C>,
) -> CompiledCondition<C> {
    match condition {
        Condition::NeighborCount { state, comparison } => CompiledCondition::NeighborCount {
            state: *state,
            comparison: *comparison,
        },
        Condition::NeighborWeightedSum { state, comparison } => {
            CompiledCondition::NeighborWeightedSum {
                state: *state,
                comparison: *comparison,
            }
        }
        Condition::RandomChance { stream, one_in } => CompiledCondition::RandomChance {
            stream: *stream,
            one_in: *one_in,
        },
        Condition::Attribute {
            attribute,
            comparison,
        } => CompiledCondition::Attribute {
            attribute: blueprint
                .attributes()
                .iter()
                .position(|candidate| candidate.name == attribute.as_str())
                .expect("validated attribute conditions must resolve during compilation"),
            comparison: *comparison,
        },
        Condition::And(conditions) => CompiledCondition::And(
            conditions
                .iter()
                .map(|condition| compile_condition(condition, blueprint))
                .collect(),
        ),
        Condition::Or(conditions) => CompiledCondition::Or(
            conditions
                .iter()
                .map(|condition| compile_condition(condition, blueprint))
                .collect(),
        ),
        Condition::Not(condition) => {
            CompiledCondition::Not(Box::new(compile_condition(condition, blueprint)))
        }
    }
}

fn compile_attribute_assignment<C: Cell + Eq + CellModel>(
    update: &AttributeAssignment,
    blueprint: &ResolvedBlueprint<C>,
) -> CompiledAttributeAssignment {
    CompiledAttributeAssignment {
        attribute: blueprint
            .attributes()
            .iter()
            .position(|candidate| candidate.name == update.attribute.as_str())
            .expect("validated attribute updates must resolve during compilation"),
        value: update.value,
    }
}

fn evaluate_condition<C: Cell + Eq>(
    condition: &CompiledCondition<C>,
    neighborhood: &Neighborhood<C>,
    pos: [i32; 3],
    step: u32,
    seed: u64,
    read_attribute: &impl Fn(usize) -> AttributeValue,
) -> bool {
    match condition {
        CompiledCondition::NeighborCount { state, comparison } => {
            let count = neighborhood.count(|entry| entry.cell == *state);
            compare_count(count, *comparison)
        }
        CompiledCondition::NeighborWeightedSum { state, comparison } => {
            let weight = neighborhood.weighted_sum(|entry| entry.cell == *state);
            compare_weight(weight, *comparison)
        }
        CompiledCondition::RandomChance { stream, one_in } => {
            cell_rng(pos, step, *stream, seed).chance(*one_in)
        }
        CompiledCondition::Attribute {
            attribute,
            comparison,
        } => compare_attribute(read_attribute(*attribute), *comparison),
        CompiledCondition::And(conditions) => conditions.iter().all(|condition| {
            evaluate_condition(condition, neighborhood, pos, step, seed, read_attribute)
        }),
        CompiledCondition::Or(conditions) => conditions.iter().any(|condition| {
            evaluate_condition(condition, neighborhood, pos, step, seed, read_attribute)
        }),
        CompiledCondition::Not(condition) => {
            !evaluate_condition(condition, neighborhood, pos, step, seed, read_attribute)
        }
    }
}

fn compare_count(count: u32, comparison: CountComparison) -> bool {
    match comparison {
        CountComparison::Eq(expected) => count == expected,
        CountComparison::InRange { min, max } => (min..=max).contains(&count),
        CountComparison::NotInRange { min, max } => !(min..=max).contains(&count),
        CountComparison::AtLeast(expected) => count >= expected,
        CountComparison::AtMost(expected) => count <= expected,
    }
}

fn compare_weight(weight: u64, comparison: WeightComparison) -> bool {
    match comparison {
        WeightComparison::Eq(expected) => weight == expected.units(),
        WeightComparison::InRange { min, max } => (min.units()..=max.units()).contains(&weight),
        WeightComparison::NotInRange { min, max } => !(min.units()..=max.units()).contains(&weight),
        WeightComparison::AtLeast(expected) => weight >= expected.units(),
        WeightComparison::AtMost(expected) => weight <= expected.units(),
    }
}

fn compare_attribute(value: AttributeValue, comparison: AttributeComparison) -> bool {
    match comparison {
        AttributeComparison::Eq(expected) => value == expected,
        AttributeComparison::InRange { min, max } => {
            ordered_attribute_compare(value, min, max, true)
        }
        AttributeComparison::NotInRange { min, max } => {
            !ordered_attribute_compare(value, min, max, true)
        }
        AttributeComparison::AtLeast(expected) => attribute_ge(value, expected),
        AttributeComparison::AtMost(expected) => attribute_le(value, expected),
    }
}

fn ordered_attribute_compare(
    value: AttributeValue,
    min: AttributeValue,
    max: AttributeValue,
    inclusive: bool,
) -> bool {
    if inclusive {
        attribute_ge(value, min) && attribute_le(value, max)
    } else {
        false
    }
}

fn attribute_ge(value: AttributeValue, expected: AttributeValue) -> bool {
    match (value, expected) {
        (AttributeValue::U8(value), AttributeValue::U8(expected)) => value >= expected,
        (AttributeValue::U16(value), AttributeValue::U16(expected)) => value >= expected,
        (AttributeValue::U32(value), AttributeValue::U32(expected)) => value >= expected,
        (AttributeValue::I8(value), AttributeValue::I8(expected)) => value >= expected,
        (AttributeValue::I16(value), AttributeValue::I16(expected)) => value >= expected,
        (AttributeValue::I32(value), AttributeValue::I32(expected)) => value >= expected,
        _ => panic!("attribute comparison types must match"),
    }
}

fn attribute_le(value: AttributeValue, expected: AttributeValue) -> bool {
    match (value, expected) {
        (AttributeValue::U8(value), AttributeValue::U8(expected)) => value <= expected,
        (AttributeValue::U16(value), AttributeValue::U16(expected)) => value <= expected,
        (AttributeValue::U32(value), AttributeValue::U32(expected)) => value <= expected,
        (AttributeValue::I8(value), AttributeValue::I8(expected)) => value <= expected,
        (AttributeValue::I16(value), AttributeValue::I16(expected)) => value <= expected,
        (AttributeValue::I32(value), AttributeValue::I32(expected)) => value <= expected,
        _ => panic!("attribute comparison types must match"),
    }
}
