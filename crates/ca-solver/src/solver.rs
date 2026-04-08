//! Default CPU solver - double-buffered, single-threaded.

use hyle_ca_core::{
    moore, unweighted, Action, CaSolver, Cell, GridReader, GridWriter, Neighborhood, Rng, ShapeFn,
    Topology, WeightFn,
};

use crate::grid::{resolve_coord, Grid};
use crate::rule_set::{install_rule_set, RuleSet};
use crate::rules::{BoxedWorldPass, RegisteredRule};

/// Default 3D cellular automaton solver, generic over cell type `C`.
///
/// Rules are Rust closures - register them with `register_rule()`,
/// `register_rule_with_radius()`, or `register_rule_with_shape()`.
/// World passes run after all per-cell rules.
pub struct Solver<C: Cell = u32> {
    grid: Grid<C>,
    topology: Topology,

    /// Per-cell rules indexed by cell type (0-255).
    rules: Vec<Option<RegisteredRule<C>>>,

    /// World passes, run in registration order after all per-cell rules.
    world_passes: Vec<BoxedWorldPass<C>>,

    step_count: u32,
}

impl<C: Cell> Solver<C> {
    /// Create a new bounded solver filled with `C::default()`.
    pub fn new(width: u32, height: u32, depth: u32) -> Self {
        Self::with_topology(width, height, depth, Topology::Bounded)
    }

    /// Create a new solver filled with `C::default()` and the given topology.
    pub fn with_topology(width: u32, height: u32, depth: u32, topology: Topology) -> Self {
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

    /// Set the solver topology for future reads, writes, and steps.
    pub fn set_topology(&mut self, topology: Topology) {
        self.topology = topology;
    }

    /// The active topology used by this solver.
    pub fn topology(&self) -> Topology {
        self.topology
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
        let w = self.grid.width;
        let h = self.grid.height;
        let d = self.grid.depth;
        let step_count = self.step_count;
        let topology = self.topology;
        let resolve = |x, y, z| resolve_coord(topology, w, h, d, x, y, z);
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
                        match resolve(x + dx, y + dy, z + dz) {
                            Some((nx, ny, nz)) => cells[self.grid.idx(nx, ny, nz)],
                            None => C::default(),
                        }
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
        let width = self.grid.width;
        let height = self.grid.height;
        let depth = self.grid.depth;
        let topology = self.topology;
        let resolve = |x, y, z| resolve_coord(topology, width, height, depth, x, y, z);

        for pass in &self.world_passes {
            let reader = GridReader::new(&pass_read, width, height, depth, &resolve);
            let mut writer =
                GridWriter::new(&mut self.grid.cells_next, width, height, depth, &resolve);
            pass(&reader, &mut writer);
            pass_read.copy_from_slice(&self.grid.cells_next);
        }
    }
}

impl<C: Cell> CaSolver<C> for Solver<C> {
    fn width(&self) -> u32 {
        self.grid.width
    }

    fn height(&self) -> u32 {
        self.grid.height
    }

    fn depth(&self) -> u32 {
        self.grid.depth
    }

    fn resolve_coord(&self, x: i32, y: i32, z: i32) -> Option<(u32, u32, u32)> {
        resolve_coord(
            self.topology,
            self.grid.width,
            self.grid.height,
            self.grid.depth,
            x,
            y,
            z,
        )
    }

    fn get(&self, x: i32, y: i32, z: i32) -> C {
        self.grid.get(self.topology, x, y, z)
    }

    fn set(&mut self, x: i32, y: i32, z: i32, cell: C) {
        self.grid.set(self.topology, x, y, z, cell);
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
}
