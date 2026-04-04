#!/usr/bin/env bash
# Fetch MAX C API headers from the modular/modular GitHub repository.
#
# Usage: ./scripts/fetch-headers.sh [branch]
#   branch defaults to "main"
#
# Downloads to: max-sys/headers/

set -euo pipefail

BRANCH="${1:-main}"
BASE="https://raw.githubusercontent.com/modular/modular/${BRANCH}"
DEST="$(cd "$(dirname "$0")/.." && pwd)/max-sys/headers"

mkdir -p "$DEST"

HEADERS=(
    "max/include/max/c/types.h"
    "max/include/max/c/common.h"
    "max/include/max/c/device.h"
    "max/include/max/c/symbol_export.h"
    "max/include/max/c/tensor.h"
    "max/include/max/c/context.h"
    "max/include/max/c/model.h"
    "max/include/max/c/weights.h"
)

echo "Fetching MAX C headers from modular/modular@${BRANCH} ..."

for header in "${HEADERS[@]}"; do
    filename=$(basename "$header")
    url="${BASE}/${header}"
    echo "  ${filename} <- ${url}"
    if curl -fsSL "$url" -o "${DEST}/${filename}" 2>/dev/null; then
        echo "    ok"
    else
        echo "    FAILED (may not exist on this branch)"
        rm -f "${DEST}/${filename}"
    fi
done

echo ""
echo "Headers saved to: ${DEST}/"
ls -la "$DEST"/*.h 2>/dev/null || echo "(no headers downloaded)"
