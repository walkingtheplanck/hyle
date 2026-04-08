//! Dense 3D grid - double-buffered cell storage.

use hyle_ca_core::{Cell, Topology};

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

    /// Resolve coordinates to an in-bounds linear index according to topology.
    #[inline]
    pub fn resolve_idx(&self, topology: Topology, x: i32, y: i32, z: i32) -> Option<usize> {
        let (x, y, z) = resolve_coord(topology, self.width, self.height, self.depth, x, y, z)?;
        Some(self.idx(x, y, z))
    }

    /// Get a cell from the current buffer according to topology.
    pub fn get(&self, topology: Topology, x: i32, y: i32, z: i32) -> C {
        self.get_from_slice(&self.cells, topology, x, y, z)
    }

    /// Get a cell from an arbitrary backing slice according to topology.
    pub fn get_from_slice(&self, cells: &[C], topology: Topology, x: i32, y: i32, z: i32) -> C {
        match self.resolve_idx(topology, x, y, z) {
            Some(index) => cells[index],
            None => C::default(),
        }
    }

    /// Set a cell in the current buffer according to topology.
    pub fn set(&mut self, topology: Topology, x: i32, y: i32, z: i32, cell: C) {
        if let Some(index) = self.resolve_idx(topology, x, y, z) {
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

pub(crate) fn resolve_coord(
    topology: Topology,
    width: u32,
    height: u32,
    depth: u32,
    x: i32,
    y: i32,
    z: i32,
) -> Option<(u32, u32, u32)> {
    Some((
        topology.map_coord(x, width)?,
        topology.map_coord(y, height)?,
        topology.map_coord(z, depth)?,
    ))
}
