//! Runtime material-state IO capabilities.

use crate::{GridRegion, GridSnapshot, MaterialId};

/// Material-grid IO exposed by a live runtime.
pub trait RuntimeGrid {
    /// Set a material at the given coordinate.
    fn set(&mut self, x: i32, y: i32, z: i32, material: MaterialId);

    /// Read a contiguous rectangular region in x-major order.
    fn read_region(&self, region: GridRegion) -> Vec<MaterialId>;

    /// Overwrite a contiguous rectangular region from x-major ordered data.
    fn write_region(&mut self, region: GridRegion, cells: &[MaterialId]);

    /// Replace the full current state from x-major ordered data.
    fn replace_cells(&mut self, cells: &[MaterialId]);

    /// Read the full current state back to the host.
    fn readback(&self) -> GridSnapshot<MaterialId>;
}
