//! Canonical blueprint contract types.

use crate::{
    AttributeAssignment, AttributeDef, CellModel, CellSchema, CellState, NeighborhoodSpec,
    TopologyDescriptor,
};

use super::{BlueprintBuilder, Condition};

/// Portable semantics version for a blueprint contract.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Semantics {
    /// Version 1 semantics: deterministic local rules with first-match wins.
    #[default]
    V1,
}

/// A reusable named neighborhood definition.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NamedNeighborhood {
    /// Human-readable neighborhood name used by the builder DSL.
    pub name: String,
    /// Neighborhood sampling behavior.
    pub spec: NeighborhoodSpec,
}

impl NamedNeighborhood {
    /// Construct a named neighborhood definition.
    pub fn new(name: impl Into<String>, spec: NeighborhoodSpec) -> Self {
        Self {
            name: name.into(),
            spec,
        }
    }
}

/// The effect produced by a matching rule.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RuleEffect<C: CellState> {
    /// Leave the center cell unchanged and stop evaluating later rules.
    Keep,
    /// Replace the center cell with a new value and stop evaluating later rules.
    Become(C),
}

/// One deterministic rule in a [`Blueprint`].
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Rule<C: CellState> {
    /// Exact center-cell state that this rule applies to.
    pub when: C,
    /// Index into [`Blueprint::neighborhoods`].
    pub neighborhood: usize,
    /// Optional condition that must evaluate to `true`.
    pub condition: Option<Condition<C>>,
    /// Attached attribute writes applied when the rule matches.
    pub attribute_updates: Vec<AttributeAssignment>,
    /// Effect applied when the rule matches.
    pub effect: RuleEffect<C>,
}

/// Immutable, solver-agnostic blueprint contract.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Blueprint<C: CellModel> {
    cell_schema: CellSchema,
    semantics: Semantics,
    topology: TopologyDescriptor,
    attributes: Vec<AttributeDef>,
    neighborhoods: Vec<NamedNeighborhood>,
    default_neighborhood: usize,
    rules: Vec<Rule<C>>,
}

impl<C: CellModel> Blueprint<C> {
    /// Start building a solver-agnostic blueprint.
    pub fn builder() -> BlueprintBuilder<C> {
        BlueprintBuilder::new()
    }

    pub(crate) fn new(
        cell_schema: CellSchema,
        semantics: Semantics,
        topology: TopologyDescriptor,
        attributes: Vec<AttributeDef>,
        neighborhoods: Vec<NamedNeighborhood>,
        default_neighborhood: usize,
        rules: Vec<Rule<C>>,
    ) -> Self {
        Self {
            cell_schema,
            semantics,
            topology,
            attributes,
            neighborhoods,
            default_neighborhood,
            rules,
        }
    }

    /// Portable schema metadata for the blueprint cell model.
    pub fn cell_schema(&self) -> CellSchema {
        self.cell_schema
    }

    /// The declared semantics version.
    pub fn semantics(&self) -> Semantics {
        self.semantics
    }

    /// The topology descriptor shared across solver implementations.
    pub fn topology(&self) -> TopologyDescriptor {
        self.topology
    }

    /// Declared attached per-cell attributes.
    pub fn attributes(&self) -> &[AttributeDef] {
        &self.attributes
    }

    /// Reusable named neighborhoods referenced by rules.
    pub fn neighborhoods(&self) -> &[NamedNeighborhood] {
        &self.neighborhoods
    }

    /// Index of the default neighborhood used by rules that do not override it.
    pub fn default_neighborhood(&self) -> usize {
        self.default_neighborhood
    }

    /// Ordered rules evaluated with first-match-wins semantics.
    pub fn rules(&self) -> &[Rule<C>] {
        &self.rules
    }
}
