//! Canonical declarative schema contract types.

mod blueprint;
mod rule;

pub use blueprint::{Blueprint, Semantics};
pub use rule::{AttributeAssignment, ResolvedCondition, Rule, RuleEffect};
