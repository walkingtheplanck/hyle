use crate::semantics::hir::hir_ids::*;
use crate::semantics::hir::hir_values::{ConstValue, ScalarType};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BuiltinFunction {
    Clamp,
    Min,
    Max,
    Abs,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UnaryOp {
    Neg,
    Not,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BinaryOp {
    Add,
    Sub,
    Mul,
    Div,

    Eq,
    NotEq,
    Less,
    LessEq,
    Greater,
    GreaterEq,

    And,
    Or,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ReductionOp {
    Sum,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ExprKindHir {
    Const(ConstValue),

    Local(LocalId),
    Input(InputId),

    /// Refers to the current cell/state of a model in this rule context.
    Model(ModelId),

    /// Accesses a field from a model/current element/local element.
    Field {
        base: Box<ExprHir>,
        field: FieldId,
    },

    BuiltinCall {
        function: BuiltinFunction,
        arguments: Vec<ExprHir>,
    },

    Unary {
        op: UnaryOp,
        expression: Box<ExprHir>,
    },

    Binary {
        left: Box<ExprHir>,
        op: BinaryOp,
        right: Box<ExprHir>,
    },

    Neighbors {
        model: ModelId,
    },

    Reduction {
        op: ReductionOp,
        binding: LocalId,
        iterable: Box<ExprHir>,
        body: Box<ExprHir>,
    },
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ExprHir {
    pub kind: ExprKindHir,
    pub ty: TypeHir,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TypeHir {
    Scalar(ScalarType),

    /// A readable model cell/state in the current rule context.
    Model(ModelId),

    /// A neighbor/sample element exposing fields of a model.
    Element(ModelId),

    /// Iterable collection of elements.
    Collection(ModelId),

    /// Internal error recovery type so one bad expression doesn't explode diagnostics.
    Error,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LocalHir {
    pub id: LocalId,
    pub name: String,
    pub ty: ScalarType,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum RuleStatementHir {
    Let {
        local: LocalId,
        expression: ExprHir,
    },

    Next {
        model: ModelId,
        field: FieldId,
        expression: ExprHir,
    },
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RuleHir {
    pub id: RuleId,

    pub kind: RuleKind,

    pub anchor: ModelId,
    pub sampled: Option<SampledModelHir>,
    pub output: ModelId,

    pub range: Option<NeighborhoodId>,

    pub condition: Option<ExprHir>,
    pub locals: Vec<LocalHir>,
    pub statements: Vec<RuleStatementHir>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RuleKind {
    Update,
    Transform,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SampledModelHir {
    pub model: ModelId,
    pub sampling: Option<Sampling>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sampling {
    Average,
    Nearest,
    Sum,
    All,
}
