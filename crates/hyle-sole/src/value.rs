use serde::{Deserialize, Serialize};

/// Runtime bounds.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SoleBounds {
    /// Lower bound.
    pub min: SoleLiteralValue,
    /// Upper bound.
    pub max: SoleLiteralValue,
    /// Whether the lower bound is closed.
    pub min_closed: bool,
    /// Whether the upper bound is closed.
    pub max_closed: bool,
}

/// Runtime literal value in `.sole` JSON.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SoleLiteralValue {
    /// Integer value.
    Integer(i64),
    /// Floating point value.
    Float(f64),
    /// Boolean value.
    Bool(bool),
}
