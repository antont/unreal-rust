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
pub struct UAndroidRuntimeSettings {
    __padding_end: [u8; 944],
}
impl UAndroidRuntimeSettings {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAndroidRuntimeSettings")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAndroidRuntimeSettings")
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
pub struct EAndroidInstallLocation(pub u8);
impl EAndroidInstallLocation {
    pub const INTERNAL_ONLY: EAndroidInstallLocation = EAndroidInstallLocation(0);
    pub const PREFER_EXTERNAL: EAndroidInstallLocation = EAndroidInstallLocation(1);
    pub const AUTO: EAndroidInstallLocation = EAndroidInstallLocation(2);
}
#[repr(transparent)]
pub struct EAndroidScreenOrientation(pub u8);
impl EAndroidScreenOrientation {
    pub const PORTRAIT: EAndroidScreenOrientation = EAndroidScreenOrientation(0);
    pub const REVERSE_PORTRAIT: EAndroidScreenOrientation = EAndroidScreenOrientation(1);
    pub const SENSOR_PORTRAIT: EAndroidScreenOrientation = EAndroidScreenOrientation(2);
    pub const LANDSCAPE: EAndroidScreenOrientation = EAndroidScreenOrientation(3);
    pub const REVERSE_LANDSCAPE: EAndroidScreenOrientation = EAndroidScreenOrientation(
        4,
    );
    pub const SENSOR_LANDSCAPE: EAndroidScreenOrientation = EAndroidScreenOrientation(5);
    pub const SENSOR: EAndroidScreenOrientation = EAndroidScreenOrientation(6);
    pub const FULL_SENSOR: EAndroidScreenOrientation = EAndroidScreenOrientation(7);
}
#[repr(transparent)]
pub struct EAndroidRoundedEdgeSafeZoneDirection(pub u8);
impl EAndroidRoundedEdgeSafeZoneDirection {
    pub const NONE: EAndroidRoundedEdgeSafeZoneDirection = EAndroidRoundedEdgeSafeZoneDirection(
        0,
    );
    pub const HORIZONTAL: EAndroidRoundedEdgeSafeZoneDirection = EAndroidRoundedEdgeSafeZoneDirection(
        1,
    );
    pub const VERTICAL: EAndroidRoundedEdgeSafeZoneDirection = EAndroidRoundedEdgeSafeZoneDirection(
        2,
    );
    pub const BOTH: EAndroidRoundedEdgeSafeZoneDirection = EAndroidRoundedEdgeSafeZoneDirection(
        3,
    );
}
#[repr(transparent)]
pub struct EAndroidDepthBufferPreference(pub u8);
impl EAndroidDepthBufferPreference {
    pub const DEFAULT: EAndroidDepthBufferPreference = EAndroidDepthBufferPreference(0);
    pub const BITS16: EAndroidDepthBufferPreference = EAndroidDepthBufferPreference(16);
    pub const BITS24: EAndroidDepthBufferPreference = EAndroidDepthBufferPreference(24);
    pub const BITS32: EAndroidDepthBufferPreference = EAndroidDepthBufferPreference(32);
}
#[repr(transparent)]
pub struct EOculusMobileDevice(pub u8);
impl EOculusMobileDevice {
    pub const QUEST: EOculusMobileDevice = EOculusMobileDevice(1);
    pub const QUEST2: EOculusMobileDevice = EOculusMobileDevice(2);
}
#[repr(transparent)]
pub struct ETagForChildDirectedTreatment(pub u8);
impl ETagForChildDirectedTreatment {
    pub const TAG_FOR_CHILD_DIRECTED_TREATMENT_UNSPECIFIED: ETagForChildDirectedTreatment = ETagForChildDirectedTreatment(
        0,
    );
    pub const TAG_FOR_CHILD_DIRECTED_TREATMENT_TRUE: ETagForChildDirectedTreatment = ETagForChildDirectedTreatment(
        1,
    );
    pub const TAG_FOR_CHILD_DIRECTED_TREATMENT_FALSE: ETagForChildDirectedTreatment = ETagForChildDirectedTreatment(
        2,
    );
}
#[repr(transparent)]
pub struct ETagForUnderAgeOfConsent(pub u8);
impl ETagForUnderAgeOfConsent {
    pub const TAG_FOR_UNDER_AGE_OF_CONSENT_UNSPECIFIED: ETagForUnderAgeOfConsent = ETagForUnderAgeOfConsent(
        0,
    );
    pub const TAG_FOR_UNDER_AGE_OF_CONSENT_TRUE: ETagForUnderAgeOfConsent = ETagForUnderAgeOfConsent(
        1,
    );
    pub const TAG_FOR_UNDER_AGE_OF_CONSENT_FALSE: ETagForUnderAgeOfConsent = ETagForUnderAgeOfConsent(
        2,
    );
}
#[repr(transparent)]
pub struct EMaxAdContentRating(pub u8);
impl EMaxAdContentRating {
    pub const MAX_AD_CONTENT_RATING_G: EMaxAdContentRating = EMaxAdContentRating(0);
    pub const MAX_AD_CONTENT_RATING_PG: EMaxAdContentRating = EMaxAdContentRating(1);
    pub const MAX_AD_CONTENT_RATING_T: EMaxAdContentRating = EMaxAdContentRating(2);
    pub const MAX_AD_CONTENT_RATING_MA: EMaxAdContentRating = EMaxAdContentRating(2);
}
#[repr(transparent)]
pub struct EAndroidGraphicsDebugger(pub u8);
impl EAndroidGraphicsDebugger {
    pub const NONE: EAndroidGraphicsDebugger = EAndroidGraphicsDebugger(0);
    pub const MALI: EAndroidGraphicsDebugger = EAndroidGraphicsDebugger(1);
    pub const ADRENO: EAndroidGraphicsDebugger = EAndroidGraphicsDebugger(2);
}
