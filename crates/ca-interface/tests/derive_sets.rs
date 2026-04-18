use hyle_ca_interface::{
    AttributeSet, AttributeType, Blueprint, BuildError, MaterialSet, NeighborhoodFalloff,
    NeighborhoodRadius, NeighborhoodSet, NeighborhoodShape, NeighborhoodSpec, RuleSpec,
    SetContractError,
};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, MaterialSet)]
enum Material {
    #[default]
    #[label("dead")]
    Dead,
    #[label("alive")]
    Alive,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, AttributeSet)]
enum Attribute {
    #[label("heat")]
    #[attribute_type(U8)]
    Heat,
    #[label("charged")]
    #[attribute_type(Bool)]
    Charged,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, NeighborhoodSet)]
enum Neighborhood {
    #[label("adjacent")]
    Adjacent,
    #[label("far")]
    Far,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
enum BrokenMaterial {
    #[default]
    Dead,
    Alive,
}

impl MaterialSet for BrokenMaterial {
    fn variants() -> &'static [Self] {
        &[BrokenMaterial::Dead]
    }

    fn label(self) -> &'static str {
        match self {
            BrokenMaterial::Dead => "dead",
            BrokenMaterial::Alive => "alive",
        }
    }
}

#[test]
fn material_set_derive_exposes_variants_labels_and_default() {
    assert_eq!(Material::variants(), &[Material::Dead, Material::Alive]);
    assert_eq!(Material::Dead.label(), "dead");
    assert_eq!(
        Material::Alive
            .id()
            .expect("derived material set should resolve")
            .index(),
        1
    );
    assert_eq!(
        Material::default_material().expect("derived material set should resolve"),
        Material::Dead
            .id()
            .expect("derived material set should resolve")
    );
}

#[test]
fn attribute_set_derive_exposes_value_types_and_ids() {
    assert_eq!(
        Attribute::variants(),
        &[Attribute::Heat, Attribute::Charged]
    );
    assert_eq!(Attribute::Heat.label(), "heat");
    assert_eq!(Attribute::Heat.value_type(), AttributeType::U8);
    assert_eq!(Attribute::Charged.value_type(), AttributeType::Bool);
    assert_eq!(
        Attribute::Charged
            .id()
            .expect("derived attribute set should resolve")
            .index(),
        1
    );
}

#[test]
fn neighborhood_set_derive_exposes_labels_and_default() {
    assert_eq!(
        Neighborhood::variants(),
        &[Neighborhood::Adjacent, Neighborhood::Far]
    );
    assert_eq!(Neighborhood::Far.label(), "far");
    assert_eq!(
        Neighborhood::default_neighborhood().expect("derived neighborhood set should resolve"),
        Neighborhood::Adjacent
            .id()
            .expect("derived neighborhood set should resolve")
    );
}

#[test]
fn broken_manual_material_set_reports_contract_error() {
    let result = Blueprint::builder()
        .materials::<BrokenMaterial>()
        .neighborhoods::<Neighborhood>()
        .neighborhood_specs([
            NeighborhoodSpec::new(
                Neighborhood::Adjacent,
                NeighborhoodShape::Moore,
                NeighborhoodRadius::new(1),
                NeighborhoodFalloff::Uniform,
            )
            .expect("derived neighborhood set should resolve"),
            NeighborhoodSpec::new(
                Neighborhood::Far,
                NeighborhoodShape::Moore,
                NeighborhoodRadius::new(2),
                NeighborhoodFalloff::Uniform,
            )
            .expect("derived neighborhood set should resolve"),
        ])
        .rules([RuleSpec::when(BrokenMaterial::Alive).keep()])
        .build();

    assert!(matches!(
        result,
        Err(BuildError::InvalidSetContract(
            SetContractError::MissingMaterialVariant { label: "alive", .. }
        ))
    ));
}
