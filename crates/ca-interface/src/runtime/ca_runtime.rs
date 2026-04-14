//! Object-safe runtime interface for consumers that only need to drive a simulation.

use crate::{Cell, GridSnapshot};

use super::solver::CaSolver;

/// A minimal object-safe simulation runtime.
///
/// This trait exists for consumers such as viewers and tools that need to
/// drive a running simulation without depending on a concrete solver type or
/// the full generic [`CaSolver`] interface.
pub trait CaRuntime<C: Cell>: Send {
    /// Advance the simulation by one step.
    fn step(&mut self);

    /// Set a cell value at the given coordinate.
    fn set(&mut self, x: i32, y: i32, z: i32, cell: C);

    /// Read the full current state back to the host.
    fn readback(&self) -> GridSnapshot<C>;

    /// Number of completed steps.
    fn step_count(&self) -> u32;
}

impl<C, T> CaRuntime<C> for T
where
    C: Cell,
    T: CaSolver<C> + Send,
{
    fn step(&mut self) {
        CaSolver::step(self);
    }

    fn set(&mut self, x: i32, y: i32, z: i32, cell: C) {
        CaSolver::set(self, x, y, z, cell);
    }

    fn readback(&self) -> GridSnapshot<C> {
        CaSolver::readback(self)
    }

    fn step_count(&self) -> u32 {
        CaSolver::step_count(self)
    }
}
