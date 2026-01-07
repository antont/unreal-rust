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
pub static mut U_COMPOSITE_CORE_SUBSYSTEM_UNREGISTER_PRIMITIVE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_COMPOSITE_CORE_SUBSYSTEM_REGISTER_PRIMITIVE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_HOLDOUT_COMPOSITE_COMPONENT_SET_ENABLED: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_HOLDOUT_COMPOSITE_COMPONENT_IS_ENABLED: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
pub fn initialize() {
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UCompositeCoreSubsystem::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("UnregisterPrimitive"),
            &raw mut U_COMPOSITE_CORE_SUBSYSTEM_UNREGISTER_PRIMITIVE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RegisterPrimitive"),
            &raw mut U_COMPOSITE_CORE_SUBSYSTEM_REGISTER_PRIMITIVE,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UHoldoutCompositeComponent::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetEnabled"),
            &raw mut U_HOLDOUT_COMPOSITE_COMPONENT_SET_ENABLED,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("IsEnabled"),
            &raw mut U_HOLDOUT_COMPOSITE_COMPONENT_IS_ENABLED,
        );
    }
}
#[repr(C, align(8))]
pub struct UCompositeCorePluginSettings {
    __padding_end: [u8; 240],
}
impl UCompositeCorePluginSettings {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UCompositeCorePluginSettings")
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
pub struct UCompositeCoreSubsystem {
    __padding_end: [u8; 112],
}
impl UCompositeCoreSubsystem {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UCompositeCoreSubsystem")
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
    pub fn unregister_primitive(
        &mut self,
        in_primitive_component: UPtr<crate::bindings::engine::UPrimitiveComponent>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::composite_core::U_COMPOSITE_CORE_SUBSYSTEM_UNREGISTER_PRIMITIVE,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_primitive_component,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::engine::UPrimitiveComponent>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::composite_core::U_COMPOSITE_CORE_SUBSYSTEM_UNREGISTER_PRIMITIVE,
                __buffer,
            )
        };
    }
    pub fn register_primitive(
        &mut self,
        in_primitive_component: UPtr<crate::bindings::engine::UPrimitiveComponent>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::composite_core::U_COMPOSITE_CORE_SUBSYSTEM_REGISTER_PRIMITIVE,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_primitive_component,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::engine::UPrimitiveComponent>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::composite_core::U_COMPOSITE_CORE_SUBSYSTEM_REGISTER_PRIMITIVE,
                __buffer,
            )
        };
    }
}
#[repr(C, align(16))]
pub struct UHoldoutCompositeComponent {
    #[doc(hidden)]
    __padding_656: [u8; 656],
    pub b_is_enabled: bool,
}
impl UHoldoutCompositeComponent {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UHoldoutCompositeComponent")
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
    pub fn set_enabled(&mut self, b_in_enabled: bool) {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::composite_core::U_HOLDOUT_COMPOSITE_COMPONENT_SET_ENABLED,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_in_enabled,
                __buffer.add(0).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::composite_core::U_HOLDOUT_COMPOSITE_COMPONENT_SET_ENABLED,
                __buffer,
            )
        };
    }
    pub fn is_enabled(&self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::composite_core::U_HOLDOUT_COMPOSITE_COMPONENT_IS_ENABLED,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::composite_core::U_HOLDOUT_COMPOSITE_COMPONENT_IS_ENABLED,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
}
