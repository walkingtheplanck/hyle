#!/bin/sh
#
# Shared CI checks — called by both .githooks/pre-commit and .github/workflows/ci.yml.
#
# Usage: scripts/ci-check.sh <check>
#   checks: fmt, clippy, test, doc, all
#
# The viewer is excluded (needs GPU/display libs unavailable in CI).

set -e

PACKAGES="-p hyle-ca-analysis -p hyle-ca-contracts -p hyle-ca-semantics -p hyle-ca-solver"

run_fmt() {
    echo "[fmt] cargo fmt..."
    cargo fmt --all --check
}

run_clippy() {
    echo "[clippy] cargo clippy..."
    cargo clippy $PACKAGES --all-targets -- -D warnings
}

run_test() {
    echo "[test] cargo test (debug)..."
    cargo test $PACKAGES --quiet
    echo "[test] cargo test (release)..."
    cargo test $PACKAGES --release --quiet
}

run_doc() {
    echo "[doc] cargo doc..."
    RUSTDOCFLAGS="-D warnings" cargo doc --no-deps --quiet
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
