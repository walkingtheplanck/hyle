use serde::{Deserialize, Serialize};

use crate::Identifier;

/// A field type exposed by the Hyle IR.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", content = "name", rename_all = "snake_case")]
pub enum TypeIr {
    /// Boolean scalar.
    Bool,
    /// Signed 32-bit integer scalar.
    I32,
    /// 32-bit floating-point scalar.
    F32,
    /// Named type placeholder for future aliases or structs.
    Custom(Identifier),
}
