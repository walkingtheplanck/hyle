#![doc = include_str!("../README.md")]
#![deny(missing_docs)]

mod contracts;
/// Common framework imports for blueprint authoring and runtime setup.
pub mod prelude;
mod runtime;
/// Canonical interpretation helpers derived from declarative contracts.
pub mod semantics;

pub use contracts::{
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
    AttributeAccessError, CaRuntime, CaSolver, CaSolverProvider, Instance, StepReport, Topology,
    TransitionCount, ValidatedSolver,
};
pub use semantics::Rng;
