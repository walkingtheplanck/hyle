//! Tests for the declarative schema builder.

use hyle_ca_interface::{
    attr, neighbors, rng, AttrAssign, AttributeComparison, AttributeSet, AttributeType,
    AttributeValue, Blueprint, BuildError, MatAttr, MaterialSet, NeighborhoodFalloff,
    NeighborhoodRadius, NeighborhoodSet, NeighborhoodShape, NeighborhoodSpec, ResolvedCondition,
    RuleEffect, RuleSpec, TopologyDescriptor, Weight, WeightComparison,
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
    Charged,
}

impl AttributeSet for A {
    fn variants() -> &'static [Self] {
        &[A::Heat, A::Charged]
    }

    fn label(self) -> &'static str {
        match self {
            A::Heat => "heat",
            A::Charged => "charged",
        }
    }

    fn value_type(self) -> AttributeType {
        match self {
            A::Heat => AttributeType::U8,
            A::Charged => AttributeType::Bool,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum N {
    Adjacent,
    Far,
}

impl NeighborhoodSet for N {
    fn variants() -> &'static [Self] {
        &[N::Adjacent, N::Far]
    }

    fn label(self) -> &'static str {
        match self {
            N::Adjacent => "adjacent",
            N::Far => "far",
        }
    }
}

fn neighborhood_specs() -> [NeighborhoodSpec; 2] {
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
    ]
}

#[test]
fn builder_emits_registered_materials_and_neighborhoods() {
    let spec = Blueprint::builder()
        .materials::<M>()
        .attributes::<A>()
        .material_attributes([
            MatAttr::new(M::Dead, []),
            MatAttr::new(M::Alive, [AttrAssign::new(A::Heat).default(0u8)]),
        ])
        .neighborhoods::<N>()
        .neighborhood_specs(neighborhood_specs())
        .rules([RuleSpec::when(M::Dead)
            .require(neighbors(M::Alive).count().eq(3))
            .becomes(M::Alive)])
        .build()
        .expect("valid spec");

    assert_eq!(spec.topology(), TopologyDescriptor::bounded());
    assert_eq!(spec.default_material(), M::Dead.id());
    assert_eq!(spec.default_neighborhood(), N::Adjacent.id());
    assert_eq!(spec.materials().len(), 2);
    assert_eq!(spec.materials()[1].name, "alive");
    assert_eq!(spec.neighborhoods()[0].name(), "adjacent");
    assert_eq!(spec.neighborhoods()[1].radius().get(), 2);
    assert_eq!(spec.rules()[0].when, M::Dead.id());
    assert_eq!(spec.rules()[0].neighborhood, N::Adjacent.id());
    assert_eq!(spec.rules()[0].effect, RuleEffect::Become(M::Alive.id()));
}

#[test]
fn builder_emits_attribute_conditions_and_updates() {
    let spec = Blueprint::builder()
        .materials::<M>()
        .attributes::<A>()
        .material_attributes([
            MatAttr::new(M::Dead, []),
            MatAttr::new(
                M::Alive,
                [
                    AttrAssign::new(A::Heat).default(1u8),
                    AttrAssign::new(A::Charged).default(true),
                ],
            ),
        ])
        .neighborhoods::<N>()
        .neighborhood_specs(neighborhood_specs())
        .rules([RuleSpec::when(M::Alive)
            .require(attr(A::Heat).at_least(2u8))
            .set_attr(A::Heat, 0u8)
            .keep()])
        .build()
        .expect("valid spec");

    assert_eq!(
        spec.rules()[0].condition,
        Some(ResolvedCondition::Attribute {
            attribute: A::Heat.id(),
            comparison: AttributeComparison::AtLeast(AttributeValue::U8(2)),
        })
    );
    assert_eq!(spec.rules()[0].attribute_updates.len(), 1);
    assert_eq!(spec.rules()[0].attribute_updates[0].attribute, A::Heat.id());
}

