# hyle-ca-solver

Default CPU solver for the [Hyle](https://github.com/walkingtheplanck/hyle) cellular automaton framework.

Double-buffered, single-threaded, and driven by portable
[`Blueprint`](https://docs.rs/hyle-ca-interface/latest/hyle_ca_interface/struct.Blueprint.html)
definitions from [`hyle-ca-interface`](https://crates.io/crates/hyle-ca-interface).

For apps and tools that should not depend on the concrete
[`Solver`](https://docs.rs/hyle-ca-solver/latest/hyle_ca_solver/struct.Solver.html)
type directly, this crate also exposes
[`CpuSolverProvider`](https://docs.rs/hyle-ca-solver/latest/hyle_ca_solver/struct.CpuSolverProvider.html),
which builds concrete
[`CaRuntime`](https://docs.rs/hyle-ca-interface/latest/hyle_ca_interface/trait.CaRuntime.html)
instances through the shared `CaSolverProvider` interface.

## Quick Start

```rust
use hyle_ca_interface::{
    neighbors, Blueprint, CaSolver, Instance, MaterialSet, NeighborhoodFalloff,
    NeighborhoodRadius, NeighborhoodSet, NeighborhoodShape, NeighborhoodSpec, RuleSpec,
};
use hyle_ca_solver::Solver;

#[derive(Copy, Clone, Default, PartialEq, Eq, MaterialSet)]
enum Material {
    #[default]
    Dead,
    Alive,
}

#[derive(Copy, Clone, PartialEq, Eq)]
enum Neighborhood {
    Adjacent,
}

impl NeighborhoodSet for Neighborhood {
    fn variants() -> &'static [Self] {
        &[Neighborhood::Adjacent]
    }

    fn label(self) -> &'static str {
        "adjacent"
    }
}

let spec = Blueprint::builder()
    .materials::<Material>()
    .neighborhoods::<Neighborhood>()
    .neighborhood_specs([NeighborhoodSpec::new(
        Neighborhood::Adjacent,
        NeighborhoodShape::Moore,
        NeighborhoodRadius::new(1),
        NeighborhoodFalloff::Uniform,
    )])
    .rules([
        RuleSpec::when(Material::Dead)
            .require(neighbors(Material::Alive).count().eq(3))
            .becomes(Material::Alive),
        RuleSpec::when(Material::Alive)
            .require(neighbors(Material::Alive).count().in_range(2..=3).negate())
            .becomes(Material::Dead),
    ])
    .build()?;

let mut solver = Solver::from_spec_instance(Instance::new(64, 64, 64).with_seed(7), &spec);
solver.step();
# Ok::<(), hyle_ca_interface::BuildError>(())
```

## Decoupled Consumer Path

```rust
use hyle_ca_interface::{
    Blueprint, CaRuntime, CaSolverProvider, Instance, MaterialSet, NeighborhoodFalloff,
    NeighborhoodRadius, NeighborhoodSet, NeighborhoodShape, NeighborhoodSpec,
};
use hyle_ca_solver::CpuSolverProvider;

#[derive(Copy, Clone, Default, PartialEq, Eq)]
enum Material {
    #[default]
    Empty,
}

impl MaterialSet for Material {
    fn variants() -> &'static [Self] { &[Material::Empty] }
    fn label(self) -> &'static str { "empty" }
}

#[derive(Copy, Clone, PartialEq, Eq)]
enum Neighborhood {
    Adjacent,
}

impl NeighborhoodSet for Neighborhood {
    fn variants() -> &'static [Self] { &[Neighborhood::Adjacent] }
    fn label(self) -> &'static str { "adjacent" }
}

let spec = Blueprint::builder()
    .materials::<Material>()
    .neighborhoods::<Neighborhood>()
    .neighborhood_specs([NeighborhoodSpec::new(
        Neighborhood::Adjacent,
        NeighborhoodShape::Moore,
        NeighborhoodRadius::new(1),
        NeighborhoodFalloff::Uniform,
    )])
    .build()?;
let provider = CpuSolverProvider::new();
let mut runtime = provider.build(Instance::new(16, 16, 16), &spec);

runtime.step();
# Ok::<(), hyle_ca_interface::BuildError>(())
```

## Topology

The solver still supports direct construction with built-in topology types:

```rust
use hyle_ca_solver::{Solver, TorusTopology};

let _solver = Solver::with_topology(64, 64, 64, TorusTopology);
```

When you create a solver from a `Blueprint`, the solver interprets that
blueprint into a resolved blueprint and uses the descriptor declared by it.

Custom solver implementations that need canonical neighborhood expansion can use
`hyle_ca_interface::resolved` directly without reimplementing schema interpretation.

## Declaring Custom Neighborhoods

Use named neighborhoods in the spec, then reference them from rules:

```rust
use hyle_ca_interface::{
    neighbors, Blueprint, CaSolver, MaterialSet, NeighborhoodFalloff, NeighborhoodRadius,
    NeighborhoodSet, NeighborhoodShape, NeighborhoodSpec, RuleSpec,
};
use hyle_ca_solver::Solver;

#[derive(Copy, Clone, Default, PartialEq, Eq)]
enum Material {
    #[default]
    Dead,
    Alive,
}

impl MaterialSet for Material {
    fn variants() -> &'static [Self] {
        &[Material::Dead, Material::Alive]
    }

    fn label(self) -> &'static str {
        match self {
            Material::Dead => "dead",
            Material::Alive => "alive",
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq)]
enum Neighborhood {
    Adjacent,
    Far,
}

impl NeighborhoodSet for Neighborhood {
    fn variants() -> &'static [Self] {
        &[Neighborhood::Adjacent, Neighborhood::Far]
    }

    fn label(self) -> &'static str {
        match self {
            Neighborhood::Adjacent => "adjacent",
            Neighborhood::Far => "far",
        }
    }
}

let spec = Blueprint::builder()
    .materials::<Material>()
    .neighborhoods::<Neighborhood>()
    .neighborhood_specs([
        NeighborhoodSpec::new(
            Neighborhood::Adjacent,
            NeighborhoodShape::Moore,
            NeighborhoodRadius::new(1),
            NeighborhoodFalloff::Uniform,
        ),
        NeighborhoodSpec::new(
            Neighborhood::Far,
            NeighborhoodShape::Moore,
            NeighborhoodRadius::new(2),
            NeighborhoodFalloff::Uniform,
        ),
    ])
    .default_neighborhood(Neighborhood::Adjacent)
    .rules([RuleSpec::when(Material::Dead)
        .using(Neighborhood::Far)
        .require(neighbors(Material::Alive).count().at_least(1))
        .becomes(Material::Alive)])
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
