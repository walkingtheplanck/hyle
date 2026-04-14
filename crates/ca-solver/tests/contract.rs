//! Contract tests: does the solver API behave as documented?

use hyle_ca_interface::{
    neighbors, CaSolver, Cell, CellModel, CellSchema, GridRegion, Hyle, TopologyDescriptor,
};
use hyle_ca_solver::{DescriptorTopology, Solver, TorusTopology};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
struct TestCell(u32);

impl Cell for TestCell {
    fn rule_id(&self) -> u8 {
        self.0 as u8
    }

    fn is_alive(&self) -> bool {
        self.0 != 0
    }
}

impl CellModel for TestCell {
    fn schema() -> CellSchema {
        CellSchema::opaque("TestCell")
    }
}

#[test]
fn dimensions_match_constructor() {
    let s = Solver::<TestCell>::new(8, 16, 4);
    assert_eq!(s.width(), 8);
    assert_eq!(s.height(), 16);
    assert_eq!(s.depth(), 4);
}

#[test]
fn default_cells_are_zero() {
    let s = Solver::<TestCell>::new(4, 4, 4);
    assert_eq!(s.get(0, 0, 0), TestCell(0));
    assert_eq!(s.get(3, 3, 3), TestCell(0));
    assert_eq!(s.get(2, 1, 3), TestCell(0));
}

#[test]
fn set_then_get_roundtrip() {
    let mut s = Solver::<TestCell>::new(4, 4, 4);
    s.set(1, 2, 3, TestCell(42));
    assert_eq!(s.get(1, 2, 3), TestCell(42));
}

#[test]
fn set_does_not_affect_other_cells() {
    let mut s = Solver::<TestCell>::new(4, 4, 4);
    s.set(1, 1, 1, TestCell(99));
    assert_eq!(s.get(0, 0, 0), TestCell(0));
    assert_eq!(s.get(1, 1, 0), TestCell(0));
    assert_eq!(s.get(2, 2, 2), TestCell(0));
}

#[test]
fn out_of_bounds_get_returns_default() {
    let s = Solver::<TestCell>::new(4, 4, 4);
    assert_eq!(s.get(-1, 0, 0), TestCell(0));
    assert_eq!(s.get(4, 0, 0), TestCell(0));
    assert_eq!(s.get(0, -1, 0), TestCell(0));
    assert_eq!(s.get(0, 0, 100), TestCell(0));
}

#[test]
fn out_of_bounds_set_is_noop() {
    let mut s = Solver::<TestCell>::new(4, 4, 4);
    s.set(-1, 0, 0, TestCell(99)); // should not panic
    s.set(100, 0, 0, TestCell(99));
    // Grid unchanged
    assert_eq!(s.get(0, 0, 0), TestCell(0));
}

#[test]
fn step_count_starts_at_zero() {
    let s = Solver::<TestCell>::new(4, 4, 4);
    assert_eq!(s.step_count(), 0);
}

#[test]
fn step_increments_count() {
    let mut s = Solver::<TestCell>::new(4, 4, 4);
    s.step();
    assert_eq!(s.step_count(), 1);
    s.step();
    assert_eq!(s.step_count(), 2);
}

#[test]
fn iter_cells_returns_all_cells() {
    let s = Solver::<TestCell>::new(3, 4, 5);
    let cells = s.iter_cells();
    assert_eq!(cells.len(), 3 * 4 * 5);
}

#[test]
fn iter_cells_reflects_set() {
    let mut s = Solver::<TestCell>::new(4, 4, 4);
    s.set(1, 2, 3, TestCell(7));
    let alive: Vec<_> = s
        .iter_cells()
        .into_iter()
        .filter(|(_, _, _, c)| *c != TestCell(0))
        .collect();
    assert_eq!(alive.len(), 1);
    assert_eq!(alive[0], (1, 2, 3, TestCell(7)));
}

#[test]
fn readback_returns_contiguous_snapshot() {
    let mut s = Solver::<TestCell>::new(2, 2, 2);
    s.set(1, 0, 0, TestCell(5));
    s.set(0, 1, 1, TestCell(9));

    let snapshot = s.readback();

    assert_eq!(snapshot.dims, s.dims());
    assert_eq!(
        snapshot.cells,
        vec![
            TestCell(0),
            TestCell(5),
            TestCell(0),
            TestCell(0),
            TestCell(0),
            TestCell(0),
            TestCell(9),
            TestCell(0),
        ]
    );
}

