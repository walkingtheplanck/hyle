//! Rule application tests using declarative blueprint specs.

use hyle_ca_interface::semantics::{cell_rng, interpret_blueprint};
use hyle_ca_interface::{
    neighbors, rng, BlueprintSpec, CaSolver, CellModel, CellSchema, Instance, NeighborhoodFalloff,
    NeighborhoodShape, NeighborhoodSpec, StateDef, TopologyDescriptor, Weight,
};
use hyle_ca_solver::Solver;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
enum LifeCell {
    #[default]
    Dead,
    Alive,
}

const LIFE_CELL_STATES: [StateDef; 2] = [StateDef::new("Dead"), StateDef::new("Alive")];

impl CellModel for LifeCell {
    fn schema() -> CellSchema {
        CellSchema::enumeration("LifeCell", &LIFE_CELL_STATES)
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
enum MatterCell {
    #[default]
    Empty,
    Water,
    Ice,
}

const MATTER_CELL_STATES: [StateDef; 3] = [
    StateDef::new("Empty"),
    StateDef::new("Water"),
    StateDef::new("Ice"),
];

impl CellModel for MatterCell {
    fn schema() -> CellSchema {
        CellSchema::enumeration("MatterCell", &MATTER_CELL_STATES)
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
enum PriorityCell {
    #[default]
    Empty,
    Source,
    FirstChoice,
    SecondChoice,
}

const PRIORITY_CELL_STATES: [StateDef; 4] = [
    StateDef::new("Empty"),
    StateDef::new("Source"),
    StateDef::new("FirstChoice"),
    StateDef::new("SecondChoice"),
];

impl CellModel for PriorityCell {
    fn schema() -> CellSchema {
        CellSchema::enumeration("PriorityCell", &PRIORITY_CELL_STATES)
    }
}

fn kill_all_spec() -> BlueprintSpec<LifeCell> {
    BlueprintSpec::<LifeCell>::builder()
        .rules(|rules| {
            rules.when(LifeCell::Alive).becomes(LifeCell::Dead);
        })
        .build()
        .expect("valid spec")
}

#[test]
fn rule_kill_all() {
    let spec = kill_all_spec();
    let mut solver = Solver::from_spec(4, 4, 4, &spec);
    solver.set(2, 2, 2, LifeCell::Alive);
    solver.set(1, 1, 1, LifeCell::Alive);

    solver.step();

    assert_eq!(solver.get(2, 2, 2), LifeCell::Dead);
    assert_eq!(solver.get(1, 1, 1), LifeCell::Dead);
}

#[test]
fn solver_from_blueprint_matches_from_spec() {
    let spec = kill_all_spec();
    let blueprint = interpret_blueprint(&spec);

    let mut from_spec = Solver::from_spec(4, 4, 4, &spec);
    let mut from_blueprint = Solver::from_blueprint(4, 4, 4, &blueprint);

    from_spec.set(2, 2, 2, LifeCell::Alive);
    from_blueprint.set(2, 2, 2, LifeCell::Alive);

    from_spec.step();
    from_blueprint.step();

    let from_spec_snapshot = from_spec.readback();
    let from_blueprint_snapshot = from_blueprint.readback();

    assert_eq!(from_spec_snapshot.dims, from_blueprint_snapshot.dims);
    assert_eq!(from_spec_snapshot.cells, from_blueprint_snapshot.cells);
}

#[test]
fn rule_spread_to_neighbors() {
    let spec = BlueprintSpec::<LifeCell>::builder()
        .rules(|rules| {
            rules
                .when(LifeCell::Dead)
                .require(neighbors(LifeCell::Alive).count().at_least(1))
                .becomes(LifeCell::Alive);
        })
        .build()
        .expect("valid spec");

    let mut solver = Solver::from_spec(5, 5, 5, &spec);
    solver.set(2, 2, 2, LifeCell::Alive);

    solver.step();

    assert_eq!(solver.get(2, 2, 2), LifeCell::Alive);
    assert_eq!(solver.get(1, 1, 1), LifeCell::Alive);
    assert_eq!(solver.get(3, 3, 3), LifeCell::Alive);
    assert_eq!(solver.get(2, 2, 1), LifeCell::Alive);
    assert_eq!(solver.get(2, 2, 3), LifeCell::Alive);
}

#[test]
fn rule_threshold_birth() {
    let spec = BlueprintSpec::<LifeCell>::builder()
        .rules(|rules| {
            rules
                .when(LifeCell::Dead)
                .require(neighbors(LifeCell::Alive).count().eq(2))
                .becomes(LifeCell::Alive);
        })
        .build()
        .expect("valid spec");

    let mut solver = Solver::from_spec(5, 5, 5, &spec);
    solver.set(1, 2, 2, LifeCell::Alive);
    solver.set(3, 2, 2, LifeCell::Alive);

    solver.step();

    assert_eq!(solver.get(2, 2, 2), LifeCell::Alive);
}

#[test]
fn rule_threshold_no_birth() {
    let spec = BlueprintSpec::<LifeCell>::builder()
        .rules(|rules| {
            rules
                .when(LifeCell::Dead)
                .require(neighbors(LifeCell::Alive).count().eq(2))
                .becomes(LifeCell::Alive);
        })
        .build()
        .expect("valid spec");

    let mut solver = Solver::from_spec(5, 5, 5, &spec);
    solver.set(1, 2, 2, LifeCell::Alive);

    solver.step();

    assert_eq!(solver.get(2, 2, 2), LifeCell::Dead);
}

#[test]
fn rule_type_interaction() {
    let spec = BlueprintSpec::<MatterCell>::builder()
        .rules(|rules| {
            rules
                .when(MatterCell::Water)
                .require(neighbors(MatterCell::Ice).count().eq(26))
                .becomes(MatterCell::Ice);
        })
        .build()
        .expect("valid spec");

    let mut solver = Solver::from_spec(5, 5, 5, &spec);
    solver.set(2, 2, 2, MatterCell::Water);
    for dz in -1i32..=1 {
        for dy in -1i32..=1 {
            for dx in -1i32..=1 {
                if dx == 0 && dy == 0 && dz == 0 {
                    continue;
                }
                solver.set(2 + dx, 2 + dy, 2 + dz, MatterCell::Ice);
            }
        }
    }

    solver.step();

    assert_eq!(solver.get(2, 2, 2), MatterCell::Ice);
}

#[test]
fn deterministic_across_runs() {
    let spec = BlueprintSpec::<LifeCell>::builder()
        .rules(|rules| {
            rules
                .when(LifeCell::Dead)
                .require(neighbors(LifeCell::Alive).count().eq(3))
                .becomes(LifeCell::Alive);
            rules
                .when(LifeCell::Alive)
                .unless(neighbors(LifeCell::Alive).count().in_range(2..=3))
                .becomes(LifeCell::Dead);
        })
        .build()
        .expect("valid spec");

    fn run_sim(spec: &BlueprintSpec<LifeCell>) -> Vec<(u32, u32, u32, LifeCell)> {
        let mut solver = Solver::from_spec(8, 8, 8, spec);
        solver.set(4, 4, 4, LifeCell::Alive);
        solver.set(3, 4, 4, LifeCell::Alive);
        solver.set(5, 4, 4, LifeCell::Alive);

        for _ in 0..10 {
            solver.step();
        }

        solver.iter_cells()
    }

    let first = run_sim(&spec);
    let second = run_sim(&spec);
    assert_eq!(first, second, "CA is not deterministic");
}

#[test]
fn first_matching_rule_wins() {
    let spec = BlueprintSpec::<PriorityCell>::builder()
        .rules(|rules| {
            rules
                .when(PriorityCell::Source)
                .becomes(PriorityCell::FirstChoice);
            rules
                .when(PriorityCell::Source)
                .becomes(PriorityCell::SecondChoice);
        })
        .build()
        .expect("valid spec");

    let mut solver = Solver::from_spec(3, 3, 3, &spec);
    solver.set(1, 1, 1, PriorityCell::Source);

    solver.step();

    assert_eq!(solver.get(1, 1, 1), PriorityCell::FirstChoice);
}

#[test]
fn random_chance_rules_follow_semantic_rng() {
    let spec = BlueprintSpec::<LifeCell>::builder()
        .rules(|rules| {
            rules
                .when(LifeCell::Dead)
                .require(rng(3).one_in(5))
                .becomes(LifeCell::Alive);
        })
        .build()
        .expect("valid spec");

    let instance = Instance::new(2, 2, 2).with_seed(41);
    let mut solver = Solver::from_spec_instance(instance, &spec);
    let expected = if cell_rng([0, 0, 0], 0, 3, 41).chance(5) {
        LifeCell::Alive
    } else {
        LifeCell::Dead
    };

    solver.step();

    assert_eq!(solver.get(0, 0, 0), expected);
}

#[test]
fn random_chance_rules_change_with_seed() {
    let spec = BlueprintSpec::<LifeCell>::builder()
        .rules(|rules| {
            rules
                .when(LifeCell::Dead)
                .require(rng(3).one_in(5))
                .becomes(LifeCell::Alive);
        })
        .build()
        .expect("valid spec");

    let first = Instance::new(2, 2, 2).with_seed(1);
    let second = Instance::new(2, 2, 2).with_seed(2);

    let mut a = Solver::from_spec_instance(first, &spec);
    let mut b = Solver::from_spec_instance(second, &spec);

    a.step();
    b.step();

    let expected_a = cell_rng([0, 0, 0], 0, 3, 1).chance(5);
    let expected_b = cell_rng([0, 0, 0], 0, 3, 2).chance(5);

    assert_eq!(a.get(0, 0, 0) == LifeCell::Alive, expected_a);
    assert_eq!(b.get(0, 0, 0) == LifeCell::Alive, expected_b);
}

#[test]
fn rule_with_radius_2() {
    let spec = BlueprintSpec::<LifeCell>::builder()
        .neighborhood(
            "radius-two",
            NeighborhoodSpec::new(NeighborhoodShape::Moore, 2, NeighborhoodFalloff::Uniform),
        )
        .rules(|rules| {
            rules
                .when(LifeCell::Dead)
                .using("radius-two")
                .require(neighbors(LifeCell::Alive).count().at_least(1))
                .becomes(LifeCell::Alive);
        })
        .build()
        .expect("valid spec");

    let mut solver = Solver::from_spec(8, 8, 8, &spec);
    solver.set(4, 4, 4, LifeCell::Alive);

    solver.step();

    assert_eq!(solver.get(2, 4, 4), LifeCell::Alive);
    assert_eq!(solver.get(6, 4, 4), LifeCell::Alive);
    assert_eq!(solver.get(1, 4, 4), LifeCell::Dead);
}

#[test]
fn rule_respects_torus_topology_from_spec() {
    let spec = BlueprintSpec::<LifeCell>::builder()
        .topology(TopologyDescriptor::wrap())
        .rules(|rules| {
            rules
                .when(LifeCell::Dead)
                .require(neighbors(LifeCell::Alive).any())
                .becomes(LifeCell::Alive);
        })
        .build()
        .expect("valid spec");

    let mut solver = Solver::from_spec(4, 4, 4, &spec);
    solver.set(3, 0, 0, LifeCell::Alive);

    solver.step();

    assert_eq!(solver.get(0, 0, 0), LifeCell::Alive);
}

#[test]
fn weighted_sum_rules_follow_portable_weights() {
    let spec = BlueprintSpec::<LifeCell>::builder()
        .rules(|rules| {
            rules
                .when(LifeCell::Dead)
                .require(
                    neighbors(LifeCell::Alive)
                        .weighted_sum()
                        .at_least(Weight::cells(6)),
                )
                .becomes(LifeCell::Alive);
        })
        .build()
        .expect("valid spec");

    let mut solver = Solver::from_spec(3, 3, 3, &spec);
    solver.set(0, 1, 1, LifeCell::Alive);
    solver.set(2, 1, 1, LifeCell::Alive);
    solver.set(1, 0, 1, LifeCell::Alive);
    solver.set(1, 2, 1, LifeCell::Alive);
    solver.set(1, 1, 0, LifeCell::Alive);
    solver.set(1, 1, 2, LifeCell::Alive);

    solver.step();

    assert_eq!(solver.get(1, 1, 1), LifeCell::Alive);
}
