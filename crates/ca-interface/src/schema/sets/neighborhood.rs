use std::any::TypeId;

use crate::NeighborhoodId;

/// Type-erased reference to one neighborhood symbol from a specific neighborhood set.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct NeighborhoodRef {
    owner: TypeId,
    id: NeighborhoodId,
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
    pub const fn id(self) -> NeighborhoodId {
        self.id
    }

    /// Return the human-readable neighborhood label.
    pub const fn label(self) -> &'static str {
        self.label
    }
}

/// Enum-backed neighborhood universe used by a schema.
pub trait NeighborhoodSet: Copy + Eq + Send + Sync + 'static {
    /// Return the full ordered neighborhood set.
    fn variants() -> &'static [Self];

    /// Return the human-readable neighborhood label.
    fn label(self) -> &'static str;

    /// Return the stable numeric identifier for this neighborhood.
    ///
    /// # Panics
    ///
    /// Panics if a manual `NeighborhoodSet` implementation returns a
    /// `variants()` slice that does not contain `self`.
    fn id(self) -> NeighborhoodId {
        let index = Self::variants()
            .iter()
            .position(|candidate| *candidate == self)
            .expect("neighborhood must appear in its declared variant list");
        NeighborhoodId::new(index as u16)
    }

    /// Return a type-erased reference to this neighborhood.
    fn neighborhood(self) -> NeighborhoodRef {
        NeighborhoodRef::new(self)
    }

    /// Return the default neighborhood identifier, using the first variant.
    ///
    /// # Panics
    ///
    /// Panics if a manual `NeighborhoodSet` implementation returns an empty
    /// `variants()` slice. Neighborhood sets are expected to declare at least
    /// one usable neighborhood.
    fn default_neighborhood() -> NeighborhoodId {
        Self::variants()
            .first()
            .copied()
            .expect("neighborhood set must not be empty")
            .id()
    }
}
