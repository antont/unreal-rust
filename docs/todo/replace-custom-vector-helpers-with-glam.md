# Replace custom vector helpers with glam

`gatherers-sim/src/movement.rs` has hand-rolled f64 vector helpers:
`normalize_f64x3`, `is_nearly_zero_f64x3`, `dot_f64x3`, `reflect_direction`, `reverse_direction`.

These duplicate what `glam::DVec3` provides (`.normalize()`, `.dot()`, operator overloads).

## Options

1. **Add `glam` dep, use `DVec3` as fragment field type** — cleanest API but requires `MassFragment` derive macro to handle `DVec3` for C++ default generation. `DVec3` is `#[repr(C)]` so layout matches `[f64; 3]` / `FVector`.

2. **Add `glam` dep, keep `[f64; 3]` fields, convert at use site** — `DVec3::from(arr).normalize().to_array()`. No macro changes but verbose.

3. **Keep current helpers** — zero new deps, 5 small functions. Works today.

## Context

`glam` is not currently in the dependency tree — `bevy_ecs` doesn't pull it in (that's `bevy_math`). Adding it is trivial but adds a dep just for vector math.

Option 1 is the long-term goal if we want idiomatic Bevy code. The blocker is the `MassFragment` derive macro needing to understand `DVec3` for `#[mass(default = "...")]` generation.
