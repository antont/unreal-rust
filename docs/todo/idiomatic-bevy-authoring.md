# Idiomatic Bevy sim authoring — remaining

Items 1–16 are done — see [done/idiomatic-bevy-authoring.md](done/idiomatic-bevy-authoring.md).

The goal is not to hide UE entirely, but to provide a nice way to use UE from Rust where core sim logic authoring is idiomatic Bevy — and the same sim can run with pure Bevy too (when standalone systems replicate the relevant UE features).

## 17. Bevy-style entity spawning (deferred)

The UE bridge layer (`gatherers-bevy-mass/src/init.rs`) uses `EntityArchetype::new("food").fragment::<T>().spawn(count, |i, writer| {...})` for entity setup. The standalone runner uses standard `commands.spawn((Food, FoodState::default(), ...))`.

A shared `spawn_group` facade would look like:

```rust
sim.spawn_group("food", food_count, |i| {
    (Food, FoodState::default(), Transform::from_translation(...))
});
```

Status: **deferred**. Reasons the convergence is non-trivial:

- Standalone entities carry rendering components (`Sprite`, `Transform` for screen-space, colour config) that have no UE counterpart. A shared tuple would need per-backend extension hooks.
- UE bulk-spawns into chunk archetypes and returns `MassEntityHandle`; Bevy spawns individual entities and returns `Entity`. A unified return type is possible but adds surface.
- Init code is bridge-layer — it's written once per entity type and rarely touched, so the return on the abstraction is low compared with per-frame system code.

If revisited, the right shape is probably a trait-driven spawner where the game crate declares fragment groups + per-backend rendering extension, and the framework owns bulk-spawn vs `spawn_batch` dispatch.
