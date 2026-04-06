//! Core CA types: Cell trait, Neighborhood, Action, grid views, RNG.

mod action;
mod grid;
mod neighborhood;
mod rng;
mod traits;

pub use action::Action;
pub use grid::{GridReader, GridWriter};
pub use neighborhood::Neighborhood;
pub use rng::Rng;
pub use traits::Cell;
