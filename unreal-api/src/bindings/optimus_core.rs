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
    pub u_optimus_node_set_graph_position: *mut crate::ffi::UFunctionOpague,
    pub u_optimus_node_get_node_name: *mut crate::ffi::UFunctionOpague,
    pub u_optimus_node_get_node_category: *mut crate::ffi::UFunctionOpague,
    pub u_optimus_node_get_graph_position: *mut crate::ffi::UFunctionOpague,
    pub u_optimus_node_get_display_name: *mut crate::ffi::UFunctionOpague,
    pub u_optimus_deformer_get_variables: *mut crate::ffi::UFunctionOpague,
    pub u_optimus_deformer_get_resources: *mut crate::ffi::UFunctionOpague,
    pub u_optimus_deformer_get_primary_component_binding: *mut crate::ffi::UFunctionOpague,
    pub u_optimus_deformer_get_component_bindings: *mut crate::ffi::UFunctionOpague,
    pub u_optimus_deformer_instance_set_vector_variable: *mut crate::ffi::UFunctionOpague,
    pub u_optimus_deformer_instance_set_vector_array_variable: *mut crate::ffi::UFunctionOpague,
    pub u_optimus_deformer_instance_set_vector4_variable: *mut crate::ffi::UFunctionOpague,
    pub u_optimus_deformer_instance_set_vector4_array_variable: *mut crate::ffi::UFunctionOpague,
    pub u_optimus_deformer_instance_set_vector2_variable: *mut crate::ffi::UFunctionOpague,
    pub u_optimus_deformer_instance_set_vector2_array_variable: *mut crate::ffi::UFunctionOpague,
    pub u_optimus_deformer_instance_set_transform_variable: *mut crate::ffi::UFunctionOpague,
    pub u_optimus_deformer_instance_set_transform_array_variable: *mut crate::ffi::UFunctionOpague,
    pub u_optimus_deformer_instance_set_rotator_variable: *mut crate::ffi::UFunctionOpague,
    pub u_optimus_deformer_instance_set_rotator_array_variable: *mut crate::ffi::UFunctionOpague,
    pub u_optimus_deformer_instance_set_quat_variable: *mut crate::ffi::UFunctionOpague,
    pub u_optimus_deformer_instance_set_quat_array_variable: *mut crate::ffi::UFunctionOpague,
    pub u_optimus_deformer_instance_set_name_variable: *mut crate::ffi::UFunctionOpague,
    pub u_optimus_deformer_instance_set_name_array_variable: *mut crate::ffi::UFunctionOpague,
    pub u_optimus_deformer_instance_set_linear_color_variable: *mut crate::ffi::UFunctionOpague,
    pub u_optimus_deformer_instance_set_linear_color_array_variable: *mut crate::ffi::UFunctionOpague,
    pub u_optimus_deformer_instance_set_int_variable: *mut crate::ffi::UFunctionOpague,
    pub u_optimus_deformer_instance_set_int_array_variable: *mut crate::ffi::UFunctionOpague,
    pub u_optimus_deformer_instance_set_int4_variable: *mut crate::ffi::UFunctionOpague,
    pub u_optimus_deformer_instance_set_int4_array_variable: *mut crate::ffi::UFunctionOpague,
    pub u_optimus_deformer_instance_set_int3_variable: *mut crate::ffi::UFunctionOpague,
    pub u_optimus_deformer_instance_set_int3_array_variable: *mut crate::ffi::UFunctionOpague,
    pub u_optimus_deformer_instance_set_int2_variable: *mut crate::ffi::UFunctionOpague,
    pub u_optimus_deformer_instance_set_int2_array_variable: *mut crate::ffi::UFunctionOpague,
    pub u_optimus_deformer_instance_set_float_variable: *mut crate::ffi::UFunctionOpague,
    pub u_optimus_deformer_instance_set_float_array_variable: *mut crate::ffi::UFunctionOpague,
    pub u_optimus_deformer_instance_set_bool_variable: *mut crate::ffi::UFunctionOpague,
    pub u_optimus_deformer_instance_set_bool_array_variable: *mut crate::ffi::UFunctionOpague,
    pub u_optimus_deformer_instance_enqueue_trigger_graph: *mut crate::ffi::UFunctionOpague,
    pub u_optimus_node_graph_rename_graph_direct: *mut crate::ffi::UFunctionOpague,
    pub u_optimus_node_graph_rename_graph: *mut crate::ffi::UFunctionOpague,
    pub u_optimus_node_graph_remove_nodes: *mut crate::ffi::UFunctionOpague,
    pub u_optimus_node_graph_remove_node: *mut crate::ffi::UFunctionOpague,
    pub u_optimus_node_graph_remove_link: *mut crate::ffi::UFunctionOpague,
    pub u_optimus_node_graph_remove_all_links: *mut crate::ffi::UFunctionOpague,
    pub u_optimus_node_graph_move_graph_direct: *mut crate::ffi::UFunctionOpague,
    pub u_optimus_node_graph_is_sub_graph_reference: *mut crate::ffi::UFunctionOpague,
    pub u_optimus_node_graph_is_kernel_function: *mut crate::ffi::UFunctionOpague,
    pub u_optimus_node_graph_is_function_reference: *mut crate::ffi::UFunctionOpague,
    pub u_optimus_node_graph_is_function_graph: *mut crate::ffi::UFunctionOpague,
    pub u_optimus_node_graph_is_execution_graph: *mut crate::ffi::UFunctionOpague,
    pub u_optimus_node_graph_is_custom_kernel: *mut crate::ffi::UFunctionOpague,
    pub u_optimus_node_graph_get_graph_type: *mut crate::ffi::UFunctionOpague,
    pub u_optimus_node_graph_get_graphs: *mut crate::ffi::UFunctionOpague,
    pub u_optimus_node_graph_get_graph_index: *mut crate::ffi::UFunctionOpague,
    pub u_optimus_node_graph_expand_collapsed_nodes: *mut crate::ffi::UFunctionOpague,
    pub u_optimus_node_graph_duplicate_nodes: *mut crate::ffi::UFunctionOpague,
    pub u_optimus_node_graph_duplicate_node: *mut crate::ffi::UFunctionOpague,
    pub u_optimus_node_graph_convert_to_sub_graph: *mut crate::ffi::UFunctionOpague,
    pub u_optimus_node_graph_convert_to_function: *mut crate::ffi::UFunctionOpague,
    pub u_optimus_node_graph_convert_function_to_custom_kernel: *mut crate::ffi::UFunctionOpague,
    pub u_optimus_node_graph_convert_custom_kernel_to_function: *mut crate::ffi::UFunctionOpague,
    pub u_optimus_node_graph_collapse_nodes_to_sub_graph: *mut crate::ffi::UFunctionOpague,
    pub u_optimus_node_graph_collapse_nodes_to_function: *mut crate::ffi::UFunctionOpague,
    pub u_optimus_node_graph_add_variable_get_node: *mut crate::ffi::UFunctionOpague,
    pub u_optimus_node_graph_add_value_node: *mut crate::ffi::UFunctionOpague,
    pub u_optimus_node_graph_add_resource_set_node: *mut crate::ffi::UFunctionOpague,
    pub u_optimus_node_graph_add_resource_node: *mut crate::ffi::UFunctionOpague,
    pub u_optimus_node_graph_add_resource_get_node: *mut crate::ffi::UFunctionOpague,
    pub u_optimus_node_graph_add_node: *mut crate::ffi::UFunctionOpague,
    pub u_optimus_node_graph_add_loop_terminal_nodes: *mut crate::ffi::UFunctionOpague,
    pub u_optimus_node_graph_add_link: *mut crate::ffi::UFunctionOpague,
    pub u_optimus_node_graph_add_function_reference_node: *mut crate::ffi::UFunctionOpague,
    pub u_optimus_node_graph_add_data_interface_node: *mut crate::ffi::UFunctionOpague,
    pub u_optimus_node_graph_add_component_binding_get_node: *mut crate::ffi::UFunctionOpague,
    pub u_optimus_node_graph_add_comment_node: *mut crate::ffi::UFunctionOpague,
    pub u_optimus_function_node_graph_get_access_specifier_options: *mut crate::ffi::UFunctionOpague,
}
impl FunctionPtrs {
    pub const fn empty() -> Self {
        Self {
            u_optimus_node_set_graph_position: std::ptr::null_mut(),
            u_optimus_node_get_node_name: std::ptr::null_mut(),
            u_optimus_node_get_node_category: std::ptr::null_mut(),
            u_optimus_node_get_graph_position: std::ptr::null_mut(),
            u_optimus_node_get_display_name: std::ptr::null_mut(),
            u_optimus_deformer_get_variables: std::ptr::null_mut(),
            u_optimus_deformer_get_resources: std::ptr::null_mut(),
            u_optimus_deformer_get_primary_component_binding: std::ptr::null_mut(),
            u_optimus_deformer_get_component_bindings: std::ptr::null_mut(),
            u_optimus_deformer_instance_set_vector_variable: std::ptr::null_mut(),
            u_optimus_deformer_instance_set_vector_array_variable: std::ptr::null_mut(),
            u_optimus_deformer_instance_set_vector4_variable: std::ptr::null_mut(),
            u_optimus_deformer_instance_set_vector4_array_variable: std::ptr::null_mut(),
            u_optimus_deformer_instance_set_vector2_variable: std::ptr::null_mut(),
            u_optimus_deformer_instance_set_vector2_array_variable: std::ptr::null_mut(),
            u_optimus_deformer_instance_set_transform_variable: std::ptr::null_mut(),
            u_optimus_deformer_instance_set_transform_array_variable: std::ptr::null_mut(),
            u_optimus_deformer_instance_set_rotator_variable: std::ptr::null_mut(),
            u_optimus_deformer_instance_set_rotator_array_variable: std::ptr::null_mut(),
            u_optimus_deformer_instance_set_quat_variable: std::ptr::null_mut(),
            u_optimus_deformer_instance_set_quat_array_variable: std::ptr::null_mut(),
            u_optimus_deformer_instance_set_name_variable: std::ptr::null_mut(),
            u_optimus_deformer_instance_set_name_array_variable: std::ptr::null_mut(),
            u_optimus_deformer_instance_set_linear_color_variable: std::ptr::null_mut(),
            u_optimus_deformer_instance_set_linear_color_array_variable: std::ptr::null_mut(),
            u_optimus_deformer_instance_set_int_variable: std::ptr::null_mut(),
            u_optimus_deformer_instance_set_int_array_variable: std::ptr::null_mut(),
            u_optimus_deformer_instance_set_int4_variable: std::ptr::null_mut(),
            u_optimus_deformer_instance_set_int4_array_variable: std::ptr::null_mut(),
            u_optimus_deformer_instance_set_int3_variable: std::ptr::null_mut(),
            u_optimus_deformer_instance_set_int3_array_variable: std::ptr::null_mut(),
            u_optimus_deformer_instance_set_int2_variable: std::ptr::null_mut(),
            u_optimus_deformer_instance_set_int2_array_variable: std::ptr::null_mut(),
            u_optimus_deformer_instance_set_float_variable: std::ptr::null_mut(),
            u_optimus_deformer_instance_set_float_array_variable: std::ptr::null_mut(),
            u_optimus_deformer_instance_set_bool_variable: std::ptr::null_mut(),
            u_optimus_deformer_instance_set_bool_array_variable: std::ptr::null_mut(),
            u_optimus_deformer_instance_enqueue_trigger_graph: std::ptr::null_mut(),
            u_optimus_node_graph_rename_graph_direct: std::ptr::null_mut(),
            u_optimus_node_graph_rename_graph: std::ptr::null_mut(),
            u_optimus_node_graph_remove_nodes: std::ptr::null_mut(),
            u_optimus_node_graph_remove_node: std::ptr::null_mut(),
            u_optimus_node_graph_remove_link: std::ptr::null_mut(),
            u_optimus_node_graph_remove_all_links: std::ptr::null_mut(),
            u_optimus_node_graph_move_graph_direct: std::ptr::null_mut(),
            u_optimus_node_graph_is_sub_graph_reference: std::ptr::null_mut(),
            u_optimus_node_graph_is_kernel_function: std::ptr::null_mut(),
            u_optimus_node_graph_is_function_reference: std::ptr::null_mut(),
            u_optimus_node_graph_is_function_graph: std::ptr::null_mut(),
            u_optimus_node_graph_is_execution_graph: std::ptr::null_mut(),
            u_optimus_node_graph_is_custom_kernel: std::ptr::null_mut(),
            u_optimus_node_graph_get_graph_type: std::ptr::null_mut(),
            u_optimus_node_graph_get_graphs: std::ptr::null_mut(),
            u_optimus_node_graph_get_graph_index: std::ptr::null_mut(),
            u_optimus_node_graph_expand_collapsed_nodes: std::ptr::null_mut(),
            u_optimus_node_graph_duplicate_nodes: std::ptr::null_mut(),
            u_optimus_node_graph_duplicate_node: std::ptr::null_mut(),
            u_optimus_node_graph_convert_to_sub_graph: std::ptr::null_mut(),
            u_optimus_node_graph_convert_to_function: std::ptr::null_mut(),
            u_optimus_node_graph_convert_function_to_custom_kernel: std::ptr::null_mut(),
            u_optimus_node_graph_convert_custom_kernel_to_function: std::ptr::null_mut(),
            u_optimus_node_graph_collapse_nodes_to_sub_graph: std::ptr::null_mut(),
            u_optimus_node_graph_collapse_nodes_to_function: std::ptr::null_mut(),
            u_optimus_node_graph_add_variable_get_node: std::ptr::null_mut(),
            u_optimus_node_graph_add_value_node: std::ptr::null_mut(),
            u_optimus_node_graph_add_resource_set_node: std::ptr::null_mut(),
            u_optimus_node_graph_add_resource_node: std::ptr::null_mut(),
            u_optimus_node_graph_add_resource_get_node: std::ptr::null_mut(),
            u_optimus_node_graph_add_node: std::ptr::null_mut(),
            u_optimus_node_graph_add_loop_terminal_nodes: std::ptr::null_mut(),
            u_optimus_node_graph_add_link: std::ptr::null_mut(),
            u_optimus_node_graph_add_function_reference_node: std::ptr::null_mut(),
            u_optimus_node_graph_add_data_interface_node: std::ptr::null_mut(),
            u_optimus_node_graph_add_component_binding_get_node: std::ptr::null_mut(),
            u_optimus_node_graph_add_comment_node: std::ptr::null_mut(),
            u_optimus_function_node_graph_get_access_specifier_options: std::ptr::null_mut(),
        }
    }
}
pub fn initialize() {
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UOptimusNode::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetGraphPosition"),
            &raw mut __FUNCTION_PTRS.u_optimus_node_set_graph_position,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetNodeName"),
            &raw mut __FUNCTION_PTRS.u_optimus_node_get_node_name,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetNodeCategory"),
            &raw mut __FUNCTION_PTRS.u_optimus_node_get_node_category,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetGraphPosition"),
            &raw mut __FUNCTION_PTRS.u_optimus_node_get_graph_position,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetDisplayName"),
            &raw mut __FUNCTION_PTRS.u_optimus_node_get_display_name,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UOptimusDeformer::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetVariables"),
            &raw mut __FUNCTION_PTRS.u_optimus_deformer_get_variables,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetResources"),
            &raw mut __FUNCTION_PTRS.u_optimus_deformer_get_resources,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetPrimaryComponentBinding"),
            &raw mut __FUNCTION_PTRS.u_optimus_deformer_get_primary_component_binding,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetComponentBindings"),
            &raw mut __FUNCTION_PTRS.u_optimus_deformer_get_component_bindings,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UOptimusDeformerInstance::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetVectorVariable"),
            &raw mut __FUNCTION_PTRS.u_optimus_deformer_instance_set_vector_variable,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetVectorArrayVariable"),
            &raw mut __FUNCTION_PTRS
                .u_optimus_deformer_instance_set_vector_array_variable,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetVector4Variable"),
            &raw mut __FUNCTION_PTRS.u_optimus_deformer_instance_set_vector4_variable,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetVector4ArrayVariable"),
            &raw mut __FUNCTION_PTRS
                .u_optimus_deformer_instance_set_vector4_array_variable,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetVector2Variable"),
            &raw mut __FUNCTION_PTRS.u_optimus_deformer_instance_set_vector2_variable,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetVector2ArrayVariable"),
            &raw mut __FUNCTION_PTRS
                .u_optimus_deformer_instance_set_vector2_array_variable,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetTransformVariable"),
            &raw mut __FUNCTION_PTRS.u_optimus_deformer_instance_set_transform_variable,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetTransformArrayVariable"),
            &raw mut __FUNCTION_PTRS
                .u_optimus_deformer_instance_set_transform_array_variable,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetRotatorVariable"),
            &raw mut __FUNCTION_PTRS.u_optimus_deformer_instance_set_rotator_variable,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetRotatorArrayVariable"),
            &raw mut __FUNCTION_PTRS
                .u_optimus_deformer_instance_set_rotator_array_variable,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetQuatVariable"),
            &raw mut __FUNCTION_PTRS.u_optimus_deformer_instance_set_quat_variable,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetQuatArrayVariable"),
            &raw mut __FUNCTION_PTRS.u_optimus_deformer_instance_set_quat_array_variable,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetNameVariable"),
            &raw mut __FUNCTION_PTRS.u_optimus_deformer_instance_set_name_variable,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetNameArrayVariable"),
            &raw mut __FUNCTION_PTRS.u_optimus_deformer_instance_set_name_array_variable,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetLinearColorVariable"),
            &raw mut __FUNCTION_PTRS
                .u_optimus_deformer_instance_set_linear_color_variable,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetLinearColorArrayVariable"),
            &raw mut __FUNCTION_PTRS
                .u_optimus_deformer_instance_set_linear_color_array_variable,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetIntVariable"),
            &raw mut __FUNCTION_PTRS.u_optimus_deformer_instance_set_int_variable,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetIntArrayVariable"),
            &raw mut __FUNCTION_PTRS.u_optimus_deformer_instance_set_int_array_variable,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetInt4Variable"),
            &raw mut __FUNCTION_PTRS.u_optimus_deformer_instance_set_int4_variable,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetInt4ArrayVariable"),
            &raw mut __FUNCTION_PTRS.u_optimus_deformer_instance_set_int4_array_variable,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetInt3Variable"),
            &raw mut __FUNCTION_PTRS.u_optimus_deformer_instance_set_int3_variable,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetInt3ArrayVariable"),
            &raw mut __FUNCTION_PTRS.u_optimus_deformer_instance_set_int3_array_variable,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetInt2Variable"),
            &raw mut __FUNCTION_PTRS.u_optimus_deformer_instance_set_int2_variable,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetInt2ArrayVariable"),
            &raw mut __FUNCTION_PTRS.u_optimus_deformer_instance_set_int2_array_variable,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetFloatVariable"),
            &raw mut __FUNCTION_PTRS.u_optimus_deformer_instance_set_float_variable,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetFloatArrayVariable"),
            &raw mut __FUNCTION_PTRS.u_optimus_deformer_instance_set_float_array_variable,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetBoolVariable"),
            &raw mut __FUNCTION_PTRS.u_optimus_deformer_instance_set_bool_variable,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetBoolArrayVariable"),
            &raw mut __FUNCTION_PTRS.u_optimus_deformer_instance_set_bool_array_variable,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("EnqueueTriggerGraph"),
            &raw mut __FUNCTION_PTRS.u_optimus_deformer_instance_enqueue_trigger_graph,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UOptimusNodeGraph::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RenameGraphDirect"),
            &raw mut __FUNCTION_PTRS.u_optimus_node_graph_rename_graph_direct,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RenameGraph"),
            &raw mut __FUNCTION_PTRS.u_optimus_node_graph_rename_graph,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RemoveNodes"),
            &raw mut __FUNCTION_PTRS.u_optimus_node_graph_remove_nodes,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RemoveNode"),
            &raw mut __FUNCTION_PTRS.u_optimus_node_graph_remove_node,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RemoveLink"),
            &raw mut __FUNCTION_PTRS.u_optimus_node_graph_remove_link,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RemoveAllLinks"),
            &raw mut __FUNCTION_PTRS.u_optimus_node_graph_remove_all_links,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("MoveGraphDirect"),
            &raw mut __FUNCTION_PTRS.u_optimus_node_graph_move_graph_direct,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("IsSubGraphReference"),
            &raw mut __FUNCTION_PTRS.u_optimus_node_graph_is_sub_graph_reference,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("IsKernelFunction"),
            &raw mut __FUNCTION_PTRS.u_optimus_node_graph_is_kernel_function,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("IsFunctionReference"),
            &raw mut __FUNCTION_PTRS.u_optimus_node_graph_is_function_reference,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("IsFunctionGraph"),
            &raw mut __FUNCTION_PTRS.u_optimus_node_graph_is_function_graph,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("IsExecutionGraph"),
            &raw mut __FUNCTION_PTRS.u_optimus_node_graph_is_execution_graph,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("IsCustomKernel"),
            &raw mut __FUNCTION_PTRS.u_optimus_node_graph_is_custom_kernel,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetGraphType"),
            &raw mut __FUNCTION_PTRS.u_optimus_node_graph_get_graph_type,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetGraphs"),
            &raw mut __FUNCTION_PTRS.u_optimus_node_graph_get_graphs,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetGraphIndex"),
            &raw mut __FUNCTION_PTRS.u_optimus_node_graph_get_graph_index,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ExpandCollapsedNodes"),
            &raw mut __FUNCTION_PTRS.u_optimus_node_graph_expand_collapsed_nodes,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("DuplicateNodes"),
            &raw mut __FUNCTION_PTRS.u_optimus_node_graph_duplicate_nodes,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("DuplicateNode"),
            &raw mut __FUNCTION_PTRS.u_optimus_node_graph_duplicate_node,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ConvertToSubGraph"),
            &raw mut __FUNCTION_PTRS.u_optimus_node_graph_convert_to_sub_graph,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ConvertToFunction"),
            &raw mut __FUNCTION_PTRS.u_optimus_node_graph_convert_to_function,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ConvertFunctionToCustomKernel"),
            &raw mut __FUNCTION_PTRS
                .u_optimus_node_graph_convert_function_to_custom_kernel,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ConvertCustomKernelToFunction"),
            &raw mut __FUNCTION_PTRS
                .u_optimus_node_graph_convert_custom_kernel_to_function,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CollapseNodesToSubGraph"),
            &raw mut __FUNCTION_PTRS.u_optimus_node_graph_collapse_nodes_to_sub_graph,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CollapseNodesToFunction"),
            &raw mut __FUNCTION_PTRS.u_optimus_node_graph_collapse_nodes_to_function,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AddVariableGetNode"),
            &raw mut __FUNCTION_PTRS.u_optimus_node_graph_add_variable_get_node,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AddValueNode"),
            &raw mut __FUNCTION_PTRS.u_optimus_node_graph_add_value_node,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AddResourceSetNode"),
            &raw mut __FUNCTION_PTRS.u_optimus_node_graph_add_resource_set_node,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AddResourceNode"),
            &raw mut __FUNCTION_PTRS.u_optimus_node_graph_add_resource_node,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AddResourceGetNode"),
            &raw mut __FUNCTION_PTRS.u_optimus_node_graph_add_resource_get_node,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AddNode"),
            &raw mut __FUNCTION_PTRS.u_optimus_node_graph_add_node,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AddLoopTerminalNodes"),
            &raw mut __FUNCTION_PTRS.u_optimus_node_graph_add_loop_terminal_nodes,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AddLink"),
            &raw mut __FUNCTION_PTRS.u_optimus_node_graph_add_link,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AddFunctionReferenceNode"),
            &raw mut __FUNCTION_PTRS.u_optimus_node_graph_add_function_reference_node,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AddDataInterfaceNode"),
            &raw mut __FUNCTION_PTRS.u_optimus_node_graph_add_data_interface_node,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AddComponentBindingGetNode"),
            &raw mut __FUNCTION_PTRS.u_optimus_node_graph_add_component_binding_get_node,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AddCommentNode"),
            &raw mut __FUNCTION_PTRS.u_optimus_node_graph_add_comment_node,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UOptimusFunctionNodeGraph::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetAccessSpecifierOptions"),
            &raw mut __FUNCTION_PTRS
                .u_optimus_function_node_graph_get_access_specifier_options,
        );
    }
}
#[repr(C, align(8))]
pub struct FOptimusDataTypeRef {
    pub(crate) __padding_end: [u8; 64],
}
impl FOptimusDataTypeRef {}
#[repr(C, align(8))]
pub struct FRigVMTrait_OptimusDeformerSettings {
    #[doc(hidden)]
    pub(crate) __padding_24: [u8; 24],
    pub execution_phase: EOptimusDeformerExecutionPhase,
    pub execution_group: i32,
    pub deform_child_components: bool,
    pub exclude_child_components_with_tag: FName,
}
impl FRigVMTrait_OptimusDeformerSettings {}
#[repr(C, align(8))]
pub struct FRigUnit_AddOptimusDeformer {
    pub(crate) __padding_end: [u8; 32],
}
impl FRigUnit_AddOptimusDeformer {}
#[repr(C, align(8))]
pub struct FRigVMTrait_SetDeformerIntVariable {
    pub(crate) __padding_end: [u8; 32],
}
impl FRigVMTrait_SetDeformerIntVariable {}
#[repr(C, align(8))]
pub struct FRigVMTrait_SetDeformerIntArrayVariable {
    pub(crate) __padding_end: [u8; 40],
}
impl FRigVMTrait_SetDeformerIntArrayVariable {}
#[repr(C, align(8))]
pub struct FRigVMTrait_SetDeformerInt2Variable {
    pub(crate) __padding_end: [u8; 32],
}
impl FRigVMTrait_SetDeformerInt2Variable {}
#[repr(C, align(8))]
pub struct FRigVMTrait_SetDeformerInt2ArrayVariable {
    pub(crate) __padding_end: [u8; 40],
}
impl FRigVMTrait_SetDeformerInt2ArrayVariable {}
#[repr(C, align(8))]
pub struct FRigVMTrait_SetDeformerInt3Variable {
    pub(crate) __padding_end: [u8; 40],
}
impl FRigVMTrait_SetDeformerInt3Variable {}
#[repr(C, align(8))]
pub struct FRigVMTrait_SetDeformerInt3ArrayVariable {
    pub(crate) __padding_end: [u8; 40],
}
impl FRigVMTrait_SetDeformerInt3ArrayVariable {}
#[repr(C, align(8))]
pub struct FRigVMTrait_SetDeformerInt4Variable {
    pub(crate) __padding_end: [u8; 40],
}
impl FRigVMTrait_SetDeformerInt4Variable {}
#[repr(C, align(8))]
pub struct FRigVMTrait_SetDeformerInt4ArrayVariable {
    pub(crate) __padding_end: [u8; 40],
}
impl FRigVMTrait_SetDeformerInt4ArrayVariable {}
#[repr(C, align(8))]
pub struct FRigVMTrait_SetDeformerFloatVariable {
    pub(crate) __padding_end: [u8; 32],
}
impl FRigVMTrait_SetDeformerFloatVariable {}
#[repr(C, align(8))]
pub struct FRigVMTrait_SetDeformerFloatArrayVariable {
    pub(crate) __padding_end: [u8; 40],
}
impl FRigVMTrait_SetDeformerFloatArrayVariable {}
#[repr(C, align(8))]
pub struct FRigVMTrait_SetDeformerVector2Variable {
    pub(crate) __padding_end: [u8; 40],
}
impl FRigVMTrait_SetDeformerVector2Variable {}
#[repr(C, align(8))]
pub struct FRigVMTrait_SetDeformerVector2ArrayVariable {
    pub(crate) __padding_end: [u8; 40],
}
impl FRigVMTrait_SetDeformerVector2ArrayVariable {}
#[repr(C, align(8))]
pub struct FRigVMTrait_SetDeformerVectorVariable {
    pub(crate) __padding_end: [u8; 48],
}
impl FRigVMTrait_SetDeformerVectorVariable {}
#[repr(C, align(8))]
pub struct FRigVMTrait_SetDeformerVectorArrayVariable {
    pub(crate) __padding_end: [u8; 40],
}
impl FRigVMTrait_SetDeformerVectorArrayVariable {}
#[repr(C, align(16))]
pub struct FRigVMTrait_SetDeformerVector4Variable {
    pub(crate) __padding_end: [u8; 64],
}
impl FRigVMTrait_SetDeformerVector4Variable {}
#[repr(C, align(8))]
pub struct FRigVMTrait_SetDeformerVector4ArrayVariable {
    pub(crate) __padding_end: [u8; 40],
}
impl FRigVMTrait_SetDeformerVector4ArrayVariable {}
#[repr(C, align(8))]
pub struct FRigVMTrait_SetDeformerLinearColorVariable {
    pub(crate) __padding_end: [u8; 40],
}
impl FRigVMTrait_SetDeformerLinearColorVariable {}
#[repr(C, align(8))]
pub struct FRigVMTrait_SetDeformerLinearColorArrayVariable {
    pub(crate) __padding_end: [u8; 40],
}
impl FRigVMTrait_SetDeformerLinearColorArrayVariable {}
#[repr(C, align(16))]
pub struct FRigVMTrait_SetDeformerQuatVariable {
    pub(crate) __padding_end: [u8; 64],
}
impl FRigVMTrait_SetDeformerQuatVariable {}
#[repr(C, align(8))]
pub struct FRigVMTrait_SetDeformerQuatArrayVariable {
    pub(crate) __padding_end: [u8; 40],
}
impl FRigVMTrait_SetDeformerQuatArrayVariable {}
#[repr(C, align(8))]
pub struct FRigVMTrait_SetDeformerRotatorVariable {
    pub(crate) __padding_end: [u8; 48],
}
impl FRigVMTrait_SetDeformerRotatorVariable {}
#[repr(C, align(8))]
pub struct FRigVMTrait_SetDeformerRotatorArrayVariable {
    pub(crate) __padding_end: [u8; 40],
}
impl FRigVMTrait_SetDeformerRotatorArrayVariable {}
#[repr(C, align(16))]
pub struct FRigVMTrait_SetDeformerTransformVariable {
    pub(crate) __padding_end: [u8; 128],
}
impl FRigVMTrait_SetDeformerTransformVariable {}
#[repr(C, align(8))]
pub struct FRigVMTrait_SetDeformerTransformArrayVariable {
    pub(crate) __padding_end: [u8; 40],
}
impl FRigVMTrait_SetDeformerTransformArrayVariable {}
#[repr(C, align(8))]
pub struct FRigVMTrait_SetDeformerNameVariable {
    pub(crate) __padding_end: [u8; 40],
}
impl FRigVMTrait_SetDeformerNameVariable {}
#[repr(C, align(8))]
pub struct FRigVMTrait_SetDeformerNameArrayVariable {
    pub(crate) __padding_end: [u8; 40],
}
impl FRigVMTrait_SetDeformerNameArrayVariable {}
#[repr(C, align(8))]
pub struct FRigVMTrait_SetDeformerBoolVariable {
    pub(crate) __padding_end: [u8; 32],
}
impl FRigVMTrait_SetDeformerBoolVariable {}
#[repr(C, align(8))]
pub struct FRigVMTrait_SetDeformerBoolArrayVariable {
    pub(crate) __padding_end: [u8; 40],
}
impl FRigVMTrait_SetDeformerBoolArrayVariable {}
#[repr(C, align(4))]
pub struct FOptimusDeformerInstanceComponentBinding {
    pub(crate) __padding_end: [u8; 24],
}
impl FOptimusDeformerInstanceComponentBinding {}
pub struct IOptimusAlternativeSelectedObjectProvider {}
#[repr(C, align(8))]
pub struct UOptimusAlternativeSelectedObjectProvider {
    __padding_end: [u8; 48],
}
impl UOptimusAlternativeSelectedObjectProvider {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UOptimusAlternativeSelectedObjectProvider")
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
pub struct IOptimusComponentBindingProvider {}
#[repr(C, align(8))]
pub struct UOptimusComponentBindingProvider {
    __padding_end: [u8; 48],
}
impl UOptimusComponentBindingProvider {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UOptimusComponentBindingProvider")
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
pub struct IOptimusComponentBindingReceiver {}
#[repr(C, align(8))]
pub struct UOptimusComponentBindingReceiver {
    __padding_end: [u8; 48],
}
impl UOptimusComponentBindingReceiver {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UOptimusComponentBindingReceiver")
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
pub struct IOptimusComputeKernelDataInterface {}
#[repr(C, align(8))]
pub struct UOptimusComputeKernelDataInterface {
    __padding_end: [u8; 48],
}
impl UOptimusComputeKernelDataInterface {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UOptimusComputeKernelDataInterface")
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
pub struct IOptimusComputeKernelProvider {}
#[repr(C, align(8))]
pub struct UOptimusComputeKernelProvider {
    __padding_end: [u8; 48],
}
impl UOptimusComputeKernelProvider {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UOptimusComputeKernelProvider")
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
pub struct IOptimusDataInterfaceProvider {}
#[repr(C, align(8))]
pub struct UOptimusDataInterfaceProvider {
    __padding_end: [u8; 48],
}
impl UOptimusDataInterfaceProvider {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UOptimusDataInterfaceProvider")
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
pub struct IOptimusDeformerAssetPathAccessor {}
#[repr(C, align(8))]
pub struct UOptimusDeformerAssetPathAccessor {
    __padding_end: [u8; 48],
}
impl UOptimusDeformerAssetPathAccessor {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UOptimusDeformerAssetPathAccessor")
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
pub struct IOptimusDeformerGeometryReadbackProvider {}
#[repr(C, align(8))]
pub struct UOptimusDeformerGeometryReadbackProvider {
    __padding_end: [u8; 48],
}
impl UOptimusDeformerGeometryReadbackProvider {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UOptimusDeformerGeometryReadbackProvider")
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
pub struct IOptimusDeformerInstanceAccessor {}
#[repr(C, align(8))]
pub struct UOptimusDeformerInstanceAccessor {
    __padding_end: [u8; 48],
}
impl UOptimusDeformerInstanceAccessor {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UOptimusDeformerInstanceAccessor")
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
pub struct IOptimusDeprecatedExecutionDataInterface {}
#[repr(C, align(8))]
pub struct UOptimusDeprecatedExecutionDataInterface {
    __padding_end: [u8; 48],
}
impl UOptimusDeprecatedExecutionDataInterface {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UOptimusDeprecatedExecutionDataInterface")
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
pub struct IOptimusExecutionDomainProvider {}
#[repr(C, align(8))]
pub struct UOptimusExecutionDomainProvider {
    __padding_end: [u8; 48],
}
impl UOptimusExecutionDomainProvider {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UOptimusExecutionDomainProvider")
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
pub struct IOptimusGeneratedClassDefiner {}
#[repr(C, align(8))]
pub struct UOptimusGeneratedClassDefiner {
    __padding_end: [u8; 48],
}
impl UOptimusGeneratedClassDefiner {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UOptimusGeneratedClassDefiner")
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
pub struct IOptimusNodeAdderPinProvider {}
#[repr(C, align(8))]
pub struct UOptimusNodeAdderPinProvider {
    __padding_end: [u8; 48],
}
impl UOptimusNodeAdderPinProvider {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UOptimusNodeAdderPinProvider")
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
pub struct IOptimusNodeFunctionLibraryOwner {}
#[repr(C, align(8))]
pub struct UOptimusNodeFunctionLibraryOwner {
    __padding_end: [u8; 48],
}
impl UOptimusNodeFunctionLibraryOwner {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UOptimusNodeFunctionLibraryOwner")
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
pub struct IOptimusNodeGraphCollectionOwner {}
#[repr(C, align(8))]
pub struct UOptimusNodeGraphCollectionOwner {
    __padding_end: [u8; 48],
}
impl UOptimusNodeGraphCollectionOwner {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UOptimusNodeGraphCollectionOwner")
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
pub struct IOptimusNodeGraphProvider {}
#[repr(C, align(8))]
pub struct UOptimusNodeGraphProvider {
    __padding_end: [u8; 48],
}
impl UOptimusNodeGraphProvider {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UOptimusNodeGraphProvider")
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
pub struct IOptimusNodePairProvider {}
#[repr(C, align(8))]
pub struct UOptimusNodePairProvider {
    __padding_end: [u8; 48],
}
impl UOptimusNodePairProvider {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UOptimusNodePairProvider")
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
pub struct IOptimusNodePinRouter {}
#[repr(C, align(8))]
pub struct UOptimusNodePinRouter {
    __padding_end: [u8; 48],
}
impl UOptimusNodePinRouter {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UOptimusNodePinRouter")
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
pub struct IOptimusNodeSubGraphReferencer {}
#[repr(C, align(8))]
pub struct UOptimusNodeSubGraphReferencer {
    __padding_end: [u8; 48],
}
impl UOptimusNodeSubGraphReferencer {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UOptimusNodeSubGraphReferencer")
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
pub struct IOptimusNonCollapsibleNode {}
#[repr(C, align(8))]
pub struct UOptimusNonCollapsibleNode {
    __padding_end: [u8; 48],
}
impl UOptimusNonCollapsibleNode {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UOptimusNonCollapsibleNode")
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
pub struct IOptimusNonCopyableNode {}
#[repr(C, align(8))]
pub struct UOptimusNonCopyableNode {
    __padding_end: [u8; 48],
}
impl UOptimusNonCopyableNode {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UOptimusNonCopyableNode")
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
pub struct IOptimusOutputBufferWriter {}
#[repr(C, align(8))]
pub struct UOptimusOutputBufferWriter {
    __padding_end: [u8; 48],
}
impl UOptimusOutputBufferWriter {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UOptimusOutputBufferWriter")
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
pub struct IOptimusParameterBindingProvider {}
#[repr(C, align(8))]
pub struct UOptimusParameterBindingProvider {
    __padding_end: [u8; 48],
}
impl UOptimusParameterBindingProvider {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UOptimusParameterBindingProvider")
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
pub struct IOptimusPathResolver {}
#[repr(C, align(8))]
pub struct UOptimusPathResolver {
    __padding_end: [u8; 48],
}
impl UOptimusPathResolver {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UOptimusPathResolver")
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
pub struct IOptimusPersistentBufferProvider {}
#[repr(C, align(8))]
pub struct UOptimusPersistentBufferProvider {
    __padding_end: [u8; 48],
}
impl UOptimusPersistentBufferProvider {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UOptimusPersistentBufferProvider")
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
pub struct IOptimusPinMutabilityDefiner {}
#[repr(C, align(8))]
pub struct UOptimusPinMutabilityDefiner {
    __padding_end: [u8; 48],
}
impl UOptimusPinMutabilityDefiner {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UOptimusPinMutabilityDefiner")
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
pub struct IOptimusPropertyPinProvider {}
#[repr(C, align(8))]
pub struct UOptimusPropertyPinProvider {
    __padding_end: [u8; 48],
}
impl UOptimusPropertyPinProvider {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UOptimusPropertyPinProvider")
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
pub struct IOptimusShaderTextProvider {}
#[repr(C, align(8))]
pub struct UOptimusShaderTextProvider {
    __padding_end: [u8; 48],
}
impl UOptimusShaderTextProvider {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UOptimusShaderTextProvider")
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
pub struct IOptimusUnnamedNodePinProvider {}
#[repr(C, align(8))]
pub struct UOptimusUnnamedNodePinProvider {
    __padding_end: [u8; 48],
}
impl UOptimusUnnamedNodePinProvider {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UOptimusUnnamedNodePinProvider")
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
pub struct IOptimusValueProvider {}
#[repr(C, align(8))]
pub struct UOptimusValueProvider {
    __padding_end: [u8; 48],
}
impl UOptimusValueProvider {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UOptimusValueProvider")
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
pub struct UOptimusKernelSource {
    __padding_end: [u8; 176],
}
impl UOptimusKernelSource {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UOptimusKernelSource")
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
pub struct UOptimusComponentSource {
    __padding_end: [u8; 48],
}
impl UOptimusComponentSource {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UOptimusComponentSource")
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
pub struct UOptimusSceneComponentSource {
    __padding_end: [u8; 48],
}
impl UOptimusSceneComponentSource {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UOptimusSceneComponentSource")
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
pub struct UOptimusSkinnedMeshComponentSource {
    __padding_end: [u8; 48],
}
impl UOptimusSkinnedMeshComponentSource {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UOptimusSkinnedMeshComponentSource")
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
pub struct UOptimusSkeletalMeshComponentSource {
    __padding_end: [u8; 48],
}
impl UOptimusSkeletalMeshComponentSource {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UOptimusSkeletalMeshComponentSource")
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
pub struct UOptimusComputeDataInterface {
    __padding_end: [u8; 48],
}
impl UOptimusComputeDataInterface {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UOptimusComputeDataInterface")
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
pub struct UOptimusAdvancedSkeletonDataInterface {
    __padding_end: [u8; 152],
}
impl UOptimusAdvancedSkeletonDataInterface {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UOptimusAdvancedSkeletonDataInterface")
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
pub struct UOptimusAdvancedSkeletonDataProvider {
    __padding_end: [u8; 232],
}
impl UOptimusAdvancedSkeletonDataProvider {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UOptimusAdvancedSkeletonDataProvider")
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
pub struct UOptimusAnimAttributeDataInterface {
    __padding_end: [u8; 112],
}
impl UOptimusAnimAttributeDataInterface {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UOptimusAnimAttributeDataInterface")
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
pub struct UOptimusAnimAttributeDataProvider {
    __padding_end: [u8; 80],
}
impl UOptimusAnimAttributeDataProvider {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UOptimusAnimAttributeDataProvider")
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
pub struct UOptimusClothDataInterface {
    __padding_end: [u8; 48],
}
impl UOptimusClothDataInterface {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UOptimusClothDataInterface")
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
pub struct UOptimusClothDataProvider {
    __padding_end: [u8; 56],
}
impl UOptimusClothDataProvider {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UOptimusClothDataProvider")
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
pub struct UOptimusConnectivityDataInterface {
    __padding_end: [u8; 48],
}
impl UOptimusConnectivityDataInterface {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UOptimusConnectivityDataInterface")
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
pub struct UOptimusConnectivityDataProvider {
    __padding_end: [u8; 72],
}
impl UOptimusConnectivityDataProvider {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UOptimusConnectivityDataProvider")
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
pub struct UOptimusCopyKernelDataInterface {
    __padding_end: [u8; 80],
}
impl UOptimusCopyKernelDataInterface {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UOptimusCopyKernelDataInterface")
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
pub struct UOptimusCopyKernelDataProvider {
    __padding_end: [u8; 216],
}
impl UOptimusCopyKernelDataProvider {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UOptimusCopyKernelDataProvider")
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
pub struct UOptimusCustomComputeKernelDataInterface {
    __padding_end: [u8; 120],
}
impl UOptimusCustomComputeKernelDataInterface {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UOptimusCustomComputeKernelDataInterface")
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
pub struct UOptimusCustomComputeKernelDataProvider {
    __padding_end: [u8; 216],
}
impl UOptimusCustomComputeKernelDataProvider {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UOptimusCustomComputeKernelDataProvider")
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
pub struct UOptimusDebugDrawDataInterface {
    __padding_end: [u8; 72],
}
impl UOptimusDebugDrawDataInterface {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UOptimusDebugDrawDataInterface")
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
pub struct UOptimusDebugDrawDataProvider {
    __padding_end: [u8; 80],
}
impl UOptimusDebugDrawDataProvider {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UOptimusDebugDrawDataProvider")
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
pub struct UOptimusDuplicateVerticesDataInterface {
    __padding_end: [u8; 48],
}
impl UOptimusDuplicateVerticesDataInterface {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UOptimusDuplicateVerticesDataInterface")
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
pub struct UOptimusDuplicateVerticesDataProvider {
    __padding_end: [u8; 56],
}
impl UOptimusDuplicateVerticesDataProvider {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UOptimusDuplicateVerticesDataProvider")
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
pub struct UOptimusGraphDataInterface {
    __padding_end: [u8; 72],
}
impl UOptimusGraphDataInterface {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UOptimusGraphDataInterface")
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
pub struct UOptimusGraphDataProvider {
    __padding_end: [u8; 104],
}
impl UOptimusGraphDataProvider {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UOptimusGraphDataProvider")
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
pub struct UOptimusHalfEdgeDataInterface {
    __padding_end: [u8; 48],
}
impl UOptimusHalfEdgeDataInterface {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UOptimusHalfEdgeDataInterface")
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
pub struct UOptimusHalfEdgeDataProvider {
    __padding_end: [u8; 192],
}
impl UOptimusHalfEdgeDataProvider {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UOptimusHalfEdgeDataProvider")
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
pub struct UOptimusLoopTerminalDataInterface {
    __padding_end: [u8; 56],
}
impl UOptimusLoopTerminalDataInterface {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UOptimusLoopTerminalDataInterface")
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
pub struct UOptimusLoopTerminalDataProvider {
    __padding_end: [u8; 56],
}
impl UOptimusLoopTerminalDataProvider {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UOptimusLoopTerminalDataProvider")
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
pub struct UOptimusMorphTargetDataInterface {
    __padding_end: [u8; 48],
}
impl UOptimusMorphTargetDataInterface {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UOptimusMorphTargetDataInterface")
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
pub struct UOptimusMorphTargetDataProvider {
    __padding_end: [u8; 56],
}
impl UOptimusMorphTargetDataProvider {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UOptimusMorphTargetDataProvider")
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
pub struct UOptimusRawBufferDataInterface {
    __padding_end: [u8; 168],
}
impl UOptimusRawBufferDataInterface {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UOptimusRawBufferDataInterface")
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
pub struct UOptimusTransientBufferDataInterface {
    __padding_end: [u8; 176],
}
impl UOptimusTransientBufferDataInterface {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UOptimusTransientBufferDataInterface")
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
pub struct UOptimusImplicitPersistentBufferDataInterface {
    __padding_end: [u8; 176],
}
impl UOptimusImplicitPersistentBufferDataInterface {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UOptimusImplicitPersistentBufferDataInterface")
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
pub struct UOptimusPersistentBufferDataInterface {
    __padding_end: [u8; 184],
}
impl UOptimusPersistentBufferDataInterface {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UOptimusPersistentBufferDataInterface")
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
pub struct UOptimusRawBufferDataProvider {
    __padding_end: [u8; 224],
}
impl UOptimusRawBufferDataProvider {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UOptimusRawBufferDataProvider")
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
pub struct UOptimusTransientBufferDataProvider {
    __padding_end: [u8; 232],
}
impl UOptimusTransientBufferDataProvider {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UOptimusTransientBufferDataProvider")
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
pub struct UOptimusImplicitPersistentBufferDataProvider {
    __padding_end: [u8; 264],
}
impl UOptimusImplicitPersistentBufferDataProvider {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UOptimusImplicitPersistentBufferDataProvider")
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
pub struct UOptimusPersistentBufferDataProvider {
    __padding_end: [u8; 264],
}
impl UOptimusPersistentBufferDataProvider {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UOptimusPersistentBufferDataProvider")
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
pub struct UOptimusSceneDataInterface {
    __padding_end: [u8; 48],
}
impl UOptimusSceneDataInterface {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UOptimusSceneDataInterface")
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
pub struct UOptimusSceneDataProvider {
    __padding_end: [u8; 56],
}
impl UOptimusSceneDataProvider {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UOptimusSceneDataProvider")
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
pub struct UOptimusSkeletonDataInterface {
    __padding_end: [u8; 48],
}
impl UOptimusSkeletonDataInterface {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UOptimusSkeletonDataInterface")
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
pub struct UOptimusSkeletonDataProvider {
    __padding_end: [u8; 56],
}
impl UOptimusSkeletonDataProvider {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UOptimusSkeletonDataProvider")
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
pub struct UOptimusSkinnedMeshDataInterface {
    __padding_end: [u8; 48],
}
impl UOptimusSkinnedMeshDataInterface {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UOptimusSkinnedMeshDataInterface")
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
pub struct UOptimusSkinnedMeshDataProvider {
    __padding_end: [u8; 56],
}
impl UOptimusSkinnedMeshDataProvider {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UOptimusSkinnedMeshDataProvider")
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
pub struct UDEPRECATED_OptimusSkinnedMeshExecDataInterface {
    __padding_end: [u8; 64],
}
impl UDEPRECATED_OptimusSkinnedMeshExecDataInterface {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDEPRECATED_OptimusSkinnedMeshExecDataInterface")
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
pub struct UOptimusSkinnedMeshReadDataInterface {
    __padding_end: [u8; 48],
}
impl UOptimusSkinnedMeshReadDataInterface {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UOptimusSkinnedMeshReadDataInterface")
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
pub struct UOptimusSkinnedMeshReadDataProvider {
    __padding_end: [u8; 88],
}
impl UOptimusSkinnedMeshReadDataProvider {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UOptimusSkinnedMeshReadDataProvider")
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
pub struct UOptimusSkinnedMeshVertexAttributeDataInterface {
    __padding_end: [u8; 64],
}
impl UOptimusSkinnedMeshVertexAttributeDataInterface {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UOptimusSkinnedMeshVertexAttributeDataInterface")
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
pub struct UOptimusSkinnedMeshVertexAttributeDataProvider {
    #[doc(hidden)]
    pub(crate) __padding_64: [u8; 64],
    pub attribute_name: FName,
    pub default_value: f32,
    __padding_end: [u8; 16],
}
impl UOptimusSkinnedMeshVertexAttributeDataProvider {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UOptimusSkinnedMeshVertexAttributeDataProvider")
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
pub struct UOptimusSkinnedMeshWriteDataInterface {
    __padding_end: [u8; 56],
}
impl UOptimusSkinnedMeshWriteDataInterface {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UOptimusSkinnedMeshWriteDataInterface")
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
pub struct UOptimusSkinnedMeshWriteDataProvider {
    __padding_end: [u8; 96],
}
impl UOptimusSkinnedMeshWriteDataProvider {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UOptimusSkinnedMeshWriteDataProvider")
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
pub struct UOptimusSkinWeightsAsVertexMaskDataInterface {
    __padding_end: [u8; 168],
}
impl UOptimusSkinWeightsAsVertexMaskDataInterface {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UOptimusSkinWeightsAsVertexMaskDataInterface")
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
pub struct UOptimusSkinWeightsAsVertexMaskDataProvider {
    __padding_end: [u8; 232],
}
impl UOptimusSkinWeightsAsVertexMaskDataProvider {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UOptimusSkinWeightsAsVertexMaskDataProvider")
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
pub struct UOptimusNode {
    __padding_end: [u8; 272],
}
impl UOptimusNode {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UOptimusNode")
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
    pub fn set_graph_position(
        &mut self,
        in_position: &crate::bindings::core_u_object::FVector2D,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<17>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::optimus_core::__FUNCTION_PTRS
                    .u_optimus_node_set_graph_position,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_position,
                __buffer.add(0).cast::<crate::bindings::core_u_object::FVector2D>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::optimus_core::__FUNCTION_PTRS
                    .u_optimus_node_set_graph_position,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<bool>().read() }
    }
    pub fn get_node_name(&self) -> FName {
        let mut __stack = crate::core_data::StackAlloc::<12>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::optimus_core::__FUNCTION_PTRS
                    .u_optimus_node_get_node_name,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::optimus_core::__FUNCTION_PTRS
                    .u_optimus_node_get_node_name,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<FName>().read() }
    }
    pub fn get_node_category(&self) -> FName {
        let mut __stack = crate::core_data::StackAlloc::<12>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::optimus_core::__FUNCTION_PTRS
                    .u_optimus_node_get_node_category,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::optimus_core::__FUNCTION_PTRS
                    .u_optimus_node_get_node_category,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<FName>().read() }
    }
    pub fn get_graph_position(&self) -> crate::bindings::core_u_object::FVector2D {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::optimus_core::__FUNCTION_PTRS
                    .u_optimus_node_get_graph_position,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::optimus_core::__FUNCTION_PTRS
                    .u_optimus_node_get_graph_position,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<crate::bindings::core_u_object::FVector2D>().read()
        }
    }
    pub fn get_display_name(&self) -> FText {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::optimus_core::__FUNCTION_PTRS
                    .u_optimus_node_get_display_name,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::optimus_core::__FUNCTION_PTRS
                    .u_optimus_node_get_display_name,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<FText>().read() }
    }
}
#[repr(C, align(8))]
pub struct UOptimusNode_DataInterface {
    __padding_end: [u8; 320],
}
impl UOptimusNode_DataInterface {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UOptimusNode_DataInterface")
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
pub struct UOptimusNode_AnimAttributeDataInterface {
    __padding_end: [u8; 320],
}
impl UOptimusNode_AnimAttributeDataInterface {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UOptimusNode_AnimAttributeDataInterface")
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
pub struct UOptimusNode_ComponentSource {
    __padding_end: [u8; 320],
}
impl UOptimusNode_ComponentSource {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UOptimusNode_ComponentSource")
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
pub struct UOptimusNode_ComputeKernelBase {
    __padding_end: [u8; 280],
}
impl UOptimusNode_ComputeKernelBase {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UOptimusNode_ComputeKernelBase")
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
pub struct UOptimusNode_ComputeKernelFunctionGeneratorClass {
    __padding_end: [u8; 744],
}
impl UOptimusNode_ComputeKernelFunctionGeneratorClass {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UOptimusNode_ComputeKernelFunctionGeneratorClass")
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
pub struct UOptimusNode_ComputeKernelFunction {
    __padding_end: [u8; 280],
}
impl UOptimusNode_ComputeKernelFunction {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UOptimusNode_ComputeKernelFunction")
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
pub struct UOptimusNode_ConstantValueGeneratorClass {
    __padding_end: [u8; 688],
}
impl UOptimusNode_ConstantValueGeneratorClass {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UOptimusNode_ConstantValueGeneratorClass")
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
pub struct UOptimusNode_ConstantValue {
    __padding_end: [u8; 296],
}
impl UOptimusNode_ConstantValue {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UOptimusNode_ConstantValue")
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
pub struct UOptimusNode_CustomComputeKernel {
    __padding_end: [u8; 528],
}
impl UOptimusNode_CustomComputeKernel {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UOptimusNode_CustomComputeKernel")
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
pub struct UOptimusNode_FunctionReference {
    __padding_end: [u8; 392],
}
impl UOptimusNode_FunctionReference {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UOptimusNode_FunctionReference")
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
pub struct UOptimusNode_ResourceAccessorBase {
    __padding_end: [u8; 464],
}
impl UOptimusNode_ResourceAccessorBase {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UOptimusNode_ResourceAccessorBase")
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
pub struct UOptimusNode_GetResource {
    __padding_end: [u8; 464],
}
impl UOptimusNode_GetResource {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UOptimusNode_GetResource")
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
pub struct UOptimusNode_GetVariable {
    __padding_end: [u8; 400],
}
impl UOptimusNode_GetVariable {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UOptimusNode_GetVariable")
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
pub struct UOptimusNode_GraphTerminal {
    __padding_end: [u8; 328],
}
impl UOptimusNode_GraphTerminal {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UOptimusNode_GraphTerminal")
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
pub struct UOptimusNode_LoopTerminal {
    __padding_end: [u8; 376],
}
impl UOptimusNode_LoopTerminal {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UOptimusNode_LoopTerminal")
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
pub struct UOptimusNode_Resource {
    __padding_end: [u8; 464],
}
impl UOptimusNode_Resource {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UOptimusNode_Resource")
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
pub struct UOptimusNode_SetResource {
    __padding_end: [u8; 464],
}
impl UOptimusNode_SetResource {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UOptimusNode_SetResource")
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
pub struct UOptimusNode_SubGraphReference {
    __padding_end: [u8; 328],
}
impl UOptimusNode_SubGraphReference {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UOptimusNode_SubGraphReference")
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
pub struct UOptimusActionStack {
    __padding_end: [u8; 192],
}
impl UOptimusActionStack {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UOptimusActionStack")
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
pub struct UOptimusComponentSourceBinding {
    __padding_end: [u8; 104],
}
impl UOptimusComponentSourceBinding {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UOptimusComponentSourceBinding")
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
pub struct UOptimusComputeGraph {
    __padding_end: [u8; 248],
}
impl UOptimusComputeGraph {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UOptimusComputeGraph")
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
pub struct UOptimusComponentSourceBindingContainer {
    __padding_end: [u8; 64],
}
impl UOptimusComponentSourceBindingContainer {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UOptimusComponentSourceBindingContainer")
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
pub struct UOptimusVariableContainer {
    __padding_end: [u8; 64],
}
impl UOptimusVariableContainer {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UOptimusVariableContainer")
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
pub struct UOptimusResourceContainer {
    __padding_end: [u8; 64],
}
impl UOptimusResourceContainer {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UOptimusResourceContainer")
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
pub struct UOptimusDeformer {
    __padding_end: [u8; 520],
}
impl UOptimusDeformer {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UOptimusDeformer")
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
    pub fn get_variables(&self) -> TArray<UPtr<UOptimusVariableDescription>> {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::optimus_core::__FUNCTION_PTRS
                    .u_optimus_deformer_get_variables,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::optimus_core::__FUNCTION_PTRS
                    .u_optimus_deformer_get_variables,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<TArray<UPtr<UOptimusVariableDescription>>>().read()
        }
    }
    pub fn get_resources(&self) -> TArray<UPtr<UOptimusResourceDescription>> {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::optimus_core::__FUNCTION_PTRS
                    .u_optimus_deformer_get_resources,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::optimus_core::__FUNCTION_PTRS
                    .u_optimus_deformer_get_resources,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<TArray<UPtr<UOptimusResourceDescription>>>().read()
        }
    }
    pub fn get_primary_component_binding(&self) -> UPtr<UOptimusComponentSourceBinding> {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::optimus_core::__FUNCTION_PTRS
                    .u_optimus_deformer_get_primary_component_binding,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::optimus_core::__FUNCTION_PTRS
                    .u_optimus_deformer_get_primary_component_binding,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<UPtr<UOptimusComponentSourceBinding>>().read() }
    }
    pub fn get_component_bindings(
        &self,
    ) -> TArray<UPtr<UOptimusComponentSourceBinding>> {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::optimus_core::__FUNCTION_PTRS
                    .u_optimus_deformer_get_component_bindings,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::optimus_core::__FUNCTION_PTRS
                    .u_optimus_deformer_get_component_bindings,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<TArray<UPtr<UOptimusComponentSourceBinding>>>().read()
        }
    }
}
#[repr(C, align(8))]
pub struct UOptimusDeformerDynamicInstanceManager {
    __padding_end: [u8; 448],
}
impl UOptimusDeformerDynamicInstanceManager {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UOptimusDeformerDynamicInstanceManager")
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
pub struct UOptimusDeformerInstanceSettings {
    __padding_end: [u8; 72],
}
impl UOptimusDeformerInstanceSettings {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UOptimusDeformerInstanceSettings")
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
pub struct UOptimusDeformerInstance {
    __padding_end: [u8; 448],
}
impl UOptimusDeformerInstance {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UOptimusDeformerInstance")
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
    pub fn set_vector_variable(
        &mut self,
        in_variable_name: FName,
        in_value: &crate::bindings::core_u_object::FVector,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<41>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::optimus_core::__FUNCTION_PTRS
                    .u_optimus_deformer_instance_set_vector_variable,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_variable_name,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_value,
                __buffer.add(16).cast::<crate::bindings::core_u_object::FVector>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::optimus_core::__FUNCTION_PTRS
                    .u_optimus_deformer_instance_set_vector_variable,
                __buffer,
            )
        };
        unsafe { __buffer.add(40).cast::<bool>().read() }
    }
    pub fn set_vector_array_variable(
        &mut self,
        in_variable_name: FName,
        in_value: &TArray<crate::bindings::core_u_object::FVector>,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<33>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::optimus_core::__FUNCTION_PTRS
                    .u_optimus_deformer_instance_set_vector_array_variable,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_variable_name,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_value,
                __buffer
                    .add(16)
                    .cast::<TArray<crate::bindings::core_u_object::FVector>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::optimus_core::__FUNCTION_PTRS
                    .u_optimus_deformer_instance_set_vector_array_variable,
                __buffer,
            )
        };
        unsafe { __buffer.add(32).cast::<bool>().read() }
    }
    pub fn set_vector4_variable(
        &mut self,
        in_variable_name: FName,
        in_value: &crate::bindings::core_u_object::FVector4,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<49>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::optimus_core::__FUNCTION_PTRS
                    .u_optimus_deformer_instance_set_vector4_variable,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_variable_name,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_value,
                __buffer.add(16).cast::<crate::bindings::core_u_object::FVector4>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::optimus_core::__FUNCTION_PTRS
                    .u_optimus_deformer_instance_set_vector4_variable,
                __buffer,
            )
        };
        unsafe { __buffer.add(48).cast::<bool>().read() }
    }
    pub fn set_vector4_array_variable(
        &mut self,
        in_variable_name: FName,
        in_value: &TArray<crate::bindings::core_u_object::FVector4>,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<33>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::optimus_core::__FUNCTION_PTRS
                    .u_optimus_deformer_instance_set_vector4_array_variable,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_variable_name,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_value,
                __buffer
                    .add(16)
                    .cast::<TArray<crate::bindings::core_u_object::FVector4>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::optimus_core::__FUNCTION_PTRS
                    .u_optimus_deformer_instance_set_vector4_array_variable,
                __buffer,
            )
        };
        unsafe { __buffer.add(32).cast::<bool>().read() }
    }
    pub fn set_vector2_variable(
        &mut self,
        in_variable_name: FName,
        in_value: &crate::bindings::core_u_object::FVector2D,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<33>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::optimus_core::__FUNCTION_PTRS
                    .u_optimus_deformer_instance_set_vector2_variable,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_variable_name,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_value,
                __buffer.add(16).cast::<crate::bindings::core_u_object::FVector2D>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::optimus_core::__FUNCTION_PTRS
                    .u_optimus_deformer_instance_set_vector2_variable,
                __buffer,
            )
        };
        unsafe { __buffer.add(32).cast::<bool>().read() }
    }
    pub fn set_vector2_array_variable(
        &mut self,
        in_variable_name: FName,
        in_value: &TArray<crate::bindings::core_u_object::FVector2D>,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<33>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::optimus_core::__FUNCTION_PTRS
                    .u_optimus_deformer_instance_set_vector2_array_variable,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_variable_name,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_value,
                __buffer
                    .add(16)
                    .cast::<TArray<crate::bindings::core_u_object::FVector2D>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::optimus_core::__FUNCTION_PTRS
                    .u_optimus_deformer_instance_set_vector2_array_variable,
                __buffer,
            )
        };
        unsafe { __buffer.add(32).cast::<bool>().read() }
    }
    pub fn set_transform_variable(
        &mut self,
        in_variable_name: FName,
        in_value: &crate::bindings::core_u_object::FTransform,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<113>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::optimus_core::__FUNCTION_PTRS
                    .u_optimus_deformer_instance_set_transform_variable,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_variable_name,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_value,
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
                crate::bindings::optimus_core::__FUNCTION_PTRS
                    .u_optimus_deformer_instance_set_transform_variable,
                __buffer,
            )
        };
        unsafe { __buffer.add(112).cast::<bool>().read() }
    }
    pub fn set_transform_array_variable(
        &mut self,
        in_variable_name: FName,
        in_value: &TArray<crate::bindings::core_u_object::FTransform>,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<33>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::optimus_core::__FUNCTION_PTRS
                    .u_optimus_deformer_instance_set_transform_array_variable,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_variable_name,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_value,
                __buffer
                    .add(16)
                    .cast::<TArray<crate::bindings::core_u_object::FTransform>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::optimus_core::__FUNCTION_PTRS
                    .u_optimus_deformer_instance_set_transform_array_variable,
                __buffer,
            )
        };
        unsafe { __buffer.add(32).cast::<bool>().read() }
    }
    pub fn set_rotator_variable(
        &mut self,
        in_variable_name: FName,
        in_value: &crate::bindings::core_u_object::FRotator,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<41>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::optimus_core::__FUNCTION_PTRS
                    .u_optimus_deformer_instance_set_rotator_variable,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_variable_name,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_value,
                __buffer.add(16).cast::<crate::bindings::core_u_object::FRotator>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::optimus_core::__FUNCTION_PTRS
                    .u_optimus_deformer_instance_set_rotator_variable,
                __buffer,
            )
        };
        unsafe { __buffer.add(40).cast::<bool>().read() }
    }
    pub fn set_rotator_array_variable(
        &mut self,
        in_variable_name: FName,
        in_value: &TArray<crate::bindings::core_u_object::FRotator>,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<33>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::optimus_core::__FUNCTION_PTRS
                    .u_optimus_deformer_instance_set_rotator_array_variable,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_variable_name,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_value,
                __buffer
                    .add(16)
                    .cast::<TArray<crate::bindings::core_u_object::FRotator>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::optimus_core::__FUNCTION_PTRS
                    .u_optimus_deformer_instance_set_rotator_array_variable,
                __buffer,
            )
        };
        unsafe { __buffer.add(32).cast::<bool>().read() }
    }
    pub fn set_quat_variable(
        &mut self,
        in_variable_name: FName,
        in_value: &crate::bindings::core_u_object::FQuat,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<49>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::optimus_core::__FUNCTION_PTRS
                    .u_optimus_deformer_instance_set_quat_variable,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_variable_name,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_value,
                __buffer.add(16).cast::<crate::bindings::core_u_object::FQuat>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::optimus_core::__FUNCTION_PTRS
                    .u_optimus_deformer_instance_set_quat_variable,
                __buffer,
            )
        };
        unsafe { __buffer.add(48).cast::<bool>().read() }
    }
    pub fn set_quat_array_variable(
        &mut self,
        in_variable_name: FName,
        in_value: &TArray<crate::bindings::core_u_object::FQuat>,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<33>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::optimus_core::__FUNCTION_PTRS
                    .u_optimus_deformer_instance_set_quat_array_variable,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_variable_name,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_value,
                __buffer.add(16).cast::<TArray<crate::bindings::core_u_object::FQuat>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::optimus_core::__FUNCTION_PTRS
                    .u_optimus_deformer_instance_set_quat_array_variable,
                __buffer,
            )
        };
        unsafe { __buffer.add(32).cast::<bool>().read() }
    }
    pub fn set_name_variable(
        &mut self,
        in_variable_name: FName,
        in_value: &FName,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<25>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::optimus_core::__FUNCTION_PTRS
                    .u_optimus_deformer_instance_set_name_variable,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_variable_name,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(in_value, __buffer.add(12).cast::<FName>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::optimus_core::__FUNCTION_PTRS
                    .u_optimus_deformer_instance_set_name_variable,
                __buffer,
            )
        };
        unsafe { __buffer.add(24).cast::<bool>().read() }
    }
    pub fn set_name_array_variable(
        &mut self,
        in_variable_name: FName,
        in_value: &TArray<FName>,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<33>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::optimus_core::__FUNCTION_PTRS
                    .u_optimus_deformer_instance_set_name_array_variable,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_variable_name,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_value,
                __buffer.add(16).cast::<TArray<FName>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::optimus_core::__FUNCTION_PTRS
                    .u_optimus_deformer_instance_set_name_array_variable,
                __buffer,
            )
        };
        unsafe { __buffer.add(32).cast::<bool>().read() }
    }
    pub fn set_linear_color_variable(
        &mut self,
        in_variable_name: FName,
        in_value: &crate::bindings::core_u_object::FLinearColor,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<29>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::optimus_core::__FUNCTION_PTRS
                    .u_optimus_deformer_instance_set_linear_color_variable,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_variable_name,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_value,
                __buffer.add(12).cast::<crate::bindings::core_u_object::FLinearColor>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::optimus_core::__FUNCTION_PTRS
                    .u_optimus_deformer_instance_set_linear_color_variable,
                __buffer,
            )
        };
        unsafe { __buffer.add(28).cast::<bool>().read() }
    }
    pub fn set_linear_color_array_variable(
        &mut self,
        in_variable_name: FName,
        in_value: &TArray<crate::bindings::core_u_object::FLinearColor>,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<33>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::optimus_core::__FUNCTION_PTRS
                    .u_optimus_deformer_instance_set_linear_color_array_variable,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_variable_name,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_value,
                __buffer
                    .add(16)
                    .cast::<TArray<crate::bindings::core_u_object::FLinearColor>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::optimus_core::__FUNCTION_PTRS
                    .u_optimus_deformer_instance_set_linear_color_array_variable,
                __buffer,
            )
        };
        unsafe { __buffer.add(32).cast::<bool>().read() }
    }
    pub fn set_int_variable(&mut self, in_variable_name: FName, in_value: i32) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<17>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::optimus_core::__FUNCTION_PTRS
                    .u_optimus_deformer_instance_set_int_variable,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_variable_name,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&in_value, __buffer.add(12).cast::<i32>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::optimus_core::__FUNCTION_PTRS
                    .u_optimus_deformer_instance_set_int_variable,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<bool>().read() }
    }
    pub fn set_int_array_variable(
        &mut self,
        in_variable_name: FName,
        in_value: &TArray<i32>,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<33>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::optimus_core::__FUNCTION_PTRS
                    .u_optimus_deformer_instance_set_int_array_variable,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_variable_name,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_value,
                __buffer.add(16).cast::<TArray<i32>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::optimus_core::__FUNCTION_PTRS
                    .u_optimus_deformer_instance_set_int_array_variable,
                __buffer,
            )
        };
        unsafe { __buffer.add(32).cast::<bool>().read() }
    }
    pub fn set_int4_variable(
        &mut self,
        in_variable_name: FName,
        in_value: &crate::bindings::core_u_object::FIntVector4,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<29>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::optimus_core::__FUNCTION_PTRS
                    .u_optimus_deformer_instance_set_int4_variable,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_variable_name,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_value,
                __buffer.add(12).cast::<crate::bindings::core_u_object::FIntVector4>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::optimus_core::__FUNCTION_PTRS
                    .u_optimus_deformer_instance_set_int4_variable,
                __buffer,
            )
        };
        unsafe { __buffer.add(28).cast::<bool>().read() }
    }
    pub fn set_int4_array_variable(
        &mut self,
        in_variable_name: FName,
        in_value: &TArray<crate::bindings::core_u_object::FIntVector4>,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<33>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::optimus_core::__FUNCTION_PTRS
                    .u_optimus_deformer_instance_set_int4_array_variable,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_variable_name,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_value,
                __buffer
                    .add(16)
                    .cast::<TArray<crate::bindings::core_u_object::FIntVector4>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::optimus_core::__FUNCTION_PTRS
                    .u_optimus_deformer_instance_set_int4_array_variable,
                __buffer,
            )
        };
        unsafe { __buffer.add(32).cast::<bool>().read() }
    }
    pub fn set_int3_variable(
        &mut self,
        in_variable_name: FName,
        in_value: &crate::bindings::core_u_object::FIntVector,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<25>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::optimus_core::__FUNCTION_PTRS
                    .u_optimus_deformer_instance_set_int3_variable,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_variable_name,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_value,
                __buffer.add(12).cast::<crate::bindings::core_u_object::FIntVector>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::optimus_core::__FUNCTION_PTRS
                    .u_optimus_deformer_instance_set_int3_variable,
                __buffer,
            )
        };
        unsafe { __buffer.add(24).cast::<bool>().read() }
    }
    pub fn set_int3_array_variable(
        &mut self,
        in_variable_name: FName,
        in_value: &TArray<crate::bindings::core_u_object::FIntVector>,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<33>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::optimus_core::__FUNCTION_PTRS
                    .u_optimus_deformer_instance_set_int3_array_variable,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_variable_name,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_value,
                __buffer
                    .add(16)
                    .cast::<TArray<crate::bindings::core_u_object::FIntVector>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::optimus_core::__FUNCTION_PTRS
                    .u_optimus_deformer_instance_set_int3_array_variable,
                __buffer,
            )
        };
        unsafe { __buffer.add(32).cast::<bool>().read() }
    }
    pub fn set_int2_variable(
        &mut self,
        in_variable_name: FName,
        in_value: &crate::bindings::core_u_object::FIntPoint,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<21>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::optimus_core::__FUNCTION_PTRS
                    .u_optimus_deformer_instance_set_int2_variable,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_variable_name,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_value,
                __buffer.add(12).cast::<crate::bindings::core_u_object::FIntPoint>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::optimus_core::__FUNCTION_PTRS
                    .u_optimus_deformer_instance_set_int2_variable,
                __buffer,
            )
        };
        unsafe { __buffer.add(20).cast::<bool>().read() }
    }
    pub fn set_int2_array_variable(
        &mut self,
        in_variable_name: FName,
        in_value: &TArray<crate::bindings::core_u_object::FIntPoint>,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<33>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::optimus_core::__FUNCTION_PTRS
                    .u_optimus_deformer_instance_set_int2_array_variable,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_variable_name,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_value,
                __buffer
                    .add(16)
                    .cast::<TArray<crate::bindings::core_u_object::FIntPoint>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::optimus_core::__FUNCTION_PTRS
                    .u_optimus_deformer_instance_set_int2_array_variable,
                __buffer,
            )
        };
        unsafe { __buffer.add(32).cast::<bool>().read() }
    }
    pub fn set_float_variable(
        &mut self,
        in_variable_name: FName,
        in_value: f64,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<25>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::optimus_core::__FUNCTION_PTRS
                    .u_optimus_deformer_instance_set_float_variable,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_variable_name,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&in_value, __buffer.add(16).cast::<f64>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::optimus_core::__FUNCTION_PTRS
                    .u_optimus_deformer_instance_set_float_variable,
                __buffer,
            )
        };
        unsafe { __buffer.add(24).cast::<bool>().read() }
    }
    pub fn set_float_array_variable(
        &mut self,
        in_variable_name: FName,
        in_value: &TArray<f64>,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<33>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::optimus_core::__FUNCTION_PTRS
                    .u_optimus_deformer_instance_set_float_array_variable,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_variable_name,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_value,
                __buffer.add(16).cast::<TArray<f64>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::optimus_core::__FUNCTION_PTRS
                    .u_optimus_deformer_instance_set_float_array_variable,
                __buffer,
            )
        };
        unsafe { __buffer.add(32).cast::<bool>().read() }
    }
    pub fn set_bool_variable(
        &mut self,
        in_variable_name: FName,
        in_value: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<14>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::optimus_core::__FUNCTION_PTRS
                    .u_optimus_deformer_instance_set_bool_variable,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_variable_name,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&in_value, __buffer.add(12).cast::<bool>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::optimus_core::__FUNCTION_PTRS
                    .u_optimus_deformer_instance_set_bool_variable,
                __buffer,
            )
        };
        unsafe { __buffer.add(13).cast::<bool>().read() }
    }
    pub fn set_bool_array_variable(
        &mut self,
        in_variable_name: FName,
        in_value: &TArray<bool>,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<33>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::optimus_core::__FUNCTION_PTRS
                    .u_optimus_deformer_instance_set_bool_array_variable,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_variable_name,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_value,
                __buffer.add(16).cast::<TArray<bool>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::optimus_core::__FUNCTION_PTRS
                    .u_optimus_deformer_instance_set_bool_array_variable,
                __buffer,
            )
        };
        unsafe { __buffer.add(32).cast::<bool>().read() }
    }
    pub fn enqueue_trigger_graph(&mut self, in_trigger_graph_name: FName) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<13>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::optimus_core::__FUNCTION_PTRS
                    .u_optimus_deformer_instance_enqueue_trigger_graph,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_trigger_graph_name,
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
                crate::bindings::optimus_core::__FUNCTION_PTRS
                    .u_optimus_deformer_instance_enqueue_trigger_graph,
                __buffer,
            )
        };
        unsafe { __buffer.add(12).cast::<bool>().read() }
    }
}
#[repr(C, align(8))]
pub struct UOptimusNodeGraph {
    #[doc(hidden)]
    pub(crate) __padding_56: [u8; 56],
    pub graph_type: EOptimusNodeGraphType,
    __padding_end: [u8; 124],
}
impl UOptimusNodeGraph {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UOptimusNodeGraph")
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
    pub fn rename_graph_direct(
        &mut self,
        in_graph: UPtr<UOptimusNodeGraph>,
        in_new_name: FString,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<25>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::optimus_core::__FUNCTION_PTRS
                    .u_optimus_node_graph_rename_graph_direct,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_graph,
                __buffer.add(0).cast::<UPtr<UOptimusNodeGraph>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_new_name,
                __buffer.add(8).cast::<FString>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::optimus_core::__FUNCTION_PTRS
                    .u_optimus_node_graph_rename_graph_direct,
                __buffer,
            )
        };
        unsafe { __buffer.add(24).cast::<bool>().read() }
    }
    pub fn rename_graph(
        &mut self,
        in_graph: UPtr<UOptimusNodeGraph>,
        in_new_name: FString,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<25>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::optimus_core::__FUNCTION_PTRS
                    .u_optimus_node_graph_rename_graph,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_graph,
                __buffer.add(0).cast::<UPtr<UOptimusNodeGraph>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_new_name,
                __buffer.add(8).cast::<FString>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::optimus_core::__FUNCTION_PTRS
                    .u_optimus_node_graph_rename_graph,
                __buffer,
            )
        };
        unsafe { __buffer.add(24).cast::<bool>().read() }
    }
    pub fn remove_nodes(&mut self, in_nodes: &TArray<UPtr<UOptimusNode>>) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<17>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::optimus_core::__FUNCTION_PTRS
                    .u_optimus_node_graph_remove_nodes,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_nodes,
                __buffer.add(0).cast::<TArray<UPtr<UOptimusNode>>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::optimus_core::__FUNCTION_PTRS
                    .u_optimus_node_graph_remove_nodes,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<bool>().read() }
    }
    pub fn remove_node(&mut self, in_node: UPtr<UOptimusNode>) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<9>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::optimus_core::__FUNCTION_PTRS
                    .u_optimus_node_graph_remove_node,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_node,
                __buffer.add(0).cast::<UPtr<UOptimusNode>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::optimus_core::__FUNCTION_PTRS
                    .u_optimus_node_graph_remove_node,
                __buffer,
            )
        };
        unsafe { __buffer.add(8).cast::<bool>().read() }
    }
    pub fn remove_link(
        &mut self,
        in_node_output_pin: UPtr<UOptimusNodePin>,
        in_node_input_pin: UPtr<UOptimusNodePin>,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<17>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::optimus_core::__FUNCTION_PTRS
                    .u_optimus_node_graph_remove_link,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_node_output_pin,
                __buffer.add(0).cast::<UPtr<UOptimusNodePin>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_node_input_pin,
                __buffer.add(8).cast::<UPtr<UOptimusNodePin>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::optimus_core::__FUNCTION_PTRS
                    .u_optimus_node_graph_remove_link,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<bool>().read() }
    }
    pub fn remove_all_links(&mut self, in_node_pin: UPtr<UOptimusNodePin>) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<9>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::optimus_core::__FUNCTION_PTRS
                    .u_optimus_node_graph_remove_all_links,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_node_pin,
                __buffer.add(0).cast::<UPtr<UOptimusNodePin>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::optimus_core::__FUNCTION_PTRS
                    .u_optimus_node_graph_remove_all_links,
                __buffer,
            )
        };
        unsafe { __buffer.add(8).cast::<bool>().read() }
    }
    pub fn move_graph_direct(
        &mut self,
        in_graph: UPtr<UOptimusNodeGraph>,
        in_insert_before: i32,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<13>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::optimus_core::__FUNCTION_PTRS
                    .u_optimus_node_graph_move_graph_direct,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_graph,
                __buffer.add(0).cast::<UPtr<UOptimusNodeGraph>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_insert_before,
                __buffer.add(8).cast::<i32>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::optimus_core::__FUNCTION_PTRS
                    .u_optimus_node_graph_move_graph_direct,
                __buffer,
            )
        };
        unsafe { __buffer.add(12).cast::<bool>().read() }
    }
    pub fn is_sub_graph_reference(&self, in_node: UPtr<UOptimusNode>) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<9>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::optimus_core::__FUNCTION_PTRS
                    .u_optimus_node_graph_is_sub_graph_reference,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_node,
                __buffer.add(0).cast::<UPtr<UOptimusNode>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::optimus_core::__FUNCTION_PTRS
                    .u_optimus_node_graph_is_sub_graph_reference,
                __buffer,
            )
        };
        unsafe { __buffer.add(8).cast::<bool>().read() }
    }
    pub fn is_kernel_function(&self, in_node: UPtr<UOptimusNode>) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<9>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::optimus_core::__FUNCTION_PTRS
                    .u_optimus_node_graph_is_kernel_function,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_node,
                __buffer.add(0).cast::<UPtr<UOptimusNode>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::optimus_core::__FUNCTION_PTRS
                    .u_optimus_node_graph_is_kernel_function,
                __buffer,
            )
        };
        unsafe { __buffer.add(8).cast::<bool>().read() }
    }
    pub fn is_function_reference(&self, in_node: UPtr<UOptimusNode>) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<9>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::optimus_core::__FUNCTION_PTRS
                    .u_optimus_node_graph_is_function_reference,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_node,
                __buffer.add(0).cast::<UPtr<UOptimusNode>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::optimus_core::__FUNCTION_PTRS
                    .u_optimus_node_graph_is_function_reference,
                __buffer,
            )
        };
        unsafe { __buffer.add(8).cast::<bool>().read() }
    }
    pub fn is_function_graph(&self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::optimus_core::__FUNCTION_PTRS
                    .u_optimus_node_graph_is_function_graph,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::optimus_core::__FUNCTION_PTRS
                    .u_optimus_node_graph_is_function_graph,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn is_execution_graph(&self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::optimus_core::__FUNCTION_PTRS
                    .u_optimus_node_graph_is_execution_graph,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::optimus_core::__FUNCTION_PTRS
                    .u_optimus_node_graph_is_execution_graph,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn is_custom_kernel(&self, in_node: UPtr<UOptimusNode>) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<9>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::optimus_core::__FUNCTION_PTRS
                    .u_optimus_node_graph_is_custom_kernel,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_node,
                __buffer.add(0).cast::<UPtr<UOptimusNode>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::optimus_core::__FUNCTION_PTRS
                    .u_optimus_node_graph_is_custom_kernel,
                __buffer,
            )
        };
        unsafe { __buffer.add(8).cast::<bool>().read() }
    }
    pub fn get_graph_type(&self) -> EOptimusNodeGraphType {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::optimus_core::__FUNCTION_PTRS
                    .u_optimus_node_graph_get_graph_type,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::optimus_core::__FUNCTION_PTRS
                    .u_optimus_node_graph_get_graph_type,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<EOptimusNodeGraphType>().read() }
    }
    pub fn get_graphs(&self) -> TArray<UPtr<UOptimusNodeGraph>> {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::optimus_core::__FUNCTION_PTRS
                    .u_optimus_node_graph_get_graphs,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::optimus_core::__FUNCTION_PTRS
                    .u_optimus_node_graph_get_graphs,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<TArray<UPtr<UOptimusNodeGraph>>>().read() }
    }
    pub fn get_graph_index(&self) -> i32 {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::optimus_core::__FUNCTION_PTRS
                    .u_optimus_node_graph_get_graph_index,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::optimus_core::__FUNCTION_PTRS
                    .u_optimus_node_graph_get_graph_index,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<i32>().read() }
    }
    pub fn expand_collapsed_nodes(
        &mut self,
        in_graph_reference_node: UPtr<UOptimusNode>,
    ) -> TArray<UPtr<UOptimusNode>> {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::optimus_core::__FUNCTION_PTRS
                    .u_optimus_node_graph_expand_collapsed_nodes,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_graph_reference_node,
                __buffer.add(0).cast::<UPtr<UOptimusNode>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::optimus_core::__FUNCTION_PTRS
                    .u_optimus_node_graph_expand_collapsed_nodes,
                __buffer,
            )
        };
        unsafe { __buffer.add(8).cast::<TArray<UPtr<UOptimusNode>>>().read() }
    }
    pub fn duplicate_nodes(
        &mut self,
        in_nodes: &TArray<UPtr<UOptimusNode>>,
        in_position: &crate::bindings::core_u_object::FVector2D,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<33>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::optimus_core::__FUNCTION_PTRS
                    .u_optimus_node_graph_duplicate_nodes,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_nodes,
                __buffer.add(0).cast::<TArray<UPtr<UOptimusNode>>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_position,
                __buffer.add(16).cast::<crate::bindings::core_u_object::FVector2D>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::optimus_core::__FUNCTION_PTRS
                    .u_optimus_node_graph_duplicate_nodes,
                __buffer,
            )
        };
        unsafe { __buffer.add(32).cast::<bool>().read() }
    }
    pub fn duplicate_node(
        &mut self,
        in_node: UPtr<UOptimusNode>,
        in_position: &crate::bindings::core_u_object::FVector2D,
    ) -> UPtr<UOptimusNode> {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::optimus_core::__FUNCTION_PTRS
                    .u_optimus_node_graph_duplicate_node,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_node,
                __buffer.add(0).cast::<UPtr<UOptimusNode>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_position,
                __buffer.add(8).cast::<crate::bindings::core_u_object::FVector2D>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::optimus_core::__FUNCTION_PTRS
                    .u_optimus_node_graph_duplicate_node,
                __buffer,
            )
        };
        unsafe { __buffer.add(24).cast::<UPtr<UOptimusNode>>().read() }
    }
    pub fn convert_to_sub_graph(
        &mut self,
        in_function_node: UPtr<UOptimusNode>,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<9>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::optimus_core::__FUNCTION_PTRS
                    .u_optimus_node_graph_convert_to_sub_graph,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_function_node,
                __buffer.add(0).cast::<UPtr<UOptimusNode>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::optimus_core::__FUNCTION_PTRS
                    .u_optimus_node_graph_convert_to_sub_graph,
                __buffer,
            )
        };
        unsafe { __buffer.add(8).cast::<bool>().read() }
    }
    pub fn convert_to_function(
        &mut self,
        in_sub_graph_node: UPtr<UOptimusNode>,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<9>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::optimus_core::__FUNCTION_PTRS
                    .u_optimus_node_graph_convert_to_function,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_sub_graph_node,
                __buffer.add(0).cast::<UPtr<UOptimusNode>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::optimus_core::__FUNCTION_PTRS
                    .u_optimus_node_graph_convert_to_function,
                __buffer,
            )
        };
        unsafe { __buffer.add(8).cast::<bool>().read() }
    }
    pub fn convert_function_to_custom_kernel(
        &mut self,
        in_kernel_function: UPtr<UOptimusNode>,
    ) -> UPtr<UOptimusNode> {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::optimus_core::__FUNCTION_PTRS
                    .u_optimus_node_graph_convert_function_to_custom_kernel,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_kernel_function,
                __buffer.add(0).cast::<UPtr<UOptimusNode>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::optimus_core::__FUNCTION_PTRS
                    .u_optimus_node_graph_convert_function_to_custom_kernel,
                __buffer,
            )
        };
        unsafe { __buffer.add(8).cast::<UPtr<UOptimusNode>>().read() }
    }
    pub fn convert_custom_kernel_to_function(
        &mut self,
        in_custom_kernel: UPtr<UOptimusNode>,
    ) -> UPtr<UOptimusNode> {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::optimus_core::__FUNCTION_PTRS
                    .u_optimus_node_graph_convert_custom_kernel_to_function,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_custom_kernel,
                __buffer.add(0).cast::<UPtr<UOptimusNode>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::optimus_core::__FUNCTION_PTRS
                    .u_optimus_node_graph_convert_custom_kernel_to_function,
                __buffer,
            )
        };
        unsafe { __buffer.add(8).cast::<UPtr<UOptimusNode>>().read() }
    }
    pub fn collapse_nodes_to_sub_graph(
        &mut self,
        in_nodes: &TArray<UPtr<UOptimusNode>>,
    ) -> UPtr<UOptimusNode> {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::optimus_core::__FUNCTION_PTRS
                    .u_optimus_node_graph_collapse_nodes_to_sub_graph,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_nodes,
                __buffer.add(0).cast::<TArray<UPtr<UOptimusNode>>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::optimus_core::__FUNCTION_PTRS
                    .u_optimus_node_graph_collapse_nodes_to_sub_graph,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<UPtr<UOptimusNode>>().read() }
    }
    pub fn collapse_nodes_to_function(
        &mut self,
        in_nodes: &TArray<UPtr<UOptimusNode>>,
    ) -> UPtr<UOptimusNode> {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::optimus_core::__FUNCTION_PTRS
                    .u_optimus_node_graph_collapse_nodes_to_function,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_nodes,
                __buffer.add(0).cast::<TArray<UPtr<UOptimusNode>>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::optimus_core::__FUNCTION_PTRS
                    .u_optimus_node_graph_collapse_nodes_to_function,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<UPtr<UOptimusNode>>().read() }
    }
    pub fn add_variable_get_node(
        &mut self,
        in_variable_desc: UPtr<UOptimusVariableDescription>,
        in_position: &crate::bindings::core_u_object::FVector2D,
    ) -> UPtr<UOptimusNode> {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::optimus_core::__FUNCTION_PTRS
                    .u_optimus_node_graph_add_variable_get_node,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_variable_desc,
                __buffer.add(0).cast::<UPtr<UOptimusVariableDescription>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_position,
                __buffer.add(8).cast::<crate::bindings::core_u_object::FVector2D>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::optimus_core::__FUNCTION_PTRS
                    .u_optimus_node_graph_add_variable_get_node,
                __buffer,
            )
        };
        unsafe { __buffer.add(24).cast::<UPtr<UOptimusNode>>().read() }
    }
    pub fn add_value_node(
        &mut self,
        in_data_type_ref: FOptimusDataTypeRef,
        in_position: &crate::bindings::core_u_object::FVector2D,
    ) -> UPtr<UOptimusNode> {
        let mut __stack = crate::core_data::StackAlloc::<88>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::optimus_core::__FUNCTION_PTRS
                    .u_optimus_node_graph_add_value_node,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_data_type_ref,
                __buffer.add(0).cast::<FOptimusDataTypeRef>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_position,
                __buffer.add(64).cast::<crate::bindings::core_u_object::FVector2D>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::optimus_core::__FUNCTION_PTRS
                    .u_optimus_node_graph_add_value_node,
                __buffer,
            )
        };
        unsafe { __buffer.add(80).cast::<UPtr<UOptimusNode>>().read() }
    }
    pub fn add_resource_set_node(
        &mut self,
        in_resource_desc: UPtr<UOptimusResourceDescription>,
        in_position: &crate::bindings::core_u_object::FVector2D,
    ) -> UPtr<UOptimusNode> {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::optimus_core::__FUNCTION_PTRS
                    .u_optimus_node_graph_add_resource_set_node,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_resource_desc,
                __buffer.add(0).cast::<UPtr<UOptimusResourceDescription>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_position,
                __buffer.add(8).cast::<crate::bindings::core_u_object::FVector2D>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::optimus_core::__FUNCTION_PTRS
                    .u_optimus_node_graph_add_resource_set_node,
                __buffer,
            )
        };
        unsafe { __buffer.add(24).cast::<UPtr<UOptimusNode>>().read() }
    }
    pub fn add_resource_node(
        &mut self,
        in_resource_desc: UPtr<UOptimusResourceDescription>,
        in_position: &crate::bindings::core_u_object::FVector2D,
    ) -> UPtr<UOptimusNode> {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::optimus_core::__FUNCTION_PTRS
                    .u_optimus_node_graph_add_resource_node,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_resource_desc,
                __buffer.add(0).cast::<UPtr<UOptimusResourceDescription>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_position,
                __buffer.add(8).cast::<crate::bindings::core_u_object::FVector2D>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::optimus_core::__FUNCTION_PTRS
                    .u_optimus_node_graph_add_resource_node,
                __buffer,
            )
        };
        unsafe { __buffer.add(24).cast::<UPtr<UOptimusNode>>().read() }
    }
    pub fn add_resource_get_node(
        &mut self,
        in_resource_desc: UPtr<UOptimusResourceDescription>,
        in_position: &crate::bindings::core_u_object::FVector2D,
    ) -> UPtr<UOptimusNode> {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::optimus_core::__FUNCTION_PTRS
                    .u_optimus_node_graph_add_resource_get_node,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_resource_desc,
                __buffer.add(0).cast::<UPtr<UOptimusResourceDescription>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_position,
                __buffer.add(8).cast::<crate::bindings::core_u_object::FVector2D>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::optimus_core::__FUNCTION_PTRS
                    .u_optimus_node_graph_add_resource_get_node,
                __buffer,
            )
        };
        unsafe { __buffer.add(24).cast::<UPtr<UOptimusNode>>().read() }
    }
    pub fn add_node(
        &mut self,
        in_node_class: TSubclassOf<UOptimusNode>,
        in_position: &crate::bindings::core_u_object::FVector2D,
    ) -> UPtr<UOptimusNode> {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::optimus_core::__FUNCTION_PTRS
                    .u_optimus_node_graph_add_node,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_node_class,
                __buffer.add(0).cast::<TSubclassOf<UOptimusNode>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_position,
                __buffer.add(8).cast::<crate::bindings::core_u_object::FVector2D>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::optimus_core::__FUNCTION_PTRS
                    .u_optimus_node_graph_add_node,
                __buffer,
            )
        };
        unsafe { __buffer.add(24).cast::<UPtr<UOptimusNode>>().read() }
    }
    pub fn add_loop_terminal_nodes(
        &mut self,
        in_position: &crate::bindings::core_u_object::FVector2D,
    ) -> TArray<UPtr<UOptimusNode>> {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::optimus_core::__FUNCTION_PTRS
                    .u_optimus_node_graph_add_loop_terminal_nodes,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_position,
                __buffer.add(0).cast::<crate::bindings::core_u_object::FVector2D>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::optimus_core::__FUNCTION_PTRS
                    .u_optimus_node_graph_add_loop_terminal_nodes,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<TArray<UPtr<UOptimusNode>>>().read() }
    }
    pub fn add_link(
        &mut self,
        in_node_output_pin: UPtr<UOptimusNodePin>,
        in_node_input_pin: UPtr<UOptimusNodePin>,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<17>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::optimus_core::__FUNCTION_PTRS
                    .u_optimus_node_graph_add_link,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_node_output_pin,
                __buffer.add(0).cast::<UPtr<UOptimusNodePin>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_node_input_pin,
                __buffer.add(8).cast::<UPtr<UOptimusNodePin>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::optimus_core::__FUNCTION_PTRS
                    .u_optimus_node_graph_add_link,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<bool>().read() }
    }
    pub fn add_function_reference_node(
        &mut self,
        in_function_graph: UPtr<UOptimusFunctionNodeGraph>,
        in_position: &crate::bindings::core_u_object::FVector2D,
    ) -> UPtr<UOptimusNode> {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::optimus_core::__FUNCTION_PTRS
                    .u_optimus_node_graph_add_function_reference_node,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_function_graph,
                __buffer.add(0).cast::<UPtr<UOptimusFunctionNodeGraph>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_position,
                __buffer.add(8).cast::<crate::bindings::core_u_object::FVector2D>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::optimus_core::__FUNCTION_PTRS
                    .u_optimus_node_graph_add_function_reference_node,
                __buffer,
            )
        };
        unsafe { __buffer.add(24).cast::<UPtr<UOptimusNode>>().read() }
    }
    pub fn add_data_interface_node(
        &mut self,
        in_data_interface_class: TSubclassOf<UOptimusComputeDataInterface>,
        in_position: &crate::bindings::core_u_object::FVector2D,
    ) -> UPtr<UOptimusNode> {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::optimus_core::__FUNCTION_PTRS
                    .u_optimus_node_graph_add_data_interface_node,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_data_interface_class,
                __buffer.add(0).cast::<TSubclassOf<UOptimusComputeDataInterface>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_position,
                __buffer.add(8).cast::<crate::bindings::core_u_object::FVector2D>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::optimus_core::__FUNCTION_PTRS
                    .u_optimus_node_graph_add_data_interface_node,
                __buffer,
            )
        };
        unsafe { __buffer.add(24).cast::<UPtr<UOptimusNode>>().read() }
    }
    pub fn add_component_binding_get_node(
        &mut self,
        in_component_binding: UPtr<UOptimusComponentSourceBinding>,
        in_position: &crate::bindings::core_u_object::FVector2D,
    ) -> UPtr<UOptimusNode> {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::optimus_core::__FUNCTION_PTRS
                    .u_optimus_node_graph_add_component_binding_get_node,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_component_binding,
                __buffer.add(0).cast::<UPtr<UOptimusComponentSourceBinding>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_position,
                __buffer.add(8).cast::<crate::bindings::core_u_object::FVector2D>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::optimus_core::__FUNCTION_PTRS
                    .u_optimus_node_graph_add_component_binding_get_node,
                __buffer,
            )
        };
        unsafe { __buffer.add(24).cast::<UPtr<UOptimusNode>>().read() }
    }
    pub fn add_comment_node(
        &mut self,
        in_position: &crate::bindings::core_u_object::FVector2D,
        in_size: &crate::bindings::core_u_object::FVector2D,
        in_color: &crate::bindings::core_u_object::FLinearColor,
    ) -> UPtr<UOptimusNode> {
        let mut __stack = crate::core_data::StackAlloc::<56>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::optimus_core::__FUNCTION_PTRS
                    .u_optimus_node_graph_add_comment_node,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_position,
                __buffer.add(0).cast::<crate::bindings::core_u_object::FVector2D>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_size,
                __buffer.add(16).cast::<crate::bindings::core_u_object::FVector2D>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_color,
                __buffer.add(32).cast::<crate::bindings::core_u_object::FLinearColor>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::optimus_core::__FUNCTION_PTRS
                    .u_optimus_node_graph_add_comment_node,
                __buffer,
            )
        };
        unsafe { __buffer.add(48).cast::<UPtr<UOptimusNode>>().read() }
    }
}
#[repr(C, align(8))]
pub struct UOptimusNodeSubGraph {
    __padding_end: [u8; 416],
}
impl UOptimusNodeSubGraph {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UOptimusNodeSubGraph")
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
pub struct UOptimusFunctionNodeGraph {
    __padding_end: [u8; 456],
}
impl UOptimusFunctionNodeGraph {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UOptimusFunctionNodeGraph")
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
pub struct UOptimusNodeLink {
    __padding_end: [u8; 64],
}
impl UOptimusNodeLink {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UOptimusNodeLink")
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
pub struct UOptimusNodePair {
    __padding_end: [u8; 64],
}
impl UOptimusNodePair {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UOptimusNodePair")
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
pub struct UOptimusNodePin {
    __padding_end: [u8; 216],
}
impl UOptimusNodePin {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UOptimusNodePin")
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
pub struct UOptimusNode_Comment {
    __padding_end: [u8; 352],
}
impl UOptimusNode_Comment {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UOptimusNode_Comment")
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
pub struct UOptimusResourceDescription {
    #[doc(hidden)]
    pub(crate) __padding_48: [u8; 48],
    pub resource_name: FName,
    __padding_end: [u8; 164],
}
impl UOptimusResourceDescription {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UOptimusResourceDescription")
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
pub struct UOptimusSource {
    __padding_end: [u8; 88],
}
impl UOptimusSource {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UOptimusSource")
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
pub struct UOptimusValueContainerGeneratorClass {
    __padding_end: [u8; 688],
}
impl UOptimusValueContainerGeneratorClass {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UOptimusValueContainerGeneratorClass")
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
pub struct UOptimusValueContainer {
    __padding_end: [u8; 48],
}
impl UOptimusValueContainer {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UOptimusValueContainer")
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
pub struct UOptimusVariableDescription {
    __padding_end: [u8; 232],
}
impl UOptimusVariableDescription {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UOptimusVariableDescription")
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
pub struct EOptimusDataDomainType(pub i32);
impl EOptimusDataDomainType {
    pub const DIMENSIONAL: EOptimusDataDomainType = EOptimusDataDomainType(0);
    pub const EXPRESSION: EOptimusDataDomainType = EOptimusDataDomainType(1);
}
#[repr(transparent)]
pub struct EOptimusValueType(pub u8);
impl EOptimusValueType {
    pub const INVALID: EOptimusValueType = EOptimusValueType(0);
    pub const CONSTANT: EOptimusValueType = EOptimusValueType(1);
    pub const VARIABLE: EOptimusValueType = EOptimusValueType(2);
}
#[repr(transparent)]
pub struct EOptimusValueUsage(pub u8);
impl EOptimusValueUsage {
    pub const NONE: EOptimusValueUsage = EOptimusValueUsage(0);
    pub const CPU: EOptimusValueUsage = EOptimusValueUsage(1);
    pub const GPU: EOptimusValueUsage = EOptimusValueUsage(2);
}
#[repr(transparent)]
pub struct EOptimusDeformerExecutionPhase(pub u8);
impl EOptimusDeformerExecutionPhase {
    pub const AFTER_DEFAULT_DEFORMER: EOptimusDeformerExecutionPhase = EOptimusDeformerExecutionPhase(
        0,
    );
    pub const OVERRIDE_DEFAULT_DEFORMER: EOptimusDeformerExecutionPhase = EOptimusDeformerExecutionPhase(
        1,
    );
    pub const BEFORE_DEFAULT_DEFORMER: EOptimusDeformerExecutionPhase = EOptimusDeformerExecutionPhase(
        2,
    );
}
#[repr(transparent)]
pub struct EOptimusConstantType(pub i32);
impl EOptimusConstantType {
    pub const INPUT: EOptimusConstantType = EOptimusConstantType(0);
    pub const OUTPUT: EOptimusConstantType = EOptimusConstantType(1);
}
#[repr(transparent)]
pub struct EOptimusDataTypeUsageFlags(pub u8);
impl EOptimusDataTypeUsageFlags {
    pub const NONE: EOptimusDataTypeUsageFlags = EOptimusDataTypeUsageFlags(0);
    pub const RESOURCE: EOptimusDataTypeUsageFlags = EOptimusDataTypeUsageFlags(1);
    pub const VARIABLE: EOptimusDataTypeUsageFlags = EOptimusDataTypeUsageFlags(2);
    pub const ANIM_ATTRIBUTES: EOptimusDataTypeUsageFlags = EOptimusDataTypeUsageFlags(
        4,
    );
    pub const DATA_INTERFACE_OUTPUT: EOptimusDataTypeUsageFlags = EOptimusDataTypeUsageFlags(
        8,
    );
    pub const PIN_TYPE: EOptimusDataTypeUsageFlags = EOptimusDataTypeUsageFlags(16);
    pub const PER_BONE_ANIM_ATTRIBUTE: EOptimusDataTypeUsageFlags = EOptimusDataTypeUsageFlags(
        32,
    );
    pub const PROPERTY: EOptimusDataTypeUsageFlags = EOptimusDataTypeUsageFlags(64);
}
#[repr(transparent)]
pub struct EOptimusDataTypeFlags(pub u8);
impl EOptimusDataTypeFlags {
    pub const NONE: EOptimusDataTypeFlags = EOptimusDataTypeFlags(0);
    pub const IS_STRUCT_TYPE: EOptimusDataTypeFlags = EOptimusDataTypeFlags(1);
    pub const SHOW_ELEMENTS: EOptimusDataTypeFlags = EOptimusDataTypeFlags(2);
}
#[repr(transparent)]
pub struct EOptimusNodeGraphType(pub i32);
impl EOptimusNodeGraphType {
    pub const SETUP: EOptimusNodeGraphType = EOptimusNodeGraphType(0);
    pub const UPDATE: EOptimusNodeGraphType = EOptimusNodeGraphType(1);
    pub const EXTERNAL_TRIGGER: EOptimusNodeGraphType = EOptimusNodeGraphType(2);
    pub const FUNCTION: EOptimusNodeGraphType = EOptimusNodeGraphType(3);
    pub const SUB_GRAPH: EOptimusNodeGraphType = EOptimusNodeGraphType(4);
    pub const TRANSIENT: EOptimusNodeGraphType = EOptimusNodeGraphType(5);
}
#[repr(transparent)]
pub struct EOptimusExecutionDomainType(pub i32);
impl EOptimusExecutionDomainType {
    pub const DOMAIN_NAME: EOptimusExecutionDomainType = EOptimusExecutionDomainType(0);
    pub const EXPRESSION: EOptimusExecutionDomainType = EOptimusExecutionDomainType(1);
}
#[repr(transparent)]
pub struct EOptimusSkinnedMeshExecDomain(pub u8);
impl EOptimusSkinnedMeshExecDomain {
    pub const NONE: EOptimusSkinnedMeshExecDomain = EOptimusSkinnedMeshExecDomain(0);
    pub const VERTEX: EOptimusSkinnedMeshExecDomain = EOptimusSkinnedMeshExecDomain(1);
    pub const TRIANGLE: EOptimusSkinnedMeshExecDomain = EOptimusSkinnedMeshExecDomain(2);
}
#[repr(transparent)]
pub struct EOptimusDiagnosticLevel(pub u8);
impl EOptimusDiagnosticLevel {
    pub const NONE: EOptimusDiagnosticLevel = EOptimusDiagnosticLevel(0);
    pub const INFO: EOptimusDiagnosticLevel = EOptimusDiagnosticLevel(1);
    pub const WARNING: EOptimusDiagnosticLevel = EOptimusDiagnosticLevel(2);
    pub const ERROR: EOptimusDiagnosticLevel = EOptimusDiagnosticLevel(3);
}
#[repr(transparent)]
pub struct EOptimusBufferWriteType(pub u8);
impl EOptimusBufferWriteType {
    pub const WRITE: EOptimusBufferWriteType = EOptimusBufferWriteType(0);
    pub const WRITE_ATOMIC_ADD: EOptimusBufferWriteType = EOptimusBufferWriteType(1);
    pub const WRITE_ATOMIC_MIN: EOptimusBufferWriteType = EOptimusBufferWriteType(2);
    pub const WRITE_ATOMIC_MAX: EOptimusBufferWriteType = EOptimusBufferWriteType(3);
    pub const COUNT: EOptimusBufferWriteType = EOptimusBufferWriteType(4);
}
#[repr(transparent)]
pub struct EOptimusTerminalType(pub i32);
impl EOptimusTerminalType {
    pub const UNKNOWN: EOptimusTerminalType = EOptimusTerminalType(0);
    pub const ENTRY: EOptimusTerminalType = EOptimusTerminalType(1);
    pub const RETURN: EOptimusTerminalType = EOptimusTerminalType(2);
}
#[repr(transparent)]
pub struct EOptimusDeformerStatus(pub i32);
impl EOptimusDeformerStatus {
    pub const COMPILED: EOptimusDeformerStatus = EOptimusDeformerStatus(0);
    pub const COMPILED_WITH_WARNINGS: EOptimusDeformerStatus = EOptimusDeformerStatus(1);
    pub const MODIFIED: EOptimusDeformerStatus = EOptimusDeformerStatus(2);
    pub const HAS_ERRORS: EOptimusDeformerStatus = EOptimusDeformerStatus(3);
}
#[repr(transparent)]
pub struct EOptimusNodePinDirection(pub u8);
impl EOptimusNodePinDirection {
    pub const UNKNOWN: EOptimusNodePinDirection = EOptimusNodePinDirection(0);
    pub const INPUT: EOptimusNodePinDirection = EOptimusNodePinDirection(1);
    pub const OUTPUT: EOptimusNodePinDirection = EOptimusNodePinDirection(2);
}
#[repr(transparent)]
pub struct EOptimusNodePinStorageType(pub u8);
impl EOptimusNodePinStorageType {
    pub const VALUE: EOptimusNodePinStorageType = EOptimusNodePinStorageType(0);
    pub const RESOURCE: EOptimusNodePinStorageType = EOptimusNodePinStorageType(1);
}
