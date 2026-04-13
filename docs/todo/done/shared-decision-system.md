# Shared food decision system via messages — DONE

Both standalone and Unreal now use the message-based pipeline:

1. **Collision prepass** (mode-specific) → emits `AntFoodHit` messages
   - Standalone: brute-force proximity search
   - Unreal: `MassSpatialQueries` physics sweep
2. **`food_decision_system`** (shared, in `gatherers-sim`) → reads hits, calls `ant_food_decision()`, emits `FoodMutation`
   - Standalone uses this directly
   - Unreal has its own system wrapper (chunk iteration + HashMap lookup) but calls the same pure function
3. **`apply_food_mutations`** (mode-specific) → reads mutations, updates `FoodFragment`

Single source of truth for pickup/drop logic: `ant_food_decision()` in `gatherers-sim/src/food_decision.rs`.
