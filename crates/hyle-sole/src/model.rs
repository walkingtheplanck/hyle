use serde::{Deserialize, Serialize};

use crate::{SoleBounds, SoleLiteralValue};

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
