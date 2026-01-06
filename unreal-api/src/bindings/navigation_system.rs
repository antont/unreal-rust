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
pub static mut U_NAVIGATION_PATH_IS_VALID: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_NAVIGATION_PATH_IS_STRING_PULLED: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_NAVIGATION_PATH_IS_PARTIAL: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_NAVIGATION_PATH_GET_PATH_LENGTH: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_NAVIGATION_PATH_GET_PATH_COST: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_NAVIGATION_PATH_GET_DEBUG_STRING: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_NAVIGATION_PATH_ENABLE_RECALCULATION_ON_INVALIDATION: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_NAVIGATION_PATH_ENABLE_DEBUG_DRAWING: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_NAVIGATION_SYSTEM_V1_UNREGISTER_NAVIGATION_INVOKER: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_NAVIGATION_SYSTEM_V1_SET_MAX_SIMULTANEOUS_TILE_GENERATION_JOBS_COUNT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_NAVIGATION_SYSTEM_V1_SET_GEOMETRY_GATHERING_MODE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_NAVIGATION_SYSTEM_V1_RESET_MAX_SIMULTANEOUS_TILE_GENERATION_JOBS_COUNT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_NAVIGATION_SYSTEM_V1_REGISTER_NAVIGATION_INVOKER: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_NAVIGATION_SYSTEM_V1_ON_NAVIGATION_BOUNDS_UPDATED: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_NAVIGATION_SYSTEM_V1_NAVIGATION_RAYCAST: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_NAVIGATION_SYSTEM_V1_K2_REPLACE_AREA_IN_OCTREE_DATA: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_NAVIGATION_SYSTEM_V1_K2_PROJECT_POINT_TO_NAVIGATION: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_NAVIGATION_SYSTEM_V1_K2_GET_RANDOM_REACHABLE_POINT_IN_RADIUS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_NAVIGATION_SYSTEM_V1_K2_GET_RANDOM_POINT_IN_NAVIGABLE_RADIUS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_NAVIGATION_SYSTEM_V1_K2_GET_RANDOM_LOCATION_IN_NAVIGABLE_RADIUS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_NAVIGATION_SYSTEM_V1_IS_NAVIGATION_BEING_BUILT_OR_LOCKED: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_NAVIGATION_SYSTEM_V1_IS_NAVIGATION_BEING_BUILT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_NAVIGATION_SYSTEM_V1_GET_PATH_LENGTH: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_NAVIGATION_SYSTEM_V1_GET_PATH_COST: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_NAVIGATION_SYSTEM_V1_GET_NAVIGATION_SYSTEM: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_NAVIGATION_SYSTEM_V1_FIND_PATH_TO_LOCATION_SYNCHRONOUSLY: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_NAVIGATION_SYSTEM_V1_FIND_PATH_TO_ACTOR_SYNCHRONOUSLY: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_NAV_RELEVANT_COMPONENT_SET_NAVIGATION_RELEVANCY: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut A_RECAST_NAV_MESH_K2_REPLACE_AREA_IN_TILE_BOUNDS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_NAV_MODIFIER_COMPONENT_SET_AREA_CLASS_TO_REPLACE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_NAV_MODIFIER_COMPONENT_SET_AREA_CLASS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut A_NAV_MODIFIER_VOLUME_SET_AREA_CLASS_TO_REPLACE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut A_NAV_MODIFIER_VOLUME_SET_AREA_CLASS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut A_NAV_SYSTEM_CONFIG_OVERRIDE_APPLY_CHANGES: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_SPLINE_NAV_MODIFIER_COMPONENT_UPDATE_NAVIGATION_WITH_COMPONENT_DATA: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
pub fn initialize() {
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UNavigationPath::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("IsValid"),
            &raw mut U_NAVIGATION_PATH_IS_VALID,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("IsStringPulled"),
            &raw mut U_NAVIGATION_PATH_IS_STRING_PULLED,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("IsPartial"),
            &raw mut U_NAVIGATION_PATH_IS_PARTIAL,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetPathLength"),
            &raw mut U_NAVIGATION_PATH_GET_PATH_LENGTH,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetPathCost"),
            &raw mut U_NAVIGATION_PATH_GET_PATH_COST,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetDebugString"),
            &raw mut U_NAVIGATION_PATH_GET_DEBUG_STRING,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("EnableRecalculationOnInvalidation"),
            &raw mut U_NAVIGATION_PATH_ENABLE_RECALCULATION_ON_INVALIDATION,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("EnableDebugDrawing"),
            &raw mut U_NAVIGATION_PATH_ENABLE_DEBUG_DRAWING,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UNavigationSystemV1::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("UnregisterNavigationInvoker"),
            &raw mut U_NAVIGATION_SYSTEM_V1_UNREGISTER_NAVIGATION_INVOKER,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetMaxSimultaneousTileGenerationJobsCount"),
            &raw mut U_NAVIGATION_SYSTEM_V1_SET_MAX_SIMULTANEOUS_TILE_GENERATION_JOBS_COUNT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetGeometryGatheringMode"),
            &raw mut U_NAVIGATION_SYSTEM_V1_SET_GEOMETRY_GATHERING_MODE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ResetMaxSimultaneousTileGenerationJobsCount"),
            &raw mut U_NAVIGATION_SYSTEM_V1_RESET_MAX_SIMULTANEOUS_TILE_GENERATION_JOBS_COUNT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RegisterNavigationInvoker"),
            &raw mut U_NAVIGATION_SYSTEM_V1_REGISTER_NAVIGATION_INVOKER,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("OnNavigationBoundsUpdated"),
            &raw mut U_NAVIGATION_SYSTEM_V1_ON_NAVIGATION_BOUNDS_UPDATED,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("NavigationRaycast"),
            &raw mut U_NAVIGATION_SYSTEM_V1_NAVIGATION_RAYCAST,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("K2_ReplaceAreaInOctreeData"),
            &raw mut U_NAVIGATION_SYSTEM_V1_K2_REPLACE_AREA_IN_OCTREE_DATA,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("K2_ProjectPointToNavigation"),
            &raw mut U_NAVIGATION_SYSTEM_V1_K2_PROJECT_POINT_TO_NAVIGATION,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("K2_GetRandomReachablePointInRadius"),
            &raw mut U_NAVIGATION_SYSTEM_V1_K2_GET_RANDOM_REACHABLE_POINT_IN_RADIUS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("K2_GetRandomPointInNavigableRadius"),
            &raw mut U_NAVIGATION_SYSTEM_V1_K2_GET_RANDOM_POINT_IN_NAVIGABLE_RADIUS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("K2_GetRandomLocationInNavigableRadius"),
            &raw mut U_NAVIGATION_SYSTEM_V1_K2_GET_RANDOM_LOCATION_IN_NAVIGABLE_RADIUS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("IsNavigationBeingBuiltOrLocked"),
            &raw mut U_NAVIGATION_SYSTEM_V1_IS_NAVIGATION_BEING_BUILT_OR_LOCKED,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("IsNavigationBeingBuilt"),
            &raw mut U_NAVIGATION_SYSTEM_V1_IS_NAVIGATION_BEING_BUILT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetPathLength"),
            &raw mut U_NAVIGATION_SYSTEM_V1_GET_PATH_LENGTH,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetPathCost"),
            &raw mut U_NAVIGATION_SYSTEM_V1_GET_PATH_COST,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetNavigationSystem"),
            &raw mut U_NAVIGATION_SYSTEM_V1_GET_NAVIGATION_SYSTEM,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("FindPathToLocationSynchronously"),
            &raw mut U_NAVIGATION_SYSTEM_V1_FIND_PATH_TO_LOCATION_SYNCHRONOUSLY,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("FindPathToActorSynchronously"),
            &raw mut U_NAVIGATION_SYSTEM_V1_FIND_PATH_TO_ACTOR_SYNCHRONOUSLY,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UNavRelevantComponent::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetNavigationRelevancy"),
            &raw mut U_NAV_RELEVANT_COMPONENT_SET_NAVIGATION_RELEVANCY,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = ARecastNavMesh::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("K2_ReplaceAreaInTileBounds"),
            &raw mut A_RECAST_NAV_MESH_K2_REPLACE_AREA_IN_TILE_BOUNDS,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UNavModifierComponent::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetAreaClassToReplace"),
            &raw mut U_NAV_MODIFIER_COMPONENT_SET_AREA_CLASS_TO_REPLACE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetAreaClass"),
            &raw mut U_NAV_MODIFIER_COMPONENT_SET_AREA_CLASS,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = ANavModifierVolume::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetAreaClassToReplace"),
            &raw mut A_NAV_MODIFIER_VOLUME_SET_AREA_CLASS_TO_REPLACE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetAreaClass"),
            &raw mut A_NAV_MODIFIER_VOLUME_SET_AREA_CLASS,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = ANavSystemConfigOverride::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ApplyChanges"),
            &raw mut A_NAV_SYSTEM_CONFIG_OVERRIDE_APPLY_CHANGES,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = USplineNavModifierComponent::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("UpdateNavigationWithComponentData"),
            &raw mut U_SPLINE_NAV_MODIFIER_COMPONENT_UPDATE_NAVIGATION_WITH_COMPONENT_DATA,
        );
    }
}
#[repr(C, align(8))]
pub struct UCrowdManagerBase {
    __padding_end: [u8; 48],
}
impl UCrowdManagerBase {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UCrowdManagerBase")
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
pub struct UBaseGeneratedNavLinksProxy {
    __padding_end: [u8; 72],
}
impl UBaseGeneratedNavLinksProxy {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBaseGeneratedNavLinksProxy")
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
pub struct UNavigationQueryFilter {
    __padding_end: [u8; 80],
}
impl UNavigationQueryFilter {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNavigationQueryFilter")
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
pub struct ANavigationGraphNode {
    __padding_end: [u8; 1136],
}
impl ANavigationGraphNode {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("ANavigationGraphNode")
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
pub struct UNavigationGraphNodeComponent {
    __padding_end: [u8; 704],
}
impl UNavigationGraphNodeComponent {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNavigationGraphNodeComponent")
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
pub struct INavigationPathGenerator {}
#[repr(C, align(8))]
pub struct UNavigationPathGenerator {
    __padding_end: [u8; 48],
}
impl UNavigationPathGenerator {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNavigationPathGenerator")
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
pub struct INavLinkCustomInterface {}
#[repr(C, align(8))]
pub struct UNavLinkCustomInterface {
    __padding_end: [u8; 48],
}
impl UNavLinkCustomInterface {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNavLinkCustomInterface")
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
pub struct INavLinkHostInterface {}
#[repr(C, align(8))]
pub struct UNavLinkHostInterface {
    __padding_end: [u8; 48],
}
impl UNavLinkHostInterface {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNavLinkHostInterface")
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
pub struct UNavLinkTrivial {
    __padding_end: [u8; 88],
}
impl UNavLinkTrivial {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNavLinkTrivial")
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
pub struct INavNodeInterface {}
#[repr(C, align(8))]
pub struct UNavNodeInterface {
    __padding_end: [u8; 48],
}
impl UNavNodeInterface {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNavNodeInterface")
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
pub struct ANavigationData {
    __padding_end: [u8; 1760],
}
impl ANavigationData {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("ANavigationData")
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
pub struct AAbstractNavData {
    __padding_end: [u8; 1760],
}
impl AAbstractNavData {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("AAbstractNavData")
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
pub struct UNavArea {
    __padding_end: [u8; 80],
}
impl UNavArea {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS.wait().name_to_ptr.get("UNavArea").unwrap()
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
pub struct UNavAreaMeta {
    __padding_end: [u8; 80],
}
impl UNavAreaMeta {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNavAreaMeta")
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
pub struct UNavAreaMeta_SwitchByAgent {
    __padding_end: [u8; 208],
}
impl UNavAreaMeta_SwitchByAgent {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNavAreaMeta_SwitchByAgent")
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
pub struct UNavArea_Default {
    __padding_end: [u8; 80],
}
impl UNavArea_Default {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNavArea_Default")
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
pub struct UNavArea_LowHeight {
    __padding_end: [u8; 80],
}
impl UNavArea_LowHeight {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNavArea_LowHeight")
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
pub struct UNavArea_Null {
    __padding_end: [u8; 80],
}
impl UNavArea_Null {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNavArea_Null")
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
pub struct UNavArea_Obstacle {
    __padding_end: [u8; 80],
}
impl UNavArea_Obstacle {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNavArea_Obstacle")
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
pub struct UNavCollision {
    __padding_end: [u8; 312],
}
impl UNavCollision {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNavCollision")
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
pub struct URecastFilter_UseDefaultArea {
    __padding_end: [u8; 80],
}
impl URecastFilter_UseDefaultArea {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("URecastFilter_UseDefaultArea")
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
pub struct ANavigationGraph {
    __padding_end: [u8; 1760],
}
impl ANavigationGraph {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("ANavigationGraph")
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
pub struct UNavigationInvokerComponent {
    #[doc(hidden)]
    __padding_240: [u8; 240],
    pub tile_generation_radius: f32,
    pub tile_removal_radius: f32,
    __padding_end: [u8; 8],
}
impl UNavigationInvokerComponent {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNavigationInvokerComponent")
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
pub struct UNavigationObjectRepository {
    __padding_end: [u8; 392],
}
impl UNavigationObjectRepository {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNavigationObjectRepository")
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
pub struct UNavigationPath {
    #[doc(hidden)]
    __padding_72: [u8; 72],
    pub path_points: TArray<crate::bindings::core_u_object::FVector>,
    pub recalculate_on_invalidation: crate::bindings::engine::ENavigationOptionFlag,
    __padding_end: [u8; 71],
}
impl UNavigationPath {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNavigationPath")
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
pub struct UNavigationSystemV1 {
    #[doc(hidden)]
    __padding_64: [u8; 64],
    pub default_agent_name: FName,
    pub crowd_manager_class: TSoftObjectPtr<crate::bindings::core_u_object::UClass>,
    __padding_end: [u8; 5712],
}
impl UNavigationSystemV1 {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNavigationSystemV1")
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
pub struct UNavigationSystemModuleConfig {
    __padding_end: [u8; 120],
}
impl UNavigationSystemModuleConfig {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNavigationSystemModuleConfig")
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
pub struct ANavigationTestingActor {
    #[doc(hidden)]
    __padding_1280: [u8; 1280],
    pub projected_location: crate::bindings::core_u_object::FVector,
    pub flags_1304: u8,
    #[doc(hidden)]
    __padding_1360: [u8; 55],
    pub flags_1360: u8,
    pub pathfinding_time: f32,
    pub path_cost: f64,
    pub pathfinding_steps: i32,
    __padding_end: [u8; 148],
}
impl ANavigationTestingActor {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("ANavigationTestingActor")
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
pub struct UNavLinkComponent {
    #[doc(hidden)]
    __padding_1512: [u8; 1512],
    pub links: TArray<crate::bindings::engine::FNavigationLink>,
    __padding_end: [u8; 8],
}
impl UNavLinkComponent {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNavLinkComponent")
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
pub struct UNavRelevantComponent {
    __padding_end: [u8; 320],
}
impl UNavRelevantComponent {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNavRelevantComponent")
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
pub struct UNavLinkCustomComponent {
    __padding_end: [u8; 592],
}
impl UNavLinkCustomComponent {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNavLinkCustomComponent")
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
pub struct UNavLinkRenderingComponent {
    __padding_end: [u8; 1504],
}
impl UNavLinkRenderingComponent {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNavLinkRenderingComponent")
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
pub struct ANavMeshBoundsVolume {
    __padding_end: [u8; 1216],
}
impl ANavMeshBoundsVolume {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("ANavMeshBoundsVolume")
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
pub struct UNavMeshRenderingComponent {
    __padding_end: [u8; 1760],
}
impl UNavMeshRenderingComponent {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNavMeshRenderingComponent")
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
pub struct UNavTestRenderingComponent {
    __padding_end: [u8; 1840],
}
impl UNavTestRenderingComponent {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNavTestRenderingComponent")
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
pub struct ARecastNavMesh {
    __padding_end: [u8; 2480],
}
impl ARecastNavMesh {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("ARecastNavMesh")
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
pub struct URecastNavMeshDataChunk {
    __padding_end: [u8; 80],
}
impl URecastNavMeshDataChunk {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("URecastNavMeshDataChunk")
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
pub struct UNavModifierComponent {
    #[doc(hidden)]
    __padding_320: [u8; 320],
    pub area_class: TSubclassOf<UNavArea>,
    pub area_class_to_replace: TSubclassOf<UNavArea>,
    __padding_end: [u8; 176],
}
impl UNavModifierComponent {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNavModifierComponent")
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
pub struct ANavModifierVolume {
    #[doc(hidden)]
    __padding_1216: [u8; 1216],
    pub area_class: TSubclassOf<UNavArea>,
    pub area_class_to_replace: TSubclassOf<UNavArea>,
    __padding_end: [u8; 24],
}
impl ANavModifierVolume {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("ANavModifierVolume")
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
pub struct ANavSystemConfigOverride {
    #[doc(hidden)]
    __padding_1144: [u8; 1144],
    pub navigation_system_config: UPtr<crate::bindings::engine::UNavigationSystemConfig>,
    pub override_policy: ENavSystemOverridePolicy,
    pub flags_1153: u8,
    __padding_end: [u8; 6],
}
impl ANavSystemConfigOverride {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("ANavSystemConfigOverride")
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
pub struct USplineNavModifierComponent {
    __padding_end: [u8; 688],
}
impl USplineNavModifierComponent {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("USplineNavModifierComponent")
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
