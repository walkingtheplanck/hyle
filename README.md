# Hyle

[![docs.rs: hyle-ca-contracts](https://img.shields.io/docsrs/hyle-ca-contracts?label=hyle-ca-contracts%20docs)](https://docs.rs/hyle-ca-contracts)
[![docs.rs: hyle-ca-solver](https://img.shields.io/docsrs/hyle-ca-solver?label=hyle-ca-solver%20docs)](https://docs.rs/hyle-ca-solver)

A 3D cellular automaton framework for Rust.

> Define portable automaton specs, run them on solver backends, and keep the
> same rule semantics across CPU and GPU implementations.

---

## Crates

| Crate | Purpose |
|-------|---------|
| [`hyle-ca-contracts`](crates/ca-contracts) | Shared contracts, descriptors, and declarative automaton specs |
| [`hyle-ca-solver`](crates/ca-solver) | Default CPU solver that executes portable automaton specs |

---

## Quick Start

```rust
use hyle_ca_contracts::{neighbors, Hyle};
use hyle_ca_solver::Solver;

let spec = Hyle::builder()
    .cells::<u32>()
    .rules(|rules| {
        rules.when(0).require(neighbors(1).count().eq(5)).becomes(1);
        rules.when(1).unless(neighbors(1).count().in_range(4..=5)).becomes(0);
    })
    .build()?;

let mut solver = Solver::from_spec(64, 64, 64, &spec);
solver.step();
# Ok::<(), hyle_ca_contracts::BuildError>(())
```

---

## Features

### Custom Cell Types

```rust
#[derive(Copy, Clone, Default)]
struct FluidCell { density: u8, velocity: [i8; 6], material: u8 }

impl Cell for FluidCell {
    fn rule_id(&self) -> u8 { self.material }
    fn is_alive(&self) -> bool { self.density > 0 }
}

let solver = Solver::<FluidCell>::new(64, 64, 64);
```

### Variable-Radius Neighborhoods

```rust
use hyle_ca_contracts::{neighbors, Hyle, NeighborhoodSpec};

let spec = Hyle::builder()
    .cells::<u32>()
    .neighborhood("far", NeighborhoodSpec::cube(3))
    .rules(|rules| {
        rules.when(0)
            .using("far")
            .require(neighbors(1).count().at_least(1))
            .becomes(1);
    })
    .build()?;
# Ok::<(), hyle_ca_contracts::BuildError>(())
```

### Torus Topology

```rust
use hyle_ca_solver::TorusTopology;

let solver = Solver::<u32>::with_topology(64, 64, 64, TorusTopology);
```

Reads, writes, rule neighborhoods, and world passes all wrap across grid edges.

### Debug Contract Validation

```rust
use hyle_ca_contracts::ValidatedSolver;

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

- [x] **Declarative automaton specs** - Portable builder API and canonical spec shared across backends
- [x] **Named neighborhoods** - Reusable neighborhood definitions referenced by rules
- [x] **Descriptor-backed topology** - Uploadable topology descriptors with bounded and torus behavior
- [ ] **State/schema metadata** - Declare the valid state space more explicitly so tools can analyze specs without guessing
- [ ] **Spec serialization** - Save and load automaton specs and grid patterns in a stable portable format
- [ ] **Capability checks** - Validate a spec against backend feature limits before runtime

### Analysis And Tooling

- [ ] **Shared analysis crate** - Move spec analysis, diagnostics, and backend compatibility checks into a separate optional crate
- [ ] **Rule diagnostics** - Detect shadowed rules, unused neighborhoods, and other first-match issues from the spec alone
- [ ] **Simulation analysis tools** - Population counts, entropy, and other runtime-facing metrics exposed through shared APIs
- [ ] **Architecture docs** - Document the mental model clearly: builder -> spec -> analysis -> backend runtime

### Backends

- [x] **Default CPU backend** - Execute declarative specs with deterministic first-match semantics
- [ ] **GPU backend** - Compile the same spec model to a GPU execution path
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
