//! Solver trait - the common interface all CA solvers implement.

use crate::semantics;
use crate::{
    AttributeAccessError, AttributeDef, AttributeId, AttributeValue, Cell, CellAttributeValue,
    CellQueryError, GridDims, GridRegion, GridSnapshot, MaterialDef, MaterialId, NeighborhoodId,
    NeighborhoodSpec, Topology, TransitionCount,
};

/// The common interface shared by all CA solvers (CPU, GPU, etc.).
///
/// Blueprint authoring is intentionally kept out of this trait. Solvers are
/// free to consume declarative specs, interpreted blueprints, precompiled
/// programs, or other portable representations as long as they uphold the
/// runtime contract below.
///
/// Contracts (enforced by `ValidatedSolver` in debug builds):
/// - `get(x,y,z)` and `set(x,y,z,...)` follow `resolve_index(...)`.
/// - If `resolve_index(...)` returns `guard_index()`, `get(...)` returns
///   `MaterialId::default()` and `set(...)` is a silent no-op.
/// - If `resolve_index(...)` returns an index other than `guard_index()`,
///   `get(...)` and `set(...)` must behave as if they were applied to that
///   resolved in-bounds cell index.
/// - `step()` increments `step_count()` by exactly 1.
/// - `width()`, `height()`, `depth()` never change after construction.
/// - After `set(...)`, `write_region(...)`, `replace_cells(...)`, or `step()`
///   returns, subsequent reads through `get(...)`, `iter_materials()`,
///   `read_region(...)`, and `readback()` must observe the latest state.
///   GPU solvers may block internally to satisfy this contract.
pub trait CaSolver {
    /// Topology policy used by this solver.
    type Topology: Topology;

    /// Grid width in cells.
    fn width(&self) -> u32;
    /// Grid height in cells.
    fn height(&self) -> u32;
    /// Grid depth in cells.
    fn depth(&self) -> u32;
    /// Grid dimensions.
    fn dims(&self) -> GridDims {
        GridDims::new(self.width(), self.height(), self.depth())
    }

    /// Deterministic run seed used for semantic randomness.
    fn seed(&self) -> u64 {
        0
    }

    /// Topology policy used to resolve coordinates for reads, writes, and steps.
    fn topology(&self) -> &Self::Topology;

    /// Number of logical cells in the current grid.
    fn cell_count(&self) -> usize {
        self.dims().cell_count()
    }

    /// One-past-the-end logical cell index used as the "no cell" sentinel.
    fn guard_index(&self) -> usize {
        self.cell_count()
    }

    /// Resolve a possibly out-of-range coordinate to a linear cell index.
    fn resolve_index(&self, x: i32, y: i32, z: i32) -> usize {
        self.topology()
            .resolve_index(x, y, z, self.dims(), self.guard_index())
    }

    /// Read one material value at the given coordinate.
    fn get(&self, x: i32, y: i32, z: i32) -> MaterialId;

    /// Set one material value at the given coordinate.
    fn set(&mut self, x: i32, y: i32, z: i32, material: MaterialId);

    /// Read one attached attribute by id from the given coordinate.
    fn get_attr(
        &self,
        attribute: AttributeId,
        x: i32,
        y: i32,
        z: i32,
    ) -> Result<AttributeValue, AttributeAccessError>;

    /// Write one attached attribute by id to the given coordinate.
    fn set_attr(
        &mut self,
        attribute: AttributeId,
        x: i32,
        y: i32,
        z: i32,
        value: AttributeValue,
    ) -> Result<(), AttributeAccessError>;

    /// Material descriptors declared on the active blueprint, if available.
    fn material_defs(&self) -> &[MaterialDef] {
        &[]
    }

    /// Attribute descriptors declared on the active blueprint, if available.
    fn attribute_defs(&self) -> &[AttributeDef] {
        &[]
    }

    /// Neighborhood specs declared on the active blueprint, if available.
    fn neighborhood_specs(&self) -> &[NeighborhoodSpec] {
        &[]
    }

    /// Resolve one logical cell handle from grid coordinates.
    fn cell_at(&self, x: i32, y: i32, z: i32) -> Option<Cell> {
        let index = self.resolve_index(x, y, z);
        (index != self.guard_index()).then(|| Cell::new(index as u32))
    }

    /// Decode a cell handle back into its canonical grid position.
    fn cell_position(&self, cell: Cell) -> Result<[u32; 3], CellQueryError> {
        if cell.index() >= self.cell_count() {
            return Err(CellQueryError::UnknownCell(cell));
        }

        let width = self.width() as usize;
        let height = self.height() as usize;
        let x = cell.index() % width;
        let y = (cell.index() / width) % height;
        let z = cell.index() / (width * height);
        Ok([x as u32, y as u32, z as u32])
    }

    /// Read one material from a resolved cell handle.
    fn material(&self, cell: Cell) -> Result<MaterialId, CellQueryError> {
        let [x, y, z] = self.cell_position(cell)?;
        Ok(self.get(x as i32, y as i32, z as i32))
    }

