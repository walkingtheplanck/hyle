//! Cellular-automaton state and world data for the viewer.

mod simulation;
mod world;

pub use simulation::Simulation;
pub use world::{gol_world, Aabb, Materials, SimpleWorld, AIR};
