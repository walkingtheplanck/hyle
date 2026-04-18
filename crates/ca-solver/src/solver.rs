//! Default CPU solver - double-buffered, single-threaded.

use std::collections::BTreeMap;
use std::sync::Arc;

use hyle_ca_interface::resolved::{interpret_blueprint, ResolvedBlueprint};
use hyle_ca_interface::{
    AttributeAccessError, AttributeDef, AttributeId, AttributeValue, Blueprint,
    CellAttributeValue, CellId, CellQueryError, GridAccessError, GridDims, GridRegion,
    GridSnapshot, Instance, MaterialDef, MaterialId, NeighborhoodId, NeighborhoodSpec,
    RuleEffect, SolverAttributes, SolverCells, SolverExecution, SolverGrid, SolverMetadata,
    SolverMetrics, Topology, TransitionCount,
};

use crate::attributes::AttributeStore;
use crate::grid::{resolve_index, Grid};
use crate::program::CompiledProgram;
use crate::{BoundedTopology, DescriptorTopology};

struct RuntimeSchema {
    resolved: ResolvedBlueprint,
    neighborhood_specs: Vec<NeighborhoodSpec>,
}

/// Default 3D cellular schema solver.
pub struct Solver<T: Topology = BoundedTopology> {
    grid: Grid,
    attributes: AttributeStore,
    schema: Option<Arc<RuntimeSchema>>,
    material_defaults: Vec<Vec<AttributeValue>>,
    topology: T,
    program: Option<CompiledProgram>,
    step_count: u32,
    seed: u64,
    last_changed_cells: u64,
    last_transitions: Vec<TransitionCount>,
}

impl Solver<BoundedTopology> {
    /// Create a new bounded solver filled with the default material.
    pub fn new(width: u32, height: u32, depth: u32) -> Self {
        let grid = Grid::new(width, height, depth, MaterialId::default());
        let dims = grid.dims();
        Solver {
            grid,
            attributes: AttributeStore::new(dims.cell_count() + 1, &[]),
            schema: None,
            material_defaults: Vec::new(),
            topology: BoundedTopology,
            program: None,
            step_count: 0,
            seed: 0,
            last_changed_cells: 0,
            last_transitions: Vec::new(),
        }
    }

    /// Create a new solver filled with the default material and the given topology.
    pub fn with_topology<U: Topology>(
        width: u32,
        height: u32,
        depth: u32,
        topology: U,
    ) -> Solver<U> {
        let grid = Grid::new(width, height, depth, MaterialId::default());
        let dims = grid.dims();
        Solver {
            grid,
            attributes: AttributeStore::new(dims.cell_count() + 1, &[]),
            schema: None,
            material_defaults: Vec::new(),
            topology,
            program: None,
            step_count: 0,
            seed: 0,
            last_changed_cells: 0,
            last_transitions: Vec::new(),
        }
    }

    /// Create a new solver from a runtime instance and topology policy.
    pub fn with_instance_and_topology<U: Topology>(
        instance: Instance,
        topology: U,
        default_material: MaterialId,
    ) -> Solver<U> {
        Solver {
            grid: Grid::new(
                instance.dims().width(),
                instance.dims().height(),
                instance.dims().depth(),
                default_material,
            ),
            attributes: AttributeStore::new(instance.dims().cell_count() + 1, &[]),
            schema: None,
            material_defaults: Vec::new(),
            topology,
            program: None,
            step_count: 0,
            seed: instance.seed(),
            last_changed_cells: 0,
            last_transitions: Vec::new(),
        }
    }
}

