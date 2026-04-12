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
    pub u_blueprint_material_texture_nodes_bp_library_update_mic: *mut crate::ffi::UFunctionOpague,
    pub u_blueprint_material_texture_nodes_bp_library_texture2_d_sample_uv_editor_only: *mut crate::ffi::UFunctionOpague,
    pub u_blueprint_material_texture_nodes_bp_library_set_mic_vector_param_editor_only: *mut crate::ffi::UFunctionOpague,
    pub u_blueprint_material_texture_nodes_bp_library_set_mic_two_sided_editor_only: *mut crate::ffi::UFunctionOpague,
    pub u_blueprint_material_texture_nodes_bp_library_set_mic_texture_param_editor_only: *mut crate::ffi::UFunctionOpague,
    pub u_blueprint_material_texture_nodes_bp_library_set_mic_shading_model_editor_only: *mut crate::ffi::UFunctionOpague,
    pub u_blueprint_material_texture_nodes_bp_library_set_mic_scalar_param_editor_only: *mut crate::ffi::UFunctionOpague,
    pub u_blueprint_material_texture_nodes_bp_library_set_mic_is_thin_surface_editor_only: *mut crate::ffi::UFunctionOpague,
    pub u_blueprint_material_texture_nodes_bp_library_set_mic_dithered_lod_transition_editor_only: *mut crate::ffi::UFunctionOpague,
    pub u_blueprint_material_texture_nodes_bp_library_set_mic_blend_mode_editor_only: *mut crate::ffi::UFunctionOpague,
    pub u_blueprint_material_texture_nodes_bp_library_render_target_sample_uv_editor_only: *mut crate::ffi::UFunctionOpague,
    pub u_blueprint_material_texture_nodes_bp_library_render_target_sample_rectangle_editor_only: *mut crate::ffi::UFunctionOpague,
    pub u_blueprint_material_texture_nodes_bp_library_create_mic_editor_only: *mut crate::ffi::UFunctionOpague,
}
impl FunctionPtrs {
    pub const fn empty() -> Self {
        Self {
            u_blueprint_material_texture_nodes_bp_library_update_mic: std::ptr::null_mut(),
            u_blueprint_material_texture_nodes_bp_library_texture2_d_sample_uv_editor_only: std::ptr::null_mut(),
            u_blueprint_material_texture_nodes_bp_library_set_mic_vector_param_editor_only: std::ptr::null_mut(),
            u_blueprint_material_texture_nodes_bp_library_set_mic_two_sided_editor_only: std::ptr::null_mut(),
            u_blueprint_material_texture_nodes_bp_library_set_mic_texture_param_editor_only: std::ptr::null_mut(),
            u_blueprint_material_texture_nodes_bp_library_set_mic_shading_model_editor_only: std::ptr::null_mut(),
            u_blueprint_material_texture_nodes_bp_library_set_mic_scalar_param_editor_only: std::ptr::null_mut(),
            u_blueprint_material_texture_nodes_bp_library_set_mic_is_thin_surface_editor_only: std::ptr::null_mut(),
            u_blueprint_material_texture_nodes_bp_library_set_mic_dithered_lod_transition_editor_only: std::ptr::null_mut(),
            u_blueprint_material_texture_nodes_bp_library_set_mic_blend_mode_editor_only: std::ptr::null_mut(),
            u_blueprint_material_texture_nodes_bp_library_render_target_sample_uv_editor_only: std::ptr::null_mut(),
            u_blueprint_material_texture_nodes_bp_library_render_target_sample_rectangle_editor_only: std::ptr::null_mut(),
            u_blueprint_material_texture_nodes_bp_library_create_mic_editor_only: std::ptr::null_mut(),
        }
    }
}
pub fn initialize() {
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UBlueprintMaterialTextureNodesBPLibrary::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("UpdateMIC"),
                &raw mut __FUNCTION_PTRS
                    .u_blueprint_material_texture_nodes_bp_library_update_mic,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("Texture2D_SampleUV_EditorOnly"),
                &raw mut __FUNCTION_PTRS
                    .u_blueprint_material_texture_nodes_bp_library_texture2_d_sample_uv_editor_only,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetMICVectorParam_EditorOnly"),
                &raw mut __FUNCTION_PTRS
                    .u_blueprint_material_texture_nodes_bp_library_set_mic_vector_param_editor_only,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetMICTwoSided_EditorOnly"),
                &raw mut __FUNCTION_PTRS
                    .u_blueprint_material_texture_nodes_bp_library_set_mic_two_sided_editor_only,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetMICTextureParam_EditorOnly"),
                &raw mut __FUNCTION_PTRS
                    .u_blueprint_material_texture_nodes_bp_library_set_mic_texture_param_editor_only,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetMICShadingModel_EditorOnly"),
                &raw mut __FUNCTION_PTRS
                    .u_blueprint_material_texture_nodes_bp_library_set_mic_shading_model_editor_only,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetMICScalarParam_EditorOnly"),
                &raw mut __FUNCTION_PTRS
                    .u_blueprint_material_texture_nodes_bp_library_set_mic_scalar_param_editor_only,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetMICIsThinSurface_EditorOnly"),
                &raw mut __FUNCTION_PTRS
                    .u_blueprint_material_texture_nodes_bp_library_set_mic_is_thin_surface_editor_only,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetMICDitheredLODTransition_EditorOnly"),
                &raw mut __FUNCTION_PTRS
                    .u_blueprint_material_texture_nodes_bp_library_set_mic_dithered_lod_transition_editor_only,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetMICBlendMode_EditorOnly"),
                &raw mut __FUNCTION_PTRS
                    .u_blueprint_material_texture_nodes_bp_library_set_mic_blend_mode_editor_only,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("RenderTarget_SampleUV_EditorOnly"),
                &raw mut __FUNCTION_PTRS
                    .u_blueprint_material_texture_nodes_bp_library_render_target_sample_uv_editor_only,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("RenderTarget_SampleRectangle_EditorOnly"),
                &raw mut __FUNCTION_PTRS
                    .u_blueprint_material_texture_nodes_bp_library_render_target_sample_rectangle_editor_only,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("CreateMIC_EditorOnly"),
                &raw mut __FUNCTION_PTRS
                    .u_blueprint_material_texture_nodes_bp_library_create_mic_editor_only,
            );
        }
    }
}
#[repr(C, align(8))]
pub struct UBlueprintMaterialTextureNodesBPLibrary {
    __padding_end: [u8; 48],
}
impl UBlueprintMaterialTextureNodesBPLibrary {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBlueprintMaterialTextureNodesBPLibrary")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBlueprintMaterialTextureNodesBPLibrary")
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
    pub fn texture2_d_sample_uv_editor_only(
        texture: UPtr<crate::bindings::engine::UTexture2D>,
        uv: crate::bindings::core_u_object::FVector2D,
    ) -> crate::bindings::core_u_object::FLinearColor {
        let mut __stack = crate::core_data::StackAlloc::<40>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::blueprint_material_texture_nodes::__FUNCTION_PTRS
                    .u_blueprint_material_texture_nodes_bp_library_texture2_d_sample_uv_editor_only,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &texture,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::UTexture2D>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &uv,
                __buffer.add(8).cast::<crate::bindings::core_u_object::FVector2D>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::blueprint_material_texture_nodes::UBlueprintMaterialTextureNodesBPLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::blueprint_material_texture_nodes::__FUNCTION_PTRS
                    .u_blueprint_material_texture_nodes_bp_library_texture2_d_sample_uv_editor_only,
                __buffer,
            )
        };
        std::mem::forget(texture);
        std::mem::forget(uv);
        unsafe {
            __buffer
                .add(24)
                .cast::<crate::bindings::core_u_object::FLinearColor>()
                .read()
        }
    }
    pub fn set_mic_vector_param_editor_only(
        material: UPtr<crate::bindings::engine::UMaterialInstanceConstant>,
        param_name: FString,
        value: crate::bindings::core_u_object::FLinearColor,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<41>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::blueprint_material_texture_nodes::__FUNCTION_PTRS
                    .u_blueprint_material_texture_nodes_bp_library_set_mic_vector_param_editor_only,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &material,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::engine::UMaterialInstanceConstant>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &param_name,
                __buffer.add(8).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &value,
                __buffer.add(24).cast::<crate::bindings::core_u_object::FLinearColor>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::blueprint_material_texture_nodes::UBlueprintMaterialTextureNodesBPLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::blueprint_material_texture_nodes::__FUNCTION_PTRS
                    .u_blueprint_material_texture_nodes_bp_library_set_mic_vector_param_editor_only,
                __buffer,
            )
        };
        std::mem::forget(material);
        std::mem::forget(param_name);
        std::mem::forget(value);
        unsafe { __buffer.add(40).cast::<bool>().read() }
    }
    pub fn set_mic_two_sided_editor_only(
        material: UPtr<crate::bindings::engine::UMaterialInstanceConstant>,
        two_sided: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<10>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::blueprint_material_texture_nodes::__FUNCTION_PTRS
                    .u_blueprint_material_texture_nodes_bp_library_set_mic_two_sided_editor_only,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &material,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::engine::UMaterialInstanceConstant>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&two_sided, __buffer.add(8).cast::<bool>(), 1);
        }
        let __object_ptr = crate::bindings::blueprint_material_texture_nodes::UBlueprintMaterialTextureNodesBPLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::blueprint_material_texture_nodes::__FUNCTION_PTRS
                    .u_blueprint_material_texture_nodes_bp_library_set_mic_two_sided_editor_only,
                __buffer,
            )
        };
        std::mem::forget(material);
        std::mem::forget(two_sided);
        unsafe { __buffer.add(9).cast::<bool>().read() }
    }
    pub fn set_mic_texture_param_editor_only(
        material: UPtr<crate::bindings::engine::UMaterialInstanceConstant>,
        param_name: FString,
        texture: UPtr<crate::bindings::engine::UTexture2D>,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<33>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::blueprint_material_texture_nodes::__FUNCTION_PTRS
                    .u_blueprint_material_texture_nodes_bp_library_set_mic_texture_param_editor_only,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &material,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::engine::UMaterialInstanceConstant>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &param_name,
                __buffer.add(8).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &texture,
                __buffer.add(24).cast::<UPtr<crate::bindings::engine::UTexture2D>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::blueprint_material_texture_nodes::UBlueprintMaterialTextureNodesBPLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::blueprint_material_texture_nodes::__FUNCTION_PTRS
                    .u_blueprint_material_texture_nodes_bp_library_set_mic_texture_param_editor_only,
                __buffer,
            )
        };
        std::mem::forget(material);
        std::mem::forget(param_name);
        std::mem::forget(texture);
        unsafe { __buffer.add(32).cast::<bool>().read() }
    }
    pub fn set_mic_shading_model_editor_only(
        material: UPtr<crate::bindings::engine::UMaterialInstanceConstant>,
        shading_model: crate::bindings::engine::EMaterialShadingModel,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<10>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::blueprint_material_texture_nodes::__FUNCTION_PTRS
                    .u_blueprint_material_texture_nodes_bp_library_set_mic_shading_model_editor_only,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &material,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::engine::UMaterialInstanceConstant>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &shading_model,
                __buffer.add(8).cast::<crate::bindings::engine::EMaterialShadingModel>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::blueprint_material_texture_nodes::UBlueprintMaterialTextureNodesBPLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::blueprint_material_texture_nodes::__FUNCTION_PTRS
                    .u_blueprint_material_texture_nodes_bp_library_set_mic_shading_model_editor_only,
                __buffer,
            )
        };
        std::mem::forget(material);
        std::mem::forget(shading_model);
        unsafe { __buffer.add(9).cast::<bool>().read() }
    }
    pub fn set_mic_scalar_param_editor_only(
        material: UPtr<crate::bindings::engine::UMaterialInstanceConstant>,
        param_name: FString,
        value: f32,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<29>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::blueprint_material_texture_nodes::__FUNCTION_PTRS
                    .u_blueprint_material_texture_nodes_bp_library_set_mic_scalar_param_editor_only,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &material,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::engine::UMaterialInstanceConstant>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &param_name,
                __buffer.add(8).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&value, __buffer.add(24).cast::<f32>(), 1);
        }
        let __object_ptr = crate::bindings::blueprint_material_texture_nodes::UBlueprintMaterialTextureNodesBPLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::blueprint_material_texture_nodes::__FUNCTION_PTRS
                    .u_blueprint_material_texture_nodes_bp_library_set_mic_scalar_param_editor_only,
                __buffer,
            )
        };
        std::mem::forget(material);
        std::mem::forget(param_name);
        std::mem::forget(value);
        unsafe { __buffer.add(28).cast::<bool>().read() }
    }
    pub fn set_mic_is_thin_surface_editor_only(
        material: UPtr<crate::bindings::engine::UMaterialInstanceConstant>,
        b_is_thin_surface: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<10>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::blueprint_material_texture_nodes::__FUNCTION_PTRS
                    .u_blueprint_material_texture_nodes_bp_library_set_mic_is_thin_surface_editor_only,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &material,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::engine::UMaterialInstanceConstant>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_is_thin_surface,
                __buffer.add(8).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::blueprint_material_texture_nodes::UBlueprintMaterialTextureNodesBPLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::blueprint_material_texture_nodes::__FUNCTION_PTRS
                    .u_blueprint_material_texture_nodes_bp_library_set_mic_is_thin_surface_editor_only,
                __buffer,
            )
        };
        std::mem::forget(material);
        std::mem::forget(b_is_thin_surface);
        unsafe { __buffer.add(9).cast::<bool>().read() }
    }
    pub fn set_mic_dithered_lod_transition_editor_only(
        material: UPtr<crate::bindings::engine::UMaterialInstanceConstant>,
        dithered_lod_transition: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<10>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::blueprint_material_texture_nodes::__FUNCTION_PTRS
                    .u_blueprint_material_texture_nodes_bp_library_set_mic_dithered_lod_transition_editor_only,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &material,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::engine::UMaterialInstanceConstant>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &dithered_lod_transition,
                __buffer.add(8).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::blueprint_material_texture_nodes::UBlueprintMaterialTextureNodesBPLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::blueprint_material_texture_nodes::__FUNCTION_PTRS
                    .u_blueprint_material_texture_nodes_bp_library_set_mic_dithered_lod_transition_editor_only,
                __buffer,
            )
        };
        std::mem::forget(material);
        std::mem::forget(dithered_lod_transition);
        unsafe { __buffer.add(9).cast::<bool>().read() }
    }
    pub fn set_mic_blend_mode_editor_only(
        material: UPtr<crate::bindings::engine::UMaterialInstanceConstant>,
        blend_mode: crate::bindings::engine::EBlendMode,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<10>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::blueprint_material_texture_nodes::__FUNCTION_PTRS
                    .u_blueprint_material_texture_nodes_bp_library_set_mic_blend_mode_editor_only,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &material,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::engine::UMaterialInstanceConstant>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &blend_mode,
                __buffer.add(8).cast::<crate::bindings::engine::EBlendMode>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::blueprint_material_texture_nodes::UBlueprintMaterialTextureNodesBPLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::blueprint_material_texture_nodes::__FUNCTION_PTRS
                    .u_blueprint_material_texture_nodes_bp_library_set_mic_blend_mode_editor_only,
                __buffer,
            )
        };
        std::mem::forget(material);
        std::mem::forget(blend_mode);
        unsafe { __buffer.add(9).cast::<bool>().read() }
    }
    pub fn render_target_sample_uv_editor_only(
        in_render_target: UPtr<crate::bindings::engine::UTextureRenderTarget2D>,
        uv: crate::bindings::core_u_object::FVector2D,
    ) -> crate::bindings::core_u_object::FLinearColor {
        let mut __stack = crate::core_data::StackAlloc::<40>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::blueprint_material_texture_nodes::__FUNCTION_PTRS
                    .u_blueprint_material_texture_nodes_bp_library_render_target_sample_uv_editor_only,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_render_target,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::engine::UTextureRenderTarget2D>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &uv,
                __buffer.add(8).cast::<crate::bindings::core_u_object::FVector2D>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::blueprint_material_texture_nodes::UBlueprintMaterialTextureNodesBPLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::blueprint_material_texture_nodes::__FUNCTION_PTRS
                    .u_blueprint_material_texture_nodes_bp_library_render_target_sample_uv_editor_only,
                __buffer,
            )
        };
        std::mem::forget(in_render_target);
        std::mem::forget(uv);
        unsafe {
            __buffer
                .add(24)
                .cast::<crate::bindings::core_u_object::FLinearColor>()
                .read()
        }
    }
    pub fn render_target_sample_rectangle_editor_only(
        in_render_target: UPtr<crate::bindings::engine::UTextureRenderTarget2D>,
        in_rect: crate::bindings::core_u_object::FLinearColor,
    ) -> TArray<crate::bindings::core_u_object::FLinearColor> {
        let mut __stack = crate::core_data::StackAlloc::<40>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::blueprint_material_texture_nodes::__FUNCTION_PTRS
                    .u_blueprint_material_texture_nodes_bp_library_render_target_sample_rectangle_editor_only,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_render_target,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::engine::UTextureRenderTarget2D>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_rect,
                __buffer.add(8).cast::<crate::bindings::core_u_object::FLinearColor>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::blueprint_material_texture_nodes::UBlueprintMaterialTextureNodesBPLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::blueprint_material_texture_nodes::__FUNCTION_PTRS
                    .u_blueprint_material_texture_nodes_bp_library_render_target_sample_rectangle_editor_only,
                __buffer,
            )
        };
        std::mem::forget(in_render_target);
        std::mem::forget(in_rect);
        unsafe {
            __buffer
                .add(24)
                .cast::<TArray<crate::bindings::core_u_object::FLinearColor>>()
                .read()
        }
    }
    pub fn create_mic_editor_only(
        material: UPtr<crate::bindings::engine::UMaterialInterface>,
        name: FString,
    ) -> UPtr<crate::bindings::engine::UMaterialInstanceConstant> {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::blueprint_material_texture_nodes::__FUNCTION_PTRS
                    .u_blueprint_material_texture_nodes_bp_library_create_mic_editor_only,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &material,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::engine::UMaterialInterface>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&name, __buffer.add(8).cast::<FString>(), 1);
        }
        let __object_ptr = crate::bindings::blueprint_material_texture_nodes::UBlueprintMaterialTextureNodesBPLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::blueprint_material_texture_nodes::__FUNCTION_PTRS
                    .u_blueprint_material_texture_nodes_bp_library_create_mic_editor_only,
                __buffer,
            )
        };
        std::mem::forget(material);
        std::mem::forget(name);
        unsafe {
            __buffer
                .add(24)
                .cast::<UPtr<crate::bindings::engine::UMaterialInstanceConstant>>()
                .read()
        }
    }
}
