# hyle-ca-analysis

Shared analysis and diagnostics for the [Hyle](https://github.com/walkingtheplanck/hyle) cellular automaton framework.

This crate builds on [`hyle-ca-contracts`](https://crates.io/crates/hyle-ca-contracts) and provides **derived tooling** over declarative automaton specs:
- static spec summaries
- rule and neighborhood diagnostics

It intentionally does **not** execute simulations. Solvers consume the same
contracts directly; this crate helps inspect them consistently.

## Quick Start

```rust
use hyle_ca_analysis::analyze_spec;
use hyle_ca_contracts::{neighbors, Hyle};

let spec = Hyle::builder()
    .cells::<u32>()
    .rules(|rules| {
        rules.when(0).require(neighbors(1).count().eq(3)).becomes(1);
    })
    .build()?;

let analysis = analyze_spec(&spec);
assert_eq!(analysis.summary.rule_count, 1);
# Ok::<(), hyle_ca_contracts::BuildError>(())
```

## What It Analyzes

### Spec Analysis

- summary data such as rule count, neighborhood count, and max radius
- unused named neighborhoods
- duplicate rules
- rules shadowed by earlier unconditional rules

## Relationship To Other Crates

| Crate | Role |
|------|------|
| [`hyle-ca-contracts`](https://crates.io/crates/hyle-ca-contracts) | Canonical specs and solver traits |
| [`hyle-ca-analysis`](https://crates.io/crates/hyle-ca-analysis) | Shared spec analysis and diagnostics |
| [`hyle-ca-solver`](https://crates.io/crates/hyle-ca-solver) | Default CPU execution backend |
