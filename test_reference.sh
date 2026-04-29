#!/bin/bash
# test_reference.sh — Run tests against reference implementations.
#
# Usage:
#   ./test_reference.sh           # run all days
#   ./test_reference.sh day05     # run specific day
#   ./test_reference.sh -- --nocapture  # pass extra flags to cargo
#
# This script temporarily replaces stub implementations with reference
# solutions, runs cargo test, then restores the stubs.

set -euo pipefail

BACKUP_DIR=".stub_backup"
mkdir -p "$BACKUP_DIR"

cp src/day*.rs "$BACKUP_DIR/"

cleanup() {
    cp "$BACKUP_DIR"/day*.rs src/ 2>/dev/null || true
    rm -rf "$BACKUP_DIR"
}
trap cleanup EXIT

echo "==> Assembling reference solutions..."

for i in $(seq -w 1 17); do
    ref_file="reference/day${i}.rs"
    backup_file="$BACKUP_DIR/day${i}.rs"
    src_file="src/day${i}.rs"

    if [ ! -f "$ref_file" ]; then
        continue
    fi

    test_line=$(grep -n "#\[cfg(test)\]" "$backup_file" | head -1 | cut -d: -f1)

    if [ -n "$test_line" ]; then
        {
            cat "$ref_file"
            echo ""
            tail -n "+${test_line}" "$backup_file"
        } > "$src_file"
    fi
done

echo "==> Running cargo test $*"
cargo test "$@"
