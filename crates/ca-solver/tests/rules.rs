//! Rule application tests using declarative blueprint specs.

use hyle_ca_interface::{
    neighbors, BlueprintSpec, CaSolver, Cell, Hyle, NeighborhoodFalloff, NeighborhoodShape,
    NeighborhoodSpec, TopologyDescriptor,
};
use hyle_ca_semantics::interpret_blueprint;
use hyle_ca_solver::Solver;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
enum LifeCell {
    #[default]
    Dead,
    Alive,
}

impl Cell for LifeCell {
    fn rule_id(&self) -> u8 {
        match self {
            Self::Dead => 0,
            Self::Alive => 1,
        }
    }

    fn is_alive(&self) -> bool {
        matches!(self, Self::Alive)
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
enum MatterCell {
    #[default]
    Empty,
    Water,
    Ice,
}

impl Cell for MatterCell {
    fn rule_id(&self) -> u8 {
        match self {
            Self::Empty => 0,
            Self::Water => 1,
            Self::Ice => 2,
        }
    }

    fn is_alive(&self) -> bool {
        !matches!(self, Self::Empty)
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

impl Cell for PriorityCell {
    fn rule_id(&self) -> u8 {
        match self {
            Self::Empty => 0,
            Self::Source => 1,
            Self::FirstChoice => 2,
            Self::SecondChoice => 3,
        }
    }

    fn is_alive(&self) -> bool {
        !matches!(self, Self::Empty)
    }
}

fn kill_all_spec() -> BlueprintSpec<LifeCell> {
    Hyle::builder()
        .cells::<LifeCell>()
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
    let spec = Hyle::builder()
        .cells::<LifeCell>()
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
    let spec = Hyle::builder()
        .cells::<LifeCell>()
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
    let spec = Hyle::builder()
        .cells::<LifeCell>()
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
    let spec = Hyle::builder()
        .cells::<MatterCell>()
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
    let spec = Hyle::builder()
        .cells::<LifeCell>()
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
    let spec = Hyle::builder()
        .cells::<PriorityCell>()
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
fn rule_with_radius_2() {
    let spec = Hyle::builder()
        .cells::<LifeCell>()
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
    let spec = Hyle::builder()
        .cells::<LifeCell>()
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
