//! Tests for Neighborhood struct, shapes, and weights.

use hyle_ca_interface::semantics::expand_neighborhood;
use hyle_ca_interface::semantics::WEIGHT_SCALE;
use hyle_ca_interface::{NeighborhoodFalloff, NeighborhoodShape, NeighborhoodSpec};
use hyle_ca_solver::Neighborhood;

fn runtime_neighborhood(
    shape: NeighborhoodShape,
    radius: u32,
    falloff: NeighborhoodFalloff,
) -> Neighborhood<u32> {
    let semantic = expand_neighborhood(NeighborhoodSpec::new(shape, radius, falloff));
    Neighborhood::new(semantic.samples())
}

// ---------------------------------------------------------------------------
// Moore
// ---------------------------------------------------------------------------

fn filled_moore(center: u32) -> Neighborhood<u32> {
    let mut n = runtime_neighborhood(NeighborhoodShape::Moore, 1, NeighborhoodFalloff::Uniform);
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
    let mut n = runtime_neighborhood(NeighborhoodShape::Moore, 1, NeighborhoodFalloff::Uniform);
    n.fill(0, [0, 0, 0], |dx, _dy, _dz| if dx > 0 { 1 } else { 0 });
    assert_eq!(n.count_alive(), 9);
}

#[test]
fn moore_count_with_predicate() {
    let mut n = runtime_neighborhood(NeighborhoodShape::Moore, 1, NeighborhoodFalloff::Uniform);
    n.fill(0, [0, 0, 0], |dx, dy, dz| (dx + dy + dz + 3) as u32);
    let count = n.count(|e| e.cell > 4);
    assert!(count > 0);
    assert!(count < 26);
}

#[test]
fn moore_r1_has_26_neighbors() {
    let mut n = runtime_neighborhood(NeighborhoodShape::Moore, 1, NeighborhoodFalloff::Uniform);
    n.fill(0, [0, 0, 0], |_, _, _| 1);
    assert_eq!(n.count_alive(), 26);
}

#[test]
fn moore_r2_has_124_neighbors() {
    let mut n = runtime_neighborhood(NeighborhoodShape::Moore, 2, NeighborhoodFalloff::Uniform);
    n.fill(0, [0, 0, 0], |_, _, _| 1);
    assert_eq!(n.count_alive(), 124);
}

#[test]
fn moore_r3_has_342_neighbors() {
    let mut n = runtime_neighborhood(NeighborhoodShape::Moore, 3, NeighborhoodFalloff::Uniform);
    n.fill(0, [0, 0, 0], |_, _, _| 1);
    assert_eq!(n.count_alive(), 342);
}

#[test]
fn moore_symmetry_opposite_offsets_are_different() {
    let mut n = runtime_neighborhood(NeighborhoodShape::Moore, 1, NeighborhoodFalloff::Uniform);
    n.fill(0, [0, 0, 0], |dx, dy, dz| {
        ((dx + 2) * 100 + (dy + 2) * 10 + (dz + 2)) as u32
    });
    assert_ne!(n.get(-1, -1, -1), n.get(1, 1, 1));
    assert_ne!(n.get(-1, 0, 0), n.get(1, 0, 0));
}

#[test]
#[cfg(debug_assertions)]
#[should_panic]
fn moore_get_center_panics_in_debug() {
    let n = filled_moore(0);
    n.get(0, 0, 0);
}

#[test]
#[cfg(not(debug_assertions))]
fn moore_get_center_returns_default_in_release() {
    let n = filled_moore(0);
    assert_eq!(n.get(0, 0, 0), 0);
}

// ---------------------------------------------------------------------------
// Von Neumann
// ---------------------------------------------------------------------------

#[test]
fn vn_r1_has_6_neighbors() {
    let mut n = runtime_neighborhood(
        NeighborhoodShape::VonNeumann,
        1,
        NeighborhoodFalloff::Uniform,
    );
    n.fill(0, [0, 0, 0], |_, _, _| 1);
    assert_eq!(n.count_alive(), 6);
}

#[test]
fn vn_r2_has_24_neighbors() {
    let mut n = runtime_neighborhood(
        NeighborhoodShape::VonNeumann,
        2,
        NeighborhoodFalloff::Uniform,
    );
    n.fill(0, [0, 0, 0], |_, _, _| 1);
    assert_eq!(n.count_alive(), 24);
}

#[test]
fn vn_r3_has_62_neighbors() {
    let mut n = runtime_neighborhood(
        NeighborhoodShape::VonNeumann,
        3,
        NeighborhoodFalloff::Uniform,
    );
    n.fill(0, [0, 0, 0], |_, _, _| 1);
    assert_eq!(n.count_alive(), 62);
}

#[test]
fn vn_r1_only_face_adjacent() {
    let mut n = runtime_neighborhood(
        NeighborhoodShape::VonNeumann,
        1,
        NeighborhoodFalloff::Uniform,
    );
    n.fill(0, [0, 0, 0], |_, _, _| 1);
    assert_eq!(n.get(1, 0, 0), 1);
    assert_eq!(n.get(-1, 0, 0), 1);
    assert_eq!(n.get(0, 1, 0), 1);
    assert_eq!(n.get(0, -1, 0), 1);
    assert_eq!(n.get(0, 0, 1), 1);
    assert_eq!(n.get(0, 0, -1), 1);
}

