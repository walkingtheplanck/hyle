#![doc = include_str!("../README.md")]
#![deny(missing_docs)]

mod grid;
pub mod neighborhood;
mod program;
mod rng;
mod solver;
pub mod topology;

pub use neighborhood::{
    inverse_square, moore, spherical, unweighted, von_neumann, Entry, Neighborhood, Offset,
    ShapeFn, WeightFn,
};
pub use rng::Rng;
pub use solver::Solver;
pub use topology::{BoundedTopology, DescriptorTopology, TorusTopology};
