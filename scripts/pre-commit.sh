#!/bin/bash
# Git pre-commit hook: runs the standalone screenshot regression check when
# staged changes touch paths that could affect sim visuals.
#
# Install:
#   ln -sf ../../scripts/pre-commit.sh .git/hooks/pre-commit
#
# Bypass (for unrelated, intentional, or emergency commits):
#   git commit --no-verify

set -euo pipefail

WORKSPACE="$(git rev-parse --show-toplevel)"

# Only run when staged files touch these paths. Anything outside this set
# can't affect the standalone sim's pixel output, so the hook stays a no-op
# and the commit is instant.
SCOPES=(
    "gatherers-standalone/"
    "gatherers-sim/src/"
    "bevy_mass/src/"
    "scripts/verify_standalone.sh"
    "gatherers-standalone/test/reference_screenshots/"
)

staged=$(git diff --cached --name-only --diff-filter=ACMR)
relevant=0
while IFS= read -r path; do
    [[ -z "$path" ]] && continue
    for scope in "${SCOPES[@]}"; do
        if [[ "$path" == "$scope"* || "$path" == "$scope" ]]; then
            relevant=1
            break 2
        fi
    done
done <<< "$staged"

if (( relevant == 0 )); then
    exit 0
fi

echo "== Pre-commit: running standalone screenshot regression check =="
if ! "$WORKSPACE/scripts/verify_standalone.sh"; then
    echo ""
    echo "Pre-commit failed. To bypass intentionally:  git commit --no-verify"
    exit 1
fi
