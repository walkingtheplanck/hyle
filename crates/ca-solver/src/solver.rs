//! Default CPU solver - double-buffered, single-threaded.

use hyle_ca_contracts::{
    BlueprintSpec, CaSolver, Cell, GridRegion, GridSnapshot, RuleEffect, Topology,
};
use hyle_ca_semantics::{interpret_blueprint, Blueprint};

use crate::grid::{resolve_index, Grid};
use crate::program::CompiledProgram;
use crate::{BoundedTopology, DescriptorTopology};

/// Default 3D cellular automaton solver, generic over cell type `C`.
///
/// The solver can run without an attached blueprint, in which case `step()`
/// preserves the current state. Use [`Solver::from_blueprint`] to construct a
/// solver from an interpreted [`Blueprint`], or [`Solver::from_spec`] to
/// interpret a declarative [`BlueprintSpec`] and construct one in a single step.
pub struct Solver<C: Cell + Eq = u32, T: Topology = BoundedTopology> {
    grid: Grid<C>,
    topology: T,
    program: Option<CompiledProgram<C>>,
    step_count: u32,
}

impl<C: Cell + Eq> Solver<C, BoundedTopology> {
    /// Create a new bounded solver filled with `C::default()`.
    pub fn new(width: u32, height: u32, depth: u32) -> Self {
        Self::with_topology(width, height, depth, BoundedTopology)
    }

    /// Create a new solver filled with `C::default()` and the given topology.
    pub fn with_topology<U: Topology>(
        width: u32,
        height: u32,
        depth: u32,
        topology: U,
    ) -> Solver<C, U> {
        Solver {
            grid: Grid::new(width, height, depth),
            topology,
            program: None,
            step_count: 0,
        }
    }
}

impl<C: Cell + Eq> Solver<C, DescriptorTopology> {
    /// Create a solver whose topology and rules come from an interpreted blueprint.
    pub fn from_blueprint(width: u32, height: u32, depth: u32, blueprint: &Blueprint<C>) -> Self {
        Solver {
            grid: Grid::new(width, height, depth),
            topology: DescriptorTopology::new(blueprint.topology()),
            program: Some(CompiledProgram::from_blueprint(blueprint)),
            step_count: 0,
        }
    }

    /// Interpret a declarative blueprint spec and create a solver from it.
    pub fn from_spec(width: u32, height: u32, depth: u32, spec: &BlueprintSpec<C>) -> Self
    where
        C: Clone,
    {
        let blueprint = interpret_blueprint(spec);
        Self::from_blueprint(width, height, depth, &blueprint)
    }
}

impl<C: Cell + Eq, T: Topology> Solver<C, T> {
    /// Convert the solver to a new topology policy without changing its state.
    pub fn into_topology<U: Topology>(self, topology: U) -> Solver<C, U> {
        Solver {
            grid: self.grid,
            topology,
            program: self.program,
            step_count: self.step_count,
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
                    let effect = program.evaluate(center, [x, y, z], |dx, dy, dz| {
                        cells[resolve(x + dx, y + dy, z + dz)]
                    });

                    match effect {
                        Some(RuleEffect::Keep) | None => {}
                        Some(RuleEffect::Become(cell)) => self.grid.cells_next[idx] = cell,
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

    fn step(&mut self) {
        self.grid.prepare_step();
        self.step_program();
        self.grid.swap();
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
