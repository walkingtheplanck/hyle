use std::any::type_name;

use crate::schema::{refs::AttributeRef, SetContractError};
use crate::{AttributeId, AttributeType, AttributeValue};

/// Enum-backed attribute universe used by a schema.
///
/// Prefer `#[derive(AttributeSet)]` for downstream enums. Manual implementations
/// remain an escape hatch, but the derive is the supported path.
pub trait AttributeSet: Copy + Eq + Send + Sync + 'static {
    /// Return the full ordered attribute set.
    fn variants() -> &'static [Self];

    /// Return the human-readable attribute label.
    fn label(self) -> &'static str;

    /// Return the scalar type of this attribute.
    fn value_type(self) -> AttributeType;

    /// Return the stable numeric identifier for this attribute.
    ///
    /// # Errors
    ///
    /// Returns [`SetContractError`] when a manual implementation omits `self`
    /// from `variants()`.
    fn id(self) -> Result<AttributeId, SetContractError> {
        let label = self.label();
        Self::variants()
            .iter()
            .position(|candidate| *candidate == self)
            .map(|index| AttributeId::new(index as u16))
            .ok_or(SetContractError::MissingAttributeVariant {
                set_type: type_name::<Self>(),
                label,
            })
    }

    /// Return a type-erased reference to this attribute.
    ///
    /// The reference carries any set-contract failure until schema validation
    /// resolves it into ids.
    fn attribute(self) -> AttributeRef {
        AttributeRef::new(self)
    }

    /// Return the zero value for this attribute.
    fn zero(self) -> AttributeValue {
        AttributeValue::zero(self.value_type())
    }
}
