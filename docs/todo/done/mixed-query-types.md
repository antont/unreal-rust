# Mixed query types: pure-Bevy components alongside MassFragments — DONE

The `#[mass_system]` macro now supports mixed param types in the same system:

- **MassQuery / MassQueryAll** — rewritten to chunk access (zero-copy FFI fragments)
- **BevyQuery** — passed through as real `bevy_ecs::Query` (pure-Bevy components on shadow entities)
- **Commands** — passed through for entity manipulation (add/remove components)
- **Res<T>** — passed through for Bevy resources (e.g. MassSpatialQueries)

Shadow Bevy entities are spawned per Mass Entity handle via `MassEntityMap`. Cooldown was migrated from MassFragment to pure-Bevy component as the first use case — added on food pickup/drop, removed when timer expires.

Pure-Bevy systems (zero fragment requirements) skip C++ processor registration entirely — Mass Entity asserts on empty queries.
