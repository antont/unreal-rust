#!/bin/bash
# Open the Unreal Editor with a sim's BevyMass level loaded, ready for PIE testing.
#
# Usage:
#   ./scripts/pie.sh                  # gatherers (default)
#   ./scripts/pie.sh gatherers        # gatherers
#   ./scripts/pie.sh vivarium         # vivarium
#   ./scripts/pie.sh /Game/Other/Map  # raw /Game/... path (uses gatherers host unless
#                                     # UNREAL_RUST_HOST_NAME is already exported)
#
# Selecting "vivarium" sets UNREAL_RUST_HOST_NAME=vivarium_rust_host so the
# loader picks the vivarium dylib. "gatherers" leaves the env var unset so
# the default (unreal_rust_host) is used.

ARG="${1:-gatherers}"

case "$ARG" in
  gatherers)
    MAP="/Game/Gatherers/GatherersBevyMass"
    ;;
  vivarium)
    MAP="/Game/Vivarium/VivariumBevyMass"
    export UNREAL_RUST_HOST_NAME="vivarium_rust_host"
    ;;
  /Game/*)
    MAP="$ARG"
    ;;
  *)
    echo "Unknown sim or map: $ARG" >&2
    echo "Pass 'gatherers', 'vivarium', or a /Game/... path." >&2
    exit 1
    ;;
esac

echo "Opening $MAP (UNREAL_RUST_HOST_NAME=${UNREAL_RUST_HOST_NAME:-<unset: default gatherers>})"

"/Users/Shared/Epic Games/UE_5.7/Engine/Binaries/Mac/UnrealEditor.app/Contents/MacOS/UnrealEditor" \
  "/Users/tonialatalo/src/unreal-rust/example/RustExample/RustExample.uproject" \
  "$MAP" \
  2>&1 | tee /tmp/ue_pie.log
