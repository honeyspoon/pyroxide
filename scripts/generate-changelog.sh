#!/usr/bin/env bash
# Generate CHANGELOG.md from merged PRs.
# Run: ./scripts/generate-changelog.sh > CHANGELOG.md
set -euo pipefail

echo "# Changelog"
echo ""
echo "Auto-generated from merged pull requests."
echo "Do not edit manually — run \`./scripts/generate-changelog.sh > CHANGELOG.md\`."
echo ""

# Get all tags sorted by version
tags=$(git tag --sort=-version:refname 2>/dev/null || true)

if [ -z "$tags" ]; then
    # No tags — show all PRs as [Unreleased]
    echo "## [Unreleased]"
    echo ""
    gh pr list --state merged --json number,title,mergedAt \
        --jq 'sort_by(.mergedAt) | reverse | .[] | "- \(.title) (#\(.number))"' 2>/dev/null || \
        git log --oneline --format="- %s" HEAD
else
    # Show unreleased (HEAD..latest tag)
    latest=$(echo "$tags" | head -1)
    echo "## [Unreleased]"
    echo ""
    gh pr list --state merged --json number,title,mergedAt \
        --jq "sort_by(.mergedAt) | reverse | .[] | \"- \(.title) (#\(.number))\"" 2>/dev/null | head -20
    echo ""

    # Show each tagged version
    prev=""
    for tag in $tags; do
        date=$(git log -1 --format=%ai "$tag" | cut -d' ' -f1)
        echo "## [$tag] - $date"
        echo ""
        if [ -n "$prev" ]; then
            git log --oneline --format="- %s" "$tag..$prev" | grep -v "^$"
        else
            git log --oneline --format="- %s" "$tag" -10
        fi
        echo ""
        prev=$tag
    done
fi
