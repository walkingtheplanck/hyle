//! Public report types returned by runtime analysis.

use hyle_ca_interface::{MaterialId, TransitionCount};

/// Population count for one material after a completed step.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MaterialPopulation {
    /// Material identifier.
    pub material: MaterialId,
    /// Number of cells with that material after the step.
    pub count: u64,
}

/// Higher-level runtime summary derived from one solver step report.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RuntimeReport {
    /// Step number after the transition has been applied.
    pub step: u32,
    /// Total number of cells covered by the report.
    pub total_cells: u64,
    /// Number of cells that changed material during the step.
    pub changed_cells: u64,
    /// Number of cells that kept the same material.
    pub stable_cells: u64,
    /// Number of cells considered alive under the caller-supplied alive set.
    pub living_cells: u64,
    /// Number of cells that transitioned from non-alive to alive.
    pub born_cells: u64,
    /// Number of cells that transitioned from alive to non-alive.
    pub died_cells: u64,
    /// Final non-zero per-material populations.
    pub populations: Vec<MaterialPopulation>,
    /// Material-to-material transition counts for changed cells.
    pub transitions: Vec<TransitionCount>,
}
