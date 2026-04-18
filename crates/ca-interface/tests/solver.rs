//! Tests for the default bounded index resolution on the solver trait.

use hyle_ca_interface::{
    AttributeAccessError, AttributeDef, AttributeId, AttributeType, AttributeValue, AxisTopology,
    CellId, GridAccessError, GridDims, GridRegion, MaterialDef, MaterialId, NeighborhoodFalloff,
    NeighborhoodId, NeighborhoodRadius, NeighborhoodSet, NeighborhoodShape, NeighborhoodSpec,
    SolverCells, SolverExecution, SolverGrid, SolverMetadata, SolverMetrics, Topology,
    TopologyDescriptor, TransitionCount,
};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
struct BoundedLikeTopology;

impl Topology for BoundedLikeTopology {
    fn descriptor(&self) -> TopologyDescriptor {
        TopologyDescriptor::uniform(AxisTopology::Bounded)
    }

    fn resolve_index(&self, x: i32, y: i32, z: i32, dims: GridDims, guard_idx: usize) -> usize {
        let ux = x as u32;
        let uy = y as u32;
        let uz = z as u32;
        let max_dim = i32::MAX as u32;
        let in_bounds = (dims.width <= max_dim)
            && (dims.height <= max_dim)
            && (dims.depth <= max_dim)
            && (ux < dims.width)
            && (uy < dims.height)
            && (uz < dims.depth);

        if in_bounds {
            (ux as usize)
                + (uy as usize) * (dims.width as usize)
                + (uz as usize) * (dims.width as usize) * (dims.height as usize)
        } else {
            guard_idx
        }
    }
}

struct DummySolver {
    width: u32,
    height: u32,
    depth: u32,
    topology: BoundedLikeTopology,
    cells: Vec<MaterialId>,
    material_defs: Vec<MaterialDef>,
    attribute_defs: Vec<AttributeDef>,
    neighborhood_specs: Vec<NeighborhoodSpec>,
}

impl DummySolver {
    fn new(width: u32, height: u32, depth: u32) -> Self {
        let cell_count = (width as usize)
            .checked_mul(height as usize)
            .and_then(|xy| xy.checked_mul(depth as usize))
            .expect("grid cell count must fit in usize");
        let cells = if cell_count <= 1 << 20 {
            vec![MaterialId::default(); cell_count]
        } else {
            Vec::new()
        };
        Self {
            width,
            height,
            depth,
            topology: BoundedLikeTopology,
            cells,
            material_defs: vec![
                MaterialDef::new(MaterialId::new(0), "dead", Vec::new()),
                MaterialDef::new(MaterialId::new(1), "alive", Vec::new()),
            ],
            attribute_defs: vec![AttributeDef::new(
                AttributeId::new(0),
                "heat",
                AttributeType::U8,
            )],
            neighborhood_specs: vec![NeighborhoodSpec::from_ref(
                hyle_ca_interface::NeighborhoodRef::new(AdjacentNeighborhood::Adjacent),
                NeighborhoodShape::Moore,
                NeighborhoodRadius::new(1),
                NeighborhoodFalloff::Uniform,
            )],
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum AdjacentNeighborhood {
    Adjacent,
}

impl NeighborhoodSet for AdjacentNeighborhood {
    fn variants() -> &'static [Self] {
        &[Self::Adjacent]
    }

    fn label(self) -> &'static str {
        "adjacent"
    }
}

impl SolverExecution for DummySolver {
    type Topology = BoundedLikeTopology;

    fn width(&self) -> u32 {
        self.width
    }

    fn height(&self) -> u32 {
        self.height
    }

    fn depth(&self) -> u32 {
        self.depth
    }

    fn topology(&self) -> &Self::Topology {
        &self.topology
    }

    fn get(&self, x: i32, y: i32, z: i32) -> MaterialId {
        let index = self.resolve_index(x, y, z);
        if index == self.guard_index() {
            MaterialId::default()
        } else {
            self.cells.get(index).copied().unwrap_or_default()
        }
    }

    fn set(&mut self, x: i32, y: i32, z: i32, material: MaterialId) {
        let index = self.resolve_index(x, y, z);
        if index != self.guard_index() {
            if let Some(slot) = self.cells.get_mut(index) {
                *slot = material;
            }
        }
    }

