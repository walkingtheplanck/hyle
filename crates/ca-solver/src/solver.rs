//! Default CPU solver - double-buffered, single-threaded.

use hyle_ca_core::{
    Action, BoundedTopology, CaSolver, Cell, GridRegion, GridSnapshot, NeighborhoodSpec, Rng,
    Topology,
};

use crate::grid::{resolve_index, Grid};
use crate::rule_set::{install_rule_set, RuleSet};
use crate::rules::{BoxedWorldPass, RegisteredRule};
use crate::{moore, unweighted, GridReader, GridWriter, Neighborhood, ShapeFn, WeightFn};

/// Default 3D cellular automaton solver, generic over cell type `C`.
///
/// Rules are Rust closures - register them with `register_rule()`,
/// `register_rule_with_radius()`, or `register_rule_with_shape()`.
/// World passes run after all per-cell rules.
pub struct Solver<C: Cell = u32, T: Topology = BoundedTopology> {
    grid: Grid<C>,
    topology: T,

    /// Per-cell rules indexed by cell type (0-255).
    rules: Vec<Option<RegisteredRule<C>>>,

    /// World passes, run in registration order after all per-cell rules.
    world_passes: Vec<BoxedWorldPass<C>>,

    step_count: u32,
}

impl<C: Cell> Solver<C, BoundedTopology> {
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
        let mut rules = Vec::with_capacity(256);
        rules.resize_with(256, || None);
        Solver {
            grid: Grid::new(width, height, depth),
            topology,
            rules,
            world_passes: Vec::new(),
            step_count: 0,
        }
    }
}

impl<C: Cell, T: Topology> Solver<C, T> {
    /// Convert the solver to a new topology policy without changing its state.
    pub fn into_topology<U: Topology>(self, topology: U) -> Solver<C, U> {
        Solver {
            grid: self.grid,
            topology,
            rules: self.rules,
            world_passes: self.world_passes,
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

    /// Register a per-cell rule with radius 1 and Moore neighborhood (26 neighbors).
    pub fn register_rule(
        &mut self,
        cell_type: u8,
        rule: impl Fn(&Neighborhood<C>, Rng) -> Action<C> + 'static,
    ) {
        self.rules[cell_type as usize] =
            Some(RegisteredRule::with_default_neighborhood(Box::new(rule)));
    }

    /// Register a per-cell rule with a custom radius and Moore neighborhood.
    pub fn register_rule_with_radius(
        &mut self,
        cell_type: u8,
        radius: u32,
        rule: impl Fn(&Neighborhood<C>, Rng) -> Action<C> + 'static,
    ) {
        self.rules[cell_type as usize] = Some(RegisteredRule::new(
            radius,
            moore,
            unweighted,
            Box::new(rule),
        ));
    }

    /// Register a per-cell rule with a custom radius, shape, and weight.
    pub fn register_rule_with_shape(
        &mut self,
        cell_type: u8,
        radius: u32,
        shape: ShapeFn,
        weight: WeightFn,
        rule: impl Fn(&Neighborhood<C>, Rng) -> Action<C> + 'static,
    ) {
        self.rules[cell_type as usize] =
            Some(RegisteredRule::new(radius, shape, weight, Box::new(rule)));
    }

    /// Register a per-cell rule from a declarative neighborhood specification.
    pub fn register_rule_with_spec(
        &mut self,
        cell_type: u8,
        spec: NeighborhoodSpec,
        rule: impl Fn(&Neighborhood<C>, Rng) -> Action<C> + 'static,
    ) {
        self.rules[cell_type as usize] = Some(RegisteredRule::with_spec(spec, Box::new(rule)));
    }

    /// Register a world pass. Runs after all per-cell rules, in registration order.
    pub fn register_world_pass(
        &mut self,
        pass: impl Fn(&GridReader<C>, &mut GridWriter<C>) + 'static,
    ) {
        self.world_passes.push(Box::new(pass));
    }

    /// Install a named batch of rules and world passes.
    ///
    /// Per-cell rules still follow the same semantics as direct registration:
    /// later registrations override earlier ones for the same `cell_type`.
    /// World passes are appended and run in installation order.
    pub fn install_rule_set(&mut self, rule_set: RuleSet<C>) {
        install_rule_set(&mut self.rules, &mut self.world_passes, rule_set);
    }

    /// Evaluate per-cell rules.
    fn step_cell_rules(&mut self) {
        let dims = self.grid.dims();
        let w = dims.width;
        let h = dims.height;
        let d = dims.depth;
        let guard_idx = self.grid.guard_idx();
        let step_count = self.step_count;
        let topology = &self.topology;
        let resolve = |x, y, z| resolve_index(topology, dims, guard_idx, x, y, z);
        let cells: &[C] = &self.grid.cells;

        for z in 0..d as i32 {
            for y in 0..h as i32 {
                for x in 0..w as i32 {
                    let idx = (x as u32 + y as u32 * w + z as u32 * w * h) as usize;
                    let center = cells[idx];
                    let cell_type = center.rule_id() as usize;

                    let reg = match &mut self.rules[cell_type] {
                        Some(r) => r,
                        None => continue,
                    };

                    reg.neighborhood.fill(center, [x, y, z], |dx, dy, dz| {
                        cells[resolve(x + dx, y + dy, z + dz)]
                    });

                    let action = (reg.rule)(
                        &reg.neighborhood,
                        Rng::new(x as u32, y as u32, z as u32, step_count),
                    );

                    if let Action::Become(c) = action {
                        self.grid.cells_next[idx] = c;
                    }
                }
            }
        }
    }

    /// Run world passes sequentially over cells_next.
    fn step_world_passes(&mut self) {
        if self.world_passes.is_empty() {
            return;
        }

        let mut pass_read = self.grid.cells_next.clone();
        let dims = self.grid.dims();
        let width = dims.width;
        let height = dims.height;
        let depth = dims.depth;
        let guard_idx = self.grid.guard_idx();
        let topology = &self.topology;
        let resolve = |x, y, z| resolve_index(topology, dims, guard_idx, x, y, z);

        for pass in &self.world_passes {
            let reader = GridReader::new(&pass_read, width, height, depth, &resolve);
            let mut writer =
                GridWriter::new(&mut self.grid.cells_next, width, height, depth, &resolve);
            pass(&reader, &mut writer);
            pass_read.copy_from_slice(&self.grid.cells_next);
        }
    }
}

impl<C: Cell, T: Topology> CaSolver<C> for Solver<C, T> {
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
        self.step_cell_rules();
        self.step_world_passes();
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
