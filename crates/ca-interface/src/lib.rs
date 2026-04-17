#![doc = include_str!("../README.md")]
#![deny(missing_docs)]

extern crate self as hyle_ca_interface;

mod handles;
mod schema;
/// Common framework imports for schema authoring and runtime setup.
pub mod prelude;
mod runtime;
/// Canonical resolved forms derived from declarative schemas.
pub mod resolved;

pub use hyle_ca_interface_derive::MaterialSet;
pub use handles::{AttributeId, CellId, MaterialId, NeighborhoodId, RngStreamId};
pub use schema::{
    attr, neighbors, rng, AttrAssign, AttributeAssignment, AttributeComparison, AttributeDef,
    AttributeRef, AttributeSelector, AttributeSet, AttributeType, AttributeValue, AxisTopology,
    Blueprint, BlueprintBuilder, BuildError, Condition, CountComparison, GridDims, GridRegion,
    GridSnapshot, MatAttr, MaterialAttributeBinding, MaterialDef, MaterialRef, MaterialSet,
    NeighborCount, NeighborSelector, NeighborWeightedSum, NeighborhoodFalloff, NeighborhoodRadius,
    NeighborhoodRef, NeighborhoodSet, NeighborhoodShape, NeighborhoodSpec, RandomSource,
    ResolvedCondition, Rule, RuleEffect, RuleSpec, Semantics, TopologyDescriptor, Weight,
    WeightComparison, WEIGHT_SCALE,
};
pub use runtime::{
    AttributeAccessError, CaRuntime, CaSolver, CaSolverProvider, CellAttributeValue,
    CellQueryError, Instance, Runtime, Topology, TransitionCount, ValidatedSolver,
};
pub use resolved::Rng;
