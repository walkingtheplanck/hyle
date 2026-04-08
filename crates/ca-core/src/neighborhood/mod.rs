//! Neighborhood — pre-fetched neighbors around a center cell.
//!
//! Shape and weight are configured via function pointers at construction.
//! Built-in shape functions: [`moore`], [`von_neumann`], [`spherical`].
//! Built-in weight functions: [`unweighted`], [`inverse_square`].

mod shapes;
mod weights;

pub use shapes::{moore, spherical, von_neumann};
pub use weights::{inverse_square, unweighted};

use crate::Cell;

// ---------------------------------------------------------------------------
// Types
// ---------------------------------------------------------------------------

/// Relative position of a neighbor to the center cell.
#[derive(Clone, Copy, Debug)]
pub struct Offset {
    /// X offset from center.
    pub dx: i32,
    /// Y offset from center.
    pub dy: i32,
    /// Z offset from center.
    pub dz: i32,
}

/// A single neighbor: its offset, cell value, and precomputed weight.
#[derive(Clone, Copy, Debug)]
pub struct Entry<C: Cell> {
    /// Position relative to the center cell.
    pub offset: Offset,
    /// The cell value at this offset.
    pub cell: C,
    /// Precomputed influence weight for this offset.
    pub weight: f32,
}

/// Shape function: returns whether offset `(dx, dy, dz)` is included at `radius`.
pub type ShapeFn = fn(dx: i32, dy: i32, dz: i32, radius: u32) -> bool;

/// Weight function: returns the influence weight for offset `(dx, dy, dz)`.
pub type WeightFn = fn(dx: i32, dy: i32, dz: i32) -> f32;

// ---------------------------------------------------------------------------
// Neighborhood
// ---------------------------------------------------------------------------

/// A pre-fetched set of neighbors around a center cell.
///
/// Constructed with a shape function (which offsets to include) and a weight
/// function (influence per offset). The solver calls [`fill`](Neighborhood::fill)
/// once per cell per step. Rules read precomputed values in O(1).
///
/// ```ignore
/// use hyle_ca_core::neighborhood::{Neighborhood, moore, unweighted};
///
/// let mut n = Neighborhood::<u32>::new(1, moore, unweighted);
/// ```
pub struct Neighborhood<C: Cell> {
    center: C,
    pos: [i32; 3],
    radius: u32,
    entries: Vec<Entry<C>>,
    alive_count: u32,
    weighted_sum: f32,
}

impl<C: Cell> Neighborhood<C> {
    /// Create a new neighborhood for the given radius, shape, and weight.
    ///
    /// Precomputes which offsets belong to the shape and the weight of each.
    pub fn new(radius: u32, includes: ShapeFn, weight: WeightFn) -> Self {
        let r = radius as i32;
        let mut entries = Vec::new();
        for dz in -r..=r {
            for dy in -r..=r {
                for dx in -r..=r {
                    if dx == 0 && dy == 0 && dz == 0 {
                        continue;
                    }
                    if includes(dx, dy, dz, radius) {
                        entries.push(Entry {
                            offset: Offset { dx, dy, dz },
                            cell: C::default(),
                            weight: weight(dx, dy, dz),
                        });
                    }
                }
            }
        }
        Neighborhood {
            center: C::default(),
            pos: [0; 3],
            radius,
            entries,
            alive_count: 0,
            weighted_sum: 0.0,
        }
    }

    /// Populate the neighborhood by sampling from the grid.
    ///
    /// Called by the solver once per cell per step. Precomputes
    /// `alive_count` and `weighted_sum` in the same pass.
    pub fn fill(&mut self, center: C, pos: [i32; 3], sample: impl Fn(i32, i32, i32) -> C) {
        self.center = center;
        self.pos = pos;
        self.alive_count = 0;
        self.weighted_sum = 0.0;
        for entry in &mut self.entries {
            entry.cell = sample(entry.offset.dx, entry.offset.dy, entry.offset.dz);
            if entry.cell.is_alive() {
                self.alive_count += 1;
                self.weighted_sum += entry.weight;
            }
        }
    }

    // --- Data access (called by rules) ---

    /// The center cell this rule is evaluating.
    pub fn center(&self) -> C {
        self.center
    }

    /// World-space position of the center cell.
    pub fn pos(&self) -> [i32; 3] {
        self.pos
    }

    /// The radius of this neighborhood.
    pub fn radius(&self) -> u32 {
        self.radius
    }

    /// Get a neighbor by relative offset.
    ///
    /// Panics in debug builds if the offset is outside this shape.
    /// Returns `C::default()` in release builds for out-of-shape offsets.
    pub fn get(&self, dx: i32, dy: i32, dz: i32) -> C {
        for entry in &self.entries {
            if entry.offset.dx == dx && entry.offset.dy == dy && entry.offset.dz == dz {
                return entry.cell;
            }
        }
        debug_assert!(
            false,
            "offset ({dx},{dy},{dz}) is not part of this neighborhood shape"
        );
        C::default()
    }

    /// All neighbor entries (offset, cell, weight).
    pub fn iter(&self) -> &[Entry<C>] {
        &self.entries
    }

    // --- Precomputed (O(1)) ---

    /// Number of neighbors in this shape at this radius.
    pub fn neighbor_count(&self) -> u32 {
        self.entries.len() as u32
    }

    /// Number of alive neighbors. Precomputed during [`Neighborhood::fill`].
    pub fn count_alive(&self) -> u32 {
        self.alive_count
    }

    /// Weighted sum of alive neighbors. Precomputed during [`Neighborhood::fill`].
    ///
    /// With [`unweighted`], this equals `count_alive()` as a float.
    /// With [`inverse_square`], closer neighbors contribute more.
    pub fn weighted_sum(&self) -> f32 {
        self.weighted_sum
    }

    // --- Computed (O(n)) ---

    /// Count neighbors satisfying a predicate.
    pub fn count(&self, pred: impl Fn(&Entry<C>) -> bool) -> u32 {
        self.entries.iter().filter(|e| pred(e)).count() as u32
    }
}
