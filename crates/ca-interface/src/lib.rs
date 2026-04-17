#![doc = include_str!("../README.md")]
#![deny(missing_docs)]

mod schema;
/// Common framework imports for schema authoring and runtime setup.
pub mod prelude;
mod runtime;
/// Canonical resolved forms derived from declarative schemas.
pub mod resolved;

pub use schema::{
    attr, neighbors, rng, AttrAssign, AttributeAssignment, AttributeComparison, AttributeDef,
    AttributeId, AttributeRef, AttributeSelector, AttributeSet, AttributeType, AttributeValue,
    AxisTopology, Blueprint, BlueprintBuilder, BuildError, Condition, CountComparison, GridDims,
    GridRegion, GridSnapshot, MatAttr, MaterialAttributeBinding, MaterialDef, MaterialId,
    MaterialRef, MaterialSet, NeighborCount, NeighborSelector, NeighborWeightedSum,
    NeighborhoodFalloff, NeighborhoodId, NeighborhoodRadius, NeighborhoodRef, NeighborhoodSet,
    NeighborhoodShape, NeighborhoodSpec, RandomSource, ResolvedCondition, Rule, RuleEffect,
    RuleSpec, Semantics, TopologyDescriptor, Weight, WeightComparison, WEIGHT_SCALE,
};
pub use runtime::{
    AttributeAccessError, CaRuntime, CaSolver, CaSolverProvider, CellAttributeValue, CellId,
    CellQueryError, Instance, Topology, TransitionCount, ValidatedSolver,
};
pub use resolved::Rng;
