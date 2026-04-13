//! Neighborhood-derived analysis helpers.

use hyle_ca_contracts::NeighborhoodSpec;
use hyle_ca_semantics::neighbor_count;

use super::NeighborhoodAnalysis;

pub(crate) fn analyze_neighborhoods(
    names: impl Iterator<Item = (usize, String, NeighborhoodSpec)>,
    usage_counts: &[usize],
) -> Vec<NeighborhoodAnalysis> {
    names
        .map(|(index, name, spec)| NeighborhoodAnalysis {
            index,
            name,
            spec,
            neighbor_count: neighbor_count(spec),
            used_by_rules: usage_counts.get(index).copied().unwrap_or(0),
        })
        .collect()
}