impl Solver<DescriptorTopology> {
    /// Create a solver whose topology and rules come from an interpreted schema.
    pub fn from_blueprint(
        width: u32,
        height: u32,
        depth: u32,
        blueprint: &ResolvedBlueprint,
    ) -> Self {
        let schema = Arc::new(RuntimeSchema {
            resolved: blueprint.clone(),
            neighborhood_specs: blueprint
                .neighborhoods()
                .iter()
                .map(|item| item.spec())
                .collect(),
        });
        let material_defaults = compile_material_defaults(&schema.resolved);
        let default_material = schema.resolved.default_material();
        let grid = Grid::new(width, height, depth, default_material);
        let dims = grid.dims();

        let mut solver = Solver {
            grid,
            attributes: AttributeStore::new(dims.cell_count() + 1, schema.resolved.attributes()),
            schema: Some(schema),
            material_defaults,
            topology: DescriptorTopology::new(blueprint.topology().descriptor()),
            program: Some(CompiledProgram::from_blueprint(blueprint)),
            step_count: 0,
            seed: 0,
            last_changed_cells: 0,
            last_transitions: Vec::new(),
        };

        if let Some(defaults) = solver.material_defaults.get(default_material.index()).cloned() {
            for index in 0..solver.grid.cell_count() {
                solver.attributes.reset_next_to_defaults(index, &defaults);
            }
            solver.attributes.swap();
        }

        solver
    }

    /// Create a solver from a runtime instance and interpreted schema.
    pub fn from_blueprint_instance(instance: Instance, blueprint: &ResolvedBlueprint) -> Self {
        let schema = Arc::new(RuntimeSchema {
            resolved: blueprint.clone(),
            neighborhood_specs: blueprint
                .neighborhoods()
                .iter()
                .map(|item| item.spec())
                .collect(),
        });
        let material_defaults = compile_material_defaults(&schema.resolved);
        let default_material = schema.resolved.default_material();

        let mut solver = Solver {
            grid: Grid::new(
                instance.dims().width(),
                instance.dims().height(),
                instance.dims().depth(),
                default_material,
            ),
            attributes: AttributeStore::new(
                instance.dims().cell_count() + 1,
                schema.resolved.attributes(),
            ),
            schema: Some(schema),
            material_defaults,
            topology: DescriptorTopology::new(blueprint.topology().descriptor()),
            program: Some(CompiledProgram::from_blueprint(blueprint)),
            step_count: 0,
            seed: instance.seed(),
            last_changed_cells: 0,
            last_transitions: Vec::new(),
        };

        // Ensure freshly allocated cells use the schema defaults for the default material.
        if let Some(defaults) = solver
            .material_defaults
            .get(default_material.index())
            .cloned()
        {
            for index in 0..solver.grid.cell_count() {
                solver.attributes.reset_next_to_defaults(index, &defaults);
            }
            solver.attributes.swap();
        }

        solver
    }

    /// Interpret a declarative schema and create a solver from it.
    pub fn from_spec(width: u32, height: u32, depth: u32, blueprint: &Blueprint) -> Self {
        let resolved = interpret_blueprint(blueprint);
        Self::from_blueprint(width, height, depth, &resolved)
    }

    /// Interpret a declarative schema and create a solver from a runtime instance.
    pub fn from_spec_instance(instance: Instance, blueprint: &Blueprint) -> Self {
        let resolved = interpret_blueprint(blueprint);
        Self::from_blueprint_instance(instance, &resolved)
    }
}

impl<T: Topology> Solver<T> {
    /// Convert the solver to a new topology policy without changing its state.
    pub fn into_topology<U: Topology>(self, topology: U) -> Solver<U> {
        Solver {
            grid: self.grid,
            attributes: self.attributes,
            schema: self.schema,
            material_defaults: self.material_defaults,
            topology,
            program: self.program,
            step_count: self.step_count,
            seed: self.seed,
            last_changed_cells: self.last_changed_cells,
            last_transitions: self.last_transitions,
        }
    }

    /// The active topology policy used by this solver.
    pub fn topology(&self) -> &T {
        &self.topology
    }

    /// Replace the topology policy while preserving the solver state.
    pub fn set_topology<U: Topology>(self, topology: U) -> Solver<U> {
        self.into_topology(topology)
    }
}

