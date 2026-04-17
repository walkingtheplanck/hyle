//! Runtime metric capabilities.

use crate::{MaterialId, TransitionCount};

/// Step and population metrics exposed by a live runtime.
pub trait RuntimeMetrics {
    /// Number of cells whose material changed during the latest completed step.
    fn last_changed_cells(&self) -> u64;

    /// Population of one material in the current grid state.
    fn population(&self, material: MaterialId) -> u64;

    /// Full current per-material population table.
    fn populations(&self) -> Vec<u64>;

    /// Material-to-material transition counts from the latest completed step.
    fn last_transitions(&self) -> &[TransitionCount];
}
