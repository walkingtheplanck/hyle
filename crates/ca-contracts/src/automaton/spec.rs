//! Canonical automaton specification types.

use crate::{Cell, NeighborhoodSpec, TopologyDescriptor};

use super::Condition;

/// Portable semantics version for an automaton specification.
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
pub enum RuleEffect<C: Cell> {
    /// Leave the center cell unchanged and stop evaluating later rules.
    Keep,
    /// Replace the center cell with a new value and stop evaluating later rules.
    Become(C),
}

/// One deterministic rule in an [`AutomatonSpec`].
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Rule<C: Cell> {
    /// Exact center-cell state that this rule applies to.
    pub when: C,
    /// Index into [`AutomatonSpec::neighborhoods`].
    pub neighborhood: usize,
    /// Optional condition that must evaluate to `true`.
    pub condition: Option<Condition<C>>,
    /// Effect applied when the rule matches.
    pub effect: RuleEffect<C>,
}

/// Immutable, backend-agnostic automaton specification.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AutomatonSpec<C: Cell> {
    semantics: Semantics,
    topology: TopologyDescriptor,
    neighborhoods: Vec<NamedNeighborhood>,
    default_neighborhood: usize,
    rules: Vec<Rule<C>>,
}

impl<C: Cell> AutomatonSpec<C> {
    pub(crate) fn new(
        semantics: Semantics,
        topology: TopologyDescriptor,
        neighborhoods: Vec<NamedNeighborhood>,
        default_neighborhood: usize,
        rules: Vec<Rule<C>>,
    ) -> Self {
        Self {
            semantics,
            topology,
            neighborhoods,
            default_neighborhood,
            rules,
        }
    }

    /// The declared semantics version.
    pub fn semantics(&self) -> Semantics {
        self.semantics
    }

    /// The topology descriptor shared across backends.
    pub fn topology(&self) -> TopologyDescriptor {
        self.topology
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
