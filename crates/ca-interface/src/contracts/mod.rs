//! Declarative blueprint contracts and portable descriptors.

/// Declarative blueprint specification and builder APIs.
pub mod automaton;
/// Portable descriptor types shared across solver implementations.
pub mod descriptors;

pub use automaton::{
    neighbors, AutomatonBuilder, AutomatonSpec, BlueprintBuilder, BlueprintSpec, BuildError,
    Condition, CountComparison, Hyle, HyleBuilder, NamedNeighborhood, NeighborCount,
    NeighborSelector, Rule, RuleBuilder, RuleEffect, RulesBuilder, Semantics,
};
pub use descriptors::{
    AxisTopology, GridDims, GridRegion, GridSnapshot, NeighborhoodFalloff, NeighborhoodShape,
    NeighborhoodSpec, TopologyDescriptor,
};
