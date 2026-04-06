//! Direction — the six face-adjacent directions in 3D.

/// The six face-adjacent directions in 3D.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Direction {
    PosX,
    NegX,
    PosY,
    NegY,
    PosZ,
    NegZ,
}

impl Direction {
    /// Convert to a (dx, dy, dz) offset.
    #[inline]
    pub fn offset(self) -> (i32, i32, i32) {
        match self {
            Direction::PosX => (1, 0, 0),
            Direction::NegX => (-1, 0, 0),
            Direction::PosY => (0, 1, 0),
            Direction::NegY => (0, -1, 0),
            Direction::PosZ => (0, 0, 1),
            Direction::NegZ => (0, 0, -1),
        }
    }
}
