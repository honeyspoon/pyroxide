#!/usr/bin/env bash
# Check that documentation doesn't reference types/functions that were removed.
# Catches: CHANGELOG listing removed types, ADRs referencing dead code, README stale.
set -euo pipefail

REPO_ROOT="$(cd "$(dirname "$0")/.." && pwd)"
SRC="$REPO_ROOT/pyroxide/src"
ERRORS=0

# Types that were removed and should NOT appear in docs as "shipped" or "available"
# (they can appear in "removed" or "tried and removed" context)
REMOVED_TYPES=(
    "MojoAddr"
    "MojoArg"
    "MojoResult"
    "MojoError"
    "catch_mojo_result"
    "mojo_import!"
)

# Extract only "### Added" sections from CHANGELOG, check for removed types
added_lines=$(awk '/^### Added/{flag=1; next} /^### /{flag=0} flag' "$REPO_ROOT/CHANGELOG.md" 2>/dev/null || true)
for type in "${REMOVED_TYPES[@]}"; do
    if echo "$added_lines" | grep -q "\`$type\`"; then
        echo "FAIL: CHANGELOG lists removed type '$type' under Added"
        ERRORS=$((ERRORS + 1))
    fi
done

# Check that public items mentioned in README actually exist in source
for item in IntoMojo FromMojo MojoRef MojoMut MojoSlice MojoSliceMut MojoStr OutParam catch_panic_at_ffi; do
    if ! grep -rq "pub.*$item\|pub fn $item\|pub struct $item\|pub trait $item" "$SRC/"; then
        echo "FAIL: README references '$item' but it's not pub in source"
        ERRORS=$((ERRORS + 1))
    fi
done

# Check that types in the prelude actually exist
prelude_items=$(grep "pub use" "$SRC/lib.rs" | grep -oE '[A-Za-z_]+' | grep -v pub | grep -v use | grep -v crate | grep -v feature | grep -v max | grep -v cfg)
for item in $prelude_items; do
    if [ ${#item} -gt 3 ] && ! grep -rq "pub.*$item" "$SRC/" 2>/dev/null; then
        echo "WARN: prelude exports '$item' but can't find it in source"
    fi
done

if [ $ERRORS -gt 0 ]; then
    echo ""
    echo "FAIL: $ERRORS stale reference(s) found"
    exit 1
fi

echo "No stale references found."
