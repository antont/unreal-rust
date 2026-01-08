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
    pub u_chaos_destruction_listener_sort_trailing_events: *mut crate::ffi::UFunctionOpague,
    pub u_chaos_destruction_listener_sort_removal_events: *mut crate::ffi::UFunctionOpague,
    pub u_chaos_destruction_listener_sort_collision_events: *mut crate::ffi::UFunctionOpague,
    pub u_chaos_destruction_listener_sort_breaking_events: *mut crate::ffi::UFunctionOpague,
    pub u_chaos_destruction_listener_set_trailing_event_request_settings: *mut crate::ffi::UFunctionOpague,
    pub u_chaos_destruction_listener_set_trailing_event_enabled: *mut crate::ffi::UFunctionOpague,
    pub u_chaos_destruction_listener_set_removal_event_request_settings: *mut crate::ffi::UFunctionOpague,
    pub u_chaos_destruction_listener_set_removal_event_enabled: *mut crate::ffi::UFunctionOpague,
    pub u_chaos_destruction_listener_set_collision_event_request_settings: *mut crate::ffi::UFunctionOpague,
    pub u_chaos_destruction_listener_set_collision_event_enabled: *mut crate::ffi::UFunctionOpague,
    pub u_chaos_destruction_listener_set_breaking_event_request_settings: *mut crate::ffi::UFunctionOpague,
    pub u_chaos_destruction_listener_set_breaking_event_enabled: *mut crate::ffi::UFunctionOpague,
    pub u_chaos_destruction_listener_remove_geometry_collection_actor: *mut crate::ffi::UFunctionOpague,
    pub u_chaos_destruction_listener_remove_chaos_solver_actor: *mut crate::ffi::UFunctionOpague,
    pub u_chaos_destruction_listener_is_event_listening: *mut crate::ffi::UFunctionOpague,
    pub u_chaos_destruction_listener_add_geometry_collection_actor: *mut crate::ffi::UFunctionOpague,
    pub u_chaos_destruction_listener_add_chaos_solver_actor: *mut crate::ffi::UFunctionOpague,
    pub a_geometry_collection_actor_raycast_single: *mut crate::ffi::UFunctionOpague,
    pub u_geometry_collection_blueprint_library_set_ism_pool_custom_instance_data: *mut crate::ffi::UFunctionOpague,
    pub u_geometry_collection_blueprint_library_set_custom_instance_data_by_name: *mut crate::ffi::UFunctionOpague,
    pub u_geometry_collection_blueprint_library_set_custom_instance_data_by_index: *mut crate::ffi::UFunctionOpague,
    pub u_geometry_collection_component_set_use_static_mesh_collision_for_traces: *mut crate::ffi::UFunctionOpague,
    pub u_geometry_collection_component_set_use_material_damage_modifiers: *mut crate::ffi::UFunctionOpague,
    pub u_geometry_collection_component_set_solver_actor: *mut crate::ffi::UFunctionOpague,
    pub u_geometry_collection_component_set_root_proxy_component_space_transform: *mut crate::ffi::UFunctionOpague,
    pub u_geometry_collection_component_set_rest_collection: *mut crate::ffi::UFunctionOpague,
    pub u_geometry_collection_component_set_per_particle_collision_profile_name: *mut crate::ffi::UFunctionOpague,
    pub u_geometry_collection_component_set_per_level_collision_profile_names: *mut crate::ffi::UFunctionOpague,
    pub u_geometry_collection_component_set_one_way_interaction_level: *mut crate::ffi::UFunctionOpague,
    pub u_geometry_collection_component_set_notify_removals: *mut crate::ffi::UFunctionOpague,
    pub u_geometry_collection_component_set_notify_global_removals: *mut crate::ffi::UFunctionOpague,
    pub u_geometry_collection_component_set_notify_global_crumblings: *mut crate::ffi::UFunctionOpague,
    pub u_geometry_collection_component_set_notify_global_collision: *mut crate::ffi::UFunctionOpague,
    pub u_geometry_collection_component_set_notify_global_breaks: *mut crate::ffi::UFunctionOpague,
    pub u_geometry_collection_component_set_notify_crumblings: *mut crate::ffi::UFunctionOpague,
    pub u_geometry_collection_component_set_notify_breaks: *mut crate::ffi::UFunctionOpague,
    pub u_geometry_collection_component_set_local_rest_transforms: *mut crate::ffi::UFunctionOpague,
    pub u_geometry_collection_component_set_gravity_group_index: *mut crate::ffi::UFunctionOpague,
    pub u_geometry_collection_component_set_enable_damage_from_collision: *mut crate::ffi::UFunctionOpague,
    pub u_geometry_collection_component_set_density_from_physics_material: *mut crate::ffi::UFunctionOpague,
    pub u_geometry_collection_component_set_damage_threshold: *mut crate::ffi::UFunctionOpague,
    pub u_geometry_collection_component_set_damage_propagation_data: *mut crate::ffi::UFunctionOpague,
    pub u_geometry_collection_component_set_damage_model: *mut crate::ffi::UFunctionOpague,
    pub u_geometry_collection_component_set_anchored_by_transformed_box: *mut crate::ffi::UFunctionOpague,
    pub u_geometry_collection_component_set_anchored_by_index: *mut crate::ffi::UFunctionOpague,
    pub u_geometry_collection_component_set_anchored_by_box: *mut crate::ffi::UFunctionOpague,
    pub u_geometry_collection_component_set_abandoned_particle_collision_profile_name: *mut crate::ffi::UFunctionOpague,
    pub u_geometry_collection_component_reset_state: *mut crate::ffi::UFunctionOpague,
    pub u_geometry_collection_component_remove_all_anchors: *mut crate::ffi::UFunctionOpague,
    pub u_geometry_collection_component_receive_physics_collision: *mut crate::ffi::UFunctionOpague,
    pub u_geometry_collection_component_on_rep_rep_state_data: *mut crate::ffi::UFunctionOpague,
    pub u_geometry_collection_component_on_rep_rep_dynamic_data: *mut crate::ffi::UFunctionOpague,
    pub u_geometry_collection_component_on_rep_rep_data: *mut crate::ffi::UFunctionOpague,
    pub u_geometry_collection_component_notify_geometry_collection_physics_state_change_delegate_signature: *mut crate::ffi::UFunctionOpague,
    pub u_geometry_collection_component_notify_geometry_collection_physics_loading_state_change_delegate_signature: *mut crate::ffi::UFunctionOpague,
    pub u_geometry_collection_component_is_root_broken: *mut crate::ffi::UFunctionOpague,
    pub u_geometry_collection_component_get_use_static_mesh_collision_for_traces: *mut crate::ffi::UFunctionOpague,
    pub u_geometry_collection_component_get_solver_actor: *mut crate::ffi::UFunctionOpague,
    pub u_geometry_collection_component_get_root_initial_transform: *mut crate::ffi::UFunctionOpague,
    pub u_geometry_collection_component_get_root_index: *mut crate::ffi::UFunctionOpague,
    pub u_geometry_collection_component_get_root_current_transform: *mut crate::ffi::UFunctionOpague,
    pub u_geometry_collection_component_get_mass_and_extents: *mut crate::ffi::UFunctionOpague,
    pub u_geometry_collection_component_get_local_rest_transforms: *mut crate::ffi::UFunctionOpague,
    pub u_geometry_collection_component_get_local_bounds: *mut crate::ffi::UFunctionOpague,
    pub u_geometry_collection_component_get_initial_local_rest_transforms: *mut crate::ffi::UFunctionOpague,
    pub u_geometry_collection_component_get_initial_level: *mut crate::ffi::UFunctionOpague,
    pub u_geometry_collection_component_get_debug_info: *mut crate::ffi::UFunctionOpague,
    pub u_geometry_collection_component_get_damage_threshold: *mut crate::ffi::UFunctionOpague,
    pub u_geometry_collection_component_force_broken_for_custom_renderer: *mut crate::ffi::UFunctionOpague,
    pub u_geometry_collection_component_find_leaf_transform_by_line_trace: *mut crate::ffi::UFunctionOpague,
    pub u_geometry_collection_component_enable_root_proxy_for_custom_renderer: *mut crate::ffi::UFunctionOpague,
    pub u_geometry_collection_component_crumble_cluster: *mut crate::ffi::UFunctionOpague,
    pub u_geometry_collection_component_crumble_active_clusters: *mut crate::ffi::UFunctionOpague,
    pub u_geometry_collection_component_apply_physics_field: *mut crate::ffi::UFunctionOpague,
    pub u_geometry_collection_component_apply_linear_velocity: *mut crate::ffi::UFunctionOpague,
    pub u_geometry_collection_component_apply_kinematic_field: *mut crate::ffi::UFunctionOpague,
    pub u_geometry_collection_component_apply_internal_strain: *mut crate::ffi::UFunctionOpague,
    pub u_geometry_collection_component_apply_external_strain: *mut crate::ffi::UFunctionOpague,
    pub u_geometry_collection_component_apply_breaking_linear_velocity: *mut crate::ffi::UFunctionOpague,
    pub u_geometry_collection_component_apply_breaking_angular_velocity: *mut crate::ffi::UFunctionOpague,
    pub u_geometry_collection_component_apply_asset_defaults: *mut crate::ffi::UFunctionOpague,
    pub u_geometry_collection_component_apply_angular_velocity: *mut crate::ffi::UFunctionOpague,
    pub u_geometry_collection_ism_pool_sub_system_on_actor_end_play: *mut crate::ffi::UFunctionOpague,
    pub u_geometry_collection_set_enable_nanite: *mut crate::ffi::UFunctionOpague,
    pub u_geometry_collection_set_dataflow_asset: *mut crate::ffi::UFunctionOpague,
    pub u_geometry_collection_set_convert_vertex_colors_to_srgb: *mut crate::ffi::UFunctionOpague,
    pub u_geometry_collection_get_dataflow_asset: *mut crate::ffi::UFunctionOpague,
}
impl FunctionPtrs {
    pub const fn empty() -> Self {
        Self {
            u_chaos_destruction_listener_sort_trailing_events: std::ptr::null_mut(),
            u_chaos_destruction_listener_sort_removal_events: std::ptr::null_mut(),
            u_chaos_destruction_listener_sort_collision_events: std::ptr::null_mut(),
            u_chaos_destruction_listener_sort_breaking_events: std::ptr::null_mut(),
            u_chaos_destruction_listener_set_trailing_event_request_settings: std::ptr::null_mut(),
            u_chaos_destruction_listener_set_trailing_event_enabled: std::ptr::null_mut(),
            u_chaos_destruction_listener_set_removal_event_request_settings: std::ptr::null_mut(),
            u_chaos_destruction_listener_set_removal_event_enabled: std::ptr::null_mut(),
            u_chaos_destruction_listener_set_collision_event_request_settings: std::ptr::null_mut(),
            u_chaos_destruction_listener_set_collision_event_enabled: std::ptr::null_mut(),
            u_chaos_destruction_listener_set_breaking_event_request_settings: std::ptr::null_mut(),
            u_chaos_destruction_listener_set_breaking_event_enabled: std::ptr::null_mut(),
            u_chaos_destruction_listener_remove_geometry_collection_actor: std::ptr::null_mut(),
            u_chaos_destruction_listener_remove_chaos_solver_actor: std::ptr::null_mut(),
            u_chaos_destruction_listener_is_event_listening: std::ptr::null_mut(),
            u_chaos_destruction_listener_add_geometry_collection_actor: std::ptr::null_mut(),
            u_chaos_destruction_listener_add_chaos_solver_actor: std::ptr::null_mut(),
            a_geometry_collection_actor_raycast_single: std::ptr::null_mut(),
            u_geometry_collection_blueprint_library_set_ism_pool_custom_instance_data: std::ptr::null_mut(),
            u_geometry_collection_blueprint_library_set_custom_instance_data_by_name: std::ptr::null_mut(),
            u_geometry_collection_blueprint_library_set_custom_instance_data_by_index: std::ptr::null_mut(),
            u_geometry_collection_component_set_use_static_mesh_collision_for_traces: std::ptr::null_mut(),
            u_geometry_collection_component_set_use_material_damage_modifiers: std::ptr::null_mut(),
            u_geometry_collection_component_set_solver_actor: std::ptr::null_mut(),
            u_geometry_collection_component_set_root_proxy_component_space_transform: std::ptr::null_mut(),
            u_geometry_collection_component_set_rest_collection: std::ptr::null_mut(),
            u_geometry_collection_component_set_per_particle_collision_profile_name: std::ptr::null_mut(),
            u_geometry_collection_component_set_per_level_collision_profile_names: std::ptr::null_mut(),
            u_geometry_collection_component_set_one_way_interaction_level: std::ptr::null_mut(),
            u_geometry_collection_component_set_notify_removals: std::ptr::null_mut(),
            u_geometry_collection_component_set_notify_global_removals: std::ptr::null_mut(),
            u_geometry_collection_component_set_notify_global_crumblings: std::ptr::null_mut(),
            u_geometry_collection_component_set_notify_global_collision: std::ptr::null_mut(),
            u_geometry_collection_component_set_notify_global_breaks: std::ptr::null_mut(),
            u_geometry_collection_component_set_notify_crumblings: std::ptr::null_mut(),
            u_geometry_collection_component_set_notify_breaks: std::ptr::null_mut(),
            u_geometry_collection_component_set_local_rest_transforms: std::ptr::null_mut(),
            u_geometry_collection_component_set_gravity_group_index: std::ptr::null_mut(),
            u_geometry_collection_component_set_enable_damage_from_collision: std::ptr::null_mut(),
            u_geometry_collection_component_set_density_from_physics_material: std::ptr::null_mut(),
            u_geometry_collection_component_set_damage_threshold: std::ptr::null_mut(),
            u_geometry_collection_component_set_damage_propagation_data: std::ptr::null_mut(),
            u_geometry_collection_component_set_damage_model: std::ptr::null_mut(),
            u_geometry_collection_component_set_anchored_by_transformed_box: std::ptr::null_mut(),
            u_geometry_collection_component_set_anchored_by_index: std::ptr::null_mut(),
            u_geometry_collection_component_set_anchored_by_box: std::ptr::null_mut(),
            u_geometry_collection_component_set_abandoned_particle_collision_profile_name: std::ptr::null_mut(),
            u_geometry_collection_component_reset_state: std::ptr::null_mut(),
            u_geometry_collection_component_remove_all_anchors: std::ptr::null_mut(),
            u_geometry_collection_component_receive_physics_collision: std::ptr::null_mut(),
            u_geometry_collection_component_on_rep_rep_state_data: std::ptr::null_mut(),
            u_geometry_collection_component_on_rep_rep_dynamic_data: std::ptr::null_mut(),
            u_geometry_collection_component_on_rep_rep_data: std::ptr::null_mut(),
            u_geometry_collection_component_notify_geometry_collection_physics_state_change_delegate_signature: std::ptr::null_mut(),
            u_geometry_collection_component_notify_geometry_collection_physics_loading_state_change_delegate_signature: std::ptr::null_mut(),
            u_geometry_collection_component_is_root_broken: std::ptr::null_mut(),
            u_geometry_collection_component_get_use_static_mesh_collision_for_traces: std::ptr::null_mut(),
            u_geometry_collection_component_get_solver_actor: std::ptr::null_mut(),
            u_geometry_collection_component_get_root_initial_transform: std::ptr::null_mut(),
            u_geometry_collection_component_get_root_index: std::ptr::null_mut(),
            u_geometry_collection_component_get_root_current_transform: std::ptr::null_mut(),
            u_geometry_collection_component_get_mass_and_extents: std::ptr::null_mut(),
            u_geometry_collection_component_get_local_rest_transforms: std::ptr::null_mut(),
            u_geometry_collection_component_get_local_bounds: std::ptr::null_mut(),
            u_geometry_collection_component_get_initial_local_rest_transforms: std::ptr::null_mut(),
            u_geometry_collection_component_get_initial_level: std::ptr::null_mut(),
            u_geometry_collection_component_get_debug_info: std::ptr::null_mut(),
            u_geometry_collection_component_get_damage_threshold: std::ptr::null_mut(),
            u_geometry_collection_component_force_broken_for_custom_renderer: std::ptr::null_mut(),
            u_geometry_collection_component_find_leaf_transform_by_line_trace: std::ptr::null_mut(),
            u_geometry_collection_component_enable_root_proxy_for_custom_renderer: std::ptr::null_mut(),
            u_geometry_collection_component_crumble_cluster: std::ptr::null_mut(),
            u_geometry_collection_component_crumble_active_clusters: std::ptr::null_mut(),
            u_geometry_collection_component_apply_physics_field: std::ptr::null_mut(),
            u_geometry_collection_component_apply_linear_velocity: std::ptr::null_mut(),
            u_geometry_collection_component_apply_kinematic_field: std::ptr::null_mut(),
            u_geometry_collection_component_apply_internal_strain: std::ptr::null_mut(),
            u_geometry_collection_component_apply_external_strain: std::ptr::null_mut(),
            u_geometry_collection_component_apply_breaking_linear_velocity: std::ptr::null_mut(),
            u_geometry_collection_component_apply_breaking_angular_velocity: std::ptr::null_mut(),
            u_geometry_collection_component_apply_asset_defaults: std::ptr::null_mut(),
            u_geometry_collection_component_apply_angular_velocity: std::ptr::null_mut(),
            u_geometry_collection_ism_pool_sub_system_on_actor_end_play: std::ptr::null_mut(),
            u_geometry_collection_set_enable_nanite: std::ptr::null_mut(),
            u_geometry_collection_set_dataflow_asset: std::ptr::null_mut(),
            u_geometry_collection_set_convert_vertex_colors_to_srgb: std::ptr::null_mut(),
            u_geometry_collection_get_dataflow_asset: std::ptr::null_mut(),
        }
    }
}
pub fn initialize() {
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UChaosDestructionListener::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SortTrailingEvents"),
            &raw mut __FUNCTION_PTRS.u_chaos_destruction_listener_sort_trailing_events,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SortRemovalEvents"),
            &raw mut __FUNCTION_PTRS.u_chaos_destruction_listener_sort_removal_events,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SortCollisionEvents"),
            &raw mut __FUNCTION_PTRS.u_chaos_destruction_listener_sort_collision_events,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SortBreakingEvents"),
            &raw mut __FUNCTION_PTRS.u_chaos_destruction_listener_sort_breaking_events,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetTrailingEventRequestSettings"),
            &raw mut __FUNCTION_PTRS
                .u_chaos_destruction_listener_set_trailing_event_request_settings,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetTrailingEventEnabled"),
            &raw mut __FUNCTION_PTRS
                .u_chaos_destruction_listener_set_trailing_event_enabled,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetRemovalEventRequestSettings"),
            &raw mut __FUNCTION_PTRS
                .u_chaos_destruction_listener_set_removal_event_request_settings,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetRemovalEventEnabled"),
            &raw mut __FUNCTION_PTRS
                .u_chaos_destruction_listener_set_removal_event_enabled,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetCollisionEventRequestSettings"),
            &raw mut __FUNCTION_PTRS
                .u_chaos_destruction_listener_set_collision_event_request_settings,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetCollisionEventEnabled"),
            &raw mut __FUNCTION_PTRS
                .u_chaos_destruction_listener_set_collision_event_enabled,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetBreakingEventRequestSettings"),
            &raw mut __FUNCTION_PTRS
                .u_chaos_destruction_listener_set_breaking_event_request_settings,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetBreakingEventEnabled"),
            &raw mut __FUNCTION_PTRS
                .u_chaos_destruction_listener_set_breaking_event_enabled,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RemoveGeometryCollectionActor"),
            &raw mut __FUNCTION_PTRS
                .u_chaos_destruction_listener_remove_geometry_collection_actor,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RemoveChaosSolverActor"),
            &raw mut __FUNCTION_PTRS
                .u_chaos_destruction_listener_remove_chaos_solver_actor,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("IsEventListening"),
            &raw mut __FUNCTION_PTRS.u_chaos_destruction_listener_is_event_listening,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AddGeometryCollectionActor"),
            &raw mut __FUNCTION_PTRS
                .u_chaos_destruction_listener_add_geometry_collection_actor,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AddChaosSolverActor"),
            &raw mut __FUNCTION_PTRS.u_chaos_destruction_listener_add_chaos_solver_actor,
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
            &raw mut __FUNCTION_PTRS.a_geometry_collection_actor_raycast_single,
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
            &raw mut __FUNCTION_PTRS
                .u_geometry_collection_blueprint_library_set_ism_pool_custom_instance_data,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetCustomInstanceDataByName"),
            &raw mut __FUNCTION_PTRS
                .u_geometry_collection_blueprint_library_set_custom_instance_data_by_name,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetCustomInstanceDataByIndex"),
            &raw mut __FUNCTION_PTRS
                .u_geometry_collection_blueprint_library_set_custom_instance_data_by_index,
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
            &raw mut __FUNCTION_PTRS
                .u_geometry_collection_component_set_use_static_mesh_collision_for_traces,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetUseMaterialDamageModifiers"),
            &raw mut __FUNCTION_PTRS
                .u_geometry_collection_component_set_use_material_damage_modifiers,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetSolverActor"),
            &raw mut __FUNCTION_PTRS.u_geometry_collection_component_set_solver_actor,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetRootProxyComponentSpaceTransform"),
            &raw mut __FUNCTION_PTRS
                .u_geometry_collection_component_set_root_proxy_component_space_transform,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetRestCollection"),
            &raw mut __FUNCTION_PTRS.u_geometry_collection_component_set_rest_collection,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetPerParticleCollisionProfileName"),
            &raw mut __FUNCTION_PTRS
                .u_geometry_collection_component_set_per_particle_collision_profile_name,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetPerLevelCollisionProfileNames"),
            &raw mut __FUNCTION_PTRS
                .u_geometry_collection_component_set_per_level_collision_profile_names,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetOneWayInteractionLevel"),
            &raw mut __FUNCTION_PTRS
                .u_geometry_collection_component_set_one_way_interaction_level,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetNotifyRemovals"),
            &raw mut __FUNCTION_PTRS.u_geometry_collection_component_set_notify_removals,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetNotifyGlobalRemovals"),
            &raw mut __FUNCTION_PTRS
                .u_geometry_collection_component_set_notify_global_removals,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetNotifyGlobalCrumblings"),
            &raw mut __FUNCTION_PTRS
                .u_geometry_collection_component_set_notify_global_crumblings,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetNotifyGlobalCollision"),
            &raw mut __FUNCTION_PTRS
                .u_geometry_collection_component_set_notify_global_collision,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetNotifyGlobalBreaks"),
            &raw mut __FUNCTION_PTRS
                .u_geometry_collection_component_set_notify_global_breaks,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetNotifyCrumblings"),
            &raw mut __FUNCTION_PTRS
                .u_geometry_collection_component_set_notify_crumblings,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetNotifyBreaks"),
            &raw mut __FUNCTION_PTRS.u_geometry_collection_component_set_notify_breaks,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetLocalRestTransforms"),
            &raw mut __FUNCTION_PTRS
                .u_geometry_collection_component_set_local_rest_transforms,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetGravityGroupIndex"),
            &raw mut __FUNCTION_PTRS
                .u_geometry_collection_component_set_gravity_group_index,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetEnableDamageFromCollision"),
            &raw mut __FUNCTION_PTRS
                .u_geometry_collection_component_set_enable_damage_from_collision,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetDensityFromPhysicsMaterial"),
            &raw mut __FUNCTION_PTRS
                .u_geometry_collection_component_set_density_from_physics_material,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetDamageThreshold"),
            &raw mut __FUNCTION_PTRS.u_geometry_collection_component_set_damage_threshold,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetDamagePropagationData"),
            &raw mut __FUNCTION_PTRS
                .u_geometry_collection_component_set_damage_propagation_data,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetDamageModel"),
            &raw mut __FUNCTION_PTRS.u_geometry_collection_component_set_damage_model,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetAnchoredByTransformedBox"),
            &raw mut __FUNCTION_PTRS
                .u_geometry_collection_component_set_anchored_by_transformed_box,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetAnchoredByIndex"),
            &raw mut __FUNCTION_PTRS
                .u_geometry_collection_component_set_anchored_by_index,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetAnchoredByBox"),
            &raw mut __FUNCTION_PTRS.u_geometry_collection_component_set_anchored_by_box,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetAbandonedParticleCollisionProfileName"),
            &raw mut __FUNCTION_PTRS
                .u_geometry_collection_component_set_abandoned_particle_collision_profile_name,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ResetState"),
            &raw mut __FUNCTION_PTRS.u_geometry_collection_component_reset_state,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RemoveAllAnchors"),
            &raw mut __FUNCTION_PTRS.u_geometry_collection_component_remove_all_anchors,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ReceivePhysicsCollision"),
            &raw mut __FUNCTION_PTRS
                .u_geometry_collection_component_receive_physics_collision,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("OnRep_RepStateData"),
            &raw mut __FUNCTION_PTRS
                .u_geometry_collection_component_on_rep_rep_state_data,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("OnRep_RepDynamicData"),
            &raw mut __FUNCTION_PTRS
                .u_geometry_collection_component_on_rep_rep_dynamic_data,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("OnRep_RepData"),
            &raw mut __FUNCTION_PTRS.u_geometry_collection_component_on_rep_rep_data,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from(
                "NotifyGeometryCollectionPhysicsStateChange__DelegateSignature",
            ),
            &raw mut __FUNCTION_PTRS
                .u_geometry_collection_component_notify_geometry_collection_physics_state_change_delegate_signature,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from(
                "NotifyGeometryCollectionPhysicsLoadingStateChange__DelegateSignature",
            ),
            &raw mut __FUNCTION_PTRS
                .u_geometry_collection_component_notify_geometry_collection_physics_loading_state_change_delegate_signature,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("IsRootBroken"),
            &raw mut __FUNCTION_PTRS.u_geometry_collection_component_is_root_broken,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetUseStaticMeshCollisionForTraces"),
            &raw mut __FUNCTION_PTRS
                .u_geometry_collection_component_get_use_static_mesh_collision_for_traces,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetSolverActor"),
            &raw mut __FUNCTION_PTRS.u_geometry_collection_component_get_solver_actor,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetRootInitialTransform"),
            &raw mut __FUNCTION_PTRS
                .u_geometry_collection_component_get_root_initial_transform,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetRootIndex"),
            &raw mut __FUNCTION_PTRS.u_geometry_collection_component_get_root_index,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetRootCurrentTransform"),
            &raw mut __FUNCTION_PTRS
                .u_geometry_collection_component_get_root_current_transform,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetMassAndExtents"),
            &raw mut __FUNCTION_PTRS.u_geometry_collection_component_get_mass_and_extents,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetLocalRestTransforms"),
            &raw mut __FUNCTION_PTRS
                .u_geometry_collection_component_get_local_rest_transforms,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetLocalBounds"),
            &raw mut __FUNCTION_PTRS.u_geometry_collection_component_get_local_bounds,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetInitialLocalRestTransforms"),
            &raw mut __FUNCTION_PTRS
                .u_geometry_collection_component_get_initial_local_rest_transforms,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetInitialLevel"),
            &raw mut __FUNCTION_PTRS.u_geometry_collection_component_get_initial_level,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetDebugInfo"),
            &raw mut __FUNCTION_PTRS.u_geometry_collection_component_get_debug_info,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetDamageThreshold"),
            &raw mut __FUNCTION_PTRS.u_geometry_collection_component_get_damage_threshold,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ForceBrokenForCustomRenderer"),
            &raw mut __FUNCTION_PTRS
                .u_geometry_collection_component_force_broken_for_custom_renderer,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("FindLeafTransformByLineTrace"),
            &raw mut __FUNCTION_PTRS
                .u_geometry_collection_component_find_leaf_transform_by_line_trace,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("EnableRootProxyForCustomRenderer"),
            &raw mut __FUNCTION_PTRS
                .u_geometry_collection_component_enable_root_proxy_for_custom_renderer,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CrumbleCluster"),
            &raw mut __FUNCTION_PTRS.u_geometry_collection_component_crumble_cluster,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CrumbleActiveClusters"),
            &raw mut __FUNCTION_PTRS
                .u_geometry_collection_component_crumble_active_clusters,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ApplyPhysicsField"),
            &raw mut __FUNCTION_PTRS.u_geometry_collection_component_apply_physics_field,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ApplyLinearVelocity"),
            &raw mut __FUNCTION_PTRS
                .u_geometry_collection_component_apply_linear_velocity,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ApplyKinematicField"),
            &raw mut __FUNCTION_PTRS
                .u_geometry_collection_component_apply_kinematic_field,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ApplyInternalStrain"),
            &raw mut __FUNCTION_PTRS
                .u_geometry_collection_component_apply_internal_strain,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ApplyExternalStrain"),
            &raw mut __FUNCTION_PTRS
                .u_geometry_collection_component_apply_external_strain,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ApplyBreakingLinearVelocity"),
            &raw mut __FUNCTION_PTRS
                .u_geometry_collection_component_apply_breaking_linear_velocity,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ApplyBreakingAngularVelocity"),
            &raw mut __FUNCTION_PTRS
                .u_geometry_collection_component_apply_breaking_angular_velocity,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ApplyAssetDefaults"),
            &raw mut __FUNCTION_PTRS.u_geometry_collection_component_apply_asset_defaults,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ApplyAngularVelocity"),
            &raw mut __FUNCTION_PTRS
                .u_geometry_collection_component_apply_angular_velocity,
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
            &raw mut __FUNCTION_PTRS
                .u_geometry_collection_ism_pool_sub_system_on_actor_end_play,
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
            &raw mut __FUNCTION_PTRS.u_geometry_collection_set_enable_nanite,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetDataflowAsset"),
            &raw mut __FUNCTION_PTRS.u_geometry_collection_set_dataflow_asset,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetConvertVertexColorsToSRGB"),
            &raw mut __FUNCTION_PTRS
                .u_geometry_collection_set_convert_vertex_colors_to_srgb,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetDataflowAsset"),
            &raw mut __FUNCTION_PTRS.u_geometry_collection_get_dataflow_asset,
        );
    }
}
#[repr(C, align(8))]
pub struct FChaosBreakingEventData {
    pub location: crate::bindings::core_u_object::FVector,
    pub velocity: crate::bindings::core_u_object::FVector,
    pub mass: f32,
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
}
impl FChaosCollisionEventRequestSettings {}
#[repr(C, align(4))]
pub struct FChaosRemovalEventRequestSettings {
    pub max_number_of_results: i32,
    pub min_mass: f32,
    pub max_distance: f32,
    pub sort_method: EChaosRemovalSortMethod,
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
    pub fn sort_trailing_events(
        &mut self,
        trailing_events: &mut TArray<FChaosTrailingEventData>,
        sort_method: EChaosTrailingSortMethod,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<17>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::geometry_collection_engine::__FUNCTION_PTRS
                    .u_chaos_destruction_listener_sort_trailing_events,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                trailing_events,
                __buffer.add(0).cast::<TArray<FChaosTrailingEventData>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &sort_method,
                __buffer.add(16).cast::<EChaosTrailingSortMethod>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::geometry_collection_engine::__FUNCTION_PTRS
                    .u_chaos_destruction_listener_sort_trailing_events,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(0)
                .cast::<TArray<FChaosTrailingEventData>>()
                .swap(trailing_events);
        }
    }
    pub fn sort_removal_events(
        &mut self,
        removal_events: &mut TArray<FChaosRemovalEventData>,
        sort_method: EChaosRemovalSortMethod,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<17>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::geometry_collection_engine::__FUNCTION_PTRS
                    .u_chaos_destruction_listener_sort_removal_events,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                removal_events,
                __buffer.add(0).cast::<TArray<FChaosRemovalEventData>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &sort_method,
                __buffer.add(16).cast::<EChaosRemovalSortMethod>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::geometry_collection_engine::__FUNCTION_PTRS
                    .u_chaos_destruction_listener_sort_removal_events,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(0)
                .cast::<TArray<FChaosRemovalEventData>>()
                .swap(removal_events);
        }
    }
    pub fn sort_collision_events(
        &mut self,
        collision_events: &mut TArray<FChaosCollisionEventData>,
        sort_method: EChaosCollisionSortMethod,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<17>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::geometry_collection_engine::__FUNCTION_PTRS
                    .u_chaos_destruction_listener_sort_collision_events,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                collision_events,
                __buffer.add(0).cast::<TArray<FChaosCollisionEventData>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &sort_method,
                __buffer.add(16).cast::<EChaosCollisionSortMethod>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::geometry_collection_engine::__FUNCTION_PTRS
                    .u_chaos_destruction_listener_sort_collision_events,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(0)
                .cast::<TArray<FChaosCollisionEventData>>()
                .swap(collision_events);
        }
    }
    pub fn sort_breaking_events(
        &mut self,
        breaking_events: &mut TArray<FChaosBreakingEventData>,
        sort_method: EChaosBreakingSortMethod,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<17>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::geometry_collection_engine::__FUNCTION_PTRS
                    .u_chaos_destruction_listener_sort_breaking_events,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                breaking_events,
                __buffer.add(0).cast::<TArray<FChaosBreakingEventData>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &sort_method,
                __buffer.add(16).cast::<EChaosBreakingSortMethod>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::geometry_collection_engine::__FUNCTION_PTRS
                    .u_chaos_destruction_listener_sort_breaking_events,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(0)
                .cast::<TArray<FChaosBreakingEventData>>()
                .swap(breaking_events);
        }
    }
    pub fn set_trailing_event_request_settings(
        &mut self,
        in_settings: &FChaosTrailingEventRequestSettings,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::geometry_collection_engine::__FUNCTION_PTRS
                    .u_chaos_destruction_listener_set_trailing_event_request_settings,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_settings,
                __buffer.add(0).cast::<FChaosTrailingEventRequestSettings>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::geometry_collection_engine::__FUNCTION_PTRS
                    .u_chaos_destruction_listener_set_trailing_event_request_settings,
                __buffer,
            )
        };
    }
    pub fn set_trailing_event_enabled(&mut self, b_is_enabled: bool) {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::geometry_collection_engine::__FUNCTION_PTRS
                    .u_chaos_destruction_listener_set_trailing_event_enabled,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_is_enabled,
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
                crate::bindings::geometry_collection_engine::__FUNCTION_PTRS
                    .u_chaos_destruction_listener_set_trailing_event_enabled,
                __buffer,
            )
        };
    }
    pub fn set_removal_event_request_settings(
        &mut self,
        in_settings: &FChaosRemovalEventRequestSettings,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::geometry_collection_engine::__FUNCTION_PTRS
                    .u_chaos_destruction_listener_set_removal_event_request_settings,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_settings,
                __buffer.add(0).cast::<FChaosRemovalEventRequestSettings>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::geometry_collection_engine::__FUNCTION_PTRS
                    .u_chaos_destruction_listener_set_removal_event_request_settings,
                __buffer,
            )
        };
    }
    pub fn set_removal_event_enabled(&mut self, b_is_enabled: bool) {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::geometry_collection_engine::__FUNCTION_PTRS
                    .u_chaos_destruction_listener_set_removal_event_enabled,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_is_enabled,
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
                crate::bindings::geometry_collection_engine::__FUNCTION_PTRS
                    .u_chaos_destruction_listener_set_removal_event_enabled,
                __buffer,
            )
        };
    }
    pub fn set_collision_event_request_settings(
        &mut self,
        in_settings: &FChaosCollisionEventRequestSettings,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::geometry_collection_engine::__FUNCTION_PTRS
                    .u_chaos_destruction_listener_set_collision_event_request_settings,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_settings,
                __buffer.add(0).cast::<FChaosCollisionEventRequestSettings>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::geometry_collection_engine::__FUNCTION_PTRS
                    .u_chaos_destruction_listener_set_collision_event_request_settings,
                __buffer,
            )
        };
    }
    pub fn set_collision_event_enabled(&mut self, b_is_enabled: bool) {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::geometry_collection_engine::__FUNCTION_PTRS
                    .u_chaos_destruction_listener_set_collision_event_enabled,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_is_enabled,
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
                crate::bindings::geometry_collection_engine::__FUNCTION_PTRS
                    .u_chaos_destruction_listener_set_collision_event_enabled,
                __buffer,
            )
        };
    }
    pub fn set_breaking_event_request_settings(
        &mut self,
        in_settings: &FChaosBreakingEventRequestSettings,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::geometry_collection_engine::__FUNCTION_PTRS
                    .u_chaos_destruction_listener_set_breaking_event_request_settings,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_settings,
                __buffer.add(0).cast::<FChaosBreakingEventRequestSettings>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::geometry_collection_engine::__FUNCTION_PTRS
                    .u_chaos_destruction_listener_set_breaking_event_request_settings,
                __buffer,
            )
        };
    }
    pub fn set_breaking_event_enabled(&mut self, b_is_enabled: bool) {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::geometry_collection_engine::__FUNCTION_PTRS
                    .u_chaos_destruction_listener_set_breaking_event_enabled,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_is_enabled,
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
                crate::bindings::geometry_collection_engine::__FUNCTION_PTRS
                    .u_chaos_destruction_listener_set_breaking_event_enabled,
                __buffer,
            )
        };
    }
    pub fn remove_geometry_collection_actor(
        &mut self,
        geometry_collection_actor: UPtr<AGeometryCollectionActor>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::geometry_collection_engine::__FUNCTION_PTRS
                    .u_chaos_destruction_listener_remove_geometry_collection_actor,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &geometry_collection_actor,
                __buffer.add(0).cast::<UPtr<AGeometryCollectionActor>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::geometry_collection_engine::__FUNCTION_PTRS
                    .u_chaos_destruction_listener_remove_geometry_collection_actor,
                __buffer,
            )
        };
    }
    pub fn remove_chaos_solver_actor(
        &mut self,
        chaos_solver_actor: UPtr<crate::bindings::chaos_solver_engine::AChaosSolverActor>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::geometry_collection_engine::__FUNCTION_PTRS
                    .u_chaos_destruction_listener_remove_chaos_solver_actor,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &chaos_solver_actor,
                __buffer
                    .add(0)
                    .cast::<
                        UPtr<crate::bindings::chaos_solver_engine::AChaosSolverActor>,
                    >(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::geometry_collection_engine::__FUNCTION_PTRS
                    .u_chaos_destruction_listener_remove_chaos_solver_actor,
                __buffer,
            )
        };
    }
    pub fn is_event_listening(&self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::geometry_collection_engine::__FUNCTION_PTRS
                    .u_chaos_destruction_listener_is_event_listening,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::geometry_collection_engine::__FUNCTION_PTRS
                    .u_chaos_destruction_listener_is_event_listening,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn add_geometry_collection_actor(
        &mut self,
        geometry_collection_actor: UPtr<AGeometryCollectionActor>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::geometry_collection_engine::__FUNCTION_PTRS
                    .u_chaos_destruction_listener_add_geometry_collection_actor,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &geometry_collection_actor,
                __buffer.add(0).cast::<UPtr<AGeometryCollectionActor>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::geometry_collection_engine::__FUNCTION_PTRS
                    .u_chaos_destruction_listener_add_geometry_collection_actor,
                __buffer,
            )
        };
    }
    pub fn add_chaos_solver_actor(
        &mut self,
        chaos_solver_actor: UPtr<crate::bindings::chaos_solver_engine::AChaosSolverActor>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::geometry_collection_engine::__FUNCTION_PTRS
                    .u_chaos_destruction_listener_add_chaos_solver_actor,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &chaos_solver_actor,
                __buffer
                    .add(0)
                    .cast::<
                        UPtr<crate::bindings::chaos_solver_engine::AChaosSolverActor>,
                    >(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::geometry_collection_engine::__FUNCTION_PTRS
                    .u_chaos_destruction_listener_add_chaos_solver_actor,
                __buffer,
            )
        };
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
    pub fn raycast_single(
        &self,
        start: crate::bindings::core_u_object::FVector,
        end: crate::bindings::core_u_object::FVector,
        out_hit: &mut crate::bindings::engine::FHitResult,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<313>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::geometry_collection_engine::__FUNCTION_PTRS
                    .a_geometry_collection_actor_raycast_single,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &start,
                __buffer.add(0).cast::<crate::bindings::core_u_object::FVector>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &end,
                __buffer.add(24).cast::<crate::bindings::core_u_object::FVector>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_hit,
                __buffer.add(48).cast::<crate::bindings::engine::FHitResult>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::geometry_collection_engine::__FUNCTION_PTRS
                    .a_geometry_collection_actor_raycast_single,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(48).cast::<crate::bindings::engine::FHitResult>().swap(out_hit);
        }
        unsafe { __buffer.add(312).cast::<bool>().read() }
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
    pub fn set_ism_pool_custom_instance_data(
        geometry_collection_component: UPtr<UGeometryCollectionComponent>,
        custom_data_index: i32,
        custom_data_value: f32,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::geometry_collection_engine::__FUNCTION_PTRS
                    .u_geometry_collection_blueprint_library_set_ism_pool_custom_instance_data,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &geometry_collection_component,
                __buffer.add(0).cast::<UPtr<UGeometryCollectionComponent>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &custom_data_index,
                __buffer.add(8).cast::<i32>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &custom_data_value,
                __buffer.add(12).cast::<f32>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::geometry_collection_engine::UGeometryCollectionBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::geometry_collection_engine::__FUNCTION_PTRS
                    .u_geometry_collection_blueprint_library_set_ism_pool_custom_instance_data,
                __buffer,
            )
        };
    }
    pub fn set_custom_instance_data_by_name(
        geometry_collection_component: UPtr<UGeometryCollectionComponent>,
        custom_data_name: FName,
        custom_data_value: f32,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::geometry_collection_engine::__FUNCTION_PTRS
                    .u_geometry_collection_blueprint_library_set_custom_instance_data_by_name,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &geometry_collection_component,
                __buffer.add(0).cast::<UPtr<UGeometryCollectionComponent>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &custom_data_name,
                __buffer.add(8).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &custom_data_value,
                __buffer.add(20).cast::<f32>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::geometry_collection_engine::UGeometryCollectionBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::geometry_collection_engine::__FUNCTION_PTRS
                    .u_geometry_collection_blueprint_library_set_custom_instance_data_by_name,
                __buffer,
            )
        };
    }
    pub fn set_custom_instance_data_by_index(
        geometry_collection_component: UPtr<UGeometryCollectionComponent>,
        custom_data_index: i32,
        custom_data_value: f32,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::geometry_collection_engine::__FUNCTION_PTRS
                    .u_geometry_collection_blueprint_library_set_custom_instance_data_by_index,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &geometry_collection_component,
                __buffer.add(0).cast::<UPtr<UGeometryCollectionComponent>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &custom_data_index,
                __buffer.add(8).cast::<i32>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &custom_data_value,
                __buffer.add(12).cast::<f32>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::geometry_collection_engine::UGeometryCollectionBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::geometry_collection_engine::__FUNCTION_PTRS
                    .u_geometry_collection_blueprint_library_set_custom_instance_data_by_index,
                __buffer,
            )
        };
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
    pub fn set_use_static_mesh_collision_for_traces(
        &mut self,
        b_in_use_static_mesh_collision_for_traces: bool,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::geometry_collection_engine::__FUNCTION_PTRS
                    .u_geometry_collection_component_set_use_static_mesh_collision_for_traces,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_in_use_static_mesh_collision_for_traces,
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
                crate::bindings::geometry_collection_engine::__FUNCTION_PTRS
                    .u_geometry_collection_component_set_use_static_mesh_collision_for_traces,
                __buffer,
            )
        };
    }
    pub fn set_use_material_damage_modifiers(
        &mut self,
        b_in_use_material_damage_modifiers: bool,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::geometry_collection_engine::__FUNCTION_PTRS
                    .u_geometry_collection_component_set_use_material_damage_modifiers,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_in_use_material_damage_modifiers,
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
                crate::bindings::geometry_collection_engine::__FUNCTION_PTRS
                    .u_geometry_collection_component_set_use_material_damage_modifiers,
                __buffer,
            )
        };
    }
    pub fn set_solver_actor(
        &mut self,
        in_solver_actor: UPtr<crate::bindings::chaos_solver_engine::AChaosSolverActor>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::geometry_collection_engine::__FUNCTION_PTRS
                    .u_geometry_collection_component_set_solver_actor,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_solver_actor,
                __buffer
                    .add(0)
                    .cast::<
                        UPtr<crate::bindings::chaos_solver_engine::AChaosSolverActor>,
                    >(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::geometry_collection_engine::__FUNCTION_PTRS
                    .u_geometry_collection_component_set_solver_actor,
                __buffer,
            )
        };
    }
    pub fn set_root_proxy_component_space_transform(
        &mut self,
        index: i32,
        root_proxy_transform: &crate::bindings::core_u_object::FTransform,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<112>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::geometry_collection_engine::__FUNCTION_PTRS
                    .u_geometry_collection_component_set_root_proxy_component_space_transform,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&index, __buffer.add(0).cast::<i32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                root_proxy_transform,
                __buffer.add(16).cast::<crate::bindings::core_u_object::FTransform>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::geometry_collection_engine::__FUNCTION_PTRS
                    .u_geometry_collection_component_set_root_proxy_component_space_transform,
                __buffer,
            )
        };
    }
    pub fn set_rest_collection(
        &mut self,
        rest_collection_in: UPtr<UGeometryCollection>,
        b_apply_asset_defaults: bool,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<9>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::geometry_collection_engine::__FUNCTION_PTRS
                    .u_geometry_collection_component_set_rest_collection,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &rest_collection_in,
                __buffer.add(0).cast::<UPtr<UGeometryCollection>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_apply_asset_defaults,
                __buffer.add(8).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::geometry_collection_engine::__FUNCTION_PTRS
                    .u_geometry_collection_component_set_rest_collection,
                __buffer,
            )
        };
    }
    pub fn set_per_particle_collision_profile_name(
        &mut self,
        bone_ids: &TArray<i32>,
        profile_name: FName,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<28>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::geometry_collection_engine::__FUNCTION_PTRS
                    .u_geometry_collection_component_set_per_particle_collision_profile_name,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                bone_ids,
                __buffer.add(0).cast::<TArray<i32>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &profile_name,
                __buffer.add(16).cast::<FName>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::geometry_collection_engine::__FUNCTION_PTRS
                    .u_geometry_collection_component_set_per_particle_collision_profile_name,
                __buffer,
            )
        };
    }
    pub fn set_per_level_collision_profile_names(
        &mut self,
        profile_names: &TArray<FName>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::geometry_collection_engine::__FUNCTION_PTRS
                    .u_geometry_collection_component_set_per_level_collision_profile_names,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                profile_names,
                __buffer.add(0).cast::<TArray<FName>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::geometry_collection_engine::__FUNCTION_PTRS
                    .u_geometry_collection_component_set_per_level_collision_profile_names,
                __buffer,
            )
        };
    }
    pub fn set_one_way_interaction_level(&mut self, in_one_way_interaction_level: i32) {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::geometry_collection_engine::__FUNCTION_PTRS
                    .u_geometry_collection_component_set_one_way_interaction_level,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_one_way_interaction_level,
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
                crate::bindings::geometry_collection_engine::__FUNCTION_PTRS
                    .u_geometry_collection_component_set_one_way_interaction_level,
                __buffer,
            )
        };
    }
    pub fn set_notify_removals(&mut self, b_new_notify_removals: bool) {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::geometry_collection_engine::__FUNCTION_PTRS
                    .u_geometry_collection_component_set_notify_removals,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_new_notify_removals,
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
                crate::bindings::geometry_collection_engine::__FUNCTION_PTRS
                    .u_geometry_collection_component_set_notify_removals,
                __buffer,
            )
        };
    }
    pub fn set_notify_global_removals(&mut self, b_new_notify_global_removals: bool) {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::geometry_collection_engine::__FUNCTION_PTRS
                    .u_geometry_collection_component_set_notify_global_removals,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_new_notify_global_removals,
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
                crate::bindings::geometry_collection_engine::__FUNCTION_PTRS
                    .u_geometry_collection_component_set_notify_global_removals,
                __buffer,
            )
        };
    }
    pub fn set_notify_global_crumblings(
        &mut self,
        b_new_notify_global_crumblings: bool,
        b_global_new_crumbling_event_includes_children: bool,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<2>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::geometry_collection_engine::__FUNCTION_PTRS
                    .u_geometry_collection_component_set_notify_global_crumblings,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_new_notify_global_crumblings,
                __buffer.add(0).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_global_new_crumbling_event_includes_children,
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
                crate::bindings::geometry_collection_engine::__FUNCTION_PTRS
                    .u_geometry_collection_component_set_notify_global_crumblings,
                __buffer,
            )
        };
    }
    pub fn set_notify_global_collision(&mut self, b_new_notify_global_collisions: bool) {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::geometry_collection_engine::__FUNCTION_PTRS
                    .u_geometry_collection_component_set_notify_global_collision,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_new_notify_global_collisions,
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
                crate::bindings::geometry_collection_engine::__FUNCTION_PTRS
                    .u_geometry_collection_component_set_notify_global_collision,
                __buffer,
            )
        };
    }
    pub fn set_notify_global_breaks(&mut self, b_new_notify_global_breaks: bool) {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::geometry_collection_engine::__FUNCTION_PTRS
                    .u_geometry_collection_component_set_notify_global_breaks,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_new_notify_global_breaks,
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
                crate::bindings::geometry_collection_engine::__FUNCTION_PTRS
                    .u_geometry_collection_component_set_notify_global_breaks,
                __buffer,
            )
        };
    }
    pub fn set_notify_crumblings(
        &mut self,
        b_new_notify_crumblings: bool,
        b_new_crumbling_event_includes_children: bool,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<2>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::geometry_collection_engine::__FUNCTION_PTRS
                    .u_geometry_collection_component_set_notify_crumblings,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_new_notify_crumblings,
                __buffer.add(0).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_new_crumbling_event_includes_children,
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
                crate::bindings::geometry_collection_engine::__FUNCTION_PTRS
                    .u_geometry_collection_component_set_notify_crumblings,
                __buffer,
            )
        };
    }
    pub fn set_notify_breaks(&mut self, b_new_notify_breaks: bool) {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::geometry_collection_engine::__FUNCTION_PTRS
                    .u_geometry_collection_component_set_notify_breaks,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_new_notify_breaks,
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
                crate::bindings::geometry_collection_engine::__FUNCTION_PTRS
                    .u_geometry_collection_component_set_notify_breaks,
                __buffer,
            )
        };
    }
    pub fn set_local_rest_transforms(
        &mut self,
        transforms: &TArray<crate::bindings::core_u_object::FTransform>,
        b_only_leaves: bool,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<17>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::geometry_collection_engine::__FUNCTION_PTRS
                    .u_geometry_collection_component_set_local_rest_transforms,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                transforms,
                __buffer
                    .add(0)
                    .cast::<TArray<crate::bindings::core_u_object::FTransform>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_only_leaves,
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
                crate::bindings::geometry_collection_engine::__FUNCTION_PTRS
                    .u_geometry_collection_component_set_local_rest_transforms,
                __buffer,
            )
        };
    }
    pub fn set_gravity_group_index(&mut self, in_gravity_group_index: i32) {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::geometry_collection_engine::__FUNCTION_PTRS
                    .u_geometry_collection_component_set_gravity_group_index,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_gravity_group_index,
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
                crate::bindings::geometry_collection_engine::__FUNCTION_PTRS
                    .u_geometry_collection_component_set_gravity_group_index,
                __buffer,
            )
        };
    }
    pub fn set_enable_damage_from_collision(&mut self, b_value: bool) {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::geometry_collection_engine::__FUNCTION_PTRS
                    .u_geometry_collection_component_set_enable_damage_from_collision,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&b_value, __buffer.add(0).cast::<bool>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::geometry_collection_engine::__FUNCTION_PTRS
                    .u_geometry_collection_component_set_enable_damage_from_collision,
                __buffer,
            )
        };
    }
    pub fn set_density_from_physics_material(
        &mut self,
        b_in_density_from_physics_material: bool,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::geometry_collection_engine::__FUNCTION_PTRS
                    .u_geometry_collection_component_set_density_from_physics_material,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_in_density_from_physics_material,
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
                crate::bindings::geometry_collection_engine::__FUNCTION_PTRS
                    .u_geometry_collection_component_set_density_from_physics_material,
                __buffer,
            )
        };
    }
    pub fn set_damage_threshold(&mut self, in_damage_threshold: &TArray<f32>) {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::geometry_collection_engine::__FUNCTION_PTRS
                    .u_geometry_collection_component_set_damage_threshold,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_damage_threshold,
                __buffer.add(0).cast::<TArray<f32>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::geometry_collection_engine::__FUNCTION_PTRS
                    .u_geometry_collection_component_set_damage_threshold,
                __buffer,
            )
        };
    }
    pub fn set_damage_propagation_data(
        &mut self,
        in_damage_propagation_data: &FGeometryCollectionDamagePropagationData,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<12>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::geometry_collection_engine::__FUNCTION_PTRS
                    .u_geometry_collection_component_set_damage_propagation_data,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_damage_propagation_data,
                __buffer.add(0).cast::<FGeometryCollectionDamagePropagationData>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::geometry_collection_engine::__FUNCTION_PTRS
                    .u_geometry_collection_component_set_damage_propagation_data,
                __buffer,
            )
        };
    }
    pub fn set_damage_model(
        &mut self,
        in_damage_model: crate::bindings::chaos::EDamageModelTypeEnum,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::geometry_collection_engine::__FUNCTION_PTRS
                    .u_geometry_collection_component_set_damage_model,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_damage_model,
                __buffer.add(0).cast::<crate::bindings::chaos::EDamageModelTypeEnum>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::geometry_collection_engine::__FUNCTION_PTRS
                    .u_geometry_collection_component_set_damage_model,
                __buffer,
            )
        };
    }
    pub fn set_anchored_by_transformed_box(
        &mut self,
        box_: crate::bindings::core_u_object::FBox,
        transform: crate::bindings::core_u_object::FTransform,
        b_anchored: bool,
        max_level: i32,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<168>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::geometry_collection_engine::__FUNCTION_PTRS
                    .u_geometry_collection_component_set_anchored_by_transformed_box,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &box_,
                __buffer.add(0).cast::<crate::bindings::core_u_object::FBox>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &transform,
                __buffer.add(64).cast::<crate::bindings::core_u_object::FTransform>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_anchored,
                __buffer.add(160).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &max_level,
                __buffer.add(164).cast::<i32>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::geometry_collection_engine::__FUNCTION_PTRS
                    .u_geometry_collection_component_set_anchored_by_transformed_box,
                __buffer,
            )
        };
    }
    pub fn set_anchored_by_index(&mut self, index: i32, b_anchored: bool) {
        let mut __stack = crate::core_data::StackAlloc::<5>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::geometry_collection_engine::__FUNCTION_PTRS
                    .u_geometry_collection_component_set_anchored_by_index,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&index, __buffer.add(0).cast::<i32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_anchored,
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
                crate::bindings::geometry_collection_engine::__FUNCTION_PTRS
                    .u_geometry_collection_component_set_anchored_by_index,
                __buffer,
            )
        };
    }
    pub fn set_anchored_by_box(
        &mut self,
        world_space_box: crate::bindings::core_u_object::FBox,
        b_anchored: bool,
        max_level: i32,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<64>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::geometry_collection_engine::__FUNCTION_PTRS
                    .u_geometry_collection_component_set_anchored_by_box,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &world_space_box,
                __buffer.add(0).cast::<crate::bindings::core_u_object::FBox>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_anchored,
                __buffer.add(56).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&max_level, __buffer.add(60).cast::<i32>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::geometry_collection_engine::__FUNCTION_PTRS
                    .u_geometry_collection_component_set_anchored_by_box,
                __buffer,
            )
        };
    }
    pub fn set_abandoned_particle_collision_profile_name(
        &mut self,
        collision_profile: FName,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<12>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::geometry_collection_engine::__FUNCTION_PTRS
                    .u_geometry_collection_component_set_abandoned_particle_collision_profile_name,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &collision_profile,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::geometry_collection_engine::__FUNCTION_PTRS
                    .u_geometry_collection_component_set_abandoned_particle_collision_profile_name,
                __buffer,
            )
        };
    }
    pub fn reset_state(&mut self) {
        let mut __stack = crate::core_data::StackAlloc::<0>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::geometry_collection_engine::__FUNCTION_PTRS
                    .u_geometry_collection_component_reset_state,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::geometry_collection_engine::__FUNCTION_PTRS
                    .u_geometry_collection_component_reset_state,
                __buffer,
            )
        };
    }
    pub fn remove_all_anchors(&mut self) {
        let mut __stack = crate::core_data::StackAlloc::<0>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::geometry_collection_engine::__FUNCTION_PTRS
                    .u_geometry_collection_component_remove_all_anchors,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::geometry_collection_engine::__FUNCTION_PTRS
                    .u_geometry_collection_component_remove_all_anchors,
                __buffer,
            )
        };
    }
    pub fn receive_physics_collision(
        &mut self,
        collision_info: &crate::bindings::chaos_solver_engine::FChaosPhysicsCollisionInfo,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<192>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::geometry_collection_engine::__FUNCTION_PTRS
                    .u_geometry_collection_component_receive_physics_collision,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                collision_info,
                __buffer
                    .add(0)
                    .cast::<
                        crate::bindings::chaos_solver_engine::FChaosPhysicsCollisionInfo,
                    >(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::geometry_collection_engine::__FUNCTION_PTRS
                    .u_geometry_collection_component_receive_physics_collision,
                __buffer,
            )
        };
    }
    pub fn is_root_broken(&self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::geometry_collection_engine::__FUNCTION_PTRS
                    .u_geometry_collection_component_is_root_broken,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::geometry_collection_engine::__FUNCTION_PTRS
                    .u_geometry_collection_component_is_root_broken,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn get_use_static_mesh_collision_for_traces(&self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::geometry_collection_engine::__FUNCTION_PTRS
                    .u_geometry_collection_component_get_use_static_mesh_collision_for_traces,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::geometry_collection_engine::__FUNCTION_PTRS
                    .u_geometry_collection_component_get_use_static_mesh_collision_for_traces,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn get_solver_actor(
        &self,
    ) -> UPtr<crate::bindings::chaos_solver_engine::AChaosSolverActor> {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::geometry_collection_engine::__FUNCTION_PTRS
                    .u_geometry_collection_component_get_solver_actor,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::geometry_collection_engine::__FUNCTION_PTRS
                    .u_geometry_collection_component_get_solver_actor,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(0)
                .cast::<UPtr<crate::bindings::chaos_solver_engine::AChaosSolverActor>>()
                .read()
        }
    }
    pub fn get_root_initial_transform(
        &self,
    ) -> crate::bindings::core_u_object::FTransform {
        let mut __stack = crate::core_data::StackAlloc::<96>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::geometry_collection_engine::__FUNCTION_PTRS
                    .u_geometry_collection_component_get_root_initial_transform,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::geometry_collection_engine::__FUNCTION_PTRS
                    .u_geometry_collection_component_get_root_initial_transform,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<crate::bindings::core_u_object::FTransform>().read()
        }
    }
    pub fn get_root_index(&self) -> i32 {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::geometry_collection_engine::__FUNCTION_PTRS
                    .u_geometry_collection_component_get_root_index,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::geometry_collection_engine::__FUNCTION_PTRS
                    .u_geometry_collection_component_get_root_index,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<i32>().read() }
    }
    pub fn get_root_current_transform(
        &self,
    ) -> crate::bindings::core_u_object::FTransform {
        let mut __stack = crate::core_data::StackAlloc::<96>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::geometry_collection_engine::__FUNCTION_PTRS
                    .u_geometry_collection_component_get_root_current_transform,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::geometry_collection_engine::__FUNCTION_PTRS
                    .u_geometry_collection_component_get_root_current_transform,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<crate::bindings::core_u_object::FTransform>().read()
        }
    }
    pub fn get_mass_and_extents(
        &mut self,
        item_index: i32,
        out_mass: &mut f32,
        out_extents: &mut crate::bindings::core_u_object::FBox,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<64>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::geometry_collection_engine::__FUNCTION_PTRS
                    .u_geometry_collection_component_get_mass_and_extents,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&item_index, __buffer.add(0).cast::<i32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(out_mass, __buffer.add(4).cast::<f32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_extents,
                __buffer.add(8).cast::<crate::bindings::core_u_object::FBox>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::geometry_collection_engine::__FUNCTION_PTRS
                    .u_geometry_collection_component_get_mass_and_extents,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(4).cast::<f32>().swap(out_mass);
        }
        unsafe {
            __buffer
                .add(8)
                .cast::<crate::bindings::core_u_object::FBox>()
                .swap(out_extents);
        }
    }
    pub fn get_local_rest_transforms(
        &self,
        b_initial_transforms: bool,
    ) -> TArray<crate::bindings::core_u_object::FTransform> {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::geometry_collection_engine::__FUNCTION_PTRS
                    .u_geometry_collection_component_get_local_rest_transforms,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_initial_transforms,
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
                crate::bindings::geometry_collection_engine::__FUNCTION_PTRS
                    .u_geometry_collection_component_get_local_rest_transforms,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(8)
                .cast::<TArray<crate::bindings::core_u_object::FTransform>>()
                .read()
        }
    }
    pub fn get_local_bounds(&self) -> crate::bindings::core_u_object::FBox {
        let mut __stack = crate::core_data::StackAlloc::<56>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::geometry_collection_engine::__FUNCTION_PTRS
                    .u_geometry_collection_component_get_local_bounds,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::geometry_collection_engine::__FUNCTION_PTRS
                    .u_geometry_collection_component_get_local_bounds,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<crate::bindings::core_u_object::FBox>().read() }
    }
    pub fn get_initial_local_rest_transforms(
        &self,
    ) -> TArray<crate::bindings::core_u_object::FTransform> {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::geometry_collection_engine::__FUNCTION_PTRS
                    .u_geometry_collection_component_get_initial_local_rest_transforms,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::geometry_collection_engine::__FUNCTION_PTRS
                    .u_geometry_collection_component_get_initial_local_rest_transforms,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(0)
                .cast::<TArray<crate::bindings::core_u_object::FTransform>>()
                .read()
        }
    }
    pub fn get_initial_level(&mut self, item_index: i32) -> i32 {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::geometry_collection_engine::__FUNCTION_PTRS
                    .u_geometry_collection_component_get_initial_level,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&item_index, __buffer.add(0).cast::<i32>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::geometry_collection_engine::__FUNCTION_PTRS
                    .u_geometry_collection_component_get_initial_level,
                __buffer,
            )
        };
        unsafe { __buffer.add(4).cast::<i32>().read() }
    }
    pub fn get_debug_info(&mut self) -> FString {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::geometry_collection_engine::__FUNCTION_PTRS
                    .u_geometry_collection_component_get_debug_info,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::geometry_collection_engine::__FUNCTION_PTRS
                    .u_geometry_collection_component_get_debug_info,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<FString>().read() }
    }
    pub fn get_damage_threshold(&self) -> TArray<f32> {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::geometry_collection_engine::__FUNCTION_PTRS
                    .u_geometry_collection_component_get_damage_threshold,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::geometry_collection_engine::__FUNCTION_PTRS
                    .u_geometry_collection_component_get_damage_threshold,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<TArray<f32>>().read() }
    }
    pub fn force_broken_for_custom_renderer(&mut self, b_force_broken: bool) {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::geometry_collection_engine::__FUNCTION_PTRS
                    .u_geometry_collection_component_force_broken_for_custom_renderer,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_force_broken,
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
                crate::bindings::geometry_collection_engine::__FUNCTION_PTRS
                    .u_geometry_collection_component_force_broken_for_custom_renderer,
                __buffer,
            )
        };
    }
    pub fn find_leaf_transform_by_line_trace(
        &self,
        start: crate::bindings::core_u_object::FVector,
        end: crate::bindings::core_u_object::FVector,
        leaf_transform_index: &mut i32,
        leaf_transform_name: &mut FName,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<65>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::geometry_collection_engine::__FUNCTION_PTRS
                    .u_geometry_collection_component_find_leaf_transform_by_line_trace,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &start,
                __buffer.add(0).cast::<crate::bindings::core_u_object::FVector>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &end,
                __buffer.add(24).cast::<crate::bindings::core_u_object::FVector>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                leaf_transform_index,
                __buffer.add(48).cast::<i32>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                leaf_transform_name,
                __buffer.add(52).cast::<FName>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::geometry_collection_engine::__FUNCTION_PTRS
                    .u_geometry_collection_component_find_leaf_transform_by_line_trace,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(48).cast::<i32>().swap(leaf_transform_index);
        }
        unsafe {
            __buffer.add(52).cast::<FName>().swap(leaf_transform_name);
        }
        unsafe { __buffer.add(64).cast::<bool>().read() }
    }
    pub fn enable_root_proxy_for_custom_renderer(&mut self, b_enable: bool) {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::geometry_collection_engine::__FUNCTION_PTRS
                    .u_geometry_collection_component_enable_root_proxy_for_custom_renderer,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&b_enable, __buffer.add(0).cast::<bool>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::geometry_collection_engine::__FUNCTION_PTRS
                    .u_geometry_collection_component_enable_root_proxy_for_custom_renderer,
                __buffer,
            )
        };
    }
    pub fn crumble_cluster(&mut self, item_index: i32) {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::geometry_collection_engine::__FUNCTION_PTRS
                    .u_geometry_collection_component_crumble_cluster,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&item_index, __buffer.add(0).cast::<i32>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::geometry_collection_engine::__FUNCTION_PTRS
                    .u_geometry_collection_component_crumble_cluster,
                __buffer,
            )
        };
    }
    pub fn crumble_active_clusters(&mut self) {
        let mut __stack = crate::core_data::StackAlloc::<0>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::geometry_collection_engine::__FUNCTION_PTRS
                    .u_geometry_collection_component_crumble_active_clusters,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::geometry_collection_engine::__FUNCTION_PTRS
                    .u_geometry_collection_component_crumble_active_clusters,
                __buffer,
            )
        };
    }
    pub fn apply_physics_field(
        &mut self,
        enabled: bool,
        target: crate::bindings::chaos::EGeometryCollectionPhysicsTypeEnum,
        meta_data: UPtr<crate::bindings::field_system_engine::UFieldSystemMetaData>,
        field: UPtr<crate::bindings::field_system_engine::UFieldNodeBase>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::geometry_collection_engine::__FUNCTION_PTRS
                    .u_geometry_collection_component_apply_physics_field,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&enabled, __buffer.add(0).cast::<bool>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &target,
                __buffer
                    .add(1)
                    .cast::<
                        crate::bindings::chaos::EGeometryCollectionPhysicsTypeEnum,
                    >(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &meta_data,
                __buffer
                    .add(8)
                    .cast::<
                        UPtr<crate::bindings::field_system_engine::UFieldSystemMetaData>,
                    >(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &field,
                __buffer
                    .add(16)
                    .cast::<
                        UPtr<crate::bindings::field_system_engine::UFieldNodeBase>,
                    >(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::geometry_collection_engine::__FUNCTION_PTRS
                    .u_geometry_collection_component_apply_physics_field,
                __buffer,
            )
        };
    }
    pub fn apply_linear_velocity(
        &mut self,
        item_index: i32,
        linear_velocity: &crate::bindings::core_u_object::FVector,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::geometry_collection_engine::__FUNCTION_PTRS
                    .u_geometry_collection_component_apply_linear_velocity,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&item_index, __buffer.add(0).cast::<i32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                linear_velocity,
                __buffer.add(8).cast::<crate::bindings::core_u_object::FVector>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::geometry_collection_engine::__FUNCTION_PTRS
                    .u_geometry_collection_component_apply_linear_velocity,
                __buffer,
            )
        };
    }
    pub fn apply_kinematic_field(
        &mut self,
        radius: f32,
        position: crate::bindings::core_u_object::FVector,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::geometry_collection_engine::__FUNCTION_PTRS
                    .u_geometry_collection_component_apply_kinematic_field,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&radius, __buffer.add(0).cast::<f32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &position,
                __buffer.add(8).cast::<crate::bindings::core_u_object::FVector>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::geometry_collection_engine::__FUNCTION_PTRS
                    .u_geometry_collection_component_apply_kinematic_field,
                __buffer,
            )
        };
    }
    pub fn apply_internal_strain(
        &mut self,
        item_index: i32,
        location: &crate::bindings::core_u_object::FVector,
        radius: f32,
        propagation_depth: i32,
        propagation_factor: f32,
        strain: f32,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<48>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::geometry_collection_engine::__FUNCTION_PTRS
                    .u_geometry_collection_component_apply_internal_strain,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&item_index, __buffer.add(0).cast::<i32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                location,
                __buffer.add(8).cast::<crate::bindings::core_u_object::FVector>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&radius, __buffer.add(32).cast::<f32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &propagation_depth,
                __buffer.add(36).cast::<i32>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &propagation_factor,
                __buffer.add(40).cast::<f32>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&strain, __buffer.add(44).cast::<f32>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::geometry_collection_engine::__FUNCTION_PTRS
                    .u_geometry_collection_component_apply_internal_strain,
                __buffer,
            )
        };
    }
    pub fn apply_external_strain(
        &mut self,
        item_index: i32,
        location: &crate::bindings::core_u_object::FVector,
        radius: f32,
        propagation_depth: i32,
        propagation_factor: f32,
        strain: f32,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<48>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::geometry_collection_engine::__FUNCTION_PTRS
                    .u_geometry_collection_component_apply_external_strain,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&item_index, __buffer.add(0).cast::<i32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                location,
                __buffer.add(8).cast::<crate::bindings::core_u_object::FVector>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&radius, __buffer.add(32).cast::<f32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &propagation_depth,
                __buffer.add(36).cast::<i32>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &propagation_factor,
                __buffer.add(40).cast::<f32>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&strain, __buffer.add(44).cast::<f32>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::geometry_collection_engine::__FUNCTION_PTRS
                    .u_geometry_collection_component_apply_external_strain,
                __buffer,
            )
        };
    }
    pub fn apply_breaking_linear_velocity(
        &mut self,
        item_index: i32,
        linear_velocity: &crate::bindings::core_u_object::FVector,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::geometry_collection_engine::__FUNCTION_PTRS
                    .u_geometry_collection_component_apply_breaking_linear_velocity,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&item_index, __buffer.add(0).cast::<i32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                linear_velocity,
                __buffer.add(8).cast::<crate::bindings::core_u_object::FVector>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::geometry_collection_engine::__FUNCTION_PTRS
                    .u_geometry_collection_component_apply_breaking_linear_velocity,
                __buffer,
            )
        };
    }
    pub fn apply_breaking_angular_velocity(
        &mut self,
        item_index: i32,
        angular_velocity: &crate::bindings::core_u_object::FVector,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::geometry_collection_engine::__FUNCTION_PTRS
                    .u_geometry_collection_component_apply_breaking_angular_velocity,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&item_index, __buffer.add(0).cast::<i32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                angular_velocity,
                __buffer.add(8).cast::<crate::bindings::core_u_object::FVector>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::geometry_collection_engine::__FUNCTION_PTRS
                    .u_geometry_collection_component_apply_breaking_angular_velocity,
                __buffer,
            )
        };
    }
    pub fn apply_asset_defaults(&mut self) {
        let mut __stack = crate::core_data::StackAlloc::<0>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::geometry_collection_engine::__FUNCTION_PTRS
                    .u_geometry_collection_component_apply_asset_defaults,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::geometry_collection_engine::__FUNCTION_PTRS
                    .u_geometry_collection_component_apply_asset_defaults,
                __buffer,
            )
        };
    }
    pub fn apply_angular_velocity(
        &mut self,
        item_index: i32,
        angular_velocity: &crate::bindings::core_u_object::FVector,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::geometry_collection_engine::__FUNCTION_PTRS
                    .u_geometry_collection_component_apply_angular_velocity,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&item_index, __buffer.add(0).cast::<i32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                angular_velocity,
                __buffer.add(8).cast::<crate::bindings::core_u_object::FVector>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::geometry_collection_engine::__FUNCTION_PTRS
                    .u_geometry_collection_component_apply_angular_velocity,
                __buffer,
            )
        };
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
    pub fn set_enable_nanite(&mut self, b_value: bool) {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::geometry_collection_engine::__FUNCTION_PTRS
                    .u_geometry_collection_set_enable_nanite,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&b_value, __buffer.add(0).cast::<bool>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::geometry_collection_engine::__FUNCTION_PTRS
                    .u_geometry_collection_set_enable_nanite,
                __buffer,
            )
        };
    }
    pub fn set_dataflow_asset(
        &mut self,
        in_dataflow_asset: UPtr<crate::bindings::dataflow_engine::UDataflow>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::geometry_collection_engine::__FUNCTION_PTRS
                    .u_geometry_collection_set_dataflow_asset,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_dataflow_asset,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::dataflow_engine::UDataflow>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::geometry_collection_engine::__FUNCTION_PTRS
                    .u_geometry_collection_set_dataflow_asset,
                __buffer,
            )
        };
    }
    pub fn set_convert_vertex_colors_to_srgb(&mut self, b_value: bool) {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::geometry_collection_engine::__FUNCTION_PTRS
                    .u_geometry_collection_set_convert_vertex_colors_to_srgb,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&b_value, __buffer.add(0).cast::<bool>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::geometry_collection_engine::__FUNCTION_PTRS
                    .u_geometry_collection_set_convert_vertex_colors_to_srgb,
                __buffer,
            )
        };
    }
    pub fn get_dataflow_asset(
        &self,
    ) -> UPtr<crate::bindings::dataflow_engine::UDataflow> {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::geometry_collection_engine::__FUNCTION_PTRS
                    .u_geometry_collection_get_dataflow_asset,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::geometry_collection_engine::__FUNCTION_PTRS
                    .u_geometry_collection_get_dataflow_asset,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(0)
                .cast::<UPtr<crate::bindings::dataflow_engine::UDataflow>>()
                .read()
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
