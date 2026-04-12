# Runtime archetype mutation support

Enable adding/removing fragments from entities during simulation, allowing idiomatic Bevy patterns (component presence/absence) to work in both pure Bevy and Unreal modes.

## Current state

The zero-copy bridge caches raw pointers into Mass Entity chunk memory on first frame and reuses them every subsequent frame. If an entity gains or loses a fragment, it moves to a different archetype's chunks, invalidating cached pointers. The cache is only rebuilt between PIE sessions, not mid-simulation.

**Workaround proven:** Cooldown was migrated to a pure-Bevy component on shadow entities (not a MassFragment), enabling idiomatic add/remove via `Commands`. This is fully working with `Without<Cooldown>` filters, shadow entity mapping, and pure-Bevy system scheduling. It covers lightweight state like timers but doesn't help for MassFragment-level mutations (e.g., adding/removing a fragment that C++ also reads).

## Why it matters

Idiomatic Bevy uses component presence for state transitions. The shadow-entity approach works for Bevy-only components, but if a fragment visible to C++ (e.g., a behavior mode tag) needs to be added/removed at runtime, the current bridge can't handle it.

## Mass Entity supports this

Mass Entity natively supports archetype mutation — entities move between archetypes when fragments are added/removed, just like Bevy. The limitation is entirely in our bridge's caching layer, not in UE.

## What's needed

- Detect archetype changes in `RustMassDynamicProcessor.cpp` (Mass Entity has observer hooks)
- Invalidate and rebuild the chunk pointer cache when mutations occur
- Handle entity count changes per chunk after mutations
- Expose add/remove fragment operations through the FFI for Rust systems to call
- Update `bevy_mass` facade to map Bevy `Commands::insert`/`Commands::remove` to the FFI in Unreal mode
