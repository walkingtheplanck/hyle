//! Tier 1: Neighborhood — radius-1 Moore neighborhood (26 cells).

use super::Cell;

/// All 26 neighbors in a 3×3×3 Moore neighborhood around the center cell.
///
/// Generic over the cell type `C`. Use `get(dx, dy, dz)` to access
/// neighbors by relative offset.
#[derive(Clone, Copy)]
pub struct Neighborhood<C: Cell> {
    pub center: C,
    /// All 26 surrounding cells. Index via `get(dx, dy, dz)`.
    pub neighbors: [C; 26],
}

impl<C: Cell> Neighborhood<C> {
    /// Get a neighbor by relative offset. Each component must be in -1..=1.
    /// Passing (0, 0, 0) is a logic error and will panic in debug builds.
    #[inline]
    pub fn get(&self, dx: i32, dy: i32, dz: i32) -> C {
        debug_assert!(dx != 0 || dy != 0 || dz != 0, "get(0,0,0) is the center cell");
        let flat = (dx + 1) + (dy + 1) * 3 + (dz + 1) * 9;
        let idx = if flat < 13 { flat as usize } else { flat as usize - 1 };
        self.neighbors[idx]
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

    /// Build a Neighborhood by sampling the grid at all 26 offsets.
    /// `sample(dx, dy, dz)` should return the cell at (cx+dx, cy+dy, cz+dz),
    /// returning `C::default()` for out-of-bounds positions.
    #[inline]
    pub fn build(center: C, sample: impl Fn(i32, i32, i32) -> C) -> Self {
        let mut neighbors = [C::default(); 26];
        let mut i = 0;
        for dz in -1i32..=1 {
            for dy in -1i32..=1 {
                for dx in -1i32..=1 {
                    if dx == 0 && dy == 0 && dz == 0 {
                        continue;
                    }
                    neighbors[i] = sample(dx, dy, dz);
                    i += 1;
                }
            }
        }
        Neighborhood { center, neighbors }
    }
}
