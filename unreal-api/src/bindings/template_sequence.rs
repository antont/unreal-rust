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
    pub a_template_sequence_actor_set_sequence: *mut crate::ffi::UFunctionOpague,
    pub a_template_sequence_actor_set_binding: *mut crate::ffi::UFunctionOpague,
    pub a_template_sequence_actor_load_sequence: *mut crate::ffi::UFunctionOpague,
    pub a_template_sequence_actor_get_sequence_player: *mut crate::ffi::UFunctionOpague,
    pub a_template_sequence_actor_get_sequence: *mut crate::ffi::UFunctionOpague,
    pub u_template_sequence_player_create_template_sequence_player: *mut crate::ffi::UFunctionOpague,
    pub u_sequence_camera_shake_test_util_get_post_process_blend_cache: *mut crate::ffi::UFunctionOpague,
    pub u_sequence_camera_shake_test_util_get_last_frame_camera_cache_pov: *mut crate::ffi::UFunctionOpague,
    pub u_sequence_camera_shake_test_util_get_camera_cache_pov: *mut crate::ffi::UFunctionOpague,
}
impl FunctionPtrs {
    pub const fn empty() -> Self {
        Self {
            a_template_sequence_actor_set_sequence: std::ptr::null_mut(),
            a_template_sequence_actor_set_binding: std::ptr::null_mut(),
            a_template_sequence_actor_load_sequence: std::ptr::null_mut(),
            a_template_sequence_actor_get_sequence_player: std::ptr::null_mut(),
            a_template_sequence_actor_get_sequence: std::ptr::null_mut(),
            u_template_sequence_player_create_template_sequence_player: std::ptr::null_mut(),
            u_sequence_camera_shake_test_util_get_post_process_blend_cache: std::ptr::null_mut(),
            u_sequence_camera_shake_test_util_get_last_frame_camera_cache_pov: std::ptr::null_mut(),
            u_sequence_camera_shake_test_util_get_camera_cache_pov: std::ptr::null_mut(),
        }
    }
}
pub fn initialize() {
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = ATemplateSequenceActor::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetSequence"),
                &raw mut __FUNCTION_PTRS.a_template_sequence_actor_set_sequence,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetBinding"),
                &raw mut __FUNCTION_PTRS.a_template_sequence_actor_set_binding,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("LoadSequence"),
                &raw mut __FUNCTION_PTRS.a_template_sequence_actor_load_sequence,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetSequencePlayer"),
                &raw mut __FUNCTION_PTRS.a_template_sequence_actor_get_sequence_player,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetSequence"),
                &raw mut __FUNCTION_PTRS.a_template_sequence_actor_get_sequence,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UTemplateSequencePlayer::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("CreateTemplateSequencePlayer"),
                &raw mut __FUNCTION_PTRS
                    .u_template_sequence_player_create_template_sequence_player,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = USequenceCameraShakeTestUtil::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetPostProcessBlendCache"),
                &raw mut __FUNCTION_PTRS
                    .u_sequence_camera_shake_test_util_get_post_process_blend_cache,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetLastFrameCameraCachePOV"),
                &raw mut __FUNCTION_PTRS
                    .u_sequence_camera_shake_test_util_get_last_frame_camera_cache_pov,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetCameraCachePOV"),
                &raw mut __FUNCTION_PTRS
                    .u_sequence_camera_shake_test_util_get_camera_cache_pov,
            );
        }
    }
}
#[repr(C, align(4))]
pub struct FTemplateSequenceBindingOverrideData {
    pub(crate) __padding_end: [u8; 12],
}
impl FTemplateSequenceBindingOverrideData {}
#[repr(C, align(8))]
pub struct UTemplateSequence {
    __padding_end: [u8; 264],
}
impl UTemplateSequence {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UTemplateSequence")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UTemplateSequence")
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
}
#[repr(C, align(8))]
pub struct UCameraAnimationSequence {
    __padding_end: [u8; 264],
}
impl UCameraAnimationSequence {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UCameraAnimationSequence")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UCameraAnimationSequence")
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
}
#[repr(C, align(16))]
pub struct UCameraAnimationSequenceCameraStandIn {
    __padding_end: [u8; 2320],
}
impl UCameraAnimationSequenceCameraStandIn {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UCameraAnimationSequenceCameraStandIn")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UCameraAnimationSequenceCameraStandIn")
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
}
#[repr(C, align(8))]
pub struct UCameraAnimationSequencePlayer {
    __padding_end: [u8; 824],
}
impl UCameraAnimationSequencePlayer {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UCameraAnimationSequencePlayer")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UCameraAnimationSequencePlayer")
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
}
#[repr(C, align(8))]
pub struct UCameraAnimationSpawnableSystem {
    __padding_end: [u8; 80],
}
impl UCameraAnimationSpawnableSystem {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UCameraAnimationSpawnableSystem")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UCameraAnimationSpawnableSystem")
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
}
#[repr(C, align(8))]
pub struct UCameraAnimationBoundObjectInstantiator {
    __padding_end: [u8; 80],
}
impl UCameraAnimationBoundObjectInstantiator {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UCameraAnimationBoundObjectInstantiator")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UCameraAnimationBoundObjectInstantiator")
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
}
#[repr(C, align(8))]
pub struct UCameraAnimationEntitySystemLinker {
    __padding_end: [u8; 1952],
}
impl UCameraAnimationEntitySystemLinker {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UCameraAnimationEntitySystemLinker")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UCameraAnimationEntitySystemLinker")
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
}
#[repr(C, align(8))]
pub struct UCameraAnimationSequenceSubsystem {
    __padding_end: [u8; 88],
}
impl UCameraAnimationSequenceSubsystem {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UCameraAnimationSequenceSubsystem")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UCameraAnimationSequenceSubsystem")
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
}
#[repr(C, align(8))]
pub struct UTemplateSequenceSection {
    __padding_end: [u8; 2440],
}
impl UTemplateSequenceSection {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UTemplateSequenceSection")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UTemplateSequenceSection")
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
}
#[repr(C, align(8))]
pub struct USequenceCameraShakePattern {
    __padding_end: [u8; 128],
}
impl USequenceCameraShakePattern {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("USequenceCameraShakePattern")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("USequenceCameraShakePattern")
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
}
#[repr(C, align(8))]
pub struct UTemplateSequenceSystem {
    __padding_end: [u8; 224],
}
impl UTemplateSequenceSystem {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UTemplateSequenceSystem")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UTemplateSequenceSystem")
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
}
#[repr(C, align(8))]
pub struct UTemplateSequencePropertyScalingInstantiatorSystem {
    __padding_end: [u8; 168],
}
impl UTemplateSequencePropertyScalingInstantiatorSystem {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UTemplateSequencePropertyScalingInstantiatorSystem")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UTemplateSequencePropertyScalingInstantiatorSystem")
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
}
#[repr(C, align(8))]
pub struct UTemplateSequencePropertyScalingEvaluatorSystem {
    __padding_end: [u8; 160],
}
impl UTemplateSequencePropertyScalingEvaluatorSystem {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UTemplateSequencePropertyScalingEvaluatorSystem")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UTemplateSequencePropertyScalingEvaluatorSystem")
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
}
#[repr(C, align(8))]
pub struct ATemplateSequenceActor {
    #[doc(hidden)]
    pub(crate) __padding_1144: [u8; 1144],
    pub playback_settings: crate::bindings::movie_scene::FMovieSceneSequencePlaybackSettings,
    pub sequence_player: UPtr<UTemplateSequencePlayer>,
    pub template_sequence: crate::bindings::core_u_object::FSoftObjectPath,
    pub binding_override: FTemplateSequenceBindingOverrideData,
}
impl ATemplateSequenceActor {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("ATemplateSequenceActor")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("ATemplateSequenceActor")
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
    pub fn set_sequence(&mut self, in_sequence: UPtr<UTemplateSequence>) {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::template_sequence::__FUNCTION_PTRS
                    .a_template_sequence_actor_set_sequence,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_sequence,
                __buffer.add(0).cast::<UPtr<UTemplateSequence>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::template_sequence::__FUNCTION_PTRS
                    .a_template_sequence_actor_set_sequence,
                __buffer,
            )
        };
    }
    pub fn set_binding(
        &mut self,
        actor: UPtr<crate::bindings::engine::AActor>,
        b_overrides_default: bool,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<9>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::template_sequence::__FUNCTION_PTRS
                    .a_template_sequence_actor_set_binding,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &actor,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::AActor>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_overrides_default,
                __buffer.add(8).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::template_sequence::__FUNCTION_PTRS
                    .a_template_sequence_actor_set_binding,
                __buffer,
            )
        };
    }
    pub fn load_sequence(&self) -> UPtr<UTemplateSequence> {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::template_sequence::__FUNCTION_PTRS
                    .a_template_sequence_actor_load_sequence,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::template_sequence::__FUNCTION_PTRS
                    .a_template_sequence_actor_load_sequence,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<UPtr<UTemplateSequence>>().read() }
    }
    pub fn get_sequence_player(&self) -> UPtr<UTemplateSequencePlayer> {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::template_sequence::__FUNCTION_PTRS
                    .a_template_sequence_actor_get_sequence_player,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::template_sequence::__FUNCTION_PTRS
                    .a_template_sequence_actor_get_sequence_player,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<UPtr<UTemplateSequencePlayer>>().read() }
    }
    pub fn get_sequence(&self) -> UPtr<UTemplateSequence> {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::template_sequence::__FUNCTION_PTRS
                    .a_template_sequence_actor_get_sequence,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::template_sequence::__FUNCTION_PTRS
                    .a_template_sequence_actor_get_sequence,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<UPtr<UTemplateSequence>>().read() }
    }
}
#[repr(C, align(8))]
pub struct UTemplateSequencePlayer {
    __padding_end: [u8; 1224],
}
impl UTemplateSequencePlayer {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UTemplateSequencePlayer")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UTemplateSequencePlayer")
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
    pub fn create_template_sequence_player(
        world_context_object: UPtr<crate::bindings::core_u_object::UObject>,
        template_sequence: UPtr<UTemplateSequence>,
        settings: crate::bindings::movie_scene::FMovieSceneSequencePlaybackSettings,
        out_actor: &mut UPtr<ATemplateSequenceActor>,
    ) -> UPtr<UTemplateSequencePlayer> {
        let mut __stack = crate::core_data::StackAlloc::<72>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::template_sequence::__FUNCTION_PTRS
                    .u_template_sequence_player_create_template_sequence_player,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &world_context_object,
                __buffer.add(0).cast::<UPtr<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &template_sequence,
                __buffer.add(8).cast::<UPtr<UTemplateSequence>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &settings,
                __buffer
                    .add(16)
                    .cast::<
                        crate::bindings::movie_scene::FMovieSceneSequencePlaybackSettings,
                    >(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_actor,
                __buffer.add(56).cast::<UPtr<ATemplateSequenceActor>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::template_sequence::UTemplateSequencePlayer::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::template_sequence::__FUNCTION_PTRS
                    .u_template_sequence_player_create_template_sequence_player,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(56).cast::<UPtr<ATemplateSequenceActor>>().swap(out_actor);
        }
        unsafe { __buffer.add(64).cast::<UPtr<UTemplateSequencePlayer>>().read() }
    }
}
#[repr(C, align(8))]
pub struct USequenceCameraShakeTestUtil {
    __padding_end: [u8; 48],
}
impl USequenceCameraShakeTestUtil {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("USequenceCameraShakeTestUtil")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("USequenceCameraShakeTestUtil")
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
    pub fn get_post_process_blend_cache(
        player_controller: UPtr<crate::bindings::engine::APlayerController>,
        pp_index: i32,
        out_pp_settings: &mut crate::bindings::engine::FPostProcessSettings,
        out_pp_blend_weight: &mut f32,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1973>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::template_sequence::__FUNCTION_PTRS
                    .u_sequence_camera_shake_test_util_get_post_process_blend_cache,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &player_controller,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::engine::APlayerController>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&pp_index, __buffer.add(8).cast::<i32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_pp_settings,
                __buffer.add(16).cast::<crate::bindings::engine::FPostProcessSettings>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_pp_blend_weight,
                __buffer.add(1968).cast::<f32>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::template_sequence::USequenceCameraShakeTestUtil::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::template_sequence::__FUNCTION_PTRS
                    .u_sequence_camera_shake_test_util_get_post_process_blend_cache,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(16)
                .cast::<crate::bindings::engine::FPostProcessSettings>()
                .swap(out_pp_settings);
        }
        unsafe {
            __buffer.add(1968).cast::<f32>().swap(out_pp_blend_weight);
        }
        unsafe { __buffer.add(1972).cast::<bool>().read() }
    }
    pub fn get_last_frame_camera_cache_pov(
        player_controller: UPtr<crate::bindings::engine::APlayerController>,
    ) -> crate::bindings::engine::FMinimalViewInfo {
        let mut __stack = crate::core_data::StackAlloc::<2320>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::template_sequence::__FUNCTION_PTRS
                    .u_sequence_camera_shake_test_util_get_last_frame_camera_cache_pov,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &player_controller,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::engine::APlayerController>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::template_sequence::USequenceCameraShakeTestUtil::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::template_sequence::__FUNCTION_PTRS
                    .u_sequence_camera_shake_test_util_get_last_frame_camera_cache_pov,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(16).cast::<crate::bindings::engine::FMinimalViewInfo>().read()
        }
    }
    pub fn get_camera_cache_pov(
        player_controller: UPtr<crate::bindings::engine::APlayerController>,
    ) -> crate::bindings::engine::FMinimalViewInfo {
        let mut __stack = crate::core_data::StackAlloc::<2320>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::template_sequence::__FUNCTION_PTRS
                    .u_sequence_camera_shake_test_util_get_camera_cache_pov,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &player_controller,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::engine::APlayerController>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::template_sequence::USequenceCameraShakeTestUtil::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::template_sequence::__FUNCTION_PTRS
                    .u_sequence_camera_shake_test_util_get_camera_cache_pov,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(16).cast::<crate::bindings::engine::FMinimalViewInfo>().read()
        }
    }
}
#[repr(C, align(8))]
pub struct UTemplateSequenceTrack {
    __padding_end: [u8; 416],
}
impl UTemplateSequenceTrack {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UTemplateSequenceTrack")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UTemplateSequenceTrack")
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
}
#[repr(transparent)]
pub struct ETemplateSectionPropertyScaleType(pub i32);
impl ETemplateSectionPropertyScaleType {
    pub const FLOAT_PROPERTY: ETemplateSectionPropertyScaleType = ETemplateSectionPropertyScaleType(
        0,
    );
    pub const TRANSFORM_PROPERTY_LOCATION_ONLY: ETemplateSectionPropertyScaleType = ETemplateSectionPropertyScaleType(
        1,
    );
    pub const TRANSFORM_PROPERTY_ROTATION_ONLY: ETemplateSectionPropertyScaleType = ETemplateSectionPropertyScaleType(
        2,
    );
}
