//! Runtime analysis over completed solver step reports.

mod analyze;
mod report;

pub use analyze::analyze_runtime;
pub use report::{MaterialPopulation, RuntimeReport};
