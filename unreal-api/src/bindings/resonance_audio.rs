#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(clippy::non_camel_case_types)]
#![allow(clippy::new_without_default)]
#![allow(clippy::new_ret_no_self)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[doc(hidden)]
pub static mut __FUNCTION_PTRS: FunctionPtrs = FunctionPtrs::empty();
pub struct FunctionPtrs {
    pub u_resonance_audio_blueprint_function_library_set_global_reverb_preset: *mut crate::ffi::UFunctionOpague,
    pub u_resonance_audio_blueprint_function_library_get_global_reverb_preset: *mut crate::ffi::UFunctionOpague,
    pub u_resonance_audio_reverb_plugin_preset_set_room_rotation: *mut crate::ffi::UFunctionOpague,
    pub u_resonance_audio_reverb_plugin_preset_set_room_position: *mut crate::ffi::UFunctionOpague,
    pub u_resonance_audio_reverb_plugin_preset_set_room_materials: *mut crate::ffi::UFunctionOpague,
    pub u_resonance_audio_reverb_plugin_preset_set_room_dimensions: *mut crate::ffi::UFunctionOpague,
    pub u_resonance_audio_reverb_plugin_preset_set_reverb_time_modifier: *mut crate::ffi::UFunctionOpague,
    pub u_resonance_audio_reverb_plugin_preset_set_reverb_gain: *mut crate::ffi::UFunctionOpague,
    pub u_resonance_audio_reverb_plugin_preset_set_reverb_brightness: *mut crate::ffi::UFunctionOpague,
    pub u_resonance_audio_reverb_plugin_preset_set_reflection_scalar: *mut crate::ffi::UFunctionOpague,
    pub u_resonance_audio_reverb_plugin_preset_set_enable_room_effects: *mut crate::ffi::UFunctionOpague,
    pub u_resonance_audio_spatialization_source_settings_set_sound_source_spread: *mut crate::ffi::UFunctionOpague,
    pub u_resonance_audio_spatialization_source_settings_set_sound_source_directivity: *mut crate::ffi::UFunctionOpague,
}
impl FunctionPtrs {
    pub const fn empty() -> Self {
        Self {
            u_resonance_audio_blueprint_function_library_set_global_reverb_preset: std::ptr::null_mut(),
            u_resonance_audio_blueprint_function_library_get_global_reverb_preset: std::ptr::null_mut(),
            u_resonance_audio_reverb_plugin_preset_set_room_rotation: std::ptr::null_mut(),
            u_resonance_audio_reverb_plugin_preset_set_room_position: std::ptr::null_mut(),
            u_resonance_audio_reverb_plugin_preset_set_room_materials: std::ptr::null_mut(),
            u_resonance_audio_reverb_plugin_preset_set_room_dimensions: std::ptr::null_mut(),
            u_resonance_audio_reverb_plugin_preset_set_reverb_time_modifier: std::ptr::null_mut(),
            u_resonance_audio_reverb_plugin_preset_set_reverb_gain: std::ptr::null_mut(),
            u_resonance_audio_reverb_plugin_preset_set_reverb_brightness: std::ptr::null_mut(),
            u_resonance_audio_reverb_plugin_preset_set_reflection_scalar: std::ptr::null_mut(),
            u_resonance_audio_reverb_plugin_preset_set_enable_room_effects: std::ptr::null_mut(),
            u_resonance_audio_spatialization_source_settings_set_sound_source_spread: std::ptr::null_mut(),
            u_resonance_audio_spatialization_source_settings_set_sound_source_directivity: std::ptr::null_mut(),
        }
    }
}
pub fn initialize() {
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UResonanceAudioBlueprintFunctionLibrary::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetGlobalReverbPreset"),
            &raw mut __FUNCTION_PTRS
                .u_resonance_audio_blueprint_function_library_set_global_reverb_preset,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetGlobalReverbPreset"),
            &raw mut __FUNCTION_PTRS
                .u_resonance_audio_blueprint_function_library_get_global_reverb_preset,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UResonanceAudioReverbPluginPreset::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetRoomRotation"),
            &raw mut __FUNCTION_PTRS
                .u_resonance_audio_reverb_plugin_preset_set_room_rotation,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetRoomPosition"),
            &raw mut __FUNCTION_PTRS
                .u_resonance_audio_reverb_plugin_preset_set_room_position,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetRoomMaterials"),
            &raw mut __FUNCTION_PTRS
                .u_resonance_audio_reverb_plugin_preset_set_room_materials,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetRoomDimensions"),
            &raw mut __FUNCTION_PTRS
                .u_resonance_audio_reverb_plugin_preset_set_room_dimensions,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetReverbTimeModifier"),
            &raw mut __FUNCTION_PTRS
                .u_resonance_audio_reverb_plugin_preset_set_reverb_time_modifier,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetReverbGain"),
            &raw mut __FUNCTION_PTRS
                .u_resonance_audio_reverb_plugin_preset_set_reverb_gain,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetReverbBrightness"),
            &raw mut __FUNCTION_PTRS
                .u_resonance_audio_reverb_plugin_preset_set_reverb_brightness,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetReflectionScalar"),
            &raw mut __FUNCTION_PTRS
                .u_resonance_audio_reverb_plugin_preset_set_reflection_scalar,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetEnableRoomEffects"),
            &raw mut __FUNCTION_PTRS
                .u_resonance_audio_reverb_plugin_preset_set_enable_room_effects,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UResonanceAudioSpatializationSourceSettings::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetSoundSourceSpread"),
            &raw mut __FUNCTION_PTRS
                .u_resonance_audio_spatialization_source_settings_set_sound_source_spread,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetSoundSourceDirectivity"),
            &raw mut __FUNCTION_PTRS
                .u_resonance_audio_spatialization_source_settings_set_sound_source_directivity,
        );
    }
}
#[repr(C, align(16))]
pub struct FResonanceAudioReverbPluginSettings {
    pub b_enable_room_effects: bool,
    pub room_position: crate::bindings::core_u_object::FVector,
    pub room_rotation: crate::bindings::core_u_object::FQuat,
    pub room_dimensions: crate::bindings::core_u_object::FVector,
    pub left_wall_material: ERaMaterialName,
    pub right_wall_material: ERaMaterialName,
    pub floor_material: ERaMaterialName,
    pub ceiling_material: ERaMaterialName,
    pub front_wall_material: ERaMaterialName,
    pub back_wall_material: ERaMaterialName,
    pub reflection_scalar: f32,
    pub reverb_gain: f32,
    pub reverb_time_modifier: f32,
    pub reverb_brightness: f32,
}
impl FResonanceAudioReverbPluginSettings {}
#[repr(C, align(8))]
pub struct UResonanceAudioSoundfieldSettings {
    #[doc(hidden)]
    __padding_48: [u8; 48],
    pub render_mode: EResonanceRenderMode,
}
impl UResonanceAudioSoundfieldSettings {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UResonanceAudioSoundfieldSettings")
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
pub struct UResonanceAudioBlueprintFunctionLibrary {
    __padding_end: [u8; 48],
}
impl UResonanceAudioBlueprintFunctionLibrary {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UResonanceAudioBlueprintFunctionLibrary")
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
    pub fn set_global_reverb_preset(in_preset: UPtr<UResonanceAudioReverbPluginPreset>) {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::resonance_audio::__FUNCTION_PTRS
                    .u_resonance_audio_blueprint_function_library_set_global_reverb_preset,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_preset,
                __buffer.add(0).cast::<UPtr<UResonanceAudioReverbPluginPreset>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::resonance_audio::UResonanceAudioBlueprintFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::resonance_audio::__FUNCTION_PTRS
                    .u_resonance_audio_blueprint_function_library_set_global_reverb_preset,
                __buffer,
            )
        };
    }
    pub fn get_global_reverb_preset() -> UPtr<UResonanceAudioReverbPluginPreset> {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::resonance_audio::__FUNCTION_PTRS
                    .u_resonance_audio_blueprint_function_library_get_global_reverb_preset,
                __buffer,
            )
        };
        let __object_ptr = crate::bindings::resonance_audio::UResonanceAudioBlueprintFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::resonance_audio::__FUNCTION_PTRS
                    .u_resonance_audio_blueprint_function_library_get_global_reverb_preset,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<UPtr<UResonanceAudioReverbPluginPreset>>().read()
        }
    }
}
#[repr(C, align(8))]
pub struct AResonanceAudioDirectivityVisualizer {
    __padding_end: [u8; 1264],
}
impl AResonanceAudioDirectivityVisualizer {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("AResonanceAudioDirectivityVisualizer")
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
pub struct UResonanceAudioReverbPluginPreset {
    __padding_end: [u8; 384],
}
impl UResonanceAudioReverbPluginPreset {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UResonanceAudioReverbPluginPreset")
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
    pub fn set_room_rotation(
        &mut self,
        in_rotation: &crate::bindings::core_u_object::FQuat,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::resonance_audio::__FUNCTION_PTRS
                    .u_resonance_audio_reverb_plugin_preset_set_room_rotation,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_rotation,
                __buffer.add(0).cast::<crate::bindings::core_u_object::FQuat>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::resonance_audio::__FUNCTION_PTRS
                    .u_resonance_audio_reverb_plugin_preset_set_room_rotation,
                __buffer,
            )
        };
    }
    pub fn set_room_position(
        &mut self,
        in_position: &crate::bindings::core_u_object::FVector,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::resonance_audio::__FUNCTION_PTRS
                    .u_resonance_audio_reverb_plugin_preset_set_room_position,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_position,
                __buffer.add(0).cast::<crate::bindings::core_u_object::FVector>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::resonance_audio::__FUNCTION_PTRS
                    .u_resonance_audio_reverb_plugin_preset_set_room_position,
                __buffer,
            )
        };
    }
    pub fn set_room_materials(&mut self, in_materials: &TArray<ERaMaterialName>) {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::resonance_audio::__FUNCTION_PTRS
                    .u_resonance_audio_reverb_plugin_preset_set_room_materials,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_materials,
                __buffer.add(0).cast::<TArray<ERaMaterialName>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::resonance_audio::__FUNCTION_PTRS
                    .u_resonance_audio_reverb_plugin_preset_set_room_materials,
                __buffer,
            )
        };
    }
    pub fn set_room_dimensions(
        &mut self,
        in_dimensions: &crate::bindings::core_u_object::FVector,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::resonance_audio::__FUNCTION_PTRS
                    .u_resonance_audio_reverb_plugin_preset_set_room_dimensions,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_dimensions,
                __buffer.add(0).cast::<crate::bindings::core_u_object::FVector>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::resonance_audio::__FUNCTION_PTRS
                    .u_resonance_audio_reverb_plugin_preset_set_room_dimensions,
                __buffer,
            )
        };
    }
    pub fn set_reverb_time_modifier(&mut self, in_reverb_time_modifier: f32) {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::resonance_audio::__FUNCTION_PTRS
                    .u_resonance_audio_reverb_plugin_preset_set_reverb_time_modifier,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_reverb_time_modifier,
                __buffer.add(0).cast::<f32>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::resonance_audio::__FUNCTION_PTRS
                    .u_resonance_audio_reverb_plugin_preset_set_reverb_time_modifier,
                __buffer,
            )
        };
    }
    pub fn set_reverb_gain(&mut self, in_reverb_gain: f32) {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::resonance_audio::__FUNCTION_PTRS
                    .u_resonance_audio_reverb_plugin_preset_set_reverb_gain,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_reverb_gain,
                __buffer.add(0).cast::<f32>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::resonance_audio::__FUNCTION_PTRS
                    .u_resonance_audio_reverb_plugin_preset_set_reverb_gain,
                __buffer,
            )
        };
    }
    pub fn set_reverb_brightness(&mut self, in_reverb_brightness: f32) {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::resonance_audio::__FUNCTION_PTRS
                    .u_resonance_audio_reverb_plugin_preset_set_reverb_brightness,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_reverb_brightness,
                __buffer.add(0).cast::<f32>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::resonance_audio::__FUNCTION_PTRS
                    .u_resonance_audio_reverb_plugin_preset_set_reverb_brightness,
                __buffer,
            )
        };
    }
    pub fn set_reflection_scalar(&mut self, in_reflection_scalar: f32) {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::resonance_audio::__FUNCTION_PTRS
                    .u_resonance_audio_reverb_plugin_preset_set_reflection_scalar,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_reflection_scalar,
                __buffer.add(0).cast::<f32>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::resonance_audio::__FUNCTION_PTRS
                    .u_resonance_audio_reverb_plugin_preset_set_reflection_scalar,
                __buffer,
            )
        };
    }
    pub fn set_enable_room_effects(&mut self, b_in_enable_room_effects: bool) {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::resonance_audio::__FUNCTION_PTRS
                    .u_resonance_audio_reverb_plugin_preset_set_enable_room_effects,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_in_enable_room_effects,
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
                crate::bindings::resonance_audio::__FUNCTION_PTRS
                    .u_resonance_audio_reverb_plugin_preset_set_enable_room_effects,
                __buffer,
            )
        };
    }
}
#[repr(C, align(8))]
pub struct UResonanceAudioSettings {
    __padding_end: [u8; 176],
}
impl UResonanceAudioSettings {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UResonanceAudioSettings")
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
pub struct UResonanceAudioSpatializationSourceSettings {
    __padding_end: [u8; 88],
}
impl UResonanceAudioSpatializationSourceSettings {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UResonanceAudioSpatializationSourceSettings")
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
    pub fn set_sound_source_spread(&mut self, in_spread: f32) {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::resonance_audio::__FUNCTION_PTRS
                    .u_resonance_audio_spatialization_source_settings_set_sound_source_spread,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&in_spread, __buffer.add(0).cast::<f32>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::resonance_audio::__FUNCTION_PTRS
                    .u_resonance_audio_spatialization_source_settings_set_sound_source_spread,
                __buffer,
            )
        };
    }
    pub fn set_sound_source_directivity(&mut self, in_pattern: f32, in_sharpness: f32) {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::resonance_audio::__FUNCTION_PTRS
                    .u_resonance_audio_spatialization_source_settings_set_sound_source_directivity,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&in_pattern, __buffer.add(0).cast::<f32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_sharpness,
                __buffer.add(4).cast::<f32>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::resonance_audio::__FUNCTION_PTRS
                    .u_resonance_audio_spatialization_source_settings_set_sound_source_directivity,
                __buffer,
            )
        };
    }
}
#[repr(transparent)]
pub struct ERaMaterialName(pub u8);
impl ERaMaterialName {
    pub const TRANSPARENT: ERaMaterialName = ERaMaterialName(0);
    pub const ACOUSTIC_CEILING_TILES: ERaMaterialName = ERaMaterialName(1);
    pub const BRICK_BARE: ERaMaterialName = ERaMaterialName(2);
    pub const BRICK_PAINTED: ERaMaterialName = ERaMaterialName(3);
    pub const CONCRETE_BLOCK_COARSE: ERaMaterialName = ERaMaterialName(4);
    pub const CONCRETE_BLOCK_PAINTED: ERaMaterialName = ERaMaterialName(5);
    pub const CURTAIN_HEAVY: ERaMaterialName = ERaMaterialName(6);
    pub const FIBER_GLASS_INSULATION: ERaMaterialName = ERaMaterialName(7);
    pub const GLASS_THIN: ERaMaterialName = ERaMaterialName(8);
    pub const GLASS_THICK: ERaMaterialName = ERaMaterialName(9);
    pub const GRASS: ERaMaterialName = ERaMaterialName(10);
    pub const LINOLEUM_ON_CONCRETE: ERaMaterialName = ERaMaterialName(11);
    pub const MARBLE: ERaMaterialName = ERaMaterialName(12);
    pub const METAL: ERaMaterialName = ERaMaterialName(13);
    pub const PARQUET_ONCONCRETE: ERaMaterialName = ERaMaterialName(14);
    pub const PLASTER_ROUGH: ERaMaterialName = ERaMaterialName(15);
    pub const PLASTER_SMOOTH: ERaMaterialName = ERaMaterialName(16);
    pub const PLYWOOD_PANEL: ERaMaterialName = ERaMaterialName(17);
    pub const POLISHED_CONCRETE_OR_TILE: ERaMaterialName = ERaMaterialName(18);
    pub const SHEETROCK: ERaMaterialName = ERaMaterialName(19);
    pub const WATER_OR_ICE_SURFACE: ERaMaterialName = ERaMaterialName(20);
    pub const WOOD_CEILING: ERaMaterialName = ERaMaterialName(21);
    pub const WOOD_PANEL: ERaMaterialName = ERaMaterialName(22);
    pub const UNIFORM: ERaMaterialName = ERaMaterialName(23);
}
#[repr(transparent)]
pub struct EResonanceRenderMode(pub u8);
impl EResonanceRenderMode {
    pub const STEREO_PANNING: EResonanceRenderMode = EResonanceRenderMode(0);
    pub const BINAURAL_LOW_QUALITY: EResonanceRenderMode = EResonanceRenderMode(1);
    pub const BINAURAL_MEDIUM_QUALITY: EResonanceRenderMode = EResonanceRenderMode(2);
    pub const BINAURAL_HIGH_QUALITY: EResonanceRenderMode = EResonanceRenderMode(3);
    pub const ROOM_EFFECTS_ONLY: EResonanceRenderMode = EResonanceRenderMode(4);
}
#[repr(transparent)]
pub struct ERaQualityMode(pub u8);
impl ERaQualityMode {
    pub const STEREO_PANNING: ERaQualityMode = ERaQualityMode(0);
    pub const BINAURAL_LOW: ERaQualityMode = ERaQualityMode(1);
    pub const BINAURAL_MEDIUM: ERaQualityMode = ERaQualityMode(2);
    pub const BINAURAL_HIGH: ERaQualityMode = ERaQualityMode(3);
}
#[repr(transparent)]
pub struct ERaSpatializationMethod(pub u8);
impl ERaSpatializationMethod {
    pub const STEREO_PANNING: ERaSpatializationMethod = ERaSpatializationMethod(0);
    pub const HRTF: ERaSpatializationMethod = ERaSpatializationMethod(1);
}
#[repr(transparent)]
pub struct ERaDistanceRolloffModel(pub u8);
impl ERaDistanceRolloffModel {
    pub const LOGARITHMIC: ERaDistanceRolloffModel = ERaDistanceRolloffModel(0);
    pub const LINEAR: ERaDistanceRolloffModel = ERaDistanceRolloffModel(1);
    pub const NONE: ERaDistanceRolloffModel = ERaDistanceRolloffModel(2);
}
