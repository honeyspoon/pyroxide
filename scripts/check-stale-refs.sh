#!/usr/bin/env bash
# Check that documentation doesn't reference types that were removed.
set -euo pipefail

REPO_ROOT="$(cd "$(dirname "$0")/.." && pwd)"
SRC="$REPO_ROOT/pyroxide/src"
ERRORS=0

REMOVED_TYPES=(
    "MojoAddr"
    "MojoArg"
    "MojoResult"
    "MojoError"
    "catch_mojo_result"
    "catch_mojo_call"
    "mojo_import!"
    "MojoRef"
    "MojoMut"
    "OutParam"
    "BorrowedDescriptor"
)

# Check README for removed types (anywhere, not just Added sections)
for type in "${REMOVED_TYPES[@]}"; do
    # Allow in contexts that say "removed" or "renamed"
    hits=$(grep -c "\`$type\`" "$REPO_ROOT/README.md" 2>/dev/null || true)
    allowed=$(grep "\`$type\`" "$REPO_ROOT/README.md" 2>/dev/null | grep -ciE "removed|renamed|deprecated|replaced|old" || true)
    stale=$((hits - allowed))
    if [ "$stale" -gt 0 ]; then
        echo "FAIL: README references removed type '$type' ($stale non-removal references)"
        ERRORS=$((ERRORS + 1))
    fi
done

# Check that public items mentioned in README exist in source
for item in IntoMojo FromMojo MojoSlice MojoSliceMut MojoStr OutSlot catch_panic_at_ffi DescriptorGuard; do
    if ! grep -rq "pub.*$item\|pub fn $item\|pub struct $item\|pub trait $item" "$SRC/"; then
        echo "FAIL: README references '$item' but not pub in source"
        ERRORS=$((ERRORS + 1))
    fi
done

if [ $ERRORS -gt 0 ]; then
    echo ""
    echo "FAIL: $ERRORS stale reference(s) found"
    exit 1
fi

echo "No stale references found."
