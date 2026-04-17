//! Solver bulk material-grid capabilities.

use crate::{GridRegion, GridSnapshot, MaterialId};

use super::SolverExecution;

/// Bulk material-grid IO derived from core solver execution.
pub trait SolverGrid: SolverExecution {
    /// Read back all logical materials in x-major order.
    fn iter_cells(&self) -> Vec<(u32, u32, u32, MaterialId)> {
        let dims = self.dims();
        let mut cells = Vec::with_capacity(dims.cell_count());
        for z in 0..dims.depth {
            for y in 0..dims.height {
                for x in 0..dims.width {
                    cells.push((x, y, z, self.get(x as i32, y as i32, z as i32)));
                }
            }
        }
        cells
    }

    /// Read the full current state back to the host.
    fn readback(&self) -> GridSnapshot<MaterialId> {
        let dims = self.dims();
        let mut cells = vec![MaterialId::default(); dims.cell_count()];
        let width = dims.width as usize;
        let height = dims.height as usize;

        for (x, y, z, material) in self.iter_cells() {
            let index = (x as usize) + (y as usize) * width + (z as usize) * width * height;
            cells[index] = material;
        }

        GridSnapshot::new(dims, cells)
    }

    /// Read a contiguous rectangular region in x-major order.
    fn read_region(&self, region: GridRegion) -> Vec<MaterialId> {
        let dims = self.dims();
        assert!(dims.contains_region(region), "region must lie within solver dimensions");

        let mut cells = Vec::with_capacity(region.cell_count());
        let [ox, oy, oz] = region.origin;
        let [sx, sy, sz] = region.size;

        for z in oz..oz + sz {
            for y in oy..oy + sy {
                for x in ox..ox + sx {
                    cells.push(self.get(x as i32, y as i32, z as i32));
                }
            }
        }

        cells
    }

    /// Overwrite a contiguous rectangular region from x-major ordered data.
    fn write_region(&mut self, region: GridRegion, cells: &[MaterialId]) {
        let dims = self.dims();
        assert!(dims.contains_region(region), "region must lie within solver dimensions");
        assert_eq!(
            cells.len(),
            region.cell_count(),
            "region write must provide exactly one cell per destination slot"
        );

        let [ox, oy, oz] = region.origin;
        let [sx, sy, sz] = region.size;
        let mut index = 0;

        for z in oz..oz + sz {
            for y in oy..oy + sy {
                for x in ox..ox + sx {
                    self.set(x as i32, y as i32, z as i32, cells[index]);
                    index += 1;
                }
            }
        }
    }

    /// Replace the full solver state from x-major ordered data.
    fn replace_cells(&mut self, cells: &[MaterialId]) {
        let dims = self.dims();
        assert_eq!(
            cells.len(),
            dims.cell_count(),
            "full-grid replacement must match solver dimensions"
        );
        self.write_region(dims.as_region(), cells);
    }
}
