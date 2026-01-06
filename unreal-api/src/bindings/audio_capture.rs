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
pub static mut U_AUDIO_CAPTURE_STOP_CAPTURING_AUDIO: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_AUDIO_CAPTURE_START_CAPTURING_AUDIO: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_AUDIO_CAPTURE_IS_CAPTURING_AUDIO: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_AUDIO_CAPTURE_GET_AUDIO_CAPTURE_DEVICE_INFO: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_AUDIO_CAPTURE_FUNCTION_LIBRARY_CREATE_AUDIO_CAPTURE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_AUDIO_CAPTURE_BLUEPRINT_LIBRARY_GET_AVAILABLE_AUDIO_INPUT_DEVICES: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_AUDIO_CAPTURE_BLUEPRINT_LIBRARY_CONV_AUDIO_INPUT_DEVICE_INFO_TO_STRING: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
pub fn initialize() {
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UAudioCapture::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("StopCapturingAudio"),
            &raw mut U_AUDIO_CAPTURE_STOP_CAPTURING_AUDIO,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("StartCapturingAudio"),
            &raw mut U_AUDIO_CAPTURE_START_CAPTURING_AUDIO,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("IsCapturingAudio"),
            &raw mut U_AUDIO_CAPTURE_IS_CAPTURING_AUDIO,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetAudioCaptureDeviceInfo"),
            &raw mut U_AUDIO_CAPTURE_GET_AUDIO_CAPTURE_DEVICE_INFO,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UAudioCaptureFunctionLibrary::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CreateAudioCapture"),
            &raw mut U_AUDIO_CAPTURE_FUNCTION_LIBRARY_CREATE_AUDIO_CAPTURE,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UAudioCaptureBlueprintLibrary::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetAvailableAudioInputDevices"),
            &raw mut U_AUDIO_CAPTURE_BLUEPRINT_LIBRARY_GET_AVAILABLE_AUDIO_INPUT_DEVICES,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("Conv_AudioInputDeviceInfoToString"),
            &raw mut U_AUDIO_CAPTURE_BLUEPRINT_LIBRARY_CONV_AUDIO_INPUT_DEVICE_INFO_TO_STRING,
        );
    }
}
#[repr(C, align(8))]
pub struct FAudioInputDeviceInfo {
    pub device_name: FString,
    pub device_id: FString,
    pub input_channels: i32,
    pub preferred_sample_rate: i32,
    pub flags_40: u8,
    __padding_end: [u8; 7],
}
impl FAudioInputDeviceInfo {}
#[repr(C, align(4))]
pub struct FAudioCaptureDeviceInfo {
    pub device_name: FName,
    pub num_input_channels: i32,
    pub sample_rate: i32,
}
impl FAudioCaptureDeviceInfo {}
#[repr(C, align(8))]
pub struct UAudioCapture {
    __padding_end: [u8; 184],
}
impl UAudioCapture {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAudioCapture")
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
pub struct UAudioCaptureFunctionLibrary {
    __padding_end: [u8; 48],
}
impl UAudioCaptureFunctionLibrary {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAudioCaptureFunctionLibrary")
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
pub struct UAudioCaptureBlueprintLibrary {
    __padding_end: [u8; 48],
}
impl UAudioCaptureBlueprintLibrary {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAudioCaptureBlueprintLibrary")
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
pub struct UAudioCaptureComponent {
    #[doc(hidden)]
    __padding_2384: [u8; 2384],
    pub jitter_latency_frames: i32,
    __padding_end: [u8; 188],
}
impl UAudioCaptureComponent {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAudioCaptureComponent")
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
pub struct FGetAvailableAudioInputDevices_OnObtainDevicesEvent {
    _opague: [u8; 32],
}
