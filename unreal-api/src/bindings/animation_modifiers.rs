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
    pub u_animation_modifier_on_revert: *mut crate::ffi::UFunctionOpague,
    pub u_animation_modifier_on_apply: *mut crate::ffi::UFunctionOpague,
    pub u_animation_modifiers_asset_user_data_add_animation_modifier_of_class: *mut crate::ffi::UFunctionOpague,
}
impl FunctionPtrs {
    pub const fn empty() -> Self {
        Self {
            u_animation_modifier_on_revert: std::ptr::null_mut(),
            u_animation_modifier_on_apply: std::ptr::null_mut(),
            u_animation_modifiers_asset_user_data_add_animation_modifier_of_class: std::ptr::null_mut(),
        }
    }
}
pub fn initialize() {
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UAnimationModifier::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("OnRevert"),
            &raw mut __FUNCTION_PTRS.u_animation_modifier_on_revert,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("OnApply"),
            &raw mut __FUNCTION_PTRS.u_animation_modifier_on_apply,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UAnimationModifiersAssetUserData::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AddAnimationModifierOfClass"),
            &raw mut __FUNCTION_PTRS
                .u_animation_modifiers_asset_user_data_add_animation_modifier_of_class,
        );
    }
}
#[repr(C, align(8))]
pub struct UAnimationModifierSettings {
    __padding_end: [u8; 128],
}
impl UAnimationModifierSettings {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAnimationModifierSettings")
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
pub struct UAnimationModifier {
    __padding_end: [u8; 120],
}
impl UAnimationModifier {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAnimationModifier")
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
    pub fn on_revert(
        &mut self,
        animation_sequence: UPtr<crate::bindings::engine::UAnimSequence>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::animation_modifiers::__FUNCTION_PTRS
                    .u_animation_modifier_on_revert,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &animation_sequence,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::UAnimSequence>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::animation_modifiers::__FUNCTION_PTRS
                    .u_animation_modifier_on_revert,
                __buffer,
            )
        };
    }
    pub fn on_apply(
        &mut self,
        animation_sequence: UPtr<crate::bindings::engine::UAnimSequence>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::animation_modifiers::__FUNCTION_PTRS
                    .u_animation_modifier_on_apply,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &animation_sequence,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::UAnimSequence>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::animation_modifiers::__FUNCTION_PTRS
                    .u_animation_modifier_on_apply,
                __buffer,
            )
        };
    }
}
#[repr(C, align(8))]
pub struct UAnimationModifiersAssetUserData {
    #[doc(hidden)]
    pub(crate) __padding_48: [u8; 48],
    pub animation_modifier_instances: TArray<UPtr<UAnimationModifier>>,
    pub applied_modifiers: TMap<
        crate::bindings::core_u_object::FSoftObjectPath,
        UPtr<UAnimationModifier>,
    >,
}
impl UAnimationModifiersAssetUserData {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAnimationModifiersAssetUserData")
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
    pub fn add_animation_modifier_of_class(
        animation_sequence_base: UPtr<crate::bindings::engine::UAnimSequenceBase>,
        in_modifier_class: TSubclassOf<UAnimationModifier>,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<17>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::animation_modifiers::__FUNCTION_PTRS
                    .u_animation_modifiers_asset_user_data_add_animation_modifier_of_class,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &animation_sequence_base,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::engine::UAnimSequenceBase>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_modifier_class,
                __buffer.add(8).cast::<TSubclassOf<UAnimationModifier>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::animation_modifiers::UAnimationModifiersAssetUserData::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::animation_modifiers::__FUNCTION_PTRS
                    .u_animation_modifiers_asset_user_data_add_animation_modifier_of_class,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<bool>().read() }
    }
}
