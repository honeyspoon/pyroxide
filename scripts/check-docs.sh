#!/usr/bin/env bash
# Verify documentation is not stale.
# Run from repo root: ./scripts/check-docs.sh
set -e

REPO_ROOT="$(cd "$(dirname "$0")/.." && pwd)"

# Count actual examples
EXAMPLE_COUNT=$(ls "$REPO_ROOT/examples/examples/"*.rs 2>/dev/null | wc -l | tr -d ' ')
echo "Examples found: $EXAMPLE_COUNT"

# Count examples listed in README (lines like "| 01 | ...")
README_EXAMPLE_COUNT=$(grep -cE '^\| [0-9]{2} ' "$REPO_ROOT/README.md" || true)
echo "Examples in README: $README_EXAMPLE_COUNT"

if [ "$EXAMPLE_COUNT" != "$README_EXAMPLE_COUNT" ]; then
    echo "FAIL: README lists $README_EXAMPLE_COUNT examples but $EXAMPLE_COUNT exist"
    exit 1
fi

# Count examples in Cargo.toml
CARGO_EXAMPLE_COUNT=$(grep -c '^\[\[example\]\]' "$REPO_ROOT/examples/Cargo.toml" || true)
echo "Examples in Cargo.toml: $CARGO_EXAMPLE_COUNT"

if [ "$EXAMPLE_COUNT" != "$CARGO_EXAMPLE_COUNT" ]; then
    echo "FAIL: Cargo.toml lists $CARGO_EXAMPLE_COUNT examples but $EXAMPLE_COUNT exist"
    exit 1
fi

# Count examples in Makefile
MAKEFILE_EXAMPLE_COUNT=$(grep -o '[0-9][0-9]_[a-z_]*' "$REPO_ROOT/Makefile" | sort -u | wc -l | tr -d ' ')
echo "Examples in Makefile: $MAKEFILE_EXAMPLE_COUNT"

if [ "$EXAMPLE_COUNT" != "$MAKEFILE_EXAMPLE_COUNT" ]; then
    echo "FAIL: Makefile lists $MAKEFILE_EXAMPLE_COUNT examples but $EXAMPLE_COUNT exist"
    exit 1
fi

echo "Docs are in sync."
