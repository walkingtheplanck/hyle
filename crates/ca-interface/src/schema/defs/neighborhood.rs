use crate::schema::{NeighborhoodId, NeighborhoodRef, NeighborhoodSet};

/// Fixed-point scale used for deterministic neighborhood weights.
pub const WEIGHT_SCALE: u32 = 1024;

/// Declarative neighborhood radius wrapper.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct NeighborhoodRadius(u32);

impl NeighborhoodRadius {
    /// Construct a new neighborhood radius.
    pub const fn new(radius: u32) -> Self {
        Self(radius)
    }

    /// Return the raw numeric radius.
    pub const fn get(self) -> u32 {
        self.0
    }
}

/// Declarative description of how a rule samples nearby cells.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct NeighborhoodSpec {
    id: NeighborhoodId,
    name: &'static str,
    shape: NeighborhoodShape,
    radius: NeighborhoodRadius,
    falloff: NeighborhoodFalloff,
}

impl NeighborhoodSpec {
    /// Construct a new neighborhood specification.
    pub fn new<N: NeighborhoodSet>(
        neighborhood: N,
        shape: NeighborhoodShape,
        radius: NeighborhoodRadius,
        falloff: NeighborhoodFalloff,
    ) -> Self {
        Self {
            id: neighborhood.id(),
            name: neighborhood.label(),
            shape,
            radius,
            falloff,
        }
    }

    /// Construct a neighborhood specification from an already typed-erased reference.
    pub const fn from_ref(
        neighborhood: NeighborhoodRef,
        shape: NeighborhoodShape,
        radius: NeighborhoodRadius,
        falloff: NeighborhoodFalloff,
    ) -> Self {
        Self {
            id: neighborhood.id(),
            name: neighborhood.label(),
            shape,
            radius,
            falloff,
        }
    }

    /// Return the neighborhood identifier.
    pub const fn id(&self) -> NeighborhoodId {
        self.id
    }

    /// Return the human-readable neighborhood name.
    pub const fn name(&self) -> &'static str {
        self.name
    }

    /// Return the declared neighborhood shape.
    pub const fn shape(&self) -> NeighborhoodShape {
        self.shape
    }

    /// Return the declared neighborhood radius.
    pub const fn radius(&self) -> NeighborhoodRadius {
        self.radius
    }

    /// Return the declared neighborhood falloff.
    pub const fn falloff(&self) -> NeighborhoodFalloff {
        self.falloff
    }
}

/// Declarative neighborhood shape.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NeighborhoodShape {
    /// All offsets in the surrounding cube.
    Moore,
    /// Only axis-aligned offsets within the radius.
    VonNeumann,
    /// Offsets within a Euclidean sphere.
    Spherical,
}

/// Declarative neighborhood falloff strategy.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NeighborhoodFalloff {
    /// Every included offset has uniform influence.
    Uniform,
    /// Weight falls off as inverse squared distance.
    InverseSquare,
}
