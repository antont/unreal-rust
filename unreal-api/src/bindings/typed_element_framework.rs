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
pub static mut U_TYPED_ELEMENT_HANDLE_LIBRARY_RELEASE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_TYPED_ELEMENT_HANDLE_LIBRARY_NOT_EQUAL: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_TYPED_ELEMENT_HANDLE_LIBRARY_IS_SET: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_TYPED_ELEMENT_HANDLE_LIBRARY_EQUAL: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_TYPED_ELEMENT_LIST_LIBRARY_SHRINK: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_TYPED_ELEMENT_LIST_LIBRARY_RESET: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_TYPED_ELEMENT_LIST_LIBRARY_RESERVE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_TYPED_ELEMENT_LIST_LIBRARY_REMOVE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_TYPED_ELEMENT_LIST_LIBRARY_NUM: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_TYPED_ELEMENT_LIST_LIBRARY_IS_VALID_INDEX: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_TYPED_ELEMENT_LIST_LIBRARY_HAS_ELEMENTS_OF_TYPE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_TYPED_ELEMENT_LIST_LIBRARY_HAS_ELEMENTS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_TYPED_ELEMENT_LIST_LIBRARY_GET_ELEMENT_INTERFACE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_TYPED_ELEMENT_LIST_LIBRARY_GET_ELEMENT_HANDLES: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_TYPED_ELEMENT_LIST_LIBRARY_GET_ELEMENT_HANDLE_AT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_TYPED_ELEMENT_LIST_LIBRARY_EMPTY: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_TYPED_ELEMENT_LIST_LIBRARY_CREATE_SCRIPT_ELEMENT_LIST: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_TYPED_ELEMENT_LIST_LIBRARY_COUNT_ELEMENTS_OF_TYPE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_TYPED_ELEMENT_LIST_LIBRARY_COUNT_ELEMENTS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_TYPED_ELEMENT_LIST_LIBRARY_CONTAINS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_TYPED_ELEMENT_LIST_LIBRARY_CLONE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_TYPED_ELEMENT_LIST_LIBRARY_APPEND_LIST: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_TYPED_ELEMENT_LIST_LIBRARY_APPEND: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_TYPED_ELEMENT_LIST_LIBRARY_ADD: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_TYPED_ELEMENT_REGISTRY_GET_INSTANCE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_TYPED_ELEMENT_REGISTRY_GET_ELEMENT_INTERFACE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_TEST_TYPED_ELEMENT_INTERFACE_A_SET_DISPLAY_NAME: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_TEST_TYPED_ELEMENT_INTERFACE_A_GET_DISPLAY_NAME: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_TEST_TYPED_ELEMENT_INTERFACE_B_MARK_AS_TESTED: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_TEST_TYPED_ELEMENT_INTERFACE_C_GET_IS_TESTED: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
pub fn initialize() {
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UTypedElementHandleLibrary::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("Release"),
            &raw mut U_TYPED_ELEMENT_HANDLE_LIBRARY_RELEASE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("NotEqual"),
            &raw mut U_TYPED_ELEMENT_HANDLE_LIBRARY_NOT_EQUAL,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("IsSet"),
            &raw mut U_TYPED_ELEMENT_HANDLE_LIBRARY_IS_SET,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("Equal"),
            &raw mut U_TYPED_ELEMENT_HANDLE_LIBRARY_EQUAL,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UTypedElementListLibrary::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("Shrink"),
            &raw mut U_TYPED_ELEMENT_LIST_LIBRARY_SHRINK,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("Reset"),
            &raw mut U_TYPED_ELEMENT_LIST_LIBRARY_RESET,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("Reserve"),
            &raw mut U_TYPED_ELEMENT_LIST_LIBRARY_RESERVE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("Remove"),
            &raw mut U_TYPED_ELEMENT_LIST_LIBRARY_REMOVE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("Num"),
            &raw mut U_TYPED_ELEMENT_LIST_LIBRARY_NUM,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("IsValidIndex"),
            &raw mut U_TYPED_ELEMENT_LIST_LIBRARY_IS_VALID_INDEX,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("HasElementsOfType"),
            &raw mut U_TYPED_ELEMENT_LIST_LIBRARY_HAS_ELEMENTS_OF_TYPE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("HasElements"),
            &raw mut U_TYPED_ELEMENT_LIST_LIBRARY_HAS_ELEMENTS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetElementInterface"),
            &raw mut U_TYPED_ELEMENT_LIST_LIBRARY_GET_ELEMENT_INTERFACE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetElementHandles"),
            &raw mut U_TYPED_ELEMENT_LIST_LIBRARY_GET_ELEMENT_HANDLES,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetElementHandleAt"),
            &raw mut U_TYPED_ELEMENT_LIST_LIBRARY_GET_ELEMENT_HANDLE_AT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("Empty"),
            &raw mut U_TYPED_ELEMENT_LIST_LIBRARY_EMPTY,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CreateScriptElementList"),
            &raw mut U_TYPED_ELEMENT_LIST_LIBRARY_CREATE_SCRIPT_ELEMENT_LIST,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CountElementsOfType"),
            &raw mut U_TYPED_ELEMENT_LIST_LIBRARY_COUNT_ELEMENTS_OF_TYPE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CountElements"),
            &raw mut U_TYPED_ELEMENT_LIST_LIBRARY_COUNT_ELEMENTS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("Contains"),
            &raw mut U_TYPED_ELEMENT_LIST_LIBRARY_CONTAINS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("Clone"),
            &raw mut U_TYPED_ELEMENT_LIST_LIBRARY_CLONE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AppendList"),
            &raw mut U_TYPED_ELEMENT_LIST_LIBRARY_APPEND_LIST,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("Append"),
            &raw mut U_TYPED_ELEMENT_LIST_LIBRARY_APPEND,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("Add"),
            &raw mut U_TYPED_ELEMENT_LIST_LIBRARY_ADD,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UTypedElementRegistry::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetInstance"),
            &raw mut U_TYPED_ELEMENT_REGISTRY_GET_INSTANCE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetElementInterface"),
            &raw mut U_TYPED_ELEMENT_REGISTRY_GET_ELEMENT_INTERFACE,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UTestTypedElementInterfaceA::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetDisplayName"),
            &raw mut U_TEST_TYPED_ELEMENT_INTERFACE_A_SET_DISPLAY_NAME,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetDisplayName"),
            &raw mut U_TEST_TYPED_ELEMENT_INTERFACE_A_GET_DISPLAY_NAME,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UTestTypedElementInterfaceB::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("MarkAsTested"),
            &raw mut U_TEST_TYPED_ELEMENT_INTERFACE_B_MARK_AS_TESTED,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UTestTypedElementInterfaceC::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetIsTested"),
            &raw mut U_TEST_TYPED_ELEMENT_INTERFACE_C_GET_IS_TESTED,
        );
    }
}
#[repr(C, align(8))]
pub struct FScriptTypedElementHandle {
    __padding_end: [u8; 8],
}
impl FScriptTypedElementHandle {}
#[repr(C, align(8))]
pub struct FScriptTypedElementListProxy {
    __padding_end: [u8; 16],
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
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
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
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
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
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
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
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
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
