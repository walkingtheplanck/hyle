//! Dense 3D grid — double-buffered cell storage.

use hyle_ca_core::Cell;

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
        let n = (width * height * depth) as usize;
        Grid {
            width,
            height,
            depth,
            cells: vec![C::default(); n],
            cells_next: vec![C::default(); n],
        }
    }

    /// Linear index from 3D coordinates.
    #[inline]
    pub fn idx(&self, x: u32, y: u32, z: u32) -> usize {
        (x + y * self.width + z * self.width * self.height) as usize
    }

    /// Get cell with bounds checking. Returns `C::default()` for out-of-bounds.
    pub fn get(&self, x: i32, y: i32, z: i32) -> C {
        if (x as u32) >= self.width || (y as u32) >= self.height || (z as u32) >= self.depth {
            return C::default();
        }
        self.cells[self.idx(x as u32, y as u32, z as u32)]
    }

    /// Set cell with bounds checking. No-op for out-of-bounds.
    pub fn set(&mut self, x: i32, y: i32, z: i32, cell: C) {
        if (x as u32) >= self.width || (y as u32) >= self.height || (z as u32) >= self.depth {
            return;
        }
        let i = self.idx(x as u32, y as u32, z as u32);
        self.cells[i] = cell;
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
