#![doc = include_str!("../README.md")]
#![deny(missing_docs)]

pub mod contracts;
pub mod models;
/// Convenient imports for the declarative blueprint API.
pub mod prelude;
/// Canonical interpretation helpers derived from declarative contracts.
pub mod semantics;

pub use contracts::{
    neighbors, rng, AutomatonBuilder, AutomatonSpec, AxisTopology, BlueprintBuilder, BlueprintSpec,
    BuildError, Condition, CountComparison, GridDims, GridRegion, GridSnapshot, Hyle, HyleBuilder,
    NamedNeighborhood, NeighborCount, NeighborSelector, NeighborhoodFalloff, NeighborhoodShape,
    NeighborhoodSpec, RandomSource, Rule, RuleBuilder, RuleEffect, RulesBuilder, Semantics,
    TopologyDescriptor,
};
pub use contracts::{CellModel, CellSchema, CellState, StateDef};
pub use models::{
    CaRuntime, CaSolver, CaSolverProvider, Cell, Instance, Topology, ValidatedSolver,
};
pub use semantics::{Rng, WEIGHT_SCALE};
