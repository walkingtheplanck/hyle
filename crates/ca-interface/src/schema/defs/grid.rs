/// Errors raised while constructing validated grid descriptors.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GridShapeError {
    /// Grid dimensions overflow `usize` when computing the logical cell count.
    GridCellCountOverflow {
        /// Requested grid width.
        width: u32,
        /// Requested grid height.
        height: u32,
        /// Requested grid depth.
        depth: u32,
    },
    /// Region extents overflow `usize` when computing the logical cell count.
    RegionCellCountOverflow {
        /// Requested region origin.
        origin: [u32; 3],
        /// Requested region size.
        size: [u32; 3],
    },
    /// Region end coordinates overflow `u32`.
    RegionEndOverflow {
        /// Requested region origin.
        origin: [u32; 3],
        /// Requested region size.
        size: [u32; 3],
    },
}

/// Immutable grid dimensions in cells.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GridDims {
    width: u32,
    height: u32,
    depth: u32,
    cell_count: usize,
}

impl Default for GridDims {
    fn default() -> Self {
        Self {
            width: 0,
            height: 0,
            depth: 0,
            cell_count: 0,
        }
    }
}

impl GridDims {
    /// Construct a new set of grid dimensions.
    pub fn new(width: u32, height: u32, depth: u32) -> Result<Self, GridShapeError> {
        let Some(cell_count) = (width as usize)
            .checked_mul(height as usize)
            .and_then(|xy| xy.checked_mul(depth as usize))
        else {
            return Err(GridShapeError::GridCellCountOverflow {
                width,
                height,
                depth,
            });
        };

        Ok(Self {
            width,
            height,
            depth,
            cell_count,
        })
    }

    #[doc(hidden)]
    pub const fn from_validated(
        width: u32,
        height: u32,
        depth: u32,
        cell_count: usize,
    ) -> Self {
        Self {
            width,
            height,
            depth,
            cell_count,
        }
    }

    /// Grid width in cells.
    pub const fn width(self) -> u32 {
        self.width
    }

    /// Grid height in cells.
    pub const fn height(self) -> u32 {
        self.height
    }

    /// Grid depth in cells.
    pub const fn depth(self) -> u32 {
        self.depth
    }

    /// Total number of logical cells in the grid.
    pub const fn cell_count(self) -> usize {
        self.cell_count
    }

    /// The full grid expressed as a region starting at the origin.
    pub const fn as_region(self) -> GridRegion {
        GridRegion::from_validated(
            [0, 0, 0],
            [self.width, self.height, self.depth],
            [self.width, self.height, self.depth],
            self.cell_count,
        )
    }

    /// Whether a region lies completely inside these dimensions.
    pub fn contains_region(self, region: GridRegion) -> bool {
        let end = region.end_exclusive();
        end[0] <= self.width && end[1] <= self.height && end[2] <= self.depth
    }
}

/// A rectangular subregion of the grid.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GridRegion {
    origin: [u32; 3],
    size: [u32; 3],
    end_exclusive: [u32; 3],
    cell_count: usize,
}

impl Default for GridRegion {
    fn default() -> Self {
        Self {
            origin: [0, 0, 0],
            size: [0, 0, 0],
            end_exclusive: [0, 0, 0],
            cell_count: 0,
        }
    }
}

impl GridRegion {
    /// Construct a new region from an origin and size.
    pub fn new(origin: [u32; 3], size: [u32; 3]) -> Result<Self, GridShapeError> {
        let Some(end_exclusive) = Some([
            origin[0].checked_add(size[0]),
            origin[1].checked_add(size[1]),
            origin[2].checked_add(size[2]),
        ])
        .and_then(|coords| Some([coords[0]?, coords[1]?, coords[2]?]))
        else {
            return Err(GridShapeError::RegionEndOverflow { origin, size });
        };

        let Some(cell_count) = (size[0] as usize)
            .checked_mul(size[1] as usize)
            .and_then(|xy| xy.checked_mul(size[2] as usize))
        else {
            return Err(GridShapeError::RegionCellCountOverflow { origin, size });
        };

        Ok(Self {
            origin,
            size,
            end_exclusive,
            cell_count,
        })
    }

    #[doc(hidden)]
    pub const fn from_validated(
        origin: [u32; 3],
        size: [u32; 3],
        end_exclusive: [u32; 3],
        cell_count: usize,
    ) -> Self {
        Self {
            origin,
            size,
            end_exclusive,
            cell_count,
        }
    }

    /// Origin of the region, inclusive.
    pub const fn origin(self) -> [u32; 3] {
        self.origin
    }

    /// Extent of the region in cells.
    pub const fn size(self) -> [u32; 3] {
        self.size
    }

    /// Total number of cells inside the region.
    pub const fn cell_count(self) -> usize {
        self.cell_count
    }

    /// The exclusive end coordinate of the region.
    pub const fn end_exclusive(self) -> [u32; 3] {
        self.end_exclusive
    }
}

/// A contiguous host-side snapshot of the current solver state.
///
/// Cells are stored in x-major order: x changes fastest, then y, then z.
#[derive(Clone, Debug)]
pub struct GridSnapshot<C> {
    /// Dimensions of the captured grid.
    pub dims: GridDims,
    /// Captured cell values in x-major order.
    pub cells: Vec<C>,
}

impl<C> GridSnapshot<C> {
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
        if x >= self.dims.width() || y >= self.dims.height() || z >= self.dims.depth() {
            return None;
        }

        let width = self.dims.width() as usize;
        let height = self.dims.height() as usize;
        Some((x as usize) + (y as usize) * width + (z as usize) * width * height)
    }

    /// Return the cell at a coordinate, if it lies inside the snapshot.
    pub fn get(&self, coord: [u32; 3]) -> Option<&C> {
        self.index_of(coord).map(|index| &self.cells[index])
    }

    /// Iterate all cells with their coordinates in x-major order.
    pub fn iter_xyz(&self) -> impl Iterator<Item = (u32, u32, u32, &C)> {
        let width = self.dims.width() as usize;
        let height = self.dims.height() as usize;

        self.cells.iter().enumerate().map(move |(index, cell)| {
            let x = (index % width) as u32;
            let y = ((index / width) % height) as u32;
            let z = (index / (width * height)) as u32;
            (x, y, z, cell)
        })
    }
}
