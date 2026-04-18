#![doc = include_str!("../README.md")]
#![deny(missing_docs)]

extern crate self as hyle_ca_interface;

mod handles;
/// Common framework imports for schema authoring and runtime setup.
pub mod prelude;
/// Canonical resolved forms derived from declarative schemas.
pub mod resolved;
mod runtime;
mod schema;

pub use handles::{AttributeId, CellId, MaterialId, NeighborhoodId, RngStreamId};
pub use hyle_ca_interface_derive::MaterialSet;
pub use resolved::Rng;
pub use runtime::{
    AttributeAccessError, CaRuntime, CaSolver, CaSolverProvider, CellAttributeValue,
    CellQueryError, GridAccessError, Instance, Runtime, RuntimeAttributes, RuntimeCells,
    RuntimeGrid, RuntimeMetadata, RuntimeMetrics, RuntimeStepping, SolverAttributes, SolverCells,
    SolverExecution, SolverGrid, SolverMetadata, SolverMetrics, Topology, TransitionCount,
};
pub use schema::{
    attr, neighbors, rng, AttrAssign, AttributeAssignment, AttributeComparison, AttributeDef,
    AttributeRef, AttributeSelector, AttributeSet, AttributeType, AttributeValue, AxisTopology,
    Blueprint, BlueprintBuilder, BuildError, Condition, CountComparison, GridDims, GridRegion,
    GridShapeError, GridSnapshot, MatAttr, MaterialAttributeBinding, MaterialDef, MaterialRef, MaterialSet,
    NeighborCount, NeighborSelector, NeighborWeightedSum, NeighborhoodFalloff, NeighborhoodRadius,
    NeighborhoodRef, NeighborhoodSet, NeighborhoodShape, NeighborhoodSpec, RandomSource,
    ResolvedCondition, Rule, RuleEffect, RuleSpec, Semantics, TopologyDescriptor, Weight,
    WeightComparison, WEIGHT_SCALE,
};
