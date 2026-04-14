//! Debug-only wrapper that validates all `CaSolver` contracts at runtime.

use std::marker::PhantomData;

use crate::{CaSolver, Cell};

/// Wrapper that validates `CaSolver` contracts on every operation.
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

    /// Access the inner solver directly.
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
    type Topology = S::Topology;

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

    fn topology(&self) -> &Self::Topology {
        self.inner.topology()
    }

    fn seed(&self) -> u64 {
        self.inner.seed()
    }

    fn guard_index(&self) -> usize {
        self.inner.guard_index()
    }

    fn resolve_index(&self, x: i32, y: i32, z: i32) -> usize {
        let resolved = self.inner.resolve_index(x, y, z);
        let guard = self.inner.guard_index();

        assert!(
            resolved <= guard,
            "contract violation: resolve_index({x},{y},{z}) returned {resolved}, which is larger than guard index {guard}"
        );

        if resolved != guard {
            let (ix, iy, iz) = decode_index(resolved, self.inner.width(), self.inner.height());
            let canonical = self.inner.resolve_index(ix as i32, iy as i32, iz as i32);
            assert!(
                canonical == resolved,
                "contract violation: resolve_index({x},{y},{z}) returned {resolved}, but resolving its decoded coordinate ({ix},{iy},{iz}) produced {canonical}"
            );
        }

        resolved
    }

    fn get(&self, x: i32, y: i32, z: i32) -> C {
        let result = self.inner.get(x, y, z);
        let resolved = self.inner.resolve_index(x, y, z);
        let guard = self.inner.guard_index();

        if resolved == guard {
            assert!(
                result == C::default(),
                "contract violation: get({x},{y},{z}) resolved to the guard index {guard} but returned {result:?} instead of {:?}",
                C::default()
            );
        } else {
            let (ix, iy, iz) = decode_index(resolved, self.inner.width(), self.inner.height());
            let canonical = self.inner.get(ix as i32, iy as i32, iz as i32);
            assert!(
                result == canonical,
                "contract violation: get({x},{y},{z}) returned {result:?}, but its resolved cell ({ix},{iy},{iz}) returned {canonical:?}"
            );
        }

        result
    }

    fn set(&mut self, x: i32, y: i32, z: i32, cell: C) {
        let resolved = self.inner.resolve_index(x, y, z);
        let guard = self.inner.guard_index();
        self.inner.set(x, y, z, cell);

        if resolved != guard {
            let (ix, iy, iz) = decode_index(resolved, self.inner.width(), self.inner.height());
            let readback = self.inner.get(ix as i32, iy as i32, iz as i32);
            assert!(
                readback == cell,
                "contract violation: set({x},{y},{z}, {cell:?}) resolved to index {resolved} / ({ix},{iy},{iz}) but get returned {readback:?}"
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

    fn readback(&self) -> crate::GridSnapshot<C> {
        self.inner.readback()
    }

    fn read_region(&self, region: crate::GridRegion) -> Vec<C> {
        self.inner.read_region(region)
    }

    fn write_region(&mut self, region: crate::GridRegion, cells: &[C]) {
        self.inner.write_region(region, cells);
    }

    fn iter_cells(&self) -> Vec<(u32, u32, u32, C)> {
        self.inner.iter_cells()
    }
}

fn decode_index(index: usize, width: u32, height: u32) -> (u32, u32, u32) {
    let width = width as usize;
    let height = height as usize;
    let x = index % width;
    let y = (index / width) % height;
    let z = index / (width * height);
    (x as u32, y as u32, z as u32)
}
