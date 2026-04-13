//! Shared runtime-facing model traits.

mod cell;
mod solver;
mod topology;

pub use cell::Cell;
pub use solver::{CaSolver, ValidatedSolver};
pub use topology::Topology;
