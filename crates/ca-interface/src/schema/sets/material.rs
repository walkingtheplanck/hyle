use std::any::TypeId;

use crate::MaterialId;

/// Type-erased reference to one material symbol from a specific material set.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct MaterialRef {
    owner: TypeId,
    id: MaterialId,
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
    pub const fn id(self) -> MaterialId {
        self.id
    }

    /// Return the human-readable material label.
    pub const fn label(self) -> &'static str {
        self.label
    }
}

/// Enum-backed material universe used by a schema.
pub trait MaterialSet: Copy + Default + Eq + Send + Sync + 'static {
    /// Return the full ordered material set.
    fn variants() -> &'static [Self];

    /// Return the human-readable material label.
    fn label(self) -> &'static str;

    /// Return the stable numeric identifier for this material.
    ///
    /// # Panics
    ///
    /// Panics if a manual `MaterialSet` implementation returns a `variants()`
    /// slice that does not contain `self`. The derive macro maintains this
    /// contract automatically.
    fn id(self) -> MaterialId {
        let index = Self::variants()
            .iter()
            .position(|candidate| *candidate == self)
            .expect("material must appear in its declared variant list");
        MaterialId::new(index as u16)
    }

    /// Return a type-erased reference to this material.
    fn material(self) -> MaterialRef {
        MaterialRef::new(self)
    }

    /// Return the default material identifier used to initialize new grids.
    fn default_material() -> MaterialId {
        Self::default().id()
    }
}
