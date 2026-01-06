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
pub static mut U_OPTIMUS_NODE_SET_GRAPH_POSITION: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_OPTIMUS_NODE_GET_NODE_NAME: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_OPTIMUS_NODE_GET_NODE_CATEGORY: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_OPTIMUS_NODE_GET_GRAPH_POSITION: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_OPTIMUS_NODE_GET_DISPLAY_NAME: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_OPTIMUS_DEFORMER_GET_VARIABLES: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_OPTIMUS_DEFORMER_GET_RESOURCES: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_OPTIMUS_DEFORMER_GET_PRIMARY_COMPONENT_BINDING: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_OPTIMUS_DEFORMER_GET_COMPONENT_BINDINGS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_OPTIMUS_DEFORMER_INSTANCE_SET_VECTOR_VARIABLE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_OPTIMUS_DEFORMER_INSTANCE_SET_VECTOR_ARRAY_VARIABLE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_OPTIMUS_DEFORMER_INSTANCE_SET_VECTOR4_VARIABLE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_OPTIMUS_DEFORMER_INSTANCE_SET_VECTOR4_ARRAY_VARIABLE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_OPTIMUS_DEFORMER_INSTANCE_SET_VECTOR2_VARIABLE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_OPTIMUS_DEFORMER_INSTANCE_SET_VECTOR2_ARRAY_VARIABLE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_OPTIMUS_DEFORMER_INSTANCE_SET_TRANSFORM_VARIABLE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_OPTIMUS_DEFORMER_INSTANCE_SET_TRANSFORM_ARRAY_VARIABLE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_OPTIMUS_DEFORMER_INSTANCE_SET_ROTATOR_VARIABLE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_OPTIMUS_DEFORMER_INSTANCE_SET_ROTATOR_ARRAY_VARIABLE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_OPTIMUS_DEFORMER_INSTANCE_SET_QUAT_VARIABLE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_OPTIMUS_DEFORMER_INSTANCE_SET_QUAT_ARRAY_VARIABLE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_OPTIMUS_DEFORMER_INSTANCE_SET_NAME_VARIABLE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_OPTIMUS_DEFORMER_INSTANCE_SET_NAME_ARRAY_VARIABLE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_OPTIMUS_DEFORMER_INSTANCE_SET_LINEAR_COLOR_VARIABLE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_OPTIMUS_DEFORMER_INSTANCE_SET_LINEAR_COLOR_ARRAY_VARIABLE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_OPTIMUS_DEFORMER_INSTANCE_SET_INT_VARIABLE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_OPTIMUS_DEFORMER_INSTANCE_SET_INT_ARRAY_VARIABLE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_OPTIMUS_DEFORMER_INSTANCE_SET_INT4_VARIABLE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_OPTIMUS_DEFORMER_INSTANCE_SET_INT4_ARRAY_VARIABLE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_OPTIMUS_DEFORMER_INSTANCE_SET_INT3_VARIABLE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_OPTIMUS_DEFORMER_INSTANCE_SET_INT3_ARRAY_VARIABLE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_OPTIMUS_DEFORMER_INSTANCE_SET_INT2_VARIABLE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_OPTIMUS_DEFORMER_INSTANCE_SET_INT2_ARRAY_VARIABLE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_OPTIMUS_DEFORMER_INSTANCE_SET_FLOAT_VARIABLE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_OPTIMUS_DEFORMER_INSTANCE_SET_FLOAT_ARRAY_VARIABLE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_OPTIMUS_DEFORMER_INSTANCE_SET_BOOL_VARIABLE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_OPTIMUS_DEFORMER_INSTANCE_SET_BOOL_ARRAY_VARIABLE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_OPTIMUS_DEFORMER_INSTANCE_ENQUEUE_TRIGGER_GRAPH: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_OPTIMUS_NODE_GRAPH_RENAME_GRAPH_DIRECT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_OPTIMUS_NODE_GRAPH_RENAME_GRAPH: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_OPTIMUS_NODE_GRAPH_REMOVE_NODES: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_OPTIMUS_NODE_GRAPH_REMOVE_NODE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_OPTIMUS_NODE_GRAPH_REMOVE_LINK: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_OPTIMUS_NODE_GRAPH_REMOVE_ALL_LINKS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_OPTIMUS_NODE_GRAPH_MOVE_GRAPH_DIRECT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_OPTIMUS_NODE_GRAPH_IS_SUB_GRAPH_REFERENCE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_OPTIMUS_NODE_GRAPH_IS_KERNEL_FUNCTION: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_OPTIMUS_NODE_GRAPH_IS_FUNCTION_REFERENCE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_OPTIMUS_NODE_GRAPH_IS_FUNCTION_GRAPH: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_OPTIMUS_NODE_GRAPH_IS_EXECUTION_GRAPH: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_OPTIMUS_NODE_GRAPH_IS_CUSTOM_KERNEL: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_OPTIMUS_NODE_GRAPH_GET_GRAPH_TYPE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_OPTIMUS_NODE_GRAPH_GET_GRAPHS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_OPTIMUS_NODE_GRAPH_GET_GRAPH_INDEX: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_OPTIMUS_NODE_GRAPH_EXPAND_COLLAPSED_NODES: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_OPTIMUS_NODE_GRAPH_DUPLICATE_NODES: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_OPTIMUS_NODE_GRAPH_DUPLICATE_NODE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_OPTIMUS_NODE_GRAPH_CONVERT_TO_SUB_GRAPH: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_OPTIMUS_NODE_GRAPH_CONVERT_TO_FUNCTION: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_OPTIMUS_NODE_GRAPH_CONVERT_FUNCTION_TO_CUSTOM_KERNEL: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_OPTIMUS_NODE_GRAPH_CONVERT_CUSTOM_KERNEL_TO_FUNCTION: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_OPTIMUS_NODE_GRAPH_COLLAPSE_NODES_TO_SUB_GRAPH: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_OPTIMUS_NODE_GRAPH_COLLAPSE_NODES_TO_FUNCTION: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_OPTIMUS_NODE_GRAPH_ADD_VARIABLE_GET_NODE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_OPTIMUS_NODE_GRAPH_ADD_VALUE_NODE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_OPTIMUS_NODE_GRAPH_ADD_RESOURCE_SET_NODE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_OPTIMUS_NODE_GRAPH_ADD_RESOURCE_NODE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_OPTIMUS_NODE_GRAPH_ADD_RESOURCE_GET_NODE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_OPTIMUS_NODE_GRAPH_ADD_NODE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_OPTIMUS_NODE_GRAPH_ADD_LOOP_TERMINAL_NODES: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_OPTIMUS_NODE_GRAPH_ADD_LINK: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_OPTIMUS_NODE_GRAPH_ADD_FUNCTION_REFERENCE_NODE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_OPTIMUS_NODE_GRAPH_ADD_DATA_INTERFACE_NODE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_OPTIMUS_NODE_GRAPH_ADD_COMPONENT_BINDING_GET_NODE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_OPTIMUS_NODE_GRAPH_ADD_COMMENT_NODE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_OPTIMUS_FUNCTION_NODE_GRAPH_GET_ACCESS_SPECIFIER_OPTIONS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
pub fn initialize() {
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UOptimusNode::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetGraphPosition"),
            &raw mut U_OPTIMUS_NODE_SET_GRAPH_POSITION,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetNodeName"),
            &raw mut U_OPTIMUS_NODE_GET_NODE_NAME,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetNodeCategory"),
            &raw mut U_OPTIMUS_NODE_GET_NODE_CATEGORY,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetGraphPosition"),
            &raw mut U_OPTIMUS_NODE_GET_GRAPH_POSITION,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetDisplayName"),
            &raw mut U_OPTIMUS_NODE_GET_DISPLAY_NAME,
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
            &raw mut U_OPTIMUS_DEFORMER_GET_VARIABLES,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetResources"),
            &raw mut U_OPTIMUS_DEFORMER_GET_RESOURCES,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetPrimaryComponentBinding"),
            &raw mut U_OPTIMUS_DEFORMER_GET_PRIMARY_COMPONENT_BINDING,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetComponentBindings"),
            &raw mut U_OPTIMUS_DEFORMER_GET_COMPONENT_BINDINGS,
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
            &raw mut U_OPTIMUS_DEFORMER_INSTANCE_SET_VECTOR_VARIABLE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetVectorArrayVariable"),
            &raw mut U_OPTIMUS_DEFORMER_INSTANCE_SET_VECTOR_ARRAY_VARIABLE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetVector4Variable"),
            &raw mut U_OPTIMUS_DEFORMER_INSTANCE_SET_VECTOR4_VARIABLE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetVector4ArrayVariable"),
            &raw mut U_OPTIMUS_DEFORMER_INSTANCE_SET_VECTOR4_ARRAY_VARIABLE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetVector2Variable"),
            &raw mut U_OPTIMUS_DEFORMER_INSTANCE_SET_VECTOR2_VARIABLE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetVector2ArrayVariable"),
            &raw mut U_OPTIMUS_DEFORMER_INSTANCE_SET_VECTOR2_ARRAY_VARIABLE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetTransformVariable"),
            &raw mut U_OPTIMUS_DEFORMER_INSTANCE_SET_TRANSFORM_VARIABLE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetTransformArrayVariable"),
            &raw mut U_OPTIMUS_DEFORMER_INSTANCE_SET_TRANSFORM_ARRAY_VARIABLE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetRotatorVariable"),
            &raw mut U_OPTIMUS_DEFORMER_INSTANCE_SET_ROTATOR_VARIABLE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetRotatorArrayVariable"),
            &raw mut U_OPTIMUS_DEFORMER_INSTANCE_SET_ROTATOR_ARRAY_VARIABLE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetQuatVariable"),
            &raw mut U_OPTIMUS_DEFORMER_INSTANCE_SET_QUAT_VARIABLE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetQuatArrayVariable"),
            &raw mut U_OPTIMUS_DEFORMER_INSTANCE_SET_QUAT_ARRAY_VARIABLE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetNameVariable"),
            &raw mut U_OPTIMUS_DEFORMER_INSTANCE_SET_NAME_VARIABLE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetNameArrayVariable"),
            &raw mut U_OPTIMUS_DEFORMER_INSTANCE_SET_NAME_ARRAY_VARIABLE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetLinearColorVariable"),
            &raw mut U_OPTIMUS_DEFORMER_INSTANCE_SET_LINEAR_COLOR_VARIABLE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetLinearColorArrayVariable"),
            &raw mut U_OPTIMUS_DEFORMER_INSTANCE_SET_LINEAR_COLOR_ARRAY_VARIABLE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetIntVariable"),
            &raw mut U_OPTIMUS_DEFORMER_INSTANCE_SET_INT_VARIABLE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetIntArrayVariable"),
            &raw mut U_OPTIMUS_DEFORMER_INSTANCE_SET_INT_ARRAY_VARIABLE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetInt4Variable"),
            &raw mut U_OPTIMUS_DEFORMER_INSTANCE_SET_INT4_VARIABLE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetInt4ArrayVariable"),
            &raw mut U_OPTIMUS_DEFORMER_INSTANCE_SET_INT4_ARRAY_VARIABLE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetInt3Variable"),
            &raw mut U_OPTIMUS_DEFORMER_INSTANCE_SET_INT3_VARIABLE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetInt3ArrayVariable"),
            &raw mut U_OPTIMUS_DEFORMER_INSTANCE_SET_INT3_ARRAY_VARIABLE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetInt2Variable"),
            &raw mut U_OPTIMUS_DEFORMER_INSTANCE_SET_INT2_VARIABLE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetInt2ArrayVariable"),
            &raw mut U_OPTIMUS_DEFORMER_INSTANCE_SET_INT2_ARRAY_VARIABLE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetFloatVariable"),
            &raw mut U_OPTIMUS_DEFORMER_INSTANCE_SET_FLOAT_VARIABLE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetFloatArrayVariable"),
            &raw mut U_OPTIMUS_DEFORMER_INSTANCE_SET_FLOAT_ARRAY_VARIABLE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetBoolVariable"),
            &raw mut U_OPTIMUS_DEFORMER_INSTANCE_SET_BOOL_VARIABLE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetBoolArrayVariable"),
            &raw mut U_OPTIMUS_DEFORMER_INSTANCE_SET_BOOL_ARRAY_VARIABLE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("EnqueueTriggerGraph"),
            &raw mut U_OPTIMUS_DEFORMER_INSTANCE_ENQUEUE_TRIGGER_GRAPH,
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
            &raw mut U_OPTIMUS_NODE_GRAPH_RENAME_GRAPH_DIRECT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RenameGraph"),
            &raw mut U_OPTIMUS_NODE_GRAPH_RENAME_GRAPH,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RemoveNodes"),
            &raw mut U_OPTIMUS_NODE_GRAPH_REMOVE_NODES,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RemoveNode"),
            &raw mut U_OPTIMUS_NODE_GRAPH_REMOVE_NODE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RemoveLink"),
            &raw mut U_OPTIMUS_NODE_GRAPH_REMOVE_LINK,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RemoveAllLinks"),
            &raw mut U_OPTIMUS_NODE_GRAPH_REMOVE_ALL_LINKS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("MoveGraphDirect"),
            &raw mut U_OPTIMUS_NODE_GRAPH_MOVE_GRAPH_DIRECT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("IsSubGraphReference"),
            &raw mut U_OPTIMUS_NODE_GRAPH_IS_SUB_GRAPH_REFERENCE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("IsKernelFunction"),
            &raw mut U_OPTIMUS_NODE_GRAPH_IS_KERNEL_FUNCTION,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("IsFunctionReference"),
            &raw mut U_OPTIMUS_NODE_GRAPH_IS_FUNCTION_REFERENCE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("IsFunctionGraph"),
            &raw mut U_OPTIMUS_NODE_GRAPH_IS_FUNCTION_GRAPH,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("IsExecutionGraph"),
            &raw mut U_OPTIMUS_NODE_GRAPH_IS_EXECUTION_GRAPH,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("IsCustomKernel"),
            &raw mut U_OPTIMUS_NODE_GRAPH_IS_CUSTOM_KERNEL,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetGraphType"),
            &raw mut U_OPTIMUS_NODE_GRAPH_GET_GRAPH_TYPE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetGraphs"),
            &raw mut U_OPTIMUS_NODE_GRAPH_GET_GRAPHS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetGraphIndex"),
            &raw mut U_OPTIMUS_NODE_GRAPH_GET_GRAPH_INDEX,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ExpandCollapsedNodes"),
            &raw mut U_OPTIMUS_NODE_GRAPH_EXPAND_COLLAPSED_NODES,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("DuplicateNodes"),
            &raw mut U_OPTIMUS_NODE_GRAPH_DUPLICATE_NODES,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("DuplicateNode"),
            &raw mut U_OPTIMUS_NODE_GRAPH_DUPLICATE_NODE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ConvertToSubGraph"),
            &raw mut U_OPTIMUS_NODE_GRAPH_CONVERT_TO_SUB_GRAPH,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ConvertToFunction"),
            &raw mut U_OPTIMUS_NODE_GRAPH_CONVERT_TO_FUNCTION,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ConvertFunctionToCustomKernel"),
            &raw mut U_OPTIMUS_NODE_GRAPH_CONVERT_FUNCTION_TO_CUSTOM_KERNEL,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ConvertCustomKernelToFunction"),
            &raw mut U_OPTIMUS_NODE_GRAPH_CONVERT_CUSTOM_KERNEL_TO_FUNCTION,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CollapseNodesToSubGraph"),
            &raw mut U_OPTIMUS_NODE_GRAPH_COLLAPSE_NODES_TO_SUB_GRAPH,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CollapseNodesToFunction"),
            &raw mut U_OPTIMUS_NODE_GRAPH_COLLAPSE_NODES_TO_FUNCTION,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AddVariableGetNode"),
            &raw mut U_OPTIMUS_NODE_GRAPH_ADD_VARIABLE_GET_NODE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AddValueNode"),
            &raw mut U_OPTIMUS_NODE_GRAPH_ADD_VALUE_NODE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AddResourceSetNode"),
            &raw mut U_OPTIMUS_NODE_GRAPH_ADD_RESOURCE_SET_NODE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AddResourceNode"),
            &raw mut U_OPTIMUS_NODE_GRAPH_ADD_RESOURCE_NODE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AddResourceGetNode"),
            &raw mut U_OPTIMUS_NODE_GRAPH_ADD_RESOURCE_GET_NODE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AddNode"),
            &raw mut U_OPTIMUS_NODE_GRAPH_ADD_NODE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AddLoopTerminalNodes"),
            &raw mut U_OPTIMUS_NODE_GRAPH_ADD_LOOP_TERMINAL_NODES,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AddLink"),
            &raw mut U_OPTIMUS_NODE_GRAPH_ADD_LINK,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AddFunctionReferenceNode"),
            &raw mut U_OPTIMUS_NODE_GRAPH_ADD_FUNCTION_REFERENCE_NODE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AddDataInterfaceNode"),
            &raw mut U_OPTIMUS_NODE_GRAPH_ADD_DATA_INTERFACE_NODE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AddComponentBindingGetNode"),
            &raw mut U_OPTIMUS_NODE_GRAPH_ADD_COMPONENT_BINDING_GET_NODE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AddCommentNode"),
            &raw mut U_OPTIMUS_NODE_GRAPH_ADD_COMMENT_NODE,
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
            &raw mut U_OPTIMUS_FUNCTION_NODE_GRAPH_GET_ACCESS_SPECIFIER_OPTIONS,
        );
    }
}
#[repr(C, align(8))]
pub struct FOptimusDataTypeRef {
    __padding_end: [u8; 64],
}
impl FOptimusDataTypeRef {}
#[repr(C, align(8))]
pub struct FRigVMTrait_OptimusDeformerSettings {
    #[doc(hidden)]
    __padding_24: [u8; 24],
    pub execution_phase: EOptimusDeformerExecutionPhase,
    pub execution_group: i32,
    pub deform_child_components: bool,
    pub exclude_child_components_with_tag: FName,
}
impl FRigVMTrait_OptimusDeformerSettings {}
#[repr(C, align(8))]
pub struct FRigUnit_AddOptimusDeformer {
    __padding_end: [u8; 32],
}
impl FRigUnit_AddOptimusDeformer {}
#[repr(C, align(8))]
pub struct FRigVMTrait_SetDeformerIntVariable {
    __padding_end: [u8; 32],
}
impl FRigVMTrait_SetDeformerIntVariable {}
#[repr(C, align(8))]
pub struct FRigVMTrait_SetDeformerIntArrayVariable {
    __padding_end: [u8; 40],
}
impl FRigVMTrait_SetDeformerIntArrayVariable {}
#[repr(C, align(8))]
pub struct FRigVMTrait_SetDeformerInt2Variable {
    __padding_end: [u8; 32],
}
impl FRigVMTrait_SetDeformerInt2Variable {}
#[repr(C, align(8))]
pub struct FRigVMTrait_SetDeformerInt2ArrayVariable {
    __padding_end: [u8; 40],
}
impl FRigVMTrait_SetDeformerInt2ArrayVariable {}
#[repr(C, align(8))]
pub struct FRigVMTrait_SetDeformerInt3Variable {
    __padding_end: [u8; 40],
}
impl FRigVMTrait_SetDeformerInt3Variable {}
#[repr(C, align(8))]
pub struct FRigVMTrait_SetDeformerInt3ArrayVariable {
    __padding_end: [u8; 40],
}
impl FRigVMTrait_SetDeformerInt3ArrayVariable {}
#[repr(C, align(8))]
pub struct FRigVMTrait_SetDeformerInt4Variable {
    __padding_end: [u8; 40],
}
impl FRigVMTrait_SetDeformerInt4Variable {}
#[repr(C, align(8))]
pub struct FRigVMTrait_SetDeformerInt4ArrayVariable {
    __padding_end: [u8; 40],
}
impl FRigVMTrait_SetDeformerInt4ArrayVariable {}
#[repr(C, align(8))]
pub struct FRigVMTrait_SetDeformerFloatVariable {
    __padding_end: [u8; 32],
}
impl FRigVMTrait_SetDeformerFloatVariable {}
#[repr(C, align(8))]
pub struct FRigVMTrait_SetDeformerFloatArrayVariable {
    __padding_end: [u8; 40],
}
impl FRigVMTrait_SetDeformerFloatArrayVariable {}
#[repr(C, align(8))]
pub struct FRigVMTrait_SetDeformerVector2Variable {
    __padding_end: [u8; 40],
}
impl FRigVMTrait_SetDeformerVector2Variable {}
#[repr(C, align(8))]
pub struct FRigVMTrait_SetDeformerVector2ArrayVariable {
    __padding_end: [u8; 40],
}
impl FRigVMTrait_SetDeformerVector2ArrayVariable {}
#[repr(C, align(8))]
pub struct FRigVMTrait_SetDeformerVectorVariable {
    __padding_end: [u8; 48],
}
impl FRigVMTrait_SetDeformerVectorVariable {}
#[repr(C, align(8))]
pub struct FRigVMTrait_SetDeformerVectorArrayVariable {
    __padding_end: [u8; 40],
}
impl FRigVMTrait_SetDeformerVectorArrayVariable {}
#[repr(C, align(16))]
pub struct FRigVMTrait_SetDeformerVector4Variable {
    __padding_end: [u8; 64],
}
impl FRigVMTrait_SetDeformerVector4Variable {}
#[repr(C, align(8))]
pub struct FRigVMTrait_SetDeformerVector4ArrayVariable {
    __padding_end: [u8; 40],
}
impl FRigVMTrait_SetDeformerVector4ArrayVariable {}
#[repr(C, align(8))]
pub struct FRigVMTrait_SetDeformerLinearColorVariable {
    __padding_end: [u8; 40],
}
impl FRigVMTrait_SetDeformerLinearColorVariable {}
#[repr(C, align(8))]
pub struct FRigVMTrait_SetDeformerLinearColorArrayVariable {
    __padding_end: [u8; 40],
}
impl FRigVMTrait_SetDeformerLinearColorArrayVariable {}
#[repr(C, align(16))]
pub struct FRigVMTrait_SetDeformerQuatVariable {
    __padding_end: [u8; 64],
}
impl FRigVMTrait_SetDeformerQuatVariable {}
#[repr(C, align(8))]
pub struct FRigVMTrait_SetDeformerQuatArrayVariable {
    __padding_end: [u8; 40],
}
impl FRigVMTrait_SetDeformerQuatArrayVariable {}
#[repr(C, align(8))]
pub struct FRigVMTrait_SetDeformerRotatorVariable {
    __padding_end: [u8; 48],
}
impl FRigVMTrait_SetDeformerRotatorVariable {}
#[repr(C, align(8))]
pub struct FRigVMTrait_SetDeformerRotatorArrayVariable {
    __padding_end: [u8; 40],
}
impl FRigVMTrait_SetDeformerRotatorArrayVariable {}
#[repr(C, align(16))]
pub struct FRigVMTrait_SetDeformerTransformVariable {
    __padding_end: [u8; 128],
}
impl FRigVMTrait_SetDeformerTransformVariable {}
#[repr(C, align(8))]
pub struct FRigVMTrait_SetDeformerTransformArrayVariable {
    __padding_end: [u8; 40],
}
impl FRigVMTrait_SetDeformerTransformArrayVariable {}
#[repr(C, align(8))]
pub struct FRigVMTrait_SetDeformerNameVariable {
    __padding_end: [u8; 40],
}
impl FRigVMTrait_SetDeformerNameVariable {}
#[repr(C, align(8))]
pub struct FRigVMTrait_SetDeformerNameArrayVariable {
    __padding_end: [u8; 40],
}
impl FRigVMTrait_SetDeformerNameArrayVariable {}
#[repr(C, align(8))]
pub struct FRigVMTrait_SetDeformerBoolVariable {
    __padding_end: [u8; 32],
}
impl FRigVMTrait_SetDeformerBoolVariable {}
#[repr(C, align(8))]
pub struct FRigVMTrait_SetDeformerBoolArrayVariable {
    __padding_end: [u8; 40],
}
impl FRigVMTrait_SetDeformerBoolArrayVariable {}
#[repr(C, align(4))]
pub struct FOptimusDeformerInstanceComponentBinding {
    __padding_end: [u8; 24],
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
    __padding_64: [u8; 64],
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
}
#[repr(C, align(8))]
pub struct UOptimusNodeGraph {
    #[doc(hidden)]
    __padding_56: [u8; 56],
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
    __padding_48: [u8; 48],
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
