//! Weighted inverse-square growth scenario.

use hyle_ca_interface::{
    neighbors, rng, Blueprint, BuildError, CaRuntime, NeighborhoodFalloff, NeighborhoodRadius,
    NeighborhoodSet, NeighborhoodShape, NeighborhoodSpec, RuleSpec, SetContractError, Weight,
};

use super::shared::{fill_sphere, seed_random_box, ViewerCell};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Neighborhoods {
    Field,
}

impl NeighborhoodSet for Neighborhoods {
    fn variants() -> &'static [Self] {
        &[Neighborhoods::Field]
    }

    fn label(self) -> &'static str {
        "field"
    }
}

pub(super) fn blueprint() -> Result<Blueprint, BuildError> {
    Blueprint::builder()
        .materials::<ViewerCell>()
        .neighborhoods::<Neighborhoods>()
        .neighborhood_specs([NeighborhoodSpec::new(
            Neighborhoods::Field,
            NeighborhoodShape::Spherical,
            NeighborhoodRadius::new(4),
            NeighborhoodFalloff::InverseSquare,
        )?])
        .rules([
            RuleSpec::when(ViewerCell::Bloom)
                .require(
                    neighbors(ViewerCell::Bloom)
                        .weighted_sum()
                        .in_range(Weight::cells(2)..=Weight::cells(7))
                        .negate(),
                )
                .becomes(ViewerCell::Dead),
            RuleSpec::when(ViewerCell::Dead)
                .require(
                    neighbors(ViewerCell::Bloom)
                        .weighted_sum()
                        .in_range(Weight::cells(3)..=Weight::cells(5)),
                )
                .require(rng(0).one_in(3))
                .becomes(ViewerCell::Bloom),
            RuleSpec::when(ViewerCell::Dead)
                .require(neighbors(ViewerCell::Bloom).count().eq(4))
                .becomes(ViewerCell::Bloom),
        ])
        .build()
}

pub(super) fn seed(ca: &mut impl CaRuntime) -> Result<(), SetContractError> {
    fill_sphere(ca, [20, 22, 20], 3, ViewerCell::Bloom)?;
    fill_sphere(ca, [42, 30, 40], 3, ViewerCell::Bloom)?;
    fill_sphere(ca, [30, 44, 28], 2, ViewerCell::Bloom)?;
    seed_random_box(ca, 16..48, 16..48, 16..48, ViewerCell::Bloom, 28, 19, 1)
}
