//! Runtime stepping capabilities.

/// Step-oriented capabilities exposed by a live runtime.
pub trait RuntimeStepping {
    /// Advance the simulation by one step.
    fn step(&mut self);

    /// Number of completed steps.
    fn step_count(&self) -> u32;
}
