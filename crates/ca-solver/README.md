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
# fn main() -> Result<(), Box<dyn std::error::Error>> {
use hyle_ca_interface::{
    neighbors, Blueprint, Instance, MaterialSet, NeighborhoodFalloff, NeighborhoodRadius,
    NeighborhoodSet, NeighborhoodShape, NeighborhoodSpec, RuleSpec, SolverExecution,
};
use hyle_ca_solver::Solver;

#[derive(Copy, Clone, Default, PartialEq, Eq, MaterialSet)]
enum Material {
    #[default]
    Dead,
    Alive,
}

#[derive(Copy, Clone, PartialEq, Eq, NeighborhoodSet)]
enum Neighborhood {
    #[label("adjacent")]
    Adjacent,
}

let spec = Blueprint::builder()
    .materials::<Material>()
    .neighborhoods::<Neighborhood>()
    .neighborhood_specs([NeighborhoodSpec::new(
        Neighborhood::Adjacent,
        NeighborhoodShape::Moore,
        NeighborhoodRadius::new(1),
        NeighborhoodFalloff::Uniform,
    )?])
    .rules([
        RuleSpec::when(Material::Dead)
            .require(neighbors(Material::Alive).count().eq(3))
            .becomes(Material::Alive),
        RuleSpec::when(Material::Alive)
            .require(neighbors(Material::Alive).count().in_range(2..=3).negate())
            .becomes(Material::Dead),
    ])
    .build()?;

let instance = Instance::new(64, 64, 64).map_err(|err| format!("{err:?}"))?;
let mut solver = Solver::from_spec_instance(instance.with_seed(7), &spec);
solver.step();
# let _ = solver;
# Ok(())
# }
```

## Decoupled Consumer Path

```rust
# fn main() -> Result<(), Box<dyn std::error::Error>> {
use hyle_ca_interface::{
    Blueprint, CaSolverProvider, Instance, MaterialSet, NeighborhoodFalloff, NeighborhoodRadius,
    NeighborhoodSet, NeighborhoodShape, NeighborhoodSpec, RuntimeStepping,
};
use hyle_ca_solver::CpuSolverProvider;

#[derive(Copy, Clone, Default, PartialEq, Eq, MaterialSet)]
enum Material {
    #[default]
    #[label("empty")]
    Empty,
}

#[derive(Copy, Clone, PartialEq, Eq, NeighborhoodSet)]
enum Neighborhood {
    #[label("adjacent")]
    Adjacent,
}

let spec = Blueprint::builder()
    .materials::<Material>()
    .neighborhoods::<Neighborhood>()
    .neighborhood_specs([NeighborhoodSpec::new(
        Neighborhood::Adjacent,
        NeighborhoodShape::Moore,
        NeighborhoodRadius::new(1),
        NeighborhoodFalloff::Uniform,
    )?])
    .build()?;
let provider = CpuSolverProvider::new();
let instance = Instance::new(16, 16, 16).map_err(|err| format!("{err:?}"))?;
let mut runtime = provider.build(instance, &spec);

runtime.step();
# let _ = runtime;
# Ok(())
# }
```

## Topology

The solver still supports direct construction with built-in topology types:

```rust
# fn main() -> Result<(), Box<dyn std::error::Error>> {
use hyle_ca_solver::{Solver, TorusTopology};

let _solver = Solver::with_topology(64, 64, 64, TorusTopology)
    .map_err(|err| format!("{err:?}"))?;
# Ok(())
# }
```

When you create a solver from a `Blueprint`, the solver interprets that
blueprint into a resolved blueprint and uses the descriptor declared by it.

Custom solver implementations that need canonical neighborhood expansion can use
`hyle_ca_interface::resolved` directly without reimplementing schema interpretation.

## Declaring Custom Neighborhoods

Use named neighborhoods in the spec, then reference them from rules:

```rust
use hyle_ca_interface::{
    neighbors, Blueprint, MaterialSet, NeighborhoodFalloff, NeighborhoodRadius, NeighborhoodSet,
    NeighborhoodShape, NeighborhoodSpec, RuleSpec, SolverExecution,
};
use hyle_ca_solver::Solver;

#[derive(Copy, Clone, Default, PartialEq, Eq, MaterialSet)]
enum Material {
    #[default]
    #[label("dead")]
    Dead,
    #[label("alive")]
    Alive,
}

#[derive(Copy, Clone, PartialEq, Eq, NeighborhoodSet)]
enum Neighborhood {
    #[label("adjacent")]
    Adjacent,
    #[label("far")]
    Far,
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
        )?,
        NeighborhoodSpec::new(
            Neighborhood::Far,
            NeighborhoodShape::Moore,
            NeighborhoodRadius::new(2),
            NeighborhoodFalloff::Uniform,
        )?,
    ])
    .default_neighborhood(Neighborhood::Adjacent)
    .rules([RuleSpec::when(Material::Dead)
        .using(Neighborhood::Far)
        .require(neighbors(Material::Alive).count().at_least(1))
        .becomes(Material::Alive)])
    .build()?;

let mut solver = Solver::from_spec(8, 8, 8, &spec).map_err(|err| format!("{err:?}"))?;
solver.step();
# Ok::<(), Box<dyn std::error::Error>>(())
```

## How It Works

Each call to `step()`:

1. Copies the current buffer to the next buffer.
2. Evaluates the ordered blueprint rules against the current buffer.
3. Applies the first matching rule per cell to the next buffer.
4. Swaps buffers and increments the step counter.

The double-buffer design keeps rule evaluation order-independent at the cell
level, which makes the semantics portable to future GPU solvers.
