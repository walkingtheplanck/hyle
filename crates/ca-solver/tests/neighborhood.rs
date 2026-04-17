//! Tests for Neighborhood struct, shapes, and weights.

use hyle_ca_interface::resolved::expand_neighborhood;
use hyle_ca_interface::resolved::WEIGHT_SCALE;
use hyle_ca_interface::{
    MaterialId, NeighborhoodFalloff, NeighborhoodRadius, NeighborhoodSet, NeighborhoodShape,
    NeighborhoodSpec,
};
use hyle_ca_solver::Neighborhood;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum N {
    Sample,
}

impl NeighborhoodSet for N {
    fn variants() -> &'static [Self] {
        &[N::Sample]
    }

    fn label(self) -> &'static str {
        "sample"
    }
}

fn runtime_neighborhood(
    shape: NeighborhoodShape,
    radius: u32,
    falloff: NeighborhoodFalloff,
) -> Neighborhood {
    let semantic = expand_neighborhood(NeighborhoodSpec::new(
        N::Sample,
        shape,
        NeighborhoodRadius::new(radius),
        falloff,
    ));
    Neighborhood::new(semantic.samples())
}

#[test]
fn moore_center_and_position_are_stored() {
    let mut n = runtime_neighborhood(NeighborhoodShape::Moore, 1, NeighborhoodFalloff::Uniform);
    n.fill(MaterialId::new(42), [5, 5, 5], |dx, dy, dz| {
        MaterialId::new(((dx + 1) + (dy + 1) * 3 + (dz + 1) * 9) as u16)
    });
    assert_eq!(n.center(), MaterialId::new(42));
    assert_eq!(n.pos(), [5, 5, 5]);
    assert_eq!(n.get(1, 1, 1), MaterialId::new(26));
}

#[test]
fn moore_and_von_neumann_neighbor_counts_match() {
    let mut moore = runtime_neighborhood(NeighborhoodShape::Moore, 2, NeighborhoodFalloff::Uniform);
    moore.fill(MaterialId::new(0), [0, 0, 0], |_, _, _| MaterialId::new(1));
    assert_eq!(moore.count(|entry| entry.cell == MaterialId::new(1)), 124);

    let mut vn = runtime_neighborhood(
        NeighborhoodShape::VonNeumann,
        3,
        NeighborhoodFalloff::Uniform,
    );
    vn.fill(MaterialId::new(0), [0, 0, 0], |_, _, _| MaterialId::new(1));
    assert_eq!(vn.count(|entry| entry.cell == MaterialId::new(1)), 62);
}

#[test]
fn weighted_sum_uses_precomputed_fixed_point_weights() {
    let mut n = runtime_neighborhood(
        NeighborhoodShape::VonNeumann,
        1,
        NeighborhoodFalloff::InverseSquare,
    );
    n.fill(MaterialId::new(0), [0, 0, 0], |_, _, _| MaterialId::new(1));
    assert_eq!(
        n.weighted_sum(|entry| entry.cell == MaterialId::new(1)),
        6 * WEIGHT_SCALE as u64
    );
}
