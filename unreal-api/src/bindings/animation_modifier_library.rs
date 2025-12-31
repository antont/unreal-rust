#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(4))]
pub struct FBoneReferencePair {
    pub source_bone: crate::bindings::engine::FBoneReference,
    pub target_bone: crate::bindings::engine::FBoneReference,
}
#[repr(C, align(4))]
pub struct FEncodeRootBoneWeightedBone {
    pub bone: crate::bindings::engine::FBoneReference,
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
    pub footstep_notify: TSubclassOf<crate::bindings::engine::UAnimNotify>,
    pub footstep_notify_detection_technique: EDetectionTechnique,
    pub b_should_skip_notify_if_foot_bone_speed_starts_below_threshold: bool,
}
pub struct UCopyBonesModifier {
    pub bone_pairs: TArray<FBoneReferencePair>,
    pub bone_pose_space: crate::bindings::animation_blueprint_library::EAnimPoseSpaces,
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
    pub mirror_data_table: UPtr<crate::bindings::engine::UMirrorDataTable>,
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
    pub rotator: crate::bindings::core_u_object::FRotator,
}
pub struct UZeroOutRootBoneModifier {
    pub b_clip_start_frames_with_no_motion: bool,
    pub b_clip_end_frames_with_no_motion: bool,
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EEncodeRootBoneAxis(pub u8);
impl EEncodeRootBoneAxis {
    pub const X: EEncodeRootBoneAxis = EEncodeRootBoneAxis(0);
    pub const Y: EEncodeRootBoneAxis = EEncodeRootBoneAxis(1);
    pub const Z: EEncodeRootBoneAxis = EEncodeRootBoneAxis(2);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EDetectionTechnique(pub u8);
impl EDetectionTechnique {
    pub const PASS_THROUGH_REFERENCE_BONE: EDetectionTechnique = EDetectionTechnique(0);
    pub const FOOT_BONE_REACHES_GROUND: EDetectionTechnique = EDetectionTechnique(1);
    pub const FOOT_BONE_SPEED: EDetectionTechnique = EDetectionTechnique(2);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EMotionExtractor_MotionType(pub u8);
impl EMotionExtractor_MotionType {
    pub const NONE: EMotionExtractor_MotionType = EMotionExtractor_MotionType(0);
    pub const TRANSLATION: EMotionExtractor_MotionType = EMotionExtractor_MotionType(1);
    pub const ROTATION: EMotionExtractor_MotionType = EMotionExtractor_MotionType(2);
    pub const SCALE: EMotionExtractor_MotionType = EMotionExtractor_MotionType(4);
    pub const TRANSLATION_SPEED: EMotionExtractor_MotionType = EMotionExtractor_MotionType(
        8,
    );
    pub const ROTATION_SPEED: EMotionExtractor_MotionType = EMotionExtractor_MotionType(
        16,
    );
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EMotionExtractor_Axis(pub u8);
impl EMotionExtractor_Axis {
    pub const X: EMotionExtractor_Axis = EMotionExtractor_Axis(0);
    pub const Y: EMotionExtractor_Axis = EMotionExtractor_Axis(1);
    pub const Z: EMotionExtractor_Axis = EMotionExtractor_Axis(2);
    pub const XY: EMotionExtractor_Axis = EMotionExtractor_Axis(3);
    pub const XZ: EMotionExtractor_Axis = EMotionExtractor_Axis(4);
    pub const YZ: EMotionExtractor_Axis = EMotionExtractor_Axis(5);
    pub const XYZ: EMotionExtractor_Axis = EMotionExtractor_Axis(6);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EMotionExtractor_Space(pub u8);
impl EMotionExtractor_Space {
    pub const COMPONENT_SPACE: EMotionExtractor_Space = EMotionExtractor_Space(0);
    pub const LOCAL_SPACE: EMotionExtractor_Space = EMotionExtractor_Space(1);
    pub const RELATIVE_TO_BONE: EMotionExtractor_Space = EMotionExtractor_Space(2);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EMotionExtractor_MathOperation(pub u8);
impl EMotionExtractor_MathOperation {
    pub const NONE: EMotionExtractor_MathOperation = EMotionExtractor_MathOperation(0);
    pub const ADDITION: EMotionExtractor_MathOperation = EMotionExtractor_MathOperation(
        1,
    );
    pub const SUBTRACTION: EMotionExtractor_MathOperation = EMotionExtractor_MathOperation(
        2,
    );
    pub const DIVISION: EMotionExtractor_MathOperation = EMotionExtractor_MathOperation(
        3,
    );
    pub const MULTIPLICATION: EMotionExtractor_MathOperation = EMotionExtractor_MathOperation(
        4,
    );
}
