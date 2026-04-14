//! Neighborhood struct - pre-fetched neighbors around a center cell.

use hyle_ca_interface::semantics::NeighborhoodSample;
use hyle_ca_interface::Cell;

use super::types::Entry;

/// A pre-fetched set of neighbors around a center cell.
///
/// Constructed from interpreted semantic neighborhood samples. The CPU solver
/// calls [`fill`](Neighborhood::fill) once per cell per step. Rules then read
/// precomputed values in O(1).
///
/// ```rust
/// use hyle_ca_interface::{Cell, NeighborhoodFalloff, NeighborhoodShape, NeighborhoodSpec};
/// use hyle_ca_interface::semantics::expand_neighborhood;
/// use hyle_ca_solver::Neighborhood;
///
/// #[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
/// struct TestCell(u32);
///
/// let semantic = expand_neighborhood(NeighborhoodSpec::new(
///     NeighborhoodShape::Moore,
///     1,
///     NeighborhoodFalloff::Uniform,
/// ));
/// let mut n = Neighborhood::<TestCell>::new(semantic.samples());
/// ```
pub struct Neighborhood<C: Cell> {
    center: C,
    pos: [i32; 3],
    radius: u32,
    entries: Vec<Entry<C>>,
}

impl<C: Cell> Neighborhood<C> {
    /// Create a new neighborhood buffer from interpreted semantic samples.
    pub fn new(samples: &[NeighborhoodSample]) -> Self {
        let entries = samples
            .iter()
            .map(|sample| Entry {
                offset: sample.offset(),
                cell: C::default(),
                weight: sample.weight(),
            })
            .collect();
        Neighborhood {
            center: C::default(),
            pos: [0; 3],
            radius: samples
                .iter()
                .map(|sample| {
                    let offset = sample.offset();
                    offset
                        .dx
                        .unsigned_abs()
                        .max(offset.dy.unsigned_abs())
                        .max(offset.dz.unsigned_abs())
                })
                .max()
                .unwrap_or(0),
            entries,
        }
    }

    /// Populate the neighborhood by sampling from the grid.
    pub fn fill(&mut self, center: C, pos: [i32; 3], sample: impl Fn(i32, i32, i32) -> C) {
        self.center = center;
        self.pos = pos;
        for entry in &mut self.entries {
            entry.cell = sample(entry.offset.dx, entry.offset.dy, entry.offset.dz);
        }
    }

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

    /// Number of neighbors in this shape at this radius.
    pub fn neighbor_count(&self) -> u32 {
        self.entries.len() as u32
    }

    /// Count neighbors satisfying a predicate.
    pub fn count(&self, pred: impl Fn(&Entry<C>) -> bool) -> u32 {
        self.entries.iter().filter(|e| pred(e)).count() as u32
    }

    /// Weighted sum of neighbors satisfying a predicate.
    pub fn weighted_sum(&self, pred: impl Fn(&Entry<C>) -> bool) -> u64 {
        self.entries
            .iter()
            .filter(|entry| pred(entry))
            .map(|entry| entry.weight as u64)
            .sum()
    }
}
