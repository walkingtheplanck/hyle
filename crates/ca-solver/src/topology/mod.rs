//! Built-in topology implementations for the default CPU solver.

mod bounded;
mod descriptor;
mod index;
mod torus;

pub use bounded::BoundedTopology;
pub use descriptor::DescriptorTopology;
pub use torus::TorusTopology;
