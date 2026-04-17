//! Object-safe runtime interface for consumers that only need to drive a simulation.

use crate::{
    AttributeAccessError, AttributeDef, AttributeId, AttributeValue, CellAttributeValue, CellId,
    CellQueryError, GridDims, GridRegion, GridSnapshot, MaterialDef, MaterialId, NeighborhoodId,
    NeighborhoodSpec, TransitionCount,
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

    /// Material descriptors declared on the active schema, if available.
    fn material_defs(&self) -> &[MaterialDef];

    /// Attribute descriptors declared on the active schema, if available.
    fn attribute_defs(&self) -> &[AttributeDef];

    /// Neighborhood specs declared on the active schema, if available.
    fn neighborhood_specs(&self) -> &[NeighborhoodSpec];

    /// Resolve one logical cell handle from grid coordinates.
    fn cell_at(&self, x: i32, y: i32, z: i32) -> Option<CellId>;

    /// Decode a cell handle back into its canonical grid position.
    fn cell_position(&self, cell: CellId) -> Result<[u32; 3], CellQueryError>;

    /// Read one material from a resolved cell handle.
    fn material(&self, cell: CellId) -> Result<MaterialId, CellQueryError>;

    /// Read one attached attribute from a resolved cell handle.
    fn attribute(
        &self,
        cell: CellId,
        attribute: AttributeId,
    ) -> Result<AttributeValue, CellQueryError>;

    /// Read all declared attached attributes from a resolved cell handle.
    fn attributes(&self, cell: CellId) -> Result<Vec<CellAttributeValue>, CellQueryError>;

    /// Resolve all neighbors around one cell for the given neighborhood.
    fn neighbors(
        &self,
        cell: CellId,
        neighborhood: NeighborhoodId,
    ) -> Result<Vec<CellId>, CellQueryError>;

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

    /// Number of cells whose material changed during the latest completed step.
    fn last_changed_cells(&self) -> u64;

    /// Population of one material in the current grid state.
    fn population(&self, material: MaterialId) -> u64;

    /// Full current per-material population table.
    fn populations(&self) -> Vec<u64>;

    /// Material-to-material transition counts from the latest completed step.
    fn last_transitions(&self) -> &[TransitionCount];
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

    fn material_defs(&self) -> &[MaterialDef] {
        CaSolver::material_defs(self)
    }

    fn attribute_defs(&self) -> &[AttributeDef] {
        CaSolver::attribute_defs(self)
    }

    fn neighborhood_specs(&self) -> &[NeighborhoodSpec] {
        CaSolver::neighborhood_specs(self)
    }

    fn cell_at(&self, x: i32, y: i32, z: i32) -> Option<CellId> {
        CaSolver::cell_at(self, x, y, z)
    }

    fn cell_position(&self, cell: CellId) -> Result<[u32; 3], CellQueryError> {
        CaSolver::cell_position(self, cell)
    }

    fn material(&self, cell: CellId) -> Result<MaterialId, CellQueryError> {
        CaSolver::material(self, cell)
    }

    fn attribute(
        &self,
        cell: CellId,
        attribute: AttributeId,
    ) -> Result<AttributeValue, CellQueryError> {
        CaSolver::attribute(self, cell, attribute)
    }

    fn attributes(&self, cell: CellId) -> Result<Vec<CellAttributeValue>, CellQueryError> {
        CaSolver::attributes(self, cell)
    }

    fn neighbors(
        &self,
        cell: CellId,
        neighborhood: NeighborhoodId,
    ) -> Result<Vec<CellId>, CellQueryError> {
        CaSolver::neighbors(self, cell, neighborhood)
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

    fn last_changed_cells(&self) -> u64 {
        CaSolver::last_changed_cells(self)
    }

    fn population(&self, material: MaterialId) -> u64 {
        CaSolver::population(self, material)
    }

    fn populations(&self) -> Vec<u64> {
        CaSolver::populations(self)
    }

    fn last_transitions(&self) -> &[TransitionCount] {
        CaSolver::last_transitions(self)
    }
}
