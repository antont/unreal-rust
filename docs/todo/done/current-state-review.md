# Current state review — UE integration & idiomatic Bevy authoring (2026-04-16)

## What's working well

**5/6 game systems are portable** — they compile identically in Bevy standalone and UE modes with zero `#[cfg]` gates:

| System | Purpose | Portable? |
|--------|---------|-----------|
| `entity_cooldown` | Decrement/remove pickup timer | Yes |
| `entity_boundary_reflect` | Reflect velocity at sim bounds | Yes |
| `ant_food_decision` | Read hit messages, decide pickup/drop | Yes |
| `apply_food_mutations` | Update food state from decisions | Yes |
| `carried_food_tracking` | Food follows carrying ant | Yes |
| `ant_collision_prepass` | Detect food encounters | UE-only (spatial queries) |

**Shared decision logic** — `ant_food_decision` is a pure Rust function with zero `#[cfg]` gates. Both UE and standalone modes call the same function.

**Framework-provided movement** — `MovementPlugin<T, P, D>` applies velocity to position in Bevy mode; in UE mode it's a no-op (C++ `UMassApplyMovementProcessor` handles it). Game systems only write `DesiredMovement` velocity, never positions directly.

**`QueryAll` facade** — index-based global entity access works across both backends:
- UE: zero-copy chunk access via `MassQueryAllMut`
- Bevy: `EntityIndex<Tag>` resource + Query lookup via `QueryAllWrapper`
- Game code writes `foods: QueryAll<&mut FoodFragment, With<FoodTag>>` — identical in both modes.

**Clean infrastructure split** — C++ handles only UE-specific concerns:
- Position application (velocity -> transform via `UMassApplyMovementProcessor`)
- PreviousTranslation save (for spatial sweep queries)
- Visualization pipeline
- Spatial query physics sweeps

## Intentionally engine-specific infrastructure

| Concern | Standalone | UE | Notes |
|---------|-----------|-----|-------|
| Movement (`pos += vel * dt`) | `MovementPlugin` Rust system | C++ `UMassApplyMovementProcessor` | Game code writes `DesiredMovement`, engine applies it |
| Collision detection | Direct Bevy queries (brute-force distance) | C++ physics sweeps via `SpatialQuery` | Game code reads `HitEvent` messages, engine detects collisions |

These are intentionally engine-specific. UE has powerful native implementations for movement application and spatial queries — the framework uses them in UE mode rather than reimplementing in Rust. The game author's interface is identical: write `DesiredMovement` for movement, read `HitEvent` messages for collisions.

`SpatialQuery` (in `bevy_mass`) wraps UE's `MassSpatialQueries` with a cleaner Rust API (`SpatialHit` with `DVec3` instead of raw FFI types). It's also available as a general-purpose resource for custom spatial query needs in UE mode.

## Future improvements

1. **Entity references instead of numeric indices** — Change `Carrying.food_index: i32` to `Option<Entity>`. More Bevy-idiomatic; UE backend would resolve Entity -> chunk index. Requires reworking spatial query result delivery.

## Verdict

The **authoring experience is identical** across backends for game logic. A game developer writing systems uses the same `Query`, `QueryAll`, `Res<Time>`, `Commands`, `MessageWriter`/`MessageReader`, and `DesiredMovement`/`HitEvent` types in both modes. The `#[mass_system]` macro handles all backend-specific rewrites.

Engine infrastructure (movement application, collision detection) is intentionally backend-specific — different implementations, same interface, same output. This mirrors UE's own architecture where game code writes desired intent and the engine handles execution.
