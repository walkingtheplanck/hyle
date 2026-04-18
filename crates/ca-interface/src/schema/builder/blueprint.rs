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
    materials: Option<Result<MaterialRegistry, BuildError>>,
    attributes: Option<Result<AttributeRegistry, BuildError>>,
    material_attributes: Vec<MatAttr>,
    neighborhoods: Option<Result<NeighborhoodRegistry, BuildError>>,
    neighborhood_specs: Vec<NeighborhoodSpec>,
    default_neighborhood: Option<NeighborhoodRef>,
    rules: Vec<RuleSpec>,
}

impl BlueprintBuilder {
    /// Construct a builder with versioned defaults.
    ///
    /// This stays crate-private so callers enter through `Blueprint::builder()`
    /// instead of depending on the builder's internal default choices.
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
    ///
    /// The builder stores the declarative descriptor and leaves its
    /// interpretation to the `resolved` layer so authoring stays backend-agnostic.
    pub fn topology(mut self, topology: TopologyDescriptor) -> Self {
        self.topology = topology;
        self
    }

    /// Register the enum-backed material universe for this schema.
    ///
    /// This fixes the single material set that every later material reference
    /// must belong to. Cross-set references are rejected during `build()`.
    pub fn materials<M: crate::schema::MaterialSet>(mut self) -> Self {
        self.materials = Some(register_materials::<M>());
        self
    }

    /// Register the enum-backed attribute universe for this schema.
    ///
    /// Attributes remain optional at the schema level, but once one set is
    /// registered every attribute reference is validated against that family.
    pub fn attributes<A: crate::schema::AttributeSet>(mut self) -> Self {
        self.attributes = Some(register_attributes::<A>());
        self
    }

    /// Attach attributes to materials with material-specific defaults.
    ///
    /// These assignments declare which attributes exist on each material at
    /// runtime and what value they reset to when a cell becomes that material.
    pub fn material_attributes<I>(mut self, assignments: I) -> Self
    where
        I: IntoIterator<Item = MatAttr>,
    {
        self.material_attributes = assignments.into_iter().collect();
        self
    }

    /// Register the enum-backed neighborhood universe for this schema.
    ///
    /// Neighborhood specs are provided separately so the enum declares the
    /// names/ids while the builder call supplies the concrete shapes.
    pub fn neighborhoods<N: NeighborhoodSet>(mut self) -> Self {
        self.neighborhoods = Some(register_neighborhoods::<N>());
        self
    }

    /// Override the default neighborhood used by rules without `using(...)`.
    ///
    /// This keeps individual rules terse while still making the fallback
    /// explicit at schema construction time.
    pub fn default_neighborhood<N: NeighborhoodSet>(mut self, neighborhood: N) -> Self {
        self.default_neighborhood = Some(neighborhood.neighborhood());
        self
    }

    /// Provide one spec for each declared neighborhood.
    ///
    /// Validation later ensures that the declared enum universe and the supplied
    /// specs line up one-for-one with no gaps or duplicates.
    pub fn neighborhood_specs<I>(mut self, neighborhoods: I) -> Self
    where
        I: IntoIterator<Item = NeighborhoodSpec>,
    {
        self.neighborhood_specs = neighborhoods.into_iter().collect();
        self
    }

    /// Provide the ordered rules for this schema.
    ///
    /// Rule order is preserved because solvers evaluate the compiled rules in
    /// declaration order and stop at the first match.
    pub fn rules<I>(mut self, rules: I) -> Self
    where
        I: IntoIterator<Item = RuleSpec>,
    {
        self.rules = rules.into_iter().collect();
        self
    }

    /// Validate and build the portable schema.
    ///
    /// This is the boundary where the builder's erased references are checked
    /// against the registered material, attribute, and neighborhood sets.
    ///
    /// # Errors
    ///
    /// Returns [`BuildError`] when the declaration is incomplete, mixes enum
    /// families, repeats labels or assignments, or uses incompatible attribute
    /// values and rule conditions.
    pub fn build(self) -> Result<Blueprint, BuildError> {
        let mut materials = self.materials.ok_or(BuildError::MissingMaterials)??;
        validate_unique_material_labels(&materials.materials)?;

        let attributes = match self.attributes {
            Some(attributes) => {
                let attributes = attributes?;
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

        let neighborhoods = self
            .neighborhoods
            .ok_or(BuildError::MissingNeighborhoods)??;
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
