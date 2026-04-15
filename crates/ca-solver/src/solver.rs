//! Default CPU solver - double-buffered, single-threaded.

use hyle_ca_interface::semantics::{interpret_blueprint, ResolvedBlueprint};
use hyle_ca_interface::{
    AttributeAccessError, AttributeId, AttributeValue, Blueprint, CaSolver, GridRegion,
    GridSnapshot, Instance, MaterialId, RuleEffect, Topology,
};

use crate::attributes::AttributeStore;
use crate::grid::{resolve_index, Grid};
use crate::program::CompiledProgram;
use crate::{BoundedTopology, DescriptorTopology};

/// Default 3D cellular blueprint solver.
pub struct Solver<T: Topology = BoundedTopology> {
    grid: Grid,
    attributes: AttributeStore,
    material_defaults: Vec<Vec<AttributeValue>>,
    topology: T,
    program: Option<CompiledProgram>,
    step_count: u32,
    seed: u64,
}

impl Solver<BoundedTopology> {
    /// Create a new bounded solver filled with the default material.
    pub fn new(width: u32, height: u32, depth: u32) -> Self {
        Self::with_instance_and_topology(
            Instance::new(width, height, depth),
            BoundedTopology,
            MaterialId::default(),
        )
    }

    /// Create a new solver filled with the default material and the given topology.
    pub fn with_topology<U: Topology>(
        width: u32,
        height: u32,
        depth: u32,
        topology: U,
    ) -> Solver<U> {
        Self::with_instance_and_topology(
            Instance::new(width, height, depth),
            topology,
            MaterialId::default(),
        )
    }

    /// Create a new solver from a runtime instance and topology policy.
    pub fn with_instance_and_topology<U: Topology>(
        instance: Instance,
        topology: U,
        default_material: MaterialId,
    ) -> Solver<U> {
        Solver {
            grid: Grid::new(
                instance.dims().width,
                instance.dims().height,
                instance.dims().depth,
                default_material,
            ),
            attributes: AttributeStore::new(instance.dims().cell_count() + 1, &[]),
            material_defaults: Vec::new(),
            topology,
            program: None,
            step_count: 0,
            seed: instance.seed(),
        }
    }
}

impl Solver<DescriptorTopology> {
    /// Create a solver whose topology and rules come from an interpreted blueprint.
    pub fn from_blueprint(
        width: u32,
        height: u32,
        depth: u32,
        blueprint: &ResolvedBlueprint,
    ) -> Self {
        Self::from_blueprint_instance(Instance::new(width, height, depth), blueprint)
    }

    /// Create a solver from a runtime instance and interpreted blueprint.
    pub fn from_blueprint_instance(instance: Instance, blueprint: &ResolvedBlueprint) -> Self {
        let material_defaults = compile_material_defaults(blueprint);
        let default_material = blueprint.default_material();

        let mut solver = Solver {
            grid: Grid::new(
                instance.dims().width,
                instance.dims().height,
                instance.dims().depth,
                default_material,
            ),
            attributes: AttributeStore::new(
                instance.dims().cell_count() + 1,
                blueprint.attributes(),
            ),
            material_defaults,
            topology: DescriptorTopology::new(blueprint.topology().descriptor()),
            program: Some(CompiledProgram::from_blueprint(blueprint)),
            step_count: 0,
            seed: instance.seed(),
        };

        // Ensure freshly allocated cells use the blueprint defaults for the default material.
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

    /// Interpret a declarative blueprint and create a solver from it.
    pub fn from_spec(width: u32, height: u32, depth: u32, blueprint: &Blueprint) -> Self {
        Self::from_spec_instance(Instance::new(width, height, depth), blueprint)
    }

    /// Interpret a declarative blueprint and create a solver from a runtime instance.
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
            material_defaults: self.material_defaults,
            topology,
            program: self.program,
            step_count: self.step_count,
            seed: self.seed,
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
    fn step_program(&mut self) {
        let program = match &mut self.program {
            Some(program) => program,
            None => return,
        };

        let dims = self.grid.dims();
        let width = dims.width;
        let height = dims.height;
        let depth = dims.depth;
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
                            if let Some(defaults) = self.material_defaults.get(target_material.index())
                            {
                                self.attributes.reset_next_to_defaults(idx, defaults);
                            }
                        }

                        for update in evaluation.attribute_updates {
                            self.attributes.set_next(update.attribute, idx, update.value);
                        }
                    }
                }
            }
        }
    }
}

impl<T: Topology> CaSolver for Solver<T> {
    type Topology = T;

    fn width(&self) -> u32 {
        self.grid.width
    }

    fn height(&self) -> u32 {
        self.grid.height
    }

    fn depth(&self) -> u32 {
        self.grid.depth
    }

    fn topology(&self) -> &Self::Topology {
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

        self.attributes.set_current(attribute, index, value);
        Ok(())
    }

    fn step(&mut self) {
        self.grid.prepare_step();
        self.attributes.prepare_step();
        self.step_program();
        self.grid.swap();
        self.attributes.swap();
        self.step_count += 1;
    }

    fn step_count(&self) -> u32 {
        self.step_count
    }

    fn iter_cells(&self) -> Vec<(u32, u32, u32, MaterialId)> {
        self.grid.iter_cells()
    }

    fn readback(&self) -> GridSnapshot<MaterialId> {
        GridSnapshot::new(
            self.grid.dims(),
            self.grid.cells[..self.grid.cell_count()].to_vec(),
        )
    }

    fn read_region(&self, region: GridRegion) -> Vec<MaterialId> {
        let dims = self.grid.dims();
        assert!(dims.contains_region(region), "region must lie within solver dimensions");

        let [ox, oy, oz] = region.origin;
        let [sx, sy, sz] = region.size;
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

        cells
    }

    fn write_region(&mut self, region: GridRegion, cells: &[MaterialId]) {
        let dims = self.grid.dims();
        assert!(dims.contains_region(region), "region must lie within solver dimensions");
        assert_eq!(
            cells.len(),
            region.cell_count(),
            "region write must provide exactly one material per destination slot"
        );

        let [ox, oy, oz] = region.origin;
        let [sx, sy, sz] = region.size;
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
