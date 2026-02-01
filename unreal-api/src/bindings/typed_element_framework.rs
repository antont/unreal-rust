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
    pub u_typed_element_handle_library_release: *mut crate::ffi::UFunctionOpague,
    pub u_typed_element_handle_library_not_equal: *mut crate::ffi::UFunctionOpague,
    pub u_typed_element_handle_library_is_set: *mut crate::ffi::UFunctionOpague,
    pub u_typed_element_handle_library_equal: *mut crate::ffi::UFunctionOpague,
    pub u_typed_element_list_library_shrink: *mut crate::ffi::UFunctionOpague,
    pub u_typed_element_list_library_reset: *mut crate::ffi::UFunctionOpague,
    pub u_typed_element_list_library_reserve: *mut crate::ffi::UFunctionOpague,
    pub u_typed_element_list_library_remove: *mut crate::ffi::UFunctionOpague,
    pub u_typed_element_list_library_num: *mut crate::ffi::UFunctionOpague,
    pub u_typed_element_list_library_is_valid_index: *mut crate::ffi::UFunctionOpague,
    pub u_typed_element_list_library_has_elements_of_type: *mut crate::ffi::UFunctionOpague,
    pub u_typed_element_list_library_has_elements: *mut crate::ffi::UFunctionOpague,
    pub u_typed_element_list_library_get_element_interface: *mut crate::ffi::UFunctionOpague,
    pub u_typed_element_list_library_get_element_handles: *mut crate::ffi::UFunctionOpague,
    pub u_typed_element_list_library_get_element_handle_at: *mut crate::ffi::UFunctionOpague,
    pub u_typed_element_list_library_empty: *mut crate::ffi::UFunctionOpague,
    pub u_typed_element_list_library_create_script_element_list: *mut crate::ffi::UFunctionOpague,
    pub u_typed_element_list_library_count_elements_of_type: *mut crate::ffi::UFunctionOpague,
    pub u_typed_element_list_library_count_elements: *mut crate::ffi::UFunctionOpague,
    pub u_typed_element_list_library_contains: *mut crate::ffi::UFunctionOpague,
    pub u_typed_element_list_library_clone: *mut crate::ffi::UFunctionOpague,
    pub u_typed_element_list_library_append_list: *mut crate::ffi::UFunctionOpague,
    pub u_typed_element_list_library_append: *mut crate::ffi::UFunctionOpague,
    pub u_typed_element_list_library_add: *mut crate::ffi::UFunctionOpague,
    pub u_typed_element_registry_get_instance: *mut crate::ffi::UFunctionOpague,
    pub u_typed_element_registry_get_element_interface: *mut crate::ffi::UFunctionOpague,
    pub u_test_typed_element_interface_a_set_display_name: *mut crate::ffi::UFunctionOpague,
    pub u_test_typed_element_interface_a_get_display_name: *mut crate::ffi::UFunctionOpague,
    pub u_test_typed_element_interface_b_mark_as_tested: *mut crate::ffi::UFunctionOpague,
    pub u_test_typed_element_interface_c_get_is_tested: *mut crate::ffi::UFunctionOpague,
}
impl FunctionPtrs {
    pub const fn empty() -> Self {
        Self {
            u_typed_element_handle_library_release: std::ptr::null_mut(),
            u_typed_element_handle_library_not_equal: std::ptr::null_mut(),
            u_typed_element_handle_library_is_set: std::ptr::null_mut(),
            u_typed_element_handle_library_equal: std::ptr::null_mut(),
            u_typed_element_list_library_shrink: std::ptr::null_mut(),
            u_typed_element_list_library_reset: std::ptr::null_mut(),
            u_typed_element_list_library_reserve: std::ptr::null_mut(),
            u_typed_element_list_library_remove: std::ptr::null_mut(),
            u_typed_element_list_library_num: std::ptr::null_mut(),
            u_typed_element_list_library_is_valid_index: std::ptr::null_mut(),
            u_typed_element_list_library_has_elements_of_type: std::ptr::null_mut(),
            u_typed_element_list_library_has_elements: std::ptr::null_mut(),
            u_typed_element_list_library_get_element_interface: std::ptr::null_mut(),
            u_typed_element_list_library_get_element_handles: std::ptr::null_mut(),
            u_typed_element_list_library_get_element_handle_at: std::ptr::null_mut(),
            u_typed_element_list_library_empty: std::ptr::null_mut(),
            u_typed_element_list_library_create_script_element_list: std::ptr::null_mut(),
            u_typed_element_list_library_count_elements_of_type: std::ptr::null_mut(),
            u_typed_element_list_library_count_elements: std::ptr::null_mut(),
            u_typed_element_list_library_contains: std::ptr::null_mut(),
            u_typed_element_list_library_clone: std::ptr::null_mut(),
            u_typed_element_list_library_append_list: std::ptr::null_mut(),
            u_typed_element_list_library_append: std::ptr::null_mut(),
            u_typed_element_list_library_add: std::ptr::null_mut(),
            u_typed_element_registry_get_instance: std::ptr::null_mut(),
            u_typed_element_registry_get_element_interface: std::ptr::null_mut(),
            u_test_typed_element_interface_a_set_display_name: std::ptr::null_mut(),
            u_test_typed_element_interface_a_get_display_name: std::ptr::null_mut(),
            u_test_typed_element_interface_b_mark_as_tested: std::ptr::null_mut(),
            u_test_typed_element_interface_c_get_is_tested: std::ptr::null_mut(),
        }
    }
}
pub fn initialize() {
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UTypedElementHandleLibrary::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("Release"),
                &raw mut __FUNCTION_PTRS.u_typed_element_handle_library_release,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("NotEqual"),
                &raw mut __FUNCTION_PTRS.u_typed_element_handle_library_not_equal,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("IsSet"),
                &raw mut __FUNCTION_PTRS.u_typed_element_handle_library_is_set,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("Equal"),
                &raw mut __FUNCTION_PTRS.u_typed_element_handle_library_equal,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UTypedElementListLibrary::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("Shrink"),
                &raw mut __FUNCTION_PTRS.u_typed_element_list_library_shrink,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("Reset"),
                &raw mut __FUNCTION_PTRS.u_typed_element_list_library_reset,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("Reserve"),
                &raw mut __FUNCTION_PTRS.u_typed_element_list_library_reserve,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("Remove"),
                &raw mut __FUNCTION_PTRS.u_typed_element_list_library_remove,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("Num"),
                &raw mut __FUNCTION_PTRS.u_typed_element_list_library_num,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("IsValidIndex"),
                &raw mut __FUNCTION_PTRS.u_typed_element_list_library_is_valid_index,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("HasElementsOfType"),
                &raw mut __FUNCTION_PTRS
                    .u_typed_element_list_library_has_elements_of_type,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("HasElements"),
                &raw mut __FUNCTION_PTRS.u_typed_element_list_library_has_elements,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetElementInterface"),
                &raw mut __FUNCTION_PTRS
                    .u_typed_element_list_library_get_element_interface,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetElementHandles"),
                &raw mut __FUNCTION_PTRS.u_typed_element_list_library_get_element_handles,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetElementHandleAt"),
                &raw mut __FUNCTION_PTRS
                    .u_typed_element_list_library_get_element_handle_at,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("Empty"),
                &raw mut __FUNCTION_PTRS.u_typed_element_list_library_empty,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("CreateScriptElementList"),
                &raw mut __FUNCTION_PTRS
                    .u_typed_element_list_library_create_script_element_list,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("CountElementsOfType"),
                &raw mut __FUNCTION_PTRS
                    .u_typed_element_list_library_count_elements_of_type,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("CountElements"),
                &raw mut __FUNCTION_PTRS.u_typed_element_list_library_count_elements,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("Contains"),
                &raw mut __FUNCTION_PTRS.u_typed_element_list_library_contains,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("Clone"),
                &raw mut __FUNCTION_PTRS.u_typed_element_list_library_clone,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("AppendList"),
                &raw mut __FUNCTION_PTRS.u_typed_element_list_library_append_list,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("Append"),
                &raw mut __FUNCTION_PTRS.u_typed_element_list_library_append,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("Add"),
                &raw mut __FUNCTION_PTRS.u_typed_element_list_library_add,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UTypedElementRegistry::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetInstance"),
                &raw mut __FUNCTION_PTRS.u_typed_element_registry_get_instance,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetElementInterface"),
                &raw mut __FUNCTION_PTRS.u_typed_element_registry_get_element_interface,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UTestTypedElementInterfaceA::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetDisplayName"),
                &raw mut __FUNCTION_PTRS
                    .u_test_typed_element_interface_a_set_display_name,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetDisplayName"),
                &raw mut __FUNCTION_PTRS
                    .u_test_typed_element_interface_a_get_display_name,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UTestTypedElementInterfaceB::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("MarkAsTested"),
                &raw mut __FUNCTION_PTRS.u_test_typed_element_interface_b_mark_as_tested,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UTestTypedElementInterfaceC::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetIsTested"),
                &raw mut __FUNCTION_PTRS.u_test_typed_element_interface_c_get_is_tested,
            );
        }
    }
}
#[repr(C, align(8))]
pub struct FScriptTypedElementHandle {
    pub(crate) __padding_end: [u8; 8],
}
impl FScriptTypedElementHandle {}
#[repr(C, align(8))]
pub struct FScriptTypedElementListProxy {
    pub(crate) __padding_end: [u8; 16],
}
impl FScriptTypedElementListProxy {}
#[repr(C, align(8))]
pub struct UEditorDataStorageFactory {
    __padding_end: [u8; 48],
}
impl UEditorDataStorageFactory {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEditorDataStorageFactory")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEditorDataStorageFactory")
            .copied()
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
pub struct UTypedElementHandleLibrary {
    __padding_end: [u8; 48],
}
impl UTypedElementHandleLibrary {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UTypedElementHandleLibrary")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UTypedElementHandleLibrary")
            .copied()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
    pub fn release(element_handle: &mut FScriptTypedElementHandle) {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::typed_element_framework::__FUNCTION_PTRS
                    .u_typed_element_handle_library_release,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                element_handle,
                __buffer.add(0).cast::<FScriptTypedElementHandle>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::typed_element_framework::UTypedElementHandleLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::typed_element_framework::__FUNCTION_PTRS
                    .u_typed_element_handle_library_release,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<FScriptTypedElementHandle>().swap(element_handle);
        }
    }
    pub fn not_equal(
        lhs: &FScriptTypedElementHandle,
        rhs: &FScriptTypedElementHandle,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<17>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::typed_element_framework::__FUNCTION_PTRS
                    .u_typed_element_handle_library_not_equal,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                lhs,
                __buffer.add(0).cast::<FScriptTypedElementHandle>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                rhs,
                __buffer.add(8).cast::<FScriptTypedElementHandle>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::typed_element_framework::UTypedElementHandleLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::typed_element_framework::__FUNCTION_PTRS
                    .u_typed_element_handle_library_not_equal,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<bool>().read() }
    }
    pub fn is_set(element_handle: &FScriptTypedElementHandle) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<9>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::typed_element_framework::__FUNCTION_PTRS
                    .u_typed_element_handle_library_is_set,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                element_handle,
                __buffer.add(0).cast::<FScriptTypedElementHandle>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::typed_element_framework::UTypedElementHandleLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::typed_element_framework::__FUNCTION_PTRS
                    .u_typed_element_handle_library_is_set,
                __buffer,
            )
        };
        unsafe { __buffer.add(8).cast::<bool>().read() }
    }
    pub fn equal(
        lhs: &FScriptTypedElementHandle,
        rhs: &FScriptTypedElementHandle,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<17>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::typed_element_framework::__FUNCTION_PTRS
                    .u_typed_element_handle_library_equal,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                lhs,
                __buffer.add(0).cast::<FScriptTypedElementHandle>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                rhs,
                __buffer.add(8).cast::<FScriptTypedElementHandle>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::typed_element_framework::UTypedElementHandleLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::typed_element_framework::__FUNCTION_PTRS
                    .u_typed_element_handle_library_equal,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<bool>().read() }
    }
}
#[repr(C, align(8))]
pub struct UTypedElementListLibrary {
    __padding_end: [u8; 48],
}
impl UTypedElementListLibrary {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UTypedElementListLibrary")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UTypedElementListLibrary")
            .copied()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
    pub fn shrink(element_list: FScriptTypedElementListProxy) {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::typed_element_framework::__FUNCTION_PTRS
                    .u_typed_element_list_library_shrink,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &element_list,
                __buffer.add(0).cast::<FScriptTypedElementListProxy>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::typed_element_framework::UTypedElementListLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::typed_element_framework::__FUNCTION_PTRS
                    .u_typed_element_list_library_shrink,
                __buffer,
            )
        };
    }
    pub fn reset(element_list: FScriptTypedElementListProxy) {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::typed_element_framework::__FUNCTION_PTRS
                    .u_typed_element_list_library_reset,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &element_list,
                __buffer.add(0).cast::<FScriptTypedElementListProxy>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::typed_element_framework::UTypedElementListLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::typed_element_framework::__FUNCTION_PTRS
                    .u_typed_element_list_library_reset,
                __buffer,
            )
        };
    }
    pub fn reserve(element_list: FScriptTypedElementListProxy, size: i32) {
        let mut __stack = crate::core_data::StackAlloc::<20>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::typed_element_framework::__FUNCTION_PTRS
                    .u_typed_element_list_library_reserve,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &element_list,
                __buffer.add(0).cast::<FScriptTypedElementListProxy>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&size, __buffer.add(16).cast::<i32>(), 1);
        }
        let __object_ptr = crate::bindings::typed_element_framework::UTypedElementListLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::typed_element_framework::__FUNCTION_PTRS
                    .u_typed_element_list_library_reserve,
                __buffer,
            )
        };
    }
    pub fn remove(
        element_list: FScriptTypedElementListProxy,
        element_handle: &FScriptTypedElementHandle,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<25>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::typed_element_framework::__FUNCTION_PTRS
                    .u_typed_element_list_library_remove,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &element_list,
                __buffer.add(0).cast::<FScriptTypedElementListProxy>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                element_handle,
                __buffer.add(16).cast::<FScriptTypedElementHandle>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::typed_element_framework::UTypedElementListLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::typed_element_framework::__FUNCTION_PTRS
                    .u_typed_element_list_library_remove,
                __buffer,
            )
        };
        unsafe { __buffer.add(24).cast::<bool>().read() }
    }
    pub fn num(element_list: FScriptTypedElementListProxy) -> i32 {
        let mut __stack = crate::core_data::StackAlloc::<20>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::typed_element_framework::__FUNCTION_PTRS
                    .u_typed_element_list_library_num,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &element_list,
                __buffer.add(0).cast::<FScriptTypedElementListProxy>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::typed_element_framework::UTypedElementListLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::typed_element_framework::__FUNCTION_PTRS
                    .u_typed_element_list_library_num,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<i32>().read() }
    }
    pub fn is_valid_index(
        element_list: FScriptTypedElementListProxy,
        index: i32,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<21>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::typed_element_framework::__FUNCTION_PTRS
                    .u_typed_element_list_library_is_valid_index,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &element_list,
                __buffer.add(0).cast::<FScriptTypedElementListProxy>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&index, __buffer.add(16).cast::<i32>(), 1);
        }
        let __object_ptr = crate::bindings::typed_element_framework::UTypedElementListLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::typed_element_framework::__FUNCTION_PTRS
                    .u_typed_element_list_library_is_valid_index,
                __buffer,
            )
        };
        unsafe { __buffer.add(20).cast::<bool>().read() }
    }
    pub fn has_elements_of_type(
        element_list: FScriptTypedElementListProxy,
        element_type_name: FName,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<29>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::typed_element_framework::__FUNCTION_PTRS
                    .u_typed_element_list_library_has_elements_of_type,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &element_list,
                __buffer.add(0).cast::<FScriptTypedElementListProxy>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &element_type_name,
                __buffer.add(16).cast::<FName>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::typed_element_framework::UTypedElementListLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::typed_element_framework::__FUNCTION_PTRS
                    .u_typed_element_list_library_has_elements_of_type,
                __buffer,
            )
        };
        unsafe { __buffer.add(28).cast::<bool>().read() }
    }
    pub fn has_elements(
        element_list: FScriptTypedElementListProxy,
        base_interface_type: TSubclassOf<crate::bindings::core_u_object::UInterface>,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<25>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::typed_element_framework::__FUNCTION_PTRS
                    .u_typed_element_list_library_has_elements,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &element_list,
                __buffer.add(0).cast::<FScriptTypedElementListProxy>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &base_interface_type,
                __buffer
                    .add(16)
                    .cast::<TSubclassOf<crate::bindings::core_u_object::UInterface>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::typed_element_framework::UTypedElementListLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::typed_element_framework::__FUNCTION_PTRS
                    .u_typed_element_list_library_has_elements,
                __buffer,
            )
        };
        unsafe { __buffer.add(24).cast::<bool>().read() }
    }
    pub fn get_element_interface(
        element_list: FScriptTypedElementListProxy,
        element_handle: &FScriptTypedElementHandle,
        base_interface_type: TSubclassOf<crate::bindings::core_u_object::UInterface>,
    ) -> UPtr<crate::bindings::core_u_object::UObject> {
        let mut __stack = crate::core_data::StackAlloc::<40>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::typed_element_framework::__FUNCTION_PTRS
                    .u_typed_element_list_library_get_element_interface,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &element_list,
                __buffer.add(0).cast::<FScriptTypedElementListProxy>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                element_handle,
                __buffer.add(16).cast::<FScriptTypedElementHandle>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &base_interface_type,
                __buffer
                    .add(24)
                    .cast::<TSubclassOf<crate::bindings::core_u_object::UInterface>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::typed_element_framework::UTypedElementListLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::typed_element_framework::__FUNCTION_PTRS
                    .u_typed_element_list_library_get_element_interface,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(32)
                .cast::<UPtr<crate::bindings::core_u_object::UObject>>()
                .read()
        }
    }
    pub fn get_element_handles(
        element_list: FScriptTypedElementListProxy,
        base_interface_type: TSubclassOf<crate::bindings::core_u_object::UInterface>,
    ) -> TArray<FScriptTypedElementHandle> {
        let mut __stack = crate::core_data::StackAlloc::<40>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::typed_element_framework::__FUNCTION_PTRS
                    .u_typed_element_list_library_get_element_handles,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &element_list,
                __buffer.add(0).cast::<FScriptTypedElementListProxy>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &base_interface_type,
                __buffer
                    .add(16)
                    .cast::<TSubclassOf<crate::bindings::core_u_object::UInterface>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::typed_element_framework::UTypedElementListLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::typed_element_framework::__FUNCTION_PTRS
                    .u_typed_element_list_library_get_element_handles,
                __buffer,
            )
        };
        unsafe { __buffer.add(24).cast::<TArray<FScriptTypedElementHandle>>().read() }
    }
    pub fn get_element_handle_at(
        element_list: FScriptTypedElementListProxy,
        index: i32,
    ) -> FScriptTypedElementHandle {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::typed_element_framework::__FUNCTION_PTRS
                    .u_typed_element_list_library_get_element_handle_at,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &element_list,
                __buffer.add(0).cast::<FScriptTypedElementListProxy>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&index, __buffer.add(16).cast::<i32>(), 1);
        }
        let __object_ptr = crate::bindings::typed_element_framework::UTypedElementListLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::typed_element_framework::__FUNCTION_PTRS
                    .u_typed_element_list_library_get_element_handle_at,
                __buffer,
            )
        };
        unsafe { __buffer.add(24).cast::<FScriptTypedElementHandle>().read() }
    }
    pub fn empty(element_list: FScriptTypedElementListProxy, slack: i32) {
        let mut __stack = crate::core_data::StackAlloc::<20>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::typed_element_framework::__FUNCTION_PTRS
                    .u_typed_element_list_library_empty,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &element_list,
                __buffer.add(0).cast::<FScriptTypedElementListProxy>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&slack, __buffer.add(16).cast::<i32>(), 1);
        }
        let __object_ptr = crate::bindings::typed_element_framework::UTypedElementListLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::typed_element_framework::__FUNCTION_PTRS
                    .u_typed_element_list_library_empty,
                __buffer,
            )
        };
    }
    pub fn create_element_list(
        registry: UPtr<UTypedElementRegistry>,
    ) -> FScriptTypedElementListProxy {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::typed_element_framework::__FUNCTION_PTRS
                    .u_typed_element_list_library_create_script_element_list,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &registry,
                __buffer.add(0).cast::<UPtr<UTypedElementRegistry>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::typed_element_framework::UTypedElementListLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::typed_element_framework::__FUNCTION_PTRS
                    .u_typed_element_list_library_create_script_element_list,
                __buffer,
            )
        };
        unsafe { __buffer.add(8).cast::<FScriptTypedElementListProxy>().read() }
    }
    pub fn count_elements_of_type(
        element_list: FScriptTypedElementListProxy,
        element_type_name: FName,
    ) -> i32 {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::typed_element_framework::__FUNCTION_PTRS
                    .u_typed_element_list_library_count_elements_of_type,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &element_list,
                __buffer.add(0).cast::<FScriptTypedElementListProxy>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &element_type_name,
                __buffer.add(16).cast::<FName>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::typed_element_framework::UTypedElementListLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::typed_element_framework::__FUNCTION_PTRS
                    .u_typed_element_list_library_count_elements_of_type,
                __buffer,
            )
        };
        unsafe { __buffer.add(28).cast::<i32>().read() }
    }
    pub fn count_elements(
        element_list: FScriptTypedElementListProxy,
        base_interface_type: TSubclassOf<crate::bindings::core_u_object::UInterface>,
    ) -> i32 {
        let mut __stack = crate::core_data::StackAlloc::<28>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::typed_element_framework::__FUNCTION_PTRS
                    .u_typed_element_list_library_count_elements,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &element_list,
                __buffer.add(0).cast::<FScriptTypedElementListProxy>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &base_interface_type,
                __buffer
                    .add(16)
                    .cast::<TSubclassOf<crate::bindings::core_u_object::UInterface>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::typed_element_framework::UTypedElementListLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::typed_element_framework::__FUNCTION_PTRS
                    .u_typed_element_list_library_count_elements,
                __buffer,
            )
        };
        unsafe { __buffer.add(24).cast::<i32>().read() }
    }
    pub fn contains(
        element_list: FScriptTypedElementListProxy,
        element_handle: &FScriptTypedElementHandle,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<25>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::typed_element_framework::__FUNCTION_PTRS
                    .u_typed_element_list_library_contains,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &element_list,
                __buffer.add(0).cast::<FScriptTypedElementListProxy>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                element_handle,
                __buffer.add(16).cast::<FScriptTypedElementHandle>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::typed_element_framework::UTypedElementListLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::typed_element_framework::__FUNCTION_PTRS
                    .u_typed_element_list_library_contains,
                __buffer,
            )
        };
        unsafe { __buffer.add(24).cast::<bool>().read() }
    }
    pub fn clone(
        element_list: FScriptTypedElementListProxy,
    ) -> FScriptTypedElementListProxy {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::typed_element_framework::__FUNCTION_PTRS
                    .u_typed_element_list_library_clone,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &element_list,
                __buffer.add(0).cast::<FScriptTypedElementListProxy>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::typed_element_framework::UTypedElementListLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::typed_element_framework::__FUNCTION_PTRS
                    .u_typed_element_list_library_clone,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<FScriptTypedElementListProxy>().read() }
    }
    pub fn append_list(
        element_list: FScriptTypedElementListProxy,
        other_element_list: FScriptTypedElementListProxy,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::typed_element_framework::__FUNCTION_PTRS
                    .u_typed_element_list_library_append_list,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &element_list,
                __buffer.add(0).cast::<FScriptTypedElementListProxy>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &other_element_list,
                __buffer.add(16).cast::<FScriptTypedElementListProxy>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::typed_element_framework::UTypedElementListLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::typed_element_framework::__FUNCTION_PTRS
                    .u_typed_element_list_library_append_list,
                __buffer,
            )
        };
    }
    pub fn append(
        element_list: FScriptTypedElementListProxy,
        element_handles: &TArray<FScriptTypedElementHandle>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::typed_element_framework::__FUNCTION_PTRS
                    .u_typed_element_list_library_append,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &element_list,
                __buffer.add(0).cast::<FScriptTypedElementListProxy>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                element_handles,
                __buffer.add(16).cast::<TArray<FScriptTypedElementHandle>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::typed_element_framework::UTypedElementListLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::typed_element_framework::__FUNCTION_PTRS
                    .u_typed_element_list_library_append,
                __buffer,
            )
        };
    }
    pub fn add(
        element_list: FScriptTypedElementListProxy,
        element_handle: &FScriptTypedElementHandle,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<25>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::typed_element_framework::__FUNCTION_PTRS
                    .u_typed_element_list_library_add,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &element_list,
                __buffer.add(0).cast::<FScriptTypedElementListProxy>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                element_handle,
                __buffer.add(16).cast::<FScriptTypedElementHandle>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::typed_element_framework::UTypedElementListLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::typed_element_framework::__FUNCTION_PTRS
                    .u_typed_element_list_library_add,
                __buffer,
            )
        };
        unsafe { __buffer.add(24).cast::<bool>().read() }
    }
}
pub struct ITypedElementCounterInterface {}
#[repr(C, align(8))]
pub struct UTypedElementCounterInterface {
    __padding_end: [u8; 48],
}
impl UTypedElementCounterInterface {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UTypedElementCounterInterface")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UTypedElementCounterInterface")
            .copied()
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
pub struct UTypedElementRegistry {
    __padding_end: [u8; 2408],
}
impl UTypedElementRegistry {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UTypedElementRegistry")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UTypedElementRegistry")
            .copied()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
    pub fn get_default_typed_element_registry() -> UPtr<UTypedElementRegistry> {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::typed_element_framework::__FUNCTION_PTRS
                    .u_typed_element_registry_get_instance,
                __buffer,
            )
        };
        let __object_ptr = crate::bindings::typed_element_framework::UTypedElementRegistry::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::typed_element_framework::__FUNCTION_PTRS
                    .u_typed_element_registry_get_instance,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<UPtr<UTypedElementRegistry>>().read() }
    }
    pub fn get_element_interface(
        &self,
        in_element_handle: &FScriptTypedElementHandle,
        in_base_interface_type: TSubclassOf<crate::bindings::core_u_object::UInterface>,
    ) -> UPtr<crate::bindings::core_u_object::UObject> {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::typed_element_framework::__FUNCTION_PTRS
                    .u_typed_element_registry_get_element_interface,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_element_handle,
                __buffer.add(0).cast::<FScriptTypedElementHandle>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_base_interface_type,
                __buffer
                    .add(8)
                    .cast::<TSubclassOf<crate::bindings::core_u_object::UInterface>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::typed_element_framework::__FUNCTION_PTRS
                    .u_typed_element_registry_get_element_interface,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(16)
                .cast::<UPtr<crate::bindings::core_u_object::UObject>>()
                .read()
        }
    }
}
pub struct ITestTypedElementInterfaceA {}
#[repr(C, align(8))]
pub struct UTestTypedElementInterfaceA {
    __padding_end: [u8; 48],
}
impl UTestTypedElementInterfaceA {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UTestTypedElementInterfaceA")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UTestTypedElementInterfaceA")
            .copied()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
    pub fn set_display_name(
        &mut self,
        in_element_handle: &FScriptTypedElementHandle,
        in_new_name: FText,
        b_notify: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<26>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::typed_element_framework::__FUNCTION_PTRS
                    .u_test_typed_element_interface_a_set_display_name,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_element_handle,
                __buffer.add(0).cast::<FScriptTypedElementHandle>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_new_name,
                __buffer.add(8).cast::<FText>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&b_notify, __buffer.add(24).cast::<bool>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::typed_element_framework::__FUNCTION_PTRS
                    .u_test_typed_element_interface_a_set_display_name,
                __buffer,
            )
        };
        unsafe { __buffer.add(25).cast::<bool>().read() }
    }
    pub fn get_display_name(
        &mut self,
        in_element_handle: &FScriptTypedElementHandle,
    ) -> FText {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::typed_element_framework::__FUNCTION_PTRS
                    .u_test_typed_element_interface_a_get_display_name,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_element_handle,
                __buffer.add(0).cast::<FScriptTypedElementHandle>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::typed_element_framework::__FUNCTION_PTRS
                    .u_test_typed_element_interface_a_get_display_name,
                __buffer,
            )
        };
        unsafe { __buffer.add(8).cast::<FText>().read() }
    }
}
pub struct ITestTypedElementInterfaceB {}
#[repr(C, align(8))]
pub struct UTestTypedElementInterfaceB {
    __padding_end: [u8; 48],
}
impl UTestTypedElementInterfaceB {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UTestTypedElementInterfaceB")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UTestTypedElementInterfaceB")
            .copied()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
    pub fn mark_as_tested(
        &mut self,
        in_element_handle: &FScriptTypedElementHandle,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<9>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::typed_element_framework::__FUNCTION_PTRS
                    .u_test_typed_element_interface_b_mark_as_tested,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_element_handle,
                __buffer.add(0).cast::<FScriptTypedElementHandle>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::typed_element_framework::__FUNCTION_PTRS
                    .u_test_typed_element_interface_b_mark_as_tested,
                __buffer,
            )
        };
        unsafe { __buffer.add(8).cast::<bool>().read() }
    }
}
pub struct ITestTypedElementInterfaceC {}
#[repr(C, align(8))]
pub struct UTestTypedElementInterfaceC {
    __padding_end: [u8; 48],
}
impl UTestTypedElementInterfaceC {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UTestTypedElementInterfaceC")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UTestTypedElementInterfaceC")
            .copied()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
    pub fn get_is_tested(&self, in_element_handle: &FScriptTypedElementHandle) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<9>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::typed_element_framework::__FUNCTION_PTRS
                    .u_test_typed_element_interface_c_get_is_tested,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_element_handle,
                __buffer.add(0).cast::<FScriptTypedElementHandle>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::typed_element_framework::__FUNCTION_PTRS
                    .u_test_typed_element_interface_c_get_is_tested,
                __buffer,
            )
        };
        unsafe { __buffer.add(8).cast::<bool>().read() }
    }
}
#[repr(C, align(8))]
pub struct UTestTypedElementInterfaceA_ImplTyped {
    __padding_end: [u8; 56],
}
impl UTestTypedElementInterfaceA_ImplTyped {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UTestTypedElementInterfaceA_ImplTyped")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UTestTypedElementInterfaceA_ImplTyped")
            .copied()
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
pub struct UTestTypedElementInterfaceA_ImplUntyped {
    __padding_end: [u8; 56],
}
impl UTestTypedElementInterfaceA_ImplUntyped {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UTestTypedElementInterfaceA_ImplUntyped")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UTestTypedElementInterfaceA_ImplUntyped")
            .copied()
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
pub struct UTestTypedElementInterfaceBAndC_Typed {
    __padding_end: [u8; 64],
}
impl UTestTypedElementInterfaceBAndC_Typed {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UTestTypedElementInterfaceBAndC_Typed")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UTestTypedElementInterfaceBAndC_Typed")
            .copied()
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
pub struct UTest_PingPongBetweenPhaseFactory {
    __padding_end: [u8; 48],
}
impl UTest_PingPongBetweenPhaseFactory {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UTest_PingPongBetweenPhaseFactory")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UTest_PingPongBetweenPhaseFactory")
            .copied()
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
pub struct ESCCModification(pub i32);
impl ESCCModification {
    pub const CONFLICTED: ESCCModification = ESCCModification(0);
    pub const MODIFIED: ESCCModification = ESCCModification(1);
    pub const ADDED: ESCCModification = ESCCModification(2);
    pub const REMOVED: ESCCModification = ESCCModification(3);
}
