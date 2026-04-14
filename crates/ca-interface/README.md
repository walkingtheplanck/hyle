# hyle-ca-interface

Shared interfaces, contracts, and descriptors for the [Hyle](https://github.com/walkingtheplanck/hyle) cellular automaton framework.

This crate defines the shared public interface layer. Depend on it to:
- define custom cell types
- author portable blueprints with `Blueprint::<C>::builder()`
- implement new solver implementations against the shared `CaSolver` trait
- centralize backend construction through `CaRuntime` and `CaSolverProvider`

Derived analysis and diagnostics live in
[`hyle-ca-analysis`](https://crates.io/crates/hyle-ca-analysis). Canonical
interpretation helpers live in this crate under `hyle_ca_interface::semantics`.

It has **zero dependencies** and is split conceptually into:
- `contracts` for declarative blueprint and descriptor data
- `semantics` for interpreted blueprint, neighborhood, and topology meaning
- `runtime` for running-simulation interfaces and shared runtime types such as `Cell`, `Instance`, `Topology`, `CaSolver`, `CaRuntime`, `CaSolverProvider`, and `ValidatedSolver`

## Key Types

| Type | Role |
|------|------|
| [`Cell`] | Marker trait for runtime cell state |
| [`Instance`] | Runtime dimensions and deterministic seed for one solver run |
| [`Blueprint`] | Declarative blueprint builder and canonical contract |
| [`CaSolver`] | Trait that all solver implementations implement |
| [`CaRuntime`] / [`CaSolverProvider`] | Concrete runtime and factory interfaces that keep backend selection localized |
| [`GridDims`] / [`GridRegion`] / [`GridSnapshot`] | Solver-neutral grid descriptors and bulk transfer types |
| [`NeighborhoodSpec`] | Declarative neighborhood description shared across solvers |
| [`Weight`] | Fixed-point weight threshold used by weighted neighborhood predicates |
| [`Rng`] | Shared deterministic random-number primitive parameterized by seed, position, step, and stream |
| [`Topology`] / [`TopologyDescriptor`] | Boundary behavior traits and descriptors |
| [`ValidatedSolver`] | Debug wrapper that asserts solver contracts on every call |

Semantic forms are available under `hyle_ca_interface::semantics`, for example:
- `hyle_ca_interface::semantics::ResolvedBlueprint`
- `hyle_ca_interface::semantics::Neighborhood`
- `hyle_ca_interface::semantics::ResolvedTopology`

## Preferred Imports

Use the crate root or the prelude as the main entry points:

- Prefer explicit root imports for application and library code:
  `use hyle_ca_interface::{Blueprint, CaSolverProvider, Instance};`
- Use `hyle_ca_interface::prelude::*` when you want a compact common import set
  for blueprint authoring and runtime setup.
- Treat `hyle_ca_interface::semantics` as an advanced namespace for interpreted
  forms and semantic helpers.

The internal `contracts` and `runtime` module layout is crate organization, not
the intended consumer-facing path.

## Building a Portable Blueprint

```rust
use hyle_ca_interface::{neighbors, Blueprint, CellModel, CellSchema, StateDef, TopologyDescriptor};

#[derive(Copy, Clone, Default, PartialEq, Eq)]
enum LifeCell {
    #[default]
    Dead,
    Alive,
}

const LIFE_CELL_STATES: [StateDef; 2] = [StateDef::new("Dead"), StateDef::new("Alive")];

impl CellModel for LifeCell {
    fn schema() -> CellSchema {
        CellSchema::enumeration("LifeCell", &LIFE_CELL_STATES)
    }
}

let spec = Blueprint::<LifeCell>::builder()
    .topology(TopologyDescriptor::bounded())
    .attribute("heat", hyle_ca_interface::AttributeType::U8)
    .rules(|rules| {
        rules.when(LifeCell::Dead)
            .require(neighbors(LifeCell::Alive).count().eq(3))
            .becomes(LifeCell::Alive);
        rules.when(LifeCell::Alive)
            .unless(neighbors(LifeCell::Alive).count().in_range(2..=3))
            .becomes(LifeCell::Dead);
    })
    .build()?;
# Ok::<(), hyle_ca_interface::BuildError>(())
```

Rules are evaluated in declaration order with **first-match wins** semantics.
If no rule matches, the center cell is kept unchanged.

## Attached Attributes

Blueprints can declare named attached per-cell attributes and use them in rules:

```rust
use hyle_ca_interface::{attr, AttributeType, AttributeValue, Blueprint, CellModel, CellSchema};

#[derive(Copy, Clone, Default, PartialEq, Eq)]
struct TestCell(u8);

impl CellModel for TestCell {
    fn schema() -> CellSchema { CellSchema::opaque("TestCell") }
}

let spec = Blueprint::<TestCell>::builder()
    .attribute("heat", AttributeType::U8)
    .attribute_with_default("charged", AttributeValue::Bool(true))
    .rules(|rules| {
        rules
            .when(TestCell(1))
            .require(attr("heat").at_least(2u8))
            .set_attr("heat", 0u8)
            .keep();
    })
    .build()?;

assert_eq!(spec.attributes().len(), 2);
# Ok::<(), hyle_ca_interface::BuildError>(())
```

These attributes are part of the portable blueprint contract. The current DSL
supports center-cell attribute predicates and rule-driven attribute writes.

## Decoupled Runtime Construction

Consumers such as viewers can depend on the centralized runtime/provider seam
instead of naming a concrete solver type directly:

```ignore
use hyle_ca_interface::{Blueprint, CaRuntime, CaSolverProvider, CellModel, CellSchema, Instance};
use hyle_ca_solver::CpuSolverProvider;

#[derive(Copy, Clone, Default, PartialEq, Eq)]
struct TestCell(u32);

impl CellModel for TestCell {
    fn schema() -> CellSchema { CellSchema::opaque("TestCell") }
}

let spec = Blueprint::<TestCell>::builder().build()?;
let provider = CpuSolverProvider::new();
let runtime = provider.build(Instance::new(16, 16, 16), &spec);

# Ok::<(), hyle_ca_interface::BuildError>(())
```

This keeps backend selection localized to one construction site while preserving static dispatch.

## Defining a Custom Cell

```rust
use hyle_ca_interface::{CellModel, CellSchema};

#[derive(Copy, Clone, Default, PartialEq, Eq)]
struct FluidCell {
    density: u8,
    velocity: [i8; 6],
    material: u8,
}

impl CellModel for FluidCell {
    fn schema() -> CellSchema {
        CellSchema::opaque("FluidCell")
    }
}
```

Any `CellState` automatically implements the runtime `Cell` marker trait.
Blueprint builders require `CellModel` so the spec carries portable schema
metadata. The default solver requires `Eq` so it can match exact cell states
from a `Blueprint`.

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

Weighted predicates use the same portable units:

```rust
use hyle_ca_interface::{neighbors, Weight};

let condition = neighbors(1u8).weighted_sum().at_least(Weight::cells(2));
```

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
Solvers are expected to consume a portable representation such as [`Blueprint`]
and uphold the runtime contract documented on the trait.
