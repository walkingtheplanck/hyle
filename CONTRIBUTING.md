# Contributing to Hyle

## Getting Started

```bash
git clone https://github.com/walkingtheplanck/hyle
cd hyle
git config core.hooksPath .githooks
cargo build --workspace
```

The `core.hooksPath` command activates the tracked pre-commit and commit-msg hooks.

## Code Organization

These standards apply to the library crates (`ca-interface`, `ca-analysis`, `ca-solver`):

- `main.rs`, `lib.rs`, and `mod.rs` are entry points and re-exports only — no logic.
- Use semantically named files (`shapes.rs`, `weights.rs`, not `utils.rs` or `helpers.rs`).
- Separate responsibilities into different files or modules.

### `ca-interface` ownership boundaries

When working in `crates/ca-interface`, keep the module ownership line explicit:

- `domain`: neutral value types shared by schema and runtime code
- `schema`: declarative authoring records, builder state, and rule DSL
- `runtime`: live execution/query traits, runtime errors, and runtime-only models
- `resolved`: interpreted semantic helpers derived from declarations

If a type means the same thing in both schema and runtime code, prefer one
shared `domain` type over duplicated layer-specific copies.

The **viewer** (`tools/viewer/`) is an internal dev tool, not a published crate. It is excluded from CI, documentation standards, and code organization rules. Shortcuts and workarounds are acceptable there.

## Documentation

All public items in library crates require doc comments (`#![deny(missing_docs)]`).

### What to document

| Item | Required? |
|------|-----------|
| Public types, traits, functions | Yes |
| Public fields | Yes |
| Module-level (`//!`) | Yes |
| Private items | No |
| Re-exports in lib.rs | No (original has docs) |

### Doc comment format

```rust
/// One-line summary ending with a period.
///
/// Longer explanation if needed — when it behaves, what it assumes.
///
/// # Panics
///
/// Only if it can panic. State the condition.
pub fn thing() {}
```

Trivial getters need only the summary: `/// The grid width.`

Don't over-document:

```rust
// Bad — restates the signature
/// Gets the width.
///
/// # Returns
/// The width as u32.
pub fn width(&self) -> u32 { self.width }

// Good
/// The grid width in cells.
pub fn width(&self) -> u32 { self.width }
```

### Crate-level docs

Each crate's README is the single source of truth, included via `#![doc = include_str!("../README.md")]`.
Edit the README to update both crates.io and docs.rs.

## Commit Messages

Conventional format: `<type>: <lowercase description>`

| Type | Use for |
|------|---------|
| `feat` | New feature |
| `fix` | Bug fix |
| `refactor` | Code restructuring, no behavior change |
| `chore` | Maintenance, dependencies |
| `config` | IDE, Claude, editor, CI configuration |
| `docs` | Documentation only |
| `test` | Adding or updating tests |
| `perf` | Performance improvement |
| `ci` | CI pipeline changes |

Rules:
- Description starts lowercase, no trailing period.
- Every commit must include a description of what the change includes and the motivation for making it.
- Put that extra context in the commit body after the subject line.
- The body should briefly cover:
  - what changed
  - why the change was needed
- The commit-msg hook enforces this format.

Example:

```text
feat: add weighted neighborhood presets

Includes preset builders for common weighted neighborhoods and tests for the
new construction paths.

Motivation: make common solver setups easier to create without repeating the
same boilerplate in downstream code.
```

## CI Pipeline

CI runs on push to `master` and on pull requests. Cargo checks are skipped when only non-code files change (docs, config, etc.).

| Check | What it does |
|-------|-------------|
| Commit message | Validates conventional format |
| `cargo fmt` | Formatting |
| `cargo clippy` | Lints with `-D warnings` |
| `cargo test` | All tests (release mode) |
| `cargo doc` | Documentation with `-D warnings` |

The viewer is excluded from CI (needs GPU/display libraries).

### Running checks locally

```bash
# All checks (same as pre-commit hook)
scripts/ci-check.sh all

# Individual checks
scripts/ci-check.sh fmt
scripts/ci-check.sh clippy
scripts/ci-check.sh test
scripts/ci-check.sh doc
```

## Publishing

- Tests are excluded from crates.io packages via `exclude = ["tests/"]`.
- Crate READMEs are included via `readme = "README.md"` in Cargo.toml.
