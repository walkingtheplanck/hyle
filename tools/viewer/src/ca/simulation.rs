//! Viewer-facing simulation wrapper over CA runtimes.

use std::time::Instant;

use hyle_ca_interface::{CaSolverProvider, MaterialSet, RuntimeGrid, RuntimeStepping};

use crate::ca::{Materials, Scenario, SimpleWorld, ViewerCell, ViewerResult, AIR};

pub struct StepOutcome {
    pub stepped: bool,
}

pub struct Simulation<P>
where
    P: CaSolverProvider,
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
    P: CaSolverProvider,
{
    pub fn new(solver: P) -> ViewerResult<Self> {
        let scenario = Scenario::default();
        let ca = Self::build_ca(&solver, scenario)?;

        Ok(Self {
            auto_step: true,
            step_interval_ms: scenario.step_interval_ms(),
            solver,
            ca,
            scenario,
            last_step: Instant::now(),
        })
    }

    pub fn scenario(&self) -> Scenario {
        self.scenario
    }

    pub fn materials(&self) -> Materials {
        self.scenario.materials()
    }

    pub fn runtime(&self) -> &P::Runtime {
        &self.ca
    }

    fn build_ca(solver: &P, scenario: Scenario) -> ViewerResult<P::Runtime> {
        let spec = scenario.blueprint()?;
        let mut ca = solver.build(scenario.instance(), &spec);
        scenario.seed(&mut ca)?;
        Ok(ca)
    }

    pub fn set_scenario(
        &mut self,
        scenario: Scenario,
        world: &mut SimpleWorld,
    ) -> ViewerResult<bool> {
        if scenario == self.scenario {
            return Ok(false);
        }

        let ca = Self::build_ca(&self.solver, scenario)?;
        self.ca = ca;
        self.scenario = scenario;
        self.step_interval_ms = scenario.step_interval_ms();
        self.last_step = Instant::now();
        self.sync_to_world(world)?;
        Ok(true)
    }

    pub fn step(
        &mut self,
        world: &mut SimpleWorld,
        with_report: bool,
    ) -> ViewerResult<StepOutcome> {
        let _ = with_report;
        self.ca.step();
        self.sync_to_world(world)?;
        Ok(StepOutcome { stepped: true })
    }

    pub fn reset(&mut self, world: &mut SimpleWorld) -> ViewerResult<()> {
        self.ca = Self::build_ca(&self.solver, self.scenario)?;
        self.last_step = Instant::now();
        self.sync_to_world(world)
    }

    pub fn maybe_auto_step(
        &mut self,
        world: &mut SimpleWorld,
        with_report: bool,
    ) -> ViewerResult<StepOutcome> {
        if !self.auto_step {
            return Ok(StepOutcome { stepped: false });
        }
        if self.last_step.elapsed().as_secs_f64() * 1000.0 >= self.step_interval_ms {
            self.last_step = Instant::now();
            self.step(world, with_report)
        } else {
            Ok(StepOutcome { stepped: false })
        }
    }

    fn sync_to_world(&self, world: &mut SimpleWorld) -> ViewerResult<()> {
        let snapshot = self.ca.readback();
        let dead_material = ViewerCell::Dead
            .id()
            .map_err(|error| format!("failed to sync the viewer world: {error}"))?;
        for (x, y, z, cell) in snapshot.iter_xyz() {
            let voxel = match cell {
                id if *id == dead_material => AIR,
                id => id.raw(),
            };
            world.set(x as i32, y as i32, z as i32, voxel);
        }

        Ok(())
    }
}
