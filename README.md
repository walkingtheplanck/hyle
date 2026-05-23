# Hyle

Hyle is an experimental lattice-simulation language and runtime stack. A
`.hyle` script is parsed and lowered into `.sole` JSON, then executed through a
backend-agnostic runtime contract.

The repository is still early, but it is no longer just a scaffold: the parser,
compiler lowering, SOLE data model, public SDK facade, runtime contract, CPU
backend, FFI surfaces, quickstart example, and viewer core all compile and have
tests.

## Workspace Layout

```text
crates/
  hyle/                         public Rust SDK facade
  hyle-sole/                    .sole IR data model and JSON codec
  hyle-compiler/                .hyle parser and compiler
  hyle-runtime/                 backend-facing runtime contract
  hyle-ffi/
    hyle-compiler/              C ABI compiler surface
    hyle-runtime/               C ABI runtime surface
  backends/
    hyle-cpu/                   functional CPU runtime backend
    hyle-gpu/                   GPU backend scaffold
  tools/
    hyle-viewer/                viewer data model and executable

examples/
  quickstart/                   SDK example crate
  game.hyle                     sample Hyle script
  game.sole.json                expected compiled SOLE fixture
```

## Current Status

Implemented now:

- `.hyle` lexer and parser for the current single-file Hyle 0.1 syntax
- semantic lowering into `.sole` JSON
- `hyle-sole` typed IR model and serde JSON codec
- `hyle` public SDK facade with `compiler`, `runtime`, `sole`, and `prelude`
  modules
- `hyle-runtime` contract built around `SolverBackend`, `Instance`, `Step`,
  `InputAccess`, `CellRead`, `CellWrite`, and `EncodedCellIo`
- functional `hyle-cpu` backend with:
  - module validation
  - model/input/field lookup
  - cell storage by model and position
  - default field initialization
  - input get/set with bounds validation
  - cell add/update/upsert/remove
  - field read/write
  - rule stepping with literals, inputs, locals, field reads, arithmetic,
    comparisons, booleans, `clamp`, `Sum` reductions, neighborhoods, sampling,
    and transform rules
- `hyle-gpu` backend scaffold that validates runtime wiring
- compiler/runtime FFI crates under `crates/hyle-ffi`
- `examples/quickstart`, which compiles `examples/game.hyle`, initializes the
  CPU solver, mutates cells, and steps the runtime
- `hyle-viewer` view-state library and executable that can load `.sole.json`,
  seed demo cells, emit a `ViewerFrame`, and export a WebGL HTML view


## Quickstart

Run the SDK example:

```sh
cargo run -p hyle-quickstart
```

Expected output:

```text
ran cpu solver with 2 cells
```

Run the full test suite:

```sh
cargo test --workspace
```

## Viewer

Generate a standalone WebGL HTML view from a `.sole.json` module:

```sh
cargo run -p hyle-viewer -- examples/game.sole.json --out viewer.html
```

Optionally also write the intermediate viewer frame:

```sh
cargo run -p hyle-viewer -- examples/game.sole.json \
  --out viewer.html \
  --frame-json frame.json \
  --grid 4
```

The current viewer is not yet the planned live editor/runtime studio. It is a
viewer core plus executable that turns SOLE metadata and cell batches into a
renderable frame and exported WebGL HTML.

## Example Input

```text
#hyle 0.1
#dimensions 3
#cell Cube

neighborhood VonNeumann1 {
    radius 1
    center false
    metric Manhattan
}

model Fire {
    resolution 2
    range VonNeumann1

    fields {
        intensity: Float<0.5> [0.0 1.0)
    }
}

in wind_speed: Float<0.01> [0.0 250];

Fire -> Fire {
    let incoming = sum n in neighbors(Fire) {
        n.intensity
    };

    next Fire.intensity = clamp(
        Fire.intensity + incoming - 0.01 - wind_speed,
        0.0,
        1.0
    );
}
```

See [examples/game.hyle](examples/game.hyle) and
[examples/game.sole.json](examples/game.sole.json) for the current end-to-end
fixture.

## Public SDK Shape

Typical Rust usage goes through the `hyle` facade:

```rust
use hyle::prelude::*;
use hyle_cpu::CpuSolver;

let output = compile(
    CompileInput {
        source: SourceFile::new("game.hyle", source),
        module_name: Some("game".to_owned()),
    },
    CompileOptions::default(),
)?;

let sole_json = output.module.to_json_string()?;
let solver = solver(CpuSolver);
let mut instance = solver.init(sole_json.as_bytes())?;

instance.set_input("wind_speed", HyleValue::F32(0.2))?;
instance.step()?;
```

## Notes

The old `hyle-ca-*` crates are obsolete in this repository. Some historical
versions were published to crates.io and remain immutable there. Any cleanup of
published versions would be a separate manual decision such as yanking specific
releases; this repository does not automate crates.io changes.
