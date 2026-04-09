#![doc = include_str!("../README.md")]
#![deny(missing_docs)]

pub mod action;
pub mod backend;
pub mod cell;
pub mod grid;
pub mod neighborhood;
pub mod rng;
pub mod topology;
pub mod validated;

pub use action::Action;
pub use backend::CaSolver;
pub use cell::Cell;
pub use grid::{GridReader, GridWriter};
pub use neighborhood::{
    inverse_square, moore, spherical, unweighted, von_neumann, Entry, Neighborhood, Offset,
    ShapeFn, WeightFn,
};
pub use rng::Rng;
pub use topology::{BoundedTopology, Topology, TorusTopology};
pub use validated::ValidatedSolver;

/// A rule function: given a neighborhood and RNG, return what happens to the center cell.
pub type Rule<C> = fn(&Neighborhood<C>, Rng) -> Action<C>;

/// A world pass: full grid access, runs as a separate stage after all per-cell rules.
/// Use for global operations like pressure solving, gravity fields, or conservation correction.
pub type WorldPass<C> = fn(&GridReader<C>, &mut GridWriter<C>);
