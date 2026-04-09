//! Tests for topology coordinate mapping.

use hyle_ca_core::{BoundedTopology, Topology, TorusTopology};

#[test]
fn bounded_rejects_out_of_bounds_coordinates() {
    assert_eq!(BoundedTopology.map_coord(-1, 4), None);
    assert_eq!(BoundedTopology.map_coord(4, 4), None);
    assert_eq!(BoundedTopology.map_coord(2, 4), Some(2));
}

#[test]
fn torus_wraps_coordinates_on_both_sides() {
    assert_eq!(TorusTopology.map_coord(-1, 4), Some(3));
    assert_eq!(TorusTopology.map_coord(4, 4), Some(0));
    assert_eq!(TorusTopology.map_coord(5, 4), Some(1));
}

#[test]
fn zero_sized_axes_cannot_be_addressed() {
    assert_eq!(BoundedTopology.map_coord(0, 0), None);
    assert_eq!(TorusTopology.map_coord(0, 0), None);
}
