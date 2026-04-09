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
