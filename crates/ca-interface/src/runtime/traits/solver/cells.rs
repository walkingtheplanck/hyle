//! Solver cell and neighborhood query capabilities.

use crate::resolved;
use crate::{CellId, CellQueryError, GridAccessError, GridRegion, MaterialId, NeighborhoodId};

use super::{SolverExecution, SolverMetadata};

/// Cell-oriented queries derived from core solver execution.
pub trait SolverCells: SolverExecution + SolverMetadata {
    /// Resolve one logical cell handle from grid coordinates.
    fn cell_at(&self, x: i32, y: i32, z: i32) -> Option<CellId> {
        let index = self.resolve_index(x, y, z);
        (index != self.guard_index()).then(|| CellId::new(index as u32))
    }

    /// Decode a cell handle back into its canonical grid position.
    fn cell_position(&self, cell: CellId) -> Result<[u32; 3], CellQueryError> {
        if cell.index() >= self.cell_count() {
            return Err(CellQueryError::UnknownCell(cell));
        }

        let width = self.width() as usize;
        let height = self.height() as usize;
        let x = cell.index() % width;
        let y = (cell.index() / width) % height;
        let z = cell.index() / (width * height);
        Ok([x as u32, y as u32, z as u32])
    }

    /// Return `true` when the given cell handle belongs to the active solver.
    fn contains_cell(&self, cell: CellId) -> bool {
        self.cell_position(cell).is_ok()
    }

    /// Resolve every logical cell handle in the active grid in x-major order.
    fn cells(&self) -> Vec<CellId> {
        self.cells_in_region(self.dims().as_region())
            .expect("the full solver region must lie within solver dimensions")
    }

    /// Resolve all logical cell handles in one region in x-major order.
    fn cells_in_region(&self, region: GridRegion) -> Result<Vec<CellId>, GridAccessError> {
        let dims = self.dims();
        if !dims.contains_region(region) {
            return Err(GridAccessError::RegionOutOfBounds { region, dims });
        }

        let [ox, oy, oz] = region.origin;
        let [sx, sy, sz] = region.size;
        let mut cells = Vec::with_capacity(region.cell_count());

        for z in oz..oz + sz {
            for y in oy..oy + sy {
                for x in ox..ox + sx {
                    let cell = self
                        .cell_at(x as i32, y as i32, z as i32)
                        .ok_or(GridAccessError::CoordinateUnresolvable { x, y, z })?;
                    cells.push(cell);
                }
            }
        }

        Ok(cells)
    }

    /// Read one material from a resolved cell handle.
    fn material(&self, cell: CellId) -> Result<MaterialId, CellQueryError> {
        let [x, y, z] = self.cell_position(cell)?;
        Ok(self.get(x as i32, y as i32, z as i32))
    }

    /// Resolve all neighbors around one cell for the given neighborhood.
    fn neighbors(
        &self,
        cell: CellId,
        neighborhood: NeighborhoodId,
    ) -> Result<Vec<CellId>, CellQueryError> {
        let spec = self
            .neighborhood_spec(neighborhood)
            .copied()
            .ok_or(CellQueryError::UnknownNeighborhood(neighborhood))?;
        let [x, y, z] = self.cell_position(cell)?;
        let mut cells = Vec::new();

        for offset in resolved::offsets(spec) {
            if let Some(neighbor) = self.cell_at(
                x as i32 + offset.dx,
                y as i32 + offset.dy,
                z as i32 + offset.dz,
            ) {
                cells.push(neighbor);
            }
        }

        Ok(cells)
    }

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
