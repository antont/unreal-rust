# Unify system authoring styles — DONE

Reduced from three styles to two:

1. **Shared facade Query** — `bevy_mass::prelude::Query<&mut T>` + `Res<DeltaTime>` (entity_movement, entity_boundary_reflect, entity_cooldown). Compiles in both pure-Bevy and Unreal modes.
2. **Tuple MassQuery** — `MassQuery<(Entity, &mut T, ...), (With<Tag>, Without<V>)>` (ant_food_decision, ant_collision_prepass, carried_food_tracking). Unreal-only, supports Entity access, tuple destructuring, With/Without filters.

The old zip pattern (separate `MassQuery<&T>` per component) was eliminated — `ant_collision_prepass` and `carried_food_tracking` were migrated to tuple MassQuery form.
