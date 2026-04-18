use hyle_ca_analysis::{analyze_cell, analyze_runtime};
use hyle_ca_interface::{
    Blueprint, MaterialId, MaterialSet, NeighborhoodFalloff, NeighborhoodRadius, NeighborhoodSet,
    NeighborhoodShape, NeighborhoodSpec, RuleSpec, Runtime, RuntimeGrid, RuntimeStepping,
};
use hyle_ca_solver::Solver;

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

fn material_id(material: M) -> MaterialId {
    material
        .id()
        .expect("test material set should be internally consistent")
}

#[test]
fn runtime_analysis_tracks_living_birth_and_death_counts() {
    let spec = Blueprint::builder()
        .materials::<M>()
        .neighborhoods::<N>()
        .neighborhood_specs([NeighborhoodSpec::new(
            N::Adjacent,
            NeighborhoodShape::Moore,
            NeighborhoodRadius::new(1),
            NeighborhoodFalloff::Uniform,
        )
        .expect("test neighborhood set should be internally consistent")])
        .rules([RuleSpec::when(M::Alive).becomes(M::Dead)])
        .build()
        .expect("valid spec");

    let mut runtime = Runtime::new(Solver::from_spec(2, 2, 2, &spec).expect("valid grid"));
    runtime.set(0, 0, 0, material_id(M::Alive));
    runtime.set(1, 1, 1, material_id(M::Alive));
    runtime.step();

    let report = analyze_runtime(&runtime, &[material_id(M::Alive)]);

    assert_eq!(report.step, 1);
    assert_eq!(report.total_cells, 8);
    assert_eq!(report.changed_cells, 2);
    assert_eq!(report.stable_cells, 6);
    assert_eq!(report.living_cells, 0);
    assert_eq!(report.born_cells, 0);
    assert_eq!(report.died_cells, 2);
    assert_eq!(report.populations.len(), 1);
}

#[test]
fn cell_analysis_reports_material_attributes_and_neighborhoods() {
    let spec = Blueprint::builder()
        .materials::<M>()
        .neighborhoods::<N>()
        .neighborhood_specs([NeighborhoodSpec::new(
            N::Adjacent,
            NeighborhoodShape::Moore,
            NeighborhoodRadius::new(1),
            NeighborhoodFalloff::Uniform,
        )
        .expect("test neighborhood set should be internally consistent")])
        .rules([RuleSpec::when(M::Alive).keep()])
        .build()
        .expect("valid spec");

    let mut runtime = Runtime::new(Solver::from_spec(3, 3, 3, &spec).expect("valid grid"));
    runtime.set(1, 1, 1, material_id(M::Alive));
    runtime.set(2, 1, 1, material_id(M::Alive));

    let report = analyze_cell(&runtime, [1, 1, 1]).expect("in-bounds cell");

    assert_eq!(report.material.name, "Alive");
    assert_eq!(report.resolved_position, [1, 1, 1]);
    assert!(report.attributes.is_empty());
    assert_eq!(report.neighborhoods.len(), 1);
    assert_eq!(report.neighborhoods[0].name, "adjacent");
    assert_eq!(report.neighborhoods[0].neighbor_count, 26);
    assert!(report.neighborhoods[0]
        .materials
        .iter()
        .any(|material| material.name == "Alive" && material.count == 1));
}
