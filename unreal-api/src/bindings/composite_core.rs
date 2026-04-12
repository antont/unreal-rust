#![allow(clippy::all)]
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
#![allow(forgetting_copy_types)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::new_without_default)]
#![allow(clippy::new_ret_no_self)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[doc(hidden)]
pub static mut __FUNCTION_PTRS: FunctionPtrs = FunctionPtrs::empty();
pub struct FunctionPtrs {
    pub u_composite_core_subsystem_unregister_primitive: *mut crate::ffi::UFunctionOpague,
    pub u_composite_core_subsystem_register_primitive: *mut crate::ffi::UFunctionOpague,
    pub u_holdout_composite_component_set_enabled: *mut crate::ffi::UFunctionOpague,
    pub u_holdout_composite_component_is_enabled: *mut crate::ffi::UFunctionOpague,
}
impl FunctionPtrs {
    pub const fn empty() -> Self {
        Self {
            u_composite_core_subsystem_unregister_primitive: std::ptr::null_mut(),
            u_composite_core_subsystem_register_primitive: std::ptr::null_mut(),
            u_holdout_composite_component_set_enabled: std::ptr::null_mut(),
            u_holdout_composite_component_is_enabled: std::ptr::null_mut(),
        }
    }
}
pub fn initialize() {
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UCompositeCoreSubsystem::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("UnregisterPrimitive"),
                &raw mut __FUNCTION_PTRS.u_composite_core_subsystem_unregister_primitive,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("RegisterPrimitive"),
                &raw mut __FUNCTION_PTRS.u_composite_core_subsystem_register_primitive,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UHoldoutCompositeComponent::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetEnabled"),
                &raw mut __FUNCTION_PTRS.u_holdout_composite_component_set_enabled,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("IsEnabled"),
                &raw mut __FUNCTION_PTRS.u_holdout_composite_component_is_enabled,
            );
        }
    }
}
#[repr(C, align(8))]
pub struct UCompositeCorePluginSettings {
    __padding_end: [u8; 240],
}
impl UCompositeCorePluginSettings {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UCompositeCorePluginSettings")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UCompositeCorePluginSettings")
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
pub struct UCompositeCoreSubsystem {
    __padding_end: [u8; 112],
}
impl UCompositeCoreSubsystem {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UCompositeCoreSubsystem")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UCompositeCoreSubsystem")
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
                crate::bindings::composite_core::__FUNCTION_PTRS
                    .u_composite_core_subsystem_unregister_primitive,
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
                crate::bindings::composite_core::__FUNCTION_PTRS
                    .u_composite_core_subsystem_unregister_primitive,
                __buffer,
            )
        };
        std::mem::forget(in_primitive_component);
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
                crate::bindings::composite_core::__FUNCTION_PTRS
                    .u_composite_core_subsystem_register_primitive,
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
                crate::bindings::composite_core::__FUNCTION_PTRS
                    .u_composite_core_subsystem_register_primitive,
                __buffer,
            )
        };
        std::mem::forget(in_primitive_component);
    }
}
#[repr(C, align(16))]
pub struct UHoldoutCompositeComponent {
    #[doc(hidden)]
    pub(crate) __padding_656: [u8; 656],
    pub b_is_enabled: bool,
}
impl UHoldoutCompositeComponent {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UHoldoutCompositeComponent")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UHoldoutCompositeComponent")
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
    pub fn set_enabled(&mut self, b_in_enabled: bool) {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::composite_core::__FUNCTION_PTRS
                    .u_holdout_composite_component_set_enabled,
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
                crate::bindings::composite_core::__FUNCTION_PTRS
                    .u_holdout_composite_component_set_enabled,
                __buffer,
            )
        };
        std::mem::forget(b_in_enabled);
    }
    pub fn is_enabled(&self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::composite_core::__FUNCTION_PTRS
                    .u_holdout_composite_component_is_enabled,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::composite_core::__FUNCTION_PTRS
                    .u_holdout_composite_component_is_enabled,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
}
