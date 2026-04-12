# Migrate Cooldown to pure-Bevy component — DONE

Cooldown was converted from a MassFragment (`FGatherersCooldown` in C++) to a plain `#[derive(Component)]` on shadow Bevy entities. Enables idiomatic add/remove via Commands instead of sentinel values.

- `gatherers-sim/src/fragments.rs`: `Cooldown` is `#[derive(Component)]`, not `mass_fragment!`
- `gatherers-bevy-mass/src/init.rs`: no `.fragment::<Cooldown>()` in archetype
- `ant_food_decision`: uses `Without<Cooldown>` filter + `commands.entity(e).insert(Cooldown {...})`
- `entity_cooldown`: uses `BevyQuery<(Entity, &mut Cooldown)>` + `commands.entity(e).remove::<Cooldown>()`
- C++ side: removed `FGatherersCooldown` from generated header and tests
- Pure-Bevy systems (zero fragment requirements) skip C++ processor registration
