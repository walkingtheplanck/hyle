use serde::{Deserialize, Serialize};

use crate::{Identifier, TypeIr};

/// A declared field in the simulation model.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct FieldIr {
    /// Stable field name.
    pub name: Identifier,
    /// Declared field type.
    pub ty: TypeIr,
}

/// The data model referenced by rules and backends.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct ModelIr {
    /// Fields available to rules and runtime backends.
    pub fields: Vec<FieldIr>,
}
