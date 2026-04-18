//! Shared grid geometry and snapshot transport types.
//!
//! These types are used by both schema-facing setup APIs and runtime bulk IO.

use std::error::Error;
use std::fmt::{Display, Formatter};

/// Errors raised while constructing validated grid descriptors.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GridShapeError {
    /// One or more axes exceed the framework coordinate range.
    CoordinateRangeOverflow {
        /// Requested grid width.
        width: u32,
        /// Requested grid height.
        height: u32,
        /// Requested grid depth.
        depth: u32,
        /// Largest supported axis length.
        max: u32,
    },
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

impl Display for GridShapeError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            GridShapeError::CoordinateRangeOverflow {
                width,
                height,
                depth,
                max,
            } => write!(
                f,
                "grid dimensions ({width}, {height}, {depth}) exceed the supported coordinate range max of {max}"
            ),
            GridShapeError::GridCellCountOverflow {
                width,
                height,
                depth,
            } => write!(
                f,
                "grid dimensions ({width}, {height}, {depth}) overflow host cell indexing"
            ),
            GridShapeError::RegionCellCountOverflow { origin, size } => write!(
                f,
                "grid region at {:?} with size {:?} overflows host cell indexing",
                origin, size
            ),
            GridShapeError::RegionEndOverflow { origin, size } => write!(
                f,
                "grid region at {:?} with size {:?} overflows its end coordinates",
                origin, size
            ),
        }
    }
}

impl Error for GridShapeError {}

/// Errors raised while building host-side grid data containers.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GridDataError {
    /// The provided host-side cell slice length does not match the grid shape.
    CellCountMismatch {
        /// Number of cells implied by the grid dimensions.
        expected: usize,
        /// Number of cells actually provided by the caller.
        actual: usize,
    },
}

impl Display for GridDataError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            GridDataError::CellCountMismatch { expected, actual } => write!(
                f,
                "grid data length mismatch: expected {expected} cells, got {actual}"
            ),
        }
    }
}

impl Error for GridDataError {}

/// Immutable grid dimensions in cells.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct GridDims {
    width: u32,
    height: u32,
    depth: u32,
    cell_count: usize,
}

impl GridDims {
    /// Construct a new set of grid dimensions.
    ///
    /// # Errors
    ///
    /// Returns [`GridShapeError::CoordinateRangeOverflow`] when any axis exceeds
    /// the largest coordinate range supported by topology resolution.
    ///
    /// Returns [`GridShapeError::GridCellCountOverflow`] when the logical cell
    /// count does not fit in host indexing types.
    pub fn new(width: u32, height: u32, depth: u32) -> Result<Self, GridShapeError> {
        let max = i32::MAX as u32;
        if width > max || height > max || depth > max {
            return Err(GridShapeError::CoordinateRangeOverflow {
                width,
                height,
                depth,
                max,
            });
        }

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

    /// Construct grid dimensions from already validated state.
    ///
    /// # Invariants
    ///
    /// `cell_count` must equal `width * height * depth`, and the three axes must
    /// already satisfy the same range checks enforced by [`GridDims::new`].
    ///
    /// # Performance
    ///
    /// This skips redundant validation for internal paths that already proved the
    /// shape is valid, such as full-grid region helpers and instance-backed
    /// solver construction.
    #[doc(hidden)]
    pub const fn from_validated(width: u32, height: u32, depth: u32, cell_count: usize) -> Self {
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
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct GridRegion {
    origin: [u32; 3],
    size: [u32; 3],
    end_exclusive: [u32; 3],
    cell_count: usize,
}

impl GridRegion {
    /// Construct a new region from an origin and size.
    ///
    /// # Errors
    ///
    /// Returns [`GridShapeError::RegionEndOverflow`] when `origin + size`
    /// exceeds the coordinate range, or
    /// [`GridShapeError::RegionCellCountOverflow`] when the region volume does
    /// not fit in host indexing types.
    pub fn new(origin: [u32; 3], size: [u32; 3]) -> Result<Self, GridShapeError> {
        let Some(end_exclusive) = Some([
            origin[0].checked_add(size[0]),
            origin[1].checked_add(size[1]),
            origin[2].checked_add(size[2]),
        ])
        .and_then(|coords| Some([coords[0]?, coords[1]?, coords[2]?])) else {
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

    /// Construct a region from already validated state.
    ///
    /// # Invariants
    ///
    /// `end_exclusive` must equal `origin + size`, and `cell_count` must equal
    /// the region volume implied by `size`.
    ///
    /// # Performance
    ///
    /// This keeps helpers such as [`GridDims::as_region`] infallible once the
    /// enclosing grid shape has already been validated.
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
pub struct GridSnapshot {
    /// Dimensions of the captured grid.
    pub dims: GridDims,
    /// Captured cell values in x-major order.
    pub cells: Vec<crate::MaterialId>,
}

impl GridSnapshot {
    /// Construct a validated grid snapshot.
    ///
    /// # Errors
    ///
    /// Returns [`GridDataError::CellCountMismatch`] when the provided cells do
    /// not fully cover the declared grid shape.
    pub fn new(dims: GridDims, cells: Vec<crate::MaterialId>) -> Result<Self, GridDataError> {
        if cells.len() != dims.cell_count() {
            return Err(GridDataError::CellCountMismatch {
                expected: dims.cell_count(),
                actual: cells.len(),
            });
        }

        Ok(Self { dims, cells })
    }

    /// Construct a snapshot from already validated state.
    ///
    /// # Invariants
    ///
    /// `cells.len()` must match `dims.cell_count()`.
    ///
    /// # Performance
    ///
    /// Runtime readback paths allocate the exact output length themselves, so
    /// rechecking that length here would only duplicate already-proven work.
    #[doc(hidden)]
    pub const fn from_validated(dims: GridDims, cells: Vec<crate::MaterialId>) -> Self {
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
    pub fn get(&self, coord: [u32; 3]) -> Option<&crate::MaterialId> {
        self.index_of(coord).map(|index| &self.cells[index])
    }

    /// Iterate all cells with their coordinates in x-major order.
    pub fn iter_xyz(&self) -> impl Iterator<Item = (u32, u32, u32, &crate::MaterialId)> {
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
