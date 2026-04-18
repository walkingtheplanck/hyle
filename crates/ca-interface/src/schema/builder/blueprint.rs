use crate::schema::{
    Blueprint, NeighborhoodRef, NeighborhoodSet, NeighborhoodSpec, Semantics, TopologyDescriptor,
};

use super::errors::BuildError;
use super::materials::{
    apply_material_attributes, register_attributes, register_materials,
    validate_unique_attribute_labels, validate_unique_material_labels, AttributeRegistry, MatAttr,
    MaterialRegistry,
};
use super::neighborhoods::{register_neighborhoods, validate_neighborhoods, NeighborhoodRegistry};
use super::rules::RuleSpec;

/// Typed schema builder.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BlueprintBuilder {
    semantics: Semantics,
    topology: TopologyDescriptor,
    materials: Option<MaterialRegistry>,
    attributes: Option<AttributeRegistry>,
    material_attributes: Vec<MatAttr>,
    neighborhoods: Option<NeighborhoodRegistry>,
    neighborhood_specs: Vec<NeighborhoodSpec>,
    default_neighborhood: Option<NeighborhoodRef>,
    rules: Vec<RuleSpec>,
}

impl BlueprintBuilder {
    pub(crate) fn new() -> Self {
        Self {
            semantics: Semantics::V1,
            topology: TopologyDescriptor::bounded(),
            materials: None,
            attributes: None,
            material_attributes: Vec::new(),
            neighborhoods: None,
            neighborhood_specs: Vec::new(),
            default_neighborhood: None,
            rules: Vec::new(),
        }
    }

    /// Override the topology descriptor used by this schema.
    pub fn topology(mut self, topology: TopologyDescriptor) -> Self {
        self.topology = topology;
        self
    }

    /// Register the enum-backed material universe for this schema.
    pub fn materials<M: crate::schema::MaterialSet>(mut self) -> Self {
        self.materials = Some(register_materials::<M>());
        self
    }

    /// Register the enum-backed attribute universe for this schema.
    pub fn attributes<A: crate::schema::AttributeSet>(mut self) -> Self {
        self.attributes = Some(register_attributes::<A>());
        self
    }

    /// Attach attributes to materials with material-specific defaults.
    pub fn material_attributes<I>(mut self, assignments: I) -> Self
    where
        I: IntoIterator<Item = MatAttr>,
    {
        self.material_attributes = assignments.into_iter().collect();
        self
    }

    /// Register the enum-backed neighborhood universe for this schema.
    pub fn neighborhoods<N: NeighborhoodSet>(mut self) -> Self {
        self.neighborhoods = Some(register_neighborhoods::<N>());
        self
    }

    /// Override the default neighborhood used by rules without `using(...)`.
    pub fn default_neighborhood<N: NeighborhoodSet>(mut self, neighborhood: N) -> Self {
        self.default_neighborhood = Some(neighborhood.neighborhood());
        self
    }

    /// Provide one spec for each declared neighborhood.
    pub fn neighborhood_specs<I>(mut self, neighborhoods: I) -> Self
    where
        I: IntoIterator<Item = NeighborhoodSpec>,
    {
        self.neighborhood_specs = neighborhoods.into_iter().collect();
        self
    }

    /// Provide the ordered rules for this schema.
    pub fn rules<I>(mut self, rules: I) -> Self
    where
        I: IntoIterator<Item = RuleSpec>,
    {
        self.rules = rules.into_iter().collect();
        self
    }

    /// Validate and build the portable schema.
    pub fn build(self) -> Result<Blueprint, BuildError> {
        let mut materials = self.materials.ok_or(BuildError::MissingMaterials)?;
        validate_unique_material_labels(&materials.materials)?;

        let attributes = match self.attributes {
            Some(attributes) => {
                validate_unique_attribute_labels(&attributes.attributes)?;
                Some(attributes)
            }
            None => None,
        };

        apply_material_attributes(
            &mut materials,
            attributes.as_ref(),
            &self.material_attributes,
        )?;

        let neighborhoods = self.neighborhoods.ok_or(BuildError::MissingNeighborhoods)?;
        let (neighborhood_specs, default_neighborhood) = validate_neighborhoods(
            &neighborhoods,
            &self.neighborhood_specs,
            self.default_neighborhood,
        )?;

        let rules = self
            .rules
            .into_iter()
            .map(|rule| {
                rule.build(
                    &materials,
                    attributes.as_ref(),
                    &neighborhoods,
                    default_neighborhood,
                )
            })
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Blueprint::new(
            self.semantics,
            self.topology,
            materials.default_material,
            materials.materials,
            attributes.map_or_else(Vec::new, |registry| registry.attributes),
            neighborhood_specs,
            default_neighborhood,
            rules,
        ))
    }
}
