use std::any::TypeId;

use crate::schema::{NeighborhoodSet, SetContractError};
use crate::NeighborhoodId;

/// Type-erased reference to one neighborhood symbol from a specific neighborhood set.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct NeighborhoodRef {
    owner: TypeId,
    id: Result<NeighborhoodId, SetContractError>,
    label: &'static str,
}

impl NeighborhoodRef {
    /// Construct a new neighborhood reference.
    pub fn new<N: NeighborhoodSet>(neighborhood: N) -> Self {
        Self {
            owner: TypeId::of::<N>(),
            id: neighborhood.id(),
            label: neighborhood.label(),
        }
    }

    /// Return the owning neighborhood-set type.
    pub fn owner(self) -> TypeId {
        self.owner
    }

    /// Return the resolved neighborhood identifier.
    pub const fn id(self) -> Result<NeighborhoodId, SetContractError> {
        self.id
    }

    /// Return the human-readable neighborhood label.
    pub const fn label(self) -> &'static str {
        self.label
    }
}
