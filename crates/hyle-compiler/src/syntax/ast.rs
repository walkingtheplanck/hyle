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
    /// Radius literal.
    pub radius: LiteralAst,
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
    /// Integer literal text.
    Integer(String),
    /// Floating-point literal text.
    Float(String),
    /// Boolean literal.
    Bool(bool),
}

/// Runtime field bounds.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BoundsAst {
    /// Lower bound literal.
    pub lower: LiteralAst,
    /// Whether the lower bound is inclusive.
    pub lower_inclusive: bool,
    /// Upper bound literal.
    pub upper: LiteralAst,
    /// Whether the upper bound is inclusive.
    pub upper_inclusive: bool,
}

/// Rule declaration.
#[derive(Clone, Debug, PartialEq)]
pub struct RuleAst {
    /// Anchor model that defines the dispatch grid.
    pub anchor: String,
    /// Optional sampled model read by the rule.
    pub sampled: Option<RuleSourceAst>,
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

/// Expression syntax.
#[derive(Clone, Debug, PartialEq)]
pub struct ExprAst {
    /// Parsed expression tree.
    pub kind: ExprKindAst,
}

/// Parsed expression form.
#[derive(Clone, Debug, PartialEq)]
pub enum ExprKindAst {
    /// Literal value.
    Literal(LiteralAst),
    /// Named value or model reference.
    Name(String),
    /// Field access.
    Field {
        /// Base expression.
        base: Box<ExprAst>,
        /// Field name.
        field: String,
    },
    /// Function call.
    Call {
        /// Callee expression.
        callee: Box<ExprAst>,
        /// Call arguments.
        arguments: Vec<ExprAst>,
    },
    /// Prefix operator expression.
    Unary {
        /// Operator.
        op: UnaryOpAst,
        /// Operand.
        expression: Box<ExprAst>,
    },
    /// Binary operator expression.
    Binary {
        /// Left operand.
        left: Box<ExprAst>,
        /// Operator.
        op: BinaryOpAst,
        /// Right operand.
        right: Box<ExprAst>,
    },
    /// Reduction over a neighborhood or sampled collection.
    Reduction {
        /// Reduction operation.
        op: ReductionOpAst,
        /// Binding name for each element.
        binding: String,
        /// Iterable expression.
        iterable: Box<ExprAst>,
        /// Reduction body expression.
        body: Box<ExprAst>,
    },
}

/// Prefix expression operator.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UnaryOpAst {
    /// Numeric negation.
    Neg,
    /// Boolean negation.
    Not,
}

/// Binary expression operator.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BinaryOpAst {
    /// Addition.
    Add,
    /// Subtraction.
    Sub,
    /// Multiplication.
    Mul,
    /// Division.
    Div,
    /// Equality.
    Eq,
    /// Inequality.
    NotEq,
    /// Less-than comparison.
    Less,
    /// Less-than-or-equal comparison.
    LessEq,
    /// Greater-than comparison.
    Greater,
    /// Greater-than-or-equal comparison.
    GreaterEq,
    /// Boolean conjunction.
    And,
    /// Boolean disjunction.
    Or,
}

/// Reduction expression operator.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ReductionOpAst {
    /// Sum reduction.
    Sum,
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
