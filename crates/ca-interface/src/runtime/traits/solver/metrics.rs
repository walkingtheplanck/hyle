//! Solver step and population metrics.

use crate::{MaterialId, TransitionCount};

use super::SolverGrid;

/// Step and population metrics derived from solver state.
pub trait SolverMetrics: SolverGrid {
    /// Number of cells whose material changed during the latest completed step.
    fn last_changed_cells(&self) -> u64;

    /// Population of one material in the current grid state.
    fn population(&self, material: MaterialId) -> u64 {
        self.populations()
            .get(material.index())
            .copied()
            .unwrap_or_default()
    }

    /// Full current per-material population table.
    fn populations(&self) -> Vec<u64> {
        let mut populations = Vec::new();
        for (_, _, _, material) in self.iter_cells() {
            if populations.len() <= material.index() {
                populations.resize(material.index() + 1, 0);
            }
            populations[material.index()] += 1;
        }
        populations
    }

    /// Material-to-material transition counts from the latest completed step.
    fn last_transitions(&self) -> &[TransitionCount];
}