#[test]
fn write_region_updates_subvolume_in_x_major_order() {
    let mut s = Solver::<TestCell>::new(3, 3, 2);
    let region = GridRegion::new([1, 1, 0], [2, 2, 1]);
    s.write_region(
        region,
        &[TestCell(1), TestCell(2), TestCell(3), TestCell(4)],
    );

    assert_eq!(s.get(1, 1, 0), TestCell(1));
    assert_eq!(s.get(2, 1, 0), TestCell(2));
    assert_eq!(s.get(1, 2, 0), TestCell(3));
    assert_eq!(s.get(2, 2, 0), TestCell(4));
    assert_eq!(
        s.read_region(region),
        vec![TestCell(1), TestCell(2), TestCell(3), TestCell(4)]
    );
}

#[test]
fn replace_cells_overwrites_the_full_grid() {
    let mut s = Solver::<TestCell>::new(2, 2, 2);
    s.replace_cells(&[
        TestCell(1),
        TestCell(2),
        TestCell(3),
        TestCell(4),
        TestCell(5),
        TestCell(6),
        TestCell(7),
        TestCell(8),
    ]);

    assert_eq!(
        s.readback().cells,
        vec![
            TestCell(1),
            TestCell(2),
            TestCell(3),
            TestCell(4),
            TestCell(5),
            TestCell(6),
            TestCell(7),
            TestCell(8),
        ]
    );
}

#[test]
fn step_without_rules_preserves_state() {
    let mut s = Solver::<TestCell>::new(4, 4, 4);
    s.set(1, 1, 1, TestCell(5));
    s.step(); // no rules registered
    assert_eq!(s.get(1, 1, 1), TestCell(5)); // cell unchanged
}

#[test]
fn torus_topology_is_reported() {
    let s = Solver::<TestCell>::with_topology(4, 4, 4, TorusTopology);
    assert_eq!(s.topology(), &TorusTopology);
}

#[test]
fn bounded_resolve_index_maps_out_of_bounds_to_guard() {
    let s = Solver::<TestCell>::new(4, 4, 4);
    assert_eq!(s.resolve_index(-1, 0, 0), s.guard_index());
    assert_eq!(s.resolve_index(3, 0, 0), 3);
}

#[test]
fn torus_resolve_index_wraps_coordinates() {
    let s = Solver::<TestCell>::with_topology(4, 4, 4, TorusTopology);
    assert_eq!(s.resolve_index(-1, 0, 0), 3);
    assert_eq!(s.resolve_index(4, 0, 0), 0);
}

#[test]
fn torus_get_wraps_coordinates() {
    let mut s = Solver::<TestCell>::with_topology(4, 4, 4, TorusTopology);
    s.set(3, 0, 0, TestCell(9));
    assert_eq!(s.get(-1, 0, 0), TestCell(9));
    assert_eq!(s.get(7, 0, 0), TestCell(9));
}

#[test]
fn torus_set_wraps_coordinates() {
    let mut s = Solver::<TestCell>::with_topology(4, 4, 4, TorusTopology);
    s.set(-1, 0, 0, TestCell(11));
    assert_eq!(s.get(3, 0, 0), TestCell(11));
}

#[test]
fn from_spec_uses_descriptor_topology() {
    let spec = Hyle::builder()
        .cells::<TestCell>()
        .topology(TopologyDescriptor::wrap())
        .rules(|rules| {
            rules
                .when(TestCell(0))
                .require(neighbors(TestCell(1)).any())
                .becomes(TestCell(1));
        })
        .build()
        .expect("valid spec");

    let solver = Solver::from_spec(4, 4, 4, &spec);

    assert_eq!(
        solver.topology(),
        &DescriptorTopology::new(TopologyDescriptor::wrap())
    );
}

#[test]
#[should_panic(expected = "width must be <= i32::MAX")]
fn constructor_rejects_dimensions_larger_than_i32() {
    let _ = Solver::<TestCell>::new(i32::MAX as u32 + 1, 1, 1);
}
