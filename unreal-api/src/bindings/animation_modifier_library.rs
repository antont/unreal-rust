#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(4))]
pub struct FBoneReferencePair {
    __padding_end: [u8; 40],
}
impl FBoneReferencePair {}
#[repr(C, align(8))]
pub struct FFootDefinition {
    #[doc(hidden)]
    __padding_12: [u8; 12],
    pub reference_bone_name: FName,
    __padding_end: [u8; 64],
}
impl FFootDefinition {}
#[repr(C, align(8))]
pub struct UCopyBonesModifier {
    #[doc(hidden)]
    __padding_120: [u8; 120],
    pub bone_pairs: TArray<FBoneReferencePair>,
    pub bone_pose_space: crate::bindings::animation_blueprint_library::EAnimPoseSpaces,
    __padding_end: [u8; 7],
}
impl UCopyBonesModifier {}
#[repr(C, align(8))]
pub struct UEncodeRootBoneModifier {
    __padding_end: [u8; 152],
}
impl UEncodeRootBoneModifier {}
#[repr(C, align(8))]
pub struct UFootstepAnimEventsModifier {
    #[doc(hidden)]
    __padding_120: [u8; 120],
    pub sample_rate: i32,
    pub ground_threshold: f32,
    pub speed_threshold: f32,
    pub foot_definitions: TArray<FFootDefinition>,
    pub b_should_remove_pre_existing_notifies_or_sync_markers: bool,
    __padding_end: [u8; 167],
}
impl UFootstepAnimEventsModifier {}
#[repr(C, align(8))]
pub struct UMirrorModifier {
    __padding_end: [u8; 136],
}
impl UMirrorModifier {}
#[repr(C, align(8))]
pub struct UMotionExtractorModifier {
    #[doc(hidden)]
    __padding_120: [u8; 120],
    pub bone_name: FName,
    pub relative_to_bone_name: FName,
    pub motion_type: EMotionExtractor_MotionType,
    pub axis: EMotionExtractor_Axis,
    pub b_remove_curve_on_revert: bool,
    pub b_relative_to_first_frame: bool,
    pub space: EMotionExtractor_Space,
    #[doc(hidden)]
    __padding_150: [u8; 1],
    pub b_absolute_value: bool,
    pub math_operation: EMotionExtractor_MathOperation,
    pub modifier: f32,
    pub b_normalize: bool,
    #[doc(hidden)]
    __padding_164: [u8; 7],
    pub b_use_custom_curve_name: bool,
    pub custom_curve_name: FName,
    __padding_end: [u8; 4],
}
impl UMotionExtractorModifier {}
#[repr(C, align(8))]
pub struct UMotionExtractorUtilityLibrary {
    __padding_end: [u8; 48],
}
impl UMotionExtractorUtilityLibrary {}
#[repr(C, align(8))]
pub struct UReOrientRootBoneModifier {
    #[doc(hidden)]
    __padding_120: [u8; 120],
    pub rotator: crate::bindings::core_u_object::FRotator,
}
impl UReOrientRootBoneModifier {}
#[repr(C, align(8))]
pub struct UZeroOutRootBoneModifier {
    #[doc(hidden)]
    __padding_120: [u8; 120],
    pub b_clip_start_frames_with_no_motion: bool,
    pub b_clip_end_frames_with_no_motion: bool,
    __padding_end: [u8; 6],
}
impl UZeroOutRootBoneModifier {}
#[repr(transparent)]
pub struct EEncodeRootBoneAxis(pub u8);
impl EEncodeRootBoneAxis {
    pub const X: EEncodeRootBoneAxis = EEncodeRootBoneAxis(0);
    pub const Y: EEncodeRootBoneAxis = EEncodeRootBoneAxis(1);
    pub const Z: EEncodeRootBoneAxis = EEncodeRootBoneAxis(2);
}
#[repr(transparent)]
pub struct EDetectionTechnique(pub u8);
impl EDetectionTechnique {
    pub const PASS_THROUGH_REFERENCE_BONE: EDetectionTechnique = EDetectionTechnique(0);
    pub const FOOT_BONE_REACHES_GROUND: EDetectionTechnique = EDetectionTechnique(1);
    pub const FOOT_BONE_SPEED: EDetectionTechnique = EDetectionTechnique(2);
}
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
#[repr(transparent)]
pub struct EMotionExtractor_Space(pub u8);
impl EMotionExtractor_Space {
    pub const COMPONENT_SPACE: EMotionExtractor_Space = EMotionExtractor_Space(0);
    pub const LOCAL_SPACE: EMotionExtractor_Space = EMotionExtractor_Space(1);
    pub const RELATIVE_TO_BONE: EMotionExtractor_Space = EMotionExtractor_Space(2);
}
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
