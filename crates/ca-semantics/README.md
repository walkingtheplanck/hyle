# hyle-ca-semantics

Canonical semantic helpers for the [Hyle](https://github.com/walkingtheplanck/hyle) cellular automaton framework.

This crate builds on [`hyle-ca-contracts`](https://crates.io/crates/hyle-ca-contracts) and interprets declarative blueprint specs into canonical semantic data:
- interpreted blueprints
- exact neighbor counts
- canonical neighborhood offsets
- reusable interpreted neighborhood values

It is optional for normal framework users. Solver implementations, analysis tooling, and advanced integrations can depend on it to share one blueprint interpretation layer.

## Quick Start

```rust
use hyle_ca_contracts::{BlueprintSpec, Hyle, NeighborhoodFalloff, NeighborhoodShape, NeighborhoodSpec};
use hyle_ca_semantics::{expand_neighborhood, interpret_blueprint, neighbor_count};

let spec = NeighborhoodSpec::new(
    NeighborhoodShape::Moore,
    2,
    NeighborhoodFalloff::Uniform,
);

let expanded = expand_neighborhood(spec);
let blueprint_spec: BlueprintSpec<u32> = Hyle::builder().cells::<u32>().build().unwrap();
let blueprint = interpret_blueprint(&blueprint_spec);

assert_eq!(neighbor_count(spec), 124);
assert_eq!(expanded.offsets().len(), 124);
assert_eq!(blueprint.neighborhoods().len(), 1);
```

## What It Exposes

- `neighbor_count(spec)` for exact neighborhood counts
- `offsets(spec)` for canonical relative offsets
- `expand_neighborhood(spec)` for a reusable interpreted neighborhood
- `interpret_blueprint(spec)` for a reusable interpreted blueprint

## Relationship To Other Crates

| Crate | Role |
|------|------|
| [`hyle-ca-contracts`](https://crates.io/crates/hyle-ca-contracts) | Declarative blueprint specs and solver traits |
| [`hyle-ca-semantics`](https://crates.io/crates/hyle-ca-semantics) | Canonical interpretation of declarative specs |
| [`hyle-ca-analysis`](https://crates.io/crates/hyle-ca-analysis) | Shared diagnostics over declarative specs |
| [`hyle-ca-solver`](https://crates.io/crates/hyle-ca-solver) | Default CPU solver implementation |