    fn get_attr(
        &self,
        attribute: AttributeId,
        x: i32,
        y: i32,
        z: i32,
    ) -> Result<AttributeValue, AttributeAccessError> {
        let index = self.resolve_index(x, y, z);
        if index == self.guard_index() {
            Err(AttributeAccessError::OutOfBounds { x, y, z })
        } else {
            Err(AttributeAccessError::UnknownAttribute(attribute))
        }
    }

    fn set_attr(
        &mut self,
        attribute: AttributeId,
        x: i32,
        y: i32,
        z: i32,
        _value: AttributeValue,
    ) -> Result<(), AttributeAccessError> {
        let index = self.resolve_index(x, y, z);
        if index == self.guard_index() {
            Err(AttributeAccessError::OutOfBounds { x, y, z })
        } else {
            Err(AttributeAccessError::UnknownAttribute(attribute))
        }
    }

    fn step(&mut self) {}

    fn step_count(&self) -> u32 {
        0
    }
}

impl SolverMetadata for DummySolver {
    fn material_defs(&self) -> &[MaterialDef] {
        &self.material_defs
    }

    fn attribute_defs(&self) -> &[AttributeDef] {
        &self.attribute_defs
    }

    fn neighborhood_specs(&self) -> &[NeighborhoodSpec] {
        &self.neighborhood_specs
    }
}

impl SolverCells for DummySolver {}

impl SolverGrid for DummySolver {}

impl SolverMetrics for DummySolver {
    fn last_changed_cells(&self) -> u64 {
        0
    }

    fn last_transitions(&self) -> &[TransitionCount] {
        &[]
    }
}

struct BrokenRegionSolver {
    inner: DummySolver,
}

impl BrokenRegionSolver {
    fn new(width: u32, height: u32, depth: u32) -> Self {
        Self {
            inner: DummySolver::new(width, height, depth),
        }
    }
}

impl SolverExecution for BrokenRegionSolver {
    type Topology = BoundedLikeTopology;

    fn width(&self) -> u32 {
        self.inner.width()
    }

    fn height(&self) -> u32 {
        self.inner.height()
    }

    fn depth(&self) -> u32 {
        self.inner.depth()
    }

    fn topology(&self) -> &Self::Topology {
        self.inner.topology()
    }

    fn get(&self, x: i32, y: i32, z: i32) -> MaterialId {
        self.inner.get(x, y, z)
    }

    fn set(&mut self, x: i32, y: i32, z: i32, material: MaterialId) {
        self.inner.set(x, y, z, material);
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
        self.inner.step();
    }

    fn step_count(&self) -> u32 {
        self.inner.step_count()
    }
}

impl SolverMetadata for BrokenRegionSolver {
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

impl SolverCells for BrokenRegionSolver {
    fn cell_at(&self, x: i32, y: i32, z: i32) -> Option<CellId> {
        if [x, y, z] == [1, 0, 0] {
            None
        } else {
            self.inner.cell_at(x, y, z)
        }
    }
}

impl SolverGrid for BrokenRegionSolver {}

impl SolverMetrics for BrokenRegionSolver {
    fn last_changed_cells(&self) -> u64 {
        0
    }

