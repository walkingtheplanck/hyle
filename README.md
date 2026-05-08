# Hyle

Hyle is a backend-agnostic frontend/runtime experiment for lattice simulation
languages. It is being reset around a config + DSL pipeline that compiles into a
typed IR and then validates execution semantics through CPU and GPU
proof-of-concept backends.

The current repository state is a scaffold. It does not yet ship a production
parser, compiler, solver, GPU backend, or viewer.

## Scope

- `hyle-ir` defines the shared typed IR, schema versioning, serde support, and
  light validation.
- `hyle-compiler` owns KDL config parsing, Hyle DSL parsing, diagnostics, name
  resolution, type checking, and lowering into IR.
- `hyle-runtime` defines backend-facing contracts shared by solver
  implementations.
- `poc/hyle-cpu` and `poc/hyle-gpu` are disposable proof-of-concept backends.
- `poc/hyle-viewer` is a disposable visualization placeholder.

The old `hyle-ca-*` crates are obsolete in this repository. Some historical
versions were published to crates.io and remain immutable there. Any cleanup of
published versions would be a separate manual decision such as yanking specific
releases; this scaffold does not automate crates.io changes.

## Workspace Layout

```text
crates/
  hyle-ir/
  hyle-compiler/
  hyle-runtime/

poc/
  hyle-cpu/
  hyle-gpu/
  hyle-viewer/
```

## Current Status

Implemented now:

- workspace scaffold
- placeholder public APIs
- minimal IR validation
- compiler/runtime/backend wiring that compiles and tests

Planned later:

- real `hyle.kdl` parsing
- real `logic.hyle` parsing
- diagnostics with source spans
- semantic resolution and type checking
- executable CPU/GPU solvers
- viewer integration

## Intended Future Inputs

`hyle.kdl`

```kdl
module "life"
lattice dimensions=2 topology="rect"
```

`logic.hyle`

```text
rule survive when alive_neighbors in 2..=3
rule birth when alive_neighbors == 3
```

Intended `ModuleIr` JSON shape:

```json
{
  "schema_version": "v1alpha1",
  "name": "life",
  "lattice": {
    "dimensions": 2,
    "topology": "rect"
  },
  "model": {
    "fields": [
      { "name": "state", "ty": "bool" }
    ]
  },
  "rules": [
    { "name": "survive", "expression": "..." }
  ],
  "pipeline": {
    "stages": [
      { "name": "step", "rules": ["survive"] }
    ]
  }
}
```

That JSON is aspirational. The current scaffold only produces placeholder IR.
