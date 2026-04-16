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

## Remaining asymmetries

| Concern | Standalone | UE | Notes |
|---------|-----------|-----|-------|
| Movement (`pos += vel * dt`) | `MovementPlugin` Rust system | C++ processor | Transparent — game code identical |
| Collision detection | Brute-force distance check | UE spatial query (C++) | Different infrastructure, same message output |
| `ant_collision_prepass` | Standalone-specific impl | UE-specific impl (facade Query + `Res<MassSpatialQueries>`) | Only system that differs |

## Future improvements

1. **Spatial query facade** (Phase 3) — Add a `SpatialQuery` trait to `bevy_mass` with Bevy-mode brute-force search and UE-mode physics sweep. Would make `ant_collision_prepass` portable (6/6).

2. **Entity references instead of numeric indices** — Change `Carrying.food_index: i32` to `Option<Entity>`. More Bevy-idiomatic; UE backend would resolve Entity -> chunk index. Requires reworking spatial query result delivery.

## Verdict

The **authoring experience is nearly identical** across backends. A game developer writing systems uses the same `Query`, `QueryAll`, `Res<Time>`, `Commands`, and `MessageWriter`/`MessageReader` types in both modes. The `#[mass_system]` macro handles all backend-specific rewrites.

The single remaining asymmetry (spatial queries) is inherent to the architecture — Bevy collision detection and UE physics sweeps are fundamentally different APIs. A facade trait would bridge this but hasn't been prioritized.
