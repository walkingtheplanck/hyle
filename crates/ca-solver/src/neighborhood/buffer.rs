//! Neighborhood struct - pre-fetched neighbors around a center material.

use hyle_ca_interface::resolved::NeighborhoodSample;
use hyle_ca_interface::MaterialId;

use super::types::Entry;

/// A pre-fetched set of neighbors around a center material.
pub struct Neighborhood {
    center: MaterialId,
    pos: [i32; 3],
    radius: u32,
    entries: Vec<Entry>,
}

impl Neighborhood {
    /// Create a new neighborhood buffer from interpreted semantic samples.
    pub fn new(samples: &[NeighborhoodSample]) -> Self {
        let entries = samples
            .iter()
            .map(|sample| Entry {
                offset: sample.offset(),
                cell: MaterialId::default(),
                weight: sample.weight(),
            })
            .collect();
        Self {
            center: MaterialId::default(),
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
    pub fn fill(
        &mut self,
        center: MaterialId,
        pos: [i32; 3],
        sample: impl Fn(i32, i32, i32) -> MaterialId,
    ) {
        self.center = center;
        self.pos = pos;
        for entry in &mut self.entries {
            entry.cell = sample(entry.offset.dx, entry.offset.dy, entry.offset.dz);
        }
    }

    /// The center material this rule is evaluating.
    pub fn center(&self) -> MaterialId {
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
    pub fn get(&self, dx: i32, dy: i32, dz: i32) -> MaterialId {
        for entry in &self.entries {
            if entry.offset.dx == dx && entry.offset.dy == dy && entry.offset.dz == dz {
                return entry.cell;
            }
        }
        debug_assert!(
            false,
            "offset ({dx},{dy},{dz}) is not part of this neighborhood shape"
        );
        MaterialId::default()
    }

    /// All neighbor entries (offset, material, weight).
    pub fn iter(&self) -> &[Entry] {
        &self.entries
    }

    /// Number of neighbors in this shape at this radius.
    pub fn neighbor_count(&self) -> u32 {
        self.entries.len() as u32
    }

    /// Count neighbors satisfying a predicate.
    pub fn count(&self, pred: impl Fn(&Entry) -> bool) -> u32 {
        self.entries.iter().filter(|e| pred(e)).count() as u32
    }

    /// Weighted sum of neighbors satisfying a predicate.
    pub fn weighted_sum(&self, pred: impl Fn(&Entry) -> bool) -> u64 {
        self.entries
            .iter()
            .filter(|entry| pred(entry))
            .map(|entry| entry.weight as u64)
            .sum()
    }
}
