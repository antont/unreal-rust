#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::prelude::*;
#[repr(C, align(8))]
pub struct FCustomTextFilterState {
    pub b_is_checked: bool,
    pub b_is_active: bool,
    pub filter_data: FCustomTextFilterData,
}
#[repr(C, align(8))]
pub struct FFilterBarSettings {
    pub custom_filters: TMap<FString, bool>,
    pub type_filters: TMap<FString, bool>,
    pub custom_text_filters: TArray<FCustomTextFilterState>,
    pub b_is_layout_saved: bool,
    pub filter_bar_layout: EFilterBarLayout,
    pub b_filter_recursively: bool,
}
pub struct UFilterBarConfig {
    pub filter_bars: TMap<FName, FFilterBarSettings>,
}
pub struct UAssetFilterBarContext {}
