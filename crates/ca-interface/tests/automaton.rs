//! Tests for the declarative blueprint builder.

use hyle_ca_interface::{
    neighbors, rng, AttributeDef, AttributeType, AttributeValue, Blueprint, BuildError, CellModel,
    CellSchema, Condition, NeighborhoodFalloff, NeighborhoodShape, NeighborhoodSpec, StateDef,
    TopologyDescriptor, Weight, WeightComparison,
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
    let spec = Blueprint::<LifeCell>::builder()
        .attribute("heat", AttributeType::U8)
        .rules(|rules| {
            rules
                .when(LifeCell::Dead)
                .require(neighbors(LifeCell::Alive).count().eq(3))
                .becomes(LifeCell::Alive);
        })
        .build()
        .expect("valid spec");

    assert_eq!(spec.topology(), TopologyDescriptor::bounded());
    assert_eq!(spec.attributes().len(), 1);
    assert_eq!(
        spec.attributes()[0],
        AttributeDef::new("heat", AttributeType::U8)
    );
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
fn builder_emits_attributes_with_defaults() {
    let spec = Blueprint::<LifeCell>::builder()
        .attribute("age", AttributeType::U16)
        .attribute_with_default("charged", AttributeValue::Bool(true))
        .build()
        .expect("valid spec");

    assert_eq!(
        spec.attributes(),
        &[
            AttributeDef::new("age", AttributeType::U16),
            AttributeDef::with_default("charged", AttributeValue::Bool(true)),
        ]
    );
}

#[test]
fn builder_rejects_empty_attribute_names() {
    let error = Blueprint::<LifeCell>::builder()
        .attribute("", AttributeType::U8)
        .build()
        .expect_err("empty names must fail");

    assert_eq!(error, BuildError::EmptyAttributeName);
}

#[test]
fn builder_rejects_duplicate_attribute_names() {
    let error = Blueprint::<LifeCell>::builder()
        .attribute("heat", AttributeType::U8)
        .attribute_with_default("heat", AttributeValue::U8(3))
        .build()
        .expect_err("duplicate names must fail");

    assert_eq!(error, BuildError::DuplicateAttribute("heat".to_string()));
}

#[test]
fn builder_resolves_named_neighborhoods() {
    let spec = Blueprint::<LifeCell>::builder()
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
    let error = Blueprint::<LifeCell>::builder()
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
fn builder_rejects_empty_neighborhood_names() {
    let error = Blueprint::<LifeCell>::builder()
        .neighborhood("", NeighborhoodSpec::adjacent())
        .build()
        .expect_err("empty names must fail");

    assert_eq!(error, BuildError::EmptyNeighborhoodName);
}

#[test]
fn builder_rejects_unknown_rule_neighborhoods() {
    let error = Blueprint::<LifeCell>::builder()
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
    let spec = Blueprint::<LifeCell>::builder()
        .rules(|rules| {
            rules.when(LifeCell::Alive).becomes(LifeCell::Dead);
        })
        .build()
        .expect("plain blueprint cell types should build");

    assert_eq!(spec.rules().len(), 1);
}

#[test]
fn builder_emits_random_chance_conditions() {
    let spec = Blueprint::<LifeCell>::builder()
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

#[test]
fn builder_emits_weighted_sum_conditions() {
    let spec = Blueprint::<LifeCell>::builder()
        .rules(|rules| {
            rules
                .when(LifeCell::Dead)
                .require(
                    neighbors(LifeCell::Alive)
                        .weighted_sum()
                        .at_least(Weight::cells(2)),
                )
                .becomes(LifeCell::Alive);
        })
        .build()
        .expect("valid spec");

    assert_eq!(
        spec.rules()[0].condition,
        Some(Condition::NeighborWeightedSum {
            state: LifeCell::Alive,
            comparison: WeightComparison::AtLeast(Weight::cells(2)),
        })
    );
}

#[test]
fn builder_rejects_zero_random_denominator() {
    let error = Blueprint::<LifeCell>::builder()
        .rules(|rules| {
            rules
                .when(LifeCell::Dead)
                .require(rng(2).one_in(0))
                .becomes(LifeCell::Alive);
        })
        .build()
        .expect_err("zero denominator must fail");

    assert_eq!(
        error,
        BuildError::InvalidRandomChance {
            stream: 2,
            one_in: 0
        }
    );
}
