//! Runtime stepping capabilities.

/// Step-oriented capabilities exposed by a live runtime.
pub trait RuntimeStepping {
    /// Advance the simulation by one step.
    ///
    /// This mutates the live runtime state and updates any last-step metrics the
    /// runtime exposes.
    fn step(&mut self);

    /// Number of completed steps.
    ///
    /// The initial state is step `0`; the counter increases after each
    /// successful call to [`RuntimeStepping::step`].
    fn step_count(&self) -> u32;
}