impl<T: Topology> Solver<T> {
    fn record_step_metrics(&mut self) {
        let mut transitions = BTreeMap::<(u16, u16), u64>::new();
        self.last_changed_cells = 0;

        for index in 0..self.grid.cell_count() {
            let before = self.grid.cells[index];
            let after = self.grid.cells_next[index];

            if before != after {
                self.last_changed_cells += 1;
                *transitions.entry((before.raw(), after.raw())).or_default() += 1;
            }
        }

        self.last_transitions = transitions
            .into_iter()
            .map(|((from, to), count)| TransitionCount {
                from: MaterialId::new(from),
                to: MaterialId::new(to),
                count,
            })
            .collect();
    }

    fn decode_cell(&self, cell: CellId) -> Option<[u32; 3]> {
        if cell.raw() as usize >= self.grid.cell_count() {
            return None;
        }

        let width = self.grid.width as usize;
        let height = self.grid.height as usize;
        let index = cell.raw() as usize;
        let x = index % width;
        let y = (index / width) % height;
        let z = index / (width * height);
        Some([x as u32, y as u32, z as u32])
    }

    fn step_program(&mut self) {
        let program = match &mut self.program {
            Some(program) => program,
            None => return,
        };

        let dims = self.grid.dims();
        let width = dims.width();
        let height = dims.height();
        let depth = dims.depth();
        let guard_idx = self.grid.guard_idx();
        let topology = &self.topology;
        let resolve = |x, y, z| resolve_index(topology, dims, guard_idx, x, y, z);
        let cells: &[MaterialId] = &self.grid.cells;

        for z in 0..depth as i32 {
            for y in 0..height as i32 {
                for x in 0..width as i32 {
                    let idx = (x as u32 + y as u32 * width + z as u32 * width * height) as usize;
                    let center = cells[idx];
                    let evaluation = program.evaluate(
                        center,
                        [x, y, z],
                        self.step_count,
                        self.seed,
                        |dx, dy, dz| cells[resolve(x + dx, y + dy, z + dz)],
                        |attribute| self.attributes.get(attribute, idx),
                    );

                    if let Some(evaluation) = evaluation {
                        let target_material = match evaluation.effect {
                            RuleEffect::Keep => center,
                            RuleEffect::Become(material) => material,
                        };

                        if target_material != center {
                            self.grid.cells_next[idx] = target_material;
                            if let Some(defaults) =
                                self.material_defaults.get(target_material.index())
                            {
                                self.attributes.reset_next_to_defaults(idx, defaults);
                            }
                        }

                        for update in evaluation.attribute_updates {
                            self.attributes
                                .set_next(update.attribute, idx, update.value);
                        }
                    }
                }
            }
        }
    }
}

impl<T: Topology> SolverExecution for Solver<T> {
    type Topology = T;

    fn dims(&self) -> GridDims {
        self.grid.dims()
    }

    fn width(&self) -> u32 {
        self.grid.width
    }

    fn height(&self) -> u32 {
        self.grid.height
    }

    fn depth(&self) -> u32 {
        self.grid.depth
    }

    fn topology(&self) -> &<Self as SolverExecution>::Topology {
        &self.topology
    }

    fn seed(&self) -> u64 {
        self.seed
    }

    fn cell_count(&self) -> usize {
        self.grid.cell_count()
    }

    fn guard_index(&self) -> usize {
        self.grid.guard_idx()
    }

    fn get(&self, x: i32, y: i32, z: i32) -> MaterialId {
        self.grid.get(&self.topology, x, y, z)
    }

    fn set(&mut self, x: i32, y: i32, z: i32, material: MaterialId) {
        self.grid.set(&self.topology, x, y, z, material);
    }

    fn get_attr(
        &self,
        attribute: AttributeId,
        x: i32,
        y: i32,
        z: i32,
    ) -> Result<AttributeValue, AttributeAccessError> {
        if !self.attributes.contains(attribute) {
            return Err(AttributeAccessError::UnknownAttribute(attribute));
        }

        let index = self.grid.resolve_idx(&self.topology, x, y, z);
        if index == self.grid.guard_idx() {
            Err(AttributeAccessError::OutOfBounds { x, y, z })
        } else {
            Ok(self.attributes.get(attribute, index))
        }
    }

