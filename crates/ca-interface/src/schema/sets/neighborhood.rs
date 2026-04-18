use crate::schema::refs::NeighborhoodRef;
use crate::NeighborhoodId;

/// Enum-backed neighborhood universe used by a schema.
pub trait NeighborhoodSet: Copy + Eq + Send + Sync + 'static {
    /// Return the full ordered neighborhood set.
    fn variants() -> &'static [Self];

    /// Return the human-readable neighborhood label.
    fn label(self) -> &'static str;

    /// Return the stable numeric identifier for this neighborhood.
    fn id(self) -> NeighborhoodId {
        self.try_id().unwrap_or_default()
    }

    /// Return the stable numeric identifier for this neighborhood, if the
    /// trait implementation is internally consistent.
    fn try_id(self) -> Option<NeighborhoodId> {
        Self::variants()
            .iter()
            .position(|candidate| *candidate == self)
            .map(|index| NeighborhoodId::new(index as u16))
    }

    /// Return a type-erased reference to this neighborhood.
    fn neighborhood(self) -> NeighborhoodRef {
        NeighborhoodRef::new(self)
    }

    /// Return the default neighborhood identifier, using the first variant.
    fn default_neighborhood() -> NeighborhoodId {
        Self::variants()
            .first()
            .copied()
            .and_then(|neighborhood| neighborhood.try_id())
            .unwrap_or_default()
    }

    /// Return the default neighborhood identifier when the set declares at
    /// least one neighborhood and its variant list is internally consistent.
    fn try_default_neighborhood() -> Option<NeighborhoodId> {
        Self::variants()
            .first()
            .copied()
            .and_then(|neighborhood| neighborhood.try_id())
    }
}
