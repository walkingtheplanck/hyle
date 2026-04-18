use hyle_ca_interface::{AttributeSet, AttributeType, MaterialSet, NeighborhoodSet};

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

#[test]
fn material_set_derive_exposes_variants_labels_and_default() {
    assert_eq!(Material::variants(), &[Material::Dead, Material::Alive]);
    assert_eq!(Material::Dead.label(), "dead");
    assert_eq!(Material::Alive.id().index(), 1);
    assert_eq!(Material::default_material(), Material::Dead.id());
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
    assert_eq!(Attribute::Charged.id().index(), 1);
}

#[test]
fn neighborhood_set_derive_exposes_labels_and_default() {
    assert_eq!(
        Neighborhood::variants(),
        &[Neighborhood::Adjacent, Neighborhood::Far]
    );
    assert_eq!(Neighborhood::Far.label(), "far");
    assert_eq!(
        Neighborhood::default_neighborhood(),
        Neighborhood::Adjacent.id()
    );
}
