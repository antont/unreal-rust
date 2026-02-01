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
    pub a_caustics_generator_actor_spawn_water_preview_grid: *mut crate::ffi::UFunctionOpague,
    pub a_caustics_generator_actor_spawn_caustic_particle_grid: *mut crate::ffi::UFunctionOpague,
    pub a_caustics_generator_actor_set_editor_tick_enabled: *mut crate::ffi::UFunctionOpague,
    pub a_caustics_generator_actor_editor_tick: *mut crate::ffi::UFunctionOpague,
    pub u_jump_flood_component2_d_single_jump_step: *mut crate::ffi::UFunctionOpague,
    pub u_jump_flood_component2_d_single_blur_step: *mut crate::ffi::UFunctionOpague,
    pub u_jump_flood_component2_d_jump_flood: *mut crate::ffi::UFunctionOpague,
    pub u_jump_flood_component2_d_find_edges_debug: *mut crate::ffi::UFunctionOpague,
    pub u_jump_flood_component2_d_find_edges: *mut crate::ffi::UFunctionOpague,
    pub u_jump_flood_component2_d_create_mi_ds: *mut crate::ffi::UFunctionOpague,
    pub u_jump_flood_component2_d_assign_render_targets: *mut crate::ffi::UFunctionOpague,
    pub a_water_landscape_brush_set_water_body_cache: *mut crate::ffi::UFunctionOpague,
    pub a_water_landscape_brush_set_actor_cache: *mut crate::ffi::UFunctionOpague,
    pub a_water_landscape_brush_get_water_body_islands: *mut crate::ffi::UFunctionOpague,
    pub a_water_landscape_brush_get_water_body_cache: *mut crate::ffi::UFunctionOpague,
    pub a_water_landscape_brush_get_water_bodies: *mut crate::ffi::UFunctionOpague,
    pub a_water_landscape_brush_get_actors_affecting_landscape: *mut crate::ffi::UFunctionOpague,
    pub a_water_landscape_brush_get_actor_cache: *mut crate::ffi::UFunctionOpague,
    pub a_water_landscape_brush_force_water_texture_update: *mut crate::ffi::UFunctionOpague,
    pub a_water_landscape_brush_clear_water_body_cache: *mut crate::ffi::UFunctionOpague,
    pub a_water_landscape_brush_clear_actor_cache: *mut crate::ffi::UFunctionOpague,
    pub a_water_landscape_brush_blueprint_water_body_changed: *mut crate::ffi::UFunctionOpague,
    pub a_water_landscape_brush_blueprint_water_bodies_changed: *mut crate::ffi::UFunctionOpague,
    pub a_water_landscape_brush_blueprint_on_render_target_textures_updated: *mut crate::ffi::UFunctionOpague,
    pub a_water_landscape_brush_blueprint_get_render_targets: *mut crate::ffi::UFunctionOpague,
    pub a_water_brush_manager_sort_water_bodies_for_brush_render: *mut crate::ffi::UFunctionOpague,
    pub a_water_brush_manager_single_jump_step: *mut crate::ffi::UFunctionOpague,
    pub a_water_brush_manager_single_blur_step: *mut crate::ffi::UFunctionOpague,
    pub a_water_brush_manager_setup_default_materials: *mut crate::ffi::UFunctionOpague,
    pub a_water_brush_manager_get_water_cache_key: *mut crate::ffi::UFunctionOpague,
    pub a_water_brush_manager_force_update: *mut crate::ffi::UFunctionOpague,
    pub a_water_brush_manager_find_edges: *mut crate::ffi::UFunctionOpague,
}
impl FunctionPtrs {
    pub const fn empty() -> Self {
        Self {
            a_caustics_generator_actor_spawn_water_preview_grid: std::ptr::null_mut(),
            a_caustics_generator_actor_spawn_caustic_particle_grid: std::ptr::null_mut(),
            a_caustics_generator_actor_set_editor_tick_enabled: std::ptr::null_mut(),
            a_caustics_generator_actor_editor_tick: std::ptr::null_mut(),
            u_jump_flood_component2_d_single_jump_step: std::ptr::null_mut(),
            u_jump_flood_component2_d_single_blur_step: std::ptr::null_mut(),
            u_jump_flood_component2_d_jump_flood: std::ptr::null_mut(),
            u_jump_flood_component2_d_find_edges_debug: std::ptr::null_mut(),
            u_jump_flood_component2_d_find_edges: std::ptr::null_mut(),
            u_jump_flood_component2_d_create_mi_ds: std::ptr::null_mut(),
            u_jump_flood_component2_d_assign_render_targets: std::ptr::null_mut(),
            a_water_landscape_brush_set_water_body_cache: std::ptr::null_mut(),
            a_water_landscape_brush_set_actor_cache: std::ptr::null_mut(),
            a_water_landscape_brush_get_water_body_islands: std::ptr::null_mut(),
            a_water_landscape_brush_get_water_body_cache: std::ptr::null_mut(),
            a_water_landscape_brush_get_water_bodies: std::ptr::null_mut(),
            a_water_landscape_brush_get_actors_affecting_landscape: std::ptr::null_mut(),
            a_water_landscape_brush_get_actor_cache: std::ptr::null_mut(),
            a_water_landscape_brush_force_water_texture_update: std::ptr::null_mut(),
            a_water_landscape_brush_clear_water_body_cache: std::ptr::null_mut(),
            a_water_landscape_brush_clear_actor_cache: std::ptr::null_mut(),
            a_water_landscape_brush_blueprint_water_body_changed: std::ptr::null_mut(),
            a_water_landscape_brush_blueprint_water_bodies_changed: std::ptr::null_mut(),
            a_water_landscape_brush_blueprint_on_render_target_textures_updated: std::ptr::null_mut(),
            a_water_landscape_brush_blueprint_get_render_targets: std::ptr::null_mut(),
            a_water_brush_manager_sort_water_bodies_for_brush_render: std::ptr::null_mut(),
            a_water_brush_manager_single_jump_step: std::ptr::null_mut(),
            a_water_brush_manager_single_blur_step: std::ptr::null_mut(),
            a_water_brush_manager_setup_default_materials: std::ptr::null_mut(),
            a_water_brush_manager_get_water_cache_key: std::ptr::null_mut(),
            a_water_brush_manager_force_update: std::ptr::null_mut(),
            a_water_brush_manager_find_edges: std::ptr::null_mut(),
        }
    }
}
pub fn initialize() {
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = ACausticsGeneratorActor::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SpawnWaterPreviewGrid"),
                &raw mut __FUNCTION_PTRS
                    .a_caustics_generator_actor_spawn_water_preview_grid,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SpawnCausticParticleGrid"),
                &raw mut __FUNCTION_PTRS
                    .a_caustics_generator_actor_spawn_caustic_particle_grid,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetEditorTickEnabled"),
                &raw mut __FUNCTION_PTRS
                    .a_caustics_generator_actor_set_editor_tick_enabled,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("EditorTick"),
                &raw mut __FUNCTION_PTRS.a_caustics_generator_actor_editor_tick,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UJumpFloodComponent2D::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SingleJumpStep"),
                &raw mut __FUNCTION_PTRS.u_jump_flood_component2_d_single_jump_step,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SingleBlurStep"),
                &raw mut __FUNCTION_PTRS.u_jump_flood_component2_d_single_blur_step,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("JumpFlood"),
                &raw mut __FUNCTION_PTRS.u_jump_flood_component2_d_jump_flood,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("FindEdges_Debug"),
                &raw mut __FUNCTION_PTRS.u_jump_flood_component2_d_find_edges_debug,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("FindEdges"),
                &raw mut __FUNCTION_PTRS.u_jump_flood_component2_d_find_edges,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("CreateMIDs"),
                &raw mut __FUNCTION_PTRS.u_jump_flood_component2_d_create_mi_ds,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("AssignRenderTargets"),
                &raw mut __FUNCTION_PTRS.u_jump_flood_component2_d_assign_render_targets,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = AWaterLandscapeBrush::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetWaterBodyCache"),
                &raw mut __FUNCTION_PTRS.a_water_landscape_brush_set_water_body_cache,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetActorCache"),
                &raw mut __FUNCTION_PTRS.a_water_landscape_brush_set_actor_cache,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetWaterBodyIslands"),
                &raw mut __FUNCTION_PTRS.a_water_landscape_brush_get_water_body_islands,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetWaterBodyCache"),
                &raw mut __FUNCTION_PTRS.a_water_landscape_brush_get_water_body_cache,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetWaterBodies"),
                &raw mut __FUNCTION_PTRS.a_water_landscape_brush_get_water_bodies,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetActorsAffectingLandscape"),
                &raw mut __FUNCTION_PTRS
                    .a_water_landscape_brush_get_actors_affecting_landscape,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetActorCache"),
                &raw mut __FUNCTION_PTRS.a_water_landscape_brush_get_actor_cache,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ForceWaterTextureUpdate"),
                &raw mut __FUNCTION_PTRS
                    .a_water_landscape_brush_force_water_texture_update,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ClearWaterBodyCache"),
                &raw mut __FUNCTION_PTRS.a_water_landscape_brush_clear_water_body_cache,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ClearActorCache"),
                &raw mut __FUNCTION_PTRS.a_water_landscape_brush_clear_actor_cache,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("BlueprintWaterBodyChanged"),
                &raw mut __FUNCTION_PTRS
                    .a_water_landscape_brush_blueprint_water_body_changed,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("BlueprintWaterBodiesChanged"),
                &raw mut __FUNCTION_PTRS
                    .a_water_landscape_brush_blueprint_water_bodies_changed,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("BlueprintOnRenderTargetTexturesUpdated"),
                &raw mut __FUNCTION_PTRS
                    .a_water_landscape_brush_blueprint_on_render_target_textures_updated,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("BlueprintGetRenderTargets"),
                &raw mut __FUNCTION_PTRS
                    .a_water_landscape_brush_blueprint_get_render_targets,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = AWaterBrushManager::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SortWaterBodiesForBrushRender"),
                &raw mut __FUNCTION_PTRS
                    .a_water_brush_manager_sort_water_bodies_for_brush_render,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SingleJumpStep"),
                &raw mut __FUNCTION_PTRS.a_water_brush_manager_single_jump_step,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SingleBlurStep"),
                &raw mut __FUNCTION_PTRS.a_water_brush_manager_single_blur_step,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetupDefaultMaterials"),
                &raw mut __FUNCTION_PTRS.a_water_brush_manager_setup_default_materials,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetWaterCacheKey"),
                &raw mut __FUNCTION_PTRS.a_water_brush_manager_get_water_cache_key,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ForceUpdate"),
                &raw mut __FUNCTION_PTRS.a_water_brush_manager_force_update,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("FindEdges"),
                &raw mut __FUNCTION_PTRS.a_water_brush_manager_find_edges,
            );
        }
    }
}
#[repr(C, align(8))]
pub struct FWaterBodyBrushCache {
    pub cache_render_target: UPtr<crate::bindings::engine::UTextureRenderTarget2D>,
    pub cache_is_valid: bool,
}
impl FWaterBodyBrushCache {}
#[repr(C, align(8))]
pub struct UAssetDefinition_WaterWaves {
    __padding_end: [u8; 72],
}
impl UAssetDefinition_WaterWaves {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAssetDefinition_WaterWaves")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAssetDefinition_WaterWaves")
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
pub struct ACausticsGeneratorActor {
    #[doc(hidden)]
    pub(crate) __padding_1136: [u8; 1136],
    pub default_scene_root: UPtr<crate::bindings::engine::USceneComponent>,
    __padding_end: [u8; 8],
}
impl ACausticsGeneratorActor {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("ACausticsGeneratorActor")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("ACausticsGeneratorActor")
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
    pub fn spawn_water_preview_grid(
        &mut self,
        hismc: UPtr<crate::bindings::engine::UHierarchicalInstancedStaticMeshComponent>,
        grid_size: f32,
        grid_tiles: i32,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::water_editor::__FUNCTION_PTRS
                    .a_caustics_generator_actor_spawn_water_preview_grid,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &hismc,
                __buffer
                    .add(0)
                    .cast::<
                        UPtr<
                            crate::bindings::engine::UHierarchicalInstancedStaticMeshComponent,
                        >,
                    >(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&grid_size, __buffer.add(8).cast::<f32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &grid_tiles,
                __buffer.add(12).cast::<i32>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::water_editor::__FUNCTION_PTRS
                    .a_caustics_generator_actor_spawn_water_preview_grid,
                __buffer,
            )
        };
    }
    pub fn spawn_caustic_particle_grid(
        &mut self,
        hismc: UPtr<crate::bindings::engine::UHierarchicalInstancedStaticMeshComponent>,
        grid_size: f32,
        grid_tiles: i32,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::water_editor::__FUNCTION_PTRS
                    .a_caustics_generator_actor_spawn_caustic_particle_grid,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &hismc,
                __buffer
                    .add(0)
                    .cast::<
                        UPtr<
                            crate::bindings::engine::UHierarchicalInstancedStaticMeshComponent,
                        >,
                    >(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&grid_size, __buffer.add(8).cast::<f32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &grid_tiles,
                __buffer.add(12).cast::<i32>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::water_editor::__FUNCTION_PTRS
                    .a_caustics_generator_actor_spawn_caustic_particle_grid,
                __buffer,
            )
        };
    }
    pub fn set_editor_tick_enabled(&mut self, b_enabled: bool) {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::water_editor::__FUNCTION_PTRS
                    .a_caustics_generator_actor_set_editor_tick_enabled,
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
                crate::bindings::water_editor::__FUNCTION_PTRS
                    .a_caustics_generator_actor_set_editor_tick_enabled,
                __buffer,
            )
        };
    }
    pub fn editor_tick(&mut self, delta_seconds: f32) {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::water_editor::__FUNCTION_PTRS
                    .a_caustics_generator_actor_editor_tick,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &delta_seconds,
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
                crate::bindings::water_editor::__FUNCTION_PTRS
                    .a_caustics_generator_actor_editor_tick,
                __buffer,
            )
        };
    }
}
#[repr(C, align(8))]
pub struct UJumpFloodComponent2D {
    #[doc(hidden)]
    pub(crate) __padding_240: [u8; 240],
    pub jump_step_material: UPtr<crate::bindings::engine::UMaterialInterface>,
    pub find_edges_material: UPtr<crate::bindings::engine::UMaterialInterface>,
    pub blur_edges_material: UPtr<crate::bindings::engine::UMaterialInterface>,
    pub use_blur: bool,
    pub blur_passes: i32,
    __padding_end: [u8; 56],
}
impl UJumpFloodComponent2D {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UJumpFloodComponent2D")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UJumpFloodComponent2D")
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
    pub fn single_jump_step(
        &mut self,
    ) -> UPtr<crate::bindings::engine::UTextureRenderTarget2D> {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::water_editor::__FUNCTION_PTRS
                    .u_jump_flood_component2_d_single_jump_step,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::water_editor::__FUNCTION_PTRS
                    .u_jump_flood_component2_d_single_jump_step,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(0)
                .cast::<UPtr<crate::bindings::engine::UTextureRenderTarget2D>>()
                .read()
        }
    }
    pub fn single_blur_step(
        &mut self,
    ) -> UPtr<crate::bindings::engine::UTextureRenderTarget2D> {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::water_editor::__FUNCTION_PTRS
                    .u_jump_flood_component2_d_single_blur_step,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::water_editor::__FUNCTION_PTRS
                    .u_jump_flood_component2_d_single_blur_step,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(0)
                .cast::<UPtr<crate::bindings::engine::UTextureRenderTarget2D>>()
                .read()
        }
    }
    pub fn jump_flood(
        &mut self,
        seed_rt: UPtr<crate::bindings::engine::UTextureRenderTarget2D>,
        scene_capture_z: f32,
        curl: crate::bindings::core_u_object::FLinearColor,
        use_depth: bool,
        zx_location_t: f32,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<36>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::water_editor::__FUNCTION_PTRS
                    .u_jump_flood_component2_d_jump_flood,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &seed_rt,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::engine::UTextureRenderTarget2D>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &scene_capture_z,
                __buffer.add(8).cast::<f32>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &curl,
                __buffer.add(12).cast::<crate::bindings::core_u_object::FLinearColor>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &use_depth,
                __buffer.add(28).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &zx_location_t,
                __buffer.add(32).cast::<f32>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::water_editor::__FUNCTION_PTRS
                    .u_jump_flood_component2_d_jump_flood,
                __buffer,
            )
        };
    }
    pub fn find_edges_debug(
        &mut self,
        seed_rt: UPtr<crate::bindings::engine::UTextureRenderTarget2D>,
        capture_z: f32,
        curl: crate::bindings::core_u_object::FLinearColor,
        dest_rt: UPtr<crate::bindings::engine::UTextureRenderTarget2D>,
        z_offset: f32,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<44>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::water_editor::__FUNCTION_PTRS
                    .u_jump_flood_component2_d_find_edges_debug,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &seed_rt,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::engine::UTextureRenderTarget2D>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&capture_z, __buffer.add(8).cast::<f32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &curl,
                __buffer.add(12).cast::<crate::bindings::core_u_object::FLinearColor>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &dest_rt,
                __buffer
                    .add(32)
                    .cast::<UPtr<crate::bindings::engine::UTextureRenderTarget2D>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&z_offset, __buffer.add(40).cast::<f32>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::water_editor::__FUNCTION_PTRS
                    .u_jump_flood_component2_d_find_edges_debug,
                __buffer,
            )
        };
    }
    pub fn find_edges(
        &mut self,
        seed_rt: UPtr<crate::bindings::engine::UTextureRenderTarget2D>,
        capture_z: f32,
        curl: crate::bindings::core_u_object::FLinearColor,
        use_depth: bool,
        zx_location_t: f32,
    ) -> UPtr<crate::bindings::engine::UTextureRenderTarget2D> {
        let mut __stack = crate::core_data::StackAlloc::<48>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::water_editor::__FUNCTION_PTRS
                    .u_jump_flood_component2_d_find_edges,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &seed_rt,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::engine::UTextureRenderTarget2D>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&capture_z, __buffer.add(8).cast::<f32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &curl,
                __buffer.add(12).cast::<crate::bindings::core_u_object::FLinearColor>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &use_depth,
                __buffer.add(28).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &zx_location_t,
                __buffer.add(32).cast::<f32>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::water_editor::__FUNCTION_PTRS
                    .u_jump_flood_component2_d_find_edges,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(40)
                .cast::<UPtr<crate::bindings::engine::UTextureRenderTarget2D>>()
                .read()
        }
    }
    pub fn create_mi_ds(&mut self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::water_editor::__FUNCTION_PTRS
                    .u_jump_flood_component2_d_create_mi_ds,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::water_editor::__FUNCTION_PTRS
                    .u_jump_flood_component2_d_create_mi_ds,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn assign_render_targets(
        &mut self,
        in_rta: UPtr<crate::bindings::engine::UTextureRenderTarget2D>,
        in_rtb: UPtr<crate::bindings::engine::UTextureRenderTarget2D>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::water_editor::__FUNCTION_PTRS
                    .u_jump_flood_component2_d_assign_render_targets,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_rta,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::engine::UTextureRenderTarget2D>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_rtb,
                __buffer
                    .add(8)
                    .cast::<UPtr<crate::bindings::engine::UTextureRenderTarget2D>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::water_editor::__FUNCTION_PTRS
                    .u_jump_flood_component2_d_assign_render_targets,
                __buffer,
            )
        };
    }
}
#[repr(C, align(8))]
pub struct UWaterBodyActorFactory {
    __padding_end: [u8; 144],
}
impl UWaterBodyActorFactory {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UWaterBodyActorFactory")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UWaterBodyActorFactory")
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
pub struct UWaterBodyRiverActorFactory {
    __padding_end: [u8; 144],
}
impl UWaterBodyRiverActorFactory {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UWaterBodyRiverActorFactory")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UWaterBodyRiverActorFactory")
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
pub struct UWaterBodyOceanActorFactory {
    __padding_end: [u8; 144],
}
impl UWaterBodyOceanActorFactory {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UWaterBodyOceanActorFactory")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UWaterBodyOceanActorFactory")
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
pub struct UWaterBodyLakeActorFactory {
    __padding_end: [u8; 144],
}
impl UWaterBodyLakeActorFactory {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UWaterBodyLakeActorFactory")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UWaterBodyLakeActorFactory")
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
pub struct UWaterBodyCustomActorFactory {
    __padding_end: [u8; 144],
}
impl UWaterBodyCustomActorFactory {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UWaterBodyCustomActorFactory")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UWaterBodyCustomActorFactory")
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
pub struct UWaterBodyBrushCacheContainerThumbnailRenderer {
    __padding_end: [u8; 48],
}
impl UWaterBodyBrushCacheContainerThumbnailRenderer {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UWaterBodyBrushCacheContainerThumbnailRenderer")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UWaterBodyBrushCacheContainerThumbnailRenderer")
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
pub struct UWaterBodyIslandActorFactory {
    __padding_end: [u8; 144],
}
impl UWaterBodyIslandActorFactory {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UWaterBodyIslandActorFactory")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UWaterBodyIslandActorFactory")
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
pub struct UWaterBodyBrushCacheContainer {
    #[doc(hidden)]
    pub(crate) __padding_48: [u8; 48],
    pub cache: FWaterBodyBrushCache,
}
impl UWaterBodyBrushCacheContainer {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UWaterBodyBrushCacheContainer")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UWaterBodyBrushCacheContainer")
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
pub struct AWaterLandscapeBrush {
    __padding_end: [u8; 1488],
}
impl AWaterLandscapeBrush {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("AWaterLandscapeBrush")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("AWaterLandscapeBrush")
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
    pub fn set_water_body_cache(
        &mut self,
        water_body: UPtr<crate::bindings::water::AWaterBody>,
        in_cache: UPtr<crate::bindings::core_u_object::UObject>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::water_editor::__FUNCTION_PTRS
                    .a_water_landscape_brush_set_water_body_cache,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &water_body,
                __buffer.add(0).cast::<UPtr<crate::bindings::water::AWaterBody>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_cache,
                __buffer.add(8).cast::<UPtr<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::water_editor::__FUNCTION_PTRS
                    .a_water_landscape_brush_set_water_body_cache,
                __buffer,
            )
        };
    }
    pub fn set_actor_cache(
        &mut self,
        in_actor: UPtr<crate::bindings::engine::AActor>,
        in_cache: UPtr<crate::bindings::core_u_object::UObject>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::water_editor::__FUNCTION_PTRS
                    .a_water_landscape_brush_set_actor_cache,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_actor,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::AActor>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_cache,
                __buffer.add(8).cast::<UPtr<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::water_editor::__FUNCTION_PTRS
                    .a_water_landscape_brush_set_actor_cache,
                __buffer,
            )
        };
    }
    pub fn get_water_body_islands(
        &self,
        water_body_island_class: TSubclassOf<crate::bindings::water::AWaterBodyIsland>,
        out_water_body_islands: &mut TArray<
            UPtr<crate::bindings::water::AWaterBodyIsland>,
        >,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::water_editor::__FUNCTION_PTRS
                    .a_water_landscape_brush_get_water_body_islands,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &water_body_island_class,
                __buffer
                    .add(0)
                    .cast::<TSubclassOf<crate::bindings::water::AWaterBodyIsland>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_water_body_islands,
                __buffer
                    .add(8)
                    .cast::<TArray<UPtr<crate::bindings::water::AWaterBodyIsland>>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::water_editor::__FUNCTION_PTRS
                    .a_water_landscape_brush_get_water_body_islands,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(8)
                .cast::<TArray<UPtr<crate::bindings::water::AWaterBodyIsland>>>()
                .swap(out_water_body_islands);
        }
    }
    pub fn get_water_body_cache(
        &self,
        water_body: UPtr<crate::bindings::water::AWaterBody>,
        cache_class: TSubclassOf<crate::bindings::core_u_object::UObject>,
    ) -> UPtr<crate::bindings::core_u_object::UObject> {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::water_editor::__FUNCTION_PTRS
                    .a_water_landscape_brush_get_water_body_cache,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &water_body,
                __buffer.add(0).cast::<UPtr<crate::bindings::water::AWaterBody>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &cache_class,
                __buffer
                    .add(8)
                    .cast::<TSubclassOf<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::water_editor::__FUNCTION_PTRS
                    .a_water_landscape_brush_get_water_body_cache,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(16)
                .cast::<UPtr<crate::bindings::core_u_object::UObject>>()
                .read()
        }
    }
    pub fn get_water_bodies(
        &self,
        water_body_class: TSubclassOf<crate::bindings::water::AWaterBody>,
        out_water_bodies: &mut TArray<UPtr<crate::bindings::water::AWaterBody>>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::water_editor::__FUNCTION_PTRS
                    .a_water_landscape_brush_get_water_bodies,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &water_body_class,
                __buffer
                    .add(0)
                    .cast::<TSubclassOf<crate::bindings::water::AWaterBody>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_water_bodies,
                __buffer
                    .add(8)
                    .cast::<TArray<UPtr<crate::bindings::water::AWaterBody>>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::water_editor::__FUNCTION_PTRS
                    .a_water_landscape_brush_get_water_bodies,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(8)
                .cast::<TArray<UPtr<crate::bindings::water::AWaterBody>>>()
                .swap(out_water_bodies);
        }
    }
    pub fn get_actors_affecting_landscape(
        &self,
        out_water_brush_actors: &mut TArray<
            TScriptInterface<crate::bindings::water::UWaterBrushActorInterface>,
        >,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::water_editor::__FUNCTION_PTRS
                    .a_water_landscape_brush_get_actors_affecting_landscape,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_water_brush_actors,
                __buffer
                    .add(0)
                    .cast::<
                        TArray<
                            TScriptInterface<
                                crate::bindings::water::UWaterBrushActorInterface,
                            >,
                        >,
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
                crate::bindings::water_editor::__FUNCTION_PTRS
                    .a_water_landscape_brush_get_actors_affecting_landscape,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(0)
                .cast::<
                    TArray<
                        TScriptInterface<
                            crate::bindings::water::UWaterBrushActorInterface,
                        >,
                    >,
                >()
                .swap(out_water_brush_actors);
        }
    }
    pub fn get_actor_cache(
        &self,
        in_actor: UPtr<crate::bindings::engine::AActor>,
        cache_class: TSubclassOf<crate::bindings::core_u_object::UObject>,
    ) -> UPtr<crate::bindings::core_u_object::UObject> {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::water_editor::__FUNCTION_PTRS
                    .a_water_landscape_brush_get_actor_cache,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_actor,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::AActor>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &cache_class,
                __buffer
                    .add(8)
                    .cast::<TSubclassOf<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::water_editor::__FUNCTION_PTRS
                    .a_water_landscape_brush_get_actor_cache,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(16)
                .cast::<UPtr<crate::bindings::core_u_object::UObject>>()
                .read()
        }
    }
    pub fn force_water_texture_update(&mut self) {
        let mut __stack = crate::core_data::StackAlloc::<0>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::water_editor::__FUNCTION_PTRS
                    .a_water_landscape_brush_force_water_texture_update,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::water_editor::__FUNCTION_PTRS
                    .a_water_landscape_brush_force_water_texture_update,
                __buffer,
            )
        };
    }
    pub fn clear_water_body_cache(
        &mut self,
        water_body: UPtr<crate::bindings::water::AWaterBody>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::water_editor::__FUNCTION_PTRS
                    .a_water_landscape_brush_clear_water_body_cache,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &water_body,
                __buffer.add(0).cast::<UPtr<crate::bindings::water::AWaterBody>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::water_editor::__FUNCTION_PTRS
                    .a_water_landscape_brush_clear_water_body_cache,
                __buffer,
            )
        };
    }
    pub fn clear_actor_cache(
        &mut self,
        in_actor: UPtr<crate::bindings::engine::AActor>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::water_editor::__FUNCTION_PTRS
                    .a_water_landscape_brush_clear_actor_cache,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_actor,
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
                crate::bindings::water_editor::__FUNCTION_PTRS
                    .a_water_landscape_brush_clear_actor_cache,
                __buffer,
            )
        };
    }
    pub fn blueprint_water_body_changed(
        &mut self,
        actor: UPtr<crate::bindings::engine::AActor>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::water_editor::__FUNCTION_PTRS
                    .a_water_landscape_brush_blueprint_water_body_changed,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &actor,
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
                crate::bindings::water_editor::__FUNCTION_PTRS
                    .a_water_landscape_brush_blueprint_water_body_changed,
                __buffer,
            )
        };
    }
    pub fn blueprint_water_bodies_changed(&mut self) {
        let mut __stack = crate::core_data::StackAlloc::<0>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::water_editor::__FUNCTION_PTRS
                    .a_water_landscape_brush_blueprint_water_bodies_changed,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::water_editor::__FUNCTION_PTRS
                    .a_water_landscape_brush_blueprint_water_bodies_changed,
                __buffer,
            )
        };
    }
    pub fn blueprint_on_render_target_textures_updated(
        &mut self,
        velocity_texture: UPtr<crate::bindings::engine::UTexture2D>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::water_editor::__FUNCTION_PTRS
                    .a_water_landscape_brush_blueprint_on_render_target_textures_updated,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &velocity_texture,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::UTexture2D>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::water_editor::__FUNCTION_PTRS
                    .a_water_landscape_brush_blueprint_on_render_target_textures_updated,
                __buffer,
            )
        };
    }
    pub fn blueprint_get_render_targets(
        &mut self,
        in_height_render_target: UPtr<crate::bindings::engine::UTextureRenderTarget2D>,
        out_velocity_render_target: &mut UPtr<
            crate::bindings::engine::UTextureRenderTarget2D,
        >,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::water_editor::__FUNCTION_PTRS
                    .a_water_landscape_brush_blueprint_get_render_targets,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_height_render_target,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::engine::UTextureRenderTarget2D>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_velocity_render_target,
                __buffer
                    .add(8)
                    .cast::<UPtr<crate::bindings::engine::UTextureRenderTarget2D>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::water_editor::__FUNCTION_PTRS
                    .a_water_landscape_brush_blueprint_get_render_targets,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(8)
                .cast::<UPtr<crate::bindings::engine::UTextureRenderTarget2D>>()
                .swap(out_velocity_render_target);
        }
    }
}
#[repr(C, align(16))]
pub struct AWaterBrushManager {
    #[doc(hidden)]
    pub(crate) __padding_1584: [u8; 1584],
    pub brush_angle_falloff_material: UPtr<crate::bindings::engine::UMaterialInterface>,
    pub brush_width_falloff_material: UPtr<crate::bindings::engine::UMaterialInterface>,
    pub distance_field_cache_material: UPtr<crate::bindings::engine::UMaterialInterface>,
    pub render_river_spline_depth_material: UPtr<
        crate::bindings::engine::UMaterialInterface,
    >,
    pub debug_distance_field_material: UPtr<crate::bindings::engine::UMaterialInterface>,
    pub weightmap_material: UPtr<crate::bindings::engine::UMaterialInterface>,
    pub draw_canvas_material: UPtr<crate::bindings::engine::UMaterialInterface>,
    pub composite_water_body_texture_material: UPtr<
        crate::bindings::engine::UMaterialInterface,
    >,
    pub island_falloff_material: UPtr<crate::bindings::engine::UMaterialInterface>,
    pub jump_step_material: UPtr<crate::bindings::engine::UMaterialInterface>,
    pub find_edges_material: UPtr<crate::bindings::engine::UMaterialInterface>,
    pub blur_edges_material: UPtr<crate::bindings::engine::UMaterialInterface>,
    pub brush_angle_falloff_mid: UPtr<crate::bindings::engine::UMaterialInstanceDynamic>,
    pub brush_width_falloff_mid: UPtr<crate::bindings::engine::UMaterialInstanceDynamic>,
    pub distance_field_cache_mid: UPtr<
        crate::bindings::engine::UMaterialInstanceDynamic,
    >,
    pub river_spline_mi_ds: TArray<
        UPtr<crate::bindings::engine::UMaterialInstanceDynamic>,
    >,
    pub debug_distance_field_mid: UPtr<
        crate::bindings::engine::UMaterialInstanceDynamic,
    >,
    pub weightmap_mid: UPtr<crate::bindings::engine::UMaterialInstanceDynamic>,
    pub draw_canvas_mid: UPtr<crate::bindings::engine::UMaterialInstanceDynamic>,
    pub composite_water_body_texture_mid: UPtr<
        crate::bindings::engine::UMaterialInstanceDynamic,
    >,
    pub island_falloff_mid: UPtr<crate::bindings::engine::UMaterialInstanceDynamic>,
    pub brush_curve_rt_cache: TMap<
        UPtr<crate::bindings::engine::UCurveFloat>,
        FWaterBodyBrushCache,
    >,
    pub world_size: crate::bindings::core_u_object::FVector,
    pub landscape_rt_res: crate::bindings::core_u_object::FIntPoint,
    pub landscape_quads: crate::bindings::core_u_object::FIntPoint,
    pub landscape_transform: crate::bindings::core_u_object::FTransform,
    pub show_gradient: bool,
    pub distance_divisor: f32,
    pub show_distance: bool,
    pub show_grid: bool,
    pub canvas_segment_size: f32,
    pub water_clear_height: f32,
    pub spline_mesh_extension: f32,
    pub use_dynamic_preview_rt: bool,
    pub disable_brush_texture_effects: bool,
    __padding_end: [u8; 54],
}
impl AWaterBrushManager {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("AWaterBrushManager")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("AWaterBrushManager")
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
    pub fn sort_water_bodies_for_brush_render(
        &self,
        in_out_water_bodies: &mut TArray<UPtr<crate::bindings::water::AWaterBody>>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::water_editor::__FUNCTION_PTRS
                    .a_water_brush_manager_sort_water_bodies_for_brush_render,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_out_water_bodies,
                __buffer
                    .add(0)
                    .cast::<TArray<UPtr<crate::bindings::water::AWaterBody>>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::water_editor::__FUNCTION_PTRS
                    .a_water_brush_manager_sort_water_bodies_for_brush_render,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(0)
                .cast::<TArray<UPtr<crate::bindings::water::AWaterBody>>>()
                .swap(in_out_water_bodies);
        }
    }
    pub fn single_jump_step(&mut self) {
        let mut __stack = crate::core_data::StackAlloc::<0>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::water_editor::__FUNCTION_PTRS
                    .a_water_brush_manager_single_jump_step,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::water_editor::__FUNCTION_PTRS
                    .a_water_brush_manager_single_jump_step,
                __buffer,
            )
        };
    }
    pub fn single_blur_step(&mut self) {
        let mut __stack = crate::core_data::StackAlloc::<0>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::water_editor::__FUNCTION_PTRS
                    .a_water_brush_manager_single_blur_step,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::water_editor::__FUNCTION_PTRS
                    .a_water_brush_manager_single_blur_step,
                __buffer,
            )
        };
    }
    pub fn setup_default_materials(&mut self) {
        let mut __stack = crate::core_data::StackAlloc::<0>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::water_editor::__FUNCTION_PTRS
                    .a_water_brush_manager_setup_default_materials,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::water_editor::__FUNCTION_PTRS
                    .a_water_brush_manager_setup_default_materials,
                __buffer,
            )
        };
    }
    pub fn get_water_cache_key(
        &mut self,
        water_brush: UPtr<crate::bindings::engine::AActor>,
        container_object: &mut UPtr<UWaterBodyBrushCacheContainer>,
        value: &mut FWaterBodyBrushCache,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::water_editor::__FUNCTION_PTRS
                    .a_water_brush_manager_get_water_cache_key,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &water_brush,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::AActor>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                container_object,
                __buffer.add(8).cast::<UPtr<UWaterBodyBrushCacheContainer>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                value,
                __buffer.add(16).cast::<FWaterBodyBrushCache>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::water_editor::__FUNCTION_PTRS
                    .a_water_brush_manager_get_water_cache_key,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(8)
                .cast::<UPtr<UWaterBodyBrushCacheContainer>>()
                .swap(container_object);
        }
        unsafe {
            __buffer.add(16).cast::<FWaterBodyBrushCache>().swap(value);
        }
    }
    pub fn force_update(&mut self) {
        let mut __stack = crate::core_data::StackAlloc::<0>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::water_editor::__FUNCTION_PTRS
                    .a_water_brush_manager_force_update,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::water_editor::__FUNCTION_PTRS
                    .a_water_brush_manager_force_update,
                __buffer,
            )
        };
    }
    pub fn find_edges(&mut self) {
        let mut __stack = crate::core_data::StackAlloc::<0>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::water_editor::__FUNCTION_PTRS
                    .a_water_brush_manager_find_edges,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::water_editor::__FUNCTION_PTRS
                    .a_water_brush_manager_find_edges,
                __buffer,
            )
        };
    }
}
#[repr(C, align(8))]
pub struct UWaterBrushManagerFactory {
    __padding_end: [u8; 144],
}
impl UWaterBrushManagerFactory {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UWaterBrushManagerFactory")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UWaterBrushManagerFactory")
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
pub struct UWaterEditorSettings {
    __padding_end: [u8; 2960],
}
impl UWaterEditorSettings {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UWaterEditorSettings")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UWaterEditorSettings")
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
pub struct UWaterEditorSubsystem {
    __padding_end: [u8; 256],
}
impl UWaterEditorSubsystem {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UWaterEditorSubsystem")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UWaterEditorSubsystem")
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
pub struct UWaterSplineComponentVisualizerSelectionState {
    __padding_end: [u8; 272],
}
impl UWaterSplineComponentVisualizerSelectionState {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UWaterSplineComponentVisualizerSelectionState")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UWaterSplineComponentVisualizerSelectionState")
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
pub struct UWaterSplineMetadataDetailsFactory {
    __padding_end: [u8; 48],
}
impl UWaterSplineMetadataDetailsFactory {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UWaterSplineMetadataDetailsFactory")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UWaterSplineMetadataDetailsFactory")
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
pub struct UWaterWavesAssetFactory {
    __padding_end: [u8; 136],
}
impl UWaterWavesAssetFactory {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UWaterWavesAssetFactory")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UWaterWavesAssetFactory")
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
pub struct UWaterZoneActorFactory {
    __padding_end: [u8; 144],
}
impl UWaterZoneActorFactory {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UWaterZoneActorFactory")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UWaterZoneActorFactory")
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
