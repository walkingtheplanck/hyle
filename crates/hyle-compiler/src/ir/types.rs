use serde::{Deserialize, Serialize};

use crate::ir::Identifier;

/// A field type exposed by the Hyle IR.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", content = "name", rename_all = "snake_case")]
pub enum TypeIr {
    /// Boolean scalar.
    Bool,
    /// Integer scalar.
    Int,
    /// Floating-point scalar.
    Float,
    /// Named type placeholder for future aliases or structs.
    Custom(Identifier),
}
