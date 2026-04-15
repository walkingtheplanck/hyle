//! Shared runtime-facing model traits.

mod attribute_access;
mod ca_runtime;
mod cell;
mod instance;
mod provider;
mod solver;
mod topology;

pub use attribute_access::AttributeAccessError;
pub use ca_runtime::CaRuntime;
pub use cell::Cell;
pub use instance::Instance;
pub use provider::CaSolverProvider;
pub use solver::{CaSolver, ValidatedSolver};
pub use topology::Topology;
