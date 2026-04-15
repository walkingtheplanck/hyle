//! Portable blueprint material descriptors.

use crate::contracts::{MaterialAttributeBinding, MaterialId};

/// One named material declared by a blueprint.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MaterialDef {
    /// Stable numeric identifier.
    pub id: MaterialId,
    /// Human-readable material name.
    pub name: &'static str,
    /// Attached attributes and their material-specific defaults.
    pub attributes: Vec<MaterialAttributeBinding>,
}

impl MaterialDef {
    /// Construct a named material descriptor.
    pub fn new(
        id: MaterialId,
        name: &'static str,
        attributes: Vec<MaterialAttributeBinding>,
    ) -> Self {
        Self {
            id,
            name,
            attributes,
        }
    }
}
