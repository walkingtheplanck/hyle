# hyle-ca-semantics

Canonical semantic helpers for the [Hyle](https://github.com/walkingtheplanck/hyle) cellular automaton framework.

This crate builds on [`hyle-ca-contracts`](https://crates.io/crates/hyle-ca-contracts) and expands declarative neighborhood specs into canonical semantic data:
- exact neighbor counts
- canonical neighborhood offsets
- reusable expanded neighborhood values

It is optional for normal framework users. Solver backends, analysis tooling, and advanced integrations can depend on it to share one neighborhood interpretation layer.

## Quick Start

```rust
use hyle_ca_contracts::{NeighborhoodFalloff, NeighborhoodShape, NeighborhoodSpec};
use hyle_ca_semantics::{expand_neighborhood, neighbor_count};

let spec = NeighborhoodSpec::new(
    NeighborhoodShape::Moore,
    2,
    NeighborhoodFalloff::Uniform,
);

let expanded = expand_neighborhood(spec);

assert_eq!(neighbor_count(spec), 124);
assert_eq!(expanded.offsets().len(), 124);
```

## What It Exposes

- `neighbor_count(spec)` for exact neighborhood counts
- `offsets(spec)` for canonical relative offsets
- `expand_neighborhood(spec)` for a reusable expanded form

## Relationship To Other Crates

| Crate | Role |
|------|------|
| [`hyle-ca-contracts`](https://crates.io/crates/hyle-ca-contracts) | Declarative specs and solver traits |
| [`hyle-ca-semantics`](https://crates.io/crates/hyle-ca-semantics) | Canonical semantic expansion of declarative specs |
| [`hyle-ca-analysis`](https://crates.io/crates/hyle-ca-analysis) | Shared diagnostics over declarative specs |
| [`hyle-ca-solver`](https://crates.io/crates/hyle-ca-solver) | Default CPU execution backend |
