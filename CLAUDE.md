# CLAUDE.md

Use this file as the strict operational checklist for working in this repo.

- Always work in a dedicated AI worktree under `.ai/worktrees/`.
- If a usable worktree already exists there, use it instead of the repository root.
- If no usable worktree exists there, create one under `.ai/worktrees/`.
- When creating the worktree branch, use the local branch name `ai/worktree`.
- Before making changes, check whether the worktree branch is behind `master`.
- If it is behind `master`, merge `master` first.
- If that merge creates conflicts, stop and ask before resolving them.
- Always commit after completing a change.
- Use conventional commit subject format: `<type>: <lowercase description>`.
- Every commit must include a body that states what changed and why.
- When adding or changing functionality, add or update tests to cover it.
- Run the relevant test suite after each meaningful change set, not only at the end.
- If tests fail because of your changes, treat that as a regression and fix it before continuing or committing.
- Do not place code in `mod.rs`, `lib.rs`, or `main.rs`; keep implementation in dedicated module files instead.
- Prefer rustdoc-style comments that explain intent, invariants, and motivation instead of restating syntax.
- Document all public items, all `pub(crate)` items with invariants, and any hidden/public helper such as `from_validated` or `*_unchecked`.
- For private items, add comments only when the behavior, contract, or tradeoff would not be obvious from the code alone.
- Do not add comments that merely restate what the syntax already says.

## Scope Notes

- The repository is a scaffold-only reset for a backend-agnostic frontend/runtime experiment.
- The old `hyle-ca-*` implementation is obsolete in this checkout.
- The core crates are `hyle-ir`, `hyle-compiler`, and `hyle-runtime`.
- The POC crates under `poc/` are disposable and should stay shallow.

For repo context, terminology, docs to read first, testing expectations, and publishing guidance, read [`AI_INSTRUCTIONS.md`](./AI_INSTRUCTIONS.md).
