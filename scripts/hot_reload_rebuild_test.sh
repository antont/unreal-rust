#!/bin/bash
# End-to-end hot-reload rebuild test: launch the editor, wait for initial
# plugin load, inject a marker log line into a game-crate source file, rebuild
# the release dylib with cargo, verify the editor picked up the new code and
# the marker fires in the editor log. Proves that *a real rebuild produces
# new behavior mid-editor-session*, not just that the reload infrastructure
# flips a timestamp.
#
# The source mutation is guaranteed-reverted on exit via trap. The marker
# lives in gatherers-bevy-mass/src/systems.rs and the line it's injected into
# is tick-frequency, so we don't have to wait more than a few editor frames
# after reload to see it.
#
# Exit 0 on success, non-zero on any failure.

set -uo pipefail

WORKSPACE="$(cd "$(dirname "$0")/.." && pwd)"
UPROJECT="$WORKSPACE/example/RustExample/RustExample.uproject"
EDITOR="/Users/Shared/Epic Games/UE_5.7/Engine/Binaries/Mac/UnrealEditor.app/Contents/MacOS/UnrealEditor"
DYLIB="$WORKSPACE/target/release/libunreal_rust_host.dylib"
SRC="$WORKSPACE/unreal-module/src/lib.rs"
MAP="/Game/Gatherers/GatherersBevyMass"
LOG="/tmp/ue_hot_reload_rebuild.log"

# We need a code path that runs *on every dylib load*, not just during PIE
# tick. `initialize_module` in unreal-module/src/lib.rs is called from
# `register_unreal_bindings` on every (re)load, so injecting a marker there
# guarantees it fires synchronously with the reload — no PIE required.
#
# The PID+timestamp suffix prevents a stale log from a previous run from
# spoofing success.
MARKER="HOT_RELOAD_MARKER_$$_$(date +%s)"
ANCHOR='pub unsafe fn initialize_module('

LOAD_TIMEOUT=120
REBUILD_TIMEOUT=300   # cargo release builds can take a while
RELOAD_TIMEOUT=30
MARKER_TIMEOUT=20     # marker log appears on next tick after reload

fail() { echo "FAIL: $*" >&2; exit 1; }

[[ -x "$EDITOR" ]] || fail "UnrealEditor not found at $EDITOR"
[[ -f "$UPROJECT" ]] || fail "uproject not found at $UPROJECT"
[[ -f "$DYLIB" ]] || fail "dylib not found at $DYLIB — run 'cargo build --release -p unreal-rust-host' first"
[[ -f "$SRC" ]] || fail "source file not found at $SRC"
grep -q "$ANCHOR" "$SRC" || fail "anchor '$ANCHOR' not found in $SRC (source drift — update this script)"

rm -f "$LOG"

echo "== Launching editor (headless) =="
"$EDITOR" "$UPROJECT" "$MAP" \
  -stdout -FullStdOutLogOutput -nullrhi -nosplash -nosound -unattended \
  >"$LOG" 2>&1 &
EDITOR_PID=$!

REVERT_NEEDED=0

cleanup() {
  # Always revert the source edit first, whether or not we succeeded.
  if (( REVERT_NEEDED == 1 )) && [[ -f "${SRC}.hotreload_test.bak" ]]; then
    echo "== Reverting source edit =="
    mv "${SRC}.hotreload_test.bak" "$SRC"
  fi
  if kill -0 "$EDITOR_PID" 2>/dev/null; then
    echo "== Shutting down editor (pid $EDITOR_PID) =="
    kill "$EDITOR_PID" 2>/dev/null || true
    for _ in $(seq 1 20); do
      kill -0 "$EDITOR_PID" 2>/dev/null || break
      sleep 0.5
    done
    kill -9 "$EDITOR_PID" 2>/dev/null || true
  fi
}
trap cleanup EXIT

echo "== Waiting for initial plugin load (up to ${LOAD_TIMEOUT}s) =="
for i in $(seq 1 "$LOAD_TIMEOUT"); do
  if grep -q "\[unreal-rust\] Loaded plugin from" "$LOG" 2>/dev/null; then
    echo "  initial load detected after ${i}s"
    break
  fi
  kill -0 "$EDITOR_PID" 2>/dev/null || fail "editor died before initial load — see $LOG"
  sleep 1
