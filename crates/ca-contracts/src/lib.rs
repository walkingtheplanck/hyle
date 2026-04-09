#![doc = include_str!("../README.md")]
#![deny(missing_docs)]

pub mod action;
pub mod backend;
pub mod cell;
/// Declarative, backend-neutral data structures shared across solvers.
pub mod descriptors;
pub mod topology;
pub mod validated;

pub use action::Action;
pub use backend::CaSolver;
pub use cell::Cell;
pub use descriptors::{
    AxisTopology, GridDims, GridRegion, GridSnapshot, NeighborhoodShape, NeighborhoodSpec,
    NeighborhoodWeight, TopologyDescriptor,
};
pub use topology::Topology;
pub use validated::ValidatedSolver;
