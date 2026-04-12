# Deduplicate standalone cooldown system

`gatherers-standalone/src/main.rs` has its own `entity_cooldown_system` that duplicates the shared `gatherers_sim::movement::entity_cooldown`. The standalone version exists because the shared one uses `BevyQuery` (which resolves to `bevy_ecs::Query` in both modes) — so it should be directly usable.

## Goal

Delete the standalone copy and import the shared system:
```rust
use gatherers_sim::movement::entity_cooldown;
```

May need minor signature alignment (the standalone version takes `Res<bevy_mass::DeltaTime>` while the shared one also does — they should already be compatible).
