# hyle-ca-contracts

Shared contracts, descriptors, and declarative automaton specs for the [Hyle](https://github.com/walkingtheplanck/hyle) cellular automaton framework.

This crate defines the backend-neutral contract layer. Depend on it to:
- define custom cell types
- author portable automaton specs with `Hyle::builder()`
- implement new solver backends against the shared `CaSolver` trait

It has **zero dependencies**.

## Key Types

| Type | Role |
|------|------|
| [`Cell`] | Trait for custom cell state |
| [`Hyle`] / [`AutomatonSpec`] | Declarative automaton builder and canonical spec |
| [`CaSolver`] | Trait that all solver backends implement |
| [`GridDims`] / [`GridRegion`] / [`GridSnapshot`] | Backend-neutral grid descriptors and bulk transfer types |
| [`NeighborhoodSpec`] | Declarative neighborhood description shared across backends |
| [`Topology`] / [`TopologyDescriptor`] | Boundary behavior traits and descriptors |
| [`ValidatedSolver`] | Debug wrapper that asserts solver contracts on every call |

## Building a Portable Automaton

```rust
use hyle_ca_contracts::{neighbors, Hyle, TopologyDescriptor};

let spec = Hyle::builder()
    .cells::<u32>()
    .topology(TopologyDescriptor::bounded())
    .rules(|rules| {
        rules.when(0).require(neighbors(1).count().eq(3)).becomes(1);
        rules.when(1).unless(neighbors(1).count().in_range(2..=3)).becomes(0);
    })
    .build()?;
# Ok::<(), hyle_ca_contracts::BuildError>(())
```

Rules are evaluated in declaration order with **first-match wins** semantics.
If no rule matches, the center cell is kept unchanged.

## Defining a Custom Cell

```rust
use hyle_ca_contracts::Cell;

#[derive(Copy, Clone, Default, PartialEq, Eq)]
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

The default solver requires `Eq` so it can match exact cell states from an
`AutomatonSpec`.

## Grid Descriptors

```rust
use hyle_ca_contracts::{GridDims, GridRegion, GridSnapshot};

let dims = GridDims::new(8, 8, 8);
let region = GridRegion::new([2, 2, 2], [2, 2, 2]);
let snapshot = GridSnapshot::new(dims, vec![0u32; dims.cell_count()]);

assert!(dims.contains_region(region));
assert_eq!(snapshot.cells.len(), dims.cell_count());
```

## Declarative Neighborhoods

```rust
use hyle_ca_contracts::NeighborhoodSpec;

let adjacent = NeighborhoodSpec::adjacent();
let far = NeighborhoodSpec::cube(2);

assert_eq!(adjacent.radius, 1);
assert_eq!(far.radius, 2);
```

## Topology

Solvers choose how coordinates beyond the grid bounds behave by implementing [`Topology`].
Built-in CPU topology implementations live in
[`hyle-ca-solver`](https://crates.io/crates/hyle-ca-solver), but the portable
descriptor type lives here:

```rust
use hyle_ca_contracts::{AxisTopology, TopologyDescriptor};

let bounded = TopologyDescriptor::bounded();
let mixed = TopologyDescriptor::by_axis(
    AxisTopology::Wrap,
    AxisTopology::Bounded,
    AxisTopology::Wrap,
);

assert_eq!(bounded.x, AxisTopology::Bounded);
assert_eq!(mixed.y, AxisTopology::Bounded);
```

## Implementing a Solver

Implement the [`CaSolver`] trait to create a custom backend (GPU, distributed, etc.).
Backends are expected to consume a portable representation such as [`AutomatonSpec`]
and uphold the runtime contract documented on the trait.
