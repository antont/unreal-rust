#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::prelude::*;
#[repr(C, align(8))]
pub struct FActorPaletteMapEntry {
    pub map_path: FString,
}
pub struct UActorPaletteSettings {
    pub settings_per_level: TArray<FActorPaletteMapEntry>,
    pub recently_used_list: TArray<FString>,
    pub most_recent_level_by_tab: TArray<FString>,
    pub favorites_list: TArray<FString>,
    pub num_recent_levels_to_keep: i32,
}
