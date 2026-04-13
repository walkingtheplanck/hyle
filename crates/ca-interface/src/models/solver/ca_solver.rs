//! Solver trait - the common interface all CA solvers implement.

use crate::{Cell, GridDims, GridRegion, GridSnapshot, Topology};

/// The common interface shared by all CA solvers (CPU, GPU, etc.).
///
/// Blueprint authoring is intentionally kept out of this trait. Solvers are
/// free to consume declarative specs, interpreted blueprints, precompiled
/// programs, or other portable representations as long as they uphold the
/// runtime contract below.
///
/// Contracts (enforced by `ValidatedSolver` in debug builds):
/// - `get(x,y,z)` and `set(x,y,z,...)` follow `resolve_index(...)`.
/// - If `resolve_index(...)` returns `guard_index()`, `get(...)` returns
///   `C::default()` and `set(...)` is a silent no-op.
/// - If `resolve_index(...)` returns an index other than `guard_index()`,
///   `get(...)` and `set(...)` must behave as if they were applied to that
///   resolved in-bounds cell index.
/// - `step()` increments `step_count()` by exactly 1.
/// - `width()`, `height()`, `depth()` never change after construction.
/// - After `set(...)`, `write_region(...)`, `replace_cells(...)`, or `step()`
///   returns, subsequent reads through `get(...)`, `iter_cells()`,
///   `read_region(...)`, and `readback()` must observe the latest state.
///   GPU solvers may block internally to satisfy this contract.
pub trait CaSolver<C: Cell> {
    /// Topology policy used by this solver.
    type Topology: Topology;

    /// Grid width in cells.
    fn width(&self) -> u32;
    /// Grid height in cells.
    fn height(&self) -> u32;
    /// Grid depth in cells.
    fn depth(&self) -> u32;
    /// Grid dimensions.
    fn dims(&self) -> GridDims {
        GridDims::new(self.width(), self.height(), self.depth())
    }
    /// Topology policy used to resolve coordinates for reads, writes, and steps.
    fn topology(&self) -> &Self::Topology;

    /// Number of logical cells in the current grid.
    fn cell_count(&self) -> usize {
        self.dims().cell_count()
    }

    /// One-past-the-end logical cell index used as the "no cell" sentinel.
    fn guard_index(&self) -> usize {
        self.cell_count()
    }

    /// Resolve a possibly out-of-range coordinate to a linear cell index.
    ///
    /// The default implementation delegates to the solver's topology policy.
    fn resolve_index(&self, x: i32, y: i32, z: i32) -> usize {
        self.topology()
            .resolve_index(x, y, z, self.dims(), self.guard_index())
    }

    /// Get the cell at (x, y, z) according to `resolve_index(...)`.
    fn get(&self, x: i32, y: i32, z: i32) -> C;

    /// Set the cell at (x, y, z) according to `resolve_index(...)`.
    fn set(&mut self, x: i32, y: i32, z: i32, cell: C);

    /// Advance the blueprint by one step.
    fn step(&mut self);

    /// Number of steps completed so far.
    fn step_count(&self) -> u32;

    /// Iterate all cells as `(x, y, z, cell)`.
    ///
    /// For GPU solvers, this may trigger a device-to-host download.
    /// The returned Vec is owned - no lifetime issues across solvers.
    fn iter_cells(&self) -> Vec<(u32, u32, u32, C)>;

    /// Download the full current solver state as a contiguous snapshot.
    ///
    /// Cells are returned in x-major order: x changes fastest, then y, then z.
    fn readback(&self) -> GridSnapshot<C> {
        let dims = self.dims();
        let mut cells = vec![C::default(); dims.cell_count()];
        let width = dims.width as usize;
        let height = dims.height as usize;

        for (x, y, z, cell) in self.iter_cells() {
            let index = (x as usize) + (y as usize) * width + (z as usize) * width * height;
            cells[index] = cell;
        }

        GridSnapshot::new(dims, cells)
    }

    /// Read a contiguous rectangular region in x-major order.
    fn read_region(&self, region: GridRegion) -> Vec<C> {
        let dims = self.dims();
        assert!(
            dims.contains_region(region),
            "region must lie within solver dimensions"
        );

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
    fn write_region(&mut self, region: GridRegion, cells: &[C]) {
        let dims = self.dims();
        assert!(
            dims.contains_region(region),
            "region must lie within solver dimensions"
        );
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
    fn replace_cells(&mut self, cells: &[C]) {
        let dims = self.dims();
        assert_eq!(
            cells.len(),
            dims.cell_count(),
            "full-grid replacement must match solver dimensions"
        );
        self.write_region(dims.as_region(), cells);
    }
}
