//! Declarative schema attribute records.

use crate::{AttributeId, AttributeType, AttributeValue};

/// One named attached per-cell attribute declared by a schema.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AttributeDef {
    /// Stable numeric identifier.
    pub id: AttributeId,
    /// Human-readable attribute name.
    pub name: &'static str,
    /// Scalar type of the attribute channel.
    pub value_type: AttributeType,
}

impl AttributeDef {
    /// Construct a named attribute descriptor.
    pub const fn new(id: AttributeId, name: &'static str, value_type: AttributeType) -> Self {
        Self {
            id,
            name,
            value_type,
        }
    }
}

/// One material-scoped default for an attached attribute channel.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MaterialAttributeBinding {
    /// Attribute being attached to the material.
    pub attribute: AttributeId,
    /// Default value applied when a cell enters the material.
    pub default: AttributeValue,
}

impl MaterialAttributeBinding {
    /// Construct a new material-scoped attribute binding.
    pub const fn new(attribute: AttributeId, default: AttributeValue) -> Self {
        Self { attribute, default }
    }
}
