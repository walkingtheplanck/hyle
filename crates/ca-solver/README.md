# hyle-ca-solver

Default CPU solver for the [Hyle](https://github.com/walkingtheplanck/hyle) cellular automaton framework.

Double-buffered, single-threaded solver with a two-tier rule system: per-cell
rules (any radius) and world passes (full grid access). Depends on
[`hyle-ca-core`](https://crates.io/crates/hyle-ca-core) for traits and types.

## Quick Start

```rust
use hyle_ca_core::{Action, CaSolver, Neighborhood, Rng};
use hyle_ca_solver::Solver;

const ALIVE: u32 = 1;
const DEAD: u32 = 0;

let mut solver = Solver::<u32>::new(64, 64, 64);

// Birth rule: dead cells with exactly 5 alive neighbors come alive
solver.register_rule(DEAD as u8, |n: &Neighborhood<u32>, _rng: Rng| {
    match n.count_alive() {
        5 => Action::Become(ALIVE),
        _ => Action::Keep,
    }
});

// Survival rule: alive cells with 4-5 alive neighbors survive
solver.register_rule(ALIVE as u8, |n: &Neighborhood<u32>, _rng: Rng| {
    match n.count_alive() {
        4..=5 => Action::Keep,
        _ => Action::Become(DEAD),
    }
});

// Advance one step
solver.step();
```

## Topology

The default solver is bounded, but you can opt into torus wrapping:

```rust
use hyle_ca_core::TorusTopology;
use hyle_ca_solver::Solver;

let _solver = Solver::<u32>::with_topology(64, 64, 64, TorusTopology);
```

With `TorusTopology`, direct `get`/`set`, neighborhood sampling, and world
passes all wrap across grid edges.

## Registering Rules

Rules are Rust closures registered per cell type (keyed by `Cell::rule_id()`).
The default radius is 1 (26-cell Moore neighborhood):

```rust
use hyle_ca_core::{Action, Neighborhood, Rng};
use hyle_ca_solver::Solver;

let mut solver = Solver::<u32>::new(8, 8, 8);
let cell_type = 0u8;

solver.register_rule(cell_type, |neighborhood, rng| {
    let _ = (neighborhood, rng);
    Action::Keep // or Action::Become(new_cell)
});
```

For larger neighborhoods, use `register_rule_with_radius`:

```rust
use hyle_ca_core::{Action, Neighborhood, Rng};
use hyle_ca_solver::Solver;

let mut solver = Solver::<u32>::new(8, 8, 8);
let cell_type = 0u8;

// Radius 3 = 342 neighbors
solver.register_rule_with_radius(cell_type, 3, |n, rng| {
    let _ = rng;
    let far_cell = n.get(3, 0, 0);
    let _ = far_cell;
    Action::Keep
});
```

## Rule Sets

`RuleSet` lets you group related registrations under one name and install them
onto a solver in one call:

```rust
use hyle_ca_core::{Action, Neighborhood, Rng};
use hyle_ca_solver::{RuleSet, Solver};

let rules = RuleSet::new("life-4555")
    .rule(1, |n: &Neighborhood<u32>, _rng: Rng| match n.count_alive() {
        4..=5 => Action::Keep,
        _ => Action::Become(0),
    })
    .rule(0, |n: &Neighborhood<u32>, _rng: Rng| match n.count_alive() {
        5 => Action::Become(1),
        _ => Action::Keep,
    })
    .world_pass(|grid, out| {
        let alive = grid.iter().filter(|(_, _, _, c)| *c != 0).count() as u32;
        out.set(0, 0, 0, alive);
    });

let mut solver = Solver::<u32>::new(64, 64, 64);
solver.install_rule_set(rules);
```

Installing a rule set keeps the existing low-level semantics:
- later registrations still override earlier rules for the same cell type
- world passes are appended in order

## World Passes

World passes run after all per-cell rules, in registration order. They receive
read-only access to the post-rule grid and write-only access to the output:

```rust
use hyle_ca_solver::Solver;

let mut solver = Solver::<u32>::new(4, 4, 4);

solver.register_world_pass(|grid, out| {
    for (x, y, z, cell) in grid.iter() {
        out.set(x as i32, y as i32, z as i32, cell);
    }
});
```

## How It Works

Each call to `step()`:

1. Copies the current buffer to the next buffer
2. Evaluates per-cell rules - reads from current, writes to next
3. Runs world passes sequentially over the next buffer
4. Swaps buffers and increments the step counter

Rules are **order-independent**: the double-buffer design ensures that evaluation
order never affects the result. Rules can safely be parallelized in the future.
