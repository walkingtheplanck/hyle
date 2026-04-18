use crate::schema::refs::MaterialRef;
use crate::MaterialId;

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
