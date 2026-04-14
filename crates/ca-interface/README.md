# hyle-ca-interface

Shared interfaces, contracts, and descriptors for the [Hyle](https://github.com/walkingtheplanck/hyle) cellular automaton framework.

This crate defines the shared public interface layer. Depend on it to:
- define custom cell types
- author portable blueprint specs with `Hyle::builder()`
- implement new solver implementations against the shared `CaSolver` trait
- decouple apps and tools from concrete solver types through `CaRuntime` and `CaSolverProvider`

Derived analysis and diagnostics live in
[`hyle-ca-analysis`](https://crates.io/crates/hyle-ca-analysis). Canonical
interpretation helpers live in this crate under `hyle_ca_interface::semantics`.

It has **zero dependencies** and is split conceptually into:
- `contracts` for declarative blueprint and descriptor data
- `semantics` for interpreted blueprint, neighborhood, and topology meaning
- runtime interfaces and shared primitives such as `Cell`, `Instance`, `Rng`, `Topology`, `CaSolver`, and `ValidatedSolver`

## Key Types

| Type | Role |
|------|------|
| [`Cell`] | Trait for custom cell state |
| [`Instance`] | Runtime dimensions and deterministic seed for one solver run |
| [`Hyle`] / [`BlueprintSpec`] | Declarative blueprint builder and canonical spec |
| [`CaSolver`] | Trait that all solver implementations implement |
| [`CaRuntime`] / [`CaSolverProvider`] | Erased runtime and factory interfaces for consumers that should not depend on concrete solvers |
| [`GridDims`] / [`GridRegion`] / [`GridSnapshot`] | Solver-neutral grid descriptors and bulk transfer types |
| [`NeighborhoodSpec`] | Declarative neighborhood description shared across solvers |
| [`Rng`] | Shared deterministic random-number primitive parameterized by seed, position, step, and stream |
| [`Topology`] / [`TopologyDescriptor`] | Boundary behavior traits and descriptors |
| [`ValidatedSolver`] | Debug wrapper that asserts solver contracts on every call |

Semantic forms are available under `hyle_ca_interface::semantics`, for example:
- `hyle_ca_interface::semantics::Blueprint`
- `hyle_ca_interface::semantics::Neighborhood`
- `hyle_ca_interface::semantics::Topology`

## Building a Portable Blueprint

```rust
use hyle_ca_interface::{neighbors, Hyle, TopologyDescriptor};

let spec = Hyle::builder()
    .cells::<u32>()
    .topology(TopologyDescriptor::bounded())
    .rules(|rules| {
        rules.when(0).require(neighbors(1).count().eq(3)).becomes(1);
        rules.when(1).unless(neighbors(1).count().in_range(2..=3)).becomes(0);
    })
    .build()?;
# Ok::<(), hyle_ca_interface::BuildError>(())
```

Rules are evaluated in declaration order with **first-match wins** semantics.
If no rule matches, the center cell is kept unchanged.

## Decoupled Runtime Construction

Consumers such as viewers can depend on the centralized runtime/provider seam
instead of naming a concrete solver type:

```ignore
use hyle_ca_interface::{CaRuntime, CaSolverProvider, Hyle, Instance};
use hyle_ca_solver::CpuSolverProvider;

let spec = Hyle::builder().cells::<u32>().build()?;
let provider = CpuSolverProvider::new();
let runtime: Box<dyn CaRuntime<u32>> =
    provider.build(Instance::new(16, 16, 16), &spec);

# Ok::<(), hyle_ca_interface::BuildError>(())
```

This keeps backend selection localized to one construction site.

## Defining a Custom Cell

```rust
use hyle_ca_interface::{Cell, CellModel, CellSchema};

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

impl CellModel for FluidCell {
    fn schema() -> CellSchema {
        CellSchema::opaque("FluidCell")
    }
}
```

Blueprint builders require `CellModel` so the spec carries portable schema
metadata. The default solver requires `Eq` so it can match exact cell states
from a `BlueprintSpec`.

## Grid Descriptors

```rust
use hyle_ca_interface::{GridDims, GridRegion, GridSnapshot};

let dims = GridDims::new(8, 8, 8);
let region = GridRegion::new([2, 2, 2], [2, 2, 2]);
let snapshot = GridSnapshot::new(dims, vec![0u32; dims.cell_count()]);

assert!(dims.contains_region(region));
assert_eq!(snapshot.cells.len(), dims.cell_count());
```

## Deterministic RNG

```rust
use hyle_ca_interface::{Instance, Rng};

let instance = Instance::new(64, 64, 64).with_seed(42);
let rng = Rng::with_seed(10, 20, 30, 4, instance.seed());

assert!(rng.chance(1));
assert!(rng.range(8) < 8);
```

This RNG is deterministic and portable: the same `(seed, x, y, z, step, stream)`
input always produces the same output across all solvers.

## Declarative Neighborhoods

```rust
use hyle_ca_interface::{NeighborhoodFalloff, NeighborhoodShape, NeighborhoodSpec};

let adjacent = NeighborhoodSpec::adjacent();
let far = NeighborhoodSpec::new(
    NeighborhoodShape::Moore,
    2,
    NeighborhoodFalloff::Uniform,
);

assert_eq!(adjacent.radius(), 1);
assert_eq!(far.radius(), 2);
assert_eq!(far.shape(), NeighborhoodShape::Moore);
assert_eq!(far.falloff(), NeighborhoodFalloff::Uniform);
```

Neighborhood falloff expands to deterministic fixed-point weights in the semantic layer,
so CPU and GPU backends can agree on the same values exactly.

## Topology

Solvers choose how coordinates beyond the grid bounds behave by implementing [`Topology`].
Built-in CPU topology implementations live in
[`hyle-ca-solver`](https://crates.io/crates/hyle-ca-solver), but the portable
descriptor type lives here:

```rust
use hyle_ca_interface::{AxisTopology, TopologyDescriptor};

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

Implement the [`CaSolver`] trait to create a custom solver (GPU, distributed, etc.).
Solvers are expected to consume a portable representation such as [`BlueprintSpec`]
and uphold the runtime contract documented on the trait.
