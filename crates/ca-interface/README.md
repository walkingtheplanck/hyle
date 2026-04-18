# hyle-ca-interface

Shared interfaces, schema types, and runtime traits for the [Hyle](https://github.com/walkingtheplanck/hyle) cellular automaton framework.

This crate defines the shared public interface layer. Depend on it to:
- define enum-backed material, attribute, and neighborhood sets
- author portable blueprints with `Blueprint::builder()`
- implement new solver implementations against the shared `CaSolver` trait
- centralize backend construction through `CaRuntime` and `CaSolverProvider`

Derived analysis and diagnostics live in
[`hyle-ca-analysis`](https://crates.io/crates/hyle-ca-analysis). Canonical
resolved helpers live in this crate under `hyle_ca_interface::resolved`.

It has **zero dependencies** and is split conceptually into:
- `schema` for declarative blueprint and descriptor data
- `resolved` for canonical interpreted blueprint, neighborhood, and topology meaning
- `runtime` for running-simulation interfaces, split into execution traits, runtime error types, and small runtime models

## Key Types

| Type | Role |
|------|------|
| [`MaterialSet`] / [`AttributeSet`] / [`NeighborhoodSet`] | Enum-backed registries for portable blueprint authoring |
| [`Instance`] | Runtime dimensions and deterministic seed for one solver run |
| [`Blueprint`] | Declarative blueprint builder and canonical contract |
| [`CaSolver`] | Trait that all solver implementations implement |
| [`CaRuntime`] / [`Runtime`] / [`CaSolverProvider`] | Consumer-facing runtime trait, standard runtime wrapper, and factory interface |
| [`GridDims`] / [`GridRegion`] / [`GridSnapshot`] | Solver-neutral grid descriptors and bulk transfer types |
| [`NeighborhoodSpec`] | Declarative neighborhood description shared across solvers |
| [`Weight`] | Fixed-point weight threshold used by weighted neighborhood predicates |
| [`Rng`] | Shared deterministic random-number primitive parameterized by seed, position, step, and stream |
| [`CellId`] | Opaque runtime handle to one logical cell for query-oriented tooling |
| [`Topology`] / [`TopologyDescriptor`] | Boundary behavior traits and descriptors |

Resolved forms are available under `hyle_ca_interface::resolved`, for example:
- `hyle_ca_interface::resolved::ResolvedBlueprint`
- `hyle_ca_interface::resolved::Neighborhood`
- `hyle_ca_interface::resolved::ResolvedTopology`

## Preferred Imports

Use the crate root or the prelude as the main entry points:

- Prefer explicit root imports for application and library code:
  `use hyle_ca_interface::{Blueprint, CaSolverProvider, Instance};`
- Use `hyle_ca_interface::prelude::*` when you want a compact common import set
  for blueprint authoring and runtime setup.
- Treat `hyle_ca_interface::resolved` as an advanced namespace for interpreted
  forms and semantic helpers.

The internal `schema`, `resolved`, and `runtime` module layout is crate organization, not
the intended consumer-facing path.

## Building a Portable Blueprint

```rust
use hyle_ca_interface::{
    neighbors, Blueprint, MaterialSet, NeighborhoodFalloff, NeighborhoodRadius, NeighborhoodSet,
    NeighborhoodShape, NeighborhoodSpec, RuleSpec, TopologyDescriptor,
};

#[derive(Copy, Clone, Default, PartialEq, Eq, MaterialSet)]
enum Material {
    #[default]
    Dead,
    Alive,
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
    .topology(TopologyDescriptor::bounded())
    .neighborhoods::<Neighborhood>()
    .neighborhood_specs([NeighborhoodSpec::new(
        Neighborhood::Adjacent,
        NeighborhoodShape::Moore,
        NeighborhoodRadius::new(1),
        NeighborhoodFalloff::Uniform,
    )])
    .rules([
        RuleSpec::when(Material::Dead)
            .require(neighbors(Material::Alive).count().eq(3))
            .becomes(Material::Alive),
        RuleSpec::when(Material::Alive)
            .require(neighbors(Material::Alive).count().in_range(2..=3).negate())
            .becomes(Material::Dead),
    ])
    .build()?;
# Ok::<(), hyle_ca_interface::BuildError>(())
```

Rules are evaluated in declaration order with **first-match wins** semantics.
If no rule matches, the center cell is kept unchanged.

## Attached Attributes

Blueprints can declare named attached per-cell attributes and use them in rules:

```rust
use hyle_ca_interface::{
    attr, AttrAssign, AttributeSet, AttributeType, AttributeValue, Blueprint, MatAttr,
    MaterialSet, NeighborhoodFalloff, NeighborhoodRadius, NeighborhoodSet, NeighborhoodShape,
    NeighborhoodSpec, RuleSpec,
};

#[derive(Copy, Clone, Default, PartialEq, Eq)]
enum Material {
    #[default]
    Idle,
    Hot,
}

impl MaterialSet for Material {
    fn variants() -> &'static [Self] {
        &[Material::Idle, Material::Hot]
    }

    fn label(self) -> &'static str {
        match self {
            Material::Idle => "idle",
            Material::Hot => "hot",
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq)]
enum Attribute {
    Heat,
    Charged,
}

impl AttributeSet for Attribute {
    fn variants() -> &'static [Self] {
        &[Attribute::Heat, Attribute::Charged]
    }

    fn label(self) -> &'static str {
        match self {
            Attribute::Heat => "heat",
            Attribute::Charged => "charged",
        }
    }

    fn value_type(self) -> AttributeType {
        match self {
            Attribute::Heat => AttributeType::U8,
            Attribute::Charged => AttributeType::Bool,
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
    .attributes::<Attribute>()
    .material_attributes([
        MatAttr::new(Material::Idle, []),
        MatAttr::new(
            Material::Hot,
            [
                AttrAssign::new(Attribute::Heat).default(2u8),
                AttrAssign::new(Attribute::Charged).default(true),
            ],
        ),
    ])
    .neighborhoods::<Neighborhood>()
    .neighborhood_specs([NeighborhoodSpec::new(
        Neighborhood::Adjacent,
        NeighborhoodShape::Moore,
        NeighborhoodRadius::new(1),
        NeighborhoodFalloff::Uniform,
    )])
    .rules([RuleSpec::when(Material::Hot)
        .require(attr(Attribute::Heat).at_least(2u8))
        .set_attr(Attribute::Heat, 0u8)
        .keep()])
    .build()?;

assert_eq!(spec.attributes().len(), 2);
# Ok::<(), hyle_ca_interface::BuildError>(())
```

These attributes are part of the portable blueprint contract. The current DSL
supports center-cell attribute predicates and rule-driven attribute writes.

## Decoupled Runtime Construction

Consumers such as viewers can depend on the centralized runtime/provider seam
instead of naming a concrete solver type directly. Providers return the
standard [`Runtime<S>`] wrapper over a concrete solver:

```ignore
use hyle_ca_interface::{Blueprint, CaRuntime, CaSolverProvider, Instance, MaterialSet};
use hyle_ca_solver::CpuSolverProvider;

#[derive(Copy, Clone, Default, PartialEq, Eq)]
enum Material {
    #[default]
    Empty,
}

impl MaterialSet for Material {
    fn variants() -> &'static [Self] { &[Material::Empty] }
    fn label(self) -> &'static str { "empty" }
}

let spec = Blueprint::builder().materials::<Material>().build()?;
let provider = CpuSolverProvider::new();
let runtime = provider.build(Instance::new(16, 16, 16), &spec);

# Ok::<(), hyle_ca_interface::BuildError>(())
```

This keeps backend selection localized to one construction site while preserving static dispatch.
When a consumer needs low-level runtime metrics and inspection data, both
`CaSolver` and `CaRuntime` expose raw query methods for cells, neighbors, current
populations, changed-cell counts, and latest material transitions instead of one
bundled report object.

## Defining Symbol Sets

```rust
use hyle_ca_interface::{AttributeSet, AttributeType, MaterialSet};

#[derive(Copy, Clone, Default, PartialEq, Eq)]
enum Material {
    #[default]
    Empty,
    Fluid,
}

impl MaterialSet for Material {
    fn variants() -> &'static [Self] {
        &[Material::Empty, Material::Fluid]
    }

    fn label(self) -> &'static str {
        match self {
            Material::Empty => "empty",
            Material::Fluid => "fluid",
        }
    }
}
 
#[derive(Copy, Clone, PartialEq, Eq)]
enum Attribute {
    Density,
}

impl AttributeSet for Attribute {
    fn variants() -> &'static [Self] { &[Attribute::Density] }
    fn label(self) -> &'static str { "density" }
    fn value_type(self) -> AttributeType { AttributeType::U8 }
}
```

Materials stay compact and discrete for solver performance. Extra per-cell data
belongs in blueprint attributes, which solvers can store as SoA channels.

## Grid Descriptors

```rust
# fn main() -> Result<(), hyle_ca_interface::GridShapeError> {
use hyle_ca_interface::{GridDims, GridRegion, GridSnapshot, MaterialId};

let dims = GridDims::new(8, 8, 8)?;
let region = GridRegion::new([2, 2, 2], [2, 2, 2])?;
let snapshot = GridSnapshot::new(dims, vec![MaterialId::default(); dims.cell_count()]);

assert!(dims.contains_region(region));
assert_eq!(snapshot.cells.len(), dims.cell_count());
# Ok(())
# }
```

## Deterministic RNG

```rust
# fn main() -> Result<(), hyle_ca_interface::GridShapeError> {
use hyle_ca_interface::{Instance, Rng};

let instance = Instance::new(64, 64, 64)?.with_seed(42);
let rng = Rng::with_seed(10, 20, 30, 4, instance.seed());

assert!(rng.chance(1));
assert!(rng.range(8) < 8);
# Ok(())
# }
```

This RNG is deterministic and portable: the same `(seed, x, y, z, step, stream)`
input always produces the same output across all solvers.

## Declarative Neighborhoods

```ignore
use hyle_ca_interface::{
    NeighborhoodFalloff, NeighborhoodRadius, NeighborhoodSet, NeighborhoodShape, NeighborhoodSpec,
};

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

let adjacent = NeighborhoodSpec::new(
    Neighborhood::Adjacent,
    NeighborhoodShape::Moore,
    NeighborhoodRadius::new(1),
    NeighborhoodFalloff::Uniform,
);
let far = NeighborhoodSpec::new(
    Neighborhood::Far,
    NeighborhoodShape::Moore,
    NeighborhoodRadius::new(2),
    NeighborhoodFalloff::Uniform,
);

assert_eq!(adjacent.radius().get(), 1);
assert_eq!(far.radius().get(), 2);
assert_eq!(far.shape(), NeighborhoodShape::Moore);
assert_eq!(far.falloff(), NeighborhoodFalloff::Uniform);
```

Neighborhood falloff expands to deterministic fixed-point weights in the semantic layer,
so CPU and GPU backends can agree on the same values exactly.

Weighted predicates use the same portable units:

```ignore
use hyle_ca_interface::{neighbors, MaterialSet, Weight};

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

let condition = neighbors(Material::Alive)
    .weighted_sum()
    .at_least(Weight::cells(2));
```

## Topology

Solvers choose how coordinates beyond the grid bounds behave by implementing [`Topology`].
Built-in CPU topology implementations live in
[`hyle-ca-solver`](https://crates.io/crates/hyle-ca-solver), but the portable
descriptor type lives here:

```rust
use hyle_ca_interface::{AxisTopology, TopologyDescriptor};

let bounded = TopologyDescriptor::bounded();
let mixed = TopologyDescriptor::by_axis(
    AxisTopology::Wrap,
    AxisTopology::Bounded,
    AxisTopology::Wrap,
);

assert_eq!(bounded.x, AxisTopology::Bounded);
assert_eq!(mixed.y, AxisTopology::Bounded);
```

## Implementing a Solver

Implement the [`CaSolver`] trait to create a custom solver (GPU, distributed, etc.).
Solvers are expected to consume a portable representation such as [`Blueprint`]
and uphold the runtime contract documented on the trait.
