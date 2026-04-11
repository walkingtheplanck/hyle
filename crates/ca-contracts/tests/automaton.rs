//! Tests for the declarative automaton builder.

use hyle_ca_contracts::{neighbors, BuildError, Hyle, NeighborhoodSpec, TopologyDescriptor};

#[test]
fn builder_emits_default_adjacent_neighborhood() {
    let spec = Hyle::builder()
        .cells::<u32>()
        .rules(|rules| {
            rules.when(0).require(neighbors(1).count().eq(3)).becomes(1);
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
        .cells::<u32>()
        .neighborhood("far", NeighborhoodSpec::cube(2))
        .default_neighborhood("far")
        .rules(|rules| {
            rules
                .when(0)
                .require(neighbors(1).count().at_least(1))
                .becomes(1);
        })
        .build()
        .expect("valid spec");

    assert_eq!(spec.default_neighborhood(), 1);
    assert_eq!(spec.rules()[0].neighborhood, 1);
}

#[test]
fn builder_rejects_duplicate_neighborhood_names() {
    let error = Hyle::builder()
        .cells::<u32>()
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
        .cells::<u32>()
        .rules(|rules| {
            rules.when(0).using("missing").becomes(1);
        })
        .build()
        .expect_err("unknown rule neighborhoods must fail");

    assert_eq!(
        error,
        BuildError::UnknownRuleNeighborhood("missing".to_string())
    );
}
