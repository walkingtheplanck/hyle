//! 3D cellular automaton simulation using hyle-ca.
//!
//! Uses "Life 4555" — a classic 3D rule discovered by Carter Bays (1987).
//! S4-5/B5 with 26-neighbor Moore neighborhood.
//! Known to produce gliders and structured patterns in 3D.

use std::time::Instant;

use hyle_ca_contracts::{neighbors, CaSolver, Hyle};
use hyle_ca_solver::{Rng, Solver};

use crate::world::{self, SimpleWorld};

const ALIVE: u32 = 1;
const DEAD: u32 = 0;

pub struct Simulation {
    pub auto_step: bool,
    pub step_interval_ms: f64,
    ca: Solver<u32>,
    last_step: Instant,
}

impl Simulation {
    pub fn new() -> Self {
        Self {
            auto_step: true,
            step_interval_ms: 200.0,
            ca: Self::build_ca(),
            last_step: Instant::now(),
        }
    }

    fn build_ca() -> Solver<u32> {
        let spec = Hyle::builder()
            .cells::<u32>()
            .rules(|rules| {
                rules
                    .when(ALIVE)
                    .unless(neighbors(ALIVE).count().in_range(4..=5))
                    .becomes(DEAD);
                rules
                    .when(DEAD)
                    .require(neighbors(ALIVE).count().eq(5))
                    .becomes(ALIVE);
            })
            .build()
            .expect("viewer life spec should build");
        let mut ca = Solver::from_spec(64, 64, 64, &spec);

        Self::seed(&mut ca);
        ca
    }

    /// Seed: ~18% random fill in a 16³ region at center.
    fn seed(ca: &mut Solver<u32>) {
        for z in 24u32..40 {
            for y in 24u32..40 {
                for x in 24u32..40 {
                    if Rng::new(x, y, z, 0).chance(6) {
                        ca.set(x as i32, y as i32, z as i32, ALIVE);
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
        self.ca = Self::build_ca();
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
        for (x, y, z, cell) in self.ca.iter_cells() {
            world.set(
                x as i32,
                y as i32,
                z as i32,
                if cell == ALIVE { 1 } else { world::AIR },
            );
        }
    }
}
