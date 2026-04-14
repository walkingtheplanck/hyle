//! Declarative blueprint contracts and builder APIs.

mod builder;
mod dsl;
mod spec;

pub use builder::{BlueprintBuilder, BuildError, RuleBuilder, RulesBuilder};
pub use dsl::{
    attr, neighbors, rng, AttributeAssignment, AttributeComparison, AttributeSelector, Condition,
    CountComparison, NeighborCount, NeighborSelector, NeighborWeightedSum, RandomSource, Weight,
    WeightComparison,
};
pub use spec::{Blueprint, NamedNeighborhood, Rule, RuleEffect, Semantics};
