//! Static analysis over declarative blueprints.

mod analyze;
mod neighborhoods;
mod report;
mod rules;

pub use analyze::analyze_spec;
pub use report::{NeighborhoodAnalysis, RuleAnalysis, SpecAnalysis, SpecSummary};
