use crate::{
    AttributeDef, Blueprint, CellModel, CellSchema, NamedNeighborhood as NamedNeighborhoodSpec,
    Rule, Semantics,
};

use super::{interpret_topology, Neighborhood, ResolvedTopology};

/// A named interpreted neighborhood used by a blueprint.
#[derive(Clone, Debug, PartialEq)]
pub struct NamedNeighborhood {
    name: String,
    neighborhood: Neighborhood,
}

impl NamedNeighborhood {
    /// Construct a named interpreted neighborhood.
    pub fn new(name: impl Into<String>, neighborhood: Neighborhood) -> Self {
        Self {
            name: name.into(),
            neighborhood,
        }
    }

    /// Return the human-readable neighborhood name.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Return the interpreted neighborhood.
    pub fn neighborhood(&self) -> &Neighborhood {
        &self.neighborhood
    }
}

/// A canonical interpreted blueprint derived from a declarative blueprint.
#[derive(Clone, Debug, PartialEq)]
pub struct ResolvedBlueprint<C: CellModel> {
    cell_schema: CellSchema,
    semantics: Semantics,
    topology: ResolvedTopology,
    attributes: Vec<AttributeDef>,
    neighborhoods: Vec<NamedNeighborhood>,
    default_neighborhood: usize,
    rules: Vec<Rule<C>>,
}

impl<C: CellModel> ResolvedBlueprint<C> {
    /// Return the portable schema metadata for the cell model.
    pub fn cell_schema(&self) -> CellSchema {
        self.cell_schema
    }

    /// Return the declared semantics version.
    pub fn semantics(&self) -> Semantics {
        self.semantics
    }

    /// Return the interpreted topology descriptor.
    pub fn topology(&self) -> &ResolvedTopology {
        &self.topology
    }

    /// Return the declared attached per-cell attributes.
    pub fn attributes(&self) -> &[AttributeDef] {
        &self.attributes
    }

    /// Return the interpreted named neighborhoods.
    pub fn neighborhoods(&self) -> &[NamedNeighborhood] {
        &self.neighborhoods
    }

    /// Return the default neighborhood index.
    pub fn default_neighborhood(&self) -> usize {
        self.default_neighborhood
    }

    /// Return the ordered blueprint rules.
    pub fn rules(&self) -> &[Rule<C>] {
        &self.rules
    }
}

/// Interpret a declarative blueprint into its canonical semantic form.
pub fn interpret_blueprint<C: CellModel>(blueprint: &Blueprint<C>) -> ResolvedBlueprint<C> {
    ResolvedBlueprint {
        cell_schema: blueprint.cell_schema(),
        semantics: blueprint.semantics(),
        topology: interpret_topology(blueprint.topology()),
        attributes: blueprint.attributes().to_vec(),
        neighborhoods: blueprint
            .neighborhoods()
            .iter()
            .map(NamedNeighborhood::from)
            .collect(),
        default_neighborhood: blueprint.default_neighborhood(),
        rules: blueprint.rules().to_vec(),
    }
}

impl From<&NamedNeighborhoodSpec> for NamedNeighborhood {
    fn from(value: &NamedNeighborhoodSpec) -> Self {
        Self::new(value.name.clone(), Neighborhood::from_spec(value.spec))
    }
}
