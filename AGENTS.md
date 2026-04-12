# AGENTS.md

## Project overview

Fork of [MaikKlein/unreal-rust](https://github.com/MaikKlein/unreal-rust) adding zero-copy Mass Entity integration for Unreal Engine 5.7 on macOS. Game developers write simulation logic, entity setup, and integration tests entirely in Rust — no C++ per-game code.

The example game is an ant colony simulation (gatherers) with ~5000 entities: movement, boundary reflection, food encounter detection via physics sweeps, and pickup/drop behavior.

## Architecture

Two layers: **C++ infrastructure** (one-time, game-agnostic) and **Rust game code** (per-game).

### Rust crates

| Crate | Purpose |
|-------|---------|
| `gatherers-sim` | Pure game logic (movement, cooldown, boundary reflect). No UE dependency. |
| `gatherers-bevy-mass` | UE integration: fragment definitions, entity init, spatial queries, visualizer config, UE tests. |
| `unreal-api` | Safe Rust API over FFI: `MassFragment` trait, `TestCtx`, system registration. |
| `unreal-ffi` | `#[repr(C)]` FFI types shared between Rust and C++. Source of truth for struct layouts. |
| `unreal-module` | FFI function implementations wired into `RustBindings`. |
| `bevy_mass` | Facade crate — Bevy-style API that compiles against either pure Bevy or UE MassEntity. |

### C++ side

| Directory | Purpose |
|-----------|---------|
| `RustPlugin/Source/RustPlugin/` | Core plugin: subsystem, processor bridge, bindings, visualizer. |
| `RustPlugin/Source/RustPluginTests/` | UE automation tests (both C++ and Rust-authored runner). |

### Key patterns

- **`inventory::submit!`** — Rust registers sim defaults, spatial queries, visualizer groups, and tests at link time. C++ discovers them via count/descriptor FFI functions.
- **`#[mass_system]`** — Macro turns a Rust function into a MassEntity processor with auto-generated FFI.
- **`#[derive(MassFragment)]`** — Generates C++ header and compile-time layout verification.
- **Compile-time FFI verification** — `static_assert` in C++ and layout tests in Rust ensure struct sizes/alignments match across the boundary.

## TDD workflow

**Always run tests after code changes.** This is non-negotiable.

### Test layers

1. **Rust unit tests** — Pure logic, fast, no UE dependency.
   ```bash
   cargo test
   # or specific crate:
   cargo test -p gatherers-sim
   cargo test -p unreal-ffi
   ```

2. **UE automation tests** — Integration tests running inside the UE editor with full Mass Entity + physics.
   ```bash
   # Rebuild release dylib first (UE only loads release builds):
   cargo build --release -p unreal-rust-host

   # Run all RustPlugin tests:
   "/Users/Shared/Epic Games/UE_5.7/Engine/Binaries/Mac/UnrealEditor" \
     "/Users/tonialatalo/src/unreal-rust/example/RustExample/RustExample.uproject" \
     -ExecCmds="Automation RunTests supplemental.RustPlugin;Quit" \
     -stdout -FullStdOutLogOutput
   ```

   **Do NOT use** `-NullRHI`, `-Unattended`, or `-game` flags — they break RustPlugin module loading.

   Check results (shell exit code is unreliable):
   ```bash
   grep "TEST COMPLETE. EXIT CODE" "/Users/tonialatalo/Library/Logs/Unreal Engine/RustExampleEditor/RustExample.log"
   ```
   `EXIT CODE: 0` = all passed.

3. **C++ build** (only needed when C++ files change):
   ```bash
   "/Users/Shared/Epic Games/UE_5.7/Engine/Build/BatchFiles/Mac/Build.sh" \
     RustExampleEditor Mac Development \
     "/Users/tonialatalo/src/unreal-rust/example/RustExample/RustExample.uproject"
   ```

### Writing new tests

**Rust-authored UE tests** (preferred for game logic):
```rust
use unreal_api::mass::{MassTestRegistration, TestCtx};
use crate::fragments::Position;

inventory::submit!(MassTestRegistration {
    name: "MyTest",
    test_fn: my_test,
});

fn my_test(ctx: &TestCtx) {
    ctx.init_sim(&[("ants", 5)], [-500.0,-500.0,0.0], [500.0,500.0,100.0], 42);
    assert_eq!(ctx.entity_count("ants"), 5);
    ctx.step(0.016, 10);
    let pos = ctx.read::<Position>("ants", 0).unwrap();
    assert!(pos.position[0].abs() > 0.1, "ant should have moved: {:?}", pos.position);
    ctx.reset();
}
```

No C++ needed. Rebuild the dylib, run UE automation tests, it appears as `supplemental.RustPlugin.RustTests.MyTest`.

**Write assertions with specific magnitude checks**, not just "did it change". Bad: `assert!(pos != initial)`. Good: `assert!(distance > 1.0, "expected ~32 units: actual={}", distance)`.

### When to run what

| Changed | Run |
|---------|-----|
| Rust game logic (`gatherers-sim`, `gatherers-bevy-mass`) | `cargo test` + rebuild dylib + UE automation tests |
| FFI types (`unreal-ffi`) | `cargo test -p unreal-ffi` + rebuild dylib + UE automation tests |
| C++ files (`RustPlugin/`) | C++ build + UE automation tests |
| Only Rust tests (no UE behavior) | `cargo test` is sufficient |

## Goals

- **Zero C++ game authoring** — Game developers never touch C++ for game logic, entity setup, or tests.
- **Test-driven development** — Write the test first, then the implementation. Both Rust unit tests and UE integration tests.
- **Compile-time safety** — FFI layout mismatches caught at compile time, not runtime.
- **Hot-reload** — Change Rust code, `cargo build --release`, see it live in PIE without restarting the editor.

## Common pitfalls

- UE loads only **release** builds of the Rust dylib. `cargo build` (debug) won't be picked up.
- `LogTemp, Warning` in C++ shows as test warnings in the UE test GUI. Use `LogTemp, Display` for informational logging.
- The `unreal-api-generator` crate has a pre-existing build error (`path_add_extension` unstable). Don't try to fix it — run specific crate tests instead of bare `cargo test`.
- Fragment struct changes require updating both Rust (`unreal-ffi`) and C++ (`Bindings.h`) with matching `static_assert` / layout tests.
