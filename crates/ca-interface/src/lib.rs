#![doc = include_str!("../README.md")]
#![deny(missing_docs)]

mod contracts;
/// Common framework imports for blueprint authoring and runtime setup.
pub mod prelude;
mod runtime;
/// Canonical interpretation helpers derived from declarative contracts.
pub mod semantics;

pub use contracts::{
    attr, neighbors, rng, AttributeAssignment, AttributeComparison, AttributeDef,
    AttributeSelector, AttributeType, AttributeValue, AxisTopology, Blueprint, BlueprintBuilder,
    BuildError, Condition, CountComparison, GridDims, GridRegion, GridSnapshot, NamedNeighborhood,
    NeighborCount, NeighborSelector, NeighborWeightedSum, NeighborhoodFalloff, NeighborhoodShape,
    NeighborhoodSpec, RandomSource, Rule, RuleBuilder, RuleEffect, RulesBuilder, Semantics,
    TopologyDescriptor, Weight, WeightComparison, WEIGHT_SCALE,
};
pub use contracts::{CellModel, CellSchema, CellState, StateDef};
pub use runtime::{
    AttributeAccessError, CaRuntime, CaSolver, CaSolverProvider, Cell, Instance, Topology,
    ValidatedSolver,
};
pub use semantics::Rng;
