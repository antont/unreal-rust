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
    pub u_navigation_path_is_valid: *mut crate::ffi::UFunctionOpague,
    pub u_navigation_path_is_string_pulled: *mut crate::ffi::UFunctionOpague,
    pub u_navigation_path_is_partial: *mut crate::ffi::UFunctionOpague,
    pub u_navigation_path_get_path_length: *mut crate::ffi::UFunctionOpague,
    pub u_navigation_path_get_path_cost: *mut crate::ffi::UFunctionOpague,
    pub u_navigation_path_get_debug_string: *mut crate::ffi::UFunctionOpague,
    pub u_navigation_path_enable_recalculation_on_invalidation: *mut crate::ffi::UFunctionOpague,
    pub u_navigation_path_enable_debug_drawing: *mut crate::ffi::UFunctionOpague,
    pub u_navigation_system_v1_unregister_navigation_invoker: *mut crate::ffi::UFunctionOpague,
    pub u_navigation_system_v1_set_max_simultaneous_tile_generation_jobs_count: *mut crate::ffi::UFunctionOpague,
    pub u_navigation_system_v1_set_geometry_gathering_mode: *mut crate::ffi::UFunctionOpague,
    pub u_navigation_system_v1_reset_max_simultaneous_tile_generation_jobs_count: *mut crate::ffi::UFunctionOpague,
    pub u_navigation_system_v1_register_navigation_invoker: *mut crate::ffi::UFunctionOpague,
    pub u_navigation_system_v1_on_navigation_bounds_updated: *mut crate::ffi::UFunctionOpague,
    pub u_navigation_system_v1_navigation_raycast: *mut crate::ffi::UFunctionOpague,
    pub u_navigation_system_v1_k2_replace_area_in_octree_data: *mut crate::ffi::UFunctionOpague,
    pub u_navigation_system_v1_k2_project_point_to_navigation: *mut crate::ffi::UFunctionOpague,
    pub u_navigation_system_v1_k2_get_random_reachable_point_in_radius: *mut crate::ffi::UFunctionOpague,
    pub u_navigation_system_v1_k2_get_random_point_in_navigable_radius: *mut crate::ffi::UFunctionOpague,
    pub u_navigation_system_v1_k2_get_random_location_in_navigable_radius: *mut crate::ffi::UFunctionOpague,
    pub u_navigation_system_v1_is_navigation_being_built_or_locked: *mut crate::ffi::UFunctionOpague,
    pub u_navigation_system_v1_is_navigation_being_built: *mut crate::ffi::UFunctionOpague,
    pub u_navigation_system_v1_get_path_length: *mut crate::ffi::UFunctionOpague,
    pub u_navigation_system_v1_get_path_cost: *mut crate::ffi::UFunctionOpague,
    pub u_navigation_system_v1_get_navigation_system: *mut crate::ffi::UFunctionOpague,
    pub u_navigation_system_v1_find_path_to_location_synchronously: *mut crate::ffi::UFunctionOpague,
    pub u_navigation_system_v1_find_path_to_actor_synchronously: *mut crate::ffi::UFunctionOpague,
    pub u_nav_relevant_component_set_navigation_relevancy: *mut crate::ffi::UFunctionOpague,
    pub a_recast_nav_mesh_k2_replace_area_in_tile_bounds: *mut crate::ffi::UFunctionOpague,
    pub u_nav_modifier_component_set_area_class_to_replace: *mut crate::ffi::UFunctionOpague,
    pub u_nav_modifier_component_set_area_class: *mut crate::ffi::UFunctionOpague,
    pub a_nav_modifier_volume_set_area_class_to_replace: *mut crate::ffi::UFunctionOpague,
    pub a_nav_modifier_volume_set_area_class: *mut crate::ffi::UFunctionOpague,
    pub a_nav_system_config_override_apply_changes: *mut crate::ffi::UFunctionOpague,
    pub u_spline_nav_modifier_component_update_navigation_with_component_data: *mut crate::ffi::UFunctionOpague,
}
impl FunctionPtrs {
    pub const fn empty() -> Self {
        Self {
            u_navigation_path_is_valid: std::ptr::null_mut(),
            u_navigation_path_is_string_pulled: std::ptr::null_mut(),
            u_navigation_path_is_partial: std::ptr::null_mut(),
            u_navigation_path_get_path_length: std::ptr::null_mut(),
            u_navigation_path_get_path_cost: std::ptr::null_mut(),
            u_navigation_path_get_debug_string: std::ptr::null_mut(),
            u_navigation_path_enable_recalculation_on_invalidation: std::ptr::null_mut(),
            u_navigation_path_enable_debug_drawing: std::ptr::null_mut(),
            u_navigation_system_v1_unregister_navigation_invoker: std::ptr::null_mut(),
            u_navigation_system_v1_set_max_simultaneous_tile_generation_jobs_count: std::ptr::null_mut(),
            u_navigation_system_v1_set_geometry_gathering_mode: std::ptr::null_mut(),
            u_navigation_system_v1_reset_max_simultaneous_tile_generation_jobs_count: std::ptr::null_mut(),
            u_navigation_system_v1_register_navigation_invoker: std::ptr::null_mut(),
            u_navigation_system_v1_on_navigation_bounds_updated: std::ptr::null_mut(),
            u_navigation_system_v1_navigation_raycast: std::ptr::null_mut(),
            u_navigation_system_v1_k2_replace_area_in_octree_data: std::ptr::null_mut(),
            u_navigation_system_v1_k2_project_point_to_navigation: std::ptr::null_mut(),
            u_navigation_system_v1_k2_get_random_reachable_point_in_radius: std::ptr::null_mut(),
            u_navigation_system_v1_k2_get_random_point_in_navigable_radius: std::ptr::null_mut(),
            u_navigation_system_v1_k2_get_random_location_in_navigable_radius: std::ptr::null_mut(),
            u_navigation_system_v1_is_navigation_being_built_or_locked: std::ptr::null_mut(),
            u_navigation_system_v1_is_navigation_being_built: std::ptr::null_mut(),
            u_navigation_system_v1_get_path_length: std::ptr::null_mut(),
            u_navigation_system_v1_get_path_cost: std::ptr::null_mut(),
            u_navigation_system_v1_get_navigation_system: std::ptr::null_mut(),
            u_navigation_system_v1_find_path_to_location_synchronously: std::ptr::null_mut(),
            u_navigation_system_v1_find_path_to_actor_synchronously: std::ptr::null_mut(),
            u_nav_relevant_component_set_navigation_relevancy: std::ptr::null_mut(),
            a_recast_nav_mesh_k2_replace_area_in_tile_bounds: std::ptr::null_mut(),
            u_nav_modifier_component_set_area_class_to_replace: std::ptr::null_mut(),
            u_nav_modifier_component_set_area_class: std::ptr::null_mut(),
            a_nav_modifier_volume_set_area_class_to_replace: std::ptr::null_mut(),
            a_nav_modifier_volume_set_area_class: std::ptr::null_mut(),
            a_nav_system_config_override_apply_changes: std::ptr::null_mut(),
            u_spline_nav_modifier_component_update_navigation_with_component_data: std::ptr::null_mut(),
        }
    }
}
pub fn initialize() {
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UNavigationPath::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("IsValid"),
                &raw mut __FUNCTION_PTRS.u_navigation_path_is_valid,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("IsStringPulled"),
                &raw mut __FUNCTION_PTRS.u_navigation_path_is_string_pulled,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("IsPartial"),
                &raw mut __FUNCTION_PTRS.u_navigation_path_is_partial,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetPathLength"),
                &raw mut __FUNCTION_PTRS.u_navigation_path_get_path_length,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetPathCost"),
                &raw mut __FUNCTION_PTRS.u_navigation_path_get_path_cost,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetDebugString"),
                &raw mut __FUNCTION_PTRS.u_navigation_path_get_debug_string,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("EnableRecalculationOnInvalidation"),
                &raw mut __FUNCTION_PTRS
                    .u_navigation_path_enable_recalculation_on_invalidation,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("EnableDebugDrawing"),
                &raw mut __FUNCTION_PTRS.u_navigation_path_enable_debug_drawing,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UNavigationSystemV1::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("UnregisterNavigationInvoker"),
                &raw mut __FUNCTION_PTRS
                    .u_navigation_system_v1_unregister_navigation_invoker,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetMaxSimultaneousTileGenerationJobsCount"),
                &raw mut __FUNCTION_PTRS
                    .u_navigation_system_v1_set_max_simultaneous_tile_generation_jobs_count,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetGeometryGatheringMode"),
                &raw mut __FUNCTION_PTRS
                    .u_navigation_system_v1_set_geometry_gathering_mode,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ResetMaxSimultaneousTileGenerationJobsCount"),
                &raw mut __FUNCTION_PTRS
                    .u_navigation_system_v1_reset_max_simultaneous_tile_generation_jobs_count,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("RegisterNavigationInvoker"),
                &raw mut __FUNCTION_PTRS
                    .u_navigation_system_v1_register_navigation_invoker,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("OnNavigationBoundsUpdated"),
                &raw mut __FUNCTION_PTRS
                    .u_navigation_system_v1_on_navigation_bounds_updated,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("NavigationRaycast"),
                &raw mut __FUNCTION_PTRS.u_navigation_system_v1_navigation_raycast,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("K2_ReplaceAreaInOctreeData"),
                &raw mut __FUNCTION_PTRS
                    .u_navigation_system_v1_k2_replace_area_in_octree_data,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("K2_ProjectPointToNavigation"),
                &raw mut __FUNCTION_PTRS
                    .u_navigation_system_v1_k2_project_point_to_navigation,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("K2_GetRandomReachablePointInRadius"),
                &raw mut __FUNCTION_PTRS
                    .u_navigation_system_v1_k2_get_random_reachable_point_in_radius,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("K2_GetRandomPointInNavigableRadius"),
                &raw mut __FUNCTION_PTRS
                    .u_navigation_system_v1_k2_get_random_point_in_navigable_radius,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("K2_GetRandomLocationInNavigableRadius"),
                &raw mut __FUNCTION_PTRS
                    .u_navigation_system_v1_k2_get_random_location_in_navigable_radius,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("IsNavigationBeingBuiltOrLocked"),
                &raw mut __FUNCTION_PTRS
                    .u_navigation_system_v1_is_navigation_being_built_or_locked,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("IsNavigationBeingBuilt"),
                &raw mut __FUNCTION_PTRS.u_navigation_system_v1_is_navigation_being_built,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetPathLength"),
                &raw mut __FUNCTION_PTRS.u_navigation_system_v1_get_path_length,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetPathCost"),
                &raw mut __FUNCTION_PTRS.u_navigation_system_v1_get_path_cost,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetNavigationSystem"),
                &raw mut __FUNCTION_PTRS.u_navigation_system_v1_get_navigation_system,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("FindPathToLocationSynchronously"),
                &raw mut __FUNCTION_PTRS
                    .u_navigation_system_v1_find_path_to_location_synchronously,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("FindPathToActorSynchronously"),
                &raw mut __FUNCTION_PTRS
                    .u_navigation_system_v1_find_path_to_actor_synchronously,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UNavRelevantComponent::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetNavigationRelevancy"),
                &raw mut __FUNCTION_PTRS
                    .u_nav_relevant_component_set_navigation_relevancy,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = ARecastNavMesh::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("K2_ReplaceAreaInTileBounds"),
                &raw mut __FUNCTION_PTRS.a_recast_nav_mesh_k2_replace_area_in_tile_bounds,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UNavModifierComponent::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetAreaClassToReplace"),
                &raw mut __FUNCTION_PTRS
                    .u_nav_modifier_component_set_area_class_to_replace,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetAreaClass"),
                &raw mut __FUNCTION_PTRS.u_nav_modifier_component_set_area_class,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = ANavModifierVolume::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetAreaClassToReplace"),
                &raw mut __FUNCTION_PTRS.a_nav_modifier_volume_set_area_class_to_replace,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetAreaClass"),
                &raw mut __FUNCTION_PTRS.a_nav_modifier_volume_set_area_class,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = ANavSystemConfigOverride::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ApplyChanges"),
                &raw mut __FUNCTION_PTRS.a_nav_system_config_override_apply_changes,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = USplineNavModifierComponent::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("UpdateNavigationWithComponentData"),
                &raw mut __FUNCTION_PTRS
                    .u_spline_nav_modifier_component_update_navigation_with_component_data,
            );
        }
    }
}
#[repr(C, align(8))]
pub struct UCrowdManagerBase {
    __padding_end: [u8; 48],
}
impl UCrowdManagerBase {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UCrowdManagerBase")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UCrowdManagerBase")
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
pub struct UBaseGeneratedNavLinksProxy {
    __padding_end: [u8; 72],
}
impl UBaseGeneratedNavLinksProxy {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBaseGeneratedNavLinksProxy")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBaseGeneratedNavLinksProxy")
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
pub struct UNavigationQueryFilter {
    __padding_end: [u8; 80],
}
impl UNavigationQueryFilter {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNavigationQueryFilter")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNavigationQueryFilter")
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
pub struct ANavigationGraphNode {
    __padding_end: [u8; 1136],
}
impl ANavigationGraphNode {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("ANavigationGraphNode")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("ANavigationGraphNode")
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
pub struct UNavigationGraphNodeComponent {
    __padding_end: [u8; 704],
}
impl UNavigationGraphNodeComponent {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNavigationGraphNodeComponent")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNavigationGraphNodeComponent")
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
pub struct INavigationPathGenerator {}
#[repr(C, align(8))]
pub struct UNavigationPathGenerator {
    __padding_end: [u8; 48],
}
impl UNavigationPathGenerator {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNavigationPathGenerator")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNavigationPathGenerator")
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
pub struct INavLinkCustomInterface {}
#[repr(C, align(8))]
pub struct UNavLinkCustomInterface {
    __padding_end: [u8; 48],
}
impl UNavLinkCustomInterface {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNavLinkCustomInterface")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNavLinkCustomInterface")
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
pub struct INavLinkHostInterface {}
#[repr(C, align(8))]
pub struct UNavLinkHostInterface {
    __padding_end: [u8; 48],
}
impl UNavLinkHostInterface {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNavLinkHostInterface")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNavLinkHostInterface")
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
pub struct UNavLinkTrivial {
    __padding_end: [u8; 88],
}
impl UNavLinkTrivial {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNavLinkTrivial")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNavLinkTrivial")
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
pub struct INavNodeInterface {}
#[repr(C, align(8))]
pub struct UNavNodeInterface {
    __padding_end: [u8; 48],
}
impl UNavNodeInterface {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNavNodeInterface")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNavNodeInterface")
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
pub struct ANavigationData {
    __padding_end: [u8; 1760],
}
impl ANavigationData {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("ANavigationData")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("ANavigationData")
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
pub struct AAbstractNavData {
    __padding_end: [u8; 1760],
}
impl AAbstractNavData {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("AAbstractNavData")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("AAbstractNavData")
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
pub struct UNavArea {
    __padding_end: [u8; 80],
}
impl UNavArea {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNavArea")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS.wait().name_to_ptr.get("UNavArea").copied()
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
pub struct UNavAreaMeta {
    __padding_end: [u8; 80],
}
impl UNavAreaMeta {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNavAreaMeta")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNavAreaMeta")
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
pub struct UNavAreaMeta_SwitchByAgent {
    __padding_end: [u8; 208],
}
impl UNavAreaMeta_SwitchByAgent {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNavAreaMeta_SwitchByAgent")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNavAreaMeta_SwitchByAgent")
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
pub struct UNavArea_Default {
    __padding_end: [u8; 80],
}
impl UNavArea_Default {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNavArea_Default")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNavArea_Default")
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
pub struct UNavArea_LowHeight {
    __padding_end: [u8; 80],
}
impl UNavArea_LowHeight {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNavArea_LowHeight")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNavArea_LowHeight")
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
pub struct UNavArea_Null {
    __padding_end: [u8; 80],
}
impl UNavArea_Null {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNavArea_Null")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNavArea_Null")
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
pub struct UNavArea_Obstacle {
    __padding_end: [u8; 80],
}
impl UNavArea_Obstacle {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNavArea_Obstacle")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNavArea_Obstacle")
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
pub struct UNavCollision {
    __padding_end: [u8; 312],
}
impl UNavCollision {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNavCollision")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNavCollision")
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
pub struct URecastFilter_UseDefaultArea {
    __padding_end: [u8; 80],
}
impl URecastFilter_UseDefaultArea {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("URecastFilter_UseDefaultArea")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("URecastFilter_UseDefaultArea")
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
pub struct ANavigationGraph {
    __padding_end: [u8; 1760],
}
impl ANavigationGraph {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("ANavigationGraph")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("ANavigationGraph")
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
pub struct UNavigationInvokerComponent {
    #[doc(hidden)]
    pub(crate) __padding_240: [u8; 240],
    pub tile_generation_radius: f32,
    pub tile_removal_radius: f32,
    __padding_end: [u8; 8],
}
impl UNavigationInvokerComponent {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNavigationInvokerComponent")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNavigationInvokerComponent")
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
pub struct UNavigationObjectRepository {
    __padding_end: [u8; 392],
}
impl UNavigationObjectRepository {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNavigationObjectRepository")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNavigationObjectRepository")
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
pub struct UNavigationPath {
    #[doc(hidden)]
    pub(crate) __padding_72: [u8; 72],
    pub path_points: TArray<crate::bindings::core_u_object::FVector>,
    pub recalculate_on_invalidation: crate::bindings::engine::ENavigationOptionFlag,
    __padding_end: [u8; 71],
}
impl UNavigationPath {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNavigationPath")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNavigationPath")
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
    pub fn is_valid(&self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::navigation_system::__FUNCTION_PTRS
                    .u_navigation_path_is_valid,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::navigation_system::__FUNCTION_PTRS
                    .u_navigation_path_is_valid,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn is_string_pulled(&self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::navigation_system::__FUNCTION_PTRS
                    .u_navigation_path_is_string_pulled,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::navigation_system::__FUNCTION_PTRS
                    .u_navigation_path_is_string_pulled,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn is_partial(&self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::navigation_system::__FUNCTION_PTRS
                    .u_navigation_path_is_partial,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::navigation_system::__FUNCTION_PTRS
                    .u_navigation_path_is_partial,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn get_path_length(&self) -> f64 {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::navigation_system::__FUNCTION_PTRS
                    .u_navigation_path_get_path_length,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::navigation_system::__FUNCTION_PTRS
                    .u_navigation_path_get_path_length,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<f64>().read() }
    }
    pub fn get_path_cost(&self) -> f64 {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::navigation_system::__FUNCTION_PTRS
                    .u_navigation_path_get_path_cost,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::navigation_system::__FUNCTION_PTRS
                    .u_navigation_path_get_path_cost,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<f64>().read() }
    }
    pub fn get_debug_string(&self) -> FString {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::navigation_system::__FUNCTION_PTRS
                    .u_navigation_path_get_debug_string,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::navigation_system::__FUNCTION_PTRS
                    .u_navigation_path_get_debug_string,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<FString>().read() }
    }
    pub fn enable_recalculation_on_invalidation(
        &mut self,
        do_recalculation: crate::bindings::engine::ENavigationOptionFlag,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::navigation_system::__FUNCTION_PTRS
                    .u_navigation_path_enable_recalculation_on_invalidation,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &do_recalculation,
                __buffer.add(0).cast::<crate::bindings::engine::ENavigationOptionFlag>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::navigation_system::__FUNCTION_PTRS
                    .u_navigation_path_enable_recalculation_on_invalidation,
                __buffer,
            )
        };
        std::mem::forget(do_recalculation);
    }
    pub fn enable_debug_drawing(
        &mut self,
        b_should_draw_debug_data: bool,
        path_color: crate::bindings::core_u_object::FLinearColor,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<20>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::navigation_system::__FUNCTION_PTRS
                    .u_navigation_path_enable_debug_drawing,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_should_draw_debug_data,
                __buffer.add(0).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &path_color,
                __buffer.add(4).cast::<crate::bindings::core_u_object::FLinearColor>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::navigation_system::__FUNCTION_PTRS
                    .u_navigation_path_enable_debug_drawing,
                __buffer,
            )
        };
        std::mem::forget(b_should_draw_debug_data);
        std::mem::forget(path_color);
    }
}
#[repr(C, align(16))]
pub struct UNavigationSystemV1 {
    #[doc(hidden)]
    pub(crate) __padding_64: [u8; 64],
    pub default_agent_name: FName,
    pub crowd_manager_class: TSoftObjectPtr<crate::bindings::core_u_object::UClass>,
    __padding_end: [u8; 5712],
}
impl UNavigationSystemV1 {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNavigationSystemV1")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNavigationSystemV1")
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
    pub fn unregister_navigation_invoker(
        &mut self,
        invoker: UPtr<crate::bindings::engine::AActor>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::navigation_system::__FUNCTION_PTRS
                    .u_navigation_system_v1_unregister_navigation_invoker,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &invoker,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::AActor>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::navigation_system::__FUNCTION_PTRS
                    .u_navigation_system_v1_unregister_navigation_invoker,
                __buffer,
            )
        };
        std::mem::forget(invoker);
    }
    pub fn set_max_simultaneous_tile_generation_jobs_count(
        &mut self,
        max_number_of_jobs: i32,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::navigation_system::__FUNCTION_PTRS
                    .u_navigation_system_v1_set_max_simultaneous_tile_generation_jobs_count,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &max_number_of_jobs,
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
                crate::bindings::navigation_system::__FUNCTION_PTRS
                    .u_navigation_system_v1_set_max_simultaneous_tile_generation_jobs_count,
                __buffer,
            )
        };
        std::mem::forget(max_number_of_jobs);
    }
    pub fn set_geometry_gathering_mode(
        &mut self,
        new_mode: crate::bindings::engine::ENavDataGatheringModeConfig,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::navigation_system::__FUNCTION_PTRS
                    .u_navigation_system_v1_set_geometry_gathering_mode,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &new_mode,
                __buffer
                    .add(0)
                    .cast::<crate::bindings::engine::ENavDataGatheringModeConfig>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::navigation_system::__FUNCTION_PTRS
                    .u_navigation_system_v1_set_geometry_gathering_mode,
                __buffer,
            )
        };
        std::mem::forget(new_mode);
    }
    pub fn reset_max_simultaneous_tile_generation_jobs_count(&mut self) {
        let mut __stack = crate::core_data::StackAlloc::<0>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::navigation_system::__FUNCTION_PTRS
                    .u_navigation_system_v1_reset_max_simultaneous_tile_generation_jobs_count,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::navigation_system::__FUNCTION_PTRS
                    .u_navigation_system_v1_reset_max_simultaneous_tile_generation_jobs_count,
                __buffer,
            )
        };
    }
    pub fn register_navigation_invoker(
        &mut self,
        invoker: UPtr<crate::bindings::engine::AActor>,
        tile_generation_radius: f32,
        tile_removal_radius: f32,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::navigation_system::__FUNCTION_PTRS
                    .u_navigation_system_v1_register_navigation_invoker,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &invoker,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::AActor>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &tile_generation_radius,
                __buffer.add(8).cast::<f32>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &tile_removal_radius,
                __buffer.add(12).cast::<f32>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::navigation_system::__FUNCTION_PTRS
                    .u_navigation_system_v1_register_navigation_invoker,
                __buffer,
            )
        };
        std::mem::forget(invoker);
        std::mem::forget(tile_generation_radius);
        std::mem::forget(tile_removal_radius);
    }
    pub fn on_navigation_bounds_updated(
        &mut self,
        nav_volume: UPtr<ANavMeshBoundsVolume>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::navigation_system::__FUNCTION_PTRS
                    .u_navigation_system_v1_on_navigation_bounds_updated,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &nav_volume,
                __buffer.add(0).cast::<UPtr<ANavMeshBoundsVolume>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::navigation_system::__FUNCTION_PTRS
                    .u_navigation_system_v1_on_navigation_bounds_updated,
                __buffer,
            )
        };
        std::mem::forget(nav_volume);
    }
    pub fn navigation_raycast(
        world_context_object: UPtr<crate::bindings::core_u_object::UObject>,
        ray_start: &crate::bindings::core_u_object::FVector,
        ray_end: &crate::bindings::core_u_object::FVector,
        hit_location: &mut crate::bindings::core_u_object::FVector,
        filter_class: TSubclassOf<UNavigationQueryFilter>,
        querier: UPtr<crate::bindings::engine::AController>,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<97>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::navigation_system::__FUNCTION_PTRS
                    .u_navigation_system_v1_navigation_raycast,
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
                ray_start,
                __buffer.add(8).cast::<crate::bindings::core_u_object::FVector>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                ray_end,
                __buffer.add(32).cast::<crate::bindings::core_u_object::FVector>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                hit_location,
                __buffer.add(56).cast::<crate::bindings::core_u_object::FVector>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &filter_class,
                __buffer.add(80).cast::<TSubclassOf<UNavigationQueryFilter>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &querier,
                __buffer.add(88).cast::<UPtr<crate::bindings::engine::AController>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::navigation_system::UNavigationSystemV1::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::navigation_system::__FUNCTION_PTRS
                    .u_navigation_system_v1_navigation_raycast,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(56)
                .cast::<crate::bindings::core_u_object::FVector>()
                .swap(hit_location);
        }
        std::mem::forget(world_context_object);
        std::mem::forget(filter_class);
        std::mem::forget(querier);
        unsafe { __buffer.add(96).cast::<bool>().read() }
    }
    pub fn k2_replace_area_in_octree_data(
        &mut self,
        object: UPtr<crate::bindings::core_u_object::UObject>,
        old_area: TSubclassOf<UNavArea>,
        new_area: TSubclassOf<UNavArea>,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<25>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::navigation_system::__FUNCTION_PTRS
                    .u_navigation_system_v1_k2_replace_area_in_octree_data,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &object,
                __buffer.add(0).cast::<UPtr<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &old_area,
                __buffer.add(8).cast::<TSubclassOf<UNavArea>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &new_area,
                __buffer.add(16).cast::<TSubclassOf<UNavArea>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::navigation_system::__FUNCTION_PTRS
                    .u_navigation_system_v1_k2_replace_area_in_octree_data,
                __buffer,
            )
        };
        std::mem::forget(object);
        std::mem::forget(old_area);
        std::mem::forget(new_area);
        unsafe { __buffer.add(24).cast::<bool>().read() }
    }
    pub fn project_point_to_navigation(
        world_context_object: UPtr<crate::bindings::core_u_object::UObject>,
        point: &crate::bindings::core_u_object::FVector,
        projected_location: &mut crate::bindings::core_u_object::FVector,
        nav_data: UPtr<ANavigationData>,
        filter_class: TSubclassOf<UNavigationQueryFilter>,
        query_extent: crate::bindings::core_u_object::FVector,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<97>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::navigation_system::__FUNCTION_PTRS
                    .u_navigation_system_v1_k2_project_point_to_navigation,
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
                point,
                __buffer.add(8).cast::<crate::bindings::core_u_object::FVector>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                projected_location,
                __buffer.add(32).cast::<crate::bindings::core_u_object::FVector>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &nav_data,
                __buffer.add(56).cast::<UPtr<ANavigationData>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &filter_class,
                __buffer.add(64).cast::<TSubclassOf<UNavigationQueryFilter>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &query_extent,
                __buffer.add(72).cast::<crate::bindings::core_u_object::FVector>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::navigation_system::UNavigationSystemV1::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::navigation_system::__FUNCTION_PTRS
                    .u_navigation_system_v1_k2_project_point_to_navigation,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(32)
                .cast::<crate::bindings::core_u_object::FVector>()
                .swap(projected_location);
        }
        std::mem::forget(world_context_object);
        std::mem::forget(nav_data);
        std::mem::forget(filter_class);
        std::mem::forget(query_extent);
        unsafe { __buffer.add(96).cast::<bool>().read() }
    }
    pub fn get_random_reachable_point_in_radius(
        world_context_object: UPtr<crate::bindings::core_u_object::UObject>,
        origin: &crate::bindings::core_u_object::FVector,
        random_location: &mut crate::bindings::core_u_object::FVector,
        radius: f32,
        nav_data: UPtr<ANavigationData>,
        filter_class: TSubclassOf<UNavigationQueryFilter>,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<81>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::navigation_system::__FUNCTION_PTRS
                    .u_navigation_system_v1_k2_get_random_reachable_point_in_radius,
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
                origin,
                __buffer.add(8).cast::<crate::bindings::core_u_object::FVector>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                random_location,
                __buffer.add(32).cast::<crate::bindings::core_u_object::FVector>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&radius, __buffer.add(56).cast::<f32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &nav_data,
                __buffer.add(64).cast::<UPtr<ANavigationData>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &filter_class,
                __buffer.add(72).cast::<TSubclassOf<UNavigationQueryFilter>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::navigation_system::UNavigationSystemV1::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::navigation_system::__FUNCTION_PTRS
                    .u_navigation_system_v1_k2_get_random_reachable_point_in_radius,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(32)
                .cast::<crate::bindings::core_u_object::FVector>()
                .swap(random_location);
        }
        std::mem::forget(world_context_object);
        std::mem::forget(radius);
        std::mem::forget(nav_data);
        std::mem::forget(filter_class);
        unsafe { __buffer.add(80).cast::<bool>().read() }
    }
    pub fn get_random_point_in_navigable_radius(
        world_context_object: UPtr<crate::bindings::core_u_object::UObject>,
        origin: &crate::bindings::core_u_object::FVector,
        random_location: &mut crate::bindings::core_u_object::FVector,
        radius: f32,
        nav_data: UPtr<ANavigationData>,
        filter_class: TSubclassOf<UNavigationQueryFilter>,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<81>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::navigation_system::__FUNCTION_PTRS
                    .u_navigation_system_v1_k2_get_random_point_in_navigable_radius,
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
                origin,
                __buffer.add(8).cast::<crate::bindings::core_u_object::FVector>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                random_location,
                __buffer.add(32).cast::<crate::bindings::core_u_object::FVector>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&radius, __buffer.add(56).cast::<f32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &nav_data,
                __buffer.add(64).cast::<UPtr<ANavigationData>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &filter_class,
                __buffer.add(72).cast::<TSubclassOf<UNavigationQueryFilter>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::navigation_system::UNavigationSystemV1::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::navigation_system::__FUNCTION_PTRS
                    .u_navigation_system_v1_k2_get_random_point_in_navigable_radius,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(32)
                .cast::<crate::bindings::core_u_object::FVector>()
                .swap(random_location);
        }
        std::mem::forget(world_context_object);
        std::mem::forget(radius);
        std::mem::forget(nav_data);
        std::mem::forget(filter_class);
        unsafe { __buffer.add(80).cast::<bool>().read() }
    }
    pub fn get_random_location_in_navigable_radius(
        world_context_object: UPtr<crate::bindings::core_u_object::UObject>,
        origin: &crate::bindings::core_u_object::FVector,
        random_location: &mut crate::bindings::core_u_object::FVector,
        radius: f32,
        nav_data: UPtr<ANavigationData>,
        filter_class: TSubclassOf<UNavigationQueryFilter>,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<81>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::navigation_system::__FUNCTION_PTRS
                    .u_navigation_system_v1_k2_get_random_location_in_navigable_radius,
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
                origin,
                __buffer.add(8).cast::<crate::bindings::core_u_object::FVector>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                random_location,
                __buffer.add(32).cast::<crate::bindings::core_u_object::FVector>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&radius, __buffer.add(56).cast::<f32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &nav_data,
                __buffer.add(64).cast::<UPtr<ANavigationData>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &filter_class,
                __buffer.add(72).cast::<TSubclassOf<UNavigationQueryFilter>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::navigation_system::UNavigationSystemV1::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::navigation_system::__FUNCTION_PTRS
                    .u_navigation_system_v1_k2_get_random_location_in_navigable_radius,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(32)
                .cast::<crate::bindings::core_u_object::FVector>()
                .swap(random_location);
        }
        std::mem::forget(world_context_object);
        std::mem::forget(radius);
        std::mem::forget(nav_data);
        std::mem::forget(filter_class);
        unsafe { __buffer.add(80).cast::<bool>().read() }
    }
    pub fn is_navigation_being_built_or_locked(
        world_context_object: UPtr<crate::bindings::core_u_object::UObject>,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<9>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::navigation_system::__FUNCTION_PTRS
                    .u_navigation_system_v1_is_navigation_being_built_or_locked,
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
        let __object_ptr = crate::bindings::navigation_system::UNavigationSystemV1::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::navigation_system::__FUNCTION_PTRS
                    .u_navigation_system_v1_is_navigation_being_built_or_locked,
                __buffer,
            )
        };
        std::mem::forget(world_context_object);
        unsafe { __buffer.add(8).cast::<bool>().read() }
    }
    pub fn is_navigation_being_built(
        world_context_object: UPtr<crate::bindings::core_u_object::UObject>,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<9>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::navigation_system::__FUNCTION_PTRS
                    .u_navigation_system_v1_is_navigation_being_built,
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
        let __object_ptr = crate::bindings::navigation_system::UNavigationSystemV1::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::navigation_system::__FUNCTION_PTRS
                    .u_navigation_system_v1_is_navigation_being_built,
                __buffer,
            )
        };
        std::mem::forget(world_context_object);
        unsafe { __buffer.add(8).cast::<bool>().read() }
    }
    pub fn get_path_length(
        world_context_object: UPtr<crate::bindings::core_u_object::UObject>,
        path_start: &crate::bindings::core_u_object::FVector,
        path_end: &crate::bindings::core_u_object::FVector,
        path_length: &mut f64,
        nav_data: UPtr<ANavigationData>,
        filter_class: TSubclassOf<UNavigationQueryFilter>,
    ) -> crate::bindings::engine::ENavigationQueryResult {
        let mut __stack = crate::core_data::StackAlloc::<81>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::navigation_system::__FUNCTION_PTRS
                    .u_navigation_system_v1_get_path_length,
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
                path_start,
                __buffer.add(8).cast::<crate::bindings::core_u_object::FVector>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                path_end,
                __buffer.add(32).cast::<crate::bindings::core_u_object::FVector>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                path_length,
                __buffer.add(56).cast::<f64>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &nav_data,
                __buffer.add(64).cast::<UPtr<ANavigationData>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &filter_class,
                __buffer.add(72).cast::<TSubclassOf<UNavigationQueryFilter>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::navigation_system::UNavigationSystemV1::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::navigation_system::__FUNCTION_PTRS
                    .u_navigation_system_v1_get_path_length,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(56).cast::<f64>().swap(path_length);
        }
        std::mem::forget(world_context_object);
        std::mem::forget(nav_data);
        std::mem::forget(filter_class);
        unsafe {
            __buffer
                .add(80)
                .cast::<crate::bindings::engine::ENavigationQueryResult>()
                .read()
        }
    }
    pub fn get_path_cost(
        world_context_object: UPtr<crate::bindings::core_u_object::UObject>,
        path_start: &crate::bindings::core_u_object::FVector,
        path_end: &crate::bindings::core_u_object::FVector,
        path_cost: &mut f64,
        nav_data: UPtr<ANavigationData>,
        filter_class: TSubclassOf<UNavigationQueryFilter>,
    ) -> crate::bindings::engine::ENavigationQueryResult {
        let mut __stack = crate::core_data::StackAlloc::<81>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::navigation_system::__FUNCTION_PTRS
                    .u_navigation_system_v1_get_path_cost,
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
                path_start,
                __buffer.add(8).cast::<crate::bindings::core_u_object::FVector>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                path_end,
                __buffer.add(32).cast::<crate::bindings::core_u_object::FVector>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(path_cost, __buffer.add(56).cast::<f64>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &nav_data,
                __buffer.add(64).cast::<UPtr<ANavigationData>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &filter_class,
                __buffer.add(72).cast::<TSubclassOf<UNavigationQueryFilter>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::navigation_system::UNavigationSystemV1::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::navigation_system::__FUNCTION_PTRS
                    .u_navigation_system_v1_get_path_cost,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(56).cast::<f64>().swap(path_cost);
        }
        std::mem::forget(world_context_object);
        std::mem::forget(nav_data);
        std::mem::forget(filter_class);
        unsafe {
            __buffer
                .add(80)
                .cast::<crate::bindings::engine::ENavigationQueryResult>()
                .read()
        }
    }
    pub fn get_navigation_system(
        world_context_object: UPtr<crate::bindings::core_u_object::UObject>,
    ) -> UPtr<UNavigationSystemV1> {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::navigation_system::__FUNCTION_PTRS
                    .u_navigation_system_v1_get_navigation_system,
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
        let __object_ptr = crate::bindings::navigation_system::UNavigationSystemV1::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::navigation_system::__FUNCTION_PTRS
                    .u_navigation_system_v1_get_navigation_system,
                __buffer,
            )
        };
        std::mem::forget(world_context_object);
        unsafe { __buffer.add(8).cast::<UPtr<UNavigationSystemV1>>().read() }
    }
    pub fn find_path_to_location_synchronously(
        world_context_object: UPtr<crate::bindings::core_u_object::UObject>,
        path_start: &crate::bindings::core_u_object::FVector,
        path_end: &crate::bindings::core_u_object::FVector,
        pathfinding_context: UPtr<crate::bindings::engine::AActor>,
        filter_class: TSubclassOf<UNavigationQueryFilter>,
    ) -> UPtr<UNavigationPath> {
        let mut __stack = crate::core_data::StackAlloc::<80>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::navigation_system::__FUNCTION_PTRS
                    .u_navigation_system_v1_find_path_to_location_synchronously,
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
                path_start,
                __buffer.add(8).cast::<crate::bindings::core_u_object::FVector>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                path_end,
                __buffer.add(32).cast::<crate::bindings::core_u_object::FVector>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &pathfinding_context,
                __buffer.add(56).cast::<UPtr<crate::bindings::engine::AActor>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &filter_class,
                __buffer.add(64).cast::<TSubclassOf<UNavigationQueryFilter>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::navigation_system::UNavigationSystemV1::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::navigation_system::__FUNCTION_PTRS
                    .u_navigation_system_v1_find_path_to_location_synchronously,
                __buffer,
            )
        };
        std::mem::forget(world_context_object);
        std::mem::forget(pathfinding_context);
        std::mem::forget(filter_class);
        unsafe { __buffer.add(72).cast::<UPtr<UNavigationPath>>().read() }
    }
    pub fn find_path_to_actor_synchronously(
        world_context_object: UPtr<crate::bindings::core_u_object::UObject>,
        path_start: &crate::bindings::core_u_object::FVector,
        goal_actor: UPtr<crate::bindings::engine::AActor>,
        tether_distance: f32,
        pathfinding_context: UPtr<crate::bindings::engine::AActor>,
        filter_class: TSubclassOf<UNavigationQueryFilter>,
    ) -> UPtr<UNavigationPath> {
        let mut __stack = crate::core_data::StackAlloc::<72>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::navigation_system::__FUNCTION_PTRS
                    .u_navigation_system_v1_find_path_to_actor_synchronously,
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
                path_start,
                __buffer.add(8).cast::<crate::bindings::core_u_object::FVector>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &goal_actor,
                __buffer.add(32).cast::<UPtr<crate::bindings::engine::AActor>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &tether_distance,
                __buffer.add(40).cast::<f32>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &pathfinding_context,
                __buffer.add(48).cast::<UPtr<crate::bindings::engine::AActor>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &filter_class,
                __buffer.add(56).cast::<TSubclassOf<UNavigationQueryFilter>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::navigation_system::UNavigationSystemV1::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::navigation_system::__FUNCTION_PTRS
                    .u_navigation_system_v1_find_path_to_actor_synchronously,
                __buffer,
            )
        };
        std::mem::forget(world_context_object);
        std::mem::forget(goal_actor);
        std::mem::forget(tether_distance);
        std::mem::forget(pathfinding_context);
        std::mem::forget(filter_class);
        unsafe { __buffer.add(64).cast::<UPtr<UNavigationPath>>().read() }
    }
}
#[repr(C, align(8))]
pub struct UNavigationSystemModuleConfig {
    __padding_end: [u8; 120],
}
impl UNavigationSystemModuleConfig {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNavigationSystemModuleConfig")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNavigationSystemModuleConfig")
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
pub struct ANavigationTestingActor {
    #[doc(hidden)]
    pub(crate) __padding_1280: [u8; 1280],
    pub projected_location: crate::bindings::core_u_object::FVector,
    pub flags_1304: u8,
    #[doc(hidden)]
    pub(crate) __padding_1360: [u8; 55],
    pub flags_1360: u8,
    pub pathfinding_time: f32,
    pub path_cost: f64,
    pub pathfinding_steps: i32,
    __padding_end: [u8; 148],
}
impl ANavigationTestingActor {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("ANavigationTestingActor")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("ANavigationTestingActor")
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
pub struct UNavLinkComponent {
    #[doc(hidden)]
    pub(crate) __padding_1512: [u8; 1512],
    pub links: TArray<crate::bindings::engine::FNavigationLink>,
}
impl UNavLinkComponent {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNavLinkComponent")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNavLinkComponent")
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
pub struct UNavRelevantComponent {
    __padding_end: [u8; 320],
}
impl UNavRelevantComponent {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNavRelevantComponent")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNavRelevantComponent")
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
    pub fn set_navigation_relevancy(&mut self, b_relevant: bool) {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::navigation_system::__FUNCTION_PTRS
                    .u_nav_relevant_component_set_navigation_relevancy,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_relevant,
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
                crate::bindings::navigation_system::__FUNCTION_PTRS
                    .u_nav_relevant_component_set_navigation_relevancy,
                __buffer,
            )
        };
        std::mem::forget(b_relevant);
    }
}
#[repr(C, align(8))]
pub struct UNavLinkCustomComponent {
    __padding_end: [u8; 592],
}
impl UNavLinkCustomComponent {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNavLinkCustomComponent")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNavLinkCustomComponent")
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
pub struct UNavLinkRenderingComponent {
    __padding_end: [u8; 1504],
}
impl UNavLinkRenderingComponent {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNavLinkRenderingComponent")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNavLinkRenderingComponent")
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
pub struct ANavMeshBoundsVolume {
    __padding_end: [u8; 1216],
}
impl ANavMeshBoundsVolume {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("ANavMeshBoundsVolume")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("ANavMeshBoundsVolume")
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
pub struct UNavMeshRenderingComponent {
    __padding_end: [u8; 1760],
}
impl UNavMeshRenderingComponent {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNavMeshRenderingComponent")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNavMeshRenderingComponent")
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
pub struct UNavTestRenderingComponent {
    __padding_end: [u8; 1840],
}
impl UNavTestRenderingComponent {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNavTestRenderingComponent")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNavTestRenderingComponent")
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
pub struct ARecastNavMesh {
    __padding_end: [u8; 2480],
}
impl ARecastNavMesh {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("ARecastNavMesh")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("ARecastNavMesh")
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
    pub fn k2_replace_area_in_tile_bounds(
        &mut self,
        bounds: crate::bindings::core_u_object::FBox,
        old_area: TSubclassOf<UNavArea>,
        new_area: TSubclassOf<UNavArea>,
        replace_links: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<74>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::navigation_system::__FUNCTION_PTRS
                    .a_recast_nav_mesh_k2_replace_area_in_tile_bounds,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &bounds,
                __buffer.add(0).cast::<crate::bindings::core_u_object::FBox>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &old_area,
                __buffer.add(56).cast::<TSubclassOf<UNavArea>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &new_area,
                __buffer.add(64).cast::<TSubclassOf<UNavArea>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &replace_links,
                __buffer.add(72).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::navigation_system::__FUNCTION_PTRS
                    .a_recast_nav_mesh_k2_replace_area_in_tile_bounds,
                __buffer,
            )
        };
        std::mem::forget(bounds);
        std::mem::forget(old_area);
        std::mem::forget(new_area);
        std::mem::forget(replace_links);
        unsafe { __buffer.add(73).cast::<bool>().read() }
    }
}
#[repr(C, align(8))]
pub struct URecastNavMeshDataChunk {
    __padding_end: [u8; 80],
}
impl URecastNavMeshDataChunk {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("URecastNavMeshDataChunk")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("URecastNavMeshDataChunk")
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
pub struct UNavModifierComponent {
    #[doc(hidden)]
    pub(crate) __padding_320: [u8; 320],
    pub area_class: TSubclassOf<UNavArea>,
    pub area_class_to_replace: TSubclassOf<UNavArea>,
    __padding_end: [u8; 176],
}
impl UNavModifierComponent {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNavModifierComponent")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNavModifierComponent")
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
    pub fn set_area_class_to_replace(
        &mut self,
        new_area_class_to_replace: TSubclassOf<UNavArea>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::navigation_system::__FUNCTION_PTRS
                    .u_nav_modifier_component_set_area_class_to_replace,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &new_area_class_to_replace,
                __buffer.add(0).cast::<TSubclassOf<UNavArea>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::navigation_system::__FUNCTION_PTRS
                    .u_nav_modifier_component_set_area_class_to_replace,
                __buffer,
            )
        };
        std::mem::forget(new_area_class_to_replace);
    }
    pub fn set_area_class(&mut self, new_area_class: TSubclassOf<UNavArea>) {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::navigation_system::__FUNCTION_PTRS
                    .u_nav_modifier_component_set_area_class,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &new_area_class,
                __buffer.add(0).cast::<TSubclassOf<UNavArea>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::navigation_system::__FUNCTION_PTRS
                    .u_nav_modifier_component_set_area_class,
                __buffer,
            )
        };
        std::mem::forget(new_area_class);
    }
}
#[repr(C, align(8))]
pub struct ANavModifierVolume {
    #[doc(hidden)]
    pub(crate) __padding_1216: [u8; 1216],
    pub area_class: TSubclassOf<UNavArea>,
    pub area_class_to_replace: TSubclassOf<UNavArea>,
    __padding_end: [u8; 24],
}
impl ANavModifierVolume {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("ANavModifierVolume")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("ANavModifierVolume")
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
    pub fn set_area_class_to_replace(
        &mut self,
        new_area_class_to_replace: TSubclassOf<UNavArea>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::navigation_system::__FUNCTION_PTRS
                    .a_nav_modifier_volume_set_area_class_to_replace,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &new_area_class_to_replace,
                __buffer.add(0).cast::<TSubclassOf<UNavArea>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::navigation_system::__FUNCTION_PTRS
                    .a_nav_modifier_volume_set_area_class_to_replace,
                __buffer,
            )
        };
        std::mem::forget(new_area_class_to_replace);
    }
    pub fn set_area_class(&mut self, new_area_class: TSubclassOf<UNavArea>) {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::navigation_system::__FUNCTION_PTRS
                    .a_nav_modifier_volume_set_area_class,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &new_area_class,
                __buffer.add(0).cast::<TSubclassOf<UNavArea>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::navigation_system::__FUNCTION_PTRS
                    .a_nav_modifier_volume_set_area_class,
                __buffer,
            )
        };
        std::mem::forget(new_area_class);
    }
}
#[repr(C, align(8))]
pub struct ANavSystemConfigOverride {
    #[doc(hidden)]
    pub(crate) __padding_1144: [u8; 1144],
    pub navigation_system_config: UPtr<crate::bindings::engine::UNavigationSystemConfig>,
    pub override_policy: ENavSystemOverridePolicy,
    pub flags_1153: u8,
}
impl ANavSystemConfigOverride {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("ANavSystemConfigOverride")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("ANavSystemConfigOverride")
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
pub struct USplineNavModifierComponent {
    __padding_end: [u8; 688],
}
impl USplineNavModifierComponent {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("USplineNavModifierComponent")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("USplineNavModifierComponent")
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
pub struct FNavigationPath_PathUpdatedNotifier {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FNavigationSystemV1_OnNavDataRegisteredEvent {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FNavigationSystemV1_OnNavigationGenerationFinishedDelegate {
    _opague: [u8; 24],
}
#[repr(transparent)]
pub struct EHeightFieldRenderMode(pub u8);
impl EHeightFieldRenderMode {
    pub const SOLID: EHeightFieldRenderMode = EHeightFieldRenderMode(0);
    pub const WALKABLE: EHeightFieldRenderMode = EHeightFieldRenderMode(1);
}
#[repr(transparent)]
pub struct ERuntimeGenerationType(pub u8);
impl ERuntimeGenerationType {
    pub const STATIC: ERuntimeGenerationType = ERuntimeGenerationType(0);
    pub const DYNAMIC_MODIFIERS_ONLY: ERuntimeGenerationType = ERuntimeGenerationType(1);
    pub const DYNAMIC: ERuntimeGenerationType = ERuntimeGenerationType(2);
    pub const LEGACY_GENERATION: ERuntimeGenerationType = ERuntimeGenerationType(3);
}
#[repr(transparent)]
pub struct ENavCostDisplay(pub u8);
impl ENavCostDisplay {
    pub const TOTAL_COST: ENavCostDisplay = ENavCostDisplay(0);
    pub const HEURISTIC_ONLY: ENavCostDisplay = ENavCostDisplay(1);
    pub const REAL_COST_ONLY: ENavCostDisplay = ENavCostDisplay(2);
}
#[repr(transparent)]
pub struct ENavigationLedgeSlopeFilterMode(pub u8);
impl ENavigationLedgeSlopeFilterMode {
    pub const RECAST: ENavigationLedgeSlopeFilterMode = ENavigationLedgeSlopeFilterMode(
        0,
    );
    pub const NONE: ENavigationLedgeSlopeFilterMode = ENavigationLedgeSlopeFilterMode(1);
    pub const USE_STEP_HEIGHT_FROM_AGENT_MAX_SLOPE: ENavigationLedgeSlopeFilterMode = ENavigationLedgeSlopeFilterMode(
        2,
    );
}
#[repr(transparent)]
pub struct ERecastPartitioning(pub u8);
impl ERecastPartitioning {
    pub const MONOTONE: ERecastPartitioning = ERecastPartitioning(0);
    pub const WATERSHED: ERecastPartitioning = ERecastPartitioning(1);
    pub const CHUNKY_MONOTONE: ERecastPartitioning = ERecastPartitioning(2);
}
#[repr(transparent)]
pub struct ENavSystemOverridePolicy(pub u8);
impl ENavSystemOverridePolicy {
    pub const OVERRIDE: ENavSystemOverridePolicy = ENavSystemOverridePolicy(0);
    pub const APPEND: ENavSystemOverridePolicy = ENavSystemOverridePolicy(1);
    pub const SKIP: ENavSystemOverridePolicy = ENavSystemOverridePolicy(2);
}
#[repr(transparent)]
pub struct ESubdivisionLOD(pub i32);
impl ESubdivisionLOD {
    pub const LOW: ESubdivisionLOD = ESubdivisionLOD(0);
    pub const MEDIUM: ESubdivisionLOD = ESubdivisionLOD(1);
    pub const HIGH: ESubdivisionLOD = ESubdivisionLOD(2);
    pub const ULTRA: ESubdivisionLOD = ESubdivisionLOD(3);
}
