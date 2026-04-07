# CLAUDE.md

## Context

Read these before making changes:
- `README.md` — project overview
- `crates/ca-core/README.md` — framework traits and types
- `crates/ca-solver/README.md` — default solver implementation

## Terminology

This is a **framework** and **solver**, not an engine. It does not own a loop or lifecycle.

## Workflow

- Always commit after completing a change — especially in worktrees.
- Use conventional commit format: `<type>: <lowercase description>`
  - Types: feat, fix, refactor, chore, docs, test, perf, ci
- Pre-commit hook runs CI checks (fmt, clippy, test, doc).
- Viewer is excluded from CI (needs GPU/display libs).

## Publishing

- Crate READMEs are the single source of truth for docs (`#![doc = include_str!]`).
- Tests are excluded from crates.io packages via `exclude = ["tests/"]`.
