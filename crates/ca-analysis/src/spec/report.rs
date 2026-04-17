//! Public report types returned by spec analysis.

use hyle_ca_interface::{
    AttributeDef, MaterialDef, MaterialId, NeighborhoodId, NeighborhoodSpec, RuleEffect, Semantics,
    TopologyDescriptor,
};

use crate::Diagnostic;

/// Top-level summary derived from a schema.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SpecSummary {
    /// Declared material universe.
    pub materials: Vec<MaterialDef>,
    /// Declared semantics version.
    pub semantics: Semantics,
    /// Declared attached per-cell attributes.
    pub attributes: Vec<AttributeDef>,
    /// Total number of rules in declaration order.
    pub rule_count: usize,
    /// Total number of named neighborhoods.
    pub neighborhood_count: usize,
    /// Total number of named attributes.
    pub attribute_count: usize,
    /// Maximum neighborhood radius used anywhere in the spec.
    pub max_radius: u32,
    /// Declared topology.
    pub topology: TopologyDescriptor,
}

/// Derived information about a single rule.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RuleAnalysis {
    /// Zero-based rule index.
    pub index: usize,
    /// Material matched by the rule.
    pub when: MaterialId,
    /// Referenced neighborhood id.
    pub neighborhood: NeighborhoodId,
    /// Whether the rule has a condition.
    pub conditional: bool,
    /// Effect applied when the rule matches.
    pub effect: RuleEffect,
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
    pub name: &'static str,
    /// Underlying neighborhood specification.
    pub spec: NeighborhoodSpec,
    /// Number of neighbor positions included by the neighborhood.
    pub neighbor_count: u32,
    /// Number of rules referencing this neighborhood.
    pub used_by_rules: usize,
}

/// Full analysis result for a schema.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SpecAnalysis {
    /// High-level summary.
    pub summary: SpecSummary,
    /// Per-rule analysis in declaration order.
    pub rules: Vec<RuleAnalysis>,
    /// Per-neighborhood analysis in declaration order.
    pub neighborhoods: Vec<NeighborhoodAnalysis>,
    /// Diagnostics that apply at the spec, topology, or neighborhood level.
    pub diagnostics: Vec<Diagnostic>,
}

impl SpecAnalysis {
    /// Iterate all diagnostics, including rule-local ones.
    pub fn all_diagnostics(&self) -> impl Iterator<Item = &Diagnostic> {
        self.diagnostics
            .iter()
            .chain(self.rules.iter().flat_map(|rule| rule.diagnostics.iter()))
    }
}
