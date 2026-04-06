//! CPU solver for the Hyle cellular automaton engine.
//!
//! Double-buffered, single-threaded simulation with three-tier rule system.

use hyle_ca_core::cell::{cell_rng, Action, Cell, GridReader, GridWriter, Neighborhood, Region};
use hyle_ca_core::{CaSolver, RegionalRule, Rule, WorldPass};

/// CPU-based 3D cellular automaton, generic over cell type `C`.
///
/// Supports three tiers of rules:
/// - **Local rules** (Tier 1): radius-1 neighborhood, cheapest.
/// - **Regional rules** (Tier 2): configurable radius, pre-fetched region.
/// - **World passes** (Tier 3): full grid access, runs as a separate stage.
///
/// Rule registration is CPU-specific (Rust `fn()` pointers).
pub struct Solver<C: Cell = u32> {
    width: u32,
    height: u32,
    depth: u32,

    cells: Vec<C>,
    cells_next: Vec<C>,

    rules: [Option<Rule<C>>; 256],
    regional_rules: [Option<(u32, RegionalRule<C>)>; 256],
    world_passes: Vec<WorldPass<C>>,

    step_count: u32,
}

impl<C: Cell> Solver<C> {
    /// Create a new world filled with `C::default()`.
    pub fn new(width: u32, height: u32, depth: u32) -> Self {
        let n = (width * height * depth) as usize;
        Solver {
            width,
            height,
            depth,
            cells: vec![C::default(); n],
            cells_next: vec![C::default(); n],
            rules: [None; 256],
            regional_rules: [None; 256],
            world_passes: Vec::new(),
            step_count: 0,
        }
    }

    /// Register a Tier 1 local rule (radius-1 neighborhood).
    pub fn register_rule(&mut self, cell_type: u8, rule: Rule<C>) {
        self.rules[cell_type as usize] = Some(rule);
    }

    /// Register a Tier 2 regional rule with a custom radius.
    /// Takes priority over any local rule for the same cell type.
    pub fn register_regional_rule(
        &mut self,
        cell_type: u8,
        radius: u32,
        rule: RegionalRule<C>,
    ) {
        assert!(radius >= 1, "radius must be >= 1; use register_rule() for radius 1");
        self.regional_rules[cell_type as usize] = Some((radius, rule));
    }

    /// Register a Tier 3 world pass. Runs after all per-cell rules, in order.
    pub fn register_world_pass(&mut self, pass: WorldPass<C>) {
        self.world_passes.push(pass);
    }

    /// Iterate all cells as `(x, y, z, cell)`.
    pub fn iter(&self) -> impl Iterator<Item = (u32, u32, u32, C)> + '_ {
        let w = self.width;
        let h = self.height;
        self.cells.iter().enumerate().map(move |(i, &c)| {
            let x = (i as u32) % w;
            let y = ((i as u32) / w) % h;
            let z = (i as u32) / (w * h);
            (x, y, z, c)
        })
    }

    /// Get cell without bounds checking. Caller must guarantee in-bounds.
    #[inline]
    unsafe fn get_unchecked(&self, x: u32, y: u32, z: u32) -> C {
        *self.cells.get_unchecked(self.idx(x, y, z))
    }

    /// Tier 1 + 2: evaluate per-cell rules (local and regional).
    fn step_cell_rules(&mut self) {
        let w = self.width as i32;
        let h = self.height as i32;
        let d = self.depth as i32;

        let max_radius = self.regional_rules.iter()
            .filter_map(|r| r.as_ref())
            .map(|(radius, _)| *radius)
            .max()
            .unwrap_or(0);

        let mut region_buf = if max_radius > 0 {
            Some(Region::new(max_radius))
        } else {
            None
        };

        for z in 0..d {
            for y in 0..h {
                for x in 0..w {
                    // SAFETY: x,y,z are in 0..w, 0..h, 0..d — always in bounds.
                    let center = unsafe { self.get_unchecked(x as u32, y as u32, z as u32) };
                    let cell_type = center.rule_id() as usize;
                    let rng = cell_rng(x as u32, y as u32, z as u32, self.step_count);

                    let action = if let Some((radius, rule)) = self.regional_rules[cell_type] {
                        let region = region_buf.as_mut().unwrap();
                        region.resize(radius);
                        region.fill(center, [x, y, z], |dx, dy, dz| {
                            self.get(x + dx, y + dy, z + dz)
                        });
                        rule(region, rng)
                    } else if let Some(rule) = self.rules[cell_type] {
                        let neighborhood = Neighborhood::build(center, |dx, dy, dz| {
                            self.get(x + dx, y + dy, z + dz)
                        });
                        rule(neighborhood, rng)
                    } else {
                        continue;
                    };

                    self.apply_action(action, center, x, y, z);
                }
            }
        }
    }

    /// Tier 3: run world passes sequentially over cells_next.
    fn step_world_passes(&mut self) {
        if self.world_passes.is_empty() {
            return;
        }

        let mut pass_read = self.cells_next.clone();

        for pass in &self.world_passes {
            let reader = GridReader::new(&pass_read, self.width, self.height, self.depth);
            let mut writer = GridWriter::new(
                &mut self.cells_next,
                self.width,
                self.height,
                self.depth,
            );
            pass(&reader, &mut writer);
            pass_read.copy_from_slice(&self.cells_next);
        }
    }

    /// Apply an action to cells_next.
    #[inline]
    fn apply_action(&mut self, action: Action<C>, center: C, x: i32, y: i32, z: i32) {
        let ci = self.idx(x as u32, y as u32, z as u32);
        match action {
            Action::Keep => {}
            Action::Become(c) => {
                self.cells_next[ci] = c;
            }
            Action::Swap(dir) => {
                let (dx, dy, dz) = dir.offset();
                let nx = x + dx;
                let ny = y + dy;
                let nz = z + dz;
                if (nx as u32) < self.width
                    && (ny as u32) < self.height
                    && (nz as u32) < self.depth
                {
                    let ni = self.idx(nx as u32, ny as u32, nz as u32);
                    let neighbor = self.cells[ni];
                    self.cells_next[ci] = neighbor;
                    self.cells_next[ni] = center;
                }
            }
            Action::Set(dir, c) => {
                let (dx, dy, dz) = dir.offset();
                let nx = x + dx;
                let ny = y + dy;
                let nz = z + dz;
                if (nx as u32) < self.width
                    && (ny as u32) < self.height
                    && (nz as u32) < self.depth
                {
                    let ni = self.idx(nx as u32, ny as u32, nz as u32);
                    self.cells_next[ni] = c;
                }
            }
        }
    }

    #[inline]
    fn idx(&self, x: u32, y: u32, z: u32) -> usize {
        (x + y * self.width + z * self.width * self.height) as usize
    }
}

impl<C: Cell> CaSolver<C> for Solver<C> {
    fn width(&self) -> u32 { self.width }
    fn height(&self) -> u32 { self.height }
    fn depth(&self) -> u32 { self.depth }

    fn get(&self, x: i32, y: i32, z: i32) -> C {
        if (x as u32) >= self.width
            || (y as u32) >= self.height
            || (z as u32) >= self.depth
        {
            return C::default();
        }
        self.cells[self.idx(x as u32, y as u32, z as u32)]
    }

    fn set(&mut self, x: i32, y: i32, z: i32, cell: C) {
        if (x as u32) >= self.width
            || (y as u32) >= self.height
            || (z as u32) >= self.depth
        {
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
}
