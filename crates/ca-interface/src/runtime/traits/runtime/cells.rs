//! Runtime cell and neighborhood query capabilities.

use crate::{CellId, CellQueryError, GridAccessError, GridRegion, MaterialId, NeighborhoodId};

use super::RuntimeMetadata;

/// Cell-oriented queries exposed by a live runtime.
pub trait RuntimeCells: RuntimeMetadata {
    /// Resolve one logical cell handle from grid coordinates.
    fn cell_at(&self, x: i32, y: i32, z: i32) -> Option<CellId>;

    /// Decode a cell handle back into its canonical grid position.
    fn cell_position(&self, cell: CellId) -> Result<[u32; 3], CellQueryError>;

    /// Return `true` when the given cell handle belongs to the active runtime.
    fn contains_cell(&self, cell: CellId) -> bool {
        self.cell_position(cell).is_ok()
    }

    /// Resolve every logical cell handle in the active grid in x-major order.
    ///
    /// This is a convenience wrapper over [`RuntimeCells::cells_in_region`] for
    /// the full active grid, so it now shares the same error surface instead of
    /// assuming full-grid region resolution can never fail.
    fn cells(&self) -> Result<Vec<CellId>, GridAccessError> {
        self.cells_in_region(self.dims().as_region())
    }

    /// Resolve all logical cell handles in one region in x-major order.
    fn cells_in_region(&self, region: GridRegion) -> Result<Vec<CellId>, GridAccessError>;

    /// Read one material from a resolved cell handle.
    fn material(&self, cell: CellId) -> Result<MaterialId, CellQueryError>;

    /// Resolve all neighbors around one cell for the given neighborhood.
    fn neighbors(
        &self,
        cell: CellId,
        neighborhood: NeighborhoodId,
    ) -> Result<Vec<CellId>, CellQueryError>;

    /// Resolve all neighbors and read their current materials.
    fn neighbor_materials(
        &self,
        cell: CellId,
        neighborhood: NeighborhoodId,
    ) -> Result<Vec<(CellId, MaterialId)>, CellQueryError> {
        self.neighbors(cell, neighborhood)?
            .into_iter()
            .map(|neighbor| Ok((neighbor, self.material(neighbor)?)))
            .collect()
    }
}
