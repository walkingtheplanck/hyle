//! Shared runtime-facing model traits.

mod cell;
mod instance;
mod solver;
mod topology;

pub use cell::Cell;
pub use instance::Instance;
pub use solver::{CaSolver, ValidatedSolver};
pub use topology::Topology;
