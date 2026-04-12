# Make entity_group type-safe

Currently `entity_group = "ants"` is a string that must match the string used in `MassEntityMap::insert_group("ants", ...)`. No compile-time verification.

## Problem

A typo like `entity_group = "ant"` compiles fine but panics at runtime when the wrapper calls `entity_map.group("ant")` and gets None.

## Possible approaches

- Derive entity group identifiers from the With<Tag> filter (e.g. `With<BevyMassAntTag>` implies group "ants")
- Use a const or type instead of a string
- Validate at startup by checking all registered systems against available entity map groups
