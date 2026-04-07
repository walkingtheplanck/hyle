//! Rule application tests: define rules, step, assert outcomes.
//! These are generic CA behaviors, not tied to any specific game.

use hyle_ca_core::{Action, CaSolver, Neighborhood, Rng};
use hyle_ca_solver::Solver;

// -- Rule: unconditional death ------------------------------------------------

#[test]
fn rule_kill_all() {
    let mut s = Solver::<u32>::new(4, 4, 4);
    s.set(2, 2, 2, 1);
    s.set(1, 1, 1, 1);
    s.register_rule(1, |_n: &dyn Neighborhood<u32>, _rng: Rng| Action::Become(0));

    s.step();

    // All type-1 cells should be dead
    assert_eq!(s.get(2, 2, 2), 0);
    assert_eq!(s.get(1, 1, 1), 0);
}

// -- Rule: unconditional spread -----------------------------------------------

#[test]
fn rule_spread_to_neighbors() {
    // Type 0 (dead) becomes alive if ANY neighbor is alive
    let mut s = Solver::<u32>::new(5, 5, 5);
    s.set(2, 2, 2, 1); // single alive cell in center

    s.register_rule(0, |n: &dyn Neighborhood<u32>, _rng: Rng| {
        if n.count_alive() > 0 {
            Action::Become(1)
        } else {
            Action::Keep
        }
    });
    // Alive cells stay alive
    s.register_rule(1, |_n: &dyn Neighborhood<u32>, _rng: Rng| Action::Keep);

    s.step();

    // Center stays alive
    assert_eq!(s.get(2, 2, 2), 1);
    // All 26 neighbors should now be alive
    assert_eq!(s.get(1, 1, 1), 1);
    assert_eq!(s.get(3, 3, 3), 1);
    assert_eq!(s.get(2, 2, 1), 1);
    assert_eq!(s.get(2, 2, 3), 1);
}

// -- Rule: threshold birth/death ----------------------------------------------

#[test]
fn rule_threshold_birth() {
    // Dead cell becomes alive with exactly 2 alive neighbors
    let mut s = Solver::<u32>::new(5, 5, 5);

    // Place 2 alive cells adjacent to (2,2,2)
    s.set(1, 2, 2, 1);
    s.set(3, 2, 2, 1);

    s.register_rule(0, |n: &dyn Neighborhood<u32>, _rng: Rng| {
        if n.count_alive() == 2 {
            Action::Become(1)
        } else {
            Action::Keep
        }
    });
    s.register_rule(1, |_n: &dyn Neighborhood<u32>, _rng: Rng| Action::Keep);

    s.step();

    // (2,2,2) had exactly 2 alive neighbors → should be born
    assert_eq!(s.get(2, 2, 2), 1);
}

#[test]
fn rule_threshold_no_birth() {
    // Dead cell needs exactly 2 neighbors — give it 1
    let mut s = Solver::<u32>::new(5, 5, 5);
    s.set(1, 2, 2, 1); // only 1 neighbor

    s.register_rule(0, |n: &dyn Neighborhood<u32>, _rng: Rng| {
        if n.count_alive() == 2 {
            Action::Become(1)
        } else {
            Action::Keep
        }
    });
    s.register_rule(1, |_n: &dyn Neighborhood<u32>, _rng: Rng| Action::Keep);

    s.step();

    // (2,2,2) had only 1 neighbor → stays dead
    assert_eq!(s.get(2, 2, 2), 0);
}

// -- Rule: multi-type interaction ---------------------------------------------

#[test]
fn rule_type_interaction() {
    // Type 1 (water) becomes type 2 (ice) when surrounded by type 2
    // Type 2 (ice) stays ice
    let mut s = Solver::<u32>::new(5, 5, 5);

    // Single water cell surrounded by ice
    s.set(2, 2, 2, 1); // water
    for dz in -1i32..=1 {
        for dy in -1i32..=1 {
            for dx in -1i32..=1 {
                if dx == 0 && dy == 0 && dz == 0 {
                    continue;
                }
                s.set(2 + dx, 2 + dy, 2 + dz, 2); // ice
            }
        }
    }

    s.register_rule(1, |n: &dyn Neighborhood<u32>, _rng: Rng| {
        let ice_count = n.count(&|c| c == 2);
        if ice_count == 26 {
            Action::Become(2) // freeze
        } else {
            Action::Keep
        }
    });
    s.register_rule(2, |_n: &dyn Neighborhood<u32>, _rng: Rng| Action::Keep);

    s.step();

    // Water should have frozen
    assert_eq!(s.get(2, 2, 2), 2);
}

