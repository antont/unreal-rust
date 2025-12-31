#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
pub struct UMacTargetSettings {
    pub targeted_rh_is: TArray<FString>,
    pub editor_target_architecture: EMacTargetArchitecture,
    pub target_architecture: EMacTargetArchitecture,
    pub editor_default_architecture: EMacTargetArchitecture,
    pub default_architecture: EMacTargetArchitecture,
    pub b_build_all_supported_on_build_machine: bool,
    pub metal_language_version: i32,
    pub use_fast_intrinsics: bool,
    pub enable_math_optimisations: bool,
    pub audio_sample_rate: i32,
    pub audio_callback_buffer_frame_size: i32,
    pub audio_num_buffers_to_enqueue: i32,
    pub audio_max_channels: i32,
    pub audio_num_source_workers: i32,
    pub spatialization_plugin: FString,
    pub source_data_override_plugin: FString,
    pub reverb_plugin: FString,
    pub occlusion_plugin: FString,
    pub sound_cue_cook_quality_index: i32,
}
pub struct UXcodeProjectSettings {
    pub b_use_modern_xcode: bool,
    pub code_signing_team: FString,
    pub bundle_identifier: FString,
    pub code_signing_prefix: FString,
    pub application_display_name: FString,
    pub app_category: FString,
    pub template_mac_plist: crate::bindings::core_u_object::FFilePath,
    pub template_ios_plist: crate::bindings::core_u_object::FFilePath,
    pub premade_mac_entitlements: crate::bindings::core_u_object::FFilePath,
    pub shipping_specific_mac_entitlements: crate::bindings::core_u_object::FFilePath,
    pub premade_ios_entitlements: crate::bindings::core_u_object::FFilePath,
    pub shipping_specific_ios_entitlements: crate::bindings::core_u_object::FFilePath,
    pub b_use_automatic_code_signing: bool,
    pub b_mac_sign_to_run_locally: bool,
    pub mac_signing_identity: FString,
    pub ios_signing_identity: FString,
    pub ios_provisioning_profile: crate::bindings::core_u_object::FFilePath,
    pub tvos_provisioning_profile: crate::bindings::core_u_object::FFilePath,
    pub b_use_app_store_connect: bool,
    pub app_store_connect_issuer_id: FString,
    pub app_store_connect_key_id: FString,
    pub app_store_connect_key_path: crate::bindings::core_u_object::FFilePath,
    pub additional_privacy_info_mac: crate::bindings::core_u_object::FFilePath,
    pub additional_privacy_info_ios: crate::bindings::core_u_object::FFilePath,
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EMacTargetArchitecture(pub u8);
impl EMacTargetArchitecture {
    pub const MAC_TARGET_ARCHITECTURE_INTEL: EMacTargetArchitecture = EMacTargetArchitecture(
        0,
    );
    pub const MAC_TARGET_ARCHITECTURE_UNIVERSAL: EMacTargetArchitecture = EMacTargetArchitecture(
        1,
    );
    pub const MAC_TARGET_ARCHITECTURE_APPLE_SILICON: EMacTargetArchitecture = EMacTargetArchitecture(
        2,
    );
    pub const MAC_TARGET_ARCHITECTURE_HOST: EMacTargetArchitecture = EMacTargetArchitecture(
        3,
    );
}
