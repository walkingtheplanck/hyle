# AI Instructions

## Context

Read these before making changes:
- `README.md` - project overview
- `CONTRIBUTING.md` - code style, docs, CI, commit conventions
- `crates/ca-core/README.md` - framework traits and types
- `crates/ca-solver/README.md` - default solver implementation
- `.github/workflows/ci.yml` - CI pipeline and checks

## Terminology

This is a **framework** and **solver**, not an engine. It does not own a loop or lifecycle.

## Workflow

- Use a dedicated AI worktree under `.ai/worktrees/` when working on the repository.
- Before making changes, check whether `.ai/worktrees/` already contains a usable worktree for the task.
- If a worktree exists there, work inside that worktree instead of the repository root.
- If no worktree exists there, create one under `.ai/worktrees/` and work there.
- When creating the AI worktree, use the local branch name `ai/worktree`.
- At the start of a session, check if the worktree branch is behind `master`. If it is, merge `master` into the branch. If there are merge conflicts, ask before resolving.
- Always commit after completing a change, especially in worktrees.
- Use conventional commit format: `<type>: <lowercase description>`
- Allowed types: `feat`, `fix`, `refactor`, `chore`, `config`, `docs`, `test`, `perf`, `ci`
- After completing a feature or change, add or update tests and update relevant docs such as READMEs and doc comments.
- Git hooks are tracked in `.githooks/`. After cloning, run `git config core.hooksPath .githooks`
- The pre-commit hook runs CI checks: `fmt`, `clippy`, `test`, and `doc`
- Viewer is excluded from CI because it needs GPU/display libraries.

## Publishing

- Crate READMEs are the single source of truth for docs via `#![doc = include_str!(...)]`
- Tests are excluded from crates.io packages via `exclude = ["tests/"]`
