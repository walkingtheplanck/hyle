//! Core solver execution capabilities implemented directly by backends.

use crate::{AttributeAccessError, AttributeId, AttributeValue, GridDims, MaterialId, Topology};

/// Low-level execution and coordinate-resolution capabilities every solver must provide.
pub trait SolverExecution {
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

    /// Deterministic run seed used for semantic randomness.
    fn seed(&self) -> u64 {
        0
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
    fn resolve_index(&self, x: i32, y: i32, z: i32) -> usize {
        self.topology()
            .resolve_index(x, y, z, self.dims(), self.guard_index())
    }

    /// Read one material value at the given coordinate.
    fn get(&self, x: i32, y: i32, z: i32) -> MaterialId;

    /// Set one material value at the given coordinate.
    fn set(&mut self, x: i32, y: i32, z: i32, material: MaterialId);

    /// Read one attached attribute by id from the given coordinate.
    fn get_attr(
        &self,
        attribute: AttributeId,
        x: i32,
        y: i32,
        z: i32,
    ) -> Result<AttributeValue, AttributeAccessError>;

    /// Write one attached attribute by id to the given coordinate.
    fn set_attr(
        &mut self,
        attribute: AttributeId,
        x: i32,
        y: i32,
        z: i32,
        value: AttributeValue,
    ) -> Result<(), AttributeAccessError>;

    /// Advance the simulation by one logical step.
    fn step(&mut self);

    /// Number of logical steps already completed.
    fn step_count(&self) -> u32;
}
