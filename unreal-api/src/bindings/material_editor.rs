#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(4))]
pub struct FCheckerboardSettings {
    pub color_one: crate::bindings::core_u_object::FColor,
    pub color_two: crate::bindings::core_u_object::FColor,
    pub size: i32,
}
#[repr(C, align(4))]
pub struct FPreviewBackgroundSettings {
    pub b_show_border: bool,
    pub border_color: crate::bindings::core_u_object::FColor,
    pub background_type: EBackgroundType,
    pub background_color: crate::bindings::core_u_object::FColor,
    pub checkerboard: FCheckerboardSettings,
}
#[repr(C, align(4))]
pub struct FMaterialStatistics {
    pub num_vertex_shader_instructions: i32,
    pub num_pixel_shader_instructions: i32,
    pub num_samplers: i32,
    pub num_vertex_texture_samples: i32,
    pub num_pixel_texture_samples: i32,
    pub num_virtual_texture_samples: i32,
    pub num_uv_scalars: i32,
    pub num_interpolator_scalars: i32,
}
#[repr(C, align(8))]
pub struct FSortedParamData {
    pub parameter: UPtr<crate::bindings::unreal_ed::UDEditorParameterValue>,
}
#[repr(C, align(8))]
pub struct FUnsortedParamData {
    pub parameter: UPtr<crate::bindings::unreal_ed::UDEditorParameterValue>,
}
pub struct UMaterialEditorMenuContext {}
pub struct UMaterialEditorSettings {
    pub b_allow_ignoring_compilation_errors: bool,
    pub context_menu_node_title_weight: f32,
    pub context_menu_keyword_weight: f32,
    pub context_menu_description_weight: f32,
    pub context_menu_category_weight: f32,
    pub context_menu_whole_match_localized_weight_multiplier: f32,
    pub context_menu_whole_match_weight_multiplier: f32,
    pub context_menu_starts_with_bonus_weight_multiplier: f32,
    pub context_menu_percentage_match_weight_multiplier: f32,
    pub context_menu_shorter_match_weight: f32,
    pub offline_compiler: EOfflineShaderCompiler,
    pub offline_compiler_path: crate::bindings::core_u_object::FFilePath,
    pub gpu_target: FString,
    pub b_save_compiler_stats_files: bool,
    pub b_dump_all: bool,
    pub default_preview_width: i32,
    pub default_preview_height: i32,
    pub preview_background: FPreviewBackgroundSettings,
}
pub struct UMaterialEditingLibrary {}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EBackgroundType(pub u8);
impl EBackgroundType {
    pub const SOLID_COLOR: EBackgroundType = EBackgroundType(0);
    pub const CHECKERED: EBackgroundType = EBackgroundType(1);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EOfflineShaderCompiler(pub u8);
impl EOfflineShaderCompiler {
    pub const MALI: EOfflineShaderCompiler = EOfflineShaderCompiler(0);
    pub const ADRENO: EOfflineShaderCompiler = EOfflineShaderCompiler(1);
}
