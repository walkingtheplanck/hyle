#![doc = include_str!("../README.md")]
#![deny(missing_docs)]

pub mod cell;
pub mod contracts;
/// Convenient imports for the declarative blueprint API.
pub mod prelude;
pub mod solver;
pub mod topology;

pub use cell::Cell;
pub use contracts::{
    neighbors, AutomatonBuilder, AutomatonSpec, AxisTopology, BlueprintBuilder, BlueprintSpec,
    BuildError, Condition, CountComparison, GridDims, GridRegion, GridSnapshot, Hyle, HyleBuilder,
    NamedNeighborhood, NeighborCount, NeighborSelector, NeighborhoodFalloff, NeighborhoodShape,
    NeighborhoodSpec, Rule, RuleBuilder, RuleEffect, RulesBuilder, Semantics, TopologyDescriptor,
};
pub use solver::CaSolver;
pub use solver::ValidatedSolver;
pub use topology::Topology;
