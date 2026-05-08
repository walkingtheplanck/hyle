#!/bin/sh
# Install git hooks for Hyle development.
# Run once: ./scripts/install-hooks.sh
#
# This uses the tracked `.githooks/` directory through `core.hooksPath` so the
# same hooks work from the main checkout and from linked worktrees.

set -e

ROOT="$(git rev-parse --show-toplevel)"
HOOK_DIR="$ROOT/.githooks"

chmod +x "$HOOK_DIR/pre-commit" "$HOOK_DIR/commit-msg"

git config core.hooksPath .githooks
git config commit.template .gitmessage

echo "Git hooks installed:"
echo "  - hooksPath:  .githooks"
echo "  - pre-commit: mirrors CI pipeline (fmt, clippy, test, doc)"
echo "  - commit-msg: enforces conventional commit format"
echo "  - template:   shows format guide on \`git commit\`"
