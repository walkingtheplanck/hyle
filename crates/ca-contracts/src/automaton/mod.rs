//! Declarative automaton specification and builder APIs.

mod builder;
mod dsl;
mod spec;

pub use builder::{AutomatonBuilder, BuildError, Hyle, HyleBuilder, RuleBuilder, RulesBuilder};
pub use dsl::{neighbors, Condition, CountComparison, NeighborCount, NeighborSelector};
pub use spec::{AutomatonSpec, NamedNeighborhood, Rule, RuleEffect, Semantics};
