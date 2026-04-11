# hyle-ca-solver

Default CPU solver for the [Hyle](https://github.com/walkingtheplanck/hyle) cellular automaton framework.

Double-buffered, single-threaded, and driven by portable
[`AutomatonSpec`](https://docs.rs/hyle-ca-contracts/latest/hyle_ca_contracts/struct.AutomatonSpec.html)
definitions from [`hyle-ca-contracts`](https://crates.io/crates/hyle-ca-contracts).

## Quick Start

```rust
use hyle_ca_contracts::{neighbors, CaSolver, Hyle};
use hyle_ca_solver::Solver;

let spec = Hyle::builder()
    .cells::<u32>()
    .rules(|rules| {
        rules.when(0).require(neighbors(1).count().eq(3)).becomes(1);
        rules.when(1).unless(neighbors(1).count().in_range(2..=3)).becomes(0);
    })
    .build()?;

let mut solver = Solver::from_spec(64, 64, 64, &spec);
solver.step();
# Ok::<(), hyle_ca_contracts::BuildError>(())
```

## Topology

The solver still supports direct construction with built-in topology types:

```rust
use hyle_ca_solver::{Solver, TorusTopology};

let _solver = Solver::<u32>::with_topology(64, 64, 64, TorusTopology);
```

When you create a solver from an `AutomatonSpec`, the solver uses the
descriptor declared by that spec.

## Declaring Custom Neighborhoods

Use named neighborhoods in the spec, then reference them from rules:

```rust
use hyle_ca_contracts::{neighbors, CaSolver, Hyle, NeighborhoodSpec};
use hyle_ca_solver::Solver;

let spec = Hyle::builder()
    .cells::<u32>()
    .neighborhood("far", NeighborhoodSpec::cube(2))
    .rules(|rules| {
        rules.when(0)
            .using("far")
            .require(neighbors(1).count().at_least(1))
            .becomes(1);
    })
    .build()?;

let mut solver = Solver::from_spec(8, 8, 8, &spec);
solver.step();
# Ok::<(), hyle_ca_contracts::BuildError>(())
```

## How It Works

Each call to `step()`:

1. Copies the current buffer to the next buffer.
2. Evaluates the ordered automaton rules against the current buffer.
3. Applies the first matching rule per cell to the next buffer.
4. Swaps buffers and increments the step counter.

The double-buffer design keeps rule evaluation order-independent at the cell
level, which makes the semantics portable to future GPU backends.
