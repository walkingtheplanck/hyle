# hyle-ca-analysis

Shared analysis and diagnostics for the [Hyle](https://github.com/walkingtheplanck/hyle) cellular automaton framework.

This crate builds on [`hyle-ca-interface`](https://crates.io/crates/hyle-ca-interface) and provides
**derived tooling** over declarative blueprints:
- static spec summaries
- rule and neighborhood diagnostics
- runtime report analysis over completed solver steps
- single-cell reports over runtime query APIs

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
use hyle_ca_analysis::analyze_runtime;
use hyle_ca_interface::{
    Blueprint, CaSolver, MaterialSet, NeighborhoodFalloff, NeighborhoodRadius, NeighborhoodSet,
    NeighborhoodShape, NeighborhoodSpec, RuleSpec,
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
    .rules([RuleSpec::when(Material::Alive).becomes(Material::Dead)])
    .build()?;

let mut solver = Solver::from_spec(2, 2, 2, &spec);
solver.set(0, 0, 0, Material::Alive.id());
solver.step();

let runtime = analyze_runtime(&solver, &[Material::Alive.id()]);
assert_eq!(runtime.living_cells, 0);
assert_eq!(runtime.died_cells, 1);
# Ok::<(), hyle_ca_interface::BuildError>(())
```

### Cell Analysis

- current material and attached attributes for one selected position
- resolved in-bounds cell handle and canonical position
- neighborhood-by-neighborhood material summaries around the selected cell

```rust
use hyle_ca_analysis::analyze_cell;
use hyle_ca_interface::{
    Blueprint, CaSolver, MaterialSet, NeighborhoodFalloff, NeighborhoodRadius, NeighborhoodSet,
    NeighborhoodShape, NeighborhoodSpec,
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
    .build()?;

let mut solver = Solver::from_spec(3, 3, 3, &spec);
solver.set(1, 1, 1, Material::Alive.id());

let report = analyze_cell(&solver, [1, 1, 1]).expect("cell exists");
assert_eq!(report.material.name, "alive");
assert_eq!(report.neighborhoods[0].name, "adjacent");
# Ok::<(), hyle_ca_interface::BuildError>(())
```

## Relationship To Other Crates

| Crate | Role |
|------|------|
| [`hyle-ca-interface`](https://crates.io/crates/hyle-ca-interface) | Contracts, semantics, and shared solver interfaces |
| [`hyle-ca-analysis`](https://crates.io/crates/hyle-ca-analysis) | Shared spec analysis and diagnostics |
| [`hyle-ca-solver`](https://crates.io/crates/hyle-ca-solver) | Default CPU solver implementation |
