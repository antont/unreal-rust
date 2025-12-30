#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::prelude::*;
#[repr(C, align(4))]
pub struct FBoneReferencePair {
    pub source_bone: FBoneReference,
    pub target_bone: FBoneReference,
}
#[repr(C, align(4))]
pub struct FEncodeRootBoneWeightedBone {
    pub bone: FBoneReference,
    pub weight: f32,
}
#[repr(C, align(4))]
pub struct FEncodeRootBoneWeightedBoneAxis {
    pub bone_axis: EEncodeRootBoneAxis,
}
#[repr(C, align(8))]
pub struct FFootDefinition {
    pub foot_bone_name: FName,
    pub reference_bone_name: FName,
    pub b_should_generate_sync_markers: bool,
    pub sync_marker_track_name: FName,
    pub sync_marker_name: FName,
    pub sync_marker_detection_technique: EDetectionTechnique,
    pub b_should_skip_sync_marker_if_foot_bone_speed_starts_below_threshold: bool,
    pub b_should_generate_notifies: bool,
    pub footstep_notify_track_name: FName,
    pub footstep_notify: TSubclassOf<UAnimNotify>,
    pub footstep_notify_detection_technique: EDetectionTechnique,
    pub b_should_skip_notify_if_foot_bone_speed_starts_below_threshold: bool,
}
pub struct UCopyBonesModifier {
    pub bone_pairs: TArray<FBoneReferencePair>,
    pub bone_pose_space: EAnimPoseSpaces,
}
pub struct UEncodeRootBoneModifier {
    pub weighted_bone_to_compute_root_position: TArray<FEncodeRootBoneWeightedBone>,
    pub weighted_bone_to_compute_root_orientation: TArray<
        FEncodeRootBoneWeightedBoneAxis,
    >,
}
pub struct UFootstepAnimEventsModifier {
    pub sample_rate: i32,
    pub ground_threshold: f32,
    pub speed_threshold: f32,
    pub foot_definitions: TArray<FFootDefinition>,
    pub b_should_remove_pre_existing_notifies_or_sync_markers: bool,
    pub generated_notify_tracks: TSet<FName>,
    pub processed_notify_tracks: TSet<FName>,
}
pub struct UMirrorModifier {
    pub mirror_data_table: UPtr<UMirrorDataTable>,
    pub b_update_sync_markers: bool,
    pub b_update_notifies: bool,
}
pub struct UMotionExtractorModifier {
    pub bone_name: FName,
    pub relative_to_bone_name: FName,
    pub motion_type: EMotionExtractor_MotionType,
    pub axis: EMotionExtractor_Axis,
    pub b_remove_curve_on_revert: bool,
    pub b_relative_to_first_frame: bool,
    pub space: EMotionExtractor_Space,
    pub b_component_space_deprecated: bool,
    pub b_absolute_value: bool,
    pub math_operation: EMotionExtractor_MathOperation,
    pub modifier: f32,
    pub b_normalize: bool,
    pub sample_rate: i32,
    pub b_use_custom_curve_name: bool,
    pub custom_curve_name: FName,
}
pub struct UMotionExtractorUtilityLibrary {}
pub struct UReOrientRootBoneModifier {
    pub rotator: FRotator,
}
pub struct UZeroOutRootBoneModifier {
    pub b_clip_start_frames_with_no_motion: bool,
    pub b_clip_end_frames_with_no_motion: bool,
}
