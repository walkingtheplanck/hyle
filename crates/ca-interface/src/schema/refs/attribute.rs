use std::any::TypeId;

use crate::schema::AttributeSet;
use crate::{AttributeId, AttributeType};

/// Type-erased reference to one attribute symbol from a specific attribute set.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct AttributeRef {
    owner: TypeId,
    id: AttributeId,
    label: &'static str,
    value_type: AttributeType,
}

impl AttributeRef {
    /// Construct a new attribute reference.
    pub fn new<A: AttributeSet>(attribute: A) -> Self {
        Self {
            owner: TypeId::of::<A>(),
            id: attribute.id(),
            label: attribute.label(),
            value_type: attribute.value_type(),
        }
    }

    /// Return the owning attribute-set type.
    pub fn owner(self) -> TypeId {
        self.owner
    }

    /// Return the resolved attribute identifier.
    pub const fn id(self) -> AttributeId {
        self.id
    }

    /// Return the human-readable attribute label.
    pub const fn label(self) -> &'static str {
        self.label
    }

    /// Return the declared scalar type.
    pub const fn value_type(self) -> AttributeType {
        self.value_type
    }
}
