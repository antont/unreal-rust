#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(8))]
pub struct FGooglePlayAchievementMapping {
    pub name: FString,
    pub achievement_id: FString,
}
#[repr(C, align(8))]
pub struct FGooglePlayLeaderboardMapping {
    pub name: FString,
    pub leaderboard_id: FString,
}
pub struct UAndroidRuntimeSettings {
    pub package_name: FString,
    pub store_version: i32,
    pub store_version_offset_arm64: i32,
    pub store_version_offset_x8664: i32,
    pub application_display_name: FString,
    pub version_display_name: FString,
    pub min_sdk_version: i32,
    pub target_sdk_version: i32,
    pub install_location: EAndroidInstallLocation,
    pub b_enable_lint: bool,
    pub b_package_data_inside_apk: bool,
    pub b_create_all_platforms_install: bool,
    pub b_disable_verify_obb_on_start_up: bool,
    pub b_force_small_obb_files: bool,
    pub b_allow_large_obb_files: bool,
    pub b_allow_patch_obb_file: bool,
    pub b_allow_overflow_obb_files: bool,
    pub b_dont_bundle_libraries_in_apk: bool,
    pub b_use_external_files_dir: bool,
    pub b_public_log_files: bool,
    pub orientation: EAndroidScreenOrientation,
    pub min_aspect_ratio: f32,
    pub max_aspect_ratio: f32,
    pub b_use_display_cutout: bool,
    pub rounded_edge_safe_zone: EAndroidRoundedEdgeSafeZoneDirection,
    pub b_allow_resizing: bool,
    pub b_support_size_changes: bool,
    pub b_restore_notifications_on_reboot: bool,
    pub b_full_screen: bool,
    pub b_enable_new_keyboard: bool,
    pub depth_buffer_preference: EAndroidDepthBufferPreference,
    pub b_validate_texture_formats: bool,
    pub b_force_compress_native_libs: bool,
    pub b_enable_advanced_binary_compression: bool,
    pub b_enable_bundle: bool,
    pub b_enable_universal_apk: bool,
    pub b_bundle_abi_split: bool,
    pub b_bundle_language_split: bool,
    pub b_bundle_density_split: bool,
    pub extra_manifest_node_tags: TArray<FString>,
    pub extra_application_node_tags: TArray<FString>,
    pub extra_application_settings: FString,
    pub extra_activity_node_tags: TArray<FString>,
    pub extra_activity_settings: FString,
    pub extra_permissions: TArray<FString>,
    pub b_android_voice_enabled: bool,
    pub b_record_permission_at_startup_enabled: bool,
    pub b_enable_multicast_support: bool,
    pub package_for_oculus_mobile: TArray<EOculusMobileDevice>,
    pub b_package_for_meta_quest: bool,
    pub b_remove_osig: bool,
    pub key_store: FString,
    pub key_alias: FString,
    pub key_store_password: FString,
    pub key_password: FString,
    pub debug_key_store: FString,
    pub debug_key_alias: FString,
    pub debug_key_store_password: FString,
    pub debug_key_password: FString,
    pub b_build_for_arm64: bool,
    pub b_build_for_x8664: bool,
    pub b_build_for_es31: bool,
    pub b_supports_vulkan: bool,
    pub b_supports_vulkan_sm5: bool,
    pub debug_vulkan_layer_directory: crate::bindings::core_u_object::FDirectoryPath,
    pub debug_vulkan_device_layers: TArray<FString>,
    pub debug_vulkan_instance_layers: TArray<FString>,
    pub b_android_open_gl_supports_backbuffer_sampling: bool,
    pub b_force_emulator_profile_selection_in_non_shipping_builds: bool,
    pub b_detect_vulkan_by_default: bool,
    pub b_build_with_hidden_symbol_visibility: bool,
    pub b_compress_debug_symbols: bool,
    pub b_disable_stack_protector: bool,
    pub b_disable_lib_cpp_shared_dependency_validation: bool,
    pub b_save_symbols: bool,
    pub b_strip_shader_reflection: bool,
    pub b_enable_google_play_support: bool,
    pub request_code_for_play_games_activities: i32,
    pub play_games_client_id: FString,
    pub b_force_refresh_token: bool,
    pub games_app_id: FString,
    pub achievement_map: TArray<FGooglePlayAchievementMapping>,
    pub leaderboard_map: TArray<FGooglePlayLeaderboardMapping>,
    pub b_support_ad_mob: bool,
    pub ad_mob_app_id: FString,
    pub tag_for_child_directed_treatment: ETagForChildDirectedTreatment,
    pub tag_for_under_age_of_consent: ETagForUnderAgeOfConsent,
    pub max_ad_content_rating: EMaxAdContentRating,
    pub ad_mob_ad_unit_id: FString,
    pub ad_mob_ad_unit_i_ds: TArray<FString>,
    pub google_play_license_key: FString,
    pub gcm_client_sender_id: FString,
    pub b_show_launch_image: bool,
    pub b_allow_imu: bool,
    pub b_allow_controllers: bool,
    pub b_block_android_keys_on_controllers: bool,
    pub b_controllers_block_device_feedback: bool,
    pub audio_sample_rate: i32,
    pub audio_callback_buffer_frame_size: i32,
    pub audio_num_buffers_to_enqueue: i32,
    pub audio_max_channels: i32,
    pub audio_num_source_workers: i32,
    pub spatialization_plugin: FString,
    pub source_data_override_plugin: FString,
    pub reverb_plugin: FString,
    pub occlusion_plugin: FString,
    pub compression_overrides: crate::bindings::audio_platform_configuration::FPlatformRuntimeAudioCompressionOverrides,
    pub cache_size_kb: i32,
    pub max_chunk_size_override_kb: i32,
    pub b_resample_for_device: bool,
    pub sound_cue_cook_quality_index: i32,
    pub max_sample_rate: f32,
    pub high_sample_rate: f32,
    pub med_sample_rate: f32,
    pub low_sample_rate: f32,
    pub min_sample_rate: f32,
    pub compression_quality_modifier: f32,
    pub auto_streaming_threshold: f32,
    pub android_graphics_debugger: EAndroidGraphicsDebugger,
    pub mali_graphics_debugger_path: crate::bindings::core_u_object::FDirectoryPath,
    pub b_enable_mali_perf_counters: bool,
    pub b_multi_target_format_etc2: bool,
    pub b_multi_target_format_dxt: bool,
    pub b_multi_target_format_astc: bool,
    pub texture_format_priority_etc2: f32,
    pub texture_format_priority_dxt: f32,
    pub texture_format_priority_astc: f32,
    pub b_astc_use_rdo: bool,
    pub sdkapi_level_override: FString,
    pub ndkapi_level_override: FString,
    pub build_tools_override: FString,
    pub b_stream_landscape_mesh_lo_ds: bool,
    pub b_enable_dom_storage: bool,
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EAndroidInstallLocation(pub u8);
impl EAndroidInstallLocation {
    pub const INTERNAL_ONLY: EAndroidInstallLocation = EAndroidInstallLocation(0);
    pub const PREFER_EXTERNAL: EAndroidInstallLocation = EAndroidInstallLocation(1);
    pub const AUTO: EAndroidInstallLocation = EAndroidInstallLocation(2);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EAndroidScreenOrientation(pub u8);
impl EAndroidScreenOrientation {
    pub const PORTRAIT: EAndroidScreenOrientation = EAndroidScreenOrientation(0);
    pub const REVERSE_PORTRAIT: EAndroidScreenOrientation = EAndroidScreenOrientation(1);
    pub const SENSOR_PORTRAIT: EAndroidScreenOrientation = EAndroidScreenOrientation(2);
    pub const LANDSCAPE: EAndroidScreenOrientation = EAndroidScreenOrientation(3);
    pub const REVERSE_LANDSCAPE: EAndroidScreenOrientation = EAndroidScreenOrientation(
        4,
    );
    pub const SENSOR_LANDSCAPE: EAndroidScreenOrientation = EAndroidScreenOrientation(5);
    pub const SENSOR: EAndroidScreenOrientation = EAndroidScreenOrientation(6);
    pub const FULL_SENSOR: EAndroidScreenOrientation = EAndroidScreenOrientation(7);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EAndroidRoundedEdgeSafeZoneDirection(pub u8);
impl EAndroidRoundedEdgeSafeZoneDirection {
    pub const NONE: EAndroidRoundedEdgeSafeZoneDirection = EAndroidRoundedEdgeSafeZoneDirection(
        0,
    );
    pub const HORIZONTAL: EAndroidRoundedEdgeSafeZoneDirection = EAndroidRoundedEdgeSafeZoneDirection(
        1,
    );
    pub const VERTICAL: EAndroidRoundedEdgeSafeZoneDirection = EAndroidRoundedEdgeSafeZoneDirection(
        2,
    );
    pub const BOTH: EAndroidRoundedEdgeSafeZoneDirection = EAndroidRoundedEdgeSafeZoneDirection(
        3,
    );
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EAndroidDepthBufferPreference(pub u8);
impl EAndroidDepthBufferPreference {
    pub const DEFAULT: EAndroidDepthBufferPreference = EAndroidDepthBufferPreference(0);
    pub const BITS16: EAndroidDepthBufferPreference = EAndroidDepthBufferPreference(16);
    pub const BITS24: EAndroidDepthBufferPreference = EAndroidDepthBufferPreference(24);
    pub const BITS32: EAndroidDepthBufferPreference = EAndroidDepthBufferPreference(32);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EOculusMobileDevice(pub u8);
impl EOculusMobileDevice {
    pub const QUEST: EOculusMobileDevice = EOculusMobileDevice(1);
    pub const QUEST2: EOculusMobileDevice = EOculusMobileDevice(2);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ETagForChildDirectedTreatment(pub u8);
impl ETagForChildDirectedTreatment {
    pub const TAG_FOR_CHILD_DIRECTED_TREATMENT_UNSPECIFIED: ETagForChildDirectedTreatment = ETagForChildDirectedTreatment(
        0,
    );
    pub const TAG_FOR_CHILD_DIRECTED_TREATMENT_TRUE: ETagForChildDirectedTreatment = ETagForChildDirectedTreatment(
        1,
    );
    pub const TAG_FOR_CHILD_DIRECTED_TREATMENT_FALSE: ETagForChildDirectedTreatment = ETagForChildDirectedTreatment(
        2,
    );
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ETagForUnderAgeOfConsent(pub u8);
impl ETagForUnderAgeOfConsent {
    pub const TAG_FOR_UNDER_AGE_OF_CONSENT_UNSPECIFIED: ETagForUnderAgeOfConsent = ETagForUnderAgeOfConsent(
        0,
    );
    pub const TAG_FOR_UNDER_AGE_OF_CONSENT_TRUE: ETagForUnderAgeOfConsent = ETagForUnderAgeOfConsent(
        1,
    );
    pub const TAG_FOR_UNDER_AGE_OF_CONSENT_FALSE: ETagForUnderAgeOfConsent = ETagForUnderAgeOfConsent(
        2,
    );
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EMaxAdContentRating(pub u8);
impl EMaxAdContentRating {
    pub const MAX_AD_CONTENT_RATING_G: EMaxAdContentRating = EMaxAdContentRating(0);
    pub const MAX_AD_CONTENT_RATING_PG: EMaxAdContentRating = EMaxAdContentRating(1);
    pub const MAX_AD_CONTENT_RATING_T: EMaxAdContentRating = EMaxAdContentRating(2);
    pub const MAX_AD_CONTENT_RATING_MA: EMaxAdContentRating = EMaxAdContentRating(2);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EAndroidGraphicsDebugger(pub u8);
impl EAndroidGraphicsDebugger {
    pub const NONE: EAndroidGraphicsDebugger = EAndroidGraphicsDebugger(0);
    pub const MALI: EAndroidGraphicsDebugger = EAndroidGraphicsDebugger(1);
    pub const ADRENO: EAndroidGraphicsDebugger = EAndroidGraphicsDebugger(2);
}
