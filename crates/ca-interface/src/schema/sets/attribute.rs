use std::any::TypeId;

use crate::{AttributeId, AttributeType, AttributeValue};

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

/// Enum-backed attribute universe used by a schema.
pub trait AttributeSet: Copy + Eq + Send + Sync + 'static {
    /// Return the full ordered attribute set.
    fn variants() -> &'static [Self];

    /// Return the human-readable attribute label.
    fn label(self) -> &'static str;

    /// Return the scalar type of this attribute.
    fn value_type(self) -> AttributeType;

    /// Return the stable numeric identifier for this attribute.
    ///
    /// # Panics
    ///
    /// Panics if a manual `AttributeSet` implementation returns a `variants()`
    /// slice that does not contain `self`. The derive-like contract here is
    /// that `variants()` is the source of truth for the entire set.
    fn id(self) -> AttributeId {
        let index = Self::variants()
            .iter()
            .position(|candidate| *candidate == self)
            .expect("attribute must appear in its declared variant list");
        AttributeId::new(index as u16)
    }

    /// Return a type-erased reference to this attribute.
    fn attribute(self) -> AttributeRef {
        AttributeRef::new(self)
    }

    /// Return the zero value for this attribute.
    fn zero(self) -> AttributeValue {
        AttributeValue::zero(self.value_type())
    }
}
