#![allow(clippy::all)]
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
    pub u_animation_warping_library_get_vector_value_from_curve: *mut crate::ffi::UFunctionOpague,
    pub u_animation_warping_library_get_offset_root_transform: *mut crate::ffi::UFunctionOpague,
    pub u_animation_warping_library_get_float_value_from_curve: *mut crate::ffi::UFunctionOpague,
    pub u_animation_warping_library_get_curve_value_from_animation: *mut crate::ffi::UFunctionOpague,
}
impl FunctionPtrs {
    pub const fn empty() -> Self {
        Self {
            u_animation_warping_library_get_vector_value_from_curve: std::ptr::null_mut(),
            u_animation_warping_library_get_offset_root_transform: std::ptr::null_mut(),
            u_animation_warping_library_get_float_value_from_curve: std::ptr::null_mut(),
            u_animation_warping_library_get_curve_value_from_animation: std::ptr::null_mut(),
        }
    }
}
pub fn initialize() {
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UAnimationWarpingLibrary::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetVectorValueFromCurve"),
                &raw mut __FUNCTION_PTRS
                    .u_animation_warping_library_get_vector_value_from_curve,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetOffsetRootTransform"),
                &raw mut __FUNCTION_PTRS
                    .u_animation_warping_library_get_offset_root_transform,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetFloatValueFromCurve"),
                &raw mut __FUNCTION_PTRS
                    .u_animation_warping_library_get_float_value_from_curve,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetCurveValueFromAnimation"),
                &raw mut __FUNCTION_PTRS
                    .u_animation_warping_library_get_curve_value_from_animation,
            );
        }
    }
}
#[repr(C, align(4))]
pub struct FFootPlacementInterpolationSettings {
    pub(crate) __padding_end: [u8; 44],
}
impl FFootPlacementInterpolationSettings {}
#[repr(C, align(4))]
pub struct FFootPlacementTraceSettings {
    pub(crate) __padding_end: [u8; 24],
}
impl FFootPlacementTraceSettings {}
#[repr(C, align(4))]
pub struct FFootPlacementPelvisSettings {
    pub(crate) __padding_end: [u8; 40],
}
impl FFootPlacementPelvisSettings {}
#[repr(C, align(4))]
pub struct FFootPlacementPlantSettings {
    pub(crate) __padding_end: [u8; 52],
}
impl FFootPlacementPlantSettings {}
#[repr(C, align(16))]
pub struct FAnimNode_FootPlacement {
    #[doc(hidden)]
    pub(crate) __padding_468: [u8; 468],
    pub pelvis_settings: FFootPlacementPelvisSettings,
    #[doc(hidden)]
    pub(crate) __padding_528: [u8; 20],
    pub plant_settings: FFootPlacementPlantSettings,
    pub interpolation_settings: FFootPlacementInterpolationSettings,
    pub trace_settings: FFootPlacementTraceSettings,
    pub(crate) __padding_end: [u8; 904],
}
impl FAnimNode_FootPlacement {}
#[repr(C, align(16))]
pub struct FAnimNode_OffsetRootBone {
    #[doc(hidden)]
    pub(crate) __padding_136: [u8; 136],
    pub source: crate::bindings::engine::FPoseLink,
    pub(crate) __padding_end: [u8; 368],
}
impl FAnimNode_OffsetRootBone {}
#[repr(C, align(16))]
pub struct FAnimNode_OrientationWarping {
    #[doc(hidden)]
    pub(crate) __padding_428: [u8; 428],
    pub target_time: f32,
    pub orientation_angle: f32,
    pub locomotion_angle: f32,
    pub locomotion_direction: crate::bindings::core_u_object::FVector,
    pub min_root_motion_speed_threshold: f32,
    pub locomotion_angle_delta_threshold: f32,
    #[doc(hidden)]
    pub(crate) __padding_528: [u8; 56],
    pub current_anim_asset: UPtr<crate::bindings::engine::UAnimationAsset>,
    pub current_anim_asset_time: f32,
    #[doc(hidden)]
    pub(crate) __padding_568: [u8; 24],
    pub manual_root_motion_velocity: crate::bindings::core_u_object::FVector,
    pub warping_space: EOrientationWarpingSpace,
    pub warping_space_transform: crate::bindings::core_u_object::FTransform,
    pub(crate) __padding_end: [u8; 192],
}
impl FAnimNode_OrientationWarping {}
#[repr(C, align(8))]
pub struct FAnimNode_OverrideRootMotion {
    #[doc(hidden)]
    pub(crate) __padding_136: [u8; 136],
    pub source: crate::bindings::engine::FPoseLink,
    pub(crate) __padding_end: [u8; 48],
}
impl FAnimNode_OverrideRootMotion {}
#[repr(C, align(8))]
pub struct FAnimNode_SlopeWarping {
    pub(crate) __padding_end: [u8; 968],
}
impl FAnimNode_SlopeWarping {}
#[repr(C, align(16))]
pub struct FAnimNode_Steering {
    #[doc(hidden)]
    pub(crate) __padding_432: [u8; 432],
    pub target_orientation: crate::bindings::core_u_object::FQuat,
    pub b_mirrored: bool,
    pub procedural_target_time: f32,
    #[doc(hidden)]
    pub(crate) __padding_476: [u8; 4],
    pub animated_target_time: f32,
    #[doc(hidden)]
    pub(crate) __padding_504: [u8; 24],
    pub mirror_data_table: UPtr<crate::bindings::engine::UMirrorDataTable>,
    pub current_anim_asset: UPtr<crate::bindings::engine::UAnimationAsset>,
    pub current_anim_asset_time: f32,
    pub(crate) __padding_end: [u8; 132],
}
impl FAnimNode_Steering {}
#[repr(C, align(4))]
pub struct FStrideWarpingFootDefinition {
    pub(crate) __padding_end: [u8; 60],
}
impl FStrideWarpingFootDefinition {}
#[repr(C, align(8))]
pub struct FAnimNode_StrideWarping {
    #[doc(hidden)]
    pub(crate) __padding_432: [u8; 432],
    pub stride_direction: crate::bindings::core_u_object::FVector,
    pub stride_scale: f32,
    pub locomotion_speed: f32,
    pub min_root_motion_speed_threshold: f32,
    #[doc(hidden)]
    pub(crate) __padding_552: [u8; 80],
    pub floor_normal_direction: crate::bindings::anim_graph_runtime::FWarpingVectorValue,
    pub gravity_direction: crate::bindings::anim_graph_runtime::FWarpingVectorValue,
    pub(crate) __padding_end: [u8; 240],
}
impl FAnimNode_StrideWarping {}
#[repr(C, align(16))]
pub struct FAnimNode_WarpTest {
    #[doc(hidden)]
    pub(crate) __padding_136: [u8; 136],
    pub source: crate::bindings::engine::FPoseLink,
    pub(crate) __padding_end: [u8; 144],
}
impl FAnimNode_WarpTest {}
#[repr(C, align(8))]
pub struct UAnimationWarpingLibrary {
    __padding_end: [u8; 48],
}
impl UAnimationWarpingLibrary {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAnimationWarpingLibrary")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAnimationWarpingLibrary")
            .copied()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings()
                .core_fns
                .get_cdo_from_class)(class.ptr.cast(), &raw mut cdo);
            cdo
        }
    }
    pub fn get_vector_value_from_curve(
        in_curve: UPtr<crate::bindings::engine::UCurveVector>,
        time: f32,
        out_value: &mut crate::bindings::core_u_object::FVector,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<41>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::animation_warping_runtime::__FUNCTION_PTRS
                    .u_animation_warping_library_get_vector_value_from_curve,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_curve,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::UCurveVector>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&time, __buffer.add(8).cast::<f32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_value,
                __buffer.add(16).cast::<crate::bindings::core_u_object::FVector>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::animation_warping_runtime::UAnimationWarpingLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::animation_warping_runtime::__FUNCTION_PTRS
                    .u_animation_warping_library_get_vector_value_from_curve,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(16)
                .cast::<crate::bindings::core_u_object::FVector>()
                .swap(out_value);
        }
        unsafe { __buffer.add(40).cast::<bool>().read() }
    }
    pub fn get_offset_root_transform(
        node: &crate::bindings::engine::FAnimNodeReference,
    ) -> crate::bindings::core_u_object::FTransform {
        let mut __stack = crate::core_data::StackAlloc::<112>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::animation_warping_runtime::__FUNCTION_PTRS
                    .u_animation_warping_library_get_offset_root_transform,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                node,
                __buffer.add(0).cast::<crate::bindings::engine::FAnimNodeReference>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::animation_warping_runtime::UAnimationWarpingLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::animation_warping_runtime::__FUNCTION_PTRS
                    .u_animation_warping_library_get_offset_root_transform,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(16).cast::<crate::bindings::core_u_object::FTransform>().read()
        }
    }
    pub fn get_float_value_from_curve(
        in_curve: UPtr<crate::bindings::engine::UCurveFloat>,
        time: f32,
        out_value: &mut f32,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<17>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::animation_warping_runtime::__FUNCTION_PTRS
                    .u_animation_warping_library_get_float_value_from_curve,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_curve,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::UCurveFloat>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&time, __buffer.add(8).cast::<f32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(out_value, __buffer.add(12).cast::<f32>(), 1);
        }
        let __object_ptr = crate::bindings::animation_warping_runtime::UAnimationWarpingLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::animation_warping_runtime::__FUNCTION_PTRS
                    .u_animation_warping_library_get_float_value_from_curve,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(12).cast::<f32>().swap(out_value);
        }
        unsafe { __buffer.add(16).cast::<bool>().read() }
    }
    pub fn get_curve_value_from_animation(
        animation: UPtr<crate::bindings::engine::UAnimSequenceBase>,
        curve_name: FName,
        time: f32,
        out_value: &mut f32,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<29>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::animation_warping_runtime::__FUNCTION_PTRS
                    .u_animation_warping_library_get_curve_value_from_animation,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &animation,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::engine::UAnimSequenceBase>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &curve_name,
                __buffer.add(8).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&time, __buffer.add(20).cast::<f32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(out_value, __buffer.add(24).cast::<f32>(), 1);
        }
        let __object_ptr = crate::bindings::animation_warping_runtime::UAnimationWarpingLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::animation_warping_runtime::__FUNCTION_PTRS
                    .u_animation_warping_library_get_curve_value_from_animation,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(24).cast::<f32>().swap(out_value);
        }
        unsafe { __buffer.add(28).cast::<bool>().read() }
    }
}
#[repr(transparent)]
pub struct EPelvisHeightMode(pub u8);
impl EPelvisHeightMode {
    pub const ALL_LEGS: EPelvisHeightMode = EPelvisHeightMode(0);
    pub const ALL_PLANTED_FEET: EPelvisHeightMode = EPelvisHeightMode(1);
    pub const FRONT_PLANTED_FEET_UPHILL_FRONT_FEET_DOWNHILL: EPelvisHeightMode = EPelvisHeightMode(
        2,
    );
}
#[repr(transparent)]
pub struct EActorMovementCompensationMode(pub u8);
impl EActorMovementCompensationMode {
    pub const COMPONENT_SPACE: EActorMovementCompensationMode = EActorMovementCompensationMode(
        0,
    );
    pub const WORLD_SPACE: EActorMovementCompensationMode = EActorMovementCompensationMode(
        1,
    );
    pub const SUDDEN_MOTION_ONLY: EActorMovementCompensationMode = EActorMovementCompensationMode(
        2,
    );
}
#[repr(transparent)]
pub struct EFootPlacementLockType(pub u8);
impl EFootPlacementLockType {
    pub const UNLOCKED: EFootPlacementLockType = EFootPlacementLockType(0);
    pub const PIVOT_AROUND_BALL: EFootPlacementLockType = EFootPlacementLockType(1);
    pub const PIVOT_AROUND_ANKLE: EFootPlacementLockType = EFootPlacementLockType(2);
    pub const LOCK_ROTATION: EFootPlacementLockType = EFootPlacementLockType(3);
}
#[repr(transparent)]
pub struct EOffsetRootBoneMode(pub u8);
impl EOffsetRootBoneMode {
    pub const ACCUMULATE: EOffsetRootBoneMode = EOffsetRootBoneMode(0);
    pub const INTERPOLATE: EOffsetRootBoneMode = EOffsetRootBoneMode(1);
    pub const LOCK_OFFSET_AND_CONSUME_ANIMATION: EOffsetRootBoneMode = EOffsetRootBoneMode(
        2,
    );
    pub const LOCK_OFFSET_INCREASE_AND_CONSUME_ANIMATION: EOffsetRootBoneMode = EOffsetRootBoneMode(
        3,
    );
    pub const LOCK_OFFSET_AND_IGNORE_ANIMATION: EOffsetRootBoneMode = EOffsetRootBoneMode(
        4,
    );
    pub const RELEASE: EOffsetRootBoneMode = EOffsetRootBoneMode(5);
}
#[repr(transparent)]
pub struct EOffsetRootBone_CollisionTestingMode(pub u8);
impl EOffsetRootBone_CollisionTestingMode {
    pub const DISABLED: EOffsetRootBone_CollisionTestingMode = EOffsetRootBone_CollisionTestingMode(
        0,
    );
    pub const SHRINK_MAX_TRANSLATION: EOffsetRootBone_CollisionTestingMode = EOffsetRootBone_CollisionTestingMode(
        1,
    );
    pub const PLANAR_COLLISION: EOffsetRootBone_CollisionTestingMode = EOffsetRootBone_CollisionTestingMode(
        2,
    );
}
#[repr(transparent)]
pub struct EOrientationWarpingSpace(pub u8);
impl EOrientationWarpingSpace {
    pub const COMPONENT_TRANSFORM: EOrientationWarpingSpace = EOrientationWarpingSpace(
        0,
    );
    pub const ROOT_BONE_TRANSFORM: EOrientationWarpingSpace = EOrientationWarpingSpace(
        1,
    );
    pub const CUSTOM_TRANSFORM: EOrientationWarpingSpace = EOrientationWarpingSpace(2);
}
