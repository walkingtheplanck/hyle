//! Runtime simulation-instance parameters.

use crate::{GridDims, GridShapeError};

/// Portable runtime parameters for one solver instance.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Instance {
    dims: GridDims,
    seed: u64,
}

impl Instance {
    /// Construct a new instance with zero seed.
    pub fn new(width: u32, height: u32, depth: u32) -> Result<Self, GridShapeError> {
        Ok(Self {
            dims: GridDims::new(width, height, depth)?,
            seed: 0,
        })
    }

    /// Construct an instance from existing grid dimensions.
    pub const fn from_dims(dims: GridDims) -> Self {
        Self { dims, seed: 0 }
    }

    /// Return the logical grid dimensions.
    pub const fn dims(&self) -> GridDims {
        self.dims
    }

    /// Return the deterministic run seed.
    pub const fn seed(&self) -> u64 {
        self.seed
    }

    /// Return a copy with the given deterministic run seed.
    pub const fn with_seed(self, seed: u64) -> Self {
        Self {
            dims: self.dims,
            seed,
        }
    }
}
