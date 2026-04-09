# hyle-ca-core

Core types and traits for the [Hyle](https://github.com/walkingtheplanck/hyle) cellular automaton framework.

This crate defines the shared **contracts and descriptors**. Depend on it to implement custom cell types or build a new solver backend. It has **zero dependencies**.

## Key Types

| Type | Role |
|------|------|
| [`Cell`] | Trait for custom cell state (rule dispatch + alive/dead query) |
| [`CaSolver`] | Trait that all solver backends implement (get/set/step) |
| [`GridDims`] / [`GridRegion`] / [`GridSnapshot`] | Backend-neutral grid descriptors and bulk transfer types |
| [`NeighborhoodSpec`] | Declarative neighborhood description shared across backends |
| [`Rng`] | Deterministic per-cell RNG seeded from position and step count |
| [`Topology`] | Trait for boundary behavior policies used by solvers |
| [`BoundedTopology`] / [`TorusTopology`] | Built-in topology policies |
| [`TopologyDescriptor`] | Uploadable / serializable topology descriptor |
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

`rule_id()` selects which rule evaluates this cell. The built-in `u32`
implementation uses the low byte as rule ID and treats non-zero values as alive.

## Grid Descriptors

Backends share the same bulk-transfer descriptors:

```rust
use hyle_ca_core::{GridDims, GridRegion, GridSnapshot};

let dims = GridDims::new(8, 8, 8);
let region = GridRegion::new([2, 2, 2], [2, 2, 2]);
let snapshot = GridSnapshot::new(dims, vec![0u32; dims.cell_count()]);

assert!(dims.contains_region(region));
assert_eq!(snapshot.cells.len(), dims.cell_count());
```

CPU rule registration and runtime neighborhood buffers live in
[`hyle-ca-solver`](https://crates.io/crates/hyle-ca-solver), not in this crate.

## Declarative Neighborhoods

```rust
use hyle_ca_core::{NeighborhoodShape, NeighborhoodSpec, NeighborhoodWeight};

let spec = NeighborhoodSpec::new(
    2,
    NeighborhoodShape::Moore,
    NeighborhoodWeight::Unweighted,
);

assert_eq!(spec.radius, 2);
```

## Topology

Solvers can choose how coordinates beyond the grid bounds behave:

- [`BoundedTopology`] treats out-of-bounds coordinates as absent
- [`TorusTopology`] wraps coordinates across each axis

```rust
use hyle_ca_core::{AxisTopology, BoundedTopology, Topology, TopologyDescriptor, TorusTopology};

assert_eq!(
    BoundedTopology.descriptor(),
    TopologyDescriptor::uniform(AxisTopology::Bounded),
);
assert_eq!(
    TorusTopology.descriptor(),
    TopologyDescriptor::uniform(AxisTopology::Wrap),
);
```

## Implementing a Solver

Implement the [`CaSolver`] trait to create a custom backend (GPU, distributed, etc.):

```rust
use std::marker::PhantomData;

use hyle_ca_core::{BoundedTopology, CaSolver, Cell};

struct MySolver<C: Cell> {
    topology: BoundedTopology,
    _marker: PhantomData<C>,
}

impl<C: Cell> CaSolver<C> for MySolver<C> {
    type Topology = BoundedTopology;

    fn width(&self) -> u32 { todo!() }
    fn height(&self) -> u32 { todo!() }
    fn depth(&self) -> u32 { todo!() }
    fn topology(&self) -> &Self::Topology { &self.topology }
    fn get(&self, x: i32, y: i32, z: i32) -> C { todo!() }
    fn set(&mut self, x: i32, y: i32, z: i32, cell: C) { todo!() }
    fn step(&mut self) { todo!() }
    fn step_count(&self) -> u32 { todo!() }
    fn iter_cells(&self) -> Vec<(u32, u32, u32, C)> { todo!() }
}
```

Rule registration is **not** part of the trait - it's solver-specific. CPU solvers
take Rust closures, GPU solvers would take shader source.

Use [`ValidatedSolver`] to wrap your implementation and assert contracts in debug builds.
