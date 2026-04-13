# Runtime archetype mutation — decided against

## What this was

Adding/removing MassFragments from entities at runtime, which would move them between archetypes in Mass Entity's chunk storage.

## Why we're not doing it

**Performance grain:** Mass Entity is designed around stable archetypes. Archetype moves are expensive — the entity's data gets memcopied between chunks. UE's own Mass processors (e.g. MassStateTreeProcessor) typically use flag fields or enums within fragments rather than toggling fragment presence for hot-path state changes.

**Bridge complexity:** Our zero-copy bridge caches raw pointers into chunk memory. Archetype mutation invalidates these pointers. Supporting it would require observer hooks to detect archetype changes, cache invalidation/rebuild per frame, and FFI operations for add/remove — significant complexity for the caching layer.

**Existing patterns cover real needs:**

- **Shadow Bevy entities** for lightweight state with add/remove semantics (proven with Cooldown — supports `Without<Cooldown>` filters, Commands insert/remove, pure-Bevy scheduling)
- **Enum/flag fields** inside existing fragments for state visible to C++ (e.g. `Carrying.food_index == -1` means "not carrying", `Behavior` enum fields for mode switches)

These two patterns handle every use case we've encountered without archetype mutation.

## If this ever becomes needed

The path would be: detect archetype changes via Mass Entity observer hooks in `RustMassDynamicProcessor.cpp`, invalidate and rebuild chunk pointer caches when mutations occur, expose add/remove fragment operations through the FFI. But the expected use cases don't justify this.
