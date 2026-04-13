//! Shared runtime types for CPU neighborhoods.

use hyle_ca_interface::Cell;
use hyle_ca_semantics::Offset3;

/// A single neighbor: its offset, cell value, and precomputed weight.
#[derive(Clone, Copy, Debug)]
pub struct Entry<C: Cell> {
    /// Position relative to the center cell.
    pub offset: Offset3,
    /// The cell value at this offset.
    pub cell: C,
    /// Precomputed influence weight for this offset.
    pub weight: f32,
}
