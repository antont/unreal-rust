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
    pub u_buoyancy_component_on_pontoon_exited_water: *mut crate::ffi::UFunctionOpague,
    pub u_buoyancy_component_on_pontoon_entered_water: *mut crate::ffi::UFunctionOpague,
    pub u_buoyancy_component_is_overlapping_water_body: *mut crate::ffi::UFunctionOpague,
    pub u_buoyancy_component_is_in_water_body: *mut crate::ffi::UFunctionOpague,
    pub u_buoyancy_component_get_last_water_surface_info: *mut crate::ffi::UFunctionOpague,
    pub u_buoyancy_component_get_current_water_body_components: *mut crate::ffi::UFunctionOpague,
    pub a_buoyancy_manager_get_buoyancy_component_manager: *mut crate::ffi::UFunctionOpague,
    pub u_gerstner_water_wave_generator_base_generate_gerstner_waves: *mut crate::ffi::UFunctionOpague,
    pub u_niagara_water_function_library_set_water_body_component: *mut crate::ffi::UFunctionOpague,
    pub u_niagara_water_function_library_set_water_body: *mut crate::ffi::UFunctionOpague,
    pub a_water_body_set_water_waves: *mut crate::ffi::UFunctionOpague,
    pub a_water_body_set_water_material: *mut crate::ffi::UFunctionOpague,
    pub a_water_body_on_water_body_changed: *mut crate::ffi::UFunctionOpague,
    pub a_water_body_get_water_velocity_vector_at_spline_input_key: *mut crate::ffi::UFunctionOpague,
    pub a_water_body_get_water_velocity_at_spline_input_key: *mut crate::ffi::UFunctionOpague,
    pub a_water_body_get_water_spline: *mut crate::ffi::UFunctionOpague,
    pub a_water_body_get_water_material_instance: *mut crate::ffi::UFunctionOpague,
    pub a_water_body_get_water_body_type: *mut crate::ffi::UFunctionOpague,
    pub a_water_body_get_water_body_component: *mut crate::ffi::UFunctionOpague,
    pub a_water_body_get_river_to_ocean_transition_material_instance: *mut crate::ffi::UFunctionOpague,
    pub a_water_body_get_river_to_lake_transition_material_instance: *mut crate::ffi::UFunctionOpague,
    pub a_water_body_get_islands: *mut crate::ffi::UFunctionOpague,
    pub a_water_body_get_exclusion_volumes: *mut crate::ffi::UFunctionOpague,
    pub a_water_body_get_audio_intensity_at_spline_input_key: *mut crate::ffi::UFunctionOpague,
    pub u_water_body_component_set_water_zone_override: *mut crate::ffi::UFunctionOpague,
    pub u_water_body_component_set_water_velocity_at_spline_input_key: *mut crate::ffi::UFunctionOpague,
    pub u_water_body_component_set_water_static_mesh_material: *mut crate::ffi::UFunctionOpague,
    pub u_water_body_component_set_water_material: *mut crate::ffi::UFunctionOpague,
    pub u_water_body_component_set_water_info_material: *mut crate::ffi::UFunctionOpague,
    pub u_water_body_component_set_water_body_static_mesh_enabled: *mut crate::ffi::UFunctionOpague,
    pub u_water_body_component_set_water_and_under_water_post_process_material: *mut crate::ffi::UFunctionOpague,
    pub u_water_body_component_set_underwater_post_process_material: *mut crate::ffi::UFunctionOpague,
    pub u_water_body_component_set_audio_intensity_at_spline_input_key: *mut crate::ffi::UFunctionOpague,
    pub u_water_body_component_on_water_body_changed: *mut crate::ffi::UFunctionOpague,
    pub u_water_body_component_get_water_waves: *mut crate::ffi::UFunctionOpague,
    pub u_water_body_component_get_water_velocity_vector_at_spline_input_key: *mut crate::ffi::UFunctionOpague,
    pub u_water_body_component_get_water_velocity_at_spline_input_key: *mut crate::ffi::UFunctionOpague,
    pub u_water_body_component_get_water_surface_info_at_location: *mut crate::ffi::UFunctionOpague,
    pub u_water_body_component_get_water_static_mesh_material_instance: *mut crate::ffi::UFunctionOpague,
    pub u_water_body_component_get_water_spline: *mut crate::ffi::UFunctionOpague,
    pub u_water_body_component_get_water_material_instance: *mut crate::ffi::UFunctionOpague,
    pub u_water_body_component_get_water_material: *mut crate::ffi::UFunctionOpague,
    pub u_water_body_component_get_water_lod_material_instance: *mut crate::ffi::UFunctionOpague,
    pub u_water_body_component_get_water_info_material_instance: *mut crate::ffi::UFunctionOpague,
    pub u_water_body_component_get_water_body_actor: *mut crate::ffi::UFunctionOpague,
    pub u_water_body_component_get_underwater_post_process_material_instance: *mut crate::ffi::UFunctionOpague,
    pub u_water_body_component_get_standard_renderable_components: *mut crate::ffi::UFunctionOpague,
    pub u_water_body_component_get_river_to_ocean_transition_material_instance: *mut crate::ffi::UFunctionOpague,
    pub u_water_body_component_get_river_to_ocean_transition_material: *mut crate::ffi::UFunctionOpague,
    pub u_water_body_component_get_river_to_lake_transition_material_instance: *mut crate::ffi::UFunctionOpague,
    pub u_water_body_component_get_river_to_lake_transition_material: *mut crate::ffi::UFunctionOpague,
    pub u_water_body_component_get_max_wave_height: *mut crate::ffi::UFunctionOpague,
    pub u_water_body_component_get_islands: *mut crate::ffi::UFunctionOpague,
    pub u_water_body_component_get_exclusion_volumes: *mut crate::ffi::UFunctionOpague,
    pub u_water_body_component_get_collision_components: *mut crate::ffi::UFunctionOpague,
    pub u_water_body_component_get_audio_intensity_at_spline_input_key: *mut crate::ffi::UFunctionOpague,
    pub a_water_body_island_get_water_spline: *mut crate::ffi::UFunctionOpague,
    pub u_water_body_ocean_component_fill_water_zone_with_ocean: *mut crate::ffi::UFunctionOpague,
    pub u_water_body_river_component_set_river_width_at_spline_input_key: *mut crate::ffi::UFunctionOpague,
    pub u_water_body_river_component_set_river_depth_at_spline_input_key: *mut crate::ffi::UFunctionOpague,
    pub u_water_body_river_component_set_ocean_transition_material: *mut crate::ffi::UFunctionOpague,
    pub u_water_body_river_component_set_lake_transition_material: *mut crate::ffi::UFunctionOpague,
    pub u_water_body_river_component_set_lake_and_ocean_transition_materials: *mut crate::ffi::UFunctionOpague,
    pub u_water_body_river_component_get_river_width_at_spline_input_key: *mut crate::ffi::UFunctionOpague,
    pub u_water_body_river_component_get_river_depth_at_spline_input_key: *mut crate::ffi::UFunctionOpague,
    pub u_water_mesh_component_is_enabled: *mut crate::ffi::UFunctionOpague,
    pub u_water_spline_component_k2_synchronize_and_broadcast_data_change: *mut crate::ffi::UFunctionOpague,
    pub u_water_subsystem_set_ocean_flood_height: *mut crate::ffi::UFunctionOpague,
    pub u_water_subsystem_print_to_water_log: *mut crate::ffi::UFunctionOpague,
    pub u_water_subsystem_is_water_rendering_enabled: *mut crate::ffi::UFunctionOpague,
    pub u_water_subsystem_is_underwater_post_process_enabled: *mut crate::ffi::UFunctionOpague,
    pub u_water_subsystem_is_shallow_water_simulation_enabled: *mut crate::ffi::UFunctionOpague,
    pub u_water_subsystem_get_water_time_seconds: *mut crate::ffi::UFunctionOpague,
    pub u_water_subsystem_get_underwater_precise_trace_distance: *mut crate::ffi::UFunctionOpague,
    pub u_water_subsystem_get_underwater_collision_trace_distance: *mut crate::ffi::UFunctionOpague,
    pub u_water_subsystem_get_smoothed_world_time_seconds: *mut crate::ffi::UFunctionOpague,
    pub u_water_subsystem_get_shallow_water_simulation_render_target_size: *mut crate::ffi::UFunctionOpague,
    pub u_water_subsystem_get_shallow_water_max_impulse_forces: *mut crate::ffi::UFunctionOpague,
    pub u_water_subsystem_get_shallow_water_max_dynamic_forces: *mut crate::ffi::UFunctionOpague,
    pub u_water_subsystem_get_ocean_total_height: *mut crate::ffi::UFunctionOpague,
    pub u_water_subsystem_get_ocean_flood_height: *mut crate::ffi::UFunctionOpague,
    pub u_water_subsystem_get_ocean_base_height: *mut crate::ffi::UFunctionOpague,
    pub u_water_subsystem_get_camera_underwater_depth: *mut crate::ffi::UFunctionOpague,
    pub a_water_zone_set_far_mesh_material: *mut crate::ffi::UFunctionOpague,
    pub a_water_zone_get_water_zone_index: *mut crate::ffi::UFunctionOpague,
    pub a_water_zone_force_update_water_info_texture: *mut crate::ffi::UFunctionOpague,
}
impl FunctionPtrs {
    pub const fn empty() -> Self {
        Self {
            u_buoyancy_component_on_pontoon_exited_water: std::ptr::null_mut(),
            u_buoyancy_component_on_pontoon_entered_water: std::ptr::null_mut(),
            u_buoyancy_component_is_overlapping_water_body: std::ptr::null_mut(),
            u_buoyancy_component_is_in_water_body: std::ptr::null_mut(),
            u_buoyancy_component_get_last_water_surface_info: std::ptr::null_mut(),
            u_buoyancy_component_get_current_water_body_components: std::ptr::null_mut(),
            a_buoyancy_manager_get_buoyancy_component_manager: std::ptr::null_mut(),
            u_gerstner_water_wave_generator_base_generate_gerstner_waves: std::ptr::null_mut(),
            u_niagara_water_function_library_set_water_body_component: std::ptr::null_mut(),
            u_niagara_water_function_library_set_water_body: std::ptr::null_mut(),
            a_water_body_set_water_waves: std::ptr::null_mut(),
            a_water_body_set_water_material: std::ptr::null_mut(),
            a_water_body_on_water_body_changed: std::ptr::null_mut(),
            a_water_body_get_water_velocity_vector_at_spline_input_key: std::ptr::null_mut(),
            a_water_body_get_water_velocity_at_spline_input_key: std::ptr::null_mut(),
            a_water_body_get_water_spline: std::ptr::null_mut(),
            a_water_body_get_water_material_instance: std::ptr::null_mut(),
            a_water_body_get_water_body_type: std::ptr::null_mut(),
            a_water_body_get_water_body_component: std::ptr::null_mut(),
            a_water_body_get_river_to_ocean_transition_material_instance: std::ptr::null_mut(),
            a_water_body_get_river_to_lake_transition_material_instance: std::ptr::null_mut(),
            a_water_body_get_islands: std::ptr::null_mut(),
            a_water_body_get_exclusion_volumes: std::ptr::null_mut(),
            a_water_body_get_audio_intensity_at_spline_input_key: std::ptr::null_mut(),
            u_water_body_component_set_water_zone_override: std::ptr::null_mut(),
            u_water_body_component_set_water_velocity_at_spline_input_key: std::ptr::null_mut(),
            u_water_body_component_set_water_static_mesh_material: std::ptr::null_mut(),
            u_water_body_component_set_water_material: std::ptr::null_mut(),
            u_water_body_component_set_water_info_material: std::ptr::null_mut(),
            u_water_body_component_set_water_body_static_mesh_enabled: std::ptr::null_mut(),
            u_water_body_component_set_water_and_under_water_post_process_material: std::ptr::null_mut(),
            u_water_body_component_set_underwater_post_process_material: std::ptr::null_mut(),
            u_water_body_component_set_audio_intensity_at_spline_input_key: std::ptr::null_mut(),
            u_water_body_component_on_water_body_changed: std::ptr::null_mut(),
            u_water_body_component_get_water_waves: std::ptr::null_mut(),
            u_water_body_component_get_water_velocity_vector_at_spline_input_key: std::ptr::null_mut(),
            u_water_body_component_get_water_velocity_at_spline_input_key: std::ptr::null_mut(),
            u_water_body_component_get_water_surface_info_at_location: std::ptr::null_mut(),
            u_water_body_component_get_water_static_mesh_material_instance: std::ptr::null_mut(),
            u_water_body_component_get_water_spline: std::ptr::null_mut(),
            u_water_body_component_get_water_material_instance: std::ptr::null_mut(),
            u_water_body_component_get_water_material: std::ptr::null_mut(),
            u_water_body_component_get_water_lod_material_instance: std::ptr::null_mut(),
            u_water_body_component_get_water_info_material_instance: std::ptr::null_mut(),
            u_water_body_component_get_water_body_actor: std::ptr::null_mut(),
            u_water_body_component_get_underwater_post_process_material_instance: std::ptr::null_mut(),
            u_water_body_component_get_standard_renderable_components: std::ptr::null_mut(),
            u_water_body_component_get_river_to_ocean_transition_material_instance: std::ptr::null_mut(),
            u_water_body_component_get_river_to_ocean_transition_material: std::ptr::null_mut(),
            u_water_body_component_get_river_to_lake_transition_material_instance: std::ptr::null_mut(),
            u_water_body_component_get_river_to_lake_transition_material: std::ptr::null_mut(),
            u_water_body_component_get_max_wave_height: std::ptr::null_mut(),
            u_water_body_component_get_islands: std::ptr::null_mut(),
            u_water_body_component_get_exclusion_volumes: std::ptr::null_mut(),
            u_water_body_component_get_collision_components: std::ptr::null_mut(),
            u_water_body_component_get_audio_intensity_at_spline_input_key: std::ptr::null_mut(),
            a_water_body_island_get_water_spline: std::ptr::null_mut(),
            u_water_body_ocean_component_fill_water_zone_with_ocean: std::ptr::null_mut(),
            u_water_body_river_component_set_river_width_at_spline_input_key: std::ptr::null_mut(),
            u_water_body_river_component_set_river_depth_at_spline_input_key: std::ptr::null_mut(),
            u_water_body_river_component_set_ocean_transition_material: std::ptr::null_mut(),
            u_water_body_river_component_set_lake_transition_material: std::ptr::null_mut(),
            u_water_body_river_component_set_lake_and_ocean_transition_materials: std::ptr::null_mut(),
            u_water_body_river_component_get_river_width_at_spline_input_key: std::ptr::null_mut(),
            u_water_body_river_component_get_river_depth_at_spline_input_key: std::ptr::null_mut(),
            u_water_mesh_component_is_enabled: std::ptr::null_mut(),
            u_water_spline_component_k2_synchronize_and_broadcast_data_change: std::ptr::null_mut(),
            u_water_subsystem_set_ocean_flood_height: std::ptr::null_mut(),
            u_water_subsystem_print_to_water_log: std::ptr::null_mut(),
            u_water_subsystem_is_water_rendering_enabled: std::ptr::null_mut(),
            u_water_subsystem_is_underwater_post_process_enabled: std::ptr::null_mut(),
            u_water_subsystem_is_shallow_water_simulation_enabled: std::ptr::null_mut(),
            u_water_subsystem_get_water_time_seconds: std::ptr::null_mut(),
            u_water_subsystem_get_underwater_precise_trace_distance: std::ptr::null_mut(),
            u_water_subsystem_get_underwater_collision_trace_distance: std::ptr::null_mut(),
            u_water_subsystem_get_smoothed_world_time_seconds: std::ptr::null_mut(),
            u_water_subsystem_get_shallow_water_simulation_render_target_size: std::ptr::null_mut(),
            u_water_subsystem_get_shallow_water_max_impulse_forces: std::ptr::null_mut(),
            u_water_subsystem_get_shallow_water_max_dynamic_forces: std::ptr::null_mut(),
            u_water_subsystem_get_ocean_total_height: std::ptr::null_mut(),
            u_water_subsystem_get_ocean_flood_height: std::ptr::null_mut(),
            u_water_subsystem_get_ocean_base_height: std::ptr::null_mut(),
            u_water_subsystem_get_camera_underwater_depth: std::ptr::null_mut(),
            a_water_zone_set_far_mesh_material: std::ptr::null_mut(),
            a_water_zone_get_water_zone_index: std::ptr::null_mut(),
            a_water_zone_force_update_water_info_texture: std::ptr::null_mut(),
        }
    }
}
pub fn initialize() {
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UBuoyancyComponent::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("OnPontoonExitedWater"),
                &raw mut __FUNCTION_PTRS.u_buoyancy_component_on_pontoon_exited_water,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("OnPontoonEnteredWater"),
                &raw mut __FUNCTION_PTRS.u_buoyancy_component_on_pontoon_entered_water,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("IsOverlappingWaterBody"),
                &raw mut __FUNCTION_PTRS.u_buoyancy_component_is_overlapping_water_body,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("IsInWaterBody"),
                &raw mut __FUNCTION_PTRS.u_buoyancy_component_is_in_water_body,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetLastWaterSurfaceInfo"),
                &raw mut __FUNCTION_PTRS.u_buoyancy_component_get_last_water_surface_info,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetCurrentWaterBodyComponents"),
                &raw mut __FUNCTION_PTRS
                    .u_buoyancy_component_get_current_water_body_components,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = ABuoyancyManager::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetBuoyancyComponentManager"),
                &raw mut __FUNCTION_PTRS
                    .a_buoyancy_manager_get_buoyancy_component_manager,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UGerstnerWaterWaveGeneratorBase::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GenerateGerstnerWaves"),
                &raw mut __FUNCTION_PTRS
                    .u_gerstner_water_wave_generator_base_generate_gerstner_waves,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UNiagaraWaterFunctionLibrary::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetWaterBodyComponent"),
                &raw mut __FUNCTION_PTRS
                    .u_niagara_water_function_library_set_water_body_component,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetWaterBody"),
                &raw mut __FUNCTION_PTRS.u_niagara_water_function_library_set_water_body,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = AWaterBody::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetWaterWaves"),
                &raw mut __FUNCTION_PTRS.a_water_body_set_water_waves,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetWaterMaterial"),
                &raw mut __FUNCTION_PTRS.a_water_body_set_water_material,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("OnWaterBodyChanged"),
                &raw mut __FUNCTION_PTRS.a_water_body_on_water_body_changed,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetWaterVelocityVectorAtSplineInputKey"),
                &raw mut __FUNCTION_PTRS
                    .a_water_body_get_water_velocity_vector_at_spline_input_key,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetWaterVelocityAtSplineInputKey"),
                &raw mut __FUNCTION_PTRS
                    .a_water_body_get_water_velocity_at_spline_input_key,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetWaterSpline"),
                &raw mut __FUNCTION_PTRS.a_water_body_get_water_spline,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetWaterMaterialInstance"),
                &raw mut __FUNCTION_PTRS.a_water_body_get_water_material_instance,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetWaterBodyType"),
                &raw mut __FUNCTION_PTRS.a_water_body_get_water_body_type,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetWaterBodyComponent"),
                &raw mut __FUNCTION_PTRS.a_water_body_get_water_body_component,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetRiverToOceanTransitionMaterialInstance"),
                &raw mut __FUNCTION_PTRS
                    .a_water_body_get_river_to_ocean_transition_material_instance,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetRiverToLakeTransitionMaterialInstance"),
                &raw mut __FUNCTION_PTRS
                    .a_water_body_get_river_to_lake_transition_material_instance,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetIslands"),
                &raw mut __FUNCTION_PTRS.a_water_body_get_islands,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetExclusionVolumes"),
                &raw mut __FUNCTION_PTRS.a_water_body_get_exclusion_volumes,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetAudioIntensityAtSplineInputKey"),
                &raw mut __FUNCTION_PTRS
                    .a_water_body_get_audio_intensity_at_spline_input_key,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UWaterBodyComponent::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetWaterZoneOverride"),
                &raw mut __FUNCTION_PTRS.u_water_body_component_set_water_zone_override,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetWaterVelocityAtSplineInputKey"),
                &raw mut __FUNCTION_PTRS
                    .u_water_body_component_set_water_velocity_at_spline_input_key,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetWaterStaticMeshMaterial"),
                &raw mut __FUNCTION_PTRS
                    .u_water_body_component_set_water_static_mesh_material,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetWaterMaterial"),
                &raw mut __FUNCTION_PTRS.u_water_body_component_set_water_material,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetWaterInfoMaterial"),
                &raw mut __FUNCTION_PTRS.u_water_body_component_set_water_info_material,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetWaterBodyStaticMeshEnabled"),
                &raw mut __FUNCTION_PTRS
                    .u_water_body_component_set_water_body_static_mesh_enabled,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetWaterAndUnderWaterPostProcessMaterial"),
                &raw mut __FUNCTION_PTRS
                    .u_water_body_component_set_water_and_under_water_post_process_material,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetUnderwaterPostProcessMaterial"),
                &raw mut __FUNCTION_PTRS
                    .u_water_body_component_set_underwater_post_process_material,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetAudioIntensityAtSplineInputKey"),
                &raw mut __FUNCTION_PTRS
                    .u_water_body_component_set_audio_intensity_at_spline_input_key,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("OnWaterBodyChanged"),
                &raw mut __FUNCTION_PTRS.u_water_body_component_on_water_body_changed,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetWaterWaves"),
                &raw mut __FUNCTION_PTRS.u_water_body_component_get_water_waves,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetWaterVelocityVectorAtSplineInputKey"),
                &raw mut __FUNCTION_PTRS
                    .u_water_body_component_get_water_velocity_vector_at_spline_input_key,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetWaterVelocityAtSplineInputKey"),
                &raw mut __FUNCTION_PTRS
                    .u_water_body_component_get_water_velocity_at_spline_input_key,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetWaterSurfaceInfoAtLocation"),
                &raw mut __FUNCTION_PTRS
                    .u_water_body_component_get_water_surface_info_at_location,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetWaterStaticMeshMaterialInstance"),
                &raw mut __FUNCTION_PTRS
                    .u_water_body_component_get_water_static_mesh_material_instance,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetWaterSpline"),
                &raw mut __FUNCTION_PTRS.u_water_body_component_get_water_spline,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetWaterMaterialInstance"),
                &raw mut __FUNCTION_PTRS
                    .u_water_body_component_get_water_material_instance,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetWaterMaterial"),
                &raw mut __FUNCTION_PTRS.u_water_body_component_get_water_material,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetWaterLODMaterialInstance"),
                &raw mut __FUNCTION_PTRS
                    .u_water_body_component_get_water_lod_material_instance,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetWaterInfoMaterialInstance"),
                &raw mut __FUNCTION_PTRS
                    .u_water_body_component_get_water_info_material_instance,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetWaterBodyActor"),
                &raw mut __FUNCTION_PTRS.u_water_body_component_get_water_body_actor,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetUnderwaterPostProcessMaterialInstance"),
                &raw mut __FUNCTION_PTRS
                    .u_water_body_component_get_underwater_post_process_material_instance,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetStandardRenderableComponents"),
                &raw mut __FUNCTION_PTRS
                    .u_water_body_component_get_standard_renderable_components,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetRiverToOceanTransitionMaterialInstance"),
                &raw mut __FUNCTION_PTRS
                    .u_water_body_component_get_river_to_ocean_transition_material_instance,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetRiverToOceanTransitionMaterial"),
                &raw mut __FUNCTION_PTRS
                    .u_water_body_component_get_river_to_ocean_transition_material,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetRiverToLakeTransitionMaterialInstance"),
                &raw mut __FUNCTION_PTRS
                    .u_water_body_component_get_river_to_lake_transition_material_instance,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetRiverToLakeTransitionMaterial"),
                &raw mut __FUNCTION_PTRS
                    .u_water_body_component_get_river_to_lake_transition_material,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetMaxWaveHeight"),
                &raw mut __FUNCTION_PTRS.u_water_body_component_get_max_wave_height,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetIslands"),
                &raw mut __FUNCTION_PTRS.u_water_body_component_get_islands,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetExclusionVolumes"),
                &raw mut __FUNCTION_PTRS.u_water_body_component_get_exclusion_volumes,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetCollisionComponents"),
                &raw mut __FUNCTION_PTRS.u_water_body_component_get_collision_components,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetAudioIntensityAtSplineInputKey"),
                &raw mut __FUNCTION_PTRS
                    .u_water_body_component_get_audio_intensity_at_spline_input_key,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = AWaterBodyIsland::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetWaterSpline"),
                &raw mut __FUNCTION_PTRS.a_water_body_island_get_water_spline,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UWaterBodyOceanComponent::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("FillWaterZoneWithOcean"),
                &raw mut __FUNCTION_PTRS
                    .u_water_body_ocean_component_fill_water_zone_with_ocean,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UWaterBodyRiverComponent::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetRiverWidthAtSplineInputKey"),
                &raw mut __FUNCTION_PTRS
                    .u_water_body_river_component_set_river_width_at_spline_input_key,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetRiverDepthAtSplineInputKey"),
                &raw mut __FUNCTION_PTRS
                    .u_water_body_river_component_set_river_depth_at_spline_input_key,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetOceanTransitionMaterial"),
                &raw mut __FUNCTION_PTRS
                    .u_water_body_river_component_set_ocean_transition_material,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetLakeTransitionMaterial"),
                &raw mut __FUNCTION_PTRS
                    .u_water_body_river_component_set_lake_transition_material,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetLakeAndOceanTransitionMaterials"),
                &raw mut __FUNCTION_PTRS
                    .u_water_body_river_component_set_lake_and_ocean_transition_materials,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetRiverWidthAtSplineInputKey"),
                &raw mut __FUNCTION_PTRS
                    .u_water_body_river_component_get_river_width_at_spline_input_key,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetRiverDepthAtSplineInputKey"),
                &raw mut __FUNCTION_PTRS
                    .u_water_body_river_component_get_river_depth_at_spline_input_key,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UWaterMeshComponent::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("IsEnabled"),
                &raw mut __FUNCTION_PTRS.u_water_mesh_component_is_enabled,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UWaterSplineComponent::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("K2_SynchronizeAndBroadcastDataChange"),
                &raw mut __FUNCTION_PTRS
                    .u_water_spline_component_k2_synchronize_and_broadcast_data_change,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UWaterSubsystem::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetOceanFloodHeight"),
                &raw mut __FUNCTION_PTRS.u_water_subsystem_set_ocean_flood_height,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("PrintToWaterLog"),
                &raw mut __FUNCTION_PTRS.u_water_subsystem_print_to_water_log,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("IsWaterRenderingEnabled"),
                &raw mut __FUNCTION_PTRS.u_water_subsystem_is_water_rendering_enabled,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("IsUnderwaterPostProcessEnabled"),
                &raw mut __FUNCTION_PTRS
                    .u_water_subsystem_is_underwater_post_process_enabled,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("IsShallowWaterSimulationEnabled"),
                &raw mut __FUNCTION_PTRS
                    .u_water_subsystem_is_shallow_water_simulation_enabled,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetWaterTimeSeconds"),
                &raw mut __FUNCTION_PTRS.u_water_subsystem_get_water_time_seconds,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetUnderwaterPreciseTraceDistance"),
                &raw mut __FUNCTION_PTRS
                    .u_water_subsystem_get_underwater_precise_trace_distance,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetUnderwaterCollisionTraceDistance"),
                &raw mut __FUNCTION_PTRS
                    .u_water_subsystem_get_underwater_collision_trace_distance,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetSmoothedWorldTimeSeconds"),
                &raw mut __FUNCTION_PTRS
                    .u_water_subsystem_get_smoothed_world_time_seconds,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetShallowWaterSimulationRenderTargetSize"),
                &raw mut __FUNCTION_PTRS
                    .u_water_subsystem_get_shallow_water_simulation_render_target_size,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetShallowWaterMaxImpulseForces"),
                &raw mut __FUNCTION_PTRS
                    .u_water_subsystem_get_shallow_water_max_impulse_forces,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetShallowWaterMaxDynamicForces"),
                &raw mut __FUNCTION_PTRS
                    .u_water_subsystem_get_shallow_water_max_dynamic_forces,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetOceanTotalHeight"),
                &raw mut __FUNCTION_PTRS.u_water_subsystem_get_ocean_total_height,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetOceanFloodHeight"),
                &raw mut __FUNCTION_PTRS.u_water_subsystem_get_ocean_flood_height,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetOceanBaseHeight"),
                &raw mut __FUNCTION_PTRS.u_water_subsystem_get_ocean_base_height,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetCameraUnderwaterDepth"),
                &raw mut __FUNCTION_PTRS.u_water_subsystem_get_camera_underwater_depth,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = AWaterZone::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetFarMeshMaterial"),
                &raw mut __FUNCTION_PTRS.a_water_zone_set_far_mesh_material,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetWaterZoneIndex"),
                &raw mut __FUNCTION_PTRS.a_water_zone_get_water_zone_index,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ForceUpdateWaterInfoTexture"),
                &raw mut __FUNCTION_PTRS.a_water_zone_force_update_water_info_texture,
            );
        }
    }
}
#[repr(C, align(16))]
pub struct FSphericalPontoon {
    pub center_socket: FName,
    pub relative_location: crate::bindings::core_u_object::FVector,
    pub radius: f32,
    pub b_fx_enabled: bool,
    pub local_force: crate::bindings::core_u_object::FVector,
    pub center_location: crate::bindings::core_u_object::FVector,
    pub socket_rotation: crate::bindings::core_u_object::FQuat,
    pub offset: crate::bindings::core_u_object::FVector,
    #[doc(hidden)]
    pub(crate) __padding_156: [u8; 4],
    pub water_height: f32,
    pub water_depth: f32,
    pub immersion_depth: f32,
    pub water_plane_location: crate::bindings::core_u_object::FVector,
    pub water_plane_normal: crate::bindings::core_u_object::FVector,
    pub water_surface_position: crate::bindings::core_u_object::FVector,
    pub water_velocity: crate::bindings::core_u_object::FVector,
    pub water_body_index: i32,
    pub b_is_in_water: bool,
    #[doc(hidden)]
    pub(crate) __padding_696: [u8; 424],
    pub current_water_body_component: UPtr<UWaterBodyComponent>,
    pub(crate) __padding_end: [u8; 16],
}
impl FSphericalPontoon {}
#[repr(C, align(8))]
pub struct FBuoyancyData {
    pub pontoons: TArray<FSphericalPontoon>,
    pub b_center_pontoons_on_com: bool,
    pub(crate) __padding_end: [u8; 127],
}
impl FBuoyancyData {}
#[repr(C, align(8))]
pub struct FWaterBodyWeightmapSettings {
    pub falloff_width: f32,
    pub edge_offset: f32,
    pub modulation_texture: TSoftObjectPtr<crate::bindings::engine::UTexture2D>,
    pub texture_tiling: f32,
    pub texture_influence: f32,
    pub midpoint: f32,
    pub final_opacity: f32,
}
impl FWaterBodyWeightmapSettings {}
#[repr(C, align(8))]
pub struct FWaterCurveSettings {
    pub b_use_curve_channel: bool,
    pub elevation_curve_asset: UPtr<crate::bindings::engine::UCurveFloat>,
    pub channel_edge_offset: f32,
    pub channel_depth: f32,
    pub curve_ramp_width: f32,
}
impl FWaterCurveSettings {}
#[repr(C, align(8))]
pub struct FShallowWaterSimulationGrid {
    pub(crate) __padding_end: [u8; 88],
}
impl FShallowWaterSimulationGrid {}
#[repr(C, align(8))]
pub struct FGerstnerWave {
    pub wave_length: f32,
    pub amplitude: f32,
    pub steepness: f32,
    pub direction: crate::bindings::core_u_object::FVector,
    pub(crate) __padding_end: [u8; 32],
}
impl FGerstnerWave {}
#[repr(C, align(4))]
pub struct FGerstnerWaveOctave {
    pub num_waves: i32,
    pub amplitude_scale: f32,
    pub main_direction: f32,
    pub spread_angle: f32,
    pub b_uniform_spread: bool,
}
impl FGerstnerWaveOctave {}
#[repr(C, align(16))]
pub struct FUnderwaterPostProcessSettings {
    pub b_enabled: bool,
    pub priority: f32,
    pub blend_radius: f32,
    pub blend_weight: f32,
    pub post_process_settings: crate::bindings::engine::FPostProcessSettings,
    pub(crate) __padding_end: [u8; 16],
}
impl FUnderwaterPostProcessSettings {}
#[repr(C, align(8))]
pub struct FWaterBodyHeightmapSettings {
    pub blend_mode: EWaterBrushBlendType,
    pub falloff_settings: FWaterFalloffSettings,
    pub effects: FWaterBrushEffects,
    pub(crate) __padding_end: [u8; 8],
}
impl FWaterBodyHeightmapSettings {}
#[repr(C, align(8))]
pub struct FWaterBrushEffects {
    pub blurring: FWaterBrushEffectBlurring,
    pub curl_noise: FWaterBrushEffectCurlNoise,
    pub displacement: FWaterBrushEffectDisplacement,
    pub smooth_blending: FWaterBrushEffectSmoothBlending,
    pub terracing: FWaterBrushEffectTerracing,
}
impl FWaterBrushEffects {}
#[repr(C, align(4))]
pub struct FWaterBrushEffectTerracing {
    pub terrace_alpha: f32,
    pub terrace_spacing: f32,
    pub terrace_smoothness: f32,
    pub mask_length: f32,
    pub mask_start_offset: f32,
}
impl FWaterBrushEffectTerracing {}
#[repr(C, align(4))]
pub struct FWaterBrushEffectSmoothBlending {
    pub inner_smooth_distance: f32,
    pub outer_smooth_distance: f32,
}
impl FWaterBrushEffectSmoothBlending {}
#[repr(C, align(8))]
pub struct FWaterBrushEffectDisplacement {
    pub displacement_height: f32,
    pub displacement_tiling: f32,
    pub texture: TSoftObjectPtr<crate::bindings::engine::UTexture2D>,
    pub midpoint: f32,
    pub channel: crate::bindings::core_u_object::FLinearColor,
    pub weightmap_influence: f32,
}
impl FWaterBrushEffectDisplacement {}
#[repr(C, align(4))]
pub struct FWaterBrushEffectCurlNoise {
    pub curl1_amount: f32,
    pub curl2_amount: f32,
    pub curl1_tiling: f32,
    pub curl2_tiling: f32,
}
impl FWaterBrushEffectCurlNoise {}
#[repr(C, align(4))]
pub struct FWaterBrushEffectBlurring {
    pub b_blur_shape: bool,
    pub radius: i32,
}
impl FWaterBrushEffectBlurring {}
#[repr(C, align(4))]
pub struct FWaterFalloffSettings {
    pub falloff_mode: EWaterBrushFalloffMode,
    pub falloff_angle: f32,
    pub falloff_width: f32,
    pub edge_offset: f32,
    pub z_offset: f32,
}
impl FWaterFalloffSettings {}
#[repr(C, align(8))]
pub struct FWaterBrushEffectCurves {
    pub b_use_curve_channel: bool,
    pub elevation_curve_asset: UPtr<crate::bindings::engine::UCurveFloat>,
    pub channel_edge_offset: f32,
    pub channel_depth: f32,
    pub curve_ramp_width: f32,
}
impl FWaterBrushEffectCurves {}
#[repr(C, align(16))]
pub struct UWaterBodyMeshComponent {
    __padding_end: [u8; 1888],
}
impl UWaterBodyMeshComponent {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UWaterBodyMeshComponent")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UWaterBodyMeshComponent")
            .copied()
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
pub struct UWaterBodyStaticMeshComponent {
    __padding_end: [u8; 1888],
}
impl UWaterBodyStaticMeshComponent {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UWaterBodyStaticMeshComponent")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UWaterBodyStaticMeshComponent")
            .copied()
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
pub struct UBakedShallowWaterSimulationComponent {
    __padding_end: [u8; 1680],
}
impl UBakedShallowWaterSimulationComponent {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBakedShallowWaterSimulationComponent")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBakedShallowWaterSimulationComponent")
            .copied()
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
pub struct UBuoyancyComponent {
    #[doc(hidden)]
    pub(crate) __padding_304: [u8; 304],
    pub buoyancy_data: FBuoyancyData,
    __padding_end: [u8; 168],
}
impl UBuoyancyComponent {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBuoyancyComponent")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBuoyancyComponent")
            .copied()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
    pub fn on_pontoon_exited_water(&mut self, pontoon: &FSphericalPontoon) {
        let mut __stack = crate::core_data::StackAlloc::<720>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::water::__FUNCTION_PTRS
                    .u_buoyancy_component_on_pontoon_exited_water,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                pontoon,
                __buffer.add(0).cast::<FSphericalPontoon>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::water::__FUNCTION_PTRS
                    .u_buoyancy_component_on_pontoon_exited_water,
                __buffer,
            )
        };
    }
    pub fn on_pontoon_entered_water(&mut self, pontoon: &FSphericalPontoon) {
        let mut __stack = crate::core_data::StackAlloc::<720>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::water::__FUNCTION_PTRS
                    .u_buoyancy_component_on_pontoon_entered_water,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                pontoon,
                __buffer.add(0).cast::<FSphericalPontoon>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::water::__FUNCTION_PTRS
                    .u_buoyancy_component_on_pontoon_entered_water,
                __buffer,
            )
        };
    }
    pub fn is_overlapping_water_body(&self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::water::__FUNCTION_PTRS
                    .u_buoyancy_component_is_overlapping_water_body,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::water::__FUNCTION_PTRS
                    .u_buoyancy_component_is_overlapping_water_body,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn is_in_water_body(&self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::water::__FUNCTION_PTRS
                    .u_buoyancy_component_is_in_water_body,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::water::__FUNCTION_PTRS
                    .u_buoyancy_component_is_in_water_body,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn get_last_water_surface_info(
        &mut self,
        out_water_plane_location: &mut crate::bindings::core_u_object::FVector,
        out_water_plane_normal: &mut crate::bindings::core_u_object::FVector,
        out_water_surface_position: &mut crate::bindings::core_u_object::FVector,
        out_water_depth: &mut f32,
        out_water_body_idx: &mut i32,
        out_water_velocity: &mut crate::bindings::core_u_object::FVector,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<104>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::water::__FUNCTION_PTRS
                    .u_buoyancy_component_get_last_water_surface_info,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_water_plane_location,
                __buffer.add(0).cast::<crate::bindings::core_u_object::FVector>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_water_plane_normal,
                __buffer.add(24).cast::<crate::bindings::core_u_object::FVector>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_water_surface_position,
                __buffer.add(48).cast::<crate::bindings::core_u_object::FVector>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_water_depth,
                __buffer.add(72).cast::<f32>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_water_body_idx,
                __buffer.add(76).cast::<i32>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_water_velocity,
                __buffer.add(80).cast::<crate::bindings::core_u_object::FVector>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::water::__FUNCTION_PTRS
                    .u_buoyancy_component_get_last_water_surface_info,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(0)
                .cast::<crate::bindings::core_u_object::FVector>()
                .swap(out_water_plane_location);
        }
        unsafe {
            __buffer
                .add(24)
                .cast::<crate::bindings::core_u_object::FVector>()
                .swap(out_water_plane_normal);
        }
        unsafe {
            __buffer
                .add(48)
                .cast::<crate::bindings::core_u_object::FVector>()
                .swap(out_water_surface_position);
        }
        unsafe {
            __buffer.add(72).cast::<f32>().swap(out_water_depth);
        }
        unsafe {
            __buffer.add(76).cast::<i32>().swap(out_water_body_idx);
        }
        unsafe {
            __buffer
                .add(80)
                .cast::<crate::bindings::core_u_object::FVector>()
                .swap(out_water_velocity);
        }
    }
    pub fn get_current_water_body_components(
        &self,
    ) -> TArray<UPtr<UWaterBodyComponent>> {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::water::__FUNCTION_PTRS
                    .u_buoyancy_component_get_current_water_body_components,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::water::__FUNCTION_PTRS
                    .u_buoyancy_component_get_current_water_body_components,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<TArray<UPtr<UWaterBodyComponent>>>().read() }
    }
}
#[repr(C, align(8))]
pub struct ABuoyancyManager {
    __padding_end: [u8; 1336],
}
impl ABuoyancyManager {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("ABuoyancyManager")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("ABuoyancyManager")
            .copied()
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
pub struct UEnvQueryTest_InsideWaterBody {
    __padding_end: [u8; 616],
}
impl UEnvQueryTest_InsideWaterBody {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEnvQueryTest_InsideWaterBody")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEnvQueryTest_InsideWaterBody")
            .copied()
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
pub struct UGerstnerWaterWaveGeneratorBase {
    __padding_end: [u8; 48],
}
impl UGerstnerWaterWaveGeneratorBase {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UGerstnerWaterWaveGeneratorBase")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UGerstnerWaterWaveGeneratorBase")
            .copied()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
    pub fn generate_gerstner_waves(&self, out_waves: &mut TArray<FGerstnerWave>) {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::water::__FUNCTION_PTRS
                    .u_gerstner_water_wave_generator_base_generate_gerstner_waves,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_waves,
                __buffer.add(0).cast::<TArray<FGerstnerWave>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::water::__FUNCTION_PTRS
                    .u_gerstner_water_wave_generator_base_generate_gerstner_waves,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<TArray<FGerstnerWave>>().swap(out_waves);
        }
    }
}
#[repr(C, align(8))]
pub struct UGerstnerWaterWaveGeneratorSimple {
    #[doc(hidden)]
    pub(crate) __padding_48: [u8; 48],
    pub num_waves: i32,
    pub seed: i32,
    pub randomness: f32,
    pub min_wavelength: f32,
    pub max_wavelength: f32,
    pub wavelength_falloff: f32,
    pub min_amplitude: f32,
    pub max_amplitude: f32,
    pub amplitude_falloff: f32,
    pub wind_angle_deg: f32,
    pub direction_angular_spread_deg: f32,
    pub small_wave_steepness: f32,
    pub large_wave_steepness: f32,
    pub steepness_falloff: f32,
}
impl UGerstnerWaterWaveGeneratorSimple {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UGerstnerWaterWaveGeneratorSimple")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UGerstnerWaterWaveGeneratorSimple")
            .copied()
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
pub struct UGerstnerWaterWaveGeneratorSpectrum {
    __padding_end: [u8; 72],
}
impl UGerstnerWaterWaveGeneratorSpectrum {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UGerstnerWaterWaveGeneratorSpectrum")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UGerstnerWaterWaveGeneratorSpectrum")
            .copied()
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
pub struct UWaterWavesBase {
    __padding_end: [u8; 80],
}
impl UWaterWavesBase {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UWaterWavesBase")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UWaterWavesBase")
            .copied()
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
pub struct UWaterWaves {
    __padding_end: [u8; 80],
}
impl UWaterWaves {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UWaterWaves")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UWaterWaves")
            .copied()
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
pub struct UGerstnerWaterWaves {
    #[doc(hidden)]
    pub(crate) __padding_80: [u8; 80],
    pub gerstner_wave_generator: UPtr<UGerstnerWaterWaveGeneratorBase>,
    pub gerstner_waves: TArray<FGerstnerWave>,
    pub max_wave_height: f32,
}
impl UGerstnerWaterWaves {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UGerstnerWaterWaves")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UGerstnerWaterWaves")
            .copied()
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
pub struct UGerstnerWaterWaveSubsystem {
    __padding_end: [u8; 80],
}
impl UGerstnerWaterWaveSubsystem {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UGerstnerWaterWaveSubsystem")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UGerstnerWaterWaveSubsystem")
            .copied()
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
pub struct ULakeCollisionComponent {
    __padding_end: [u8; 1536],
}
impl ULakeCollisionComponent {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("ULakeCollisionComponent")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("ULakeCollisionComponent")
            .copied()
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
pub struct UNiagaraDataInterfaceWater {
    __padding_end: [u8; 216],
}
impl UNiagaraDataInterfaceWater {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraDataInterfaceWater")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraDataInterfaceWater")
            .copied()
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
pub struct UNiagaraWaterFunctionLibrary {
    __padding_end: [u8; 48],
}
impl UNiagaraWaterFunctionLibrary {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraWaterFunctionLibrary")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraWaterFunctionLibrary")
            .copied()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
    pub fn set_water_body_component(
        niagara_system: UPtr<crate::bindings::niagara::UNiagaraComponent>,
        override_name: FString,
        water_body_component: UPtr<UWaterBodyComponent>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::water::__FUNCTION_PTRS
                    .u_niagara_water_function_library_set_water_body_component,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &niagara_system,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::niagara::UNiagaraComponent>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &override_name,
                __buffer.add(8).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &water_body_component,
                __buffer.add(24).cast::<UPtr<UWaterBodyComponent>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::water::UNiagaraWaterFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::water::__FUNCTION_PTRS
                    .u_niagara_water_function_library_set_water_body_component,
                __buffer,
            )
        };
    }
    pub fn set_water_body(
        niagara_system: UPtr<crate::bindings::niagara::UNiagaraComponent>,
        override_name: FString,
        water_body: UPtr<AWaterBody>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::water::__FUNCTION_PTRS
                    .u_niagara_water_function_library_set_water_body,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &niagara_system,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::niagara::UNiagaraComponent>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &override_name,
                __buffer.add(8).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &water_body,
                __buffer.add(24).cast::<UPtr<AWaterBody>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::water::UNiagaraWaterFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::water::__FUNCTION_PTRS
                    .u_niagara_water_function_library_set_water_body,
                __buffer,
            )
        };
    }
}
#[repr(C, align(16))]
pub struct UOceanCollisionComponent {
    __padding_end: [u8; 1568],
}
impl UOceanCollisionComponent {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UOceanCollisionComponent")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UOceanCollisionComponent")
            .copied()
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
pub struct UOceanBoxCollisionComponent {
    __padding_end: [u8; 1632],
}
impl UOceanBoxCollisionComponent {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UOceanBoxCollisionComponent")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UOceanBoxCollisionComponent")
            .copied()
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
pub struct AWaterBody {
    #[doc(hidden)]
    pub(crate) __padding_1144: [u8; 1144],
    pub spline_comp: UPtr<UWaterSplineComponent>,
    #[doc(hidden)]
    pub(crate) __padding_1160: [u8; 8],
    pub water_body_component: UPtr<UWaterBodyComponent>,
    pub water_body_index: i32,
    pub water_waves: UPtr<UWaterWavesBase>,
    __padding_end: [u8; 2608],
}
impl AWaterBody {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("AWaterBody")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("AWaterBody")
            .copied()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
    pub fn set_water_waves(&mut self, in_water_waves: UPtr<UWaterWavesBase>) {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::water::__FUNCTION_PTRS.a_water_body_set_water_waves,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_water_waves,
                __buffer.add(0).cast::<UPtr<UWaterWavesBase>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::water::__FUNCTION_PTRS.a_water_body_set_water_waves,
                __buffer,
            )
        };
    }
    pub fn set_water_material(
        &mut self,
        in_material: UPtr<crate::bindings::engine::UMaterialInterface>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::water::__FUNCTION_PTRS.a_water_body_set_water_material,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_material,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::engine::UMaterialInterface>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::water::__FUNCTION_PTRS.a_water_body_set_water_material,
                __buffer,
            )
        };
    }
    pub fn on_water_body_changed(
        &mut self,
        b_shape_or_position_changed: bool,
        b_weightmap_settings_changed: bool,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<2>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::water::__FUNCTION_PTRS
                    .a_water_body_on_water_body_changed,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_shape_or_position_changed,
                __buffer.add(0).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_weightmap_settings_changed,
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
                crate::bindings::water::__FUNCTION_PTRS
                    .a_water_body_on_water_body_changed,
                __buffer,
            )
        };
    }
    pub fn get_water_velocity_vector_at_spline_input_key(
        &self,
        in_key: f32,
    ) -> crate::bindings::core_u_object::FVector {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::water::__FUNCTION_PTRS
                    .a_water_body_get_water_velocity_vector_at_spline_input_key,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&in_key, __buffer.add(0).cast::<f32>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::water::__FUNCTION_PTRS
                    .a_water_body_get_water_velocity_vector_at_spline_input_key,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(8).cast::<crate::bindings::core_u_object::FVector>().read()
        }
    }
    pub fn get_water_velocity_at_spline_input_key(&self, in_key: f32) -> f32 {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::water::__FUNCTION_PTRS
                    .a_water_body_get_water_velocity_at_spline_input_key,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&in_key, __buffer.add(0).cast::<f32>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::water::__FUNCTION_PTRS
                    .a_water_body_get_water_velocity_at_spline_input_key,
                __buffer,
            )
        };
        unsafe { __buffer.add(4).cast::<f32>().read() }
    }
    pub fn get_water_spline(&self) -> UPtr<UWaterSplineComponent> {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::water::__FUNCTION_PTRS.a_water_body_get_water_spline,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::water::__FUNCTION_PTRS.a_water_body_get_water_spline,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<UPtr<UWaterSplineComponent>>().read() }
    }
    pub fn get_water_material_instance(
        &mut self,
    ) -> UPtr<crate::bindings::engine::UMaterialInstanceDynamic> {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::water::__FUNCTION_PTRS
                    .a_water_body_get_water_material_instance,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::water::__FUNCTION_PTRS
                    .a_water_body_get_water_material_instance,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(0)
                .cast::<UPtr<crate::bindings::engine::UMaterialInstanceDynamic>>()
                .read()
        }
    }
    pub fn get_water_body_type(&self) -> EWaterBodyType {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::water::__FUNCTION_PTRS.a_water_body_get_water_body_type,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::water::__FUNCTION_PTRS.a_water_body_get_water_body_type,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<EWaterBodyType>().read() }
    }
    pub fn get_water_body_component(&self) -> UPtr<UWaterBodyComponent> {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::water::__FUNCTION_PTRS
                    .a_water_body_get_water_body_component,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::water::__FUNCTION_PTRS
                    .a_water_body_get_water_body_component,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<UPtr<UWaterBodyComponent>>().read() }
    }
    pub fn get_river_to_ocean_transition_material_instance(
        &mut self,
    ) -> UPtr<crate::bindings::engine::UMaterialInstanceDynamic> {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::water::__FUNCTION_PTRS
                    .a_water_body_get_river_to_ocean_transition_material_instance,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::water::__FUNCTION_PTRS
                    .a_water_body_get_river_to_ocean_transition_material_instance,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(0)
                .cast::<UPtr<crate::bindings::engine::UMaterialInstanceDynamic>>()
                .read()
        }
    }
    pub fn get_river_to_lake_transition_material_instance(
        &mut self,
    ) -> UPtr<crate::bindings::engine::UMaterialInstanceDynamic> {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::water::__FUNCTION_PTRS
                    .a_water_body_get_river_to_lake_transition_material_instance,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::water::__FUNCTION_PTRS
                    .a_water_body_get_river_to_lake_transition_material_instance,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(0)
                .cast::<UPtr<crate::bindings::engine::UMaterialInstanceDynamic>>()
                .read()
        }
    }
    pub fn get_islands(&self) -> TArray<UPtr<AWaterBodyIsland>> {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::water::__FUNCTION_PTRS.a_water_body_get_islands,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::water::__FUNCTION_PTRS.a_water_body_get_islands,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<TArray<UPtr<AWaterBodyIsland>>>().read() }
    }
    pub fn get_exclusion_volumes(&self) -> TArray<UPtr<AWaterBodyExclusionVolume>> {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::water::__FUNCTION_PTRS
                    .a_water_body_get_exclusion_volumes,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::water::__FUNCTION_PTRS
                    .a_water_body_get_exclusion_volumes,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<TArray<UPtr<AWaterBodyExclusionVolume>>>().read()
        }
    }
    pub fn get_audio_intensity_at_spline_input_key(&self, in_key: f32) -> f32 {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::water::__FUNCTION_PTRS
                    .a_water_body_get_audio_intensity_at_spline_input_key,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&in_key, __buffer.add(0).cast::<f32>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::water::__FUNCTION_PTRS
                    .a_water_body_get_audio_intensity_at_spline_input_key,
                __buffer,
            )
        };
        unsafe { __buffer.add(4).cast::<f32>().read() }
    }
}
#[repr(C, align(8))]
pub struct UDEPRECATED_WaterBodyGenerator {
    __padding_end: [u8; 48],
}
impl UDEPRECATED_WaterBodyGenerator {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDEPRECATED_WaterBodyGenerator")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDEPRECATED_WaterBodyGenerator")
            .copied()
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
pub struct UWaterBodyComponent {
    #[doc(hidden)]
    pub(crate) __padding_1504: [u8; 1504],
    pub target_wave_mask_depth: f32,
    pub underwater_post_process_settings: FUnderwaterPostProcessSettings,
    pub curve_settings: FWaterCurveSettings,
    pub water_material: UPtr<crate::bindings::engine::UMaterialInterface>,
    pub water_hlod_material: UPtr<crate::bindings::engine::UMaterialInterface>,
    pub water_static_mesh_material: UPtr<crate::bindings::engine::UMaterialInterface>,
    pub underwater_post_process_material: UPtr<
        crate::bindings::engine::UMaterialInterface,
    >,
    pub water_info_material: UPtr<crate::bindings::engine::UMaterialInterface>,
    pub water_heightmap_settings: FWaterBodyHeightmapSettings,
    pub layer_weightmap_settings: TMap<FName, FWaterBodyWeightmapSettings>,
    pub shape_dilation: f32,
    pub collision_height_offset: f32,
    pub b_affects_landscape: bool,
    #[doc(hidden)]
    pub(crate) __padding_3856: [u8; 20],
    pub water_body_index: i32,
    pub water_mesh_override: UPtr<crate::bindings::engine::UStaticMesh>,
    pub b_always_generate_water_mesh_tiles: bool,
    pub overlap_material_priority: i32,
    #[doc(hidden)]
    pub(crate) __padding_4008: [u8; 128],
    pub water_zone_override: TSoftObjectPtr<AWaterZone>,
    #[doc(hidden)]
    pub(crate) __padding_6016: [u8; 1960],
    pub water_nav_area_class: TSubclassOf<crate::bindings::engine::UNavAreaBase>,
    __padding_end: [u8; 104],
}
impl UWaterBodyComponent {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UWaterBodyComponent")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UWaterBodyComponent")
            .copied()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
    pub fn set_water_zone_override(
        &mut self,
        in_water_zone_override: &TSoftObjectPtr<AWaterZone>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<48>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::water::__FUNCTION_PTRS
                    .u_water_body_component_set_water_zone_override,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_water_zone_override,
                __buffer.add(0).cast::<TSoftObjectPtr<AWaterZone>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::water::__FUNCTION_PTRS
                    .u_water_body_component_set_water_zone_override,
                __buffer,
            )
        };
    }
    pub fn set_water_velocity_at_spline_input_key(
        &mut self,
        in_key: f32,
        in_velocity: f32,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::water::__FUNCTION_PTRS
                    .u_water_body_component_set_water_velocity_at_spline_input_key,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&in_key, __buffer.add(0).cast::<f32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_velocity,
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
                crate::bindings::water::__FUNCTION_PTRS
                    .u_water_body_component_set_water_velocity_at_spline_input_key,
                __buffer,
            )
        };
    }
    pub fn set_water_static_mesh_material(
        &mut self,
        in_material: UPtr<crate::bindings::engine::UMaterialInterface>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::water::__FUNCTION_PTRS
                    .u_water_body_component_set_water_static_mesh_material,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_material,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::engine::UMaterialInterface>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::water::__FUNCTION_PTRS
                    .u_water_body_component_set_water_static_mesh_material,
                __buffer,
            )
        };
    }
    pub fn set_water_material(
        &mut self,
        in_material: UPtr<crate::bindings::engine::UMaterialInterface>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::water::__FUNCTION_PTRS
                    .u_water_body_component_set_water_material,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_material,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::engine::UMaterialInterface>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::water::__FUNCTION_PTRS
                    .u_water_body_component_set_water_material,
                __buffer,
            )
        };
    }
    pub fn set_water_info_material(
        &mut self,
        in_material: UPtr<crate::bindings::engine::UMaterialInterface>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::water::__FUNCTION_PTRS
                    .u_water_body_component_set_water_info_material,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_material,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::engine::UMaterialInterface>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::water::__FUNCTION_PTRS
                    .u_water_body_component_set_water_info_material,
                __buffer,
            )
        };
    }
    pub fn set_water_body_static_mesh_enabled(&mut self, b_enabled: bool) {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::water::__FUNCTION_PTRS
                    .u_water_body_component_set_water_body_static_mesh_enabled,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&b_enabled, __buffer.add(0).cast::<bool>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::water::__FUNCTION_PTRS
                    .u_water_body_component_set_water_body_static_mesh_enabled,
                __buffer,
            )
        };
    }
    pub fn set_water_and_under_water_post_process_material(
        &mut self,
        in_water_material: UPtr<crate::bindings::engine::UMaterialInterface>,
        in_under_water_post_process_material: UPtr<
            crate::bindings::engine::UMaterialInterface,
        >,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::water::__FUNCTION_PTRS
                    .u_water_body_component_set_water_and_under_water_post_process_material,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_water_material,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::engine::UMaterialInterface>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_under_water_post_process_material,
                __buffer
                    .add(8)
                    .cast::<UPtr<crate::bindings::engine::UMaterialInterface>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::water::__FUNCTION_PTRS
                    .u_water_body_component_set_water_and_under_water_post_process_material,
                __buffer,
            )
        };
    }
    pub fn set_underwater_post_process_material(
        &mut self,
        in_material: UPtr<crate::bindings::engine::UMaterialInterface>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::water::__FUNCTION_PTRS
                    .u_water_body_component_set_underwater_post_process_material,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_material,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::engine::UMaterialInterface>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::water::__FUNCTION_PTRS
                    .u_water_body_component_set_underwater_post_process_material,
                __buffer,
            )
        };
    }
    pub fn set_audio_intensity_at_spline_input_key(
        &mut self,
        in_key: f32,
        in_audio_intensity: f32,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::water::__FUNCTION_PTRS
                    .u_water_body_component_set_audio_intensity_at_spline_input_key,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&in_key, __buffer.add(0).cast::<f32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_audio_intensity,
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
                crate::bindings::water::__FUNCTION_PTRS
                    .u_water_body_component_set_audio_intensity_at_spline_input_key,
                __buffer,
            )
        };
    }
    pub fn on_water_body_changed(
        &mut self,
        b_shape_or_position_changed: bool,
        b_weightmap_settings_changed: bool,
        b_user_triggered_changed: bool,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<3>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::water::__FUNCTION_PTRS
                    .u_water_body_component_on_water_body_changed,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_shape_or_position_changed,
                __buffer.add(0).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_weightmap_settings_changed,
                __buffer.add(1).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_user_triggered_changed,
                __buffer.add(2).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::water::__FUNCTION_PTRS
                    .u_water_body_component_on_water_body_changed,
                __buffer,
            )
        };
    }
    pub fn get_water_waves(&self) -> UPtr<UWaterWavesBase> {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::water::__FUNCTION_PTRS
                    .u_water_body_component_get_water_waves,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::water::__FUNCTION_PTRS
                    .u_water_body_component_get_water_waves,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<UPtr<UWaterWavesBase>>().read() }
    }
    pub fn get_water_velocity_vector_at_spline_input_key(
        &self,
        in_key: f32,
    ) -> crate::bindings::core_u_object::FVector {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::water::__FUNCTION_PTRS
                    .u_water_body_component_get_water_velocity_vector_at_spline_input_key,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&in_key, __buffer.add(0).cast::<f32>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::water::__FUNCTION_PTRS
                    .u_water_body_component_get_water_velocity_vector_at_spline_input_key,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(8).cast::<crate::bindings::core_u_object::FVector>().read()
        }
    }
    pub fn get_water_velocity_at_spline_input_key(&self, in_key: f32) -> f32 {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::water::__FUNCTION_PTRS
                    .u_water_body_component_get_water_velocity_at_spline_input_key,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&in_key, __buffer.add(0).cast::<f32>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::water::__FUNCTION_PTRS
                    .u_water_body_component_get_water_velocity_at_spline_input_key,
                __buffer,
            )
        };
        unsafe { __buffer.add(4).cast::<f32>().read() }
    }
    pub fn get_water_surface_info_at_location(
        &self,
        in_location: &crate::bindings::core_u_object::FVector,
        out_water_surface_location: &mut crate::bindings::core_u_object::FVector,
        out_water_surface_normal: &mut crate::bindings::core_u_object::FVector,
        out_water_velocity: &mut crate::bindings::core_u_object::FVector,
        out_water_depth: &mut f32,
        b_include_depth: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<102>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::water::__FUNCTION_PTRS
                    .u_water_body_component_get_water_surface_info_at_location,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_location,
                __buffer.add(0).cast::<crate::bindings::core_u_object::FVector>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_water_surface_location,
                __buffer.add(24).cast::<crate::bindings::core_u_object::FVector>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_water_surface_normal,
                __buffer.add(48).cast::<crate::bindings::core_u_object::FVector>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_water_velocity,
                __buffer.add(72).cast::<crate::bindings::core_u_object::FVector>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_water_depth,
                __buffer.add(96).cast::<f32>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_include_depth,
                __buffer.add(100).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::water::__FUNCTION_PTRS
                    .u_water_body_component_get_water_surface_info_at_location,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(24)
                .cast::<crate::bindings::core_u_object::FVector>()
                .swap(out_water_surface_location);
        }
        unsafe {
            __buffer
                .add(48)
                .cast::<crate::bindings::core_u_object::FVector>()
                .swap(out_water_surface_normal);
        }
        unsafe {
            __buffer
                .add(72)
                .cast::<crate::bindings::core_u_object::FVector>()
                .swap(out_water_velocity);
        }
        unsafe {
            __buffer.add(96).cast::<f32>().swap(out_water_depth);
        }
        unsafe { __buffer.add(101).cast::<bool>().read() }
    }
    pub fn get_water_static_mesh_material_instance(
        &mut self,
    ) -> UPtr<crate::bindings::engine::UMaterialInstanceDynamic> {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::water::__FUNCTION_PTRS
                    .u_water_body_component_get_water_static_mesh_material_instance,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::water::__FUNCTION_PTRS
                    .u_water_body_component_get_water_static_mesh_material_instance,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(0)
                .cast::<UPtr<crate::bindings::engine::UMaterialInstanceDynamic>>()
                .read()
        }
    }
    pub fn get_water_spline(&self) -> UPtr<UWaterSplineComponent> {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::water::__FUNCTION_PTRS
                    .u_water_body_component_get_water_spline,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::water::__FUNCTION_PTRS
                    .u_water_body_component_get_water_spline,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<UPtr<UWaterSplineComponent>>().read() }
    }
    pub fn get_water_material_instance(
        &mut self,
    ) -> UPtr<crate::bindings::engine::UMaterialInstanceDynamic> {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::water::__FUNCTION_PTRS
                    .u_water_body_component_get_water_material_instance,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::water::__FUNCTION_PTRS
                    .u_water_body_component_get_water_material_instance,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(0)
                .cast::<UPtr<crate::bindings::engine::UMaterialInstanceDynamic>>()
                .read()
        }
    }
    pub fn get_water_material(
        &self,
    ) -> UPtr<crate::bindings::engine::UMaterialInterface> {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::water::__FUNCTION_PTRS
                    .u_water_body_component_get_water_material,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::water::__FUNCTION_PTRS
                    .u_water_body_component_get_water_material,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(0)
                .cast::<UPtr<crate::bindings::engine::UMaterialInterface>>()
                .read()
        }
    }
    pub fn get_water_lod_material_instance(
        &mut self,
    ) -> UPtr<crate::bindings::engine::UMaterialInstanceDynamic> {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::water::__FUNCTION_PTRS
                    .u_water_body_component_get_water_lod_material_instance,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::water::__FUNCTION_PTRS
                    .u_water_body_component_get_water_lod_material_instance,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(0)
                .cast::<UPtr<crate::bindings::engine::UMaterialInstanceDynamic>>()
                .read()
        }
    }
    pub fn get_water_info_material_instance(
        &mut self,
    ) -> UPtr<crate::bindings::engine::UMaterialInstanceDynamic> {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::water::__FUNCTION_PTRS
                    .u_water_body_component_get_water_info_material_instance,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::water::__FUNCTION_PTRS
                    .u_water_body_component_get_water_info_material_instance,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(0)
                .cast::<UPtr<crate::bindings::engine::UMaterialInstanceDynamic>>()
                .read()
        }
    }
    pub fn get_water_body_actor(&self) -> UPtr<AWaterBody> {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::water::__FUNCTION_PTRS
                    .u_water_body_component_get_water_body_actor,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::water::__FUNCTION_PTRS
                    .u_water_body_component_get_water_body_actor,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<UPtr<AWaterBody>>().read() }
    }
    pub fn get_underwater_post_process_material_instance(
        &mut self,
    ) -> UPtr<crate::bindings::engine::UMaterialInstanceDynamic> {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::water::__FUNCTION_PTRS
                    .u_water_body_component_get_underwater_post_process_material_instance,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::water::__FUNCTION_PTRS
                    .u_water_body_component_get_underwater_post_process_material_instance,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(0)
                .cast::<UPtr<crate::bindings::engine::UMaterialInstanceDynamic>>()
                .read()
        }
    }
    pub fn get_standard_renderable_components(
        &self,
    ) -> TArray<UPtr<crate::bindings::engine::UPrimitiveComponent>> {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::water::__FUNCTION_PTRS
                    .u_water_body_component_get_standard_renderable_components,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::water::__FUNCTION_PTRS
                    .u_water_body_component_get_standard_renderable_components,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(0)
                .cast::<TArray<UPtr<crate::bindings::engine::UPrimitiveComponent>>>()
                .read()
        }
    }
    pub fn get_river_to_ocean_transition_material_instance(
        &mut self,
    ) -> UPtr<crate::bindings::engine::UMaterialInstanceDynamic> {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::water::__FUNCTION_PTRS
                    .u_water_body_component_get_river_to_ocean_transition_material_instance,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::water::__FUNCTION_PTRS
                    .u_water_body_component_get_river_to_ocean_transition_material_instance,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(0)
                .cast::<UPtr<crate::bindings::engine::UMaterialInstanceDynamic>>()
                .read()
        }
    }
    pub fn get_river_to_ocean_transition_material(
        &self,
    ) -> UPtr<crate::bindings::engine::UMaterialInterface> {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::water::__FUNCTION_PTRS
                    .u_water_body_component_get_river_to_ocean_transition_material,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::water::__FUNCTION_PTRS
                    .u_water_body_component_get_river_to_ocean_transition_material,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(0)
                .cast::<UPtr<crate::bindings::engine::UMaterialInterface>>()
                .read()
        }
    }
    pub fn get_river_to_lake_transition_material_instance(
        &mut self,
    ) -> UPtr<crate::bindings::engine::UMaterialInstanceDynamic> {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::water::__FUNCTION_PTRS
                    .u_water_body_component_get_river_to_lake_transition_material_instance,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::water::__FUNCTION_PTRS
                    .u_water_body_component_get_river_to_lake_transition_material_instance,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(0)
                .cast::<UPtr<crate::bindings::engine::UMaterialInstanceDynamic>>()
                .read()
        }
    }
    pub fn get_river_to_lake_transition_material(
        &self,
    ) -> UPtr<crate::bindings::engine::UMaterialInterface> {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::water::__FUNCTION_PTRS
                    .u_water_body_component_get_river_to_lake_transition_material,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::water::__FUNCTION_PTRS
                    .u_water_body_component_get_river_to_lake_transition_material,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(0)
                .cast::<UPtr<crate::bindings::engine::UMaterialInterface>>()
                .read()
        }
    }
    pub fn get_max_wave_height(&self) -> f32 {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::water::__FUNCTION_PTRS
                    .u_water_body_component_get_max_wave_height,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::water::__FUNCTION_PTRS
                    .u_water_body_component_get_max_wave_height,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<f32>().read() }
    }
    pub fn get_islands(&self) -> TArray<UPtr<AWaterBodyIsland>> {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::water::__FUNCTION_PTRS
                    .u_water_body_component_get_islands,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::water::__FUNCTION_PTRS
                    .u_water_body_component_get_islands,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<TArray<UPtr<AWaterBodyIsland>>>().read() }
    }
    pub fn get_exclusion_volumes(&self) -> TArray<UPtr<AWaterBodyExclusionVolume>> {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::water::__FUNCTION_PTRS
                    .u_water_body_component_get_exclusion_volumes,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::water::__FUNCTION_PTRS
                    .u_water_body_component_get_exclusion_volumes,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<TArray<UPtr<AWaterBodyExclusionVolume>>>().read()
        }
    }
    pub fn get_collision_components(
        &self,
        b_in_only_enabled_components: bool,
    ) -> TArray<UPtr<crate::bindings::engine::UPrimitiveComponent>> {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::water::__FUNCTION_PTRS
                    .u_water_body_component_get_collision_components,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_in_only_enabled_components,
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
                crate::bindings::water::__FUNCTION_PTRS
                    .u_water_body_component_get_collision_components,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(8)
                .cast::<TArray<UPtr<crate::bindings::engine::UPrimitiveComponent>>>()
                .read()
        }
    }
    pub fn get_audio_intensity_at_spline_input_key(&self, in_key: f32) -> f32 {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::water::__FUNCTION_PTRS
                    .u_water_body_component_get_audio_intensity_at_spline_input_key,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&in_key, __buffer.add(0).cast::<f32>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::water::__FUNCTION_PTRS
                    .u_water_body_component_get_audio_intensity_at_spline_input_key,
                __buffer,
            )
        };
        unsafe { __buffer.add(4).cast::<f32>().read() }
    }
}
#[repr(C, align(8))]
pub struct UDEPRECATED_CustomMeshGenerator {
    __padding_end: [u8; 56],
}
impl UDEPRECATED_CustomMeshGenerator {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDEPRECATED_CustomMeshGenerator")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDEPRECATED_CustomMeshGenerator")
            .copied()
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
pub struct AWaterBodyCustom {
    __padding_end: [u8; 3808],
}
impl AWaterBodyCustom {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("AWaterBodyCustom")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("AWaterBodyCustom")
            .copied()
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
pub struct UWaterBodyCustomComponent {
    __padding_end: [u8; 6128],
}
impl UWaterBodyCustomComponent {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UWaterBodyCustomComponent")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UWaterBodyCustomComponent")
            .copied()
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
pub struct AWaterBodyExclusionVolume {
    #[doc(hidden)]
    pub(crate) __padding_1232: [u8; 1232],
    pub water_bodies: TArray<TSoftObjectPtr<AWaterBody>>,
    __padding_end: [u8; 64],
}
impl AWaterBodyExclusionVolume {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("AWaterBodyExclusionVolume")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("AWaterBodyExclusionVolume")
            .copied()
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
pub struct UWaterBodyHLODBuilder {
    __padding_end: [u8; 72],
}
impl UWaterBodyHLODBuilder {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UWaterBodyHLODBuilder")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UWaterBodyHLODBuilder")
            .copied()
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
pub struct UWaterBodyInfoMeshComponent {
    __padding_end: [u8; 1904],
}
impl UWaterBodyInfoMeshComponent {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UWaterBodyInfoMeshComponent")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UWaterBodyInfoMeshComponent")
            .copied()
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
pub struct AWaterBodyIsland {
    #[doc(hidden)]
    pub(crate) __padding_1144: [u8; 1144],
    pub water_curve_settings: FWaterCurveSettings,
    pub water_heightmap_settings: FWaterBodyHeightmapSettings,
    pub water_weightmap_settings: TMap<FName, FWaterBodyWeightmapSettings>,
    #[doc(hidden)]
    pub(crate) __padding_1432: [u8; 8],
    pub spline_comp: UPtr<UWaterSplineComponent>,
}
impl AWaterBodyIsland {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("AWaterBodyIsland")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("AWaterBodyIsland")
            .copied()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
    pub fn get_water_spline(&self) -> UPtr<UWaterSplineComponent> {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::water::__FUNCTION_PTRS
                    .a_water_body_island_get_water_spline,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::water::__FUNCTION_PTRS
                    .a_water_body_island_get_water_spline,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<UPtr<UWaterSplineComponent>>().read() }
    }
}
#[repr(C, align(8))]
pub struct UDEPRECATED_LakeGenerator {
    __padding_end: [u8; 72],
}
impl UDEPRECATED_LakeGenerator {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDEPRECATED_LakeGenerator")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDEPRECATED_LakeGenerator")
            .copied()
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
pub struct AWaterBodyLake {
    __padding_end: [u8; 3808],
}
impl AWaterBodyLake {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("AWaterBodyLake")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("AWaterBodyLake")
            .copied()
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
pub struct UWaterBodyLakeComponent {
    __padding_end: [u8; 6144],
}
impl UWaterBodyLakeComponent {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UWaterBodyLakeComponent")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UWaterBodyLakeComponent")
            .copied()
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
pub struct UDEPRECATED_OceanGenerator {
    __padding_end: [u8; 80],
}
impl UDEPRECATED_OceanGenerator {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDEPRECATED_OceanGenerator")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDEPRECATED_OceanGenerator")
            .copied()
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
pub struct AWaterBodyOcean {
    __padding_end: [u8; 3824],
}
impl AWaterBodyOcean {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("AWaterBodyOcean")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("AWaterBodyOcean")
            .copied()
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
pub struct UWaterBodyOceanComponent {
    #[doc(hidden)]
    pub(crate) __padding_6152: [u8; 6152],
    pub collision_extents: crate::bindings::core_u_object::FVector,
    pub ocean_extents: crate::bindings::core_u_object::FVector2D,
    __padding_end: [u8; 48],
}
impl UWaterBodyOceanComponent {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UWaterBodyOceanComponent")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UWaterBodyOceanComponent")
            .copied()
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
pub struct UDEPRECATED_RiverGenerator {
    __padding_end: [u8; 64],
}
impl UDEPRECATED_RiverGenerator {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDEPRECATED_RiverGenerator")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDEPRECATED_RiverGenerator")
            .copied()
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
pub struct AWaterBodyRiver {
    __padding_end: [u8; 3840],
}
impl AWaterBodyRiver {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("AWaterBodyRiver")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("AWaterBodyRiver")
            .copied()
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
pub struct UWaterBodyRiverComponent {
    #[doc(hidden)]
    pub(crate) __padding_6136: [u8; 6136],
    pub lake_transition_material: UPtr<crate::bindings::engine::UMaterialInterface>,
    #[doc(hidden)]
    pub(crate) __padding_6152: [u8; 8],
    pub ocean_transition_material: UPtr<crate::bindings::engine::UMaterialInterface>,
    __padding_end: [u8; 16],
}
impl UWaterBodyRiverComponent {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UWaterBodyRiverComponent")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UWaterBodyRiverComponent")
            .copied()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
    pub fn set_river_width_at_spline_input_key(&mut self, in_key: f32, in_width: f32) {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::water::__FUNCTION_PTRS
                    .u_water_body_river_component_set_river_width_at_spline_input_key,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&in_key, __buffer.add(0).cast::<f32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&in_width, __buffer.add(4).cast::<f32>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::water::__FUNCTION_PTRS
                    .u_water_body_river_component_set_river_width_at_spline_input_key,
                __buffer,
            )
        };
    }
    pub fn set_river_depth_at_spline_input_key(&mut self, in_key: f32, in_depth: f32) {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::water::__FUNCTION_PTRS
                    .u_water_body_river_component_set_river_depth_at_spline_input_key,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&in_key, __buffer.add(0).cast::<f32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&in_depth, __buffer.add(4).cast::<f32>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::water::__FUNCTION_PTRS
                    .u_water_body_river_component_set_river_depth_at_spline_input_key,
                __buffer,
            )
        };
    }
    pub fn set_ocean_transition_material(
        &mut self,
        in_mat: UPtr<crate::bindings::engine::UMaterialInterface>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::water::__FUNCTION_PTRS
                    .u_water_body_river_component_set_ocean_transition_material,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_mat,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::engine::UMaterialInterface>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::water::__FUNCTION_PTRS
                    .u_water_body_river_component_set_ocean_transition_material,
                __buffer,
            )
        };
    }
    pub fn set_lake_transition_material(
        &mut self,
        in_mat: UPtr<crate::bindings::engine::UMaterialInterface>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::water::__FUNCTION_PTRS
                    .u_water_body_river_component_set_lake_transition_material,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_mat,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::engine::UMaterialInterface>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::water::__FUNCTION_PTRS
                    .u_water_body_river_component_set_lake_transition_material,
                __buffer,
            )
        };
    }
    pub fn set_lake_and_ocean_transition_materials(
        &mut self,
        in_lake_transition: UPtr<crate::bindings::engine::UMaterialInterface>,
        in_ocean_transition: UPtr<crate::bindings::engine::UMaterialInterface>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::water::__FUNCTION_PTRS
                    .u_water_body_river_component_set_lake_and_ocean_transition_materials,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_lake_transition,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::engine::UMaterialInterface>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_ocean_transition,
                __buffer
                    .add(8)
                    .cast::<UPtr<crate::bindings::engine::UMaterialInterface>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::water::__FUNCTION_PTRS
                    .u_water_body_river_component_set_lake_and_ocean_transition_materials,
                __buffer,
            )
        };
    }
    pub fn get_river_width_at_spline_input_key(&self, in_key: f32) -> f32 {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::water::__FUNCTION_PTRS
                    .u_water_body_river_component_get_river_width_at_spline_input_key,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&in_key, __buffer.add(0).cast::<f32>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::water::__FUNCTION_PTRS
                    .u_water_body_river_component_get_river_width_at_spline_input_key,
                __buffer,
            )
        };
        unsafe { __buffer.add(4).cast::<f32>().read() }
    }
    pub fn get_river_depth_at_spline_input_key(&self, in_key: f32) -> f32 {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::water::__FUNCTION_PTRS
                    .u_water_body_river_component_get_river_depth_at_spline_input_key,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&in_key, __buffer.add(0).cast::<f32>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::water::__FUNCTION_PTRS
                    .u_water_body_river_component_get_river_depth_at_spline_input_key,
                __buffer,
            )
        };
        unsafe { __buffer.add(4).cast::<f32>().read() }
    }
}
pub struct IWaterBrushActorInterface {}
#[repr(C, align(8))]
pub struct UWaterBrushActorInterface {
    __padding_end: [u8; 48],
}
impl UWaterBrushActorInterface {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UWaterBrushActorInterface")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UWaterBrushActorInterface")
            .copied()
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
pub struct UWaterMeshComponent {
    __padding_end: [u8; 2160],
}
impl UWaterMeshComponent {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UWaterMeshComponent")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UWaterMeshComponent")
            .copied()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
    pub fn is_enabled(&self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::water::__FUNCTION_PTRS
                    .u_water_mesh_component_is_enabled,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::water::__FUNCTION_PTRS
                    .u_water_mesh_component_is_enabled,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
}
#[repr(C, align(8))]
pub struct UWaterRuntimeSettings {
    __padding_end: [u8; 264],
}
impl UWaterRuntimeSettings {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UWaterRuntimeSettings")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UWaterRuntimeSettings")
            .copied()
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
pub struct UWaterSplineComponent {
    __padding_end: [u8; 2224],
}
impl UWaterSplineComponent {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UWaterSplineComponent")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UWaterSplineComponent")
            .copied()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
    pub fn k2_synchronize_and_broadcast_data_change(&mut self) {
        let mut __stack = crate::core_data::StackAlloc::<0>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::water::__FUNCTION_PTRS
                    .u_water_spline_component_k2_synchronize_and_broadcast_data_change,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::water::__FUNCTION_PTRS
                    .u_water_spline_component_k2_synchronize_and_broadcast_data_change,
                __buffer,
            )
        };
    }
}
#[repr(C, align(8))]
pub struct UWaterSplineMetadata {
    __padding_end: [u8; 208],
}
impl UWaterSplineMetadata {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UWaterSplineMetadata")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UWaterSplineMetadata")
            .copied()
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
pub struct UWaterSubsystem {
    __padding_end: [u8; 544],
}
impl UWaterSubsystem {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UWaterSubsystem")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UWaterSubsystem")
            .copied()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
    pub fn set_ocean_flood_height(&mut self, in_flood_height: f32) {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::water::__FUNCTION_PTRS
                    .u_water_subsystem_set_ocean_flood_height,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_flood_height,
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
                crate::bindings::water::__FUNCTION_PTRS
                    .u_water_subsystem_set_ocean_flood_height,
                __buffer,
            )
        };
    }
    pub fn print_to_water_log(&mut self, message: FString, b_warning: bool) {
        let mut __stack = crate::core_data::StackAlloc::<17>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::water::__FUNCTION_PTRS
                    .u_water_subsystem_print_to_water_log,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &message,
                __buffer.add(0).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_warning,
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
                crate::bindings::water::__FUNCTION_PTRS
                    .u_water_subsystem_print_to_water_log,
                __buffer,
            )
        };
    }
    pub fn is_water_rendering_enabled(&self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::water::__FUNCTION_PTRS
                    .u_water_subsystem_is_water_rendering_enabled,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::water::__FUNCTION_PTRS
                    .u_water_subsystem_is_water_rendering_enabled,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn is_underwater_post_process_enabled(&self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::water::__FUNCTION_PTRS
                    .u_water_subsystem_is_underwater_post_process_enabled,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::water::__FUNCTION_PTRS
                    .u_water_subsystem_is_underwater_post_process_enabled,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn is_shallow_water_simulation_enabled(&self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::water::__FUNCTION_PTRS
                    .u_water_subsystem_is_shallow_water_simulation_enabled,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::water::__FUNCTION_PTRS
                    .u_water_subsystem_is_shallow_water_simulation_enabled,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn get_water_time_seconds(&self) -> f32 {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::water::__FUNCTION_PTRS
                    .u_water_subsystem_get_water_time_seconds,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::water::__FUNCTION_PTRS
                    .u_water_subsystem_get_water_time_seconds,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<f32>().read() }
    }
    pub fn get_underwater_precise_trace_distance() -> f32 {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::water::__FUNCTION_PTRS
                    .u_water_subsystem_get_underwater_precise_trace_distance,
                __buffer,
            )
        };
        let __object_ptr = crate::bindings::water::UWaterSubsystem::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::water::__FUNCTION_PTRS
                    .u_water_subsystem_get_underwater_precise_trace_distance,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<f32>().read() }
    }
    pub fn get_underwater_collision_trace_distance() -> f32 {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::water::__FUNCTION_PTRS
                    .u_water_subsystem_get_underwater_collision_trace_distance,
                __buffer,
            )
        };
        let __object_ptr = crate::bindings::water::UWaterSubsystem::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::water::__FUNCTION_PTRS
                    .u_water_subsystem_get_underwater_collision_trace_distance,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<f32>().read() }
    }
    pub fn get_smoothed_world_time_seconds(&self) -> f32 {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::water::__FUNCTION_PTRS
                    .u_water_subsystem_get_smoothed_world_time_seconds,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::water::__FUNCTION_PTRS
                    .u_water_subsystem_get_smoothed_world_time_seconds,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<f32>().read() }
    }
    pub fn get_shallow_water_simulation_render_target_size() -> i32 {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::water::__FUNCTION_PTRS
                    .u_water_subsystem_get_shallow_water_simulation_render_target_size,
                __buffer,
            )
        };
        let __object_ptr = crate::bindings::water::UWaterSubsystem::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::water::__FUNCTION_PTRS
                    .u_water_subsystem_get_shallow_water_simulation_render_target_size,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<i32>().read() }
    }
    pub fn get_shallow_water_max_impulse_forces() -> i32 {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::water::__FUNCTION_PTRS
                    .u_water_subsystem_get_shallow_water_max_impulse_forces,
                __buffer,
            )
        };
        let __object_ptr = crate::bindings::water::UWaterSubsystem::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::water::__FUNCTION_PTRS
                    .u_water_subsystem_get_shallow_water_max_impulse_forces,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<i32>().read() }
    }
    pub fn get_shallow_water_max_dynamic_forces() -> i32 {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::water::__FUNCTION_PTRS
                    .u_water_subsystem_get_shallow_water_max_dynamic_forces,
                __buffer,
            )
        };
        let __object_ptr = crate::bindings::water::UWaterSubsystem::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::water::__FUNCTION_PTRS
                    .u_water_subsystem_get_shallow_water_max_dynamic_forces,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<i32>().read() }
    }
    pub fn get_ocean_total_height(&self) -> f32 {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::water::__FUNCTION_PTRS
                    .u_water_subsystem_get_ocean_total_height,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::water::__FUNCTION_PTRS
                    .u_water_subsystem_get_ocean_total_height,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<f32>().read() }
    }
    pub fn get_ocean_flood_height(&self) -> f32 {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::water::__FUNCTION_PTRS
                    .u_water_subsystem_get_ocean_flood_height,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::water::__FUNCTION_PTRS
                    .u_water_subsystem_get_ocean_flood_height,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<f32>().read() }
    }
    pub fn get_ocean_base_height(&self) -> f32 {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::water::__FUNCTION_PTRS
                    .u_water_subsystem_get_ocean_base_height,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::water::__FUNCTION_PTRS
                    .u_water_subsystem_get_ocean_base_height,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<f32>().read() }
    }
    pub fn get_camera_underwater_depth(&self) -> f32 {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::water::__FUNCTION_PTRS
                    .u_water_subsystem_get_camera_underwater_depth,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::water::__FUNCTION_PTRS
                    .u_water_subsystem_get_camera_underwater_depth,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<f32>().read() }
    }
}
#[repr(C, align(8))]
pub struct UWaterTerrainComponent {
    __padding_end: [u8; 288],
}
impl UWaterTerrainComponent {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UWaterTerrainComponent")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UWaterTerrainComponent")
            .copied()
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
pub struct UWaterWavesAsset {
    __padding_end: [u8; 88],
}
impl UWaterWavesAsset {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UWaterWavesAsset")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UWaterWavesAsset")
            .copied()
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
pub struct UWaterWavesAssetReference {
    __padding_end: [u8; 88],
}
impl UWaterWavesAssetReference {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UWaterWavesAssetReference")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UWaterWavesAssetReference")
            .copied()
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
pub struct AWaterZone {
    #[doc(hidden)]
    pub(crate) __padding_1136: [u8; 1136],
    pub water_info_texture_array: UPtr<
        crate::bindings::engine::UTextureRenderTarget2DArray,
    >,
    pub water_info_texture_array_num_slices: i32,
    #[doc(hidden)]
    pub(crate) __padding_1168: [u8; 20],
    pub render_target_resolution: crate::bindings::core_u_object::FIntPoint,
    pub water_mesh: UPtr<UWaterMeshComponent>,
    pub zone_extent: crate::bindings::core_u_object::FVector2D,
    __padding_end: [u8; 184],
}
impl AWaterZone {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("AWaterZone")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("AWaterZone")
            .copied()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
    pub fn set_far_mesh_material(
        &mut self,
        in_far_material: UPtr<crate::bindings::engine::UMaterialInterface>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::water::__FUNCTION_PTRS
                    .a_water_zone_set_far_mesh_material,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_far_material,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::engine::UMaterialInterface>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::water::__FUNCTION_PTRS
                    .a_water_zone_set_far_mesh_material,
                __buffer,
            )
        };
    }
    pub fn get_water_zone_index(&self) -> i32 {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::water::__FUNCTION_PTRS
                    .a_water_zone_get_water_zone_index,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::water::__FUNCTION_PTRS
                    .a_water_zone_get_water_zone_index,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<i32>().read() }
    }
}
#[repr(C, align(8))]
pub struct FBuoyancyComponent_OnEnteredWaterDelegate {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FBuoyancyComponent_OnExitedWaterDelegate {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FWaterSubsystem_OnCameraUnderwaterStateChanged {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FWaterSubsystem_OnWaterScalabilityChanged {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FWaterZone_OnWaterInfoTextureArrayCreated {
    _opague: [u8; 24],
}
#[repr(transparent)]
pub struct EWaterBrushBlendType(pub u8);
impl EWaterBrushBlendType {
    pub const ALPHA_BLEND: EWaterBrushBlendType = EWaterBrushBlendType(0);
    pub const MIN: EWaterBrushBlendType = EWaterBrushBlendType(1);
    pub const MAX: EWaterBrushBlendType = EWaterBrushBlendType(2);
    pub const ADDITIVE: EWaterBrushBlendType = EWaterBrushBlendType(3);
}
#[repr(transparent)]
pub struct EWaterBrushFalloffMode(pub u8);
impl EWaterBrushFalloffMode {
    pub const ANGLE: EWaterBrushFalloffMode = EWaterBrushFalloffMode(0);
    pub const WIDTH: EWaterBrushFalloffMode = EWaterBrushFalloffMode(1);
}
#[repr(transparent)]
pub struct EWaterBodyType(pub u8);
impl EWaterBodyType {
    pub const RIVER: EWaterBodyType = EWaterBodyType(0);
    pub const LAKE: EWaterBodyType = EWaterBodyType(1);
    pub const OCEAN: EWaterBodyType = EWaterBodyType(2);
    pub const TRANSITION: EWaterBodyType = EWaterBodyType(3);
    pub const NUM: EWaterBodyType = EWaterBodyType(4);
}
#[repr(transparent)]
pub struct EWaveSpectrumType(pub u8);
impl EWaveSpectrumType {
    pub const PHILLIPS: EWaveSpectrumType = EWaveSpectrumType(0);
    pub const PIERSON_MOSKOWITZ: EWaveSpectrumType = EWaveSpectrumType(1);
    pub const JONSWAP: EWaveSpectrumType = EWaveSpectrumType(2);
}
#[repr(transparent)]
pub struct EWaterExclusionMode(pub i32);
impl EWaterExclusionMode {
    pub const ADD_WATER_BODIES_LIST_TO_EXCLUSION: EWaterExclusionMode = EWaterExclusionMode(
        0,
    );
    pub const REMOVE_WATER_BODIES_LIST_FROM_EXCLUSION: EWaterExclusionMode = EWaterExclusionMode(
        1,
    );
}
