//! Declarative blueprint authoring types and helpers.
//!
//! `schema` owns declarations, builder state, and rule DSL types.
//! Shared domain value types such as grid geometry and scalar attribute values
//! live outside this module so they can be used by both schema and runtime code
//! without implying ownership by either layer.

mod builder;
mod decl;
mod dsl;
pub mod refs;
mod sets;
mod spec;

pub use builder::{BlueprintBuilder, BuildError, MatAttr, RuleSpec};
pub use decl::{
    AttributeDef, AxisTopology, MaterialAttributeBinding, MaterialDef, NeighborhoodFalloff,
    NeighborhoodShape, NeighborhoodSpec, TopologyDescriptor,
};
pub use dsl::{
    attr, neighbors, rng, AttrAssign, AttributeComparison, AttributeSelector, Condition,
    CountComparison, NeighborCount, NeighborSelector, NeighborWeightedSum, RandomSource, Weight,
    WeightComparison,
};
pub use refs::{AttributeRef, MaterialRef, NeighborhoodRef};
pub use sets::{AttributeSet, MaterialSet, NeighborhoodSet};
pub use spec::{AttributeAssignment, Blueprint, ResolvedCondition, Rule, RuleEffect, Semantics};
