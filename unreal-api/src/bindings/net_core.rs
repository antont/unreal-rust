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
pub struct FunctionPtrs {}
impl FunctionPtrs {
    pub const fn empty() -> Self {
        Self {}
    }
}
pub fn initialize() {}
#[repr(C, align(8))]
pub struct UNetAnalyticsAggregatorConfig {
    __padding_end: [u8; 64],
}
impl UNetAnalyticsAggregatorConfig {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNetAnalyticsAggregatorConfig")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNetAnalyticsAggregatorConfig")
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
pub struct UStatePerObjectConfig {
    __padding_end: [u8; 128],
}
impl UStatePerObjectConfig {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UStatePerObjectConfig")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UStatePerObjectConfig")
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
pub struct UEscalationManagerConfig {
    __padding_end: [u8; 160],
}
impl UEscalationManagerConfig {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEscalationManagerConfig")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEscalationManagerConfig")
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
pub struct EFastArraySerializerDeltaFlags(pub u8);
impl EFastArraySerializerDeltaFlags {
    pub const NONE: EFastArraySerializerDeltaFlags = EFastArraySerializerDeltaFlags(0);
    pub const HAS_BEEN_SERIALIZED: EFastArraySerializerDeltaFlags = EFastArraySerializerDeltaFlags(
        1,
    );
    pub const HAS_DELTA_BEEN_REQUESTED: EFastArraySerializerDeltaFlags = EFastArraySerializerDeltaFlags(
        2,
    );
    pub const IS_USING_DELTA_SERIALIZATION: EFastArraySerializerDeltaFlags = EFastArraySerializerDeltaFlags(
        4,
    );
}
#[repr(transparent)]
pub struct ENetworkFailure(pub u8);
impl ENetworkFailure {
    pub const NET_DRIVER_ALREADY_EXISTS: ENetworkFailure = ENetworkFailure(0);
    pub const NET_DRIVER_CREATE_FAILURE: ENetworkFailure = ENetworkFailure(1);
    pub const NET_DRIVER_LISTEN_FAILURE: ENetworkFailure = ENetworkFailure(2);
    pub const CONNECTION_LOST: ENetworkFailure = ENetworkFailure(3);
    pub const CONNECTION_TIMEOUT: ENetworkFailure = ENetworkFailure(4);
    pub const FAILURE_RECEIVED: ENetworkFailure = ENetworkFailure(5);
    pub const OUTDATED_CLIENT: ENetworkFailure = ENetworkFailure(6);
    pub const OUTDATED_SERVER: ENetworkFailure = ENetworkFailure(7);
    pub const PENDING_CONNECTION_FAILURE: ENetworkFailure = ENetworkFailure(8);
    pub const NET_GUID_MISMATCH: ENetworkFailure = ENetworkFailure(9);
    pub const NET_CHECKSUM_MISMATCH: ENetworkFailure = ENetworkFailure(10);
}
#[repr(transparent)]
pub struct EReplicationSystem(pub u8);
impl EReplicationSystem {
    pub const DEFAULT: EReplicationSystem = EReplicationSystem(0);
    pub const GENERIC: EReplicationSystem = EReplicationSystem(1);
    pub const IRIS: EReplicationSystem = EReplicationSystem(2);
}
