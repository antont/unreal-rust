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
pub static mut U_AUDIO_PROPERTIES_SHEET_ASSET_BASE_COPY_TO_OBJECT_PROPERTIES: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_AUDIO_PARAMETER_CONTROLLER_INTERFACE_SET_TRIGGER_PARAMETER: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_AUDIO_PARAMETER_CONTROLLER_INTERFACE_SET_STRING_PARAMETER: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_AUDIO_PARAMETER_CONTROLLER_INTERFACE_SET_STRING_ARRAY_PARAMETER: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_AUDIO_PARAMETER_CONTROLLER_INTERFACE_SET_PARAMETERS_BLUEPRINT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_AUDIO_PARAMETER_CONTROLLER_INTERFACE_SET_OBJECT_PARAMETER: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_AUDIO_PARAMETER_CONTROLLER_INTERFACE_SET_OBJECT_ARRAY_PARAMETER: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_AUDIO_PARAMETER_CONTROLLER_INTERFACE_SET_INT_PARAMETER: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_AUDIO_PARAMETER_CONTROLLER_INTERFACE_SET_INT_ARRAY_PARAMETER: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_AUDIO_PARAMETER_CONTROLLER_INTERFACE_SET_FLOAT_PARAMETER: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_AUDIO_PARAMETER_CONTROLLER_INTERFACE_SET_FLOAT_ARRAY_PARAMETER: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_AUDIO_PARAMETER_CONTROLLER_INTERFACE_SET_BOOL_PARAMETER: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_AUDIO_PARAMETER_CONTROLLER_INTERFACE_SET_BOOL_ARRAY_PARAMETER: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_AUDIO_PARAMETER_CONTROLLER_INTERFACE_RESET_PARAMETERS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
pub fn initialize() {
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UAudioPropertiesSheetAssetBase::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CopyToObjectProperties"),
            &raw mut U_AUDIO_PROPERTIES_SHEET_ASSET_BASE_COPY_TO_OBJECT_PROPERTIES,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UAudioParameterControllerInterface::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetTriggerParameter"),
            &raw mut U_AUDIO_PARAMETER_CONTROLLER_INTERFACE_SET_TRIGGER_PARAMETER,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetStringParameter"),
            &raw mut U_AUDIO_PARAMETER_CONTROLLER_INTERFACE_SET_STRING_PARAMETER,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetStringArrayParameter"),
            &raw mut U_AUDIO_PARAMETER_CONTROLLER_INTERFACE_SET_STRING_ARRAY_PARAMETER,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetParameters_Blueprint"),
            &raw mut U_AUDIO_PARAMETER_CONTROLLER_INTERFACE_SET_PARAMETERS_BLUEPRINT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetObjectParameter"),
            &raw mut U_AUDIO_PARAMETER_CONTROLLER_INTERFACE_SET_OBJECT_PARAMETER,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetObjectArrayParameter"),
            &raw mut U_AUDIO_PARAMETER_CONTROLLER_INTERFACE_SET_OBJECT_ARRAY_PARAMETER,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetIntParameter"),
            &raw mut U_AUDIO_PARAMETER_CONTROLLER_INTERFACE_SET_INT_PARAMETER,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetIntArrayParameter"),
            &raw mut U_AUDIO_PARAMETER_CONTROLLER_INTERFACE_SET_INT_ARRAY_PARAMETER,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetFloatParameter"),
            &raw mut U_AUDIO_PARAMETER_CONTROLLER_INTERFACE_SET_FLOAT_PARAMETER,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetFloatArrayParameter"),
            &raw mut U_AUDIO_PARAMETER_CONTROLLER_INTERFACE_SET_FLOAT_ARRAY_PARAMETER,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetBoolParameter"),
            &raw mut U_AUDIO_PARAMETER_CONTROLLER_INTERFACE_SET_BOOL_PARAMETER,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetBoolArrayParameter"),
            &raw mut U_AUDIO_PARAMETER_CONTROLLER_INTERFACE_SET_BOOL_ARRAY_PARAMETER,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ResetParameters"),
            &raw mut U_AUDIO_PARAMETER_CONTROLLER_INTERFACE_RESET_PARAMETERS,
        );
    }
}
#[repr(C, align(4))]
pub struct FSoundGeneratorOutput {
    pub name: FName,
}
impl FSoundGeneratorOutput {}
#[repr(C, align(8))]
pub struct FAudioParameter {
    pub param_name: FName,
    pub float_param: f32,
    pub bool_param: bool,
    pub int_param: i32,
    pub object_param: UPtr<crate::bindings::core_u_object::UObject>,
    pub string_param: FString,
    pub array_float_param: TArray<f32>,
    pub array_bool_param: TArray<bool>,
    pub array_int_param: TArray<i32>,
    pub array_object_param: TArray<UPtr<crate::bindings::core_u_object::UObject>>,
    pub array_string_param: TArray<FString>,
    pub param_type: EAudioParameterType,
    __padding_end: [u8; 31],
}
impl FAudioParameter {}
#[repr(C, align(8))]
pub struct FSoundWaveCuePoint {
    pub cue_point_id: i32,
    pub label: FString,
    pub frame_position: i64,
    pub frame_length: i64,
    pub b_is_loop_region: bool,
    __padding_end: [u8; 7],
}
impl FSoundWaveCuePoint {}
pub struct IAudioPropertiesSheetAssetUserInterface {}
#[repr(C, align(8))]
pub struct UAudioPropertiesSheetAssetUserInterface {
    __padding_end: [u8; 48],
}
impl UAudioPropertiesSheetAssetUserInterface {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAudioPropertiesSheetAssetUserInterface")
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
pub struct UAudioPropertiesSheetAssetBase {
    __padding_end: [u8; 48],
}
impl UAudioPropertiesSheetAssetBase {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAudioPropertiesSheetAssetBase")
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
pub struct USpatializationPluginSourceSettingsBase {
    __padding_end: [u8; 48],
}
impl USpatializationPluginSourceSettingsBase {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("USpatializationPluginSourceSettingsBase")
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
pub struct USourceDataOverridePluginSourceSettingsBase {
    __padding_end: [u8; 48],
}
impl USourceDataOverridePluginSourceSettingsBase {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("USourceDataOverridePluginSourceSettingsBase")
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
pub struct UOcclusionPluginSourceSettingsBase {
    __padding_end: [u8; 48],
}
impl UOcclusionPluginSourceSettingsBase {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UOcclusionPluginSourceSettingsBase")
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
pub struct UReverbPluginSourceSettingsBase {
    __padding_end: [u8; 48],
}
impl UReverbPluginSourceSettingsBase {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UReverbPluginSourceSettingsBase")
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
pub struct IAudioParameterControllerInterface {}
#[repr(C, align(8))]
pub struct UAudioParameterControllerInterface {
    __padding_end: [u8; 48],
}
impl UAudioParameterControllerInterface {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAudioParameterControllerInterface")
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
pub struct UAudioEndpointSettingsBase {
    __padding_end: [u8; 48],
}
impl UAudioEndpointSettingsBase {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAudioEndpointSettingsBase")
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
pub struct UDummyEndpointSettings {
    __padding_end: [u8; 48],
}
impl UDummyEndpointSettings {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDummyEndpointSettings")
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
pub struct USoundModulatorBase {
    __padding_end: [u8; 56],
}
impl USoundModulatorBase {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("USoundModulatorBase")
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
pub struct USoundfieldEndpointSettingsBase {
    __padding_end: [u8; 48],
}
impl USoundfieldEndpointSettingsBase {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("USoundfieldEndpointSettingsBase")
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
pub struct USoundfieldEncodingSettingsBase {
    __padding_end: [u8; 48],
}
impl USoundfieldEncodingSettingsBase {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("USoundfieldEncodingSettingsBase")
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
pub struct USoundfieldEffectSettingsBase {
    __padding_end: [u8; 48],
}
impl USoundfieldEffectSettingsBase {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("USoundfieldEffectSettingsBase")
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
pub struct USoundfieldEffectBase {
    __padding_end: [u8; 56],
}
impl USoundfieldEffectBase {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("USoundfieldEffectBase")
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
pub struct UWaveformTransformationBase {
    __padding_end: [u8; 96],
}
impl UWaveformTransformationBase {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UWaveformTransformationBase")
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
pub struct UWaveformTransformationChain {
    __padding_end: [u8; 64],
}
impl UWaveformTransformationChain {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UWaveformTransformationChain")
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
pub struct ESoundWaveCloudStreamingPlatformProjectEnableType(pub u8);
impl ESoundWaveCloudStreamingPlatformProjectEnableType {
    pub const ENABLED: ESoundWaveCloudStreamingPlatformProjectEnableType = ESoundWaveCloudStreamingPlatformProjectEnableType(
        0,
    );
    pub const DISABLED: ESoundWaveCloudStreamingPlatformProjectEnableType = ESoundWaveCloudStreamingPlatformProjectEnableType(
        1,
    );
}
#[repr(transparent)]
pub struct ESoundWaveCloudStreamingPlatformEnableType(pub u8);
impl ESoundWaveCloudStreamingPlatformEnableType {
    pub const INHERITED: ESoundWaveCloudStreamingPlatformEnableType = ESoundWaveCloudStreamingPlatformEnableType(
        0,
    );
    pub const DISABLED: ESoundWaveCloudStreamingPlatformEnableType = ESoundWaveCloudStreamingPlatformEnableType(
        1,
    );
    pub const SWC_MULTIPLE_VALUES: ESoundWaveCloudStreamingPlatformEnableType = ESoundWaveCloudStreamingPlatformEnableType(
        2,
    );
}
#[repr(transparent)]
pub struct EAudioParameterType(pub u8);
impl EAudioParameterType {
    pub const NONE: EAudioParameterType = EAudioParameterType(0);
    pub const BOOLEAN: EAudioParameterType = EAudioParameterType(1);
    pub const INTEGER: EAudioParameterType = EAudioParameterType(2);
    pub const FLOAT: EAudioParameterType = EAudioParameterType(3);
    pub const STRING: EAudioParameterType = EAudioParameterType(4);
    pub const OBJECT: EAudioParameterType = EAudioParameterType(5);
    pub const NONE_ARRAY: EAudioParameterType = EAudioParameterType(6);
    pub const BOOLEAN_ARRAY: EAudioParameterType = EAudioParameterType(7);
    pub const INTEGER_ARRAY: EAudioParameterType = EAudioParameterType(8);
    pub const FLOAT_ARRAY: EAudioParameterType = EAudioParameterType(9);
    pub const STRING_ARRAY: EAudioParameterType = EAudioParameterType(10);
    pub const OBJECT_ARRAY: EAudioParameterType = EAudioParameterType(11);
    pub const TRIGGER: EAudioParameterType = EAudioParameterType(12);
    pub const COUNT: EAudioParameterType = EAudioParameterType(13);
}
