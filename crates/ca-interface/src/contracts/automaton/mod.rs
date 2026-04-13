//! Declarative blueprint specification and builder APIs.

mod builder;
mod dsl;
mod spec;

pub use builder::{
    AutomatonBuilder, BlueprintBuilder, BuildError, Hyle, HyleBuilder, RuleBuilder, RulesBuilder,
};
pub use dsl::{neighbors, Condition, CountComparison, NeighborCount, NeighborSelector};
pub use spec::{AutomatonSpec, BlueprintSpec, NamedNeighborhood, Rule, RuleEffect, Semantics};
