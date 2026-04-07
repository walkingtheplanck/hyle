# hyle-ca-core

Core types and traits for the [Hyle](https://github.com/walkingtheplanck/hyle) cellular automaton framework.

This crate defines the **interface** — depend on it to write rules, implement custom cell types, or build a new solver backend. It has **zero dependencies**.

## Key Types

| Type | Role |
|------|------|
| [`Cell`] | Trait for custom cell state (rule dispatch + alive/dead query) |
| [`CaSolver`] | Trait that all solver backends implement (get/set/step) |
| [`Action`] | What a rule returns: `Keep` the current cell or `Become(new_cell)` |
| [`Neighborhood`] | Pre-fetched cube of neighbors passed to rules (any radius) |
| [`Rng`] | Deterministic per-cell RNG seeded from position and step count |
| [`GridReader`] / [`GridWriter`] | Read-only and write-only grid views for world passes |
| [`ValidatedSolver`] | Debug wrapper that asserts solver contracts on every call |

## Defining a Custom Cell

```rust
use hyle_ca_core::Cell;

#[derive(Copy, Clone, Default)]
struct FluidCell {
    density: u8,
    velocity: [i8; 6],
    material: u8,
}

impl Cell for FluidCell {
    fn rule_id(&self) -> u8 { self.material }
    fn is_alive(&self) -> bool { self.density > 0 }
}
```

`rule_id()` selects which rule evaluates this cell. `is_alive()` is used by
`Neighborhood::count_alive()`. The built-in `u32` implementation uses the low
byte as rule ID and treats non-zero values as alive.

## Writing Rules

Rules are closures that receive a [`Neighborhood`] and [`Rng`], and return an [`Action`]:

```rust,ignore
use hyle_ca_core::{Action, Neighborhood, Rng};

// 3D Game of Life (Life 4555): survive with 4-5 neighbors, birth with 5
fn alive_rule(n: &dyn Neighborhood<u32>, _rng: Rng) -> Action<u32> {
    match n.count_alive() {
        4..=5 => Action::Keep,
        _ => Action::Become(0),
    }
}
```

Rules are **order-independent** — the solver reads from one buffer and writes to
another, so evaluation order never affects the result.

## World Passes

World passes run after all per-cell rules and have full grid access. Use them
for global operations like pressure solving, gravity, or conservation correction:

```rust,ignore
use hyle_ca_core::{GridReader, GridWriter};

fn gravity_pass(grid: &GridReader<u32>, out: &mut GridWriter<u32>) {
    for (x, y, z, cell) in grid.iter() {
        // GridWriter has no get() — you cannot read your own output,
        // preventing order-dependent bugs.
        out.set(x as i32, y as i32, z as i32, cell);
    }
}
```

## Implementing a Solver

Implement the [`CaSolver`] trait to create a custom backend (GPU, distributed, etc.):

```rust,ignore
use hyle_ca_core::{CaSolver, Cell};

struct MySolver<C: Cell> { /* ... */ }

impl<C: Cell> CaSolver<C> for MySolver<C> {
    fn width(&self) -> u32 { todo!() }
    fn height(&self) -> u32 { todo!() }
    fn depth(&self) -> u32 { todo!() }
    fn get(&self, x: i32, y: i32, z: i32) -> C { todo!() }
    fn set(&mut self, x: i32, y: i32, z: i32, cell: C) { todo!() }
    fn step(&mut self) { todo!() }
    fn step_count(&self) -> u32 { todo!() }
    fn iter_cells(&self) -> Vec<(u32, u32, u32, C)> { todo!() }
}
```

Rule registration is **not** part of the trait — it's solver-specific. CPU solvers
take Rust closures, GPU solvers would take shader source.

Use [`ValidatedSolver`] to wrap your implementation and assert contracts in debug builds.
