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
pub struct FunctionPtrs {}
impl FunctionPtrs {
    pub const fn empty() -> Self {
        Self {}
    }
}
pub fn initialize() {}
#[repr(C, align(8))]
pub struct UIOSRuntimeSettings {
    __padding_end: [u8; 736],
}
impl UIOSRuntimeSettings {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UIOSRuntimeSettings")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UIOSRuntimeSettings")
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
pub struct EIOSCloudKitSyncStrategy(pub u8);
impl EIOSCloudKitSyncStrategy {
    pub const NONE: EIOSCloudKitSyncStrategy = EIOSCloudKitSyncStrategy(0);
    pub const ONLY_AT_GAME_START: EIOSCloudKitSyncStrategy = EIOSCloudKitSyncStrategy(1);
    pub const ALWAYS: EIOSCloudKitSyncStrategy = EIOSCloudKitSyncStrategy(2);
}
#[repr(transparent)]
pub struct EIOSVersion(pub u8);
impl EIOSVersion {
    pub const IOS_MINIMUM: EIOSVersion = EIOSVersion(15);
    pub const IOS_15: EIOSVersion = EIOSVersion(15);
    pub const IOS_16: EIOSVersion = EIOSVersion(16);
    pub const IOS_17: EIOSVersion = EIOSVersion(17);
}
#[repr(transparent)]
pub struct EPowerUsageFrameRateLock(pub u8);
impl EPowerUsageFrameRateLock {
    pub const PUFRL_NONE: EPowerUsageFrameRateLock = EPowerUsageFrameRateLock(0);
    pub const PUFRL_20: EPowerUsageFrameRateLock = EPowerUsageFrameRateLock(20);
    pub const PUFRL_30: EPowerUsageFrameRateLock = EPowerUsageFrameRateLock(30);
    pub const PUFRL_60: EPowerUsageFrameRateLock = EPowerUsageFrameRateLock(60);
}
#[repr(transparent)]
pub struct EIOSLandscapeOrientation(pub u8);
impl EIOSLandscapeOrientation {
    pub const LANDSCAPE_LEFT: EIOSLandscapeOrientation = EIOSLandscapeOrientation(0);
    pub const LANDSCAPE_RIGHT: EIOSLandscapeOrientation = EIOSLandscapeOrientation(1);
}