#[test]
#[cfg(debug_assertions)]
#[should_panic]
fn vn_r1_diagonal_panics_in_debug() {
    let mut n = runtime_neighborhood(
        NeighborhoodShape::VonNeumann,
        1,
        NeighborhoodFalloff::Uniform,
    );
    n.fill(0, [0, 0, 0], |_, _, _| 1);
    n.get(1, 1, 0);
}

#[test]
#[cfg(not(debug_assertions))]
fn vn_r1_diagonal_returns_default_in_release() {
    let mut n = runtime_neighborhood(
        NeighborhoodShape::VonNeumann,
        1,
        NeighborhoodFalloff::Uniform,
    );
    n.fill(0, [0, 0, 0], |_, _, _| 1);
    assert_eq!(n.get(1, 1, 0), 0);
}

// ---------------------------------------------------------------------------
// Spherical
// ---------------------------------------------------------------------------

#[test]
fn spherical_r1_has_6_neighbors() {
    let mut n = runtime_neighborhood(
        NeighborhoodShape::Spherical,
        1,
        NeighborhoodFalloff::Uniform,
    );
    n.fill(0, [0, 0, 0], |_, _, _| 1);
    assert_eq!(n.count_alive(), 6);
}

#[test]
fn spherical_r2_has_32_neighbors() {
    let mut n = runtime_neighborhood(
        NeighborhoodShape::Spherical,
        2,
        NeighborhoodFalloff::Uniform,
    );
    n.fill(0, [0, 0, 0], |_, _, _| 1);
    assert_eq!(n.count_alive(), 32);
}

#[test]
fn spherical_r3_has_122_neighbors() {
    let mut n = runtime_neighborhood(
        NeighborhoodShape::Spherical,
        3,
        NeighborhoodFalloff::Uniform,
    );
    n.fill(0, [0, 0, 0], |_, _, _| 1);
    assert_eq!(n.count_alive(), 122);
}

#[test]
fn spherical_r2_includes_face_and_edge_but_not_corner() {
    let mut n = runtime_neighborhood(
        NeighborhoodShape::Spherical,
        2,
        NeighborhoodFalloff::Uniform,
    );
    n.fill(0, [0, 0, 0], |_, _, _| 1);
    assert_eq!(n.get(2, 0, 0), 1);
    assert_eq!(n.get(1, 1, 0), 1);
    assert_eq!(n.get(1, 1, 1), 1);
}

// ---------------------------------------------------------------------------
// Weighted
// ---------------------------------------------------------------------------

#[test]
fn unweighted_sum_equals_count_alive() {
    let mut n = runtime_neighborhood(NeighborhoodShape::Moore, 1, NeighborhoodFalloff::Uniform);
    n.fill(0, [0, 0, 0], |_, _, _| 1);
    assert_eq!(n.weighted_sum(), 26 * WEIGHT_SCALE as u64);
    assert_eq!(n.count_alive(), 26);
}

#[test]
fn inverse_square_weighted_sum() {
    let mut n = runtime_neighborhood(
        NeighborhoodShape::Moore,
        1,
        NeighborhoodFalloff::InverseSquare,
    );
    n.fill(0, [0, 0, 0], |_, _, _| 1);
    assert_eq!(n.weighted_sum(), 15_016);
}

#[test]
fn weighted_sum_excludes_dead_cells() {
    let mut n = runtime_neighborhood(
        NeighborhoodShape::Moore,
        1,
        NeighborhoodFalloff::InverseSquare,
    );
    n.fill(0, [0, 0, 0], |dx, _dy, _dz| if dx > 0 { 1 } else { 0 });
    assert!(n.weighted_sum() < 15_016);
    assert!(n.weighted_sum() > 0);
}

// ---------------------------------------------------------------------------
// iter() and neighbor_count()
// ---------------------------------------------------------------------------

#[test]
fn iter_returns_all_entries() {
    let mut n = runtime_neighborhood(
        NeighborhoodShape::VonNeumann,
        1,
        NeighborhoodFalloff::Uniform,
    );
    n.fill(0, [0, 0, 0], |_, _, _| 1);
    assert_eq!(n.iter().len(), 6);
    assert_eq!(n.neighbor_count(), 6);
}

#[test]
fn iter_entries_have_correct_offsets() {
    let mut n = runtime_neighborhood(
        NeighborhoodShape::VonNeumann,
        1,
        NeighborhoodFalloff::Uniform,
    );
    n.fill(0, [0, 0, 0], |dx, dy, dz| {
        (dx.abs() + dy.abs() + dz.abs()) as u32
    });
    for entry in n.iter() {
        let manhattan = entry.offset.dx.abs() + entry.offset.dy.abs() + entry.offset.dz.abs();
        assert_eq!(manhattan, 1);
        assert_eq!(entry.cell, 1);
    }
}
