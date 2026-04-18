//! Solver metadata capabilities.

use crate::{AttributeDef, AttributeId, MaterialDef, MaterialId, NeighborhoodId, NeighborhoodSpec};

/// Static schema metadata exposed by a solver.
pub trait SolverMetadata {
    /// Material descriptors declared on the active schema, if available.
    fn material_defs(&self) -> &[MaterialDef] {
        &[]
    }

    /// Resolve one material descriptor by identifier.
    fn material_def(&self, material: MaterialId) -> Option<&MaterialDef> {
        self.material_defs()
            .iter()
            .find(|definition| definition.id == material)
    }

    /// Attribute descriptors declared on the active schema, if available.
    fn attribute_defs(&self) -> &[AttributeDef] {
        &[]
    }

    /// Resolve one attribute descriptor by identifier.
    fn attribute_def(&self, attribute: AttributeId) -> Option<&AttributeDef> {
        self.attribute_defs()
            .iter()
            .find(|definition| definition.id == attribute)
    }

    /// Neighborhood specs declared on the active schema, if available.
    fn neighborhood_specs(&self) -> &[NeighborhoodSpec] {
        &[]
    }

    /// Resolve one neighborhood spec by identifier.
    fn neighborhood_spec(&self, neighborhood: NeighborhoodId) -> Option<&NeighborhoodSpec> {
        self.neighborhood_specs()
            .iter()
            .find(|spec| spec.id() == neighborhood)
    }
}
