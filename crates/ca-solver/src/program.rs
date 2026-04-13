//! Compiled solver programs derived from interpreted blueprints.

use hyle_ca_interface::semantics::Blueprint;
use hyle_ca_interface::{Cell, Condition, CountComparison, RuleEffect};

use crate::Neighborhood;

pub(crate) struct CompiledProgram<C: Cell + Eq> {
    rules: Vec<CompiledRule<C>>,
}

impl<C: Cell + Eq> CompiledProgram<C> {
    pub(crate) fn from_blueprint(blueprint: &Blueprint<C>) -> Self {
        let neighborhoods = blueprint.neighborhoods();
        let rules = blueprint
            .rules()
            .iter()
            .map(|rule| {
                let neighborhood = neighborhoods[rule.neighborhood].neighborhood();
                CompiledRule {
                    when: rule.when,
                    condition: rule.condition.clone(),
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
        sample: impl Fn(i32, i32, i32) -> C,
    ) -> Option<RuleEffect<C>> {
        for rule in &mut self.rules {
            if let Some(effect) = rule.evaluate(center, pos, &sample) {
                return Some(effect);
            }
        }

        None
    }
}

struct CompiledRule<C: Cell + Eq> {
    when: C,
    condition: Option<Condition<C>>,
    effect: RuleEffect<C>,
    neighborhood: Neighborhood<C>,
}

impl<C: Cell + Eq> CompiledRule<C> {
    fn evaluate(
        &mut self,
        center: C,
        pos: [i32; 3],
        sample: &impl Fn(i32, i32, i32) -> C,
    ) -> Option<RuleEffect<C>> {
        if center != self.when {
            return None;
        }

        if let Some(condition) = &self.condition {
            self.neighborhood.fill(center, pos, sample);
            if !evaluate_condition(condition, &self.neighborhood) {
                return None;
            }
        }

        Some(self.effect)
    }
}

fn evaluate_condition<C: Cell + Eq>(
    condition: &Condition<C>,
    neighborhood: &Neighborhood<C>,
) -> bool {
    match condition {
        Condition::NeighborCount { state, comparison } => {
            let count = neighborhood.count(|entry| entry.cell == *state);
            compare_count(count, *comparison)
        }
        Condition::And(conditions) => conditions
            .iter()
            .all(|condition| evaluate_condition(condition, neighborhood)),
        Condition::Or(conditions) => conditions
            .iter()
            .any(|condition| evaluate_condition(condition, neighborhood)),
        Condition::Not(condition) => !evaluate_condition(condition, neighborhood),
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
