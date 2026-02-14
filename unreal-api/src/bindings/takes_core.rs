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
    pub u_take_meta_data_unlock: *mut crate::ffi::UFunctionOpague,
    pub u_take_meta_data_set_timestamp: *mut crate::ffi::UFunctionOpague,
    pub u_take_meta_data_set_timecode_out: *mut crate::ffi::UFunctionOpague,
    pub u_take_meta_data_set_timecode_in: *mut crate::ffi::UFunctionOpague,
    pub u_take_meta_data_set_take_number: *mut crate::ffi::UFunctionOpague,
    pub u_take_meta_data_set_slate: *mut crate::ffi::UFunctionOpague,
    pub u_take_meta_data_set_preset_origin: *mut crate::ffi::UFunctionOpague,
    pub u_take_meta_data_set_level_origin: *mut crate::ffi::UFunctionOpague,
    pub u_take_meta_data_set_frame_rate_from_timecode: *mut crate::ffi::UFunctionOpague,
    pub u_take_meta_data_set_frame_rate: *mut crate::ffi::UFunctionOpague,
    pub u_take_meta_data_set_duration: *mut crate::ffi::UFunctionOpague,
    pub u_take_meta_data_set_description: *mut crate::ffi::UFunctionOpague,
    pub u_take_meta_data_recorded: *mut crate::ffi::UFunctionOpague,
    pub u_take_meta_data_lock: *mut crate::ffi::UFunctionOpague,
    pub u_take_meta_data_is_locked: *mut crate::ffi::UFunctionOpague,
    pub u_take_meta_data_get_timestamp: *mut crate::ffi::UFunctionOpague,
    pub u_take_meta_data_get_timecode_out: *mut crate::ffi::UFunctionOpague,
    pub u_take_meta_data_get_timecode_in: *mut crate::ffi::UFunctionOpague,
    pub u_take_meta_data_get_take_number: *mut crate::ffi::UFunctionOpague,
    pub u_take_meta_data_get_slate: *mut crate::ffi::UFunctionOpague,
    pub u_take_meta_data_get_preset_origin: *mut crate::ffi::UFunctionOpague,
    pub u_take_meta_data_get_level_path: *mut crate::ffi::UFunctionOpague,
    pub u_take_meta_data_get_level_origin: *mut crate::ffi::UFunctionOpague,
    pub u_take_meta_data_get_frame_rate_from_timecode: *mut crate::ffi::UFunctionOpague,
    pub u_take_meta_data_get_frame_rate: *mut crate::ffi::UFunctionOpague,
    pub u_take_meta_data_get_duration: *mut crate::ffi::UFunctionOpague,
    pub u_take_meta_data_get_description: *mut crate::ffi::UFunctionOpague,
    pub u_take_meta_data_generate_asset_path: *mut crate::ffi::UFunctionOpague,
    pub u_take_recorder_sources_start_recording_source: *mut crate::ffi::UFunctionOpague,
    pub u_take_recorder_sources_remove_source: *mut crate::ffi::UFunctionOpague,
    pub u_take_recorder_sources_get_sources_copy: *mut crate::ffi::UFunctionOpague,
    pub u_take_recorder_sources_add_source: *mut crate::ffi::UFunctionOpague,
    pub u_takes_core_blueprint_library_set_on_take_recorder_take_number_changed: *mut crate::ffi::UFunctionOpague,
    pub u_takes_core_blueprint_library_set_on_take_recorder_slate_changed: *mut crate::ffi::UFunctionOpague,
    pub u_takes_core_blueprint_library_on_take_recorder_take_number_changed_delegate_signature: *mut crate::ffi::UFunctionOpague,
    pub u_takes_core_blueprint_library_on_take_recorder_slate_changed_delegate_signature: *mut crate::ffi::UFunctionOpague,
    pub u_takes_core_blueprint_library_find_takes: *mut crate::ffi::UFunctionOpague,
    pub u_takes_core_blueprint_library_compute_next_take_number: *mut crate::ffi::UFunctionOpague,
}
impl FunctionPtrs {
    pub const fn empty() -> Self {
        Self {
            u_take_meta_data_unlock: std::ptr::null_mut(),
            u_take_meta_data_set_timestamp: std::ptr::null_mut(),
            u_take_meta_data_set_timecode_out: std::ptr::null_mut(),
            u_take_meta_data_set_timecode_in: std::ptr::null_mut(),
            u_take_meta_data_set_take_number: std::ptr::null_mut(),
            u_take_meta_data_set_slate: std::ptr::null_mut(),
            u_take_meta_data_set_preset_origin: std::ptr::null_mut(),
            u_take_meta_data_set_level_origin: std::ptr::null_mut(),
            u_take_meta_data_set_frame_rate_from_timecode: std::ptr::null_mut(),
            u_take_meta_data_set_frame_rate: std::ptr::null_mut(),
            u_take_meta_data_set_duration: std::ptr::null_mut(),
            u_take_meta_data_set_description: std::ptr::null_mut(),
            u_take_meta_data_recorded: std::ptr::null_mut(),
            u_take_meta_data_lock: std::ptr::null_mut(),
            u_take_meta_data_is_locked: std::ptr::null_mut(),
            u_take_meta_data_get_timestamp: std::ptr::null_mut(),
            u_take_meta_data_get_timecode_out: std::ptr::null_mut(),
            u_take_meta_data_get_timecode_in: std::ptr::null_mut(),
            u_take_meta_data_get_take_number: std::ptr::null_mut(),
            u_take_meta_data_get_slate: std::ptr::null_mut(),
            u_take_meta_data_get_preset_origin: std::ptr::null_mut(),
            u_take_meta_data_get_level_path: std::ptr::null_mut(),
            u_take_meta_data_get_level_origin: std::ptr::null_mut(),
            u_take_meta_data_get_frame_rate_from_timecode: std::ptr::null_mut(),
            u_take_meta_data_get_frame_rate: std::ptr::null_mut(),
            u_take_meta_data_get_duration: std::ptr::null_mut(),
            u_take_meta_data_get_description: std::ptr::null_mut(),
            u_take_meta_data_generate_asset_path: std::ptr::null_mut(),
            u_take_recorder_sources_start_recording_source: std::ptr::null_mut(),
            u_take_recorder_sources_remove_source: std::ptr::null_mut(),
            u_take_recorder_sources_get_sources_copy: std::ptr::null_mut(),
            u_take_recorder_sources_add_source: std::ptr::null_mut(),
            u_takes_core_blueprint_library_set_on_take_recorder_take_number_changed: std::ptr::null_mut(),
            u_takes_core_blueprint_library_set_on_take_recorder_slate_changed: std::ptr::null_mut(),
            u_takes_core_blueprint_library_on_take_recorder_take_number_changed_delegate_signature: std::ptr::null_mut(),
            u_takes_core_blueprint_library_on_take_recorder_slate_changed_delegate_signature: std::ptr::null_mut(),
            u_takes_core_blueprint_library_find_takes: std::ptr::null_mut(),
            u_takes_core_blueprint_library_compute_next_take_number: std::ptr::null_mut(),
        }
    }
}
pub fn initialize() {
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UTakeMetaData::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("Unlock"),
                &raw mut __FUNCTION_PTRS.u_take_meta_data_unlock,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetTimestamp"),
                &raw mut __FUNCTION_PTRS.u_take_meta_data_set_timestamp,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetTimecodeOut"),
                &raw mut __FUNCTION_PTRS.u_take_meta_data_set_timecode_out,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetTimecodeIn"),
                &raw mut __FUNCTION_PTRS.u_take_meta_data_set_timecode_in,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetTakeNumber"),
                &raw mut __FUNCTION_PTRS.u_take_meta_data_set_take_number,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetSlate"),
                &raw mut __FUNCTION_PTRS.u_take_meta_data_set_slate,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetPresetOrigin"),
                &raw mut __FUNCTION_PTRS.u_take_meta_data_set_preset_origin,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetLevelOrigin"),
                &raw mut __FUNCTION_PTRS.u_take_meta_data_set_level_origin,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetFrameRateFromTimecode"),
                &raw mut __FUNCTION_PTRS.u_take_meta_data_set_frame_rate_from_timecode,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetFrameRate"),
                &raw mut __FUNCTION_PTRS.u_take_meta_data_set_frame_rate,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetDuration"),
                &raw mut __FUNCTION_PTRS.u_take_meta_data_set_duration,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetDescription"),
                &raw mut __FUNCTION_PTRS.u_take_meta_data_set_description,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("Recorded"),
                &raw mut __FUNCTION_PTRS.u_take_meta_data_recorded,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("Lock"),
                &raw mut __FUNCTION_PTRS.u_take_meta_data_lock,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("IsLocked"),
                &raw mut __FUNCTION_PTRS.u_take_meta_data_is_locked,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetTimestamp"),
                &raw mut __FUNCTION_PTRS.u_take_meta_data_get_timestamp,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetTimecodeOut"),
                &raw mut __FUNCTION_PTRS.u_take_meta_data_get_timecode_out,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetTimecodeIn"),
                &raw mut __FUNCTION_PTRS.u_take_meta_data_get_timecode_in,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetTakeNumber"),
                &raw mut __FUNCTION_PTRS.u_take_meta_data_get_take_number,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetSlate"),
                &raw mut __FUNCTION_PTRS.u_take_meta_data_get_slate,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetPresetOrigin"),
                &raw mut __FUNCTION_PTRS.u_take_meta_data_get_preset_origin,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetLevelPath"),
                &raw mut __FUNCTION_PTRS.u_take_meta_data_get_level_path,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetLevelOrigin"),
                &raw mut __FUNCTION_PTRS.u_take_meta_data_get_level_origin,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetFrameRateFromTimecode"),
                &raw mut __FUNCTION_PTRS.u_take_meta_data_get_frame_rate_from_timecode,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetFrameRate"),
                &raw mut __FUNCTION_PTRS.u_take_meta_data_get_frame_rate,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetDuration"),
                &raw mut __FUNCTION_PTRS.u_take_meta_data_get_duration,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetDescription"),
                &raw mut __FUNCTION_PTRS.u_take_meta_data_get_description,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GenerateAssetPath"),
                &raw mut __FUNCTION_PTRS.u_take_meta_data_generate_asset_path,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UTakeRecorderSources::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("StartRecordingSource"),
                &raw mut __FUNCTION_PTRS.u_take_recorder_sources_start_recording_source,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("RemoveSource"),
                &raw mut __FUNCTION_PTRS.u_take_recorder_sources_remove_source,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetSourcesCopy"),
                &raw mut __FUNCTION_PTRS.u_take_recorder_sources_get_sources_copy,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("AddSource"),
                &raw mut __FUNCTION_PTRS.u_take_recorder_sources_add_source,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UTakesCoreBlueprintLibrary::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetOnTakeRecorderTakeNumberChanged"),
                &raw mut __FUNCTION_PTRS
                    .u_takes_core_blueprint_library_set_on_take_recorder_take_number_changed,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetOnTakeRecorderSlateChanged"),
                &raw mut __FUNCTION_PTRS
                    .u_takes_core_blueprint_library_set_on_take_recorder_slate_changed,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from(
                    "OnTakeRecorderTakeNumberChanged__DelegateSignature",
                ),
                &raw mut __FUNCTION_PTRS
                    .u_takes_core_blueprint_library_on_take_recorder_take_number_changed_delegate_signature,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from(
                    "OnTakeRecorderSlateChanged__DelegateSignature",
                ),
                &raw mut __FUNCTION_PTRS
                    .u_takes_core_blueprint_library_on_take_recorder_slate_changed_delegate_signature,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("FindTakes"),
                &raw mut __FUNCTION_PTRS.u_takes_core_blueprint_library_find_takes,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ComputeNextTakeNumber"),
                &raw mut __FUNCTION_PTRS
                    .u_takes_core_blueprint_library_compute_next_take_number,
            );
        }
    }
}
#[repr(C, align(8))]
pub struct FActorRecordedProperty {
    pub property_name: FName,
    pub b_enabled: bool,
    pub(crate) __padding_end: [u8; 19],
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
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UTakeRecorderNamingTokensContext")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UTakeRecorderNamingTokensContext")
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
pub struct UTakeMetaData {
    __padding_end: [u8; 280],
}
impl UTakeMetaData {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UTakeMetaData")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UTakeMetaData")
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
    pub fn unlock(&mut self) {
        let mut __stack = crate::core_data::StackAlloc::<0>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::takes_core::__FUNCTION_PTRS.u_take_meta_data_unlock,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::takes_core::__FUNCTION_PTRS.u_take_meta_data_unlock,
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
                crate::bindings::takes_core::__FUNCTION_PTRS
                    .u_take_meta_data_set_timestamp,
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
                crate::bindings::takes_core::__FUNCTION_PTRS
                    .u_take_meta_data_set_timestamp,
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
                crate::bindings::takes_core::__FUNCTION_PTRS
                    .u_take_meta_data_set_timecode_out,
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
                crate::bindings::takes_core::__FUNCTION_PTRS
                    .u_take_meta_data_set_timecode_out,
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
                crate::bindings::takes_core::__FUNCTION_PTRS
                    .u_take_meta_data_set_timecode_in,
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
                crate::bindings::takes_core::__FUNCTION_PTRS
                    .u_take_meta_data_set_timecode_in,
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
                crate::bindings::takes_core::__FUNCTION_PTRS
                    .u_take_meta_data_set_take_number,
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
                crate::bindings::takes_core::__FUNCTION_PTRS
                    .u_take_meta_data_set_take_number,
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
                crate::bindings::takes_core::__FUNCTION_PTRS.u_take_meta_data_set_slate,
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
                crate::bindings::takes_core::__FUNCTION_PTRS.u_take_meta_data_set_slate,
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
                crate::bindings::takes_core::__FUNCTION_PTRS
                    .u_take_meta_data_set_preset_origin,
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
                crate::bindings::takes_core::__FUNCTION_PTRS
                    .u_take_meta_data_set_preset_origin,
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
                crate::bindings::takes_core::__FUNCTION_PTRS
                    .u_take_meta_data_set_level_origin,
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
                crate::bindings::takes_core::__FUNCTION_PTRS
                    .u_take_meta_data_set_level_origin,
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
                crate::bindings::takes_core::__FUNCTION_PTRS
                    .u_take_meta_data_set_frame_rate_from_timecode,
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
                crate::bindings::takes_core::__FUNCTION_PTRS
                    .u_take_meta_data_set_frame_rate_from_timecode,
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
                crate::bindings::takes_core::__FUNCTION_PTRS
                    .u_take_meta_data_set_frame_rate,
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
                crate::bindings::takes_core::__FUNCTION_PTRS
                    .u_take_meta_data_set_frame_rate,
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
                crate::bindings::takes_core::__FUNCTION_PTRS
                    .u_take_meta_data_set_duration,
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
                crate::bindings::takes_core::__FUNCTION_PTRS
                    .u_take_meta_data_set_duration,
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
                crate::bindings::takes_core::__FUNCTION_PTRS
                    .u_take_meta_data_set_description,
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
                crate::bindings::takes_core::__FUNCTION_PTRS
                    .u_take_meta_data_set_description,
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
                crate::bindings::takes_core::__FUNCTION_PTRS.u_take_meta_data_recorded,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::takes_core::__FUNCTION_PTRS.u_take_meta_data_recorded,
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
                crate::bindings::takes_core::__FUNCTION_PTRS.u_take_meta_data_lock,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::takes_core::__FUNCTION_PTRS.u_take_meta_data_lock,
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
                crate::bindings::takes_core::__FUNCTION_PTRS.u_take_meta_data_is_locked,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::takes_core::__FUNCTION_PTRS.u_take_meta_data_is_locked,
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
                crate::bindings::takes_core::__FUNCTION_PTRS
                    .u_take_meta_data_get_timestamp,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::takes_core::__FUNCTION_PTRS
                    .u_take_meta_data_get_timestamp,
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
                crate::bindings::takes_core::__FUNCTION_PTRS
                    .u_take_meta_data_get_timecode_out,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::takes_core::__FUNCTION_PTRS
                    .u_take_meta_data_get_timecode_out,
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
                crate::bindings::takes_core::__FUNCTION_PTRS
                    .u_take_meta_data_get_timecode_in,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::takes_core::__FUNCTION_PTRS
                    .u_take_meta_data_get_timecode_in,
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
                crate::bindings::takes_core::__FUNCTION_PTRS
                    .u_take_meta_data_get_take_number,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::takes_core::__FUNCTION_PTRS
                    .u_take_meta_data_get_take_number,
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
                crate::bindings::takes_core::__FUNCTION_PTRS.u_take_meta_data_get_slate,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::takes_core::__FUNCTION_PTRS.u_take_meta_data_get_slate,
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
                crate::bindings::takes_core::__FUNCTION_PTRS
                    .u_take_meta_data_get_preset_origin,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::takes_core::__FUNCTION_PTRS
                    .u_take_meta_data_get_preset_origin,
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
                crate::bindings::takes_core::__FUNCTION_PTRS
                    .u_take_meta_data_get_level_path,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::takes_core::__FUNCTION_PTRS
                    .u_take_meta_data_get_level_path,
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
                crate::bindings::takes_core::__FUNCTION_PTRS
                    .u_take_meta_data_get_level_origin,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::takes_core::__FUNCTION_PTRS
                    .u_take_meta_data_get_level_origin,
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
                crate::bindings::takes_core::__FUNCTION_PTRS
                    .u_take_meta_data_get_frame_rate_from_timecode,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::takes_core::__FUNCTION_PTRS
                    .u_take_meta_data_get_frame_rate_from_timecode,
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
                crate::bindings::takes_core::__FUNCTION_PTRS
                    .u_take_meta_data_get_frame_rate,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::takes_core::__FUNCTION_PTRS
                    .u_take_meta_data_get_frame_rate,
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
                crate::bindings::takes_core::__FUNCTION_PTRS
                    .u_take_meta_data_get_duration,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::takes_core::__FUNCTION_PTRS
                    .u_take_meta_data_get_duration,
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
                crate::bindings::takes_core::__FUNCTION_PTRS
                    .u_take_meta_data_get_description,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::takes_core::__FUNCTION_PTRS
                    .u_take_meta_data_get_description,
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
                crate::bindings::takes_core::__FUNCTION_PTRS
                    .u_take_meta_data_generate_asset_path,
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
                crate::bindings::takes_core::__FUNCTION_PTRS
                    .u_take_meta_data_generate_asset_path,
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
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UTakePreset")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UTakePreset")
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
pub struct UTakePresetSettings {
    __padding_end: [u8; 128],
}
impl UTakePresetSettings {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UTakePresetSettings")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UTakePresetSettings")
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
pub struct UTakeRecorderSource {
    #[doc(hidden)]
    pub(crate) __padding_48: [u8; 48],
    pub b_enabled: bool,
    pub take_number: i32,
    pub track_tint: crate::bindings::core_u_object::FColor,
}
impl UTakeRecorderSource {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UTakeRecorderSource")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UTakeRecorderSource")
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
pub struct UActorRecorderPropertyMap {
    #[doc(hidden)]
    pub(crate) __padding_48: [u8; 48],
    pub recorded_object: TSoftObjectPtr<crate::bindings::core_u_object::UObject>,
    pub properties: TArray<FActorRecordedProperty>,
    pub children: TArray<UPtr<UActorRecorderPropertyMap>>,
    __padding_end: [u8; 16],
}
impl UActorRecorderPropertyMap {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UActorRecorderPropertyMap")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UActorRecorderPropertyMap")
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
pub struct UTakeRecorderAudioInputSettings {
    __padding_end: [u8; 80],
}
impl UTakeRecorderAudioInputSettings {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UTakeRecorderAudioInputSettings")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UTakeRecorderAudioInputSettings")
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
#[repr(C, align(16))]
pub struct UTakeRecorderSources {
    __padding_end: [u8; 416],
}
impl UTakeRecorderSources {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UTakeRecorderSources")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UTakeRecorderSources")
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
                crate::bindings::takes_core::__FUNCTION_PTRS
                    .u_take_recorder_sources_start_recording_source,
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
                crate::bindings::takes_core::__FUNCTION_PTRS
                    .u_take_recorder_sources_start_recording_source,
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
                crate::bindings::takes_core::__FUNCTION_PTRS
                    .u_take_recorder_sources_remove_source,
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
                crate::bindings::takes_core::__FUNCTION_PTRS
                    .u_take_recorder_sources_remove_source,
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
                crate::bindings::takes_core::__FUNCTION_PTRS
                    .u_take_recorder_sources_get_sources_copy,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::takes_core::__FUNCTION_PTRS
                    .u_take_recorder_sources_get_sources_copy,
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
                crate::bindings::takes_core::__FUNCTION_PTRS
                    .u_take_recorder_sources_add_source,
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
                crate::bindings::takes_core::__FUNCTION_PTRS
                    .u_take_recorder_sources_add_source,
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
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UTakesCoreBlueprintLibrary")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UTakesCoreBlueprintLibrary")
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
                crate::bindings::takes_core::__FUNCTION_PTRS
                    .u_takes_core_blueprint_library_set_on_take_recorder_take_number_changed,
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
                crate::bindings::takes_core::__FUNCTION_PTRS
                    .u_takes_core_blueprint_library_set_on_take_recorder_take_number_changed,
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
                crate::bindings::takes_core::__FUNCTION_PTRS
                    .u_takes_core_blueprint_library_set_on_take_recorder_slate_changed,
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
                crate::bindings::takes_core::__FUNCTION_PTRS
                    .u_takes_core_blueprint_library_set_on_take_recorder_slate_changed,
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
                crate::bindings::takes_core::__FUNCTION_PTRS
                    .u_takes_core_blueprint_library_find_takes,
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
                crate::bindings::takes_core::__FUNCTION_PTRS
                    .u_takes_core_blueprint_library_find_takes,
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
                crate::bindings::takes_core::__FUNCTION_PTRS
                    .u_takes_core_blueprint_library_compute_next_take_number,
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
                crate::bindings::takes_core::__FUNCTION_PTRS
                    .u_takes_core_blueprint_library_compute_next_take_number,
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
