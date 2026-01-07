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
pub static mut U_TAKE_META_DATA_UNLOCK: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_TAKE_META_DATA_SET_TIMESTAMP: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_TAKE_META_DATA_SET_TIMECODE_OUT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_TAKE_META_DATA_SET_TIMECODE_IN: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_TAKE_META_DATA_SET_TAKE_NUMBER: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_TAKE_META_DATA_SET_SLATE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_TAKE_META_DATA_SET_PRESET_ORIGIN: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_TAKE_META_DATA_SET_LEVEL_ORIGIN: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_TAKE_META_DATA_SET_FRAME_RATE_FROM_TIMECODE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_TAKE_META_DATA_SET_FRAME_RATE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_TAKE_META_DATA_SET_DURATION: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_TAKE_META_DATA_SET_DESCRIPTION: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_TAKE_META_DATA_RECORDED: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_TAKE_META_DATA_LOCK: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_TAKE_META_DATA_IS_LOCKED: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_TAKE_META_DATA_GET_TIMESTAMP: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_TAKE_META_DATA_GET_TIMECODE_OUT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_TAKE_META_DATA_GET_TIMECODE_IN: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_TAKE_META_DATA_GET_TAKE_NUMBER: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_TAKE_META_DATA_GET_SLATE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_TAKE_META_DATA_GET_PRESET_ORIGIN: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_TAKE_META_DATA_GET_LEVEL_PATH: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_TAKE_META_DATA_GET_LEVEL_ORIGIN: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_TAKE_META_DATA_GET_FRAME_RATE_FROM_TIMECODE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_TAKE_META_DATA_GET_FRAME_RATE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_TAKE_META_DATA_GET_DURATION: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_TAKE_META_DATA_GET_DESCRIPTION: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_TAKE_META_DATA_GENERATE_ASSET_PATH: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_TAKE_RECORDER_SOURCES_START_RECORDING_SOURCE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_TAKE_RECORDER_SOURCES_REMOVE_SOURCE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_TAKE_RECORDER_SOURCES_GET_SOURCES_COPY: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_TAKE_RECORDER_SOURCES_ADD_SOURCE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_TAKES_CORE_BLUEPRINT_LIBRARY_SET_ON_TAKE_RECORDER_TAKE_NUMBER_CHANGED: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_TAKES_CORE_BLUEPRINT_LIBRARY_SET_ON_TAKE_RECORDER_SLATE_CHANGED: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_TAKES_CORE_BLUEPRINT_LIBRARY_ON_TAKE_RECORDER_TAKE_NUMBER_CHANGED_DELEGATE_SIGNATURE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_TAKES_CORE_BLUEPRINT_LIBRARY_ON_TAKE_RECORDER_SLATE_CHANGED_DELEGATE_SIGNATURE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_TAKES_CORE_BLUEPRINT_LIBRARY_FIND_TAKES: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_TAKES_CORE_BLUEPRINT_LIBRARY_COMPUTE_NEXT_TAKE_NUMBER: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
pub fn initialize() {
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UTakeMetaData::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("Unlock"),
            &raw mut U_TAKE_META_DATA_UNLOCK,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetTimestamp"),
            &raw mut U_TAKE_META_DATA_SET_TIMESTAMP,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetTimecodeOut"),
            &raw mut U_TAKE_META_DATA_SET_TIMECODE_OUT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetTimecodeIn"),
            &raw mut U_TAKE_META_DATA_SET_TIMECODE_IN,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetTakeNumber"),
            &raw mut U_TAKE_META_DATA_SET_TAKE_NUMBER,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetSlate"),
            &raw mut U_TAKE_META_DATA_SET_SLATE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetPresetOrigin"),
            &raw mut U_TAKE_META_DATA_SET_PRESET_ORIGIN,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetLevelOrigin"),
            &raw mut U_TAKE_META_DATA_SET_LEVEL_ORIGIN,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetFrameRateFromTimecode"),
            &raw mut U_TAKE_META_DATA_SET_FRAME_RATE_FROM_TIMECODE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetFrameRate"),
            &raw mut U_TAKE_META_DATA_SET_FRAME_RATE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetDuration"),
            &raw mut U_TAKE_META_DATA_SET_DURATION,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetDescription"),
            &raw mut U_TAKE_META_DATA_SET_DESCRIPTION,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("Recorded"),
            &raw mut U_TAKE_META_DATA_RECORDED,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("Lock"),
            &raw mut U_TAKE_META_DATA_LOCK,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("IsLocked"),
            &raw mut U_TAKE_META_DATA_IS_LOCKED,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetTimestamp"),
            &raw mut U_TAKE_META_DATA_GET_TIMESTAMP,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetTimecodeOut"),
            &raw mut U_TAKE_META_DATA_GET_TIMECODE_OUT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetTimecodeIn"),
            &raw mut U_TAKE_META_DATA_GET_TIMECODE_IN,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetTakeNumber"),
            &raw mut U_TAKE_META_DATA_GET_TAKE_NUMBER,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetSlate"),
            &raw mut U_TAKE_META_DATA_GET_SLATE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetPresetOrigin"),
            &raw mut U_TAKE_META_DATA_GET_PRESET_ORIGIN,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetLevelPath"),
            &raw mut U_TAKE_META_DATA_GET_LEVEL_PATH,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetLevelOrigin"),
            &raw mut U_TAKE_META_DATA_GET_LEVEL_ORIGIN,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetFrameRateFromTimecode"),
            &raw mut U_TAKE_META_DATA_GET_FRAME_RATE_FROM_TIMECODE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetFrameRate"),
            &raw mut U_TAKE_META_DATA_GET_FRAME_RATE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetDuration"),
            &raw mut U_TAKE_META_DATA_GET_DURATION,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetDescription"),
            &raw mut U_TAKE_META_DATA_GET_DESCRIPTION,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GenerateAssetPath"),
            &raw mut U_TAKE_META_DATA_GENERATE_ASSET_PATH,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UTakeRecorderSources::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("StartRecordingSource"),
            &raw mut U_TAKE_RECORDER_SOURCES_START_RECORDING_SOURCE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RemoveSource"),
            &raw mut U_TAKE_RECORDER_SOURCES_REMOVE_SOURCE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetSourcesCopy"),
            &raw mut U_TAKE_RECORDER_SOURCES_GET_SOURCES_COPY,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AddSource"),
            &raw mut U_TAKE_RECORDER_SOURCES_ADD_SOURCE,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UTakesCoreBlueprintLibrary::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetOnTakeRecorderTakeNumberChanged"),
            &raw mut U_TAKES_CORE_BLUEPRINT_LIBRARY_SET_ON_TAKE_RECORDER_TAKE_NUMBER_CHANGED,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetOnTakeRecorderSlateChanged"),
            &raw mut U_TAKES_CORE_BLUEPRINT_LIBRARY_SET_ON_TAKE_RECORDER_SLATE_CHANGED,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from(
                "OnTakeRecorderTakeNumberChanged__DelegateSignature",
            ),
            &raw mut U_TAKES_CORE_BLUEPRINT_LIBRARY_ON_TAKE_RECORDER_TAKE_NUMBER_CHANGED_DELEGATE_SIGNATURE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("OnTakeRecorderSlateChanged__DelegateSignature"),
            &raw mut U_TAKES_CORE_BLUEPRINT_LIBRARY_ON_TAKE_RECORDER_SLATE_CHANGED_DELEGATE_SIGNATURE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("FindTakes"),
            &raw mut U_TAKES_CORE_BLUEPRINT_LIBRARY_FIND_TAKES,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ComputeNextTakeNumber"),
            &raw mut U_TAKES_CORE_BLUEPRINT_LIBRARY_COMPUTE_NEXT_TAKE_NUMBER,
        );
    }
}
#[repr(C, align(8))]
pub struct FActorRecordedProperty {
    pub property_name: FName,
    pub b_enabled: bool,
    __padding_end: [u8; 19],
}
impl FActorRecordedProperty {}
#[repr(C, align(8))]
pub struct FAudioInputDeviceInfoProperty {
    pub device_name: FString,
    pub device_id: FString,
    pub input_channels: i32,
    pub preferred_sample_rate: i32,
    pub b_is_default_device: bool,
}
impl FAudioInputDeviceInfoProperty {}
#[repr(C, align(8))]
pub struct FAudioInputDeviceProperty {
    pub b_use_system_default_audio_device: bool,
    pub device_info_array: TArray<FAudioInputDeviceInfoProperty>,
    pub device_id: FString,
    pub audio_input_buffer_size: i32,
}
impl FAudioInputDeviceProperty {}
#[repr(C, align(4))]
pub struct FAudioInputDeviceChannelProperty {
    pub audio_input_device_channel: i32,
}
impl FAudioInputDeviceChannelProperty {}
#[repr(C, align(8))]
pub struct UTakeRecorderNamingTokensContext {
    __padding_end: [u8; 72],
}
impl UTakeRecorderNamingTokensContext {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UTakeRecorderNamingTokensContext")
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
pub struct UTakeMetaData {
    __padding_end: [u8; 280],
}
impl UTakeMetaData {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UTakeMetaData")
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
    pub fn unlock(&mut self) {
        let mut __stack = crate::core_data::StackAlloc::<0>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::takes_core::U_TAKE_META_DATA_UNLOCK,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::takes_core::U_TAKE_META_DATA_UNLOCK,
                __buffer,
            )
        };
    }
    pub fn set_timestamp(
        &mut self,
        in_timestamp: crate::bindings::core_u_object::FDateTime,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::takes_core::U_TAKE_META_DATA_SET_TIMESTAMP,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_timestamp,
                __buffer.add(0).cast::<crate::bindings::core_u_object::FDateTime>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::takes_core::U_TAKE_META_DATA_SET_TIMESTAMP,
                __buffer,
            )
        };
    }
    pub fn set_timecode_out(
        &mut self,
        in_timecode_out: crate::bindings::core_u_object::FTimecode,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::takes_core::U_TAKE_META_DATA_SET_TIMECODE_OUT,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_timecode_out,
                __buffer.add(0).cast::<crate::bindings::core_u_object::FTimecode>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::takes_core::U_TAKE_META_DATA_SET_TIMECODE_OUT,
                __buffer,
            )
        };
    }
    pub fn set_timecode_in(
        &mut self,
        in_timecode_in: crate::bindings::core_u_object::FTimecode,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::takes_core::U_TAKE_META_DATA_SET_TIMECODE_IN,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_timecode_in,
                __buffer.add(0).cast::<crate::bindings::core_u_object::FTimecode>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::takes_core::U_TAKE_META_DATA_SET_TIMECODE_IN,
                __buffer,
            )
        };
    }
    pub fn set_take_number(&mut self, in_take_number: i32, b_emit_changed: bool) {
        let mut __stack = crate::core_data::StackAlloc::<5>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::takes_core::U_TAKE_META_DATA_SET_TAKE_NUMBER,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_take_number,
                __buffer.add(0).cast::<i32>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_emit_changed,
                __buffer.add(4).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::takes_core::U_TAKE_META_DATA_SET_TAKE_NUMBER,
                __buffer,
            )
        };
    }
    pub fn set_slate(&mut self, in_slate: FString, b_emit_changed: bool) {
        let mut __stack = crate::core_data::StackAlloc::<17>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::takes_core::U_TAKE_META_DATA_SET_SLATE,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_slate,
                __buffer.add(0).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_emit_changed,
                __buffer.add(16).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::takes_core::U_TAKE_META_DATA_SET_SLATE,
                __buffer,
            )
        };
    }
    pub fn set_preset_origin(&mut self, in_preset_origin: UPtr<UTakePreset>) {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::takes_core::U_TAKE_META_DATA_SET_PRESET_ORIGIN,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_preset_origin,
                __buffer.add(0).cast::<UPtr<UTakePreset>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::takes_core::U_TAKE_META_DATA_SET_PRESET_ORIGIN,
                __buffer,
            )
        };
    }
    pub fn set_level_origin(
        &mut self,
        in_level_origin: UPtr<crate::bindings::engine::ULevel>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::takes_core::U_TAKE_META_DATA_SET_LEVEL_ORIGIN,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_level_origin,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::ULevel>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::takes_core::U_TAKE_META_DATA_SET_LEVEL_ORIGIN,
                __buffer,
            )
        };
    }
    pub fn set_frame_rate_from_timecode(&mut self, in_from_timecode: bool) {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::takes_core::U_TAKE_META_DATA_SET_FRAME_RATE_FROM_TIMECODE,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_from_timecode,
                __buffer.add(0).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::takes_core::U_TAKE_META_DATA_SET_FRAME_RATE_FROM_TIMECODE,
                __buffer,
            )
        };
    }
    pub fn set_frame_rate(
        &mut self,
        in_frame_rate: crate::bindings::core_u_object::FFrameRate,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::takes_core::U_TAKE_META_DATA_SET_FRAME_RATE,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_frame_rate,
                __buffer.add(0).cast::<crate::bindings::core_u_object::FFrameRate>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::takes_core::U_TAKE_META_DATA_SET_FRAME_RATE,
                __buffer,
            )
        };
    }
    pub fn set_duration(
        &mut self,
        in_duration: crate::bindings::core_u_object::FFrameTime,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::takes_core::U_TAKE_META_DATA_SET_DURATION,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_duration,
                __buffer.add(0).cast::<crate::bindings::core_u_object::FFrameTime>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::takes_core::U_TAKE_META_DATA_SET_DURATION,
                __buffer,
            )
        };
    }
    pub fn set_description(&mut self, in_description: FString) {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::takes_core::U_TAKE_META_DATA_SET_DESCRIPTION,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_description,
                __buffer.add(0).cast::<FString>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::takes_core::U_TAKE_META_DATA_SET_DESCRIPTION,
                __buffer,
            )
        };
    }
    pub fn recorded(&self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::takes_core::U_TAKE_META_DATA_RECORDED,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::takes_core::U_TAKE_META_DATA_RECORDED,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn lock(&mut self) {
        let mut __stack = crate::core_data::StackAlloc::<0>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::takes_core::U_TAKE_META_DATA_LOCK,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::takes_core::U_TAKE_META_DATA_LOCK,
                __buffer,
            )
        };
    }
    pub fn is_locked(&self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::takes_core::U_TAKE_META_DATA_IS_LOCKED,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::takes_core::U_TAKE_META_DATA_IS_LOCKED,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn get_timestamp(&self) -> crate::bindings::core_u_object::FDateTime {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::takes_core::U_TAKE_META_DATA_GET_TIMESTAMP,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::takes_core::U_TAKE_META_DATA_GET_TIMESTAMP,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<crate::bindings::core_u_object::FDateTime>().read()
        }
    }
    pub fn get_timecode_out(&self) -> crate::bindings::core_u_object::FTimecode {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::takes_core::U_TAKE_META_DATA_GET_TIMECODE_OUT,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::takes_core::U_TAKE_META_DATA_GET_TIMECODE_OUT,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<crate::bindings::core_u_object::FTimecode>().read()
        }
    }
    pub fn get_timecode_in(&self) -> crate::bindings::core_u_object::FTimecode {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::takes_core::U_TAKE_META_DATA_GET_TIMECODE_IN,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::takes_core::U_TAKE_META_DATA_GET_TIMECODE_IN,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<crate::bindings::core_u_object::FTimecode>().read()
        }
    }
    pub fn get_take_number(&self) -> i32 {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::takes_core::U_TAKE_META_DATA_GET_TAKE_NUMBER,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::takes_core::U_TAKE_META_DATA_GET_TAKE_NUMBER,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<i32>().read() }
    }
    pub fn get_slate(&self) -> FString {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::takes_core::U_TAKE_META_DATA_GET_SLATE,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::takes_core::U_TAKE_META_DATA_GET_SLATE,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<FString>().read() }
    }
    pub fn get_preset_origin(&self) -> UPtr<UTakePreset> {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::takes_core::U_TAKE_META_DATA_GET_PRESET_ORIGIN,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::takes_core::U_TAKE_META_DATA_GET_PRESET_ORIGIN,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<UPtr<UTakePreset>>().read() }
    }
    pub fn get_level_path(&self) -> FString {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::takes_core::U_TAKE_META_DATA_GET_LEVEL_PATH,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::takes_core::U_TAKE_META_DATA_GET_LEVEL_PATH,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<FString>().read() }
    }
    pub fn get_level_origin(&self) -> UPtr<crate::bindings::engine::ULevel> {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::takes_core::U_TAKE_META_DATA_GET_LEVEL_ORIGIN,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::takes_core::U_TAKE_META_DATA_GET_LEVEL_ORIGIN,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<UPtr<crate::bindings::engine::ULevel>>().read() }
    }
    pub fn get_frame_rate_from_timecode(&self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::takes_core::U_TAKE_META_DATA_GET_FRAME_RATE_FROM_TIMECODE,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::takes_core::U_TAKE_META_DATA_GET_FRAME_RATE_FROM_TIMECODE,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn get_frame_rate(&mut self) -> crate::bindings::core_u_object::FFrameRate {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::takes_core::U_TAKE_META_DATA_GET_FRAME_RATE,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::takes_core::U_TAKE_META_DATA_GET_FRAME_RATE,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<crate::bindings::core_u_object::FFrameRate>().read()
        }
    }
    pub fn get_duration(&self) -> crate::bindings::core_u_object::FFrameTime {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::takes_core::U_TAKE_META_DATA_GET_DURATION,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::takes_core::U_TAKE_META_DATA_GET_DURATION,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<crate::bindings::core_u_object::FFrameTime>().read()
        }
    }
    pub fn get_description(&self) -> FString {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::takes_core::U_TAKE_META_DATA_GET_DESCRIPTION,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::takes_core::U_TAKE_META_DATA_GET_DESCRIPTION,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<FString>().read() }
    }
    pub fn generate_asset_path(
        &self,
        path_format_string: FString,
        in_context: UPtr<UTakeRecorderNamingTokensContext>,
    ) -> FString {
        let mut __stack = crate::core_data::StackAlloc::<40>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::takes_core::U_TAKE_META_DATA_GENERATE_ASSET_PATH,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &path_format_string,
                __buffer.add(0).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_context,
                __buffer.add(16).cast::<UPtr<UTakeRecorderNamingTokensContext>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::takes_core::U_TAKE_META_DATA_GENERATE_ASSET_PATH,
                __buffer,
            )
        };
        unsafe { __buffer.add(24).cast::<FString>().read() }
    }
}
#[repr(C, align(8))]
pub struct UTakePreset {
    __padding_end: [u8; 88],
}
impl UTakePreset {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UTakePreset")
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
pub struct UTakePresetSettings {
    __padding_end: [u8; 128],
}
impl UTakePresetSettings {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UTakePresetSettings")
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
pub struct UTakeRecorderSource {
    #[doc(hidden)]
    __padding_48: [u8; 48],
    pub b_enabled: bool,
    pub take_number: i32,
    pub track_tint: crate::bindings::core_u_object::FColor,
}
impl UTakeRecorderSource {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UTakeRecorderSource")
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
pub struct UActorRecorderPropertyMap {
    #[doc(hidden)]
    __padding_48: [u8; 48],
    pub recorded_object: TSoftObjectPtr<crate::bindings::core_u_object::UObject>,
    pub properties: TArray<FActorRecordedProperty>,
    pub children: TArray<UPtr<UActorRecorderPropertyMap>>,
    __padding_end: [u8; 16],
}
impl UActorRecorderPropertyMap {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UActorRecorderPropertyMap")
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
pub struct UTakeRecorderAudioInputSettings {
    __padding_end: [u8; 80],
}
impl UTakeRecorderAudioInputSettings {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UTakeRecorderAudioInputSettings")
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
pub struct UTakeRecorderSources {
    __padding_end: [u8; 416],
}
impl UTakeRecorderSources {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UTakeRecorderSources")
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
    pub fn start_recording_source(
        &mut self,
        in_sources: TArray<UPtr<UTakeRecorderSource>>,
        current_frame_time: &crate::bindings::core_u_object::FQualifiedFrameTime,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::takes_core::U_TAKE_RECORDER_SOURCES_START_RECORDING_SOURCE,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_sources,
                __buffer.add(0).cast::<TArray<UPtr<UTakeRecorderSource>>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                current_frame_time,
                __buffer
                    .add(16)
                    .cast::<crate::bindings::core_u_object::FQualifiedFrameTime>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::takes_core::U_TAKE_RECORDER_SOURCES_START_RECORDING_SOURCE,
                __buffer,
            )
        };
    }
    pub fn remove_source(&mut self, in_source: UPtr<UTakeRecorderSource>) {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::takes_core::U_TAKE_RECORDER_SOURCES_REMOVE_SOURCE,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_source,
                __buffer.add(0).cast::<UPtr<UTakeRecorderSource>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::takes_core::U_TAKE_RECORDER_SOURCES_REMOVE_SOURCE,
                __buffer,
            )
        };
    }
    pub fn get_sources_copy(&self) -> TArray<UPtr<UTakeRecorderSource>> {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::takes_core::U_TAKE_RECORDER_SOURCES_GET_SOURCES_COPY,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::takes_core::U_TAKE_RECORDER_SOURCES_GET_SOURCES_COPY,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<TArray<UPtr<UTakeRecorderSource>>>().read() }
    }
    pub fn add_source(
        &mut self,
        in_source_type: TSubclassOf<UTakeRecorderSource>,
    ) -> UPtr<UTakeRecorderSource> {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::takes_core::U_TAKE_RECORDER_SOURCES_ADD_SOURCE,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_source_type,
                __buffer.add(0).cast::<TSubclassOf<UTakeRecorderSource>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::takes_core::U_TAKE_RECORDER_SOURCES_ADD_SOURCE,
                __buffer,
            )
        };
        unsafe { __buffer.add(8).cast::<UPtr<UTakeRecorderSource>>().read() }
    }
}
#[repr(C, align(8))]
pub struct UTakesCoreBlueprintLibrary {
    __padding_end: [u8; 48],
}
impl UTakesCoreBlueprintLibrary {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UTakesCoreBlueprintLibrary")
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
    pub fn set_on_take_recorder_take_number_changed(
        on_take_recorder_take_number_changed: FSetOnTakeRecorderTakeNumberChanged_OnTakeRecorderTakeNumberChanged,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::takes_core::U_TAKES_CORE_BLUEPRINT_LIBRARY_SET_ON_TAKE_RECORDER_TAKE_NUMBER_CHANGED,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &on_take_recorder_take_number_changed,
                __buffer
                    .add(0)
                    .cast::<
                        FSetOnTakeRecorderTakeNumberChanged_OnTakeRecorderTakeNumberChanged,
                    >(),
                1,
            );
        }
        let __object_ptr = crate::bindings::takes_core::UTakesCoreBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::takes_core::U_TAKES_CORE_BLUEPRINT_LIBRARY_SET_ON_TAKE_RECORDER_TAKE_NUMBER_CHANGED,
                __buffer,
            )
        };
    }
    pub fn set_on_take_recorder_slate_changed(
        on_take_recorder_slate_changed: FSetOnTakeRecorderSlateChanged_OnTakeRecorderSlateChanged,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::takes_core::U_TAKES_CORE_BLUEPRINT_LIBRARY_SET_ON_TAKE_RECORDER_SLATE_CHANGED,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &on_take_recorder_slate_changed,
                __buffer
                    .add(0)
                    .cast::<FSetOnTakeRecorderSlateChanged_OnTakeRecorderSlateChanged>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::takes_core::UTakesCoreBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::takes_core::U_TAKES_CORE_BLUEPRINT_LIBRARY_SET_ON_TAKE_RECORDER_SLATE_CHANGED,
                __buffer,
            )
        };
    }
    pub fn find_takes(
        slate: FString,
        take_number: i32,
    ) -> TArray<crate::bindings::core_u_object::FAssetData> {
        let mut __stack = crate::core_data::StackAlloc::<40>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::takes_core::U_TAKES_CORE_BLUEPRINT_LIBRARY_FIND_TAKES,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&slate, __buffer.add(0).cast::<FString>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &take_number,
                __buffer.add(16).cast::<i32>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::takes_core::UTakesCoreBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::takes_core::U_TAKES_CORE_BLUEPRINT_LIBRARY_FIND_TAKES,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(24)
                .cast::<TArray<crate::bindings::core_u_object::FAssetData>>()
                .read()
        }
    }
    pub fn compute_next_take_number(slate: FString) -> i32 {
        let mut __stack = crate::core_data::StackAlloc::<20>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::takes_core::U_TAKES_CORE_BLUEPRINT_LIBRARY_COMPUTE_NEXT_TAKE_NUMBER,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&slate, __buffer.add(0).cast::<FString>(), 1);
        }
        let __object_ptr = crate::bindings::takes_core::UTakesCoreBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::takes_core::U_TAKES_CORE_BLUEPRINT_LIBRARY_COMPUTE_NEXT_TAKE_NUMBER,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<i32>().read() }
    }
}
#[repr(C, align(8))]
pub struct FSetOnTakeRecorderSlateChanged_OnTakeRecorderSlateChanged {
    _opague: [u8; 32],
}
#[repr(C, align(8))]
pub struct FSetOnTakeRecorderTakeNumberChanged_OnTakeRecorderTakeNumberChanged {
    _opague: [u8; 32],
}
