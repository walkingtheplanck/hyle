//! Shared runtime-facing model traits.

mod cell;
mod instance;
mod provider;
mod runtime;
mod solver;
mod topology;

pub use cell::Cell;
pub use instance::Instance;
pub use provider::CaSolverProvider;
pub use runtime::CaRuntime;
pub use solver::{CaSolver, ValidatedSolver};
pub use topology::Topology;
