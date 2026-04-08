//! Debug-only wrapper that validates all CaSolver contracts at runtime.
//!
//! Wraps any solver and asserts invariants on every call.
//! Use in debug builds to catch contract violations early, with a stack trace
//! at the exact call site.
//!
//! Zero cost in release builds - just use the inner solver directly.
//!
//! ```ignore
//! #[cfg(debug_assertions)]
//! type Solver = ValidatedSolver<u32, CpuSolver<u32>>;
//! #[cfg(not(debug_assertions))]
//! type Solver = CpuSolver<u32>;
//! ```

use std::marker::PhantomData;

use crate::{CaSolver, Cell};

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
}

impl<C: Cell + PartialEq + core::fmt::Debug, S: CaSolver<C>> CaSolver<C> for ValidatedSolver<C, S> {
    fn width(&self) -> u32 {
        let w = self.inner.width();
        assert!(
            w == self.initial_width,
            "contract violation: width() changed from {} to {}",
            self.initial_width,
            w
        );
        w
    }

    fn height(&self) -> u32 {
        let h = self.inner.height();
        assert!(
            h == self.initial_height,
            "contract violation: height() changed from {} to {}",
            self.initial_height,
            h
        );
        h
    }

    fn depth(&self) -> u32 {
        let d = self.inner.depth();
        assert!(
            d == self.initial_depth,
            "contract violation: depth() changed from {} to {}",
            self.initial_depth,
            d
        );
        d
    }

    fn resolve_coord(&self, x: i32, y: i32, z: i32) -> Option<(u32, u32, u32)> {
        let resolved = self.inner.resolve_coord(x, y, z);

        if let Some((ix, iy, iz)) = resolved {
            assert!(
                ix < self.inner.width() && iy < self.inner.height() && iz < self.inner.depth(),
                "contract violation: resolve_coord({x},{y},{z}) returned out-of-bounds coordinate ({ix},{iy},{iz})"
            );

            let canonical = self.inner.resolve_coord(ix as i32, iy as i32, iz as i32);
            assert!(
                canonical == Some((ix, iy, iz)),
                "contract violation: resolve_coord({x},{y},{z}) returned ({ix},{iy},{iz}), but resolving that coordinate again produced {canonical:?}"
            );
        }

        resolved
    }

    fn get(&self, x: i32, y: i32, z: i32) -> C {
        let result = self.inner.get(x, y, z);
        match self.inner.resolve_coord(x, y, z) {
            Some((ix, iy, iz)) => {
                let resolved = self.inner.get(ix as i32, iy as i32, iz as i32);
                assert!(
                    result == resolved,
                    "contract violation: get({x},{y},{z}) returned {result:?}, but its resolved coordinate ({ix},{iy},{iz}) returned {resolved:?}"
                );
            }
            None => {
                assert!(
                    result == C::default(),
                    "contract violation: get({x},{y},{z}) resolved to None but returned {result:?} instead of {:?}",
                    C::default()
                );
            }
        }

        result
    }

    fn set(&mut self, x: i32, y: i32, z: i32, cell: C) {
        let resolved = self.inner.resolve_coord(x, y, z);
        self.inner.set(x, y, z, cell);

        if let Some((ix, iy, iz)) = resolved {
            let readback = self.inner.get(ix as i32, iy as i32, iz as i32);
            assert!(
                readback == cell,
                "contract violation: set({x},{y},{z}, {cell:?}) resolved to ({ix},{iy},{iz}) but get returned {readback:?}"
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

    fn iter_cells(&self) -> Vec<(u32, u32, u32, C)> {
        self.inner.iter_cells()
    }
}
