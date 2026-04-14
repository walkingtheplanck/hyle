#![doc = include_str!("../README.md")]
#![deny(missing_docs)]

mod attributes;
mod grid;
mod neighborhood;
mod program;
mod provider;
mod solver;
mod topology;

pub use neighborhood::{Entry, Neighborhood};
pub use provider::CpuSolverProvider;
pub use solver::{AttributeWriteError, Solver};
pub use topology::{BoundedTopology, DescriptorTopology, TorusTopology};
