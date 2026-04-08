//! Default CPU solver — double-buffered, single-threaded.

use hyle_ca_core::{
    moore, unweighted, Action, CaSolver, Cell, GridReader, GridWriter, Neighborhood, Rng, ShapeFn,
    WeightFn,
};

/// A boxed rule closure.
type BoxedRule<C> = Box<dyn Fn(&Neighborhood<C>, Rng) -> Action<C>>;

/// A per-cell rule with its pre-built neighborhood buffer.
struct RegisteredRule<C: Cell> {
    neighborhood: Neighborhood<C>,
    rule: BoxedRule<C>,
}

/// A boxed world pass closure.
type BoxedWorldPass<C> = Box<dyn Fn(&GridReader<C>, &mut GridWriter<C>)>;

/// Default 3D cellular automaton solver, generic over cell type `C`.
///
/// Rules are Rust closures — register them with `register_rule()`,
/// `register_rule_with_radius()`, or `register_rule_with_shape()`.
/// World passes run after all per-cell rules.
pub struct Solver<C: Cell = u32> {
    width: u32,
    height: u32,
    depth: u32,

    cells: Vec<C>,
    cells_next: Vec<C>,

    /// Per-cell rules indexed by cell type (0-255).
    rules: Vec<Option<RegisteredRule<C>>>,

    /// World passes, run in registration order after all per-cell rules.
    world_passes: Vec<BoxedWorldPass<C>>,

    step_count: u32,
}

impl<C: Cell> Solver<C> {
    /// Create a new solver filled with `C::default()`.
    pub fn new(width: u32, height: u32, depth: u32) -> Self {
        let n = (width * height * depth) as usize;
        let mut rules = Vec::with_capacity(256);
        rules.resize_with(256, || None);
        Solver {
            width,
            height,
            depth,
            cells: vec![C::default(); n],
            cells_next: vec![C::default(); n],
            rules,
            world_passes: Vec::new(),
            step_count: 0,
        }
    }

    /// Register a per-cell rule with radius 1 and Moore neighborhood (26 neighbors).
    pub fn register_rule(
        &mut self,
        cell_type: u8,
        rule: impl Fn(&Neighborhood<C>, Rng) -> Action<C> + 'static,
    ) {
        self.rules[cell_type as usize] = Some(RegisteredRule {
            neighborhood: Neighborhood::new(1, moore, unweighted),
            rule: Box::new(rule),
        });
    }

    /// Register a per-cell rule with a custom radius and Moore neighborhood.
    pub fn register_rule_with_radius(
        &mut self,
        cell_type: u8,
        radius: u32,
        rule: impl Fn(&Neighborhood<C>, Rng) -> Action<C> + 'static,
    ) {
        assert!(radius >= 1, "radius must be >= 1");
        self.rules[cell_type as usize] = Some(RegisteredRule {
            neighborhood: Neighborhood::new(radius, moore, unweighted),
            rule: Box::new(rule),
        });
    }

    /// Register a per-cell rule with a custom radius, shape, and weight.
    pub fn register_rule_with_shape(
        &mut self,
        cell_type: u8,
        radius: u32,
        shape: ShapeFn,
        weight: WeightFn,
        rule: impl Fn(&Neighborhood<C>, Rng) -> Action<C> + 'static,
    ) {
        assert!(radius >= 1, "radius must be >= 1");
        self.rules[cell_type as usize] = Some(RegisteredRule {
            neighborhood: Neighborhood::new(radius, shape, weight),
            rule: Box::new(rule),
        });
    }

    /// Register a world pass. Runs after all per-cell rules, in registration order.
    pub fn register_world_pass(
        &mut self,
        pass: impl Fn(&GridReader<C>, &mut GridWriter<C>) + 'static,
    ) {
        self.world_passes.push(Box::new(pass));
    }

    /// Evaluate per-cell rules.
    fn step_cell_rules(&mut self) {
        let w = self.width;
        let h = self.height;
        let d = self.depth;
        let step_count = self.step_count;

        // Take cells slice to avoid borrow conflicts with self.rules.
        let cells: &[C] = &self.cells;

        for z in 0..d as i32 {
            for y in 0..h as i32 {
                for x in 0..w as i32 {
                    let idx = (x as u32 + y as u32 * w + z as u32 * w * h) as usize;
                    let center = cells[idx];
                    let cell_type = center.rule_id() as usize;

                    let reg = match &mut self.rules[cell_type] {
                        Some(r) => r,
                        None => continue,
                    };

                    reg.neighborhood.fill(center, [x, y, z], |dx, dy, dz| {
                        let nx = x + dx;
                        let ny = y + dy;
                        let nz = z + dz;
                        if (nx as u32) >= w || (ny as u32) >= h || (nz as u32) >= d {
                            return C::default();
                        }
                        cells[(nx as u32 + ny as u32 * w + nz as u32 * w * h) as usize]
                    });

                    let action = (reg.rule)(
                        &reg.neighborhood,
                        Rng::new(x as u32, y as u32, z as u32, step_count),
                    );

                    if let Action::Become(c) = action {
                        self.cells_next[idx] = c;
                    }
                }
            }
        }
    }

    /// Run world passes sequentially over cells_next.
    fn step_world_passes(&mut self) {
        if self.world_passes.is_empty() {
            return;
        }

        let mut pass_read = self.cells_next.clone();

        for pass in &self.world_passes {
            let reader = GridReader::new(&pass_read, self.width, self.height, self.depth);
            let mut writer =
                GridWriter::new(&mut self.cells_next, self.width, self.height, self.depth);
            pass(&reader, &mut writer);
            pass_read.copy_from_slice(&self.cells_next);
        }
    }

    #[inline]
    fn idx(&self, x: u32, y: u32, z: u32) -> usize {
        (x + y * self.width + z * self.width * self.height) as usize
    }
}

impl<C: Cell> CaSolver<C> for Solver<C> {
    fn width(&self) -> u32 {
        self.width
    }
    fn height(&self) -> u32 {
        self.height
    }
    fn depth(&self) -> u32 {
        self.depth
    }

    fn get(&self, x: i32, y: i32, z: i32) -> C {
        if (x as u32) >= self.width || (y as u32) >= self.height || (z as u32) >= self.depth {
            return C::default();
        }
        self.cells[self.idx(x as u32, y as u32, z as u32)]
    }

    fn set(&mut self, x: i32, y: i32, z: i32, cell: C) {
        if (x as u32) >= self.width || (y as u32) >= self.height || (z as u32) >= self.depth {
            return;
        }
        let i = self.idx(x as u32, y as u32, z as u32);
        self.cells[i] = cell;
    }

    fn step(&mut self) {
        self.cells_next.copy_from_slice(&self.cells);
        self.step_cell_rules();
        self.step_world_passes();
        std::mem::swap(&mut self.cells, &mut self.cells_next);
        self.step_count += 1;
    }

    fn step_count(&self) -> u32 {
        self.step_count
    }

    fn iter_cells(&self) -> Vec<(u32, u32, u32, C)> {
        let w = self.width;
        let h = self.height;
        self.cells
            .iter()
            .enumerate()
            .map(move |(i, &c)| {
                let x = (i as u32) % w;
                let y = ((i as u32) / w) % h;
                let z = (i as u32) / (w * h);
                (x, y, z, c)
            })
            .collect()
    }
}
