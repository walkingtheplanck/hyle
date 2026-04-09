//! Tests for topology index mapping.

use hyle_ca_core::{BoundedTopology, Topology, TorusTopology};

#[test]
fn bounded_maps_out_of_bounds_to_guard_index() {
    let guard = 4 * 5 * 6;
    assert_eq!(
        BoundedTopology.resolve_index(-1, 0, 0, 4, 5, 6, guard),
        guard
    );
    assert_eq!(
        BoundedTopology.resolve_index(4, 0, 0, 4, 5, 6, guard),
        guard
    );
    assert_eq!(BoundedTopology.resolve_index(2, 0, 0, 4, 5, 6, guard), 2);
}

#[test]
fn torus_wraps_coordinates_on_both_sides() {
    let guard = 4 * 5 * 6;
    assert_eq!(TorusTopology.resolve_index(-1, 0, 0, 4, 5, 6, guard), 3);
    assert_eq!(TorusTopology.resolve_index(4, 0, 0, 4, 5, 6, guard), 0);
    assert_eq!(TorusTopology.resolve_index(5, 0, 0, 4, 5, 6, guard), 1);
}

#[test]
fn zero_sized_axes_cannot_be_addressed() {
    assert_eq!(BoundedTopology.resolve_index(0, 0, 0, 0, 4, 4, 0), 0);
    assert_eq!(TorusTopology.resolve_index(0, 0, 0, 0, 4, 4, 0), 0);
}
