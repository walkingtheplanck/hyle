//! Weighted inverse-square growth scenario.

use hyle_ca_interface::{
    neighbors, rng, Blueprint, CaRuntime, NeighborhoodFalloff, NeighborhoodShape, NeighborhoodSpec,
    Weight,
};

use super::shared::{fill_sphere, seed_random_box, ViewerCell};

pub(super) fn blueprint() -> Blueprint<ViewerCell> {
    Blueprint::<ViewerCell>::builder()
        .neighborhood(
            "field",
            NeighborhoodSpec::new(
                NeighborhoodShape::Spherical,
                4,
                NeighborhoodFalloff::InverseSquare,
            ),
        )
        .rules(|rules| {
            rules
                .when(ViewerCell::Bloom)
                .using("field")
                .unless(
                    neighbors(ViewerCell::Bloom)
                        .weighted_sum()
                        .in_range(Weight::cells(2)..=Weight::cells(7)),
                )
                .becomes(ViewerCell::Dead);
            rules
                .when(ViewerCell::Dead)
                .using("field")
                .require(
                    neighbors(ViewerCell::Bloom)
                        .weighted_sum()
                        .in_range(Weight::cells(3)..=Weight::cells(5)),
                )
                .require(rng(0).one_in(3))
                .becomes(ViewerCell::Bloom);
            rules
                .when(ViewerCell::Dead)
                .require(neighbors(ViewerCell::Bloom).count().eq(4))
                .becomes(ViewerCell::Bloom);
        })
        .build()
        .expect("weighted bloom blueprint should build")
}

pub(super) fn seed(ca: &mut impl CaRuntime<ViewerCell>) {
    fill_sphere(ca, [20, 22, 20], 3, ViewerCell::Bloom);
    fill_sphere(ca, [42, 30, 40], 3, ViewerCell::Bloom);
    fill_sphere(ca, [30, 44, 28], 2, ViewerCell::Bloom);
    seed_random_box(ca, 16..48, 16..48, 16..48, ViewerCell::Bloom, 28, 19, 1);
}
