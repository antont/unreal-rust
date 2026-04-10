#!/bin/bash
# Open the Unreal Editor with the GatherersBevyMass level loaded, ready for PIE testing.
# Usage: ./scripts/pie.sh [map-path]
# Default map: /Game/Gatherers/GatherersBevyMass

MAP="${1:-/Game/Gatherers/GatherersBevyMass}"

"/Users/Shared/Epic Games/UE_5.7/Engine/Binaries/Mac/UnrealEditor.app/Contents/MacOS/UnrealEditor" \
  "/Users/tonialatalo/src/unreal-rust/example/RustExample/RustExample.uproject" \
  "$MAP" \
  2>&1 | tee /tmp/ue_pie.log
