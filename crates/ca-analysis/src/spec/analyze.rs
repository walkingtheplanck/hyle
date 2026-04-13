//! Entry points for static spec analysis.

use hyle_ca_interface::{BlueprintSpec, Cell};

use crate::{Diagnostic, Subject};

use super::{
    neighborhoods::analyze_neighborhoods, rules::analyze_rules, SpecAnalysis, SpecSummary,
};

/// Analyze a declarative blueprint spec and return shared diagnostics and summaries.
pub fn analyze_spec<C: Cell + Eq>(spec: &BlueprintSpec<C>) -> SpecAnalysis<C> {
    let mut usage_counts = vec![0usize; spec.neighborhoods().len()];
    for rule in spec.rules() {
        if let Some(count) = usage_counts.get_mut(rule.neighborhood) {
            *count += 1;
        }
    }

    let neighborhoods = analyze_neighborhoods(
        spec.neighborhoods()
            .iter()
            .enumerate()
            .map(|(index, neighborhood)| (index, neighborhood.name.clone(), neighborhood.spec)),
        &usage_counts,
    );
    let rules = analyze_rules(spec);

    let mut diagnostics = Vec::new();
    for neighborhood in &neighborhoods {
        if neighborhood.used_by_rules == 0 {
            diagnostics.push(Diagnostic::warning(
                "unused_neighborhood",
                format!(
                    "named neighborhood '{}' is never used by any rule",
                    neighborhood.name
                ),
                Subject::Neighborhood {
                    index: neighborhood.index,
                },
            ));
        }
    }

    let max_radius = neighborhoods
        .iter()
        .map(|neighborhood| neighborhood.spec.radius())
        .max()
        .unwrap_or(0);

    SpecAnalysis {
        summary: SpecSummary {
            semantics: spec.semantics(),
            rule_count: spec.rules().len(),
            neighborhood_count: neighborhoods.len(),
            max_radius,
            topology: spec.topology(),
        },
        rules,
        neighborhoods,
        diagnostics,
    }
}
