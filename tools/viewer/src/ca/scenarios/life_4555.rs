//! Classic wrapped 3D Life 4555 scenario.

use hyle_ca_interface::{neighbors, Blueprint, CaRuntime, TopologyDescriptor};

use super::shared::{seed_random_box, ViewerCell};

pub(super) fn blueprint() -> Blueprint<ViewerCell> {
    Blueprint::<ViewerCell>::builder()
        .topology(TopologyDescriptor::wrap())
        .rules(|rules| {
            rules
                .when(ViewerCell::Alive)
                .unless(neighbors(ViewerCell::Alive).count().in_range(4..=5))
                .becomes(ViewerCell::Dead);
            rules
                .when(ViewerCell::Dead)
                .require(neighbors(ViewerCell::Alive).count().eq(5))
                .becomes(ViewerCell::Alive);
        })
        .build()
        .expect("life blueprint should build")
}

pub(super) fn seed(ca: &mut impl CaRuntime<ViewerCell>) {
    seed_random_box(ca, 24..40, 24..40, 24..40, ViewerCell::Alive, 6, 11, 0);
}