done
grep -q "\[unreal-rust\] Loaded plugin from" "$LOG" || fail "editor did not log initial plugin load within ${LOAD_TIMEOUT}s — see $LOG"

INITIAL_LOADS=$(grep -c "\[unreal-rust\] Loaded plugin from" "$LOG")
echo "== Load events before rebuild: $INITIAL_LOADS =="

# Start PIE so apply_food_mutations actually runs. Without PIE the sim doesn't tick.
# The editor's command line -ExecCmds doesn't include a PIE toggle, so we instead
# rely on the map's default — GatherersBevyMass auto-runs sim in editor world.
# If that changes, we'll see the marker never fire and report accordingly.

echo "== Injecting marker log into $SRC =="
cp "$SRC" "${SRC}.hotreload_test.bak"
REVERT_NEEDED=1
# Inject `log::info!("MARKER");` right after the opening brace of
# apply_food_mutations. Python handles multi-line signatures correctly where
# a line-oriented awk/sed would need a multi-line pattern dance.
python3 - "$SRC" "$ANCHOR" "$MARKER" <<'PY'
import sys
src_path, anchor, marker = sys.argv[1], sys.argv[2], sys.argv[3]
with open(src_path) as f:
    text = f.read()
idx = text.find(anchor)
if idx < 0:
    sys.exit(f"anchor not found: {anchor!r}")
brace = text.find("{", idx)
if brace < 0:
    sys.exit("opening brace of apply_food_mutations not found")
insertion = f'\n    eprintln!("{marker}");'
text = text[:brace+1] + insertion + text[brace+1:]
with open(src_path, "w") as f:
    f.write(text)
PY

grep -q "$MARKER" "$SRC" || fail "could not inject marker into $SRC"
echo "  marker '$MARKER' injected"

echo "== Rebuilding release dylib (up to ${REBUILD_TIMEOUT}s) =="
REBUILD_START=$(date +%s)
( cd "$WORKSPACE" && cargo build --release -p unreal-rust-host ) || fail "cargo build failed"
REBUILD_ELAPSED=$(( $(date +%s) - REBUILD_START ))
echo "  rebuild completed in ${REBUILD_ELAPSED}s"

echo "== Waiting for reload (up to ${RELOAD_TIMEOUT}s) =="
for i in $(seq 1 "$RELOAD_TIMEOUT"); do
  CURRENT_LOADS=$(grep -c "\[unreal-rust\] Loaded plugin from" "$LOG" 2>/dev/null || echo 0)
  if (( CURRENT_LOADS > INITIAL_LOADS )) && grep -q "LogTemp: Display: Hotreload" "$LOG" 2>/dev/null; then
    echo "  reload detected after ${i}s (load events now: $CURRENT_LOADS)"
    break
  fi
  kill -0 "$EDITOR_PID" 2>/dev/null || fail "editor died during reload — see $LOG"
  sleep 1
done

NEW_LOADS=$(grep -c "\[unreal-rust\] Loaded plugin from" "$LOG" 2>/dev/null || echo 0)
(( NEW_LOADS > INITIAL_LOADS )) || fail "no reload detected within ${RELOAD_TIMEOUT}s after rebuild — see $LOG"
grep -q "LogTemp: Display: Hotreload" "$LOG" || fail "reload loaded the new dylib but 'Hotreload' broadcast line is missing — see $LOG"

echo "== Waiting for marker '$MARKER' in log (up to ${MARKER_TIMEOUT}s) =="
for i in $(seq 1 "$MARKER_TIMEOUT"); do
  if grep -q "$MARKER" "$LOG" 2>/dev/null; then
    echo "  marker appeared after ${i}s"
    echo "OK: rebuild-triggered hot-reload executed new code"
    exit 0
  fi
  kill -0 "$EDITOR_PID" 2>/dev/null || fail "editor died while waiting for marker — see $LOG"
  sleep 1
done

echo "--- tail of log for diagnosis ---"
tail -80 "$LOG"
echo "--- marker grep ---"
grep "HOT_RELOAD_MARKER" "$LOG" || true
fail "marker '$MARKER' did not appear in log within ${MARKER_TIMEOUT}s of reload — editor may not be ticking apply_food_mutations (no PIE / no sim init)"
