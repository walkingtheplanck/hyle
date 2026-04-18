use crate::WEIGHT_SCALE;

/// Fixed-point portable weight used by weighted neighborhood predicates.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Weight(u64);

impl Weight {
    /// Construct a weight from raw fixed-point units.
    ///
    /// This exists for advanced callers that already speak the portable
    /// fixed-point representation used by weighted neighborhoods.
    pub const fn raw(units: u64) -> Self {
        Self(units)
    }

    /// Construct the weight corresponding to `cells` uniform neighbors.
    ///
    /// This is usually the ergonomic entry point when authoring rules against
    /// uniform neighborhoods.
    pub const fn cells(cells: u32) -> Self {
        Self(cells as u64 * WEIGHT_SCALE as u64)
    }

    /// Return the raw fixed-point value.
    pub const fn units(self) -> u64 {
        self.0
    }
}
