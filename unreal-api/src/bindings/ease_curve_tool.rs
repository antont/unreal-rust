#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::prelude::*;
#[repr(C, align(8))]
pub struct FEaseCurvePreset {
    pub category: FText,
    pub name: FText,
    pub tangents: FEaseCurveTangents,
}
#[repr(C, align(8))]
pub struct FEaseCurveTangents {
    pub start: f64,
    pub start_weight: f64,
    pub end: f64,
    pub end_weight: f64,
}
pub struct UEaseCurveSerializer {}
pub struct UCubicBezierCurveSerializer {}
pub struct UEaseCurve {}
pub struct UEaseCurveLibrary {
    pub presets: TArray<FEaseCurvePreset>,
    pub empty_categories: TArray<FText>,
}
pub struct UEaseCurveLibraryFactory {}
pub struct UEaseCurveToolMenuContext {}
pub struct UEaseCurveToolSettings {
    pub b_show_in_sidebar: bool,
    pub b_show_curve_editor_toolbar_button: bool,
    pub b_tool_tab_visible: bool,
    pub default_preset_library: TSoftObjectPtr<UEaseCurveLibrary>,
    pub new_preset_category: FText,
    pub quick_ease_tangents: FString,
    pub graph_size: i32,
    pub b_grid_snap: bool,
    pub grid_size: i32,
    pub b_auto_zoom_to_fit: bool,
    pub b_auto_flip_tangents: bool,
}
