#!/bin/bash
# Regenerate the committed screenshot references for gatherers-standalone.
#
# Runs the same deterministic capture as verify_standalone.sh, then (after
# confirmation) copies the PNGs into gatherers-standalone/test/reference_screenshots/.
#
# Use `-y` to skip the confirmation prompt (CI / scripted regen).

set -euo pipefail

ASSUME_YES=0
for arg in "$@"; do
    case "$arg" in
        -y|--yes) ASSUME_YES=1 ;;
        *) echo "Usage: $0 [-y]"; exit 2 ;;
    esac
done

WORKSPACE="$(cd "$(dirname "$0")/.." && pwd)"
OUT_DIR="/tmp/standalone_regression"
REF_DIR="$WORKSPACE/gatherers-standalone/test/reference_screenshots"
FRAMES=(30 180 600)

mkdir -p "$REF_DIR" "$OUT_DIR"
find "$OUT_DIR" -maxdepth 1 -name '*.png' -delete

echo "== Building gatherers-standalone (release) =="
( cd "$WORKSPACE" && cargo build --release -p gatherers-standalone )

TARGET_DIR="$(cd "$WORKSPACE" && cargo metadata --format-version=1 --no-deps \
    | python3 -c 'import json,sys; print(json.load(sys.stdin)["target_directory"])')"
BIN="$TARGET_DIR/release/gatherers-standalone"

FRAMES_CSV=$(IFS=,; echo "${FRAMES[*]}")
LAST_FRAME="${FRAMES[$((${#FRAMES[@]} - 1))]}"

echo "== Capturing reference frames $FRAMES_CSV =="
"$BIN" \
    --deterministic-clock \
    --screenshot-at "$FRAMES_CSV" \
    --exit-after "$LAST_FRAME" \
    --out-dir "$OUT_DIR"

echo ""
echo "Captured screenshots:"
for f in "${FRAMES[@]}"; do
    printf -v frame_pad "%04d" "$f"
    actual="$OUT_DIR/standalone_frame_${frame_pad}.png"
    if [[ ! -f "$actual" ]]; then
        echo "  FAIL: missing $actual"
        exit 1
    fi
    echo "  $actual"
done

if (( ASSUME_YES == 0 )); then
    echo ""
    read -r -p "Overwrite references in $REF_DIR? [y/N] " reply
    case "$reply" in
        [yY]|[yY][eE][sS]) ;;
        *) echo "Aborted."; exit 1 ;;
    esac
fi

for f in "${FRAMES[@]}"; do
    printf -v frame_pad "%04d" "$f"
    src="$OUT_DIR/standalone_frame_${frame_pad}.png"
    dst="$REF_DIR/frame_${frame_pad}_reference.png"
    cp "$src" "$dst"
    echo "  updated $dst"
done

echo ""
echo "References updated. Stage them with:"
echo "  git add gatherers-standalone/test/reference_screenshots/"
