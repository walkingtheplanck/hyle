#![doc = include_str!("../README.md")]
#![deny(missing_docs)]

mod diagnostic;
mod runtime;
mod spec;

pub use diagnostic::{Diagnostic, Severity, Subject};
pub use runtime::{
    analyze_cell, analyze_runtime, AttributeView, CellReport, MaterialPopulation, MaterialView,
    NeighborhoodMaterialCount, NeighborhoodReport, RuntimeReport,
};
pub use spec::{analyze_spec, NeighborhoodAnalysis, RuleAnalysis, SpecAnalysis, SpecSummary};
