//! Runtime analysis over completed solver step reports.

mod analyze;
mod report;

pub use analyze::{analyze_cell, analyze_runtime};
pub use report::{
    AttributeView, CellReport, MaterialPopulation, MaterialView, NeighborhoodMaterialCount,
    NeighborhoodReport, RuntimeReport,
};
