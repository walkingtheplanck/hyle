//! Dense 3D grid - double-buffered cell storage.

use hyle_ca_contracts::{Cell, GridDims, Topology};

/// Dense 3D grid with double buffering for order-independent stepping.
pub(crate) struct Grid<C: Cell> {
    pub width: u32,
    pub height: u32,
    pub depth: u32,
    pub cells: Vec<C>,
    pub cells_next: Vec<C>,
}

impl<C: Cell> Grid<C> {
    /// Create a grid filled with `C::default()`.
    pub fn new(width: u32, height: u32, depth: u32) -> Self {
        // Coordinate resolution starts from `i32` positions. Keeping each axis
        // within `i32::MAX` lets bounded topology use the cast-and-compare path
        // safely: any negative `i32` becomes a `u32` value that is necessarily
        // larger than every valid dimension and therefore rejected.
        let max_dim = i32::MAX as u32;
        assert!(width <= max_dim, "width must be <= i32::MAX");
        assert!(height <= max_dim, "height must be <= i32::MAX");
        assert!(depth <= max_dim, "depth must be <= i32::MAX");

        let n = (width as usize)
            .checked_mul(height as usize)
            .and_then(|xy| xy.checked_mul(depth as usize))
            .expect("grid cell count must fit in usize");
        let total = n.checked_add(1).expect("grid allocation size overflow");
        Grid {
            width,
            height,
            depth,
            cells: vec![C::default(); total],
            cells_next: vec![C::default(); total],
        }
    }

    /// Number of logical cells in the grid.
    #[inline]
    pub fn cell_count(&self) -> usize {
        self.cells.len() - 1
    }

    /// Index of the dedicated guard cell.
    #[inline]
    pub fn guard_idx(&self) -> usize {
        self.cell_count()
    }

    /// Grid dimensions.
    #[inline]
    pub fn dims(&self) -> GridDims {
        GridDims::new(self.width, self.height, self.depth)
    }

    /// Resolve coordinates to an in-bounds linear index according to topology.
    #[inline]
    pub fn resolve_idx<T: Topology>(&self, topology: &T, x: i32, y: i32, z: i32) -> usize {
        resolve_index(topology, self.dims(), self.guard_idx(), x, y, z)
    }

    /// Get a cell from the current buffer according to topology.
    pub fn get<T: Topology>(&self, topology: &T, x: i32, y: i32, z: i32) -> C {
        self.get_from_slice(&self.cells, topology, x, y, z)
    }

    /// Get a cell from an arbitrary backing slice according to topology.
    pub fn get_from_slice<T: Topology>(
        &self,
        cells: &[C],
        topology: &T,
        x: i32,
        y: i32,
        z: i32,
    ) -> C {
        let index = self.resolve_idx(topology, x, y, z);
        cells[index]
    }

    /// Set a cell in the current buffer according to topology.
    pub fn set<T: Topology>(&mut self, topology: &T, x: i32, y: i32, z: i32, cell: C) {
        let index = self.resolve_idx(topology, x, y, z);
        if index != self.guard_idx() {
            self.cells[index] = cell;
        }
    }

    /// Copy current cells to next buffer, preparing for a step.
    pub fn prepare_step(&mut self) {
        self.cells_next.copy_from_slice(&self.cells);
    }

    /// Swap current and next buffers, completing a step.
    pub fn swap(&mut self) {
        std::mem::swap(&mut self.cells, &mut self.cells_next);
    }

    /// Iterate all cells as `(x, y, z, cell)`.
    pub fn iter_cells(&self) -> Vec<(u32, u32, u32, C)> {
        let w = self.width;
        let h = self.height;
        self.cells
            .iter()
            .take(self.cell_count())
            .enumerate()
            .map(move |(i, &c)| {
                let x = (i as u32) % w;
                let y = ((i as u32) / w) % h;
                let z = (i as u32) / (w * h);
                (x, y, z, c)
            })
            .collect()
    }
}

#[allow(clippy::too_many_arguments)]
pub(crate) fn resolve_index<T: Topology>(
    topology: &T,
    dims: GridDims,
    guard_idx: usize,
    x: i32,
    y: i32,
    z: i32,
) -> usize {
    topology.resolve_index(x, y, z, dims, guard_idx)
}
