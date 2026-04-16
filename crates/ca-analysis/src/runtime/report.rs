//! Public report types returned by runtime analysis.

use hyle_ca_interface::{
    AttributeId, AttributeType, AttributeValue, Cell, MaterialId, NeighborhoodId, TransitionCount,
};

/// Population count for one material after a completed step.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MaterialPopulation {
    /// Material identifier.
    pub material: MaterialId,
    /// Number of cells with that material after the step.
    pub count: u64,
}

/// Higher-level runtime summary derived from one solver step report.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RuntimeReport {
    /// Step number after the transition has been applied.
    pub step: u32,
    /// Total number of cells covered by the report.
    pub total_cells: u64,
    /// Number of cells that changed material during the step.
    pub changed_cells: u64,
    /// Number of cells that kept the same material.
    pub stable_cells: u64,
    /// Number of cells considered alive under the caller-supplied alive set.
    pub living_cells: u64,
    /// Number of cells that transitioned from non-alive to alive.
    pub born_cells: u64,
    /// Number of cells that transitioned from alive to non-alive.
    pub died_cells: u64,
    /// Final non-zero per-material populations.
    pub populations: Vec<MaterialPopulation>,
    /// Material-to-material transition counts for changed cells.
    pub transitions: Vec<TransitionCount>,
}

/// One material reference resolved to a human-readable name.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MaterialView {
    /// Material identifier.
    pub id: MaterialId,
    /// Human-readable material name.
    pub name: &'static str,
}

/// One current attached attribute value on an inspected cell.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AttributeView {
    /// Attribute identifier.
    pub id: AttributeId,
    /// Human-readable attribute name.
    pub name: &'static str,
    /// Declared scalar type.
    pub value_type: AttributeType,
    /// Current cell value.
    pub value: AttributeValue,
}

/// Material breakdown for one inspected neighborhood.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct NeighborhoodMaterialCount {
    /// Material identifier.
    pub material: MaterialId,
    /// Human-readable material name.
    pub name: &'static str,
    /// Number of matching neighbors carrying this material.
    pub count: u64,
}

/// Summary for one neighborhood around an inspected cell.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NeighborhoodReport {
    /// Neighborhood identifier.
    pub id: NeighborhoodId,
    /// Human-readable neighborhood name.
    pub name: &'static str,
    /// Number of resolved neighbors in this neighborhood.
    pub neighbor_count: usize,
    /// Material distribution across those neighbors.
    pub materials: Vec<NeighborhoodMaterialCount>,
}

/// Structured report for one selected cell position.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CellReport {
    /// Position requested by the caller.
    pub requested_position: [i32; 3],
    /// Resolved runtime cell handle.
    pub cell: Cell,
    /// Canonical in-bounds cell position.
    pub resolved_position: [u32; 3],
    /// Current material on the cell.
    pub material: MaterialView,
    /// Current attached attributes on the cell.
    pub attributes: Vec<AttributeView>,
    /// Neighborhood summaries around the cell.
    pub neighborhoods: Vec<NeighborhoodReport>,
}
