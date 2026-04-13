/// Declarative description of how a rule samples nearby cells.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct NeighborhoodSpec {
    shape: NeighborhoodShape,
    radius: u32,
    falloff: NeighborhoodFalloff,
}

impl NeighborhoodSpec {
    /// Construct a new neighborhood specification.
    pub const fn new(shape: NeighborhoodShape, radius: u32, falloff: NeighborhoodFalloff) -> Self {
        Self {
            shape,
            radius,
            falloff,
        }
    }

    /// Construct the standard adjacent neighborhood: radius-1 Moore, unweighted.
    pub const fn adjacent() -> Self {
        Self::new(NeighborhoodShape::Moore, 1, NeighborhoodFalloff::Uniform)
    }

    /// Return the declared neighborhood shape.
    pub const fn shape(&self) -> NeighborhoodShape {
        self.shape
    }

    /// Return the declared neighborhood radius.
    pub const fn radius(&self) -> u32 {
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
