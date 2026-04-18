//! Solver-facing capability traits.

mod attributes;
mod ca_solver;
mod cells;
mod execution;
mod grid;
mod metadata;
mod metrics;

pub use attributes::SolverAttributes;
pub use ca_solver::CaSolver;
pub use cells::SolverCells;
pub use execution::SolverExecution;
pub use grid::SolverGrid;
pub use metadata::SolverMetadata;
pub use metrics::SolverMetrics;
