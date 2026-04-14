//! Crystal growth and heat spread scenario.

use hyle_ca_interface::{
    neighbors, rng, Blueprint, CaRuntime, NeighborhoodFalloff, NeighborhoodShape, NeighborhoodSpec,
    Weight,
};

use super::shared::{fill_sphere, seed_random_box, ViewerCell};

pub(super) fn blueprint() -> Blueprint<ViewerCell> {
    Blueprint::<ViewerCell>::builder()
        .neighborhood(
            "heat",
            NeighborhoodSpec::new(
                NeighborhoodShape::Spherical,
                3,
                NeighborhoodFalloff::InverseSquare,
            ),
        )
        .rules(|rules| {
            rules
                .when(ViewerCell::Hot)
                .using("heat")
                .unless(
                    neighbors(ViewerCell::Hot)
                        .weighted_sum()
                        .at_least(Weight::cells(2)),
                )
                .becomes(ViewerCell::Crystal);
            rules
                .when(ViewerCell::Crystal)
                .using("heat")
                .require(
                    neighbors(ViewerCell::Hot)
                        .weighted_sum()
                        .at_least(Weight::cells(2)),
                )
                .becomes(ViewerCell::Hot);
            rules
                .when(ViewerCell::Dead)
                .require(neighbors(ViewerCell::Crystal).count().in_range(2..=3))
                .require(rng(1).one_in(4))
                .becomes(ViewerCell::Crystal);
            rules
                .when(ViewerCell::Dead)
                .using("heat")
                .require(
                    neighbors(ViewerCell::Hot)
                        .weighted_sum()
                        .at_least(Weight::cells(3)),
                )
                .becomes(ViewerCell::Hot);
        })
        .build()
        .expect("crystal forge blueprint should build")
}

pub(super) fn seed(ca: &mut impl CaRuntime<ViewerCell>) {
    fill_sphere(ca, [32, 32, 32], 3, ViewerCell::Crystal);
    fill_sphere(ca, [32, 32, 32], 1, ViewerCell::Hot);
    fill_sphere(ca, [20, 24, 20], 2, ViewerCell::Crystal);
    fill_sphere(ca, [44, 40, 44], 2, ViewerCell::Crystal);
    seed_random_box(ca, 18..46, 18..46, 18..46, ViewerCell::Crystal, 32, 23, 2);
}
