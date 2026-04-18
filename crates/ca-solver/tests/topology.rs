//! Tests for topology index mapping.

use hyle_ca_interface::{AxisTopology, GridDims, GridShapeError, Topology, TopologyDescriptor};
use hyle_ca_solver::{BoundedTopology, DescriptorTopology, TorusTopology};

#[test]
fn bounded_maps_out_of_bounds_to_guard_index() -> Result<(), GridShapeError> {
    let guard = 4 * 5 * 6;
    assert_eq!(
        BoundedTopology.resolve_index(-1, 0, 0, GridDims::new(4, 5, 6)?, guard),
        guard
    );
    assert_eq!(
        BoundedTopology.resolve_index(4, 0, 0, GridDims::new(4, 5, 6)?, guard),
        guard
    );
    assert_eq!(
        BoundedTopology.resolve_index(2, 0, 0, GridDims::new(4, 5, 6)?, guard),
        2
    );
    Ok(())
}

#[test]
fn torus_wraps_coordinates_on_both_sides() -> Result<(), GridShapeError> {
    let guard = 4 * 5 * 6;
    assert_eq!(
        TorusTopology.resolve_index(-1, 0, 0, GridDims::new(4, 5, 6)?, guard),
        3
    );
    assert_eq!(
        TorusTopology.resolve_index(4, 0, 0, GridDims::new(4, 5, 6)?, guard),
        0
    );
    assert_eq!(
        TorusTopology.resolve_index(5, 0, 0, GridDims::new(4, 5, 6)?, guard),
        1
    );
    Ok(())
}

#[test]
fn zero_sized_axes_cannot_be_addressed() -> Result<(), GridShapeError> {
    assert_eq!(
        BoundedTopology.resolve_index(0, 0, 0, GridDims::new(0, 4, 4)?, 0),
        0
    );
    assert_eq!(
        TorusTopology.resolve_index(0, 0, 0, GridDims::new(0, 4, 4)?, 0),
        0
    );
    Ok(())
}

#[test]
fn built_in_topologies_expose_uploadable_descriptors() {
    assert_eq!(
        BoundedTopology.descriptor(),
        TopologyDescriptor::uniform(AxisTopology::Bounded)
    );
    assert_eq!(
        TorusTopology.descriptor(),
        TopologyDescriptor::uniform(AxisTopology::Wrap)
    );
}

#[test]
fn descriptor_topology_supports_mixed_axis_behavior() -> Result<(), GridShapeError> {
    let descriptor = TopologyDescriptor::by_axis(
        AxisTopology::Wrap,
        AxisTopology::Bounded,
        AxisTopology::Wrap,
    );
    let topology = DescriptorTopology::new(descriptor);
    let guard = 4 * 5 * 6;

    assert_eq!(
        topology.resolve_index(-1, 0, -1, GridDims::new(4, 5, 6)?, guard),
        103
    );
    assert_eq!(
        topology.resolve_index(0, 5, 0, GridDims::new(4, 5, 6)?, guard),
        guard
    );
    Ok(())
}
