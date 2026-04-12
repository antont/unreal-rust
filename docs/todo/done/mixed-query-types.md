# Mixed query types: pure-Bevy components alongside MassFragments

Support having both MassQuery parameters (for FFI fragments in chunk memory) and regular Bevy Query parameters (for pure-Bevy components) in the same system.

## Why

Currently all system parameters in a `#[mass_system]` are rewritten to MassQuery, backed by Mass Entity chunk memory. This forces all per-entity state to be MassFragments with `#[repr(C)]` layout, even for internal sim state that UE doesn't need (e.g. cooldown timers, AI state).

Idiomatic Bevy uses component presence for state transitions (add when triggered, remove when expired). This is cleaner and lets systems use `With<T>` / `Without<T>` filters. But it requires the component to live on real Bevy entities, not in chunk memory.

## What's needed

- Real Bevy entities that correspond to Mass Entity handles (at least for non-FFI components)
- The `#[mass_system]` macro needs to distinguish which params to rewrite (MassQuery for FFI fragments) and which to leave as real Bevy Queries (for pure components)
- An annotation or trait to mark which types are MassFragments vs pure components
- Entity correspondence mapping between Mass Entity handles and Bevy Entity IDs

## Example: Cooldown

Currently `Cooldown` is a MassFragment (`FGatherersCooldown` in C++). But UE never reads or writes it — it's purely internal sim logic. It could be a pure Bevy component: added on food pickup/drop, removed when timer expires. The `entity_cooldown` system would use `Query<(Entity, &mut Cooldown)>` and call `commands.entity(e).remove::<Cooldown>()`.

## Depends on

- `runtime-archetype-mutation.md` (for the Bevy-side add/remove to work)
- Changes to `unreal-api-derive/src/mass_system.rs` (macro rewrite logic)
- Entity mapping in `unreal-module/src/mass_system_registry.rs`
