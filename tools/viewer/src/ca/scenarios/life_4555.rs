//! Classic wrapped 3D Life 4555 scenario.

use hyle_ca_interface::{
    neighbors, Blueprint, BuildError, CaRuntime, NeighborhoodFalloff, NeighborhoodRadius,
    NeighborhoodSet, NeighborhoodShape, NeighborhoodSpec, RuleSpec, TopologyDescriptor,
};

use super::shared::{seed_random_box, ViewerCell};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Neighborhoods {
    Adjacent,
}

impl NeighborhoodSet for Neighborhoods {
    fn variants() -> &'static [Self] {
        &[Neighborhoods::Adjacent]
    }

    fn label(self) -> &'static str {
        "adjacent"
    }
}

pub(super) fn blueprint() -> Result<Blueprint, BuildError> {
    Blueprint::builder()
        .materials::<ViewerCell>()
        .topology(TopologyDescriptor::wrap())
        .neighborhoods::<Neighborhoods>()
        .neighborhood_specs([NeighborhoodSpec::new(
            Neighborhoods::Adjacent,
            NeighborhoodShape::Moore,
            NeighborhoodRadius::new(1),
            NeighborhoodFalloff::Uniform,
        )?])
        .rules([
            RuleSpec::when(ViewerCell::Alive)
                .require(
                    neighbors(ViewerCell::Alive)
                        .count()
                        .in_range(4..=5)
                        .negate(),
                )
                .becomes(ViewerCell::Dead),
            RuleSpec::when(ViewerCell::Dead)
                .require(neighbors(ViewerCell::Alive).count().eq(5))
                .becomes(ViewerCell::Alive),
        ])
        .build()
}

pub(super) fn seed(ca: &mut impl CaRuntime) -> Result<(), hyle_ca_interface::SetContractError> {
    seed_random_box(ca, 24..40, 24..40, 24..40, ViewerCell::Alive, 6, 11, 0)
}
