use crate::schema::refs::MaterialRef;
use crate::MaterialId;

/// Enum-backed material universe used by a schema.
pub trait MaterialSet: Copy + Default + Eq + Send + Sync + 'static {
    /// Return the full ordered material set.
    fn variants() -> &'static [Self];

    /// Return the human-readable material label.
    fn label(self) -> &'static str;

    /// Return the stable numeric identifier for this material.
    fn id(self) -> MaterialId {
        self.try_id().unwrap_or_default()
    }

    /// Return the stable numeric identifier for this material, if the trait
    /// implementation is internally consistent.
    ///
    /// Manual impls are expected to include every variant in `variants()`. The
    /// derive macro maintains that contract automatically.
    fn try_id(self) -> Option<MaterialId> {
        Self::variants()
            .iter()
            .position(|candidate| *candidate == self)
            .map(|index| MaterialId::new(index as u16))
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
