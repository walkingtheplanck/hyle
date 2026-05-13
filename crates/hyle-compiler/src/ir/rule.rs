use serde::{Deserialize, Serialize};

use crate::ir::Identifier;

/// Supported sampling algorithms for model sources.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum SamplingIr {
    /// Average sampled cells.
    Average,
    /// Nearest sampled cell.
    Nearest,
    /// Sum sampled cells.
    Sum,
    /// Preserve all sampled cells.
    All,
    /// Forward-compatible custom sampling algorithm.
    Custom(Identifier),
}

/// One model read by a rule.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct RuleSourceIr {
    /// Referenced model name.
    pub model: Identifier,
    /// Optional sampling algorithm. `None` marks the anchor model.
    pub sampling: Option<SamplingIr>,
}

/// Lowered rule body statement.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum RuleStatementIr {
    /// Local binding statement.
    Let {
        /// Binding name.
        name: Identifier,
        /// Expression text preserved until expression IR/codegen lands.
        expression: String,
    },
    /// Write to a next-state model field.
    Next {
        /// Destination model.
        model: Identifier,
        /// Destination field.
        field: Identifier,
        /// Expression text preserved until expression IR/codegen lands.
        expression: String,
    },
}

/// A rule lowered from the Hyle DSL.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct RuleIr {
    /// Stable rule name.
    pub name: Identifier,
    /// Model sources read by this rule.
    pub sources: Vec<RuleSourceIr>,
    /// Output model written or emitted by this rule.
    pub output: Identifier,
    /// Rule-specific neighborhood override.
    pub range: Option<Identifier>,
    /// Optional guard expression.
    pub condition: Option<String>,
    /// Lowered body statements.
    pub statements: Vec<RuleStatementIr>,
}
