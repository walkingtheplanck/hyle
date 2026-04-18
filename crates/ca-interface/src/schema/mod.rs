//! Declarative schema types and authoring helpers.

mod builder;
pub mod defs;
mod dsl;
mod sets;
mod spec;

pub use builder::{BlueprintBuilder, BuildError, MatAttr, RuleSpec};
pub use defs::{
    AttributeDef, AttributeType, AttributeValue, AxisTopology, GridDims, GridRegion,
    GridShapeError, GridSnapshot, MaterialAttributeBinding, MaterialDef, NeighborhoodFalloff, NeighborhoodRadius,
    NeighborhoodShape, NeighborhoodSpec, TopologyDescriptor, WEIGHT_SCALE,
};
pub use dsl::{
    attr, neighbors, rng, AttrAssign, AttributeComparison, AttributeSelector, Condition,
    CountComparison, NeighborCount, NeighborSelector, NeighborWeightedSum, RandomSource, Weight,
    WeightComparison,
};
pub use sets::{
    AttributeRef, AttributeSet, MaterialRef, MaterialSet, NeighborhoodRef, NeighborhoodSet,
};
pub use spec::{AttributeAssignment, Blueprint, ResolvedCondition, Rule, RuleEffect, Semantics};
