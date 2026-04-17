//! Entry points for static spec analysis.

use hyle_ca_interface::Blueprint;

use crate::{Diagnostic, Subject};

use super::{
    neighborhoods::analyze_neighborhoods, rules::analyze_rules, SpecAnalysis, SpecSummary,
};

/// Analyze a declarative schema and return shared diagnostics and summaries.
pub fn analyze_spec(blueprint: &Blueprint) -> SpecAnalysis {
    let mut usage_counts = vec![0usize; blueprint.neighborhoods().len()];
    for rule in blueprint.rules() {
        if let Some(count) = usage_counts.get_mut(rule.neighborhood.index()) {
            *count += 1;
        }
    }

    let neighborhoods = analyze_neighborhoods(
        blueprint
            .neighborhoods()
            .iter()
            .enumerate()
            .map(|(index, neighborhood)| (index, neighborhood.name(), *neighborhood)),
        &usage_counts,
    );
    let rules = analyze_rules(blueprint);

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
        .map(|neighborhood| neighborhood.spec.radius().get())
        .max()
        .unwrap_or(0);

    SpecAnalysis {
        summary: SpecSummary {
            materials: blueprint.materials().to_vec(),
            semantics: blueprint.semantics(),
            attributes: blueprint.attributes().to_vec(),
            rule_count: blueprint.rules().len(),
            neighborhood_count: neighborhoods.len(),
            attribute_count: blueprint.attributes().len(),
            max_radius,
            topology: blueprint.topology(),
        },
        rules,
        neighborhoods,
        diagnostics,
    }
}
