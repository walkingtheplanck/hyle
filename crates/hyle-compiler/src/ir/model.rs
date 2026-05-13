use serde::{Deserialize, Serialize};

use crate::ir::{Identifier, TypeIr};

/// Literal value preserved in compiler IR.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", content = "value", rename_all = "snake_case")]
pub enum LiteralIr {
    /// Numeric literal text.
    Number(String),
    /// Boolean literal.
    Bool(bool),
}

/// Runtime bounds declared for a field.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct BoundsIr {
    /// Lower bound literal.
    pub lower: String,
    /// Whether the lower bound is inclusive.
    pub lower_inclusive: bool,
    /// Upper bound literal.
    pub upper: String,
    /// Whether the upper bound is inclusive.
    pub upper_inclusive: bool,
}

/// A declared field in the simulation model.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct FieldIr {
    /// Stable field name.
    pub name: Identifier,
    /// Declared field type.
    pub ty: TypeIr,
    /// Optional default value.
    pub default: Option<LiteralIr>,
    /// Optional runtime bounds.
    pub bounds: Option<BoundsIr>,
}

/// The data model referenced by rules and backends.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct ModelIr {
    /// Stable model name.
    pub name: Identifier,
    /// Model resolution relative to the base world cell.
    pub resolution: u32,
    /// Default neighborhood used by rules when no range override is present.
    pub default_neighborhood: Option<Identifier>,
    /// Fields available to rules and runtime backends.
    pub fields: Vec<FieldIr>,
}

/// External simulation input.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct InputIr {
    /// Stable input name.
    pub name: Identifier,
    /// Declared input type.
    pub ty: TypeIr,
    /// Optional default value.
    pub default: Option<LiteralIr>,
}
