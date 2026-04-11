/// Declarative description of how a rule samples nearby cells.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct NeighborhoodSpec {
    /// Radius, in cells, around the center cell.
    pub radius: u32,
    /// Which offsets are included in the neighborhood.
    pub shape: NeighborhoodShape,
    /// How included offsets contribute to aggregate values.
    pub weight: NeighborhoodWeight,
}

impl NeighborhoodSpec {
    /// Construct a new neighborhood specification.
    pub const fn new(radius: u32, shape: NeighborhoodShape, weight: NeighborhoodWeight) -> Self {
        Self {
            radius,
            shape,
            weight,
        }
    }

    /// Construct the standard adjacent neighborhood: radius-1 Moore, unweighted.
    pub const fn adjacent() -> Self {
        Self::new(1, NeighborhoodShape::Moore, NeighborhoodWeight::Unweighted)
    }

    /// Construct a Moore neighborhood with the given radius.
    pub const fn cube(radius: u32) -> Self {
        Self::new(
            radius,
            NeighborhoodShape::Moore,
            NeighborhoodWeight::Unweighted,
        )
    }

    /// Construct a Von Neumann neighborhood with the given radius.
    pub const fn cross(radius: u32) -> Self {
        Self::new(
            radius,
            NeighborhoodShape::VonNeumann,
            NeighborhoodWeight::Unweighted,
        )
    }

    /// Construct a spherical neighborhood with the given radius.
    pub const fn sphere(radius: u32) -> Self {
        Self::new(
            radius,
            NeighborhoodShape::Spherical,
            NeighborhoodWeight::Unweighted,
        )
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

/// Declarative neighborhood weighting strategy.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NeighborhoodWeight {
    /// Every included offset has weight 1.0.
    Unweighted,
    /// Weight falls off as inverse squared distance.
    InverseSquare,
}
