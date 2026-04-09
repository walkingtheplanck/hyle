//! Tests for the default bounded index resolution on the solver trait.

use std::marker::PhantomData;

use hyle_ca_core::{BoundedTopology, CaSolver, Cell};

struct DummySolver<C: Cell> {
    width: u32,
    height: u32,
    depth: u32,
    topology: BoundedTopology,
    _marker: PhantomData<C>,
}

impl<C: Cell> DummySolver<C> {
    fn new(width: u32, height: u32, depth: u32) -> Self {
        Self {
            width,
            height,
            depth,
            topology: BoundedTopology,
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

    fn get(&self, _x: i32, _y: i32, _z: i32) -> C {
        C::default()
    }

    fn set(&mut self, _x: i32, _y: i32, _z: i32, _cell: C) {}

    fn step(&mut self) {}

    fn step_count(&self) -> u32 {
        0
    }

    fn iter_cells(&self) -> Vec<(u32, u32, u32, C)> {
        Vec::new()
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
