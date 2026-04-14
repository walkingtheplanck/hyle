//! Viewer-facing simulation wrapper over CA runtimes.

use std::time::Instant;

use hyle_ca_interface::{CaRuntime, CaSolverProvider};

use crate::ca::{Materials, Scenario, SimpleWorld, ViewerCell, AIR};

pub struct Simulation<P>
where
    P: CaSolverProvider<ViewerCell>,
{
    pub auto_step: bool,
    pub step_interval_ms: f64,
    solver: P,
    ca: P::Runtime,
    scenario: Scenario,
    last_step: Instant,
}

impl<P> Simulation<P>
where
    P: CaSolverProvider<ViewerCell>,
{
    pub fn new(solver: P) -> Self {
        let scenario = Scenario::default();
        let mut ca = Self::build_ca(&solver, scenario);
        scenario.seed(&mut ca);

        Self {
            auto_step: true,
            step_interval_ms: scenario.step_interval_ms(),
            solver,
            ca,
            scenario,
            last_step: Instant::now(),
        }
    }

    pub fn scenario(&self) -> Scenario {
        self.scenario
    }

    pub fn materials(&self) -> Materials {
        self.scenario.materials()
    }

    fn build_ca(solver: &P, scenario: Scenario) -> P::Runtime {
        let spec = scenario.blueprint();
        solver.build(scenario.instance(), &spec)
    }

    pub fn set_scenario(&mut self, scenario: Scenario, world: &mut SimpleWorld) -> bool {
        if scenario == self.scenario {
            return false;
        }

        self.scenario = scenario;
        self.step_interval_ms = scenario.step_interval_ms();
        self.reset(world);
        true
    }

    pub fn step(&mut self, world: &mut SimpleWorld) -> bool {
        self.ca.step();
        self.sync_to_world(world);
        true
    }

    pub fn reset(&mut self, world: &mut SimpleWorld) {
        self.ca = Self::build_ca(&self.solver, self.scenario);
        self.scenario.seed(&mut self.ca);
        self.last_step = Instant::now();
        self.sync_to_world(world);
    }

    pub fn maybe_auto_step(&mut self, world: &mut SimpleWorld) -> bool {
        if !self.auto_step {
            return false;
        }
        if self.last_step.elapsed().as_secs_f64() * 1000.0 >= self.step_interval_ms {
            self.last_step = Instant::now();
            self.step(world)
        } else {
            false
        }
    }

    fn sync_to_world(&self, world: &mut SimpleWorld) {
        let snapshot = self.ca.readback();
        for (x, y, z, cell) in snapshot.iter_xyz() {
            let voxel = match cell {
                ViewerCell::Dead => AIR,
                _ => cell.voxel(),
            };
            world.set(x as i32, y as i32, z as i32, voxel);
        }
    }
}
