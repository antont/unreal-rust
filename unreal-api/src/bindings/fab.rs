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
    pub u_fab_browser_api_set_preferred_quality_tier: *mut crate::ffi::UFunctionOpague,
    pub u_fab_browser_api_plugin_opened: *mut crate::ffi::UFunctionOpague,
    pub u_fab_browser_api_open_url_in_browser: *mut crate::ffi::UFunctionOpague,
    pub u_fab_browser_api_open_plugin_settings: *mut crate::ffi::UFunctionOpague,
    pub u_fab_browser_api_on_drag_info_success: *mut crate::ffi::UFunctionOpague,
    pub u_fab_browser_api_on_drag_info_failure: *mut crate::ffi::UFunctionOpague,
    pub u_fab_browser_api_logout: *mut crate::ffi::UFunctionOpague,
    pub u_fab_browser_api_login: *mut crate::ffi::UFunctionOpague,
    pub u_fab_browser_api_get_url: *mut crate::ffi::UFunctionOpague,
    pub u_fab_browser_api_get_settings: *mut crate::ffi::UFunctionOpague,
    pub u_fab_browser_api_get_refresh_token: *mut crate::ffi::UFunctionOpague,
    pub u_fab_browser_api_get_auth_token: *mut crate::ffi::UFunctionOpague,
    pub u_fab_browser_api_get_api_version: *mut crate::ffi::UFunctionOpague,
    pub u_fab_browser_api_drag_start: *mut crate::ffi::UFunctionOpague,
    pub u_fab_browser_api_copy_to_clipboard: *mut crate::ffi::UFunctionOpague,
    pub u_fab_browser_api_add_to_project: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_instanced_foliage_type_factory_node_set_custom_world_position_offset_disable_distance: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_instanced_foliage_type_factory_node_set_custom_static_mesh: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_instanced_foliage_type_factory_node_set_custom_scaling: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_instanced_foliage_type_factory_node_set_custom_scale_z: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_instanced_foliage_type_factory_node_set_custom_scale_y: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_instanced_foliage_type_factory_node_set_custom_scale_x: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_instanced_foliage_type_factory_node_set_custom_random_yaw: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_instanced_foliage_type_factory_node_set_custom_random_pitch_angle: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_instanced_foliage_type_factory_node_set_custom_align_to_normal: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_instanced_foliage_type_factory_node_set_custom_affect_distance_field_lighting: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_instanced_foliage_type_factory_node_get_custom_world_position_offset_disable_distance: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_instanced_foliage_type_factory_node_get_custom_static_mesh: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_instanced_foliage_type_factory_node_get_custom_scaling: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_instanced_foliage_type_factory_node_get_custom_scale_z: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_instanced_foliage_type_factory_node_get_custom_scale_y: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_instanced_foliage_type_factory_node_get_custom_scale_x: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_instanced_foliage_type_factory_node_get_custom_random_yaw: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_instanced_foliage_type_factory_node_get_custom_random_pitch_angle: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_instanced_foliage_type_factory_node_get_custom_align_to_normal: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_instanced_foliage_type_factory_node_get_custom_affect_distance_field_lighting: *mut crate::ffi::UFunctionOpague,
}
impl FunctionPtrs {
    pub const fn empty() -> Self {
        Self {
            u_fab_browser_api_set_preferred_quality_tier: std::ptr::null_mut(),
            u_fab_browser_api_plugin_opened: std::ptr::null_mut(),
            u_fab_browser_api_open_url_in_browser: std::ptr::null_mut(),
            u_fab_browser_api_open_plugin_settings: std::ptr::null_mut(),
            u_fab_browser_api_on_drag_info_success: std::ptr::null_mut(),
            u_fab_browser_api_on_drag_info_failure: std::ptr::null_mut(),
            u_fab_browser_api_logout: std::ptr::null_mut(),
            u_fab_browser_api_login: std::ptr::null_mut(),
            u_fab_browser_api_get_url: std::ptr::null_mut(),
            u_fab_browser_api_get_settings: std::ptr::null_mut(),
            u_fab_browser_api_get_refresh_token: std::ptr::null_mut(),
            u_fab_browser_api_get_auth_token: std::ptr::null_mut(),
            u_fab_browser_api_get_api_version: std::ptr::null_mut(),
            u_fab_browser_api_drag_start: std::ptr::null_mut(),
            u_fab_browser_api_copy_to_clipboard: std::ptr::null_mut(),
            u_fab_browser_api_add_to_project: std::ptr::null_mut(),
            u_interchange_instanced_foliage_type_factory_node_set_custom_world_position_offset_disable_distance: std::ptr::null_mut(),
            u_interchange_instanced_foliage_type_factory_node_set_custom_static_mesh: std::ptr::null_mut(),
            u_interchange_instanced_foliage_type_factory_node_set_custom_scaling: std::ptr::null_mut(),
            u_interchange_instanced_foliage_type_factory_node_set_custom_scale_z: std::ptr::null_mut(),
            u_interchange_instanced_foliage_type_factory_node_set_custom_scale_y: std::ptr::null_mut(),
            u_interchange_instanced_foliage_type_factory_node_set_custom_scale_x: std::ptr::null_mut(),
            u_interchange_instanced_foliage_type_factory_node_set_custom_random_yaw: std::ptr::null_mut(),
            u_interchange_instanced_foliage_type_factory_node_set_custom_random_pitch_angle: std::ptr::null_mut(),
            u_interchange_instanced_foliage_type_factory_node_set_custom_align_to_normal: std::ptr::null_mut(),
            u_interchange_instanced_foliage_type_factory_node_set_custom_affect_distance_field_lighting: std::ptr::null_mut(),
            u_interchange_instanced_foliage_type_factory_node_get_custom_world_position_offset_disable_distance: std::ptr::null_mut(),
            u_interchange_instanced_foliage_type_factory_node_get_custom_static_mesh: std::ptr::null_mut(),
            u_interchange_instanced_foliage_type_factory_node_get_custom_scaling: std::ptr::null_mut(),
            u_interchange_instanced_foliage_type_factory_node_get_custom_scale_z: std::ptr::null_mut(),
            u_interchange_instanced_foliage_type_factory_node_get_custom_scale_y: std::ptr::null_mut(),
            u_interchange_instanced_foliage_type_factory_node_get_custom_scale_x: std::ptr::null_mut(),
            u_interchange_instanced_foliage_type_factory_node_get_custom_random_yaw: std::ptr::null_mut(),
            u_interchange_instanced_foliage_type_factory_node_get_custom_random_pitch_angle: std::ptr::null_mut(),
            u_interchange_instanced_foliage_type_factory_node_get_custom_align_to_normal: std::ptr::null_mut(),
            u_interchange_instanced_foliage_type_factory_node_get_custom_affect_distance_field_lighting: std::ptr::null_mut(),
        }
    }
}
pub fn initialize() {
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UFabBrowserApi::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetPreferredQualityTier"),
            &raw mut __FUNCTION_PTRS.u_fab_browser_api_set_preferred_quality_tier,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("PluginOpened"),
            &raw mut __FUNCTION_PTRS.u_fab_browser_api_plugin_opened,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("OpenUrlInBrowser"),
            &raw mut __FUNCTION_PTRS.u_fab_browser_api_open_url_in_browser,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("OpenPluginSettings"),
            &raw mut __FUNCTION_PTRS.u_fab_browser_api_open_plugin_settings,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("OnDragInfoSuccess"),
            &raw mut __FUNCTION_PTRS.u_fab_browser_api_on_drag_info_success,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("OnDragInfoFailure"),
            &raw mut __FUNCTION_PTRS.u_fab_browser_api_on_drag_info_failure,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("Logout"),
            &raw mut __FUNCTION_PTRS.u_fab_browser_api_logout,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("Login"),
            &raw mut __FUNCTION_PTRS.u_fab_browser_api_login,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetUrl"),
            &raw mut __FUNCTION_PTRS.u_fab_browser_api_get_url,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetSettings"),
            &raw mut __FUNCTION_PTRS.u_fab_browser_api_get_settings,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetRefreshToken"),
            &raw mut __FUNCTION_PTRS.u_fab_browser_api_get_refresh_token,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetAuthToken"),
            &raw mut __FUNCTION_PTRS.u_fab_browser_api_get_auth_token,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetApiVersion"),
            &raw mut __FUNCTION_PTRS.u_fab_browser_api_get_api_version,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("DragStart"),
            &raw mut __FUNCTION_PTRS.u_fab_browser_api_drag_start,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CopyToClipboard"),
            &raw mut __FUNCTION_PTRS.u_fab_browser_api_copy_to_clipboard,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AddToProject"),
            &raw mut __FUNCTION_PTRS.u_fab_browser_api_add_to_project,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UInterchangeInstancedFoliageTypeFactoryNode::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetCustomWorldPositionOffsetDisableDistance"),
            &raw mut __FUNCTION_PTRS
                .u_interchange_instanced_foliage_type_factory_node_set_custom_world_position_offset_disable_distance,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetCustomStaticMesh"),
            &raw mut __FUNCTION_PTRS
                .u_interchange_instanced_foliage_type_factory_node_set_custom_static_mesh,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetCustomScaling"),
            &raw mut __FUNCTION_PTRS
                .u_interchange_instanced_foliage_type_factory_node_set_custom_scaling,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetCustomScaleZ"),
            &raw mut __FUNCTION_PTRS
                .u_interchange_instanced_foliage_type_factory_node_set_custom_scale_z,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetCustomScaleY"),
            &raw mut __FUNCTION_PTRS
                .u_interchange_instanced_foliage_type_factory_node_set_custom_scale_y,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetCustomScaleX"),
            &raw mut __FUNCTION_PTRS
                .u_interchange_instanced_foliage_type_factory_node_set_custom_scale_x,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetCustomRandomYaw"),
            &raw mut __FUNCTION_PTRS
                .u_interchange_instanced_foliage_type_factory_node_set_custom_random_yaw,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetCustomRandomPitchAngle"),
            &raw mut __FUNCTION_PTRS
                .u_interchange_instanced_foliage_type_factory_node_set_custom_random_pitch_angle,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetCustomAlignToNormal"),
            &raw mut __FUNCTION_PTRS
                .u_interchange_instanced_foliage_type_factory_node_set_custom_align_to_normal,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetCustomAffectDistanceFieldLighting"),
            &raw mut __FUNCTION_PTRS
                .u_interchange_instanced_foliage_type_factory_node_set_custom_affect_distance_field_lighting,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetCustomWorldPositionOffsetDisableDistance"),
            &raw mut __FUNCTION_PTRS
                .u_interchange_instanced_foliage_type_factory_node_get_custom_world_position_offset_disable_distance,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetCustomStaticMesh"),
            &raw mut __FUNCTION_PTRS
                .u_interchange_instanced_foliage_type_factory_node_get_custom_static_mesh,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetCustomScaling"),
            &raw mut __FUNCTION_PTRS
                .u_interchange_instanced_foliage_type_factory_node_get_custom_scaling,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetCustomScaleZ"),
            &raw mut __FUNCTION_PTRS
                .u_interchange_instanced_foliage_type_factory_node_get_custom_scale_z,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetCustomScaleY"),
            &raw mut __FUNCTION_PTRS
                .u_interchange_instanced_foliage_type_factory_node_get_custom_scale_y,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetCustomScaleX"),
            &raw mut __FUNCTION_PTRS
                .u_interchange_instanced_foliage_type_factory_node_get_custom_scale_x,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetCustomRandomYaw"),
            &raw mut __FUNCTION_PTRS
                .u_interchange_instanced_foliage_type_factory_node_get_custom_random_yaw,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetCustomRandomPitchAngle"),
            &raw mut __FUNCTION_PTRS
                .u_interchange_instanced_foliage_type_factory_node_get_custom_random_pitch_angle,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetCustomAlignToNormal"),
            &raw mut __FUNCTION_PTRS
                .u_interchange_instanced_foliage_type_factory_node_get_custom_align_to_normal,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetCustomAffectDistanceFieldLighting"),
            &raw mut __FUNCTION_PTRS
                .u_interchange_instanced_foliage_type_factory_node_get_custom_affect_distance_field_lighting,
        );
    }
}
#[repr(C, align(8))]
pub struct UFabLocalAssets {
    __padding_end: [u8; 208],
}
impl UFabLocalAssets {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UFabLocalAssets")
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
pub struct UEosConstants {
    __padding_end: [u8; 312],
}
impl UEosConstants {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEosConstants")
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
pub struct UFabBrowserApi {
    __padding_end: [u8; 96],
}
impl UFabBrowserApi {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UFabBrowserApi")
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
pub struct UFabSettings {
    __padding_end: [u8; 152],
}
impl UFabSettings {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UFabSettings")
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
pub struct UFabPlaceholderSpawner {
    __padding_end: [u8; 168],
}
impl UFabPlaceholderSpawner {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UFabPlaceholderSpawner")
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
pub struct UFabStaticMeshPlaceholderSpawner {
    __padding_end: [u8; 168],
}
impl UFabStaticMeshPlaceholderSpawner {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UFabStaticMeshPlaceholderSpawner")
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
pub struct UFabSkeletalMeshPlaceholderSpawner {
    __padding_end: [u8; 168],
}
impl UFabSkeletalMeshPlaceholderSpawner {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UFabSkeletalMeshPlaceholderSpawner")
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
pub struct UFabDecalPlaceholderSpawner {
    __padding_end: [u8; 168],
}
impl UFabDecalPlaceholderSpawner {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UFabDecalPlaceholderSpawner")
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
pub struct UInterchangeInstancedFoliageTypeFactory {
    __padding_end: [u8; 56],
}
impl UInterchangeInstancedFoliageTypeFactory {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UInterchangeInstancedFoliageTypeFactory")
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
pub struct UMegascansMaterialParentSettings {
    __padding_end: [u8; 184],
}
impl UMegascansMaterialParentSettings {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMegascansMaterialParentSettings")
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
pub struct UInterchangeMegascansPipeline {
    #[doc(hidden)]
    __padding_344: [u8; 344],
    pub megascan_import_type: EMegascanImportType,
    pub megascans_material_parent_settings: UPtr<UMegascansMaterialParentSettings>,
    __padding_end: [u8; 80],
}
impl UInterchangeMegascansPipeline {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UInterchangeMegascansPipeline")
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
pub struct UInterchangeInstancedFoliageTypeFactoryNode {
    __padding_end: [u8; 624],
}
impl UInterchangeInstancedFoliageTypeFactoryNode {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UInterchangeInstancedFoliageTypeFactoryNode")
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
    pub fn set_custom_world_position_offset_disable_distance(
        &mut self,
        attribute_value: i32,
        b_add_apply_delegate: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<6>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::fab::__FUNCTION_PTRS
                    .u_interchange_instanced_foliage_type_factory_node_set_custom_world_position_offset_disable_distance,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &attribute_value,
                __buffer.add(0).cast::<i32>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_add_apply_delegate,
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
                crate::bindings::fab::__FUNCTION_PTRS
                    .u_interchange_instanced_foliage_type_factory_node_set_custom_world_position_offset_disable_distance,
                __buffer,
            )
        };
        unsafe { __buffer.add(5).cast::<bool>().read() }
    }
    pub fn set_custom_static_mesh(&mut self, attribute_value: FString) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<17>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::fab::__FUNCTION_PTRS
                    .u_interchange_instanced_foliage_type_factory_node_set_custom_static_mesh,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &attribute_value,
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
                crate::bindings::fab::__FUNCTION_PTRS
                    .u_interchange_instanced_foliage_type_factory_node_set_custom_static_mesh,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<bool>().read() }
    }
    pub fn set_custom_scaling(
        &mut self,
        attribute_value: crate::bindings::foliage::EFoliageScaling,
        b_add_apply_delegate: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<3>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::fab::__FUNCTION_PTRS
                    .u_interchange_instanced_foliage_type_factory_node_set_custom_scaling,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &attribute_value,
                __buffer.add(0).cast::<crate::bindings::foliage::EFoliageScaling>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_add_apply_delegate,
                __buffer.add(1).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::fab::__FUNCTION_PTRS
                    .u_interchange_instanced_foliage_type_factory_node_set_custom_scaling,
                __buffer,
            )
        };
        unsafe { __buffer.add(2).cast::<bool>().read() }
    }
    pub fn set_custom_scale_z(
        &mut self,
        attribute_value: &crate::bindings::core_u_object::FVector2f,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<9>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::fab::__FUNCTION_PTRS
                    .u_interchange_instanced_foliage_type_factory_node_set_custom_scale_z,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                attribute_value,
                __buffer.add(0).cast::<crate::bindings::core_u_object::FVector2f>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::fab::__FUNCTION_PTRS
                    .u_interchange_instanced_foliage_type_factory_node_set_custom_scale_z,
                __buffer,
            )
        };
        unsafe { __buffer.add(8).cast::<bool>().read() }
    }
    pub fn set_custom_scale_y(
        &mut self,
        attribute_value: &crate::bindings::core_u_object::FVector2f,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<9>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::fab::__FUNCTION_PTRS
                    .u_interchange_instanced_foliage_type_factory_node_set_custom_scale_y,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                attribute_value,
                __buffer.add(0).cast::<crate::bindings::core_u_object::FVector2f>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::fab::__FUNCTION_PTRS
                    .u_interchange_instanced_foliage_type_factory_node_set_custom_scale_y,
                __buffer,
            )
        };
        unsafe { __buffer.add(8).cast::<bool>().read() }
    }
    pub fn set_custom_scale_x(
        &mut self,
        attribute_value: &crate::bindings::core_u_object::FVector2f,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<9>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::fab::__FUNCTION_PTRS
                    .u_interchange_instanced_foliage_type_factory_node_set_custom_scale_x,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                attribute_value,
                __buffer.add(0).cast::<crate::bindings::core_u_object::FVector2f>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::fab::__FUNCTION_PTRS
                    .u_interchange_instanced_foliage_type_factory_node_set_custom_scale_x,
                __buffer,
            )
        };
        unsafe { __buffer.add(8).cast::<bool>().read() }
    }
    pub fn set_custom_random_yaw(
        &mut self,
        attribute_value: bool,
        b_add_apply_delegate: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<3>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::fab::__FUNCTION_PTRS
                    .u_interchange_instanced_foliage_type_factory_node_set_custom_random_yaw,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &attribute_value,
                __buffer.add(0).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_add_apply_delegate,
                __buffer.add(1).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::fab::__FUNCTION_PTRS
                    .u_interchange_instanced_foliage_type_factory_node_set_custom_random_yaw,
                __buffer,
            )
        };
        unsafe { __buffer.add(2).cast::<bool>().read() }
    }
    pub fn set_custom_random_pitch_angle(
        &mut self,
        attribute_value: f32,
        b_add_apply_delegate: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<6>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::fab::__FUNCTION_PTRS
                    .u_interchange_instanced_foliage_type_factory_node_set_custom_random_pitch_angle,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &attribute_value,
                __buffer.add(0).cast::<f32>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_add_apply_delegate,
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
                crate::bindings::fab::__FUNCTION_PTRS
                    .u_interchange_instanced_foliage_type_factory_node_set_custom_random_pitch_angle,
                __buffer,
            )
        };
        unsafe { __buffer.add(5).cast::<bool>().read() }
    }
    pub fn set_custom_align_to_normal(
        &mut self,
        attribute_value: bool,
        b_add_apply_delegate: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<3>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::fab::__FUNCTION_PTRS
                    .u_interchange_instanced_foliage_type_factory_node_set_custom_align_to_normal,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &attribute_value,
                __buffer.add(0).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_add_apply_delegate,
                __buffer.add(1).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::fab::__FUNCTION_PTRS
                    .u_interchange_instanced_foliage_type_factory_node_set_custom_align_to_normal,
                __buffer,
            )
        };
        unsafe { __buffer.add(2).cast::<bool>().read() }
    }
    pub fn set_custom_affect_distance_field_lighting(
        &mut self,
        attribute_value: bool,
        b_add_apply_delegate: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<3>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::fab::__FUNCTION_PTRS
                    .u_interchange_instanced_foliage_type_factory_node_set_custom_affect_distance_field_lighting,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &attribute_value,
                __buffer.add(0).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_add_apply_delegate,
                __buffer.add(1).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::fab::__FUNCTION_PTRS
                    .u_interchange_instanced_foliage_type_factory_node_set_custom_affect_distance_field_lighting,
                __buffer,
            )
        };
        unsafe { __buffer.add(2).cast::<bool>().read() }
    }
    pub fn get_custom_world_position_offset_disable_distance(
        &self,
        attribute_value: &mut i32,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<5>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::fab::__FUNCTION_PTRS
                    .u_interchange_instanced_foliage_type_factory_node_get_custom_world_position_offset_disable_distance,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                attribute_value,
                __buffer.add(0).cast::<i32>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::fab::__FUNCTION_PTRS
                    .u_interchange_instanced_foliage_type_factory_node_get_custom_world_position_offset_disable_distance,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<i32>().swap(attribute_value);
        }
        unsafe { __buffer.add(4).cast::<bool>().read() }
    }
    pub fn get_custom_static_mesh(&self, attribute_value: &mut FString) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<17>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::fab::__FUNCTION_PTRS
                    .u_interchange_instanced_foliage_type_factory_node_get_custom_static_mesh,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                attribute_value,
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
                crate::bindings::fab::__FUNCTION_PTRS
                    .u_interchange_instanced_foliage_type_factory_node_get_custom_static_mesh,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<FString>().swap(attribute_value);
        }
        unsafe { __buffer.add(16).cast::<bool>().read() }
    }
    pub fn get_custom_scaling(
        &self,
        attribute_value: &mut crate::bindings::foliage::EFoliageScaling,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<2>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::fab::__FUNCTION_PTRS
                    .u_interchange_instanced_foliage_type_factory_node_get_custom_scaling,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                attribute_value,
                __buffer.add(0).cast::<crate::bindings::foliage::EFoliageScaling>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::fab::__FUNCTION_PTRS
                    .u_interchange_instanced_foliage_type_factory_node_get_custom_scaling,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(0)
                .cast::<crate::bindings::foliage::EFoliageScaling>()
                .swap(attribute_value);
        }
        unsafe { __buffer.add(1).cast::<bool>().read() }
    }
    pub fn get_custom_scale_z(
        &self,
        attribute_value: &mut crate::bindings::core_u_object::FVector2f,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<9>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::fab::__FUNCTION_PTRS
                    .u_interchange_instanced_foliage_type_factory_node_get_custom_scale_z,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                attribute_value,
                __buffer.add(0).cast::<crate::bindings::core_u_object::FVector2f>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::fab::__FUNCTION_PTRS
                    .u_interchange_instanced_foliage_type_factory_node_get_custom_scale_z,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(0)
                .cast::<crate::bindings::core_u_object::FVector2f>()
                .swap(attribute_value);
        }
        unsafe { __buffer.add(8).cast::<bool>().read() }
    }
    pub fn get_custom_scale_y(
        &self,
        attribute_value: &mut crate::bindings::core_u_object::FVector2f,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<9>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::fab::__FUNCTION_PTRS
                    .u_interchange_instanced_foliage_type_factory_node_get_custom_scale_y,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                attribute_value,
                __buffer.add(0).cast::<crate::bindings::core_u_object::FVector2f>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::fab::__FUNCTION_PTRS
                    .u_interchange_instanced_foliage_type_factory_node_get_custom_scale_y,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(0)
                .cast::<crate::bindings::core_u_object::FVector2f>()
                .swap(attribute_value);
        }
        unsafe { __buffer.add(8).cast::<bool>().read() }
    }
    pub fn get_custom_scale_x(
        &self,
        attribute_value: &mut crate::bindings::core_u_object::FVector2f,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<9>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::fab::__FUNCTION_PTRS
                    .u_interchange_instanced_foliage_type_factory_node_get_custom_scale_x,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                attribute_value,
                __buffer.add(0).cast::<crate::bindings::core_u_object::FVector2f>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::fab::__FUNCTION_PTRS
                    .u_interchange_instanced_foliage_type_factory_node_get_custom_scale_x,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(0)
                .cast::<crate::bindings::core_u_object::FVector2f>()
                .swap(attribute_value);
        }
        unsafe { __buffer.add(8).cast::<bool>().read() }
    }
    pub fn get_custom_random_yaw(&self, attribute_value: &mut bool) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<2>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::fab::__FUNCTION_PTRS
                    .u_interchange_instanced_foliage_type_factory_node_get_custom_random_yaw,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                attribute_value,
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
                crate::bindings::fab::__FUNCTION_PTRS
                    .u_interchange_instanced_foliage_type_factory_node_get_custom_random_yaw,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<bool>().swap(attribute_value);
        }
        unsafe { __buffer.add(1).cast::<bool>().read() }
    }
    pub fn get_custom_random_pitch_angle(&self, attribute_value: &mut f32) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<5>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::fab::__FUNCTION_PTRS
                    .u_interchange_instanced_foliage_type_factory_node_get_custom_random_pitch_angle,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                attribute_value,
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
                crate::bindings::fab::__FUNCTION_PTRS
                    .u_interchange_instanced_foliage_type_factory_node_get_custom_random_pitch_angle,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<f32>().swap(attribute_value);
        }
        unsafe { __buffer.add(4).cast::<bool>().read() }
    }
    pub fn get_custom_align_to_normal(&self, attribute_value: &mut bool) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<2>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::fab::__FUNCTION_PTRS
                    .u_interchange_instanced_foliage_type_factory_node_get_custom_align_to_normal,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                attribute_value,
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
                crate::bindings::fab::__FUNCTION_PTRS
                    .u_interchange_instanced_foliage_type_factory_node_get_custom_align_to_normal,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<bool>().swap(attribute_value);
        }
        unsafe { __buffer.add(1).cast::<bool>().read() }
    }
    pub fn get_custom_affect_distance_field_lighting(
        &self,
        attribute_value: &mut bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<2>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::fab::__FUNCTION_PTRS
                    .u_interchange_instanced_foliage_type_factory_node_get_custom_affect_distance_field_lighting,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                attribute_value,
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
                crate::bindings::fab::__FUNCTION_PTRS
                    .u_interchange_instanced_foliage_type_factory_node_get_custom_affect_distance_field_lighting,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<bool>().swap(attribute_value);
        }
        unsafe { __buffer.add(1).cast::<bool>().read() }
    }
}
#[repr(C, align(8))]
pub struct UFabFactory {
    __padding_end: [u8; 56],
}
impl UFabFactory {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UFabFactory")
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
pub struct EFabEnvironment(pub u8);
impl EFabEnvironment {
    pub const PROD: EFabEnvironment = EFabEnvironment(0);
    pub const GAMEDEV: EFabEnvironment = EFabEnvironment(1);
    pub const TEST: EFabEnvironment = EFabEnvironment(2);
    pub const CUSTOM_URL: EFabEnvironment = EFabEnvironment(3);
}
#[repr(transparent)]
pub struct EFabPreferredFormats(pub u8);
impl EFabPreferredFormats {
    pub const GLTF: EFabPreferredFormats = EFabPreferredFormats(0);
    pub const FBX: EFabPreferredFormats = EFabPreferredFormats(1);
}
#[repr(transparent)]
pub struct EFabPreferredQualityTier(pub u8);
impl EFabPreferredQualityTier {
    pub const LOW: EFabPreferredQualityTier = EFabPreferredQualityTier(0);
    pub const MEDIUM: EFabPreferredQualityTier = EFabPreferredQualityTier(1);
    pub const HIGH: EFabPreferredQualityTier = EFabPreferredQualityTier(2);
    pub const RAW: EFabPreferredQualityTier = EFabPreferredQualityTier(3);
}
#[repr(transparent)]
pub struct EMegascanMaterialType(pub i32);
impl EMegascanMaterialType {
    pub const INVALID: EMegascanMaterialType = EMegascanMaterialType(0);
    pub const BASE: EMegascanMaterialType = EMegascanMaterialType(1);
    pub const BASE_MASKED: EMegascanMaterialType = EMegascanMaterialType(2);
    pub const BASE_FUZZ: EMegascanMaterialType = EMegascanMaterialType(3);
    pub const BASE_TRANSMISSION: EMegascanMaterialType = EMegascanMaterialType(4);
    pub const GLASS: EMegascanMaterialType = EMegascanMaterialType(5);
    pub const SURFACE: EMegascanMaterialType = EMegascanMaterialType(6);
    pub const SURFACE_MASKED: EMegascanMaterialType = EMegascanMaterialType(7);
    pub const SURFACE_FUZZ: EMegascanMaterialType = EMegascanMaterialType(8);
    pub const SURFACE_TRANSMISSION: EMegascanMaterialType = EMegascanMaterialType(9);
    pub const FABRIC: EMegascanMaterialType = EMegascanMaterialType(10);
    pub const FABRIC_MASKED: EMegascanMaterialType = EMegascanMaterialType(11);
    pub const DECAL: EMegascanMaterialType = EMegascanMaterialType(12);
    pub const PLANT: EMegascanMaterialType = EMegascanMaterialType(13);
    pub const PLANT_BILLBOARD: EMegascanMaterialType = EMegascanMaterialType(14);
}
#[repr(transparent)]
pub struct EMegascanImportType(pub i32);
impl EMegascanImportType {
    pub const MODEL3_D: EMegascanImportType = EMegascanImportType(0);
    pub const SURFACE: EMegascanImportType = EMegascanImportType(1);
    pub const DECAL: EMegascanImportType = EMegascanImportType(2);
    pub const IMPERFECTION: EMegascanImportType = EMegascanImportType(3);
    pub const PLANT: EMegascanImportType = EMegascanImportType(4);
}
