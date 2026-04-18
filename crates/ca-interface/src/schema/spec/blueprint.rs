use crate::schema::{AttributeDef, MaterialDef, NeighborhoodSpec, TopologyDescriptor};
use crate::{MaterialId, NeighborhoodId};

use super::Rule;
use crate::schema::BlueprintBuilder;

/// Portable semantics version for a schema contract.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Semantics {
    /// Version 1 semantics: deterministic local rules with first-match wins.
    #[default]
    V1,
}

/// Immutable, solver-agnostic schema contract.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Blueprint {
    semantics: Semantics,
    topology: TopologyDescriptor,
    default_material: MaterialId,
    materials: Vec<MaterialDef>,
    attributes: Vec<AttributeDef>,
    neighborhoods: Vec<NeighborhoodSpec>,
    default_neighborhood: NeighborhoodId,
    rules: Vec<Rule>,
}

impl Blueprint {
    /// Start building a solver-agnostic schema.
    pub fn builder() -> BlueprintBuilder {
        BlueprintBuilder::new()
    }

    #[allow(clippy::too_many_arguments)]
    pub(crate) fn new(
        semantics: Semantics,
        topology: TopologyDescriptor,
        default_material: MaterialId,
        materials: Vec<MaterialDef>,
        attributes: Vec<AttributeDef>,
        neighborhoods: Vec<NeighborhoodSpec>,
        default_neighborhood: NeighborhoodId,
        rules: Vec<Rule>,
    ) -> Self {
        Self {
            semantics,
            topology,
            default_material,
            materials,
            attributes,
            neighborhoods,
            default_neighborhood,
            rules,
        }
    }

    /// The declared semantics version.
    pub fn semantics(&self) -> Semantics {
        self.semantics
    }

    /// The topology descriptor shared across solver implementations.
    pub fn topology(&self) -> TopologyDescriptor {
        self.topology
    }

    /// Material used to initialize empty runtime grids and guard reads.
    pub fn default_material(&self) -> MaterialId {
        self.default_material
    }

    /// Declared material universe with attached attributes.
    pub fn materials(&self) -> &[MaterialDef] {
        &self.materials
    }

    /// Declared attached per-cell attributes.
    pub fn attributes(&self) -> &[AttributeDef] {
        &self.attributes
    }

    /// Reusable named neighborhoods referenced by rules.
    pub fn neighborhoods(&self) -> &[NeighborhoodSpec] {
        &self.neighborhoods
    }

    /// Default neighborhood used by rules that do not override it.
    pub fn default_neighborhood(&self) -> NeighborhoodId {
        self.default_neighborhood
    }

    /// Ordered rules evaluated with first-match-wins semantics.
    pub fn rules(&self) -> &[Rule] {
        &self.rules
    }
}
