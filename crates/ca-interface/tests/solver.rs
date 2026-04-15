//! Tests for the default bounded index resolution on the solver trait.

use hyle_ca_interface::{
    AttributeAccessError, AttributeId, AttributeValue, AxisTopology, CaSolver, GridDims,
    GridRegion, MaterialId, StepReport, Topology, TopologyDescriptor,
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
        }
    }
}

impl CaSolver for DummySolver {
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

    fn step_report(&mut self) -> StepReport {
        let mut populations = Vec::new();
        for material in &self.cells {
            if populations.len() <= material.index() {
                populations.resize(material.index() + 1, 0);
            }
            populations[material.index()] += 1;
        }

        StepReport::new(1, 0, populations, Vec::new())
    }

    fn step_count(&self) -> u32 {
        0
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
    solver.write_region(
        region,
        &[
            MaterialId::new(1),
            MaterialId::new(2),
            MaterialId::new(3),
            MaterialId::new(4),
        ],
    );

    assert_eq!(solver.get(1, 1, 0), MaterialId::new(1));
    assert_eq!(solver.get(2, 1, 0), MaterialId::new(2));
    assert_eq!(solver.get(1, 2, 0), MaterialId::new(3));
    assert_eq!(solver.get(2, 2, 0), MaterialId::new(4));
    assert_eq!(
        solver.read_region(region),
        vec![
            MaterialId::new(1),
            MaterialId::new(2),
            MaterialId::new(3),
            MaterialId::new(4),
        ]
    );
}
