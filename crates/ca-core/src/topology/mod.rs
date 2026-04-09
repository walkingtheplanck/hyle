//! Coordinate-topology policies for solver access.

mod bounded;
mod index;
mod policy;
mod torus;

pub use bounded::BoundedTopology;
pub use policy::Topology;
pub use torus::TorusTopology;
