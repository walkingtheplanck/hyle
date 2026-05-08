# Contributing to Hyle

## Getting Started

```bash
git clone https://github.com/walkingtheplanck/hyle
cd hyle
git config core.hooksPath .githooks
cargo build --workspace
```

The `core.hooksPath` command activates the tracked pre-commit and commit-msg
hooks.

## Code Organization

These standards apply to the core crates under `crates/`:

- `main.rs`, `lib.rs`, and `mod.rs` are entry points and re-exports only.
- Use semantically named files instead of catch-all modules.
- Keep the current ownership split explicit:
  - `hyle-ir`: shared typed IR and light validation
  - `hyle-compiler`: source ingestion, parsing, diagnostics, and lowering
  - `hyle-runtime`: backend-facing contracts

The proof-of-concept crates under `poc/` are intentionally disposable. Keep them
small and avoid introducing stable APIs there unless the architecture is already
settled in the core crates.

## Documentation

Document public APIs in the core crates.

### What to document

| Item | Required? |
|------|-----------|
| Public types, traits, functions | Yes |
| Public fields | Yes |
| Module-level docs | Helpful when the module has a distinct role |
| Private items | Only when behavior or invariants are non-obvious |

### Doc comment format

```rust
/// One-line summary ending with a period.
///
/// Longer explanation when intent, invariants, or tradeoffs are not obvious.
///
/// # Errors
///
/// State the error cases when relevant.
pub fn thing() {}
```

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

- Description starts lowercase, with no trailing period.
- Every commit must include a body that states what changed and why.

## CI Pipeline

CI runs on push to `master` and on pull requests.

| Check | What it does |
|-------|-------------|
| Commit message | Validates conventional format |
| `cargo fmt` | Formatting |
| `cargo clippy` | Lints with `-D warnings` |
| `cargo test` | Workspace tests in debug and release |
| `cargo doc` | Documentation with `-D warnings` |

### Running checks locally

```bash
scripts/ci-check.sh all
scripts/ci-check.sh fmt
scripts/ci-check.sh clippy
scripts/ci-check.sh test
scripts/ci-check.sh doc
```

## Publishing

- Do not automate crates.io changes during scaffold work.
- Old published `hyle-ca-*` crates remain immutable historical artifacts.
- Keep future publish workflows focused on the core crates unless there is a
  deliberate reason to ship a POC crate.
