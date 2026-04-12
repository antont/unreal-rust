# Deduplicate standalone cooldown system — DONE

Standalone now imports the shared `entity_cooldown` from `gatherers_sim::movement` instead of having its own copy. `BevyQuery` resolves to `bevy_ecs::Query` in standalone mode, so the shared system works directly.
