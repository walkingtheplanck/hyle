//! Crystal growth and heat spread scenario.

use hyle_ca_interface::{
    neighbors, rng, Blueprint, CaRuntime, NeighborhoodFalloff, NeighborhoodRadius,
    NeighborhoodSet, NeighborhoodShape, NeighborhoodSpec, RuleSpec, Weight,
};

use super::shared::{fill_sphere, seed_random_box, ViewerCell};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Neighborhoods {
    Adjacent,
    Heat,
}

impl NeighborhoodSet for Neighborhoods {
    fn variants() -> &'static [Self] {
        &[Neighborhoods::Adjacent, Neighborhoods::Heat]
    }

    fn label(self) -> &'static str {
        match self {
            Neighborhoods::Adjacent => "adjacent",
            Neighborhoods::Heat => "heat",
        }
    }
}

pub(super) fn blueprint() -> Blueprint {
    Blueprint::builder()
        .materials::<ViewerCell>()
        .neighborhoods::<Neighborhoods>()
        .neighborhood_specs([
            NeighborhoodSpec::new(
                Neighborhoods::Adjacent,
                NeighborhoodShape::Moore,
                NeighborhoodRadius::new(1),
                NeighborhoodFalloff::Uniform,
            ),
            NeighborhoodSpec::new(
                Neighborhoods::Heat,
                NeighborhoodShape::Spherical,
                NeighborhoodRadius::new(3),
                NeighborhoodFalloff::InverseSquare,
            ),
        ])
        .rules([
            RuleSpec::when(ViewerCell::Hot)
                .using(Neighborhoods::Heat)
                .require(
                    neighbors(ViewerCell::Hot)
                        .weighted_sum()
                        .at_least(Weight::cells(2))
                        .negate(),
                )
                .becomes(ViewerCell::Crystal),
            RuleSpec::when(ViewerCell::Crystal)
                .using(Neighborhoods::Heat)
                .require(
                    neighbors(ViewerCell::Hot)
                        .weighted_sum()
                        .at_least(Weight::cells(2)),
                )
                .becomes(ViewerCell::Hot),
            RuleSpec::when(ViewerCell::Dead)
                .require(neighbors(ViewerCell::Crystal).count().in_range(2..=3))
                .require(rng(1).one_in(4))
                .becomes(ViewerCell::Crystal),
            RuleSpec::when(ViewerCell::Dead)
                .using(Neighborhoods::Heat)
                .require(
                    neighbors(ViewerCell::Hot)
                        .weighted_sum()
                        .at_least(Weight::cells(3)),
                )
                .becomes(ViewerCell::Hot),
        ])
        .build()
        .expect("crystal forge blueprint should build")
}

pub(super) fn seed(ca: &mut impl CaRuntime) {
    fill_sphere(ca, [32, 32, 32], 3, ViewerCell::Crystal);
    fill_sphere(ca, [32, 32, 32], 1, ViewerCell::Hot);
    fill_sphere(ca, [20, 24, 20], 2, ViewerCell::Crystal);
    fill_sphere(ca, [44, 40, 44], 2, ViewerCell::Crystal);
    seed_random_box(ca, 18..46, 18..46, 18..46, ViewerCell::Crystal, 32, 23, 2);
}
