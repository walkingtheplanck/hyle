use hyle_ca_contracts::{
    BlueprintSpec, Cell, NamedNeighborhood as NamedNeighborhoodSpec, Rule, Semantics,
    TopologyDescriptor,
};

use crate::Neighborhood;

/// A named interpreted neighborhood used by a blueprint.
#[derive(Clone, Debug, PartialEq, Eq)]
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
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Blueprint<C: Cell> {
    semantics: Semantics,
    topology: TopologyDescriptor,
    neighborhoods: Vec<NamedNeighborhood>,
    default_neighborhood: usize,
    rules: Vec<Rule<C>>,
}

impl<C: Cell> Blueprint<C> {
    /// Return the declared semantics version.
    pub fn semantics(&self) -> Semantics {
        self.semantics
    }

    /// Return the interpreted topology descriptor.
    pub fn topology(&self) -> TopologyDescriptor {
        self.topology
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
pub fn interpret_blueprint<C: Cell + Clone>(spec: &BlueprintSpec<C>) -> Blueprint<C> {
    Blueprint {
        semantics: spec.semantics(),
        topology: spec.topology(),
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
