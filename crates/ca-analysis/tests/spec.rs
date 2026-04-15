use hyle_ca_analysis::analyze_spec;
use hyle_ca_interface::{
    neighbors, AttrAssign, AttributeSet, AttributeType, Blueprint, MatAttr, MaterialSet,
    NeighborhoodFalloff, NeighborhoodRadius, NeighborhoodSet, NeighborhoodShape, NeighborhoodSpec,
    RuleSpec, Weight,
};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
enum M {
    #[default]
    Dead,
    Alive,
}

impl MaterialSet for M {
    fn variants() -> &'static [Self] {
        &[M::Dead, M::Alive]
    }

    fn label(self) -> &'static str {
        match self {
            M::Dead => "dead",
            M::Alive => "alive",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum A {
    Heat,
    Charge,
}

impl AttributeSet for A {
    fn variants() -> &'static [Self] {
        &[A::Heat, A::Charge]
    }

    fn label(self) -> &'static str {
        match self {
            A::Heat => "heat",
            A::Charge => "charge",
        }
    }

    fn value_type(self) -> AttributeType {
        match self {
            A::Heat => AttributeType::U8,
            A::Charge => AttributeType::I16,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum N {
    Adjacent,
    Far,
    Unused,
}

impl NeighborhoodSet for N {
    fn variants() -> &'static [Self] {
        &[N::Adjacent, N::Far, N::Unused]
    }

    fn label(self) -> &'static str {
        match self {
            N::Adjacent => "adjacent",
            N::Far => "far",
            N::Unused => "unused",
        }
    }
}

fn neighborhood_specs() -> [NeighborhoodSpec; 3] {
    [
        NeighborhoodSpec::new(
            N::Adjacent,
            NeighborhoodShape::Moore,
            NeighborhoodRadius::new(1),
            NeighborhoodFalloff::Uniform,
        ),
        NeighborhoodSpec::new(
            N::Far,
            NeighborhoodShape::Moore,
            NeighborhoodRadius::new(2),
            NeighborhoodFalloff::Uniform,
        ),
        NeighborhoodSpec::new(
            N::Unused,
            NeighborhoodShape::Moore,
            NeighborhoodRadius::new(3),
            NeighborhoodFalloff::Uniform,
        ),
    ]
}

#[test]
fn summarizes_rules_and_neighborhoods() {
    let spec = Blueprint::builder()
        .materials::<M>()
        .attributes::<A>()
        .material_attributes([
            MatAttr::new(M::Dead, []),
            MatAttr::new(
                M::Alive,
                [
                    AttrAssign::new(A::Heat).default(0u8),
                    AttrAssign::new(A::Charge).default(-1i16),
                ],
            ),
        ])
        .neighborhoods::<N>()
        .neighborhood_specs(neighborhood_specs())
        .rules([
            RuleSpec::when(M::Dead)
                .require(neighbors(M::Alive).count().eq(3))
                .becomes(M::Alive),
            RuleSpec::when(M::Alive)
                .using(N::Far)
                .require(neighbors(M::Alive).count().at_least(1))
                .keep(),
        ])
        .build()
        .expect("valid spec");

    let analysis = analyze_spec(&spec);

    assert_eq!(analysis.summary.rule_count, 2);
    assert_eq!(analysis.summary.materials.len(), 2);
    assert_eq!(analysis.summary.attribute_count, 2);
    assert_eq!(analysis.summary.neighborhood_count, 3);
    assert_eq!(analysis.summary.max_radius, 3);
    assert_eq!(analysis.neighborhoods[0].used_by_rules, 1);
    assert_eq!(analysis.neighborhoods[1].used_by_rules, 1);
    assert_eq!(analysis.neighborhoods[1].neighbor_count, 124);
}

#[test]
fn detects_shadowed_and_duplicate_rules() {
    let spec = Blueprint::builder()
        .materials::<M>()
        .neighborhoods::<N>()
        .neighborhood_specs(neighborhood_specs())
        .rules([
            RuleSpec::when(M::Alive).keep(),
            RuleSpec::when(M::Alive).keep(),
            RuleSpec::when(M::Alive)
                .require(neighbors(M::Alive).any())
                .becomes(M::Dead),
        ])
        .build()
        .expect("valid spec");

    let analysis = analyze_spec(&spec);

    assert_eq!(analysis.rules[1].duplicate_of, Some(0));
    assert_eq!(analysis.rules[1].shadowed_by, Some(0));
    assert_eq!(analysis.rules[2].shadowed_by, Some(0));
}

#[test]
fn warns_about_unused_named_neighborhoods() {
    let spec = Blueprint::builder()
        .materials::<M>()
        .neighborhoods::<N>()
        .neighborhood_specs(neighborhood_specs())
        .rules([RuleSpec::when(M::Alive).keep()])
        .build()
        .expect("valid spec");

    let analysis = analyze_spec(&spec);
    assert!(analysis
        .diagnostics
        .iter()
        .any(|diagnostic| diagnostic.code == "unused_neighborhood"));
}

#[test]
fn warns_about_impossible_weighted_sum_conditions() {
    let spec = Blueprint::builder()
        .materials::<M>()
        .neighborhoods::<N>()
        .neighborhood_specs(neighborhood_specs())
        .rules([RuleSpec::when(M::Dead)
            .require(
                neighbors(M::Alive)
                    .weighted_sum()
                    .at_least(Weight::cells(27)),
            )
            .becomes(M::Alive)])
        .build()
        .expect("valid spec");

    let analysis = analyze_spec(&spec);
    assert!(analysis.rules[0]
        .diagnostics
        .iter()
        .any(|diagnostic| diagnostic.code == "impossible_weighted_sum"));
}

#[test]
fn warns_about_impossible_neighbor_count_conditions() {
    let spec = Blueprint::builder()
        .materials::<M>()
        .neighborhoods::<N>()
        .neighborhood_specs(neighborhood_specs())
        .rules([RuleSpec::when(M::Dead)
            .require(neighbors(M::Alive).count().eq(27))
            .becomes(M::Alive)])
        .build()
        .expect("valid spec");

    let analysis = analyze_spec(&spec);
    assert!(analysis.rules[0]
        .diagnostics
        .iter()
        .any(|diagnostic| diagnostic.code == "impossible_neighbor_count"));
}
