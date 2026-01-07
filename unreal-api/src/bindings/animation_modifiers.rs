#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
#![allow(clippy::non_camel_case_types)]
#![allow(clippy::new_without_default)]
#![allow(clippy::new_ret_no_self)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[doc(hidden)]
pub static mut U_ANIMATION_MODIFIER_ON_REVERT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_ANIMATION_MODIFIER_ON_APPLY: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_ANIMATION_MODIFIERS_ASSET_USER_DATA_ADD_ANIMATION_MODIFIER_OF_CLASS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
pub fn initialize() {
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UAnimationModifier::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("OnRevert"),
            &raw mut U_ANIMATION_MODIFIER_ON_REVERT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("OnApply"),
            &raw mut U_ANIMATION_MODIFIER_ON_APPLY,
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
            &raw mut U_ANIMATION_MODIFIERS_ASSET_USER_DATA_ADD_ANIMATION_MODIFIER_OF_CLASS,
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
}
#[repr(C, align(8))]
pub struct UAnimationModifiersAssetUserData {
    #[doc(hidden)]
    __padding_48: [u8; 48],
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
                crate::bindings::animation_modifiers::U_ANIMATION_MODIFIERS_ASSET_USER_DATA_ADD_ANIMATION_MODIFIER_OF_CLASS,
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
                crate::bindings::animation_modifiers::U_ANIMATION_MODIFIERS_ASSET_USER_DATA_ADD_ANIMATION_MODIFIER_OF_CLASS,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<bool>().read() }
    }
}
