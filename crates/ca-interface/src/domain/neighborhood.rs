/// Fixed-point scale used for deterministic neighborhood weights.
pub const WEIGHT_SCALE: u32 = 1024;

/// Shared neighborhood radius wrapper.
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
