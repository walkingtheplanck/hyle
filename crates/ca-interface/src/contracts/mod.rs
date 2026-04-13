//! Declarative blueprint contracts and portable descriptors.

/// Declarative blueprint specification and builder APIs.
pub mod blueprint;
/// Declarative cell-state constraints shared by blueprints.
pub mod cell;
/// Portable descriptor types shared across solver implementations.
pub mod descriptors;

pub use blueprint::{
    neighbors, rng, AutomatonBuilder, AutomatonSpec, BlueprintBuilder, BlueprintSpec, BuildError,
    Condition, CountComparison, Hyle, HyleBuilder, NamedNeighborhood, NeighborCount,
    NeighborSelector, RandomSource, Rule, RuleBuilder, RuleEffect, RulesBuilder, Semantics,
};
pub use cell::{CellModel, CellSchema, CellState, StateDef};
pub use descriptors::{
    AxisTopology, GridDims, GridRegion, GridSnapshot, NeighborhoodFalloff, NeighborhoodShape,
    NeighborhoodSpec, TopologyDescriptor,
};
