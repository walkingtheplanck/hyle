use crate::CellState;

/// Immutable grid dimensions in cells.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct GridDims {
    /// Grid width in cells.
    pub width: u32,
    /// Grid height in cells.
    pub height: u32,
    /// Grid depth in cells.
    pub depth: u32,
}

impl GridDims {
    /// Construct a new set of grid dimensions.
    pub const fn new(width: u32, height: u32, depth: u32) -> Self {
        Self {
            width,
            height,
            depth,
        }
    }

    /// Total number of logical cells in the grid.
    pub fn cell_count(self) -> usize {
        (self.width as usize)
            .checked_mul(self.height as usize)
            .and_then(|xy| xy.checked_mul(self.depth as usize))
            .expect("grid cell count must fit in usize")
    }

    /// The full grid expressed as a region starting at the origin.
    pub const fn as_region(self) -> GridRegion {
        GridRegion::new([0, 0, 0], [self.width, self.height, self.depth])
    }

    /// Whether a region lies completely inside these dimensions.
    pub fn contains_region(self, region: GridRegion) -> bool {
        let Some(end) = region.end_exclusive() else {
            return false;
        };

        end[0] <= self.width && end[1] <= self.height && end[2] <= self.depth
    }
}

/// A rectangular subregion of the grid.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct GridRegion {
    /// Origin of the region, inclusive.
    pub origin: [u32; 3],
    /// Extent of the region in cells.
    pub size: [u32; 3],
}

impl GridRegion {
    /// Construct a new region from an origin and size.
    pub const fn new(origin: [u32; 3], size: [u32; 3]) -> Self {
        Self { origin, size }
    }

    /// Total number of cells inside the region.
    pub fn cell_count(self) -> usize {
        (self.size[0] as usize)
            .checked_mul(self.size[1] as usize)
            .and_then(|xy| xy.checked_mul(self.size[2] as usize))
            .expect("grid region cell count must fit in usize")
    }

    /// The exclusive end coordinate of the region, if it does not overflow.
    pub fn end_exclusive(self) -> Option<[u32; 3]> {
        Some([
            self.origin[0].checked_add(self.size[0])?,
            self.origin[1].checked_add(self.size[1])?,
            self.origin[2].checked_add(self.size[2])?,
        ])
    }
}

/// A contiguous host-side snapshot of the current solver state.
///
/// Cells are stored in x-major order: x changes fastest, then y, then z.
#[derive(Clone, Debug)]
pub struct GridSnapshot<C: CellState> {
    /// Dimensions of the captured grid.
    pub dims: GridDims,
    /// Captured cell values in x-major order.
    pub cells: Vec<C>,
}

impl<C: CellState> GridSnapshot<C> {
    /// Construct a validated grid snapshot.
    pub fn new(dims: GridDims, cells: Vec<C>) -> Self {
        assert_eq!(
            cells.len(),
            dims.cell_count(),
            "snapshot cell count must match grid dimensions"
        );
        Self { dims, cells }
    }

    /// Return the dimensions of the captured grid.
    pub const fn dims(&self) -> GridDims {
        self.dims
    }

    /// Return the flat index for a coordinate, if it lies inside the snapshot.
    pub fn index_of(&self, coord: [u32; 3]) -> Option<usize> {
        let [x, y, z] = coord;
        if x >= self.dims.width || y >= self.dims.height || z >= self.dims.depth {
            return None;
        }

        let width = self.dims.width as usize;
        let height = self.dims.height as usize;
        Some((x as usize) + (y as usize) * width + (z as usize) * width * height)
    }

    /// Return the cell at a coordinate, if it lies inside the snapshot.
    pub fn get(&self, coord: [u32; 3]) -> Option<&C> {
        self.index_of(coord).map(|index| &self.cells[index])
    }

    /// Iterate all cells with their coordinates in x-major order.
    pub fn iter_xyz(&self) -> impl Iterator<Item = (u32, u32, u32, &C)> {
        let width = self.dims.width as usize;
        let height = self.dims.height as usize;

        self.cells.iter().enumerate().map(move |(index, cell)| {
            let x = (index % width) as u32;
            let y = ((index / width) % height) as u32;
            let z = (index / (width * height)) as u32;
            (x, y, z, cell)
        })
    }
}
