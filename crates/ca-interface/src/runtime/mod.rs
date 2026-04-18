//! Shared runtime-facing execution traits, errors, and data models.

mod errors;
mod model;
mod topology;
mod traits;

pub use errors::{AttributeAccessError, CellQueryError, GridAccessError};
pub use model::{CellAttributeValue, Instance, TransitionCount};
pub use topology::Topology;
pub use traits::{
    CaRuntime, CaSolver, CaSolverProvider, Runtime, RuntimeAttributes, RuntimeCells, RuntimeGrid,
    RuntimeMetadata, RuntimeMetrics, RuntimeStepping, SolverAttributes, SolverCells,
    SolverExecution, SolverGrid, SolverMetadata, SolverMetrics, ValidatedSolver,
};
