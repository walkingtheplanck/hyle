//! Standard concrete `CaRuntime` wrapper around a `CaSolver`.

use crate::{
    AttributeAccessError, AttributeDef, AttributeId, AttributeValue, CaSolver, CellAttributeValue,
    CellId, CellQueryError, GridAccessError, GridDims, GridRegion, GridSnapshot, MaterialDef,
    MaterialId, NeighborhoodId, NeighborhoodSpec, TransitionCount,
};

use crate::runtime::traits::{
    SolverAttributes, SolverCells, SolverExecution, SolverGrid, SolverMetadata, SolverMetrics,
};

use super::{
    RuntimeAttributes, RuntimeCells, RuntimeGrid, RuntimeMetadata, RuntimeMetrics, RuntimeStepping,
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

impl<S> RuntimeMetadata for Runtime<S>
where
    S: CaSolver + Send,
{
    fn dims(&self) -> GridDims {
        SolverExecution::dims(&self.solver)
    }

    fn material_defs(&self) -> &[MaterialDef] {
        SolverMetadata::material_defs(&self.solver)
    }

    fn attribute_defs(&self) -> &[AttributeDef] {
        SolverMetadata::attribute_defs(&self.solver)
    }

    fn neighborhood_specs(&self) -> &[NeighborhoodSpec] {
        SolverMetadata::neighborhood_specs(&self.solver)
    }
}

impl<S> RuntimeCells for Runtime<S>
where
    S: CaSolver + Send,
{
    fn cell_at(&self, x: i32, y: i32, z: i32) -> Option<CellId> {
        SolverCells::cell_at(&self.solver, x, y, z)
    }

    fn cell_position(&self, cell: CellId) -> Result<[u32; 3], CellQueryError> {
        SolverCells::cell_position(&self.solver, cell)
    }

    fn cells_in_region(&self, region: GridRegion) -> Result<Vec<CellId>, GridAccessError> {
        SolverCells::cells_in_region(&self.solver, region)
    }

    fn material(&self, cell: CellId) -> Result<MaterialId, CellQueryError> {
        SolverCells::material(&self.solver, cell)
    }

    fn neighbors(
        &self,
        cell: CellId,
        neighborhood: NeighborhoodId,
    ) -> Result<Vec<CellId>, CellQueryError> {
        SolverCells::neighbors(&self.solver, cell, neighborhood)
    }
}

impl<S> RuntimeAttributes for Runtime<S>
where
    S: CaSolver + Send,
{
    fn attribute(
        &self,
        cell: CellId,
        attribute: AttributeId,
    ) -> Result<AttributeValue, CellQueryError> {
        SolverAttributes::attribute(&self.solver, cell, attribute)
    }

    fn attributes(&self, cell: CellId) -> Result<Vec<CellAttributeValue>, CellQueryError> {
        SolverAttributes::attributes(&self.solver, cell)
    }

    fn get_attr(
        &self,
        attribute: AttributeId,
        x: i32,
        y: i32,
        z: i32,
    ) -> Result<AttributeValue, AttributeAccessError> {
        SolverExecution::get_attr(&self.solver, attribute, x, y, z)
    }

    fn set_attr(
        &mut self,
        attribute: AttributeId,
        x: i32,
        y: i32,
        z: i32,
        value: AttributeValue,
    ) -> Result<(), AttributeAccessError> {
        SolverExecution::set_attr(&mut self.solver, attribute, x, y, z, value)
    }
}

impl<S> RuntimeGrid for Runtime<S>
where
    S: CaSolver + Send,
{
    fn set(&mut self, x: i32, y: i32, z: i32, material: MaterialId) {
        SolverExecution::set(&mut self.solver, x, y, z, material);
    }

    fn read_region(&self, region: GridRegion) -> Result<Vec<MaterialId>, GridAccessError> {
        SolverGrid::read_region(&self.solver, region)
    }

    fn write_region(
        &mut self,
        region: GridRegion,
        cells: &[MaterialId],
    ) -> Result<(), GridAccessError> {
        SolverGrid::write_region(&mut self.solver, region, cells)
    }

    fn replace_cells(&mut self, cells: &[MaterialId]) -> Result<(), GridAccessError> {
        SolverGrid::replace_cells(&mut self.solver, cells)
    }

    fn readback(&self) -> GridSnapshot<MaterialId> {
        SolverGrid::readback(&self.solver)
    }
}

impl<S> RuntimeStepping for Runtime<S>
where
    S: CaSolver + Send,
{
    fn step(&mut self) {
        SolverExecution::step(&mut self.solver);
    }

    fn step_count(&self) -> u32 {
        SolverExecution::step_count(&self.solver)
    }
}

impl<S> RuntimeMetrics for Runtime<S>
where
    S: CaSolver + Send,
{
    fn last_changed_cells(&self) -> u64 {
        SolverMetrics::last_changed_cells(&self.solver)
    }

    fn population(&self, material: MaterialId) -> u64 {
        SolverMetrics::population(&self.solver, material)
    }

    fn populations(&self) -> Vec<u64> {
        SolverMetrics::populations(&self.solver)
    }

    fn last_transitions(&self) -> &[TransitionCount] {
        SolverMetrics::last_transitions(&self.solver)
    }
}
