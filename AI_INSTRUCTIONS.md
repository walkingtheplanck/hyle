# AI Instructions

## Context

Read these before making changes:
- `README.md` - project overview
- `CONTRIBUTING.md` - code style, docs, CI, commit conventions
- `crates/ca-interface/README.md` - framework traits and types
- `crates/ca-solver/README.md` - default solver implementation
- `.github/workflows/ci.yml` - CI pipeline and checks

## Terminology

This is a **framework** and **solver**, not an engine. It does not own a loop or lifecycle.

## Workflow Guidance

- After completing a feature or change, add or update tests and update relevant docs such as READMEs and doc comments.
- Git hooks are tracked in `.githooks/`. After cloning, run `git config core.hooksPath .githooks`.
- The pre-commit hook runs the same checks as CI.
- The shared test script runs both debug and release test suites so debug-only assertion behavior is exercised too.
- Viewer is excluded from CI because it needs GPU/display libraries.

## Publishing

- Crate READMEs are the single source of truth for docs via `#![doc = include_str!(...)]`.
- Tests are excluded from crates.io packages via `exclude = ["tests/"]`.
