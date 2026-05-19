use serde::{Deserialize, Serialize};

use crate::SoleExpr;

/// Rule declaration.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SoleRule {
    /// Rule id.
    pub id: usize,
    /// Stable generated rule name.
    pub name: String,
    /// Anchor model id.
    pub anchor: usize,
    /// Target model id.
    pub target: usize,
    /// Neighborhood range id.
    pub range: usize,
    /// Sampled models.
    pub samples: Vec<SoleSample>,
    /// Optional guard expression.
    pub when: Option<SoleExpr>,
    /// Local bindings.
    pub lets: Vec<SoleLet>,
    /// Field writes.
    pub writes: Vec<SoleWrite>,
}

/// Sampled model declaration.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct SoleSample {
    /// Sampled model id.
    pub model: usize,
    /// Sampling mode.
    pub mode: String,
}

/// Local binding.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SoleLet {
    /// Local id.
    pub id: usize,
    /// Local name.
    pub name: String,
    /// Concrete local type.
    #[serde(rename = "type")]
    pub ty: String,
    /// Bound expression.
    pub value: SoleExpr,
}

/// Field write.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SoleWrite {
    /// Field id in the target model.
    pub field: usize,
    /// New value expression.
    pub value: SoleExpr,
}
