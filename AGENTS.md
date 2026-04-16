# AGENTS.md

## Project overview

Fork of [MaikKlein/unreal-rust](https://github.com/MaikKlein/unreal-rust) adding zero-copy Mass Entity integration for UE 5.7 on macOS. Game developers write simulation logic, entity setup, and tests entirely in Rust — no C++ per-game code.

Example: ant colony simulation (gatherers) with ~5000 entities across two crates: `gatherers-sim` (pure logic) and `gatherers-bevy-mass` (UE integration, fragments, tests).

## Architecture

**C++ infrastructure** (game-agnostic, rarely changed) + **Rust game code** (per-game).

Key Rust crates: `gatherers-sim`, `gatherers-bevy-mass` (game), `gatherers-standalone` (standalone Bevy app), `unreal-api` (safe API), `unreal-ffi` (FFI types, source of truth), `unreal-module` (FFI impls), `bevy_mass` (Bevy/UE facade), `unreal-api-derive` (proc macros).

Key patterns: `inventory::submit!` for registration, `#[mass_system]` macro (handles `Query`, `QueryAll`, `Res`, `With`/`Without`, `Commands`, messages), `#[derive(MassFragment)]` with auto-generated C++ headers, compile-time `static_assert` / layout tests for FFI safety.

`bevy_mass` facade provides: `Query` (iteration), `QueryAll` (index-based global access), `MovementPlugin` (pos += vel * dt in Bevy, no-op in UE), `EntityIndex<Tag>` (spawn-order entity lookup), `mass_fragment!`/`mass_tag!` macros. 5/6 game systems are portable across both backends with zero cfg gates.

## TDD workflow

**Always run tests after code changes.** See `docs/testing.md` for exact commands and flags.

- **Rust tests**: `cargo test` (or `-p <crate>`). Fast, no UE needed.
- **UE automation tests**: Rebuild dylib (`cargo build --release -p unreal-rust-host`), then run via CLI. See `docs/testing.md`.
- **C++ build**: Only when C++ files change. See `docs/testing.md`.

Prefer **Rust-authored UE tests** for game logic — write in `gatherers-bevy-mass/src/ue_tests.rs` using `inventory::submit!(MassTestRegistration { ... })`. No C++ needed. See existing tests for the pattern.

Write assertions with **specific magnitude checks**, not just "did it change".

## Goals

- **Zero C++ game authoring** — game logic, entity setup, and tests all in Rust.
- **Test-driven** — write the test first. Both Rust unit tests and UE integration tests.
- **Compile-time FFI safety** — layout mismatches caught at compile time, not runtime.
- **Hot-reload** — `cargo build --release`, changes appear in PIE without editor restart.

## Pitfalls

- UE loads only **release** builds. `cargo build` (debug) won't be picked up.
- Use `LogTemp, Display` (not `Warning`) for informational C++ logging — warnings show in the test GUI.
- `unreal-api-generator` has a pre-existing build error. Run specific crate tests, not bare `cargo test`.
- Fragment changes require syncing both Rust (`unreal-ffi`) and C++ (`Bindings.h`) with matching layout checks.
