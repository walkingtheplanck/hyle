#!/bin/sh
# Install git hooks for Hyle development.
# Run once: ./scripts/install-hooks.sh
#
# Pre-commit mirrors CI: fmt → clippy → test → doc.
# Commit-msg enforces conventional commit format.
# Commit template shows the format when you run `git commit` without -m.

HOOK_DIR="$(git rev-parse --git-dir)/hooks"
mkdir -p "$HOOK_DIR"

# --- Commit template ---
git config commit.template .gitmessage
echo "Commit template set (.gitmessage)"

# --- Pre-commit hook ---
cat > "$HOOK_DIR/pre-commit" << 'HOOK'
#!/bin/sh
set -e

echo "=== Pre-commit: same checks as CI ==="

echo "[1/4] cargo fmt..."
cargo fmt --all
git diff --name-only | xargs -r git add

echo "[2/4] cargo clippy..."
cargo clippy --workspace --all-targets -- -D warnings

echo "[3/4] cargo test..."
cargo test --workspace --release --quiet

echo "[4/4] cargo doc..."
RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps --quiet

echo "=== All checks passed ==="
HOOK

chmod +x "$HOOK_DIR/pre-commit"

# --- Commit-msg hook ---
cat > "$HOOK_DIR/commit-msg" << 'HOOK'
#!/bin/sh
#
# Enforces conventional commit format on the SUBJECT LINE only.
# Body and footers are free-form.
#
# Format: <type>: <lowercase description>
# Types:  feat, fix, refactor, chore, docs, test, perf, ci

# Extract first non-comment, non-empty line (the subject)
SUBJECT=$(grep -v '^#' "$1" | grep -v '^$' | head -1)

# Allow merge commits
if echo "$SUBJECT" | grep -qE '^Merge '; then
    exit 0
fi

PATTERN='^(feat|fix|refactor|chore|docs|test|perf|ci): [a-z].*[^.]$'

if ! echo "$SUBJECT" | grep -qE "$PATTERN"; then
    echo ""
    echo "ERROR: commit subject does not follow conventional format."
    echo ""
    echo "  Expected: <type>: <lowercase description>"
    echo "  Got:      $SUBJECT"
    echo ""
    echo "  Types: feat, fix, refactor, chore, docs, test, perf, ci"
    echo "  Description must start lowercase, no trailing period."
    echo ""
    exit 1
fi
HOOK

chmod +x "$HOOK_DIR/commit-msg"

echo "Git hooks installed:"
echo "  - pre-commit: mirrors CI pipeline (fmt, clippy, test, doc)"
echo "  - commit-msg: enforces conventional commit format"
echo "  - template:   shows format guide on \`git commit\`"
