//! Tests for the default bounded index resolution on the solver trait.

use std::marker::PhantomData;

use hyle_ca_core::{BoundedTopology, CaSolver, Cell, GridRegion};

struct DummySolver<C: Cell> {
    width: u32,
    height: u32,
    depth: u32,
    topology: BoundedTopology,
    cells: Vec<C>,
    _marker: PhantomData<C>,
}

impl<C: Cell> DummySolver<C> {
    fn new(width: u32, height: u32, depth: u32) -> Self {
        let cell_count = (width as usize)
            .checked_mul(height as usize)
            .and_then(|xy| xy.checked_mul(depth as usize))
            .expect("grid cell count must fit in usize");
        let cells = if cell_count <= 1 << 20 {
            vec![C::default(); cell_count]
        } else {
            Vec::new()
        };
        Self {
            width,
            height,
            depth,
            topology: BoundedTopology,
            cells,
            _marker: PhantomData,
        }
    }
}

impl<C: Cell> CaSolver<C> for DummySolver<C> {
    type Topology = BoundedTopology;

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

    fn get(&self, x: i32, y: i32, z: i32) -> C {
        let index = self.resolve_index(x, y, z);
        if index == self.guard_index() {
            C::default()
        } else {
            self.cells.get(index).copied().unwrap_or_default()
        }
    }

    fn set(&mut self, x: i32, y: i32, z: i32, cell: C) {
        let index = self.resolve_index(x, y, z);
        if index != self.guard_index() {
            if let Some(slot) = self.cells.get_mut(index) {
                *slot = cell;
            }
        }
    }

    fn step(&mut self) {}

    fn step_count(&self) -> u32 {
        0
    }

    fn iter_cells(&self) -> Vec<(u32, u32, u32, C)> {
        let width = self.width as usize;
        let height = self.height as usize;
        self.cells
            .iter()
            .enumerate()
            .map(move |(index, &cell)| {
                let x = (index % width) as u32;
                let y = ((index / width) % height) as u32;
                let z = (index / (width * height)) as u32;
                (x, y, z, cell)
            })
            .collect()
    }
}

#[test]
fn default_resolve_index_rejects_negative_and_large_values() {
    let solver = DummySolver::<u32>::new(4, 5, 6);
    let guard = solver.guard_index();
    assert_eq!(solver.resolve_index(-1, 0, 0), guard);
    assert_eq!(solver.resolve_index(4, 0, 0), guard);
    assert_eq!(solver.resolve_index(0, 5, 0), guard);
    assert_eq!(solver.resolve_index(0, 0, 6), guard);
}

#[test]
fn default_resolve_index_accepts_in_bounds_values() {
    let solver = DummySolver::<u32>::new(4, 5, 6);
    assert_eq!(solver.resolve_index(3, 4, 5), 119);
}

#[test]
fn default_resolve_index_rejects_oversized_dimensions() {
    let solver = DummySolver::<u32>::new(i32::MAX as u32 + 1, 5, 6);
    let guard = solver.guard_index();
    assert_eq!(solver.resolve_index(0, 0, 0), guard);
    assert_eq!(solver.resolve_index(-1, 0, 0), guard);
}

#[test]
fn default_readback_returns_x_major_snapshot() {
    let mut solver = DummySolver::<u32>::new(2, 2, 2);
    solver.set(1, 0, 0, 5);
    solver.set(0, 1, 1, 9);

    let snapshot = solver.readback();

    assert_eq!(snapshot.dims, solver.dims());
    assert_eq!(snapshot.cells, vec![0, 5, 0, 0, 0, 0, 9, 0]);
}

#[test]
fn default_read_and_write_region_follow_x_major_order() {
    let mut solver = DummySolver::<u32>::new(3, 3, 2);
    let region = GridRegion::new([1, 1, 0], [2, 2, 1]);
    solver.write_region(region, &[1, 2, 3, 4]);

    assert_eq!(solver.get(1, 1, 0), 1);
    assert_eq!(solver.get(2, 1, 0), 2);
    assert_eq!(solver.get(1, 2, 0), 3);
    assert_eq!(solver.get(2, 2, 0), 4);
    assert_eq!(solver.read_region(region), vec![1, 2, 3, 4]);
}

#[test]
fn default_replace_cells_overwrites_the_full_grid() {
    let mut solver = DummySolver::<u32>::new(2, 2, 2);
    solver.replace_cells(&[1, 2, 3, 4, 5, 6, 7, 8]);

    assert_eq!(solver.readback().cells, vec![1, 2, 3, 4, 5, 6, 7, 8]);
}
