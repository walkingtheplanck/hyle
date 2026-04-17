//! Execution-facing runtime traits.

mod provider;
mod runtime;
mod runtime_adapter;
mod runtime_attributes;
mod runtime_cells;
mod runtime_grid;
mod runtime_metadata;
mod runtime_metrics;
mod runtime_stepping;
mod solver;
mod solver_attributes;
mod solver_cells;
mod solver_execution;
mod solver_grid;
mod solver_metadata;
mod solver_metrics;
mod validated;

pub use provider::CaSolverProvider;
pub use runtime::CaRuntime;
pub use runtime_attributes::RuntimeAttributes;
pub use runtime_cells::RuntimeCells;
pub use runtime_grid::RuntimeGrid;
pub use runtime_metadata::RuntimeMetadata;
pub use runtime_metrics::RuntimeMetrics;
pub use runtime_stepping::RuntimeStepping;
pub use runtime_adapter::Runtime;
pub use solver::CaSolver;
pub use solver_attributes::SolverAttributes;
pub use solver_cells::SolverCells;
pub use solver_execution::SolverExecution;
pub use solver_grid::SolverGrid;
pub use solver_metadata::SolverMetadata;
pub use solver_metrics::SolverMetrics;
pub use validated::ValidatedSolver;
