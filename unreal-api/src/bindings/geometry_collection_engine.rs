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
pub static mut U_CHAOS_DESTRUCTION_LISTENER_SORT_TRAILING_EVENTS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_CHAOS_DESTRUCTION_LISTENER_SORT_REMOVAL_EVENTS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_CHAOS_DESTRUCTION_LISTENER_SORT_COLLISION_EVENTS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_CHAOS_DESTRUCTION_LISTENER_SORT_BREAKING_EVENTS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_CHAOS_DESTRUCTION_LISTENER_SET_TRAILING_EVENT_REQUEST_SETTINGS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_CHAOS_DESTRUCTION_LISTENER_SET_TRAILING_EVENT_ENABLED: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_CHAOS_DESTRUCTION_LISTENER_SET_REMOVAL_EVENT_REQUEST_SETTINGS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_CHAOS_DESTRUCTION_LISTENER_SET_REMOVAL_EVENT_ENABLED: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_CHAOS_DESTRUCTION_LISTENER_SET_COLLISION_EVENT_REQUEST_SETTINGS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_CHAOS_DESTRUCTION_LISTENER_SET_COLLISION_EVENT_ENABLED: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_CHAOS_DESTRUCTION_LISTENER_SET_BREAKING_EVENT_REQUEST_SETTINGS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_CHAOS_DESTRUCTION_LISTENER_SET_BREAKING_EVENT_ENABLED: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_CHAOS_DESTRUCTION_LISTENER_REMOVE_GEOMETRY_COLLECTION_ACTOR: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_CHAOS_DESTRUCTION_LISTENER_REMOVE_CHAOS_SOLVER_ACTOR: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_CHAOS_DESTRUCTION_LISTENER_IS_EVENT_LISTENING: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_CHAOS_DESTRUCTION_LISTENER_ADD_GEOMETRY_COLLECTION_ACTOR: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_CHAOS_DESTRUCTION_LISTENER_ADD_CHAOS_SOLVER_ACTOR: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut A_GEOMETRY_COLLECTION_ACTOR_RAYCAST_SINGLE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_GEOMETRY_COLLECTION_BLUEPRINT_LIBRARY_SET_ISM_POOL_CUSTOM_INSTANCE_DATA: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_GEOMETRY_COLLECTION_BLUEPRINT_LIBRARY_SET_CUSTOM_INSTANCE_DATA_BY_NAME: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_GEOMETRY_COLLECTION_BLUEPRINT_LIBRARY_SET_CUSTOM_INSTANCE_DATA_BY_INDEX: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_GEOMETRY_COLLECTION_COMPONENT_SET_USE_STATIC_MESH_COLLISION_FOR_TRACES: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_GEOMETRY_COLLECTION_COMPONENT_SET_USE_MATERIAL_DAMAGE_MODIFIERS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_GEOMETRY_COLLECTION_COMPONENT_SET_SOLVER_ACTOR: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_GEOMETRY_COLLECTION_COMPONENT_SET_ROOT_PROXY_COMPONENT_SPACE_TRANSFORM: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_GEOMETRY_COLLECTION_COMPONENT_SET_REST_COLLECTION: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_GEOMETRY_COLLECTION_COMPONENT_SET_PER_PARTICLE_COLLISION_PROFILE_NAME: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_GEOMETRY_COLLECTION_COMPONENT_SET_PER_LEVEL_COLLISION_PROFILE_NAMES: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_GEOMETRY_COLLECTION_COMPONENT_SET_ONE_WAY_INTERACTION_LEVEL: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_GEOMETRY_COLLECTION_COMPONENT_SET_NOTIFY_REMOVALS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_GEOMETRY_COLLECTION_COMPONENT_SET_NOTIFY_GLOBAL_REMOVALS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_GEOMETRY_COLLECTION_COMPONENT_SET_NOTIFY_GLOBAL_CRUMBLINGS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_GEOMETRY_COLLECTION_COMPONENT_SET_NOTIFY_GLOBAL_COLLISION: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_GEOMETRY_COLLECTION_COMPONENT_SET_NOTIFY_GLOBAL_BREAKS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_GEOMETRY_COLLECTION_COMPONENT_SET_NOTIFY_CRUMBLINGS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_GEOMETRY_COLLECTION_COMPONENT_SET_NOTIFY_BREAKS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_GEOMETRY_COLLECTION_COMPONENT_SET_LOCAL_REST_TRANSFORMS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_GEOMETRY_COLLECTION_COMPONENT_SET_GRAVITY_GROUP_INDEX: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_GEOMETRY_COLLECTION_COMPONENT_SET_ENABLE_DAMAGE_FROM_COLLISION: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_GEOMETRY_COLLECTION_COMPONENT_SET_DENSITY_FROM_PHYSICS_MATERIAL: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_GEOMETRY_COLLECTION_COMPONENT_SET_DAMAGE_THRESHOLD: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_GEOMETRY_COLLECTION_COMPONENT_SET_DAMAGE_PROPAGATION_DATA: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_GEOMETRY_COLLECTION_COMPONENT_SET_DAMAGE_MODEL: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_GEOMETRY_COLLECTION_COMPONENT_SET_ANCHORED_BY_TRANSFORMED_BOX: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_GEOMETRY_COLLECTION_COMPONENT_SET_ANCHORED_BY_INDEX: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_GEOMETRY_COLLECTION_COMPONENT_SET_ANCHORED_BY_BOX: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_GEOMETRY_COLLECTION_COMPONENT_SET_ABANDONED_PARTICLE_COLLISION_PROFILE_NAME: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_GEOMETRY_COLLECTION_COMPONENT_RESET_STATE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_GEOMETRY_COLLECTION_COMPONENT_REMOVE_ALL_ANCHORS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_GEOMETRY_COLLECTION_COMPONENT_RECEIVE_PHYSICS_COLLISION: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_GEOMETRY_COLLECTION_COMPONENT_ON_REP_REP_STATE_DATA: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_GEOMETRY_COLLECTION_COMPONENT_ON_REP_REP_DYNAMIC_DATA: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_GEOMETRY_COLLECTION_COMPONENT_ON_REP_REP_DATA: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_GEOMETRY_COLLECTION_COMPONENT_NOTIFY_GEOMETRY_COLLECTION_PHYSICS_STATE_CHANGE_DELEGATE_SIGNATURE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_GEOMETRY_COLLECTION_COMPONENT_NOTIFY_GEOMETRY_COLLECTION_PHYSICS_LOADING_STATE_CHANGE_DELEGATE_SIGNATURE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_GEOMETRY_COLLECTION_COMPONENT_IS_ROOT_BROKEN: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_GEOMETRY_COLLECTION_COMPONENT_GET_USE_STATIC_MESH_COLLISION_FOR_TRACES: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_GEOMETRY_COLLECTION_COMPONENT_GET_SOLVER_ACTOR: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_GEOMETRY_COLLECTION_COMPONENT_GET_ROOT_INITIAL_TRANSFORM: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_GEOMETRY_COLLECTION_COMPONENT_GET_ROOT_INDEX: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_GEOMETRY_COLLECTION_COMPONENT_GET_ROOT_CURRENT_TRANSFORM: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_GEOMETRY_COLLECTION_COMPONENT_GET_MASS_AND_EXTENTS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_GEOMETRY_COLLECTION_COMPONENT_GET_LOCAL_REST_TRANSFORMS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_GEOMETRY_COLLECTION_COMPONENT_GET_LOCAL_BOUNDS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_GEOMETRY_COLLECTION_COMPONENT_GET_INITIAL_LOCAL_REST_TRANSFORMS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_GEOMETRY_COLLECTION_COMPONENT_GET_INITIAL_LEVEL: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_GEOMETRY_COLLECTION_COMPONENT_GET_DEBUG_INFO: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_GEOMETRY_COLLECTION_COMPONENT_GET_DAMAGE_THRESHOLD: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_GEOMETRY_COLLECTION_COMPONENT_FORCE_BROKEN_FOR_CUSTOM_RENDERER: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_GEOMETRY_COLLECTION_COMPONENT_FIND_LEAF_TRANSFORM_BY_LINE_TRACE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_GEOMETRY_COLLECTION_COMPONENT_ENABLE_ROOT_PROXY_FOR_CUSTOM_RENDERER: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_GEOMETRY_COLLECTION_COMPONENT_CRUMBLE_CLUSTER: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_GEOMETRY_COLLECTION_COMPONENT_CRUMBLE_ACTIVE_CLUSTERS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_GEOMETRY_COLLECTION_COMPONENT_APPLY_PHYSICS_FIELD: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_GEOMETRY_COLLECTION_COMPONENT_APPLY_LINEAR_VELOCITY: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_GEOMETRY_COLLECTION_COMPONENT_APPLY_KINEMATIC_FIELD: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_GEOMETRY_COLLECTION_COMPONENT_APPLY_INTERNAL_STRAIN: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_GEOMETRY_COLLECTION_COMPONENT_APPLY_EXTERNAL_STRAIN: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_GEOMETRY_COLLECTION_COMPONENT_APPLY_BREAKING_LINEAR_VELOCITY: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_GEOMETRY_COLLECTION_COMPONENT_APPLY_BREAKING_ANGULAR_VELOCITY: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_GEOMETRY_COLLECTION_COMPONENT_APPLY_ASSET_DEFAULTS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_GEOMETRY_COLLECTION_COMPONENT_APPLY_ANGULAR_VELOCITY: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_GEOMETRY_COLLECTION_ISM_POOL_SUB_SYSTEM_ON_ACTOR_END_PLAY: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_GEOMETRY_COLLECTION_SET_ENABLE_NANITE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_GEOMETRY_COLLECTION_SET_DATAFLOW_ASSET: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_GEOMETRY_COLLECTION_SET_CONVERT_VERTEX_COLORS_TO_SRGB: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_GEOMETRY_COLLECTION_GET_DATAFLOW_ASSET: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
pub fn initialize() {
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UChaosDestructionListener::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SortTrailingEvents"),
            &raw mut U_CHAOS_DESTRUCTION_LISTENER_SORT_TRAILING_EVENTS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SortRemovalEvents"),
            &raw mut U_CHAOS_DESTRUCTION_LISTENER_SORT_REMOVAL_EVENTS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SortCollisionEvents"),
            &raw mut U_CHAOS_DESTRUCTION_LISTENER_SORT_COLLISION_EVENTS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SortBreakingEvents"),
            &raw mut U_CHAOS_DESTRUCTION_LISTENER_SORT_BREAKING_EVENTS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetTrailingEventRequestSettings"),
            &raw mut U_CHAOS_DESTRUCTION_LISTENER_SET_TRAILING_EVENT_REQUEST_SETTINGS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetTrailingEventEnabled"),
            &raw mut U_CHAOS_DESTRUCTION_LISTENER_SET_TRAILING_EVENT_ENABLED,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetRemovalEventRequestSettings"),
            &raw mut U_CHAOS_DESTRUCTION_LISTENER_SET_REMOVAL_EVENT_REQUEST_SETTINGS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetRemovalEventEnabled"),
            &raw mut U_CHAOS_DESTRUCTION_LISTENER_SET_REMOVAL_EVENT_ENABLED,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetCollisionEventRequestSettings"),
            &raw mut U_CHAOS_DESTRUCTION_LISTENER_SET_COLLISION_EVENT_REQUEST_SETTINGS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetCollisionEventEnabled"),
            &raw mut U_CHAOS_DESTRUCTION_LISTENER_SET_COLLISION_EVENT_ENABLED,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetBreakingEventRequestSettings"),
            &raw mut U_CHAOS_DESTRUCTION_LISTENER_SET_BREAKING_EVENT_REQUEST_SETTINGS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetBreakingEventEnabled"),
            &raw mut U_CHAOS_DESTRUCTION_LISTENER_SET_BREAKING_EVENT_ENABLED,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RemoveGeometryCollectionActor"),
            &raw mut U_CHAOS_DESTRUCTION_LISTENER_REMOVE_GEOMETRY_COLLECTION_ACTOR,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RemoveChaosSolverActor"),
            &raw mut U_CHAOS_DESTRUCTION_LISTENER_REMOVE_CHAOS_SOLVER_ACTOR,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("IsEventListening"),
            &raw mut U_CHAOS_DESTRUCTION_LISTENER_IS_EVENT_LISTENING,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AddGeometryCollectionActor"),
            &raw mut U_CHAOS_DESTRUCTION_LISTENER_ADD_GEOMETRY_COLLECTION_ACTOR,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AddChaosSolverActor"),
            &raw mut U_CHAOS_DESTRUCTION_LISTENER_ADD_CHAOS_SOLVER_ACTOR,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = AGeometryCollectionActor::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RaycastSingle"),
            &raw mut A_GEOMETRY_COLLECTION_ACTOR_RAYCAST_SINGLE,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UGeometryCollectionBlueprintLibrary::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetISMPoolCustomInstanceData"),
            &raw mut U_GEOMETRY_COLLECTION_BLUEPRINT_LIBRARY_SET_ISM_POOL_CUSTOM_INSTANCE_DATA,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetCustomInstanceDataByName"),
            &raw mut U_GEOMETRY_COLLECTION_BLUEPRINT_LIBRARY_SET_CUSTOM_INSTANCE_DATA_BY_NAME,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetCustomInstanceDataByIndex"),
            &raw mut U_GEOMETRY_COLLECTION_BLUEPRINT_LIBRARY_SET_CUSTOM_INSTANCE_DATA_BY_INDEX,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UGeometryCollectionComponent::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetUseStaticMeshCollisionForTraces"),
            &raw mut U_GEOMETRY_COLLECTION_COMPONENT_SET_USE_STATIC_MESH_COLLISION_FOR_TRACES,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetUseMaterialDamageModifiers"),
            &raw mut U_GEOMETRY_COLLECTION_COMPONENT_SET_USE_MATERIAL_DAMAGE_MODIFIERS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetSolverActor"),
            &raw mut U_GEOMETRY_COLLECTION_COMPONENT_SET_SOLVER_ACTOR,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetRootProxyComponentSpaceTransform"),
            &raw mut U_GEOMETRY_COLLECTION_COMPONENT_SET_ROOT_PROXY_COMPONENT_SPACE_TRANSFORM,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetRestCollection"),
            &raw mut U_GEOMETRY_COLLECTION_COMPONENT_SET_REST_COLLECTION,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetPerParticleCollisionProfileName"),
            &raw mut U_GEOMETRY_COLLECTION_COMPONENT_SET_PER_PARTICLE_COLLISION_PROFILE_NAME,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetPerLevelCollisionProfileNames"),
            &raw mut U_GEOMETRY_COLLECTION_COMPONENT_SET_PER_LEVEL_COLLISION_PROFILE_NAMES,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetOneWayInteractionLevel"),
            &raw mut U_GEOMETRY_COLLECTION_COMPONENT_SET_ONE_WAY_INTERACTION_LEVEL,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetNotifyRemovals"),
            &raw mut U_GEOMETRY_COLLECTION_COMPONENT_SET_NOTIFY_REMOVALS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetNotifyGlobalRemovals"),
            &raw mut U_GEOMETRY_COLLECTION_COMPONENT_SET_NOTIFY_GLOBAL_REMOVALS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetNotifyGlobalCrumblings"),
            &raw mut U_GEOMETRY_COLLECTION_COMPONENT_SET_NOTIFY_GLOBAL_CRUMBLINGS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetNotifyGlobalCollision"),
            &raw mut U_GEOMETRY_COLLECTION_COMPONENT_SET_NOTIFY_GLOBAL_COLLISION,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetNotifyGlobalBreaks"),
            &raw mut U_GEOMETRY_COLLECTION_COMPONENT_SET_NOTIFY_GLOBAL_BREAKS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetNotifyCrumblings"),
            &raw mut U_GEOMETRY_COLLECTION_COMPONENT_SET_NOTIFY_CRUMBLINGS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetNotifyBreaks"),
            &raw mut U_GEOMETRY_COLLECTION_COMPONENT_SET_NOTIFY_BREAKS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetLocalRestTransforms"),
            &raw mut U_GEOMETRY_COLLECTION_COMPONENT_SET_LOCAL_REST_TRANSFORMS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetGravityGroupIndex"),
            &raw mut U_GEOMETRY_COLLECTION_COMPONENT_SET_GRAVITY_GROUP_INDEX,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetEnableDamageFromCollision"),
            &raw mut U_GEOMETRY_COLLECTION_COMPONENT_SET_ENABLE_DAMAGE_FROM_COLLISION,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetDensityFromPhysicsMaterial"),
            &raw mut U_GEOMETRY_COLLECTION_COMPONENT_SET_DENSITY_FROM_PHYSICS_MATERIAL,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetDamageThreshold"),
            &raw mut U_GEOMETRY_COLLECTION_COMPONENT_SET_DAMAGE_THRESHOLD,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetDamagePropagationData"),
            &raw mut U_GEOMETRY_COLLECTION_COMPONENT_SET_DAMAGE_PROPAGATION_DATA,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetDamageModel"),
            &raw mut U_GEOMETRY_COLLECTION_COMPONENT_SET_DAMAGE_MODEL,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetAnchoredByTransformedBox"),
            &raw mut U_GEOMETRY_COLLECTION_COMPONENT_SET_ANCHORED_BY_TRANSFORMED_BOX,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetAnchoredByIndex"),
            &raw mut U_GEOMETRY_COLLECTION_COMPONENT_SET_ANCHORED_BY_INDEX,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetAnchoredByBox"),
            &raw mut U_GEOMETRY_COLLECTION_COMPONENT_SET_ANCHORED_BY_BOX,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetAbandonedParticleCollisionProfileName"),
            &raw mut U_GEOMETRY_COLLECTION_COMPONENT_SET_ABANDONED_PARTICLE_COLLISION_PROFILE_NAME,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ResetState"),
            &raw mut U_GEOMETRY_COLLECTION_COMPONENT_RESET_STATE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RemoveAllAnchors"),
            &raw mut U_GEOMETRY_COLLECTION_COMPONENT_REMOVE_ALL_ANCHORS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ReceivePhysicsCollision"),
            &raw mut U_GEOMETRY_COLLECTION_COMPONENT_RECEIVE_PHYSICS_COLLISION,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("OnRep_RepStateData"),
            &raw mut U_GEOMETRY_COLLECTION_COMPONENT_ON_REP_REP_STATE_DATA,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("OnRep_RepDynamicData"),
            &raw mut U_GEOMETRY_COLLECTION_COMPONENT_ON_REP_REP_DYNAMIC_DATA,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("OnRep_RepData"),
            &raw mut U_GEOMETRY_COLLECTION_COMPONENT_ON_REP_REP_DATA,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from(
                "NotifyGeometryCollectionPhysicsStateChange__DelegateSignature",
            ),
            &raw mut U_GEOMETRY_COLLECTION_COMPONENT_NOTIFY_GEOMETRY_COLLECTION_PHYSICS_STATE_CHANGE_DELEGATE_SIGNATURE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from(
                "NotifyGeometryCollectionPhysicsLoadingStateChange__DelegateSignature",
            ),
            &raw mut U_GEOMETRY_COLLECTION_COMPONENT_NOTIFY_GEOMETRY_COLLECTION_PHYSICS_LOADING_STATE_CHANGE_DELEGATE_SIGNATURE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("IsRootBroken"),
            &raw mut U_GEOMETRY_COLLECTION_COMPONENT_IS_ROOT_BROKEN,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetUseStaticMeshCollisionForTraces"),
            &raw mut U_GEOMETRY_COLLECTION_COMPONENT_GET_USE_STATIC_MESH_COLLISION_FOR_TRACES,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetSolverActor"),
            &raw mut U_GEOMETRY_COLLECTION_COMPONENT_GET_SOLVER_ACTOR,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetRootInitialTransform"),
            &raw mut U_GEOMETRY_COLLECTION_COMPONENT_GET_ROOT_INITIAL_TRANSFORM,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetRootIndex"),
            &raw mut U_GEOMETRY_COLLECTION_COMPONENT_GET_ROOT_INDEX,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetRootCurrentTransform"),
            &raw mut U_GEOMETRY_COLLECTION_COMPONENT_GET_ROOT_CURRENT_TRANSFORM,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetMassAndExtents"),
            &raw mut U_GEOMETRY_COLLECTION_COMPONENT_GET_MASS_AND_EXTENTS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetLocalRestTransforms"),
            &raw mut U_GEOMETRY_COLLECTION_COMPONENT_GET_LOCAL_REST_TRANSFORMS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetLocalBounds"),
            &raw mut U_GEOMETRY_COLLECTION_COMPONENT_GET_LOCAL_BOUNDS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetInitialLocalRestTransforms"),
            &raw mut U_GEOMETRY_COLLECTION_COMPONENT_GET_INITIAL_LOCAL_REST_TRANSFORMS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetInitialLevel"),
            &raw mut U_GEOMETRY_COLLECTION_COMPONENT_GET_INITIAL_LEVEL,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetDebugInfo"),
            &raw mut U_GEOMETRY_COLLECTION_COMPONENT_GET_DEBUG_INFO,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetDamageThreshold"),
            &raw mut U_GEOMETRY_COLLECTION_COMPONENT_GET_DAMAGE_THRESHOLD,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ForceBrokenForCustomRenderer"),
            &raw mut U_GEOMETRY_COLLECTION_COMPONENT_FORCE_BROKEN_FOR_CUSTOM_RENDERER,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("FindLeafTransformByLineTrace"),
            &raw mut U_GEOMETRY_COLLECTION_COMPONENT_FIND_LEAF_TRANSFORM_BY_LINE_TRACE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("EnableRootProxyForCustomRenderer"),
            &raw mut U_GEOMETRY_COLLECTION_COMPONENT_ENABLE_ROOT_PROXY_FOR_CUSTOM_RENDERER,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CrumbleCluster"),
            &raw mut U_GEOMETRY_COLLECTION_COMPONENT_CRUMBLE_CLUSTER,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CrumbleActiveClusters"),
            &raw mut U_GEOMETRY_COLLECTION_COMPONENT_CRUMBLE_ACTIVE_CLUSTERS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ApplyPhysicsField"),
            &raw mut U_GEOMETRY_COLLECTION_COMPONENT_APPLY_PHYSICS_FIELD,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ApplyLinearVelocity"),
            &raw mut U_GEOMETRY_COLLECTION_COMPONENT_APPLY_LINEAR_VELOCITY,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ApplyKinematicField"),
            &raw mut U_GEOMETRY_COLLECTION_COMPONENT_APPLY_KINEMATIC_FIELD,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ApplyInternalStrain"),
            &raw mut U_GEOMETRY_COLLECTION_COMPONENT_APPLY_INTERNAL_STRAIN,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ApplyExternalStrain"),
            &raw mut U_GEOMETRY_COLLECTION_COMPONENT_APPLY_EXTERNAL_STRAIN,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ApplyBreakingLinearVelocity"),
            &raw mut U_GEOMETRY_COLLECTION_COMPONENT_APPLY_BREAKING_LINEAR_VELOCITY,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ApplyBreakingAngularVelocity"),
            &raw mut U_GEOMETRY_COLLECTION_COMPONENT_APPLY_BREAKING_ANGULAR_VELOCITY,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ApplyAssetDefaults"),
            &raw mut U_GEOMETRY_COLLECTION_COMPONENT_APPLY_ASSET_DEFAULTS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ApplyAngularVelocity"),
            &raw mut U_GEOMETRY_COLLECTION_COMPONENT_APPLY_ANGULAR_VELOCITY,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UGeometryCollectionISMPoolSubSystem::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("OnActorEndPlay"),
            &raw mut U_GEOMETRY_COLLECTION_ISM_POOL_SUB_SYSTEM_ON_ACTOR_END_PLAY,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UGeometryCollection::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetEnableNanite"),
            &raw mut U_GEOMETRY_COLLECTION_SET_ENABLE_NANITE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetDataflowAsset"),
            &raw mut U_GEOMETRY_COLLECTION_SET_DATAFLOW_ASSET,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetConvertVertexColorsToSRGB"),
            &raw mut U_GEOMETRY_COLLECTION_SET_CONVERT_VERTEX_COLORS_TO_SRGB,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetDataflowAsset"),
            &raw mut U_GEOMETRY_COLLECTION_GET_DATAFLOW_ASSET,
        );
    }
}
#[repr(C, align(8))]
pub struct FChaosBreakingEventData {
    pub location: crate::bindings::core_u_object::FVector,
    pub velocity: crate::bindings::core_u_object::FVector,
    pub mass: f32,
    __padding_end: [u8; 4],
}
impl FChaosBreakingEventData {}
#[repr(C, align(8))]
pub struct FChaosCollisionEventData {
    pub location: crate::bindings::core_u_object::FVector,
    pub normal: crate::bindings::core_u_object::FVector,
    pub velocity1: crate::bindings::core_u_object::FVector,
    pub velocity2: crate::bindings::core_u_object::FVector,
    pub mass1: f32,
    pub mass2: f32,
    pub impulse: crate::bindings::core_u_object::FVector,
}
impl FChaosCollisionEventData {}
#[repr(C, align(8))]
pub struct FChaosRemovalEventData {
    pub location: crate::bindings::core_u_object::FVector,
    pub mass: f32,
    pub particle_index: i32,
}
impl FChaosRemovalEventData {}
#[repr(C, align(8))]
pub struct FChaosTrailingEventData {
    pub location: crate::bindings::core_u_object::FVector,
    pub velocity: crate::bindings::core_u_object::FVector,
    pub angular_velocity: crate::bindings::core_u_object::FVector,
    pub mass: f32,
    pub particle_index: i32,
}
impl FChaosTrailingEventData {}
#[repr(C, align(4))]
pub struct FGeometryCollectionDamagePropagationData {
    pub b_enabled: bool,
    pub break_damage_propagation_factor: f32,
    pub shock_damage_propagation_factor: f32,
}
impl FGeometryCollectionDamagePropagationData {}
#[repr(C, align(4))]
pub struct FChaosBreakingEventRequestSettings {
    pub max_number_of_results: i32,
    pub min_radius: f32,
    pub min_speed: f32,
    pub min_mass: f32,
    pub max_distance: f32,
    pub sort_method: EChaosBreakingSortMethod,
    __padding_end: [u8; 3],
}
impl FChaosBreakingEventRequestSettings {}
#[repr(C, align(4))]
pub struct FChaosCollisionEventRequestSettings {
    pub max_number_results: i32,
    pub min_mass: f32,
    pub min_speed: f32,
    pub min_impulse: f32,
    pub max_distance: f32,
    pub sort_method: EChaosCollisionSortMethod,
    __padding_end: [u8; 3],
}
impl FChaosCollisionEventRequestSettings {}
#[repr(C, align(4))]
pub struct FChaosRemovalEventRequestSettings {
    pub max_number_of_results: i32,
    pub min_mass: f32,
    pub max_distance: f32,
    pub sort_method: EChaosRemovalSortMethod,
    __padding_end: [u8; 3],
}
impl FChaosRemovalEventRequestSettings {}
#[repr(C, align(4))]
pub struct FChaosTrailingEventRequestSettings {
    pub max_number_of_results: i32,
    pub min_mass: f32,
    pub min_speed: f32,
    pub min_angular_speed: f32,
    pub max_distance: f32,
    pub sort_method: EChaosTrailingSortMethod,
    __padding_end: [u8; 3],
}
impl FChaosTrailingEventRequestSettings {}
#[repr(C, align(16))]
pub struct FGeometryCollectionSource {
    pub source_geometry_object: crate::bindings::core_u_object::FSoftObjectPath,
    pub local_transform: crate::bindings::core_u_object::FTransform,
    pub source_material: TArray<UPtr<crate::bindings::engine::UMaterialInterface>>,
    pub instance_custom_data: TArray<f32>,
    pub b_add_internal_materials: bool,
    pub b_split_components: bool,
    pub b_set_internal_from_material_index: bool,
    __padding_end: [u8; 13],
}
impl FGeometryCollectionSource {}
#[repr(C, align(8))]
pub struct FGeometryCollectionAutoInstanceMesh {
    #[doc(hidden)]
    __padding_40: [u8; 40],
    pub mesh: UPtr<crate::bindings::engine::UStaticMesh>,
    pub materials: TArray<UPtr<crate::bindings::engine::UMaterialInterface>>,
    __padding_end: [u8; 24],
}
impl FGeometryCollectionAutoInstanceMesh {}
#[repr(C, align(8))]
pub struct FGeometryCollectionEmbeddedExemplar {
    __padding_end: [u8; 56],
}
impl FGeometryCollectionEmbeddedExemplar {}
#[repr(C, align(8))]
pub struct FGeometryCollectionProxyMeshMaterials {
    pub materials: TArray<UPtr<crate::bindings::engine::UMaterialInterface>>,
}
impl FGeometryCollectionProxyMeshMaterials {}
#[repr(C, align(8))]
pub struct FGeometryCollectionProxyMeshData {
    pub proxy_meshes: TArray<UPtr<crate::bindings::engine::UStaticMesh>>,
    pub mesh_transforms: TArray<crate::bindings::core_u_object::FTransform3f>,
    pub mesh_override_materials: TArray<FGeometryCollectionProxyMeshMaterials>,
}
impl FGeometryCollectionProxyMeshData {}
pub struct IGeometryCollectionExternalRenderInterface {}
#[repr(C, align(8))]
pub struct UGeometryCollectionExternalRenderInterface {
    __padding_end: [u8; 48],
}
impl UGeometryCollectionExternalRenderInterface {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UGeometryCollectionExternalRenderInterface")
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
pub struct IGeometryCollectionCustomDataInterface {}
#[repr(C, align(8))]
pub struct UGeometryCollectionCustomDataInterface {
    __padding_end: [u8; 48],
}
impl UGeometryCollectionCustomDataInterface {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UGeometryCollectionCustomDataInterface")
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
pub struct UChaosDestructionListener {
    #[doc(hidden)]
    __padding_656: [u8; 656],
    pub flags_656: u8,
    pub collision_event_request_settings: FChaosCollisionEventRequestSettings,
    pub breaking_event_request_settings: FChaosBreakingEventRequestSettings,
    pub trailing_event_request_settings: FChaosTrailingEventRequestSettings,
    pub removal_event_request_settings: FChaosRemovalEventRequestSettings,
    pub chaos_solver_actors: TSet<
        UPtr<crate::bindings::chaos_solver_engine::AChaosSolverActor>,
    >,
    pub geometry_collection_actors: TSet<UPtr<AGeometryCollectionActor>>,
    __padding_end: [u8; 464],
}
impl UChaosDestructionListener {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UChaosDestructionListener")
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
pub struct AGeometryCollectionActor {
    #[doc(hidden)]
    __padding_1136: [u8; 1136],
    pub geometry_collection_component: UPtr<UGeometryCollectionComponent>,
    __padding_end: [u8; 8],
}
impl AGeometryCollectionActor {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("AGeometryCollectionActor")
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
pub struct UGeometryCollectionBlueprintLibrary {
    __padding_end: [u8; 48],
}
impl UGeometryCollectionBlueprintLibrary {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UGeometryCollectionBlueprintLibrary")
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
pub struct UGeometryCollectionCache {
    __padding_end: [u8; 88],
}
impl UGeometryCollectionCache {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UGeometryCollectionCache")
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
pub struct UGeometryCollectionComponent {
    #[doc(hidden)]
    __padding_1616: [u8; 1616],
    pub chaos_solver_actor: UPtr<
        crate::bindings::chaos_solver_engine::AChaosSolverActor,
    >,
    pub rest_collection: UPtr<UGeometryCollection>,
    pub initialization_fields: TArray<
        UPtr<crate::bindings::field_system_engine::AFieldSystemActor>,
    >,
    #[doc(hidden)]
    __padding_1650: [u8; 2],
    pub object_type: crate::bindings::chaos::EObjectStateTypeEnum,
    pub gravity_group_index: i32,
    pub one_way_interaction_level: i32,
    pub b_density_from_physics_material: bool,
    pub b_force_motion_blur: bool,
    pub enable_clustering: bool,
    pub cluster_group_index: i32,
    pub max_cluster_level: i32,
    pub max_simulated_level: i32,
    pub damage_model: crate::bindings::chaos::EDamageModelTypeEnum,
    pub damage_threshold: TArray<f32>,
    pub b_use_size_specific_damage_threshold: bool,
    pub b_use_material_damage_modifiers: bool,
    pub damage_propagation_data: FGeometryCollectionDamagePropagationData,
    pub b_enable_damage_from_collision: bool,
    pub b_allow_removal_on_sleep: bool,
    pub b_allow_removal_on_break: bool,
    pub b_force_update_active_transforms: bool,
    #[doc(hidden)]
    __padding_1720: [u8; 4],
    pub collision_group: i32,
    pub collision_sample_fraction: f32,
    #[doc(hidden)]
    __padding_1744: [u8; 16],
    pub initial_velocity_type: crate::bindings::chaos::EInitialVelocityTypeEnum,
    pub initial_linear_velocity: crate::bindings::core_u_object::FVector,
    pub initial_angular_velocity: crate::bindings::core_u_object::FVector,
    #[doc(hidden)]
    __padding_2152: [u8; 352],
    pub desired_cache_time: f32,
    pub cache_playback: bool,
    #[doc(hidden)]
    __padding_2184: [u8; 27],
    pub b_notify_breaks: bool,
    pub b_notify_collisions: bool,
    pub b_notify_trailing: bool,
    pub b_notify_removals: bool,
    pub b_notify_crumblings: bool,
    pub b_crumbling_event_includes_children: bool,
    pub b_notify_global_breaks: bool,
    pub b_notify_global_collisions: bool,
    pub b_notify_global_removals: bool,
    pub b_notify_global_crumblings: bool,
    pub b_global_crumbling_event_includes_children: bool,
    pub b_store_velocities: bool,
    #[doc(hidden)]
    __padding_2197: [u8; 1],
    pub b_show_bone_colors: bool,
    pub b_update_component_transform_to_root_bone: bool,
    pub b_use_root_proxy_for_navigation: bool,
    pub b_update_navigation_in_tick: bool,
    pub run_time_data_collection_guid: crate::bindings::core_u_object::FGuid,
    #[doc(hidden)]
    __padding_2221: [u8; 1],
    pub b_enable_replication: bool,
    pub b_enable_abandon_after_level: bool,
    pub abandoned_collision_profile_name: FName,
    #[doc(hidden)]
    __padding_2248: [u8; 8],
    pub custom_renderer_type: TSubclassOf<crate::bindings::core_u_object::UObject>,
    pub b_override_custom_renderer: bool,
    #[doc(hidden)]
    __padding_2259: [u8; 2],
    pub b_use_static_mesh_collision_for_traces: bool,
    #[doc(hidden)]
    __padding_2280: [u8; 16],
    pub collision_profile_per_level: TArray<FName>,
    #[doc(hidden)]
    __padding_2312: [u8; 16],
    pub replication_abandon_after_level: i32,
    pub replication_max_position_and_velocity_correction_level: i32,
    __padding_end: [u8; 672],
}
impl UGeometryCollectionComponent {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UGeometryCollectionComponent")
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
pub struct AGeometryCollectionDebugDrawActor {
    __padding_end: [u8; 1328],
}
impl AGeometryCollectionDebugDrawActor {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("AGeometryCollectionDebugDrawActor")
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
pub struct UGeometryCollectionDebugDrawComponent {
    __padding_end: [u8; 264],
}
impl UGeometryCollectionDebugDrawComponent {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UGeometryCollectionDebugDrawComponent")
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
pub struct AGeometryCollectionISMPoolActor {
    #[doc(hidden)]
    __padding_1136: [u8; 1136],
    pub ism_pool_comp: UPtr<UGeometryCollectionISMPoolComponent>,
    pub ism_pool_debug_draw_comp: UPtr<UGeometryCollectionISMPoolDebugDrawComponent>,
}
impl AGeometryCollectionISMPoolActor {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("AGeometryCollectionISMPoolActor")
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
pub struct UGeometryCollectionISMPoolComponent {
    __padding_end: [u8; 960],
}
impl UGeometryCollectionISMPoolComponent {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UGeometryCollectionISMPoolComponent")
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
pub struct UGeometryCollectionISMPoolRenderer {
    __padding_end: [u8; 240],
}
impl UGeometryCollectionISMPoolRenderer {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UGeometryCollectionISMPoolRenderer")
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
pub struct UGeometryCollectionISMPoolSubSystem {
    __padding_end: [u8; 144],
}
impl UGeometryCollectionISMPoolSubSystem {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UGeometryCollectionISMPoolSubSystem")
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
pub struct UGeometryCollection {
    #[doc(hidden)]
    __padding_132: [u8; 132],
    pub damage_model: crate::bindings::chaos::EDamageModelTypeEnum,
    #[doc(hidden)]
    __padding_153: [u8; 20],
    pub b_use_material_damage_modifiers: bool,
    #[doc(hidden)]
    __padding_176: [u8; 16],
    pub geometry_source: TArray<FGeometryCollectionSource>,
    #[doc(hidden)]
    __padding_225: [u8; 33],
    pub b_strip_on_cook: bool,
    pub b_strip_render_data_on_cook: bool,
    pub custom_renderer_type: TSubclassOf<crate::bindings::core_u_object::UObject>,
    pub root_proxy_data: FGeometryCollectionProxyMeshData,
    #[doc(hidden)]
    __padding_304: [u8; 16],
    pub enable_nanite: bool,
    #[doc(hidden)]
    __padding_312: [u8; 7],
    pub b_convert_vertex_colors_to_srgb: bool,
    #[doc(hidden)]
    __padding_376: [u8; 56],
    pub physics_material: UPtr<crate::bindings::physics_core::UPhysicalMaterial>,
    pub b_density_from_physics_material: bool,
    #[doc(hidden)]
    __padding_392: [u8; 7],
    pub b_mass_as_density: bool,
    pub mass: f32,
    pub minimum_mass_clamp: f32,
    pub b_import_collision_from_source: bool,
    pub b_optimize_convexes: bool,
    #[doc(hidden)]
    __padding_416: [u8; 10],
    pub b_scale_on_removal: bool,
    pub b_remove_on_max_sleep: bool,
    pub b_automatic_crumble_partial_clusters: bool,
    pub maximum_sleep_time: crate::bindings::core_u_object::FVector2D,
    pub removal_duration: crate::bindings::core_u_object::FVector2D,
    pub b_slow_moving_as_sleeping: bool,
    pub slow_moving_velocity_threshold: f32,
    #[doc(hidden)]
    __padding_504: [u8; 40],
    pub asset_import_data: UPtr<crate::bindings::engine::UAssetImportData>,
    #[doc(hidden)]
    __padding_520: [u8; 8],
    pub dataflow_asset: UPtr<crate::bindings::dataflow_engine::UDataflow>,
    #[doc(hidden)]
    __padding_544: [u8; 16],
    pub overrides: TMap<FString, FString>,
    __padding_end: [u8; 264],
}
impl UGeometryCollection {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UGeometryCollection")
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
pub struct AGeometryCollectionRenderLevelSetActor {
    #[doc(hidden)]
    __padding_1136: [u8; 1136],
    pub target_volume_texture: UPtr<crate::bindings::engine::UVolumeTexture>,
    pub ray_march_material: UPtr<crate::bindings::engine::UMaterial>,
    pub surface_tolerance: f32,
    pub isovalue: f32,
    pub enabled: bool,
    pub render_volume_bounding_box: bool,
    __padding_end: [u8; 214],
}
impl AGeometryCollectionRenderLevelSetActor {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("AGeometryCollectionRenderLevelSetActor")
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
pub struct UGeometryCollectionRootProxyRenderer {
    __padding_end: [u8; 80],
}
impl UGeometryCollectionRootProxyRenderer {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UGeometryCollectionRootProxyRenderer")
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
pub struct UGeometryCollectionISMPoolDebugDrawComponent {
    __padding_end: [u8; 1744],
}
impl UGeometryCollectionISMPoolDebugDrawComponent {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UGeometryCollectionISMPoolDebugDrawComponent")
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
pub struct FChaosDestructionListener_OnCollisionEvents {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FChaosDestructionListener_OnBreakingEvents {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FChaosDestructionListener_OnTrailingEvents {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FChaosDestructionListener_OnRemovalEvents {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FGeometryCollectionComponent_NotifyGeometryCollectionPhysicsStateChange {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FGeometryCollectionComponent_NotifyGeometryCollectionPhysicsLoadingStateChange {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FGeometryCollectionComponent_OnChaosBreakEvent {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FGeometryCollectionComponent_OnChaosRemovalEvent {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FGeometryCollectionComponent_OnChaosCrumblingEvent {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FGeometryCollectionComponent_OnChaosPhysicsCollision {
    _opague: [u8; 24],
}
#[repr(transparent)]
pub struct EChaosBreakingSortMethod(pub u8);
impl EChaosBreakingSortMethod {
    pub const SORT_NONE: EChaosBreakingSortMethod = EChaosBreakingSortMethod(0);
    pub const SORT_BY_HIGHEST_MASS: EChaosBreakingSortMethod = EChaosBreakingSortMethod(
        1,
    );
    pub const SORT_BY_HIGHEST_SPEED: EChaosBreakingSortMethod = EChaosBreakingSortMethod(
        2,
    );
    pub const SORT_BY_NEAREST_FIRST: EChaosBreakingSortMethod = EChaosBreakingSortMethod(
        3,
    );
    pub const COUNT: EChaosBreakingSortMethod = EChaosBreakingSortMethod(4);
}
#[repr(transparent)]
pub struct EChaosCollisionSortMethod(pub u8);
impl EChaosCollisionSortMethod {
    pub const SORT_NONE: EChaosCollisionSortMethod = EChaosCollisionSortMethod(0);
    pub const SORT_BY_HIGHEST_MASS: EChaosCollisionSortMethod = EChaosCollisionSortMethod(
        1,
    );
    pub const SORT_BY_HIGHEST_SPEED: EChaosCollisionSortMethod = EChaosCollisionSortMethod(
        2,
    );
    pub const SORT_BY_HIGHEST_IMPULSE: EChaosCollisionSortMethod = EChaosCollisionSortMethod(
        3,
    );
    pub const SORT_BY_NEAREST_FIRST: EChaosCollisionSortMethod = EChaosCollisionSortMethod(
        4,
    );
    pub const COUNT: EChaosCollisionSortMethod = EChaosCollisionSortMethod(5);
}
#[repr(transparent)]
pub struct EChaosRemovalSortMethod(pub u8);
impl EChaosRemovalSortMethod {
    pub const SORT_NONE: EChaosRemovalSortMethod = EChaosRemovalSortMethod(0);
    pub const SORT_BY_HIGHEST_MASS: EChaosRemovalSortMethod = EChaosRemovalSortMethod(1);
    pub const SORT_BY_NEAREST_FIRST: EChaosRemovalSortMethod = EChaosRemovalSortMethod(
        2,
    );
    pub const COUNT: EChaosRemovalSortMethod = EChaosRemovalSortMethod(3);
}
#[repr(transparent)]
pub struct EChaosTrailingSortMethod(pub u8);
impl EChaosTrailingSortMethod {
    pub const SORT_NONE: EChaosTrailingSortMethod = EChaosTrailingSortMethod(0);
    pub const SORT_BY_HIGHEST_MASS: EChaosTrailingSortMethod = EChaosTrailingSortMethod(
        1,
    );
    pub const SORT_BY_HIGHEST_SPEED: EChaosTrailingSortMethod = EChaosTrailingSortMethod(
        2,
    );
    pub const SORT_BY_NEAREST_FIRST: EChaosTrailingSortMethod = EChaosTrailingSortMethod(
        3,
    );
    pub const COUNT: EChaosTrailingSortMethod = EChaosTrailingSortMethod(4);
}
#[repr(transparent)]
pub struct EGeometryCollectionDebugDrawActorHideGeometry(pub u8);
impl EGeometryCollectionDebugDrawActorHideGeometry {
    pub const HIDE_NONE: EGeometryCollectionDebugDrawActorHideGeometry = EGeometryCollectionDebugDrawActorHideGeometry(
        0,
    );
    pub const HIDE_WITH_COLLISION: EGeometryCollectionDebugDrawActorHideGeometry = EGeometryCollectionDebugDrawActorHideGeometry(
        1,
    );
    pub const HIDE_SELECTED: EGeometryCollectionDebugDrawActorHideGeometry = EGeometryCollectionDebugDrawActorHideGeometry(
        2,
    );
    pub const HIDE_WHOLE_COLLECTION: EGeometryCollectionDebugDrawActorHideGeometry = EGeometryCollectionDebugDrawActorHideGeometry(
        3,
    );
    pub const HIDE_ALL: EGeometryCollectionDebugDrawActorHideGeometry = EGeometryCollectionDebugDrawActorHideGeometry(
        4,
    );
}
