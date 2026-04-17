use crate::{AttributeDef, Blueprint, MaterialDef, NeighborhoodSpec, Rule, Semantics};

use super::{interpret_topology, Neighborhood, ResolvedTopology};

/// An interpreted neighborhood used by a schema.
#[derive(Clone, Debug, PartialEq)]
pub struct NamedNeighborhood {
    spec: NeighborhoodSpec,
    neighborhood: Neighborhood,
}

impl NamedNeighborhood {
    /// Construct an interpreted neighborhood.
    pub fn new(spec: NeighborhoodSpec, neighborhood: Neighborhood) -> Self {
        Self { spec, neighborhood }
    }

    /// Return the declarative neighborhood spec.
    pub const fn spec(&self) -> NeighborhoodSpec {
        self.spec
    }

    /// Return the interpreted neighborhood.
    pub fn neighborhood(&self) -> &Neighborhood {
        &self.neighborhood
    }
}

/// A canonical interpreted schema derived from a declarative schema.
#[derive(Clone, Debug, PartialEq)]
pub struct ResolvedBlueprint {
    semantics: Semantics,
    topology: ResolvedTopology,
    default_material: crate::MaterialId,
    materials: Vec<MaterialDef>,
    attributes: Vec<AttributeDef>,
    neighborhoods: Vec<NamedNeighborhood>,
    default_neighborhood: crate::NeighborhoodId,
    rules: Vec<Rule>,
}

impl ResolvedBlueprint {
    /// Return the declared semantics version.
    pub fn semantics(&self) -> Semantics {
        self.semantics
    }

    /// Return the interpreted topology descriptor.
    pub fn topology(&self) -> &ResolvedTopology {
        &self.topology
    }

    /// Return the default material identifier.
    pub fn default_material(&self) -> crate::MaterialId {
        self.default_material
    }

    /// Return the declared material universe.
    pub fn materials(&self) -> &[MaterialDef] {
        &self.materials
    }

    /// Return the declared attached per-cell attributes.
    pub fn attributes(&self) -> &[AttributeDef] {
        &self.attributes
    }

    /// Return the interpreted neighborhoods.
    pub fn neighborhoods(&self) -> &[NamedNeighborhood] {
        &self.neighborhoods
    }

    /// Return the default neighborhood identifier.
    pub fn default_neighborhood(&self) -> crate::NeighborhoodId {
        self.default_neighborhood
    }

    /// Return the ordered schema rules.
    pub fn rules(&self) -> &[Rule] {
        &self.rules
    }
}

/// Interpret a declarative schema into its canonical semantic form.
pub fn interpret_blueprint(blueprint: &Blueprint) -> ResolvedBlueprint {
    ResolvedBlueprint {
        semantics: blueprint.semantics(),
        topology: interpret_topology(blueprint.topology()),
        default_material: blueprint.default_material(),
        materials: blueprint.materials().to_vec(),
        attributes: blueprint.attributes().to_vec(),
        neighborhoods: blueprint
            .neighborhoods()
            .iter()
            .copied()
            .map(|spec| NamedNeighborhood::new(spec, Neighborhood::from_spec(spec)))
            .collect(),
        default_neighborhood: blueprint.default_neighborhood(),
        rules: blueprint.rules().to_vec(),
    }
}
