# Standalone Bevy runner

Run gatherers sim (or any sim using this framework) with rudimentary visualization in pure Bevy, without Unreal.

## Motivation

- Faster iteration for sim logic (no UE editor startup)
- Demonstrates Bevy compatibility of the facade architecture
- Useful for CI testing of sim behavior

## What exists

- `gatherers-sim` compiles against pure Bevy (no `unreal` feature)
- All fragment types use `glam::DVec3` — Bevy's native math library
- Unit tests already run systems in a real Bevy `World`
- `DeltaTime` resource works in both modes via `bevy_mass` facade

## What's needed

- A `gatherers-bevy-standalone` binary crate (or example) that:
  1. Creates a Bevy `App` with a window
  2. Spawns entities using the same fragment types
  3. Adds the sim systems to a schedule
  4. Renders entity positions (simple 2D circles or sprites)
- Position sync: `transform.translation = pos.position.as_vec3()` (DVec3 -> Vec3 at render edge)
- No UE dependency — pure `bevy` + `gatherers-sim`
