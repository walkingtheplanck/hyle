//! Declarative blueprint contracts and portable descriptors.

/// Declarative blueprint specification and builder APIs.
pub mod blueprint;
/// Portable descriptor types shared across solver implementations.
pub mod descriptors;
/// Enum-backed symbol traits and ids shared by blueprints and runtimes.
pub mod symbols;

pub use blueprint::{
    attr, neighbors, rng, AttrAssign, AttributeAssignment, AttributeComparison, AttributeSelector,
    Blueprint, BlueprintBuilder, BuildError, Condition, CountComparison, MatAttr, NeighborCount,
    NeighborSelector, NeighborWeightedSum, RandomSource, ResolvedCondition, Rule, RuleEffect,
    RuleSpec, Semantics, Weight, WeightComparison,
};
pub use descriptors::{
    AttributeDef, AttributeType, AttributeValue, AxisTopology, GridDims, GridRegion, GridSnapshot,
    MaterialAttributeBinding, MaterialDef, NeighborhoodFalloff, NeighborhoodRadius,
    NeighborhoodShape, NeighborhoodSpec, TopologyDescriptor, WEIGHT_SCALE,
};
pub use symbols::{
    AttributeId, AttributeRef, AttributeSet, MaterialId, MaterialRef, MaterialSet, NeighborhoodId,
    NeighborhoodRef, NeighborhoodSet,
};
