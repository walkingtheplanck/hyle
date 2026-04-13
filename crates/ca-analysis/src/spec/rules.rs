//! Rule-derived analysis helpers.

use hyle_ca_interface::{BlueprintSpec, CellState};

use crate::{Diagnostic, Subject};

use super::RuleAnalysis;

pub(crate) fn analyze_rules<C: CellState>(spec: &BlueprintSpec<C>) -> Vec<RuleAnalysis<C>> {
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
