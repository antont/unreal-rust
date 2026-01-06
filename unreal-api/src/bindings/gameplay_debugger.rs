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
pub static mut A_GAMEPLAY_DEBUGGER_CATEGORY_REPLICATOR_SERVER_SET_VIEW_POINT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut A_GAMEPLAY_DEBUGGER_CATEGORY_REPLICATOR_SERVER_SET_ENABLED: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut A_GAMEPLAY_DEBUGGER_CATEGORY_REPLICATOR_SERVER_SET_DEBUG_ACTOR: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut A_GAMEPLAY_DEBUGGER_CATEGORY_REPLICATOR_SERVER_SET_CATEGORY_ENABLED: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut A_GAMEPLAY_DEBUGGER_CATEGORY_REPLICATOR_SERVER_SEND_EXTENSION_INPUT_EVENT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut A_GAMEPLAY_DEBUGGER_CATEGORY_REPLICATOR_SERVER_SEND_CATEGORY_INPUT_EVENT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut A_GAMEPLAY_DEBUGGER_CATEGORY_REPLICATOR_SERVER_RESET_VIEW_POINT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut A_GAMEPLAY_DEBUGGER_CATEGORY_REPLICATOR_ON_REP_REPLICATED_DATA: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut A_GAMEPLAY_DEBUGGER_CATEGORY_REPLICATOR_CLIENT_DATA_PACK_PACKET: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
pub fn initialize() {
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = AGameplayDebuggerCategoryReplicator::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ServerSetViewPoint"),
            &raw mut A_GAMEPLAY_DEBUGGER_CATEGORY_REPLICATOR_SERVER_SET_VIEW_POINT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ServerSetEnabled"),
            &raw mut A_GAMEPLAY_DEBUGGER_CATEGORY_REPLICATOR_SERVER_SET_ENABLED,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ServerSetDebugActor"),
            &raw mut A_GAMEPLAY_DEBUGGER_CATEGORY_REPLICATOR_SERVER_SET_DEBUG_ACTOR,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ServerSetCategoryEnabled"),
            &raw mut A_GAMEPLAY_DEBUGGER_CATEGORY_REPLICATOR_SERVER_SET_CATEGORY_ENABLED,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ServerSendExtensionInputEvent"),
            &raw mut A_GAMEPLAY_DEBUGGER_CATEGORY_REPLICATOR_SERVER_SEND_EXTENSION_INPUT_EVENT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ServerSendCategoryInputEvent"),
            &raw mut A_GAMEPLAY_DEBUGGER_CATEGORY_REPLICATOR_SERVER_SEND_CATEGORY_INPUT_EVENT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ServerResetViewPoint"),
            &raw mut A_GAMEPLAY_DEBUGGER_CATEGORY_REPLICATOR_SERVER_RESET_VIEW_POINT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("OnRep_ReplicatedData"),
            &raw mut A_GAMEPLAY_DEBUGGER_CATEGORY_REPLICATOR_ON_REP_REPLICATED_DATA,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ClientDataPackPacket"),
            &raw mut A_GAMEPLAY_DEBUGGER_CATEGORY_REPLICATOR_CLIENT_DATA_PACK_PACKET,
        );
    }
}
#[repr(C, align(8))]
pub struct AGameplayDebuggerCategoryReplicator {
    __padding_end: [u8; 1328],
}
impl AGameplayDebuggerCategoryReplicator {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("AGameplayDebuggerCategoryReplicator")
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
pub struct UGameplayDebuggerConfig {
    __padding_end: [u8; 712],
}
impl UGameplayDebuggerConfig {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UGameplayDebuggerConfig")
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
pub struct UGameplayDebuggerUserSettings {
    __padding_end: [u8; 208],
}
impl UGameplayDebuggerUserSettings {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UGameplayDebuggerUserSettings")
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
pub struct UGameplayDebuggerLocalController {
    __padding_end: [u8; 336],
}
impl UGameplayDebuggerLocalController {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UGameplayDebuggerLocalController")
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
pub struct AGameplayDebuggerPlayerManager {
    __padding_end: [u8; 1208],
}
impl AGameplayDebuggerPlayerManager {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("AGameplayDebuggerPlayerManager")
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
#[repr(C, align(16))]
pub struct UGameplayDebuggerRenderingComponent {
    __padding_end: [u8; 1728],
}
impl UGameplayDebuggerRenderingComponent {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UGameplayDebuggerRenderingComponent")
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
pub struct EGameplayDebuggerShape(pub u8);
impl EGameplayDebuggerShape {
    pub const INVALID: EGameplayDebuggerShape = EGameplayDebuggerShape(0);
    pub const POINT: EGameplayDebuggerShape = EGameplayDebuggerShape(1);
    pub const SEGMENT: EGameplayDebuggerShape = EGameplayDebuggerShape(2);
    pub const BOX: EGameplayDebuggerShape = EGameplayDebuggerShape(3);
    pub const CONE: EGameplayDebuggerShape = EGameplayDebuggerShape(4);
    pub const CYLINDER: EGameplayDebuggerShape = EGameplayDebuggerShape(5);
    pub const CIRCLE: EGameplayDebuggerShape = EGameplayDebuggerShape(6);
    pub const RECTANGLE: EGameplayDebuggerShape = EGameplayDebuggerShape(7);
    pub const CAPSULE: EGameplayDebuggerShape = EGameplayDebuggerShape(8);
    pub const POLYGON: EGameplayDebuggerShape = EGameplayDebuggerShape(9);
    pub const POLYLINE: EGameplayDebuggerShape = EGameplayDebuggerShape(10);
    pub const ARROW: EGameplayDebuggerShape = EGameplayDebuggerShape(11);
}
#[repr(transparent)]
pub struct EGameplayDebuggerOverrideMode(pub u8);
impl EGameplayDebuggerOverrideMode {
    pub const ENABLE: EGameplayDebuggerOverrideMode = EGameplayDebuggerOverrideMode(0);
    pub const DISABLE: EGameplayDebuggerOverrideMode = EGameplayDebuggerOverrideMode(1);
    pub const USE_DEFAULT: EGameplayDebuggerOverrideMode = EGameplayDebuggerOverrideMode(
        2,
    );
}
