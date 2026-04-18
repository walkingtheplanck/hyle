//! Execution-facing runtime traits.

mod provider;
mod runtime;
mod solver;

pub use provider::CaSolverProvider;
pub use runtime::{
    CaRuntime, Runtime, RuntimeAttributes, RuntimeCells, RuntimeGrid, RuntimeMetadata,
    RuntimeMetrics, RuntimeStepping,
};
pub use solver::{
    CaSolver, SolverAttributes, SolverCells, SolverExecution, SolverGrid, SolverMetadata,
    SolverMetrics,
};
