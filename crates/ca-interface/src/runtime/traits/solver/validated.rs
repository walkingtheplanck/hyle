//! Debug-only wrapper that validates all `CaSolver` contracts at runtime.

use crate::{
    AttributeAccessError, AttributeDef, AttributeId, AttributeValue, CaSolver, GridDims,
    MaterialDef, MaterialId, NeighborhoodSpec, TransitionCount,
};

use super::{
    SolverAttributes, SolverCells, SolverExecution, SolverGrid, SolverMetadata, SolverMetrics,
};

/// Wrapper that validates `CaSolver` contracts on every operation.
///
/// Panics immediately when a contract is violated, with a message describing
/// which invariant failed and the arguments that triggered it.
pub struct ValidatedSolver<S: CaSolver> {
    inner: S,
    initial_width: u32,
    initial_height: u32,
    initial_depth: u32,
}

impl<S: CaSolver> ValidatedSolver<S> {
    /// Wrap a solver with contract validation.
    pub fn new(inner: S) -> Self {
        let w = inner.width();
        let h = inner.height();
        let d = inner.depth();
        Self {
            inner,
            initial_width: w,
            initial_height: h,
            initial_depth: d,
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

impl<S> SolverExecution for ValidatedSolver<S>
where
    S: CaSolver,
{
    type Topology = S::Topology;

    fn dims(&self) -> GridDims {
        GridDims::from_validated(self.width(), self.height(), self.depth(), self.inner.cell_count())
    }

    fn width(&self) -> u32 {
        let w = self.inner.width();
        assert_eq!(
            w, self.initial_width,
            "contract violation: width() changed from {} to {}",
            self.initial_width, w
        );
        w
    }

    fn height(&self) -> u32 {
        let h = self.inner.height();
        assert_eq!(
            h, self.initial_height,
            "contract violation: height() changed from {} to {}",
            self.initial_height, h
        );
        h
    }

    fn depth(&self) -> u32 {
        let d = self.inner.depth();
        assert_eq!(
            d, self.initial_depth,
            "contract violation: depth() changed from {} to {}",
            self.initial_depth, d
        );
        d
    }

    fn topology(&self) -> &<Self as SolverExecution>::Topology {
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
            assert_eq!(
                canonical, resolved,
                "contract violation: resolve_index({x},{y},{z}) returned {resolved}, but resolving its decoded coordinate ({ix},{iy},{iz}) produced {canonical}"
            );
        }

        resolved
    }

    fn get(&self, x: i32, y: i32, z: i32) -> MaterialId {
        let result = self.inner.get(x, y, z);
        let resolved = self.inner.resolve_index(x, y, z);
        let guard = self.inner.guard_index();

        if resolved == guard {
            assert_eq!(
                result,
                MaterialId::default(),
                "contract violation: get({x},{y},{z}) resolved to the guard index {guard} but returned {result:?}"
            );
        } else {
            let (ix, iy, iz) = decode_index(resolved, self.inner.width(), self.inner.height());
            let canonical = self.inner.get(ix as i32, iy as i32, iz as i32);
            assert_eq!(
                result, canonical,
                "contract violation: get({x},{y},{z}) returned {result:?}, but its resolved cell ({ix},{iy},{iz}) returned {canonical:?}"
            );
        }

        result
    }

    fn set(&mut self, x: i32, y: i32, z: i32, material: MaterialId) {
        let resolved = self.inner.resolve_index(x, y, z);
        let guard = self.inner.guard_index();
        self.inner.set(x, y, z, material);

        if resolved != guard {
            let (ix, iy, iz) = decode_index(resolved, self.inner.width(), self.inner.height());
            let readback = self.inner.get(ix as i32, iy as i32, iz as i32);
            assert_eq!(
                readback, material,
                "contract violation: set({x},{y},{z}, {material:?}) resolved to index {resolved} / ({ix},{iy},{iz}) but get returned {readback:?}"
            );
        }
    }

    fn get_attr(
        &self,
        attribute: AttributeId,
        x: i32,
        y: i32,
        z: i32,
    ) -> Result<AttributeValue, AttributeAccessError> {
        self.inner.get_attr(attribute, x, y, z)
    }

    fn set_attr(
        &mut self,
        attribute: AttributeId,
        x: i32,
        y: i32,
        z: i32,
        value: AttributeValue,
    ) -> Result<(), AttributeAccessError> {
        self.inner.set_attr(attribute, x, y, z, value)
    }

    fn step(&mut self) {
        let before = self.inner.step_count();
        self.inner.step();
        let after = self.inner.step_count();
        assert_eq!(
            after,
            before + 1,
            "contract violation: step_count was {before}, after step() it is {after}"
        );
    }

    fn step_count(&self) -> u32 {
        self.inner.step_count()
    }
}

impl<S> SolverMetadata for ValidatedSolver<S>
where
    S: CaSolver,
{
    fn material_defs(&self) -> &[MaterialDef] {
        self.inner.material_defs()
    }

    fn attribute_defs(&self) -> &[AttributeDef] {
        self.inner.attribute_defs()
    }

    fn neighborhood_specs(&self) -> &[NeighborhoodSpec] {
        self.inner.neighborhood_specs()
    }
}

impl<S> SolverCells for ValidatedSolver<S> where S: CaSolver {}

impl<S> SolverAttributes for ValidatedSolver<S> where S: CaSolver {}

impl<S> SolverGrid for ValidatedSolver<S> where S: CaSolver {}

impl<S> SolverMetrics for ValidatedSolver<S>
where
    S: CaSolver,
{
    fn last_changed_cells(&self) -> u64 {
        self.inner.last_changed_cells()
    }

    fn last_transitions(&self) -> &[TransitionCount] {
        self.inner.last_transitions()
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
