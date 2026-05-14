use serde::{Deserialize, Serialize};

/// `.sole` JSON module produced by code generation.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SoleModule {
    /// `.sole` schema version.
    pub version: String,
    /// World lattice declaration.
    pub world: SoleWorld,
    /// Named neighborhood ranges.
    pub ranges: Vec<SoleRange>,
    /// Runtime models.
    pub models: Vec<SoleModel>,
    /// External inputs.
    pub inputs: Vec<SoleInput>,
    /// Executable update/transform rules.
    pub rules: Vec<SoleRule>,
}

/// World lattice declaration.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct SoleWorld {
    /// Number of dimensions.
    pub dimensions: u8,
    /// Cell shape name.
    pub cell: String,
}

/// Named neighborhood range.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SoleRange {
    /// Range id.
    pub id: usize,
    /// Source range name.
    pub name: String,
    /// Numeric radius.
    pub radius: SoleLiteralValue,
    /// Whether the center cell is included.
    pub center: bool,
    /// Distance metric name.
    pub metric: String,
}

/// Runtime model.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SoleModel {
    /// Model id.
    pub id: usize,
    /// Source model name.
    pub name: String,
    /// Resolution relative to the base world cell.
    pub resolution: u32,
    /// Default range id.
    pub range: usize,
    /// Runtime fields.
    pub fields: Vec<SoleField>,
}

/// Runtime field.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SoleField {
    /// Field id inside the model.
    pub id: usize,
    /// Source field name.
    pub name: String,
    /// Concrete runtime type.
    #[serde(rename = "type")]
    pub ty: String,
    /// Default field value.
    pub default: SoleLiteralValue,
    /// Runtime bounds.
    pub bounds: Option<SoleBounds>,
    /// Numeric equality/precision epsilon.
    pub epsilon: f64,
}

/// External input.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SoleInput {
    /// Input id.
    pub id: usize,
    /// Source input name.
    pub name: String,
    /// Concrete runtime type.
    #[serde(rename = "type")]
    pub ty: String,
    /// Default input value.
    pub default: SoleLiteralValue,
    /// Runtime bounds.
    pub bounds: Option<SoleBounds>,
    /// Numeric equality/precision epsilon.
    pub epsilon: f64,
}

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
