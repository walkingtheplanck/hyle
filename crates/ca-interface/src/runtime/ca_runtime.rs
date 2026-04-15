//! Object-safe runtime interface for consumers that only need to drive a simulation.

use crate::{
    AttributeAccessError, AttributeId, AttributeValue, GridDims, GridRegion, GridSnapshot,
    MaterialId,
};

use super::solver::CaSolver;

/// A compact simulation runtime surface for consumers.
///
/// This trait exists for consumers such as viewers and tools that need common
/// simulation operations without depending on the full [`CaSolver`] contract.
pub trait CaRuntime: Send {
    /// Logical grid dimensions.
    fn dims(&self) -> GridDims;

    /// Advance the simulation by one step.
    fn step(&mut self);

    /// Set a material at the given coordinate.
    fn set(&mut self, x: i32, y: i32, z: i32, material: MaterialId);

    /// Read one attached attribute by id from the resolved cell coordinate.
    fn get_attr(
        &self,
        attribute: AttributeId,
        x: i32,
        y: i32,
        z: i32,
    ) -> Result<AttributeValue, AttributeAccessError>;

    /// Overwrite one attached attribute by id at the resolved cell coordinate.
    fn set_attr(
        &mut self,
        attribute: AttributeId,
        x: i32,
        y: i32,
        z: i32,
        value: AttributeValue,
    ) -> Result<(), AttributeAccessError>;

    /// Read a contiguous rectangular region in x-major order.
    fn read_region(&self, region: GridRegion) -> Vec<MaterialId>;

    /// Overwrite a contiguous rectangular region from x-major ordered data.
    fn write_region(&mut self, region: GridRegion, cells: &[MaterialId]);

    /// Replace the full current state from x-major ordered data.
    fn replace_cells(&mut self, cells: &[MaterialId]);

    /// Read the full current state back to the host.
    fn readback(&self) -> GridSnapshot<MaterialId>;

    /// Number of completed steps.
    fn step_count(&self) -> u32;
}

impl<T> CaRuntime for T
where
    T: CaSolver + Send,
{
    fn dims(&self) -> GridDims {
        CaSolver::dims(self)
    }

    fn step(&mut self) {
        CaSolver::step(self);
    }

    fn set(&mut self, x: i32, y: i32, z: i32, material: MaterialId) {
        CaSolver::set(self, x, y, z, material);
    }

    fn get_attr(
        &self,
        attribute: AttributeId,
        x: i32,
        y: i32,
        z: i32,
    ) -> Result<AttributeValue, AttributeAccessError> {
        CaSolver::get_attr(self, attribute, x, y, z)
    }

    fn set_attr(
        &mut self,
        attribute: AttributeId,
        x: i32,
        y: i32,
        z: i32,
        value: AttributeValue,
    ) -> Result<(), AttributeAccessError> {
        CaSolver::set_attr(self, attribute, x, y, z, value)
    }

    fn read_region(&self, region: GridRegion) -> Vec<MaterialId> {
        CaSolver::read_region(self, region)
    }

    fn write_region(&mut self, region: GridRegion, cells: &[MaterialId]) {
        CaSolver::write_region(self, region, cells);
    }

    fn replace_cells(&mut self, cells: &[MaterialId]) {
        CaSolver::replace_cells(self, cells);
    }

    fn readback(&self) -> GridSnapshot<MaterialId> {
        CaSolver::readback(self)
    }

    fn step_count(&self) -> u32 {
        CaSolver::step_count(self)
    }
}
