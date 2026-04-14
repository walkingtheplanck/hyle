//! Canonical interpretation helpers derived from declarative contracts.

mod blueprint;
mod neighborhood;
mod offset;
mod random;
mod rng;
mod topology;

pub use crate::WEIGHT_SCALE;
pub use blueprint::{interpret_blueprint, Blueprint, NamedNeighborhood};
pub use neighborhood::{
    expand_neighborhood, max_weighted_sum, neighbor_count, offsets, samples, Neighborhood,
    NeighborhoodSample,
};
pub use offset::Offset3;
pub use random::cell_rng;
pub use rng::Rng;
pub use topology::{interpret_topology, ResolvedTopology};
