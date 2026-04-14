# hyle-ca-solver

Default CPU solver for the [Hyle](https://github.com/walkingtheplanck/hyle) cellular automaton framework.

Double-buffered, single-threaded, and driven by portable
[`BlueprintSpec`](https://docs.rs/hyle-ca-interface/latest/hyle_ca_interface/struct.BlueprintSpec.html)
definitions from [`hyle-ca-interface`](https://crates.io/crates/hyle-ca-interface).

For apps and tools that should not depend on the concrete
[`Solver`](https://docs.rs/hyle-ca-solver/latest/hyle_ca_solver/struct.Solver.html)
type directly, this crate also exposes
[`CpuSolverProvider`](https://docs.rs/hyle-ca-solver/latest/hyle_ca_solver/struct.CpuSolverProvider.html),
which builds erased
[`CaRuntime`](https://docs.rs/hyle-ca-interface/latest/hyle_ca_interface/trait.CaRuntime.html)
instances through the shared `CaSolverProvider` interface.

## Quick Start

```rust
use hyle_ca_interface::{neighbors, CaSolver, Cell, CellModel, CellSchema, Hyle, Instance, StateDef};
use hyle_ca_solver::Solver;

#[derive(Copy, Clone, Default, PartialEq, Eq)]
enum LifeCell {
    #[default]
    Dead,
    Alive,
}

const LIFE_CELL_STATES: [StateDef; 2] = [StateDef::new("Dead"), StateDef::new("Alive")];

impl Cell for LifeCell {
    fn rule_id(&self) -> u8 {
        match self {
            Self::Dead => 0,
            Self::Alive => 1,
        }
    }

    fn is_alive(&self) -> bool {
        matches!(self, Self::Alive)
    }
}

impl CellModel for LifeCell {
    fn schema() -> CellSchema {
        CellSchema::enumeration("LifeCell", &LIFE_CELL_STATES)
    }
}

let spec = Hyle::builder()
    .cells::<LifeCell>()
    .rules(|rules| {
        rules.when(LifeCell::Dead)
            .require(neighbors(LifeCell::Alive).count().eq(3))
            .becomes(LifeCell::Alive);
        rules.when(LifeCell::Alive)
            .unless(neighbors(LifeCell::Alive).count().in_range(2..=3))
            .becomes(LifeCell::Dead);
    })
    .build()?;

let mut solver = Solver::from_spec_instance(Instance::new(64, 64, 64).with_seed(7), &spec);
solver.step();
# Ok::<(), hyle_ca_interface::BuildError>(())
```

## Decoupled Consumer Path

```rust
use hyle_ca_interface::{CaRuntime, CaSolverProvider, Cell, CellModel, CellSchema, Hyle, Instance};
use hyle_ca_solver::CpuSolverProvider;

#[derive(Copy, Clone, Default, PartialEq, Eq)]
struct TestCell(u32);

impl Cell for TestCell {
    fn rule_id(&self) -> u8 { self.0 as u8 }
    fn is_alive(&self) -> bool { self.0 != 0 }
}

impl CellModel for TestCell {
    fn schema() -> CellSchema { CellSchema::opaque("TestCell") }
}

let spec = Hyle::builder().cells::<TestCell>().build()?;
let provider = CpuSolverProvider::new();
let mut runtime: Box<dyn CaRuntime<TestCell>> =
    provider.build(Instance::new(16, 16, 16), &spec);

runtime.step();
# Ok::<(), hyle_ca_interface::BuildError>(())
```

## Topology

The solver still supports direct construction with built-in topology types:

```rust
use hyle_ca_interface::Cell;
use hyle_ca_solver::{Solver, TorusTopology};

#[derive(Copy, Clone, Default, PartialEq, Eq)]
struct TestCell(u32);

impl Cell for TestCell {
    fn rule_id(&self) -> u8 { self.0 as u8 }
    fn is_alive(&self) -> bool { self.0 != 0 }
}

let _solver = Solver::<TestCell>::with_topology(64, 64, 64, TorusTopology);
```

When you create a solver from a `BlueprintSpec`, the solver interprets that
spec into a semantic blueprint and uses the descriptor declared by it.

Custom solver implementations that need canonical neighborhood expansion can use
`hyle_ca_interface::semantics` directly without reimplementing contract interpretation.

## Declaring Custom Neighborhoods

Use named neighborhoods in the spec, then reference them from rules:

```rust
use hyle_ca_interface::{
    neighbors, CaSolver, Cell, CellModel, CellSchema, Hyle, NeighborhoodFalloff, NeighborhoodShape, NeighborhoodSpec, StateDef,
};
use hyle_ca_solver::Solver;

#[derive(Copy, Clone, Default, PartialEq, Eq)]
enum LifeCell {
    #[default]
    Dead,
    Alive,
}

const LIFE_CELL_STATES: [StateDef; 2] = [StateDef::new("Dead"), StateDef::new("Alive")];

impl Cell for LifeCell {
    fn rule_id(&self) -> u8 {
        match self {
            Self::Dead => 0,
            Self::Alive => 1,
        }
    }

    fn is_alive(&self) -> bool {
        matches!(self, Self::Alive)
    }
}

impl CellModel for LifeCell {
    fn schema() -> CellSchema {
        CellSchema::enumeration("LifeCell", &LIFE_CELL_STATES)
    }
}

let spec = Hyle::builder()
    .cells::<LifeCell>()
    .neighborhood(
        "far",
        NeighborhoodSpec::new(
            NeighborhoodShape::Moore,
            2,
            NeighborhoodFalloff::Uniform,
        ),
    )
    .rules(|rules| {
        rules.when(LifeCell::Dead)
            .using("far")
            .require(neighbors(LifeCell::Alive).count().at_least(1))
            .becomes(LifeCell::Alive);
    })
    .build()?;

let mut solver = Solver::from_spec(8, 8, 8, &spec);
solver.step();
# Ok::<(), hyle_ca_interface::BuildError>(())
```

## How It Works

Each call to `step()`:

1. Copies the current buffer to the next buffer.
2. Evaluates the ordered blueprint rules against the current buffer.
3. Applies the first matching rule per cell to the next buffer.
4. Swaps buffers and increments the step counter.

The double-buffer design keeps rule evaluation order-independent at the cell
level, which makes the semantics portable to future GPU solvers.
