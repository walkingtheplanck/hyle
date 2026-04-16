//! Runtime transition metrics exposed after each completed step.

use crate::MaterialId;

/// Aggregate count for one material transition observed during the latest step.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TransitionCount {
    /// Source material before the step.
    pub from: MaterialId,
    /// Destination material after the step.
    pub to: MaterialId,
    /// Number of cells that followed this transition.
    pub count: u64,
}
