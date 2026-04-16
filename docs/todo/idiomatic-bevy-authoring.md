# Idiomatic Bevy sim authoring — remaining

Items 1–8, 10–11 are done — see [done/idiomatic-bevy-authoring.md](done/idiomatic-bevy-authoring.md).

## 9. Entity references instead of numeric indices (future)

Currently game code uses `Carrying.food_index: i32` (numeric index into "all food entities") because UE Mass Entity's chunk architecture uses index-based access. In pure Bevy this would be `Option<Entity>`. Consider changing game logic to use `Entity` references with the UE backend resolving them to chunk indices — more Bevy-idiomatic, but requires reworking how C++ spatial query results are consumed and how cross-archetype references work.

## 12. `#[derive(Component)]` instead of `#[component]` attribute (future)

Currently game components use `#[component]` (attribute macro) which adds `#[repr(C)]`, `#[derive(Component, ...)]`, and MassFragment registration. Vanilla Bevy uses `#[derive(Component)]`. To match that exactly, enhance the existing `Component` derive to auto-register as MassFragment when `#[repr(C)]` is present (the FFI layout marker doubles as the opt-in signal). Tags (unit structs) would be auto-detected. Pure-Bevy components without `#[repr(C)]` stay unaffected. This would make game code identical to standard Bevy except for the `#[repr(C)]` line on chunk-memory types.
