#[derive(Clone, Debug, PartialEq, Eq)]
pub enum NumberValue {
    Integer(String),
    Float(String),
}

impl NumberValue {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Integer(value) | Self::Float(value) => value,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ConstValue {
    Integer(String),
    Float(String),
    Bool(bool),
}

impl ConstValue {
    pub fn as_number(&self) -> Option<NumberValue> {
        match self {
            Self::Integer(value) => Some(NumberValue::Integer(value.clone())),
            Self::Float(value) => Some(NumberValue::Float(value.clone())),
            Self::Bool(_) => None,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ScalarType {
    Int,
    Float,
    Bool,
    Custom(String),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum PrecisionHir {
    F32,
    Literal(NumberValue),
}

impl PrecisionHir {
    pub const DEFAULT: Self = Self::F32;

    pub fn as_ir_text(&self) -> &str {
        match self {
            Self::F32 => "f32",
            Self::Literal(value) => value.as_str(),
        }
    }
}

impl Default for PrecisionHir {
    fn default() -> Self {
        Self::DEFAULT
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BoundsHir {
    pub lower: NumberValue,
    pub lower_inclusive: bool,
    pub upper: NumberValue,
    pub upper_inclusive: bool,
}
