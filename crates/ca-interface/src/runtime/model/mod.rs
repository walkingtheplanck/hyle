//! Small runtime data carriers shared by solvers and tools.

mod instance;
mod metrics;

pub use instance::Instance;
pub use metrics::TransitionCount;
