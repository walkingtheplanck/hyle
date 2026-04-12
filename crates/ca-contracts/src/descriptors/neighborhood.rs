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

    /// Return whether this neighborhood uses a weighted falloff.
    pub const fn is_weighted(&self) -> bool {
        !matches!(self.falloff, NeighborhoodFalloff::Uniform)
    }

    /// Return the number of neighbor positions included by this specification.
    pub const fn neighbor_count(&self) -> u32 {
        self.shape.neighbor_count(self.radius)
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

impl NeighborhoodShape {
    /// Return whether the offset belongs to this shape at the given radius.
    pub const fn includes(self, dx: i32, dy: i32, dz: i32, radius: u32) -> bool {
        if dx == 0 && dy == 0 && dz == 0 {
            return false;
        }

        let radius = radius as i32;

        match self {
            Self::Moore => true,
            Self::VonNeumann => dx.abs() + dy.abs() + dz.abs() <= radius,
            Self::Spherical => dx * dx + dy * dy + dz * dz <= radius * radius,
        }
    }

    /// Return the number of neighbor positions included by this shape at the given radius.
    pub const fn neighbor_count(self, radius: u32) -> u32 {
        let radius = radius as i32;
        let mut count = 0u32;
        let mut dz = -radius;

        while dz <= radius {
            let mut dy = -radius;
            while dy <= radius {
                let mut dx = -radius;
                while dx <= radius {
                    if self.includes(dx, dy, dz, radius as u32) {
                        count += 1;
                    }

                    dx += 1;
                }
                dy += 1;
            }
            dz += 1;
        }

        count
    }
}

/// Declarative neighborhood falloff strategy.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NeighborhoodFalloff {
    /// Every included offset has uniform influence.
    Uniform,
    /// Weight falls off as inverse squared distance.
    InverseSquare,
}
