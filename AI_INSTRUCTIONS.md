# AI Instructions

## Read First

- `README.md` - current project scope and workspace layout
- `CONTRIBUTING.md` - code style, commit rules, and CI expectations
- `.github/workflows/ci.yml` - authoritative CI pipeline

## Repository State

- This repo is in a scaffold-only reset.
- The old `hyle-ca-*` APIs and architecture are obsolete here.
- Keep core implementation focused on:
  - `crates/hyle-ir`
  - `crates/hyle-compiler`
  - `crates/hyle-runtime`
- Treat `poc/hyle-cpu`, `poc/hyle-gpu`, and `poc/hyle-viewer` as disposable
  proof-of-concept crates.

## Workflow Guidance

- Prefer deletion over adapting obsolete cellular automata code.
- Keep parsers, solvers, and viewer code intentionally shallow until the new
  architecture stabilizes.
- Add or update tests for meaningful changes, even when the behavior is only
  scaffold-level.
- Git hooks are tracked in `.githooks/`. After cloning, run
  `git config core.hooksPath .githooks`.
- The pre-commit hook mirrors CI and should pass from both the main checkout and
  the AI worktree.

## Publishing Guidance

- Published crates from the old scope are immutable on crates.io.
- Do not automate yanks or publishes as part of routine scaffold work.
- If publishable crates are added later, keep POC crates out of the publish
  path unless there is a deliberate reason to ship them.
