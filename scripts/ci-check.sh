#!/bin/sh
#
# Shared CI checks for the scaffold workspace.
#
# Usage: scripts/ci-check.sh <check>
#   checks: fmt, clippy, test, doc, all

set -e

run_fmt() {
    echo "[fmt] cargo fmt..."
    cargo fmt --all --check
}

run_clippy() {
    echo "[clippy] cargo clippy..."
    cargo clippy --workspace --all-targets -- -D warnings
}

run_test() {
    echo "[test] cargo test (debug)..."
    cargo test --workspace --quiet
    echo "[test] cargo test (release)..."
    cargo test --workspace --release --quiet
}

run_doc() {
    echo "[doc] cargo doc..."
    CARGO_TARGET_DIR="${CARGO_TARGET_DIR:-target-docci}" \
    RUSTDOCFLAGS="-D warnings" \
    cargo doc --workspace --no-deps --quiet
}

case "${1:-all}" in
    fmt)     run_fmt ;;
    clippy)  run_clippy ;;
    test)    run_test ;;
    doc)     run_doc ;;
    all)
        run_fmt
        run_clippy
        run_test
        run_doc
        ;;
    *)
        echo "Unknown check: $1"
        echo "Usage: scripts/ci-check.sh {fmt|clippy|test|doc|all}"
        exit 1
        ;;
esac
