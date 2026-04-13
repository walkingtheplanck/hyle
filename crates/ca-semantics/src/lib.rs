#![doc = include_str!("../README.md")]
#![deny(missing_docs)]

/// Canonical blueprint interpretation helpers.
pub mod blueprint;
/// Canonical neighborhood expansion helpers.
pub mod neighborhood;
/// Shared semantic offset types.
pub mod offset;
/// Canonical topology interpretation helpers.
pub mod topology;

pub use blueprint::{interpret_blueprint, Blueprint, NamedNeighborhood};
pub use neighborhood::{
    expand_neighborhood, neighbor_count, offsets, samples, Neighborhood, NeighborhoodSample,
};
pub use offset::Offset3;
pub use topology::{interpret_topology, Topology};
