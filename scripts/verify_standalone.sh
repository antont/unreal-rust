#!/bin/bash
# Screenshot-regression check for gatherers-standalone.
#
# Rebuilds the standalone release binary, runs it with a deterministic clock
# and fixed seed, captures 3 frames to /tmp/standalone_regression/, and diffs
# each against its committed reference with odiff. Fails if any frame differs
# by more than TOLERANCE percent (default 0.5).
#
# Override tolerance:  TOLERANCE=1.0 scripts/verify_standalone.sh

set -euo pipefail

WORKSPACE="$(cd "$(dirname "$0")/.." && pwd)"
OUT_DIR="/tmp/standalone_regression"
REF_DIR="$WORKSPACE/gatherers-standalone/test/reference_screenshots"
TOLERANCE="${TOLERANCE:-0.5}"
FRAMES=(30 180 600)

mkdir -p "$OUT_DIR"
find "$OUT_DIR" -maxdepth 1 -name '*.png' -delete

echo "== Building gatherers-standalone (release) =="
( cd "$WORKSPACE" && cargo build --release -p gatherers-standalone )

# Honour a workspace- or user-level cargo target-dir override (this repo
# shares its target dir with other projects via ~/.cargo/config.toml).
TARGET_DIR="$(cd "$WORKSPACE" && cargo metadata --format-version=1 --no-deps \
    | python3 -c 'import json,sys; print(json.load(sys.stdin)["target_directory"])')"
BIN="$TARGET_DIR/release/gatherers-standalone"

FRAMES_CSV=$(IFS=,; echo "${FRAMES[*]}")
LAST_FRAME="${FRAMES[$((${#FRAMES[@]} - 1))]}"

echo "== Running sim, capturing frames $FRAMES_CSV =="
"$BIN" \
    --deterministic-clock \
    --screenshot-at "$FRAMES_CSV" \
    --exit-after "$LAST_FRAME" \
    --out-dir "$OUT_DIR"

FAILED=0
for f in "${FRAMES[@]}"; do
    printf -v frame_pad "%04d" "$f"
    actual="$OUT_DIR/standalone_frame_${frame_pad}.png"
    expected="$REF_DIR/frame_${frame_pad}_reference.png"
    diff_out="$OUT_DIR/frame_${frame_pad}_diff.png"

    if [[ ! -f "$actual" ]]; then
        echo "FAIL: missing actual screenshot for frame $f at $actual"
        FAILED=1
        continue
    fi
    if [[ ! -f "$expected" ]]; then
        echo "FAIL: missing reference for frame $f at $expected"
        echo "  Create references with: scripts/update_standalone_references.sh"
        FAILED=1
        continue
    fi

    # odiff --parsable-stdout: prints "0" when images match (exit 0); prints
    # "PIXELS;PERCENT" when they differ (exit 22).
    result=$(odiff --parsable-stdout --threshold=0.1 \
        "$expected" "$actual" "$diff_out" 2>&1 || true)
    last_line=$(echo "$result" | tail -1)

    if [[ "$last_line" == "0" ]]; then
        echo "OK:   frame $f pixel-identical"
        continue
    fi

    percent=$(echo "$last_line" | awk -F';' '{print $2}')
    if [[ -z "$percent" ]]; then
        echo "FAIL: frame $f — could not parse odiff output: $result"
        FAILED=1
        continue
    fi

    fail=$(awk -v p="$percent" -v t="$TOLERANCE" 'BEGIN{print (p+0 > t+0) ? 1 : 0}')
    if [[ "$fail" == "1" ]]; then
        echo "FAIL: frame $f differs by ${percent}% (> ${TOLERANCE}%)"
        echo "  diff image: $diff_out"
        FAILED=1
    else
        echo "OK:   frame $f differs by ${percent}% (<= ${TOLERANCE}%)"
    fi
done

if (( FAILED == 1 )); then
    echo ""
    echo "Standalone regression check FAILED."
    echo "  Inspect diffs in $OUT_DIR"
    echo "  If the change is intentional, update references:"
    echo "    scripts/update_standalone_references.sh"
    exit 1
fi
echo ""
echo "Standalone regression check passed."
