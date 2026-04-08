//! Tests for installing grouped rules and world passes.

use hyle_ca_core::{moore, unweighted, Action, CaSolver, Neighborhood, Rng};
use hyle_ca_solver::{RuleSet, Solver};

#[test]
fn installs_named_rule_set_rules() {
    let mut s = Solver::<u32>::new(5, 5, 5);
    s.set(2, 2, 2, 1);

    let rules =
        RuleSet::new("kill-alive").rule(1, |_n: &Neighborhood<u32>, _rng: Rng| Action::Become(0));

    assert_eq!(rules.name(), "kill-alive");

    s.install_rule_set(rules);
    s.step();

    assert_eq!(s.get(2, 2, 2), 0);
}

#[test]
fn rule_set_supports_custom_radius() {
    let mut s = Solver::<u32>::new(8, 8, 8);
    s.set(4, 4, 4, 1);

    let rules = RuleSet::new("radius-two")
        .rule_with_radius(0, 2, |n: &Neighborhood<u32>, _rng: Rng| {
            if n.count_alive() > 0 {
                Action::Become(1)
            } else {
                Action::Keep
            }
        })
        .rule(1, |_n: &Neighborhood<u32>, _rng: Rng| Action::Keep);

    s.install_rule_set(rules);
    s.step();

    assert_eq!(s.get(2, 4, 4), 1);
    assert_eq!(s.get(1, 4, 4), 0);
}

#[test]
fn rule_set_supports_custom_shape() {
    let mut s = Solver::<u32>::new(5, 5, 5);
    s.set(2, 2, 2, 1);

    let rules = RuleSet::new("moore-shape")
        .rule_with_shape(
            0,
            1,
            moore,
            unweighted,
            |n: &Neighborhood<u32>, _rng: Rng| {
                if n.count_alive() > 0 {
                    Action::Become(2)
                } else {
                    Action::Keep
                }
            },
        )
        .rule(1, |_n: &Neighborhood<u32>, _rng: Rng| Action::Keep);

    s.install_rule_set(rules);
    s.step();

    assert_eq!(s.get(1, 1, 1), 2);
}

#[test]
fn rule_set_world_passes_append_in_order() {
    let mut s = Solver::<u32>::new(4, 4, 4);

    let rules = RuleSet::new("passes")
        .world_pass(|_grid, out| {
            out.set(0, 0, 0, 10);
        })
        .world_pass(|grid, out| {
            out.set(1, 0, 0, grid.get(0, 0, 0) * 2);
        });

    s.install_rule_set(rules);
    s.step();

    assert_eq!(s.get(0, 0, 0), 10);
    assert_eq!(s.get(1, 0, 0), 20);
}

#[test]
fn later_rule_set_registration_overrides_same_cell_type() {
    let mut s = Solver::<u32>::new(4, 4, 4);
    s.set(1, 1, 1, 1);

    s.install_rule_set(
        RuleSet::new("first").rule(1, |_n: &Neighborhood<u32>, _rng: Rng| Action::Become(2)),
    );
    s.install_rule_set(
        RuleSet::new("second").rule(1, |_n: &Neighborhood<u32>, _rng: Rng| Action::Become(3)),
    );

    s.step();

    assert_eq!(s.get(1, 1, 1), 3);
}
