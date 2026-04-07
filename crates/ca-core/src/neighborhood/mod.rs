//! Neighborhood trait and built-in shapes: Moore, Von Neumann, Spherical.

mod moore;
mod spherical;
mod von_neumann;

pub use moore::MooreNeighborhood;
pub use spherical::SphericalNeighborhood;
pub use von_neumann::VonNeumannNeighborhood;

use crate::Cell;

/// A neighborhood of cells around a center cell.
///
/// Implement this trait to define custom neighborhood shapes. The five required
/// methods provide data access; counting and weighting are default methods
/// built on top of [`iter`](Neighborhood::iter).
pub trait Neighborhood<C: Cell> {
    /// The center cell this rule is evaluating.
    fn center(&self) -> C;

    /// World-space position of the center cell.
    fn pos(&self) -> [i32; 3];

    /// The radius of this neighborhood.
    fn radius(&self) -> u32;

    /// Get a neighbor by relative offset.
    ///
    /// Panics in debug builds if the offset is outside this shape.
    /// Returns `C::default()` in release builds for out-of-shape offsets.
    fn get(&self, dx: i32, dy: i32, dz: i32) -> C;

    /// All neighbors as `(dx, dy, dz, cell)` tuples.
    fn iter(&self) -> &[(i32, i32, i32, C)];

    /// Number of neighbors in this neighborhood.
    fn neighbor_count(&self) -> u32 {
        self.iter().len() as u32
    }

    /// Count neighbors satisfying a predicate.
    fn count(&self, pred: &dyn Fn(C) -> bool) -> u32 {
        self.iter().iter().filter(|(_, _, _, c)| pred(*c)).count() as u32
    }

    /// Count neighbors where `is_alive()` returns true.
    fn count_alive(&self) -> u32 {
        self.iter()
            .iter()
            .filter(|(_, _, _, c)| c.is_alive())
            .count() as u32
    }

    /// Weighted sum of alive neighbors. The weight function receives the
    /// Euclidean distance from the center to each alive neighbor.
    fn count_weighted(&self, weight_fn: &dyn Fn(f32) -> f32) -> f32 {
        self.iter()
            .iter()
            .filter(|(_, _, _, c)| c.is_alive())
            .map(|&(dx, dy, dz, _)| {
                let d = ((dx * dx + dy * dy + dz * dz) as f32).sqrt();
                weight_fn(d)
            })
            .sum()
    }
}

/// Inverse square weight function (default for 3D): `1 / d²`.
pub fn inverse_square(distance: f32) -> f32 {
    1.0 / (distance * distance)
}

// ---------------------------------------------------------------------------
// Shared internals
// ---------------------------------------------------------------------------

/// Internal storage shared by all built-in neighborhood types.
pub(crate) struct NeighborhoodData<C: Cell> {
    center: C,
    pos: [i32; 3],
    radius: u32,
    offsets: Vec<(i32, i32, i32)>,
    entries: Vec<(i32, i32, i32, C)>,
}

impl<C: Cell> NeighborhoodData<C> {
    pub(crate) fn new(radius: u32, include: impl Fn(i32, i32, i32, u32) -> bool) -> Self {
        let offsets = compute_offsets(radius, &include);
        let len = offsets.len();
        NeighborhoodData {
            center: C::default(),
            pos: [0; 3],
            radius,
            offsets,
            entries: vec![(0, 0, 0, C::default()); len],
        }
    }

    pub(crate) fn resize(&mut self, radius: u32, include: impl Fn(i32, i32, i32, u32) -> bool) {
        if self.radius == radius {
            return;
        }
        self.radius = radius;
        self.offsets = compute_offsets(radius, &include);
        self.entries
            .resize(self.offsets.len(), (0, 0, 0, C::default()));
    }

    pub(crate) fn fill(&mut self, center: C, pos: [i32; 3], sample: impl Fn(i32, i32, i32) -> C) {
        self.center = center;
        self.pos = pos;
        for (i, &(dx, dy, dz)) in self.offsets.iter().enumerate() {
            self.entries[i] = (dx, dy, dz, sample(dx, dy, dz));
        }
    }

    pub(crate) fn get(&self, dx: i32, dy: i32, dz: i32) -> C {
        for &(ox, oy, oz, c) in &self.entries {
            if ox == dx && oy == dy && oz == dz {
                return c;
            }
        }
        debug_assert!(
            false,
            "offset ({dx},{dy},{dz}) is not part of this neighborhood shape"
        );
        C::default()
    }
}

fn compute_offsets(
    radius: u32,
    include: &dyn Fn(i32, i32, i32, u32) -> bool,
) -> Vec<(i32, i32, i32)> {
    let r = radius as i32;
    let mut offsets = Vec::new();
    for dz in -r..=r {
        for dy in -r..=r {
            for dx in -r..=r {
                if dx == 0 && dy == 0 && dz == 0 {
                    continue;
                }
                if include(dx, dy, dz, radius) {
                    offsets.push((dx, dy, dz));
                }
            }
        }
    }
    offsets
}

macro_rules! impl_neighborhood {
    ($ty:ident, $include_fn:expr) => {
        impl<C: Cell> $ty<C> {
            /// Create a new neighborhood buffer for the given radius.
            pub fn new(radius: u32) -> Self {
                $ty(NeighborhoodData::new(radius, $include_fn))
            }

            /// Resize the internal buffer if the radius changed.
            pub fn resize(&mut self, radius: u32) {
                self.0.resize(radius, $include_fn);
            }

            /// Fill the neighborhood by sampling from the grid.
            pub fn fill(&mut self, center: C, pos: [i32; 3], sample: impl Fn(i32, i32, i32) -> C) {
                self.0.fill(center, pos, sample);
            }
        }

        impl<C: Cell> Neighborhood<C> for $ty<C> {
            fn center(&self) -> C {
                self.0.center
            }
            fn pos(&self) -> [i32; 3] {
                self.0.pos
            }
            fn radius(&self) -> u32 {
                self.0.radius
            }
            fn get(&self, dx: i32, dy: i32, dz: i32) -> C {
                self.0.get(dx, dy, dz)
            }
            fn iter(&self) -> &[(i32, i32, i32, C)] {
                &self.0.entries
            }
        }
    };
}

pub(crate) use impl_neighborhood;
