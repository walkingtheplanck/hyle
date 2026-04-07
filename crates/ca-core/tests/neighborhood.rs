//! Tests for Neighborhood trait and built-in shapes.

use hyle_ca_core::{
    inverse_square, MooreNeighborhood, Neighborhood, SphericalNeighborhood, VonNeumannNeighborhood,
};

// ---------------------------------------------------------------------------
// Moore
// ---------------------------------------------------------------------------

fn filled_moore(center: u32) -> MooreNeighborhood<u32> {
    let mut n = MooreNeighborhood::new(1);
    n.fill(center, [5, 5, 5], |dx, dy, dz| {
        ((dx + 1) + (dy + 1) * 3 + (dz + 1) * 9) as u32
    });
    n
}

#[test]
fn moore_center_is_stored() {
    let n = filled_moore(42);
    assert_eq!(n.center(), 42);
}

#[test]
fn moore_pos_is_stored() {
    let n = filled_moore(0);
    assert_eq!(n.pos(), [5, 5, 5]);
}

#[test]
fn moore_get_returns_sampled_values() {
    let n = filled_moore(0);
    assert_eq!(n.get(-1, -1, -1), 0);
    assert_eq!(n.get(1, 1, 1), 26);
}

#[test]
fn moore_count_alive_excludes_zeros() {
    let mut n = MooreNeighborhood::new(1);
    n.fill(0, [0, 0, 0], |dx, _dy, _dz| if dx > 0 { 1 } else { 0 });
    assert_eq!(n.count_alive(), 9);
}

#[test]
fn moore_count_with_predicate() {
    let mut n = MooreNeighborhood::new(1);
    n.fill(0, [0, 0, 0], |dx, dy, dz| (dx + dy + dz + 3) as u32);
    let count = n.count(&|c| c > 4);
    assert!(count > 0);
    assert!(count < 26);
}

#[test]
fn moore_r1_has_26_neighbors() {
    let mut n = MooreNeighborhood::new(1);
    n.fill(0, [0, 0, 0], |_, _, _| 1);
    assert_eq!(n.count_alive(), 26);
}

#[test]
fn moore_r2_has_124_neighbors() {
    let mut n = MooreNeighborhood::new(2);
    n.fill(0, [0, 0, 0], |_, _, _| 1);
    assert_eq!(n.count_alive(), 124);
}

#[test]
fn moore_r3_has_342_neighbors() {
    let mut n = MooreNeighborhood::new(3);
    n.fill(0, [0, 0, 0], |_, _, _| 1);
    assert_eq!(n.count_alive(), 342);
}

#[test]
fn moore_resize_changes_capacity() {
    let mut n = MooreNeighborhood::<u32>::new(1);
    assert_eq!(n.radius(), 1);

    n.resize(3);
    assert_eq!(n.radius(), 3);

    n.fill(0, [0, 0, 0], |_, _, _| 1);
    assert_eq!(n.count_alive(), 342);
}

#[test]
fn moore_symmetry_opposite_offsets_are_different() {
    let mut n = MooreNeighborhood::new(1);
    n.fill(0, [0, 0, 0], |dx, dy, dz| {
        ((dx + 2) * 100 + (dy + 2) * 10 + (dz + 2)) as u32
    });
    assert_ne!(n.get(-1, -1, -1), n.get(1, 1, 1));
    assert_ne!(n.get(-1, 0, 0), n.get(1, 0, 0));
}

#[test]
#[cfg_attr(not(debug_assertions), ignore)]
#[should_panic]
fn moore_get_center_panics_in_debug() {
    let n = filled_moore(0);
    n.get(0, 0, 0);
}

// ---------------------------------------------------------------------------
// Von Neumann
// ---------------------------------------------------------------------------

#[test]
fn vn_r1_has_6_neighbors() {
    let mut n = VonNeumannNeighborhood::new(1);
    n.fill(0, [0, 0, 0], |_, _, _| 1);
    assert_eq!(n.count_alive(), 6);
}

#[test]
fn vn_r2_has_24_neighbors() {
    let mut n = VonNeumannNeighborhood::new(2);
    n.fill(0, [0, 0, 0], |_, _, _| 1);
    assert_eq!(n.count_alive(), 24);
}

#[test]
fn vn_r3_has_62_neighbors() {
    let mut n = VonNeumannNeighborhood::new(3);
    n.fill(0, [0, 0, 0], |_, _, _| 1);
    assert_eq!(n.count_alive(), 62);
}

