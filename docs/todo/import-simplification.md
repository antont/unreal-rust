# Simplify imports for game authors

## Current state

`bevy_mass::prelude` already re-exports `Entity`, `With`, `Without`, `Commands`, `BevyQuery`, `mass_system` (conditional on `unreal` feature). But existing code in `gatherers-bevy-mass/src/systems.rs` still imports these from `bevy_ecs` directly:

```rust
use unreal_api::mass::{MassQuery, MassQueryAll};  // not in prelude
use unreal_api::mass_system;                       // already in prelude (unused import)
use glam::DVec3;                                   // not in prelude
use bevy_ecs::prelude::{With, Without, Entity, Commands};  // already in prelude (unused import)
use bevy_ecs::system::Query as BevyQuery;          // already in prelude (unused import)
```

## What's left

1. **Add `MassQuery` / `MassQueryAll` to the prelude** (conditional on `unreal` feature)
2. **Add `glam::DVec3`** to the prelude (used in virtually every system)
3. **Clean up existing imports** to use `use bevy_mass::prelude::*` instead of redundant direct imports

## Goal

One import covers everything a game author needs:

```rust
use bevy_mass::prelude::*;
```
