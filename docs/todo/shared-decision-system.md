# Shared food decision system via encounter pattern

Refactor standalone to use the same two-system pattern as Unreal mode (collision pre-pass -> encounter data -> decision), making `ant_food_decision` shared code.

## Current state

- **Unreal**: C++ spatial query writes `AntEncounterFragment` -> `ant_food_decision` reads it
- **Standalone**: `simple_food_interaction` does brute-force proximity + pickup/drop in one system

The decision logic (pickup/drop/cooldown) is identical — only collision detection differs.

## What's needed

1. Standalone gets a `simple_collision_prepass` that writes `AntEncounterFragment` (brute-force proximity search)
2. `ant_food_decision` becomes a shared system using facade Query — same source in both modes
3. Only the collision/encounter detection remains mode-specific (C++ spatial query vs brute-force)

## Benefit

Single source of truth for food interaction logic. Changes to pickup/drop behavior automatically apply to both modes.
