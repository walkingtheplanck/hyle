//! Shared runtime grid and region access error types.

use crate::{GridDims, GridRegion};

/// Errors raised by host-side region queries and bulk grid IO.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GridAccessError {
    /// The requested region extends outside the active grid dimensions.
    RegionOutOfBounds {
        /// Region requested by the caller.
        region: GridRegion,
        /// Active grid dimensions used for validation.
        dims: GridDims,
    },
    /// A coordinate inside a validated region failed to resolve to a cell handle.
    CoordinateUnresolvable {
        /// X coordinate that failed to resolve.
        x: u32,
        /// Y coordinate that failed to resolve.
        y: u32,
        /// Z coordinate that failed to resolve.
        z: u32,
    },
    /// The provided host-side cell slice length does not match the destination size.
    CellCountMismatch {
        /// Number of cells the operation expected.
        expected: usize,
        /// Number of cells the caller provided.
        actual: usize,
    },
}
