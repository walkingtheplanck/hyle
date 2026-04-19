//! Mixed-axis topology with simple flow-like growth around walls.

use hyle_ca_interface::{
    neighbors, rng, AxisTopology, Blueprint, BuildError, CaRuntime, MaterialSet,
    NeighborhoodFalloff, NeighborhoodRadius, NeighborhoodSet, NeighborhoodShape, NeighborhoodSpec,
    RuleSpec, SetContractError, TopologyDescriptor,
};

use super::shared::{seed_random_box, ViewerCell};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Neighborhoods {
    Flow,
    Support,
}

impl NeighborhoodSet for Neighborhoods {
    fn variants() -> &'static [Self] {
        &[Neighborhoods::Flow, Neighborhoods::Support]
    }

    fn label(self) -> &'static str {
        match self {
            Neighborhoods::Flow => "flow",
            Neighborhoods::Support => "support",
        }
    }
}

pub(super) fn blueprint() -> Result<Blueprint, BuildError> {
    Blueprint::builder()
        .materials::<ViewerCell>()
        .topology(TopologyDescriptor::by_axis(
            AxisTopology::Wrap,
            AxisTopology::Bounded,
            AxisTopology::Wrap,
        ))
        .neighborhoods::<Neighborhoods>()
        .default_neighborhood(Neighborhoods::Flow)
        .neighborhood_specs([
            NeighborhoodSpec::new(
                Neighborhoods::Flow,
                NeighborhoodShape::VonNeumann,
                NeighborhoodRadius::new(1),
                NeighborhoodFalloff::Uniform,
            )?,
            NeighborhoodSpec::new(
                Neighborhoods::Support,
                NeighborhoodShape::Moore,
                NeighborhoodRadius::new(1),
                NeighborhoodFalloff::Uniform,
            )?,
        ])
        .rules([
            RuleSpec::when(ViewerCell::Wall).keep(),
            RuleSpec::when(ViewerCell::Alive)
                .using(Neighborhoods::Support)
                .require(neighbors(ViewerCell::Wall).count().eq(0))
                .becomes(ViewerCell::Dead),
            RuleSpec::when(ViewerCell::Alive)
                .require(
                    neighbors(ViewerCell::Alive)
                        .count()
                        .in_range(1..=2)
                        .negate(),
                )
                .becomes(ViewerCell::Dead),
            RuleSpec::when(ViewerCell::Dead)
                .using(Neighborhoods::Support)
                .require(neighbors(ViewerCell::Wall).count().at_least(3))
                .require(rng(5).one_in(5))
                .becomes(ViewerCell::Alive),
            RuleSpec::when(ViewerCell::Dead)
                .require(neighbors(ViewerCell::Alive).count().eq(2))
                .becomes(ViewerCell::Alive),
        ])
        .build()
}

pub(super) fn seed(ca: &mut impl CaRuntime) -> Result<(), SetContractError> {
    let wall = ViewerCell::Wall.id()?;

    for x in (8..56).step_by(16) {
        for z in (8..56).step_by(16) {
            for y in 6..58 {
                ca.set(x, y, z, wall);
            }
        }
    }

    seed_random_box(ca, 6..58, 10..54, 6..58, ViewerCell::Alive, 20, 31, 4)
}
