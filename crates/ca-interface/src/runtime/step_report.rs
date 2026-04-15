//! Low-level runtime step reporting shared across solver implementations.

use crate::MaterialId;

/// Aggregate count for one material transition observed during a step.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TransitionCount {
    /// Source material before the step.
    pub from: MaterialId,
    /// Destination material after the step.
    pub to: MaterialId,
    /// Number of cells that followed this transition.
    pub count: u64,
}

/// Low-level material and transition summary for one completed step.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct StepReport {
    /// Step number after the transition has been applied.
    pub step: u32,
    /// Number of cells whose material changed during the step.
    pub changed_cells: u64,
    /// Final per-material populations indexed by `MaterialId::index()`.
    pub populations: Vec<u64>,
    /// Material-to-material transition counts for changed cells only.
    pub transitions: Vec<TransitionCount>,
}

impl StepReport {
    /// Construct a new step report.
    pub fn new(
        step: u32,
        changed_cells: u64,
        populations: Vec<u64>,
        transitions: Vec<TransitionCount>,
    ) -> Self {
        Self {
            step,
            changed_cells,
            populations,
            transitions,
        }
    }

    /// Total number of logical cells covered by the report.
    pub fn total_cells(&self) -> u64 {
        self.populations.iter().sum()
    }

    /// Population of one material after the step.
    pub fn population(&self, material: MaterialId) -> u64 {
        self.populations
            .get(material.index())
            .copied()
            .unwrap_or_default()
    }
}
