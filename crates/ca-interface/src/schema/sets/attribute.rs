use crate::schema::refs::AttributeRef;
use crate::{AttributeId, AttributeType, AttributeValue};

/// Enum-backed attribute universe used by a schema.
pub trait AttributeSet: Copy + Eq + Send + Sync + 'static {
    /// Return the full ordered attribute set.
    fn variants() -> &'static [Self];

    /// Return the human-readable attribute label.
    fn label(self) -> &'static str;

    /// Return the scalar type of this attribute.
    fn value_type(self) -> AttributeType;

    /// Return the stable numeric identifier for this attribute.
    fn id(self) -> AttributeId {
        self.try_id().unwrap_or_default()
    }

    /// Return the stable numeric identifier for this attribute, if the trait
    /// implementation is internally consistent.
    fn try_id(self) -> Option<AttributeId> {
        Self::variants()
            .iter()
            .position(|candidate| *candidate == self)
            .map(|index| AttributeId::new(index as u16))
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
