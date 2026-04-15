# Hyle

[![docs.rs: hyle-ca-interface](https://img.shields.io/docsrs/hyle-ca-interface?label=hyle-ca-interface%20docs)](https://docs.rs/hyle-ca-interface)
[![docs.rs: hyle-ca-analysis](https://img.shields.io/docsrs/hyle-ca-analysis?label=hyle-ca-analysis%20docs)](https://docs.rs/hyle-ca-analysis)
[![docs.rs: hyle-ca-solver](https://img.shields.io/docsrs/hyle-ca-solver?label=hyle-ca-solver%20docs)](https://docs.rs/hyle-ca-solver)

A 3D cellular automaton framework for Rust.

> Define portable blueprint specs, run them on solver implementations, and keep the
> same rule semantics across CPU and GPU implementations.

---

## Crates

| Crate | Purpose |
|-------|---------|
| [`hyle-ca-interface`](crates/ca-interface) | Shared interface crate containing contracts, semantics, and runtime traits |
| [`hyle-ca-analysis`](crates/ca-analysis) | Shared spec analysis and diagnostics |
| [`hyle-ca-solver`](crates/ca-solver) | Default CPU solver that executes portable blueprint specs |

---

## Quick Start

```rust
use hyle_ca_interface::{
    neighbors, Blueprint, Instance, MaterialSet, NeighborhoodFalloff, NeighborhoodRadius,
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
            .require(neighbors(Material::Alive).count().eq(5))
            .becomes(Material::Alive),
        RuleSpec::when(Material::Alive)
            .require(neighbors(Material::Alive).count().in_range(4..=5).negate())
            .becomes(Material::Dead),
    ])
    .build()?;

let mut solver = Solver::from_spec_instance(Instance::new(64, 64, 64).with_seed(7), &spec);
solver.step();
# Ok::<(), hyle_ca_interface::BuildError>(())
```

---

## Features

### Enum-Backed Materials And Attributes

```rust
use hyle_ca_interface::{AttrAssign, AttributeSet, AttributeType, MatAttr, MaterialSet};

#[derive(Copy, Clone, Default, PartialEq, Eq)]
enum Material {
    #[default]
    Grass,
    Fire,
    Ash,
}

impl MaterialSet for Material {
    fn variants() -> &'static [Self] { &[Material::Grass, Material::Fire, Material::Ash] }
    fn label(self) -> &'static str {
        match self {
            Material::Grass => "grass",
            Material::Fire => "fire",
            Material::Ash => "ash",
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq)]
enum Attribute {
    Dryness,
    BurnAge,
}

impl AttributeSet for Attribute {
    fn variants() -> &'static [Self] { &[Attribute::Dryness, Attribute::BurnAge] }
    fn label(self) -> &'static str {
        match self {
            Attribute::Dryness => "dryness",
            Attribute::BurnAge => "burn_age",
        }
    }
    fn value_type(self) -> AttributeType { AttributeType::U8 }
}

let bindings = [
    MatAttr::new(Material::Grass, [AttrAssign::new(Attribute::Dryness).default(3u8)]),
    MatAttr::new(Material::Fire, [AttrAssign::new(Attribute::BurnAge).default(0u8)]),
    MatAttr::new(Material::Ash, []),
];

assert_eq!(bindings.len(), 3);
```

### Variable-Radius Neighborhoods

```rust
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
    fn variants() -> &'static [Self] { &[Material::Dead, Material::Alive] }
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
    fn variants() -> &'static [Self] { &[Neighborhood::Adjacent, Neighborhood::Far] }
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
            NeighborhoodRadius::new(3),
            NeighborhoodFalloff::Uniform,
        ),
    ])
    .default_neighborhood(Neighborhood::Adjacent)
    .rules([
        RuleSpec::when(Material::Dead)
            .using(Neighborhood::Far)
            .require(neighbors(Material::Alive).count().at_least(1))
            .becomes(Material::Alive),
    ])
    .build()?;
# Ok::<(), hyle_ca_interface::BuildError>(())
```

### Torus Topology

```rust
use hyle_ca_solver::{Solver, TorusTopology};

let solver = Solver::with_topology(64, 64, 64, TorusTopology);
```

Reads, writes, and rule neighborhoods all wrap across grid edges.

### Debug Contract Validation

```rust
use hyle_ca_interface::ValidatedSolver;

// Wraps any solver, asserts contracts on every call (debug builds only)
let validated = ValidatedSolver::new(solver);
```

---

## Viewer

```
cargo run --release -p hyle-viewer
```

3D Game of Life (Life 4555) with GPU raytracing.

| Input | Action |
|---|---|
| Right-drag | Orbit camera |
| Middle-drag | Pan |
| Scroll | Zoom |
| WASD / QE | Move camera |
| R | Reset |
| Tab | Toggle mouse capture |

---

## TODO

### Contracts And Spec

- [x] **Declarative blueprint specs** - Portable builder API and canonical spec shared across solver implementations
- [x] **Named neighborhoods** - Reusable neighborhood definitions referenced by rules
- [x] **Descriptor-backed topology** - Uploadable topology descriptors with bounded and torus behavior
- [x] **Enum-backed material metadata** - Declare materials, attributes, and neighborhoods explicitly so tools can analyze specs without guessing
- [ ] **Spec serialization** - Save and load automaton specs and grid patterns in a stable portable format
- [ ] **Stricter validation** - Catch invalid random gates, impossible thresholds, and other malformed specs earlier
- [ ] **Solver-specific support checks** - Keep any execution-limit or solver support checks in solver crates instead of the shared analysis layer

### Analysis And Tooling

- [x] **Shared analysis crate** - Optional crate for spec-derived analysis and diagnostics
- [x] **Rule diagnostics** - Detect shadowed rules, duplicate rules, and unused neighborhoods from the spec alone
- [ ] **Simulation analysis tools** - Population counts, entropy, and other runtime-facing metrics exposed through shared APIs
- [ ] **Architecture docs** - Document the mental model clearly: builder -> spec -> blueprint -> analysis -> solver runtime

### Backends

- [x] **Default CPU solver** - Execute declarative specs with deterministic first-match semantics
- [ ] **GPU solver** - Compile the same spec model to a GPU execution path
- [ ] **Parallel CPU stepping** - Rayon or equivalent, preserving current deterministic semantics

### World And Topology

- [x] **Neighborhood types** - Moore, Von Neumann, Spherical shapes plus configurable weight functions
- [x] **Global torus topology** - Wrapping boundaries for the whole solver grid
- [ ] **Explicit portals / stitched regions** - Model nontrivial space connections before reaching for full regional topology
- [ ] **Regional topology control** - Let different areas of the same world resolve coordinates differently if real use cases justify it
- [ ] **Chunk-based sparse storage** - Skip empty regions and scale to larger worlds

### Examples And UX

- [ ] **Enum-first examples** - Prefer readable named states over raw numeric literals in docs and examples
- [ ] **More complete example library** - Include richer examples beyond Life-style rules
- [ ] **Editor ergonomics** - Reduce type inference friction in the DSL and improve diagnostics for builder usage

---

## License

GPL-3.0-only
