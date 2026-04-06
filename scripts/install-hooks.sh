#!/bin/sh
# Install git hooks for Hyle development.
# Run once: ./scripts/install-hooks.sh
#
# Pre-commit mirrors CI: fmt → clippy → test → doc.
# If pre-commit passes, CI will pass.

HOOK_DIR="$(git rev-parse --git-dir)/hooks"
mkdir -p "$HOOK_DIR"

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
echo "Git hooks installed. Pre-commit mirrors CI pipeline."
