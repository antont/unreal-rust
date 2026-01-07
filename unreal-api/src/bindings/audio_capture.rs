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
    pub fn stop_capturing_audio(&mut self) {
        let mut __stack = crate::core_data::StackAlloc::<0>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::audio_capture::U_AUDIO_CAPTURE_STOP_CAPTURING_AUDIO,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::audio_capture::U_AUDIO_CAPTURE_STOP_CAPTURING_AUDIO,
                __buffer,
            )
        };
    }
    pub fn start_capturing_audio(&mut self) {
        let mut __stack = crate::core_data::StackAlloc::<0>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::audio_capture::U_AUDIO_CAPTURE_START_CAPTURING_AUDIO,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::audio_capture::U_AUDIO_CAPTURE_START_CAPTURING_AUDIO,
                __buffer,
            )
        };
    }
    pub fn is_capturing_audio(&mut self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::audio_capture::U_AUDIO_CAPTURE_IS_CAPTURING_AUDIO,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::audio_capture::U_AUDIO_CAPTURE_IS_CAPTURING_AUDIO,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn get_audio_capture_device_info(
        &mut self,
        out_info: &mut FAudioCaptureDeviceInfo,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<21>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::audio_capture::U_AUDIO_CAPTURE_GET_AUDIO_CAPTURE_DEVICE_INFO,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_info,
                __buffer.add(0).cast::<FAudioCaptureDeviceInfo>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::audio_capture::U_AUDIO_CAPTURE_GET_AUDIO_CAPTURE_DEVICE_INFO,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<FAudioCaptureDeviceInfo>().swap(out_info);
        }
        unsafe { __buffer.add(20).cast::<bool>().read() }
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
    pub fn create_audio_capture() -> UPtr<UAudioCapture> {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::audio_capture::U_AUDIO_CAPTURE_FUNCTION_LIBRARY_CREATE_AUDIO_CAPTURE,
                __buffer,
            )
        };
        let __object_ptr = crate::bindings::audio_capture::UAudioCaptureFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::audio_capture::U_AUDIO_CAPTURE_FUNCTION_LIBRARY_CREATE_AUDIO_CAPTURE,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<UPtr<UAudioCapture>>().read() }
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
    pub fn get_available_audio_input_devices(
        world_context_object: UPtr<crate::bindings::core_u_object::UObject>,
        on_obtain_devices_event: &FGetAvailableAudioInputDevices_OnObtainDevicesEvent,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<40>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::audio_capture::U_AUDIO_CAPTURE_BLUEPRINT_LIBRARY_GET_AVAILABLE_AUDIO_INPUT_DEVICES,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &world_context_object,
                __buffer.add(0).cast::<UPtr<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                on_obtain_devices_event,
                __buffer
                    .add(8)
                    .cast::<FGetAvailableAudioInputDevices_OnObtainDevicesEvent>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::audio_capture::UAudioCaptureBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::audio_capture::U_AUDIO_CAPTURE_BLUEPRINT_LIBRARY_GET_AVAILABLE_AUDIO_INPUT_DEVICES,
                __buffer,
            )
        };
    }
    pub fn conv_audio_input_device_info_to_string(
        info: &FAudioInputDeviceInfo,
    ) -> FString {
        let mut __stack = crate::core_data::StackAlloc::<64>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::audio_capture::U_AUDIO_CAPTURE_BLUEPRINT_LIBRARY_CONV_AUDIO_INPUT_DEVICE_INFO_TO_STRING,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                info,
                __buffer.add(0).cast::<FAudioInputDeviceInfo>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::audio_capture::UAudioCaptureBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::audio_capture::U_AUDIO_CAPTURE_BLUEPRINT_LIBRARY_CONV_AUDIO_INPUT_DEVICE_INFO_TO_STRING,
                __buffer,
            )
        };
        unsafe { __buffer.add(48).cast::<FString>().read() }
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
