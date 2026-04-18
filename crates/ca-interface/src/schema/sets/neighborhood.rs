use std::any::type_name;

use crate::schema::{refs::NeighborhoodRef, SetContractError};
use crate::NeighborhoodId;

/// Enum-backed neighborhood universe used by a schema.
pub trait NeighborhoodSet: Copy + Eq + Send + Sync + 'static {
    /// Return the full ordered neighborhood set.
    fn variants() -> &'static [Self];

    /// Return the human-readable neighborhood label.
    fn label(self) -> &'static str;

    /// Return the stable numeric identifier for this neighborhood.
    ///
    /// # Errors
    ///
    /// Returns [`SetContractError`] when a manual implementation omits `self`
    /// from `variants()`.
    fn id(self) -> Result<NeighborhoodId, SetContractError> {
        let label = self.label();
        Self::variants()
            .iter()
            .position(|candidate| *candidate == self)
            .map(|index| NeighborhoodId::new(index as u16))
            .ok_or(SetContractError::MissingNeighborhoodVariant {
                set_type: type_name::<Self>(),
                label,
            })
    }

    /// Return a type-erased reference to this neighborhood.
    ///
    /// The reference carries any set-contract failure until schema validation
    /// resolves it into ids.
    fn neighborhood(self) -> NeighborhoodRef {
        NeighborhoodRef::new(self)
    }

    /// Return the default neighborhood identifier, using the first variant.
    ///
    /// # Errors
    ///
    /// Returns [`SetContractError`] when no neighborhood variants are declared
    /// or the first variant does not resolve back into the set.
    fn default_neighborhood() -> Result<NeighborhoodId, SetContractError> {
        Self::variants()
            .first()
            .copied()
            .ok_or(SetContractError::EmptyNeighborhoodSet {
                set_type: type_name::<Self>(),
            })?
            .id()
    }
}
