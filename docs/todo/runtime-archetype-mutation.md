# Runtime archetype mutation support

Enable adding/removing fragments from entities during simulation, allowing idiomatic Bevy patterns (component presence/absence) to work in both pure Bevy and Unreal modes.

## Current state

The zero-copy bridge caches raw pointers into Mass Entity chunk memory on first frame and reuses them every subsequent frame. If an entity gains or loses a fragment, it moves to a different archetype's chunks, invalidating cached pointers. The cache is only rebuilt between PIE sessions, not mid-simulation.

This forces dual-mode code to use persistent fields with sentinel values (e.g. `Cooldown { remaining_seconds: 0.0 }` meaning "inactive") instead of idiomatic Bevy add/remove component patterns.

## Why it matters

Idiomatic Bevy uses component presence for state transitions (add `Cooldown` when triggered, remove when expired, filter with `With<Cooldown>`). This is cleaner and more efficient than sentinel-value checks. For the facade architecture to feel natural to Bevy developers, it should support this pattern.

## Mass Entity supports this

Mass Entity natively supports archetype mutation — entities move between archetypes when fragments are added/removed, just like Bevy. The limitation is entirely in our bridge's caching layer, not in UE.

## What's needed

- Detect archetype changes in `RustMassDynamicProcessor.cpp` (Mass Entity has observer hooks)
- Invalidate and rebuild the chunk pointer cache when mutations occur
- Handle entity count changes per chunk after mutations
- Expose add/remove fragment operations through the FFI for Rust systems to call
- Update `bevy_mass` facade to map Bevy `Commands::insert`/`Commands::remove` to the FFI in Unreal mode
