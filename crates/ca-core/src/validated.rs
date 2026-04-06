//! Debug-only wrapper that validates all CaSolver contracts at runtime.
//!
//! Wraps any solver and asserts invariants on every call.
//! Use in debug builds to catch contract violations early, with a stack trace
//! at the exact call site.
//!
//! Zero cost in release builds — just use the inner solver directly.
//!
//! ```ignore
//! #[cfg(debug_assertions)]
//! type Solver = ValidatedSolver<u32, CpuSolver<u32>>;
//! #[cfg(not(debug_assertions))]
//! type Solver = CpuSolver<u32>;
//! ```

use std::marker::PhantomData;

use crate::cell::Cell;
use crate::CaSolver;

/// Wrapper that validates CaSolver contracts on every operation.
///
/// Panics immediately when a contract is violated, with a message
/// describing which invariant failed and the arguments that triggered it.
pub struct ValidatedSolver<C: Cell, S: CaSolver<C>> {
    inner: S,
    initial_width: u32,
    initial_height: u32,
    initial_depth: u32,
    _phantom: PhantomData<C>,
}

impl<C: Cell + PartialEq + core::fmt::Debug, S: CaSolver<C>> ValidatedSolver<C, S> {
    /// Wrap a solver with contract validation.
    pub fn new(inner: S) -> Self {
        let w = inner.width();
        let h = inner.height();
        let d = inner.depth();
        ValidatedSolver {
            inner,
            initial_width: w,
            initial_height: h,
            initial_depth: d,
            _phantom: PhantomData,
        }
    }

    /// Access the inner solver directly (e.g. for solver-specific methods).
    pub fn inner(&self) -> &S {
        &self.inner
    }

    /// Mutable access to the inner solver.
    pub fn inner_mut(&mut self) -> &mut S {
        &mut self.inner
    }

    /// Consume the wrapper and return the inner solver.
    pub fn into_inner(self) -> S {
        self.inner
    }

    #[inline]
    fn is_in_bounds(&self, x: i32, y: i32, z: i32) -> bool {
        (x as u32) < self.inner.width()
            && (y as u32) < self.inner.height()
            && (z as u32) < self.inner.depth()
    }
}

impl<C: Cell + PartialEq + core::fmt::Debug, S: CaSolver<C>> CaSolver<C>
    for ValidatedSolver<C, S>
{
    fn width(&self) -> u32 {
        let w = self.inner.width();
        assert!(
            w == self.initial_width,
            "contract violation: width() changed from {} to {}",
            self.initial_width, w
        );
        w
    }

    fn height(&self) -> u32 {
        let h = self.inner.height();
        assert!(
            h == self.initial_height,
            "contract violation: height() changed from {} to {}",
            self.initial_height, h
        );
        h
    }

    fn depth(&self) -> u32 {
        let d = self.inner.depth();
        assert!(
            d == self.initial_depth,
            "contract violation: depth() changed from {} to {}",
            self.initial_depth, d
        );
        d
    }

    fn get(&self, x: i32, y: i32, z: i32) -> C {
        let result = self.inner.get(x, y, z);

        // Out-of-bounds must return C::default()
        if !self.is_in_bounds(x, y, z) {
            assert!(
                result == C::default(),
                "contract violation: get({x},{y},{z}) is out-of-bounds but returned {result:?} instead of {:?}",
                C::default()
            );
        }

        result
    }

    fn set(&mut self, x: i32, y: i32, z: i32, cell: C) {
        self.inner.set(x, y, z, cell);

        // If in-bounds, get must return what we just set
        if self.is_in_bounds(x, y, z) {
            let readback = self.inner.get(x, y, z);
            assert!(
                readback == cell,
                "contract violation: set({x},{y},{z}, {cell:?}) then get returned {readback:?}"
            );
        }
    }

    fn step(&mut self) {
        let before = self.inner.step_count();
        self.inner.step();
        let after = self.inner.step_count();
        assert!(
            after == before + 1,
            "contract violation: step_count was {before}, after step() it is {after} (expected {})",
            before + 1
        );
    }

    fn step_count(&self) -> u32 {
        self.inner.step_count()
    }
}