    fn set_attr(
        &mut self,
        attribute: AttributeId,
        x: i32,
        y: i32,
        z: i32,
        value: AttributeValue,
    ) -> Result<(), AttributeAccessError> {
        if !self.attributes.contains(attribute) {
            return Err(AttributeAccessError::UnknownAttribute(attribute));
        }

        let index = self.grid.resolve_idx(&self.topology, x, y, z);
        if index == self.grid.guard_idx() {
            return Err(AttributeAccessError::OutOfBounds { x, y, z });
        }

        self.attributes
            .set_current(attribute, index, value)
            .map_err(|(expected, actual)| AttributeAccessError::TypeMismatch {
                attribute,
                expected,
                actual,
            })?;
        Ok(())
    }

    fn step(&mut self) {
        self.grid.prepare_step();
        self.attributes.prepare_step();
        self.step_program();
        self.record_step_metrics();
        self.grid.swap();
        self.attributes.swap();
        self.step_count += 1;
    }

    fn step_count(&self) -> u32 {
        self.step_count
    }
}

impl<T: Topology> SolverMetadata for Solver<T> {
    fn material_defs(&self) -> &[MaterialDef] {
        self.schema
            .as_ref()
            .map(|schema| schema.resolved.materials())
            .unwrap_or(&[])
    }

    fn attribute_defs(&self) -> &[AttributeDef] {
        self.schema
            .as_ref()
            .map(|schema| schema.resolved.attributes())
            .unwrap_or(&[])
    }

    fn neighborhood_specs(&self) -> &[NeighborhoodSpec] {
        self.schema
            .as_ref()
            .map(|schema| schema.neighborhood_specs.as_slice())
            .unwrap_or(&[])
    }
}

impl<T: Topology> SolverCells for Solver<T> {
    fn cell_position(&self, cell: CellId) -> Result<[u32; 3], CellQueryError> {
        self.decode_cell(cell)
            .ok_or(CellQueryError::UnknownCell(cell))
    }

    fn material(&self, cell: CellId) -> Result<MaterialId, CellQueryError> {
        self.decode_cell(cell)
            .map(|_| self.grid.cells[cell.raw() as usize])
            .ok_or(CellQueryError::UnknownCell(cell))
    }

    fn neighbors(
        &self,
        cell: CellId,
        neighborhood: NeighborhoodId,
    ) -> Result<Vec<CellId>, CellQueryError> {
        let schema = self
            .schema
            .as_ref()
            .ok_or(CellQueryError::SchemaUnavailable)?;
        let interpreted = schema
            .resolved
            .neighborhoods()
            .iter()
            .find(|item| item.spec().id() == neighborhood)
            .ok_or(CellQueryError::UnknownNeighborhood(neighborhood))?;
        let [x, y, z] = self
            .decode_cell(cell)
            .ok_or(CellQueryError::UnknownCell(cell))?;
        let mut cells = Vec::new();

        for offset in interpreted.neighborhood().offsets() {
            let neighbor_idx = self.grid.resolve_idx(
                &self.topology,
                x as i32 + offset.dx,
                y as i32 + offset.dy,
                z as i32 + offset.dz,
            );
            if neighbor_idx != self.grid.guard_idx() {
                cells.push(CellId::new(neighbor_idx as u32));
            }
        }

        Ok(cells)
    }
}

impl<T: Topology> SolverAttributes for Solver<T> {
    fn attribute(
        &self,
        cell: CellId,
        attribute: AttributeId,
    ) -> Result<AttributeValue, CellQueryError> {
        if !self.attributes.contains(attribute) {
            return Err(CellQueryError::from(
                AttributeAccessError::UnknownAttribute(attribute),
            ));
        }
        if cell.raw() as usize >= self.grid.cell_count() {
            return Err(CellQueryError::UnknownCell(cell));
        }

        Ok(self.attributes.get(attribute, cell.raw() as usize))
    }

