#![doc = include_str!("../README.md")]
#![deny(missing_docs)]

mod grid;
pub mod neighborhood;
mod program;
mod rng;
mod solver;
pub mod topology;

pub use neighborhood::{Entry, Neighborhood};
pub use rng::Rng;
pub use solver::Solver;
pub use topology::{BoundedTopology, DescriptorTopology, TorusTopology};
