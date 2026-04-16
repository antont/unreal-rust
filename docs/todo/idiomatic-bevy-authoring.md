# Idiomatic Bevy sim authoring — remaining

Items 1–8, 10–11, 13–14 are done — see [done/idiomatic-bevy-authoring.md](done/idiomatic-bevy-authoring.md).

The goal is not to hide UE entirely, but to provide a nice way to use UE from Rust where core sim logic authoring is idiomatic Bevy — and the same sim can run with pure Bevy too (when standalone systems replicate the relevant UE features).

## 9. Entity references instead of numeric indices (future)

Currently game code uses `Carrying.food_index: i32` (numeric index into "all food entities") because UE Mass Entity's chunk architecture uses index-based access. In pure Bevy this would be `Option<Entity>`. Consider changing game logic to use `Entity` references with the UE backend resolving them to chunk indices — more Bevy-idiomatic, but requires reworking how C++ spatial query results are consumed and how cross-archetype references work.

## 12. `#[derive(Component)]` instead of `#[component]` attribute (future)

Currently game components use `#[component]` (attribute macro) which adds `#[repr(C)]`, `#[derive(Component, ...)]`, and MassFragment registration. Vanilla Bevy uses `#[derive(Component)]`. To match that exactly, enhance the existing `Component` derive to auto-register as MassFragment when `#[repr(C)]` is present (the FFI layout marker doubles as the opt-in signal). Tags (unit structs) would be auto-detected. Pure-Bevy components without `#[repr(C)]` stay unaffected. This would make game code identical to standard Bevy except for the `#[repr(C)]` line on chunk-memory types.

## 15. Standard Bevy system ordering instead of `#[mass_system(order = N)]`

Currently systems use `#[mass_system(order = 50)]` for execution ordering. A Bevy developer would expect `.add_systems(Update, (a, b, c).chain())` or explicit ordering constraints like `.before()` / `.after()`.

The `#[mass_system]` attribute is needed for UE chunk rewriting (rewriting `Query` to chunk iteration), but the **ordering** could potentially be expressed in standard Bevy style. Options:

**A. Keep `#[mass_system]` for UE chunk rewriting, move ordering to plugin registration.**
Systems would be `#[mass_system]` with no order arg. Ordering expressed via `.chain()` or `.before()`/`.after()` in the plugin's `build()`. The framework extracts the schedule order for UE processor registration. This is how vanilla Bevy does it.

**B. Support both — attribute order as shorthand, plugin ordering as override.**
Keep `order = N` as a convenient default (maps directly to UE processor execution order), but allow plugin-level overrides for projects that prefer Bevy-style ordering.

The UE side needs a numeric execution order for `URustMassDynamicProcessor`. Whatever Bevy-style ordering is used, it must resolve to a total order for the C++ pipeline.

## 16. Reduce extra query types (`QueryAll`, `BevyQuery`)

Game systems currently use three query types beyond standard `Query`:

- **`QueryAll<&mut T, With<Tag>>`** — index-based global access (`get_mut(i)`). Needed because UE chunk architecture uses indices for cross-archetype references. In vanilla Bevy this would be a normal `Query` with entity lookup. Closely related to item 9 (entity references) — if entity references replace indices, `QueryAll` may become unnecessary.

- **`BevyQuery<D, F>`** — ~~escapes `#[mass_system]` chunk rewriting for pure-Bevy shadow components~~ **DONE (Step A)**: Replaced with `#[bevy]` parameter attribute. **DONE (Step B)**: Fully automatic via `QueryBackend::IS_CHUNK` const-if dispatch. The macro emits both chunk and Bevy paths; the compiler eliminates the dead branch. No annotation needed — `#[bevy]` attribute is deprecated. Works for both single queries (`Query<&T>`) and tuple queries (`Query<(Entity, &mut T)>`).

Possible improvements:

- **For `BevyQuery`**: **DONE** — fully automatic detection via `ChunkBacked` marker trait + `MaybeFragment` specialization. Non-MassFragment types auto-dispatch to Bevy storage.

- **For `QueryAll`**: Depends on item 9. If entity references replace indices, systems would use normal `Query::get(entity)` instead of `QueryAll::get_mut(index)`. Until then, `QueryAll` is a necessary facade.

### Implementation details (Step B)

The dual-mode dispatch works via:
- **`ChunkBacked` marker trait** — implemented by `#[derive(MassFragment)]` types, sets `QueryBackend::IS_CHUNK = true`
- **`MaybeFragment` specialization trait** — compile-time detection of whether a type has C++ MassFragment representation (`IS_FRAGMENT`, `CPP_TYPE_NAME_OR_EMPTY`)
- **Const-if in generated code** — `if <T as QueryBackend>::IS_CHUNK { chunk_path } else { bevy_path }`, compiler eliminates dead branch
- **IS_CHUNK consistency guard** — compile-time assert prevents mixing chunk-backed and Bevy-only fragments in one system's primary queries
- **`is_valid` on requirements** — non-MassFragment types get `is_valid: false`, filtered before C++ registration
- **Relaxed bounds** — `MassChunks`, `MassSystemChunks`, `MassQueryRef/Mut` accept `Copy + 'static` (not just `MassFragment`), allowing empty chunk resources for Bevy-only types

## 17. Bevy-style entity spawning

The UE bridge layer (`gatherers-bevy-mass/src/init.rs`) uses `EntityArchetype::new("food").fragment::<T>().spawn(count, |i, writer| {...})` for entity setup. The standalone runner uses standard `commands.spawn((Food, FoodState::default(), ...))`.

This divergence is acceptable — init code is bridge-layer, not core sim logic, and the two backends have genuinely different spawning models (UE bulk-spawns into chunk archetypes, Bevy spawns individual entities). But it's worth exploring whether a shared spawning API could cover both:

```rust
// Hypothetical: same code compiles for both backends
sim.spawn_group("food", food_count, |i| {
    (Food, FoodState::default(), Transform::from_translation(...))
});
```

This would let game authors define entity setup once. The framework would translate to `EntityArchetype` in UE mode and `commands.spawn()` in Bevy mode. Lower priority than system-level items since init code is written once per entity type.
