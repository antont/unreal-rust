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
    pub a_gameplay_debugger_category_replicator_server_set_view_point: *mut crate::ffi::UFunctionOpague,
    pub a_gameplay_debugger_category_replicator_server_set_enabled: *mut crate::ffi::UFunctionOpague,
    pub a_gameplay_debugger_category_replicator_server_set_debug_actor: *mut crate::ffi::UFunctionOpague,
    pub a_gameplay_debugger_category_replicator_server_set_category_enabled: *mut crate::ffi::UFunctionOpague,
    pub a_gameplay_debugger_category_replicator_server_send_extension_input_event: *mut crate::ffi::UFunctionOpague,
    pub a_gameplay_debugger_category_replicator_server_send_category_input_event: *mut crate::ffi::UFunctionOpague,
    pub a_gameplay_debugger_category_replicator_server_reset_view_point: *mut crate::ffi::UFunctionOpague,
    pub a_gameplay_debugger_category_replicator_on_rep_replicated_data: *mut crate::ffi::UFunctionOpague,
    pub a_gameplay_debugger_category_replicator_client_data_pack_packet: *mut crate::ffi::UFunctionOpague,
}
impl FunctionPtrs {
    pub const fn empty() -> Self {
        Self {
            a_gameplay_debugger_category_replicator_server_set_view_point: std::ptr::null_mut(),
            a_gameplay_debugger_category_replicator_server_set_enabled: std::ptr::null_mut(),
            a_gameplay_debugger_category_replicator_server_set_debug_actor: std::ptr::null_mut(),
            a_gameplay_debugger_category_replicator_server_set_category_enabled: std::ptr::null_mut(),
            a_gameplay_debugger_category_replicator_server_send_extension_input_event: std::ptr::null_mut(),
            a_gameplay_debugger_category_replicator_server_send_category_input_event: std::ptr::null_mut(),
            a_gameplay_debugger_category_replicator_server_reset_view_point: std::ptr::null_mut(),
            a_gameplay_debugger_category_replicator_on_rep_replicated_data: std::ptr::null_mut(),
            a_gameplay_debugger_category_replicator_client_data_pack_packet: std::ptr::null_mut(),
        }
    }
}
pub fn initialize() {
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = AGameplayDebuggerCategoryReplicator::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ServerSetViewPoint"),
                &raw mut __FUNCTION_PTRS
                    .a_gameplay_debugger_category_replicator_server_set_view_point,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ServerSetEnabled"),
                &raw mut __FUNCTION_PTRS
                    .a_gameplay_debugger_category_replicator_server_set_enabled,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ServerSetDebugActor"),
                &raw mut __FUNCTION_PTRS
                    .a_gameplay_debugger_category_replicator_server_set_debug_actor,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ServerSetCategoryEnabled"),
                &raw mut __FUNCTION_PTRS
                    .a_gameplay_debugger_category_replicator_server_set_category_enabled,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ServerSendExtensionInputEvent"),
                &raw mut __FUNCTION_PTRS
                    .a_gameplay_debugger_category_replicator_server_send_extension_input_event,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ServerSendCategoryInputEvent"),
                &raw mut __FUNCTION_PTRS
                    .a_gameplay_debugger_category_replicator_server_send_category_input_event,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ServerResetViewPoint"),
                &raw mut __FUNCTION_PTRS
                    .a_gameplay_debugger_category_replicator_server_reset_view_point,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("OnRep_ReplicatedData"),
                &raw mut __FUNCTION_PTRS
                    .a_gameplay_debugger_category_replicator_on_rep_replicated_data,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ClientDataPackPacket"),
                &raw mut __FUNCTION_PTRS
                    .a_gameplay_debugger_category_replicator_client_data_pack_packet,
            );
        }
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
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("AGameplayDebuggerCategoryReplicator")
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
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UGameplayDebuggerConfig")
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
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UGameplayDebuggerUserSettings")
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
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UGameplayDebuggerLocalController")
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
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("AGameplayDebuggerPlayerManager")
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
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UGameplayDebuggerRenderingComponent")
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
