use crate::{
    BlueprintSpec, CellModel, CellSchema, NamedNeighborhood as NamedNeighborhoodSpec, Rule,
    Semantics,
};

use super::{interpret_topology, Neighborhood, Topology};

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

/// A canonical interpreted blueprint derived from a declarative blueprint spec.
#[derive(Clone, Debug, PartialEq)]
pub struct Blueprint<C: CellModel> {
    cell_schema: CellSchema,
    semantics: Semantics,
    topology: Topology,
    neighborhoods: Vec<NamedNeighborhood>,
    default_neighborhood: usize,
    rules: Vec<Rule<C>>,
}

impl<C: CellModel> Blueprint<C> {
    /// Return the portable schema metadata for the cell model.
    pub fn cell_schema(&self) -> CellSchema {
        self.cell_schema
    }

    /// Return the declared semantics version.
    pub fn semantics(&self) -> Semantics {
        self.semantics
    }

    /// Return the interpreted topology descriptor.
    pub fn topology(&self) -> &Topology {
        &self.topology
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

/// Interpret a declarative blueprint spec into its canonical semantic form.
pub fn interpret_blueprint<C: CellModel>(spec: &BlueprintSpec<C>) -> Blueprint<C> {
    Blueprint {
        cell_schema: spec.cell_schema(),
        semantics: spec.semantics(),
        topology: interpret_topology(spec.topology()),
        neighborhoods: spec
            .neighborhoods()
            .iter()
            .map(NamedNeighborhood::from)
            .collect(),
        default_neighborhood: spec.default_neighborhood(),
        rules: spec.rules().to_vec(),
    }
}

impl From<&NamedNeighborhoodSpec> for NamedNeighborhood {
    fn from(value: &NamedNeighborhoodSpec) -> Self {
        Self::new(value.name.clone(), Neighborhood::from_spec(value.spec))
    }
}
