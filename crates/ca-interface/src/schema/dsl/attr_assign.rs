use crate::schema::{AttributeRef, AttributeSet};
use crate::AttributeValue;

/// One material-scoped attribute assignment used by `material_attributes`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AttrAssign {
    /// Attribute being attached to a material.
    pub attribute: AttributeRef,
    /// Default value for that material.
    pub default: AttributeValue,
}

impl AttrAssign {
    /// Start building a material-scoped attribute assignment.
    ///
    /// The typed first step keeps the authored API ergonomic while the builder
    /// later validates that the default value matches the attribute's type.
    #[allow(clippy::new_ret_no_self)]
    pub fn new<A: AttributeSet>(attribute: A) -> PendingAttrAssign {
        PendingAttrAssign {
            attribute: attribute.attribute(),
        }
    }

    /// Construct a material-scoped attribute assignment with a default value.
    ///
    /// This is a compact form of `AttrAssign::new(attribute).default(value)`.
    pub fn with_default<A: AttributeSet>(attribute: A, default: impl Into<AttributeValue>) -> Self {
        Self {
            attribute: attribute.attribute(),
            default: default.into(),
        }
    }
}

/// Pending material-scoped attribute assignment awaiting its default value.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PendingAttrAssign {
    attribute: AttributeRef,
}

impl PendingAttrAssign {
    /// Finalize the assignment with a material default value.
    ///
    /// The builder later checks that this value matches the declared attribute
    /// scalar type before it reaches the schema.
    pub fn default(self, value: impl Into<AttributeValue>) -> AttrAssign {
        AttrAssign {
            attribute: self.attribute,
            default: value.into(),
        }
    }
}
