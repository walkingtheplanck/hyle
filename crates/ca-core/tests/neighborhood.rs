//! Tests for Neighborhood: construction, access, counting.

use hyle_ca_core::Neighborhood;

fn filled_neighborhood(center: u32) -> Neighborhood<u32> {
    let mut n = Neighborhood::new(1);
    // Fill with values 1..=26, center at origin
    n.fill(center, [5, 5, 5], |dx, dy, dz| {
        ((dx + 1) + (dy + 1) * 3 + (dz + 1) * 9) as u32
    });
    n
}

#[test]
fn center_is_stored() {
    let n = filled_neighborhood(42);
    assert_eq!(n.center, 42);
}

#[test]
fn pos_is_stored() {
    let n = filled_neighborhood(0);
    assert_eq!(n.pos, [5, 5, 5]);
}

#[test]
fn get_returns_sampled_values() {
    let n = filled_neighborhood(0);
    // (-1,-1,-1) → sampled with offsets (-1,-1,-1) → value 0
    assert_eq!(n.get(-1, -1, -1), 0);

    // (1,1,1) → sampled with offsets (1,1,1) → (1+1) + (1+1)*3 + (1+1)*9 = 26
    assert_eq!(n.get(1, 1, 1), 26);
}

#[test]
fn count_alive_excludes_zeros() {
    let mut n = Neighborhood::new(1);
    // Fill: only cells where dx > 0 are alive
    n.fill(0, [0, 0, 0], |dx, _dy, _dz| if dx > 0 { 1 } else { 0 });
    // dx=1 with dy in -1..=1 and dz in -1..=1 = 3*3 = 9 alive cells
    assert_eq!(n.count_alive(), 9);
}

#[test]
fn count_with_predicate() {
    let mut n = Neighborhood::new(1);
    n.fill(0, [0, 0, 0], |dx, dy, dz| (dx + dy + dz + 3) as u32);
    // Count cells with value > 4
    let count = n.count(|c| c > 4);
    assert!(count > 0);
    assert!(count < 26);
}

#[test]
fn radius_1_has_26_neighbors() {
    let mut n = Neighborhood::new(1);
    n.fill(0, [0, 0, 0], |_, _, _| 1);
    // 3^3 - 1 = 26
    assert_eq!(n.count_alive(), 26);
}

#[test]
fn radius_2_has_124_neighbors() {
    let mut n = Neighborhood::new(2);
    n.fill(0, [0, 0, 0], |_, _, _| 1);
    // 5^3 - 1 = 124
    assert_eq!(n.count_alive(), 124);
}

#[test]
fn resize_changes_capacity() {
    let mut n = Neighborhood::<u32>::new(1);
    assert_eq!(n.radius(), 1);

    n.resize(3);
    assert_eq!(n.radius(), 3);

    n.fill(0, [0, 0, 0], |_, _, _| 1);
    // 7^3 - 1 = 342
    assert_eq!(n.count_alive(), 342);
}

#[test]
#[cfg_attr(not(debug_assertions), ignore)]
#[should_panic]
fn get_center_panics_in_debug() {
    let n = filled_neighborhood(0);
    n.get(0, 0, 0); // should panic — only in debug builds
}

#[test]
fn symmetry_opposite_offsets_are_different() {
    let mut n = Neighborhood::new(1);
    n.fill(0, [0, 0, 0], |dx, dy, dz| {
        // Unique value per offset
        ((dx + 2) * 100 + (dy + 2) * 10 + (dz + 2)) as u32
    });
    // Opposite corners should have different values
    assert_ne!(n.get(-1, -1, -1), n.get(1, 1, 1));
    assert_ne!(n.get(-1, 0, 0), n.get(1, 0, 0));
}
