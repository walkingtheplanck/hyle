//! Canonical interpretation helpers derived from declarative contracts.

mod blueprint;
mod neighborhood;
mod offset;
mod random;
mod topology;

pub use blueprint::{interpret_blueprint, Blueprint, NamedNeighborhood};
pub use neighborhood::{
    expand_neighborhood, neighbor_count, offsets, samples, Neighborhood, NeighborhoodSample,
};
pub use offset::Offset3;
pub use random::cell_rng;
pub use topology::{interpret_topology, Topology};
