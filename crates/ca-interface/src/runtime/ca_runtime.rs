//! Object-safe runtime interface for consumers that only need to drive a simulation.

use crate::{AttributeAccessError, AttributeValue, Cell, GridDims, GridRegion, GridSnapshot};

use super::solver::CaSolver;

/// A compact simulation runtime surface for consumers.
///
/// This trait exists for consumers such as viewers and tools that need common
/// simulation operations without depending on the full generic [`CaSolver`]
/// contract.
pub trait CaRuntime<C: Cell>: Send {
    /// Logical grid dimensions.
    fn dims(&self) -> GridDims;

    /// Advance the simulation by one step.
    fn step(&mut self);

    /// Set a cell value at the given coordinate.
    fn set(&mut self, x: i32, y: i32, z: i32, cell: C);

    /// Read one attached attribute by name from the resolved cell coordinate.
    fn get_attr(
        &self,
        name: &str,
        x: i32,
        y: i32,
        z: i32,
    ) -> Result<AttributeValue, AttributeAccessError>;

    /// Overwrite one attached attribute by name at the resolved cell coordinate.
    fn set_attr(
        &mut self,
        name: &str,
        x: i32,
        y: i32,
        z: i32,
        value: AttributeValue,
    ) -> Result<(), AttributeAccessError>;

    /// Read a contiguous rectangular region in x-major order.
    fn read_region(&self, region: GridRegion) -> Vec<C>;

    /// Overwrite a contiguous rectangular region from x-major ordered data.
    fn write_region(&mut self, region: GridRegion, cells: &[C]);

    /// Replace the full current state from x-major ordered data.
    fn replace_cells(&mut self, cells: &[C]);

    /// Read the full current state back to the host.
    fn readback(&self) -> GridSnapshot<C>;

    /// Number of completed steps.
    fn step_count(&self) -> u32;
}

impl<C, T> CaRuntime<C> for T
where
    C: Cell,
    T: CaSolver<C> + Send,
{
    fn dims(&self) -> GridDims {
        CaSolver::dims(self)
    }

    fn step(&mut self) {
        CaSolver::step(self);
    }

    fn set(&mut self, x: i32, y: i32, z: i32, cell: C) {
        CaSolver::set(self, x, y, z, cell);
    }

    fn get_attr(
        &self,
        name: &str,
        x: i32,
        y: i32,
        z: i32,
    ) -> Result<AttributeValue, AttributeAccessError> {
        CaSolver::get_attr(self, name, x, y, z)
    }

    fn set_attr(
        &mut self,
        name: &str,
        x: i32,
        y: i32,
        z: i32,
        value: AttributeValue,
    ) -> Result<(), AttributeAccessError> {
        CaSolver::set_attr(self, name, x, y, z, value)
    }

    fn read_region(&self, region: GridRegion) -> Vec<C> {
        CaSolver::read_region(self, region)
    }

    fn write_region(&mut self, region: GridRegion, cells: &[C]) {
        CaSolver::write_region(self, region, cells);
    }

    fn replace_cells(&mut self, cells: &[C]) {
        CaSolver::replace_cells(self, cells);
    }

    fn readback(&self) -> GridSnapshot<C> {
        CaSolver::readback(self)
    }

    fn step_count(&self) -> u32 {
        CaSolver::step_count(self)
    }
}
