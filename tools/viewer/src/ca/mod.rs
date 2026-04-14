//! Cellular-blueprint state and world data for the viewer.

mod simulation;
mod world;

pub(crate) use simulation::LifeCell;
pub use simulation::Simulation;
pub use world::{gol_world, Aabb, Materials, SimpleWorld, AIR};
