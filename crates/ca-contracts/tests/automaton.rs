//! Tests for the declarative automaton builder.

use hyle_ca_contracts::{neighbors, BuildError, Cell, Hyle, NeighborhoodSpec, TopologyDescriptor};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
enum LifeCell {
    #[default]
    Dead,
    Alive,
}

impl Cell for LifeCell {
    fn rule_id(&self) -> u8 {
        match self {
            Self::Dead => 0,
            Self::Alive => 1,
        }
    }

    fn is_alive(&self) -> bool {
        matches!(self, Self::Alive)
    }
}

#[test]
fn builder_emits_default_adjacent_neighborhood() {
    let spec = Hyle::builder()
        .cells::<LifeCell>()
        .rules(|rules| {
            rules
                .when(LifeCell::Dead)
                .require(neighbors(LifeCell::Alive).count().eq(3))
                .becomes(LifeCell::Alive);
        })
        .build()
        .expect("valid spec");

    assert_eq!(spec.topology(), TopologyDescriptor::bounded());
    assert_eq!(spec.default_neighborhood(), 0);
    assert_eq!(spec.neighborhoods()[0].name, "adjacent");
    assert_eq!(spec.neighborhoods()[0].spec, NeighborhoodSpec::adjacent());
    assert_eq!(spec.rules().len(), 1);
}

#[test]
fn builder_resolves_named_neighborhoods() {
    let spec = Hyle::builder()
        .cells::<LifeCell>()
        .neighborhood("far", NeighborhoodSpec::cube(2))
        .default_neighborhood("far")
        .rules(|rules| {
            rules
                .when(LifeCell::Dead)
                .require(neighbors(LifeCell::Alive).count().at_least(1))
                .becomes(LifeCell::Alive);
        })
        .build()
        .expect("valid spec");

    assert_eq!(spec.default_neighborhood(), 1);
    assert_eq!(spec.rules()[0].neighborhood, 1);
}

#[test]
fn builder_rejects_duplicate_neighborhood_names() {
    let error = Hyle::builder()
        .cells::<LifeCell>()
        .neighborhood("adjacent", NeighborhoodSpec::cube(2))
        .build()
        .expect_err("duplicate names must fail");

    assert_eq!(
        error,
        BuildError::DuplicateNeighborhood("adjacent".to_string())
    );
}

#[test]
fn builder_rejects_unknown_rule_neighborhoods() {
    let error = Hyle::builder()
        .cells::<LifeCell>()
        .rules(|rules| {
            rules
                .when(LifeCell::Dead)
                .using("missing")
                .becomes(LifeCell::Alive);
        })
        .build()
        .expect_err("unknown rule neighborhoods must fail");

    assert_eq!(
        error,
        BuildError::UnknownRuleNeighborhood("missing".to_string())
    );
}