// -- Rule: closure captures state ---------------------------------------------

#[test]
fn rule_closure_captures_threshold() {
    let threshold = 3u32;
    let mut s = Solver::<u32>::new(5, 5, 5);

    // Place 3 alive cells around center
    s.set(1, 2, 2, 1);
    s.set(3, 2, 2, 1);
    s.set(2, 1, 2, 1);

    s.register_rule(0, move |n: &dyn Neighborhood<u32>, _rng: Rng| {
        if n.count_alive() >= threshold {
            Action::Become(1)
        } else {
            Action::Keep
        }
    });
    s.register_rule(1, |_n: &dyn Neighborhood<u32>, _rng: Rng| Action::Keep);

    s.step();

    assert_eq!(s.get(2, 2, 2), 1); // born: had 3 neighbors >= threshold
}

// -- Determinism: same setup produces same result -----------------------------

#[test]
fn deterministic_across_runs() {
    fn run_sim() -> Vec<(u32, u32, u32, u32)> {
        let mut s = Solver::<u32>::new(8, 8, 8);
        s.set(4, 4, 4, 1);
        s.set(3, 4, 4, 1);
        s.set(5, 4, 4, 1);

        s.register_rule(0, |n: &dyn Neighborhood<u32>, rng: Rng| {
            if n.count_alive() == 3 && rng.chance(2) {
                Action::Become(1)
            } else {
                Action::Keep
            }
        });
        s.register_rule(1, |n: &dyn Neighborhood<u32>, _rng: Rng| {
            if n.count_alive() < 2 {
                Action::Become(0)
            } else {
                Action::Keep
            }
        });

        for _ in 0..10 {
            s.step();
        }
        s.iter_cells()
    }

    let a = run_sim();
    let b = run_sim();
    assert_eq!(a, b, "CA is not deterministic");
}

// -- Double buffer: rules see pre-step state, not partial updates -------------

#[test]
fn double_buffer_isolation() {
    // A rule that kills cells. If double-buffering is broken,
    // earlier cells being killed would affect later cells' neighbor counts.
    let mut s = Solver::<u32>::new(5, 5, 5);

    // Fill a 3x3x3 cube with alive cells
    for z in 1..=3 {
        for y in 1..=3 {
            for x in 1..=3 {
                s.set(x, y, z, 1);
            }
        }
    }

    // Rule: die if you have exactly 26 alive neighbors (fully surrounded)
    s.register_rule(1, |n: &dyn Neighborhood<u32>, _rng: Rng| {
        if n.count_alive() == 26 {
            Action::Become(0)
        } else {
            Action::Keep
        }
    });

    s.step();

    // Only (2,2,2) is fully surrounded — it should die
    assert_eq!(s.get(2, 2, 2), 0);
    // Corner cells had < 26 neighbors alive — should survive
    assert_eq!(s.get(1, 1, 1), 1);
    assert_eq!(s.get(3, 3, 3), 1);
}

// -- Extended radius ----------------------------------------------------------

#[test]
fn rule_with_radius_2() {
    let mut s = Solver::<u32>::new(8, 8, 8);
    s.set(4, 4, 4, 1); // single cell at center

    // Dead cells at radius 2 can see the alive cell
    s.register_rule_with_radius(0, 2, |n: &dyn Neighborhood<u32>, _rng: Rng| {
        if n.count_alive() > 0 {
            Action::Become(1)
        } else {
            Action::Keep
        }
    });
    s.register_rule(1, |_n: &dyn Neighborhood<u32>, _rng: Rng| Action::Keep);

    s.step();

    // Cell at distance 2 should be alive (within radius-2 neighborhood)
    assert_eq!(s.get(2, 4, 4), 1); // 2 away on x axis
    assert_eq!(s.get(6, 4, 4), 1); // 2 away on other side

    // Cell at distance 3 should still be dead (outside radius 2)
    assert_eq!(s.get(1, 4, 4), 0);
}
