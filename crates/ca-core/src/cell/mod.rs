//! Core CA types: Cell trait, Neighborhood, Action, grid views, RNG.

mod action;
mod grid;
mod neighborhood;
mod rng;
mod traits;

pub use action::Action;
pub use grid::{GridReader, GridWriter};
pub use neighborhood::{
    inverse_square, MooreNeighborhood, Neighborhood, SphericalNeighborhood, VonNeumannNeighborhood,
};
pub use rng::Rng;
pub use traits::Cell;
