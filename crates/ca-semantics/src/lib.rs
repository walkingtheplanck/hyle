#![doc = include_str!("../README.md")]
#![deny(missing_docs)]

/// Canonical neighborhood expansion helpers.
pub mod neighborhood;
/// Shared semantic offset types.
pub mod offset;

pub use neighborhood::{expand_neighborhood, neighbor_count, offsets, ExpandedNeighborhood};
pub use offset::Offset3;
