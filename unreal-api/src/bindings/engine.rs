#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(4))]
pub struct FFloatRK4SpringInterpolator {
    __padding_end: [u8; 8],
}
impl FFloatRK4SpringInterpolator {}
#[repr(C, align(8))]
pub struct FFormatArgumentData {
    pub argument_name: FString,
    pub argument_value_type: EFormatArgumentType,
    pub argument_value: FText,
    pub argument_value_int: i64,
    pub argument_value_float: f32,
    pub argument_value_double: f64,
    pub argument_value_gender: ETextGender,
    __padding_end: [u8; 7],
}
impl FFormatArgumentData {}
#[repr(C, align(4))]
pub struct FVectorRK4SpringInterpolator {
    __padding_end: [u8; 8],
}
impl FVectorRK4SpringInterpolator {}
#[repr(C, align(8))]
pub struct FAnimDataModelNotifPayload {
    __padding_end: [u8; 16],
}
impl FAnimDataModelNotifPayload {}
#[repr(C, align(16))]
pub struct FChaosBreakEvent {
    pub component: UPtr<UPrimitiveComponent>,
    pub location: crate::bindings::core_u_object::FVector,
    pub orientation: crate::bindings::core_u_object::FQuat,
    pub velocity: crate::bindings::core_u_object::FVector,
    pub angular_velocity: crate::bindings::core_u_object::FVector,
    pub extents: crate::bindings::core_u_object::FVector,
    pub mass: f32,
    pub index: i32,
    pub b_from_crumble: bool,
    __padding_end: [u8; 15],
}
impl FChaosBreakEvent {}
#[repr(C, align(8))]
pub struct FCollisionChaosEvent {
    pub location: crate::bindings::core_u_object::FVector,
    pub accumulated_impulse: crate::bindings::core_u_object::FVector,
    pub normal: crate::bindings::core_u_object::FVector,
    pub penetration_depth: f32,
    pub body1: FCollisionChaosEventBodyInfo,
    pub body2: FCollisionChaosEventBodyInfo,
}
impl FCollisionChaosEvent {}
#[repr(C, align(8))]
pub struct FCollisionChaosEventBodyInfo {
    pub velocity: crate::bindings::core_u_object::FVector,
    pub delta_velocity: crate::bindings::core_u_object::FVector,
    pub angular_velocity: crate::bindings::core_u_object::FVector,
    pub mass: f32,
    pub phys_material: UPtr<crate::bindings::physics_core::UPhysicalMaterial>,
    pub component: TWeakObjectPtr<UPrimitiveComponent>,
    pub body_index: i32,
    pub bone_name: FName,
}
impl FCollisionChaosEventBodyInfo {}
#[repr(C, align(16))]
pub struct FChaosCrumblingEvent {
    pub component: UPtr<UPrimitiveComponent>,
    pub location: crate::bindings::core_u_object::FVector,
    pub orientation: crate::bindings::core_u_object::FQuat,
    pub linear_velocity: crate::bindings::core_u_object::FVector,
    pub angular_velocity: crate::bindings::core_u_object::FVector,
    pub mass: f32,
    pub local_bounds: crate::bindings::core_u_object::FBox,
    pub children: TArray<i32>,
}
impl FChaosCrumblingEvent {}
#[repr(C, align(8))]
pub struct FHitResult {
    __padding_end: [u8; 264],
}
impl FHitResult {
    pub fn verify_layout() {}
}
#[repr(C, align(8))]
pub struct FActorInstanceHandle {
    __padding_end: [u8; 32],
}
impl FActorInstanceHandle {}
#[repr(C, align(8))]
pub struct FVector_NetQuantize {
    __padding_end: [u8; 24],
}
impl FVector_NetQuantize {}
#[repr(C, align(8))]
pub struct FVector_NetQuantizeNormal {
    __padding_end: [u8; 24],
}
impl FVector_NetQuantizeNormal {}
#[repr(C, align(8))]
pub struct FSubtitleCue {
    pub text: FText,
    pub time: f32,
    __padding_end: [u8; 4],
}
impl FSubtitleCue {}
#[repr(C, align(8))]
pub struct FChaosRemovalEvent {
    pub component: UPtr<UPrimitiveComponent>,
    pub location: crate::bindings::core_u_object::FVector,
    pub mass: f32,
    __padding_end: [u8; 4],
}
impl FChaosRemovalEvent {}
#[repr(C, align(8))]
pub struct FInterpControlPoint {
    pub position_control_point: crate::bindings::core_u_object::FVector,
    pub b_position_is_relative: bool,
    __padding_end: [u8; 15],
}
impl FInterpControlPoint {}
#[repr(C, align(8))]
pub struct FDebugFloatHistory {
    #[doc(hidden)]
    __padding_16: [u8; 16],
    pub max_samples: i32,
    pub min_value: f32,
    pub max_value: f32,
    pub b_auto_adjust_min_max: bool,
    __padding_end: [u8; 3],
}
impl FDebugFloatHistory {}
#[repr(C, align(8))]
pub struct FBaseComponentReference {
    pub component_property: FName,
    __padding_end: [u8; 28],
}
impl FBaseComponentReference {}
#[repr(C, align(8))]
pub struct FSoftComponentReference {
    #[doc(hidden)]
    __padding_40: [u8; 40],
    pub other_actor: TSoftObjectPtr<AActor>,
}
impl FSoftComponentReference {}
#[repr(C, align(8))]
pub struct FComponentReference {
    #[doc(hidden)]
    __padding_40: [u8; 40],
    pub other_actor: TWeakObjectPtr<AActor>,
}
impl FComponentReference {}
#[repr(C, align(8))]
pub struct FLatentActionInfo {
    __padding_end: [u8; 32],
}
impl FLatentActionInfo {}
#[repr(C, align(8))]
pub struct FTimerHandle {
    __padding_end: [u8; 8],
}
impl FTimerHandle {}
#[repr(C, align(4))]
pub struct FCollisionProfileName {
    pub name: FName,
}
impl FCollisionProfileName {}
#[repr(C, align(4))]
pub struct FGenericStruct {
    __padding_end: [u8; 4],
}
impl FGenericStruct {}
#[repr(C, align(8))]
pub struct FUserActivity {
    pub action_name: FString,
    __padding_end: [u8; 8],
}
impl FUserActivity {}
#[repr(C, align(4))]
pub struct FFloatAnimationAttribute {
    pub value: f32,
}
impl FFloatAnimationAttribute {}
#[repr(C, align(4))]
pub struct FIntegerAnimationAttribute {
    pub value: i32,
}
impl FIntegerAnimationAttribute {}
#[repr(C, align(8))]
pub struct FStringAnimationAttribute {
    pub value: FString,
}
impl FStringAnimationAttribute {}
#[repr(C, align(16))]
pub struct FTransformAnimationAttribute {
    pub value: crate::bindings::core_u_object::FTransform,
}
impl FTransformAnimationAttribute {}
#[repr(C, align(8))]
pub struct FVectorAnimationAttribute {
    pub value: crate::bindings::core_u_object::FVector,
}
impl FVectorAnimationAttribute {}
#[repr(C, align(16))]
pub struct FQuaternionAnimationAttribute {
    pub value: crate::bindings::core_u_object::FQuat,
}
impl FQuaternionAnimationAttribute {}
#[repr(C, align(8))]
pub struct FBodyInstanceAsyncPhysicsTickHandle {
    __padding_end: [u8; 8],
}
impl FBodyInstanceAsyncPhysicsTickHandle {}
#[repr(C, align(4))]
pub struct FWalkableSlopeOverride {
    pub walkable_slope_behavior: EWalkableSlopeBehavior,
    pub walkable_slope_angle: f32,
    __padding_end: [u8; 8],
}
impl FWalkableSlopeOverride {}
#[repr(C, align(8))]
pub struct FBodyInstance {
    #[doc(hidden)]
    __padding_18: [u8; 18],
    pub position_solver_iteration_count: u8,
    pub velocity_solver_iteration_count: u8,
    pub projection_solver_iteration_count: u8,
    #[doc(hidden)]
    __padding_57: [u8; 36],
    pub sleep_family: crate::bindings::physics_core::ESleepFamily,
    #[doc(hidden)]
    __padding_59: [u8; 1],
    pub flags_59: u8,
    pub flags_60: u8,
    #[doc(hidden)]
    __padding_62: [u8; 1],
    pub flags_62: u8,
    pub solver_async_delta_time: f32,
    #[doc(hidden)]
    __padding_184: [u8; 116],
    pub max_depenetration_velocity: f32,
    pub mass_in_kg_override: f32,
    #[doc(hidden)]
    __padding_200: [u8; 8],
    pub linear_damping: f32,
    pub angular_damping: f32,
    #[doc(hidden)]
    __padding_232: [u8; 24],
    pub com_nudge: crate::bindings::core_u_object::FVector,
    pub mass_scale: f32,
    pub gravity_group_index: u8,
    pub inertia_tensor_scale: crate::bindings::core_u_object::FVector,
    #[doc(hidden)]
    __padding_304: [u8; 16],
    pub walkable_slope_override: FWalkableSlopeOverride,
    pub phys_material_override: UPtr<crate::bindings::physics_core::UPhysicalMaterial>,
    pub max_angular_velocity: f32,
    pub custom_sleep_threshold_multiplier: f32,
    pub stabilization_threshold_multiplier: f32,
    __padding_end: [u8; 92],
}
impl FBodyInstance {
    pub fn verify_layout() {
        log::warn!(
            "{} = {} vs {}", "position_solver_iteration_count",
            std::mem::offset_of!(FBodyInstance, position_solver_iteration_count), 18usize
        );
        log::warn!(
            "{} = {} vs {}", "velocity_solver_iteration_count",
            std::mem::offset_of!(FBodyInstance, velocity_solver_iteration_count), 19usize
        );
        log::warn!(
            "{} = {} vs {}", "projection_solver_iteration_count",
            std::mem::offset_of!(FBodyInstance, projection_solver_iteration_count),
            20usize
        );
        log::warn!(
            "{} = {} vs {}", "sleep_family", std::mem::offset_of!(FBodyInstance,
            sleep_family), 57usize
        );
        log::warn!(
            "{} = {} vs {}", "solver_async_delta_time",
            std::mem::offset_of!(FBodyInstance, solver_async_delta_time), 64usize
        );
        log::warn!(
            "{} = {} vs {}", "max_depenetration_velocity",
            std::mem::offset_of!(FBodyInstance, max_depenetration_velocity), 184usize
        );
        log::warn!(
            "{} = {} vs {}", "mass_in_kg_override", std::mem::offset_of!(FBodyInstance,
            mass_in_kg_override), 188usize
        );
        log::warn!(
            "{} = {} vs {}", "linear_damping", std::mem::offset_of!(FBodyInstance,
            linear_damping), 200usize
        );
        log::warn!(
            "{} = {} vs {}", "angular_damping", std::mem::offset_of!(FBodyInstance,
            angular_damping), 204usize
        );
        log::warn!(
            "{} = {} vs {}", "com_nudge", std::mem::offset_of!(FBodyInstance, com_nudge),
            232usize
        );
        log::warn!(
            "{} = {} vs {}", "mass_scale", std::mem::offset_of!(FBodyInstance,
            mass_scale), 256usize
        );
        log::warn!(
            "{} = {} vs {}", "gravity_group_index", std::mem::offset_of!(FBodyInstance,
            gravity_group_index), 260usize
        );
        log::warn!(
            "{} = {} vs {}", "inertia_tensor_scale", std::mem::offset_of!(FBodyInstance,
            inertia_tensor_scale), 264usize
        );
        log::warn!(
            "{} = {} vs {}", "walkable_slope_override",
            std::mem::offset_of!(FBodyInstance, walkable_slope_override), 304usize
        );
        log::warn!(
            "{} = {} vs {}", "phys_material_override",
            std::mem::offset_of!(FBodyInstance, phys_material_override), 320usize
        );
        log::warn!(
            "{} = {} vs {}", "max_angular_velocity", std::mem::offset_of!(FBodyInstance,
            max_angular_velocity), 328usize
        );
        log::warn!(
            "{} = {} vs {}", "custom_sleep_threshold_multiplier",
            std::mem::offset_of!(FBodyInstance, custom_sleep_threshold_multiplier),
            332usize
        );
        log::warn!(
            "{} = {} vs {}", "stabilization_threshold_multiplier",
            std::mem::offset_of!(FBodyInstance, stabilization_threshold_multiplier),
            336usize
        );
    }
}
#[repr(C, align(1))]
pub struct FCollisionResponseContainer {
    pub world_static: ECollisionResponse,
    pub world_dynamic: ECollisionResponse,
    pub pawn: ECollisionResponse,
    pub visibility: ECollisionResponse,
    pub camera: ECollisionResponse,
    pub physics_body: ECollisionResponse,
    pub vehicle: ECollisionResponse,
    pub destructible: ECollisionResponse,
    pub engine_trace_channel1: ECollisionResponse,
    pub engine_trace_channel2: ECollisionResponse,
    pub engine_trace_channel3: ECollisionResponse,
    pub engine_trace_channel4: ECollisionResponse,
    pub engine_trace_channel5: ECollisionResponse,
    pub engine_trace_channel6: ECollisionResponse,
    pub game_trace_channel1: ECollisionResponse,
    pub game_trace_channel2: ECollisionResponse,
    pub game_trace_channel3: ECollisionResponse,
    pub game_trace_channel4: ECollisionResponse,
    pub game_trace_channel5: ECollisionResponse,
    pub game_trace_channel6: ECollisionResponse,
    pub game_trace_channel7: ECollisionResponse,
    pub game_trace_channel8: ECollisionResponse,
    pub game_trace_channel9: ECollisionResponse,
    pub game_trace_channel10: ECollisionResponse,
    pub game_trace_channel11: ECollisionResponse,
    pub game_trace_channel12: ECollisionResponse,
    pub game_trace_channel13: ECollisionResponse,
    pub game_trace_channel14: ECollisionResponse,
    pub game_trace_channel15: ECollisionResponse,
    pub game_trace_channel16: ECollisionResponse,
    pub game_trace_channel17: ECollisionResponse,
    pub game_trace_channel18: ECollisionResponse,
}
impl FCollisionResponseContainer {}
#[repr(C, align(1))]
pub struct FLightingChannels {
    pub flags_0: u8,
}
impl FLightingChannels {}
#[repr(C, align(16))]
pub struct FPostProcessSettings {
    pub flags_0: u8,
    pub flags_1: u8,
    pub flags_2: u8,
    pub flags_3: u8,
    pub flags_4: u8,
    pub flags_5: u8,
    pub flags_6: u8,
    pub flags_7: u8,
    pub flags_8: u8,
    pub flags_9: u8,
    pub flags_10: u8,
    pub flags_11: u8,
    pub flags_12: u8,
    pub flags_13: u8,
    pub flags_14: u8,
    pub flags_15: u8,
    pub flags_16: u8,
    #[doc(hidden)]
    __padding_20: [u8; 3],
    pub flags_20: u8,
    #[doc(hidden)]
    __padding_25: [u8; 4],
    pub flags_25: u8,
    pub flags_26: u8,
    pub flags_27: u8,
    pub flags_28: u8,
    pub flags_29: u8,
    pub flags_30: u8,
    pub flags_31: u8,
    pub flags_32: u8,
    pub flags_33: u8,
    pub flags_34: u8,
    pub flags_35: u8,
    pub flags_36: u8,
    #[doc(hidden)]
    __padding_40: [u8; 3],
    pub flags_40: u8,
    #[doc(hidden)]
    __padding_44: [u8; 3],
    pub flags_44: u8,
    #[doc(hidden)]
    __padding_48: [u8; 3],
    pub flags_48: u8,
    pub flags_49: u8,
    pub flags_50: u8,
    #[doc(hidden)]
    __padding_52: [u8; 1],
    pub flags_52: u8,
    pub bloom_method: EBloomMethod,
    pub auto_exposure_method: EAutoExposureMethod,
    #[doc(hidden)]
    __padding_56: [u8; 1],
    pub temperature_type: ETemperatureMethod,
    pub white_temp: f32,
    pub white_tint: f32,
    pub color_saturation: crate::bindings::core_u_object::FVector4,
    pub color_contrast: crate::bindings::core_u_object::FVector4,
    pub color_gamma: crate::bindings::core_u_object::FVector4,
    pub color_gain: crate::bindings::core_u_object::FVector4,
    pub color_offset: crate::bindings::core_u_object::FVector4,
    pub color_saturation_shadows: crate::bindings::core_u_object::FVector4,
    pub color_contrast_shadows: crate::bindings::core_u_object::FVector4,
    pub color_gamma_shadows: crate::bindings::core_u_object::FVector4,
    pub color_gain_shadows: crate::bindings::core_u_object::FVector4,
    pub color_offset_shadows: crate::bindings::core_u_object::FVector4,
    pub color_saturation_midtones: crate::bindings::core_u_object::FVector4,
    pub color_contrast_midtones: crate::bindings::core_u_object::FVector4,
    pub color_gamma_midtones: crate::bindings::core_u_object::FVector4,
    pub color_gain_midtones: crate::bindings::core_u_object::FVector4,
    pub color_offset_midtones: crate::bindings::core_u_object::FVector4,
    pub color_saturation_highlights: crate::bindings::core_u_object::FVector4,
    pub color_contrast_highlights: crate::bindings::core_u_object::FVector4,
    pub color_gamma_highlights: crate::bindings::core_u_object::FVector4,
    pub color_gain_highlights: crate::bindings::core_u_object::FVector4,
    pub color_offset_highlights: crate::bindings::core_u_object::FVector4,
    pub color_correction_highlights_min: f32,
    pub color_correction_highlights_max: f32,
    pub color_correction_shadows_max: f32,
    pub blue_correction: f32,
    pub expand_gamut: f32,
    pub tone_curve_amount: f32,
    pub film_slope: f32,
    pub film_toe: f32,
    pub film_shoulder: f32,
    pub film_black_clip: f32,
    pub film_white_clip: f32,
    pub scene_color_tint: crate::bindings::core_u_object::FLinearColor,
    pub scene_fringe_intensity: f32,
    pub chromatic_aberration_start_offset: f32,
    pub bloom_intensity: f32,
    pub bloom_gaussian_intensity: f32,
    pub bloom_threshold: f32,
    pub bloom_size_scale: f32,
    pub bloom1_size: f32,
    pub bloom2_size: f32,
    pub bloom3_size: f32,
    pub bloom4_size: f32,
    pub bloom5_size: f32,
    pub bloom6_size: f32,
    pub bloom1_tint: crate::bindings::core_u_object::FLinearColor,
    pub bloom2_tint: crate::bindings::core_u_object::FLinearColor,
    pub bloom3_tint: crate::bindings::core_u_object::FLinearColor,
    pub bloom4_tint: crate::bindings::core_u_object::FLinearColor,
    pub bloom5_tint: crate::bindings::core_u_object::FLinearColor,
    pub bloom6_tint: crate::bindings::core_u_object::FLinearColor,
    pub bloom_convolution_intensity: f32,
    pub bloom_convolution_scatter_dispersion: f32,
    pub bloom_convolution_size: f32,
    pub bloom_convolution_texture: UPtr<UTexture2D>,
    pub bloom_convolution_center_uv: crate::bindings::core_u_object::FVector2D,
    #[doc(hidden)]
    __padding_972: [u8; 12],
    pub bloom_convolution_pre_filter_min: f32,
    pub bloom_convolution_pre_filter_max: f32,
    pub bloom_convolution_pre_filter_mult: f32,
    pub bloom_convolution_buffer_scale: f32,
    pub bloom_dirt_mask: UPtr<UTexture>,
    pub bloom_dirt_mask_intensity: f32,
    pub bloom_dirt_mask_tint: crate::bindings::core_u_object::FLinearColor,
    pub dynamic_global_illumination_method: EDynamicGlobalIlluminationMethod,
    pub indirect_lighting_color: crate::bindings::core_u_object::FLinearColor,
    pub indirect_lighting_intensity: f32,
    pub lumen_ray_lighting_mode: ELumenRayLightingModeOverride,
    pub lumen_scene_lighting_quality: f32,
    pub lumen_scene_detail: f32,
    pub lumen_scene_view_distance: f32,
    pub lumen_scene_lighting_update_speed: f32,
    pub lumen_final_gather_quality: f32,
    pub lumen_final_gather_lighting_update_speed: f32,
    pub flags_1072: u8,
    pub lumen_max_trace_distance: f32,
    pub lumen_diffuse_color_boost: f32,
    pub lumen_skylight_leaking: f32,
    pub lumen_skylight_leaking_tint: crate::bindings::core_u_object::FLinearColor,
    pub lumen_full_skylight_leaking_distance: f32,
    pub lumen_surface_cache_resolution: f32,
    pub reflection_method: EReflectionMethod,
    pub lumen_reflection_quality: f32,
    pub flags_1120: u8,
    pub lumen_max_roughness_to_trace_reflections: f32,
    pub lumen_max_reflection_bounces: i32,
    pub lumen_max_refraction_bounces: i32,
    pub screen_space_reflection_intensity: f32,
    pub screen_space_reflection_quality: f32,
    pub screen_space_reflection_max_roughness: f32,
    pub flags_1148: u8,
    pub ambient_cubemap_tint: crate::bindings::core_u_object::FLinearColor,
    pub ambient_cubemap_intensity: f32,
    pub ambient_cubemap: UPtr<UTextureCube>,
    pub camera_shutter_speed: f32,
    pub camera_iso: f32,
    pub depth_of_field_fstop: f32,
    pub depth_of_field_min_fstop: f32,
    pub depth_of_field_blade_count: i32,
    pub auto_exposure_bias: f32,
    #[doc(hidden)]
    __padding_1216: [u8; 8],
    pub flags_1216: u8,
    pub auto_exposure_bias_curve: UPtr<UCurveFloat>,
    pub auto_exposure_meter_mask: UPtr<UTexture>,
    pub auto_exposure_low_percent: f32,
    pub auto_exposure_high_percent: f32,
    pub auto_exposure_min_brightness: f32,
    pub auto_exposure_max_brightness: f32,
    pub auto_exposure_speed_up: f32,
    pub auto_exposure_speed_down: f32,
    pub histogram_log_min: f32,
    pub histogram_log_max: f32,
    #[doc(hidden)]
    __padding_1276: [u8; 4],
    pub local_exposure_method: ELocalExposureMethod,
    #[doc(hidden)]
    __padding_1284: [u8; 4],
    pub local_exposure_highlight_contrast_scale: f32,
    pub local_exposure_shadow_contrast_scale: f32,
    pub local_exposure_highlight_contrast_curve: UPtr<UCurveFloat>,
    pub local_exposure_shadow_contrast_curve: UPtr<UCurveFloat>,
    pub local_exposure_highlight_threshold: f32,
    pub local_exposure_shadow_threshold: f32,
    pub local_exposure_detail_strength: f32,
    pub local_exposure_blurred_luminance_blend: f32,
    pub local_exposure_blurred_luminance_kernel_size_percent: f32,
    pub local_exposure_highlight_threshold_strength: f32,
    pub local_exposure_shadow_threshold_strength: f32,
    pub local_exposure_middle_grey_bias: f32,
    pub lens_flare_intensity: f32,
    pub lens_flare_tint: crate::bindings::core_u_object::FLinearColor,
    pub lens_flare_bokeh_size: f32,
    pub lens_flare_threshold: f32,
    pub lens_flare_bokeh_shape: UPtr<UTexture>,
    #[doc(hidden)]
    __padding_1512: [u8; 128],
    pub vignette_intensity: f32,
    pub sharpen: f32,
    #[doc(hidden)]
    __padding_1528: [u8; 8],
    pub film_grain_intensity: f32,
    pub film_grain_intensity_shadows: f32,
    pub film_grain_intensity_midtones: f32,
    pub film_grain_intensity_highlights: f32,
    pub film_grain_shadows_max: f32,
    pub film_grain_highlights_min: f32,
    pub film_grain_highlights_max: f32,
    pub film_grain_texel_size: f32,
    pub film_grain_texture: UPtr<UTexture2D>,
    pub ambient_occlusion_intensity: f32,
    pub ambient_occlusion_static_fraction: f32,
    pub ambient_occlusion_radius: f32,
    pub flags_1580: u8,
    pub ambient_occlusion_fade_distance: f32,
    pub ambient_occlusion_fade_radius: f32,
    #[doc(hidden)]
    __padding_1596: [u8; 4],
    pub ambient_occlusion_power: f32,
    pub ambient_occlusion_bias: f32,
    pub ambient_occlusion_quality: f32,
    pub ambient_occlusion_mip_blend: f32,
    pub ambient_occlusion_mip_scale: f32,
    pub ambient_occlusion_mip_threshold: f32,
    pub ambient_occlusion_temporal_blend_weight: f32,
    pub flags_1624: u8,
    pub ray_tracing_ao_samples_per_pixel: i32,
    pub ray_tracing_ao_intensity: f32,
    pub ray_tracing_ao_radius: f32,
    pub color_grading_intensity: f32,
    pub color_grading_lut: UPtr<UTexture>,
    pub depth_of_field_sensor_width: f32,
    pub depth_of_field_squeeze_factor: f32,
    pub depth_of_field_focal_distance: f32,
    pub depth_of_field_depth_blur_amount: f32,
    pub depth_of_field_depth_blur_radius: f32,
    pub flags_1676: u8,
    pub depth_of_field_petzval_bokeh: f32,
    pub depth_of_field_petzval_bokeh_falloff: f32,
    pub depth_of_field_petzval_exclusion_box_extents: crate::bindings::core_u_object::FVector2f,
    pub depth_of_field_petzval_exclusion_box_radius: f32,
    pub depth_of_field_aspect_ratio_scalar: f32,
    pub depth_of_field_barrel_radius: f32,
    pub depth_of_field_barrel_length: f32,
    #[doc(hidden)]
    __padding_1748: [u8; 36],
    pub depth_of_field_focal_region: f32,
    pub depth_of_field_near_transition_region: f32,
    pub depth_of_field_far_transition_region: f32,
    pub depth_of_field_scale: f32,
    pub depth_of_field_near_blur_size: f32,
    pub depth_of_field_far_blur_size: f32,
    pub depth_of_field_occlusion: f32,
    pub depth_of_field_sky_focus_distance: f32,
    pub depth_of_field_vignette_size: f32,
    pub motion_blur_amount: f32,
    pub motion_blur_max: f32,
    pub motion_blur_target_fps: i32,
    pub motion_blur_per_object_size: f32,
    #[doc(hidden)]
    __padding_1852: [u8; 52],
    pub translucency_type: ETranslucencyType,
    pub ray_tracing_translucency_max_roughness: f32,
    pub ray_tracing_translucency_refraction_rays: i32,
    pub ray_tracing_translucency_samples_per_pixel: i32,
    pub ray_tracing_translucency_max_primary_hit_events: i32,
    pub ray_tracing_translucency_max_secondary_hit_events: i32,
    pub ray_tracing_translucency_shadows: EReflectedAndRefractedRayTracedShadows,
    pub flags_1877: u8,
    pub path_tracing_max_bounces: i32,
    pub path_tracing_samples_per_pixel: i32,
    pub path_tracing_max_path_intensity: f32,
    pub flags_1892: u8,
    pub flags_1893: u8,
    #[doc(hidden)]
    __padding_1908: [u8; 12],
    pub user_flags: i32,
    pub weighted_blendables: FWeightedBlendables,
    __padding_end: [u8; 24],
}
impl FPostProcessSettings {}
#[repr(C, align(8))]
pub struct FWeightedBlendables {
    pub array: TArray<FWeightedBlendable>,
}
impl FWeightedBlendables {}
#[repr(C, align(8))]
pub struct FWeightedBlendable {
    pub weight: f32,
    pub object: UPtr<crate::bindings::core_u_object::UObject>,
}
impl FWeightedBlendable {}
#[repr(C, align(4))]
pub struct FMatteBoxFlag {
    pub pitch: f32,
    pub roll: f32,
    pub length: f32,
}
impl FMatteBoxFlag {}
#[repr(C, align(8))]
pub struct FTableRowBase {
    __padding_end: [u8; 8],
}
impl FTableRowBase {}
#[repr(C, align(8))]
pub struct FRuntimeFloatCurve {
    __padding_end: [u8; 136],
}
impl FRuntimeFloatCurve {}
#[repr(C, align(4))]
pub struct FRichCurveKey {
    pub interp_mode: ERichCurveInterpMode,
    pub tangent_mode: ERichCurveTangentMode,
    pub tangent_weight_mode: ERichCurveTangentWeightMode,
    pub time: f32,
    pub value: f32,
    __padding_end: [u8; 16],
}
impl FRichCurveKey {}
#[repr(C, align(8))]
pub struct FEdGraphPinType {
    __padding_end: [u8; 112],
}
impl FEdGraphPinType {}
#[repr(C, align(8))]
pub struct FPoseLinkBase {
    __padding_end: [u8; 24],
}
impl FPoseLinkBase {}
#[repr(C, align(8))]
pub struct FPoseLink {
    __padding_end: [u8; 24],
}
impl FPoseLink {}
#[repr(C, align(8))]
pub struct FAnimNode_Root {
    #[doc(hidden)]
    __padding_136: [u8; 136],
    pub result: FPoseLink,
    __padding_end: [u8; 40],
}
impl FAnimNode_Root {}
#[repr(C, align(4))]
pub struct FInputScaleBiasClamp {
    pub b_map_range: bool,
    pub b_clamp_result: bool,
    pub b_interp_result: bool,
    pub in_range: FInputRange,
    pub out_range: FInputRange,
    pub scale: f32,
    pub bias: f32,
    pub clamp_min: f32,
    pub clamp_max: f32,
    pub interp_speed_increasing: f32,
    pub interp_speed_decreasing: f32,
    __padding_end: [u8; 4],
}
impl FInputScaleBiasClamp {}
#[repr(C, align(4))]
pub struct FInputRange {
    pub min: f32,
    pub max: f32,
}
impl FInputRange {}
#[repr(C, align(8))]
pub struct FInputAlphaBoolBlend {
    pub blend_in_time: f32,
    pub blend_out_time: f32,
    pub blend_option: EAlphaBlendOption,
    pub custom_curve: UPtr<UCurveFloat>,
    __padding_end: [u8; 48],
}
impl FInputAlphaBoolBlend {}
#[repr(C, align(8))]
pub struct FAlphaBlend {
    __padding_end: [u8; 48],
}
impl FAlphaBlend {}
#[repr(C, align(4))]
pub struct FInputScaleBias {
    pub scale: f32,
    pub bias: f32,
}
impl FInputScaleBias {}
#[repr(C, align(8))]
pub struct FComponentSpacePoseLink {
    __padding_end: [u8; 24],
}
impl FComponentSpacePoseLink {}
#[repr(C, align(8))]
pub struct FAnimNodeReference {
    __padding_end: [u8; 16],
}
impl FAnimNodeReference {}
#[repr(C, align(8))]
pub struct FAnimNode_AssetPlayerRelevancyBase {
    __padding_end: [u8; 136],
}
impl FAnimNode_AssetPlayerRelevancyBase {}
#[repr(C, align(8))]
pub struct FAnimNode_AssetPlayerBase {
    #[doc(hidden)]
    __padding_160: [u8; 160],
    pub blend_weight: f32,
    pub internal_time_accumulator: f32,
    __padding_end: [u8; 16],
}
impl FAnimNode_AssetPlayerBase {}
#[repr(C, align(4))]
pub struct FPerBoneBlendWeight {
    __padding_end: [u8; 8],
}
impl FPerBoneBlendWeight {}
#[repr(C, align(8))]
pub struct FPoseSnapshot {
    pub local_transforms: TArray<crate::bindings::core_u_object::FTransform>,
    pub bone_names: TArray<FName>,
    pub skeletal_mesh_name: FName,
    pub snapshot_name: FName,
    pub b_is_valid: bool,
    __padding_end: [u8; 7],
}
impl FPoseSnapshot {}
#[repr(C, align(4))]
pub struct FMaterialParameterInfo {
    pub name: FName,
    pub association: EMaterialParameterAssociation,
    pub index: i32,
}
impl FMaterialParameterInfo {}
#[repr(C, align(8))]
pub struct FParameterChannelNames {
    __padding_end: [u8; 64],
}
impl FParameterChannelNames {}
#[repr(C, align(4))]
pub struct FActorDataLayer {
    pub name: FName,
}
impl FActorDataLayer {}
#[repr(C, align(8))]
pub struct FVector_NetQuantize100 {
    __padding_end: [u8; 24],
}
impl FVector_NetQuantize100 {}
#[repr(C, align(8))]
pub struct FPerQualityLevelInt {
    #[doc(hidden)]
    __padding_40: [u8; 40],
    pub default: i32,
    __padding_end: [u8; 84],
}
impl FPerQualityLevelInt {}
#[repr(C, align(8))]
pub struct FPerQualityLevelFloat {
    #[doc(hidden)]
    __padding_40: [u8; 40],
    pub default: f32,
    __padding_end: [u8; 84],
}
impl FPerQualityLevelFloat {}
#[repr(C, align(4))]
pub struct FNamedFloat {
    pub value: f32,
    pub name: FName,
}
impl FNamedFloat {}
#[repr(C, align(8))]
pub struct FNamedVector {
    pub value: crate::bindings::core_u_object::FVector,
    pub name: FName,
    __padding_end: [u8; 4],
}
impl FNamedVector {}
#[repr(C, align(4))]
pub struct FNamedColor {
    pub value: crate::bindings::core_u_object::FColor,
    pub name: FName,
}
impl FNamedColor {}
#[repr(C, align(16))]
pub struct FNamedTransform {
    pub value: crate::bindings::core_u_object::FTransform,
    pub name: FName,
    __padding_end: [u8; 4],
}
impl FNamedTransform {}
#[repr(C, align(8))]
pub struct FLocalSpacePose {
    pub transforms: TArray<crate::bindings::core_u_object::FTransform>,
    pub names: TArray<FName>,
}
impl FLocalSpacePose {}
#[repr(C, align(8))]
pub struct FComponentSpacePose {
    pub transforms: TArray<crate::bindings::core_u_object::FTransform>,
    pub names: TArray<FName>,
}
impl FComponentSpacePose {}
#[repr(C, align(1))]
pub struct FEmptyPayload {
    __padding_end: [u8; 1],
}
impl FEmptyPayload {}
#[repr(C, align(8))]
pub struct FBracketPayload {
    pub description: FString,
}
impl FBracketPayload {}
#[repr(C, align(4))]
pub struct FAnimationTrackPayload {
    pub name: FName,
}
impl FAnimationTrackPayload {}
#[repr(C, align(4))]
pub struct FAnimationTrackAddedPayload {
    #[doc(hidden)]
    __padding_12: [u8; 12],
    pub track_index: i32,
}
impl FAnimationTrackAddedPayload {}
#[repr(C, align(4))]
pub struct FSequenceLengthChangedPayload {
    pub previous_length: f32,
    pub t0: f32,
    pub t1: f32,
    pub previous_number_of_frames: crate::bindings::core_u_object::FFrameNumber,
    pub frame0: crate::bindings::core_u_object::FFrameNumber,
    pub frame1: crate::bindings::core_u_object::FFrameNumber,
}
impl FSequenceLengthChangedPayload {}
#[repr(C, align(4))]
pub struct FFrameRateChangedPayload {
    pub previous_frame_rate: crate::bindings::core_u_object::FFrameRate,
}
impl FFrameRateChangedPayload {}
#[repr(C, align(4))]
pub struct FCurvePayload {
    pub identifier: FAnimationCurveIdentifier,
}
impl FCurvePayload {}
#[repr(C, align(4))]
pub struct FAnimationCurveIdentifier {
    __padding_end: [u8; 32],
}
impl FAnimationCurveIdentifier {}
#[repr(C, align(4))]
pub struct FCurveScaledPayload {
    #[doc(hidden)]
    __padding_32: [u8; 32],
    pub factor: f32,
    pub origin: f32,
}
impl FCurveScaledPayload {}
#[repr(C, align(4))]
pub struct FCurveRenamedPayload {
    #[doc(hidden)]
    __padding_32: [u8; 32],
    pub new_identifier: FAnimationCurveIdentifier,
}
impl FCurveRenamedPayload {}
#[repr(C, align(4))]
pub struct FCurveFlagsChangedPayload {
    #[doc(hidden)]
    __padding_32: [u8; 32],
    pub old_flags: i32,
}
impl FCurveFlagsChangedPayload {}
#[repr(C, align(8))]
pub struct FAttributePayload {
    pub identifier: FAnimationAttributeIdentifier,
}
impl FAttributePayload {}
#[repr(C, align(8))]
pub struct FAnimationAttributeIdentifier {
    pub name: FName,
    pub bone_name: FName,
    pub bone_index: i32,
    pub script_struct: UPtr<crate::bindings::core_u_object::UScriptStruct>,
    pub script_struct_path: crate::bindings::core_u_object::FSoftObjectPath,
}
impl FAnimationAttributeIdentifier {}
#[repr(C, align(8))]
pub struct FAnimNodeConstantData {
    __padding_end: [u8; 24],
}
impl FAnimNodeConstantData {}
#[repr(C, align(16))]
pub struct FNonBlendableQuaternionAnimationAttribute {
    __padding_end: [u8; 32],
}
impl FNonBlendableQuaternionAnimationAttribute {}
#[repr(C, align(8))]
pub struct FNonBlendableVectorAnimationAttribute {
    __padding_end: [u8; 24],
}
impl FNonBlendableVectorAnimationAttribute {}
#[repr(C, align(16))]
pub struct FNonBlendableTransformAnimationAttribute {
    __padding_end: [u8; 96],
}
impl FNonBlendableTransformAnimationAttribute {}
#[repr(C, align(4))]
pub struct FNonBlendableFloatAnimationAttribute {
    __padding_end: [u8; 4],
}
impl FNonBlendableFloatAnimationAttribute {}
#[repr(C, align(4))]
pub struct FNonBlendableIntegerAnimationAttribute {
    __padding_end: [u8; 4],
}
impl FNonBlendableIntegerAnimationAttribute {}
#[repr(C, align(16))]
pub struct FMinimalViewInfo {
    pub location: crate::bindings::core_u_object::FVector,
    pub rotation: crate::bindings::core_u_object::FRotator,
    pub fov: f32,
    #[doc(hidden)]
    __padding_56: [u8; 4],
    pub first_person_fov: f32,
    pub first_person_scale: f32,
    pub ortho_width: f32,
    pub b_auto_calculate_ortho_planes: bool,
    pub auto_plane_shift: f32,
    pub b_update_ortho_planes: bool,
    pub b_use_camera_height_as_view_target: bool,
    pub ortho_near_clip_plane: f32,
    pub ortho_far_clip_plane: f32,
    pub perspective_near_clip_plane: f32,
    pub aspect_ratio: f32,
    #[doc(hidden)]
    __padding_104: [u8; 8],
    pub flags_104: u8,
    #[doc(hidden)]
    __padding_108: [u8; 3],
    pub projection_mode: ECameraProjectionMode,
    pub post_process_blend_weight: f32,
    pub post_process_settings: FPostProcessSettings,
    pub off_center_projection_offset: crate::bindings::core_u_object::FVector2D,
    #[doc(hidden)]
    __padding_2208: [u8; 112],
    pub overscan_resolution_fraction: f32,
    pub crop_fraction: f32,
    pub asymmetric_crop_fraction: crate::bindings::core_u_object::FVector4f,
    __padding_end: [u8; 64],
}
impl FMinimalViewInfo {}
#[repr(C, align(4))]
pub struct FExponentialHeightFogData {
    pub fog_density: f32,
    pub fog_height_falloff: f32,
    pub fog_height_offset: f32,
}
impl FExponentialHeightFogData {}
#[repr(C, align(8))]
pub struct FBaseAttenuationSettings {
    #[doc(hidden)]
    __padding_8: [u8; 8],
    pub distance_algorithm: EAttenuationDistanceModel,
    pub attenuation_shape: EAttenuationShape,
    pub falloff_mode: ENaturalSoundFalloffMode,
    pub d_b_attenuation_at_max: f32,
    pub attenuation_shape_extents: crate::bindings::core_u_object::FVector,
    pub cone_offset: f32,
    pub falloff_distance: f32,
    pub cone_sphere_radius: f32,
    pub cone_sphere_falloff_distance: f32,
    pub custom_attenuation_curve: FRuntimeFloatCurve,
}
impl FBaseAttenuationSettings {}
#[repr(C, align(8))]
pub struct FForceFeedbackAttenuationSettings {
    __padding_end: [u8; 192],
}
impl FForceFeedbackAttenuationSettings {}
#[repr(C, align(8))]
pub struct FPredictProjectilePathParams {
    pub start_location: crate::bindings::core_u_object::FVector,
    pub launch_velocity: crate::bindings::core_u_object::FVector,
    pub b_trace_with_collision: bool,
    pub projectile_radius: f32,
    pub max_sim_time: f32,
    pub b_trace_with_channel: bool,
    pub trace_channel: ECollisionChannel,
    pub object_types: TArray<EObjectTypeQuery>,
    pub actors_to_ignore: TArray<UPtr<AActor>>,
    pub sim_frequency: f32,
    pub override_gravity_z: f32,
    pub draw_debug_type: EDrawDebugTrace,
    pub draw_debug_time: f32,
    pub b_trace_complex: bool,
    __padding_end: [u8; 7],
}
impl FPredictProjectilePathParams {}
#[repr(C, align(8))]
pub struct FPredictProjectilePathPointData {
    pub location: crate::bindings::core_u_object::FVector,
    pub velocity: crate::bindings::core_u_object::FVector,
    pub time: f32,
    __padding_end: [u8; 4],
}
impl FPredictProjectilePathPointData {}
#[repr(C, align(8))]
pub struct FPredictProjectilePathResult {
    pub path_data: TArray<FPredictProjectilePathPointData>,
    pub last_trace_destination: FPredictProjectilePathPointData,
    pub hit_result: FHitResult,
}
impl FPredictProjectilePathResult {}
#[repr(C, align(4))]
pub struct FPrimitiveInstanceId {
    __padding_end: [u8; 4],
}
impl FPrimitiveInstanceId {}
#[repr(C, align(4))]
pub struct FDisplacementFadeRange {
    pub start_size_pixels: f32,
    pub end_size_pixels: f32,
}
impl FDisplacementFadeRange {}
#[repr(C, align(4))]
pub struct FDisplacementScaling {
    pub magnitude: f32,
    pub center: f32,
}
impl FDisplacementScaling {}
#[repr(C, align(4))]
pub struct FMeshUVChannelInfo {
    __padding_end: [u8; 20],
}
impl FMeshUVChannelInfo {}
#[repr(C, align(8))]
pub struct FMLLevelSetModelAndBonesBinningInfo {
    #[doc(hidden)]
    __padding_8: [u8; 8],
    pub parent_bone_name: FString,
    pub active_bone_names: FString,
    pub ml_model_inference_info_data_table_path: FString,
    pub ml_model_inference_info_data_table_index: FString,
    pub ml_model_inference_for_incorrect_zone_info_data_table_index: FString,
    pub number_of_rotation_components_per_bone: TArray<i32>,
    pub rotation_component_indexes: TArray<i32>,
    pub signed_distance_scaling: f64,
    pub debug_grid_resolution: TArray<i32>,
    pub reference_bone_rotations: TArray<f64>,
    pub reference_bone_translations: TArray<f64>,
    pub training_grid_origin: TArray<f32>,
    pub training_grid_axis_x: TArray<f32>,
    pub training_grid_axis_y: TArray<f32>,
    pub training_grid_axis_z: TArray<f32>,
}
impl FMLLevelSetModelAndBonesBinningInfo {}
#[repr(C, align(8))]
pub struct FMLLevelSetModelInferenceInfo {
    #[doc(hidden)]
    __padding_8: [u8; 8],
    pub nne_model_path: FString,
    pub model_architecture_activation_node_sizes: TArray<i32>,
    pub ml_model_weights: FString,
}
impl FMLLevelSetModelInferenceInfo {}
#[repr(C, align(4))]
pub struct FNaniteAssemblyBoneInfluence {
    pub bone_index: i32,
    pub bone_weight: f32,
}
impl FNaniteAssemblyBoneInfluence {}
#[repr(C, align(4))]
pub struct FNavAvoidanceMask {
    pub flags_0: u8,
    pub flags_1: u8,
    pub flags_2: u8,
    pub flags_3: u8,
}
impl FNavAvoidanceMask {}
#[repr(C, align(8))]
pub struct FNavigationLinkBase {
    #[doc(hidden)]
    __padding_56: [u8; 56],
    pub direction: ENavLinkDirection,
    __padding_end: [u8; 23],
}
impl FNavigationLinkBase {}
#[repr(C, align(8))]
pub struct FNavigationLink {
    #[doc(hidden)]
    __padding_80: [u8; 80],
    pub left: crate::bindings::core_u_object::FVector,
    pub right: crate::bindings::core_u_object::FVector,
}
impl FNavigationLink {}
#[repr(C, align(8))]
pub struct FNavigationSegmentLink {
    __padding_end: [u8; 176],
}
impl FNavigationSegmentLink {}
#[repr(C, align(8))]
pub struct FVector_NetQuantize10 {
    __padding_end: [u8; 24],
}
impl FVector_NetQuantize10 {}
#[repr(C, align(16))]
pub struct FSceneViewExtensionIsActiveFunctor {
    __padding_end: [u8; 64],
}
impl FSceneViewExtensionIsActiveFunctor {}
#[repr(C, align(8))]
pub struct FSoftWorldReference {
    pub world_asset: TSoftObjectPtr<UWorld>,
}
impl FSoftWorldReference {}
#[repr(C, align(8))]
pub struct FSourceEffectChainEntry {
    pub preset: UPtr<USoundEffectSourcePreset>,
    pub flags_8: u8,
    __padding_end: [u8; 7],
}
impl FSourceEffectChainEntry {}
#[repr(C, align(8))]
pub struct FSoundSourceBusSendInfo {
    #[doc(hidden)]
    __padding_28: [u8; 28],
    pub min_send_level: f32,
    pub max_send_level: f32,
    pub min_send_distance: f32,
    pub max_send_distance: f32,
    pub custom_send_level_curve: FRuntimeFloatCurve,
}
impl FSoundSourceBusSendInfo {}
#[repr(C, align(8))]
pub struct FAttributeCurve {
    __padding_end: [u8; 184],
}
impl FAttributeCurve {}
#[repr(C, align(4))]
pub struct FQuartzPulseOverrideStep {
    pub number_of_pulses: i32,
    pub pulse_duration: EQuartzCommandQuantization,
    __padding_end: [u8; 3],
}
impl FQuartzPulseOverrideStep {}
#[repr(C, align(8))]
pub struct FQuartzTimeSignature {
    pub num_beats: i32,
    pub beat_type: EQuartzTimeSignatureQuantization,
    pub optional_pulse_override: TArray<FQuartzPulseOverrideStep>,
}
impl FQuartzTimeSignature {}
#[repr(C, align(4))]
pub struct FQuartzTransportTimeStamp {
    pub bars: i32,
    pub beat: i32,
    pub beat_fraction: f32,
    pub seconds: f32,
}
impl FQuartzTransportTimeStamp {}
#[repr(C, align(8))]
pub struct FQuartzClockSettings {
    pub time_signature: FQuartzTimeSignature,
    pub b_ignore_level_change: bool,
    __padding_end: [u8; 7],
}
impl FQuartzClockSettings {}
#[repr(C, align(8))]
pub struct FQuartzQuantizationBoundary {
    pub quantization: EQuartzCommandQuantization,
    pub multiplier: f32,
    pub counting_reference_point: EQuarztQuantizationReference,
    pub b_fire_on_clock_start: bool,
    pub b_cancel_command_if_clock_is_not_running: bool,
    pub b_reset_clock_on_queued: bool,
    pub b_resume_clock_on_queued: bool,
    __padding_end: [u8; 19],
}
impl FQuartzQuantizationBoundary {}
#[repr(C, align(1))]
pub struct FMovementProperties {
    pub flags_0: u8,
}
impl FMovementProperties {}
#[repr(C, align(4))]
pub struct FNavMovementProperties {
    __padding_end: [u8; 8],
}
impl FNavMovementProperties {}
#[repr(C, align(8))]
pub struct FNavAgentProperties {
    #[doc(hidden)]
    __padding_4: [u8; 4],
    pub agent_radius: f32,
    pub agent_height: f32,
    pub agent_step_height: f32,
    pub nav_walking_search_height_scale: f32,
    pub preferred_nav_data: crate::bindings::core_u_object::FSoftClassPath,
}
impl FNavAgentProperties {}
#[repr(C, align(8))]
pub struct FNavDataConfig {
    #[doc(hidden)]
    __padding_64: [u8; 64],
    pub name: FName,
    pub color: crate::bindings::core_u_object::FColor,
    pub default_query_extent: crate::bindings::core_u_object::FVector,
    pub nav_data_class: TSoftObjectPtr<crate::bindings::core_u_object::UClass>,
}
impl FNavDataConfig {}
#[repr(C, align(8))]
pub struct FAlphaBlendArgs {
    pub custom_curve: UPtr<UCurveFloat>,
    pub blend_time: f32,
    pub blend_option: EAlphaBlendOption,
    __padding_end: [u8; 3],
}
impl FAlphaBlendArgs {}
#[repr(C, align(8))]
pub struct FBlendSampleData {
    __padding_end: [u8; 96],
}
impl FBlendSampleData {}
#[repr(C, align(4))]
pub struct FMarkerSyncAnimPosition {
    pub previous_marker_name: FName,
    pub next_marker_name: FName,
    pub position_between_markers: f32,
}
impl FMarkerSyncAnimPosition {}
#[repr(C, align(8))]
pub struct FAnimBankSequence {
    pub sequence: UPtr<UAnimSequence>,
    pub flags_8: u8,
    pub position: f32,
    pub play_rate: f32,
    pub bounds_scale: f32,
}
impl FAnimBankSequence {}
#[repr(C, align(8))]
pub struct FAnimBankItem {
    pub bank_asset: UPtr<UAnimBank>,
    pub sequence_index: i32,
    __padding_end: [u8; 4],
}
impl FAnimBankItem {}
#[repr(C, align(8))]
pub struct FSoftAnimBankItem {
    pub bank_asset: TSoftObjectPtr<UAnimBank>,
    pub sequence_index: i32,
    __padding_end: [u8; 4],
}
impl FSoftAnimBankItem {}
#[repr(C, align(8))]
pub struct FAnimCurveBase {
    __padding_end: [u8; 80],
}
impl FAnimCurveBase {}
#[repr(C, align(8))]
pub struct FFloatCurve {
    __padding_end: [u8; 208],
}
impl FFloatCurve {}
#[repr(C, align(8))]
pub struct FVectorCurve {
    __padding_end: [u8; 464],
}
impl FVectorCurve {}
#[repr(C, align(8))]
pub struct FTransformCurve {
    __padding_end: [u8; 1472],
}
impl FTransformCurve {}
#[repr(C, align(4))]
pub struct FCachedFloatCurve {
    pub curve_name: FName,
}
impl FCachedFloatCurve {}
#[repr(C, align(8))]
pub struct FBoneAnimationTrack {
    pub internal_track_data: FRawAnimSequenceTrack,
    pub bone_tree_index: i32,
    pub name: FName,
}
impl FBoneAnimationTrack {}
#[repr(C, align(8))]
pub struct FRawAnimSequenceTrack {
    __padding_end: [u8; 48],
}
impl FRawAnimSequenceTrack {}
#[repr(C, align(8))]
pub struct FAnimationCurveData {
    pub float_curves: TArray<FFloatCurve>,
    pub transform_curves: TArray<FTransformCurve>,
}
impl FAnimationCurveData {}
#[repr(C, align(8))]
pub struct FAnimatedBoneAttribute {
    pub identifier: FAnimationAttributeIdentifier,
    pub curve: FAttributeCurve,
}
impl FAnimatedBoneAttribute {}
#[repr(C, align(8))]
pub struct FAnimExecutionContext {
    __padding_end: [u8; 16],
}
impl FAnimExecutionContext {}
#[repr(C, align(8))]
pub struct FAnimInitializationContext {
    __padding_end: [u8; 16],
}
impl FAnimInitializationContext {}
#[repr(C, align(8))]
pub struct FAnimUpdateContext {
    __padding_end: [u8; 16],
}
impl FAnimUpdateContext {}
#[repr(C, align(8))]
pub struct FAnimPoseContext {
    __padding_end: [u8; 16],
}
impl FAnimPoseContext {}
#[repr(C, align(8))]
pub struct FAnimComponentSpacePoseContext {
    __padding_end: [u8; 16],
}
impl FAnimComponentSpacePoseContext {}
#[repr(C, align(8))]
pub struct FMontageBlendSettings {
    pub blend_profile: UPtr<UBlendProfile>,
    pub blend: FAlphaBlendArgs,
    pub blend_mode: EMontageBlendMode,
    __padding_end: [u8; 7],
}
impl FMontageBlendSettings {}
#[repr(C, align(8))]
pub struct FAnimNotifyEvent {
    #[doc(hidden)]
    __padding_60: [u8; 60],
    pub trigger_weight_threshold: f32,
    pub notify_name: FName,
    pub notify: UPtr<UAnimNotify>,
    pub notify_state_class: UPtr<UAnimNotifyState>,
    #[doc(hidden)]
    __padding_153: [u8; 57],
    pub montage_tick_type: EMontageNotifyTickType,
    pub notify_trigger_chance: f32,
    pub notify_filter_type: ENotifyFilterType,
    pub notify_filter_lod: i32,
    pub b_can_be_filtered_via_request: bool,
    pub b_trigger_on_dedicated_server: bool,
    pub b_trigger_on_follower: bool,
    __padding_end: [u8; 53],
}
impl FAnimNotifyEvent {}
#[repr(C, align(8))]
pub struct FAnimNodeData {
    __padding_end: [u8; 40],
}
impl FAnimNodeData {}
#[repr(C, align(8))]
pub struct FAnimNode_ConvertComponentToLocalSpace {
    #[doc(hidden)]
    __padding_136: [u8; 136],
    pub component_pose: FComponentSpacePoseLink,
}
impl FAnimNode_ConvertComponentToLocalSpace {}
#[repr(C, align(8))]
pub struct FAnimNode_ConvertLocalToComponentSpace {
    #[doc(hidden)]
    __padding_136: [u8; 136],
    pub local_pose: FPoseLink,
}
impl FAnimNode_ConvertLocalToComponentSpace {}
#[repr(C, align(8))]
pub struct FAnimNode_ApplyMeshSpaceAdditive {
    #[doc(hidden)]
    __padding_136: [u8; 136],
    pub base: FPoseLink,
    pub additive: FPoseLink,
    pub flags_184: u8,
    pub alpha_input_type: EAnimAlphaInputType,
    pub alpha: f32,
    pub flags_192: u8,
    pub alpha_bool_blend: FInputAlphaBoolBlend,
    pub alpha_curve_name: FName,
    pub alpha_scale_bias: FInputScaleBias,
    pub alpha_scale_bias_clamp: FInputScaleBiasClamp,
    pub lod_threshold: i32,
    __padding_end: [u8; 8],
}
impl FAnimNode_ApplyMeshSpaceAdditive {}
#[repr(C, align(16))]
pub struct FAnimNode_DeadBlending {
    __padding_end: [u8; 1872],
}
impl FAnimNode_DeadBlending {}
#[repr(C, align(16))]
pub struct FAnimNode_Inertialization {
    __padding_end: [u8; 1520],
}
impl FAnimNode_Inertialization {}
#[repr(C, align(8))]
pub struct FAnimNode_LinkedAnimGraph {
    __padding_end: [u8; 328],
}
impl FAnimNode_LinkedAnimGraph {}
#[repr(C, align(8))]
pub struct FAnimNode_LinkedAnimLayer {
    __padding_end: [u8; 384],
}
impl FAnimNode_LinkedAnimLayer {}
#[repr(C, align(8))]
pub struct FAnimNode_SaveCachedPose {
    #[doc(hidden)]
    __padding_136: [u8; 136],
    pub pose: FPoseLink,
    __padding_end: [u8; 96],
}
impl FAnimNode_SaveCachedPose {}
#[repr(C, align(8))]
pub struct FAnimNode_SequencePlayerBase {
    __padding_end: [u8; 192],
}
impl FAnimNode_SequencePlayerBase {}
#[repr(C, align(4))]
pub struct FInputScaleBiasClampState {
    __padding_end: [u8; 8],
}
impl FInputScaleBiasClampState {}
#[repr(C, align(8))]
pub struct FAnimNode_SequencePlayer {
    __padding_end: [u8; 328],
}
impl FAnimNode_SequencePlayer {}
#[repr(C, align(4))]
pub struct FInputScaleBiasClampConstants {
    pub b_map_range: bool,
    pub b_clamp_result: bool,
    pub b_interp_result: bool,
    pub in_range: FInputRange,
    pub out_range: FInputRange,
    pub scale: f32,
    pub bias: f32,
    pub clamp_min: f32,
    pub clamp_max: f32,
    pub interp_speed_increasing: f32,
    pub interp_speed_decreasing: f32,
}
impl FInputScaleBiasClampConstants {}
#[repr(C, align(8))]
pub struct FAnimNode_SequencePlayer_Standalone {
    __padding_end: [u8; 280],
}
impl FAnimNode_SequencePlayer_Standalone {}
#[repr(C, align(8))]
pub struct FAnimNode_StateResult {
    __padding_end: [u8; 368],
}
impl FAnimNode_StateResult {}
#[repr(C, align(8))]
pub struct FAnimNode_TransitionPoseEvaluator {
    #[doc(hidden)]
    __padding_256: [u8; 256],
    pub frames_to_cache_pose: i32,
    #[doc(hidden)]
    __padding_264: [u8; 4],
    pub data_source: EEvaluatorDataSource,
    pub evaluator_mode: EEvaluatorMode,
    __padding_end: [u8; 22],
}
impl FAnimNode_TransitionPoseEvaluator {}
#[repr(C, align(8))]
pub struct FAnimNode_TransitionResult {
    #[doc(hidden)]
    __padding_136: [u8; 136],
    pub b_can_enter_transition: bool,
    __padding_end: [u8; 31],
}
impl FAnimNode_TransitionResult {}
#[repr(C, align(8))]
pub struct FAnimNotifyEventReference {
    __padding_end: [u8; 48],
}
impl FAnimNotifyEventReference {}
#[repr(C, align(8))]
pub struct FAnimNode_SingleNode {
    #[doc(hidden)]
    __padding_136: [u8; 136],
    pub source_pose: FPoseLink,
    __padding_end: [u8; 24],
}
impl FAnimNode_SingleNode {}
#[repr(C, align(8))]
pub struct FPerBoneBlendWeights {
    __padding_end: [u8; 16],
}
impl FPerBoneBlendWeights {}
#[repr(C, align(4))]
pub struct FAnimSyncMarker {
    pub marker_name: FName,
    pub time: f32,
    __padding_end: [u8; 20],
}
impl FAnimSyncMarker {}
#[repr(C, align(4))]
pub struct FCachedAnimStateData {
    pub state_machine_name: FName,
    pub state_name: FName,
    __padding_end: [u8; 12],
}
impl FCachedAnimStateData {}
#[repr(C, align(8))]
pub struct FCachedAnimStateArray {
    pub states: TArray<FCachedAnimStateData>,
    __padding_end: [u8; 8],
}
impl FCachedAnimStateArray {}
#[repr(C, align(4))]
pub struct FCachedAnimAssetPlayerData {
    pub state_machine_name: FName,
    pub state_name: FName,
    __padding_end: [u8; 8],
}
impl FCachedAnimAssetPlayerData {}
#[repr(C, align(4))]
pub struct FCachedAnimRelevancyData {
    pub state_machine_name: FName,
    pub state_name: FName,
    __padding_end: [u8; 12],
}
impl FCachedAnimRelevancyData {}
#[repr(C, align(4))]
pub struct FCachedAnimTransitionData {
    pub state_machine_name: FName,
    pub from_state_name: FName,
    pub to_state_name: FName,
    __padding_end: [u8; 12],
}
impl FCachedAnimTransitionData {}
#[repr(C, align(4))]
pub struct FNamedCurveValue {
    pub name: FName,
    pub value: f32,
}
impl FNamedCurveValue {}
#[repr(C, align(4))]
pub struct FInputClampConstants {
    pub b_clamp_result: bool,
    pub b_interp_result: bool,
    pub clamp_min: f32,
    pub clamp_max: f32,
    pub interp_speed_increasing: f32,
    pub interp_speed_decreasing: f32,
}
impl FInputClampConstants {}
#[repr(C, align(4))]
pub struct FInputClampState {
    __padding_end: [u8; 8],
}
impl FInputClampState {}
#[repr(C, align(8))]
pub struct FMirrorTableRow {
    #[doc(hidden)]
    __padding_8: [u8; 8],
    pub name: FName,
    pub mirrored_name: FName,
    pub mirror_entry_type: EMirrorRowType,
    __padding_end: [u8; 7],
}
impl FMirrorTableRow {}
#[repr(C, align(16))]
pub struct FTrajectorySample {
    pub accumulated_seconds: f32,
    pub transform: crate::bindings::core_u_object::FTransform,
    pub linear_velocity: crate::bindings::core_u_object::FVector,
    __padding_end: [u8; 8],
}
impl FTrajectorySample {}
#[repr(C, align(16))]
pub struct FTransformTrajectorySample {
    pub facing: crate::bindings::core_u_object::FQuat,
    pub position: crate::bindings::core_u_object::FVector,
    pub time_in_seconds: f32,
    __padding_end: [u8; 4],
}
impl FTransformTrajectorySample {}
#[repr(C, align(8))]
pub struct FTransformTrajectory {
    pub samples: TArray<FTransformTrajectorySample>,
}
impl FTransformTrajectory {}
#[repr(C, align(4))]
pub struct FAssetCompileData {
    pub asset: TWeakObjectPtr<crate::bindings::core_u_object::UObject>,
}
impl FAssetCompileData {}
#[repr(C, align(8))]
pub struct FAssetManagerSearchRules {
    pub asset_scan_paths: TArray<FString>,
    pub include_patterns: TArray<FString>,
    pub exclude_patterns: TArray<FString>,
    pub asset_base_class: TSubclassOf<crate::bindings::core_u_object::UObject>,
    pub b_has_blueprint_classes: bool,
    pub b_force_synchronous_scan: bool,
    pub b_skip_virtual_path_expansion: bool,
    pub b_skip_manager_include_check: bool,
    __padding_end: [u8; 28],
}
impl FAssetManagerSearchRules {}
#[repr(C, align(8))]
pub struct FAudioVolumeSubmixSendSettings {
    pub listener_location_state: EAudioVolumeLocationState,
    pub submix_sends: TArray<FSoundSubmixSendInfo>,
}
impl FAudioVolumeSubmixSendSettings {}
#[repr(C, align(8))]
pub struct FSoundSubmixSendInfoBase {
    pub send_level_control_method: ESendLevelControlMethod,
    pub sound_submix: UPtr<USoundSubmixBase>,
    pub send_level: f32,
    pub disable_manual_send_clamp: bool,
    pub min_send_level: f32,
    pub max_send_level: f32,
    pub min_send_distance: f32,
    pub max_send_distance: f32,
    pub custom_send_level_curve: FRuntimeFloatCurve,
}
impl FSoundSubmixSendInfoBase {}
#[repr(C, align(8))]
pub struct FSoundSubmixSendInfo {
    #[doc(hidden)]
    __padding_176: [u8; 176],
    pub send_stage: ESubmixSendStage,
    __padding_end: [u8; 7],
}
impl FSoundSubmixSendInfo {}
#[repr(C, align(8))]
pub struct FAudioVolumeSubmixOverrideSettings {
    pub submix: UPtr<USoundSubmix>,
    pub submix_effect_chain: TArray<UPtr<USoundEffectSubmixPreset>>,
    pub crossfade_time: f32,
    __padding_end: [u8; 4],
}
impl FAudioVolumeSubmixOverrideSettings {}
#[repr(C, align(4))]
pub struct FInteriorSettings {
    #[doc(hidden)]
    __padding_4: [u8; 4],
    pub exterior_volume: f32,
    pub exterior_time: f32,
    pub exterior_lpf: f32,
    pub exterior_lpf_time: f32,
    pub interior_volume: f32,
    pub interior_time: f32,
    pub interior_lpf: f32,
    pub interior_lpf_time: f32,
}
impl FInteriorSettings {}
#[repr(C, align(8))]
pub struct FCameraLensInterfaceClassSupport {
    __padding_end: [u8; 8],
}
impl FCameraLensInterfaceClassSupport {}
#[repr(C, align(4))]
pub struct FCameraShakePatternStartParams {
    pub b_is_restarting: bool,
    pub b_override_duration: bool,
    pub duration_override: f32,
}
impl FCameraShakePatternStartParams {}
#[repr(C, align(16))]
pub struct FCameraShakePatternUpdateParams {
    pub delta_time: f32,
    pub shake_scale: f32,
    pub dynamic_scale: f32,
    pub pov: FMinimalViewInfo,
}
impl FCameraShakePatternUpdateParams {}
#[repr(C, align(16))]
pub struct FCameraShakePatternScrubParams {
    pub absolute_time: f32,
    pub shake_scale: f32,
    pub dynamic_scale: f32,
    pub pov: FMinimalViewInfo,
}
impl FCameraShakePatternScrubParams {}
#[repr(C, align(16))]
pub struct FCameraShakePatternUpdateResult {
    __padding_end: [u8; 2032],
}
impl FCameraShakePatternUpdateResult {}
#[repr(C, align(1))]
pub struct FCameraShakePatternStopParams {
    pub b_immediately: bool,
}
impl FCameraShakePatternStopParams {}
#[repr(C, align(4))]
pub struct FCameraShakeDuration {
    __padding_end: [u8; 8],
}
impl FCameraShakeDuration {}
#[repr(C, align(4))]
pub struct FCameraShakeInfo {
    __padding_end: [u8; 16],
}
impl FCameraShakeInfo {}
#[repr(C, align(8))]
pub struct FFindFloorResult {
    pub flags_0: u8,
    pub floor_dist: f32,
    pub line_dist: f32,
    pub hit_result: FHitResult,
}
impl FFindFloorResult {}
#[repr(C, align(8))]
pub struct FAudioComponentParam {
    #[doc(hidden)]
    __padding_160: [u8; 160],
    pub sound_wave_param: UPtr<USoundWave>,
}
impl FAudioComponentParam {}
#[repr(C, align(8))]
pub struct FLODMappingData {
    pub mapping: TArray<i32>,
    __padding_end: [u8; 16],
}
impl FLODMappingData {}
#[repr(C, align(4))]
pub struct FComponentSync {
    pub name: FName,
    pub sync_option: ESyncOption,
    __padding_end: [u8; 3],
}
impl FComponentSync {}
#[repr(C, align(8))]
pub struct FMaterialSpriteElement {
    pub material: UPtr<UMaterialInterface>,
    pub distance_to_opacity_curve: UPtr<UCurveFloat>,
    pub flags_16: u8,
    pub base_size_x: f32,
    pub base_size_y: f32,
    pub distance_to_size_curve: UPtr<UCurveFloat>,
}
impl FMaterialSpriteElement {}
#[repr(C, align(8))]
pub struct FEngineShowFlagsSetting {
    pub show_flag_name: FString,
    pub enabled: bool,
    __padding_end: [u8; 7],
}
impl FEngineShowFlagsSetting {}
#[repr(C, align(4))]
pub struct FSkelMeshSkinWeightInfo {
    __padding_end: [u8; 60],
}
impl FSkelMeshSkinWeightInfo {}
#[repr(C, align(4))]
pub struct FVertexOffsetUsage {
    __padding_end: [u8; 4],
}
impl FVertexOffsetUsage {}
#[repr(C, align(4))]
pub struct FTentDistribution {
    pub tip_altitude: f32,
    pub tip_value: f32,
    pub width: f32,
}
impl FTentDistribution {}
#[repr(C, align(8))]
pub struct FSplinePoint {
    pub input_key: f32,
    pub position: crate::bindings::core_u_object::FVector,
    pub arrive_tangent: crate::bindings::core_u_object::FVector,
    pub leave_tangent: crate::bindings::core_u_object::FVector,
    pub rotation: crate::bindings::core_u_object::FRotator,
    pub scale: crate::bindings::core_u_object::FVector,
    pub ty: ESplinePointType,
    __padding_end: [u8; 7],
}
impl FSplinePoint {}
#[repr(C, align(8))]
pub struct FEquirectProps {
    pub left_uv_rect: crate::bindings::core_u_object::FBox2D,
    pub right_uv_rect: crate::bindings::core_u_object::FBox2D,
    pub left_scale: crate::bindings::core_u_object::FVector2D,
    pub right_scale: crate::bindings::core_u_object::FVector2D,
    pub left_bias: crate::bindings::core_u_object::FVector2D,
    pub right_bias: crate::bindings::core_u_object::FVector2D,
    pub radius: f32,
    __padding_end: [u8; 4],
}
impl FEquirectProps {}
#[repr(C, align(4))]
pub struct FCullDistanceSizePair {
    pub size: f32,
    pub cull_distance: f32,
}
impl FCullDistanceSizePair {}
#[repr(C, align(8))]
pub struct FRuntimeCurveLinearColor {
    __padding_end: [u8; 520],
}
impl FRuntimeCurveLinearColor {}
#[repr(C, align(8))]
pub struct FRuntimeVectorCurve {
    __padding_end: [u8; 392],
}
impl FRuntimeVectorCurve {}
#[repr(C, align(8))]
pub struct FCurveTableRowHandle {
    pub curve_table: UPtr<UCurveTable>,
    pub row_name: FName,
    __padding_end: [u8; 4],
}
impl FCurveTableRowHandle {}
#[repr(C, align(8))]
pub struct FDataDrivenConsoleVariable {
    __padding_end: [u8; 96],
}
impl FDataDrivenConsoleVariable {}
#[repr(C, align(8))]
pub struct FDataTableRowHandle {
    pub data_table: UPtr<UDataTable>,
    pub row_name: FName,
    __padding_end: [u8; 4],
}
impl FDataTableRowHandle {}
#[repr(C, align(8))]
pub struct FDataTableCategoryHandle {
    pub data_table: UPtr<UDataTable>,
    pub column_name: FName,
    pub row_contents: FName,
}
impl FDataTableCategoryHandle {}
#[repr(C, align(8))]
pub struct FDialogueContext {
    pub speaker: UPtr<UDialogueVoice>,
    pub targets: TArray<UPtr<UDialogueVoice>>,
}
impl FDialogueContext {}
#[repr(C, align(8))]
pub struct FTypedElementPasteOptions {
    pub selection_set_to_modify: UPtr<
        crate::bindings::typed_element_runtime::UTypedElementSelectionSet,
    >,
    pub b_paste_at_location: bool,
    pub paste_location: crate::bindings::core_u_object::FVector,
    __padding_end: [u8; 80],
}
impl FTypedElementPasteOptions {}
#[repr(C, align(1))]
pub struct FTypedElementDeletionOptions {
    pub b_verify_deletion_can_happen: bool,
    pub b_warn_about_references: bool,
    pub b_warn_about_soft_references: bool,
}
impl FTypedElementDeletionOptions {}
#[repr(C, align(1))]
pub struct FViewLightingChannels {
    pub flags_0: u8,
}
impl FViewLightingChannels {}
#[repr(C, align(8))]
pub struct FFractureEffect {
    pub particle_system: UPtr<UParticleSystem>,
    pub sound: UPtr<USoundBase>,
}
impl FFractureEffect {}
#[repr(C, align(8))]
pub struct FBasedPosition {
    pub base: UPtr<AActor>,
    pub position: crate::bindings::core_u_object::FVector,
    __padding_end: [u8; 72],
}
impl FBasedPosition {}
#[repr(C, align(8))]
pub struct FPOV {
    pub location: crate::bindings::core_u_object::FVector,
    pub rotation: crate::bindings::core_u_object::FRotator,
    pub fov: f32,
    __padding_end: [u8; 4],
}
impl FPOV {}
#[repr(C, align(8))]
pub struct FMeshBuildSettings {
    pub flags_0: u8,
    pub flags_1: u8,
    pub min_lightmap_resolution: i32,
    pub src_lightmap_index: i32,
    pub dst_lightmap_index: i32,
    #[doc(hidden)]
    __padding_24: [u8; 8],
    pub build_scale3_d: crate::bindings::core_u_object::FVector,
    pub distance_field_resolution_scale: f32,
    pub distance_field_replacement_mesh: UPtr<UStaticMesh>,
    pub max_lumen_mesh_cards: i32,
    __padding_end: [u8; 4],
}
impl FMeshBuildSettings {}
#[repr(C, align(4))]
pub struct FSkeletalMeshBuildSettings {
    pub flags_0: u8,
    pub flags_1: u8,
    pub threshold_position: f32,
    pub threshold_tangent_normal: f32,
    pub threshold_uv: f32,
    pub morph_threshold_position: f32,
    pub bone_influence_limit: i32,
}
impl FSkeletalMeshBuildSettings {}
#[repr(C, align(8))]
pub struct FMeshDisplacementMap {
    __padding_end: [u8; 16],
}
impl FMeshDisplacementMap {}
#[repr(C, align(8))]
pub struct FMeshNaniteSettings {
    pub flags_0: u8,
    pub shape_preservation: ENaniteShapePreservation,
    pub position_precision: i32,
    pub normal_precision: i32,
    pub tangent_precision: i32,
    pub bone_weight_precision: i32,
    #[doc(hidden)]
    __padding_24: [u8; 4],
    pub keep_percent_triangles: f32,
    pub trim_relative_error: f32,
    pub generate_fallback: ENaniteGenerateFallback,
    pub fallback_target: ENaniteFallbackTarget,
    pub fallback_percent_triangles: f32,
    pub fallback_relative_error: f32,
    pub max_edge_length_factor: f32,
    pub num_rays: i32,
    pub voxel_level: i32,
    pub ray_back_up: f32,
    pub displacement_uv_channel: i32,
    __padding_end: [u8; 48],
}
impl FMeshNaniteSettings {}
#[repr(C, align(4))]
pub struct FMeshRayTracingProxySettings {
    pub flags_0: u8,
    pub fallback_target: ENaniteFallbackTarget,
    pub fallback_percent_triangles: f32,
    pub fallback_relative_error: f32,
    pub lod1_percent_triangles: f32,
    pub foliage_over_occlusion_bias: f32,
}
impl FMeshRayTracingProxySettings {}
#[repr(C, align(4))]
pub struct FCollectionReference {
    pub collection_name: FName,
}
impl FCollectionReference {}
#[repr(C, align(8))]
pub struct FDepthFieldGlowInfo {
    pub flags_0: u8,
    pub glow_color: crate::bindings::core_u_object::FLinearColor,
    pub glow_outer_radius: crate::bindings::core_u_object::FVector2D,
    pub glow_inner_radius: crate::bindings::core_u_object::FVector2D,
}
impl FDepthFieldGlowInfo {}
#[repr(C, align(8))]
pub struct FFontRenderInfo {
    pub flags_0: u8,
    pub glow_info: FDepthFieldGlowInfo,
}
impl FFontRenderInfo {}
#[repr(C, align(8))]
pub struct FCanvasUVTri {
    pub v0_pos: crate::bindings::core_u_object::FVector2D,
    pub v0_uv: crate::bindings::core_u_object::FVector2D,
    pub v0_color: crate::bindings::core_u_object::FLinearColor,
    pub v1_pos: crate::bindings::core_u_object::FVector2D,
    pub v1_uv: crate::bindings::core_u_object::FVector2D,
    pub v1_color: crate::bindings::core_u_object::FLinearColor,
    pub v2_pos: crate::bindings::core_u_object::FVector2D,
    pub v2_uv: crate::bindings::core_u_object::FVector2D,
    pub v2_color: crate::bindings::core_u_object::FLinearColor,
}
impl FCanvasUVTri {}
#[repr(C, align(8))]
pub struct FDamageEvent {
    __padding_end: [u8; 16],
}
impl FDamageEvent {}
#[repr(C, align(8))]
pub struct FPointDamageEvent {
    __padding_end: [u8; 312],
}
impl FPointDamageEvent {}
#[repr(C, align(4))]
pub struct FRadialDamageParams {
    pub base_damage: f32,
    pub minimum_damage: f32,
    pub inner_radius: f32,
    pub outer_radius: f32,
    pub damage_falloff: f32,
}
impl FRadialDamageParams {}
#[repr(C, align(8))]
pub struct FRadialDamageEvent {
    __padding_end: [u8; 80],
}
impl FRadialDamageEvent {}
#[repr(C, align(1))]
pub struct FFontFacePlatformRasterizationOverrides {
    __padding_end: [u8; 3],
}
impl FFontFacePlatformRasterizationOverrides {}
#[repr(C, align(4))]
pub struct FInputDevicePropertyHandle {
    __padding_end: [u8; 4],
}
impl FInputDevicePropertyHandle {}
#[repr(C, align(4))]
pub struct FDeviceColorData {
    pub b_enable: bool,
    pub b_reset_after_completion: bool,
    pub light_color: crate::bindings::core_u_object::FColor,
}
impl FDeviceColorData {}
#[repr(C, align(8))]
pub struct FDeviceColorCurveData {
    pub b_enable: bool,
    pub b_reset_after_completion: bool,
    pub device_color_curve: UPtr<UCurveLinearColor>,
}
impl FDeviceColorCurveData {}
#[repr(C, align(1))]
pub struct FDeviceTriggerBaseData {
    pub affected_triggers: crate::bindings::core_u_object::EInputDeviceTriggerMask,
    pub b_reset_upon_completion: bool,
}
impl FDeviceTriggerBaseData {}
#[repr(C, align(8))]
pub struct FDeviceTriggerFeedbackData {
    pub feedback_position_curve: UPtr<UCurveFloat>,
    pub feedback_strengh_curve: UPtr<UCurveFloat>,
}
impl FDeviceTriggerFeedbackData {}
#[repr(C, align(4))]
pub struct FDeviceTriggerTriggerResistanceData {
    pub start_position: i32,
    pub start_strengh: i32,
    pub end_position: i32,
    pub end_strengh: i32,
}
impl FDeviceTriggerTriggerResistanceData {}
#[repr(C, align(8))]
pub struct FDeviceTriggerTriggerVibrationData {
    pub trigger_position_curve: UPtr<UCurveFloat>,
    pub vibration_frequency_curve: UPtr<UCurveFloat>,
    pub vibration_amplitude_curve: UPtr<UCurveFloat>,
}
impl FDeviceTriggerTriggerVibrationData {}
#[repr(C, align(8))]
pub struct FAudioBasedVibrationData {
    pub sound: UPtr<USoundBase>,
}
impl FAudioBasedVibrationData {}
#[repr(C, align(4))]
pub struct FActivateDevicePropertyParams {
    pub user_id: crate::bindings::core_u_object::FPlatformUserId,
    pub device_id: crate::bindings::core_u_object::FInputDeviceId,
    pub flags_8: u8,
    __padding_end: [u8; 3],
}
impl FActivateDevicePropertyParams {}
#[repr(C, align(8))]
pub struct FMeshApproximationSettings {
    pub output_type: EMeshApproximationType,
    pub approximation_accuracy: f32,
    pub clamp_voxel_dimension: i32,
    pub b_attempt_auto_thickening: bool,
    pub target_min_thickness_multiplier: f32,
    pub b_ignore_tiny_parts: bool,
    pub tiny_part_size_multiplier: f32,
    pub base_capping: EMeshApproximationBaseCappingType,
    pub winding_threshold: f32,
    pub b_fill_gaps: bool,
    pub gap_distance: f32,
    pub occlusion_method: EOccludedGeometryFilteringPolicy,
    pub b_occlude_from_bottom: bool,
    pub simplify_method: EMeshApproximationSimplificationPolicy,
    pub target_tri_count: i32,
    pub triangles_per_m: f32,
    pub geometric_deviation: f32,
    pub ground_clipping: EMeshApproximationGroundPlaneClippingPolicy,
    pub ground_clipping_z_height: f32,
    pub b_estimate_hard_normals: bool,
    pub hard_normal_angle: f32,
    pub uv_generation_method: EMeshApproximationUVGenerationPolicy,
    pub initial_patch_count: i32,
    pub curvature_alignment: f32,
    pub merging_threshold: f32,
    pub max_angle_deviation: f32,
    pub b_generate_nanite_enabled_mesh: bool,
    pub nanite_fallback_target: ENaniteFallbackTarget,
    pub nanite_fallback_percent_triangles: f32,
    pub nanite_fallback_relative_error: f32,
    pub b_support_ray_tracing: bool,
    pub b_allow_distance_field: bool,
    pub multi_sampling_aa: i32,
    pub render_capture_resolution: i32,
    pub material_settings: FMaterialProxySettings,
    pub capture_field_of_view: f32,
    pub near_plane_dist: f32,
    pub b_use_render_lod_meshes: bool,
    pub b_enable_simplify_pre_pass: bool,
    pub b_enable_parallel_baking: bool,
    pub b_print_debug_messages: bool,
    pub b_emit_full_debug_mesh: bool,
    __padding_end: [u8; 11],
}
impl FMeshApproximationSettings {}
#[repr(C, align(8))]
pub struct FMaterialProxySettings {
    pub texture_sizing_type: ETextureSizingType,
    #[doc(hidden)]
    __padding_12: [u8; 8],
    pub target_texel_density_per_meter: f32,
    pub mesh_max_screen_size_percent: f32,
    pub mesh_min_draw_distance: f64,
    pub gutter_space: f32,
    pub metallic_constant: f32,
    pub roughness_constant: f32,
    pub anisotropy_constant: f32,
    pub specular_constant: f32,
    pub opacity_constant: f32,
    pub opacity_mask_constant: f32,
    pub ambient_occlusion_constant: f32,
    #[doc(hidden)]
    __padding_65: [u8; 1],
    pub blend_mode: EBlendMode,
    pub flags_66: u8,
    pub flags_67: u8,
    pub diffuse_texture_size: crate::bindings::core_u_object::FIntPoint,
    pub normal_texture_size: crate::bindings::core_u_object::FIntPoint,
    pub tangent_texture_size: crate::bindings::core_u_object::FIntPoint,
    pub metallic_texture_size: crate::bindings::core_u_object::FIntPoint,
    pub roughness_texture_size: crate::bindings::core_u_object::FIntPoint,
    pub anisotropy_texture_size: crate::bindings::core_u_object::FIntPoint,
    pub specular_texture_size: crate::bindings::core_u_object::FIntPoint,
    pub emissive_texture_size: crate::bindings::core_u_object::FIntPoint,
    pub opacity_texture_size: crate::bindings::core_u_object::FIntPoint,
    pub opacity_mask_texture_size: crate::bindings::core_u_object::FIntPoint,
    pub ambient_occlusion_texture_size: crate::bindings::core_u_object::FIntPoint,
    __padding_end: [u8; 4],
}
impl FMaterialProxySettings {}
#[repr(C, align(8))]
pub struct FMeshMergingSettings {
    pub target_light_map_resolution: i32,
    #[doc(hidden)]
    __padding_16: [u8; 8],
    pub material_settings: FMaterialProxySettings,
    #[doc(hidden)]
    __padding_180: [u8; 4],
    pub lod_selection_type: EMeshLODSelectionType,
    pub specific_lod: i32,
    pub flags_188: u8,
    pub flags_189: u8,
    pub nanite_settings: FMeshNaniteSettings,
    __padding_end: [u8; 24],
}
impl FMeshMergingSettings {}
#[repr(C, align(8))]
pub struct FMeshProxySettings {
    pub screen_size: i32,
    pub voxel_size: f32,
    pub material_settings: FMaterialProxySettings,
    #[doc(hidden)]
    __padding_184: [u8; 16],
    pub merge_distance: f32,
    pub unresolved_geometry_color: crate::bindings::core_u_object::FColor,
    pub max_ray_cast_dist: f32,
    pub hard_angle_threshold: f32,
    pub light_map_resolution: i32,
    pub normal_calculation_method: EProxyNormalComputationMethod,
    pub landscape_culling_precision: ELandscapeCullingPrecision,
    pub flags_206: u8,
    pub flags_207: u8,
    pub nanite_settings: FMeshNaniteSettings,
}
impl FMeshProxySettings {}
#[repr(C, align(1))]
pub struct FSlateModifierKeysState {
    __padding_end: [u8; 1],
}
impl FSlateModifierKeysState {}
#[repr(C, align(4))]
pub struct FFloatSpringState {
    __padding_end: [u8; 12],
}
impl FFloatSpringState {}
#[repr(C, align(8))]
pub struct FVectorSpringState {
    __padding_end: [u8; 56],
}
impl FVectorSpringState {}
#[repr(C, align(16))]
pub struct FQuaternionSpringState {
    __padding_end: [u8; 64],
}
impl FQuaternionSpringState {}
#[repr(C, align(8))]
pub struct FDrawToRenderTargetContext {
    __padding_end: [u8; 16],
}
impl FDrawToRenderTargetContext {}
#[repr(C, align(8))]
pub struct FImportanceTexture {
    __padding_end: [u8; 96],
}
impl FImportanceTexture {}
#[repr(C, align(8))]
pub struct FScalarParameterValue {
    #[doc(hidden)]
    __padding_120: [u8; 120],
    pub parameter_info: FMaterialParameterInfo,
    pub parameter_value: f32,
    __padding_end: [u8; 16],
}
impl FScalarParameterValue {}
#[repr(C, align(4))]
pub struct FVectorParameterValue {
    #[doc(hidden)]
    __padding_12: [u8; 12],
    pub parameter_info: FMaterialParameterInfo,
    pub parameter_value: crate::bindings::core_u_object::FLinearColor,
    __padding_end: [u8; 16],
}
impl FVectorParameterValue {}
#[repr(C, align(16))]
pub struct FDoubleVectorParameterValue {
    pub parameter_info: FMaterialParameterInfo,
    __padding_end: [u8; 60],
}
impl FDoubleVectorParameterValue {}
#[repr(C, align(8))]
pub struct FTextureParameterValue {
    #[doc(hidden)]
    __padding_12: [u8; 12],
    pub parameter_info: FMaterialParameterInfo,
    pub parameter_value: UPtr<UTexture>,
    __padding_end: [u8; 16],
}
impl FTextureParameterValue {}
#[repr(C, align(8))]
pub struct FTextureCollectionParameterValue {
    pub parameter_info: FMaterialParameterInfo,
    pub parameter_value: UPtr<UTextureCollection>,
    __padding_end: [u8; 16],
}
impl FTextureCollectionParameterValue {}
#[repr(C, align(8))]
pub struct FParameterCollectionParameterValue {
    pub parameter_info: FMaterialParameterInfo,
    pub parameter_value: UPtr<UMaterialParameterCollection>,
    __padding_end: [u8; 16],
}
impl FParameterCollectionParameterValue {}
#[repr(C, align(8))]
pub struct FRuntimeVirtualTextureParameterValue {
    pub parameter_info: FMaterialParameterInfo,
    pub parameter_value: UPtr<URuntimeVirtualTexture>,
    __padding_end: [u8; 16],
}
impl FRuntimeVirtualTextureParameterValue {}
#[repr(C, align(8))]
pub struct FSparseVolumeTextureParameterValue {
    pub parameter_info: FMaterialParameterInfo,
    pub parameter_value: UPtr<USparseVolumeTexture>,
    __padding_end: [u8; 16],
}
impl FSparseVolumeTextureParameterValue {}
#[repr(C, align(8))]
pub struct FFontParameterValue {
    #[doc(hidden)]
    __padding_12: [u8; 12],
    pub parameter_info: FMaterialParameterInfo,
    pub font_value: UPtr<UFont>,
    pub font_page: i32,
    __padding_end: [u8; 20],
}
impl FFontParameterValue {}
#[repr(C, align(4))]
pub struct FUserSceneTextureOverride {
    pub key: FName,
    pub value: FName,
}
impl FUserSceneTextureOverride {}
#[repr(C, align(8))]
pub struct FMeshInstancingSettings {
    pub actor_class_to_use: TSubclassOf<AActor>,
    pub instance_replacement_threshold: i32,
    pub b_skip_meshes_with_vertex_colors: bool,
    pub b_use_hlod_volumes: bool,
    pub ism_component_to_use: TSubclassOf<UInstancedStaticMeshComponent>,
}
impl FMeshInstancingSettings {}
#[repr(C, align(4))]
pub struct FMeshReductionSettings {
    pub percent_triangles: f32,
    #[doc(hidden)]
    __padding_8: [u8; 4],
    pub percent_vertices: f32,
    #[doc(hidden)]
    __padding_16: [u8; 4],
    pub max_deviation: f32,
    pub pixel_error: f32,
    pub welding_threshold: f32,
    pub hard_angle_threshold: f32,
    pub base_lod_model: i32,
    pub silhouette_importance: EMeshFeatureImportance,
    pub texture_importance: EMeshFeatureImportance,
    pub shading_importance: EMeshFeatureImportance,
    pub flags_39: u8,
    pub termination_criterion: EStaticMeshReductionTerimationCriterion,
    pub visibility_aggressiveness: EMeshFeatureImportance,
    pub vertex_color_importance: EMeshFeatureImportance,
    __padding_end: [u8; 1],
}
impl FMeshReductionSettings {}
#[repr(C, align(8))]
pub struct FUniqueNetIdRepl {
    __padding_end: [u8; 48],
}
impl FUniqueNetIdRepl {}
#[repr(C, align(16))]
pub struct FParticleSysParam {
    pub name: FName,
    pub param_type: EParticleSysParamType,
    pub scalar: f32,
    pub scalar_low: f32,
    pub vector: crate::bindings::core_u_object::FVector,
    pub vector_low: crate::bindings::core_u_object::FVector,
    pub color: crate::bindings::core_u_object::FColor,
    pub actor: UPtr<AActor>,
    pub material: UPtr<UMaterialInterface>,
    __padding_end: [u8; 128],
}
impl FParticleSysParam {}
#[repr(C, align(8))]
pub struct FFXSystemSpawnParameters {
    pub world_context_object: UPtr<crate::bindings::core_u_object::UObject>,
    pub system_template: UPtr<UFXSystemAsset>,
    pub location: crate::bindings::core_u_object::FVector,
    pub rotation: crate::bindings::core_u_object::FRotator,
    pub scale: crate::bindings::core_u_object::FVector,
    pub attach_to_component: UPtr<USceneComponent>,
    pub attach_point_name: FName,
    pub location_type: EAttachLocation,
    pub b_auto_destroy: bool,
    pub b_auto_activate: bool,
    pub pooling_method: EPSCPoolMethod,
    pub b_pre_cull_check: bool,
    pub b_is_player_effect: bool,
    __padding_end: [u8; 6],
}
impl FFXSystemSpawnParameters {}
#[repr(C, align(16))]
pub struct FConstraintInstanceAccessor {
    __padding_end: [u8; 64],
}
impl FConstraintInstanceAccessor {}
#[repr(C, align(4))]
pub struct FPhysicalAnimationData {
    #[doc(hidden)]
    __padding_12: [u8; 12],
    pub flags_12: u8,
    pub orientation_strength: f32,
    pub angular_velocity_strength: f32,
    pub position_strength: f32,
    pub velocity_strength: f32,
    pub max_linear_force: f32,
    pub max_angular_force: f32,
}
impl FPhysicalAnimationData {}
#[repr(C, align(4))]
pub struct FPhysicsAssetSolverSettings {
    pub position_iterations: i32,
    pub velocity_iterations: i32,
    pub projection_iterations: i32,
    pub cull_distance: f32,
    pub max_depenetration_velocity: f32,
    pub fixed_time_step: f32,
    pub b_use_linear_joint_solver: bool,
    pub b_use_manifolds: bool,
    __padding_end: [u8; 2],
}
impl FPhysicsAssetSolverSettings {}
#[repr(C, align(4))]
pub struct FSolverIterations {
    pub solver_iterations: i32,
    pub joint_iterations: i32,
    pub collision_iterations: i32,
    pub solver_push_out_iterations: i32,
    pub joint_push_out_iterations: i32,
    pub collision_push_out_iterations: i32,
}
impl FSolverIterations {}
#[repr(C, align(16))]
pub struct FTViewTarget {
    pub target: UPtr<AActor>,
    pub pov: FMinimalViewInfo,
    pub player_state: UPtr<APlayerState>,
    __padding_end: [u8; 8],
}
impl FTViewTarget {}
#[repr(C, align(4))]
pub struct FViewTargetTransitionParams {
    pub blend_time: f32,
    pub blend_function: EViewTargetBlendFunction,
    pub blend_exp: f32,
    pub flags_12: u8,
    __padding_end: [u8; 3],
}
impl FViewTargetTransitionParams {}
#[repr(C, align(8))]
pub struct FNeuralProfileStruct {
    pub input_format: ENeuralProfileFormat,
    pub output_format: ENeuralProfileFormat,
    pub runtime_type: ENeuralProfileRuntimeType,
    pub nne_model_data: UPtr<crate::bindings::core_u_object::UObject>,
    pub input_dimension: crate::bindings::core_u_object::FIntVector4,
    pub output_dimension: crate::bindings::core_u_object::FIntVector4,
    pub batch_size_override: i32,
    __padding_end: [u8; 20],
}
impl FNeuralProfileStruct {}
#[repr(C, align(8))]
pub struct FSpecularProfileStruct {
    pub format: ESpecularProfileFormat,
    #[doc(hidden)]
    __padding_1048: [u8; 1040],
    pub texture: UPtr<UTexture2D>,
}
impl FSpecularProfileStruct {}
#[repr(C, align(4))]
pub struct FSubsurfaceProfileStruct {
    pub surface_albedo: crate::bindings::core_u_object::FLinearColor,
    pub mean_free_path_color: crate::bindings::core_u_object::FLinearColor,
    pub mean_free_path_distance: f32,
    pub world_unit_scale: f32,
    pub b_enable_burley: bool,
    pub b_enable_mean_free_path: bool,
    pub tint: crate::bindings::core_u_object::FLinearColor,
    pub scatter_radius: f32,
    pub subsurface_color: crate::bindings::core_u_object::FLinearColor,
    pub falloff_color: crate::bindings::core_u_object::FLinearColor,
    pub boundary_color_bleed: crate::bindings::core_u_object::FLinearColor,
    pub implementation: ESubsurfaceImplementationTechniqueHint,
    pub extinction_scale: f32,
    pub normal_scale: f32,
    pub scattering_distribution: f32,
    pub ior: f32,
    pub roughness0: f32,
    pub roughness1: f32,
    pub lobe_mix: f32,
    pub transmission_tint_color: crate::bindings::core_u_object::FLinearColor,
}
impl FSubsurfaceProfileStruct {}
#[repr(C, align(8))]
pub struct FReverbSettings {
    pub b_apply_reverb: bool,
    pub reverb_effect: UPtr<UReverbEffect>,
    pub reverb_plugin_effect: UPtr<USoundEffectSubmixPreset>,
    pub volume: f32,
    pub fade_time: f32,
}
impl FReverbSettings {}
#[repr(C, align(16))]
pub struct FColorGradePerRangeSettings {
    pub saturation: crate::bindings::core_u_object::FVector4,
    pub contrast: crate::bindings::core_u_object::FVector4,
    pub gamma: crate::bindings::core_u_object::FVector4,
    pub gain: crate::bindings::core_u_object::FVector4,
    pub offset: crate::bindings::core_u_object::FVector4,
}
impl FColorGradePerRangeSettings {}
#[repr(C, align(16))]
pub struct FColorGradingSettings {
    pub global: FColorGradePerRangeSettings,
    pub shadows: FColorGradePerRangeSettings,
    pub midtones: FColorGradePerRangeSettings,
    pub highlights: FColorGradePerRangeSettings,
    pub shadows_max: f32,
    pub highlights_min: f32,
    pub highlights_max: f32,
    __padding_end: [u8; 4],
}
impl FColorGradingSettings {}
#[repr(C, align(4))]
pub struct FFilmStockSettings {
    pub slope: f32,
    pub toe: f32,
    pub shoulder: f32,
    pub black_clip: f32,
    pub white_clip: f32,
}
impl FFilmStockSettings {}
#[repr(C, align(4))]
pub struct FGaussianSumBloomSettings {
    pub intensity: f32,
    pub threshold: f32,
    pub size_scale: f32,
    pub filter1_size: f32,
    pub filter2_size: f32,
    pub filter3_size: f32,
    pub filter4_size: f32,
    pub filter5_size: f32,
    pub filter6_size: f32,
    pub filter1_tint: crate::bindings::core_u_object::FLinearColor,
    pub filter2_tint: crate::bindings::core_u_object::FLinearColor,
    pub filter3_tint: crate::bindings::core_u_object::FLinearColor,
    pub filter4_tint: crate::bindings::core_u_object::FLinearColor,
    pub filter5_tint: crate::bindings::core_u_object::FLinearColor,
    pub filter6_tint: crate::bindings::core_u_object::FLinearColor,
}
impl FGaussianSumBloomSettings {}
#[repr(C, align(8))]
pub struct FConvolutionBloomSettings {
    pub intensity: f32,
    pub texture: UPtr<UTexture2D>,
    pub scatter_dispersion: f32,
    pub size: f32,
    pub center_uv: crate::bindings::core_u_object::FVector2D,
    pub pre_filter_min: f32,
    pub pre_filter_max: f32,
    pub pre_filter_mult: f32,
    pub buffer_scale: f32,
}
impl FConvolutionBloomSettings {}
#[repr(C, align(8))]
pub struct FLensBloomSettings {
    pub intensity: f32,
    pub gaussian_sum: FGaussianSumBloomSettings,
    pub convolution: FConvolutionBloomSettings,
    pub method: EBloomMethod,
    __padding_end: [u8; 7],
}
impl FLensBloomSettings {}
#[repr(C, align(8))]
pub struct FLensImperfectionSettings {
    pub dirt_mask: UPtr<UTexture>,
    pub dirt_mask_intensity: f32,
    pub dirt_mask_tint: crate::bindings::core_u_object::FLinearColor,
    __padding_end: [u8; 4],
}
impl FLensImperfectionSettings {}
#[repr(C, align(8))]
pub struct FLensSettings {
    pub bloom: FLensBloomSettings,
    pub imperfections: FLensImperfectionSettings,
    pub chromatic_aberration: f32,
    __padding_end: [u8; 4],
}
impl FLensSettings {}
#[repr(C, align(8))]
pub struct FCameraExposureSettings {
    pub method: EAutoExposureMethod,
    pub low_percent: f32,
    pub high_percent: f32,
    pub min_brightness: f32,
    pub max_brightness: f32,
    pub speed_up: f32,
    pub speed_down: f32,
    pub bias: f32,
    pub bias_curve: UPtr<UCurveFloat>,
    pub meter_mask: UPtr<UTexture>,
    pub histogram_log_min: f32,
    pub histogram_log_max: f32,
    pub calibration_constant: f32,
    pub flags_60: u8,
    __padding_end: [u8; 3],
}
impl FCameraExposureSettings {}
#[repr(C, align(8))]
pub struct FSingleAnimationPlayData {
    pub anim_to_play: UPtr<UAnimationAsset>,
    pub flags_8: u8,
    pub saved_position: f32,
    pub saved_play_rate: f32,
    __padding_end: [u8; 4],
}
impl FSingleAnimationPlayData {}
#[repr(C, align(8))]
pub struct FSkelMeshMergeSectionMapping {
    pub section_i_ds: TArray<i32>,
}
impl FSkelMeshMergeSectionMapping {}
#[repr(C, align(8))]
pub struct FSkelMeshMergeMeshUVTransforms {
    pub uv_transforms: TArray<crate::bindings::core_u_object::FTransform>,
}
impl FSkelMeshMergeMeshUVTransforms {}
#[repr(C, align(8))]
pub struct FSkelMeshMergeUVTransformMapping {
    pub uv_transforms_per_mesh: TArray<FSkelMeshMergeMeshUVTransforms>,
}
impl FSkelMeshMergeUVTransformMapping {}
#[repr(C, align(8))]
pub struct FSkeletalMaterial {
    pub material_interface: UPtr<UMaterialInterface>,
    pub material_slot_name: FName,
    #[doc(hidden)]
    __padding_36: [u8; 16],
    pub uv_channel_data: FMeshUVChannelInfo,
    pub overlay_material_interface: UPtr<UMaterialInterface>,
}
impl FSkeletalMaterial {}
#[repr(C, align(8))]
pub struct FSoundAttenuationPluginSettings {
    pub spatialization_plugin_settings_array: TArray<
        UPtr<crate::bindings::audio_extensions::USpatializationPluginSourceSettingsBase>,
    >,
    pub occlusion_plugin_settings_array: TArray<
        UPtr<crate::bindings::audio_extensions::UOcclusionPluginSourceSettingsBase>,
    >,
    pub reverb_plugin_settings_array: TArray<
        UPtr<crate::bindings::audio_extensions::UReverbPluginSourceSettingsBase>,
    >,
    pub source_data_override_plugin_settings_array: TArray<
        UPtr<
            crate::bindings::audio_extensions::USourceDataOverridePluginSourceSettingsBase,
        >,
    >,
}
impl FSoundAttenuationPluginSettings {}
#[repr(C, align(8))]
pub struct FAttenuationSubmixSendSettings {
    __padding_end: [u8; 176],
}
impl FAttenuationSubmixSendSettings {}
#[repr(C, align(8))]
pub struct FSoundAttenuationSettings {
    #[doc(hidden)]
    __padding_192: [u8; 192],
    pub flags_192: u8,
    pub flags_193: u8,
    pub spatialization_algorithm: ESoundSpatializationAlgorithm,
    pub audio_link_settings_override: UPtr<
        crate::bindings::audio_link_core::UAudioLinkSettingsAbstract,
    >,
    pub binaural_radius: f32,
    pub custom_lowpass_air_absorption_curve: FRuntimeFloatCurve,
    pub custom_highpass_air_absorption_curve: FRuntimeFloatCurve,
    pub absorption_method: EAirAbsorptionMethod,
    pub occlusion_trace_channel: ECollisionChannel,
    pub reverb_send_method: EReverbSendMethod,
    pub priority_attenuation_method: EPriorityAttenuationMethod,
    #[doc(hidden)]
    __padding_500: [u8; 8],
    pub non_spatialized_radius_start: f32,
    pub non_spatialized_radius_end: f32,
    pub non_spatialized_radius_mode: ENonSpatializedRadiusSpeakerMapMode,
    pub stereo_spread: f32,
    #[doc(hidden)]
    __padding_536: [u8; 20],
    pub lpf_radius_min: f32,
    pub lpf_radius_max: f32,
    pub lpf_frequency_at_min: f32,
    pub lpf_frequency_at_max: f32,
    pub hpf_frequency_at_min: f32,
    pub hpf_frequency_at_max: f32,
    pub focus_azimuth: f32,
    pub non_focus_azimuth: f32,
    pub focus_distance_scale: f32,
    pub non_focus_distance_scale: f32,
    pub focus_priority_scale: f32,
    pub non_focus_priority_scale: f32,
    pub focus_volume_attenuation: f32,
    pub non_focus_volume_attenuation: f32,
    pub focus_attack_interp_speed: f32,
    pub focus_release_interp_speed: f32,
    pub occlusion_low_pass_filter_frequency: f32,
    pub occlusion_volume_attenuation: f32,
    pub occlusion_interpolation_time: f32,
    #[doc(hidden)]
    __padding_632: [u8; 20],
    pub reverb_wet_level_min: f32,
    pub reverb_wet_level_max: f32,
    pub reverb_distance_min: f32,
    pub reverb_distance_max: f32,
    pub manual_reverb_send_level: f32,
    pub priority_attenuation_min: f32,
    pub priority_attenuation_max: f32,
    pub priority_attenuation_distance_min: f32,
    pub priority_attenuation_distance_max: f32,
    pub manual_priority_attenuation: f32,
    pub custom_reverb_send_curve: FRuntimeFloatCurve,
    pub submix_send_settings: TArray<FAttenuationSubmixSendSettings>,
    pub custom_priority_attenuation_curve: FRuntimeFloatCurve,
    pub plugin_settings: FSoundAttenuationPluginSettings,
}
impl FSoundAttenuationSettings {}
#[repr(C, align(8))]
pub struct FSoundClassProperties {
    pub volume: f32,
    pub pitch: f32,
    pub low_pass_filter_frequency: f32,
    pub attenuation_distance_scale: f32,
    pub lfe_bleed: f32,
    pub voice_center_channel_volume: f32,
    pub radio_filter_volume: f32,
    pub radio_filter_volume_threshold: f32,
    pub flags_32: u8,
    pub default2_d_reverb_send_amount: f32,
    pub modulation_settings: FSoundModulationDefaultSettings,
    pub output_target: EAudioOutputTarget,
    pub loading_behavior: ESoundWaveLoadingBehavior,
    #[doc(hidden)]
    __padding_520: [u8; 88],
    pub default_submix: UPtr<USoundSubmix>,
}
impl FSoundClassProperties {}
#[repr(C, align(8))]
pub struct FSoundModulationDefaultSettings {
    pub volume_modulation_destination: FSoundModulationDestinationSettings,
    pub pitch_modulation_destination: FSoundModulationDestinationSettings,
    pub highpass_modulation_destination: FSoundModulationDestinationSettings,
    pub lowpass_modulation_destination: FSoundModulationDestinationSettings,
}
impl FSoundModulationDefaultSettings {}
#[repr(C, align(8))]
pub struct FSoundModulationDestinationSettings {
    pub value: f32,
    #[doc(hidden)]
    __padding_16: [u8; 8],
    pub modulators: TSet<UPtr<crate::bindings::audio_extensions::USoundModulatorBase>>,
}
impl FSoundModulationDestinationSettings {}
#[repr(C, align(8))]
pub struct FPassiveSoundMixModifier {
    pub sound_mix: UPtr<USoundMix>,
    pub min_volume_threshold: f32,
    pub max_volume_threshold: f32,
}
impl FPassiveSoundMixModifier {}
#[repr(C, align(8))]
pub struct FSoundConcurrencySettings {
    pub max_count: i32,
    pub flags_4: u8,
    pub resolution_rule: EMaxConcurrentResolutionRule,
    pub retrigger_time: f32,
    pub platform_max_count: crate::bindings::core_u_object::FPerPlatformInt,
    #[doc(hidden)]
    __padding_108: [u8; 4],
    pub volume_scale_mode: EConcurrencyVolumeScaleMode,
    pub volume_scale_attack_time: f32,
    pub volume_scale_release_time: f32,
    pub voice_steal_release_time: f32,
    __padding_end: [u8; 4],
}
impl FSoundConcurrencySettings {}
#[repr(C, align(8))]
pub struct FSoundClassAdjuster {
    pub sound_class_object: UPtr<USoundClass>,
    pub volume_adjuster: f32,
    pub pitch_adjuster: f32,
    pub low_pass_filter_frequency: f32,
    pub flags_20: u8,
    pub voice_center_channel_volume_adjuster: f32,
    __padding_end: [u8; 4],
}
impl FSoundClassAdjuster {}
#[repr(C, align(8))]
pub struct FSoundModulationDefaultRoutingSettings {
    #[doc(hidden)]
    __padding_384: [u8; 384],
    pub volume_routing: EModulationRouting,
    pub pitch_routing: EModulationRouting,
    pub highpass_routing: EModulationRouting,
    pub lowpass_routing: EModulationRouting,
    __padding_end: [u8; 4],
}
impl FSoundModulationDefaultRoutingSettings {}
#[repr(C, align(4))]
pub struct FModulatorContinuousParams {
    pub parameter_name: FName,
    __padding_end: [u8; 24],
}
impl FModulatorContinuousParams {}
#[repr(C, align(4))]
pub struct FSoundSubmixSpectralAnalysisBandSettings {
    pub band_frequency: f32,
    pub attack_time_msec: i32,
    pub release_time_msec: i32,
    pub q_factor: f32,
}
impl FSoundSubmixSpectralAnalysisBandSettings {}
#[repr(C, align(4))]
pub struct FSoundWaveSpectralData {
    pub frequency_hz: f32,
    pub magnitude: f32,
    pub normalized_magnitude: f32,
}
impl FSoundWaveSpectralData {}
#[repr(C, align(8))]
pub struct FSoundWaveSpectralDataPerSound {
    pub spectral_data: TArray<FSoundWaveSpectralData>,
    pub playback_time: f32,
    pub sound_wave: UPtr<USoundWave>,
}
impl FSoundWaveSpectralDataPerSound {}
#[repr(C, align(8))]
pub struct FSoundWaveEnvelopeDataPerSound {
    pub envelope: f32,
    pub playback_time: f32,
    pub sound_wave: UPtr<USoundWave>,
}
impl FSoundWaveEnvelopeDataPerSound {}
#[repr(C, align(8))]
pub struct FStaticMaterial {
    pub material_interface: UPtr<UMaterialInterface>,
    pub material_slot_name: FName,
    #[doc(hidden)]
    __padding_32: [u8; 12],
    pub uv_channel_data: FMeshUVChannelInfo,
    pub overlay_material_interface: UPtr<UMaterialInterface>,
}
impl FStaticMaterial {}
#[repr(C, align(8))]
pub struct FSubtitleAssetData {
    pub text: FText,
    pub subtitle_duration_type: ESubtitleDurationType,
    pub duration: f32,
    pub start_offset: f32,
    pub priority: f32,
    pub subtitle_type: ESubtitleType,
    __padding_end: [u8; 23],
}
impl FSubtitleAssetData {}
#[repr(C, align(8))]
pub struct FTextureSourceColorSettings {
    pub encoding_override: ETextureSourceEncoding,
    pub color_space: ETextureColorSpace,
    pub red_chromaticity_coordinate: crate::bindings::core_u_object::FVector2D,
    pub green_chromaticity_coordinate: crate::bindings::core_u_object::FVector2D,
    pub blue_chromaticity_coordinate: crate::bindings::core_u_object::FVector2D,
    pub white_chromaticity_coordinate: crate::bindings::core_u_object::FVector2D,
    pub chromatic_adaptation_method: ETextureChromaticAdaptationMethod,
    __padding_end: [u8; 7],
}
impl FTextureSourceColorSettings {}
#[repr(C, align(8))]
pub struct FCanvasIcon {
    pub texture: UPtr<UTexture>,
    pub u: f32,
    pub v: f32,
    pub ul: f32,
    pub vl: f32,
}
impl FCanvasIcon {}
#[repr(C, align(4))]
pub struct FHardwareDeviceIdentifier {
    pub input_class_name: FName,
    pub hardware_device_identifier: FName,
    pub primary_device_type: EHardwareDevicePrimaryType,
    pub supported_features_mask: i32,
}
impl FHardwareDeviceIdentifier {}
#[repr(C, align(8))]
pub struct FInputActionKeyMapping {
    pub action_name: FName,
    pub flags_12: u8,
    pub key: crate::bindings::input_core::FKey,
}
impl FInputActionKeyMapping {}
#[repr(C, align(8))]
pub struct FInputAxisKeyMapping {
    pub axis_name: FName,
    pub scale: f32,
    pub key: crate::bindings::input_core::FKey,
}
impl FInputAxisKeyMapping {}
#[repr(C, align(4))]
pub struct FInputActionSpeechMapping {
    __padding_end: [u8; 24],
}
impl FInputActionSpeechMapping {}
#[repr(C, align(8))]
pub struct FVoiceSettings {
    pub component_to_attach_to: UPtr<USceneComponent>,
    pub attenuation_settings: UPtr<USoundAttenuation>,
    pub source_effect_chain: UPtr<USoundEffectSourcePresetChain>,
}
impl FVoiceSettings {}
#[repr(C, align(8))]
pub struct FActorDesc {
    pub guid: crate::bindings::core_u_object::FGuid,
    pub native_class: TSubclassOf<crate::bindings::core_u_object::UObject>,
    pub class: crate::bindings::core_u_object::FSoftObjectPath,
    pub name: FName,
    pub label: FName,
    pub bounds: crate::bindings::core_u_object::FBox,
    pub runtime_grid: FName,
    pub b_is_spatially_loaded: bool,
    pub b_actor_is_editor_only: bool,
    pub actor_package: FName,
    pub actor_path: FName,
    pub data_layer_assets: TArray<crate::bindings::core_u_object::FSoftObjectPath>,
}
impl FActorDesc {}
#[repr(C, align(8))]
pub struct FExportHLODAssetsParams {
    __padding_end: [u8; 32],
}
impl FExportHLODAssetsParams {}
#[repr(C, align(8))]
pub struct FWorldPartitionHLODDestructionTag {
    __padding_end: [u8; 16],
}
impl FWorldPartitionHLODDestructionTag {}
#[repr(C, align(8))]
pub struct FStreamingSourceShape {
    pub b_use_grid_loading_range: bool,
    pub loading_range_scale: f32,
    pub radius: f32,
    pub b_is_sector: bool,
    pub sector_angle: f32,
    pub location: crate::bindings::core_u_object::FVector,
    pub rotation: crate::bindings::core_u_object::FRotator,
}
impl FStreamingSourceShape {}
#[repr(C, align(8))]
pub struct FWorldPartitionStreamingQuerySource {
    pub location: crate::bindings::core_u_object::FVector,
    pub radius: f32,
    pub b_use_grid_loading_range: bool,
    pub data_layers: TArray<FName>,
    pub b_data_layers_only: bool,
    pub b_spatial_query: bool,
    __padding_end: [u8; 214],
}
impl FWorldPartitionStreamingQuerySource {}
#[repr(C, align(4))]
pub struct FLightmassWorldInfoSettings {
    pub static_lighting_level_scale: f32,
    pub num_indirect_lighting_bounces: i32,
    pub num_sky_lighting_bounces: i32,
    pub indirect_lighting_quality: f32,
    pub indirect_lighting_smoothness: f32,
    pub environment_color: crate::bindings::core_u_object::FColor,
    pub environment_intensity: f32,
    #[doc(hidden)]
    __padding_32: [u8; 4],
    pub diffuse_boost: f32,
    pub volume_lighting_method: EVolumeLightingMethod,
    pub flags_37: u8,
    pub volumetric_lightmap_detail_cell_size: f32,
    pub volumetric_lightmap_maximum_brick_memory_mb: f32,
    pub volumetric_lightmap_loading_cell_size: f32,
    pub volumetric_lightmap_spherical_harmonic_smoothing: f32,
    pub volume_light_sample_placement_scale: f32,
    pub direct_illumination_occlusion_fraction: f32,
    pub indirect_illumination_occlusion_fraction: f32,
    pub occlusion_exponent: f32,
    pub fully_occluded_samples_fraction: f32,
    pub max_occlusion_distance: f32,
    __padding_end: [u8; 4],
}
impl FLightmassWorldInfoSettings {}
#[repr(C, align(8))]
pub struct UMaterialExpression {
    #[doc(hidden)]
    __padding_48: [u8; 48],
    pub material_expression_editor_x: i32,
    pub material_expression_editor_y: i32,
    __padding_end: [u8; 144],
}
impl UMaterialExpression {}
#[repr(C, align(8))]
pub struct UMaterialExpressionCustomOutput {
    __padding_end: [u8; 200],
}
impl UMaterialExpressionCustomOutput {}
#[repr(C, align(8))]
pub struct UMaterialExpressionBentNormalCustomOutput {
    __padding_end: [u8; 248],
}
impl UMaterialExpressionBentNormalCustomOutput {}
#[repr(C, align(8))]
pub struct UMaterialExpressionClearCoatNormalCustomOutput {
    __padding_end: [u8; 248],
}
impl UMaterialExpressionClearCoatNormalCustomOutput {}
#[repr(C, align(8))]
pub struct UMaterialExpressionTangentOutput {
    __padding_end: [u8; 248],
}
impl UMaterialExpressionTangentOutput {}
#[repr(C, align(8))]
pub struct UMaterialExpressionThinTranslucentMaterialOutput {
    __padding_end: [u8; 296],
}
impl UMaterialExpressionThinTranslucentMaterialOutput {}
#[repr(C, align(8))]
pub struct UMaterialExpressionFirstPersonOutput {
    __padding_end: [u8; 256],
}
impl UMaterialExpressionFirstPersonOutput {}
#[repr(C, align(8))]
pub struct UMaterialExpressionMaterialCache {
    __padding_end: [u8; 552],
}
impl UMaterialExpressionMaterialCache {}
#[repr(C, align(8))]
pub struct UMaterialExpressionTemporalResponsivenessOutput {
    __padding_end: [u8; 248],
}
impl UMaterialExpressionTemporalResponsivenessOutput {}
#[repr(C, align(8))]
pub struct UMaterialExpressionMotionVectorWorldOffsetOutput {
    __padding_end: [u8; 248],
}
impl UMaterialExpressionMotionVectorWorldOffsetOutput {}
#[repr(C, align(8))]
pub struct UEdGraph {
    __padding_end: [u8; 192],
}
impl UEdGraph {}
#[repr(C, align(8))]
pub struct UEdGraphNode {
    __padding_end: [u8; 192],
}
impl UEdGraphNode {}
#[repr(C, align(8))]
pub struct UEdGraphSchema {
    __padding_end: [u8; 48],
}
impl UEdGraphSchema {}
#[repr(C, align(8))]
pub struct UDataAsset {
    __padding_end: [u8; 56],
}
impl UDataAsset {}
#[repr(C, align(8))]
pub struct AActor {
    #[doc(hidden)]
    __padding_104: [u8; 104],
    pub flags_104: u8,
    #[doc(hidden)]
    __padding_108: [u8; 3],
    pub flags_108: u8,
    pub flags_109: u8,
    pub flags_110: u8,
    pub flags_111: u8,
    #[doc(hidden)]
    __padding_120: [u8; 8],
    pub initial_life_span: f32,
    pub custom_time_dilation: f32,
    #[doc(hidden)]
    __padding_136: [u8; 8],
    pub runtime_grid: FName,
    #[doc(hidden)]
    __padding_421: [u8; 273],
    pub net_dormancy: ENetDormancy,
    pub spawn_collision_handling_method: ESpawnActorCollisionHandlingMethod,
    #[doc(hidden)]
    __padding_444: [u8; 20],
    pub net_cull_distance_squared: f32,
    pub net_update_frequency: f32,
    pub min_net_update_frequency: f32,
    pub net_priority: f32,
    #[doc(hidden)]
    __padding_472: [u8; 8],
    pub instigator: UPtr<APawn>,
    #[doc(hidden)]
    __padding_496: [u8; 16],
    pub root_component: UPtr<USceneComponent>,
    pub pivot_offset: crate::bindings::core_u_object::FVector,
    #[doc(hidden)]
    __padding_592: [u8; 64],
    pub actor_guid: crate::bindings::core_u_object::FGuid,
    pub actor_instance_guid: crate::bindings::core_u_object::FGuid,
    pub content_bundle_guid: crate::bindings::core_u_object::FGuid,
    #[doc(hidden)]
    __padding_712: [u8; 72],
    pub sprite_scale: f32,
    #[doc(hidden)]
    __padding_805: [u8; 89],
    pub flags_805: u8,
    pub tags: TArray<FName>,
    __padding_end: [u8; 312],
}
impl AActor {
    fn set_actor_location(
        new_location: crate::bindings::core_u_object::FVector,
        b_sweep: bool,
        sweep_hit_result: FHitResult,
        b_teleport: bool,
    ) -> bool {
        #[repr(C)]
        pub struct Param {
            new_location: crate::bindings::core_u_object::FVector,
            b_sweep: bool,
            sweep_hit_result: FHitResult,
            b_teleport: bool,
            return_value: bool,
        }
        todo!()
    }
    fn get_actor_location() -> crate::bindings::core_u_object::FVector {
        #[repr(C)]
        pub struct Param {
            return_value: crate::bindings::core_u_object::FVector,
        }
        todo!()
    }
    pub fn verify_layout() {
        log::warn!(
            "{} = {} vs {}", "initial_life_span", std::mem::offset_of!(AActor,
            initial_life_span), 120usize
        );
        log::warn!(
            "{} = {} vs {}", "custom_time_dilation", std::mem::offset_of!(AActor,
            custom_time_dilation), 124usize
        );
        log::warn!(
            "{} = {} vs {}", "runtime_grid", std::mem::offset_of!(AActor, runtime_grid),
            136usize
        );
        log::warn!(
            "{} = {} vs {}", "net_dormancy", std::mem::offset_of!(AActor, net_dormancy),
            421usize
        );
        log::warn!(
            "{} = {} vs {}", "spawn_collision_handling_method",
            std::mem::offset_of!(AActor, spawn_collision_handling_method), 422usize
        );
        log::warn!(
            "{} = {} vs {}", "net_cull_distance_squared", std::mem::offset_of!(AActor,
            net_cull_distance_squared), 444usize
        );
        log::warn!(
            "{} = {} vs {}", "net_update_frequency", std::mem::offset_of!(AActor,
            net_update_frequency), 448usize
        );
        log::warn!(
            "{} = {} vs {}", "min_net_update_frequency", std::mem::offset_of!(AActor,
            min_net_update_frequency), 452usize
        );
        log::warn!(
            "{} = {} vs {}", "net_priority", std::mem::offset_of!(AActor, net_priority),
            456usize
        );
        log::warn!(
            "{} = {} vs {}", "instigator", std::mem::offset_of!(AActor, instigator),
            472usize
        );
        log::warn!(
            "{} = {} vs {}", "root_component", std::mem::offset_of!(AActor,
            root_component), 496usize
        );
        log::warn!(
            "{} = {} vs {}", "pivot_offset", std::mem::offset_of!(AActor, pivot_offset),
            504usize
        );
        log::warn!(
            "{} = {} vs {}", "actor_guid", std::mem::offset_of!(AActor, actor_guid),
            592usize
        );
        log::warn!(
            "{} = {} vs {}", "actor_instance_guid", std::mem::offset_of!(AActor,
            actor_instance_guid), 608usize
        );
        log::warn!(
            "{} = {} vs {}", "content_bundle_guid", std::mem::offset_of!(AActor,
            content_bundle_guid), 624usize
        );
        log::warn!(
            "{} = {} vs {}", "sprite_scale", std::mem::offset_of!(AActor, sprite_scale),
            712usize
        );
        log::warn!(
            "{} = {} vs {}", "tags", std::mem::offset_of!(AActor, tags), 808usize
        );
    }
}
#[repr(C, align(8))]
pub struct UActorComponent {
    #[doc(hidden)]
    __padding_120: [u8; 120],
    pub component_tags: TArray<FName>,
    #[doc(hidden)]
    __padding_184: [u8; 48],
    pub flags_184: u8,
    #[doc(hidden)]
    __padding_186: [u8; 1],
    pub flags_186: u8,
    pub flags_187: u8,
    __padding_end: [u8; 52],
}
impl UActorComponent {}
#[repr(C, align(16))]
pub struct USceneComponent {
    #[doc(hidden)]
    __padding_384: [u8; 384],
    pub relative_location: crate::bindings::core_u_object::FVector,
    pub relative_rotation: crate::bindings::core_u_object::FRotator,
    pub relative_scale3_d: crate::bindings::core_u_object::FVector,
    #[doc(hidden)]
    __padding_480: [u8; 24],
    pub flags_480: u8,
    pub flags_481: u8,
    #[doc(hidden)]
    __padding_484: [u8; 2],
    pub mobility: EComponentMobility,
    pub detail_mode: EDetailMode,
    __padding_end: [u8; 170],
}
impl USceneComponent {
    pub fn verify_layout() {
        log::warn!(
            "{} = {} vs {}", "relative_location", std::mem::offset_of!(USceneComponent,
            relative_location), 384usize
        );
        log::warn!(
            "{} = {} vs {}", "relative_rotation", std::mem::offset_of!(USceneComponent,
            relative_rotation), 408usize
        );
        log::warn!(
            "{} = {} vs {}", "relative_scale3_d", std::mem::offset_of!(USceneComponent,
            relative_scale3_d), 432usize
        );
        log::warn!(
            "{} = {} vs {}", "mobility", std::mem::offset_of!(USceneComponent, mobility),
            484usize
        );
        log::warn!(
            "{} = {} vs {}", "detail_mode", std::mem::offset_of!(USceneComponent,
            detail_mode), 485usize
        );
    }
}
#[repr(C, align(16))]
pub struct UPrimitiveComponent {
    #[doc(hidden)]
    __padding_680: [u8; 680],
    pub min_draw_distance: f32,
    pub ld_max_draw_distance: f32,
    pub cached_max_draw_distance: f32,
    #[doc(hidden)]
    __padding_694: [u8; 2],
    pub indirect_lighting_cache_quality: EIndirectLightingCacheQuality,
    pub lightmap_type: ELightmapType,
    pub hlod_batching_policy: EHLODBatchingPolicy,
    pub shadow_cache_invalidation_behavior: EShadowCacheInvalidationBehavior,
    pub flags_698: u8,
    pub flags_699: u8,
    pub flags_700: u8,
    pub flags_701: u8,
    pub flags_702: u8,
    pub flags_703: u8,
    pub flags_704: u8,
    pub flags_705: u8,
    pub flags_706: u8,
    pub first_person_primitive_type: EFirstPersonPrimitiveType,
    pub flags_708: u8,
    #[doc(hidden)]
    __padding_720: [u8; 8],
    pub exclude_for_specific_hlod_levels_deprecated: TArray<i32>,
    #[doc(hidden)]
    __padding_739: [u8; 3],
    pub can_character_step_up_on: ECanBeCharacterBase,
    pub lighting_channels: FLightingChannels,
    #[doc(hidden)]
    __padding_769: [u8; 28],
    pub ray_tracing_group_culling_priority: ERayTracingGroupCullingPriority,
    pub custom_depth_stencil_write_mask: ERendererStencilMask,
    pub ray_tracing_group_id: i32,
    #[doc(hidden)]
    __padding_780: [u8; 4],
    pub custom_depth_stencil_value: i32,
    #[doc(hidden)]
    __padding_856: [u8; 72],
    pub translucency_sort_priority: i32,
    pub translucency_sort_distance_offset: f32,
    pub runtime_virtual_textures: TArray<UPtr<URuntimeVirtualTexture>>,
    #[doc(hidden)]
    __padding_883: [u8; 3],
    pub virtual_texture_render_pass_type: ERuntimeVirtualTextureMainPassType,
    #[doc(hidden)]
    __padding_1032: [u8; 144],
    pub body_instance: FBodyInstance,
    __padding_end: [u8; 40],
}
impl UPrimitiveComponent {}
#[repr(C, align(16))]
pub struct UMeshComponent {
    #[doc(hidden)]
    __padding_1520: [u8; 1520],
    pub overlay_material: UPtr<UMaterialInterface>,
    pub overlay_material_max_draw_distance: f32,
    pub material_slots_overlay_material: TArray<UPtr<UMaterialInterface>>,
    pub flags_1552: u8,
    __padding_end: [u8; 31],
}
impl UMeshComponent {}
#[repr(C, align(16))]
pub struct UStaticMeshComponent {
    #[doc(hidden)]
    __padding_1576: [u8; 1576],
    pub forced_lod_model: i32,
    pub min_lod: i32,
    #[doc(hidden)]
    __padding_1588: [u8; 4],
    pub wireframe_color_override: crate::bindings::core_u_object::FColor,
    pub static_mesh: UPtr<UStaticMesh>,
    #[doc(hidden)]
    __padding_1608: [u8; 8],
    pub world_position_offset_disable_distance: i32,
    pub flags_1612: u8,
    #[doc(hidden)]
    __padding_1636: [u8; 23],
    pub flags_1636: u8,
    pub flags_1637: u8,
    pub flags_1638: u8,
    pub overridden_mesh_paint_texture_coordinate_index: i32,
    pub overridden_mesh_paint_texture_resolution: i32,
    pub overridden_light_map_res: i32,
    #[doc(hidden)]
    __padding_1680: [u8; 28],
    pub material_cache_tile_count: crate::bindings::core_u_object::FIntPoint,
    #[doc(hidden)]
    __padding_1704: [u8; 16],
    pub distance_field_indirect_shadow_min_visibility: f32,
    pub distance_field_self_shadow_bias: f32,
    pub streaming_distance_multiplier: f32,
    pub nanite_pixel_programmable_distance: f32,
    __padding_end: [u8; 168],
}
impl UStaticMeshComponent {}
#[repr(C, align(8))]
pub struct USubsystem {
    __padding_end: [u8; 56],
}
impl USubsystem {}
#[repr(C, align(8))]
pub struct UDynamicSubsystem {
    __padding_end: [u8; 56],
}
impl UDynamicSubsystem {}
#[repr(C, align(8))]
pub struct UBlueprintFunctionLibrary {
    __padding_end: [u8; 48],
}
impl UBlueprintFunctionLibrary {}
#[repr(C, align(16))]
pub struct UDebugDrawComponent {
    __padding_end: [u8; 1616],
}
impl UDebugDrawComponent {}
#[repr(C, align(8))]
pub struct AController {
    #[doc(hidden)]
    __padding_1144: [u8; 1144],
    pub player_state: UPtr<APlayerState>,
    __padding_end: [u8; 168],
}
impl AController {}
#[repr(C, align(8))]
pub struct UAISystemBase {
    __padding_end: [u8; 120],
}
impl UAISystemBase {}
#[repr(C, align(8))]
pub struct APawn {
    #[doc(hidden)]
    __padding_1144: [u8; 1144],
    pub flags_1144: u8,
    pub base_eye_height: f32,
    #[doc(hidden)]
    __padding_1160: [u8; 8],
    pub ai_controller_class: TSubclassOf<AController>,
    pub player_state: UPtr<APlayerState>,
    pub last_hit_by: UPtr<AController>,
    __padding_end: [u8; 80],
}
impl APawn {}
#[repr(C, align(16))]
pub struct ACharacter {
    #[doc(hidden)]
    __padding_1264: [u8; 1264],
    pub mesh: UPtr<USkeletalMeshComponent>,
    pub character_movement: UPtr<UCharacterMovementComponent>,
    pub capsule_component: UPtr<UCapsuleComponent>,
    #[doc(hidden)]
    __padding_1564: [u8; 276],
    pub crouched_eye_height: f32,
    pub flags_1568: u8,
    pub flags_1569: u8,
    pub jump_key_hold_time: f32,
    pub jump_force_time_remaining: f32,
    pub proxy_jump_force_started_time: f32,
    pub jump_max_hold_time: f32,
    pub jump_max_count: i32,
    pub jump_current_count: i32,
    pub jump_current_count_pre_jump: i32,
    __padding_end: [u8; 512],
}
impl ACharacter {}
#[repr(C, align(8))]
pub struct UWorldSubsystem {
    __padding_end: [u8; 64],
}
impl UWorldSubsystem {}
#[repr(C, align(8))]
pub struct UTickableWorldSubsystem {
    __padding_end: [u8; 80],
}
impl UTickableWorldSubsystem {}
#[repr(C, align(8))]
pub struct UNavLinkDefinition {
    __padding_end: [u8; 88],
}
impl UNavLinkDefinition {}
#[repr(C, align(8))]
pub struct UNavAreaBase {
    __padding_end: [u8; 56],
}
impl UNavAreaBase {}
#[repr(C, align(8))]
pub struct UNavCollisionBase {
    __padding_end: [u8; 152],
}
impl UNavCollisionBase {}
#[repr(C, align(8))]
pub struct UWorld {
    __padding_end: [u8; 3248],
}
impl UWorld {}
#[repr(C, align(8))]
pub struct UNavigationSystemBase {
    __padding_end: [u8; 48],
}
impl UNavigationSystemBase {}
#[repr(C, align(8))]
pub struct UNavigationSystemConfig {
    __padding_end: [u8; 112],
}
impl UNavigationSystemConfig {}
#[repr(C, align(8))]
pub struct ABrush {
    #[doc(hidden)]
    __padding_1160: [u8; 1160],
    pub brush_component: UPtr<UBrushComponent>,
    __padding_end: [u8; 40],
}
impl ABrush {}
#[repr(C, align(8))]
pub struct AVolume {
    __padding_end: [u8; 1208],
}
impl AVolume {}
#[repr(C, align(8))]
pub struct UNavigationDataChunk {
    __padding_end: [u8; 64],
}
impl UNavigationDataChunk {}
#[repr(C, align(8))]
pub struct UEngineSubsystem {
    __padding_end: [u8; 56],
}
impl UEngineSubsystem {}
#[repr(C, align(16))]
pub struct ACameraActor {
    #[doc(hidden)]
    __padding_1144: [u8; 1144],
    pub camera_component: UPtr<UCameraComponent>,
    pub scene_component: UPtr<USceneComponent>,
    __padding_end: [u8; 1976],
}
impl ACameraActor {}
#[repr(C, align(16))]
pub struct UCameraComponent {
    #[doc(hidden)]
    __padding_656: [u8; 656],
    pub field_of_view: f32,
    pub first_person_field_of_view: f32,
    pub first_person_scale: f32,
    pub ortho_width: f32,
    pub b_auto_calculate_ortho_planes: bool,
    pub auto_plane_shift: f32,
    pub ortho_near_clip_plane: f32,
    pub ortho_far_clip_plane: f32,
    pub b_update_ortho_planes: bool,
    pub b_use_camera_height_as_view_target: bool,
    pub aspect_ratio: f32,
    pub aspect_ratio_axis_constraint: EAspectRatioAxisConstraint,
    pub flags_697: u8,
    pub overscan: f32,
    #[doc(hidden)]
    __padding_720: [u8; 16],
    pub b_scale_resolution_with_overscan: bool,
    pub b_crop_overscan: bool,
    pub b_draw_frustum_allowed: bool,
    pub flags_723: u8,
    pub projection_mode: ECameraProjectionMode,
    #[doc(hidden)]
    __padding_736: [u8; 8],
    pub camera_mesh: UPtr<UStaticMesh>,
    #[doc(hidden)]
    __padding_848: [u8; 104],
    pub post_process_blend_weight: f32,
    #[doc(hidden)]
    __padding_896: [u8; 32],
    pub post_process_settings: FPostProcessSettings,
    __padding_end: [u8; 16],
}
impl UCameraComponent {}
#[repr(C, align(8))]
pub struct UAssetExportTask {
    #[doc(hidden)]
    __padding_48: [u8; 48],
    pub object: UPtr<crate::bindings::core_u_object::UObject>,
    pub exporter: UPtr<UExporter>,
    pub filename: FString,
    pub b_selected: bool,
    pub b_replace_identical: bool,
    pub b_prompt: bool,
    pub b_automated: bool,
    pub b_use_file_archive: bool,
    pub b_write_empty_files: bool,
    pub ignore_object_list: TArray<UPtr<crate::bindings::core_u_object::UObject>>,
    pub options: UPtr<crate::bindings::core_u_object::UObject>,
    pub errors: TArray<FString>,
}
impl UAssetExportTask {}
#[repr(C, align(8))]
pub struct UBlueprintExtension {
    __padding_end: [u8; 48],
}
impl UBlueprintExtension {}
#[repr(C, align(8))]
pub struct UBlueprintAsyncActionBase {
    __padding_end: [u8; 56],
}
impl UBlueprintAsyncActionBase {}
#[repr(C, align(8))]
pub struct UBlueprintCore {
    __padding_end: [u8; 88],
}
impl UBlueprintCore {}
#[repr(C, align(8))]
pub struct UBlueprint {
    __padding_end: [u8; 1432],
}
impl UBlueprint {}
#[repr(C, align(8))]
pub struct UDynamicBlueprintBinding {
    __padding_end: [u8; 48],
}
impl UDynamicBlueprintBinding {}
#[repr(C, align(8))]
pub struct UBlueprintGeneratedClass {
    __padding_end: [u8; 1640],
}
impl UBlueprintGeneratedClass {}
#[repr(C, align(8))]
pub struct UGameInstance {
    __padding_end: [u8; 528],
}
impl UGameInstance {}
#[repr(C, align(8))]
pub struct UAssetImportData {
    __padding_end: [u8; 96],
}
impl UAssetImportData {}
#[repr(C, align(16))]
pub struct USkinnedMeshComponent {
    #[doc(hidden)]
    __padding_1592: [u8; 1592],
    pub skeletal_mesh: UPtr<USkeletalMesh>,
    pub skinned_asset: UPtr<USkinnedAsset>,
    pub leader_pose_component: TWeakObjectPtr<USkinnedMeshComponent>,
    #[doc(hidden)]
    __padding_1624: [u8; 8],
    pub skin_cache_usage: TArray<ESkinCacheUsage>,
    pub b_set_mesh_deformer: bool,
    pub mesh_deformer: UPtr<UMeshDeformer>,
    pub b_always_use_mesh_deformer: bool,
    pub mesh_deformer_instance_settings: UPtr<UMeshDeformerInstanceSettings>,
    pub mesh_deformer_instance_deprecated: UPtr<UMeshDeformerInstance>,
    #[doc(hidden)]
    __padding_2152: [u8; 472],
    pub physics_asset_override: UPtr<UPhysicsAsset>,
    pub forced_lod_model: i32,
    #[doc(hidden)]
    __padding_2168: [u8; 4],
    pub min_lod_model: i32,
    #[doc(hidden)]
    __padding_2180: [u8; 8],
    pub streaming_distance_multiplier: f32,
    pub nanite_pixel_programmable_distance: f32,
    #[doc(hidden)]
    __padding_2252: [u8; 64],
    pub visibility_based_anim_tick_option: EVisibilityBasedAnimTickOption,
    #[doc(hidden)]
    __padding_2266: [u8; 13],
    pub flags_2266: u8,
    pub flags_2267: u8,
    pub flags_2268: u8,
    pub flags_2269: u8,
    pub capsule_indirect_shadow_min_visibility: f32,
    __padding_end: [u8; 316],
}
impl USkinnedMeshComponent {}
#[repr(C, align(16))]
pub struct USkeletalMeshComponent {
    #[doc(hidden)]
    __padding_2600: [u8; 2600],
    pub skeletal_mesh_asset: UPtr<USkeletalMesh>,
    #[doc(hidden)]
    __padding_2616: [u8; 8],
    pub anim_blueprint_generated_class: TSubclassOf<
        crate::bindings::core_u_object::UObject,
    >,
    pub anim_class: TSubclassOf<UAnimInstance>,
    #[doc(hidden)]
    __padding_2672: [u8; 40],
    pub animation_data: FSingleAnimationPlayData,
    #[doc(hidden)]
    __padding_3016: [u8; 320],
    pub global_anim_rate_scale: f32,
    pub kinematic_bones_update_type: EKinematicBonesUpdateToPhysics,
    pub physics_transform_update_mode: EPhysicsTransformUpdateMode,
    pub cloth_teleport_mode: crate::bindings::clothing_system_runtime_interface::EClothingTeleportMode,
    pub animation_mode: EAnimationMode,
    #[doc(hidden)]
    __padding_3025: [u8; 1],
    pub flags_3025: u8,
    pub flags_3026: u8,
    #[doc(hidden)]
    __padding_3032: [u8; 5],
    pub flags_3032: u8,
    pub flags_3033: u8,
    pub cloth_velocity_scale: f32,
    pub flags_3040: u8,
    #[doc(hidden)]
    __padding_3042: [u8; 1],
    pub flags_3042: u8,
    #[doc(hidden)]
    __padding_3048: [u8; 4],
    pub cloth_blend_weight: f32,
    pub b_wait_for_parallel_cloth_task: bool,
    #[doc(hidden)]
    __padding_3084: [u8; 28],
    pub cloth_max_distance_scale: f32,
    pub cloth_geometry_scale: f32,
    pub post_process_anim_bplod_threshold: i32,
    #[doc(hidden)]
    __padding_3392: [u8; 296],
    pub teleport_distance_threshold: f32,
    pub teleport_rotation_threshold: f32,
    #[doc(hidden)]
    __padding_4456: [u8; 1056],
    pub default_animating_rig_override: TSoftObjectPtr<
        crate::bindings::core_u_object::UObject,
    >,
    __padding_end: [u8; 168],
}
impl USkeletalMeshComponent {}
#[repr(C, align(16))]
pub struct UAnimInstance {
    __padding_end: [u8; 1136],
}
impl UAnimInstance {}
#[repr(C, align(16))]
pub struct UAnimSingleNodeInstance {
    __padding_end: [u8; 1168],
}
impl UAnimSingleNodeInstance {}
#[repr(C, align(8))]
pub struct UAnimNotify {
    #[doc(hidden)]
    __padding_48: [u8; 48],
    pub notify_color: crate::bindings::core_u_object::FColor,
    pub b_should_fire_in_editor: bool,
    __padding_end: [u8; 11],
}
impl UAnimNotify {}
#[repr(C, align(8))]
pub struct UAnimNotifyState {
    #[doc(hidden)]
    __padding_48: [u8; 48],
    pub notify_color: crate::bindings::core_u_object::FColor,
    pub b_should_fire_in_editor: bool,
    __padding_end: [u8; 3],
}
impl UAnimNotifyState {}
#[repr(C, align(8))]
pub struct UEngineCustomTimeStep {
    __padding_end: [u8; 48],
}
impl UEngineCustomTimeStep {}
#[repr(C, align(8))]
pub struct UTimecodeProvider {
    __padding_end: [u8; 56],
}
impl UTimecodeProvider {}
#[repr(C, align(8))]
pub struct UAudioEngineSubsystem {
    __padding_end: [u8; 56],
}
impl UAudioEngineSubsystem {}
#[repr(C, align(8))]
pub struct USoundBase {
    #[doc(hidden)]
    __padding_64: [u8; 64],
    pub sound_class_object: UPtr<USoundClass>,
    #[doc(hidden)]
    __padding_73: [u8; 1],
    pub flags_73: u8,
    pub flags_74: u8,
    pub virtualization_mode: EVirtualizationMode,
    #[doc(hidden)]
    __padding_168: [u8; 88],
    pub concurrency_set: TSet<UPtr<USoundConcurrency>>,
    pub concurrency_overrides: FSoundConcurrencySettings,
    #[doc(hidden)]
    __padding_380: [u8; 4],
    pub duration: f32,
    pub max_distance: f32,
    pub total_samples: f32,
    pub priority: f32,
    #[doc(hidden)]
    __padding_408: [u8; 8],
    pub sound_submix_object: UPtr<USoundSubmixBase>,
    pub sound_submix_sends: TArray<FSoundSubmixSendInfo>,
    pub source_effect_chain: UPtr<USoundEffectSourcePresetChain>,
    pub bus_sends: TArray<FSoundSourceBusSendInfo>,
    pub pre_effect_bus_sends: TArray<FSoundSourceBusSendInfo>,
    __padding_end: [u8; 48],
}
impl USoundBase {}
#[repr(C, align(8))]
pub struct USoundWave {
    #[doc(hidden)]
    __padding_608: [u8; 608],
    pub modulation_settings: FSoundModulationDefaultRoutingSettings,
    #[doc(hidden)]
    __padding_1084: [u8; 84],
    pub flags_1084: u8,
    #[doc(hidden)]
    __padding_1200: [u8; 112],
    pub subtitle_priority: f32,
    #[doc(hidden)]
    __padding_1264: [u8; 56],
    pub cue_points: TArray<crate::bindings::audio_extensions::FSoundWaveCuePoint>,
    #[doc(hidden)]
    __padding_1296: [u8; 16],
    pub subtitles: TArray<FSubtitleCue>,
    __padding_end: [u8; 768],
}
impl USoundWave {}
#[repr(C, align(16))]
pub struct USoundWaveProcedural {
    __padding_end: [u8; 2176],
}
impl USoundWaveProcedural {}
#[repr(C, align(8))]
pub struct USoundEffectPreset {
    __padding_end: [u8; 112],
}
impl USoundEffectPreset {}
#[repr(C, align(8))]
pub struct USoundEffectSubmixPreset {
    __padding_end: [u8; 112],
}
impl USoundEffectSubmixPreset {}
#[repr(C, align(8))]
pub struct UStreamableRenderAsset {
    #[doc(hidden)]
    __padding_80: [u8; 80],
    pub num_cinematic_mip_levels: i32,
    #[doc(hidden)]
    __padding_240: [u8; 156],
    pub flags_240: u8,
    __padding_end: [u8; 15],
}
impl UStreamableRenderAsset {}
#[repr(C, align(16))]
pub struct UTexture {
    #[doc(hidden)]
    __padding_664: [u8; 664],
    pub adjust_brightness: f32,
    pub adjust_brightness_curve: f32,
    pub adjust_vibrance: f32,
    pub adjust_saturation: f32,
    pub adjust_rgb_curve: f32,
    pub adjust_hue: f32,
    pub adjust_min_alpha: f32,
    pub adjust_max_alpha: f32,
    pub flags_696: u8,
    #[doc(hidden)]
    __padding_700: [u8; 3],
    pub lossy_compression_amount: ETextureLossyCompressionAmount,
    pub oodle_texture_sdk_version: FName,
    pub max_texture_size: i32,
    pub compression_quality: ETextureCompressionQuality,
    #[doc(hidden)]
    __padding_744: [u8; 23],
    pub b_do_scale_mips_for_alpha_coverage: bool,
    pub alpha_coverage_thresholds: crate::bindings::core_u_object::FVector4,
    pub b_use_new_mip_filter: bool,
    #[doc(hidden)]
    __padding_788: [u8; 3],
    pub flags_788: u8,
    #[doc(hidden)]
    __padding_792: [u8; 3],
    pub power_of_two_mode: ETexturePowerOfTwoSetting,
    pub padding_color: crate::bindings::core_u_object::FColor,
    pub b_pad_with_border_color: bool,
    pub resize_during_build_x: i32,
    pub resize_during_build_y: i32,
    pub b_chroma_key_texture: bool,
    pub chroma_key_threshold: f32,
    pub chroma_key_color: crate::bindings::core_u_object::FColor,
    pub mip_gen_settings: TextureMipGenSettings,
    pub composite_texture: UPtr<UTexture>,
    #[doc(hidden)]
    __padding_848: [u8; 8],
    pub composite_texture_mode: ECompositeTextureMode,
    pub composite_power: f32,
    #[doc(hidden)]
    __padding_876: [u8; 20],
    pub lod_bias: i32,
    pub compression_settings: TextureCompressionSettings,
    pub filter: TextureFilter,
    pub mip_load_options: ETextureMipLoadOptions,
    pub virtual_texture_streaming_priority: crate::bindings::render_core::EVTProducerPriority,
    #[doc(hidden)]
    __padding_885: [u8; 1],
    pub cook_platform_tiling_settings: TextureCookPlatformTilingSettings,
    pub b_oodle_preserve_extremes: bool,
    pub lod_group: TextureGroup,
    #[doc(hidden)]
    __padding_977: [u8; 89],
    pub availability: ETextureAvailability,
    pub flags_978: u8,
    pub source_color_settings: FTextureSourceColorSettings,
    #[doc(hidden)]
    __padding_1100: [u8; 36],
    pub flags_1100: u8,
    __padding_end: [u8; 147],
}
impl UTexture {}
#[repr(C, align(8))]
pub struct UAssetUserData {
    __padding_end: [u8; 48],
}
impl UAssetUserData {}
#[repr(C, align(8))]
pub struct UExporter {
    #[doc(hidden)]
    __padding_48: [u8; 48],
    pub supported_class: TSubclassOf<crate::bindings::core_u_object::UObject>,
    #[doc(hidden)]
    __padding_64: [u8; 8],
    pub format_extension: TArray<FString>,
    pub format_description: TArray<FString>,
    #[doc(hidden)]
    __padding_104: [u8; 8],
    pub flags_104: u8,
    pub export_task: UPtr<UAssetExportTask>,
    __padding_end: [u8; 8],
}
impl UExporter {}
#[repr(C, align(8))]
pub struct UCommandlet {
    __padding_end: [u8; 136],
}
impl UCommandlet {}
#[repr(C, align(8))]
pub struct UBrushBuilder {
    __padding_end: [u8; 136],
}
impl UBrushBuilder {}
#[repr(C, align(8))]
pub struct UEngine {
    __padding_end: [u8; 6832],
}
impl UEngine {}
#[repr(C, align(8))]
pub struct UActorElementAssetDataInterface {
    __padding_end: [u8; 56],
}
impl UActorElementAssetDataInterface {}
#[repr(C, align(8))]
pub struct UActorElementSelectionInterface {
    __padding_end: [u8; 56],
}
impl UActorElementSelectionInterface {}
#[repr(C, align(8))]
pub struct UActorElementWorldInterface {
    __padding_end: [u8; 56],
}
impl UActorElementWorldInterface {}
#[repr(C, align(8))]
pub struct UComponentElementSelectionInterface {
    __padding_end: [u8; 56],
}
impl UComponentElementSelectionInterface {}
#[repr(C, align(8))]
pub struct UComponentElementWorldInterface {
    __padding_end: [u8; 56],
}
impl UComponentElementWorldInterface {}
#[repr(C, align(8))]
pub struct UObjectElementSelectionInterface {
    __padding_end: [u8; 56],
}
impl UObjectElementSelectionInterface {}
#[repr(C, align(8))]
pub struct USMInstanceProxyEditingObject {
    __padding_end: [u8; 48],
}
impl USMInstanceProxyEditingObject {}
#[repr(C, align(8))]
pub struct USMInstanceElementSelectionInterface {
    __padding_end: [u8; 56],
}
impl USMInstanceElementSelectionInterface {}
#[repr(C, align(8))]
pub struct USMInstanceElementWorldInterface {
    __padding_end: [u8; 56],
}
impl USMInstanceElementWorldInterface {}
#[repr(C, align(16))]
pub struct UInstancedPlacemenClientSettings {
    __padding_end: [u8; 800],
}
impl UInstancedPlacemenClientSettings {}
#[repr(C, align(8))]
pub struct UMaterialInterface {
    #[doc(hidden)]
    __padding_72: [u8; 72],
    pub subsurface_profile: UPtr<USubsurfaceProfile>,
    #[doc(hidden)]
    __padding_112: [u8; 32],
    pub neural_profile: UPtr<UNeuralProfile>,
    __padding_end: [u8; 720],
}
impl UMaterialInterface {}
#[repr(C, align(8))]
pub struct UMaterial {
    #[doc(hidden)]
    __padding_936: [u8; 936],
    pub material_domain: EMaterialDomain,
    pub blend_mode: EBlendMode,
    #[doc(hidden)]
    __padding_939: [u8; 1],
    pub material_decal_response: EMaterialDecalResponse,
    #[doc(hidden)]
    __padding_1016: [u8; 76],
    pub displacement_scaling: FDisplacementScaling,
    pub displacement_fade_range: FDisplacementFadeRange,
    #[doc(hidden)]
    __padding_1116: [u8; 84],
    pub flags_1116: u8,
    pub flags_1117: u8,
    pub flags_1118: u8,
    #[doc(hidden)]
    __padding_1120: [u8; 1],
    pub flags_1120: u8,
    #[doc(hidden)]
    __padding_1124: [u8; 3],
    pub flags_1124: u8,
    #[doc(hidden)]
    __padding_1128: [u8; 3],
    pub flags_1128: u8,
    #[doc(hidden)]
    __padding_1132: [u8; 3],
    pub flags_1132: u8,
    pub float_precision_mode: EMaterialFloatPrecisionMode,
    pub flags_1134: u8,
    #[doc(hidden)]
    __padding_1136: [u8; 1],
    pub flags_1136: u8,
    #[doc(hidden)]
    __padding_1140: [u8; 3],
    pub flags_1140: u8,
    #[doc(hidden)]
    __padding_1175: [u8; 34],
    pub blendable_location: EBlendableLocation,
    pub flags_1176: u8,
    pub user_scene_texture: FName,
    pub user_texture_divisor: crate::bindings::core_u_object::FIntPoint,
    pub resolution_relative_to_input: FName,
    #[doc(hidden)]
    __padding_1220: [u8; 8],
    pub blendable_priority: i32,
    pub flags_1224: u8,
    #[doc(hidden)]
    __padding_1240: [u8; 12],
    pub max_world_position_offset_displacement: f32,
    pub b_always_evaluate_world_position_offset: bool,
    __padding_end: [u8; 2155],
}
impl UMaterial {}
#[repr(C, align(8))]
pub struct ASkeletalMeshActor {
    #[doc(hidden)]
    __padding_1136: [u8; 1136],
    pub flags_1136: u8,
    pub skeletal_mesh_component: UPtr<USkeletalMeshComponent>,
    __padding_end: [u8; 112],
}
impl ASkeletalMeshActor {}
#[repr(C, align(8))]
pub struct UThumbnailInfo {
    __padding_end: [u8; 48],
}
impl UThumbnailInfo {}
#[repr(C, align(16))]
pub struct UInstancedStaticMeshComponent {
    #[doc(hidden)]
    __padding_2036: [u8; 2036],
    pub instancing_random_seed: i32,
    #[doc(hidden)]
    __padding_2072: [u8; 32],
    pub instance_lod_distance_scale: f32,
    pub instance_min_draw_distance: i32,
    pub instance_start_cull_distance: i32,
    pub instance_end_cull_distance: i32,
    pub flags_2088: u8,
    __padding_end: [u8; 775],
}
impl UInstancedStaticMeshComponent {}
#[repr(C, align(16))]
pub struct UHierarchicalInstancedStaticMeshComponent {
    __padding_end: [u8; 3280],
}
impl UHierarchicalInstancedStaticMeshComponent {}
#[repr(C, align(8))]
pub struct APartitionActor {
    __padding_end: [u8; 1144],
}
impl APartitionActor {}
#[repr(C, align(8))]
pub struct AISMPartitionActor {
    __padding_end: [u8; 1296],
}
impl AISMPartitionActor {}
#[repr(C, align(8))]
pub struct AStaticMeshActor {
    #[doc(hidden)]
    __padding_1136: [u8; 1136],
    pub static_mesh_component: UPtr<UStaticMeshComponent>,
    __padding_end: [u8; 8],
}
impl AStaticMeshActor {}
#[repr(C, align(8))]
pub struct AInfo {
    __padding_end: [u8; 1144],
}
impl AInfo {}
#[repr(C, align(8))]
pub struct UMaterialInstance {
    #[doc(hidden)]
    __padding_840: [u8; 840],
    pub phys_material: UPtr<crate::bindings::physics_core::UPhysicalMaterial>,
    #[doc(hidden)]
    __padding_912: [u8; 64],
    pub parent: UPtr<UMaterialInterface>,
    #[doc(hidden)]
    __padding_1000: [u8; 80],
    pub flags_1000: u8,
    #[doc(hidden)]
    __padding_1003: [u8; 2],
    pub blendable_location_override: EBlendableLocation,
    pub blendable_priority_override: i32,
    #[doc(hidden)]
    __padding_1040: [u8; 32],
    pub scalar_parameter_values: TArray<FScalarParameterValue>,
    pub vector_parameter_values: TArray<FVectorParameterValue>,
    pub double_vector_parameter_values: TArray<FDoubleVectorParameterValue>,
    pub texture_parameter_values: TArray<FTextureParameterValue>,
    pub texture_collection_parameter_values: TArray<FTextureCollectionParameterValue>,
    pub parameter_collection_parameter_values: TArray<
        FParameterCollectionParameterValue,
    >,
    pub runtime_virtual_texture_parameter_values: TArray<
        FRuntimeVirtualTextureParameterValue,
    >,
    pub sparse_volume_texture_parameter_values: TArray<
        FSparseVolumeTextureParameterValue,
    >,
    pub font_parameter_values: TArray<FFontParameterValue>,
    __padding_end: [u8; 640],
}
impl UMaterialInstance {}
#[repr(C, align(8))]
pub struct UMaterialInstanceConstant {
    __padding_end: [u8; 1848],
}
impl UMaterialInstanceConstant {}
#[repr(C, align(16))]
pub struct UTexture2D {
    #[doc(hidden)]
    __padding_1245: [u8; 1245],
    pub address_x: TextureAddress,
    pub address_y: TextureAddress,
    __padding_end: [u8; 113],
}
impl UTexture2D {}
#[repr(C, align(8))]
pub struct UHLODBuilder {
    __padding_end: [u8; 72],
}
impl UHLODBuilder {}
#[repr(C, align(8))]
pub struct UTextureMipDataProviderFactory {
    __padding_end: [u8; 48],
}
impl UTextureMipDataProviderFactory {}
#[repr(C, align(8))]
pub struct UTextureAllMipDataProviderFactory {
    __padding_end: [u8; 48],
}
impl UTextureAllMipDataProviderFactory {}
pub struct IActiveSoundUpdateInterface {}
#[repr(C, align(8))]
pub struct UActiveSoundUpdateInterface {
    __padding_end: [u8; 48],
}
impl UActiveSoundUpdateInterface {}
#[repr(C, align(8))]
pub struct UChannel {
    __padding_end: [u8; 120],
}
impl UChannel {}
#[repr(C, align(8))]
pub struct UActorChannel {
    __padding_end: [u8; 680],
}
impl UActorChannel {}
pub struct IActorInstanceManagerInterface {}
#[repr(C, align(8))]
pub struct UActorInstanceManagerInterface {
    __padding_end: [u8; 48],
}
impl UActorInstanceManagerInterface {}
#[repr(C, align(8))]
pub struct UAnimationAssetExtensions {
    __padding_end: [u8; 48],
}
impl UAnimationAssetExtensions {}
#[repr(C, align(8))]
pub struct UAnimBlueprintClassSubsystem_PropertyAccess {
    __padding_end: [u8; 48],
}
impl UAnimBlueprintClassSubsystem_PropertyAccess {}
pub struct IInterface_AnimCurveMetaData {}
#[repr(C, align(8))]
pub struct UInterface_AnimCurveMetaData {
    __padding_end: [u8; 48],
}
impl UInterface_AnimCurveMetaData {}
#[repr(C, align(8))]
pub struct UAnimCurveMetaData {
    __padding_end: [u8; 176],
}
impl UAnimCurveMetaData {}
#[repr(C, align(8))]
pub struct UAnimationDataModelNotifiesExtensions {
    __padding_end: [u8; 48],
}
impl UAnimationDataModelNotifiesExtensions {}
pub struct IAnimLayerInterface {}
#[repr(C, align(8))]
pub struct UAnimLayerInterface {
    __padding_end: [u8; 48],
}
impl UAnimLayerInterface {}
#[repr(C, align(8))]
pub struct UAssetManagerSettings {
    __padding_end: [u8; 304],
}
impl UAssetManagerSettings {}
#[repr(C, align(8))]
pub struct UAsyncPhysicsData {
    __padding_end: [u8; 56],
}
impl UAsyncPhysicsData {}
pub struct IAudioPanelWidgetInterface {}
#[repr(C, align(8))]
pub struct UAudioPanelWidgetInterface {
    __padding_end: [u8; 48],
}
impl UAudioPanelWidgetInterface {}
#[repr(C, align(8))]
pub struct AAutoRTFMTestActor {
    __padding_end: [u8; 1200],
}
impl AAutoRTFMTestActor {}
#[repr(C, align(8))]
pub struct AAutoRTFMTestAnotherActor {
    __padding_end: [u8; 1144],
}
impl AAutoRTFMTestAnotherActor {}
#[repr(C, align(8))]
pub struct UBodySetup {
    __padding_end: [u8; 912],
}
impl UBodySetup {}
#[repr(C, align(8))]
pub struct UAutoRTFMTestBodySetup {
    __padding_end: [u8; 920],
}
impl UAutoRTFMTestBodySetup {}
#[repr(C, align(16))]
pub struct UCameraShakeBase {
    #[doc(hidden)]
    __padding_52: [u8; 52],
    pub shake_scale: f32,
    __padding_end: [u8; 168],
}
impl UCameraShakeBase {}
#[repr(C, align(16))]
pub struct UAutoRTFMTestCameraShake {
    __padding_end: [u8; 224],
}
impl UAutoRTFMTestCameraShake {}
#[repr(C, align(16))]
pub struct UChildActorComponent {
    #[doc(hidden)]
    __padding_656: [u8; 656],
    pub child_actor_class: TSubclassOf<AActor>,
    pub child_actor: UPtr<AActor>,
    __padding_end: [u8; 80],
}
impl UChildActorComponent {}
#[repr(C, align(16))]
pub struct UAutoRTFMTestChildActorComponent {
    __padding_end: [u8; 752],
}
impl UAutoRTFMTestChildActorComponent {}
#[repr(C, align(8))]
pub struct ULevel {
    __padding_end: [u8; 2888],
}
impl ULevel {}
#[repr(C, align(8))]
pub struct UAutoRTFMTestLevel {
    __padding_end: [u8; 2896],
}
impl UAutoRTFMTestLevel {}
#[repr(C, align(8))]
pub struct UAutoRTFMTestObject {
    __padding_end: [u8; 56],
}
impl UAutoRTFMTestObject {}
#[repr(C, align(16))]
pub struct UAutoRTFMTestPrimitiveComponent {
    __padding_end: [u8; 1584],
}
impl UAutoRTFMTestPrimitiveComponent {}
pub struct IBlendableInterface {}
#[repr(C, align(8))]
pub struct UBlendableInterface {
    __padding_end: [u8; 48],
}
impl UBlendableInterface {}
#[repr(C, align(8))]
pub struct UBlueprintSpringMathLibrary {
    __padding_end: [u8; 48],
}
impl UBlueprintSpringMathLibrary {}
#[repr(C, align(8))]
pub struct UBookmarkBase {
    __padding_end: [u8; 48],
}
impl UBookmarkBase {}
#[repr(C, align(8))]
pub struct UBookMark2D {
    __padding_end: [u8; 64],
}
impl UBookMark2D {}
#[repr(C, align(8))]
pub struct AReflectionCapture {
    #[doc(hidden)]
    __padding_1136: [u8; 1136],
    pub capture_component: UPtr<UReflectionCaptureComponent>,
    __padding_end: [u8; 16],
}
impl AReflectionCapture {}
#[repr(C, align(8))]
pub struct ABoxReflectionCapture {
    __padding_end: [u8; 1160],
}
impl ABoxReflectionCapture {}
#[repr(C, align(16))]
pub struct UReflectionCaptureComponent {
    #[doc(hidden)]
    __padding_664: [u8; 664],
    pub reflection_source_type: EReflectionSourceType,
    pub cubemap: UPtr<UTextureCube>,
    pub source_cubemap_angle: f32,
    pub brightness: f32,
    pub capture_offset: crate::bindings::core_u_object::FVector,
    __padding_end: [u8; 56],
}
impl UReflectionCaptureComponent {}
#[repr(C, align(16))]
pub struct UBoxReflectionCaptureComponent {
    #[doc(hidden)]
    __padding_768: [u8; 768],
    pub box_transition_distance: f32,
    __padding_end: [u8; 28],
}
impl UBoxReflectionCaptureComponent {}
#[repr(C, align(8))]
pub struct UBuiltInAttributesExtensions {
    __padding_end: [u8; 48],
}
impl UBuiltInAttributesExtensions {}
#[repr(C, align(8))]
pub struct UPlayer {
    __padding_end: [u8; 80],
}
impl UPlayer {}
#[repr(C, align(16))]
pub struct UNetConnection {
    __padding_end: [u8; 8048],
}
impl UNetConnection {}
#[repr(C, align(16))]
pub struct UChildConnection {
    __padding_end: [u8; 8064],
}
impl UChildConnection {}
#[repr(C, align(8))]
pub struct UPlatformInterfaceBase {
    __padding_end: [u8; 64],
}
impl UPlatformInterfaceBase {}
#[repr(C, align(8))]
pub struct UCloudStorageBase {
    __padding_end: [u8; 88],
}
impl UCloudStorageBase {}
#[repr(C, align(8))]
pub struct UControlChannel {
    __padding_end: [u8; 144],
}
impl UControlChannel {}
#[repr(C, align(16))]
pub struct UDemoNetConnection {
    __padding_end: [u8; 8128],
}
impl UDemoNetConnection {}
#[repr(C, align(8))]
pub struct UPendingNetGame {
    __padding_end: [u8; 200],
}
impl UPendingNetGame {}
#[repr(C, align(8))]
pub struct UDemoPendingNetGame {
    __padding_end: [u8; 200],
}
impl UDemoPendingNetGame {}
#[repr(C, align(8))]
pub struct UDeviceProfileFragment {
    __padding_end: [u8; 64],
}
impl UDeviceProfileFragment {}
#[repr(C, align(8))]
pub struct UDialogueSoundWaveProxy {
    __padding_end: [u8; 560],
}
impl UDialogueSoundWaveProxy {}
#[repr(C, align(8))]
pub struct ALight {
    #[doc(hidden)]
    __padding_1136: [u8; 1136],
    pub light_component: UPtr<ULightComponent>,
    __padding_end: [u8; 8],
}
impl ALight {}
#[repr(C, align(8))]
pub struct ADirectionalLight {
    __padding_end: [u8; 1168],
}
impl ADirectionalLight {}
#[repr(C, align(8))]
pub struct UDistribution {
    __padding_end: [u8; 56],
}
impl UDistribution {}
#[repr(C, align(8))]
pub struct UDistributionFloat {
    __padding_end: [u8; 64],
}
impl UDistributionFloat {}
#[repr(C, align(8))]
pub struct UDistributionFloatConstant {
    __padding_end: [u8; 72],
}
impl UDistributionFloatConstant {}
#[repr(C, align(8))]
pub struct UDistributionFloatConstantCurve {
    __padding_end: [u8; 88],
}
impl UDistributionFloatConstantCurve {}
#[repr(C, align(8))]
pub struct UDistributionFloatParameterBase {
    __padding_end: [u8; 104],
}
impl UDistributionFloatParameterBase {}
#[repr(C, align(8))]
pub struct UDistributionFloatParticleParameter {
    __padding_end: [u8; 104],
}
impl UDistributionFloatParticleParameter {}
#[repr(C, align(8))]
pub struct UDistributionFloatUniform {
    __padding_end: [u8; 72],
}
impl UDistributionFloatUniform {}
#[repr(C, align(8))]
pub struct UDistributionFloatUniformCurve {
    __padding_end: [u8; 88],
}
impl UDistributionFloatUniformCurve {}
#[repr(C, align(8))]
pub struct UDistributionVector {
    __padding_end: [u8; 64],
}
impl UDistributionVector {}
#[repr(C, align(8))]
pub struct UDistributionVectorConstant {
    __padding_end: [u8; 96],
}
impl UDistributionVectorConstant {}
#[repr(C, align(8))]
pub struct UDistributionVectorConstantCurve {
    __padding_end: [u8; 96],
}
impl UDistributionVectorConstantCurve {}
#[repr(C, align(8))]
pub struct UDistributionVectorParameterBase {
    __padding_end: [u8; 216],
}
impl UDistributionVectorParameterBase {}
#[repr(C, align(8))]
pub struct UDistributionVectorParticleParameter {
    __padding_end: [u8; 216],
}
impl UDistributionVectorParticleParameter {}
#[repr(C, align(8))]
pub struct UDistributionVectorUniform {
    __padding_end: [u8; 128],
}
impl UDistributionVectorUniform {}
#[repr(C, align(8))]
pub struct UDistributionVectorUniformCurve {
    __padding_end: [u8; 104],
}
impl UDistributionVectorUniformCurve {}
#[repr(C, align(8))]
pub struct AEmitter {
    #[doc(hidden)]
    __padding_1136: [u8; 1136],
    pub particle_system_component: UPtr<UParticleSystemComponent>,
    pub flags_1144: u8,
    __padding_end: [u8; 119],
}
impl AEmitter {}
#[repr(C, align(16))]
pub struct AEmitterCameraLensEffectBase {
    __padding_end: [u8; 1424],
}
impl AEmitterCameraLensEffectBase {}
#[repr(C, align(8))]
pub struct UViewModeUtils {
    __padding_end: [u8; 48],
}
impl UViewModeUtils {}
#[repr(C, align(8))]
pub struct UEngineBaseTypes {
    __padding_end: [u8; 48],
}
impl UEngineBaseTypes {}
#[repr(C, align(8))]
pub struct AExponentialHeightFog {
    #[doc(hidden)]
    __padding_1144: [u8; 1144],
    pub component: UPtr<UExponentialHeightFogComponent>,
    __padding_end: [u8; 8],
}
impl AExponentialHeightFog {}
#[repr(C, align(8))]
pub struct UFontImportOptions {
    __padding_end: [u8; 224],
}
impl UFontImportOptions {}
#[repr(C, align(8))]
pub struct UForceFeedbackAttenuation {
    #[doc(hidden)]
    __padding_48: [u8; 48],
    pub attenuation: FForceFeedbackAttenuationSettings,
}
impl UForceFeedbackAttenuation {}
#[repr(C, align(8))]
pub struct ASpotLight {
    #[doc(hidden)]
    __padding_1152: [u8; 1152],
    pub spot_light_component: UPtr<USpotLightComponent>,
    __padding_end: [u8; 8],
}
impl ASpotLight {}
#[repr(C, align(8))]
pub struct AGeneratedMeshAreaLight {
    __padding_end: [u8; 1168],
}
impl AGeneratedMeshAreaLight {}
#[repr(C, align(8))]
pub struct UHapticFeedbackEffect_Base {
    __padding_end: [u8; 48],
}
impl UHapticFeedbackEffect_Base {}
#[repr(C, align(8))]
pub struct UHapticFeedbackEffect_Buffer {
    __padding_end: [u8; 72],
}
impl UHapticFeedbackEffect_Buffer {}
#[repr(C, align(8))]
pub struct UHapticFeedbackEffect_Curve {
    __padding_end: [u8; 320],
}
impl UHapticFeedbackEffect_Curve {}
#[repr(C, align(8))]
pub struct UHapticFeedbackEffect_SoundWave {
    __padding_end: [u8; 80],
}
impl UHapticFeedbackEffect_SoundWave {}
pub struct IAssetRegistryTagProviderInterface {}
#[repr(C, align(8))]
pub struct UAssetRegistryTagProviderInterface {
    __padding_end: [u8; 48],
}
impl UAssetRegistryTagProviderInterface {}
#[repr(C, align(8))]
pub struct UInGameAdManager {
    __padding_end: [u8; 104],
}
impl UInGameAdManager {}
pub struct IInterface_ActorSubobject {}
#[repr(C, align(8))]
pub struct UInterface_ActorSubobject {
    __padding_end: [u8; 48],
}
impl UInterface_ActorSubobject {}
pub struct IInterface_AssetUserData {}
#[repr(C, align(8))]
pub struct UInterface_AssetUserData {
    __padding_end: [u8; 48],
}
impl UInterface_AssetUserData {}
pub struct IInterface_AsyncCompilation {}
#[repr(C, align(8))]
pub struct UInterface_AsyncCompilation {
    __padding_end: [u8; 48],
}
impl UInterface_AsyncCompilation {}
pub struct IBoneReferenceSkeletonProvider {}
#[repr(C, align(8))]
pub struct UBoneReferenceSkeletonProvider {
    __padding_end: [u8; 48],
}
impl UBoneReferenceSkeletonProvider {}
pub struct IInterface_CollisionDataProvider {}
#[repr(C, align(8))]
pub struct UInterface_CollisionDataProvider {
    __padding_end: [u8; 48],
}
impl UInterface_CollisionDataProvider {}
pub struct IInterface_PostProcessVolume {}
#[repr(C, align(8))]
pub struct UInterface_PostProcessVolume {
    __padding_end: [u8; 48],
}
impl UInterface_PostProcessVolume {}
pub struct IInterface_PreviewMeshProvider {}
#[repr(C, align(8))]
pub struct UInterface_PreviewMeshProvider {
    __padding_end: [u8; 48],
}
impl UInterface_PreviewMeshProvider {}
pub struct IPhysicsComponent {}
#[repr(C, align(8))]
pub struct UPhysicsComponent {
    __padding_end: [u8; 48],
}
impl UPhysicsComponent {}
pub struct IISMPartitionInstanceManager {}
#[repr(C, align(8))]
pub struct UISMPartitionInstanceManager {
    __padding_end: [u8; 48],
}
impl UISMPartitionInstanceManager {}
pub struct IISMPartitionInstanceManagerProvider {}
#[repr(C, align(8))]
pub struct UISMPartitionInstanceManagerProvider {
    __padding_end: [u8; 48],
}
impl UISMPartitionInstanceManagerProvider {}
pub struct IViewportSelectableObject {}
#[repr(C, align(8))]
pub struct UViewportSelectableObject {
    __padding_end: [u8; 48],
}
impl UViewportSelectableObject {}
pub struct IWorldPartitionObjectResolver {}
#[repr(C, align(8))]
pub struct UWorldPartitionObjectResolver {
    __padding_end: [u8; 48],
}
impl UWorldPartitionObjectResolver {}
pub struct ILevelInstanceEditorPivotInterface {}
#[repr(C, align(8))]
pub struct ULevelInstanceEditorPivotInterface {
    __padding_end: [u8; 48],
}
impl ULevelInstanceEditorPivotInterface {}
#[repr(C, align(16))]
pub struct ULevelStreaming {
    #[doc(hidden)]
    __padding_64: [u8; 64],
    pub world_asset: TSoftObjectPtr<UWorld>,
    pub streaming_priority: i32,
    #[doc(hidden)]
    __padding_176: [u8; 48],
    pub level_transform: crate::bindings::core_u_object::FTransform,
    #[doc(hidden)]
    __padding_284: [u8; 12],
    pub level_lod_index: i32,
    pub flags_288: u8,
    #[doc(hidden)]
    __padding_346: [u8; 57],
    pub flags_346: u8,
    __padding_end: [u8; 245],
}
impl ULevelStreaming {}
#[repr(C, align(16))]
pub struct ULevelStreamingAlwaysLoaded {
    __padding_end: [u8; 592],
}
impl ULevelStreamingAlwaysLoaded {}
#[repr(C, align(16))]
pub struct ULevelStreamingDynamic {
    __padding_end: [u8; 592],
}
impl ULevelStreamingDynamic {}
#[repr(C, align(16))]
pub struct ULevelStreamingPersistent {
    __padding_end: [u8; 592],
}
impl ULevelStreamingPersistent {}
#[repr(C, align(16))]
pub struct ULightComponentBase {
    #[doc(hidden)]
    __padding_692: [u8; 692],
    pub intensity: f32,
    pub light_color: crate::bindings::core_u_object::FColor,
    pub flags_700: u8,
    #[doc(hidden)]
    __padding_704: [u8; 3],
    pub cast_raytraced_shadow: ECastRayTracedShadow,
    #[doc(hidden)]
    __padding_708: [u8; 3],
    pub flags_708: u8,
    pub deep_shadow_layer_distribution: f32,
    pub indirect_lighting_intensity: f32,
    pub volumetric_scattering_intensity: f32,
    pub samples_per_pixel: i32,
    __padding_end: [u8; 40],
}
impl ULightComponentBase {}
#[repr(C, align(16))]
pub struct ULightMapTexture2D {
    __padding_end: [u8; 1376],
}
impl ULightMapTexture2D {}
#[repr(C, align(8))]
pub struct ALightmassPortal {
    #[doc(hidden)]
    __padding_1136: [u8; 1136],
    pub portal_component: UPtr<ULightmassPortalComponent>,
    __padding_end: [u8; 8],
}
impl ALightmassPortal {}
#[repr(C, align(16))]
pub struct ULightmassPortalComponent {
    __padding_end: [u8; 672],
}
impl ULightmassPortalComponent {}
#[repr(C, align(8))]
pub struct ALocalFogVolume {
    #[doc(hidden)]
    __padding_1144: [u8; 1144],
    pub local_fog_volume_volume: UPtr<ULocalFogVolumeComponent>,
}
impl ALocalFogVolume {}
#[repr(C, align(8))]
pub struct UMapBuildDataRegistry {
    __padding_end: [u8; 736],
}
impl UMapBuildDataRegistry {}
#[repr(C, align(8))]
pub struct UMaterialCacheStackProvider {
    __padding_end: [u8; 48],
}
impl UMaterialCacheStackProvider {}
#[repr(C, align(8))]
pub struct UMaterialExpressionAbs {
    __padding_end: [u8; 248],
}
impl UMaterialExpressionAbs {}
#[repr(C, align(8))]
pub struct UMaterialExpressionAbsorptionMediumMaterialOutput {
    __padding_end: [u8; 248],
}
impl UMaterialExpressionAbsorptionMediumMaterialOutput {}
#[repr(C, align(8))]
pub struct UMaterialExpressionActorPositionWS {
    __padding_end: [u8; 208],
}
impl UMaterialExpressionActorPositionWS {}
#[repr(C, align(8))]
pub struct UMaterialExpressionAdd {
    __padding_end: [u8; 304],
}
impl UMaterialExpressionAdd {}
#[repr(C, align(8))]
pub struct UMaterialExpressionAggregate {
    __padding_end: [u8; 336],
}
impl UMaterialExpressionAggregate {}
#[repr(C, align(8))]
pub struct UMaterialExpressionTextureBase {
    #[doc(hidden)]
    __padding_200: [u8; 200],
    pub texture: UPtr<UTexture>,
    pub sampler_type: EMaterialSamplerType,
    __padding_end: [u8; 7],
}
impl UMaterialExpressionTextureBase {}
#[repr(C, align(8))]
pub struct UMaterialExpressionTextureSample {
    __padding_end: [u8; 584],
}
impl UMaterialExpressionTextureSample {}
#[repr(C, align(8))]
pub struct UMaterialExpressionTextureSampleParameter {
    __padding_end: [u8; 632],
}
impl UMaterialExpressionTextureSampleParameter {}
#[repr(C, align(8))]
pub struct UMaterialExpressionTextureSampleParameter2D {
    __padding_end: [u8; 632],
}
impl UMaterialExpressionTextureSampleParameter2D {}
#[repr(C, align(8))]
pub struct UMaterialExpressionAntialiasedTextureMask {
    __padding_end: [u8; 640],
}
impl UMaterialExpressionAntialiasedTextureMask {}
#[repr(C, align(8))]
pub struct UMaterialExpressionAppendVector {
    __padding_end: [u8; 296],
}
impl UMaterialExpressionAppendVector {}
#[repr(C, align(8))]
pub struct UMaterialExpressionArccosine {
    __padding_end: [u8; 248],
}
impl UMaterialExpressionArccosine {}
#[repr(C, align(8))]
pub struct UMaterialExpressionArccosineFast {
    __padding_end: [u8; 248],
}
impl UMaterialExpressionArccosineFast {}
#[repr(C, align(8))]
pub struct UMaterialExpressionArcsine {
    __padding_end: [u8; 248],
}
impl UMaterialExpressionArcsine {}
#[repr(C, align(8))]
pub struct UMaterialExpressionArcsineFast {
    __padding_end: [u8; 248],
}
impl UMaterialExpressionArcsineFast {}
#[repr(C, align(8))]
pub struct UMaterialExpressionArctangent {
    __padding_end: [u8; 248],
}
impl UMaterialExpressionArctangent {}
#[repr(C, align(8))]
pub struct UMaterialExpressionArctangent2 {
    __padding_end: [u8; 296],
}
impl UMaterialExpressionArctangent2 {}
#[repr(C, align(8))]
pub struct UMaterialExpressionArctangent2Fast {
    __padding_end: [u8; 296],
}
impl UMaterialExpressionArctangent2Fast {}
#[repr(C, align(8))]
pub struct UMaterialExpressionArctangentFast {
    __padding_end: [u8; 248],
}
impl UMaterialExpressionArctangentFast {}
#[repr(C, align(8))]
pub struct UMaterialExpressionExternalCodeBase {
    __padding_end: [u8; 216],
}
impl UMaterialExpressionExternalCodeBase {}
#[repr(C, align(8))]
pub struct UMaterialExpressionAtmosphericFogColor {
    __padding_end: [u8; 272],
}
impl UMaterialExpressionAtmosphericFogColor {}
#[repr(C, align(8))]
pub struct UMaterialExpressionAtmosphericLightColor {
    __padding_end: [u8; 216],
}
impl UMaterialExpressionAtmosphericLightColor {}
#[repr(C, align(8))]
pub struct UMaterialExpressionAtmosphericLightVector {
    __padding_end: [u8; 216],
}
impl UMaterialExpressionAtmosphericLightVector {}
#[repr(C, align(8))]
pub struct UMaterialExpressionBindlessSwitch {
    __padding_end: [u8; 296],
}
impl UMaterialExpressionBindlessSwitch {}
#[repr(C, align(8))]
pub struct UMaterialExpressionBlackBody {
    __padding_end: [u8; 248],
}
impl UMaterialExpressionBlackBody {}
#[repr(C, align(8))]
pub struct UMaterialExpressionBlend {
    __padding_end: [u8; 352],
}
impl UMaterialExpressionBlend {}
#[repr(C, align(8))]
pub struct UMaterialExpressionBlendMaterialAttributes {
    __padding_end: [u8; 368],
}
impl UMaterialExpressionBlendMaterialAttributes {}
#[repr(C, align(8))]
pub struct UMaterialExpressionLegacyBlendMaterialAttributes {
    __padding_end: [u8; 568],
}
impl UMaterialExpressionLegacyBlendMaterialAttributes {}
#[repr(C, align(8))]
pub struct UMaterialExpressionBounds {
    __padding_end: [u8; 240],
}
impl UMaterialExpressionBounds {}
#[repr(C, align(8))]
pub struct UMaterialExpressionBreakMaterialAttributes {
    __padding_end: [u8; 256],
}
impl UMaterialExpressionBreakMaterialAttributes {}
#[repr(C, align(8))]
pub struct UMaterialExpressionBumpOffset {
    __padding_end: [u8; 360],
}
impl UMaterialExpressionBumpOffset {}
#[repr(C, align(8))]
pub struct UMaterialExpressionCameraPositionWS {
    __padding_end: [u8; 200],
}
impl UMaterialExpressionCameraPositionWS {}
#[repr(C, align(8))]
pub struct UMaterialExpressionCameraVectorWS {
    __padding_end: [u8; 216],
}
impl UMaterialExpressionCameraVectorWS {}
#[repr(C, align(8))]
pub struct UMaterialExpressionCeil {
    __padding_end: [u8; 248],
}
impl UMaterialExpressionCeil {}
#[repr(C, align(8))]
pub struct UMaterialExpressionParameter {
    __padding_end: [u8; 248],
}
impl UMaterialExpressionParameter {}
#[repr(C, align(8))]
pub struct UMaterialExpressionVectorParameter {
    __padding_end: [u8; 336],
}
impl UMaterialExpressionVectorParameter {}
#[repr(C, align(8))]
pub struct UMaterialExpressionChannelMaskParameter {
    __padding_end: [u8; 392],
}
impl UMaterialExpressionChannelMaskParameter {}
#[repr(C, align(8))]
pub struct UMaterialExpressionClamp {
    __padding_end: [u8; 360],
}
impl UMaterialExpressionClamp {}
#[repr(C, align(8))]
pub struct UMaterialExpressionCloudSampleAttribute {
    __padding_end: [u8; 216],
}
impl UMaterialExpressionCloudSampleAttribute {}
#[repr(C, align(8))]
pub struct UMaterialExpressionCollectionParameter {
    __padding_end: [u8; 272],
}
impl UMaterialExpressionCollectionParameter {}
#[repr(C, align(8))]
pub struct UMaterialExpressionCollectionTransform {
    __padding_end: [u8; 288],
}
impl UMaterialExpressionCollectionTransform {}
#[repr(C, align(8))]
pub struct UMaterialExpressionColorRamp {
    __padding_end: [u8; 264],
}
impl UMaterialExpressionColorRamp {}
#[repr(C, align(8))]
pub struct UMaterialExpressionComment {
    __padding_end: [u8; 256],
}
impl UMaterialExpressionComment {}
#[repr(C, align(8))]
pub struct UMaterialExpressionComponentMask {
    __padding_end: [u8; 256],
}
impl UMaterialExpressionComponentMask {}
#[repr(C, align(8))]
pub struct UMaterialExpressionComposite {
    __padding_end: [u8; 232],
}
impl UMaterialExpressionComposite {}
#[repr(C, align(8))]
pub struct UMaterialExpressionConstant {
    #[doc(hidden)]
    __padding_200: [u8; 200],
    pub r: f32,
    __padding_end: [u8; 4],
}
impl UMaterialExpressionConstant {}
#[repr(C, align(8))]
pub struct UMaterialExpressionConstant2Vector {
    #[doc(hidden)]
    __padding_200: [u8; 200],
    pub r: f32,
    pub g: f32,
}
impl UMaterialExpressionConstant2Vector {}
#[repr(C, align(8))]
pub struct UMaterialExpressionConstant3Vector {
    #[doc(hidden)]
    __padding_200: [u8; 200],
    pub constant: crate::bindings::core_u_object::FLinearColor,
}
impl UMaterialExpressionConstant3Vector {}
#[repr(C, align(8))]
pub struct UMaterialExpressionConstant4Vector {
    #[doc(hidden)]
    __padding_200: [u8; 200],
    pub constant: crate::bindings::core_u_object::FLinearColor,
}
impl UMaterialExpressionConstant4Vector {}
#[repr(C, align(8))]
pub struct UMaterialExpressionConstantBiasScale {
    __padding_end: [u8; 256],
}
impl UMaterialExpressionConstantBiasScale {}
#[repr(C, align(8))]
pub struct UMaterialExpressionConvert {
    __padding_end: [u8; 264],
}
impl UMaterialExpressionConvert {}
#[repr(C, align(8))]
pub struct UMaterialExpressionCosine {
    __padding_end: [u8; 256],
}
impl UMaterialExpressionCosine {}
#[repr(C, align(8))]
pub struct UMaterialExpressionCrossProduct {
    __padding_end: [u8; 296],
}
impl UMaterialExpressionCrossProduct {}
#[repr(C, align(8))]
pub struct UMaterialExpressionScalarParameter {
    __padding_end: [u8; 320],
}
impl UMaterialExpressionScalarParameter {}
#[repr(C, align(8))]
pub struct UMaterialExpressionCurveAtlasRowParameter {
    __padding_end: [u8; 384],
}
impl UMaterialExpressionCurveAtlasRowParameter {}
#[repr(C, align(8))]
pub struct UMaterialExpressionCustom {
    __padding_end: [u8; 312],
}
impl UMaterialExpressionCustom {}
#[repr(C, align(8))]
pub struct UMaterialExpressionDataDrivenShaderPlatformInfoSwitch {
    __padding_end: [u8; 320],
}
impl UMaterialExpressionDataDrivenShaderPlatformInfoSwitch {}
#[repr(C, align(8))]
pub struct UMaterialExpressionDBufferTexture {
    __padding_end: [u8; 256],
}
impl UMaterialExpressionDBufferTexture {}
#[repr(C, align(8))]
pub struct UMaterialExpressionDDX {
    __padding_end: [u8; 248],
}
impl UMaterialExpressionDDX {}
#[repr(C, align(8))]
pub struct UMaterialExpressionDDY {
    __padding_end: [u8; 248],
}
impl UMaterialExpressionDDY {}
#[repr(C, align(8))]
pub struct UMaterialExpressionDecalColor {
    __padding_end: [u8; 216],
}
impl UMaterialExpressionDecalColor {}
#[repr(C, align(8))]
pub struct UMaterialExpressionDecalDerivative {
    __padding_end: [u8; 200],
}
impl UMaterialExpressionDecalDerivative {}
#[repr(C, align(8))]
pub struct UMaterialExpressionDecalLifetimeOpacity {
    __padding_end: [u8; 216],
}
impl UMaterialExpressionDecalLifetimeOpacity {}
#[repr(C, align(8))]
pub struct UMaterialExpressionDecalMipmapLevel {
    __padding_end: [u8; 256],
}
impl UMaterialExpressionDecalMipmapLevel {}
#[repr(C, align(8))]
pub struct UMaterialExpressionDeltaTime {
    __padding_end: [u8; 216],
}
impl UMaterialExpressionDeltaTime {}
#[repr(C, align(8))]
pub struct UMaterialExpressionDepthFade {
    __padding_end: [u8; 304],
}
impl UMaterialExpressionDepthFade {}
#[repr(C, align(8))]
pub struct UMaterialExpressionDepthOfFieldFunction {
    __padding_end: [u8; 256],
}
impl UMaterialExpressionDepthOfFieldFunction {}
#[repr(C, align(8))]
pub struct UMaterialExpressionDeriveNormalZ {
    __padding_end: [u8; 248],
}
impl UMaterialExpressionDeriveNormalZ {}
#[repr(C, align(8))]
pub struct UMaterialExpressionDesaturation {
    __padding_end: [u8; 312],
}
impl UMaterialExpressionDesaturation {}
#[repr(C, align(8))]
pub struct UMaterialExpressionDistance {
    __padding_end: [u8; 296],
}
impl UMaterialExpressionDistance {}
#[repr(C, align(8))]
pub struct UMaterialExpressionDistanceCullFade {
    __padding_end: [u8; 200],
}
impl UMaterialExpressionDistanceCullFade {}
#[repr(C, align(8))]
pub struct UMaterialExpressionDistanceFieldApproxAO {
    __padding_end: [u8; 424],
}
impl UMaterialExpressionDistanceFieldApproxAO {}
#[repr(C, align(8))]
pub struct UMaterialExpressionDistanceFieldGradient {
    __padding_end: [u8; 256],
}
impl UMaterialExpressionDistanceFieldGradient {}
#[repr(C, align(8))]
pub struct UMaterialExpressionDistanceFieldsRenderingSwitch {
    __padding_end: [u8; 296],
}
impl UMaterialExpressionDistanceFieldsRenderingSwitch {}
#[repr(C, align(8))]
pub struct UMaterialExpressionDistanceToNearestSurface {
    __padding_end: [u8; 256],
}
impl UMaterialExpressionDistanceToNearestSurface {}
#[repr(C, align(8))]
pub struct UMaterialExpressionDivide {
    __padding_end: [u8; 304],
}
impl UMaterialExpressionDivide {}
#[repr(C, align(8))]
pub struct UMaterialExpressionDotProduct {
    __padding_end: [u8; 296],
}
impl UMaterialExpressionDotProduct {}
#[repr(C, align(16))]
pub struct UMaterialExpressionDoubleVectorParameter {
    __padding_end: [u8; 288],
}
impl UMaterialExpressionDoubleVectorParameter {}
#[repr(C, align(8))]
pub struct UMaterialExpressionDynamicParameter {
    __padding_end: [u8; 240],
}
impl UMaterialExpressionDynamicParameter {}
#[repr(C, align(8))]
pub struct UMaterialExpressionExponential {
    __padding_end: [u8; 248],
}
impl UMaterialExpressionExponential {}
#[repr(C, align(8))]
pub struct UMaterialExpressionExponential2 {
    __padding_end: [u8; 248],
}
impl UMaterialExpressionExponential2 {}
#[repr(C, align(8))]
pub struct UMaterialExpressionEyeAdaptation {
    __padding_end: [u8; 216],
}
impl UMaterialExpressionEyeAdaptation {}
#[repr(C, align(8))]
pub struct UMaterialExpressionEyeAdaptationInverse {
    __padding_end: [u8; 312],
}
impl UMaterialExpressionEyeAdaptationInverse {}
#[repr(C, align(8))]
pub struct UMaterialExpressionFeatureLevelSwitch {
    __padding_end: [u8; 488],
}
impl UMaterialExpressionFeatureLevelSwitch {}
#[repr(C, align(8))]
pub struct UMaterialExpressionFloatToUInt {
    __padding_end: [u8; 256],
}
impl UMaterialExpressionFloatToUInt {}
#[repr(C, align(8))]
pub struct UMaterialExpressionUIntToFloat {
    __padding_end: [u8; 248],
}
impl UMaterialExpressionUIntToFloat {}
#[repr(C, align(8))]
pub struct UMaterialExpressionFloor {
    __padding_end: [u8; 248],
}
impl UMaterialExpressionFloor {}
#[repr(C, align(8))]
pub struct UMaterialExpressionFmod {
    __padding_end: [u8; 296],
}
impl UMaterialExpressionFmod {}
#[repr(C, align(8))]
pub struct UMaterialExpressionFontSample {
    __padding_end: [u8; 216],
}
impl UMaterialExpressionFontSample {}
#[repr(C, align(8))]
pub struct UMaterialExpressionFontSampleParameter {
    __padding_end: [u8; 264],
}
impl UMaterialExpressionFontSampleParameter {}
#[repr(C, align(8))]
pub struct UMaterialExpressionFontSignedDistance {
    __padding_end: [u8; 216],
}
impl UMaterialExpressionFontSignedDistance {}
#[repr(C, align(8))]
pub struct UMaterialExpressionFrac {
    __padding_end: [u8; 248],
}
impl UMaterialExpressionFrac {}
#[repr(C, align(8))]
pub struct UMaterialExpressionFresnel {
    __padding_end: [u8; 360],
}
impl UMaterialExpressionFresnel {}
#[repr(C, align(16))]
pub struct UMaterialExpressionFunctionInput {
    #[doc(hidden)]
    __padding_248: [u8; 248],
    pub input_name: FName,
    #[doc(hidden)]
    __padding_296: [u8; 36],
    pub input_type: EFunctionInputType,
    pub preview_value: crate::bindings::core_u_object::FVector4f,
    pub flags_320: u8,
    pub sort_priority: i32,
    __padding_end: [u8; 40],
}
impl UMaterialExpressionFunctionInput {}
#[repr(C, align(8))]
pub struct UMaterialExpressionFunctionOutput {
    #[doc(hidden)]
    __padding_200: [u8; 200],
    pub output_name: FName,
    __padding_end: [u8; 108],
}
impl UMaterialExpressionFunctionOutput {}
#[repr(C, align(8))]
pub struct UMaterialExpressionGenericConstant {
    __padding_end: [u8; 200],
}
impl UMaterialExpressionGenericConstant {}
#[repr(C, align(8))]
pub struct UMaterialExpressionConstantDouble {
    #[doc(hidden)]
    __padding_200: [u8; 200],
    pub value: f64,
}
impl UMaterialExpressionConstantDouble {}
#[repr(C, align(8))]
pub struct UMaterialExpressionGetMaterialAttributes {
    __padding_end: [u8; 288],
}
impl UMaterialExpressionGetMaterialAttributes {}
#[repr(C, align(8))]
pub struct UMaterialExpressionGIReplace {
    __padding_end: [u8; 344],
}
impl UMaterialExpressionGIReplace {}
#[repr(C, align(8))]
pub struct UMaterialExpressionHairAttributes {
    __padding_end: [u8; 208],
}
impl UMaterialExpressionHairAttributes {}
#[repr(C, align(8))]
pub struct UMaterialExpressionHairColor {
    __padding_end: [u8; 344],
}
impl UMaterialExpressionHairColor {}
#[repr(C, align(8))]
pub struct UMaterialExpressionHsvToRgb {
    __padding_end: [u8; 248],
}
impl UMaterialExpressionHsvToRgb {}
#[repr(C, align(8))]
pub struct UMaterialExpressionIf {
    __padding_end: [u8; 456],
}
impl UMaterialExpressionIf {}
#[repr(C, align(8))]
pub struct UMaterialExpressionIfThenElse {
    __padding_end: [u8; 344],
}
impl UMaterialExpressionIfThenElse {}
#[repr(C, align(8))]
pub struct UMaterialExpressionInverseLinearInterpolate {
    __padding_end: [u8; 360],
}
impl UMaterialExpressionInverseLinearInterpolate {}
#[repr(C, align(8))]
pub struct UMaterialExpressionIsFirstPerson {
    __padding_end: [u8; 216],
}
impl UMaterialExpressionIsFirstPerson {}
#[repr(C, align(8))]
pub struct UMaterialExpressionIsOrthographic {
    __padding_end: [u8; 216],
}
impl UMaterialExpressionIsOrthographic {}
#[repr(C, align(8))]
pub struct UMaterialExpressionLength {
    __padding_end: [u8; 248],
}
impl UMaterialExpressionLength {}
#[repr(C, align(8))]
pub struct UMaterialExpressionLightmapUVs {
    __padding_end: [u8; 216],
}
impl UMaterialExpressionLightmapUVs {}
#[repr(C, align(8))]
pub struct UMaterialExpressionLightmassReplace {
    __padding_end: [u8; 296],
}
impl UMaterialExpressionLightmassReplace {}
#[repr(C, align(8))]
pub struct UMaterialExpressionLightVector {
    __padding_end: [u8; 216],
}
impl UMaterialExpressionLightVector {}
#[repr(C, align(8))]
pub struct UMaterialExpressionLinearInterpolate {
    __padding_end: [u8; 360],
}
impl UMaterialExpressionLinearInterpolate {}
#[repr(C, align(8))]
pub struct UMaterialExpressionLocalPosition {
    __padding_end: [u8; 208],
}
impl UMaterialExpressionLocalPosition {}
#[repr(C, align(8))]
pub struct UMaterialExpressionLogarithm {
    __padding_end: [u8; 248],
}
impl UMaterialExpressionLogarithm {}
#[repr(C, align(8))]
pub struct UMaterialExpressionLogarithm10 {
    __padding_end: [u8; 248],
}
impl UMaterialExpressionLogarithm10 {}
#[repr(C, align(8))]
pub struct UMaterialExpressionLogarithm2 {
    __padding_end: [u8; 248],
}
impl UMaterialExpressionLogarithm2 {}
#[repr(C, align(8))]
pub struct UMaterialExpressionMakeMaterialAttributes {
    __padding_end: [u8; 1496],
}
impl UMaterialExpressionMakeMaterialAttributes {}
#[repr(C, align(8))]
pub struct UMaterialExpressionMapARPassthroughCameraUV {
    __padding_end: [u8; 248],
}
impl UMaterialExpressionMapARPassthroughCameraUV {}
#[repr(C, align(8))]
pub struct UMaterialExpressionMaterialAttributeLayers {
    __padding_end: [u8; 608],
}
impl UMaterialExpressionMaterialAttributeLayers {}
#[repr(C, align(8))]
pub struct UMaterialExpressionMaterialFunctionCall {
    __padding_end: [u8; 280],
}
impl UMaterialExpressionMaterialFunctionCall {}
#[repr(C, align(8))]
pub struct UMaterialExpressionMaterialLayerOutput {
    __padding_end: [u8; 320],
}
impl UMaterialExpressionMaterialLayerOutput {}
#[repr(C, align(8))]
pub struct UMaterialExpressionMaterialProxyReplace {
    __padding_end: [u8; 296],
}
impl UMaterialExpressionMaterialProxyReplace {}
#[repr(C, align(8))]
pub struct UMaterialExpressionMax {
    __padding_end: [u8; 304],
}
impl UMaterialExpressionMax {}
#[repr(C, align(8))]
pub struct UMaterialExpressionMeshPaintTextureCoordinateIndex {
    __padding_end: [u8; 216],
}
impl UMaterialExpressionMeshPaintTextureCoordinateIndex {}
#[repr(C, align(8))]
pub struct UMaterialExpressionMeshPaintTextureObject {
    __padding_end: [u8; 216],
}
impl UMaterialExpressionMeshPaintTextureObject {}
#[repr(C, align(8))]
pub struct UMaterialExpressionMeshPaintTextureReplace {
    __padding_end: [u8; 296],
}
impl UMaterialExpressionMeshPaintTextureReplace {}
#[repr(C, align(8))]
pub struct UMaterialExpressionMin {
    __padding_end: [u8; 304],
}
impl UMaterialExpressionMin {}
#[repr(C, align(8))]
pub struct UMaterialExpressionModulo {
    __padding_end: [u8; 296],
}
impl UMaterialExpressionModulo {}
#[repr(C, align(8))]
pub struct UMaterialExpressionMultiply {
    __padding_end: [u8; 304],
}
impl UMaterialExpressionMultiply {}
#[repr(C, align(8))]
pub struct UMaterialExpressionRerouteBase {
    __padding_end: [u8; 200],
}
impl UMaterialExpressionRerouteBase {}
#[repr(C, align(8))]
pub struct UMaterialExpressionNamedRerouteBase {
    __padding_end: [u8; 200],
}
impl UMaterialExpressionNamedRerouteBase {}
#[repr(C, align(8))]
pub struct UMaterialExpressionNamedRerouteDeclaration {
    __padding_end: [u8; 296],
}
impl UMaterialExpressionNamedRerouteDeclaration {}
#[repr(C, align(8))]
pub struct UMaterialExpressionNamedRerouteUsage {
    __padding_end: [u8; 224],
}
impl UMaterialExpressionNamedRerouteUsage {}
#[repr(C, align(8))]
pub struct UMaterialExpressionNaniteReplace {
    __padding_end: [u8; 296],
}
impl UMaterialExpressionNaniteReplace {}
#[repr(C, align(8))]
pub struct UMaterialExpressionNeuralNetworkInput {
    __padding_end: [u8; 352],
}
impl UMaterialExpressionNeuralNetworkInput {}
#[repr(C, align(8))]
pub struct UMaterialExpressionNeuralNetworkOutput {
    __padding_end: [u8; 256],
}
impl UMaterialExpressionNeuralNetworkOutput {}
#[repr(C, align(8))]
pub struct UMaterialExpressionNormalize {
    __padding_end: [u8; 248],
}
impl UMaterialExpressionNormalize {}
#[repr(C, align(8))]
pub struct UMaterialExpressionObjectBounds {
    __padding_end: [u8; 216],
}
impl UMaterialExpressionObjectBounds {}
#[repr(C, align(8))]
pub struct UMaterialExpressionObjectLocalBounds {
    __padding_end: [u8; 232],
}
impl UMaterialExpressionObjectLocalBounds {}
#[repr(C, align(8))]
pub struct UMaterialExpressionObjectOrientation {
    __padding_end: [u8; 216],
}
impl UMaterialExpressionObjectOrientation {}
#[repr(C, align(8))]
pub struct UMaterialExpressionObjectPositionWS {
    __padding_end: [u8; 208],
}
impl UMaterialExpressionObjectPositionWS {}
#[repr(C, align(8))]
pub struct UMaterialExpressionObjectRadius {
    __padding_end: [u8; 216],
}
impl UMaterialExpressionObjectRadius {}
#[repr(C, align(8))]
pub struct UMaterialExpressionOneMinus {
    __padding_end: [u8; 248],
}
impl UMaterialExpressionOneMinus {}
#[repr(C, align(8))]
pub struct UMaterialExpressionOperator {
    __padding_end: [u8; 232],
}
impl UMaterialExpressionOperator {}
#[repr(C, align(8))]
pub struct UMaterialExpressionPanner {
    __padding_end: [u8; 360],
}
impl UMaterialExpressionPanner {}
#[repr(C, align(8))]
pub struct UMaterialExpressionParticleColor {
    __padding_end: [u8; 216],
}
impl UMaterialExpressionParticleColor {}
#[repr(C, align(8))]
pub struct UMaterialExpressionParticleDirection {
    __padding_end: [u8; 216],
}
impl UMaterialExpressionParticleDirection {}
#[repr(C, align(8))]
pub struct UMaterialExpressionParticleMacroUV {
    __padding_end: [u8; 216],
}
impl UMaterialExpressionParticleMacroUV {}
#[repr(C, align(8))]
pub struct UMaterialExpressionParticleMotionBlurFade {
    __padding_end: [u8; 216],
}
impl UMaterialExpressionParticleMotionBlurFade {}
#[repr(C, align(8))]
pub struct UMaterialExpressionParticlePositionWS {
    __padding_end: [u8; 224],
}
impl UMaterialExpressionParticlePositionWS {}
#[repr(C, align(8))]
pub struct UMaterialExpressionParticleRadius {
    __padding_end: [u8; 216],
}
impl UMaterialExpressionParticleRadius {}
#[repr(C, align(8))]
pub struct UMaterialExpressionParticleRandom {
    __padding_end: [u8; 216],
}
impl UMaterialExpressionParticleRandom {}
#[repr(C, align(8))]
pub struct UMaterialExpressionParticleRelativeTime {
    __padding_end: [u8; 216],
}
impl UMaterialExpressionParticleRelativeTime {}
#[repr(C, align(8))]
pub struct UMaterialExpressionParticleSize {
    __padding_end: [u8; 216],
}
impl UMaterialExpressionParticleSize {}
#[repr(C, align(8))]
pub struct UMaterialExpressionParticleSpeed {
    __padding_end: [u8; 216],
}
impl UMaterialExpressionParticleSpeed {}
#[repr(C, align(8))]
pub struct UMaterialExpressionParticleSpriteRotation {
    __padding_end: [u8; 216],
}
impl UMaterialExpressionParticleSpriteRotation {}
#[repr(C, align(8))]
pub struct UMaterialExpressionParticleSubUV {
    __padding_end: [u8; 592],
}
impl UMaterialExpressionParticleSubUV {}
#[repr(C, align(8))]
pub struct UMaterialExpressionParticleSubUVProperties {
    __padding_end: [u8; 216],
}
impl UMaterialExpressionParticleSubUVProperties {}
#[repr(C, align(8))]
pub struct UMaterialExpressionPathTracingBufferTexture {
    __padding_end: [u8; 256],
}
impl UMaterialExpressionPathTracingBufferTexture {}
#[repr(C, align(8))]
pub struct UMaterialExpressionPathTracingQualitySwitch {
    __padding_end: [u8; 296],
}
impl UMaterialExpressionPathTracingQualitySwitch {}
#[repr(C, align(8))]
pub struct UMaterialExpressionPathTracingRayTypeSwitch {
    __padding_end: [u8; 440],
}
impl UMaterialExpressionPathTracingRayTypeSwitch {}
#[repr(C, align(8))]
pub struct UMaterialExpressionPerInstanceCustomData {
    __padding_end: [u8; 272],
}
impl UMaterialExpressionPerInstanceCustomData {}
#[repr(C, align(8))]
pub struct UMaterialExpressionPerInstanceCustomData3Vector {
    __padding_end: [u8; 288],
}
impl UMaterialExpressionPerInstanceCustomData3Vector {}
#[repr(C, align(8))]
pub struct UMaterialExpressionPerInstanceFadeAmount {
    __padding_end: [u8; 216],
}
impl UMaterialExpressionPerInstanceFadeAmount {}
#[repr(C, align(8))]
pub struct UMaterialExpressionPerInstanceRandom {
    __padding_end: [u8; 216],
}
impl UMaterialExpressionPerInstanceRandom {}
#[repr(C, align(8))]
pub struct UMaterialExpressionPinBase {
    __padding_end: [u8; 240],
}
impl UMaterialExpressionPinBase {}
#[repr(C, align(8))]
pub struct UMaterialExpressionPixelDepth {
    __padding_end: [u8; 216],
}
impl UMaterialExpressionPixelDepth {}
#[repr(C, align(8))]
pub struct UMaterialExpressionPixelNormalWS {
    __padding_end: [u8; 200],
}
impl UMaterialExpressionPixelNormalWS {}
#[repr(C, align(8))]
pub struct UMaterialExpressionPostVolumeUserFlagTest {
    __padding_end: [u8; 256],
}
impl UMaterialExpressionPostVolumeUserFlagTest {}
#[repr(C, align(8))]
pub struct UMaterialExpressionPower {
    __padding_end: [u8; 304],
}
impl UMaterialExpressionPower {}
#[repr(C, align(8))]
pub struct UMaterialExpressionPrecomputedAOMask {
    __padding_end: [u8; 216],
}
impl UMaterialExpressionPrecomputedAOMask {}
#[repr(C, align(8))]
pub struct UMaterialExpressionPreSkinnedLocalBounds {
    __padding_end: [u8; 216],
}
impl UMaterialExpressionPreSkinnedLocalBounds {}
#[repr(C, align(8))]
pub struct UMaterialExpressionPreSkinnedNormal {
    __padding_end: [u8; 216],
}
impl UMaterialExpressionPreSkinnedNormal {}
#[repr(C, align(8))]
pub struct UMaterialExpressionPreSkinnedPosition {
    __padding_end: [u8; 216],
}
impl UMaterialExpressionPreSkinnedPosition {}
#[repr(C, align(8))]
pub struct UMaterialExpressionPreviousFrameSwitch {
    __padding_end: [u8; 296],
}
impl UMaterialExpressionPreviousFrameSwitch {}
#[repr(C, align(8))]
pub struct UMaterialExpressionQualitySwitch {
    __padding_end: [u8; 440],
}
impl UMaterialExpressionQualitySwitch {}
#[repr(C, align(8))]
pub struct UMaterialExpressionRayTracingQualitySwitch {
    __padding_end: [u8; 296],
}
impl UMaterialExpressionRayTracingQualitySwitch {}
#[repr(C, align(8))]
pub struct UMaterialExpressionRecordTextureStreamingInfo {
    __padding_end: [u8; 296],
}
impl UMaterialExpressionRecordTextureStreamingInfo {}
#[repr(C, align(8))]
pub struct UMaterialExpressionReflectionCapturePassSwitch {
    __padding_end: [u8; 296],
}
impl UMaterialExpressionReflectionCapturePassSwitch {}
#[repr(C, align(8))]
pub struct UMaterialExpressionReflectionVectorWS {
    __padding_end: [u8; 256],
}
impl UMaterialExpressionReflectionVectorWS {}
#[repr(C, align(8))]
pub struct UMaterialExpressionRequiredSamplersSwitch {
    __padding_end: [u8; 304],
}
impl UMaterialExpressionRequiredSamplersSwitch {}
#[repr(C, align(8))]
pub struct UMaterialExpressionReroute {
    __padding_end: [u8; 248],
}
impl UMaterialExpressionReroute {}
#[repr(C, align(8))]
pub struct UMaterialExpressionRgbToHsv {
    __padding_end: [u8; 248],
}
impl UMaterialExpressionRgbToHsv {}
#[repr(C, align(8))]
pub struct UMaterialExpressionRotateAboutAxis {
    __padding_end: [u8; 400],
}
impl UMaterialExpressionRotateAboutAxis {}
#[repr(C, align(8))]
pub struct UMaterialExpressionRotator {
    __padding_end: [u8; 312],
}
impl UMaterialExpressionRotator {}
#[repr(C, align(8))]
pub struct UMaterialExpressionRound {
    __padding_end: [u8; 248],
}
impl UMaterialExpressionRound {}
#[repr(C, align(8))]
pub struct UMaterialExpressionRuntimeVirtualTextureCustomData {
    __padding_end: [u8; 200],
}
impl UMaterialExpressionRuntimeVirtualTextureCustomData {}
#[repr(C, align(8))]
pub struct UMaterialExpressionRuntimeVirtualTextureOutput {
    __padding_end: [u8; 632],
}
impl UMaterialExpressionRuntimeVirtualTextureOutput {}
#[repr(C, align(8))]
pub struct UMaterialExpressionRuntimeVirtualTextureReplace {
    __padding_end: [u8; 296],
}
impl UMaterialExpressionRuntimeVirtualTextureReplace {}
#[repr(C, align(8))]
pub struct UMaterialExpressionRuntimeVirtualTextureSample {
    __padding_end: [u8; 456],
}
impl UMaterialExpressionRuntimeVirtualTextureSample {}
#[repr(C, align(8))]
pub struct UMaterialExpressionRuntimeVirtualTextureSampleParameter {
    __padding_end: [u8; 504],
}
impl UMaterialExpressionRuntimeVirtualTextureSampleParameter {}
#[repr(C, align(8))]
pub struct UMaterialExpressionSamplePhysicsVectorField {
    __padding_end: [u8; 256],
}
impl UMaterialExpressionSamplePhysicsVectorField {}
#[repr(C, align(8))]
pub struct UMaterialExpressionSamplePhysicsScalarField {
    __padding_end: [u8; 256],
}
impl UMaterialExpressionSamplePhysicsScalarField {}
#[repr(C, align(8))]
pub struct UMaterialExpressionSamplePhysicsIntegerField {
    __padding_end: [u8; 256],
}
impl UMaterialExpressionSamplePhysicsIntegerField {}
#[repr(C, align(8))]
pub struct UMaterialExpressionSaturate {
    __padding_end: [u8; 248],
}
impl UMaterialExpressionSaturate {}
#[repr(C, align(8))]
pub struct UMaterialExpressionSceneColor {
    __padding_end: [u8; 320],
}
impl UMaterialExpressionSceneColor {}
#[repr(C, align(8))]
pub struct UMaterialExpressionSceneDepth {
    __padding_end: [u8; 320],
}
impl UMaterialExpressionSceneDepth {}
#[repr(C, align(8))]
pub struct UMaterialExpressionSceneDepthWithoutWater {
    __padding_end: [u8; 280],
}
impl UMaterialExpressionSceneDepthWithoutWater {}
#[repr(C, align(8))]
pub struct UMaterialExpressionSceneTexelSize {
    __padding_end: [u8; 200],
}
impl UMaterialExpressionSceneTexelSize {}
#[repr(C, align(8))]
pub struct UMaterialExpressionSceneTexture {
    __padding_end: [u8; 256],
}
impl UMaterialExpressionSceneTexture {}
#[repr(C, align(8))]
pub struct UMaterialExpressionScreenPosition {
    __padding_end: [u8; 216],
}
impl UMaterialExpressionScreenPosition {}
#[repr(C, align(8))]
pub struct UMaterialExpressionSetMaterialAttributes {
    __padding_end: [u8; 248],
}
impl UMaterialExpressionSetMaterialAttributes {}
#[repr(C, align(8))]
pub struct UMaterialExpressionShaderStageSwitch {
    __padding_end: [u8; 296],
}
impl UMaterialExpressionShaderStageSwitch {}
#[repr(C, align(8))]
pub struct UMaterialExpressionShadingModel {
    __padding_end: [u8; 208],
}
impl UMaterialExpressionShadingModel {}
#[repr(C, align(8))]
pub struct UMaterialExpressionShadingPathSwitch {
    __padding_end: [u8; 392],
}
impl UMaterialExpressionShadingPathSwitch {}
#[repr(C, align(8))]
pub struct UMaterialExpressionShadowReplace {
    __padding_end: [u8; 296],
}
impl UMaterialExpressionShadowReplace {}
#[repr(C, align(8))]
pub struct UMaterialExpressionSign {
    __padding_end: [u8; 248],
}
impl UMaterialExpressionSign {}
#[repr(C, align(8))]
pub struct UMaterialExpressionSine {
    __padding_end: [u8; 256],
}
impl UMaterialExpressionSine {}
#[repr(C, align(8))]
pub struct UMaterialExpressionSingleLayerWaterMaterialOutput {
    __padding_end: [u8; 392],
}
impl UMaterialExpressionSingleLayerWaterMaterialOutput {}
#[repr(C, align(8))]
pub struct UMaterialExpressionSkyAtmosphereLightDirection {
    #[doc(hidden)]
    __padding_216: [u8; 216],
    pub light_index: i32,
    __padding_end: [u8; 4],
}
impl UMaterialExpressionSkyAtmosphereLightDirection {}
#[repr(C, align(8))]
pub struct UMaterialExpressionSkyAtmosphereLightIlluminance {
    #[doc(hidden)]
    __padding_216: [u8; 216],
    pub light_index: i32,
    __padding_end: [u8; 60],
}
impl UMaterialExpressionSkyAtmosphereLightIlluminance {}
#[repr(C, align(8))]
pub struct UMaterialExpressionSkyAtmosphereLightIlluminanceOnGround {
    #[doc(hidden)]
    __padding_216: [u8; 216],
    pub light_index: i32,
    __padding_end: [u8; 4],
}
impl UMaterialExpressionSkyAtmosphereLightIlluminanceOnGround {}
#[repr(C, align(8))]
pub struct UMaterialExpressionSkyAtmosphereLightDiskLuminance {
    #[doc(hidden)]
    __padding_216: [u8; 216],
    pub light_index: i32,
    __padding_end: [u8; 52],
}
impl UMaterialExpressionSkyAtmosphereLightDiskLuminance {}
#[repr(C, align(8))]
pub struct UMaterialExpressionSkyAtmosphereAerialPerspective {
    __padding_end: [u8; 272],
}
impl UMaterialExpressionSkyAtmosphereAerialPerspective {}
#[repr(C, align(8))]
pub struct UMaterialExpressionSkyAtmosphereDistantLightScatteredLuminance {
    __padding_end: [u8; 216],
}
impl UMaterialExpressionSkyAtmosphereDistantLightScatteredLuminance {}
#[repr(C, align(8))]
pub struct UMaterialExpressionSkyAtmosphereViewLuminance {
    __padding_end: [u8; 264],
}
impl UMaterialExpressionSkyAtmosphereViewLuminance {}
#[repr(C, align(8))]
pub struct UMaterialExpressionSkyLightEnvMapSample {
    __padding_end: [u8; 312],
}
impl UMaterialExpressionSkyLightEnvMapSample {}
#[repr(C, align(8))]
pub struct UMaterialExpressionSmoothStep {
    __padding_end: [u8; 360],
}
impl UMaterialExpressionSmoothStep {}
#[repr(C, align(8))]
pub struct UMaterialExpressionSobol {
    __padding_end: [u8; 368],
}
impl UMaterialExpressionSobol {}
#[repr(C, align(8))]
pub struct UMaterialExpressionSparseVolumeTextureBase {
    #[doc(hidden)]
    __padding_200: [u8; 200],
    pub sparse_volume_texture: UPtr<USparseVolumeTexture>,
}
impl UMaterialExpressionSparseVolumeTextureBase {}
#[repr(C, align(8))]
pub struct UMaterialExpressionSparseVolumeTextureObject {
    __padding_end: [u8; 208],
}
impl UMaterialExpressionSparseVolumeTextureObject {}
#[repr(C, align(8))]
pub struct UMaterialExpressionSparseVolumeTextureSample {
    __padding_end: [u8; 456],
}
impl UMaterialExpressionSparseVolumeTextureSample {}
#[repr(C, align(8))]
pub struct UMaterialExpressionSparseVolumeTextureSampleParameter {
    __padding_end: [u8; 504],
}
impl UMaterialExpressionSparseVolumeTextureSampleParameter {}
#[repr(C, align(8))]
pub struct UMaterialExpressionSparseVolumeTextureObjectParameter {
    __padding_end: [u8; 504],
}
impl UMaterialExpressionSparseVolumeTextureObjectParameter {}
#[repr(C, align(8))]
pub struct UMaterialExpressionSpeedTree {
    __padding_end: [u8; 408],
}
impl UMaterialExpressionSpeedTree {}
#[repr(C, align(8))]
pub struct UMaterialExpressionSphereMask {
    __padding_end: [u8; 400],
}
impl UMaterialExpressionSphereMask {}
#[repr(C, align(8))]
pub struct UMaterialExpressionSphericalParticleOpacity {
    __padding_end: [u8; 272],
}
impl UMaterialExpressionSphericalParticleOpacity {}
#[repr(C, align(8))]
pub struct UMaterialExpressionSquareRoot {
    __padding_end: [u8; 248],
}
impl UMaterialExpressionSquareRoot {}
#[repr(C, align(8))]
pub struct UMaterialExpressionSRGBColorToWorkingColorSpace {
    __padding_end: [u8; 248],
}
impl UMaterialExpressionSRGBColorToWorkingColorSpace {}
#[repr(C, align(8))]
pub struct UMaterialExpressionStaticBool {
    __padding_end: [u8; 208],
}
impl UMaterialExpressionStaticBool {}
#[repr(C, align(8))]
pub struct UMaterialExpressionStaticBoolParameter {
    __padding_end: [u8; 256],
}
impl UMaterialExpressionStaticBoolParameter {}
#[repr(C, align(8))]
pub struct UMaterialExpressionStaticComponentMaskParameter {
    __padding_end: [u8; 304],
}
impl UMaterialExpressionStaticComponentMaskParameter {}
#[repr(C, align(8))]
pub struct UMaterialExpressionStaticSwitch {
    __padding_end: [u8; 352],
}
impl UMaterialExpressionStaticSwitch {}
#[repr(C, align(8))]
pub struct UMaterialExpressionStaticSwitchParameter {
    __padding_end: [u8; 352],
}
impl UMaterialExpressionStaticSwitchParameter {}
#[repr(C, align(8))]
pub struct UMaterialExpressionStep {
    __padding_end: [u8; 304],
}
impl UMaterialExpressionStep {}
#[repr(C, align(8))]
pub struct UMaterialExpressionSubsurfaceMediumMaterialOutput {
    __padding_end: [u8; 296],
}
impl UMaterialExpressionSubsurfaceMediumMaterialOutput {}
#[repr(C, align(8))]
pub struct UMaterialExpressionSubtract {
    __padding_end: [u8; 304],
}
impl UMaterialExpressionSubtract {}
#[repr(C, align(8))]
pub struct UMaterialExpressionSwitch {
    __padding_end: [u8; 344],
}
impl UMaterialExpressionSwitch {}
#[repr(C, align(8))]
pub struct UMaterialExpressionTangent {
    __padding_end: [u8; 256],
}
impl UMaterialExpressionTangent {}
#[repr(C, align(8))]
pub struct UMaterialExpressionTemporalSobol {
    __padding_end: [u8; 320],
}
impl UMaterialExpressionTemporalSobol {}
#[repr(C, align(8))]
pub struct UMaterialExpressionTextureCollection {
    #[doc(hidden)]
    __padding_200: [u8; 200],
    pub texture_collection: UPtr<UTextureCollection>,
}
impl UMaterialExpressionTextureCollection {}
#[repr(C, align(8))]
pub struct UMaterialExpressionTextureCollectionParameter {
    __padding_end: [u8; 256],
}
impl UMaterialExpressionTextureCollectionParameter {}
#[repr(C, align(8))]
pub struct UMaterialExpressionTextureCoordinate {
    #[doc(hidden)]
    __padding_200: [u8; 200],
    pub coordinate_index: i32,
    pub u_tiling: f32,
    pub v_tiling: f32,
    __padding_end: [u8; 4],
}
impl UMaterialExpressionTextureCoordinate {}
#[repr(C, align(8))]
pub struct UMaterialExpressionTextureObject {
    __padding_end: [u8; 216],
}
impl UMaterialExpressionTextureObject {}
#[repr(C, align(8))]
pub struct UMaterialExpressionTextureObjectFromCollection {
    #[doc(hidden)]
    __padding_248: [u8; 248],
    pub texture_collection_object: UPtr<UTextureCollection>,
    #[doc(hidden)]
    __padding_304: [u8; 48],
    pub const_collection_index: i32,
    pub texture_type: ETextureCollectionMemberType,
    __padding_end: [u8; 3],
}
impl UMaterialExpressionTextureObjectFromCollection {}
#[repr(C, align(8))]
pub struct UMaterialExpressionTextureObjectParameter {
    __padding_end: [u8; 632],
}
impl UMaterialExpressionTextureObjectParameter {}
#[repr(C, align(8))]
pub struct UMaterialExpressionTextureProperty {
    __padding_end: [u8; 256],
}
impl UMaterialExpressionTextureProperty {}
#[repr(C, align(8))]
pub struct UMaterialExpressionTextureSampleParameter2DArray {
    __padding_end: [u8; 632],
}
impl UMaterialExpressionTextureSampleParameter2DArray {}
#[repr(C, align(8))]
pub struct UMaterialExpressionTextureSampleParameterCube {
    __padding_end: [u8; 632],
}
impl UMaterialExpressionTextureSampleParameterCube {}
#[repr(C, align(8))]
pub struct UMaterialExpressionTextureSampleParameterCubeArray {
    __padding_end: [u8; 632],
}
impl UMaterialExpressionTextureSampleParameterCubeArray {}
#[repr(C, align(8))]
pub struct UMaterialExpressionTextureSampleParameterSubUV {
    __padding_end: [u8; 640],
}
impl UMaterialExpressionTextureSampleParameterSubUV {}
#[repr(C, align(8))]
pub struct UMaterialExpressionTextureSampleParameterVolume {
    __padding_end: [u8; 632],
}
impl UMaterialExpressionTextureSampleParameterVolume {}
#[repr(C, align(8))]
pub struct UMaterialExpressionTime {
    __padding_end: [u8; 208],
}
impl UMaterialExpressionTime {}
#[repr(C, align(8))]
pub struct UMaterialExpressionTransform {
    __padding_end: [u8; 256],
}
impl UMaterialExpressionTransform {}
#[repr(C, align(8))]
pub struct UMaterialExpressionTransformPosition {
    __padding_end: [u8; 368],
}
impl UMaterialExpressionTransformPosition {}
#[repr(C, align(8))]
pub struct UMaterialExpressionTruncate {
    __padding_end: [u8; 248],
}
impl UMaterialExpressionTruncate {}
#[repr(C, align(8))]
pub struct UMaterialExpressionTruncateLWC {
    __padding_end: [u8; 248],
}
impl UMaterialExpressionTruncateLWC {}
#[repr(C, align(8))]
pub struct UMaterialExpressionTwoSidedSign {
    __padding_end: [u8; 216],
}
impl UMaterialExpressionTwoSidedSign {}
#[repr(C, align(8))]
pub struct UMaterialExpressionUserSceneTexture {
    __padding_end: [u8; 264],
}
impl UMaterialExpressionUserSceneTexture {}
#[repr(C, align(8))]
pub struct UMaterialExpressionVectorNoise {
    __padding_end: [u8; 264],
}
impl UMaterialExpressionVectorNoise {}
#[repr(C, align(8))]
pub struct UMaterialExpressionVertexColor {
    __padding_end: [u8; 216],
}
impl UMaterialExpressionVertexColor {}
#[repr(C, align(8))]
pub struct UMaterialExpressionVertexInterpolator {
    __padding_end: [u8; 304],
}
impl UMaterialExpressionVertexInterpolator {}
#[repr(C, align(8))]
pub struct UMaterialExpressionVertexNormalWS {
    __padding_end: [u8; 216],
}
impl UMaterialExpressionVertexNormalWS {}
#[repr(C, align(8))]
pub struct UMaterialExpressionVertexTangentWS {
    __padding_end: [u8; 216],
}
impl UMaterialExpressionVertexTangentWS {}
#[repr(C, align(8))]
pub struct UMaterialExpressionViewProperty {
    __padding_end: [u8; 208],
}
impl UMaterialExpressionViewProperty {}
#[repr(C, align(8))]
pub struct UMaterialExpressionViewSize {
    __padding_end: [u8; 200],
}
impl UMaterialExpressionViewSize {}
#[repr(C, align(8))]
pub struct UMaterialExpressionVirtualTextureFeatureSwitch {
    __padding_end: [u8; 296],
}
impl UMaterialExpressionVirtualTextureFeatureSwitch {}
#[repr(C, align(8))]
pub struct UMaterialExpressionVolumetricAdvancedMaterialInput {
    __padding_end: [u8; 200],
}
impl UMaterialExpressionVolumetricAdvancedMaterialInput {}
#[repr(C, align(8))]
pub struct UMaterialExpressionVolumetricCloudEmptySpaceSkippingInput {
    __padding_end: [u8; 216],
}
impl UMaterialExpressionVolumetricCloudEmptySpaceSkippingInput {}
#[repr(C, align(8))]
pub struct UMaterialExpressionVolumetricAdvancedMaterialOutput {
    __padding_end: [u8; 576],
}
impl UMaterialExpressionVolumetricAdvancedMaterialOutput {}
#[repr(C, align(8))]
pub struct UMaterialExpressionVolumetricCloudEmptySpaceSkippingOutput {
    __padding_end: [u8; 248],
}
impl UMaterialExpressionVolumetricCloudEmptySpaceSkippingOutput {}
#[repr(C, align(8))]
pub struct UMaterialExpressionWorldPosition {
    __padding_end: [u8; 208],
}
impl UMaterialExpressionWorldPosition {}
#[repr(C, align(8))]
pub struct UMaterialFunctionInterfaceEditorOnlyData {
    __padding_end: [u8; 48],
}
impl UMaterialFunctionInterfaceEditorOnlyData {}
#[repr(C, align(8))]
pub struct UMaterialFunctionEditorOnlyData {
    __padding_end: [u8; 96],
}
impl UMaterialFunctionEditorOnlyData {}
#[repr(C, align(8))]
pub struct UMaterialFunctionInterface {
    __padding_end: [u8; 104],
}
impl UMaterialFunctionInterface {}
#[repr(C, align(8))]
pub struct UMaterialFunction {
    __padding_end: [u8; 272],
}
impl UMaterialFunction {}
#[repr(C, align(8))]
pub struct UMaterialFunctionInstance {
    #[doc(hidden)]
    __padding_104: [u8; 104],
    pub parent: UPtr<UMaterialFunctionInterface>,
    __padding_end: [u8; 192],
}
impl UMaterialFunctionInstance {}
#[repr(C, align(8))]
pub struct UMaterialFunctionMaterialLayer {
    __padding_end: [u8; 272],
}
impl UMaterialFunctionMaterialLayer {}
#[repr(C, align(8))]
pub struct UMaterialFunctionMaterialLayerInstance {
    __padding_end: [u8; 304],
}
impl UMaterialFunctionMaterialLayerInstance {}
#[repr(C, align(8))]
pub struct UMaterialFunctionMaterialLayerBlend {
    __padding_end: [u8; 272],
}
impl UMaterialFunctionMaterialLayerBlend {}
#[repr(C, align(8))]
pub struct UMaterialFunctionMaterialLayerBlendInstance {
    __padding_end: [u8; 304],
}
impl UMaterialFunctionMaterialLayerBlendInstance {}
#[repr(C, align(8))]
pub struct UMaterialParameterCollectionInstance {
    __padding_end: [u8; 312],
}
impl UMaterialParameterCollectionInstance {}
#[repr(C, align(8))]
pub struct UMeshDeformer {
    __padding_end: [u8; 48],
}
impl UMeshDeformer {}
#[repr(C, align(8))]
pub struct UMeshDeformerInstanceSettings {
    __padding_end: [u8; 48],
}
impl UMeshDeformerInstanceSettings {}
#[repr(C, align(8))]
pub struct UMeshDeformerInstance {
    __padding_end: [u8; 48],
}
impl UMeshDeformerInstance {}
pub struct IMeshDeformerProducer {}
#[repr(C, align(8))]
pub struct UMeshDeformerProducer {
    __padding_end: [u8; 48],
}
impl UMeshDeformerProducer {}
#[repr(C, align(8))]
pub struct UMeshDrawCommandStatsSettings {
    __padding_end: [u8; 144],
}
impl UMeshDrawCommandStatsSettings {}
#[repr(C, align(8))]
pub struct UMicroTransactionBase {
    __padding_end: [u8; 112],
}
impl UMicroTransactionBase {}
pub struct INavAgentInterface {}
#[repr(C, align(8))]
pub struct UNavAgentInterface {
    __padding_end: [u8; 48],
}
impl UNavAgentInterface {}
pub struct INavEdgeProviderInterface {}
#[repr(C, align(8))]
pub struct UNavEdgeProviderInterface {
    __padding_end: [u8; 48],
}
impl UNavEdgeProviderInterface {}
pub struct INavigationDataInterface {}
#[repr(C, align(8))]
pub struct UNavigationDataInterface {
    __padding_end: [u8; 48],
}
impl UNavigationDataInterface {}
pub struct INavigationInvokerInterface {}
#[repr(C, align(8))]
pub struct UNavigationInvokerInterface {
    __padding_end: [u8; 48],
}
impl UNavigationInvokerInterface {}
pub struct INavPathObserverInterface {}
#[repr(C, align(8))]
pub struct UNavPathObserverInterface {
    __padding_end: [u8; 48],
}
impl UNavPathObserverInterface {}
pub struct INavRelevantInterface {}
#[repr(C, align(8))]
pub struct UNavRelevantInterface {
    __padding_end: [u8; 48],
}
impl UNavRelevantInterface {}
pub struct INetworkPredictionInterface {}
#[repr(C, align(8))]
pub struct UNetworkPredictionInterface {
    __padding_end: [u8; 48],
}
impl UNetworkPredictionInterface {}
#[repr(C, align(8))]
pub struct UFXSystemAsset {
    __padding_end: [u8; 96],
}
impl UFXSystemAsset {}
#[repr(C, align(8))]
pub struct UParticleSystem {
    __padding_end: [u8; 544],
}
impl UParticleSystem {}
#[repr(C, align(8))]
pub struct UParticleModule {
    __padding_end: [u8; 56],
}
impl UParticleModule {}
#[repr(C, align(8))]
pub struct UParticleModuleAccelerationBase {
    __padding_end: [u8; 64],
}
impl UParticleModuleAccelerationBase {}
#[repr(C, align(8))]
pub struct UParticleModuleAcceleration {
    __padding_end: [u8; 168],
}
impl UParticleModuleAcceleration {}
#[repr(C, align(8))]
pub struct UParticleModuleAccelerationConstant {
    __padding_end: [u8; 88],
}
impl UParticleModuleAccelerationConstant {}
#[repr(C, align(8))]
pub struct UParticleModuleAccelerationDrag {
    __padding_end: [u8; 120],
}
impl UParticleModuleAccelerationDrag {}
#[repr(C, align(8))]
pub struct UParticleModuleAccelerationDragScaleOverLife {
    __padding_end: [u8; 120],
}
impl UParticleModuleAccelerationDragScaleOverLife {}
#[repr(C, align(8))]
pub struct UParticleModuleAccelerationOverLifetime {
    __padding_end: [u8; 160],
}
impl UParticleModuleAccelerationOverLifetime {}
#[repr(C, align(8))]
pub struct UParticleModuleAttractorBase {
    __padding_end: [u8; 56],
}
impl UParticleModuleAttractorBase {}
#[repr(C, align(8))]
pub struct UParticleModuleAttractorLine {
    __padding_end: [u8; 200],
}
impl UParticleModuleAttractorLine {}
#[repr(C, align(8))]
pub struct UParticleModuleAttractorParticle {
    __padding_end: [u8; 192],
}
impl UParticleModuleAttractorParticle {}
#[repr(C, align(8))]
pub struct UParticleModuleAttractorPoint {
    __padding_end: [u8; 256],
}
impl UParticleModuleAttractorPoint {}
#[repr(C, align(8))]
pub struct UParticleModuleAttractorPointGravity {
    __padding_end: [u8; 144],
}
impl UParticleModuleAttractorPointGravity {}
#[repr(C, align(8))]
pub struct UParticleModuleBeamBase {
    __padding_end: [u8; 56],
}
impl UParticleModuleBeamBase {}
#[repr(C, align(8))]
pub struct UParticleModuleBeamModifier {
    __padding_end: [u8; 320],
}
impl UParticleModuleBeamModifier {}
#[repr(C, align(8))]
pub struct UParticleModuleBeamNoise {
    __padding_end: [u8; 456],
}
impl UParticleModuleBeamNoise {}
#[repr(C, align(8))]
pub struct UParticleModuleBeamSource {
    __padding_end: [u8; 344],
}
impl UParticleModuleBeamSource {}
#[repr(C, align(8))]
pub struct UParticleModuleBeamTarget {
    __padding_end: [u8; 344],
}
impl UParticleModuleBeamTarget {}
#[repr(C, align(8))]
pub struct UParticleModuleCameraBase {
    __padding_end: [u8; 56],
}
impl UParticleModuleCameraBase {}
#[repr(C, align(8))]
pub struct UParticleModuleCameraOffset {
    __padding_end: [u8; 112],
}
impl UParticleModuleCameraOffset {}
#[repr(C, align(8))]
pub struct UParticleModuleCollisionBase {
    __padding_end: [u8; 56],
}
impl UParticleModuleCollisionBase {}
#[repr(C, align(8))]
pub struct UParticleModuleCollision {
    __padding_end: [u8; 456],
}
impl UParticleModuleCollision {}
#[repr(C, align(8))]
pub struct UParticleModuleCollisionGPU {
    __padding_end: [u8; 176],
}
impl UParticleModuleCollisionGPU {}
#[repr(C, align(8))]
pub struct UParticleModuleColorBase {
    __padding_end: [u8; 56],
}
impl UParticleModuleColorBase {}
#[repr(C, align(8))]
pub struct UParticleModuleColor {
    __padding_end: [u8; 208],
}
impl UParticleModuleColor {}
#[repr(C, align(8))]
pub struct UParticleModuleColorOverLife {
    __padding_end: [u8; 208],
}
impl UParticleModuleColorOverLife {}
#[repr(C, align(8))]
pub struct UParticleModuleColorScaleOverLife {
    __padding_end: [u8; 208],
}
impl UParticleModuleColorScaleOverLife {}
#[repr(C, align(8))]
pub struct UParticleModuleColor_Seeded {
    __padding_end: [u8; 240],
}
impl UParticleModuleColor_Seeded {}
#[repr(C, align(8))]
pub struct UParticleModuleEventBase {
    __padding_end: [u8; 56],
}
impl UParticleModuleEventBase {}
#[repr(C, align(8))]
pub struct UParticleModuleEventGenerator {
    __padding_end: [u8; 72],
}
impl UParticleModuleEventGenerator {}
#[repr(C, align(8))]
pub struct UParticleModuleEventReceiverBase {
    __padding_end: [u8; 72],
}
impl UParticleModuleEventReceiverBase {}
#[repr(C, align(8))]
pub struct UParticleModuleEventReceiverKillParticles {
    __padding_end: [u8; 80],
}
impl UParticleModuleEventReceiverKillParticles {}
#[repr(C, align(8))]
pub struct UParticleModuleEventReceiverSpawn {
    __padding_end: [u8; 248],
}
impl UParticleModuleEventReceiverSpawn {}
#[repr(C, align(8))]
pub struct UParticleModuleEventSendToGame {
    __padding_end: [u8; 48],
}
impl UParticleModuleEventSendToGame {}
#[repr(C, align(8))]
pub struct UParticleModuleKillBase {
    __padding_end: [u8; 56],
}
impl UParticleModuleKillBase {}
#[repr(C, align(8))]
pub struct UParticleModuleKillBox {
    __padding_end: [u8; 256],
}
impl UParticleModuleKillBox {}
#[repr(C, align(8))]
pub struct UParticleModuleKillHeight {
    __padding_end: [u8; 112],
}
impl UParticleModuleKillHeight {}
#[repr(C, align(8))]
pub struct UParticleModuleLifetimeBase {
    __padding_end: [u8; 56],
}
impl UParticleModuleLifetimeBase {}
#[repr(C, align(8))]
pub struct UParticleModuleLifetime {
    __padding_end: [u8; 104],
}
impl UParticleModuleLifetime {}
#[repr(C, align(8))]
pub struct UParticleModuleLifetime_Seeded {
    __padding_end: [u8; 136],
}
impl UParticleModuleLifetime_Seeded {}
#[repr(C, align(8))]
pub struct UParticleModuleLightBase {
    __padding_end: [u8; 56],
}
impl UParticleModuleLightBase {}
#[repr(C, align(8))]
pub struct UParticleModuleLight {
    #[doc(hidden)]
    __padding_312: [u8; 312],
    pub inverse_exposure_blend: f32,
    pub lighting_channels: FLightingChannels,
    pub volumetric_scattering_intensity: f32,
    __padding_end: [u8; 4],
}
impl UParticleModuleLight {}
#[repr(C, align(8))]
pub struct UParticleModuleLight_Seeded {
    __padding_end: [u8; 360],
}
impl UParticleModuleLight_Seeded {}
#[repr(C, align(8))]
pub struct UParticleModuleLocationBase {
    __padding_end: [u8; 56],
}
impl UParticleModuleLocationBase {}
#[repr(C, align(8))]
pub struct UParticleModuleLocation {
    __padding_end: [u8; 160],
}
impl UParticleModuleLocation {}
#[repr(C, align(8))]
pub struct UParticleModuleLocationBoneSocket {
    __padding_end: [u8; 152],
}
impl UParticleModuleLocationBoneSocket {}
#[repr(C, align(8))]
pub struct UParticleModuleLocationDirect {
    __padding_end: [u8; 440],
}
impl UParticleModuleLocationDirect {}
#[repr(C, align(8))]
pub struct UParticleModuleLocationEmitter {
    __padding_end: [u8; 88],
}
impl UParticleModuleLocationEmitter {}
#[repr(C, align(8))]
pub struct UParticleModuleLocationEmitterDirect {
    __padding_end: [u8; 72],
}
impl UParticleModuleLocationEmitterDirect {}
#[repr(C, align(8))]
pub struct UParticleModuleLocationPrimitiveBase {
    __padding_end: [u8; 208],
}
impl UParticleModuleLocationPrimitiveBase {}
#[repr(C, align(8))]
pub struct UParticleModuleLocationPrimitiveCylinder {
    __padding_end: [u8; 320],
}
impl UParticleModuleLocationPrimitiveCylinder {}
#[repr(C, align(8))]
pub struct UParticleModuleLocationPrimitiveCylinder_Seeded {
    __padding_end: [u8; 352],
}
impl UParticleModuleLocationPrimitiveCylinder_Seeded {}
#[repr(C, align(8))]
pub struct UParticleModuleLocationPrimitiveSphere {
    __padding_end: [u8; 256],
}
impl UParticleModuleLocationPrimitiveSphere {}
#[repr(C, align(8))]
pub struct UParticleModuleLocationPrimitiveSphere_Seeded {
    __padding_end: [u8; 288],
}
impl UParticleModuleLocationPrimitiveSphere_Seeded {}
#[repr(C, align(8))]
pub struct UParticleModuleLocationPrimitiveTriangle {
    __padding_end: [u8; 296],
}
impl UParticleModuleLocationPrimitiveTriangle {}
#[repr(C, align(8))]
pub struct UParticleModuleLocationSkelVertSurface {
    __padding_end: [u8; 200],
}
impl UParticleModuleLocationSkelVertSurface {}
#[repr(C, align(8))]
pub struct UParticleModuleLocationWorldOffset {
    __padding_end: [u8; 160],
}
impl UParticleModuleLocationWorldOffset {}
#[repr(C, align(8))]
pub struct UParticleModuleLocationWorldOffset_Seeded {
    __padding_end: [u8; 192],
}
impl UParticleModuleLocationWorldOffset_Seeded {}
#[repr(C, align(8))]
pub struct UParticleModuleLocation_Seeded {
    __padding_end: [u8; 192],
}
impl UParticleModuleLocation_Seeded {}
#[repr(C, align(8))]
pub struct UParticleModuleMaterialBase {
    __padding_end: [u8; 56],
}
impl UParticleModuleMaterialBase {}
#[repr(C, align(8))]
pub struct UParticleModuleMeshMaterial {
    __padding_end: [u8; 72],
}
impl UParticleModuleMeshMaterial {}
#[repr(C, align(8))]
pub struct UParticleModuleRotationBase {
    __padding_end: [u8; 56],
}
impl UParticleModuleRotationBase {}
#[repr(C, align(8))]
pub struct UParticleModuleMeshRotation {
    __padding_end: [u8; 160],
}
impl UParticleModuleMeshRotation {}
#[repr(C, align(8))]
pub struct UParticleModuleRotationRateBase {
    __padding_end: [u8; 56],
}
impl UParticleModuleRotationRateBase {}
#[repr(C, align(8))]
pub struct UParticleModuleMeshRotationRate {
    __padding_end: [u8; 152],
}
impl UParticleModuleMeshRotationRate {}
#[repr(C, align(8))]
pub struct UParticleModuleMeshRotationRateMultiplyLife {
    __padding_end: [u8; 152],
}
impl UParticleModuleMeshRotationRateMultiplyLife {}
#[repr(C, align(8))]
pub struct UParticleModuleMeshRotationRateOverLife {
    __padding_end: [u8; 160],
}
impl UParticleModuleMeshRotationRateOverLife {}
#[repr(C, align(8))]
pub struct UParticleModuleMeshRotationRate_Seeded {
    __padding_end: [u8; 184],
}
impl UParticleModuleMeshRotationRate_Seeded {}
#[repr(C, align(8))]
pub struct UParticleModuleMeshRotation_Seeded {
    __padding_end: [u8; 192],
}
impl UParticleModuleMeshRotation_Seeded {}
#[repr(C, align(8))]
pub struct UParticleModuleOrbitBase {
    __padding_end: [u8; 64],
}
impl UParticleModuleOrbitBase {}
#[repr(C, align(8))]
pub struct UParticleModuleOrbit {
    __padding_end: [u8; 384],
}
impl UParticleModuleOrbit {}
#[repr(C, align(8))]
pub struct UParticleModuleOrientationBase {
    __padding_end: [u8; 56],
}
impl UParticleModuleOrientationBase {}
#[repr(C, align(8))]
pub struct UParticleModuleOrientationAxisLock {
    __padding_end: [u8; 64],
}
impl UParticleModuleOrientationAxisLock {}
#[repr(C, align(8))]
pub struct UParticleModuleParameterBase {
    __padding_end: [u8; 56],
}
impl UParticleModuleParameterBase {}
#[repr(C, align(8))]
pub struct UParticleModuleParameterDynamic {
    __padding_end: [u8; 80],
}
impl UParticleModuleParameterDynamic {}
#[repr(C, align(8))]
pub struct UParticleModuleParameterDynamic_Seeded {
    __padding_end: [u8; 112],
}
impl UParticleModuleParameterDynamic_Seeded {}
#[repr(C, align(8))]
pub struct UParticleModulePivotOffset {
    __padding_end: [u8; 72],
}
impl UParticleModulePivotOffset {}
#[repr(C, align(8))]
pub struct UParticleModuleRequired {
    __padding_end: [u8; 376],
}
impl UParticleModuleRequired {}
#[repr(C, align(8))]
pub struct UParticleModuleRotation {
    __padding_end: [u8; 104],
}
impl UParticleModuleRotation {}
#[repr(C, align(8))]
pub struct UParticleModuleRotationOverLifetime {
    __padding_end: [u8; 112],
}
impl UParticleModuleRotationOverLifetime {}
#[repr(C, align(8))]
pub struct UParticleModuleRotationRate {
    __padding_end: [u8; 104],
}
impl UParticleModuleRotationRate {}
#[repr(C, align(8))]
pub struct UParticleModuleRotationRateMultiplyLife {
    __padding_end: [u8; 104],
}
impl UParticleModuleRotationRateMultiplyLife {}
#[repr(C, align(8))]
pub struct UParticleModuleRotationRate_Seeded {
    __padding_end: [u8; 136],
}
impl UParticleModuleRotationRate_Seeded {}
#[repr(C, align(8))]
pub struct UParticleModuleRotation_Seeded {
    __padding_end: [u8; 136],
}
impl UParticleModuleRotation_Seeded {}
#[repr(C, align(8))]
pub struct UParticleModuleSizeBase {
    __padding_end: [u8; 56],
}
impl UParticleModuleSizeBase {}
#[repr(C, align(8))]
pub struct UParticleModuleSize {
    __padding_end: [u8; 152],
}
impl UParticleModuleSize {}
#[repr(C, align(8))]
pub struct UParticleModuleSizeMultiplyLife {
    __padding_end: [u8; 160],
}
impl UParticleModuleSizeMultiplyLife {}
#[repr(C, align(8))]
pub struct UParticleModuleSizeScale {
    __padding_end: [u8; 160],
}
impl UParticleModuleSizeScale {}
#[repr(C, align(8))]
pub struct UParticleModuleSizeScaleBySpeed {
    __padding_end: [u8; 88],
}
impl UParticleModuleSizeScaleBySpeed {}
#[repr(C, align(8))]
pub struct UParticleModuleSize_Seeded {
    __padding_end: [u8; 184],
}
impl UParticleModuleSize_Seeded {}
#[repr(C, align(8))]
pub struct UParticleModuleSourceMovement {
    __padding_end: [u8; 152],
}
impl UParticleModuleSourceMovement {}
#[repr(C, align(8))]
pub struct UParticleModuleSpawnBase {
    __padding_end: [u8; 64],
}
impl UParticleModuleSpawnBase {}
#[repr(C, align(8))]
pub struct UParticleModuleSpawn {
    __padding_end: [u8; 232],
}
impl UParticleModuleSpawn {}
#[repr(C, align(8))]
pub struct UParticleModuleSpawnPerUnit {
    __padding_end: [u8; 128],
}
impl UParticleModuleSpawnPerUnit {}
#[repr(C, align(8))]
pub struct UParticleModuleSubUVBase {
    __padding_end: [u8; 56],
}
impl UParticleModuleSubUVBase {}
#[repr(C, align(8))]
pub struct UParticleModuleSubUV {
    __padding_end: [u8; 120],
}
impl UParticleModuleSubUV {}
#[repr(C, align(8))]
pub struct UParticleModuleSubUVMovie {
    __padding_end: [u8; 184],
}
impl UParticleModuleSubUVMovie {}
#[repr(C, align(8))]
pub struct UParticleModuleTrailBase {
    __padding_end: [u8; 56],
}
impl UParticleModuleTrailBase {}
#[repr(C, align(8))]
pub struct UParticleModuleTrailSource {
    __padding_end: [u8; 152],
}
impl UParticleModuleTrailSource {}
#[repr(C, align(8))]
pub struct UParticleModuleTypeDataBase {
    __padding_end: [u8; 56],
}
impl UParticleModuleTypeDataBase {}
#[repr(C, align(8))]
pub struct UParticleModuleTypeDataAnimTrail {
    __padding_end: [u8; 80],
}
impl UParticleModuleTypeDataAnimTrail {}
#[repr(C, align(8))]
pub struct UParticleModuleTypeDataBeam2 {
    __padding_end: [u8; 344],
}
impl UParticleModuleTypeDataBeam2 {}
#[repr(C, align(16))]
pub struct UParticleModuleTypeDataGpu {
    __padding_end: [u8; 1504],
}
impl UParticleModuleTypeDataGpu {}
#[repr(C, align(8))]
pub struct UParticleModuleTypeDataMesh {
    __padding_end: [u8; 184],
}
impl UParticleModuleTypeDataMesh {}
#[repr(C, align(8))]
pub struct UParticleModuleTypeDataRibbon {
    __padding_end: [u8; 104],
}
impl UParticleModuleTypeDataRibbon {}
#[repr(C, align(8))]
pub struct UParticleModuleVectorFieldBase {
    __padding_end: [u8; 56],
}
impl UParticleModuleVectorFieldBase {}
#[repr(C, align(8))]
pub struct UParticleModuleVectorFieldGlobal {
    __padding_end: [u8; 72],
}
impl UParticleModuleVectorFieldGlobal {}
#[repr(C, align(8))]
pub struct UParticleModuleVectorFieldLocal {
    __padding_end: [u8; 152],
}
impl UParticleModuleVectorFieldLocal {}
#[repr(C, align(8))]
pub struct UParticleModuleVectorFieldRotation {
    __padding_end: [u8; 104],
}
impl UParticleModuleVectorFieldRotation {}
#[repr(C, align(8))]
pub struct UParticleModuleVectorFieldRotationRate {
    __padding_end: [u8; 80],
}
impl UParticleModuleVectorFieldRotationRate {}
#[repr(C, align(8))]
pub struct UParticleModuleVectorFieldScale {
    __padding_end: [u8; 112],
}
impl UParticleModuleVectorFieldScale {}
#[repr(C, align(8))]
pub struct UParticleModuleVectorFieldScaleOverLife {
    __padding_end: [u8; 112],
}
impl UParticleModuleVectorFieldScaleOverLife {}
#[repr(C, align(8))]
pub struct UParticleModuleVelocityBase {
    __padding_end: [u8; 64],
}
impl UParticleModuleVelocityBase {}
#[repr(C, align(8))]
pub struct UParticleModuleVelocity {
    __padding_end: [u8; 208],
}
impl UParticleModuleVelocity {}
#[repr(C, align(8))]
pub struct UParticleModuleVelocityCone {
    __padding_end: [u8; 184],
}
impl UParticleModuleVelocityCone {}
#[repr(C, align(8))]
pub struct UParticleModuleVelocityInheritParent {
    __padding_end: [u8; 160],
}
impl UParticleModuleVelocityInheritParent {}
#[repr(C, align(8))]
pub struct UParticleModuleVelocityOverLifetime {
    __padding_end: [u8; 168],
}
impl UParticleModuleVelocityOverLifetime {}
#[repr(C, align(8))]
pub struct UParticleModuleVelocity_Seeded {
    __padding_end: [u8; 240],
}
impl UParticleModuleVelocity_Seeded {}
#[repr(C, align(8))]
pub struct UParticleEmitter {
    __padding_end: [u8; 496],
}
impl UParticleEmitter {}
#[repr(C, align(8))]
pub struct UParticleSpriteEmitter {
    __padding_end: [u8; 496],
}
impl UParticleSpriteEmitter {}
#[repr(C, align(8))]
pub struct UParticleSystemReplay {
    __padding_end: [u8; 72],
}
impl UParticleSystemReplay {}
pub struct IPathFollowingAgentInterface {}
#[repr(C, align(8))]
pub struct UPathFollowingAgentInterface {
    __padding_end: [u8; 48],
}
impl UPathFollowingAgentInterface {}
#[repr(C, align(16))]
pub struct UPhysicsSpringComponent {
    #[doc(hidden)]
    __padding_656: [u8; 656],
    pub spring_stiffness: f32,
    pub spring_damping: f32,
    pub spring_length_at_rest: f32,
    pub spring_radius: f32,
    pub spring_channel: ECollisionChannel,
    pub b_ignore_self: bool,
    pub spring_compression: f32,
    __padding_end: [u8; 24],
}
impl UPhysicsSpringComponent {}
#[repr(C, align(8))]
pub struct UPhysicsThreadLibrary {
    __padding_end: [u8; 48],
}
impl UPhysicsThreadLibrary {}
#[repr(C, align(16))]
pub struct UPhysicsThrusterComponent {
    #[doc(hidden)]
    __padding_656: [u8; 656],
    pub thrust_strength: f32,
    __padding_end: [u8; 12],
}
impl UPhysicsThrusterComponent {}
#[repr(C, align(8))]
pub struct UPieFixupTestObject {
    __padding_end: [u8; 240],
}
impl UPieFixupTestObject {}
#[repr(C, align(8))]
pub struct ASceneCapture {
    #[doc(hidden)]
    __padding_1144: [u8; 1144],
    pub scene_component: UPtr<USceneComponent>,
}
impl ASceneCapture {}
#[repr(C, align(8))]
pub struct APlanarReflection {
    #[doc(hidden)]
    __padding_1152: [u8; 1152],
    pub planar_reflection_component: UPtr<UPlanarReflectionComponent>,
    __padding_end: [u8; 16],
}
impl APlanarReflection {}
#[repr(C, align(16))]
pub struct USceneCaptureComponent {
    #[doc(hidden)]
    __padding_656: [u8; 656],
    pub primitive_render_mode: ESceneCapturePrimitiveRenderMode,
    pub capture_source: ESceneCaptureSource,
    pub flags_658: u8,
    pub b_always_persist_rendering_state: bool,
    #[doc(hidden)]
    __padding_680: [u8; 16],
    pub hidden_actors: TArray<UPtr<AActor>>,
    #[doc(hidden)]
    __padding_712: [u8; 16],
    pub show_only_actors: TArray<UPtr<AActor>>,
    #[doc(hidden)]
    __padding_732: [u8; 4],
    pub max_view_distance_override: f32,
    pub capture_sort_priority: i32,
    pub b_use_ray_tracing_if_enabled: bool,
    pub collection_transform: UPtr<UMaterialParameterCollection>,
    pub collection_transform_world_to_local: FName,
    pub collection_transform_projection: FName,
    pub view_lighting_channels: FViewLightingChannels,
    pub show_flag_settings: TArray<FEngineShowFlagsSetting>,
    #[doc(hidden)]
    __padding_864: [u8; 64],
    pub profiling_event_name: FString,
    __padding_end: [u8; 80],
}
impl USceneCaptureComponent {}
#[repr(C, align(16))]
pub struct UPlanarReflectionComponent {
    __padding_end: [u8; 1344],
}
impl UPlanarReflectionComponent {}
#[repr(C, align(8))]
pub struct APlaneReflectionCapture {
    __padding_end: [u8; 1160],
}
impl APlaneReflectionCapture {}
#[repr(C, align(16))]
pub struct UPlaneReflectionCaptureComponent {
    #[doc(hidden)]
    __padding_768: [u8; 768],
    pub influence_radius_scale: f32,
    __padding_end: [u8; 28],
}
impl UPlaneReflectionCaptureComponent {}
#[repr(C, align(8))]
pub struct UPlatformInterfaceWebResponse {
    __padding_end: [u8; 184],
}
impl UPlatformInterfaceWebResponse {}
#[repr(C, align(8))]
pub struct APointLight {
    #[doc(hidden)]
    __padding_1152: [u8; 1152],
    pub point_light_component: UPtr<UPointLightComponent>,
}
impl APointLight {}
#[repr(C, align(8))]
pub struct UPolys {
    __padding_end: [u8; 64],
}
impl UPolys {}
#[repr(C, align(8))]
pub struct APrecomputedVisibilityOverrideVolume {
    #[doc(hidden)]
    __padding_1208: [u8; 1208],
    pub override_visible_actors: TArray<UPtr<AActor>>,
    pub override_invisible_actors: TArray<UPtr<AActor>>,
    pub override_invisible_levels: TArray<FName>,
}
impl APrecomputedVisibilityOverrideVolume {}
#[repr(C, align(8))]
pub struct ARigidBodyBase {
    __padding_end: [u8; 1136],
}
impl ARigidBodyBase {}
#[repr(C, align(8))]
pub struct ARadialForceActor {
    #[doc(hidden)]
    __padding_1136: [u8; 1136],
    pub force_component: UPtr<URadialForceComponent>,
    __padding_end: [u8; 8],
}
impl ARadialForceActor {}
#[repr(C, align(8))]
pub struct UReplicationDriver {
    __padding_end: [u8; 48],
}
impl UReplicationDriver {}
#[repr(C, align(8))]
pub struct UReplicationConnectionDriver {
    __padding_end: [u8; 48],
}
impl UReplicationConnectionDriver {}
pub struct IRVOAvoidanceInterface {}
#[repr(C, align(8))]
pub struct URVOAvoidanceInterface {
    __padding_end: [u8; 48],
}
impl URVOAvoidanceInterface {}
#[repr(C, align(8))]
pub struct ASceneCapture2D {
    #[doc(hidden)]
    __padding_1152: [u8; 1152],
    pub capture_component2_d: UPtr<USceneCaptureComponent2D>,
}
impl ASceneCapture2D {}
#[repr(C, align(16))]
pub struct USceneCaptureComponent2D {
    #[doc(hidden)]
    __padding_952: [u8; 952],
    pub projection_type: ECameraProjectionMode,
    pub fov_angle: f32,
    pub first_person_field_of_view: f32,
    pub first_person_scale: f32,
    pub ortho_width: f32,
    pub b_auto_calculate_ortho_planes: bool,
    pub auto_plane_shift: f32,
    pub b_update_ortho_planes: bool,
    pub b_use_camera_height_as_view_target: bool,
    pub overscan: f32,
    pub texture_target: UPtr<UTextureRenderTarget2D>,
    pub composite_mode: ESceneCaptureCompositeMode,
    pub post_process_settings: FPostProcessSettings,
    pub post_process_blend_weight: f32,
    pub flags_2964: u8,
    pub custom_near_clipping_plane: f32,
    pub b_use_custom_projection_matrix: bool,
    pub custom_projection_matrix: crate::bindings::core_u_object::FMatrix,
    #[doc(hidden)]
    __padding_3105: [u8; 1],
    pub b_enable_orthographic_tiling: bool,
    pub num_x_tiles: i32,
    pub num_y_tiles: i32,
    pub b_enable_clip_plane: bool,
    pub clip_plane_base: crate::bindings::core_u_object::FVector,
    pub clip_plane_normal: crate::bindings::core_u_object::FVector,
    pub b_render_in_main_renderer: bool,
    pub unlit_viewmode: ESceneCaptureUnlitViewmode,
    #[doc(hidden)]
    __padding_3172: [u8; 2],
    pub flags_3172: u8,
    pub flags_3173: u8,
    pub main_view_resolution_divisor: crate::bindings::core_u_object::FIntPoint,
    pub user_scene_texture_base_color: FName,
    pub user_scene_texture_normal: FName,
    pub user_scene_texture_scene_color: FName,
    __padding_end: [u8; 44],
}
impl USceneCaptureComponent2D {}
#[repr(C, align(16))]
pub struct USceneCaptureComponentCube {
    #[doc(hidden)]
    __padding_952: [u8; 952],
    pub texture_target: UPtr<UTextureRenderTargetCube>,
    pub b_capture_rotation: bool,
    pub post_process_settings: FPostProcessSettings,
    pub post_process_blend_weight: f32,
    __padding_end: [u8; 12],
}
impl USceneCaptureComponentCube {}
#[repr(C, align(8))]
pub struct ASceneCaptureCube {
    #[doc(hidden)]
    __padding_1152: [u8; 1152],
    pub capture_component_cube: UPtr<USceneCaptureComponentCube>,
}
impl ASceneCaptureCube {}
#[repr(C, align(16))]
pub struct UShadowMapTexture2D {
    __padding_end: [u8; 1376],
}
impl UShadowMapTexture2D {}
#[repr(C, align(8))]
pub struct USkeletalMeshSocket {
    #[doc(hidden)]
    __padding_48: [u8; 48],
    pub socket_name: FName,
    pub bone_name: FName,
    pub relative_location: crate::bindings::core_u_object::FVector,
    pub relative_rotation: crate::bindings::core_u_object::FRotator,
    pub relative_scale: crate::bindings::core_u_object::FVector,
    pub b_force_always_animated: bool,
    __padding_end: [u8; 39],
}
impl USkeletalMeshSocket {}
#[repr(C, align(8))]
pub struct ASkyLight {
    #[doc(hidden)]
    __padding_1144: [u8; 1144],
    pub light_component: UPtr<USkyLightComponent>,
    __padding_end: [u8; 8],
}
impl ASkyLight {}
pub struct ISMInstanceManager {}
#[repr(C, align(8))]
pub struct USMInstanceManager {
    __padding_end: [u8; 48],
}
impl USMInstanceManager {}
pub struct ISMInstanceManagerProvider {}
#[repr(C, align(8))]
pub struct USMInstanceManagerProvider {
    __padding_end: [u8; 48],
}
impl USMInstanceManagerProvider {}
#[repr(C, align(8))]
pub struct USoundAttenuationEditorSettings {
    __padding_end: [u8; 56],
}
impl USoundAttenuationEditorSettings {}
pub struct ISoundEffectPresetWidgetInterface {}
#[repr(C, align(8))]
pub struct USoundEffectPresetWidgetInterface {
    __padding_end: [u8; 48],
}
impl USoundEffectPresetWidgetInterface {}
#[repr(C, align(8))]
pub struct USoundEffectSourcePreset {
    __padding_end: [u8; 112],
}
impl USoundEffectSourcePreset {}
#[repr(C, align(8))]
pub struct USoundEffectSourcePresetChain {
    __padding_end: [u8; 72],
}
impl USoundEffectSourcePresetChain {}
pub struct ISoundSubmixWidgetInterface {}
#[repr(C, align(8))]
pub struct USoundSubmixWidgetInterface {
    __padding_end: [u8; 48],
}
impl USoundSubmixWidgetInterface {}
#[repr(C, align(8))]
pub struct ASphereReflectionCapture {
    __padding_end: [u8; 1168],
}
impl ASphereReflectionCapture {}
#[repr(C, align(16))]
pub struct USphereReflectionCaptureComponent {
    #[doc(hidden)]
    __padding_768: [u8; 768],
    pub influence_radius: f32,
    __padding_end: [u8; 12],
}
impl USphereReflectionCaptureComponent {}
#[repr(C, align(8))]
pub struct UStaticMeshSocket {
    #[doc(hidden)]
    __padding_48: [u8; 48],
    pub socket_name: FName,
    pub relative_location: crate::bindings::core_u_object::FVector,
    pub relative_rotation: crate::bindings::core_u_object::FRotator,
    pub relative_scale: crate::bindings::core_u_object::FVector,
    pub tag: FString,
    __padding_end: [u8; 48],
}
impl UStaticMeshSocket {}
pub struct IStreamingWorldSubsystemInterface {}
#[repr(C, align(8))]
pub struct UStreamingWorldSubsystemInterface {
    __padding_end: [u8; 48],
}
impl UStreamingWorldSubsystemInterface {}
pub struct ITedsTypedElementBridgeInterface {}
#[repr(C, align(8))]
pub struct UTedsTypedElementBridgeInterface {
    __padding_end: [u8; 48],
}
impl UTedsTypedElementBridgeInterface {}
#[repr(C, align(8))]
pub struct UTextPropertyTestObject {
    __padding_end: [u8; 96],
}
impl UTextPropertyTestObject {}
#[repr(C, align(8))]
pub struct ATextRenderActor {
    #[doc(hidden)]
    __padding_1136: [u8; 1136],
    pub text_render: UPtr<UTextRenderComponent>,
    __padding_end: [u8; 8],
}
impl ATextRenderActor {}
#[repr(C, align(8))]
pub struct UTextureEncodingProjectSettings {
    __padding_end: [u8; 128],
}
impl UTextureEncodingProjectSettings {}
#[repr(C, align(8))]
pub struct UTextureEncodingUserSettings {
    __padding_end: [u8; 112],
}
impl UTextureEncodingUserSettings {}
#[repr(C, align(8))]
pub struct UTimelineComponent {
    __padding_end: [u8; 448],
}
impl UTimelineComponent {}
#[repr(C, align(8))]
pub struct UTransformProviderData {
    __padding_end: [u8; 56],
}
impl UTransformProviderData {}
#[repr(C, align(8))]
pub struct ATriggerBase {
    #[doc(hidden)]
    __padding_1136: [u8; 1136],
    pub collision_component: UPtr<UShapeComponent>,
    pub sprite_component: UPtr<UBillboardComponent>,
}
impl ATriggerBase {}
#[repr(C, align(8))]
pub struct ATriggerBox {
    __padding_end: [u8; 1152],
}
impl ATriggerBox {}
#[repr(C, align(8))]
pub struct ATriggerCapsule {
    __padding_end: [u8; 1152],
}
impl ATriggerCapsule {}
#[repr(C, align(8))]
pub struct ATriggerSphere {
    __padding_end: [u8; 1152],
}
impl ATriggerSphere {}
#[repr(C, align(8))]
pub struct UTwitterIntegrationBase {
    __padding_end: [u8; 64],
}
impl UTwitterIntegrationBase {}
#[repr(C, align(8))]
pub struct UVectorField {
    __padding_end: [u8; 112],
}
impl UVectorField {}
#[repr(C, align(8))]
pub struct UVectorFieldAnimated {
    __padding_end: [u8; 176],
}
impl UVectorFieldAnimated {}
#[repr(C, align(16))]
pub struct UVectorFieldComponent {
    #[doc(hidden)]
    __padding_1504: [u8; 1504],
    pub vector_field: UPtr<UVectorField>,
    pub intensity: f32,
    pub tightness: f32,
    __padding_end: [u8; 32],
}
impl UVectorFieldComponent {}
#[repr(C, align(8))]
pub struct UVectorFieldStatic {
    __padding_end: [u8; 256],
}
impl UVectorFieldStatic {}
pub struct IVisualLoggerDebugSnapshotInterface {}
#[repr(C, align(8))]
pub struct UVisualLoggerDebugSnapshotInterface {
    __padding_end: [u8; 48],
}
impl UVisualLoggerDebugSnapshotInterface {}
#[repr(C, align(16))]
pub struct UWindDirectionalSourceComponent {
    #[doc(hidden)]
    __padding_656: [u8; 656],
    pub strength: f32,
    pub speed: f32,
    pub min_gust_amount: f32,
    pub max_gust_amount: f32,
    pub radius: f32,
    pub flags_676: u8,
    __padding_end: [u8; 11],
}
impl UWindDirectionalSourceComponent {}
#[repr(C, align(8))]
pub struct UWorldPartitionEditorLoaderAdapter {
    __padding_end: [u8; 64],
}
impl UWorldPartitionEditorLoaderAdapter {}
#[repr(C, align(16))]
pub struct UTexture2DArray {
    __padding_end: [u8; 1360],
}
impl UTexture2DArray {}
#[repr(C, align(8))]
pub struct UMaterialExpressionMaterialSample {
    __padding_end: [u8; 208],
}
impl UMaterialExpressionMaterialSample {}
#[repr(C, align(8))]
pub struct UActorFolder {
    __padding_end: [u8; 104],
}
impl UActorFolder {}
#[repr(C, align(8))]
pub struct UActorPartitionSubsystem {
    __padding_end: [u8; 160],
}
impl UActorPartitionSubsystem {}
#[repr(C, align(8))]
pub struct UNullNavSysConfig {
    __padding_end: [u8; 112],
}
impl UNullNavSysConfig {}
#[repr(C, align(8))]
pub struct UAvoidanceManager {
    __padding_end: [u8; 256],
}
impl UAvoidanceManager {}
#[repr(C, align(8))]
pub struct AAmbientSound {
    #[doc(hidden)]
    __padding_1136: [u8; 1136],
    pub audio_component: UPtr<UAudioComponent>,
}
impl AAmbientSound {}
#[repr(C, align(8))]
pub struct UAnimationAsset {
    __padding_end: [u8; 232],
}
impl UAnimationAsset {}
#[repr(C, align(8))]
pub struct UBlendSpace {
    __padding_end: [u8; 712],
}
impl UBlendSpace {}
#[repr(C, align(8))]
pub struct UAimOffsetBlendSpace {
    __padding_end: [u8; 712],
}
impl UAimOffsetBlendSpace {}
#[repr(C, align(8))]
pub struct UBlendSpace1D {
    __padding_end: [u8; 720],
}
impl UBlendSpace1D {}
#[repr(C, align(8))]
pub struct UAimOffsetBlendSpace1D {
    __padding_end: [u8; 720],
}
impl UAimOffsetBlendSpace1D {}
#[repr(C, align(8))]
pub struct UAnimationSettings {
    __padding_end: [u8; 408],
}
impl UAnimationSettings {}
#[repr(C, align(8))]
pub struct UAnimBank {
    #[doc(hidden)]
    __padding_240: [u8; 240],
    pub sequences: TArray<FAnimBankSequence>,
    pub asset: UPtr<USkinnedAsset>,
    __padding_end: [u8; 352],
}
impl UAnimBank {}
#[repr(C, align(8))]
pub struct UAnimBankData {
    #[doc(hidden)]
    __padding_56: [u8; 56],
    pub anim_bank_items: TArray<FAnimBankItem>,
}
impl UAnimBankData {}
#[repr(C, align(8))]
pub struct UAnimBlueprintGeneratedStruct {
    __padding_end: [u8; 224],
}
impl UAnimBlueprintGeneratedStruct {}
#[repr(C, align(8))]
pub struct UAnimBlueprintGeneratedClass {
    __padding_end: [u8; 3352],
}
impl UAnimBlueprintGeneratedClass {}
#[repr(C, align(8))]
pub struct UAnimBoneCompressionCodec {
    __padding_end: [u8; 64],
}
impl UAnimBoneCompressionCodec {}
#[repr(C, align(8))]
pub struct UAnimBoneCompressionSettings {
    __padding_end: [u8; 72],
}
impl UAnimBoneCompressionSettings {}
pub struct IAnimClassInterface {}
#[repr(C, align(8))]
pub struct UAnimClassInterface {
    __padding_end: [u8; 48],
}
impl UAnimClassInterface {}
#[repr(C, align(8))]
pub struct UAnimSequenceBase {
    #[doc(hidden)]
    __padding_248: [u8; 248],
    pub sequence_length: f32,
    #[doc(hidden)]
    __padding_360: [u8; 104],
    pub data_model: UPtr<UAnimDataModel>,
    pub data_model_interface: TScriptInterface<UAnimationDataModel>,
    #[doc(hidden)]
    __padding_392: [u8; 8],
    pub controller: TScriptInterface<UAnimationDataController>,
    __padding_end: [u8; 88],
}
impl UAnimSequenceBase {}
#[repr(C, align(8))]
pub struct UAnimCompositeBase {
    __padding_end: [u8; 504],
}
impl UAnimCompositeBase {}
#[repr(C, align(8))]
pub struct UAnimComposite {
    __padding_end: [u8; 528],
}
impl UAnimComposite {}
#[repr(C, align(8))]
pub struct UAnimCompress {
    __padding_end: [u8; 72],
}
impl UAnimCompress {}
#[repr(C, align(8))]
pub struct UAnimCompress_BitwiseCompressOnly {
    __padding_end: [u8; 72],
}
impl UAnimCompress_BitwiseCompressOnly {}
#[repr(C, align(8))]
pub struct UAnimCompress_LeastDestructive {
    __padding_end: [u8; 72],
}
impl UAnimCompress_LeastDestructive {}
#[repr(C, align(8))]
pub struct UAnimCompress_RemoveLinearKeys {
    __padding_end: [u8; 104],
}
impl UAnimCompress_RemoveLinearKeys {}
#[repr(C, align(8))]
pub struct UAnimCompress_PerTrackCompression {
    __padding_end: [u8; 224],
}
impl UAnimCompress_PerTrackCompression {}
#[repr(C, align(8))]
pub struct UAnimCompress_RemoveEverySecondKey {
    __padding_end: [u8; 80],
}
impl UAnimCompress_RemoveEverySecondKey {}
#[repr(C, align(8))]
pub struct UAnimCompress_RemoveTrivialKeys {
    __padding_end: [u8; 88],
}
impl UAnimCompress_RemoveTrivialKeys {}
#[repr(C, align(8))]
pub struct UAnimCurveCompressionCodec {
    __padding_end: [u8; 48],
}
impl UAnimCurveCompressionCodec {}
#[repr(C, align(8))]
pub struct UAnimCurveCompressionCodec_CompressedRichCurve {
    __padding_end: [u8; 64],
}
impl UAnimCurveCompressionCodec_CompressedRichCurve {}
#[repr(C, align(8))]
pub struct UAnimCurveCompressionCodec_UniformIndexable {
    __padding_end: [u8; 48],
}
impl UAnimCurveCompressionCodec_UniformIndexable {}
#[repr(C, align(8))]
pub struct UAnimCurveCompressionCodec_UniformlySampled {
    __padding_end: [u8; 56],
}
impl UAnimCurveCompressionCodec_UniformlySampled {}
#[repr(C, align(8))]
pub struct UAnimCurveCompressionSettings {
    __padding_end: [u8; 56],
}
impl UAnimCurveCompressionSettings {}
#[repr(C, align(8))]
pub struct UAnimDataModel {
    #[doc(hidden)]
    __padding_120: [u8; 120],
    pub bone_animation_tracks: TArray<FBoneAnimationTrack>,
    pub play_length: f32,
    pub frame_rate: crate::bindings::core_u_object::FFrameRate,
    pub number_of_frames: i32,
    pub number_of_keys: i32,
    pub curve_data: FAnimationCurveData,
    pub animated_bone_attributes: TArray<FAnimatedBoneAttribute>,
    __padding_end: [u8; 16],
}
impl UAnimDataModel {}
pub struct IAnimationDataController {}
#[repr(C, align(8))]
pub struct UAnimationDataController {
    __padding_end: [u8; 48],
}
impl UAnimationDataController {}
pub struct IAnimationDataModel {}
#[repr(C, align(8))]
pub struct UAnimationDataModel {
    __padding_end: [u8; 48],
}
impl UAnimationDataModel {}
#[repr(C, align(8))]
pub struct UAnimMetaData {
    __padding_end: [u8; 48],
}
impl UAnimMetaData {}
#[repr(C, align(8))]
pub struct UAnimMontage {
    #[doc(hidden)]
    __padding_504: [u8; 504],
    pub blend_mode_in: EMontageBlendMode,
    pub blend_mode_out: EMontageBlendMode,
    #[doc(hidden)]
    __padding_728: [u8; 216],
    pub blend_profile_in: UPtr<UBlendProfile>,
    pub blend_profile_out: UPtr<UBlendProfile>,
    __padding_end: [u8; 136],
}
impl UAnimMontage {}
#[repr(C, align(8))]
pub struct UAnimNotifyLibrary {
    __padding_end: [u8; 48],
}
impl UAnimNotifyLibrary {}
#[repr(C, align(8))]
pub struct UAnimNotifyMirrorInspectionLibrary {
    __padding_end: [u8; 48],
}
impl UAnimNotifyMirrorInspectionLibrary {}
#[repr(C, align(8))]
pub struct UAnimNotifyStateMachineInspectionLibrary {
    __padding_end: [u8; 48],
}
impl UAnimNotifyStateMachineInspectionLibrary {}
#[repr(C, align(8))]
pub struct UAnimNotifyState_DisableRootMotion {
    __padding_end: [u8; 56],
}
impl UAnimNotifyState_DisableRootMotion {}
#[repr(C, align(8))]
pub struct UAnimNotifyState_TimedParticleEffect {
    __padding_end: [u8; 168],
}
impl UAnimNotifyState_TimedParticleEffect {}
#[repr(C, align(8))]
pub struct UAnimNotifyState_Trail {
    #[doc(hidden)]
    __padding_56: [u8; 56],
    pub ps_template: UPtr<UParticleSystem>,
    pub first_socket_name: FName,
    pub second_socket_name: FName,
    pub width_scale_mode: ETrailWidthMode,
    pub width_scale_curve: FName,
    pub flags_104: u8,
    __padding_end: [u8; 7],
}
impl UAnimNotifyState_Trail {}
#[repr(C, align(8))]
pub struct UAnimNotify_PauseClothingSimulation {
    __padding_end: [u8; 64],
}
impl UAnimNotify_PauseClothingSimulation {}
#[repr(C, align(16))]
pub struct UAnimNotify_PlayParticleEffect {
    #[doc(hidden)]
    __padding_64: [u8; 64],
    pub ps_template: UPtr<UParticleSystem>,
    pub location_offset: crate::bindings::core_u_object::FVector,
    pub rotation_offset: crate::bindings::core_u_object::FRotator,
    #[doc(hidden)]
    __padding_176: [u8; 56],
    pub flags_176: u8,
    pub socket_name: FName,
}
impl UAnimNotify_PlayParticleEffect {}
#[repr(C, align(8))]
pub struct UAnimNotify_PlaySound {
    #[doc(hidden)]
    __padding_64: [u8; 64],
    pub sound: UPtr<USoundBase>,
    pub volume_multiplier: f32,
    pub pitch_multiplier: f32,
    pub flags_80: u8,
    pub attach_name: FName,
}
impl UAnimNotify_PlaySound {}
#[repr(C, align(8))]
pub struct UAnimNotify_ResetClothingSimulation {
    __padding_end: [u8; 64],
}
impl UAnimNotify_ResetClothingSimulation {}
#[repr(C, align(8))]
pub struct UAnimNotify_ResetDynamics {
    __padding_end: [u8; 64],
}
impl UAnimNotify_ResetDynamics {}
#[repr(C, align(8))]
pub struct UAnimNotify_ResumeClothingSimulation {
    __padding_end: [u8; 64],
}
impl UAnimNotify_ResumeClothingSimulation {}
#[repr(C, align(8))]
pub struct UAnimSequence {
    __padding_end: [u8; 1680],
}
impl UAnimSequence {}
#[repr(C, align(8))]
pub struct UAnimSet {
    __padding_end: [u8; 272],
}
impl UAnimSet {}
#[repr(C, align(8))]
pub struct UAnimStateMachineTypes {
    __padding_end: [u8; 48],
}
impl UAnimStateMachineTypes {}
#[repr(C, align(8))]
pub struct UAnimStreamable {
    __padding_end: [u8; 728],
}
impl UAnimStreamable {}
#[repr(C, align(8))]
pub struct URawAnimSequenceTrackExtensions {
    __padding_end: [u8; 48],
}
impl URawAnimSequenceTrackExtensions {}
#[repr(C, align(8))]
pub struct UAssetMappingTable {
    __padding_end: [u8; 64],
}
impl UAssetMappingTable {}
#[repr(C, align(8))]
pub struct UAnimationAttributeIdentifierExtensions {
    __padding_end: [u8; 48],
}
impl UAnimationAttributeIdentifierExtensions {}
pub struct IBlendProfileProviderInterface {}
#[repr(C, align(8))]
pub struct UBlendProfileProviderInterface {
    __padding_end: [u8; 48],
}
impl UBlendProfileProviderInterface {}
#[repr(C, align(8))]
pub struct UBlendProfile {
    __padding_end: [u8; 88],
}
impl UBlendProfile {}
#[repr(C, align(8))]
pub struct UAnalysisProperties {
    __padding_end: [u8; 64],
}
impl UAnalysisProperties {}
#[repr(C, align(8))]
pub struct UBoneMaskFilter {
    __padding_end: [u8; 64],
}
impl UBoneMaskFilter {}
#[repr(C, align(8))]
pub struct UCachedAnimDataLibrary {
    __padding_end: [u8; 48],
}
impl UCachedAnimDataLibrary {}
#[repr(C, align(8))]
pub struct UAnimationCurveIdentifierExtensions {
    __padding_end: [u8; 48],
}
impl UAnimationCurveIdentifierExtensions {}
pub struct ICurveSourceInterface {}
#[repr(C, align(8))]
pub struct UCurveSourceInterface {
    __padding_end: [u8; 48],
}
impl UCurveSourceInterface {}
#[repr(C, align(8))]
pub struct UMeshDeformerCollection {
    __padding_end: [u8; 104],
}
impl UMeshDeformerCollection {}
#[repr(C, align(8))]
pub struct UDataTable {
    __padding_end: [u8; 352],
}
impl UDataTable {}
#[repr(C, align(8))]
pub struct UMirrorDataTable {
    __padding_end: [u8; 672],
}
impl UMirrorDataTable {}
#[repr(C, align(8))]
pub struct UNodeMappingContainer {
    __padding_end: [u8; 384],
}
impl UNodeMappingContainer {}
pub struct INodeMappingProviderInterface {}
#[repr(C, align(8))]
pub struct UNodeMappingProviderInterface {
    __padding_end: [u8; 48],
}
impl UNodeMappingProviderInterface {}
#[repr(C, align(8))]
pub struct UPoseAsset {
    __padding_end: [u8; 504],
}
impl UPoseAsset {}
pub struct IPreviewCollectionInterface {}
#[repr(C, align(8))]
pub struct UPreviewCollectionInterface {
    __padding_end: [u8; 48],
}
impl UPreviewCollectionInterface {}
#[repr(C, align(8))]
pub struct UPreviewMeshCollection {
    __padding_end: [u8; 88],
}
impl UPreviewMeshCollection {}
#[repr(C, align(8))]
pub struct USkeleton {
    #[doc(hidden)]
    __padding_432: [u8; 432],
    pub compatible_skeletons: TArray<TSoftObjectPtr<USkeleton>>,
    __padding_end: [u8; 752],
}
impl USkeleton {}
#[repr(C, align(8))]
pub struct UTransformTrajectoryBlueprintLibrary {
    __padding_end: [u8; 48],
}
impl UTransformTrajectoryBlueprintLibrary {}
#[repr(C, align(8))]
pub struct UVariableFrameStrippingSettings {
    __padding_end: [u8; 224],
}
impl UVariableFrameStrippingSettings {}
#[repr(C, align(8))]
pub struct UAnimBlueprint {
    __padding_end: [u8; 1712],
}
impl UAnimBlueprint {}
#[repr(C, align(8))]
pub struct UAssetManager {
    __padding_end: [u8; 1568],
}
impl UAssetManager {}
#[repr(C, align(8))]
pub struct UAsyncActionLoadPrimaryAssetBase {
    __padding_end: [u8; 128],
}
impl UAsyncActionLoadPrimaryAssetBase {}
#[repr(C, align(8))]
pub struct UAsyncActionLoadPrimaryAsset {
    __padding_end: [u8; 152],
}
impl UAsyncActionLoadPrimaryAsset {}
#[repr(C, align(8))]
pub struct UAsyncActionLoadPrimaryAssetClass {
    __padding_end: [u8; 152],
}
impl UAsyncActionLoadPrimaryAssetClass {}
#[repr(C, align(8))]
pub struct UAsyncActionLoadPrimaryAssetList {
    __padding_end: [u8; 152],
}
impl UAsyncActionLoadPrimaryAssetList {}
#[repr(C, align(8))]
pub struct UAsyncActionLoadPrimaryAssetClassList {
    __padding_end: [u8; 152],
}
impl UAsyncActionLoadPrimaryAssetClassList {}
#[repr(C, align(8))]
pub struct UAsyncActionChangePrimaryAssetBundles {
    __padding_end: [u8; 152],
}
impl UAsyncActionChangePrimaryAssetBundles {}
#[repr(C, align(8))]
pub struct AAtmosphericFog {
    #[doc(hidden)]
    __padding_1144: [u8; 1144],
    pub atmospheric_fog_component: UPtr<UAtmosphericFogComponent>,
    __padding_end: [u8; 8],
}
impl AAtmosphericFog {}
#[repr(C, align(16))]
pub struct USkyAtmosphereComponent {
    #[doc(hidden)]
    __padding_656: [u8; 656],
    pub transform_mode: ESkyAtmosphereTransformMode,
    pub bottom_radius: f32,
    pub ground_albedo: crate::bindings::core_u_object::FColor,
    pub atmosphere_height: f32,
    pub multi_scattering_factor: f32,
    pub trace_sample_count_scale: f32,
    pub rayleigh_scattering_scale: f32,
    pub rayleigh_scattering: crate::bindings::core_u_object::FLinearColor,
    pub rayleigh_exponential_distribution: f32,
    pub mie_scattering_scale: f32,
    pub mie_scattering: crate::bindings::core_u_object::FLinearColor,
    pub mie_absorption_scale: f32,
    pub mie_absorption: crate::bindings::core_u_object::FLinearColor,
    pub mie_anisotropy: f32,
    pub mie_exponential_distribution: f32,
    pub other_absorption_scale: f32,
    pub other_absorption: crate::bindings::core_u_object::FLinearColor,
    pub other_tent_distribution: FTentDistribution,
    pub sky_luminance_factor: crate::bindings::core_u_object::FLinearColor,
    pub sky_and_aerial_perspective_luminance_factor: crate::bindings::core_u_object::FLinearColor,
    pub aerial_pespective_view_distance_scale: f32,
    pub height_fog_contribution: f32,
    pub transmittance_min_light_elevation_angle: f32,
    pub aerial_perspective_start_depth: f32,
    pub flags_832: u8,
    __padding_end: [u8; 95],
}
impl USkyAtmosphereComponent {}
#[repr(C, align(16))]
pub struct UAtmosphericFogComponent {
    __padding_end: [u8; 928],
}
impl UAtmosphericFogComponent {}
#[repr(C, align(8))]
pub struct UAudioBus {
    __padding_end: [u8; 64],
}
impl UAudioBus {}
#[repr(C, align(8))]
pub struct UAudioSettings {
    __padding_end: [u8; 872],
}
impl UAudioSettings {}
#[repr(C, align(8))]
pub struct AAudioVolume {
    #[doc(hidden)]
    __padding_1208: [u8; 1208],
    pub priority: f32,
    pub flags_1212: u8,
    pub settings: FReverbSettings,
    pub ambient_zone_settings: FInteriorSettings,
    pub submix_send_settings: TArray<FAudioVolumeSubmixSendSettings>,
    pub submix_override_settings: TArray<FAudioVolumeSubmixOverrideSettings>,
}
impl AAudioVolume {}
pub struct IActorSoundParameterInterface {}
#[repr(C, align(8))]
pub struct UActorSoundParameterInterface {
    __padding_end: [u8; 48],
}
impl UActorSoundParameterInterface {}
#[repr(C, align(8))]
pub struct UAudioWidgetSubsystem {
    __padding_end: [u8; 56],
}
impl UAudioWidgetSubsystem {}
pub struct ISoundParameterControllerInterface {}
#[repr(C, align(8))]
pub struct USoundParameterControllerInterface {
    __padding_end: [u8; 48],
}
impl USoundParameterControllerInterface {}
#[repr(C, align(8))]
pub struct UAudioParameterConversionStatics {
    __padding_end: [u8; 48],
}
impl UAudioParameterConversionStatics {}
#[repr(C, align(8))]
pub struct ABlockingVolume {
    __padding_end: [u8; 1208],
}
impl ABlockingVolume {}
#[repr(C, align(8))]
pub struct UDEPRECATED_Breakpoint {
    __padding_end: [u8; 48],
}
impl UDEPRECATED_Breakpoint {}
#[repr(C, align(8))]
pub struct UBlueprintInstancedStructLibrary {
    __padding_end: [u8; 48],
}
impl UBlueprintInstancedStructLibrary {}
#[repr(C, align(8))]
pub struct UBlueprintMapLibrary {
    __padding_end: [u8; 48],
}
impl UBlueprintMapLibrary {}
#[repr(C, align(8))]
pub struct UBlueprintSetLibrary {
    __padding_end: [u8; 48],
}
impl UBlueprintSetLibrary {}
#[repr(C, align(8))]
pub struct UBookMark {
    __padding_end: [u8; 112],
}
impl UBookMark {}
#[repr(C, align(8))]
pub struct ABrushShape {
    __padding_end: [u8; 1208],
}
impl ABrushShape {}
#[repr(C, align(8))]
pub struct ACameraBlockingVolume {
    __padding_end: [u8; 1208],
}
impl ACameraBlockingVolume {}
#[repr(C, align(16))]
pub struct UCameraProxyMeshComponent {
    __padding_end: [u8; 1888],
}
impl UCameraProxyMeshComponent {}
pub struct ICameraLensEffectInterface {}
#[repr(C, align(8))]
pub struct UCameraLensEffectInterface {
    __padding_end: [u8; 48],
}
impl UCameraLensEffectInterface {}
#[repr(C, align(8))]
pub struct UCameraLensEffectInterfaceClassSupportLibrary {
    __padding_end: [u8; 48],
}
impl UCameraLensEffectInterfaceClassSupportLibrary {}
#[repr(C, align(8))]
pub struct UCameraModifier {
    #[doc(hidden)]
    __padding_48: [u8; 48],
    pub flags_48: u8,
    #[doc(hidden)]
    __padding_52: [u8; 3],
    pub priority: u8,
    pub camera_owner: UPtr<APlayerCameraManager>,
    pub alpha_in_time: f32,
    pub alpha_out_time: f32,
    pub alpha: f32,
    __padding_end: [u8; 4],
}
impl UCameraModifier {}
#[repr(C, align(8))]
pub struct UCameraModifier_CameraShake {
    __padding_end: [u8; 208],
}
impl UCameraModifier_CameraShake {}
#[repr(C, align(8))]
pub struct UCameraShakePattern {
    __padding_end: [u8; 48],
}
impl UCameraShakePattern {}
#[repr(C, align(8))]
pub struct ACameraShakeSourceActor {
    #[doc(hidden)]
    __padding_1136: [u8; 1136],
    pub camera_shake_source_component: UPtr<UCameraShakeSourceComponent>,
}
impl ACameraShakeSourceActor {}
#[repr(C, align(16))]
pub struct UCameraShakeSourceComponent {
    #[doc(hidden)]
    __padding_664: [u8; 664],
    pub attenuation: ECameraShakeAttenuation,
    pub inner_attenuation_radius: f32,
    pub outer_attenuation_radius: f32,
    pub camera_shake: TSubclassOf<UCameraShakeBase>,
    pub b_auto_start: bool,
    __padding_end: [u8; 31],
}
impl UCameraShakeSourceComponent {}
#[repr(C, align(16))]
pub struct UTextureRenderTarget {
    #[doc(hidden)]
    __padding_1240: [u8; 1240],
    pub target_gamma: f32,
    __padding_end: [u8; 4],
}
impl UTextureRenderTarget {}
#[repr(C, align(16))]
pub struct UTextureRenderTarget2D {
    #[doc(hidden)]
    __padding_1248: [u8; 1248],
    pub size_x: i32,
    pub size_y: i32,
    pub clear_color: crate::bindings::core_u_object::FLinearColor,
    pub address_x: TextureAddress,
    pub address_y: TextureAddress,
    pub flags_1274: u8,
    pub render_target_format: ETextureRenderTargetFormat,
    pub flags_1276: u8,
    pub mips_sampler_filter: TextureFilter,
    pub mips_address_u: TextureAddress,
    pub mips_address_v: TextureAddress,
    __padding_end: [u8; 16],
}
impl UTextureRenderTarget2D {}
#[repr(C, align(16))]
pub struct UCanvasRenderTarget2D {
    #[doc(hidden)]
    __padding_1320: [u8; 1320],
    pub sample_count: ETextureRenderTargetSampleCount,
    __padding_end: [u8; 7],
}
impl UCanvasRenderTarget2D {}
#[repr(C, align(8))]
pub struct APlayerController {
    #[doc(hidden)]
    __padding_1352: [u8; 1352],
    pub player_camera_manager: UPtr<APlayerCameraManager>,
    pub player_camera_manager_class: TSubclassOf<APlayerCameraManager>,
    #[doc(hidden)]
    __padding_1424: [u8; 56],
    pub smooth_target_view_rotation_speed: f32,
    #[doc(hidden)]
    __padding_1528: [u8; 96],
    pub cheat_manager: UPtr<UCheatManager>,
    pub cheat_class: TSubclassOf<UCheatManager>,
    #[doc(hidden)]
    __padding_1712: [u8; 168],
    pub flags_1712: u8,
    #[doc(hidden)]
    __padding_1860: [u8; 147],
    pub flags_1860: u8,
    pub flags_1861: u8,
    #[doc(hidden)]
    __padding_1864: [u8; 2],
    pub streaming_source_priority: EStreamingSourcePriority,
    pub streaming_source_debug_color: crate::bindings::core_u_object::FColor,
    pub streaming_source_shapes: TArray<FStreamingSourceShape>,
    #[doc(hidden)]
    __padding_1896: [u8; 8],
    pub click_event_keys: TArray<crate::bindings::input_core::FKey>,
    pub default_mouse_cursor: crate::bindings::core_u_object::EMouseCursor,
    pub current_mouse_cursor: crate::bindings::core_u_object::EMouseCursor,
    pub default_click_trace_channel: ECollisionChannel,
    pub current_click_trace_channel: ECollisionChannel,
    pub hit_result_trace_distance: f32,
    #[doc(hidden)]
    __padding_2064: [u8; 144],
    pub flags_2064: u8,
    __padding_end: [u8; 303],
}
impl APlayerController {}
#[repr(C, align(8))]
pub struct UCheatManager {
    #[doc(hidden)]
    __padding_56: [u8; 56],
    pub debug_camera_controller_class: TSubclassOf<ADebugCameraController>,
    __padding_end: [u8; 80],
}
impl UCheatManager {}
#[repr(C, align(8))]
pub struct UCheatManagerExtension {
    __padding_end: [u8; 48],
}
impl UCheatManagerExtension {}
#[repr(C, align(8))]
pub struct UCollisionProfile {
    __padding_end: [u8; 424],
}
impl UCollisionProfile {}
#[repr(C, align(8))]
pub struct UPluginCommandlet {
    __padding_end: [u8; 168],
}
impl UPluginCommandlet {}
#[repr(C, align(8))]
pub struct USmokeTestCommandlet {
    __padding_end: [u8; 136],
}
impl USmokeTestCommandlet {}
#[repr(C, align(8))]
pub struct UComponentDelegateBinding {
    __padding_end: [u8; 64],
}
impl UComponentDelegateBinding {}
#[repr(C, align(8))]
pub struct UApplicationLifecycleComponent {
    __padding_end: [u8; 456],
}
impl UApplicationLifecycleComponent {}
#[repr(C, align(16))]
pub struct UArrowComponent {
    #[doc(hidden)]
    __padding_1504: [u8; 1504],
    pub arrow_color: crate::bindings::core_u_object::FColor,
    pub arrow_size: f32,
    pub arrow_length: f32,
    pub screen_size: f32,
    pub flags_1520: u8,
    #[doc(hidden)]
    __padding_1588: [u8; 67],
    pub b_use_in_editor_scaling: bool,
    __padding_end: [u8; 11],
}
impl UArrowComponent {}
#[repr(C, align(8))]
pub struct UInitialActiveSoundParams {
    #[doc(hidden)]
    __padding_48: [u8; 48],
    pub audio_params: TArray<crate::bindings::audio_extensions::FAudioParameter>,
}
impl UInitialActiveSoundParams {}
#[repr(C, align(16))]
pub struct UAudioComponent {
    #[doc(hidden)]
    __padding_1240: [u8; 1240],
    pub sound: UPtr<USoundBase>,
    pub default_parameters: TArray<crate::bindings::audio_extensions::FAudioParameter>,
    #[doc(hidden)]
    __padding_1288: [u8; 24],
    pub flags_1288: u8,
    pub flags_1289: u8,
    #[doc(hidden)]
    __padding_1291: [u8; 1],
    pub flags_1291: u8,
    #[doc(hidden)]
    __padding_1308: [u8; 16],
    pub pitch_modulation_min: f32,
    pub pitch_modulation_max: f32,
    pub volume_modulation_min: f32,
    pub volume_modulation_max: f32,
    pub volume_multiplier: f32,
    pub envelope_follower_attack_time: i32,
    pub envelope_follower_release_time: i32,
    pub priority: f32,
    pub subtitle_priority: f32,
    pub source_effect_chain: UPtr<USoundEffectSourcePresetChain>,
    #[doc(hidden)]
    __padding_1360: [u8; 8],
    pub pitch_multiplier: f32,
    pub low_pass_filter_frequency: f32,
    pub high_pass_filter_frequency: f32,
    pub attenuation_settings: UPtr<USoundAttenuation>,
    pub attenuation_overrides: FSoundAttenuationSettings,
    #[doc(hidden)]
    __padding_2416: [u8; 8],
    pub concurrency_set: TSet<UPtr<USoundConcurrency>>,
    #[doc(hidden)]
    __padding_2508: [u8; 12],
    pub auto_attach_location_rule: EAttachmentRule,
    pub auto_attach_rotation_rule: EAttachmentRule,
    pub auto_attach_scale_rule: EAttachmentRule,
    pub modulation_routing: FSoundModulationDefaultRoutingSettings,
    #[doc(hidden)]
    __padding_3288: [u8; 384],
    pub auto_attach_parent: TWeakObjectPtr<USceneComponent>,
    pub auto_attach_socket_name: FName,
    __padding_end: [u8; 244],
}
impl UAudioComponent {}
#[repr(C, align(16))]
pub struct UBillboardComponent {
    #[doc(hidden)]
    __padding_1504: [u8; 1504],
    pub sprite: UPtr<UTexture2D>,
    pub flags_1512: u8,
    pub screen_size: f32,
    pub u: f32,
    pub ul: f32,
    pub v: f32,
    pub vl: f32,
    pub opacity_mask_ref_val: f32,
    #[doc(hidden)]
    __padding_1600: [u8; 60],
    pub b_use_in_editor_scaling: bool,
    __padding_end: [u8; 15],
}
impl UBillboardComponent {}
#[repr(C, align(16))]
pub struct UBoundsCopyComponent {
    __padding_end: [u8; 416],
}
impl UBoundsCopyComponent {}
#[repr(C, align(16))]
pub struct UShapeComponent {
    #[doc(hidden)]
    __padding_1520: [u8; 1520],
    pub shape_color: crate::bindings::core_u_object::FColor,
    #[doc(hidden)]
    __padding_1536: [u8; 12],
    pub line_thickness: f32,
    __padding_end: [u8; 12],
}
impl UShapeComponent {}
#[repr(C, align(16))]
pub struct UBoxComponent {
    #[doc(hidden)]
    __padding_1544: [u8; 1544],
    pub box_extent: crate::bindings::core_u_object::FVector,
    __padding_end: [u8; 64],
}
impl UBoxComponent {}
#[repr(C, align(16))]
pub struct UBrushComponent {
    __padding_end: [u8; 1552],
}
impl UBrushComponent {}
#[repr(C, align(16))]
pub struct UCapsuleComponent {
    #[doc(hidden)]
    __padding_1544: [u8; 1544],
    pub capsule_half_height: f32,
    pub capsule_radius: f32,
    __padding_end: [u8; 16],
}
impl UCapsuleComponent {}
#[repr(C, align(8))]
pub struct UMovementComponent {
    #[doc(hidden)]
    __padding_240: [u8; 240],
    pub updated_component: UPtr<USceneComponent>,
    pub updated_primitive: UPtr<UPrimitiveComponent>,
    #[doc(hidden)]
    __padding_264: [u8; 8],
    pub velocity: crate::bindings::core_u_object::FVector,
    pub plane_constraint_normal: crate::bindings::core_u_object::FVector,
    pub plane_constraint_origin: crate::bindings::core_u_object::FVector,
    pub flags_336: u8,
    __padding_end: [u8; 7],
}
impl UMovementComponent {}
#[repr(C, align(8))]
pub struct UNavMovementComponent {
    #[doc(hidden)]
    __padding_368: [u8; 368],
    pub nav_agent_props: FNavAgentProperties,
    __padding_end: [u8; 16],
}
impl UNavMovementComponent {}
#[repr(C, align(8))]
pub struct UPawnMovementComponent {
    __padding_end: [u8; 456],
}
impl UPawnMovementComponent {}
#[repr(C, align(16))]
pub struct UCharacterMovementComponent {
    #[doc(hidden)]
    __padding_480: [u8; 480],
    pub gravity_scale: f32,
    pub max_step_height: f32,
    pub jump_z_velocity: f32,
    pub jump_off_jump_z_factor: f32,
    #[doc(hidden)]
    __padding_536: [u8; 40],
    pub gravity_direction: crate::bindings::core_u_object::FVector,
    pub world_to_gravity_transform: crate::bindings::core_u_object::FQuat,
    pub gravity_to_world_transform: crate::bindings::core_u_object::FQuat,
    #[doc(hidden)]
    __padding_625: [u8; 1],
    pub movement_mode: EMovementMode,
    pub custom_movement_mode: u8,
    pub network_smoothing_mode: ENetworkSmoothingMode,
    pub ground_friction: f32,
    #[doc(hidden)]
    __padding_696: [u8; 64],
    pub max_walk_speed: f32,
    pub max_walk_speed_crouched: f32,
    pub max_swim_speed: f32,
    pub max_fly_speed: f32,
    pub max_custom_movement_speed: f32,
    pub max_acceleration: f32,
    pub min_analog_walk_speed: f32,
    pub braking_friction_factor: f32,
    pub braking_friction: f32,
    pub braking_sub_step_time: f32,
    pub braking_deceleration_walking: f32,
    pub braking_deceleration_falling: f32,
    pub braking_deceleration_swimming: f32,
    pub braking_deceleration_flying: f32,
    pub air_control: f32,
    pub air_control_boost_multiplier: f32,
    pub air_control_boost_velocity_threshold: f32,
    pub falling_lateral_friction: f32,
    pub crouched_half_height: f32,
    pub buoyancy: f32,
    pub perch_radius_threshold: f32,
    pub perch_additional_height: f32,
    pub rotation_rate: crate::bindings::core_u_object::FRotator,
    pub flags_808: u8,
    pub flags_809: u8,
    pub flags_810: u8,
    pub flags_811: u8,
    #[doc(hidden)]
    __padding_824: [u8; 12],
    pub max_out_of_water_step_height: f32,
    pub outof_water_z: f32,
    pub mass: f32,
    pub standing_downward_force_scale: f32,
    pub initial_push_force_factor: f32,
    pub push_force_factor: f32,
    pub push_force_point_z_offset_factor: f32,
    pub touch_force_factor: f32,
    pub min_touch_force: f32,
    pub max_touch_force: f32,
    pub repulsion_force: f32,
    #[doc(hidden)]
    __padding_1072: [u8; 204],
    pub max_simulation_time_step: f32,
    pub max_simulation_iterations: i32,
    pub max_jump_apex_attempts_per_simulation: i32,
    pub max_depenetration_with_geometry: f32,
    pub max_depenetration_with_geometry_as_proxy: f32,
    pub max_depenetration_with_pawn: f32,
    pub max_depenetration_with_pawn_as_proxy: f32,
    #[doc(hidden)]
    __padding_1148: [u8; 48],
    pub ledge_check_threshold: f32,
    pub jump_out_of_water_pitch: f32,
    pub current_floor: FFindFloorResult,
    pub default_land_movement_mode: EMovementMode,
    pub default_water_movement_mode: EMovementMode,
    #[doc(hidden)]
    __padding_1452: [u8; 10],
    pub flags_1452: u8,
    pub flags_1453: u8,
    pub flags_1454: u8,
    pub flags_1455: u8,
    pub former_base_velocity_decay_half_life: f32,
    pub flags_1460: u8,
    #[doc(hidden)]
    __padding_1492: [u8; 28],
    pub avoidance_consideration_radius: f32,
    #[doc(hidden)]
    __padding_1544: [u8; 48],
    pub avoidance_uid: i32,
    pub avoidance_group: FNavAvoidanceMask,
    pub groups_to_avoid: FNavAvoidanceMask,
    pub groups_to_ignore: FNavAvoidanceMask,
    pub avoidance_weight: f32,
    #[doc(hidden)]
    __padding_1936: [u8; 372],
    pub nav_mesh_projection_interval: f32,
    #[doc(hidden)]
    __padding_1944: [u8; 4],
    pub nav_mesh_projection_interp_speed: f32,
    pub nav_mesh_projection_height_scale_up: f32,
    pub nav_mesh_projection_height_scale_down: f32,
    pub nav_walking_floor_dist_tolerance: f32,
    pub b_based_movement_ignore_physics_base: bool,
    pub b_base_on_attachment_root: bool,
    pub b_stay_based_in_air: bool,
    pub stay_based_in_air_height: f32,
    __padding_end: [u8; 2160],
}
impl UCharacterMovementComponent {
    pub fn verify_layout() {
        log::warn!(
            "{} = {} vs {}", "gravity_scale",
            std::mem::offset_of!(UCharacterMovementComponent, gravity_scale), 480usize
        );
        log::warn!(
            "{} = {} vs {}", "max_step_height",
            std::mem::offset_of!(UCharacterMovementComponent, max_step_height), 484usize
        );
        log::warn!(
            "{} = {} vs {}", "jump_z_velocity",
            std::mem::offset_of!(UCharacterMovementComponent, jump_z_velocity), 488usize
        );
        log::warn!(
            "{} = {} vs {}", "jump_off_jump_z_factor",
            std::mem::offset_of!(UCharacterMovementComponent, jump_off_jump_z_factor),
            492usize
        );
        log::warn!(
            "{} = {} vs {}", "gravity_direction",
            std::mem::offset_of!(UCharacterMovementComponent, gravity_direction),
            536usize
        );
        log::warn!(
            "{} = {} vs {}", "world_to_gravity_transform",
            std::mem::offset_of!(UCharacterMovementComponent,
            world_to_gravity_transform), 560usize
        );
        log::warn!(
            "{} = {} vs {}", "gravity_to_world_transform",
            std::mem::offset_of!(UCharacterMovementComponent,
            gravity_to_world_transform), 592usize
        );
        log::warn!(
            "{} = {} vs {}", "movement_mode",
            std::mem::offset_of!(UCharacterMovementComponent, movement_mode), 625usize
        );
        log::warn!(
            "{} = {} vs {}", "custom_movement_mode",
            std::mem::offset_of!(UCharacterMovementComponent, custom_movement_mode),
            626usize
        );
        log::warn!(
            "{} = {} vs {}", "network_smoothing_mode",
            std::mem::offset_of!(UCharacterMovementComponent, network_smoothing_mode),
            627usize
        );
        log::warn!(
            "{} = {} vs {}", "ground_friction",
            std::mem::offset_of!(UCharacterMovementComponent, ground_friction), 628usize
        );
        log::warn!(
            "{} = {} vs {}", "max_walk_speed",
            std::mem::offset_of!(UCharacterMovementComponent, max_walk_speed), 696usize
        );
        log::warn!(
            "{} = {} vs {}", "max_walk_speed_crouched",
            std::mem::offset_of!(UCharacterMovementComponent, max_walk_speed_crouched),
            700usize
        );
        log::warn!(
            "{} = {} vs {}", "max_swim_speed",
            std::mem::offset_of!(UCharacterMovementComponent, max_swim_speed), 704usize
        );
        log::warn!(
            "{} = {} vs {}", "max_fly_speed",
            std::mem::offset_of!(UCharacterMovementComponent, max_fly_speed), 708usize
        );
        log::warn!(
            "{} = {} vs {}", "max_custom_movement_speed",
            std::mem::offset_of!(UCharacterMovementComponent, max_custom_movement_speed),
            712usize
        );
        log::warn!(
            "{} = {} vs {}", "max_acceleration",
            std::mem::offset_of!(UCharacterMovementComponent, max_acceleration), 716usize
        );
        log::warn!(
            "{} = {} vs {}", "min_analog_walk_speed",
            std::mem::offset_of!(UCharacterMovementComponent, min_analog_walk_speed),
            720usize
        );
        log::warn!(
            "{} = {} vs {}", "braking_friction_factor",
            std::mem::offset_of!(UCharacterMovementComponent, braking_friction_factor),
            724usize
        );
        log::warn!(
            "{} = {} vs {}", "braking_friction",
            std::mem::offset_of!(UCharacterMovementComponent, braking_friction), 728usize
        );
        log::warn!(
            "{} = {} vs {}", "braking_sub_step_time",
            std::mem::offset_of!(UCharacterMovementComponent, braking_sub_step_time),
            732usize
        );
        log::warn!(
            "{} = {} vs {}", "braking_deceleration_walking",
            std::mem::offset_of!(UCharacterMovementComponent,
            braking_deceleration_walking), 736usize
        );
        log::warn!(
            "{} = {} vs {}", "braking_deceleration_falling",
            std::mem::offset_of!(UCharacterMovementComponent,
            braking_deceleration_falling), 740usize
        );
        log::warn!(
            "{} = {} vs {}", "braking_deceleration_swimming",
            std::mem::offset_of!(UCharacterMovementComponent,
            braking_deceleration_swimming), 744usize
        );
        log::warn!(
            "{} = {} vs {}", "braking_deceleration_flying",
            std::mem::offset_of!(UCharacterMovementComponent,
            braking_deceleration_flying), 748usize
        );
        log::warn!(
            "{} = {} vs {}", "air_control",
            std::mem::offset_of!(UCharacterMovementComponent, air_control), 752usize
        );
        log::warn!(
            "{} = {} vs {}", "air_control_boost_multiplier",
            std::mem::offset_of!(UCharacterMovementComponent,
            air_control_boost_multiplier), 756usize
        );
        log::warn!(
            "{} = {} vs {}", "air_control_boost_velocity_threshold",
            std::mem::offset_of!(UCharacterMovementComponent,
            air_control_boost_velocity_threshold), 760usize
        );
        log::warn!(
            "{} = {} vs {}", "falling_lateral_friction",
            std::mem::offset_of!(UCharacterMovementComponent, falling_lateral_friction),
            764usize
        );
        log::warn!(
            "{} = {} vs {}", "crouched_half_height",
            std::mem::offset_of!(UCharacterMovementComponent, crouched_half_height),
            768usize
        );
        log::warn!(
            "{} = {} vs {}", "buoyancy",
            std::mem::offset_of!(UCharacterMovementComponent, buoyancy), 772usize
        );
        log::warn!(
            "{} = {} vs {}", "perch_radius_threshold",
            std::mem::offset_of!(UCharacterMovementComponent, perch_radius_threshold),
            776usize
        );
        log::warn!(
            "{} = {} vs {}", "perch_additional_height",
            std::mem::offset_of!(UCharacterMovementComponent, perch_additional_height),
            780usize
        );
        log::warn!(
            "{} = {} vs {}", "rotation_rate",
            std::mem::offset_of!(UCharacterMovementComponent, rotation_rate), 784usize
        );
        log::warn!(
            "{} = {} vs {}", "max_out_of_water_step_height",
            std::mem::offset_of!(UCharacterMovementComponent,
            max_out_of_water_step_height), 824usize
        );
        log::warn!(
            "{} = {} vs {}", "outof_water_z",
            std::mem::offset_of!(UCharacterMovementComponent, outof_water_z), 828usize
        );
        log::warn!(
            "{} = {} vs {}", "mass", std::mem::offset_of!(UCharacterMovementComponent,
            mass), 832usize
        );
        log::warn!(
            "{} = {} vs {}", "standing_downward_force_scale",
            std::mem::offset_of!(UCharacterMovementComponent,
            standing_downward_force_scale), 836usize
        );
        log::warn!(
            "{} = {} vs {}", "initial_push_force_factor",
            std::mem::offset_of!(UCharacterMovementComponent, initial_push_force_factor),
            840usize
        );
        log::warn!(
            "{} = {} vs {}", "push_force_factor",
            std::mem::offset_of!(UCharacterMovementComponent, push_force_factor),
            844usize
        );
        log::warn!(
            "{} = {} vs {}", "push_force_point_z_offset_factor",
            std::mem::offset_of!(UCharacterMovementComponent,
            push_force_point_z_offset_factor), 848usize
        );
        log::warn!(
            "{} = {} vs {}", "touch_force_factor",
            std::mem::offset_of!(UCharacterMovementComponent, touch_force_factor),
            852usize
        );
        log::warn!(
            "{} = {} vs {}", "min_touch_force",
            std::mem::offset_of!(UCharacterMovementComponent, min_touch_force), 856usize
        );
        log::warn!(
            "{} = {} vs {}", "max_touch_force",
            std::mem::offset_of!(UCharacterMovementComponent, max_touch_force), 860usize
        );
        log::warn!(
            "{} = {} vs {}", "repulsion_force",
            std::mem::offset_of!(UCharacterMovementComponent, repulsion_force), 864usize
        );
        log::warn!(
            "{} = {} vs {}", "max_simulation_time_step",
            std::mem::offset_of!(UCharacterMovementComponent, max_simulation_time_step),
            1072usize
        );
        log::warn!(
            "{} = {} vs {}", "max_simulation_iterations",
            std::mem::offset_of!(UCharacterMovementComponent, max_simulation_iterations),
            1076usize
        );
        log::warn!(
            "{} = {} vs {}", "max_jump_apex_attempts_per_simulation",
            std::mem::offset_of!(UCharacterMovementComponent,
            max_jump_apex_attempts_per_simulation), 1080usize
        );
        log::warn!(
            "{} = {} vs {}", "max_depenetration_with_geometry",
            std::mem::offset_of!(UCharacterMovementComponent,
            max_depenetration_with_geometry), 1084usize
        );
        log::warn!(
            "{} = {} vs {}", "max_depenetration_with_geometry_as_proxy",
            std::mem::offset_of!(UCharacterMovementComponent,
            max_depenetration_with_geometry_as_proxy), 1088usize
        );
        log::warn!(
            "{} = {} vs {}", "max_depenetration_with_pawn",
            std::mem::offset_of!(UCharacterMovementComponent,
            max_depenetration_with_pawn), 1092usize
        );
        log::warn!(
            "{} = {} vs {}", "max_depenetration_with_pawn_as_proxy",
            std::mem::offset_of!(UCharacterMovementComponent,
            max_depenetration_with_pawn_as_proxy), 1096usize
        );
        log::warn!(
            "{} = {} vs {}", "ledge_check_threshold",
            std::mem::offset_of!(UCharacterMovementComponent, ledge_check_threshold),
            1148usize
        );
        log::warn!(
            "{} = {} vs {}", "jump_out_of_water_pitch",
            std::mem::offset_of!(UCharacterMovementComponent, jump_out_of_water_pitch),
            1152usize
        );
        log::warn!(
            "{} = {} vs {}", "current_floor",
            std::mem::offset_of!(UCharacterMovementComponent, current_floor), 1160usize
        );
        log::warn!(
            "{} = {} vs {}", "default_land_movement_mode",
            std::mem::offset_of!(UCharacterMovementComponent,
            default_land_movement_mode), 1440usize
        );
        log::warn!(
            "{} = {} vs {}", "default_water_movement_mode",
            std::mem::offset_of!(UCharacterMovementComponent,
            default_water_movement_mode), 1441usize
        );
        log::warn!(
            "{} = {} vs {}", "former_base_velocity_decay_half_life",
            std::mem::offset_of!(UCharacterMovementComponent,
            former_base_velocity_decay_half_life), 1456usize
        );
        log::warn!(
            "{} = {} vs {}", "avoidance_consideration_radius",
            std::mem::offset_of!(UCharacterMovementComponent,
            avoidance_consideration_radius), 1492usize
        );
        log::warn!(
            "{} = {} vs {}", "avoidance_uid",
            std::mem::offset_of!(UCharacterMovementComponent, avoidance_uid), 1544usize
        );
        log::warn!(
            "{} = {} vs {}", "avoidance_group",
            std::mem::offset_of!(UCharacterMovementComponent, avoidance_group), 1548usize
        );
        log::warn!(
            "{} = {} vs {}", "groups_to_avoid",
            std::mem::offset_of!(UCharacterMovementComponent, groups_to_avoid), 1552usize
        );
        log::warn!(
            "{} = {} vs {}", "groups_to_ignore",
            std::mem::offset_of!(UCharacterMovementComponent, groups_to_ignore),
            1556usize
        );
        log::warn!(
            "{} = {} vs {}", "avoidance_weight",
            std::mem::offset_of!(UCharacterMovementComponent, avoidance_weight),
            1560usize
        );
        log::warn!(
            "{} = {} vs {}", "nav_mesh_projection_interval",
            std::mem::offset_of!(UCharacterMovementComponent,
            nav_mesh_projection_interval), 1936usize
        );
        log::warn!(
            "{} = {} vs {}", "nav_mesh_projection_interp_speed",
            std::mem::offset_of!(UCharacterMovementComponent,
            nav_mesh_projection_interp_speed), 1944usize
        );
        log::warn!(
            "{} = {} vs {}", "nav_mesh_projection_height_scale_up",
            std::mem::offset_of!(UCharacterMovementComponent,
            nav_mesh_projection_height_scale_up), 1948usize
        );
        log::warn!(
            "{} = {} vs {}", "nav_mesh_projection_height_scale_down",
            std::mem::offset_of!(UCharacterMovementComponent,
            nav_mesh_projection_height_scale_down), 1952usize
        );
        log::warn!(
            "{} = {} vs {}", "nav_walking_floor_dist_tolerance",
            std::mem::offset_of!(UCharacterMovementComponent,
            nav_walking_floor_dist_tolerance), 1956usize
        );
        log::warn!(
            "{} = {} vs {}", "b_based_movement_ignore_physics_base",
            std::mem::offset_of!(UCharacterMovementComponent,
            b_based_movement_ignore_physics_base), 1960usize
        );
        log::warn!(
            "{} = {} vs {}", "b_base_on_attachment_root",
            std::mem::offset_of!(UCharacterMovementComponent, b_base_on_attachment_root),
            1961usize
        );
        log::warn!(
            "{} = {} vs {}", "b_stay_based_in_air",
            std::mem::offset_of!(UCharacterMovementComponent, b_stay_based_in_air),
            1962usize
        );
        log::warn!(
            "{} = {} vs {}", "stay_based_in_air_height",
            std::mem::offset_of!(UCharacterMovementComponent, stay_based_in_air_height),
            1964usize
        );
    }
}
#[repr(C, align(16))]
pub struct UDecalComponent {
    #[doc(hidden)]
    __padding_656: [u8; 656],
    pub decal_material: UPtr<UMaterialInterface>,
    pub sort_order: i32,
    pub fade_screen_size: f32,
    pub fade_start_delay: f32,
    pub fade_duration: f32,
    pub fade_in_duration: f32,
    pub fade_in_start_delay: f32,
    pub flags_688: u8,
    pub decal_size: crate::bindings::core_u_object::FVector,
    pub decal_color: crate::bindings::core_u_object::FLinearColor,
    __padding_end: [u8; 32],
}
impl UDecalComponent {}
#[repr(C, align(16))]
pub struct ULightComponent {
    #[doc(hidden)]
    __padding_760: [u8; 760],
    pub temperature: f32,
    pub max_draw_distance: f32,
    pub max_distance_fade_range: f32,
    pub flags_772: u8,
    #[doc(hidden)]
    __padding_788: [u8; 12],
    pub specular_scale: f32,
    pub diffuse_scale: f32,
    pub shadow_resolution_scale: f32,
    pub shadow_bias: f32,
    pub shadow_slope_bias: f32,
    pub shadow_sharpen: f32,
    pub contact_shadow_length: f32,
    pub flags_816: u8,
    pub contact_shadow_casting_intensity: f32,
    pub contact_shadow_non_casting_intensity: f32,
    pub flags_828: u8,
    #[doc(hidden)]
    __padding_832: [u8; 3],
    pub mega_lights_shadow_method: EMegaLightsShadowMethod,
    #[doc(hidden)]
    __padding_836: [u8; 3],
    pub flags_836: u8,
    pub ray_end_bias: f32,
    pub lighting_channels: FLightingChannels,
    pub view_lighting_channels: FViewLightingChannels,
    pub light_function_material: UPtr<UMaterialInterface>,
    #[doc(hidden)]
    __padding_864: [u8; 8],
    pub light_function_scale: crate::bindings::core_u_object::FVector,
    pub ies_texture: UPtr<UTextureLightProfile>,
    pub flags_896: u8,
    pub ies_brightness_scale: f32,
    pub light_function_fade_distance: f32,
    pub disabled_brightness: f32,
    pub flags_912: u8,
    pub bloom_scale: f32,
    pub bloom_threshold: f32,
    pub bloom_max_brightness: f32,
    pub bloom_tint: crate::bindings::core_u_object::FColor,
    pub b_use_ray_traced_distance_field_shadows: bool,
    pub ray_start_offset_depth_scale: f32,
    __padding_end: [u8; 132],
}
impl ULightComponent {}
#[repr(C, align(16))]
pub struct UDirectionalLightComponent {
    #[doc(hidden)]
    __padding_1072: [u8; 1072],
    pub shadow_cascade_bias_distribution: f32,
    pub flags_1076: u8,
    pub occlusion_mask_darkness: f32,
    pub occlusion_depth_range: f32,
    pub light_shaft_override_direction: crate::bindings::core_u_object::FVector,
    #[doc(hidden)]
    __padding_1116: [u8; 4],
    pub dynamic_shadow_distance_movable_light: f32,
    pub dynamic_shadow_distance_stationary_light: f32,
    pub dynamic_shadow_cascades: i32,
    pub cascade_distribution_exponent: f32,
    pub cascade_transition_fraction: f32,
    pub shadow_distance_fadeout_fraction: f32,
    pub flags_1140: u8,
    pub far_shadow_cascade_count: i32,
    pub far_shadow_distance: f32,
    pub distance_field_shadow_distance: f32,
    pub forward_shading_priority: i32,
    pub light_source_angle: f32,
    pub light_source_soft_angle: f32,
    pub shadow_source_angle_factor: f32,
    pub trace_distance: f32,
    pub flags_1176: u8,
    pub atmosphere_sun_light_index: i32,
    pub atmosphere_sun_disk_color_scale: crate::bindings::core_u_object::FLinearColor,
    pub flags_1200: u8,
    pub cloud_shadow_strength: f32,
    pub cloud_shadow_on_atmosphere_strength: f32,
    pub cloud_shadow_on_surface_strength: f32,
    pub cloud_shadow_depth_bias: f32,
    pub cloud_shadow_extent: f32,
    pub cloud_shadow_map_resolution_scale: f32,
    pub cloud_shadow_ray_sample_count_scale: f32,
    pub cloud_scattered_luminance_scale: crate::bindings::core_u_object::FLinearColor,
    #[doc(hidden)]
    __padding_1264: [u8; 16],
    pub flags_1264: u8,
    pub modulated_shadow_color: crate::bindings::core_u_object::FColor,
    pub shadow_amount: f32,
    __padding_end: [u8; 4],
}
impl UDirectionalLightComponent {}
#[repr(C, align(16))]
pub struct UDrawFrustumComponent {
    #[doc(hidden)]
    __padding_1504: [u8; 1504],
    pub b_frustum_enabled: bool,
    pub frustum_color: crate::bindings::core_u_object::FColor,
    pub frustum_angle: f32,
    pub frustum_aspect_ratio: f32,
    pub frustum_start_dist: f32,
    pub frustum_end_dist: f32,
    pub texture: UPtr<UTexture>,
}
impl UDrawFrustumComponent {}
#[repr(C, align(16))]
pub struct USphereComponent {
    #[doc(hidden)]
    __padding_1544: [u8; 1544],
    pub sphere_radius: f32,
    __padding_end: [u8; 4],
}
impl USphereComponent {}
#[repr(C, align(16))]
pub struct UDrawSphereComponent {
    __padding_end: [u8; 1552],
}
impl UDrawSphereComponent {}
#[repr(C, align(16))]
pub struct UExponentialHeightFogComponent {
    #[doc(hidden)]
    __padding_656: [u8; 656],
    pub fog_density: f32,
    pub fog_height_falloff: f32,
    pub second_fog_data: FExponentialHeightFogData,
    #[doc(hidden)]
    __padding_692: [u8; 16],
    pub fog_inscattering_luminance: crate::bindings::core_u_object::FLinearColor,
    pub sky_atmosphere_ambient_contribution_color_scale: crate::bindings::core_u_object::FLinearColor,
    pub inscattering_color_cubemap: UPtr<UTextureCube>,
    pub inscattering_color_cubemap_angle: f32,
    pub inscattering_texture_tint: crate::bindings::core_u_object::FLinearColor,
    pub fully_directional_inscattering_color_distance: f32,
    pub non_directional_inscattering_color_distance: f32,
    pub directional_inscattering_exponent: f32,
    pub directional_inscattering_start_distance: f32,
    #[doc(hidden)]
    __padding_788: [u8; 16],
    pub directional_inscattering_luminance: crate::bindings::core_u_object::FLinearColor,
    pub fog_max_opacity: f32,
    pub start_distance: f32,
    pub end_distance: f32,
    pub fog_cutoff_distance: f32,
    pub b_enable_volumetric_fog: bool,
    pub volumetric_fog_scattering_distribution: f32,
    pub volumetric_fog_albedo: crate::bindings::core_u_object::FColor,
    pub volumetric_fog_emissive: crate::bindings::core_u_object::FLinearColor,
    pub volumetric_fog_extinction_scale: f32,
    pub volumetric_fog_distance: f32,
    pub volumetric_fog_start_distance: f32,
    pub volumetric_fog_near_fade_in_distance: f32,
    pub volumetric_fog_static_lighting_scattering_intensity: f32,
    pub b_override_light_colors_with_fog_inscattering_colors: bool,
    pub flags_869: u8,
    __padding_end: [u8; 10],
}
impl UExponentialHeightFogComponent {}
#[repr(C, align(16))]
pub struct UForceFeedbackComponent {
    #[doc(hidden)]
    __padding_656: [u8; 656],
    pub force_feedback_effect: UPtr<UForceFeedbackEffect>,
    pub flags_664: u8,
    pub intensity_multiplier: f32,
    pub attenuation_settings: UPtr<UForceFeedbackAttenuation>,
    pub attenuation_overrides: FForceFeedbackAttenuationSettings,
    __padding_end: [u8; 40],
}
impl UForceFeedbackComponent {}
#[repr(C, align(16))]
pub struct UHeterogeneousVolumeComponent {
    #[doc(hidden)]
    __padding_1576: [u8; 1576],
    pub volume_resolution: crate::bindings::core_u_object::FIntVector,
    pub frame_transform: crate::bindings::core_u_object::FTransform,
    pub frame: f32,
    __padding_end: [u8; 60],
}
impl UHeterogeneousVolumeComponent {}
#[repr(C, align(8))]
pub struct AHeterogeneousVolume {
    #[doc(hidden)]
    __padding_1144: [u8; 1144],
    pub heterogeneous_volume_component: UPtr<UHeterogeneousVolumeComponent>,
}
impl AHeterogeneousVolume {}
#[repr(C, align(8))]
pub struct UInputComponent {
    __padding_end: [u8; 376],
}
impl UInputComponent {}
#[repr(C, align(16))]
pub struct UInstancedSkinnedMeshComponent {
    #[doc(hidden)]
    __padding_2592: [u8; 2592],
    pub transform_provider: UPtr<UTransformProviderData>,
    #[doc(hidden)]
    __padding_2640: [u8; 40],
    pub animation_min_screen_size: f32,
    pub instance_min_draw_distance: i32,
    pub instance_start_cull_distance: i32,
    pub instance_end_cull_distance: i32,
    __padding_end: [u8; 368],
}
impl UInstancedSkinnedMeshComponent {}
#[repr(C, align(8))]
pub struct UInterpToMovementComponent {
    #[doc(hidden)]
    __padding_344: [u8; 344],
    pub duration: f32,
    pub flags_348: u8,
    #[doc(hidden)]
    __padding_352: [u8; 3],
    pub b_sweep: bool,
    pub teleport_type: ETeleportType,
    pub behaviour_type: EInterpToBehaviourType,
    pub b_check_if_still_in_world: bool,
    pub flags_356: u8,
    #[doc(hidden)]
    __padding_480: [u8; 120],
    pub max_simulation_time_step: f32,
    pub speed_multiplier: f32,
    pub max_simulation_iterations: i32,
    pub control_points: TArray<FInterpControlPoint>,
    __padding_end: [u8; 56],
}
impl UInterpToMovementComponent {}
#[repr(C, align(16))]
pub struct ULineBatchComponent {
    __padding_end: [u8; 1568],
}
impl ULineBatchComponent {}
#[repr(C, align(16))]
pub struct ULocalFogVolumeComponent {
    #[doc(hidden)]
    __padding_656: [u8; 656],
    pub radial_fog_extinction: f32,
    pub height_fog_extinction: f32,
    pub height_fog_falloff: f32,
    pub height_fog_offset: f32,
    pub fog_phase_g: f32,
    pub fog_albedo: crate::bindings::core_u_object::FLinearColor,
    pub fog_emissive: crate::bindings::core_u_object::FLinearColor,
    pub fog_sort_priority: i32,
    __padding_end: [u8; 8],
}
impl ULocalFogVolumeComponent {}
#[repr(C, align(16))]
pub struct ULocalLightComponent {
    #[doc(hidden)]
    __padding_1072: [u8; 1072],
    pub intensity_units: ELightUnits,
    pub inverse_exposure_blend: f32,
    #[doc(hidden)]
    __padding_1084: [u8; 4],
    pub attenuation_radius: f32,
    __padding_end: [u8; 16],
}
impl ULocalLightComponent {}
#[repr(C, align(8))]
pub struct ULODSyncComponent {
    #[doc(hidden)]
    __padding_240: [u8; 240],
    pub num_lo_ds: i32,
    pub forced_lod: i32,
    pub min_lod: i32,
    pub components_to_sync: TArray<FComponentSync>,
    pub custom_lod_mapping: TMap<FName, FLODMappingData>,
    __padding_end: [u8; 40],
}
impl ULODSyncComponent {}
#[repr(C, align(16))]
pub struct UMaterialBillboardComponent {
    #[doc(hidden)]
    __padding_1504: [u8; 1504],
    pub elements: TArray<FMaterialSpriteElement>,
}
impl UMaterialBillboardComponent {}
#[repr(C, align(16))]
pub struct UModelComponent {
    __padding_end: [u8; 1568],
}
impl UModelComponent {}
#[repr(C, align(8))]
pub struct UPawnNoiseEmitterComponent {
    #[doc(hidden)]
    __padding_272: [u8; 272],
    pub noise_lifetime: f32,
    __padding_end: [u8; 20],
}
impl UPawnNoiseEmitterComponent {}
#[repr(C, align(8))]
pub struct UPlatformEventsComponent {
    __padding_end: [u8; 288],
}
impl UPlatformEventsComponent {}
#[repr(C, align(16))]
pub struct UPointLightComponent {
    #[doc(hidden)]
    __padding_1104: [u8; 1104],
    pub flags_1104: u8,
    pub light_falloff_exponent: f32,
    pub source_radius: f32,
    pub soft_source_radius: f32,
    pub source_length: f32,
    __padding_end: [u8; 12],
}
impl UPointLightComponent {}
#[repr(C, align(16))]
pub struct UPoseableMeshComponent {
    __padding_end: [u8; 2960],
}
impl UPoseableMeshComponent {}
#[repr(C, align(16))]
pub struct UPostProcessComponent {
    #[doc(hidden)]
    __padding_672: [u8; 672],
    pub settings: FPostProcessSettings,
    pub priority: f32,
    pub blend_radius: f32,
    pub blend_weight: f32,
    pub flags_2636: u8,
    __padding_end: [u8; 19],
}
impl UPostProcessComponent {}
#[repr(C, align(16))]
pub struct UProjectileMovementComponent {
    #[doc(hidden)]
    __padding_344: [u8; 344],
    pub initial_speed: f32,
    pub max_speed: f32,
    pub flags_352: u8,
    pub flags_353: u8,
    #[doc(hidden)]
    __padding_360: [u8; 6],
    pub flags_360: u8,
    pub previous_hit_time: f32,
    pub previous_hit_normal: crate::bindings::core_u_object::FVector,
    pub projectile_gravity_scale: f32,
    #[doc(hidden)]
    __padding_400: [u8; 4],
    pub bounciness: f32,
    pub friction: f32,
    pub bounce_velocity_stop_simulating_threshold: f32,
    pub min_friction_fraction: f32,
    #[doc(hidden)]
    __padding_464: [u8; 48],
    pub homing_acceleration_magnitude: f32,
    pub homing_target_component: TWeakObjectPtr<USceneComponent>,
    pub max_simulation_time_step: f32,
    pub max_simulation_iterations: i32,
    pub bounce_additional_iterations: i32,
    pub interp_location_time: f32,
    pub interp_rotation_time: f32,
    pub interp_location_max_lag_distance: f32,
    pub interp_location_snap_to_target_distance: f32,
    pub throttle_interpolation_threshold_not_rendered_short_time: f32,
    pub throttle_interpolation_threshold_not_rendered_long_time: f32,
    pub throttle_interpolation_skip_frames_recent: i32,
    pub throttle_interpolation_skip_frames_not_recent: i32,
    __padding_end: [u8; 184],
}
impl UProjectileMovementComponent {}
#[repr(C, align(16))]
pub struct URectLightComponent {
    #[doc(hidden)]
    __padding_1104: [u8; 1104],
    pub source_width: f32,
    pub source_height: f32,
    pub barn_door_angle: f32,
    pub barn_door_length: f32,
    pub light_function_cone_angle: f32,
    pub source_texture: UPtr<UTexture>,
    pub source_texture_scale: crate::bindings::core_u_object::FVector2f,
    pub source_texture_offset: crate::bindings::core_u_object::FVector2f,
    __padding_end: [u8; 16],
}
impl URectLightComponent {}
#[repr(C, align(8))]
pub struct URotatingMovementComponent {
    #[doc(hidden)]
    __padding_344: [u8; 344],
    pub rotation_rate: crate::bindings::core_u_object::FRotator,
    pub pivot_translation: crate::bindings::core_u_object::FVector,
    pub flags_392: u8,
    __padding_end: [u8; 7],
}
impl URotatingMovementComponent {}
#[repr(C, align(16))]
pub struct URuntimeVirtualTextureComponent {
    #[doc(hidden)]
    __padding_712: [u8; 712],
    pub virtual_texture: UPtr<URuntimeVirtualTexture>,
    #[doc(hidden)]
    __padding_824: [u8; 104],
    pub streaming_texture: UPtr<UVirtualTextureBuilder>,
    #[doc(hidden)]
    __padding_837: [u8; 5],
    pub lossy_compression_amount: ETextureLossyCompressionAmount,
    pub b_use_streaming_mips_fixed_color: bool,
    pub streaming_mips_fixed_color: crate::bindings::core_u_object::FLinearColor,
    __padding_end: [u8; 72],
}
impl URuntimeVirtualTextureComponent {}
#[repr(C, align(8))]
pub struct ASkyAtmosphere {
    #[doc(hidden)]
    __padding_1144: [u8; 1144],
    pub sky_atmosphere_component: UPtr<USkyAtmosphereComponent>,
    __padding_end: [u8; 8],
}
impl ASkyAtmosphere {}
#[repr(C, align(16))]
pub struct USkyLightComponent {
    #[doc(hidden)]
    __padding_760: [u8; 760],
    pub b_real_time_capture: bool,
    pub source_type: ESkyLightSourceType,
    pub cubemap: UPtr<UTextureCube>,
    pub source_cubemap_angle: f32,
    pub cubemap_resolution: i32,
    pub sky_distance_threshold: f32,
    pub b_capture_emissive_only: bool,
    pub b_lower_hemisphere_is_black: bool,
    pub lower_hemisphere_color: crate::bindings::core_u_object::FLinearColor,
    pub occlusion_max_distance: f32,
    pub contrast: f32,
    pub occlusion_exponent: f32,
    pub min_occlusion: f32,
    pub occlusion_tint: crate::bindings::core_u_object::FColor,
    pub flags_828: u8,
    pub cloud_ambient_occlusion_strength: f32,
    pub cloud_ambient_occlusion_extent: f32,
    pub cloud_ambient_occlusion_map_resolution_scale: f32,
    pub cloud_ambient_occlusion_aperture_scale: f32,
    pub occlusion_combine_mode: EOcclusionCombineMode,
    __padding_end: [u8; 399],
}
impl USkyLightComponent {}
#[repr(C, align(8))]
pub struct USplineMetadata {
    __padding_end: [u8; 48],
}
impl USplineMetadata {}
#[repr(C, align(16))]
pub struct USplineComponent {
    #[doc(hidden)]
    __padding_1808: [u8; 1808],
    pub duration: f32,
    pub b_stationary_endpoints: bool,
    #[doc(hidden)]
    __padding_1815: [u8; 2],
    pub b_input_spline_points_to_construction_script: bool,
    pub b_draw_debug: bool,
    #[doc(hidden)]
    __padding_1944: [u8; 120],
    pub default_up_vector: crate::bindings::core_u_object::FVector,
    __padding_end: [u8; 192],
}
impl USplineComponent {}
#[repr(C, align(16))]
pub struct USplineMeshComponent {
    __padding_end: [u8; 2192],
}
impl USplineMeshComponent {}
#[repr(C, align(16))]
pub struct USpotLightComponent {
    #[doc(hidden)]
    __padding_1128: [u8; 1128],
    pub inner_cone_angle: f32,
    pub outer_cone_angle: f32,
}
impl USpotLightComponent {}
#[repr(C, align(8))]
pub struct UEditorFlagCollector {
    __padding_end: [u8; 48],
}
impl UEditorFlagCollector {}
#[repr(C, align(8))]
pub struct UStereoLayerShape {
    __padding_end: [u8; 48],
}
impl UStereoLayerShape {}
#[repr(C, align(8))]
pub struct UStereoLayerShapeQuad {
    __padding_end: [u8; 48],
}
impl UStereoLayerShapeQuad {}
#[repr(C, align(8))]
pub struct UStereoLayerShapeCylinder {
    #[doc(hidden)]
    __padding_48: [u8; 48],
    pub radius: f32,
    pub overlay_arc: f32,
    pub height: i32,
    __padding_end: [u8; 4],
}
impl UStereoLayerShapeCylinder {}
#[repr(C, align(8))]
pub struct UStereoLayerShapeCubemap {
    __padding_end: [u8; 48],
}
impl UStereoLayerShapeCubemap {}
#[repr(C, align(8))]
pub struct UStereoLayerShapeEquirect {
    #[doc(hidden)]
    __padding_48: [u8; 48],
    pub left_uv_rect: crate::bindings::core_u_object::FBox2D,
    pub right_uv_rect: crate::bindings::core_u_object::FBox2D,
    pub left_scale: crate::bindings::core_u_object::FVector2D,
    pub right_scale: crate::bindings::core_u_object::FVector2D,
    pub left_bias: crate::bindings::core_u_object::FVector2D,
    pub right_bias: crate::bindings::core_u_object::FVector2D,
    pub radius: f32,
    __padding_end: [u8; 4],
}
impl UStereoLayerShapeEquirect {}
#[repr(C, align(16))]
pub struct UStereoLayerComponent {
    #[doc(hidden)]
    __padding_656: [u8; 656],
    pub flags_656: u8,
    pub texture: UPtr<UTexture>,
    pub left_texture: UPtr<UTexture>,
    pub flags_680: u8,
    pub additional_flags: TArray<FName>,
    pub quad_size: crate::bindings::core_u_object::FVector2D,
    pub uv_rect: crate::bindings::core_u_object::FBox2D,
    pub stereo_layer_type: EStereoLayerType,
    pub shape: UPtr<UStereoLayerShape>,
    pub priority: i32,
    __padding_end: [u8; 132],
}
impl UStereoLayerComponent {}
#[repr(C, align(16))]
pub struct UTextRenderComponent {
    #[doc(hidden)]
    __padding_1504: [u8; 1504],
    pub text: FText,
    pub text_material: UPtr<UMaterialInterface>,
    pub font: UPtr<UFont>,
    pub horizontal_alignment: EHorizTextAligment,
    pub vertical_alignment: EVerticalTextAligment,
    pub text_render_color: crate::bindings::core_u_object::FColor,
    pub x_scale: f32,
    pub y_scale: f32,
    pub world_size: f32,
    #[doc(hidden)]
    __padding_1560: [u8; 4],
    pub horiz_spacing_adjust: f32,
    pub vert_spacing_adjust: f32,
    pub flags_1568: u8,
    __padding_end: [u8; 15],
}
impl UTextRenderComponent {}
#[repr(C, align(16))]
pub struct UVolumetricCloudComponent {
    #[doc(hidden)]
    __padding_656: [u8; 656],
    pub layer_bottom_altitude: f32,
    pub layer_height: f32,
    pub tracing_start_max_distance: f32,
    pub tracing_start_distance_from_camera: f32,
    pub tracing_max_distance_mode: EVolumetricCloudTracingMaxDistanceMode,
    pub tracing_max_distance: f32,
    pub planet_radius: f32,
    pub ground_albedo: crate::bindings::core_u_object::FColor,
    pub material: TSoftObjectPtr<UMaterialInterface>,
    pub flags_736: u8,
    pub sky_light_cloud_bottom_occlusion: f32,
    pub view_sample_count_scale: f32,
    pub reflection_view_sample_count_scale_value: f32,
    #[doc(hidden)]
    __padding_760: [u8; 8],
    pub shadow_view_sample_count_scale: f32,
    pub shadow_reflection_view_sample_count_scale_value: f32,
    #[doc(hidden)]
    __padding_776: [u8; 8],
    pub shadow_tracing_distance: f32,
    pub stop_tracing_transmittance_threshold: f32,
    pub aerial_pespective_rayleigh_scattering_start_distance: f32,
    pub aerial_pespective_rayleigh_scattering_fade_distance: f32,
    pub aerial_pespective_mie_scattering_start_distance: f32,
    pub aerial_pespective_mie_scattering_fade_distance: f32,
    pub flags_800: u8,
    __padding_end: [u8; 31],
}
impl UVolumetricCloudComponent {
    pub fn verify_layout() {
        log::warn!(
            "{} = {} vs {}", "layer_bottom_altitude",
            std::mem::offset_of!(UVolumetricCloudComponent, layer_bottom_altitude),
            656usize
        );
        log::warn!(
            "{} = {} vs {}", "layer_height",
            std::mem::offset_of!(UVolumetricCloudComponent, layer_height), 660usize
        );
        log::warn!(
            "{} = {} vs {}", "tracing_start_max_distance",
            std::mem::offset_of!(UVolumetricCloudComponent, tracing_start_max_distance),
            664usize
        );
        log::warn!(
            "{} = {} vs {}", "tracing_start_distance_from_camera",
            std::mem::offset_of!(UVolumetricCloudComponent,
            tracing_start_distance_from_camera), 668usize
        );
        log::warn!(
            "{} = {} vs {}", "tracing_max_distance_mode",
            std::mem::offset_of!(UVolumetricCloudComponent, tracing_max_distance_mode),
            672usize
        );
        log::warn!(
            "{} = {} vs {}", "tracing_max_distance",
            std::mem::offset_of!(UVolumetricCloudComponent, tracing_max_distance),
            676usize
        );
        log::warn!(
            "{} = {} vs {}", "planet_radius",
            std::mem::offset_of!(UVolumetricCloudComponent, planet_radius), 680usize
        );
        log::warn!(
            "{} = {} vs {}", "ground_albedo",
            std::mem::offset_of!(UVolumetricCloudComponent, ground_albedo), 684usize
        );
        log::warn!(
            "{} = {} vs {}", "material", std::mem::offset_of!(UVolumetricCloudComponent,
            material), 688usize
        );
        log::warn!(
            "{} = {} vs {}", "sky_light_cloud_bottom_occlusion",
            std::mem::offset_of!(UVolumetricCloudComponent,
            sky_light_cloud_bottom_occlusion), 740usize
        );
        log::warn!(
            "{} = {} vs {}", "view_sample_count_scale",
            std::mem::offset_of!(UVolumetricCloudComponent, view_sample_count_scale),
            744usize
        );
        log::warn!(
            "{} = {} vs {}", "reflection_view_sample_count_scale_value",
            std::mem::offset_of!(UVolumetricCloudComponent,
            reflection_view_sample_count_scale_value), 748usize
        );
        log::warn!(
            "{} = {} vs {}", "shadow_view_sample_count_scale",
            std::mem::offset_of!(UVolumetricCloudComponent,
            shadow_view_sample_count_scale), 760usize
        );
        log::warn!(
            "{} = {} vs {}", "shadow_reflection_view_sample_count_scale_value",
            std::mem::offset_of!(UVolumetricCloudComponent,
            shadow_reflection_view_sample_count_scale_value), 764usize
        );
        log::warn!(
            "{} = {} vs {}", "shadow_tracing_distance",
            std::mem::offset_of!(UVolumetricCloudComponent, shadow_tracing_distance),
            776usize
        );
        log::warn!(
            "{} = {} vs {}", "stop_tracing_transmittance_threshold",
            std::mem::offset_of!(UVolumetricCloudComponent,
            stop_tracing_transmittance_threshold), 780usize
        );
        log::warn!(
            "{} = {} vs {}", "aerial_pespective_rayleigh_scattering_start_distance",
            std::mem::offset_of!(UVolumetricCloudComponent,
            aerial_pespective_rayleigh_scattering_start_distance), 784usize
        );
        log::warn!(
            "{} = {} vs {}", "aerial_pespective_rayleigh_scattering_fade_distance",
            std::mem::offset_of!(UVolumetricCloudComponent,
            aerial_pespective_rayleigh_scattering_fade_distance), 788usize
        );
        log::warn!(
            "{} = {} vs {}", "aerial_pespective_mie_scattering_start_distance",
            std::mem::offset_of!(UVolumetricCloudComponent,
            aerial_pespective_mie_scattering_start_distance), 792usize
        );
        log::warn!(
            "{} = {} vs {}", "aerial_pespective_mie_scattering_fade_distance",
            std::mem::offset_of!(UVolumetricCloudComponent,
            aerial_pespective_mie_scattering_fade_distance), 796usize
        );
    }
}
#[repr(C, align(8))]
pub struct AVolumetricCloud {
    #[doc(hidden)]
    __padding_1144: [u8; 1144],
    pub volumetric_cloud_component: UPtr<UVolumetricCloudComponent>,
}
impl AVolumetricCloud {}
#[repr(C, align(8))]
pub struct UWorldPartitionStreamingSourceComponent {
    #[doc(hidden)]
    __padding_252: [u8; 252],
    pub target_behavior: EStreamingSourceTargetBehavior,
    pub target_grids: TArray<FName>,
    #[doc(hidden)]
    __padding_312: [u8; 40],
    pub shapes: TArray<FStreamingSourceShape>,
    pub priority: EStreamingSourcePriority,
    pub b_streaming_source_enabled: bool,
    __padding_end: [u8; 6],
}
impl UWorldPartitionStreamingSourceComponent {}
#[repr(C, align(8))]
pub struct UCurveTable {
    __padding_end: [u8; 200],
}
impl UCurveTable {}
#[repr(C, align(8))]
pub struct UCompositeCurveTable {
    #[doc(hidden)]
    __padding_200: [u8; 200],
    pub parent_tables: TArray<UPtr<UCurveTable>>,
    __padding_end: [u8; 24],
}
impl UCompositeCurveTable {}
#[repr(C, align(8))]
pub struct UCompositeDataTable {
    #[doc(hidden)]
    __padding_352: [u8; 352],
    pub parent_tables: TArray<UPtr<UDataTable>>,
    __padding_end: [u8; 104],
}
impl UCompositeDataTable {}
#[repr(C, align(8))]
pub struct UStreamingSettings {
    __padding_end: [u8; 160],
}
impl UStreamingSettings {}
#[repr(C, align(8))]
pub struct UGarbageCollectionSettings {
    __padding_end: [u8; 136],
}
impl UGarbageCollectionSettings {}
#[repr(C, align(8))]
pub struct ACullDistanceVolume {
    #[doc(hidden)]
    __padding_1208: [u8; 1208],
    pub cull_distances: TArray<FCullDistanceSizePair>,
    pub flags_1224: u8,
    __padding_end: [u8; 7],
}
impl ACullDistanceVolume {}
#[repr(C, align(8))]
pub struct UCurveBase {
    __padding_end: [u8; 120],
}
impl UCurveBase {}
#[repr(C, align(8))]
pub struct UDEPRECATED_CurveEdPresetCurve {
    __padding_end: [u8; 48],
}
impl UDEPRECATED_CurveEdPresetCurve {}
#[repr(C, align(8))]
pub struct UCurveFloat {
    __padding_end: [u8; 256],
}
impl UCurveFloat {}
#[repr(C, align(8))]
pub struct UCurveLinearColor {
    __padding_end: [u8; 664],
}
impl UCurveLinearColor {}
#[repr(C, align(16))]
pub struct UCurveLinearColorAtlas {
    __padding_end: [u8; 1424],
}
impl UCurveLinearColorAtlas {}
#[repr(C, align(8))]
pub struct UCurveVector {
    __padding_end: [u8; 504],
}
impl UCurveVector {}
#[repr(C, align(8))]
pub struct UDamageType {
    #[doc(hidden)]
    __padding_48: [u8; 48],
    pub flags_48: u8,
    pub damage_impulse: f32,
    pub destructible_impulse: f32,
    pub destructible_damage_spread_scale: f32,
    pub damage_falloff: f32,
    __padding_end: [u8; 4],
}
impl UDamageType {}
#[repr(C, align(8))]
pub struct UPrimaryDataAsset {
    __padding_end: [u8; 72],
}
impl UPrimaryDataAsset {}
#[repr(C, align(8))]
pub struct UDataDrivenCVarEngineSubsystem {
    __padding_end: [u8; 80],
}
impl UDataDrivenCVarEngineSubsystem {}
#[repr(C, align(8))]
pub struct UDataDrivenConsoleVariableSettings {
    __padding_end: [u8; 168],
}
impl UDataDrivenConsoleVariableSettings {}
#[repr(C, align(8))]
pub struct UDataTableFunctionLibrary {
    __padding_end: [u8; 48],
}
impl UDataTableFunctionLibrary {}
#[repr(C, align(8))]
pub struct ADebugCameraController {
    #[doc(hidden)]
    __padding_2680: [u8; 2680],
    pub speed_scale: f32,
    pub initial_max_speed: f32,
    pub initial_accel: f32,
    pub initial_decel: f32,
    __padding_end: [u8; 88],
}
impl ADebugCameraController {}
#[repr(C, align(8))]
pub struct UDebugCameraControllerSettings {
    __padding_end: [u8; 120],
}
impl UDebugCameraControllerSettings {}
#[repr(C, align(8))]
pub struct AHUD {
    #[doc(hidden)]
    __padding_1136: [u8; 1136],
    pub player_owner: UPtr<APlayerController>,
    pub flags_1144: u8,
    #[doc(hidden)]
    __padding_1152: [u8; 7],
    pub flags_1152: u8,
    __padding_end: [u8; 223],
}
impl AHUD {}
#[repr(C, align(8))]
pub struct ADebugCameraHUD {
    __padding_end: [u8; 1376],
}
impl ADebugCameraHUD {}
#[repr(C, align(8))]
pub struct UDebugGarbageCollectionGraph {
    __padding_end: [u8; 80],
}
impl UDebugGarbageCollectionGraph {}
#[repr(C, align(8))]
pub struct UDebugDrawService {
    __padding_end: [u8; 48],
}
impl UDebugDrawService {}
#[repr(C, align(8))]
pub struct UReporterBase {
    __padding_end: [u8; 56],
}
impl UReporterBase {}
#[repr(C, align(8))]
pub struct UReporterGraph {
    __padding_end: [u8; 208],
}
impl UReporterGraph {}
#[repr(C, align(8))]
pub struct ADecalActor {
    #[doc(hidden)]
    __padding_1136: [u8; 1136],
    pub decal: UPtr<UDecalComponent>,
    __padding_end: [u8; 24],
}
impl ADecalActor {}
#[repr(C, align(8))]
pub struct ADefaultPawn {
    #[doc(hidden)]
    __padding_1264: [u8; 1264],
    pub base_turn_rate: f32,
    pub base_look_up_rate: f32,
    pub movement_component: UPtr<UPawnMovementComponent>,
    pub collision_component: UPtr<USphereComponent>,
    pub mesh_component: UPtr<UStaticMeshComponent>,
    pub flags_1296: u8,
    __padding_end: [u8; 7],
}
impl ADefaultPawn {}
#[repr(C, align(8))]
pub struct APhysicsVolume {
    #[doc(hidden)]
    __padding_1208: [u8; 1208],
    pub terminal_velocity: f32,
    pub priority: i32,
    pub fluid_friction: f32,
    pub flags_1220: u8,
    __padding_end: [u8; 3],
}
impl APhysicsVolume {}
#[repr(C, align(8))]
pub struct ADefaultPhysicsVolume {
    __padding_end: [u8; 1224],
}
impl ADefaultPhysicsVolume {}
pub struct IDeformableInterface {}
#[repr(C, align(8))]
pub struct UDeformableInterface {
    __padding_end: [u8; 48],
}
impl UDeformableInterface {}
#[repr(C, align(8))]
pub struct UDeletedObjectPlaceholder {
    __padding_end: [u8; 80],
}
impl UDeletedObjectPlaceholder {}
#[repr(C, align(8))]
pub struct UNetDriver {
    __padding_end: [u8; 2664],
}
impl UNetDriver {}
#[repr(C, align(16))]
pub struct UDemoNetDriver {
    __padding_end: [u8; 5936],
}
impl UDemoNetDriver {}
pub struct IDestructibleInterface {}
#[repr(C, align(8))]
pub struct UDestructibleInterface {
    __padding_end: [u8; 48],
}
impl UDestructibleInterface {}
#[repr(C, align(8))]
pub struct UTextureLODSettings {
    __padding_end: [u8; 72],
}
impl UTextureLODSettings {}
#[repr(C, align(8))]
pub struct UDeviceProfile {
    __padding_end: [u8; 520],
}
impl UDeviceProfile {}
#[repr(C, align(8))]
pub struct UDeviceProfileManager {
    __padding_end: [u8; 264],
}
impl UDeviceProfileManager {}
#[repr(C, align(8))]
pub struct UDialogueVoice {
    __padding_end: [u8; 72],
}
impl UDialogueVoice {}
#[repr(C, align(8))]
pub struct UDialogueWave {
    #[doc(hidden)]
    __padding_48: [u8; 48],
    pub flags_48: u8,
    pub spoken_text: FString,
    pub subtitle_override: FString,
    __padding_end: [u8; 48],
}
impl UDialogueWave {}
#[repr(C, align(8))]
pub struct ADocumentationActor {
    #[doc(hidden)]
    __padding_1136: [u8; 1136],
    pub document_link: FString,
    pub billboard: UPtr<UMaterialBillboardComponent>,
    __padding_end: [u8; 8],
}
impl ADocumentationActor {}
#[repr(C, align(8))]
pub struct UDPICustomScalingRule {
    __padding_end: [u8; 48],
}
impl UDPICustomScalingRule {}
#[repr(C, align(8))]
pub struct UGraphNodeContextMenuContext {
    __padding_end: [u8; 88],
}
impl UGraphNodeContextMenuContext {}
#[repr(C, align(8))]
pub struct UEdGraphNode_Documentation {
    __padding_end: [u8; 224],
}
impl UEdGraphNode_Documentation {}
#[repr(C, align(8))]
pub struct UEdGraphPin_Deprecated {
    __padding_end: [u8; 344],
}
impl UEdGraphPin_Deprecated {}
#[repr(C, align(8))]
pub struct UActorElementCounterInterface {
    __padding_end: [u8; 56],
}
impl UActorElementCounterInterface {}
#[repr(C, align(8))]
pub struct UActorElementHierarchyInterface {
    __padding_end: [u8; 56],
}
impl UActorElementHierarchyInterface {}
#[repr(C, align(8))]
pub struct UActorElementObjectInterface {
    __padding_end: [u8; 56],
}
impl UActorElementObjectInterface {}
#[repr(C, align(8))]
pub struct UActorElementTedsTypedElementBridgeInterface {
    __padding_end: [u8; 56],
}
impl UActorElementTedsTypedElementBridgeInterface {}
#[repr(C, align(8))]
pub struct UComponentElementCounterInterface {
    __padding_end: [u8; 56],
}
impl UComponentElementCounterInterface {}
#[repr(C, align(8))]
pub struct UComponentElementHierarchyInterface {
    __padding_end: [u8; 56],
}
impl UComponentElementHierarchyInterface {}
#[repr(C, align(8))]
pub struct UComponentElementObjectInterface {
    __padding_end: [u8; 56],
}
impl UComponentElementObjectInterface {}
#[repr(C, align(8))]
pub struct UEngineElementsLibrary {
    __padding_end: [u8; 48],
}
impl UEngineElementsLibrary {}
#[repr(C, align(8))]
pub struct UTypedElementCommonActions {
    __padding_end: [u8; 2096],
}
impl UTypedElementCommonActions {}
pub struct ITypedElementWorldInterface {}
#[repr(C, align(8))]
pub struct UTypedElementWorldInterface {
    __padding_end: [u8; 48],
}
impl UTypedElementWorldInterface {}
#[repr(C, align(8))]
pub struct UObjectElementAssetDataInterface {
    __padding_end: [u8; 56],
}
impl UObjectElementAssetDataInterface {}
#[repr(C, align(8))]
pub struct UObjectElementCounterInterface {
    __padding_end: [u8; 56],
}
impl UObjectElementCounterInterface {}
#[repr(C, align(8))]
pub struct UObjectElementObjectInterface {
    __padding_end: [u8; 56],
}
impl UObjectElementObjectInterface {}
#[repr(C, align(8))]
pub struct USMInstanceElementAssetDataInterface {
    __padding_end: [u8; 56],
}
impl USMInstanceElementAssetDataInterface {}
#[repr(C, align(8))]
pub struct USMInstanceElementHierarchyInterface {
    __padding_end: [u8; 56],
}
impl USMInstanceElementHierarchyInterface {}
#[repr(C, align(8))]
pub struct USMInstanceElementIdMapTransactor {
    __padding_end: [u8; 56],
}
impl USMInstanceElementIdMapTransactor {}
#[repr(C, align(8))]
pub struct USMInstanceElementPrimitiveCustomDataInterface {
    __padding_end: [u8; 56],
}
impl USMInstanceElementPrimitiveCustomDataInterface {}
#[repr(C, align(8))]
pub struct ULocalMessage {
    __padding_end: [u8; 48],
}
impl ULocalMessage {}
#[repr(C, align(8))]
pub struct UEngineMessage {
    __padding_end: [u8; 176],
}
impl UEngineMessage {}
#[repr(C, align(8))]
pub struct UEngineTypes {
    __padding_end: [u8; 48],
}
impl UEngineTypes {}
#[repr(C, align(8))]
pub struct UAutoDestroySubsystem {
    __padding_end: [u8; 96],
}
impl UAutoDestroySubsystem {}
#[repr(C, align(8))]
pub struct UCancellableAsyncAction {
    __padding_end: [u8; 56],
}
impl UCancellableAsyncAction {}
pub struct ILODSyncInterface {}
#[repr(C, align(8))]
pub struct ULODSyncInterface {
    __padding_end: [u8; 48],
}
impl ULODSyncInterface {}
#[repr(C, align(8))]
pub struct UPoseWatchFolder {
    __padding_end: [u8; 80],
}
impl UPoseWatchFolder {}
#[repr(C, align(8))]
pub struct UPoseWatchElement {
    __padding_end: [u8; 96],
}
impl UPoseWatchElement {}
#[repr(C, align(8))]
pub struct UPoseWatchPoseElement {
    __padding_end: [u8; 136],
}
impl UPoseWatchPoseElement {}
#[repr(C, align(8))]
pub struct UPoseWatch {
    __padding_end: [u8; 160],
}
impl UPoseWatch {}
#[repr(C, align(8))]
pub struct AServerStatReplicator {
    __padding_end: [u8; 1336],
}
impl AServerStatReplicator {}
#[repr(C, align(8))]
pub struct USystemTimeTimecodeProvider {
    __padding_end: [u8; 72],
}
impl USystemTimeTimecodeProvider {}
#[repr(C, align(8))]
pub struct UViewportStatsSubsystem {
    __padding_end: [u8; 96],
}
impl UViewportStatsSubsystem {}
#[repr(C, align(8))]
pub struct UFieldNotificationLibrary {
    __padding_end: [u8; 48],
}
impl UFieldNotificationLibrary {}
#[repr(C, align(8))]
pub struct UFloatingPawnMovement {
    #[doc(hidden)]
    __padding_456: [u8; 456],
    pub max_speed: f32,
    pub acceleration: f32,
    pub deceleration: f32,
    pub turning_boost: f32,
    __padding_end: [u8; 8],
}
impl UFloatingPawnMovement {}
#[repr(C, align(8))]
pub struct UFont {
    __padding_end: [u8; 488],
}
impl UFont {}
#[repr(C, align(8))]
pub struct UFontFace {
    #[doc(hidden)]
    __padding_56: [u8; 56],
    pub source_filename: FString,
    pub hinting: crate::bindings::slate_core::EFontHinting,
    pub loading_policy: crate::bindings::slate_core::EFontLoadingPolicy,
    pub layout_method: crate::bindings::slate_core::EFontLayoutMethod,
    #[doc(hidden)]
    __padding_92: [u8; 16],
    pub strike_brush_height_percentage: i32,
    __padding_end: [u8; 88],
}
impl UFontFace {}
#[repr(C, align(8))]
pub struct UGameEngine {
    __padding_end: [u8; 6920],
}
impl UGameEngine {}
#[repr(C, align(8))]
pub struct UAsyncActionHandleSaveGame {
    __padding_end: [u8; 120],
}
impl UAsyncActionHandleSaveGame {}
#[repr(C, align(8))]
pub struct UForceFeedbackEffect {
    #[doc(hidden)]
    __padding_160: [u8; 160],
    pub duration: f32,
    __padding_end: [u8; 4],
}
impl UForceFeedbackEffect {}
#[repr(C, align(8))]
pub struct UInputDeviceProperty {
    #[doc(hidden)]
    __padding_48: [u8; 48],
    pub property_duration: f32,
    __padding_end: [u8; 4],
}
impl UInputDeviceProperty {}
#[repr(C, align(8))]
pub struct UColorInputDeviceProperty {
    #[doc(hidden)]
    __padding_56: [u8; 56],
    pub color_data: FDeviceColorData,
    pub device_override_data: TMap<FName, FDeviceColorData>,
    __padding_end: [u8; 24],
}
impl UColorInputDeviceProperty {}
#[repr(C, align(8))]
pub struct UColorInputDeviceCurveProperty {
    #[doc(hidden)]
    __padding_56: [u8; 56],
    pub color_data: FDeviceColorCurveData,
    pub device_override_data: TMap<FName, FDeviceColorCurveData>,
    __padding_end: [u8; 24],
}
impl UColorInputDeviceCurveProperty {}
#[repr(C, align(8))]
pub struct UInputDeviceTriggerEffect {
    #[doc(hidden)]
    __padding_56: [u8; 56],
    pub base_trigger_data: FDeviceTriggerBaseData,
    __padding_end: [u8; 22],
}
impl UInputDeviceTriggerEffect {}
#[repr(C, align(8))]
pub struct UInputDeviceTriggerFeedbackProperty {
    #[doc(hidden)]
    __padding_80: [u8; 80],
    pub trigger_data: FDeviceTriggerFeedbackData,
    pub device_override_data: TMap<FName, FDeviceTriggerFeedbackData>,
    __padding_end: [u8; 24],
}
impl UInputDeviceTriggerFeedbackProperty {}
#[repr(C, align(8))]
pub struct UInputDeviceTriggerResistanceProperty {
    #[doc(hidden)]
    __padding_80: [u8; 80],
    pub trigger_data: FDeviceTriggerTriggerResistanceData,
    pub device_override_data: TMap<FName, FDeviceTriggerTriggerResistanceData>,
    __padding_end: [u8; 32],
}
impl UInputDeviceTriggerResistanceProperty {}
#[repr(C, align(8))]
pub struct UInputDeviceTriggerVibrationProperty {
    #[doc(hidden)]
    __padding_80: [u8; 80],
    pub trigger_data: FDeviceTriggerTriggerVibrationData,
    pub device_override_data: TMap<FName, FDeviceTriggerTriggerVibrationData>,
    __padding_end: [u8; 32],
}
impl UInputDeviceTriggerVibrationProperty {}
#[repr(C, align(8))]
pub struct UInputDeviceAudioBasedVibrationProperty {
    #[doc(hidden)]
    __padding_56: [u8; 56],
    pub data: FAudioBasedVibrationData,
    pub device_override_data: TMap<FName, FAudioBasedVibrationData>,
}
impl UInputDeviceAudioBasedVibrationProperty {}
#[repr(C, align(8))]
pub struct UInputDeviceSubsystem {
    __padding_end: [u8; 448],
}
impl UInputDeviceSubsystem {}
#[repr(C, align(8))]
pub struct UPlayerStateCountLimiterConfig {
    __padding_end: [u8; 72],
}
impl UPlayerStateCountLimiterConfig {}
#[repr(C, align(8))]
pub struct USaveGame {
    __padding_end: [u8; 48],
}
impl USaveGame {}
#[repr(C, align(8))]
pub struct ULocalPlayerSaveGame {
    __padding_end: [u8; 96],
}
impl ULocalPlayerSaveGame {}
#[repr(C, align(16))]
pub struct USpringArmComponent {
    #[doc(hidden)]
    __padding_656: [u8; 656],
    pub target_arm_length: f32,
    pub socket_offset: crate::bindings::core_u_object::FVector,
    pub target_offset: crate::bindings::core_u_object::FVector,
    pub probe_size: f32,
    pub probe_channel: ECollisionChannel,
    #[doc(hidden)]
    __padding_720: [u8; 3],
    pub flags_720: u8,
    pub flags_721: u8,
    pub camera_lag_speed: f32,
    pub camera_rotation_lag_speed: f32,
    pub camera_lag_max_time_step: f32,
    pub camera_lag_max_distance: f32,
    pub flags_740: u8,
    __padding_end: [u8; 171],
}
impl USpringArmComponent {}
#[repr(C, align(8))]
pub struct UTouchInterface {
    __padding_end: [u8; 96],
}
impl UTouchInterface {}
#[repr(C, align(8))]
pub struct AGameModeBase {
    #[doc(hidden)]
    __padding_1144: [u8; 1144],
    pub options_string: FString,
    pub game_session_class: TSubclassOf<AGameSession>,
    pub game_state_class: TSubclassOf<AGameStateBase>,
    pub player_controller_class: TSubclassOf<APlayerController>,
    pub player_state_class: TSubclassOf<APlayerState>,
    pub hud_class: TSubclassOf<AHUD>,
    pub default_pawn_class: TSubclassOf<APawn>,
    pub spectator_class: TSubclassOf<ASpectatorPawn>,
    pub replay_spectator_player_controller_class: TSubclassOf<APlayerController>,
    pub server_stat_replicator_class: TSubclassOf<AServerStatReplicator>,
    #[doc(hidden)]
    __padding_1272: [u8; 40],
    pub flags_1272: u8,
    #[doc(hidden)]
    __padding_1276: [u8; 3],
    pub game_net_driver_replication_system: crate::bindings::net_core::EReplicationSystem,
    __padding_end: [u8; 19],
}
impl AGameModeBase {}
#[repr(C, align(8))]
pub struct AGameMode {
    #[doc(hidden)]
    __padding_1308: [u8; 1308],
    pub flags_1308: u8,
    pub num_spectators: i32,
    pub num_players: i32,
    pub num_bots: i32,
    pub min_respawn_delay: f32,
    pub num_travelling_players: i32,
    __padding_end: [u8; 44],
}
impl AGameMode {}
#[repr(C, align(8))]
pub struct AGameNetworkManager {
    __padding_end: [u8; 1312],
}
impl AGameNetworkManager {}
#[repr(C, align(8))]
pub struct UGameplayStatics {
    __padding_end: [u8; 48],
}
impl UGameplayStatics {}
#[repr(C, align(8))]
pub struct AGameSession {
    __padding_end: [u8; 1192],
}
impl AGameSession {}
#[repr(C, align(8))]
pub struct AGameStateBase {
    #[doc(hidden)]
    __padding_1144: [u8; 1144],
    pub game_mode_class: TSubclassOf<AGameModeBase>,
    pub authority_game_mode: UPtr<AGameModeBase>,
    pub spectator_class: TSubclassOf<ASpectatorPawn>,
    pub player_array: TArray<UPtr<APlayerState>>,
    __padding_end: [u8; 48],
}
impl AGameStateBase {}
#[repr(C, align(8))]
pub struct AGameState {
    #[doc(hidden)]
    __padding_1232: [u8; 1232],
    pub match_state: FName,
    pub previous_match_state: FName,
    pub elapsed_time: i32,
    __padding_end: [u8; 12],
}
impl AGameState {}
#[repr(C, align(8))]
pub struct UGameUserSettings {
    __padding_end: [u8; 376],
}
impl UGameUserSettings {}
#[repr(C, align(8))]
pub struct UScriptViewportClient {
    __padding_end: [u8; 64],
}
impl UScriptViewportClient {}
#[repr(C, align(8))]
pub struct UGameViewportClient {
    __padding_end: [u8; 1128],
}
impl UGameViewportClient {}
#[repr(C, align(8))]
pub struct UGeneratedBlueprintBinding {
    __padding_end: [u8; 64],
}
impl UGeneratedBlueprintBinding {}
#[repr(C, align(8))]
pub struct UHLODProxy {
    __padding_end: [u8; 192],
}
impl UHLODProxy {}
#[repr(C, align(8))]
pub struct UHLODEngineSubsystem {
    __padding_end: [u8; 88],
}
impl UHLODEngineSubsystem {}
#[repr(C, align(16))]
pub struct UHLODProxyDesc {
    __padding_end: [u8; 448],
}
impl UHLODProxyDesc {}
#[repr(C, align(8))]
pub struct UHierarchicalLODSetup {
    __padding_end: [u8; 112],
}
impl UHierarchicalLODSetup {}
pub struct IImportantToggleSettingInterface {}
#[repr(C, align(8))]
pub struct UImportantToggleSettingInterface {
    __padding_end: [u8; 48],
}
impl UImportantToggleSettingInterface {}
#[repr(C, align(8))]
pub struct UInheritableComponentHandler {
    __padding_end: [u8; 80],
}
impl UInheritableComponentHandler {}
#[repr(C, align(8))]
pub struct UInputDelegateBinding {
    __padding_end: [u8; 48],
}
impl UInputDelegateBinding {}
#[repr(C, align(8))]
pub struct UInputActionDelegateBinding {
    __padding_end: [u8; 64],
}
impl UInputActionDelegateBinding {}
#[repr(C, align(8))]
pub struct UInputAxisDelegateBinding {
    __padding_end: [u8; 64],
}
impl UInputAxisDelegateBinding {}
#[repr(C, align(8))]
pub struct UInputAxisKeyDelegateBinding {
    __padding_end: [u8; 64],
}
impl UInputAxisKeyDelegateBinding {}
#[repr(C, align(8))]
pub struct UInputDeviceLibrary {
    __padding_end: [u8; 48],
}
impl UInputDeviceLibrary {}
#[repr(C, align(8))]
pub struct UInputKeyDelegateBinding {
    __padding_end: [u8; 64],
}
impl UInputKeyDelegateBinding {}
#[repr(C, align(8))]
pub struct UInputTouchDelegateBinding {
    __padding_end: [u8; 64],
}
impl UInputTouchDelegateBinding {}
#[repr(C, align(8))]
pub struct UInputVectorAxisDelegateBinding {
    __padding_end: [u8; 64],
}
impl UInputVectorAxisDelegateBinding {}
#[repr(C, align(8))]
pub struct AInstancedPlacementPartitionActor {
    __padding_end: [u8; 1392],
}
impl AInstancedPlacementPartitionActor {}
#[repr(C, align(8))]
pub struct UStringTable {
    __padding_end: [u8; 80],
}
impl UStringTable {}
#[repr(C, align(8))]
pub struct UInterpCurveEdSetup {
    __padding_end: [u8; 72],
}
impl UInterpCurveEdSetup {}
#[repr(C, align(8))]
pub struct UIntSerialization {
    __padding_end: [u8; 88],
}
impl UIntSerialization {}
#[repr(C, align(8))]
pub struct AKillZVolume {
    __padding_end: [u8; 1224],
}
impl AKillZVolume {}
#[repr(C, align(8))]
pub struct UKismetArrayLibrary {
    __padding_end: [u8; 48],
}
impl UKismetArrayLibrary {}
#[repr(C, align(8))]
pub struct UKismetGuidLibrary {
    __padding_end: [u8; 48],
}
impl UKismetGuidLibrary {}
#[repr(C, align(8))]
pub struct UKismetInputLibrary {
    __padding_end: [u8; 48],
}
impl UKismetInputLibrary {}
#[repr(C, align(8))]
pub struct UKismetInternationalizationLibrary {
    __padding_end: [u8; 48],
}
impl UKismetInternationalizationLibrary {}
#[repr(C, align(8))]
pub struct UKismetMaterialLibrary {
    __padding_end: [u8; 48],
}
impl UKismetMaterialLibrary {}
#[repr(C, align(8))]
pub struct UKismetMathLibrary {
    __padding_end: [u8; 48],
}
impl UKismetMathLibrary {}
#[repr(C, align(8))]
pub struct UKismetNodeHelperLibrary {
    __padding_end: [u8; 48],
}
impl UKismetNodeHelperLibrary {}
#[repr(C, align(8))]
pub struct UKismetRenderingLibrary {
    __padding_end: [u8; 48],
}
impl UKismetRenderingLibrary {}
#[repr(C, align(8))]
pub struct UKismetStringLibrary {
    __padding_end: [u8; 48],
}
impl UKismetStringLibrary {}
#[repr(C, align(8))]
pub struct UKismetStringTableLibrary {
    __padding_end: [u8; 48],
}
impl UKismetStringTableLibrary {}
#[repr(C, align(8))]
pub struct UKismetSystemLibrary {
    __padding_end: [u8; 48],
}
impl UKismetSystemLibrary {}
#[repr(C, align(8))]
pub struct UKismetTextLibrary {
    __padding_end: [u8; 48],
}
impl UKismetTextLibrary {}
#[repr(C, align(8))]
pub struct UBlueprintPathsLibrary {
    __padding_end: [u8; 48],
}
impl UBlueprintPathsLibrary {}
#[repr(C, align(8))]
pub struct UPlatformGameInstance {
    __padding_end: [u8; 840],
}
impl UPlatformGameInstance {}
#[repr(C, align(8))]
pub struct UBlueprintPlatformLibrary {
    __padding_end: [u8; 48],
}
impl UBlueprintPlatformLibrary {}
#[repr(C, align(8))]
pub struct UBlueprintTypeConversions {
    __padding_end: [u8; 48],
}
impl UBlueprintTypeConversions {}
#[repr(C, align(8))]
pub struct UImportanceSamplingLibrary {
    __padding_end: [u8; 48],
}
impl UImportanceSamplingLibrary {}
#[repr(C, align(8))]
pub struct ULayer {
    __padding_end: [u8; 80],
}
impl ULayer {}
#[repr(C, align(8))]
pub struct UActorContainer {
    __padding_end: [u8; 136],
}
impl UActorContainer {}
#[repr(C, align(8))]
pub struct ULevelActorContainer {
    __padding_end: [u8; 64],
}
impl ULevelActorContainer {}
#[repr(C, align(8))]
pub struct ALevelBounds {
    __padding_end: [u8; 1184],
}
impl ALevelBounds {}
#[repr(C, align(8))]
pub struct ALevelInstance {
    __padding_end: [u8; 1488],
}
impl ALevelInstance {}
#[repr(C, align(16))]
pub struct ULevelInstanceComponent {
    __padding_end: [u8; 1504],
}
impl ULevelInstanceComponent {}
#[repr(C, align(8))]
pub struct ALevelInstanceEditorInstanceActor {
    __padding_end: [u8; 1192],
}
impl ALevelInstanceEditorInstanceActor {}
#[repr(C, align(16))]
pub struct ULevelStreamingLevelInstanceEditor {
    __padding_end: [u8; 672],
}
impl ULevelStreamingLevelInstanceEditor {}
#[repr(C, align(8))]
pub struct ULevelInstanceEditorObject {
    __padding_end: [u8; 88],
}
impl ULevelInstanceEditorObject {}
#[repr(C, align(16))]
pub struct ALevelInstancePivot {
    __padding_end: [u8; 1344],
}
impl ALevelInstancePivot {}
#[repr(C, align(16))]
pub struct ULevelStreamingLevelInstanceEditorPropertyOverride {
    __padding_end: [u8; 704],
}
impl ULevelStreamingLevelInstanceEditorPropertyOverride {}
pub struct ILevelInstanceInterface {}
#[repr(C, align(8))]
pub struct ULevelInstanceInterface {
    __padding_end: [u8; 48],
}
impl ULevelInstanceInterface {}
#[repr(C, align(16))]
pub struct ULevelStreamingLevelInstance {
    __padding_end: [u8; 832],
}
impl ULevelStreamingLevelInstance {}
#[repr(C, align(8))]
pub struct UWorldPartitionPropertyOverride {
    __padding_end: [u8; 128],
}
impl UWorldPartitionPropertyOverride {}
#[repr(C, align(8))]
pub struct ULevelInstancePropertyOverrideAsset {
    __padding_end: [u8; 184],
}
impl ULevelInstancePropertyOverrideAsset {}
#[repr(C, align(8))]
pub struct UWorldPartitionPropertyOverridePolicy {
    __padding_end: [u8; 48],
}
impl UWorldPartitionPropertyOverridePolicy {}
#[repr(C, align(8))]
pub struct ULevelInstancePropertyOverridePolicy {
    __padding_end: [u8; 48],
}
impl ULevelInstancePropertyOverridePolicy {}
#[repr(C, align(8))]
pub struct ULevelInstanceSettings {
    __padding_end: [u8; 80],
}
impl ULevelInstanceSettings {}
#[repr(C, align(8))]
pub struct ULevelInstanceSubsystem {
    __padding_end: [u8; 696],
}
impl ULevelInstanceSubsystem {}
#[repr(C, align(8))]
pub struct ULevelInstancePropertyOverrideSamplePolicy {
    __padding_end: [u8; 48],
}
impl ULevelInstancePropertyOverrideSamplePolicy {}
#[repr(C, align(8))]
pub struct ALevelScriptActor {
    __padding_end: [u8; 1144],
}
impl ALevelScriptActor {}
#[repr(C, align(8))]
pub struct ULevelScriptBlueprint {
    __padding_end: [u8; 1448],
}
impl ULevelScriptBlueprint {}
#[repr(C, align(8))]
pub struct ALevelStreamingVolume {
    #[doc(hidden)]
    __padding_1208: [u8; 1208],
    pub streaming_level_names: TArray<FName>,
    pub flags_1224: u8,
    #[doc(hidden)]
    __padding_1228: [u8; 3],
    pub streaming_usage: EStreamingVolumeUsage,
    __padding_end: [u8; 3],
}
impl ALevelStreamingVolume {}
#[repr(C, align(8))]
pub struct ULightmappedSurfaceCollection {
    __padding_end: [u8; 72],
}
impl ULightmappedSurfaceCollection {}
#[repr(C, align(8))]
pub struct ALightmassCharacterIndirectDetailVolume {
    __padding_end: [u8; 1208],
}
impl ALightmassCharacterIndirectDetailVolume {}
#[repr(C, align(8))]
pub struct ALightmassImportanceVolume {
    __padding_end: [u8; 1208],
}
impl ALightmassImportanceVolume {}
#[repr(C, align(8))]
pub struct ULightmassPrimitiveSettingsObject {
    __padding_end: [u8; 72],
}
impl ULightmassPrimitiveSettingsObject {}
#[repr(C, align(8))]
pub struct ULightWeightInstanceBlueprintFunctionLibrary {
    __padding_end: [u8; 48],
}
impl ULightWeightInstanceBlueprintFunctionLibrary {}
#[repr(C, align(8))]
pub struct ALightWeightInstanceManager {
    __padding_end: [u8; 1336],
}
impl ALightWeightInstanceManager {}
#[repr(C, align(8))]
pub struct ALightWeightInstanceStaticMeshManager {
    #[doc(hidden)]
    __padding_1344: [u8; 1344],
    pub static_mesh: TSoftObjectPtr<UStaticMesh>,
    #[doc(hidden)]
    __padding_1400: [u8; 8],
    pub ism_component: UPtr<UInstancedStaticMeshComponent>,
    __padding_end: [u8; 48],
}
impl ALightWeightInstanceStaticMeshManager {}
#[repr(C, align(8))]
pub struct ULocalPlayer {
    __padding_end: [u8; 736],
}
impl ULocalPlayer {}
#[repr(C, align(8))]
pub struct ALocationVolume {
    __padding_end: [u8; 1232],
}
impl ALocationVolume {}
#[repr(C, align(8))]
pub struct ALODActor {
    __padding_end: [u8; 1520],
}
impl ALODActor {}
#[repr(C, align(16))]
pub struct UMaterialCacheVirtualTexture {
    __padding_end: [u8; 1296],
}
impl UMaterialCacheVirtualTexture {}
#[repr(C, align(8))]
pub struct UMaterialCacheVirtualTextureTag {
    __padding_end: [u8; 104],
}
impl UMaterialCacheVirtualTextureTag {}
#[repr(C, align(8))]
pub struct AMaterialInstanceActor {
    #[doc(hidden)]
    __padding_1136: [u8; 1136],
    pub target_actors: TArray<UPtr<AActor>>,
    __padding_end: [u8; 8],
}
impl AMaterialInstanceActor {}
#[repr(C, align(8))]
pub struct UMaterialInterfaceEditorOnlyData {
    __padding_end: [u8; 72],
}
impl UMaterialInterfaceEditorOnlyData {}
#[repr(C, align(8))]
pub struct UMaterialEditorOnlyData {
    __padding_end: [u8; 1968],
}
impl UMaterialEditorOnlyData {}
#[repr(C, align(8))]
pub struct UMaterialAggregate {
    __padding_end: [u8; 72],
}
impl UMaterialAggregate {}
pub struct IMaterialEnumerationProvider {}
#[repr(C, align(8))]
pub struct UMaterialEnumerationProvider {
    __padding_end: [u8; 48],
}
impl UMaterialEnumerationProvider {}
#[repr(C, align(8))]
pub struct UMaterialExpressionLayerStack {
    __padding_end: [u8; 808],
}
impl UMaterialExpressionLayerStack {}
#[repr(C, align(8))]
pub struct UMaterialExpressionNoise {
    __padding_end: [u8; 344],
}
impl UMaterialExpressionNoise {}
#[repr(C, align(8))]
pub struct UMaterialExpressionScalarBlueNoise {
    __padding_end: [u8; 216],
}
impl UMaterialExpressionScalarBlueNoise {}
#[repr(C, align(8))]
pub struct UMaterialExpressionSubstrateBSDF {
    __padding_end: [u8; 200],
}
impl UMaterialExpressionSubstrateBSDF {}
#[repr(C, align(8))]
pub struct UMaterialExpressionSubstrateShadingModels {
    #[doc(hidden)]
    __padding_1224: [u8; 1224],
    pub subsurface_profile: UPtr<USubsurfaceProfile>,
}
impl UMaterialExpressionSubstrateShadingModels {}
#[repr(C, align(8))]
pub struct UMaterialExpressionSubstrateSlabBSDF {
    #[doc(hidden)]
    __padding_1064: [u8; 1064],
    pub subsurface_profile: UPtr<USubsurfaceProfile>,
    pub specular_profile: UPtr<USpecularProfile>,
    __padding_end: [u8; 8],
}
impl UMaterialExpressionSubstrateSlabBSDF {}
#[repr(C, align(8))]
pub struct UMaterialExpressionSubstrateSimpleClearCoatBSDF {
    __padding_end: [u8; 584],
}
impl UMaterialExpressionSubstrateSimpleClearCoatBSDF {}
#[repr(C, align(8))]
pub struct UMaterialExpressionSubstrateVolumetricFogCloudBSDF {
    __padding_end: [u8; 400],
}
impl UMaterialExpressionSubstrateVolumetricFogCloudBSDF {}
#[repr(C, align(8))]
pub struct UMaterialExpressionSubstrateUnlitBSDF {
    __padding_end: [u8; 344],
}
impl UMaterialExpressionSubstrateUnlitBSDF {}
#[repr(C, align(8))]
pub struct UMaterialExpressionSubstrateHairBSDF {
    __padding_end: [u8; 536],
}
impl UMaterialExpressionSubstrateHairBSDF {}
#[repr(C, align(8))]
pub struct UMaterialExpressionSubstrateEyeBSDF {
    #[doc(hidden)]
    __padding_584: [u8; 584],
    pub subsurface_profile: UPtr<USubsurfaceProfile>,
}
impl UMaterialExpressionSubstrateEyeBSDF {}
#[repr(C, align(8))]
pub struct UMaterialExpressionSubstrateSingleLayerWaterBSDF {
    __padding_end: [u8; 728],
}
impl UMaterialExpressionSubstrateSingleLayerWaterBSDF {}
#[repr(C, align(8))]
pub struct UMaterialExpressionSubstrateLightFunction {
    __padding_end: [u8; 248],
}
impl UMaterialExpressionSubstrateLightFunction {}
#[repr(C, align(8))]
pub struct UMaterialExpressionSubstratePostProcess {
    __padding_end: [u8; 296],
}
impl UMaterialExpressionSubstratePostProcess {}
#[repr(C, align(8))]
pub struct UMaterialExpressionSubstrateUI {
    __padding_end: [u8; 296],
}
impl UMaterialExpressionSubstrateUI {}
#[repr(C, align(8))]
pub struct UMaterialExpressionSubstrateConvertToDecal {
    __padding_end: [u8; 296],
}
impl UMaterialExpressionSubstrateConvertToDecal {}
#[repr(C, align(8))]
pub struct UMaterialExpressionSubstrateConvertMaterialAttributes {
    #[doc(hidden)]
    __padding_448: [u8; 448],
    pub subsurface_profile: UPtr<USubsurfaceProfile>,
    __padding_end: [u8; 8],
}
impl UMaterialExpressionSubstrateConvertMaterialAttributes {}
#[repr(C, align(8))]
pub struct UMaterialExpressionSubstrateHorizontalMixing {
    __padding_end: [u8; 352],
}
impl UMaterialExpressionSubstrateHorizontalMixing {}
#[repr(C, align(8))]
pub struct UMaterialExpressionSubstrateVerticalLayering {
    __padding_end: [u8; 352],
}
impl UMaterialExpressionSubstrateVerticalLayering {}
#[repr(C, align(8))]
pub struct UMaterialExpressionSubstrateAdd {
    __padding_end: [u8; 304],
}
impl UMaterialExpressionSubstrateAdd {}
#[repr(C, align(8))]
pub struct UMaterialExpressionSubstrateWeight {
    __padding_end: [u8; 296],
}
impl UMaterialExpressionSubstrateWeight {}
#[repr(C, align(8))]
pub struct UMaterialExpressionSubstrateSelect {
    __padding_end: [u8; 352],
}
impl UMaterialExpressionSubstrateSelect {}
#[repr(C, align(8))]
pub struct UMaterialExpressionSubstrateUtilityBase {
    __padding_end: [u8; 200],
}
impl UMaterialExpressionSubstrateUtilityBase {}
#[repr(C, align(8))]
pub struct UMaterialExpressionSubstrateTransmittanceToMFP {
    __padding_end: [u8; 296],
}
impl UMaterialExpressionSubstrateTransmittanceToMFP {}
#[repr(C, align(8))]
pub struct UMaterialExpressionSubstrateMetalnessToDiffuseAlbedoF0 {
    __padding_end: [u8; 344],
}
impl UMaterialExpressionSubstrateMetalnessToDiffuseAlbedoF0 {}
#[repr(C, align(8))]
pub struct UMaterialExpressionSubstrateHazinessToSecondaryRoughness {
    __padding_end: [u8; 296],
}
impl UMaterialExpressionSubstrateHazinessToSecondaryRoughness {}
#[repr(C, align(8))]
pub struct UMaterialExpressionSubstrateThinFilm {
    __padding_end: [u8; 440],
}
impl UMaterialExpressionSubstrateThinFilm {}
#[repr(C, align(8))]
pub struct UMaterialExternalCodeCollection {
    __padding_end: [u8; 64],
}
impl UMaterialExternalCodeCollection {}
#[repr(C, align(8))]
pub struct UMaterialInstanceEditorOnlyData {
    __padding_end: [u8; 232],
}
impl UMaterialInstanceEditorOnlyData {}
#[repr(C, align(8))]
pub struct UMaterialInstanceDynamic {
    __padding_end: [u8; 1904],
}
impl UMaterialInstanceDynamic {}
#[repr(C, align(8))]
pub struct UMaterialParameterCollection {
    __padding_end: [u8; 336],
}
impl UMaterialParameterCollection {}
#[repr(C, align(8))]
pub struct UMeshBudgetProjectSettings {
    __padding_end: [u8; 128],
}
impl UMeshBudgetProjectSettings {}
#[repr(C, align(8))]
pub struct AMeshMergeCullingVolume {
    __padding_end: [u8; 1208],
}
impl AMeshMergeCullingVolume {}
#[repr(C, align(8))]
pub struct UMeshSimplificationSettings {
    __padding_end: [u8; 120],
}
impl UMeshSimplificationSettings {}
#[repr(C, align(8))]
pub struct UMeshVertexPainterKismetLibrary {
    __padding_end: [u8; 48],
}
impl UMeshVertexPainterKismetLibrary {}
#[repr(C, align(8))]
pub struct UModel {
    __padding_end: [u8; 952],
}
impl UModel {}
#[repr(C, align(8))]
pub struct UMorphTarget {
    __padding_end: [u8; 72],
}
impl UMorphTarget {}
#[repr(C, align(8))]
pub struct ANavigationObjectBase {
    __padding_end: [u8; 1176],
}
impl ANavigationObjectBase {}
pub struct INavMovementInterface {}
#[repr(C, align(8))]
pub struct UNavMovementInterface {
    __padding_end: [u8; 48],
}
impl UNavMovementInterface {}
#[repr(C, align(16))]
pub struct USimulatedClientNetConnection {
    __padding_end: [u8; 8048],
}
impl USimulatedClientNetConnection {}
#[repr(C, align(8))]
pub struct UNetworkSettings {
    __padding_end: [u8; 128],
}
impl UNetworkSettings {}
#[repr(C, align(8))]
pub struct ABandwidthTestActor {
    __padding_end: [u8; 1168],
}
impl ABandwidthTestActor {}
#[repr(C, align(8))]
pub struct UDataStreamChannel {
    __padding_end: [u8; 8360],
}
impl UDataStreamChannel {}
#[repr(C, align(16))]
pub struct UEngineReplicationBridge {
    __padding_end: [u8; 1760],
}
impl UEngineReplicationBridge {}
#[repr(C, align(8))]
pub struct UNetActorFactory {
    __padding_end: [u8; 168],
}
impl UNetActorFactory {}
#[repr(C, align(8))]
pub struct UNetSubObjectFactory {
    __padding_end: [u8; 64],
}
impl UNetSubObjectFactory {}
#[repr(C, align(8))]
pub struct UNetFaultConfig {
    __padding_end: [u8; 160],
}
impl UNetFaultConfig {}
#[repr(C, align(8))]
pub struct UNetPushModelHelpers {
    __padding_end: [u8; 48],
}
impl UNetPushModelHelpers {}
#[repr(C, align(8))]
pub struct UNetworkMetricsConfig {
    __padding_end: [u8; 64],
}
impl UNetworkMetricsConfig {}
#[repr(C, align(8))]
pub struct UNetworkMetricsDatabase {
    __padding_end: [u8; 448],
}
impl UNetworkMetricsDatabase {}
#[repr(C, align(8))]
pub struct UNetworkMetricsBaseListener {
    __padding_end: [u8; 72],
}
impl UNetworkMetricsBaseListener {}
#[repr(C, align(8))]
pub struct UNetworkMetricsLog {
    __padding_end: [u8; 72],
}
impl UNetworkMetricsLog {}
#[repr(C, align(8))]
pub struct UNetworkMetricsCSV {
    __padding_end: [u8; 80],
}
impl UNetworkMetricsCSV {}
#[repr(C, align(8))]
pub struct UNetworkMetricsCSV_Replication {
    __padding_end: [u8; 80],
}
impl UNetworkMetricsCSV_Replication {}
#[repr(C, align(8))]
pub struct UNetworkMetricsPerfCounters {
    __padding_end: [u8; 72],
}
impl UNetworkMetricsPerfCounters {}
#[repr(C, align(8))]
pub struct UNetworkMetricsStats {
    __padding_end: [u8; 88],
}
impl UNetworkMetricsStats {}
pub struct IReplicatedObjectInterface {}
#[repr(C, align(8))]
pub struct UReplicatedObjectInterface {
    __padding_end: [u8; 48],
}
impl UReplicatedObjectInterface {}
#[repr(C, align(8))]
pub struct URPCDoSDetectionConfig {
    __padding_end: [u8; 144],
}
impl URPCDoSDetectionConfig {}
#[repr(C, align(8))]
pub struct UNetworkSubsystem {
    __padding_end: [u8; 144],
}
impl UNetworkSubsystem {}
#[repr(C, align(8))]
pub struct ANote {
    #[doc(hidden)]
    __padding_1136: [u8; 1136],
    pub text: FString,
    __padding_end: [u8; 16],
}
impl ANote {}
#[repr(C, align(8))]
pub struct UValkyrieMetaData {
    __padding_end: [u8; 112],
}
impl UValkyrieMetaData {}
#[repr(C, align(8))]
pub struct UObjectLibrary {
    __padding_end: [u8; 216],
}
impl UObjectLibrary {}
#[repr(C, align(8))]
pub struct UObjectReferencer {
    __padding_end: [u8; 64],
}
impl UObjectReferencer {}
#[repr(C, align(8))]
pub struct UObjectTraceWorldSubsystem {
    __padding_end: [u8; 80],
}
impl UObjectTraceWorldSubsystem {}
#[repr(C, align(8))]
pub struct UOnlineBlueprintCallProxyBase {
    __padding_end: [u8; 56],
}
impl UOnlineBlueprintCallProxyBase {}
#[repr(C, align(8))]
pub struct UOnlineEngineInterface {
    __padding_end: [u8; 48],
}
impl UOnlineEngineInterface {}
#[repr(C, align(8))]
pub struct UOnlineSession {
    __padding_end: [u8; 48],
}
impl UOnlineSession {}
#[repr(C, align(8))]
pub struct UPackageMapClient {
    __padding_end: [u8; 1216],
}
impl UPackageMapClient {}
#[repr(C, align(8))]
pub struct APackedLevelActor {
    __padding_end: [u8; 1512],
}
impl APackedLevelActor {}
#[repr(C, align(8))]
pub struct UEngineHandlerComponentFactory {
    __padding_end: [u8; 48],
}
impl UEngineHandlerComponentFactory {}
#[repr(C, align(8))]
pub struct APainCausingVolume {
    #[doc(hidden)]
    __padding_1224: [u8; 1224],
    pub flags_1224: u8,
    pub damage_per_sec: f32,
    pub damage_type: TSubclassOf<UDamageType>,
    pub pain_interval: f32,
    pub flags_1244: u8,
    __padding_end: [u8; 19],
}
impl APainCausingVolume {}
#[repr(C, align(8))]
pub struct AParticleEventManager {
    __padding_end: [u8; 1136],
}
impl AParticleEventManager {}
#[repr(C, align(8))]
pub struct UParticleLODLevel {
    __padding_end: [u8; 192],
}
impl UParticleLODLevel {}
#[repr(C, align(16))]
pub struct UFXSystemComponent {
    __padding_end: [u8; 1520],
}
impl UFXSystemComponent {}
#[repr(C, align(16))]
pub struct UParticleSystemComponent {
    #[doc(hidden)]
    __padding_1512: [u8; 1512],
    pub template: UPtr<UParticleSystem>,
    #[doc(hidden)]
    __padding_1553: [u8; 33],
    pub flags_1553: u8,
    pub flags_1554: u8,
    #[doc(hidden)]
    __padding_1565: [u8; 10],
    pub lod_method: ParticleSystemLODMethod,
    pub instance_parameters: TArray<FParticleSysParam>,
    #[doc(hidden)]
    __padding_1748: [u8; 164],
    pub seconds_before_inactive: f32,
    #[doc(hidden)]
    __padding_1824: [u8; 72],
    pub custom_time_dilation: f32,
    #[doc(hidden)]
    __padding_1912: [u8; 84],
    pub auto_attach_parent: TWeakObjectPtr<USceneComponent>,
    pub auto_attach_socket_name: FName,
    #[doc(hidden)]
    __padding_1933: [u8; 1],
    pub auto_attach_location_rule: EAttachmentRule,
    pub auto_attach_rotation_rule: EAttachmentRule,
    pub auto_attach_scale_rule: EAttachmentRule,
    __padding_end: [u8; 416],
}
impl UParticleSystemComponent {}
#[repr(C, align(8))]
pub struct USubUVAnimation {
    __padding_end: [u8; 104],
}
impl USubUVAnimation {}
#[repr(C, align(8))]
pub struct UAsyncPhysicsInputComponent {
    __padding_end: [u8; 280],
}
impl UAsyncPhysicsInputComponent {}
#[repr(C, align(8))]
pub struct AClusterUnionActor {
    #[doc(hidden)]
    __padding_1136: [u8; 1136],
    pub cluster_union: UPtr<UClusterUnionComponent>,
}
impl AClusterUnionActor {}
#[repr(C, align(16))]
pub struct UClusterUnionComponent {
    #[doc(hidden)]
    __padding_1504: [u8; 1504],
    pub b_enable_damage_from_collision: bool,
    __padding_end: [u8; 639],
}
impl UClusterUnionComponent {}
#[repr(C, align(8))]
pub struct UClusterUnionReplicatedProxyComponent {
    __padding_end: [u8; 384],
}
impl UClusterUnionReplicatedProxyComponent {}
#[repr(C, align(8))]
pub struct UConstraintInstanceBlueprintLibrary {
    __padding_end: [u8; 48],
}
impl UConstraintInstanceBlueprintLibrary {}
#[repr(C, align(8))]
pub struct UPhysicsQueryHandler {
    __padding_end: [u8; 48],
}
impl UPhysicsQueryHandler {}
#[repr(C, align(8))]
pub struct UDefaultPhysicsQueryHandler {
    __padding_end: [u8; 48],
}
impl UDefaultPhysicsQueryHandler {}
#[repr(C, align(8))]
pub struct UChaosBlueprintLibrary {
    __padding_end: [u8; 48],
}
impl UChaosBlueprintLibrary {}
#[repr(C, align(8))]
pub struct UChaosEventRelay {
    __padding_end: [u8; 144],
}
impl UChaosEventRelay {}
#[repr(C, align(8))]
pub struct UNetworkPhysicsSystem {
    __padding_end: [u8; 64],
}
impl UNetworkPhysicsSystem {}
#[repr(C, align(8))]
pub struct UNetworkPhysicsComponent {
    __padding_end: [u8; 656],
}
impl UNetworkPhysicsComponent {}
#[repr(C, align(8))]
pub struct UNetworkPhysicsSettingsDataAsset {
    __padding_end: [u8; 408],
}
impl UNetworkPhysicsSettingsDataAsset {}
#[repr(C, align(8))]
pub struct UNetworkPhysicsSettingsComponent {
    #[doc(hidden)]
    __padding_240: [u8; 240],
    pub settings_data_asset: UPtr<UNetworkPhysicsSettingsDataAsset>,
    __padding_end: [u8; 336],
}
impl UNetworkPhysicsSettingsComponent {}
#[repr(C, align(8))]
pub struct UPhysicalAnimationComponent {
    #[doc(hidden)]
    __padding_240: [u8; 240],
    pub strength_multiplyer: f32,
    __padding_end: [u8; 60],
}
impl UPhysicalAnimationComponent {}
#[repr(C, align(8))]
pub struct UPhysicalMaterialMask {
    #[doc(hidden)]
    __padding_56: [u8; 56],
    pub mask_texture: UPtr<UTexture>,
    pub uv_channel_index: i32,
    pub address_x: TextureAddress,
    pub address_y: TextureAddress,
    __padding_end: [u8; 10],
}
impl UPhysicalMaterialMask {}
#[repr(C, align(8))]
pub struct UPhysicsAsset {
    __padding_end: [u8; 528],
}
impl UPhysicsAsset {}
pub struct IPhysicsBodyInstanceOwnerResolver {}
#[repr(C, align(8))]
pub struct UPhysicsBodyInstanceOwnerResolver {
    __padding_end: [u8; 48],
}
impl UPhysicsBodyInstanceOwnerResolver {}
#[repr(C, align(8))]
pub struct UPhysicsCollisionHandler {
    #[doc(hidden)]
    __padding_48: [u8; 48],
    pub impact_threshold: f32,
    pub impact_re_fire_delay: f32,
    pub default_impact_sound: UPtr<USoundBase>,
    __padding_end: [u8; 8],
}
impl UPhysicsCollisionHandler {}
#[repr(C, align(8))]
pub struct APhysicsConstraintActor {
    #[doc(hidden)]
    __padding_1136: [u8; 1136],
    pub constraint_comp: UPtr<UPhysicsConstraintComponent>,
    __padding_end: [u8; 24],
}
impl APhysicsConstraintActor {}
#[repr(C, align(16))]
pub struct UPhysicsConstraintComponent {
    __padding_end: [u8; 1744],
}
impl UPhysicsConstraintComponent {}
#[repr(C, align(16))]
pub struct UPhysicsConstraintTemplate {
    __padding_end: [u8; 1632],
}
impl UPhysicsConstraintTemplate {}
#[repr(C, align(16))]
pub struct UPhysicsHandleComponent {
    #[doc(hidden)]
    __padding_260: [u8; 260],
    pub flags_260: u8,
    pub linear_damping: f32,
    pub linear_stiffness: f32,
    pub angular_damping: f32,
    pub angular_stiffness: f32,
    #[doc(hidden)]
    __padding_480: [u8; 200],
    pub interpolation_speed: f32,
    __padding_end: [u8; 1164],
}
impl UPhysicsHandleComponent {}
#[repr(C, align(8))]
pub struct UPhysicsObjectBlueprintLibrary {
    __padding_end: [u8; 48],
}
impl UPhysicsObjectBlueprintLibrary {}
#[repr(C, align(8))]
pub struct UPhysicsSettings {
    __padding_end: [u8; 632],
}
impl UPhysicsSettings {}
#[repr(C, align(8))]
pub struct APhysicsThruster {
    #[doc(hidden)]
    __padding_1136: [u8; 1136],
    pub thruster_component: UPtr<UPhysicsThrusterComponent>,
    __padding_end: [u8; 8],
}
impl APhysicsThruster {}
#[repr(C, align(16))]
pub struct URadialForceComponent {
    #[doc(hidden)]
    __padding_656: [u8; 656],
    pub radius: f32,
    pub falloff: crate::bindings::physics_core::ERadialImpulseFalloff,
    pub impulse_strength: f32,
    pub flags_668: u8,
    pub force_strength: f32,
    pub destructible_damage: f32,
    __padding_end: [u8; 24],
}
impl URadialForceComponent {}
#[repr(C, align(8))]
pub struct USkeletalBodySetup {
    __padding_end: [u8; 984],
}
impl USkeletalBodySetup {}
#[repr(C, align(16))]
pub struct UPhysicsFieldComponent {
    __padding_end: [u8; 880],
}
impl UPhysicsFieldComponent {}
#[repr(C, align(8))]
pub struct UPhysicsFieldStatics {
    __padding_end: [u8; 48],
}
impl UPhysicsFieldStatics {}
#[repr(C, align(16))]
pub struct APlayerCameraManager {
    #[doc(hidden)]
    __padding_1144: [u8; 1144],
    pub transform_component: UPtr<USceneComponent>,
    #[doc(hidden)]
    __padding_1164: [u8; 12],
    pub default_fov: f32,
    #[doc(hidden)]
    __padding_1172: [u8; 4],
    pub default_ortho_width: f32,
    #[doc(hidden)]
    __padding_1180: [u8; 4],
    pub default_aspect_ratio: f32,
    #[doc(hidden)]
    __padding_10688: [u8; 9504],
    pub default_modifiers: TArray<TSubclassOf<UCameraModifier>>,
    pub free_cam_distance: f32,
    pub free_cam_offset: crate::bindings::core_u_object::FVector,
    pub view_target_offset: crate::bindings::core_u_object::FVector,
    #[doc(hidden)]
    __padding_10888: [u8; 128],
    pub flags_10888: u8,
    pub auto_plane_shift: f32,
    pub flags_10896: u8,
    pub flags_10897: u8,
    pub view_pitch_min: f32,
    pub view_pitch_max: f32,
    pub view_yaw_min: f32,
    pub view_yaw_max: f32,
    pub view_roll_min: f32,
    pub view_roll_max: f32,
    __padding_end: [u8; 20],
}
impl APlayerCameraManager {}
#[repr(C, align(8))]
pub struct ANoPawnPlayerController {
    __padding_end: [u8; 2376],
}
impl ANoPawnPlayerController {}
#[repr(C, align(8))]
pub struct APlayerStart {
    #[doc(hidden)]
    __padding_1176: [u8; 1176],
    pub player_start_tag: FName,
    __padding_end: [u8; 12],
}
impl APlayerStart {}
#[repr(C, align(8))]
pub struct APlayerStartPIE {
    __padding_end: [u8; 1200],
}
impl APlayerStartPIE {}
#[repr(C, align(8))]
pub struct APlayerState {
    #[doc(hidden)]
    __padding_1144: [u8; 1144],
    pub score: f32,
    pub player_id: i32,
    pub compressed_ping: u8,
    #[doc(hidden)]
    __padding_1154: [u8; 1],
    pub flags_1154: u8,
    #[doc(hidden)]
    __padding_1280: [u8; 120],
    pub pawn_private: UPtr<APawn>,
    __padding_end: [u8; 56],
}
impl APlayerState {}
#[repr(C, align(8))]
pub struct UPluginBlueprintLibrary {
    __padding_end: [u8; 48],
}
impl UPluginBlueprintLibrary {}
#[repr(C, align(16))]
pub struct APostProcessVolume {
    #[doc(hidden)]
    __padding_1216: [u8; 1216],
    pub settings: FPostProcessSettings,
    pub priority: f32,
    pub blend_radius: f32,
    pub blend_weight: f32,
    pub flags_3180: u8,
    __padding_end: [u8; 19],
}
impl APostProcessVolume {}
#[repr(C, align(8))]
pub struct APrecomputedVisibilityVolume {
    __padding_end: [u8; 1208],
}
impl APrecomputedVisibilityVolume {}
#[repr(C, align(8))]
pub struct UPrimaryAssetLabel {
    __padding_end: [u8; 136],
}
impl UPrimaryAssetLabel {}
#[repr(C, align(8))]
pub struct UHealthSnapshotBlueprintLibrary {
    __padding_end: [u8; 48],
}
impl UHealthSnapshotBlueprintLibrary {}
#[repr(C, align(8))]
pub struct ULevelStreamingProfilingSubsystem {
    __padding_end: [u8; 448],
}
impl ULevelStreamingProfilingSubsystem {}
#[repr(C, align(8))]
pub struct UProxyLODMeshSimplificationSettings {
    __padding_end: [u8; 120],
}
impl UProxyLODMeshSimplificationSettings {}
#[repr(C, align(8))]
pub struct ARectLight {
    #[doc(hidden)]
    __padding_1152: [u8; 1152],
    pub rect_light_component: UPtr<URectLightComponent>,
}
impl ARectLight {}
#[repr(C, align(8))]
pub struct URendererSettings {
    __padding_end: [u8; 1000],
}
impl URendererSettings {}
#[repr(C, align(8))]
pub struct URendererOverrideSettings {
    __padding_end: [u8; 112],
}
impl URendererOverrideSettings {}
#[repr(C, align(8))]
pub struct UNeuralProfile {
    __padding_end: [u8; 136],
}
impl UNeuralProfile {}
pub struct ISkeletalMeshHalfEdgeBufferAccessor {}
#[repr(C, align(8))]
pub struct USkeletalMeshHalfEdgeBufferAccessor {
    __padding_end: [u8; 48],
}
impl USkeletalMeshHalfEdgeBufferAccessor {}
#[repr(C, align(8))]
pub struct USpecularProfile {
    __padding_end: [u8; 1120],
}
impl USpecularProfile {}
#[repr(C, align(8))]
pub struct USubsurfaceProfile {
    __padding_end: [u8; 224],
}
impl USubsurfaceProfile {}
#[repr(C, align(16))]
pub struct UReplayNetConnection {
    __padding_end: [u8; 10736],
}
impl UReplayNetConnection {}
#[repr(C, align(8))]
pub struct UGameInstanceSubsystem {
    __padding_end: [u8; 56],
}
impl UGameInstanceSubsystem {}
#[repr(C, align(8))]
pub struct UReplaySubsystem {
    __padding_end: [u8; 72],
}
impl UReplaySubsystem {}
#[repr(C, align(8))]
pub struct UReverbEffect {
    #[doc(hidden)]
    __padding_48: [u8; 48],
    pub b_bypass_early_reflections: bool,
    #[doc(hidden)]
    __padding_64: [u8; 15],
    pub b_bypass_late_reflections: bool,
    __padding_end: [u8; 47],
}
impl UReverbEffect {}
#[repr(C, align(8))]
pub struct URuntimeOptionsBase {
    __padding_end: [u8; 64],
}
impl URuntimeOptionsBase {}
#[repr(C, align(8))]
pub struct UScene {
    __padding_end: [u8; 48],
}
impl UScene {}
#[repr(C, align(8))]
pub struct USCS_Node {
    __padding_end: [u8; 304],
}
impl USCS_Node {}
#[repr(C, align(8))]
pub struct USimpleConstructionScript {
    __padding_end: [u8; 216],
}
impl USimpleConstructionScript {}
#[repr(C, align(8))]
pub struct USkinnedAsset {
    __padding_end: [u8; 288],
}
impl USkinnedAsset {}
#[repr(C, align(8))]
pub struct USkeletalMesh {
    #[doc(hidden)]
    __padding_352: [u8; 352],
    pub skeleton: UPtr<USkeleton>,
    #[doc(hidden)]
    __padding_472: [u8; 112],
    pub positive_bounds_extension: crate::bindings::core_u_object::FVector,
    pub negative_bounds_extension: crate::bindings::core_u_object::FVector,
    pub materials: TArray<FSkeletalMaterial>,
    #[doc(hidden)]
    __padding_1288: [u8; 752],
    pub lod_settings: UPtr<USkeletalMeshLODSettings>,
    pub default_animating_rig: TSoftObjectPtr<crate::bindings::core_u_object::UObject>,
    #[doc(hidden)]
    __padding_1376: [u8; 32],
    pub physics_asset: UPtr<UPhysicsAsset>,
    pub shadow_physics_asset: UPtr<UPhysicsAsset>,
    pub node_mapping_data: TArray<UPtr<UNodeMappingContainer>>,
    #[doc(hidden)]
    __padding_1576: [u8; 168],
    pub morph_targets: TArray<UPtr<UMorphTarget>>,
    #[doc(hidden)]
    __padding_2008: [u8; 416],
    pub post_process_anim_blueprint: TSubclassOf<UAnimInstance>,
    #[doc(hidden)]
    __padding_2024: [u8; 8],
    pub mesh_clothing_assets: TArray<
        UPtr<crate::bindings::clothing_system_runtime_interface::UClothingAssetBase>,
    >,
    #[doc(hidden)]
    __padding_2272: [u8; 232],
    pub default_mesh_deformer: UPtr<UMeshDeformer>,
    pub target_mesh_deformers: UPtr<UMeshDeformerCollection>,
    pub overlay_material: UPtr<UMaterialInterface>,
    pub overlay_material_max_draw_distance: f32,
    pub forward_axis: crate::bindings::core_u_object::EAxis,
    __padding_end: [u8; 27],
}
impl USkeletalMesh {}
#[repr(C, align(8))]
pub struct USkeletalMeshEditorData {
    __padding_end: [u8; 64],
}
impl USkeletalMeshEditorData {}
#[repr(C, align(8))]
pub struct USkeletalMeshLODSettings {
    __padding_end: [u8; 648],
}
impl USkeletalMeshLODSettings {}
#[repr(C, align(8))]
pub struct USkeletalMeshSimplificationSettings {
    __padding_end: [u8; 120],
}
impl USkeletalMeshSimplificationSettings {}
#[repr(C, align(8))]
pub struct USkeletalMeshDescriptionBulkData {
    __padding_end: [u8; 72],
}
impl USkeletalMeshDescriptionBulkData {}
#[repr(C, align(16))]
pub struct UButtonStyleAsset {
    __padding_end: [u8; 1136],
}
impl UButtonStyleAsset {}
#[repr(C, align(16))]
pub struct UCheckBoxStyleAsset {
    __padding_end: [u8; 2880],
}
impl UCheckBoxStyleAsset {}
#[repr(C, align(16))]
pub struct USlateBrushAsset {
    __padding_end: [u8; 256],
}
impl USlateBrushAsset {}
pub struct ISlateTextureAtlasInterface {}
#[repr(C, align(8))]
pub struct USlateTextureAtlasInterface {
    __padding_end: [u8; 48],
}
impl USlateTextureAtlasInterface {}
#[repr(C, align(8))]
pub struct USoundAttenuation {
    #[doc(hidden)]
    __padding_48: [u8; 48],
    pub attenuation: FSoundAttenuationSettings,
}
impl USoundAttenuation {}
#[repr(C, align(8))]
pub struct USoundClass {
    #[doc(hidden)]
    __padding_56: [u8; 56],
    pub properties: FSoundClassProperties,
    pub child_classes: TArray<UPtr<USoundClass>>,
    pub passive_sound_mix_modifiers: TArray<FPassiveSoundMixModifier>,
    pub parent_class: UPtr<USoundClass>,
    __padding_end: [u8; 24],
}
impl USoundClass {}
#[repr(C, align(8))]
pub struct USoundConcurrency {
    #[doc(hidden)]
    __padding_48: [u8; 48],
    pub concurrency: FSoundConcurrencySettings,
}
impl USoundConcurrency {}
#[repr(C, align(8))]
pub struct USoundCue {
    #[doc(hidden)]
    __padding_520: [u8; 520],
    pub first_node: UPtr<USoundNode>,
    pub volume_multiplier: f32,
    pub pitch_multiplier: f32,
    __padding_end: [u8; 1112],
}
impl USoundCue {}
#[repr(C, align(8))]
pub struct USoundGroups {
    __padding_end: [u8; 144],
}
impl USoundGroups {}
#[repr(C, align(8))]
pub struct USoundMix {
    #[doc(hidden)]
    __padding_120: [u8; 120],
    pub sound_class_effects: TArray<FSoundClassAdjuster>,
    __padding_end: [u8; 24],
}
impl USoundMix {}
#[repr(C, align(8))]
pub struct USoundNode {
    #[doc(hidden)]
    __padding_48: [u8; 48],
    pub child_nodes: TArray<UPtr<USoundNode>>,
    __padding_end: [u8; 24],
}
impl USoundNode {}
#[repr(C, align(8))]
pub struct USoundNodeAssetReferencer {
    __padding_end: [u8; 88],
}
impl USoundNodeAssetReferencer {}
#[repr(C, align(8))]
pub struct USoundNodeAttenuation {
    __padding_end: [u8; 1128],
}
impl USoundNodeAttenuation {}
#[repr(C, align(8))]
pub struct USoundNodeBranch {
    __padding_end: [u8; 104],
}
impl USoundNodeBranch {}
#[repr(C, align(8))]
pub struct USoundNodeConcatenator {
    __padding_end: [u8; 104],
}
impl USoundNodeConcatenator {}
#[repr(C, align(8))]
pub struct USoundNodeDelay {
    __padding_end: [u8; 96],
}
impl USoundNodeDelay {}
#[repr(C, align(8))]
pub struct USoundNodeDialoguePlayer {
    __padding_end: [u8; 128],
}
impl USoundNodeDialoguePlayer {}
#[repr(C, align(8))]
pub struct USoundNodeDistanceCrossFade {
    __padding_end: [u8; 104],
}
impl USoundNodeDistanceCrossFade {}
#[repr(C, align(8))]
pub struct USoundNodeDoppler {
    __padding_end: [u8; 104],
}
impl USoundNodeDoppler {}
#[repr(C, align(8))]
pub struct USoundNodeEnveloper {
    __padding_end: [u8; 416],
}
impl USoundNodeEnveloper {}
#[repr(C, align(8))]
pub struct USoundNodeGroupControl {
    __padding_end: [u8; 104],
}
impl USoundNodeGroupControl {}
#[repr(C, align(8))]
pub struct USoundNodeLooping {
    __padding_end: [u8; 96],
}
impl USoundNodeLooping {}
#[repr(C, align(8))]
pub struct USoundNodeMature {
    __padding_end: [u8; 88],
}
impl USoundNodeMature {}
#[repr(C, align(8))]
pub struct USoundNodeMixer {
    __padding_end: [u8; 104],
}
impl USoundNodeMixer {}
#[repr(C, align(8))]
pub struct USoundNodeModulator {
    __padding_end: [u8; 104],
}
impl USoundNodeModulator {}
#[repr(C, align(8))]
pub struct USoundNodeModulatorContinuous {
    __padding_end: [u8; 160],
}
impl USoundNodeModulatorContinuous {}
#[repr(C, align(8))]
pub struct USoundNodeOscillator {
    __padding_end: [u8; 128],
}
impl USoundNodeOscillator {}
#[repr(C, align(8))]
pub struct USoundNodeParamCrossFade {
    __padding_end: [u8; 120],
}
impl USoundNodeParamCrossFade {}
#[repr(C, align(8))]
pub struct USoundNodeQualityLevel {
    __padding_end: [u8; 136],
}
impl USoundNodeQualityLevel {}
#[repr(C, align(8))]
pub struct USoundNodeRandom {
    __padding_end: [u8; 152],
}
impl USoundNodeRandom {}
#[repr(C, align(8))]
pub struct USoundNodeSoundClass {
    __padding_end: [u8; 104],
}
impl USoundNodeSoundClass {}
#[repr(C, align(8))]
pub struct USoundNodeSwitch {
    __padding_end: [u8; 104],
}
impl USoundNodeSwitch {}
#[repr(C, align(8))]
pub struct USoundNodeWaveParam {
    __padding_end: [u8; 104],
}
impl USoundNodeWaveParam {}
#[repr(C, align(8))]
pub struct USoundNodeWavePlayer {
    __padding_end: [u8; 152],
}
impl USoundNodeWavePlayer {}
#[repr(C, align(8))]
pub struct USoundSourceBus {
    __padding_end: [u8; 2104],
}
impl USoundSourceBus {}
#[repr(C, align(8))]
pub struct USoundSubmixBase {
    #[doc(hidden)]
    __padding_56: [u8; 56],
    pub b_auto_disable: bool,
    pub auto_disable_time: f32,
    pub child_submixes: TArray<UPtr<USoundSubmixBase>>,
    __padding_end: [u8; 80],
}
impl USoundSubmixBase {}
#[repr(C, align(8))]
pub struct USoundSubmixWithParentBase {
    #[doc(hidden)]
    __padding_160: [u8; 160],
    pub parent_submix: UPtr<USoundSubmixBase>,
    #[doc(hidden)]
    __padding_248: [u8; 80],
    pub flags_248: u8,
    __padding_end: [u8; 7],
}
impl USoundSubmixWithParentBase {}
#[repr(C, align(8))]
pub struct USoundSubmix {
    #[doc(hidden)]
    __padding_256: [u8; 256],
    pub flags_256: u8,
    pub submix_effect_chain: TArray<UPtr<USoundEffectSubmixPreset>>,
    pub ambisonics_plugin_settings: UPtr<
        crate::bindings::audio_extensions::USoundfieldEncodingSettingsBase,
    >,
    pub envelope_follower_attack_time: i32,
    pub envelope_follower_release_time: i32,
    pub output_volume_modulation: FSoundModulationDestinationSettings,
    pub wet_level_modulation: FSoundModulationDestinationSettings,
    pub dry_level_modulation: FSoundModulationDestinationSettings,
    pub flags_584: u8,
    pub audio_link_settings: UPtr<
        crate::bindings::audio_link_core::UAudioLinkSettingsAbstract,
    >,
    __padding_end: [u8; 48],
}
impl USoundSubmix {}
#[repr(C, align(8))]
pub struct USoundfieldSubmix {
    __padding_end: [u8; 304],
}
impl USoundfieldSubmix {}
#[repr(C, align(8))]
pub struct UEndpointSubmix {
    __padding_end: [u8; 192],
}
impl UEndpointSubmix {}
#[repr(C, align(8))]
pub struct USoundfieldEndpointSubmix {
    __padding_end: [u8; 224],
}
impl USoundfieldEndpointSubmix {}
#[repr(C, align(8))]
pub struct USparseVolumeTexture {
    __padding_end: [u8; 48],
}
impl USparseVolumeTexture {}
#[repr(C, align(16))]
pub struct USparseVolumeTextureFrame {
    __padding_end: [u8; 704],
}
impl USparseVolumeTextureFrame {}
#[repr(C, align(16))]
pub struct UStreamableSparseVolumeTexture {
    #[doc(hidden)]
    __padding_112: [u8; 112],
    pub address_x: TextureAddress,
    pub address_y: TextureAddress,
    pub address_z: TextureAddress,
    pub b_local_ddc_only: bool,
    __padding_end: [u8; 92],
}
impl UStreamableSparseVolumeTexture {}
#[repr(C, align(16))]
pub struct UStaticSparseVolumeTexture {
    __padding_end: [u8; 208],
}
impl UStaticSparseVolumeTexture {}
#[repr(C, align(16))]
pub struct UAnimatedSparseVolumeTexture {
    __padding_end: [u8; 224],
}
impl UAnimatedSparseVolumeTexture {}
#[repr(C, align(8))]
pub struct UAnimatedSparseVolumeTextureController {
    #[doc(hidden)]
    __padding_48: [u8; 48],
    pub sparse_volume_texture: UPtr<USparseVolumeTexture>,
    pub time: f32,
    pub b_is_playing: bool,
    pub frame_rate: f32,
    pub mip_level: i32,
    pub b_blocking_streaming_requests: bool,
    __padding_end: [u8; 7],
}
impl UAnimatedSparseVolumeTextureController {}
#[repr(C, align(8))]
pub struct ASpectatorPawn {
    __padding_end: [u8; 1304],
}
impl ASpectatorPawn {}
#[repr(C, align(8))]
pub struct USpectatorPawnMovement {
    __padding_end: [u8; 488],
}
impl USpectatorPawnMovement {}
#[repr(C, align(8))]
pub struct ASplineMeshActor {
    #[doc(hidden)]
    __padding_1136: [u8; 1136],
    pub spline_mesh_component: UPtr<USplineMeshComponent>,
}
impl ASplineMeshActor {}
#[repr(C, align(8))]
pub struct UStaticMesh {
    #[doc(hidden)]
    __padding_1272: [u8; 1272],
    pub static_materials: TArray<FStaticMaterial>,
    #[doc(hidden)]
    __padding_1372: [u8; 84],
    pub static_mesh_paint_support: EStaticMeshPaintSupport,
    #[doc(hidden)]
    __padding_1400: [u8; 24],
    pub lod_for_collision: i32,
    __padding_end: [u8; 604],
}
impl UStaticMesh {
    pub fn verify_layout() {
        log::warn!(
            "{} = {} vs {}", "static_materials", std::mem::offset_of!(UStaticMesh,
            static_materials), 1272usize
        );
        log::warn!(
            "{} = {} vs {}", "static_mesh_paint_support",
            std::mem::offset_of!(UStaticMesh, static_mesh_paint_support), 1372usize
        );
        log::warn!(
            "{} = {} vs {}", "lod_for_collision", std::mem::offset_of!(UStaticMesh,
            lod_for_collision), 1400usize
        );
    }
}
#[repr(C, align(8))]
pub struct UStaticMeshDescriptionBulkData {
    __padding_end: [u8; 72],
}
impl UStaticMeshDescriptionBulkData {}
#[repr(C, align(8))]
pub struct UActorTextureStreamingBuildDataComponent {
    __padding_end: [u8; 272],
}
impl UActorTextureStreamingBuildDataComponent {}
#[repr(C, align(8))]
pub struct AServerStreamingLevelsVisibility {
    __padding_end: [u8; 1216],
}
impl AServerStreamingLevelsVisibility {}
#[repr(C, align(8))]
pub struct UAudioSubsystemCollectionRoot {
    __padding_end: [u8; 56],
}
impl UAudioSubsystemCollectionRoot {}
#[repr(C, align(8))]
pub struct ULocalPlayerSubsystem {
    __padding_end: [u8; 56],
}
impl ULocalPlayerSubsystem {}
#[repr(C, align(8))]
pub struct USubsystemBlueprintLibrary {
    __padding_end: [u8; 48],
}
impl USubsystemBlueprintLibrary {}
#[repr(C, align(8))]
pub struct USubtitleAssetUserData {
    #[doc(hidden)]
    __padding_48: [u8; 48],
    pub subtitles: TArray<FSubtitleAssetData>,
}
impl USubtitleAssetUserData {}
#[repr(C, align(8))]
pub struct ATargetPoint {
    #[doc(hidden)]
    __padding_1136: [u8; 1136],
    pub sprite_component: UPtr<UBillboardComponent>,
    pub arrow_component: UPtr<UArrowComponent>,
}
impl ATargetPoint {}
#[repr(C, align(8))]
pub struct UTaskSyncManagerSettings {
    __padding_end: [u8; 120],
}
impl UTaskSyncManagerSettings {}
#[repr(C, align(8))]
pub struct UAutomationTestSettings {
    __padding_end: [u8; 872],
}
impl UAutomationTestSettings {}
#[repr(C, align(8))]
pub struct UAsyncLoadingTests_ConvertFromType_V1 {
    __padding_end: [u8; 96],
}
impl UAsyncLoadingTests_ConvertFromType_V1 {}
#[repr(C, align(8))]
pub struct UAsyncLoadingTests_ConvertFromType_V2 {
    __padding_end: [u8; 56],
}
impl UAsyncLoadingTests_ConvertFromType_V2 {}
#[repr(C, align(8))]
pub struct UAsyncLoadingTests_Shared {
    __padding_end: [u8; 104],
}
impl UAsyncLoadingTests_Shared {}
#[repr(C, align(8))]
pub struct UTextImportContainer {
    __padding_end: [u8; 72],
}
impl UTextImportContainer {}
#[repr(C, align(8))]
pub struct UTransactionDiffingTestObject {
    __padding_end: [u8; 176],
}
impl UTransactionDiffingTestObject {}
#[repr(C, align(16))]
pub struct UTexture2DDynamic {
    __padding_end: [u8; 1264],
}
impl UTexture2DDynamic {}
#[repr(C, align(8))]
pub struct UTextureCollection {
    __padding_end: [u8; 88],
}
impl UTextureCollection {}
#[repr(C, align(16))]
pub struct UTextureCube {
    __padding_end: [u8; 1328],
}
impl UTextureCube {}
#[repr(C, align(16))]
pub struct UTextureCubeArray {
    __padding_end: [u8; 1360],
}
impl UTextureCubeArray {}
#[repr(C, align(16))]
pub struct UTextureLightProfile {
    __padding_end: [u8; 1376],
}
impl UTextureLightProfile {}
#[repr(C, align(16))]
pub struct UTextureRenderTarget2DArray {
    #[doc(hidden)]
    __padding_1248: [u8; 1248],
    pub size_x: i32,
    pub size_y: i32,
    pub slices: i32,
    #[doc(hidden)]
    __padding_1277: [u8; 17],
    pub flags_1277: u8,
    __padding_end: [u8; 2],
}
impl UTextureRenderTarget2DArray {}
#[repr(C, align(16))]
pub struct UTextureRenderTargetCube {
    #[doc(hidden)]
    __padding_1248: [u8; 1248],
    pub size_x: i32,
    #[doc(hidden)]
    __padding_1269: [u8; 17],
    pub flags_1269: u8,
    pub mips_sampler_filter: TextureFilter,
    __padding_end: [u8; 9],
}
impl UTextureRenderTargetCube {}
#[repr(C, align(16))]
pub struct UTextureRenderTargetVolume {
    #[doc(hidden)]
    __padding_1248: [u8; 1248],
    pub size_x: i32,
    pub size_y: i32,
    pub size_z: i32,
    #[doc(hidden)]
    __padding_1277: [u8; 17],
    pub flags_1277: u8,
    __padding_end: [u8; 2],
}
impl UTextureRenderTargetVolume {}
#[repr(C, align(8))]
pub struct UTimelineTemplate {
    #[doc(hidden)]
    __padding_53: [u8; 53],
    pub flags_53: u8,
    __padding_end: [u8; 170],
}
impl UTimelineTemplate {}
#[repr(C, align(8))]
pub struct ATriggerVolume {
    __padding_end: [u8; 1208],
}
impl ATriggerVolume {}
#[repr(C, align(8))]
pub struct UUniversalObjectLocatorScriptingExtensions {
    __padding_end: [u8; 48],
}
impl UUniversalObjectLocatorScriptingExtensions {}
#[repr(C, align(8))]
pub struct UUserDefinedEnum {
    __padding_end: [u8; 232],
}
impl UUserDefinedEnum {}
#[repr(C, align(8))]
pub struct UUserInterfaceSettings {
    __padding_end: [u8; 808],
}
impl UUserInterfaceSettings {}
#[repr(C, align(16))]
pub struct UCanvas {
    __padding_end: [u8; 944],
}
impl UCanvas {}
#[repr(C, align(8))]
pub struct UConsole {
    __padding_end: [u8; 368],
}
impl UConsole {}
#[repr(C, align(8))]
pub struct UInputSettings {
    __padding_end: [u8; 432],
}
impl UInputSettings {}
#[repr(C, align(8))]
pub struct UInputPlatformSettings {
    __padding_end: [u8; 128],
}
impl UInputPlatformSettings {}
#[repr(C, align(8))]
pub struct UPlayerInput {
    __padding_end: [u8; 1184],
}
impl UPlayerInput {}
#[repr(C, align(8))]
pub struct AVectorFieldVolume {
    #[doc(hidden)]
    __padding_1136: [u8; 1136],
    pub vector_field_component: UPtr<UVectorFieldComponent>,
    __padding_end: [u8; 8],
}
impl AVectorFieldVolume {}
#[repr(C, align(8))]
pub struct UTireType {
    __padding_end: [u8; 64],
}
impl UTireType {}
#[repr(C, align(8))]
pub struct UVirtualTextureCollection {
    __padding_end: [u8; 96],
}
impl UVirtualTextureCollection {}
#[repr(C, align(8))]
pub struct UVisualLoggerAutomationTests {
    __padding_end: [u8; 48],
}
impl UVisualLoggerAutomationTests {}
#[repr(C, align(8))]
pub struct AVisualLoggerFilterVolume {
    __padding_end: [u8; 1208],
}
impl AVisualLoggerFilterVolume {}
#[repr(C, align(8))]
pub struct UVisualLoggerKismetLibrary {
    __padding_end: [u8; 48],
}
impl UVisualLoggerKismetLibrary {}
#[repr(C, align(8))]
pub struct UVoiceChannel {
    __padding_end: [u8; 136],
}
impl UVoiceChannel {}
#[repr(C, align(8))]
pub struct UVOIPTalker {
    #[doc(hidden)]
    __padding_240: [u8; 240],
    pub settings: FVoiceSettings,
    __padding_end: [u8; 40],
}
impl UVOIPTalker {}
#[repr(C, align(8))]
pub struct UVOIPStatics {
    __padding_end: [u8; 48],
}
impl UVOIPStatics {}
#[repr(C, align(16))]
pub struct UVolumeTexture {
    #[doc(hidden)]
    __padding_1360: [u8; 1360],
    pub address_mode: TextureAddress,
    __padding_end: [u8; 15],
}
impl UVolumeTexture {}
#[repr(C, align(8))]
pub struct AVolumetricLightmapDensityVolume {
    __padding_end: [u8; 1216],
}
impl AVolumetricLightmapDensityVolume {}
#[repr(C, align(16))]
pub struct ULightMapVirtualTexture2D {
    __padding_end: [u8; 1392],
}
impl ULightMapVirtualTexture2D {}
#[repr(C, align(16))]
pub struct UMeshPaintVirtualTexture {
    __padding_end: [u8; 1376],
}
impl UMeshPaintVirtualTexture {}
#[repr(C, align(16))]
pub struct URuntimeVirtualTexture {
    #[doc(hidden)]
    __padding_48: [u8; 48],
    pub tile_count: i32,
    pub tile_size: i32,
    pub tile_border_size: i32,
    pub material_type: ERuntimeVirtualTextureMaterialType,
    pub b_compress_textures: bool,
    pub b_use_low_quality_compression: bool,
    #[doc(hidden)]
    __padding_80: [u8; 16],
    pub custom_material_data: crate::bindings::core_u_object::FVector4f,
    pub lod_group: TextureGroup,
    __padding_end: [u8; 191],
}
impl URuntimeVirtualTexture {}
#[repr(C, align(8))]
pub struct ARuntimeVirtualTextureVolume {
    #[doc(hidden)]
    __padding_1136: [u8; 1136],
    pub virtual_texture_component: UPtr<URuntimeVirtualTextureComponent>,
    __padding_end: [u8; 8],
}
impl ARuntimeVirtualTextureVolume {}
#[repr(C, align(8))]
pub struct UVirtualTexture {
    __padding_end: [u8; 48],
}
impl UVirtualTexture {}
#[repr(C, align(8))]
pub struct ULightMapVirtualTexture {
    __padding_end: [u8; 48],
}
impl ULightMapVirtualTexture {}
#[repr(C, align(16))]
pub struct URuntimeVirtualTextureStreamingProxy {
    __padding_end: [u8; 1360],
}
impl URuntimeVirtualTextureStreamingProxy {}
#[repr(C, align(16))]
pub struct UVirtualTexture2D {
    __padding_end: [u8; 1376],
}
impl UVirtualTexture2D {}
#[repr(C, align(16))]
pub struct UVirtualTextureAdapter {
    __padding_end: [u8; 1280],
}
impl UVirtualTextureAdapter {}
#[repr(C, align(8))]
pub struct UVirtualTextureBuilder {
    #[doc(hidden)]
    __padding_48: [u8; 48],
    pub texture: UPtr<UVirtualTexture2D>,
    pub texture_mobile: UPtr<UVirtualTexture2D>,
    __padding_end: [u8; 16],
}
impl UVirtualTextureBuilder {}
#[repr(C, align(8))]
pub struct UVirtualTexturePoolConfig {
    __padding_end: [u8; 144],
}
impl UVirtualTexturePoolConfig {}
#[repr(C, align(8))]
pub struct AWindDirectionalSource {
    #[doc(hidden)]
    __padding_1144: [u8; 1144],
    pub component: UPtr<UWindDirectionalSourceComponent>,
    __padding_end: [u8; 8],
}
impl AWindDirectionalSource {}
#[repr(C, align(8))]
pub struct UWorldComposition {
    __padding_end: [u8; 120],
}
impl UWorldComposition {}
#[repr(C, align(8))]
pub struct UWorldPartitionBlueprintLibrary {
    __padding_end: [u8; 48],
}
impl UWorldPartitionBlueprintLibrary {}
#[repr(C, align(8))]
pub struct UActorDescContainer {
    __padding_end: [u8; 512],
}
impl UActorDescContainer {}
#[repr(C, align(16))]
pub struct UActorDescContainerInstance {
    __padding_end: [u8; 688],
}
impl UActorDescContainerInstance {}
#[repr(C, align(8))]
pub struct UActorDescContainerSubsystem {
    __padding_end: [u8; 288],
}
impl UActorDescContainerSubsystem {}
#[repr(C, align(8))]
pub struct UContentBundleDescriptor {
    __padding_end: [u8; 88],
}
impl UContentBundleDescriptor {}
#[repr(C, align(8))]
pub struct UContentBundleUnsavedActorMonitor {
    __padding_end: [u8; 72],
}
impl UContentBundleUnsavedActorMonitor {}
#[repr(C, align(8))]
pub struct UContentBundleEngineSubsystem {
    __padding_end: [u8; 272],
}
impl UContentBundleEngineSubsystem {}
#[repr(C, align(8))]
pub struct UContentBundleTypeFactory {
    __padding_end: [u8; 48],
}
impl UContentBundleTypeFactory {}
#[repr(C, align(8))]
pub struct UContentBundleManager {
    __padding_end: [u8; 72],
}
impl UContentBundleManager {}
#[repr(C, align(8))]
pub struct UContentBundleDuplicateForPIEHelper {
    __padding_end: [u8; 128],
}
impl UContentBundleDuplicateForPIEHelper {}
pub struct IWorldPartitionCookPackageObject {}
#[repr(C, align(8))]
pub struct UWorldPartitionCookPackageObject {
    __padding_end: [u8; 48],
}
impl UWorldPartitionCookPackageObject {}
#[repr(C, align(8))]
pub struct AWorldDataLayers {
    __padding_end: [u8; 2552],
}
impl AWorldDataLayers {}
#[repr(C, align(8))]
pub struct UDEPRECATED_DataLayer {
    __padding_end: [u8; 104],
}
impl UDEPRECATED_DataLayer {}
#[repr(C, align(8))]
pub struct UDataLayerAsset {
    __padding_end: [u8; 72],
}
impl UDataLayerAsset {}
#[repr(C, align(8))]
pub struct UDataLayerInstance {
    __padding_end: [u8; 88],
}
impl UDataLayerInstance {}
#[repr(C, align(8))]
pub struct UDataLayerInstancePrivate {
    __padding_end: [u8; 120],
}
impl UDataLayerInstancePrivate {}
pub struct IDataLayerInstanceProvider {}
#[repr(C, align(8))]
pub struct UDataLayerInstanceProvider {
    __padding_end: [u8; 48],
}
impl UDataLayerInstanceProvider {}
#[repr(C, align(8))]
pub struct UDataLayerInstanceWithAsset {
    __padding_end: [u8; 120],
}
impl UDataLayerInstanceWithAsset {}
#[repr(C, align(16))]
pub struct UWorldPartition {
    __padding_end: [u8; 1184],
}
impl UWorldPartition {
    pub fn verify_layout() {}
}
#[repr(C, align(8))]
pub struct UDataLayerManager {
    __padding_end: [u8; 280],
}
impl UDataLayerManager {}
#[repr(C, align(8))]
pub struct UDataLayerLoadingPolicy {
    __padding_end: [u8; 48],
}
impl UDataLayerLoadingPolicy {}
#[repr(C, align(8))]
pub struct UDataLayerSubsystem {
    __padding_end: [u8; 136],
}
impl UDataLayerSubsystem {}
#[repr(C, align(8))]
pub struct UDeprecatedDataLayerInstance {
    __padding_end: [u8; 120],
}
impl UDeprecatedDataLayerInstance {}
#[repr(C, align(8))]
pub struct UExternalDataLayerAsset {
    __padding_end: [u8; 80],
}
impl UExternalDataLayerAsset {}
#[repr(C, align(8))]
pub struct UExternalDataLayerEngineSubsystem {
    __padding_end: [u8; 400],
}
impl UExternalDataLayerEngineSubsystem {}
#[repr(C, align(8))]
pub struct UExternalDataLayerInjectionPolicy {
    __padding_end: [u8; 48],
}
impl UExternalDataLayerInjectionPolicy {}
#[repr(C, align(8))]
pub struct UExternalDataLayerInstance {
    __padding_end: [u8; 120],
}
impl UExternalDataLayerInstance {}
#[repr(C, align(8))]
pub struct UExternalDataLayerManager {
    __padding_end: [u8; 456],
}
impl UExternalDataLayerManager {}
#[repr(C, align(8))]
pub struct AWorldPartitionCustomHLOD {
    #[doc(hidden)]
    __padding_1144: [u8; 1144],
    pub static_mesh_component: UPtr<UStaticMeshComponent>,
    __padding_end: [u8; 16],
}
impl AWorldPartitionCustomHLOD {}
#[repr(C, align(8))]
pub struct AWorldPartitionCustomHLODPlaceholder {
    __padding_end: [u8; 1160],
}
impl AWorldPartitionCustomHLODPlaceholder {}
#[repr(C, align(16))]
pub struct UWorldPartitionDestructibleHLODComponent {
    __padding_end: [u8; 1152],
}
impl UWorldPartitionDestructibleHLODComponent {}
#[repr(C, align(16))]
pub struct UDEPRECATED_UWorldPartitionDestructibleHLODMeshComponent {
    __padding_end: [u8; 1152],
}
impl UDEPRECATED_UWorldPartitionDestructibleHLODMeshComponent {}
#[repr(C, align(8))]
pub struct AWorldPartitionHLOD {
    __padding_end: [u8; 1528],
}
impl AWorldPartitionHLOD {}
#[repr(C, align(8))]
pub struct UHLODBuilderSettings {
    __padding_end: [u8; 48],
}
impl UHLODBuilderSettings {}
#[repr(C, align(8))]
pub struct UNullHLODBuilder {
    __padding_end: [u8; 72],
}
impl UNullHLODBuilder {}
pub struct IWorldPartitionDestructibleInHLODInterface {}
#[repr(C, align(8))]
pub struct UWorldPartitionDestructibleInHLODInterface {
    __padding_end: [u8; 48],
}
impl UWorldPartitionDestructibleInHLODInterface {}
#[repr(C, align(8))]
pub struct UWorldPartitionDestructibleInHLODSupportLibrary {
    __padding_end: [u8; 48],
}
impl UWorldPartitionDestructibleInHLODSupportLibrary {}
#[repr(C, align(16))]
pub struct UHLODInstancedSkinnedMeshComponent {
    __padding_end: [u8; 3024],
}
impl UHLODInstancedSkinnedMeshComponent {}
#[repr(C, align(16))]
pub struct UHLODInstancedStaticMeshComponent {
    __padding_end: [u8; 2880],
}
impl UHLODInstancedStaticMeshComponent {}
#[repr(C, align(8))]
pub struct UHLODLayer {
    __padding_end: [u8; 1128],
}
impl UHLODLayer {}
#[repr(C, align(8))]
pub struct UWorldPartitionHLODModifier {
    __padding_end: [u8; 48],
}
impl UWorldPartitionHLODModifier {}
#[repr(C, align(8))]
pub struct AWorldPartitionHLODOnlyLevelInstance {
    __padding_end: [u8; 1416],
}
impl AWorldPartitionHLODOnlyLevelInstance {}
pub struct IWorldPartitionHLODProvider {}
#[repr(C, align(8))]
pub struct UWorldPartitionHLODProvider {
    __padding_end: [u8; 48],
}
impl UWorldPartitionHLODProvider {}
#[repr(C, align(8))]
pub struct UWorldPartitionHLODRuntimeSubsystem {
    __padding_end: [u8; 728],
}
impl UWorldPartitionHLODRuntimeSubsystem {}
#[repr(C, align(8))]
pub struct UWorldPartitionHLODSourceActors {
    __padding_end: [u8; 56],
}
impl UWorldPartitionHLODSourceActors {}
#[repr(C, align(8))]
pub struct UWorldPartitionHLODSourceActorsFromCell {
    __padding_end: [u8; 72],
}
impl UWorldPartitionHLODSourceActorsFromCell {}
#[repr(C, align(8))]
pub struct UWorldPartitionHLODSourceActorsFromLevel {
    __padding_end: [u8; 104],
}
impl UWorldPartitionHLODSourceActorsFromLevel {}
#[repr(C, align(8))]
pub struct UMaterialParameterCollectionHLODModifier {
    __padding_end: [u8; 96],
}
impl UMaterialParameterCollectionHLODModifier {}
#[repr(C, align(8))]
pub struct AWorldPartitionStandaloneHLOD {
    __padding_end: [u8; 1416],
}
impl AWorldPartitionStandaloneHLOD {}
#[repr(C, align(8))]
pub struct UWorldPartitionStandaloneHLODSubsystem {
    __padding_end: [u8; 232],
}
impl UWorldPartitionStandaloneHLODSubsystem {}
#[repr(C, align(16))]
pub struct ULevelInstanceContainerInstance {
    __padding_end: [u8; 800],
}
impl ULevelInstanceContainerInstance {}
#[repr(C, align(8))]
pub struct ULevelInstancePropertyOverrideContainer {
    __padding_end: [u8; 528],
}
impl ULevelInstancePropertyOverrideContainer {}
#[repr(C, align(8))]
pub struct ANavigationDataChunkActor {
    __padding_end: [u8; 1216],
}
impl ANavigationDataChunkActor {}
#[repr(C, align(8))]
pub struct URuntimePartition {
    __padding_end: [u8; 96],
}
impl URuntimePartition {}
#[repr(C, align(8))]
pub struct URuntimePartitionLevelStreaming {
    __padding_end: [u8; 96],
}
impl URuntimePartitionLevelStreaming {}
#[repr(C, align(8))]
pub struct URuntimePartitionLHGrid {
    __padding_end: [u8; 144],
}
impl URuntimePartitionLHGrid {}
#[repr(C, align(8))]
pub struct URuntimePartitionPersistent {
    __padding_end: [u8; 96],
}
impl URuntimePartitionPersistent {}
#[repr(C, align(8))]
pub struct UWorldPartitionRuntimeCell {
    __padding_end: [u8; 216],
}
impl UWorldPartitionRuntimeCell {}
#[repr(C, align(8))]
pub struct UWorldPartitionRuntimeCellData {
    __padding_end: [u8; 264],
}
impl UWorldPartitionRuntimeCellData {}
#[repr(C, align(8))]
pub struct UWorldPartitionRuntimeCellDataHashSet {
    __padding_end: [u8; 272],
}
impl UWorldPartitionRuntimeCellDataHashSet {}
#[repr(C, align(8))]
pub struct URuntimeHashExternalStreamingObjectBase {
    __padding_end: [u8; 664],
}
impl URuntimeHashExternalStreamingObjectBase {}
#[repr(C, align(8))]
pub struct URuntimeHashSetExternalStreamingObject {
    __padding_end: [u8; 680],
}
impl URuntimeHashSetExternalStreamingObject {}
#[repr(C, align(8))]
pub struct UWorldPartitionRuntimeHash {
    __padding_end: [u8; 224],
}
impl UWorldPartitionRuntimeHash {}
#[repr(C, align(8))]
pub struct UWorldPartitionRuntimeHashSet {
    __padding_end: [u8; 592],
}
impl UWorldPartitionRuntimeHashSet {}
#[repr(C, align(8))]
pub struct AMapBuildDataActor {
    __padding_end: [u8; 1264],
}
impl AMapBuildDataActor {}
pub struct IWorldPartitionActorLoaderInterface {}
#[repr(C, align(8))]
pub struct UWorldPartitionActorLoaderInterface {
    __padding_end: [u8; 48],
}
impl UWorldPartitionActorLoaderInterface {}
#[repr(C, align(8))]
pub struct UWorldPartitionEditorHash {
    __padding_end: [u8; 48],
}
impl UWorldPartitionEditorHash {}
#[repr(C, align(8))]
pub struct UWorldPartitionEditorPerProjectUserSettings {
    __padding_end: [u8; 192],
}
impl UWorldPartitionEditorPerProjectUserSettings {}
#[repr(C, align(8))]
pub struct UWorldPartitionEditorSpatialHash {
    __padding_end: [u8; 2088],
}
impl UWorldPartitionEditorSpatialHash {}
#[repr(C, align(16))]
pub struct UWorldPartitionLevelStreamingDynamic {
    __padding_end: [u8; 736],
}
impl UWorldPartitionLevelStreamingDynamic {}
#[repr(C, align(8))]
pub struct UWorldPartitionStreamingPolicy {
    __padding_end: [u8; 1112],
}
impl UWorldPartitionStreamingPolicy {}
#[repr(C, align(8))]
pub struct UWorldPartitionLevelStreamingPolicy {
    __padding_end: [u8; 1488],
}
impl UWorldPartitionLevelStreamingPolicy {}
#[repr(C, align(8))]
pub struct AWorldPartitionMiniMap {
    __padding_end: [u8; 1352],
}
impl AWorldPartitionMiniMap {}
#[repr(C, align(8))]
pub struct AWorldPartitionMiniMapVolume {
    __padding_end: [u8; 1208],
}
impl AWorldPartitionMiniMapVolume {}
#[repr(C, align(8))]
pub struct AWorldPartitionReplay {
    __padding_end: [u8; 1168],
}
impl AWorldPartitionReplay {}
#[repr(C, align(8))]
pub struct UWorldPartitionRuntimeCellDataSpatialHash {
    __padding_end: [u8; 384],
}
impl UWorldPartitionRuntimeCellDataSpatialHash {}
pub struct IWorldPartitionCell {}
#[repr(C, align(8))]
pub struct UWorldPartitionCell {
    __padding_end: [u8; 48],
}
impl UWorldPartitionCell {}
#[repr(C, align(8))]
pub struct UWorldPartitionRuntimeCellTransformerSettings {
    __padding_end: [u8; 80],
}
impl UWorldPartitionRuntimeCellTransformerSettings {}
#[repr(C, align(8))]
pub struct UWorldPartitionRuntimeCellTransformer {
    __padding_end: [u8; 56],
}
impl UWorldPartitionRuntimeCellTransformer {}
#[repr(C, align(8))]
pub struct UWorldPartitionRuntimeCellTransformerISM {
    __padding_end: [u8; 96],
}
impl UWorldPartitionRuntimeCellTransformerISM {}
#[repr(C, align(8))]
pub struct AWorldPartitionAutoInstancedActor {
    __padding_end: [u8; 1136],
}
impl AWorldPartitionAutoInstancedActor {}
#[repr(C, align(8))]
pub struct UWorldPartitionRuntimeCellTransformerLog {
    __padding_end: [u8; 144],
}
impl UWorldPartitionRuntimeCellTransformerLog {}
#[repr(C, align(8))]
pub struct UWorldPartitionRuntimeLevelStreamingCell {
    __padding_end: [u8; 320],
}
impl UWorldPartitionRuntimeLevelStreamingCell {}
#[repr(C, align(8))]
pub struct ASpatialHashRuntimeGridInfo {
    __padding_end: [u8; 1216],
}
impl ASpatialHashRuntimeGridInfo {}
#[repr(C, align(8))]
pub struct URuntimeSpatialHashExternalStreamingObject {
    __padding_end: [u8; 680],
}
impl URuntimeSpatialHashExternalStreamingObject {}
#[repr(C, align(8))]
pub struct UWorldPartitionRuntimeSpatialHash {
    __padding_end: [u8; 704],
}
impl UWorldPartitionRuntimeSpatialHash {}
#[repr(C, align(8))]
pub struct UWorldPartitionSettings {
    __padding_end: [u8; 224],
}
impl UWorldPartitionSettings {}
#[repr(C, align(8))]
pub struct UWorldPartitionSubsystem {
    __padding_end: [u8; 872],
}
impl UWorldPartitionSubsystem {}
#[repr(C, align(8))]
pub struct ADEPRECATED_WorldPartitionVolume {
    __padding_end: [u8; 1208],
}
impl ADEPRECATED_WorldPartitionVolume {}
#[repr(C, align(16))]
pub struct AWorldSettings {
    #[doc(hidden)]
    __padding_1157: [u8; 1157],
    pub flags_1157: u8,
    pub flags_1158: u8,
    pub ai_system_class: TSoftObjectPtr<crate::bindings::core_u_object::UClass>,
    #[doc(hidden)]
    __padding_1232: [u8; 24],
    pub navigation_system_config: UPtr<UNavigationSystemConfig>,
    #[doc(hidden)]
    __padding_1304: [u8; 64],
    pub world_to_meters: f32,
    pub kill_z: f32,
    pub kill_z_damage_type: TSubclassOf<UDamageType>,
    #[doc(hidden)]
    __padding_1324: [u8; 4],
    pub global_gravity_z: f32,
    pub default_physics_volume_class: TSubclassOf<ADefaultPhysicsVolume>,
    pub physics_collision_handler_class: TSubclassOf<UPhysicsCollisionHandler>,
    pub default_game_mode: TSubclassOf<AGameModeBase>,
    #[doc(hidden)]
    __padding_1368: [u8; 16],
    pub default_color_scale: crate::bindings::core_u_object::FVector,
    #[doc(hidden)]
    __padding_1404: [u8; 12],
    pub lightmass_settings: FLightmassWorldInfoSettings,
    pub volumetric_lightmap_loading_range: f32,
    __padding_end: [u8; 732],
}
impl AWorldSettings {}
#[repr(C, align(8))]
pub struct FDelegateArray_Delegates {
    _opague: [u8; 32],
}
#[repr(C, align(8))]
pub struct FTimelineEventEntry_EventFunc {
    _opague: [u8; 32],
}
#[repr(C, align(8))]
pub struct FTimelineVectorTrack_InterpFunc {
    _opague: [u8; 32],
}
#[repr(C, align(8))]
pub struct FTimelineFloatTrack_InterpFunc {
    _opague: [u8; 32],
}
#[repr(C, align(8))]
pub struct FTimelineLinearColorTrack_InterpFunc {
    _opague: [u8; 32],
}
#[repr(C, align(8))]
pub struct FTimeline_EventFunc {
    _opague: [u8; 32],
}
#[repr(C, align(8))]
pub struct FTimeline_InterpFunc {
    _opague: [u8; 32],
}
#[repr(C, align(8))]
pub struct FTimeline_TimelinePostUpdateFunc {
    _opague: [u8; 32],
}
#[repr(C, align(8))]
pub struct FTimeline_TimelineFinishedFunc {
    _opague: [u8; 32],
}
#[repr(C, align(8))]
pub struct FK2_ClearTimerDelegate_Delegate {
    _opague: [u8; 32],
}
#[repr(C, align(8))]
pub struct FK2_GetTimerElapsedTimeDelegate_Delegate {
    _opague: [u8; 32],
}
#[repr(C, align(8))]
pub struct FK2_GetTimerRemainingTimeDelegate_Delegate {
    _opague: [u8; 32],
}
#[repr(C, align(8))]
pub struct FK2_IsTimerActiveDelegate_Delegate {
    _opague: [u8; 32],
}
#[repr(C, align(8))]
pub struct FK2_IsTimerPausedDelegate_Delegate {
    _opague: [u8; 32],
}
#[repr(C, align(8))]
pub struct FK2_PauseTimerDelegate_Delegate {
    _opague: [u8; 32],
}
#[repr(C, align(8))]
pub struct FK2_SetTimerDelegate_Delegate {
    _opague: [u8; 32],
}
#[repr(C, align(8))]
pub struct FK2_SetTimerForNextTickDelegate_Delegate {
    _opague: [u8; 32],
}
#[repr(C, align(8))]
pub struct FK2_TimerExistsDelegate_Delegate {
    _opague: [u8; 32],
}
#[repr(C, align(8))]
pub struct FK2_UnPauseTimerDelegate_Delegate {
    _opague: [u8; 32],
}
#[repr(C, align(8))]
pub struct FLoadAsset_OnLoaded {
    _opague: [u8; 32],
}
#[repr(C, align(8))]
pub struct FLoadAssetClass_OnLoaded {
    _opague: [u8; 32],
}
#[repr(C, align(8))]
pub struct FLoadAssets_OnLoaded {
    _opague: [u8; 32],
}
#[repr(C, align(8))]
pub struct FAddEvent_EventFunc {
    _opague: [u8; 32],
}
#[repr(C, align(8))]
pub struct FAddInterpFloat_InterpFunc {
    _opague: [u8; 32],
}
#[repr(C, align(8))]
pub struct FAddInterpLinearColor_InterpFunc {
    _opague: [u8; 32],
}
#[repr(C, align(8))]
pub struct FAddInterpVector_InterpFunc {
    _opague: [u8; 32],
}
#[repr(C, align(8))]
pub struct FOnRep_Timeline_EventFunc {
    _opague: [u8; 32],
}
#[repr(C, align(8))]
pub struct FSetTimelineFinishedFunc_NewTimelineFinishedFunc {
    _opague: [u8; 32],
}
#[repr(C, align(8))]
pub struct FSetTimelinePostUpdateFunc_NewTimelinePostUpdateFunc {
    _opague: [u8; 32],
}
#[repr(C, align(8))]
pub struct FPlayQuantized_InDelegate {
    _opague: [u8; 32],
}
#[repr(C, align(8))]
pub struct FAddDisplayDelegate_Delegate {
    _opague: [u8; 32],
}
#[repr(C, align(8))]
pub struct FAsyncLoadOrCreateSaveGameForLocalPlayer_Delegate {
    _opague: [u8; 32],
}
#[repr(C, align(8))]
pub struct FAddEnvelopeFollowerDelegate_OnSubmixEnvelopeBP {
    _opague: [u8; 32],
}
#[repr(C, align(8))]
pub struct FAddSpectralAnalysisDelegate_OnSubmixSpectralAnalysisBP {
    _opague: [u8; 32],
}
#[repr(C, align(8))]
pub struct FRemoveEnvelopeFollowerDelegate_OnSubmixEnvelopeBP {
    _opague: [u8; 32],
}
#[repr(C, align(8))]
pub struct FRemoveSpectralAnalysisDelegate_OnSubmixSpectralAnalysisBP {
    _opague: [u8; 32],
}
#[repr(C, align(1))]
pub struct FActor_OnTakeAnyDamage {
    _opague: [u8; 1],
}
#[repr(C, align(1))]
pub struct FActor_OnTakePointDamage {
    _opague: [u8; 1],
}
#[repr(C, align(1))]
pub struct FActor_OnTakeRadialDamage {
    _opague: [u8; 1],
}
#[repr(C, align(1))]
pub struct FActor_OnActorBeginOverlap {
    _opague: [u8; 1],
}
#[repr(C, align(1))]
pub struct FActor_OnActorEndOverlap {
    _opague: [u8; 1],
}
#[repr(C, align(1))]
pub struct FActor_OnBeginCursorOver {
    _opague: [u8; 1],
}
#[repr(C, align(1))]
pub struct FActor_OnEndCursorOver {
    _opague: [u8; 1],
}
#[repr(C, align(1))]
pub struct FActor_OnClicked {
    _opague: [u8; 1],
}
#[repr(C, align(1))]
pub struct FActor_OnReleased {
    _opague: [u8; 1],
}
#[repr(C, align(1))]
pub struct FActor_OnInputTouchBegin {
    _opague: [u8; 1],
}
#[repr(C, align(1))]
pub struct FActor_OnInputTouchEnd {
    _opague: [u8; 1],
}
#[repr(C, align(1))]
pub struct FActor_OnInputTouchEnter {
    _opague: [u8; 1],
}
#[repr(C, align(1))]
pub struct FActor_OnInputTouchLeave {
    _opague: [u8; 1],
}
#[repr(C, align(1))]
pub struct FActor_OnActorHit {
    _opague: [u8; 1],
}
#[repr(C, align(1))]
pub struct FActor_OnDestroyed {
    _opague: [u8; 1],
}
#[repr(C, align(1))]
pub struct FActor_OnEndPlay {
    _opague: [u8; 1],
}
#[repr(C, align(1))]
pub struct FActorComponent_OnComponentActivated {
    _opague: [u8; 1],
}
#[repr(C, align(1))]
pub struct FActorComponent_OnComponentDeactivated {
    _opague: [u8; 1],
}
#[repr(C, align(1))]
pub struct FSceneComponent_PhysicsVolumeChangedDelegate {
    _opague: [u8; 1],
}
#[repr(C, align(1))]
pub struct FPrimitiveComponent_OnComponentHit {
    _opague: [u8; 1],
}
#[repr(C, align(1))]
pub struct FPrimitiveComponent_OnComponentBeginOverlap {
    _opague: [u8; 1],
}
#[repr(C, align(1))]
pub struct FPrimitiveComponent_OnComponentEndOverlap {
    _opague: [u8; 1],
}
#[repr(C, align(1))]
pub struct FPrimitiveComponent_OnComponentWake {
    _opague: [u8; 1],
}
#[repr(C, align(1))]
pub struct FPrimitiveComponent_OnComponentSleep {
    _opague: [u8; 1],
}
#[repr(C, align(1))]
pub struct FPrimitiveComponent_OnComponentPhysicsStateChanged {
    _opague: [u8; 1],
}
#[repr(C, align(1))]
pub struct FPrimitiveComponent_OnBeginCursorOver {
    _opague: [u8; 1],
}
#[repr(C, align(1))]
pub struct FPrimitiveComponent_OnEndCursorOver {
    _opague: [u8; 1],
}
#[repr(C, align(1))]
pub struct FPrimitiveComponent_OnClicked {
    _opague: [u8; 1],
}
#[repr(C, align(1))]
pub struct FPrimitiveComponent_OnReleased {
    _opague: [u8; 1],
}
#[repr(C, align(1))]
pub struct FPrimitiveComponent_OnInputTouchBegin {
    _opague: [u8; 1],
}
#[repr(C, align(1))]
pub struct FPrimitiveComponent_OnInputTouchEnd {
    _opague: [u8; 1],
}
#[repr(C, align(1))]
pub struct FPrimitiveComponent_OnInputTouchEnter {
    _opague: [u8; 1],
}
#[repr(C, align(1))]
pub struct FPrimitiveComponent_OnInputTouchLeave {
    _opague: [u8; 1],
}
#[repr(C, align(8))]
pub struct FController_OnInstigatedAnyDamage {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FController_OnPossessedPawnChanged {
    _opague: [u8; 24],
}
#[repr(C, align(1))]
pub struct FPawn_ReceiveControllerChangedDelegate {
    _opague: [u8; 1],
}
#[repr(C, align(1))]
pub struct FPawn_ReceiveRestartedDelegate {
    _opague: [u8; 1],
}
#[repr(C, align(8))]
pub struct FCharacter_OnReachedJumpApex {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FCharacter_LandedDelegate {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FCharacter_MovementModeChangedDelegate {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FCharacter_OnCharacterMovementUpdated {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FGameInstance_OnPawnControllerChangedDelegates {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FGameInstance_OnInputDeviceConnectionChange {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FGameInstance_OnUserInputDevicePairingChange {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FSkeletalMeshComponent_OnConstraintBroken {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FSkeletalMeshComponent_OnPlasticDeformation {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FSkeletalMeshComponent_OnAnimInitialized {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FAnimInstance_OnMontageBlendingOut {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FAnimInstance_OnMontageBlendedIn {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FAnimInstance_OnMontageStarted {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FAnimInstance_OnMontageEnded {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FAnimInstance_OnAllMontageInstancesEnded {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FAnimInstance_OnMontageSectionChanged {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FAnimSingleNodeInstance_PostEvaluateAnimEvent {
    _opague: [u8; 32],
}
#[repr(C, align(8))]
pub struct FPlatformInterfaceBase_Delegates {
    _opague: [u8; 32],
}
#[repr(C, align(8))]
pub struct FEmitter_OnParticleSpawn {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FEmitter_OnParticleBurst {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FEmitter_OnParticleDeath {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FEmitter_OnParticleCollide {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FInGameAdManager_ClickedBannerDelegates {
    _opague: [u8; 32],
}
#[repr(C, align(8))]
pub struct FInGameAdManager_ClosedAdDelegates {
    _opague: [u8; 32],
}
#[repr(C, align(8))]
pub struct FLevelStreaming_OnLevelLoaded {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FLevelStreaming_OnLevelUnloaded {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FLevelStreaming_OnLevelShown {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FLevelStreaming_OnLevelHidden {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FTimelineComponent_EventFunc {
    _opague: [u8; 32],
}
#[repr(C, align(8))]
pub struct FAnimDataModel_ModifiedEventDynamic {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FAsyncActionLoadPrimaryAsset_Completed {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FAsyncActionLoadPrimaryAssetClass_Completed {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FAsyncActionLoadPrimaryAssetList_Completed {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FAsyncActionLoadPrimaryAssetClassList_Completed {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FAsyncActionChangePrimaryAssetBundles_Completed {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FCanvasRenderTarget2D_OnCanvasRenderTargetUpdate {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FApplicationLifecycleComponent_ApplicationWillDeactivateDelegate {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FApplicationLifecycleComponent_ApplicationHasReactivatedDelegate {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FApplicationLifecycleComponent_ApplicationWillEnterBackgroundDelegate {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FApplicationLifecycleComponent_ApplicationHasEnteredForegroundDelegate {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FApplicationLifecycleComponent_ApplicationWillTerminateDelegate {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FApplicationLifecycleComponent_ApplicationShouldUnloadResourcesDelegate {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FApplicationLifecycleComponent_ApplicationReceivedStartupArgumentsDelegate {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FApplicationLifecycleComponent_OnTemperatureChangeDelegate {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FApplicationLifecycleComponent_OnLowPowerModeDelegate {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FAudioComponent_OnAudioPlayStateChanged {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FAudioComponent_OnAudioVirtualizationChanged {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FAudioComponent_OnAudioFinished {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FAudioComponent_OnAudioPlaybackPercent {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FAudioComponent_OnAudioSingleEnvelopeValue {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FAudioComponent_OnAudioMultiEnvelopeValue {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FAudioComponent_OnQueueSubtitles {
    _opague: [u8; 32],
}
#[repr(C, align(8))]
pub struct FForceFeedbackComponent_OnForceFeedbackFinished {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FInterpToMovementComponent_OnInterpToReverse {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FInterpToMovementComponent_OnInterpToStop {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FInterpToMovementComponent_OnWaitBeginDelegate {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FInterpToMovementComponent_OnWaitEndDelegate {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FInterpToMovementComponent_OnResetDelegate {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FPlatformEventsComponent_PlatformChangedToLaptopModeDelegate {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FPlatformEventsComponent_PlatformChangedToTabletModeDelegate {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FProjectileMovementComponent_OnProjectileBounce {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FProjectileMovementComponent_OnProjectileStop {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FDataDrivenCVarEngineSubsystem_OnDataDrivenCVarDelegate {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FAsyncActionHandleSaveGame_Completed {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FInputDeviceSubsystem_OnInputHardwareDeviceChanged {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FGameUserSettings_OnGameUserSettingsUINeedsUpdate {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FPlatformGameInstance_ApplicationWillDeactivateDelegate {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FPlatformGameInstance_ApplicationHasReactivatedDelegate {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FPlatformGameInstance_ApplicationWillEnterBackgroundDelegate {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FPlatformGameInstance_ApplicationHasEnteredForegroundDelegate {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FPlatformGameInstance_ApplicationWillTerminateDelegate {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FPlatformGameInstance_ApplicationShouldUnloadResourcesDelegate {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FPlatformGameInstance_ApplicationReceivedStartupArgumentsDelegate {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FPlatformGameInstance_ApplicationRegisteredForRemoteNotificationsDelegate {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FPlatformGameInstance_ApplicationRegisteredForUserNotificationsDelegate {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FPlatformGameInstance_ApplicationFailedToRegisterForRemoteNotificationsDelegate {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FPlatformGameInstance_ApplicationReceivedRemoteNotificationDelegate {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FPlatformGameInstance_ApplicationReceivedLocalNotificationDelegate {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FPlatformGameInstance_ApplicationReceivedScreenOrientationChangedNotificationDelegate {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FParticleSystemComponent_OnParticleSpawn {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FParticleSystemComponent_OnParticleBurst {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FParticleSystemComponent_OnParticleDeath {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FParticleSystemComponent_OnParticleCollide {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FParticleSystemComponent_OnSystemFinished {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FClusterUnionComponent_OnComponentAddedEvent {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FClusterUnionComponent_OnComponentRemovedEvent {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FClusterUnionComponent_OnComponentBoundsChangedEvent {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FChaosEventRelay_OnCollisionEvent {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FChaosEventRelay_OnBreakEvent {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FChaosEventRelay_OnRemovalEvent {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FChaosEventRelay_OnCrumblingEvent {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FPhysicsConstraintComponent_OnConstraintBroken {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FPhysicsConstraintComponent_OnPlasticDeformation {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FPlayerCameraManager_OnAudioFadeChangeEvent {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FPlayerState_OnPawnSet {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FSoundSubmix_OnSubmixRecordedFileDone {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FDataLayerManager_OnDataLayerInstanceRuntimeStateChanged {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FDataLayerSubsystem_OnDataLayerRuntimeStateChanged {
    _opague: [u8; 24],
}
#[repr(transparent)]
pub struct EFormatArgumentType(pub u8);
impl EFormatArgumentType {
    pub const INT: EFormatArgumentType = EFormatArgumentType(0);
    pub const U_INT: EFormatArgumentType = EFormatArgumentType(1);
    pub const FLOAT: EFormatArgumentType = EFormatArgumentType(2);
    pub const DOUBLE: EFormatArgumentType = EFormatArgumentType(3);
    pub const TEXT: EFormatArgumentType = EFormatArgumentType(4);
    pub const GENDER: EFormatArgumentType = EFormatArgumentType(5);
}
#[repr(transparent)]
pub struct ETextGender(pub u8);
impl ETextGender {
    pub const MASCULINE: ETextGender = ETextGender(0);
    pub const FEMININE: ETextGender = ETextGender(1);
    pub const NEUTER: ETextGender = ETextGender(2);
}
#[repr(transparent)]
pub struct EPlatformInterfaceDataType(pub u8);
impl EPlatformInterfaceDataType {
    pub const PIDT_NONE: EPlatformInterfaceDataType = EPlatformInterfaceDataType(0);
    pub const PIDT_INT: EPlatformInterfaceDataType = EPlatformInterfaceDataType(1);
    pub const PIDT_FLOAT: EPlatformInterfaceDataType = EPlatformInterfaceDataType(2);
    pub const PIDT_STRING: EPlatformInterfaceDataType = EPlatformInterfaceDataType(3);
    pub const PIDT_OBJECT: EPlatformInterfaceDataType = EPlatformInterfaceDataType(4);
    pub const PIDT_CUSTOM: EPlatformInterfaceDataType = EPlatformInterfaceDataType(5);
}
#[repr(transparent)]
pub struct ETickingGroup(pub u8);
impl ETickingGroup {
    pub const TG_PRE_PHYSICS: ETickingGroup = ETickingGroup(0);
    pub const TG_START_PHYSICS: ETickingGroup = ETickingGroup(1);
    pub const TG_DURING_PHYSICS: ETickingGroup = ETickingGroup(2);
    pub const TG_END_PHYSICS: ETickingGroup = ETickingGroup(3);
    pub const TG_POST_PHYSICS: ETickingGroup = ETickingGroup(4);
    pub const TG_POST_UPDATE_WORK: ETickingGroup = ETickingGroup(5);
    pub const TG_LAST_DEMOTABLE: ETickingGroup = ETickingGroup(6);
    pub const TG_NEWLY_SPAWNED: ETickingGroup = ETickingGroup(7);
}
#[repr(transparent)]
pub struct EWalkableSlopeBehavior(pub u8);
impl EWalkableSlopeBehavior {
    pub const WALKABLE_SLOPE_DEFAULT: EWalkableSlopeBehavior = EWalkableSlopeBehavior(0);
    pub const WALKABLE_SLOPE_INCREASE: EWalkableSlopeBehavior = EWalkableSlopeBehavior(
        1,
    );
    pub const WALKABLE_SLOPE_DECREASE: EWalkableSlopeBehavior = EWalkableSlopeBehavior(
        2,
    );
    pub const WALKABLE_SLOPE_UNWALKABLE: EWalkableSlopeBehavior = EWalkableSlopeBehavior(
        3,
    );
}
#[repr(transparent)]
pub struct ECollisionChannel(pub u8);
impl ECollisionChannel {
    pub const ECC_WORLD_STATIC: ECollisionChannel = ECollisionChannel(0);
    pub const ECC_WORLD_DYNAMIC: ECollisionChannel = ECollisionChannel(1);
    pub const ECC_PAWN: ECollisionChannel = ECollisionChannel(2);
    pub const ECC_VISIBILITY: ECollisionChannel = ECollisionChannel(3);
    pub const ECC_CAMERA: ECollisionChannel = ECollisionChannel(4);
    pub const ECC_PHYSICS_BODY: ECollisionChannel = ECollisionChannel(5);
    pub const ECC_VEHICLE: ECollisionChannel = ECollisionChannel(6);
    pub const ECC_DESTRUCTIBLE: ECollisionChannel = ECollisionChannel(7);
    pub const ECC_ENGINE_TRACE_CHANNEL1: ECollisionChannel = ECollisionChannel(8);
    pub const ECC_ENGINE_TRACE_CHANNEL2: ECollisionChannel = ECollisionChannel(9);
    pub const ECC_ENGINE_TRACE_CHANNEL3: ECollisionChannel = ECollisionChannel(10);
    pub const ECC_ENGINE_TRACE_CHANNEL4: ECollisionChannel = ECollisionChannel(11);
    pub const ECC_ENGINE_TRACE_CHANNEL5: ECollisionChannel = ECollisionChannel(12);
    pub const ECC_ENGINE_TRACE_CHANNEL6: ECollisionChannel = ECollisionChannel(13);
    pub const ECC_GAME_TRACE_CHANNEL1: ECollisionChannel = ECollisionChannel(14);
    pub const ECC_GAME_TRACE_CHANNEL2: ECollisionChannel = ECollisionChannel(15);
    pub const ECC_GAME_TRACE_CHANNEL3: ECollisionChannel = ECollisionChannel(16);
    pub const ECC_GAME_TRACE_CHANNEL4: ECollisionChannel = ECollisionChannel(17);
    pub const ECC_GAME_TRACE_CHANNEL5: ECollisionChannel = ECollisionChannel(18);
    pub const ECC_GAME_TRACE_CHANNEL6: ECollisionChannel = ECollisionChannel(19);
    pub const ECC_GAME_TRACE_CHANNEL7: ECollisionChannel = ECollisionChannel(20);
    pub const ECC_GAME_TRACE_CHANNEL8: ECollisionChannel = ECollisionChannel(21);
    pub const ECC_GAME_TRACE_CHANNEL9: ECollisionChannel = ECollisionChannel(22);
    pub const ECC_GAME_TRACE_CHANNEL10: ECollisionChannel = ECollisionChannel(23);
    pub const ECC_GAME_TRACE_CHANNEL11: ECollisionChannel = ECollisionChannel(24);
    pub const ECC_GAME_TRACE_CHANNEL12: ECollisionChannel = ECollisionChannel(25);
    pub const ECC_GAME_TRACE_CHANNEL13: ECollisionChannel = ECollisionChannel(26);
    pub const ECC_GAME_TRACE_CHANNEL14: ECollisionChannel = ECollisionChannel(27);
    pub const ECC_GAME_TRACE_CHANNEL15: ECollisionChannel = ECollisionChannel(28);
    pub const ECC_GAME_TRACE_CHANNEL16: ECollisionChannel = ECollisionChannel(29);
    pub const ECC_GAME_TRACE_CHANNEL17: ECollisionChannel = ECollisionChannel(30);
    pub const ECC_GAME_TRACE_CHANNEL18: ECollisionChannel = ECollisionChannel(31);
    pub const ECC_OVERLAP_ALL_DEPRECATED: ECollisionChannel = ECollisionChannel(32);
}
#[repr(transparent)]
pub struct ECollisionEnabled(pub u8);
impl ECollisionEnabled {
    pub const NO_COLLISION: ECollisionEnabled = ECollisionEnabled(0);
    pub const QUERY_ONLY: ECollisionEnabled = ECollisionEnabled(1);
    pub const PHYSICS_ONLY: ECollisionEnabled = ECollisionEnabled(2);
    pub const QUERY_AND_PHYSICS: ECollisionEnabled = ECollisionEnabled(3);
    pub const PROBE_ONLY: ECollisionEnabled = ECollisionEnabled(4);
    pub const QUERY_AND_PROBE: ECollisionEnabled = ECollisionEnabled(5);
}
#[repr(transparent)]
pub struct EDOFMode(pub u8);
impl EDOFMode {
    pub const DEFAULT: EDOFMode = EDOFMode(0);
    pub const SIX_DOF: EDOFMode = EDOFMode(1);
    pub const YZ_PLANE: EDOFMode = EDOFMode(2);
    pub const XZ_PLANE: EDOFMode = EDOFMode(3);
    pub const XY_PLANE: EDOFMode = EDOFMode(4);
    pub const CUSTOM_PLANE: EDOFMode = EDOFMode(5);
    pub const NONE: EDOFMode = EDOFMode(6);
}
#[repr(transparent)]
pub struct ECollisionResponse(pub u8);
impl ECollisionResponse {
    pub const ECR_IGNORE: ECollisionResponse = ECollisionResponse(0);
    pub const ECR_OVERLAP: ECollisionResponse = ECollisionResponse(1);
    pub const ECR_BLOCK: ECollisionResponse = ECollisionResponse(2);
}
#[repr(transparent)]
pub struct EInputEvent(pub u8);
impl EInputEvent {
    pub const IE_PRESSED: EInputEvent = EInputEvent(0);
    pub const IE_RELEASED: EInputEvent = EInputEvent(1);
    pub const IE_REPEAT: EInputEvent = EInputEvent(2);
    pub const IE_DOUBLE_CLICK: EInputEvent = EInputEvent(3);
    pub const IE_AXIS: EInputEvent = EInputEvent(4);
}
#[repr(transparent)]
pub struct EMaterialProperty(pub u8);
impl EMaterialProperty {
    pub const MP_EMISSIVE_COLOR: EMaterialProperty = EMaterialProperty(0);
    pub const MP_OPACITY: EMaterialProperty = EMaterialProperty(1);
    pub const MP_OPACITY_MASK: EMaterialProperty = EMaterialProperty(2);
    pub const MP_DIFFUSE_COLOR: EMaterialProperty = EMaterialProperty(3);
    pub const MP_SPECULAR_COLOR: EMaterialProperty = EMaterialProperty(4);
    pub const MP_BASE_COLOR: EMaterialProperty = EMaterialProperty(5);
    pub const MP_METALLIC: EMaterialProperty = EMaterialProperty(6);
    pub const MP_SPECULAR: EMaterialProperty = EMaterialProperty(7);
    pub const MP_ROUGHNESS: EMaterialProperty = EMaterialProperty(8);
    pub const MP_ANISOTROPY: EMaterialProperty = EMaterialProperty(9);
    pub const MP_NORMAL: EMaterialProperty = EMaterialProperty(10);
    pub const MP_TANGENT: EMaterialProperty = EMaterialProperty(11);
    pub const MP_WORLD_POSITION_OFFSET: EMaterialProperty = EMaterialProperty(12);
    pub const MP_WORLD_DISPLACEMENT_DEPRECATED: EMaterialProperty = EMaterialProperty(
        13,
    );
    pub const MP_TESSELLATION_MULTIPLIER_DEPRECATED: EMaterialProperty = EMaterialProperty(
        14,
    );
    pub const MP_SUBSURFACE_COLOR: EMaterialProperty = EMaterialProperty(15);
    pub const MP_CUSTOM_DATA0: EMaterialProperty = EMaterialProperty(16);
    pub const MP_CUSTOM_DATA1: EMaterialProperty = EMaterialProperty(17);
    pub const MP_AMBIENT_OCCLUSION: EMaterialProperty = EMaterialProperty(18);
    pub const MP_REFRACTION: EMaterialProperty = EMaterialProperty(19);
    pub const MP_CUSTOMIZED_U_VS0: EMaterialProperty = EMaterialProperty(20);
    pub const MP_CUSTOMIZED_U_VS1: EMaterialProperty = EMaterialProperty(21);
    pub const MP_CUSTOMIZED_U_VS2: EMaterialProperty = EMaterialProperty(22);
    pub const MP_CUSTOMIZED_U_VS3: EMaterialProperty = EMaterialProperty(23);
    pub const MP_CUSTOMIZED_U_VS4: EMaterialProperty = EMaterialProperty(24);
    pub const MP_CUSTOMIZED_U_VS5: EMaterialProperty = EMaterialProperty(25);
    pub const MP_CUSTOMIZED_U_VS6: EMaterialProperty = EMaterialProperty(26);
    pub const MP_CUSTOMIZED_U_VS7: EMaterialProperty = EMaterialProperty(27);
    pub const MP_PIXEL_DEPTH_OFFSET: EMaterialProperty = EMaterialProperty(28);
    pub const MP_SHADING_MODEL: EMaterialProperty = EMaterialProperty(29);
    pub const MP_FRONT_MATERIAL: EMaterialProperty = EMaterialProperty(30);
    pub const MP_SURFACE_THICKNESS: EMaterialProperty = EMaterialProperty(31);
    pub const MP_DISPLACEMENT: EMaterialProperty = EMaterialProperty(32);
    pub const MP_MATERIAL_ATTRIBUTES: EMaterialProperty = EMaterialProperty(33);
    pub const MP_CUSTOM_OUTPUT: EMaterialProperty = EMaterialProperty(34);
}
#[repr(transparent)]
pub struct EBloomMethod(pub u8);
impl EBloomMethod {
    pub const BM_SOG: EBloomMethod = EBloomMethod(0);
    pub const BM_FFT: EBloomMethod = EBloomMethod(1);
}
#[repr(transparent)]
pub struct EAutoExposureMethod(pub u8);
impl EAutoExposureMethod {
    pub const AEM_HISTOGRAM: EAutoExposureMethod = EAutoExposureMethod(0);
    pub const AEM_BASIC: EAutoExposureMethod = EAutoExposureMethod(1);
    pub const AEM_MANUAL: EAutoExposureMethod = EAutoExposureMethod(2);
}
#[repr(transparent)]
pub struct EDepthOfFieldMethod(pub u8);
impl EDepthOfFieldMethod {
    pub const DOFM_BOKEH_DOF: EDepthOfFieldMethod = EDepthOfFieldMethod(0);
    pub const DOFM_GAUSSIAN: EDepthOfFieldMethod = EDepthOfFieldMethod(1);
    pub const DOFM_CIRCLE_DOF: EDepthOfFieldMethod = EDepthOfFieldMethod(2);
}
#[repr(transparent)]
pub struct ETemperatureMethod(pub u8);
impl ETemperatureMethod {
    pub const TEMP_WHITE_BALANCE: ETemperatureMethod = ETemperatureMethod(0);
    pub const TEMP_COLOR_TEMPERATURE: ETemperatureMethod = ETemperatureMethod(1);
}
#[repr(transparent)]
pub struct EDynamicGlobalIlluminationMethod(pub u8);
impl EDynamicGlobalIlluminationMethod {
    pub const NONE: EDynamicGlobalIlluminationMethod = EDynamicGlobalIlluminationMethod(
        0,
    );
    pub const LUMEN: EDynamicGlobalIlluminationMethod = EDynamicGlobalIlluminationMethod(
        1,
    );
    pub const SCREEN_SPACE: EDynamicGlobalIlluminationMethod = EDynamicGlobalIlluminationMethod(
        2,
    );
    pub const PLUGIN: EDynamicGlobalIlluminationMethod = EDynamicGlobalIlluminationMethod(
        3,
    );
}
#[repr(transparent)]
pub struct ELumenRayLightingModeOverride(pub u8);
impl ELumenRayLightingModeOverride {
    pub const DEFAULT: ELumenRayLightingModeOverride = ELumenRayLightingModeOverride(0);
    pub const SURFACE_CACHE: ELumenRayLightingModeOverride = ELumenRayLightingModeOverride(
        1,
    );
    pub const HIT_LIGHTING_FOR_REFLECTIONS: ELumenRayLightingModeOverride = ELumenRayLightingModeOverride(
        2,
    );
    pub const HIT_LIGHTING: ELumenRayLightingModeOverride = ELumenRayLightingModeOverride(
        3,
    );
}
#[repr(transparent)]
pub struct EReflectionMethod(pub u8);
impl EReflectionMethod {
    pub const NONE: EReflectionMethod = EReflectionMethod(0);
    pub const LUMEN: EReflectionMethod = EReflectionMethod(1);
    pub const SCREEN_SPACE: EReflectionMethod = EReflectionMethod(2);
}
#[repr(transparent)]
pub struct EReflectionsType(pub u8);
impl EReflectionsType {
    pub const SCREEN_SPACE: EReflectionsType = EReflectionsType(0);
    pub const RAY_TRACING: EReflectionsType = EReflectionsType(1);
}
#[repr(transparent)]
pub struct ELocalExposureMethod(pub u8);
impl ELocalExposureMethod {
    pub const BILATERAL: ELocalExposureMethod = ELocalExposureMethod(0);
    pub const FUSION: ELocalExposureMethod = ELocalExposureMethod(1);
}
#[repr(transparent)]
pub struct ETranslucencyType(pub u8);
impl ETranslucencyType {
    pub const RASTER: ETranslucencyType = ETranslucencyType(0);
    pub const RAY_TRACED_DEPRECATED: ETranslucencyType = ETranslucencyType(1);
    pub const RAY_TRACING: ETranslucencyType = ETranslucencyType(1);
    pub const RAY_TRACED: ETranslucencyType = ETranslucencyType(2);
}
#[repr(transparent)]
pub struct EReflectedAndRefractedRayTracedShadows(pub u8);
impl EReflectedAndRefractedRayTracedShadows {
    pub const DISABLED: EReflectedAndRefractedRayTracedShadows = EReflectedAndRefractedRayTracedShadows(
        0,
    );
    pub const HARD_SHADOWS: EReflectedAndRefractedRayTracedShadows = EReflectedAndRefractedRayTracedShadows(
        1,
    );
    pub const AREA_SHADOWS: EReflectedAndRefractedRayTracedShadows = EReflectedAndRefractedRayTracedShadows(
        2,
    );
}
#[repr(transparent)]
pub struct ETraceTypeQuery(pub u8);
impl ETraceTypeQuery {
    pub const TRACE_TYPE_QUERY1: ETraceTypeQuery = ETraceTypeQuery(0);
    pub const TRACE_TYPE_QUERY2: ETraceTypeQuery = ETraceTypeQuery(1);
    pub const TRACE_TYPE_QUERY3: ETraceTypeQuery = ETraceTypeQuery(2);
    pub const TRACE_TYPE_QUERY4: ETraceTypeQuery = ETraceTypeQuery(3);
    pub const TRACE_TYPE_QUERY5: ETraceTypeQuery = ETraceTypeQuery(4);
    pub const TRACE_TYPE_QUERY6: ETraceTypeQuery = ETraceTypeQuery(5);
    pub const TRACE_TYPE_QUERY7: ETraceTypeQuery = ETraceTypeQuery(6);
    pub const TRACE_TYPE_QUERY8: ETraceTypeQuery = ETraceTypeQuery(7);
    pub const TRACE_TYPE_QUERY9: ETraceTypeQuery = ETraceTypeQuery(8);
    pub const TRACE_TYPE_QUERY10: ETraceTypeQuery = ETraceTypeQuery(9);
    pub const TRACE_TYPE_QUERY11: ETraceTypeQuery = ETraceTypeQuery(10);
    pub const TRACE_TYPE_QUERY12: ETraceTypeQuery = ETraceTypeQuery(11);
    pub const TRACE_TYPE_QUERY13: ETraceTypeQuery = ETraceTypeQuery(12);
    pub const TRACE_TYPE_QUERY14: ETraceTypeQuery = ETraceTypeQuery(13);
    pub const TRACE_TYPE_QUERY15: ETraceTypeQuery = ETraceTypeQuery(14);
    pub const TRACE_TYPE_QUERY16: ETraceTypeQuery = ETraceTypeQuery(15);
    pub const TRACE_TYPE_QUERY17: ETraceTypeQuery = ETraceTypeQuery(16);
    pub const TRACE_TYPE_QUERY18: ETraceTypeQuery = ETraceTypeQuery(17);
    pub const TRACE_TYPE_QUERY19: ETraceTypeQuery = ETraceTypeQuery(18);
    pub const TRACE_TYPE_QUERY20: ETraceTypeQuery = ETraceTypeQuery(19);
    pub const TRACE_TYPE_QUERY21: ETraceTypeQuery = ETraceTypeQuery(20);
    pub const TRACE_TYPE_QUERY22: ETraceTypeQuery = ETraceTypeQuery(21);
    pub const TRACE_TYPE_QUERY23: ETraceTypeQuery = ETraceTypeQuery(22);
    pub const TRACE_TYPE_QUERY24: ETraceTypeQuery = ETraceTypeQuery(23);
    pub const TRACE_TYPE_QUERY25: ETraceTypeQuery = ETraceTypeQuery(24);
    pub const TRACE_TYPE_QUERY26: ETraceTypeQuery = ETraceTypeQuery(25);
    pub const TRACE_TYPE_QUERY27: ETraceTypeQuery = ETraceTypeQuery(26);
    pub const TRACE_TYPE_QUERY28: ETraceTypeQuery = ETraceTypeQuery(27);
    pub const TRACE_TYPE_QUERY29: ETraceTypeQuery = ETraceTypeQuery(28);
    pub const TRACE_TYPE_QUERY30: ETraceTypeQuery = ETraceTypeQuery(29);
    pub const TRACE_TYPE_QUERY31: ETraceTypeQuery = ETraceTypeQuery(30);
    pub const TRACE_TYPE_QUERY32: ETraceTypeQuery = ETraceTypeQuery(31);
}
#[repr(transparent)]
pub struct EComponentCreationMethod(pub u8);
impl EComponentCreationMethod {
    pub const NATIVE: EComponentCreationMethod = EComponentCreationMethod(0);
    pub const SIMPLE_CONSTRUCTION_SCRIPT: EComponentCreationMethod = EComponentCreationMethod(
        1,
    );
    pub const USER_CONSTRUCTION_SCRIPT: EComponentCreationMethod = EComponentCreationMethod(
        2,
    );
    pub const INSTANCE: EComponentCreationMethod = EComponentCreationMethod(3);
}
#[repr(transparent)]
pub struct ERichCurveExtrapolation(pub u8);
impl ERichCurveExtrapolation {
    pub const RCCE_CYCLE: ERichCurveExtrapolation = ERichCurveExtrapolation(0);
    pub const RCCE_CYCLE_WITH_OFFSET: ERichCurveExtrapolation = ERichCurveExtrapolation(
        1,
    );
    pub const RCCE_OSCILLATE: ERichCurveExtrapolation = ERichCurveExtrapolation(2);
    pub const RCCE_LINEAR: ERichCurveExtrapolation = ERichCurveExtrapolation(3);
    pub const RCCE_CONSTANT: ERichCurveExtrapolation = ERichCurveExtrapolation(4);
    pub const RCCE_NONE: ERichCurveExtrapolation = ERichCurveExtrapolation(5);
}
#[repr(transparent)]
pub struct ERichCurveInterpMode(pub u8);
impl ERichCurveInterpMode {
    pub const RCIM_LINEAR: ERichCurveInterpMode = ERichCurveInterpMode(0);
    pub const RCIM_CONSTANT: ERichCurveInterpMode = ERichCurveInterpMode(1);
    pub const RCIM_CUBIC: ERichCurveInterpMode = ERichCurveInterpMode(2);
    pub const RCIM_NONE: ERichCurveInterpMode = ERichCurveInterpMode(3);
}
#[repr(transparent)]
pub struct ERichCurveTangentMode(pub u8);
impl ERichCurveTangentMode {
    pub const RCTM_AUTO: ERichCurveTangentMode = ERichCurveTangentMode(0);
    pub const RCTM_USER: ERichCurveTangentMode = ERichCurveTangentMode(1);
    pub const RCTM_BREAK: ERichCurveTangentMode = ERichCurveTangentMode(2);
    pub const RCTM_NONE: ERichCurveTangentMode = ERichCurveTangentMode(3);
    pub const RCTM_SMART_AUTO: ERichCurveTangentMode = ERichCurveTangentMode(4);
}
#[repr(transparent)]
pub struct ERichCurveTangentWeightMode(pub u8);
impl ERichCurveTangentWeightMode {
    pub const RCTWM_WEIGHTED_NONE: ERichCurveTangentWeightMode = ERichCurveTangentWeightMode(
        0,
    );
    pub const RCTWM_WEIGHTED_ARRIVE: ERichCurveTangentWeightMode = ERichCurveTangentWeightMode(
        1,
    );
    pub const RCTWM_WEIGHTED_LEAVE: ERichCurveTangentWeightMode = ERichCurveTangentWeightMode(
        2,
    );
    pub const RCTWM_WEIGHTED_BOTH: ERichCurveTangentWeightMode = ERichCurveTangentWeightMode(
        3,
    );
}
#[repr(transparent)]
pub struct EPinContainerType(pub u8);
impl EPinContainerType {
    pub const NONE: EPinContainerType = EPinContainerType(0);
    pub const ARRAY: EPinContainerType = EPinContainerType(1);
    pub const SET: EPinContainerType = EPinContainerType(2);
    pub const MAP: EPinContainerType = EPinContainerType(3);
}
#[repr(transparent)]
pub struct EEdGraphPinDirection(pub u8);
impl EEdGraphPinDirection {
    pub const EGPD_INPUT: EEdGraphPinDirection = EEdGraphPinDirection(0);
    pub const EGPD_OUTPUT: EEdGraphPinDirection = EEdGraphPinDirection(1);
}
#[repr(transparent)]
pub struct EGraphType(pub u8);
impl EGraphType {
    pub const GT_FUNCTION: EGraphType = EGraphType(0);
    pub const GT_UBERGRAPH: EGraphType = EGraphType(1);
    pub const GT_MACRO: EGraphType = EGraphType(2);
    pub const GT_ANIMATION: EGraphType = EGraphType(3);
    pub const GT_STATE_MACHINE: EGraphType = EGraphType(4);
}
#[repr(transparent)]
pub struct EAlphaBlendOption(pub u8);
impl EAlphaBlendOption {
    pub const LINEAR: EAlphaBlendOption = EAlphaBlendOption(0);
    pub const CUBIC: EAlphaBlendOption = EAlphaBlendOption(1);
    pub const HERMITE_CUBIC: EAlphaBlendOption = EAlphaBlendOption(2);
    pub const SINUSOIDAL: EAlphaBlendOption = EAlphaBlendOption(3);
    pub const QUADRATIC_IN_OUT: EAlphaBlendOption = EAlphaBlendOption(4);
    pub const CUBIC_IN_OUT: EAlphaBlendOption = EAlphaBlendOption(5);
    pub const QUARTIC_IN_OUT: EAlphaBlendOption = EAlphaBlendOption(6);
    pub const QUINTIC_IN_OUT: EAlphaBlendOption = EAlphaBlendOption(7);
    pub const CIRCULAR_IN: EAlphaBlendOption = EAlphaBlendOption(8);
    pub const CIRCULAR_OUT: EAlphaBlendOption = EAlphaBlendOption(9);
    pub const CIRCULAR_IN_OUT: EAlphaBlendOption = EAlphaBlendOption(10);
    pub const EXP_IN: EAlphaBlendOption = EAlphaBlendOption(11);
    pub const EXP_OUT: EAlphaBlendOption = EAlphaBlendOption(12);
    pub const EXP_IN_OUT: EAlphaBlendOption = EAlphaBlendOption(13);
    pub const CUSTOM: EAlphaBlendOption = EAlphaBlendOption(14);
}
#[repr(transparent)]
pub struct EAnimGroupRole(pub u8);
impl EAnimGroupRole {
    pub const CAN_BE_LEADER: EAnimGroupRole = EAnimGroupRole(0);
    pub const ALWAYS_FOLLOWER: EAnimGroupRole = EAnimGroupRole(1);
    pub const ALWAYS_LEADER: EAnimGroupRole = EAnimGroupRole(2);
    pub const TRANSITION_LEADER: EAnimGroupRole = EAnimGroupRole(3);
    pub const TRANSITION_FOLLOWER: EAnimGroupRole = EAnimGroupRole(4);
    pub const EXCLUSIVE_ALWAYS_LEADER: EAnimGroupRole = EAnimGroupRole(5);
}
#[repr(transparent)]
pub struct EAnimAlphaInputType(pub u8);
impl EAnimAlphaInputType {
    pub const FLOAT: EAnimAlphaInputType = EAnimAlphaInputType(0);
    pub const BOOL: EAnimAlphaInputType = EAnimAlphaInputType(1);
    pub const CURVE: EAnimAlphaInputType = EAnimAlphaInputType(2);
}
#[repr(transparent)]
pub struct EBoneAxis(pub u8);
impl EBoneAxis {
    pub const BA_X: EBoneAxis = EBoneAxis(0);
    pub const BA_Y: EBoneAxis = EBoneAxis(1);
    pub const BA_Z: EBoneAxis = EBoneAxis(2);
}
#[repr(transparent)]
pub struct EAnimSyncGroupScope(pub u8);
impl EAnimSyncGroupScope {
    pub const LOCAL: EAnimSyncGroupScope = EAnimSyncGroupScope(0);
    pub const COMPONENT: EAnimSyncGroupScope = EAnimSyncGroupScope(1);
}
#[repr(transparent)]
pub struct EAnimSyncMethod(pub u8);
impl EAnimSyncMethod {
    pub const DO_NOT_SYNC: EAnimSyncMethod = EAnimSyncMethod(0);
    pub const SYNC_GROUP: EAnimSyncMethod = EAnimSyncMethod(1);
    pub const GRAPH: EAnimSyncMethod = EAnimSyncMethod(2);
}
#[repr(transparent)]
pub struct EBoneControlSpace(pub u8);
impl EBoneControlSpace {
    pub const BCS_WORLD_SPACE: EBoneControlSpace = EBoneControlSpace(0);
    pub const BCS_COMPONENT_SPACE: EBoneControlSpace = EBoneControlSpace(1);
    pub const BCS_PARENT_BONE_SPACE: EBoneControlSpace = EBoneControlSpace(2);
    pub const BCS_BONE_SPACE: EBoneControlSpace = EBoneControlSpace(3);
}
#[repr(transparent)]
pub struct ECurveBlendOption(pub u8);
impl ECurveBlendOption {
    pub const OVERRIDE: ECurveBlendOption = ECurveBlendOption(0);
    pub const DO_NOT_OVERRIDE: ECurveBlendOption = ECurveBlendOption(1);
    pub const NORMALIZE_BY_WEIGHT: ECurveBlendOption = ECurveBlendOption(2);
    pub const BLEND_BY_WEIGHT: ECurveBlendOption = ECurveBlendOption(3);
    pub const USE_BASE_POSE: ECurveBlendOption = ECurveBlendOption(4);
    pub const USE_MAX_VALUE: ECurveBlendOption = ECurveBlendOption(5);
    pub const USE_MIN_VALUE: ECurveBlendOption = ECurveBlendOption(6);
}
#[repr(transparent)]
pub struct AnimPhysTwistAxis(pub u8);
impl AnimPhysTwistAxis {
    pub const AXIS_X: AnimPhysTwistAxis = AnimPhysTwistAxis(0);
    pub const AXIS_Y: AnimPhysTwistAxis = AnimPhysTwistAxis(1);
    pub const AXIS_Z: AnimPhysTwistAxis = AnimPhysTwistAxis(2);
}
#[repr(transparent)]
pub struct AnimPhysCollisionType(pub u8);
impl AnimPhysCollisionType {
    pub const CO_M: AnimPhysCollisionType = AnimPhysCollisionType(0);
    pub const CUSTOM_SPHERE: AnimPhysCollisionType = AnimPhysCollisionType(1);
    pub const INNER_SPHERE: AnimPhysCollisionType = AnimPhysCollisionType(2);
    pub const OUTER_SPHERE: AnimPhysCollisionType = AnimPhysCollisionType(3);
}
#[repr(transparent)]
pub struct EComponentType(pub u8);
impl EComponentType {
    pub const NONE: EComponentType = EComponentType(0);
    pub const TRANSLATION_X: EComponentType = EComponentType(1);
    pub const TRANSLATION_Y: EComponentType = EComponentType(2);
    pub const TRANSLATION_Z: EComponentType = EComponentType(3);
    pub const ROTATION_X: EComponentType = EComponentType(4);
    pub const ROTATION_Y: EComponentType = EComponentType(5);
    pub const ROTATION_Z: EComponentType = EComponentType(6);
    pub const SCALE: EComponentType = EComponentType(7);
    pub const SCALE_X: EComponentType = EComponentType(8);
    pub const SCALE_Y: EComponentType = EComponentType(9);
    pub const SCALE_Z: EComponentType = EComponentType(10);
}
#[repr(transparent)]
pub struct EBoneRotationSource(pub u8);
impl EBoneRotationSource {
    pub const BRS_KEEP_COMPONENT_SPACE_ROTATION: EBoneRotationSource = EBoneRotationSource(
        0,
    );
    pub const BRS_KEEP_LOCAL_SPACE_ROTATION: EBoneRotationSource = EBoneRotationSource(
        1,
    );
    pub const BRS_COPY_FROM_TARGET: EBoneRotationSource = EBoneRotationSource(2);
}
#[repr(transparent)]
pub struct EAxisOption(pub u8);
impl EAxisOption {
    pub const X: EAxisOption = EAxisOption(0);
    pub const Y: EAxisOption = EAxisOption(1);
    pub const Z: EAxisOption = EAxisOption(2);
    pub const X_NEG: EAxisOption = EAxisOption(3);
    pub const Y_NEG: EAxisOption = EAxisOption(4);
    pub const Z_NEG: EAxisOption = EAxisOption(5);
    pub const CUSTOM: EAxisOption = EAxisOption(6);
}
#[repr(transparent)]
pub struct ECameraShakePlaySpace(pub u8);
impl ECameraShakePlaySpace {
    pub const CAMERA_LOCAL: ECameraShakePlaySpace = ECameraShakePlaySpace(0);
    pub const WORLD: ECameraShakePlaySpace = ECameraShakePlaySpace(1);
    pub const USER_DEFINED: ECameraShakePlaySpace = ECameraShakePlaySpace(2);
}
#[repr(transparent)]
pub struct EMaterialParameterAssociation(pub u8);
impl EMaterialParameterAssociation {
    pub const LAYER_PARAMETER: EMaterialParameterAssociation = EMaterialParameterAssociation(
        0,
    );
    pub const BLEND_PARAMETER: EMaterialParameterAssociation = EMaterialParameterAssociation(
        1,
    );
    pub const GLOBAL_PARAMETER: EMaterialParameterAssociation = EMaterialParameterAssociation(
        2,
    );
}
#[repr(transparent)]
pub struct EVectorQuantization(pub u8);
impl EVectorQuantization {
    pub const ROUND_WHOLE_NUMBER: EVectorQuantization = EVectorQuantization(0);
    pub const ROUND_ONE_DECIMAL: EVectorQuantization = EVectorQuantization(1);
    pub const ROUND_TWO_DECIMALS: EVectorQuantization = EVectorQuantization(2);
}
#[repr(transparent)]
pub struct ERotatorQuantization(pub u8);
impl ERotatorQuantization {
    pub const BYTE_COMPONENTS: ERotatorQuantization = ERotatorQuantization(0);
    pub const SHORT_COMPONENTS: ERotatorQuantization = ERotatorQuantization(1);
}
#[repr(transparent)]
pub struct EAspectRatioAxisConstraint(pub u8);
impl EAspectRatioAxisConstraint {
    pub const ASPECT_RATIO_MAINTAIN_YFOV: EAspectRatioAxisConstraint = EAspectRatioAxisConstraint(
        0,
    );
    pub const ASPECT_RATIO_MAINTAIN_XFOV: EAspectRatioAxisConstraint = EAspectRatioAxisConstraint(
        1,
    );
    pub const ASPECT_RATIO_MAJOR_AXIS_FOV: EAspectRatioAxisConstraint = EAspectRatioAxisConstraint(
        2,
    );
}
#[repr(transparent)]
pub struct EAnimInterpolationType(pub u8);
impl EAnimInterpolationType {
    pub const LINEAR: EAnimInterpolationType = EAnimInterpolationType(0);
    pub const STEP: EAnimInterpolationType = EAnimInterpolationType(1);
}
#[repr(transparent)]
pub struct EViewModeIndex(pub u8);
impl EViewModeIndex {
    pub const VMI_BRUSH_WIREFRAME: EViewModeIndex = EViewModeIndex(0);
    pub const VMI_WIREFRAME: EViewModeIndex = EViewModeIndex(1);
    pub const VMI_UNLIT: EViewModeIndex = EViewModeIndex(2);
    pub const VMI_LIT: EViewModeIndex = EViewModeIndex(3);
    pub const VMI_LIT_DETAIL_LIGHTING: EViewModeIndex = EViewModeIndex(4);
    pub const VMI_LIGHTING_ONLY: EViewModeIndex = EViewModeIndex(5);
    pub const VMI_LIGHT_COMPLEXITY: EViewModeIndex = EViewModeIndex(6);
    pub const VMI_SHADER_COMPLEXITY: EViewModeIndex = EViewModeIndex(8);
    pub const VMI_LIGHTMAP_DENSITY: EViewModeIndex = EViewModeIndex(9);
    pub const VMI_LIT_LIGHTMAP_DENSITY: EViewModeIndex = EViewModeIndex(10);
    pub const VMI_REFLECTION_OVERRIDE: EViewModeIndex = EViewModeIndex(11);
    pub const VMI_VISUALIZE_BUFFER: EViewModeIndex = EViewModeIndex(12);
    pub const VMI_STATIONARY_LIGHT_OVERLAP: EViewModeIndex = EViewModeIndex(14);
    pub const VMI_COLLISION_PAWN: EViewModeIndex = EViewModeIndex(15);
    pub const VMI_COLLISION_VISIBILITY: EViewModeIndex = EViewModeIndex(16);
    pub const VMI_LOD_COLORATION: EViewModeIndex = EViewModeIndex(18);
    pub const VMI_QUAD_OVERDRAW: EViewModeIndex = EViewModeIndex(19);
    pub const VMI_PRIMITIVE_DISTANCE_ACCURACY: EViewModeIndex = EViewModeIndex(20);
    pub const VMI_MESH_UV_DENSITY_ACCURACY: EViewModeIndex = EViewModeIndex(21);
    pub const VMI_SHADER_COMPLEXITY_WITH_QUAD_OVERDRAW: EViewModeIndex = EViewModeIndex(
        22,
    );
    pub const VMI_HLOD_COLORATION: EViewModeIndex = EViewModeIndex(23);
    pub const VMI_GROUP_LOD_COLORATION: EViewModeIndex = EViewModeIndex(24);
    pub const VMI_MATERIAL_TEXTURE_SCALE_ACCURACY: EViewModeIndex = EViewModeIndex(25);
    pub const VMI_REQUIRED_TEXTURE_RESOLUTION: EViewModeIndex = EViewModeIndex(26);
    pub const VMI_PATH_TRACING: EViewModeIndex = EViewModeIndex(27);
    pub const VMI_RAY_TRACING_DEBUG: EViewModeIndex = EViewModeIndex(28);
    pub const VMI_VISUALIZE_NANITE: EViewModeIndex = EViewModeIndex(29);
    pub const VMI_VISUALIZE_VIRTUAL_TEXTURE: EViewModeIndex = EViewModeIndex(30);
    pub const VMI_VISUALIZE_LUMEN: EViewModeIndex = EViewModeIndex(31);
    pub const VMI_VISUALIZE_VIRTUAL_SHADOW_MAP: EViewModeIndex = EViewModeIndex(32);
    pub const VMI_VISUALIZE_GPU_SKIN_CACHE: EViewModeIndex = EViewModeIndex(33);
    pub const VMI_VISUALIZE_SUBSTRATE: EViewModeIndex = EViewModeIndex(34);
    pub const VMI_VISUALIZE_GROOM: EViewModeIndex = EViewModeIndex(35);
    pub const VMI_LWC_COMPLEXITY: EViewModeIndex = EViewModeIndex(36);
    pub const VMI_LIT_WIREFRAME: EViewModeIndex = EViewModeIndex(37);
    pub const VMI_VISUALIZE_ACTOR_COLORATION: EViewModeIndex = EViewModeIndex(38);
    pub const VMI_SHADOW_CASTERS: EViewModeIndex = EViewModeIndex(39);
    pub const VMI_CLAY: EViewModeIndex = EViewModeIndex(40);
    pub const VMI_ZEBRA: EViewModeIndex = EViewModeIndex(41);
    pub const VMI_FRONT_BACK_FACE: EViewModeIndex = EViewModeIndex(42);
    pub const VMI_RANDOM_COLOR: EViewModeIndex = EViewModeIndex(43);
    pub const VMI_UNKNOWN: EViewModeIndex = EViewModeIndex(255);
}
#[repr(transparent)]
pub struct EBlendableLocation(pub u8);
impl EBlendableLocation {
    pub const BL_SCENE_COLOR_BEFORE_DOF: EBlendableLocation = EBlendableLocation(2);
    pub const BL_SCENE_COLOR_AFTER_DOF: EBlendableLocation = EBlendableLocation(1);
    pub const BL_TRANSLUCENCY_AFTER_DOF: EBlendableLocation = EBlendableLocation(5);
    pub const BL_SSR_INPUT: EBlendableLocation = EBlendableLocation(4);
    pub const BL_SCENE_COLOR_BEFORE_BLOOM: EBlendableLocation = EBlendableLocation(6);
    pub const BL_REPLACING_TONEMAPPER: EBlendableLocation = EBlendableLocation(3);
    pub const BL_SCENE_COLOR_AFTER_TONEMAPPING: EBlendableLocation = EBlendableLocation(
        0,
    );
    pub const BL_BEFORE_TRANSLUCENCY: EBlendableLocation = EBlendableLocation(2);
    pub const BL_BEFORE_TONEMAPPING: EBlendableLocation = EBlendableLocation(1);
    pub const BL_AFTER_TONEMAPPING: EBlendableLocation = EBlendableLocation(0);
}
#[repr(transparent)]
pub struct ECollectionScriptingShareType(pub u8);
impl ECollectionScriptingShareType {
    pub const LOCAL: ECollectionScriptingShareType = ECollectionScriptingShareType(0);
    pub const PRIVATE: ECollectionScriptingShareType = ECollectionScriptingShareType(1);
    pub const SHARED: ECollectionScriptingShareType = ECollectionScriptingShareType(2);
}
#[repr(transparent)]
pub struct EShadowCacheInvalidationBehavior(pub u8);
impl EShadowCacheInvalidationBehavior {
    pub const AUTO: EShadowCacheInvalidationBehavior = EShadowCacheInvalidationBehavior(
        0,
    );
    pub const ALWAYS: EShadowCacheInvalidationBehavior = EShadowCacheInvalidationBehavior(
        1,
    );
    pub const RIGID: EShadowCacheInvalidationBehavior = EShadowCacheInvalidationBehavior(
        2,
    );
    pub const STATIC: EShadowCacheInvalidationBehavior = EShadowCacheInvalidationBehavior(
        3,
    );
}
#[repr(transparent)]
pub struct ESplineMeshAxis(pub u8);
impl ESplineMeshAxis {
    pub const X: ESplineMeshAxis = ESplineMeshAxis(0);
    pub const Y: ESplineMeshAxis = ESplineMeshAxis(1);
    pub const Z: ESplineMeshAxis = ESplineMeshAxis(2);
}
#[repr(transparent)]
pub struct ERawCurveTrackTypes(pub u8);
impl ERawCurveTrackTypes {
    pub const RCT_FLOAT: ERawCurveTrackTypes = ERawCurveTrackTypes(0);
    pub const RCT_VECTOR: ERawCurveTrackTypes = ERawCurveTrackTypes(1);
    pub const RCT_TRANSFORM: ERawCurveTrackTypes = ERawCurveTrackTypes(2);
}
#[repr(transparent)]
pub struct ETransformCurveChannel(pub u8);
impl ETransformCurveChannel {
    pub const POSITION: ETransformCurveChannel = ETransformCurveChannel(0);
    pub const ROTATION: ETransformCurveChannel = ETransformCurveChannel(1);
    pub const SCALE: ETransformCurveChannel = ETransformCurveChannel(2);
    pub const INVALID: ETransformCurveChannel = ETransformCurveChannel(3);
}
#[repr(transparent)]
pub struct EVectorCurveChannel(pub u8);
impl EVectorCurveChannel {
    pub const X: EVectorCurveChannel = EVectorCurveChannel(0);
    pub const Y: EVectorCurveChannel = EVectorCurveChannel(1);
    pub const Z: EVectorCurveChannel = EVectorCurveChannel(2);
    pub const INVALID: EVectorCurveChannel = EVectorCurveChannel(3);
}
#[repr(transparent)]
pub struct EAdditiveAnimationType(pub u8);
impl EAdditiveAnimationType {
    pub const AAT_NONE: EAdditiveAnimationType = EAdditiveAnimationType(0);
    pub const AAT_LOCAL_SPACE_BASE: EAdditiveAnimationType = EAdditiveAnimationType(1);
    pub const AAT_ROTATION_OFFSET_MESH_SPACE: EAdditiveAnimationType = EAdditiveAnimationType(
        2,
    );
}
#[repr(transparent)]
pub struct EPrimaryAssetCookRule(pub u8);
impl EPrimaryAssetCookRule {
    pub const UNKNOWN: EPrimaryAssetCookRule = EPrimaryAssetCookRule(0);
    pub const NEVER_COOK: EPrimaryAssetCookRule = EPrimaryAssetCookRule(1);
    pub const PRODUCTION_NEVER_COOK: EPrimaryAssetCookRule = EPrimaryAssetCookRule(2);
    pub const DEVELOPMENT_COOK: EPrimaryAssetCookRule = EPrimaryAssetCookRule(2);
    pub const DEVELOPMENT_ALWAYS_PRODUCTION_NEVER_COOK: EPrimaryAssetCookRule = EPrimaryAssetCookRule(
        3,
    );
    pub const DEVELOPMENT_ALWAYS_COOK: EPrimaryAssetCookRule = EPrimaryAssetCookRule(3);
    pub const DEVELOPMENT_ALWAYS_PRODUCTION_UNKNOWN_COOK: EPrimaryAssetCookRule = EPrimaryAssetCookRule(
        4,
    );
    pub const ALWAYS_COOK: EPrimaryAssetCookRule = EPrimaryAssetCookRule(5);
}
#[repr(transparent)]
pub struct ECameraProjectionMode(pub u8);
impl ECameraProjectionMode {
    pub const PERSPECTIVE: ECameraProjectionMode = ECameraProjectionMode(0);
    pub const ORTHOGRAPHIC: ECameraProjectionMode = ECameraProjectionMode(1);
}
#[repr(transparent)]
pub struct EFullyLoadPackageType(pub u8);
impl EFullyLoadPackageType {
    pub const FULLYLOAD_MAP: EFullyLoadPackageType = EFullyLoadPackageType(0);
    pub const FULLYLOAD_GAME_PRE_LOAD_CLASS: EFullyLoadPackageType = EFullyLoadPackageType(
        1,
    );
    pub const FULLYLOAD_GAME_POST_LOAD_CLASS: EFullyLoadPackageType = EFullyLoadPackageType(
        2,
    );
    pub const FULLYLOAD_ALWAYS: EFullyLoadPackageType = EFullyLoadPackageType(3);
    pub const FULLYLOAD_MUTATOR: EFullyLoadPackageType = EFullyLoadPackageType(4);
}
#[repr(transparent)]
pub struct EFontImportCharacterSet(pub u8);
impl EFontImportCharacterSet {
    pub const FONT_ICS_DEFAULT: EFontImportCharacterSet = EFontImportCharacterSet(0);
    pub const FONT_ICS_ANSI: EFontImportCharacterSet = EFontImportCharacterSet(1);
    pub const FONT_ICS_SYMBOL: EFontImportCharacterSet = EFontImportCharacterSet(2);
}
#[repr(transparent)]
pub struct EAttenuationDistanceModel(pub u8);
impl EAttenuationDistanceModel {
    pub const LINEAR: EAttenuationDistanceModel = EAttenuationDistanceModel(0);
    pub const LOGARITHMIC: EAttenuationDistanceModel = EAttenuationDistanceModel(1);
    pub const INVERSE: EAttenuationDistanceModel = EAttenuationDistanceModel(2);
    pub const LOG_REVERSE: EAttenuationDistanceModel = EAttenuationDistanceModel(3);
    pub const NATURAL_SOUND: EAttenuationDistanceModel = EAttenuationDistanceModel(4);
    pub const CUSTOM: EAttenuationDistanceModel = EAttenuationDistanceModel(5);
}
#[repr(transparent)]
pub struct EAttenuationShape(pub u8);
impl EAttenuationShape {
    pub const SPHERE: EAttenuationShape = EAttenuationShape(0);
    pub const CAPSULE: EAttenuationShape = EAttenuationShape(1);
    pub const BOX: EAttenuationShape = EAttenuationShape(2);
    pub const CONE: EAttenuationShape = EAttenuationShape(3);
}
#[repr(transparent)]
pub struct ENaturalSoundFalloffMode(pub u8);
impl ENaturalSoundFalloffMode {
    pub const CONTINUES: ENaturalSoundFalloffMode = ENaturalSoundFalloffMode(0);
    pub const SILENT: ENaturalSoundFalloffMode = ENaturalSoundFalloffMode(1);
    pub const HOLD: ENaturalSoundFalloffMode = ENaturalSoundFalloffMode(2);
}
#[repr(transparent)]
pub struct EObjectTypeQuery(pub u8);
impl EObjectTypeQuery {
    pub const OBJECT_TYPE_QUERY1: EObjectTypeQuery = EObjectTypeQuery(0);
    pub const OBJECT_TYPE_QUERY2: EObjectTypeQuery = EObjectTypeQuery(1);
    pub const OBJECT_TYPE_QUERY3: EObjectTypeQuery = EObjectTypeQuery(2);
    pub const OBJECT_TYPE_QUERY4: EObjectTypeQuery = EObjectTypeQuery(3);
    pub const OBJECT_TYPE_QUERY5: EObjectTypeQuery = EObjectTypeQuery(4);
    pub const OBJECT_TYPE_QUERY6: EObjectTypeQuery = EObjectTypeQuery(5);
    pub const OBJECT_TYPE_QUERY7: EObjectTypeQuery = EObjectTypeQuery(6);
    pub const OBJECT_TYPE_QUERY8: EObjectTypeQuery = EObjectTypeQuery(7);
    pub const OBJECT_TYPE_QUERY9: EObjectTypeQuery = EObjectTypeQuery(8);
    pub const OBJECT_TYPE_QUERY10: EObjectTypeQuery = EObjectTypeQuery(9);
    pub const OBJECT_TYPE_QUERY11: EObjectTypeQuery = EObjectTypeQuery(10);
    pub const OBJECT_TYPE_QUERY12: EObjectTypeQuery = EObjectTypeQuery(11);
    pub const OBJECT_TYPE_QUERY13: EObjectTypeQuery = EObjectTypeQuery(12);
    pub const OBJECT_TYPE_QUERY14: EObjectTypeQuery = EObjectTypeQuery(13);
    pub const OBJECT_TYPE_QUERY15: EObjectTypeQuery = EObjectTypeQuery(14);
    pub const OBJECT_TYPE_QUERY16: EObjectTypeQuery = EObjectTypeQuery(15);
    pub const OBJECT_TYPE_QUERY17: EObjectTypeQuery = EObjectTypeQuery(16);
    pub const OBJECT_TYPE_QUERY18: EObjectTypeQuery = EObjectTypeQuery(17);
    pub const OBJECT_TYPE_QUERY19: EObjectTypeQuery = EObjectTypeQuery(18);
    pub const OBJECT_TYPE_QUERY20: EObjectTypeQuery = EObjectTypeQuery(19);
    pub const OBJECT_TYPE_QUERY21: EObjectTypeQuery = EObjectTypeQuery(20);
    pub const OBJECT_TYPE_QUERY22: EObjectTypeQuery = EObjectTypeQuery(21);
    pub const OBJECT_TYPE_QUERY23: EObjectTypeQuery = EObjectTypeQuery(22);
    pub const OBJECT_TYPE_QUERY24: EObjectTypeQuery = EObjectTypeQuery(23);
    pub const OBJECT_TYPE_QUERY25: EObjectTypeQuery = EObjectTypeQuery(24);
    pub const OBJECT_TYPE_QUERY26: EObjectTypeQuery = EObjectTypeQuery(25);
    pub const OBJECT_TYPE_QUERY27: EObjectTypeQuery = EObjectTypeQuery(26);
    pub const OBJECT_TYPE_QUERY28: EObjectTypeQuery = EObjectTypeQuery(27);
    pub const OBJECT_TYPE_QUERY29: EObjectTypeQuery = EObjectTypeQuery(28);
    pub const OBJECT_TYPE_QUERY30: EObjectTypeQuery = EObjectTypeQuery(29);
    pub const OBJECT_TYPE_QUERY31: EObjectTypeQuery = EObjectTypeQuery(30);
    pub const OBJECT_TYPE_QUERY32: EObjectTypeQuery = EObjectTypeQuery(31);
}
#[repr(transparent)]
pub struct EDrawDebugTrace(pub u8);
impl EDrawDebugTrace {
    pub const NONE: EDrawDebugTrace = EDrawDebugTrace(0);
    pub const FOR_ONE_FRAME: EDrawDebugTrace = EDrawDebugTrace(1);
    pub const FOR_DURATION: EDrawDebugTrace = EDrawDebugTrace(2);
    pub const PERSISTENT: EDrawDebugTrace = EDrawDebugTrace(3);
}
#[repr(transparent)]
pub struct ELevelInstanceCreationType(pub u8);
impl ELevelInstanceCreationType {
    pub const LEVEL_INSTANCE: ELevelInstanceCreationType = ELevelInstanceCreationType(0);
    pub const PACKED_LEVEL_ACTOR: ELevelInstanceCreationType = ELevelInstanceCreationType(
        1,
    );
}
#[repr(transparent)]
pub struct ELevelInstancePivotType(pub u8);
impl ELevelInstancePivotType {
    pub const CENTER_MIN_Z: ELevelInstancePivotType = ELevelInstancePivotType(0);
    pub const CENTER: ELevelInstancePivotType = ELevelInstancePivotType(1);
    pub const ACTOR: ELevelInstancePivotType = ELevelInstancePivotType(2);
    pub const WORLD_ORIGIN: ELevelInstancePivotType = ELevelInstancePivotType(3);
}
#[repr(transparent)]
pub struct EMaterialCacheAttributeIdentity(pub u8);
impl EMaterialCacheAttributeIdentity {
    pub const NONE: EMaterialCacheAttributeIdentity = EMaterialCacheAttributeIdentity(0);
    pub const BASE_COLOR_ROUGHNESS: EMaterialCacheAttributeIdentity = EMaterialCacheAttributeIdentity(
        1,
    );
    pub const NORMAL_SPECULAR_OPACITY: EMaterialCacheAttributeIdentity = EMaterialCacheAttributeIdentity(
        2,
    );
    pub const METALLIC_WORLD_POSITION_OFFSET: EMaterialCacheAttributeIdentity = EMaterialCacheAttributeIdentity(
        3,
    );
}
#[repr(transparent)]
pub struct EMaterialCacheAttribute(pub u8);
impl EMaterialCacheAttribute {
    pub const BASE_COLOR: EMaterialCacheAttribute = EMaterialCacheAttribute(0);
    pub const NORMAL: EMaterialCacheAttribute = EMaterialCacheAttribute(1);
    pub const ROUGHNESS: EMaterialCacheAttribute = EMaterialCacheAttribute(2);
    pub const SPECULAR: EMaterialCacheAttribute = EMaterialCacheAttribute(3);
    pub const METALLIC: EMaterialCacheAttribute = EMaterialCacheAttribute(4);
    pub const OPACITY: EMaterialCacheAttribute = EMaterialCacheAttribute(5);
    pub const WORLD_POSITION: EMaterialCacheAttribute = EMaterialCacheAttribute(6);
    pub const WORLD_HEIGHT: EMaterialCacheAttribute = EMaterialCacheAttribute(7);
    pub const MASK: EMaterialCacheAttribute = EMaterialCacheAttribute(8);
    pub const FLOAT: EMaterialCacheAttribute = EMaterialCacheAttribute(9);
}
#[repr(transparent)]
pub struct EMaterialExpressionConvertType(pub u8);
impl EMaterialExpressionConvertType {
    pub const SCALAR: EMaterialExpressionConvertType = EMaterialExpressionConvertType(0);
    pub const VECTOR2: EMaterialExpressionConvertType = EMaterialExpressionConvertType(
        1,
    );
    pub const VECTOR3: EMaterialExpressionConvertType = EMaterialExpressionConvertType(
        2,
    );
    pub const VECTOR4: EMaterialExpressionConvertType = EMaterialExpressionConvertType(
        3,
    );
}
#[repr(transparent)]
pub struct ECustomMaterialOutputType(pub u8);
impl ECustomMaterialOutputType {
    pub const CMOT_FLOAT1: ECustomMaterialOutputType = ECustomMaterialOutputType(0);
    pub const CMOT_FLOAT2: ECustomMaterialOutputType = ECustomMaterialOutputType(1);
    pub const CMOT_FLOAT3: ECustomMaterialOutputType = ECustomMaterialOutputType(2);
    pub const CMOT_FLOAT4: ECustomMaterialOutputType = ECustomMaterialOutputType(3);
    pub const CMOT_MATERIAL_ATTRIBUTES: ECustomMaterialOutputType = ECustomMaterialOutputType(
        4,
    );
}
#[repr(transparent)]
pub struct EDataDrivenShaderPlatformInfoCondition(pub u8);
impl EDataDrivenShaderPlatformInfoCondition {
    pub const COND_TRUE: EDataDrivenShaderPlatformInfoCondition = EDataDrivenShaderPlatformInfoCondition(
        0,
    );
    pub const COND_FALSE: EDataDrivenShaderPlatformInfoCondition = EDataDrivenShaderPlatformInfoCondition(
        1,
    );
}
#[repr(transparent)]
pub struct EBlendMode(pub u8);
impl EBlendMode {
    pub const BLEND_OPAQUE: EBlendMode = EBlendMode(0);
    pub const BLEND_MASKED: EBlendMode = EBlendMode(1);
    pub const BLEND_TRANSLUCENT: EBlendMode = EBlendMode(2);
    pub const BLEND_ADDITIVE: EBlendMode = EBlendMode(3);
    pub const BLEND_MODULATE: EBlendMode = EBlendMode(4);
    pub const BLEND_ALPHA_COMPOSITE: EBlendMode = EBlendMode(5);
    pub const BLEND_ALPHA_HOLDOUT: EBlendMode = EBlendMode(6);
    pub const BLEND_TRANSLUCENT_COLORED_TRANSMITTANCE: EBlendMode = EBlendMode(7);
    pub const BLEND_TRANSLUCENT_GREY_TRANSMITTANCE: EBlendMode = EBlendMode(2);
    pub const BLEND_COLORED_TRANSMITTANCE_ONLY: EBlendMode = EBlendMode(4);
}
#[repr(transparent)]
pub struct EMaterialShadingModel(pub u8);
impl EMaterialShadingModel {
    pub const MSM_UNLIT: EMaterialShadingModel = EMaterialShadingModel(0);
    pub const MSM_DEFAULT_LIT: EMaterialShadingModel = EMaterialShadingModel(1);
    pub const MSM_SUBSURFACE: EMaterialShadingModel = EMaterialShadingModel(2);
    pub const MSM_PREINTEGRATED_SKIN: EMaterialShadingModel = EMaterialShadingModel(3);
    pub const MSM_CLEAR_COAT: EMaterialShadingModel = EMaterialShadingModel(4);
    pub const MSM_SUBSURFACE_PROFILE: EMaterialShadingModel = EMaterialShadingModel(5);
    pub const MSM_TWO_SIDED_FOLIAGE: EMaterialShadingModel = EMaterialShadingModel(6);
    pub const MSM_HAIR: EMaterialShadingModel = EMaterialShadingModel(7);
    pub const MSM_CLOTH: EMaterialShadingModel = EMaterialShadingModel(8);
    pub const MSM_EYE: EMaterialShadingModel = EMaterialShadingModel(9);
    pub const MSM_SINGLE_LAYER_WATER: EMaterialShadingModel = EMaterialShadingModel(10);
    pub const MSM_THIN_TRANSLUCENT: EMaterialShadingModel = EMaterialShadingModel(11);
    pub const MSM_STRATA: EMaterialShadingModel = EMaterialShadingModel(12);
    pub const MSM_NUM: EMaterialShadingModel = EMaterialShadingModel(13);
    pub const MSM_FROM_MATERIAL_EXPRESSION: EMaterialShadingModel = EMaterialShadingModel(
        14,
    );
}
#[repr(transparent)]
pub struct EMaterialLayerLinkState(pub u8);
impl EMaterialLayerLinkState {
    pub const UNINITIALIZED: EMaterialLayerLinkState = EMaterialLayerLinkState(0);
    pub const LINKED_TO_PARENT: EMaterialLayerLinkState = EMaterialLayerLinkState(1);
    pub const UNLINKED_FROM_PARENT: EMaterialLayerLinkState = EMaterialLayerLinkState(2);
    pub const NOT_FROM_PARENT: EMaterialLayerLinkState = EMaterialLayerLinkState(3);
}
#[repr(transparent)]
pub struct ENaniteAssemblyNodeTransformSpace(pub u8);
impl ENaniteAssemblyNodeTransformSpace {
    pub const LOCAL: ENaniteAssemblyNodeTransformSpace = ENaniteAssemblyNodeTransformSpace(
        0,
    );
    pub const BONE_RELATIVE: ENaniteAssemblyNodeTransformSpace = ENaniteAssemblyNodeTransformSpace(
        1,
    );
}
#[repr(transparent)]
pub struct ENavLinkDirection(pub u8);
impl ENavLinkDirection {
    pub const BOTH_WAYS: ENavLinkDirection = ENavLinkDirection(0);
    pub const LEFT_TO_RIGHT: ENavLinkDirection = ENavLinkDirection(1);
    pub const RIGHT_TO_LEFT: ENavLinkDirection = ENavLinkDirection(2);
}
#[repr(transparent)]
pub struct EParticleEventType(pub u8);
impl EParticleEventType {
    pub const EPET_ANY: EParticleEventType = EParticleEventType(0);
    pub const EPET_SPAWN: EParticleEventType = EParticleEventType(1);
    pub const EPET_DEATH: EParticleEventType = EParticleEventType(2);
    pub const EPET_COLLISION: EParticleEventType = EParticleEventType(3);
    pub const EPET_BURST: EParticleEventType = EParticleEventType(4);
    pub const EPET_BLUEPRINT: EParticleEventType = EParticleEventType(5);
}
#[repr(transparent)]
pub struct EEmitterDynamicParameterValue(pub u8);
impl EEmitterDynamicParameterValue {
    pub const EDPV_USER_SET: EEmitterDynamicParameterValue = EEmitterDynamicParameterValue(
        0,
    );
    pub const EDPV_AUTO_SET: EEmitterDynamicParameterValue = EEmitterDynamicParameterValue(
        1,
    );
    pub const EDPV_VELOCITY_X: EEmitterDynamicParameterValue = EEmitterDynamicParameterValue(
        2,
    );
    pub const EDPV_VELOCITY_Y: EEmitterDynamicParameterValue = EEmitterDynamicParameterValue(
        3,
    );
    pub const EDPV_VELOCITY_Z: EEmitterDynamicParameterValue = EEmitterDynamicParameterValue(
        4,
    );
    pub const EDPV_VELOCITY_MAG: EEmitterDynamicParameterValue = EEmitterDynamicParameterValue(
        5,
    );
}
#[repr(transparent)]
pub struct EParticleScreenAlignment(pub u8);
impl EParticleScreenAlignment {
    pub const PSA_FACING_CAMERA_POSITION: EParticleScreenAlignment = EParticleScreenAlignment(
        0,
    );
    pub const PSA_SQUARE: EParticleScreenAlignment = EParticleScreenAlignment(1);
    pub const PSA_RECTANGLE: EParticleScreenAlignment = EParticleScreenAlignment(2);
    pub const PSA_VELOCITY: EParticleScreenAlignment = EParticleScreenAlignment(3);
    pub const PSA_AWAY_FROM_CENTER: EParticleScreenAlignment = EParticleScreenAlignment(
        4,
    );
    pub const PSA_TYPE_SPECIFIC: EParticleScreenAlignment = EParticleScreenAlignment(5);
    pub const PSA_FACING_CAMERA_DISTANCE_BLEND: EParticleScreenAlignment = EParticleScreenAlignment(
        6,
    );
}
#[repr(transparent)]
pub struct EParticleAxisLock(pub u8);
impl EParticleAxisLock {
    pub const EPAL_NONE: EParticleAxisLock = EParticleAxisLock(0);
    pub const EPAL_X: EParticleAxisLock = EParticleAxisLock(1);
    pub const EPAL_Y: EParticleAxisLock = EParticleAxisLock(2);
    pub const EPAL_Z: EParticleAxisLock = EParticleAxisLock(3);
    pub const EPAL_NEGATIVE_X: EParticleAxisLock = EParticleAxisLock(4);
    pub const EPAL_NEGATIVE_Y: EParticleAxisLock = EParticleAxisLock(5);
    pub const EPAL_NEGATIVE_Z: EParticleAxisLock = EParticleAxisLock(6);
    pub const EPAL_ROTATE_X: EParticleAxisLock = EParticleAxisLock(7);
    pub const EPAL_ROTATE_Y: EParticleAxisLock = EParticleAxisLock(8);
    pub const EPAL_ROTATE_Z: EParticleAxisLock = EParticleAxisLock(9);
}
#[repr(transparent)]
pub struct EParticleCollisionMode(pub u8);
impl EParticleCollisionMode {
    pub const SCENE_DEPTH: EParticleCollisionMode = EParticleCollisionMode(0);
    pub const DISTANCE_FIELD: EParticleCollisionMode = EParticleCollisionMode(1);
}
#[repr(transparent)]
pub struct SkeletalMeshTerminationCriterion(pub u8);
impl SkeletalMeshTerminationCriterion {
    pub const SMTC_NUM_OF_TRIANGLES: SkeletalMeshTerminationCriterion = SkeletalMeshTerminationCriterion(
        0,
    );
    pub const SMTC_NUM_OF_VERTS: SkeletalMeshTerminationCriterion = SkeletalMeshTerminationCriterion(
        1,
    );
    pub const SMTC_TRIANGLE_OR_VERT: SkeletalMeshTerminationCriterion = SkeletalMeshTerminationCriterion(
        2,
    );
    pub const SMTC_ABS_NUM_OF_TRIANGLES: SkeletalMeshTerminationCriterion = SkeletalMeshTerminationCriterion(
        3,
    );
    pub const SMTC_ABS_NUM_OF_VERTS: SkeletalMeshTerminationCriterion = SkeletalMeshTerminationCriterion(
        4,
    );
    pub const SMTC_ABS_TRIANGLE_OR_VERT: SkeletalMeshTerminationCriterion = SkeletalMeshTerminationCriterion(
        5,
    );
}
#[repr(transparent)]
pub struct SkeletalMeshOptimizationType(pub u8);
impl SkeletalMeshOptimizationType {
    pub const SMOT_NUM_OF_TRIANGLES: SkeletalMeshOptimizationType = SkeletalMeshOptimizationType(
        0,
    );
    pub const SMOT_TRIANGLE_OR_DEVIATION: SkeletalMeshOptimizationType = SkeletalMeshOptimizationType(
        2,
    );
}
#[repr(transparent)]
pub struct SkeletalMeshOptimizationImportance(pub u8);
impl SkeletalMeshOptimizationImportance {
    pub const SMOI_OFF: SkeletalMeshOptimizationImportance = SkeletalMeshOptimizationImportance(
        0,
    );
    pub const SMOI_LOWEST: SkeletalMeshOptimizationImportance = SkeletalMeshOptimizationImportance(
        1,
    );
    pub const SMOI_LOW: SkeletalMeshOptimizationImportance = SkeletalMeshOptimizationImportance(
        2,
    );
    pub const SMOI_NORMAL: SkeletalMeshOptimizationImportance = SkeletalMeshOptimizationImportance(
        3,
    );
    pub const SMOI_HIGH: SkeletalMeshOptimizationImportance = SkeletalMeshOptimizationImportance(
        4,
    );
    pub const SMOI_HIGHEST: SkeletalMeshOptimizationImportance = SkeletalMeshOptimizationImportance(
        5,
    );
}
#[repr(transparent)]
pub struct ESkyAtmosphereTransformMode(pub u8);
impl ESkyAtmosphereTransformMode {
    pub const PLANET_TOP_AT_ABSOLUTE_WORLD_ORIGIN: ESkyAtmosphereTransformMode = ESkyAtmosphereTransformMode(
        0,
    );
    pub const PLANET_TOP_AT_COMPONENT_TRANSFORM: ESkyAtmosphereTransformMode = ESkyAtmosphereTransformMode(
        1,
    );
    pub const PLANET_CENTER_AT_COMPONENT_TRANSFORM: ESkyAtmosphereTransformMode = ESkyAtmosphereTransformMode(
        2,
    );
}
#[repr(transparent)]
pub struct ESourceBusSendLevelControlMethod(pub u8);
impl ESourceBusSendLevelControlMethod {
    pub const LINEAR: ESourceBusSendLevelControlMethod = ESourceBusSendLevelControlMethod(
        0,
    );
    pub const CUSTOM_CURVE: ESourceBusSendLevelControlMethod = ESourceBusSendLevelControlMethod(
        1,
    );
    pub const MANUAL: ESourceBusSendLevelControlMethod = ESourceBusSendLevelControlMethod(
        2,
    );
}
#[repr(transparent)]
pub struct ETimelineLengthMode(pub u8);
impl ETimelineLengthMode {
    pub const TL_TIMELINE_LENGTH: ETimelineLengthMode = ETimelineLengthMode(0);
    pub const TL_LAST_KEY_FRAME: ETimelineLengthMode = ETimelineLengthMode(1);
}
#[repr(transparent)]
pub struct EQuartzCommandQuantization(pub u8);
impl EQuartzCommandQuantization {
    pub const BAR: EQuartzCommandQuantization = EQuartzCommandQuantization(0);
    pub const BEAT: EQuartzCommandQuantization = EQuartzCommandQuantization(1);
    pub const THIRTY_SECOND_NOTE: EQuartzCommandQuantization = EQuartzCommandQuantization(
        2,
    );
    pub const SIXTEENTH_NOTE: EQuartzCommandQuantization = EQuartzCommandQuantization(3);
    pub const EIGHTH_NOTE: EQuartzCommandQuantization = EQuartzCommandQuantization(4);
    pub const QUARTER_NOTE: EQuartzCommandQuantization = EQuartzCommandQuantization(5);
    pub const HALF_NOTE: EQuartzCommandQuantization = EQuartzCommandQuantization(6);
    pub const WHOLE_NOTE: EQuartzCommandQuantization = EQuartzCommandQuantization(7);
    pub const DOTTED_SIXTEENTH_NOTE: EQuartzCommandQuantization = EQuartzCommandQuantization(
        8,
    );
    pub const DOTTED_EIGHTH_NOTE: EQuartzCommandQuantization = EQuartzCommandQuantization(
        9,
    );
    pub const DOTTED_QUARTER_NOTE: EQuartzCommandQuantization = EQuartzCommandQuantization(
        10,
    );
    pub const DOTTED_HALF_NOTE: EQuartzCommandQuantization = EQuartzCommandQuantization(
        11,
    );
    pub const DOTTED_WHOLE_NOTE: EQuartzCommandQuantization = EQuartzCommandQuantization(
        12,
    );
    pub const SIXTEENTH_NOTE_TRIPLET: EQuartzCommandQuantization = EQuartzCommandQuantization(
        13,
    );
    pub const EIGHTH_NOTE_TRIPLET: EQuartzCommandQuantization = EQuartzCommandQuantization(
        14,
    );
    pub const QUARTER_NOTE_TRIPLET: EQuartzCommandQuantization = EQuartzCommandQuantization(
        15,
    );
    pub const HALF_NOTE_TRIPLET: EQuartzCommandQuantization = EQuartzCommandQuantization(
        16,
    );
    pub const TICK: EQuartzCommandQuantization = EQuartzCommandQuantization(17);
    pub const COUNT: EQuartzCommandQuantization = EQuartzCommandQuantization(18);
    pub const NONE: EQuartzCommandQuantization = EQuartzCommandQuantization(19);
}
#[repr(transparent)]
pub struct EQuartzTimeSignatureQuantization(pub u8);
impl EQuartzTimeSignatureQuantization {
    pub const HALF_NOTE: EQuartzTimeSignatureQuantization = EQuartzTimeSignatureQuantization(
        0,
    );
    pub const QUARTER_NOTE: EQuartzTimeSignatureQuantization = EQuartzTimeSignatureQuantization(
        1,
    );
    pub const EIGHTH_NOTE: EQuartzTimeSignatureQuantization = EQuartzTimeSignatureQuantization(
        2,
    );
    pub const SIXTEENTH_NOTE: EQuartzTimeSignatureQuantization = EQuartzTimeSignatureQuantization(
        3,
    );
    pub const THIRTY_SECOND_NOTE: EQuartzTimeSignatureQuantization = EQuartzTimeSignatureQuantization(
        4,
    );
    pub const COUNT: EQuartzTimeSignatureQuantization = EQuartzTimeSignatureQuantization(
        5,
    );
}
#[repr(transparent)]
pub struct EQuarztQuantizationReference(pub u8);
impl EQuarztQuantizationReference {
    pub const BAR_RELATIVE: EQuarztQuantizationReference = EQuarztQuantizationReference(
        0,
    );
    pub const TRANSPORT_RELATIVE: EQuarztQuantizationReference = EQuarztQuantizationReference(
        1,
    );
    pub const CURRENT_TIME_RELATIVE: EQuarztQuantizationReference = EQuarztQuantizationReference(
        2,
    );
    pub const COUNT: EQuarztQuantizationReference = EQuarztQuantizationReference(3);
}
#[repr(transparent)]
pub struct EComponentMobility(pub u8);
impl EComponentMobility {
    pub const STATIC: EComponentMobility = EComponentMobility(0);
    pub const STATIONARY: EComponentMobility = EComponentMobility(1);
    pub const MOVABLE: EComponentMobility = EComponentMobility(2);
}
#[repr(transparent)]
pub struct EHLODBatchingPolicy(pub u8);
impl EHLODBatchingPolicy {
    pub const NONE: EHLODBatchingPolicy = EHLODBatchingPolicy(0);
    pub const MESH_SECTION: EHLODBatchingPolicy = EHLODBatchingPolicy(1);
    pub const INSTANCING: EHLODBatchingPolicy = EHLODBatchingPolicy(2);
}
#[repr(transparent)]
pub struct EAnimLinkMethod(pub u8);
impl EAnimLinkMethod {
    pub const ABSOLUTE: EAnimLinkMethod = EAnimLinkMethod(0);
    pub const RELATIVE: EAnimLinkMethod = EAnimLinkMethod(1);
    pub const PROPORTIONAL: EAnimLinkMethod = EAnimLinkMethod(2);
}
#[repr(transparent)]
pub struct EAnimNotifyEventType(pub u8);
impl EAnimNotifyEventType {
    pub const BEGIN: EAnimNotifyEventType = EAnimNotifyEventType(0);
    pub const END: EAnimNotifyEventType = EAnimNotifyEventType(1);
}
#[repr(transparent)]
pub struct EMontageBlendMode(pub u8);
impl EMontageBlendMode {
    pub const STANDARD: EMontageBlendMode = EMontageBlendMode(0);
    pub const INERTIALIZATION: EMontageBlendMode = EMontageBlendMode(1);
}
#[repr(transparent)]
pub struct EMontageNotifyTickType(pub u8);
impl EMontageNotifyTickType {
    pub const QUEUED: EMontageNotifyTickType = EMontageNotifyTickType(0);
    pub const BRANCHING_POINT: EMontageNotifyTickType = EMontageNotifyTickType(1);
}
#[repr(transparent)]
pub struct ENotifyFilterType(pub u8);
impl ENotifyFilterType {
    pub const NO_FILTERING: ENotifyFilterType = ENotifyFilterType(0);
    pub const LOD: ENotifyFilterType = ENotifyFilterType(1);
}
#[repr(transparent)]
pub struct EEvaluatorDataSource(pub u8);
impl EEvaluatorDataSource {
    pub const EDS_SOURCE_POSE: EEvaluatorDataSource = EEvaluatorDataSource(0);
    pub const EDS_DESTINATION_POSE: EEvaluatorDataSource = EEvaluatorDataSource(1);
}
#[repr(transparent)]
pub struct EEvaluatorMode(pub u8);
impl EEvaluatorMode {
    pub const EM_STANDARD: EEvaluatorMode = EEvaluatorMode(0);
    pub const EM_FREEZE: EEvaluatorMode = EEvaluatorMode(1);
    pub const EM_DELAYED_FREEZE: EEvaluatorMode = EEvaluatorMode(2);
}
#[repr(transparent)]
pub struct ETransitionLogicType(pub u8);
impl ETransitionLogicType {
    pub const TLT_STANDARD_BLEND: ETransitionLogicType = ETransitionLogicType(0);
    pub const TLT_INERTIALIZATION: ETransitionLogicType = ETransitionLogicType(1);
    pub const TLT_CUSTOM: ETransitionLogicType = ETransitionLogicType(2);
}
#[repr(transparent)]
pub struct EPropertyAccessCopyType(pub u8);
impl EPropertyAccessCopyType {
    pub const NONE: EPropertyAccessCopyType = EPropertyAccessCopyType(0);
    pub const PLAIN: EPropertyAccessCopyType = EPropertyAccessCopyType(1);
    pub const COMPLEX: EPropertyAccessCopyType = EPropertyAccessCopyType(2);
    pub const BOOL: EPropertyAccessCopyType = EPropertyAccessCopyType(3);
    pub const STRUCT: EPropertyAccessCopyType = EPropertyAccessCopyType(4);
    pub const OBJECT: EPropertyAccessCopyType = EPropertyAccessCopyType(5);
    pub const NAME: EPropertyAccessCopyType = EPropertyAccessCopyType(6);
    pub const ARRAY: EPropertyAccessCopyType = EPropertyAccessCopyType(7);
    pub const PROMOTE_BOOL_TO_BYTE: EPropertyAccessCopyType = EPropertyAccessCopyType(8);
    pub const PROMOTE_BOOL_TO_INT32: EPropertyAccessCopyType = EPropertyAccessCopyType(
        9,
    );
    pub const PROMOTE_BOOL_TO_INT64: EPropertyAccessCopyType = EPropertyAccessCopyType(
        10,
    );
    pub const PROMOTE_BOOL_TO_FLOAT: EPropertyAccessCopyType = EPropertyAccessCopyType(
        11,
    );
    pub const PROMOTE_BOOL_TO_DOUBLE: EPropertyAccessCopyType = EPropertyAccessCopyType(
        12,
    );
    pub const PROMOTE_BYTE_TO_INT32: EPropertyAccessCopyType = EPropertyAccessCopyType(
        13,
    );
    pub const PROMOTE_BYTE_TO_INT64: EPropertyAccessCopyType = EPropertyAccessCopyType(
        14,
    );
    pub const PROMOTE_BYTE_TO_FLOAT: EPropertyAccessCopyType = EPropertyAccessCopyType(
        15,
    );
    pub const PROMOTE_BYTE_TO_DOUBLE: EPropertyAccessCopyType = EPropertyAccessCopyType(
        16,
    );
    pub const PROMOTE_INT32_TO_INT64: EPropertyAccessCopyType = EPropertyAccessCopyType(
        17,
    );
    pub const PROMOTE_INT32_TO_FLOAT: EPropertyAccessCopyType = EPropertyAccessCopyType(
        18,
    );
    pub const PROMOTE_INT32_TO_DOUBLE: EPropertyAccessCopyType = EPropertyAccessCopyType(
        19,
    );
    pub const PROMOTE_FLOAT_TO_DOUBLE: EPropertyAccessCopyType = EPropertyAccessCopyType(
        20,
    );
    pub const DEMOTE_DOUBLE_TO_FLOAT: EPropertyAccessCopyType = EPropertyAccessCopyType(
        21,
    );
    pub const PROMOTE_ARRAY_FLOAT_TO_DOUBLE: EPropertyAccessCopyType = EPropertyAccessCopyType(
        22,
    );
    pub const DEMOTE_ARRAY_DOUBLE_TO_FLOAT: EPropertyAccessCopyType = EPropertyAccessCopyType(
        23,
    );
    pub const PROMOTE_MAP_VALUE_FLOAT_TO_DOUBLE: EPropertyAccessCopyType = EPropertyAccessCopyType(
        24,
    );
    pub const DEMOTE_MAP_VALUE_DOUBLE_TO_FLOAT: EPropertyAccessCopyType = EPropertyAccessCopyType(
        25,
    );
}
#[repr(transparent)]
pub struct EFilterInterpolationType(pub u8);
impl EFilterInterpolationType {
    pub const BSIT_AVERAGE: EFilterInterpolationType = EFilterInterpolationType(0);
    pub const BSIT_LINEAR: EFilterInterpolationType = EFilterInterpolationType(1);
    pub const BSIT_CUBIC: EFilterInterpolationType = EFilterInterpolationType(2);
    pub const BSIT_EASE_IN_OUT: EFilterInterpolationType = EFilterInterpolationType(3);
    pub const BSIT_EXPONENTIAL_DECAY: EFilterInterpolationType = EFilterInterpolationType(
        4,
    );
    pub const BSIT_SPRING_DAMPER: EFilterInterpolationType = EFilterInterpolationType(5);
}
#[repr(transparent)]
pub struct EPostCopyOperation(pub u8);
impl EPostCopyOperation {
    pub const NONE: EPostCopyOperation = EPostCopyOperation(0);
    pub const LOGICAL_NEGATE_BOOL: EPostCopyOperation = EPostCopyOperation(1);
}
#[repr(transparent)]
pub struct EMirrorRowType(pub u8);
impl EMirrorRowType {
    pub const BONE: EMirrorRowType = EMirrorRowType(0);
    pub const ANIMATION_NOTIFY: EMirrorRowType = EMirrorRowType(1);
    pub const CURVE: EMirrorRowType = EMirrorRowType(2);
    pub const SYNC_MARKER: EMirrorRowType = EMirrorRowType(3);
    pub const CUSTOM: EMirrorRowType = EMirrorRowType(4);
}
#[repr(transparent)]
pub struct EMirrorFindReplaceMethod(pub u8);
impl EMirrorFindReplaceMethod {
    pub const PREFIX: EMirrorFindReplaceMethod = EMirrorFindReplaceMethod(0);
    pub const SUFFIX: EMirrorFindReplaceMethod = EMirrorFindReplaceMethod(1);
    pub const REGULAR_EXPRESSION: EMirrorFindReplaceMethod = EMirrorFindReplaceMethod(2);
}
#[repr(transparent)]
pub struct ESkeletalMeshVertexAttributeDataType(pub i32);
impl ESkeletalMeshVertexAttributeDataType {
    pub const FLOAT: ESkeletalMeshVertexAttributeDataType = ESkeletalMeshVertexAttributeDataType(
        0,
    );
    pub const HALF_FLOAT: ESkeletalMeshVertexAttributeDataType = ESkeletalMeshVertexAttributeDataType(
        1,
    );
    pub const NU_INT8: ESkeletalMeshVertexAttributeDataType = ESkeletalMeshVertexAttributeDataType(
        2,
    );
}
#[repr(transparent)]
pub struct EBoneTranslationRetargetingMode(pub u8);
impl EBoneTranslationRetargetingMode {
    pub const ANIMATION: EBoneTranslationRetargetingMode = EBoneTranslationRetargetingMode(
        0,
    );
    pub const SKELETON: EBoneTranslationRetargetingMode = EBoneTranslationRetargetingMode(
        1,
    );
    pub const ANIMATION_SCALED: EBoneTranslationRetargetingMode = EBoneTranslationRetargetingMode(
        2,
    );
    pub const ANIMATION_RELATIVE: EBoneTranslationRetargetingMode = EBoneTranslationRetargetingMode(
        3,
    );
    pub const ORIENT_AND_SCALE: EBoneTranslationRetargetingMode = EBoneTranslationRetargetingMode(
        4,
    );
}
#[repr(transparent)]
pub struct EAudioVolumeLocationState(pub u8);
impl EAudioVolumeLocationState {
    pub const INSIDE_THE_VOLUME: EAudioVolumeLocationState = EAudioVolumeLocationState(
        0,
    );
    pub const OUTSIDE_THE_VOLUME: EAudioVolumeLocationState = EAudioVolumeLocationState(
        1,
    );
}
#[repr(transparent)]
pub struct ESubmixSendStage(pub u8);
impl ESubmixSendStage {
    pub const POST_DISTANCE_ATTENUATION: ESubmixSendStage = ESubmixSendStage(0);
    pub const PRE_DISTANCE_ATTENUATION: ESubmixSendStage = ESubmixSendStage(1);
}
#[repr(transparent)]
pub struct ESendLevelControlMethod(pub u8);
impl ESendLevelControlMethod {
    pub const LINEAR: ESendLevelControlMethod = ESendLevelControlMethod(0);
    pub const CUSTOM_CURVE: ESendLevelControlMethod = ESendLevelControlMethod(1);
    pub const MANUAL: ESendLevelControlMethod = ESendLevelControlMethod(2);
}
#[repr(transparent)]
pub struct ECameraShakeDurationType(pub u8);
impl ECameraShakeDurationType {
    pub const FIXED: ECameraShakeDurationType = ECameraShakeDurationType(0);
    pub const INFINITE: ECameraShakeDurationType = ECameraShakeDurationType(1);
    pub const CUSTOM: ECameraShakeDurationType = ECameraShakeDurationType(2);
}
#[repr(transparent)]
pub struct ESyncOption(pub u8);
impl ESyncOption {
    pub const DRIVE: ESyncOption = ESyncOption(0);
    pub const PASSIVE: ESyncOption = ESyncOption(1);
    pub const DISABLED: ESyncOption = ESyncOption(2);
}
#[repr(transparent)]
pub struct ESplinePointType(pub u8);
impl ESplinePointType {
    pub const LINEAR: ESplinePointType = ESplinePointType(0);
    pub const CURVE: ESplinePointType = ESplinePointType(1);
    pub const CONSTANT: ESplinePointType = ESplinePointType(2);
    pub const CURVE_CLAMPED: ESplinePointType = ESplinePointType(3);
    pub const CURVE_CUSTOM_TANGENT: ESplinePointType = ESplinePointType(4);
}
#[repr(transparent)]
pub struct FDataDrivenCVarType(pub u8);
impl FDataDrivenCVarType {
    pub const C_VAR_FLOAT: FDataDrivenCVarType = FDataDrivenCVarType(0);
    pub const C_VAR_INT: FDataDrivenCVarType = FDataDrivenCVarType(1);
    pub const C_VAR_BOOL: FDataDrivenCVarType = FDataDrivenCVarType(2);
}
#[repr(transparent)]
pub struct EUpdateRateShiftBucket(pub u8);
impl EUpdateRateShiftBucket {
    pub const SHIFT_BUCKET0: EUpdateRateShiftBucket = EUpdateRateShiftBucket(0);
    pub const SHIFT_BUCKET1: EUpdateRateShiftBucket = EUpdateRateShiftBucket(1);
    pub const SHIFT_BUCKET2: EUpdateRateShiftBucket = EUpdateRateShiftBucket(2);
    pub const SHIFT_BUCKET3: EUpdateRateShiftBucket = EUpdateRateShiftBucket(3);
    pub const SHIFT_BUCKET4: EUpdateRateShiftBucket = EUpdateRateShiftBucket(4);
    pub const SHIFT_BUCKET5: EUpdateRateShiftBucket = EUpdateRateShiftBucket(5);
    pub const SHIFT_BUCKET_MAX: EUpdateRateShiftBucket = EUpdateRateShiftBucket(6);
}
#[repr(transparent)]
pub struct ENaniteShapePreservation(pub u8);
impl ENaniteShapePreservation {
    pub const NONE: ENaniteShapePreservation = ENaniteShapePreservation(0);
    pub const PRESERVE_AREA: ENaniteShapePreservation = ENaniteShapePreservation(1);
    pub const VOXELIZE: ENaniteShapePreservation = ENaniteShapePreservation(2);
}
#[repr(transparent)]
pub struct ENaniteGenerateFallback(pub u8);
impl ENaniteGenerateFallback {
    pub const PLATFORM_DEFAULT: ENaniteGenerateFallback = ENaniteGenerateFallback(0);
    pub const ENABLED: ENaniteGenerateFallback = ENaniteGenerateFallback(1);
}
#[repr(transparent)]
pub struct ENaniteFallbackTarget(pub u8);
impl ENaniteFallbackTarget {
    pub const AUTO: ENaniteFallbackTarget = ENaniteFallbackTarget(0);
    pub const PERCENT_TRIANGLES: ENaniteFallbackTarget = ENaniteFallbackTarget(1);
    pub const RELATIVE_ERROR: ENaniteFallbackTarget = ENaniteFallbackTarget(2);
}
#[repr(transparent)]
pub struct ERootMotionFinishVelocityMode(pub u8);
impl ERootMotionFinishVelocityMode {
    pub const MAINTAIN_LAST_ROOT_MOTION_VELOCITY: ERootMotionFinishVelocityMode = ERootMotionFinishVelocityMode(
        0,
    );
    pub const SET_VELOCITY: ERootMotionFinishVelocityMode = ERootMotionFinishVelocityMode(
        1,
    );
    pub const CLAMP_VELOCITY: ERootMotionFinishVelocityMode = ERootMotionFinishVelocityMode(
        2,
    );
}
#[repr(transparent)]
pub struct ERootMotionAccumulateMode(pub u8);
impl ERootMotionAccumulateMode {
    pub const OVERRIDE: ERootMotionAccumulateMode = ERootMotionAccumulateMode(0);
    pub const ADDITIVE: ERootMotionAccumulateMode = ERootMotionAccumulateMode(1);
}
#[repr(transparent)]
pub struct EHierarchicalSimplificationMethod(pub u8);
impl EHierarchicalSimplificationMethod {
    pub const NONE: EHierarchicalSimplificationMethod = EHierarchicalSimplificationMethod(
        0,
    );
    pub const MERGE: EHierarchicalSimplificationMethod = EHierarchicalSimplificationMethod(
        1,
    );
    pub const SIMPLIFY: EHierarchicalSimplificationMethod = EHierarchicalSimplificationMethod(
        2,
    );
    pub const APPROXIMATE: EHierarchicalSimplificationMethod = EHierarchicalSimplificationMethod(
        3,
    );
}
#[repr(transparent)]
pub struct ETextureSizingType(pub u8);
impl ETextureSizingType {
    pub const TEXTURE_SIZING_TYPE_USE_SINGLE_TEXTURE_SIZE: ETextureSizingType = ETextureSizingType(
        0,
    );
    pub const TEXTURE_SIZING_TYPE_USE_AUTOMATIC_BIASED_SIZES: ETextureSizingType = ETextureSizingType(
        1,
    );
    pub const TEXTURE_SIZING_TYPE_USE_MANUAL_OVERRIDE_TEXTURE_SIZE: ETextureSizingType = ETextureSizingType(
        2,
    );
    pub const TEXTURE_SIZING_TYPE_USE_SIMPLYGON_AUTOMATIC_SIZING: ETextureSizingType = ETextureSizingType(
        3,
    );
    pub const TEXTURE_SIZING_TYPE_AUTOMATIC_FROM_TEXEL_DENSITY: ETextureSizingType = ETextureSizingType(
        4,
    );
    pub const TEXTURE_SIZING_TYPE_AUTOMATIC_FROM_MESH_SCREEN_SIZE: ETextureSizingType = ETextureSizingType(
        5,
    );
    pub const TEXTURE_SIZING_TYPE_AUTOMATIC_FROM_MESH_DRAW_DISTANCE: ETextureSizingType = ETextureSizingType(
        6,
    );
}
#[repr(transparent)]
pub struct EUVOutput(pub u8);
impl EUVOutput {
    pub const DO_NOT_OUTPUT_CHANNEL: EUVOutput = EUVOutput(0);
    pub const OUTPUT_CHANNEL: EUVOutput = EUVOutput(1);
}
#[repr(transparent)]
pub struct EMeshApproximationType(pub u8);
impl EMeshApproximationType {
    pub const MESH_AND_MATERIALS: EMeshApproximationType = EMeshApproximationType(0);
    pub const MESH_SHAPE_ONLY: EMeshApproximationType = EMeshApproximationType(1);
}
#[repr(transparent)]
pub struct EMeshApproximationBaseCappingType(pub u8);
impl EMeshApproximationBaseCappingType {
    pub const NO_BASE_CAPPING: EMeshApproximationBaseCappingType = EMeshApproximationBaseCappingType(
        0,
    );
    pub const CONVEX_POLYGON: EMeshApproximationBaseCappingType = EMeshApproximationBaseCappingType(
        1,
    );
    pub const CONVEX_SOLID: EMeshApproximationBaseCappingType = EMeshApproximationBaseCappingType(
        2,
    );
}
#[repr(transparent)]
pub struct EOccludedGeometryFilteringPolicy(pub u8);
impl EOccludedGeometryFilteringPolicy {
    pub const NO_OCCLUSION_FILTERING: EOccludedGeometryFilteringPolicy = EOccludedGeometryFilteringPolicy(
        0,
    );
    pub const VISIBILITY_BASED_FILTERING: EOccludedGeometryFilteringPolicy = EOccludedGeometryFilteringPolicy(
        1,
    );
}
#[repr(transparent)]
pub struct EMeshApproximationSimplificationPolicy(pub u8);
impl EMeshApproximationSimplificationPolicy {
    pub const FIXED_TRIANGLE_COUNT: EMeshApproximationSimplificationPolicy = EMeshApproximationSimplificationPolicy(
        0,
    );
    pub const TRIANGLES_PER_AREA: EMeshApproximationSimplificationPolicy = EMeshApproximationSimplificationPolicy(
        1,
    );
    pub const GEOMETRIC_TOLERANCE: EMeshApproximationSimplificationPolicy = EMeshApproximationSimplificationPolicy(
        2,
    );
}
#[repr(transparent)]
pub struct EMeshApproximationGroundPlaneClippingPolicy(pub u8);
impl EMeshApproximationGroundPlaneClippingPolicy {
    pub const NO_GROUND_CLIPPING: EMeshApproximationGroundPlaneClippingPolicy = EMeshApproximationGroundPlaneClippingPolicy(
        0,
    );
    pub const DISCARD_WITH_Z_PLANE: EMeshApproximationGroundPlaneClippingPolicy = EMeshApproximationGroundPlaneClippingPolicy(
        1,
    );
    pub const CUT_WITH_Z_PLANE: EMeshApproximationGroundPlaneClippingPolicy = EMeshApproximationGroundPlaneClippingPolicy(
        2,
    );
    pub const CUT_AND_FILL_WITH_Z_PLANE: EMeshApproximationGroundPlaneClippingPolicy = EMeshApproximationGroundPlaneClippingPolicy(
        3,
    );
}
#[repr(transparent)]
pub struct EMeshApproximationUVGenerationPolicy(pub u8);
impl EMeshApproximationUVGenerationPolicy {
    pub const PREFER_UV_ATLAS: EMeshApproximationUVGenerationPolicy = EMeshApproximationUVGenerationPolicy(
        0,
    );
    pub const PREFER_X_ATLAS: EMeshApproximationUVGenerationPolicy = EMeshApproximationUVGenerationPolicy(
        1,
    );
    pub const PREFER_PATCH_BUILDER: EMeshApproximationUVGenerationPolicy = EMeshApproximationUVGenerationPolicy(
        2,
    );
}
#[repr(transparent)]
pub struct EMaterialMergeType(pub u8);
impl EMaterialMergeType {
    pub const MATERIAL_MERGE_TYPE_DEFAULT: EMaterialMergeType = EMaterialMergeType(0);
    pub const MATERIAL_MERGE_TYPE_SIMPLYGON: EMaterialMergeType = EMaterialMergeType(1);
}
#[repr(transparent)]
pub struct EMeshLODSelectionType(pub u8);
impl EMeshLODSelectionType {
    pub const ALL_LO_DS: EMeshLODSelectionType = EMeshLODSelectionType(0);
    pub const SPECIFIC_LOD: EMeshLODSelectionType = EMeshLODSelectionType(1);
    pub const CALCULATE_LOD: EMeshLODSelectionType = EMeshLODSelectionType(2);
    pub const LOWEST_DETAIL_LOD: EMeshLODSelectionType = EMeshLODSelectionType(3);
}
#[repr(transparent)]
pub struct EProxyNormalComputationMethod(pub u8);
impl EProxyNormalComputationMethod {
    pub const ANGLE_WEIGHTED: EProxyNormalComputationMethod = EProxyNormalComputationMethod(
        0,
    );
    pub const AREA_WEIGHTED: EProxyNormalComputationMethod = EProxyNormalComputationMethod(
        1,
    );
    pub const EQUAL_WEIGHTED: EProxyNormalComputationMethod = EProxyNormalComputationMethod(
        2,
    );
}
#[repr(transparent)]
pub struct ELandscapeCullingPrecision(pub u8);
impl ELandscapeCullingPrecision {
    pub const HIGH: ELandscapeCullingPrecision = ELandscapeCullingPrecision(0);
    pub const MEDIUM: ELandscapeCullingPrecision = ELandscapeCullingPrecision(1);
    pub const LOW: ELandscapeCullingPrecision = ELandscapeCullingPrecision(2);
}
#[repr(transparent)]
pub struct ERuntimeVirtualTextureMainPassType(pub u8);
impl ERuntimeVirtualTextureMainPassType {
    pub const NEVER: ERuntimeVirtualTextureMainPassType = ERuntimeVirtualTextureMainPassType(
        0,
    );
    pub const EXCLUSIVE: ERuntimeVirtualTextureMainPassType = ERuntimeVirtualTextureMainPassType(
        1,
    );
    pub const ALWAYS: ERuntimeVirtualTextureMainPassType = ERuntimeVirtualTextureMainPassType(
        2,
    );
}
#[repr(transparent)]
pub struct ELightmapType(pub u8);
impl ELightmapType {
    pub const DEFAULT: ELightmapType = ELightmapType(0);
    pub const FORCE_SURFACE: ELightmapType = ELightmapType(1);
    pub const FORCE_VOLUMETRIC: ELightmapType = ELightmapType(2);
}
#[repr(transparent)]
pub struct ERayTracingGroupCullingPriority(pub u8);
impl ERayTracingGroupCullingPriority {
    pub const CP_0_NEVER_CULL: ERayTracingGroupCullingPriority = ERayTracingGroupCullingPriority(
        0,
    );
    pub const CP_1: ERayTracingGroupCullingPriority = ERayTracingGroupCullingPriority(1);
    pub const CP_2: ERayTracingGroupCullingPriority = ERayTracingGroupCullingPriority(2);
    pub const CP_3: ERayTracingGroupCullingPriority = ERayTracingGroupCullingPriority(3);
    pub const CP_4_DEFAULT: ERayTracingGroupCullingPriority = ERayTracingGroupCullingPriority(
        4,
    );
    pub const CP_5: ERayTracingGroupCullingPriority = ERayTracingGroupCullingPriority(5);
    pub const CP_6: ERayTracingGroupCullingPriority = ERayTracingGroupCullingPriority(6);
    pub const CP_7: ERayTracingGroupCullingPriority = ERayTracingGroupCullingPriority(7);
    pub const CP_8_QUICKLY_CULL: ERayTracingGroupCullingPriority = ERayTracingGroupCullingPriority(
        8,
    );
}
#[repr(transparent)]
pub struct EHasCustomNavigableGeometry(pub u8);
impl EHasCustomNavigableGeometry {
    pub const NO: EHasCustomNavigableGeometry = EHasCustomNavigableGeometry(0);
    pub const YES: EHasCustomNavigableGeometry = EHasCustomNavigableGeometry(1);
    pub const EVEN_IF_NOT_COLLIDABLE: EHasCustomNavigableGeometry = EHasCustomNavigableGeometry(
        2,
    );
    pub const DONT_EXPORT: EHasCustomNavigableGeometry = EHasCustomNavigableGeometry(3);
}
#[repr(transparent)]
pub struct ERendererStencilMask(pub u8);
impl ERendererStencilMask {
    pub const ERSM_DEFAULT: ERendererStencilMask = ERendererStencilMask(0);
    pub const ERSM_255: ERendererStencilMask = ERendererStencilMask(1);
    pub const ERSM_1: ERendererStencilMask = ERendererStencilMask(2);
    pub const ERSM_2: ERendererStencilMask = ERendererStencilMask(3);
    pub const ERSM_4: ERendererStencilMask = ERendererStencilMask(4);
    pub const ERSM_8: ERendererStencilMask = ERendererStencilMask(5);
    pub const ERSM_16: ERendererStencilMask = ERendererStencilMask(6);
    pub const ERSM_32: ERendererStencilMask = ERendererStencilMask(7);
    pub const ERSM_64: ERendererStencilMask = ERendererStencilMask(8);
    pub const ERSM_128: ERendererStencilMask = ERendererStencilMask(9);
}
#[repr(transparent)]
pub struct EDetailMode(pub u8);
impl EDetailMode {
    pub const DM_LOW: EDetailMode = EDetailMode(0);
    pub const DM_MEDIUM: EDetailMode = EDetailMode(1);
    pub const DM_HIGH: EDetailMode = EDetailMode(2);
    pub const DM_EPIC: EDetailMode = EDetailMode(3);
}
#[repr(transparent)]
pub struct EImportanceWeight(pub u8);
impl EImportanceWeight {
    pub const LUMINANCE: EImportanceWeight = EImportanceWeight(0);
    pub const RED: EImportanceWeight = EImportanceWeight(1);
    pub const GREEN: EImportanceWeight = EImportanceWeight(2);
    pub const BLUE: EImportanceWeight = EImportanceWeight(3);
    pub const ALPHA: EImportanceWeight = EImportanceWeight(4);
}
#[repr(transparent)]
pub struct EMaterialAggregateAttributeType(pub i32);
impl EMaterialAggregateAttributeType {
    pub const BOOL1: EMaterialAggregateAttributeType = EMaterialAggregateAttributeType(
        0,
    );
    pub const BOOL2: EMaterialAggregateAttributeType = EMaterialAggregateAttributeType(
        1,
    );
    pub const BOOL3: EMaterialAggregateAttributeType = EMaterialAggregateAttributeType(
        2,
    );
    pub const BOOL4: EMaterialAggregateAttributeType = EMaterialAggregateAttributeType(
        3,
    );
    pub const U_INT1: EMaterialAggregateAttributeType = EMaterialAggregateAttributeType(
        4,
    );
    pub const U_INT2: EMaterialAggregateAttributeType = EMaterialAggregateAttributeType(
        5,
    );
    pub const U_INT3: EMaterialAggregateAttributeType = EMaterialAggregateAttributeType(
        6,
    );
    pub const U_INT4: EMaterialAggregateAttributeType = EMaterialAggregateAttributeType(
        7,
    );
    pub const FLOAT1: EMaterialAggregateAttributeType = EMaterialAggregateAttributeType(
        8,
    );
    pub const FLOAT2: EMaterialAggregateAttributeType = EMaterialAggregateAttributeType(
        9,
    );
    pub const FLOAT3: EMaterialAggregateAttributeType = EMaterialAggregateAttributeType(
        10,
    );
    pub const FLOAT4: EMaterialAggregateAttributeType = EMaterialAggregateAttributeType(
        11,
    );
    pub const SHADING_MODEL: EMaterialAggregateAttributeType = EMaterialAggregateAttributeType(
        12,
    );
    pub const MATERIAL_ATTRIBUTES: EMaterialAggregateAttributeType = EMaterialAggregateAttributeType(
        13,
    );
    pub const AGGREGATE: EMaterialAggregateAttributeType = EMaterialAggregateAttributeType(
        14,
    );
}
#[repr(transparent)]
pub struct EFunctionInputType(pub u8);
impl EFunctionInputType {
    pub const FUNCTION_INPUT_SCALAR: EFunctionInputType = EFunctionInputType(0);
    pub const FUNCTION_INPUT_VECTOR2: EFunctionInputType = EFunctionInputType(1);
    pub const FUNCTION_INPUT_VECTOR3: EFunctionInputType = EFunctionInputType(2);
    pub const FUNCTION_INPUT_VECTOR4: EFunctionInputType = EFunctionInputType(3);
    pub const FUNCTION_INPUT_TEXTURE2_D: EFunctionInputType = EFunctionInputType(4);
    pub const FUNCTION_INPUT_TEXTURE_CUBE: EFunctionInputType = EFunctionInputType(5);
    pub const FUNCTION_INPUT_TEXTURE2_D_ARRAY: EFunctionInputType = EFunctionInputType(
        6,
    );
    pub const FUNCTION_INPUT_VOLUME_TEXTURE: EFunctionInputType = EFunctionInputType(7);
    pub const FUNCTION_INPUT_STATIC_BOOL: EFunctionInputType = EFunctionInputType(8);
    pub const FUNCTION_INPUT_MATERIAL_ATTRIBUTES: EFunctionInputType = EFunctionInputType(
        9,
    );
    pub const FUNCTION_INPUT_TEXTURE_EXTERNAL: EFunctionInputType = EFunctionInputType(
        10,
    );
    pub const FUNCTION_INPUT_BOOL: EFunctionInputType = EFunctionInputType(11);
    pub const FUNCTION_INPUT_SUBSTRATE: EFunctionInputType = EFunctionInputType(12);
}
#[repr(transparent)]
pub struct EMaterialShaderFrequency(pub u8);
impl EMaterialShaderFrequency {
    pub const VERTEX: EMaterialShaderFrequency = EMaterialShaderFrequency(1);
    pub const PIXEL: EMaterialShaderFrequency = EMaterialShaderFrequency(8);
    pub const COMPUTE: EMaterialShaderFrequency = EMaterialShaderFrequency(32);
    pub const ANY: EMaterialShaderFrequency = EMaterialShaderFrequency(41);
}
#[repr(transparent)]
pub struct EMaterialValueTypeBridge(pub u64);
impl EMaterialValueTypeBridge {
    pub const FLOAT1: EMaterialValueTypeBridge = EMaterialValueTypeBridge(1);
    pub const FLOAT2: EMaterialValueTypeBridge = EMaterialValueTypeBridge(2);
    pub const FLOAT3: EMaterialValueTypeBridge = EMaterialValueTypeBridge(4);
    pub const FLOAT4: EMaterialValueTypeBridge = EMaterialValueTypeBridge(8);
    pub const TEXTURE2_D: EMaterialValueTypeBridge = EMaterialValueTypeBridge(16);
    pub const TEXTURE_CUBE: EMaterialValueTypeBridge = EMaterialValueTypeBridge(32);
    pub const TEXTURE2_D_ARRAY: EMaterialValueTypeBridge = EMaterialValueTypeBridge(64);
    pub const TEXTURE_CUBE_ARRAY: EMaterialValueTypeBridge = EMaterialValueTypeBridge(
        128,
    );
    pub const VOLUME_TEXTURE: EMaterialValueTypeBridge = EMaterialValueTypeBridge(256);
    pub const STATIC_BOOL: EMaterialValueTypeBridge = EMaterialValueTypeBridge(512);
    pub const UNKNOWN: EMaterialValueTypeBridge = EMaterialValueTypeBridge(1024);
    pub const MATERIAL_ATTRIBUTES: EMaterialValueTypeBridge = EMaterialValueTypeBridge(
        2048,
    );
    pub const TEXTURE_EXTERNAL: EMaterialValueTypeBridge = EMaterialValueTypeBridge(
        4096,
    );
    pub const TEXTURE_VIRTUAL: EMaterialValueTypeBridge = EMaterialValueTypeBridge(8192);
    pub const SPARSE_VOLUME_TEXTURE: EMaterialValueTypeBridge = EMaterialValueTypeBridge(
        16384,
    );
    pub const VT_PAGE_TABLE_RESULT: EMaterialValueTypeBridge = EMaterialValueTypeBridge(
        32768,
    );
    pub const SHADING_MODEL: EMaterialValueTypeBridge = EMaterialValueTypeBridge(65536);
    pub const SUBSTRATE: EMaterialValueTypeBridge = EMaterialValueTypeBridge(131072);
    pub const LWC_SCALAR: EMaterialValueTypeBridge = EMaterialValueTypeBridge(262144);
    pub const LWC_VECTOR2: EMaterialValueTypeBridge = EMaterialValueTypeBridge(524288);
    pub const LWC_VECTOR3: EMaterialValueTypeBridge = EMaterialValueTypeBridge(1048576);
    pub const LWC_VECTOR4: EMaterialValueTypeBridge = EMaterialValueTypeBridge(2097152);
    pub const EXECUTION: EMaterialValueTypeBridge = EMaterialValueTypeBridge(4194304);
    pub const VOID_STATEMENT: EMaterialValueTypeBridge = EMaterialValueTypeBridge(
        8388608,
    );
    pub const BOOL: EMaterialValueTypeBridge = EMaterialValueTypeBridge(16777216);
    pub const U_INT1: EMaterialValueTypeBridge = EMaterialValueTypeBridge(33554432);
    pub const U_INT2: EMaterialValueTypeBridge = EMaterialValueTypeBridge(67108864);
    pub const U_INT3: EMaterialValueTypeBridge = EMaterialValueTypeBridge(134217728);
    pub const U_INT4: EMaterialValueTypeBridge = EMaterialValueTypeBridge(268435456);
    pub const TEXTURE_COLLECTION: EMaterialValueTypeBridge = EMaterialValueTypeBridge(
        536870912,
    );
    pub const TEXTURE_MESH_PAINT: EMaterialValueTypeBridge = EMaterialValueTypeBridge(
        1073741824,
    );
    pub const TEXTURE_MATERIAL_CACHE: EMaterialValueTypeBridge = EMaterialValueTypeBridge(
        2147483648,
    );
    pub const TEXTURE: EMaterialValueTypeBridge = EMaterialValueTypeBridge(3221238256);
    pub const FLOAT: EMaterialValueTypeBridge = EMaterialValueTypeBridge(15);
    pub const U_INT: EMaterialValueTypeBridge = EMaterialValueTypeBridge(503316480);
    pub const LWC_TYPE: EMaterialValueTypeBridge = EMaterialValueTypeBridge(3932160);
    pub const NUMERIC: EMaterialValueTypeBridge = EMaterialValueTypeBridge(20709391);
    pub const FLOAT3X3: EMaterialValueTypeBridge = EMaterialValueTypeBridge(4294967296);
    pub const FLOAT4X4: EMaterialValueTypeBridge = EMaterialValueTypeBridge(8589934592);
    pub const LWC_MATRIX: EMaterialValueTypeBridge = EMaterialValueTypeBridge(
        17179869184,
    );
}
#[repr(transparent)]
pub struct EDerivativeStatus(pub u8);
impl EDerivativeStatus {
    pub const NOT_AWARE: EDerivativeStatus = EDerivativeStatus(0);
    pub const NOT_VALID: EDerivativeStatus = EDerivativeStatus(1);
    pub const ZERO: EDerivativeStatus = EDerivativeStatus(2);
    pub const VALID: EDerivativeStatus = EDerivativeStatus(3);
}
#[repr(transparent)]
pub struct EMaterialDomain(pub u8);
impl EMaterialDomain {
    pub const MD_SURFACE: EMaterialDomain = EMaterialDomain(0);
    pub const MD_DEFERRED_DECAL: EMaterialDomain = EMaterialDomain(1);
    pub const MD_LIGHT_FUNCTION: EMaterialDomain = EMaterialDomain(2);
    pub const MD_VOLUME: EMaterialDomain = EMaterialDomain(3);
    pub const MD_POST_PROCESS: EMaterialDomain = EMaterialDomain(4);
    pub const MD_UI: EMaterialDomain = EMaterialDomain(5);
    pub const MD_RUNTIME_VIRTUAL_TEXTURE: EMaterialDomain = EMaterialDomain(6);
}
#[repr(transparent)]
pub struct EMaterialFeatureLevel(pub u8);
impl EMaterialFeatureLevel {
    pub const ES2_REMOVED: EMaterialFeatureLevel = EMaterialFeatureLevel(0);
    pub const ES3_1: EMaterialFeatureLevel = EMaterialFeatureLevel(1);
    pub const SM4_REMOVED: EMaterialFeatureLevel = EMaterialFeatureLevel(2);
    pub const SM5: EMaterialFeatureLevel = EMaterialFeatureLevel(3);
    pub const SM6: EMaterialFeatureLevel = EMaterialFeatureLevel(4);
    pub const NUM: EMaterialFeatureLevel = EMaterialFeatureLevel(5);
}
#[repr(transparent)]
pub struct EMeshFeatureImportance(pub u8);
impl EMeshFeatureImportance {
    pub const OFF: EMeshFeatureImportance = EMeshFeatureImportance(0);
    pub const LOWEST: EMeshFeatureImportance = EMeshFeatureImportance(1);
    pub const LOW: EMeshFeatureImportance = EMeshFeatureImportance(2);
    pub const NORMAL: EMeshFeatureImportance = EMeshFeatureImportance(3);
    pub const HIGH: EMeshFeatureImportance = EMeshFeatureImportance(4);
    pub const HIGHEST: EMeshFeatureImportance = EMeshFeatureImportance(5);
}
#[repr(transparent)]
pub struct EStaticMeshReductionTerimationCriterion(pub u8);
impl EStaticMeshReductionTerimationCriterion {
    pub const TRIANGLES: EStaticMeshReductionTerimationCriterion = EStaticMeshReductionTerimationCriterion(
        0,
    );
    pub const VERTICES: EStaticMeshReductionTerimationCriterion = EStaticMeshReductionTerimationCriterion(
        1,
    );
    pub const ANY: EStaticMeshReductionTerimationCriterion = EStaticMeshReductionTerimationCriterion(
        2,
    );
}
#[repr(transparent)]
pub struct ENetworkMetricEnableMode(pub u8);
impl ENetworkMetricEnableMode {
    pub const ENABLE_FOR_ALL_REPLICATION: ENetworkMetricEnableMode = ENetworkMetricEnableMode(
        0,
    );
    pub const ENABLE_FOR_IRIS_ONLY: ENetworkMetricEnableMode = ENetworkMetricEnableMode(
        1,
    );
    pub const ENABLE_FOR_NON_IRIS_ONLY: ENetworkMetricEnableMode = ENetworkMetricEnableMode(
        2,
    );
    pub const SERVER_ONLY: ENetworkMetricEnableMode = ENetworkMetricEnableMode(3);
    pub const CLIENT_ONLY: ENetworkMetricEnableMode = ENetworkMetricEnableMode(4);
}
#[repr(transparent)]
pub struct EParticleSysParamType(pub u8);
impl EParticleSysParamType {
    pub const PSPT_NONE: EParticleSysParamType = EParticleSysParamType(0);
    pub const PSPT_SCALAR: EParticleSysParamType = EParticleSysParamType(1);
    pub const PSPT_SCALAR_RAND: EParticleSysParamType = EParticleSysParamType(2);
    pub const PSPT_VECTOR: EParticleSysParamType = EParticleSysParamType(3);
    pub const PSPT_VECTOR_RAND: EParticleSysParamType = EParticleSysParamType(4);
    pub const PSPT_COLOR: EParticleSysParamType = EParticleSysParamType(5);
    pub const PSPT_ACTOR: EParticleSysParamType = EParticleSysParamType(6);
    pub const PSPT_MATERIAL: EParticleSysParamType = EParticleSysParamType(7);
    pub const PSPT_VECTOR_UNIT_RAND: EParticleSysParamType = EParticleSysParamType(8);
}
#[repr(transparent)]
pub struct EAttachLocation(pub u8);
impl EAttachLocation {
    pub const KEEP_RELATIVE_OFFSET: EAttachLocation = EAttachLocation(0);
    pub const KEEP_WORLD_POSITION: EAttachLocation = EAttachLocation(1);
    pub const SNAP_TO_TARGET: EAttachLocation = EAttachLocation(2);
    pub const SNAP_TO_TARGET_INCLUDING_SCALE: EAttachLocation = EAttachLocation(3);
}
#[repr(transparent)]
pub struct EPSCPoolMethod(pub u8);
impl EPSCPoolMethod {
    pub const NONE: EPSCPoolMethod = EPSCPoolMethod(0);
    pub const AUTO_RELEASE: EPSCPoolMethod = EPSCPoolMethod(1);
    pub const MANUAL_RELEASE: EPSCPoolMethod = EPSCPoolMethod(2);
    pub const MANUAL_RELEASE_ON_COMPLETE: EPSCPoolMethod = EPSCPoolMethod(3);
    pub const FREE_IN_POOL: EPSCPoolMethod = EPSCPoolMethod(4);
}
#[repr(transparent)]
pub struct EAngularDriveMode(pub u8);
impl EAngularDriveMode {
    pub const SLERP: EAngularDriveMode = EAngularDriveMode(0);
    pub const TWIST_AND_SWING: EAngularDriveMode = EAngularDriveMode(1);
}
#[repr(transparent)]
pub struct EPhysicsReplicationMode(pub u8);
impl EPhysicsReplicationMode {
    pub const DEFAULT: EPhysicsReplicationMode = EPhysicsReplicationMode(0);
    pub const PREDICTIVE_INTERPOLATION: EPhysicsReplicationMode = EPhysicsReplicationMode(
        1,
    );
    pub const RESIMULATION: EPhysicsReplicationMode = EPhysicsReplicationMode(2);
}
#[repr(transparent)]
pub struct EViewTargetBlendFunction(pub u8);
impl EViewTargetBlendFunction {
    pub const VT_BLEND_LINEAR: EViewTargetBlendFunction = EViewTargetBlendFunction(0);
    pub const VT_BLEND_CUBIC: EViewTargetBlendFunction = EViewTargetBlendFunction(1);
    pub const VT_BLEND_EASE_IN: EViewTargetBlendFunction = EViewTargetBlendFunction(2);
    pub const VT_BLEND_EASE_OUT: EViewTargetBlendFunction = EViewTargetBlendFunction(3);
    pub const VT_BLEND_EASE_IN_OUT: EViewTargetBlendFunction = EViewTargetBlendFunction(
        4,
    );
    pub const VT_BLEND_PRE_BLENDED: EViewTargetBlendFunction = EViewTargetBlendFunction(
        5,
    );
}
#[repr(transparent)]
pub struct EPropertyAccessObjectType(pub u8);
impl EPropertyAccessObjectType {
    pub const NONE: EPropertyAccessObjectType = EPropertyAccessObjectType(0);
    pub const OBJECT: EPropertyAccessObjectType = EPropertyAccessObjectType(1);
    pub const WEAK_OBJECT: EPropertyAccessObjectType = EPropertyAccessObjectType(2);
    pub const SOFT_OBJECT: EPropertyAccessObjectType = EPropertyAccessObjectType(3);
}
#[repr(transparent)]
pub struct EPropertyAccessIndirectionType(pub u8);
impl EPropertyAccessIndirectionType {
    pub const OFFSET: EPropertyAccessIndirectionType = EPropertyAccessIndirectionType(0);
    pub const OBJECT: EPropertyAccessIndirectionType = EPropertyAccessIndirectionType(1);
    pub const ARRAY: EPropertyAccessIndirectionType = EPropertyAccessIndirectionType(2);
    pub const SCRIPT_FUNCTION: EPropertyAccessIndirectionType = EPropertyAccessIndirectionType(
        3,
    );
    pub const NATIVE_FUNCTION: EPropertyAccessIndirectionType = EPropertyAccessIndirectionType(
        4,
    );
}
#[repr(transparent)]
pub struct ENeuralProfileFormat(pub u8);
impl ENeuralProfileFormat {
    pub const TYPE32: ENeuralProfileFormat = ENeuralProfileFormat(0);
    pub const TYPE16: ENeuralProfileFormat = ENeuralProfileFormat(1);
}
#[repr(transparent)]
pub struct ENeuralProfileRuntimeType(pub u8);
impl ENeuralProfileRuntimeType {
    pub const NNE_RUNTIME_ORT_DML: ENeuralProfileRuntimeType = ENeuralProfileRuntimeType(
        0,
    );
    pub const NNE_RUNTIME_RDG_HLSL: ENeuralProfileRuntimeType = ENeuralProfileRuntimeType(
        1,
    );
    pub const MAX: ENeuralProfileRuntimeType = ENeuralProfileRuntimeType(2);
}
#[repr(transparent)]
pub struct ENeuralModelTileType(pub u8);
impl ENeuralModelTileType {
    pub const ONE_BY_ONE: ENeuralModelTileType = ENeuralModelTileType(0);
    pub const TWO_BY_TWO: ENeuralModelTileType = ENeuralModelTileType(1);
    pub const FOUR_BY_FOUR: ENeuralModelTileType = ENeuralModelTileType(2);
    pub const EIGHT_BY_EIGHT: ENeuralModelTileType = ENeuralModelTileType(3);
    pub const AUTO: ENeuralModelTileType = ENeuralModelTileType(4);
}
#[repr(transparent)]
pub struct ETileOverlapResolveType(pub u8);
impl ETileOverlapResolveType {
    pub const IGNORE: ETileOverlapResolveType = ETileOverlapResolveType(0);
    pub const FEATHERING: ETileOverlapResolveType = ETileOverlapResolveType(1);
}
#[repr(transparent)]
pub struct ESpecularProfileFormat(pub u8);
impl ESpecularProfileFormat {
    pub const VIEW_LIGHT_VECTOR: ESpecularProfileFormat = ESpecularProfileFormat(0);
    pub const HALF_VECTOR: ESpecularProfileFormat = ESpecularProfileFormat(1);
}
#[repr(transparent)]
pub struct ESubsurfaceImplementationTechniqueHint(pub u8);
impl ESubsurfaceImplementationTechniqueHint {
    pub const SIH_AFIS: ESubsurfaceImplementationTechniqueHint = ESubsurfaceImplementationTechniqueHint(
        0,
    );
    pub const SIH_SEPARABLE: ESubsurfaceImplementationTechniqueHint = ESubsurfaceImplementationTechniqueHint(
        1,
    );
}
#[repr(transparent)]
pub struct ReverbPreset(pub u8);
impl ReverbPreset {
    pub const REVERB_DEFAULT: ReverbPreset = ReverbPreset(0);
    pub const REVERB_BATHROOM: ReverbPreset = ReverbPreset(1);
    pub const REVERB_STONE_ROOM: ReverbPreset = ReverbPreset(2);
    pub const REVERB_AUDITORIUM: ReverbPreset = ReverbPreset(3);
    pub const REVERB_CONCERT_HALL: ReverbPreset = ReverbPreset(4);
    pub const REVERB_CAVE: ReverbPreset = ReverbPreset(5);
    pub const REVERB_HALLWAY: ReverbPreset = ReverbPreset(6);
    pub const REVERB_STONE_CORRIDOR: ReverbPreset = ReverbPreset(7);
    pub const REVERB_ALLEY: ReverbPreset = ReverbPreset(8);
    pub const REVERB_FOREST: ReverbPreset = ReverbPreset(9);
    pub const REVERB_CITY: ReverbPreset = ReverbPreset(10);
    pub const REVERB_MOUNTAINS: ReverbPreset = ReverbPreset(11);
    pub const REVERB_QUARRY: ReverbPreset = ReverbPreset(12);
    pub const REVERB_PLAIN: ReverbPreset = ReverbPreset(13);
    pub const REVERB_PARKING_LOT: ReverbPreset = ReverbPreset(14);
    pub const REVERB_SEWER_PIPE: ReverbPreset = ReverbPreset(15);
    pub const REVERB_UNDERWATER: ReverbPreset = ReverbPreset(16);
    pub const REVERB_SMALL_ROOM: ReverbPreset = ReverbPreset(17);
    pub const REVERB_MEDIUM_ROOM: ReverbPreset = ReverbPreset(18);
    pub const REVERB_LARGE_ROOM: ReverbPreset = ReverbPreset(19);
    pub const REVERB_MEDIUM_HALL: ReverbPreset = ReverbPreset(20);
    pub const REVERB_LARGE_HALL: ReverbPreset = ReverbPreset(21);
    pub const REVERB_PLATE: ReverbPreset = ReverbPreset(22);
}
#[repr(transparent)]
pub struct EBoneFilterActionOption(pub u8);
impl EBoneFilterActionOption {
    pub const REMOVE: EBoneFilterActionOption = EBoneFilterActionOption(0);
    pub const KEEP: EBoneFilterActionOption = EBoneFilterActionOption(1);
    pub const INVALID: EBoneFilterActionOption = EBoneFilterActionOption(2);
}
#[repr(transparent)]
pub struct ESkinCacheUsage(pub u8);
impl ESkinCacheUsage {
    pub const AUTO: ESkinCacheUsage = ESkinCacheUsage(0);
    pub const DISABLED: ESkinCacheUsage = ESkinCacheUsage(255);
    pub const ENABLED: ESkinCacheUsage = ESkinCacheUsage(1);
}
#[repr(transparent)]
pub struct ESoundSpatializationAlgorithm(pub u8);
impl ESoundSpatializationAlgorithm {
    pub const SPATIALIZATION_DEFAULT: ESoundSpatializationAlgorithm = ESoundSpatializationAlgorithm(
        0,
    );
    pub const SPATIALIZATION_HRTF: ESoundSpatializationAlgorithm = ESoundSpatializationAlgorithm(
        1,
    );
}
#[repr(transparent)]
pub struct EAirAbsorptionMethod(pub u8);
impl EAirAbsorptionMethod {
    pub const LINEAR: EAirAbsorptionMethod = EAirAbsorptionMethod(0);
    pub const CUSTOM_CURVE: EAirAbsorptionMethod = EAirAbsorptionMethod(1);
}
#[repr(transparent)]
pub struct EReverbSendMethod(pub u8);
impl EReverbSendMethod {
    pub const LINEAR: EReverbSendMethod = EReverbSendMethod(0);
    pub const CUSTOM_CURVE: EReverbSendMethod = EReverbSendMethod(1);
    pub const MANUAL: EReverbSendMethod = EReverbSendMethod(2);
}
#[repr(transparent)]
pub struct EPriorityAttenuationMethod(pub u8);
impl EPriorityAttenuationMethod {
    pub const LINEAR: EPriorityAttenuationMethod = EPriorityAttenuationMethod(0);
    pub const CUSTOM_CURVE: EPriorityAttenuationMethod = EPriorityAttenuationMethod(1);
    pub const MANUAL: EPriorityAttenuationMethod = EPriorityAttenuationMethod(2);
}
#[repr(transparent)]
pub struct ESoundDistanceCalc(pub u8);
impl ESoundDistanceCalc {
    pub const SOUNDDISTANCE_NORMAL: ESoundDistanceCalc = ESoundDistanceCalc(0);
    pub const SOUNDDISTANCE_INFINITE_XY_PLANE: ESoundDistanceCalc = ESoundDistanceCalc(
        1,
    );
    pub const SOUNDDISTANCE_INFINITE_XZ_PLANE: ESoundDistanceCalc = ESoundDistanceCalc(
        2,
    );
    pub const SOUNDDISTANCE_INFINITE_YZ_PLANE: ESoundDistanceCalc = ESoundDistanceCalc(
        3,
    );
}
#[repr(transparent)]
pub struct ENonSpatializedRadiusSpeakerMapMode(pub u8);
impl ENonSpatializedRadiusSpeakerMapMode {
    pub const OMNI_DIRECTIONAL: ENonSpatializedRadiusSpeakerMapMode = ENonSpatializedRadiusSpeakerMapMode(
        0,
    );
    pub const DIRECT2_D: ENonSpatializedRadiusSpeakerMapMode = ENonSpatializedRadiusSpeakerMapMode(
        1,
    );
    pub const SURROUND2_D: ENonSpatializedRadiusSpeakerMapMode = ENonSpatializedRadiusSpeakerMapMode(
        2,
    );
}
#[repr(transparent)]
pub struct EUseSubmixOnPreviewMode(pub u8);
impl EUseSubmixOnPreviewMode {
    pub const USE_EDITOR_PREFERENCE: EUseSubmixOnPreviewMode = EUseSubmixOnPreviewMode(
        0,
    );
    pub const USE_SUBMIXES_ON_PREVIEW: EUseSubmixOnPreviewMode = EUseSubmixOnPreviewMode(
        1,
    );
    pub const PLAY_PREVIEWS_WITHOUT_SUBMIXES: EUseSubmixOnPreviewMode = EUseSubmixOnPreviewMode(
        2,
    );
}
#[repr(transparent)]
pub struct EAudioOutputTarget(pub u8);
impl EAudioOutputTarget {
    pub const SPEAKER: EAudioOutputTarget = EAudioOutputTarget(0);
    pub const CONTROLLER: EAudioOutputTarget = EAudioOutputTarget(1);
    pub const CONTROLLER_FALLBACK_TO_SPEAKER: EAudioOutputTarget = EAudioOutputTarget(2);
}
#[repr(transparent)]
pub struct ESoundWaveLoadingBehavior(pub u8);
impl ESoundWaveLoadingBehavior {
    pub const INHERITED: ESoundWaveLoadingBehavior = ESoundWaveLoadingBehavior(0);
    pub const RETAIN_ON_LOAD: ESoundWaveLoadingBehavior = ESoundWaveLoadingBehavior(1);
    pub const PRIME_ON_LOAD: ESoundWaveLoadingBehavior = ESoundWaveLoadingBehavior(2);
    pub const LOAD_ON_DEMAND: ESoundWaveLoadingBehavior = ESoundWaveLoadingBehavior(3);
    pub const FORCE_INLINE: ESoundWaveLoadingBehavior = ESoundWaveLoadingBehavior(4);
    pub const UNINITIALIZED: ESoundWaveLoadingBehavior = ESoundWaveLoadingBehavior(255);
}
#[repr(transparent)]
pub struct EMaxConcurrentResolutionRule(pub u8);
impl EMaxConcurrentResolutionRule {
    pub const PREVENT_NEW: EMaxConcurrentResolutionRule = EMaxConcurrentResolutionRule(
        0,
    );
    pub const STOP_OLDEST: EMaxConcurrentResolutionRule = EMaxConcurrentResolutionRule(
        1,
    );
    pub const STOP_FARTHEST_THEN_PREVENT_NEW: EMaxConcurrentResolutionRule = EMaxConcurrentResolutionRule(
        2,
    );
    pub const STOP_FARTHEST_THEN_OLDEST: EMaxConcurrentResolutionRule = EMaxConcurrentResolutionRule(
        3,
    );
    pub const STOP_LOWEST_PRIORITY: EMaxConcurrentResolutionRule = EMaxConcurrentResolutionRule(
        4,
    );
    pub const STOP_QUIETEST: EMaxConcurrentResolutionRule = EMaxConcurrentResolutionRule(
        5,
    );
    pub const STOP_LOWEST_PRIORITY_THEN_PREVENT_NEW: EMaxConcurrentResolutionRule = EMaxConcurrentResolutionRule(
        6,
    );
    pub const COUNT: EMaxConcurrentResolutionRule = EMaxConcurrentResolutionRule(7);
}
#[repr(transparent)]
pub struct EConcurrencyVolumeScaleMode(pub i32);
impl EConcurrencyVolumeScaleMode {
    pub const DEFAULT: EConcurrencyVolumeScaleMode = EConcurrencyVolumeScaleMode(0);
    pub const DISTANCE: EConcurrencyVolumeScaleMode = EConcurrencyVolumeScaleMode(1);
    pub const PRIORITY: EConcurrencyVolumeScaleMode = EConcurrencyVolumeScaleMode(2);
}
#[repr(transparent)]
pub struct ESoundGroup(pub u8);
impl ESoundGroup {
    pub const SOUNDGROUP_DEFAULT: ESoundGroup = ESoundGroup(0);
    pub const SOUNDGROUP_EFFECTS: ESoundGroup = ESoundGroup(1);
    pub const SOUNDGROUP_UI: ESoundGroup = ESoundGroup(2);
    pub const SOUNDGROUP_MUSIC: ESoundGroup = ESoundGroup(3);
    pub const SOUNDGROUP_VOICE: ESoundGroup = ESoundGroup(4);
    pub const SOUNDGROUP_GAME_SOUND_GROUP1: ESoundGroup = ESoundGroup(5);
    pub const SOUNDGROUP_GAME_SOUND_GROUP2: ESoundGroup = ESoundGroup(6);
    pub const SOUNDGROUP_GAME_SOUND_GROUP3: ESoundGroup = ESoundGroup(7);
    pub const SOUNDGROUP_GAME_SOUND_GROUP4: ESoundGroup = ESoundGroup(8);
    pub const SOUNDGROUP_GAME_SOUND_GROUP5: ESoundGroup = ESoundGroup(9);
    pub const SOUNDGROUP_GAME_SOUND_GROUP6: ESoundGroup = ESoundGroup(10);
    pub const SOUNDGROUP_GAME_SOUND_GROUP7: ESoundGroup = ESoundGroup(11);
    pub const SOUNDGROUP_GAME_SOUND_GROUP8: ESoundGroup = ESoundGroup(12);
    pub const SOUNDGROUP_GAME_SOUND_GROUP9: ESoundGroup = ESoundGroup(13);
    pub const SOUNDGROUP_GAME_SOUND_GROUP10: ESoundGroup = ESoundGroup(14);
    pub const SOUNDGROUP_GAME_SOUND_GROUP11: ESoundGroup = ESoundGroup(15);
    pub const SOUNDGROUP_GAME_SOUND_GROUP12: ESoundGroup = ESoundGroup(16);
    pub const SOUNDGROUP_GAME_SOUND_GROUP13: ESoundGroup = ESoundGroup(17);
    pub const SOUNDGROUP_GAME_SOUND_GROUP14: ESoundGroup = ESoundGroup(18);
    pub const SOUNDGROUP_GAME_SOUND_GROUP15: ESoundGroup = ESoundGroup(19);
    pub const SOUNDGROUP_GAME_SOUND_GROUP16: ESoundGroup = ESoundGroup(20);
    pub const SOUNDGROUP_GAME_SOUND_GROUP17: ESoundGroup = ESoundGroup(21);
    pub const SOUNDGROUP_GAME_SOUND_GROUP18: ESoundGroup = ESoundGroup(22);
    pub const SOUNDGROUP_GAME_SOUND_GROUP19: ESoundGroup = ESoundGroup(23);
    pub const SOUNDGROUP_GAME_SOUND_GROUP20: ESoundGroup = ESoundGroup(24);
}
#[repr(transparent)]
pub struct EModulationRouting(pub u8);
impl EModulationRouting {
    pub const DISABLE: EModulationRouting = EModulationRouting(0);
    pub const INHERIT: EModulationRouting = EModulationRouting(1);
    pub const OVERRIDE: EModulationRouting = EModulationRouting(2);
    pub const UNION: EModulationRouting = EModulationRouting(3);
}
#[repr(transparent)]
pub struct ModulationParamMode(pub u8);
impl ModulationParamMode {
    pub const MPM_NORMAL: ModulationParamMode = ModulationParamMode(0);
    pub const MPM_ABS: ModulationParamMode = ModulationParamMode(1);
    pub const MPM_DIRECT: ModulationParamMode = ModulationParamMode(2);
}
#[repr(transparent)]
pub struct EOptimizationType(pub u8);
impl EOptimizationType {
    pub const OT_NUM_OF_TRIANGLES: EOptimizationType = EOptimizationType(0);
}
#[repr(transparent)]
pub struct ESubtitleDurationType(pub u8);
impl ESubtitleDurationType {
    pub const USE_SOUND_DURATION: ESubtitleDurationType = ESubtitleDurationType(0);
    pub const USE_DURATION_PROPERTY: ESubtitleDurationType = ESubtitleDurationType(1);
}
#[repr(transparent)]
pub struct ESubtitleType(pub u8);
impl ESubtitleType {
    pub const SUBTITLE: ESubtitleType = ESubtitleType(0);
    pub const CLOSED_CAPTION: ESubtitleType = ESubtitleType(1);
    pub const AUDIO_DESCRIPTION: ESubtitleType = ESubtitleType(2);
}
#[repr(transparent)]
pub struct ESyncPointEventType(pub u8);
impl ESyncPointEventType {
    pub const INVALID: ESyncPointEventType = ESyncPointEventType(0);
    pub const SIMPLE_EVENT: ESyncPointEventType = ESyncPointEventType(1);
    pub const GAME_THREAD_TASK: ESyncPointEventType = ESyncPointEventType(2);
    pub const GAME_THREAD_TASK_HIGH_PRIORITY: ESyncPointEventType = ESyncPointEventType(
        3,
    );
    pub const WORKER_THREAD_TASK: ESyncPointEventType = ESyncPointEventType(4);
    pub const WORKER_THREAD_TASK_HIGH_PRIORITY: ESyncPointEventType = ESyncPointEventType(
        5,
    );
}
#[repr(transparent)]
pub struct ESyncPointActivationRules(pub u8);
impl ESyncPointActivationRules {
    pub const INVALID: ESyncPointActivationRules = ESyncPointActivationRules(0);
    pub const ALWAYS_ACTIVATE: ESyncPointActivationRules = ESyncPointActivationRules(1);
    pub const WAIT_FOR_TRIGGER: ESyncPointActivationRules = ESyncPointActivationRules(2);
    pub const WAIT_FOR_ALL_WORK: ESyncPointActivationRules = ESyncPointActivationRules(
        3,
    );
    pub const ACTIVATE_FOR_ANY_WORK: ESyncPointActivationRules = ESyncPointActivationRules(
        4,
    );
}
#[repr(transparent)]
pub struct ETextImportTestFlags(pub u32);
impl ETextImportTestFlags {
    pub const DEFAULT: ETextImportTestFlags = ETextImportTestFlags(0);
    pub const FLAG_A: ETextImportTestFlags = ETextImportTestFlags(1);
    pub const FLAG_B: ETextImportTestFlags = ETextImportTestFlags(2);
    pub const FLAG_C: ETextImportTestFlags = ETextImportTestFlags(4);
    pub const FLAG_D: ETextImportTestFlags = ETextImportTestFlags(8);
    pub const FLAG_E: ETextImportTestFlags = ETextImportTestFlags(16);
    pub const TEST_STRUCT_DEFAULT: ETextImportTestFlags = ETextImportTestFlags(32);
}
#[repr(transparent)]
pub struct ETextureSourceCompressionFormat(pub u8);
impl ETextureSourceCompressionFormat {
    pub const TSCF_NONE: ETextureSourceCompressionFormat = ETextureSourceCompressionFormat(
        0,
    );
    pub const TSCF_PNG: ETextureSourceCompressionFormat = ETextureSourceCompressionFormat(
        1,
    );
    pub const TSCF_JPEG: ETextureSourceCompressionFormat = ETextureSourceCompressionFormat(
        2,
    );
    pub const TSCF_UEJPEG: ETextureSourceCompressionFormat = ETextureSourceCompressionFormat(
        3,
    );
    pub const TSCF_UEDELTA: ETextureSourceCompressionFormat = ETextureSourceCompressionFormat(
        4,
    );
}
#[repr(transparent)]
pub struct ETextureSourceFormat(pub u8);
impl ETextureSourceFormat {
    pub const TSF_INVALID: ETextureSourceFormat = ETextureSourceFormat(0);
    pub const TSF_G8: ETextureSourceFormat = ETextureSourceFormat(1);
    pub const TSF_BGRA8: ETextureSourceFormat = ETextureSourceFormat(2);
    pub const TSF_BGRE8: ETextureSourceFormat = ETextureSourceFormat(3);
    pub const TSF_RGBA16: ETextureSourceFormat = ETextureSourceFormat(4);
    pub const TSF_RGBA16F: ETextureSourceFormat = ETextureSourceFormat(5);
    pub const TSF_RGBA8_DEPRECATED: ETextureSourceFormat = ETextureSourceFormat(6);
    pub const TSF_RGBE8_DEPRECATED: ETextureSourceFormat = ETextureSourceFormat(7);
    pub const TSF_G16: ETextureSourceFormat = ETextureSourceFormat(8);
    pub const TSF_RGBA32F: ETextureSourceFormat = ETextureSourceFormat(9);
    pub const TSF_R16F: ETextureSourceFormat = ETextureSourceFormat(10);
    pub const TSF_R32F: ETextureSourceFormat = ETextureSourceFormat(11);
}
#[repr(transparent)]
pub struct TextureCompressionSettings(pub u8);
impl TextureCompressionSettings {
    pub const TC_DEFAULT: TextureCompressionSettings = TextureCompressionSettings(0);
    pub const TC_NORMALMAP: TextureCompressionSettings = TextureCompressionSettings(1);
    pub const TC_MASKS: TextureCompressionSettings = TextureCompressionSettings(2);
    pub const TC_GRAYSCALE: TextureCompressionSettings = TextureCompressionSettings(3);
    pub const TC_DISPLACEMENTMAP: TextureCompressionSettings = TextureCompressionSettings(
        4,
    );
    pub const TC_VECTOR_DISPLACEMENTMAP: TextureCompressionSettings = TextureCompressionSettings(
        5,
    );
    pub const TC_HDR: TextureCompressionSettings = TextureCompressionSettings(6);
    pub const TC_EDITOR_ICON: TextureCompressionSettings = TextureCompressionSettings(7);
    pub const TC_ALPHA: TextureCompressionSettings = TextureCompressionSettings(8);
    pub const TC_DISTANCE_FIELD_FONT: TextureCompressionSettings = TextureCompressionSettings(
        9,
    );
    pub const TC_HDR_COMPRESSED: TextureCompressionSettings = TextureCompressionSettings(
        10,
    );
    pub const TC_BC7: TextureCompressionSettings = TextureCompressionSettings(11);
    pub const TC_HALF_FLOAT: TextureCompressionSettings = TextureCompressionSettings(12);
    pub const TC_LQ: TextureCompressionSettings = TextureCompressionSettings(13);
    pub const TC_ENCODED_REFLECTION_CAPTURE: TextureCompressionSettings = TextureCompressionSettings(
        14,
    );
    pub const TC_SINGLE_FLOAT: TextureCompressionSettings = TextureCompressionSettings(
        15,
    );
    pub const TC_HDR_F32: TextureCompressionSettings = TextureCompressionSettings(16);
}
#[repr(transparent)]
pub struct ETextureSourceEncoding(pub u8);
impl ETextureSourceEncoding {
    pub const TSE_NONE: ETextureSourceEncoding = ETextureSourceEncoding(0);
    pub const TSE_LINEAR: ETextureSourceEncoding = ETextureSourceEncoding(1);
    pub const TSE_S_RGB: ETextureSourceEncoding = ETextureSourceEncoding(2);
    pub const TSE_ST2084: ETextureSourceEncoding = ETextureSourceEncoding(3);
    pub const TSE_GAMMA22: ETextureSourceEncoding = ETextureSourceEncoding(4);
    pub const TSE_BT1886: ETextureSourceEncoding = ETextureSourceEncoding(5);
    pub const TSE_GAMMA26: ETextureSourceEncoding = ETextureSourceEncoding(6);
    pub const TSE_CINEON: ETextureSourceEncoding = ETextureSourceEncoding(7);
    pub const TSE_RED_LOG: ETextureSourceEncoding = ETextureSourceEncoding(8);
    pub const TSE_RED_LOG3_G10: ETextureSourceEncoding = ETextureSourceEncoding(9);
    pub const TSE_S_LOG1: ETextureSourceEncoding = ETextureSourceEncoding(10);
    pub const TSE_S_LOG2: ETextureSourceEncoding = ETextureSourceEncoding(11);
    pub const TSE_S_LOG3: ETextureSourceEncoding = ETextureSourceEncoding(12);
    pub const TSE_ALEXA_V3_LOG_C: ETextureSourceEncoding = ETextureSourceEncoding(13);
    pub const TSE_CANON_LOG: ETextureSourceEncoding = ETextureSourceEncoding(14);
    pub const TSE_PRO_TUNE: ETextureSourceEncoding = ETextureSourceEncoding(15);
    pub const TSE_V_LOG: ETextureSourceEncoding = ETextureSourceEncoding(16);
}
#[repr(transparent)]
pub struct ETextureColorSpace(pub u8);
impl ETextureColorSpace {
    pub const TCS_NONE: ETextureColorSpace = ETextureColorSpace(0);
    pub const TCS_S_RGB: ETextureColorSpace = ETextureColorSpace(1);
    pub const TCS_REC2020: ETextureColorSpace = ETextureColorSpace(2);
    pub const TCS_ACESAP0: ETextureColorSpace = ETextureColorSpace(3);
    pub const TCS_ACESAP1: ETextureColorSpace = ETextureColorSpace(4);
    pub const TCS_P3DCI: ETextureColorSpace = ETextureColorSpace(5);
    pub const TCS_P3D65: ETextureColorSpace = ETextureColorSpace(6);
    pub const TCS_RED_WIDE_GAMUT: ETextureColorSpace = ETextureColorSpace(7);
    pub const TCS_SONY_S_GAMUT3: ETextureColorSpace = ETextureColorSpace(8);
    pub const TCS_SONY_S_GAMUT3_CINE: ETextureColorSpace = ETextureColorSpace(9);
    pub const TCS_ALEXA_WIDE_GAMUT: ETextureColorSpace = ETextureColorSpace(10);
    pub const TCS_CANON_CINEMA_GAMUT: ETextureColorSpace = ETextureColorSpace(11);
    pub const TCS_GO_PRO_PROTUNE_NATIVE: ETextureColorSpace = ETextureColorSpace(12);
    pub const TCS_PANASONIC_V_GAMUT: ETextureColorSpace = ETextureColorSpace(13);
    pub const TCS_CUSTOM: ETextureColorSpace = ETextureColorSpace(99);
}
#[repr(transparent)]
pub struct ETextureChromaticAdaptationMethod(pub u8);
impl ETextureChromaticAdaptationMethod {
    pub const TCAM_NONE: ETextureChromaticAdaptationMethod = ETextureChromaticAdaptationMethod(
        0,
    );
    pub const TCAM_BRADFORD: ETextureChromaticAdaptationMethod = ETextureChromaticAdaptationMethod(
        1,
    );
    pub const TCAM_CAT02: ETextureChromaticAdaptationMethod = ETextureChromaticAdaptationMethod(
        2,
    );
}
#[repr(transparent)]
pub struct TextureGroup(pub u8);
impl TextureGroup {
    pub const TEXTUREGROUP_WORLD: TextureGroup = TextureGroup(0);
    pub const TEXTUREGROUP_WORLD_NORMAL_MAP: TextureGroup = TextureGroup(1);
    pub const TEXTUREGROUP_WORLD_SPECULAR: TextureGroup = TextureGroup(2);
    pub const TEXTUREGROUP_CHARACTER: TextureGroup = TextureGroup(3);
    pub const TEXTUREGROUP_CHARACTER_NORMAL_MAP: TextureGroup = TextureGroup(4);
    pub const TEXTUREGROUP_CHARACTER_SPECULAR: TextureGroup = TextureGroup(5);
    pub const TEXTUREGROUP_WEAPON: TextureGroup = TextureGroup(6);
    pub const TEXTUREGROUP_WEAPON_NORMAL_MAP: TextureGroup = TextureGroup(7);
    pub const TEXTUREGROUP_WEAPON_SPECULAR: TextureGroup = TextureGroup(8);
    pub const TEXTUREGROUP_VEHICLE: TextureGroup = TextureGroup(9);
    pub const TEXTUREGROUP_VEHICLE_NORMAL_MAP: TextureGroup = TextureGroup(10);
    pub const TEXTUREGROUP_VEHICLE_SPECULAR: TextureGroup = TextureGroup(11);
    pub const TEXTUREGROUP_CINEMATIC: TextureGroup = TextureGroup(12);
    pub const TEXTUREGROUP_EFFECTS: TextureGroup = TextureGroup(13);
    pub const TEXTUREGROUP_EFFECTS_NOT_FILTERED: TextureGroup = TextureGroup(14);
    pub const TEXTUREGROUP_SKYBOX: TextureGroup = TextureGroup(15);
    pub const TEXTUREGROUP_UI: TextureGroup = TextureGroup(16);
    pub const TEXTUREGROUP_LIGHTMAP: TextureGroup = TextureGroup(17);
    pub const TEXTUREGROUP_RENDER_TARGET: TextureGroup = TextureGroup(18);
    pub const TEXTUREGROUP_MOBILE_FLATTENED: TextureGroup = TextureGroup(19);
    pub const TEXTUREGROUP_PROC_BUILDING_FACE: TextureGroup = TextureGroup(20);
    pub const TEXTUREGROUP_PROC_BUILDING_LIGHT_MAP: TextureGroup = TextureGroup(21);
    pub const TEXTUREGROUP_SHADOWMAP: TextureGroup = TextureGroup(22);
    pub const TEXTUREGROUP_COLOR_LOOKUP_TABLE: TextureGroup = TextureGroup(23);
    pub const TEXTUREGROUP_TERRAIN_HEIGHTMAP: TextureGroup = TextureGroup(24);
    pub const TEXTUREGROUP_TERRAIN_WEIGHTMAP: TextureGroup = TextureGroup(25);
    pub const TEXTUREGROUP_BOKEH: TextureGroup = TextureGroup(26);
    pub const TEXTUREGROUP_IES_LIGHT_PROFILE: TextureGroup = TextureGroup(27);
    pub const TEXTUREGROUP_PIXELS2_D: TextureGroup = TextureGroup(28);
    pub const TEXTUREGROUP_HIERARCHICAL_LOD: TextureGroup = TextureGroup(29);
    pub const TEXTUREGROUP_IMPOSTOR: TextureGroup = TextureGroup(30);
    pub const TEXTUREGROUP_IMPOSTOR_NORMAL_DEPTH: TextureGroup = TextureGroup(31);
    pub const TEXTUREGROUP_8BIT_DATA: TextureGroup = TextureGroup(32);
    pub const TEXTUREGROUP_16BIT_DATA: TextureGroup = TextureGroup(33);
    pub const TEXTUREGROUP_PROJECT01: TextureGroup = TextureGroup(34);
    pub const TEXTUREGROUP_PROJECT02: TextureGroup = TextureGroup(35);
    pub const TEXTUREGROUP_PROJECT03: TextureGroup = TextureGroup(36);
    pub const TEXTUREGROUP_PROJECT04: TextureGroup = TextureGroup(37);
    pub const TEXTUREGROUP_PROJECT05: TextureGroup = TextureGroup(38);
    pub const TEXTUREGROUP_PROJECT06: TextureGroup = TextureGroup(39);
    pub const TEXTUREGROUP_PROJECT07: TextureGroup = TextureGroup(40);
    pub const TEXTUREGROUP_PROJECT08: TextureGroup = TextureGroup(41);
    pub const TEXTUREGROUP_PROJECT09: TextureGroup = TextureGroup(42);
    pub const TEXTUREGROUP_PROJECT10: TextureGroup = TextureGroup(43);
    pub const TEXTUREGROUP_PROJECT11: TextureGroup = TextureGroup(44);
    pub const TEXTUREGROUP_PROJECT12: TextureGroup = TextureGroup(45);
    pub const TEXTUREGROUP_PROJECT13: TextureGroup = TextureGroup(46);
    pub const TEXTUREGROUP_PROJECT14: TextureGroup = TextureGroup(47);
    pub const TEXTUREGROUP_PROJECT15: TextureGroup = TextureGroup(48);
    pub const TEXTUREGROUP_PROJECT16: TextureGroup = TextureGroup(49);
    pub const TEXTUREGROUP_PROJECT17: TextureGroup = TextureGroup(50);
    pub const TEXTUREGROUP_PROJECT18: TextureGroup = TextureGroup(51);
    pub const TEXTUREGROUP_PROJECT19: TextureGroup = TextureGroup(52);
    pub const TEXTUREGROUP_PROJECT20: TextureGroup = TextureGroup(53);
    pub const TEXTUREGROUP_PROJECT21: TextureGroup = TextureGroup(54);
    pub const TEXTUREGROUP_PROJECT22: TextureGroup = TextureGroup(55);
    pub const TEXTUREGROUP_PROJECT23: TextureGroup = TextureGroup(56);
    pub const TEXTUREGROUP_PROJECT24: TextureGroup = TextureGroup(57);
    pub const TEXTUREGROUP_PROJECT25: TextureGroup = TextureGroup(58);
    pub const TEXTUREGROUP_PROJECT26: TextureGroup = TextureGroup(59);
    pub const TEXTUREGROUP_PROJECT27: TextureGroup = TextureGroup(60);
    pub const TEXTUREGROUP_PROJECT28: TextureGroup = TextureGroup(61);
    pub const TEXTUREGROUP_PROJECT29: TextureGroup = TextureGroup(62);
    pub const TEXTUREGROUP_PROJECT30: TextureGroup = TextureGroup(63);
    pub const TEXTUREGROUP_PROJECT31: TextureGroup = TextureGroup(64);
    pub const TEXTUREGROUP_PROJECT32: TextureGroup = TextureGroup(65);
    pub const TEXTUREGROUP_PROJECT33: TextureGroup = TextureGroup(66);
    pub const TEXTUREGROUP_PROJECT34: TextureGroup = TextureGroup(67);
    pub const TEXTUREGROUP_PROJECT35: TextureGroup = TextureGroup(68);
    pub const TEXTUREGROUP_PROJECT36: TextureGroup = TextureGroup(69);
    pub const TEXTUREGROUP_PROJECT37: TextureGroup = TextureGroup(70);
    pub const TEXTUREGROUP_PROJECT38: TextureGroup = TextureGroup(71);
    pub const TEXTUREGROUP_PROJECT39: TextureGroup = TextureGroup(72);
    pub const TEXTUREGROUP_PROJECT40: TextureGroup = TextureGroup(73);
    pub const TEXTUREGROUP_PROJECT41: TextureGroup = TextureGroup(74);
    pub const TEXTUREGROUP_PROJECT42: TextureGroup = TextureGroup(75);
    pub const TEXTUREGROUP_PROJECT43: TextureGroup = TextureGroup(76);
    pub const TEXTUREGROUP_PROJECT44: TextureGroup = TextureGroup(77);
    pub const TEXTUREGROUP_PROJECT45: TextureGroup = TextureGroup(78);
    pub const TEXTUREGROUP_PROJECT46: TextureGroup = TextureGroup(79);
    pub const TEXTUREGROUP_PROJECT47: TextureGroup = TextureGroup(80);
    pub const TEXTUREGROUP_PROJECT48: TextureGroup = TextureGroup(81);
}
#[repr(transparent)]
pub struct TextureMipGenSettings(pub u8);
impl TextureMipGenSettings {
    pub const TMGS_FROM_TEXTURE_GROUP: TextureMipGenSettings = TextureMipGenSettings(0);
    pub const TMGS_SIMPLE_AVERAGE: TextureMipGenSettings = TextureMipGenSettings(1);
    pub const TMGS_SHARPEN0: TextureMipGenSettings = TextureMipGenSettings(2);
    pub const TMGS_SHARPEN1: TextureMipGenSettings = TextureMipGenSettings(3);
    pub const TMGS_SHARPEN2: TextureMipGenSettings = TextureMipGenSettings(4);
    pub const TMGS_SHARPEN3: TextureMipGenSettings = TextureMipGenSettings(5);
    pub const TMGS_SHARPEN4: TextureMipGenSettings = TextureMipGenSettings(6);
    pub const TMGS_SHARPEN5: TextureMipGenSettings = TextureMipGenSettings(7);
    pub const TMGS_SHARPEN6: TextureMipGenSettings = TextureMipGenSettings(8);
    pub const TMGS_SHARPEN7: TextureMipGenSettings = TextureMipGenSettings(9);
    pub const TMGS_SHARPEN8: TextureMipGenSettings = TextureMipGenSettings(10);
    pub const TMGS_SHARPEN9: TextureMipGenSettings = TextureMipGenSettings(11);
    pub const TMGS_SHARPEN10: TextureMipGenSettings = TextureMipGenSettings(12);
    pub const TMGS_NO_MIPMAPS: TextureMipGenSettings = TextureMipGenSettings(13);
    pub const TMGS_LEAVE_EXISTING_MIPS: TextureMipGenSettings = TextureMipGenSettings(
        14,
    );
    pub const TMGS_BLUR1: TextureMipGenSettings = TextureMipGenSettings(15);
    pub const TMGS_BLUR2: TextureMipGenSettings = TextureMipGenSettings(16);
    pub const TMGS_BLUR3: TextureMipGenSettings = TextureMipGenSettings(17);
    pub const TMGS_BLUR4: TextureMipGenSettings = TextureMipGenSettings(18);
    pub const TMGS_BLUR5: TextureMipGenSettings = TextureMipGenSettings(19);
    pub const TMGS_UNFILTERED: TextureMipGenSettings = TextureMipGenSettings(20);
    pub const TMGS_ANGULAR: TextureMipGenSettings = TextureMipGenSettings(21);
}
#[repr(transparent)]
pub struct ETextureMipLoadOptions(pub u8);
impl ETextureMipLoadOptions {
    pub const DEFAULT: ETextureMipLoadOptions = ETextureMipLoadOptions(0);
    pub const ALL_MIPS: ETextureMipLoadOptions = ETextureMipLoadOptions(1);
    pub const ONLY_FIRST_MIP: ETextureMipLoadOptions = ETextureMipLoadOptions(2);
}
#[repr(transparent)]
pub struct ETextureDownscaleOptions(pub u8);
impl ETextureDownscaleOptions {
    pub const DEFAULT: ETextureDownscaleOptions = ETextureDownscaleOptions(0);
    pub const UNFILTERED: ETextureDownscaleOptions = ETextureDownscaleOptions(1);
    pub const SIMPLE_AVERAGE: ETextureDownscaleOptions = ETextureDownscaleOptions(2);
    pub const SHARPEN0: ETextureDownscaleOptions = ETextureDownscaleOptions(3);
    pub const SHARPEN1: ETextureDownscaleOptions = ETextureDownscaleOptions(4);
    pub const SHARPEN2: ETextureDownscaleOptions = ETextureDownscaleOptions(5);
    pub const SHARPEN3: ETextureDownscaleOptions = ETextureDownscaleOptions(6);
    pub const SHARPEN4: ETextureDownscaleOptions = ETextureDownscaleOptions(7);
    pub const SHARPEN5: ETextureDownscaleOptions = ETextureDownscaleOptions(8);
    pub const SHARPEN6: ETextureDownscaleOptions = ETextureDownscaleOptions(9);
    pub const SHARPEN7: ETextureDownscaleOptions = ETextureDownscaleOptions(10);
    pub const SHARPEN8: ETextureDownscaleOptions = ETextureDownscaleOptions(11);
    pub const SHARPEN9: ETextureDownscaleOptions = ETextureDownscaleOptions(12);
    pub const SHARPEN10: ETextureDownscaleOptions = ETextureDownscaleOptions(13);
}
#[repr(transparent)]
pub struct ETextureLossyCompressionAmount(pub u8);
impl ETextureLossyCompressionAmount {
    pub const TLCA_DEFAULT: ETextureLossyCompressionAmount = ETextureLossyCompressionAmount(
        0,
    );
    pub const TLCA_NONE: ETextureLossyCompressionAmount = ETextureLossyCompressionAmount(
        1,
    );
    pub const TLCA_LOWEST: ETextureLossyCompressionAmount = ETextureLossyCompressionAmount(
        2,
    );
    pub const TLCA_LOW: ETextureLossyCompressionAmount = ETextureLossyCompressionAmount(
        3,
    );
    pub const TLCA_MEDIUM: ETextureLossyCompressionAmount = ETextureLossyCompressionAmount(
        4,
    );
    pub const TLCA_HIGH: ETextureLossyCompressionAmount = ETextureLossyCompressionAmount(
        5,
    );
    pub const TLCA_HIGHEST: ETextureLossyCompressionAmount = ETextureLossyCompressionAmount(
        6,
    );
}
#[repr(transparent)]
pub struct EAnimInstanceLocatorFragmentType(pub i32);
impl EAnimInstanceLocatorFragmentType {
    pub const ANIM_INSTANCE: EAnimInstanceLocatorFragmentType = EAnimInstanceLocatorFragmentType(
        0,
    );
    pub const POST_PROCESS_ANIM_INSTANCE: EAnimInstanceLocatorFragmentType = EAnimInstanceLocatorFragmentType(
        1,
    );
}
#[repr(transparent)]
pub struct EHardwareDevicePrimaryType(pub u8);
impl EHardwareDevicePrimaryType {
    pub const UNSPECIFIED: EHardwareDevicePrimaryType = EHardwareDevicePrimaryType(0);
    pub const KEYBOARD_AND_MOUSE: EHardwareDevicePrimaryType = EHardwareDevicePrimaryType(
        1,
    );
    pub const GAMEPAD: EHardwareDevicePrimaryType = EHardwareDevicePrimaryType(2);
    pub const TOUCH: EHardwareDevicePrimaryType = EHardwareDevicePrimaryType(3);
    pub const MOTION_TRACKING: EHardwareDevicePrimaryType = EHardwareDevicePrimaryType(
        4,
    );
    pub const RACING_WHEEL: EHardwareDevicePrimaryType = EHardwareDevicePrimaryType(5);
    pub const FLIGHT_STICK: EHardwareDevicePrimaryType = EHardwareDevicePrimaryType(6);
    pub const CAMERA: EHardwareDevicePrimaryType = EHardwareDevicePrimaryType(7);
    pub const INSTRUMENT: EHardwareDevicePrimaryType = EHardwareDevicePrimaryType(8);
    pub const CUSTOM_TYPE_A: EHardwareDevicePrimaryType = EHardwareDevicePrimaryType(9);
    pub const CUSTOM_TYPE_B: EHardwareDevicePrimaryType = EHardwareDevicePrimaryType(10);
    pub const CUSTOM_TYPE_C: EHardwareDevicePrimaryType = EHardwareDevicePrimaryType(11);
    pub const CUSTOM_TYPE_D: EHardwareDevicePrimaryType = EHardwareDevicePrimaryType(12);
}
#[repr(transparent)]
pub struct EExportHLODMeshOrigin(pub u8);
impl EExportHLODMeshOrigin {
    pub const ACTOR: EExportHLODMeshOrigin = EExportHLODMeshOrigin(0);
    pub const WORLD: EExportHLODMeshOrigin = EExportHLODMeshOrigin(1);
}
#[repr(transparent)]
pub struct EVolumeLightingMethod(pub u8);
impl EVolumeLightingMethod {
    pub const VLM_VOLUMETRIC_LIGHTMAP: EVolumeLightingMethod = EVolumeLightingMethod(0);
    pub const VLM_SPARSE_VOLUME_LIGHTING_SAMPLES: EVolumeLightingMethod = EVolumeLightingMethod(
        1,
    );
}
#[repr(transparent)]
pub struct TextureFilter(pub u8);
impl TextureFilter {
    pub const TF_NEAREST: TextureFilter = TextureFilter(0);
    pub const TF_BILINEAR: TextureFilter = TextureFilter(1);
    pub const TF_TRILINEAR: TextureFilter = TextureFilter(2);
    pub const TF_DEFAULT: TextureFilter = TextureFilter(3);
}
#[repr(transparent)]
pub struct TextureAddress(pub u8);
impl TextureAddress {
    pub const TA_WRAP: TextureAddress = TextureAddress(0);
    pub const TA_CLAMP: TextureAddress = TextureAddress(1);
    pub const TA_MIRROR: TextureAddress = TextureAddress(2);
}
#[repr(transparent)]
pub struct EConstraintTransformComponentFlags(pub u8);
impl EConstraintTransformComponentFlags {
    pub const NONE: EConstraintTransformComponentFlags = EConstraintTransformComponentFlags(
        0,
    );
    pub const CHILD_POSITION: EConstraintTransformComponentFlags = EConstraintTransformComponentFlags(
        1,
    );
    pub const CHILD_ROTATION: EConstraintTransformComponentFlags = EConstraintTransformComponentFlags(
        2,
    );
    pub const PARENT_POSITION: EConstraintTransformComponentFlags = EConstraintTransformComponentFlags(
        4,
    );
    pub const PARENT_ROTATION: EConstraintTransformComponentFlags = EConstraintTransformComponentFlags(
        8,
    );
    pub const ALL_CHILD: EConstraintTransformComponentFlags = EConstraintTransformComponentFlags(
        3,
    );
    pub const ALL_PARENT: EConstraintTransformComponentFlags = EConstraintTransformComponentFlags(
        12,
    );
    pub const ALL_POSITION: EConstraintTransformComponentFlags = EConstraintTransformComponentFlags(
        5,
    );
    pub const ALL_ROTATION: EConstraintTransformComponentFlags = EConstraintTransformComponentFlags(
        10,
    );
    pub const ALL: EConstraintTransformComponentFlags = EConstraintTransformComponentFlags(
        15,
    );
}
#[repr(transparent)]
pub struct ESceneDepthPriorityGroup(pub u8);
impl ESceneDepthPriorityGroup {
    pub const SDPG_WORLD: ESceneDepthPriorityGroup = ESceneDepthPriorityGroup(0);
    pub const SDPG_FOREGROUND: ESceneDepthPriorityGroup = ESceneDepthPriorityGroup(1);
}
#[repr(transparent)]
pub struct EFFTSize(pub u8);
impl EFFTSize {
    pub const DEFAULT_SIZE: EFFTSize = EFFTSize(0);
    pub const MIN: EFFTSize = EFFTSize(1);
    pub const SMALL: EFFTSize = EFFTSize(2);
    pub const MEDIUM: EFFTSize = EFFTSize(3);
    pub const LARGE: EFFTSize = EFFTSize(4);
    pub const VERY_LARGE: EFFTSize = EFFTSize(5);
    pub const MAX: EFFTSize = EFFTSize(6);
}
#[repr(transparent)]
pub struct EPerQualityLevels(pub u8);
impl EPerQualityLevels {
    pub const LOW: EPerQualityLevels = EPerQualityLevels(0);
    pub const MEDIUM: EPerQualityLevels = EPerQualityLevels(1);
    pub const HIGH: EPerQualityLevels = EPerQualityLevels(2);
    pub const EPIC: EPerQualityLevels = EPerQualityLevels(3);
    pub const CINEMATIC: EPerQualityLevels = EPerQualityLevels(4);
    pub const NUM: EPerQualityLevels = EPerQualityLevels(5);
}
#[repr(transparent)]
pub struct EEndPlayReason(pub u8);
impl EEndPlayReason {
    pub const DESTROYED: EEndPlayReason = EEndPlayReason(0);
    pub const LEVEL_TRANSITION: EEndPlayReason = EEndPlayReason(1);
    pub const END_PLAY_IN_EDITOR: EEndPlayReason = EEndPlayReason(2);
    pub const REMOVED_FROM_WORLD: EEndPlayReason = EEndPlayReason(3);
    pub const QUIT: EEndPlayReason = EEndPlayReason(4);
}
#[repr(transparent)]
pub struct EPlaneConstraintAxisSetting(pub u8);
impl EPlaneConstraintAxisSetting {
    pub const CUSTOM: EPlaneConstraintAxisSetting = EPlaneConstraintAxisSetting(0);
    pub const X: EPlaneConstraintAxisSetting = EPlaneConstraintAxisSetting(1);
    pub const Y: EPlaneConstraintAxisSetting = EPlaneConstraintAxisSetting(2);
    pub const Z: EPlaneConstraintAxisSetting = EPlaneConstraintAxisSetting(3);
    pub const USE_GLOBAL_PHYSICS_SETTING: EPlaneConstraintAxisSetting = EPlaneConstraintAxisSetting(
        4,
    );
}
#[repr(transparent)]
pub struct EDrawDebugSceneDepthPriorityGroup(pub u8);
impl EDrawDebugSceneDepthPriorityGroup {
    pub const WORLD: EDrawDebugSceneDepthPriorityGroup = EDrawDebugSceneDepthPriorityGroup(
        0,
    );
    pub const FOREGROUND: EDrawDebugSceneDepthPriorityGroup = EDrawDebugSceneDepthPriorityGroup(
        1,
    );
}
#[repr(transparent)]
pub struct EEditorPropertyValueState(pub u8);
impl EEditorPropertyValueState {
    pub const DEFAULT: EEditorPropertyValueState = EEditorPropertyValueState(0);
    pub const OVERRIDDEN: EEditorPropertyValueState = EEditorPropertyValueState(1);
    pub const NOT_FOUND: EEditorPropertyValueState = EEditorPropertyValueState(2);
    pub const ACCESS_DENIED: EEditorPropertyValueState = EEditorPropertyValueState(3);
}
#[repr(transparent)]
pub struct EMoveComponentAction(pub u8);
impl EMoveComponentAction {
    pub const MOVE: EMoveComponentAction = EMoveComponentAction(0);
    pub const STOP: EMoveComponentAction = EMoveComponentAction(1);
    pub const RETURN: EMoveComponentAction = EMoveComponentAction(2);
}
#[repr(transparent)]
pub struct EQuitPreference(pub u8);
impl EQuitPreference {
    pub const QUIT: EQuitPreference = EQuitPreference(0);
    pub const BACKGROUND: EQuitPreference = EQuitPreference(1);
}
#[repr(transparent)]
pub struct ETravelFailure(pub u8);
impl ETravelFailure {
    pub const NO_LEVEL: ETravelFailure = ETravelFailure(0);
    pub const LOAD_MAP_FAILURE: ETravelFailure = ETravelFailure(1);
    pub const INVALID_URL: ETravelFailure = ETravelFailure(2);
    pub const PACKAGE_MISSING: ETravelFailure = ETravelFailure(3);
    pub const PACKAGE_VERSION: ETravelFailure = ETravelFailure(4);
    pub const NO_DOWNLOAD: ETravelFailure = ETravelFailure(5);
    pub const TRAVEL_FAILURE: ETravelFailure = ETravelFailure(6);
    pub const CHEAT_COMMANDS: ETravelFailure = ETravelFailure(7);
    pub const PENDING_NET_GAME_CREATE_FAILURE: ETravelFailure = ETravelFailure(8);
    pub const CLOUD_SAVE_FAILURE: ETravelFailure = ETravelFailure(9);
    pub const SERVER_TRAVEL_FAILURE: ETravelFailure = ETravelFailure(10);
    pub const CLIENT_TRAVEL_FAILURE: ETravelFailure = ETravelFailure(11);
}
#[repr(transparent)]
pub struct ERelativeTransformSpace(pub u8);
impl ERelativeTransformSpace {
    pub const RTS_WORLD: ERelativeTransformSpace = ERelativeTransformSpace(0);
    pub const RTS_ACTOR: ERelativeTransformSpace = ERelativeTransformSpace(1);
    pub const RTS_COMPONENT: ERelativeTransformSpace = ERelativeTransformSpace(2);
    pub const RTS_PARENT_BONE_SPACE: ERelativeTransformSpace = ERelativeTransformSpace(
        3,
    );
}
#[repr(transparent)]
pub struct EAttachmentRule(pub u8);
impl EAttachmentRule {
    pub const KEEP_RELATIVE: EAttachmentRule = EAttachmentRule(0);
    pub const KEEP_WORLD: EAttachmentRule = EAttachmentRule(1);
    pub const SNAP_TO_TARGET: EAttachmentRule = EAttachmentRule(2);
}
#[repr(transparent)]
pub struct EDetachmentRule(pub u8);
impl EDetachmentRule {
    pub const KEEP_RELATIVE: EDetachmentRule = EDetachmentRule(0);
    pub const KEEP_WORLD: EDetachmentRule = EDetachmentRule(1);
}
#[repr(transparent)]
pub struct EComponentPhysicsStateChange(pub u8);
impl EComponentPhysicsStateChange {
    pub const CREATED: EComponentPhysicsStateChange = EComponentPhysicsStateChange(0);
    pub const DESTROYED: EComponentPhysicsStateChange = EComponentPhysicsStateChange(1);
}
#[repr(transparent)]
pub struct EHLODLevelExclusion(pub u8);
impl EHLODLevelExclusion {
    pub const HLOD0: EHLODLevelExclusion = EHLODLevelExclusion(1);
    pub const HLOD1: EHLODLevelExclusion = EHLODLevelExclusion(2);
    pub const HLOD2: EHLODLevelExclusion = EHLODLevelExclusion(4);
    pub const HLOD3: EHLODLevelExclusion = EHLODLevelExclusion(8);
    pub const HLOD4: EHLODLevelExclusion = EHLODLevelExclusion(16);
    pub const HLOD5: EHLODLevelExclusion = EHLODLevelExclusion(32);
    pub const HLOD6: EHLODLevelExclusion = EHLODLevelExclusion(64);
    pub const HLOD7: EHLODLevelExclusion = EHLODLevelExclusion(128);
}
#[repr(transparent)]
pub struct EFirstPersonPrimitiveType(pub u8);
impl EFirstPersonPrimitiveType {
    pub const NONE: EFirstPersonPrimitiveType = EFirstPersonPrimitiveType(0);
    pub const FIRST_PERSON: EFirstPersonPrimitiveType = EFirstPersonPrimitiveType(1);
    pub const WORLD_SPACE_REPRESENTATION: EFirstPersonPrimitiveType = EFirstPersonPrimitiveType(
        2,
    );
}
#[repr(transparent)]
pub struct EMouseLockMode(pub u8);
impl EMouseLockMode {
    pub const DO_NOT_LOCK: EMouseLockMode = EMouseLockMode(0);
    pub const LOCK_ON_CAPTURE: EMouseLockMode = EMouseLockMode(1);
    pub const LOCK_ALWAYS: EMouseLockMode = EMouseLockMode(2);
    pub const LOCK_IN_FULLSCREEN: EMouseLockMode = EMouseLockMode(3);
}
#[repr(transparent)]
pub struct EWindowTitleBarMode(pub u8);
impl EWindowTitleBarMode {
    pub const OVERLAY: EWindowTitleBarMode = EWindowTitleBarMode(0);
    pub const VERTICAL_BOX: EWindowTitleBarMode = EWindowTitleBarMode(1);
}
#[repr(transparent)]
pub struct ENetRole(pub u8);
impl ENetRole {
    pub const ROLE_NONE: ENetRole = ENetRole(0);
    pub const ROLE_SIMULATED_PROXY: ENetRole = ENetRole(1);
    pub const ROLE_AUTONOMOUS_PROXY: ENetRole = ENetRole(2);
    pub const ROLE_AUTHORITY: ENetRole = ENetRole(3);
}
#[repr(transparent)]
pub struct ENetDormancy(pub u8);
impl ENetDormancy {
    pub const DORM_NEVER: ENetDormancy = ENetDormancy(0);
    pub const DORM_AWAKE: ENetDormancy = ENetDormancy(1);
    pub const DORM_DORMANT_ALL: ENetDormancy = ENetDormancy(2);
    pub const DORM_DORMANT_PARTIAL: ENetDormancy = ENetDormancy(3);
    pub const DORM_INITIAL: ENetDormancy = ENetDormancy(4);
}
#[repr(transparent)]
pub struct EMaterialUsage(pub u8);
impl EMaterialUsage {
    pub const MATUSAGE_SKELETAL_MESH: EMaterialUsage = EMaterialUsage(0);
    pub const MATUSAGE_PARTICLE_SPRITES: EMaterialUsage = EMaterialUsage(1);
    pub const MATUSAGE_BEAM_TRAILS: EMaterialUsage = EMaterialUsage(2);
    pub const MATUSAGE_MESH_PARTICLES: EMaterialUsage = EMaterialUsage(3);
    pub const MATUSAGE_STATIC_LIGHTING: EMaterialUsage = EMaterialUsage(4);
    pub const MATUSAGE_MORPH_TARGETS: EMaterialUsage = EMaterialUsage(5);
    pub const MATUSAGE_SPLINE_MESH: EMaterialUsage = EMaterialUsage(6);
    pub const MATUSAGE_INSTANCED_STATIC_MESHES: EMaterialUsage = EMaterialUsage(7);
    pub const MATUSAGE_GEOMETRY_COLLECTIONS: EMaterialUsage = EMaterialUsage(8);
    pub const MATUSAGE_CLOTHING: EMaterialUsage = EMaterialUsage(9);
    pub const MATUSAGE_NIAGARA_SPRITES: EMaterialUsage = EMaterialUsage(10);
    pub const MATUSAGE_NIAGARA_RIBBONS: EMaterialUsage = EMaterialUsage(11);
    pub const MATUSAGE_NIAGARA_MESH_PARTICLES: EMaterialUsage = EMaterialUsage(12);
    pub const MATUSAGE_GEOMETRY_CACHE: EMaterialUsage = EMaterialUsage(13);
    pub const MATUSAGE_WATER: EMaterialUsage = EMaterialUsage(14);
    pub const MATUSAGE_HAIR_STRANDS: EMaterialUsage = EMaterialUsage(15);
    pub const MATUSAGE_LIDAR_POINT_CLOUD: EMaterialUsage = EMaterialUsage(16);
    pub const MATUSAGE_VIRTUAL_HEIGHTFIELD_MESH: EMaterialUsage = EMaterialUsage(17);
    pub const MATUSAGE_NANITE: EMaterialUsage = EMaterialUsage(18);
    pub const MATUSAGE_VOXELS: EMaterialUsage = EMaterialUsage(19);
    pub const MATUSAGE_VOLUMETRIC_CLOUD: EMaterialUsage = EMaterialUsage(20);
    pub const MATUSAGE_HETEROGENEOUS_VOLUMES: EMaterialUsage = EMaterialUsage(21);
    pub const MATUSAGE_STATIC_MESH: EMaterialUsage = EMaterialUsage(22);
}
#[repr(transparent)]
pub struct EMovementMode(pub u8);
impl EMovementMode {
    pub const MOVE_NONE: EMovementMode = EMovementMode(0);
    pub const MOVE_WALKING: EMovementMode = EMovementMode(1);
    pub const MOVE_NAV_WALKING: EMovementMode = EMovementMode(2);
    pub const MOVE_FALLING: EMovementMode = EMovementMode(3);
    pub const MOVE_SWIMMING: EMovementMode = EMovementMode(4);
    pub const MOVE_FLYING: EMovementMode = EMovementMode(5);
    pub const MOVE_CUSTOM: EMovementMode = EMovementMode(6);
}
#[repr(transparent)]
pub struct ENavigationOptionFlag(pub u8);
impl ENavigationOptionFlag {
    pub const DEFAULT: ENavigationOptionFlag = ENavigationOptionFlag(0);
    pub const ENABLE: ENavigationOptionFlag = ENavigationOptionFlag(1);
    pub const DISABLE: ENavigationOptionFlag = ENavigationOptionFlag(2);
    pub const MAX: ENavigationOptionFlag = ENavigationOptionFlag(3);
}
#[repr(transparent)]
pub struct ENavigationQueryResult(pub u8);
impl ENavigationQueryResult {
    pub const INVALID: ENavigationQueryResult = ENavigationQueryResult(0);
    pub const ERROR: ENavigationQueryResult = ENavigationQueryResult(1);
    pub const FAIL: ENavigationQueryResult = ENavigationQueryResult(2);
    pub const SUCCESS: ENavigationQueryResult = ENavigationQueryResult(3);
}
#[repr(transparent)]
pub struct ENavDataGatheringModeConfig(pub u8);
impl ENavDataGatheringModeConfig {
    pub const INVALID: ENavDataGatheringModeConfig = ENavDataGatheringModeConfig(0);
    pub const INSTANT: ENavDataGatheringModeConfig = ENavDataGatheringModeConfig(1);
    pub const LAZY: ENavDataGatheringModeConfig = ENavDataGatheringModeConfig(2);
}
#[repr(transparent)]
pub struct EDataLayerRuntimeState(pub u8);
impl EDataLayerRuntimeState {
    pub const UNLOADED: EDataLayerRuntimeState = EDataLayerRuntimeState(0);
    pub const LOADED: EDataLayerRuntimeState = EDataLayerRuntimeState(1);
    pub const ACTIVATED: EDataLayerRuntimeState = EDataLayerRuntimeState(2);
}
#[repr(transparent)]
pub struct ELightingBuildQuality(pub u8);
impl ELightingBuildQuality {
    pub const QUALITY_PREVIEW: ELightingBuildQuality = ELightingBuildQuality(0);
    pub const QUALITY_MEDIUM: ELightingBuildQuality = ELightingBuildQuality(1);
    pub const QUALITY_HIGH: ELightingBuildQuality = ELightingBuildQuality(2);
    pub const QUALITY_PRODUCTION: ELightingBuildQuality = ELightingBuildQuality(3);
}
#[repr(transparent)]
pub struct EAnimCurveType(pub u8);
impl EAnimCurveType {
    pub const ATTRIBUTE_CURVE: EAnimCurveType = EAnimCurveType(0);
    pub const MATERIAL_CURVE: EAnimCurveType = EAnimCurveType(1);
    pub const MORPH_TARGET_CURVE: EAnimCurveType = EAnimCurveType(2);
    pub const MAX_ANIM_CURVE_TYPE: EAnimCurveType = EAnimCurveType(3);
}
#[repr(transparent)]
pub struct EMontagePlayReturnType(pub u8);
impl EMontagePlayReturnType {
    pub const MONTAGE_LENGTH: EMontagePlayReturnType = EMontagePlayReturnType(0);
    pub const DURATION: EMontagePlayReturnType = EMontagePlayReturnType(1);
}
#[repr(transparent)]
pub struct ETransitionRequestQueueMode(pub u8);
impl ETransitionRequestQueueMode {
    pub const SHARED: ETransitionRequestQueueMode = ETransitionRequestQueueMode(0);
    pub const UNIQUE: ETransitionRequestQueueMode = ETransitionRequestQueueMode(1);
}
#[repr(transparent)]
pub struct ETransitionRequestOverwriteMode(pub u8);
impl ETransitionRequestOverwriteMode {
    pub const APPEND: ETransitionRequestOverwriteMode = ETransitionRequestOverwriteMode(
        0,
    );
    pub const IGNORE: ETransitionRequestOverwriteMode = ETransitionRequestOverwriteMode(
        1,
    );
    pub const OVERWRITE: ETransitionRequestOverwriteMode = ETransitionRequestOverwriteMode(
        2,
    );
}
#[repr(transparent)]
pub struct ETeleportType(pub u8);
impl ETeleportType {
    pub const NONE: ETeleportType = ETeleportType(0);
    pub const TELEPORT_PHYSICS: ETeleportType = ETeleportType(1);
    pub const RESET_PHYSICS: ETeleportType = ETeleportType(2);
}
#[repr(transparent)]
pub struct ERootMotionMode(pub u8);
impl ERootMotionMode {
    pub const NO_ROOT_MOTION_EXTRACTION: ERootMotionMode = ERootMotionMode(0);
    pub const IGNORE_ROOT_MOTION: ERootMotionMode = ERootMotionMode(1);
    pub const ROOT_MOTION_FROM_EVERYTHING: ERootMotionMode = ERootMotionMode(2);
    pub const ROOT_MOTION_FROM_MONTAGES_ONLY: ERootMotionMode = ERootMotionMode(3);
}
#[repr(transparent)]
pub struct EAdditiveBasePoseType(pub u8);
impl EAdditiveBasePoseType {
    pub const ABPT_NONE: EAdditiveBasePoseType = EAdditiveBasePoseType(0);
    pub const ABPT_REF_POSE: EAdditiveBasePoseType = EAdditiveBasePoseType(1);
    pub const ABPT_ANIM_SCALED: EAdditiveBasePoseType = EAdditiveBasePoseType(2);
    pub const ABPT_ANIM_FRAME: EAdditiveBasePoseType = EAdditiveBasePoseType(3);
    pub const ABPT_LOCAL_ANIM_FRAME: EAdditiveBasePoseType = EAdditiveBasePoseType(4);
}
#[repr(transparent)]
pub struct ERootMotionRootLock(pub u8);
impl ERootMotionRootLock {
    pub const REF_POSE: ERootMotionRootLock = ERootMotionRootLock(0);
    pub const ANIM_FIRST_FRAME: ERootMotionRootLock = ERootMotionRootLock(1);
    pub const ZERO: ERootMotionRootLock = ERootMotionRootLock(2);
}
#[repr(transparent)]
pub struct EAnimNodeReferenceConversionResult(pub u8);
impl EAnimNodeReferenceConversionResult {
    pub const SUCCEEDED: EAnimNodeReferenceConversionResult = EAnimNodeReferenceConversionResult(
        1,
    );
    pub const FAILED: EAnimNodeReferenceConversionResult = EAnimNodeReferenceConversionResult(
        0,
    );
}
#[repr(transparent)]
pub struct EAnimExecutionContextConversionResult(pub u8);
impl EAnimExecutionContextConversionResult {
    pub const SUCCEEDED: EAnimExecutionContextConversionResult = EAnimExecutionContextConversionResult(
        1,
    );
    pub const FAILED: EAnimExecutionContextConversionResult = EAnimExecutionContextConversionResult(
        0,
    );
}
#[repr(transparent)]
pub struct ETimecodeProviderSynchronizationState(pub i32);
impl ETimecodeProviderSynchronizationState {
    pub const CLOSED: ETimecodeProviderSynchronizationState = ETimecodeProviderSynchronizationState(
        0,
    );
    pub const ERROR: ETimecodeProviderSynchronizationState = ETimecodeProviderSynchronizationState(
        1,
    );
    pub const SYNCHRONIZED: ETimecodeProviderSynchronizationState = ETimecodeProviderSynchronizationState(
        2,
    );
    pub const SYNCHRONIZING: ETimecodeProviderSynchronizationState = ETimecodeProviderSynchronizationState(
        3,
    );
}
#[repr(transparent)]
pub struct EAudioSpectrumBandPresetType(pub u8);
impl EAudioSpectrumBandPresetType {
    pub const KICK_DRUM: EAudioSpectrumBandPresetType = EAudioSpectrumBandPresetType(0);
    pub const SNARE_DRUM: EAudioSpectrumBandPresetType = EAudioSpectrumBandPresetType(1);
    pub const VOICE: EAudioSpectrumBandPresetType = EAudioSpectrumBandPresetType(2);
    pub const CYMBALS: EAudioSpectrumBandPresetType = EAudioSpectrumBandPresetType(3);
}
#[repr(transparent)]
pub struct EFFTPeakInterpolationMethod(pub u8);
impl EFFTPeakInterpolationMethod {
    pub const NEAREST_NEIGHBOR: EFFTPeakInterpolationMethod = EFFTPeakInterpolationMethod(
        0,
    );
    pub const LINEAR: EFFTPeakInterpolationMethod = EFFTPeakInterpolationMethod(1);
    pub const QUADRATIC: EFFTPeakInterpolationMethod = EFFTPeakInterpolationMethod(2);
    pub const CONSTANT_Q: EFFTPeakInterpolationMethod = EFFTPeakInterpolationMethod(3);
}
#[repr(transparent)]
pub struct EFFTWindowType(pub u8);
impl EFFTWindowType {
    pub const NONE: EFFTWindowType = EFFTWindowType(0);
    pub const HAMMING: EFFTWindowType = EFFTWindowType(1);
    pub const HANN: EFFTWindowType = EFFTWindowType(2);
    pub const BLACKMAN: EFFTWindowType = EFFTWindowType(3);
}
#[repr(transparent)]
pub struct EAudioSpectrumType(pub u8);
impl EAudioSpectrumType {
    pub const MAGNITUDE_SPECTRUM: EAudioSpectrumType = EAudioSpectrumType(0);
    pub const POWER_SPECTRUM: EAudioSpectrumType = EAudioSpectrumType(1);
    pub const DECIBEL: EAudioSpectrumType = EAudioSpectrumType(2);
}
#[repr(transparent)]
pub struct EAudioRecordingExportType(pub u8);
impl EAudioRecordingExportType {
    pub const SOUND_WAVE: EAudioRecordingExportType = EAudioRecordingExportType(0);
    pub const WAV_FILE: EAudioRecordingExportType = EAudioRecordingExportType(1);
}
#[repr(transparent)]
pub struct ESoundAssetCompressionType(pub u8);
impl ESoundAssetCompressionType {
    pub const BINK_AUDIO: ESoundAssetCompressionType = ESoundAssetCompressionType(0);
    pub const ADPCM: ESoundAssetCompressionType = ESoundAssetCompressionType(1);
    pub const PCM: ESoundAssetCompressionType = ESoundAssetCompressionType(2);
    pub const OPUS: ESoundAssetCompressionType = ESoundAssetCompressionType(3);
    pub const PLATFORM_SPECIFIC: ESoundAssetCompressionType = ESoundAssetCompressionType(
        4,
    );
    pub const PROJECT_DEFINED: ESoundAssetCompressionType = ESoundAssetCompressionType(
        5,
    );
    pub const RAD_AUDIO: ESoundAssetCompressionType = ESoundAssetCompressionType(6);
}
#[repr(transparent)]
pub struct EAudioFaderCurve(pub u8);
impl EAudioFaderCurve {
    pub const LINEAR: EAudioFaderCurve = EAudioFaderCurve(0);
    pub const LOGARITHMIC: EAudioFaderCurve = EAudioFaderCurve(1);
    pub const S_CURVE: EAudioFaderCurve = EAudioFaderCurve(2);
    pub const SIN: EAudioFaderCurve = EAudioFaderCurve(3);
    pub const COUNT: EAudioFaderCurve = EAudioFaderCurve(4);
}
#[repr(transparent)]
pub struct EModulationDestination(pub u8);
impl EModulationDestination {
    pub const VOLUME: EModulationDestination = EModulationDestination(0);
    pub const PITCH: EModulationDestination = EModulationDestination(1);
    pub const LOWPASS: EModulationDestination = EModulationDestination(2);
    pub const HIGHPASS: EModulationDestination = EModulationDestination(3);
    pub const COUNT: EModulationDestination = EModulationDestination(4);
}
#[repr(transparent)]
pub struct EQuartzCommandDelegateSubType(pub u8);
impl EQuartzCommandDelegateSubType {
    pub const COMMAND_ON_FAILED_TO_QUEUE: EQuartzCommandDelegateSubType = EQuartzCommandDelegateSubType(
        0,
    );
    pub const COMMAND_ON_QUEUED: EQuartzCommandDelegateSubType = EQuartzCommandDelegateSubType(
        1,
    );
    pub const COMMAND_ON_CANCELED: EQuartzCommandDelegateSubType = EQuartzCommandDelegateSubType(
        2,
    );
    pub const COMMAND_ON_ABOUT_TO_START: EQuartzCommandDelegateSubType = EQuartzCommandDelegateSubType(
        3,
    );
    pub const COMMAND_ON_STARTED: EQuartzCommandDelegateSubType = EQuartzCommandDelegateSubType(
        4,
    );
    pub const COUNT: EQuartzCommandDelegateSubType = EQuartzCommandDelegateSubType(5);
}
#[repr(transparent)]
pub struct ETextureStreamingMethod(pub u8);
impl ETextureStreamingMethod {
    pub const TSM_INVALID: ETextureStreamingMethod = ETextureStreamingMethod(0);
    pub const TSM_NOT_STREAMED: ETextureStreamingMethod = ETextureStreamingMethod(1);
    pub const TSM_STREAMED: ETextureStreamingMethod = ETextureStreamingMethod(2);
    pub const TSM_VIRTUAL_STREAMED: ETextureStreamingMethod = ETextureStreamingMethod(3);
}
#[repr(transparent)]
pub struct ESkinWeightProfileLayer(pub u8);
impl ESkinWeightProfileLayer {
    pub const PRIMARY: ESkinWeightProfileLayer = ESkinWeightProfileLayer(0);
    pub const SECONDARY: ESkinWeightProfileLayer = ESkinWeightProfileLayer(1);
}
#[repr(transparent)]
pub struct EPhysBodyOp(pub u8);
impl EPhysBodyOp {
    pub const PBO_NONE: EPhysBodyOp = EPhysBodyOp(0);
    pub const PBO_TERM: EPhysBodyOp = EPhysBodyOp(1);
}
#[repr(transparent)]
pub struct EAnimationMode(pub u8);
impl EAnimationMode {
    pub const ANIMATION_BLUEPRINT: EAnimationMode = EAnimationMode(0);
    pub const ANIMATION_SINGLE_NODE: EAnimationMode = EAnimationMode(1);
    pub const ANIMATION_CUSTOM_MODE: EAnimationMode = EAnimationMode(2);
}
#[repr(transparent)]
pub struct ECustomBoneAttributeLookup(pub u8);
impl ECustomBoneAttributeLookup {
    pub const BONE_ONLY: ECustomBoneAttributeLookup = ECustomBoneAttributeLookup(0);
    pub const IMMEDIATE_PARENT: ECustomBoneAttributeLookup = ECustomBoneAttributeLookup(
        1,
    );
    pub const PARENT_HIERARCHY: ECustomBoneAttributeLookup = ECustomBoneAttributeLookup(
        2,
    );
}
#[repr(transparent)]
pub struct ECastRayTracedShadow(pub u8);
impl ECastRayTracedShadow {
    pub const DISABLED: ECastRayTracedShadow = ECastRayTracedShadow(0);
    pub const USE_PROJECT_SETTING: ECastRayTracedShadow = ECastRayTracedShadow(1);
    pub const ENABLED: ECastRayTracedShadow = ECastRayTracedShadow(2);
}
#[repr(transparent)]
pub struct ETwitterRequestMethod(pub u8);
impl ETwitterRequestMethod {
    pub const TRM_GET: ETwitterRequestMethod = ETwitterRequestMethod(0);
    pub const TRM_POST: ETwitterRequestMethod = ETwitterRequestMethod(1);
    pub const TRM_DELETE: ETwitterRequestMethod = ETwitterRequestMethod(2);
}
#[repr(transparent)]
pub struct EWindSourceType(pub u8);
impl EWindSourceType {
    pub const DIRECTIONAL: EWindSourceType = EWindSourceType(0);
    pub const POINT: EWindSourceType = EWindSourceType(1);
}
#[repr(transparent)]
pub struct EAnimAssetCurveFlags(pub u8);
impl EAnimAssetCurveFlags {
    pub const AACF_NONE: EAnimAssetCurveFlags = EAnimAssetCurveFlags(0);
    pub const AACF_DRIVE_MORPH_TARGET_DEPRECATED: EAnimAssetCurveFlags = EAnimAssetCurveFlags(
        1,
    );
    pub const AACF_DRIVE_ATTRIBUTE_DEPRECATED: EAnimAssetCurveFlags = EAnimAssetCurveFlags(
        2,
    );
    pub const AACF_EDITABLE: EAnimAssetCurveFlags = EAnimAssetCurveFlags(4);
    pub const AACF_DRIVE_MATERIAL_DEPRECATED: EAnimAssetCurveFlags = EAnimAssetCurveFlags(
        8,
    );
    pub const AACF_METADATA: EAnimAssetCurveFlags = EAnimAssetCurveFlags(16);
    pub const AACF_DRIVE_TRACK: EAnimAssetCurveFlags = EAnimAssetCurveFlags(32);
    pub const AACF_DISABLED: EAnimAssetCurveFlags = EAnimAssetCurveFlags(64);
}
#[repr(transparent)]
pub struct EStructUtilsResult(pub u8);
impl EStructUtilsResult {
    pub const VALID: EStructUtilsResult = EStructUtilsResult(0);
    pub const NOT_VALID: EStructUtilsResult = EStructUtilsResult(1);
}
#[repr(transparent)]
pub struct EInterfaceValidResult(pub u8);
impl EInterfaceValidResult {
    pub const VALID: EInterfaceValidResult = EInterfaceValidResult(0);
    pub const INVALID: EInterfaceValidResult = EInterfaceValidResult(1);
}
#[repr(transparent)]
pub struct ETextureRenderTargetSampleCount(pub u8);
impl ETextureRenderTargetSampleCount {
    pub const RTSC_1: ETextureRenderTargetSampleCount = ETextureRenderTargetSampleCount(
        0,
    );
    pub const RTSC_2: ETextureRenderTargetSampleCount = ETextureRenderTargetSampleCount(
        1,
    );
    pub const RTSC_4: ETextureRenderTargetSampleCount = ETextureRenderTargetSampleCount(
        2,
    );
    pub const RTSC_8: ETextureRenderTargetSampleCount = ETextureRenderTargetSampleCount(
        3,
    );
}
#[repr(transparent)]
pub struct EAudioComponentPlayState(pub u8);
impl EAudioComponentPlayState {
    pub const PLAYING: EAudioComponentPlayState = EAudioComponentPlayState(0);
    pub const STOPPED: EAudioComponentPlayState = EAudioComponentPlayState(1);
    pub const PAUSED: EAudioComponentPlayState = EAudioComponentPlayState(2);
    pub const FADING_IN: EAudioComponentPlayState = EAudioComponentPlayState(3);
    pub const FADING_OUT: EAudioComponentPlayState = EAudioComponentPlayState(4);
    pub const COUNT: EAudioComponentPlayState = EAudioComponentPlayState(5);
}
#[repr(transparent)]
pub struct EPhysicsStateAction(pub u8);
impl EPhysicsStateAction {
    pub const ADD_FORCE: EPhysicsStateAction = EPhysicsStateAction(0);
    pub const ADD_TORQUE: EPhysicsStateAction = EPhysicsStateAction(1);
    pub const ADD_FORCE_AT_POSITION: EPhysicsStateAction = EPhysicsStateAction(2);
    pub const ADD_LINEAR_VELOCITY: EPhysicsStateAction = EPhysicsStateAction(3);
    pub const ADD_ANGULAR_VELOCITY: EPhysicsStateAction = EPhysicsStateAction(4);
    pub const ADD_VELOCITY_AT_POSITION: EPhysicsStateAction = EPhysicsStateAction(5);
    pub const ADD_LINEAR_IMPULSE: EPhysicsStateAction = EPhysicsStateAction(6);
    pub const ADD_ANGULAR_IMPULSE: EPhysicsStateAction = EPhysicsStateAction(7);
    pub const ADD_IMPULSE_AT_POSITION: EPhysicsStateAction = EPhysicsStateAction(8);
    pub const ADD_ACCELERATION: EPhysicsStateAction = EPhysicsStateAction(9);
    pub const NUM_ACTIONS: EPhysicsStateAction = EPhysicsStateAction(10);
}
#[repr(transparent)]
pub struct EControllerAnalogStick(pub u8);
impl EControllerAnalogStick {
    pub const CAS_LEFT_STICK: EControllerAnalogStick = EControllerAnalogStick(0);
    pub const CAS_RIGHT_STICK: EControllerAnalogStick = EControllerAnalogStick(1);
}
#[repr(transparent)]
pub struct ELightUnits(pub u8);
impl ELightUnits {
    pub const UNITLESS: ELightUnits = ELightUnits(0);
    pub const CANDELAS: ELightUnits = ELightUnits(1);
    pub const LUMENS: ELightUnits = ELightUnits(2);
    pub const EV: ELightUnits = ELightUnits(3);
    pub const NITS: ELightUnits = ELightUnits(4);
}
#[repr(transparent)]
pub struct EBoneSpaces(pub u8);
impl EBoneSpaces {
    pub const WORLD_SPACE: EBoneSpaces = EBoneSpaces(0);
    pub const COMPONENT_SPACE: EBoneSpaces = EBoneSpaces(1);
}
#[repr(transparent)]
pub struct ESplineCoordinateSpace(pub u8);
impl ESplineCoordinateSpace {
    pub const LOCAL: ESplineCoordinateSpace = ESplineCoordinateSpace(0);
    pub const WORLD: ESplineCoordinateSpace = ESplineCoordinateSpace(1);
}
#[repr(transparent)]
pub struct EHorizTextAligment(pub u8);
impl EHorizTextAligment {
    pub const EHTA_LEFT: EHorizTextAligment = EHorizTextAligment(0);
    pub const EHTA_CENTER: EHorizTextAligment = EHorizTextAligment(1);
    pub const EHTA_RIGHT: EHorizTextAligment = EHorizTextAligment(2);
}
#[repr(transparent)]
pub struct EVerticalTextAligment(pub u8);
impl EVerticalTextAligment {
    pub const EVRTA_TEXT_TOP: EVerticalTextAligment = EVerticalTextAligment(0);
    pub const EVRTA_TEXT_CENTER: EVerticalTextAligment = EVerticalTextAligment(1);
    pub const EVRTA_TEXT_BOTTOM: EVerticalTextAligment = EVerticalTextAligment(2);
    pub const EVRTA_QUAD_TOP: EVerticalTextAligment = EVerticalTextAligment(3);
}
#[repr(transparent)]
pub struct EEvaluateCurveTableResult(pub u8);
impl EEvaluateCurveTableResult {
    pub const ROW_FOUND: EEvaluateCurveTableResult = EEvaluateCurveTableResult(0);
    pub const ROW_NOT_FOUND: EEvaluateCurveTableResult = EEvaluateCurveTableResult(1);
}
#[repr(transparent)]
pub struct ETravelType(pub u8);
impl ETravelType {
    pub const TRAVEL_ABSOLUTE: ETravelType = ETravelType(0);
    pub const TRAVEL_PARTIAL: ETravelType = ETravelType(1);
    pub const TRAVEL_RELATIVE: ETravelType = ETravelType(2);
}
#[repr(transparent)]
pub struct EStreamingSourcePriority(pub u8);
impl EStreamingSourcePriority {
    pub const HIGHEST: EStreamingSourcePriority = EStreamingSourcePriority(0);
    pub const HIGH: EStreamingSourcePriority = EStreamingSourcePriority(64);
    pub const NORMAL: EStreamingSourcePriority = EStreamingSourcePriority(128);
    pub const LOW: EStreamingSourcePriority = EStreamingSourcePriority(192);
    pub const LOWEST: EStreamingSourcePriority = EStreamingSourcePriority(255);
    pub const DEFAULT: EStreamingSourcePriority = EStreamingSourcePriority(128);
}
#[repr(transparent)]
pub struct EDynamicForceFeedbackAction(pub u8);
impl EDynamicForceFeedbackAction {
    pub const START: EDynamicForceFeedbackAction = EDynamicForceFeedbackAction(0);
    pub const UPDATE: EDynamicForceFeedbackAction = EDynamicForceFeedbackAction(1);
    pub const STOP: EDynamicForceFeedbackAction = EDynamicForceFeedbackAction(2);
}
#[repr(transparent)]
pub struct ETypedElementWorldType(pub u8);
impl ETypedElementWorldType {
    pub const GAME: ETypedElementWorldType = ETypedElementWorldType(0);
    pub const EDITOR: ETypedElementWorldType = ETypedElementWorldType(1);
}
#[repr(transparent)]
pub struct ESpawnActorCollisionHandlingMethod(pub u8);
impl ESpawnActorCollisionHandlingMethod {
    pub const UNDEFINED: ESpawnActorCollisionHandlingMethod = ESpawnActorCollisionHandlingMethod(
        0,
    );
    pub const ALWAYS_SPAWN: ESpawnActorCollisionHandlingMethod = ESpawnActorCollisionHandlingMethod(
        1,
    );
    pub const ADJUST_IF_POSSIBLE_BUT_ALWAYS_SPAWN: ESpawnActorCollisionHandlingMethod = ESpawnActorCollisionHandlingMethod(
        2,
    );
    pub const ADJUST_IF_POSSIBLE_BUT_DONT_SPAWN_IF_COLLIDING: ESpawnActorCollisionHandlingMethod = ESpawnActorCollisionHandlingMethod(
        3,
    );
    pub const DONT_SPAWN_IF_COLLIDING: ESpawnActorCollisionHandlingMethod = ESpawnActorCollisionHandlingMethod(
        4,
    );
}
#[repr(transparent)]
pub struct ESpawnActorScaleMethod(pub u8);
impl ESpawnActorScaleMethod {
    pub const OVERRIDE_ROOT_SCALE: ESpawnActorScaleMethod = ESpawnActorScaleMethod(0);
    pub const MULTIPLY_WITH_ROOT: ESpawnActorScaleMethod = ESpawnActorScaleMethod(1);
    pub const SELECT_DEFAULT_AT_RUNTIME: ESpawnActorScaleMethod = ESpawnActorScaleMethod(
        2,
    );
}
#[repr(transparent)]
pub struct ESuggestProjVelocityTraceOption(pub u8);
impl ESuggestProjVelocityTraceOption {
    pub const DO_NOT_TRACE: ESuggestProjVelocityTraceOption = ESuggestProjVelocityTraceOption(
        0,
    );
    pub const TRACE_FULL_PATH: ESuggestProjVelocityTraceOption = ESuggestProjVelocityTraceOption(
        1,
    );
    pub const ONLY_TRACE_WHILE_ASCENDING: ESuggestProjVelocityTraceOption = ESuggestProjVelocityTraceOption(
        2,
    );
}
#[repr(transparent)]
pub struct EMouseCaptureMode(pub u8);
impl EMouseCaptureMode {
    pub const NO_CAPTURE: EMouseCaptureMode = EMouseCaptureMode(0);
    pub const CAPTURE_PERMANENTLY: EMouseCaptureMode = EMouseCaptureMode(1);
    pub const CAPTURE_PERMANENTLY_INCLUDING_INITIAL_MOUSE_DOWN: EMouseCaptureMode = EMouseCaptureMode(
        2,
    );
    pub const CAPTURE_DURING_MOUSE_DOWN: EMouseCaptureMode = EMouseCaptureMode(3);
    pub const CAPTURE_DURING_RIGHT_MOUSE_DOWN: EMouseCaptureMode = EMouseCaptureMode(4);
}
#[repr(transparent)]
pub struct EWindowMode(pub u8);
impl EWindowMode {
    pub const FULLSCREEN: EWindowMode = EWindowMode(0);
    pub const WINDOWED_FULLSCREEN: EWindowMode = EWindowMode(1);
    pub const WINDOWED: EWindowMode = EWindowMode(2);
}
#[repr(transparent)]
pub struct EArraySortOrder(pub u8);
impl EArraySortOrder {
    pub const ASCENDING: EArraySortOrder = EArraySortOrder(0);
    pub const DESCENDING: EArraySortOrder = EArraySortOrder(1);
}
#[repr(transparent)]
pub struct ESlateGesture(pub u8);
impl ESlateGesture {
    pub const NONE: ESlateGesture = ESlateGesture(0);
    pub const SCROLL: ESlateGesture = ESlateGesture(1);
    pub const MAGNIFY: ESlateGesture = ESlateGesture(2);
    pub const SWIPE: ESlateGesture = ESlateGesture(3);
    pub const ROTATE: ESlateGesture = ESlateGesture(4);
    pub const LONG_PRESS: ESlateGesture = ESlateGesture(5);
}
#[repr(transparent)]
pub struct EMIDCreationFlags(pub u8);
impl EMIDCreationFlags {
    pub const NONE: EMIDCreationFlags = EMIDCreationFlags(0);
    pub const TRANSIENT: EMIDCreationFlags = EMIDCreationFlags(1);
}
#[repr(transparent)]
pub struct EEasingFunc(pub u8);
impl EEasingFunc {
    pub const LINEAR: EEasingFunc = EEasingFunc(0);
    pub const STEP: EEasingFunc = EEasingFunc(1);
    pub const SINUSOIDAL_IN: EEasingFunc = EEasingFunc(2);
    pub const SINUSOIDAL_OUT: EEasingFunc = EEasingFunc(3);
    pub const SINUSOIDAL_IN_OUT: EEasingFunc = EEasingFunc(4);
    pub const EASE_IN: EEasingFunc = EEasingFunc(5);
    pub const EASE_OUT: EEasingFunc = EEasingFunc(6);
    pub const EASE_IN_OUT: EEasingFunc = EEasingFunc(7);
    pub const EXPO_IN: EEasingFunc = EEasingFunc(8);
    pub const EXPO_OUT: EEasingFunc = EEasingFunc(9);
    pub const EXPO_IN_OUT: EEasingFunc = EEasingFunc(10);
    pub const CIRCULAR_IN: EEasingFunc = EEasingFunc(11);
    pub const CIRCULAR_OUT: EEasingFunc = EEasingFunc(12);
    pub const CIRCULAR_IN_OUT: EEasingFunc = EEasingFunc(13);
}
#[repr(transparent)]
pub struct EMatrixColumns(pub u8);
impl EMatrixColumns {
    pub const FIRST: EMatrixColumns = EMatrixColumns(0);
    pub const SECOND: EMatrixColumns = EMatrixColumns(1);
    pub const THIRD: EMatrixColumns = EMatrixColumns(2);
    pub const FOURTH: EMatrixColumns = EMatrixColumns(3);
}
#[repr(transparent)]
pub struct ELerpInterpolationMode(pub u8);
impl ELerpInterpolationMode {
    pub const QUAT_INTERP: ELerpInterpolationMode = ELerpInterpolationMode(0);
    pub const EULER_INTERP: ELerpInterpolationMode = ELerpInterpolationMode(1);
    pub const DUAL_QUAT_INTERP: ELerpInterpolationMode = ELerpInterpolationMode(2);
}
#[repr(transparent)]
pub struct ETextureRenderTargetFormat(pub u8);
impl ETextureRenderTargetFormat {
    pub const RTF_R8: ETextureRenderTargetFormat = ETextureRenderTargetFormat(0);
    pub const RTF_RG8: ETextureRenderTargetFormat = ETextureRenderTargetFormat(1);
    pub const RTF_RGBA8: ETextureRenderTargetFormat = ETextureRenderTargetFormat(2);
    pub const RTF_RGBA8_SRGB: ETextureRenderTargetFormat = ETextureRenderTargetFormat(3);
    pub const RTF_R16F: ETextureRenderTargetFormat = ETextureRenderTargetFormat(4);
    pub const RTF_RG16F: ETextureRenderTargetFormat = ETextureRenderTargetFormat(5);
    pub const RTF_RGBA16F: ETextureRenderTargetFormat = ETextureRenderTargetFormat(6);
    pub const RTF_R32F: ETextureRenderTargetFormat = ETextureRenderTargetFormat(7);
    pub const RTF_RG32F: ETextureRenderTargetFormat = ETextureRenderTargetFormat(8);
    pub const RTF_RGBA32F: ETextureRenderTargetFormat = ETextureRenderTargetFormat(9);
    pub const RTF_RGB10A2: ETextureRenderTargetFormat = ETextureRenderTargetFormat(10);
}
#[repr(transparent)]
pub struct ERoundingMode(pub u8);
impl ERoundingMode {
    pub const HALF_TO_EVEN: ERoundingMode = ERoundingMode(0);
    pub const HALF_FROM_ZERO: ERoundingMode = ERoundingMode(1);
    pub const HALF_TO_ZERO: ERoundingMode = ERoundingMode(2);
    pub const FROM_ZERO: ERoundingMode = ERoundingMode(3);
    pub const TO_ZERO: ERoundingMode = ERoundingMode(4);
    pub const TO_NEGATIVE_INFINITY: ERoundingMode = ERoundingMode(5);
    pub const TO_POSITIVE_INFINITY: ERoundingMode = ERoundingMode(6);
}
#[repr(transparent)]
pub struct EDateTimeStyle(pub u8);
impl EDateTimeStyle {
    pub const DEFAULT: EDateTimeStyle = EDateTimeStyle(0);
    pub const SHORT: EDateTimeStyle = EDateTimeStyle(1);
    pub const MEDIUM: EDateTimeStyle = EDateTimeStyle(2);
    pub const LONG: EDateTimeStyle = EDateTimeStyle(3);
    pub const FULL: EDateTimeStyle = EDateTimeStyle(4);
    pub const CUSTOM: EDateTimeStyle = EDateTimeStyle(5);
}
#[repr(transparent)]
pub struct EMemoryUnitStandard(pub u8);
impl EMemoryUnitStandard {
    pub const IEC: EMemoryUnitStandard = EMemoryUnitStandard(0);
    pub const SI: EMemoryUnitStandard = EMemoryUnitStandard(1);
}
#[repr(transparent)]
pub struct EScreenOrientation(pub u8);
impl EScreenOrientation {
    pub const UNKNOWN: EScreenOrientation = EScreenOrientation(0);
    pub const PORTRAIT: EScreenOrientation = EScreenOrientation(1);
    pub const PORTRAIT_UPSIDE_DOWN: EScreenOrientation = EScreenOrientation(2);
    pub const LANDSCAPE_LEFT: EScreenOrientation = EScreenOrientation(3);
    pub const LANDSCAPE_RIGHT: EScreenOrientation = EScreenOrientation(4);
    pub const FACE_UP: EScreenOrientation = EScreenOrientation(5);
    pub const FACE_DOWN: EScreenOrientation = EScreenOrientation(6);
    pub const PORTRAIT_SENSOR: EScreenOrientation = EScreenOrientation(7);
    pub const LANDSCAPE_SENSOR: EScreenOrientation = EScreenOrientation(8);
    pub const FULL_SENSOR: EScreenOrientation = EScreenOrientation(9);
}
#[repr(transparent)]
pub struct EVertexPaintAxis(pub u8);
impl EVertexPaintAxis {
    pub const X: EVertexPaintAxis = EVertexPaintAxis(0);
    pub const Y: EVertexPaintAxis = EVertexPaintAxis(1);
    pub const Z: EVertexPaintAxis = EVertexPaintAxis(2);
}
#[repr(transparent)]
pub struct ETrailWidthMode(pub u8);
impl ETrailWidthMode {
    pub const E_TRAIL_WIDTH_MODE_FROM_CENTRE: ETrailWidthMode = ETrailWidthMode(0);
    pub const E_TRAIL_WIDTH_MODE_FROM_FIRST: ETrailWidthMode = ETrailWidthMode(1);
    pub const E_TRAIL_WIDTH_MODE_FROM_SECOND: ETrailWidthMode = ETrailWidthMode(2);
}
#[repr(transparent)]
pub struct EDataLayerState(pub u8);
impl EDataLayerState {
    pub const UNLOADED: EDataLayerState = EDataLayerState(0);
    pub const LOADED: EDataLayerState = EDataLayerState(1);
    pub const ACTIVATED: EDataLayerState = EDataLayerState(2);
}
#[repr(transparent)]
pub struct EDataLayerType(pub u8);
impl EDataLayerType {
    pub const RUNTIME: EDataLayerType = EDataLayerType(0);
    pub const EDITOR: EDataLayerType = EDataLayerType(1);
    pub const UNKNOWN: EDataLayerType = EDataLayerType(2);
    pub const SIZE: EDataLayerType = EDataLayerType(3);
}
#[repr(transparent)]
pub struct EWorldPartitionRuntimeCellState(pub u8);
impl EWorldPartitionRuntimeCellState {
    pub const UNLOADED: EWorldPartitionRuntimeCellState = EWorldPartitionRuntimeCellState(
        0,
    );
    pub const LOADED: EWorldPartitionRuntimeCellState = EWorldPartitionRuntimeCellState(
        1,
    );
    pub const ACTIVATED: EWorldPartitionRuntimeCellState = EWorldPartitionRuntimeCellState(
        2,
    );
}
#[repr(transparent)]
pub struct ESceneCaptureSource(pub u8);
impl ESceneCaptureSource {
    pub const SCS_SCENE_COLOR_HDR: ESceneCaptureSource = ESceneCaptureSource(0);
    pub const SCS_SCENE_COLOR_HDR_NO_ALPHA: ESceneCaptureSource = ESceneCaptureSource(1);
    pub const SCS_FINAL_COLOR_LDR: ESceneCaptureSource = ESceneCaptureSource(2);
    pub const SCS_SCENE_COLOR_SCENE_DEPTH: ESceneCaptureSource = ESceneCaptureSource(3);
    pub const SCS_SCENE_DEPTH: ESceneCaptureSource = ESceneCaptureSource(4);
    pub const SCS_DEVICE_DEPTH: ESceneCaptureSource = ESceneCaptureSource(5);
    pub const SCS_NORMAL: ESceneCaptureSource = ESceneCaptureSource(6);
    pub const SCS_BASE_COLOR: ESceneCaptureSource = ESceneCaptureSource(7);
    pub const SCS_FINAL_COLOR_HDR: ESceneCaptureSource = ESceneCaptureSource(8);
    pub const SCS_FINAL_TONE_CURVE_HDR: ESceneCaptureSource = ESceneCaptureSource(9);
}
#[repr(transparent)]
pub struct ERefractionMode(pub u8);
impl ERefractionMode {
    pub const RM_INDEX_OF_REFRACTION: ERefractionMode = ERefractionMode(0);
    pub const RM_PIXEL_NORMAL_OFFSET: ERefractionMode = ERefractionMode(1);
    pub const RM_2D_OFFSET: ERefractionMode = ERefractionMode(2);
    pub const RM_NONE: ERefractionMode = ERefractionMode(3);
    pub const RM_INDEX_OF_REFRACTION_FROM_F0: ERefractionMode = ERefractionMode(4);
}
#[repr(transparent)]
pub struct ETranslucencyLightingMode(pub u8);
impl ETranslucencyLightingMode {
    pub const TLM_VOLUMETRIC_NON_DIRECTIONAL: ETranslucencyLightingMode = ETranslucencyLightingMode(
        0,
    );
    pub const TLM_VOLUMETRIC_DIRECTIONAL: ETranslucencyLightingMode = ETranslucencyLightingMode(
        1,
    );
    pub const TLM_VOLUMETRIC_PER_VERTEX_NON_DIRECTIONAL: ETranslucencyLightingMode = ETranslucencyLightingMode(
        2,
    );
    pub const TLM_VOLUMETRIC_PER_VERTEX_DIRECTIONAL: ETranslucencyLightingMode = ETranslucencyLightingMode(
        3,
    );
    pub const TLM_SURFACE: ETranslucencyLightingMode = ETranslucencyLightingMode(4);
    pub const TLM_SURFACE_PER_PIXEL_LIGHTING: ETranslucencyLightingMode = ETranslucencyLightingMode(
        5,
    );
}
#[repr(transparent)]
pub struct ENodeAdvancedPins(pub u8);
impl ENodeAdvancedPins {
    pub const NO_PINS: ENodeAdvancedPins = ENodeAdvancedPins(0);
    pub const SHOWN: ENodeAdvancedPins = ENodeAdvancedPins(1);
    pub const HIDDEN: ENodeAdvancedPins = ENodeAdvancedPins(2);
}
#[repr(transparent)]
pub struct ENodeEnabledState(pub u8);
impl ENodeEnabledState {
    pub const ENABLED: ENodeEnabledState = ENodeEnabledState(0);
    pub const DISABLED: ENodeEnabledState = ENodeEnabledState(1);
    pub const DEVELOPMENT_ONLY: ENodeEnabledState = ENodeEnabledState(2);
}
#[repr(transparent)]
pub struct ELevelInstanceType(pub u8);
impl ELevelInstanceType {
    pub const NONE: ELevelInstanceType = ELevelInstanceType(0);
    pub const LEVEL_INSTANCE: ELevelInstanceType = ELevelInstanceType(1);
    pub const LEVEL_INSTANCE_EDIT: ELevelInstanceType = ELevelInstanceType(2);
    pub const LEVEL_INSTANCE_PROPERTY_OVERRIDE: ELevelInstanceType = ELevelInstanceType(
        3,
    );
}
#[repr(transparent)]
pub struct ELevelInstanceFlags(pub u8);
impl ELevelInstanceFlags {
    pub const NONE: ELevelInstanceFlags = ELevelInstanceFlags(0);
    pub const IS_IN_EDIT_HIERARCHY: ELevelInstanceFlags = ELevelInstanceFlags(1);
    pub const HAS_PROPERTY_OVERRIDES: ELevelInstanceFlags = ELevelInstanceFlags(2);
    pub const HAS_EDITABLE_PROPERTY_OVERRIDES: ELevelInstanceFlags = ELevelInstanceFlags(
        4,
    );
}
#[repr(transparent)]
pub struct EActorUpdateOverlapsMethod(pub u8);
impl EActorUpdateOverlapsMethod {
    pub const USE_CONFIG_DEFAULT: EActorUpdateOverlapsMethod = EActorUpdateOverlapsMethod(
        0,
    );
    pub const ALWAYS_UPDATE: EActorUpdateOverlapsMethod = EActorUpdateOverlapsMethod(1);
    pub const ONLY_UPDATE_MOVABLE: EActorUpdateOverlapsMethod = EActorUpdateOverlapsMethod(
        2,
    );
    pub const NEVER_UPDATE: EActorUpdateOverlapsMethod = EActorUpdateOverlapsMethod(3);
}
#[repr(transparent)]
pub struct EActorGridPlacement(pub u8);
impl EActorGridPlacement {
    pub const BOUNDS: EActorGridPlacement = EActorGridPlacement(0);
    pub const LOCATION: EActorGridPlacement = EActorGridPlacement(1);
    pub const ALWAYS_LOADED: EActorGridPlacement = EActorGridPlacement(2);
    pub const NONE: EActorGridPlacement = EActorGridPlacement(3);
}
#[repr(transparent)]
pub struct EAutoReceiveInput(pub u8);
impl EAutoReceiveInput {
    pub const DISABLED: EAutoReceiveInput = EAutoReceiveInput(0);
    pub const PLAYER0: EAutoReceiveInput = EAutoReceiveInput(1);
    pub const PLAYER1: EAutoReceiveInput = EAutoReceiveInput(2);
    pub const PLAYER2: EAutoReceiveInput = EAutoReceiveInput(3);
    pub const PLAYER3: EAutoReceiveInput = EAutoReceiveInput(4);
    pub const PLAYER4: EAutoReceiveInput = EAutoReceiveInput(5);
    pub const PLAYER5: EAutoReceiveInput = EAutoReceiveInput(6);
    pub const PLAYER6: EAutoReceiveInput = EAutoReceiveInput(7);
    pub const PLAYER7: EAutoReceiveInput = EAutoReceiveInput(8);
}
#[repr(transparent)]
pub struct EIndirectLightingCacheQuality(pub u8);
impl EIndirectLightingCacheQuality {
    pub const ILCQ_OFF: EIndirectLightingCacheQuality = EIndirectLightingCacheQuality(0);
    pub const ILCQ_POINT: EIndirectLightingCacheQuality = EIndirectLightingCacheQuality(
        1,
    );
    pub const ILCQ_VOLUME: EIndirectLightingCacheQuality = EIndirectLightingCacheQuality(
        2,
    );
}
#[repr(transparent)]
pub struct EHitProxyPriority(pub u8);
impl EHitProxyPriority {
    pub const HPP_WORLD: EHitProxyPriority = EHitProxyPriority(0);
    pub const HPP_WIREFRAME: EHitProxyPriority = EHitProxyPriority(1);
    pub const HPP_FOREGROUND: EHitProxyPriority = EHitProxyPriority(2);
    pub const HPP_UI: EHitProxyPriority = EHitProxyPriority(3);
}
#[repr(transparent)]
pub struct ECanBeCharacterBase(pub u8);
impl ECanBeCharacterBase {
    pub const ECB_NO: ECanBeCharacterBase = ECanBeCharacterBase(0);
    pub const ECB_YES: ECanBeCharacterBase = ECanBeCharacterBase(1);
    pub const ECB_OWNER: ECanBeCharacterBase = ECanBeCharacterBase(2);
}
#[repr(transparent)]
pub struct EAutoPossessAI(pub u8);
impl EAutoPossessAI {
    pub const DISABLED: EAutoPossessAI = EAutoPossessAI(0);
    pub const PLACED_IN_WORLD: EAutoPossessAI = EAutoPossessAI(1);
    pub const SPAWNED: EAutoPossessAI = EAutoPossessAI(2);
    pub const PLACED_IN_WORLD_OR_SPAWNED: EAutoPossessAI = EAutoPossessAI(3);
}
#[repr(transparent)]
pub struct ENavigationInvokerPriority(pub u8);
impl ENavigationInvokerPriority {
    pub const VERY_LOW: ENavigationInvokerPriority = ENavigationInvokerPriority(1);
    pub const LOW: ENavigationInvokerPriority = ENavigationInvokerPriority(2);
    pub const DEFAULT: ENavigationInvokerPriority = ENavigationInvokerPriority(3);
    pub const HIGH: ENavigationInvokerPriority = ENavigationInvokerPriority(4);
    pub const VERY_HIGH: ENavigationInvokerPriority = ENavigationInvokerPriority(5);
    pub const MAX: ENavigationInvokerPriority = ENavigationInvokerPriority(6);
}
#[repr(transparent)]
pub struct ENavPathEvent(pub u8);
impl ENavPathEvent {
    pub const CLEARED: ENavPathEvent = ENavPathEvent(0);
    pub const NEW_PATH: ENavPathEvent = ENavPathEvent(1);
    pub const UPDATED_DUE_TO_GOAL_MOVED: ENavPathEvent = ENavPathEvent(2);
    pub const UPDATED_DUE_TO_NAVIGATION_CHANGED: ENavPathEvent = ENavPathEvent(3);
    pub const INVALIDATED: ENavPathEvent = ENavPathEvent(4);
    pub const RE_PATH_FAILED: ENavPathEvent = ENavPathEvent(5);
    pub const META_PATH_UPDATE: ENavPathEvent = ENavPathEvent(6);
    pub const CUSTOM: ENavPathEvent = ENavPathEvent(7);
}
#[repr(transparent)]
pub struct FNavigationSystemRunMode(pub u8);
impl FNavigationSystemRunMode {
    pub const INVALID_MODE: FNavigationSystemRunMode = FNavigationSystemRunMode(0);
    pub const GAME_MODE: FNavigationSystemRunMode = FNavigationSystemRunMode(1);
    pub const EDITOR_MODE: FNavigationSystemRunMode = FNavigationSystemRunMode(2);
    pub const SIMULATION_MODE: FNavigationSystemRunMode = FNavigationSystemRunMode(3);
    pub const PIE_MODE: FNavigationSystemRunMode = FNavigationSystemRunMode(4);
    pub const INFER_FROM_WORLD_MODE: FNavigationSystemRunMode = FNavigationSystemRunMode(
        5,
    );
    pub const EDITOR_WORLD_PARTITION_BUILD_MODE: FNavigationSystemRunMode = FNavigationSystemRunMode(
        6,
    );
}
#[repr(transparent)]
pub struct EBrushType(pub u8);
impl EBrushType {
    pub const BRUSH_DEFAULT: EBrushType = EBrushType(0);
    pub const BRUSH_ADD: EBrushType = EBrushType(1);
    pub const BRUSH_SUBTRACT: EBrushType = EBrushType(2);
}
#[repr(transparent)]
pub struct ENavigationDataResolution(pub u8);
impl ENavigationDataResolution {
    pub const LOW: ENavigationDataResolution = ENavigationDataResolution(0);
    pub const DEFAULT: ENavigationDataResolution = ENavigationDataResolution(1);
    pub const HIGH: ENavigationDataResolution = ENavigationDataResolution(2);
    pub const INVALID: ENavigationDataResolution = ENavigationDataResolution(3);
    pub const MAX: ENavigationDataResolution = ENavigationDataResolution(3);
}
#[repr(transparent)]
pub struct EBlueprintType(pub u8);
impl EBlueprintType {
    pub const BPTYPE_NORMAL: EBlueprintType = EBlueprintType(0);
    pub const BPTYPE_CONST: EBlueprintType = EBlueprintType(1);
    pub const BPTYPE_MACRO_LIBRARY: EBlueprintType = EBlueprintType(2);
    pub const BPTYPE_INTERFACE: EBlueprintType = EBlueprintType(3);
    pub const BPTYPE_LEVEL_SCRIPT: EBlueprintType = EBlueprintType(4);
    pub const BPTYPE_FUNCTION_LIBRARY: EBlueprintType = EBlueprintType(5);
}
#[repr(transparent)]
pub struct EShouldCookBlueprintPropertyGuids(pub i32);
impl EShouldCookBlueprintPropertyGuids {
    pub const NO: EShouldCookBlueprintPropertyGuids = EShouldCookBlueprintPropertyGuids(
        0,
    );
    pub const YES: EShouldCookBlueprintPropertyGuids = EShouldCookBlueprintPropertyGuids(
        1,
    );
    pub const INHERIT: EShouldCookBlueprintPropertyGuids = EShouldCookBlueprintPropertyGuids(
        2,
    );
}
#[repr(transparent)]
pub struct EBlueprintCompileMode(pub u8);
impl EBlueprintCompileMode {
    pub const DEFAULT: EBlueprintCompileMode = EBlueprintCompileMode(0);
    pub const DEVELOPMENT: EBlueprintCompileMode = EBlueprintCompileMode(1);
    pub const FINAL_RELEASE: EBlueprintCompileMode = EBlueprintCompileMode(2);
}
#[repr(transparent)]
pub struct EBlueprintStatus(pub u8);
impl EBlueprintStatus {
    pub const BS_UNKNOWN: EBlueprintStatus = EBlueprintStatus(0);
    pub const BS_DIRTY: EBlueprintStatus = EBlueprintStatus(1);
    pub const BS_ERROR: EBlueprintStatus = EBlueprintStatus(2);
    pub const BS_UP_TO_DATE: EBlueprintStatus = EBlueprintStatus(3);
    pub const BS_BEING_CREATED: EBlueprintStatus = EBlueprintStatus(4);
    pub const BS_UP_TO_DATE_WITH_WARNINGS: EBlueprintStatus = EBlueprintStatus(5);
}
#[repr(transparent)]
pub struct EBlueprintPinStyleType(pub u8);
impl EBlueprintPinStyleType {
    pub const BPST_ORIGINAL: EBlueprintPinStyleType = EBlueprintPinStyleType(0);
    pub const BPST_VARIANT_A: EBlueprintPinStyleType = EBlueprintPinStyleType(1);
}
#[repr(transparent)]
pub struct EVisibilityBasedAnimTickOption(pub u8);
impl EVisibilityBasedAnimTickOption {
    pub const ALWAYS_TICK_POSE_AND_REFRESH_BONES: EVisibilityBasedAnimTickOption = EVisibilityBasedAnimTickOption(
        0,
    );
    pub const ALWAYS_TICK_POSE: EVisibilityBasedAnimTickOption = EVisibilityBasedAnimTickOption(
        1,
    );
    pub const ONLY_TICK_MONTAGES_AND_REFRESH_BONES_WHEN_PLAYING_MONTAGES: EVisibilityBasedAnimTickOption = EVisibilityBasedAnimTickOption(
        2,
    );
    pub const ONLY_TICK_MONTAGES_WHEN_NOT_RENDERED: EVisibilityBasedAnimTickOption = EVisibilityBasedAnimTickOption(
        3,
    );
    pub const ONLY_TICK_POSE_WHEN_RENDERED: EVisibilityBasedAnimTickOption = EVisibilityBasedAnimTickOption(
        4,
    );
}
#[repr(transparent)]
pub struct EKinematicBonesUpdateToPhysics(pub u8);
impl EKinematicBonesUpdateToPhysics {
    pub const SKIP_SIMULATING_BONES: EKinematicBonesUpdateToPhysics = EKinematicBonesUpdateToPhysics(
        0,
    );
    pub const SKIP_ALL_BONES: EKinematicBonesUpdateToPhysics = EKinematicBonesUpdateToPhysics(
        1,
    );
}
#[repr(transparent)]
pub struct EPhysicsTransformUpdateMode(pub u8);
impl EPhysicsTransformUpdateMode {
    pub const SIMULATION_UPATES_COMPONENT_TRANSFORM: EPhysicsTransformUpdateMode = EPhysicsTransformUpdateMode(
        0,
    );
    pub const COMPONENT_TRANSFORM_IS_KINEMATIC: EPhysicsTransformUpdateMode = EPhysicsTransformUpdateMode(
        1,
    );
}
#[repr(transparent)]
pub struct ETransitionBlendMode(pub u8);
impl ETransitionBlendMode {
    pub const TBM_LINEAR: ETransitionBlendMode = ETransitionBlendMode(0);
    pub const TBM_CUBIC: ETransitionBlendMode = ETransitionBlendMode(1);
}
#[repr(transparent)]
pub struct EVirtualizationMode(pub u8);
impl EVirtualizationMode {
    pub const DISABLED: EVirtualizationMode = EVirtualizationMode(0);
    pub const PLAY_WHEN_SILENT: EVirtualizationMode = EVirtualizationMode(1);
    pub const RESTART: EVirtualizationMode = EVirtualizationMode(2);
    pub const SEEK_RESTART: EVirtualizationMode = EVirtualizationMode(3);
}
#[repr(transparent)]
pub struct ESoundWaveFFTSize(pub u8);
impl ESoundWaveFFTSize {
    pub const VERY_SMALL_64: ESoundWaveFFTSize = ESoundWaveFFTSize(0);
    pub const SMALL_256: ESoundWaveFFTSize = ESoundWaveFFTSize(1);
    pub const MEDIUM_512: ESoundWaveFFTSize = ESoundWaveFFTSize(2);
    pub const LARGE_1024: ESoundWaveFFTSize = ESoundWaveFFTSize(3);
    pub const VERY_LARGE_2048: ESoundWaveFFTSize = ESoundWaveFFTSize(4);
}
#[repr(transparent)]
pub struct ESoundWaveCuePointOrigin(pub u8);
impl ESoundWaveCuePointOrigin {
    pub const WAVE_FILE: ESoundWaveCuePointOrigin = ESoundWaveCuePointOrigin(0);
    pub const MARKER_TRANSFORMATION: ESoundWaveCuePointOrigin = ESoundWaveCuePointOrigin(
        1,
    );
}
#[repr(transparent)]
pub struct ETextureCompressionQuality(pub u8);
impl ETextureCompressionQuality {
    pub const TCQ_DEFAULT: ETextureCompressionQuality = ETextureCompressionQuality(0);
    pub const TCQ_LOWEST: ETextureCompressionQuality = ETextureCompressionQuality(1);
    pub const TCQ_LOW: ETextureCompressionQuality = ETextureCompressionQuality(2);
    pub const TCQ_MEDIUM: ETextureCompressionQuality = ETextureCompressionQuality(3);
    pub const TCQ_HIGH: ETextureCompressionQuality = ETextureCompressionQuality(4);
    pub const TCQ_HIGHEST: ETextureCompressionQuality = ETextureCompressionQuality(5);
}
#[repr(transparent)]
pub struct ETexturePowerOfTwoSetting(pub u8);
impl ETexturePowerOfTwoSetting {
    pub const NONE: ETexturePowerOfTwoSetting = ETexturePowerOfTwoSetting(0);
    pub const PAD_TO_POWER_OF_TWO: ETexturePowerOfTwoSetting = ETexturePowerOfTwoSetting(
        1,
    );
    pub const PAD_TO_SQUARE_POWER_OF_TWO: ETexturePowerOfTwoSetting = ETexturePowerOfTwoSetting(
        2,
    );
    pub const STRETCH_TO_POWER_OF_TWO: ETexturePowerOfTwoSetting = ETexturePowerOfTwoSetting(
        3,
    );
    pub const STRETCH_TO_SQUARE_POWER_OF_TWO: ETexturePowerOfTwoSetting = ETexturePowerOfTwoSetting(
        4,
    );
    pub const RESIZE_TO_SPECIFIC_RESOLUTION: ETexturePowerOfTwoSetting = ETexturePowerOfTwoSetting(
        5,
    );
}
#[repr(transparent)]
pub struct ECompositeTextureMode(pub u8);
impl ECompositeTextureMode {
    pub const CTM_DISABLED: ECompositeTextureMode = ECompositeTextureMode(0);
    pub const CTM_NORMAL_ROUGHNESS_TO_RED: ECompositeTextureMode = ECompositeTextureMode(
        1,
    );
    pub const CTM_NORMAL_ROUGHNESS_TO_GREEN: ECompositeTextureMode = ECompositeTextureMode(
        2,
    );
    pub const CTM_NORMAL_ROUGHNESS_TO_BLUE: ECompositeTextureMode = ECompositeTextureMode(
        3,
    );
    pub const CTM_NORMAL_ROUGHNESS_TO_ALPHA: ECompositeTextureMode = ECompositeTextureMode(
        4,
    );
}
#[repr(transparent)]
pub struct TextureCookPlatformTilingSettings(pub u8);
impl TextureCookPlatformTilingSettings {
    pub const TCPTS_FROM_TEXTURE_GROUP: TextureCookPlatformTilingSettings = TextureCookPlatformTilingSettings(
        0,
    );
    pub const TCPTS_TILE: TextureCookPlatformTilingSettings = TextureCookPlatformTilingSettings(
        1,
    );
    pub const TCPTS_DO_NOT_TILE: TextureCookPlatformTilingSettings = TextureCookPlatformTilingSettings(
        2,
    );
}
#[repr(transparent)]
pub struct ETextureAvailability(pub u8);
impl ETextureAvailability {
    pub const GPU: ETextureAvailability = ETextureAvailability(0);
    pub const CPU: ETextureAvailability = ETextureAvailability(1);
}
#[repr(transparent)]
pub struct ETransitionType(pub u8);
impl ETransitionType {
    pub const NONE: ETransitionType = ETransitionType(0);
    pub const PAUSED: ETransitionType = ETransitionType(1);
    pub const LOADING: ETransitionType = ETransitionType(2);
    pub const SAVING: ETransitionType = ETransitionType(3);
    pub const CONNECTING: ETransitionType = ETransitionType(4);
    pub const PRECACHING: ETransitionType = ETransitionType(5);
    pub const WAITING_TO_CONNECT: ETransitionType = ETransitionType(6);
    pub const MAX: ETransitionType = ETransitionType(7);
}
#[repr(transparent)]
pub struct EDecalBlendMode(pub u8);
impl EDecalBlendMode {
    pub const DBM_TRANSLUCENT: EDecalBlendMode = EDecalBlendMode(0);
    pub const DBM_STAIN: EDecalBlendMode = EDecalBlendMode(1);
    pub const DBM_NORMAL: EDecalBlendMode = EDecalBlendMode(2);
    pub const DBM_EMISSIVE: EDecalBlendMode = EDecalBlendMode(3);
    pub const DBM_D_BUFFER_COLOR_NORMAL_ROUGHNESS: EDecalBlendMode = EDecalBlendMode(4);
    pub const DBM_D_BUFFER_COLOR: EDecalBlendMode = EDecalBlendMode(5);
    pub const DBM_D_BUFFER_COLOR_NORMAL: EDecalBlendMode = EDecalBlendMode(6);
    pub const DBM_D_BUFFER_COLOR_ROUGHNESS: EDecalBlendMode = EDecalBlendMode(7);
    pub const DBM_D_BUFFER_NORMAL: EDecalBlendMode = EDecalBlendMode(8);
    pub const DBM_D_BUFFER_NORMAL_ROUGHNESS: EDecalBlendMode = EDecalBlendMode(9);
    pub const DBM_D_BUFFER_ROUGHNESS: EDecalBlendMode = EDecalBlendMode(10);
    pub const DBM_D_BUFFER_EMISSIVE: EDecalBlendMode = EDecalBlendMode(11);
    pub const DBM_D_BUFFER_ALPHA_COMPOSITE: EDecalBlendMode = EDecalBlendMode(12);
    pub const DBM_D_BUFFER_EMISSIVE_ALPHA_COMPOSITE: EDecalBlendMode = EDecalBlendMode(
        13,
    );
    pub const DBM_VOLUMETRIC_DISTANCE_FUNCTION: EDecalBlendMode = EDecalBlendMode(14);
    pub const DBM_ALPHA_COMPOSITE: EDecalBlendMode = EDecalBlendMode(15);
    pub const DBM_AMBIENT_OCCLUSION: EDecalBlendMode = EDecalBlendMode(16);
}
#[repr(transparent)]
pub struct EMaterialDecalResponse(pub u8);
impl EMaterialDecalResponse {
    pub const MDR_NONE: EMaterialDecalResponse = EMaterialDecalResponse(0);
    pub const MDR_COLOR_NORMAL_ROUGHNESS: EMaterialDecalResponse = EMaterialDecalResponse(
        1,
    );
    pub const MDR_COLOR: EMaterialDecalResponse = EMaterialDecalResponse(2);
    pub const MDR_COLOR_NORMAL: EMaterialDecalResponse = EMaterialDecalResponse(3);
    pub const MDR_COLOR_ROUGHNESS: EMaterialDecalResponse = EMaterialDecalResponse(4);
    pub const MDR_NORMAL: EMaterialDecalResponse = EMaterialDecalResponse(5);
    pub const MDR_NORMAL_ROUGHNESS: EMaterialDecalResponse = EMaterialDecalResponse(6);
    pub const MDR_ROUGHNESS: EMaterialDecalResponse = EMaterialDecalResponse(7);
}
#[repr(transparent)]
pub struct EMaterialTranslucencyPass(pub u8);
impl EMaterialTranslucencyPass {
    pub const MTP_BEFORE_DOF: EMaterialTranslucencyPass = EMaterialTranslucencyPass(0);
    pub const MTP_AFTER_DOF: EMaterialTranslucencyPass = EMaterialTranslucencyPass(1);
    pub const MTP_AFTER_MOTION_BLUR: EMaterialTranslucencyPass = EMaterialTranslucencyPass(
        2,
    );
}
#[repr(transparent)]
pub struct EMaterialFloatPrecisionMode(pub u8);
impl EMaterialFloatPrecisionMode {
    pub const MFPM_DEFAULT: EMaterialFloatPrecisionMode = EMaterialFloatPrecisionMode(0);
    pub const MFPM_FULL_MATERIAL_EXPRESSION_ONLY: EMaterialFloatPrecisionMode = EMaterialFloatPrecisionMode(
        1,
    );
    pub const MFPM_FULL: EMaterialFloatPrecisionMode = EMaterialFloatPrecisionMode(2);
    pub const MFPM_HALF: EMaterialFloatPrecisionMode = EMaterialFloatPrecisionMode(3);
}
#[repr(transparent)]
pub struct EMaterialShadingRate(pub u8);
impl EMaterialShadingRate {
    pub const MSR_1X1: EMaterialShadingRate = EMaterialShadingRate(0);
    pub const MSR_2X1: EMaterialShadingRate = EMaterialShadingRate(1);
    pub const MSR_1X2: EMaterialShadingRate = EMaterialShadingRate(2);
    pub const MSR_2X2: EMaterialShadingRate = EMaterialShadingRate(3);
    pub const MSR_4X2: EMaterialShadingRate = EMaterialShadingRate(4);
    pub const MSR_2X4: EMaterialShadingRate = EMaterialShadingRate(5);
    pub const MSR_4X4: EMaterialShadingRate = EMaterialShadingRate(6);
    pub const MSR_COUNT: EMaterialShadingRate = EMaterialShadingRate(7);
}
#[repr(transparent)]
pub struct EMaterialStencilCompare(pub u8);
impl EMaterialStencilCompare {
    pub const MSC_LESS: EMaterialStencilCompare = EMaterialStencilCompare(0);
    pub const MSC_LESS_EQUAL: EMaterialStencilCompare = EMaterialStencilCompare(1);
    pub const MSC_GREATER: EMaterialStencilCompare = EMaterialStencilCompare(2);
    pub const MSC_GREATER_EQUAL: EMaterialStencilCompare = EMaterialStencilCompare(3);
    pub const MSC_EQUAL: EMaterialStencilCompare = EMaterialStencilCompare(4);
    pub const MSC_NOT_EQUAL: EMaterialStencilCompare = EMaterialStencilCompare(5);
    pub const MSC_NEVER: EMaterialStencilCompare = EMaterialStencilCompare(6);
    pub const MSC_ALWAYS: EMaterialStencilCompare = EMaterialStencilCompare(7);
    pub const MSC_COUNT: EMaterialStencilCompare = EMaterialStencilCompare(8);
}
#[repr(transparent)]
pub struct ERefractionCoverageMode(pub u8);
impl ERefractionCoverageMode {
    pub const RCM_COVERAGE_IGNORED: ERefractionCoverageMode = ERefractionCoverageMode(0);
    pub const RCM_COVERAGE_ACCOUNTED_FOR: ERefractionCoverageMode = ERefractionCoverageMode(
        1,
    );
}
#[repr(transparent)]
pub struct EPixelDepthOffsetMode(pub u8);
impl EPixelDepthOffsetMode {
    pub const PDOM_LEGACY: EPixelDepthOffsetMode = EPixelDepthOffsetMode(0);
    pub const PDOM_ALONG_CAMERA_VECTOR: EPixelDepthOffsetMode = EPixelDepthOffsetMode(1);
}
#[repr(transparent)]
pub struct EChildActorComponentTreeViewVisualizationMode(pub u8);
impl EChildActorComponentTreeViewVisualizationMode {
    pub const USE_DEFAULT: EChildActorComponentTreeViewVisualizationMode = EChildActorComponentTreeViewVisualizationMode(
        0,
    );
    pub const COMPONENT_ONLY: EChildActorComponentTreeViewVisualizationMode = EChildActorComponentTreeViewVisualizationMode(
        1,
    );
    pub const COMPONENT_WITH_CHILD_ACTOR: EChildActorComponentTreeViewVisualizationMode = EChildActorComponentTreeViewVisualizationMode(
        2,
    );
    pub const CHILD_ACTOR_ONLY: EChildActorComponentTreeViewVisualizationMode = EChildActorComponentTreeViewVisualizationMode(
        3,
    );
    pub const HIDDEN: EChildActorComponentTreeViewVisualizationMode = EChildActorComponentTreeViewVisualizationMode(
        4,
    );
}
#[repr(transparent)]
pub struct EScreenPercentageMode(pub i32);
impl EScreenPercentageMode {
    pub const MANUAL: EScreenPercentageMode = EScreenPercentageMode(0);
    pub const BASED_ON_DISPLAY_RESOLUTION: EScreenPercentageMode = EScreenPercentageMode(
        1,
    );
    pub const BASED_ON_DPI_SCALE: EScreenPercentageMode = EScreenPercentageMode(2);
}
#[repr(transparent)]
pub struct ENavDataGatheringMode(pub u8);
impl ENavDataGatheringMode {
    pub const DEFAULT: ENavDataGatheringMode = ENavDataGatheringMode(0);
    pub const INSTANT: ENavDataGatheringMode = ENavDataGatheringMode(1);
    pub const LAZY: ENavDataGatheringMode = ENavDataGatheringMode(2);
}
#[repr(transparent)]
pub struct EActorPackagingScheme(pub u8);
impl EActorPackagingScheme {
    pub const ORIGINAL: EActorPackagingScheme = EActorPackagingScheme(0);
    pub const REDUCED: EActorPackagingScheme = EActorPackagingScheme(1);
}
#[repr(transparent)]
pub struct EReflectionSourceType(pub u8);
impl EReflectionSourceType {
    pub const CAPTURED_SCENE: EReflectionSourceType = EReflectionSourceType(0);
    pub const SPECIFIED_CUBEMAP: EReflectionSourceType = EReflectionSourceType(1);
}
#[repr(transparent)]
pub struct DistributionParamMode(pub u8);
impl DistributionParamMode {
    pub const DPM_NORMAL: DistributionParamMode = DistributionParamMode(0);
    pub const DPM_ABS: DistributionParamMode = DistributionParamMode(1);
    pub const DPM_DIRECT: DistributionParamMode = DistributionParamMode(2);
}
#[repr(transparent)]
pub struct EDistributionVectorLockFlags(pub u8);
impl EDistributionVectorLockFlags {
    pub const EDVLF_NONE: EDistributionVectorLockFlags = EDistributionVectorLockFlags(0);
    pub const EDVLF_XY: EDistributionVectorLockFlags = EDistributionVectorLockFlags(1);
    pub const EDVLF_XZ: EDistributionVectorLockFlags = EDistributionVectorLockFlags(2);
    pub const EDVLF_YZ: EDistributionVectorLockFlags = EDistributionVectorLockFlags(3);
    pub const EDVLF_XYZ: EDistributionVectorLockFlags = EDistributionVectorLockFlags(4);
}
#[repr(transparent)]
pub struct EDistributionVectorMirrorFlags(pub u8);
impl EDistributionVectorMirrorFlags {
    pub const EDVMF_SAME: EDistributionVectorMirrorFlags = EDistributionVectorMirrorFlags(
        0,
    );
    pub const EDVMF_DIFFERENT: EDistributionVectorMirrorFlags = EDistributionVectorMirrorFlags(
        1,
    );
    pub const EDVMF_MIRROR: EDistributionVectorMirrorFlags = EDistributionVectorMirrorFlags(
        2,
    );
}
#[repr(transparent)]
pub struct EPositionOrigin(pub u8);
impl EPositionOrigin {
    pub const ABSOLUTE: EPositionOrigin = EPositionOrigin(0);
    pub const CAMERA_RELATIVE: EPositionOrigin = EPositionOrigin(1);
}
#[repr(transparent)]
pub struct EMaterialExpressionMakeAggregateKind(pub i32);
impl EMaterialExpressionMakeAggregateKind {
    pub const MATERIAL_ATTRIBUTES: EMaterialExpressionMakeAggregateKind = EMaterialExpressionMakeAggregateKind(
        0,
    );
    pub const USER_DEFINED: EMaterialExpressionMakeAggregateKind = EMaterialExpressionMakeAggregateKind(
        1,
    );
}
#[repr(transparent)]
pub struct EMaterialSamplerType(pub u8);
impl EMaterialSamplerType {
    pub const SAMPLERTYPE_COLOR: EMaterialSamplerType = EMaterialSamplerType(0);
    pub const SAMPLERTYPE_GRAYSCALE: EMaterialSamplerType = EMaterialSamplerType(1);
    pub const SAMPLERTYPE_ALPHA: EMaterialSamplerType = EMaterialSamplerType(2);
    pub const SAMPLERTYPE_NORMAL: EMaterialSamplerType = EMaterialSamplerType(3);
    pub const SAMPLERTYPE_MASKS: EMaterialSamplerType = EMaterialSamplerType(4);
    pub const SAMPLERTYPE_DISTANCE_FIELD_FONT: EMaterialSamplerType = EMaterialSamplerType(
        5,
    );
    pub const SAMPLERTYPE_LINEAR_COLOR: EMaterialSamplerType = EMaterialSamplerType(6);
    pub const SAMPLERTYPE_LINEAR_GRAYSCALE: EMaterialSamplerType = EMaterialSamplerType(
        7,
    );
    pub const SAMPLERTYPE_DATA: EMaterialSamplerType = EMaterialSamplerType(8);
    pub const SAMPLERTYPE_EXTERNAL: EMaterialSamplerType = EMaterialSamplerType(9);
    pub const SAMPLERTYPE_VIRTUAL_COLOR: EMaterialSamplerType = EMaterialSamplerType(10);
    pub const SAMPLERTYPE_VIRTUAL_GRAYSCALE: EMaterialSamplerType = EMaterialSamplerType(
        11,
    );
    pub const SAMPLERTYPE_VIRTUAL_ALPHA: EMaterialSamplerType = EMaterialSamplerType(12);
    pub const SAMPLERTYPE_VIRTUAL_NORMAL: EMaterialSamplerType = EMaterialSamplerType(
        13,
    );
    pub const SAMPLERTYPE_VIRTUAL_MASKS: EMaterialSamplerType = EMaterialSamplerType(14);
    pub const SAMPLERTYPE_VIRTUAL_LINEAR_COLOR: EMaterialSamplerType = EMaterialSamplerType(
        15,
    );
    pub const SAMPLERTYPE_VIRTUAL_LINEAR_GRAYSCALE: EMaterialSamplerType = EMaterialSamplerType(
        16,
    );
}
#[repr(transparent)]
pub struct ETextureMipValueMode(pub u8);
impl ETextureMipValueMode {
    pub const TMVM_NONE: ETextureMipValueMode = ETextureMipValueMode(0);
    pub const TMVM_MIP_LEVEL: ETextureMipValueMode = ETextureMipValueMode(1);
    pub const TMVM_MIP_BIAS: ETextureMipValueMode = ETextureMipValueMode(2);
    pub const TMVM_DERIVATIVE: ETextureMipValueMode = ETextureMipValueMode(3);
}
#[repr(transparent)]
pub struct ESamplerSourceMode(pub u8);
impl ESamplerSourceMode {
    pub const SSM_FROM_TEXTURE_ASSET: ESamplerSourceMode = ESamplerSourceMode(0);
    pub const SSM_WRAP_WORLD_GROUP_SETTINGS: ESamplerSourceMode = ESamplerSourceMode(1);
    pub const SSM_CLAMP_WORLD_GROUP_SETTINGS: ESamplerSourceMode = ESamplerSourceMode(2);
    pub const SSM_TERRAIN_WEIGHTMAP_GROUP_SETTINGS: ESamplerSourceMode = ESamplerSourceMode(
        3,
    );
}
#[repr(transparent)]
pub struct ETextureGatherMode(pub u8);
impl ETextureGatherMode {
    pub const TGM_NONE: ETextureGatherMode = ETextureGatherMode(0);
    pub const TGM_RED: ETextureGatherMode = ETextureGatherMode(1);
    pub const TGM_GREEN: ETextureGatherMode = ETextureGatherMode(2);
    pub const TGM_BLUE: ETextureGatherMode = ETextureGatherMode(3);
    pub const TGM_ALPHA: ETextureGatherMode = ETextureGatherMode(4);
}
#[repr(transparent)]
pub struct ETextureColorChannel(pub u8);
impl ETextureColorChannel {
    pub const TCC_RED: ETextureColorChannel = ETextureColorChannel(0);
    pub const TCC_GREEN: ETextureColorChannel = ETextureColorChannel(1);
    pub const TCC_BLUE: ETextureColorChannel = ETextureColorChannel(2);
    pub const TCC_ALPHA: ETextureColorChannel = ETextureColorChannel(3);
}
#[repr(transparent)]
pub struct EMaterialExpressionBlendMode(pub u8);
impl EMaterialExpressionBlendMode {
    pub const BLEND: EMaterialExpressionBlendMode = EMaterialExpressionBlendMode(0);
    pub const USE_A: EMaterialExpressionBlendMode = EMaterialExpressionBlendMode(1);
    pub const USE_B: EMaterialExpressionBlendMode = EMaterialExpressionBlendMode(2);
}
#[repr(transparent)]
pub struct EMaterialAttributeBlend(pub u8);
impl EMaterialAttributeBlend {
    pub const BLEND: EMaterialAttributeBlend = EMaterialAttributeBlend(0);
    pub const USE_A: EMaterialAttributeBlend = EMaterialAttributeBlend(1);
    pub const USE_B: EMaterialAttributeBlend = EMaterialAttributeBlend(2);
}
#[repr(transparent)]
pub struct EMaterialAttributeBlendFunction(pub u8);
impl EMaterialAttributeBlendFunction {
    pub const HORIZONTAL: EMaterialAttributeBlendFunction = EMaterialAttributeBlendFunction(
        0,
    );
    pub const VERTICAL: EMaterialAttributeBlendFunction = EMaterialAttributeBlendFunction(
        1,
    );
}
#[repr(transparent)]
pub struct EMaterialExpressionBoundsType(pub u8);
impl EMaterialExpressionBoundsType {
    pub const MEILB_INSTANCE_LOCAL: EMaterialExpressionBoundsType = EMaterialExpressionBoundsType(
        0,
    );
    pub const MEILB_OBJECT_LOCAL: EMaterialExpressionBoundsType = EMaterialExpressionBoundsType(
        1,
    );
    pub const MEILB_PRE_SKINNED_LOCAL: EMaterialExpressionBoundsType = EMaterialExpressionBoundsType(
        2,
    );
}
#[repr(transparent)]
pub struct EChannelMaskParameterColor(pub u8);
impl EChannelMaskParameterColor {
    pub const RED: EChannelMaskParameterColor = EChannelMaskParameterColor(0);
    pub const GREEN: EChannelMaskParameterColor = EChannelMaskParameterColor(1);
    pub const BLUE: EChannelMaskParameterColor = EChannelMaskParameterColor(2);
    pub const ALPHA: EChannelMaskParameterColor = EChannelMaskParameterColor(3);
}
#[repr(transparent)]
pub struct EClampMode(pub u8);
impl EClampMode {
    pub const CMODE_CLAMP: EClampMode = EClampMode(0);
    pub const CMODE_CLAMP_MIN: EClampMode = EClampMode(1);
    pub const CMODE_CLAMP_MAX: EClampMode = EClampMode(2);
}
#[repr(transparent)]
pub struct EParameterCollectionTransformType(pub u8);
impl EParameterCollectionTransformType {
    pub const POSITION: EParameterCollectionTransformType = EParameterCollectionTransformType(
        0,
    );
    pub const VECTOR: EParameterCollectionTransformType = EParameterCollectionTransformType(
        1,
    );
    pub const PROJECTION: EParameterCollectionTransformType = EParameterCollectionTransformType(
        2,
    );
    pub const LOCAL_TO_WORLD: EParameterCollectionTransformType = EParameterCollectionTransformType(
        3,
    );
    pub const WORLD_TO_LOCAL: EParameterCollectionTransformType = EParameterCollectionTransformType(
        4,
    );
}
#[repr(transparent)]
pub struct EMaterialScalarParameterControlType(pub i32);
impl EMaterialScalarParameterControlType {
    pub const NUMERIC: EMaterialScalarParameterControlType = EMaterialScalarParameterControlType(
        0,
    );
    pub const ENUMERATION: EMaterialScalarParameterControlType = EMaterialScalarParameterControlType(
        1,
    );
    pub const ENUMERATION_INDEX: EMaterialScalarParameterControlType = EMaterialScalarParameterControlType(
        2,
    );
}
#[repr(transparent)]
pub struct EDBufferTextureId(pub u8);
impl EDBufferTextureId {
    pub const DBT_A: EDBufferTextureId = EDBufferTextureId(0);
    pub const DBT_B: EDBufferTextureId = EDBufferTextureId(1);
    pub const DBT_C: EDBufferTextureId = EDBufferTextureId(2);
}
#[repr(transparent)]
pub struct EDepthOfFieldFunctionValue(pub u8);
impl EDepthOfFieldFunctionValue {
    pub const TDOF_NEAR_AND_FAR_MASK: EDepthOfFieldFunctionValue = EDepthOfFieldFunctionValue(
        0,
    );
    pub const TDOF_NEAR_MASK: EDepthOfFieldFunctionValue = EDepthOfFieldFunctionValue(1);
    pub const TDOF_FAR_MASK: EDepthOfFieldFunctionValue = EDepthOfFieldFunctionValue(2);
    pub const TDOF_CIRCLE_OF_CONFUSION_RADIUS: EDepthOfFieldFunctionValue = EDepthOfFieldFunctionValue(
        3,
    );
}
#[repr(transparent)]
pub struct EFloatToIntMode(pub u8);
impl EFloatToIntMode {
    pub const TRUNCATE: EFloatToIntMode = EFloatToIntMode(0);
    pub const FLOOR: EFloatToIntMode = EFloatToIntMode(1);
    pub const ROUND: EFloatToIntMode = EFloatToIntMode(2);
    pub const CEIL: EFloatToIntMode = EFloatToIntMode(3);
}
#[repr(transparent)]
pub struct EBlendInputRelevance(pub u8);
impl EBlendInputRelevance {
    pub const GENERAL: EBlendInputRelevance = EBlendInputRelevance(0);
    pub const TOP: EBlendInputRelevance = EBlendInputRelevance(1);
    pub const BOTTOM: EBlendInputRelevance = EBlendInputRelevance(2);
}
#[repr(transparent)]
pub struct EPositionIncludedOffsets(pub i32);
impl EPositionIncludedOffsets {
    pub const INCLUDE_OFFSETS: EPositionIncludedOffsets = EPositionIncludedOffsets(0);
    pub const EXCLUDE_OFFSETS: EPositionIncludedOffsets = EPositionIncludedOffsets(1);
}
#[repr(transparent)]
pub struct ELocalPositionOrigin(pub i32);
impl ELocalPositionOrigin {
    pub const INSTANCE: ELocalPositionOrigin = ELocalPositionOrigin(0);
    pub const INSTANCE_PRE_SKINNING: ELocalPositionOrigin = ELocalPositionOrigin(1);
    pub const PRIMITIVE: ELocalPositionOrigin = ELocalPositionOrigin(2);
}
#[repr(transparent)]
pub struct ENeuralIndexType(pub u8);
impl ENeuralIndexType {
    pub const NIT_TEXTURE_INDEX: ENeuralIndexType = ENeuralIndexType(0);
    pub const NIT_BUFFER_INDEX: ENeuralIndexType = ENeuralIndexType(1);
}
#[repr(transparent)]
pub struct EMaterialExpressionOperatorKind(pub i32);
impl EMaterialExpressionOperatorKind {
    pub const BITWISE_NOT: EMaterialExpressionOperatorKind = EMaterialExpressionOperatorKind(
        0,
    );
    pub const NEGATE: EMaterialExpressionOperatorKind = EMaterialExpressionOperatorKind(
        1,
    );
    pub const NOT: EMaterialExpressionOperatorKind = EMaterialExpressionOperatorKind(2);
    pub const ABS: EMaterialExpressionOperatorKind = EMaterialExpressionOperatorKind(3);
    pub const A_COS: EMaterialExpressionOperatorKind = EMaterialExpressionOperatorKind(
        4,
    );
    pub const A_COS_FAST: EMaterialExpressionOperatorKind = EMaterialExpressionOperatorKind(
        5,
    );
    pub const A_COSH: EMaterialExpressionOperatorKind = EMaterialExpressionOperatorKind(
        6,
    );
    pub const A_SIN: EMaterialExpressionOperatorKind = EMaterialExpressionOperatorKind(
        7,
    );
    pub const A_SIN_FAST: EMaterialExpressionOperatorKind = EMaterialExpressionOperatorKind(
        8,
    );
    pub const A_SINH: EMaterialExpressionOperatorKind = EMaterialExpressionOperatorKind(
        9,
    );
    pub const A_TAN: EMaterialExpressionOperatorKind = EMaterialExpressionOperatorKind(
        10,
    );
    pub const A_TAN_FAST: EMaterialExpressionOperatorKind = EMaterialExpressionOperatorKind(
        11,
    );
    pub const A_TANH: EMaterialExpressionOperatorKind = EMaterialExpressionOperatorKind(
        12,
    );
    pub const CEIL: EMaterialExpressionOperatorKind = EMaterialExpressionOperatorKind(
        13,
    );
    pub const COS: EMaterialExpressionOperatorKind = EMaterialExpressionOperatorKind(14);
    pub const COSH: EMaterialExpressionOperatorKind = EMaterialExpressionOperatorKind(
        15,
    );
    pub const EXPONENTIAL: EMaterialExpressionOperatorKind = EMaterialExpressionOperatorKind(
        16,
    );
    pub const EXPONENTIAL2: EMaterialExpressionOperatorKind = EMaterialExpressionOperatorKind(
        17,
    );
    pub const FLOOR: EMaterialExpressionOperatorKind = EMaterialExpressionOperatorKind(
        18,
    );
    pub const FRAC: EMaterialExpressionOperatorKind = EMaterialExpressionOperatorKind(
        19,
    );
    pub const IS_FINITE: EMaterialExpressionOperatorKind = EMaterialExpressionOperatorKind(
        20,
    );
    pub const IS_INF: EMaterialExpressionOperatorKind = EMaterialExpressionOperatorKind(
        21,
    );
    pub const IS_NAN: EMaterialExpressionOperatorKind = EMaterialExpressionOperatorKind(
        22,
    );
    pub const LENGTH: EMaterialExpressionOperatorKind = EMaterialExpressionOperatorKind(
        23,
    );
    pub const LOGARITHM: EMaterialExpressionOperatorKind = EMaterialExpressionOperatorKind(
        24,
    );
    pub const LOGARITHM10: EMaterialExpressionOperatorKind = EMaterialExpressionOperatorKind(
        25,
    );
    pub const LOGARITHM2: EMaterialExpressionOperatorKind = EMaterialExpressionOperatorKind(
        26,
    );
    pub const LWC_TILE: EMaterialExpressionOperatorKind = EMaterialExpressionOperatorKind(
        27,
    );
    pub const RECIPROCAL: EMaterialExpressionOperatorKind = EMaterialExpressionOperatorKind(
        28,
    );
    pub const ROUND: EMaterialExpressionOperatorKind = EMaterialExpressionOperatorKind(
        29,
    );
    pub const RSQRT: EMaterialExpressionOperatorKind = EMaterialExpressionOperatorKind(
        30,
    );
    pub const SATURATE: EMaterialExpressionOperatorKind = EMaterialExpressionOperatorKind(
        31,
    );
    pub const SIGN: EMaterialExpressionOperatorKind = EMaterialExpressionOperatorKind(
        32,
    );
    pub const SIN: EMaterialExpressionOperatorKind = EMaterialExpressionOperatorKind(33);
    pub const SINH: EMaterialExpressionOperatorKind = EMaterialExpressionOperatorKind(
        34,
    );
    pub const SQRT: EMaterialExpressionOperatorKind = EMaterialExpressionOperatorKind(
        35,
    );
    pub const TAN: EMaterialExpressionOperatorKind = EMaterialExpressionOperatorKind(36);
    pub const TANH: EMaterialExpressionOperatorKind = EMaterialExpressionOperatorKind(
        37,
    );
    pub const TRANSPOSE: EMaterialExpressionOperatorKind = EMaterialExpressionOperatorKind(
        38,
    );
    pub const TRUNCATE: EMaterialExpressionOperatorKind = EMaterialExpressionOperatorKind(
        39,
    );
    pub const EQUALS: EMaterialExpressionOperatorKind = EMaterialExpressionOperatorKind(
        40,
    );
    pub const GREATER_THAN: EMaterialExpressionOperatorKind = EMaterialExpressionOperatorKind(
        41,
    );
    pub const GREATER_THAN_OR_EQUALS: EMaterialExpressionOperatorKind = EMaterialExpressionOperatorKind(
        42,
    );
    pub const LESS_THAN: EMaterialExpressionOperatorKind = EMaterialExpressionOperatorKind(
        43,
    );
    pub const LESS_THAN_OR_EQUALS: EMaterialExpressionOperatorKind = EMaterialExpressionOperatorKind(
        44,
    );
    pub const NOT_EQUALS: EMaterialExpressionOperatorKind = EMaterialExpressionOperatorKind(
        45,
    );
    pub const AND: EMaterialExpressionOperatorKind = EMaterialExpressionOperatorKind(46);
    pub const OR: EMaterialExpressionOperatorKind = EMaterialExpressionOperatorKind(47);
    pub const ADD: EMaterialExpressionOperatorKind = EMaterialExpressionOperatorKind(48);
    pub const SUBTRACT: EMaterialExpressionOperatorKind = EMaterialExpressionOperatorKind(
        49,
    );
    pub const MULTIPLY: EMaterialExpressionOperatorKind = EMaterialExpressionOperatorKind(
        50,
    );
    pub const MATRIX_MULTIPLY: EMaterialExpressionOperatorKind = EMaterialExpressionOperatorKind(
        51,
    );
    pub const DIVIDE: EMaterialExpressionOperatorKind = EMaterialExpressionOperatorKind(
        52,
    );
    pub const MODULO: EMaterialExpressionOperatorKind = EMaterialExpressionOperatorKind(
        53,
    );
    pub const BITWISE_AND: EMaterialExpressionOperatorKind = EMaterialExpressionOperatorKind(
        54,
    );
    pub const BITWISE_OR: EMaterialExpressionOperatorKind = EMaterialExpressionOperatorKind(
        55,
    );
    pub const BIT_SHIFT_LEFT: EMaterialExpressionOperatorKind = EMaterialExpressionOperatorKind(
        56,
    );
    pub const BIT_SHIFT_RIGHT: EMaterialExpressionOperatorKind = EMaterialExpressionOperatorKind(
        57,
    );
    pub const A_TAN2: EMaterialExpressionOperatorKind = EMaterialExpressionOperatorKind(
        58,
    );
    pub const A_TAN2_FAST: EMaterialExpressionOperatorKind = EMaterialExpressionOperatorKind(
        59,
    );
    pub const CROSS: EMaterialExpressionOperatorKind = EMaterialExpressionOperatorKind(
        60,
    );
    pub const DISTANCE: EMaterialExpressionOperatorKind = EMaterialExpressionOperatorKind(
        61,
    );
    pub const DOT: EMaterialExpressionOperatorKind = EMaterialExpressionOperatorKind(62);
    pub const FMOD: EMaterialExpressionOperatorKind = EMaterialExpressionOperatorKind(
        63,
    );
    pub const MAX: EMaterialExpressionOperatorKind = EMaterialExpressionOperatorKind(64);
    pub const MIN: EMaterialExpressionOperatorKind = EMaterialExpressionOperatorKind(65);
    pub const POW: EMaterialExpressionOperatorKind = EMaterialExpressionOperatorKind(66);
    pub const STEP: EMaterialExpressionOperatorKind = EMaterialExpressionOperatorKind(
        67,
    );
    pub const CLAMP: EMaterialExpressionOperatorKind = EMaterialExpressionOperatorKind(
        68,
    );
    pub const LERP: EMaterialExpressionOperatorKind = EMaterialExpressionOperatorKind(
        69,
    );
    pub const SELECT: EMaterialExpressionOperatorKind = EMaterialExpressionOperatorKind(
        70,
    );
    pub const SMOOTHSTEP: EMaterialExpressionOperatorKind = EMaterialExpressionOperatorKind(
        71,
    );
}
#[repr(transparent)]
pub struct EPathTracingBufferTextureId(pub u8);
impl EPathTracingBufferTextureId {
    pub const PTBT_RADIANCE: EPathTracingBufferTextureId = EPathTracingBufferTextureId(
        0,
    );
    pub const PTBT_DENOISED_RADIANCE: EPathTracingBufferTextureId = EPathTracingBufferTextureId(
        1,
    );
    pub const PTBT_ALBEDO: EPathTracingBufferTextureId = EPathTracingBufferTextureId(2);
    pub const PTBT_NORMAL: EPathTracingBufferTextureId = EPathTracingBufferTextureId(3);
    pub const PTBT_VARIANCE: EPathTracingBufferTextureId = EPathTracingBufferTextureId(
        4,
    );
}
#[repr(transparent)]
pub struct ERuntimeVirtualTextureMaterialType(pub u8);
impl ERuntimeVirtualTextureMaterialType {
    pub const BASE_COLOR: ERuntimeVirtualTextureMaterialType = ERuntimeVirtualTextureMaterialType(
        0,
    );
    pub const MASK4: ERuntimeVirtualTextureMaterialType = ERuntimeVirtualTextureMaterialType(
        1,
    );
    pub const BASE_COLOR_NORMAL_ROUGHNESS: ERuntimeVirtualTextureMaterialType = ERuntimeVirtualTextureMaterialType(
        2,
    );
    pub const BASE_COLOR_NORMAL_SPECULAR: ERuntimeVirtualTextureMaterialType = ERuntimeVirtualTextureMaterialType(
        3,
    );
    pub const BASE_COLOR_NORMAL_SPECULAR_Y_CO_CG: ERuntimeVirtualTextureMaterialType = ERuntimeVirtualTextureMaterialType(
        4,
    );
    pub const BASE_COLOR_NORMAL_SPECULAR_MASK_Y_CO_CG: ERuntimeVirtualTextureMaterialType = ERuntimeVirtualTextureMaterialType(
        5,
    );
    pub const WORLD_HEIGHT: ERuntimeVirtualTextureMaterialType = ERuntimeVirtualTextureMaterialType(
        6,
    );
    pub const DISPLACEMENT: ERuntimeVirtualTextureMaterialType = ERuntimeVirtualTextureMaterialType(
        7,
    );
    pub const COUNT: ERuntimeVirtualTextureMaterialType = ERuntimeVirtualTextureMaterialType(
        8,
    );
}
#[repr(transparent)]
pub struct ERuntimeVirtualTextureTextureAddressMode(pub u8);
impl ERuntimeVirtualTextureTextureAddressMode {
    pub const RVTTA_CLAMP: ERuntimeVirtualTextureTextureAddressMode = ERuntimeVirtualTextureTextureAddressMode(
        0,
    );
    pub const RVTTA_WRAP: ERuntimeVirtualTextureTextureAddressMode = ERuntimeVirtualTextureTextureAddressMode(
        1,
    );
}
#[repr(transparent)]
pub struct ERuntimeVirtualTextureMipValueMode(pub u8);
impl ERuntimeVirtualTextureMipValueMode {
    pub const RVTMVM_NONE: ERuntimeVirtualTextureMipValueMode = ERuntimeVirtualTextureMipValueMode(
        0,
    );
    pub const RVTMVM_MIP_LEVEL: ERuntimeVirtualTextureMipValueMode = ERuntimeVirtualTextureMipValueMode(
        1,
    );
    pub const RVTMVM_MIP_BIAS: ERuntimeVirtualTextureMipValueMode = ERuntimeVirtualTextureMipValueMode(
        2,
    );
    pub const RVTMVM_RECALCULATE_DERIVATIVES: ERuntimeVirtualTextureMipValueMode = ERuntimeVirtualTextureMipValueMode(
        3,
    );
    pub const RVTMVM_DERIVATIVE_UV: ERuntimeVirtualTextureMipValueMode = ERuntimeVirtualTextureMipValueMode(
        4,
    );
    pub const RVTMVM_DERIVATIVE_WORLD: ERuntimeVirtualTextureMipValueMode = ERuntimeVirtualTextureMipValueMode(
        5,
    );
}
#[repr(transparent)]
pub struct EMaterialSceneAttributeInputMode(pub u8);
impl EMaterialSceneAttributeInputMode {
    pub const COORDINATES: EMaterialSceneAttributeInputMode = EMaterialSceneAttributeInputMode(
        0,
    );
    pub const OFFSET_FRACTION: EMaterialSceneAttributeInputMode = EMaterialSceneAttributeInputMode(
        1,
    );
}
#[repr(transparent)]
pub struct ESceneTextureId(pub u8);
impl ESceneTextureId {
    pub const PPI_SCENE_COLOR: ESceneTextureId = ESceneTextureId(0);
    pub const PPI_SCENE_DEPTH: ESceneTextureId = ESceneTextureId(1);
    pub const PPI_DIFFUSE_COLOR: ESceneTextureId = ESceneTextureId(2);
    pub const PPI_SPECULAR_COLOR: ESceneTextureId = ESceneTextureId(3);
    pub const PPI_SUBSURFACE_COLOR: ESceneTextureId = ESceneTextureId(4);
    pub const PPI_BASE_COLOR: ESceneTextureId = ESceneTextureId(5);
    pub const PPI_SPECULAR: ESceneTextureId = ESceneTextureId(6);
    pub const PPI_METALLIC: ESceneTextureId = ESceneTextureId(7);
    pub const PPI_WORLD_NORMAL: ESceneTextureId = ESceneTextureId(8);
    pub const PPI_SEPARATE_TRANSLUCENCY: ESceneTextureId = ESceneTextureId(9);
    pub const PPI_OPACITY: ESceneTextureId = ESceneTextureId(10);
    pub const PPI_ROUGHNESS: ESceneTextureId = ESceneTextureId(11);
    pub const PPI_MATERIAL_AO: ESceneTextureId = ESceneTextureId(12);
    pub const PPI_CUSTOM_DEPTH: ESceneTextureId = ESceneTextureId(13);
    pub const PPI_POST_PROCESS_INPUT0: ESceneTextureId = ESceneTextureId(14);
    pub const PPI_POST_PROCESS_INPUT1: ESceneTextureId = ESceneTextureId(15);
    pub const PPI_POST_PROCESS_INPUT2: ESceneTextureId = ESceneTextureId(16);
    pub const PPI_POST_PROCESS_INPUT3: ESceneTextureId = ESceneTextureId(17);
    pub const PPI_POST_PROCESS_INPUT4: ESceneTextureId = ESceneTextureId(18);
    pub const PPI_POST_PROCESS_INPUT5: ESceneTextureId = ESceneTextureId(19);
    pub const PPI_POST_PROCESS_INPUT6: ESceneTextureId = ESceneTextureId(20);
    pub const PPI_DECAL_MASK: ESceneTextureId = ESceneTextureId(21);
    pub const PPI_SHADING_MODEL_COLOR: ESceneTextureId = ESceneTextureId(22);
    pub const PPI_SHADING_MODEL_ID: ESceneTextureId = ESceneTextureId(23);
    pub const PPI_AMBIENT_OCCLUSION: ESceneTextureId = ESceneTextureId(24);
    pub const PPI_CUSTOM_STENCIL: ESceneTextureId = ESceneTextureId(25);
    pub const PPI_STORED_BASE_COLOR: ESceneTextureId = ESceneTextureId(26);
    pub const PPI_STORED_SPECULAR: ESceneTextureId = ESceneTextureId(27);
    pub const PPI_VELOCITY: ESceneTextureId = ESceneTextureId(28);
    pub const PPI_WORLD_TANGENT: ESceneTextureId = ESceneTextureId(29);
    pub const PPI_ANISOTROPY: ESceneTextureId = ESceneTextureId(30);
    pub const PPI_IS_FIRST_PERSON: ESceneTextureId = ESceneTextureId(31);
    pub const PPI_USER_SCENE_TEXTURE0: ESceneTextureId = ESceneTextureId(32);
    pub const PPI_USER_SCENE_TEXTURE1: ESceneTextureId = ESceneTextureId(33);
    pub const PPI_USER_SCENE_TEXTURE2: ESceneTextureId = ESceneTextureId(34);
    pub const PPI_USER_SCENE_TEXTURE3: ESceneTextureId = ESceneTextureId(35);
    pub const PPI_USER_SCENE_TEXTURE4: ESceneTextureId = ESceneTextureId(36);
    pub const PPI_USER_SCENE_TEXTURE5: ESceneTextureId = ESceneTextureId(37);
    pub const PPI_USER_SCENE_TEXTURE6: ESceneTextureId = ESceneTextureId(38);
}
#[repr(transparent)]
pub struct ESpeedTreeGeometryType(pub u8);
impl ESpeedTreeGeometryType {
    pub const STG_BRANCH: ESpeedTreeGeometryType = ESpeedTreeGeometryType(0);
    pub const STG_FROND: ESpeedTreeGeometryType = ESpeedTreeGeometryType(1);
    pub const STG_LEAF: ESpeedTreeGeometryType = ESpeedTreeGeometryType(2);
    pub const STG_FACING_LEAF: ESpeedTreeGeometryType = ESpeedTreeGeometryType(3);
    pub const STG_BILLBOARD: ESpeedTreeGeometryType = ESpeedTreeGeometryType(4);
}
#[repr(transparent)]
pub struct ESpeedTreeWindType(pub u8);
impl ESpeedTreeWindType {
    pub const STW_NONE: ESpeedTreeWindType = ESpeedTreeWindType(0);
    pub const STW_FASTEST: ESpeedTreeWindType = ESpeedTreeWindType(1);
    pub const STW_FAST: ESpeedTreeWindType = ESpeedTreeWindType(2);
    pub const STW_BETTER: ESpeedTreeWindType = ESpeedTreeWindType(3);
    pub const STW_BEST: ESpeedTreeWindType = ESpeedTreeWindType(4);
    pub const STW_PALM: ESpeedTreeWindType = ESpeedTreeWindType(5);
    pub const STW_BEST_PLUS: ESpeedTreeWindType = ESpeedTreeWindType(6);
}
#[repr(transparent)]
pub struct ESpeedTreeLODType(pub u8);
impl ESpeedTreeLODType {
    pub const STLOD_POP: ESpeedTreeLODType = ESpeedTreeLODType(0);
    pub const STLOD_SMOOTH: ESpeedTreeLODType = ESpeedTreeLODType(1);
}
#[repr(transparent)]
pub struct ETextureCollectionMemberType(pub u8);
impl ETextureCollectionMemberType {
    pub const TEXTURE2_D: ETextureCollectionMemberType = ETextureCollectionMemberType(0);
    pub const TEXTURE_CUBE: ETextureCollectionMemberType = ETextureCollectionMemberType(
        1,
    );
    pub const TEXTURE2_D_ARRAY: ETextureCollectionMemberType = ETextureCollectionMemberType(
        2,
    );
    pub const TEXTURE_CUBE_ARRAY: ETextureCollectionMemberType = ETextureCollectionMemberType(
        3,
    );
    pub const TEXTURE_VOLUME: ETextureCollectionMemberType = ETextureCollectionMemberType(
        4,
    );
    pub const MAX: ETextureCollectionMemberType = ETextureCollectionMemberType(5);
}
#[repr(transparent)]
pub struct EMaterialExposedTextureProperty(pub u8);
impl EMaterialExposedTextureProperty {
    pub const TMTM_TEXTURE_SIZE: EMaterialExposedTextureProperty = EMaterialExposedTextureProperty(
        0,
    );
    pub const TMTM_TEXEL_SIZE: EMaterialExposedTextureProperty = EMaterialExposedTextureProperty(
        1,
    );
}
#[repr(transparent)]
pub struct EMaterialVectorCoordTransformSource(pub u8);
impl EMaterialVectorCoordTransformSource {
    pub const TRANSFORMSOURCE_TANGENT: EMaterialVectorCoordTransformSource = EMaterialVectorCoordTransformSource(
        0,
    );
    pub const TRANSFORMSOURCE_LOCAL: EMaterialVectorCoordTransformSource = EMaterialVectorCoordTransformSource(
        1,
    );
    pub const TRANSFORMSOURCE_WORLD: EMaterialVectorCoordTransformSource = EMaterialVectorCoordTransformSource(
        2,
    );
    pub const TRANSFORMSOURCE_VIEW: EMaterialVectorCoordTransformSource = EMaterialVectorCoordTransformSource(
        3,
    );
    pub const TRANSFORMSOURCE_CAMERA: EMaterialVectorCoordTransformSource = EMaterialVectorCoordTransformSource(
        4,
    );
    pub const TRANSFORMSOURCE_PARTICLE_WORLD: EMaterialVectorCoordTransformSource = EMaterialVectorCoordTransformSource(
        5,
    );
    pub const TRANSFORMSOURCE_INSTANCE: EMaterialVectorCoordTransformSource = EMaterialVectorCoordTransformSource(
        6,
    );
}
#[repr(transparent)]
pub struct EMaterialVectorCoordTransform(pub u8);
impl EMaterialVectorCoordTransform {
    pub const TRANSFORM_TANGENT: EMaterialVectorCoordTransform = EMaterialVectorCoordTransform(
        0,
    );
    pub const TRANSFORM_LOCAL: EMaterialVectorCoordTransform = EMaterialVectorCoordTransform(
        1,
    );
    pub const TRANSFORM_WORLD: EMaterialVectorCoordTransform = EMaterialVectorCoordTransform(
        2,
    );
    pub const TRANSFORM_VIEW: EMaterialVectorCoordTransform = EMaterialVectorCoordTransform(
        3,
    );
    pub const TRANSFORM_CAMERA: EMaterialVectorCoordTransform = EMaterialVectorCoordTransform(
        4,
    );
    pub const TRANSFORM_PARTICLE_WORLD: EMaterialVectorCoordTransform = EMaterialVectorCoordTransform(
        5,
    );
    pub const TRANSFORM_INSTANCE: EMaterialVectorCoordTransform = EMaterialVectorCoordTransform(
        6,
    );
}
#[repr(transparent)]
pub struct EMaterialPositionTransformSource(pub u8);
impl EMaterialPositionTransformSource {
    pub const TRANSFORMPOSSOURCE_LOCAL: EMaterialPositionTransformSource = EMaterialPositionTransformSource(
        0,
    );
    pub const TRANSFORMPOSSOURCE_WORLD: EMaterialPositionTransformSource = EMaterialPositionTransformSource(
        1,
    );
    pub const TRANSFORMPOSSOURCE_PERIODIC_WORLD: EMaterialPositionTransformSource = EMaterialPositionTransformSource(
        2,
    );
    pub const TRANSFORMPOSSOURCE_TRANSLATED_WORLD: EMaterialPositionTransformSource = EMaterialPositionTransformSource(
        3,
    );
    pub const TRANSFORMPOSSOURCE_FIRST_PERSON_TRANSLATED_WORLD: EMaterialPositionTransformSource = EMaterialPositionTransformSource(
        4,
    );
    pub const TRANSFORMPOSSOURCE_VIEW: EMaterialPositionTransformSource = EMaterialPositionTransformSource(
        5,
    );
    pub const TRANSFORMPOSSOURCE_CAMERA: EMaterialPositionTransformSource = EMaterialPositionTransformSource(
        6,
    );
    pub const TRANSFORMPOSSOURCE_PARTICLE: EMaterialPositionTransformSource = EMaterialPositionTransformSource(
        7,
    );
    pub const TRANSFORMPOSSOURCE_INSTANCE: EMaterialPositionTransformSource = EMaterialPositionTransformSource(
        8,
    );
}
#[repr(transparent)]
pub struct EVectorNoiseFunction(pub u8);
impl EVectorNoiseFunction {
    pub const VNF_CELLNOISE_ALU: EVectorNoiseFunction = EVectorNoiseFunction(0);
    pub const VNF_VECTOR_ALU: EVectorNoiseFunction = EVectorNoiseFunction(1);
    pub const VNF_GRADIENT_ALU: EVectorNoiseFunction = EVectorNoiseFunction(2);
    pub const VNF_CURL_ALU: EVectorNoiseFunction = EVectorNoiseFunction(3);
    pub const VNF_VORONOI_ALU: EVectorNoiseFunction = EVectorNoiseFunction(4);
}
#[repr(transparent)]
pub struct EMaterialExposedViewProperty(pub u8);
impl EMaterialExposedViewProperty {
    pub const MEVP_BUFFER_SIZE: EMaterialExposedViewProperty = EMaterialExposedViewProperty(
        0,
    );
    pub const MEVP_FIELD_OF_VIEW: EMaterialExposedViewProperty = EMaterialExposedViewProperty(
        1,
    );
    pub const MEVP_TAN_HALF_FIELD_OF_VIEW: EMaterialExposedViewProperty = EMaterialExposedViewProperty(
        2,
    );
    pub const MEVP_VIEW_SIZE: EMaterialExposedViewProperty = EMaterialExposedViewProperty(
        3,
    );
    pub const MEVP_WORLD_SPACE_VIEW_POSITION: EMaterialExposedViewProperty = EMaterialExposedViewProperty(
        4,
    );
    pub const MEVP_WORLD_SPACE_CAMERA_POSITION: EMaterialExposedViewProperty = EMaterialExposedViewProperty(
        5,
    );
    pub const MEVP_VIEWPORT_OFFSET: EMaterialExposedViewProperty = EMaterialExposedViewProperty(
        6,
    );
    pub const MEVP_TEMPORAL_SAMPLE_COUNT: EMaterialExposedViewProperty = EMaterialExposedViewProperty(
        7,
    );
    pub const MEVP_TEMPORAL_SAMPLE_INDEX: EMaterialExposedViewProperty = EMaterialExposedViewProperty(
        8,
    );
    pub const MEVP_TEMPORAL_SAMPLE_OFFSET: EMaterialExposedViewProperty = EMaterialExposedViewProperty(
        9,
    );
    pub const MEVP_RUNTIME_VIRTUAL_TEXTURE_OUTPUT_LEVEL: EMaterialExposedViewProperty = EMaterialExposedViewProperty(
        10,
    );
    pub const MEVP_RUNTIME_VIRTUAL_TEXTURE_OUTPUT_DERIVATIVE: EMaterialExposedViewProperty = EMaterialExposedViewProperty(
        11,
    );
    pub const MEVP_PRE_EXPOSURE: EMaterialExposedViewProperty = EMaterialExposedViewProperty(
        12,
    );
    pub const MEVP_RUNTIME_VIRTUAL_TEXTURE_MAX_LEVEL: EMaterialExposedViewProperty = EMaterialExposedViewProperty(
        13,
    );
    pub const MEVP_RESOLUTION_FRACTION: EMaterialExposedViewProperty = EMaterialExposedViewProperty(
        14,
    );
    pub const MEVP_POST_VOLUME_USER_FLAGS: EMaterialExposedViewProperty = EMaterialExposedViewProperty(
        15,
    );
    pub const MEVP_FIRST_PERSON_FIELD_OF_VIEW: EMaterialExposedViewProperty = EMaterialExposedViewProperty(
        16,
    );
    pub const MEVP_FIRST_PERSON_TAN_HALF_FIELD_OF_VIEW: EMaterialExposedViewProperty = EMaterialExposedViewProperty(
        17,
    );
    pub const MEVP_FIRST_PERSON_SCALE: EMaterialExposedViewProperty = EMaterialExposedViewProperty(
        18,
    );
    pub const MEVP_NEAR_PLANE: EMaterialExposedViewProperty = EMaterialExposedViewProperty(
        19,
    );
}
#[repr(transparent)]
pub struct EWorldPositionIncludedOffsets(pub u8);
impl EWorldPositionIncludedOffsets {
    pub const WPT_DEFAULT: EWorldPositionIncludedOffsets = EWorldPositionIncludedOffsets(
        0,
    );
    pub const WPT_EXCLUDE_ALL_SHADER_OFFSETS: EWorldPositionIncludedOffsets = EWorldPositionIncludedOffsets(
        1,
    );
    pub const WPT_CAMERA_RELATIVE: EWorldPositionIncludedOffsets = EWorldPositionIncludedOffsets(
        2,
    );
    pub const WPT_CAMERA_RELATIVE_NO_OFFSETS: EWorldPositionIncludedOffsets = EWorldPositionIncludedOffsets(
        3,
    );
}
#[repr(transparent)]
pub struct EMaterialFunctionUsage(pub u8);
impl EMaterialFunctionUsage {
    pub const DEFAULT: EMaterialFunctionUsage = EMaterialFunctionUsage(0);
    pub const MATERIAL_LAYER: EMaterialFunctionUsage = EMaterialFunctionUsage(1);
    pub const MATERIAL_LAYER_BLEND: EMaterialFunctionUsage = EMaterialFunctionUsage(2);
}
#[repr(transparent)]
pub struct EParticleSystemUpdateMode(pub u8);
impl EParticleSystemUpdateMode {
    pub const EPSUM_REAL_TIME: EParticleSystemUpdateMode = EParticleSystemUpdateMode(0);
    pub const EPSUM_FIXED_TIME: EParticleSystemUpdateMode = EParticleSystemUpdateMode(1);
}
#[repr(transparent)]
pub struct ParticleSystemLODMethod(pub u8);
impl ParticleSystemLODMethod {
    pub const PARTICLESYSTEMLODMETHOD_AUTOMATIC: ParticleSystemLODMethod = ParticleSystemLODMethod(
        0,
    );
    pub const PARTICLESYSTEMLODMETHOD_DIRECT_SET: ParticleSystemLODMethod = ParticleSystemLODMethod(
        1,
    );
    pub const PARTICLESYSTEMLODMETHOD_ACTIVATE_AUTOMATIC: ParticleSystemLODMethod = ParticleSystemLODMethod(
        2,
    );
}
#[repr(transparent)]
pub struct EParticleSystemInsignificanceReaction(pub u8);
impl EParticleSystemInsignificanceReaction {
    pub const AUTO: EParticleSystemInsignificanceReaction = EParticleSystemInsignificanceReaction(
        0,
    );
    pub const COMPLETE: EParticleSystemInsignificanceReaction = EParticleSystemInsignificanceReaction(
        1,
    );
    pub const DISABLE_TICK: EParticleSystemInsignificanceReaction = EParticleSystemInsignificanceReaction(
        2,
    );
    pub const DISABLE_TICK_AND_KILL: EParticleSystemInsignificanceReaction = EParticleSystemInsignificanceReaction(
        3,
    );
    pub const NUM: EParticleSystemInsignificanceReaction = EParticleSystemInsignificanceReaction(
        4,
    );
}
#[repr(transparent)]
pub struct EParticleSystemOcclusionBoundsMethod(pub u8);
impl EParticleSystemOcclusionBoundsMethod {
    pub const EPSOBM_NONE: EParticleSystemOcclusionBoundsMethod = EParticleSystemOcclusionBoundsMethod(
        0,
    );
    pub const EPSOBM_PARTICLE_BOUNDS: EParticleSystemOcclusionBoundsMethod = EParticleSystemOcclusionBoundsMethod(
        1,
    );
    pub const EPSOBM_CUSTOM_BOUNDS: EParticleSystemOcclusionBoundsMethod = EParticleSystemOcclusionBoundsMethod(
        2,
    );
}
#[repr(transparent)]
pub struct EParticleSignificanceLevel(pub u8);
impl EParticleSignificanceLevel {
    pub const LOW: EParticleSignificanceLevel = EParticleSignificanceLevel(0);
    pub const MEDIUM: EParticleSignificanceLevel = EParticleSignificanceLevel(1);
    pub const HIGH: EParticleSignificanceLevel = EParticleSignificanceLevel(2);
    pub const CRITICAL: EParticleSignificanceLevel = EParticleSignificanceLevel(3);
    pub const NUM: EParticleSignificanceLevel = EParticleSignificanceLevel(4);
}
#[repr(transparent)]
pub struct EAttractorParticleSelectionMethod(pub u8);
impl EAttractorParticleSelectionMethod {
    pub const EAPSM_RANDOM: EAttractorParticleSelectionMethod = EAttractorParticleSelectionMethod(
        0,
    );
    pub const EAPSM_SEQUENTIAL: EAttractorParticleSelectionMethod = EAttractorParticleSelectionMethod(
        1,
    );
}
#[repr(transparent)]
pub struct BeamModifierType(pub u8);
impl BeamModifierType {
    pub const PEB2MT_SOURCE: BeamModifierType = BeamModifierType(0);
    pub const PEB2MT_TARGET: BeamModifierType = BeamModifierType(1);
}
#[repr(transparent)]
pub struct Beam2SourceTargetMethod(pub u8);
impl Beam2SourceTargetMethod {
    pub const PEB2STM_DEFAULT: Beam2SourceTargetMethod = Beam2SourceTargetMethod(0);
    pub const PEB2STM_USER_SET: Beam2SourceTargetMethod = Beam2SourceTargetMethod(1);
    pub const PEB2STM_EMITTER: Beam2SourceTargetMethod = Beam2SourceTargetMethod(2);
    pub const PEB2STM_PARTICLE: Beam2SourceTargetMethod = Beam2SourceTargetMethod(3);
    pub const PEB2STM_ACTOR: Beam2SourceTargetMethod = Beam2SourceTargetMethod(4);
}
#[repr(transparent)]
pub struct Beam2SourceTargetTangentMethod(pub u8);
impl Beam2SourceTargetTangentMethod {
    pub const PEB2STTM_DIRECT: Beam2SourceTargetTangentMethod = Beam2SourceTargetTangentMethod(
        0,
    );
    pub const PEB2STTM_USER_SET: Beam2SourceTargetTangentMethod = Beam2SourceTargetTangentMethod(
        1,
    );
    pub const PEB2STTM_DISTRIBUTION: Beam2SourceTargetTangentMethod = Beam2SourceTargetTangentMethod(
        2,
    );
    pub const PEB2STTM_EMITTER: Beam2SourceTargetTangentMethod = Beam2SourceTargetTangentMethod(
        3,
    );
}
#[repr(transparent)]
pub struct EParticleCameraOffsetUpdateMethod(pub u8);
impl EParticleCameraOffsetUpdateMethod {
    pub const EPCOUM_DIRECT_SET: EParticleCameraOffsetUpdateMethod = EParticleCameraOffsetUpdateMethod(
        0,
    );
    pub const EPCOUM_ADDITIVE: EParticleCameraOffsetUpdateMethod = EParticleCameraOffsetUpdateMethod(
        1,
    );
    pub const EPCOUM_SCALAR: EParticleCameraOffsetUpdateMethod = EParticleCameraOffsetUpdateMethod(
        2,
    );
}
#[repr(transparent)]
pub struct EParticleCollisionComplete(pub u8);
impl EParticleCollisionComplete {
    pub const EPCC_KILL: EParticleCollisionComplete = EParticleCollisionComplete(0);
    pub const EPCC_FREEZE: EParticleCollisionComplete = EParticleCollisionComplete(1);
    pub const EPCC_HALT_COLLISIONS: EParticleCollisionComplete = EParticleCollisionComplete(
        2,
    );
    pub const EPCC_FREEZE_TRANSLATION: EParticleCollisionComplete = EParticleCollisionComplete(
        3,
    );
    pub const EPCC_FREEZE_ROTATION: EParticleCollisionComplete = EParticleCollisionComplete(
        4,
    );
    pub const EPCC_FREEZE_MOVEMENT: EParticleCollisionComplete = EParticleCollisionComplete(
        5,
    );
}
#[repr(transparent)]
pub struct EParticleCollisionResponse(pub u8);
impl EParticleCollisionResponse {
    pub const BOUNCE: EParticleCollisionResponse = EParticleCollisionResponse(0);
    pub const STOP: EParticleCollisionResponse = EParticleCollisionResponse(1);
    pub const KILL: EParticleCollisionResponse = EParticleCollisionResponse(2);
}
#[repr(transparent)]
pub struct ELocationBoneSocketSource(pub u8);
impl ELocationBoneSocketSource {
    pub const BONESOCKETSOURCE_BONES: ELocationBoneSocketSource = ELocationBoneSocketSource(
        0,
    );
    pub const BONESOCKETSOURCE_SOCKETS: ELocationBoneSocketSource = ELocationBoneSocketSource(
        1,
    );
}
#[repr(transparent)]
pub struct ELocationBoneSocketSelectionMethod(pub u8);
impl ELocationBoneSocketSelectionMethod {
    pub const BONESOCKETSEL_SEQUENTIAL: ELocationBoneSocketSelectionMethod = ELocationBoneSocketSelectionMethod(
        0,
    );
    pub const BONESOCKETSEL_RANDOM: ELocationBoneSocketSelectionMethod = ELocationBoneSocketSelectionMethod(
        1,
    );
}
#[repr(transparent)]
pub struct ELocationEmitterSelectionMethod(pub u8);
impl ELocationEmitterSelectionMethod {
    pub const ELESM_RANDOM: ELocationEmitterSelectionMethod = ELocationEmitterSelectionMethod(
        0,
    );
    pub const ELESM_SEQUENTIAL: ELocationEmitterSelectionMethod = ELocationEmitterSelectionMethod(
        1,
    );
}
#[repr(transparent)]
pub struct CylinderHeightAxis(pub u8);
impl CylinderHeightAxis {
    pub const PMLPC_HEIGHTAXIS_X: CylinderHeightAxis = CylinderHeightAxis(0);
    pub const PMLPC_HEIGHTAXIS_Y: CylinderHeightAxis = CylinderHeightAxis(1);
    pub const PMLPC_HEIGHTAXIS_Z: CylinderHeightAxis = CylinderHeightAxis(2);
}
#[repr(transparent)]
pub struct ELocationSkelVertSurfaceSource(pub u8);
impl ELocationSkelVertSurfaceSource {
    pub const VERTSURFACESOURCE_VERT: ELocationSkelVertSurfaceSource = ELocationSkelVertSurfaceSource(
        0,
    );
    pub const VERTSURFACESOURCE_SURFACE: ELocationSkelVertSurfaceSource = ELocationSkelVertSurfaceSource(
        1,
    );
}
#[repr(transparent)]
pub struct EOrbitChainMode(pub u8);
impl EOrbitChainMode {
    pub const EO_CHAIN_MODE_ADD: EOrbitChainMode = EOrbitChainMode(0);
    pub const EO_CHAIN_MODE_SCALE: EOrbitChainMode = EOrbitChainMode(1);
    pub const EO_CHAIN_MODE_LINK: EOrbitChainMode = EOrbitChainMode(2);
}
#[repr(transparent)]
pub struct EParticleSortMode(pub u8);
impl EParticleSortMode {
    pub const PSORTMODE_NONE: EParticleSortMode = EParticleSortMode(0);
    pub const PSORTMODE_VIEW_PROJ_DEPTH: EParticleSortMode = EParticleSortMode(1);
    pub const PSORTMODE_DISTANCE_TO_VIEW: EParticleSortMode = EParticleSortMode(2);
    pub const PSORTMODE_AGE_OLDEST_FIRST: EParticleSortMode = EParticleSortMode(3);
    pub const PSORTMODE_AGE_NEWEST_FIRST: EParticleSortMode = EParticleSortMode(4);
}
#[repr(transparent)]
pub struct EParticleSubUVInterpMethod(pub u8);
impl EParticleSubUVInterpMethod {
    pub const PSUVIM_NONE: EParticleSubUVInterpMethod = EParticleSubUVInterpMethod(0);
    pub const PSUVIM_LINEAR: EParticleSubUVInterpMethod = EParticleSubUVInterpMethod(1);
    pub const PSUVIM_LINEAR_BLEND: EParticleSubUVInterpMethod = EParticleSubUVInterpMethod(
        2,
    );
    pub const PSUVIM_RANDOM: EParticleSubUVInterpMethod = EParticleSubUVInterpMethod(3);
    pub const PSUVIM_RANDOM_BLEND: EParticleSubUVInterpMethod = EParticleSubUVInterpMethod(
        4,
    );
}
#[repr(transparent)]
pub struct EParticleBurstMethod(pub u8);
impl EParticleBurstMethod {
    pub const EPBM_INSTANT: EParticleBurstMethod = EParticleBurstMethod(0);
    pub const EPBM_INTERPOLATED: EParticleBurstMethod = EParticleBurstMethod(1);
}
#[repr(transparent)]
pub struct EOpacitySourceMode(pub u8);
impl EOpacitySourceMode {
    pub const OSM_ALPHA: EOpacitySourceMode = EOpacitySourceMode(0);
    pub const OSM_COLOR_BRIGHTNESS: EOpacitySourceMode = EOpacitySourceMode(1);
    pub const OSM_RED_CHANNEL: EOpacitySourceMode = EOpacitySourceMode(2);
    pub const OSM_GREEN_CHANNEL: EOpacitySourceMode = EOpacitySourceMode(3);
    pub const OSM_BLUE_CHANNEL: EOpacitySourceMode = EOpacitySourceMode(4);
}
#[repr(transparent)]
pub struct EEmitterNormalsMode(pub u8);
impl EEmitterNormalsMode {
    pub const ENM_CAMERA_FACING: EEmitterNormalsMode = EEmitterNormalsMode(0);
    pub const ENM_SPHERICAL: EEmitterNormalsMode = EEmitterNormalsMode(1);
    pub const ENM_CYLINDRICAL: EEmitterNormalsMode = EEmitterNormalsMode(2);
}
#[repr(transparent)]
pub struct EParticleUVFlipMode(pub u8);
impl EParticleUVFlipMode {
    pub const NONE: EParticleUVFlipMode = EParticleUVFlipMode(0);
    pub const FLIP_UV: EParticleUVFlipMode = EParticleUVFlipMode(1);
    pub const FLIP_U_ONLY: EParticleUVFlipMode = EParticleUVFlipMode(2);
    pub const FLIP_V_ONLY: EParticleUVFlipMode = EParticleUVFlipMode(3);
    pub const RANDOM_FLIP_UV: EParticleUVFlipMode = EParticleUVFlipMode(4);
    pub const RANDOM_FLIP_U_ONLY: EParticleUVFlipMode = EParticleUVFlipMode(5);
    pub const RANDOM_FLIP_V_ONLY: EParticleUVFlipMode = EParticleUVFlipMode(6);
    pub const RANDOM_FLIP_UV_INDEPENDENT: EParticleUVFlipMode = EParticleUVFlipMode(7);
}
#[repr(transparent)]
pub struct ESubUVBoundingVertexCount(pub u8);
impl ESubUVBoundingVertexCount {
    pub const BVC_FOUR_VERTICES: ESubUVBoundingVertexCount = ESubUVBoundingVertexCount(
        0,
    );
    pub const BVC_EIGHT_VERTICES: ESubUVBoundingVertexCount = ESubUVBoundingVertexCount(
        1,
    );
}
#[repr(transparent)]
pub struct ETrail2SourceMethod(pub u8);
impl ETrail2SourceMethod {
    pub const PET2SRCM_DEFAULT: ETrail2SourceMethod = ETrail2SourceMethod(0);
    pub const PET2SRCM_PARTICLE: ETrail2SourceMethod = ETrail2SourceMethod(1);
    pub const PET2SRCM_ACTOR: ETrail2SourceMethod = ETrail2SourceMethod(2);
}
#[repr(transparent)]
pub struct EParticleSourceSelectionMethod(pub u8);
impl EParticleSourceSelectionMethod {
    pub const EPSSM_RANDOM: EParticleSourceSelectionMethod = EParticleSourceSelectionMethod(
        0,
    );
    pub const EPSSM_SEQUENTIAL: EParticleSourceSelectionMethod = EParticleSourceSelectionMethod(
        1,
    );
}
#[repr(transparent)]
pub struct EBeam2Method(pub u8);
impl EBeam2Method {
    pub const PEB2M_DISTANCE: EBeam2Method = EBeam2Method(0);
    pub const PEB2M_TARGET: EBeam2Method = EBeam2Method(1);
    pub const PEB2M_BRANCH: EBeam2Method = EBeam2Method(2);
}
#[repr(transparent)]
pub struct EBeamTaperMethod(pub u8);
impl EBeamTaperMethod {
    pub const PEBTM_NONE: EBeamTaperMethod = EBeamTaperMethod(0);
    pub const PEBTM_FULL: EBeamTaperMethod = EBeamTaperMethod(1);
    pub const PEBTM_PARTIAL: EBeamTaperMethod = EBeamTaperMethod(2);
}
#[repr(transparent)]
pub struct EMeshScreenAlignment(pub u8);
impl EMeshScreenAlignment {
    pub const PSMA_MESH_FACE_CAMERA_WITH_ROLL: EMeshScreenAlignment = EMeshScreenAlignment(
        0,
    );
    pub const PSMA_MESH_FACE_CAMERA_WITH_SPIN: EMeshScreenAlignment = EMeshScreenAlignment(
        1,
    );
    pub const PSMA_MESH_FACE_CAMERA_WITH_LOCKED_AXIS: EMeshScreenAlignment = EMeshScreenAlignment(
        2,
    );
}
#[repr(transparent)]
pub struct EMeshCameraFacingUpAxis(pub u8);
impl EMeshCameraFacingUpAxis {
    pub const CAMERA_FACING_NONE_UP: EMeshCameraFacingUpAxis = EMeshCameraFacingUpAxis(
        0,
    );
    pub const CAMERA_FACING_Z_UP: EMeshCameraFacingUpAxis = EMeshCameraFacingUpAxis(1);
    pub const CAMERA_FACING_NEGATIVE_Z_UP: EMeshCameraFacingUpAxis = EMeshCameraFacingUpAxis(
        2,
    );
    pub const CAMERA_FACING_Y_UP: EMeshCameraFacingUpAxis = EMeshCameraFacingUpAxis(3);
    pub const CAMERA_FACING_NEGATIVE_Y_UP: EMeshCameraFacingUpAxis = EMeshCameraFacingUpAxis(
        4,
    );
}
#[repr(transparent)]
pub struct EMeshCameraFacingOptions(pub u8);
impl EMeshCameraFacingOptions {
    pub const X_AXIS_FACING_NO_UP: EMeshCameraFacingOptions = EMeshCameraFacingOptions(
        0,
    );
    pub const X_AXIS_FACING_Z_UP: EMeshCameraFacingOptions = EMeshCameraFacingOptions(1);
    pub const X_AXIS_FACING_NEGATIVE_Z_UP: EMeshCameraFacingOptions = EMeshCameraFacingOptions(
        2,
    );
    pub const X_AXIS_FACING_Y_UP: EMeshCameraFacingOptions = EMeshCameraFacingOptions(3);
    pub const X_AXIS_FACING_NEGATIVE_Y_UP: EMeshCameraFacingOptions = EMeshCameraFacingOptions(
        4,
    );
    pub const LOCKED_AXIS_Z_AXIS_FACING: EMeshCameraFacingOptions = EMeshCameraFacingOptions(
        5,
    );
    pub const LOCKED_AXIS_NEGATIVE_Z_AXIS_FACING: EMeshCameraFacingOptions = EMeshCameraFacingOptions(
        6,
    );
    pub const LOCKED_AXIS_Y_AXIS_FACING: EMeshCameraFacingOptions = EMeshCameraFacingOptions(
        7,
    );
    pub const LOCKED_AXIS_NEGATIVE_Y_AXIS_FACING: EMeshCameraFacingOptions = EMeshCameraFacingOptions(
        8,
    );
    pub const VELOCITY_ALIGNED_Z_AXIS_FACING: EMeshCameraFacingOptions = EMeshCameraFacingOptions(
        9,
    );
    pub const VELOCITY_ALIGNED_NEGATIVE_Z_AXIS_FACING: EMeshCameraFacingOptions = EMeshCameraFacingOptions(
        10,
    );
    pub const VELOCITY_ALIGNED_Y_AXIS_FACING: EMeshCameraFacingOptions = EMeshCameraFacingOptions(
        11,
    );
    pub const VELOCITY_ALIGNED_NEGATIVE_Y_AXIS_FACING: EMeshCameraFacingOptions = EMeshCameraFacingOptions(
        12,
    );
}
#[repr(transparent)]
pub struct ETrailsRenderAxisOption(pub u8);
impl ETrailsRenderAxisOption {
    pub const TRAILS_CAMERA_UP: ETrailsRenderAxisOption = ETrailsRenderAxisOption(0);
    pub const TRAILS_SOURCE_UP: ETrailsRenderAxisOption = ETrailsRenderAxisOption(1);
    pub const TRAILS_WORLD_UP: ETrailsRenderAxisOption = ETrailsRenderAxisOption(2);
}
#[repr(transparent)]
pub struct EEmitterRenderMode(pub u8);
impl EEmitterRenderMode {
    pub const ERM_NORMAL: EEmitterRenderMode = EEmitterRenderMode(0);
    pub const ERM_POINT: EEmitterRenderMode = EEmitterRenderMode(1);
    pub const ERM_CROSS: EEmitterRenderMode = EEmitterRenderMode(2);
    pub const ERM_LIGHTS_ONLY: EEmitterRenderMode = EEmitterRenderMode(3);
    pub const ERM_NONE: EEmitterRenderMode = EEmitterRenderMode(4);
}
#[repr(transparent)]
pub struct ESceneCapturePrimitiveRenderMode(pub u8);
impl ESceneCapturePrimitiveRenderMode {
    pub const PRM_LEGACY_SCENE_CAPTURE: ESceneCapturePrimitiveRenderMode = ESceneCapturePrimitiveRenderMode(
        0,
    );
    pub const PRM_RENDER_SCENE_PRIMITIVES: ESceneCapturePrimitiveRenderMode = ESceneCapturePrimitiveRenderMode(
        1,
    );
    pub const PRM_USE_SHOW_ONLY_LIST: ESceneCapturePrimitiveRenderMode = ESceneCapturePrimitiveRenderMode(
        2,
    );
}
#[repr(transparent)]
pub struct ESceneCaptureCompositeMode(pub u8);
impl ESceneCaptureCompositeMode {
    pub const SCCM_OVERWRITE: ESceneCaptureCompositeMode = ESceneCaptureCompositeMode(0);
    pub const SCCM_ADDITIVE: ESceneCaptureCompositeMode = ESceneCaptureCompositeMode(1);
    pub const SCCM_COMPOSITE: ESceneCaptureCompositeMode = ESceneCaptureCompositeMode(2);
}
#[repr(transparent)]
pub struct ESceneCaptureUnlitViewmode(pub u8);
impl ESceneCaptureUnlitViewmode {
    pub const DISABLED: ESceneCaptureUnlitViewmode = ESceneCaptureUnlitViewmode(0);
    pub const CAPTURE: ESceneCaptureUnlitViewmode = ESceneCaptureUnlitViewmode(1);
    pub const CAPTURE_OR_CUSTOM_RENDER_PASS: ESceneCaptureUnlitViewmode = ESceneCaptureUnlitViewmode(
        2,
    );
}
#[repr(transparent)]
pub struct EShadowMapFlags(pub u8);
impl EShadowMapFlags {
    pub const SMF_NONE: EShadowMapFlags = EShadowMapFlags(0);
    pub const SMF_STREAMED: EShadowMapFlags = EShadowMapFlags(1);
}
#[repr(transparent)]
pub struct ETextureEncodeEffort(pub u8);
impl ETextureEncodeEffort {
    pub const DEFAULT: ETextureEncodeEffort = ETextureEncodeEffort(0);
    pub const LOW: ETextureEncodeEffort = ETextureEncodeEffort(10);
    pub const NORMAL: ETextureEncodeEffort = ETextureEncodeEffort(20);
    pub const HIGH: ETextureEncodeEffort = ETextureEncodeEffort(30);
}
#[repr(transparent)]
pub struct ETextureUniversalTiling(pub u8);
impl ETextureUniversalTiling {
    pub const DISABLED: ETextureUniversalTiling = ETextureUniversalTiling(0);
    pub const ENABLED_256KB: ETextureUniversalTiling = ETextureUniversalTiling(1);
    pub const ENABLED_64KB: ETextureUniversalTiling = ETextureUniversalTiling(2);
}
#[repr(transparent)]
pub struct ETextureEncodeSpeed(pub u8);
impl ETextureEncodeSpeed {
    pub const FINAL: ETextureEncodeSpeed = ETextureEncodeSpeed(0);
    pub const FINAL_IF_AVAILABLE: ETextureEncodeSpeed = ETextureEncodeSpeed(1);
    pub const FAST: ETextureEncodeSpeed = ETextureEncodeSpeed(2);
}
#[repr(transparent)]
pub struct ETextureEncodeSpeedOverride(pub u8);
impl ETextureEncodeSpeedOverride {
    pub const DISABLED: ETextureEncodeSpeedOverride = ETextureEncodeSpeedOverride(255);
    pub const FINAL: ETextureEncodeSpeedOverride = ETextureEncodeSpeedOverride(0);
    pub const FINAL_IF_AVAILABLE: ETextureEncodeSpeedOverride = ETextureEncodeSpeedOverride(
        1,
    );
    pub const FAST: ETextureEncodeSpeedOverride = ETextureEncodeSpeedOverride(2);
}
#[repr(transparent)]
pub struct EVectorFieldConstructionOp(pub u8);
impl EVectorFieldConstructionOp {
    pub const VFCO_EXTRUDE: EVectorFieldConstructionOp = EVectorFieldConstructionOp(0);
    pub const VFCO_REVOLVE: EVectorFieldConstructionOp = EVectorFieldConstructionOp(1);
}
#[repr(transparent)]
pub struct ENotifyTriggerMode(pub u8);
impl ENotifyTriggerMode {
    pub const ALL_ANIMATIONS: ENotifyTriggerMode = ENotifyTriggerMode(0);
    pub const HIGHEST_WEIGHTED_ANIMATION: ENotifyTriggerMode = ENotifyTriggerMode(1);
    pub const NONE: ENotifyTriggerMode = ENotifyTriggerMode(2);
}
#[repr(transparent)]
pub struct EPreferredTriangulationDirection(pub u8);
impl EPreferredTriangulationDirection {
    pub const NONE: EPreferredTriangulationDirection = EPreferredTriangulationDirection(
        0,
    );
    pub const TANGENTIAL: EPreferredTriangulationDirection = EPreferredTriangulationDirection(
        1,
    );
    pub const RADIAL: EPreferredTriangulationDirection = EPreferredTriangulationDirection(
        2,
    );
}
#[repr(transparent)]
pub struct EBlendSpacePerBoneBlendMode(pub u8);
impl EBlendSpacePerBoneBlendMode {
    pub const MANUAL_PER_BONE_OVERRIDE: EBlendSpacePerBoneBlendMode = EBlendSpacePerBoneBlendMode(
        0,
    );
    pub const BLEND_PROFILE: EBlendSpacePerBoneBlendMode = EBlendSpacePerBoneBlendMode(
        1,
    );
}
#[repr(transparent)]
pub struct EBlendSpaceAxis(pub u8);
impl EBlendSpaceAxis {
    pub const BSA_NONE: EBlendSpaceAxis = EBlendSpaceAxis(0);
    pub const BSA_X: EBlendSpaceAxis = EBlendSpaceAxis(1);
    pub const BSA_Y: EBlendSpaceAxis = EBlendSpaceAxis(2);
}
#[repr(transparent)]
pub struct ECustomAttributeBlendType(pub u8);
impl ECustomAttributeBlendType {
    pub const OVERRIDE: ECustomAttributeBlendType = ECustomAttributeBlendType(0);
    pub const BLEND: ECustomAttributeBlendType = ECustomAttributeBlendType(1);
}
#[repr(transparent)]
pub struct AnimationCompressionFormat(pub u8);
impl AnimationCompressionFormat {
    pub const ACF_NONE: AnimationCompressionFormat = AnimationCompressionFormat(0);
    pub const ACF_FLOAT96_NO_W: AnimationCompressionFormat = AnimationCompressionFormat(
        1,
    );
    pub const ACF_FIXED48_NO_W: AnimationCompressionFormat = AnimationCompressionFormat(
        2,
    );
    pub const ACF_INTERVAL_FIXED32_NO_W: AnimationCompressionFormat = AnimationCompressionFormat(
        3,
    );
    pub const ACF_FIXED32_NO_W: AnimationCompressionFormat = AnimationCompressionFormat(
        4,
    );
    pub const ACF_FLOAT32_NO_W: AnimationCompressionFormat = AnimationCompressionFormat(
        5,
    );
    pub const ACF_IDENTITY: AnimationCompressionFormat = AnimationCompressionFormat(6);
}
#[repr(transparent)]
pub struct EAnimDataModelNotifyType(pub u8);
impl EAnimDataModelNotifyType {
    pub const BRACKET_OPENED: EAnimDataModelNotifyType = EAnimDataModelNotifyType(0);
    pub const BRACKET_CLOSED: EAnimDataModelNotifyType = EAnimDataModelNotifyType(1);
    pub const TRACK_ADDED: EAnimDataModelNotifyType = EAnimDataModelNotifyType(2);
    pub const TRACK_CHANGED: EAnimDataModelNotifyType = EAnimDataModelNotifyType(3);
    pub const TRACK_REMOVED: EAnimDataModelNotifyType = EAnimDataModelNotifyType(4);
    pub const SEQUENCE_LENGTH_CHANGED: EAnimDataModelNotifyType = EAnimDataModelNotifyType(
        5,
    );
    pub const FRAME_RATE_CHANGED: EAnimDataModelNotifyType = EAnimDataModelNotifyType(6);
    pub const CURVE_ADDED: EAnimDataModelNotifyType = EAnimDataModelNotifyType(7);
    pub const CURVE_CHANGED: EAnimDataModelNotifyType = EAnimDataModelNotifyType(8);
    pub const CURVE_REMOVED: EAnimDataModelNotifyType = EAnimDataModelNotifyType(9);
    pub const CURVE_FLAGS_CHANGED: EAnimDataModelNotifyType = EAnimDataModelNotifyType(
        10,
    );
    pub const CURVE_RENAMED: EAnimDataModelNotifyType = EAnimDataModelNotifyType(11);
    pub const CURVE_SCALED: EAnimDataModelNotifyType = EAnimDataModelNotifyType(12);
    pub const CURVE_COLOR_CHANGED: EAnimDataModelNotifyType = EAnimDataModelNotifyType(
        13,
    );
    pub const CURVE_COMMENT_CHANGED: EAnimDataModelNotifyType = EAnimDataModelNotifyType(
        14,
    );
    pub const ATTRIBUTE_ADDED: EAnimDataModelNotifyType = EAnimDataModelNotifyType(15);
    pub const ATTRIBUTE_REMOVED: EAnimDataModelNotifyType = EAnimDataModelNotifyType(16);
    pub const ATTRIBUTE_CHANGED: EAnimDataModelNotifyType = EAnimDataModelNotifyType(17);
    pub const POPULATED: EAnimDataModelNotifyType = EAnimDataModelNotifyType(18);
    pub const RESET: EAnimDataModelNotifyType = EAnimDataModelNotifyType(19);
    pub const SKELETON_CHANGED: EAnimDataModelNotifyType = EAnimDataModelNotifyType(20);
    pub const INVALID: EAnimDataModelNotifyType = EAnimDataModelNotifyType(21);
}
#[repr(transparent)]
pub struct EStripAnimDataOnDedicatedServerSettings(pub u8);
impl EStripAnimDataOnDedicatedServerSettings {
    pub const USE_PROJECT_SETTING: EStripAnimDataOnDedicatedServerSettings = EStripAnimDataOnDedicatedServerSettings(
        0,
    );
    pub const STRIP_ANIM_DATA_ON_DEDICATED_SERVER: EStripAnimDataOnDedicatedServerSettings = EStripAnimDataOnDedicatedServerSettings(
        1,
    );
    pub const DO_NOT_STRIP_ANIM_DATA_ON_DEDICATED_SERVER: EStripAnimDataOnDedicatedServerSettings = EStripAnimDataOnDedicatedServerSettings(
        2,
    );
}
#[repr(transparent)]
pub struct EBlendProfileMode(pub u8);
impl EBlendProfileMode {
    pub const TIME_FACTOR: EBlendProfileMode = EBlendProfileMode(0);
    pub const WEIGHT_FACTOR: EBlendProfileMode = EBlendProfileMode(1);
    pub const BLEND_MASK: EBlendProfileMode = EBlendProfileMode(2);
}
#[repr(transparent)]
pub struct EPreviewAnimationBlueprintApplicationMethod(pub u8);
impl EPreviewAnimationBlueprintApplicationMethod {
    pub const LINKED_LAYERS: EPreviewAnimationBlueprintApplicationMethod = EPreviewAnimationBlueprintApplicationMethod(
        0,
    );
    pub const LINKED_ANIM_GRAPH: EPreviewAnimationBlueprintApplicationMethod = EPreviewAnimationBlueprintApplicationMethod(
        1,
    );
}
#[repr(transparent)]
pub struct EAudioBusChannels(pub u8);
impl EAudioBusChannels {
    pub const MONO: EAudioBusChannels = EAudioBusChannels(0);
    pub const STEREO: EAudioBusChannels = EAudioBusChannels(1);
    pub const QUAD: EAudioBusChannels = EAudioBusChannels(3);
    pub const FIVE_POINT_ONE: EAudioBusChannels = EAudioBusChannels(5);
    pub const SEVEN_POINT_ONE: EAudioBusChannels = EAudioBusChannels(7);
    pub const MAX_CHANNEL_COUNT: EAudioBusChannels = EAudioBusChannels(8);
}
#[repr(transparent)]
pub struct EVoiceSampleRate(pub i32);
impl EVoiceSampleRate {
    pub const LOW16000_HZ: EVoiceSampleRate = EVoiceSampleRate(16000);
    pub const NORMAL24000_HZ: EVoiceSampleRate = EVoiceSampleRate(24000);
}
#[repr(transparent)]
pub struct EDefaultAudioCompressionType(pub u8);
impl EDefaultAudioCompressionType {
    pub const BINK_AUDIO: EDefaultAudioCompressionType = EDefaultAudioCompressionType(0);
    pub const ADPCM: EDefaultAudioCompressionType = EDefaultAudioCompressionType(1);
    pub const PCM: EDefaultAudioCompressionType = EDefaultAudioCompressionType(2);
    pub const OPUS: EDefaultAudioCompressionType = EDefaultAudioCompressionType(3);
    pub const PLATFORM_SPECIFIC: EDefaultAudioCompressionType = EDefaultAudioCompressionType(
        4,
    );
    pub const RAD_AUDIO: EDefaultAudioCompressionType = EDefaultAudioCompressionType(5);
}
#[repr(transparent)]
pub struct EPanningMethod(pub i8);
impl EPanningMethod {
    pub const LINEAR: EPanningMethod = EPanningMethod(0);
    pub const EQUAL_POWER: EPanningMethod = EPanningMethod(1);
}
#[repr(transparent)]
pub struct EMonoChannelUpmixMethod(pub i8);
impl EMonoChannelUpmixMethod {
    pub const LINEAR: EMonoChannelUpmixMethod = EMonoChannelUpmixMethod(0);
    pub const EQUAL_POWER: EMonoChannelUpmixMethod = EMonoChannelUpmixMethod(1);
    pub const FULL_VOLUME: EMonoChannelUpmixMethod = EMonoChannelUpmixMethod(2);
}
#[repr(transparent)]
pub struct ECameraShakeAttenuation(pub u8);
impl ECameraShakeAttenuation {
    pub const LINEAR: ECameraShakeAttenuation = ECameraShakeAttenuation(0);
    pub const QUADRATIC: ECameraShakeAttenuation = ECameraShakeAttenuation(1);
}
#[repr(transparent)]
pub struct ETemperatureSeverityType(pub u8);
impl ETemperatureSeverityType {
    pub const UNKNOWN: ETemperatureSeverityType = ETemperatureSeverityType(0);
    pub const GOOD: ETemperatureSeverityType = ETemperatureSeverityType(1);
    pub const BAD: ETemperatureSeverityType = ETemperatureSeverityType(2);
    pub const SERIOUS: ETemperatureSeverityType = ETemperatureSeverityType(3);
    pub const CRITICAL: ETemperatureSeverityType = ETemperatureSeverityType(4);
    pub const NUM_SEVERITIES: ETemperatureSeverityType = ETemperatureSeverityType(5);
}
#[repr(transparent)]
pub struct ENetworkSmoothingMode(pub u8);
impl ENetworkSmoothingMode {
    pub const DISABLED: ENetworkSmoothingMode = ENetworkSmoothingMode(0);
    pub const LINEAR: ENetworkSmoothingMode = ENetworkSmoothingMode(1);
    pub const EXPONENTIAL: ENetworkSmoothingMode = ENetworkSmoothingMode(2);
}
#[repr(transparent)]
pub struct EMegaLightsShadowMethod(pub u8);
impl EMegaLightsShadowMethod {
    pub const DEFAULT: EMegaLightsShadowMethod = EMegaLightsShadowMethod(0);
    pub const RAY_TRACING: EMegaLightsShadowMethod = EMegaLightsShadowMethod(1);
    pub const VIRTUAL_SHADOW_MAP: EMegaLightsShadowMethod = EMegaLightsShadowMethod(2);
}
#[repr(transparent)]
pub struct EInterpToBehaviourType(pub u8);
impl EInterpToBehaviourType {
    pub const ONE_SHOT: EInterpToBehaviourType = EInterpToBehaviourType(0);
    pub const ONE_SHOT_REVERSE: EInterpToBehaviourType = EInterpToBehaviourType(1);
    pub const LOOP_RESET: EInterpToBehaviourType = EInterpToBehaviourType(2);
    pub const PING_PONG: EInterpToBehaviourType = EInterpToBehaviourType(3);
}
#[repr(transparent)]
pub struct ERuntimeVirtualTextureMaterialQuality(pub u8);
impl ERuntimeVirtualTextureMaterialQuality {
    pub const LOW: ERuntimeVirtualTextureMaterialQuality = ERuntimeVirtualTextureMaterialQuality(
        0,
    );
    pub const MEDIUM: ERuntimeVirtualTextureMaterialQuality = ERuntimeVirtualTextureMaterialQuality(
        1,
    );
    pub const HIGH: ERuntimeVirtualTextureMaterialQuality = ERuntimeVirtualTextureMaterialQuality(
        2,
    );
    pub const EPIC: ERuntimeVirtualTextureMaterialQuality = ERuntimeVirtualTextureMaterialQuality(
        3,
    );
}
#[repr(transparent)]
pub struct ERuntimeVirtualTextureUseStreamingMipsInEditorMode(pub u8);
impl ERuntimeVirtualTextureUseStreamingMipsInEditorMode {
    pub const NEVER: ERuntimeVirtualTextureUseStreamingMipsInEditorMode = ERuntimeVirtualTextureUseStreamingMipsInEditorMode(
        0,
    );
    pub const PIE_ONLY: ERuntimeVirtualTextureUseStreamingMipsInEditorMode = ERuntimeVirtualTextureUseStreamingMipsInEditorMode(
        1,
    );
    pub const ALWAYS: ERuntimeVirtualTextureUseStreamingMipsInEditorMode = ERuntimeVirtualTextureUseStreamingMipsInEditorMode(
        2,
    );
}
#[repr(transparent)]
pub struct ESkyLightSourceType(pub u8);
impl ESkyLightSourceType {
    pub const SLS_CAPTURED_SCENE: ESkyLightSourceType = ESkyLightSourceType(0);
    pub const SLS_SPECIFIED_CUBEMAP: ESkyLightSourceType = ESkyLightSourceType(1);
}
#[repr(transparent)]
pub struct EOcclusionCombineMode(pub u8);
impl EOcclusionCombineMode {
    pub const OCM_MINIMUM: EOcclusionCombineMode = EOcclusionCombineMode(0);
    pub const OCM_MULTIPLY: EOcclusionCombineMode = EOcclusionCombineMode(1);
}
#[repr(transparent)]
pub struct ELastAuthority(pub u8);
impl ELastAuthority {
    pub const UNSET: ELastAuthority = ELastAuthority(0);
    pub const SPLINE: ELastAuthority = ELastAuthority(1);
    pub const SPLINE_CURVES: ELastAuthority = ELastAuthority(2);
}
#[repr(transparent)]
pub struct EStereoLayerType(pub u8);
impl EStereoLayerType {
    pub const SLT_WORLD_LOCKED: EStereoLayerType = EStereoLayerType(0);
    pub const SLT_TRACKER_LOCKED: EStereoLayerType = EStereoLayerType(1);
    pub const SLT_FACE_LOCKED: EStereoLayerType = EStereoLayerType(2);
}
#[repr(transparent)]
pub struct EVolumetricCloudTracingMaxDistanceMode(pub u8);
impl EVolumetricCloudTracingMaxDistanceMode {
    pub const DISTANCE_FROM_CLOUD_LAYER_ENTRY_POINT: EVolumetricCloudTracingMaxDistanceMode = EVolumetricCloudTracingMaxDistanceMode(
        0,
    );
    pub const DISTANCE_FROM_POINT_OF_VIEW: EVolumetricCloudTracingMaxDistanceMode = EVolumetricCloudTracingMaxDistanceMode(
        1,
    );
}
#[repr(transparent)]
pub struct EStreamingSourceTargetBehavior(pub u8);
impl EStreamingSourceTargetBehavior {
    pub const INCLUDE: EStreamingSourceTargetBehavior = EStreamingSourceTargetBehavior(
        0,
    );
    pub const EXCLUDE: EStreamingSourceTargetBehavior = EStreamingSourceTargetBehavior(
        1,
    );
}
#[repr(transparent)]
pub struct EStreamingSourceTargetState(pub u8);
impl EStreamingSourceTargetState {
    pub const LOADED: EStreamingSourceTargetState = EStreamingSourceTargetState(0);
    pub const ACTIVATED: EStreamingSourceTargetState = EStreamingSourceTargetState(1);
}
#[repr(transparent)]
pub struct EGrammaticalGender(pub u8);
impl EGrammaticalGender {
    pub const NEUTER: EGrammaticalGender = EGrammaticalGender(0);
    pub const MASCULINE: EGrammaticalGender = EGrammaticalGender(1);
    pub const FEMININE: EGrammaticalGender = EGrammaticalGender(2);
    pub const MIXED: EGrammaticalGender = EGrammaticalGender(3);
}
#[repr(transparent)]
pub struct EGrammaticalNumber(pub u8);
impl EGrammaticalNumber {
    pub const SINGULAR: EGrammaticalNumber = EGrammaticalNumber(0);
    pub const PLURAL: EGrammaticalNumber = EGrammaticalNumber(1);
}
#[repr(transparent)]
pub struct EFontCacheType(pub u8);
impl EFontCacheType {
    pub const OFFLINE: EFontCacheType = EFontCacheType(0);
    pub const RUNTIME: EFontCacheType = EFontCacheType(1);
}
#[repr(transparent)]
pub struct ERuntimeFontSource(pub u8);
impl ERuntimeFontSource {
    pub const ASSET: ERuntimeFontSource = ERuntimeFontSource(0);
    pub const CORE_STYLE_DEFAULT: ERuntimeFontSource = ERuntimeFontSource(1);
}
#[repr(transparent)]
pub struct EApplicationState(pub u8);
impl EApplicationState {
    pub const UNKNOWN: EApplicationState = EApplicationState(0);
    pub const INACTIVE: EApplicationState = EApplicationState(1);
    pub const BACKGROUND: EApplicationState = EApplicationState(2);
    pub const ACTIVE: EApplicationState = EApplicationState(3);
}
#[repr(transparent)]
pub struct ELevelInstanceRuntimeBehavior(pub u8);
impl ELevelInstanceRuntimeBehavior {
    pub const NONE: ELevelInstanceRuntimeBehavior = ELevelInstanceRuntimeBehavior(0);
    pub const EMBEDDED_DEPRECATED: ELevelInstanceRuntimeBehavior = ELevelInstanceRuntimeBehavior(
        1,
    );
    pub const PARTITIONED: ELevelInstanceRuntimeBehavior = ELevelInstanceRuntimeBehavior(
        2,
    );
    pub const LEVEL_STREAMING: ELevelInstanceRuntimeBehavior = ELevelInstanceRuntimeBehavior(
        3,
    );
}
#[repr(transparent)]
pub struct EStreamingVolumeUsage(pub u8);
impl EStreamingVolumeUsage {
    pub const SVB_LOADING: EStreamingVolumeUsage = EStreamingVolumeUsage(0);
    pub const SVB_LOADING_AND_VISIBILITY: EStreamingVolumeUsage = EStreamingVolumeUsage(
        1,
    );
    pub const SVB_VISIBILITY_BLOCKING_ON_LOAD: EStreamingVolumeUsage = EStreamingVolumeUsage(
        2,
    );
    pub const SVB_BLOCKING_ON_LOAD: EStreamingVolumeUsage = EStreamingVolumeUsage(3);
    pub const SVB_LOADING_NOT_VISIBLE: EStreamingVolumeUsage = EStreamingVolumeUsage(4);
}
#[repr(transparent)]
pub struct ENoiseFunction(pub u8);
impl ENoiseFunction {
    pub const NOISEFUNCTION_SIMPLEX_TEX: ENoiseFunction = ENoiseFunction(0);
    pub const NOISEFUNCTION_GRADIENT_TEX: ENoiseFunction = ENoiseFunction(1);
    pub const NOISEFUNCTION_GRADIENT_TEX3_D: ENoiseFunction = ENoiseFunction(2);
    pub const NOISEFUNCTION_GRADIENT_ALU: ENoiseFunction = ENoiseFunction(3);
    pub const NOISEFUNCTION_VALUE_ALU: ENoiseFunction = ENoiseFunction(4);
    pub const NOISEFUNCTION_VORONOI_ALU: ENoiseFunction = ENoiseFunction(5);
}
#[repr(transparent)]
pub struct EMaterialSubSurfaceType(pub u8);
impl EMaterialSubSurfaceType {
    pub const MSS_NONE: EMaterialSubSurfaceType = EMaterialSubSurfaceType(0);
    pub const MSS_WRAP: EMaterialSubSurfaceType = EMaterialSubSurfaceType(1);
    pub const MSS_TWO_SIDED_WRAP: EMaterialSubSurfaceType = EMaterialSubSurfaceType(2);
    pub const MSS_DIFFUSION: EMaterialSubSurfaceType = EMaterialSubSurfaceType(3);
    pub const MSS_DIFFUSION_PROFILE: EMaterialSubSurfaceType = EMaterialSubSurfaceType(
        4,
    );
    pub const MSS_SIMPLE_VOLUME: EMaterialSubSurfaceType = EMaterialSubSurfaceType(5);
}
#[repr(transparent)]
pub struct EPhysicsAssetSolverType(pub u8);
impl EPhysicsAssetSolverType {
    pub const RBAN: EPhysicsAssetSolverType = EPhysicsAssetSolverType(0);
    pub const WORLD: EPhysicsAssetSolverType = EPhysicsAssetSolverType(1);
}
#[repr(transparent)]
pub struct ESettingsLockedAxis(pub u8);
impl ESettingsLockedAxis {
    pub const NONE: ESettingsLockedAxis = ESettingsLockedAxis(0);
    pub const X: ESettingsLockedAxis = ESettingsLockedAxis(1);
    pub const Y: ESettingsLockedAxis = ESettingsLockedAxis(2);
    pub const Z: ESettingsLockedAxis = ESettingsLockedAxis(3);
    pub const INVALID: ESettingsLockedAxis = ESettingsLockedAxis(4);
}
#[repr(transparent)]
pub struct ESettingsDOF(pub u8);
impl ESettingsDOF {
    pub const FULL3_D: ESettingsDOF = ESettingsDOF(0);
    pub const YZ_PLANE: ESettingsDOF = ESettingsDOF(1);
    pub const XZ_PLANE: ESettingsDOF = ESettingsDOF(2);
    pub const XY_PLANE: ESettingsDOF = ESettingsDOF(3);
}
#[repr(transparent)]
pub struct EMobileShadingPath(pub u8);
impl EMobileShadingPath {
    pub const FORWARD: EMobileShadingPath = EMobileShadingPath(0);
    pub const DEFERRED: EMobileShadingPath = EMobileShadingPath(1);
}
#[repr(transparent)]
pub struct EMobileAntiAliasingMethod(pub u8);
impl EMobileAntiAliasingMethod {
    pub const NONE: EMobileAntiAliasingMethod = EMobileAntiAliasingMethod(0);
    pub const FXAA: EMobileAntiAliasingMethod = EMobileAntiAliasingMethod(1);
    pub const TEMPORAL_AA: EMobileAntiAliasingMethod = EMobileAntiAliasingMethod(2);
    pub const MSAA: EMobileAntiAliasingMethod = EMobileAntiAliasingMethod(3);
    pub const TSR: EMobileAntiAliasingMethod = EMobileAntiAliasingMethod(4);
    pub const SMAA: EMobileAntiAliasingMethod = EMobileAntiAliasingMethod(5);
}
#[repr(transparent)]
pub struct EMobileFloatPrecisionMode(pub u8);
impl EMobileFloatPrecisionMode {
    pub const HALF: EMobileFloatPrecisionMode = EMobileFloatPrecisionMode(0);
    pub const FULL_MATERIAL_EXPRESSION_ONLY: EMobileFloatPrecisionMode = EMobileFloatPrecisionMode(
        1,
    );
    pub const FULL: EMobileFloatPrecisionMode = EMobileFloatPrecisionMode(2);
}
#[repr(transparent)]
pub struct EShaderCompressionFormat(pub u8);
impl EShaderCompressionFormat {
    pub const NONE: EShaderCompressionFormat = EShaderCompressionFormat(0);
    pub const LZ4: EShaderCompressionFormat = EShaderCompressionFormat(1);
    pub const OODLE: EShaderCompressionFormat = EShaderCompressionFormat(2);
    pub const ZLIB: EShaderCompressionFormat = EShaderCompressionFormat(3);
}
#[repr(transparent)]
pub struct EWorkingColorSpace(pub u8);
impl EWorkingColorSpace {
    pub const S_RGB: EWorkingColorSpace = EWorkingColorSpace(1);
    pub const REC2020: EWorkingColorSpace = EWorkingColorSpace(2);
    pub const ACESAP0: EWorkingColorSpace = EWorkingColorSpace(3);
    pub const ACESAP1: EWorkingColorSpace = EWorkingColorSpace(4);
    pub const P3DCI: EWorkingColorSpace = EWorkingColorSpace(5);
    pub const P3D65: EWorkingColorSpace = EWorkingColorSpace(6);
    pub const CUSTOM: EWorkingColorSpace = EWorkingColorSpace(7);
}
#[repr(transparent)]
pub struct ELumenRayLightingMode(pub u8);
impl ELumenRayLightingMode {
    pub const SURFACE_CACHE: ELumenRayLightingMode = ELumenRayLightingMode(0);
    pub const HIT_LIGHTING_FOR_REFLECTIONS: ELumenRayLightingMode = ELumenRayLightingMode(
        2,
    );
    pub const HIT_LIGHTING: ELumenRayLightingMode = ELumenRayLightingMode(1);
}
#[repr(transparent)]
pub struct ELumenSoftwareTracingMode(pub u8);
impl ELumenSoftwareTracingMode {
    pub const DETAIL_TRACING: ELumenSoftwareTracingMode = ELumenSoftwareTracingMode(1);
    pub const GLOBAL_TRACING: ELumenSoftwareTracingMode = ELumenSoftwareTracingMode(0);
}
#[repr(transparent)]
pub struct ELumenScreenTracingSource(pub u8);
impl ELumenScreenTracingSource {
    pub const SCENE_COLOR: ELumenScreenTracingSource = ELumenScreenTracingSource(0);
    pub const ANTIALIASED_SCENE_COLOR_WITH_TRANSLUCENCY: ELumenScreenTracingSource = ELumenScreenTracingSource(
        1,
    );
}
#[repr(transparent)]
pub struct EShadowMapMethod(pub u8);
impl EShadowMapMethod {
    pub const SHADOW_MAPS: EShadowMapMethod = EShadowMapMethod(0);
    pub const VIRTUAL_SHADOW_MAPS: EShadowMapMethod = EShadowMapMethod(1);
}
#[repr(transparent)]
pub struct ETranslucentSortPolicy(pub u8);
impl ETranslucentSortPolicy {
    pub const SORT_BY_DISTANCE: ETranslucentSortPolicy = ETranslucentSortPolicy(0);
    pub const SORT_BY_PROJECTED_Z: ETranslucentSortPolicy = ETranslucentSortPolicy(1);
    pub const SORT_ALONG_AXIS: ETranslucentSortPolicy = ETranslucentSortPolicy(2);
}
#[repr(transparent)]
pub struct EFixedFoveationLevels(pub u8);
impl EFixedFoveationLevels {
    pub const DISABLED: EFixedFoveationLevels = EFixedFoveationLevels(0);
    pub const LOW: EFixedFoveationLevels = EFixedFoveationLevels(1);
    pub const MEDIUM: EFixedFoveationLevels = EFixedFoveationLevels(2);
    pub const HIGH: EFixedFoveationLevels = EFixedFoveationLevels(3);
    pub const HIGH_TOP: EFixedFoveationLevels = EFixedFoveationLevels(4);
}
#[repr(transparent)]
pub struct ECustomDepthStencil(pub u8);
impl ECustomDepthStencil {
    pub const DISABLED: ECustomDepthStencil = ECustomDepthStencil(0);
    pub const ENABLED: ECustomDepthStencil = ECustomDepthStencil(1);
    pub const ENABLED_ON_DEMAND: ECustomDepthStencil = ECustomDepthStencil(2);
    pub const ENABLED_WITH_STENCIL: ECustomDepthStencil = ECustomDepthStencil(3);
}
#[repr(transparent)]
pub struct EAutoExposureMethodUI(pub u8);
impl EAutoExposureMethodUI {
    pub const AEM_HISTOGRAM: EAutoExposureMethodUI = EAutoExposureMethodUI(0);
    pub const AEM_BASIC: EAutoExposureMethodUI = EAutoExposureMethodUI(1);
    pub const AEM_MANUAL: EAutoExposureMethodUI = EAutoExposureMethodUI(2);
}
#[repr(transparent)]
pub struct EAntiAliasingMethod(pub u8);
impl EAntiAliasingMethod {
    pub const AAM_NONE: EAntiAliasingMethod = EAntiAliasingMethod(0);
    pub const AAM_FXAA: EAntiAliasingMethod = EAntiAliasingMethod(1);
    pub const AAM_TEMPORAL_AA: EAntiAliasingMethod = EAntiAliasingMethod(2);
    pub const AAM_MSAA: EAntiAliasingMethod = EAntiAliasingMethod(3);
    pub const AAM_TSR: EAntiAliasingMethod = EAntiAliasingMethod(4);
    pub const AAM_SMAA: EAntiAliasingMethod = EAntiAliasingMethod(5);
}
#[repr(transparent)]
pub struct ECompositingSampleCount(pub u8);
impl ECompositingSampleCount {
    pub const ONE: ECompositingSampleCount = ECompositingSampleCount(1);
    pub const TWO: ECompositingSampleCount = ECompositingSampleCount(2);
    pub const FOUR: ECompositingSampleCount = ECompositingSampleCount(4);
    pub const EIGHT: ECompositingSampleCount = ECompositingSampleCount(8);
}
#[repr(transparent)]
pub struct EDefaultBackBufferPixelFormat(pub u8);
impl EDefaultBackBufferPixelFormat {
    pub const DBBPF_B8G8R8A8: EDefaultBackBufferPixelFormat = EDefaultBackBufferPixelFormat(
        0,
    );
    pub const DBBPF_A16B16G16R16_DEPRECATED: EDefaultBackBufferPixelFormat = EDefaultBackBufferPixelFormat(
        1,
    );
    pub const DBBPF_FLOAT_RGB_DEPRECATED: EDefaultBackBufferPixelFormat = EDefaultBackBufferPixelFormat(
        2,
    );
    pub const DBBPF_FLOAT_RGBA: EDefaultBackBufferPixelFormat = EDefaultBackBufferPixelFormat(
        3,
    );
    pub const DBBPF_A2B10G10R10: EDefaultBackBufferPixelFormat = EDefaultBackBufferPixelFormat(
        4,
    );
}
#[repr(transparent)]
pub struct EEarlyZPass(pub u8);
impl EEarlyZPass {
    pub const NONE: EEarlyZPass = EEarlyZPass(0);
    pub const OPAQUE_ONLY: EEarlyZPass = EEarlyZPass(1);
    pub const OPAQUE_AND_MASKED: EEarlyZPass = EEarlyZPass(2);
    pub const AUTO: EEarlyZPass = EEarlyZPass(3);
}
#[repr(transparent)]
pub struct EClearSceneOptions(pub u8);
impl EClearSceneOptions {
    pub const NO_CLEAR: EClearSceneOptions = EClearSceneOptions(0);
    pub const HARDWARE_CLEAR: EClearSceneOptions = EClearSceneOptions(1);
    pub const QUAD_AT_MAX_Z: EClearSceneOptions = EClearSceneOptions(2);
}
#[repr(transparent)]
pub struct EVelocityOutputPass(pub u8);
impl EVelocityOutputPass {
    pub const DEPTH_PASS: EVelocityOutputPass = EVelocityOutputPass(0);
    pub const BASE_PASS: EVelocityOutputPass = EVelocityOutputPass(1);
    pub const AFTER_BASE_PASS: EVelocityOutputPass = EVelocityOutputPass(2);
}
#[repr(transparent)]
pub struct EVertexDeformationOutputsVelocity(pub u8);
impl EVertexDeformationOutputsVelocity {
    pub const OFF: EVertexDeformationOutputsVelocity = EVertexDeformationOutputsVelocity(
        0,
    );
    pub const ON: EVertexDeformationOutputsVelocity = EVertexDeformationOutputsVelocity(
        1,
    );
    pub const AUTO: EVertexDeformationOutputsVelocity = EVertexDeformationOutputsVelocity(
        2,
    );
}
#[repr(transparent)]
pub struct EGBufferFormat(pub u8);
impl EGBufferFormat {
    pub const FORCE8_BITS_PER_CHANNEL: EGBufferFormat = EGBufferFormat(0);
    pub const DEFAULT: EGBufferFormat = EGBufferFormat(1);
    pub const HIGH_PRECISION_NORMALS: EGBufferFormat = EGBufferFormat(3);
    pub const FORCE16_BITS_PER_CHANNEL: EGBufferFormat = EGBufferFormat(5);
}
#[repr(transparent)]
pub struct ELightFunctionAtlasPixelFormat(pub u8);
impl ELightFunctionAtlasPixelFormat {
    pub const LFAPF_R8: ELightFunctionAtlasPixelFormat = ELightFunctionAtlasPixelFormat(
        0,
    );
    pub const LFAPF_R8G8B8: ELightFunctionAtlasPixelFormat = ELightFunctionAtlasPixelFormat(
        1,
    );
}
#[repr(transparent)]
pub struct ESubstrateStorageFormat(pub u8);
impl ESubstrateStorageFormat {
    pub const BLENDABLE_G_BUFFER: ESubstrateStorageFormat = ESubstrateStorageFormat(0);
    pub const ADAPTIVE_BUFFER: ESubstrateStorageFormat = ESubstrateStorageFormat(1);
}
#[repr(transparent)]
pub struct ESubstrateClosureConfig(pub u8);
impl ESubstrateClosureConfig {
    pub const USE_MIN_PROJECT_AND_PLATFOR_CLOSURES_PER_PIXEL: ESubstrateClosureConfig = ESubstrateClosureConfig(
        0,
    );
    pub const FORCE_PROJECT_CLOSURES_PER_PIXEL: ESubstrateClosureConfig = ESubstrateClosureConfig(
        1,
    );
}
#[repr(transparent)]
pub struct ESkinCacheDefaultBehavior(pub u8);
impl ESkinCacheDefaultBehavior {
    pub const EXCLUSIVE: ESkinCacheDefaultBehavior = ESkinCacheDefaultBehavior(0);
    pub const INCLUSIVE: ESkinCacheDefaultBehavior = ESkinCacheDefaultBehavior(1);
}
#[repr(transparent)]
pub struct EMobileLocalLightSetting(pub u8);
impl EMobileLocalLightSetting {
    pub const LOCAL_LIGHTS_DISABLED: EMobileLocalLightSetting = EMobileLocalLightSetting(
        0,
    );
    pub const LOCAL_LIGHTS_ENABLED: EMobileLocalLightSetting = EMobileLocalLightSetting(
        1,
    );
    pub const LOCAL_LIGHTS_BUFFER: EMobileLocalLightSetting = EMobileLocalLightSetting(
        2,
    );
}
#[repr(transparent)]
pub struct EClothLODBiasMode(pub u8);
impl EClothLODBiasMode {
    pub const MAPPINGS_TO_SAME_LOD: EClothLODBiasMode = EClothLODBiasMode(0);
    pub const MAPPINGS_TO_MIN_LOD: EClothLODBiasMode = EClothLODBiasMode(1);
    pub const MAPPINGS_TO_ANY_LOD: EClothLODBiasMode = EClothLODBiasMode(2);
}
#[repr(transparent)]
pub struct ESourceBusChannels(pub u8);
impl ESourceBusChannels {
    pub const MONO: ESourceBusChannels = ESourceBusChannels(0);
    pub const STEREO: ESourceBusChannels = ESourceBusChannels(1);
}
#[repr(transparent)]
pub struct EStaticMeshPaintSupport(pub u8);
impl EStaticMeshPaintSupport {
    pub const DEFAULT: EStaticMeshPaintSupport = EStaticMeshPaintSupport(0);
    pub const ENABLED: EStaticMeshPaintSupport = EStaticMeshPaintSupport(1);
    pub const DISABLED: EStaticMeshPaintSupport = EStaticMeshPaintSupport(2);
}
#[repr(transparent)]
pub struct ERenderFocusRule(pub u8);
impl ERenderFocusRule {
    pub const ALWAYS: ERenderFocusRule = ERenderFocusRule(0);
    pub const NON_POINTER: ERenderFocusRule = ERenderFocusRule(1);
    pub const NAVIGATION_ONLY: ERenderFocusRule = ERenderFocusRule(2);
    pub const NEVER: ERenderFocusRule = ERenderFocusRule(3);
}
#[repr(transparent)]
pub struct EUIScalingRule(pub u8);
impl EUIScalingRule {
    pub const SHORTEST_SIDE: EUIScalingRule = EUIScalingRule(0);
    pub const LONGEST_SIDE: EUIScalingRule = EUIScalingRule(1);
    pub const HORIZONTAL: EUIScalingRule = EUIScalingRule(2);
    pub const VERTICAL: EUIScalingRule = EUIScalingRule(3);
    pub const SCALE_TO_FIT: EUIScalingRule = EUIScalingRule(4);
    pub const CUSTOM: EUIScalingRule = EUIScalingRule(5);
}
#[repr(transparent)]
pub struct EFontDPI(pub u8);
impl EFontDPI {
    pub const STANDARD: EFontDPI = EFontDPI(0);
    pub const UNREAL: EFontDPI = EFontDPI(1);
    pub const CUSTOM: EFontDPI = EFontDPI(2);
}
#[repr(transparent)]
pub struct EDataLayerLoadFilter(pub u8);
impl EDataLayerLoadFilter {
    pub const NONE: EDataLayerLoadFilter = EDataLayerLoadFilter(0);
    pub const CLIENT_ONLY: EDataLayerLoadFilter = EDataLayerLoadFilter(1);
    pub const SERVER_ONLY: EDataLayerLoadFilter = EDataLayerLoadFilter(2);
}
#[repr(transparent)]
pub struct EOverrideBlockOnSlowStreaming(pub u8);
impl EOverrideBlockOnSlowStreaming {
    pub const NO_OVERRIDE: EOverrideBlockOnSlowStreaming = EOverrideBlockOnSlowStreaming(
        0,
    );
    pub const BLOCKING: EOverrideBlockOnSlowStreaming = EOverrideBlockOnSlowStreaming(1);
    pub const NOT_BLOCKING: EOverrideBlockOnSlowStreaming = EOverrideBlockOnSlowStreaming(
        2,
    );
}
#[repr(transparent)]
pub struct EWorldPartitionServerStreamingMode(pub u8);
impl EWorldPartitionServerStreamingMode {
    pub const PROJECT_DEFAULT: EWorldPartitionServerStreamingMode = EWorldPartitionServerStreamingMode(
        0,
    );
    pub const DISABLED: EWorldPartitionServerStreamingMode = EWorldPartitionServerStreamingMode(
        1,
    );
    pub const ENABLED: EWorldPartitionServerStreamingMode = EWorldPartitionServerStreamingMode(
        2,
    );
    pub const ENABLED_IN_PIE: EWorldPartitionServerStreamingMode = EWorldPartitionServerStreamingMode(
        3,
    );
}
#[repr(transparent)]
pub struct EWorldPartitionServerStreamingOutMode(pub u8);
impl EWorldPartitionServerStreamingOutMode {
    pub const PROJECT_DEFAULT: EWorldPartitionServerStreamingOutMode = EWorldPartitionServerStreamingOutMode(
        0,
    );
    pub const DISABLED: EWorldPartitionServerStreamingOutMode = EWorldPartitionServerStreamingOutMode(
        1,
    );
    pub const ENABLED: EWorldPartitionServerStreamingOutMode = EWorldPartitionServerStreamingOutMode(
        2,
    );
}
#[repr(transparent)]
pub struct EWorldPartitionDataLayersLogicOperator(pub u8);
impl EWorldPartitionDataLayersLogicOperator {
    pub const OR: EWorldPartitionDataLayersLogicOperator = EWorldPartitionDataLayersLogicOperator(
        0,
    );
    pub const AND: EWorldPartitionDataLayersLogicOperator = EWorldPartitionDataLayersLogicOperator(
        1,
    );
}
#[repr(transparent)]
pub struct EHLODLayerType(pub u8);
impl EHLODLayerType {
    pub const INSTANCING: EHLODLayerType = EHLODLayerType(0);
    pub const MESH_MERGE: EHLODLayerType = EHLODLayerType(1);
    pub const MESH_SIMPLIFY: EHLODLayerType = EHLODLayerType(2);
    pub const MESH_APPROXIMATE: EHLODLayerType = EHLODLayerType(3);
    pub const CUSTOM: EHLODLayerType = EHLODLayerType(4);
    pub const CUSTOM_HLOD_ACTOR: EHLODLayerType = EHLODLayerType(5);
}
#[repr(transparent)]
pub struct ERuntimePartitionCellBoundsMethod(pub u8);
impl ERuntimePartitionCellBoundsMethod {
    pub const USE_CONTENT: ERuntimePartitionCellBoundsMethod = ERuntimePartitionCellBoundsMethod(
        0,
    );
    pub const USE_CELL_BOUNDS: ERuntimePartitionCellBoundsMethod = ERuntimePartitionCellBoundsMethod(
        1,
    );
    pub const USE_MIN_CONTENT_CELL_BOUNDS: ERuntimePartitionCellBoundsMethod = ERuntimePartitionCellBoundsMethod(
        2,
    );
}
#[repr(transparent)]
pub struct EWorldPartitionStreamingPerformance(pub u8);
impl EWorldPartitionStreamingPerformance {
    pub const GOOD: EWorldPartitionStreamingPerformance = EWorldPartitionStreamingPerformance(
        0,
    );
    pub const SLOW: EWorldPartitionStreamingPerformance = EWorldPartitionStreamingPerformance(
        1,
    );
    pub const CRITICAL: EWorldPartitionStreamingPerformance = EWorldPartitionStreamingPerformance(
        2,
    );
    pub const IMMEDIATE: EWorldPartitionStreamingPerformance = EWorldPartitionStreamingPerformance(
        3,
    );
}
#[repr(transparent)]
pub struct EWorldPartitionCVarProjectDefaultOverride(pub u8);
impl EWorldPartitionCVarProjectDefaultOverride {
    pub const PROJECT_DEFAULT: EWorldPartitionCVarProjectDefaultOverride = EWorldPartitionCVarProjectDefaultOverride(
        0,
    );
    pub const DISABLED: EWorldPartitionCVarProjectDefaultOverride = EWorldPartitionCVarProjectDefaultOverride(
        1,
    );
    pub const ENABLED: EWorldPartitionCVarProjectDefaultOverride = EWorldPartitionCVarProjectDefaultOverride(
        2,
    );
}
#[repr(transparent)]
pub struct EVisibilityAggressiveness(pub u8);
impl EVisibilityAggressiveness {
    pub const VIS_LEAST_AGGRESSIVE: EVisibilityAggressiveness = EVisibilityAggressiveness(
        0,
    );
    pub const VIS_MODERATELY_AGGRESSIVE: EVisibilityAggressiveness = EVisibilityAggressiveness(
        1,
    );
    pub const VIS_MOST_AGGRESSIVE: EVisibilityAggressiveness = EVisibilityAggressiveness(
        2,
    );
}
