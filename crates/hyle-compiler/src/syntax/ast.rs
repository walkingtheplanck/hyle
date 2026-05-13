/// Parsed Hyle script.
#[derive(Clone, Debug, PartialEq)]
pub struct ScriptAst {
    /// Source path used for diagnostics.
    pub source_path: String,
    /// Declared Hyle language version.
    pub version: String,
    /// Number of world dimensions.
    pub dimensions: u8,
    /// World cell shape.
    pub cell: String,
    /// Named neighborhoods.
    pub neighborhoods: Vec<NeighborhoodAst>,
    /// Declared models.
    pub models: Vec<ModelAst>,
    /// External inputs.
    pub inputs: Vec<InputAst>,
    /// Update and transform rules.
    pub rules: Vec<RuleAst>,
}

/// Named neighborhood declaration.
#[derive(Clone, Debug, PartialEq)]
pub struct NeighborhoodAst {
    /// Neighborhood name.
    pub name: String,
    /// Radius expression.
    pub radius: String,
    /// Whether the center cell is included.
    pub center: bool,
    /// Distance metric.
    pub metric: String,
}

/// Model declaration.
#[derive(Clone, Debug, PartialEq)]
pub struct ModelAst {
    /// Model name.
    pub name: String,
    /// Optional resolution. Semantic lowering applies the default.
    pub resolution: Option<u32>,
    /// Optional default neighborhood.
    pub range: Option<String>,
    /// Declared fields.
    pub fields: Vec<FieldAst>,
}

/// Field declaration.
#[derive(Clone, Debug, PartialEq)]
pub struct FieldAst {
    /// Field name.
    pub name: String,
    /// Logical type.
    pub ty: TypeAst,
    /// Optional default value.
    pub default: Option<LiteralAst>,
    /// Optional runtime bounds.
    pub bounds: Option<BoundsAst>,
}

/// External input declaration.
#[derive(Clone, Debug, PartialEq)]
pub struct InputAst {
    /// Input name.
    pub name: String,
    /// Logical type.
    pub ty: TypeAst,
    /// Optional default value.
    pub default: Option<LiteralAst>,
}

/// Logical scalar type.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TypeAst {
    /// Integer scalar.
    Int,
    /// Floating-point scalar.
    Float,
    /// Boolean scalar.
    Bool,
    /// Forward-compatible custom type.
    Custom(String),
}

/// Literal value.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum LiteralAst {
    /// Numeric literal text.
    Number(String),
    /// Boolean literal.
    Bool(bool),
}

/// Runtime field bounds.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BoundsAst {
    /// Lower bound literal.
    pub lower: String,
    /// Whether the lower bound is inclusive.
    pub lower_inclusive: bool,
    /// Upper bound literal.
    pub upper: String,
    /// Whether the upper bound is inclusive.
    pub upper_inclusive: bool,
}

/// Rule declaration.
#[derive(Clone, Debug, PartialEq)]
pub struct RuleAst {
    /// Model sources read by the rule.
    pub sources: Vec<RuleSourceAst>,
    /// Output model written or emitted by the rule.
    pub output: String,
    /// Optional rule-specific neighborhood.
    pub range: Option<String>,
    /// Optional guard expression.
    pub condition: Option<ExprAst>,
    /// Rule body statements.
    pub statements: Vec<RuleStatementAst>,
}

/// Rule source model.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RuleSourceAst {
    /// Referenced model name.
    pub model: String,
    /// Optional sampling algorithm.
    pub sampling: Option<SamplingAst>,
}

/// Sampling algorithm.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SamplingAst {
    /// Average sampled cells.
    Average,
    /// Nearest sampled cell.
    Nearest,
    /// Sum sampled cells.
    Sum,
    /// Preserve all sampled cells.
    All,
    /// Forward-compatible custom sampling algorithm.
    Custom(String),
}

/// Rule body statement.
#[derive(Clone, Debug, PartialEq)]
pub enum RuleStatementAst {
    /// Local binding.
    Let {
        /// Binding name.
        name: String,
        /// Expression text.
        expression: ExprAst,
    },
    /// Next-state assignment.
    Next {
        /// Destination model.
        model: String,
        /// Destination field.
        field: String,
        /// Expression text.
        expression: ExprAst,
    },
}

/// Expression placeholder.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ExprAst {
    /// Source text for the expression.
    pub text: String,
}
