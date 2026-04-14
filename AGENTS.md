# AGENTS.md

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
- Prefer explanations in simple terms that are easy to digest, especially when clarifying architecture, tradeoffs, or errors.
- When adding or changing functionality, add or update tests to cover it.
- Run the relevant test suite after each meaningful change set, not only at the end.
- If tests fail because of your changes, treat that as a regression and fix it before continuing or committing.
- Do not place code in `mod.rs`, `lib.rs`, or `main.rs`; keep implementation in dedicated module files instead.

For repo context, terminology, docs to read first, testing expectations, and publishing guidance, read [`AI_INSTRUCTIONS.md`](./AI_INSTRUCTIONS.md).
