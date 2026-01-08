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
    pub u_skeletal_mesh_editor_subsystem_strip_lod_geometry: *mut crate::ffi::UFunctionOpague,
    pub u_skeletal_mesh_editor_subsystem_set_skeletal_mesh_overlay_material: *mut crate::ffi::UFunctionOpague,
    pub u_skeletal_mesh_editor_subsystem_set_section_visible_in_ray_tracing: *mut crate::ffi::UFunctionOpague,
    pub u_skeletal_mesh_editor_subsystem_set_section_recompute_tangents_vertex_mask_channel: *mut crate::ffi::UFunctionOpague,
    pub u_skeletal_mesh_editor_subsystem_set_section_recompute_tangent: *mut crate::ffi::UFunctionOpague,
    pub u_skeletal_mesh_editor_subsystem_set_section_cast_shadow: *mut crate::ffi::UFunctionOpague,
    pub u_skeletal_mesh_editor_subsystem_set_morph_targets_to_generated_by_engine_for_all_skeletal_mesh: *mut crate::ffi::UFunctionOpague,
    pub u_skeletal_mesh_editor_subsystem_set_morph_targets_to_generated_by_engine: *mut crate::ffi::UFunctionOpague,
    pub u_skeletal_mesh_editor_subsystem_set_material_slot_overlay_material: *mut crate::ffi::UFunctionOpague,
    pub u_skeletal_mesh_editor_subsystem_set_lod_build_settings: *mut crate::ffi::UFunctionOpague,
    pub u_skeletal_mesh_editor_subsystem_rename_socket: *mut crate::ffi::UFunctionOpague,
    pub u_skeletal_mesh_editor_subsystem_remove_lo_ds: *mut crate::ffi::UFunctionOpague,
    pub u_skeletal_mesh_editor_subsystem_reimport_all_custom_lo_ds: *mut crate::ffi::UFunctionOpague,
    pub u_skeletal_mesh_editor_subsystem_regenerate_lod: *mut crate::ffi::UFunctionOpague,
    pub u_skeletal_mesh_editor_subsystem_is_physics_asset_compatible: *mut crate::ffi::UFunctionOpague,
    pub u_skeletal_mesh_editor_subsystem_import_lod: *mut crate::ffi::UFunctionOpague,
    pub u_skeletal_mesh_editor_subsystem_get_skeleton_curve_meta_data_names: *mut crate::ffi::UFunctionOpague,
    pub u_skeletal_mesh_editor_subsystem_get_skeletal_mesh_overlay_material: *mut crate::ffi::UFunctionOpague,
    pub u_skeletal_mesh_editor_subsystem_get_section_visible_in_ray_tracing: *mut crate::ffi::UFunctionOpague,
    pub u_skeletal_mesh_editor_subsystem_get_section_recompute_tangents_vertex_mask_channel: *mut crate::ffi::UFunctionOpague,
    pub u_skeletal_mesh_editor_subsystem_get_section_recompute_tangent: *mut crate::ffi::UFunctionOpague,
    pub u_skeletal_mesh_editor_subsystem_get_section_cast_shadow: *mut crate::ffi::UFunctionOpague,
    pub u_skeletal_mesh_editor_subsystem_get_num_verts: *mut crate::ffi::UFunctionOpague,
    pub u_skeletal_mesh_editor_subsystem_get_num_sections: *mut crate::ffi::UFunctionOpague,
    pub u_skeletal_mesh_editor_subsystem_get_morph_targets_generated_by_engine: *mut crate::ffi::UFunctionOpague,
    pub u_skeletal_mesh_editor_subsystem_get_material_slot_overlay_material: *mut crate::ffi::UFunctionOpague,
    pub u_skeletal_mesh_editor_subsystem_get_lod_material_slot: *mut crate::ffi::UFunctionOpague,
    pub u_skeletal_mesh_editor_subsystem_get_lod_count: *mut crate::ffi::UFunctionOpague,
    pub u_skeletal_mesh_editor_subsystem_get_lod_build_settings: *mut crate::ffi::UFunctionOpague,
    pub u_skeletal_mesh_editor_subsystem_create_physics_asset: *mut crate::ffi::UFunctionOpague,
    pub u_skeletal_mesh_editor_subsystem_assign_physics_asset: *mut crate::ffi::UFunctionOpague,
}
impl FunctionPtrs {
    pub const fn empty() -> Self {
        Self {
            u_skeletal_mesh_editor_subsystem_strip_lod_geometry: std::ptr::null_mut(),
            u_skeletal_mesh_editor_subsystem_set_skeletal_mesh_overlay_material: std::ptr::null_mut(),
            u_skeletal_mesh_editor_subsystem_set_section_visible_in_ray_tracing: std::ptr::null_mut(),
            u_skeletal_mesh_editor_subsystem_set_section_recompute_tangents_vertex_mask_channel: std::ptr::null_mut(),
            u_skeletal_mesh_editor_subsystem_set_section_recompute_tangent: std::ptr::null_mut(),
            u_skeletal_mesh_editor_subsystem_set_section_cast_shadow: std::ptr::null_mut(),
            u_skeletal_mesh_editor_subsystem_set_morph_targets_to_generated_by_engine_for_all_skeletal_mesh: std::ptr::null_mut(),
            u_skeletal_mesh_editor_subsystem_set_morph_targets_to_generated_by_engine: std::ptr::null_mut(),
            u_skeletal_mesh_editor_subsystem_set_material_slot_overlay_material: std::ptr::null_mut(),
            u_skeletal_mesh_editor_subsystem_set_lod_build_settings: std::ptr::null_mut(),
            u_skeletal_mesh_editor_subsystem_rename_socket: std::ptr::null_mut(),
            u_skeletal_mesh_editor_subsystem_remove_lo_ds: std::ptr::null_mut(),
            u_skeletal_mesh_editor_subsystem_reimport_all_custom_lo_ds: std::ptr::null_mut(),
            u_skeletal_mesh_editor_subsystem_regenerate_lod: std::ptr::null_mut(),
            u_skeletal_mesh_editor_subsystem_is_physics_asset_compatible: std::ptr::null_mut(),
            u_skeletal_mesh_editor_subsystem_import_lod: std::ptr::null_mut(),
            u_skeletal_mesh_editor_subsystem_get_skeleton_curve_meta_data_names: std::ptr::null_mut(),
            u_skeletal_mesh_editor_subsystem_get_skeletal_mesh_overlay_material: std::ptr::null_mut(),
            u_skeletal_mesh_editor_subsystem_get_section_visible_in_ray_tracing: std::ptr::null_mut(),
            u_skeletal_mesh_editor_subsystem_get_section_recompute_tangents_vertex_mask_channel: std::ptr::null_mut(),
            u_skeletal_mesh_editor_subsystem_get_section_recompute_tangent: std::ptr::null_mut(),
            u_skeletal_mesh_editor_subsystem_get_section_cast_shadow: std::ptr::null_mut(),
            u_skeletal_mesh_editor_subsystem_get_num_verts: std::ptr::null_mut(),
            u_skeletal_mesh_editor_subsystem_get_num_sections: std::ptr::null_mut(),
            u_skeletal_mesh_editor_subsystem_get_morph_targets_generated_by_engine: std::ptr::null_mut(),
            u_skeletal_mesh_editor_subsystem_get_material_slot_overlay_material: std::ptr::null_mut(),
            u_skeletal_mesh_editor_subsystem_get_lod_material_slot: std::ptr::null_mut(),
            u_skeletal_mesh_editor_subsystem_get_lod_count: std::ptr::null_mut(),
            u_skeletal_mesh_editor_subsystem_get_lod_build_settings: std::ptr::null_mut(),
            u_skeletal_mesh_editor_subsystem_create_physics_asset: std::ptr::null_mut(),
            u_skeletal_mesh_editor_subsystem_assign_physics_asset: std::ptr::null_mut(),
        }
    }
}
pub fn initialize() {
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = USkeletalMeshEditorSubsystem::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("StripLODGeometry"),
            &raw mut __FUNCTION_PTRS.u_skeletal_mesh_editor_subsystem_strip_lod_geometry,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetSkeletalMeshOverlayMaterial"),
            &raw mut __FUNCTION_PTRS
                .u_skeletal_mesh_editor_subsystem_set_skeletal_mesh_overlay_material,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetSectionVisibleInRayTracing"),
            &raw mut __FUNCTION_PTRS
                .u_skeletal_mesh_editor_subsystem_set_section_visible_in_ray_tracing,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetSectionRecomputeTangentsVertexMaskChannel"),
            &raw mut __FUNCTION_PTRS
                .u_skeletal_mesh_editor_subsystem_set_section_recompute_tangents_vertex_mask_channel,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetSectionRecomputeTangent"),
            &raw mut __FUNCTION_PTRS
                .u_skeletal_mesh_editor_subsystem_set_section_recompute_tangent,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetSectionCastShadow"),
            &raw mut __FUNCTION_PTRS
                .u_skeletal_mesh_editor_subsystem_set_section_cast_shadow,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from(
                "SetMorphTargetsToGeneratedByEngineForAllSkeletalMesh",
            ),
            &raw mut __FUNCTION_PTRS
                .u_skeletal_mesh_editor_subsystem_set_morph_targets_to_generated_by_engine_for_all_skeletal_mesh,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetMorphTargetsToGeneratedByEngine"),
            &raw mut __FUNCTION_PTRS
                .u_skeletal_mesh_editor_subsystem_set_morph_targets_to_generated_by_engine,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetMaterialSlotOverlayMaterial"),
            &raw mut __FUNCTION_PTRS
                .u_skeletal_mesh_editor_subsystem_set_material_slot_overlay_material,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetLodBuildSettings"),
            &raw mut __FUNCTION_PTRS
                .u_skeletal_mesh_editor_subsystem_set_lod_build_settings,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RenameSocket"),
            &raw mut __FUNCTION_PTRS.u_skeletal_mesh_editor_subsystem_rename_socket,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RemoveLODs"),
            &raw mut __FUNCTION_PTRS.u_skeletal_mesh_editor_subsystem_remove_lo_ds,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ReimportAllCustomLODs"),
            &raw mut __FUNCTION_PTRS
                .u_skeletal_mesh_editor_subsystem_reimport_all_custom_lo_ds,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RegenerateLOD"),
            &raw mut __FUNCTION_PTRS.u_skeletal_mesh_editor_subsystem_regenerate_lod,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("IsPhysicsAssetCompatible"),
            &raw mut __FUNCTION_PTRS
                .u_skeletal_mesh_editor_subsystem_is_physics_asset_compatible,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ImportLOD"),
            &raw mut __FUNCTION_PTRS.u_skeletal_mesh_editor_subsystem_import_lod,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetSkeletonCurveMetaDataNames"),
            &raw mut __FUNCTION_PTRS
                .u_skeletal_mesh_editor_subsystem_get_skeleton_curve_meta_data_names,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetSkeletalMeshOverlayMaterial"),
            &raw mut __FUNCTION_PTRS
                .u_skeletal_mesh_editor_subsystem_get_skeletal_mesh_overlay_material,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetSectionVisibleInRayTracing"),
            &raw mut __FUNCTION_PTRS
                .u_skeletal_mesh_editor_subsystem_get_section_visible_in_ray_tracing,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetSectionRecomputeTangentsVertexMaskChannel"),
            &raw mut __FUNCTION_PTRS
                .u_skeletal_mesh_editor_subsystem_get_section_recompute_tangents_vertex_mask_channel,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetSectionRecomputeTangent"),
            &raw mut __FUNCTION_PTRS
                .u_skeletal_mesh_editor_subsystem_get_section_recompute_tangent,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetSectionCastShadow"),
            &raw mut __FUNCTION_PTRS
                .u_skeletal_mesh_editor_subsystem_get_section_cast_shadow,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetNumVerts"),
            &raw mut __FUNCTION_PTRS.u_skeletal_mesh_editor_subsystem_get_num_verts,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetNumSections"),
            &raw mut __FUNCTION_PTRS.u_skeletal_mesh_editor_subsystem_get_num_sections,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetMorphTargetsGeneratedByEngine"),
            &raw mut __FUNCTION_PTRS
                .u_skeletal_mesh_editor_subsystem_get_morph_targets_generated_by_engine,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetMaterialSlotOverlayMaterial"),
            &raw mut __FUNCTION_PTRS
                .u_skeletal_mesh_editor_subsystem_get_material_slot_overlay_material,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetLODMaterialSlot"),
            &raw mut __FUNCTION_PTRS
                .u_skeletal_mesh_editor_subsystem_get_lod_material_slot,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetLODCount"),
            &raw mut __FUNCTION_PTRS.u_skeletal_mesh_editor_subsystem_get_lod_count,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetLodBuildSettings"),
            &raw mut __FUNCTION_PTRS
                .u_skeletal_mesh_editor_subsystem_get_lod_build_settings,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CreatePhysicsAsset"),
            &raw mut __FUNCTION_PTRS
                .u_skeletal_mesh_editor_subsystem_create_physics_asset,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AssignPhysicsAsset"),
            &raw mut __FUNCTION_PTRS
                .u_skeletal_mesh_editor_subsystem_assign_physics_asset,
        );
    }
}
#[repr(C, align(8))]
pub struct USkeletalMeshEditorContextMenuContext {
    __padding_end: [u8; 56],
}
impl USkeletalMeshEditorContextMenuContext {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("USkeletalMeshEditorContextMenuContext")
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
pub struct USkeletalMeshEditorUISubsystem {
    __padding_end: [u8; 56],
}
impl USkeletalMeshEditorUISubsystem {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("USkeletalMeshEditorUISubsystem")
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
pub struct USkeletalMeshEditorSubsystem {
    __padding_end: [u8; 56],
}
impl USkeletalMeshEditorSubsystem {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("USkeletalMeshEditorSubsystem")
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
    pub fn strip_lod_geometry(
        skeletal_mesh: UPtr<crate::bindings::engine::USkeletalMesh>,
        lod_index: i32,
        texture_mask: UPtr<crate::bindings::engine::UTexture2D>,
        threshold: f32,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<29>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::skeletal_mesh_editor::__FUNCTION_PTRS
                    .u_skeletal_mesh_editor_subsystem_strip_lod_geometry,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &skeletal_mesh,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::USkeletalMesh>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&lod_index, __buffer.add(8).cast::<i32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &texture_mask,
                __buffer.add(16).cast::<UPtr<crate::bindings::engine::UTexture2D>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&threshold, __buffer.add(24).cast::<f32>(), 1);
        }
        let __object_ptr = crate::bindings::skeletal_mesh_editor::USkeletalMeshEditorSubsystem::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::skeletal_mesh_editor::__FUNCTION_PTRS
                    .u_skeletal_mesh_editor_subsystem_strip_lod_geometry,
                __buffer,
            )
        };
        unsafe { __buffer.add(28).cast::<bool>().read() }
    }
    pub fn set_skeletal_mesh_overlay_material(
        &mut self,
        skeletal_mesh: UPtr<crate::bindings::engine::USkeletalMesh>,
        new_overlay_material: UPtr<crate::bindings::engine::UMaterialInterface>,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<17>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::skeletal_mesh_editor::__FUNCTION_PTRS
                    .u_skeletal_mesh_editor_subsystem_set_skeletal_mesh_overlay_material,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &skeletal_mesh,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::USkeletalMesh>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &new_overlay_material,
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
                crate::bindings::skeletal_mesh_editor::__FUNCTION_PTRS
                    .u_skeletal_mesh_editor_subsystem_set_skeletal_mesh_overlay_material,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<bool>().read() }
    }
    pub fn set_section_visible_in_ray_tracing(
        &mut self,
        skeletal_mesh: UPtr<crate::bindings::engine::USkeletalMesh>,
        lod_index: i32,
        section_index: i32,
        b_visible_in_ray_tracing: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<18>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::skeletal_mesh_editor::__FUNCTION_PTRS
                    .u_skeletal_mesh_editor_subsystem_set_section_visible_in_ray_tracing,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &skeletal_mesh,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::USkeletalMesh>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&lod_index, __buffer.add(8).cast::<i32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &section_index,
                __buffer.add(12).cast::<i32>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_visible_in_ray_tracing,
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
                crate::bindings::skeletal_mesh_editor::__FUNCTION_PTRS
                    .u_skeletal_mesh_editor_subsystem_set_section_visible_in_ray_tracing,
                __buffer,
            )
        };
        unsafe { __buffer.add(17).cast::<bool>().read() }
    }
    pub fn set_section_recompute_tangents_vertex_mask_channel(
        &mut self,
        skeletal_mesh: UPtr<crate::bindings::engine::USkeletalMesh>,
        lod_index: i32,
        section_index: i32,
        recompute_tangents_vertex_mask_channel: u8,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<18>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::skeletal_mesh_editor::__FUNCTION_PTRS
                    .u_skeletal_mesh_editor_subsystem_set_section_recompute_tangents_vertex_mask_channel,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &skeletal_mesh,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::USkeletalMesh>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&lod_index, __buffer.add(8).cast::<i32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &section_index,
                __buffer.add(12).cast::<i32>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &recompute_tangents_vertex_mask_channel,
                __buffer.add(16).cast::<u8>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::skeletal_mesh_editor::__FUNCTION_PTRS
                    .u_skeletal_mesh_editor_subsystem_set_section_recompute_tangents_vertex_mask_channel,
                __buffer,
            )
        };
        unsafe { __buffer.add(17).cast::<bool>().read() }
    }
    pub fn set_section_recompute_tangent(
        &mut self,
        skeletal_mesh: UPtr<crate::bindings::engine::USkeletalMesh>,
        lod_index: i32,
        section_index: i32,
        b_recompute_tangent: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<18>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::skeletal_mesh_editor::__FUNCTION_PTRS
                    .u_skeletal_mesh_editor_subsystem_set_section_recompute_tangent,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &skeletal_mesh,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::USkeletalMesh>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&lod_index, __buffer.add(8).cast::<i32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &section_index,
                __buffer.add(12).cast::<i32>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_recompute_tangent,
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
                crate::bindings::skeletal_mesh_editor::__FUNCTION_PTRS
                    .u_skeletal_mesh_editor_subsystem_set_section_recompute_tangent,
                __buffer,
            )
        };
        unsafe { __buffer.add(17).cast::<bool>().read() }
    }
    pub fn set_section_cast_shadow(
        &mut self,
        skeletal_mesh: UPtr<crate::bindings::engine::USkeletalMesh>,
        lod_index: i32,
        section_index: i32,
        b_cast_shadow: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<18>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::skeletal_mesh_editor::__FUNCTION_PTRS
                    .u_skeletal_mesh_editor_subsystem_set_section_cast_shadow,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &skeletal_mesh,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::USkeletalMesh>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&lod_index, __buffer.add(8).cast::<i32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &section_index,
                __buffer.add(12).cast::<i32>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_cast_shadow,
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
                crate::bindings::skeletal_mesh_editor::__FUNCTION_PTRS
                    .u_skeletal_mesh_editor_subsystem_set_section_cast_shadow,
                __buffer,
            )
        };
        unsafe { __buffer.add(17).cast::<bool>().read() }
    }
    pub fn set_morph_targets_to_generated_by_engine_for_all_skeletal_mesh(
        optional_names: &TArray<FString>,
        optional_paths: &TArray<FName>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::skeletal_mesh_editor::__FUNCTION_PTRS
                    .u_skeletal_mesh_editor_subsystem_set_morph_targets_to_generated_by_engine_for_all_skeletal_mesh,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                optional_names,
                __buffer.add(0).cast::<TArray<FString>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                optional_paths,
                __buffer.add(16).cast::<TArray<FName>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::skeletal_mesh_editor::USkeletalMeshEditorSubsystem::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::skeletal_mesh_editor::__FUNCTION_PTRS
                    .u_skeletal_mesh_editor_subsystem_set_morph_targets_to_generated_by_engine_for_all_skeletal_mesh,
                __buffer,
            )
        };
    }
    pub fn set_morph_targets_to_generated_by_engine(
        target_mesh: UPtr<crate::bindings::engine::USkeletalMesh>,
        optional_names: &TArray<FString>,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<25>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::skeletal_mesh_editor::__FUNCTION_PTRS
                    .u_skeletal_mesh_editor_subsystem_set_morph_targets_to_generated_by_engine,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &target_mesh,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::USkeletalMesh>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                optional_names,
                __buffer.add(8).cast::<TArray<FString>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::skeletal_mesh_editor::USkeletalMeshEditorSubsystem::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::skeletal_mesh_editor::__FUNCTION_PTRS
                    .u_skeletal_mesh_editor_subsystem_set_morph_targets_to_generated_by_engine,
                __buffer,
            )
        };
        unsafe { __buffer.add(24).cast::<bool>().read() }
    }
    pub fn set_material_slot_overlay_material(
        &mut self,
        skeletal_mesh: UPtr<crate::bindings::engine::USkeletalMesh>,
        slot_index: i32,
        new_section_overlay_material: UPtr<crate::bindings::engine::UMaterialInterface>,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<25>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::skeletal_mesh_editor::__FUNCTION_PTRS
                    .u_skeletal_mesh_editor_subsystem_set_material_slot_overlay_material,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &skeletal_mesh,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::USkeletalMesh>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&slot_index, __buffer.add(8).cast::<i32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &new_section_overlay_material,
                __buffer
                    .add(16)
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
                crate::bindings::skeletal_mesh_editor::__FUNCTION_PTRS
                    .u_skeletal_mesh_editor_subsystem_set_material_slot_overlay_material,
                __buffer,
            )
        };
        unsafe { __buffer.add(24).cast::<bool>().read() }
    }
    pub fn set_lod_build_settings(
        skeletal_mesh: UPtr<crate::bindings::engine::USkeletalMesh>,
        lod_index: i32,
        build_options: &crate::bindings::engine::FSkeletalMeshBuildSettings,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<36>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::skeletal_mesh_editor::__FUNCTION_PTRS
                    .u_skeletal_mesh_editor_subsystem_set_lod_build_settings,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &skeletal_mesh,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::USkeletalMesh>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&lod_index, __buffer.add(8).cast::<i32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                build_options,
                __buffer
                    .add(12)
                    .cast::<crate::bindings::engine::FSkeletalMeshBuildSettings>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::skeletal_mesh_editor::USkeletalMeshEditorSubsystem::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::skeletal_mesh_editor::__FUNCTION_PTRS
                    .u_skeletal_mesh_editor_subsystem_set_lod_build_settings,
                __buffer,
            )
        };
    }
    pub fn rename_socket(
        skeletal_mesh: UPtr<crate::bindings::engine::USkeletalMesh>,
        old_name: FName,
        new_name: FName,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<33>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::skeletal_mesh_editor::__FUNCTION_PTRS
                    .u_skeletal_mesh_editor_subsystem_rename_socket,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &skeletal_mesh,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::USkeletalMesh>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&old_name, __buffer.add(8).cast::<FName>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &new_name,
                __buffer.add(20).cast::<FName>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::skeletal_mesh_editor::USkeletalMeshEditorSubsystem::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::skeletal_mesh_editor::__FUNCTION_PTRS
                    .u_skeletal_mesh_editor_subsystem_rename_socket,
                __buffer,
            )
        };
        unsafe { __buffer.add(32).cast::<bool>().read() }
    }
    pub fn remove_lods_remove_lo_ds(
        base_mesh: UPtr<crate::bindings::engine::USkeletalMesh>,
        to_remove_lo_ds: TArray<i32>,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<25>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::skeletal_mesh_editor::__FUNCTION_PTRS
                    .u_skeletal_mesh_editor_subsystem_remove_lo_ds,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &base_mesh,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::USkeletalMesh>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &to_remove_lo_ds,
                __buffer.add(8).cast::<TArray<i32>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::skeletal_mesh_editor::USkeletalMeshEditorSubsystem::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::skeletal_mesh_editor::__FUNCTION_PTRS
                    .u_skeletal_mesh_editor_subsystem_remove_lo_ds,
                __buffer,
            )
        };
        unsafe { __buffer.add(24).cast::<bool>().read() }
    }
    pub fn reimport_all_custom_lods_reimport_all_custom_lo_ds(
        skeletal_mesh: UPtr<crate::bindings::engine::USkeletalMesh>,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<9>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::skeletal_mesh_editor::__FUNCTION_PTRS
                    .u_skeletal_mesh_editor_subsystem_reimport_all_custom_lo_ds,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &skeletal_mesh,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::USkeletalMesh>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::skeletal_mesh_editor::USkeletalMeshEditorSubsystem::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::skeletal_mesh_editor::__FUNCTION_PTRS
                    .u_skeletal_mesh_editor_subsystem_reimport_all_custom_lo_ds,
                __buffer,
            )
        };
        unsafe { __buffer.add(8).cast::<bool>().read() }
    }
    pub fn regenerate_lod(
        skeletal_mesh: UPtr<crate::bindings::engine::USkeletalMesh>,
        new_lod_count: i32,
        b_regenerate_even_if_imported: bool,
        b_generate_base_lod: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<15>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::skeletal_mesh_editor::__FUNCTION_PTRS
                    .u_skeletal_mesh_editor_subsystem_regenerate_lod,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &skeletal_mesh,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::USkeletalMesh>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &new_lod_count,
                __buffer.add(8).cast::<i32>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_regenerate_even_if_imported,
                __buffer.add(12).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_generate_base_lod,
                __buffer.add(13).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::skeletal_mesh_editor::USkeletalMeshEditorSubsystem::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::skeletal_mesh_editor::__FUNCTION_PTRS
                    .u_skeletal_mesh_editor_subsystem_regenerate_lod,
                __buffer,
            )
        };
        unsafe { __buffer.add(14).cast::<bool>().read() }
    }
    pub fn is_physics_asset_compatible(
        target_mesh: UPtr<crate::bindings::engine::USkeletalMesh>,
        physics_asset: UPtr<crate::bindings::engine::UPhysicsAsset>,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<17>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::skeletal_mesh_editor::__FUNCTION_PTRS
                    .u_skeletal_mesh_editor_subsystem_is_physics_asset_compatible,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &target_mesh,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::USkeletalMesh>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &physics_asset,
                __buffer.add(8).cast::<UPtr<crate::bindings::engine::UPhysicsAsset>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::skeletal_mesh_editor::USkeletalMeshEditorSubsystem::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::skeletal_mesh_editor::__FUNCTION_PTRS
                    .u_skeletal_mesh_editor_subsystem_is_physics_asset_compatible,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<bool>().read() }
    }
    pub fn import_lod(
        base_mesh: UPtr<crate::bindings::engine::USkeletalMesh>,
        lod_index: i32,
        source_filename: FString,
    ) -> i32 {
        let mut __stack = crate::core_data::StackAlloc::<36>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::skeletal_mesh_editor::__FUNCTION_PTRS
                    .u_skeletal_mesh_editor_subsystem_import_lod,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &base_mesh,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::USkeletalMesh>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&lod_index, __buffer.add(8).cast::<i32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &source_filename,
                __buffer.add(16).cast::<FString>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::skeletal_mesh_editor::USkeletalMeshEditorSubsystem::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::skeletal_mesh_editor::__FUNCTION_PTRS
                    .u_skeletal_mesh_editor_subsystem_import_lod,
                __buffer,
            )
        };
        unsafe { __buffer.add(32).cast::<i32>().read() }
    }
    pub fn get_skeleton_curve_meta_data_names(
        target_skeleton: UPtr<crate::bindings::engine::USkeleton>,
        out_names: &mut TArray<FName>,
        filter: ESkelSubSysQueryCurvesMetatdataNamesFilter,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<25>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::skeletal_mesh_editor::__FUNCTION_PTRS
                    .u_skeletal_mesh_editor_subsystem_get_skeleton_curve_meta_data_names,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &target_skeleton,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::USkeleton>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_names,
                __buffer.add(8).cast::<TArray<FName>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &filter,
                __buffer.add(24).cast::<ESkelSubSysQueryCurvesMetatdataNamesFilter>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::skeletal_mesh_editor::USkeletalMeshEditorSubsystem::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::skeletal_mesh_editor::__FUNCTION_PTRS
                    .u_skeletal_mesh_editor_subsystem_get_skeleton_curve_meta_data_names,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(8).cast::<TArray<FName>>().swap(out_names);
        }
    }
    pub fn get_skeletal_mesh_overlay_material(
        &mut self,
        skeletal_mesh: UPtr<crate::bindings::engine::USkeletalMesh>,
    ) -> UPtr<crate::bindings::engine::UMaterialInterface> {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::skeletal_mesh_editor::__FUNCTION_PTRS
                    .u_skeletal_mesh_editor_subsystem_get_skeletal_mesh_overlay_material,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &skeletal_mesh,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::USkeletalMesh>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::skeletal_mesh_editor::__FUNCTION_PTRS
                    .u_skeletal_mesh_editor_subsystem_get_skeletal_mesh_overlay_material,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(8)
                .cast::<UPtr<crate::bindings::engine::UMaterialInterface>>()
                .read()
        }
    }
    pub fn get_section_visible_in_ray_tracing(
        &mut self,
        skeletal_mesh: UPtr<crate::bindings::engine::USkeletalMesh>,
        lod_index: i32,
        section_index: i32,
        b_out_visible_in_ray_tracing: &mut bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<18>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::skeletal_mesh_editor::__FUNCTION_PTRS
                    .u_skeletal_mesh_editor_subsystem_get_section_visible_in_ray_tracing,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &skeletal_mesh,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::USkeletalMesh>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&lod_index, __buffer.add(8).cast::<i32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &section_index,
                __buffer.add(12).cast::<i32>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                b_out_visible_in_ray_tracing,
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
                crate::bindings::skeletal_mesh_editor::__FUNCTION_PTRS
                    .u_skeletal_mesh_editor_subsystem_get_section_visible_in_ray_tracing,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(16).cast::<bool>().swap(b_out_visible_in_ray_tracing);
        }
        unsafe { __buffer.add(17).cast::<bool>().read() }
    }
    pub fn get_section_recompute_tangents_vertex_mask_channel(
        &mut self,
        skeletal_mesh: UPtr<crate::bindings::engine::USkeletalMesh>,
        lod_index: i32,
        section_index: i32,
        out_recompute_tangents_vertex_mask_channel: &mut u8,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<18>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::skeletal_mesh_editor::__FUNCTION_PTRS
                    .u_skeletal_mesh_editor_subsystem_get_section_recompute_tangents_vertex_mask_channel,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &skeletal_mesh,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::USkeletalMesh>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&lod_index, __buffer.add(8).cast::<i32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &section_index,
                __buffer.add(12).cast::<i32>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_recompute_tangents_vertex_mask_channel,
                __buffer.add(16).cast::<u8>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::skeletal_mesh_editor::__FUNCTION_PTRS
                    .u_skeletal_mesh_editor_subsystem_get_section_recompute_tangents_vertex_mask_channel,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(16)
                .cast::<u8>()
                .swap(out_recompute_tangents_vertex_mask_channel);
        }
        unsafe { __buffer.add(17).cast::<bool>().read() }
    }
    pub fn get_section_recompute_tangent(
        &mut self,
        skeletal_mesh: UPtr<crate::bindings::engine::USkeletalMesh>,
        lod_index: i32,
        section_index: i32,
        b_out_recompute_tangent: &mut bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<18>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::skeletal_mesh_editor::__FUNCTION_PTRS
                    .u_skeletal_mesh_editor_subsystem_get_section_recompute_tangent,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &skeletal_mesh,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::USkeletalMesh>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&lod_index, __buffer.add(8).cast::<i32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &section_index,
                __buffer.add(12).cast::<i32>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                b_out_recompute_tangent,
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
                crate::bindings::skeletal_mesh_editor::__FUNCTION_PTRS
                    .u_skeletal_mesh_editor_subsystem_get_section_recompute_tangent,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(16).cast::<bool>().swap(b_out_recompute_tangent);
        }
        unsafe { __buffer.add(17).cast::<bool>().read() }
    }
    pub fn get_section_cast_shadow(
        &mut self,
        skeletal_mesh: UPtr<crate::bindings::engine::USkeletalMesh>,
        lod_index: i32,
        section_index: i32,
        b_out_cast_shadow: &mut bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<18>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::skeletal_mesh_editor::__FUNCTION_PTRS
                    .u_skeletal_mesh_editor_subsystem_get_section_cast_shadow,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &skeletal_mesh,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::USkeletalMesh>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&lod_index, __buffer.add(8).cast::<i32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &section_index,
                __buffer.add(12).cast::<i32>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                b_out_cast_shadow,
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
                crate::bindings::skeletal_mesh_editor::__FUNCTION_PTRS
                    .u_skeletal_mesh_editor_subsystem_get_section_cast_shadow,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(16).cast::<bool>().swap(b_out_cast_shadow);
        }
        unsafe { __buffer.add(17).cast::<bool>().read() }
    }
    pub fn get_num_verts(
        &mut self,
        skeletal_mesh: UPtr<crate::bindings::engine::USkeletalMesh>,
        lod_index: i32,
    ) -> i32 {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::skeletal_mesh_editor::__FUNCTION_PTRS
                    .u_skeletal_mesh_editor_subsystem_get_num_verts,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &skeletal_mesh,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::USkeletalMesh>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&lod_index, __buffer.add(8).cast::<i32>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::skeletal_mesh_editor::__FUNCTION_PTRS
                    .u_skeletal_mesh_editor_subsystem_get_num_verts,
                __buffer,
            )
        };
        unsafe { __buffer.add(12).cast::<i32>().read() }
    }
    pub fn get_num_sections(
        &mut self,
        skeletal_mesh: UPtr<crate::bindings::engine::USkeletalMesh>,
        lod_index: i32,
    ) -> i32 {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::skeletal_mesh_editor::__FUNCTION_PTRS
                    .u_skeletal_mesh_editor_subsystem_get_num_sections,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &skeletal_mesh,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::USkeletalMesh>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&lod_index, __buffer.add(8).cast::<i32>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::skeletal_mesh_editor::__FUNCTION_PTRS
                    .u_skeletal_mesh_editor_subsystem_get_num_sections,
                __buffer,
            )
        };
        unsafe { __buffer.add(12).cast::<i32>().read() }
    }
    pub fn get_morph_targets_generated_by_engine(
        target_mesh: UPtr<crate::bindings::engine::USkeletalMesh>,
        out_names: &mut TArray<FName>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::skeletal_mesh_editor::__FUNCTION_PTRS
                    .u_skeletal_mesh_editor_subsystem_get_morph_targets_generated_by_engine,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &target_mesh,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::USkeletalMesh>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_names,
                __buffer.add(8).cast::<TArray<FName>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::skeletal_mesh_editor::USkeletalMeshEditorSubsystem::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::skeletal_mesh_editor::__FUNCTION_PTRS
                    .u_skeletal_mesh_editor_subsystem_get_morph_targets_generated_by_engine,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(8).cast::<TArray<FName>>().swap(out_names);
        }
    }
    pub fn get_material_slot_overlay_material(
        &mut self,
        skeletal_mesh: UPtr<crate::bindings::engine::USkeletalMesh>,
        slot_index: i32,
    ) -> UPtr<crate::bindings::engine::UMaterialInterface> {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::skeletal_mesh_editor::__FUNCTION_PTRS
                    .u_skeletal_mesh_editor_subsystem_get_material_slot_overlay_material,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &skeletal_mesh,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::USkeletalMesh>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&slot_index, __buffer.add(8).cast::<i32>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::skeletal_mesh_editor::__FUNCTION_PTRS
                    .u_skeletal_mesh_editor_subsystem_get_material_slot_overlay_material,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(16)
                .cast::<UPtr<crate::bindings::engine::UMaterialInterface>>()
                .read()
        }
    }
    pub fn get_lod_material_slot(
        &mut self,
        skeletal_mesh: UPtr<crate::bindings::engine::USkeletalMesh>,
        lod_index: i32,
        section_index: i32,
    ) -> i32 {
        let mut __stack = crate::core_data::StackAlloc::<20>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::skeletal_mesh_editor::__FUNCTION_PTRS
                    .u_skeletal_mesh_editor_subsystem_get_lod_material_slot,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &skeletal_mesh,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::USkeletalMesh>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&lod_index, __buffer.add(8).cast::<i32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &section_index,
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
                crate::bindings::skeletal_mesh_editor::__FUNCTION_PTRS
                    .u_skeletal_mesh_editor_subsystem_get_lod_material_slot,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<i32>().read() }
    }
    pub fn get_lod_count(
        skeletal_mesh: UPtr<crate::bindings::engine::USkeletalMesh>,
    ) -> i32 {
        let mut __stack = crate::core_data::StackAlloc::<12>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::skeletal_mesh_editor::__FUNCTION_PTRS
                    .u_skeletal_mesh_editor_subsystem_get_lod_count,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &skeletal_mesh,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::USkeletalMesh>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::skeletal_mesh_editor::USkeletalMeshEditorSubsystem::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::skeletal_mesh_editor::__FUNCTION_PTRS
                    .u_skeletal_mesh_editor_subsystem_get_lod_count,
                __buffer,
            )
        };
        unsafe { __buffer.add(8).cast::<i32>().read() }
    }
    pub fn get_lod_build_settings(
        skeletal_mesh: UPtr<crate::bindings::engine::USkeletalMesh>,
        lod_index: i32,
        out_build_options: &mut crate::bindings::engine::FSkeletalMeshBuildSettings,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<36>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::skeletal_mesh_editor::__FUNCTION_PTRS
                    .u_skeletal_mesh_editor_subsystem_get_lod_build_settings,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &skeletal_mesh,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::USkeletalMesh>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&lod_index, __buffer.add(8).cast::<i32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_build_options,
                __buffer
                    .add(12)
                    .cast::<crate::bindings::engine::FSkeletalMeshBuildSettings>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::skeletal_mesh_editor::USkeletalMeshEditorSubsystem::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::skeletal_mesh_editor::__FUNCTION_PTRS
                    .u_skeletal_mesh_editor_subsystem_get_lod_build_settings,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(12)
                .cast::<crate::bindings::engine::FSkeletalMeshBuildSettings>()
                .swap(out_build_options);
        }
    }
    pub fn create_physics_asset(
        skeletal_mesh: UPtr<crate::bindings::engine::USkeletalMesh>,
        b_set_to_mesh: bool,
        lod_index: i32,
    ) -> UPtr<crate::bindings::engine::UPhysicsAsset> {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::skeletal_mesh_editor::__FUNCTION_PTRS
                    .u_skeletal_mesh_editor_subsystem_create_physics_asset,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &skeletal_mesh,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::USkeletalMesh>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_set_to_mesh,
                __buffer.add(8).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&lod_index, __buffer.add(12).cast::<i32>(), 1);
        }
        let __object_ptr = crate::bindings::skeletal_mesh_editor::USkeletalMeshEditorSubsystem::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::skeletal_mesh_editor::__FUNCTION_PTRS
                    .u_skeletal_mesh_editor_subsystem_create_physics_asset,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(16)
                .cast::<UPtr<crate::bindings::engine::UPhysicsAsset>>()
                .read()
        }
    }
    pub fn assign_physics_asset(
        target_mesh: UPtr<crate::bindings::engine::USkeletalMesh>,
        physics_asset: UPtr<crate::bindings::engine::UPhysicsAsset>,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<17>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::skeletal_mesh_editor::__FUNCTION_PTRS
                    .u_skeletal_mesh_editor_subsystem_assign_physics_asset,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &target_mesh,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::USkeletalMesh>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &physics_asset,
                __buffer.add(8).cast::<UPtr<crate::bindings::engine::UPhysicsAsset>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::skeletal_mesh_editor::USkeletalMeshEditorSubsystem::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::skeletal_mesh_editor::__FUNCTION_PTRS
                    .u_skeletal_mesh_editor_subsystem_assign_physics_asset,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<bool>().read() }
    }
}
#[repr(transparent)]
pub struct ESkelSubSysQueryCurvesMetatdataNamesFilter(pub u8);
impl ESkelSubSysQueryCurvesMetatdataNamesFilter {
    pub const ALL: ESkelSubSysQueryCurvesMetatdataNamesFilter = ESkelSubSysQueryCurvesMetatdataNamesFilter(
        0,
    );
    pub const MORPH_TARGET_ONLY: ESkelSubSysQueryCurvesMetatdataNamesFilter = ESkelSubSysQueryCurvesMetatdataNamesFilter(
        1,
    );
    pub const MATERIAL_ONLY: ESkelSubSysQueryCurvesMetatdataNamesFilter = ESkelSubSysQueryCurvesMetatdataNamesFilter(
        2,
    );
}
