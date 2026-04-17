//! Standard concrete `CaRuntime` wrapper around a `CaSolver`.

use crate::{
    AttributeAccessError, AttributeDef, AttributeId, AttributeValue, CaRuntime, CaSolver,
    CellAttributeValue, CellId, CellQueryError, GridDims, GridRegion, GridSnapshot, MaterialDef,
    MaterialId, NeighborhoodId, NeighborhoodSpec, TransitionCount,
};

/// Standard consumer-facing runtime backed by one concrete solver.
pub struct Runtime<S> {
    solver: S,
}

impl<S> Runtime<S> {
    /// Wrap a concrete solver in the shared runtime surface.
    pub const fn new(solver: S) -> Self {
        Self { solver }
    }

    /// Borrow the inner solver directly.
    pub const fn solver(&self) -> &S {
        &self.solver
    }

    /// Mutably borrow the inner solver directly.
    pub fn solver_mut(&mut self) -> &mut S {
        &mut self.solver
    }

    /// Consume the runtime and return the wrapped solver.
    pub fn into_solver(self) -> S {
        self.solver
    }
}

impl<S> CaRuntime for Runtime<S>
where
    S: CaSolver + Send,
{
    fn dims(&self) -> GridDims {
        CaSolver::dims(&self.solver)
    }

    fn step(&mut self) {
        CaSolver::step(&mut self.solver);
    }

    fn material_defs(&self) -> &[MaterialDef] {
        CaSolver::material_defs(&self.solver)
    }

    fn attribute_defs(&self) -> &[AttributeDef] {
        CaSolver::attribute_defs(&self.solver)
    }

    fn neighborhood_specs(&self) -> &[NeighborhoodSpec] {
        CaSolver::neighborhood_specs(&self.solver)
    }

    fn cell_at(&self, x: i32, y: i32, z: i32) -> Option<CellId> {
        CaSolver::cell_at(&self.solver, x, y, z)
    }

    fn cell_position(&self, cell: CellId) -> Result<[u32; 3], CellQueryError> {
        CaSolver::cell_position(&self.solver, cell)
    }

    fn material(&self, cell: CellId) -> Result<MaterialId, CellQueryError> {
        CaSolver::material(&self.solver, cell)
    }

    fn attribute(
        &self,
        cell: CellId,
        attribute: AttributeId,
    ) -> Result<AttributeValue, CellQueryError> {
        CaSolver::attribute(&self.solver, cell, attribute)
    }

    fn attributes(&self, cell: CellId) -> Result<Vec<CellAttributeValue>, CellQueryError> {
        CaSolver::attributes(&self.solver, cell)
    }

    fn neighbors(
        &self,
        cell: CellId,
        neighborhood: NeighborhoodId,
    ) -> Result<Vec<CellId>, CellQueryError> {
        CaSolver::neighbors(&self.solver, cell, neighborhood)
    }

    fn set(&mut self, x: i32, y: i32, z: i32, material: MaterialId) {
        CaSolver::set(&mut self.solver, x, y, z, material);
    }

    fn get_attr(
        &self,
        attribute: AttributeId,
        x: i32,
        y: i32,
        z: i32,
    ) -> Result<AttributeValue, AttributeAccessError> {
        CaSolver::get_attr(&self.solver, attribute, x, y, z)
    }

    fn set_attr(
        &mut self,
        attribute: AttributeId,
        x: i32,
        y: i32,
        z: i32,
        value: AttributeValue,
    ) -> Result<(), AttributeAccessError> {
        CaSolver::set_attr(&mut self.solver, attribute, x, y, z, value)
    }

    fn read_region(&self, region: GridRegion) -> Vec<MaterialId> {
        CaSolver::read_region(&self.solver, region)
    }

    fn write_region(&mut self, region: GridRegion, cells: &[MaterialId]) {
        CaSolver::write_region(&mut self.solver, region, cells);
    }

    fn replace_cells(&mut self, cells: &[MaterialId]) {
        CaSolver::replace_cells(&mut self.solver, cells);
    }

    fn readback(&self) -> GridSnapshot<MaterialId> {
        CaSolver::readback(&self.solver)
    }

    fn step_count(&self) -> u32 {
        CaSolver::step_count(&self.solver)
    }

    fn last_changed_cells(&self) -> u64 {
        CaSolver::last_changed_cells(&self.solver)
    }

    fn population(&self, material: MaterialId) -> u64 {
        CaSolver::population(&self.solver, material)
    }

    fn populations(&self) -> Vec<u64> {
        CaSolver::populations(&self.solver)
    }

    fn last_transitions(&self) -> &[TransitionCount] {
        CaSolver::last_transitions(&self.solver)
    }
}
