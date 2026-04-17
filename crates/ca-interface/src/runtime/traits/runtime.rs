//! Object-safe runtime interface for consumers that only need to drive a simulation.

use crate::{
    AttributeAccessError, AttributeDef, AttributeId, AttributeValue, CellAttributeValue, CellId,
    CellQueryError, GridDims, GridRegion, GridSnapshot, MaterialDef, MaterialId, NeighborhoodId,
    NeighborhoodSpec, TransitionCount,
};

/// A compact simulation runtime surface for consumers.
///
/// This trait exists for consumers such as viewers and tools that need common
/// simulation operations without depending on the full [`CaSolver`] contract.
pub trait CaRuntime: Send {
    /// Logical grid dimensions.
    fn dims(&self) -> GridDims;

    /// Number of logical cells in the current grid.
    fn cell_count(&self) -> usize {
        self.dims().cell_count()
    }

    /// Advance the simulation by one step.
    fn step(&mut self);

    /// Material descriptors declared on the active schema, if available.
    fn material_defs(&self) -> &[MaterialDef];

    /// Resolve one material descriptor by identifier.
    fn material_def(&self, material: MaterialId) -> Option<&MaterialDef> {
        self.material_defs()
            .iter()
            .find(|definition| definition.id == material)
    }

    /// Attribute descriptors declared on the active schema, if available.
    fn attribute_defs(&self) -> &[AttributeDef];

    /// Resolve one attribute descriptor by identifier.
    fn attribute_def(&self, attribute: AttributeId) -> Option<&AttributeDef> {
        self.attribute_defs()
            .iter()
            .find(|definition| definition.id == attribute)
    }

    /// Neighborhood specs declared on the active schema, if available.
    fn neighborhood_specs(&self) -> &[NeighborhoodSpec];

    /// Resolve one neighborhood spec by identifier.
    fn neighborhood_spec(&self, neighborhood: NeighborhoodId) -> Option<&NeighborhoodSpec> {
        self.neighborhood_specs()
            .iter()
            .find(|spec| spec.id() == neighborhood)
    }

    /// Resolve one logical cell handle from grid coordinates.
    fn cell_at(&self, x: i32, y: i32, z: i32) -> Option<CellId>;

    /// Return `true` when the given cell handle belongs to the active runtime.
    fn contains_cell(&self, cell: CellId) -> bool {
        self.cell_position(cell).is_ok()
    }

    /// Resolve every logical cell handle in the active grid in x-major order.
    fn cells(&self) -> Vec<CellId> {
        self.cells_in_region(self.dims().as_region())
    }

    /// Resolve all logical cell handles in one in-bounds region in x-major order.
    fn cells_in_region(&self, region: GridRegion) -> Vec<CellId> {
        let dims = self.dims();
        assert!(
            dims.contains_region(region),
            "region must lie within runtime dimensions"
        );

        let [ox, oy, oz] = region.origin;
        let [sx, sy, sz] = region.size;
        let mut cells = Vec::with_capacity(region.cell_count());

        for z in oz..oz + sz {
            for y in oy..oy + sy {
                for x in ox..ox + sx {
                    let cell = self
                        .cell_at(x as i32, y as i32, z as i32)
                        .expect("in-bounds region coordinates must resolve to cells");
                    cells.push(cell);
                }
            }
        }

        cells
    }

    /// Decode a cell handle back into its canonical grid position.
    fn cell_position(&self, cell: CellId) -> Result<[u32; 3], CellQueryError>;

    /// Read one material from a resolved cell handle.
    fn material(&self, cell: CellId) -> Result<MaterialId, CellQueryError>;

    /// Read one attached attribute from a resolved cell handle.
    fn attribute(
        &self,
        cell: CellId,
        attribute: AttributeId,
    ) -> Result<AttributeValue, CellQueryError>;

    /// Read all declared attached attributes from a resolved cell handle.
    fn attributes(&self, cell: CellId) -> Result<Vec<CellAttributeValue>, CellQueryError>;

    /// Resolve all neighbors around one cell for the given neighborhood.
    fn neighbors(
        &self,
        cell: CellId,
        neighborhood: NeighborhoodId,
    ) -> Result<Vec<CellId>, CellQueryError>;

    /// Resolve all neighbors and read their current materials.
    fn neighbor_materials(
        &self,
        cell: CellId,
        neighborhood: NeighborhoodId,
    ) -> Result<Vec<(CellId, MaterialId)>, CellQueryError> {
        self.neighbors(cell, neighborhood)?
            .into_iter()
            .map(|neighbor| Ok((neighbor, self.material(neighbor)?)))
            .collect()
    }

    /// Set a material at the given coordinate.
    fn set(&mut self, x: i32, y: i32, z: i32, material: MaterialId);

    /// Read one attached attribute by id from the resolved cell coordinate.
    fn get_attr(
        &self,
        attribute: AttributeId,
        x: i32,
        y: i32,
        z: i32,
    ) -> Result<AttributeValue, AttributeAccessError>;

    /// Overwrite one attached attribute by id at the resolved cell coordinate.
    fn set_attr(
        &mut self,
        attribute: AttributeId,
        x: i32,
        y: i32,
        z: i32,
        value: AttributeValue,
    ) -> Result<(), AttributeAccessError>;

    /// Read a contiguous rectangular region in x-major order.
    fn read_region(&self, region: GridRegion) -> Vec<MaterialId>;

    /// Overwrite a contiguous rectangular region from x-major ordered data.
    fn write_region(&mut self, region: GridRegion, cells: &[MaterialId]);

    /// Replace the full current state from x-major ordered data.
    fn replace_cells(&mut self, cells: &[MaterialId]);

    /// Read the full current state back to the host.
    fn readback(&self) -> GridSnapshot<MaterialId>;

    /// Number of completed steps.
    fn step_count(&self) -> u32;

    /// Number of cells whose material changed during the latest completed step.
    fn last_changed_cells(&self) -> u64;

    /// Population of one material in the current grid state.
    fn population(&self, material: MaterialId) -> u64;

    /// Full current per-material population table.
    fn populations(&self) -> Vec<u64>;

    /// Material-to-material transition counts from the latest completed step.
    fn last_transitions(&self) -> &[TransitionCount];
}
