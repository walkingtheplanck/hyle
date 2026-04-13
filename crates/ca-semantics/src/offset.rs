/// A relative neighborhood offset in 3D grid space.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Offset3 {
    /// Offset along the x axis.
    pub dx: i32,
    /// Offset along the y axis.
    pub dy: i32,
    /// Offset along the z axis.
    pub dz: i32,
}

impl Offset3 {
    /// Construct a new offset.
    pub const fn new(dx: i32, dy: i32, dz: i32) -> Self {
        Self { dx, dy, dz }
    }
}
