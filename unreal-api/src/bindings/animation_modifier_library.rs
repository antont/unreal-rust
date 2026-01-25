#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::new_without_default)]
#![allow(clippy::new_ret_no_self)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[doc(hidden)]
pub static mut __FUNCTION_PTRS: FunctionPtrs = FunctionPtrs::empty();
pub struct FunctionPtrs {
    pub u_motion_extractor_utility_library_get_stopped_ranges_from_root_motion: *mut crate::ffi::UFunctionOpague,
    pub u_motion_extractor_utility_library_get_moving_ranges_from_root_motion: *mut crate::ffi::UFunctionOpague,
    pub u_motion_extractor_utility_library_get_desired_value: *mut crate::ffi::UFunctionOpague,
    pub u_motion_extractor_utility_library_generate_curve_name: *mut crate::ffi::UFunctionOpague,
}
impl FunctionPtrs {
    pub const fn empty() -> Self {
        Self {
            u_motion_extractor_utility_library_get_stopped_ranges_from_root_motion: std::ptr::null_mut(),
            u_motion_extractor_utility_library_get_moving_ranges_from_root_motion: std::ptr::null_mut(),
            u_motion_extractor_utility_library_get_desired_value: std::ptr::null_mut(),
            u_motion_extractor_utility_library_generate_curve_name: std::ptr::null_mut(),
        }
    }
}
pub fn initialize() {
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UMotionExtractorUtilityLibrary::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetStoppedRangesFromRootMotion"),
            &raw mut __FUNCTION_PTRS
                .u_motion_extractor_utility_library_get_stopped_ranges_from_root_motion,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetMovingRangesFromRootMotion"),
            &raw mut __FUNCTION_PTRS
                .u_motion_extractor_utility_library_get_moving_ranges_from_root_motion,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetDesiredValue"),
            &raw mut __FUNCTION_PTRS.u_motion_extractor_utility_library_get_desired_value,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GenerateCurveName"),
            &raw mut __FUNCTION_PTRS
                .u_motion_extractor_utility_library_generate_curve_name,
        );
    }
}
#[repr(C, align(4))]
pub struct FBoneReferencePair {
    pub(crate) __padding_end: [u8; 40],
}
impl FBoneReferencePair {}
#[repr(C, align(8))]
pub struct FFootDefinition {
    #[doc(hidden)]
    pub(crate) __padding_12: [u8; 12],
    pub reference_bone_name: FName,
    pub(crate) __padding_end: [u8; 64],
}
impl FFootDefinition {}
#[repr(C, align(8))]
pub struct UCopyBonesModifier {
    #[doc(hidden)]
    pub(crate) __padding_120: [u8; 120],
    pub bone_pairs: TArray<FBoneReferencePair>,
    pub bone_pose_space: crate::bindings::animation_blueprint_library::EAnimPoseSpaces,
}
impl UCopyBonesModifier {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UCopyBonesModifier")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UEncodeRootBoneModifier {
    __padding_end: [u8; 152],
}
impl UEncodeRootBoneModifier {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEncodeRootBoneModifier")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UFootstepAnimEventsModifier {
    #[doc(hidden)]
    pub(crate) __padding_120: [u8; 120],
    pub sample_rate: i32,
    pub ground_threshold: f32,
    pub speed_threshold: f32,
    pub foot_definitions: TArray<FFootDefinition>,
    pub b_should_remove_pre_existing_notifies_or_sync_markers: bool,
    __padding_end: [u8; 167],
}
impl UFootstepAnimEventsModifier {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UFootstepAnimEventsModifier")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UMirrorModifier {
    __padding_end: [u8; 136],
}
impl UMirrorModifier {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMirrorModifier")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UMotionExtractorModifier {
    #[doc(hidden)]
    pub(crate) __padding_120: [u8; 120],
    pub bone_name: FName,
    pub relative_to_bone_name: FName,
    pub motion_type: EMotionExtractor_MotionType,
    pub axis: EMotionExtractor_Axis,
    pub b_remove_curve_on_revert: bool,
    pub b_relative_to_first_frame: bool,
    pub space: EMotionExtractor_Space,
    #[doc(hidden)]
    pub(crate) __padding_150: [u8; 1],
    pub b_absolute_value: bool,
    pub math_operation: EMotionExtractor_MathOperation,
    pub modifier: f32,
    pub b_normalize: bool,
    #[doc(hidden)]
    pub(crate) __padding_164: [u8; 7],
    pub b_use_custom_curve_name: bool,
    pub custom_curve_name: FName,
}
impl UMotionExtractorModifier {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMotionExtractorModifier")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UMotionExtractorUtilityLibrary {
    __padding_end: [u8; 48],
}
impl UMotionExtractorUtilityLibrary {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMotionExtractorUtilityLibrary")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
    pub fn get_stopped_ranges_from_root_motion(
        anim_sequence: UPtr<crate::bindings::engine::UAnimSequence>,
        stop_speed_threshold: f32,
        sample_rate: f32,
    ) -> TArray<crate::bindings::core_u_object::FVector2D> {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::animation_modifier_library::__FUNCTION_PTRS
                    .u_motion_extractor_utility_library_get_stopped_ranges_from_root_motion,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &anim_sequence,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::UAnimSequence>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &stop_speed_threshold,
                __buffer.add(8).cast::<f32>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &sample_rate,
                __buffer.add(12).cast::<f32>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::animation_modifier_library::UMotionExtractorUtilityLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::animation_modifier_library::__FUNCTION_PTRS
                    .u_motion_extractor_utility_library_get_stopped_ranges_from_root_motion,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(16)
                .cast::<TArray<crate::bindings::core_u_object::FVector2D>>()
                .read()
        }
    }
    pub fn get_moving_ranges_from_root_motion(
        anim_sequence: UPtr<crate::bindings::engine::UAnimSequence>,
        stop_speed_threshold: f32,
        sample_rate: f32,
    ) -> TArray<crate::bindings::core_u_object::FVector2D> {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::animation_modifier_library::__FUNCTION_PTRS
                    .u_motion_extractor_utility_library_get_moving_ranges_from_root_motion,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &anim_sequence,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::UAnimSequence>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &stop_speed_threshold,
                __buffer.add(8).cast::<f32>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &sample_rate,
                __buffer.add(12).cast::<f32>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::animation_modifier_library::UMotionExtractorUtilityLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::animation_modifier_library::__FUNCTION_PTRS
                    .u_motion_extractor_utility_library_get_moving_ranges_from_root_motion,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(16)
                .cast::<TArray<crate::bindings::core_u_object::FVector2D>>()
                .read()
        }
    }
    pub fn get_desired_value(
        bone_transform: &crate::bindings::core_u_object::FTransform,
        last_bone_transform: &crate::bindings::core_u_object::FTransform,
        delta_time: f32,
        motion_type: EMotionExtractor_MotionType,
        axis: EMotionExtractor_Axis,
    ) -> f32 {
        let mut __stack = crate::core_data::StackAlloc::<204>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::animation_modifier_library::__FUNCTION_PTRS
                    .u_motion_extractor_utility_library_get_desired_value,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                bone_transform,
                __buffer.add(0).cast::<crate::bindings::core_u_object::FTransform>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                last_bone_transform,
                __buffer.add(96).cast::<crate::bindings::core_u_object::FTransform>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &delta_time,
                __buffer.add(192).cast::<f32>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &motion_type,
                __buffer.add(196).cast::<EMotionExtractor_MotionType>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &axis,
                __buffer.add(197).cast::<EMotionExtractor_Axis>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::animation_modifier_library::UMotionExtractorUtilityLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::animation_modifier_library::__FUNCTION_PTRS
                    .u_motion_extractor_utility_library_get_desired_value,
                __buffer,
            )
        };
        unsafe { __buffer.add(200).cast::<f32>().read() }
    }
    pub fn generate_curve_name(
        bone_name: FName,
        motion_type: EMotionExtractor_MotionType,
        axis: EMotionExtractor_Axis,
    ) -> FName {
        let mut __stack = crate::core_data::StackAlloc::<28>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::animation_modifier_library::__FUNCTION_PTRS
                    .u_motion_extractor_utility_library_generate_curve_name,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &bone_name,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &motion_type,
                __buffer.add(12).cast::<EMotionExtractor_MotionType>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &axis,
                __buffer.add(13).cast::<EMotionExtractor_Axis>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::animation_modifier_library::UMotionExtractorUtilityLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::animation_modifier_library::__FUNCTION_PTRS
                    .u_motion_extractor_utility_library_generate_curve_name,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<FName>().read() }
    }
}
#[repr(C, align(8))]
pub struct UReOrientRootBoneModifier {
    #[doc(hidden)]
    pub(crate) __padding_120: [u8; 120],
    pub rotator: crate::bindings::core_u_object::FRotator,
}
impl UReOrientRootBoneModifier {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UReOrientRootBoneModifier")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UZeroOutRootBoneModifier {
    #[doc(hidden)]
    pub(crate) __padding_120: [u8; 120],
    pub b_clip_start_frames_with_no_motion: bool,
    pub b_clip_end_frames_with_no_motion: bool,
}
impl UZeroOutRootBoneModifier {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UZeroOutRootBoneModifier")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
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
