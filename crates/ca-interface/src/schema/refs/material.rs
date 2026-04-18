use std::any::TypeId;

use crate::schema::{MaterialSet, SetContractError};
use crate::MaterialId;

/// Type-erased reference to one material symbol from a specific material set.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct MaterialRef {
    owner: TypeId,
    id: Result<MaterialId, SetContractError>,
    label: &'static str,
}

impl MaterialRef {
    /// Construct a new material reference.
    pub fn new<M: MaterialSet>(material: M) -> Self {
        Self {
            owner: TypeId::of::<M>(),
            id: material.id(),
            label: material.label(),
        }
    }

    /// Return the owning material-set type.
    pub fn owner(self) -> TypeId {
        self.owner
    }

    /// Return the resolved material identifier.
    pub const fn id(self) -> Result<MaterialId, SetContractError> {
        self.id
    }

    /// Return the human-readable material label.
    pub const fn label(self) -> &'static str {
        self.label
    }
}
