# Replace custom vector helpers with glam

`gatherers-sim/src/movement.rs` has hand-rolled f64 vector helpers:
`normalize_f64x3`, `is_nearly_zero_f64x3`, `dot_f64x3`, `reflect_direction`, `reverse_direction`.

These duplicate what `glam::DVec3` provides (`.normalize()`, `.dot()`, operator overloads).

## Status: DONE

Fragment fields use `DVec3` directly. The type mapping in `rust_type_to_cpp()` maps `DVec3` → `FVector`, so the generated C++ header is unchanged. Custom vector helpers replaced with glam operations.

## Options considered

### 1. DVec3 as fragment field type (chosen, implemented)

Cleanest API. Fragment fields are `DVec3` instead of `[f64; 3]`.
`DVec3` is `#[repr(C)]` with the same layout as `[f64; 3]` / Unreal `FVector`, so FFI is unaffected.

Required one line in the type mapping: `"DVec3" => "FVector"`. The derive macro's `#[mass(default = "...")]` strings are already C++ literals (e.g. `"FVector(1.0f, 0.0f, 0.0f)"`) and pass through unchanged.

### 2. Keep `[f64; 3]` fields, use DVec3 at call sites (rejected)

`DVec3::from(arr).normalize().to_array()`. No macro changes but adds conversion noise.

### 3. Keep current helpers (rejected)

Zero new deps, 5 small functions. Works today but doesn't move toward Bevy compatibility.

## f64 vs f32

| | f64 (DVec3) | f32 (Vec3) |
|---|---|---|
| **Unreal** | Native — FVector is double since UE5 | Needs conversion at FFI |
| **Bevy rendering** | Needs `.as_vec3()` at render edge | Native with Transform |
| **Sim precision** | Better for large worlds | Fine for most sims |
| **Compile-time switching** | See below | See below |

**Chosen: DVec3 (f64)**. Matches Unreal's FVector natively. The conversion cost at the Bevy render boundary is trivial (`transform.translation = pos.position.as_vec3()`).

## Compile-time f32/f64 switching (rejected for now)

Could use a feature flag:
```rust
#[cfg(feature = "unreal")]
pub type SimVec3 = DVec3;
#[cfg(not(feature = "unreal"))]
pub type SimVec3 = Vec3;
```

Rejected because it leaks complexity into:
- Fragment struct definitions (conditional sizes: 24 vs 48 bytes)
- C++ mirror structs (would need both versions)
- All static_assert layout checks
- The MassFragment derive macro

Not worth the complexity for negligible runtime benefit. f64 is fine for both targets.

## Standalone Bevy compatibility

Using glam types (even DVec3) makes the sim code directly usable in vanilla Bevy — glam is Bevy's native math library. A standalone Bevy app would just need a render sync system:

```rust
fn sync_positions(query: Query<(&Position, &mut Transform)>) {
    for (pos, mut transform) in &query {
        transform.translation = pos.position.as_vec3();
    }
}
```

No adapter layers, no conversion in the sim itself.