    fn attributes(&self, cell: CellId) -> Result<Vec<CellAttributeValue>, CellQueryError> {
        if cell.raw() as usize >= self.grid.cell_count() {
            return Err(CellQueryError::UnknownCell(cell));
        }

        Ok(self
            .attribute_defs()
            .iter()
            .map(|attribute| {
                CellAttributeValue::new(
                    attribute.id,
                    self.attributes.get(attribute.id, cell.raw() as usize),
                )
            })
            .collect())
    }
}

impl<T: Topology> SolverMetrics for Solver<T> {
    fn last_changed_cells(&self) -> u64 {
        self.last_changed_cells
    }

    fn populations(&self) -> Vec<u64> {
        let mut populations = Vec::new();
        for material in self.grid.cells.iter().take(self.grid.cell_count()).copied() {
            if populations.len() <= material.index() {
                populations.resize(material.index() + 1, 0);
            }
            populations[material.index()] += 1;
        }
        populations
    }

    fn last_transitions(&self) -> &[TransitionCount] {
        &self.last_transitions
    }
}

impl<T: Topology> SolverGrid for Solver<T> {
    fn iter_cells(&self) -> Vec<(u32, u32, u32, MaterialId)> {
        self.grid.iter_cells()
    }

    fn readback(&self) -> GridSnapshot<MaterialId> {
        GridSnapshot::new(
            self.grid.dims(),
            self.grid.cells[..self.grid.cell_count()].to_vec(),
        )
    }

    fn read_region(&self, region: GridRegion) -> Result<Vec<MaterialId>, GridAccessError> {
        let dims = self.grid.dims();
        if !dims.contains_region(region) {
            return Err(GridAccessError::RegionOutOfBounds { region, dims });
        }

        let [ox, oy, oz] = region.origin();
        let [sx, sy, sz] = region.size();
        let width = self.grid.width as usize;
        let height = self.grid.height as usize;
        let mut cells = Vec::with_capacity(region.cell_count());

        for z in oz..oz + sz {
            for y in oy..oy + sy {
                for x in ox..ox + sx {
                    let index = (x as usize) + (y as usize) * width + (z as usize) * width * height;
                    cells.push(self.grid.cells[index]);
                }
            }
        }

        Ok(cells)
    }

    fn write_region(
        &mut self,
        region: GridRegion,
        cells: &[MaterialId],
    ) -> Result<(), GridAccessError> {
        let dims = self.grid.dims();
        if !dims.contains_region(region) {
            return Err(GridAccessError::RegionOutOfBounds { region, dims });
        }
        if cells.len() != region.cell_count() {
            return Err(GridAccessError::CellCountMismatch {
                expected: region.cell_count(),
                actual: cells.len(),
            });
        }

        let [ox, oy, oz] = region.origin();
        let [sx, sy, sz] = region.size();
        let width = self.grid.width as usize;
        let height = self.grid.height as usize;
        let mut source = 0;

        for z in oz..oz + sz {
            for y in oy..oy + sy {
                for x in ox..ox + sx {
                    let index = (x as usize) + (y as usize) * width + (z as usize) * width * height;
                    self.grid.cells[index] = cells[source];
                    self.grid.cells_next[index] = cells[source];
                    source += 1;
                }
            }
        }

        Ok(())
    }
}

fn compile_material_defaults(blueprint: &ResolvedBlueprint) -> Vec<Vec<AttributeValue>> {
    let base_defaults = blueprint
        .attributes()
        .iter()
        .map(|attribute| AttributeValue::zero(attribute.value_type))
        .collect::<Vec<_>>();

    let mut defaults = vec![base_defaults; blueprint.materials().len()];
    for material in blueprint.materials() {
        let row = &mut defaults[material.id.index()];
        for binding in &material.attributes {
            row[binding.attribute.index()] = binding.default;
        }
    }
    defaults
}
