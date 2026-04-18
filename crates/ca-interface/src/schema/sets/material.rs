use std::any::type_name;

use crate::schema::{refs::MaterialRef, SetContractError};
use crate::MaterialId;

/// Enum-backed material universe used by a schema.
///
/// Prefer `#[derive(MaterialSet)]` for downstream enums. Manual implementations
/// remain an escape hatch, but the derive is the supported path.
pub trait MaterialSet: Copy + Default + Eq + Send + Sync + 'static {
    /// Return the full ordered material set.
    fn variants() -> &'static [Self];

    /// Return the human-readable material label.
    fn label(self) -> &'static str;

    /// Return the stable numeric identifier for this material.
    ///
    /// # Errors
    ///
    /// Returns [`SetContractError`] when a manual implementation omits `self`
    /// from `variants()`.
    fn id(self) -> Result<MaterialId, SetContractError> {
        let label = self.label();
        Self::variants()
            .iter()
            .position(|candidate| *candidate == self)
            .map(|index| MaterialId::new(index as u16))
            .ok_or(SetContractError::MissingMaterialVariant {
                set_type: type_name::<Self>(),
                label,
            })
    }

    /// Return a type-erased reference to this material.
    ///
    /// The reference carries any set-contract failure until schema validation
    /// resolves it into ids.
    fn material(self) -> MaterialRef {
        MaterialRef::new(self)
    }

    /// Return the default material identifier used to initialize new grids.
    ///
    /// # Errors
    ///
    /// Returns [`SetContractError`] when the default material is missing from
    /// `variants()`.
    fn default_material() -> Result<MaterialId, SetContractError> {
        Self::default().id()
    }
}
