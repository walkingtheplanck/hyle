# Hyle

[![docs.rs: hyle-ca-core](https://img.shields.io/docsrs/hyle-ca-core?label=hyle-ca-core%20docs)](https://docs.rs/hyle-ca-core)
[![docs.rs: hyle-ca-solver](https://img.shields.io/docsrs/hyle-ca-solver?label=hyle-ca-solver%20docs)](https://docs.rs/hyle-ca-solver)

A 3D cellular automaton framework for Rust.

> Define rules as closures, register them, step the simulation.
> Supports custom cell types, variable-radius neighborhoods, torus topology, and world passes.

---

## Crates

| Crate | Purpose |
|-------|---------|
| [`hyle-ca-core`](crates/ca-core) | Traits and types - depend on this to write rules or a custom solver |
| [`hyle-ca-solver`](crates/ca-solver) | Default CPU solver (double-buffered) - depend on this to run automata |

---

## Quick Start

```rust
use hyle_ca_core::{Action, Neighborhood, Rng};
use hyle_ca_solver::Solver;

const ALIVE: u32 = 1;
const DEAD: u32 = 0;

let mut solver = Solver::<u32>::new(64, 64, 64);

solver.register_rule(DEAD as u8, |n: &Neighborhood<u32>, _rng: Rng| {
    match n.count_alive() {
        5 => Action::Become(ALIVE),
        _ => Action::Keep,
    }
});

solver.register_rule(ALIVE as u8, |n: &Neighborhood<u32>, _rng: Rng| {
    match n.count_alive() {
        4..=5 => Action::Keep,
        _ => Action::Become(DEAD),
    }
});

solver.step();
```

---

## Features

### Custom Cell Types

```rust
#[derive(Copy, Clone, Default)]
struct FluidCell { density: u8, velocity: [i8; 6], material: u8 }

impl Cell for FluidCell {
    fn rule_id(&self) -> u8 { self.material }
    fn is_alive(&self) -> bool { self.density > 0 }
}

let solver = Solver::<FluidCell>::new(64, 64, 64);
```

### Variable-Radius Neighborhoods

```rust
// Radius 3 = 342 neighbors instead of 26
solver.register_rule_with_radius(0, 3, |n, rng| {
    let far_cell = n.get(3, 0, 0);  // within radius
    Action::Keep
});
```

### Torus Topology

```rust
use hyle_ca_core::TorusTopology;

let solver = Solver::<u32>::with_topology(64, 64, 64, TorusTopology);
```

Reads, writes, rule neighborhoods, and world passes all wrap across grid edges.

### World Passes

```rust
// Full grid access - runs after all per-cell rules
solver.register_world_pass(|grid, out| {
    for (x, y, z, cell) in grid.iter() {
        out.set(x as i32, y as i32, z as i32, cell);
    }
});
```

### Debug Contract Validation

```rust
use hyle_ca_core::ValidatedSolver;

// Wraps any solver, asserts contracts on every call (debug builds only)
let validated = ValidatedSolver::new(solver);
```

---

## Viewer

```
cargo run --release -p hyle-viewer
```

3D Game of Life (Life 4555) with GPU raytracing.

| Input | Action |
|---|---|
| Right-drag | Orbit camera |
| Middle-drag | Pan |
| Scroll | Zoom |
| WASD / QE | Move camera |
| R | Reset |
| Tab | Toggle mouse capture |

---

## Roadmap

- [x] **Neighborhood types** - Moore, Von Neumann, Spherical shapes + configurable weight functions
- [x] **Global torus topology** - Wrapping boundaries for the whole solver grid
- [ ] **Regional topology control** - Let wrapping and non-wrapping cells coexist in the same world
- [ ] **Pattern serialization** - Save/load grid state
- [ ] **Chunk-based sparse storage** - Skip empty regions, scale to large grids
- [ ] **Analysis tools** - Population counts, entropy, step statistics
- [ ] **Parallel stepping** - Rayon for CPU, leveraging the existing double-buffer design

---

## License

GPL-3.0-only
