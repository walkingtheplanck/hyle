//! Execution-facing runtime traits.

mod provider;
mod runtime;
mod runtime_adapter;
mod solver;
mod validated;

pub use provider::CaSolverProvider;
pub use runtime::CaRuntime;
pub use runtime_adapter::Runtime;
pub use solver::CaSolver;
pub use validated::ValidatedSolver;
