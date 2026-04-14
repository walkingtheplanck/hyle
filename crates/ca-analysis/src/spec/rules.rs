//! Rule-derived analysis helpers.

use hyle_ca_interface::semantics::{max_weighted_sum, neighbor_count};
use hyle_ca_interface::{Blueprint, CellModel, Condition, CountComparison, WeightComparison};

use crate::{Diagnostic, Subject};

use super::RuleAnalysis;

pub(crate) fn analyze_rules<C: CellModel>(spec: &Blueprint<C>) -> Vec<RuleAnalysis<C>> {
    let rules = spec.rules();
    let mut analyses = Vec::with_capacity(rules.len());

    for (index, rule) in rules.iter().enumerate() {
        let shadowed_by = rules[..index]
            .iter()
            .position(|earlier| earlier.when == rule.when && earlier.condition.is_none());
        let duplicate_of = rules[..index].iter().position(|earlier| {
            earlier.when == rule.when
                && earlier.neighborhood == rule.neighborhood
                && earlier.condition == rule.condition
                && earlier.effect == rule.effect
        });

        let mut diagnostics = Vec::new();

        if let Some(earlier) = duplicate_of {
            diagnostics.push(Diagnostic::warning(
                "duplicate_rule",
                format!("rule duplicates rule {earlier} exactly"),
                Subject::Rule { index },
            ));
        }

        if let Some(earlier) = shadowed_by {
            diagnostics.push(Diagnostic::warning(
                "shadowed_rule",
                format!("rule can never match because rule {earlier} already matches this state unconditionally"),
                Subject::Rule { index },
            ));
        }

        if let Some(condition) = &rule.condition {
            collect_condition_diagnostics(
                condition,
                spec.neighborhoods()[rule.neighborhood].spec,
                index,
                &mut diagnostics,
            );
        }

        analyses.push(RuleAnalysis {
            index,
            when: rule.when,
            neighborhood: rule.neighborhood,
            conditional: rule.condition.is_some(),
            effect: rule.effect,
            shadowed_by,
            duplicate_of,
            diagnostics,
        });
    }

    analyses
}

fn collect_condition_diagnostics<C: CellModel>(
    condition: &Condition<C>,
    neighborhood: hyle_ca_interface::NeighborhoodSpec,
    rule_index: usize,
    diagnostics: &mut Vec<Diagnostic>,
) {
    match condition {
        Condition::NeighborCount { comparison, .. } => {
            let max_count = neighbor_count(neighborhood);
            if count_never_matches(*comparison, max_count) {
                diagnostics.push(Diagnostic::warning(
                    "impossible_neighbor_count",
                    format!(
                        "rule cannot match because its neighbor-count predicate exceeds the maximum count {max_count} for this neighborhood"
                    ),
                    Subject::Rule { index: rule_index },
                ));
            }
        }
        Condition::NeighborWeightedSum { comparison, .. } => {
            let max_weight = max_weighted_sum(neighborhood);
            if weighted_never_matches(*comparison, max_weight) {
                diagnostics.push(Diagnostic::warning(
                    "impossible_weighted_sum",
                    format!(
                        "rule cannot match because its weighted-sum predicate exceeds the maximum weight {max_weight} for this neighborhood"
                    ),
                    Subject::Rule { index: rule_index },
                ));
            }
        }
        Condition::RandomChance { .. } => {}
        Condition::And(conditions) | Condition::Or(conditions) => {
            for condition in conditions {
                collect_condition_diagnostics(condition, neighborhood, rule_index, diagnostics);
            }
        }
        Condition::Not(condition) => {
            collect_condition_diagnostics(condition, neighborhood, rule_index, diagnostics);
        }
    }
}

fn count_never_matches(comparison: CountComparison, max_count: u32) -> bool {
    match comparison {
        CountComparison::Eq(expected) => expected > max_count,
        CountComparison::InRange { min, .. } => min > max_count,
        CountComparison::NotInRange { .. } => false,
        CountComparison::AtLeast(expected) => expected > max_count,
        CountComparison::AtMost(_) => false,
    }
}

fn weighted_never_matches(comparison: WeightComparison, max_weight: u64) -> bool {
    match comparison {
        WeightComparison::Eq(expected) => expected.units() > max_weight,
        WeightComparison::InRange { min, .. } => min.units() > max_weight,
        WeightComparison::NotInRange { .. } => false,
        WeightComparison::AtLeast(expected) => expected.units() > max_weight,
        WeightComparison::AtMost(_) => false,
    }
}
