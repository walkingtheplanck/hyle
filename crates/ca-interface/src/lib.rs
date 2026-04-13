#![doc = include_str!("../README.md")]
#![deny(missing_docs)]

pub mod contracts;
pub mod models;
/// Convenient imports for the declarative blueprint API.
pub mod prelude;
/// Shared deterministic random-number primitives.
pub mod rng;
/// Canonical interpretation helpers derived from declarative contracts.
pub mod semantics;
pub mod solver;
pub mod topology;

pub use contracts::CellState;
pub use contracts::{
    neighbors, rng, AutomatonBuilder, AutomatonSpec, AxisTopology, BlueprintBuilder, BlueprintSpec,
    BuildError, Condition, CountComparison, GridDims, GridRegion, GridSnapshot, Hyle, HyleBuilder,
    NamedNeighborhood, NeighborCount, NeighborSelector, NeighborhoodFalloff, NeighborhoodShape,
    NeighborhoodSpec, RandomSource, Rule, RuleBuilder, RuleEffect, RulesBuilder, Semantics,
    TopologyDescriptor,
};
pub use models::Cell;
pub use rng::Rng;
pub use solver::CaSolver;
pub use solver::ValidatedSolver;
pub use topology::Topology;