#[test]
fn builder_emits_random_chance_and_weighted_conditions() {
    let spec = Blueprint::builder()
        .materials::<M>()
        .neighborhoods::<N>()
        .neighborhood_specs(neighborhood_specs())
        .rules([
            RuleSpec::when(M::Dead)
                .require(rng(7).one_in(3))
                .becomes(M::Alive),
            RuleSpec::when(M::Dead)
                .using(N::Far)
                .require(
                    neighbors(M::Alive)
                        .weighted_sum()
                        .at_least(Weight::cells(2)),
                )
                .becomes(M::Alive),
        ])
        .build()
        .expect("valid spec");

    assert_eq!(
        spec.rules()[0].condition,
        Some(ResolvedCondition::RandomChance {
            stream: 7,
            one_in: 3,
        })
    );
    assert_eq!(
        spec.rules()[1].condition,
        Some(ResolvedCondition::NeighborWeightedSum {
            material: M::Alive.id(),
            comparison: WeightComparison::AtLeast(Weight::cells(2)),
        })
    );
    assert_eq!(spec.rules()[1].neighborhood, N::Far.id());
}

#[test]
fn builder_rejects_missing_attached_attributes_in_conditions() {
    let error = Blueprint::builder()
        .materials::<M>()
        .attributes::<A>()
        .material_attributes([
            MatAttr::new(M::Dead, []),
            MatAttr::new(M::Alive, []),
        ])
        .neighborhoods::<N>()
        .neighborhood_specs(neighborhood_specs())
        .rules([RuleSpec::when(M::Alive).require(attr(A::Heat).eq(1u8)).keep()])
        .build()
        .expect_err("missing material attribute must fail");

    assert_eq!(
        error,
        BuildError::MissingMaterialAttribute {
            material: "alive",
            attribute: "heat",
        }
    );
}

#[test]
fn builder_rejects_duplicate_material_attribute_defaults() {
    let error = Blueprint::builder()
        .materials::<M>()
        .attributes::<A>()
        .material_attributes([
            MatAttr::new(M::Dead, []),
            MatAttr::new(
                M::Alive,
                [
                    AttrAssign::new(A::Heat).default(0u8),
                    AttrAssign::new(A::Heat).default(1u8),
                ],
            ),
        ])
        .neighborhoods::<N>()
        .neighborhood_specs(neighborhood_specs())
        .build()
        .expect_err("duplicate material attributes must fail");

    assert_eq!(
        error,
        BuildError::DuplicateMaterialAttribute {
            material: "alive",
            attribute: "heat",
        }
    );
}

#[test]
fn builder_rejects_invalid_boolean_ordered_comparisons() {
    let error = Blueprint::builder()
        .materials::<M>()
        .attributes::<A>()
        .material_attributes([
            MatAttr::new(M::Dead, []),
            MatAttr::new(M::Alive, [AttrAssign::new(A::Charged).default(true)]),
        ])
        .neighborhoods::<N>()
        .neighborhood_specs(neighborhood_specs())
        .rules([RuleSpec::when(M::Alive)
            .require(attr(A::Charged).at_least(true))
            .keep()])
        .build()
        .expect_err("ordered bool comparisons must fail");

    assert_eq!(
        error,
        BuildError::UnsupportedAttributeComparison {
            attribute: "charged",
            comparison: "at_least",
            value_type: AttributeType::Bool,
        }
    );
}

#[test]
fn builder_rejects_zero_random_denominator() {
    let error = Blueprint::builder()
        .materials::<M>()
        .neighborhoods::<N>()
        .neighborhood_specs(neighborhood_specs())
        .rules([RuleSpec::when(M::Dead)
            .require(rng(2).one_in(0))
            .becomes(M::Alive)])
        .build()
        .expect_err("zero denominator must fail");

    assert_eq!(
        error,
        BuildError::InvalidRandomChance {
            stream: 2,
            one_in: 0,
        }
    );
}
