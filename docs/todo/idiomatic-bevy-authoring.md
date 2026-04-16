# Idiomatic Bevy sim authoring — remaining

Items 1–8, 10 are done — see [done/idiomatic-bevy-authoring.md](done/idiomatic-bevy-authoring.md).

## 9. Entity references instead of numeric indices (future)

Currently game code uses `Carrying.food_index: i32` (numeric index into "all food entities") because UE Mass Entity's chunk architecture uses index-based access. In pure Bevy this would be `Option<Entity>`. Consider changing game logic to use `Entity` references with the UE backend resolving them to chunk indices — more Bevy-idiomatic, but requires reworking how C++ spatial query results are consumed and how cross-archetype references work.
