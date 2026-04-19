//! Grass, fire, ember, ash, and stone ecology.

use hyle_ca_interface::{
    neighbors, rng, Blueprint, BuildError, CaRuntime, NeighborhoodFalloff, NeighborhoodRadius,
    NeighborhoodSet, NeighborhoodShape, NeighborhoodSpec, RuleSpec, SetContractError, Weight,
};

use super::shared::{fill_box, fill_sphere, seed_random_box, ViewerCell};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Neighborhoods {
    Adjacent,
    EmberField,
}

impl NeighborhoodSet for Neighborhoods {
    fn variants() -> &'static [Self] {
        &[Neighborhoods::Adjacent, Neighborhoods::EmberField]
    }

    fn label(self) -> &'static str {
        match self {
            Neighborhoods::Adjacent => "adjacent",
            Neighborhoods::EmberField => "ember_field",
        }
    }
}

pub(super) fn blueprint() -> Result<Blueprint, BuildError> {
    Blueprint::builder()
        .materials::<ViewerCell>()
        .neighborhoods::<Neighborhoods>()
        .neighborhood_specs([
            NeighborhoodSpec::new(
                Neighborhoods::Adjacent,
                NeighborhoodShape::Moore,
                NeighborhoodRadius::new(1),
                NeighborhoodFalloff::Uniform,
            )?,
            NeighborhoodSpec::new(
                Neighborhoods::EmberField,
                NeighborhoodShape::Spherical,
                NeighborhoodRadius::new(2),
                NeighborhoodFalloff::InverseSquare,
            )?,
        ])
        .rules([
            RuleSpec::when(ViewerCell::Stone).keep(),
            RuleSpec::when(ViewerCell::Fire).becomes(ViewerCell::Ember),
            RuleSpec::when(ViewerCell::Ember)
                .require(rng(1).one_in(2))
                .becomes(ViewerCell::Ash),
            RuleSpec::when(ViewerCell::Grass)
                .require(neighbors(ViewerCell::Fire).any())
                .becomes(ViewerCell::Fire),
            RuleSpec::when(ViewerCell::Grass)
                .using(Neighborhoods::EmberField)
                .require(
                    neighbors(ViewerCell::Ember)
                        .weighted_sum()
                        .at_least(Weight::cells(1)),
                )
                .require(rng(2).one_in(2))
                .becomes(ViewerCell::Fire),
            RuleSpec::when(ViewerCell::Ash)
                .require(neighbors(ViewerCell::Grass).count().at_least(2))
                .require(rng(3).one_in(6))
                .becomes(ViewerCell::Grass),
            RuleSpec::when(ViewerCell::Dead)
                .require(neighbors(ViewerCell::Grass).count().at_least(3))
                .require(neighbors(ViewerCell::Fire).count().eq(0))
                .require(rng(4).one_in(8))
                .becomes(ViewerCell::Grass),
        ])
        .build()
}

pub(super) fn seed(ca: &mut impl CaRuntime) -> Result<(), SetContractError> {
    fill_box(ca, 8..56, 6..30, 8..56, ViewerCell::Grass)?;
    fill_sphere(ca, [20, 16, 20], 4, ViewerCell::Stone)?;
    fill_sphere(ca, [42, 14, 38], 3, ViewerCell::Stone)?;
    fill_sphere(ca, [28, 18, 44], 3, ViewerCell::Stone)?;
    fill_sphere(ca, [16, 12, 16], 2, ViewerCell::Fire)?;
    fill_sphere(ca, [48, 14, 44], 2, ViewerCell::Fire)?;
    seed_random_box(ca, 10..54, 6..24, 10..54, ViewerCell::Grass, 18, 29, 3)
}
