//! Tests for the declarative blueprint builder.

use hyle_ca_interface::{
    neighbors, rng, BuildError, CellModel, CellSchema, Condition, Hyle, NeighborhoodFalloff,
    NeighborhoodShape, NeighborhoodSpec, StateDef, TopologyDescriptor,
};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
enum LifeCell {
    #[default]
    Dead,
    Alive,
}

const LIFE_CELL_STATES: [StateDef; 2] = [StateDef::new("Dead"), StateDef::new("Alive")];

impl CellModel for LifeCell {
    fn schema() -> CellSchema {
        CellSchema::enumeration("LifeCell", &LIFE_CELL_STATES)
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
    assert_eq!(
        spec.neighborhoods()[0].spec.shape(),
        NeighborhoodShape::Moore
    );
    assert_eq!(spec.neighborhoods()[0].spec.radius(), 1);
    assert_eq!(
        spec.neighborhoods()[0].spec.falloff(),
        NeighborhoodFalloff::Uniform
    );
    assert_eq!(spec.rules().len(), 1);
    assert_eq!(spec.cell_schema().state_count(), Some(2));
}

#[test]
fn builder_resolves_named_neighborhoods() {
    let spec = Hyle::builder()
        .cells::<LifeCell>()
        .neighborhood(
            "far",
            NeighborhoodSpec::new(NeighborhoodShape::Moore, 2, NeighborhoodFalloff::Uniform),
        )
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
        .neighborhood(
            "adjacent",
            NeighborhoodSpec::new(NeighborhoodShape::Moore, 2, NeighborhoodFalloff::Uniform),
        )
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

#[test]
fn builder_accepts_plain_contract_cell_types() {
    let spec = Hyle::builder()
        .cells::<LifeCell>()
        .rules(|rules| {
            rules.when(LifeCell::Alive).becomes(LifeCell::Dead);
        })
        .build()
        .expect("plain blueprint cell types should build");

    assert_eq!(spec.rules().len(), 1);
}

#[test]
fn builder_emits_random_chance_conditions() {
    let spec = Hyle::builder()
        .cells::<LifeCell>()
        .rules(|rules| {
            rules
                .when(LifeCell::Dead)
                .require(rng(7).one_in(3))
                .becomes(LifeCell::Alive);
        })
        .build()
        .expect("valid spec");

    assert_eq!(
        spec.rules()[0].condition,
        Some(Condition::RandomChance {
            stream: 7,
            one_in: 3,
        })
    );
}
