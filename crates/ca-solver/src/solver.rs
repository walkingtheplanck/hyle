//! Default CPU solver - double-buffered, single-threaded.

use hyle_ca_interface::semantics::{interpret_blueprint, ResolvedBlueprint};
use hyle_ca_interface::{
    AttributeAccessError, AttributeValue, Blueprint, CaSolver, Cell, CellModel, GridRegion,
    GridSnapshot, Instance, RuleEffect, Topology,
};

use crate::attributes::AttributeStore;
use crate::grid::{resolve_index, Grid};
use crate::program::CompiledProgram;
use crate::{BoundedTopology, DescriptorTopology};

/// Default 3D cellular blueprint solver, generic over cell type `C`.
///
/// The solver can run without an attached blueprint, in which case `step()`
/// preserves the current state. Use [`Solver::from_blueprint`] to construct a
/// solver from an interpreted [`ResolvedBlueprint`], or [`Solver::from_spec`] to
/// interpret a declarative [`Blueprint`] and construct one in a single step.
pub struct Solver<C: Cell + Eq, T: Topology = BoundedTopology> {
    grid: Grid<C>,
    attributes: AttributeStore,
    topology: T,
    program: Option<CompiledProgram<C>>,
    step_count: u32,
    seed: u64,
}

impl<C: Cell + Eq> Solver<C, BoundedTopology> {
    /// Create a new bounded solver filled with `C::default()`.
    pub fn new(width: u32, height: u32, depth: u32) -> Self {
        Self::with_instance_and_topology(Instance::new(width, height, depth), BoundedTopology)
    }

    /// Create a new solver filled with `C::default()` and the given topology.
    pub fn with_topology<U: Topology>(
        width: u32,
        height: u32,
        depth: u32,
        topology: U,
    ) -> Solver<C, U> {
        Self::with_instance_and_topology(Instance::new(width, height, depth), topology)
    }

    /// Create a new solver from a runtime instance and topology policy.
    pub fn with_instance_and_topology<U: Topology>(
        instance: Instance,
        topology: U,
    ) -> Solver<C, U> {
        Solver {
            grid: Grid::new(
                instance.dims().width,
                instance.dims().height,
                instance.dims().depth,
            ),
            attributes: AttributeStore::new(instance.dims().cell_count() + 1, &[]),
            topology,
            program: None,
            step_count: 0,
            seed: instance.seed(),
        }
    }
}

impl<C: Cell + CellModel + Eq> Solver<C, DescriptorTopology> {
    /// Create a solver whose topology and rules come from an interpreted blueprint.
    pub fn from_blueprint(
        width: u32,
        height: u32,
        depth: u32,
        blueprint: &ResolvedBlueprint<C>,
    ) -> Self {
        Self::from_blueprint_instance(Instance::new(width, height, depth), blueprint)
    }

    /// Create a solver from a runtime instance and interpreted blueprint.
    pub fn from_blueprint_instance(instance: Instance, blueprint: &ResolvedBlueprint<C>) -> Self {
        Solver {
            grid: Grid::new(
                instance.dims().width,
                instance.dims().height,
                instance.dims().depth,
            ),
            attributes: AttributeStore::new(
                instance.dims().cell_count() + 1,
                blueprint.attributes(),
            ),
            topology: DescriptorTopology::new(blueprint.topology().descriptor()),
            program: Some(CompiledProgram::from_blueprint(blueprint)),
            step_count: 0,
            seed: instance.seed(),
        }
    }

    /// Interpret a declarative blueprint and create a solver from it.
    pub fn from_spec(width: u32, height: u32, depth: u32, blueprint: &Blueprint<C>) -> Self
    where
        C: Clone,
    {
        Self::from_spec_instance(Instance::new(width, height, depth), blueprint)
    }

    /// Interpret a declarative blueprint and create a solver from a runtime instance.
    pub fn from_spec_instance(instance: Instance, blueprint: &Blueprint<C>) -> Self
    where
        C: Clone,
    {
        let resolved = interpret_blueprint(blueprint);
        Self::from_blueprint_instance(instance, &resolved)
    }
}

impl<C: Cell + Eq, T: Topology> Solver<C, T> {
    /// Convert the solver to a new topology policy without changing its state.
    pub fn into_topology<U: Topology>(self, topology: U) -> Solver<C, U> {
        Solver {
            grid: self.grid,
            attributes: self.attributes,
            topology,
            program: self.program,
            step_count: self.step_count,
            seed: self.seed,
        }
    }

    /// The active topology policy used by this solver.
    pub fn topology(&self) -> &T {
        &self.topology
    }

    /// Replace the topology policy while preserving the solver state.
    pub fn set_topology<U: Topology>(self, topology: U) -> Solver<C, U> {
        self.into_topology(topology)
    }
}

