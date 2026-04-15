# hyle-ca-analysis

Shared analysis and diagnostics for the [Hyle](https://github.com/walkingtheplanck/hyle) cellular automaton framework.

This crate builds on [`hyle-ca-interface`](https://crates.io/crates/hyle-ca-interface) and provides
**derived tooling** over declarative blueprints:
- static spec summaries
- rule and neighborhood diagnostics
- runtime report analysis over completed solver steps

It intentionally does **not** execute simulations. Solvers consume the same
contracts directly; this crate helps inspect them consistently.

## Quick Start

```rust
use hyle_ca_analysis::analyze_spec;
use hyle_ca_interface::{
    neighbors, Blueprint, MaterialSet, NeighborhoodFalloff, NeighborhoodRadius, NeighborhoodSet,
    NeighborhoodShape, NeighborhoodSpec, RuleSpec,
};

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
    .rules([RuleSpec::when(Material::Dead)
        .require(neighbors(Material::Alive).count().eq(3))
        .becomes(Material::Alive)])
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

### Runtime Analysis

- living cell counts for a caller-supplied alive-material set
- born and died cell counts per completed step
- stable vs changed cell counts
- material populations and material-to-material transitions

```rust
use hyle_ca_analysis::analyze_step_report;
use hyle_ca_interface::{MaterialId, StepReport, TransitionCount};

let step = StepReport::new(
    4,
    3,
    vec![6, 2],
    vec![
        TransitionCount {
            from: MaterialId::new(0),
            to: MaterialId::new(1),
            count: 2,
        },
        TransitionCount {
            from: MaterialId::new(1),
            to: MaterialId::new(0),
            count: 1,
        },
    ],
);

let runtime = analyze_step_report(&step, &[MaterialId::new(1)]);
assert_eq!(runtime.living_cells, 2);
assert_eq!(runtime.born_cells, 2);
assert_eq!(runtime.died_cells, 1);
```

## Relationship To Other Crates

| Crate | Role |
|------|------|
| [`hyle-ca-interface`](https://crates.io/crates/hyle-ca-interface) | Contracts, semantics, and shared solver interfaces |
| [`hyle-ca-analysis`](https://crates.io/crates/hyle-ca-analysis) | Shared spec analysis and diagnostics |
| [`hyle-ca-solver`](https://crates.io/crates/hyle-ca-solver) | Default CPU solver implementation |
