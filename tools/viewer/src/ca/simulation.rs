//! 3D cellular blueprint simulation using hyle-ca.
//!
//! Uses "Life 4555" — a classic 3D rule discovered by Carter Bays (1987).
//! S4-5/B5 with 26-neighbor Moore neighborhood.
//! Known to produce gliders and structured patterns in 3D.

use std::time::Instant;

use hyle_ca_interface::{
    neighbors, Blueprint, CaRuntime, CaSolverProvider, CellModel, CellSchema, Instance, Rng,
    StateDef,
};

use crate::ca::{SimpleWorld, AIR};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub(crate) enum LifeCell {
    #[default]
    Dead,
    Alive,
}

const LIFE_CELL_STATES: [StateDef; 2] = [StateDef::new("Dead"), StateDef::new("Alive")];

impl CellModel for LifeCell {
    fn schema() -> CellSchema {
        CellSchema::enumeration("LifeCell", &LIFE_CELL_STATES)
    }
}

pub struct Simulation<P>
where
    P: CaSolverProvider<LifeCell>,
{
    pub auto_step: bool,
    pub step_interval_ms: f64,
    solver: P,
    ca: P::Runtime,
    last_step: Instant,
}

impl<P> Simulation<P>
where
    P: CaSolverProvider<LifeCell>,
{
    pub fn new(solver: P) -> Self {
        let mut ca = Self::build_ca(&solver);
        Self::seed(&mut ca);

        Self {
            auto_step: true,
            step_interval_ms: 200.0,
            solver,
            ca,
            last_step: Instant::now(),
        }
    }

    fn spec() -> Blueprint<LifeCell> {
        Blueprint::<LifeCell>::builder()
            .rules(|rules| {
                rules
                    .when(LifeCell::Alive)
                    .unless(neighbors(LifeCell::Alive).count().in_range(4..=5))
                    .becomes(LifeCell::Dead);
                rules
                    .when(LifeCell::Dead)
                    .require(neighbors(LifeCell::Alive).count().eq(5))
                    .becomes(LifeCell::Alive);
            })
            .build()
            .expect("viewer life spec should build")
    }

    fn instance() -> Instance {
        Instance::new(64, 64, 64).with_seed(1)
    }

    fn build_ca(solver: &P) -> P::Runtime {
        let spec = Self::spec();
        solver.build(Self::instance(), &spec)
    }

    /// Seed: ~18% random fill in a 16^3 region at center.
    fn seed(ca: &mut impl CaRuntime<LifeCell>) {
        for z in 24u32..40 {
            for y in 24u32..40 {
                for x in 24u32..40 {
                    if Rng::with_seed(x, y, z, 0, 1).chance(6) {
                        ca.set(x as i32, y as i32, z as i32, LifeCell::Alive);
                    }
                }
            }
        }
    }

    /// Run one step and sync to the voxel world.
    pub fn step(&mut self, world: &mut SimpleWorld) -> bool {
        self.ca.step();
        self.sync_to_world(world);
        true
    }

    pub fn reset(&mut self, world: &mut SimpleWorld) {
        self.ca = Self::build_ca(&self.solver);
        Self::seed(&mut self.ca);
        self.sync_to_world(world);
    }

    /// Auto-step if enough time has elapsed. Returns true if a step ran.
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
            world.set(
                x as i32,
                y as i32,
                z as i32,
                if *cell == LifeCell::Alive { 1 } else { AIR },
            );
        }
    }
}
