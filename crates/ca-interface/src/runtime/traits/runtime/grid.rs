//! Runtime material-state IO capabilities.

use crate::{GridAccessError, GridRegion, GridSnapshot, MaterialId};

/// Material-grid IO exposed by a live runtime.
pub trait RuntimeGrid {
    /// Set a material at the given coordinate.
    ///
    /// This low-level write follows the runtime's topology semantics and keeps
    /// the solver-style "out of bounds may become a no-op" behavior.
    fn set(&mut self, x: i32, y: i32, z: i32, material: MaterialId);

    /// Read a contiguous rectangular region in x-major order.
    ///
    /// # Errors
    ///
    /// Returns [`GridAccessError`] when the requested region is outside the
    /// active grid.
    fn read_region(&self, region: GridRegion) -> Result<Vec<MaterialId>, GridAccessError>;

    /// Overwrite a contiguous rectangular region from x-major ordered data.
    ///
    /// # Errors
    ///
    /// Returns [`GridAccessError`] when the region is out of bounds or the
    /// provided slice length does not match the region volume.
    fn write_region(
        &mut self,
        region: GridRegion,
        cells: &[MaterialId],
    ) -> Result<(), GridAccessError>;

    /// Replace the full current state from x-major ordered data.
    ///
    /// # Errors
    ///
    /// Returns [`GridAccessError::CellCountMismatch`] when `cells` does not
    /// contain exactly one material for every logical grid position.
    fn replace_cells(&mut self, cells: &[MaterialId]) -> Result<(), GridAccessError>;

    /// Read the full current state back to the host.
    ///
    /// The snapshot owns a dense x-major material buffer suitable for testing,
    /// serialization, or UI upload.
    fn readback(&self) -> GridSnapshot;
}
