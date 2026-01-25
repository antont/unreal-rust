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
    pub u_material_editing_library_update_material_instance: *mut crate::ffi::UFunctionOpague,
    pub u_material_editing_library_update_material_function: *mut crate::ffi::UFunctionOpague,
    pub u_material_editing_library_set_material_usage: *mut crate::ffi::UFunctionOpague,
    pub u_material_editing_library_set_material_instance_vector_parameter_value: *mut crate::ffi::UFunctionOpague,
    pub u_material_editing_library_set_material_instance_texture_parameter_value: *mut crate::ffi::UFunctionOpague,
    pub u_material_editing_library_set_material_instance_static_switch_parameter_value: *mut crate::ffi::UFunctionOpague,
    pub u_material_editing_library_set_material_instance_sparse_volume_texture_parameter_value: *mut crate::ffi::UFunctionOpague,
    pub u_material_editing_library_set_material_instance_scalar_parameter_value: *mut crate::ffi::UFunctionOpague,
    pub u_material_editing_library_set_material_instance_runtime_virtual_texture_parameter_value: *mut crate::ffi::UFunctionOpague,
    pub u_material_editing_library_set_material_instance_parent: *mut crate::ffi::UFunctionOpague,
    pub u_material_editing_library_rename_material_parameter_group: *mut crate::ffi::UFunctionOpague,
    pub u_material_editing_library_rename_material_function_parameter_group: *mut crate::ffi::UFunctionOpague,
    pub u_material_editing_library_recompile_material: *mut crate::ffi::UFunctionOpague,
    pub u_material_editing_library_layout_material_function_expressions: *mut crate::ffi::UFunctionOpague,
    pub u_material_editing_library_layout_material_expressions: *mut crate::ffi::UFunctionOpague,
    pub u_material_editing_library_has_material_usage: *mut crate::ffi::UFunctionOpague,
    pub u_material_editing_library_get_vector_parameter_source: *mut crate::ffi::UFunctionOpague,
    pub u_material_editing_library_get_vector_parameter_names: *mut crate::ffi::UFunctionOpague,
    pub u_material_editing_library_get_used_textures: *mut crate::ffi::UFunctionOpague,
    pub u_material_editing_library_get_texture_parameter_source: *mut crate::ffi::UFunctionOpague,
    pub u_material_editing_library_get_texture_parameter_names: *mut crate::ffi::UFunctionOpague,
    pub u_material_editing_library_get_statistics: *mut crate::ffi::UFunctionOpague,
    pub u_material_editing_library_get_static_switch_parameter_source: *mut crate::ffi::UFunctionOpague,
    pub u_material_editing_library_get_static_switch_parameter_names: *mut crate::ffi::UFunctionOpague,
    pub u_material_editing_library_get_scalar_parameter_source: *mut crate::ffi::UFunctionOpague,
    pub u_material_editing_library_get_scalar_parameter_names: *mut crate::ffi::UFunctionOpague,
    pub u_material_editing_library_get_num_material_expressions_in_function: *mut crate::ffi::UFunctionOpague,
    pub u_material_editing_library_get_num_material_expressions: *mut crate::ffi::UFunctionOpague,
    pub u_material_editing_library_get_nanite_override_material: *mut crate::ffi::UFunctionOpague,
    pub u_material_editing_library_get_material_selected_nodes: *mut crate::ffi::UFunctionOpague,
    pub u_material_editing_library_get_material_property_input_node_output_name: *mut crate::ffi::UFunctionOpague,
    pub u_material_editing_library_get_material_property_input_node: *mut crate::ffi::UFunctionOpague,
    pub u_material_editing_library_get_material_instance_vector_parameter_value: *mut crate::ffi::UFunctionOpague,
    pub u_material_editing_library_get_material_instance_texture_parameter_value: *mut crate::ffi::UFunctionOpague,
    pub u_material_editing_library_get_material_instance_static_switch_parameter_value: *mut crate::ffi::UFunctionOpague,
    pub u_material_editing_library_get_material_instance_sparse_volume_texture_parameter_value: *mut crate::ffi::UFunctionOpague,
    pub u_material_editing_library_get_material_instance_scalar_parameter_value: *mut crate::ffi::UFunctionOpague,
    pub u_material_editing_library_get_material_instance_runtime_virtual_texture_parameter_value: *mut crate::ffi::UFunctionOpague,
    pub u_material_editing_library_get_material_expression_node_position: *mut crate::ffi::UFunctionOpague,
    pub u_material_editing_library_get_material_expression_input_types: *mut crate::ffi::UFunctionOpague,
    pub u_material_editing_library_get_material_expression_input_names: *mut crate::ffi::UFunctionOpague,
    pub u_material_editing_library_get_material_default_vector_parameter_value: *mut crate::ffi::UFunctionOpague,
    pub u_material_editing_library_get_material_default_texture_parameter_value: *mut crate::ffi::UFunctionOpague,
    pub u_material_editing_library_get_material_default_static_switch_parameter_value: *mut crate::ffi::UFunctionOpague,
    pub u_material_editing_library_get_material_default_scalar_parameter_value: *mut crate::ffi::UFunctionOpague,
    pub u_material_editing_library_get_inputs_for_material_expression: *mut crate::ffi::UFunctionOpague,
    pub u_material_editing_library_get_input_node_output_name_for_material_expression: *mut crate::ffi::UFunctionOpague,
    pub u_material_editing_library_get_child_instances: *mut crate::ffi::UFunctionOpague,
    pub u_material_editing_library_duplicate_material_expression: *mut crate::ffi::UFunctionOpague,
    pub u_material_editing_library_delete_material_expression_in_function: *mut crate::ffi::UFunctionOpague,
    pub u_material_editing_library_delete_material_expression: *mut crate::ffi::UFunctionOpague,
    pub u_material_editing_library_delete_all_material_expressions_in_function: *mut crate::ffi::UFunctionOpague,
    pub u_material_editing_library_delete_all_material_expressions: *mut crate::ffi::UFunctionOpague,
    pub u_material_editing_library_create_material_expression_in_function: *mut crate::ffi::UFunctionOpague,
    pub u_material_editing_library_create_material_expression: *mut crate::ffi::UFunctionOpague,
    pub u_material_editing_library_connect_material_property: *mut crate::ffi::UFunctionOpague,
    pub u_material_editing_library_connect_material_expressions: *mut crate::ffi::UFunctionOpague,
    pub u_material_editing_library_clear_all_material_instance_parameters: *mut crate::ffi::UFunctionOpague,
}
impl FunctionPtrs {
    pub const fn empty() -> Self {
        Self {
            u_material_editing_library_update_material_instance: std::ptr::null_mut(),
            u_material_editing_library_update_material_function: std::ptr::null_mut(),
            u_material_editing_library_set_material_usage: std::ptr::null_mut(),
            u_material_editing_library_set_material_instance_vector_parameter_value: std::ptr::null_mut(),
            u_material_editing_library_set_material_instance_texture_parameter_value: std::ptr::null_mut(),
            u_material_editing_library_set_material_instance_static_switch_parameter_value: std::ptr::null_mut(),
            u_material_editing_library_set_material_instance_sparse_volume_texture_parameter_value: std::ptr::null_mut(),
            u_material_editing_library_set_material_instance_scalar_parameter_value: std::ptr::null_mut(),
            u_material_editing_library_set_material_instance_runtime_virtual_texture_parameter_value: std::ptr::null_mut(),
            u_material_editing_library_set_material_instance_parent: std::ptr::null_mut(),
            u_material_editing_library_rename_material_parameter_group: std::ptr::null_mut(),
            u_material_editing_library_rename_material_function_parameter_group: std::ptr::null_mut(),
            u_material_editing_library_recompile_material: std::ptr::null_mut(),
            u_material_editing_library_layout_material_function_expressions: std::ptr::null_mut(),
            u_material_editing_library_layout_material_expressions: std::ptr::null_mut(),
            u_material_editing_library_has_material_usage: std::ptr::null_mut(),
            u_material_editing_library_get_vector_parameter_source: std::ptr::null_mut(),
            u_material_editing_library_get_vector_parameter_names: std::ptr::null_mut(),
            u_material_editing_library_get_used_textures: std::ptr::null_mut(),
            u_material_editing_library_get_texture_parameter_source: std::ptr::null_mut(),
            u_material_editing_library_get_texture_parameter_names: std::ptr::null_mut(),
            u_material_editing_library_get_statistics: std::ptr::null_mut(),
            u_material_editing_library_get_static_switch_parameter_source: std::ptr::null_mut(),
            u_material_editing_library_get_static_switch_parameter_names: std::ptr::null_mut(),
            u_material_editing_library_get_scalar_parameter_source: std::ptr::null_mut(),
            u_material_editing_library_get_scalar_parameter_names: std::ptr::null_mut(),
            u_material_editing_library_get_num_material_expressions_in_function: std::ptr::null_mut(),
            u_material_editing_library_get_num_material_expressions: std::ptr::null_mut(),
            u_material_editing_library_get_nanite_override_material: std::ptr::null_mut(),
            u_material_editing_library_get_material_selected_nodes: std::ptr::null_mut(),
            u_material_editing_library_get_material_property_input_node_output_name: std::ptr::null_mut(),
            u_material_editing_library_get_material_property_input_node: std::ptr::null_mut(),
            u_material_editing_library_get_material_instance_vector_parameter_value: std::ptr::null_mut(),
            u_material_editing_library_get_material_instance_texture_parameter_value: std::ptr::null_mut(),
            u_material_editing_library_get_material_instance_static_switch_parameter_value: std::ptr::null_mut(),
            u_material_editing_library_get_material_instance_sparse_volume_texture_parameter_value: std::ptr::null_mut(),
            u_material_editing_library_get_material_instance_scalar_parameter_value: std::ptr::null_mut(),
            u_material_editing_library_get_material_instance_runtime_virtual_texture_parameter_value: std::ptr::null_mut(),
            u_material_editing_library_get_material_expression_node_position: std::ptr::null_mut(),
            u_material_editing_library_get_material_expression_input_types: std::ptr::null_mut(),
            u_material_editing_library_get_material_expression_input_names: std::ptr::null_mut(),
            u_material_editing_library_get_material_default_vector_parameter_value: std::ptr::null_mut(),
            u_material_editing_library_get_material_default_texture_parameter_value: std::ptr::null_mut(),
            u_material_editing_library_get_material_default_static_switch_parameter_value: std::ptr::null_mut(),
            u_material_editing_library_get_material_default_scalar_parameter_value: std::ptr::null_mut(),
            u_material_editing_library_get_inputs_for_material_expression: std::ptr::null_mut(),
            u_material_editing_library_get_input_node_output_name_for_material_expression: std::ptr::null_mut(),
            u_material_editing_library_get_child_instances: std::ptr::null_mut(),
            u_material_editing_library_duplicate_material_expression: std::ptr::null_mut(),
            u_material_editing_library_delete_material_expression_in_function: std::ptr::null_mut(),
            u_material_editing_library_delete_material_expression: std::ptr::null_mut(),
            u_material_editing_library_delete_all_material_expressions_in_function: std::ptr::null_mut(),
            u_material_editing_library_delete_all_material_expressions: std::ptr::null_mut(),
            u_material_editing_library_create_material_expression_in_function: std::ptr::null_mut(),
            u_material_editing_library_create_material_expression: std::ptr::null_mut(),
            u_material_editing_library_connect_material_property: std::ptr::null_mut(),
            u_material_editing_library_connect_material_expressions: std::ptr::null_mut(),
            u_material_editing_library_clear_all_material_instance_parameters: std::ptr::null_mut(),
        }
    }
}
pub fn initialize() {
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UMaterialEditingLibrary::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("UpdateMaterialInstance"),
            &raw mut __FUNCTION_PTRS.u_material_editing_library_update_material_instance,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("UpdateMaterialFunction"),
            &raw mut __FUNCTION_PTRS.u_material_editing_library_update_material_function,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetMaterialUsage"),
            &raw mut __FUNCTION_PTRS.u_material_editing_library_set_material_usage,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetMaterialInstanceVectorParameterValue"),
            &raw mut __FUNCTION_PTRS
                .u_material_editing_library_set_material_instance_vector_parameter_value,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetMaterialInstanceTextureParameterValue"),
            &raw mut __FUNCTION_PTRS
                .u_material_editing_library_set_material_instance_texture_parameter_value,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetMaterialInstanceStaticSwitchParameterValue"),
            &raw mut __FUNCTION_PTRS
                .u_material_editing_library_set_material_instance_static_switch_parameter_value,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from(
                "SetMaterialInstanceSparseVolumeTextureParameterValue",
            ),
            &raw mut __FUNCTION_PTRS
                .u_material_editing_library_set_material_instance_sparse_volume_texture_parameter_value,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetMaterialInstanceScalarParameterValue"),
            &raw mut __FUNCTION_PTRS
                .u_material_editing_library_set_material_instance_scalar_parameter_value,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from(
                "SetMaterialInstanceRuntimeVirtualTextureParameterValue",
            ),
            &raw mut __FUNCTION_PTRS
                .u_material_editing_library_set_material_instance_runtime_virtual_texture_parameter_value,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetMaterialInstanceParent"),
            &raw mut __FUNCTION_PTRS
                .u_material_editing_library_set_material_instance_parent,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RenameMaterialParameterGroup"),
            &raw mut __FUNCTION_PTRS
                .u_material_editing_library_rename_material_parameter_group,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RenameMaterialFunctionParameterGroup"),
            &raw mut __FUNCTION_PTRS
                .u_material_editing_library_rename_material_function_parameter_group,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RecompileMaterial"),
            &raw mut __FUNCTION_PTRS.u_material_editing_library_recompile_material,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("LayoutMaterialFunctionExpressions"),
            &raw mut __FUNCTION_PTRS
                .u_material_editing_library_layout_material_function_expressions,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("LayoutMaterialExpressions"),
            &raw mut __FUNCTION_PTRS
                .u_material_editing_library_layout_material_expressions,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("HasMaterialUsage"),
            &raw mut __FUNCTION_PTRS.u_material_editing_library_has_material_usage,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetVectorParameterSource"),
            &raw mut __FUNCTION_PTRS
                .u_material_editing_library_get_vector_parameter_source,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetVectorParameterNames"),
            &raw mut __FUNCTION_PTRS
                .u_material_editing_library_get_vector_parameter_names,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetUsedTextures"),
            &raw mut __FUNCTION_PTRS.u_material_editing_library_get_used_textures,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetTextureParameterSource"),
            &raw mut __FUNCTION_PTRS
                .u_material_editing_library_get_texture_parameter_source,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetTextureParameterNames"),
            &raw mut __FUNCTION_PTRS
                .u_material_editing_library_get_texture_parameter_names,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetStatistics"),
            &raw mut __FUNCTION_PTRS.u_material_editing_library_get_statistics,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetStaticSwitchParameterSource"),
            &raw mut __FUNCTION_PTRS
                .u_material_editing_library_get_static_switch_parameter_source,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetStaticSwitchParameterNames"),
            &raw mut __FUNCTION_PTRS
                .u_material_editing_library_get_static_switch_parameter_names,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetScalarParameterSource"),
            &raw mut __FUNCTION_PTRS
                .u_material_editing_library_get_scalar_parameter_source,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetScalarParameterNames"),
            &raw mut __FUNCTION_PTRS
                .u_material_editing_library_get_scalar_parameter_names,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetNumMaterialExpressionsInFunction"),
            &raw mut __FUNCTION_PTRS
                .u_material_editing_library_get_num_material_expressions_in_function,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetNumMaterialExpressions"),
            &raw mut __FUNCTION_PTRS
                .u_material_editing_library_get_num_material_expressions,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetNaniteOverrideMaterial"),
            &raw mut __FUNCTION_PTRS
                .u_material_editing_library_get_nanite_override_material,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetMaterialSelectedNodes"),
            &raw mut __FUNCTION_PTRS
                .u_material_editing_library_get_material_selected_nodes,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetMaterialPropertyInputNodeOutputName"),
            &raw mut __FUNCTION_PTRS
                .u_material_editing_library_get_material_property_input_node_output_name,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetMaterialPropertyInputNode"),
            &raw mut __FUNCTION_PTRS
                .u_material_editing_library_get_material_property_input_node,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetMaterialInstanceVectorParameterValue"),
            &raw mut __FUNCTION_PTRS
                .u_material_editing_library_get_material_instance_vector_parameter_value,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetMaterialInstanceTextureParameterValue"),
            &raw mut __FUNCTION_PTRS
                .u_material_editing_library_get_material_instance_texture_parameter_value,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetMaterialInstanceStaticSwitchParameterValue"),
            &raw mut __FUNCTION_PTRS
                .u_material_editing_library_get_material_instance_static_switch_parameter_value,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from(
                "GetMaterialInstanceSparseVolumeTextureParameterValue",
            ),
            &raw mut __FUNCTION_PTRS
                .u_material_editing_library_get_material_instance_sparse_volume_texture_parameter_value,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetMaterialInstanceScalarParameterValue"),
            &raw mut __FUNCTION_PTRS
                .u_material_editing_library_get_material_instance_scalar_parameter_value,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from(
                "GetMaterialInstanceRuntimeVirtualTextureParameterValue",
            ),
            &raw mut __FUNCTION_PTRS
                .u_material_editing_library_get_material_instance_runtime_virtual_texture_parameter_value,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetMaterialExpressionNodePosition"),
            &raw mut __FUNCTION_PTRS
                .u_material_editing_library_get_material_expression_node_position,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetMaterialExpressionInputTypes"),
            &raw mut __FUNCTION_PTRS
                .u_material_editing_library_get_material_expression_input_types,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetMaterialExpressionInputNames"),
            &raw mut __FUNCTION_PTRS
                .u_material_editing_library_get_material_expression_input_names,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetMaterialDefaultVectorParameterValue"),
            &raw mut __FUNCTION_PTRS
                .u_material_editing_library_get_material_default_vector_parameter_value,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetMaterialDefaultTextureParameterValue"),
            &raw mut __FUNCTION_PTRS
                .u_material_editing_library_get_material_default_texture_parameter_value,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetMaterialDefaultStaticSwitchParameterValue"),
            &raw mut __FUNCTION_PTRS
                .u_material_editing_library_get_material_default_static_switch_parameter_value,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetMaterialDefaultScalarParameterValue"),
            &raw mut __FUNCTION_PTRS
                .u_material_editing_library_get_material_default_scalar_parameter_value,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetInputsForMaterialExpression"),
            &raw mut __FUNCTION_PTRS
                .u_material_editing_library_get_inputs_for_material_expression,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetInputNodeOutputNameForMaterialExpression"),
            &raw mut __FUNCTION_PTRS
                .u_material_editing_library_get_input_node_output_name_for_material_expression,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetChildInstances"),
            &raw mut __FUNCTION_PTRS.u_material_editing_library_get_child_instances,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("DuplicateMaterialExpression"),
            &raw mut __FUNCTION_PTRS
                .u_material_editing_library_duplicate_material_expression,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("DeleteMaterialExpressionInFunction"),
            &raw mut __FUNCTION_PTRS
                .u_material_editing_library_delete_material_expression_in_function,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("DeleteMaterialExpression"),
            &raw mut __FUNCTION_PTRS
                .u_material_editing_library_delete_material_expression,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("DeleteAllMaterialExpressionsInFunction"),
            &raw mut __FUNCTION_PTRS
                .u_material_editing_library_delete_all_material_expressions_in_function,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("DeleteAllMaterialExpressions"),
            &raw mut __FUNCTION_PTRS
                .u_material_editing_library_delete_all_material_expressions,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CreateMaterialExpressionInFunction"),
            &raw mut __FUNCTION_PTRS
                .u_material_editing_library_create_material_expression_in_function,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CreateMaterialExpression"),
            &raw mut __FUNCTION_PTRS
                .u_material_editing_library_create_material_expression,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ConnectMaterialProperty"),
            &raw mut __FUNCTION_PTRS.u_material_editing_library_connect_material_property,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ConnectMaterialExpressions"),
            &raw mut __FUNCTION_PTRS
                .u_material_editing_library_connect_material_expressions,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ClearAllMaterialInstanceParameters"),
            &raw mut __FUNCTION_PTRS
                .u_material_editing_library_clear_all_material_instance_parameters,
        );
    }
}
#[repr(C, align(4))]
pub struct FMaterialStatistics {
    pub num_vertex_shader_instructions: i32,
    pub num_pixel_shader_instructions: i32,
    pub num_samplers: i32,
    pub num_vertex_texture_samples: i32,
    pub num_pixel_texture_samples: i32,
    pub num_virtual_texture_samples: i32,
    pub num_uv_scalars: i32,
    pub num_interpolator_scalars: i32,
}
impl FMaterialStatistics {}
#[repr(C, align(8))]
pub struct UMaterialEditorMenuContext {
    __padding_end: [u8; 64],
}
impl UMaterialEditorMenuContext {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMaterialEditorMenuContext")
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
pub struct UMaterialEditorSettings {
    __padding_end: [u8; 216],
}
impl UMaterialEditorSettings {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMaterialEditorSettings")
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
pub struct UMaterialEditingLibrary {
    __padding_end: [u8; 48],
}
impl UMaterialEditingLibrary {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMaterialEditingLibrary")
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
    pub fn update_material_instance(
        instance: UPtr<crate::bindings::engine::UMaterialInstanceConstant>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::material_editor::__FUNCTION_PTRS
                    .u_material_editing_library_update_material_instance,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &instance,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::engine::UMaterialInstanceConstant>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::material_editor::UMaterialEditingLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::material_editor::__FUNCTION_PTRS
                    .u_material_editing_library_update_material_instance,
                __buffer,
            )
        };
    }
    pub fn update_material_function(
        material_function: UPtr<crate::bindings::engine::UMaterialFunctionInterface>,
        preview_material: UPtr<crate::bindings::engine::UMaterial>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::material_editor::__FUNCTION_PTRS
                    .u_material_editing_library_update_material_function,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &material_function,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::engine::UMaterialFunctionInterface>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &preview_material,
                __buffer.add(8).cast::<UPtr<crate::bindings::engine::UMaterial>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::material_editor::UMaterialEditingLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::material_editor::__FUNCTION_PTRS
                    .u_material_editing_library_update_material_function,
                __buffer,
            )
        };
    }
    pub fn set_material_usage(
        material: UPtr<crate::bindings::engine::UMaterial>,
        usage: crate::bindings::engine::EMaterialUsage,
        b_needs_recompile: &mut bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<11>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::material_editor::__FUNCTION_PTRS
                    .u_material_editing_library_set_material_usage,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &material,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::UMaterial>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &usage,
                __buffer.add(8).cast::<crate::bindings::engine::EMaterialUsage>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                b_needs_recompile,
                __buffer.add(9).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::material_editor::UMaterialEditingLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::material_editor::__FUNCTION_PTRS
                    .u_material_editing_library_set_material_usage,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(9).cast::<bool>().swap(b_needs_recompile);
        }
        unsafe { __buffer.add(10).cast::<bool>().read() }
    }
    pub fn set_material_instance_vector_parameter_value(
        instance: UPtr<crate::bindings::engine::UMaterialInstanceConstant>,
        parameter_name: FName,
        value: crate::bindings::core_u_object::FLinearColor,
        association: crate::bindings::engine::EMaterialParameterAssociation,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<38>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::material_editor::__FUNCTION_PTRS
                    .u_material_editing_library_set_material_instance_vector_parameter_value,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &instance,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::engine::UMaterialInstanceConstant>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &parameter_name,
                __buffer.add(8).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &value,
                __buffer.add(20).cast::<crate::bindings::core_u_object::FLinearColor>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &association,
                __buffer
                    .add(36)
                    .cast::<crate::bindings::engine::EMaterialParameterAssociation>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::material_editor::UMaterialEditingLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::material_editor::__FUNCTION_PTRS
                    .u_material_editing_library_set_material_instance_vector_parameter_value,
                __buffer,
            )
        };
        unsafe { __buffer.add(37).cast::<bool>().read() }
    }
    pub fn set_material_instance_texture_parameter_value(
        instance: UPtr<crate::bindings::engine::UMaterialInstanceConstant>,
        parameter_name: FName,
        value: UPtr<crate::bindings::engine::UTexture>,
        association: crate::bindings::engine::EMaterialParameterAssociation,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<34>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::material_editor::__FUNCTION_PTRS
                    .u_material_editing_library_set_material_instance_texture_parameter_value,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &instance,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::engine::UMaterialInstanceConstant>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &parameter_name,
                __buffer.add(8).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &value,
                __buffer.add(24).cast::<UPtr<crate::bindings::engine::UTexture>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &association,
                __buffer
                    .add(32)
                    .cast::<crate::bindings::engine::EMaterialParameterAssociation>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::material_editor::UMaterialEditingLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::material_editor::__FUNCTION_PTRS
                    .u_material_editing_library_set_material_instance_texture_parameter_value,
                __buffer,
            )
        };
        unsafe { __buffer.add(33).cast::<bool>().read() }
    }
    pub fn set_material_instance_static_switch_parameter_value(
        instance: UPtr<crate::bindings::engine::UMaterialInstanceConstant>,
        parameter_name: FName,
        value: bool,
        association: crate::bindings::engine::EMaterialParameterAssociation,
        b_update_material_instance: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::material_editor::__FUNCTION_PTRS
                    .u_material_editing_library_set_material_instance_static_switch_parameter_value,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &instance,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::engine::UMaterialInstanceConstant>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &parameter_name,
                __buffer.add(8).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&value, __buffer.add(20).cast::<bool>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &association,
                __buffer
                    .add(21)
                    .cast::<crate::bindings::engine::EMaterialParameterAssociation>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_update_material_instance,
                __buffer.add(22).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::material_editor::UMaterialEditingLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::material_editor::__FUNCTION_PTRS
                    .u_material_editing_library_set_material_instance_static_switch_parameter_value,
                __buffer,
            )
        };
        unsafe { __buffer.add(23).cast::<bool>().read() }
    }
    pub fn set_material_instance_sparse_volume_texture_parameter_value(
        instance: UPtr<crate::bindings::engine::UMaterialInstanceConstant>,
        parameter_name: FName,
        value: UPtr<crate::bindings::engine::USparseVolumeTexture>,
        association: crate::bindings::engine::EMaterialParameterAssociation,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<34>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::material_editor::__FUNCTION_PTRS
                    .u_material_editing_library_set_material_instance_sparse_volume_texture_parameter_value,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &instance,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::engine::UMaterialInstanceConstant>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &parameter_name,
                __buffer.add(8).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &value,
                __buffer
                    .add(24)
                    .cast::<UPtr<crate::bindings::engine::USparseVolumeTexture>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &association,
                __buffer
                    .add(32)
                    .cast::<crate::bindings::engine::EMaterialParameterAssociation>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::material_editor::UMaterialEditingLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::material_editor::__FUNCTION_PTRS
                    .u_material_editing_library_set_material_instance_sparse_volume_texture_parameter_value,
                __buffer,
            )
        };
        unsafe { __buffer.add(33).cast::<bool>().read() }
    }
    pub fn set_material_instance_scalar_parameter_value(
        instance: UPtr<crate::bindings::engine::UMaterialInstanceConstant>,
        parameter_name: FName,
        value: f32,
        association: crate::bindings::engine::EMaterialParameterAssociation,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<26>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::material_editor::__FUNCTION_PTRS
                    .u_material_editing_library_set_material_instance_scalar_parameter_value,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &instance,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::engine::UMaterialInstanceConstant>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &parameter_name,
                __buffer.add(8).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&value, __buffer.add(20).cast::<f32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &association,
                __buffer
                    .add(24)
                    .cast::<crate::bindings::engine::EMaterialParameterAssociation>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::material_editor::UMaterialEditingLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::material_editor::__FUNCTION_PTRS
                    .u_material_editing_library_set_material_instance_scalar_parameter_value,
                __buffer,
            )
        };
        unsafe { __buffer.add(25).cast::<bool>().read() }
    }
    pub fn set_material_instance_runtime_virtual_texture_parameter_value(
        instance: UPtr<crate::bindings::engine::UMaterialInstanceConstant>,
        parameter_name: FName,
        value: UPtr<crate::bindings::engine::URuntimeVirtualTexture>,
        association: crate::bindings::engine::EMaterialParameterAssociation,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<34>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::material_editor::__FUNCTION_PTRS
                    .u_material_editing_library_set_material_instance_runtime_virtual_texture_parameter_value,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &instance,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::engine::UMaterialInstanceConstant>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &parameter_name,
                __buffer.add(8).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &value,
                __buffer
                    .add(24)
                    .cast::<UPtr<crate::bindings::engine::URuntimeVirtualTexture>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &association,
                __buffer
                    .add(32)
                    .cast::<crate::bindings::engine::EMaterialParameterAssociation>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::material_editor::UMaterialEditingLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::material_editor::__FUNCTION_PTRS
                    .u_material_editing_library_set_material_instance_runtime_virtual_texture_parameter_value,
                __buffer,
            )
        };
        unsafe { __buffer.add(33).cast::<bool>().read() }
    }
    pub fn set_material_instance_parent(
        instance: UPtr<crate::bindings::engine::UMaterialInstanceConstant>,
        new_parent: UPtr<crate::bindings::engine::UMaterialInterface>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::material_editor::__FUNCTION_PTRS
                    .u_material_editing_library_set_material_instance_parent,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &instance,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::engine::UMaterialInstanceConstant>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &new_parent,
                __buffer
                    .add(8)
                    .cast::<UPtr<crate::bindings::engine::UMaterialInterface>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::material_editor::UMaterialEditingLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::material_editor::__FUNCTION_PTRS
                    .u_material_editing_library_set_material_instance_parent,
                __buffer,
            )
        };
    }
    pub fn rename_material_parameter_group(
        material: UPtr<crate::bindings::engine::UMaterial>,
        old_group_name: FName,
        new_group_name: FName,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<33>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::material_editor::__FUNCTION_PTRS
                    .u_material_editing_library_rename_material_parameter_group,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &material,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::UMaterial>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &old_group_name,
                __buffer.add(8).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &new_group_name,
                __buffer.add(20).cast::<FName>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::material_editor::UMaterialEditingLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::material_editor::__FUNCTION_PTRS
                    .u_material_editing_library_rename_material_parameter_group,
                __buffer,
            )
        };
        unsafe { __buffer.add(32).cast::<bool>().read() }
    }
    pub fn rename_material_function_parameter_group(
        material_function: UPtr<crate::bindings::engine::UMaterialFunctionInterface>,
        old_group_name: FName,
        new_group_name: FName,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<33>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::material_editor::__FUNCTION_PTRS
                    .u_material_editing_library_rename_material_function_parameter_group,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &material_function,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::engine::UMaterialFunctionInterface>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &old_group_name,
                __buffer.add(8).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &new_group_name,
                __buffer.add(20).cast::<FName>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::material_editor::UMaterialEditingLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::material_editor::__FUNCTION_PTRS
                    .u_material_editing_library_rename_material_function_parameter_group,
                __buffer,
            )
        };
        unsafe { __buffer.add(32).cast::<bool>().read() }
    }
    pub fn recompile_material(material: UPtr<crate::bindings::engine::UMaterial>) {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::material_editor::__FUNCTION_PTRS
                    .u_material_editing_library_recompile_material,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &material,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::UMaterial>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::material_editor::UMaterialEditingLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::material_editor::__FUNCTION_PTRS
                    .u_material_editing_library_recompile_material,
                __buffer,
            )
        };
    }
    pub fn layout_material_function_expressions(
        material_function: UPtr<crate::bindings::engine::UMaterialFunction>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::material_editor::__FUNCTION_PTRS
                    .u_material_editing_library_layout_material_function_expressions,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &material_function,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::engine::UMaterialFunction>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::material_editor::UMaterialEditingLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::material_editor::__FUNCTION_PTRS
                    .u_material_editing_library_layout_material_function_expressions,
                __buffer,
            )
        };
    }
    pub fn layout_material_expressions(
        material: UPtr<crate::bindings::engine::UMaterial>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::material_editor::__FUNCTION_PTRS
                    .u_material_editing_library_layout_material_expressions,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &material,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::UMaterial>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::material_editor::UMaterialEditingLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::material_editor::__FUNCTION_PTRS
                    .u_material_editing_library_layout_material_expressions,
                __buffer,
            )
        };
    }
    pub fn has_material_usage(
        material: UPtr<crate::bindings::engine::UMaterial>,
        usage: crate::bindings::engine::EMaterialUsage,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<10>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::material_editor::__FUNCTION_PTRS
                    .u_material_editing_library_has_material_usage,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &material,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::UMaterial>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &usage,
                __buffer.add(8).cast::<crate::bindings::engine::EMaterialUsage>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::material_editor::UMaterialEditingLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::material_editor::__FUNCTION_PTRS
                    .u_material_editing_library_has_material_usage,
                __buffer,
            )
        };
        unsafe { __buffer.add(9).cast::<bool>().read() }
    }
    pub fn get_vector_parameter_source(
        material: UPtr<crate::bindings::engine::UMaterialInterface>,
        parameter_name: FName,
        parameter_source: &mut crate::bindings::core_u_object::FSoftObjectPath,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<65>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::material_editor::__FUNCTION_PTRS
                    .u_material_editing_library_get_vector_parameter_source,
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
            std::ptr::copy_nonoverlapping(
                &parameter_name,
                __buffer.add(8).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                parameter_source,
                __buffer
                    .add(24)
                    .cast::<crate::bindings::core_u_object::FSoftObjectPath>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::material_editor::UMaterialEditingLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::material_editor::__FUNCTION_PTRS
                    .u_material_editing_library_get_vector_parameter_source,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(24)
                .cast::<crate::bindings::core_u_object::FSoftObjectPath>()
                .swap(parameter_source);
        }
        unsafe { __buffer.add(64).cast::<bool>().read() }
    }
    pub fn get_vector_parameter_names(
        material: UPtr<crate::bindings::engine::UMaterialInterface>,
        parameter_names: &mut TArray<FName>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::material_editor::__FUNCTION_PTRS
                    .u_material_editing_library_get_vector_parameter_names,
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
            std::ptr::copy_nonoverlapping(
                parameter_names,
                __buffer.add(8).cast::<TArray<FName>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::material_editor::UMaterialEditingLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::material_editor::__FUNCTION_PTRS
                    .u_material_editing_library_get_vector_parameter_names,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(8).cast::<TArray<FName>>().swap(parameter_names);
        }
    }
    pub fn get_used_textures(
        material: UPtr<crate::bindings::engine::UMaterial>,
    ) -> TArray<UPtr<crate::bindings::engine::UTexture>> {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::material_editor::__FUNCTION_PTRS
                    .u_material_editing_library_get_used_textures,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &material,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::UMaterial>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::material_editor::UMaterialEditingLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::material_editor::__FUNCTION_PTRS
                    .u_material_editing_library_get_used_textures,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(8)
                .cast::<TArray<UPtr<crate::bindings::engine::UTexture>>>()
                .read()
        }
    }
    pub fn get_texture_parameter_source(
        material: UPtr<crate::bindings::engine::UMaterialInterface>,
        parameter_name: FName,
        parameter_source: &mut crate::bindings::core_u_object::FSoftObjectPath,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<65>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::material_editor::__FUNCTION_PTRS
                    .u_material_editing_library_get_texture_parameter_source,
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
            std::ptr::copy_nonoverlapping(
                &parameter_name,
                __buffer.add(8).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                parameter_source,
                __buffer
                    .add(24)
                    .cast::<crate::bindings::core_u_object::FSoftObjectPath>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::material_editor::UMaterialEditingLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::material_editor::__FUNCTION_PTRS
                    .u_material_editing_library_get_texture_parameter_source,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(24)
                .cast::<crate::bindings::core_u_object::FSoftObjectPath>()
                .swap(parameter_source);
        }
        unsafe { __buffer.add(64).cast::<bool>().read() }
    }
    pub fn get_texture_parameter_names(
        material: UPtr<crate::bindings::engine::UMaterialInterface>,
        parameter_names: &mut TArray<FName>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::material_editor::__FUNCTION_PTRS
                    .u_material_editing_library_get_texture_parameter_names,
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
            std::ptr::copy_nonoverlapping(
                parameter_names,
                __buffer.add(8).cast::<TArray<FName>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::material_editor::UMaterialEditingLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::material_editor::__FUNCTION_PTRS
                    .u_material_editing_library_get_texture_parameter_names,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(8).cast::<TArray<FName>>().swap(parameter_names);
        }
    }
    pub fn get_statistics(
        material: UPtr<crate::bindings::engine::UMaterialInterface>,
    ) -> FMaterialStatistics {
        let mut __stack = crate::core_data::StackAlloc::<40>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::material_editor::__FUNCTION_PTRS
                    .u_material_editing_library_get_statistics,
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
        let __object_ptr = crate::bindings::material_editor::UMaterialEditingLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::material_editor::__FUNCTION_PTRS
                    .u_material_editing_library_get_statistics,
                __buffer,
            )
        };
        unsafe { __buffer.add(8).cast::<FMaterialStatistics>().read() }
    }
    pub fn get_static_switch_parameter_source(
        material: UPtr<crate::bindings::engine::UMaterialInterface>,
        parameter_name: FName,
        parameter_source: &mut crate::bindings::core_u_object::FSoftObjectPath,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<65>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::material_editor::__FUNCTION_PTRS
                    .u_material_editing_library_get_static_switch_parameter_source,
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
            std::ptr::copy_nonoverlapping(
                &parameter_name,
                __buffer.add(8).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                parameter_source,
                __buffer
                    .add(24)
                    .cast::<crate::bindings::core_u_object::FSoftObjectPath>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::material_editor::UMaterialEditingLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::material_editor::__FUNCTION_PTRS
                    .u_material_editing_library_get_static_switch_parameter_source,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(24)
                .cast::<crate::bindings::core_u_object::FSoftObjectPath>()
                .swap(parameter_source);
        }
        unsafe { __buffer.add(64).cast::<bool>().read() }
    }
    pub fn get_static_switch_parameter_names(
        material: UPtr<crate::bindings::engine::UMaterialInterface>,
        parameter_names: &mut TArray<FName>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::material_editor::__FUNCTION_PTRS
                    .u_material_editing_library_get_static_switch_parameter_names,
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
            std::ptr::copy_nonoverlapping(
                parameter_names,
                __buffer.add(8).cast::<TArray<FName>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::material_editor::UMaterialEditingLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::material_editor::__FUNCTION_PTRS
                    .u_material_editing_library_get_static_switch_parameter_names,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(8).cast::<TArray<FName>>().swap(parameter_names);
        }
    }
    pub fn get_scalar_parameter_source(
        material: UPtr<crate::bindings::engine::UMaterialInterface>,
        parameter_name: FName,
        parameter_source: &mut crate::bindings::core_u_object::FSoftObjectPath,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<65>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::material_editor::__FUNCTION_PTRS
                    .u_material_editing_library_get_scalar_parameter_source,
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
            std::ptr::copy_nonoverlapping(
                &parameter_name,
                __buffer.add(8).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                parameter_source,
                __buffer
                    .add(24)
                    .cast::<crate::bindings::core_u_object::FSoftObjectPath>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::material_editor::UMaterialEditingLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::material_editor::__FUNCTION_PTRS
                    .u_material_editing_library_get_scalar_parameter_source,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(24)
                .cast::<crate::bindings::core_u_object::FSoftObjectPath>()
                .swap(parameter_source);
        }
        unsafe { __buffer.add(64).cast::<bool>().read() }
    }
    pub fn get_scalar_parameter_names(
        material: UPtr<crate::bindings::engine::UMaterialInterface>,
        parameter_names: &mut TArray<FName>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::material_editor::__FUNCTION_PTRS
                    .u_material_editing_library_get_scalar_parameter_names,
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
            std::ptr::copy_nonoverlapping(
                parameter_names,
                __buffer.add(8).cast::<TArray<FName>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::material_editor::UMaterialEditingLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::material_editor::__FUNCTION_PTRS
                    .u_material_editing_library_get_scalar_parameter_names,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(8).cast::<TArray<FName>>().swap(parameter_names);
        }
    }
    pub fn get_num_material_expressions_in_function(
        material_function: UPtr<crate::bindings::engine::UMaterialFunction>,
    ) -> i32 {
        let mut __stack = crate::core_data::StackAlloc::<12>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::material_editor::__FUNCTION_PTRS
                    .u_material_editing_library_get_num_material_expressions_in_function,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &material_function,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::engine::UMaterialFunction>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::material_editor::UMaterialEditingLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::material_editor::__FUNCTION_PTRS
                    .u_material_editing_library_get_num_material_expressions_in_function,
                __buffer,
            )
        };
        unsafe { __buffer.add(8).cast::<i32>().read() }
    }
    pub fn get_num_material_expressions(
        material: UPtr<crate::bindings::engine::UMaterial>,
    ) -> i32 {
        let mut __stack = crate::core_data::StackAlloc::<12>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::material_editor::__FUNCTION_PTRS
                    .u_material_editing_library_get_num_material_expressions,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &material,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::UMaterial>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::material_editor::UMaterialEditingLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::material_editor::__FUNCTION_PTRS
                    .u_material_editing_library_get_num_material_expressions,
                __buffer,
            )
        };
        unsafe { __buffer.add(8).cast::<i32>().read() }
    }
    pub fn get_nanite_override_material(
        material: UPtr<crate::bindings::engine::UMaterialInterface>,
    ) -> UPtr<crate::bindings::engine::UMaterialInterface> {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::material_editor::__FUNCTION_PTRS
                    .u_material_editing_library_get_nanite_override_material,
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
        let __object_ptr = crate::bindings::material_editor::UMaterialEditingLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::material_editor::__FUNCTION_PTRS
                    .u_material_editing_library_get_nanite_override_material,
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
    pub fn get_material_selected_nodes(
        material: UPtr<crate::bindings::engine::UMaterial>,
    ) -> TSet<UPtr<crate::bindings::core_u_object::UObject>> {
        let mut __stack = crate::core_data::StackAlloc::<88>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::material_editor::__FUNCTION_PTRS
                    .u_material_editing_library_get_material_selected_nodes,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &material,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::UMaterial>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::material_editor::UMaterialEditingLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::material_editor::__FUNCTION_PTRS
                    .u_material_editing_library_get_material_selected_nodes,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(8)
                .cast::<TSet<UPtr<crate::bindings::core_u_object::UObject>>>()
                .read()
        }
    }
    pub fn get_material_property_input_node_output_name(
        material: UPtr<crate::bindings::engine::UMaterial>,
        property: crate::bindings::engine::EMaterialProperty,
    ) -> FString {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::material_editor::__FUNCTION_PTRS
                    .u_material_editing_library_get_material_property_input_node_output_name,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &material,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::UMaterial>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &property,
                __buffer.add(8).cast::<crate::bindings::engine::EMaterialProperty>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::material_editor::UMaterialEditingLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::material_editor::__FUNCTION_PTRS
                    .u_material_editing_library_get_material_property_input_node_output_name,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<FString>().read() }
    }
    pub fn get_material_property_input_node(
        material: UPtr<crate::bindings::engine::UMaterial>,
        property: crate::bindings::engine::EMaterialProperty,
    ) -> UPtr<crate::bindings::engine::UMaterialExpression> {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::material_editor::__FUNCTION_PTRS
                    .u_material_editing_library_get_material_property_input_node,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &material,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::UMaterial>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &property,
                __buffer.add(8).cast::<crate::bindings::engine::EMaterialProperty>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::material_editor::UMaterialEditingLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::material_editor::__FUNCTION_PTRS
                    .u_material_editing_library_get_material_property_input_node,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(16)
                .cast::<UPtr<crate::bindings::engine::UMaterialExpression>>()
                .read()
        }
    }
    pub fn get_material_instance_vector_parameter_value(
        instance: UPtr<crate::bindings::engine::UMaterialInstanceConstant>,
        parameter_name: FName,
        association: crate::bindings::engine::EMaterialParameterAssociation,
    ) -> crate::bindings::core_u_object::FLinearColor {
        let mut __stack = crate::core_data::StackAlloc::<40>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::material_editor::__FUNCTION_PTRS
                    .u_material_editing_library_get_material_instance_vector_parameter_value,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &instance,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::engine::UMaterialInstanceConstant>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &parameter_name,
                __buffer.add(8).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &association,
                __buffer
                    .add(20)
                    .cast::<crate::bindings::engine::EMaterialParameterAssociation>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::material_editor::UMaterialEditingLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::material_editor::__FUNCTION_PTRS
                    .u_material_editing_library_get_material_instance_vector_parameter_value,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(24)
                .cast::<crate::bindings::core_u_object::FLinearColor>()
                .read()
        }
    }
    pub fn get_material_instance_texture_parameter_value(
        instance: UPtr<crate::bindings::engine::UMaterialInstanceConstant>,
        parameter_name: FName,
        association: crate::bindings::engine::EMaterialParameterAssociation,
    ) -> UPtr<crate::bindings::engine::UTexture> {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::material_editor::__FUNCTION_PTRS
                    .u_material_editing_library_get_material_instance_texture_parameter_value,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &instance,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::engine::UMaterialInstanceConstant>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &parameter_name,
                __buffer.add(8).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &association,
                __buffer
                    .add(20)
                    .cast::<crate::bindings::engine::EMaterialParameterAssociation>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::material_editor::UMaterialEditingLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::material_editor::__FUNCTION_PTRS
                    .u_material_editing_library_get_material_instance_texture_parameter_value,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(24).cast::<UPtr<crate::bindings::engine::UTexture>>().read()
        }
    }
    pub fn get_material_instance_static_switch_parameter_value(
        instance: UPtr<crate::bindings::engine::UMaterialInstanceConstant>,
        parameter_name: FName,
        association: crate::bindings::engine::EMaterialParameterAssociation,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<22>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::material_editor::__FUNCTION_PTRS
                    .u_material_editing_library_get_material_instance_static_switch_parameter_value,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &instance,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::engine::UMaterialInstanceConstant>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &parameter_name,
                __buffer.add(8).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &association,
                __buffer
                    .add(20)
                    .cast::<crate::bindings::engine::EMaterialParameterAssociation>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::material_editor::UMaterialEditingLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::material_editor::__FUNCTION_PTRS
                    .u_material_editing_library_get_material_instance_static_switch_parameter_value,
                __buffer,
            )
        };
        unsafe { __buffer.add(21).cast::<bool>().read() }
    }
    pub fn get_material_instance_sparse_volume_texture_parameter_value(
        instance: UPtr<crate::bindings::engine::UMaterialInstanceConstant>,
        parameter_name: FName,
        association: crate::bindings::engine::EMaterialParameterAssociation,
    ) -> UPtr<crate::bindings::engine::USparseVolumeTexture> {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::material_editor::__FUNCTION_PTRS
                    .u_material_editing_library_get_material_instance_sparse_volume_texture_parameter_value,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &instance,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::engine::UMaterialInstanceConstant>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &parameter_name,
                __buffer.add(8).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &association,
                __buffer
                    .add(20)
                    .cast::<crate::bindings::engine::EMaterialParameterAssociation>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::material_editor::UMaterialEditingLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::material_editor::__FUNCTION_PTRS
                    .u_material_editing_library_get_material_instance_sparse_volume_texture_parameter_value,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(24)
                .cast::<UPtr<crate::bindings::engine::USparseVolumeTexture>>()
                .read()
        }
    }
    pub fn get_material_instance_scalar_parameter_value(
        instance: UPtr<crate::bindings::engine::UMaterialInstanceConstant>,
        parameter_name: FName,
        association: crate::bindings::engine::EMaterialParameterAssociation,
    ) -> f32 {
        let mut __stack = crate::core_data::StackAlloc::<28>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::material_editor::__FUNCTION_PTRS
                    .u_material_editing_library_get_material_instance_scalar_parameter_value,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &instance,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::engine::UMaterialInstanceConstant>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &parameter_name,
                __buffer.add(8).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &association,
                __buffer
                    .add(20)
                    .cast::<crate::bindings::engine::EMaterialParameterAssociation>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::material_editor::UMaterialEditingLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::material_editor::__FUNCTION_PTRS
                    .u_material_editing_library_get_material_instance_scalar_parameter_value,
                __buffer,
            )
        };
        unsafe { __buffer.add(24).cast::<f32>().read() }
    }
    pub fn get_material_instance_runtime_virtual_texture_parameter_value(
        instance: UPtr<crate::bindings::engine::UMaterialInstanceConstant>,
        parameter_name: FName,
        association: crate::bindings::engine::EMaterialParameterAssociation,
    ) -> UPtr<crate::bindings::engine::URuntimeVirtualTexture> {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::material_editor::__FUNCTION_PTRS
                    .u_material_editing_library_get_material_instance_runtime_virtual_texture_parameter_value,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &instance,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::engine::UMaterialInstanceConstant>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &parameter_name,
                __buffer.add(8).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &association,
                __buffer
                    .add(20)
                    .cast::<crate::bindings::engine::EMaterialParameterAssociation>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::material_editor::UMaterialEditingLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::material_editor::__FUNCTION_PTRS
                    .u_material_editing_library_get_material_instance_runtime_virtual_texture_parameter_value,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(24)
                .cast::<UPtr<crate::bindings::engine::URuntimeVirtualTexture>>()
                .read()
        }
    }
    pub fn get_material_expression_node_position(
        material_expression: UPtr<crate::bindings::engine::UMaterialExpression>,
        node_pos_x: &mut i32,
        node_pos_y: &mut i32,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::material_editor::__FUNCTION_PTRS
                    .u_material_editing_library_get_material_expression_node_position,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &material_expression,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::engine::UMaterialExpression>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(node_pos_x, __buffer.add(8).cast::<i32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(node_pos_y, __buffer.add(12).cast::<i32>(), 1);
        }
        let __object_ptr = crate::bindings::material_editor::UMaterialEditingLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::material_editor::__FUNCTION_PTRS
                    .u_material_editing_library_get_material_expression_node_position,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(8).cast::<i32>().swap(node_pos_x);
        }
        unsafe {
            __buffer.add(12).cast::<i32>().swap(node_pos_y);
        }
    }
    pub fn get_material_expression_input_types(
        material_expression: UPtr<crate::bindings::engine::UMaterialExpression>,
    ) -> TArray<i32> {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::material_editor::__FUNCTION_PTRS
                    .u_material_editing_library_get_material_expression_input_types,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &material_expression,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::engine::UMaterialExpression>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::material_editor::UMaterialEditingLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::material_editor::__FUNCTION_PTRS
                    .u_material_editing_library_get_material_expression_input_types,
                __buffer,
            )
        };
        unsafe { __buffer.add(8).cast::<TArray<i32>>().read() }
    }
    pub fn get_material_expression_input_names(
        material_expression: UPtr<crate::bindings::engine::UMaterialExpression>,
    ) -> TArray<FString> {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::material_editor::__FUNCTION_PTRS
                    .u_material_editing_library_get_material_expression_input_names,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &material_expression,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::engine::UMaterialExpression>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::material_editor::UMaterialEditingLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::material_editor::__FUNCTION_PTRS
                    .u_material_editing_library_get_material_expression_input_names,
                __buffer,
            )
        };
        unsafe { __buffer.add(8).cast::<TArray<FString>>().read() }
    }
    pub fn get_material_default_vector_parameter_value(
        material: UPtr<crate::bindings::engine::UMaterial>,
        parameter_name: FName,
    ) -> crate::bindings::core_u_object::FLinearColor {
        let mut __stack = crate::core_data::StackAlloc::<36>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::material_editor::__FUNCTION_PTRS
                    .u_material_editing_library_get_material_default_vector_parameter_value,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &material,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::UMaterial>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &parameter_name,
                __buffer.add(8).cast::<FName>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::material_editor::UMaterialEditingLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::material_editor::__FUNCTION_PTRS
                    .u_material_editing_library_get_material_default_vector_parameter_value,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(20)
                .cast::<crate::bindings::core_u_object::FLinearColor>()
                .read()
        }
    }
    pub fn get_material_default_texture_parameter_value(
        material: UPtr<crate::bindings::engine::UMaterial>,
        parameter_name: FName,
    ) -> UPtr<crate::bindings::engine::UTexture> {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::material_editor::__FUNCTION_PTRS
                    .u_material_editing_library_get_material_default_texture_parameter_value,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &material,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::UMaterial>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &parameter_name,
                __buffer.add(8).cast::<FName>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::material_editor::UMaterialEditingLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::material_editor::__FUNCTION_PTRS
                    .u_material_editing_library_get_material_default_texture_parameter_value,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(24).cast::<UPtr<crate::bindings::engine::UTexture>>().read()
        }
    }
    pub fn get_material_default_static_switch_parameter_value(
        material: UPtr<crate::bindings::engine::UMaterial>,
        parameter_name: FName,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<21>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::material_editor::__FUNCTION_PTRS
                    .u_material_editing_library_get_material_default_static_switch_parameter_value,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &material,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::UMaterial>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &parameter_name,
                __buffer.add(8).cast::<FName>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::material_editor::UMaterialEditingLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::material_editor::__FUNCTION_PTRS
                    .u_material_editing_library_get_material_default_static_switch_parameter_value,
                __buffer,
            )
        };
        unsafe { __buffer.add(20).cast::<bool>().read() }
    }
    pub fn get_material_default_scalar_parameter_value(
        material: UPtr<crate::bindings::engine::UMaterial>,
        parameter_name: FName,
    ) -> f32 {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::material_editor::__FUNCTION_PTRS
                    .u_material_editing_library_get_material_default_scalar_parameter_value,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &material,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::UMaterial>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &parameter_name,
                __buffer.add(8).cast::<FName>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::material_editor::UMaterialEditingLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::material_editor::__FUNCTION_PTRS
                    .u_material_editing_library_get_material_default_scalar_parameter_value,
                __buffer,
            )
        };
        unsafe { __buffer.add(20).cast::<f32>().read() }
    }
    pub fn get_inputs_for_material_expression(
        material: UPtr<crate::bindings::engine::UMaterial>,
        material_expression: UPtr<crate::bindings::engine::UMaterialExpression>,
    ) -> TArray<UPtr<crate::bindings::engine::UMaterialExpression>> {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::material_editor::__FUNCTION_PTRS
                    .u_material_editing_library_get_inputs_for_material_expression,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &material,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::UMaterial>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &material_expression,
                __buffer
                    .add(8)
                    .cast::<UPtr<crate::bindings::engine::UMaterialExpression>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::material_editor::UMaterialEditingLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::material_editor::__FUNCTION_PTRS
                    .u_material_editing_library_get_inputs_for_material_expression,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(16)
                .cast::<TArray<UPtr<crate::bindings::engine::UMaterialExpression>>>()
                .read()
        }
    }
    pub fn get_input_node_output_name_for_material_expression(
        material_expression: UPtr<crate::bindings::engine::UMaterialExpression>,
        input_node: UPtr<crate::bindings::engine::UMaterialExpression>,
        output_name: &mut FString,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<33>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::material_editor::__FUNCTION_PTRS
                    .u_material_editing_library_get_input_node_output_name_for_material_expression,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &material_expression,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::engine::UMaterialExpression>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &input_node,
                __buffer
                    .add(8)
                    .cast::<UPtr<crate::bindings::engine::UMaterialExpression>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                output_name,
                __buffer.add(16).cast::<FString>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::material_editor::UMaterialEditingLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::material_editor::__FUNCTION_PTRS
                    .u_material_editing_library_get_input_node_output_name_for_material_expression,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(16).cast::<FString>().swap(output_name);
        }
        unsafe { __buffer.add(32).cast::<bool>().read() }
    }
    pub fn get_child_instances(
        parent: UPtr<crate::bindings::engine::UMaterialInterface>,
        child_instances: &mut TArray<crate::bindings::core_u_object::FAssetData>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::material_editor::__FUNCTION_PTRS
                    .u_material_editing_library_get_child_instances,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &parent,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::engine::UMaterialInterface>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                child_instances,
                __buffer
                    .add(8)
                    .cast::<TArray<crate::bindings::core_u_object::FAssetData>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::material_editor::UMaterialEditingLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::material_editor::__FUNCTION_PTRS
                    .u_material_editing_library_get_child_instances,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(8)
                .cast::<TArray<crate::bindings::core_u_object::FAssetData>>()
                .swap(child_instances);
        }
    }
    pub fn duplicate_material_expression(
        material: UPtr<crate::bindings::engine::UMaterial>,
        material_function: UPtr<crate::bindings::engine::UMaterialFunction>,
        expression: UPtr<crate::bindings::engine::UMaterialExpression>,
    ) -> UPtr<crate::bindings::engine::UMaterialExpression> {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::material_editor::__FUNCTION_PTRS
                    .u_material_editing_library_duplicate_material_expression,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &material,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::UMaterial>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &material_function,
                __buffer
                    .add(8)
                    .cast::<UPtr<crate::bindings::engine::UMaterialFunction>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &expression,
                __buffer
                    .add(16)
                    .cast::<UPtr<crate::bindings::engine::UMaterialExpression>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::material_editor::UMaterialEditingLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::material_editor::__FUNCTION_PTRS
                    .u_material_editing_library_duplicate_material_expression,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(24)
                .cast::<UPtr<crate::bindings::engine::UMaterialExpression>>()
                .read()
        }
    }
    pub fn delete_material_expression_in_function(
        material_function: UPtr<crate::bindings::engine::UMaterialFunction>,
        expression: UPtr<crate::bindings::engine::UMaterialExpression>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::material_editor::__FUNCTION_PTRS
                    .u_material_editing_library_delete_material_expression_in_function,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &material_function,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::engine::UMaterialFunction>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &expression,
                __buffer
                    .add(8)
                    .cast::<UPtr<crate::bindings::engine::UMaterialExpression>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::material_editor::UMaterialEditingLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::material_editor::__FUNCTION_PTRS
                    .u_material_editing_library_delete_material_expression_in_function,
                __buffer,
            )
        };
    }
    pub fn delete_material_expression(
        material: UPtr<crate::bindings::engine::UMaterial>,
        expression: UPtr<crate::bindings::engine::UMaterialExpression>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::material_editor::__FUNCTION_PTRS
                    .u_material_editing_library_delete_material_expression,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &material,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::UMaterial>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &expression,
                __buffer
                    .add(8)
                    .cast::<UPtr<crate::bindings::engine::UMaterialExpression>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::material_editor::UMaterialEditingLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::material_editor::__FUNCTION_PTRS
                    .u_material_editing_library_delete_material_expression,
                __buffer,
            )
        };
    }
    pub fn delete_all_material_expressions_in_function(
        material_function: UPtr<crate::bindings::engine::UMaterialFunction>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::material_editor::__FUNCTION_PTRS
                    .u_material_editing_library_delete_all_material_expressions_in_function,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &material_function,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::engine::UMaterialFunction>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::material_editor::UMaterialEditingLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::material_editor::__FUNCTION_PTRS
                    .u_material_editing_library_delete_all_material_expressions_in_function,
                __buffer,
            )
        };
    }
    pub fn delete_all_material_expressions(
        material: UPtr<crate::bindings::engine::UMaterial>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::material_editor::__FUNCTION_PTRS
                    .u_material_editing_library_delete_all_material_expressions,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &material,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::UMaterial>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::material_editor::UMaterialEditingLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::material_editor::__FUNCTION_PTRS
                    .u_material_editing_library_delete_all_material_expressions,
                __buffer,
            )
        };
    }
    pub fn create_material_expression_in_function(
        material_function: UPtr<crate::bindings::engine::UMaterialFunction>,
        expression_class: TSubclassOf<crate::bindings::engine::UMaterialExpression>,
        node_pos_x: i32,
        node_pos_y: i32,
    ) -> UPtr<crate::bindings::engine::UMaterialExpression> {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::material_editor::__FUNCTION_PTRS
                    .u_material_editing_library_create_material_expression_in_function,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &material_function,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::engine::UMaterialFunction>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &expression_class,
                __buffer
                    .add(8)
                    .cast::<TSubclassOf<crate::bindings::engine::UMaterialExpression>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &node_pos_x,
                __buffer.add(16).cast::<i32>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &node_pos_y,
                __buffer.add(20).cast::<i32>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::material_editor::UMaterialEditingLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::material_editor::__FUNCTION_PTRS
                    .u_material_editing_library_create_material_expression_in_function,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(24)
                .cast::<UPtr<crate::bindings::engine::UMaterialExpression>>()
                .read()
        }
    }
    pub fn create_material_expression(
        material: UPtr<crate::bindings::engine::UMaterial>,
        expression_class: TSubclassOf<crate::bindings::engine::UMaterialExpression>,
        node_pos_x: i32,
        node_pos_y: i32,
    ) -> UPtr<crate::bindings::engine::UMaterialExpression> {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::material_editor::__FUNCTION_PTRS
                    .u_material_editing_library_create_material_expression,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &material,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::UMaterial>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &expression_class,
                __buffer
                    .add(8)
                    .cast::<TSubclassOf<crate::bindings::engine::UMaterialExpression>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &node_pos_x,
                __buffer.add(16).cast::<i32>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &node_pos_y,
                __buffer.add(20).cast::<i32>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::material_editor::UMaterialEditingLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::material_editor::__FUNCTION_PTRS
                    .u_material_editing_library_create_material_expression,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(24)
                .cast::<UPtr<crate::bindings::engine::UMaterialExpression>>()
                .read()
        }
    }
    pub fn connect_material_property(
        from_expression: UPtr<crate::bindings::engine::UMaterialExpression>,
        from_output_name: FString,
        property: crate::bindings::engine::EMaterialProperty,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<26>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::material_editor::__FUNCTION_PTRS
                    .u_material_editing_library_connect_material_property,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &from_expression,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::engine::UMaterialExpression>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &from_output_name,
                __buffer.add(8).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &property,
                __buffer.add(24).cast::<crate::bindings::engine::EMaterialProperty>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::material_editor::UMaterialEditingLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::material_editor::__FUNCTION_PTRS
                    .u_material_editing_library_connect_material_property,
                __buffer,
            )
        };
        unsafe { __buffer.add(25).cast::<bool>().read() }
    }
    pub fn connect_material_expressions(
        from_expression: UPtr<crate::bindings::engine::UMaterialExpression>,
        from_output_name: FString,
        to_expression: UPtr<crate::bindings::engine::UMaterialExpression>,
        to_input_name: FString,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<49>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::material_editor::__FUNCTION_PTRS
                    .u_material_editing_library_connect_material_expressions,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &from_expression,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::engine::UMaterialExpression>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &from_output_name,
                __buffer.add(8).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &to_expression,
                __buffer
                    .add(24)
                    .cast::<UPtr<crate::bindings::engine::UMaterialExpression>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &to_input_name,
                __buffer.add(32).cast::<FString>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::material_editor::UMaterialEditingLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::material_editor::__FUNCTION_PTRS
                    .u_material_editing_library_connect_material_expressions,
                __buffer,
            )
        };
        unsafe { __buffer.add(48).cast::<bool>().read() }
    }
    pub fn clear_all_material_instance_parameters(
        instance: UPtr<crate::bindings::engine::UMaterialInstanceConstant>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::material_editor::__FUNCTION_PTRS
                    .u_material_editing_library_clear_all_material_instance_parameters,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &instance,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::engine::UMaterialInstanceConstant>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::material_editor::UMaterialEditingLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::material_editor::__FUNCTION_PTRS
                    .u_material_editing_library_clear_all_material_instance_parameters,
                __buffer,
            )
        };
    }
}
#[repr(transparent)]
pub struct EBackgroundType(pub u8);
impl EBackgroundType {
    pub const SOLID_COLOR: EBackgroundType = EBackgroundType(0);
    pub const CHECKERED: EBackgroundType = EBackgroundType(1);
}
#[repr(transparent)]
pub struct EOfflineShaderCompiler(pub u8);
impl EOfflineShaderCompiler {
    pub const MALI: EOfflineShaderCompiler = EOfflineShaderCompiler(0);
    pub const ADRENO: EOfflineShaderCompiler = EOfflineShaderCompiler(1);
}
