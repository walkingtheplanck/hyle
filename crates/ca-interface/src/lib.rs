#![doc = include_str!("../README.md")]
#![deny(missing_docs)]

mod contracts;
/// Common framework imports for blueprint authoring and runtime setup.
pub mod prelude;
mod runtime;
/// Canonical interpretation helpers derived from declarative contracts.
pub mod semantics;

pub use contracts::{
    neighbors, rng, AxisTopology, BlueprintBuilder, BlueprintSpec, BuildError, Condition,
    CountComparison, GridDims, GridRegion, GridSnapshot, Hyle, HyleBuilder, NamedNeighborhood,
    NeighborCount, NeighborSelector, NeighborhoodFalloff, NeighborhoodShape, NeighborhoodSpec,
    RandomSource, Rule, RuleBuilder, RuleEffect, RulesBuilder, Semantics, TopologyDescriptor,
};
pub use contracts::{CellModel, CellSchema, CellState, StateDef};
pub use runtime::{
    CaRuntime, CaSolver, CaSolverProvider, Cell, Instance, Topology, ValidatedSolver,
};
pub use semantics::Rng;
