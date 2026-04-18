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
    ///
    /// Backends are expected to treat these as immutable after construction.
    fn dims(&self) -> GridDims;

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
    ///
    /// Returning [`SolverExecution::guard_index`] is the contract-level signal
    /// for "no logical cell", which lets hot-path solver code avoid `Option`.
    fn resolve_index(&self, x: i32, y: i32, z: i32) -> usize {
        self.topology()
            .resolve_index(x, y, z, self.dims(), self.guard_index())
    }

    /// Read one material value at the given coordinate.
    ///
    /// This is the low-level primitive used by derived query helpers and rule
    /// execution. Out-of-range behavior follows the surrounding `CaSolver`
    /// contract instead of returning `Result`.
    fn get(&self, x: i32, y: i32, z: i32) -> MaterialId;

    /// Set one material value at the given coordinate.
    ///
    /// Like [`SolverExecution::get`], this remains infallible to keep the
    /// execution surface branch-light in the solver step loop.
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
    ///
    /// This is fallible because attribute channels have typed schemas and can be
    /// absent even when material reads/writes stay sentinel-based.
    fn set_attr(
        &mut self,
        attribute: AttributeId,
        x: i32,
        y: i32,
        z: i32,
        value: AttributeValue,
    ) -> Result<(), AttributeAccessError>;

    /// Advance the simulation by one logical step.
    ///
    /// Backends must publish the completed step's state before returning.
    fn step(&mut self);

    /// Number of logical steps already completed.
    fn step_count(&self) -> u32;
}
