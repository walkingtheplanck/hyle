use serde::{Deserialize, Serialize};

use crate::SoleLiteralValue;

/// Expression encoded in `.sole` JSON.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SoleExpr {
    /// Literal expression.
    Literal { literal: SoleLiteral },
    /// Field read expression.
    Read { read: SoleRead },
    /// Local read expression.
    Local { local: usize },
    /// Input read expression.
    Input { input: usize },
    /// Function call expression.
    Call { call: SoleCall },
    /// Binary/unary operator expression.
    Op(SoleOpExpr),
    /// Reduction expression.
    Reduce { reduce: SoleReduce },
    /// Neighbor collection expression.
    Neighbors { neighbors: SoleNeighbors },
}

/// Typed literal expression payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SoleLiteral {
    /// Concrete literal type.
    #[serde(rename = "type")]
    pub ty: String,
    /// Literal value.
    pub value: SoleLiteralValue,
}

/// Field read expression payload.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct SoleRead {
    /// Model id for direct model reads.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<usize>,
    /// Reduction variable name for element reads.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub var: Option<String>,
    /// Field id.
    pub field: usize,
}

/// Function call expression payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SoleCall {
    /// Function name.
    #[serde(rename = "fn")]
    pub function: String,
    /// Call arguments.
    pub args: Vec<SoleExpr>,
}

/// Operator expression payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SoleOpExpr {
    /// Operator name.
    pub op: String,
    /// Operator arguments.
    pub args: Vec<SoleExpr>,
}

/// Reduction expression payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SoleReduce {
    /// Reduction operator.
    pub op: String,
    /// Bound variable name.
    pub var: String,
    /// Collection being reduced.
    pub over: Box<SoleExpr>,
    /// Reduction body.
    pub expr: Box<SoleExpr>,
}

/// Neighbor collection expression payload.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct SoleNeighbors {
    /// Model id.
    pub model: usize,
    /// Range id.
    pub range: usize,
}
