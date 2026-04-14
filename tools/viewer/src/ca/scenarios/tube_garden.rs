//! Mixed-axis topology with simple flow-like growth around walls.

use hyle_ca_interface::{
    neighbors, rng, AxisTopology, Blueprint, CaRuntime, NeighborhoodFalloff, NeighborhoodShape,
    NeighborhoodSpec, TopologyDescriptor,
};

use super::shared::{seed_random_box, ViewerCell};

pub(super) fn blueprint() -> Blueprint<ViewerCell> {
    Blueprint::<ViewerCell>::builder()
        .topology(TopologyDescriptor::by_axis(
            AxisTopology::Wrap,
            AxisTopology::Bounded,
            AxisTopology::Wrap,
        ))
        .neighborhood(
            "flow",
            NeighborhoodSpec::new(
                NeighborhoodShape::VonNeumann,
                1,
                NeighborhoodFalloff::Uniform,
            ),
        )
        .neighborhood("support", NeighborhoodSpec::adjacent())
        .default_neighborhood("flow")
        .rules(|rules| {
            rules.when(ViewerCell::Wall).keep();
            rules
                .when(ViewerCell::Alive)
                .using("support")
                .require(neighbors(ViewerCell::Wall).count().eq(0))
                .becomes(ViewerCell::Dead);
            rules
                .when(ViewerCell::Alive)
                .unless(neighbors(ViewerCell::Alive).count().in_range(1..=2))
                .becomes(ViewerCell::Dead);
            rules
                .when(ViewerCell::Dead)
                .using("support")
                .require(neighbors(ViewerCell::Wall).count().at_least(3))
                .require(rng(5).one_in(5))
                .becomes(ViewerCell::Alive);
            rules
                .when(ViewerCell::Dead)
                .require(neighbors(ViewerCell::Alive).count().eq(2))
                .becomes(ViewerCell::Alive);
        })
        .build()
        .expect("tube garden blueprint should build")
}

pub(super) fn seed(ca: &mut impl CaRuntime<ViewerCell>) {
    for x in (8..56).step_by(16) {
        for z in (8..56).step_by(16) {
            for y in 6..58 {
                ca.set(x, y, z, ViewerCell::Wall);
            }
        }
    }

    seed_random_box(ca, 6..58, 10..54, 6..58, ViewerCell::Alive, 20, 31, 4);
}
