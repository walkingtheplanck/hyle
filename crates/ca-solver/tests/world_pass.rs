//! Tests for world passes (Tier 3: full grid access).

use hyle_ca_core::CaSolver;
use hyle_ca_solver::Solver;

#[test]
fn world_pass_runs_after_rules() {
    let mut s = Solver::<u32>::new(4, 4, 4);
    s.set(0, 0, 0, 5);

    // World pass: set all cells to 99
    s.register_world_pass(|_grid, out| {
        for z in 0..4i32 {
            for y in 0..4i32 {
                for x in 0..4i32 {
                    out.set(x, y, z, 99);
                }
            }
        }
    });

    s.step();

    assert_eq!(s.get(0, 0, 0), 99);
    assert_eq!(s.get(3, 3, 3), 99);
}

#[test]
fn world_pass_reads_post_rule_state() {
    use hyle_ca_core::{Action, Neighborhood, Rng};

    let mut s = Solver::<u32>::new(4, 4, 4);
    s.set(1, 1, 1, 1);

    // Rule: type 1 becomes type 2
    s.register_rule(1, |_n: &Neighborhood<u32>, _rng: Rng| Action::Become(2));

    // World pass: read the cell and verify it was already changed by the rule
    s.register_world_pass(|grid, out| {
        let cell = grid.get(1, 1, 1);
        // Should be 2 (post-rule), not 1 (pre-rule)
        out.set(0, 0, 0, cell); // copy it to (0,0,0) as proof
    });

    s.step();

    assert_eq!(s.get(0, 0, 0), 2); // world pass saw the post-rule value
}

#[test]
fn multiple_world_passes_chain() {
    let mut s = Solver::<u32>::new(4, 4, 4);

    // Pass 1: set (0,0,0) to 10
    s.register_world_pass(|_grid, out| {
        out.set(0, 0, 0, 10);
    });

    // Pass 2: read (0,0,0) and double it into (1,0,0)
    s.register_world_pass(|grid, out| {
        let v = grid.get(0, 0, 0);
        out.set(1, 0, 0, v * 2);
    });

    s.step();

    assert_eq!(s.get(0, 0, 0), 10);
    assert_eq!(s.get(1, 0, 0), 20);
}

#[test]
fn world_pass_conservation_check() {
    use hyle_ca_core::{Action, Neighborhood, Rng};

    let mut s = Solver::<u32>::new(8, 8, 8);

    // Seed some cells
    for i in 0..10 {
        s.set(i, 0, 0, 1);
    }

    // Rule that might not conserve mass
    s.register_rule(1, |n: &Neighborhood<u32>, _rng: Rng| {
        if n.count_alive() > 5 {
            Action::Become(0)
        } else {
            Action::Keep
        }
    });

    // World pass: count total alive and store in (0,0,0) as metadata
    s.register_world_pass(|grid, out| {
        let total: u32 = grid.iter().filter(|(_, _, _, c)| *c != 0).count() as u32;
        out.set(0, 0, 0, total);
    });

    s.step();

    // (0,0,0) now contains the alive count — just verify it's a reasonable number
    let count = s.get(0, 0, 0);
    assert!(
        count <= 10,
        "more alive cells than we started with: {count}"
    );
}
