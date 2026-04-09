//! Tests for the default bounded coordinate resolution on the solver trait.

use std::marker::PhantomData;

use hyle_ca_core::{CaSolver, Cell};

struct DummySolver<C: Cell> {
    width: u32,
    height: u32,
    depth: u32,
    _marker: PhantomData<C>,
}

impl<C: Cell> DummySolver<C> {
    fn new(width: u32, height: u32, depth: u32) -> Self {
        Self {
            width,
            height,
            depth,
            _marker: PhantomData,
        }
    }
}

impl<C: Cell> CaSolver<C> for DummySolver<C> {
    fn width(&self) -> u32 {
        self.width
    }

    fn height(&self) -> u32 {
        self.height
    }

    fn depth(&self) -> u32 {
        self.depth
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
fn default_resolve_coord_rejects_negative_and_large_values() {
    let solver = DummySolver::<u32>::new(4, 5, 6);
    assert_eq!(solver.resolve_coord(-1, 0, 0), None);
    assert_eq!(solver.resolve_coord(4, 0, 0), None);
    assert_eq!(solver.resolve_coord(0, 5, 0), None);
    assert_eq!(solver.resolve_coord(0, 0, 6), None);
}

#[test]
fn default_resolve_coord_accepts_in_bounds_values() {
    let solver = DummySolver::<u32>::new(4, 5, 6);
    assert_eq!(solver.resolve_coord(3, 4, 5), Some((3, 4, 5)));
}
