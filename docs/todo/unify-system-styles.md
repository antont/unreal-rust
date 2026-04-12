# Unify system authoring styles

Three coexisting system styles after the tuple facade Query work:

1. **Shared facade Query** — `bevy_mass::prelude::Query<&mut T>` + `Res<DeltaTime>` (entity_movement, entity_boundary_reflect). Compiles in both pure-Bevy and Unreal modes.
2. **Tuple MassQuery** — `MassQuery<(Entity, &mut T, ...), (With<Tag>, Without<V>)>` (ant_food_decision). Unreal-only, supports Entity access, tuple destructuring, With/Without filters.
3. **Zip MassQuery** — separate `MassQuery<&T, With<Tag>>` per component, zipped manually (ant_collision_prepass, carried_food_tracking). Unreal-only, legacy pattern.

The key split: style 1 is cross-platform, styles 2+3 are Unreal-only (needed for MassSpatialQueries, MassQueryAll cross-archetype access, etc.). A new author has to learn when each applies.

## Goal

Reduce to two styles: shared facade Query (cross-platform) and a single Unreal-only MassQuery that supports tuples, Entity, and filters. The zip pattern (style 3) should be migrated to tuple form (style 2) where possible. Document the two-style boundary clearly.

## Depends on

- Current macro infrastructure (tuple facade struct generation)
- ant_collision_prepass and carried_food_tracking could potentially be rewritten with tuple MassQuery to eliminate style 3
