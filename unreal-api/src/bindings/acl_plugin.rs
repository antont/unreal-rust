#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
pub struct UAnimationCompressionLibraryDatabase {
    pub cooked_compressed_bytes: TArray<u8>,
    pub cooked_anim_sequence_mappings: TArray<u64>,
    pub highest_importance_proportion: f32,
    pub medium_importance_proportion: f32,
    pub lowest_importance_proportion: f32,
    pub strip_lowest_importance_tier: crate::bindings::core_u_object::FPerPlatformBool,
    pub max_stream_request_size_kb: u32,
    pub default_visual_fidelity: ACLVisualFidelity,
    pub preview_visual_fidelity: ACLVisualFidelity,
    pub anim_sequences: TArray<UPtr<crate::bindings::engine::UAnimSequence>>,
    pub num_anim_sequences: i32,
    pub anim_sequences_old_size_kb: i32,
    pub anim_sequences_new_size_kb: i32,
    pub database_size_kb: i32,
    pub database_metadata_size_kb: i32,
    pub medium_importance_size_kb: i32,
    pub low_importance_size_size_kb: i32,
}
pub struct UAnimBoneCompressionCodec_ACLBase {
    pub compression_level: ACLCompressionLevel,
    pub phantom_track_mode: ACLPhantomTrackMode,
    pub default_virtual_vertex_distance: f32,
    pub safe_virtual_vertex_distance: f32,
    pub error_threshold: f32,
}
pub struct UAnimBoneCompressionCodec_ACL {
    pub optimization_targets: TArray<UPtr<crate::bindings::engine::USkeletalMesh>>,
    pub keyframe_stripping_proportion: crate::bindings::core_u_object::FPerPlatformFloat,
    pub keyframe_stripping_threshold: crate::bindings::core_u_object::FPerPlatformFloat,
}
pub struct UAnimBoneCompressionCodec_ACLCustom {
    pub rotation_format: ACLRotationFormat,
    pub translation_format: ACLVectorFormat,
    pub scale_format: ACLVectorFormat,
    pub optimization_targets: TArray<UPtr<crate::bindings::engine::USkeletalMesh>>,
    pub keyframe_stripping_proportion: crate::bindings::core_u_object::FPerPlatformFloat,
    pub keyframe_stripping_threshold: crate::bindings::core_u_object::FPerPlatformFloat,
}
pub struct UAnimBoneCompressionCodec_ACLDatabase {
    pub database_asset: UPtr<UAnimationCompressionLibraryDatabase>,
    pub optimization_targets: TArray<UPtr<crate::bindings::engine::USkeletalMesh>>,
}
pub struct UAnimBoneCompressionCodec_ACLSafe {}
pub struct UAnimCurveCompressionCodec_ACL {
    pub curve_precision: f32,
    pub morph_target_position_precision: f32,
    pub morph_target_source: UPtr<crate::bindings::engine::USkeletalMesh>,
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ACLVisualFidelity(pub u8);
impl ACLVisualFidelity {
    pub const HIGHEST: ACLVisualFidelity = ACLVisualFidelity(0);
    pub const MEDIUM: ACLVisualFidelity = ACLVisualFidelity(1);
    pub const LOWEST: ACLVisualFidelity = ACLVisualFidelity(2);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ACLVisualFidelityChangeResult(pub u8);
impl ACLVisualFidelityChangeResult {
    pub const DISPATCHED: ACLVisualFidelityChangeResult = ACLVisualFidelityChangeResult(
        0,
    );
    pub const COMPLETED: ACLVisualFidelityChangeResult = ACLVisualFidelityChangeResult(
        1,
    );
    pub const FAILED: ACLVisualFidelityChangeResult = ACLVisualFidelityChangeResult(2);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ACLCompressionLevel(pub u8);
impl ACLCompressionLevel {
    pub const ACLCL_LOWEST: ACLCompressionLevel = ACLCompressionLevel(0);
    pub const ACLCL_LOW: ACLCompressionLevel = ACLCompressionLevel(1);
    pub const ACLCL_MEDIUM: ACLCompressionLevel = ACLCompressionLevel(2);
    pub const ACLCL_HIGH: ACLCompressionLevel = ACLCompressionLevel(3);
    pub const ACLCL_HIGHEST: ACLCompressionLevel = ACLCompressionLevel(4);
    pub const ACLCL_AUTOMATIC: ACLCompressionLevel = ACLCompressionLevel(5);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ACLPhantomTrackMode(pub u8);
impl ACLPhantomTrackMode {
    pub const IGNORE: ACLPhantomTrackMode = ACLPhantomTrackMode(0);
    pub const STRIP: ACLPhantomTrackMode = ACLPhantomTrackMode(1);
    pub const WARN: ACLPhantomTrackMode = ACLPhantomTrackMode(2);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ACLRotationFormat(pub u8);
impl ACLRotationFormat {
    pub const ACLRF_QUAT_128: ACLRotationFormat = ACLRotationFormat(0);
    pub const ACLRF_QUAT_DROP_W_96: ACLRotationFormat = ACLRotationFormat(1);
    pub const ACLRF_QUAT_DROP_W_VARIABLE: ACLRotationFormat = ACLRotationFormat(2);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ACLVectorFormat(pub u8);
impl ACLVectorFormat {
    pub const ACLVF_VECTOR3_96: ACLVectorFormat = ACLVectorFormat(0);
    pub const ACLVF_VECTOR3_VARIABLE: ACLVectorFormat = ACLVectorFormat(1);
}
