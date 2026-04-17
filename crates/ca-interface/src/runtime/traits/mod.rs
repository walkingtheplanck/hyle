//! Execution-facing runtime traits.

mod provider;
mod runtime;
mod solver;
mod validated;

pub use provider::CaSolverProvider;
pub use runtime::CaRuntime;
pub use solver::CaSolver;
pub use validated::ValidatedSolver;
