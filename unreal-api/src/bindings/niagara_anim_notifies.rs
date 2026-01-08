#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(clippy::non_camel_case_types)]
#![allow(clippy::new_without_default)]
#![allow(clippy::new_ret_no_self)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[doc(hidden)]
pub static mut __FUNCTION_PTRS: FunctionPtrs = FunctionPtrs::empty();
pub struct FunctionPtrs {
    pub u_anim_notify_state_timed_niagara_effect_get_spawned_effect: *mut crate::ffi::UFunctionOpague,
    pub u_anim_notify_state_timed_niagara_effect_advanced_get_notify_progress: *mut crate::ffi::UFunctionOpague,
    pub u_anim_notify_play_niagara_effect_get_spawned_effect: *mut crate::ffi::UFunctionOpague,
}
impl FunctionPtrs {
    pub const fn empty() -> Self {
        Self {
            u_anim_notify_state_timed_niagara_effect_get_spawned_effect: std::ptr::null_mut(),
            u_anim_notify_state_timed_niagara_effect_advanced_get_notify_progress: std::ptr::null_mut(),
            u_anim_notify_play_niagara_effect_get_spawned_effect: std::ptr::null_mut(),
        }
    }
}
pub fn initialize() {
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UAnimNotifyState_TimedNiagaraEffect::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetSpawnedEffect"),
            &raw mut __FUNCTION_PTRS
                .u_anim_notify_state_timed_niagara_effect_get_spawned_effect,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UAnimNotifyState_TimedNiagaraEffectAdvanced::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetNotifyProgress"),
            &raw mut __FUNCTION_PTRS
                .u_anim_notify_state_timed_niagara_effect_advanced_get_notify_progress,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UAnimNotify_PlayNiagaraEffect::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetSpawnedEffect"),
            &raw mut __FUNCTION_PTRS.u_anim_notify_play_niagara_effect_get_spawned_effect,
        );
    }
}
#[repr(C, align(4))]
pub struct FCurveParameterPair {
    pub anim_curve_name: FName,
    pub user_variable_name: FName,
}
impl FCurveParameterPair {}
#[repr(C, align(8))]
pub struct UAnimNotifyState_TimedNiagaraEffect {
    #[doc(hidden)]
    __padding_56: [u8; 56],
    pub template: UPtr<crate::bindings::niagara::UNiagaraSystem>,
    pub socket_name: FName,
    pub location_offset: crate::bindings::core_u_object::FVector,
    pub rotation_offset: crate::bindings::core_u_object::FRotator,
    #[doc(hidden)]
    __padding_152: [u8; 24],
    pub b_apply_rate_scale_as_time_dilation: bool,
    pub b_destroy_at_end: bool,
}
impl UAnimNotifyState_TimedNiagaraEffect {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAnimNotifyState_TimedNiagaraEffect")
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
    pub fn get_spawned_effect(
        &self,
        mesh_comp: UPtr<crate::bindings::engine::UMeshComponent>,
    ) -> UPtr<crate::bindings::engine::UFXSystemComponent> {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::niagara_anim_notifies::__FUNCTION_PTRS
                    .u_anim_notify_state_timed_niagara_effect_get_spawned_effect,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &mesh_comp,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::UMeshComponent>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::niagara_anim_notifies::__FUNCTION_PTRS
                    .u_anim_notify_state_timed_niagara_effect_get_spawned_effect,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(8)
                .cast::<UPtr<crate::bindings::engine::UFXSystemComponent>>()
                .read()
        }
    }
}
#[repr(C, align(8))]
pub struct UAnimNotifyState_TimedNiagaraEffectAdvanced {
    #[doc(hidden)]
    __padding_161: [u8; 161],
    pub b_apply_rate_scale_to_progress: bool,
    __padding_end: [u8; 118],
}
impl UAnimNotifyState_TimedNiagaraEffectAdvanced {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAnimNotifyState_TimedNiagaraEffectAdvanced")
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
    pub fn get_notify_progress(
        &self,
        mesh_comp: UPtr<crate::bindings::engine::UMeshComponent>,
    ) -> f32 {
        let mut __stack = crate::core_data::StackAlloc::<12>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::niagara_anim_notifies::__FUNCTION_PTRS
                    .u_anim_notify_state_timed_niagara_effect_advanced_get_notify_progress,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &mesh_comp,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::UMeshComponent>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::niagara_anim_notifies::__FUNCTION_PTRS
                    .u_anim_notify_state_timed_niagara_effect_advanced_get_notify_progress,
                __buffer,
            )
        };
        unsafe { __buffer.add(8).cast::<f32>().read() }
    }
}
#[repr(C, align(16))]
pub struct UAnimNotify_PlayNiagaraEffect {
    #[doc(hidden)]
    __padding_64: [u8; 64],
    pub template: UPtr<crate::bindings::niagara::UNiagaraSystem>,
    pub location_offset: crate::bindings::core_u_object::FVector,
    pub rotation_offset: crate::bindings::core_u_object::FRotator,
    #[doc(hidden)]
    __padding_192: [u8; 72],
    pub flags_192: u8,
    pub socket_name: FName,
}
impl UAnimNotify_PlayNiagaraEffect {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAnimNotify_PlayNiagaraEffect")
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
    pub fn get_spawned_effect(
        &self,
    ) -> UPtr<crate::bindings::engine::UFXSystemComponent> {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::niagara_anim_notifies::__FUNCTION_PTRS
                    .u_anim_notify_play_niagara_effect_get_spawned_effect,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::niagara_anim_notifies::__FUNCTION_PTRS
                    .u_anim_notify_play_niagara_effect_get_spawned_effect,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(0)
                .cast::<UPtr<crate::bindings::engine::UFXSystemComponent>>()
                .read()
        }
    }
}
#[repr(transparent)]
pub struct ENiagaraAnimNotifyProgressType(pub u8);
impl ENiagaraAnimNotifyProgressType {
    pub const NONE: ENiagaraAnimNotifyProgressType = ENiagaraAnimNotifyProgressType(0);
    pub const FORWARD: ENiagaraAnimNotifyProgressType = ENiagaraAnimNotifyProgressType(
        1,
    );
    pub const REVERSE: ENiagaraAnimNotifyProgressType = ENiagaraAnimNotifyProgressType(
        2,
    );
}
