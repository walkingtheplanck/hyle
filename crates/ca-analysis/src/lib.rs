#![doc = include_str!("../README.md")]
#![deny(missing_docs)]

mod diagnostic;
mod spec;

pub use diagnostic::{Diagnostic, Severity, Subject};
pub use spec::{analyze_spec, NeighborhoodAnalysis, RuleAnalysis, SpecAnalysis, SpecSummary};
