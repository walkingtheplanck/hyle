# hyle-ca-analysis

Shared analysis and diagnostics for the [Hyle](https://github.com/walkingtheplanck/hyle) cellular automaton framework.

This crate builds on [`hyle-ca-interface`](https://crates.io/crates/hyle-ca-interface) and
[`hyle-ca-semantics`](https://crates.io/crates/hyle-ca-semantics) and provides
**derived tooling** over declarative blueprint specs:
- static spec summaries
- rule and neighborhood diagnostics

It intentionally does **not** execute simulations. Solvers consume the same
contracts directly; this crate helps inspect them consistently.

## Quick Start

```rust
use hyle_ca_analysis::analyze_spec;
use hyle_ca_interface::{neighbors, Hyle};

let spec = Hyle::builder()
    .cells::<u32>()
    .rules(|rules| {
        rules.when(0).require(neighbors(1).count().eq(3)).becomes(1);
    })
    .build()?;

let analysis = analyze_spec(&spec);
assert_eq!(analysis.summary.rule_count, 1);
# Ok::<(), hyle_ca_interface::BuildError>(())
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
| [`hyle-ca-interface`](https://crates.io/crates/hyle-ca-interface) | Canonical blueprint specs and solver traits |
| [`hyle-ca-semantics`](https://crates.io/crates/hyle-ca-semantics) | Canonical neighborhood interpretation helpers |
| [`hyle-ca-analysis`](https://crates.io/crates/hyle-ca-analysis) | Shared spec analysis and diagnostics |
| [`hyle-ca-solver`](https://crates.io/crates/hyle-ca-solver) | Default CPU solver implementation |
