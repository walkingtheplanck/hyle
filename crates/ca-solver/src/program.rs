//! Compiled solver programs derived from interpreted blueprints.

use hyle_ca_interface::semantics::{cell_rng, ResolvedBlueprint};
use hyle_ca_interface::{
    AttributeAssignment, AttributeComparison, AttributeId, AttributeValue, CountComparison,
    MaterialId, ResolvedCondition, RuleEffect, WeightComparison,
};

use crate::Neighborhood;

pub(crate) struct CompiledProgram {
    rules: Vec<CompiledRule>,
}

pub(crate) struct Evaluation {
    pub(crate) effect: RuleEffect,
    pub(crate) attribute_updates: Vec<AttributeAssignment>,
}

impl CompiledProgram {
    pub(crate) fn from_blueprint(blueprint: &ResolvedBlueprint) -> Self {
        let neighborhoods = blueprint.neighborhoods();
        let rules = blueprint
            .rules()
            .iter()
            .map(|rule| {
                let neighborhood = neighborhoods[rule.neighborhood.index()].neighborhood();
                CompiledRule {
                    when: rule.when,
                    condition: rule.condition.clone().map(compile_condition),
                    attribute_updates: rule.attribute_updates.clone(),
                    effect: rule.effect,
                    neighborhood: Neighborhood::new(neighborhood.samples()),
                }
            })
            .collect();

        Self { rules }
    }

    pub(crate) fn evaluate(
        &mut self,
        center: MaterialId,
        pos: [i32; 3],
        step: u32,
        seed: u64,
        sample: impl Fn(i32, i32, i32) -> MaterialId,
        read_attribute: impl Fn(AttributeId) -> AttributeValue,
    ) -> Option<Evaluation> {
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

struct CompiledRule {
    when: MaterialId,
    condition: Option<CompiledCondition>,
    attribute_updates: Vec<AttributeAssignment>,
    effect: RuleEffect,
    neighborhood: Neighborhood,
}

#[derive(Clone)]
enum CompiledCondition {
    NeighborCount {
        material: MaterialId,
        comparison: CountComparison,
    },
    NeighborWeightedSum {
        material: MaterialId,
        comparison: WeightComparison,
    },
    RandomChance {
        stream: u32,
        one_in: u32,
    },
    Attribute {
        attribute: AttributeId,
        comparison: AttributeComparison,
    },
    And(Vec<CompiledCondition>),
    Or(Vec<CompiledCondition>),
    Not(Box<CompiledCondition>),
}

impl CompiledRule {
    fn evaluate(
        &mut self,
        center: MaterialId,
        pos: [i32; 3],
        step: u32,
        seed: u64,
        sample: &impl Fn(i32, i32, i32) -> MaterialId,
        read_attribute: &impl Fn(AttributeId) -> AttributeValue,
    ) -> Option<Evaluation> {
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

fn compile_condition(condition: ResolvedCondition) -> CompiledCondition {
    match condition {
        ResolvedCondition::NeighborCount {
            material,
            comparison,
        } => CompiledCondition::NeighborCount {
            material,
            comparison,
        },
        ResolvedCondition::NeighborWeightedSum {
            material,
            comparison,
        } => CompiledCondition::NeighborWeightedSum {
            material,
            comparison,
        },
        ResolvedCondition::RandomChance { stream, one_in } => {
            CompiledCondition::RandomChance { stream, one_in }
        }
        ResolvedCondition::Attribute {
            attribute,
            comparison,
        } => CompiledCondition::Attribute {
            attribute,
            comparison,
        },
        ResolvedCondition::And(conditions) => {
            CompiledCondition::And(conditions.into_iter().map(compile_condition).collect())
        }
        ResolvedCondition::Or(conditions) => {
            CompiledCondition::Or(conditions.into_iter().map(compile_condition).collect())
        }
        ResolvedCondition::Not(condition) => {
            CompiledCondition::Not(Box::new(compile_condition(*condition)))
        }
    }
}

fn evaluate_condition(
    condition: &CompiledCondition,
    neighborhood: &Neighborhood,
    pos: [i32; 3],
    step: u32,
    seed: u64,
    read_attribute: &impl Fn(AttributeId) -> AttributeValue,
) -> bool {
    match condition {
        CompiledCondition::NeighborCount {
            material,
            comparison,
        } => {
            let count = neighborhood.count(|entry| entry.cell == *material);
            compare_count(count, *comparison)
        }
        CompiledCondition::NeighborWeightedSum {
            material,
            comparison,
        } => {
            let weight = neighborhood.weighted_sum(|entry| entry.cell == *material);
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
