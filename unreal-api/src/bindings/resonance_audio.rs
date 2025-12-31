#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(16))]
pub struct FResonanceAudioReverbPluginSettings {
    pub b_enable_room_effects: bool,
    pub b_get_transform_from_audio_volume: bool,
    pub room_position: crate::bindings::core_u_object::FVector,
    pub room_rotation: crate::bindings::core_u_object::FQuat,
    pub room_dimensions: crate::bindings::core_u_object::FVector,
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
    pub material: UPtr<crate::bindings::engine::UMaterial>,
    pub settings: UPtr<UResonanceAudioSpatializationSourceSettings>,
}
pub struct UResonanceAudioReverbPluginPreset {
    pub settings: FResonanceAudioReverbPluginSettings,
}
pub struct UResonanceAudioSettings {
    pub output_submix: crate::bindings::core_u_object::FSoftObjectPath,
    pub quality_mode: ERaQualityMode,
    pub global_reverb_preset: crate::bindings::core_u_object::FSoftObjectPath,
    pub global_source_preset: crate::bindings::core_u_object::FSoftObjectPath,
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
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ERaMaterialName(pub u8);
impl ERaMaterialName {
    pub const TRANSPARENT: ERaMaterialName = ERaMaterialName(0);
    pub const ACOUSTIC_CEILING_TILES: ERaMaterialName = ERaMaterialName(1);
    pub const BRICK_BARE: ERaMaterialName = ERaMaterialName(2);
    pub const BRICK_PAINTED: ERaMaterialName = ERaMaterialName(3);
    pub const CONCRETE_BLOCK_COARSE: ERaMaterialName = ERaMaterialName(4);
    pub const CONCRETE_BLOCK_PAINTED: ERaMaterialName = ERaMaterialName(5);
    pub const CURTAIN_HEAVY: ERaMaterialName = ERaMaterialName(6);
    pub const FIBER_GLASS_INSULATION: ERaMaterialName = ERaMaterialName(7);
    pub const GLASS_THIN: ERaMaterialName = ERaMaterialName(8);
    pub const GLASS_THICK: ERaMaterialName = ERaMaterialName(9);
    pub const GRASS: ERaMaterialName = ERaMaterialName(10);
    pub const LINOLEUM_ON_CONCRETE: ERaMaterialName = ERaMaterialName(11);
    pub const MARBLE: ERaMaterialName = ERaMaterialName(12);
    pub const METAL: ERaMaterialName = ERaMaterialName(13);
    pub const PARQUET_ONCONCRETE: ERaMaterialName = ERaMaterialName(14);
    pub const PLASTER_ROUGH: ERaMaterialName = ERaMaterialName(15);
    pub const PLASTER_SMOOTH: ERaMaterialName = ERaMaterialName(16);
    pub const PLYWOOD_PANEL: ERaMaterialName = ERaMaterialName(17);
    pub const POLISHED_CONCRETE_OR_TILE: ERaMaterialName = ERaMaterialName(18);
    pub const SHEETROCK: ERaMaterialName = ERaMaterialName(19);
    pub const WATER_OR_ICE_SURFACE: ERaMaterialName = ERaMaterialName(20);
    pub const WOOD_CEILING: ERaMaterialName = ERaMaterialName(21);
    pub const WOOD_PANEL: ERaMaterialName = ERaMaterialName(22);
    pub const UNIFORM: ERaMaterialName = ERaMaterialName(23);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EResonanceRenderMode(pub u8);
impl EResonanceRenderMode {
    pub const STEREO_PANNING: EResonanceRenderMode = EResonanceRenderMode(0);
    pub const BINAURAL_LOW_QUALITY: EResonanceRenderMode = EResonanceRenderMode(1);
    pub const BINAURAL_MEDIUM_QUALITY: EResonanceRenderMode = EResonanceRenderMode(2);
    pub const BINAURAL_HIGH_QUALITY: EResonanceRenderMode = EResonanceRenderMode(3);
    pub const ROOM_EFFECTS_ONLY: EResonanceRenderMode = EResonanceRenderMode(4);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ERaQualityMode(pub u8);
impl ERaQualityMode {
    pub const STEREO_PANNING: ERaQualityMode = ERaQualityMode(0);
    pub const BINAURAL_LOW: ERaQualityMode = ERaQualityMode(1);
    pub const BINAURAL_MEDIUM: ERaQualityMode = ERaQualityMode(2);
    pub const BINAURAL_HIGH: ERaQualityMode = ERaQualityMode(3);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ERaSpatializationMethod(pub u8);
impl ERaSpatializationMethod {
    pub const STEREO_PANNING: ERaSpatializationMethod = ERaSpatializationMethod(0);
    pub const HRTF: ERaSpatializationMethod = ERaSpatializationMethod(1);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ERaDistanceRolloffModel(pub u8);
impl ERaDistanceRolloffModel {
    pub const LOGARITHMIC: ERaDistanceRolloffModel = ERaDistanceRolloffModel(0);
    pub const LINEAR: ERaDistanceRolloffModel = ERaDistanceRolloffModel(1);
    pub const NONE: ERaDistanceRolloffModel = ERaDistanceRolloffModel(2);
}