    fn last_transitions(&self) -> &[TransitionCount] {
        &[]
    }
}

#[test]
fn default_resolve_index_rejects_negative_and_large_values() {
    let solver = DummySolver::new(4, 5, 6);
    let guard = solver.guard_index();
    assert_eq!(solver.resolve_index(-1, 0, 0), guard);
    assert_eq!(solver.resolve_index(4, 0, 0), guard);
    assert_eq!(solver.resolve_index(0, 5, 0), guard);
    assert_eq!(solver.resolve_index(0, 0, 6), guard);
}

#[test]
fn default_resolve_index_accepts_in_bounds_values() {
    let solver = DummySolver::new(4, 5, 6);
    assert_eq!(solver.resolve_index(3, 4, 5), 119);
}

#[test]
fn default_readback_returns_x_major_snapshot() {
    let mut solver = DummySolver::new(2, 2, 2);
    solver.set(1, 0, 0, MaterialId::new(5));
    solver.set(0, 1, 1, MaterialId::new(9));

    let snapshot = solver.readback();
    assert_eq!(snapshot.dims, solver.dims());
    assert_eq!(
        snapshot.cells,
        vec![
            MaterialId::new(0),
            MaterialId::new(5),
            MaterialId::new(0),
            MaterialId::new(0),
            MaterialId::new(0),
            MaterialId::new(0),
            MaterialId::new(9),
            MaterialId::new(0),
        ]
    );
}

#[test]
fn default_read_and_write_region_follow_x_major_order() {
    let mut solver = DummySolver::new(3, 3, 2);
    let region = GridRegion::new([1, 1, 0], [2, 2, 1]);
    solver
        .write_region(
            region,
            &[
                MaterialId::new(1),
                MaterialId::new(2),
                MaterialId::new(3),
                MaterialId::new(4),
            ],
        )
        .expect("region write should succeed");

    assert_eq!(solver.get(1, 1, 0), MaterialId::new(1));
    assert_eq!(solver.get(2, 1, 0), MaterialId::new(2));
    assert_eq!(solver.get(1, 2, 0), MaterialId::new(3));
    assert_eq!(solver.get(2, 2, 0), MaterialId::new(4));
    assert_eq!(
        solver.read_region(region),
        Ok(vec![
            MaterialId::new(1),
            MaterialId::new(2),
            MaterialId::new(3),
            MaterialId::new(4),
        ])
    );
}

#[test]
fn region_queries_return_typed_errors_for_invalid_regions() {
    let solver = DummySolver::new(3, 3, 1);
    let region = GridRegion::new([2, 2, 0], [2, 1, 1]);
    let error = GridAccessError::RegionOutOfBounds {
        region,
        dims: solver.dims(),
    };

    assert_eq!(solver.cells_in_region(region), Err(error));
    assert_eq!(solver.read_region(region), Err(error));
}

#[test]
fn region_queries_return_typed_errors_for_unresolvable_coordinates() {
    let solver = BrokenRegionSolver::new(2, 1, 1);

    assert_eq!(
        solver.cells_in_region(GridRegion::new([0, 0, 0], [2, 1, 1])),
        Err(GridAccessError::CoordinateUnresolvable { x: 1, y: 0, z: 0 })
    );
}

#[test]
fn grid_writes_return_typed_errors_for_wrong_cell_counts() {
    let mut solver = DummySolver::new(3, 3, 1);
    let region = GridRegion::new([0, 0, 0], [2, 1, 1]);

    assert_eq!(
        solver.write_region(region, &[MaterialId::new(1)]),
        Err(GridAccessError::CellCountMismatch {
            expected: region.cell_count(),
            actual: 1,
        })
    );
    assert_eq!(
        solver.replace_cells(&[MaterialId::new(1)]),
        Err(GridAccessError::CellCountMismatch {
            expected: solver.dims().cell_count(),
            actual: 1,
        })
    );
}

#[test]
fn default_cell_queries_enumerate_regions_and_neighbors() {
    let mut solver = DummySolver::new(3, 3, 1);
    solver.set(1, 1, 0, MaterialId::new(1));
    solver.set(2, 1, 0, MaterialId::new(1));

    let center = solver.cell_at(1, 1, 0).expect("center cell must resolve");
    assert!(solver.contains_cell(center));
    assert!(!solver.contains_cell(CellId::new(99)));
    assert_eq!(solver.cells().len(), 9);
    assert_eq!(
        solver
            .cells_in_region(GridRegion::new([1, 1, 0], [2, 1, 1]))
            .expect("valid region must resolve")
            .len(),
        2
    );

    let neighbors = solver
        .neighbor_materials(center, NeighborhoodId::new(0))
        .expect("known neighborhood must resolve");
    assert!(neighbors
        .iter()
        .any(|(_, material)| *material == MaterialId::new(1)));
}
#[test]
fn metadata_queries_resolve_by_id() {
    let solver = DummySolver::new(2, 2, 2);

    assert_eq!(
        solver
            .material_def(MaterialId::new(1))
            .map(|definition| definition.name),
        Some("alive")
    );
    assert_eq!(
        solver
            .attribute_def(AttributeId::new(0))
            .map(|definition| definition.name),
        Some("heat")
    );
    assert_eq!(
        solver
            .neighborhood_spec(NeighborhoodId::new(0))
            .map(|spec| spec.name()),
        Some("adjacent")
    );
}
