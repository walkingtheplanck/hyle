//! Declarative blueprint specification and builder APIs.

mod builder;
mod dsl;
mod spec;

pub use builder::{BlueprintBuilder, BuildError, RuleBuilder, RulesBuilder};
pub use dsl::{
    neighbors, rng, Condition, CountComparison, NeighborCount, NeighborSelector,
    NeighborWeightedSum, RandomSource, Weight, WeightComparison,
};
pub use spec::{BlueprintSpec, NamedNeighborhood, Rule, RuleEffect, Semantics};
