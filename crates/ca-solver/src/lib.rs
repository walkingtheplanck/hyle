#![doc = include_str!("../README.md")]
#![deny(missing_docs)]

mod grid;
pub mod neighborhood;
mod program;
mod provider;
mod solver;
pub mod topology;

pub use neighborhood::{Entry, Neighborhood};
pub use provider::CpuSolverProvider;
pub use solver::Solver;
pub use topology::{BoundedTopology, DescriptorTopology, TorusTopology};
