#!/bin/bash
# End-to-end hot-reload smoke test: launch the editor, wait for initial plugin
# load, `touch` the Rust dylib to bump its mtime, verify the timestamp-polling
# reload path fires. Does NOT rebuild the dylib — this only exercises the
# FRustLoader reload *infrastructure* (timestamp detect → unique-name copy →
# codesign → LoadRust → OnRustReloaded broadcast).
#
# For the rebuild-and-see-new-code-take-effect variant, use
# hot_reload_rebuild_test.sh.
#
# Exit 0 on success, non-zero on any failure.

set -uo pipefail

WORKSPACE="$(cd "$(dirname "$0")/.." && pwd)"
UPROJECT="$WORKSPACE/example/RustExample/RustExample.uproject"
EDITOR="/Users/Shared/Epic Games/UE_5.7/Engine/Binaries/Mac/UnrealEditor.app/Contents/MacOS/UnrealEditor"
DYLIB="$WORKSPACE/target/release/libunreal_rust_host.dylib"
MAP="/Game/Gatherers/GatherersBevyMass"
LOG="/tmp/ue_hot_reload_smoke.log"

LOAD_TIMEOUT=120   # seconds to wait for initial plugin load
RELOAD_TIMEOUT=30  # seconds to wait for reload after touch

fail() { echo "FAIL: $*" >&2; exit 1; }

[[ -x "$EDITOR" ]] || fail "UnrealEditor not found at $EDITOR"
[[ -f "$UPROJECT" ]] || fail "uproject not found at $UPROJECT"
[[ -f "$DYLIB" ]] || fail "dylib not found at $DYLIB — run 'cargo build --release -p unreal-rust-host' first"

rm -f "$LOG"

echo "== Launching editor (headless) =="
"$EDITOR" "$UPROJECT" "$MAP" \
  -stdout -FullStdOutLogOutput -nullrhi -nosplash -nosound -unattended \
  >"$LOG" 2>&1 &
EDITOR_PID=$!

cleanup() {
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

# Count pre-touch load events so we can assert the touch caused a *new* one.
INITIAL_LOADS=$(grep -c "\[unreal-rust\] Loaded plugin from" "$LOG")
echo "== Load events before touch: $INITIAL_LOADS =="

# Give the editor another couple of seconds to settle (tick loop steady-state)
# so the touch clearly lands *after* initial load.
sleep 2

echo "== Touching dylib to trigger timestamp reload =="
touch "$DYLIB"
TOUCH_TIME=$(date +%s)

echo "== Waiting for Hotreload log line (up to ${RELOAD_TIMEOUT}s) =="
for i in $(seq 1 "$RELOAD_TIMEOUT"); do
  CURRENT_LOADS=$(grep -c "\[unreal-rust\] Loaded plugin from" "$LOG" 2>/dev/null || echo 0)
  if (( CURRENT_LOADS > INITIAL_LOADS )) && grep -q "LogTemp: Display: Hotreload" "$LOG" 2>/dev/null; then
    echo "  reload detected after ${i}s (load events now: $CURRENT_LOADS)"
    ELAPSED=$(( $(date +%s) - TOUCH_TIME ))
    echo "  reload latency: ~${ELAPSED}s"
    echo "OK: hot-reload infrastructure is healthy"
    exit 0
  fi
  kill -0 "$EDITOR_PID" 2>/dev/null || fail "editor died during reload — see $LOG"
  sleep 1
done

echo "--- tail of log for diagnosis ---"
tail -60 "$LOG"
fail "no reload detected within ${RELOAD_TIMEOUT}s after touch — see $LOG"
