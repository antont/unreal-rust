#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(16))]
pub struct FPreviewSceneProfile {
    pub profile_name: FString,
    pub b_shared_profile: bool,
    pub b_is_engine_default_profile: bool,
    pub b_use_sky_lighting: bool,
    pub directional_light_intensity: f32,
    pub directional_light_color: crate::bindings::core_u_object::FLinearColor,
    pub sky_light_intensity: f32,
    pub b_rotate_lighting_rig: bool,
    pub b_show_environment: bool,
    pub b_show_floor: bool,
    pub b_show_grid: bool,
    pub environment_color: crate::bindings::core_u_object::FLinearColor,
    pub environment_intensity: f32,
    pub environment_cube_map: TSoftObjectPtr<crate::bindings::engine::UTextureCube>,
    pub environment_cube_map_path: FString,
    pub b_post_processing_enabled: bool,
    pub post_processing_settings: crate::bindings::engine::FPostProcessSettings,
    pub lighting_rig_rotation: f32,
    pub rotation_speed: f32,
    pub directional_light_rotation: crate::bindings::core_u_object::FRotator,
    pub b_enable_tone_mapping: bool,
    pub b_show_mesh_edges: bool,
}
pub struct UDefaultEditorProfiles {
    pub profiles: TArray<FPreviewSceneProfile>,
}
pub struct ULocalProfiles {
    pub profiles: TArray<FPreviewSceneProfile>,
}
pub struct USharedProfiles {
    pub profiles: TArray<FPreviewSceneProfile>,
}
pub struct UAssetViewerSettings {
    pub profiles: TArray<FPreviewSceneProfile>,
    pub b_fake_config_value_hack: bool,
}
