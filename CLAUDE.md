# CLAUDE.md

## Context

Read these before making changes:
- `README.md` — project overview
- `CONTRIBUTING.md` — code style, docs, CI, commit conventions
- `crates/ca-core/README.md` — framework traits and types
- `crates/ca-solver/README.md` — default solver implementation
- `.github/workflows/ci.yml` — CI pipeline and checks

## Terminology

This is a **framework** and **solver**, not an engine. It does not own a loop or lifecycle.

## Workflow

- At the start of a session, check if the worktree branch is behind master. If it is, merge master into the branch. If there are merge conflicts, ask before resolving.
- Always commit after completing a change — especially in worktrees.
- Use conventional commit format: `<type>: <lowercase description>`
  - Types: feat, fix, refactor, chore, config, docs, test, perf, ci
- After completing a feature or change, add/update tests and update relevant docs (READMEs, doc comments).
- Git hooks are tracked in `.githooks/`. After cloning, run: `git config core.hooksPath .githooks`
- Pre-commit hook runs CI checks (fmt, clippy, test, doc).
- Viewer is excluded from CI (needs GPU/display libs).

## Code Organization

- No logic in `main.rs`, `lib.rs`, or `mod.rs` — these are entry points and re-exports only.
- Use semantically named files (`shapes.rs`, `weights.rs`, not `utils.rs` or `helpers.rs`).
- Separate responsibilities into different files or modules.

## Publishing

- Crate READMEs are the single source of truth for docs (`#![doc = include_str!]`).
- Tests are excluded from crates.io packages via `exclude = ["tests/"]`.
