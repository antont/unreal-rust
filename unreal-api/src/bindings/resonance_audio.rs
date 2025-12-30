#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::prelude::*;
#[repr(C, align(16))]
pub struct FResonanceAudioReverbPluginSettings {
    pub b_enable_room_effects: bool,
    pub b_get_transform_from_audio_volume: bool,
    pub room_position: FVector,
    pub room_rotation: FQuat,
    pub room_dimensions: FVector,
    pub left_wall_material: ERaMaterialName,
    pub right_wall_material: ERaMaterialName,
    pub floor_material: ERaMaterialName,
    pub ceiling_material: ERaMaterialName,
    pub front_wall_material: ERaMaterialName,
    pub back_wall_material: ERaMaterialName,
    pub reflection_scalar: f32,
    pub reverb_gain: f32,
    pub reverb_time_modifier: f32,
    pub reverb_brightness: f32,
}
pub struct UResonanceAudioSoundfieldSettings {
    pub render_mode: EResonanceRenderMode,
}
pub struct UResonanceAudioBlueprintFunctionLibrary {}
pub struct AResonanceAudioDirectivityVisualizer {
    pub material: UPtr<UMaterial>,
    pub settings: UPtr<UResonanceAudioSpatializationSourceSettings>,
}
pub struct UResonanceAudioReverbPluginPreset {
    pub settings: FResonanceAudioReverbPluginSettings,
}
pub struct UResonanceAudioSettings {
    pub output_submix: FSoftObjectPath,
    pub quality_mode: ERaQualityMode,
    pub global_reverb_preset: FSoftObjectPath,
    pub global_source_preset: FSoftObjectPath,
}
pub struct UResonanceAudioSpatializationSourceSettings {
    pub spatialization_method: ERaSpatializationMethod,
    pub pattern: f32,
    pub sharpness: f32,
    pub b_toggle_visualization: bool,
    pub scale: f32,
    pub spread: f32,
    pub rolloff: ERaDistanceRolloffModel,
    pub min_distance: f32,
    pub max_distance: f32,
}
