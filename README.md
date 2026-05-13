# Hyle

Hyle is a backend-agnostic frontend/runtime experiment for lattice simulation
languages. It is being reset around a single `.hyle` script pipeline that parses
and lowers into typed compiler IR before future `.sole` code generation.

The current repository state is a scaffold. It does not yet ship a production
parser, compiler, solver, GPU backend, or viewer.

## Scope

- `hyle-compiler` owns syntax analysis, semantic lowering, typed IR, schema
  versioning, serde support, light validation, and the future `.sole` codegen
  stage.
- `hyle-runtime` defines backend-facing contracts shared by solver
  implementations.
- `backends/hyle-cpu` and `backends/hyle-gpu` are experimental backend
  implementations.
- `tools/hyle-viewer` is a disposable visualization placeholder.

The old `hyle-ca-*` crates are obsolete in this repository. Some historical
versions were published to crates.io and remain immutable there. Any cleanup of
published versions would be a separate manual decision such as yanking specific
releases; this scaffold does not automate crates.io changes.

## Workspace Layout

```text
crates/
  hyle-compiler/
  hyle-runtime/

backends/
  hyle-cpu/
  hyle-gpu/

tools/
  hyle-viewer/
```

## Current Status

Implemented now:

- workspace scaffold
- placeholder public APIs
- parser and lexer for the single-file `.hyle` script format
- semantic lowering into compiler-owned IR
- code generation scaffold for future `.sole` output
- compiler/runtime/backend wiring that compiles and tests

Planned later:

- diagnostics with source spans
- expression IR and full type checking
- `.sole` code generation
- executable CPU/GPU solvers
- viewer integration

## Input

```text
#hyle 0.1
#dimensions 3
#cell Cube

model Fire {
    fields {
        intensity: Float<0.5> [0.0 1.0)
    }
}

Fire -> Fire {
    next Fire.intensity = Fire.intensity;
}
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
  "models": [],
  "rules": [
    { "name": "rule_0_Fire_to_Fire", "sources": [], "output": "Fire" }
  ],
  "pipeline": {
    "stages": [
      { "name": "step", "rules": ["survive"] }
    ]
  }
}
```

The `.sole` output is intentionally still a scaffold.
