//! Cellular-schema state and world data for the viewer.

mod error;
mod scenarios;
mod simulation;
mod world;

pub(crate) use error::ViewerResult;
pub use scenarios::Scenario;
pub(crate) use scenarios::ViewerCell;
pub use simulation::Simulation;
pub use world::{viewer_world, Aabb, Materials, SimpleWorld, AIR};
