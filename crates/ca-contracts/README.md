# hyle-ca-contracts

Shared contracts and descriptors for the [Hyle](https://github.com/walkingtheplanck/hyle) cellular automaton framework.

This crate defines the shared **contracts and descriptors**. Depend on it to implement custom cell types or build a new solver backend. It has **zero dependencies**.

## Key Types

| Type | Role |
|------|------|
| [`Cell`] | Trait for custom cell state (rule dispatch + alive/dead query) |
| [`CaSolver`] | Trait that all solver backends implement (get/set/step) |
| [`GridDims`] / [`GridRegion`] / [`GridSnapshot`] | Backend-neutral grid descriptors and bulk transfer types |
| [`NeighborhoodSpec`] | Declarative neighborhood description shared across backends |
| [`Topology`] | Trait for boundary behavior policies used by solvers |
| [`TopologyDescriptor`] | Uploadable / serializable topology descriptor |
| [`ValidatedSolver`] | Debug wrapper that asserts solver contracts on every call |

## Defining a Custom Cell

```rust
use hyle_ca_contracts::Cell;

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
use hyle_ca_contracts::{GridDims, GridRegion, GridSnapshot};

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
use hyle_ca_contracts::{NeighborhoodShape, NeighborhoodSpec, NeighborhoodWeight};

let spec = NeighborhoodSpec::new(
    2,
    NeighborhoodShape::Moore,
    NeighborhoodWeight::Unweighted,
);

assert_eq!(spec.radius, 2);
```

## Topology

Solvers choose how coordinates beyond the grid bounds behave by implementing [`Topology`].
Built-in topology policies such as bounded and torus live in
[`hyle-ca-solver`](https://crates.io/crates/hyle-ca-solver), not in this crate.

```rust
use hyle_ca_contracts::{AxisTopology, GridDims, Topology, TopologyDescriptor};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
struct WrapAllTopology;

impl Topology for WrapAllTopology {
    fn descriptor(&self) -> TopologyDescriptor {
        TopologyDescriptor::uniform(AxisTopology::Wrap)
    }

    fn resolve_index(&self, x: i32, y: i32, z: i32, dims: GridDims, guard_idx: usize) -> usize {
        if dims.width == 0 || dims.height == 0 || dims.depth == 0 {
            return guard_idx;
        }

        let x = i64::from(x).rem_euclid(i64::from(dims.width)) as u32;
        let y = i64::from(y).rem_euclid(i64::from(dims.height)) as u32;
        let z = i64::from(z).rem_euclid(i64::from(dims.depth)) as u32;

        (x as usize)
            + (y as usize) * (dims.width as usize)
            + (z as usize) * (dims.width as usize) * (dims.height as usize)
    }
}

assert_eq!(
    WrapAllTopology.descriptor(),
    TopologyDescriptor::uniform(AxisTopology::Wrap),
);
```

## Implementing a Solver

Implement the [`CaSolver`] trait to create a custom backend (GPU, distributed, etc.):

```rust
use std::marker::PhantomData;

use hyle_ca_contracts::{AxisTopology, CaSolver, Cell, GridDims, Topology, TopologyDescriptor};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
struct ExampleTopology;

struct MySolver<C: Cell> {
    topology: ExampleTopology,
    _marker: PhantomData<C>,
}

impl Topology for ExampleTopology {
    fn descriptor(&self) -> TopologyDescriptor {
        TopologyDescriptor::uniform(AxisTopology::Bounded)
    }

    fn resolve_index(&self, x: i32, y: i32, z: i32, dims: GridDims, guard_idx: usize) -> usize {
        let ux = x as u32;
        let uy = y as u32;
        let uz = z as u32;
        let max_dim = i32::MAX as u32;
        let in_bounds = (dims.width <= max_dim)
            && (dims.height <= max_dim)
            && (dims.depth <= max_dim)
            && (ux < dims.width)
            && (uy < dims.height)
            && (uz < dims.depth);

        if in_bounds {
            (ux as usize)
                + (uy as usize) * (dims.width as usize)
                + (uz as usize) * (dims.width as usize) * (dims.height as usize)
        } else {
            guard_idx
        }
    }
}

impl<C: Cell> CaSolver<C> for MySolver<C> {
    type Topology = ExampleTopology;

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