#[test]
fn vn_r1_only_face_adjacent() {
    let mut n = VonNeumannNeighborhood::new(1);
    n.fill(0, [0, 0, 0], |_, _, _| 1);
    // Face neighbors exist
    assert_eq!(n.get(1, 0, 0), 1);
    assert_eq!(n.get(-1, 0, 0), 1);
    assert_eq!(n.get(0, 1, 0), 1);
    assert_eq!(n.get(0, -1, 0), 1);
    assert_eq!(n.get(0, 0, 1), 1);
    assert_eq!(n.get(0, 0, -1), 1);
}

#[test]
#[cfg_attr(not(debug_assertions), ignore)]
#[should_panic]
fn vn_r1_diagonal_panics_in_debug() {
    let mut n = VonNeumannNeighborhood::new(1);
    n.fill(0, [0, 0, 0], |_, _, _| 1);
    n.get(1, 1, 0); // diagonal — not in Von Neumann R=1
}

// ---------------------------------------------------------------------------
// Spherical
// ---------------------------------------------------------------------------

#[test]
fn spherical_r1_has_6_neighbors() {
    let mut n = SphericalNeighborhood::new(1);
    n.fill(0, [0, 0, 0], |_, _, _| 1);
    assert_eq!(n.count_alive(), 6);
}

#[test]
fn spherical_r2_has_32_neighbors() {
    let mut n = SphericalNeighborhood::new(2);
    n.fill(0, [0, 0, 0], |_, _, _| 1);
    assert_eq!(n.count_alive(), 32);
}

#[test]
fn spherical_r3_has_122_neighbors() {
    let mut n = SphericalNeighborhood::new(3);
    n.fill(0, [0, 0, 0], |_, _, _| 1);
    assert_eq!(n.count_alive(), 122);
}

#[test]
fn spherical_r2_includes_face_and_edge_but_not_corner() {
    let mut n = SphericalNeighborhood::new(2);
    n.fill(0, [0, 0, 0], |_, _, _| 1);
    // (2,0,0) → d²=4 ≤ 4 ✓
    assert_eq!(n.get(2, 0, 0), 1);
    // (1,1,0) → d²=2 ≤ 4 ✓
    assert_eq!(n.get(1, 1, 0), 1);
    // (1,1,1) → d²=3 ≤ 4 ✓
    assert_eq!(n.get(1, 1, 1), 1);
}

// ---------------------------------------------------------------------------
// Weighted counting
// ---------------------------------------------------------------------------

#[test]
fn count_weighted_inverse_square() {
    let mut n = MooreNeighborhood::new(1);
    n.fill(0, [0, 0, 0], |_, _, _| 1);
    let w = n.count_weighted(&inverse_square);
    assert!(w > 0.0);
    // 6 face neighbors at d=1 contribute 6.0
    // 12 edge neighbors at d=√2 contribute 12/2 = 6.0
    // 8 corner neighbors at d=√3 contribute 8/3 ≈ 2.67
    // Total ≈ 14.67
    assert!((w - 14.667).abs() < 0.1);
}

#[test]
fn count_weighted_custom_fn() {
    let mut n = VonNeumannNeighborhood::new(1);
    n.fill(0, [0, 0, 0], |_, _, _| 1);
    // All 6 neighbors at distance 1.0, weight = 2.0 each → total 12.0
    let w = n.count_weighted(&|_d| 2.0);
    assert!((w - 12.0).abs() < f32::EPSILON);
}

// ---------------------------------------------------------------------------
// iter() and neighbor_count()
// ---------------------------------------------------------------------------

#[test]
fn iter_returns_all_entries() {
    let mut n = VonNeumannNeighborhood::new(1);
    n.fill(0, [0, 0, 0], |_, _, _| 1);
    assert_eq!(n.iter().len(), 6);
    assert_eq!(n.neighbor_count(), 6);
}

#[test]
fn iter_entries_have_correct_offsets() {
    let mut n = VonNeumannNeighborhood::new(1);
    n.fill(0, [0, 0, 0], |dx, dy, dz| {
        (dx.abs() + dy.abs() + dz.abs()) as u32
    });
    for &(dx, dy, dz, c) in n.iter() {
        // All VN R=1 neighbors have Manhattan distance 1
        assert_eq!(dx.abs() + dy.abs() + dz.abs(), 1);
        assert_eq!(c, 1);
    }
}
