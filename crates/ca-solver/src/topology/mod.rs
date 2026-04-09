//! Built-in topology implementations for the default CPU solver.

mod bounded;
mod index;
mod torus;

pub use bounded::BoundedTopology;
pub use torus::TorusTopology;
