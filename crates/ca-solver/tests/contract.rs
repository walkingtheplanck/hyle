//! Contract tests: does the solver API behave as documented?

use hyle_ca_interface::{
    neighbors, Blueprint, CellQueryError, GridRegion, MaterialId, MaterialSet, NeighborhoodFalloff,
    NeighborhoodId, NeighborhoodRadius, NeighborhoodSet, NeighborhoodShape, NeighborhoodSpec,
    RuleSpec, SolverCells, SolverExecution, SolverGrid, TopologyDescriptor,
};
use hyle_ca_solver::{DescriptorTopology, Solver, TorusTopology};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, MaterialSet)]
enum M {
    #[default]
    Dead,
    Alive,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum N {
    Adjacent,
}

impl NeighborhoodSet for N {
    fn variants() -> &'static [Self] {
        &[N::Adjacent]
    }

    fn label(self) -> &'static str {
        "adjacent"
    }
}

#[test]
fn dimensions_match_constructor() {
    let s = Solver::new(8, 16, 4);
    assert_eq!(s.width(), 8);
    assert_eq!(s.height(), 16);
    assert_eq!(s.depth(), 4);
}

#[test]
fn default_materials_are_zero() {
    let s = Solver::new(4, 4, 4);
    assert_eq!(s.get(0, 0, 0), MaterialId::default());
    assert_eq!(s.get(3, 3, 3), MaterialId::default());
}

#[test]
fn set_then_get_roundtrip() {
    let mut s = Solver::new(4, 4, 4);
    s.set(1, 2, 3, MaterialId::new(42));
    assert_eq!(s.get(1, 2, 3), MaterialId::new(42));
}

#[test]
fn readback_returns_contiguous_snapshot() {
    let mut s = Solver::new(2, 2, 2);
    s.set(1, 0, 0, MaterialId::new(5));
    s.set(0, 1, 1, MaterialId::new(9));

    assert_eq!(
        s.readback().cells,
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
fn write_region_updates_subvolume_in_x_major_order() {
    let mut s = Solver::new(3, 3, 2);
    let region = GridRegion::new([1, 1, 0], [2, 2, 1]);
    s.write_region(
        region,
        &[
            MaterialId::new(1),
            MaterialId::new(2),
            MaterialId::new(3),
            MaterialId::new(4),
        ],
    )
    .expect("region write should succeed");
    assert_eq!(
        s.read_region(region).expect("region read should succeed")[..],
        [
            MaterialId::new(1),
            MaterialId::new(2),
            MaterialId::new(3),
            MaterialId::new(4)
        ]
    );
}

#[test]
fn torus_topology_wraps_coordinates() {
    let mut s = Solver::with_topology(4, 4, 4, TorusTopology);
    s.set(-1, 0, 0, MaterialId::new(11));
    assert_eq!(s.get(3, 0, 0), MaterialId::new(11));
    assert_eq!(s.get(7, 0, 0), MaterialId::new(11));
}

#[test]
fn from_spec_uses_descriptor_topology() {
    let spec = Blueprint::builder()
        .materials::<M>()
        .neighborhoods::<N>()
        .topology(TopologyDescriptor::wrap())
        .neighborhood_specs([NeighborhoodSpec::new(
            N::Adjacent,
            NeighborhoodShape::Moore,
            NeighborhoodRadius::new(1),
            NeighborhoodFalloff::Uniform,
        )])
        .rules([RuleSpec::when(M::Dead)
            .require(neighbors(M::Alive).any())
            .becomes(M::Alive)])
        .build()
        .expect("valid spec");

    let solver = Solver::from_spec(4, 4, 4, &spec);
    assert_eq!(
        solver.topology(),
        &DescriptorTopology::new(TopologyDescriptor::wrap())
    );
}

#[test]
fn neighborhood_queries_without_schema_report_missing_schema() {
    let solver = Solver::new(2, 2, 2);
    let cell = solver.cell_at(0, 0, 0).expect("origin cell should exist");

    assert_eq!(
        solver.neighbors(cell, NeighborhoodId::new(0)),
        Err(CellQueryError::SchemaUnavailable)
    );
}

#[test]
#[should_panic(expected = "width must be <= i32::MAX")]
fn constructor_rejects_dimensions_larger_than_i32() {
    let _ = Solver::new(i32::MAX as u32 + 1, 1, 1);
}