impl<C: Cell + Eq, T: Topology> Solver<C, T> {
    fn step_program(&mut self) {
        let program = match &mut self.program {
            Some(program) => program,
            None => return,
        };

        let dims = self.grid.dims();
        let width = dims.width;
        let height = dims.height;
        let depth = dims.depth;
        let guard_idx = self.grid.guard_idx();
        let topology = &self.topology;
        let resolve = |x, y, z| resolve_index(topology, dims, guard_idx, x, y, z);
        let cells: &[C] = &self.grid.cells;

        for z in 0..depth as i32 {
            for y in 0..height as i32 {
                for x in 0..width as i32 {
                    let idx = (x as u32 + y as u32 * width + z as u32 * width * height) as usize;
                    let center = cells[idx];
                    let effect = program.evaluate(
                        center,
                        [x, y, z],
                        self.step_count,
                        self.seed,
                        |dx, dy, dz| cells[resolve(x + dx, y + dy, z + dz)],
                        |attribute| self.attributes.get(attribute, idx),
                    );

                    match effect {
                        Some(evaluation) => {
                            for update in evaluation.attribute_updates {
                                self.attributes
                                    .set_next(update.attribute, idx, update.value);
                            }
                            match evaluation.effect {
                                RuleEffect::Keep => {}
                                RuleEffect::Become(cell) => self.grid.cells_next[idx] = cell,
                            }
                        }
                        None => {}
                    }
                }
            }
        }
    }
}

impl<C: Cell + Eq, T: Topology> CaSolver<C> for Solver<C, T> {
    type Topology = T;

    fn width(&self) -> u32 {
        self.grid.width
    }

    fn height(&self) -> u32 {
        self.grid.height
    }

    fn depth(&self) -> u32 {
        self.grid.depth
    }

    fn topology(&self) -> &Self::Topology {
        &self.topology
    }

    fn seed(&self) -> u64 {
        self.seed
    }

    fn cell_count(&self) -> usize {
        self.grid.cell_count()
    }

    fn guard_index(&self) -> usize {
        self.grid.guard_idx()
    }

    fn get(&self, x: i32, y: i32, z: i32) -> C {
        self.grid.get(&self.topology, x, y, z)
    }

    fn set(&mut self, x: i32, y: i32, z: i32, cell: C) {
        self.grid.set(&self.topology, x, y, z, cell);
    }

    fn get_attr(
        &self,
        name: &str,
        x: i32,
        y: i32,
        z: i32,
    ) -> Result<AttributeValue, AttributeAccessError> {
        let Some(attribute) = self.attributes.index_of(name) else {
            return Err(AttributeAccessError::UnknownAttribute(name.to_string()));
        };

        let index = self.grid.resolve_idx(&self.topology, x, y, z);
        if index == self.grid.guard_idx() {
            Err(AttributeAccessError::OutOfBounds { x, y, z })
        } else {
            Ok(self.attributes.get(attribute, index))
        }
    }

    fn set_attr(
        &mut self,
        name: &str,
        x: i32,
        y: i32,
        z: i32,
        value: AttributeValue,
    ) -> Result<(), AttributeAccessError> {
        let Some(attribute) = self.attributes.index_of(name) else {
            return Err(AttributeAccessError::UnknownAttribute(name.to_string()));
        };

        let index = self.grid.resolve_idx(&self.topology, x, y, z);
        if index == self.grid.guard_idx() {
            return Err(AttributeAccessError::OutOfBounds { x, y, z });
        }

        self.attributes.set_current(attribute, index, value);
        Ok(())
    }

    fn step(&mut self) {
        self.grid.prepare_step();
        self.attributes.prepare_step();
        self.step_program();
        self.grid.swap();
        self.attributes.swap();
        self.step_count += 1;
    }

    fn step_count(&self) -> u32 {
        self.step_count
    }

    fn iter_cells(&self) -> Vec<(u32, u32, u32, C)> {
        self.grid.iter_cells()
    }

    fn readback(&self) -> GridSnapshot<C> {
        GridSnapshot::new(
            self.grid.dims(),
            self.grid.cells[..self.grid.cell_count()].to_vec(),
        )
    }

    fn read_region(&self, region: GridRegion) -> Vec<C> {
        let dims = self.grid.dims();
        assert!(
            dims.contains_region(region),
            "region must lie within solver dimensions"
        );

        let [ox, oy, oz] = region.origin;
        let [sx, sy, sz] = region.size;
        let width = self.grid.width as usize;
        let height = self.grid.height as usize;
        let mut cells = Vec::with_capacity(region.cell_count());

        for z in oz..oz + sz {
            for y in oy..oy + sy {
                for x in ox..ox + sx {
                    let index = (x as usize) + (y as usize) * width + (z as usize) * width * height;
                    cells.push(self.grid.cells[index]);
                }
            }
        }

        cells
    }

    fn write_region(&mut self, region: GridRegion, cells: &[C]) {
        let dims = self.grid.dims();
        assert!(
            dims.contains_region(region),
            "region must lie within solver dimensions"
        );
        assert_eq!(
            cells.len(),
            region.cell_count(),
            "region write must provide exactly one cell per destination slot"
        );

        let [ox, oy, oz] = region.origin;
        let [sx, sy, sz] = region.size;
        let width = self.grid.width as usize;
        let height = self.grid.height as usize;
        let mut src = 0;

        for z in oz..oz + sz {
            for y in oy..oy + sy {
                for x in ox..ox + sx {
                    let index = (x as usize) + (y as usize) * width + (z as usize) * width * height;
                    self.grid.cells[index] = cells[src];
                    src += 1;
                }
            }
        }
    }

    fn replace_cells(&mut self, cells: &[C]) {
        let cell_count = self.grid.cell_count();
        assert_eq!(
            cells.len(),
            cell_count,
            "full-grid replacement must match solver dimensions"
        );
        self.grid.cells[..cell_count].copy_from_slice(cells);
    }
}
