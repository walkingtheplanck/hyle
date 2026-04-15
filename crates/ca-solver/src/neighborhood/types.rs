//! Shared runtime types for CPU neighborhoods.

use hyle_ca_interface::semantics::Offset3;
use hyle_ca_interface::MaterialId;

/// A single neighbor: its offset, cell value, and precomputed weight.
#[derive(Clone, Copy, Debug)]
pub struct Entry {
    /// Position relative to the center cell.
    pub offset: Offset3,
    /// The material value at this offset.
    pub cell: MaterialId,
    /// Precomputed fixed-point influence weight for this offset.
    pub weight: u32,
}
