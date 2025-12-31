#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(8))]
pub struct FTraceFilterData {
    pub name: FString,
    pub allowlisted_names: TArray<FString>,
}
pub struct ULocalTraceFilterPresetContainer {
    pub user_presets: TArray<FTraceFilterData>,
}
pub struct USharedTraceFilterPresetContainer {
    pub shared_presets: TArray<FTraceFilterData>,
}
