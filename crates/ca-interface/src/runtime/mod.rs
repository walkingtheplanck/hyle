//! Shared runtime-facing execution traits and query models.

mod model;
mod query;
mod traits;
mod topology;

pub use model::{Instance, TransitionCount};
pub use query::{AttributeAccessError, CellAttributeValue, CellId, CellQueryError};
pub use traits::{CaRuntime, CaSolver, CaSolverProvider, Runtime, ValidatedSolver};
pub use topology::Topology;
