//! Dense 3D grid - double-buffered material storage.

use hyle_ca_interface::{GridDims, GridShapeError, MaterialId, Topology};

/// Dense 3D grid with double buffering for order-independent stepping.
pub(crate) struct Grid {
    pub width: u32,
    pub height: u32,
    pub depth: u32,
    dims: GridDims,
    pub cells: Vec<MaterialId>,
    pub cells_next: Vec<MaterialId>,
}

impl Grid {
    /// Create a grid filled with the default material id.
    pub fn new(
        width: u32,
        height: u32,
        depth: u32,
        default_material: MaterialId,
    ) -> Result<Self, GridShapeError> {
        let dims = GridDims::new(width, height, depth)?;
        Ok(Self::from_dims(dims, default_material))
    }

    /// Create a grid from already validated dimensions.
    pub fn from_dims(dims: GridDims, default_material: MaterialId) -> Self {
        let total = dims.cell_count() + 1;
        Self {
            width: dims.width(),
            height: dims.height(),
            depth: dims.depth(),
            dims,
            cells: vec![default_material; total],
            cells_next: vec![default_material; total],
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
        self.dims
    }

    /// Resolve coordinates to an in-bounds linear index according to topology.
    #[inline]
    pub fn resolve_idx<T: Topology>(&self, topology: &T, x: i32, y: i32, z: i32) -> usize {
        resolve_index(topology, self.dims(), self.guard_idx(), x, y, z)
    }

    /// Get a material from the current buffer according to topology.
    pub fn get<T: Topology>(&self, topology: &T, x: i32, y: i32, z: i32) -> MaterialId {
        let index = self.resolve_idx(topology, x, y, z);
        self.cells[index]
    }

    /// Set a material in the current buffer according to topology.
    pub fn set<T: Topology>(&mut self, topology: &T, x: i32, y: i32, z: i32, material: MaterialId) {
        let index = self.resolve_idx(topology, x, y, z);
        if index != self.guard_idx() {
            self.cells[index] = material;
        }
    }

    /// Copy current materials to the next buffer, preparing for a step.
    pub fn prepare_step(&mut self) {
        self.cells_next.copy_from_slice(&self.cells);
    }

    /// Swap current and next buffers, completing a step.
    pub fn swap(&mut self) {
        std::mem::swap(&mut self.cells, &mut self.cells_next);
    }

    /// Iterate all materials as `(x, y, z, material)`.
    pub fn iter_cells(&self) -> Vec<(u32, u32, u32, MaterialId)> {
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
