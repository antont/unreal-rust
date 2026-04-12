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
    pub u_audio_capture_stop_capturing_audio: *mut crate::ffi::UFunctionOpague,
    pub u_audio_capture_start_capturing_audio: *mut crate::ffi::UFunctionOpague,
    pub u_audio_capture_is_capturing_audio: *mut crate::ffi::UFunctionOpague,
    pub u_audio_capture_get_audio_capture_device_info: *mut crate::ffi::UFunctionOpague,
    pub u_audio_capture_function_library_create_audio_capture: *mut crate::ffi::UFunctionOpague,
    pub u_audio_capture_blueprint_library_get_available_audio_input_devices: *mut crate::ffi::UFunctionOpague,
    pub u_audio_capture_blueprint_library_conv_audio_input_device_info_to_string: *mut crate::ffi::UFunctionOpague,
}
impl FunctionPtrs {
    pub const fn empty() -> Self {
        Self {
            u_audio_capture_stop_capturing_audio: std::ptr::null_mut(),
            u_audio_capture_start_capturing_audio: std::ptr::null_mut(),
            u_audio_capture_is_capturing_audio: std::ptr::null_mut(),
            u_audio_capture_get_audio_capture_device_info: std::ptr::null_mut(),
            u_audio_capture_function_library_create_audio_capture: std::ptr::null_mut(),
            u_audio_capture_blueprint_library_get_available_audio_input_devices: std::ptr::null_mut(),
            u_audio_capture_blueprint_library_conv_audio_input_device_info_to_string: std::ptr::null_mut(),
        }
    }
}
pub fn initialize() {
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UAudioCapture::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("StopCapturingAudio"),
                &raw mut __FUNCTION_PTRS.u_audio_capture_stop_capturing_audio,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("StartCapturingAudio"),
                &raw mut __FUNCTION_PTRS.u_audio_capture_start_capturing_audio,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("IsCapturingAudio"),
                &raw mut __FUNCTION_PTRS.u_audio_capture_is_capturing_audio,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetAudioCaptureDeviceInfo"),
                &raw mut __FUNCTION_PTRS.u_audio_capture_get_audio_capture_device_info,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UAudioCaptureFunctionLibrary::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("CreateAudioCapture"),
                &raw mut __FUNCTION_PTRS
                    .u_audio_capture_function_library_create_audio_capture,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UAudioCaptureBlueprintLibrary::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetAvailableAudioInputDevices"),
                &raw mut __FUNCTION_PTRS
                    .u_audio_capture_blueprint_library_get_available_audio_input_devices,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("Conv_AudioInputDeviceInfoToString"),
                &raw mut __FUNCTION_PTRS
                    .u_audio_capture_blueprint_library_conv_audio_input_device_info_to_string,
            );
        }
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
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAudioCapture")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAudioCapture")
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
    pub fn stop_capturing_audio(&mut self) {
        let mut __stack = crate::core_data::StackAlloc::<0>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::audio_capture::__FUNCTION_PTRS
                    .u_audio_capture_stop_capturing_audio,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::audio_capture::__FUNCTION_PTRS
                    .u_audio_capture_stop_capturing_audio,
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
                crate::bindings::audio_capture::__FUNCTION_PTRS
                    .u_audio_capture_start_capturing_audio,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::audio_capture::__FUNCTION_PTRS
                    .u_audio_capture_start_capturing_audio,
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
                crate::bindings::audio_capture::__FUNCTION_PTRS
                    .u_audio_capture_is_capturing_audio,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::audio_capture::__FUNCTION_PTRS
                    .u_audio_capture_is_capturing_audio,
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
                crate::bindings::audio_capture::__FUNCTION_PTRS
                    .u_audio_capture_get_audio_capture_device_info,
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
                crate::bindings::audio_capture::__FUNCTION_PTRS
                    .u_audio_capture_get_audio_capture_device_info,
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
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAudioCaptureFunctionLibrary")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAudioCaptureFunctionLibrary")
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
    pub fn create_audio_capture() -> UPtr<UAudioCapture> {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::audio_capture::__FUNCTION_PTRS
                    .u_audio_capture_function_library_create_audio_capture,
                __buffer,
            )
        };
        let __object_ptr = crate::bindings::audio_capture::UAudioCaptureFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::audio_capture::__FUNCTION_PTRS
                    .u_audio_capture_function_library_create_audio_capture,
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
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAudioCaptureBlueprintLibrary")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAudioCaptureBlueprintLibrary")
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
                crate::bindings::audio_capture::__FUNCTION_PTRS
                    .u_audio_capture_blueprint_library_get_available_audio_input_devices,
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
                crate::bindings::audio_capture::__FUNCTION_PTRS
                    .u_audio_capture_blueprint_library_get_available_audio_input_devices,
                __buffer,
            )
        };
        std::mem::forget(world_context_object);
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
                crate::bindings::audio_capture::__FUNCTION_PTRS
                    .u_audio_capture_blueprint_library_conv_audio_input_device_info_to_string,
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
                crate::bindings::audio_capture::__FUNCTION_PTRS
                    .u_audio_capture_blueprint_library_conv_audio_input_device_info_to_string,
                __buffer,
            )
        };
        unsafe { __buffer.add(48).cast::<FString>().read() }
    }
}
#[repr(C, align(16))]
pub struct UAudioCaptureComponent {
    #[doc(hidden)]
    pub(crate) __padding_2384: [u8; 2384],
    pub jitter_latency_frames: i32,
    __padding_end: [u8; 188],
}
impl UAudioCaptureComponent {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAudioCaptureComponent")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAudioCaptureComponent")
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
pub struct FGetAvailableAudioInputDevices_OnObtainDevicesEvent {
    _opague: [u8; 32],
}
