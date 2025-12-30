#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::prelude::*;
pub struct UAnimationCompressionLibraryDatabase {
    pub cooked_compressed_bytes: TArray<u8>,
    pub cooked_anim_sequence_mappings: TArray<u64>,
    pub highest_importance_proportion: f32,
    pub medium_importance_proportion: f32,
    pub lowest_importance_proportion: f32,
    pub strip_lowest_importance_tier: FPerPlatformBool,
    pub max_stream_request_size_kb: u32,
    pub default_visual_fidelity: ACLVisualFidelity,
    pub preview_visual_fidelity: ACLVisualFidelity,
    pub anim_sequences: TArray<UPtr<UAnimSequence>>,
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
    pub optimization_targets: TArray<UPtr<USkeletalMesh>>,
    pub keyframe_stripping_proportion: FPerPlatformFloat,
    pub keyframe_stripping_threshold: FPerPlatformFloat,
}
pub struct UAnimBoneCompressionCodec_ACLCustom {
    pub rotation_format: ACLRotationFormat,
    pub translation_format: ACLVectorFormat,
    pub scale_format: ACLVectorFormat,
    pub optimization_targets: TArray<UPtr<USkeletalMesh>>,
    pub keyframe_stripping_proportion: FPerPlatformFloat,
    pub keyframe_stripping_threshold: FPerPlatformFloat,
}
pub struct UAnimBoneCompressionCodec_ACLDatabase {
    pub database_asset: UPtr<UAnimationCompressionLibraryDatabase>,
    pub optimization_targets: TArray<UPtr<USkeletalMesh>>,
}
pub struct UAnimBoneCompressionCodec_ACLSafe {}
pub struct UAnimCurveCompressionCodec_ACL {
    pub curve_precision: f32,
    pub morph_target_position_precision: f32,
    pub morph_target_source: UPtr<USkeletalMesh>,
}
