//! Public report types returned by spec analysis.

use hyle_ca_interface::{CellState, NeighborhoodSpec, RuleEffect, Semantics, TopologyDescriptor};

use crate::Diagnostic;

/// Top-level summary derived from a blueprint spec.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SpecSummary {
    /// Declared semantics version.
    pub semantics: Semantics,
    /// Total number of rules in declaration order.
    pub rule_count: usize,
    /// Total number of named neighborhoods.
    pub neighborhood_count: usize,
    /// Maximum neighborhood radius used anywhere in the spec.
    pub max_radius: u32,
    /// Declared topology.
    pub topology: TopologyDescriptor,
}

/// Derived information about a single rule.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RuleAnalysis<C: CellState> {
    /// Zero-based rule index.
    pub index: usize,
    /// State matched by the rule.
    pub when: C,
    /// Referenced neighborhood index.
    pub neighborhood: usize,
    /// Whether the rule has a condition.
    pub conditional: bool,
    /// Effect applied when the rule matches.
    pub effect: RuleEffect<C>,
    /// Earlier rule index that makes this rule unreachable, if any.
    pub shadowed_by: Option<usize>,
    /// Earlier rule index that duplicates this rule exactly, if any.
    pub duplicate_of: Option<usize>,
    /// Rule-local diagnostics.
    pub diagnostics: Vec<Diagnostic>,
}

/// Derived information about a single named neighborhood.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NeighborhoodAnalysis {
    /// Zero-based neighborhood index.
    pub index: usize,
    /// Neighborhood name.
    pub name: String,
    /// Underlying neighborhood specification.
    pub spec: NeighborhoodSpec,
    /// Number of neighbor positions included by the neighborhood.
    pub neighbor_count: u32,
    /// Number of rules referencing this neighborhood.
    pub used_by_rules: usize,
}

/// Full analysis result for a blueprint spec.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SpecAnalysis<C: CellState> {
    /// High-level summary.
    pub summary: SpecSummary,
    /// Per-rule analysis in declaration order.
    pub rules: Vec<RuleAnalysis<C>>,
    /// Per-neighborhood analysis in declaration order.
    pub neighborhoods: Vec<NeighborhoodAnalysis>,
    /// Diagnostics that apply at the spec, topology, or neighborhood level.
    pub diagnostics: Vec<Diagnostic>,
}

impl<C: CellState> SpecAnalysis<C> {
    /// Iterate all diagnostics, including rule-local ones.
    pub fn all_diagnostics(&self) -> impl Iterator<Item = &Diagnostic> {
        self.diagnostics
            .iter()
            .chain(self.rules.iter().flat_map(|rule| rule.diagnostics.iter()))
    }
}
