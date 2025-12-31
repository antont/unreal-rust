#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(8))]
pub struct FMaterialInput {
    pub expression: UPtr<UMaterialExpression>,
    pub output_index: i32,
    pub input_name: FName,
    pub mask: i32,
    pub mask_r: i32,
    pub mask_g: i32,
    pub mask_b: i32,
    pub mask_a: i32,
}
#[repr(C, align(8))]
pub struct FColorMaterialInput {
    pub flags_48: u8,
    pub constant: crate::bindings::core_u_object::FLinearColor,
}
#[repr(C, align(8))]
pub struct FDistributionLookupTable {
    pub time_scale: f32,
    pub time_bias: f32,
    pub values: TArray<f32>,
    pub op: u8,
    pub entry_count: u8,
    pub entry_stride: u8,
    pub sub_entry_stride: u8,
    pub lock_flag: u8,
}
#[repr(C, align(8))]
pub struct FExpressionInput {
    pub expression: UPtr<UMaterialExpression>,
    pub output_index: i32,
    pub input_name: FName,
    pub mask: i32,
    pub mask_r: i32,
    pub mask_g: i32,
    pub mask_b: i32,
    pub mask_a: i32,
}
#[repr(C, align(4))]
pub struct FExpressionOutput {
    pub output_name: FName,
    pub mask: i32,
    pub mask_r: i32,
    pub mask_g: i32,
    pub mask_b: i32,
    pub mask_a: i32,
}
#[repr(C, align(8))]
pub struct FFloatDistribution {
    pub table: FDistributionLookupTable,
}
#[repr(C, align(4))]
pub struct FFloatRK4SpringInterpolator {
    pub stiffness_constant: f32,
    pub dampening_ratio: f32,
}
#[repr(C, align(8))]
pub struct FFormatArgumentData {
    pub argument_name: FString,
    pub argument_value_type: EFormatArgumentType,
    pub argument_value: FText,
    pub argument_value_int: i64,
    pub argument_value_float: f32,
    pub argument_value_double: f64,
    pub argument_value_gender: ETextGender,
}
#[repr(C, align(8))]
pub struct FMaterialAttributesInput {
    pub property_connected_mask: i64,
}
#[repr(C, align(8))]
pub struct FRawDistribution {
    pub table: FDistributionLookupTable,
}
#[repr(C, align(8))]
pub struct FScalarMaterialInput {
    pub flags_48: u8,
    pub constant: f32,
}
#[repr(C, align(8))]
pub struct FShadingModelMaterialInput {}
#[repr(C, align(8))]
pub struct FSubstrateMaterialInput {}
#[repr(C, align(8))]
pub struct FVector2MaterialInput {
    pub flags_48: u8,
    pub constant_x: f32,
    pub constant_y: f32,
}
#[repr(C, align(8))]
pub struct FVector4Distribution {
    pub table: FDistributionLookupTable,
}
#[repr(C, align(8))]
pub struct FVectorDistribution {
    pub table: FDistributionLookupTable,
}
#[repr(C, align(8))]
pub struct FVectorMaterialInput {
    pub flags_48: u8,
    pub constant: crate::bindings::core_u_object::FVector3f,
}
#[repr(C, align(4))]
pub struct FVectorRK4SpringInterpolator {
    pub stiffness_constant: f32,
    pub dampening_ratio: f32,
}
#[repr(C, align(8))]
pub struct FAnimDataModelNotifPayload {}
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
}
#[repr(C, align(8))]
pub struct FCollisionChaosEvent {
    pub location: crate::bindings::core_u_object::FVector,
    pub accumulated_impulse: crate::bindings::core_u_object::FVector,
    pub normal: crate::bindings::core_u_object::FVector,
    pub penetration_depth: f32,
    pub body1: FCollisionChaosEventBodyInfo,
    pub body2: FCollisionChaosEventBodyInfo,
}
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
#[repr(C, align(8))]
pub struct FHitResult {
    pub face_index: i32,
    pub time: f32,
    pub distance: f32,
    pub location: FVector_NetQuantize,
    pub impact_point: FVector_NetQuantize,
    pub normal: FVector_NetQuantizeNormal,
    pub impact_normal: FVector_NetQuantizeNormal,
    pub trace_start: FVector_NetQuantize,
    pub trace_end: FVector_NetQuantize,
    pub penetration_depth: f32,
    pub my_item: i32,
    pub item: i32,
    pub element_index: u8,
    pub flags_173: u8,
    pub phys_material: TWeakObjectPtr<crate::bindings::physics_core::UPhysicalMaterial>,
    pub hit_object_handle: FActorInstanceHandle,
    pub component: TWeakObjectPtr<UPrimitiveComponent>,
    pub bone_name: FName,
    pub my_bone_name: FName,
}
#[repr(C, align(8))]
pub struct FActorInstanceHandle {
    pub reference_object: TWeakObjectPtr<crate::bindings::core_u_object::UObject>,
}
#[repr(C, align(8))]
pub struct FVector_NetQuantize {}
#[repr(C, align(8))]
pub struct FVector_NetQuantizeNormal {}
#[repr(C, align(8))]
pub struct FSubtitleCue {
    pub text: FText,
    pub time: f32,
}
#[repr(C, align(8))]
pub struct FPlatformInterfaceDelegateResult {
    pub b_successful: bool,
    pub data: FPlatformInterfaceData,
}
#[repr(C, align(8))]
pub struct FPlatformInterfaceData {
    pub data_name: FName,
    pub ty: EPlatformInterfaceDataType,
    pub int_value: i32,
    pub float_value: f32,
    pub string_value: FString,
    pub object_value: UPtr<crate::bindings::core_u_object::UObject>,
}
#[repr(C, align(8))]
pub struct FBranchingPointNotifyPayload {}
#[repr(C, align(8))]
pub struct FChaosRemovalEvent {
    pub component: UPtr<UPrimitiveComponent>,
    pub location: crate::bindings::core_u_object::FVector,
    pub mass: f32,
}
#[repr(C, align(8))]
pub struct FSimpleMemberReference {
    pub member_parent: UPtr<crate::bindings::core_u_object::UObject>,
    pub member_name: FName,
    pub member_guid: crate::bindings::core_u_object::FGuid,
}
#[repr(C, align(8))]
pub struct FTickFunction {
    pub tick_group: ETickingGroup,
    pub end_tick_group: ETickingGroup,
    pub flags_10: u8,
    pub tick_interval: f32,
}
#[repr(C, align(8))]
pub struct FActorComponentTickFunction {}
#[repr(C, align(8))]
pub struct FInterpControlPoint {
    pub position_control_point: crate::bindings::core_u_object::FVector,
    pub b_position_is_relative: bool,
}
#[repr(C, align(8))]
pub struct FDebugFloatHistory {
    pub samples: TArray<f32>,
    pub max_samples: i32,
    pub min_value: f32,
    pub max_value: f32,
    pub b_auto_adjust_min_max: bool,
}
#[repr(C, align(8))]
pub struct FBaseComponentReference {
    pub component_property: FName,
    pub path_to_component: FString,
}
#[repr(C, align(8))]
pub struct FSoftComponentReference {
    pub other_actor: TSoftObjectPtr<AActor>,
}
#[repr(C, align(8))]
pub struct FComponentReference {
    pub other_actor: TWeakObjectPtr<AActor>,
}
#[repr(C, align(8))]
pub struct FLatentActionInfo {
    pub linkage: i32,
    pub uuid: i32,
    pub execution_function: FName,
    pub callback_target: UPtr<crate::bindings::core_u_object::UObject>,
}
#[repr(C, align(8))]
pub struct FTimerHandle {
    pub handle: u64,
}
#[repr(C, align(4))]
pub struct FCollisionProfileName {
    pub name: FName,
}
#[repr(C, align(4))]
pub struct FGenericStruct {
    pub data: i32,
}
#[repr(C, align(8))]
pub struct FUserActivity {
    pub action_name: FString,
}
#[repr(C, align(4))]
pub struct FFloatAnimationAttribute {
    pub value: f32,
}
#[repr(C, align(4))]
pub struct FIntegerAnimationAttribute {
    pub value: i32,
}
#[repr(C, align(8))]
pub struct FStringAnimationAttribute {
    pub value: FString,
}
#[repr(C, align(16))]
pub struct FTransformAnimationAttribute {
    pub value: crate::bindings::core_u_object::FTransform,
}
#[repr(C, align(8))]
pub struct FVectorAnimationAttribute {
    pub value: crate::bindings::core_u_object::FVector,
}
#[repr(C, align(16))]
pub struct FQuaternionAnimationAttribute {
    pub value: crate::bindings::core_u_object::FQuat,
}
#[repr(C, align(8))]
pub struct FBodyInstanceAsyncPhysicsTickHandle {}
#[repr(C, align(4))]
pub struct FWalkableSlopeOverride {
    pub walkable_slope_behavior: EWalkableSlopeBehavior,
    pub walkable_slope_angle: f32,
}
#[repr(C, align(8))]
pub struct FBodyInstance {
    pub position_solver_iteration_count: u8,
    pub velocity_solver_iteration_count: u8,
    pub projection_solver_iteration_count: u8,
    pub object_type: ECollisionChannel,
    pub collision_enabled: ECollisionEnabled,
    pub sleep_family: crate::bindings::physics_core::ESleepFamily,
    pub dof_mode: EDOFMode,
    pub flags_59: u8,
    pub flags_60: u8,
    pub flags_61: u8,
    pub flags_62: u8,
    pub solver_async_delta_time: f32,
    pub collision_profile_name: FName,
    pub response_to_channels_deprecated: FCollisionResponseContainer,
    pub collision_responses: FCollisionResponse,
    pub max_depenetration_velocity: f32,
    pub mass_in_kg_override: f32,
    pub linear_damping: f32,
    pub angular_damping: f32,
    pub custom_dof_plane_normal: crate::bindings::core_u_object::FVector,
    pub com_nudge: crate::bindings::core_u_object::FVector,
    pub mass_scale: f32,
    pub gravity_group_index: u8,
    pub inertia_tensor_scale: crate::bindings::core_u_object::FVector,
    pub walkable_slope_override: FWalkableSlopeOverride,
    pub phys_material_override: UPtr<crate::bindings::physics_core::UPhysicalMaterial>,
    pub max_angular_velocity: f32,
    pub custom_sleep_threshold_multiplier: f32,
    pub stabilization_threshold_multiplier: f32,
    pub physics_blend_weight: f32,
}
#[repr(C, align(8))]
pub struct FCollisionResponse {
    pub response_to_channels: FCollisionResponseContainer,
    pub response_array: TArray<FResponseChannel>,
}
#[repr(C, align(4))]
pub struct FResponseChannel {
    pub channel: FName,
    pub response: ECollisionResponse,
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
#[repr(C, align(8))]
pub struct FCustomPrimitiveData {
    pub data: TArray<f32>,
}
#[repr(C, align(1))]
pub struct FLightingChannels {
    pub flags_0: u8,
}
#[repr(C, align(8))]
pub struct FEdGraphSchemaAction {
    pub menu_description: FText,
    pub tooltip_description: FText,
    pub category: FText,
    pub keywords: FText,
    pub category_chain: TArray<FString>,
    pub grouping: i32,
    pub section_id: i32,
    pub menu_description_array: TArray<FString>,
    pub full_search_titles_array: TArray<FString>,
    pub full_search_keywords_array: TArray<FString>,
    pub full_search_category_array: TArray<FString>,
    pub localized_menu_description_array: TArray<FString>,
    pub localized_full_search_titles_array: TArray<FString>,
    pub localized_full_search_keywords_array: TArray<FString>,
    pub localized_full_search_category_array: TArray<FString>,
    pub search_text: FString,
}
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
    pub flags_20: u8,
    pub flags_24: u8,
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
    pub flags_40: u8,
    pub flags_44: u8,
    pub flags_48: u8,
    pub flags_49: u8,
    pub flags_50: u8,
    pub flags_52: u8,
    pub bloom_method: EBloomMethod,
    pub auto_exposure_method: EAutoExposureMethod,
    pub depth_of_field_method_deprecated: EDepthOfFieldMethod,
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
    pub bloom_convolution_pre_filter_deprecated: crate::bindings::core_u_object::FVector3f,
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
    pub reflections_type_deprecated: EReflectionsType,
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
    pub auto_exposure_bias_backup: f32,
    pub flags_1212: u8,
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
    pub auto_exposure_calibration_constant_deprecated: f32,
    pub local_exposure_method: ELocalExposureMethod,
    pub local_exposure_contrast_scale_deprecated: f32,
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
    pub lens_flare_tints: crate::bindings::core_u_object::FLinearColor,
    pub vignette_intensity: f32,
    pub sharpen: f32,
    pub grain_jitter_deprecated: f32,
    pub grain_intensity_deprecated: f32,
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
    pub ambient_occlusion_distance_deprecated: f32,
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
    pub depth_of_field_matte_box_flags: FMatteBoxFlag,
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
    pub lpv_intensity_deprecated: f32,
    pub lpv_vpl_injection_bias_deprecated: f32,
    pub lpv_size_deprecated: f32,
    pub lpv_secondary_occlusion_intensity_deprecated: f32,
    pub lpv_secondary_bounce_intensity_deprecated: f32,
    pub lpv_geometry_volume_bias_deprecated: f32,
    pub lpv_emissive_injection_intensity_deprecated: f32,
    pub lpv_directional_occlusion_intensity_deprecated: f32,
    pub lpv_directional_occlusion_radius_deprecated: f32,
    pub lpv_diffuse_occlusion_exponent_deprecated: f32,
    pub lpv_specular_occlusion_exponent_deprecated: f32,
    pub lpv_diffuse_occlusion_intensity_deprecated: f32,
    pub lpv_specular_occlusion_intensity_deprecated: f32,
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
    pub lpv_fade_range_deprecated: f32,
    pub lpv_directional_occlusion_fade_range_deprecated: f32,
    pub screen_percentage_deprecated: f32,
    pub user_flags: i32,
    pub weighted_blendables: FWeightedBlendables,
    pub preview_blendable: UPtr<crate::bindings::core_u_object::UObject>,
    pub blendables_deprecated: TArray<UPtr<crate::bindings::core_u_object::UObject>>,
}
#[repr(C, align(8))]
pub struct FWeightedBlendables {
    pub array: TArray<FWeightedBlendable>,
}
#[repr(C, align(8))]
pub struct FWeightedBlendable {
    pub weight: f32,
    pub object: UPtr<crate::bindings::core_u_object::UObject>,
}
#[repr(C, align(4))]
pub struct FMatteBoxFlag {
    pub pitch: f32,
    pub roll: f32,
    pub length: f32,
}
#[repr(C, align(8))]
pub struct FTableRowBase {}
#[repr(C, align(8))]
pub struct FInstanceCacheDataBase {
    pub saved_properties: TArray<u8>,
    pub unique_transient_package: FDataCacheDuplicatedObjectData,
    pub duplicated_objects: TArray<FDataCacheDuplicatedObjectData>,
    pub referenced_objects: TArray<UPtr<crate::bindings::core_u_object::UObject>>,
    pub referenced_names: TArray<FName>,
}
#[repr(C, align(8))]
pub struct FDataCacheDuplicatedObjectData {}
#[repr(C, align(8))]
pub struct FActorComponentInstanceData {
    pub source_component_template: UPtr<crate::bindings::core_u_object::UObject>,
    pub source_component_creation_method: EComponentCreationMethod,
    pub source_component_type_serialized_index: i32,
}
#[repr(C, align(8))]
pub struct FNavLinkAuxiliaryId {
    pub id: u64,
}
#[repr(C, align(8))]
pub struct FNavLinkId {
    pub id: u64,
}
#[repr(C, align(8))]
pub struct FKeyHandleLookupTable {}
#[repr(C, align(8))]
pub struct FRuntimeFloatCurve {
    pub editor_curve_data: FRichCurve,
    pub external_curve: UPtr<UCurveFloat>,
}
#[repr(C, align(8))]
pub struct FIndexedCurve {
    pub key_handles_to_indices: FKeyHandleMap,
}
#[repr(C, align(8))]
pub struct FKeyHandleMap {}
#[repr(C, align(8))]
pub struct FRealCurve {
    pub default_value: f32,
    pub pre_infinity_extrap: ERichCurveExtrapolation,
    pub post_infinity_extrap: ERichCurveExtrapolation,
}
#[repr(C, align(8))]
pub struct FRichCurve {
    pub keys: TArray<FRichCurveKey>,
}
#[repr(C, align(4))]
pub struct FRichCurveKey {
    pub interp_mode: ERichCurveInterpMode,
    pub tangent_mode: ERichCurveTangentMode,
    pub tangent_weight_mode: ERichCurveTangentWeightMode,
    pub time: f32,
    pub value: f32,
    pub arrive_tangent: f32,
    pub arrive_tangent_weight: f32,
    pub leave_tangent: f32,
    pub leave_tangent_weight: f32,
}
#[repr(C, align(8))]
pub struct FSceneComponentInstanceData {
    pub attached_instance_components: TMap<
        UPtr<USceneComponent>,
        crate::bindings::core_u_object::FTransform,
    >,
}
#[repr(C, align(8))]
pub struct FEdGraphPinType {
    pub pin_category: FName,
    pub pin_sub_category: FName,
    pub pin_sub_category_object: TWeakObjectPtr<crate::bindings::core_u_object::UObject>,
    pub pin_sub_category_member_reference: FSimpleMemberReference,
    pub pin_value_type: FEdGraphTerminalType,
    pub container_type: EPinContainerType,
    pub flags_109: u8,
}
#[repr(C, align(4))]
pub struct FEdGraphTerminalType {
    pub terminal_category: FName,
    pub terminal_sub_category: FName,
    pub terminal_sub_category_object: TWeakObjectPtr<
        crate::bindings::core_u_object::UObject,
    >,
    pub b_terminal_is_const: bool,
    pub b_terminal_is_weak_pointer: bool,
    pub b_terminal_is_u_object_wrapper: bool,
}
#[repr(C, align(16))]
pub struct FAnimInstanceProxy {
    pub slot_group_inertialization_request_data_map: TMap<
        FName,
        FInertializationRequest,
    >,
}
#[repr(C, align(8))]
pub struct FInertializationRequest {
    pub duration: f32,
    pub blend_profile: UPtr<UBlendProfile>,
    pub b_use_blend_mode: bool,
    pub blend_mode: EAlphaBlendOption,
    pub custom_blend_curve: UPtr<UCurveFloat>,
    pub tag: FName,
    pub description_deprecated: FText,
    pub description_string: FString,
    pub node_id: i32,
    pub anim_instance: UPtr<crate::bindings::core_u_object::UObject>,
}
#[repr(C, align(16))]
pub struct FAnimSingleNodeInstanceProxy {}
#[repr(C, align(8))]
pub struct FAnimNode_Base {
    pub initial_update_function: FAnimNodeFunctionRef,
    pub become_relevant_function: FAnimNodeFunctionRef,
    pub update_function: FAnimNodeFunctionRef,
}
#[repr(C, align(8))]
pub struct FAnimNodeFunctionRef {
    pub class_name: FName,
    pub function_name: FName,
    pub class: TSubclassOf<crate::bindings::core_u_object::UObject>,
    pub function: UPtr<crate::bindings::core_u_object::UFunction>,
}
#[repr(C, align(8))]
pub struct FPoseLinkBase {
    pub link_id: i32,
    pub source_link_id: i32,
}
#[repr(C, align(8))]
pub struct FPoseLink {}
#[repr(C, align(8))]
pub struct FAnimNode_Root {
    pub result: FPoseLink,
    pub name: FName,
    pub layer_group: FName,
    pub group_deprecated: FName,
}
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
}
#[repr(C, align(4))]
pub struct FInputRange {
    pub min: f32,
    pub max: f32,
}
#[repr(C, align(8))]
pub struct FInputAlphaBoolBlend {
    pub blend_in_time: f32,
    pub blend_out_time: f32,
    pub blend_option: EAlphaBlendOption,
    pub b_initialized: bool,
    pub custom_curve: UPtr<UCurveFloat>,
    pub alpha_blend: FAlphaBlend,
}
#[repr(C, align(8))]
pub struct FAlphaBlend {
    pub custom_curve: UPtr<UCurveFloat>,
    pub blend_time: f32,
    pub blend_option: EAlphaBlendOption,
}
#[repr(C, align(4))]
pub struct FInputScaleBias {
    pub scale: f32,
    pub bias: f32,
}
#[repr(C, align(8))]
pub struct FComponentSpacePoseLink {}
#[repr(C, align(4))]
pub struct FBoneReference {
    pub bone_name: FName,
}
#[repr(C, align(8))]
pub struct FAnimNodeReference {}
#[repr(C, align(8))]
pub struct FAnimNode_AssetPlayerRelevancyBase {}
#[repr(C, align(8))]
pub struct FAnimNode_AssetPlayerBase {
    pub group_index_deprecated: i32,
    pub group_scope_deprecated: EAnimSyncGroupScope,
    pub blend_weight: f32,
    pub internal_time_accumulator: f32,
}
#[repr(C, align(4))]
pub struct FPerBoneBlendWeight {
    pub source_index: i32,
    pub blend_weight: f32,
}
#[repr(C, align(8))]
pub struct FInputBlendPose {
    pub branch_filters: TArray<FBranchFilter>,
}
#[repr(C, align(4))]
pub struct FBranchFilter {
    pub bone_name: FName,
    pub blend_depth: i32,
}
#[repr(C, align(8))]
pub struct FPoseSnapshot {
    pub local_transforms: TArray<crate::bindings::core_u_object::FTransform>,
    pub bone_names: TArray<FName>,
    pub skeletal_mesh_name: FName,
    pub snapshot_name: FName,
    pub b_is_valid: bool,
}
#[repr(C, align(16))]
pub struct FBoneSocketTarget {
    pub b_use_socket: bool,
    pub bone_reference: FBoneReference,
    pub socket_reference: FSocketReference,
}
#[repr(C, align(16))]
pub struct FSocketReference {
    pub socket_name: FName,
}
#[repr(C, align(4))]
pub struct FAnimCurveParam {
    pub name: FName,
}
#[repr(C, align(4))]
pub struct FMaterialParameterInfo {
    pub name: FName,
    pub association: EMaterialParameterAssociation,
    pub index: i32,
}
#[repr(C, align(8))]
pub struct FParameterChannelNames {
    pub r: FText,
    pub g: FText,
    pub b: FText,
    pub a: FText,
}
#[repr(C, align(4))]
pub struct FActorDataLayer {
    pub name: FName,
}
#[repr(C, align(8))]
pub struct FRepMovement {
    pub linear_velocity: crate::bindings::core_u_object::FVector,
    pub angular_velocity: crate::bindings::core_u_object::FVector,
    pub location: crate::bindings::core_u_object::FVector,
    pub rotation: crate::bindings::core_u_object::FRotator,
    pub acceleration: crate::bindings::core_u_object::FVector,
    pub flags_120: u8,
    pub server_frame: i32,
    pub server_physics_handle: i32,
    pub location_quantization_level: EVectorQuantization,
    pub velocity_quantization_level: EVectorQuantization,
    pub rotation_quantization_level: ERotatorQuantization,
}
#[repr(C, align(8))]
pub struct FRepAttachment {
    pub attach_parent: UPtr<AActor>,
    pub location_offset: FVector_NetQuantize100,
    pub relative_scale3_d: FVector_NetQuantize100,
    pub rotation_offset: crate::bindings::core_u_object::FRotator,
    pub attach_socket: FName,
    pub attach_component: UPtr<USceneComponent>,
}
#[repr(C, align(8))]
pub struct FVector_NetQuantize100 {}
#[repr(C, align(8))]
pub struct FActorTickFunction {}
#[repr(C, align(4))]
pub struct FWorldPartitionResolveData {
    pub container_id: FActorContainerID,
    pub source_world_asset_path: crate::bindings::core_u_object::FTopLevelAssetPath,
}
#[repr(C, align(4))]
pub struct FActorContainerID {
    pub guid: crate::bindings::core_u_object::FGuid,
}
#[repr(C, align(4))]
pub struct FExposureSettings {
    pub fixed_ev100: f32,
    pub b_fixed: bool,
}
#[repr(C, align(8))]
pub struct FLevelViewportInfo {
    pub cam_position: crate::bindings::core_u_object::FVector,
    pub cam_rotation: crate::bindings::core_u_object::FRotator,
    pub cam_ortho_zoom: f32,
    pub cam_updated: bool,
}
#[repr(C, align(8))]
pub struct FPerQualityLevelInt {
    pub default: i32,
    pub per_quality: TMap<i32, i32>,
}
#[repr(C, align(8))]
pub struct FPerQualityLevelFloat {
    pub default: f32,
    pub per_quality: TMap<i32, f32>,
}
#[repr(C, align(8))]
pub struct FKAggregateGeom {
    pub sphere_elems: TArray<FKSphereElem>,
    pub box_elems: TArray<FKBoxElem>,
    pub sphyl_elems: TArray<FKSphylElem>,
    pub convex_elems: TArray<FKConvexElem>,
    pub tapered_capsule_elems: TArray<FKTaperedCapsuleElem>,
    pub level_set_elems: TArray<FKLevelSetElem>,
    pub skinned_level_set_elems: TArray<FKSkinnedLevelSetElem>,
    pub ml_level_set_elems: TArray<FKMLLevelSetElem>,
    pub skinned_triangle_mesh_elems: TArray<FKSkinnedTriangleMeshElem>,
}
#[repr(C, align(8))]
pub struct FKShapeElem {
    pub rest_offset: f32,
    pub flags_12: u8,
    pub name: FName,
    pub flags_32: u8,
    pub collision_enabled: ECollisionEnabled,
}
#[repr(C, align(8))]
pub struct FKSkinnedTriangleMeshElem {}
#[repr(C, align(8))]
pub struct FKMLLevelSetElem {
    pub nne_signed_distance_model_data: UPtr<crate::bindings::nne::UNNEModelData>,
    pub nne_incorrect_zone_model_data: UPtr<crate::bindings::nne::UNNEModelData>,
}
#[repr(C, align(8))]
pub struct FKSkinnedLevelSetElem {}
#[repr(C, align(16))]
pub struct FKLevelSetElem {
    pub transform: crate::bindings::core_u_object::FTransform,
}
#[repr(C, align(8))]
pub struct FKTaperedCapsuleElem {
    pub center: crate::bindings::core_u_object::FVector,
    pub rotation: crate::bindings::core_u_object::FRotator,
    pub radius0: f32,
    pub radius1: f32,
    pub length: f32,
    pub b_one_sided_collision: bool,
}
#[repr(C, align(16))]
pub struct FKConvexElem {
    pub vertex_data: TArray<crate::bindings::core_u_object::FVector>,
    pub index_data: TArray<i32>,
    pub elem_box: crate::bindings::core_u_object::FBox,
    pub transform: crate::bindings::core_u_object::FTransform,
}
#[repr(C, align(16))]
pub struct FKSphylElem {
    pub tm_deprecated: crate::bindings::core_u_object::FMatrix,
    pub orientation_deprecated: crate::bindings::core_u_object::FQuat,
    pub center: crate::bindings::core_u_object::FVector,
    pub rotation: crate::bindings::core_u_object::FRotator,
    pub radius: f32,
    pub length: f32,
}
#[repr(C, align(16))]
pub struct FKBoxElem {
    pub tm_deprecated: crate::bindings::core_u_object::FMatrix,
    pub orientation_deprecated: crate::bindings::core_u_object::FQuat,
    pub center: crate::bindings::core_u_object::FVector,
    pub rotation: crate::bindings::core_u_object::FRotator,
    pub x: f32,
    pub y: f32,
    pub z: f32,
}
#[repr(C, align(16))]
pub struct FKSphereElem {
    pub tm_deprecated: crate::bindings::core_u_object::FMatrix,
    pub center: crate::bindings::core_u_object::FVector,
    pub radius: f32,
}
#[repr(C, align(4))]
pub struct FNamedFloat {
    pub value: f32,
    pub name: FName,
}
#[repr(C, align(8))]
pub struct FNamedVector {
    pub value: crate::bindings::core_u_object::FVector,
    pub name: FName,
}
#[repr(C, align(4))]
pub struct FNamedColor {
    pub value: crate::bindings::core_u_object::FColor,
    pub name: FName,
}
#[repr(C, align(16))]
pub struct FNamedTransform {
    pub value: crate::bindings::core_u_object::FTransform,
    pub name: FName,
}
#[repr(C, align(8))]
pub struct FLocalSpacePose {
    pub transforms: TArray<crate::bindings::core_u_object::FTransform>,
    pub names: TArray<FName>,
}
#[repr(C, align(8))]
pub struct FComponentSpacePose {
    pub transforms: TArray<crate::bindings::core_u_object::FTransform>,
    pub names: TArray<FName>,
}
#[repr(C, align(1))]
pub struct FAnimCurveType {
    pub b_material: bool,
    pub b_morphtarget: bool,
}
#[repr(C, align(8))]
pub struct FCurveMetaData {
    pub linked_bones: TArray<FBoneReference>,
    pub max_lod: u8,
    pub ty: FAnimCurveType,
}
#[repr(C, align(1))]
pub struct FEmptyPayload {}
#[repr(C, align(8))]
pub struct FBracketPayload {
    pub description: FString,
}
#[repr(C, align(4))]
pub struct FAnimationTrackPayload {
    pub name: FName,
}
#[repr(C, align(4))]
pub struct FAnimationTrackAddedPayload {
    pub track_index: i32,
}
#[repr(C, align(4))]
pub struct FSequenceLengthChangedPayload {
    pub previous_length: f32,
    pub t0: f32,
    pub t1: f32,
    pub previous_number_of_frames: crate::bindings::core_u_object::FFrameNumber,
    pub frame0: crate::bindings::core_u_object::FFrameNumber,
    pub frame1: crate::bindings::core_u_object::FFrameNumber,
}
#[repr(C, align(4))]
pub struct FFrameRateChangedPayload {
    pub previous_frame_rate: crate::bindings::core_u_object::FFrameRate,
}
#[repr(C, align(4))]
pub struct FCurvePayload {
    pub identifier: FAnimationCurveIdentifier,
}
#[repr(C, align(4))]
pub struct FAnimationCurveIdentifier {
    pub internal_name_deprecated: FSmartName,
    pub curve_name: FName,
    pub curve_type: ERawCurveTrackTypes,
    pub channel: ETransformCurveChannel,
    pub axis: EVectorCurveChannel,
}
#[repr(C, align(4))]
pub struct FSmartName {
    pub display_name: FName,
}
#[repr(C, align(4))]
pub struct FCurveScaledPayload {
    pub factor: f32,
    pub origin: f32,
}
#[repr(C, align(4))]
pub struct FCurveRenamedPayload {
    pub new_identifier: FAnimationCurveIdentifier,
}
#[repr(C, align(4))]
pub struct FCurveFlagsChangedPayload {
    pub old_flags: i32,
}
#[repr(C, align(8))]
pub struct FAttributePayload {
    pub identifier: FAnimationAttributeIdentifier,
}
#[repr(C, align(8))]
pub struct FAnimationAttributeIdentifier {
    pub name: FName,
    pub bone_name: FName,
    pub bone_index: i32,
    pub script_struct: UPtr<crate::bindings::core_u_object::UScriptStruct>,
    pub script_struct_path: crate::bindings::core_u_object::FSoftObjectPath,
}
#[repr(C, align(8))]
pub struct FAnimNodeConstantData {
    pub anim_class_interface: TScriptInterface<IAnimClassInterface>,
    pub node_index: i32,
}
#[repr(C, align(8))]
pub struct FSlotEvaluationPose {
    pub additive_type: EAdditiveAnimationType,
    pub weight: f32,
}
#[repr(C, align(8))]
pub struct FAnimSubsystem {}
#[repr(C, align(8))]
pub struct FAnimSubsystemInstance {}
#[repr(C, align(8))]
pub struct FAnimSubsystem_BlendSpaceGraph {
    pub blend_spaces: TArray<UPtr<UBlendSpace>>,
}
#[repr(C, align(8))]
pub struct FAssetManagerRedirect {
    pub old: FString,
    pub new: FString,
}
#[repr(C, align(4))]
pub struct FPrimaryAssetRulesOverride {
    pub primary_asset_id: crate::bindings::core_u_object::FPrimaryAssetId,
    pub rules: FPrimaryAssetRules,
}
#[repr(C, align(4))]
pub struct FPrimaryAssetRules {
    pub priority: i32,
    pub chunk_id: i32,
    pub b_apply_recursively: bool,
    pub cook_rule: EPrimaryAssetCookRule,
}
#[repr(C, align(8))]
pub struct FPrimaryAssetRulesCustomOverride {
    pub primary_asset_type: crate::bindings::core_u_object::FPrimaryAssetType,
    pub filter_directory: crate::bindings::core_u_object::FDirectoryPath,
    pub filter_string: FString,
    pub rules: FPrimaryAssetRules,
}
#[repr(C, align(1))]
pub struct FBookmark2DJumpToSettings {}
#[repr(C, align(1))]
pub struct FBookmarkBaseJumpToSettings {}
#[repr(C, align(16))]
pub struct FNonBlendableQuaternionAnimationAttribute {}
#[repr(C, align(8))]
pub struct FNonBlendableVectorAnimationAttribute {}
#[repr(C, align(16))]
pub struct FNonBlendableTransformAnimationAttribute {}
#[repr(C, align(4))]
pub struct FNonBlendableFloatAnimationAttribute {}
#[repr(C, align(4))]
pub struct FNonBlendableIntegerAnimationAttribute {}
#[repr(C, align(16))]
pub struct FMinimalViewInfo {
    pub location: crate::bindings::core_u_object::FVector,
    pub rotation: crate::bindings::core_u_object::FRotator,
    pub fov: f32,
    pub desired_fov: f32,
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
    pub flags_104: u8,
    pub projection_mode: ECameraProjectionMode,
    pub post_process_blend_weight: f32,
    pub post_process_settings: FPostProcessSettings,
    pub off_center_projection_offset: crate::bindings::core_u_object::FVector2D,
    pub overscan_resolution_fraction: f32,
    pub crop_fraction: f32,
    pub asymmetric_crop_fraction: crate::bindings::core_u_object::FVector4f,
}
#[repr(C, align(8))]
pub struct FTextSizingParameters {
    pub draw_x: f32,
    pub draw_y: f32,
    pub draw_xl: f32,
    pub draw_yl: f32,
    pub scaling: crate::bindings::core_u_object::FVector2D,
    pub draw_font: UPtr<UFont>,
    pub spacing_adjust: crate::bindings::core_u_object::FVector2D,
}
#[repr(C, align(8))]
pub struct FWrappedStringElement {
    pub value: FString,
    pub line_extent: crate::bindings::core_u_object::FVector2D,
}
#[repr(C, align(8))]
pub struct FCharacterNetworkSerializationPackedBits {}
#[repr(C, align(8))]
pub struct FCharacterServerMovePackedBits {}
#[repr(C, align(8))]
pub struct FCharacterMoveResponsePackedBits {}
#[repr(C, align(8))]
pub struct FCustomAttributeSetting {
    pub name: FString,
    pub meaning: FString,
}
#[repr(C, align(4))]
pub struct FTimecodeCustomAttributeNameSettings {
    pub hour_attribute_name: FName,
    pub minute_attribute_name: FName,
    pub second_attribute_name: FName,
    pub frame_attribute_name: FName,
    pub subframe_attribute_name: FName,
    pub rate_attribute_name: FName,
    pub takename_attribute_name: FName,
}
#[repr(C, align(8))]
pub struct FCustomAttribute {
    pub name: FName,
    pub variant_type: i32,
    pub times: TArray<f32>,
}
#[repr(C, align(8))]
pub struct FCustomAttributePerBoneData {
    pub bone_tree_index: i32,
    pub attributes: TArray<FCustomAttribute>,
}
#[repr(C, align(8))]
pub struct FBakedStringCustomAttribute {
    pub attribute_name: FName,
    pub string_curve: FStringCurve,
}
#[repr(C, align(8))]
pub struct FStringCurve {
    pub default_value: FString,
    pub keys: TArray<FStringCurveKey>,
}
#[repr(C, align(8))]
pub struct FStringCurveKey {
    pub time: f32,
    pub value: FString,
}
#[repr(C, align(8))]
pub struct FBakedIntegerCustomAttribute {
    pub attribute_name: FName,
    pub int_curve: FIntegralCurve,
}
#[repr(C, align(8))]
pub struct FIntegralCurve {
    pub keys: TArray<FIntegralKey>,
    pub default_value: i32,
    pub b_use_default_value_before_first_key: bool,
}
#[repr(C, align(4))]
pub struct FIntegralKey {
    pub time: f32,
    pub value: i32,
}
#[repr(C, align(8))]
pub struct FBakedFloatCustomAttribute {
    pub attribute_name: FName,
    pub float_curve: FSimpleCurve,
}
#[repr(C, align(8))]
pub struct FSimpleCurve {
    pub interp_mode: ERichCurveInterpMode,
    pub keys: TArray<FSimpleCurveKey>,
}
#[repr(C, align(4))]
pub struct FSimpleCurveKey {
    pub time: f32,
    pub value: f32,
}
#[repr(C, align(8))]
pub struct FBakedCustomAttributePerBoneData {
    pub bone_tree_index: i32,
    pub string_attributes: TArray<FBakedStringCustomAttribute>,
    pub int_attributes: TArray<FBakedIntegerCustomAttribute>,
    pub float_attributes: TArray<FBakedFloatCustomAttribute>,
}
#[repr(C, align(8))]
pub struct FDataLayerInstanceNames {
    pub b_is_first_data_layer_external: bool,
    pub data_layers: TArray<FName>,
}
#[repr(C, align(8))]
pub struct FDebugDisplayProperty {
    pub obj: UPtr<crate::bindings::core_u_object::UObject>,
    pub within_class: TSubclassOf<crate::bindings::core_u_object::UObject>,
}
#[repr(C, align(8))]
pub struct FDebugTextInfo {
    pub src_actor: UPtr<AActor>,
    pub src_actor_offset: crate::bindings::core_u_object::FVector,
    pub src_actor_desired_offset: crate::bindings::core_u_object::FVector,
    pub debug_text: FString,
    pub time_remaining: f32,
    pub duration: f32,
    pub text_color: crate::bindings::core_u_object::FColor,
    pub flags_84: u8,
    pub orig_actor_location: crate::bindings::core_u_object::FVector,
    pub font: UPtr<UFont>,
    pub font_scale: f32,
}
#[repr(C, align(8))]
pub struct FRawDistributionFloat {
    pub min_value: f32,
    pub max_value: f32,
    pub distribution: UPtr<UDistributionFloat>,
}
#[repr(C, align(8))]
pub struct FRawDistributionVector {
    pub min_value: f32,
    pub max_value: f32,
    pub min_value_vec: crate::bindings::core_u_object::FVector,
    pub max_value_vec: crate::bindings::core_u_object::FVector,
    pub distribution: UPtr<UDistributionVector>,
}
#[repr(C, align(8))]
pub struct FFullyLoadedPackagesInfo {
    pub fully_load_type: EFullyLoadPackageType,
    pub tag: FString,
    pub packages_to_load: TArray<FName>,
    pub loaded_objects: TArray<UPtr<crate::bindings::core_u_object::UObject>>,
}
#[repr(C, align(4))]
pub struct FLevelStreamingStatus {
    pub package_name: FName,
    pub flags_12: u8,
    pub lod_index: u32,
}
#[repr(C, align(4))]
pub struct FNetDriverDefinition {
    pub def_name: FName,
    pub driver_class_name: FName,
    pub driver_class_name_fallback: FName,
    pub max_channels_override: i32,
}
#[repr(C, align(8))]
pub struct FIrisNetDriverConfig {
    pub net_driver_definition: FName,
    pub net_driver_name: FName,
    pub net_driver_wildcard_name: FString,
    pub b_can_use_iris: bool,
}
#[repr(C, align(8))]
pub struct FNamedNetDriver {
    pub net_driver: UPtr<UNetDriver>,
}
#[repr(C, align(8))]
pub struct FWorldContext {
    pub last_url: FURL,
    pub last_remote_url: FURL,
    pub pending_net_game: UPtr<UPendingNetGame>,
    pub packages_to_fully_load: TArray<FFullyLoadedPackagesInfo>,
    pub loaded_levels_for_pending_map_change: TArray<UPtr<ULevel>>,
    pub object_referencers: TArray<UPtr<UObjectReferencer>>,
    pub pending_level_streaming_status_updates: TArray<FLevelStreamingStatus>,
    pub game_viewport: UPtr<UGameViewportClient>,
    pub owning_game_instance: UPtr<UGameInstance>,
    pub active_net_drivers: TArray<FNamedNetDriver>,
}
#[repr(C, align(8))]
pub struct FURL {
    pub protocol: FString,
    pub host: FString,
    pub port: i32,
    pub valid: i32,
    pub map: FString,
    pub redirect_url: FString,
    pub op: TArray<FString>,
    pub portal: FString,
}
#[repr(C, align(4))]
pub struct FStatColorMapEntry {
    pub in_: f32,
    pub out: crate::bindings::core_u_object::FColor,
}
#[repr(C, align(8))]
pub struct FStatColorMapping {
    pub stat_name: FString,
    pub color_map: TArray<FStatColorMapEntry>,
    pub flags_32: u8,
}
#[repr(C, align(8))]
pub struct FDropNoteInfo {
    pub location: crate::bindings::core_u_object::FVector,
    pub rotation: crate::bindings::core_u_object::FRotator,
    pub comment: FString,
}
#[repr(C, align(8))]
pub struct FScreenMessageString {
    pub key: u64,
    pub screen_message: FString,
    pub display_color: crate::bindings::core_u_object::FColor,
    pub time_to_display: f32,
    pub current_time_displayed: f32,
    pub text_scale: crate::bindings::core_u_object::FVector2D,
}
#[repr(C, align(4))]
pub struct FGameNameRedirect {
    pub old_game_name: FName,
    pub new_game_name: FName,
}
#[repr(C, align(4))]
pub struct FClassRedirect {
    pub object_name: FName,
    pub old_class_name: FName,
    pub new_class_name: FName,
    pub old_subobj_name: FName,
    pub new_subobj_name: FName,
    pub new_class_class: FName,
    pub new_class_package: FName,
    pub instance_only: bool,
}
#[repr(C, align(4))]
pub struct FStructRedirect {
    pub old_struct_name: FName,
    pub new_struct_name: FName,
}
#[repr(C, align(8))]
pub struct FPluginRedirect {
    pub old_plugin_name: FString,
    pub new_plugin_name: FString,
}
#[repr(C, align(8))]
pub struct FTickPrerequisite {}
#[repr(C, align(4))]
pub struct FExponentialHeightFogData {
    pub fog_density: f32,
    pub fog_height_falloff: f32,
    pub fog_height_offset: f32,
}
#[repr(C, align(4))]
pub struct FExponentialHeightFogStaticState {}
#[repr(C, align(8))]
pub struct FExponentialHeightFogDynamicState {
    pub fog_density: f32,
    pub fog_height_falloff: f32,
    pub second_fog_data: FExponentialHeightFogData,
    pub fog_inscattering_luminance: crate::bindings::core_u_object::FLinearColor,
    pub sky_atmosphere_ambient_contribution_color_scale: crate::bindings::core_u_object::FLinearColor,
    pub inscattering_color_cubemap: UPtr<UTextureCube>,
    pub inscattering_color_cubemap_angle: f32,
    pub inscattering_texture_tint: crate::bindings::core_u_object::FLinearColor,
    pub fully_directional_inscattering_color_distance: f32,
    pub non_directional_inscattering_color_distance: f32,
    pub directional_inscattering_exponent: f32,
    pub directional_inscattering_start_distance: f32,
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
    pub flags_181: u8,
    pub height: f32,
}
#[repr(C, align(8))]
pub struct FExponentialHeightFogHandle {}
#[repr(C, align(8))]
pub struct FFontImportOptionsData {
    pub font_name: FString,
    pub height: f32,
    pub flags_20: u8,
    pub character_set: EFontImportCharacterSet,
    pub chars: FString,
    pub unicode_range: FString,
    pub chars_file_path: FString,
    pub chars_file_wildcard: FString,
    pub flags_96: u8,
    pub foreground_color: crate::bindings::core_u_object::FLinearColor,
    pub flags_116: u8,
    pub texture_page_width: i32,
    pub texture_page_max_height: i32,
    pub x_padding: i32,
    pub y_padding: i32,
    pub extend_box_top: i32,
    pub extend_box_bottom: i32,
    pub extend_box_right: i32,
    pub extend_box_left: i32,
    pub flags_152: u8,
    pub kerning: i32,
    pub flags_160: u8,
    pub distance_field_scale_factor: i32,
    pub distance_field_scan_radius_scale: f32,
}
#[repr(C, align(8))]
pub struct FBaseAttenuationSettings {
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
#[repr(C, align(8))]
pub struct FForceFeedbackAttenuationSettings {}
#[repr(C, align(4))]
pub struct FForceFeedbackParameters {
    pub tag: FName,
    pub b_looping: bool,
    pub b_ignore_time_dilation: bool,
    pub b_play_while_paused: bool,
}
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
}
#[repr(C, align(8))]
pub struct FPredictProjectilePathPointData {
    pub location: crate::bindings::core_u_object::FVector,
    pub velocity: crate::bindings::core_u_object::FVector,
    pub time: f32,
}
#[repr(C, align(8))]
pub struct FPredictProjectilePathResult {
    pub path_data: TArray<FPredictProjectilePathPointData>,
    pub last_trace_destination: FPredictProjectilePathPointData,
    pub hit_result: FHitResult,
}
#[repr(C, align(8))]
pub struct FActiveHapticFeedbackEffect {
    pub haptic_effect: UPtr<UHapticFeedbackEffect_Base>,
}
#[repr(C, align(8))]
pub struct FHapticFeedbackDetails_Curve {
    pub frequency: FRuntimeFloatCurve,
    pub amplitude: FRuntimeFloatCurve,
}
#[repr(C, align(4))]
pub struct FClusterNode_DEPRECATED {
    pub bound_min: crate::bindings::core_u_object::FVector3f,
    pub first_child: i32,
    pub bound_max: crate::bindings::core_u_object::FVector3f,
    pub last_child: i32,
    pub first_instance: i32,
    pub last_instance: i32,
}
#[repr(C, align(4))]
pub struct FClusterNode {
    pub bound_min: crate::bindings::core_u_object::FVector3f,
    pub first_child: i32,
    pub bound_max: crate::bindings::core_u_object::FVector3f,
    pub last_child: i32,
    pub first_instance: i32,
    pub last_instance: i32,
    pub min_instance_scale: crate::bindings::core_u_object::FVector3f,
    pub max_instance_scale: crate::bindings::core_u_object::FVector3f,
}
#[repr(C, align(4))]
pub struct FPrimitiveInstanceId {}
#[repr(C, align(16))]
pub struct FInstancedStaticMeshInstanceData {
    pub transform: crate::bindings::core_u_object::FMatrix,
}
#[repr(C, align(8))]
pub struct FInstancedStaticMeshMappingInfo {}
#[repr(C, align(4))]
pub struct FInstancedStaticMeshRandomSeed {
    pub start_instance_index: i32,
    pub random_seed: i32,
}
#[repr(C, align(8))]
pub struct FBoundsCacheElement {
    pub b_is_valid: bool,
    pub hash: u32,
    pub value: crate::bindings::core_u_object::FBoxSphereBounds,
}
#[repr(C, align(16))]
pub struct FInstancedStaticMeshLightMapInstanceData {
    pub transform: crate::bindings::core_u_object::FTransform,
    pub map_build_data_ids: TArray<crate::bindings::core_u_object::FGuid>,
}
#[repr(C, align(16))]
pub struct FInstancedStaticMeshComponentInstanceData {
    pub static_mesh: UPtr<UStaticMesh>,
    pub cached_static_lighting: FInstancedStaticMeshLightMapInstanceData,
    pub per_instance_sm_data: TArray<FInstancedStaticMeshInstanceData>,
    pub instancing_random_seed: i32,
    pub additional_random_seeds: TArray<FInstancedStaticMeshRandomSeed>,
    pub b_has_per_instance_hit_proxies: bool,
}
#[repr(C, align(8))]
pub struct FUniqueNetIdReplNetSerializerStringStruct {
    pub string: FString,
}
#[repr(C, align(4))]
pub struct FUniqueNetIdReplNetSerializerNameStruct {
    pub name: FName,
}
#[repr(C, align(4))]
pub struct FISMClientHandle {
    pub index: i32,
    pub guid: crate::bindings::core_u_object::FGuid,
}
#[repr(C, align(8))]
pub struct FISMClientInstanceManagerData {}
#[repr(C, align(8))]
pub struct FNewLevelInstanceParams {
    pub ty: ELevelInstanceCreationType,
    pub pivot_type: ELevelInstancePivotType,
    pub pivot_actor: UPtr<AActor>,
    pub b_always_show_dialog: bool,
    pub template_world: UPtr<UWorld>,
    pub level_package_name: FString,
    pub b_prompt_for_save: bool,
    pub level_instance_class: TSubclassOf<AActor>,
    pub b_enable_streaming: bool,
    pub b_external_actors: bool,
    pub b_force_external_actors: bool,
    pub b_hide_creation_type: bool,
}
#[repr(C, align(8))]
pub struct FMaterialCacheLayer {
    pub render_format: crate::bindings::core_u_object::EPixelFormat,
    pub compressed_format: crate::bindings::core_u_object::EPixelFormat,
    pub component_count: u8,
    pub b_is_srgb: bool,
    pub identity: EMaterialCacheAttributeIdentity,
    pub attributes: TArray<EMaterialCacheAttribute>,
}
#[repr(C, align(8))]
pub struct FMaterialExpressionCollection {
    pub expressions: TArray<UPtr<UMaterialExpression>>,
    pub editor_comments: TArray<UPtr<UMaterialExpressionComment>>,
    pub expression_exec_begin: UPtr<UMaterialExpression>,
    pub expression_exec_end: UPtr<UMaterialExpression>,
}
#[repr(C, align(8))]
pub struct FMaterialExpressionAggregateEntry {
    pub attribute_index: i32,
    pub input: FExpressionInput,
}
#[repr(C, align(8))]
pub struct FMaterialExpressionConvertInput {
    pub expression_input: FExpressionInput,
    pub ty: EMaterialExpressionConvertType,
    pub default_value: crate::bindings::core_u_object::FLinearColor,
}
#[repr(C, align(4))]
pub struct FMaterialExpressionConvertOutput {
    pub ty: EMaterialExpressionConvertType,
    pub default_value: crate::bindings::core_u_object::FLinearColor,
}
#[repr(C, align(4))]
pub struct FMaterialExpressionConvertMapping {
    pub input_index: i32,
    pub input_component_index: i32,
    pub output_index: i32,
    pub output_component_index: i32,
}
#[repr(C, align(8))]
pub struct FCustomInput {
    pub input_name: FName,
    pub input: FExpressionInput,
}
#[repr(C, align(4))]
pub struct FCustomOutput {
    pub output_name: FName,
    pub output_type: ECustomMaterialOutputType,
}
#[repr(C, align(8))]
pub struct FCustomDefine {
    pub define_name: FString,
    pub define_value: FString,
}
#[repr(C, align(4))]
pub struct FDataDrivenShaderPlatformInfoInput {
    pub input_name: FName,
    pub property_condition: EDataDrivenShaderPlatformInfoCondition,
}
#[repr(C, align(8))]
pub struct FMaterialExpressionMaterialCacheAttribute {
    pub decoration: FString,
    pub value_type: u64,
    pub input: FExpressionInput,
}
#[repr(C, align(8))]
pub struct FFunctionExpressionInput {
    pub expression_input: UPtr<UMaterialExpressionFunctionInput>,
    pub expression_input_id: crate::bindings::core_u_object::FGuid,
    pub input: FExpressionInput,
}
#[repr(C, align(8))]
pub struct FFunctionExpressionOutput {
    pub expression_output: UPtr<UMaterialExpressionFunctionOutput>,
    pub expression_output_id: crate::bindings::core_u_object::FGuid,
    pub output: FExpressionOutput,
}
#[repr(C, align(8))]
pub struct FMaterialExpressionOperatorInput {
    pub expression_input: FExpressionInput,
    pub const_value: f32,
}
#[repr(C, align(8))]
pub struct FCompositeReroute {
    pub name: FName,
    pub expression: UPtr<UMaterialExpressionReroute>,
}
#[repr(C, align(8))]
pub struct FSwitchCustomInput {
    pub input_name: FName,
    pub input: FExpressionInput,
}
#[repr(C, align(4))]
pub struct FMaterialInstanceBasePropertyOverrides {
    pub flags_0: u8,
    pub flags_1: u8,
    pub flags_2: u8,
    pub blend_mode: EBlendMode,
    pub shading_model: EMaterialShadingModel,
    pub opacity_mask_clip_value: f32,
    pub displacement_scaling: FDisplacementScaling,
    pub displacement_fade_range: FDisplacementFadeRange,
    pub max_world_position_offset_displacement: f32,
}
#[repr(C, align(4))]
pub struct FDisplacementFadeRange {
    pub start_size_pixels: f32,
    pub end_size_pixels: f32,
}
#[repr(C, align(4))]
pub struct FDisplacementScaling {
    pub magnitude: f32,
    pub center: f32,
}
#[repr(C, align(8))]
pub struct FMaterialLayersFunctionsEditorOnlyData {
    pub layer_states: TArray<bool>,
    pub layer_names: TArray<FText>,
    pub restrict_to_layer_relatives: TArray<bool>,
    pub restrict_to_blend_relatives: TArray<bool>,
    pub layer_guids: TArray<crate::bindings::core_u_object::FGuid>,
    pub layer_link_states: TArray<EMaterialLayerLinkState>,
    pub deleted_parent_layer_guids: TArray<crate::bindings::core_u_object::FGuid>,
}
#[repr(C, align(8))]
pub struct FMaterialLayersFunctionsRuntimeData {
    pub layers: TArray<UPtr<UMaterialFunctionInterface>>,
    pub blends: TArray<UPtr<UMaterialFunctionInterface>>,
}
#[repr(C, align(8))]
pub struct FMaterialLayersFunctions {
    pub editor_only: FMaterialLayersFunctionsEditorOnlyData,
    pub layer_states_deprecated: TArray<bool>,
    pub layer_names_deprecated: TArray<FText>,
    pub restrict_to_layer_relatives_deprecated: TArray<bool>,
    pub restrict_to_blend_relatives_deprecated: TArray<bool>,
    pub layer_guids_deprecated: TArray<crate::bindings::core_u_object::FGuid>,
    pub layer_link_states_deprecated: TArray<EMaterialLayerLinkState>,
    pub deleted_parent_layer_guids_deprecated: TArray<
        crate::bindings::core_u_object::FGuid,
    >,
}
#[repr(C, align(1))]
pub struct FStaticComponentMaskValue {
    pub r: bool,
    pub g: bool,
    pub b: bool,
    pub a: bool,
}
#[repr(C, align(8))]
pub struct FMeshDrawCommandStatsBudget {
    pub category_name: FName,
    pub linked_stat_names: TArray<FName>,
    pub primitive_budget: i32,
    pub vertex_budget: i32,
    pub collection: i32,
    pub passes: TArray<FName>,
}
#[repr(C, align(4))]
pub struct FMeshDrawCommandStatsBudgetTotals {
    pub primitive_budget: i32,
    pub vertex_budget: i32,
    pub collection: i32,
}
#[repr(C, align(4))]
pub struct FMeshUVChannelInfo {
    pub b_initialized: bool,
    pub b_override_densities: bool,
    pub local_uv_densities: f32,
}
#[repr(C, align(8))]
pub struct FPurchaseInfo {
    pub identifier: FString,
    pub display_name: FString,
    pub display_description: FString,
    pub display_price: FString,
}
#[repr(C, align(8))]
pub struct FMLLevelSetModelAndBonesBinningInfo {
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
#[repr(C, align(8))]
pub struct FMLLevelSetModelInferenceInfo {
    pub nne_model_path: FString,
    pub model_architecture_activation_node_sizes: TArray<i32>,
    pub ml_model_weights: FString,
}
#[repr(C, align(4))]
pub struct FNaniteAssemblyBoneInfluence {
    pub bone_index: i32,
    pub bone_weight: f32,
}
#[repr(C, align(16))]
pub struct FNaniteAssemblyNode {
    pub part_index: i32,
    pub transform_space: ENaniteAssemblyNodeTransformSpace,
    pub transform: crate::bindings::core_u_object::FTransform3f,
    pub bone_influences: TArray<FNaniteAssemblyBoneInfluence>,
}
#[repr(C, align(8))]
pub struct FNaniteAssemblyPart {
    pub mesh_object_path: crate::bindings::core_u_object::FSoftObjectPath,
    pub material_remap: TArray<i32>,
}
#[repr(C, align(8))]
pub struct FNaniteAssemblyData {
    pub parts: TArray<FNaniteAssemblyPart>,
    pub nodes: TArray<FNaniteAssemblyNode>,
}
#[repr(C, align(4))]
pub struct FNavAgentSelector {
    pub flags_0: u8,
    pub flags_1: u8,
}
#[repr(C, align(4))]
pub struct FNavAvoidanceMask {
    pub flags_0: u8,
    pub flags_1: u8,
    pub flags_2: u8,
    pub flags_3: u8,
}
#[repr(C, align(8))]
pub struct FNavigationLinkBase {
    pub left_project_height: f32,
    pub max_fall_down_length: f32,
    pub snap_radius: f32,
    pub snap_height: f32,
    pub supported_agents: FNavAgentSelector,
    pub flags_36: u8,
    pub flags_37: u8,
    pub description: FString,
    pub direction: ENavLinkDirection,
    pub flags_57: u8,
    pub flags_58: u8,
    pub area_class: TSubclassOf<UNavAreaBase>,
}
#[repr(C, align(8))]
pub struct FNavigationLink {
    pub left: crate::bindings::core_u_object::FVector,
    pub right: crate::bindings::core_u_object::FVector,
}
#[repr(C, align(8))]
pub struct FNavigationSegmentLink {
    pub left_start: crate::bindings::core_u_object::FVector,
    pub left_end: crate::bindings::core_u_object::FVector,
    pub right_start: crate::bindings::core_u_object::FVector,
    pub right_end: crate::bindings::core_u_object::FVector,
}
#[repr(C, align(8))]
pub struct FVector_NetQuantize10 {}
#[repr(C, align(8))]
pub struct FOverlapInfo {
    pub b_from_sweep: bool,
    pub overlap_info: FHitResult,
}
#[repr(C, align(8))]
pub struct FParticleCurvePair {
    pub curve_name: FString,
    pub curve_object: UPtr<crate::bindings::core_u_object::UObject>,
}
#[repr(C, align(8))]
pub struct FParticleRandomSeedInfo {
    pub parameter_name: FName,
    pub flags_12: u8,
    pub random_seeds: TArray<i32>,
}
#[repr(C, align(4))]
pub struct FBeamModifierOptions {
    pub flags_0: u8,
}
#[repr(C, align(8))]
pub struct FParticleEvent_GenerateInfo {
    pub ty: EParticleEventType,
    pub frequency: i32,
    pub particle_frequency: i32,
    pub flags_12: u8,
    pub custom_name: FName,
    pub particle_module_events_to_send_to_game: TArray<
        UPtr<UParticleModuleEventSendToGame>,
    >,
}
#[repr(C, align(8))]
pub struct FLocationBoneSocketInfo {
    pub bone_socket_name: FName,
    pub offset: crate::bindings::core_u_object::FVector,
}
#[repr(C, align(4))]
pub struct FOrbitOptions {
    pub flags_0: u8,
}
#[repr(C, align(8))]
pub struct FEmitterDynamicParameter {
    pub param_name: FName,
    pub flags_12: u8,
    pub value_method: EEmitterDynamicParameterValue,
    pub flags_20: u8,
    pub param_value: FRawDistributionFloat,
}
#[repr(C, align(4))]
pub struct FBeamTargetData {
    pub target_name: FName,
    pub target_percentage: f32,
}
#[repr(C, align(16))]
pub struct FGPUSpriteLocalVectorFieldInfo {
    pub field: UPtr<UVectorField>,
    pub transform: crate::bindings::core_u_object::FTransform,
    pub min_initial_rotation: crate::bindings::core_u_object::FRotator,
    pub max_initial_rotation: crate::bindings::core_u_object::FRotator,
    pub rotation_rate: crate::bindings::core_u_object::FRotator,
    pub intensity: f32,
    pub tightness: f32,
    pub flags_192: u8,
}
#[repr(C, align(16))]
pub struct FGPUSpriteEmitterInfo {
    pub required_module: UPtr<UParticleModuleRequired>,
    pub spawn_module: UPtr<UParticleModuleSpawn>,
    pub spawn_per_unit_module: UPtr<UParticleModuleSpawnPerUnit>,
    pub spawn_modules: TArray<UPtr<UParticleModule>>,
    pub local_vector_field: FGPUSpriteLocalVectorFieldInfo,
    pub vector_field_scale: FFloatDistribution,
    pub drag_coefficient: FFloatDistribution,
    pub point_attractor_strength: FFloatDistribution,
    pub resilience: FFloatDistribution,
    pub constant_acceleration: crate::bindings::core_u_object::FVector,
    pub point_attractor_position: crate::bindings::core_u_object::FVector,
    pub point_attractor_radius_sq: f32,
    pub orbit_offset_base: crate::bindings::core_u_object::FVector,
    pub orbit_offset_range: crate::bindings::core_u_object::FVector,
    pub inv_max_size: crate::bindings::core_u_object::FVector2D,
    pub inv_rotation_rate_scale: f32,
    pub max_lifetime: f32,
    pub max_particle_count: i32,
    pub screen_alignment: EParticleScreenAlignment,
    pub lock_axis_flag: EParticleAxisLock,
    pub flags_520: u8,
    pub collision_mode: EParticleCollisionMode,
    pub flags_528: u8,
    pub min_facing_camera_blend_distance: f32,
    pub max_facing_camera_blend_distance: f32,
    pub dynamic_color: FRawDistributionVector,
    pub dynamic_alpha: FRawDistributionFloat,
    pub dynamic_color_scale: FRawDistributionVector,
    pub dynamic_alpha_scale: FRawDistributionFloat,
}
#[repr(C, align(16))]
pub struct FGPUSpriteResourceData {
    pub quantized_color_samples: TArray<crate::bindings::core_u_object::FColor>,
    pub quantized_misc_samples: TArray<crate::bindings::core_u_object::FColor>,
    pub quantized_simulation_attr_samples: TArray<
        crate::bindings::core_u_object::FColor,
    >,
    pub color_scale: crate::bindings::core_u_object::FVector4,
    pub color_bias: crate::bindings::core_u_object::FVector4,
    pub misc_scale: crate::bindings::core_u_object::FVector4,
    pub misc_bias: crate::bindings::core_u_object::FVector4,
    pub simulation_attr_curve_scale: crate::bindings::core_u_object::FVector4,
    pub simulation_attr_curve_bias: crate::bindings::core_u_object::FVector4,
    pub sub_image_size: crate::bindings::core_u_object::FVector4,
    pub size_by_speed: crate::bindings::core_u_object::FVector4,
    pub constant_acceleration: crate::bindings::core_u_object::FVector,
    pub orbit_offset_base: crate::bindings::core_u_object::FVector,
    pub orbit_offset_range: crate::bindings::core_u_object::FVector,
    pub orbit_frequency_base: crate::bindings::core_u_object::FVector,
    pub orbit_frequency_range: crate::bindings::core_u_object::FVector,
    pub orbit_phase_base: crate::bindings::core_u_object::FVector,
    pub orbit_phase_range: crate::bindings::core_u_object::FVector,
    pub global_vector_field_scale: f32,
    pub global_vector_field_tightness: f32,
    pub per_particle_vector_field_scale: f32,
    pub per_particle_vector_field_bias: f32,
    pub drag_coefficient_scale: f32,
    pub drag_coefficient_bias: f32,
    pub resilience_scale: f32,
    pub resilience_bias: f32,
    pub collision_radius_scale: f32,
    pub collision_radius_bias: f32,
    pub collision_time_bias: f32,
    pub collision_random_spread: f32,
    pub collision_random_distribution: f32,
    pub one_minus_friction: f32,
    pub rotation_rate_scale: f32,
    pub camera_motion_blur_amount: f32,
    pub screen_alignment: EParticleScreenAlignment,
    pub lock_axis_flag: EParticleAxisLock,
    pub pivot_offset: crate::bindings::core_u_object::FVector2D,
    pub flags_560: u8,
    pub min_facing_camera_blend_distance: f32,
    pub max_facing_camera_blend_distance: f32,
}
#[repr(C, align(8))]
pub struct FParticleEmitterReplayFrame {}
#[repr(C, align(8))]
pub struct FParticleSystemReplayFrame {}
#[repr(C, align(4))]
pub struct FParticleSystemStaticState {}
#[repr(C, align(8))]
pub struct FParticleSystemDynamicState {
    pub transform: crate::bindings::state_stream::FTransformHandle,
    pub system_asset: UPtr<UFXSystemAsset>,
}
#[repr(C, align(8))]
pub struct FParticleSystemHandle {}
#[repr(C, align(8))]
pub struct FPieFixupStructWithSoftObjectPath {
    pub path: crate::bindings::core_u_object::FSoftObjectPath,
    pub typed_ptr: TSoftObjectPtr<AActor>,
}
#[repr(C, align(8))]
pub struct FDelegateArray {
    pub delegates: TArray<FDelegateArray_Delegates>,
}
#[repr(C, align(16))]
pub struct FSceneViewExtensionIsActiveFunctor {}
#[repr(C, align(8))]
pub struct FSkeletalMeshOptimizationSettings {
    pub termination_criterion: SkeletalMeshTerminationCriterion,
    pub num_of_triangles_percentage: f32,
    pub num_of_vert_percentage: f32,
    pub max_num_of_triangles: u32,
    pub max_num_of_verts: u32,
    pub max_num_of_triangles_percentage: u32,
    pub max_num_of_verts_percentage: u32,
    pub max_deviation_percentage: f32,
    pub reduction_method: SkeletalMeshOptimizationType,
    pub silhouette_importance: SkeletalMeshOptimizationImportance,
    pub texture_importance: SkeletalMeshOptimizationImportance,
    pub shading_importance: SkeletalMeshOptimizationImportance,
    pub skinning_importance: SkeletalMeshOptimizationImportance,
    pub flags_37: u8,
    pub welding_threshold: f32,
    pub normals_threshold: f32,
    pub max_bones_per_vertex: i32,
    pub flags_52: u8,
    pub volume_importance: f32,
    pub flags_60: u8,
    pub base_lod: i32,
    pub bones_to_remove_deprecated: TArray<FBoneReference>,
    pub bake_pose_deprecated: UPtr<UAnimSequence>,
}
#[repr(C, align(8))]
pub struct FSkinnedMeshStaticState {
    pub material_relevance: u64,
}
#[repr(C, align(8))]
pub struct FSkinnedMeshDynamicState {
    pub transform: crate::bindings::state_stream::FTransformHandle,
    pub skinned_asset: UPtr<USkinnedAsset>,
    pub override_materials: TArray<UPtr<UMaterialInterface>>,
}
#[repr(C, align(8))]
pub struct FSkinnedMeshHandle {}
#[repr(C, align(4))]
pub struct FSkyAtmosphereStaticState {}
#[repr(C, align(8))]
pub struct FOverrideAtmosphericLight {
    pub enabled_mask: u8,
    pub direction: crate::bindings::core_u_object::FVector,
}
#[repr(C, align(16))]
pub struct FSkyAtmosphereDynamicState {
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
    pub other_tent_distribution_tip_altitude: f32,
    pub other_tent_distribution_tip_value: f32,
    pub other_tent_distribution_width: f32,
    pub sky_luminance_factor: crate::bindings::core_u_object::FLinearColor,
    pub sky_and_aerial_perspective_luminance_factor: crate::bindings::core_u_object::FLinearColor,
    pub aerial_pespective_view_distance_scale: f32,
    pub height_fog_contribution: f32,
    pub transmittance_min_light_elevation_angle: f32,
    pub aerial_perspective_start_depth: f32,
    pub flags_180: u8,
    pub component_transform: crate::bindings::core_u_object::FTransform,
    pub override_atmospheric_light: FOverrideAtmosphericLight,
}
#[repr(C, align(8))]
pub struct FSkyAtmosphereHandle {}
#[repr(C, align(8))]
pub struct FSoftWorldReference {
    pub world_asset: TSoftObjectPtr<UWorld>,
}
#[repr(C, align(8))]
pub struct FSourceEffectChainEntry {
    pub preset: UPtr<USoundEffectSourcePreset>,
    pub flags_8: u8,
}
#[repr(C, align(8))]
pub struct FSoundSourceBusSendInfo {
    pub source_bus_send_level_control_method: ESourceBusSendLevelControlMethod,
    pub sound_source_bus: UPtr<USoundSourceBus>,
    pub audio_bus: UPtr<UAudioBus>,
    pub send_level: f32,
    pub min_send_level: f32,
    pub max_send_level: f32,
    pub min_send_distance: f32,
    pub max_send_distance: f32,
    pub custom_send_level_curve: FRuntimeFloatCurve,
}
#[repr(C, align(8))]
pub struct FSoundTimecodeOffset {
    pub num_of_seconds_since_midnight: f64,
}
#[repr(C, align(8))]
pub struct FSoundWaveTimecodeInfo {
    pub num_samples_since_midnight: u64,
    pub num_samples_per_second: u32,
    pub description: FString,
    pub originator_time: FString,
    pub originator_date: FString,
    pub originator_description: FString,
    pub originator_reference: FString,
    pub timecode_rate: crate::bindings::core_u_object::FFrameRate,
    pub b_timecode_is_drop_frame: bool,
}
#[repr(C, align(8))]
pub struct FStaticMeshComponentLODInfo {}
#[repr(C, align(4))]
pub struct FStaticMeshStaticState {}
#[repr(C, align(8))]
pub struct FStaticMeshDynamicState {
    pub transform: crate::bindings::state_stream::FTransformHandle,
    pub mesh: UPtr<UStaticMesh>,
    pub override_materials: TArray<UPtr<UMaterialInterface>>,
    pub owners: TArray<u32>,
    pub flags_64: u8,
}
#[repr(C, align(8))]
pub struct FStaticMeshHandle {}
#[repr(C, align(4))]
pub struct FStaticParameterBase {
    pub parameter_info: FMaterialParameterInfo,
    pub b_override: bool,
    pub expression_guid: crate::bindings::core_u_object::FGuid,
}
#[repr(C, align(4))]
pub struct FStaticSwitchParameter {
    pub value: bool,
}
#[repr(C, align(4))]
pub struct FStaticComponentMaskParameter {
    pub r: bool,
    pub g: bool,
    pub b: bool,
    pub a: bool,
}
#[repr(C, align(4))]
pub struct FStaticTerrainLayerWeightParameter {
    pub parameter_info_deprecated: FMaterialParameterInfo,
    pub expression_guid_deprecated: crate::bindings::core_u_object::FGuid,
    pub b_override_deprecated: bool,
    pub layer_name: FName,
    pub weightmap_index: i32,
    pub b_is_repeated_layer: bool,
    pub b_weight_based_blend: bool,
}
#[repr(C, align(8))]
pub struct FStaticMaterialLayersParameter {
    pub value: FMaterialLayersFunctions,
}
#[repr(C, align(8))]
pub struct FStaticParameterSetRuntimeData {
    pub static_switch_parameters: TArray<FStaticSwitchParameter>,
    pub material_layers: FMaterialLayersFunctionsRuntimeData,
    pub flags_72: u8,
}
#[repr(C, align(8))]
pub struct FStaticParameterSetEditorOnlyData {
    pub static_switch_parameters_deprecated: TArray<FStaticSwitchParameter>,
    pub static_component_mask_parameters: TArray<FStaticComponentMaskParameter>,
    pub terrain_layer_weight_parameters: TArray<FStaticTerrainLayerWeightParameter>,
    pub material_layers: FMaterialLayersFunctionsEditorOnlyData,
}
#[repr(C, align(8))]
pub struct FStaticParameterSet {
    pub editor_only: FStaticParameterSetEditorOnlyData,
    pub material_layers_parameters_deprecated: TArray<FStaticMaterialLayersParameter>,
    pub static_switch_parameters_deprecated: TArray<FStaticSwitchParameter>,
    pub static_component_mask_parameters_deprecated: TArray<
        FStaticComponentMaskParameter,
    >,
    pub terrain_layer_weight_parameters_deprecated: TArray<
        FStaticTerrainLayerWeightParameter,
    >,
}
#[repr(C, align(8))]
pub struct FStreamingRenderAssetPrimitiveInfo {
    pub render_asset: UPtr<UStreamableRenderAsset>,
    pub bounds: crate::bindings::core_u_object::FBoxSphereBounds,
    pub texel_factor: f32,
    pub packed_relative_box: u32,
    pub flags_72: u8,
}
#[repr(C, align(4))]
pub struct FStreamingTextureBuildInfo {
    pub packed_relative_box: u32,
    pub texture_level_index: i32,
    pub texel_factor: f32,
}
#[repr(C, align(8))]
pub struct FTimelineEventEntry {
    pub time: f32,
    pub event_func: FTimelineEventEntry_EventFunc,
}
#[repr(C, align(8))]
pub struct FTimelineVectorTrack {
    pub vector_curve: UPtr<UCurveVector>,
    pub interp_func: FTimelineVectorTrack_InterpFunc,
    pub track_name: FName,
    pub vector_property_name: FName,
}
#[repr(C, align(8))]
pub struct FTimelineFloatTrack {
    pub float_curve: UPtr<UCurveFloat>,
    pub interp_func: FTimelineFloatTrack_InterpFunc,
    pub track_name: FName,
    pub float_property_name: FName,
}
#[repr(C, align(8))]
pub struct FTimelineLinearColorTrack {
    pub linear_color_curve: UPtr<UCurveLinearColor>,
    pub interp_func: FTimelineLinearColorTrack_InterpFunc,
    pub track_name: FName,
    pub linear_color_property_name: FName,
}
#[repr(C, align(8))]
pub struct FTimeline {
    pub length_mode: ETimelineLengthMode,
    pub flags_1: u8,
    pub playing_state_tracker: u8,
    pub length: f32,
    pub play_rate: f32,
    pub position: f32,
    pub events: TArray<FTimelineEventEntry>,
    pub interp_vectors: TArray<FTimelineVectorTrack>,
    pub interp_floats: TArray<FTimelineFloatTrack>,
    pub interp_linear_colors: TArray<FTimelineLinearColorTrack>,
    pub timeline_post_update_func: FTimeline_TimelinePostUpdateFunc,
    pub timeline_finished_func: FTimeline_TimelineFinishedFunc,
    pub property_set_object: TWeakObjectPtr<crate::bindings::core_u_object::UObject>,
    pub direction_property_name: FName,
}
#[repr(C, align(8))]
pub struct FActorContainerPath {
    pub container_guids: TArray<crate::bindings::core_u_object::FGuid>,
}
#[repr(C, align(8))]
pub struct FAttributeKey {
    pub time: f32,
}
#[repr(C, align(8))]
pub struct FAttributeCurve {
    pub keys: TArray<FAttributeKey>,
    pub script_struct_path: crate::bindings::core_u_object::FSoftObjectPath,
    pub script_struct: UPtr<crate::bindings::core_u_object::UScriptStruct>,
    pub b_should_interpolate: bool,
}
#[repr(C, align(4))]
pub struct FQuartzPulseOverrideStep {
    pub number_of_pulses: i32,
    pub pulse_duration: EQuartzCommandQuantization,
}
#[repr(C, align(8))]
pub struct FQuartzTimeSignature {
    pub num_beats: i32,
    pub beat_type: EQuartzTimeSignatureQuantization,
    pub optional_pulse_override: TArray<FQuartzPulseOverrideStep>,
}
#[repr(C, align(4))]
pub struct FQuartzTransportTimeStamp {
    pub bars: i32,
    pub beat: i32,
    pub beat_fraction: f32,
    pub seconds: f32,
}
#[repr(C, align(8))]
pub struct FQuartzClockSettings {
    pub time_signature: FQuartzTimeSignature,
    pub b_ignore_level_change: bool,
}
#[repr(C, align(8))]
pub struct FQuartzQuantizationBoundary {
    pub quantization: EQuartzCommandQuantization,
    pub multiplier: f32,
    pub counting_reference_point: EQuarztQuantizationReference,
    pub b_fire_on_clock_start: bool,
    pub b_cancel_command_if_clock_is_not_running: bool,
    pub b_reset_clock_on_queued: bool,
    pub b_resume_clock_on_queued: bool,
}
#[repr(C, align(8))]
pub struct FNavAvoidanceData {}
#[repr(C, align(1))]
pub struct FMovementProperties {
    pub flags_0: u8,
}
#[repr(C, align(4))]
pub struct FNavMovementProperties {
    pub fixed_path_braking_distance: f32,
    pub b_update_nav_agent_with_owners_collision: bool,
    pub b_use_acceleration_for_paths: bool,
    pub b_use_fixed_braking_distance_for_paths: bool,
    pub b_stop_movement_abort_paths: bool,
}
#[repr(C, align(8))]
pub struct FNavAgentProperties {
    pub agent_radius: f32,
    pub agent_height: f32,
    pub agent_step_height: f32,
    pub nav_walking_search_height_scale: f32,
    pub preferred_nav_data: crate::bindings::core_u_object::FSoftClassPath,
}
#[repr(C, align(8))]
pub struct FNavDataConfig {
    pub name: FName,
    pub color: crate::bindings::core_u_object::FColor,
    pub default_query_extent: crate::bindings::core_u_object::FVector,
    pub nav_data_class: TSoftObjectPtr<crate::bindings::core_u_object::UClass>,
}
#[repr(C, align(8))]
pub struct FAlphaBlendArgs {
    pub custom_curve: UPtr<UCurveFloat>,
    pub blend_time: f32,
    pub blend_option: EAlphaBlendOption,
}
#[repr(C, align(8))]
pub struct FBlendSampleData {
    pub sample_data_index: i32,
    pub animation: UPtr<UAnimSequence>,
    pub total_weight: f32,
    pub weight_rate: f32,
    pub time: f32,
    pub previous_time: f32,
    pub sample_play_rate: f32,
}
#[repr(C, align(8))]
pub struct FBlendFilter {}
#[repr(C, align(4))]
pub struct FMarkerSyncAnimPosition {
    pub previous_marker_name: FName,
    pub next_marker_name: FName,
    pub position_between_markers: f32,
}
#[repr(C, align(8))]
pub struct FAnimTickRecord {
    pub source_asset: UPtr<UAnimationAsset>,
}
#[repr(C, align(8))]
pub struct FAnimGroupInstance {}
#[repr(C, align(16))]
pub struct FRootMotionMovementParams {
    pub b_has_root_motion: bool,
    pub blend_weight: f32,
    pub root_motion_transform: crate::bindings::core_u_object::FTransform,
}
#[repr(C, align(4))]
pub struct FAnimationGroupReference {
    pub method: EAnimSyncMethod,
    pub group_name: FName,
    pub group_role: EAnimGroupRole,
}
#[repr(C, align(8))]
pub struct FEncounteredStateMachineStack {}
#[repr(C, align(8))]
pub struct FAnimationRecordingSettings {
    pub b_record_in_world_space: bool,
    pub b_remove_root_animation: bool,
    pub b_auto_save_asset: bool,
    pub sample_frame_rate: crate::bindings::core_u_object::FFrameRate,
    pub length: f32,
    pub interpolation: EAnimInterpolationType,
    pub interp_mode: ERichCurveInterpMode,
    pub tangent_mode: ERichCurveTangentMode,
    pub b_record_transforms: bool,
    pub b_record_morph_targets: bool,
    pub b_record_attribute_curves: bool,
    pub b_record_material_curves: bool,
    pub b_transact_recording: bool,
    pub include_animation_names: TArray<FString>,
    pub exclude_animation_names: TArray<FString>,
}
#[repr(C, align(8))]
pub struct FAnimBankSequence {
    pub sequence: UPtr<UAnimSequence>,
    pub flags_8: u8,
    pub position: f32,
    pub play_rate: f32,
    pub bounds_scale: f32,
}
#[repr(C, align(8))]
pub struct FAnimBankItem {
    pub bank_asset: UPtr<UAnimBank>,
    pub sequence_index: i32,
}
#[repr(C, align(8))]
pub struct FSoftAnimBankItem {
    pub bank_asset: TSoftObjectPtr<UAnimBank>,
    pub sequence_index: i32,
}
#[repr(C, align(8))]
pub struct FSkinnedMeshComponentDescriptorBase {
    pub hash: u32,
    pub mobility: EComponentMobility,
    pub component_class: TSubclassOf<UInstancedSkinnedMeshComponent>,
    pub instance_min_draw_distance: i32,
    pub instance_start_cull_distance: i32,
    pub instance_end_cull_distance: i32,
    pub flags_36: u8,
    pub flags_37: u8,
    pub b_visible_in_ray_tracing: bool,
    pub b_affect_dynamic_indirect_lighting: bool,
    pub b_affect_distance_field_lighting: bool,
    pub primitive_bounds_override: crate::bindings::core_u_object::FBox,
    pub b_is_instance_data_gpu_only: bool,
    pub num_instances_gpu_only: i32,
    pub num_custom_data_floats_gpu_only: i32,
    pub flags_116: u8,
    pub hlod_batching_policy: EHLODBatchingPolicy,
}
#[repr(C, align(8))]
pub struct FSkinnedMeshComponentDescriptor {
    pub skinned_asset: UPtr<USkinnedAsset>,
    pub transform_provider: UPtr<UTransformProviderData>,
}
#[repr(C, align(8))]
pub struct FSoftSkinnedMeshComponentDescriptor {
    pub skinned_asset: TSoftObjectPtr<USkinnedAsset>,
    pub transform_provider: TSoftObjectPtr<UTransformProviderData>,
}
#[repr(C, align(4))]
pub struct FStateMachineStateDebugData {}
#[repr(C, align(8))]
pub struct FStateMachineDebugData {}
#[repr(C, align(8))]
pub struct FAnimationFrameSnapshot {}
#[repr(C, align(8))]
pub struct FAnimBlueprintDebugData {}
#[repr(C, align(1))]
pub struct FAnimBlueprintMutableData {}
#[repr(C, align(1))]
pub struct FAnimBlueprintConstantData {}
#[repr(C, align(8))]
pub struct FAnimBlueprintFunction {
    pub name: FName,
    pub group: FName,
    pub output_pose_node_index: i32,
    pub input_pose_names: TArray<FName>,
    pub input_pose_node_indices: TArray<i32>,
    pub b_implemented: bool,
}
#[repr(C, align(8))]
pub struct FCachedPoseIndices {
    pub ordered_saved_pose_node_indices: TArray<i32>,
}
#[repr(C, align(8))]
pub struct FGraphAssetPlayerInformation {
    pub player_node_indices: TArray<i32>,
}
#[repr(C, align(8))]
pub struct FAnimGraphBlendOptions {
    pub blend_in_time: f32,
    pub blend_in_profile: UPtr<UBlendProfile>,
    pub blend_out_time: f32,
    pub blend_out_profile: UPtr<UBlendProfile>,
}
#[repr(C, align(8))]
pub struct FRootMotionExtractionStep {
    pub anim_sequence: UPtr<UAnimSequence>,
    pub start_position: f32,
    pub end_position: f32,
}
#[repr(C, align(8))]
pub struct FAnimSegment {
    pub cached_play_length: f32,
    pub anim_reference: UPtr<UAnimSequenceBase>,
    pub start_pos: f32,
    pub anim_start_time: f32,
    pub anim_end_time: f32,
    pub anim_play_rate: f32,
    pub looping_count: i32,
}
#[repr(C, align(8))]
pub struct FAnimTrack {
    pub anim_segments: TArray<FAnimSegment>,
}
#[repr(C, align(4))]
pub struct FAnimationErrorStats {}
#[repr(C, align(4))]
pub struct FAnimCompressedCurveIndexedName {
    pub curve_name: FName,
}
#[repr(C, align(8))]
pub struct FAnimCurveBase {
    pub last_observed_name_deprecated: FName,
    pub name_deprecated: FSmartName,
    pub color: crate::bindings::core_u_object::FLinearColor,
    pub comment: FString,
    pub curve_name: FName,
    pub curve_type_flags: i32,
}
#[repr(C, align(8))]
pub struct FFloatCurve {
    pub float_curve: FRichCurve,
}
#[repr(C, align(8))]
pub struct FVectorCurve {
    pub float_curves: FRichCurve,
}
#[repr(C, align(8))]
pub struct FTransformCurve {
    pub translation_curve: FVectorCurve,
    pub rotation_curve: FVectorCurve,
    pub scale_curve: FVectorCurve,
}
#[repr(C, align(4))]
pub struct FCachedFloatCurve {
    pub curve_name: FName,
}
#[repr(C, align(8))]
pub struct FRawCurveTracks {
    pub float_curves: TArray<FFloatCurve>,
    pub vector_curves: TArray<FVectorCurve>,
    pub transform_curves: TArray<FTransformCurve>,
}
#[repr(C, align(8))]
pub struct FBoneAnimationTrack {
    pub internal_track_data: FRawAnimSequenceTrack,
    pub bone_tree_index: i32,
    pub name: FName,
}
#[repr(C, align(8))]
pub struct FRawAnimSequenceTrack {
    pub pos_keys: TArray<crate::bindings::core_u_object::FVector3f>,
    pub rot_keys: TArray<crate::bindings::core_u_object::FQuat4f>,
    pub scale_keys: TArray<crate::bindings::core_u_object::FVector3f>,
}
#[repr(C, align(8))]
pub struct FAnimationCurveData {
    pub float_curves: TArray<FFloatCurve>,
    pub transform_curves: TArray<FTransformCurve>,
}
#[repr(C, align(8))]
pub struct FAnimatedBoneAttribute {
    pub identifier: FAnimationAttributeIdentifier,
    pub curve: FAttributeCurve,
}
#[repr(C, align(8))]
pub struct FAnimExecutionContext {}
#[repr(C, align(8))]
pub struct FAnimInitializationContext {}
#[repr(C, align(8))]
pub struct FAnimUpdateContext {}
#[repr(C, align(8))]
pub struct FAnimPoseContext {}
#[repr(C, align(8))]
pub struct FAnimComponentSpacePoseContext {}
#[repr(C, align(8))]
pub struct FA2Pose {
    pub bones: TArray<crate::bindings::core_u_object::FTransform>,
}
#[repr(C, align(8))]
pub struct FA2CSPose {
    pub component_space_flags: TArray<u8>,
}
#[repr(C, align(8))]
pub struct FAnimLinkableElement {
    pub linked_montage: UPtr<UAnimMontage>,
    pub slot_index: i32,
    pub segment_index: i32,
    pub link_method: EAnimLinkMethod,
    pub cached_link_method: EAnimLinkMethod,
    pub segment_begin_time: f32,
    pub segment_length: f32,
    pub link_value: f32,
    pub linked_sequence: UPtr<UAnimSequenceBase>,
}
#[repr(C, align(8))]
pub struct FCompositeSection {
    pub section_name: FName,
    pub start_time_deprecated: f32,
    pub next_section_name: FName,
    pub meta_data: TArray<UPtr<UAnimMetaData>>,
}
#[repr(C, align(8))]
pub struct FSlotAnimationTrack {
    pub slot_name: FName,
    pub anim_track: FAnimTrack,
}
#[repr(C, align(8))]
pub struct FBranchingPoint {
    pub event_name: FName,
    pub display_time_deprecated: f32,
    pub trigger_time_offset: f32,
}
#[repr(C, align(4))]
pub struct FBranchingPointMarker {
    pub notify_index: i32,
    pub trigger_time: f32,
    pub notify_event_type: EAnimNotifyEventType,
}
#[repr(C, align(8))]
pub struct FMontageBlendSettings {
    pub blend_profile: UPtr<UBlendProfile>,
    pub blend: FAlphaBlendArgs,
    pub blend_mode: EMontageBlendMode,
}
#[repr(C, align(8))]
pub struct FAnimMontageInstance {
    pub montage: UPtr<UAnimMontage>,
    pub b_playing: bool,
    pub default_blend_time_multiplier: f32,
    pub next_sections: TArray<i32>,
    pub prev_sections: TArray<i32>,
    pub active_state_branching_points: TArray<FAnimNotifyEvent>,
    pub position: f32,
    pub play_rate: f32,
    pub blend: FAlphaBlend,
    pub disable_root_motion_count: i32,
}
#[repr(C, align(8))]
pub struct FAnimNotifyEvent {
    pub display_time_deprecated: f32,
    pub trigger_time_offset: f32,
    pub end_trigger_time_offset: f32,
    pub trigger_weight_threshold: f32,
    pub notify_name: FName,
    pub notify: UPtr<UAnimNotify>,
    pub notify_state_class: UPtr<UAnimNotifyState>,
    pub duration: f32,
    pub end_link: FAnimLinkableElement,
    pub b_converted_from_branching_point: bool,
    pub montage_tick_type: EMontageNotifyTickType,
    pub notify_trigger_chance: f32,
    pub notify_filter_type: ENotifyFilterType,
    pub notify_filter_lod: i32,
    pub b_can_be_filtered_via_request: bool,
    pub b_trigger_on_dedicated_server: bool,
    pub b_trigger_on_follower: bool,
    pub notify_color: crate::bindings::core_u_object::FColor,
    pub guid: crate::bindings::core_u_object::FGuid,
    pub track_index: i32,
}
#[repr(C, align(8))]
pub struct FAnimNodeData {
    pub anim_class_interface: TScriptInterface<IAnimClassInterface>,
    pub entries: TArray<u32>,
    pub node_index: i32,
    pub flags: u32,
}
#[repr(C, align(8))]
pub struct FAnimNodeStructData {
    pub name_to_index_map: TMap<FName, i32>,
    pub num_properties: i32,
}
#[repr(C, align(8))]
pub struct FAnimNode_ConvertComponentToLocalSpace {
    pub component_pose: FComponentSpacePoseLink,
}
#[repr(C, align(8))]
pub struct FAnimNode_ConvertLocalToComponentSpace {
    pub local_pose: FPoseLink,
}
#[repr(C, align(8))]
pub struct FAnimNode_ApplyMeshSpaceAdditive {
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
}
#[repr(C, align(8))]
pub struct FAnimNode_CustomProperty {
    pub source_property_names: TArray<FName>,
    pub dest_property_names: TArray<FName>,
    pub target_instance: UPtr<crate::bindings::core_u_object::UObject>,
    pub source_instance: UPtr<crate::bindings::core_u_object::UObject>,
}
#[repr(C, align(16))]
pub struct FAnimNode_DeadBlending {
    pub source: FPoseLink,
    pub b_always_use_default_blend_settings: bool,
    pub default_blend_duration: f32,
    pub default_blend_profile: UPtr<UBlendProfile>,
    pub default_blend_mode: EAlphaBlendOption,
    pub default_custom_blend_curve: UPtr<UCurveFloat>,
    pub blend_time_multiplier: f32,
    pub b_linearly_interpolate_scales: bool,
    pub filtered_curves: TArray<FName>,
    pub extrapolation_filtered_curves: TArray<FName>,
    pub filtered_bones: TArray<FBoneReference>,
    pub extrapolation_half_life: f32,
    pub extrapolation_half_life_min: f32,
    pub extrapolation_half_life_max: f32,
    pub maximum_translation_velocity: f32,
    pub maximum_rotation_velocity: f32,
    pub maximum_scale_velocity: f32,
    pub maximum_curve_velocity: f32,
    pub b_reset_on_becoming_relevant: bool,
    pub b_forward_requests_through_skipped_cached_pose_nodes: bool,
    pub tag: FName,
    pub b_show_extrapolations: bool,
    pub request_queue: TArray<FInertializationRequest>,
    pub inertialization_custom_blend_curve: UPtr<UCurveFloat>,
    pub inertialization_request_anim_instance: UPtr<
        crate::bindings::core_u_object::UObject,
    >,
}
#[repr(C, align(16))]
pub struct FInertializationPose {}
#[repr(C, align(8))]
pub struct FInertializationBoneDiff {}
#[repr(C, align(8))]
pub struct FInertializationPoseDiff {}
#[repr(C, align(16))]
pub struct FAnimNode_Inertialization {
    pub source: FPoseLink,
    pub default_blend_profile: UPtr<UBlendProfile>,
    pub filtered_curves: TArray<FName>,
    pub filtered_bones: TArray<FBoneReference>,
    pub b_preallocate_memory_deprecated: bool,
    pub b_reset_on_becoming_relevant: bool,
    pub b_forward_requests_through_skipped_cached_pose_nodes: bool,
    pub tag: FName,
    pub request_queue: TArray<FInertializationRequest>,
    pub inertialization_request_anim_instance: UPtr<
        crate::bindings::core_u_object::UObject,
    >,
}
#[repr(C, align(8))]
pub struct FAnimNode_LinkedAnimGraph {
    pub input_poses: TArray<FPoseLink>,
    pub input_pose_names: TArray<FName>,
    pub instance_class: TSubclassOf<UAnimInstance>,
    pub tag_deprecated: FName,
    pub pending_blend_out_profile: UPtr<UBlendProfile>,
    pub pending_blend_in_profile: UPtr<UBlendProfile>,
    pub flags_320: u8,
}
#[repr(C, align(8))]
pub struct FAnimNode_LinkedAnimLayer {
    pub interface: TSubclassOf<UAnimLayerInterface>,
    pub layer: FName,
}
#[repr(C, align(8))]
pub struct FAnimNode_LinkedInputPose {
    pub name: FName,
    pub graph: FName,
    pub input_pose: FPoseLink,
    pub flags_304: u8,
}
#[repr(C, align(8))]
pub struct FAnimNode_SaveCachedPose {
    pub pose: FPoseLink,
    pub cache_pose_name: FName,
}
#[repr(C, align(8))]
pub struct FAnimNode_SequencePlayerBase {
    pub play_rate_scale_bias_clamp_state: FInputScaleBiasClampState,
}
#[repr(C, align(4))]
pub struct FInputScaleBiasClampState {}
#[repr(C, align(8))]
pub struct FAnimNode_SequencePlayer {
    pub group_name: FName,
    pub group_role: EAnimGroupRole,
    pub b_override_position_when_joining_sync_group_as_leader: bool,
    pub method: EAnimSyncMethod,
    pub b_ignore_for_relevancy_test: bool,
    pub sequence: UPtr<UAnimSequenceBase>,
    pub play_rate_basis: f32,
    pub play_rate: f32,
    pub play_rate_scale_bias_clamp_constants: FInputScaleBiasClampConstants,
    pub play_rate_scale_bias_clamp_deprecated: FInputScaleBiasClamp,
    pub start_position: f32,
    pub b_loop_animation: bool,
    pub b_start_from_matching_pose: bool,
}
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
#[repr(C, align(8))]
pub struct FAnimNode_SequencePlayer_Standalone {
    pub group_name: FName,
    pub group_role: EAnimGroupRole,
    pub b_override_position_when_joining_sync_group_as_leader: bool,
    pub method: EAnimSyncMethod,
    pub b_ignore_for_relevancy_test: bool,
    pub sequence: UPtr<UAnimSequenceBase>,
    pub play_rate_basis: f32,
    pub play_rate: f32,
    pub play_rate_scale_bias_clamp_constants: FInputScaleBiasClampConstants,
    pub start_position: f32,
    pub b_loop_animation: bool,
    pub b_start_from_matching_pose: bool,
}
#[repr(C, align(8))]
pub struct FAnimationActiveTransitionEntry {
    pub blend_profile: UPtr<UBlendProfile>,
}
#[repr(C, align(8))]
pub struct FAnimationPotentialTransition {}
#[repr(C, align(8))]
pub struct FAnimNode_StateMachine {
    pub state_machine_index_in_class: i32,
    pub max_transitions_per_frame: i32,
    pub max_transitions_requests: i32,
    pub b_skip_first_update_transition: bool,
    pub b_reinitialize_on_becoming_relevant: bool,
    pub b_create_notify_meta_data: bool,
    pub b_allow_conduit_entry_states: bool,
}
#[repr(C, align(8))]
pub struct FAnimNode_StateResult {
    pub state_index: i32,
    pub state_entry_function: FAnimNodeFunctionRef,
    pub state_fully_blended_in_function: FAnimNodeFunctionRef,
    pub state_exit_function: FAnimNodeFunctionRef,
    pub state_fully_blended_out_function: FAnimNodeFunctionRef,
}
#[repr(C, align(8))]
pub struct FAnimNode_TransitionPoseEvaluator {
    pub frames_to_cache_pose: i32,
    pub data_source: EEvaluatorDataSource,
    pub evaluator_mode: EEvaluatorMode,
}
#[repr(C, align(8))]
pub struct FAnimNode_TransitionResult {
    pub b_can_enter_transition: bool,
}
#[repr(C, align(8))]
pub struct FAnimNode_UseCachedPose {
    pub link_to_caching_node: FPoseLink,
    pub cache_pose_name: FName,
}
#[repr(C, align(8))]
pub struct FAnimNotifyEventReference {
    pub mirror_table: UPtr<UMirrorDataTable>,
    pub notify_source: UPtr<crate::bindings::core_u_object::UObject>,
}
#[repr(C, align(8))]
pub struct FAnimNotifyArray {
    pub notifies: TArray<FAnimNotifyEventReference>,
}
#[repr(C, align(8))]
pub struct FAnimNotifyContext {}
#[repr(C, align(8))]
pub struct FAnimNotifyQueue {
    pub anim_notifies: TArray<FAnimNotifyEventReference>,
    pub unfiltered_montage_anim_notifies: TMap<FName, FAnimNotifyArray>,
    pub world: TWeakObjectPtr<UWorld>,
}
#[repr(C, align(8))]
pub struct FAnimSequenceTrackContainer {
    pub animation_tracks: TArray<FRawAnimSequenceTrack>,
    pub track_names: TArray<FName>,
}
#[repr(C, align(8))]
pub struct FTranslationTrack {
    pub pos_keys: TArray<crate::bindings::core_u_object::FVector3f>,
    pub times: TArray<f32>,
}
#[repr(C, align(8))]
pub struct FRotationTrack {
    pub rot_keys: TArray<crate::bindings::core_u_object::FQuat4f>,
    pub times: TArray<f32>,
}
#[repr(C, align(8))]
pub struct FScaleTrack {
    pub scale_keys: TArray<crate::bindings::core_u_object::FVector3f>,
    pub times: TArray<f32>,
}
#[repr(C, align(8))]
pub struct FCurveTrack {
    pub curve_name: FName,
    pub curve_weights: TArray<f32>,
}
#[repr(C, align(8))]
pub struct FCompressedTrack {
    pub byte_stream: TArray<u8>,
    pub times: TArray<f32>,
    pub mins: f32,
    pub ranges: f32,
}
#[repr(C, align(8))]
pub struct FAnimSetMeshLinkup {
    pub bone_to_track_table: TArray<i32>,
}
#[repr(C, align(8))]
pub struct FAnimNode_SingleNode {
    pub source_pose: FPoseLink,
}
#[repr(C, align(4))]
pub struct FAnimationTransitionRule {
    pub rule_to_execute: FName,
    pub transition_return_val: bool,
    pub transition_index: i32,
}
#[repr(C, align(4))]
pub struct FAnimationStateBase {
    pub state_name: FName,
}
#[repr(C, align(8))]
pub struct FAnimationState {
    pub transitions: TArray<FAnimationTransitionRule>,
    pub state_root_node_index: i32,
    pub start_notify: i32,
    pub end_notify: i32,
    pub fully_blended_notify: i32,
}
#[repr(C, align(8))]
pub struct FAnimationTransitionBetweenStates {
    pub custom_curve: UPtr<UCurveFloat>,
    pub blend_profile: UPtr<UBlendProfile>,
    pub previous_state: i32,
    pub next_state: i32,
    pub crossfade_duration: f32,
    pub min_time_before_reentry: f32,
    pub start_notify: i32,
    pub end_notify: i32,
    pub interrupt_notify: i32,
    pub blend_mode: EAlphaBlendOption,
    pub logic_type: ETransitionLogicType,
    pub flags_62: u8,
}
#[repr(C, align(8))]
pub struct FBakedStateExitTransition {
    pub can_take_delegate_index: i32,
    pub custom_result_node_index: i32,
    pub transition_index: i32,
    pub b_desired_transition_return_value: bool,
    pub b_automatic_remaining_time_rule: bool,
    pub automatic_rule_trigger_time: f32,
    pub sync_group_name_to_require_valid_markers_rule: FName,
    pub pose_evaluator_links: TArray<i32>,
}
#[repr(C, align(8))]
pub struct FBakedAnimationState {
    pub player_node_indices: TArray<i32>,
    pub layer_node_indices: TArray<i32>,
    pub transitions: TArray<FBakedStateExitTransition>,
    pub state_name: FName,
    pub state_root_node_index: i32,
    pub start_notify: i32,
    pub end_notify: i32,
    pub fully_blended_notify: i32,
    pub entry_rule_node_index: i32,
    pub b_always_reset_on_entry: bool,
    pub b_is_a_conduit: bool,
}
#[repr(C, align(8))]
pub struct FBakedAnimationStateMachine {
    pub machine_name: FName,
    pub initial_state: i32,
    pub states: TArray<FBakedAnimationState>,
    pub transitions: TArray<FAnimationTransitionBetweenStates>,
}
#[repr(C, align(8))]
pub struct FAnimSubsystem_Base {}
#[repr(C, align(8))]
pub struct FAnimSubsystemInstance_NodeRelevancy {}
#[repr(C, align(8))]
pub struct FAnimSubsystem_PropertyAccess {
    pub library: FPropertyAccessLibrary,
}
#[repr(C, align(8))]
pub struct FPropertyAccessLibrary {
    pub path_segments: TArray<FPropertyAccessSegment>,
    pub src_paths: TArray<FPropertyAccessPath>,
    pub dest_paths: TArray<FPropertyAccessPath>,
    pub copy_batches_deprecated: FPropertyAccessCopyBatch,
    pub copy_batch_array: TArray<FPropertyAccessCopyBatch>,
}
#[repr(C, align(8))]
pub struct FPropertyAccessCopyBatch {
    pub copies: TArray<FPropertyAccessCopy>,
}
#[repr(C, align(4))]
pub struct FPropertyAccessCopy {
    pub access_index: i32,
    pub dest_access_start_index: i32,
    pub dest_access_end_index: i32,
    pub ty: EPropertyAccessCopyType,
}
#[repr(C, align(4))]
pub struct FPropertyAccessPath {
    pub path_segment_start_index: i32,
    pub path_segment_count: i32,
}
#[repr(C, align(8))]
pub struct FPropertyAccessSegment {
    pub name: FName,
    pub _struct: UPtr<crate::bindings::core_u_object::UStruct>,
    pub property: TFieldPath<FProperty>,
    pub function: UPtr<crate::bindings::core_u_object::UFunction>,
    pub array_index: i32,
    pub flags: u16,
}
#[repr(C, align(8))]
pub struct FLinkedAnimLayerInstanceData {
    pub instance: UPtr<UAnimInstance>,
    pub linked_functions: TMap<FName, TWeakObjectPtr<UAnimInstance>>,
}
#[repr(C, align(8))]
pub struct FLinkedAnimLayerClassData {
    pub instances_data: TArray<FLinkedAnimLayerInstanceData>,
}
#[repr(C, align(8))]
pub struct FAnimSubsystem_SharedLinkedAnimLayers {
    pub classes_data: TArray<FLinkedAnimLayerClassData>,
    pub persistent_classes: TArray<TSubclassOf<UAnimInstance>>,
}
#[repr(C, align(8))]
pub struct FAnimSubsystem_Tag {
    pub node_indices: TMap<FName, i32>,
}
#[repr(C, align(8))]
pub struct FPerBoneBlendWeights {
    pub bone_blend_weights: TArray<FPerBoneBlendWeight>,
}
#[repr(C, align(4))]
pub struct FAnimSyncMarker {
    pub marker_name: FName,
    pub time: f32,
    pub track_index: i32,
    pub guid: crate::bindings::core_u_object::FGuid,
}
#[repr(C, align(8))]
pub struct FAnimNotifyTrack {
    pub track_name: FName,
    pub track_color: crate::bindings::core_u_object::FLinearColor,
}
#[repr(C, align(8))]
pub struct FMarkerSyncData {
    pub authored_sync_markers: TArray<FAnimSyncMarker>,
}
#[repr(C, align(4))]
pub struct FTrackToSkeletonMap {
    pub bone_tree_index: i32,
}
#[repr(C, align(8))]
pub struct FAssetMapping {
    pub source_asset: UPtr<UAnimationAsset>,
    pub target_asset: UPtr<UAnimationAsset>,
}
#[repr(C, align(8))]
pub struct FAnimationAttributeIdentifierArray {
    pub attribute_identifiers: TArray<FAnimationAttributeIdentifier>,
}
#[repr(C, align(4))]
pub struct FBlendProfileBoneEntry {
    pub bone_reference: FBoneReference,
    pub blend_scale: f32,
}
#[repr(C, align(8))]
pub struct FBlendProfileInterfaceWrapper {
    pub b_is_skeleton_blend_profile: bool,
    pub blend_profile_provider: TScriptInterface<IBlendProfileProviderInterface>,
    pub blend_profile: UPtr<UBlendProfile>,
}
#[repr(C, align(8))]
pub struct FBlendSpaceBlendProfile {
    pub blend_profile: UPtr<UBlendProfile>,
    pub target_weight_interpolation_speed_per_sec: f32,
}
#[repr(C, align(4))]
pub struct FInterpolationParameter {
    pub interpolation_time: f32,
    pub damping_ratio: f32,
    pub max_speed: f32,
    pub interpolation_type: EFilterInterpolationType,
}
#[repr(C, align(8))]
pub struct FBlendParameter {
    pub display_name: FString,
    pub min: f32,
    pub max: f32,
    pub grid_num: i32,
    pub b_snap_to_grid: bool,
    pub b_wrap_input: bool,
}
#[repr(C, align(8))]
pub struct FBlendSample {
    pub animation: UPtr<UAnimSequence>,
    pub sample_value: crate::bindings::core_u_object::FVector,
    pub rate_scale: f32,
    pub b_use_single_frame_for_blending: bool,
    pub frame_index_to_sample: u32,
    pub flags_44: u8,
}
#[repr(C, align(4))]
pub struct FBlendSpaceSegment {
    pub sample_indices: i32,
    pub vertices: f32,
}
#[repr(C, align(8))]
pub struct FBlendSpaceTriangleEdgeInfo {
    pub normal: crate::bindings::core_u_object::FVector2D,
    pub neighbour_triangle_index: i32,
    pub adjacent_perimeter_triangle_indices: i32,
    pub adjacent_perimeter_vertex_indices: i32,
}
#[repr(C, align(8))]
pub struct FBlendSpaceTriangle {
    pub sample_indices: i32,
    pub vertices: crate::bindings::core_u_object::FVector2D,
    pub edge_info: FBlendSpaceTriangleEdgeInfo,
}
#[repr(C, align(4))]
pub struct FWeightedBlendSample {
    pub sample_index: i32,
    pub sample_weight: f32,
}
#[repr(C, align(8))]
pub struct FBlendSpaceData {
    pub segments: TArray<FBlendSpaceSegment>,
    pub triangles: TArray<FBlendSpaceTriangle>,
}
#[repr(C, align(4))]
pub struct FEditorElement {
    pub indices: i32,
    pub weights: f32,
}
#[repr(C, align(4))]
pub struct FGridBlendSample {
    pub grid_element: FEditorElement,
    pub blend_weight: f32,
}
#[repr(C, align(4))]
pub struct FPerBoneInterpolation {
    pub bone_reference: FBoneReference,
    pub interpolation_speed_per_sec: f32,
}
#[repr(C, align(4))]
pub struct FCachedAnimStateData {
    pub state_machine_name: FName,
    pub state_name: FName,
}
#[repr(C, align(8))]
pub struct FCachedAnimStateArray {
    pub states: TArray<FCachedAnimStateData>,
}
#[repr(C, align(4))]
pub struct FCachedAnimAssetPlayerData {
    pub state_machine_name: FName,
    pub state_name: FName,
}
#[repr(C, align(4))]
pub struct FCachedAnimRelevancyData {
    pub state_machine_name: FName,
    pub state_name: FName,
}
#[repr(C, align(4))]
pub struct FCachedAnimTransitionData {
    pub state_machine_name: FName,
    pub from_state_name: FName,
    pub to_state_name: FName,
}
#[repr(C, align(4))]
pub struct FNamedCurveValue {
    pub name: FName,
    pub value: f32,
}
#[repr(C, align(4))]
pub struct FExposedValueCopyRecord {
    pub copy_index: i32,
    pub post_copy_operation: EPostCopyOperation,
    pub b_only_update_when_active: bool,
}
#[repr(C, align(8))]
pub struct FExposedValueHandler {}
#[repr(C, align(8))]
pub struct FAnimNodeExposedValueHandler {}
#[repr(C, align(8))]
pub struct FAnimNodeExposedValueHandler_Base {
    pub function: UPtr<crate::bindings::core_u_object::UFunction>,
    pub bound_function: FName,
}
#[repr(C, align(8))]
pub struct FAnimNodeExposedValueHandler_PropertyAccess {
    pub copy_records: TArray<FExposedValueCopyRecord>,
}
#[repr(C, align(4))]
pub struct FInputClampConstants {
    pub b_clamp_result: bool,
    pub b_interp_result: bool,
    pub clamp_min: f32,
    pub clamp_max: f32,
    pub interp_speed_increasing: f32,
    pub interp_speed_decreasing: f32,
}
#[repr(C, align(4))]
pub struct FInputClampState {}
#[repr(C, align(8))]
pub struct FMirrorTableRow {
    pub name: FName,
    pub mirrored_name: FName,
    pub mirror_entry_type: EMirrorRowType,
}
#[repr(C, align(4))]
pub struct FMirrorFindReplaceExpression {
    pub find_expression: FName,
    pub replace_expression: FName,
    pub find_replace_method: EMirrorFindReplaceMethod,
}
#[repr(C, align(16))]
pub struct FTrajectorySample {
    pub accumulated_seconds: f32,
    pub transform: crate::bindings::core_u_object::FTransform,
    pub linear_velocity: crate::bindings::core_u_object::FVector,
}
#[repr(C, align(16))]
pub struct FNodeItem {
    pub parent_name: FName,
    pub transform: crate::bindings::core_u_object::FTransform,
}
#[repr(C, align(8))]
pub struct FPoseData {
    pub source_local_space_pose: TArray<crate::bindings::core_u_object::FTransform>,
    pub source_curve_data: TArray<f32>,
    pub local_space_pose: TArray<crate::bindings::core_u_object::FTransform>,
    pub curve_data: TArray<f32>,
}
#[repr(C, align(4))]
pub struct FPoseAssetInfluence {
    pub pose_index: i32,
    pub bone_transform_index: i32,
}
#[repr(C, align(8))]
pub struct FPoseAssetInfluences {
    pub influences: TArray<FPoseAssetInfluence>,
}
#[repr(C, align(8))]
pub struct FPoseDataContainer {
    pub pose_names_deprecated: TArray<FSmartName>,
    pub pose_f_names: TArray<FName>,
    pub tracks: TArray<FName>,
    pub track_bone_indices: TArray<i32>,
    pub track_pose_influence_indices: TArray<FPoseAssetInfluences>,
    pub poses: TArray<FPoseData>,
    pub curves: TArray<FAnimCurveBase>,
}
#[repr(C, align(8))]
pub struct FPreviewAttachedObjectPair {
    pub attached_object: TSoftObjectPtr<crate::bindings::core_u_object::UObject>,
    pub object_deprecated: UPtr<crate::bindings::core_u_object::UObject>,
    pub attached_to: FName,
}
#[repr(C, align(8))]
pub struct FPreviewAssetAttachContainer {
    pub attached_objects: TArray<FPreviewAttachedObjectPair>,
}
#[repr(C, align(8))]
pub struct FPreviewMeshCollectionEntry {
    pub skeletal_mesh: TSoftObjectPtr<USkeletalMesh>,
    pub anim_blueprint: TSoftObjectPtr<UAnimBlueprint>,
}
#[repr(C, align(8))]
pub struct FSkeletalMeshVertexAttributeInfo {
    pub enabled_for_render: crate::bindings::core_u_object::FPerPlatformBool,
    pub name: FName,
    pub data_type: ESkeletalMeshVertexAttributeDataType,
}
#[repr(C, align(8))]
pub struct FSkeletonToMeshLinkup {
    pub skeleton_to_mesh_table: TArray<i32>,
    pub mesh_to_skeleton_table: TArray<i32>,
}
#[repr(C, align(4))]
pub struct FBoneNode {
    pub name_deprecated: FName,
    pub parent_index_deprecated: i32,
    pub translation_retargeting_mode: EBoneTranslationRetargetingMode,
}
#[repr(C, align(8))]
pub struct FReferencePose {
    pub pose_name: FName,
    pub reference_pose: TArray<crate::bindings::core_u_object::FTransform>,
    pub source_reference_mesh: TSoftObjectPtr<USkeletalMesh>,
}
#[repr(C, align(8))]
pub struct FBoneReductionSetting {
    pub bones_to_remove: TArray<FName>,
}
#[repr(C, align(4))]
pub struct FNameMapping {
    pub node_name: FName,
    pub bone_name: FName,
}
#[repr(C, align(8))]
pub struct FAnimSlotGroup {
    pub group_name: FName,
    pub slot_names: TArray<FName>,
}
#[repr(C, align(4))]
pub struct FVirtualBone {
    pub source_bone_name: FName,
    pub target_bone_name: FName,
    pub virtual_bone_name: FName,
}
#[repr(C, align(8))]
pub struct FSkinWeightProfileInfo {
    pub name: FName,
    pub default_profile: crate::bindings::core_u_object::FPerPlatformBool,
    pub default_profile_from_lod_index: crate::bindings::core_u_object::FPerPlatformInt,
    pub per_lod_source_files: TMap<i32, FString>,
}
#[repr(C, align(8))]
pub struct FSkinWeightProfileManagerTickFunction {}
#[repr(C, align(8))]
pub struct FSmartNameMapping {}
#[repr(C, align(8))]
pub struct FSmartNameContainer {}
#[repr(C, align(4))]
pub struct FTimeStretchCurveMarker {
    pub time: f32,
    pub alpha: f32,
}
#[repr(C, align(8))]
pub struct FTimeStretchCurve {
    pub sampling_rate: f32,
    pub curve_value_min_precision: f32,
    pub markers: TArray<FTimeStretchCurveMarker>,
    pub sum_d_t_i_by_c_i: f32,
}
#[repr(C, align(8))]
pub struct FTimeStretchCurveInstance {
    pub b_has_valid_data: bool,
}
#[repr(C, align(16))]
pub struct FTransformTrajectorySample {
    pub facing: crate::bindings::core_u_object::FQuat,
    pub position: crate::bindings::core_u_object::FVector,
    pub time_in_seconds: f32,
}
#[repr(C, align(8))]
pub struct FTransformTrajectory {
    pub samples: TArray<FTransformTrajectorySample>,
}
#[repr(C, align(4))]
pub struct FAnimGroupInfo {
    pub name: FName,
    pub color: crate::bindings::core_u_object::FLinearColor,
}
#[repr(C, align(8))]
pub struct FAnimParentNodeAssetOverride {
    pub new_asset: UPtr<UAnimationAsset>,
    pub parent_node_guid: crate::bindings::core_u_object::FGuid,
}
#[repr(C, align(4))]
pub struct FAssetCompileData {
    pub asset: TWeakObjectPtr<crate::bindings::core_u_object::UObject>,
}
#[repr(C, align(8))]
pub struct FPrimaryAssetTypeInfo {
    pub primary_asset_type: FName,
    pub asset_base_class: TSoftObjectPtr<crate::bindings::core_u_object::UClass>,
    pub asset_base_class_loaded: TSubclassOf<crate::bindings::core_u_object::UObject>,
    pub b_has_blueprint_classes: bool,
    pub b_is_editor_only: bool,
    pub directories: TArray<crate::bindings::core_u_object::FDirectoryPath>,
    pub specific_assets: TArray<crate::bindings::core_u_object::FSoftObjectPath>,
    pub rules: FPrimaryAssetRules,
    pub asset_scan_paths: TArray<FString>,
    pub b_is_dynamic_asset: bool,
    pub number_of_assets: i32,
}
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
}
#[repr(C, align(8))]
pub struct FAudioCookOutputs {
    pub id: u32,
    pub version: i32,
    pub num_channels: i32,
    pub sample_rate: i32,
    pub num_frames: u32,
    pub encoded_data: TArray<u8>,
}
#[repr(C, align(8))]
pub struct FAudioEffectParameters {}
#[repr(C, align(8))]
pub struct FAudioReverbEffect {}
#[repr(C, align(8))]
pub struct FAudioQualitySettings {
    pub display_name: FText,
    pub max_channels: i32,
}
#[repr(C, align(8))]
pub struct FSoundDebugEntry {
    pub debug_name: FName,
    pub sound: crate::bindings::core_u_object::FSoftObjectPath,
}
#[repr(C, align(8))]
pub struct FDefaultAudioBusSettings {
    pub audio_bus: crate::bindings::core_u_object::FSoftObjectPath,
}
#[repr(C, align(8))]
pub struct FAudioVolumeSubmixSendSettings {
    pub listener_location_state: EAudioVolumeLocationState,
    pub source_location_state_deprecated: EAudioVolumeLocationState,
    pub submix_sends: TArray<FSoundSubmixSendInfo>,
}
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
#[repr(C, align(8))]
pub struct FSoundSubmixSendInfo {
    pub send_stage: ESubmixSendStage,
}
#[repr(C, align(8))]
pub struct FAudioVolumeSubmixOverrideSettings {
    pub submix: UPtr<USoundSubmix>,
    pub submix_effect_chain: TArray<UPtr<USoundEffectSubmixPreset>>,
    pub crossfade_time: f32,
}
#[repr(C, align(4))]
pub struct FInteriorSettings {
    pub b_is_world_settings: bool,
    pub exterior_volume: f32,
    pub exterior_time: f32,
    pub exterior_lpf: f32,
    pub exterior_lpf_time: f32,
    pub interior_volume: f32,
    pub interior_time: f32,
    pub interior_lpf: f32,
    pub interior_lpf_time: f32,
}
#[repr(C, align(1))]
pub struct FBlueprintMacroCosmeticInfo {}
#[repr(C, align(8))]
pub struct FBPVariableMetaDataEntry {
    pub data_key: FName,
    pub data_value: FString,
}
#[repr(C, align(8))]
pub struct FBPVariableDescription {
    pub var_name: FName,
    pub var_guid: crate::bindings::core_u_object::FGuid,
    pub var_type: FEdGraphPinType,
    pub friendly_name: FString,
    pub category: FText,
    pub property_flags: u64,
    pub rep_notify_func: FName,
    pub replication_condition: crate::bindings::core_u_object::ELifetimeCondition,
    pub meta_data_array: TArray<FBPVariableMetaDataEntry>,
    pub default_value: FString,
}
#[repr(C, align(8))]
pub struct FBPInterfaceDescription {
    pub interface: TSubclassOf<crate::bindings::core_u_object::UInterface>,
    pub graphs: TArray<UPtr<UEdGraph>>,
}
#[repr(C, align(8))]
pub struct FEditedDocumentInfo {
    pub edited_object_path: crate::bindings::core_u_object::FSoftObjectPath,
    pub saved_view_offset: crate::bindings::slate_core::FDeprecateSlateVector2D,
    pub saved_zoom_amount: f32,
    pub edited_object_deprecated: UPtr<crate::bindings::core_u_object::UObject>,
}
#[repr(C, align(8))]
pub struct FBPEditorBookmarkNode {
    pub node_guid: crate::bindings::core_u_object::FGuid,
    pub parent_guid: crate::bindings::core_u_object::FGuid,
    pub display_name: FText,
}
#[repr(C, align(4))]
pub struct FNodeToCodeAssociation {}
#[repr(C, align(8))]
pub struct FDebuggingInfoForSingleFunction {}
#[repr(C, align(8))]
pub struct FPointerToUberGraphFrame {}
#[repr(C, align(8))]
pub struct FBlueprintDebugData {}
#[repr(C, align(8))]
pub struct FEventGraphFastCallPair {
    pub function_to_patch: UPtr<crate::bindings::core_u_object::UFunction>,
    pub event_graph_call_offset: i32,
}
#[repr(C, align(8))]
pub struct FBlueprintComponentChangedPropertyInfo {
    pub property_name: FName,
    pub array_index: i32,
    pub property_scope: UPtr<crate::bindings::core_u_object::UStruct>,
}
#[repr(C, align(8))]
pub struct FBlueprintCookedComponentInstancingData {
    pub changed_property_list: TArray<FBlueprintComponentChangedPropertyInfo>,
    pub b_has_valid_cooked_data: bool,
}
#[repr(C, align(8))]
pub struct FBPComponentClassOverride {
    pub component_name: FName,
    pub component_class: TSubclassOf<crate::bindings::core_u_object::UObject>,
}
#[repr(C, align(1))]
pub struct FBookmarkJumpToSettings {}
#[repr(C, align(4))]
pub struct FGeomSelection {
    pub ty: i32,
    pub index: i32,
    pub selection_index: i32,
}
#[repr(C, align(8))]
pub struct FBuilderPoly {
    pub vertex_indices: TArray<i32>,
    pub direction: i32,
    pub item_name: FName,
    pub poly_flags: i32,
}
#[repr(C, align(8))]
pub struct FCameraLensInterfaceClassSupport {
    pub class: TSubclassOf<AActor>,
}
#[repr(C, align(8))]
pub struct FPooledCameraShakes {
    pub pooled_shakes: TArray<UPtr<UCameraShakeBase>>,
}
#[repr(C, align(8))]
pub struct FActiveCameraShakeInfo {
    pub shake_instance: UPtr<UCameraShakeBase>,
    pub shake_source: TWeakObjectPtr<UCameraShakeSourceComponent>,
    pub b_is_custom_initialized: bool,
}
#[repr(C, align(4))]
pub struct FCameraShakePatternStartParams {
    pub b_is_restarting: bool,
    pub b_override_duration: bool,
    pub duration_override: f32,
}
#[repr(C, align(16))]
pub struct FCameraShakePatternUpdateParams {
    pub delta_time: f32,
    pub shake_scale: f32,
    pub dynamic_scale: f32,
    pub pov: FMinimalViewInfo,
}
#[repr(C, align(16))]
pub struct FCameraShakePatternScrubParams {
    pub absolute_time: f32,
    pub shake_scale: f32,
    pub dynamic_scale: f32,
    pub pov: FMinimalViewInfo,
}
#[repr(C, align(16))]
pub struct FCameraShakePatternUpdateResult {}
#[repr(C, align(1))]
pub struct FCameraShakePatternStopParams {
    pub b_immediately: bool,
}
#[repr(C, align(4))]
pub struct FCameraShakeDuration {
    pub duration: f32,
    pub ty: ECameraShakeDurationType,
}
#[repr(C, align(4))]
pub struct FCameraShakeInfo {
    pub duration: FCameraShakeDuration,
    pub blend_in: f32,
    pub blend_out: f32,
}
#[repr(C, align(1))]
pub struct FDummySpacerCameraTypes {}
#[repr(C, align(8))]
pub struct FRepRootMotionMontage {
    pub animation: UPtr<UAnimSequenceBase>,
    pub b_is_active: bool,
    pub b_relative_position: bool,
    pub b_relative_rotation: bool,
    pub position: f32,
    pub location: FVector_NetQuantize100,
    pub rotation: crate::bindings::core_u_object::FRotator,
    pub movement_base: UPtr<UPrimitiveComponent>,
    pub movement_base_bone_name: FName,
    pub authoritative_root_motion: FRootMotionSourceGroup,
    pub acceleration: FVector_NetQuantize10,
    pub linear_velocity: FVector_NetQuantize10,
}
#[repr(C, align(8))]
pub struct FRootMotionSourceGroup {
    pub flags_40: u8,
    pub last_accumulated_settings: FRootMotionSourceSettings,
    pub last_pre_additive_velocity: FVector_NetQuantize10,
}
#[repr(C, align(1))]
pub struct FRootMotionSourceSettings {
    pub flags: u8,
}
#[repr(C, align(8))]
pub struct FSimulatedRootMotionReplicatedMove {
    pub time: f32,
    pub root_motion: FRepRootMotionMontage,
}
#[repr(C, align(8))]
pub struct FBasedMovementInfo {
    pub base_id: u16,
    pub flags_2: u8,
    pub bone_name: FName,
    pub movement_base: UPtr<UPrimitiveComponent>,
    pub location: FVector_NetQuantize100,
    pub rotation: crate::bindings::core_u_object::FRotator,
}
#[repr(C, align(8))]
pub struct FFindFloorResult {
    pub flags_0: u8,
    pub floor_dist: f32,
    pub line_dist: f32,
    pub hit_result: FHitResult,
}
#[repr(C, align(8))]
pub struct FCollisionResponseTemplate {
    pub name: FName,
    pub collision_enabled: ECollisionEnabled,
    pub b_can_modify: bool,
    pub object_type_name: FName,
    pub custom_responses: TArray<FResponseChannel>,
    pub help_message: FString,
}
#[repr(C, align(4))]
pub struct FCustomChannelSetup {
    pub channel: ECollisionChannel,
    pub default_response: ECollisionResponse,
    pub b_trace_type: bool,
    pub b_static_object: bool,
    pub name: FName,
}
#[repr(C, align(8))]
pub struct FCustomProfile {
    pub name: FName,
    pub custom_responses: TArray<FResponseChannel>,
}
#[repr(C, align(4))]
pub struct FBlueprintComponentDelegateBinding {
    pub component_property_name: FName,
    pub delegate_property_name: FName,
    pub function_name_to_bind: FName,
}
#[repr(C, align(8))]
pub struct FActorComponentInstanceSourceInfo {
    pub source_component_template: UPtr<crate::bindings::core_u_object::UObject>,
    pub source_component_creation_method: EComponentCreationMethod,
    pub source_component_type_serialized_index: i32,
}
#[repr(C, align(8))]
pub struct FActorInstanceData {
    pub actor_class: TSubclassOf<AActor>,
}
#[repr(C, align(8))]
pub struct FAudioComponentParam {
    pub sound_wave_param: UPtr<USoundWave>,
}
#[repr(C, align(8))]
pub struct FCharacterMovementComponentPostPhysicsTickFunction {}
#[repr(C, align(8))]
pub struct FCharacterMovementComponentPrePhysicsTickFunction {}
#[repr(C, align(16))]
pub struct FChildActorAttachedActorInfo {
    pub actor: TWeakObjectPtr<AActor>,
    pub socket_name: FName,
    pub relative_transform: crate::bindings::core_u_object::FTransform,
}
#[repr(C, align(8))]
pub struct FChildActorComponentInstanceData {
    pub child_actor_class: TSubclassOf<AActor>,
    pub child_actor_name: FName,
    pub attached_actors: TArray<FChildActorAttachedActorInfo>,
}
#[repr(C, align(8))]
pub struct FCachedKeyToActionInfo {
    pub player_input: TWeakObjectPtr<UPlayerInput>,
}
#[repr(C, align(16))]
pub struct FSkinnedMeshInstanceData {
    pub transform: crate::bindings::core_u_object::FTransform3f,
    pub animation_index: u32,
}
#[repr(C, align(8))]
pub struct FInstancedSkinnedMeshComponentInstanceData {
    pub skinned_asset: UPtr<USkinnedAsset>,
    pub instance_data: TArray<FSkinnedMeshInstanceData>,
    pub b_has_per_instance_hit_proxies: bool,
    pub primitive_bounds_override: crate::bindings::core_u_object::FBox,
    pub b_is_instance_data_gpu_only: bool,
    pub num_instances_gpu_only: i32,
}
#[repr(C, align(16))]
pub struct FPrecomputedLightInstanceData {
    pub transform: crate::bindings::core_u_object::FTransform,
    pub original_light_guid: crate::bindings::core_u_object::FGuid,
    pub light_guid: crate::bindings::core_u_object::FGuid,
    pub preview_shadow_map_channel: i32,
}
#[repr(C, align(8))]
pub struct FBatchedLine {
    pub start: crate::bindings::core_u_object::FVector,
    pub end: crate::bindings::core_u_object::FVector,
    pub color: crate::bindings::core_u_object::FLinearColor,
    pub thickness: f32,
    pub remaining_life_time: f32,
    pub depth_priority: u8,
    pub batch_id: u32,
}
#[repr(C, align(8))]
pub struct FBatchedPoint {
    pub position: crate::bindings::core_u_object::FVector,
    pub color: crate::bindings::core_u_object::FLinearColor,
    pub point_size: f32,
    pub remaining_life_time: f32,
    pub depth_priority: u8,
    pub batch_id: u32,
}
#[repr(C, align(8))]
pub struct FLODMappingData {
    pub mapping: TArray<i32>,
    pub inverse_mapping: TArray<i32>,
}
#[repr(C, align(4))]
pub struct FComponentSync {
    pub name: FName,
    pub sync_option: ESyncOption,
}
#[repr(C, align(8))]
pub struct FMaterialSpriteElement {
    pub material: UPtr<UMaterialInterface>,
    pub distance_to_opacity_curve: UPtr<UCurveFloat>,
    pub flags_16: u8,
    pub base_size_x: f32,
    pub base_size_y: f32,
    pub distance_to_size_curve: UPtr<UCurveFloat>,
}
#[repr(C, align(8))]
pub struct FSpriteCategoryInfo {
    pub category: FName,
    pub display_name: FText,
    pub description: FText,
}
#[repr(C, align(16))]
pub struct FPrimitiveComponentInstanceData {
    pub component_transform: crate::bindings::core_u_object::FTransform,
    pub visibility_id: i32,
    pub lod_parent: UPtr<UPrimitiveComponent>,
}
#[repr(C, align(8))]
pub struct FEngineShowFlagsSetting {
    pub show_flag_name: FString,
    pub enabled: bool,
}
#[repr(C, align(8))]
pub struct FSkeletalMeshComponentEndPhysicsTickFunction {}
#[repr(C, align(8))]
pub struct FSkeletalMeshComponentClothTickFunction {}
#[repr(C, align(8))]
pub struct FSkeletalMeshComponentInstanceData {}
#[repr(C, align(4))]
pub struct FSkelMeshSkinWeightInfo {
    pub bones: i32,
    pub weights: u8,
}
#[repr(C, align(8))]
pub struct FSkelMeshComponentLODInfo {
    pub hidden_materials: TArray<bool>,
}
#[repr(C, align(4))]
pub struct FVertexOffsetUsage {
    pub usage: i32,
}
#[repr(C, align(8))]
pub struct FMeshDeformerInstanceSet {
    pub deformer_instances: TArray<UPtr<UMeshDeformerInstance>>,
}
#[repr(C, align(4))]
pub struct FTentDistribution {
    pub tip_altitude: f32,
    pub tip_value: f32,
    pub width: f32,
}
#[repr(C, align(16))]
pub struct FPrecomputedSkyLightInstanceData {
    pub original_light_guid: crate::bindings::core_u_object::FGuid,
    pub light_guid: crate::bindings::core_u_object::FGuid,
    pub average_brightness: f32,
}
#[repr(C, align(8))]
pub struct FSplineCurves {
    pub position: crate::bindings::core_u_object::FInterpCurveVector,
    pub rotation: crate::bindings::core_u_object::FInterpCurveQuat,
    pub scale: crate::bindings::core_u_object::FInterpCurveVector,
    pub reparam_table: crate::bindings::core_u_object::FInterpCurveFloat,
    pub metadata_deprecated: UPtr<USplineMetadata>,
    pub version: u32,
}
#[repr(C, align(8))]
pub struct FSplinePoint {
    pub input_key: f32,
    pub position: crate::bindings::core_u_object::FVector,
    pub arrive_tangent: crate::bindings::core_u_object::FVector,
    pub leave_tangent: crate::bindings::core_u_object::FVector,
    pub rotation: crate::bindings::core_u_object::FRotator,
    pub scale: crate::bindings::core_u_object::FVector,
    pub ty: ESplinePointType,
}
#[repr(C, align(8))]
pub struct FSplineInstanceData {
    pub b_spline_has_been_edited: bool,
    pub spline_curves: FSplineCurves,
    pub b_closed_loop: bool,
    pub spline_curves_pre_ucs: FSplineCurves,
}
#[repr(C, align(8))]
pub struct FSplineComponentInstanceData {
    pub b_spline_has_been_edited: bool,
    pub spline: FSpline,
    pub b_closed_loop: bool,
    pub spline_pre_ucs: FSpline,
}
#[repr(C, align(8))]
pub struct FSpline {}
#[repr(C, align(8))]
pub struct FSplineMeshParams {
    pub start_pos: crate::bindings::core_u_object::FVector,
    pub start_tangent: crate::bindings::core_u_object::FVector,
    pub start_scale: crate::bindings::core_u_object::FVector2D,
    pub start_roll: f32,
    pub end_roll: f32,
    pub start_offset: crate::bindings::core_u_object::FVector2D,
    pub end_pos: crate::bindings::core_u_object::FVector,
    pub end_scale: crate::bindings::core_u_object::FVector2D,
    pub end_tangent: crate::bindings::core_u_object::FVector,
    pub end_offset: crate::bindings::core_u_object::FVector2D,
    pub nanite_cluster_bounds_scale: f32,
}
#[repr(C, align(16))]
pub struct FStaticMeshComponentInstanceData {
    pub static_mesh: UPtr<UStaticMesh>,
    pub vertex_color_lo_ds: TArray<FStaticMeshVertexColorLODData>,
    pub cached_static_lighting: TArray<crate::bindings::core_u_object::FGuid>,
    pub streaming_texture_data: TArray<FStreamingTextureBuildInfo>,
    pub material_streaming_relative_boxes: TArray<u32>,
}
#[repr(C, align(8))]
pub struct FStaticMeshVertexColorLODData {
    pub painted_vertices: TArray<FPaintedVertex>,
    pub vertex_buffer_colors: TArray<crate::bindings::core_u_object::FColor>,
    pub lod_index: u32,
}
#[repr(C, align(16))]
pub struct FPaintedVertex {
    pub position: crate::bindings::core_u_object::FVector,
    pub color: crate::bindings::core_u_object::FColor,
    pub normal: crate::bindings::core_u_object::FVector4,
}
#[repr(C, align(16))]
pub struct FSplineMeshInstanceData {
    pub start_pos: crate::bindings::core_u_object::FVector,
    pub end_pos: crate::bindings::core_u_object::FVector,
    pub start_tangent: crate::bindings::core_u_object::FVector,
    pub end_tangent: crate::bindings::core_u_object::FVector,
}
#[repr(C, align(8))]
pub struct FEquirectProps {
    pub left_uv_rect: crate::bindings::core_u_object::FBox2D,
    pub right_uv_rect: crate::bindings::core_u_object::FBox2D,
    pub left_scale: crate::bindings::core_u_object::FVector2D,
    pub right_scale: crate::bindings::core_u_object::FVector2D,
    pub left_bias: crate::bindings::core_u_object::FVector2D,
    pub right_bias: crate::bindings::core_u_object::FVector2D,
    pub radius: f32,
}
#[repr(C, align(4))]
pub struct FCullDistanceSizePair {
    pub size: f32,
    pub cull_distance: f32,
}
#[repr(C, align(8))]
pub struct FRuntimeCurveLinearColor {
    pub color_curves: FRichCurve,
    pub external_curve: UPtr<UCurveLinearColor>,
}
#[repr(C, align(4))]
pub struct FCurveAtlasColorAdjustments {
    pub flags_0: u8,
    pub adjust_brightness: f32,
    pub adjust_brightness_curve: f32,
    pub adjust_vibrance: f32,
    pub adjust_saturation: f32,
    pub adjust_rgb_curve: f32,
    pub adjust_hue: f32,
    pub adjust_min_alpha: f32,
    pub adjust_max_alpha: f32,
}
#[repr(C, align(8))]
pub struct FRuntimeVectorCurve {
    pub vector_curves: FRichCurve,
    pub external_curve: UPtr<UCurveVector>,
}
#[repr(C, align(4))]
pub struct FNameCurveKey {
    pub time: f32,
    pub value: FName,
}
#[repr(C, align(8))]
pub struct FNameCurve {
    pub keys: TArray<FNameCurveKey>,
}
#[repr(C, align(8))]
pub struct FCompressedRichCurve {}
#[repr(C, align(8))]
pub struct FCurveTableRowHandle {
    pub curve_table: UPtr<UCurveTable>,
    pub row_name: FName,
}
#[repr(C, align(8))]
pub struct FDataDrivenConsoleVariable {
    pub ty: FDataDrivenCVarType,
    pub name: FString,
    pub tool_tip: FString,
    pub default_value_float: f32,
    pub default_value_int: i32,
    pub default_value_bool: bool,
}
#[repr(C, align(8))]
pub struct FDataTableRowHandle {
    pub data_table: UPtr<UDataTable>,
    pub row_name: FName,
}
#[repr(C, align(8))]
pub struct FDataTableCategoryHandle {
    pub data_table: UPtr<UDataTable>,
    pub column_name: FName,
    pub row_contents: FName,
}
#[repr(C, align(8))]
pub struct FDebugCameraControllerSettingsViewModeIndex {
    pub view_mode_index: EViewModeIndex,
}
#[repr(C, align(8))]
pub struct FRollbackNetStartupActorInfo {
    pub archetype: UPtr<crate::bindings::core_u_object::UObject>,
    pub obj_references: TArray<UPtr<crate::bindings::core_u_object::UObject>>,
}
#[repr(C, align(8))]
pub struct FMulticastRecordOptions {
    pub func_path_name: FString,
    pub b_server_skip: bool,
    pub b_client_skip: bool,
}
#[repr(C, align(8))]
pub struct FSelectedFragmentProperties {
    pub tag: FName,
    pub fragment: FString,
    pub b_enabled: bool,
}
#[repr(C, align(8))]
pub struct FDPMatchingIfCondition {
    pub operator: FName,
    pub arg1: FString,
    pub arg2: FString,
}
#[repr(C, align(8))]
pub struct FDPMatchingRulestructBase {
    pub rule_name: FString,
    pub if_conditions: TArray<FDPMatchingIfCondition>,
    pub append_fragments: FString,
    pub set_user_var: FString,
}
#[repr(C, align(8))]
pub struct FDPMatchingRulestructA {
    pub on_true: TArray<FDPMatchingRulestructBase>,
    pub on_false: TArray<FDPMatchingRulestructBase>,
}
#[repr(C, align(8))]
pub struct FDPMatchingRulestructB {
    pub on_true: TArray<FDPMatchingRulestructA>,
    pub on_false: TArray<FDPMatchingRulestructA>,
}
#[repr(C, align(8))]
pub struct FDPMatchingRulestructC {
    pub on_true: TArray<FDPMatchingRulestructB>,
    pub on_false: TArray<FDPMatchingRulestructB>,
}
#[repr(C, align(8))]
pub struct FDPMatchingRulestructD {
    pub on_true: TArray<FDPMatchingRulestructC>,
    pub on_false: TArray<FDPMatchingRulestructC>,
}
#[repr(C, align(8))]
pub struct FDPMatchingRulestructE {
    pub on_true: TArray<FDPMatchingRulestructD>,
    pub on_false: TArray<FDPMatchingRulestructD>,
}
#[repr(C, align(8))]
pub struct FDPMatchingRulestruct {
    pub on_true: TArray<FDPMatchingRulestructE>,
    pub on_false: TArray<FDPMatchingRulestructE>,
}
#[repr(C, align(8))]
pub struct FDialogueContext {
    pub speaker: UPtr<UDialogueVoice>,
    pub targets: TArray<UPtr<UDialogueVoice>>,
}
#[repr(C, align(8))]
pub struct FDialogueWaveParameter {
    pub dialogue_wave: UPtr<UDialogueWave>,
    pub context: FDialogueContext,
}
#[repr(C, align(8))]
pub struct FDialogueContextMapping {
    pub context: FDialogueContext,
    pub sound_wave: UPtr<USoundWave>,
    pub localization_key_format: FString,
    pub proxy: UPtr<UDialogueSoundWaveProxy>,
}
#[repr(C, align(8))]
pub struct FGraphReference {
    pub macro_graph: UPtr<UEdGraph>,
    pub graph_blueprint: UPtr<UBlueprint>,
    pub graph_guid: crate::bindings::core_u_object::FGuid,
}
#[repr(C, align(4))]
pub struct FEdGraphPinReference {
    pub owning_node: TWeakObjectPtr<UEdGraphNode>,
    pub pin_id: crate::bindings::core_u_object::FGuid,
}
#[repr(C, align(8))]
pub struct FEdGraphSchemaAction_NewNode {
    pub node_template: UPtr<UEdGraphNode>,
}
#[repr(C, align(8))]
pub struct FAssetImportInfo {}
#[repr(C, align(8))]
pub struct FTypedElementPasteOptions {
    pub selection_set_to_modify: UPtr<
        crate::bindings::typed_element_runtime::UTypedElementSelectionSet,
    >,
    pub b_paste_at_location: bool,
    pub paste_location: crate::bindings::core_u_object::FVector,
}
#[repr(C, align(1))]
pub struct FTypedElementDeletionOptions {
    pub b_verify_deletion_can_happen: bool,
    pub b_warn_about_references: bool,
    pub b_warn_about_soft_references: bool,
}
#[repr(C, align(1))]
pub struct FViewLightingChannels {
    pub flags_0: u8,
}
#[repr(C, align(2))]
pub struct FMaterialShadingModelField {
    pub shading_model_field: u16,
}
#[repr(C, align(4))]
pub struct FRigidBodyErrorCorrection {
    pub ping_extrapolation: f32,
    pub ping_limit: f32,
    pub error_per_linear_difference: f32,
    pub error_per_angular_difference: f32,
    pub max_restored_state_error: f32,
    pub max_linear_hard_snap_distance: f32,
    pub position_lerp: f32,
    pub angle_lerp: f32,
    pub linear_velocity_coefficient: f32,
    pub angular_velocity_coefficient: f32,
    pub error_accumulation_seconds: f32,
    pub error_accumulation_distance_sq: f32,
    pub error_accumulation_similarity: f32,
}
#[repr(C, align(8))]
pub struct FRigidBodyContactInfo {
    pub contact_position: crate::bindings::core_u_object::FVector,
    pub contact_normal: crate::bindings::core_u_object::FVector,
    pub contact_penetration: f32,
    pub b_contact_probe: bool,
    pub phys_material: UPtr<crate::bindings::physics_core::UPhysicalMaterial>,
}
#[repr(C, align(8))]
pub struct FCollisionImpactData {
    pub contact_infos: TArray<FRigidBodyContactInfo>,
    pub total_normal_impulse: crate::bindings::core_u_object::FVector,
    pub total_friction_impulse: crate::bindings::core_u_object::FVector,
    pub b_is_velocity_delta_under_threshold: bool,
}
#[repr(C, align(8))]
pub struct FFractureEffect {
    pub particle_system: UPtr<UParticleSystem>,
    pub sound: UPtr<USoundBase>,
}
#[repr(C, align(8))]
pub struct FBasedPosition {
    pub base: UPtr<AActor>,
    pub position: crate::bindings::core_u_object::FVector,
    pub cached_base_location: crate::bindings::core_u_object::FVector,
    pub cached_base_rotation: crate::bindings::core_u_object::FRotator,
    pub cached_trans_position: crate::bindings::core_u_object::FVector,
}
#[repr(C, align(4))]
pub struct FLightmassLightSettings {
    pub indirect_lighting_saturation: f32,
    pub shadow_exponent: f32,
    pub b_use_area_shadows_for_stationary_light: bool,
}
#[repr(C, align(4))]
pub struct FLightmassPointLightSettings {}
#[repr(C, align(4))]
pub struct FLightmassDirectionalLightSettings {
    pub light_source_angle: f32,
}
#[repr(C, align(4))]
pub struct FLightmassPrimitiveSettings {
    pub flags_0: u8,
    pub emissive_light_falloff_exponent: f32,
    pub emissive_light_explicit_influence_radius: f32,
    pub emissive_boost: f32,
    pub diffuse_boost: f32,
    pub fully_occluded_samples_fraction: f32,
}
#[repr(C, align(4))]
pub struct FLightmassDebugOptions {
    pub flags_0: u8,
    pub coplanar_tolerance: f32,
    pub flags_8: u8,
    pub flags_9: u8,
    pub execution_time_divisor: f32,
}
#[repr(C, align(4))]
pub struct FSwarmDebugOptions {
    pub flags_0: u8,
}
#[repr(C, align(8))]
pub struct FMTDResult {
    pub direction: crate::bindings::core_u_object::FVector,
    pub distance: f32,
}
#[repr(C, align(4))]
pub struct FAnimSlotDesc {
    pub slot_name: FName,
    pub num_channels: i32,
}
#[repr(C, align(8))]
pub struct FAnimUpdateRateParameters {
    pub shift_bucket: EUpdateRateShiftBucket,
    pub flags_2: u8,
    pub update_rate: i32,
    pub evaluation_rate: i32,
    pub ticked_pose_offest_time: f32,
    pub additional_time: f32,
    pub base_non_rendered_update_rate: i32,
    pub max_eval_rate_for_interpolation: i32,
    pub base_visible_distance_factor_thesholds: TArray<f32>,
    pub lod_to_frame_skip_map: TMap<i32, i32>,
    pub skipped_update_frames: i32,
    pub skipped_eval_frames: i32,
}
#[repr(C, align(8))]
pub struct FPOV {
    pub location: crate::bindings::core_u_object::FVector,
    pub rotation: crate::bindings::core_u_object::FRotator,
    pub fov: f32,
}
#[repr(C, align(8))]
pub struct FMeshBuildSettings {
    pub flags_0: u8,
    pub flags_1: u8,
    pub min_lightmap_resolution: i32,
    pub src_lightmap_index: i32,
    pub dst_lightmap_index: i32,
    pub build_scale_deprecated: f32,
    pub build_scale3_d: crate::bindings::core_u_object::FVector,
    pub distance_field_resolution_scale: f32,
    pub distance_field_bias_deprecated: f32,
    pub distance_field_replacement_mesh: UPtr<UStaticMesh>,
    pub max_lumen_mesh_cards: i32,
}
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
#[repr(C, align(8))]
pub struct FMeshDisplacementMap {
    pub texture: UPtr<UTexture2D>,
    pub magnitude: f32,
    pub center: f32,
}
#[repr(C, align(8))]
pub struct FMeshNaniteSettings {
    pub flags_0: u8,
    pub shape_preservation: ENaniteShapePreservation,
    pub position_precision: i32,
    pub normal_precision: i32,
    pub tangent_precision: i32,
    pub bone_weight_precision: i32,
    pub target_minimum_residency_in_kb: u32,
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
    pub displacement_maps: TArray<FMeshDisplacementMap>,
    pub nanite_assembly_data: FNaniteAssemblyData,
}
#[repr(C, align(4))]
pub struct FMeshRayTracingProxySettings {
    pub flags_0: u8,
    pub fallback_target: ENaniteFallbackTarget,
    pub fallback_percent_triangles: f32,
    pub fallback_relative_error: f32,
    pub lod1_percent_triangles: f32,
    pub foliage_over_occlusion_bias: f32,
}
#[repr(C, align(4))]
pub struct FConstrainComponentPropName {
    pub component_name: FName,
}
#[repr(C, align(4))]
pub struct FCollectionReference {
    pub collection_name: FName,
}
#[repr(C, align(4))]
pub struct FRedirector {
    pub old_name: FName,
    pub new_name: FName,
}
#[repr(C, align(8))]
pub struct FDepthFieldGlowInfo {
    pub flags_0: u8,
    pub glow_color: crate::bindings::core_u_object::FLinearColor,
    pub glow_outer_radius: crate::bindings::core_u_object::FVector2D,
    pub glow_inner_radius: crate::bindings::core_u_object::FVector2D,
}
#[repr(C, align(8))]
pub struct FFontRenderInfo {
    pub flags_0: u8,
    pub glow_info: FDepthFieldGlowInfo,
}
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
#[repr(C, align(8))]
pub struct FDamageEvent {
    pub damage_type_class: TSubclassOf<UDamageType>,
}
#[repr(C, align(8))]
pub struct FPointDamageEvent {
    pub damage: f32,
    pub shot_direction: FVector_NetQuantizeNormal,
    pub hit_info: FHitResult,
}
#[repr(C, align(4))]
pub struct FRadialDamageParams {
    pub base_damage: f32,
    pub minimum_damage: f32,
    pub inner_radius: f32,
    pub outer_radius: f32,
    pub damage_falloff: f32,
}
#[repr(C, align(8))]
pub struct FRadialDamageEvent {
    pub params: FRadialDamageParams,
    pub origin: crate::bindings::core_u_object::FVector,
    pub component_hits: TArray<FHitResult>,
}
#[repr(C, align(8))]
pub struct FHitResultNetSerializerConfig {}
#[repr(C, align(8))]
pub struct FOverlapResult {
    pub overlap_object_handle: FActorInstanceHandle,
    pub component: TWeakObjectPtr<UPrimitiveComponent>,
    pub flags_64: u8,
}
#[repr(C, align(16))]
pub struct FRigidBodyState {
    pub position: FVector_NetQuantize100,
    pub quaternion: crate::bindings::core_u_object::FQuat,
    pub lin_vel: FVector_NetQuantize100,
    pub ang_vel: FVector_NetQuantize100,
    pub flags: u8,
}
#[repr(C, align(4))]
pub struct FFontCharacter {
    pub start_u: i32,
    pub start_v: i32,
    pub u_size: i32,
    pub v_size: i32,
    pub texture_index: u8,
    pub vertical_offset: i32,
}
#[repr(C, align(1))]
pub struct FFontFacePlatformRasterizationOverrides {
    pub msdf_override: crate::bindings::slate_core::EFontRasterizationMode,
    pub sdf_override: crate::bindings::slate_core::EFontRasterizationMode,
    pub sdf_approximation_override: crate::bindings::slate_core::EFontRasterizationMode,
}
#[repr(C, align(8))]
pub struct FCharacterNetworkSerializationPackedBitsNetSerializerConfig {
    pub max_allowed_data_bits: u32,
}
#[repr(C, align(8))]
pub struct FForceFeedbackChannelDetails {
    pub flags_0: u8,
    pub curve: FRuntimeFloatCurve,
}
#[repr(C, align(8))]
pub struct FActiveForceFeedbackEffect {
    pub force_feedback_effect: UPtr<UForceFeedbackEffect>,
    pub active_device_properties: TSet<FInputDevicePropertyHandle>,
}
#[repr(C, align(4))]
pub struct FInputDevicePropertyHandle {
    pub internal_id: u32,
}
#[repr(C, align(8))]
pub struct FForceFeedbackEffectOverridenChannelDetails {
    pub channel_details: TArray<FForceFeedbackChannelDetails>,
}
#[repr(C, align(4))]
pub struct FDeviceColorData {
    pub b_enable: bool,
    pub b_reset_after_completion: bool,
    pub light_color: crate::bindings::core_u_object::FColor,
}
#[repr(C, align(8))]
pub struct FDeviceColorCurveData {
    pub b_enable: bool,
    pub b_reset_after_completion: bool,
    pub device_color_curve: UPtr<UCurveLinearColor>,
}
#[repr(C, align(1))]
pub struct FDeviceTriggerBaseData {
    pub affected_triggers: crate::bindings::core_u_object::EInputDeviceTriggerMask,
    pub b_reset_upon_completion: bool,
}
#[repr(C, align(8))]
pub struct FDeviceTriggerFeedbackData {
    pub feedback_position_curve: UPtr<UCurveFloat>,
    pub feedback_strengh_curve: UPtr<UCurveFloat>,
}
#[repr(C, align(4))]
pub struct FDeviceTriggerTriggerResistanceData {
    pub start_position: i32,
    pub start_strengh: i32,
    pub end_position: i32,
    pub end_strengh: i32,
}
#[repr(C, align(8))]
pub struct FDeviceTriggerTriggerVibrationData {
    pub trigger_position_curve: UPtr<UCurveFloat>,
    pub vibration_frequency_curve: UPtr<UCurveFloat>,
    pub vibration_amplitude_curve: UPtr<UCurveFloat>,
}
#[repr(C, align(8))]
pub struct FAudioBasedVibrationData {
    pub sound: UPtr<USoundBase>,
}
#[repr(C, align(4))]
pub struct FActivateDevicePropertyParams {
    pub user_id: crate::bindings::core_u_object::FPlatformUserId,
    pub device_id: crate::bindings::core_u_object::FInputDeviceId,
    pub flags_8: u8,
}
#[repr(C, align(8))]
pub struct FActiveDeviceProperty {
    pub property: TWeakObjectPtr<UInputDeviceProperty>,
}
#[repr(C, align(1))]
pub struct FRootMotionSourceStatus {
    pub flags: u8,
}
#[repr(C, align(8))]
pub struct FRootMotionFinishVelocitySettings {
    pub mode: ERootMotionFinishVelocityMode,
    pub clamp_velocity: f32,
    pub set_velocity: crate::bindings::core_u_object::FVector,
}
#[repr(C, align(16))]
pub struct FRootMotionSource {
    pub priority: u16,
    pub local_id: u16,
    pub instance_name: FName,
    pub start_time: f32,
    pub current_time: f32,
    pub previous_time: f32,
    pub duration: f32,
    pub status: FRootMotionSourceStatus,
    pub settings: FRootMotionSourceSettings,
    pub accumulate_mode: ERootMotionAccumulateMode,
    pub b_in_local_space: bool,
    pub root_motion_params: FRootMotionMovementParams,
    pub finish_velocity_params: FRootMotionFinishVelocitySettings,
}
#[repr(C, align(16))]
pub struct FRootMotionSource_ConstantForce {
    pub force: crate::bindings::core_u_object::FVector,
    pub strength_over_time: UPtr<UCurveFloat>,
}
#[repr(C, align(16))]
pub struct FRootMotionSource_RadialForce {
    pub location: crate::bindings::core_u_object::FVector,
    pub location_actor: UPtr<AActor>,
    pub radius: f32,
    pub strength: f32,
    pub b_is_push: bool,
    pub b_no_z_force: bool,
    pub strength_distance_falloff: UPtr<UCurveFloat>,
    pub strength_over_time: UPtr<UCurveFloat>,
    pub b_use_fixed_world_direction: bool,
    pub fixed_world_direction: crate::bindings::core_u_object::FRotator,
}
#[repr(C, align(16))]
pub struct FRootMotionSource_MoveToForce {
    pub start_location: crate::bindings::core_u_object::FVector,
    pub target_location: crate::bindings::core_u_object::FVector,
    pub b_restrict_speed_to_expected: bool,
    pub path_offset_curve: UPtr<UCurveVector>,
}
#[repr(C, align(16))]
pub struct FRootMotionSource_MoveToDynamicForce {
    pub start_location: crate::bindings::core_u_object::FVector,
    pub initial_target_location: crate::bindings::core_u_object::FVector,
    pub target_location: crate::bindings::core_u_object::FVector,
    pub b_restrict_speed_to_expected: bool,
    pub path_offset_curve: UPtr<UCurveVector>,
    pub time_mapping_curve: UPtr<UCurveFloat>,
}
#[repr(C, align(16))]
pub struct FRootMotionSource_JumpForce {
    pub rotation: crate::bindings::core_u_object::FRotator,
    pub distance: f32,
    pub height: f32,
    pub b_disable_timeout: bool,
    pub path_offset_curve: UPtr<UCurveVector>,
    pub time_mapping_curve: UPtr<UCurveFloat>,
}
#[repr(C, align(8))]
pub struct FRootMotionSourceGroupNetSerializerConfig {}
#[repr(C, align(8))]
pub struct FTouchInputControl {
    pub b_treat_as_button: bool,
    pub image1: UPtr<UTexture2D>,
    pub image2: UPtr<UTexture2D>,
    pub center: crate::bindings::core_u_object::FVector2D,
    pub visual_size: crate::bindings::core_u_object::FVector2D,
    pub thumb_size: crate::bindings::core_u_object::FVector2D,
    pub interaction_size: crate::bindings::core_u_object::FVector2D,
    pub input_scale: crate::bindings::core_u_object::FVector2D,
    pub main_input_key: crate::bindings::input_core::FKey,
    pub alt_input_key: crate::bindings::input_core::FKey,
}
#[repr(C, align(8))]
pub struct FUniqueNetIdReplNetSerializerConfig {}
#[repr(C, align(4))]
pub struct FNetLevelVisibilityTransactionId {
    pub data: u32,
}
#[repr(C, align(4))]
pub struct FUpdateLevelVisibilityLevelInfo {
    pub package_name: FName,
    pub file_name: FName,
    pub visibility_request_id: FNetLevelVisibilityTransactionId,
    pub flags_28: u8,
}
#[repr(C, align(4))]
pub struct FGeneratedBlueprintDelegateBinding {
    pub delegate_property_name: FName,
    pub function_name_to_bind: FName,
}
#[repr(C, align(8))]
pub struct FHLODISMComponentDesc {
    pub static_mesh: UPtr<UStaticMesh>,
    pub material: UPtr<UMaterialInterface>,
    pub instances: TArray<crate::bindings::core_u_object::FTransform>,
    pub instances_custom_primitive_data: TArray<FCustomPrimitiveData>,
}
#[repr(C, align(8))]
pub struct FHLODProxyMesh {
    pub lod_actor: TLazyObjectPtr<ALODActor>,
    pub static_mesh: UPtr<UStaticMesh>,
    pub key: FName,
}
#[repr(C, align(8))]
pub struct FHierarchicalSimplification {
    pub transition_screen_size: f32,
    pub override_draw_distance: f32,
    pub flags_8: u8,
    pub simplification_method: EHierarchicalSimplificationMethod,
    pub proxy_setting: FMeshProxySettings,
    pub merge_setting: FMeshMergingSettings,
    pub approximate_settings: FMeshApproximationSettings,
    pub desired_bound_radius: f32,
    pub desired_filling_percentage: f32,
    pub min_number_of_actors_to_build: i32,
    pub flags_980: u8,
}
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
    pub nanite_proxy_triangle_percent_deprecated: f32,
}
#[repr(C, align(8))]
pub struct FMaterialProxySettings {
    pub texture_sizing_type: ETextureSizingType,
    pub texture_size: crate::bindings::core_u_object::FIntPoint,
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
    pub material_merge_type: EMaterialMergeType,
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
}
#[repr(C, align(8))]
pub struct FMeshMergingSettings {
    pub target_light_map_resolution: i32,
    pub output_u_vs: EUVOutput,
    pub material_settings: FMaterialProxySettings,
    pub gutter_size: i32,
    pub lod_selection_type: EMeshLODSelectionType,
    pub specific_lod: i32,
    pub flags_188: u8,
    pub flags_189: u8,
    pub nanite_settings: FMeshNaniteSettings,
    pub flags_304: u8,
    pub merged_material_atlas_resolution_deprecated: i32,
    pub export_specific_lod_deprecated: i32,
    pub flags_316: u8,
    pub nanite_fallback_triangle_percent_deprecated: f32,
}
#[repr(C, align(8))]
pub struct FMeshProxySettings {
    pub screen_size: i32,
    pub voxel_size: f32,
    pub material_settings: FMaterialProxySettings,
    pub texture_width_deprecated: i32,
    pub texture_height_deprecated: i32,
    pub flags_176: u8,
    pub nanite_proxy_triangle_percent_deprecated: f32,
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
#[repr(C, align(8))]
pub struct FComponentKey {
    pub owner_class: TSubclassOf<crate::bindings::core_u_object::UObject>,
    pub scs_variable_name: FName,
    pub associated_guid: crate::bindings::core_u_object::FGuid,
}
#[repr(C, align(8))]
pub struct FComponentOverrideRecord {
    pub component_class: TSubclassOf<crate::bindings::core_u_object::UObject>,
    pub component_template: UPtr<UActorComponent>,
    pub component_key: FComponentKey,
    pub cooked_component_instancing_data: FBlueprintCookedComponentInstancingData,
}
#[repr(C, align(4))]
pub struct FBlueprintInputDelegateBinding {
    pub flags_0: u8,
}
#[repr(C, align(4))]
pub struct FBlueprintInputActionDelegateBinding {
    pub input_action_name: FName,
    pub input_key_event: EInputEvent,
    pub function_name_to_bind: FName,
}
#[repr(C, align(4))]
pub struct FBlueprintInputAxisDelegateBinding {
    pub input_axis_name: FName,
    pub function_name_to_bind: FName,
}
#[repr(C, align(8))]
pub struct FBlueprintInputAxisKeyDelegateBinding {
    pub axis_key: crate::bindings::input_core::FKey,
    pub function_name_to_bind: FName,
}
#[repr(C, align(8))]
pub struct FBlueprintInputKeyDelegateBinding {
    pub input_chord: crate::bindings::slate::FInputChord,
    pub input_key_event: EInputEvent,
    pub function_name_to_bind: FName,
}
#[repr(C, align(4))]
pub struct FBlueprintInputTouchDelegateBinding {
    pub input_key_event: EInputEvent,
    pub function_name_to_bind: FName,
}
#[repr(C, align(8))]
pub struct FCurveEdEntry {
    pub curve_object: UPtr<crate::bindings::core_u_object::UObject>,
    pub curve_color: crate::bindings::core_u_object::FColor,
    pub curve_name: FString,
    pub b_hide_curve: i32,
    pub b_color_curve: i32,
    pub b_floating_point_color_curve: i32,
    pub b_clamp: i32,
    pub clamp_low: f32,
    pub clamp_high: f32,
}
#[repr(C, align(8))]
pub struct FCurveEdTab {
    pub tab_name: FString,
    pub curves: TArray<FCurveEdEntry>,
    pub view_start_input: f32,
    pub view_end_input: f32,
    pub view_start_output: f32,
    pub view_end_output: f32,
}
#[repr(C, align(4))]
pub struct FISMComponentInstance {
    pub client_index: i32,
    pub instance_index: i32,
    pub instance_sub_index: i32,
}
#[repr(C, align(8))]
pub struct FISMClientInstance {
    pub component_indices: TArray<i32>,
}
#[repr(C, align(8))]
pub struct FISMClientData {
    pub instances: TArray<FISMClientInstance>,
}
#[repr(C, align(8))]
pub struct FISMComponentData {
    pub instances: TArray<FISMComponentInstance>,
    pub client_instances: TArray<FISMClientData>,
    pub component: UPtr<UInstancedStaticMeshComponent>,
}
#[repr(C, align(8))]
pub struct FISMComponentDescriptorBase {
    pub hash: u32,
    pub component_class: TSubclassOf<UInstancedStaticMeshComponent>,
    pub mobility: EComponentMobility,
    pub virtual_texture_render_pass_type: ERuntimeVirtualTextureMainPassType,
    pub lightmap_type: ELightmapType,
    pub lighting_channels: FLightingChannels,
    pub ray_tracing_group_id: i32,
    pub ray_tracing_group_culling_priority: ERayTracingGroupCullingPriority,
    pub b_has_custom_navigable_geometry: EHasCustomNavigableGeometry,
    pub custom_depth_stencil_write_mask: ERendererStencilMask,
    pub body_instance: FBodyInstance,
    pub instance_min_draw_distance: i32,
    pub instance_start_cull_distance: i32,
    pub instance_end_cull_distance: i32,
    pub instance_lod_distance_scale: f32,
    pub virtual_texture_cull_mips: i32,
    pub translucency_sort_priority: i32,
    pub overridden_light_map_res: i32,
    pub custom_depth_stencil_value: i32,
    pub hlod_batching_policy: EHLODBatchingPolicy,
    pub flags_505: u8,
    pub flags_506: u8,
    pub flags_507: u8,
    pub flags_508: u8,
    pub flags_509: u8,
    pub flags_510: u8,
    pub world_position_offset_disable_distance: i32,
    pub nanite_pixel_programmable_distance: i32,
    pub shadow_cache_invalidation_behavior: EShadowCacheInvalidationBehavior,
    pub detail_mode: EDetailMode,
}
#[repr(C, align(16))]
pub struct FISMComponentDescriptor {
    pub static_mesh: UPtr<UStaticMesh>,
    pub override_materials: TArray<UPtr<UMaterialInterface>>,
    pub overlay_material: UPtr<UMaterialInterface>,
    pub runtime_virtual_textures: TArray<UPtr<URuntimeVirtualTexture>>,
    pub local_transform: crate::bindings::core_u_object::FTransform,
    pub static_mesh_component: TWeakObjectPtr<UStaticMeshComponent>,
}
#[repr(C, align(8))]
pub struct FSoftISMComponentDescriptor {
    pub static_mesh: TSoftObjectPtr<UStaticMesh>,
    pub override_materials: TArray<TSoftObjectPtr<UMaterialInterface>>,
    pub overlay_material: TSoftObjectPtr<UMaterialInterface>,
    pub runtime_virtual_textures: TArray<TSoftObjectPtr<URuntimeVirtualTexture>>,
}
#[repr(C, align(8))]
pub struct FProceduralISMComponentDescriptor {
    pub hash: u32,
    pub static_mesh: TSoftObjectPtr<UStaticMesh>,
    pub override_materials: TArray<UPtr<UMaterialInterface>>,
    pub overlay_material: UPtr<UMaterialInterface>,
    pub runtime_virtual_textures: TArray<UPtr<URuntimeVirtualTexture>>,
    pub num_instances: i32,
    pub num_custom_floats: i32,
    pub world_bounds: crate::bindings::core_u_object::FBox,
    pub instance_min_draw_distance: i32,
    pub instance_start_cull_distance: i32,
    pub instance_end_cull_distance: i32,
    pub mobility: EComponentMobility,
    pub virtual_texture_render_pass_type: ERuntimeVirtualTextureMainPassType,
    pub lighting_channels: FLightingChannels,
    pub custom_depth_stencil_write_mask: ERendererStencilMask,
    pub virtual_texture_cull_mips: i32,
    pub translucency_sort_priority: i32,
    pub custom_depth_stencil_value: i32,
    pub ray_tracing_group_id: i32,
    pub ray_tracing_group_culling_priority: ERayTracingGroupCullingPriority,
    pub flags_201: u8,
    pub flags_202: u8,
    pub world_position_offset_disable_distance: i32,
    pub shadow_cache_invalidation_behavior: EShadowCacheInvalidationBehavior,
    pub detail_mode: EDetailMode,
}
#[repr(C, align(1))]
pub struct FSlateModifierKeysState {
    pub modifier_keys_state_mask: u8,
}
#[repr(C, align(4))]
pub struct FFloatSpringState {}
#[repr(C, align(8))]
pub struct FVectorSpringState {}
#[repr(C, align(16))]
pub struct FQuaternionSpringState {}
#[repr(C, align(8))]
pub struct FDrawToRenderTargetContext {
    pub render_target: UPtr<UTextureRenderTarget2D>,
}
#[repr(C, align(8))]
pub struct FImportanceTexture {
    pub size: crate::bindings::core_u_object::FIntPoint,
    pub num_mips: i32,
    pub marginal_cdf: TArray<f32>,
    pub conditional_cdf: TArray<f32>,
    pub texture_data_deprecated: TArray<crate::bindings::core_u_object::FColor>,
    pub linear_texture_data: TArray<crate::bindings::core_u_object::FLinearColor>,
    pub texture: TWeakObjectPtr<UTexture2D>,
    pub weighting: EImportanceWeight,
}
#[repr(C, align(8))]
pub struct FLatentActionManager {}
#[repr(C, align(8))]
pub struct FLayerActorStats {
    pub ty: TSubclassOf<crate::bindings::core_u_object::UObject>,
    pub total: i32,
}
#[repr(C, align(8))]
pub struct FActorFolderSet {
    pub actor_folders: TSet<UPtr<UActorFolder>>,
}
#[repr(C, align(8))]
pub struct FStreamableTextureInstance {}
#[repr(C, align(8))]
pub struct FDynamicTextureInstance {
    pub texture: UPtr<UTexture2D>,
    pub b_attached: bool,
    pub original_radius: f32,
}
#[repr(C, align(8))]
pub struct FLevelSimplificationDetails {
    pub b_create_package_per_asset: bool,
    pub details_percentage: f32,
    pub static_mesh_material_settings: FMaterialProxySettings,
    pub b_override_landscape_export_lod: bool,
    pub landscape_export_lod: i32,
    pub landscape_material_settings: FMaterialProxySettings,
    pub b_bake_foliage_to_landscape: bool,
    pub b_bake_grass_to_landscape: bool,
}
#[repr(C, align(8))]
pub struct FReplicatedStaticActorDestructionInfo {
    pub obj_class: TSubclassOf<crate::bindings::core_u_object::UObject>,
}
#[repr(C, align(8))]
pub struct FClientReceiveData {
    pub local_pc: UPtr<APlayerController>,
    pub message_type: FName,
    pub message_index: i32,
    pub message_string: FString,
    pub related_player_state_1: UPtr<APlayerState>,
    pub related_player_state_2: UPtr<APlayerState>,
    pub optional_object: UPtr<crate::bindings::core_u_object::UObject>,
}
#[repr(C, align(8))]
pub struct FHLODInstancingKey {
    pub static_mesh: UPtr<UStaticMesh>,
    pub material: UPtr<UMaterialInterface>,
}
#[repr(C, align(8))]
pub struct FParameterGroupData {
    pub group_name: FString,
    pub group_sort_priority: i32,
}
#[repr(C, align(16))]
pub struct FMaterialAggregateAttribute {
    pub name: FName,
    pub ty: EMaterialAggregateAttributeType,
    pub aggregate: UPtr<UMaterialAggregate>,
    pub default_value: crate::bindings::core_u_object::FVector4f,
}
#[repr(C, align(8))]
pub struct FMaterialFunctionInfo {
    pub state_id: crate::bindings::core_u_object::FGuid,
    pub function: UPtr<UMaterialFunctionInterface>,
}
#[repr(C, align(8))]
pub struct FMaterialParameterCollectionInfo {
    pub state_id: crate::bindings::core_u_object::FGuid,
    pub parameter_collection: UPtr<UMaterialParameterCollection>,
}
#[repr(C, align(8))]
pub struct FMaterialCachedParameterEditorInfo {
    pub description: FString,
    pub group: FName,
    pub sort_priority: i32,
    pub asset_index: i32,
    pub expression_guid: crate::bindings::core_u_object::FGuid,
}
#[repr(C, align(8))]
pub struct FMaterialCachedParameterEntry {
    pub parameter_info_set: TSet<FMaterialParameterInfo>,
}
#[repr(C, align(8))]
pub struct FMaterialCachedParameterEditorEntry {
    pub editor_info: TArray<FMaterialCachedParameterEditorInfo>,
}
#[repr(C, align(8))]
pub struct FMaterialCachedExpressionEditorOnlyData {
    pub editor_only_entries: FMaterialCachedParameterEntry,
    pub editor_entries: FMaterialCachedParameterEditorEntry,
    pub static_switch_values_deprecated: TArray<bool>,
    pub static_component_mask_values: TArray<FStaticComponentMaskValue>,
    pub scalar_min_max_values: TArray<crate::bindings::core_u_object::FVector2D>,
    pub scalar_enumeration_values: TArray<
        TSoftObjectPtr<crate::bindings::core_u_object::UObject>,
    >,
    pub scalar_enumeration_index_values: TArray<i32>,
    pub scalar_curve_values: TArray<TSoftObjectPtr<UCurveLinearColor>>,
    pub scalar_curve_atlas_values: TArray<TSoftObjectPtr<UCurveLinearColorAtlas>>,
    pub vector_channel_name_values: TArray<FParameterChannelNames>,
    pub vector_used_as_channel_mask_values: TArray<bool>,
    pub texture_channel_name_values: TArray<FParameterChannelNames>,
    pub material_layers: FMaterialLayersFunctionsEditorOnlyData,
    pub asset_paths: TArray<FString>,
    pub landscape_layer_names: TArray<FName>,
    pub expression_include_file_paths: TSet<FString>,
    pub user_scene_texture_inputs: TSet<FName>,
    pub shader_tags: TArray<FName>,
}
#[repr(C, align(8))]
pub struct FMaterialCachedExpressionData {
    pub runtime_entries: FMaterialCachedParameterEntry,
    pub scalar_primitive_data_index_values: TArray<i32>,
    pub vector_primitive_data_index_values: TArray<i32>,
    pub scalar_values: TArray<f32>,
    pub static_switch_values: TArray<bool>,
    pub dynamic_switch_values: TArray<bool>,
    pub vector_values: TArray<crate::bindings::core_u_object::FLinearColor>,
    pub double_vector_values: TArray<crate::bindings::core_u_object::FVector4d>,
    pub texture_values: TArray<TSoftObjectPtr<UTexture>>,
    pub texture_collection_values: TArray<TSoftObjectPtr<UTextureCollection>>,
    pub parameter_collection_values: TArray<
        TSoftObjectPtr<UMaterialParameterCollection>,
    >,
    pub font_values: TArray<TSoftObjectPtr<UFont>>,
    pub font_page_values: TArray<i32>,
    pub runtime_virtual_texture_values: TArray<TSoftObjectPtr<URuntimeVirtualTexture>>,
    pub sparse_volume_texture_values: TArray<TSoftObjectPtr<USparseVolumeTexture>>,
    pub referenced_textures: TArray<UPtr<crate::bindings::core_u_object::UObject>>,
    pub referenced_texture_collections: TArray<UPtr<UTextureCollection>>,
    pub referenced_external_code_expression_classes: TArray<
        TSubclassOf<crate::bindings::core_u_object::UObject>,
    >,
    pub function_infos: TArray<FMaterialFunctionInfo>,
    pub function_infos_state_crc: u32,
    pub parameter_collection_infos: TArray<FMaterialParameterCollectionInfo>,
    pub grass_types: TArray<UPtr<crate::bindings::landscape::ULandscapeGrassType>>,
    pub material_cache_tags: TArray<UPtr<UMaterialCacheVirtualTextureTag>>,
    pub material_layers: FMaterialLayersFunctionsRuntimeData,
    pub dynamic_parameter_names: TArray<FName>,
    pub quality_levels_used: TArray<bool>,
    pub material_cache_uv_coordinates_used_mask: u64,
    pub flags_1256: u8,
    pub flags_1257: u8,
    pub property_connected_bitmask_deprecated: u32,
    pub property_connected_mask: u64,
}
#[repr(C, align(8))]
pub struct FMaterialLayerInput {
    pub input_type: EFunctionInputType,
}
#[repr(C, align(4))]
pub struct FMaterialExternalCodeEnvironmentDefine {
    pub name: FName,
    pub shader_frequency: EMaterialShaderFrequency,
}
#[repr(C, align(8))]
pub struct FMaterialExternalCodeDeclaration {
    pub flags_0: u8,
    pub return_type: EMaterialValueTypeBridge,
    pub name: FName,
    pub definition: FString,
    pub definition_ddx: FString,
    pub definition_ddy: FString,
    pub derivative: EDerivativeStatus,
    pub shader_frequency: EMaterialShaderFrequency,
    pub domains: TArray<EMaterialDomain>,
    pub environment_defines: TArray<FMaterialExternalCodeEnvironmentDefine>,
    pub minimum_feature_level: EMaterialFeatureLevel,
}
#[repr(C, align(8))]
pub struct FScalarParameterAtlasInstanceData {
    pub b_is_used_as_atlas_position: bool,
    pub curve: TSoftObjectPtr<UCurveLinearColor>,
    pub atlas: TSoftObjectPtr<UCurveLinearColorAtlas>,
}
#[repr(C, align(8))]
pub struct FScalarParameterValue {
    pub parameter_name_deprecated: FName,
    pub atlas_data: FScalarParameterAtlasInstanceData,
    pub parameter_info: FMaterialParameterInfo,
    pub parameter_value: f32,
    pub expression_guid: crate::bindings::core_u_object::FGuid,
}
#[repr(C, align(4))]
pub struct FVectorParameterValue {
    pub parameter_name_deprecated: FName,
    pub parameter_info: FMaterialParameterInfo,
    pub parameter_value: crate::bindings::core_u_object::FLinearColor,
    pub expression_guid: crate::bindings::core_u_object::FGuid,
}
#[repr(C, align(16))]
pub struct FDoubleVectorParameterValue {
    pub parameter_info: FMaterialParameterInfo,
    pub parameter_value: crate::bindings::core_u_object::FVector4d,
    pub expression_guid: crate::bindings::core_u_object::FGuid,
}
#[repr(C, align(8))]
pub struct FTextureParameterValue {
    pub parameter_name_deprecated: FName,
    pub parameter_info: FMaterialParameterInfo,
    pub parameter_value: UPtr<UTexture>,
    pub expression_guid: crate::bindings::core_u_object::FGuid,
}
#[repr(C, align(8))]
pub struct FTextureCollectionParameterValue {
    pub parameter_info: FMaterialParameterInfo,
    pub parameter_value: UPtr<UTextureCollection>,
    pub expression_guid: crate::bindings::core_u_object::FGuid,
}
#[repr(C, align(8))]
pub struct FParameterCollectionParameterValue {
    pub parameter_info: FMaterialParameterInfo,
    pub parameter_value: UPtr<UMaterialParameterCollection>,
    pub expression_guid: crate::bindings::core_u_object::FGuid,
}
#[repr(C, align(8))]
pub struct FRuntimeVirtualTextureParameterValue {
    pub parameter_info: FMaterialParameterInfo,
    pub parameter_value: UPtr<URuntimeVirtualTexture>,
    pub expression_guid: crate::bindings::core_u_object::FGuid,
}
#[repr(C, align(8))]
pub struct FSparseVolumeTextureParameterValue {
    pub parameter_info: FMaterialParameterInfo,
    pub parameter_value: UPtr<USparseVolumeTexture>,
    pub expression_guid: crate::bindings::core_u_object::FGuid,
}
#[repr(C, align(8))]
pub struct FFontParameterValue {
    pub parameter_name_deprecated: FName,
    pub parameter_info: FMaterialParameterInfo,
    pub font_value: UPtr<UFont>,
    pub font_page: i32,
    pub expression_guid: crate::bindings::core_u_object::FGuid,
}
#[repr(C, align(4))]
pub struct FUserSceneTextureOverride {
    pub key: FName,
    pub value: FName,
}
#[repr(C, align(8))]
pub struct FMaterialInstanceCachedData {
    pub parent_layer_index_remap: TArray<i32>,
}
#[repr(C, align(4))]
pub struct FLightmassMaterialInterfaceSettings {
    pub emissive_boost: f32,
    pub diffuse_boost: f32,
    pub export_resolution_scale: f32,
    pub flags_12: u8,
}
#[repr(C, align(8))]
pub struct FMaterialTextureInfo {
    pub sampling_scale: f32,
    pub uv_channel_index: i32,
    pub texture_name: FName,
    pub texture_reference: crate::bindings::core_u_object::FSoftObjectPath,
    pub texture_index: i32,
}
#[repr(C, align(8))]
pub struct FTextureSamplingInfo {
    pub texture: UPtr<UTexture>,
}
#[repr(C, align(8))]
pub struct FMaterialCachedTexturesSamplingInfo {
    pub texture_sampling_infos: TArray<FTextureSamplingInfo>,
}
#[repr(C, align(8))]
pub struct FMaterialOverrideNanite {
    pub b_enable_override: bool,
    pub override_material_editor: UPtr<UMaterialInterface>,
    pub override_material: UPtr<UMaterialInterface>,
    pub override_material_ref: TSoftObjectPtr<UMaterialInterface>,
}
#[repr(C, align(4))]
pub struct FCollectionParameterBase {
    pub parameter_name: FName,
    pub id: crate::bindings::core_u_object::FGuid,
}
#[repr(C, align(4))]
pub struct FCollectionScalarParameter {
    pub default_value: f32,
}
#[repr(C, align(4))]
pub struct FCollectionVectorParameter {
    pub default_value: crate::bindings::core_u_object::FLinearColor,
}
#[repr(C, align(8))]
pub struct FMemberReference {
    pub member_parent: UPtr<crate::bindings::core_u_object::UObject>,
    pub member_scope: FString,
    pub member_name: FName,
    pub member_guid: crate::bindings::core_u_object::FGuid,
    pub b_self_context: bool,
    pub b_was_deprecated: bool,
}
#[repr(C, align(8))]
pub struct FStaticMeshBudgetInfo {
    pub lod_group_name: FName,
    pub minimum_extent: f64,
}
#[repr(C, align(8))]
pub struct FMeshInstancingSettings {
    pub actor_class_to_use: TSubclassOf<AActor>,
    pub instance_replacement_threshold: i32,
    pub b_skip_meshes_with_vertex_colors: bool,
    pub b_use_hlod_volumes: bool,
    pub ism_component_to_use: TSubclassOf<UInstancedStaticMeshComponent>,
}
#[repr(C, align(4))]
pub struct FMeshReductionSettings {
    pub percent_triangles: f32,
    pub max_num_of_triangles: u32,
    pub percent_vertices: f32,
    pub max_num_of_verts: u32,
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
}
#[repr(C, align(16))]
pub struct FBspSurf {
    pub material: UPtr<UMaterialInterface>,
    pub actor: UPtr<ABrush>,
}
#[repr(C, align(4))]
pub struct FPacketSimulationSettings {
    pub pkt_loss: i32,
    pub pkt_loss_max_size: i32,
    pub pkt_loss_min_size: i32,
    pub pkt_order: i32,
    pub pkt_dup: i32,
    pub pkt_lag: i32,
    pub pkt_lag_variance: i32,
    pub pkt_lag_min: i32,
    pub pkt_lag_max: i32,
    pub pkt_incoming_lag_min: i32,
    pub pkt_incoming_lag_max: i32,
    pub pkt_incoming_loss: i32,
    pub pkt_jitter: i32,
    pub pkt_frame_delay: i32,
    pub pkt_incoming_frame_delay: i32,
    pub pkt_buffer_bloat_in_ms: i32,
    pub pkt_incoming_buffer_bloat_in_ms: i32,
}
#[repr(C, align(4))]
pub struct FNetDriverReplicationSystemConfig {
    pub max_replicated_object_count: u32,
    pub initial_net_object_list_count: u32,
    pub net_object_list_grow_count: u32,
    pub pre_allocated_memory_buffers_object_count: u32,
    pub max_replication_writer_object_count: u32,
    pub max_delta_compressed_object_count: u32,
    pub max_net_object_group_count: u32,
    pub b_allow_parallel_tasks: bool,
}
#[repr(C, align(8))]
pub struct FChannelDefinition {
    pub channel_name: FName,
    pub class_name: FName,
    pub channel_class: TSubclassOf<crate::bindings::core_u_object::UObject>,
    pub static_channel_index: i32,
    pub flags_36: u8,
}
#[repr(C, align(8))]
pub struct FNetworkEmulationProfileDescription {
    pub profile_name: FString,
    pub tool_tip: FString,
}
#[repr(C, align(8))]
pub struct FBandwidthTestItem {
    pub kilobyte: TArray<u8>,
}
#[repr(C, align(8))]
pub struct FBandwidthTestGenerator {
    pub replicated_buffers: TArray<FBandwidthTestItem>,
}
#[repr(C, align(8))]
pub struct FNetworkMetricConfig {
    pub metric_name: FName,
    pub mutator: crate::bindings::core_u_object::FInstancedStruct,
    pub class: TSoftObjectPtr<crate::bindings::core_u_object::UClass>,
    pub enable_mode: ENetworkMetricEnableMode,
}
#[repr(C, align(8))]
pub struct FNetworkMetricsMutator {
    pub name_override: FName,
}
#[repr(C, align(8))]
pub struct FNetworkMetricsMutatorAvg {
    pub metric_name: FName,
}
#[repr(C, align(8))]
pub struct FNetworkMetricsMutatorMin {
    pub metric_name: FName,
}
#[repr(C, align(8))]
pub struct FNetworkMetricsMutatorMax {
    pub metric_name: FName,
}
#[repr(C, align(8))]
pub struct FNetworkMetricsMutatorPercent {
    pub numerator_name: FName,
    pub denominator_name: FName,
}
#[repr(C, align(8))]
pub struct FNetworkMetricsMutatorOutPacketLoss {}
#[repr(C, align(8))]
pub struct FNetworkMetricsMutatorInPacketLoss {}
#[repr(C, align(8))]
pub struct FRPCDoSState {
    pub b_log_escalate: bool,
    pub b_send_escalate_analytics: bool,
    pub b_kick_player: bool,
    pub b_track_recent_rp_cs: bool,
    pub escalate_quota_rp_cs_per_frame: i16,
    pub escalate_time_quota_ms_per_frame: i16,
    pub escalate_quota_rp_cs_per_period: i16,
    pub escalate_time_quota_ms_per_period: i16,
    pub escalate_quota_time_period: i8,
    pub escalation_count_tolerance: i8,
    pub escalation_time_tolerance_ms: i16,
    pub rpc_repeat_limit_per_period: i16,
    pub rpc_repeat_limit_ms_per_period: i16,
    pub rpc_repeat_limit_time_period: i8,
    pub cooloff_time: i16,
    pub auto_escalate_time: i16,
}
#[repr(C, align(8))]
pub struct FRPCDoSStateConfig {}
#[repr(C, align(8))]
pub struct FRPCAnalyticsThreshold {
    pub rpc: FName,
    pub count_per_sec: i32,
    pub time_per_sec: f64,
}
#[repr(C, align(8))]
pub struct FUniqueNetIdRepl {
    pub replication_bytes: TArray<u8>,
}
#[repr(C, align(4))]
pub struct FParticleBurst {
    pub count: i32,
    pub count_low: i32,
    pub time: f32,
}
#[repr(C, align(1))]
pub struct FParticleSystemLOD {}
#[repr(C, align(8))]
pub struct FLODSoloTrack {
    pub solo_enable_setting: TArray<u8>,
}
#[repr(C, align(8))]
pub struct FNamedEmitterMaterial {
    pub name: FName,
    pub material: UPtr<UMaterialInterface>,
}
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
}
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
}
#[repr(C, align(8))]
pub struct FParticleSystemWorldManagerTickFunction {}
#[repr(C, align(8))]
pub struct FPSCPoolElem {
    pub psc: UPtr<UParticleSystemComponent>,
}
#[repr(C, align(8))]
pub struct FPSCPool {
    pub free_elements: TArray<FPSCPoolElem>,
}
#[repr(C, align(8))]
pub struct FWorldPSCPool {
    pub world_particle_system_pools: TMap<UPtr<UParticleSystem>, FPSCPool>,
}
#[repr(C, align(4))]
pub struct FClusterUnionBoneData {}
#[repr(C, align(8))]
pub struct FClusteredComponentData {
    pub replicated_proxy_component: TWeakObjectPtr<
        UClusterUnionReplicatedProxyComponent,
    >,
    pub owner: TWeakObjectPtr<AActor>,
    pub b_was_replicating: bool,
    pub b_pending_deletion: bool,
}
#[repr(C, align(8))]
pub struct FClusteredActorData {
    pub b_was_replicating_movement: bool,
}
#[repr(C, align(1))]
pub struct FClusterUnionReplicatedData {
    pub object_state: u8,
    pub b_is_anchored: bool,
}
#[repr(C, align(8))]
pub struct FClusterUnionPendingAddData {
    pub acceleration_payloads: TArray<FExternalSpatialAccelerationPayload>,
    pub bones_data: TSet<FClusterUnionBoneData>,
}
#[repr(C, align(4))]
pub struct FExternalSpatialAccelerationPayload {}
#[repr(C, align(8))]
pub struct FClusterUnionInitializationData {
    pub cluster_union_component: UPtr<UClusterUnionComponent>,
    pub processed_components: TArray<UPtr<UPrimitiveComponent>>,
}
#[repr(C, align(4))]
pub struct FConstraintDrive {
    pub stiffness: f32,
    pub damping: f32,
    pub max_force: f32,
    pub flags_12: u8,
}
#[repr(C, align(8))]
pub struct FLinearDriveConstraint {
    pub position_target: crate::bindings::core_u_object::FVector,
    pub velocity_target: crate::bindings::core_u_object::FVector,
    pub x_drive: FConstraintDrive,
    pub y_drive: FConstraintDrive,
    pub z_drive: FConstraintDrive,
    pub b_acceleration_mode: bool,
    pub flags_97: u8,
}
#[repr(C, align(8))]
pub struct FAngularDriveConstraint {
    pub twist_drive: FConstraintDrive,
    pub swing_drive: FConstraintDrive,
    pub slerp_drive: FConstraintDrive,
    pub orientation_target: crate::bindings::core_u_object::FRotator,
    pub angular_velocity_target: crate::bindings::core_u_object::FVector,
    pub angular_drive_mode: EAngularDriveMode,
    pub b_acceleration_mode: bool,
}
#[repr(C, align(8))]
pub struct FConstraintProfileProperties {
    pub projection_linear_tolerance: f32,
    pub projection_angular_tolerance: f32,
    pub projection_linear_alpha: f32,
    pub projection_angular_alpha: f32,
    pub shock_propagation_alpha: f32,
    pub linear_break_threshold: f32,
    pub linear_plasticity_threshold: f32,
    pub angular_break_threshold: f32,
    pub angular_plasticity_threshold: f32,
    pub contact_transfer_scale: f32,
    pub linear_limit: FLinearConstraint,
    pub cone_limit: FConeConstraint,
    pub twist_limit: FTwistConstraint,
    pub flags_128: u8,
    pub flags_129: u8,
    pub linear_drive: FLinearDriveConstraint,
    pub angular_drive: FAngularDriveConstraint,
    pub linear_plasticity_type: crate::bindings::physics_core::EConstraintPlasticityType,
}
#[repr(C, align(4))]
pub struct FConstraintBaseParams {
    pub stiffness: f32,
    pub damping: f32,
    pub restitution: f32,
    pub contact_distance: f32,
    pub flags_16: u8,
}
#[repr(C, align(4))]
pub struct FTwistConstraint {
    pub twist_limit_degrees: f32,
    pub twist_motion: crate::bindings::physics_core::EAngularConstraintMotion,
}
#[repr(C, align(4))]
pub struct FConeConstraint {
    pub swing1_limit_degrees: f32,
    pub swing2_limit_degrees: f32,
    pub swing1_motion: crate::bindings::physics_core::EAngularConstraintMotion,
    pub swing2_motion: crate::bindings::physics_core::EAngularConstraintMotion,
}
#[repr(C, align(4))]
pub struct FLinearConstraint {
    pub limit: f32,
    pub x_motion: crate::bindings::physics_core::ELinearConstraintMotion,
    pub y_motion: crate::bindings::physics_core::ELinearConstraintMotion,
    pub z_motion: crate::bindings::physics_core::ELinearConstraintMotion,
}
#[repr(C, align(8))]
pub struct FConstraintInstanceBase {}
#[repr(C, align(16))]
pub struct FConstraintInstance {
    pub joint_name: FName,
    pub constraint_bone1: FName,
    pub constraint_bone2: FName,
    pub pos1: crate::bindings::core_u_object::FVector,
    pub pri_axis1: crate::bindings::core_u_object::FVector,
    pub sec_axis1: crate::bindings::core_u_object::FVector,
    pub pos2: crate::bindings::core_u_object::FVector,
    pub pri_axis2: crate::bindings::core_u_object::FVector,
    pub sec_axis2: crate::bindings::core_u_object::FVector,
    pub angular_rotation_offset: crate::bindings::core_u_object::FRotator,
    pub flags_304: u8,
    pub profile_instance: FConstraintProfileProperties,
    pub flags_680: u8,
    pub projection_linear_tolerance_deprecated: f32,
    pub projection_angular_tolerance_deprecated: f32,
    pub linear_x_motion_deprecated: crate::bindings::physics_core::ELinearConstraintMotion,
    pub linear_y_motion_deprecated: crate::bindings::physics_core::ELinearConstraintMotion,
    pub linear_z_motion_deprecated: crate::bindings::physics_core::ELinearConstraintMotion,
    pub linear_limit_size_deprecated: f32,
    pub flags_700: u8,
    pub linear_limit_stiffness_deprecated: f32,
    pub linear_limit_damping_deprecated: f32,
    pub flags_712: u8,
    pub linear_break_threshold_deprecated: f32,
    pub angular_swing1_motion_deprecated: crate::bindings::physics_core::EAngularConstraintMotion,
    pub angular_twist_motion_deprecated: crate::bindings::physics_core::EAngularConstraintMotion,
    pub angular_swing2_motion_deprecated: crate::bindings::physics_core::EAngularConstraintMotion,
    pub flags_724: u8,
    pub swing1_limit_angle_deprecated: f32,
    pub twist_limit_angle_deprecated: f32,
    pub swing2_limit_angle_deprecated: f32,
    pub swing_limit_stiffness_deprecated: f32,
    pub swing_limit_damping_deprecated: f32,
    pub twist_limit_stiffness_deprecated: f32,
    pub twist_limit_damping_deprecated: f32,
    pub flags_756: u8,
    pub angular_break_threshold_deprecated: f32,
    pub flags_764: u8,
    pub linear_position_target_deprecated: crate::bindings::core_u_object::FVector,
    pub linear_velocity_target_deprecated: crate::bindings::core_u_object::FVector,
    pub linear_drive_spring_deprecated: f32,
    pub linear_drive_damping_deprecated: f32,
    pub linear_drive_force_limit_deprecated: f32,
    pub flags_828: u8,
    pub flags_829: u8,
    pub angular_position_target_deprecated: crate::bindings::core_u_object::FQuat,
    pub angular_drive_mode_deprecated: EAngularDriveMode,
    pub angular_orientation_target_deprecated: crate::bindings::core_u_object::FRotator,
    pub angular_velocity_target_deprecated: crate::bindings::core_u_object::FVector,
    pub angular_drive_spring_deprecated: f32,
    pub angular_drive_damping_deprecated: f32,
    pub angular_drive_force_limit_deprecated: f32,
}
#[repr(C, align(16))]
pub struct FConstraintInstanceAccessor {
    pub owner: TWeakObjectPtr<crate::bindings::core_u_object::UObject>,
    pub index: u32,
}
#[repr(C, align(8))]
pub struct FNetworkPhysicsRewindDataProxy {
    pub owner: UPtr<UNetworkPhysicsComponent>,
}
#[repr(C, align(8))]
pub struct FNetworkPhysicsRewindDataInputProxy {}
#[repr(C, align(8))]
pub struct FNetworkPhysicsRewindDataRemoteInputProxy {}
#[repr(C, align(8))]
pub struct FNetworkPhysicsRewindDataStateProxy {}
#[repr(C, align(8))]
pub struct FNetworkPhysicsRewindDataImportantInputProxy {}
#[repr(C, align(8))]
pub struct FNetworkPhysicsRewindDataImportantStateProxy {}
#[repr(C, align(8))]
pub struct FNetworkPhysicsRewindDataDeltaSourceStateProxy {}
#[repr(C, align(8))]
pub struct FNetworkPhysicsRewindDataDeltaSourceInputProxy {}
#[repr(C, align(8))]
pub struct FNetworkPhysicsPayload {
    pub server_frame: i32,
}
#[repr(C, align(8))]
pub struct FNetworkPhysicsDataCollection {
    pub data_array: TArray<crate::bindings::core_u_object::FInstancedStruct>,
}
#[repr(C, align(8))]
pub struct FNetworkPhysicsData {
    pub input_frame_deprecated: i32,
}
#[repr(C, align(4))]
pub struct FNetworkPhysicsSettings {
    pub b_override_sim_proxy_rep_mode: bool,
    pub sim_proxy_rep_mode: EPhysicsReplicationMode,
    pub b_focal_particle_in_physics_replication_lod: bool,
    pub event_scheduling_min_delay_seconds: f32,
}
#[repr(C, align(4))]
pub struct FNetworkPhysicsSettingsDefaultReplication {
    pub flags_0: u8,
    pub max_linear_hard_snap_distance: f32,
    pub b_hardsnap_in_physics_thread: bool,
    pub b_correct_connected_bodies: bool,
    pub b_correct_connected_bodies_friction: bool,
}
#[repr(C, align(4))]
pub struct FNetworkPhysicsSettingsPredictiveInterpolation {
    pub flags_0: u8,
    pub flags_1: u8,
    pub pos_correction_time_base: f32,
    pub pos_correction_time_min: f32,
    pub pos_correction_time_multiplier: f32,
    pub rot_correction_time_base: f32,
    pub rot_correction_time_min: f32,
    pub rot_correction_time_multiplier: f32,
    pub pos_interpolation_time_multiplier: f32,
    pub rot_interpolation_time_multiplier: f32,
    pub soft_snap_pos_strength: f32,
    pub soft_snap_rot_strength: f32,
    pub b_soft_snap_to_source: bool,
    pub b_disable_soft_snap: bool,
    pub b_skip_velocity_rep_on_pos_early_out: bool,
    pub b_post_resim_wait_for_update: bool,
    pub b_correct_connected_bodies: bool,
    pub b_correct_connected_bodies_friction: bool,
}
#[repr(C, align(4))]
pub struct FNetworkPhysicsSettingsResimulationErrorCorrection {
    pub flags_0: u8,
    pub resim_error_correction_duration: f32,
    pub resim_error_maximum_distance_before_snapping: f32,
    pub resim_error_maximum_desync_time_before_snapping: f32,
    pub resim_error_directional_decay_multiplier: f32,
    pub b_render_interp_apply_exponential_decay: bool,
    pub render_interp_exponential_decay_linear_half_life: f32,
    pub render_interp_exponential_decay_angular_half_life: f32,
    pub render_interp_minimum_linear_threshold: f32,
    pub render_interp_minimum_angular_threshold: f32,
}
#[repr(C, align(4))]
pub struct FNetworkPhysicsSettingsResimulation {
    pub flags_0: u8,
    pub flags_1: u8,
    pub resimulation_error_threshold_deprecated: u32,
    pub resimulation_error_position_threshold: f32,
    pub resimulation_error_rotation_threshold: f32,
    pub resimulation_error_linear_velocity_threshold: f32,
    pub resimulation_error_angular_velocity_threshold: f32,
    pub b_runtime_correction_enabled: bool,
    pub b_runtime_velocity_correction: bool,
    pub b_runtime_correct_connected_bodies: bool,
    pub pos_stability_multiplier: f32,
    pub rot_stability_multiplier: f32,
    pub vel_stability_multiplier: f32,
    pub ang_vel_stability_multiplier: f32,
    pub resimulation_error_correction_settings: FNetworkPhysicsSettingsResimulationErrorCorrection,
}
#[repr(C, align(8))]
pub struct FNetworkPhysicsSettingsNetworkPhysicsComponent {
    pub flags_0: u8,
    pub flags_1: u8,
    pub redundant_inputs: u16,
    pub redundant_remote_inputs: u16,
    pub redundant_states: u16,
    pub b_compare_state_to_trigger_rewind: bool,
    pub b_compare_state_to_trigger_rewind_include_sim_proxies: bool,
    pub b_compare_input_to_trigger_rewind: bool,
    pub b_enable_unreliable_flow: bool,
    pub b_enable_reliable_flow: bool,
    pub b_apply_data_instead_of_merge_data: bool,
    pub b_allow_input_extrapolation: bool,
    pub b_validate_data_on_game_thread: bool,
    pub b_apply_sim_proxy_state_at_runtime: bool,
    pub b_apply_sim_proxy_input_at_runtime: bool,
    pub b_trigger_resim_on_input_receive: bool,
    pub b_apply_input_decay_over_set_time: bool,
    pub input_decay_set_time: f32,
    pub input_decay_curve: FRuntimeFloatCurve,
}
#[repr(C, align(8))]
pub struct FNetworkPhysicsSettingsData {
    pub general_settings: FNetworkPhysicsSettings,
    pub default_replication_settings: FNetworkPhysicsSettingsDefaultReplication,
    pub predictive_interpolation_settings: FNetworkPhysicsSettingsPredictiveInterpolation,
    pub resimulation_settings: FNetworkPhysicsSettingsResimulation,
    pub network_physics_component_settings: FNetworkPhysicsSettingsNetworkPhysicsComponent,
}
#[repr(C, align(4))]
pub struct FPhysicalAnimationData {
    pub body_name: FName,
    pub flags_12: u8,
    pub orientation_strength: f32,
    pub angular_velocity_strength: f32,
    pub position_strength: f32,
    pub velocity_strength: f32,
    pub max_linear_force: f32,
    pub max_angular_force: f32,
}
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
}
#[repr(C, align(4))]
pub struct FSolverIterations {
    pub solver_iterations: i32,
    pub joint_iterations: i32,
    pub collision_iterations: i32,
    pub solver_push_out_iterations: i32,
    pub joint_push_out_iterations: i32,
    pub collision_push_out_iterations: i32,
}
#[repr(C, align(8))]
pub struct FPhysicsConstraintProfileHandle {
    pub profile_properties: FConstraintProfileProperties,
    pub profile_name: FName,
}
#[repr(C, align(4))]
pub struct FPhysicalSurfaceName {
    pub ty: crate::bindings::physics_core::EPhysicalSurface,
    pub name: FName,
}
#[repr(C, align(1))]
pub struct FChaosPhysicsSettings {
    pub default_threading_model: crate::bindings::chaos::EChaosThreadingMode,
    pub dedicated_thread_tick_mode: crate::bindings::chaos::EChaosSolverTickMode,
    pub dedicated_thread_buffer_mode: crate::bindings::chaos::EChaosBufferMode,
}
#[repr(C, align(4))]
pub struct FPhysicsReplicationResimulationSettings {
    pub b_enable_resimulation_error_position_threshold: bool,
    pub resimulation_error_position_threshold: f32,
    pub b_enable_resimulation_error_rotation_threshold: bool,
    pub resimulation_error_rotation_threshold: f32,
    pub b_enable_resimulation_error_linear_velocity_threshold: bool,
    pub resimulation_error_linear_velocity_threshold: f32,
    pub b_enable_resimulation_error_angular_velocity_threshold: bool,
    pub resimulation_error_angular_velocity_threshold: f32,
}
#[repr(C, align(4))]
pub struct FPhysicsReplicationLODSettings {
    pub b_enable_physics_replication_lod: bool,
    pub minimum_base_distance: f32,
    pub base_distance_radius_multiplier: f32,
    pub base_distances_for_resimulation_mode: f32,
    pub base_distances_for_full_prediction: f32,
    pub time_over_distance: f32,
}
#[repr(C, align(4))]
pub struct FPhysicsPredictionSettings {
    pub b_enable_physics_resimulation_deprecated: bool,
    pub resimulation_error_threshold_deprecated: f32,
    pub b_enable_physics_prediction: bool,
    pub b_enable_physics_history_capture: bool,
    pub max_supported_latency_prediction: f32,
    pub resimulation_settings: FPhysicsReplicationResimulationSettings,
    pub physics_replication_lod_settings: FPhysicsReplicationLODSettings,
}
#[repr(C, align(4))]
pub struct FPhysicalAnimationProfile {
    pub profile_name: FName,
    pub physical_animation_data: FPhysicalAnimationData,
}
#[repr(C, align(16))]
pub struct FCameraCacheEntry {
    pub time_stamp: f32,
    pub pov: FMinimalViewInfo,
}
#[repr(C, align(16))]
pub struct FTViewTarget {
    pub target: UPtr<AActor>,
    pub pov: FMinimalViewInfo,
    pub player_state: UPtr<APlayerState>,
}
#[repr(C, align(4))]
pub struct FViewTargetTransitionParams {
    pub blend_time: f32,
    pub blend_function: EViewTargetBlendFunction,
    pub blend_exp: f32,
    pub flags_12: u8,
}
#[repr(C, align(4))]
pub struct FUpdateLevelStreamingLevelStatus {
    pub package_name: FName,
    pub lod_index: i32,
    pub b_new_should_be_loaded: bool,
    pub b_new_should_be_visible: bool,
    pub b_new_should_block_on_load: bool,
    pub b_new_should_block_on_unload: bool,
}
#[repr(C, align(4))]
pub struct FAsyncPhysicsTimestamp {
    pub server_frame: i32,
    pub local_frame: i32,
}
#[repr(C, align(8))]
pub struct FPlayerMuteList {
    pub b_has_voice_handshake_completed: bool,
    pub voice_channel_idx: i32,
}
#[repr(C, align(8))]
pub struct FPropertyAccessIndirection {
    pub property: TFieldPath<FProperty>,
    pub function: UPtr<crate::bindings::core_u_object::UFunction>,
    pub return_buffer_size: i32,
    pub return_buffer_alignment: i32,
    pub array_index: i32,
    pub offset: u32,
    pub object_type: EPropertyAccessObjectType,
    pub ty: EPropertyAccessIndirectionType,
}
#[repr(C, align(8))]
pub struct FPropertyAccessIndirectionChain {
    pub property: TFieldPath<FProperty>,
    pub indirection_start_index: i32,
    pub indirection_end_index: i32,
}
#[repr(C, align(8))]
pub struct FNeuralProfileStruct {
    pub input_format: ENeuralProfileFormat,
    pub output_format: ENeuralProfileFormat,
    pub runtime_type: ENeuralProfileRuntimeType,
    pub nne_model_data: UPtr<crate::bindings::core_u_object::UObject>,
    pub input_dimension: crate::bindings::core_u_object::FIntVector4,
    pub output_dimension: crate::bindings::core_u_object::FIntVector4,
    pub batch_size_override: i32,
    pub tile_size: ENeuralModelTileType,
    pub tile_overlap: crate::bindings::core_u_object::FIntPoint,
    pub tile_overlap_resolve_type: ETileOverlapResolveType,
}
#[repr(C, align(8))]
pub struct FSpecularProfileStruct {
    pub format: ESpecularProfileFormat,
    pub view_color: FRuntimeCurveLinearColor,
    pub light_color: FRuntimeCurveLinearColor,
    pub texture: UPtr<UTexture2D>,
}
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
#[repr(C, align(8))]
pub struct FLevelNameAndTime {
    pub level_name: FString,
    pub level_change_time_in_ms: u32,
}
#[repr(C, align(8))]
pub struct FRepMovementNetSerializerConfig {}
#[repr(C, align(8))]
pub struct FReverbSettings {
    pub b_apply_reverb: bool,
    pub reverb_type_deprecated: ReverbPreset,
    pub reverb_effect: UPtr<UReverbEffect>,
    pub reverb_plugin_effect: UPtr<USoundEffectSubmixPreset>,
    pub volume: f32,
    pub fade_time: f32,
}
#[repr(C, align(16))]
pub struct FColorGradePerRangeSettings {
    pub saturation: crate::bindings::core_u_object::FVector4,
    pub contrast: crate::bindings::core_u_object::FVector4,
    pub gamma: crate::bindings::core_u_object::FVector4,
    pub gain: crate::bindings::core_u_object::FVector4,
    pub offset: crate::bindings::core_u_object::FVector4,
}
#[repr(C, align(16))]
pub struct FColorGradingSettings {
    pub global: FColorGradePerRangeSettings,
    pub shadows: FColorGradePerRangeSettings,
    pub midtones: FColorGradePerRangeSettings,
    pub highlights: FColorGradePerRangeSettings,
    pub shadows_max: f32,
    pub highlights_min: f32,
    pub highlights_max: f32,
}
#[repr(C, align(4))]
pub struct FFilmStockSettings {
    pub slope: f32,
    pub toe: f32,
    pub shoulder: f32,
    pub black_clip: f32,
    pub white_clip: f32,
}
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
#[repr(C, align(8))]
pub struct FLensBloomSettings {
    pub intensity: f32,
    pub gaussian_sum: FGaussianSumBloomSettings,
    pub convolution: FConvolutionBloomSettings,
    pub method: EBloomMethod,
}
#[repr(C, align(8))]
pub struct FLensImperfectionSettings {
    pub dirt_mask: UPtr<UTexture>,
    pub dirt_mask_intensity: f32,
    pub dirt_mask_tint: crate::bindings::core_u_object::FLinearColor,
}
#[repr(C, align(8))]
pub struct FLensSettings {
    pub bloom: FLensBloomSettings,
    pub imperfections: FLensImperfectionSettings,
    pub chromatic_aberration: f32,
}
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
}
#[repr(C, align(8))]
pub struct FShaderCompilerCounters {
    pub accumulated_local_worker_idle_time: f64,
    pub times_local_workers_were_idle: f64,
    pub jobs_assigned: i64,
    pub jobs_completed: i64,
    pub accumulated_pending_time: f64,
    pub max_pending_time: f64,
    pub accumulated_job_execution_time: f64,
    pub max_job_execution_time: f64,
    pub accumulated_job_life_time: f64,
    pub max_job_life_time: f64,
    pub accumulated_task_submit_jobs: f64,
    pub accumulated_task_submit_jobs_stall: f64,
    pub local_job_batches_seen: i64,
    pub total_jobs_reported_in_local_job_batches: i64,
    pub distributed_job_batches_seen: i64,
    pub total_jobs_reported_in_distributed_job_batches: i64,
    pub min_shader_code_size: i32,
    pub max_shader_code_size: i32,
    pub accumulated_shader_code_size: u64,
    pub num_accumulated_shader_codes: u64,
    pub shader_map_ddc_misses: u32,
    pub shader_map_ddc_hits: u32,
    pub total_cache_search_attempts: u64,
    pub total_cache_hits: u64,
    pub total_cache_duplicates: u32,
    pub total_cache_ddc_queries: u32,
    pub total_cache_ddc_hits: u32,
    pub unique_cache_input_hashes: u64,
    pub unique_cache_outputs: u64,
    pub cache_mem_used: u64,
    pub cache_mem_budget: u64,
    pub max_remote_agents: u32,
    pub max_active_agent_cores: u32,
}
#[repr(C, align(8))]
pub struct FShaderCompilerMaterialCounters {
    pub material_translate_calls: i32,
    pub material_translate_total_time_sec: f64,
    pub material_translate_translation_only_time_sec: f64,
    pub material_translate_serialization_only_time_sec: f64,
    pub material_cache_hits: i32,
}
#[repr(C, align(4))]
pub struct FShaderTimings {
    pub min_compile_time: f32,
    pub max_compile_time: f32,
    pub total_compile_time: f32,
    pub total_preprocess_time: f32,
    pub num_compiled: i32,
    pub average_compile_time: f32,
}
#[repr(C, align(8))]
pub struct FShaderCompilerSinglePermutationStat {
    pub permutation_hash: u64,
    pub permutation_string: u32,
    pub compiled: u32,
    pub cooked: u32,
    pub compiled_double: u32,
    pub cooked_double: u32,
}
#[repr(C, align(8))]
pub struct FShaderStats {
    pub permutation_compilations: TArray<FShaderCompilerSinglePermutationStat>,
    pub compiled: u32,
    pub cooked: u32,
    pub compiled_double: u32,
    pub cooked_double: u32,
    pub compile_time: f32,
}
#[repr(C, align(8))]
pub struct FSingleAnimationPlayData {
    pub anim_to_play: UPtr<UAnimationAsset>,
    pub flags_8: u8,
    pub saved_position: f32,
    pub saved_play_rate: f32,
}
#[repr(C, align(4))]
pub struct FBoneMirrorInfo {
    pub source_index: i32,
    pub bone_flip_axis: crate::bindings::core_u_object::EAxis,
}
#[repr(C, align(4))]
pub struct FBoneMirrorExport {
    pub bone_name: FName,
    pub source_bone_name: FName,
    pub bone_flip_axis: crate::bindings::core_u_object::EAxis,
}
#[repr(C, align(8))]
pub struct FSkeletalMeshClothBuildParams {
    pub target_asset: TWeakObjectPtr<
        crate::bindings::clothing_system_runtime_interface::UClothingAssetBase,
    >,
    pub target_lod: i32,
    pub b_remap_parameters: bool,
    pub asset_name: FString,
    pub lod_index: i32,
    pub source_section: i32,
    pub b_remove_from_mesh: bool,
    pub physics_asset: TSoftObjectPtr<UPhysicsAsset>,
}
#[repr(C, align(4))]
pub struct FClothPhysicsProperties_Legacy {
    pub vertical_resistance: f32,
    pub horizontal_resistance: f32,
    pub bend_resistance: f32,
    pub shear_resistance: f32,
    pub friction: f32,
    pub damping: f32,
    pub tether_stiffness: f32,
    pub tether_limit: f32,
    pub drag: f32,
    pub stiffness_frequency: f32,
    pub gravity_scale: f32,
    pub mass_scale: f32,
    pub inertia_blend: f32,
    pub self_collision_thickness: f32,
    pub self_collision_squash_scale: f32,
    pub self_collision_stiffness: f32,
    pub solver_frequency: f32,
    pub fiber_compression: f32,
    pub fiber_expansion: f32,
    pub fiber_resistance: f32,
}
#[repr(C, align(8))]
pub struct FClothingAssetData_Legacy {
    pub asset_name: FName,
    pub apex_file_name: FString,
    pub b_cloth_properties_changed: bool,
    pub physics_properties: FClothPhysicsProperties_Legacy,
}
#[repr(C, align(4))]
pub struct FBoneFilter {
    pub b_exclude_self: bool,
    pub bone_name: FName,
}
#[repr(C, align(8))]
pub struct FSkeletalMeshLODGroupSettings {
    pub screen_size: crate::bindings::core_u_object::FPerPlatformFloat,
    pub lod_hysteresis: f32,
    pub bone_filter_action_option: EBoneFilterActionOption,
    pub bone_list: TArray<FBoneFilter>,
    pub bones_to_prioritize: TArray<FName>,
    pub sections_to_prioritize: TArray<i32>,
    pub weight_of_prioritization: f32,
    pub bake_pose: UPtr<UAnimSequence>,
    pub reduction_settings: FSkeletalMeshOptimizationSettings,
    pub b_allow_mesh_deformer: bool,
}
#[repr(C, align(8))]
pub struct FSkelMeshMergeSectionMapping {
    pub section_i_ds: TArray<i32>,
}
#[repr(C, align(8))]
pub struct FSkelMeshMergeMeshUVTransforms {
    pub uv_transforms: TArray<crate::bindings::core_u_object::FTransform>,
}
#[repr(C, align(8))]
pub struct FSkelMeshMergeUVTransformMapping {
    pub uv_transforms_per_mesh: TArray<FSkelMeshMergeMeshUVTransforms>,
}
#[repr(C, align(8))]
pub struct FSkeletalMeshSamplingRegionBuiltData {}
#[repr(C, align(8))]
pub struct FSkeletalMeshSamplingLODBuiltData {}
#[repr(C, align(8))]
pub struct FSkeletalMeshSamplingBuiltData {
    pub whole_mesh_built_data: TArray<FSkeletalMeshSamplingLODBuiltData>,
    pub region_built_data: TArray<FSkeletalMeshSamplingRegionBuiltData>,
}
#[repr(C, align(4))]
pub struct FSkeletalMeshSamplingRegionBoneFilter {
    pub bone_name: FName,
    pub flags_12: u8,
}
#[repr(C, align(4))]
pub struct FSkeletalMeshSamplingRegionMaterialFilter {
    pub material_name: FName,
}
#[repr(C, align(8))]
pub struct FSkeletalMeshSamplingRegion {
    pub name: FName,
    pub lod_index: i32,
    pub flags_16: u8,
    pub material_filters: TArray<FSkeletalMeshSamplingRegionMaterialFilter>,
    pub bone_filters: TArray<FSkeletalMeshSamplingRegionBoneFilter>,
}
#[repr(C, align(8))]
pub struct FSkeletalMeshSamplingInfo {
    pub regions: TArray<FSkeletalMeshSamplingRegion>,
    pub built_data: FSkeletalMeshSamplingBuiltData,
}
#[repr(C, align(8))]
pub struct FSkeletalMeshLODInfo {
    pub screen_size: crate::bindings::core_u_object::FPerPlatformFloat,
    pub lod_hysteresis: f32,
    pub lod_material_map: TArray<i32>,
    pub b_enable_shadow_casting_deprecated: TArray<bool>,
    pub removed_bones_deprecated: TArray<FName>,
    pub build_settings: FSkeletalMeshBuildSettings,
    pub reduction_settings: FSkeletalMeshOptimizationSettings,
    pub bones_to_remove: TArray<FBoneReference>,
    pub bones_to_prioritize: TArray<FBoneReference>,
    pub sections_to_prioritize: TArray<FSectionReference>,
    pub weight_of_prioritization: f32,
    pub bake_pose: UPtr<UAnimSequence>,
    pub bake_pose_override: UPtr<UAnimSequence>,
    pub source_import_filename: FString,
    pub skin_cache_usage: ESkinCacheUsage,
    pub morph_target_position_error_tolerance: f32,
    pub imported_morph_target_source_filename: TMap<
        FString,
        FMorphTargetImportedSourceFileInfo,
    >,
    pub vertex_attributes: TArray<FSkeletalMeshVertexAttributeInfo>,
    pub flags_480: u8,
}
#[repr(C, align(8))]
pub struct FMorphTargetImportedSourceFileInfo {
    pub source_filename: FString,
    pub derived_data_hash: crate::bindings::core_u_object::FGuid,
}
#[repr(C, align(4))]
pub struct FSectionReference {
    pub section_index: i32,
}
#[repr(C, align(8))]
pub struct FSkeletalMeshSourceModel {
    pub triangle_count: i32,
    pub vertex_count: i32,
    pub bounds: crate::bindings::core_u_object::FBoxSphereBounds,
    pub cached_skin_weight_profile_names: TArray<FName>,
    pub cached_morph_target_names: TArray<FName>,
    pub cached_has_vertex_colors: TOptional<bool>,
    pub cached_vertex_color_guid: crate::bindings::core_u_object::FGuid,
    pub mesh_description_bulk_data: UPtr<USkeletalMeshDescriptionBulkData>,
}
#[repr(C, align(8))]
pub struct FSkeletalMaterial {
    pub material_interface: UPtr<UMaterialInterface>,
    pub material_slot_name: FName,
    pub b_enable_shadow_casting_deprecated: bool,
    pub b_recompute_tangent_deprecated: bool,
    pub imported_material_slot_name: FName,
    pub uv_channel_data: FMeshUVChannelInfo,
    pub overlay_material_interface: UPtr<UMaterialInterface>,
}
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
#[repr(C, align(8))]
pub struct FAttenuationSubmixSendSettings {}
#[repr(C, align(8))]
pub struct FSoundAttenuationSettings {
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
    pub distance_type_deprecated: ESoundDistanceCalc,
    pub omni_radius_deprecated: f32,
    pub non_spatialized_radius_start: f32,
    pub non_spatialized_radius_end: f32,
    pub non_spatialized_radius_mode: ENonSpatializedRadiusSpeakerMapMode,
    pub stereo_spread: f32,
    pub spatialization_plugin_settings_deprecated: UPtr<
        crate::bindings::audio_extensions::USpatializationPluginSourceSettingsBase,
    >,
    pub radius_min_deprecated: f32,
    pub radius_max_deprecated: f32,
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
    pub occlusion_plugin_settings_deprecated: UPtr<
        crate::bindings::audio_extensions::UOcclusionPluginSourceSettingsBase,
    >,
    pub reverb_plugin_settings_deprecated: UPtr<
        crate::bindings::audio_extensions::UReverbPluginSourceSettingsBase,
    >,
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
#[repr(C, align(1))]
pub struct FSoundBaseEditorData {
    pub override_use_submixes_on_preview: EUseSubmixOnPreviewMode,
}
#[repr(C, align(4))]
pub struct FSoundClassEditorData {}
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
    pub size_of_first_audio_chunk_in_seconds: crate::bindings::core_u_object::FPerPlatformFloat,
    pub default_submix: UPtr<USoundSubmix>,
}
#[repr(C, align(8))]
pub struct FSoundModulationDefaultSettings {
    pub volume_modulation_destination: FSoundModulationDestinationSettings,
    pub pitch_modulation_destination: FSoundModulationDestinationSettings,
    pub highpass_modulation_destination: FSoundModulationDestinationSettings,
    pub lowpass_modulation_destination: FSoundModulationDestinationSettings,
}
#[repr(C, align(8))]
pub struct FSoundModulationDestinationSettings {
    pub value: f32,
    pub b_enable_modulation: bool,
    pub modulator: UPtr<crate::bindings::audio_extensions::USoundModulatorBase>,
    pub modulators: TSet<UPtr<crate::bindings::audio_extensions::USoundModulatorBase>>,
}
#[repr(C, align(8))]
pub struct FPassiveSoundMixModifier {
    pub sound_mix: UPtr<USoundMix>,
    pub min_volume_threshold: f32,
    pub max_volume_threshold: f32,
}
#[repr(C, align(8))]
pub struct FSoundConcurrencySettings {
    pub max_count: i32,
    pub flags_4: u8,
    pub resolution_rule: EMaxConcurrentResolutionRule,
    pub retrigger_time: f32,
    pub platform_max_count: crate::bindings::core_u_object::FPerPlatformInt,
    pub volume_scale: f32,
    pub volume_scale_mode: EConcurrencyVolumeScaleMode,
    pub volume_scale_attack_time: f32,
    pub volume_scale_release_time: f32,
    pub voice_steal_release_time: f32,
}
#[repr(C, align(4))]
pub struct FSoundNodeEditorData {}
#[repr(C, align(8))]
pub struct FSoundGroup {
    pub sound_group: ESoundGroup,
    pub display_name: FString,
    pub flags_24: u8,
    pub decompressed_duration: f32,
}
#[repr(C, align(8))]
pub struct FAudioEQEffect {
    pub frequency_center0: f32,
    pub gain0: f32,
    pub bandwidth0: f32,
    pub frequency_center1: f32,
    pub gain1: f32,
    pub bandwidth1: f32,
    pub frequency_center2: f32,
    pub gain2: f32,
    pub bandwidth2: f32,
    pub frequency_center3: f32,
    pub gain3: f32,
    pub bandwidth3: f32,
}
#[repr(C, align(8))]
pub struct FSoundClassAdjuster {
    pub sound_class_object: UPtr<USoundClass>,
    pub volume_adjuster: f32,
    pub pitch_adjuster: f32,
    pub low_pass_filter_frequency: f32,
    pub flags_20: u8,
    pub voice_center_channel_volume_adjuster: f32,
}
#[repr(C, align(8))]
pub struct FSoundModulationDefaultRoutingSettings {
    pub volume_routing: EModulationRouting,
    pub pitch_routing: EModulationRouting,
    pub highpass_routing: EModulationRouting,
    pub lowpass_routing: EModulationRouting,
}
#[repr(C, align(4))]
pub struct FDistanceDatum {
    pub fade_in_distance_start: f32,
    pub fade_in_distance_end: f32,
    pub fade_out_distance_start: f32,
    pub fade_out_distance_end: f32,
    pub volume: f32,
}
#[repr(C, align(4))]
pub struct FModulatorContinuousParams {
    pub parameter_name: FName,
    pub default: f32,
    pub min_input: f32,
    pub max_input: f32,
    pub min_output: f32,
    pub max_output: f32,
    pub param_mode: ModulationParamMode,
}
#[repr(C, align(8))]
pub struct FDynamicChildSubmix {
    pub child_submixes: TArray<UPtr<USoundSubmixBase>>,
}
#[repr(C, align(4))]
pub struct FSoundSubmixSpectralAnalysisBandSettings {
    pub band_frequency: f32,
    pub attack_time_msec: i32,
    pub release_time_msec: i32,
    pub q_factor: f32,
}
#[repr(C, align(8))]
pub struct FStreamedAudioPlatformData {}
#[repr(C, align(4))]
pub struct FSoundWaveSpectralData {
    pub frequency_hz: f32,
    pub magnitude: f32,
    pub normalized_magnitude: f32,
}
#[repr(C, align(8))]
pub struct FSoundWaveSpectralDataPerSound {
    pub spectral_data: TArray<FSoundWaveSpectralData>,
    pub playback_time: f32,
    pub sound_wave: UPtr<USoundWave>,
}
#[repr(C, align(8))]
pub struct FSoundWaveEnvelopeDataPerSound {
    pub envelope: f32,
    pub playback_time: f32,
    pub sound_wave: UPtr<USoundWave>,
}
#[repr(C, align(4))]
pub struct FSoundWaveSpectralDataEntry {
    pub magnitude: f32,
    pub normalized_magnitude: f32,
}
#[repr(C, align(8))]
pub struct FSoundWaveSpectralTimeData {
    pub data: TArray<FSoundWaveSpectralDataEntry>,
    pub time_sec: f32,
}
#[repr(C, align(4))]
pub struct FSoundWaveEnvelopeTimeData {
    pub amplitude: f32,
    pub time_sec: f32,
}
#[repr(C, align(8))]
pub struct FSplineMeshComponentDescriptorBase {
    pub hash: u32,
    pub component_class: TSubclassOf<USplineMeshComponent>,
    pub mobility: EComponentMobility,
    pub virtual_texture_render_pass_type: ERuntimeVirtualTextureMainPassType,
    pub lightmap_type: ELightmapType,
    pub lighting_channels: FLightingChannels,
    pub ray_tracing_group_id: i32,
    pub ray_tracing_group_culling_priority: ERayTracingGroupCullingPriority,
    pub b_has_custom_navigable_geometry: EHasCustomNavigableGeometry,
    pub custom_depth_stencil_write_mask: ERendererStencilMask,
    pub body_instance: FBodyInstance,
    pub virtual_texture_cull_mips: i32,
    pub translucency_sort_priority: i32,
    pub overridden_light_map_res: i32,
    pub custom_depth_stencil_value: i32,
    pub hlod_batching_policy: EHLODBatchingPolicy,
    pub flags_489: u8,
    pub flags_490: u8,
    pub flags_491: u8,
    pub flags_492: u8,
    pub world_position_offset_disable_distance: i32,
    pub shadow_cache_invalidation_behavior: EShadowCacheInvalidationBehavior,
    pub detail_mode: EDetailMode,
}
#[repr(C, align(8))]
pub struct FSplineMeshComponentDescriptor {
    pub static_mesh: UPtr<UStaticMesh>,
    pub override_materials: TArray<UPtr<UMaterialInterface>>,
    pub overlay_material: UPtr<UMaterialInterface>,
    pub runtime_virtual_textures: TArray<UPtr<URuntimeVirtualTexture>>,
}
#[repr(C, align(8))]
pub struct FSoftSplineMeshComponentDescriptor {
    pub static_mesh: TSoftObjectPtr<UStaticMesh>,
    pub override_materials: TArray<TSoftObjectPtr<UMaterialInterface>>,
    pub overlay_material: TSoftObjectPtr<UMaterialInterface>,
    pub runtime_virtual_textures: TArray<TSoftObjectPtr<URuntimeVirtualTexture>>,
}
#[repr(C, align(4))]
pub struct FStaticMeshOptimizationSettings {
    pub reduction_method: EOptimizationType,
    pub num_of_triangles_percentage: f32,
    pub max_deviation_percentage: f32,
    pub welding_threshold: f32,
    pub b_recalc_normals: bool,
    pub normals_threshold: f32,
    pub silhouette_importance: u8,
    pub texture_importance: u8,
    pub shading_importance: u8,
}
#[repr(C, align(4))]
pub struct FMeshSectionInfo {
    pub material_index: i32,
    pub b_enable_collision: bool,
    pub b_cast_shadow: bool,
    pub b_visible_in_ray_tracing: bool,
    pub b_affect_distance_field_lighting: bool,
    pub b_force_opaque: bool,
}
#[repr(C, align(8))]
pub struct FMeshSectionInfoMap {
    pub map: TMap<u32, FMeshSectionInfo>,
}
#[repr(C, align(8))]
pub struct FAssetEditorOrbitCameraPosition {
    pub b_is_set: bool,
    pub cam_orbit_point: crate::bindings::core_u_object::FVector,
    pub cam_orbit_zoom: crate::bindings::core_u_object::FVector,
    pub cam_orbit_rotation: crate::bindings::core_u_object::FRotator,
}
#[repr(C, align(8))]
pub struct FStaticMaterial {
    pub material_interface: UPtr<UMaterialInterface>,
    pub material_slot_name: FName,
    pub imported_material_slot_name: FName,
    pub uv_channel_data: FMeshUVChannelInfo,
    pub overlay_material_interface: UPtr<UMaterialInterface>,
}
#[repr(C, align(8))]
pub struct FStaticMaterialMinimalInfo {}
#[repr(C, align(8))]
pub struct FMaterialRemapIndex {
    pub import_version_key: u32,
    pub material_remap: TArray<i32>,
}
#[repr(C, align(8))]
pub struct FStaticMeshSourceModel {
    pub static_mesh_description_bulk_data: UPtr<UStaticMeshDescriptionBulkData>,
    pub build_settings: FMeshBuildSettings,
    pub reduction_settings: FMeshReductionSettings,
    pub cache_mesh_description_triangles_count: u32,
    pub cache_mesh_description_vertices_count: u32,
    pub lod_distance_deprecated: f32,
    pub screen_size: crate::bindings::core_u_object::FPerPlatformFloat,
    pub source_import_filename: FString,
    pub b_import_with_base_mesh: bool,
}
#[repr(C, align(8))]
pub struct FStreamableTexture {
    pub name: FString,
    pub guid: crate::bindings::core_u_object::FGuid,
}
#[repr(C, align(8))]
pub struct FSubtitleAssetData {
    pub text: FText,
    pub subtitle_duration_type: ESubtitleDurationType,
    pub b_can_edit_duration: bool,
    pub duration: f32,
    pub start_offset: f32,
    pub priority: f32,
    pub subtitle_type: ESubtitleType,
    pub comment: FString,
}
#[repr(C, align(8))]
pub struct FSyncPointDescription {
    pub registered_name: FName,
    pub event_type: ESyncPointEventType,
    pub activation_rules: ESyncPointActivationRules,
    pub first_possible_tick_group: ETickingGroup,
    pub last_possible_tick_group: ETickingGroup,
    pub prerequisite_sync_groups: TArray<FName>,
}
#[repr(C, align(8))]
pub struct FExternalToolDefinition {
    pub tool_name: FString,
    pub executable_path: crate::bindings::core_u_object::FFilePath,
    pub command_line_options: FString,
    pub working_directory: crate::bindings::core_u_object::FDirectoryPath,
    pub script_extension: FString,
    pub script_directory: crate::bindings::core_u_object::FDirectoryPath,
}
#[repr(C, align(8))]
pub struct FImportFactorySettingValues {
    pub setting_name: FString,
    pub value: FString,
}
#[repr(C, align(8))]
pub struct FEditorImportExportTestDefinition {
    pub import_file_path: crate::bindings::core_u_object::FFilePath,
    pub export_file_extension: FString,
    pub b_skip_export: bool,
    pub factory_settings: TArray<FImportFactorySettingValues>,
}
#[repr(C, align(8))]
pub struct FEditorImportWorkflowDefinition {
    pub import_file_path: crate::bindings::core_u_object::FFilePath,
    pub factory_settings: TArray<FImportFactorySettingValues>,
}
#[repr(C, align(8))]
pub struct FBuildPromotionImportWorkflowSettings {
    pub diffuse: FEditorImportWorkflowDefinition,
    pub normal: FEditorImportWorkflowDefinition,
    pub static_mesh: FEditorImportWorkflowDefinition,
    pub reimport_static_mesh: FEditorImportWorkflowDefinition,
    pub blend_shape_mesh: FEditorImportWorkflowDefinition,
    pub morph_mesh: FEditorImportWorkflowDefinition,
    pub skeletal_mesh: FEditorImportWorkflowDefinition,
    pub animation: FEditorImportWorkflowDefinition,
    pub sound: FEditorImportWorkflowDefinition,
    pub surround_sound: FEditorImportWorkflowDefinition,
    pub other_assets_to_import: TArray<FEditorImportWorkflowDefinition>,
}
#[repr(C, align(8))]
pub struct FBuildPromotionOpenAssetSettings {
    pub blueprint_asset: crate::bindings::core_u_object::FFilePath,
    pub material_asset: crate::bindings::core_u_object::FFilePath,
    pub particle_system_asset: crate::bindings::core_u_object::FFilePath,
    pub skeletal_mesh_asset: crate::bindings::core_u_object::FFilePath,
    pub static_mesh_asset: crate::bindings::core_u_object::FFilePath,
    pub texture_asset: crate::bindings::core_u_object::FFilePath,
}
#[repr(C, align(8))]
pub struct FBuildPromotionNewProjectSettings {
    pub new_project_folder_override: crate::bindings::core_u_object::FDirectoryPath,
    pub new_project_name_override: FString,
}
#[repr(C, align(8))]
pub struct FMaterialEditorPromotionSettings {
    pub default_material_asset: crate::bindings::core_u_object::FFilePath,
    pub default_diffuse_texture: crate::bindings::core_u_object::FFilePath,
    pub default_normal_texture: crate::bindings::core_u_object::FFilePath,
}
#[repr(C, align(8))]
pub struct FParticleEditorPromotionSettings {
    pub default_particle_asset: crate::bindings::core_u_object::FFilePath,
}
#[repr(C, align(8))]
pub struct FBlueprintEditorPromotionSettings {
    pub first_mesh_path: crate::bindings::core_u_object::FFilePath,
    pub second_mesh_path: crate::bindings::core_u_object::FFilePath,
    pub default_particle_asset: crate::bindings::core_u_object::FFilePath,
}
#[repr(C, align(8))]
pub struct FBuildPromotionTestSettings {
    pub default_static_mesh_asset: crate::bindings::core_u_object::FFilePath,
    pub import_workflow: FBuildPromotionImportWorkflowSettings,
    pub open_assets: FBuildPromotionOpenAssetSettings,
    pub new_project_settings: FBuildPromotionNewProjectSettings,
    pub source_control_material: crate::bindings::core_u_object::FFilePath,
}
#[repr(C, align(8))]
pub struct FEditorMapPerformanceTestDefinition {
    pub performance_testmap: crate::bindings::core_u_object::FSoftObjectPath,
    pub test_timer: i32,
}
#[repr(C, align(8))]
pub struct FLaunchOnTestSettings {
    pub launch_on_testmap: crate::bindings::core_u_object::FFilePath,
    pub device_id: FString,
}
#[repr(C, align(8))]
pub struct FTextImportTestStruct {
    pub embedded_flags: ETextImportTestFlags,
    pub test_int: i32,
    pub test_string: FString,
}
#[repr(C, align(4))]
pub struct FTextureSourceLayerColorInfo {
    pub color_min: crate::bindings::core_u_object::FLinearColor,
    pub color_max: crate::bindings::core_u_object::FLinearColor,
}
#[repr(C, align(4))]
pub struct FTextureSourceBlock {
    pub block_x: i32,
    pub block_y: i32,
    pub size_x: i32,
    pub size_y: i32,
    pub num_slices: i32,
    pub num_mips: i32,
}
#[repr(C, align(8))]
pub struct FTextureSource {
    pub id: crate::bindings::core_u_object::FGuid,
    pub base_block_x: i32,
    pub base_block_y: i32,
    pub size_x: i32,
    pub size_y: i32,
    pub num_slices: i32,
    pub num_mips: i32,
    pub num_layers: i32,
    pub b_png_compressed_deprecated: bool,
    pub b_long_lat_cubemap: bool,
    pub compression_format: ETextureSourceCompressionFormat,
    pub b_guid_is_hash: bool,
    pub layer_color_info_lock_protected: TArray<FTextureSourceLayerColorInfo>,
    pub format: ETextureSourceFormat,
    pub layer_format: TArray<ETextureSourceFormat>,
    pub blocks: TArray<FTextureSourceBlock>,
    pub block_data_offsets: TArray<i64>,
}
#[repr(C, align(8))]
pub struct FTexturePlatformData {}
#[repr(C, align(1))]
pub struct FTextureFormatSettings {
    pub compression_settings: TextureCompressionSettings,
    pub flags_1: u8,
}
#[repr(C, align(8))]
pub struct FTextureSourceColorSettings {
    pub encoding_override: ETextureSourceEncoding,
    pub color_space: ETextureColorSpace,
    pub red_chromaticity_coordinate: crate::bindings::core_u_object::FVector2D,
    pub green_chromaticity_coordinate: crate::bindings::core_u_object::FVector2D,
    pub blue_chromaticity_coordinate: crate::bindings::core_u_object::FVector2D,
    pub white_chromaticity_coordinate: crate::bindings::core_u_object::FVector2D,
    pub chromatic_adaptation_method: ETextureChromaticAdaptationMethod,
}
#[repr(C, align(4))]
pub struct FTextureLODGroup {
    pub group: TextureGroup,
    pub mip_gen_settings: TextureMipGenSettings,
    pub mip_load_options: ETextureMipLoadOptions,
    pub lod_bias: i32,
    pub lod_bias_smaller: i32,
    pub lod_bias_smallest: i32,
    pub lod_bias_vt: i32,
    pub num_streamed_mips: i32,
    pub min_lod_size: i32,
    pub max_lod_size: i32,
    pub max_lod_size_smaller: i32,
    pub max_lod_size_smallest: i32,
    pub max_lod_size_vt: i32,
    pub optional_lod_bias: i32,
    pub optional_max_lod_size: i32,
    pub min_mag_filter: FName,
    pub mip_filter: FName,
    pub flags_80: u8,
    pub downscale_options: ETextureDownscaleOptions,
    pub lossy_compression_amount: ETextureLossyCompressionAmount,
    pub downscale: f32,
    pub virtual_texture_tile_count_bias: i32,
    pub virtual_texture_tile_size_bias: i32,
    pub virtual_texture_streaming_priority: crate::bindings::render_core::EVTProducerPriority,
    pub max_aniso: i32,
}
#[repr(C, align(8))]
pub struct FTTTrackBase {
    pub track_name: FName,
    pub b_is_external_curve: bool,
    pub b_is_expanded: bool,
    pub b_is_curve_view_synchronized: bool,
}
#[repr(C, align(4))]
pub struct FTTTrackId {
    pub track_type: i32,
    pub track_index: i32,
}
#[repr(C, align(8))]
pub struct FTTEventTrack {
    pub function_name: FName,
    pub curve_keys: UPtr<UCurveFloat>,
}
#[repr(C, align(8))]
pub struct FTTPropertyTrack {
    pub property_name: FName,
}
#[repr(C, align(8))]
pub struct FTTFloatTrack {
    pub curve_float: UPtr<UCurveFloat>,
}
#[repr(C, align(8))]
pub struct FTTVectorTrack {
    pub curve_vector: UPtr<UCurveVector>,
}
#[repr(C, align(8))]
pub struct FTTLinearColorTrack {
    pub curve_linear_color: UPtr<UCurveLinearColor>,
}
#[repr(C, align(8))]
pub struct FActorLocatorFragmentResolveParameter {
    pub streaming_world: UPtr<UWorld>,
    pub container_id: FActorContainerID,
    pub source_asset_path: crate::bindings::core_u_object::FTopLevelAssetPath,
}
#[repr(C, align(8))]
pub struct FActorLocatorFragment {
    pub path: crate::bindings::core_u_object::FSoftObjectPath,
}
#[repr(C, align(4))]
pub struct FAnimInstanceLocatorFragment {
    pub ty: EAnimInstanceLocatorFragmentType,
}
#[repr(C, align(4))]
pub struct FAssetLocatorFragment {
    pub path: crate::bindings::core_u_object::FTopLevelAssetPath,
}
#[repr(C, align(8))]
pub struct FHardwareCursorReference {
    pub cursor_path: FName,
    pub hot_spot: crate::bindings::core_u_object::FVector2D,
}
#[repr(C, align(8))]
pub struct FCanvasIcon {
    pub texture: UPtr<UTexture>,
    pub u: f32,
    pub v: f32,
    pub ul: f32,
    pub vl: f32,
}
#[repr(C, align(8))]
pub struct FAutoCompleteNode {
    pub index_char: i32,
    pub auto_complete_list_indices: TArray<i32>,
}
#[repr(C, align(4))]
pub struct FHardwareDeviceIdentifier {
    pub input_class_name: FName,
    pub hardware_device_identifier: FName,
    pub primary_device_type: EHardwareDevicePrimaryType,
    pub supported_features_mask: i32,
}
#[repr(C, align(8))]
pub struct FKeyBind {
    pub key: crate::bindings::input_core::FKey,
    pub command: FString,
    pub flags_48: u8,
    pub flags_49: u8,
}
#[repr(C, align(4))]
pub struct FInputAxisProperties {
    pub dead_zone: f32,
    pub sensitivity: f32,
    pub exponent: f32,
    pub flags_12: u8,
}
#[repr(C, align(4))]
pub struct FInputAxisConfigEntry {
    pub axis_key_name: FName,
    pub axis_properties: FInputAxisProperties,
}
#[repr(C, align(8))]
pub struct FInputActionKeyMapping {
    pub action_name: FName,
    pub flags_12: u8,
    pub key: crate::bindings::input_core::FKey,
}
#[repr(C, align(8))]
pub struct FInputAxisKeyMapping {
    pub axis_name: FName,
    pub scale: f32,
    pub key: crate::bindings::input_core::FKey,
}
#[repr(C, align(4))]
pub struct FInputActionSpeechMapping {
    pub action_name: FName,
    pub speech_keyword: FName,
}
#[repr(C, align(8))]
pub struct FVoiceSettings {
    pub component_to_attach_to: UPtr<USceneComponent>,
    pub attenuation_settings: UPtr<USoundAttenuation>,
    pub source_effect_chain: UPtr<USoundEffectSourcePresetChain>,
}
#[repr(C, align(4))]
pub struct FVirtualTextureBuildSettings {
    pub tile_size: i32,
    pub tile_border_size: i32,
}
#[repr(C, align(8))]
pub struct FVirtualTextureSpacePoolConfig {
    pub formats: TArray<crate::bindings::core_u_object::EPixelFormat>,
    pub min_tile_size: i32,
    pub max_tile_size: i32,
    pub size_in_megabyte: i32,
    pub b_enable_residency_mip_map_bias: bool,
    pub residency_mip_map_bias_group: i32,
    pub b_allow_size_scale: bool,
    pub min_scaled_size_in_megabyte: i32,
    pub max_scaled_size_in_megabyte: i32,
}
#[repr(C, align(8))]
pub struct FStartPhysicsTickFunction {}
#[repr(C, align(8))]
pub struct FEndPhysicsTickFunction {}
#[repr(C, align(8))]
pub struct FLevelCollection {
    pub game_state: UPtr<AGameStateBase>,
    pub net_driver: UPtr<UNetDriver>,
    pub demo_net_driver: UPtr<UDemoNetDriver>,
    pub persistent_level: UPtr<ULevel>,
    pub levels: TSet<UPtr<ULevel>>,
}
#[repr(C, align(8))]
pub struct FStreamingLevelsToConsider {
    pub streaming_levels: TArray<UPtr<ULevelStreaming>>,
}
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
#[repr(C, align(4))]
pub struct FExternalDataLayerUID {
    pub value: u32,
}
#[repr(C, align(8))]
pub struct FActorPlacementDataLayers {
    pub data_layer_instance_names: TArray<FName>,
    pub external_data_layer_name: FName,
    pub current_colorized_data_layer_instance_name: FName,
    pub context_id: i32,
}
#[repr(C, align(8))]
pub struct FWorldPartitionActorFilter {}
#[repr(C, align(4))]
pub struct FWorldPartitionDestructibleHLODDamagedActorState {
    pub actor_index: i32,
    pub actor_health: u8,
}
#[repr(C, align(8))]
pub struct FWorldPartitionDestructibleHLODState {
    pub damaged_actors: TArray<FWorldPartitionDestructibleHLODDamagedActorState>,
    pub owner_component: UPtr<UWorldPartitionDestructibleHLODComponent>,
}
#[repr(C, align(4))]
pub struct FComponentInstanceMapping {
    pub a: u32,
    pub b: u32,
}
#[repr(C, align(4))]
pub struct FActorInstanceMappingsRef {
    pub a: u32,
    pub b: u32,
}
#[repr(C, align(8))]
pub struct FHLODInstancingPackedMappingData {
    pub ism_cs: TArray<UPtr<UHLODInstancedStaticMeshComponent>>,
    pub components_mapping: TArray<FComponentInstanceMapping>,
    pub per_actor_mapping_data: TMap<u32, FActorInstanceMappingsRef>,
}
#[repr(C, align(8))]
pub struct FExportHLODAssetsParams {
    pub mesh_origin: EExportHLODMeshOrigin,
    pub export_root_path: crate::bindings::core_u_object::FDirectoryPath,
}
#[repr(C, align(8))]
pub struct FHLODBuildInputReferencedAssets {
    pub static_meshes: TMap<crate::bindings::core_u_object::FTopLevelAssetPath, u32>,
}
#[repr(C, align(8))]
pub struct FHLODBuildInputStats {
    pub builders_referenced_assets: TMap<FName, FHLODBuildInputReferencedAssets>,
}
#[repr(C, align(8))]
pub struct FWorldPartitionHLODDestructionTag {
    pub hlod_destruction_component: UPtr<UWorldPartitionDestructibleHLODComponent>,
    pub actor_index: i32,
}
#[repr(C, align(8))]
pub struct FHLODSkinnedMeshComponentDescriptor {}
#[repr(C, align(16))]
pub struct FHLODISMComponentDescriptor {}
#[repr(C, align(4))]
pub struct FHLODModifierScalarParameter {
    pub parameter_name: FName,
    pub override_value: f32,
}
#[repr(C, align(8))]
pub struct FRuntimePartitionHLODSetup {
    pub name: FName,
    pub hlod_layers: TArray<UPtr<UHLODLayer>>,
    pub row_display_name: FName,
    pub partition_layer: UPtr<URuntimePartition>,
    pub b_is_spatially_loaded: bool,
}
#[repr(C, align(8))]
pub struct FRuntimePartitionDesc {
    pub name: FName,
    pub class: TSubclassOf<URuntimePartition>,
    pub main_layer: UPtr<URuntimePartition>,
    pub hlod_setups: TArray<FRuntimePartitionHLODSetup>,
}
#[repr(C, align(8))]
pub struct FRuntimePartitionStreamingData {
    pub name: FName,
    pub debug_name: FString,
    pub loading_range: i32,
    pub spatially_loaded_cells: TArray<UPtr<UWorldPartitionRuntimeCell>>,
    pub non_spatially_loaded_cells: TArray<UPtr<UWorldPartitionRuntimeCell>>,
}
#[repr(C, align(8))]
pub struct FVolumetricLightMapGridCell {
    pub bounds: crate::bindings::core_u_object::FBox,
    pub cell_id: u32,
}
#[repr(C, align(8))]
pub struct FVolumetricLightMapGridDesc {
    pub grid_bounds: crate::bindings::core_u_object::FBox,
    pub guid: crate::bindings::core_u_object::FGuid,
    pub cell_size: f32,
    pub detail_cell_size: f32,
    pub brick_size: f32,
    pub cells: TArray<FVolumetricLightMapGridCell>,
}
#[repr(C, align(8))]
pub struct FRuntimeCellTransformerInstance {
    pub class: TSubclassOf<UWorldPartitionRuntimeCellTransformer>,
    pub instance: UPtr<UWorldPartitionRuntimeCellTransformer>,
}
#[repr(C, align(8))]
pub struct FWorldPartitionPerWorldSettings {
    pub loaded_editor_regions: TArray<crate::bindings::core_u_object::FBox>,
    pub loaded_editor_location_volumes: TArray<FName>,
    pub not_loaded_data_layers: TArray<TSoftObjectPtr<UDataLayerAsset>>,
    pub loaded_data_layers: TArray<TSoftObjectPtr<UDataLayerAsset>>,
}
#[repr(C, align(8))]
pub struct FSubObjectPropertyOverride {
    pub serialized_tagged_properties: TArray<u8>,
}
#[repr(C, align(8))]
pub struct FPropertyOverrideReferenceTable {
    pub soft_object_path_table: TArray<crate::bindings::core_u_object::FSoftObjectPath>,
    pub object_references: TSet<UPtr<crate::bindings::core_u_object::UObject>>,
    pub b_is_valid: bool,
}
#[repr(C, align(8))]
pub struct FActorPropertyOverride {
    pub actor: TWeakObjectPtr<AActor>,
    pub sub_object_overrides: TMap<FString, FSubObjectPropertyOverride>,
    pub reference_table: FPropertyOverrideReferenceTable,
}
#[repr(C, align(8))]
pub struct FContainerPropertyOverride {
    pub actor_overrides: TMap<
        crate::bindings::core_u_object::FGuid,
        FActorPropertyOverride,
    >,
}
#[repr(C, align(8))]
pub struct FWorldPartitionRuntimeCellPropertyOverride {
    pub owner_container_id: FActorContainerID,
    pub asset_path: FString,
    pub package_name: FName,
    pub container_path: FActorContainerPath,
}
#[repr(C, align(16))]
pub struct FWorldPartitionRuntimeCellObjectMapping {
    pub package: FName,
    pub path: FName,
    pub base_class: crate::bindings::core_u_object::FTopLevelAssetPath,
    pub native_class: crate::bindings::core_u_object::FTopLevelAssetPath,
    pub container_id: FActorContainerID,
    pub container_transform: crate::bindings::core_u_object::FTransform,
    pub editor_only_parent_transform: crate::bindings::core_u_object::FTransform,
    pub container_package: FName,
    pub world_package: FName,
    pub actor_instance_guid: crate::bindings::core_u_object::FGuid,
    pub loaded_path: FName,
    pub b_is_editor_only: bool,
    pub property_overrides: TArray<FWorldPartitionRuntimeCellPropertyOverride>,
}
#[repr(C, align(8))]
pub struct FWorldPartitionRuntimeCellDebugInfo {
    pub name: FString,
    pub grid_name: FName,
    pub coord_x: i64,
    pub coord_y: i64,
    pub coord_z: i64,
}
#[repr(C, align(4))]
pub struct FWorldPartitionRuntimeContainerInstance {
    pub actor_guid: crate::bindings::core_u_object::FGuid,
    pub container_package: FName,
}
#[repr(C, align(8))]
pub struct FWorldPartitionRuntimeContainer {
    pub container_instances: TMap<FName, FWorldPartitionRuntimeContainerInstance>,
}
#[repr(C, align(8))]
pub struct FWorldPartitionRuntimeContainerResolver {
    pub containers: TMap<FName, FWorldPartitionRuntimeContainer>,
    pub main_container_package: FName,
    pub container_id_to_editor_path: TMap<FActorContainerID, FString>,
}
#[repr(C, align(8))]
pub struct FWorldPartitionRuntimeCellStreamingData {
    pub package_name: FString,
    pub world_asset: crate::bindings::core_u_object::FSoftObjectPath,
}
#[repr(C, align(1))]
pub struct FSpatialHashSettings {
    pub b_use_aligned_grid_levels: bool,
    pub b_snap_non_aligned_grid_levels_to_lower_levels: bool,
    pub b_place_small_actors_using_location: bool,
    pub b_place_partition_actors_using_location: bool,
}
#[repr(C, align(8))]
pub struct FSpatialHashStreamingGridLayerCell {
    pub grid_cells: TArray<UPtr<UWorldPartitionRuntimeCell>>,
}
#[repr(C, align(8))]
pub struct FSpatialHashStreamingGridLevel {
    pub layer_cells: TArray<FSpatialHashStreamingGridLayerCell>,
    pub layer_cells_mapping: TMap<i64, i32>,
}
#[repr(C, align(8))]
pub struct FSpatialHashStreamingGrid {
    pub grid_name: FName,
    pub origin: crate::bindings::core_u_object::FVector,
    pub cell_size: i32,
    pub loading_range: f32,
    pub b_block_on_slow_streaming: bool,
    pub debug_color: crate::bindings::core_u_object::FLinearColor,
    pub grid_levels: TArray<FSpatialHashStreamingGridLevel>,
    pub world_bounds: crate::bindings::core_u_object::FBox,
    pub b_client_only_visible: bool,
    pub grid_index: i32,
    pub settings: FSpatialHashSettings,
    pub injected_grid_levels: TArray<FSpatialHashStreamingGridLevel>,
}
#[repr(C, align(8))]
pub struct FSpatialHashRuntimeGrid {
    pub grid_name: FName,
    pub cell_size: i32,
    pub loading_range: f32,
    pub b_block_on_slow_streaming: bool,
    pub origin: crate::bindings::core_u_object::FVector2D,
    pub priority: i32,
    pub debug_color: crate::bindings::core_u_object::FLinearColor,
    pub b_client_only_visible: bool,
    pub hlod_layer: UPtr<UHLODLayer>,
}
#[repr(C, align(8))]
pub struct FWorldPartitionRuntimeSpatialHashGridPreviewer {
    pub material: UPtr<UMaterial>,
    pub mid: UPtr<UMaterialInstanceDynamic>,
    pub volume: UPtr<APostProcessVolume>,
}
#[repr(C, align(8))]
pub struct FHLODLayerTypeUnsupportedActorClasses {
    pub actor_classes: TArray<TSubclassOf<AActor>>,
}
#[repr(C, align(8))]
pub struct FActivatedCells {
    pub cells: TSet<UPtr<UWorldPartitionRuntimeCell>>,
}
#[repr(C, align(8))]
pub struct FWorldPartitionUpdateStreamingTargetState {
    pub to_load_cells: TArray<UPtr<UWorldPartitionRuntimeCell>>,
    pub to_activate_cells: TArray<UPtr<UWorldPartitionRuntimeCell>>,
    pub to_deactivate_cells: TArray<UPtr<UWorldPartitionRuntimeCell>>,
    pub to_unload_cells: TArray<UPtr<UWorldPartitionRuntimeCell>>,
}
#[repr(C, align(8))]
pub struct FWorldPartitionUpdateStreamingCurrentState {
    pub loaded_cells: TSet<UPtr<UWorldPartitionRuntimeCell>>,
    pub activated_cells: FActivatedCells,
}
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
#[repr(C, align(8))]
pub struct FWorldPartitionStreamingQuerySource {
    pub location: crate::bindings::core_u_object::FVector,
    pub radius: f32,
    pub b_use_grid_loading_range: bool,
    pub data_layers: TArray<FName>,
    pub b_data_layers_only: bool,
    pub b_spatial_query: bool,
}
#[repr(C, align(4))]
pub struct FLightmassWorldInfoSettings {
    pub static_lighting_level_scale: f32,
    pub num_indirect_lighting_bounces: i32,
    pub num_sky_lighting_bounces: i32,
    pub indirect_lighting_quality: f32,
    pub indirect_lighting_smoothness: f32,
    pub environment_color: crate::bindings::core_u_object::FColor,
    pub environment_intensity: f32,
    pub emissive_boost: f32,
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
    pub b_world_partition: bool,
}
#[repr(C, align(8))]
pub struct FNetViewer {
    pub connection: UPtr<UNetConnection>,
    pub in_viewer: UPtr<AActor>,
    pub view_target: UPtr<AActor>,
    pub view_location: crate::bindings::core_u_object::FVector,
    pub view_dir: crate::bindings::core_u_object::FVector,
}
#[repr(C, align(1))]
pub struct FNaniteSettings {
    pub b_allow_masked_materials: bool,
}
#[repr(C, align(8))]
pub struct FBroadphaseSettings {
    pub b_use_mbp_on_client: bool,
    pub b_use_mbp_on_server: bool,
    pub b_use_mbp_outer_bounds: bool,
    pub mbp_bounds: crate::bindings::core_u_object::FBox,
    pub mbp_outer_bounds: crate::bindings::core_u_object::FBox,
    pub mbp_num_subdivs: u32,
}
pub struct UMaterialExpression {
    pub material_expression_editor_x: i32,
    pub material_expression_editor_y: i32,
    pub graph_node: UPtr<UEdGraphNode>,
    pub subgraph_expression: UPtr<UMaterialExpression>,
    pub material_expression_guid: crate::bindings::core_u_object::FGuid,
    pub material: UPtr<UMaterial>,
    pub function: UPtr<UMaterialFunction>,
    pub desc: FString,
    pub flags_136: u8,
    pub flags_140: u8,
    pub flags_144: u8,
    pub menu_categories: TArray<FText>,
    pub outputs: TArray<FExpressionOutput>,
}
pub struct UMaterialExpressionCustomOutput {}
pub struct UMaterialExpressionBentNormalCustomOutput {
    pub input: FExpressionInput,
}
pub struct UMaterialExpressionClearCoatNormalCustomOutput {
    pub input: FExpressionInput,
}
pub struct UMaterialExpressionTangentOutput {
    pub input: FExpressionInput,
}
pub struct UMaterialExpressionThinTranslucentMaterialOutput {
    pub transmittance_color: FExpressionInput,
    pub surface_coverage: FExpressionInput,
}
pub struct UMaterialExpressionFirstPersonOutput {
    pub first_person_interpolation_alpha: FExpressionInput,
    pub const_first_person_interpolation_alpha: f32,
}
pub struct UMaterialExpressionMaterialCache {
    pub b_is_sample: bool,
    pub tag: UPtr<UMaterialCacheVirtualTextureTag>,
    pub attributes: TArray<FMaterialExpressionMaterialCacheAttribute>,
    pub primitive: FExpressionInput,
    pub uv: FExpressionInput,
}
pub struct UMaterialExpressionTemporalResponsivenessOutput {
    pub input: FExpressionInput,
}
pub struct UMaterialExpressionMotionVectorWorldOffsetOutput {
    pub input: FExpressionInput,
}
pub struct UEdGraph {
    pub schema: TSubclassOf<UEdGraphSchema>,
    pub nodes: TArray<UPtr<UEdGraphNode>>,
    pub flags_72: u8,
    pub sub_graphs: TArray<UPtr<UEdGraph>>,
    pub graph_guid: crate::bindings::core_u_object::FGuid,
    pub interface_guid: crate::bindings::core_u_object::FGuid,
}
pub struct UEdGraphNode {
    pub deprecated_pins: TArray<UPtr<UEdGraphPin_Deprecated>>,
    pub node_pos_x: i32,
    pub node_pos_y: i32,
    pub node_width: i32,
    pub node_height: i32,
    pub advanced_pin_display: ENodeAdvancedPins,
    pub enabled_state: ENodeEnabledState,
    pub flags_99: u8,
    pub flags_100: u8,
    pub node_upgrade_message: FText,
    pub node_comment: FString,
    pub error_type: i32,
    pub error_msg: FString,
    pub node_guid: crate::bindings::core_u_object::FGuid,
}
pub struct UEdGraphSchema {}
pub struct UDataAsset {
    pub native_class: TSubclassOf<UDataAsset>,
}
pub struct AActor {
    pub primary_actor_tick: FActorTickFunction,
    pub flags_104: u8,
    pub flags_105: u8,
    pub level_instance_type: ELevelInstanceType,
    pub level_instance_flags: ELevelInstanceFlags,
    pub flags_108: u8,
    pub flags_109: u8,
    pub flags_110: u8,
    pub flags_111: u8,
    pub flags_112: u8,
    pub flags_113: u8,
    pub update_overlaps_method_during_level_streaming: EActorUpdateOverlapsMethod,
    pub default_update_overlaps_method_during_level_streaming: EActorUpdateOverlapsMethod,
    pub remote_role: ENetRole,
    pub initial_life_span: f32,
    pub custom_time_dilation: f32,
    pub ray_tracing_group_id: i32,
    pub grid_placement_deprecated: EActorGridPlacement,
    pub runtime_grid: FName,
    pub attachment_replication: FRepAttachment,
    pub replicated_movement: FRepMovement,
    pub owner: UPtr<AActor>,
    pub net_driver_name: FName,
    pub role: ENetRole,
    pub net_dormancy: ENetDormancy,
    pub spawn_collision_handling_method: ESpawnActorCollisionHandlingMethod,
    pub auto_receive_input: EAutoReceiveInput,
    pub input_priority: i32,
    pub input_component: UPtr<UInputComponent>,
    pub net_tag: i32,
    pub net_cull_distance_squared: f32,
    pub net_update_frequency: f32,
    pub min_net_update_frequency: f32,
    pub net_priority: f32,
    pub physics_replication_mode: EPhysicsReplicationMode,
    pub instigator: UPtr<APawn>,
    pub children: TArray<UPtr<AActor>>,
    pub root_component: UPtr<USceneComponent>,
    pub pivot_offset: crate::bindings::core_u_object::FVector,
    pub hlod_layer: UPtr<UHLODLayer>,
    pub objects_detached_from_external_package: TArray<
        UPtr<crate::bindings::core_u_object::UObject>,
    >,
    pub layers: TArray<FName>,
    pub parent_component_actor_deprecated: TWeakObjectPtr<AActor>,
    pub parent_component: TWeakObjectPtr<UChildActorComponent>,
    pub actor_guid: crate::bindings::core_u_object::FGuid,
    pub actor_instance_guid: crate::bindings::core_u_object::FGuid,
    pub content_bundle_guid: crate::bindings::core_u_object::FGuid,
    pub data_layers: TArray<FActorDataLayer>,
    pub data_layer_assets: TArray<TSoftObjectPtr<UDataLayerAsset>>,
    pub external_data_layer_asset: UPtr<UExternalDataLayerAsset>,
    pub group_actor: UPtr<AActor>,
    pub sprite_scale: f32,
    pub hidden_editor_views: u64,
    pub actor_label: FString,
    pub folder_path: FName,
    pub folder_guid: crate::bindings::core_u_object::FGuid,
    pub flags_804: u8,
    pub flags_805: u8,
    pub tags: TArray<FName>,
    pub on_take_any_damage: FActor_OnTakeAnyDamage,
    pub on_take_point_damage: FActor_OnTakePointDamage,
    pub on_take_radial_damage: FActor_OnTakeRadialDamage,
    pub on_actor_begin_overlap: FActor_OnActorBeginOverlap,
    pub on_actor_end_overlap: FActor_OnActorEndOverlap,
    pub on_begin_cursor_over: FActor_OnBeginCursorOver,
    pub on_end_cursor_over: FActor_OnEndCursorOver,
    pub on_clicked: FActor_OnClicked,
    pub on_released: FActor_OnReleased,
    pub on_input_touch_begin: FActor_OnInputTouchBegin,
    pub on_input_touch_end: FActor_OnInputTouchEnd,
    pub on_input_touch_enter: FActor_OnInputTouchEnter,
    pub on_input_touch_leave: FActor_OnInputTouchLeave,
    pub on_actor_hit: FActor_OnActorHit,
    pub on_destroyed: FActor_OnDestroyed,
    pub on_end_play: FActor_OnEndPlay,
    pub instance_components: TArray<UPtr<UActorComponent>>,
    pub blueprint_created_components: TArray<UPtr<UActorComponent>>,
}
pub struct UActorComponent {
    pub primary_component_tick: FActorComponentTickFunction,
    pub component_tags: TArray<FName>,
    pub asset_user_data: TArray<UPtr<UAssetUserData>>,
    pub asset_user_data_editor_only: TArray<UPtr<UAssetUserData>>,
    pub ucs_serialization_index: i32,
    pub flags_184: u8,
    pub flags_186: u8,
    pub flags_187: u8,
    pub creation_method: EComponentCreationMethod,
    pub on_component_activated: FActorComponent_OnComponentActivated,
    pub on_component_deactivated: FActorComponent_OnComponentDeactivated,
    pub ucs_modified_properties_deprecated: TArray<FSimpleMemberReference>,
}
pub struct USceneComponent {
    pub physics_volume: TWeakObjectPtr<APhysicsVolume>,
    pub attach_parent: UPtr<USceneComponent>,
    pub attach_socket_name: FName,
    pub attach_children: TArray<UPtr<USceneComponent>>,
    pub client_attached_children: TArray<UPtr<USceneComponent>>,
    pub relative_location: crate::bindings::core_u_object::FVector,
    pub relative_rotation: crate::bindings::core_u_object::FRotator,
    pub relative_scale3_d: crate::bindings::core_u_object::FVector,
    pub component_velocity: crate::bindings::core_u_object::FVector,
    pub flags_480: u8,
    pub flags_481: u8,
    pub flags_482: u8,
    pub flags_483: u8,
    pub mobility: EComponentMobility,
    pub detail_mode: EDetailMode,
    pub physics_volume_changed_delegate: FSceneComponent_PhysicsVolumeChangedDelegate,
}
pub struct UPrimitiveComponent {
    pub min_draw_distance: f32,
    pub ld_max_draw_distance: f32,
    pub cached_max_draw_distance: f32,
    pub depth_priority_group: ESceneDepthPriorityGroup,
    pub view_owner_depth_priority_group: ESceneDepthPriorityGroup,
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
    pub b_has_custom_navigable_geometry: EHasCustomNavigableGeometry,
    pub hit_proxy_priority: EHitProxyPriority,
    pub exclude_for_specific_hlod_levels_deprecated: TArray<i32>,
    pub can_be_character_base_deprecated: ECanBeCharacterBase,
    pub flags_737: u8,
    pub can_character_step_up_on: ECanBeCharacterBase,
    pub lighting_channels: FLightingChannels,
    pub ray_tracing_group_culling_priority: ERayTracingGroupCullingPriority,
    pub custom_depth_stencil_write_mask: ERendererStencilMask,
    pub exclude_from_hlod_levels: u8,
    pub ray_tracing_group_id: i32,
    pub visibility_id: i32,
    pub custom_depth_stencil_value: i32,
    pub custom_primitive_data: FCustomPrimitiveData,
    pub custom_primitive_data_internal: FCustomPrimitiveData,
    pub translucency_sort_priority: i32,
    pub translucency_sort_distance_offset: f32,
    pub runtime_virtual_textures: TArray<UPtr<URuntimeVirtualTexture>>,
    pub virtual_texture_lod_bias: i8,
    pub virtual_texture_cull_mips: i8,
    pub virtual_texture_min_coverage: i8,
    pub virtual_texture_render_pass_type: ERuntimeVirtualTextureMainPassType,
    pub bounds_scale: f32,
    pub move_ignore_actors: TArray<UPtr<AActor>>,
    pub move_ignore_components: TArray<UPtr<UPrimitiveComponent>>,
    pub body_instance: FBodyInstance,
    pub on_component_hit: FPrimitiveComponent_OnComponentHit,
    pub on_component_begin_overlap: FPrimitiveComponent_OnComponentBeginOverlap,
    pub on_component_end_overlap: FPrimitiveComponent_OnComponentEndOverlap,
    pub on_component_wake: FPrimitiveComponent_OnComponentWake,
    pub on_component_sleep: FPrimitiveComponent_OnComponentSleep,
    pub on_component_physics_state_changed: FPrimitiveComponent_OnComponentPhysicsStateChanged,
    pub on_begin_cursor_over: FPrimitiveComponent_OnBeginCursorOver,
    pub on_end_cursor_over: FPrimitiveComponent_OnEndCursorOver,
    pub on_clicked: FPrimitiveComponent_OnClicked,
    pub on_released: FPrimitiveComponent_OnReleased,
    pub on_input_touch_begin: FPrimitiveComponent_OnInputTouchBegin,
    pub on_input_touch_end: FPrimitiveComponent_OnInputTouchEnd,
    pub on_input_touch_enter: FPrimitiveComponent_OnInputTouchEnter,
    pub on_input_touch_leave: FPrimitiveComponent_OnInputTouchLeave,
    pub lod_parent_primitive: UPtr<UPrimitiveComponent>,
}
pub struct UMeshComponent {
    pub override_materials: TArray<UPtr<UMaterialInterface>>,
    pub overlay_material: UPtr<UMaterialInterface>,
    pub overlay_material_max_draw_distance: f32,
    pub material_slots_overlay_material: TArray<UPtr<UMaterialInterface>>,
    pub flags_1552: u8,
}
pub struct UStaticMeshComponent {
    pub forced_lod_model: i32,
    pub min_lod: i32,
    pub sub_division_step_size: i32,
    pub wireframe_color_override: crate::bindings::core_u_object::FColor,
    pub static_mesh: UPtr<UStaticMesh>,
    pub world_position_offset_disable_distance: i32,
    pub flags_1612: u8,
    pub selected_editor_section: i32,
    pub selected_editor_material: i32,
    pub section_index_preview: i32,
    pub material_index_preview: i32,
    pub static_mesh_import_version: i32,
    pub flags_1636: u8,
    pub flags_1637: u8,
    pub flags_1638: u8,
    pub overridden_mesh_paint_texture_coordinate_index: i32,
    pub overridden_mesh_paint_texture_resolution: i32,
    pub overridden_light_map_res: i32,
    pub mesh_paint_texture: UPtr<UTexture>,
    pub mesh_paint_texture_cooked: UPtr<UTexture>,
    pub mesh_paint_texture_override: UPtr<UTexture>,
    pub material_cache_tile_count: crate::bindings::core_u_object::FIntPoint,
    pub material_cache_textures: TArray<UPtr<UMaterialCacheVirtualTexture>>,
    pub distance_field_indirect_shadow_min_visibility: f32,
    pub distance_field_self_shadow_bias: f32,
    pub streaming_distance_multiplier: f32,
    pub nanite_pixel_programmable_distance: f32,
    pub irrelevant_lights_deprecated: TArray<crate::bindings::core_u_object::FGuid>,
    pub lod_data: TArray<FStaticMeshComponentLODInfo>,
    pub streaming_texture_data: TArray<FStreamingTextureBuildInfo>,
    pub static_mesh_derived_data_key: FString,
    pub material_streaming_relative_boxes: TArray<u32>,
    pub lightmass_settings: FLightmassPrimitiveSettings,
}
pub struct USubsystem {}
pub struct UDynamicSubsystem {}
pub struct UBlueprintFunctionLibrary {}
pub struct UDebugDrawComponent {}
pub struct AController {
    pub player_state: UPtr<APlayerState>,
    pub on_instigated_any_damage: FController_OnInstigatedAnyDamage,
    pub on_possessed_pawn_changed: FController_OnPossessedPawnChanged,
    pub state_name: FName,
    pub pawn: UPtr<APawn>,
    pub character: UPtr<ACharacter>,
    pub transform_component: UPtr<USceneComponent>,
    pub control_rotation: crate::bindings::core_u_object::FRotator,
    pub flags_1312: u8,
}
pub struct UAISystemBase {
    pub ai_system_class_name: crate::bindings::core_u_object::FSoftClassPath,
    pub ai_system_module_name: FName,
    pub b_instantiate_ai_system_on_client: bool,
}
pub struct APawn {
    pub flags_1144: u8,
    pub base_eye_height: f32,
    pub auto_possess_player: EAutoReceiveInput,
    pub auto_possess_ai: EAutoPossessAI,
    pub remote_view_pitch16: u16,
    pub remote_view_pitch: u8,
    pub ai_controller_class: TSubclassOf<AController>,
    pub player_state: UPtr<APlayerState>,
    pub last_hit_by: UPtr<AController>,
    pub controller: UPtr<AController>,
    pub previous_controller: UPtr<AController>,
    pub receive_controller_changed_delegate: FPawn_ReceiveControllerChangedDelegate,
    pub receive_restarted_delegate: FPawn_ReceiveRestartedDelegate,
    pub control_input_vector: crate::bindings::core_u_object::FVector,
    pub last_control_input_vector: crate::bindings::core_u_object::FVector,
    pub override_input_component_class: TSubclassOf<UInputComponent>,
}
pub struct ACharacter {
    pub mesh: UPtr<USkeletalMeshComponent>,
    pub character_movement: UPtr<UCharacterMovementComponent>,
    pub capsule_component: UPtr<UCapsuleComponent>,
    pub arrow_component: UPtr<UArrowComponent>,
    pub based_movement: FBasedMovementInfo,
    pub replicated_based_movement: FBasedMovementInfo,
    pub replicated_server_last_transform_update_time_stamp: f32,
    pub replay_last_transform_update_time_stamp: f32,
    pub base_rotation_offset: crate::bindings::core_u_object::FQuat,
    pub base_translation_offset: crate::bindings::core_u_object::FVector,
    pub replicated_gravity_direction: FVector_NetQuantizeNormal,
    pub anim_root_motion_translation_scale: f32,
    pub crouched_eye_height: f32,
    pub flags_1568: u8,
    pub flags_1569: u8,
    pub replicated_movement_mode: u8,
    pub jump_key_hold_time: f32,
    pub jump_force_time_remaining: f32,
    pub proxy_jump_force_started_time: f32,
    pub jump_max_hold_time: f32,
    pub jump_max_count: i32,
    pub jump_current_count: i32,
    pub jump_current_count_pre_jump: i32,
    pub on_reached_jump_apex: FCharacter_OnReachedJumpApex,
    pub landed_delegate: FCharacter_LandedDelegate,
    pub movement_mode_changed_delegate: FCharacter_MovementModeChangedDelegate,
    pub on_character_movement_updated: FCharacter_OnCharacterMovementUpdated,
    pub saved_root_motion: FRootMotionSourceGroup,
    pub client_root_motion_params: FRootMotionMovementParams,
    pub root_motion_rep_moves: TArray<FSimulatedRootMotionReplicatedMove>,
    pub rep_root_motion: FRepRootMotionMontage,
}
pub struct UWorldSubsystem {}
pub struct UTickableWorldSubsystem {}
pub struct UNavLinkDefinition {
    pub links: TArray<FNavigationLink>,
    pub segment_links: TArray<FNavigationSegmentLink>,
}
pub struct UNavAreaBase {}
pub struct UNavCollisionBase {
    pub flags_48: u8,
}
pub struct UWorld {
    pub layers: TArray<UPtr<ULayer>>,
    pub active_group_actors: TArray<UPtr<AActor>>,
    pub thumbnail_info: UPtr<UThumbnailInfo>,
    pub persistent_level: UPtr<ULevel>,
    pub net_driver: UPtr<UNetDriver>,
    pub line_batcher_deprecated: UPtr<ULineBatchComponent>,
    pub persistent_line_batcher_deprecated: UPtr<ULineBatchComponent>,
    pub foreground_line_batcher_deprecated: UPtr<ULineBatchComponent>,
    pub network_manager: UPtr<AGameNetworkManager>,
    pub physics_collision_handler: UPtr<UPhysicsCollisionHandler>,
    pub physics_query_handler: UPtr<UPhysicsQueryHandler>,
    pub extra_referenced_objects: TArray<UPtr<crate::bindings::core_u_object::UObject>>,
    pub per_module_data_objects: TArray<UPtr<crate::bindings::core_u_object::UObject>>,
    pub streaming_levels: TArray<UPtr<ULevelStreaming>>,
    pub streaming_levels_to_consider: FStreamingLevelsToConsider,
    pub server_streaming_levels_visibility: UPtr<AServerStreamingLevelsVisibility>,
    pub streaming_levels_prefix: FString,
    pub line_batchers: UPtr<ULineBatchComponent>,
    pub making_visible_levels: TArray<UPtr<ULevel>>,
    pub making_invisible_levels: TArray<UPtr<ULevel>>,
    pub demo_net_driver: UPtr<UDemoNetDriver>,
    pub my_particle_event_manager: UPtr<AParticleEventManager>,
    pub default_physics_volume: UPtr<APhysicsVolume>,
    pub flags_447: u8,
    pub navigation_system: UPtr<UNavigationSystemBase>,
    pub authority_game_mode: UPtr<AGameModeBase>,
    pub game_state: UPtr<AGameStateBase>,
    pub ai_system: UPtr<UAISystemBase>,
    pub avoidance_manager: UPtr<UAvoidanceManager>,
    pub levels: TArray<UPtr<ULevel>>,
    pub level_collections: TArray<FLevelCollection>,
    pub current_level: UPtr<ULevel>,
    pub owning_game_instance: UPtr<UGameInstance>,
    pub parameter_collection_instances: TArray<
        UPtr<UMaterialParameterCollectionInstance>,
    >,
    pub canvas_for_rendering_to_target: UPtr<UCanvas>,
    pub canvas_for_draw_material_to_render_target: UPtr<UCanvas>,
    pub editor_views: TArray<FLevelViewportInfo>,
    pub physics_field: UPtr<UPhysicsFieldComponent>,
    pub components_that_need_pre_end_of_frame_sync: TSet<UPtr<UActorComponent>>,
    pub components_that_need_pre_end_of_frame_sync_async_tick: TSet<
        UPtr<UActorComponent>,
    >,
    pub components_that_need_end_of_frame_update: TArray<UPtr<UActorComponent>>,
    pub components_that_need_end_of_frame_update_async_tick: TArray<
        UPtr<UActorComponent>,
    >,
    pub components_that_need_end_of_frame_update_on_game_thread: TArray<
        UPtr<UActorComponent>,
    >,
    pub components_that_need_end_of_frame_mark_render_state_dirty: TArray<
        UPtr<UActorComponent>,
    >,
    pub selected_levels: TArray<UPtr<ULevel>>,
    pub world_composition: UPtr<UWorldComposition>,
    pub content_bundle_manager: UPtr<UContentBundleManager>,
    pub psc_pool: FWorldPSCPool,
}
pub struct UNavigationSystemBase {}
pub struct UNavigationSystemConfig {
    pub navigation_system_class: crate::bindings::core_u_object::FSoftClassPath,
    pub supported_agents_mask: FNavAgentSelector,
    pub default_agent_name: FName,
    pub flags_104: u8,
}
pub struct ABrush {
    pub brush_type: EBrushType,
    pub brush_color: crate::bindings::core_u_object::FColor,
    pub poly_flags: i32,
    pub flags_1148: u8,
    pub brush: UPtr<UModel>,
    pub brush_component: UPtr<UBrushComponent>,
    pub brush_builder: UPtr<UBrushBuilder>,
    pub flags_1176: u8,
    pub shaded_volume_opacity_value: f32,
    pub flags_1184: u8,
    pub saved_selections: TArray<FGeomSelection>,
}
pub struct AVolume {}
pub struct UNavigationDataChunk {
    pub navigation_data_name: FName,
}
pub struct UEngineSubsystem {}
pub struct ACameraActor {
    pub auto_activate_for_player: EAutoReceiveInput,
    pub camera_component: UPtr<UCameraComponent>,
    pub scene_component: UPtr<USceneComponent>,
    pub flags_1160: u8,
    pub aspect_ratio_deprecated: f32,
    pub fov_angle_deprecated: f32,
    pub post_process_blend_weight_deprecated: f32,
    pub post_process_settings_deprecated: FPostProcessSettings,
}
pub struct UCameraComponent {
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
    pub asymmetric_overscan: crate::bindings::core_u_object::FVector4f,
    pub b_scale_resolution_with_overscan: bool,
    pub b_crop_overscan: bool,
    pub b_draw_frustum_allowed: bool,
    pub flags_723: u8,
    pub projection_mode: ECameraProjectionMode,
    pub camera_mesh: UPtr<UStaticMesh>,
    pub post_process_blend_weight: f32,
    pub post_process_settings: FPostProcessSettings,
    pub flags_2848: u8,
}
pub struct UAssetExportTask {
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
pub struct UBlueprintExtension {}
pub struct UBlueprintAsyncActionBase {}
pub struct UBlueprintCore {
    pub skeleton_generated_class: TSubclassOf<crate::bindings::core_u_object::UObject>,
    pub generated_class: TSubclassOf<crate::bindings::core_u_object::UObject>,
    pub b_legacy_need_to_purge_skel_refs: bool,
    pub blueprint_guid: crate::bindings::core_u_object::FGuid,
}
pub struct UBlueprint {
    pub parent_class: TSubclassOf<crate::bindings::core_u_object::UObject>,
    pub blueprint_type: EBlueprintType,
    pub flags_105: u8,
    pub flags_106: u8,
    pub should_cook_property_guids_value: EShouldCookBlueprintPropertyGuids,
    pub compile_mode: EBlueprintCompileMode,
    pub status: EBlueprintStatus,
    pub blueprint_display_name: FString,
    pub blueprint_description: FString,
    pub blueprint_namespace: FString,
    pub blueprint_category: FString,
    pub hide_categories: TArray<FString>,
    pub blueprint_system_version: i32,
    pub simple_construction_script: UPtr<USimpleConstructionScript>,
    pub ubergraph_pages: TArray<UPtr<UEdGraph>>,
    pub function_graphs: TArray<UPtr<UEdGraph>>,
    pub delegate_signature_graphs: TArray<UPtr<UEdGraph>>,
    pub macro_graphs: TArray<UPtr<UEdGraph>>,
    pub intermediate_generated_graphs: TArray<UPtr<UEdGraph>>,
    pub event_graphs: TArray<UPtr<UEdGraph>>,
    pub private_cached_macro_info: TMap<UPtr<UEdGraph>, FBlueprintMacroCosmeticInfo>,
    pub component_templates: TArray<UPtr<UActorComponent>>,
    pub timelines: TArray<UPtr<UTimelineTemplate>>,
    pub component_class_overrides: TArray<FBPComponentClassOverride>,
    pub inheritable_component_handler: UPtr<UInheritableComponentHandler>,
    pub new_variables: TArray<FBPVariableDescription>,
    pub generated_variables: TArray<FBPVariableDescription>,
    pub category_sorting: TArray<FName>,
    pub imported_namespaces: TSet<FString>,
    pub implemented_interfaces: TArray<FBPInterfaceDescription>,
    pub last_edited_documents: TArray<FEditedDocumentInfo>,
    pub bookmarks: TMap<crate::bindings::core_u_object::FGuid, FEditedDocumentInfo>,
    pub bookmark_nodes: TArray<FBPEditorBookmarkNode>,
    pub breakpoints_deprecated: TArray<UPtr<UDEPRECATED_Breakpoint>>,
    pub watched_pins_deprecated: TArray<FEdGraphPinReference>,
    pub deprecated_pin_watches: TArray<UPtr<UEdGraphPin_Deprecated>>,
    pub component_template_name_index: TMap<FName, i32>,
    pub old_to_new_component_template_names: TMap<FName, FName>,
    pub extensions: TArray<UPtr<UBlueprintExtension>>,
    pub thumbnail_info: UPtr<UThumbnailInfo>,
    pub crc_last_compiled_cdo: u32,
    pub crc_last_compiled_signature: u32,
    pub b_cached_dependencies_up_to_date: bool,
    pub cached_dependencies: TSet<TWeakObjectPtr<UBlueprint>>,
    pub cached_dependents: TSet<TWeakObjectPtr<UBlueprint>>,
    pub cached_uds_dependencies: TSet<
        TWeakObjectPtr<crate::bindings::core_u_object::UStruct>,
    >,
    pub original_class: TSubclassOf<crate::bindings::core_u_object::UObject>,
}
pub struct UDynamicBlueprintBinding {}
pub struct UBlueprintGeneratedClass {
    pub num_replicated_properties: i32,
    pub flags_636: u8,
    pub dynamic_binding_objects: TArray<UPtr<UDynamicBlueprintBinding>>,
    pub component_templates: TArray<UPtr<UActorComponent>>,
    pub timelines: TArray<UPtr<UTimelineTemplate>>,
    pub component_class_overrides: TArray<FBPComponentClassOverride>,
    pub field_notifies: TArray<
        crate::bindings::field_notification::FFieldNotificationId,
    >,
    pub simple_construction_script: UPtr<USimpleConstructionScript>,
    pub inheritable_component_handler: UPtr<UInheritableComponentHandler>,
    pub uber_graph_function: UPtr<crate::bindings::core_u_object::UFunction>,
    pub fast_call_pairs_deprecated: TArray<FEventGraphFastCallPair>,
    pub overriden_archetype_for_cdo: UPtr<crate::bindings::core_u_object::UObject>,
    pub property_guids: TMap<FName, crate::bindings::core_u_object::FGuid>,
    pub called_functions: TArray<UPtr<crate::bindings::core_u_object::UFunction>>,
    pub cooked_property_guids: TMap<FName, crate::bindings::core_u_object::FGuid>,
    pub cooked_component_instancing_data: TMap<
        FName,
        FBlueprintCookedComponentInstancingData,
    >,
    pub cached_cooked_meta_data_ptr: UPtr<
        crate::bindings::core_u_object::UClassCookedMetaData,
    >,
}
pub struct UGameInstance {
    pub local_players: TArray<UPtr<ULocalPlayer>>,
    pub online_session: UPtr<UOnlineSession>,
    pub referenced_objects: TArray<UPtr<crate::bindings::core_u_object::UObject>>,
    pub on_pawn_controller_changed_delegates: FGameInstance_OnPawnControllerChangedDelegates,
    pub on_input_device_connection_change: FGameInstance_OnInputDeviceConnectionChange,
    pub on_user_input_device_pairing_change: FGameInstance_OnUserInputDevicePairingChange,
}
pub struct UAssetImportData {
    pub source_file_path_deprecated: FString,
    pub source_file_timestamp_deprecated: FString,
    pub source_data: FAssetImportInfo,
}
pub struct USkinnedMeshComponent {
    pub skeletal_mesh: UPtr<USkeletalMesh>,
    pub skinned_asset: UPtr<USkinnedAsset>,
    pub leader_pose_component: TWeakObjectPtr<USkinnedMeshComponent>,
    pub skin_cache_usage: TArray<ESkinCacheUsage>,
    pub b_set_mesh_deformer: bool,
    pub mesh_deformer: UPtr<UMeshDeformer>,
    pub b_always_use_mesh_deformer: bool,
    pub mesh_deformer_instance_settings: UPtr<UMeshDeformerInstanceSettings>,
    pub mesh_deformer_instance_deprecated: UPtr<UMeshDeformerInstance>,
    pub mesh_deformer_instances: FMeshDeformerInstanceSet,
    pub wireframe_color_deprecated: crate::bindings::core_u_object::FColor,
    pub physics_asset_override: UPtr<UPhysicsAsset>,
    pub forced_lod_model: i32,
    pub min_lod_model: i32,
    pub streaming_distance_multiplier: f32,
    pub nanite_pixel_programmable_distance: f32,
    pub lod_info: TArray<FSkelMeshComponentLODInfo>,
    pub visibility_based_anim_tick_option: EVisibilityBasedAnimTickOption,
    pub flags_2266: u8,
    pub flags_2267: u8,
    pub flags_2268: u8,
    pub flags_2269: u8,
    pub flags_2270: u8,
    pub capsule_indirect_shadow_min_visibility: f32,
    pub cached_world_or_local_space_bounds: crate::bindings::core_u_object::FBoxSphereBounds,
    pub cached_world_to_local_transform: crate::bindings::core_u_object::FMatrix,
}
pub struct USkeletalMeshComponent {
    pub skeletal_mesh_asset: UPtr<USkeletalMesh>,
    pub animation_blueprint_deprecated: UPtr<UAnimBlueprint>,
    pub anim_blueprint_generated_class: TSubclassOf<
        crate::bindings::core_u_object::UObject,
    >,
    pub anim_class: TSubclassOf<UAnimInstance>,
    pub anim_script_instance: UPtr<UAnimInstance>,
    pub sub_instances: TArray<UPtr<UAnimInstance>>,
    pub override_post_process_anim_bp: TSubclassOf<UAnimInstance>,
    pub post_process_anim_instance: UPtr<UAnimInstance>,
    pub animation_data: FSingleAnimationPlayData,
    pub root_bone_translation: crate::bindings::core_u_object::FVector,
    pub line_check_bounds_scale: crate::bindings::core_u_object::FVector,
    pub linked_instances: TArray<UPtr<UAnimInstance>>,
    pub cached_bone_space_transforms: TArray<crate::bindings::core_u_object::FTransform>,
    pub cached_component_space_transforms: TArray<
        crate::bindings::core_u_object::FTransform,
    >,
    pub global_anim_rate_scale: f32,
    pub kinematic_bones_update_type: EKinematicBonesUpdateToPhysics,
    pub physics_transform_update_mode: EPhysicsTransformUpdateMode,
    pub cloth_teleport_mode: crate::bindings::clothing_system_runtime_interface::EClothingTeleportMode,
    pub animation_mode: EAnimationMode,
    pub flags_3025: u8,
    pub flags_3026: u8,
    pub flags_3032: u8,
    pub flags_3033: u8,
    pub cloth_velocity_scale: f32,
    pub flags_3040: u8,
    pub flags_3041: u8,
    pub flags_3042: u8,
    pub flags_3043: u8,
    pub cached_anim_curve_uid_version: u16,
    pub cloth_blend_weight: f32,
    pub b_wait_for_parallel_cloth_task: bool,
    pub b_filtered_anim_curves_is_allow_list: bool,
    pub cached_mesh_curve_meta_data_version: u16,
    pub filtered_anim_curves: TArray<FName>,
    pub body_setup: UPtr<UBodySetup>,
    pub cloth_max_distance_scale: f32,
    pub cloth_geometry_scale: f32,
    pub post_process_anim_bplod_threshold: i32,
    pub on_constraint_broken: FSkeletalMeshComponent_OnConstraintBroken,
    pub on_plastic_deformation: FSkeletalMeshComponent_OnPlasticDeformation,
    pub clothing_simulation_factory: TSubclassOf<
        crate::bindings::clothing_system_runtime_interface::UClothingSimulationFactory,
    >,
    pub teleport_distance_threshold: f32,
    pub teleport_rotation_threshold: f32,
    pub clothing_simulation_instances: TArray<
        crate::bindings::clothing_system_runtime_interface::FClothingSimulationInstance,
    >,
    pub morph_target_curves: TMap<FName, f32>,
    pub on_anim_initialized: FSkeletalMeshComponent_OnAnimInitialized,
    pub sequence_to_play_deprecated: UPtr<UAnimSequence>,
    pub anim_to_play_deprecated: UPtr<UAnimationAsset>,
    pub flags_4248: u8,
    pub default_position_deprecated: f32,
    pub default_play_rate_deprecated: f32,
    pub default_animating_rig_override: TSoftObjectPtr<
        crate::bindings::core_u_object::UObject,
    >,
    pub last_pose_tick_frame: u32,
}
pub struct UAnimInstance {
    pub current_skeleton: UPtr<USkeleton>,
    pub root_motion_mode: ERootMotionMode,
    pub flags_57: u8,
    pub flags_59: u8,
    pub on_montage_blending_out: FAnimInstance_OnMontageBlendingOut,
    pub on_montage_blended_in: FAnimInstance_OnMontageBlendedIn,
    pub on_montage_started: FAnimInstance_OnMontageStarted,
    pub on_montage_ended: FAnimInstance_OnMontageEnded,
    pub on_all_montage_instances_ended: FAnimInstance_OnAllMontageInstancesEnded,
    pub on_montage_section_changed: FAnimInstance_OnMontageSectionChanged,
    pub slot_group_inertialization_request_data_map: TMap<
        FName,
        FInertializationRequest,
    >,
    pub post_compile_validation_class_name: crate::bindings::core_u_object::FSoftClassPath,
    pub notify_queue: FAnimNotifyQueue,
    pub active_anim_notify_state: TArray<FAnimNotifyEvent>,
    pub active_anim_notify_event_reference: TArray<FAnimNotifyEventReference>,
}
pub struct UAnimSingleNodeInstance {
    pub current_asset: UPtr<UAnimationAsset>,
    pub post_evaluate_anim_event: FAnimSingleNodeInstance_PostEvaluateAnimEvent,
}
pub struct UAnimNotify {
    pub notify_color: crate::bindings::core_u_object::FColor,
    pub b_should_fire_in_editor: bool,
}
pub struct UAnimNotifyState {
    pub notify_color: crate::bindings::core_u_object::FColor,
    pub b_should_fire_in_editor: bool,
}
pub struct UEngineCustomTimeStep {}
pub struct UTimecodeProvider {
    pub frame_delay: f32,
}
pub struct UAudioEngineSubsystem {}
pub struct USoundBase {
    pub sound_class_object: UPtr<USoundClass>,
    pub editor_data: FSoundBaseEditorData,
    pub flags_73: u8,
    pub flags_74: u8,
    pub virtualization_mode: EVirtualizationMode,
    pub max_concurrent_resolution_rule_deprecated: EMaxConcurrentResolutionRule,
    pub sound_concurrency_settings_deprecated: UPtr<USoundConcurrency>,
    pub concurrency_set: TSet<UPtr<USoundConcurrency>>,
    pub concurrency_overrides: FSoundConcurrencySettings,
    pub max_concurrent_play_count_deprecated: i32,
    pub duration: f32,
    pub max_distance: f32,
    pub total_samples: f32,
    pub priority: f32,
    pub attenuation_settings: UPtr<USoundAttenuation>,
    pub sound_submix_object: UPtr<USoundSubmixBase>,
    pub sound_submix_sends: TArray<FSoundSubmixSendInfo>,
    pub source_effect_chain: UPtr<USoundEffectSourcePresetChain>,
    pub bus_sends: TArray<FSoundSourceBusSendInfo>,
    pub pre_effect_bus_sends: TArray<FSoundSourceBusSendInfo>,
    pub asset_user_data: TArray<UPtr<UAssetUserData>>,
    pub audio_properties_sheet: UPtr<
        crate::bindings::audio_extensions::UAudioPropertiesSheetAssetBase,
    >,
    pub timecode_offset: FSoundTimecodeOffset,
    pub local_audio_properties: TArray<FName>,
}
pub struct USoundWave {
    pub compression_quality: i32,
    pub streaming_priority: i32,
    pub sample_rate_quality: crate::bindings::audio_platform_configuration::ESoundwaveSampleRateSettings,
    pub sound_group: ESoundGroup,
    pub flags_547: u8,
    pub sound_asset_compression_type: ESoundAssetCompressionType,
    pub override_sound_to_use_for_analysis: UPtr<USoundWave>,
    pub flags_576: u8,
    pub fft_size: ESoundWaveFFTSize,
    pub fft_analysis_frame_size: i32,
    pub fft_analysis_attack_time: i32,
    pub fft_analysis_release_time: i32,
    pub envelope_follower_frame_size: i32,
    pub envelope_follower_attack_time: i32,
    pub envelope_follower_release_time: i32,
    pub modulation_settings: FSoundModulationDefaultRoutingSettings,
    pub frequencies_to_analyze: TArray<f32>,
    pub cooked_spectral_time_data: TArray<FSoundWaveSpectralTimeData>,
    pub cooked_envelope_time_data: TArray<FSoundWaveEnvelopeTimeData>,
    pub initial_chunk_size_deprecated: i32,
    pub flags_1084: u8,
    pub flags_1085: u8,
    pub loading_behavior: ESoundWaveLoadingBehavior,
    pub size_of_first_audio_chunk_in_seconds: crate::bindings::core_u_object::FPerPlatformFloat,
    pub spoken_text_deprecated: FString,
    pub subtitle_priority: f32,
    pub volume: f32,
    pub pitch: f32,
    pub num_channels: i32,
    pub channel_offsets: TArray<i32>,
    pub channel_sizes: TArray<i32>,
    pub lufs: f32,
    pub sample_peak_db: f32,
    pub sample_rate: i32,
    pub imported_sample_rate: i32,
    pub cue_points: TArray<crate::bindings::audio_extensions::FSoundWaveCuePoint>,
    pub cue_point_origin: ESoundWaveCuePointOrigin,
    pub subtitles: TArray<FSubtitleCue>,
    pub comment: FString,
    pub timecode_info: FSoundWaveTimecodeInfo,
    pub source_file_path_deprecated: FString,
    pub source_file_timestamp_deprecated: FString,
    pub asset_import_data: UPtr<UAssetImportData>,
    pub curves: UPtr<UCurveTable>,
    pub internal_curves: UPtr<UCurveTable>,
    pub flags_1496: u8,
    pub platform_settings: TMap<
        crate::bindings::core_u_object::FGuid,
        crate::bindings::audio_extensions::FSoundWaveCloudStreamingPlatformSettings,
    >,
    pub transformations: TArray<
        UPtr<crate::bindings::audio_extensions::UWaveformTransformationBase>,
    >,
}
pub struct USoundWaveProcedural {}
pub struct USoundEffectPreset {}
pub struct USoundEffectSubmixPreset {}
pub struct UStreamableRenderAsset {
    pub force_mip_levels_to_be_resident_timestamp: f64,
    pub num_cinematic_mip_levels: i32,
    pub no_ref_streaming_lod_bias: FPerQualityLevelInt,
    pub streaming_index: i32,
    pub flags_240: u8,
}
pub struct UTexture {
    pub source: FTextureSource,
    pub lighting_guid: crate::bindings::core_u_object::FGuid,
    pub source_file_path_deprecated: FString,
    pub asset_import_data: UPtr<UAssetImportData>,
    pub adjust_brightness: f32,
    pub adjust_brightness_curve: f32,
    pub adjust_vibrance: f32,
    pub adjust_saturation: f32,
    pub adjust_rgb_curve: f32,
    pub adjust_hue: f32,
    pub adjust_min_alpha: f32,
    pub adjust_max_alpha: f32,
    pub flags_696: u8,
    pub lossy_compression_amount: ETextureLossyCompressionAmount,
    pub oodle_texture_sdk_version: FName,
    pub max_texture_size: i32,
    pub compression_quality: ETextureCompressionQuality,
    pub compression_cache_id: crate::bindings::core_u_object::FGuid,
    pub flags_740: u8,
    pub b_do_scale_mips_for_alpha_coverage: bool,
    pub alpha_coverage_thresholds: crate::bindings::core_u_object::FVector4,
    pub b_use_new_mip_filter: bool,
    pub flags_788: u8,
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
    pub composite_texture_mode: ECompositeTextureMode,
    pub composite_power: f32,
    pub layer_format_settings: TArray<FTextureFormatSettings>,
    pub level_index: i32,
    pub lod_bias: i32,
    pub compression_settings: TextureCompressionSettings,
    pub filter: TextureFilter,
    pub mip_load_options: ETextureMipLoadOptions,
    pub virtual_texture_streaming_priority: crate::bindings::render_core::EVTProducerPriority,
    pub virtual_texture_prefetch_mips: u8,
    pub cook_platform_tiling_settings: TextureCookPlatformTilingSettings,
    pub b_oodle_preserve_extremes: bool,
    pub lod_group: TextureGroup,
    pub downscale: crate::bindings::core_u_object::FPerPlatformFloat,
    pub downscale_options: ETextureDownscaleOptions,
    pub availability: ETextureAvailability,
    pub flags_978: u8,
    pub source_color_settings: FTextureSourceColorSettings,
    pub flags_1100: u8,
    pub flags_1104: u8,
    pub flags_1108: u8,
    pub asset_user_data: TArray<UPtr<UAssetUserData>>,
}
pub struct UAssetUserData {}
pub struct UExporter {
    pub supported_class: TSubclassOf<crate::bindings::core_u_object::UObject>,
    pub export_root_scope: UPtr<crate::bindings::core_u_object::UObject>,
    pub format_extension: TArray<FString>,
    pub format_description: TArray<FString>,
    pub preferred_format_index: i32,
    pub text_indent: i32,
    pub flags_104: u8,
    pub export_task: UPtr<UAssetExportTask>,
}
pub struct UCommandlet {
    pub help_description: FString,
    pub help_usage: FString,
    pub help_web_link: FString,
    pub help_param_names: TArray<FString>,
    pub help_param_descriptions: TArray<FString>,
    pub flags_128: u8,
}
pub struct UBrushBuilder {
    pub bitmap_filename: FString,
    pub tool_tip: FString,
    pub flags_80: u8,
    pub vertices: TArray<crate::bindings::core_u_object::FVector>,
    pub polys: TArray<FBuilderPoly>,
    pub layer: FName,
    pub flags_132: u8,
}
pub struct UEngine {
    pub tiny_font: UPtr<UFont>,
    pub tiny_font_name: crate::bindings::core_u_object::FSoftObjectPath,
    pub small_font: UPtr<UFont>,
    pub small_font_name: crate::bindings::core_u_object::FSoftObjectPath,
    pub medium_font: UPtr<UFont>,
    pub medium_font_name: crate::bindings::core_u_object::FSoftObjectPath,
    pub large_font: UPtr<UFont>,
    pub large_font_name: crate::bindings::core_u_object::FSoftObjectPath,
    pub subtitle_font: UPtr<UFont>,
    pub subtitle_font_name: crate::bindings::core_u_object::FSoftObjectPath,
    pub monospace_font: UPtr<UFont>,
    pub monospace_font_name: crate::bindings::core_u_object::FSoftObjectPath,
    pub additional_fonts: TArray<UPtr<UFont>>,
    pub additional_font_names: TArray<FString>,
    pub console_class: TSubclassOf<UConsole>,
    pub console_class_name: crate::bindings::core_u_object::FSoftClassPath,
    pub game_viewport_client_class: TSubclassOf<UGameViewportClient>,
    pub game_viewport_client_class_name: crate::bindings::core_u_object::FSoftClassPath,
    pub local_player_class: TSubclassOf<ULocalPlayer>,
    pub local_player_class_name: crate::bindings::core_u_object::FSoftClassPath,
    pub world_settings_class: TSubclassOf<AWorldSettings>,
    pub world_settings_class_name: crate::bindings::core_u_object::FSoftClassPath,
    pub navigation_system_class_name: crate::bindings::core_u_object::FSoftClassPath,
    pub navigation_system_class: TSubclassOf<UNavigationSystemBase>,
    pub navigation_system_config_class_name: crate::bindings::core_u_object::FSoftClassPath,
    pub navigation_system_config_class: TSubclassOf<UNavigationSystemConfig>,
    pub avoidance_manager_class_name: crate::bindings::core_u_object::FSoftClassPath,
    pub avoidance_manager_class: TSubclassOf<UAvoidanceManager>,
    pub ai_controller_class_name: crate::bindings::core_u_object::FSoftClassPath,
    pub physics_collision_handler_class: TSubclassOf<UPhysicsCollisionHandler>,
    pub physics_collision_handler_class_name: crate::bindings::core_u_object::FSoftClassPath,
    pub game_user_settings_class_name: crate::bindings::core_u_object::FSoftClassPath,
    pub game_user_settings_class: TSubclassOf<UGameUserSettings>,
    pub game_user_settings: UPtr<UGameUserSettings>,
    pub level_script_actor_class: TSubclassOf<ALevelScriptActor>,
    pub level_script_actor_class_name: crate::bindings::core_u_object::FSoftClassPath,
    pub default_blueprint_base_class_name: crate::bindings::core_u_object::FSoftClassPath,
    pub game_singleton_class_name: crate::bindings::core_u_object::FSoftClassPath,
    pub game_singleton: UPtr<crate::bindings::core_u_object::UObject>,
    pub asset_manager_class_name: crate::bindings::core_u_object::FSoftClassPath,
    pub asset_manager: UPtr<UAssetManager>,
    pub default_texture: UPtr<UTexture2D>,
    pub default_texture_name: crate::bindings::core_u_object::FSoftObjectPath,
    pub default_diffuse_texture: UPtr<UTexture>,
    pub default_diffuse_texture_name: crate::bindings::core_u_object::FSoftObjectPath,
    pub default_bsp_vertex_texture: UPtr<UTexture2D>,
    pub default_bsp_vertex_texture_name: crate::bindings::core_u_object::FSoftObjectPath,
    pub high_frequency_noise_texture: UPtr<UTexture2D>,
    pub high_frequency_noise_texture_name: crate::bindings::core_u_object::FSoftObjectPath,
    pub default_bokeh_texture: UPtr<UTexture2D>,
    pub default_bokeh_texture_name: crate::bindings::core_u_object::FSoftObjectPath,
    pub default_bloom_kernel_texture: UPtr<UTexture2D>,
    pub default_bloom_kernel_texture_name: crate::bindings::core_u_object::FSoftObjectPath,
    pub default_film_grain_texture: UPtr<UTexture2D>,
    pub default_film_grain_texture_name: crate::bindings::core_u_object::FSoftObjectPath,
    pub wireframe_material: UPtr<UMaterial>,
    pub wireframe_material_name: FString,
    pub geom_material: UPtr<UMaterial>,
    pub geom_material_name: crate::bindings::core_u_object::FSoftObjectPath,
    pub debug_mesh_material: UPtr<UMaterial>,
    pub debug_mesh_material_name: crate::bindings::core_u_object::FSoftObjectPath,
    pub nanite_hidden_section_material: UPtr<UMaterial>,
    pub nanite_hidden_section_material_name: FString,
    pub emissive_mesh_material: UPtr<UMaterial>,
    pub emissive_mesh_material_name: crate::bindings::core_u_object::FSoftObjectPath,
    pub level_coloration_lit_material: UPtr<UMaterial>,
    pub level_coloration_lit_material_name: FString,
    pub level_coloration_unlit_material: UPtr<UMaterial>,
    pub level_coloration_unlit_material_name: FString,
    pub lighting_texel_density_material: UPtr<UMaterial>,
    pub lighting_texel_density_name: FString,
    pub shaded_level_coloration_lit_material: UPtr<UMaterial>,
    pub shaded_level_coloration_lit_material_name: FString,
    pub shaded_level_coloration_unlit_material: UPtr<UMaterial>,
    pub shaded_level_coloration_unlit_material_name: FString,
    pub remove_surface_material: UPtr<UMaterial>,
    pub remove_surface_material_name: crate::bindings::core_u_object::FSoftObjectPath,
    pub vertex_color_material: UPtr<UMaterial>,
    pub vertex_color_material_name: FString,
    pub vertex_color_view_mode_material_color_only: UPtr<UMaterial>,
    pub vertex_color_view_mode_material_name_color_only: FString,
    pub vertex_color_view_mode_material_alpha_as_color: UPtr<UMaterial>,
    pub vertex_color_view_mode_material_name_alpha_as_color: FString,
    pub vertex_color_view_mode_material_red_only: UPtr<UMaterial>,
    pub vertex_color_view_mode_material_name_red_only: FString,
    pub vertex_color_view_mode_material_green_only: UPtr<UMaterial>,
    pub vertex_color_view_mode_material_name_green_only: FString,
    pub vertex_color_view_mode_material_blue_only: UPtr<UMaterial>,
    pub vertex_color_view_mode_material_name_blue_only: FString,
    pub texture_color_view_mode_material: UPtr<UMaterial>,
    pub texture_color_view_mode_material_name: FString,
    pub default_zen_streaming_texture_name: crate::bindings::core_u_object::FSoftObjectPath,
    pub bone_weight_material: UPtr<UMaterial>,
    pub bone_weight_material_name: crate::bindings::core_u_object::FSoftObjectPath,
    pub cloth_paint_material: UPtr<UMaterial>,
    pub cloth_paint_opaque_material: UPtr<UMaterial>,
    pub cloth_paint_material_wireframe: UPtr<UMaterial>,
    pub cloth_paint_opaque_material_wireframe: UPtr<UMaterial>,
    pub cloth_paint_material_instance: UPtr<UMaterialInstanceDynamic>,
    pub cloth_paint_opaque_material_instance: UPtr<UMaterialInstanceDynamic>,
    pub cloth_paint_material_wireframe_instance: UPtr<UMaterialInstanceDynamic>,
    pub cloth_paint_opaque_material_wireframe_instance: UPtr<UMaterialInstanceDynamic>,
    pub cloth_paint_material_name: crate::bindings::core_u_object::FSoftObjectPath,
    pub cloth_paint_opaque_material_name: crate::bindings::core_u_object::FSoftObjectPath,
    pub cloth_paint_material_wireframe_name: crate::bindings::core_u_object::FSoftObjectPath,
    pub cloth_paint_opaque_material_wireframe_name: crate::bindings::core_u_object::FSoftObjectPath,
    pub physical_material_mask_material: UPtr<UMaterial>,
    pub physical_material_mask_material_name: crate::bindings::core_u_object::FSoftObjectPath,
    pub debug_editor_material: UPtr<UMaterial>,
    pub default_flatten_material_name: crate::bindings::core_u_object::FSoftObjectPath,
    pub default_hlod_flatten_material_name: crate::bindings::core_u_object::FSoftObjectPath,
    pub default_landscape_flatten_material_name: crate::bindings::core_u_object::FSoftObjectPath,
    pub default_flatten_material: UPtr<UMaterial>,
    pub default_hlod_flatten_material: UPtr<UMaterial>,
    pub default_landscape_flatten_material: UPtr<UMaterial>,
    pub texture_painting_mask_material: UPtr<UMaterial>,
    pub texture_painting_mask_material_name: crate::bindings::core_u_object::FSoftObjectPath,
    pub clay_material: UPtr<UMaterialInterface>,
    pub clay_material_name: crate::bindings::core_u_object::FSoftObjectPath,
    pub zebra_material: UPtr<UMaterial>,
    pub zebra_material_instance: UPtr<UMaterialInstanceDynamic>,
    pub zebra_material_name: crate::bindings::core_u_object::FSoftObjectPath,
    pub front_back_face_material: UPtr<UMaterialInterface>,
    pub front_back_face_material_name: crate::bindings::core_u_object::FSoftObjectPath,
    pub random_color_material: UPtr<UMaterialInterface>,
    pub random_color_material_name: crate::bindings::core_u_object::FSoftObjectPath,
    pub debug_editor_material_name: crate::bindings::core_u_object::FSoftObjectPath,
    pub constraint_limit_material: UPtr<UMaterial>,
    pub constraint_limit_material_x: UPtr<UMaterialInstanceDynamic>,
    pub constraint_limit_material_x_axis: UPtr<UMaterialInstanceDynamic>,
    pub constraint_limit_material_y: UPtr<UMaterialInstanceDynamic>,
    pub constraint_limit_material_y_axis: UPtr<UMaterialInstanceDynamic>,
    pub constraint_limit_material_z: UPtr<UMaterialInstanceDynamic>,
    pub constraint_limit_material_z_axis: UPtr<UMaterialInstanceDynamic>,
    pub constraint_limit_material_prismatic: UPtr<UMaterialInstanceDynamic>,
    pub invalid_lightmap_settings_material: UPtr<UMaterial>,
    pub invalid_lightmap_settings_material_name: crate::bindings::core_u_object::FSoftObjectPath,
    pub preview_shadows_indicator_material: UPtr<UMaterial>,
    pub preview_shadows_indicator_material_name: crate::bindings::core_u_object::FSoftObjectPath,
    pub arrow_material: UPtr<UMaterial>,
    pub arrow_material_yellow: UPtr<UMaterialInstanceDynamic>,
    pub arrow_material_name: crate::bindings::core_u_object::FSoftObjectPath,
    pub lighting_only_brightness: crate::bindings::core_u_object::FLinearColor,
    pub shader_complexity_colors: TArray<crate::bindings::core_u_object::FLinearColor>,
    pub quad_complexity_colors: TArray<crate::bindings::core_u_object::FLinearColor>,
    pub light_complexity_colors: TArray<crate::bindings::core_u_object::FLinearColor>,
    pub stationary_light_overlap_colors: TArray<
        crate::bindings::core_u_object::FLinearColor,
    >,
    pub lod_coloration_colors: TArray<crate::bindings::core_u_object::FLinearColor>,
    pub hlod_coloration_colors: TArray<crate::bindings::core_u_object::FLinearColor>,
    pub streaming_accuracy_colors: TArray<crate::bindings::core_u_object::FLinearColor>,
    pub gpu_skin_cache_visualization_excluded_color: crate::bindings::core_u_object::FLinearColor,
    pub gpu_skin_cache_visualization_included_color: crate::bindings::core_u_object::FLinearColor,
    pub gpu_skin_cache_visualization_recompute_tangents_color: crate::bindings::core_u_object::FLinearColor,
    pub gpu_skin_cache_visualization_low_memory_threshold_in_mb: f32,
    pub gpu_skin_cache_visualization_high_memory_threshold_in_mb: f32,
    pub gpu_skin_cache_visualization_low_memory_color: crate::bindings::core_u_object::FLinearColor,
    pub gpu_skin_cache_visualization_mid_memory_color: crate::bindings::core_u_object::FLinearColor,
    pub gpu_skin_cache_visualization_high_memory_color: crate::bindings::core_u_object::FLinearColor,
    pub gpu_skin_cache_visualization_ray_tracing_lod_offset_colors: TArray<
        crate::bindings::core_u_object::FLinearColor,
    >,
    pub max_pixel_shader_additive_complexity_count: f32,
    pub max_es3_pixel_shader_additive_complexity_count: f32,
    pub min_light_map_density: f32,
    pub ideal_light_map_density: f32,
    pub max_light_map_density: f32,
    pub flags_3188: u8,
    pub render_light_map_density_grayscale_scale: f32,
    pub render_light_map_density_color_scale: f32,
    pub light_map_density_vertex_mapped_color: crate::bindings::core_u_object::FLinearColor,
    pub light_map_density_selected_color: crate::bindings::core_u_object::FLinearColor,
    pub stat_color_mappings: TArray<FStatColorMapping>,
    pub editor_brush_material: UPtr<UMaterial>,
    pub editor_brush_material_name: crate::bindings::core_u_object::FSoftObjectPath,
    pub default_phys_material: UPtr<crate::bindings::physics_core::UPhysicalMaterial>,
    pub default_phys_material_name: crate::bindings::core_u_object::FSoftObjectPath,
    pub default_destructible_phys_material: UPtr<
        crate::bindings::physics_core::UPhysicalMaterial,
    >,
    pub default_destructible_phys_material_name: crate::bindings::core_u_object::FSoftObjectPath,
    pub active_game_name_redirects: TArray<FGameNameRedirect>,
    pub active_class_redirects: TArray<FClassRedirect>,
    pub active_plugin_redirects: TArray<FPluginRedirect>,
    pub active_struct_redirects: TArray<FStructRedirect>,
    pub pre_integrated_skin_brdf_texture: UPtr<UTexture2D>,
    pub pre_integrated_skin_brdf_texture_name: crate::bindings::core_u_object::FSoftObjectPath,
    pub blue_noise_scalar_texture: UPtr<UTexture2D>,
    pub blue_noise_vec2_texture: UPtr<UTexture2D>,
    pub blue_noise_scalar_texture_name: crate::bindings::core_u_object::FSoftObjectPath,
    pub blue_noise_scalar_mobile_texture_name: crate::bindings::core_u_object::FSoftObjectPath,
    pub blue_noise_vec2_texture_name: crate::bindings::core_u_object::FSoftObjectPath,
    pub ggxltc_amp_texture: UPtr<UTexture2D>,
    pub ggxltc_amp_texture_name: crate::bindings::core_u_object::FSoftObjectPath,
    pub ggxltc_mat_texture: UPtr<UTexture2D>,
    pub ggxltc_mat_texture_name: crate::bindings::core_u_object::FSoftObjectPath,
    pub sheen_ltc_texture: UPtr<UTexture2D>,
    pub sheen_ltc_texture_name: crate::bindings::core_u_object::FSoftObjectPath,
    pub ggx_reflection_energy_texture: UPtr<UTexture2D>,
    pub ggx_reflection_energy_texture_name: crate::bindings::core_u_object::FSoftObjectPath,
    pub ggx_transmission_energy_texture: UPtr<UTexture2D>,
    pub ggx_transmission_energy_texture_name: crate::bindings::core_u_object::FSoftObjectPath,
    pub sheen_energy_texture: UPtr<UTexture2D>,
    pub sheen_legacy_energy_texture_name: crate::bindings::core_u_object::FSoftObjectPath,
    pub sheen_energy_texture_name: crate::bindings::core_u_object::FSoftObjectPath,
    pub diffuse_energy_texture: UPtr<UTexture2D>,
    pub diffuse_energy_texture_name: crate::bindings::core_u_object::FSoftObjectPath,
    pub glint_texture: UPtr<UTexture2DArray>,
    pub glint_texture2: UPtr<UTexture2DArray>,
    pub glint_texture_name: crate::bindings::core_u_object::FSoftObjectPath,
    pub glint_texture2_name: crate::bindings::core_u_object::FSoftObjectPath,
    pub simple_volume_texture: UPtr<UVolumeTexture>,
    pub simple_volume_texture_name: crate::bindings::core_u_object::FSoftObjectPath,
    pub simple_volume_env_texture: UPtr<UVolumeTexture>,
    pub simple_volume_env_texture_name: crate::bindings::core_u_object::FSoftObjectPath,
    pub mini_font_texture: UPtr<UTexture2D>,
    pub mini_font_texture_name: crate::bindings::core_u_object::FSoftObjectPath,
    pub weight_map_placeholder_texture: UPtr<UTexture>,
    pub weight_map_array_placeholder_texture: UPtr<UTexture>,
    pub weight_map_placeholder_texture_name: crate::bindings::core_u_object::FSoftObjectPath,
    pub weight_map_array_placeholder_texture_name: crate::bindings::core_u_object::FSoftObjectPath,
    pub light_map_density_texture: UPtr<UTexture2D>,
    pub light_map_density_texture_name: crate::bindings::core_u_object::FSoftObjectPath,
    pub smaa_area_texture: UPtr<UTexture2D>,
    pub smaa_area_texture_name: crate::bindings::core_u_object::FSoftObjectPath,
    pub smaa_search_texture: UPtr<UTexture2D>,
    pub smaa_search_texture_name: crate::bindings::core_u_object::FSoftObjectPath,
    pub game_viewport: UPtr<UGameViewportClient>,
    pub deferred_commands: TArray<FString>,
    pub near_clip_plane: f32,
    pub flags_4532: u8,
    pub maximum_loop_iteration_count: i32,
    pub flags_4540: u8,
    pub fixed_frame_rate: f32,
    pub smoothed_frame_rate_range: crate::bindings::core_u_object::FFloatRange,
    pub custom_time_step: UPtr<UEngineCustomTimeStep>,
    pub custom_time_step_class_name: crate::bindings::core_u_object::FSoftClassPath,
    pub timecode_provider: UPtr<UTimecodeProvider>,
    pub timecode_provider_class_name: crate::bindings::core_u_object::FSoftClassPath,
    pub b_generate_default_timecode: bool,
    pub generate_default_timecode_frame_rate: crate::bindings::core_u_object::FFrameRate,
    pub generate_default_timecode_frame_delay: f32,
    pub flags_4760: u8,
    pub num_pawns_allowed_to_be_spawned_in_a_frame: i32,
    pub flags_4768: u8,
    pub c_world_box: crate::bindings::core_u_object::FColor,
    pub c_brush_wire: crate::bindings::core_u_object::FColor,
    pub c_add_wire: crate::bindings::core_u_object::FColor,
    pub c_subtract_wire: crate::bindings::core_u_object::FColor,
    pub c_semi_solid_wire: crate::bindings::core_u_object::FColor,
    pub c_non_solid_wire: crate::bindings::core_u_object::FColor,
    pub c_wire_background: crate::bindings::core_u_object::FColor,
    pub c_scale_box_hi: crate::bindings::core_u_object::FColor,
    pub c_volume_collision: crate::bindings::core_u_object::FColor,
    pub c_bsp_collision: crate::bindings::core_u_object::FColor,
    pub c_ortho_background: crate::bindings::core_u_object::FColor,
    pub c_volume: crate::bindings::core_u_object::FColor,
    pub c_brush_shape: crate::bindings::core_u_object::FColor,
    pub game_screenshot_save_directory: crate::bindings::core_u_object::FDirectoryPath,
    pub use_static_mesh_min_lod_per_quality_levels: bool,
    pub use_skeletal_mesh_min_lod_per_quality_levels: bool,
    pub use_cloth_asset_min_lod_per_quality_levels: bool,
    pub use_grass_varity_per_quality_levels: bool,
    pub transition_type: ETransitionType,
    pub transition_description: FString,
    pub transition_game_mode: FString,
    pub flags_4880: u8,
    pub camera_rotation_threshold: f32,
    pub camera_translation_threshold: f32,
    pub primitive_probably_visible_time: f32,
    pub max_occlusion_pixels_fraction: f32,
    pub flags_4900: u8,
    pub max_particle_resize: i32,
    pub max_particle_resize_warn: i32,
    pub pending_dropped_notes: TArray<FDropNoteInfo>,
    pub net_client_ticks_per_second: f32,
    pub display_gamma: f32,
    pub min_desired_frame_rate: f32,
    pub default_selected_material_color: crate::bindings::core_u_object::FLinearColor,
    pub selected_material_color: crate::bindings::core_u_object::FLinearColor,
    pub selection_outline_color: crate::bindings::core_u_object::FLinearColor,
    pub subdued_selection_outline_color: crate::bindings::core_u_object::FLinearColor,
    pub selected_material_color_override: crate::bindings::core_u_object::FLinearColor,
    pub b_is_overriding_selected_color: bool,
    pub flags_5024: u8,
    pub b_enable_visual_log_recording_on_start: u32,
    pub screen_saver_inhibitor_semaphore: i32,
    pub flags_5036: u8,
    pub particle_event_manager_class_path: FString,
    pub selection_highlight_intensity: f32,
    pub bsp_selection_highlight_intensity: f32,
    pub selection_highlight_intensity_billboards: f32,
    pub global_net_travel_count: u32,
    pub net_driver_definitions: TArray<FNetDriverDefinition>,
    pub iris_net_driver_configs: TArray<FIrisNetDriverConfig>,
    pub server_actors: TArray<FString>,
    pub runtime_server_actors: TArray<FString>,
    pub net_error_log_interval: f32,
    pub flags_6484: u8,
    pub next_world_context_handle: i32,
}
pub struct UActorElementAssetDataInterface {}
pub struct UActorElementSelectionInterface {}
pub struct UActorElementWorldInterface {}
pub struct UComponentElementSelectionInterface {}
pub struct UComponentElementWorldInterface {}
pub struct UObjectElementSelectionInterface {}
pub struct USMInstanceProxyEditingObject {}
pub struct USMInstanceElementSelectionInterface {}
pub struct USMInstanceElementWorldInterface {}
pub struct UInstancedPlacemenClientSettings {
    pub update_guid: crate::bindings::core_u_object::FGuid,
    pub object_path: crate::bindings::core_u_object::FSoftObjectPath,
    pub instanced_component_settings: FISMComponentDescriptor,
}
pub struct UMaterialInterface {
    pub editor_only_data: UPtr<UMaterialInterfaceEditorOnlyData>,
    pub subsurface_profile: UPtr<USubsurfaceProfile>,
    pub subsurface_profiles: TArray<UPtr<USubsurfaceProfile>>,
    pub specular_profiles: TArray<UPtr<USpecularProfile>>,
    pub neural_profile: UPtr<UNeuralProfile>,
    pub lightmass_settings: FLightmassMaterialInterfaceSettings,
    pub b_texture_streaming_data_sorted: bool,
    pub texture_streaming_data_version: i32,
    pub texture_streaming_data: TArray<FMaterialTextureInfo>,
    pub asset_user_data: TArray<UPtr<UAssetUserData>>,
    pub cached_textures_sampling_info: TOptional<FMaterialCachedTexturesSamplingInfo>,
    pub flags_244: u8,
    pub texture_streaming_data_missing_entries: TArray<FMaterialTextureInfo>,
    pub preview_mesh: crate::bindings::core_u_object::FSoftObjectPath,
    pub thumbnail_info: UPtr<UThumbnailInfo>,
    pub layer_parameter_expansion: TMap<FString, bool>,
    pub parameter_overview_expansion: TMap<FString, bool>,
    pub asset_import_data: UPtr<UAssetImportData>,
    pub lighting_guid: crate::bindings::core_u_object::FGuid,
}
pub struct UMaterial {
    pub phys_material: UPtr<crate::bindings::physics_core::UPhysicalMaterial>,
    pub phys_material_mask: UPtr<UPhysicalMaterialMask>,
    pub physical_material_map: UPtr<crate::bindings::physics_core::UPhysicalMaterial>,
    pub render_trace_physical_material_outputs: TArray<
        UPtr<crate::bindings::physics_core::UPhysicalMaterial>,
    >,
    pub material_domain: EMaterialDomain,
    pub blend_mode: EBlendMode,
    pub decal_blend_mode: EDecalBlendMode,
    pub material_decal_response: EMaterialDecalResponse,
    pub nanite_override_material: FMaterialOverrideNanite,
    pub displacement_scaling: FDisplacementScaling,
    pub displacement_fade_range: FDisplacementFadeRange,
    pub shading_model: EMaterialShadingModel,
    pub flags_1033: u8,
    pub shading_models: FMaterialShadingModelField,
    pub used_shading_models: FString,
    pub opacity_mask_clip_value: f32,
    pub flags_1060: u8,
    pub flags_1061: u8,
    pub translucency_pass: EMaterialTranslucencyPass,
    pub translucency_lighting_mode: ETranslucencyLightingMode,
    pub flags_1064: u8,
    pub num_customized_u_vs: i32,
    pub translucency_directional_lighting_intensity: f32,
    pub translucent_shadow_density_scale: f32,
    pub translucent_self_shadow_density_scale: f32,
    pub translucent_self_shadow_second_density_scale: f32,
    pub translucent_self_shadow_second_opacity: f32,
    pub translucent_backscattering_exponent: f32,
    pub translucent_multiple_scattering_extinction: crate::bindings::core_u_object::FLinearColor,
    pub translucent_shadow_start_offset: f32,
    pub flags_1116: u8,
    pub flags_1117: u8,
    pub flags_1118: u8,
    pub flags_1120: u8,
    pub flags_1124: u8,
    pub flags_1128: u8,
    pub flags_1132: u8,
    pub float_precision_mode: EMaterialFloatPrecisionMode,
    pub flags_1134: u8,
    pub flags_1136: u8,
    pub flags_1140: u8,
    pub flags_1141: u8,
    pub translucent_local_light_shadow_quality: f32,
    pub translucent_directional_light_shadow_quality: f32,
    pub shading_rate: EMaterialShadingRate,
    pub flags_1153: u8,
    pub editor_x: i32,
    pub editor_y: i32,
    pub editor_pitch: i32,
    pub editor_yaw: i32,
    pub flags_1172: u8,
    pub flags_1173: u8,
    pub flags_1174: u8,
    pub blendable_location: EBlendableLocation,
    pub flags_1176: u8,
    pub user_scene_texture: FName,
    pub user_texture_divisor: crate::bindings::core_u_object::FIntPoint,
    pub resolution_relative_to_input: FName,
    pub flags_1212: u8,
    pub stencil_compare: EMaterialStencilCompare,
    pub stencil_ref_value: u8,
    pub neural_profile_id: i8,
    pub refraction_mode_deprecated: ERefractionMode,
    pub refraction_method: ERefractionMode,
    pub refraction_coverage_mode: ERefractionCoverageMode,
    pub pixel_depth_offset_mode: EPixelDepthOffsetMode,
    pub blendable_priority: i32,
    pub flags_1224: u8,
    pub preshader_gap: u16,
    pub usage_flag_warnings: u32,
    pub refraction_depth_bias: f32,
    pub max_world_position_offset_displacement: f32,
    pub b_always_evaluate_world_position_offset: bool,
    pub state_id: crate::bindings::core_u_object::FGuid,
    pub b_saved_cached_expression_data_deprecated: bool,
    pub referenced_texture_guids: TArray<crate::bindings::core_u_object::FGuid>,
    pub editor_comments_deprecated: TArray<UPtr<UMaterialExpressionComment>>,
    pub expressions_deprecated: TArray<UPtr<UMaterialExpression>>,
    pub parameter_group_data_deprecated: TArray<FParameterGroupData>,
    pub diffuse_color_deprecated: FColorMaterialInput,
    pub specular_color_deprecated: FColorMaterialInput,
    pub base_color_deprecated: FColorMaterialInput,
    pub metallic_deprecated: FScalarMaterialInput,
    pub specular_deprecated: FScalarMaterialInput,
    pub roughness_deprecated: FScalarMaterialInput,
    pub anisotropy_deprecated: FScalarMaterialInput,
    pub normal_deprecated: FVectorMaterialInput,
    pub tangent_deprecated: FVectorMaterialInput,
    pub emissive_color_deprecated: FColorMaterialInput,
    pub opacity_deprecated: FScalarMaterialInput,
    pub opacity_mask_deprecated: FScalarMaterialInput,
    pub world_position_offset_deprecated: FVectorMaterialInput,
    pub subsurface_color_deprecated: FColorMaterialInput,
    pub clear_coat_deprecated: FScalarMaterialInput,
    pub clear_coat_roughness_deprecated: FScalarMaterialInput,
    pub ambient_occlusion_deprecated: FScalarMaterialInput,
    pub refraction_deprecated: FScalarMaterialInput,
    pub customized_u_vs_deprecated: FVector2MaterialInput,
    pub material_attributes_deprecated: FMaterialAttributesInput,
    pub pixel_depth_offset_deprecated: FScalarMaterialInput,
    pub shading_model_from_material_expression_deprecated: FShadingModelMaterialInput,
    pub front_material_deprecated: FSubstrateMaterialInput,
}
pub struct ASkeletalMeshActor {
    pub flags_1136: u8,
    pub skeletal_mesh_component: UPtr<USkeletalMeshComponent>,
    pub replicated_mesh: UPtr<USkeletalMesh>,
    pub replicated_phys_asset: UPtr<UPhysicsAsset>,
    pub replicated_material0: UPtr<UMaterialInterface>,
    pub replicated_material1: UPtr<UMaterialInterface>,
}
pub struct UThumbnailInfo {}
pub struct UInstancedStaticMeshComponent {
    pub per_instance_sm_data: TArray<FInstancedStaticMeshInstanceData>,
    pub per_instance_prev_transform: TArray<crate::bindings::core_u_object::FMatrix>,
    pub previous_component_transform: crate::bindings::core_u_object::FTransform,
    pub num_custom_data_floats: i32,
    pub instancing_random_seed: i32,
    pub per_instance_sm_custom_data: TArray<f32>,
    pub additional_random_seeds: TArray<FInstancedStaticMeshRandomSeed>,
    pub instance_lod_distance_scale: f32,
    pub instance_min_draw_distance: i32,
    pub instance_start_cull_distance: i32,
    pub instance_end_cull_distance: i32,
    pub flags_2088: u8,
    pub instance_reorder_table: TArray<i32>,
    pub num_pending_lightmaps: i32,
    pub cached_mappings: TArray<FInstancedStaticMeshMappingInfo>,
    pub cached_bounds: FBoundsCacheElement,
}
pub struct UHierarchicalInstancedStaticMeshComponent {
    pub flags_2880: u8,
    pub translated_instance_space_origin: crate::bindings::core_u_object::FVector,
    pub sorted_instances: TArray<i32>,
    pub num_built_instances: i32,
    pub built_instance_bounds: crate::bindings::core_u_object::FBox,
    pub unbuilt_instance_bounds: crate::bindings::core_u_object::FBox,
    pub unbuilt_instance_bounds_list: TArray<crate::bindings::core_u_object::FBox>,
    pub flags_3064: u8,
    pub occlusion_layer_num_nodes: i32,
    pub cache_mesh_extended_bounds: crate::bindings::core_u_object::FBoxSphereBounds,
    pub instance_count_to_render: i32,
}
pub struct APartitionActor {
    pub grid_size: u32,
}
pub struct AISMPartitionActor {
    pub clients: TArray<crate::bindings::core_u_object::FGuid>,
    pub descriptors: TArray<FISMComponentDescriptor>,
    pub descriptor_components: TArray<FISMComponentData>,
    pub client_instance_managers: TMap<
        crate::bindings::core_u_object::FGuid,
        FISMClientInstanceManagerData,
    >,
}
pub struct AStaticMeshActor {
    pub static_mesh_component: UPtr<UStaticMeshComponent>,
    pub b_static_mesh_replicate_movement: bool,
    pub static_mesh_physics_replication_mode: EPhysicsReplicationMode,
    pub navigation_geometry_gathering_mode: ENavDataGatheringMode,
}
pub struct AInfo {
    pub sprite_component: UPtr<UBillboardComponent>,
}
pub struct UMaterialInstance {
    pub phys_material: UPtr<crate::bindings::physics_core::UPhysicalMaterial>,
    pub physical_material_map: UPtr<crate::bindings::physics_core::UPhysicalMaterial>,
    pub parent: UPtr<UMaterialInterface>,
    pub nanite_override_material: FMaterialOverrideNanite,
    pub specular_profile_override: UPtr<USpecularProfile>,
    pub flags_1000: u8,
    pub blendable_location_override: EBlendableLocation,
    pub blendable_priority_override: i32,
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
    pub user_scene_texture_overrides: TArray<FUserSceneTextureOverride>,
    pub b_override_base_properties_deprecated: bool,
    pub enumeration_objects: TArray<
        TSoftObjectPtr<crate::bindings::core_u_object::UObject>,
    >,
    pub base_property_overrides: FMaterialInstanceBasePropertyOverrides,
    pub static_parameters_runtime: FStaticParameterSetRuntimeData,
    pub referenced_texture_guids: TArray<crate::bindings::core_u_object::FGuid>,
    pub static_parameters_deprecated: FStaticParameterSet,
    pub b_saved_cached_data_deprecated: bool,
}
pub struct UMaterialInstanceConstant {
    pub parameter_state_id: crate::bindings::core_u_object::FGuid,
    pub phys_material_mask: UPtr<UPhysicalMaterialMask>,
}
pub struct UTexture2D {
    pub first_resource_mem_mip: i32,
    pub flags_1244: u8,
    pub address_x: TextureAddress,
    pub address_y: TextureAddress,
    pub imported_size: crate::bindings::core_u_object::FIntPoint,
    pub cpu_copy_texture: UPtr<UTexture2D>,
}
pub struct UHLODBuilder {
    pub hlod_builder_settings: UPtr<UHLODBuilderSettings>,
    pub hlod_instanced_static_mesh_component_class: TSubclassOf<
        UHLODInstancedStaticMeshComponent,
    >,
    pub hlod_instanced_skinned_mesh_component_class: TSubclassOf<
        UHLODInstancedSkinnedMeshComponent,
    >,
}
pub struct UTextureMipDataProviderFactory {}
pub struct UTextureAllMipDataProviderFactory {}
pub struct UActiveSoundUpdateInterface {}
pub struct IActiveSoundUpdateInterface {}
pub struct UChannel {
    pub connection: UPtr<UNetConnection>,
}
pub struct UActorChannel {
    pub actor: UPtr<AActor>,
    pub create_sub_objects: TArray<UPtr<crate::bindings::core_u_object::UObject>>,
}
pub struct UActorInstanceManagerInterface {}
pub struct IActorInstanceManagerInterface {}
pub struct UAnimationAssetExtensions {}
pub struct UAnimBlueprintClassSubsystem_PropertyAccess {}
pub struct UInterface_AnimCurveMetaData {}
pub struct IInterface_AnimCurveMetaData {}
pub struct UAnimCurveMetaData {
    pub curve_meta_data: TMap<FName, FCurveMetaData>,
}
pub struct UAnimationDataModelNotifiesExtensions {}
pub struct UAnimLayerInterface {}
pub struct IAnimLayerInterface {}
pub struct UAssetManagerSettings {
    pub primary_asset_types_to_scan: TArray<FPrimaryAssetTypeInfo>,
    pub directories_to_exclude: TArray<crate::bindings::core_u_object::FDirectoryPath>,
    pub primary_asset_rules: TArray<FPrimaryAssetRulesOverride>,
    pub custom_primary_asset_rules: TArray<FPrimaryAssetRulesCustomOverride>,
    pub b_only_cook_production_assets: bool,
    pub b_should_manager_determine_type_and_name: bool,
    pub b_should_guess_type_and_name_in_editor: bool,
    pub b_should_acquire_missing_chunks_on_load: bool,
    pub b_should_warn_about_invalid_assets: bool,
    pub primary_asset_id_redirects: TArray<FAssetManagerRedirect>,
    pub primary_asset_type_redirects: TArray<FAssetManagerRedirect>,
    pub asset_path_redirects: TArray<FAssetManagerRedirect>,
    pub meta_data_tags_for_asset_registry: TSet<FName>,
}
pub struct UAsyncPhysicsData {
    pub server_frame: i32,
    pub replication_redundancy: i32,
}
pub struct UAudioPanelWidgetInterface {}
pub struct IAudioPanelWidgetInterface {}
pub struct AAutoRTFMTestActor {
    pub my_property: i32,
}
pub struct AAutoRTFMTestAnotherActor {}
pub struct UBodySetup {
    pub agg_geom: FKAggregateGeom,
    pub flags_224: u8,
    pub flags_225: u8,
    pub phys_material: UPtr<crate::bindings::physics_core::UPhysicalMaterial>,
    pub walkable_slope_override: FWalkableSlopeOverride,
    pub build_scale_deprecated: f32,
    pub default_instance: FBodyInstance,
    pub build_scale3_d: crate::bindings::core_u_object::FVector,
}
pub struct UAutoRTFMTestBodySetup {}
pub struct UCameraShakeBase {
    pub b_single_instance: bool,
    pub shake_scale: f32,
    pub root_shake_pattern: UPtr<UCameraShakePattern>,
    pub camera_manager: UPtr<APlayerCameraManager>,
}
pub struct UAutoRTFMTestCameraShake {}
pub struct UChildActorComponent {
    pub child_actor_class: TSubclassOf<AActor>,
    pub child_actor: UPtr<AActor>,
    pub child_actor_template: UPtr<AActor>,
    pub editor_tree_view_visualization_mode: EChildActorComponentTreeViewVisualizationMode,
    pub flags_713: u8,
}
pub struct UAutoRTFMTestChildActorComponent {}
pub struct ULevel {
    pub b_use_external_actors: bool,
    pub owning_world: UPtr<UWorld>,
    pub model: UPtr<UModel>,
    pub model_components: TArray<UPtr<UModelComponent>>,
    pub actor_cluster: UPtr<ULevelActorContainer>,
    pub level_script_blueprint: UPtr<ULevelScriptBlueprint>,
    pub texture_streaming_resource_guids: TArray<crate::bindings::core_u_object::FGuid>,
    pub num_texture_streaming_unbuilt_components: i32,
    pub num_texture_streaming_dirty_resources: i32,
    pub level_script_actor: UPtr<ALevelScriptActor>,
    pub nav_list_start: UPtr<ANavigationObjectBase>,
    pub nav_list_end: UPtr<ANavigationObjectBase>,
    pub nav_data_chunks: TArray<UPtr<UNavigationDataChunk>>,
    pub lightmap_total_size: f32,
    pub shadowmap_total_size: f32,
    pub static_navigable_geometry: TArray<crate::bindings::core_u_object::FVector>,
    pub streaming_texture_guids: TArray<crate::bindings::core_u_object::FGuid>,
    pub streaming_textures: TArray<FName>,
    pub packed_texture_streaming_quality_level_feature_level: u32,
    pub level_build_data_id: crate::bindings::core_u_object::FGuid,
    pub map_build_data: UPtr<UMapBuildDataRegistry>,
    pub light_build_level_offset: crate::bindings::core_u_object::FIntVector,
    pub flags_632: u8,
    pub flags_635: u8,
    pub level_simplification: FLevelSimplificationDetails,
    pub level_color: crate::bindings::core_u_object::FLinearColor,
    pub b_prompt_when_adding_to_level_before_checkout: bool,
    pub b_prompt_when_adding_to_level_outside_bounds: bool,
    pub actor_packaging_scheme: EActorPackagingScheme,
    pub world_settings: UPtr<AWorldSettings>,
    pub world_data_layers: UPtr<AWorldDataLayers>,
    pub world_partition_runtime_cell: TSoftObjectPtr<UWorldPartitionRuntimeCell>,
    pub asset_user_data: TArray<UPtr<UAssetUserData>>,
    pub destroyed_replicated_static_actors: TArray<
        FReplicatedStaticActorDestructionInfo,
    >,
    pub b_use_actor_folders: bool,
    pub actor_folders: TMap<crate::bindings::core_u_object::FGuid, UPtr<UActorFolder>>,
    pub folder_label_to_actor_folders: TMap<FString, FActorFolderSet>,
    pub loaded_external_actor_folders: TArray<UPtr<UActorFolder>>,
    pub editor_path_owner: TWeakObjectPtr<crate::bindings::core_u_object::UObject>,
}
pub struct UAutoRTFMTestLevel {}
pub struct UAutoRTFMTestObject {}
pub struct UAutoRTFMTestPrimitiveComponent {}
pub struct UBlendableInterface {}
pub struct IBlendableInterface {}
pub struct UBlueprintSpringMathLibrary {}
pub struct UBookmarkBase {}
pub struct UBookMark2D {
    pub zoom2_d: f32,
    pub location: crate::bindings::core_u_object::FIntPoint,
}
pub struct AReflectionCapture {
    pub capture_component: UPtr<UReflectionCaptureComponent>,
    pub sprite_component: UPtr<UBillboardComponent>,
    pub capture_offset_component: UPtr<UBillboardComponent>,
}
pub struct ABoxReflectionCapture {}
pub struct UReflectionCaptureComponent {
    pub capture_offset_component: UPtr<UBillboardComponent>,
    pub reflection_source_type: EReflectionSourceType,
    pub cubemap: UPtr<UTextureCube>,
    pub source_cubemap_angle: f32,
    pub brightness: f32,
    pub capture_offset: crate::bindings::core_u_object::FVector,
    pub map_build_data_id: crate::bindings::core_u_object::FGuid,
}
pub struct UBoxReflectionCaptureComponent {
    pub box_transition_distance: f32,
    pub preview_influence_box: UPtr<UBoxComponent>,
    pub preview_capture_box: UPtr<UBoxComponent>,
}
pub struct UBuiltInAttributesExtensions {}
pub struct UPlayer {
    pub player_controller: UPtr<APlayerController>,
    pub current_net_speed: i32,
    pub configured_internet_speed: i32,
    pub configured_lan_speed: i32,
}
pub struct UNetConnection {
    pub children: TArray<UPtr<UChildConnection>>,
    pub driver: UPtr<UNetDriver>,
    pub package_map_class: TSubclassOf<crate::bindings::core_u_object::UPackageMap>,
    pub package_map: UPtr<crate::bindings::core_u_object::UPackageMap>,
    pub open_channels: TArray<UPtr<UChannel>>,
    pub sent_temporaries: TArray<UPtr<AActor>>,
    pub view_target: UPtr<AActor>,
    pub owning_actor: UPtr<AActor>,
    pub max_packet: i32,
    pub flags_172: u8,
    pub children_lookup: TMap<i32, UPtr<UChildConnection>>,
    pub player_id: FUniqueNetIdRepl,
    pub last_receive_time: f64,
    pub default_max_channel_size: i32,
    pub channels_to_tick: TArray<UPtr<UChannel>>,
}
pub struct UChildConnection {
    pub parent: UPtr<UNetConnection>,
}
pub struct UPlatformInterfaceBase {
    pub all_delegates: TArray<FDelegateArray>,
}
pub struct UCloudStorageBase {
    pub local_cloud_files: TArray<FString>,
    pub flags_80: u8,
}
pub struct UControlChannel {}
pub struct UDemoNetConnection {}
pub struct UPendingNetGame {
    pub net_driver: UPtr<UNetDriver>,
    pub demo_net_driver: UPtr<UDemoNetDriver>,
}
pub struct UDemoPendingNetGame {}
pub struct UDeviceProfileFragment {
    pub c_vars: TArray<FString>,
}
pub struct UDialogueSoundWaveProxy {}
pub struct ALight {
    pub light_component: UPtr<ULightComponent>,
    pub flags_1144: u8,
}
pub struct ADirectionalLight {
    pub arrow_component: UPtr<UArrowComponent>,
    pub directional_light_component: UPtr<UDirectionalLightComponent>,
}
pub struct UDistribution {}
pub struct UDistributionFloat {
    pub flags_56: u8,
}
pub struct UDistributionFloatConstant {
    pub constant: f32,
}
pub struct UDistributionFloatConstantCurve {
    pub constant_curve: crate::bindings::core_u_object::FInterpCurveFloat,
}
pub struct UDistributionFloatParameterBase {
    pub parameter_name: FName,
    pub min_input: f32,
    pub max_input: f32,
    pub min_output: f32,
    pub max_output: f32,
    pub param_mode: DistributionParamMode,
}
pub struct UDistributionFloatParticleParameter {}
pub struct UDistributionFloatUniform {
    pub min: f32,
    pub max: f32,
}
pub struct UDistributionFloatUniformCurve {
    pub constant_curve: crate::bindings::core_u_object::FInterpCurveVector2D,
}
pub struct UDistributionVector {
    pub flags_56: u8,
}
pub struct UDistributionVectorConstant {
    pub constant: crate::bindings::core_u_object::FVector,
    pub flags_88: u8,
    pub locked_axes: EDistributionVectorLockFlags,
}
pub struct UDistributionVectorConstantCurve {
    pub constant_curve: crate::bindings::core_u_object::FInterpCurveVector,
    pub flags_88: u8,
    pub locked_axes: EDistributionVectorLockFlags,
}
pub struct UDistributionVectorParameterBase {
    pub parameter_name: FName,
    pub min_input: crate::bindings::core_u_object::FVector,
    pub max_input: crate::bindings::core_u_object::FVector,
    pub min_output: crate::bindings::core_u_object::FVector,
    pub max_output: crate::bindings::core_u_object::FVector,
    pub param_modes: DistributionParamMode,
}
pub struct UDistributionVectorParticleParameter {}
pub struct UDistributionVectorUniform {
    pub max: crate::bindings::core_u_object::FVector,
    pub min: crate::bindings::core_u_object::FVector,
    pub flags_112: u8,
    pub locked_axes: EDistributionVectorLockFlags,
    pub mirror_flags: EDistributionVectorMirrorFlags,
    pub flags_120: u8,
}
pub struct UDistributionVectorUniformCurve {
    pub constant_curve: crate::bindings::core_u_object::FInterpCurveTwoVectors,
    pub flags_88: u8,
    pub locked_axes: EDistributionVectorLockFlags,
    pub mirror_flags: EDistributionVectorMirrorFlags,
    pub flags_100: u8,
}
pub struct AEmitter {
    pub particle_system_component: UPtr<UParticleSystemComponent>,
    pub flags_1144: u8,
    pub on_particle_spawn: FEmitter_OnParticleSpawn,
    pub on_particle_burst: FEmitter_OnParticleBurst,
    pub on_particle_death: FEmitter_OnParticleDeath,
    pub on_particle_collide: FEmitter_OnParticleCollide,
    pub sprite_component: UPtr<UBillboardComponent>,
    pub arrow_component: UPtr<UArrowComponent>,
}
pub struct AEmitterCameraLensEffectBase {
    pub ps_camera_effect: UPtr<UParticleSystem>,
    pub base_camera: UPtr<APlayerCameraManager>,
    pub relative_transform: crate::bindings::core_u_object::FTransform,
    pub base_fov: f32,
    pub flags_1396: u8,
    pub emitters_to_treat_as_same: TArray<TSubclassOf<AActor>>,
    pub dist_from_camera_deprecated: f32,
}
pub struct UViewModeUtils {}
pub struct UEngineBaseTypes {}
pub struct AExponentialHeightFog {
    pub component: UPtr<UExponentialHeightFogComponent>,
    pub flags_1152: u8,
}
pub struct UFontImportOptions {
    pub data: FFontImportOptionsData,
}
pub struct UForceFeedbackAttenuation {
    pub attenuation: FForceFeedbackAttenuationSettings,
}
pub struct ASpotLight {
    pub spot_light_component: UPtr<USpotLightComponent>,
    pub arrow_component: UPtr<UArrowComponent>,
}
pub struct AGeneratedMeshAreaLight {}
pub struct UHapticFeedbackEffect_Base {}
pub struct UHapticFeedbackEffect_Buffer {
    pub amplitudes: TArray<u8>,
    pub sample_rate: i32,
}
pub struct UHapticFeedbackEffect_Curve {
    pub haptic_details: FHapticFeedbackDetails_Curve,
}
pub struct UHapticFeedbackEffect_SoundWave {
    pub sound_wave: UPtr<USoundWave>,
    pub b_use_stereo: bool,
}
pub struct UAssetRegistryTagProviderInterface {}
pub struct IAssetRegistryTagProviderInterface {}
pub struct UInGameAdManager {
    pub flags_64: u8,
    pub clicked_banner_delegates: TArray<FInGameAdManager_ClickedBannerDelegates>,
    pub closed_ad_delegates: TArray<FInGameAdManager_ClosedAdDelegates>,
}
pub struct UInterface_ActorSubobject {}
pub struct IInterface_ActorSubobject {}
pub struct UInterface_AssetUserData {}
pub struct IInterface_AssetUserData {}
pub struct UInterface_AsyncCompilation {}
pub struct IInterface_AsyncCompilation {}
pub struct UBoneReferenceSkeletonProvider {}
pub struct IBoneReferenceSkeletonProvider {}
pub struct UInterface_CollisionDataProvider {}
pub struct IInterface_CollisionDataProvider {}
pub struct UInterface_PostProcessVolume {}
pub struct IInterface_PostProcessVolume {}
pub struct UInterface_PreviewMeshProvider {}
pub struct IInterface_PreviewMeshProvider {}
pub struct UPhysicsComponent {}
pub struct IPhysicsComponent {}
pub struct UISMPartitionInstanceManager {}
pub struct IISMPartitionInstanceManager {}
pub struct UISMPartitionInstanceManagerProvider {}
pub struct IISMPartitionInstanceManagerProvider {}
pub struct UViewportSelectableObject {}
pub struct IViewportSelectableObject {}
pub struct UWorldPartitionObjectResolver {}
pub struct IWorldPartitionObjectResolver {}
pub struct ULevelInstanceEditorPivotInterface {}
pub struct ILevelInstanceEditorPivotInterface {}
pub struct ULevelStreaming {
    pub package_name_deprecated: FName,
    pub world_asset: TSoftObjectPtr<UWorld>,
    pub streaming_priority: i32,
    pub package_name_to_load: FName,
    pub lod_package_names: TArray<FName>,
    pub level_transform: crate::bindings::core_u_object::FTransform,
    pub b_client_only_visible: bool,
    pub editor_path_owner: TWeakObjectPtr<crate::bindings::core_u_object::UObject>,
    pub level_lod_index: i32,
    pub flags_288: u8,
    pub flags_346: u8,
    pub draw_color_deprecated: crate::bindings::core_u_object::FColor,
    pub level_color: crate::bindings::core_u_object::FLinearColor,
    pub editor_streaming_volumes: TArray<UPtr<ALevelStreamingVolume>>,
    pub min_time_between_volume_unload_requests: f32,
    pub keywords: TArray<FString>,
    pub on_level_loaded: FLevelStreaming_OnLevelLoaded,
    pub on_level_unloaded: FLevelStreaming_OnLevelUnloaded,
    pub on_level_shown: FLevelStreaming_OnLevelShown,
    pub on_level_hidden: FLevelStreaming_OnLevelHidden,
    pub loaded_level: UPtr<ULevel>,
    pub pending_unload_level: UPtr<ULevel>,
    pub folder_path: FName,
}
pub struct ULevelStreamingAlwaysLoaded {}
pub struct ULevelStreamingDynamic {
    pub flags_584: u8,
}
pub struct ULevelStreamingPersistent {}
pub struct ULightComponentBase {
    pub original_light_guid: crate::bindings::core_u_object::FGuid,
    pub light_guid: crate::bindings::core_u_object::FGuid,
    pub brightness_deprecated: f32,
    pub intensity: f32,
    pub light_color: crate::bindings::core_u_object::FColor,
    pub flags_700: u8,
    pub flags_701: u8,
    pub cast_raytraced_shadow: ECastRayTracedShadow,
    pub flags_708: u8,
    pub deep_shadow_layer_distribution: f32,
    pub indirect_lighting_intensity: f32,
    pub volumetric_scattering_intensity: f32,
    pub samples_per_pixel: i32,
    pub static_editor_texture: UPtr<UTexture2D>,
    pub static_editor_texture_scale: f32,
    pub dynamic_editor_texture: UPtr<UTexture2D>,
    pub dynamic_editor_texture_scale: f32,
}
pub struct ULightMapTexture2D {}
pub struct ALightmassPortal {
    pub portal_component: UPtr<ULightmassPortalComponent>,
    pub sprite_component: UPtr<UBillboardComponent>,
}
pub struct ULightmassPortalComponent {
    pub preview_box: UPtr<UBoxComponent>,
}
pub struct ALocalFogVolume {
    pub local_fog_volume_volume: UPtr<ULocalFogVolumeComponent>,
}
pub struct UMapBuildDataRegistry {
    pub level_lighting_quality: ELightingBuildQuality,
}
pub struct UMaterialCacheStackProvider {}
pub struct UMaterialExpressionAbs {
    pub input: FExpressionInput,
}
pub struct UMaterialExpressionAbsorptionMediumMaterialOutput {
    pub transmittance_color: FExpressionInput,
}
pub struct UMaterialExpressionActorPositionWS {
    pub origin_type: EPositionOrigin,
}
pub struct UMaterialExpressionAdd {
    pub a: FExpressionInput,
    pub b: FExpressionInput,
    pub const_a: f32,
    pub const_b: f32,
}
pub struct UMaterialExpressionAggregate {
    pub kind: EMaterialExpressionMakeAggregateKind,
    pub user_aggregate: UPtr<UMaterialAggregate>,
    pub attribute_names: TArray<FName>,
    pub prototype_input: FExpressionInput,
    pub entries: TArray<FMaterialExpressionAggregateEntry>,
}
pub struct UMaterialExpressionTextureBase {
    pub texture: UPtr<UTexture>,
    pub sampler_type: EMaterialSamplerType,
    pub flags_209: u8,
}
pub struct UMaterialExpressionTextureSample {
    pub coordinates: FExpressionInput,
    pub texture_object: FExpressionInput,
    pub mip_value: FExpressionInput,
    pub coordinates_dx: FExpressionInput,
    pub coordinates_dy: FExpressionInput,
    pub automatic_view_mip_bias_value: FExpressionInput,
    pub mip_value_mode: ETextureMipValueMode,
    pub sampler_source: ESamplerSourceMode,
    pub gather_mode: ETextureGatherMode,
    pub flags_507: u8,
    pub channel_names: FParameterChannelNames,
    pub const_coordinate: u8,
    pub const_mip_value: i32,
}
pub struct UMaterialExpressionTextureSampleParameter {
    pub parameter_name: FName,
    pub expression_guid: crate::bindings::core_u_object::FGuid,
    pub group: FName,
    pub sort_priority: i32,
}
pub struct UMaterialExpressionTextureSampleParameter2D {}
pub struct UMaterialExpressionAntialiasedTextureMask {
    pub threshold: f32,
    pub channel: ETextureColorChannel,
}
pub struct UMaterialExpressionAppendVector {
    pub a: FExpressionInput,
    pub b: FExpressionInput,
}
pub struct UMaterialExpressionArccosine {
    pub input: FExpressionInput,
}
pub struct UMaterialExpressionArccosineFast {
    pub input: FExpressionInput,
}
pub struct UMaterialExpressionArcsine {
    pub input: FExpressionInput,
}
pub struct UMaterialExpressionArcsineFast {
    pub input: FExpressionInput,
}
pub struct UMaterialExpressionArctangent {
    pub input: FExpressionInput,
}
pub struct UMaterialExpressionArctangent2 {
    pub y: FExpressionInput,
    pub x: FExpressionInput,
}
pub struct UMaterialExpressionArctangent2Fast {
    pub y: FExpressionInput,
    pub x: FExpressionInput,
}
pub struct UMaterialExpressionArctangentFast {
    pub input: FExpressionInput,
}
pub struct UMaterialExpressionExternalCodeBase {
    pub external_code_identifiers: TArray<FName>,
}
pub struct UMaterialExpressionAtmosphericFogColor {
    pub world_position: FExpressionInput,
    pub world_position_origin_type: EPositionOrigin,
}
pub struct UMaterialExpressionAtmosphericLightColor {}
pub struct UMaterialExpressionAtmosphericLightVector {}
pub struct UMaterialExpressionBindlessSwitch {
    pub default: FExpressionInput,
    pub bindless: FExpressionInput,
}
pub struct UMaterialExpressionBlackBody {
    pub temp: FExpressionInput,
}
pub struct UMaterialExpressionBlend {
    pub a: FExpressionInput,
    pub b: FExpressionInput,
    pub alpha: FExpressionInput,
    pub pixel_attributes_blend_mode: EMaterialExpressionBlendMode,
    pub vertex_attributes_blend_mode: EMaterialExpressionBlendMode,
}
pub struct UMaterialExpressionBlendMaterialAttributes {
    pub a: FMaterialAttributesInput,
    pub b: FMaterialAttributesInput,
    pub alpha: FExpressionInput,
    pub pixel_attribute_blend_type: EMaterialAttributeBlend,
    pub vertex_attribute_blend_type: EMaterialAttributeBlend,
}
pub struct UMaterialExpressionLegacyBlendMaterialAttributes {
    pub vertex_attribute_use_a: FExpressionInput,
    pub vertex_attribute_use_b: FExpressionInput,
    pub pixel_attribute_use_a: FExpressionInput,
    pub pixel_attribute_use_b: FExpressionInput,
    pub blend_function_type: EMaterialAttributeBlendFunction,
}
pub struct UMaterialExpressionBounds {
    pub ty: EMaterialExpressionBoundsType,
}
pub struct UMaterialExpressionBreakMaterialAttributes {
    pub material_attributes: FMaterialAttributesInput,
}
pub struct UMaterialExpressionBumpOffset {
    pub coordinate: FExpressionInput,
    pub height: FExpressionInput,
    pub height_ratio_input: FExpressionInput,
    pub height_ratio: f32,
    pub reference_plane: f32,
    pub const_coordinate: u32,
}
pub struct UMaterialExpressionCameraPositionWS {}
pub struct UMaterialExpressionCameraVectorWS {}
pub struct UMaterialExpressionCeil {
    pub input: FExpressionInput,
}
pub struct UMaterialExpressionParameter {
    pub parameter_name: FName,
    pub expression_guid: crate::bindings::core_u_object::FGuid,
    pub group: FName,
    pub sort_priority: i32,
}
pub struct UMaterialExpressionVectorParameter {
    pub default_value: crate::bindings::core_u_object::FLinearColor,
    pub b_use_custom_primitive_data: bool,
    pub primitive_data_index: u8,
    pub channel_names: FParameterChannelNames,
}
pub struct UMaterialExpressionChannelMaskParameter {
    pub mask_channel: EChannelMaskParameterColor,
    pub input: FExpressionInput,
}
pub struct UMaterialExpressionClamp {
    pub input: FExpressionInput,
    pub min: FExpressionInput,
    pub max: FExpressionInput,
    pub clamp_mode: EClampMode,
    pub min_default: f32,
    pub max_default: f32,
}
pub struct UMaterialExpressionCloudSampleAttribute {}
pub struct UMaterialExpressionCollectionParameter {
    pub collection: UPtr<UMaterialParameterCollection>,
    pub parameter_name: FName,
    pub expression_guid: crate::bindings::core_u_object::FGuid,
    pub group: FName,
    pub sort_priority: i32,
    pub parameter_id: crate::bindings::core_u_object::FGuid,
}
pub struct UMaterialExpressionCollectionTransform {
    pub input: FExpressionInput,
    pub collection: UPtr<UMaterialParameterCollection>,
    pub parameter_name: FName,
    pub parameter_id: crate::bindings::core_u_object::FGuid,
    pub transform_type: EParameterCollectionTransformType,
}
pub struct UMaterialExpressionColorRamp {
    pub input: FExpressionInput,
    pub const_input: f32,
    pub color_curve: UPtr<UCurveLinearColor>,
}
pub struct UMaterialExpressionComment {
    pub size_x: i32,
    pub size_y: i32,
    pub text: FString,
    pub comment_color: crate::bindings::core_u_object::FLinearColor,
    pub font_size: i32,
    pub flags_244: u8,
    pub b_group_mode: bool,
}
pub struct UMaterialExpressionComponentMask {
    pub input: FExpressionInput,
    pub flags_248: u8,
}
pub struct UMaterialExpressionComposite {
    pub subgraph_name: FString,
    pub input_expressions: UPtr<UMaterialExpressionPinBase>,
    pub output_expressions: UPtr<UMaterialExpressionPinBase>,
}
pub struct UMaterialExpressionConstant {
    pub r: f32,
}
pub struct UMaterialExpressionConstant2Vector {
    pub r: f32,
    pub g: f32,
}
pub struct UMaterialExpressionConstant3Vector {
    pub constant: crate::bindings::core_u_object::FLinearColor,
}
pub struct UMaterialExpressionConstant4Vector {
    pub constant: crate::bindings::core_u_object::FLinearColor,
}
pub struct UMaterialExpressionConstantBiasScale {
    pub input: FExpressionInput,
    pub bias: f32,
    pub scale: f32,
}
pub struct UMaterialExpressionConvert {
    pub convert_inputs: TArray<FMaterialExpressionConvertInput>,
    pub convert_outputs: TArray<FMaterialExpressionConvertOutput>,
    pub convert_mappings: TArray<FMaterialExpressionConvertMapping>,
    pub node_name: FString,
}
pub struct UMaterialExpressionCosine {
    pub input: FExpressionInput,
    pub period: f32,
}
pub struct UMaterialExpressionCrossProduct {
    pub a: FExpressionInput,
    pub b: FExpressionInput,
}
pub struct UMaterialExpressionScalarParameter {
    pub default_value: f32,
    pub control_type: EMaterialScalarParameterControlType,
    pub slider_min: f32,
    pub slider_max: f32,
    pub enumeration: TSoftObjectPtr<crate::bindings::core_u_object::UObject>,
    pub enumeration_index: u8,
    pub b_use_custom_primitive_data: bool,
    pub primitive_data_index: u8,
}
pub struct UMaterialExpressionCurveAtlasRowParameter {
    pub curve: UPtr<UCurveLinearColor>,
    pub atlas: UPtr<UCurveLinearColorAtlas>,
    pub input_time: FExpressionInput,
}
pub struct UMaterialExpressionCustom {
    pub code: FString,
    pub output_type: ECustomMaterialOutputType,
    pub description: FString,
    pub inputs: TArray<FCustomInput>,
    pub additional_outputs: TArray<FCustomOutput>,
    pub additional_defines: TArray<FCustomDefine>,
    pub include_file_paths: TArray<FString>,
    pub show_code: bool,
}
pub struct UMaterialExpressionDataDrivenShaderPlatformInfoSwitch {
    pub input_true: FExpressionInput,
    pub input_false: FExpressionInput,
    pub ddspi_property_names: TArray<FDataDrivenShaderPlatformInfoInput>,
    pub flags_312: u8,
}
pub struct UMaterialExpressionDBufferTexture {
    pub coordinates: FExpressionInput,
    pub d_buffer_texture_id: EDBufferTextureId,
}
pub struct UMaterialExpressionDDX {
    pub value: FExpressionInput,
}
pub struct UMaterialExpressionDDY {
    pub value: FExpressionInput,
}
pub struct UMaterialExpressionDecalColor {}
pub struct UMaterialExpressionDecalDerivative {}
pub struct UMaterialExpressionDecalLifetimeOpacity {}
pub struct UMaterialExpressionDecalMipmapLevel {
    pub texture_size: FExpressionInput,
    pub const_width: f32,
    pub const_height: f32,
}
pub struct UMaterialExpressionDeltaTime {}
pub struct UMaterialExpressionDepthFade {
    pub in_opacity: FExpressionInput,
    pub fade_distance: FExpressionInput,
    pub opacity_default: f32,
    pub fade_distance_default: f32,
}
pub struct UMaterialExpressionDepthOfFieldFunction {
    pub function_value: EDepthOfFieldFunctionValue,
    pub depth: FExpressionInput,
}
pub struct UMaterialExpressionDeriveNormalZ {
    pub in_xy: FExpressionInput,
}
pub struct UMaterialExpressionDesaturation {
    pub input: FExpressionInput,
    pub fraction: FExpressionInput,
    pub luminance_factors: crate::bindings::core_u_object::FLinearColor,
}
pub struct UMaterialExpressionDistance {
    pub a: FExpressionInput,
    pub b: FExpressionInput,
}
pub struct UMaterialExpressionDistanceCullFade {}
pub struct UMaterialExpressionDistanceFieldApproxAO {
    pub position: FExpressionInput,
    pub world_position_origin_type: EPositionOrigin,
    pub normal: FExpressionInput,
    pub base_distance: FExpressionInput,
    pub base_distance_default: f32,
    pub radius: FExpressionInput,
    pub radius_default: f32,
    pub num_steps: u32,
    pub step_scale_default: f32,
}
pub struct UMaterialExpressionDistanceFieldGradient {
    pub position: FExpressionInput,
    pub world_position_origin_type: EPositionOrigin,
}
pub struct UMaterialExpressionDistanceFieldsRenderingSwitch {
    pub no: FExpressionInput,
    pub yes: FExpressionInput,
}
pub struct UMaterialExpressionDistanceToNearestSurface {
    pub position: FExpressionInput,
    pub world_position_origin_type: EPositionOrigin,
}
pub struct UMaterialExpressionDivide {
    pub a: FExpressionInput,
    pub b: FExpressionInput,
    pub const_a: f32,
    pub const_b: f32,
}
pub struct UMaterialExpressionDotProduct {
    pub a: FExpressionInput,
    pub b: FExpressionInput,
}
pub struct UMaterialExpressionDoubleVectorParameter {
    pub default_value: crate::bindings::core_u_object::FVector4d,
}
pub struct UMaterialExpressionDynamicParameter {
    pub param_names: TArray<FString>,
    pub default_value: crate::bindings::core_u_object::FLinearColor,
    pub parameter_index: u32,
}
pub struct UMaterialExpressionExponential {
    pub input: FExpressionInput,
}
pub struct UMaterialExpressionExponential2 {
    pub input: FExpressionInput,
}
pub struct UMaterialExpressionEyeAdaptation {}
pub struct UMaterialExpressionEyeAdaptationInverse {
    pub light_value_input: FExpressionInput,
    pub alpha_input: FExpressionInput,
}
pub struct UMaterialExpressionFeatureLevelSwitch {
    pub default: FExpressionInput,
    pub inputs: FExpressionInput,
}
pub struct UMaterialExpressionFloatToUInt {
    pub input: FExpressionInput,
    pub mode: EFloatToIntMode,
}
pub struct UMaterialExpressionUIntToFloat {
    pub input: FExpressionInput,
}
pub struct UMaterialExpressionFloor {
    pub input: FExpressionInput,
}
pub struct UMaterialExpressionFmod {
    pub a: FExpressionInput,
    pub b: FExpressionInput,
}
pub struct UMaterialExpressionFontSample {
    pub font: UPtr<UFont>,
    pub font_texture_page: i32,
}
pub struct UMaterialExpressionFontSampleParameter {
    pub parameter_name: FName,
    pub expression_guid: crate::bindings::core_u_object::FGuid,
    pub group: FName,
    pub sort_priority: i32,
}
pub struct UMaterialExpressionFontSignedDistance {}
pub struct UMaterialExpressionFrac {
    pub input: FExpressionInput,
}
pub struct UMaterialExpressionFresnel {
    pub exponent_in: FExpressionInput,
    pub exponent: f32,
    pub base_reflect_fraction_in: FExpressionInput,
    pub base_reflect_fraction: f32,
    pub normal: FExpressionInput,
}
pub struct UMaterialExpressionFunctionInput {
    pub preview: FExpressionInput,
    pub input_name: FName,
    pub description: FString,
    pub id: crate::bindings::core_u_object::FGuid,
    pub input_type: EFunctionInputType,
    pub preview_value: crate::bindings::core_u_object::FVector4f,
    pub flags_320: u8,
    pub sort_priority: i32,
    pub blend_input_relevance: EBlendInputRelevance,
    pub flags_332: u8,
}
pub struct UMaterialExpressionFunctionOutput {
    pub output_name: FName,
    pub description: FString,
    pub sort_priority: i32,
    pub a: FExpressionInput,
    pub flags_288: u8,
    pub id: crate::bindings::core_u_object::FGuid,
}
pub struct UMaterialExpressionGenericConstant {}
pub struct UMaterialExpressionConstantDouble {
    pub value: f64,
}
pub struct UMaterialExpressionGetMaterialAttributes {
    pub material_attributes: FMaterialAttributesInput,
    pub attribute_get_types: TArray<crate::bindings::core_u_object::FGuid>,
}
pub struct UMaterialExpressionGIReplace {
    pub default: FExpressionInput,
    pub static_indirect: FExpressionInput,
    pub dynamic_indirect: FExpressionInput,
}
pub struct UMaterialExpressionHairAttributes {
    pub flags_200: u8,
}
pub struct UMaterialExpressionHairColor {
    pub melanin: FExpressionInput,
    pub redness: FExpressionInput,
    pub dye_color: FExpressionInput,
}
pub struct UMaterialExpressionHsvToRgb {
    pub input: FExpressionInput,
}
pub struct UMaterialExpressionIf {
    pub a: FExpressionInput,
    pub b: FExpressionInput,
    pub a_greater_than_b: FExpressionInput,
    pub a_equals_b: FExpressionInput,
    pub a_less_than_b: FExpressionInput,
    pub equals_threshold: f32,
    pub const_b: f32,
    pub const_a_equals_b_deprecated: f32,
}
pub struct UMaterialExpressionIfThenElse {
    pub condition: FExpressionInput,
    pub true_: FExpressionInput,
    pub false_: FExpressionInput,
}
pub struct UMaterialExpressionInverseLinearInterpolate {
    pub a: FExpressionInput,
    pub b: FExpressionInput,
    pub value: FExpressionInput,
    pub const_a: f32,
    pub const_b: f32,
    pub const_value: f32,
    pub b_clamp_result: bool,
}
pub struct UMaterialExpressionIsFirstPerson {}
pub struct UMaterialExpressionIsOrthographic {}
pub struct UMaterialExpressionLength {
    pub input: FExpressionInput,
}
pub struct UMaterialExpressionLightmapUVs {}
pub struct UMaterialExpressionLightmassReplace {
    pub realtime: FExpressionInput,
    pub lightmass: FExpressionInput,
}
pub struct UMaterialExpressionLightVector {}
pub struct UMaterialExpressionLinearInterpolate {
    pub a: FExpressionInput,
    pub b: FExpressionInput,
    pub alpha: FExpressionInput,
    pub const_a: f32,
    pub const_b: f32,
    pub const_alpha: f32,
}
pub struct UMaterialExpressionLocalPosition {
    pub included_offsets: EPositionIncludedOffsets,
    pub local_origin: ELocalPositionOrigin,
}
pub struct UMaterialExpressionLogarithm {
    pub input: FExpressionInput,
}
pub struct UMaterialExpressionLogarithm10 {
    pub x: FExpressionInput,
}
pub struct UMaterialExpressionLogarithm2 {
    pub x: FExpressionInput,
}
pub struct UMaterialExpressionMakeMaterialAttributes {
    pub base_color: FExpressionInput,
    pub metallic: FExpressionInput,
    pub specular: FExpressionInput,
    pub roughness: FExpressionInput,
    pub anisotropy: FExpressionInput,
    pub emissive_color: FExpressionInput,
    pub opacity: FExpressionInput,
    pub opacity_mask: FExpressionInput,
    pub normal: FExpressionInput,
    pub tangent: FExpressionInput,
    pub world_position_offset: FExpressionInput,
    pub subsurface_color: FExpressionInput,
    pub clear_coat: FExpressionInput,
    pub clear_coat_roughness: FExpressionInput,
    pub ambient_occlusion: FExpressionInput,
    pub refraction: FExpressionInput,
    pub customized_u_vs: FExpressionInput,
    pub pixel_depth_offset: FExpressionInput,
    pub shading_model: FExpressionInput,
    pub displacement: FExpressionInput,
}
pub struct UMaterialExpressionMapARPassthroughCameraUV {
    pub coordinates: FExpressionInput,
}
pub struct UMaterialExpressionMaterialAttributeLayers {
    pub input: FMaterialAttributesInput,
    pub default_layers: FMaterialLayersFunctions,
    pub layer_callers: TArray<UPtr<UMaterialExpressionMaterialFunctionCall>>,
    pub num_active_layer_callers: i32,
    pub blend_callers: TArray<UPtr<UMaterialExpressionMaterialFunctionCall>>,
    pub num_active_blend_callers: i32,
    pub output_caller: UPtr<UMaterialExpressionMaterialFunctionCall>,
    pub b_is_layer_graph_built: bool,
}
pub struct UMaterialExpressionMaterialFunctionCall {
    pub material_function: UPtr<UMaterialFunctionInterface>,
    pub function_inputs: TArray<FFunctionExpressionInput>,
    pub function_outputs: TArray<FFunctionExpressionOutput>,
    pub function_parameter_info: FMaterialParameterInfo,
}
pub struct UMaterialExpressionMaterialLayerOutput {}
pub struct UMaterialExpressionMaterialProxyReplace {
    pub realtime: FExpressionInput,
    pub material_proxy: FExpressionInput,
}
pub struct UMaterialExpressionMax {
    pub a: FExpressionInput,
    pub b: FExpressionInput,
    pub const_a: f32,
    pub const_b: f32,
}
pub struct UMaterialExpressionMeshPaintTextureCoordinateIndex {}
pub struct UMaterialExpressionMeshPaintTextureObject {}
pub struct UMaterialExpressionMeshPaintTextureReplace {
    pub default: FExpressionInput,
    pub mesh_paint_texture: FExpressionInput,
}
pub struct UMaterialExpressionMin {
    pub a: FExpressionInput,
    pub b: FExpressionInput,
    pub const_a: f32,
    pub const_b: f32,
}
pub struct UMaterialExpressionModulo {
    pub a: FExpressionInput,
    pub b: FExpressionInput,
}
pub struct UMaterialExpressionMultiply {
    pub a: FExpressionInput,
    pub b: FExpressionInput,
    pub const_a: f32,
    pub const_b: f32,
}
pub struct UMaterialExpressionRerouteBase {}
pub struct UMaterialExpressionNamedRerouteBase {}
pub struct UMaterialExpressionNamedRerouteDeclaration {
    pub input: FExpressionInput,
    pub name: FName,
    pub node_color: crate::bindings::core_u_object::FLinearColor,
    pub variable_guid: crate::bindings::core_u_object::FGuid,
}
pub struct UMaterialExpressionNamedRerouteUsage {
    pub declaration: UPtr<UMaterialExpressionNamedRerouteDeclaration>,
    pub declaration_guid: crate::bindings::core_u_object::FGuid,
}
pub struct UMaterialExpressionNaniteReplace {
    pub default: FExpressionInput,
    pub nanite: FExpressionInput,
}
pub struct UMaterialExpressionNeuralNetworkInput {
    pub coordinates: FExpressionInput,
    pub input0: FExpressionInput,
    pub mask: FExpressionInput,
    pub neural_index_type: ENeuralIndexType,
}
pub struct UMaterialExpressionNeuralNetworkOutput {
    pub coordinates: FExpressionInput,
    pub neural_index_type: ENeuralIndexType,
}
pub struct UMaterialExpressionNormalize {
    pub vector_input: FExpressionInput,
}
pub struct UMaterialExpressionObjectBounds {}
pub struct UMaterialExpressionObjectLocalBounds {}
pub struct UMaterialExpressionObjectOrientation {}
pub struct UMaterialExpressionObjectPositionWS {
    pub origin_type: EPositionOrigin,
}
pub struct UMaterialExpressionObjectRadius {}
pub struct UMaterialExpressionOneMinus {
    pub input: FExpressionInput,
}
pub struct UMaterialExpressionOperator {
    pub dynamic_inputs: TArray<FMaterialExpressionOperatorInput>,
    pub operator: EMaterialExpressionOperatorKind,
}
pub struct UMaterialExpressionPanner {
    pub coordinate: FExpressionInput,
    pub time: FExpressionInput,
    pub speed: FExpressionInput,
    pub speed_x: f32,
    pub speed_y: f32,
    pub const_coordinate: u32,
    pub b_fractional_part: bool,
}
pub struct UMaterialExpressionParticleColor {}
pub struct UMaterialExpressionParticleDirection {}
pub struct UMaterialExpressionParticleMacroUV {}
pub struct UMaterialExpressionParticleMotionBlurFade {}
pub struct UMaterialExpressionParticlePositionWS {
    pub origin_type: EPositionOrigin,
}
pub struct UMaterialExpressionParticleRadius {}
pub struct UMaterialExpressionParticleRandom {}
pub struct UMaterialExpressionParticleRelativeTime {}
pub struct UMaterialExpressionParticleSize {}
pub struct UMaterialExpressionParticleSpeed {}
pub struct UMaterialExpressionParticleSpriteRotation {}
pub struct UMaterialExpressionParticleSubUV {
    pub flags_584: u8,
}
pub struct UMaterialExpressionParticleSubUVProperties {}
pub struct UMaterialExpressionPathTracingBufferTexture {
    pub coordinates: FExpressionInput,
    pub path_tracing_buffer_texture_id: EPathTracingBufferTextureId,
}
pub struct UMaterialExpressionPathTracingQualitySwitch {
    pub normal: FExpressionInput,
    pub path_traced: FExpressionInput,
}
pub struct UMaterialExpressionPathTracingRayTypeSwitch {
    pub main: FExpressionInput,
    pub shadow: FExpressionInput,
    pub indirect_diffuse: FExpressionInput,
    pub indirect_specular: FExpressionInput,
    pub indirect_volume: FExpressionInput,
}
pub struct UMaterialExpressionPerInstanceCustomData {
    pub default_value: FExpressionInput,
    pub const_default_value: f32,
    pub data_index: u32,
}
pub struct UMaterialExpressionPerInstanceCustomData3Vector {
    pub default_value: FExpressionInput,
    pub const_default_value: crate::bindings::core_u_object::FLinearColor,
    pub data_index: u32,
}
pub struct UMaterialExpressionPerInstanceFadeAmount {}
pub struct UMaterialExpressionPerInstanceRandom {}
pub struct UMaterialExpressionPinBase {
    pub reroute_pins: TArray<FCompositeReroute>,
    pub pin_direction: EEdGraphPinDirection,
}
pub struct UMaterialExpressionPixelDepth {}
pub struct UMaterialExpressionPixelNormalWS {}
pub struct UMaterialExpressionPostVolumeUserFlagTest {
    pub bit_index: FExpressionInput,
    pub const_bit_index: i32,
}
pub struct UMaterialExpressionPower {
    pub base: FExpressionInput,
    pub exponent: FExpressionInput,
    pub const_exponent: f32,
}
pub struct UMaterialExpressionPrecomputedAOMask {}
pub struct UMaterialExpressionPreSkinnedLocalBounds {}
pub struct UMaterialExpressionPreSkinnedNormal {}
pub struct UMaterialExpressionPreSkinnedPosition {}
pub struct UMaterialExpressionPreviousFrameSwitch {
    pub current_frame: FExpressionInput,
    pub previous_frame: FExpressionInput,
}
pub struct UMaterialExpressionQualitySwitch {
    pub default: FExpressionInput,
    pub inputs: FExpressionInput,
}
pub struct UMaterialExpressionRayTracingQualitySwitch {
    pub normal: FExpressionInput,
    pub ray_traced: FExpressionInput,
}
pub struct UMaterialExpressionRecordTextureStreamingInfo {
    pub texture_object: FExpressionInput,
    pub coordinates: FExpressionInput,
}
pub struct UMaterialExpressionReflectionCapturePassSwitch {
    pub default: FExpressionInput,
    pub reflection: FExpressionInput,
}
pub struct UMaterialExpressionReflectionVectorWS {
    pub custom_world_normal: FExpressionInput,
    pub flags_248: u8,
}
pub struct UMaterialExpressionRequiredSamplersSwitch {
    pub input_true: FExpressionInput,
    pub input_false: FExpressionInput,
    pub required_samplers: u32,
}
pub struct UMaterialExpressionReroute {
    pub input: FExpressionInput,
}
pub struct UMaterialExpressionRgbToHsv {
    pub input: FExpressionInput,
}
pub struct UMaterialExpressionRotateAboutAxis {
    pub normalized_rotation_axis: FExpressionInput,
    pub rotation_angle: FExpressionInput,
    pub pivot_point: FExpressionInput,
    pub position: FExpressionInput,
    pub period: f32,
}
pub struct UMaterialExpressionRotator {
    pub coordinate: FExpressionInput,
    pub time: FExpressionInput,
    pub center_x: f32,
    pub center_y: f32,
    pub speed: f32,
    pub const_coordinate: u32,
}
pub struct UMaterialExpressionRound {
    pub input: FExpressionInput,
}
pub struct UMaterialExpressionRuntimeVirtualTextureCustomData {}
pub struct UMaterialExpressionRuntimeVirtualTextureOutput {
    pub base_color: FExpressionInput,
    pub specular: FExpressionInput,
    pub roughness: FExpressionInput,
    pub normal: FExpressionInput,
    pub world_height: FExpressionInput,
    pub opacity: FExpressionInput,
    pub mask: FExpressionInput,
    pub displacement: FExpressionInput,
    pub mask4: FExpressionInput,
}
pub struct UMaterialExpressionRuntimeVirtualTextureReplace {
    pub default: FExpressionInput,
    pub virtual_texture_output: FExpressionInput,
}
pub struct UMaterialExpressionRuntimeVirtualTextureSample {
    pub coordinates: FExpressionInput,
    pub world_position: FExpressionInput,
    pub mip_value: FExpressionInput,
    pub ddx: FExpressionInput,
    pub ddy: FExpressionInput,
    pub virtual_texture: UPtr<URuntimeVirtualTexture>,
    pub material_type: ERuntimeVirtualTextureMaterialType,
    pub b_single_physical_space: bool,
    pub b_adaptive: bool,
    pub world_position_origin_type: EPositionOrigin,
    pub texture_address_mode: ERuntimeVirtualTextureTextureAddressMode,
    pub mip_value_mode: ERuntimeVirtualTextureMipValueMode,
    pub b_enable_feedback: bool,
}
pub struct UMaterialExpressionRuntimeVirtualTextureSampleParameter {
    pub parameter_name: FName,
    pub expression_guid: crate::bindings::core_u_object::FGuid,
    pub group: FName,
    pub sort_priority: i32,
}
pub struct UMaterialExpressionSamplePhysicsVectorField {
    pub world_position: FExpressionInput,
    pub world_position_origin_type: EPositionOrigin,
    pub field_target: crate::bindings::chaos::EFieldVectorType,
}
pub struct UMaterialExpressionSamplePhysicsScalarField {
    pub world_position: FExpressionInput,
    pub world_position_origin_type: EPositionOrigin,
    pub field_target: crate::bindings::chaos::EFieldScalarType,
}
pub struct UMaterialExpressionSamplePhysicsIntegerField {
    pub world_position: FExpressionInput,
    pub world_position_origin_type: EPositionOrigin,
    pub field_target: crate::bindings::chaos::EFieldIntegerType,
}
pub struct UMaterialExpressionSaturate {
    pub input: FExpressionInput,
}
pub struct UMaterialExpressionSceneColor {
    pub input_mode: EMaterialSceneAttributeInputMode,
    pub input: FExpressionInput,
    pub offset_fraction_deprecated: FExpressionInput,
    pub const_input: crate::bindings::core_u_object::FVector2D,
}
pub struct UMaterialExpressionSceneDepth {
    pub input_mode: EMaterialSceneAttributeInputMode,
    pub input: FExpressionInput,
    pub coordinates_deprecated: FExpressionInput,
    pub const_input: crate::bindings::core_u_object::FVector2D,
}
pub struct UMaterialExpressionSceneDepthWithoutWater {
    pub input_mode: EMaterialSceneAttributeInputMode,
    pub input: FExpressionInput,
    pub const_input: crate::bindings::core_u_object::FVector2D,
    pub fallback_depth: f32,
}
pub struct UMaterialExpressionSceneTexelSize {}
pub struct UMaterialExpressionSceneTexture {
    pub coordinates: FExpressionInput,
    pub scene_texture_id: ESceneTextureId,
    pub b_filtered: bool,
}
pub struct UMaterialExpressionScreenPosition {}
pub struct UMaterialExpressionSetMaterialAttributes {
    pub inputs: TArray<FExpressionInput>,
    pub attribute_set_types: TArray<crate::bindings::core_u_object::FGuid>,
}
pub struct UMaterialExpressionShaderStageSwitch {
    pub pixel_shader: FExpressionInput,
    pub vertex_shader: FExpressionInput,
}
pub struct UMaterialExpressionShadingModel {
    pub shading_model: EMaterialShadingModel,
}
pub struct UMaterialExpressionShadingPathSwitch {
    pub default: FExpressionInput,
    pub inputs: FExpressionInput,
}
pub struct UMaterialExpressionShadowReplace {
    pub default: FExpressionInput,
    pub shadow: FExpressionInput,
}
pub struct UMaterialExpressionSign {
    pub input: FExpressionInput,
}
pub struct UMaterialExpressionSine {
    pub input: FExpressionInput,
    pub period: f32,
}
pub struct UMaterialExpressionSingleLayerWaterMaterialOutput {
    pub scattering_coefficients: FExpressionInput,
    pub absorption_coefficients: FExpressionInput,
    pub phase_g: FExpressionInput,
    pub color_scale_behind_water: FExpressionInput,
}
pub struct UMaterialExpressionSkyAtmosphereLightDirection {
    pub light_index: i32,
}
pub struct UMaterialExpressionSkyAtmosphereLightIlluminance {
    pub light_index: i32,
    pub world_position: FExpressionInput,
    pub world_position_origin_type: EPositionOrigin,
}
pub struct UMaterialExpressionSkyAtmosphereLightIlluminanceOnGround {
    pub light_index: i32,
}
pub struct UMaterialExpressionSkyAtmosphereLightDiskLuminance {
    pub light_index: i32,
    pub disk_angular_diameter_override: FExpressionInput,
}
pub struct UMaterialExpressionSkyAtmosphereAerialPerspective {
    pub world_position: FExpressionInput,
    pub world_position_origin_type: EPositionOrigin,
}
pub struct UMaterialExpressionSkyAtmosphereDistantLightScatteredLuminance {}
pub struct UMaterialExpressionSkyAtmosphereViewLuminance {
    pub world_direction: FExpressionInput,
}
pub struct UMaterialExpressionSkyLightEnvMapSample {
    pub direction: FExpressionInput,
    pub roughness: FExpressionInput,
}
pub struct UMaterialExpressionSmoothStep {
    pub min: FExpressionInput,
    pub max: FExpressionInput,
    pub value: FExpressionInput,
    pub const_min: f32,
    pub const_max: f32,
    pub const_value: f32,
}
pub struct UMaterialExpressionSobol {
    pub cell: FExpressionInput,
    pub index: FExpressionInput,
    pub seed: FExpressionInput,
    pub const_index: u32,
    pub const_seed: crate::bindings::core_u_object::FVector2D,
}
pub struct UMaterialExpressionSparseVolumeTextureBase {
    pub sparse_volume_texture: UPtr<USparseVolumeTexture>,
}
pub struct UMaterialExpressionSparseVolumeTextureObject {}
pub struct UMaterialExpressionSparseVolumeTextureSample {
    pub coordinates: FExpressionInput,
    pub texture_object: FExpressionInput,
    pub mip_value: FExpressionInput,
    pub coordinates_dx: FExpressionInput,
    pub coordinates_dy: FExpressionInput,
    pub mip_value_mode: ETextureMipValueMode,
    pub sampler_source: ESamplerSourceMode,
    pub const_mip_value: i32,
}
pub struct UMaterialExpressionSparseVolumeTextureSampleParameter {
    pub parameter_name: FName,
    pub expression_guid: crate::bindings::core_u_object::FGuid,
    pub group: FName,
    pub sort_priority: i32,
}
pub struct UMaterialExpressionSparseVolumeTextureObjectParameter {}
pub struct UMaterialExpressionSpeedTree {
    pub geometry_input: FExpressionInput,
    pub wind_input: FExpressionInput,
    pub lod_input: FExpressionInput,
    pub extra_bend_ws: FExpressionInput,
    pub geometry_type: ESpeedTreeGeometryType,
    pub wind_type: ESpeedTreeWindType,
    pub lod_type: ESpeedTreeLODType,
    pub billboard_threshold: f32,
    pub b_accurate_wind_velocities: bool,
}
pub struct UMaterialExpressionSphereMask {
    pub a: FExpressionInput,
    pub b: FExpressionInput,
    pub radius: FExpressionInput,
    pub hardness: FExpressionInput,
    pub attenuation_radius: f32,
    pub hardness_percent: f32,
}
pub struct UMaterialExpressionSphericalParticleOpacity {
    pub density: FExpressionInput,
    pub constant_density: f32,
}
pub struct UMaterialExpressionSquareRoot {
    pub input: FExpressionInput,
}
pub struct UMaterialExpressionSRGBColorToWorkingColorSpace {
    pub input: FExpressionInput,
}
pub struct UMaterialExpressionStaticBool {
    pub flags_200: u8,
}
pub struct UMaterialExpressionStaticBoolParameter {
    pub flags_248: u8,
}
pub struct UMaterialExpressionStaticComponentMaskParameter {
    pub input: FExpressionInput,
    pub flags_296: u8,
}
pub struct UMaterialExpressionStaticSwitch {
    pub flags_200: u8,
    pub a: FExpressionInput,
    pub b: FExpressionInput,
    pub value: FExpressionInput,
}
pub struct UMaterialExpressionStaticSwitchParameter {
    pub a: FExpressionInput,
    pub b: FExpressionInput,
}
pub struct UMaterialExpressionStep {
    pub y: FExpressionInput,
    pub x: FExpressionInput,
    pub const_y: f32,
    pub const_x: f32,
}
pub struct UMaterialExpressionSubsurfaceMediumMaterialOutput {
    pub mean_free_path: FExpressionInput,
    pub scattering_distribution: FExpressionInput,
}
pub struct UMaterialExpressionSubtract {
    pub a: FExpressionInput,
    pub b: FExpressionInput,
    pub const_a: f32,
    pub const_b: f32,
}
pub struct UMaterialExpressionSwitch {
    pub description: FString,
    pub switch_value: FExpressionInput,
    pub const_switch_value: f32,
    pub default: FExpressionInput,
    pub const_default: f32,
    pub inputs: TArray<FSwitchCustomInput>,
}
pub struct UMaterialExpressionTangent {
    pub input: FExpressionInput,
    pub period: f32,
}
pub struct UMaterialExpressionTemporalSobol {
    pub index: FExpressionInput,
    pub seed: FExpressionInput,
    pub const_index: u32,
    pub const_seed: crate::bindings::core_u_object::FVector2D,
}
pub struct UMaterialExpressionTextureCollection {
    pub texture_collection: UPtr<UTextureCollection>,
}
pub struct UMaterialExpressionTextureCollectionParameter {
    pub parameter_name: FName,
    pub expression_guid: crate::bindings::core_u_object::FGuid,
    pub group: FName,
    pub sort_priority: i32,
}
pub struct UMaterialExpressionTextureCoordinate {
    pub coordinate_index: i32,
    pub u_tiling: f32,
    pub v_tiling: f32,
    pub flags_212: u8,
}
pub struct UMaterialExpressionTextureObject {}
pub struct UMaterialExpressionTextureObjectFromCollection {
    pub texture_collection: FExpressionInput,
    pub texture_collection_object: UPtr<UTextureCollection>,
    pub collection_index: FExpressionInput,
    pub const_collection_index: i32,
    pub texture_type: ETextureCollectionMemberType,
}
pub struct UMaterialExpressionTextureObjectParameter {}
pub struct UMaterialExpressionTextureProperty {
    pub texture_object: FExpressionInput,
    pub property: EMaterialExposedTextureProperty,
}
pub struct UMaterialExpressionTextureSampleParameter2DArray {}
pub struct UMaterialExpressionTextureSampleParameterCube {}
pub struct UMaterialExpressionTextureSampleParameterCubeArray {}
pub struct UMaterialExpressionTextureSampleParameterSubUV {
    pub flags_632: u8,
}
pub struct UMaterialExpressionTextureSampleParameterVolume {}
pub struct UMaterialExpressionTime {
    pub flags_200: u8,
    pub period: f32,
}
pub struct UMaterialExpressionTransform {
    pub input: FExpressionInput,
    pub transform_source_type: EMaterialVectorCoordTransformSource,
    pub transform_type: EMaterialVectorCoordTransform,
}
pub struct UMaterialExpressionTransformPosition {
    pub input: FExpressionInput,
    pub transform_source_type: EMaterialPositionTransformSource,
    pub transform_type: EMaterialPositionTransformSource,
    pub periodic_world_tile_size: FExpressionInput,
    pub first_person_interpolation_alpha: FExpressionInput,
    pub const_periodic_world_tile_size: f32,
    pub const_first_person_interpolation_alpha: f32,
    pub b_uses_periodic_world_position: bool,
    pub b_uses_first_person_interpolation_alpha: bool,
}
pub struct UMaterialExpressionTruncate {
    pub input: FExpressionInput,
}
pub struct UMaterialExpressionTruncateLWC {
    pub input: FExpressionInput,
}
pub struct UMaterialExpressionTwoSidedSign {}
pub struct UMaterialExpressionUserSceneTexture {
    pub coordinates: FExpressionInput,
    pub user_scene_texture: FName,
    pub b_filtered: bool,
    pub b_clamped: bool,
}
pub struct UMaterialExpressionVectorNoise {
    pub position: FExpressionInput,
    pub world_position_origin_type: EPositionOrigin,
    pub noise_function: EVectorNoiseFunction,
    pub quality: i32,
    pub flags_256: u8,
    pub tile_size: u32,
}
pub struct UMaterialExpressionVertexColor {}
pub struct UMaterialExpressionVertexInterpolator {
    pub input: FExpressionInput,
}
pub struct UMaterialExpressionVertexNormalWS {}
pub struct UMaterialExpressionVertexTangentWS {}
pub struct UMaterialExpressionViewProperty {
    pub property: EMaterialExposedViewProperty,
}
pub struct UMaterialExpressionViewSize {}
pub struct UMaterialExpressionVirtualTextureFeatureSwitch {
    pub no: FExpressionInput,
    pub yes: FExpressionInput,
}
pub struct UMaterialExpressionVolumetricAdvancedMaterialInput {}
pub struct UMaterialExpressionVolumetricCloudEmptySpaceSkippingInput {}
pub struct UMaterialExpressionVolumetricAdvancedMaterialOutput {
    pub phase_g: FExpressionInput,
    pub phase_g2: FExpressionInput,
    pub phase_blend: FExpressionInput,
    pub multi_scattering_contribution: FExpressionInput,
    pub multi_scattering_occlusion: FExpressionInput,
    pub multi_scattering_eccentricity: FExpressionInput,
    pub conservative_density: FExpressionInput,
    pub const_phase_g: f32,
    pub const_phase_g2: f32,
    pub const_phase_blend: f32,
    pub per_sample_phase_evaluation: bool,
    pub multi_scattering_approximation_octave_count: u32,
    pub const_multi_scattering_contribution: f32,
    pub const_multi_scattering_occlusion: f32,
    pub const_multi_scattering_eccentricity: f32,
    pub b_ground_contribution: bool,
    pub b_gray_scale_material: bool,
    pub b_ray_march_volume_shadow: bool,
    pub b_clamp_multi_scattering_contribution: bool,
}
pub struct UMaterialExpressionVolumetricCloudEmptySpaceSkippingOutput {
    pub contains_matter: FExpressionInput,
}
pub struct UMaterialExpressionWorldPosition {
    pub world_position_shader_offset: EWorldPositionIncludedOffsets,
}
pub struct UMaterialFunctionInterfaceEditorOnlyData {}
pub struct UMaterialFunctionEditorOnlyData {
    pub expression_collection: FMaterialExpressionCollection,
}
pub struct UMaterialFunctionInterface {
    pub editor_only_data: UPtr<UMaterialFunctionInterfaceEditorOnlyData>,
    pub state_id: crate::bindings::core_u_object::FGuid,
    pub material_function_usage: EMaterialFunctionUsage,
    pub combined_input_types: u64,
    pub combined_output_types: u64,
    pub thumbnail_info: UPtr<UThumbnailInfo>,
}
pub struct UMaterialFunction {
    pub parent_function: UPtr<UMaterialFunction>,
    pub description: FString,
    pub user_exposed_caption: FString,
    pub flags_144: u8,
    pub library_categories_deprecated: TArray<FString>,
    pub library_categories_text: TArray<FText>,
    pub preview_material: UPtr<UMaterial>,
    pub dependent_function_expression_candidates: TArray<
        UPtr<UMaterialExpressionMaterialFunctionCall>,
    >,
    pub preview_blend_mode: EBlendMode,
    pub preview_material_domain: EMaterialDomain,
    pub flags_232: u8,
    pub function_expressions_deprecated: TArray<UPtr<UMaterialExpression>>,
    pub function_editor_comments_deprecated: TArray<UPtr<UMaterialExpressionComment>>,
}
pub struct UMaterialFunctionInstance {
    pub parent: UPtr<UMaterialFunctionInterface>,
    pub base: UPtr<UMaterialFunctionInterface>,
    pub scalar_parameter_values: TArray<FScalarParameterValue>,
    pub vector_parameter_values: TArray<FVectorParameterValue>,
    pub double_vector_parameter_values: TArray<FDoubleVectorParameterValue>,
    pub texture_parameter_values: TArray<FTextureParameterValue>,
    pub texture_collection_parameter_values: TArray<FTextureCollectionParameterValue>,
    pub parameter_collection_parameter_values: TArray<
        FParameterCollectionParameterValue,
    >,
    pub font_parameter_values: TArray<FFontParameterValue>,
    pub static_switch_parameter_values: TArray<FStaticSwitchParameter>,
    pub static_component_mask_parameter_values: TArray<FStaticComponentMaskParameter>,
    pub runtime_virtual_texture_parameter_values: TArray<
        FRuntimeVirtualTextureParameterValue,
    >,
    pub sparse_volume_texture_parameter_values: TArray<
        FSparseVolumeTextureParameterValue,
    >,
    pub preview_material: UPtr<UMaterialInstanceConstant>,
}
pub struct UMaterialFunctionMaterialLayer {}
pub struct UMaterialFunctionMaterialLayerInstance {}
pub struct UMaterialFunctionMaterialLayerBlend {}
pub struct UMaterialFunctionMaterialLayerBlendInstance {}
pub struct UMaterialParameterCollectionInstance {
    pub collection: TWeakObjectPtr<UMaterialParameterCollection>,
}
pub struct UMeshDeformer {}
pub struct UMeshDeformerInstanceSettings {}
pub struct UMeshDeformerInstance {}
pub struct UMeshDeformerProducer {}
pub struct IMeshDeformerProducer {}
pub struct UMeshDrawCommandStatsSettings {
    pub budgets: TArray<FMeshDrawCommandStatsBudget>,
    pub budget_totals: TArray<FMeshDrawCommandStatsBudgetTotals>,
    pub collection_for_csv_profiler: i32,
}
pub struct UMicroTransactionBase {
    pub available_products: TArray<FPurchaseInfo>,
    pub last_error: FString,
    pub last_error_solution: FString,
}
pub struct UNavAgentInterface {}
pub struct INavAgentInterface {}
pub struct UNavEdgeProviderInterface {}
pub struct INavEdgeProviderInterface {}
pub struct UNavigationDataInterface {}
pub struct INavigationDataInterface {}
pub struct UNavigationInvokerInterface {}
pub struct INavigationInvokerInterface {}
pub struct UNavPathObserverInterface {}
pub struct INavPathObserverInterface {}
pub struct UNavRelevantInterface {}
pub struct INavRelevantInterface {}
pub struct UNetworkPredictionInterface {}
pub struct INetworkPredictionInterface {}
pub struct UFXSystemAsset {
    pub max_pool_size: u32,
    pub pool_prime_size: u32,
}
pub struct UParticleSystem {
    pub update_time_fps: f32,
    pub update_time_delta: f32,
    pub warmup_time: f32,
    pub warmup_tick_rate: f32,
    pub emitters: TArray<UPtr<UParticleEmitter>>,
    pub preview_component: UPtr<UParticleSystemComponent>,
    pub thumbnail_angle: crate::bindings::core_u_object::FRotator,
    pub thumbnail_distance: f32,
    pub thumbnail_warmup: f32,
    pub curve_ed_setup: UPtr<UInterpCurveEdSetup>,
    pub lod_distance_check_time: f32,
    pub macro_uv_radius: f32,
    pub lod_distances: TArray<f32>,
    pub editor_lod_setting: i32,
    pub lod_settings: TArray<FParticleSystemLOD>,
    pub fixed_relative_bounding_box: crate::bindings::core_u_object::FBox,
    pub seconds_before_inactive: f32,
    pub floor_mesh: FString,
    pub floor_position: crate::bindings::core_u_object::FVector,
    pub floor_rotation: crate::bindings::core_u_object::FRotator,
    pub floor_scale: f32,
    pub floor_scale3_d: crate::bindings::core_u_object::FVector,
    pub background_color: crate::bindings::core_u_object::FColor,
    pub delay: f32,
    pub delay_low: f32,
    pub flags_396: u8,
    pub thumbnail_image: UPtr<UTexture2D>,
    pub flags_408: u8,
    pub system_update_mode: EParticleSystemUpdateMode,
    pub lod_method: ParticleSystemLODMethod,
    pub insignificant_reaction: EParticleSystemInsignificanceReaction,
    pub occlusion_bounds_method: EParticleSystemOcclusionBoundsMethod,
    pub max_significance_level: EParticleSignificanceLevel,
    pub min_time_between_ticks: u32,
    pub insignificance_delay: f32,
    pub macro_uv_position: crate::bindings::core_u_object::FVector,
    pub custom_occlusion_bounds: crate::bindings::core_u_object::FBox,
    pub solo_tracking: TArray<FLODSoloTrack>,
    pub named_material_slots: TArray<FNamedEmitterMaterial>,
}
pub struct UParticleModule {
    pub flags_48: u8,
    pub flags_49: u8,
    pub lod_validity: u8,
    pub module_editor_color: crate::bindings::core_u_object::FColor,
}
pub struct UParticleModuleAccelerationBase {
    pub flags_56: u8,
}
pub struct UParticleModuleAcceleration {
    pub acceleration: FRawDistributionVector,
    pub flags_160: u8,
}
pub struct UParticleModuleAccelerationConstant {
    pub acceleration: crate::bindings::core_u_object::FVector,
}
pub struct UParticleModuleAccelerationDrag {
    pub drag_coefficient_deprecated: UPtr<UDistributionFloat>,
    pub drag_coefficient_raw: FRawDistributionFloat,
}
pub struct UParticleModuleAccelerationDragScaleOverLife {
    pub drag_scale_deprecated: UPtr<UDistributionFloat>,
    pub drag_scale_raw: FRawDistributionFloat,
}
pub struct UParticleModuleAccelerationOverLifetime {
    pub accel_over_life: FRawDistributionVector,
}
pub struct UParticleModuleAttractorBase {}
pub struct UParticleModuleAttractorLine {
    pub end_point0: crate::bindings::core_u_object::FVector,
    pub end_point1: crate::bindings::core_u_object::FVector,
    pub range: FRawDistributionFloat,
    pub strength: FRawDistributionFloat,
}
pub struct UParticleModuleAttractorParticle {
    pub emitter_name: FName,
    pub range: FRawDistributionFloat,
    pub flags_120: u8,
    pub strength: FRawDistributionFloat,
    pub flags_176: u8,
    pub selection_method: EAttractorParticleSelectionMethod,
    pub flags_184: u8,
    pub last_sel_index: i32,
}
pub struct UParticleModuleAttractorPoint {
    pub position: FRawDistributionVector,
    pub range: FRawDistributionFloat,
    pub strength: FRawDistributionFloat,
    pub flags_248: u8,
    pub flags_249: u8,
}
pub struct UParticleModuleAttractorPointGravity {
    pub position: crate::bindings::core_u_object::FVector,
    pub radius: f32,
    pub strength_deprecated: UPtr<UDistributionFloat>,
    pub strength_raw: FRawDistributionFloat,
}
pub struct UParticleModuleBeamBase {}
pub struct UParticleModuleBeamModifier {
    pub modifier_type: BeamModifierType,
    pub position_options: FBeamModifierOptions,
    pub position: FRawDistributionVector,
    pub tangent_options: FBeamModifierOptions,
    pub tangent: FRawDistributionVector,
    pub flags_264: u8,
    pub strength_options: FBeamModifierOptions,
    pub strength: FRawDistributionFloat,
}
pub struct UParticleModuleBeamNoise {
    pub flags_56: u8,
    pub frequency: i32,
    pub frequency_low_range: i32,
    pub noise_range: FRawDistributionVector,
    pub noise_range_scale: FRawDistributionFloat,
    pub flags_216: u8,
    pub noise_speed: FRawDistributionVector,
    pub flags_320: u8,
    pub noise_lock_radius: f32,
    pub flags_328: u8,
    pub noise_lock_time: f32,
    pub noise_tension: f32,
    pub flags_340: u8,
    pub noise_tangent_strength: FRawDistributionFloat,
    pub noise_tessellation: i32,
    pub flags_396: u8,
    pub frequency_distance: f32,
    pub flags_404: u8,
    pub noise_scale: FRawDistributionFloat,
}
pub struct UParticleModuleBeamSource {
    pub source_method: Beam2SourceTargetMethod,
    pub source_name: FName,
    pub flags_72: u8,
    pub source: FRawDistributionVector,
    pub flags_176: u8,
    pub source_tangent_method: Beam2SourceTargetTangentMethod,
    pub source_tangent: FRawDistributionVector,
    pub flags_280: u8,
    pub source_strength: FRawDistributionFloat,
    pub flags_336: u8,
}
pub struct UParticleModuleBeamTarget {
    pub target_method: Beam2SourceTargetMethod,
    pub target_name: FName,
    pub target: FRawDistributionVector,
    pub flags_168: u8,
    pub target_tangent_method: Beam2SourceTargetTangentMethod,
    pub target_tangent: FRawDistributionVector,
    pub flags_272: u8,
    pub target_strength: FRawDistributionFloat,
    pub flags_328: u8,
    pub lock_radius: f32,
}
pub struct UParticleModuleCameraBase {}
pub struct UParticleModuleCameraOffset {
    pub camera_offset: FRawDistributionFloat,
    pub flags_104: u8,
    pub update_method: EParticleCameraOffsetUpdateMethod,
}
pub struct UParticleModuleCollisionBase {}
pub struct UParticleModuleCollision {
    pub damping_factor: FRawDistributionVector,
    pub damping_factor_rotation: FRawDistributionVector,
    pub max_collisions: FRawDistributionFloat,
    pub collision_completion_option: EParticleCollisionComplete,
    pub collision_types: TArray<EObjectTypeQuery>,
    pub flags_328: u8,
    pub particle_mass: FRawDistributionFloat,
    pub dir_scalar: f32,
    pub flags_388: u8,
    pub vertical_fudge_factor: f32,
    pub delay_amount: FRawDistributionFloat,
    pub flags_448: u8,
    pub max_collision_distance: f32,
}
pub struct UParticleModuleCollisionGPU {
    pub resilience: FRawDistributionFloat,
    pub resilience_scale_over_life: FRawDistributionFloat,
    pub friction: f32,
    pub random_spread: f32,
    pub random_distribution: f32,
    pub radius_scale: f32,
    pub radius_bias: f32,
    pub response: EParticleCollisionResponse,
    pub collision_mode: EParticleCollisionMode,
}
pub struct UParticleModuleColorBase {}
pub struct UParticleModuleColor {
    pub start_color: FRawDistributionVector,
    pub start_alpha: FRawDistributionFloat,
    pub flags_200: u8,
}
pub struct UParticleModuleColorOverLife {
    pub color_over_life: FRawDistributionVector,
    pub alpha_over_life: FRawDistributionFloat,
    pub flags_200: u8,
}
pub struct UParticleModuleColorScaleOverLife {
    pub color_scale_over_life: FRawDistributionVector,
    pub alpha_scale_over_life: FRawDistributionFloat,
    pub flags_200: u8,
}
pub struct UParticleModuleColor_Seeded {
    pub random_seed_info: FParticleRandomSeedInfo,
}
pub struct UParticleModuleEventBase {}
pub struct UParticleModuleEventGenerator {
    pub events: TArray<FParticleEvent_GenerateInfo>,
}
pub struct UParticleModuleEventReceiverBase {
    pub event_generator_type: EParticleEventType,
    pub event_name: FName,
}
pub struct UParticleModuleEventReceiverKillParticles {
    pub flags_72: u8,
}
pub struct UParticleModuleEventReceiverSpawn {
    pub spawn_count: FRawDistributionFloat,
    pub flags_120: u8,
    pub inherit_velocity_scale: FRawDistributionVector,
    pub physical_materials: TArray<
        UPtr<crate::bindings::physics_core::UPhysicalMaterial>,
    >,
    pub flags_240: u8,
}
pub struct UParticleModuleEventSendToGame {}
pub struct UParticleModuleKillBase {}
pub struct UParticleModuleKillBox {
    pub lower_left_corner: FRawDistributionVector,
    pub upper_right_corner: FRawDistributionVector,
    pub flags_248: u8,
}
pub struct UParticleModuleKillHeight {
    pub height: FRawDistributionFloat,
    pub flags_104: u8,
}
pub struct UParticleModuleLifetimeBase {}
pub struct UParticleModuleLifetime {
    pub lifetime: FRawDistributionFloat,
}
pub struct UParticleModuleLifetime_Seeded {
    pub random_seed_info: FParticleRandomSeedInfo,
}
pub struct UParticleModuleLightBase {}
pub struct UParticleModuleLight {
    pub b_use_inverse_squared_falloff: bool,
    pub b_affects_translucency: bool,
    pub flags_60: u8,
    pub b_preview_light_radius: bool,
    pub spawn_fraction: f32,
    pub color_scale_over_life: FRawDistributionVector,
    pub brightness_over_life: FRawDistributionFloat,
    pub radius_scale: FRawDistributionFloat,
    pub light_exponent: FRawDistributionFloat,
    pub inverse_exposure_blend: f32,
    pub lighting_channels: FLightingChannels,
    pub volumetric_scattering_intensity: f32,
    pub b_high_quality_lights: bool,
    pub b_shadow_casting_lights: bool,
}
pub struct UParticleModuleLight_Seeded {
    pub random_seed_info: FParticleRandomSeedInfo,
}
pub struct UParticleModuleLocationBase {}
pub struct UParticleModuleLocation {
    pub start_location: FRawDistributionVector,
    pub distribute_over_n_points: f32,
    pub distribute_threshold: f32,
}
pub struct UParticleModuleLocationBoneSocket {
    pub source_type: ELocationBoneSocketSource,
    pub universal_offset: crate::bindings::core_u_object::FVector,
    pub source_locations: TArray<FLocationBoneSocketInfo>,
    pub selection_method: ELocationBoneSocketSelectionMethod,
    pub flags_108: u8,
    pub inherit_velocity_scale: f32,
    pub skel_mesh_actor_param_name: FName,
    pub num_pre_selected_indices: i32,
    pub editor_skel_mesh: UPtr<USkeletalMesh>,
}
pub struct UParticleModuleLocationDirect {
    pub location: FRawDistributionVector,
    pub location_offset: FRawDistributionVector,
    pub scale_factor: FRawDistributionVector,
    pub direction: FRawDistributionVector,
}
pub struct UParticleModuleLocationEmitter {
    pub emitter_name: FName,
    pub selection_method: ELocationEmitterSelectionMethod,
    pub flags_72: u8,
    pub inherit_source_velocity_scale: f32,
    pub flags_80: u8,
    pub inherit_source_rotation_scale: f32,
}
pub struct UParticleModuleLocationEmitterDirect {
    pub emitter_name: FName,
}
pub struct UParticleModuleLocationPrimitiveBase {
    pub flags_56: u8,
    pub velocity_scale: FRawDistributionFloat,
    pub start_location: FRawDistributionVector,
}
pub struct UParticleModuleLocationPrimitiveCylinder {
    pub flags_208: u8,
    pub start_radius: FRawDistributionFloat,
    pub start_height: FRawDistributionFloat,
    pub height_axis: CylinderHeightAxis,
}
pub struct UParticleModuleLocationPrimitiveCylinder_Seeded {
    pub random_seed_info: FParticleRandomSeedInfo,
}
pub struct UParticleModuleLocationPrimitiveSphere {
    pub start_radius: FRawDistributionFloat,
}
pub struct UParticleModuleLocationPrimitiveSphere_Seeded {
    pub random_seed_info: FParticleRandomSeedInfo,
}
pub struct UParticleModuleLocationPrimitiveTriangle {
    pub start_offset: FRawDistributionVector,
    pub height: FRawDistributionFloat,
    pub angle: FRawDistributionFloat,
    pub thickness: FRawDistributionFloat,
}
pub struct UParticleModuleLocationSkelVertSurface {
    pub source_type: ELocationSkelVertSurfaceSource,
    pub universal_offset: crate::bindings::core_u_object::FVector,
    pub flags_88: u8,
    pub inherit_velocity_scale: f32,
    pub skel_mesh_actor_param_name: FName,
    pub editor_skel_mesh: UPtr<USkeletalMesh>,
    pub valid_associated_bones: TArray<FName>,
    pub flags_136: u8,
    pub normal_to_compare: crate::bindings::core_u_object::FVector,
    pub normal_check_tolerance_degrees: f32,
    pub normal_check_tolerance: f32,
    pub valid_material_indices: TArray<i32>,
    pub flags_192: u8,
    pub inherit_uv_channel: u32,
}
pub struct UParticleModuleLocationWorldOffset {}
pub struct UParticleModuleLocationWorldOffset_Seeded {
    pub random_seed_info: FParticleRandomSeedInfo,
}
pub struct UParticleModuleLocation_Seeded {
    pub random_seed_info: FParticleRandomSeedInfo,
}
pub struct UParticleModuleMaterialBase {}
pub struct UParticleModuleMeshMaterial {
    pub mesh_materials: TArray<UPtr<UMaterialInterface>>,
}
pub struct UParticleModuleRotationBase {}
pub struct UParticleModuleMeshRotation {
    pub start_rotation: FRawDistributionVector,
    pub flags_152: u8,
}
pub struct UParticleModuleRotationRateBase {}
pub struct UParticleModuleMeshRotationRate {
    pub start_rotation_rate: FRawDistributionVector,
}
pub struct UParticleModuleMeshRotationRateMultiplyLife {
    pub life_multiplier: FRawDistributionVector,
}
pub struct UParticleModuleMeshRotationRateOverLife {
    pub rot_rate: FRawDistributionVector,
    pub flags_152: u8,
}
pub struct UParticleModuleMeshRotationRate_Seeded {
    pub random_seed_info: FParticleRandomSeedInfo,
}
pub struct UParticleModuleMeshRotation_Seeded {
    pub random_seed_info: FParticleRandomSeedInfo,
}
pub struct UParticleModuleOrbitBase {
    pub flags_56: u8,
}
pub struct UParticleModuleOrbit {
    pub chain_mode: EOrbitChainMode,
    pub offset_amount: FRawDistributionVector,
    pub offset_options: FOrbitOptions,
    pub rotation_amount: FRawDistributionVector,
    pub rotation_options: FOrbitOptions,
    pub rotation_rate_amount: FRawDistributionVector,
    pub rotation_rate_options: FOrbitOptions,
}
pub struct UParticleModuleOrientationBase {}
pub struct UParticleModuleOrientationAxisLock {
    pub lock_axis_flags: EParticleAxisLock,
}
pub struct UParticleModuleParameterBase {}
pub struct UParticleModuleParameterDynamic {
    pub dynamic_params: TArray<FEmitterDynamicParameter>,
    pub update_flags: i32,
    pub flags_76: u8,
}
pub struct UParticleModuleParameterDynamic_Seeded {
    pub random_seed_info: FParticleRandomSeedInfo,
}
pub struct UParticleModulePivotOffset {
    pub pivot_offset: crate::bindings::core_u_object::FVector2D,
}
pub struct UParticleModuleRequired {
    pub material: UPtr<UMaterialInterface>,
    pub min_facing_camera_blend_distance: f32,
    pub max_facing_camera_blend_distance: f32,
    pub emitter_origin: crate::bindings::core_u_object::FVector,
    pub emitter_rotation: crate::bindings::core_u_object::FRotator,
    pub screen_alignment: EParticleScreenAlignment,
    pub flags_121: u8,
    pub sort_mode: EParticleSortMode,
    pub flags_123: u8,
    pub emitter_duration: f32,
    pub spawn_rate: FRawDistributionFloat,
    pub burst_list: TArray<FParticleBurst>,
    pub emitter_delay: f32,
    pub emitter_delay_low: f32,
    pub flags_200: u8,
    pub interpolation_method: EParticleSubUVInterpMethod,
    pub flags_202: u8,
    pub particle_burst_method: EParticleBurstMethod,
    pub flags_204: u8,
    pub opacity_source_mode: EOpacitySourceMode,
    pub emitter_normals_mode: EEmitterNormalsMode,
    pub flags_207: u8,
    pub sub_images_horizontal: i32,
    pub sub_images_vertical: i32,
    pub random_image_time: f32,
    pub random_image_changes: i32,
    pub macro_uv_position: crate::bindings::core_u_object::FVector,
    pub macro_uv_radius: f32,
    pub uv_flipping_mode: EParticleUVFlipMode,
    pub bounding_mode: ESubUVBoundingVertexCount,
    pub flags_254: u8,
    pub normals_sphere_center: crate::bindings::core_u_object::FVector,
    pub alpha_threshold: f32,
    pub emitter_loops: i32,
    pub cutout_texture: UPtr<UTexture2D>,
    pub max_draw_count: i32,
    pub emitter_duration_low: f32,
    pub normals_cylinder_direction: crate::bindings::core_u_object::FVector,
    pub named_material_overrides: TArray<FName>,
}
pub struct UParticleModuleRotation {
    pub start_rotation: FRawDistributionFloat,
}
pub struct UParticleModuleRotationOverLifetime {
    pub rotation_over_life: FRawDistributionFloat,
    pub flags_104: u8,
}
pub struct UParticleModuleRotationRate {
    pub start_rotation_rate: FRawDistributionFloat,
}
pub struct UParticleModuleRotationRateMultiplyLife {
    pub life_multiplier: FRawDistributionFloat,
}
pub struct UParticleModuleRotationRate_Seeded {
    pub random_seed_info: FParticleRandomSeedInfo,
}
pub struct UParticleModuleRotation_Seeded {
    pub random_seed_info: FParticleRandomSeedInfo,
}
pub struct UParticleModuleSizeBase {}
pub struct UParticleModuleSize {
    pub start_size: FRawDistributionVector,
}
pub struct UParticleModuleSizeMultiplyLife {
    pub life_multiplier: FRawDistributionVector,
    pub flags_152: u8,
}
pub struct UParticleModuleSizeScale {
    pub size_scale: FRawDistributionVector,
    pub flags_152: u8,
}
pub struct UParticleModuleSizeScaleBySpeed {
    pub speed_scale: crate::bindings::core_u_object::FVector2D,
    pub max_scale: crate::bindings::core_u_object::FVector2D,
}
pub struct UParticleModuleSize_Seeded {
    pub random_seed_info: FParticleRandomSeedInfo,
}
pub struct UParticleModuleSourceMovement {
    pub source_movement_scale: FRawDistributionVector,
}
pub struct UParticleModuleSpawnBase {
    pub flags_56: u8,
}
pub struct UParticleModuleSpawn {
    pub rate: FRawDistributionFloat,
    pub rate_scale: FRawDistributionFloat,
    pub burst_list: TArray<FParticleBurst>,
    pub burst_scale: FRawDistributionFloat,
    pub particle_burst_method: EParticleBurstMethod,
    pub flags_228: u8,
}
pub struct UParticleModuleSpawnPerUnit {
    pub unit_scalar: f32,
    pub movement_tolerance: f32,
    pub spawn_per_unit: FRawDistributionFloat,
    pub max_frame_distance: f32,
    pub flags_124: u8,
}
pub struct UParticleModuleSubUVBase {}
pub struct UParticleModuleSubUV {
    pub animation: UPtr<USubUVAnimation>,
    pub sub_image_index: FRawDistributionFloat,
    pub flags_112: u8,
}
pub struct UParticleModuleSubUVMovie {
    pub flags_120: u8,
    pub frame_rate: FRawDistributionFloat,
    pub starting_frame: i32,
}
pub struct UParticleModuleTrailBase {}
pub struct UParticleModuleTrailSource {
    pub source_method: ETrail2SourceMethod,
    pub source_name: FName,
    pub source_strength: FRawDistributionFloat,
    pub flags_120: u8,
    pub source_offset_count: i32,
    pub source_offset_defaults: TArray<crate::bindings::core_u_object::FVector>,
    pub selection_method: EParticleSourceSelectionMethod,
    pub flags_148: u8,
}
pub struct UParticleModuleTypeDataBase {}
pub struct UParticleModuleTypeDataAnimTrail {
    pub flags_56: u8,
    pub tiling_distance: f32,
    pub distance_tessellation_step_size: f32,
    pub tangent_tessellation_step_size: f32,
    pub width_tessellation_step_size: f32,
}
pub struct UParticleModuleTypeDataBeam2 {
    pub beam_method: EBeam2Method,
    pub texture_tile: i32,
    pub texture_tile_distance: f32,
    pub sheets: i32,
    pub max_beam_count: i32,
    pub speed: f32,
    pub interpolation_points: i32,
    pub flags_84: u8,
    pub up_vector_step_size: i32,
    pub branch_parent_name: FName,
    pub distance: FRawDistributionFloat,
    pub taper_method: EBeamTaperMethod,
    pub taper_factor: FRawDistributionFloat,
    pub taper_scale: FRawDistributionFloat,
    pub flags_256: u8,
}
pub struct UParticleModuleTypeDataGpu {
    pub emitter_info: FGPUSpriteEmitterInfo,
    pub resource_data: FGPUSpriteResourceData,
    pub camera_motion_blur_amount: f32,
    pub flags_1492: u8,
}
pub struct UParticleModuleTypeDataMesh {
    pub mesh: UPtr<UStaticMesh>,
    pub lod_size_scale: f32,
    pub flags_76: u8,
    pub mesh_alignment: EMeshScreenAlignment,
    pub flags_78: u8,
    pub roll_pitch_yaw_range: FRawDistributionVector,
    pub axis_lock_option: EParticleAxisLock,
    pub flags_177: u8,
    pub camera_facing_up_axis_option_deprecated: EMeshCameraFacingUpAxis,
    pub camera_facing_option: EMeshCameraFacingOptions,
    pub flags_180: u8,
}
pub struct UParticleModuleTypeDataRibbon {
    pub max_tessellation_between_particles: i32,
    pub sheets_per_trail: i32,
    pub max_trail_count: i32,
    pub max_particle_in_trail_count: i32,
    pub flags_72: u8,
    pub render_axis: ETrailsRenderAxisOption,
    pub tangent_spawning_scalar: f32,
    pub flags_84: u8,
    pub tiling_distance: f32,
    pub distance_tessellation_step_size: f32,
    pub flags_96: u8,
    pub tangent_tessellation_scalar: f32,
}
pub struct UParticleModuleVectorFieldBase {}
pub struct UParticleModuleVectorFieldGlobal {
    pub flags_56: u8,
    pub global_vector_field_scale: f32,
    pub global_vector_field_tightness: f32,
}
pub struct UParticleModuleVectorFieldLocal {
    pub vector_field: UPtr<UVectorField>,
    pub relative_translation: crate::bindings::core_u_object::FVector,
    pub relative_rotation: crate::bindings::core_u_object::FRotator,
    pub relative_scale3_d: crate::bindings::core_u_object::FVector,
    pub intensity: f32,
    pub tightness: f32,
    pub flags_144: u8,
}
pub struct UParticleModuleVectorFieldRotation {
    pub min_initial_rotation: crate::bindings::core_u_object::FVector,
    pub max_initial_rotation: crate::bindings::core_u_object::FVector,
}
pub struct UParticleModuleVectorFieldRotationRate {
    pub rotation_rate: crate::bindings::core_u_object::FVector,
}
pub struct UParticleModuleVectorFieldScale {
    pub vector_field_scale_deprecated: UPtr<UDistributionFloat>,
    pub vector_field_scale_raw: FRawDistributionFloat,
}
pub struct UParticleModuleVectorFieldScaleOverLife {
    pub vector_field_scale_over_life_deprecated: UPtr<UDistributionFloat>,
    pub vector_field_scale_over_life_raw: FRawDistributionFloat,
}
pub struct UParticleModuleVelocityBase {
    pub flags_56: u8,
}
pub struct UParticleModuleVelocity {
    pub start_velocity: FRawDistributionVector,
    pub start_velocity_radial: FRawDistributionFloat,
}
pub struct UParticleModuleVelocityCone {
    pub angle: FRawDistributionFloat,
    pub velocity: FRawDistributionFloat,
    pub direction: crate::bindings::core_u_object::FVector,
}
pub struct UParticleModuleVelocityInheritParent {
    pub scale: FRawDistributionVector,
}
pub struct UParticleModuleVelocityOverLifetime {
    pub vel_over_life: FRawDistributionVector,
    pub flags_160: u8,
}
pub struct UParticleModuleVelocity_Seeded {
    pub random_seed_info: FParticleRandomSeedInfo,
}
pub struct UParticleEmitter {
    pub emitter_name: FName,
    pub sub_uv_data_offset: i32,
    pub emitter_render_mode: EEmitterRenderMode,
    pub significance_level: EParticleSignificanceLevel,
    pub flags_67: u8,
    pub flags_68: u8,
    pub emitter_editor_color: crate::bindings::core_u_object::FColor,
    pub lod_levels: TArray<UPtr<UParticleLODLevel>>,
    pub peak_active_particles: i32,
    pub initial_allocation_count: i32,
    pub quality_level_spawn_rate_scale: f32,
    pub detail_mode_bitmask: u32,
    pub detail_mode_display: FString,
}
pub struct UParticleSpriteEmitter {}
pub struct UParticleSystemReplay {
    pub clip_id_number: i32,
}
pub struct UPathFollowingAgentInterface {}
pub struct IPathFollowingAgentInterface {}
pub struct UPhysicsSpringComponent {
    pub spring_stiffness: f32,
    pub spring_damping: f32,
    pub spring_length_at_rest: f32,
    pub spring_radius: f32,
    pub spring_channel: ECollisionChannel,
    pub b_ignore_self: bool,
    pub spring_compression: f32,
}
pub struct UPhysicsThreadLibrary {}
pub struct UPhysicsThrusterComponent {
    pub thrust_strength: f32,
}
pub struct UPieFixupTestObject {
    pub path: crate::bindings::core_u_object::FSoftObjectPath,
    pub typed_ptr: TSoftObjectPtr<AActor>,
    pub _struct: FPieFixupStructWithSoftObjectPath,
    pub array: TArray<FPieFixupStructWithSoftObjectPath>,
}
pub struct ASceneCapture {
    pub mesh_comp_deprecated: UPtr<UStaticMeshComponent>,
    pub scene_component: UPtr<USceneComponent>,
}
pub struct APlanarReflection {
    pub planar_reflection_component: UPtr<UPlanarReflectionComponent>,
    pub sprite_component: UPtr<UBillboardComponent>,
    pub b_show_preview_plane_deprecated: bool,
}
pub struct USceneCaptureComponent {
    pub primitive_render_mode: ESceneCapturePrimitiveRenderMode,
    pub capture_source: ESceneCaptureSource,
    pub flags_658: u8,
    pub b_always_persist_rendering_state: bool,
    pub hidden_components: TArray<TWeakObjectPtr<UPrimitiveComponent>>,
    pub hidden_actors: TArray<UPtr<AActor>>,
    pub show_only_components: TArray<TWeakObjectPtr<UPrimitiveComponent>>,
    pub show_only_actors: TArray<UPtr<AActor>>,
    pub lod_distance_factor: f32,
    pub max_view_distance_override: f32,
    pub capture_sort_priority: i32,
    pub b_use_ray_tracing_if_enabled: bool,
    pub collection_transform: UPtr<UMaterialParameterCollection>,
    pub collection_transform_world_to_local: FName,
    pub collection_transform_projection: FName,
    pub view_lighting_channels: FViewLightingChannels,
    pub show_flag_settings: TArray<FEngineShowFlagsSetting>,
    pub profiling_event_name: FString,
}
pub struct UPlanarReflectionComponent {
    pub preview_box: UPtr<UBoxComponent>,
    pub normal_distortion_strength: f32,
    pub prefilter_roughness: f32,
    pub prefilter_roughness_distance: f32,
    pub screen_percentage: i32,
    pub extra_fov: f32,
    pub distance_from_plane_fade_start_deprecated: f32,
    pub distance_from_plane_fade_end_deprecated: f32,
    pub distance_from_plane_fadeout_start: f32,
    pub distance_from_plane_fadeout_end: f32,
    pub angle_from_plane_fade_start: f32,
    pub angle_from_plane_fade_end: f32,
    pub b_show_preview_plane: bool,
    pub b_render_scene_two_sided: bool,
}
pub struct APlaneReflectionCapture {}
pub struct UPlaneReflectionCaptureComponent {
    pub influence_radius_scale: f32,
    pub preview_influence_radius: UPtr<UDrawSphereComponent>,
    pub preview_capture_box: UPtr<UBoxComponent>,
}
pub struct UPlatformInterfaceWebResponse {
    pub original_url: FString,
    pub response_code: i32,
    pub tag: i32,
    pub string_response: FString,
    pub binary_response: TArray<u8>,
}
pub struct APointLight {
    pub point_light_component: UPtr<UPointLightComponent>,
}
pub struct UPolys {}
pub struct APrecomputedVisibilityOverrideVolume {
    pub override_visible_actors: TArray<UPtr<AActor>>,
    pub override_invisible_actors: TArray<UPtr<AActor>>,
    pub override_invisible_levels: TArray<FName>,
}
pub struct ARigidBodyBase {}
pub struct ARadialForceActor {
    pub force_component: UPtr<URadialForceComponent>,
    pub sprite_component: UPtr<UBillboardComponent>,
}
pub struct UReplicationDriver {}
pub struct UReplicationConnectionDriver {}
pub struct URVOAvoidanceInterface {}
pub struct IRVOAvoidanceInterface {}
pub struct ASceneCapture2D {
    pub capture_component2_d: UPtr<USceneCaptureComponent2D>,
}
pub struct USceneCaptureComponent2D {
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
    pub b_use_faux_ortho_view_pos: bool,
    pub b_enable_orthographic_tiling: bool,
    pub num_x_tiles: i32,
    pub num_y_tiles: i32,
    pub b_enable_clip_plane: bool,
    pub clip_plane_base: crate::bindings::core_u_object::FVector,
    pub clip_plane_normal: crate::bindings::core_u_object::FVector,
    pub b_render_in_main_renderer: bool,
    pub unlit_viewmode: ESceneCaptureUnlitViewmode,
    pub flags_3172: u8,
    pub flags_3173: u8,
    pub main_view_resolution_divisor: crate::bindings::core_u_object::FIntPoint,
    pub user_scene_texture_base_color: FName,
    pub user_scene_texture_normal: FName,
    pub user_scene_texture_scene_color: FName,
}
pub struct USceneCaptureComponentCube {
    pub texture_target: UPtr<UTextureRenderTargetCube>,
    pub b_capture_rotation: bool,
    pub post_process_settings: FPostProcessSettings,
    pub post_process_blend_weight: f32,
}
pub struct ASceneCaptureCube {
    pub capture_component_cube: UPtr<USceneCaptureComponentCube>,
}
pub struct UShadowMapTexture2D {
    pub shadowmap_flags: EShadowMapFlags,
}
pub struct USkeletalMeshSocket {
    pub socket_name: FName,
    pub bone_name: FName,
    pub relative_location: crate::bindings::core_u_object::FVector,
    pub relative_rotation: crate::bindings::core_u_object::FRotator,
    pub relative_scale: crate::bindings::core_u_object::FVector,
    pub b_force_always_animated: bool,
}
pub struct ASkyLight {
    pub light_component: UPtr<USkyLightComponent>,
    pub flags_1152: u8,
}
pub struct USMInstanceManager {}
pub struct ISMInstanceManager {}
pub struct USMInstanceManagerProvider {}
pub struct ISMInstanceManagerProvider {}
pub struct USoundAttenuationEditorSettings {
    pub b_enable_reverb_send: bool,
    pub b_enable_send_to_audio_link: bool,
}
pub struct USoundEffectPresetWidgetInterface {}
pub struct ISoundEffectPresetWidgetInterface {}
pub struct USoundEffectSourcePreset {}
pub struct USoundEffectSourcePresetChain {
    pub chain: TArray<FSourceEffectChainEntry>,
    pub flags_64: u8,
}
pub struct USoundSubmixWidgetInterface {}
pub struct ISoundSubmixWidgetInterface {}
pub struct ASphereReflectionCapture {
    pub draw_capture_radius: UPtr<UDrawSphereComponent>,
}
pub struct USphereReflectionCaptureComponent {
    pub influence_radius: f32,
    pub capture_distance_scale: f32,
    pub preview_influence_radius: UPtr<UDrawSphereComponent>,
}
pub struct UStaticMeshSocket {
    pub socket_name: FName,
    pub relative_location: crate::bindings::core_u_object::FVector,
    pub relative_rotation: crate::bindings::core_u_object::FRotator,
    pub relative_scale: crate::bindings::core_u_object::FVector,
    pub tag: FString,
    pub preview_static_mesh: UPtr<UStaticMesh>,
    pub b_socket_created_at_import: bool,
}
pub struct UStreamingWorldSubsystemInterface {}
pub struct IStreamingWorldSubsystemInterface {}
pub struct UTedsTypedElementBridgeInterface {}
pub struct ITedsTypedElementBridgeInterface {}
pub struct UTextPropertyTestObject {
    pub defaulted_text: FText,
    pub undefaulted_text: FText,
    pub transient_text: FText,
}
pub struct ATextRenderActor {
    pub text_render: UPtr<UTextRenderComponent>,
    pub sprite_component: UPtr<UBillboardComponent>,
}
pub struct UTextureEncodingProjectSettings {
    pub flags_104: u8,
    pub final_rdo_lambda: i8,
    pub final_effort_level: ETextureEncodeEffort,
    pub final_universal_tiling: ETextureUniversalTiling,
    pub flags_112: u8,
    pub fast_rdo_lambda: i8,
    pub fast_effort_level: ETextureEncodeEffort,
    pub fast_universal_tiling: ETextureUniversalTiling,
    pub cook_uses_speed: ETextureEncodeSpeed,
    pub editor_uses_speed: ETextureEncodeSpeed,
}
pub struct UTextureEncodingUserSettings {
    pub force_encode_speed: ETextureEncodeSpeedOverride,
}
pub struct UTimelineComponent {
    pub the_timeline: FTimeline,
    pub flags_440: u8,
}
pub struct UTransformProviderData {
    pub b_enabled: bool,
}
pub struct ATriggerBase {
    pub collision_component: UPtr<UShapeComponent>,
    pub sprite_component: UPtr<UBillboardComponent>,
}
pub struct ATriggerBox {}
pub struct ATriggerCapsule {}
pub struct ATriggerSphere {}
pub struct UTwitterIntegrationBase {}
pub struct UVectorField {
    pub bounds: crate::bindings::core_u_object::FBox,
    pub intensity: f32,
}
pub struct UVectorFieldAnimated {
    pub texture: UPtr<UTexture2D>,
    pub construction_op: EVectorFieldConstructionOp,
    pub volume_size_x: i32,
    pub volume_size_y: i32,
    pub volume_size_z: i32,
    pub sub_images_x: i32,
    pub sub_images_y: i32,
    pub frame_count: i32,
    pub frames_per_second: f32,
    pub flags_152: u8,
    pub noise_field: UPtr<UVectorFieldStatic>,
    pub noise_scale: f32,
    pub noise_max: f32,
}
pub struct UVectorFieldComponent {
    pub vector_field: UPtr<UVectorField>,
    pub intensity: f32,
    pub tightness: f32,
    pub flags_1520: u8,
}
pub struct UVectorFieldStatic {
    pub size_x: i32,
    pub size_y: i32,
    pub size_z: i32,
    pub b_allow_cpu_access: bool,
    pub source_file_path_deprecated: FString,
    pub asset_import_data: UPtr<UAssetImportData>,
}
pub struct UVisualLoggerDebugSnapshotInterface {}
pub struct IVisualLoggerDebugSnapshotInterface {}
pub struct UWindDirectionalSourceComponent {
    pub strength: f32,
    pub speed: f32,
    pub min_gust_amount: f32,
    pub max_gust_amount: f32,
    pub radius: f32,
    pub flags_676: u8,
}
pub struct UWorldPartitionEditorLoaderAdapter {}
pub struct UTexture2DArray {
    pub address_x: TextureAddress,
    pub address_y: TextureAddress,
    pub address_z: TextureAddress,
    pub source_textures: TArray<UPtr<UTexture2D>>,
    pub b_source_generated_from_source_textures_array: bool,
}
pub struct UMaterialExpressionMaterialSample {
    pub material_reference: UPtr<UMaterialInterface>,
}
pub struct UActorFolder {
    pub parent_folder_guid: crate::bindings::core_u_object::FGuid,
    pub folder_guid: crate::bindings::core_u_object::FGuid,
    pub folder_label: FString,
    pub b_folder_initially_expanded: bool,
    pub b_is_deleted: bool,
}
pub struct UActorPartitionSubsystem {}
pub struct UNullNavSysConfig {}
pub struct UAvoidanceManager {
    pub default_time_to_live: f32,
    pub lock_time_after_avoid: f32,
    pub lock_time_after_clean: f32,
    pub delta_time_to_predict: f32,
    pub artificial_radius_expansion: f32,
    pub test_height_difference_deprecated: f32,
    pub height_check_margin: f32,
}
pub struct AAmbientSound {
    pub audio_component: UPtr<UAudioComponent>,
}
pub struct UAnimationAsset {
    pub skeleton: UPtr<USkeleton>,
    pub meta_data: TArray<UPtr<UAnimMetaData>>,
    pub parent_asset: UPtr<UAnimationAsset>,
    pub children_assets: TArray<UPtr<UAnimationAsset>>,
    pub asset_mapping_table: UPtr<UAssetMappingTable>,
    pub asset_user_data: TArray<UPtr<UAssetUserData>>,
    pub thumbnail_info: UPtr<UThumbnailInfo>,
    pub preview_pose_asset: UPtr<UPoseAsset>,
    pub preview_skeletal_mesh: TSoftObjectPtr<USkeletalMesh>,
}
pub struct UBlendSpace {
    pub b_contains_rotation_offset_mesh_space_samples: bool,
    pub interpolation_param: FInterpolationParameter,
    pub analysis_properties: UPtr<UAnalysisProperties>,
    pub target_weight_interpolation_speed_per_sec: f32,
    pub b_target_weight_interpolation_ease_in_out: bool,
    pub b_allow_mesh_space_blending: bool,
    pub b_loop: bool,
    pub b_allow_marker_based_sync: bool,
    pub b_should_match_sync_phases: bool,
    pub preview_base_pose: UPtr<UAnimSequence>,
    pub anim_length: f32,
    pub notify_trigger_mode: ENotifyTriggerMode,
    pub b_interpolate_using_grid: bool,
    pub preferred_triangulation_direction: EPreferredTriangulationDirection,
    pub per_bone_blend_mode: EBlendSpacePerBoneBlendMode,
    pub manual_per_bone_overrides: TArray<FPerBoneInterpolation>,
    pub per_bone_blend_profile: FBlendSpaceBlendProfile,
    pub sample_index_with_markers: i32,
    pub sample_data: TArray<FBlendSample>,
    pub grid_samples: TArray<FEditorElement>,
    pub blend_space_data: FBlendSpaceData,
    pub blend_parameters: FBlendParameter,
    pub axis_to_scale_animation: EBlendSpaceAxis,
    pub dimension_indices: TArray<i32>,
}
pub struct UAimOffsetBlendSpace {}
pub struct UBlendSpace1D {
    pub b_display_editor_vertically_deprecated: bool,
    pub b_scale_animation: bool,
}
pub struct UAimOffsetBlendSpace1D {}
pub struct UAnimationSettings {
    pub compress_commandlet_version: i32,
    pub key_end_effectors_match_name_array: TArray<FString>,
    pub force_recompression: bool,
    pub b_force_below_threshold: bool,
    pub b_first_recompress_using_current_or_default: bool,
    pub b_raise_max_error_to_existing: bool,
    pub b_enable_performance_log: bool,
    pub b_strip_animation_data_on_dedicated_server: bool,
    pub b_tick_animation_on_skeletal_mesh_init: bool,
    pub bone_timecode_custom_attribute_name_settings: FTimecodeCustomAttributeNameSettings,
    pub bone_custom_attributes_names: TArray<FCustomAttributeSetting>,
    pub bone_names_with_custom_attributes: TArray<FString>,
    pub attribute_blend_modes: TMap<FName, ECustomAttributeBlendType>,
    pub default_attribute_blend_mode: ECustomAttributeBlendType,
    pub transform_attribute_names: TArray<FString>,
    pub user_defined_struct_attributes: TArray<
        TSoftObjectPtr<crate::bindings::core_u_object::UUserDefinedStruct>,
    >,
    pub mirror_find_replace_expressions: TArray<FMirrorFindReplaceExpression>,
    pub default_frame_rate: crate::bindings::core_u_object::FFrameRate,
    pub b_enforce_supported_frame_rates: bool,
}
pub struct UAnimBank {
    pub sequences: TArray<FAnimBankSequence>,
    pub asset: UPtr<USkinnedAsset>,
}
pub struct UAnimBankData {
    pub anim_bank_items: TArray<FAnimBankItem>,
}
pub struct UAnimBlueprintGeneratedStruct {}
pub struct UAnimBlueprintGeneratedClass {
    pub baked_state_machines: TArray<FBakedAnimationStateMachine>,
    pub target_skeleton: UPtr<USkeleton>,
    pub anim_notifies: TArray<FAnimNotifyEvent>,
    pub ordered_saved_pose_indices_map: TMap<FName, FCachedPoseIndices>,
    pub sync_group_names: TArray<FName>,
    pub evaluate_graph_exposed_inputs_deprecated: TArray<FExposedValueHandler>,
    pub graph_asset_player_information: TMap<FName, FGraphAssetPlayerInformation>,
    pub graph_blend_options: TMap<FName, FAnimGraphBlendOptions>,
    pub anim_node_data: TArray<FAnimNodeData>,
    pub node_type_map: TMap<
        UPtr<crate::bindings::core_u_object::UScriptStruct>,
        FAnimNodeStructData,
    >,
}
pub struct UAnimBoneCompressionCodec {
    pub description: FString,
}
pub struct UAnimBoneCompressionSettings {
    pub codecs: TArray<UPtr<UAnimBoneCompressionCodec>>,
    pub error_threshold: f32,
    pub b_force_below_threshold: bool,
}
pub struct UAnimClassInterface {}
pub struct IAnimClassInterface {}
pub struct UAnimSequenceBase {
    pub notifies: TArray<FAnimNotifyEvent>,
    pub sequence_length: f32,
    pub raw_curve_data: FRawCurveTracks,
    pub rate_scale: f32,
    pub b_loop: bool,
    pub anim_notify_tracks: TArray<FAnimNotifyTrack>,
    pub data_model: UPtr<UAnimDataModel>,
    pub data_model_interface: TScriptInterface<IAnimationDataModel>,
    pub controller: TScriptInterface<IAnimationDataController>,
}
pub struct UAnimCompositeBase {
    pub common_target_frame_rate: crate::bindings::core_u_object::FFrameRate,
}
pub struct UAnimComposite {
    pub animation_track: FAnimTrack,
    pub preview_base_pose: UPtr<UAnimSequence>,
}
pub struct UAnimCompress {
    pub flags_64: u8,
    pub translation_compression_format: AnimationCompressionFormat,
    pub rotation_compression_format: AnimationCompressionFormat,
    pub scale_compression_format: AnimationCompressionFormat,
}
pub struct UAnimCompress_BitwiseCompressOnly {}
pub struct UAnimCompress_LeastDestructive {}
pub struct UAnimCompress_RemoveLinearKeys {
    pub max_pos_diff: f32,
    pub max_angle_diff: f32,
    pub max_scale_diff: f32,
    pub max_effector_diff: f32,
    pub min_effector_diff: f32,
    pub effector_diff_socket: f32,
    pub parent_key_scale: f32,
    pub flags_100: u8,
}
pub struct UAnimCompress_PerTrackCompression {
    pub max_zeroing_threshold: f32,
    pub max_pos_diff_bitwise: f32,
    pub max_angle_diff_bitwise: f32,
    pub max_scale_diff_bitwise: f32,
    pub allowed_rotation_formats: TArray<AnimationCompressionFormat>,
    pub allowed_translation_formats: TArray<AnimationCompressionFormat>,
    pub allowed_scale_formats: TArray<AnimationCompressionFormat>,
    pub flags_168: u8,
    pub resampled_framerate: f32,
    pub min_keys_for_resampling: i32,
    pub flags_180: u8,
    pub track_height_bias: i32,
    pub parenting_divisor: f32,
    pub parenting_divisor_exponent: f32,
    pub flags_196: u8,
    pub rotation_error_source_ratio: f32,
    pub translation_error_source_ratio: f32,
    pub scale_error_source_ratio: f32,
    pub max_error_per_track_ratio: f32,
    pub perturbation_probe_size: f32,
}
pub struct UAnimCompress_RemoveEverySecondKey {
    pub min_keys: i32,
    pub flags_76: u8,
}
pub struct UAnimCompress_RemoveTrivialKeys {
    pub max_pos_diff: f32,
    pub max_angle_diff: f32,
    pub max_scale_diff: f32,
}
pub struct UAnimCurveCompressionCodec {}
pub struct UAnimCurveCompressionCodec_CompressedRichCurve {
    pub max_curve_error: f32,
    pub use_anim_sequence_sample_rate: bool,
    pub error_sample_rate: f32,
}
pub struct UAnimCurveCompressionCodec_UniformIndexable {}
pub struct UAnimCurveCompressionCodec_UniformlySampled {
    pub use_anim_sequence_sample_rate: bool,
    pub sample_rate: f32,
}
pub struct UAnimCurveCompressionSettings {
    pub codec: UPtr<UAnimCurveCompressionCodec>,
}
pub struct UAnimDataModel {
    pub bracket_counter: i32,
    pub modified_event_dynamic: FAnimDataModel_ModifiedEventDynamic,
    pub bone_animation_tracks: TArray<FBoneAnimationTrack>,
    pub play_length: f32,
    pub frame_rate: crate::bindings::core_u_object::FFrameRate,
    pub number_of_frames: i32,
    pub number_of_keys: i32,
    pub curve_data: FAnimationCurveData,
    pub animated_bone_attributes: TArray<FAnimatedBoneAttribute>,
    pub b_populated: bool,
}
pub struct UAnimationDataController {}
pub struct IAnimationDataController {}
pub struct UAnimationDataModel {}
pub struct IAnimationDataModel {}
pub struct UAnimMetaData {}
pub struct UAnimMontage {
    pub blend_mode_in: EMontageBlendMode,
    pub blend_mode_out: EMontageBlendMode,
    pub blend_in: FAlphaBlend,
    pub blend_in_time_deprecated: f32,
    pub blend_out: FAlphaBlend,
    pub blend_out_time_deprecated: f32,
    pub blend_out_trigger_time: f32,
    pub sync_group: FName,
    pub sync_slot_index: i32,
    pub marker_data: FMarkerSyncData,
    pub composite_sections: TArray<FCompositeSection>,
    pub slot_anim_tracks: TArray<FSlotAnimationTrack>,
    pub branching_points_deprecated: TArray<FBranchingPoint>,
    pub b_enable_root_motion_translation: bool,
    pub b_enable_root_motion_rotation: bool,
    pub b_enable_auto_blend_out: bool,
    pub blend_profile_in: UPtr<UBlendProfile>,
    pub blend_profile_out: UPtr<UBlendProfile>,
    pub root_motion_root_lock: ERootMotionRootLock,
    pub preview_base_pose: UPtr<UAnimSequence>,
    pub branching_point_markers: TArray<FBranchingPointMarker>,
    pub branching_point_state_notify_indices: TArray<i32>,
    pub time_stretch_curve: FTimeStretchCurve,
    pub time_stretch_curve_name: FName,
}
pub struct UAnimNotifyLibrary {}
pub struct UAnimNotifyMirrorInspectionLibrary {}
pub struct UAnimNotifyStateMachineInspectionLibrary {}
pub struct UAnimNotifyState_DisableRootMotion {}
pub struct UAnimNotifyState_TimedParticleEffect {
    pub ps_template: UPtr<UParticleSystem>,
    pub socket_name: FName,
    pub location_offset: crate::bindings::core_u_object::FVector,
    pub rotation_offset: crate::bindings::core_u_object::FRotator,
    pub b_destroy_at_end: bool,
    pub previous_ps_templates: TArray<UPtr<UParticleSystem>>,
    pub previous_socket_names: TArray<FName>,
}
pub struct UAnimNotifyState_Trail {
    pub ps_template: UPtr<UParticleSystem>,
    pub first_socket_name: FName,
    pub second_socket_name: FName,
    pub width_scale_mode: ETrailWidthMode,
    pub width_scale_curve: FName,
    pub flags_104: u8,
}
pub struct UAnimNotify_PauseClothingSimulation {}
pub struct UAnimNotify_PlayParticleEffect {
    pub ps_template: UPtr<UParticleSystem>,
    pub location_offset: crate::bindings::core_u_object::FVector,
    pub rotation_offset: crate::bindings::core_u_object::FRotator,
    pub scale: crate::bindings::core_u_object::FVector,
    pub flags_176: u8,
    pub socket_name: FName,
}
pub struct UAnimNotify_PlaySound {
    pub sound: UPtr<USoundBase>,
    pub volume_multiplier: f32,
    pub pitch_multiplier: f32,
    pub flags_80: u8,
    pub attach_name: FName,
}
pub struct UAnimNotify_ResetClothingSimulation {}
pub struct UAnimNotify_ResetDynamics {}
pub struct UAnimNotify_ResumeClothingSimulation {}
pub struct UAnimSequence {
    pub import_file_framerate: f32,
    pub import_resample_framerate: i32,
    pub num_frames: i32,
    pub number_of_keys: i32,
    pub sampling_frame_rate: crate::bindings::core_u_object::FFrameRate,
    pub raw_data_guid: crate::bindings::core_u_object::FGuid,
    pub animation_track_names: TArray<FName>,
    pub b_allow_frame_stripping: bool,
    pub compression_error_threshold_scale: f32,
    pub bone_compression_settings: UPtr<UAnimBoneCompressionSettings>,
    pub curve_compression_settings: UPtr<UAnimCurveCompressionSettings>,
    pub variable_frame_stripping_settings: UPtr<UVariableFrameStrippingSettings>,
    pub additive_anim_type: EAdditiveAnimationType,
    pub ref_pose_type: EAdditiveBasePoseType,
    pub ref_frame_index: i32,
    pub ref_pose_seq: UPtr<UAnimSequence>,
    pub retarget_source: FName,
    pub retarget_source_asset: TSoftObjectPtr<USkeletalMesh>,
    pub retarget_source_asset_reference_pose: TArray<
        crate::bindings::core_u_object::FTransform,
    >,
    pub interpolation: EAnimInterpolationType,
    pub b_enable_root_motion: bool,
    pub root_motion_root_lock: ERootMotionRootLock,
    pub b_force_root_lock: bool,
    pub b_use_normalized_root_motion_scale: bool,
    pub b_root_motion_settings_copied_from_montage: bool,
    pub compress_commandlet_version: i32,
    pub flags_948: u8,
    pub asset_import_data: UPtr<UAssetImportData>,
    pub source_file_path_deprecated: FString,
    pub source_file_timestamp_deprecated: FString,
    pub strip_anim_data_on_dedicated_server: EStripAnimDataOnDedicatedServerSettings,
    pub authored_sync_markers: TArray<FAnimSyncMarker>,
    pub platform_target_frame_rate: crate::bindings::core_u_object::FPerPlatformFrameRate,
    pub target_frame_rate: crate::bindings::core_u_object::FFrameRate,
    pub number_of_sampled_keys: i32,
    pub number_of_sampled_frames: i32,
    pub per_bone_custom_attribute_data: TArray<FCustomAttributePerBoneData>,
    pub attribute_curves: TMap<FAnimationAttributeIdentifier, FAttributeCurve>,
}
pub struct UAnimSet {
    pub flags_48: u8,
    pub track_bone_names: TArray<FName>,
    pub sequences: TArray<UPtr<UAnimSequence>>,
    pub linkup_cache: TArray<FAnimSetMeshLinkup>,
    pub bone_use_anim_translation: TArray<u8>,
    pub force_use_mesh_translation: TArray<u8>,
    pub use_translation_bone_names: TArray<FName>,
    pub force_mesh_translation_bone_names: TArray<FName>,
    pub preview_skel_mesh_name: FName,
    pub best_ratio_skel_mesh_name: FName,
}
pub struct UAnimStateMachineTypes {}
pub struct UAnimStreamable {
    pub number_of_keys: i32,
    pub interpolation: EAnimInterpolationType,
    pub retarget_source: FName,
    pub sampling_frame_rate: crate::bindings::core_u_object::FFrameRate,
    pub source_sequence: UPtr<UAnimSequence>,
    pub raw_data_guid: crate::bindings::core_u_object::FGuid,
    pub num_frames: i32,
    pub raw_animation_data: TArray<FRawAnimSequenceTrack>,
    pub track_to_skeleton_map_table: TArray<FTrackToSkeletonMap>,
    pub animation_track_names: TArray<FName>,
    pub bone_compression_settings: UPtr<UAnimBoneCompressionSettings>,
    pub curve_compression_settings: UPtr<UAnimCurveCompressionSettings>,
    pub variable_frame_stripping_settings: UPtr<UVariableFrameStrippingSettings>,
    pub b_enable_root_motion: bool,
    pub root_motion_root_lock: ERootMotionRootLock,
    pub b_force_root_lock: bool,
    pub b_use_normalized_root_motion_scale: bool,
}
pub struct URawAnimSequenceTrackExtensions {}
pub struct UAssetMappingTable {
    pub mapped_assets: TArray<FAssetMapping>,
}
pub struct UAnimationAttributeIdentifierExtensions {}
pub struct UBlendProfileProviderInterface {}
pub struct IBlendProfileProviderInterface {}
pub struct UBlendProfile {
    pub owning_skeleton: UPtr<USkeleton>,
    pub profile_entries: TArray<FBlendProfileBoneEntry>,
    pub mode: EBlendProfileMode,
}
pub struct UAnalysisProperties {
    pub function: FString,
}
pub struct UBoneMaskFilter {
    pub blend_poses: TArray<FInputBlendPose>,
}
pub struct UCachedAnimDataLibrary {}
pub struct UAnimationCurveIdentifierExtensions {}
pub struct UCurveSourceInterface {}
pub struct ICurveSourceInterface {}
pub struct UMeshDeformerCollection {
    pub description: FString,
    pub mesh_deformers: TArray<TSoftObjectPtr<UMeshDeformer>>,
    pub mesh_deformer_collections: TArray<UPtr<UMeshDeformerCollection>>,
}
pub struct UDataTable {
    pub row_struct: UPtr<crate::bindings::core_u_object::UScriptStruct>,
    pub flags_136: u8,
    pub import_key_field: FString,
    pub asset_import_data: UPtr<UAssetImportData>,
    pub import_path_deprecated: FString,
    pub row_struct_name_deprecated: FName,
    pub row_struct_path_name: crate::bindings::core_u_object::FTopLevelAssetPath,
    pub rows_serialized_with_tags: TArray<u8>,
    pub temporarily_referenced_objects: TSet<
        UPtr<crate::bindings::core_u_object::UObject>,
    >,
}
pub struct UMirrorDataTable {
    pub mirror_find_replace_expressions: TArray<FMirrorFindReplaceExpression>,
    pub mirror_axis: crate::bindings::core_u_object::EAxis,
    pub b_mirror_root_motion: bool,
    pub skeleton: UPtr<USkeleton>,
}
pub struct UNodeMappingContainer {
    pub source_items: TMap<FName, FNodeItem>,
    pub target_items: TMap<FName, FNodeItem>,
    pub source_to_target: TMap<FName, FName>,
    pub source_asset: TSoftObjectPtr<crate::bindings::core_u_object::UObject>,
    pub target_asset: TSoftObjectPtr<crate::bindings::core_u_object::UObject>,
}
pub struct UNodeMappingProviderInterface {}
pub struct INodeMappingProviderInterface {}
pub struct UPoseAsset {
    pub pose_container: FPoseDataContainer,
    pub b_additive_pose: bool,
    pub base_pose_index: i32,
    pub retarget_source: FName,
    pub retarget_source_asset: TSoftObjectPtr<USkeletalMesh>,
    pub retarget_source_asset_reference_pose: TArray<
        crate::bindings::core_u_object::FTransform,
    >,
    pub source_animation: UPtr<UAnimSequence>,
    pub source_animation_raw_data_guid: crate::bindings::core_u_object::FGuid,
}
pub struct UPreviewCollectionInterface {}
pub struct IPreviewCollectionInterface {}
pub struct UPreviewMeshCollection {
    pub skeleton: UPtr<USkeleton>,
    pub skeletal_meshes: TArray<FPreviewMeshCollectionEntry>,
}
pub struct USkeleton {
    pub bone_tree: TArray<FBoneNode>,
    pub ref_local_poses_deprecated: TArray<crate::bindings::core_u_object::FTransform>,
    pub preview_forward_axis: crate::bindings::core_u_object::EAxis,
    pub virtual_bone_guid: crate::bindings::core_u_object::FGuid,
    pub virtual_bones: TArray<FVirtualBone>,
    pub compatible_skeletons: TArray<TSoftObjectPtr<USkeleton>>,
    pub b_use_retarget_modes_from_compatible_skeleton: bool,
    pub sockets: TArray<UPtr<USkeletalMeshSocket>>,
    pub smart_names_deprecated: FSmartNameContainer,
    pub blend_profiles: TArray<UPtr<UBlendProfile>>,
    pub slot_groups: TArray<FAnimSlotGroup>,
    pub preview_skeletal_mesh: TSoftObjectPtr<USkeletalMesh>,
    pub additional_preview_skeletal_meshes: TSoftObjectPtr<UDataAsset>,
    pub animation_notifies: TArray<FName>,
    pub preview_attached_asset_container: FPreviewAssetAttachContainer,
    pub asset_user_data: TArray<UPtr<UAssetUserData>>,
    pub asset_user_data_editor_only: TArray<UPtr<UAssetUserData>>,
}
pub struct UTransformTrajectoryBlueprintLibrary {}
pub struct UVariableFrameStrippingSettings {
    pub use_variable_frame_stripping: crate::bindings::core_u_object::FPerPlatformBool,
    pub frame_stripping_rate: crate::bindings::core_u_object::FPerPlatformInt,
}
pub struct UAnimBlueprint {
    pub target_skeleton: UPtr<USkeleton>,
    pub groups: TArray<FAnimGroupInfo>,
    pub b_is_template: bool,
    pub b_use_multi_threaded_animation_update: bool,
    pub b_warn_about_blueprint_usage: bool,
    pub flags_1467: u8,
    pub parent_asset_overrides: TArray<FAnimParentNodeAssetOverride>,
    pub pose_watch_folders: TArray<UPtr<UPoseWatchFolder>>,
    pub pose_watches: TArray<UPtr<UPoseWatch>>,
    pub preview_skeletal_mesh: TSoftObjectPtr<USkeletalMesh>,
    pub preview_animation_blueprint: TSoftObjectPtr<UAnimBlueprint>,
    pub default_binding_class: TSubclassOf<crate::bindings::core_u_object::UObject>,
    pub preview_animation_blueprint_application_method: EPreviewAnimationBlueprintApplicationMethod,
    pub preview_animation_blueprint_tag: FName,
}
pub struct UAssetManager {
    pub object_reference_list: TArray<UPtr<crate::bindings::core_u_object::UObject>>,
    pub b_is_global_async_scan_environment: bool,
    pub b_should_guess_type_and_name: bool,
    pub b_should_use_synchronous_load: bool,
    pub b_is_loading_from_pak_files: bool,
    pub b_should_acquire_missing_chunks_on_load: bool,
    pub b_only_cook_production_assets: bool,
    pub num_bulk_scan_requests: i32,
    pub b_is_primary_asset_directory_current: bool,
    pub b_is_management_database_current: bool,
    pub b_update_management_database_after_scan: bool,
    pub b_include_only_on_disk_assets: bool,
    pub b_has_completed_initial_scan: bool,
    pub number_of_spawned_notifications: i32,
}
pub struct UAsyncActionLoadPrimaryAssetBase {}
pub struct UAsyncActionLoadPrimaryAsset {
    pub completed: FAsyncActionLoadPrimaryAsset_Completed,
}
pub struct UAsyncActionLoadPrimaryAssetClass {
    pub completed: FAsyncActionLoadPrimaryAssetClass_Completed,
}
pub struct UAsyncActionLoadPrimaryAssetList {
    pub completed: FAsyncActionLoadPrimaryAssetList_Completed,
}
pub struct UAsyncActionLoadPrimaryAssetClassList {
    pub completed: FAsyncActionLoadPrimaryAssetClassList_Completed,
}
pub struct UAsyncActionChangePrimaryAssetBundles {
    pub completed: FAsyncActionChangePrimaryAssetBundles_Completed,
}
pub struct AAtmosphericFog {
    pub atmospheric_fog_component: UPtr<UAtmosphericFogComponent>,
    pub arrow_component: UPtr<UArrowComponent>,
}
pub struct USkyAtmosphereComponent {
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
    pub b_static_lighting_built_guid: crate::bindings::core_u_object::FGuid,
}
pub struct UAtmosphericFogComponent {}
pub struct UAudioBus {
    pub audio_bus_channels: EAudioBusChannels,
}
pub struct UAudioSettings {
    pub default_sound_class_name: crate::bindings::core_u_object::FSoftObjectPath,
    pub default_media_sound_class_name: crate::bindings::core_u_object::FSoftObjectPath,
    pub default_sound_concurrency_name: crate::bindings::core_u_object::FSoftObjectPath,
    pub default_base_sound_mix: crate::bindings::core_u_object::FSoftObjectPath,
    pub voi_p_sound_class: crate::bindings::core_u_object::FSoftObjectPath,
    pub master_submix: crate::bindings::core_u_object::FSoftObjectPath,
    pub base_default_submix: crate::bindings::core_u_object::FSoftObjectPath,
    pub reverb_submix: crate::bindings::core_u_object::FSoftObjectPath,
    pub eq_submix: crate::bindings::core_u_object::FSoftObjectPath,
    pub voi_p_sample_rate: EVoiceSampleRate,
    pub default_audio_compression_type: EDefaultAudioCompressionType,
    pub default_occlusion_check_interval: crate::bindings::core_u_object::FPerPlatformFloat,
    pub default_compression_quality_deprecated: i32,
    pub default_reverb_send_level_deprecated: f32,
    pub maximum_concurrent_streams: i32,
    pub global_min_pitch_scale: f32,
    pub global_max_pitch_scale: f32,
    pub quality_levels: TArray<FAudioQualitySettings>,
    pub flags_600: u8,
    pub num_stopping_sources: u32,
    pub panning_method: EPanningMethod,
    pub mono_channel_upmix_method: EMonoChannelUpmixMethod,
    pub dialogue_filename_format: FString,
    pub debug_sounds: TArray<FSoundDebugEntry>,
    pub default_audio_buses: TArray<FDefaultAudioBusSettings>,
    pub b_enable_legacy_asset_types: bool,
    pub default_sound_class: UPtr<USoundClass>,
    pub default_media_sound_class: UPtr<USoundClass>,
    pub default_sound_concurrency: UPtr<USoundConcurrency>,
}
pub struct AAudioVolume {
    pub priority: f32,
    pub flags_1212: u8,
    pub settings: FReverbSettings,
    pub ambient_zone_settings: FInteriorSettings,
    pub submix_send_settings: TArray<FAudioVolumeSubmixSendSettings>,
    pub submix_override_settings: TArray<FAudioVolumeSubmixOverrideSettings>,
}
pub struct UActorSoundParameterInterface {}
pub struct IActorSoundParameterInterface {}
pub struct UAudioWidgetSubsystem {}
pub struct USoundParameterControllerInterface {}
pub struct ISoundParameterControllerInterface {}
pub struct UAudioParameterConversionStatics {}
pub struct ABlockingVolume {}
pub struct UDEPRECATED_Breakpoint {}
pub struct UBlueprintInstancedStructLibrary {}
pub struct UBlueprintMapLibrary {}
pub struct UBlueprintSetLibrary {}
pub struct UBookMark {
    pub location: crate::bindings::core_u_object::FVector,
    pub rotation: crate::bindings::core_u_object::FRotator,
    pub hidden_levels: TArray<FString>,
}
pub struct ABrushShape {}
pub struct ACameraBlockingVolume {}
pub struct UCameraProxyMeshComponent {}
pub struct UCameraLensEffectInterface {}
pub struct ICameraLensEffectInterface {}
pub struct UCameraLensEffectInterfaceClassSupportLibrary {}
pub struct UCameraModifier {
    pub flags_48: u8,
    pub priority: u8,
    pub camera_owner: UPtr<APlayerCameraManager>,
    pub alpha_in_time: f32,
    pub alpha_out_time: f32,
    pub alpha: f32,
}
pub struct UCameraModifier_CameraShake {
    pub active_shakes: TArray<FActiveCameraShakeInfo>,
    pub expired_pooled_shakes_map: TMap<
        TSubclassOf<UCameraShakeBase>,
        FPooledCameraShakes,
    >,
    pub split_screen_shake_scale: f32,
}
pub struct UCameraShakePattern {}
pub struct ACameraShakeSourceActor {
    pub camera_shake_source_component: UPtr<UCameraShakeSourceComponent>,
}
pub struct UCameraShakeSourceComponent {
    pub attenuation: ECameraShakeAttenuation,
    pub inner_attenuation_radius: f32,
    pub outer_attenuation_radius: f32,
    pub camera_shake: TSubclassOf<UCameraShakeBase>,
    pub b_auto_start: bool,
    pub editor_sprite_texture: UPtr<UTexture2D>,
    pub editor_sprite_texture_scale: f32,
}
pub struct UTextureRenderTarget {
    pub target_gamma: f32,
}
pub struct UTextureRenderTarget2D {
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
    pub override_format: crate::bindings::core_u_object::EPixelFormat,
}
pub struct UCanvasRenderTarget2D {
    pub on_canvas_render_target_update: FCanvasRenderTarget2D_OnCanvasRenderTargetUpdate,
    pub world: TWeakObjectPtr<UWorld>,
    pub sample_count: ETextureRenderTargetSampleCount,
    pub b_should_clear_render_target_on_receive_update: bool,
}
pub struct APlayerController {
    pub player: UPtr<UPlayer>,
    pub acknowledged_pawn: UPtr<APawn>,
    pub my_hud: UPtr<AHUD>,
    pub player_camera_manager: UPtr<APlayerCameraManager>,
    pub player_camera_manager_class: TSubclassOf<APlayerCameraManager>,
    pub b_auto_manage_active_camera_target: bool,
    pub target_view_rotation: crate::bindings::core_u_object::FRotator,
    pub smooth_target_view_rotation_speed: f32,
    pub hidden_actors: TArray<UPtr<AActor>>,
    pub hidden_primitive_components: TArray<TWeakObjectPtr<UPrimitiveComponent>>,
    pub last_spectator_state_synch_time: f32,
    pub last_spectator_sync_location: crate::bindings::core_u_object::FVector,
    pub last_spectator_sync_rotation: crate::bindings::core_u_object::FRotator,
    pub client_cap: i32,
    pub cheat_manager: UPtr<UCheatManager>,
    pub cheat_class: TSubclassOf<UCheatManager>,
    pub player_input: UPtr<UPlayerInput>,
    pub active_force_feedback_effects: TArray<FActiveForceFeedbackEffect>,
    pub flags_1712: u8,
    pub net_player_index: u8,
    pub pending_swap_connection: UPtr<UNetConnection>,
    pub net_connection: UPtr<UNetConnection>,
    pub input_yaw_scale_deprecated: f32,
    pub input_pitch_scale_deprecated: f32,
    pub input_roll_scale_deprecated: f32,
    pub flags_1860: u8,
    pub flags_1861: u8,
    pub streaming_source_priority: EStreamingSourcePriority,
    pub streaming_source_debug_color: crate::bindings::core_u_object::FColor,
    pub streaming_source_shapes: TArray<FStreamingSourceShape>,
    pub force_feedback_scale: f32,
    pub click_event_keys: TArray<crate::bindings::input_core::FKey>,
    pub default_mouse_cursor: crate::bindings::core_u_object::EMouseCursor,
    pub current_mouse_cursor: crate::bindings::core_u_object::EMouseCursor,
    pub default_click_trace_channel: ECollisionChannel,
    pub current_click_trace_channel: ECollisionChannel,
    pub hit_result_trace_distance: f32,
    pub seamless_travel_count: u16,
    pub last_completed_seamless_travel_count: u16,
    pub inactive_state_input_component: UPtr<UInputComponent>,
    pub flags_2064: u8,
    pub current_touch_interface: UPtr<UTouchInterface>,
    pub override_player_input_class: TSubclassOf<UPlayerInput>,
    pub spectator_pawn: UPtr<ASpectatorPawn>,
    pub b_is_local_player_controller: bool,
    pub spawn_location: crate::bindings::core_u_object::FVector,
    pub cached_connection_player_id: FUniqueNetIdRepl,
    pub client_handshake_id: u32,
}
pub struct UCheatManager {
    pub debug_camera_controller_ref: UPtr<ADebugCameraController>,
    pub debug_camera_controller_class: TSubclassOf<ADebugCameraController>,
    pub cheat_manager_extensions: TArray<UPtr<UCheatManagerExtension>>,
}
pub struct UCheatManagerExtension {}
pub struct UCollisionProfile {
    pub profiles: TArray<FCollisionResponseTemplate>,
    pub default_channel_responses: TArray<FCustomChannelSetup>,
    pub edit_profiles: TArray<FCustomProfile>,
    pub profile_redirects: TArray<FRedirector>,
    pub collision_channel_redirects: TArray<FRedirector>,
}
pub struct UPluginCommandlet {}
pub struct USmokeTestCommandlet {}
pub struct UComponentDelegateBinding {
    pub component_delegate_bindings: TArray<FBlueprintComponentDelegateBinding>,
}
pub struct UApplicationLifecycleComponent {
    pub application_will_deactivate_delegate: FApplicationLifecycleComponent_ApplicationWillDeactivateDelegate,
    pub application_has_reactivated_delegate: FApplicationLifecycleComponent_ApplicationHasReactivatedDelegate,
    pub application_will_enter_background_delegate: FApplicationLifecycleComponent_ApplicationWillEnterBackgroundDelegate,
    pub application_has_entered_foreground_delegate: FApplicationLifecycleComponent_ApplicationHasEnteredForegroundDelegate,
    pub application_will_terminate_delegate: FApplicationLifecycleComponent_ApplicationWillTerminateDelegate,
    pub application_should_unload_resources_delegate: FApplicationLifecycleComponent_ApplicationShouldUnloadResourcesDelegate,
    pub application_received_startup_arguments_delegate: FApplicationLifecycleComponent_ApplicationReceivedStartupArgumentsDelegate,
    pub on_temperature_change_delegate: FApplicationLifecycleComponent_OnTemperatureChangeDelegate,
    pub on_low_power_mode_delegate: FApplicationLifecycleComponent_OnLowPowerModeDelegate,
}
pub struct UArrowComponent {
    pub arrow_color: crate::bindings::core_u_object::FColor,
    pub arrow_size: f32,
    pub arrow_length: f32,
    pub screen_size: f32,
    pub flags_1520: u8,
    pub sprite_category_name_deprecated: FName,
    pub sprite_info: FSpriteCategoryInfo,
    pub flags_1584: u8,
    pub b_use_in_editor_scaling: bool,
}
pub struct UInitialActiveSoundParams {
    pub audio_params: TArray<crate::bindings::audio_extensions::FAudioParameter>,
}
pub struct UAudioComponent {
    pub sound: UPtr<USoundBase>,
    pub default_parameters: TArray<crate::bindings::audio_extensions::FAudioParameter>,
    pub instance_parameters: TArray<crate::bindings::audio_extensions::FAudioParameter>,
    pub sound_class_override: UPtr<USoundClass>,
    pub flags_1288: u8,
    pub flags_1289: u8,
    pub flags_1291: u8,
    pub audio_component_user_id: FName,
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
    pub volume_weighted_priority_scale_deprecated: f32,
    pub high_frequency_gain_multiplier_deprecated: f32,
    pub pitch_multiplier: f32,
    pub low_pass_filter_frequency: f32,
    pub high_pass_filter_frequency: f32,
    pub attenuation_settings: UPtr<USoundAttenuation>,
    pub attenuation_overrides: FSoundAttenuationSettings,
    pub concurrency_settings_deprecated: UPtr<USoundConcurrency>,
    pub concurrency_set: TSet<UPtr<USoundConcurrency>>,
    pub occlusion_check_interval: f32,
    pub auto_attach_location_rule: EAttachmentRule,
    pub auto_attach_rotation_rule: EAttachmentRule,
    pub auto_attach_scale_rule: EAttachmentRule,
    pub modulation_routing: FSoundModulationDefaultRoutingSettings,
    pub on_audio_play_state_changed: FAudioComponent_OnAudioPlayStateChanged,
    pub on_audio_virtualization_changed: FAudioComponent_OnAudioVirtualizationChanged,
    pub on_audio_finished: FAudioComponent_OnAudioFinished,
    pub on_audio_playback_percent: FAudioComponent_OnAudioPlaybackPercent,
    pub on_audio_single_envelope_value: FAudioComponent_OnAudioSingleEnvelopeValue,
    pub on_audio_multi_envelope_value: FAudioComponent_OnAudioMultiEnvelopeValue,
    pub on_queue_subtitles: FAudioComponent_OnQueueSubtitles,
    pub auto_attach_parent: TWeakObjectPtr<USceneComponent>,
    pub auto_attach_socket_name: FName,
}
pub struct UBillboardComponent {
    pub sprite: UPtr<UTexture2D>,
    pub flags_1512: u8,
    pub screen_size: f32,
    pub u: f32,
    pub ul: f32,
    pub v: f32,
    pub vl: f32,
    pub opacity_mask_ref_val: f32,
    pub sprite_category_name_deprecated: FName,
    pub sprite_info: FSpriteCategoryInfo,
    pub b_use_in_editor_scaling: bool,
    pub b_show_locked_location: bool,
}
pub struct UBoundsCopyComponent {
    pub bounds_source_actor: TSoftObjectPtr<AActor>,
    pub b_use_colliding_components_for_source_bounds: bool,
    pub b_keep_own_bounds_scale: bool,
    pub b_use_colliding_components_for_own_bounds: bool,
    pub post_transform: crate::bindings::core_u_object::FTransform,
    pub b_copy_x_bounds: bool,
    pub b_copy_y_bounds: bool,
    pub b_copy_z_bounds: bool,
}
pub struct UShapeComponent {
    pub shape_body_setup: UPtr<UBodySetup>,
    pub area_class: TSubclassOf<UNavAreaBase>,
    pub shape_color: crate::bindings::core_u_object::FColor,
    pub flags_1524: u8,
    pub area_class_override: TSubclassOf<UNavAreaBase>,
    pub line_thickness: f32,
}
pub struct UBoxComponent {
    pub box_extent: crate::bindings::core_u_object::FVector,
}
pub struct UBrushComponent {
    pub brush: UPtr<UModel>,
    pub brush_body_setup: UPtr<UBodySetup>,
    pub pre_pivot_deprecated: crate::bindings::core_u_object::FVector,
}
pub struct UCapsuleComponent {
    pub capsule_half_height: f32,
    pub capsule_radius: f32,
    pub capsule_height_deprecated: f32,
}
pub struct UMovementComponent {
    pub updated_component: UPtr<USceneComponent>,
    pub updated_primitive: UPtr<UPrimitiveComponent>,
    pub velocity: crate::bindings::core_u_object::FVector,
    pub plane_constraint_normal: crate::bindings::core_u_object::FVector,
    pub plane_constraint_origin: crate::bindings::core_u_object::FVector,
    pub flags_336: u8,
    pub plane_constraint_axis_setting: EPlaneConstraintAxisSetting,
}
pub struct UNavMovementComponent {
    pub fixed_path_braking_distance_deprecated: f32,
    pub flags_356: u8,
    pub nav_movement_properties: FNavMovementProperties,
    pub nav_agent_props: FNavAgentProperties,
    pub movement_state: FMovementProperties,
    pub path_following_comp: UPtr<crate::bindings::core_u_object::UObject>,
}
pub struct UPawnMovementComponent {
    pub pawn_owner: UPtr<APawn>,
}
pub struct UCharacterMovementComponent {
    pub character_owner: UPtr<ACharacter>,
    pub gravity_scale: f32,
    pub max_step_height: f32,
    pub jump_z_velocity: f32,
    pub jump_off_jump_z_factor: f32,
    pub walkable_floor_angle: f32,
    pub walkable_floor_z: f32,
    pub gravity_direction: crate::bindings::core_u_object::FVector,
    pub world_to_gravity_transform: crate::bindings::core_u_object::FQuat,
    pub gravity_to_world_transform: crate::bindings::core_u_object::FQuat,
    pub movement_mode: EMovementMode,
    pub custom_movement_mode: u8,
    pub network_smoothing_mode: ENetworkSmoothingMode,
    pub ground_friction: f32,
    pub old_base_quat: crate::bindings::core_u_object::FQuat,
    pub old_base_location: crate::bindings::core_u_object::FVector,
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
    pub deferred_updated_move_component: UPtr<USceneComponent>,
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
    pub flags_868: u8,
    pub crouched_speed_multiplier_deprecated: f32,
    pub upper_impact_normal_scale_deprecated: f32,
    pub acceleration: crate::bindings::core_u_object::FVector,
    pub last_update_rotation: crate::bindings::core_u_object::FQuat,
    pub last_update_location: crate::bindings::core_u_object::FVector,
    pub last_update_velocity: crate::bindings::core_u_object::FVector,
    pub server_last_transform_update_time_stamp: f32,
    pub server_last_client_good_move_ack_time: f32,
    pub server_last_client_adjustment_time: f32,
    pub pending_impulse_to_apply: crate::bindings::core_u_object::FVector,
    pub pending_force_to_apply: crate::bindings::core_u_object::FVector,
    pub analog_input_modifier: f32,
    pub max_simulation_time_step: f32,
    pub max_simulation_iterations: i32,
    pub max_jump_apex_attempts_per_simulation: i32,
    pub max_depenetration_with_geometry: f32,
    pub max_depenetration_with_geometry_as_proxy: f32,
    pub max_depenetration_with_pawn: f32,
    pub max_depenetration_with_pawn_as_proxy: f32,
    pub network_simulated_smooth_location_time: f32,
    pub network_simulated_smooth_rotation_time: f32,
    pub listen_server_network_simulated_smooth_location_time: f32,
    pub listen_server_network_simulated_smooth_rotation_time: f32,
    pub net_proxy_shrink_radius: f32,
    pub net_proxy_shrink_half_height: f32,
    pub network_max_smooth_update_distance: f32,
    pub network_no_smooth_update_distance: f32,
    pub network_min_time_between_client_ack_good_moves: f32,
    pub network_min_time_between_client_adjustments: f32,
    pub network_min_time_between_client_adjustments_large_correction: f32,
    pub network_large_client_correction_distance: f32,
    pub ledge_check_threshold: f32,
    pub jump_out_of_water_pitch: f32,
    pub current_floor: FFindFloorResult,
    pub default_land_movement_mode: EMovementMode,
    pub default_water_movement_mode: EMovementMode,
    pub ground_movement_mode: EMovementMode,
    pub flags_1452: u8,
    pub flags_1453: u8,
    pub flags_1454: u8,
    pub flags_1455: u8,
    pub former_base_velocity_decay_half_life: f32,
    pub flags_1460: u8,
    pub avoidance_consideration_radius: f32,
    pub requested_velocity: crate::bindings::core_u_object::FVector,
    pub last_update_requested_velocity: crate::bindings::core_u_object::FVector,
    pub avoidance_uid: i32,
    pub avoidance_group: FNavAvoidanceMask,
    pub groups_to_avoid: FNavAvoidanceMask,
    pub groups_to_ignore: FNavAvoidanceMask,
    pub avoidance_weight: f32,
    pub pending_launch_velocity: crate::bindings::core_u_object::FVector,
    pub nav_mesh_projection_interval: f32,
    pub nav_mesh_projection_timer: f32,
    pub nav_mesh_projection_interp_speed: f32,
    pub nav_mesh_projection_height_scale_up: f32,
    pub nav_mesh_projection_height_scale_down: f32,
    pub nav_walking_floor_dist_tolerance: f32,
    pub b_based_movement_ignore_physics_base: bool,
    pub b_base_on_attachment_root: bool,
    pub b_stay_based_in_air: bool,
    pub stay_based_in_air_height: f32,
    pub post_physics_tick_function: FCharacterMovementComponentPostPhysicsTickFunction,
    pub min_time_between_time_stamp_resets: f32,
    pub current_root_motion: FRootMotionSourceGroup,
    pub server_correction_root_motion: FRootMotionSourceGroup,
    pub root_motion_params: FRootMotionMovementParams,
    pub anim_root_motion_velocity: crate::bindings::core_u_object::FVector,
}
pub struct UDecalComponent {
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
}
pub struct ULightComponent {
    pub temperature: f32,
    pub max_draw_distance: f32,
    pub max_distance_fade_range: f32,
    pub flags_772: u8,
    pub shadow_map_channel_deprecated: i32,
    pub preview_shadow_map_channel: i32,
    pub min_roughness_deprecated: f32,
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
    pub mega_lights_shadow_method: EMegaLightsShadowMethod,
    pub flags_836: u8,
    pub ray_end_bias: f32,
    pub lighting_channels: FLightingChannels,
    pub view_lighting_channels: FViewLightingChannels,
    pub light_function_material: UPtr<UMaterialInterface>,
    pub stashed_light_function_material: UPtr<UMaterialInterface>,
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
}
pub struct UDirectionalLightComponent {
    pub shadow_cascade_bias_distribution: f32,
    pub flags_1076: u8,
    pub occlusion_mask_darkness: f32,
    pub occlusion_depth_range: f32,
    pub light_shaft_override_direction: crate::bindings::core_u_object::FVector,
    pub whole_scene_dynamic_shadow_radius_deprecated: f32,
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
    pub lightmass_settings: FLightmassDirectionalLightSettings,
    pub flags_1264: u8,
    pub modulated_shadow_color: crate::bindings::core_u_object::FColor,
    pub shadow_amount: f32,
}
pub struct UDrawFrustumComponent {
    pub b_frustum_enabled: bool,
    pub frustum_color: crate::bindings::core_u_object::FColor,
    pub frustum_angle: f32,
    pub frustum_aspect_ratio: f32,
    pub frustum_start_dist: f32,
    pub frustum_end_dist: f32,
    pub texture: UPtr<UTexture>,
}
pub struct USphereComponent {
    pub sphere_radius: f32,
}
pub struct UDrawSphereComponent {}
pub struct UExponentialHeightFogComponent {
    pub fog_density: f32,
    pub fog_height_falloff: f32,
    pub second_fog_data: FExponentialHeightFogData,
    pub fog_inscattering_color_deprecated: crate::bindings::core_u_object::FLinearColor,
    pub fog_inscattering_luminance: crate::bindings::core_u_object::FLinearColor,
    pub sky_atmosphere_ambient_contribution_color_scale: crate::bindings::core_u_object::FLinearColor,
    pub inscattering_color_cubemap: UPtr<UTextureCube>,
    pub inscattering_color_cubemap_angle: f32,
    pub inscattering_texture_tint: crate::bindings::core_u_object::FLinearColor,
    pub fully_directional_inscattering_color_distance: f32,
    pub non_directional_inscattering_color_distance: f32,
    pub directional_inscattering_exponent: f32,
    pub directional_inscattering_start_distance: f32,
    pub directional_inscattering_color_deprecated: crate::bindings::core_u_object::FLinearColor,
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
}
pub struct UForceFeedbackComponent {
    pub force_feedback_effect: UPtr<UForceFeedbackEffect>,
    pub flags_664: u8,
    pub intensity_multiplier: f32,
    pub attenuation_settings: UPtr<UForceFeedbackAttenuation>,
    pub attenuation_overrides: FForceFeedbackAttenuationSettings,
    pub on_force_feedback_finished: FForceFeedbackComponent_OnForceFeedbackFinished,
}
pub struct UHeterogeneousVolumeComponent {
    pub volume_resolution: crate::bindings::core_u_object::FIntVector,
    pub frame_transform: crate::bindings::core_u_object::FTransform,
    pub frame: f32,
    pub frame_rate: f32,
    pub start_frame: f32,
    pub end_frame: f32,
    pub flags_1712: u8,
    pub streaming_mip_bias: f32,
    pub flags_1720: u8,
    pub step_factor: f32,
    pub shadow_step_factor: f32,
    pub shadow_bias_factor: f32,
    pub lighting_downsample_factor: f32,
    pub material_instance_dynamic: UPtr<UMaterialInstanceDynamic>,
}
pub struct AHeterogeneousVolume {
    pub heterogeneous_volume_component: UPtr<UHeterogeneousVolumeComponent>,
}
pub struct UInputComponent {
    pub cached_key_to_action_info: TArray<FCachedKeyToActionInfo>,
}
pub struct UInstancedSkinnedMeshComponent {
    pub transform_provider: UPtr<UTransformProviderData>,
    pub instance_data: TArray<FSkinnedMeshInstanceData>,
    pub num_custom_data_floats: i32,
    pub instance_custom_data: TArray<f32>,
    pub animation_min_screen_size: f32,
    pub instance_min_draw_distance: i32,
    pub instance_start_cull_distance: i32,
    pub instance_end_cull_distance: i32,
    pub flags_2656: u8,
    pub primitive_bounds_override: crate::bindings::core_u_object::FBox,
    pub b_is_instance_data_gpu_only: bool,
    pub num_instances_gpu_only: i32,
}
pub struct UInterpToMovementComponent {
    pub duration: f32,
    pub flags_348: u8,
    pub b_sweep: bool,
    pub teleport_type: ETeleportType,
    pub behaviour_type: EInterpToBehaviourType,
    pub b_check_if_still_in_world: bool,
    pub flags_356: u8,
    pub on_interp_to_reverse: FInterpToMovementComponent_OnInterpToReverse,
    pub on_interp_to_stop: FInterpToMovementComponent_OnInterpToStop,
    pub on_wait_begin_delegate: FInterpToMovementComponent_OnWaitBeginDelegate,
    pub on_wait_end_delegate: FInterpToMovementComponent_OnWaitEndDelegate,
    pub on_reset_delegate: FInterpToMovementComponent_OnResetDelegate,
    pub max_simulation_time_step: f32,
    pub speed_multiplier: f32,
    pub max_simulation_iterations: i32,
    pub control_points: TArray<FInterpControlPoint>,
}
pub struct ULineBatchComponent {}
pub struct ULocalFogVolumeComponent {
    pub radial_fog_extinction: f32,
    pub height_fog_extinction: f32,
    pub height_fog_falloff: f32,
    pub height_fog_offset: f32,
    pub fog_phase_g: f32,
    pub fog_albedo: crate::bindings::core_u_object::FLinearColor,
    pub fog_emissive: crate::bindings::core_u_object::FLinearColor,
    pub fog_sort_priority: i32,
}
pub struct ULocalLightComponent {
    pub intensity_units: ELightUnits,
    pub inverse_exposure_blend: f32,
    pub radius_deprecated: f32,
    pub attenuation_radius: f32,
    pub lightmass_settings: FLightmassPointLightSettings,
}
pub struct ULODSyncComponent {
    pub num_lo_ds: i32,
    pub forced_lod: i32,
    pub min_lod: i32,
    pub components_to_sync: TArray<FComponentSync>,
    pub custom_lod_mapping: TMap<FName, FLODMappingData>,
    pub current_lod: i32,
    pub current_num_lo_ds: i32,
    pub drive_components: TArray<UPtr<UPrimitiveComponent>>,
    pub sub_components: TArray<UPtr<UPrimitiveComponent>>,
}
pub struct UMaterialBillboardComponent {
    pub elements: TArray<FMaterialSpriteElement>,
}
pub struct UModelComponent {
    pub model_body_setup: UPtr<UBodySetup>,
}
pub struct UPawnNoiseEmitterComponent {
    pub flags_240: u8,
    pub last_remote_noise_position: crate::bindings::core_u_object::FVector,
    pub noise_lifetime: f32,
    pub last_remote_noise_volume: f32,
    pub last_remote_noise_time: f32,
    pub last_local_noise_volume: f32,
    pub last_local_noise_time: f32,
}
pub struct UPlatformEventsComponent {
    pub platform_changed_to_laptop_mode_delegate: FPlatformEventsComponent_PlatformChangedToLaptopModeDelegate,
    pub platform_changed_to_tablet_mode_delegate: FPlatformEventsComponent_PlatformChangedToTabletModeDelegate,
}
pub struct UPointLightComponent {
    pub flags_1104: u8,
    pub light_falloff_exponent: f32,
    pub source_radius: f32,
    pub soft_source_radius: f32,
    pub source_length: f32,
}
pub struct UPoseableMeshComponent {}
pub struct UPostProcessComponent {
    pub settings: FPostProcessSettings,
    pub priority: f32,
    pub blend_radius: f32,
    pub blend_weight: f32,
    pub flags_2636: u8,
    pub volume_guid: crate::bindings::core_u_object::FGuid,
}
pub struct UProjectileMovementComponent {
    pub initial_speed: f32,
    pub max_speed: f32,
    pub flags_352: u8,
    pub flags_353: u8,
    pub flags_360: u8,
    pub previous_hit_time: f32,
    pub previous_hit_normal: crate::bindings::core_u_object::FVector,
    pub projectile_gravity_scale: f32,
    pub buoyancy: f32,
    pub bounciness: f32,
    pub friction: f32,
    pub bounce_velocity_stop_simulating_threshold: f32,
    pub min_friction_fraction: f32,
    pub on_projectile_bounce: FProjectileMovementComponent_OnProjectileBounce,
    pub on_projectile_stop: FProjectileMovementComponent_OnProjectileStop,
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
}
pub struct URectLightComponent {
    pub source_width: f32,
    pub source_height: f32,
    pub barn_door_angle: f32,
    pub barn_door_length: f32,
    pub light_function_cone_angle: f32,
    pub source_texture: UPtr<UTexture>,
    pub source_texture_scale: crate::bindings::core_u_object::FVector2f,
    pub source_texture_offset: crate::bindings::core_u_object::FVector2f,
    pub b_light_requires_broken_ev_math: bool,
}
pub struct URotatingMovementComponent {
    pub rotation_rate: crate::bindings::core_u_object::FRotator,
    pub pivot_translation: crate::bindings::core_u_object::FVector,
    pub flags_392: u8,
}
pub struct URuntimeVirtualTextureComponent {
    pub bounds_align_actor: TSoftObjectPtr<AActor>,
    pub b_set_bounds_button: bool,
    pub b_snap_bounds_to_landscape: bool,
    pub expand_bounds: f32,
    pub virtual_texture: UPtr<URuntimeVirtualTexture>,
    pub enable_in_game_per_platform: crate::bindings::core_u_object::FPerPlatformBool,
    pub b_enable_for_nanite_only: bool,
    pub b_use_min_material_quality: bool,
    pub min_in_game_material_quality: ERuntimeVirtualTextureMaterialQuality,
    pub b_enable_scalability: bool,
    pub scalability_group: u32,
    pub b_hide_primitives: bool,
    pub streaming_texture: UPtr<UVirtualTextureBuilder>,
    pub stream_low_mips: i32,
    pub b_build_streaming_mips_button: bool,
    pub lossy_compression_amount: ETextureLossyCompressionAmount,
    pub b_use_streaming_mips_fixed_color: bool,
    pub streaming_mips_fixed_color: crate::bindings::core_u_object::FLinearColor,
    pub b_use_streaming_mips_only: bool,
    pub use_streaming_mips_in_editor_mode: ERuntimeVirtualTextureUseStreamingMipsInEditorMode,
    pub b_use_streaming_mips_in_editor_deprecated: bool,
}
pub struct ASkyAtmosphere {
    pub sky_atmosphere_component: UPtr<USkyAtmosphereComponent>,
    pub arrow_component: UPtr<UArrowComponent>,
}
pub struct USkyLightComponent {
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
    pub blend_destination_cubemap: UPtr<UTextureCube>,
}
pub struct USplineMetadata {}
pub struct USplineComponent {
    pub spline: FSpline,
    pub spline_curves: FSplineCurves,
    pub spline_info_deprecated: crate::bindings::core_u_object::FInterpCurveVector,
    pub spline_rot_info_deprecated: crate::bindings::core_u_object::FInterpCurveQuat,
    pub spline_scale_info_deprecated: crate::bindings::core_u_object::FInterpCurveVector,
    pub spline_reparam_table_deprecated: crate::bindings::core_u_object::FInterpCurveFloat,
    pub b_allow_spline_editing_per_instance_deprecated: bool,
    pub reparam_steps_per_segment: i32,
    pub duration: f32,
    pub b_stationary_endpoints: bool,
    pub b_spline_has_been_edited: bool,
    pub b_modified_by_construction_script: bool,
    pub b_input_spline_points_to_construction_script: bool,
    pub b_draw_debug: bool,
    pub b_closed_loop: bool,
    pub b_loop_position_override: bool,
    pub loop_position: f32,
    pub line_material: TSoftObjectPtr<UMaterialInterface>,
    pub point_material: TSoftObjectPtr<UMaterialInterface>,
    pub line_material_lifetime_ptr: UPtr<UMaterialInterface>,
    pub point_material_lifetime_ptr: UPtr<UMaterialInterface>,
    pub default_up_vector: crate::bindings::core_u_object::FVector,
    pub editor_unselected_spline_segment_color: crate::bindings::core_u_object::FLinearColor,
    pub editor_selected_spline_segment_color: crate::bindings::core_u_object::FLinearColor,
    pub editor_tangent_color: crate::bindings::core_u_object::FLinearColor,
    pub b_allow_discontinuous_spline: bool,
    pub b_adjust_tangents_on_snap: bool,
    pub b_should_visualize_scale: bool,
    pub scale_visualization_width: f32,
    pub last_authority: ELastAuthority,
}
pub struct USplineMeshComponent {
    pub spline_params: FSplineMeshParams,
    pub spline_up_dir: crate::bindings::core_u_object::FVector,
    pub spline_boundary_min: f32,
    pub cached_mesh_body_setup_guid: crate::bindings::core_u_object::FGuid,
    pub virtual_texture_main_pass_max_draw_distance: f32,
    pub body_setup: UPtr<UBodySetup>,
    pub spline_boundary_max: f32,
    pub forward_axis: ESplineMeshAxis,
    pub flags_2189: u8,
}
pub struct USpotLightComponent {
    pub inner_cone_angle: f32,
    pub outer_cone_angle: f32,
}
pub struct UEditorFlagCollector {}
pub struct UStereoLayerShape {}
pub struct UStereoLayerShapeQuad {}
pub struct UStereoLayerShapeCylinder {
    pub radius: f32,
    pub overlay_arc: f32,
    pub height: i32,
}
pub struct UStereoLayerShapeCubemap {}
pub struct UStereoLayerShapeEquirect {
    pub left_uv_rect: crate::bindings::core_u_object::FBox2D,
    pub right_uv_rect: crate::bindings::core_u_object::FBox2D,
    pub left_scale: crate::bindings::core_u_object::FVector2D,
    pub right_scale: crate::bindings::core_u_object::FVector2D,
    pub left_bias: crate::bindings::core_u_object::FVector2D,
    pub right_bias: crate::bindings::core_u_object::FVector2D,
    pub radius: f32,
}
pub struct UStereoLayerComponent {
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
}
pub struct UTextRenderComponent {
    pub text: FText,
    pub text_material: UPtr<UMaterialInterface>,
    pub font: UPtr<UFont>,
    pub horizontal_alignment: EHorizTextAligment,
    pub vertical_alignment: EVerticalTextAligment,
    pub text_render_color: crate::bindings::core_u_object::FColor,
    pub x_scale: f32,
    pub y_scale: f32,
    pub world_size: f32,
    pub inv_default_size: f32,
    pub horiz_spacing_adjust: f32,
    pub vert_spacing_adjust: f32,
    pub flags_1568: u8,
}
pub struct UVolumetricCloudComponent {
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
    pub reflection_view_sample_count_scale_deprecated: f32,
    pub reflection_sample_count_scale_deprecated: f32,
    pub shadow_view_sample_count_scale: f32,
    pub shadow_reflection_view_sample_count_scale_value: f32,
    pub shadow_reflection_view_sample_count_scale_deprecated: f32,
    pub shadow_reflection_sample_count_scale_deprecated: f32,
    pub shadow_tracing_distance: f32,
    pub stop_tracing_transmittance_threshold: f32,
    pub aerial_pespective_rayleigh_scattering_start_distance: f32,
    pub aerial_pespective_rayleigh_scattering_fade_distance: f32,
    pub aerial_pespective_mie_scattering_start_distance: f32,
    pub aerial_pespective_mie_scattering_fade_distance: f32,
    pub flags_800: u8,
    pub material_lifetime_ptr: UPtr<UMaterialInterface>,
}
pub struct AVolumetricCloud {
    pub volumetric_cloud_component: UPtr<UVolumetricCloudComponent>,
}
pub struct UWorldPartitionStreamingSourceComponent {
    pub default_visualizer_loading_range: f32,
    pub target_behavior: EStreamingSourceTargetBehavior,
    pub target_grids: TArray<FName>,
    pub target_grid_deprecated: FName,
    pub debug_color: crate::bindings::core_u_object::FColor,
    pub target_hlod_layers_deprecated: TArray<UPtr<UHLODLayer>>,
    pub target_hlod_layer_deprecated: UPtr<UHLODLayer>,
    pub shapes: TArray<FStreamingSourceShape>,
    pub priority: EStreamingSourcePriority,
    pub b_streaming_source_enabled: bool,
    pub target_state: EStreamingSourceTargetState,
}
pub struct UCurveTable {
    pub asset_import_data: UPtr<UAssetImportData>,
    pub import_path_deprecated: FString,
}
pub struct UCompositeCurveTable {
    pub parent_tables: TArray<UPtr<UCurveTable>>,
    pub old_parent_tables: TArray<UPtr<UCurveTable>>,
}
pub struct UCompositeDataTable {
    pub parent_tables: TArray<UPtr<UDataTable>>,
    pub old_parent_tables: TArray<UPtr<UDataTable>>,
}
pub struct UStreamingSettings {
    pub flags_104: u8,
    pub time_limit_exceeded_multiplier: f32,
    pub time_limit_exceeded_min_time: f32,
    pub min_bulk_data_size_for_async_loading: i32,
    pub flags_120: u8,
    pub async_loading_time_limit: f32,
    pub priority_async_loading_extra_time: f32,
    pub level_streaming_actors_update_time_limit: f32,
    pub priority_level_streaming_actors_update_extra_time: f32,
    pub level_streaming_components_registration_granularity: i32,
    pub level_streaming_add_primitive_granularity: i32,
    pub level_streaming_unregister_components_time_limit: f32,
    pub level_streaming_components_unregistration_granularity: i32,
    pub flags_156: u8,
}
pub struct UGarbageCollectionSettings {
    pub time_between_purging_pending_kill_objects: f32,
    pub flags_108: u8,
    pub flags_109: u8,
    pub min_gc_cluster_size: i32,
    pub num_retries_before_forcing_gc: i32,
    pub max_objects_not_considered_by_gc: i32,
    pub max_objects_in_game: i32,
    pub max_objects_in_editor: i32,
}
pub struct ACullDistanceVolume {
    pub cull_distances: TArray<FCullDistanceSizePair>,
    pub flags_1224: u8,
}
pub struct UCurveBase {
    pub asset_import_data: UPtr<UAssetImportData>,
    pub import_path_deprecated: FString,
}
pub struct UDEPRECATED_CurveEdPresetCurve {}
pub struct UCurveFloat {
    pub float_curve: FRichCurve,
    pub b_is_event_curve: bool,
}
pub struct UCurveLinearColor {
    pub float_curves: FRichCurve,
    pub adjust_hue: f32,
    pub adjust_saturation: f32,
    pub adjust_brightness: f32,
    pub adjust_brightness_curve: f32,
    pub adjust_vibrance: f32,
    pub adjust_min_alpha: f32,
    pub adjust_max_alpha: f32,
}
pub struct UCurveLinearColorAtlas {
    pub texture_size: u32,
    pub gradient_curves: TArray<UPtr<UCurveLinearColor>>,
    pub flags_1384: u8,
    pub cached_color_adjustments: FCurveAtlasColorAdjustments,
}
pub struct UCurveVector {
    pub float_curves: FRichCurve,
}
pub struct UDamageType {
    pub flags_48: u8,
    pub damage_impulse: f32,
    pub destructible_impulse: f32,
    pub destructible_damage_spread_scale: f32,
    pub damage_falloff: f32,
}
pub struct UPrimaryDataAsset {
    pub asset_bundle_data: crate::bindings::core_u_object::FAssetBundleData,
}
pub struct UDataDrivenCVarEngineSubsystem {
    pub on_data_driven_c_var_delegate: FDataDrivenCVarEngineSubsystem_OnDataDrivenCVarDelegate,
}
pub struct UDataDrivenConsoleVariableSettings {
    pub c_vars_array: TArray<FDataDrivenConsoleVariable>,
}
pub struct UDataTableFunctionLibrary {}
pub struct ADebugCameraController {
    pub flags_2368: u8,
    pub draw_frustum: UPtr<UDrawFrustumComponent>,
    pub selected_actor: TWeakObjectPtr<AActor>,
    pub selected_component: TWeakObjectPtr<UPrimitiveComponent>,
    pub selected_hit_point: FHitResult,
    pub original_controller_ref: UPtr<APlayerController>,
    pub original_player: UPtr<UPlayer>,
    pub speed_scale: f32,
    pub initial_max_speed: f32,
    pub initial_accel: f32,
    pub initial_decel: f32,
}
pub struct UDebugCameraControllerSettings {
    pub cycle_view_modes: TArray<FDebugCameraControllerSettingsViewModeIndex>,
}
pub struct AHUD {
    pub player_owner: UPtr<APlayerController>,
    pub flags_1144: u8,
    pub current_target_index: i32,
    pub flags_1152: u8,
    pub post_rendered_actors: TArray<UPtr<AActor>>,
    pub debug_display: TArray<FName>,
    pub toggled_debug_categories: TArray<FName>,
    pub canvas: UPtr<UCanvas>,
    pub debug_canvas: UPtr<UCanvas>,
    pub debug_text_list: TArray<FDebugTextInfo>,
    pub show_debug_target_desired_class: TSubclassOf<AActor>,
    pub show_debug_target_actor: UPtr<AActor>,
}
pub struct ADebugCameraHUD {}
pub struct UDebugGarbageCollectionGraph {}
pub struct UDebugDrawService {}
pub struct UReporterBase {}
pub struct UReporterGraph {}
pub struct ADecalActor {
    pub decal: UPtr<UDecalComponent>,
    pub arrow_component: UPtr<UArrowComponent>,
    pub sprite_component: UPtr<UBillboardComponent>,
    pub box_component_deprecated: UPtr<UBoxComponent>,
}
pub struct ADefaultPawn {
    pub base_turn_rate: f32,
    pub base_look_up_rate: f32,
    pub movement_component: UPtr<UPawnMovementComponent>,
    pub collision_component: UPtr<USphereComponent>,
    pub mesh_component: UPtr<UStaticMeshComponent>,
    pub flags_1296: u8,
}
pub struct APhysicsVolume {
    pub terminal_velocity: f32,
    pub priority: i32,
    pub fluid_friction: f32,
    pub flags_1220: u8,
}
pub struct ADefaultPhysicsVolume {}
pub struct UDeformableInterface {}
pub struct IDeformableInterface {}
pub struct UDeletedObjectPlaceholder {
    pub display_name: FString,
    pub external_data_layer_uid: u32,
    pub original_object: TWeakObjectPtr<crate::bindings::core_u_object::UObject>,
}
pub struct UNetDriver {
    pub net_connection_class_name: FString,
    pub replication_driver_class_name: FString,
    pub replication_bridge_class_name: FString,
    pub replication_system_config_server: FNetDriverReplicationSystemConfig,
    pub replication_system_config_client: FNetDriverReplicationSystemConfig,
    pub max_download_size: i32,
    pub flags_172: u8,
    pub net_server_max_tick_rate: i32,
    pub max_net_tick_rate: i32,
    pub max_internet_client_rate: i32,
    pub max_client_rate: i32,
    pub server_travel_pause: f32,
    pub spawn_priority_seconds: f32,
    pub relevant_timeout: f32,
    pub keep_alive_time: f32,
    pub initial_connect_timeout: f32,
    pub connection_timeout: f32,
    pub graceful_close_connection_timeout: f32,
    pub timeout_multiplier_for_unoptimized_builds: f32,
    pub server_connection: UPtr<UNetConnection>,
    pub client_connections: TArray<UPtr<UNetConnection>>,
    pub recently_disconnected_tracking_time: i32,
    pub world: UPtr<UWorld>,
    pub world_package: UPtr<crate::bindings::core_u_object::UPackage>,
    pub net_connection_class: TSubclassOf<crate::bindings::core_u_object::UObject>,
    pub child_net_connection_class: TSubclassOf<crate::bindings::core_u_object::UObject>,
    pub replication_driver_class: TSubclassOf<crate::bindings::core_u_object::UObject>,
    pub replication_bridge_class: TSubclassOf<crate::bindings::core_u_object::UObject>,
    pub net_driver_name: FName,
    pub channel_definitions: TArray<FChannelDefinition>,
    pub channel_definition_map: TMap<FName, FChannelDefinition>,
    pub actor_channel_pool: TArray<UPtr<UChannel>>,
    pub network_metrics_database: UPtr<UNetworkMetricsDatabase>,
    pub network_metrics_listeners: TMap<FName, UPtr<UNetworkMetricsBaseListener>>,
    pub flags_825: u8,
    pub flags_826: u8,
    pub replication_driver: UPtr<UReplicationDriver>,
    pub b_send_immediate_acks: bool,
}
pub struct UDemoNetDriver {
    pub rollback_net_startup_actors: TMap<FString, FRollbackNetStartupActorInfo>,
    pub checkpoint_save_max_ms_per_frame: f32,
    pub multicast_record_options: TArray<FMulticastRecordOptions>,
    pub spectator_controllers: TArray<UPtr<APlayerController>>,
}
pub struct UDestructibleInterface {}
pub struct IDestructibleInterface {}
pub struct UTextureLODSettings {
    pub texture_lod_groups: TArray<FTextureLODGroup>,
}
pub struct UDeviceProfile {
    pub device_type: FString,
    pub base_profile_name: FString,
    pub flags_104: u8,
    pub parent: UPtr<UDeviceProfile>,
    pub c_vars: TArray<FString>,
    pub matching_rules: TArray<FDPMatchingRulestruct>,
}
pub struct UDeviceProfileManager {
    pub profiles: TArray<UPtr<UDeviceProfile>>,
    pub backup_profiles: TArray<UPtr<UDeviceProfile>>,
}
pub struct UDialogueVoice {
    pub gender: EGrammaticalGender,
    pub plurality: EGrammaticalNumber,
    pub localization_guid: crate::bindings::core_u_object::FGuid,
}
pub struct UDialogueWave {
    pub flags_48: u8,
    pub spoken_text: FString,
    pub subtitle_override: FString,
    pub voice_actor_direction: FString,
    pub context_mappings: TArray<FDialogueContextMapping>,
    pub localization_guid: crate::bindings::core_u_object::FGuid,
}
pub struct ADocumentationActor {
    pub document_link: FString,
    pub billboard: UPtr<UMaterialBillboardComponent>,
}
pub struct UDPICustomScalingRule {}
pub struct UGraphNodeContextMenuContext {
    pub blueprint: UPtr<UBlueprint>,
    pub graph: UPtr<UEdGraph>,
    pub node: UPtr<UEdGraphNode>,
    pub b_is_debugging: bool,
}
pub struct UEdGraphNode_Documentation {
    pub link: FString,
    pub excerpt: FString,
}
pub struct UEdGraphPin_Deprecated {
    pub pin_name: FString,
    pub pin_friendly_name: FText,
    pub pin_tool_tip: FString,
    pub direction: EEdGraphPinDirection,
    pub pin_type: FEdGraphPinType,
    pub default_value: FString,
    pub autogenerated_default_value: FString,
    pub default_object: UPtr<crate::bindings::core_u_object::UObject>,
    pub default_text_value: FText,
    pub linked_to: TArray<UPtr<UEdGraphPin_Deprecated>>,
    pub sub_pins: TArray<UPtr<UEdGraphPin_Deprecated>>,
    pub parent_pin: UPtr<UEdGraphPin_Deprecated>,
    pub reference_pass_through_connection: UPtr<UEdGraphPin_Deprecated>,
    pub flags_320: u8,
    pub persistent_guid: crate::bindings::core_u_object::FGuid,
}
pub struct UActorElementCounterInterface {}
pub struct UActorElementHierarchyInterface {}
pub struct UActorElementObjectInterface {}
pub struct UActorElementTedsTypedElementBridgeInterface {}
pub struct UComponentElementCounterInterface {}
pub struct UComponentElementHierarchyInterface {}
pub struct UComponentElementObjectInterface {}
pub struct UEngineElementsLibrary {}
pub struct UTypedElementCommonActions {}
pub struct UTypedElementWorldInterface {}
pub struct ITypedElementWorldInterface {}
pub struct UObjectElementAssetDataInterface {}
pub struct UObjectElementCounterInterface {}
pub struct UObjectElementObjectInterface {}
pub struct USMInstanceElementAssetDataInterface {}
pub struct USMInstanceElementHierarchyInterface {}
pub struct USMInstanceElementIdMapTransactor {}
pub struct USMInstanceElementPrimitiveCustomDataInterface {}
pub struct ULocalMessage {}
pub struct UEngineMessage {
    pub failed_place_message: FString,
    pub maxed_out_message: FString,
    pub entered_message: FString,
    pub left_message: FString,
    pub global_name_change: FString,
    pub spec_entered_message: FString,
    pub new_player_message: FString,
    pub new_spec_message: FString,
}
pub struct UEngineTypes {}
pub struct UAutoDestroySubsystem {
    pub actors_to_poll: TArray<UPtr<AActor>>,
}
pub struct UCancellableAsyncAction {}
pub struct ULODSyncInterface {}
pub struct ILODSyncInterface {}
pub struct UPoseWatchFolder {
    pub label: FText,
    pub parent: TWeakObjectPtr<UPoseWatchFolder>,
    pub b_is_visible: bool,
    pub b_is_expanded: bool,
}
pub struct UPoseWatchElement {
    pub b_is_visible: bool,
    pub b_has_color: bool,
    pub color: crate::bindings::core_u_object::FColor,
    pub label: FText,
    pub icon_name: FName,
    pub parent: TWeakObjectPtr<UPoseWatch>,
}
pub struct UPoseWatchPoseElement {
    pub viewport_mask: UPtr<UBlendProfile>,
    pub b_invert_viewport_mask: bool,
    pub blend_scale_threshold: f32,
    pub viewport_offset: crate::bindings::core_u_object::FVector3d,
}
pub struct UPoseWatch {
    pub node: TWeakObjectPtr<UEdGraphNode>,
    pub viewport_mask_deprecated: UPtr<UBlendProfile>,
    pub b_invert_viewport_mask_deprecated: bool,
    pub blend_scale_threshold_deprecated: f32,
    pub viewport_offset_deprecated: crate::bindings::core_u_object::FVector3d,
    pub elements: TArray<UPtr<UPoseWatchElement>>,
    pub b_delete_on_deselection: bool,
    pub b_is_visible: bool,
    pub b_is_node_enabled: bool,
    pub b_is_expanded: bool,
    pub color_deprecated: crate::bindings::core_u_object::FColor,
    pub label: FText,
    pub icon_name_deprecated: FName,
    pub parent: TWeakObjectPtr<UPoseWatchFolder>,
}
pub struct AServerStatReplicator {
    pub b_update_stat_net: bool,
    pub b_overwrite_client_stats: bool,
    pub channels: u32,
    pub in_rate: u32,
    pub out_rate: u32,
    pub max_packet_overhead: u32,
    pub in_rate_client_max: u32,
    pub in_rate_client_min: u32,
    pub in_rate_client_avg: u32,
    pub in_packets_client_max: u32,
    pub in_packets_client_min: u32,
    pub in_packets_client_avg: u32,
    pub out_rate_client_max: u32,
    pub out_rate_client_min: u32,
    pub out_rate_client_avg: u32,
    pub out_packets_client_max: u32,
    pub out_packets_client_min: u32,
    pub out_packets_client_avg: u32,
    pub net_num_clients: u32,
    pub in_packets: u32,
    pub out_packets: u32,
    pub in_bunches: u32,
    pub out_bunches: u32,
    pub out_loss: u32,
    pub in_loss: u32,
    pub voice_bytes_sent: u32,
    pub voice_bytes_recv: u32,
    pub voice_packets_sent: u32,
    pub voice_packets_recv: u32,
    pub percent_in_voice: u32,
    pub percent_out_voice: u32,
    pub num_actor_channels: u32,
    pub num_considered_actors: u32,
    pub prioritized_actors: u32,
    pub num_relevant_actors: u32,
    pub num_relevant_deleted_actors: u32,
    pub num_replicated_actor_attempts: u32,
    pub num_replicated_actors: u32,
    pub num_actors: u32,
    pub num_net_actors: u32,
    pub num_dormant_actors: u32,
    pub num_initially_dormant_actors: u32,
    pub num_net_gui_ds_ackd: u32,
    pub num_net_gui_ds_pending: u32,
    pub num_net_gui_ds_un_ackd: u32,
    pub obj_path_bytes: u32,
    pub net_guid_out_rate: u32,
    pub net_guid_in_rate: u32,
    pub net_saturated: u32,
}
pub struct USystemTimeTimecodeProvider {
    pub frame_rate: crate::bindings::core_u_object::FFrameRate,
    pub b_generate_full_frame: bool,
    pub b_use_high_performance_clock: bool,
}
pub struct UViewportStatsSubsystem {}
pub struct UFieldNotificationLibrary {}
pub struct UFloatingPawnMovement {
    pub max_speed: f32,
    pub acceleration: f32,
    pub deceleration: f32,
    pub turning_boost: f32,
    pub flags_472: u8,
}
pub struct UFont {
    pub font_cache_type: EFontCacheType,
    pub runtime_font_source: ERuntimeFontSource,
    pub characters: TArray<FFontCharacter>,
    pub textures: TArray<UPtr<UTexture2D>>,
    pub is_remapped: i32,
    pub em_scale: f32,
    pub ascent: f32,
    pub descent: f32,
    pub leading: f32,
    pub kerning: i32,
    pub import_options: FFontImportOptionsData,
    pub num_characters: i32,
    pub max_char_height: TArray<i32>,
    pub scaling_factor: f32,
    pub legacy_font_size: i32,
    pub legacy_font_name: FName,
    pub composite_font: crate::bindings::slate_core::FCompositeFont,
}
pub struct UFontFace {
    pub source_filename: FString,
    pub hinting: crate::bindings::slate_core::EFontHinting,
    pub loading_policy: crate::bindings::slate_core::EFontLoadingPolicy,
    pub layout_method: crate::bindings::slate_core::EFontLayoutMethod,
    pub ascend_overridden_value: i32,
    pub b_is_ascend_overridden: bool,
    pub descend_overridden_value: i32,
    pub b_is_descend_overridden: bool,
    pub strike_brush_height_percentage: i32,
    pub font_face_data_deprecated: TArray<u8>,
    pub sub_faces: TArray<FString>,
    pub b_enable_distance_field_rendering: bool,
    pub min_distance_field_ppem: i32,
    pub mid_distance_field_ppem: i32,
    pub max_distance_field_ppem: i32,
    pub min_multi_distance_field_ppem: i32,
    pub mid_multi_distance_field_ppem: i32,
    pub max_multi_distance_field_ppem: i32,
    pub platform_rasterization_mode_overrides: TOptional<
        FFontFacePlatformRasterizationOverrides,
    >,
}
pub struct UGameEngine {
    pub max_delta_time: f32,
    pub server_flush_log_interval: f32,
    pub game_instance: UPtr<UGameInstance>,
}
pub struct UAsyncActionHandleSaveGame {
    pub completed: FAsyncActionHandleSaveGame_Completed,
    pub save_game_object: UPtr<USaveGame>,
}
pub struct UForceFeedbackEffect {
    pub channel_details: TArray<FForceFeedbackChannelDetails>,
    pub per_device_overrides: TMap<FName, FForceFeedbackEffectOverridenChannelDetails>,
    pub device_properties: TArray<UPtr<UInputDeviceProperty>>,
    pub duration: f32,
}
pub struct UInputDeviceProperty {
    pub property_duration: f32,
}
pub struct UColorInputDeviceProperty {
    pub color_data: FDeviceColorData,
    pub device_override_data: TMap<FName, FDeviceColorData>,
}
pub struct UColorInputDeviceCurveProperty {
    pub color_data: FDeviceColorCurveData,
    pub device_override_data: TMap<FName, FDeviceColorCurveData>,
}
pub struct UInputDeviceTriggerEffect {
    pub base_trigger_data: FDeviceTriggerBaseData,
}
pub struct UInputDeviceTriggerFeedbackProperty {
    pub trigger_data: FDeviceTriggerFeedbackData,
    pub device_override_data: TMap<FName, FDeviceTriggerFeedbackData>,
}
pub struct UInputDeviceTriggerResistanceProperty {
    pub trigger_data: FDeviceTriggerTriggerResistanceData,
    pub device_override_data: TMap<FName, FDeviceTriggerTriggerResistanceData>,
}
pub struct UInputDeviceTriggerVibrationProperty {
    pub trigger_data: FDeviceTriggerTriggerVibrationData,
    pub device_override_data: TMap<FName, FDeviceTriggerTriggerVibrationData>,
}
pub struct UInputDeviceAudioBasedVibrationProperty {
    pub data: FAudioBasedVibrationData,
    pub device_override_data: TMap<FName, FAudioBasedVibrationData>,
}
pub struct UInputDeviceSubsystem {
    pub on_input_hardware_device_changed: FInputDeviceSubsystem_OnInputHardwareDeviceChanged,
    pub active_properties: TSet<FActiveDeviceProperty>,
    pub properties_pending_removal: TSet<FInputDevicePropertyHandle>,
}
pub struct UPlayerStateCountLimiterConfig {}
pub struct USaveGame {}
pub struct ULocalPlayerSaveGame {
    pub owning_player: UPtr<ULocalPlayer>,
    pub save_slot_name: FString,
    pub saved_data_version: i32,
    pub loaded_data_version: i32,
    pub current_save_request: i32,
    pub last_successful_save_request: i32,
    pub last_error_save_request: i32,
}
pub struct USpringArmComponent {
    pub target_arm_length: f32,
    pub socket_offset: crate::bindings::core_u_object::FVector,
    pub target_offset: crate::bindings::core_u_object::FVector,
    pub probe_size: f32,
    pub probe_channel: ECollisionChannel,
    pub flags_720: u8,
    pub flags_721: u8,
    pub camera_lag_speed: f32,
    pub camera_rotation_lag_speed: f32,
    pub camera_lag_max_time_step: f32,
    pub camera_lag_max_distance: f32,
    pub flags_740: u8,
}
pub struct UTouchInterface {
    pub controls: TArray<FTouchInputControl>,
    pub active_opacity: f32,
    pub inactive_opacity: f32,
    pub time_until_deactive: f32,
    pub time_until_reset: f32,
    pub activation_delay: f32,
    pub b_prevent_recenter: bool,
    pub startup_delay: f32,
}
pub struct AGameModeBase {
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
    pub game_session: UPtr<AGameSession>,
    pub game_state: UPtr<AGameStateBase>,
    pub server_stat_replicator: UPtr<AServerStatReplicator>,
    pub default_player_name: FText,
    pub flags_1272: u8,
    pub game_net_driver_replication_system: crate::bindings::net_core::EReplicationSystem,
}
pub struct AGameMode {
    pub match_state: FName,
    pub flags_1308: u8,
    pub num_spectators: i32,
    pub num_players: i32,
    pub num_bots: i32,
    pub min_respawn_delay: f32,
    pub num_travelling_players: i32,
    pub engine_message_class: TSubclassOf<ULocalMessage>,
    pub inactive_player_array: TArray<UPtr<APlayerState>>,
    pub inactive_player_state_life_span: f32,
    pub max_inactive_players: i32,
    pub b_handle_dedicated_server_replays: bool,
}
pub struct AGameNetworkManager {
    pub bad_packet_loss_threshold: f32,
    pub severe_packet_loss_threshold: f32,
    pub bad_ping_threshold: i32,
    pub severe_ping_threshold: i32,
    pub adjusted_net_speed: i32,
    pub last_net_speed_update_time: f32,
    pub total_net_bandwidth: i32,
    pub min_dynamic_bandwidth: i32,
    pub max_dynamic_bandwidth: i32,
    pub flags_1180: u8,
    pub standby_rx_cheat_time: f32,
    pub standby_tx_cheat_time: f32,
    pub percent_missing_for_rx_standby: f32,
    pub percent_missing_for_tx_standby: f32,
    pub percent_for_bad_ping: f32,
    pub join_in_progress_standby_wait_time: f32,
    pub move_rep_size: f32,
    pub maxpositionerrorsquared: f32,
    pub maxnearzerovelocitysquared: f32,
    pub clientadjustupdatecost: f32,
    pub maxclientupdateinterval: f32,
    pub max_client_forced_update_duration: f32,
    pub server_forced_update_hitch_threshold: f32,
    pub server_forced_update_hitch_cooldown: f32,
    pub max_move_delta_time: f32,
    pub max_client_smoothing_delta_time: f32,
    pub client_net_send_move_delta_time: f32,
    pub client_net_send_move_delta_time_throttled: f32,
    pub client_net_send_move_delta_time_stationary: f32,
    pub client_net_send_move_throttle_at_net_speed: i32,
    pub client_net_send_move_throttle_over_player_count: i32,
    pub client_error_update_rate_limit: f32,
    pub client_net_cam_update_delta_time: f32,
    pub client_net_cam_update_position_limit: f32,
    pub client_authorative_position: bool,
    pub b_movement_time_discrepancy_detection: bool,
    pub b_movement_time_discrepancy_resolution: bool,
    pub movement_time_discrepancy_max_time_margin: f32,
    pub movement_time_discrepancy_min_time_margin: f32,
    pub movement_time_discrepancy_resolution_rate: f32,
    pub movement_time_discrepancy_drift_allowance: f32,
    pub b_movement_time_discrepancy_force_corrections_during_resolution: bool,
    pub b_use_distance_based_relevancy: bool,
}
pub struct UGameplayStatics {}
pub struct AGameSession {
    pub max_spectators: i32,
    pub max_players: i32,
    pub max_party_size: i32,
    pub max_splitscreens_per_connection: u8,
    pub b_requires_push_to_talk: bool,
    pub session_name: FName,
}
pub struct AGameStateBase {
    pub game_mode_class: TSubclassOf<AGameModeBase>,
    pub authority_game_mode: UPtr<AGameModeBase>,
    pub spectator_class: TSubclassOf<ASpectatorPawn>,
    pub player_array: TArray<UPtr<APlayerState>>,
    pub b_replicated_has_begun_play: bool,
    pub replicated_world_time_seconds_double: f64,
    pub server_world_time_seconds_delta: f32,
    pub server_world_time_seconds_update_frequency: f32,
}
pub struct AGameState {
    pub match_state: FName,
    pub previous_match_state: FName,
    pub elapsed_time: i32,
}
pub struct UGameUserSettings {
    pub b_use_v_sync: bool,
    pub b_use_dynamic_resolution: bool,
    pub resolution_size_x: u32,
    pub resolution_size_y: u32,
    pub last_user_confirmed_resolution_size_x: u32,
    pub last_user_confirmed_resolution_size_y: u32,
    pub window_pos_x_deprecated: i32,
    pub window_pos_y_deprecated: i32,
    pub window_positions: TArray<crate::bindings::core_u_object::FIntPoint>,
    pub fullscreen_mode: i32,
    pub last_confirmed_fullscreen_mode: i32,
    pub preferred_fullscreen_mode: i32,
    pub version: u32,
    pub audio_quality_level: i32,
    pub last_confirmed_audio_quality_level: i32,
    pub frame_rate_limit: f32,
    pub desired_screen_width: i32,
    pub b_use_desired_screen_height: bool,
    pub desired_screen_height: i32,
    pub last_user_confirmed_desired_screen_width: i32,
    pub last_user_confirmed_desired_screen_height: i32,
    pub last_recommended_screen_width: f32,
    pub last_recommended_screen_height: f32,
    pub last_cpu_benchmark_result: f32,
    pub last_gpu_benchmark_result: f32,
    pub last_cpu_benchmark_steps: TArray<f32>,
    pub last_gpu_benchmark_steps: TArray<f32>,
    pub last_gpu_benchmark_multiplier: f32,
    pub b_use_hdr_display_output: bool,
    pub hdr_display_output_nits: i32,
    pub on_game_user_settings_ui_needs_update: FGameUserSettings_OnGameUserSettingsUINeedsUpdate,
}
pub struct UScriptViewportClient {}
pub struct UGameViewportClient {
    pub viewport_console: UPtr<UConsole>,
    pub debug_properties: TArray<FDebugDisplayProperty>,
    pub max_splitscreen_players: i32,
    pub world: UPtr<UWorld>,
    pub game_instance: UPtr<UGameInstance>,
}
pub struct UGeneratedBlueprintBinding {
    pub generated_blueprint_bindings: TArray<FGeneratedBlueprintDelegateBinding>,
}
pub struct UHLODProxy {
    pub owning_map: TSoftObjectPtr<UWorld>,
    pub proxy_meshes: TArray<FHLODProxyMesh>,
    pub hlod_actors: TMap<UPtr<UHLODProxyDesc>, FHLODProxyMesh>,
}
pub struct UHLODEngineSubsystem {}
pub struct UHLODProxyDesc {
    pub sub_actors: TArray<FName>,
    pub static_mesh: UPtr<UStaticMesh>,
    pub ism_components_desc: TArray<FHLODISMComponentDesc>,
    pub lod_draw_distance: f32,
    pub b_override_material_merge_settings: bool,
    pub material_settings: FMaterialProxySettings,
    pub b_override_transition_screen_size: bool,
    pub transition_screen_size: f32,
    pub b_override_screen_size: bool,
    pub screen_size: i32,
    pub key: FName,
    pub lod_level: i32,
    pub lod_actor_tag: FString,
    pub location: crate::bindings::core_u_object::FVector,
    pub hlod_baking_transform: crate::bindings::core_u_object::FTransform,
    pub sub_hlod_descs: TArray<TSoftObjectPtr<UHLODProxyDesc>>,
}
pub struct UHierarchicalLODSetup {
    pub hierarchical_lod_setup: TArray<FHierarchicalSimplification>,
    pub override_base_material: TSoftObjectPtr<UMaterialInterface>,
}
pub struct UImportantToggleSettingInterface {}
pub struct IImportantToggleSettingInterface {}
pub struct UInheritableComponentHandler {
    pub records: TArray<FComponentOverrideRecord>,
    pub unnecessary_components: TArray<UPtr<UActorComponent>>,
}
pub struct UInputDelegateBinding {}
pub struct UInputActionDelegateBinding {
    pub input_action_delegate_bindings: TArray<FBlueprintInputActionDelegateBinding>,
}
pub struct UInputAxisDelegateBinding {
    pub input_axis_delegate_bindings: TArray<FBlueprintInputAxisDelegateBinding>,
}
pub struct UInputAxisKeyDelegateBinding {
    pub input_axis_key_delegate_bindings: TArray<FBlueprintInputAxisKeyDelegateBinding>,
}
pub struct UInputDeviceLibrary {}
pub struct UInputKeyDelegateBinding {
    pub input_key_delegate_bindings: TArray<FBlueprintInputKeyDelegateBinding>,
}
pub struct UInputTouchDelegateBinding {
    pub input_touch_delegate_bindings: TArray<FBlueprintInputTouchDelegateBinding>,
}
pub struct UInputVectorAxisDelegateBinding {}
pub struct AInstancedPlacementPartitionActor {
    pub placement_grid_guid: crate::bindings::core_u_object::FGuid,
}
pub struct UStringTable {}
pub struct UInterpCurveEdSetup {
    pub tabs: TArray<FCurveEdTab>,
    pub active_tab: i32,
}
pub struct UIntSerialization {
    pub unsigned_int16_variable: u16,
    pub unsigned_int32_variable: u32,
    pub unsigned_int64_variable: u64,
    pub signed_int8_variable: i8,
    pub signed_int16_variable: i16,
    pub signed_int64_variable: i64,
    pub unsigned_int8_variable: u8,
    pub signed_int32_variable: i32,
}
pub struct AKillZVolume {}
pub struct UKismetArrayLibrary {}
pub struct UKismetGuidLibrary {}
pub struct UKismetInputLibrary {}
pub struct UKismetInternationalizationLibrary {}
pub struct UKismetMaterialLibrary {}
pub struct UKismetMathLibrary {}
pub struct UKismetNodeHelperLibrary {}
pub struct UKismetRenderingLibrary {}
pub struct UKismetStringLibrary {}
pub struct UKismetStringTableLibrary {}
pub struct UKismetSystemLibrary {}
pub struct UKismetTextLibrary {}
pub struct UBlueprintPathsLibrary {}
pub struct UPlatformGameInstance {
    pub application_will_deactivate_delegate: FPlatformGameInstance_ApplicationWillDeactivateDelegate,
    pub application_has_reactivated_delegate: FPlatformGameInstance_ApplicationHasReactivatedDelegate,
    pub application_will_enter_background_delegate: FPlatformGameInstance_ApplicationWillEnterBackgroundDelegate,
    pub application_has_entered_foreground_delegate: FPlatformGameInstance_ApplicationHasEnteredForegroundDelegate,
    pub application_will_terminate_delegate: FPlatformGameInstance_ApplicationWillTerminateDelegate,
    pub application_should_unload_resources_delegate: FPlatformGameInstance_ApplicationShouldUnloadResourcesDelegate,
    pub application_received_startup_arguments_delegate: FPlatformGameInstance_ApplicationReceivedStartupArgumentsDelegate,
    pub application_registered_for_remote_notifications_delegate: FPlatformGameInstance_ApplicationRegisteredForRemoteNotificationsDelegate,
    pub application_registered_for_user_notifications_delegate: FPlatformGameInstance_ApplicationRegisteredForUserNotificationsDelegate,
    pub application_failed_to_register_for_remote_notifications_delegate: FPlatformGameInstance_ApplicationFailedToRegisterForRemoteNotificationsDelegate,
    pub application_received_remote_notification_delegate: FPlatformGameInstance_ApplicationReceivedRemoteNotificationDelegate,
    pub application_received_local_notification_delegate: FPlatformGameInstance_ApplicationReceivedLocalNotificationDelegate,
    pub application_received_screen_orientation_changed_notification_delegate: FPlatformGameInstance_ApplicationReceivedScreenOrientationChangedNotificationDelegate,
}
pub struct UBlueprintPlatformLibrary {}
pub struct UBlueprintTypeConversions {}
pub struct UImportanceSamplingLibrary {}
pub struct ULayer {
    pub layer_name: FName,
    pub flags_60: u8,
    pub actor_stats: TArray<FLayerActorStats>,
}
pub struct UActorContainer {
    pub actors: TMap<FName, UPtr<AActor>>,
    pub runtime_level: UPtr<ULevel>,
}
pub struct ULevelActorContainer {
    pub actors: TArray<UPtr<AActor>>,
}
pub struct ALevelBounds {
    pub box_component: UPtr<UBoxComponent>,
    pub b_auto_update_bounds: bool,
}
pub struct ALevelInstance {
    pub world_asset: TSoftObjectPtr<UWorld>,
    pub property_overrides: UPtr<ULevelInstancePropertyOverrideAsset>,
    pub level_instance_component: UPtr<ULevelInstanceComponent>,
    pub cooked_world_asset: TSoftObjectPtr<UWorld>,
    pub level_instance_spawn_guid: crate::bindings::core_u_object::FGuid,
    pub desired_runtime_behavior: ELevelInstanceRuntimeBehavior,
}
pub struct ULevelInstanceComponent {
    pub shadow_desired_runtime_behavior: ELevelInstanceRuntimeBehavior,
    pub filter: FWorldPartitionActorFilter,
    pub edit_filter: FWorldPartitionActorFilter,
}
pub struct ALevelInstanceEditorInstanceActor {}
pub struct ULevelStreamingLevelInstanceEditor {}
pub struct ULevelInstanceEditorObject {
    pub b_moved_actors: bool,
    pub other_packages_to_save: TArray<
        TWeakObjectPtr<crate::bindings::core_u_object::UPackage>,
    >,
}
pub struct ALevelInstancePivot {}
pub struct ULevelStreamingLevelInstanceEditorPropertyOverride {
    pub archetype_world: UPtr<UWorld>,
}
pub struct ULevelInstanceInterface {}
pub struct ILevelInstanceInterface {}
pub struct ULevelStreamingLevelInstance {}
pub struct UWorldPartitionPropertyOverride {
    pub property_overrides_per_container: TMap<
        FActorContainerPath,
        FContainerPropertyOverride,
    >,
}
pub struct ULevelInstancePropertyOverrideAsset {
    pub world_asset: TSoftObjectPtr<UWorld>,
    pub b_saving_override_edit: bool,
}
pub struct UWorldPartitionPropertyOverridePolicy {}
pub struct ULevelInstancePropertyOverridePolicy {}
pub struct ULevelInstanceSettings {
    pub property_override_policy_class: FString,
    pub property_override_policy: UPtr<ULevelInstancePropertyOverridePolicy>,
    pub b_is_level_instance_disabled: bool,
}
pub struct ULevelInstanceSubsystem {}
pub struct ULevelInstancePropertyOverrideSamplePolicy {}
pub struct ALevelScriptActor {
    pub flags_1136: u8,
}
pub struct ULevelScriptBlueprint {
    pub friendly_name: FString,
}
pub struct ALevelStreamingVolume {
    pub streaming_level_names: TArray<FName>,
    pub flags_1224: u8,
    pub streaming_usage: EStreamingVolumeUsage,
}
pub struct ULightmappedSurfaceCollection {
    pub source_model: UPtr<UModel>,
    pub surfaces: TArray<i32>,
}
pub struct ALightmassCharacterIndirectDetailVolume {}
pub struct ALightmassImportanceVolume {}
pub struct ULightmassPrimitiveSettingsObject {
    pub lightmass_settings: FLightmassPrimitiveSettings,
}
pub struct ULightWeightInstanceBlueprintFunctionLibrary {}
pub struct ALightWeightInstanceManager {
    pub represented_class: TSubclassOf<AActor>,
    pub accepted_class: TSubclassOf<AActor>,
    pub instance_transforms: TArray<crate::bindings::core_u_object::FTransform>,
    pub actors: TMap<i32, UPtr<AActor>>,
    pub free_indices: TArray<i32>,
    pub valid_indices: TArray<bool>,
}
pub struct ALightWeightInstanceStaticMeshManager {
    pub static_mesh: TSoftObjectPtr<UStaticMesh>,
    pub instanced_static_mesh_component_deprecated: UPtr<
        UHierarchicalInstancedStaticMeshComponent,
    >,
    pub ism_component: UPtr<UInstancedStaticMeshComponent>,
    pub rendering_indices_to_data_indices: TArray<i32>,
    pub data_indices_to_rendering_indices: TArray<i32>,
}
pub struct ULocalPlayer {
    pub viewport_client: UPtr<UGameViewportClient>,
    pub aspect_ratio_axis_constraint: EAspectRatioAxisConstraint,
    pub pending_level_player_controller_class: TSubclassOf<APlayerController>,
    pub flags_208: u8,
    pub controller_id: i32,
}
pub struct ALocationVolume {
    pub debug_color: crate::bindings::core_u_object::FColor,
}
pub struct ALODActor {
    pub static_mesh_component: UPtr<UStaticMeshComponent>,
    pub instanced_static_mesh_components: TMap<
        FHLODInstancingKey,
        UPtr<UInstancedStaticMeshComponent>,
    >,
    pub proxy: UPtr<UHLODProxy>,
    pub key: FName,
    pub lod_draw_distance: f32,
    pub lod_level: i32,
    pub sub_actors: TArray<UPtr<AActor>>,
    pub cached_num_hlod_levels: u8,
    pub num_triangles_in_sub_actors: u32,
    pub num_triangles_in_merged_mesh: u32,
    pub b_override_material_merge_settings: bool,
    pub material_settings: FMaterialProxySettings,
    pub b_override_transition_screen_size: bool,
    pub transition_screen_size: f32,
    pub b_override_screen_size: bool,
    pub screen_size: i32,
    pub lod_actor_tag: FString,
    pub flags_1504: u8,
    pub proxy_desc: UPtr<UHLODProxyDesc>,
}
pub struct UMaterialCacheVirtualTexture {
    pub owning_component: TWeakObjectPtr<UPrimitiveComponent>,
    pub material_stack_provider: TWeakObjectPtr<UMaterialCacheStackProvider>,
    pub tag: UPtr<UMaterialCacheVirtualTextureTag>,
    pub tile_count: crate::bindings::core_u_object::FIntPoint,
}
pub struct UMaterialCacheVirtualTextureTag {
    pub attributes: TArray<EMaterialCacheAttribute>,
    pub tile_count_multiplier: crate::bindings::core_u_object::FIntPoint,
    pub runtime_layers: TArray<FMaterialCacheLayer>,
    pub guid: crate::bindings::core_u_object::FGuid,
}
pub struct AMaterialInstanceActor {
    pub target_actors: TArray<UPtr<AActor>>,
}
pub struct UMaterialInterfaceEditorOnlyData {}
pub struct UMaterialEditorOnlyData {
    pub base_color: FColorMaterialInput,
    pub metallic: FScalarMaterialInput,
    pub specular: FScalarMaterialInput,
    pub roughness: FScalarMaterialInput,
    pub anisotropy: FScalarMaterialInput,
    pub normal: FVectorMaterialInput,
    pub tangent: FVectorMaterialInput,
    pub emissive_color: FColorMaterialInput,
    pub opacity: FScalarMaterialInput,
    pub opacity_mask: FScalarMaterialInput,
    pub world_position_offset: FVectorMaterialInput,
    pub displacement: FScalarMaterialInput,
    pub subsurface_color: FColorMaterialInput,
    pub clear_coat: FScalarMaterialInput,
    pub clear_coat_roughness: FScalarMaterialInput,
    pub ambient_occlusion: FScalarMaterialInput,
    pub refraction: FScalarMaterialInput,
    pub customized_u_vs: FVector2MaterialInput,
    pub material_attributes: FMaterialAttributesInput,
    pub pixel_depth_offset: FScalarMaterialInput,
    pub shading_model_from_material_expression: FShadingModelMaterialInput,
    pub surface_thickness: FScalarMaterialInput,
    pub front_material: FSubstrateMaterialInput,
    pub expression_collection: FMaterialExpressionCollection,
    pub parameter_group_data: TArray<FParameterGroupData>,
    pub substrate_conversion_version: i32,
    pub substrate_version: i32,
    pub b_is_subsutrate_auto_converted_and_not_saved: bool,
}
pub struct UMaterialAggregate {
    pub attributes: TArray<FMaterialAggregateAttribute>,
}
pub struct UMaterialEnumerationProvider {}
pub struct IMaterialEnumerationProvider {}
pub struct UMaterialExpressionLayerStack {
    pub layer_inputs: TArray<FMaterialLayerInput>,
    pub available_layers: TSet<UPtr<UMaterialFunctionInterface>>,
    pub available_blends: TSet<UPtr<UMaterialFunctionInterface>>,
}
pub struct UMaterialExpressionNoise {
    pub position: FExpressionInput,
    pub world_position_origin_type: EPositionOrigin,
    pub filter_width: FExpressionInput,
    pub scale: f32,
    pub quality: i32,
    pub noise_function: ENoiseFunction,
    pub flags_316: u8,
    pub levels: i32,
    pub output_min: f32,
    pub output_max: f32,
    pub level_scale: f32,
    pub flags_336: u8,
    pub repeat_size: u32,
}
pub struct UMaterialExpressionScalarBlueNoise {}
pub struct UMaterialExpressionSubstrateBSDF {}
pub struct UMaterialExpressionSubstrateShadingModels {
    pub base_color: FExpressionInput,
    pub metallic: FExpressionInput,
    pub specular: FExpressionInput,
    pub roughness: FExpressionInput,
    pub anisotropy: FExpressionInput,
    pub emissive_color: FExpressionInput,
    pub normal: FExpressionInput,
    pub tangent: FExpressionInput,
    pub sub_surface_color: FExpressionInput,
    pub clear_coat: FExpressionInput,
    pub clear_coat_roughness: FExpressionInput,
    pub opacity: FExpressionInput,
    pub transmittance_color: FExpressionInput,
    pub water_scattering_coefficients: FExpressionInput,
    pub water_absorption_coefficients: FExpressionInput,
    pub water_phase_g: FExpressionInput,
    pub color_scale_behind_water: FExpressionInput,
    pub clear_coat_normal: FExpressionInput,
    pub custom_tangent: FExpressionInput,
    pub shading_model: FShadingModelMaterialInput,
    pub thin_translucent_surface_coverage: FExpressionInput,
    pub shading_model_override: EMaterialShadingModel,
    pub subsurface_profile: UPtr<USubsurfaceProfile>,
}
pub struct UMaterialExpressionSubstrateSlabBSDF {
    pub diffuse_albedo: FExpressionInput,
    pub f0: FExpressionInput,
    pub f90: FExpressionInput,
    pub roughness: FExpressionInput,
    pub anisotropy: FExpressionInput,
    pub normal: FExpressionInput,
    pub tangent: FExpressionInput,
    pub sssmfp: FExpressionInput,
    pub sssmfp_scale: FExpressionInput,
    pub sss_phase_anisotropy: FExpressionInput,
    pub emissive_color: FExpressionInput,
    pub second_roughness: FExpressionInput,
    pub second_roughness_weight: FExpressionInput,
    pub fuzz_roughness: FExpressionInput,
    pub fuzz_amount: FExpressionInput,
    pub fuzz_color: FExpressionInput,
    pub glint_value: FExpressionInput,
    pub glint_uv: FExpressionInput,
    pub subsurface_profile: UPtr<USubsurfaceProfile>,
    pub specular_profile: UPtr<USpecularProfile>,
    pub flags_1080: u8,
    pub sub_surface_type: EMaterialSubSurfaceType,
}
pub struct UMaterialExpressionSubstrateSimpleClearCoatBSDF {
    pub diffuse_albedo: FExpressionInput,
    pub f0: FExpressionInput,
    pub roughness: FExpressionInput,
    pub clear_coat_coverage: FExpressionInput,
    pub clear_coat_roughness: FExpressionInput,
    pub normal: FExpressionInput,
    pub emissive_color: FExpressionInput,
    pub bottom_normal: FExpressionInput,
}
pub struct UMaterialExpressionSubstrateVolumetricFogCloudBSDF {
    pub albedo: FExpressionInput,
    pub extinction: FExpressionInput,
    pub emissive_color: FExpressionInput,
    pub ambient_occlusion: FExpressionInput,
    pub flags_392: u8,
}
pub struct UMaterialExpressionSubstrateUnlitBSDF {
    pub emissive_color: FExpressionInput,
    pub transmittance_color: FExpressionInput,
    pub normal: FExpressionInput,
}
pub struct UMaterialExpressionSubstrateHairBSDF {
    pub base_color: FExpressionInput,
    pub scatter: FExpressionInput,
    pub specular: FExpressionInput,
    pub roughness: FExpressionInput,
    pub backlit: FExpressionInput,
    pub tangent: FExpressionInput,
    pub emissive_color: FExpressionInput,
}
pub struct UMaterialExpressionSubstrateEyeBSDF {
    pub diffuse_color: FExpressionInput,
    pub roughness: FExpressionInput,
    pub cornea_normal: FExpressionInput,
    pub iris_normal: FExpressionInput,
    pub iris_plane_normal: FExpressionInput,
    pub iris_mask: FExpressionInput,
    pub iris_distance: FExpressionInput,
    pub emissive_color: FExpressionInput,
    pub subsurface_profile: UPtr<USubsurfaceProfile>,
}
pub struct UMaterialExpressionSubstrateSingleLayerWaterBSDF {
    pub base_color: FExpressionInput,
    pub metallic: FExpressionInput,
    pub specular: FExpressionInput,
    pub roughness: FExpressionInput,
    pub normal: FExpressionInput,
    pub emissive_color: FExpressionInput,
    pub top_material_opacity: FExpressionInput,
    pub water_albedo: FExpressionInput,
    pub water_extinction: FExpressionInput,
    pub water_phase_g: FExpressionInput,
    pub color_scale_behind_water: FExpressionInput,
}
pub struct UMaterialExpressionSubstrateLightFunction {
    pub color: FExpressionInput,
}
pub struct UMaterialExpressionSubstratePostProcess {
    pub color: FExpressionInput,
    pub opacity: FExpressionInput,
}
pub struct UMaterialExpressionSubstrateUI {
    pub color: FExpressionInput,
    pub opacity: FExpressionInput,
}
pub struct UMaterialExpressionSubstrateConvertToDecal {
    pub decal_material: FExpressionInput,
    pub coverage: FExpressionInput,
}
pub struct UMaterialExpressionSubstrateConvertMaterialAttributes {
    pub material_attributes: FMaterialAttributesInput,
    pub water_scattering_coefficients: FExpressionInput,
    pub water_absorption_coefficients: FExpressionInput,
    pub water_phase_g: FExpressionInput,
    pub color_scale_behind_water: FExpressionInput,
    pub subsurface_profile: UPtr<USubsurfaceProfile>,
    pub shading_model_override: EMaterialShadingModel,
}
pub struct UMaterialExpressionSubstrateHorizontalMixing {
    pub background: FExpressionInput,
    pub foreground: FExpressionInput,
    pub mix: FExpressionInput,
    pub flags_344: u8,
}
pub struct UMaterialExpressionSubstrateVerticalLayering {
    pub top: FExpressionInput,
    pub base: FExpressionInput,
    pub thickness: FExpressionInput,
    pub flags_344: u8,
}
pub struct UMaterialExpressionSubstrateAdd {
    pub a: FExpressionInput,
    pub b: FExpressionInput,
    pub flags_296: u8,
}
pub struct UMaterialExpressionSubstrateWeight {
    pub a: FExpressionInput,
    pub weight: FExpressionInput,
}
pub struct UMaterialExpressionSubstrateSelect {
    pub a: FExpressionInput,
    pub b: FExpressionInput,
    pub select_value: FExpressionInput,
    pub threshold: f32,
}
pub struct UMaterialExpressionSubstrateUtilityBase {}
pub struct UMaterialExpressionSubstrateTransmittanceToMFP {
    pub transmittance_color: FExpressionInput,
    pub thickness: FExpressionInput,
}
pub struct UMaterialExpressionSubstrateMetalnessToDiffuseAlbedoF0 {
    pub base_color: FExpressionInput,
    pub metallic: FExpressionInput,
    pub specular: FExpressionInput,
}
pub struct UMaterialExpressionSubstrateHazinessToSecondaryRoughness {
    pub base_roughness: FExpressionInput,
    pub haziness: FExpressionInput,
}
pub struct UMaterialExpressionSubstrateThinFilm {
    pub normal: FExpressionInput,
    pub f0: FExpressionInput,
    pub f90: FExpressionInput,
    pub thickness: FExpressionInput,
    pub ior: FExpressionInput,
}
pub struct UMaterialExternalCodeCollection {
    pub external_code_declarations: TArray<FMaterialExternalCodeDeclaration>,
}
pub struct UMaterialInstanceEditorOnlyData {
    pub static_parameters: FStaticParameterSetEditorOnlyData,
}
pub struct UMaterialInstanceDynamic {}
pub struct UMaterialParameterCollection {
    pub state_id: crate::bindings::core_u_object::FGuid,
    pub scalar_parameters: TArray<FCollectionScalarParameter>,
    pub vector_parameters: TArray<FCollectionVectorParameter>,
    pub base: UPtr<UMaterialParameterCollection>,
    pub base_state_id: crate::bindings::core_u_object::FGuid,
    pub scalar_parameter_base_overrides: TMap<
        crate::bindings::core_u_object::FGuid,
        f32,
    >,
    pub vector_parameter_base_overrides: TMap<
        crate::bindings::core_u_object::FGuid,
        crate::bindings::core_u_object::FLinearColor,
    >,
}
pub struct UMeshBudgetProjectSettings {
    pub b_enable_static_mesh_budget: bool,
    pub static_mesh_budget_infos: TArray<FStaticMeshBudgetInfo>,
}
pub struct AMeshMergeCullingVolume {}
pub struct UMeshSimplificationSettings {
    pub mesh_reduction_module_name: FName,
    pub b_mesh_reduction_backward_compatible: bool,
}
pub struct UMeshVertexPainterKismetLibrary {}
pub struct UModel {
    pub polys: UPtr<UPolys>,
    pub surfs: TArray<FBspSurf>,
}
pub struct UMorphTarget {
    pub base_skel_mesh: UPtr<USkeletalMesh>,
}
pub struct ANavigationObjectBase {
    pub capsule_component: UPtr<UCapsuleComponent>,
    pub good_sprite: UPtr<UBillboardComponent>,
    pub bad_sprite: UPtr<UBillboardComponent>,
    pub flags_1168: u8,
}
pub struct UNavMovementInterface {}
pub struct INavMovementInterface {}
pub struct USimulatedClientNetConnection {}
pub struct UNetworkSettings {
    pub flags_104: u8,
    pub network_emulation_profiles: TArray<FNetworkEmulationProfileDescription>,
}
pub struct ABandwidthTestActor {
    pub bandwidth_generator: FBandwidthTestGenerator,
}
pub struct UDataStreamChannel {}
pub struct UEngineReplicationBridge {
    pub object_reference_package_map: UPtr<
        crate::bindings::iris_core::UIrisObjectReferencePackageMap,
    >,
    pub actor_factory_name: FName,
    pub sub_object_factory_name: FName,
}
pub struct UNetActorFactory {}
pub struct UNetSubObjectFactory {}
pub struct UNetFaultConfig {}
pub struct UNetPushModelHelpers {}
pub struct UNetworkMetricsConfig {
    pub listeners: TArray<FNetworkMetricConfig>,
}
pub struct UNetworkMetricsDatabase {}
pub struct UNetworkMetricsBaseListener {
    pub interval_seconds: f64,
    pub mutators: TArray<crate::bindings::core_u_object::FInstancedStruct>,
}
pub struct UNetworkMetricsLog {}
pub struct UNetworkMetricsCSV {}
pub struct UNetworkMetricsCSV_Replication {}
pub struct UNetworkMetricsPerfCounters {}
pub struct UNetworkMetricsStats {}
pub struct UReplicatedObjectInterface {}
pub struct IReplicatedObjectInterface {}
pub struct URPCDoSDetectionConfig {
    pub b_rpc_do_s_detection: bool,
    pub b_rpc_do_s_analytics: bool,
    pub hitch_time_quota_ms: i32,
    pub hitch_suspend_detection_time_ms: i32,
    pub detection_severity: TArray<FString>,
    pub initial_connect_tolerance_ms: i32,
    pub rpc_block_whitelist: TArray<FName>,
    pub rpc_block_allowlist: TArray<FName>,
    pub rpc_analytics_thresholds: TArray<FRPCAnalyticsThreshold>,
    pub rpc_analytics_override_chance: f64,
}
pub struct UNetworkSubsystem {}
pub struct ANote {
    pub text: FString,
    pub sprite_component: UPtr<UBillboardComponent>,
    pub arrow_component: UPtr<UArrowComponent>,
}
pub struct UValkyrieMetaData {
    pub development_status_key: FName,
    pub deprecation_status_key: FName,
}
pub struct UObjectLibrary {
    pub object_base_class: TSubclassOf<crate::bindings::core_u_object::UObject>,
    pub b_has_blueprint_classes: bool,
    pub objects: TArray<UPtr<crate::bindings::core_u_object::UObject>>,
    pub weak_objects: TArray<TWeakObjectPtr<crate::bindings::core_u_object::UObject>>,
    pub b_use_weak_references: bool,
    pub b_is_fully_loaded: bool,
}
pub struct UObjectReferencer {
    pub referenced_objects: TArray<UPtr<crate::bindings::core_u_object::UObject>>,
}
pub struct UObjectTraceWorldSubsystem {}
pub struct UOnlineBlueprintCallProxyBase {}
pub struct UOnlineEngineInterface {}
pub struct UOnlineSession {}
pub struct UPackageMapClient {}
pub struct APackedLevelActor {
    pub packed_version: crate::bindings::core_u_object::FGuid,
    pub packed_hash: u32,
}
pub struct UEngineHandlerComponentFactory {}
pub struct APainCausingVolume {
    pub flags_1224: u8,
    pub damage_per_sec: f32,
    pub damage_type: TSubclassOf<UDamageType>,
    pub pain_interval: f32,
    pub flags_1244: u8,
    pub damage_instigator: UPtr<AController>,
}
pub struct AParticleEventManager {}
pub struct UParticleLODLevel {
    pub level: i32,
    pub flags_52: u8,
    pub required_module: UPtr<UParticleModuleRequired>,
    pub modules: TArray<UPtr<UParticleModule>>,
    pub type_data_module: UPtr<UParticleModuleTypeDataBase>,
    pub spawn_module: UPtr<UParticleModuleSpawn>,
    pub event_generator: UPtr<UParticleModuleEventGenerator>,
    pub spawning_modules: TArray<UPtr<UParticleModuleSpawnBase>>,
    pub spawn_modules: TArray<UPtr<UParticleModule>>,
    pub update_modules: TArray<UPtr<UParticleModule>>,
    pub orbit_modules: TArray<UPtr<UParticleModuleOrbit>>,
    pub event_receiver_modules: TArray<UPtr<UParticleModuleEventReceiverBase>>,
    pub flags_184: u8,
    pub peak_active_particles: i32,
}
pub struct UFXSystemComponent {}
pub struct UParticleSystemComponent {
    pub template: UPtr<UParticleSystem>,
    pub emitter_materials: TArray<UPtr<UMaterialInterface>>,
    pub skel_mesh_components: TArray<UPtr<USkeletalMeshComponent>>,
    pub flags_1553: u8,
    pub flags_1554: u8,
    pub lod_method: ParticleSystemLODMethod,
    pub required_significance: EParticleSignificanceLevel,
    pub instance_parameters: TArray<FParticleSysParam>,
    pub on_particle_spawn: FParticleSystemComponent_OnParticleSpawn,
    pub on_particle_burst: FParticleSystemComponent_OnParticleBurst,
    pub on_particle_death: FParticleSystemComponent_OnParticleDeath,
    pub on_particle_collide: FParticleSystemComponent_OnParticleCollide,
    pub b_old_position_valid: bool,
    pub old_position: crate::bindings::core_u_object::FVector,
    pub part_sys_velocity: crate::bindings::core_u_object::FVector,
    pub warmup_time: f32,
    pub warmup_tick_rate: f32,
    pub seconds_before_inactive: f32,
    pub max_time_before_force_update_transform: f32,
    pub editor_lod_level: i32,
    pub editor_detail_mode: i32,
    pub replay_clips: TArray<UPtr<UParticleSystemReplay>>,
    pub custom_time_dilation: f32,
    pub auto_attach_parent: TWeakObjectPtr<USceneComponent>,
    pub auto_attach_socket_name: FName,
    pub auto_attach_location_type_deprecated: EAttachLocation,
    pub auto_attach_location_rule: EAttachmentRule,
    pub auto_attach_rotation_rule: EAttachmentRule,
    pub auto_attach_scale_rule: EAttachmentRule,
    pub on_system_finished: FParticleSystemComponent_OnSystemFinished,
}
pub struct USubUVAnimation {
    pub sub_uv_texture: UPtr<UTexture2D>,
    pub sub_images_horizontal: i32,
    pub sub_images_vertical: i32,
    pub bounding_mode: ESubUVBoundingVertexCount,
    pub opacity_source_mode: EOpacitySourceMode,
    pub alpha_threshold: f32,
}
pub struct UAsyncPhysicsInputComponent {
    pub data_class: TSubclassOf<UAsyncPhysicsData>,
    pub buffered_data: TArray<UPtr<UAsyncPhysicsData>>,
    pub data_to_consume: UPtr<UAsyncPhysicsData>,
    pub data_to_write: UPtr<UAsyncPhysicsData>,
}
pub struct AClusterUnionActor {
    pub cluster_union: UPtr<UClusterUnionComponent>,
}
pub struct UClusterUnionComponent {
    pub b_enable_damage_from_collision: bool,
    pub on_component_added_event: FClusterUnionComponent_OnComponentAddedEvent,
    pub on_component_removed_event: FClusterUnionComponent_OnComponentRemovedEvent,
    pub on_component_bounds_changed_event: FClusterUnionComponent_OnComponentBoundsChangedEvent,
    pub clustered_components_references: TArray<FComponentReference>,
    pub gravity_group_index_override: i32,
    pub replicated_rigid_state: FClusterUnionReplicatedData,
}
pub struct UClusterUnionReplicatedProxyComponent {
    pub parent_cluster_union: TWeakObjectPtr<UClusterUnionComponent>,
    pub child_clustered_component: TWeakObjectPtr<UPrimitiveComponent>,
    pub particle_bone_ids: TArray<i32>,
    pub particle_child_to_parents: TArray<crate::bindings::core_u_object::FTransform>,
    pub b_net_update_parent_cluster_union: bool,
    pub b_net_update_child_clustered_component: bool,
    pub b_net_update_particle_bone_ids: bool,
    pub b_net_update_particle_child_to_parents: bool,
    pub b_is_pending_deletion: bool,
}
pub struct UConstraintInstanceBlueprintLibrary {}
pub struct UPhysicsQueryHandler {}
pub struct UDefaultPhysicsQueryHandler {}
pub struct UChaosBlueprintLibrary {}
pub struct UChaosEventRelay {
    pub on_collision_event: FChaosEventRelay_OnCollisionEvent,
    pub on_break_event: FChaosEventRelay_OnBreakEvent,
    pub on_removal_event: FChaosEventRelay_OnRemovalEvent,
    pub on_crumbling_event: FChaosEventRelay_OnCrumblingEvent,
}
pub struct UNetworkPhysicsSystem {}
pub struct UNetworkPhysicsComponent {
    pub replicated_delta_source_input: FNetworkPhysicsRewindDataDeltaSourceInputProxy,
    pub replicated_delta_source_state: FNetworkPhysicsRewindDataDeltaSourceStateProxy,
    pub replicated_important_input: FNetworkPhysicsRewindDataImportantInputProxy,
    pub replicated_important_state: FNetworkPhysicsRewindDataImportantStateProxy,
    pub replicated_inputs: FNetworkPhysicsRewindDataInputProxy,
    pub replicated_remote_inputs: FNetworkPhysicsRewindDataRemoteInputProxy,
    pub replicated_states: FNetworkPhysicsRewindDataStateProxy,
    pub replicated_input_collection: FNetworkPhysicsDataCollection,
    pub replicated_remote_input_collection: FNetworkPhysicsDataCollection,
    pub replicated_state_collection: FNetworkPhysicsDataCollection,
    pub inputs_to_network_owner: u16,
}
pub struct UNetworkPhysicsSettingsDataAsset {
    pub settings: FNetworkPhysicsSettingsData,
}
pub struct UNetworkPhysicsSettingsComponent {
    pub settings_data_asset: UPtr<UNetworkPhysicsSettingsDataAsset>,
    pub general_settings: FNetworkPhysicsSettings,
    pub default_replication_settings: FNetworkPhysicsSettingsDefaultReplication,
    pub predictive_interpolation_settings: FNetworkPhysicsSettingsPredictiveInterpolation,
    pub resimulation_settings: FNetworkPhysicsSettingsResimulation,
    pub network_physics_component_settings: FNetworkPhysicsSettingsNetworkPhysicsComponent,
}
pub struct UPhysicalAnimationComponent {
    pub strength_multiplyer: f32,
    pub skeletal_mesh_component: UPtr<USkeletalMeshComponent>,
}
pub struct UPhysicalMaterialMask {
    pub asset_import_data: UPtr<UAssetImportData>,
    pub mask_texture: UPtr<UTexture>,
    pub uv_channel_index: i32,
    pub address_x: TextureAddress,
    pub address_y: TextureAddress,
}
pub struct UPhysicsAsset {
    pub default_skel_mesh_deprecated: UPtr<USkeletalMesh>,
    pub preview_skeletal_mesh: TSoftObjectPtr<USkeletalMesh>,
    pub physical_animation_profiles: TArray<FName>,
    pub constraint_profiles: TArray<FName>,
    pub editor_body_flags: TArray<i32>,
    pub current_physical_animation_profile_name: FName,
    pub current_constraint_profile_name: FName,
    pub bounds_bodies: TArray<i32>,
    pub skeletal_body_setups: TArray<UPtr<USkeletalBodySetup>>,
    pub constraint_setup: TArray<UPtr<UPhysicsConstraintTemplate>>,
    pub solver_settings: FPhysicsAssetSolverSettings,
    pub solver_iterations: FSolverIterations,
    pub solver_type: EPhysicsAssetSolverType,
    pub flags_293: u8,
    pub thumbnail_info: UPtr<UThumbnailInfo>,
    pub body_setup_deprecated: TArray<UPtr<UBodySetup>>,
    pub asset_user_data: TArray<UPtr<UAssetUserData>>,
}
pub struct UPhysicsBodyInstanceOwnerResolver {}
pub struct IPhysicsBodyInstanceOwnerResolver {}
pub struct UPhysicsCollisionHandler {
    pub impact_threshold: f32,
    pub impact_re_fire_delay: f32,
    pub default_impact_sound: UPtr<USoundBase>,
    pub last_impact_sound_time: f32,
}
pub struct APhysicsConstraintActor {
    pub constraint_comp: UPtr<UPhysicsConstraintComponent>,
    pub constraint_actor1_deprecated: UPtr<AActor>,
    pub constraint_actor2_deprecated: UPtr<AActor>,
    pub flags_1160: u8,
}
pub struct UPhysicsConstraintComponent {
    pub constraint_actor1: UPtr<AActor>,
    pub component_name1: FConstrainComponentPropName,
    pub constraint_actor2: UPtr<AActor>,
    pub component_name2: FConstrainComponentPropName,
    pub constraint_setup_deprecated: UPtr<UPhysicsConstraintTemplate>,
    pub on_constraint_broken: FPhysicsConstraintComponent_OnConstraintBroken,
    pub on_plastic_deformation: FPhysicsConstraintComponent_OnPlasticDeformation,
    pub constraint_instance: FConstraintInstance,
}
pub struct UPhysicsConstraintTemplate {
    pub default_instance: FConstraintInstance,
    pub profile_handles: TArray<FPhysicsConstraintProfileHandle>,
    pub default_profile: FConstraintProfileProperties,
    pub joint_name_deprecated: FName,
    pub constraint_bone1_deprecated: FName,
    pub constraint_bone2_deprecated: FName,
    pub pos1_deprecated: crate::bindings::core_u_object::FVector,
    pub pri_axis1_deprecated: crate::bindings::core_u_object::FVector,
    pub sec_axis1_deprecated: crate::bindings::core_u_object::FVector,
    pub pos2_deprecated: crate::bindings::core_u_object::FVector,
    pub pri_axis2_deprecated: crate::bindings::core_u_object::FVector,
    pub sec_axis2_deprecated: crate::bindings::core_u_object::FVector,
    pub flags_1544: u8,
    pub projection_linear_tolerance_deprecated: f32,
    pub projection_angular_tolerance_deprecated: f32,
    pub linear_x_motion_deprecated: crate::bindings::physics_core::ELinearConstraintMotion,
    pub linear_y_motion_deprecated: crate::bindings::physics_core::ELinearConstraintMotion,
    pub linear_z_motion_deprecated: crate::bindings::physics_core::ELinearConstraintMotion,
    pub linear_limit_size_deprecated: f32,
    pub flags_1564: u8,
    pub linear_limit_stiffness_deprecated: f32,
    pub linear_limit_damping_deprecated: f32,
    pub flags_1576: u8,
    pub linear_break_threshold_deprecated: f32,
    pub angular_swing1_motion_deprecated: crate::bindings::physics_core::EAngularConstraintMotion,
    pub angular_swing2_motion_deprecated: crate::bindings::physics_core::EAngularConstraintMotion,
    pub angular_twist_motion_deprecated: crate::bindings::physics_core::EAngularConstraintMotion,
    pub flags_1588: u8,
    pub swing1_limit_angle_deprecated: f32,
    pub swing2_limit_angle_deprecated: f32,
    pub twist_limit_angle_deprecated: f32,
    pub swing_limit_stiffness_deprecated: f32,
    pub swing_limit_damping_deprecated: f32,
    pub twist_limit_stiffness_deprecated: f32,
    pub twist_limit_damping_deprecated: f32,
    pub flags_1620: u8,
    pub angular_break_threshold_deprecated: f32,
}
pub struct UPhysicsHandleComponent {
    pub grabbed_component: UPtr<UPrimitiveComponent>,
    pub flags_260: u8,
    pub linear_damping: f32,
    pub linear_stiffness: f32,
    pub angular_damping: f32,
    pub angular_stiffness: f32,
    pub interpolation_speed: f32,
}
pub struct UPhysicsObjectBlueprintLibrary {}
pub struct UPhysicsSettings {
    pub physics_prediction: FPhysicsPredictionSettings,
    pub physic_error_correction: FRigidBodyErrorCorrection,
    pub locked_axis_deprecated: ESettingsLockedAxis,
    pub default_degrees_of_freedom: ESettingsDOF,
    pub b_suppress_face_remap_table: bool,
    pub b_support_uv_from_hit_results: bool,
    pub b_disable_active_actors: bool,
    pub b_disable_kinematic_static_pairs: bool,
    pub b_disable_kinematic_kinematic_pairs: bool,
    pub b_disable_ccd: bool,
    pub anim_physics_min_delta_time: f32,
    pub b_simulate_anim_physics_after_reset: bool,
    pub min_physics_delta_time: f32,
    pub max_physics_delta_time: f32,
    pub b_substepping: bool,
    pub b_substepping_async: bool,
    pub b_tick_physics_async: bool,
    pub async_fixed_time_step_size: f32,
    pub max_substep_delta_time: f32,
    pub max_substeps: i32,
    pub sync_scene_smoothing_factor: f32,
    pub initial_average_frame_rate: f32,
    pub phys_x_tree_rebuild_rate: i32,
    pub physical_surfaces: TArray<FPhysicalSurfaceName>,
    pub default_broadphase_settings: FBroadphaseSettings,
    pub min_delta_velocity_for_hit_events: f32,
    pub chaos_settings: FChaosPhysicsSettings,
}
pub struct APhysicsThruster {
    pub thruster_component: UPtr<UPhysicsThrusterComponent>,
    pub arrow_component: UPtr<UArrowComponent>,
}
pub struct URadialForceComponent {
    pub radius: f32,
    pub falloff: crate::bindings::physics_core::ERadialImpulseFalloff,
    pub impulse_strength: f32,
    pub flags_668: u8,
    pub force_strength: f32,
    pub destructible_damage: f32,
    pub object_types_to_affect: TArray<EObjectTypeQuery>,
}
pub struct USkeletalBodySetup {
    pub current_physical_animation_profile: FPhysicalAnimationProfile,
    pub b_skip_scale_from_animation: bool,
    pub physical_animation_data: TArray<FPhysicalAnimationProfile>,
}
pub struct UPhysicsFieldComponent {}
pub struct UPhysicsFieldStatics {}
pub struct APlayerCameraManager {
    pub pc_owner: UPtr<APlayerController>,
    pub transform_component: UPtr<USceneComponent>,
    pub default_fov: f32,
    pub default_ortho_width: f32,
    pub default_aspect_ratio: f32,
    pub view_target: FTViewTarget,
    pub pending_view_target: FTViewTarget,
    pub camera_cache_private: FCameraCacheEntry,
    pub last_frame_camera_cache_private: FCameraCacheEntry,
    pub modifier_list: TArray<UPtr<UCameraModifier>>,
    pub default_modifiers: TArray<TSubclassOf<UCameraModifier>>,
    pub free_cam_distance: f32,
    pub free_cam_offset: crate::bindings::core_u_object::FVector,
    pub view_target_offset: crate::bindings::core_u_object::FVector,
    pub on_audio_fade_change_event: FPlayerCameraManager_OnAudioFadeChangeEvent,
    pub camera_lens_effects: TArray<TScriptInterface<ICameraLensEffectInterface>>,
    pub cached_camera_shake_mod: UPtr<UCameraModifier_CameraShake>,
    pub post_process_blend_cache: TArray<FPostProcessSettings>,
    pub anim_camera_actor: UPtr<ACameraActor>,
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
    pub server_update_camera_timeout: f32,
}
pub struct ANoPawnPlayerController {}
pub struct APlayerStart {
    pub player_start_tag: FName,
    pub arrow_component: UPtr<UArrowComponent>,
}
pub struct APlayerStartPIE {}
pub struct APlayerState {
    pub score: f32,
    pub player_id: i32,
    pub compressed_ping: u8,
    pub flags_1154: u8,
    pub start_time: i32,
    pub unique_id: FUniqueNetIdRepl,
    pub engine_message_class: TSubclassOf<ULocalMessage>,
    pub saved_network_address: FString,
    pub on_pawn_set: FPlayerState_OnPawnSet,
    pub pawn_private: UPtr<APawn>,
    pub player_name_private: FString,
}
pub struct UPluginBlueprintLibrary {}
pub struct APostProcessVolume {
    pub settings: FPostProcessSettings,
    pub priority: f32,
    pub blend_radius: f32,
    pub blend_weight: f32,
    pub flags_3180: u8,
    pub volume_guid: crate::bindings::core_u_object::FGuid,
}
pub struct APrecomputedVisibilityVolume {}
pub struct UPrimaryAssetLabel {
    pub rules: FPrimaryAssetRules,
    pub flags_84: u8,
    pub explicit_assets: TArray<TSoftObjectPtr<crate::bindings::core_u_object::UObject>>,
    pub explicit_blueprints: TArray<
        TSoftObjectPtr<crate::bindings::core_u_object::UClass>,
    >,
    pub asset_collection: FCollectionReference,
}
pub struct UHealthSnapshotBlueprintLibrary {}
pub struct ULevelStreamingProfilingSubsystem {}
pub struct UProxyLODMeshSimplificationSettings {
    pub proxy_lod_mesh_reduction_module_name: FName,
}
pub struct ARectLight {
    pub rect_light_component: UPtr<URectLightComponent>,
}
pub struct URendererSettings {
    pub mobile_shading_path: EMobileShadingPath,
    pub mobile_anti_aliasing: EMobileAntiAliasingMethod,
    pub mobile_float_precision_mode: EMobileFloatPrecisionMode,
    pub b_mobile_allow_dithered_lod_transition: bool,
    pub flags_108: u8,
    pub shader_compression_format_deprecated: EShaderCompressionFormat,
    pub flags_116: u8,
    pub min_screen_radius_for_lights: f32,
    pub min_screen_radius_for_early_z_pass: f32,
    pub flags_128: u8,
    pub virtual_texture_tile_size: u32,
    pub virtual_texture_tile_border_size: u32,
    pub flags_140: u8,
    pub mesh_paint_virtual_texture_tile_size: u32,
    pub mesh_paint_virtual_texture_tile_border_size: u32,
    pub mesh_paint_virtual_texture_use_compression: bool,
    pub mesh_paint_default_on_static_mesh: bool,
    pub mesh_paint_default_texels_per_vertex: i32,
    pub mesh_paint_virtual_texture_max_texture_size: i32,
    pub b_enable_rvt_base_color: bool,
    pub b_enable_rvt_base_color_roughness: bool,
    pub b_enable_rvt_base_color_specular: bool,
    pub b_enable_rvt_mask4: bool,
    pub b_enable_rvt_world_height: bool,
    pub b_enable_rvt_displacement: bool,
    pub b_use_high_quality_rvt_height_sampling: bool,
    pub working_color_space_choice: EWorkingColorSpace,
    pub red_chromaticity_coordinate: crate::bindings::core_u_object::FVector2D,
    pub green_chromaticity_coordinate: crate::bindings::core_u_object::FVector2D,
    pub blue_chromaticity_coordinate: crate::bindings::core_u_object::FVector2D,
    pub white_chromaticity_coordinate: crate::bindings::core_u_object::FVector2D,
    pub flags_240: u8,
    pub dynamic_global_illumination: EDynamicGlobalIlluminationMethod,
    pub reflections: EReflectionMethod,
    pub reflection_capture_resolution: i32,
    pub flags_252: u8,
    pub lumen_ray_lighting_mode: ELumenRayLightingMode,
    pub flags_260: u8,
    pub lumen_software_tracing_mode: ELumenSoftwareTracingMode,
    pub lumen_screen_tracing_source: ELumenScreenTracingSource,
    pub flags_268: u8,
    pub shadow_map_method: EShadowMapMethod,
    pub flags_276: u8,
    pub distance_field_voxel_density: f32,
    pub flags_284: u8,
    pub translucent_sort_policy: ETranslucentSortPolicy,
    pub translucent_sort_axis: crate::bindings::core_u_object::FVector,
    pub flags_320: u8,
    pub foveation_level: EFixedFoveationLevels,
    pub flags_328: u8,
    pub custom_depth_stencil: ECustomDepthStencil,
    pub flags_336: u8,
    pub default_feature_auto_exposure: EAutoExposureMethodUI,
    pub default_feature_auto_exposure_bias: f32,
    pub flags_348: u8,
    pub default_feature_local_exposure_highlight_contrast: f32,
    pub default_feature_local_exposure_shadow_contrast: f32,
    pub flags_360: u8,
    pub default_feature_anti_aliasing: EAntiAliasingMethod,
    pub msaa_sample_count: ECompositingSampleCount,
    pub default_light_units: ELightUnits,
    pub default_back_buffer_pixel_format: EDefaultBackBufferPixelFormat,
    pub default_manual_screen_percentage: f32,
    pub default_screen_percentage_desktop_mode: EScreenPercentageMode,
    pub default_screen_percentage_mobile_mode: EScreenPercentageMode,
    pub default_screen_percentage_vr_mode: EScreenPercentageMode,
    pub default_screen_percentage_path_tracer_mode: EScreenPercentageMode,
    pub flags_388: u8,
    pub early_z_pass: EEarlyZPass,
    pub flags_396: u8,
    pub clear_scene_method: EClearSceneOptions,
    pub velocity_pass: EVelocityOutputPass,
    pub vertex_deformation_outputs_velocity: EVertexDeformationOutputsVelocity,
    pub flags_404: u8,
    pub gpu_simulation_texture_size_x: i32,
    pub gpu_simulation_texture_size_y: i32,
    pub flags_416: u8,
    pub g_buffer_format: EGBufferFormat,
    pub flags_424: u8,
    pub morph_target_max_blend_weight: f32,
    pub flags_432: u8,
    pub light_function_atlas_pixel_format: ELightFunctionAtlasPixelFormat,
    pub flags_440: u8,
    pub flags_441: u8,
    pub flags_442: u8,
    pub wireframe_cull_threshold: f32,
    pub flags_448: u8,
    pub substrate_g_buffer_format: ESubstrateStorageFormat,
    pub substrate_project_closures_per_pixel: u32,
    pub substrate_closure_per_pixel_override: ESubstrateClosureConfig,
    pub substrate_project_bytes_per_pixel: u32,
    pub flags_468: u8,
    pub flags_469: u8,
    pub default_skin_cache_behavior: ESkinCacheDefaultBehavior,
    pub skin_cache_scene_memory_limit_in_mb: f32,
    pub flags_480: u8,
    pub mobile_local_light_setting: EMobileLocalLightSetting,
    pub flags_488: u8,
    pub flags_489: u8,
    pub unlimited_bon_influences_threshold: i32,
    pub default_bone_influence_limit: crate::bindings::core_u_object::FPerPlatformInt,
    pub max_skin_bones: crate::bindings::core_u_object::FPerPlatformInt,
    pub mobile_planar_reflection_mode: u8,
    pub flags_676: u8,
    pub b_stream_skeletal_mesh_lo_ds: crate::bindings::core_u_object::FPerPlatformBool,
    pub b_discard_skeletal_mesh_optional_lo_ds: crate::bindings::core_u_object::FPerPlatformBool,
    pub visualize_calibration_color_material_path: crate::bindings::core_u_object::FSoftObjectPath,
    pub visualize_calibration_custom_material_path: crate::bindings::core_u_object::FSoftObjectPath,
    pub visualize_calibration_grayscale_material_path: crate::bindings::core_u_object::FSoftObjectPath,
}
pub struct URendererOverrideSettings {
    pub flags_104: u8,
}
pub struct UNeuralProfile {
    pub settings: FNeuralProfileStruct,
    pub guid: crate::bindings::core_u_object::FGuid,
}
pub struct USkeletalMeshHalfEdgeBufferAccessor {}
pub struct ISkeletalMeshHalfEdgeBufferAccessor {}
pub struct USpecularProfile {
    pub settings: FSpecularProfileStruct,
    pub guid: crate::bindings::core_u_object::FGuid,
}
pub struct USubsurfaceProfile {
    pub settings: FSubsurfaceProfileStruct,
    pub guid: crate::bindings::core_u_object::FGuid,
}
pub struct UReplayNetConnection {}
pub struct UGameInstanceSubsystem {}
pub struct UReplaySubsystem {
    pub b_load_default_map_on_stop: bool,
}
pub struct UReverbEffect {
    pub b_bypass_early_reflections: bool,
    pub reflections_delay: f32,
    pub gain_hf: f32,
    pub reflections_gain: f32,
    pub b_bypass_late_reflections: bool,
    pub late_delay: f32,
    pub decay_time: f32,
    pub density: f32,
    pub diffusion: f32,
    pub air_absorption_gain_hf: f32,
    pub decay_hf_ratio: f32,
    pub late_gain: f32,
    pub gain: f32,
    pub room_rolloff_factor: f32,
    pub flags_104: u8,
}
pub struct URuntimeOptionsBase {}
pub struct UScene {}
pub struct USCS_Node {
    pub component_class: TSubclassOf<crate::bindings::core_u_object::UObject>,
    pub component_template: UPtr<UActorComponent>,
    pub cooked_component_instancing_data: FBlueprintCookedComponentInstancingData,
    pub category_name: FText,
    pub attach_to_name: FName,
    pub parent_component_or_variable_name: FName,
    pub parent_component_owner_class_name: FName,
    pub b_is_parent_component_native: bool,
    pub child_nodes: TArray<UPtr<USCS_Node>>,
    pub meta_data_array: TArray<FBPVariableMetaDataEntry>,
    pub variable_guid: crate::bindings::core_u_object::FGuid,
    pub b_is_native_deprecated: bool,
    pub native_component_name_deprecated: FName,
    pub b_variable_name_auto_generated_deprecated: bool,
    pub internal_variable_name: FName,
}
pub struct USimpleConstructionScript {
    pub root_nodes: TArray<UPtr<USCS_Node>>,
    pub all_nodes: TArray<UPtr<USCS_Node>>,
    pub default_scene_root_node: UPtr<USCS_Node>,
    pub root_node_deprecated: UPtr<USCS_Node>,
    pub actor_component_nodes_deprecated: TArray<UPtr<USCS_Node>>,
}
pub struct USkinnedAsset {}
pub struct USkeletalMesh {
    pub source_models: TArray<FSkeletalMeshSourceModel>,
    pub skeleton: UPtr<USkeleton>,
    pub imported_bounds: crate::bindings::core_u_object::FBoxSphereBounds,
    pub extended_bounds: crate::bindings::core_u_object::FBoxSphereBounds,
    pub positive_bounds_extension: crate::bindings::core_u_object::FVector,
    pub negative_bounds_extension: crate::bindings::core_u_object::FVector,
    pub materials: TArray<FSkeletalMaterial>,
    pub skel_mirror_table: TArray<FBoneMirrorInfo>,
    pub nanite_settings: FMeshNaniteSettings,
    pub assembly_reference_cache: TArray<UPtr<USkeletalMesh>>,
    pub min_quality_level_lod: FPerQualityLevelInt,
    pub min_lod: crate::bindings::core_u_object::FPerPlatformInt,
    pub disable_below_min_lod_stripping: crate::bindings::core_u_object::FPerPlatformBool,
    pub b_override_lod_streaming_settings: bool,
    pub b_support_lod_streaming: crate::bindings::core_u_object::FPerPlatformBool,
    pub max_num_streamed_lo_ds: crate::bindings::core_u_object::FPerPlatformInt,
    pub max_num_optional_lo_ds: crate::bindings::core_u_object::FPerPlatformInt,
    pub lod_settings: UPtr<USkeletalMeshLODSettings>,
    pub default_animating_rig: TSoftObjectPtr<crate::bindings::core_u_object::UObject>,
    pub skel_mirror_axis: crate::bindings::core_u_object::EAxis,
    pub skel_mirror_flip_axis: crate::bindings::core_u_object::EAxis,
    pub flags_1346: u8,
    pub vertex_color_guid: crate::bindings::core_u_object::FGuid,
    pub body_setup: UPtr<UBodySetup>,
    pub physics_asset: UPtr<UPhysicsAsset>,
    pub shadow_physics_asset: UPtr<UPhysicsAsset>,
    pub node_mapping_data: TArray<UPtr<UNodeMappingContainer>>,
    pub asset_import_data: UPtr<UAssetImportData>,
    pub source_file_path_deprecated: FString,
    pub source_file_timestamp_deprecated: FString,
    pub thumbnail_info: UPtr<UThumbnailInfo>,
    pub b_has_custom_default_editor_camera: bool,
    pub default_editor_camera_location: crate::bindings::core_u_object::FVector,
    pub default_editor_camera_rotation: crate::bindings::core_u_object::FRotator,
    pub default_editor_camera_look_at: crate::bindings::core_u_object::FVector,
    pub default_editor_camera_ortho_zoom: f32,
    pub preview_attached_asset_container: FPreviewAssetAttachContainer,
    pub flags_1564: u8,
    pub ray_tracing_min_lod: i32,
    pub cloth_lod_bias_mode: EClothLODBiasMode,
    pub morph_targets: TArray<UPtr<UMorphTarget>>,
    pub floor_offset: f32,
    pub clothing_assets_deprecated: TArray<FClothingAssetData_Legacy>,
    pub bone_draw_size: f32,
    pub post_process_anim_blueprint: TSubclassOf<UAnimInstance>,
    pub post_process_anim_bplod_threshold: i32,
    pub mesh_clothing_assets: TArray<
        UPtr<crate::bindings::clothing_system_runtime_interface::UClothingAssetBase>,
    >,
    pub sampling_info: FSkeletalMeshSamplingInfo,
    pub asset_user_data: TArray<UPtr<UAssetUserData>>,
    pub asset_user_data_editor_only: TArray<UPtr<UAssetUserData>>,
    pub sockets: TArray<UPtr<USkeletalMeshSocket>>,
    pub skin_weight_profiles: TArray<FSkinWeightProfileInfo>,
    pub default_mesh_deformer: UPtr<UMeshDeformer>,
    pub target_mesh_deformers: UPtr<UMeshDeformerCollection>,
    pub overlay_material: UPtr<UMaterialInterface>,
    pub overlay_material_max_draw_distance: f32,
    pub forward_axis: crate::bindings::core_u_object::EAxis,
    pub mesh_editor_data_object_deprecated: UPtr<USkeletalMeshEditorData>,
    pub lod_info_deprecated: TArray<FSkeletalMeshLODInfo>,
}
pub struct USkeletalMeshEditorData {}
pub struct USkeletalMeshLODSettings {
    pub min_quality_level_lod: FPerQualityLevelInt,
    pub min_lod: crate::bindings::core_u_object::FPerPlatformInt,
    pub disable_below_min_lod_stripping: crate::bindings::core_u_object::FPerPlatformBool,
    pub b_override_lod_streaming_settings: bool,
    pub b_support_lod_streaming: crate::bindings::core_u_object::FPerPlatformBool,
    pub max_num_streamed_lo_ds: crate::bindings::core_u_object::FPerPlatformInt,
    pub max_num_optional_lo_ds: crate::bindings::core_u_object::FPerPlatformInt,
    pub lod_groups: TArray<FSkeletalMeshLODGroupSettings>,
}
pub struct USkeletalMeshSimplificationSettings {
    pub skeletal_mesh_reduction_module_name: FName,
}
pub struct USkeletalMeshDescriptionBulkData {}
pub struct UButtonStyleAsset {
    pub button_style: crate::bindings::slate_core::FButtonStyle,
}
pub struct UCheckBoxStyleAsset {
    pub check_box_style: crate::bindings::slate_core::FCheckBoxStyle,
}
pub struct USlateBrushAsset {
    pub brush: crate::bindings::slate_core::FSlateBrush,
}
pub struct USlateTextureAtlasInterface {}
pub struct ISlateTextureAtlasInterface {}
pub struct USoundAttenuation {
    pub attenuation: FSoundAttenuationSettings,
}
pub struct USoundClass {
    pub properties: FSoundClassProperties,
    pub child_classes: TArray<UPtr<USoundClass>>,
    pub passive_sound_mix_modifiers: TArray<FPassiveSoundMixModifier>,
    pub parent_class: UPtr<USoundClass>,
    pub asset_user_data: TArray<UPtr<UAssetUserData>>,
}
pub struct USoundConcurrency {
    pub concurrency: FSoundConcurrencySettings,
}
pub struct USoundCue {
    pub first_node: UPtr<USoundNode>,
    pub volume_multiplier: f32,
    pub pitch_multiplier: f32,
    pub attenuation_overrides: FSoundAttenuationSettings,
    pub all_nodes: TArray<UPtr<USoundNode>>,
    pub sound_cue_graph: UPtr<UEdGraph>,
    pub subtitle_priority: f32,
    pub flags_1592: u8,
    pub cooked_quality_index: i32,
}
pub struct USoundGroups {
    pub sound_group_profiles: TArray<FSoundGroup>,
}
pub struct USoundMix {
    pub flags_48: u8,
    pub eq_priority: f32,
    pub eq_settings: FAudioEQEffect,
    pub sound_class_effects: TArray<FSoundClassAdjuster>,
    pub initial_delay: f32,
    pub fade_in_time: f32,
    pub duration: f32,
    pub fade_out_time: f32,
    pub flags_152: u8,
}
pub struct USoundNode {
    pub child_nodes: TArray<UPtr<USoundNode>>,
    pub graph_node: UPtr<UEdGraphNode>,
}
pub struct USoundNodeAssetReferencer {}
pub struct USoundNodeAttenuation {
    pub attenuation_settings: UPtr<USoundAttenuation>,
    pub attenuation_overrides: FSoundAttenuationSettings,
    pub flags_1120: u8,
}
pub struct USoundNodeBranch {
    pub bool_parameter_name: FName,
}
pub struct USoundNodeConcatenator {
    pub input_volume: TArray<f32>,
}
pub struct USoundNodeDelay {
    pub delay_min: f32,
    pub delay_max: f32,
}
pub struct USoundNodeDialoguePlayer {
    pub dialogue_wave_parameter: FDialogueWaveParameter,
    pub flags_120: u8,
}
pub struct USoundNodeDistanceCrossFade {
    pub cross_fade_input: TArray<FDistanceDatum>,
}
pub struct USoundNodeDoppler {
    pub doppler_intensity: f32,
    pub b_use_smoothing: bool,
    pub smoothing_interp_speed: f32,
}
pub struct USoundNodeEnveloper {
    pub loop_start: f32,
    pub loop_end: f32,
    pub duration_after_loop: f32,
    pub loop_count: i32,
    pub flags_104: u8,
    pub volume_interp_curve_deprecated: UPtr<UDistributionFloatConstantCurve>,
    pub pitch_interp_curve_deprecated: UPtr<UDistributionFloatConstantCurve>,
    pub volume_curve: FRuntimeFloatCurve,
    pub pitch_curve: FRuntimeFloatCurve,
    pub pitch_min: f32,
    pub pitch_max: f32,
    pub volume_min: f32,
    pub volume_max: f32,
}
pub struct USoundNodeGroupControl {
    pub group_sizes: TArray<i32>,
}
pub struct USoundNodeLooping {
    pub loop_count: i32,
    pub flags_92: u8,
}
pub struct USoundNodeMature {}
pub struct USoundNodeMixer {
    pub input_volume: TArray<f32>,
}
pub struct USoundNodeModulator {
    pub pitch_min: f32,
    pub pitch_max: f32,
    pub volume_min: f32,
    pub volume_max: f32,
}
pub struct USoundNodeModulatorContinuous {
    pub pitch_modulation_params: FModulatorContinuousParams,
    pub volume_modulation_params: FModulatorContinuousParams,
}
pub struct USoundNodeOscillator {
    pub flags_88: u8,
    pub amplitude_min: f32,
    pub amplitude_max: f32,
    pub frequency_min: f32,
    pub frequency_max: f32,
    pub offset_min: f32,
    pub offset_max: f32,
    pub center_min: f32,
    pub center_max: f32,
}
pub struct USoundNodeParamCrossFade {
    pub param_name: FName,
}
pub struct USoundNodeQualityLevel {
    pub cooked_quality_level_index: i32,
}
pub struct USoundNodeRandom {
    pub weights: TArray<f32>,
    pub has_been_used: TArray<bool>,
    pub num_random_used: i32,
    pub preselect_at_level_load: i32,
    pub flags_128: u8,
    pub pie_hidden_nodes: TArray<i32>,
}
pub struct USoundNodeSoundClass {
    pub sound_class_override: UPtr<USoundClass>,
}
pub struct USoundNodeSwitch {
    pub int_parameter_name: FName,
}
pub struct USoundNodeWaveParam {
    pub wave_parameter_name: FName,
}
pub struct USoundNodeWavePlayer {
    pub sound_wave_asset_ptr: TSoftObjectPtr<USoundWave>,
    pub sound_wave: UPtr<USoundWave>,
    pub flags_144: u8,
}
pub struct USoundSourceBus {
    pub source_bus_channels: ESourceBusChannels,
    pub source_bus_duration: f32,
    pub audio_bus: UPtr<UAudioBus>,
    pub flags_2096: u8,
}
pub struct USoundSubmixBase {
    pub b_auto_disable: bool,
    pub auto_disable_time: f32,
    pub child_submixes: TArray<UPtr<USoundSubmixBase>>,
    pub dynamic_child_submixes: TMap<u32, FDynamicChildSubmix>,
}
pub struct USoundSubmixWithParentBase {
    pub parent_submix: UPtr<USoundSubmixBase>,
    pub dynamic_parent_submix: TMap<u32, UPtr<USoundSubmixBase>>,
    pub flags_248: u8,
}
pub struct USoundSubmix {
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
    pub on_submix_recorded_file_done: FSoundSubmix_OnSubmixRecordedFileDone,
    pub output_volume_deprecated: f32,
    pub wet_level_deprecated: f32,
    pub dry_level_deprecated: f32,
}
pub struct USoundfieldSubmix {
    pub soundfield_encoding_format: FName,
    pub encoding_settings: UPtr<
        crate::bindings::audio_extensions::USoundfieldEncodingSettingsBase,
    >,
    pub soundfield_effect_chain: TArray<
        UPtr<crate::bindings::audio_extensions::USoundfieldEffectBase>,
    >,
    pub encoding_settings_class: TSubclassOf<
        crate::bindings::audio_extensions::USoundfieldEncodingSettingsBase,
    >,
}
pub struct UEndpointSubmix {
    pub endpoint_type: FName,
    pub endpoint_settings_class: TSubclassOf<
        crate::bindings::audio_extensions::UAudioEndpointSettingsBase,
    >,
    pub endpoint_settings: UPtr<
        crate::bindings::audio_extensions::UAudioEndpointSettingsBase,
    >,
}
pub struct USoundfieldEndpointSubmix {
    pub soundfield_endpoint_type: FName,
    pub endpoint_settings_class: TSubclassOf<
        crate::bindings::audio_extensions::UAudioEndpointSettingsBase,
    >,
    pub endpoint_settings: UPtr<
        crate::bindings::audio_extensions::USoundfieldEndpointSettingsBase,
    >,
    pub encoding_settings_class: TSubclassOf<
        crate::bindings::audio_extensions::USoundfieldEncodingSettingsBase,
    >,
    pub encoding_settings: UPtr<
        crate::bindings::audio_extensions::USoundfieldEncodingSettingsBase,
    >,
    pub soundfield_effect_chain: TArray<
        UPtr<crate::bindings::audio_extensions::USoundfieldEffectBase>,
    >,
}
pub struct USparseVolumeTexture {}
pub struct USparseVolumeTextureFrame {
    pub owner: UPtr<USparseVolumeTexture>,
    pub frame_index: i32,
    pub transform: crate::bindings::core_u_object::FTransform,
}
pub struct UStreamableSparseVolumeTexture {
    pub volume_resolution: crate::bindings::core_u_object::FIntVector,
    pub num_mip_levels: i32,
    pub num_frames: i32,
    pub format_a: crate::bindings::core_u_object::EPixelFormat,
    pub format_b: crate::bindings::core_u_object::EPixelFormat,
    pub fallback_value_a: crate::bindings::core_u_object::FVector4f,
    pub fallback_value_b: crate::bindings::core_u_object::FVector4f,
    pub address_x: TextureAddress,
    pub address_y: TextureAddress,
    pub address_z: TextureAddress,
    pub b_local_ddc_only: bool,
    pub streaming_pool_size_factor: f32,
    pub number_of_prefetch_frames: i32,
    pub prefetch_percentage_step_size: f32,
    pub prefetch_percentage_bias: f32,
    pub asset_import_data: UPtr<UAssetImportData>,
    pub asset_user_data: TArray<UPtr<UAssetUserData>>,
    pub frames: TArray<UPtr<USparseVolumeTextureFrame>>,
    pub volume_bounds_min: crate::bindings::core_u_object::FIntVector,
    pub volume_bounds_max: crate::bindings::core_u_object::FIntVector,
    pub init_state: u8,
}
pub struct UStaticSparseVolumeTexture {}
pub struct UAnimatedSparseVolumeTexture {}
pub struct UAnimatedSparseVolumeTextureController {
    pub sparse_volume_texture: UPtr<USparseVolumeTexture>,
    pub time: f32,
    pub b_is_playing: bool,
    pub frame_rate: f32,
    pub mip_level: i32,
    pub b_blocking_streaming_requests: bool,
}
pub struct ASpectatorPawn {}
pub struct USpectatorPawnMovement {
    pub flags_480: u8,
}
pub struct ASplineMeshActor {
    pub spline_mesh_component: UPtr<USplineMeshComponent>,
}
pub struct UStaticMesh {
    pub source_models: TArray<FStaticMeshSourceModel>,
    pub hi_res_source_model: FStaticMeshSourceModel,
    pub section_info_map: FMeshSectionInfoMap,
    pub original_section_info_map: FMeshSectionInfoMap,
    pub lod_group: FName,
    pub num_streamed_lo_ds: crate::bindings::core_u_object::FPerPlatformInt,
    pub import_version: i32,
    pub material_remap_index_per_import_version: TArray<FMaterialRemapIndex>,
    pub flags_904: u8,
    pub materials_deprecated: TArray<UPtr<UMaterialInterface>>,
    pub nanite_settings: FMeshNaniteSettings,
    pub lightmap_uv_version: i32,
    pub min_quality_level_lod: FPerQualityLevelInt,
    pub min_lod: crate::bindings::core_u_object::FPerPlatformInt,
    pub element_to_ignore_for_tex_factor: i32,
    pub static_materials: TArray<FStaticMaterial>,
    pub assembly_reference_cache: TArray<UPtr<UStaticMesh>>,
    pub lightmap_uv_density: f32,
    pub light_map_resolution: i32,
    pub light_map_coordinate_index: i32,
    pub static_mesh_paint_support: EStaticMeshPaintSupport,
    pub mesh_paint_texture_coordinate_index: i32,
    pub mesh_paint_texture_resolution: i32,
    pub distance_field_self_shadow_bias: f32,
    pub body_setup: UPtr<UBodySetup>,
    pub lod_for_collision: i32,
    pub flags_1404: u8,
    pub ray_tracing_proxy_settings: FMeshRayTracingProxySettings,
    pub flags_1428: u8,
    pub asset_import_data: UPtr<UAssetImportData>,
    pub source_file_path_deprecated: FString,
    pub source_file_timestamp_deprecated: FString,
    pub thumbnail_info: UPtr<UThumbnailInfo>,
    pub editor_camera_position: FAssetEditorOrbitCameraPosition,
    pub b_customized_collision: bool,
    pub sockets: TArray<UPtr<UStaticMeshSocket>>,
    pub positive_bounds_extension: crate::bindings::core_u_object::FVector,
    pub negative_bounds_extension: crate::bindings::core_u_object::FVector,
    pub extended_bounds: crate::bindings::core_u_object::FBoxSphereBounds,
    pub asset_user_data: TArray<UPtr<UAssetUserData>>,
    pub editable_mesh_deprecated: UPtr<crate::bindings::core_u_object::UObject>,
    pub complex_collision_mesh: UPtr<UStaticMesh>,
    pub nav_collision: UPtr<UNavCollisionBase>,
}
pub struct UStaticMeshDescriptionBulkData {}
pub struct UActorTextureStreamingBuildDataComponent {
    pub streamable_textures: TArray<FStreamableTexture>,
    pub packed_texture_streaming_quality_level_feature_level: u32,
}
pub struct AServerStreamingLevelsVisibility {}
pub struct UAudioSubsystemCollectionRoot {}
pub struct ULocalPlayerSubsystem {}
pub struct USubsystemBlueprintLibrary {}
pub struct USubtitleAssetUserData {
    pub subtitles: TArray<FSubtitleAssetData>,
}
pub struct ATargetPoint {
    pub sprite_component: UPtr<UBillboardComponent>,
    pub arrow_component: UPtr<UArrowComponent>,
}
pub struct UTaskSyncManagerSettings {
    pub registered_sync_points: TArray<FSyncPointDescription>,
}
pub struct UAutomationTestSettings {
    pub engine_test_modules: TArray<FString>,
    pub editor_test_modules: TArray<FString>,
    pub automation_testmap: crate::bindings::core_u_object::FSoftObjectPath,
    pub editor_performance_test_maps: TArray<FEditorMapPerformanceTestDefinition>,
    pub assets_to_open: TArray<FString>,
    pub maps_to_pie_test: TArray<FString>,
    pub b_use_all_project_maps_to_play_in_pie: bool,
    pub build_promotion_test: FBuildPromotionTestSettings,
    pub material_editor_promotion_test: FMaterialEditorPromotionSettings,
    pub particle_editor_promotion_test: FParticleEditorPromotionSettings,
    pub blueprint_editor_promotion_test: FBlueprintEditorPromotionSettings,
    pub test_level_folders: TArray<FString>,
    pub external_tools: TArray<FExternalToolDefinition>,
    pub import_export_test_definitions: TArray<FEditorImportExportTestDefinition>,
    pub launch_on_settings: TArray<FLaunchOnTestSettings>,
    pub default_screenshot_resolution: crate::bindings::core_u_object::FIntPoint,
    pub pie_test_duration: f32,
    pub default_interactive_framerate: f32,
    pub default_interactive_framerate_wait_time: f32,
    pub default_interactive_framerate_duration: f32,
}
pub struct UAsyncLoadingTests_ConvertFromType_V1 {
    pub reference: TSoftObjectPtr<crate::bindings::core_u_object::UObject>,
}
pub struct UAsyncLoadingTests_ConvertFromType_V2 {
    pub reference: UPtr<crate::bindings::core_u_object::UObject>,
}
pub struct UAsyncLoadingTests_Shared {
    pub soft_reference: TSoftObjectPtr<crate::bindings::core_u_object::UObject>,
    pub hard_reference: UPtr<crate::bindings::core_u_object::UObject>,
}
pub struct UTextImportContainer {
    pub result_struct: FTextImportTestStruct,
}
pub struct UTransactionDiffingTestObject {
    pub names_array: TArray<FName>,
    pub additional_name: FName,
    pub objects_array: TArray<UPtr<crate::bindings::core_u_object::UObject>>,
    pub additional_object: UPtr<crate::bindings::core_u_object::UObject>,
    pub soft_objects_array: TArray<
        TSoftObjectPtr<crate::bindings::core_u_object::UObject>,
    >,
    pub additional_soft_object: TSoftObjectPtr<crate::bindings::core_u_object::UObject>,
    pub property_data: i32,
}
pub struct UTexture2DDynamic {
    pub format: crate::bindings::core_u_object::EPixelFormat,
}
pub struct UTextureCollection {
    pub textures: TArray<UPtr<UTexture>>,
}
pub struct UTextureCube {}
pub struct UTextureCubeArray {
    pub source_textures: TArray<UPtr<UTextureCube>>,
    pub b_source_generated_from_source_textures_array: bool,
}
pub struct UTextureLightProfile {
    pub brightness: f32,
    pub texture_multiplier: f32,
}
pub struct UTextureRenderTarget2DArray {
    pub size_x: i32,
    pub size_y: i32,
    pub slices: i32,
    pub clear_color: crate::bindings::core_u_object::FLinearColor,
    pub override_format: crate::bindings::core_u_object::EPixelFormat,
    pub flags_1277: u8,
}
pub struct UTextureRenderTargetCube {
    pub size_x: i32,
    pub clear_color: crate::bindings::core_u_object::FLinearColor,
    pub override_format: crate::bindings::core_u_object::EPixelFormat,
    pub flags_1269: u8,
    pub mips_sampler_filter: TextureFilter,
}
pub struct UTextureRenderTargetVolume {
    pub size_x: i32,
    pub size_y: i32,
    pub size_z: i32,
    pub clear_color: crate::bindings::core_u_object::FLinearColor,
    pub override_format: crate::bindings::core_u_object::EPixelFormat,
    pub flags_1277: u8,
}
pub struct UTimelineTemplate {
    pub timeline_length: f32,
    pub length_mode: ETimelineLengthMode,
    pub flags_53: u8,
    pub event_tracks: TArray<FTTEventTrack>,
    pub float_tracks: TArray<FTTFloatTrack>,
    pub vector_tracks: TArray<FTTVectorTrack>,
    pub linear_color_tracks: TArray<FTTLinearColorTrack>,
    pub meta_data_array: TArray<FBPVariableMetaDataEntry>,
    pub timeline_guid: crate::bindings::core_u_object::FGuid,
    pub timeline_tick_group: ETickingGroup,
    pub variable_name: FName,
    pub direction_property_name: FName,
    pub update_function_name: FName,
    pub finished_function_name: FName,
    pub track_display_order: TArray<FTTTrackId>,
}
pub struct ATriggerVolume {}
pub struct UUniversalObjectLocatorScriptingExtensions {}
pub struct UUserDefinedEnum {
    pub unique_name_index: u32,
    pub enum_description: FText,
    pub display_name_map: TMap<FName, FText>,
    pub cached_cooked_meta_data_ptr: UPtr<
        crate::bindings::core_u_object::UEnumCookedMetaData,
    >,
}
pub struct UUserInterfaceSettings {
    pub render_focus_rule: ERenderFocusRule,
    pub hardware_cursors: TMap<
        crate::bindings::core_u_object::EMouseCursor,
        FHardwareCursorReference,
    >,
    pub software_cursors: TMap<
        crate::bindings::core_u_object::EMouseCursor,
        crate::bindings::core_u_object::FSoftClassPath,
    >,
    pub default_cursor_deprecated: crate::bindings::core_u_object::FSoftClassPath,
    pub text_edit_beam_cursor_deprecated: crate::bindings::core_u_object::FSoftClassPath,
    pub crosshairs_cursor_deprecated: crate::bindings::core_u_object::FSoftClassPath,
    pub hand_cursor_deprecated: crate::bindings::core_u_object::FSoftClassPath,
    pub grab_hand_cursor_deprecated: crate::bindings::core_u_object::FSoftClassPath,
    pub grab_hand_closed_cursor_deprecated: crate::bindings::core_u_object::FSoftClassPath,
    pub slashed_circle_cursor_deprecated: crate::bindings::core_u_object::FSoftClassPath,
    pub application_scale: f32,
    pub ui_scale_rule: EUIScalingRule,
    pub custom_scaling_rule_class: crate::bindings::core_u_object::FSoftClassPath,
    pub ui_scale_curve: FRuntimeFloatCurve,
    pub b_allow_high_dpi_in_game_mode: bool,
    pub design_screen_size: crate::bindings::core_u_object::FIntPoint,
    pub b_load_widgets_on_dedicated_server: bool,
    pub b_authorize_automatic_widget_variable_creation: bool,
    pub custom_font_dpi: u32,
    pub font_dpi_preset: EFontDPI,
    pub b_use_custom_font_dpi: bool,
    pub b_enable_distance_field_font_rasterization: bool,
    pub cursor_classes: TArray<UPtr<crate::bindings::core_u_object::UObject>>,
    pub custom_scaling_rule_class_instance: TSubclassOf<
        crate::bindings::core_u_object::UObject,
    >,
    pub custom_scaling_rule: UPtr<UDPICustomScalingRule>,
}
pub struct UCanvas {
    pub org_x: f32,
    pub org_y: f32,
    pub clip_x: f32,
    pub clip_y: f32,
    pub draw_color: crate::bindings::core_u_object::FColor,
    pub flags_68: u8,
    pub size_x: i32,
    pub size_y: i32,
    pub color_modulate: crate::bindings::core_u_object::FPlane,
    pub default_texture: UPtr<UTexture2D>,
    pub gradient_texture0: UPtr<UTexture2D>,
    pub reporter_graph: UPtr<UReporterGraph>,
}
pub struct UConsole {
    pub console_target_player: UPtr<ULocalPlayer>,
    pub default_texture_black: UPtr<UTexture2D>,
    pub default_texture_white: UPtr<UTexture2D>,
    pub history_buffer: TArray<FString>,
}
pub struct UInputSettings {
    pub axis_config: TArray<FInputAxisConfigEntry>,
    pub platform_settings: crate::bindings::developer_settings::FPerPlatformSettings,
    pub flags_80: u8,
    pub flags_81: u8,
    pub excluded_autocorrect_os: TArray<FString>,
    pub excluded_autocorrect_cultures: TArray<FString>,
    pub excluded_autocorrect_device_models: TArray<FString>,
    pub default_viewport_mouse_capture_mode: EMouseCaptureMode,
    pub default_viewport_mouse_lock_mode: EMouseLockMode,
    pub fov_scale: f32,
    pub double_click_time: f32,
    pub action_mappings: TArray<FInputActionKeyMapping>,
    pub axis_mappings: TArray<FInputAxisKeyMapping>,
    pub deprecated_action_and_axis_names: TSet<FName>,
    pub speech_mappings: TArray<FInputActionSpeechMapping>,
    pub default_player_input_class: TSoftObjectPtr<
        crate::bindings::core_u_object::UClass,
    >,
    pub default_input_component_class: TSoftObjectPtr<
        crate::bindings::core_u_object::UClass,
    >,
    pub default_touch_interface: crate::bindings::core_u_object::FSoftObjectPath,
    pub console_keys: TArray<crate::bindings::input_core::FKey>,
}
pub struct UInputPlatformSettings {
    pub max_platform_user_count: i32,
    pub device_mapping_policy: crate::bindings::core_u_object::EInputDeviceMappingPolicy,
    pub max_trigger_feedback_position: i32,
    pub max_trigger_feedback_strength: i32,
    pub max_trigger_vibration_trigger_position: i32,
    pub max_trigger_vibration_frequency: i32,
    pub max_trigger_vibration_amplitude: i32,
    pub hardware_devices: TArray<FHardwareDeviceIdentifier>,
}
pub struct UPlayerInput {
    pub debug_exec_bindings: TArray<FKeyBind>,
    pub inverted_axis: TArray<FName>,
}
pub struct AVectorFieldVolume {
    pub vector_field_component: UPtr<UVectorFieldComponent>,
    pub sprite_component: UPtr<UBillboardComponent>,
}
pub struct UTireType {
    pub friction_scale: f32,
}
pub struct UVirtualTextureCollection {
    pub b_allow_format_conversion: bool,
    pub b_is_srgb: bool,
    pub runtime_pixel_format: crate::bindings::core_u_object::EPixelFormat,
}
pub struct UVisualLoggerAutomationTests {}
pub struct AVisualLoggerFilterVolume {}
pub struct UVisualLoggerKismetLibrary {}
pub struct UVoiceChannel {}
pub struct UVOIPTalker {
    pub settings: FVoiceSettings,
}
pub struct UVOIPStatics {}
pub struct UVolumeTexture {
    pub source2_d_texture: UPtr<UTexture2D>,
    pub source_lighting_guid_deprecated: crate::bindings::core_u_object::FGuid,
    pub source2_d_tile_size_x: i32,
    pub source2_d_tile_size_y: i32,
    pub address_mode: TextureAddress,
}
pub struct AVolumetricLightmapDensityVolume {
    pub allowed_mip_level_range: crate::bindings::core_u_object::FInt32Interval,
}
pub struct ULightMapVirtualTexture2D {
    pub type_to_layer: TArray<i8>,
}
pub struct UMeshPaintVirtualTexture {
    pub owning_component: TWeakObjectPtr<UPrimitiveComponent>,
}
pub struct URuntimeVirtualTexture {
    pub tile_count: i32,
    pub tile_size: i32,
    pub tile_border_size: i32,
    pub material_type: ERuntimeVirtualTextureMaterialType,
    pub b_compress_textures: bool,
    pub b_use_low_quality_compression: bool,
    pub custom_priority: crate::bindings::render_core::EVTProducerPriority,
    pub b_use_custom_priority: bool,
    pub b_clear_textures: bool,
    pub b_single_physical_space: bool,
    pub b_private_space: bool,
    pub b_adaptive: bool,
    pub b_continuous_update: bool,
    pub remove_low_mips: i32,
    pub custom_material_data: crate::bindings::core_u_object::FVector4f,
    pub lod_group: TextureGroup,
    pub size_deprecated: i32,
    pub streaming_texture_deprecated: UPtr<URuntimeVirtualTextureStreamingProxy>,
}
pub struct ARuntimeVirtualTextureVolume {
    pub virtual_texture_component: UPtr<URuntimeVirtualTextureComponent>,
    pub box_: UPtr<UBoxComponent>,
}
pub struct UVirtualTexture {}
pub struct ULightMapVirtualTexture {}
pub struct URuntimeVirtualTextureStreamingProxy {}
pub struct UVirtualTexture2D {
    pub settings: FVirtualTextureBuildSettings,
    pub b_continuous_update_deprecated: bool,
    pub b_single_physical_space: bool,
}
pub struct UVirtualTextureAdapter {
    pub texture: UPtr<UTexture>,
    pub override_with_texture_format: UPtr<UTexture2D>,
    pub b_use_default_tile_sizes: bool,
    pub tile_size: i32,
    pub tile_border_size: i32,
}
pub struct UVirtualTextureBuilder {
    pub texture: UPtr<UVirtualTexture2D>,
    pub texture_mobile: UPtr<UVirtualTexture2D>,
    pub build_hash: u64,
    pub b_separate_texture_for_mobile: bool,
}
pub struct UVirtualTexturePoolConfig {
    pub default_size_in_megabyte: i32,
    pub b_pool_auto_grow_in_editor: bool,
    pub pools: TArray<FVirtualTextureSpacePoolConfig>,
    pub transient_pools: TArray<FVirtualTextureSpacePoolConfig>,
}
pub struct AWindDirectionalSource {
    pub component: UPtr<UWindDirectionalSourceComponent>,
    pub arrow_component: UPtr<UArrowComponent>,
}
pub struct UWorldComposition {
    pub tiles_streaming: TArray<UPtr<ULevelStreaming>>,
    pub tiles_streaming_time_threshold: f64,
    pub b_load_all_tiles_during_cinematic: bool,
    pub b_rebase_origin_in3_d_space: bool,
    pub b_lock_tiles_location: bool,
    pub rebase_origin_distance: f32,
}
pub struct UWorldPartitionBlueprintLibrary {}
pub struct UActorDescContainer {}
pub struct UActorDescContainerInstance {
    pub container: UPtr<UActorDescContainer>,
    pub child_container_instances: TMap<
        crate::bindings::core_u_object::FGuid,
        UPtr<UActorDescContainerInstance>,
    >,
}
pub struct UActorDescContainerSubsystem {}
pub struct UContentBundleDescriptor {
    pub display_name: FString,
    pub debug_color: crate::bindings::core_u_object::FColor,
    pub guid: crate::bindings::core_u_object::FGuid,
}
pub struct UContentBundleUnsavedActorMonitor {
    pub unsaved_actors: TArray<TWeakObjectPtr<AActor>>,
}
pub struct UContentBundleEngineSubsystem {
    pub content_bundle_type_factory_class: TSoftObjectPtr<
        crate::bindings::core_u_object::UClass,
    >,
    pub content_bundle_type_factory: UPtr<UContentBundleTypeFactory>,
}
pub struct UContentBundleTypeFactory {}
pub struct UContentBundleManager {
    pub pie_duplicate_helper: UPtr<UContentBundleDuplicateForPIEHelper>,
}
pub struct UContentBundleDuplicateForPIEHelper {
    pub streaming_objects: TMap<
        crate::bindings::core_u_object::FGuid,
        UPtr<URuntimeHashExternalStreamingObjectBase>,
    >,
}
pub struct UWorldPartitionCookPackageObject {}
pub struct IWorldPartitionCookPackageObject {}
pub struct AWorldDataLayers {
    pub b_use_external_package_data_layer_instances: bool,
    pub b_allow_runtime_data_layer_editing: bool,
    pub current_data_layers: FActorPlacementDataLayers,
    pub root_external_data_layer_instance: UPtr<UExternalDataLayerInstance>,
    pub data_layer_instances: TSet<UPtr<UDataLayerInstance>>,
    pub external_package_data_layer_instances: TSet<UPtr<UDataLayerInstance>>,
    pub loaded_external_package_data_layer_instances: TArray<UPtr<UDataLayerInstance>>,
    pub transient_data_layer_instances: TArray<UPtr<UDataLayerInstance>>,
    pub deprecated_data_layer_name_to_data_layer_instance: TMap<
        FName,
        TWeakObjectPtr<UDataLayerInstance>,
    >,
    pub world_data_layers_deprecated: TSet<UPtr<UDEPRECATED_DataLayer>>,
    pub rep_active_data_layer_names: TArray<FName>,
    pub rep_loaded_data_layer_names: TArray<FName>,
    pub rep_effective_active_data_layer_names: TArray<FName>,
    pub rep_effective_loaded_data_layer_names: TArray<FName>,
}
pub struct UDEPRECATED_DataLayer {
    pub flags_48: u8,
    pub data_layer_label: FName,
    pub flags_64: u8,
    pub initial_runtime_state: EDataLayerRuntimeState,
    pub debug_color: crate::bindings::core_u_object::FColor,
    pub parent_deprecated: UPtr<UDEPRECATED_DataLayer>,
    pub children_deprecated: TArray<UPtr<UDEPRECATED_DataLayer>>,
}
pub struct UDataLayerAsset {
    pub data_layer_type: EDataLayerType,
    pub b_supports_actor_filters: bool,
    pub debug_color: crate::bindings::core_u_object::FColor,
    pub load_filter: EDataLayerLoadFilter,
}
pub struct UDataLayerInstance {
    pub flags_48: u8,
    pub override_block_on_slow_streaming: EOverrideBlockOnSlowStreaming,
    pub streaming_priority: i32,
    pub initial_runtime_state: EDataLayerRuntimeState,
    pub parent: UPtr<UDataLayerInstance>,
    pub children: TArray<UPtr<UDataLayerInstance>>,
}
pub struct UDataLayerInstancePrivate {
    pub short_name: FString,
    pub data_layer_asset: UPtr<UDataLayerAsset>,
    pub b_is_included_in_actor_filter_default: bool,
}
pub struct UDataLayerInstanceProvider {}
pub struct IDataLayerInstanceProvider {}
pub struct UDataLayerInstanceWithAsset {
    pub data_layer_asset: UPtr<UDataLayerAsset>,
    pub b_is_included_in_actor_filter_default: bool,
}
pub struct UWorldPartition {
    pub editor_hash: UPtr<UWorldPartitionEditorHash>,
    pub world_partition_streaming_policy_class: TSubclassOf<
        UWorldPartitionStreamingPolicy,
    >,
    pub b_streaming_was_enabled: bool,
    pub runtime_hash: UPtr<UWorldPartitionRuntimeHash>,
    pub b_enable_streaming: bool,
    pub server_streaming_mode: EWorldPartitionServerStreamingMode,
    pub server_streaming_out_mode: EWorldPartitionServerStreamingOutMode,
    pub data_layers_logic_operator: EWorldPartitionDataLayersLogicOperator,
    pub flags_572: u8,
    pub runtime_cells_transformer_stack: TArray<FRuntimeCellTransformerInstance>,
    pub default_hlod_layer: UPtr<UHLODLayer>,
    pub actor_desc_container_instance: UPtr<UActorDescContainerInstance>,
    pub container_instance_class: TSubclassOf<UActorDescContainerInstance>,
    pub referenced_objects: TSet<UPtr<crate::bindings::core_u_object::UObject>>,
    pub data_layer_manager: UPtr<UDataLayerManager>,
    pub external_data_layer_manager: UPtr<UExternalDataLayerManager>,
    pub streaming_policy: UPtr<UWorldPartitionStreamingPolicy>,
    pub registered_editor_loader_adapters: TSet<
        UPtr<UWorldPartitionEditorLoaderAdapter>,
    >,
}
pub struct UDataLayerManager {
    pub on_data_layer_instance_runtime_state_changed: FDataLayerManager_OnDataLayerInstanceRuntimeStateChanged,
    pub data_layer_loading_policy_class: TSoftObjectPtr<
        crate::bindings::core_u_object::UClass,
    >,
    pub data_layer_instance_with_asset_class: TSoftObjectPtr<
        crate::bindings::core_u_object::UClass,
    >,
    pub data_layer_loading_policy: UPtr<UDataLayerLoadingPolicy>,
}
pub struct UDataLayerLoadingPolicy {}
pub struct UDataLayerSubsystem {
    pub on_data_layer_runtime_state_changed: FDataLayerSubsystem_OnDataLayerRuntimeStateChanged,
    pub data_layer_loading_policy_class: TSoftObjectPtr<
        crate::bindings::core_u_object::UClass,
    >,
}
pub struct UDeprecatedDataLayerInstance {
    pub label: FName,
    pub deprecated_data_layer_f_name: FName,
    pub data_layer_type: EDataLayerType,
    pub debug_color: crate::bindings::core_u_object::FColor,
}
pub struct UExternalDataLayerAsset {
    pub uid: FExternalDataLayerUID,
}
pub struct UExternalDataLayerEngineSubsystem {
    pub injection_policy_class: TSubclassOf<UExternalDataLayerInjectionPolicy>,
    pub injection_policy: UPtr<UExternalDataLayerInjectionPolicy>,
}
pub struct UExternalDataLayerInjectionPolicy {}
pub struct UExternalDataLayerInstance {}
pub struct UExternalDataLayerManager {
    pub external_streaming_objects: TMap<
        UPtr<UExternalDataLayerAsset>,
        UPtr<URuntimeHashExternalStreamingObjectBase>,
    >,
    pub injected_external_data_layer_assets: TSet<UPtr<UExternalDataLayerAsset>>,
}
pub struct AWorldPartitionCustomHLOD {
    pub static_mesh_component: UPtr<UStaticMeshComponent>,
    pub hlod_instance_guid: crate::bindings::core_u_object::FGuid,
}
pub struct AWorldPartitionCustomHLODPlaceholder {}
pub struct UWorldPartitionDestructibleHLODComponent {
    pub destructible_hlod_material: UPtr<UMaterialInterface>,
    pub destructible_hlod_instances_mapping_data: FHLODInstancingPackedMappingData,
    pub destructible_hlod_state: FWorldPartitionDestructibleHLODState,
    pub visibility_material: UPtr<UMaterialInstanceDynamic>,
    pub visibility_texture: UPtr<UTexture2DDynamic>,
    pub destructible_actors: TArray<FName>,
}
pub struct UDEPRECATED_UWorldPartitionDestructibleHLODMeshComponent {}
pub struct AWorldPartitionHLOD {
    pub source_actors: UPtr<UWorldPartitionHLODSourceActors>,
    pub input_stats: FHLODBuildInputStats,
    pub hlod_bounds: crate::bindings::core_u_object::FBox,
    pub min_visible_distance: f64,
    pub hlod_hash: u32,
    pub hlod_stats: TMap<FName, i64>,
    pub hlod_build_report: FString,
    pub lod_level: u32,
    pub b_require_warmup: bool,
    pub source_cell_guid: crate::bindings::core_u_object::FGuid,
    pub standalone_hlod_guid: crate::bindings::core_u_object::FGuid,
    pub source_cell_deprecated: TSoftObjectPtr<UWorldPartitionRuntimeCell>,
    pub source_cell_name_deprecated: FName,
    pub hlod_sub_actors_deprecated: TArray<FWorldPartitionRuntimeCellObjectMapping>,
    pub sub_actors_hlod_layer_deprecated: UPtr<UHLODLayer>,
}
pub struct UHLODBuilderSettings {}
pub struct UNullHLODBuilder {}
pub struct UWorldPartitionDestructibleInHLODInterface {}
pub struct IWorldPartitionDestructibleInHLODInterface {}
pub struct UWorldPartitionDestructibleInHLODSupportLibrary {}
pub struct UHLODInstancedSkinnedMeshComponent {}
pub struct UHLODInstancedStaticMeshComponent {}
pub struct UHLODLayer {
    pub layer_type: EHLODLayerType,
    pub hlod_builder_class: TSubclassOf<UHLODBuilder>,
    pub hlod_builder_settings: UPtr<UHLODBuilderSettings>,
    pub flags_72: u8,
    pub cell_size: i32,
    pub loading_range: f64,
    pub parent_layer: UPtr<UHLODLayer>,
    pub linked_layer: UPtr<UHLODLayer>,
    pub hlod_actor_class: TSubclassOf<AWorldPartitionHLOD>,
    pub hlod_modifier_class: TSubclassOf<UWorldPartitionHLODModifier>,
    pub mesh_merge_settings_deprecated: FMeshMergingSettings,
    pub mesh_simplify_settings_deprecated: FMeshProxySettings,
    pub mesh_approximation_settings_deprecated: FMeshApproximationSettings,
    pub hlod_material_deprecated: TSoftObjectPtr<UMaterialInterface>,
    pub flags_1120: u8,
}
pub struct UWorldPartitionHLODModifier {}
pub struct AWorldPartitionHLODOnlyLevelInstance {
    pub world_asset: TSoftObjectPtr<UWorld>,
    pub level_instance_spawn_guid: crate::bindings::core_u_object::FGuid,
}
pub struct UWorldPartitionHLODProvider {}
pub struct IWorldPartitionHLODProvider {}
pub struct UWorldPartitionHLODRuntimeSubsystem {}
pub struct UWorldPartitionHLODSourceActors {
    pub hlod_layer: UPtr<UHLODLayer>,
}
pub struct UWorldPartitionHLODSourceActorsFromCell {
    pub actors: TArray<FWorldPartitionRuntimeCellObjectMapping>,
}
pub struct UWorldPartitionHLODSourceActorsFromLevel {
    pub source_level: TSoftObjectPtr<UWorld>,
}
pub struct UMaterialParameterCollectionHLODModifier {
    pub mpc: UPtr<UMaterialParameterCollection>,
    pub scalar_parameters: TArray<FHLODModifierScalarParameter>,
}
pub struct AWorldPartitionStandaloneHLOD {
    pub world_asset: TSoftObjectPtr<UWorld>,
    pub level_instance_spawn_guid: crate::bindings::core_u_object::FGuid,
}
pub struct UWorldPartitionStandaloneHLODSubsystem {}
pub struct ULevelInstanceContainerInstance {
    pub override_container: UPtr<UActorDescContainer>,
    pub parent_container_references: TArray<UPtr<UActorDescContainer>>,
    pub property_override_asset: TWeakObjectPtr<ULevelInstancePropertyOverrideAsset>,
    pub container_id_to_container_path: TMap<FActorContainerID, FActorContainerPath>,
}
pub struct ULevelInstancePropertyOverrideContainer {}
pub struct ANavigationDataChunkActor {
    pub nav_data_chunks: TArray<UPtr<UNavigationDataChunk>>,
    pub data_chunk_actor_bounds: crate::bindings::core_u_object::FBox,
}
pub struct URuntimePartition {
    pub name: FName,
    pub b_block_on_slow_streaming: bool,
    pub b_client_only_visible: bool,
    pub priority: i32,
    pub bounds_method: ERuntimePartitionCellBoundsMethod,
    pub loading_range: i32,
    pub debug_color: crate::bindings::core_u_object::FLinearColor,
    pub hlod_index: i32,
}
pub struct URuntimePartitionLevelStreaming {}
pub struct URuntimePartitionLHGrid {
    pub cell_size: u32,
    pub origin: crate::bindings::core_u_object::FVector,
    pub b_is2_d: bool,
    pub b_show_grid_preview: bool,
}
pub struct URuntimePartitionPersistent {}
pub struct UWorldPartitionRuntimeCell {
    pub unsaved_actors_container: UPtr<UActorContainer>,
    pub b_is_always_loaded: bool,
    pub b_is_spatially_loaded: bool,
    pub data_layers: FDataLayerInstanceNames,
    pub b_client_only_visible: bool,
    pub b_is_hlod: bool,
    pub b_is_custom_hlod_placeholder_cell: bool,
    pub b_block_on_slow_loading: bool,
    pub content_bundle_id: crate::bindings::core_u_object::FGuid,
    pub cell_debug_color: crate::bindings::core_u_object::FLinearColor,
    pub cell_guid: crate::bindings::core_u_object::FGuid,
    pub source_cell_guid: crate::bindings::core_u_object::FGuid,
    pub external_data_layer_asset: UPtr<UExternalDataLayerAsset>,
    pub runtime_cell_data: UPtr<UWorldPartitionRuntimeCellData>,
}
pub struct UWorldPartitionRuntimeCellData {
    pub content_bounds: crate::bindings::core_u_object::FBox,
    pub cell_bounds: TOptional<crate::bindings::core_u_object::FBox>,
    pub grid_name: FName,
    pub priority: i32,
    pub hierarchical_level: i32,
}
pub struct UWorldPartitionRuntimeCellDataHashSet {
    pub b_is2_d: bool,
}
pub struct URuntimeHashExternalStreamingObjectBase {
    pub sub_objects_to_cell_remapping: TMap<FName, FName>,
    pub container_resolver: FWorldPartitionRuntimeContainerResolver,
    pub outer_world: TSoftObjectPtr<UWorld>,
    pub cell_to_streaming_data: TMap<FName, FWorldPartitionRuntimeCellStreamingData>,
    pub data_layer_instances: TSet<UPtr<UDataLayerInstance>>,
    pub root_external_data_layer_instance: UPtr<UExternalDataLayerInstance>,
    pub packages_to_generate_for_cook: TMap<FString, UPtr<UWorldPartitionRuntimeCell>>,
    pub external_data_layer_asset: UPtr<UExternalDataLayerAsset>,
}
pub struct URuntimeHashSetExternalStreamingObject {
    pub runtime_streaming_data: TArray<FRuntimePartitionStreamingData>,
}
pub struct UWorldPartitionRuntimeHash {}
pub struct UWorldPartitionRuntimeHashSet {
    pub runtime_partitions: TArray<FRuntimePartitionDesc>,
    pub runtime_streaming_data: TArray<FRuntimePartitionStreamingData>,
    pub world_asset_streaming_objects: TMap<
        crate::bindings::core_u_object::FGuid,
        UPtr<URuntimeHashSetExternalStreamingObject>,
    >,
    pub standalone_hlod_actor_to_source_cells_map: TMap<
        crate::bindings::core_u_object::FGuid,
        crate::bindings::core_u_object::FGuid,
    >,
    pub custom_hlod_actor_to_source_cells_map: TMap<
        crate::bindings::core_u_object::FGuid,
        crate::bindings::core_u_object::FGuid,
    >,
}
pub struct AMapBuildDataActor {
    pub build_data: UPtr<UMapBuildDataRegistry>,
    pub force_link_to_actor: UPtr<AActor>,
    pub actor_bounds: crate::bindings::core_u_object::FBox,
    pub cell_package: FName,
    pub actor_instances: TArray<crate::bindings::core_u_object::FGuid>,
    pub level_build_data_id: crate::bindings::core_u_object::FGuid,
}
pub struct UWorldPartitionActorLoaderInterface {}
pub struct IWorldPartitionActorLoaderInterface {}
pub struct UWorldPartitionEditorHash {}
pub struct UWorldPartitionEditorPerProjectUserSettings {
    pub flags_104: u8,
    pub flags_105: u8,
    pub minimap_unloaded_opacity: f32,
    pub per_world_editor_settings: TMap<
        TSoftObjectPtr<UWorld>,
        FWorldPartitionPerWorldSettings,
    >,
}
pub struct UWorldPartitionEditorSpatialHash {
    pub cell_size: i32,
    pub world_image: crate::bindings::core_u_object::FSoftObjectPath,
    pub world_image_top_left_w: crate::bindings::core_u_object::FVector2D,
    pub world_image_bottom_right_w: crate::bindings::core_u_object::FVector2D,
}
pub struct UWorldPartitionLevelStreamingDynamic {
    pub runtime_level: UPtr<ULevel>,
    pub b_should_be_always_loaded: bool,
    pub b_should_perform_standard_level_loading: bool,
    pub unsaved_actors_container: UPtr<UActorContainer>,
    pub streaming_cell: TWeakObjectPtr<UWorldPartitionRuntimeLevelStreamingCell>,
    pub outer_world_partition: TSoftObjectPtr<UWorldPartition>,
}
pub struct UWorldPartitionStreamingPolicy {
    pub current_state: FWorldPartitionUpdateStreamingCurrentState,
    pub current_streaming_performance: EWorldPartitionStreamingPerformance,
    pub b_current_block_on_slow_streaming: bool,
    pub target_state: FWorldPartitionUpdateStreamingTargetState,
    pub async_task_current_state: FWorldPartitionUpdateStreamingCurrentState,
    pub async_task_target_state: FWorldPartitionUpdateStreamingTargetState,
    pub b_should_merge_streaming_source_info: bool,
}
pub struct UWorldPartitionLevelStreamingPolicy {
    pub source_world_asset_path: crate::bindings::core_u_object::FTopLevelAssetPath,
    pub sub_objects_to_cell_remapping: TMap<FName, FName>,
    pub container_resolver: FWorldPartitionRuntimeContainerResolver,
    pub external_streaming_objects: TArray<
        TWeakObjectPtr<URuntimeHashExternalStreamingObjectBase>,
    >,
    pub sub_objects_to_external_streaming_objects_remapping: TMap<
        FName,
        TWeakObjectPtr<URuntimeHashExternalStreamingObjectBase>,
    >,
}
pub struct AWorldPartitionMiniMap {
    pub mini_map_world_bounds: crate::bindings::core_u_object::FBox,
    pub uv_offset: crate::bindings::core_u_object::FBox2D,
    pub mini_map_texture: UPtr<UTexture2D>,
    pub excluded_data_layers: TSet<UPtr<UDataLayerAsset>>,
    pub world_units_per_pixel: i32,
    pub builder_cell_size: i32,
    pub capture_source: ESceneCaptureSource,
    pub capture_warmup_frames: u32,
    pub mini_map_tile_size_deprecated: i32,
}
pub struct AWorldPartitionMiniMapVolume {}
pub struct AWorldPartitionReplay {
    pub streaming_source_names: TArray<FName>,
}
pub struct UWorldPartitionRuntimeCellDataSpatialHash {
    pub position: crate::bindings::core_u_object::FVector,
    pub extent: f32,
}
pub struct UWorldPartitionCell {}
pub struct IWorldPartitionCell {}
pub struct UWorldPartitionRuntimeCellTransformerSettings {
    pub ignored_component_classes: TArray<TSubclassOf<UActorComponent>>,
    pub ignored_exact_component_classes: TArray<TSubclassOf<UActorComponent>>,
}
pub struct UWorldPartitionRuntimeCellTransformer {
    pub b_enabled: bool,
}
pub struct UWorldPartitionRuntimeCellTransformerISM {
    pub allowed_classes: TArray<TSubclassOf<AActor>>,
    pub disallowed_classes: TArray<TSubclassOf<AActor>>,
    pub min_num_instances: u32,
}
pub struct AWorldPartitionAutoInstancedActor {}
pub struct UWorldPartitionRuntimeCellTransformerLog {
    pub b_only_log_differences: bool,
}
pub struct UWorldPartitionRuntimeLevelStreamingCell {
    pub packages: TArray<FWorldPartitionRuntimeCellObjectMapping>,
    pub level_streaming: UPtr<UWorldPartitionLevelStreamingDynamic>,
}
pub struct ASpatialHashRuntimeGridInfo {
    pub grid_settings: FSpatialHashRuntimeGrid,
}
pub struct URuntimeSpatialHashExternalStreamingObject {
    pub streaming_grids: TArray<FSpatialHashStreamingGrid>,
}
pub struct UWorldPartitionRuntimeSpatialHash {
    pub grids: TArray<FSpatialHashRuntimeGrid>,
    pub b_preview_grids: bool,
    pub preview_grid_level: i32,
    pub grid_previewer: FWorldPartitionRuntimeSpatialHashGridPreviewer,
    pub use_aligned_grid_levels: EWorldPartitionCVarProjectDefaultOverride,
    pub snap_non_aligned_grid_levels_to_lower_levels: EWorldPartitionCVarProjectDefaultOverride,
    pub place_small_actors_using_location: EWorldPartitionCVarProjectDefaultOverride,
    pub place_partition_actors_using_location: EWorldPartitionCVarProjectDefaultOverride,
    pub b_enable_z_culling: bool,
    pub settings: FSpatialHashSettings,
    pub streaming_grids: TArray<FSpatialHashStreamingGrid>,
    pub world_asset_streaming_objects: TMap<
        crate::bindings::core_u_object::FGuid,
        UPtr<URuntimeSpatialHashExternalStreamingObject>,
    >,
}
pub struct UWorldPartitionSettings {
    pub new_maps_data_layers_logic_operator: EWorldPartitionDataLayersLogicOperator,
    pub b_new_maps_enable_world_partition: bool,
    pub b_new_maps_enable_world_partition_streaming: bool,
    pub editor_hash_default_class: TSubclassOf<UWorldPartitionEditorHash>,
    pub runtime_hash_default_class: TSubclassOf<UWorldPartitionRuntimeHash>,
    pub property_override_policy: UPtr<UWorldPartitionPropertyOverridePolicy>,
    pub unsupported_actor_classes_per_hlod_layer_type: TMap<
        EHLODLayerType,
        FHLODLayerTypeUnsupportedActorClasses,
    >,
    pub b_should_edl_packages_inherit_world_chunk_assignments_during_cook: bool,
}
pub struct UWorldPartitionSubsystem {}
pub struct ADEPRECATED_WorldPartitionVolume {}
pub struct AWorldSettings {
    pub visibility_cell_size: i32,
    pub visibility_aggressiveness: EVisibilityAggressiveness,
    pub flags_1157: u8,
    pub flags_1158: u8,
    pub flags_1159: u8,
    pub ai_system_class: TSoftObjectPtr<crate::bindings::core_u_object::UClass>,
    pub level_instance_pivot_offset: crate::bindings::core_u_object::FVector,
    pub navigation_system_config: UPtr<UNavigationSystemConfig>,
    pub navigation_system_config_override: UPtr<UNavigationSystemConfig>,
    pub world_partition: UPtr<UWorldPartition>,
    pub instanced_foliage_grid_size: u32,
    pub b_show_instanced_foliage_grid: bool,
    pub landscape_spline_meshes_grid_size: u32,
    pub navigation_data_chunk_grid_size: u32,
    pub navigation_data_builder_loading_cell_size: u32,
    pub default_placement_grid_size: u32,
    pub base_navmesh_data_layers: TArray<UPtr<UDataLayerAsset>>,
    pub world_to_meters: f32,
    pub kill_z: f32,
    pub kill_z_damage_type: TSubclassOf<UDamageType>,
    pub world_gravity_z: f32,
    pub global_gravity_z: f32,
    pub default_physics_volume_class: TSubclassOf<ADefaultPhysicsVolume>,
    pub physics_collision_handler_class: TSubclassOf<UPhysicsCollisionHandler>,
    pub default_game_mode: TSubclassOf<AGameModeBase>,
    pub game_network_manager_class: TSubclassOf<AGameNetworkManager>,
    pub packed_light_and_shadow_map_texture_size: i32,
    pub default_color_scale: crate::bindings::core_u_object::FVector,
    pub default_max_distance_field_occlusion_distance: f32,
    pub global_distance_field_view_distance: f32,
    pub dynamic_indirect_shadows_self_shadowing_intensity: f32,
    pub lightmass_settings: FLightmassWorldInfoSettings,
    pub volumetric_lightmap_loading_range: f32,
    pub nanite_settings: FNaniteSettings,
    pub default_reverb_settings: FReverbSettings,
    pub default_ambient_zone_settings: FInteriorSettings,
    pub default_base_sound_mix: UPtr<USoundMix>,
    pub hlod_setup_asset: TSoftObjectPtr<crate::bindings::core_u_object::UClass>,
    pub override_base_material: TSoftObjectPtr<UMaterialInterface>,
    pub hierarchical_lod_setup: TArray<FHierarchicalSimplification>,
    pub num_hlod_levels: i32,
    pub hlod_baking_transform: crate::bindings::core_u_object::FTransform,
    pub book_marks: UPtr<UBookMark>,
    pub time_dilation: f32,
    pub cinematic_time_dilation: f32,
    pub demo_play_time_dilation: f32,
    pub min_global_time_dilation: f32,
    pub max_global_time_dilation: f32,
    pub min_cinematic_time_dilation: f32,
    pub max_cinematic_time_dilation: f32,
    pub min_undilated_frame_time: f32,
    pub max_undilated_frame_time: f32,
    pub broadphase_settings: FBroadphaseSettings,
    pub replication_viewers: TArray<FNetViewer>,
    pub asset_user_data: TArray<UPtr<UAssetUserData>>,
    pub asset_user_data_editor_only: TArray<UPtr<UAssetUserData>>,
    pub pauser_player_state: UPtr<APlayerState>,
    pub default_world_partition_settings: FWorldPartitionPerWorldSettings,
    pub max_number_of_bookmarks: i32,
    pub default_bookmark_class: TSubclassOf<UBookmarkBase>,
    pub bookmark_array: TArray<UPtr<UBookmarkBase>>,
    pub last_bookmark_class: TSubclassOf<UBookmarkBase>,
    pub default_bookmark: UPtr<UBookmarkBase>,
    pub b_enable_hierarchical_lod_system_deprecated: bool,
    pub flags_2209: u8,
}
pub struct FDelegateArray_Delegates;
pub struct FTimelineEventEntry_EventFunc;
pub struct FTimelineVectorTrack_InterpFunc;
pub struct FTimelineFloatTrack_InterpFunc;
pub struct FTimelineLinearColorTrack_InterpFunc;
pub struct FTimeline_EventFunc;
pub struct FTimeline_InterpFunc;
pub struct FTimeline_TimelinePostUpdateFunc;
pub struct FTimeline_TimelineFinishedFunc;
pub struct FK2_ClearTimerDelegate_Delegate;
pub struct FK2_GetTimerElapsedTimeDelegate_Delegate;
pub struct FK2_GetTimerRemainingTimeDelegate_Delegate;
pub struct FK2_IsTimerActiveDelegate_Delegate;
pub struct FK2_IsTimerPausedDelegate_Delegate;
pub struct FK2_PauseTimerDelegate_Delegate;
pub struct FK2_SetTimerDelegate_Delegate;
pub struct FK2_SetTimerForNextTickDelegate_Delegate;
pub struct FK2_TimerExistsDelegate_Delegate;
pub struct FK2_UnPauseTimerDelegate_Delegate;
pub struct FLoadAsset_OnLoaded;
pub struct FLoadAssetClass_OnLoaded;
pub struct FLoadAssets_OnLoaded;
pub struct FAddEvent_EventFunc;
pub struct FAddInterpFloat_InterpFunc;
pub struct FAddInterpLinearColor_InterpFunc;
pub struct FAddInterpVector_InterpFunc;
pub struct FOnRep_Timeline_EventFunc;
pub struct FSetTimelineFinishedFunc_NewTimelineFinishedFunc;
pub struct FSetTimelinePostUpdateFunc_NewTimelinePostUpdateFunc;
pub struct FPlayQuantized_InDelegate;
pub struct FAddDisplayDelegate_Delegate;
pub struct FAsyncLoadOrCreateSaveGameForLocalPlayer_Delegate;
pub struct FAddEnvelopeFollowerDelegate_OnSubmixEnvelopeBP;
pub struct FAddSpectralAnalysisDelegate_OnSubmixSpectralAnalysisBP;
pub struct FRemoveEnvelopeFollowerDelegate_OnSubmixEnvelopeBP;
pub struct FRemoveSpectralAnalysisDelegate_OnSubmixSpectralAnalysisBP;
pub struct FActor_OnTakeAnyDamage;
pub struct FActor_OnTakePointDamage;
pub struct FActor_OnTakeRadialDamage;
pub struct FActor_OnActorBeginOverlap;
pub struct FActor_OnActorEndOverlap;
pub struct FActor_OnBeginCursorOver;
pub struct FActor_OnEndCursorOver;
pub struct FActor_OnClicked;
pub struct FActor_OnReleased;
pub struct FActor_OnInputTouchBegin;
pub struct FActor_OnInputTouchEnd;
pub struct FActor_OnInputTouchEnter;
pub struct FActor_OnInputTouchLeave;
pub struct FActor_OnActorHit;
pub struct FActor_OnDestroyed;
pub struct FActor_OnEndPlay;
pub struct FActorComponent_OnComponentActivated;
pub struct FActorComponent_OnComponentDeactivated;
pub struct FSceneComponent_PhysicsVolumeChangedDelegate;
pub struct FPrimitiveComponent_OnComponentHit;
pub struct FPrimitiveComponent_OnComponentBeginOverlap;
pub struct FPrimitiveComponent_OnComponentEndOverlap;
pub struct FPrimitiveComponent_OnComponentWake;
pub struct FPrimitiveComponent_OnComponentSleep;
pub struct FPrimitiveComponent_OnComponentPhysicsStateChanged;
pub struct FPrimitiveComponent_OnBeginCursorOver;
pub struct FPrimitiveComponent_OnEndCursorOver;
pub struct FPrimitiveComponent_OnClicked;
pub struct FPrimitiveComponent_OnReleased;
pub struct FPrimitiveComponent_OnInputTouchBegin;
pub struct FPrimitiveComponent_OnInputTouchEnd;
pub struct FPrimitiveComponent_OnInputTouchEnter;
pub struct FPrimitiveComponent_OnInputTouchLeave;
pub struct FController_OnInstigatedAnyDamage;
pub struct FController_OnPossessedPawnChanged;
pub struct FPawn_ReceiveControllerChangedDelegate;
pub struct FPawn_ReceiveRestartedDelegate;
pub struct FCharacter_OnReachedJumpApex;
pub struct FCharacter_LandedDelegate;
pub struct FCharacter_MovementModeChangedDelegate;
pub struct FCharacter_OnCharacterMovementUpdated;
pub struct FGameInstance_OnPawnControllerChangedDelegates;
pub struct FGameInstance_OnInputDeviceConnectionChange;
pub struct FGameInstance_OnUserInputDevicePairingChange;
pub struct FSkeletalMeshComponent_OnConstraintBroken;
pub struct FSkeletalMeshComponent_OnPlasticDeformation;
pub struct FSkeletalMeshComponent_OnAnimInitialized;
pub struct FAnimInstance_OnMontageBlendingOut;
pub struct FAnimInstance_OnMontageBlendedIn;
pub struct FAnimInstance_OnMontageStarted;
pub struct FAnimInstance_OnMontageEnded;
pub struct FAnimInstance_OnAllMontageInstancesEnded;
pub struct FAnimInstance_OnMontageSectionChanged;
pub struct FAnimSingleNodeInstance_PostEvaluateAnimEvent;
pub struct FPlatformInterfaceBase_Delegates;
pub struct FEmitter_OnParticleSpawn;
pub struct FEmitter_OnParticleBurst;
pub struct FEmitter_OnParticleDeath;
pub struct FEmitter_OnParticleCollide;
pub struct FInGameAdManager_ClickedBannerDelegates;
pub struct FInGameAdManager_ClosedAdDelegates;
pub struct FLevelStreaming_OnLevelLoaded;
pub struct FLevelStreaming_OnLevelUnloaded;
pub struct FLevelStreaming_OnLevelShown;
pub struct FLevelStreaming_OnLevelHidden;
pub struct FTimelineComponent_EventFunc;
pub struct FAnimDataModel_ModifiedEventDynamic;
pub struct FAsyncActionLoadPrimaryAsset_Completed;
pub struct FAsyncActionLoadPrimaryAssetClass_Completed;
pub struct FAsyncActionLoadPrimaryAssetList_Completed;
pub struct FAsyncActionLoadPrimaryAssetClassList_Completed;
pub struct FAsyncActionChangePrimaryAssetBundles_Completed;
pub struct FCanvasRenderTarget2D_OnCanvasRenderTargetUpdate;
pub struct FApplicationLifecycleComponent_ApplicationWillDeactivateDelegate;
pub struct FApplicationLifecycleComponent_ApplicationHasReactivatedDelegate;
pub struct FApplicationLifecycleComponent_ApplicationWillEnterBackgroundDelegate;
pub struct FApplicationLifecycleComponent_ApplicationHasEnteredForegroundDelegate;
pub struct FApplicationLifecycleComponent_ApplicationWillTerminateDelegate;
pub struct FApplicationLifecycleComponent_ApplicationShouldUnloadResourcesDelegate;
pub struct FApplicationLifecycleComponent_ApplicationReceivedStartupArgumentsDelegate;
pub struct FApplicationLifecycleComponent_OnTemperatureChangeDelegate;
pub struct FApplicationLifecycleComponent_OnLowPowerModeDelegate;
pub struct FAudioComponent_OnAudioPlayStateChanged;
pub struct FAudioComponent_OnAudioVirtualizationChanged;
pub struct FAudioComponent_OnAudioFinished;
pub struct FAudioComponent_OnAudioPlaybackPercent;
pub struct FAudioComponent_OnAudioSingleEnvelopeValue;
pub struct FAudioComponent_OnAudioMultiEnvelopeValue;
pub struct FAudioComponent_OnQueueSubtitles;
pub struct FForceFeedbackComponent_OnForceFeedbackFinished;
pub struct FInterpToMovementComponent_OnInterpToReverse;
pub struct FInterpToMovementComponent_OnInterpToStop;
pub struct FInterpToMovementComponent_OnWaitBeginDelegate;
pub struct FInterpToMovementComponent_OnWaitEndDelegate;
pub struct FInterpToMovementComponent_OnResetDelegate;
pub struct FPlatformEventsComponent_PlatformChangedToLaptopModeDelegate;
pub struct FPlatformEventsComponent_PlatformChangedToTabletModeDelegate;
pub struct FProjectileMovementComponent_OnProjectileBounce;
pub struct FProjectileMovementComponent_OnProjectileStop;
pub struct FDataDrivenCVarEngineSubsystem_OnDataDrivenCVarDelegate;
pub struct FAsyncActionHandleSaveGame_Completed;
pub struct FInputDeviceSubsystem_OnInputHardwareDeviceChanged;
pub struct FGameUserSettings_OnGameUserSettingsUINeedsUpdate;
pub struct FPlatformGameInstance_ApplicationWillDeactivateDelegate;
pub struct FPlatformGameInstance_ApplicationHasReactivatedDelegate;
pub struct FPlatformGameInstance_ApplicationWillEnterBackgroundDelegate;
pub struct FPlatformGameInstance_ApplicationHasEnteredForegroundDelegate;
pub struct FPlatformGameInstance_ApplicationWillTerminateDelegate;
pub struct FPlatformGameInstance_ApplicationShouldUnloadResourcesDelegate;
pub struct FPlatformGameInstance_ApplicationReceivedStartupArgumentsDelegate;
pub struct FPlatformGameInstance_ApplicationRegisteredForRemoteNotificationsDelegate;
pub struct FPlatformGameInstance_ApplicationRegisteredForUserNotificationsDelegate;
pub struct FPlatformGameInstance_ApplicationFailedToRegisterForRemoteNotificationsDelegate;
pub struct FPlatformGameInstance_ApplicationReceivedRemoteNotificationDelegate;
pub struct FPlatformGameInstance_ApplicationReceivedLocalNotificationDelegate;
pub struct FPlatformGameInstance_ApplicationReceivedScreenOrientationChangedNotificationDelegate;
pub struct FParticleSystemComponent_OnParticleSpawn;
pub struct FParticleSystemComponent_OnParticleBurst;
pub struct FParticleSystemComponent_OnParticleDeath;
pub struct FParticleSystemComponent_OnParticleCollide;
pub struct FParticleSystemComponent_OnSystemFinished;
pub struct FClusterUnionComponent_OnComponentAddedEvent;
pub struct FClusterUnionComponent_OnComponentRemovedEvent;
pub struct FClusterUnionComponent_OnComponentBoundsChangedEvent;
pub struct FChaosEventRelay_OnCollisionEvent;
pub struct FChaosEventRelay_OnBreakEvent;
pub struct FChaosEventRelay_OnRemovalEvent;
pub struct FChaosEventRelay_OnCrumblingEvent;
pub struct FPhysicsConstraintComponent_OnConstraintBroken;
pub struct FPhysicsConstraintComponent_OnPlasticDeformation;
pub struct FPlayerCameraManager_OnAudioFadeChangeEvent;
pub struct FPlayerState_OnPawnSet;
pub struct FSoundSubmix_OnSubmixRecordedFileDone;
pub struct FDataLayerManager_OnDataLayerInstanceRuntimeStateChanged;
pub struct FDataLayerSubsystem_OnDataLayerRuntimeStateChanged;
#[allow(non_camel_case_types)]
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
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ETextGender(pub u8);
impl ETextGender {
    pub const MASCULINE: ETextGender = ETextGender(0);
    pub const FEMININE: ETextGender = ETextGender(1);
    pub const NEUTER: ETextGender = ETextGender(2);
}
#[allow(non_camel_case_types)]
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
#[allow(non_camel_case_types)]
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
#[allow(non_camel_case_types)]
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
#[allow(non_camel_case_types)]
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
#[allow(non_camel_case_types)]
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
#[allow(non_camel_case_types)]
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
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ECollisionResponse(pub u8);
impl ECollisionResponse {
    pub const ECR_IGNORE: ECollisionResponse = ECollisionResponse(0);
    pub const ECR_OVERLAP: ECollisionResponse = ECollisionResponse(1);
    pub const ECR_BLOCK: ECollisionResponse = ECollisionResponse(2);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EInputEvent(pub u8);
impl EInputEvent {
    pub const IE_PRESSED: EInputEvent = EInputEvent(0);
    pub const IE_RELEASED: EInputEvent = EInputEvent(1);
    pub const IE_REPEAT: EInputEvent = EInputEvent(2);
    pub const IE_DOUBLE_CLICK: EInputEvent = EInputEvent(3);
    pub const IE_AXIS: EInputEvent = EInputEvent(4);
}
#[allow(non_camel_case_types)]
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
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EBloomMethod(pub u8);
impl EBloomMethod {
    pub const BM_SOG: EBloomMethod = EBloomMethod(0);
    pub const BM_FFT: EBloomMethod = EBloomMethod(1);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EAutoExposureMethod(pub u8);
impl EAutoExposureMethod {
    pub const AEM_HISTOGRAM: EAutoExposureMethod = EAutoExposureMethod(0);
    pub const AEM_BASIC: EAutoExposureMethod = EAutoExposureMethod(1);
    pub const AEM_MANUAL: EAutoExposureMethod = EAutoExposureMethod(2);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EDepthOfFieldMethod(pub u8);
impl EDepthOfFieldMethod {
    pub const DOFM_BOKEH_DOF: EDepthOfFieldMethod = EDepthOfFieldMethod(0);
    pub const DOFM_GAUSSIAN: EDepthOfFieldMethod = EDepthOfFieldMethod(1);
    pub const DOFM_CIRCLE_DOF: EDepthOfFieldMethod = EDepthOfFieldMethod(2);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ETemperatureMethod(pub u8);
impl ETemperatureMethod {
    pub const TEMP_WHITE_BALANCE: ETemperatureMethod = ETemperatureMethod(0);
    pub const TEMP_COLOR_TEMPERATURE: ETemperatureMethod = ETemperatureMethod(1);
}
#[allow(non_camel_case_types)]
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
#[allow(non_camel_case_types)]
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
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EReflectionMethod(pub u8);
impl EReflectionMethod {
    pub const NONE: EReflectionMethod = EReflectionMethod(0);
    pub const LUMEN: EReflectionMethod = EReflectionMethod(1);
    pub const SCREEN_SPACE: EReflectionMethod = EReflectionMethod(2);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EReflectionsType(pub u8);
impl EReflectionsType {
    pub const SCREEN_SPACE: EReflectionsType = EReflectionsType(0);
    pub const RAY_TRACING: EReflectionsType = EReflectionsType(1);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ELocalExposureMethod(pub u8);
impl ELocalExposureMethod {
    pub const BILATERAL: ELocalExposureMethod = ELocalExposureMethod(0);
    pub const FUSION: ELocalExposureMethod = ELocalExposureMethod(1);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ETranslucencyType(pub u8);
impl ETranslucencyType {
    pub const RASTER: ETranslucencyType = ETranslucencyType(0);
    pub const RAY_TRACED_DEPRECATED: ETranslucencyType = ETranslucencyType(1);
    pub const RAY_TRACING: ETranslucencyType = ETranslucencyType(1);
    pub const RAY_TRACED: ETranslucencyType = ETranslucencyType(2);
}
#[allow(non_camel_case_types)]
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
#[allow(non_camel_case_types)]
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
#[allow(non_camel_case_types)]
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
#[allow(non_camel_case_types)]
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
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ERichCurveInterpMode(pub u8);
impl ERichCurveInterpMode {
    pub const RCIM_LINEAR: ERichCurveInterpMode = ERichCurveInterpMode(0);
    pub const RCIM_CONSTANT: ERichCurveInterpMode = ERichCurveInterpMode(1);
    pub const RCIM_CUBIC: ERichCurveInterpMode = ERichCurveInterpMode(2);
    pub const RCIM_NONE: ERichCurveInterpMode = ERichCurveInterpMode(3);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ERichCurveTangentMode(pub u8);
impl ERichCurveTangentMode {
    pub const RCTM_AUTO: ERichCurveTangentMode = ERichCurveTangentMode(0);
    pub const RCTM_USER: ERichCurveTangentMode = ERichCurveTangentMode(1);
    pub const RCTM_BREAK: ERichCurveTangentMode = ERichCurveTangentMode(2);
    pub const RCTM_NONE: ERichCurveTangentMode = ERichCurveTangentMode(3);
    pub const RCTM_SMART_AUTO: ERichCurveTangentMode = ERichCurveTangentMode(4);
}
#[allow(non_camel_case_types)]
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
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EPinContainerType(pub u8);
impl EPinContainerType {
    pub const NONE: EPinContainerType = EPinContainerType(0);
    pub const ARRAY: EPinContainerType = EPinContainerType(1);
    pub const SET: EPinContainerType = EPinContainerType(2);
    pub const MAP: EPinContainerType = EPinContainerType(3);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EEdGraphPinDirection(pub u8);
impl EEdGraphPinDirection {
    pub const EGPD_INPUT: EEdGraphPinDirection = EEdGraphPinDirection(0);
    pub const EGPD_OUTPUT: EEdGraphPinDirection = EEdGraphPinDirection(1);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EGraphType(pub u8);
impl EGraphType {
    pub const GT_FUNCTION: EGraphType = EGraphType(0);
    pub const GT_UBERGRAPH: EGraphType = EGraphType(1);
    pub const GT_MACRO: EGraphType = EGraphType(2);
    pub const GT_ANIMATION: EGraphType = EGraphType(3);
    pub const GT_STATE_MACHINE: EGraphType = EGraphType(4);
}
#[allow(non_camel_case_types)]
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
#[allow(non_camel_case_types)]
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
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EAnimAlphaInputType(pub u8);
impl EAnimAlphaInputType {
    pub const FLOAT: EAnimAlphaInputType = EAnimAlphaInputType(0);
    pub const BOOL: EAnimAlphaInputType = EAnimAlphaInputType(1);
    pub const CURVE: EAnimAlphaInputType = EAnimAlphaInputType(2);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EBoneAxis(pub u8);
impl EBoneAxis {
    pub const BA_X: EBoneAxis = EBoneAxis(0);
    pub const BA_Y: EBoneAxis = EBoneAxis(1);
    pub const BA_Z: EBoneAxis = EBoneAxis(2);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EAnimSyncGroupScope(pub u8);
impl EAnimSyncGroupScope {
    pub const LOCAL: EAnimSyncGroupScope = EAnimSyncGroupScope(0);
    pub const COMPONENT: EAnimSyncGroupScope = EAnimSyncGroupScope(1);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EAnimSyncMethod(pub u8);
impl EAnimSyncMethod {
    pub const DO_NOT_SYNC: EAnimSyncMethod = EAnimSyncMethod(0);
    pub const SYNC_GROUP: EAnimSyncMethod = EAnimSyncMethod(1);
    pub const GRAPH: EAnimSyncMethod = EAnimSyncMethod(2);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EBoneControlSpace(pub u8);
impl EBoneControlSpace {
    pub const BCS_WORLD_SPACE: EBoneControlSpace = EBoneControlSpace(0);
    pub const BCS_COMPONENT_SPACE: EBoneControlSpace = EBoneControlSpace(1);
    pub const BCS_PARENT_BONE_SPACE: EBoneControlSpace = EBoneControlSpace(2);
    pub const BCS_BONE_SPACE: EBoneControlSpace = EBoneControlSpace(3);
}
#[allow(non_camel_case_types)]
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
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct AnimPhysTwistAxis(pub u8);
impl AnimPhysTwistAxis {
    pub const AXIS_X: AnimPhysTwistAxis = AnimPhysTwistAxis(0);
    pub const AXIS_Y: AnimPhysTwistAxis = AnimPhysTwistAxis(1);
    pub const AXIS_Z: AnimPhysTwistAxis = AnimPhysTwistAxis(2);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct AnimPhysCollisionType(pub u8);
impl AnimPhysCollisionType {
    pub const CO_M: AnimPhysCollisionType = AnimPhysCollisionType(0);
    pub const CUSTOM_SPHERE: AnimPhysCollisionType = AnimPhysCollisionType(1);
    pub const INNER_SPHERE: AnimPhysCollisionType = AnimPhysCollisionType(2);
    pub const OUTER_SPHERE: AnimPhysCollisionType = AnimPhysCollisionType(3);
}
#[allow(non_camel_case_types)]
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
#[allow(non_camel_case_types)]
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
#[allow(non_camel_case_types)]
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
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ECameraShakePlaySpace(pub u8);
impl ECameraShakePlaySpace {
    pub const CAMERA_LOCAL: ECameraShakePlaySpace = ECameraShakePlaySpace(0);
    pub const WORLD: ECameraShakePlaySpace = ECameraShakePlaySpace(1);
    pub const USER_DEFINED: ECameraShakePlaySpace = ECameraShakePlaySpace(2);
}
#[allow(non_camel_case_types)]
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
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EVectorQuantization(pub u8);
impl EVectorQuantization {
    pub const ROUND_WHOLE_NUMBER: EVectorQuantization = EVectorQuantization(0);
    pub const ROUND_ONE_DECIMAL: EVectorQuantization = EVectorQuantization(1);
    pub const ROUND_TWO_DECIMALS: EVectorQuantization = EVectorQuantization(2);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ERotatorQuantization(pub u8);
impl ERotatorQuantization {
    pub const BYTE_COMPONENTS: ERotatorQuantization = ERotatorQuantization(0);
    pub const SHORT_COMPONENTS: ERotatorQuantization = ERotatorQuantization(1);
}
#[allow(non_camel_case_types)]
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
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EAnimInterpolationType(pub u8);
impl EAnimInterpolationType {
    pub const LINEAR: EAnimInterpolationType = EAnimInterpolationType(0);
    pub const STEP: EAnimInterpolationType = EAnimInterpolationType(1);
}
#[allow(non_camel_case_types)]
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
#[allow(non_camel_case_types)]
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
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ECollectionScriptingShareType(pub u8);
impl ECollectionScriptingShareType {
    pub const LOCAL: ECollectionScriptingShareType = ECollectionScriptingShareType(0);
    pub const PRIVATE: ECollectionScriptingShareType = ECollectionScriptingShareType(1);
    pub const SHARED: ECollectionScriptingShareType = ECollectionScriptingShareType(2);
}
#[allow(non_camel_case_types)]
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
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ESplineMeshAxis(pub u8);
impl ESplineMeshAxis {
    pub const X: ESplineMeshAxis = ESplineMeshAxis(0);
    pub const Y: ESplineMeshAxis = ESplineMeshAxis(1);
    pub const Z: ESplineMeshAxis = ESplineMeshAxis(2);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ERawCurveTrackTypes(pub u8);
impl ERawCurveTrackTypes {
    pub const RCT_FLOAT: ERawCurveTrackTypes = ERawCurveTrackTypes(0);
    pub const RCT_VECTOR: ERawCurveTrackTypes = ERawCurveTrackTypes(1);
    pub const RCT_TRANSFORM: ERawCurveTrackTypes = ERawCurveTrackTypes(2);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ETransformCurveChannel(pub u8);
impl ETransformCurveChannel {
    pub const POSITION: ETransformCurveChannel = ETransformCurveChannel(0);
    pub const ROTATION: ETransformCurveChannel = ETransformCurveChannel(1);
    pub const SCALE: ETransformCurveChannel = ETransformCurveChannel(2);
    pub const INVALID: ETransformCurveChannel = ETransformCurveChannel(3);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EVectorCurveChannel(pub u8);
impl EVectorCurveChannel {
    pub const X: EVectorCurveChannel = EVectorCurveChannel(0);
    pub const Y: EVectorCurveChannel = EVectorCurveChannel(1);
    pub const Z: EVectorCurveChannel = EVectorCurveChannel(2);
    pub const INVALID: EVectorCurveChannel = EVectorCurveChannel(3);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EAdditiveAnimationType(pub u8);
impl EAdditiveAnimationType {
    pub const AAT_NONE: EAdditiveAnimationType = EAdditiveAnimationType(0);
    pub const AAT_LOCAL_SPACE_BASE: EAdditiveAnimationType = EAdditiveAnimationType(1);
    pub const AAT_ROTATION_OFFSET_MESH_SPACE: EAdditiveAnimationType = EAdditiveAnimationType(
        2,
    );
}
#[allow(non_camel_case_types)]
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
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ECameraProjectionMode(pub u8);
impl ECameraProjectionMode {
    pub const PERSPECTIVE: ECameraProjectionMode = ECameraProjectionMode(0);
    pub const ORTHOGRAPHIC: ECameraProjectionMode = ECameraProjectionMode(1);
}
#[allow(non_camel_case_types)]
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
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EFontImportCharacterSet(pub u8);
impl EFontImportCharacterSet {
    pub const FONT_ICS_DEFAULT: EFontImportCharacterSet = EFontImportCharacterSet(0);
    pub const FONT_ICS_ANSI: EFontImportCharacterSet = EFontImportCharacterSet(1);
    pub const FONT_ICS_SYMBOL: EFontImportCharacterSet = EFontImportCharacterSet(2);
}
#[allow(non_camel_case_types)]
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
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EAttenuationShape(pub u8);
impl EAttenuationShape {
    pub const SPHERE: EAttenuationShape = EAttenuationShape(0);
    pub const CAPSULE: EAttenuationShape = EAttenuationShape(1);
    pub const BOX: EAttenuationShape = EAttenuationShape(2);
    pub const CONE: EAttenuationShape = EAttenuationShape(3);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ENaturalSoundFalloffMode(pub u8);
impl ENaturalSoundFalloffMode {
    pub const CONTINUES: ENaturalSoundFalloffMode = ENaturalSoundFalloffMode(0);
    pub const SILENT: ENaturalSoundFalloffMode = ENaturalSoundFalloffMode(1);
    pub const HOLD: ENaturalSoundFalloffMode = ENaturalSoundFalloffMode(2);
}
#[allow(non_camel_case_types)]
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
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EDrawDebugTrace(pub u8);
impl EDrawDebugTrace {
    pub const NONE: EDrawDebugTrace = EDrawDebugTrace(0);
    pub const FOR_ONE_FRAME: EDrawDebugTrace = EDrawDebugTrace(1);
    pub const FOR_DURATION: EDrawDebugTrace = EDrawDebugTrace(2);
    pub const PERSISTENT: EDrawDebugTrace = EDrawDebugTrace(3);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ELevelInstanceCreationType(pub u8);
impl ELevelInstanceCreationType {
    pub const LEVEL_INSTANCE: ELevelInstanceCreationType = ELevelInstanceCreationType(0);
    pub const PACKED_LEVEL_ACTOR: ELevelInstanceCreationType = ELevelInstanceCreationType(
        1,
    );
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ELevelInstancePivotType(pub u8);
impl ELevelInstancePivotType {
    pub const CENTER_MIN_Z: ELevelInstancePivotType = ELevelInstancePivotType(0);
    pub const CENTER: ELevelInstancePivotType = ELevelInstancePivotType(1);
    pub const ACTOR: ELevelInstancePivotType = ELevelInstancePivotType(2);
    pub const WORLD_ORIGIN: ELevelInstancePivotType = ELevelInstancePivotType(3);
}
#[allow(non_camel_case_types)]
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
#[allow(non_camel_case_types)]
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
#[allow(non_camel_case_types)]
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
#[allow(non_camel_case_types)]
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
#[allow(non_camel_case_types)]
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
#[allow(non_camel_case_types)]
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
#[allow(non_camel_case_types)]
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
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EMaterialLayerLinkState(pub u8);
impl EMaterialLayerLinkState {
    pub const UNINITIALIZED: EMaterialLayerLinkState = EMaterialLayerLinkState(0);
    pub const LINKED_TO_PARENT: EMaterialLayerLinkState = EMaterialLayerLinkState(1);
    pub const UNLINKED_FROM_PARENT: EMaterialLayerLinkState = EMaterialLayerLinkState(2);
    pub const NOT_FROM_PARENT: EMaterialLayerLinkState = EMaterialLayerLinkState(3);
}
#[allow(non_camel_case_types)]
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
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ENavLinkDirection(pub u8);
impl ENavLinkDirection {
    pub const BOTH_WAYS: ENavLinkDirection = ENavLinkDirection(0);
    pub const LEFT_TO_RIGHT: ENavLinkDirection = ENavLinkDirection(1);
    pub const RIGHT_TO_LEFT: ENavLinkDirection = ENavLinkDirection(2);
}
#[allow(non_camel_case_types)]
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
#[allow(non_camel_case_types)]
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
#[allow(non_camel_case_types)]
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
#[allow(non_camel_case_types)]
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
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EParticleCollisionMode(pub u8);
impl EParticleCollisionMode {
    pub const SCENE_DEPTH: EParticleCollisionMode = EParticleCollisionMode(0);
    pub const DISTANCE_FIELD: EParticleCollisionMode = EParticleCollisionMode(1);
}
#[allow(non_camel_case_types)]
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
#[allow(non_camel_case_types)]
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
#[allow(non_camel_case_types)]
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
#[allow(non_camel_case_types)]
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
#[allow(non_camel_case_types)]
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
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ETimelineLengthMode(pub u8);
impl ETimelineLengthMode {
    pub const TL_TIMELINE_LENGTH: ETimelineLengthMode = ETimelineLengthMode(0);
    pub const TL_LAST_KEY_FRAME: ETimelineLengthMode = ETimelineLengthMode(1);
}
#[allow(non_camel_case_types)]
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
#[allow(non_camel_case_types)]
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
#[allow(non_camel_case_types)]
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
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EComponentMobility(pub u8);
impl EComponentMobility {
    pub const STATIC: EComponentMobility = EComponentMobility(0);
    pub const STATIONARY: EComponentMobility = EComponentMobility(1);
    pub const MOVABLE: EComponentMobility = EComponentMobility(2);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EHLODBatchingPolicy(pub u8);
impl EHLODBatchingPolicy {
    pub const NONE: EHLODBatchingPolicy = EHLODBatchingPolicy(0);
    pub const MESH_SECTION: EHLODBatchingPolicy = EHLODBatchingPolicy(1);
    pub const INSTANCING: EHLODBatchingPolicy = EHLODBatchingPolicy(2);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EAnimLinkMethod(pub u8);
impl EAnimLinkMethod {
    pub const ABSOLUTE: EAnimLinkMethod = EAnimLinkMethod(0);
    pub const RELATIVE: EAnimLinkMethod = EAnimLinkMethod(1);
    pub const PROPORTIONAL: EAnimLinkMethod = EAnimLinkMethod(2);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EAnimNotifyEventType(pub u8);
impl EAnimNotifyEventType {
    pub const BEGIN: EAnimNotifyEventType = EAnimNotifyEventType(0);
    pub const END: EAnimNotifyEventType = EAnimNotifyEventType(1);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EMontageBlendMode(pub u8);
impl EMontageBlendMode {
    pub const STANDARD: EMontageBlendMode = EMontageBlendMode(0);
    pub const INERTIALIZATION: EMontageBlendMode = EMontageBlendMode(1);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EMontageNotifyTickType(pub u8);
impl EMontageNotifyTickType {
    pub const QUEUED: EMontageNotifyTickType = EMontageNotifyTickType(0);
    pub const BRANCHING_POINT: EMontageNotifyTickType = EMontageNotifyTickType(1);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ENotifyFilterType(pub u8);
impl ENotifyFilterType {
    pub const NO_FILTERING: ENotifyFilterType = ENotifyFilterType(0);
    pub const LOD: ENotifyFilterType = ENotifyFilterType(1);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EEvaluatorDataSource(pub u8);
impl EEvaluatorDataSource {
    pub const EDS_SOURCE_POSE: EEvaluatorDataSource = EEvaluatorDataSource(0);
    pub const EDS_DESTINATION_POSE: EEvaluatorDataSource = EEvaluatorDataSource(1);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EEvaluatorMode(pub u8);
impl EEvaluatorMode {
    pub const EM_STANDARD: EEvaluatorMode = EEvaluatorMode(0);
    pub const EM_FREEZE: EEvaluatorMode = EEvaluatorMode(1);
    pub const EM_DELAYED_FREEZE: EEvaluatorMode = EEvaluatorMode(2);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ETransitionLogicType(pub u8);
impl ETransitionLogicType {
    pub const TLT_STANDARD_BLEND: ETransitionLogicType = ETransitionLogicType(0);
    pub const TLT_INERTIALIZATION: ETransitionLogicType = ETransitionLogicType(1);
    pub const TLT_CUSTOM: ETransitionLogicType = ETransitionLogicType(2);
}
#[allow(non_camel_case_types)]
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
#[allow(non_camel_case_types)]
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
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EPostCopyOperation(pub u8);
impl EPostCopyOperation {
    pub const NONE: EPostCopyOperation = EPostCopyOperation(0);
    pub const LOGICAL_NEGATE_BOOL: EPostCopyOperation = EPostCopyOperation(1);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EMirrorRowType(pub u8);
impl EMirrorRowType {
    pub const BONE: EMirrorRowType = EMirrorRowType(0);
    pub const ANIMATION_NOTIFY: EMirrorRowType = EMirrorRowType(1);
    pub const CURVE: EMirrorRowType = EMirrorRowType(2);
    pub const SYNC_MARKER: EMirrorRowType = EMirrorRowType(3);
    pub const CUSTOM: EMirrorRowType = EMirrorRowType(4);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EMirrorFindReplaceMethod(pub u8);
impl EMirrorFindReplaceMethod {
    pub const PREFIX: EMirrorFindReplaceMethod = EMirrorFindReplaceMethod(0);
    pub const SUFFIX: EMirrorFindReplaceMethod = EMirrorFindReplaceMethod(1);
    pub const REGULAR_EXPRESSION: EMirrorFindReplaceMethod = EMirrorFindReplaceMethod(2);
}
#[allow(non_camel_case_types)]
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
#[allow(non_camel_case_types)]
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
#[allow(non_camel_case_types)]
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
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ESubmixSendStage(pub u8);
impl ESubmixSendStage {
    pub const POST_DISTANCE_ATTENUATION: ESubmixSendStage = ESubmixSendStage(0);
    pub const PRE_DISTANCE_ATTENUATION: ESubmixSendStage = ESubmixSendStage(1);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ESendLevelControlMethod(pub u8);
impl ESendLevelControlMethod {
    pub const LINEAR: ESendLevelControlMethod = ESendLevelControlMethod(0);
    pub const CUSTOM_CURVE: ESendLevelControlMethod = ESendLevelControlMethod(1);
    pub const MANUAL: ESendLevelControlMethod = ESendLevelControlMethod(2);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ECameraShakeDurationType(pub u8);
impl ECameraShakeDurationType {
    pub const FIXED: ECameraShakeDurationType = ECameraShakeDurationType(0);
    pub const INFINITE: ECameraShakeDurationType = ECameraShakeDurationType(1);
    pub const CUSTOM: ECameraShakeDurationType = ECameraShakeDurationType(2);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ESyncOption(pub u8);
impl ESyncOption {
    pub const DRIVE: ESyncOption = ESyncOption(0);
    pub const PASSIVE: ESyncOption = ESyncOption(1);
    pub const DISABLED: ESyncOption = ESyncOption(2);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ESplinePointType(pub u8);
impl ESplinePointType {
    pub const LINEAR: ESplinePointType = ESplinePointType(0);
    pub const CURVE: ESplinePointType = ESplinePointType(1);
    pub const CONSTANT: ESplinePointType = ESplinePointType(2);
    pub const CURVE_CLAMPED: ESplinePointType = ESplinePointType(3);
    pub const CURVE_CUSTOM_TANGENT: ESplinePointType = ESplinePointType(4);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct FDataDrivenCVarType(pub u8);
impl FDataDrivenCVarType {
    pub const C_VAR_FLOAT: FDataDrivenCVarType = FDataDrivenCVarType(0);
    pub const C_VAR_INT: FDataDrivenCVarType = FDataDrivenCVarType(1);
    pub const C_VAR_BOOL: FDataDrivenCVarType = FDataDrivenCVarType(2);
}
#[allow(non_camel_case_types)]
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
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ENaniteShapePreservation(pub u8);
impl ENaniteShapePreservation {
    pub const NONE: ENaniteShapePreservation = ENaniteShapePreservation(0);
    pub const PRESERVE_AREA: ENaniteShapePreservation = ENaniteShapePreservation(1);
    pub const VOXELIZE: ENaniteShapePreservation = ENaniteShapePreservation(2);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ENaniteGenerateFallback(pub u8);
impl ENaniteGenerateFallback {
    pub const PLATFORM_DEFAULT: ENaniteGenerateFallback = ENaniteGenerateFallback(0);
    pub const ENABLED: ENaniteGenerateFallback = ENaniteGenerateFallback(1);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ENaniteFallbackTarget(pub u8);
impl ENaniteFallbackTarget {
    pub const AUTO: ENaniteFallbackTarget = ENaniteFallbackTarget(0);
    pub const PERCENT_TRIANGLES: ENaniteFallbackTarget = ENaniteFallbackTarget(1);
    pub const RELATIVE_ERROR: ENaniteFallbackTarget = ENaniteFallbackTarget(2);
}
#[allow(non_camel_case_types)]
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
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ERootMotionAccumulateMode(pub u8);
impl ERootMotionAccumulateMode {
    pub const OVERRIDE: ERootMotionAccumulateMode = ERootMotionAccumulateMode(0);
    pub const ADDITIVE: ERootMotionAccumulateMode = ERootMotionAccumulateMode(1);
}
#[allow(non_camel_case_types)]
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
#[allow(non_camel_case_types)]
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
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EUVOutput(pub u8);
impl EUVOutput {
    pub const DO_NOT_OUTPUT_CHANNEL: EUVOutput = EUVOutput(0);
    pub const OUTPUT_CHANNEL: EUVOutput = EUVOutput(1);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EMeshApproximationType(pub u8);
impl EMeshApproximationType {
    pub const MESH_AND_MATERIALS: EMeshApproximationType = EMeshApproximationType(0);
    pub const MESH_SHAPE_ONLY: EMeshApproximationType = EMeshApproximationType(1);
}
#[allow(non_camel_case_types)]
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
#[allow(non_camel_case_types)]
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
#[allow(non_camel_case_types)]
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
#[allow(non_camel_case_types)]
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
#[allow(non_camel_case_types)]
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
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EMaterialMergeType(pub u8);
impl EMaterialMergeType {
    pub const MATERIAL_MERGE_TYPE_DEFAULT: EMaterialMergeType = EMaterialMergeType(0);
    pub const MATERIAL_MERGE_TYPE_SIMPLYGON: EMaterialMergeType = EMaterialMergeType(1);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EMeshLODSelectionType(pub u8);
impl EMeshLODSelectionType {
    pub const ALL_LO_DS: EMeshLODSelectionType = EMeshLODSelectionType(0);
    pub const SPECIFIC_LOD: EMeshLODSelectionType = EMeshLODSelectionType(1);
    pub const CALCULATE_LOD: EMeshLODSelectionType = EMeshLODSelectionType(2);
    pub const LOWEST_DETAIL_LOD: EMeshLODSelectionType = EMeshLODSelectionType(3);
}
#[allow(non_camel_case_types)]
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
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ELandscapeCullingPrecision(pub u8);
impl ELandscapeCullingPrecision {
    pub const HIGH: ELandscapeCullingPrecision = ELandscapeCullingPrecision(0);
    pub const MEDIUM: ELandscapeCullingPrecision = ELandscapeCullingPrecision(1);
    pub const LOW: ELandscapeCullingPrecision = ELandscapeCullingPrecision(2);
}
#[allow(non_camel_case_types)]
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
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ELightmapType(pub u8);
impl ELightmapType {
    pub const DEFAULT: ELightmapType = ELightmapType(0);
    pub const FORCE_SURFACE: ELightmapType = ELightmapType(1);
    pub const FORCE_VOLUMETRIC: ELightmapType = ELightmapType(2);
}
#[allow(non_camel_case_types)]
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
#[allow(non_camel_case_types)]
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
#[allow(non_camel_case_types)]
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
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EDetailMode(pub u8);
impl EDetailMode {
    pub const DM_LOW: EDetailMode = EDetailMode(0);
    pub const DM_MEDIUM: EDetailMode = EDetailMode(1);
    pub const DM_HIGH: EDetailMode = EDetailMode(2);
    pub const DM_EPIC: EDetailMode = EDetailMode(3);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EImportanceWeight(pub u8);
impl EImportanceWeight {
    pub const LUMINANCE: EImportanceWeight = EImportanceWeight(0);
    pub const RED: EImportanceWeight = EImportanceWeight(1);
    pub const GREEN: EImportanceWeight = EImportanceWeight(2);
    pub const BLUE: EImportanceWeight = EImportanceWeight(3);
    pub const ALPHA: EImportanceWeight = EImportanceWeight(4);
}
#[allow(non_camel_case_types)]
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
#[allow(non_camel_case_types)]
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
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EMaterialShaderFrequency(pub u8);
impl EMaterialShaderFrequency {
    pub const VERTEX: EMaterialShaderFrequency = EMaterialShaderFrequency(1);
    pub const PIXEL: EMaterialShaderFrequency = EMaterialShaderFrequency(8);
    pub const COMPUTE: EMaterialShaderFrequency = EMaterialShaderFrequency(32);
    pub const ANY: EMaterialShaderFrequency = EMaterialShaderFrequency(41);
}
#[allow(non_camel_case_types)]
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
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EDerivativeStatus(pub u8);
impl EDerivativeStatus {
    pub const NOT_AWARE: EDerivativeStatus = EDerivativeStatus(0);
    pub const NOT_VALID: EDerivativeStatus = EDerivativeStatus(1);
    pub const ZERO: EDerivativeStatus = EDerivativeStatus(2);
    pub const VALID: EDerivativeStatus = EDerivativeStatus(3);
}
#[allow(non_camel_case_types)]
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
#[allow(non_camel_case_types)]
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
#[allow(non_camel_case_types)]
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
#[allow(non_camel_case_types)]
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
#[allow(non_camel_case_types)]
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
#[allow(non_camel_case_types)]
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
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EAttachLocation(pub u8);
impl EAttachLocation {
    pub const KEEP_RELATIVE_OFFSET: EAttachLocation = EAttachLocation(0);
    pub const KEEP_WORLD_POSITION: EAttachLocation = EAttachLocation(1);
    pub const SNAP_TO_TARGET: EAttachLocation = EAttachLocation(2);
    pub const SNAP_TO_TARGET_INCLUDING_SCALE: EAttachLocation = EAttachLocation(3);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EPSCPoolMethod(pub u8);
impl EPSCPoolMethod {
    pub const NONE: EPSCPoolMethod = EPSCPoolMethod(0);
    pub const AUTO_RELEASE: EPSCPoolMethod = EPSCPoolMethod(1);
    pub const MANUAL_RELEASE: EPSCPoolMethod = EPSCPoolMethod(2);
    pub const MANUAL_RELEASE_ON_COMPLETE: EPSCPoolMethod = EPSCPoolMethod(3);
    pub const FREE_IN_POOL: EPSCPoolMethod = EPSCPoolMethod(4);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EAngularDriveMode(pub u8);
impl EAngularDriveMode {
    pub const SLERP: EAngularDriveMode = EAngularDriveMode(0);
    pub const TWIST_AND_SWING: EAngularDriveMode = EAngularDriveMode(1);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EPhysicsReplicationMode(pub u8);
impl EPhysicsReplicationMode {
    pub const DEFAULT: EPhysicsReplicationMode = EPhysicsReplicationMode(0);
    pub const PREDICTIVE_INTERPOLATION: EPhysicsReplicationMode = EPhysicsReplicationMode(
        1,
    );
    pub const RESIMULATION: EPhysicsReplicationMode = EPhysicsReplicationMode(2);
}
#[allow(non_camel_case_types)]
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
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EPropertyAccessObjectType(pub u8);
impl EPropertyAccessObjectType {
    pub const NONE: EPropertyAccessObjectType = EPropertyAccessObjectType(0);
    pub const OBJECT: EPropertyAccessObjectType = EPropertyAccessObjectType(1);
    pub const WEAK_OBJECT: EPropertyAccessObjectType = EPropertyAccessObjectType(2);
    pub const SOFT_OBJECT: EPropertyAccessObjectType = EPropertyAccessObjectType(3);
}
#[allow(non_camel_case_types)]
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
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ENeuralProfileFormat(pub u8);
impl ENeuralProfileFormat {
    pub const TYPE32: ENeuralProfileFormat = ENeuralProfileFormat(0);
    pub const TYPE16: ENeuralProfileFormat = ENeuralProfileFormat(1);
}
#[allow(non_camel_case_types)]
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
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ENeuralModelTileType(pub u8);
impl ENeuralModelTileType {
    pub const ONE_BY_ONE: ENeuralModelTileType = ENeuralModelTileType(0);
    pub const TWO_BY_TWO: ENeuralModelTileType = ENeuralModelTileType(1);
    pub const FOUR_BY_FOUR: ENeuralModelTileType = ENeuralModelTileType(2);
    pub const EIGHT_BY_EIGHT: ENeuralModelTileType = ENeuralModelTileType(3);
    pub const AUTO: ENeuralModelTileType = ENeuralModelTileType(4);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ETileOverlapResolveType(pub u8);
impl ETileOverlapResolveType {
    pub const IGNORE: ETileOverlapResolveType = ETileOverlapResolveType(0);
    pub const FEATHERING: ETileOverlapResolveType = ETileOverlapResolveType(1);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ESpecularProfileFormat(pub u8);
impl ESpecularProfileFormat {
    pub const VIEW_LIGHT_VECTOR: ESpecularProfileFormat = ESpecularProfileFormat(0);
    pub const HALF_VECTOR: ESpecularProfileFormat = ESpecularProfileFormat(1);
}
#[allow(non_camel_case_types)]
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
#[allow(non_camel_case_types)]
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
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EBoneFilterActionOption(pub u8);
impl EBoneFilterActionOption {
    pub const REMOVE: EBoneFilterActionOption = EBoneFilterActionOption(0);
    pub const KEEP: EBoneFilterActionOption = EBoneFilterActionOption(1);
    pub const INVALID: EBoneFilterActionOption = EBoneFilterActionOption(2);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ESkinCacheUsage(pub u8);
impl ESkinCacheUsage {
    pub const AUTO: ESkinCacheUsage = ESkinCacheUsage(0);
    pub const DISABLED: ESkinCacheUsage = ESkinCacheUsage(255);
    pub const ENABLED: ESkinCacheUsage = ESkinCacheUsage(1);
}
#[allow(non_camel_case_types)]
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
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EAirAbsorptionMethod(pub u8);
impl EAirAbsorptionMethod {
    pub const LINEAR: EAirAbsorptionMethod = EAirAbsorptionMethod(0);
    pub const CUSTOM_CURVE: EAirAbsorptionMethod = EAirAbsorptionMethod(1);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EReverbSendMethod(pub u8);
impl EReverbSendMethod {
    pub const LINEAR: EReverbSendMethod = EReverbSendMethod(0);
    pub const CUSTOM_CURVE: EReverbSendMethod = EReverbSendMethod(1);
    pub const MANUAL: EReverbSendMethod = EReverbSendMethod(2);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EPriorityAttenuationMethod(pub u8);
impl EPriorityAttenuationMethod {
    pub const LINEAR: EPriorityAttenuationMethod = EPriorityAttenuationMethod(0);
    pub const CUSTOM_CURVE: EPriorityAttenuationMethod = EPriorityAttenuationMethod(1);
    pub const MANUAL: EPriorityAttenuationMethod = EPriorityAttenuationMethod(2);
}
#[allow(non_camel_case_types)]
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
#[allow(non_camel_case_types)]
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
#[allow(non_camel_case_types)]
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
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EAudioOutputTarget(pub u8);
impl EAudioOutputTarget {
    pub const SPEAKER: EAudioOutputTarget = EAudioOutputTarget(0);
    pub const CONTROLLER: EAudioOutputTarget = EAudioOutputTarget(1);
    pub const CONTROLLER_FALLBACK_TO_SPEAKER: EAudioOutputTarget = EAudioOutputTarget(2);
}
#[allow(non_camel_case_types)]
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
#[allow(non_camel_case_types)]
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
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EConcurrencyVolumeScaleMode(pub i32);
impl EConcurrencyVolumeScaleMode {
    pub const DEFAULT: EConcurrencyVolumeScaleMode = EConcurrencyVolumeScaleMode(0);
    pub const DISTANCE: EConcurrencyVolumeScaleMode = EConcurrencyVolumeScaleMode(1);
    pub const PRIORITY: EConcurrencyVolumeScaleMode = EConcurrencyVolumeScaleMode(2);
}
#[allow(non_camel_case_types)]
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
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EModulationRouting(pub u8);
impl EModulationRouting {
    pub const DISABLE: EModulationRouting = EModulationRouting(0);
    pub const INHERIT: EModulationRouting = EModulationRouting(1);
    pub const OVERRIDE: EModulationRouting = EModulationRouting(2);
    pub const UNION: EModulationRouting = EModulationRouting(3);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ModulationParamMode(pub u8);
impl ModulationParamMode {
    pub const MPM_NORMAL: ModulationParamMode = ModulationParamMode(0);
    pub const MPM_ABS: ModulationParamMode = ModulationParamMode(1);
    pub const MPM_DIRECT: ModulationParamMode = ModulationParamMode(2);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EOptimizationType(pub u8);
impl EOptimizationType {
    pub const OT_NUM_OF_TRIANGLES: EOptimizationType = EOptimizationType(0);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ESubtitleDurationType(pub u8);
impl ESubtitleDurationType {
    pub const USE_SOUND_DURATION: ESubtitleDurationType = ESubtitleDurationType(0);
    pub const USE_DURATION_PROPERTY: ESubtitleDurationType = ESubtitleDurationType(1);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ESubtitleType(pub u8);
impl ESubtitleType {
    pub const SUBTITLE: ESubtitleType = ESubtitleType(0);
    pub const CLOSED_CAPTION: ESubtitleType = ESubtitleType(1);
    pub const AUDIO_DESCRIPTION: ESubtitleType = ESubtitleType(2);
}
#[allow(non_camel_case_types)]
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
#[allow(non_camel_case_types)]
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
#[allow(non_camel_case_types)]
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
#[allow(non_camel_case_types)]
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
#[allow(non_camel_case_types)]
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
#[allow(non_camel_case_types)]
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
#[allow(non_camel_case_types)]
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
#[allow(non_camel_case_types)]
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
#[allow(non_camel_case_types)]
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
#[allow(non_camel_case_types)]
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
#[allow(non_camel_case_types)]
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
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ETextureMipLoadOptions(pub u8);
impl ETextureMipLoadOptions {
    pub const DEFAULT: ETextureMipLoadOptions = ETextureMipLoadOptions(0);
    pub const ALL_MIPS: ETextureMipLoadOptions = ETextureMipLoadOptions(1);
    pub const ONLY_FIRST_MIP: ETextureMipLoadOptions = ETextureMipLoadOptions(2);
}
#[allow(non_camel_case_types)]
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
#[allow(non_camel_case_types)]
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
#[allow(non_camel_case_types)]
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
#[allow(non_camel_case_types)]
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
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EExportHLODMeshOrigin(pub u8);
impl EExportHLODMeshOrigin {
    pub const ACTOR: EExportHLODMeshOrigin = EExportHLODMeshOrigin(0);
    pub const WORLD: EExportHLODMeshOrigin = EExportHLODMeshOrigin(1);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EVolumeLightingMethod(pub u8);
impl EVolumeLightingMethod {
    pub const VLM_VOLUMETRIC_LIGHTMAP: EVolumeLightingMethod = EVolumeLightingMethod(0);
    pub const VLM_SPARSE_VOLUME_LIGHTING_SAMPLES: EVolumeLightingMethod = EVolumeLightingMethod(
        1,
    );
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct TextureFilter(pub u8);
impl TextureFilter {
    pub const TF_NEAREST: TextureFilter = TextureFilter(0);
    pub const TF_BILINEAR: TextureFilter = TextureFilter(1);
    pub const TF_TRILINEAR: TextureFilter = TextureFilter(2);
    pub const TF_DEFAULT: TextureFilter = TextureFilter(3);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct TextureAddress(pub u8);
impl TextureAddress {
    pub const TA_WRAP: TextureAddress = TextureAddress(0);
    pub const TA_CLAMP: TextureAddress = TextureAddress(1);
    pub const TA_MIRROR: TextureAddress = TextureAddress(2);
}
#[allow(non_camel_case_types)]
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
#[allow(non_camel_case_types)]
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
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ESceneDepthPriorityGroup(pub u8);
impl ESceneDepthPriorityGroup {
    pub const SDPG_WORLD: ESceneDepthPriorityGroup = ESceneDepthPriorityGroup(0);
    pub const SDPG_FOREGROUND: ESceneDepthPriorityGroup = ESceneDepthPriorityGroup(1);
}
#[allow(non_camel_case_types)]
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
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EEndPlayReason(pub u8);
impl EEndPlayReason {
    pub const DESTROYED: EEndPlayReason = EEndPlayReason(0);
    pub const LEVEL_TRANSITION: EEndPlayReason = EEndPlayReason(1);
    pub const END_PLAY_IN_EDITOR: EEndPlayReason = EEndPlayReason(2);
    pub const REMOVED_FROM_WORLD: EEndPlayReason = EEndPlayReason(3);
    pub const QUIT: EEndPlayReason = EEndPlayReason(4);
}
#[allow(non_camel_case_types)]
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
#[allow(non_camel_case_types)]
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
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EEditorPropertyValueState(pub u8);
impl EEditorPropertyValueState {
    pub const DEFAULT: EEditorPropertyValueState = EEditorPropertyValueState(0);
    pub const OVERRIDDEN: EEditorPropertyValueState = EEditorPropertyValueState(1);
    pub const NOT_FOUND: EEditorPropertyValueState = EEditorPropertyValueState(2);
    pub const ACCESS_DENIED: EEditorPropertyValueState = EEditorPropertyValueState(3);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EMoveComponentAction(pub u8);
impl EMoveComponentAction {
    pub const MOVE: EMoveComponentAction = EMoveComponentAction(0);
    pub const STOP: EMoveComponentAction = EMoveComponentAction(1);
    pub const RETURN: EMoveComponentAction = EMoveComponentAction(2);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EQuitPreference(pub u8);
impl EQuitPreference {
    pub const QUIT: EQuitPreference = EQuitPreference(0);
    pub const BACKGROUND: EQuitPreference = EQuitPreference(1);
}
#[allow(non_camel_case_types)]
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
#[allow(non_camel_case_types)]
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
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EAttachmentRule(pub u8);
impl EAttachmentRule {
    pub const KEEP_RELATIVE: EAttachmentRule = EAttachmentRule(0);
    pub const KEEP_WORLD: EAttachmentRule = EAttachmentRule(1);
    pub const SNAP_TO_TARGET: EAttachmentRule = EAttachmentRule(2);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EDetachmentRule(pub u8);
impl EDetachmentRule {
    pub const KEEP_RELATIVE: EDetachmentRule = EDetachmentRule(0);
    pub const KEEP_WORLD: EDetachmentRule = EDetachmentRule(1);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EComponentPhysicsStateChange(pub u8);
impl EComponentPhysicsStateChange {
    pub const CREATED: EComponentPhysicsStateChange = EComponentPhysicsStateChange(0);
    pub const DESTROYED: EComponentPhysicsStateChange = EComponentPhysicsStateChange(1);
}
#[allow(non_camel_case_types)]
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
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EFirstPersonPrimitiveType(pub u8);
impl EFirstPersonPrimitiveType {
    pub const NONE: EFirstPersonPrimitiveType = EFirstPersonPrimitiveType(0);
    pub const FIRST_PERSON: EFirstPersonPrimitiveType = EFirstPersonPrimitiveType(1);
    pub const WORLD_SPACE_REPRESENTATION: EFirstPersonPrimitiveType = EFirstPersonPrimitiveType(
        2,
    );
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EMouseLockMode(pub u8);
impl EMouseLockMode {
    pub const DO_NOT_LOCK: EMouseLockMode = EMouseLockMode(0);
    pub const LOCK_ON_CAPTURE: EMouseLockMode = EMouseLockMode(1);
    pub const LOCK_ALWAYS: EMouseLockMode = EMouseLockMode(2);
    pub const LOCK_IN_FULLSCREEN: EMouseLockMode = EMouseLockMode(3);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EWindowTitleBarMode(pub u8);
impl EWindowTitleBarMode {
    pub const OVERLAY: EWindowTitleBarMode = EWindowTitleBarMode(0);
    pub const VERTICAL_BOX: EWindowTitleBarMode = EWindowTitleBarMode(1);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ENetRole(pub u8);
impl ENetRole {
    pub const ROLE_NONE: ENetRole = ENetRole(0);
    pub const ROLE_SIMULATED_PROXY: ENetRole = ENetRole(1);
    pub const ROLE_AUTONOMOUS_PROXY: ENetRole = ENetRole(2);
    pub const ROLE_AUTHORITY: ENetRole = ENetRole(3);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ENetDormancy(pub u8);
impl ENetDormancy {
    pub const DORM_NEVER: ENetDormancy = ENetDormancy(0);
    pub const DORM_AWAKE: ENetDormancy = ENetDormancy(1);
    pub const DORM_DORMANT_ALL: ENetDormancy = ENetDormancy(2);
    pub const DORM_DORMANT_PARTIAL: ENetDormancy = ENetDormancy(3);
    pub const DORM_INITIAL: ENetDormancy = ENetDormancy(4);
}
#[allow(non_camel_case_types)]
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
#[allow(non_camel_case_types)]
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
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ENavigationOptionFlag(pub u8);
impl ENavigationOptionFlag {
    pub const DEFAULT: ENavigationOptionFlag = ENavigationOptionFlag(0);
    pub const ENABLE: ENavigationOptionFlag = ENavigationOptionFlag(1);
    pub const DISABLE: ENavigationOptionFlag = ENavigationOptionFlag(2);
    pub const MAX: ENavigationOptionFlag = ENavigationOptionFlag(3);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ENavigationQueryResult(pub u8);
impl ENavigationQueryResult {
    pub const INVALID: ENavigationQueryResult = ENavigationQueryResult(0);
    pub const ERROR: ENavigationQueryResult = ENavigationQueryResult(1);
    pub const FAIL: ENavigationQueryResult = ENavigationQueryResult(2);
    pub const SUCCESS: ENavigationQueryResult = ENavigationQueryResult(3);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ENavDataGatheringModeConfig(pub u8);
impl ENavDataGatheringModeConfig {
    pub const INVALID: ENavDataGatheringModeConfig = ENavDataGatheringModeConfig(0);
    pub const INSTANT: ENavDataGatheringModeConfig = ENavDataGatheringModeConfig(1);
    pub const LAZY: ENavDataGatheringModeConfig = ENavDataGatheringModeConfig(2);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EDataLayerRuntimeState(pub u8);
impl EDataLayerRuntimeState {
    pub const UNLOADED: EDataLayerRuntimeState = EDataLayerRuntimeState(0);
    pub const LOADED: EDataLayerRuntimeState = EDataLayerRuntimeState(1);
    pub const ACTIVATED: EDataLayerRuntimeState = EDataLayerRuntimeState(2);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ELightingBuildQuality(pub u8);
impl ELightingBuildQuality {
    pub const QUALITY_PREVIEW: ELightingBuildQuality = ELightingBuildQuality(0);
    pub const QUALITY_MEDIUM: ELightingBuildQuality = ELightingBuildQuality(1);
    pub const QUALITY_HIGH: ELightingBuildQuality = ELightingBuildQuality(2);
    pub const QUALITY_PRODUCTION: ELightingBuildQuality = ELightingBuildQuality(3);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EAnimCurveType(pub u8);
impl EAnimCurveType {
    pub const ATTRIBUTE_CURVE: EAnimCurveType = EAnimCurveType(0);
    pub const MATERIAL_CURVE: EAnimCurveType = EAnimCurveType(1);
    pub const MORPH_TARGET_CURVE: EAnimCurveType = EAnimCurveType(2);
    pub const MAX_ANIM_CURVE_TYPE: EAnimCurveType = EAnimCurveType(3);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EMontagePlayReturnType(pub u8);
impl EMontagePlayReturnType {
    pub const MONTAGE_LENGTH: EMontagePlayReturnType = EMontagePlayReturnType(0);
    pub const DURATION: EMontagePlayReturnType = EMontagePlayReturnType(1);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ETransitionRequestQueueMode(pub u8);
impl ETransitionRequestQueueMode {
    pub const SHARED: ETransitionRequestQueueMode = ETransitionRequestQueueMode(0);
    pub const UNIQUE: ETransitionRequestQueueMode = ETransitionRequestQueueMode(1);
}
#[allow(non_camel_case_types)]
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
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ETeleportType(pub u8);
impl ETeleportType {
    pub const NONE: ETeleportType = ETeleportType(0);
    pub const TELEPORT_PHYSICS: ETeleportType = ETeleportType(1);
    pub const RESET_PHYSICS: ETeleportType = ETeleportType(2);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ERootMotionMode(pub u8);
impl ERootMotionMode {
    pub const NO_ROOT_MOTION_EXTRACTION: ERootMotionMode = ERootMotionMode(0);
    pub const IGNORE_ROOT_MOTION: ERootMotionMode = ERootMotionMode(1);
    pub const ROOT_MOTION_FROM_EVERYTHING: ERootMotionMode = ERootMotionMode(2);
    pub const ROOT_MOTION_FROM_MONTAGES_ONLY: ERootMotionMode = ERootMotionMode(3);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EAdditiveBasePoseType(pub u8);
impl EAdditiveBasePoseType {
    pub const ABPT_NONE: EAdditiveBasePoseType = EAdditiveBasePoseType(0);
    pub const ABPT_REF_POSE: EAdditiveBasePoseType = EAdditiveBasePoseType(1);
    pub const ABPT_ANIM_SCALED: EAdditiveBasePoseType = EAdditiveBasePoseType(2);
    pub const ABPT_ANIM_FRAME: EAdditiveBasePoseType = EAdditiveBasePoseType(3);
    pub const ABPT_LOCAL_ANIM_FRAME: EAdditiveBasePoseType = EAdditiveBasePoseType(4);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ERootMotionRootLock(pub u8);
impl ERootMotionRootLock {
    pub const REF_POSE: ERootMotionRootLock = ERootMotionRootLock(0);
    pub const ANIM_FIRST_FRAME: ERootMotionRootLock = ERootMotionRootLock(1);
    pub const ZERO: ERootMotionRootLock = ERootMotionRootLock(2);
}
#[allow(non_camel_case_types)]
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
#[allow(non_camel_case_types)]
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
#[allow(non_camel_case_types)]
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
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EAudioSpectrumBandPresetType(pub u8);
impl EAudioSpectrumBandPresetType {
    pub const KICK_DRUM: EAudioSpectrumBandPresetType = EAudioSpectrumBandPresetType(0);
    pub const SNARE_DRUM: EAudioSpectrumBandPresetType = EAudioSpectrumBandPresetType(1);
    pub const VOICE: EAudioSpectrumBandPresetType = EAudioSpectrumBandPresetType(2);
    pub const CYMBALS: EAudioSpectrumBandPresetType = EAudioSpectrumBandPresetType(3);
}
#[allow(non_camel_case_types)]
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
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EFFTWindowType(pub u8);
impl EFFTWindowType {
    pub const NONE: EFFTWindowType = EFFTWindowType(0);
    pub const HAMMING: EFFTWindowType = EFFTWindowType(1);
    pub const HANN: EFFTWindowType = EFFTWindowType(2);
    pub const BLACKMAN: EFFTWindowType = EFFTWindowType(3);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EAudioSpectrumType(pub u8);
impl EAudioSpectrumType {
    pub const MAGNITUDE_SPECTRUM: EAudioSpectrumType = EAudioSpectrumType(0);
    pub const POWER_SPECTRUM: EAudioSpectrumType = EAudioSpectrumType(1);
    pub const DECIBEL: EAudioSpectrumType = EAudioSpectrumType(2);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EAudioRecordingExportType(pub u8);
impl EAudioRecordingExportType {
    pub const SOUND_WAVE: EAudioRecordingExportType = EAudioRecordingExportType(0);
    pub const WAV_FILE: EAudioRecordingExportType = EAudioRecordingExportType(1);
}
#[allow(non_camel_case_types)]
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
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EAudioFaderCurve(pub u8);
impl EAudioFaderCurve {
    pub const LINEAR: EAudioFaderCurve = EAudioFaderCurve(0);
    pub const LOGARITHMIC: EAudioFaderCurve = EAudioFaderCurve(1);
    pub const S_CURVE: EAudioFaderCurve = EAudioFaderCurve(2);
    pub const SIN: EAudioFaderCurve = EAudioFaderCurve(3);
    pub const COUNT: EAudioFaderCurve = EAudioFaderCurve(4);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EModulationDestination(pub u8);
impl EModulationDestination {
    pub const VOLUME: EModulationDestination = EModulationDestination(0);
    pub const PITCH: EModulationDestination = EModulationDestination(1);
    pub const LOWPASS: EModulationDestination = EModulationDestination(2);
    pub const HIGHPASS: EModulationDestination = EModulationDestination(3);
    pub const COUNT: EModulationDestination = EModulationDestination(4);
}
#[allow(non_camel_case_types)]
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
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ETextureStreamingMethod(pub u8);
impl ETextureStreamingMethod {
    pub const TSM_INVALID: ETextureStreamingMethod = ETextureStreamingMethod(0);
    pub const TSM_NOT_STREAMED: ETextureStreamingMethod = ETextureStreamingMethod(1);
    pub const TSM_STREAMED: ETextureStreamingMethod = ETextureStreamingMethod(2);
    pub const TSM_VIRTUAL_STREAMED: ETextureStreamingMethod = ETextureStreamingMethod(3);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ESkinWeightProfileLayer(pub u8);
impl ESkinWeightProfileLayer {
    pub const PRIMARY: ESkinWeightProfileLayer = ESkinWeightProfileLayer(0);
    pub const SECONDARY: ESkinWeightProfileLayer = ESkinWeightProfileLayer(1);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EPhysBodyOp(pub u8);
impl EPhysBodyOp {
    pub const PBO_NONE: EPhysBodyOp = EPhysBodyOp(0);
    pub const PBO_TERM: EPhysBodyOp = EPhysBodyOp(1);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EAnimationMode(pub u8);
impl EAnimationMode {
    pub const ANIMATION_BLUEPRINT: EAnimationMode = EAnimationMode(0);
    pub const ANIMATION_SINGLE_NODE: EAnimationMode = EAnimationMode(1);
    pub const ANIMATION_CUSTOM_MODE: EAnimationMode = EAnimationMode(2);
}
#[allow(non_camel_case_types)]
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
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ECastRayTracedShadow(pub u8);
impl ECastRayTracedShadow {
    pub const DISABLED: ECastRayTracedShadow = ECastRayTracedShadow(0);
    pub const USE_PROJECT_SETTING: ECastRayTracedShadow = ECastRayTracedShadow(1);
    pub const ENABLED: ECastRayTracedShadow = ECastRayTracedShadow(2);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ETwitterRequestMethod(pub u8);
impl ETwitterRequestMethod {
    pub const TRM_GET: ETwitterRequestMethod = ETwitterRequestMethod(0);
    pub const TRM_POST: ETwitterRequestMethod = ETwitterRequestMethod(1);
    pub const TRM_DELETE: ETwitterRequestMethod = ETwitterRequestMethod(2);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EWindSourceType(pub u8);
impl EWindSourceType {
    pub const DIRECTIONAL: EWindSourceType = EWindSourceType(0);
    pub const POINT: EWindSourceType = EWindSourceType(1);
}
#[allow(non_camel_case_types)]
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
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EStructUtilsResult(pub u8);
impl EStructUtilsResult {
    pub const VALID: EStructUtilsResult = EStructUtilsResult(0);
    pub const NOT_VALID: EStructUtilsResult = EStructUtilsResult(1);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EInterfaceValidResult(pub u8);
impl EInterfaceValidResult {
    pub const VALID: EInterfaceValidResult = EInterfaceValidResult(0);
    pub const INVALID: EInterfaceValidResult = EInterfaceValidResult(1);
}
#[allow(non_camel_case_types)]
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
#[allow(non_camel_case_types)]
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
#[allow(non_camel_case_types)]
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
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EControllerAnalogStick(pub u8);
impl EControllerAnalogStick {
    pub const CAS_LEFT_STICK: EControllerAnalogStick = EControllerAnalogStick(0);
    pub const CAS_RIGHT_STICK: EControllerAnalogStick = EControllerAnalogStick(1);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ELightUnits(pub u8);
impl ELightUnits {
    pub const UNITLESS: ELightUnits = ELightUnits(0);
    pub const CANDELAS: ELightUnits = ELightUnits(1);
    pub const LUMENS: ELightUnits = ELightUnits(2);
    pub const EV: ELightUnits = ELightUnits(3);
    pub const NITS: ELightUnits = ELightUnits(4);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EBoneSpaces(pub u8);
impl EBoneSpaces {
    pub const WORLD_SPACE: EBoneSpaces = EBoneSpaces(0);
    pub const COMPONENT_SPACE: EBoneSpaces = EBoneSpaces(1);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ESplineCoordinateSpace(pub u8);
impl ESplineCoordinateSpace {
    pub const LOCAL: ESplineCoordinateSpace = ESplineCoordinateSpace(0);
    pub const WORLD: ESplineCoordinateSpace = ESplineCoordinateSpace(1);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EHorizTextAligment(pub u8);
impl EHorizTextAligment {
    pub const EHTA_LEFT: EHorizTextAligment = EHorizTextAligment(0);
    pub const EHTA_CENTER: EHorizTextAligment = EHorizTextAligment(1);
    pub const EHTA_RIGHT: EHorizTextAligment = EHorizTextAligment(2);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EVerticalTextAligment(pub u8);
impl EVerticalTextAligment {
    pub const EVRTA_TEXT_TOP: EVerticalTextAligment = EVerticalTextAligment(0);
    pub const EVRTA_TEXT_CENTER: EVerticalTextAligment = EVerticalTextAligment(1);
    pub const EVRTA_TEXT_BOTTOM: EVerticalTextAligment = EVerticalTextAligment(2);
    pub const EVRTA_QUAD_TOP: EVerticalTextAligment = EVerticalTextAligment(3);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EEvaluateCurveTableResult(pub u8);
impl EEvaluateCurveTableResult {
    pub const ROW_FOUND: EEvaluateCurveTableResult = EEvaluateCurveTableResult(0);
    pub const ROW_NOT_FOUND: EEvaluateCurveTableResult = EEvaluateCurveTableResult(1);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ETravelType(pub u8);
impl ETravelType {
    pub const TRAVEL_ABSOLUTE: ETravelType = ETravelType(0);
    pub const TRAVEL_PARTIAL: ETravelType = ETravelType(1);
    pub const TRAVEL_RELATIVE: ETravelType = ETravelType(2);
}
#[allow(non_camel_case_types)]
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
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EDynamicForceFeedbackAction(pub u8);
impl EDynamicForceFeedbackAction {
    pub const START: EDynamicForceFeedbackAction = EDynamicForceFeedbackAction(0);
    pub const UPDATE: EDynamicForceFeedbackAction = EDynamicForceFeedbackAction(1);
    pub const STOP: EDynamicForceFeedbackAction = EDynamicForceFeedbackAction(2);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ETypedElementWorldType(pub u8);
impl ETypedElementWorldType {
    pub const GAME: ETypedElementWorldType = ETypedElementWorldType(0);
    pub const EDITOR: ETypedElementWorldType = ETypedElementWorldType(1);
}
#[allow(non_camel_case_types)]
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
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ESpawnActorScaleMethod(pub u8);
impl ESpawnActorScaleMethod {
    pub const OVERRIDE_ROOT_SCALE: ESpawnActorScaleMethod = ESpawnActorScaleMethod(0);
    pub const MULTIPLY_WITH_ROOT: ESpawnActorScaleMethod = ESpawnActorScaleMethod(1);
    pub const SELECT_DEFAULT_AT_RUNTIME: ESpawnActorScaleMethod = ESpawnActorScaleMethod(
        2,
    );
}
#[allow(non_camel_case_types)]
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
#[allow(non_camel_case_types)]
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
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EWindowMode(pub u8);
impl EWindowMode {
    pub const FULLSCREEN: EWindowMode = EWindowMode(0);
    pub const WINDOWED_FULLSCREEN: EWindowMode = EWindowMode(1);
    pub const WINDOWED: EWindowMode = EWindowMode(2);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EArraySortOrder(pub u8);
impl EArraySortOrder {
    pub const ASCENDING: EArraySortOrder = EArraySortOrder(0);
    pub const DESCENDING: EArraySortOrder = EArraySortOrder(1);
}
#[allow(non_camel_case_types)]
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
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EMIDCreationFlags(pub u8);
impl EMIDCreationFlags {
    pub const NONE: EMIDCreationFlags = EMIDCreationFlags(0);
    pub const TRANSIENT: EMIDCreationFlags = EMIDCreationFlags(1);
}
#[allow(non_camel_case_types)]
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
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EMatrixColumns(pub u8);
impl EMatrixColumns {
    pub const FIRST: EMatrixColumns = EMatrixColumns(0);
    pub const SECOND: EMatrixColumns = EMatrixColumns(1);
    pub const THIRD: EMatrixColumns = EMatrixColumns(2);
    pub const FOURTH: EMatrixColumns = EMatrixColumns(3);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ELerpInterpolationMode(pub u8);
impl ELerpInterpolationMode {
    pub const QUAT_INTERP: ELerpInterpolationMode = ELerpInterpolationMode(0);
    pub const EULER_INTERP: ELerpInterpolationMode = ELerpInterpolationMode(1);
    pub const DUAL_QUAT_INTERP: ELerpInterpolationMode = ELerpInterpolationMode(2);
}
#[allow(non_camel_case_types)]
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
#[allow(non_camel_case_types)]
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
#[allow(non_camel_case_types)]
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
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EMemoryUnitStandard(pub u8);
impl EMemoryUnitStandard {
    pub const IEC: EMemoryUnitStandard = EMemoryUnitStandard(0);
    pub const SI: EMemoryUnitStandard = EMemoryUnitStandard(1);
}
#[allow(non_camel_case_types)]
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
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EVertexPaintAxis(pub u8);
impl EVertexPaintAxis {
    pub const X: EVertexPaintAxis = EVertexPaintAxis(0);
    pub const Y: EVertexPaintAxis = EVertexPaintAxis(1);
    pub const Z: EVertexPaintAxis = EVertexPaintAxis(2);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ETrailWidthMode(pub u8);
impl ETrailWidthMode {
    pub const E_TRAIL_WIDTH_MODE_FROM_CENTRE: ETrailWidthMode = ETrailWidthMode(0);
    pub const E_TRAIL_WIDTH_MODE_FROM_FIRST: ETrailWidthMode = ETrailWidthMode(1);
    pub const E_TRAIL_WIDTH_MODE_FROM_SECOND: ETrailWidthMode = ETrailWidthMode(2);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EDataLayerState(pub u8);
impl EDataLayerState {
    pub const UNLOADED: EDataLayerState = EDataLayerState(0);
    pub const LOADED: EDataLayerState = EDataLayerState(1);
    pub const ACTIVATED: EDataLayerState = EDataLayerState(2);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EDataLayerType(pub u8);
impl EDataLayerType {
    pub const RUNTIME: EDataLayerType = EDataLayerType(0);
    pub const EDITOR: EDataLayerType = EDataLayerType(1);
    pub const UNKNOWN: EDataLayerType = EDataLayerType(2);
    pub const SIZE: EDataLayerType = EDataLayerType(3);
}
#[allow(non_camel_case_types)]
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
#[allow(non_camel_case_types)]
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
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ERefractionMode(pub u8);
impl ERefractionMode {
    pub const RM_INDEX_OF_REFRACTION: ERefractionMode = ERefractionMode(0);
    pub const RM_PIXEL_NORMAL_OFFSET: ERefractionMode = ERefractionMode(1);
    pub const RM_2D_OFFSET: ERefractionMode = ERefractionMode(2);
    pub const RM_NONE: ERefractionMode = ERefractionMode(3);
    pub const RM_INDEX_OF_REFRACTION_FROM_F0: ERefractionMode = ERefractionMode(4);
}
#[allow(non_camel_case_types)]
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
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ENodeAdvancedPins(pub u8);
impl ENodeAdvancedPins {
    pub const NO_PINS: ENodeAdvancedPins = ENodeAdvancedPins(0);
    pub const SHOWN: ENodeAdvancedPins = ENodeAdvancedPins(1);
    pub const HIDDEN: ENodeAdvancedPins = ENodeAdvancedPins(2);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ENodeEnabledState(pub u8);
impl ENodeEnabledState {
    pub const ENABLED: ENodeEnabledState = ENodeEnabledState(0);
    pub const DISABLED: ENodeEnabledState = ENodeEnabledState(1);
    pub const DEVELOPMENT_ONLY: ENodeEnabledState = ENodeEnabledState(2);
}
#[allow(non_camel_case_types)]
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
#[allow(non_camel_case_types)]
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
#[allow(non_camel_case_types)]
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
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EActorGridPlacement(pub u8);
impl EActorGridPlacement {
    pub const BOUNDS: EActorGridPlacement = EActorGridPlacement(0);
    pub const LOCATION: EActorGridPlacement = EActorGridPlacement(1);
    pub const ALWAYS_LOADED: EActorGridPlacement = EActorGridPlacement(2);
    pub const NONE: EActorGridPlacement = EActorGridPlacement(3);
}
#[allow(non_camel_case_types)]
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
#[allow(non_camel_case_types)]
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
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EHitProxyPriority(pub u8);
impl EHitProxyPriority {
    pub const HPP_WORLD: EHitProxyPriority = EHitProxyPriority(0);
    pub const HPP_WIREFRAME: EHitProxyPriority = EHitProxyPriority(1);
    pub const HPP_FOREGROUND: EHitProxyPriority = EHitProxyPriority(2);
    pub const HPP_UI: EHitProxyPriority = EHitProxyPriority(3);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ECanBeCharacterBase(pub u8);
impl ECanBeCharacterBase {
    pub const ECB_NO: ECanBeCharacterBase = ECanBeCharacterBase(0);
    pub const ECB_YES: ECanBeCharacterBase = ECanBeCharacterBase(1);
    pub const ECB_OWNER: ECanBeCharacterBase = ECanBeCharacterBase(2);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EAutoPossessAI(pub u8);
impl EAutoPossessAI {
    pub const DISABLED: EAutoPossessAI = EAutoPossessAI(0);
    pub const PLACED_IN_WORLD: EAutoPossessAI = EAutoPossessAI(1);
    pub const SPAWNED: EAutoPossessAI = EAutoPossessAI(2);
    pub const PLACED_IN_WORLD_OR_SPAWNED: EAutoPossessAI = EAutoPossessAI(3);
}
#[allow(non_camel_case_types)]
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
#[allow(non_camel_case_types)]
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
#[allow(non_camel_case_types)]
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
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EBrushType(pub u8);
impl EBrushType {
    pub const BRUSH_DEFAULT: EBrushType = EBrushType(0);
    pub const BRUSH_ADD: EBrushType = EBrushType(1);
    pub const BRUSH_SUBTRACT: EBrushType = EBrushType(2);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ENavigationDataResolution(pub u8);
impl ENavigationDataResolution {
    pub const LOW: ENavigationDataResolution = ENavigationDataResolution(0);
    pub const DEFAULT: ENavigationDataResolution = ENavigationDataResolution(1);
    pub const HIGH: ENavigationDataResolution = ENavigationDataResolution(2);
    pub const INVALID: ENavigationDataResolution = ENavigationDataResolution(3);
    pub const MAX: ENavigationDataResolution = ENavigationDataResolution(3);
}
#[allow(non_camel_case_types)]
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
#[allow(non_camel_case_types)]
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
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EBlueprintCompileMode(pub u8);
impl EBlueprintCompileMode {
    pub const DEFAULT: EBlueprintCompileMode = EBlueprintCompileMode(0);
    pub const DEVELOPMENT: EBlueprintCompileMode = EBlueprintCompileMode(1);
    pub const FINAL_RELEASE: EBlueprintCompileMode = EBlueprintCompileMode(2);
}
#[allow(non_camel_case_types)]
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
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EBlueprintPinStyleType(pub u8);
impl EBlueprintPinStyleType {
    pub const BPST_ORIGINAL: EBlueprintPinStyleType = EBlueprintPinStyleType(0);
    pub const BPST_VARIANT_A: EBlueprintPinStyleType = EBlueprintPinStyleType(1);
}
#[allow(non_camel_case_types)]
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
#[allow(non_camel_case_types)]
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
#[allow(non_camel_case_types)]
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
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ETransitionBlendMode(pub u8);
impl ETransitionBlendMode {
    pub const TBM_LINEAR: ETransitionBlendMode = ETransitionBlendMode(0);
    pub const TBM_CUBIC: ETransitionBlendMode = ETransitionBlendMode(1);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EVirtualizationMode(pub u8);
impl EVirtualizationMode {
    pub const DISABLED: EVirtualizationMode = EVirtualizationMode(0);
    pub const PLAY_WHEN_SILENT: EVirtualizationMode = EVirtualizationMode(1);
    pub const RESTART: EVirtualizationMode = EVirtualizationMode(2);
    pub const SEEK_RESTART: EVirtualizationMode = EVirtualizationMode(3);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ESoundWaveFFTSize(pub u8);
impl ESoundWaveFFTSize {
    pub const VERY_SMALL_64: ESoundWaveFFTSize = ESoundWaveFFTSize(0);
    pub const SMALL_256: ESoundWaveFFTSize = ESoundWaveFFTSize(1);
    pub const MEDIUM_512: ESoundWaveFFTSize = ESoundWaveFFTSize(2);
    pub const LARGE_1024: ESoundWaveFFTSize = ESoundWaveFFTSize(3);
    pub const VERY_LARGE_2048: ESoundWaveFFTSize = ESoundWaveFFTSize(4);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ESoundWaveCuePointOrigin(pub u8);
impl ESoundWaveCuePointOrigin {
    pub const WAVE_FILE: ESoundWaveCuePointOrigin = ESoundWaveCuePointOrigin(0);
    pub const MARKER_TRANSFORMATION: ESoundWaveCuePointOrigin = ESoundWaveCuePointOrigin(
        1,
    );
}
#[allow(non_camel_case_types)]
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
#[allow(non_camel_case_types)]
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
#[allow(non_camel_case_types)]
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
#[allow(non_camel_case_types)]
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
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ETextureAvailability(pub u8);
impl ETextureAvailability {
    pub const GPU: ETextureAvailability = ETextureAvailability(0);
    pub const CPU: ETextureAvailability = ETextureAvailability(1);
}
#[allow(non_camel_case_types)]
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
#[allow(non_camel_case_types)]
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
#[allow(non_camel_case_types)]
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
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EMaterialTranslucencyPass(pub u8);
impl EMaterialTranslucencyPass {
    pub const MTP_BEFORE_DOF: EMaterialTranslucencyPass = EMaterialTranslucencyPass(0);
    pub const MTP_AFTER_DOF: EMaterialTranslucencyPass = EMaterialTranslucencyPass(1);
    pub const MTP_AFTER_MOTION_BLUR: EMaterialTranslucencyPass = EMaterialTranslucencyPass(
        2,
    );
}
#[allow(non_camel_case_types)]
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
#[allow(non_camel_case_types)]
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
#[allow(non_camel_case_types)]
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
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ERefractionCoverageMode(pub u8);
impl ERefractionCoverageMode {
    pub const RCM_COVERAGE_IGNORED: ERefractionCoverageMode = ERefractionCoverageMode(0);
    pub const RCM_COVERAGE_ACCOUNTED_FOR: ERefractionCoverageMode = ERefractionCoverageMode(
        1,
    );
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EPixelDepthOffsetMode(pub u8);
impl EPixelDepthOffsetMode {
    pub const PDOM_LEGACY: EPixelDepthOffsetMode = EPixelDepthOffsetMode(0);
    pub const PDOM_ALONG_CAMERA_VECTOR: EPixelDepthOffsetMode = EPixelDepthOffsetMode(1);
}
#[allow(non_camel_case_types)]
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
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EScreenPercentageMode(pub i32);
impl EScreenPercentageMode {
    pub const MANUAL: EScreenPercentageMode = EScreenPercentageMode(0);
    pub const BASED_ON_DISPLAY_RESOLUTION: EScreenPercentageMode = EScreenPercentageMode(
        1,
    );
    pub const BASED_ON_DPI_SCALE: EScreenPercentageMode = EScreenPercentageMode(2);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ENavDataGatheringMode(pub u8);
impl ENavDataGatheringMode {
    pub const DEFAULT: ENavDataGatheringMode = ENavDataGatheringMode(0);
    pub const INSTANT: ENavDataGatheringMode = ENavDataGatheringMode(1);
    pub const LAZY: ENavDataGatheringMode = ENavDataGatheringMode(2);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EActorPackagingScheme(pub u8);
impl EActorPackagingScheme {
    pub const ORIGINAL: EActorPackagingScheme = EActorPackagingScheme(0);
    pub const REDUCED: EActorPackagingScheme = EActorPackagingScheme(1);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EReflectionSourceType(pub u8);
impl EReflectionSourceType {
    pub const CAPTURED_SCENE: EReflectionSourceType = EReflectionSourceType(0);
    pub const SPECIFIED_CUBEMAP: EReflectionSourceType = EReflectionSourceType(1);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct DistributionParamMode(pub u8);
impl DistributionParamMode {
    pub const DPM_NORMAL: DistributionParamMode = DistributionParamMode(0);
    pub const DPM_ABS: DistributionParamMode = DistributionParamMode(1);
    pub const DPM_DIRECT: DistributionParamMode = DistributionParamMode(2);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EDistributionVectorLockFlags(pub u8);
impl EDistributionVectorLockFlags {
    pub const EDVLF_NONE: EDistributionVectorLockFlags = EDistributionVectorLockFlags(0);
    pub const EDVLF_XY: EDistributionVectorLockFlags = EDistributionVectorLockFlags(1);
    pub const EDVLF_XZ: EDistributionVectorLockFlags = EDistributionVectorLockFlags(2);
    pub const EDVLF_YZ: EDistributionVectorLockFlags = EDistributionVectorLockFlags(3);
    pub const EDVLF_XYZ: EDistributionVectorLockFlags = EDistributionVectorLockFlags(4);
}
#[allow(non_camel_case_types)]
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
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EPositionOrigin(pub u8);
impl EPositionOrigin {
    pub const ABSOLUTE: EPositionOrigin = EPositionOrigin(0);
    pub const CAMERA_RELATIVE: EPositionOrigin = EPositionOrigin(1);
}
#[allow(non_camel_case_types)]
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
#[allow(non_camel_case_types)]
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
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ETextureMipValueMode(pub u8);
impl ETextureMipValueMode {
    pub const TMVM_NONE: ETextureMipValueMode = ETextureMipValueMode(0);
    pub const TMVM_MIP_LEVEL: ETextureMipValueMode = ETextureMipValueMode(1);
    pub const TMVM_MIP_BIAS: ETextureMipValueMode = ETextureMipValueMode(2);
    pub const TMVM_DERIVATIVE: ETextureMipValueMode = ETextureMipValueMode(3);
}
#[allow(non_camel_case_types)]
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
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ETextureGatherMode(pub u8);
impl ETextureGatherMode {
    pub const TGM_NONE: ETextureGatherMode = ETextureGatherMode(0);
    pub const TGM_RED: ETextureGatherMode = ETextureGatherMode(1);
    pub const TGM_GREEN: ETextureGatherMode = ETextureGatherMode(2);
    pub const TGM_BLUE: ETextureGatherMode = ETextureGatherMode(3);
    pub const TGM_ALPHA: ETextureGatherMode = ETextureGatherMode(4);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ETextureColorChannel(pub u8);
impl ETextureColorChannel {
    pub const TCC_RED: ETextureColorChannel = ETextureColorChannel(0);
    pub const TCC_GREEN: ETextureColorChannel = ETextureColorChannel(1);
    pub const TCC_BLUE: ETextureColorChannel = ETextureColorChannel(2);
    pub const TCC_ALPHA: ETextureColorChannel = ETextureColorChannel(3);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EMaterialExpressionBlendMode(pub u8);
impl EMaterialExpressionBlendMode {
    pub const BLEND: EMaterialExpressionBlendMode = EMaterialExpressionBlendMode(0);
    pub const USE_A: EMaterialExpressionBlendMode = EMaterialExpressionBlendMode(1);
    pub const USE_B: EMaterialExpressionBlendMode = EMaterialExpressionBlendMode(2);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EMaterialAttributeBlend(pub u8);
impl EMaterialAttributeBlend {
    pub const BLEND: EMaterialAttributeBlend = EMaterialAttributeBlend(0);
    pub const USE_A: EMaterialAttributeBlend = EMaterialAttributeBlend(1);
    pub const USE_B: EMaterialAttributeBlend = EMaterialAttributeBlend(2);
}
#[allow(non_camel_case_types)]
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
#[allow(non_camel_case_types)]
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
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EChannelMaskParameterColor(pub u8);
impl EChannelMaskParameterColor {
    pub const RED: EChannelMaskParameterColor = EChannelMaskParameterColor(0);
    pub const GREEN: EChannelMaskParameterColor = EChannelMaskParameterColor(1);
    pub const BLUE: EChannelMaskParameterColor = EChannelMaskParameterColor(2);
    pub const ALPHA: EChannelMaskParameterColor = EChannelMaskParameterColor(3);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EClampMode(pub u8);
impl EClampMode {
    pub const CMODE_CLAMP: EClampMode = EClampMode(0);
    pub const CMODE_CLAMP_MIN: EClampMode = EClampMode(1);
    pub const CMODE_CLAMP_MAX: EClampMode = EClampMode(2);
}
#[allow(non_camel_case_types)]
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
#[allow(non_camel_case_types)]
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
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EDBufferTextureId(pub u8);
impl EDBufferTextureId {
    pub const DBT_A: EDBufferTextureId = EDBufferTextureId(0);
    pub const DBT_B: EDBufferTextureId = EDBufferTextureId(1);
    pub const DBT_C: EDBufferTextureId = EDBufferTextureId(2);
}
#[allow(non_camel_case_types)]
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
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EFloatToIntMode(pub u8);
impl EFloatToIntMode {
    pub const TRUNCATE: EFloatToIntMode = EFloatToIntMode(0);
    pub const FLOOR: EFloatToIntMode = EFloatToIntMode(1);
    pub const ROUND: EFloatToIntMode = EFloatToIntMode(2);
    pub const CEIL: EFloatToIntMode = EFloatToIntMode(3);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EBlendInputRelevance(pub u8);
impl EBlendInputRelevance {
    pub const GENERAL: EBlendInputRelevance = EBlendInputRelevance(0);
    pub const TOP: EBlendInputRelevance = EBlendInputRelevance(1);
    pub const BOTTOM: EBlendInputRelevance = EBlendInputRelevance(2);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EPositionIncludedOffsets(pub i32);
impl EPositionIncludedOffsets {
    pub const INCLUDE_OFFSETS: EPositionIncludedOffsets = EPositionIncludedOffsets(0);
    pub const EXCLUDE_OFFSETS: EPositionIncludedOffsets = EPositionIncludedOffsets(1);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ELocalPositionOrigin(pub i32);
impl ELocalPositionOrigin {
    pub const INSTANCE: ELocalPositionOrigin = ELocalPositionOrigin(0);
    pub const INSTANCE_PRE_SKINNING: ELocalPositionOrigin = ELocalPositionOrigin(1);
    pub const PRIMITIVE: ELocalPositionOrigin = ELocalPositionOrigin(2);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ENeuralIndexType(pub u8);
impl ENeuralIndexType {
    pub const NIT_TEXTURE_INDEX: ENeuralIndexType = ENeuralIndexType(0);
    pub const NIT_BUFFER_INDEX: ENeuralIndexType = ENeuralIndexType(1);
}
#[allow(non_camel_case_types)]
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
#[allow(non_camel_case_types)]
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
#[allow(non_camel_case_types)]
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
#[allow(non_camel_case_types)]
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
#[allow(non_camel_case_types)]
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
#[allow(non_camel_case_types)]
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
#[allow(non_camel_case_types)]
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
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ESpeedTreeGeometryType(pub u8);
impl ESpeedTreeGeometryType {
    pub const STG_BRANCH: ESpeedTreeGeometryType = ESpeedTreeGeometryType(0);
    pub const STG_FROND: ESpeedTreeGeometryType = ESpeedTreeGeometryType(1);
    pub const STG_LEAF: ESpeedTreeGeometryType = ESpeedTreeGeometryType(2);
    pub const STG_FACING_LEAF: ESpeedTreeGeometryType = ESpeedTreeGeometryType(3);
    pub const STG_BILLBOARD: ESpeedTreeGeometryType = ESpeedTreeGeometryType(4);
}
#[allow(non_camel_case_types)]
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
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ESpeedTreeLODType(pub u8);
impl ESpeedTreeLODType {
    pub const STLOD_POP: ESpeedTreeLODType = ESpeedTreeLODType(0);
    pub const STLOD_SMOOTH: ESpeedTreeLODType = ESpeedTreeLODType(1);
}
#[allow(non_camel_case_types)]
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
#[allow(non_camel_case_types)]
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
#[allow(non_camel_case_types)]
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
#[allow(non_camel_case_types)]
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
#[allow(non_camel_case_types)]
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
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EVectorNoiseFunction(pub u8);
impl EVectorNoiseFunction {
    pub const VNF_CELLNOISE_ALU: EVectorNoiseFunction = EVectorNoiseFunction(0);
    pub const VNF_VECTOR_ALU: EVectorNoiseFunction = EVectorNoiseFunction(1);
    pub const VNF_GRADIENT_ALU: EVectorNoiseFunction = EVectorNoiseFunction(2);
    pub const VNF_CURL_ALU: EVectorNoiseFunction = EVectorNoiseFunction(3);
    pub const VNF_VORONOI_ALU: EVectorNoiseFunction = EVectorNoiseFunction(4);
}
#[allow(non_camel_case_types)]
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
#[allow(non_camel_case_types)]
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
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EMaterialFunctionUsage(pub u8);
impl EMaterialFunctionUsage {
    pub const DEFAULT: EMaterialFunctionUsage = EMaterialFunctionUsage(0);
    pub const MATERIAL_LAYER: EMaterialFunctionUsage = EMaterialFunctionUsage(1);
    pub const MATERIAL_LAYER_BLEND: EMaterialFunctionUsage = EMaterialFunctionUsage(2);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EParticleSystemUpdateMode(pub u8);
impl EParticleSystemUpdateMode {
    pub const EPSUM_REAL_TIME: EParticleSystemUpdateMode = EParticleSystemUpdateMode(0);
    pub const EPSUM_FIXED_TIME: EParticleSystemUpdateMode = EParticleSystemUpdateMode(1);
}
#[allow(non_camel_case_types)]
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
#[allow(non_camel_case_types)]
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
#[allow(non_camel_case_types)]
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
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EParticleSignificanceLevel(pub u8);
impl EParticleSignificanceLevel {
    pub const LOW: EParticleSignificanceLevel = EParticleSignificanceLevel(0);
    pub const MEDIUM: EParticleSignificanceLevel = EParticleSignificanceLevel(1);
    pub const HIGH: EParticleSignificanceLevel = EParticleSignificanceLevel(2);
    pub const CRITICAL: EParticleSignificanceLevel = EParticleSignificanceLevel(3);
    pub const NUM: EParticleSignificanceLevel = EParticleSignificanceLevel(4);
}
#[allow(non_camel_case_types)]
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
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct BeamModifierType(pub u8);
impl BeamModifierType {
    pub const PEB2MT_SOURCE: BeamModifierType = BeamModifierType(0);
    pub const PEB2MT_TARGET: BeamModifierType = BeamModifierType(1);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct Beam2SourceTargetMethod(pub u8);
impl Beam2SourceTargetMethod {
    pub const PEB2STM_DEFAULT: Beam2SourceTargetMethod = Beam2SourceTargetMethod(0);
    pub const PEB2STM_USER_SET: Beam2SourceTargetMethod = Beam2SourceTargetMethod(1);
    pub const PEB2STM_EMITTER: Beam2SourceTargetMethod = Beam2SourceTargetMethod(2);
    pub const PEB2STM_PARTICLE: Beam2SourceTargetMethod = Beam2SourceTargetMethod(3);
    pub const PEB2STM_ACTOR: Beam2SourceTargetMethod = Beam2SourceTargetMethod(4);
}
#[allow(non_camel_case_types)]
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
#[allow(non_camel_case_types)]
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
#[allow(non_camel_case_types)]
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
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EParticleCollisionResponse(pub u8);
impl EParticleCollisionResponse {
    pub const BOUNCE: EParticleCollisionResponse = EParticleCollisionResponse(0);
    pub const STOP: EParticleCollisionResponse = EParticleCollisionResponse(1);
    pub const KILL: EParticleCollisionResponse = EParticleCollisionResponse(2);
}
#[allow(non_camel_case_types)]
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
#[allow(non_camel_case_types)]
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
#[allow(non_camel_case_types)]
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
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct CylinderHeightAxis(pub u8);
impl CylinderHeightAxis {
    pub const PMLPC_HEIGHTAXIS_X: CylinderHeightAxis = CylinderHeightAxis(0);
    pub const PMLPC_HEIGHTAXIS_Y: CylinderHeightAxis = CylinderHeightAxis(1);
    pub const PMLPC_HEIGHTAXIS_Z: CylinderHeightAxis = CylinderHeightAxis(2);
}
#[allow(non_camel_case_types)]
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
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EOrbitChainMode(pub u8);
impl EOrbitChainMode {
    pub const EO_CHAIN_MODE_ADD: EOrbitChainMode = EOrbitChainMode(0);
    pub const EO_CHAIN_MODE_SCALE: EOrbitChainMode = EOrbitChainMode(1);
    pub const EO_CHAIN_MODE_LINK: EOrbitChainMode = EOrbitChainMode(2);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EParticleSortMode(pub u8);
impl EParticleSortMode {
    pub const PSORTMODE_NONE: EParticleSortMode = EParticleSortMode(0);
    pub const PSORTMODE_VIEW_PROJ_DEPTH: EParticleSortMode = EParticleSortMode(1);
    pub const PSORTMODE_DISTANCE_TO_VIEW: EParticleSortMode = EParticleSortMode(2);
    pub const PSORTMODE_AGE_OLDEST_FIRST: EParticleSortMode = EParticleSortMode(3);
    pub const PSORTMODE_AGE_NEWEST_FIRST: EParticleSortMode = EParticleSortMode(4);
}
#[allow(non_camel_case_types)]
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
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EParticleBurstMethod(pub u8);
impl EParticleBurstMethod {
    pub const EPBM_INSTANT: EParticleBurstMethod = EParticleBurstMethod(0);
    pub const EPBM_INTERPOLATED: EParticleBurstMethod = EParticleBurstMethod(1);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EOpacitySourceMode(pub u8);
impl EOpacitySourceMode {
    pub const OSM_ALPHA: EOpacitySourceMode = EOpacitySourceMode(0);
    pub const OSM_COLOR_BRIGHTNESS: EOpacitySourceMode = EOpacitySourceMode(1);
    pub const OSM_RED_CHANNEL: EOpacitySourceMode = EOpacitySourceMode(2);
    pub const OSM_GREEN_CHANNEL: EOpacitySourceMode = EOpacitySourceMode(3);
    pub const OSM_BLUE_CHANNEL: EOpacitySourceMode = EOpacitySourceMode(4);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EEmitterNormalsMode(pub u8);
impl EEmitterNormalsMode {
    pub const ENM_CAMERA_FACING: EEmitterNormalsMode = EEmitterNormalsMode(0);
    pub const ENM_SPHERICAL: EEmitterNormalsMode = EEmitterNormalsMode(1);
    pub const ENM_CYLINDRICAL: EEmitterNormalsMode = EEmitterNormalsMode(2);
}
#[allow(non_camel_case_types)]
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
#[allow(non_camel_case_types)]
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
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ETrail2SourceMethod(pub u8);
impl ETrail2SourceMethod {
    pub const PET2SRCM_DEFAULT: ETrail2SourceMethod = ETrail2SourceMethod(0);
    pub const PET2SRCM_PARTICLE: ETrail2SourceMethod = ETrail2SourceMethod(1);
    pub const PET2SRCM_ACTOR: ETrail2SourceMethod = ETrail2SourceMethod(2);
}
#[allow(non_camel_case_types)]
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
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EBeam2Method(pub u8);
impl EBeam2Method {
    pub const PEB2M_DISTANCE: EBeam2Method = EBeam2Method(0);
    pub const PEB2M_TARGET: EBeam2Method = EBeam2Method(1);
    pub const PEB2M_BRANCH: EBeam2Method = EBeam2Method(2);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EBeamTaperMethod(pub u8);
impl EBeamTaperMethod {
    pub const PEBTM_NONE: EBeamTaperMethod = EBeamTaperMethod(0);
    pub const PEBTM_FULL: EBeamTaperMethod = EBeamTaperMethod(1);
    pub const PEBTM_PARTIAL: EBeamTaperMethod = EBeamTaperMethod(2);
}
#[allow(non_camel_case_types)]
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
#[allow(non_camel_case_types)]
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
#[allow(non_camel_case_types)]
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
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ETrailsRenderAxisOption(pub u8);
impl ETrailsRenderAxisOption {
    pub const TRAILS_CAMERA_UP: ETrailsRenderAxisOption = ETrailsRenderAxisOption(0);
    pub const TRAILS_SOURCE_UP: ETrailsRenderAxisOption = ETrailsRenderAxisOption(1);
    pub const TRAILS_WORLD_UP: ETrailsRenderAxisOption = ETrailsRenderAxisOption(2);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EEmitterRenderMode(pub u8);
impl EEmitterRenderMode {
    pub const ERM_NORMAL: EEmitterRenderMode = EEmitterRenderMode(0);
    pub const ERM_POINT: EEmitterRenderMode = EEmitterRenderMode(1);
    pub const ERM_CROSS: EEmitterRenderMode = EEmitterRenderMode(2);
    pub const ERM_LIGHTS_ONLY: EEmitterRenderMode = EEmitterRenderMode(3);
    pub const ERM_NONE: EEmitterRenderMode = EEmitterRenderMode(4);
}
#[allow(non_camel_case_types)]
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
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ESceneCaptureCompositeMode(pub u8);
impl ESceneCaptureCompositeMode {
    pub const SCCM_OVERWRITE: ESceneCaptureCompositeMode = ESceneCaptureCompositeMode(0);
    pub const SCCM_ADDITIVE: ESceneCaptureCompositeMode = ESceneCaptureCompositeMode(1);
    pub const SCCM_COMPOSITE: ESceneCaptureCompositeMode = ESceneCaptureCompositeMode(2);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ESceneCaptureUnlitViewmode(pub u8);
impl ESceneCaptureUnlitViewmode {
    pub const DISABLED: ESceneCaptureUnlitViewmode = ESceneCaptureUnlitViewmode(0);
    pub const CAPTURE: ESceneCaptureUnlitViewmode = ESceneCaptureUnlitViewmode(1);
    pub const CAPTURE_OR_CUSTOM_RENDER_PASS: ESceneCaptureUnlitViewmode = ESceneCaptureUnlitViewmode(
        2,
    );
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EShadowMapFlags(pub u8);
impl EShadowMapFlags {
    pub const SMF_NONE: EShadowMapFlags = EShadowMapFlags(0);
    pub const SMF_STREAMED: EShadowMapFlags = EShadowMapFlags(1);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ETextureEncodeEffort(pub u8);
impl ETextureEncodeEffort {
    pub const DEFAULT: ETextureEncodeEffort = ETextureEncodeEffort(0);
    pub const LOW: ETextureEncodeEffort = ETextureEncodeEffort(10);
    pub const NORMAL: ETextureEncodeEffort = ETextureEncodeEffort(20);
    pub const HIGH: ETextureEncodeEffort = ETextureEncodeEffort(30);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ETextureUniversalTiling(pub u8);
impl ETextureUniversalTiling {
    pub const DISABLED: ETextureUniversalTiling = ETextureUniversalTiling(0);
    pub const ENABLED_256KB: ETextureUniversalTiling = ETextureUniversalTiling(1);
    pub const ENABLED_64KB: ETextureUniversalTiling = ETextureUniversalTiling(2);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ETextureEncodeSpeed(pub u8);
impl ETextureEncodeSpeed {
    pub const FINAL: ETextureEncodeSpeed = ETextureEncodeSpeed(0);
    pub const FINAL_IF_AVAILABLE: ETextureEncodeSpeed = ETextureEncodeSpeed(1);
    pub const FAST: ETextureEncodeSpeed = ETextureEncodeSpeed(2);
}
#[allow(non_camel_case_types)]
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
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EVectorFieldConstructionOp(pub u8);
impl EVectorFieldConstructionOp {
    pub const VFCO_EXTRUDE: EVectorFieldConstructionOp = EVectorFieldConstructionOp(0);
    pub const VFCO_REVOLVE: EVectorFieldConstructionOp = EVectorFieldConstructionOp(1);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ENotifyTriggerMode(pub u8);
impl ENotifyTriggerMode {
    pub const ALL_ANIMATIONS: ENotifyTriggerMode = ENotifyTriggerMode(0);
    pub const HIGHEST_WEIGHTED_ANIMATION: ENotifyTriggerMode = ENotifyTriggerMode(1);
    pub const NONE: ENotifyTriggerMode = ENotifyTriggerMode(2);
}
#[allow(non_camel_case_types)]
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
#[allow(non_camel_case_types)]
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
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EBlendSpaceAxis(pub u8);
impl EBlendSpaceAxis {
    pub const BSA_NONE: EBlendSpaceAxis = EBlendSpaceAxis(0);
    pub const BSA_X: EBlendSpaceAxis = EBlendSpaceAxis(1);
    pub const BSA_Y: EBlendSpaceAxis = EBlendSpaceAxis(2);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ECustomAttributeBlendType(pub u8);
impl ECustomAttributeBlendType {
    pub const OVERRIDE: ECustomAttributeBlendType = ECustomAttributeBlendType(0);
    pub const BLEND: ECustomAttributeBlendType = ECustomAttributeBlendType(1);
}
#[allow(non_camel_case_types)]
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
#[allow(non_camel_case_types)]
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
#[allow(non_camel_case_types)]
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
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EBlendProfileMode(pub u8);
impl EBlendProfileMode {
    pub const TIME_FACTOR: EBlendProfileMode = EBlendProfileMode(0);
    pub const WEIGHT_FACTOR: EBlendProfileMode = EBlendProfileMode(1);
    pub const BLEND_MASK: EBlendProfileMode = EBlendProfileMode(2);
}
#[allow(non_camel_case_types)]
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
#[allow(non_camel_case_types)]
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
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EVoiceSampleRate(pub i32);
impl EVoiceSampleRate {
    pub const LOW16000_HZ: EVoiceSampleRate = EVoiceSampleRate(16000);
    pub const NORMAL24000_HZ: EVoiceSampleRate = EVoiceSampleRate(24000);
}
#[allow(non_camel_case_types)]
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
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EPanningMethod(pub i8);
impl EPanningMethod {
    pub const LINEAR: EPanningMethod = EPanningMethod(0);
    pub const EQUAL_POWER: EPanningMethod = EPanningMethod(1);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EMonoChannelUpmixMethod(pub i8);
impl EMonoChannelUpmixMethod {
    pub const LINEAR: EMonoChannelUpmixMethod = EMonoChannelUpmixMethod(0);
    pub const EQUAL_POWER: EMonoChannelUpmixMethod = EMonoChannelUpmixMethod(1);
    pub const FULL_VOLUME: EMonoChannelUpmixMethod = EMonoChannelUpmixMethod(2);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ECameraShakeAttenuation(pub u8);
impl ECameraShakeAttenuation {
    pub const LINEAR: ECameraShakeAttenuation = ECameraShakeAttenuation(0);
    pub const QUADRATIC: ECameraShakeAttenuation = ECameraShakeAttenuation(1);
}
#[allow(non_camel_case_types)]
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
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ENetworkSmoothingMode(pub u8);
impl ENetworkSmoothingMode {
    pub const DISABLED: ENetworkSmoothingMode = ENetworkSmoothingMode(0);
    pub const LINEAR: ENetworkSmoothingMode = ENetworkSmoothingMode(1);
    pub const EXPONENTIAL: ENetworkSmoothingMode = ENetworkSmoothingMode(2);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EMegaLightsShadowMethod(pub u8);
impl EMegaLightsShadowMethod {
    pub const DEFAULT: EMegaLightsShadowMethod = EMegaLightsShadowMethod(0);
    pub const RAY_TRACING: EMegaLightsShadowMethod = EMegaLightsShadowMethod(1);
    pub const VIRTUAL_SHADOW_MAP: EMegaLightsShadowMethod = EMegaLightsShadowMethod(2);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EInterpToBehaviourType(pub u8);
impl EInterpToBehaviourType {
    pub const ONE_SHOT: EInterpToBehaviourType = EInterpToBehaviourType(0);
    pub const ONE_SHOT_REVERSE: EInterpToBehaviourType = EInterpToBehaviourType(1);
    pub const LOOP_RESET: EInterpToBehaviourType = EInterpToBehaviourType(2);
    pub const PING_PONG: EInterpToBehaviourType = EInterpToBehaviourType(3);
}
#[allow(non_camel_case_types)]
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
#[allow(non_camel_case_types)]
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
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ESkyLightSourceType(pub u8);
impl ESkyLightSourceType {
    pub const SLS_CAPTURED_SCENE: ESkyLightSourceType = ESkyLightSourceType(0);
    pub const SLS_SPECIFIED_CUBEMAP: ESkyLightSourceType = ESkyLightSourceType(1);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EOcclusionCombineMode(pub u8);
impl EOcclusionCombineMode {
    pub const OCM_MINIMUM: EOcclusionCombineMode = EOcclusionCombineMode(0);
    pub const OCM_MULTIPLY: EOcclusionCombineMode = EOcclusionCombineMode(1);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ELastAuthority(pub u8);
impl ELastAuthority {
    pub const UNSET: ELastAuthority = ELastAuthority(0);
    pub const SPLINE: ELastAuthority = ELastAuthority(1);
    pub const SPLINE_CURVES: ELastAuthority = ELastAuthority(2);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EStereoLayerType(pub u8);
impl EStereoLayerType {
    pub const SLT_WORLD_LOCKED: EStereoLayerType = EStereoLayerType(0);
    pub const SLT_TRACKER_LOCKED: EStereoLayerType = EStereoLayerType(1);
    pub const SLT_FACE_LOCKED: EStereoLayerType = EStereoLayerType(2);
}
#[allow(non_camel_case_types)]
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
#[allow(non_camel_case_types)]
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
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EStreamingSourceTargetState(pub u8);
impl EStreamingSourceTargetState {
    pub const LOADED: EStreamingSourceTargetState = EStreamingSourceTargetState(0);
    pub const ACTIVATED: EStreamingSourceTargetState = EStreamingSourceTargetState(1);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EGrammaticalGender(pub u8);
impl EGrammaticalGender {
    pub const NEUTER: EGrammaticalGender = EGrammaticalGender(0);
    pub const MASCULINE: EGrammaticalGender = EGrammaticalGender(1);
    pub const FEMININE: EGrammaticalGender = EGrammaticalGender(2);
    pub const MIXED: EGrammaticalGender = EGrammaticalGender(3);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EGrammaticalNumber(pub u8);
impl EGrammaticalNumber {
    pub const SINGULAR: EGrammaticalNumber = EGrammaticalNumber(0);
    pub const PLURAL: EGrammaticalNumber = EGrammaticalNumber(1);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EFontCacheType(pub u8);
impl EFontCacheType {
    pub const OFFLINE: EFontCacheType = EFontCacheType(0);
    pub const RUNTIME: EFontCacheType = EFontCacheType(1);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ERuntimeFontSource(pub u8);
impl ERuntimeFontSource {
    pub const ASSET: ERuntimeFontSource = ERuntimeFontSource(0);
    pub const CORE_STYLE_DEFAULT: ERuntimeFontSource = ERuntimeFontSource(1);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EApplicationState(pub u8);
impl EApplicationState {
    pub const UNKNOWN: EApplicationState = EApplicationState(0);
    pub const INACTIVE: EApplicationState = EApplicationState(1);
    pub const BACKGROUND: EApplicationState = EApplicationState(2);
    pub const ACTIVE: EApplicationState = EApplicationState(3);
}
#[allow(non_camel_case_types)]
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
#[allow(non_camel_case_types)]
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
#[allow(non_camel_case_types)]
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
#[allow(non_camel_case_types)]
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
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EPhysicsAssetSolverType(pub u8);
impl EPhysicsAssetSolverType {
    pub const RBAN: EPhysicsAssetSolverType = EPhysicsAssetSolverType(0);
    pub const WORLD: EPhysicsAssetSolverType = EPhysicsAssetSolverType(1);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ESettingsLockedAxis(pub u8);
impl ESettingsLockedAxis {
    pub const NONE: ESettingsLockedAxis = ESettingsLockedAxis(0);
    pub const X: ESettingsLockedAxis = ESettingsLockedAxis(1);
    pub const Y: ESettingsLockedAxis = ESettingsLockedAxis(2);
    pub const Z: ESettingsLockedAxis = ESettingsLockedAxis(3);
    pub const INVALID: ESettingsLockedAxis = ESettingsLockedAxis(4);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ESettingsDOF(pub u8);
impl ESettingsDOF {
    pub const FULL3_D: ESettingsDOF = ESettingsDOF(0);
    pub const YZ_PLANE: ESettingsDOF = ESettingsDOF(1);
    pub const XZ_PLANE: ESettingsDOF = ESettingsDOF(2);
    pub const XY_PLANE: ESettingsDOF = ESettingsDOF(3);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EMobileShadingPath(pub u8);
impl EMobileShadingPath {
    pub const FORWARD: EMobileShadingPath = EMobileShadingPath(0);
    pub const DEFERRED: EMobileShadingPath = EMobileShadingPath(1);
}
#[allow(non_camel_case_types)]
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
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EMobileFloatPrecisionMode(pub u8);
impl EMobileFloatPrecisionMode {
    pub const HALF: EMobileFloatPrecisionMode = EMobileFloatPrecisionMode(0);
    pub const FULL_MATERIAL_EXPRESSION_ONLY: EMobileFloatPrecisionMode = EMobileFloatPrecisionMode(
        1,
    );
    pub const FULL: EMobileFloatPrecisionMode = EMobileFloatPrecisionMode(2);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EShaderCompressionFormat(pub u8);
impl EShaderCompressionFormat {
    pub const NONE: EShaderCompressionFormat = EShaderCompressionFormat(0);
    pub const LZ4: EShaderCompressionFormat = EShaderCompressionFormat(1);
    pub const OODLE: EShaderCompressionFormat = EShaderCompressionFormat(2);
    pub const ZLIB: EShaderCompressionFormat = EShaderCompressionFormat(3);
}
#[allow(non_camel_case_types)]
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
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ELumenRayLightingMode(pub u8);
impl ELumenRayLightingMode {
    pub const SURFACE_CACHE: ELumenRayLightingMode = ELumenRayLightingMode(0);
    pub const HIT_LIGHTING_FOR_REFLECTIONS: ELumenRayLightingMode = ELumenRayLightingMode(
        2,
    );
    pub const HIT_LIGHTING: ELumenRayLightingMode = ELumenRayLightingMode(1);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ELumenSoftwareTracingMode(pub u8);
impl ELumenSoftwareTracingMode {
    pub const DETAIL_TRACING: ELumenSoftwareTracingMode = ELumenSoftwareTracingMode(1);
    pub const GLOBAL_TRACING: ELumenSoftwareTracingMode = ELumenSoftwareTracingMode(0);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ELumenScreenTracingSource(pub u8);
impl ELumenScreenTracingSource {
    pub const SCENE_COLOR: ELumenScreenTracingSource = ELumenScreenTracingSource(0);
    pub const ANTIALIASED_SCENE_COLOR_WITH_TRANSLUCENCY: ELumenScreenTracingSource = ELumenScreenTracingSource(
        1,
    );
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EShadowMapMethod(pub u8);
impl EShadowMapMethod {
    pub const SHADOW_MAPS: EShadowMapMethod = EShadowMapMethod(0);
    pub const VIRTUAL_SHADOW_MAPS: EShadowMapMethod = EShadowMapMethod(1);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ETranslucentSortPolicy(pub u8);
impl ETranslucentSortPolicy {
    pub const SORT_BY_DISTANCE: ETranslucentSortPolicy = ETranslucentSortPolicy(0);
    pub const SORT_BY_PROJECTED_Z: ETranslucentSortPolicy = ETranslucentSortPolicy(1);
    pub const SORT_ALONG_AXIS: ETranslucentSortPolicy = ETranslucentSortPolicy(2);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EFixedFoveationLevels(pub u8);
impl EFixedFoveationLevels {
    pub const DISABLED: EFixedFoveationLevels = EFixedFoveationLevels(0);
    pub const LOW: EFixedFoveationLevels = EFixedFoveationLevels(1);
    pub const MEDIUM: EFixedFoveationLevels = EFixedFoveationLevels(2);
    pub const HIGH: EFixedFoveationLevels = EFixedFoveationLevels(3);
    pub const HIGH_TOP: EFixedFoveationLevels = EFixedFoveationLevels(4);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ECustomDepthStencil(pub u8);
impl ECustomDepthStencil {
    pub const DISABLED: ECustomDepthStencil = ECustomDepthStencil(0);
    pub const ENABLED: ECustomDepthStencil = ECustomDepthStencil(1);
    pub const ENABLED_ON_DEMAND: ECustomDepthStencil = ECustomDepthStencil(2);
    pub const ENABLED_WITH_STENCIL: ECustomDepthStencil = ECustomDepthStencil(3);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EAutoExposureMethodUI(pub u8);
impl EAutoExposureMethodUI {
    pub const AEM_HISTOGRAM: EAutoExposureMethodUI = EAutoExposureMethodUI(0);
    pub const AEM_BASIC: EAutoExposureMethodUI = EAutoExposureMethodUI(1);
    pub const AEM_MANUAL: EAutoExposureMethodUI = EAutoExposureMethodUI(2);
}
#[allow(non_camel_case_types)]
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
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ECompositingSampleCount(pub u8);
impl ECompositingSampleCount {
    pub const ONE: ECompositingSampleCount = ECompositingSampleCount(1);
    pub const TWO: ECompositingSampleCount = ECompositingSampleCount(2);
    pub const FOUR: ECompositingSampleCount = ECompositingSampleCount(4);
    pub const EIGHT: ECompositingSampleCount = ECompositingSampleCount(8);
}
#[allow(non_camel_case_types)]
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
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EEarlyZPass(pub u8);
impl EEarlyZPass {
    pub const NONE: EEarlyZPass = EEarlyZPass(0);
    pub const OPAQUE_ONLY: EEarlyZPass = EEarlyZPass(1);
    pub const OPAQUE_AND_MASKED: EEarlyZPass = EEarlyZPass(2);
    pub const AUTO: EEarlyZPass = EEarlyZPass(3);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EClearSceneOptions(pub u8);
impl EClearSceneOptions {
    pub const NO_CLEAR: EClearSceneOptions = EClearSceneOptions(0);
    pub const HARDWARE_CLEAR: EClearSceneOptions = EClearSceneOptions(1);
    pub const QUAD_AT_MAX_Z: EClearSceneOptions = EClearSceneOptions(2);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EVelocityOutputPass(pub u8);
impl EVelocityOutputPass {
    pub const DEPTH_PASS: EVelocityOutputPass = EVelocityOutputPass(0);
    pub const BASE_PASS: EVelocityOutputPass = EVelocityOutputPass(1);
    pub const AFTER_BASE_PASS: EVelocityOutputPass = EVelocityOutputPass(2);
}
#[allow(non_camel_case_types)]
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
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EGBufferFormat(pub u8);
impl EGBufferFormat {
    pub const FORCE8_BITS_PER_CHANNEL: EGBufferFormat = EGBufferFormat(0);
    pub const DEFAULT: EGBufferFormat = EGBufferFormat(1);
    pub const HIGH_PRECISION_NORMALS: EGBufferFormat = EGBufferFormat(3);
    pub const FORCE16_BITS_PER_CHANNEL: EGBufferFormat = EGBufferFormat(5);
}
#[allow(non_camel_case_types)]
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
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ESubstrateStorageFormat(pub u8);
impl ESubstrateStorageFormat {
    pub const BLENDABLE_G_BUFFER: ESubstrateStorageFormat = ESubstrateStorageFormat(0);
    pub const ADAPTIVE_BUFFER: ESubstrateStorageFormat = ESubstrateStorageFormat(1);
}
#[allow(non_camel_case_types)]
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
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ESkinCacheDefaultBehavior(pub u8);
impl ESkinCacheDefaultBehavior {
    pub const EXCLUSIVE: ESkinCacheDefaultBehavior = ESkinCacheDefaultBehavior(0);
    pub const INCLUSIVE: ESkinCacheDefaultBehavior = ESkinCacheDefaultBehavior(1);
}
#[allow(non_camel_case_types)]
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
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EClothLODBiasMode(pub u8);
impl EClothLODBiasMode {
    pub const MAPPINGS_TO_SAME_LOD: EClothLODBiasMode = EClothLODBiasMode(0);
    pub const MAPPINGS_TO_MIN_LOD: EClothLODBiasMode = EClothLODBiasMode(1);
    pub const MAPPINGS_TO_ANY_LOD: EClothLODBiasMode = EClothLODBiasMode(2);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ESourceBusChannels(pub u8);
impl ESourceBusChannels {
    pub const MONO: ESourceBusChannels = ESourceBusChannels(0);
    pub const STEREO: ESourceBusChannels = ESourceBusChannels(1);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EStaticMeshPaintSupport(pub u8);
impl EStaticMeshPaintSupport {
    pub const DEFAULT: EStaticMeshPaintSupport = EStaticMeshPaintSupport(0);
    pub const ENABLED: EStaticMeshPaintSupport = EStaticMeshPaintSupport(1);
    pub const DISABLED: EStaticMeshPaintSupport = EStaticMeshPaintSupport(2);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ERenderFocusRule(pub u8);
impl ERenderFocusRule {
    pub const ALWAYS: ERenderFocusRule = ERenderFocusRule(0);
    pub const NON_POINTER: ERenderFocusRule = ERenderFocusRule(1);
    pub const NAVIGATION_ONLY: ERenderFocusRule = ERenderFocusRule(2);
    pub const NEVER: ERenderFocusRule = ERenderFocusRule(3);
}
#[allow(non_camel_case_types)]
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
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EFontDPI(pub u8);
impl EFontDPI {
    pub const STANDARD: EFontDPI = EFontDPI(0);
    pub const UNREAL: EFontDPI = EFontDPI(1);
    pub const CUSTOM: EFontDPI = EFontDPI(2);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EDataLayerLoadFilter(pub u8);
impl EDataLayerLoadFilter {
    pub const NONE: EDataLayerLoadFilter = EDataLayerLoadFilter(0);
    pub const CLIENT_ONLY: EDataLayerLoadFilter = EDataLayerLoadFilter(1);
    pub const SERVER_ONLY: EDataLayerLoadFilter = EDataLayerLoadFilter(2);
}
#[allow(non_camel_case_types)]
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
#[allow(non_camel_case_types)]
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
#[allow(non_camel_case_types)]
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
#[allow(non_camel_case_types)]
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
#[allow(non_camel_case_types)]
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
#[allow(non_camel_case_types)]
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
#[allow(non_camel_case_types)]
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
#[allow(non_camel_case_types)]
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
#[allow(non_camel_case_types)]
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
