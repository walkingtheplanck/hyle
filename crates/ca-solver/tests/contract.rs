//! Contract tests: does the solver API behave as documented?

use hyle_ca_core::{CaSolver, GridRegion, TorusTopology};
use hyle_ca_solver::Solver;

#[test]
fn dimensions_match_constructor() {
    let s = Solver::<u32>::new(8, 16, 4);
    assert_eq!(s.width(), 8);
    assert_eq!(s.height(), 16);
    assert_eq!(s.depth(), 4);
}

#[test]
fn default_cells_are_zero() {
    let s = Solver::<u32>::new(4, 4, 4);
    assert_eq!(s.get(0, 0, 0), 0);
    assert_eq!(s.get(3, 3, 3), 0);
    assert_eq!(s.get(2, 1, 3), 0);
}

#[test]
fn set_then_get_roundtrip() {
    let mut s = Solver::<u32>::new(4, 4, 4);
    s.set(1, 2, 3, 42);
    assert_eq!(s.get(1, 2, 3), 42);
}

#[test]
fn set_does_not_affect_other_cells() {
    let mut s = Solver::<u32>::new(4, 4, 4);
    s.set(1, 1, 1, 99);
    assert_eq!(s.get(0, 0, 0), 0);
    assert_eq!(s.get(1, 1, 0), 0);
    assert_eq!(s.get(2, 2, 2), 0);
}

#[test]
fn out_of_bounds_get_returns_default() {
    let s = Solver::<u32>::new(4, 4, 4);
    assert_eq!(s.get(-1, 0, 0), 0);
    assert_eq!(s.get(4, 0, 0), 0);
    assert_eq!(s.get(0, -1, 0), 0);
    assert_eq!(s.get(0, 0, 100), 0);
}

#[test]
fn out_of_bounds_set_is_noop() {
    let mut s = Solver::<u32>::new(4, 4, 4);
    s.set(-1, 0, 0, 99); // should not panic
    s.set(100, 0, 0, 99);
    // Grid unchanged
    assert_eq!(s.get(0, 0, 0), 0);
}

#[test]
fn step_count_starts_at_zero() {
    let s = Solver::<u32>::new(4, 4, 4);
    assert_eq!(s.step_count(), 0);
}

#[test]
fn step_increments_count() {
    let mut s = Solver::<u32>::new(4, 4, 4);
    s.step();
    assert_eq!(s.step_count(), 1);
    s.step();
    assert_eq!(s.step_count(), 2);
}

#[test]
fn iter_cells_returns_all_cells() {
    let s = Solver::<u32>::new(3, 4, 5);
    let cells = s.iter_cells();
    assert_eq!(cells.len(), 3 * 4 * 5);
}

#[test]
fn iter_cells_reflects_set() {
    let mut s = Solver::<u32>::new(4, 4, 4);
    s.set(1, 2, 3, 7);
    let alive: Vec<_> = s
        .iter_cells()
        .into_iter()
        .filter(|(_, _, _, c)| *c != 0)
        .collect();
    assert_eq!(alive.len(), 1);
    assert_eq!(alive[0], (1, 2, 3, 7));
}

#[test]
fn readback_returns_contiguous_snapshot() {
    let mut s = Solver::<u32>::new(2, 2, 2);
    s.set(1, 0, 0, 5);
    s.set(0, 1, 1, 9);

    let snapshot = s.readback();

    assert_eq!(snapshot.dims, s.dims());
    assert_eq!(snapshot.cells, vec![0, 5, 0, 0, 0, 0, 9, 0]);
}

#[test]
fn write_region_updates_subvolume_in_x_major_order() {
    let mut s = Solver::<u32>::new(3, 3, 2);
    let region = GridRegion::new([1, 1, 0], [2, 2, 1]);
    s.write_region(region, &[1, 2, 3, 4]);

    assert_eq!(s.get(1, 1, 0), 1);
    assert_eq!(s.get(2, 1, 0), 2);
    assert_eq!(s.get(1, 2, 0), 3);
    assert_eq!(s.get(2, 2, 0), 4);
    assert_eq!(s.read_region(region), vec![1, 2, 3, 4]);
}

#[test]
fn replace_cells_overwrites_the_full_grid() {
    let mut s = Solver::<u32>::new(2, 2, 2);
    s.replace_cells(&[1, 2, 3, 4, 5, 6, 7, 8]);

    assert_eq!(s.readback().cells, vec![1, 2, 3, 4, 5, 6, 7, 8]);
}

#[test]
fn step_without_rules_preserves_state() {
    let mut s = Solver::<u32>::new(4, 4, 4);
    s.set(1, 1, 1, 5);
    s.step(); // no rules registered
    assert_eq!(s.get(1, 1, 1), 5); // cell unchanged
}

#[test]
fn torus_topology_is_reported() {
    let s = Solver::<u32>::with_topology(4, 4, 4, TorusTopology);
    assert_eq!(s.topology(), &TorusTopology);
}

#[test]
fn bounded_resolve_index_maps_out_of_bounds_to_guard() {
    let s = Solver::<u32>::new(4, 4, 4);
    assert_eq!(s.resolve_index(-1, 0, 0), s.guard_index());
    assert_eq!(s.resolve_index(3, 0, 0), 3);
}

#[test]
fn torus_resolve_index_wraps_coordinates() {
    let s = Solver::<u32>::with_topology(4, 4, 4, TorusTopology);
    assert_eq!(s.resolve_index(-1, 0, 0), 3);
    assert_eq!(s.resolve_index(4, 0, 0), 0);
}

#[test]
fn torus_get_wraps_coordinates() {
    let mut s = Solver::<u32>::with_topology(4, 4, 4, TorusTopology);
    s.set(3, 0, 0, 9);
    assert_eq!(s.get(-1, 0, 0), 9);
    assert_eq!(s.get(7, 0, 0), 9);
}

#[test]
fn torus_set_wraps_coordinates() {
    let mut s = Solver::<u32>::with_topology(4, 4, 4, TorusTopology);
    s.set(-1, 0, 0, 11);
    assert_eq!(s.get(3, 0, 0), 11);
}

#[test]
#[should_panic(expected = "width must be <= i32::MAX")]
fn constructor_rejects_dimensions_larger_than_i32() {
    let _ = Solver::<u32>::new(i32::MAX as u32 + 1, 1, 1);
}
