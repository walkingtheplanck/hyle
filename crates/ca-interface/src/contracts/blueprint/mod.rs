//! Declarative blueprint contracts and builder APIs.

mod builder;
mod dsl;
mod spec;

pub use builder::{BlueprintBuilder, BuildError, MatAttr, RuleSpec};
pub use dsl::{
    attr, neighbors, rng, AttrAssign, AttributeComparison, AttributeSelector, Condition,
    CountComparison, NeighborCount, NeighborSelector, NeighborWeightedSum, RandomSource, Weight,
    WeightComparison,
};
pub use spec::{AttributeAssignment, Blueprint, ResolvedCondition, Rule, RuleEffect, Semantics};
