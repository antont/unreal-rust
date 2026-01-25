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
    pub a_camera_rig_rail_get_rail_spline_component: *mut crate::ffi::UFunctionOpague,
    pub a_cine_camera_actor_get_cine_camera_component: *mut crate::ffi::UFunctionOpague,
    pub u_cine_camera_component_set_lens_settings: *mut crate::ffi::UFunctionOpague,
    pub u_cine_camera_component_set_lens_preset_by_name: *mut crate::ffi::UFunctionOpague,
    pub u_cine_camera_component_set_focus_settings: *mut crate::ffi::UFunctionOpague,
    pub u_cine_camera_component_set_filmback_preset_by_name: *mut crate::ffi::UFunctionOpague,
    pub u_cine_camera_component_set_filmback: *mut crate::ffi::UFunctionOpague,
    pub u_cine_camera_component_set_custom_near_clipping_plane: *mut crate::ffi::UFunctionOpague,
    pub u_cine_camera_component_set_current_focal_length: *mut crate::ffi::UFunctionOpague,
    pub u_cine_camera_component_set_current_aperture: *mut crate::ffi::UFunctionOpague,
    pub u_cine_camera_component_set_crop_settings: *mut crate::ffi::UFunctionOpague,
    pub u_cine_camera_component_set_crop_preset_by_name: *mut crate::ffi::UFunctionOpague,
    pub u_cine_camera_component_get_vertical_projection_offset: *mut crate::ffi::UFunctionOpague,
    pub u_cine_camera_component_get_vertical_field_of_view: *mut crate::ffi::UFunctionOpague,
    pub u_cine_camera_component_get_lens_presets_copy: *mut crate::ffi::UFunctionOpague,
    pub u_cine_camera_component_get_lens_preset_name: *mut crate::ffi::UFunctionOpague,
    pub u_cine_camera_component_get_horizontal_projection_offset: *mut crate::ffi::UFunctionOpague,
    pub u_cine_camera_component_get_horizontal_field_of_view: *mut crate::ffi::UFunctionOpague,
    pub u_cine_camera_component_get_filmback_presets_copy: *mut crate::ffi::UFunctionOpague,
    pub u_cine_camera_component_get_filmback_preset_name: *mut crate::ffi::UFunctionOpague,
    pub u_cine_camera_component_get_default_filmback_preset_name: *mut crate::ffi::UFunctionOpague,
    pub u_cine_camera_component_get_crop_preset_name: *mut crate::ffi::UFunctionOpague,
    pub u_cine_camera_settings_set_lens_presets: *mut crate::ffi::UFunctionOpague,
    pub u_cine_camera_settings_set_filmback_presets: *mut crate::ffi::UFunctionOpague,
    pub u_cine_camera_settings_set_default_lens_preset_name: *mut crate::ffi::UFunctionOpague,
    pub u_cine_camera_settings_set_default_lens_f_stop: *mut crate::ffi::UFunctionOpague,
    pub u_cine_camera_settings_set_default_lens_focal_length: *mut crate::ffi::UFunctionOpague,
    pub u_cine_camera_settings_set_default_filmback_preset: *mut crate::ffi::UFunctionOpague,
    pub u_cine_camera_settings_set_default_crop_preset_name: *mut crate::ffi::UFunctionOpague,
    pub u_cine_camera_settings_set_crop_presets: *mut crate::ffi::UFunctionOpague,
    pub u_cine_camera_settings_get_lens_preset_names: *mut crate::ffi::UFunctionOpague,
    pub u_cine_camera_settings_get_lens_preset_by_name: *mut crate::ffi::UFunctionOpague,
    pub u_cine_camera_settings_get_filmback_preset_names: *mut crate::ffi::UFunctionOpague,
    pub u_cine_camera_settings_get_filmback_preset_by_name: *mut crate::ffi::UFunctionOpague,
    pub u_cine_camera_settings_get_crop_preset_names: *mut crate::ffi::UFunctionOpague,
    pub u_cine_camera_settings_get_crop_preset_by_name: *mut crate::ffi::UFunctionOpague,
    pub u_cine_camera_settings_get_cine_camera_settings: *mut crate::ffi::UFunctionOpague,
}
impl FunctionPtrs {
    pub const fn empty() -> Self {
        Self {
            a_camera_rig_rail_get_rail_spline_component: std::ptr::null_mut(),
            a_cine_camera_actor_get_cine_camera_component: std::ptr::null_mut(),
            u_cine_camera_component_set_lens_settings: std::ptr::null_mut(),
            u_cine_camera_component_set_lens_preset_by_name: std::ptr::null_mut(),
            u_cine_camera_component_set_focus_settings: std::ptr::null_mut(),
            u_cine_camera_component_set_filmback_preset_by_name: std::ptr::null_mut(),
            u_cine_camera_component_set_filmback: std::ptr::null_mut(),
            u_cine_camera_component_set_custom_near_clipping_plane: std::ptr::null_mut(),
            u_cine_camera_component_set_current_focal_length: std::ptr::null_mut(),
            u_cine_camera_component_set_current_aperture: std::ptr::null_mut(),
            u_cine_camera_component_set_crop_settings: std::ptr::null_mut(),
            u_cine_camera_component_set_crop_preset_by_name: std::ptr::null_mut(),
            u_cine_camera_component_get_vertical_projection_offset: std::ptr::null_mut(),
            u_cine_camera_component_get_vertical_field_of_view: std::ptr::null_mut(),
            u_cine_camera_component_get_lens_presets_copy: std::ptr::null_mut(),
            u_cine_camera_component_get_lens_preset_name: std::ptr::null_mut(),
            u_cine_camera_component_get_horizontal_projection_offset: std::ptr::null_mut(),
            u_cine_camera_component_get_horizontal_field_of_view: std::ptr::null_mut(),
            u_cine_camera_component_get_filmback_presets_copy: std::ptr::null_mut(),
            u_cine_camera_component_get_filmback_preset_name: std::ptr::null_mut(),
            u_cine_camera_component_get_default_filmback_preset_name: std::ptr::null_mut(),
            u_cine_camera_component_get_crop_preset_name: std::ptr::null_mut(),
            u_cine_camera_settings_set_lens_presets: std::ptr::null_mut(),
            u_cine_camera_settings_set_filmback_presets: std::ptr::null_mut(),
            u_cine_camera_settings_set_default_lens_preset_name: std::ptr::null_mut(),
            u_cine_camera_settings_set_default_lens_f_stop: std::ptr::null_mut(),
            u_cine_camera_settings_set_default_lens_focal_length: std::ptr::null_mut(),
            u_cine_camera_settings_set_default_filmback_preset: std::ptr::null_mut(),
            u_cine_camera_settings_set_default_crop_preset_name: std::ptr::null_mut(),
            u_cine_camera_settings_set_crop_presets: std::ptr::null_mut(),
            u_cine_camera_settings_get_lens_preset_names: std::ptr::null_mut(),
            u_cine_camera_settings_get_lens_preset_by_name: std::ptr::null_mut(),
            u_cine_camera_settings_get_filmback_preset_names: std::ptr::null_mut(),
            u_cine_camera_settings_get_filmback_preset_by_name: std::ptr::null_mut(),
            u_cine_camera_settings_get_crop_preset_names: std::ptr::null_mut(),
            u_cine_camera_settings_get_crop_preset_by_name: std::ptr::null_mut(),
            u_cine_camera_settings_get_cine_camera_settings: std::ptr::null_mut(),
        }
    }
}
pub fn initialize() {
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = ACameraRig_Rail::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetRailSplineComponent"),
            &raw mut __FUNCTION_PTRS.a_camera_rig_rail_get_rail_spline_component,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = ACineCameraActor::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetCineCameraComponent"),
            &raw mut __FUNCTION_PTRS.a_cine_camera_actor_get_cine_camera_component,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UCineCameraComponent::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetLensSettings"),
            &raw mut __FUNCTION_PTRS.u_cine_camera_component_set_lens_settings,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetLensPresetByName"),
            &raw mut __FUNCTION_PTRS.u_cine_camera_component_set_lens_preset_by_name,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetFocusSettings"),
            &raw mut __FUNCTION_PTRS.u_cine_camera_component_set_focus_settings,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetFilmbackPresetByName"),
            &raw mut __FUNCTION_PTRS.u_cine_camera_component_set_filmback_preset_by_name,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetFilmback"),
            &raw mut __FUNCTION_PTRS.u_cine_camera_component_set_filmback,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetCustomNearClippingPlane"),
            &raw mut __FUNCTION_PTRS
                .u_cine_camera_component_set_custom_near_clipping_plane,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetCurrentFocalLength"),
            &raw mut __FUNCTION_PTRS.u_cine_camera_component_set_current_focal_length,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetCurrentAperture"),
            &raw mut __FUNCTION_PTRS.u_cine_camera_component_set_current_aperture,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetCropSettings"),
            &raw mut __FUNCTION_PTRS.u_cine_camera_component_set_crop_settings,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetCropPresetByName"),
            &raw mut __FUNCTION_PTRS.u_cine_camera_component_set_crop_preset_by_name,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetVerticalProjectionOffset"),
            &raw mut __FUNCTION_PTRS
                .u_cine_camera_component_get_vertical_projection_offset,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetVerticalFieldOfView"),
            &raw mut __FUNCTION_PTRS.u_cine_camera_component_get_vertical_field_of_view,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetLensPresetsCopy"),
            &raw mut __FUNCTION_PTRS.u_cine_camera_component_get_lens_presets_copy,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetLensPresetName"),
            &raw mut __FUNCTION_PTRS.u_cine_camera_component_get_lens_preset_name,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetHorizontalProjectionOffset"),
            &raw mut __FUNCTION_PTRS
                .u_cine_camera_component_get_horizontal_projection_offset,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetHorizontalFieldOfView"),
            &raw mut __FUNCTION_PTRS.u_cine_camera_component_get_horizontal_field_of_view,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetFilmbackPresetsCopy"),
            &raw mut __FUNCTION_PTRS.u_cine_camera_component_get_filmback_presets_copy,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetFilmbackPresetName"),
            &raw mut __FUNCTION_PTRS.u_cine_camera_component_get_filmback_preset_name,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetDefaultFilmbackPresetName"),
            &raw mut __FUNCTION_PTRS
                .u_cine_camera_component_get_default_filmback_preset_name,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetCropPresetName"),
            &raw mut __FUNCTION_PTRS.u_cine_camera_component_get_crop_preset_name,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UCineCameraSettings::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetLensPresets"),
            &raw mut __FUNCTION_PTRS.u_cine_camera_settings_set_lens_presets,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetFilmbackPresets"),
            &raw mut __FUNCTION_PTRS.u_cine_camera_settings_set_filmback_presets,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetDefaultLensPresetName"),
            &raw mut __FUNCTION_PTRS.u_cine_camera_settings_set_default_lens_preset_name,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetDefaultLensFStop"),
            &raw mut __FUNCTION_PTRS.u_cine_camera_settings_set_default_lens_f_stop,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetDefaultLensFocalLength"),
            &raw mut __FUNCTION_PTRS.u_cine_camera_settings_set_default_lens_focal_length,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetDefaultFilmbackPreset"),
            &raw mut __FUNCTION_PTRS.u_cine_camera_settings_set_default_filmback_preset,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetDefaultCropPresetName"),
            &raw mut __FUNCTION_PTRS.u_cine_camera_settings_set_default_crop_preset_name,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetCropPresets"),
            &raw mut __FUNCTION_PTRS.u_cine_camera_settings_set_crop_presets,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetLensPresetNames"),
            &raw mut __FUNCTION_PTRS.u_cine_camera_settings_get_lens_preset_names,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetLensPresetByName"),
            &raw mut __FUNCTION_PTRS.u_cine_camera_settings_get_lens_preset_by_name,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetFilmbackPresetNames"),
            &raw mut __FUNCTION_PTRS.u_cine_camera_settings_get_filmback_preset_names,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetFilmbackPresetByName"),
            &raw mut __FUNCTION_PTRS.u_cine_camera_settings_get_filmback_preset_by_name,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetCropPresetNames"),
            &raw mut __FUNCTION_PTRS.u_cine_camera_settings_get_crop_preset_names,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetCropPresetByName"),
            &raw mut __FUNCTION_PTRS.u_cine_camera_settings_get_crop_preset_by_name,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetCineCameraSettings"),
            &raw mut __FUNCTION_PTRS.u_cine_camera_settings_get_cine_camera_settings,
        );
    }
}
#[repr(C, align(8))]
pub struct FCameraLookatTrackingSettings {
    pub flags_0: u8,
    pub look_at_tracking_interp_speed: f32,
    #[doc(hidden)]
    pub(crate) __padding_40: [u8; 32],
    pub actor_to_track: TSoftObjectPtr<crate::bindings::engine::AActor>,
    pub relative_offset: crate::bindings::core_u_object::FVector,
    pub flags_112: u8,
}
impl FCameraLookatTrackingSettings {}
#[repr(C, align(4))]
pub struct FCameraFilmbackSettings {
    pub sensor_width: f32,
    pub sensor_height: f32,
    pub sensor_horizontal_offset: f32,
    pub sensor_vertical_offset: f32,
    pub sensor_aspect_ratio: f32,
}
impl FCameraFilmbackSettings {}
#[repr(C, align(8))]
pub struct FNamedFilmbackPreset {
    pub name: FString,
    pub display_name: FText,
    pub filmback_settings: FCameraFilmbackSettings,
}
impl FNamedFilmbackPreset {}
#[repr(C, align(4))]
pub struct FCameraLensSettings {
    pub min_focal_length: f32,
    pub max_focal_length: f32,
    pub min_f_stop: f32,
    pub max_f_stop: f32,
    pub minimum_focus_distance: f32,
    pub squeeze_factor: f32,
    pub diaphragm_blade_count: i32,
}
impl FCameraLensSettings {}
#[repr(C, align(8))]
pub struct FNamedLensPreset {
    pub name: FString,
    pub lens_settings: FCameraLensSettings,
}
impl FNamedLensPreset {}
#[repr(C, align(4))]
pub struct FPlateCropSettings {
    pub aspect_ratio: f32,
}
impl FPlateCropSettings {}
#[repr(C, align(8))]
pub struct FNamedPlateCropPreset {
    pub name: FString,
    pub crop_settings: FPlateCropSettings,
}
impl FNamedPlateCropPreset {}
#[repr(C, align(8))]
pub struct FCameraTrackingFocusSettings {
    pub actor_to_track: TSoftObjectPtr<crate::bindings::engine::AActor>,
    pub relative_offset: crate::bindings::core_u_object::FVector,
    pub flags_72: u8,
}
impl FCameraTrackingFocusSettings {}
#[repr(C, align(8))]
pub struct FCameraFocusSettings {
    pub focus_method: ECameraFocusMethod,
    pub manual_focus_distance: f32,
    pub tracking_focus_settings: FCameraTrackingFocusSettings,
    #[doc(hidden)]
    pub(crate) __padding_96: [u8; 8],
    pub flags_96: u8,
    pub focus_smoothing_interp_speed: f32,
    pub focus_offset: f32,
}
impl FCameraFocusSettings {}
#[repr(C, align(8))]
pub struct ACameraRig_Crane {
    #[doc(hidden)]
    pub(crate) __padding_1136: [u8; 1136],
    pub crane_pitch: f32,
    pub crane_yaw: f32,
    pub crane_arm_length: f32,
    pub b_lock_mount_pitch: bool,
    pub b_lock_mount_yaw: bool,
    __padding_end: [u8; 66],
}
impl ACameraRig_Crane {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("ACameraRig_Crane")
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
pub struct ACameraRig_Rail {
    #[doc(hidden)]
    pub(crate) __padding_1136: [u8; 1136],
    pub current_position_on_rail: f32,
    pub b_lock_orientation_to_rail: bool,
    __padding_end: [u8; 75],
}
impl ACameraRig_Rail {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("ACameraRig_Rail")
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
    pub fn get_rail_spline_component(
        &mut self,
    ) -> UPtr<crate::bindings::engine::USplineComponent> {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::cinematic_camera::__FUNCTION_PTRS
                    .a_camera_rig_rail_get_rail_spline_component,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::cinematic_camera::__FUNCTION_PTRS
                    .a_camera_rig_rail_get_rail_spline_component,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(0)
                .cast::<UPtr<crate::bindings::engine::USplineComponent>>()
                .read()
        }
    }
}
#[repr(C, align(16))]
pub struct ACineCameraActor {
    #[doc(hidden)]
    pub(crate) __padding_3136: [u8; 3136],
    pub lookat_tracking_settings: FCameraLookatTrackingSettings,
    __padding_end: [u8; 24],
}
impl ACineCameraActor {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("ACineCameraActor")
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
    pub fn get_cine_camera_component(&self) -> UPtr<UCineCameraComponent> {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::cinematic_camera::__FUNCTION_PTRS
                    .a_cine_camera_actor_get_cine_camera_component,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::cinematic_camera::__FUNCTION_PTRS
                    .a_cine_camera_actor_get_cine_camera_component,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<UPtr<UCineCameraComponent>>().read() }
    }
}
#[repr(C, align(16))]
pub struct UCineCameraComponent {
    #[doc(hidden)]
    pub(crate) __padding_2876: [u8; 2876],
    pub filmback: FCameraFilmbackSettings,
    pub lens_settings: FCameraLensSettings,
    pub focus_settings: FCameraFocusSettings,
    pub crop_settings: FPlateCropSettings,
    pub current_focal_length: f32,
    pub current_aperture: f32,
    pub current_focus_distance: f32,
    pub exposure_method: ECameraExposureMethod,
    #[doc(hidden)]
    pub(crate) __padding_3060: [u8; 3],
    pub flags_3060: u8,
    pub custom_near_clipping_plane: f32,
    __padding_end: [u8; 132],
}
impl UCineCameraComponent {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UCineCameraComponent")
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
    pub fn set_lens_settings(&mut self, new_lens_settings: &FCameraLensSettings) {
        let mut __stack = crate::core_data::StackAlloc::<28>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::cinematic_camera::__FUNCTION_PTRS
                    .u_cine_camera_component_set_lens_settings,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                new_lens_settings,
                __buffer.add(0).cast::<FCameraLensSettings>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::cinematic_camera::__FUNCTION_PTRS
                    .u_cine_camera_component_set_lens_settings,
                __buffer,
            )
        };
    }
    pub fn set_lens_preset_by_name(&mut self, in_preset_name: FString) {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::cinematic_camera::__FUNCTION_PTRS
                    .u_cine_camera_component_set_lens_preset_by_name,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_preset_name,
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
                crate::bindings::cinematic_camera::__FUNCTION_PTRS
                    .u_cine_camera_component_set_lens_preset_by_name,
                __buffer,
            )
        };
    }
    pub fn set_focus_settings(&mut self, new_focus_settings: &FCameraFocusSettings) {
        let mut __stack = crate::core_data::StackAlloc::<112>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::cinematic_camera::__FUNCTION_PTRS
                    .u_cine_camera_component_set_focus_settings,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                new_focus_settings,
                __buffer.add(0).cast::<FCameraFocusSettings>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::cinematic_camera::__FUNCTION_PTRS
                    .u_cine_camera_component_set_focus_settings,
                __buffer,
            )
        };
    }
    pub fn set_filmback_preset_by_name(&mut self, in_preset_name: FString) {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::cinematic_camera::__FUNCTION_PTRS
                    .u_cine_camera_component_set_filmback_preset_by_name,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_preset_name,
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
                crate::bindings::cinematic_camera::__FUNCTION_PTRS
                    .u_cine_camera_component_set_filmback_preset_by_name,
                __buffer,
            )
        };
    }
    pub fn set_filmback(&mut self, new_filmback: &FCameraFilmbackSettings) {
        let mut __stack = crate::core_data::StackAlloc::<20>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::cinematic_camera::__FUNCTION_PTRS
                    .u_cine_camera_component_set_filmback,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                new_filmback,
                __buffer.add(0).cast::<FCameraFilmbackSettings>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::cinematic_camera::__FUNCTION_PTRS
                    .u_cine_camera_component_set_filmback,
                __buffer,
            )
        };
    }
    pub fn set_custom_near_clipping_plane(
        &mut self,
        new_custom_near_clipping_plane: f32,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::cinematic_camera::__FUNCTION_PTRS
                    .u_cine_camera_component_set_custom_near_clipping_plane,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &new_custom_near_clipping_plane,
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
                crate::bindings::cinematic_camera::__FUNCTION_PTRS
                    .u_cine_camera_component_set_custom_near_clipping_plane,
                __buffer,
            )
        };
    }
    pub fn set_current_focal_length(&mut self, in_focal_length: f32) {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::cinematic_camera::__FUNCTION_PTRS
                    .u_cine_camera_component_set_current_focal_length,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_focal_length,
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
                crate::bindings::cinematic_camera::__FUNCTION_PTRS
                    .u_cine_camera_component_set_current_focal_length,
                __buffer,
            )
        };
    }
    pub fn set_current_aperture(&mut self, new_current_aperture: f32) {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::cinematic_camera::__FUNCTION_PTRS
                    .u_cine_camera_component_set_current_aperture,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &new_current_aperture,
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
                crate::bindings::cinematic_camera::__FUNCTION_PTRS
                    .u_cine_camera_component_set_current_aperture,
                __buffer,
            )
        };
    }
    pub fn set_crop_settings(&mut self, new_crop_settings: &FPlateCropSettings) {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::cinematic_camera::__FUNCTION_PTRS
                    .u_cine_camera_component_set_crop_settings,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                new_crop_settings,
                __buffer.add(0).cast::<FPlateCropSettings>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::cinematic_camera::__FUNCTION_PTRS
                    .u_cine_camera_component_set_crop_settings,
                __buffer,
            )
        };
    }
    pub fn set_crop_preset_by_name(&mut self, in_preset_name: FString) {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::cinematic_camera::__FUNCTION_PTRS
                    .u_cine_camera_component_set_crop_preset_by_name,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_preset_name,
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
                crate::bindings::cinematic_camera::__FUNCTION_PTRS
                    .u_cine_camera_component_set_crop_preset_by_name,
                __buffer,
            )
        };
    }
    pub fn get_vertical_projection_offset(&self) -> f32 {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::cinematic_camera::__FUNCTION_PTRS
                    .u_cine_camera_component_get_vertical_projection_offset,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::cinematic_camera::__FUNCTION_PTRS
                    .u_cine_camera_component_get_vertical_projection_offset,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<f32>().read() }
    }
    pub fn get_vertical_field_of_view(&self) -> f32 {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::cinematic_camera::__FUNCTION_PTRS
                    .u_cine_camera_component_get_vertical_field_of_view,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::cinematic_camera::__FUNCTION_PTRS
                    .u_cine_camera_component_get_vertical_field_of_view,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<f32>().read() }
    }
    pub fn get_lens_presets_copy() -> TArray<FNamedLensPreset> {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::cinematic_camera::__FUNCTION_PTRS
                    .u_cine_camera_component_get_lens_presets_copy,
                __buffer,
            )
        };
        let __object_ptr = crate::bindings::cinematic_camera::UCineCameraComponent::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::cinematic_camera::__FUNCTION_PTRS
                    .u_cine_camera_component_get_lens_presets_copy,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<TArray<FNamedLensPreset>>().read() }
    }
    pub fn get_lens_preset_name(&self) -> FString {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::cinematic_camera::__FUNCTION_PTRS
                    .u_cine_camera_component_get_lens_preset_name,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::cinematic_camera::__FUNCTION_PTRS
                    .u_cine_camera_component_get_lens_preset_name,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<FString>().read() }
    }
    pub fn get_horizontal_projection_offset(&self) -> f32 {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::cinematic_camera::__FUNCTION_PTRS
                    .u_cine_camera_component_get_horizontal_projection_offset,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::cinematic_camera::__FUNCTION_PTRS
                    .u_cine_camera_component_get_horizontal_projection_offset,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<f32>().read() }
    }
    pub fn get_horizontal_field_of_view(&self) -> f32 {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::cinematic_camera::__FUNCTION_PTRS
                    .u_cine_camera_component_get_horizontal_field_of_view,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::cinematic_camera::__FUNCTION_PTRS
                    .u_cine_camera_component_get_horizontal_field_of_view,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<f32>().read() }
    }
    pub fn get_filmback_presets_copy() -> TArray<FNamedFilmbackPreset> {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::cinematic_camera::__FUNCTION_PTRS
                    .u_cine_camera_component_get_filmback_presets_copy,
                __buffer,
            )
        };
        let __object_ptr = crate::bindings::cinematic_camera::UCineCameraComponent::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::cinematic_camera::__FUNCTION_PTRS
                    .u_cine_camera_component_get_filmback_presets_copy,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<TArray<FNamedFilmbackPreset>>().read() }
    }
    pub fn get_filmback_preset_name(&self) -> FString {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::cinematic_camera::__FUNCTION_PTRS
                    .u_cine_camera_component_get_filmback_preset_name,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::cinematic_camera::__FUNCTION_PTRS
                    .u_cine_camera_component_get_filmback_preset_name,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<FString>().read() }
    }
    pub fn get_default_filmback_preset_name(&self) -> FString {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::cinematic_camera::__FUNCTION_PTRS
                    .u_cine_camera_component_get_default_filmback_preset_name,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::cinematic_camera::__FUNCTION_PTRS
                    .u_cine_camera_component_get_default_filmback_preset_name,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<FString>().read() }
    }
    pub fn get_crop_preset_name(&self) -> FString {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::cinematic_camera::__FUNCTION_PTRS
                    .u_cine_camera_component_get_crop_preset_name,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::cinematic_camera::__FUNCTION_PTRS
                    .u_cine_camera_component_get_crop_preset_name,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<FString>().read() }
    }
}
#[repr(C, align(8))]
pub struct UCineCameraSettings {
    #[doc(hidden)]
    pub(crate) __padding_104: [u8; 104],
    pub default_lens_preset_name: FString,
    pub default_lens_focal_length: f32,
    pub default_lens_f_stop: f32,
    pub lens_presets: TArray<FNamedLensPreset>,
    pub default_filmback_preset: FString,
    pub filmback_presets: TArray<FNamedFilmbackPreset>,
    pub default_crop_preset_name: FString,
    pub crop_presets: TArray<FNamedPlateCropPreset>,
    __padding_end: [u8; 16],
}
impl UCineCameraSettings {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UCineCameraSettings")
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
    pub fn set_lens_presets(&mut self, in_lens_presets: &TArray<FNamedLensPreset>) {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::cinematic_camera::__FUNCTION_PTRS
                    .u_cine_camera_settings_set_lens_presets,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_lens_presets,
                __buffer.add(0).cast::<TArray<FNamedLensPreset>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::cinematic_camera::__FUNCTION_PTRS
                    .u_cine_camera_settings_set_lens_presets,
                __buffer,
            )
        };
    }
    pub fn set_filmback_presets(
        &mut self,
        in_filmback_presets: &TArray<FNamedFilmbackPreset>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::cinematic_camera::__FUNCTION_PTRS
                    .u_cine_camera_settings_set_filmback_presets,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_filmback_presets,
                __buffer.add(0).cast::<TArray<FNamedFilmbackPreset>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::cinematic_camera::__FUNCTION_PTRS
                    .u_cine_camera_settings_set_filmback_presets,
                __buffer,
            )
        };
    }
    pub fn set_default_lens_preset_name(
        &mut self,
        in_default_lens_preset_name: FString,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::cinematic_camera::__FUNCTION_PTRS
                    .u_cine_camera_settings_set_default_lens_preset_name,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_default_lens_preset_name,
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
                crate::bindings::cinematic_camera::__FUNCTION_PTRS
                    .u_cine_camera_settings_set_default_lens_preset_name,
                __buffer,
            )
        };
    }
    pub fn set_default_lens_f_stop(&mut self, in_default_lens_f_stop: f32) {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::cinematic_camera::__FUNCTION_PTRS
                    .u_cine_camera_settings_set_default_lens_f_stop,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_default_lens_f_stop,
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
                crate::bindings::cinematic_camera::__FUNCTION_PTRS
                    .u_cine_camera_settings_set_default_lens_f_stop,
                __buffer,
            )
        };
    }
    pub fn set_default_lens_focal_length(&mut self, in_default_lens_focal_length: f32) {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::cinematic_camera::__FUNCTION_PTRS
                    .u_cine_camera_settings_set_default_lens_focal_length,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_default_lens_focal_length,
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
                crate::bindings::cinematic_camera::__FUNCTION_PTRS
                    .u_cine_camera_settings_set_default_lens_focal_length,
                __buffer,
            )
        };
    }
    pub fn set_default_filmback_preset(&mut self, in_default_filmback_preset: FString) {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::cinematic_camera::__FUNCTION_PTRS
                    .u_cine_camera_settings_set_default_filmback_preset,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_default_filmback_preset,
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
                crate::bindings::cinematic_camera::__FUNCTION_PTRS
                    .u_cine_camera_settings_set_default_filmback_preset,
                __buffer,
            )
        };
    }
    pub fn set_default_crop_preset_name(
        &mut self,
        in_default_crop_preset_name: FString,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::cinematic_camera::__FUNCTION_PTRS
                    .u_cine_camera_settings_set_default_crop_preset_name,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_default_crop_preset_name,
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
                crate::bindings::cinematic_camera::__FUNCTION_PTRS
                    .u_cine_camera_settings_set_default_crop_preset_name,
                __buffer,
            )
        };
    }
    pub fn set_crop_presets(&mut self, in_crop_presets: &TArray<FNamedPlateCropPreset>) {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::cinematic_camera::__FUNCTION_PTRS
                    .u_cine_camera_settings_set_crop_presets,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_crop_presets,
                __buffer.add(0).cast::<TArray<FNamedPlateCropPreset>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::cinematic_camera::__FUNCTION_PTRS
                    .u_cine_camera_settings_set_crop_presets,
                __buffer,
            )
        };
    }
    pub fn get_lens_preset_by_name(
        &mut self,
        preset_name: FString,
        lens_settings: &mut FCameraLensSettings,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<45>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::cinematic_camera::__FUNCTION_PTRS
                    .u_cine_camera_settings_get_lens_preset_by_name,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &preset_name,
                __buffer.add(0).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                lens_settings,
                __buffer.add(16).cast::<FCameraLensSettings>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::cinematic_camera::__FUNCTION_PTRS
                    .u_cine_camera_settings_get_lens_preset_by_name,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(16).cast::<FCameraLensSettings>().swap(lens_settings);
        }
        unsafe { __buffer.add(44).cast::<bool>().read() }
    }
    pub fn get_filmback_preset_by_name(
        &mut self,
        preset_name: FString,
        filmback_settings: &mut FCameraFilmbackSettings,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<37>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::cinematic_camera::__FUNCTION_PTRS
                    .u_cine_camera_settings_get_filmback_preset_by_name,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &preset_name,
                __buffer.add(0).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                filmback_settings,
                __buffer.add(16).cast::<FCameraFilmbackSettings>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::cinematic_camera::__FUNCTION_PTRS
                    .u_cine_camera_settings_get_filmback_preset_by_name,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(16).cast::<FCameraFilmbackSettings>().swap(filmback_settings);
        }
        unsafe { __buffer.add(36).cast::<bool>().read() }
    }
    pub fn get_crop_preset_by_name(
        &mut self,
        preset_name: FString,
        crop_settings: &mut FPlateCropSettings,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<21>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::cinematic_camera::__FUNCTION_PTRS
                    .u_cine_camera_settings_get_crop_preset_by_name,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &preset_name,
                __buffer.add(0).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                crop_settings,
                __buffer.add(16).cast::<FPlateCropSettings>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::cinematic_camera::__FUNCTION_PTRS
                    .u_cine_camera_settings_get_crop_preset_by_name,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(16).cast::<FPlateCropSettings>().swap(crop_settings);
        }
        unsafe { __buffer.add(20).cast::<bool>().read() }
    }
    pub fn get_cine_camera_settings() -> UPtr<UCineCameraSettings> {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::cinematic_camera::__FUNCTION_PTRS
                    .u_cine_camera_settings_get_cine_camera_settings,
                __buffer,
            )
        };
        let __object_ptr = crate::bindings::cinematic_camera::UCineCameraSettings::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::cinematic_camera::__FUNCTION_PTRS
                    .u_cine_camera_settings_get_cine_camera_settings,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<UPtr<UCineCameraSettings>>().read() }
    }
}
#[repr(transparent)]
pub struct ECameraFocusMethod(pub u8);
impl ECameraFocusMethod {
    pub const DO_NOT_OVERRIDE: ECameraFocusMethod = ECameraFocusMethod(0);
    pub const MANUAL: ECameraFocusMethod = ECameraFocusMethod(1);
    pub const TRACKING: ECameraFocusMethod = ECameraFocusMethod(2);
    pub const DISABLE: ECameraFocusMethod = ECameraFocusMethod(3);
    pub const MAX: ECameraFocusMethod = ECameraFocusMethod(4);
}
#[repr(transparent)]
pub struct ECameraExposureMethod(pub u8);
impl ECameraExposureMethod {
    pub const DO_NOT_OVERRIDE: ECameraExposureMethod = ECameraExposureMethod(0);
    pub const ENABLED: ECameraExposureMethod = ECameraExposureMethod(1);
    pub const MAX: ECameraExposureMethod = ECameraExposureMethod(2);
}
