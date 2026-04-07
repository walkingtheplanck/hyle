//! Neighborhood — pre-fetched cells around a center, any radius.

use super::Cell;

/// A pre-fetched cube of cells around a center cell.
///
/// Contains `(2R+1)³ - 1` neighbor cells (the center is excluded).
/// Radius 1 gives the standard 26-cell Moore neighborhood.
/// The solver pre-allocates and reuses the buffer — no per-cell allocation.
///
/// Passed by reference to rules.
pub struct Neighborhood<C: Cell> {
    /// The center cell this rule is evaluating.
    pub center: C,
    /// World-space position of the center cell.
    pub pos: [i32; 3],
    radius: u32,
    side: usize,
    neighbors: Vec<C>,
}

impl<C: Cell> Neighborhood<C> {
    /// Create a new neighborhood buffer for the given radius.
    pub fn new(radius: u32) -> Self {
        let side = (2 * radius + 1) as usize;
        let total = side * side * side - 1;
        Neighborhood {
            center: C::default(),
            pos: [0; 3],
            radius,
            side,
            neighbors: vec![C::default(); total],
        }
    }

    /// Resize the internal buffer if the radius changed.
    pub fn resize(&mut self, radius: u32) {
        if self.radius == radius {
            return;
        }
        self.radius = radius;
        self.side = (2 * radius + 1) as usize;
        let total = self.side * self.side * self.side - 1;
        self.neighbors.resize(total, C::default());
    }

    /// Fill the neighborhood by sampling from the grid.
    pub fn fill(&mut self, center: C, pos: [i32; 3], sample: impl Fn(i32, i32, i32) -> C) {
        self.center = center;
        self.pos = pos;
        let r = self.radius as i32;
        let mut i = 0;
        for dz in -r..=r {
            for dy in -r..=r {
                for dx in -r..=r {
                    if dx == 0 && dy == 0 && dz == 0 {
                        continue;
                    }
                    self.neighbors[i] = sample(dx, dy, dz);
                    i += 1;
                }
            }
        }
    }

    /// Get a neighbor by relative offset. Each component must satisfy `|d| <= radius`.
    /// Panics in debug builds if the offset exceeds the radius.
    #[inline]
    pub fn get(&self, dx: i32, dy: i32, dz: i32) -> C {
        debug_assert!(
            dx.unsigned_abs() <= self.radius
                && dy.unsigned_abs() <= self.radius
                && dz.unsigned_abs() <= self.radius,
            "offset ({dx},{dy},{dz}) exceeds neighborhood radius {}",
            self.radius
        );
        debug_assert!(
            dx != 0 || dy != 0 || dz != 0,
            "get(0,0,0) is the center cell"
        );
        let r = self.radius as i32;
        let flat = (dx + r) as usize
            + (dy + r) as usize * self.side
            + (dz + r) as usize * self.side * self.side;
        let center_flat = self.radius as usize * (1 + self.side + self.side * self.side);
        let idx = if flat < center_flat { flat } else { flat - 1 };
        self.neighbors[idx]
    }

    /// The radius of this neighborhood.
    #[inline]
    pub fn radius(&self) -> u32 {
        self.radius
    }

    /// Count neighbors satisfying `pred`.
    #[inline]
    pub fn count(&self, pred: impl Fn(C) -> bool) -> u32 {
        self.neighbors.iter().filter(|&&c| pred(c)).count() as u32
    }

    /// Count neighbors where `is_alive()` returns true.
    #[inline]
    pub fn count_alive(&self) -> u32 {
        self.neighbors.iter().filter(|c| c.is_alive()).count() as u32
    }
}
