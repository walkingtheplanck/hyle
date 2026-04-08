//! Boundary behavior for solver coordinate access.

/// How coordinates beyond the grid bounds are interpreted.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Topology {
    /// Coordinates outside the grid are treated as out-of-bounds.
    #[default]
    Bounded,
    /// Coordinates wrap around each axis independently.
    Torus,
}

impl Topology {
    /// Resolve a single coordinate according to this topology.
    ///
    /// Returns `None` when the coordinate is out-of-bounds for
    /// [`Topology::Bounded`] or when the axis size is zero.
    pub fn map_coord(self, coord: i32, size: u32) -> Option<u32> {
        if size == 0 {
            return None;
        }

        match self {
            Topology::Bounded => u32::try_from(coord).ok().filter(|&value| value < size),
            Topology::Torus => {
                let size = i64::from(size);
                let wrapped = i64::from(coord).rem_euclid(size);
                Some(wrapped as u32)
            }
        }
    }
}
