//! Rule application tests using declarative automaton specs.

use hyle_ca_contracts::{neighbors, CaSolver, Hyle, NeighborhoodSpec, TopologyDescriptor};
use hyle_ca_solver::Solver;

fn kill_all_spec() -> hyle_ca_contracts::AutomatonSpec<u32> {
    Hyle::builder()
        .cells::<u32>()
        .rules(|rules| {
            rules.when(1).becomes(0);
        })
        .build()
        .expect("valid spec")
}

#[test]
fn rule_kill_all() {
    let spec = kill_all_spec();
    let mut solver = Solver::from_spec(4, 4, 4, &spec);
    solver.set(2, 2, 2, 1);
    solver.set(1, 1, 1, 1);

    solver.step();

    assert_eq!(solver.get(2, 2, 2), 0);
    assert_eq!(solver.get(1, 1, 1), 0);
}

#[test]
fn rule_spread_to_neighbors() {
    let spec = Hyle::builder()
        .cells::<u32>()
        .rules(|rules| {
            rules
                .when(0)
                .require(neighbors(1).count().at_least(1))
                .becomes(1);
        })
        .build()
        .expect("valid spec");

    let mut solver = Solver::from_spec(5, 5, 5, &spec);
    solver.set(2, 2, 2, 1);

    solver.step();

    assert_eq!(solver.get(2, 2, 2), 1);
    assert_eq!(solver.get(1, 1, 1), 1);
    assert_eq!(solver.get(3, 3, 3), 1);
    assert_eq!(solver.get(2, 2, 1), 1);
    assert_eq!(solver.get(2, 2, 3), 1);
}

#[test]
fn rule_threshold_birth() {
    let spec = Hyle::builder()
        .cells::<u32>()
        .rules(|rules| {
            rules.when(0).require(neighbors(1).count().eq(2)).becomes(1);
        })
        .build()
        .expect("valid spec");

    let mut solver = Solver::from_spec(5, 5, 5, &spec);
    solver.set(1, 2, 2, 1);
    solver.set(3, 2, 2, 1);

    solver.step();

    assert_eq!(solver.get(2, 2, 2), 1);
}

#[test]
fn rule_threshold_no_birth() {
    let spec = Hyle::builder()
        .cells::<u32>()
        .rules(|rules| {
            rules.when(0).require(neighbors(1).count().eq(2)).becomes(1);
        })
        .build()
        .expect("valid spec");

    let mut solver = Solver::from_spec(5, 5, 5, &spec);
    solver.set(1, 2, 2, 1);

    solver.step();

    assert_eq!(solver.get(2, 2, 2), 0);
}

#[test]
fn rule_type_interaction() {
    let spec = Hyle::builder()
        .cells::<u32>()
        .rules(|rules| {
            rules
                .when(1)
                .require(neighbors(2).count().eq(26))
                .becomes(2);
        })
        .build()
        .expect("valid spec");

    let mut solver = Solver::from_spec(5, 5, 5, &spec);
    solver.set(2, 2, 2, 1);
    for dz in -1i32..=1 {
        for dy in -1i32..=1 {
            for dx in -1i32..=1 {
                if dx == 0 && dy == 0 && dz == 0 {
                    continue;
                }
                solver.set(2 + dx, 2 + dy, 2 + dz, 2);
            }
        }
    }

    solver.step();

    assert_eq!(solver.get(2, 2, 2), 2);
}

#[test]
fn deterministic_across_runs() {
    let spec = Hyle::builder()
        .cells::<u32>()
        .rules(|rules| {
            rules.when(0).require(neighbors(1).count().eq(3)).becomes(1);
            rules
                .when(1)
                .unless(neighbors(1).count().in_range(2..=3))
                .becomes(0);
        })
        .build()
        .expect("valid spec");

    fn run_sim(spec: &hyle_ca_contracts::AutomatonSpec<u32>) -> Vec<(u32, u32, u32, u32)> {
        let mut solver = Solver::from_spec(8, 8, 8, spec);
        solver.set(4, 4, 4, 1);
        solver.set(3, 4, 4, 1);
        solver.set(5, 4, 4, 1);

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
        .cells::<u32>()
        .rules(|rules| {
            rules.when(1).becomes(2);
            rules.when(1).becomes(3);
        })
        .build()
        .expect("valid spec");

    let mut solver = Solver::from_spec(3, 3, 3, &spec);
    solver.set(1, 1, 1, 1);

    solver.step();

    assert_eq!(solver.get(1, 1, 1), 2);
}

#[test]
fn rule_with_radius_2() {
    let spec = Hyle::builder()
        .cells::<u32>()
        .neighborhood("radius-two", NeighborhoodSpec::cube(2))
        .rules(|rules| {
            rules
                .when(0)
                .using("radius-two")
                .require(neighbors(1).count().at_least(1))
                .becomes(1);
        })
        .build()
        .expect("valid spec");

    let mut solver = Solver::from_spec(8, 8, 8, &spec);
    solver.set(4, 4, 4, 1);

    solver.step();

    assert_eq!(solver.get(2, 4, 4), 1);
    assert_eq!(solver.get(6, 4, 4), 1);
    assert_eq!(solver.get(1, 4, 4), 0);
}

#[test]
fn rule_respects_torus_topology_from_spec() {
    let spec = Hyle::builder()
        .cells::<u32>()
        .topology(TopologyDescriptor::wrap())
        .rules(|rules| {
            rules.when(0).require(neighbors(1).any()).becomes(1);
        })
        .build()
        .expect("valid spec");

    let mut solver = Solver::from_spec(4, 4, 4, &spec);
    solver.set(3, 0, 0, 1);

    solver.step();

    assert_eq!(solver.get(0, 0, 0), 1);
}