    /// Read one attached attribute from a resolved cell handle.
    fn attribute(
        &self,
        cell: Cell,
        attribute: AttributeId,
    ) -> Result<AttributeValue, CellQueryError> {
        let [x, y, z] = self.cell_position(cell)?;
        self.get_attr(attribute, x as i32, y as i32, z as i32)
            .map_err(CellQueryError::from)
    }

    /// Read all declared attached attributes from a resolved cell handle.
    fn attributes(&self, cell: Cell) -> Result<Vec<CellAttributeValue>, CellQueryError> {
        let mut values = Vec::with_capacity(self.attribute_defs().len());
        for attribute in self.attribute_defs() {
            values.push(CellAttributeValue::new(
                attribute.id,
                self.attribute(cell, attribute.id)?,
            ));
        }
        Ok(values)
    }

    /// Resolve all neighbors around one cell for the given neighborhood.
    fn neighbors(
        &self,
        cell: Cell,
        neighborhood: NeighborhoodId,
    ) -> Result<Vec<Cell>, CellQueryError> {
        let spec = self
            .neighborhood_specs()
            .iter()
            .find(|spec| spec.id() == neighborhood)
            .copied()
            .ok_or(CellQueryError::UnknownNeighborhood(neighborhood))?;
        let [x, y, z] = self.cell_position(cell)?;
        let mut cells = Vec::new();

        for offset in semantics::offsets(spec) {
            if let Some(neighbor) =
                self.cell_at(x as i32 + offset.dx, y as i32 + offset.dy, z as i32 + offset.dz)
            {
                cells.push(neighbor);
            }
        }

        Ok(cells)
    }

    /// Advance the simulation by one logical step.
    fn step(&mut self);

    /// Number of logical steps already completed.
    fn step_count(&self) -> u32;

    /// Number of cells whose material changed during the latest completed step.
    fn last_changed_cells(&self) -> u64;

    /// Population of one material in the current grid state.
    fn population(&self, material: MaterialId) -> u64 {
        self.populations()
            .get(material.index())
            .copied()
            .unwrap_or_default()
    }

    /// Full current per-material population table.
    fn populations(&self) -> Vec<u64> {
        let mut populations = Vec::new();
        for (_, _, _, material) in self.iter_cells() {
            if populations.len() <= material.index() {
                populations.resize(material.index() + 1, 0);
            }
            populations[material.index()] += 1;
        }
        populations
    }

    /// Material-to-material transition counts from the latest completed step.
    fn last_transitions(&self) -> &[TransitionCount];

    /// Read back all logical materials in x-major order.
    fn iter_cells(&self) -> Vec<(u32, u32, u32, MaterialId)> {
        let dims = self.dims();
        let mut cells = Vec::with_capacity(dims.cell_count());
        for z in 0..dims.depth {
            for y in 0..dims.height {
                for x in 0..dims.width {
                    cells.push((x, y, z, self.get(x as i32, y as i32, z as i32)));
                }
            }
        }
        cells
    }

    /// Read the full current state back to the host.
    fn readback(&self) -> GridSnapshot<MaterialId> {
        let dims = self.dims();
        let mut cells = vec![MaterialId::default(); dims.cell_count()];
        let width = dims.width as usize;
        let height = dims.height as usize;

        for (x, y, z, material) in self.iter_cells() {
            let index = (x as usize) + (y as usize) * width + (z as usize) * width * height;
            cells[index] = material;
        }

        GridSnapshot::new(dims, cells)
    }

    /// Read a contiguous rectangular region in x-major order.
    fn read_region(&self, region: GridRegion) -> Vec<MaterialId> {
        let dims = self.dims();
        assert!(dims.contains_region(region), "region must lie within solver dimensions");

        let mut cells = Vec::with_capacity(region.cell_count());
        let [ox, oy, oz] = region.origin;
        let [sx, sy, sz] = region.size;

        for z in oz..oz + sz {
            for y in oy..oy + sy {
                for x in ox..ox + sx {
                    cells.push(self.get(x as i32, y as i32, z as i32));
                }
            }
        }

        cells
    }

    /// Overwrite a contiguous rectangular region from x-major ordered data.
    fn write_region(&mut self, region: GridRegion, cells: &[MaterialId]) {
        let dims = self.dims();
        assert!(dims.contains_region(region), "region must lie within solver dimensions");
        assert_eq!(
            cells.len(),
            region.cell_count(),
            "region write must provide exactly one cell per destination slot"
        );

        let [ox, oy, oz] = region.origin;
        let [sx, sy, sz] = region.size;
        let mut index = 0;

        for z in oz..oz + sz {
            for y in oy..oy + sy {
                for x in ox..ox + sx {
                    self.set(x as i32, y as i32, z as i32, cells[index]);
                    index += 1;
                }
            }
        }
    }

    /// Replace the full solver state from x-major ordered data.
    fn replace_cells(&mut self, cells: &[MaterialId]) {
        let dims = self.dims();
        assert_eq!(
            cells.len(),
            dims.cell_count(),
            "full-grid replacement must match solver dimensions"
        );
        self.write_region(dims.as_region(), cells);
    }
}
