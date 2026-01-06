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
pub static mut U_MATERIAL_EDITING_LIBRARY_UPDATE_MATERIAL_INSTANCE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MATERIAL_EDITING_LIBRARY_UPDATE_MATERIAL_FUNCTION: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MATERIAL_EDITING_LIBRARY_SET_MATERIAL_USAGE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MATERIAL_EDITING_LIBRARY_SET_MATERIAL_INSTANCE_VECTOR_PARAMETER_VALUE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MATERIAL_EDITING_LIBRARY_SET_MATERIAL_INSTANCE_TEXTURE_PARAMETER_VALUE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MATERIAL_EDITING_LIBRARY_SET_MATERIAL_INSTANCE_STATIC_SWITCH_PARAMETER_VALUE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MATERIAL_EDITING_LIBRARY_SET_MATERIAL_INSTANCE_SPARSE_VOLUME_TEXTURE_PARAMETER_VALUE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MATERIAL_EDITING_LIBRARY_SET_MATERIAL_INSTANCE_SCALAR_PARAMETER_VALUE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MATERIAL_EDITING_LIBRARY_SET_MATERIAL_INSTANCE_RUNTIME_VIRTUAL_TEXTURE_PARAMETER_VALUE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MATERIAL_EDITING_LIBRARY_SET_MATERIAL_INSTANCE_PARENT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MATERIAL_EDITING_LIBRARY_RENAME_MATERIAL_PARAMETER_GROUP: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MATERIAL_EDITING_LIBRARY_RENAME_MATERIAL_FUNCTION_PARAMETER_GROUP: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MATERIAL_EDITING_LIBRARY_RECOMPILE_MATERIAL: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MATERIAL_EDITING_LIBRARY_LAYOUT_MATERIAL_FUNCTION_EXPRESSIONS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MATERIAL_EDITING_LIBRARY_LAYOUT_MATERIAL_EXPRESSIONS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MATERIAL_EDITING_LIBRARY_HAS_MATERIAL_USAGE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MATERIAL_EDITING_LIBRARY_GET_VECTOR_PARAMETER_SOURCE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MATERIAL_EDITING_LIBRARY_GET_VECTOR_PARAMETER_NAMES: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MATERIAL_EDITING_LIBRARY_GET_USED_TEXTURES: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MATERIAL_EDITING_LIBRARY_GET_TEXTURE_PARAMETER_SOURCE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MATERIAL_EDITING_LIBRARY_GET_TEXTURE_PARAMETER_NAMES: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MATERIAL_EDITING_LIBRARY_GET_STATISTICS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MATERIAL_EDITING_LIBRARY_GET_STATIC_SWITCH_PARAMETER_SOURCE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MATERIAL_EDITING_LIBRARY_GET_STATIC_SWITCH_PARAMETER_NAMES: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MATERIAL_EDITING_LIBRARY_GET_SCALAR_PARAMETER_SOURCE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MATERIAL_EDITING_LIBRARY_GET_SCALAR_PARAMETER_NAMES: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MATERIAL_EDITING_LIBRARY_GET_NUM_MATERIAL_EXPRESSIONS_IN_FUNCTION: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MATERIAL_EDITING_LIBRARY_GET_NUM_MATERIAL_EXPRESSIONS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MATERIAL_EDITING_LIBRARY_GET_NANITE_OVERRIDE_MATERIAL: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MATERIAL_EDITING_LIBRARY_GET_MATERIAL_SELECTED_NODES: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MATERIAL_EDITING_LIBRARY_GET_MATERIAL_PROPERTY_INPUT_NODE_OUTPUT_NAME: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MATERIAL_EDITING_LIBRARY_GET_MATERIAL_PROPERTY_INPUT_NODE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MATERIAL_EDITING_LIBRARY_GET_MATERIAL_INSTANCE_VECTOR_PARAMETER_VALUE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MATERIAL_EDITING_LIBRARY_GET_MATERIAL_INSTANCE_TEXTURE_PARAMETER_VALUE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MATERIAL_EDITING_LIBRARY_GET_MATERIAL_INSTANCE_STATIC_SWITCH_PARAMETER_VALUE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MATERIAL_EDITING_LIBRARY_GET_MATERIAL_INSTANCE_SPARSE_VOLUME_TEXTURE_PARAMETER_VALUE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MATERIAL_EDITING_LIBRARY_GET_MATERIAL_INSTANCE_SCALAR_PARAMETER_VALUE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MATERIAL_EDITING_LIBRARY_GET_MATERIAL_INSTANCE_RUNTIME_VIRTUAL_TEXTURE_PARAMETER_VALUE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MATERIAL_EDITING_LIBRARY_GET_MATERIAL_EXPRESSION_NODE_POSITION: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MATERIAL_EDITING_LIBRARY_GET_MATERIAL_EXPRESSION_INPUT_TYPES: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MATERIAL_EDITING_LIBRARY_GET_MATERIAL_EXPRESSION_INPUT_NAMES: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MATERIAL_EDITING_LIBRARY_GET_MATERIAL_DEFAULT_VECTOR_PARAMETER_VALUE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MATERIAL_EDITING_LIBRARY_GET_MATERIAL_DEFAULT_TEXTURE_PARAMETER_VALUE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MATERIAL_EDITING_LIBRARY_GET_MATERIAL_DEFAULT_STATIC_SWITCH_PARAMETER_VALUE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MATERIAL_EDITING_LIBRARY_GET_MATERIAL_DEFAULT_SCALAR_PARAMETER_VALUE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MATERIAL_EDITING_LIBRARY_GET_INPUTS_FOR_MATERIAL_EXPRESSION: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MATERIAL_EDITING_LIBRARY_GET_INPUT_NODE_OUTPUT_NAME_FOR_MATERIAL_EXPRESSION: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MATERIAL_EDITING_LIBRARY_GET_CHILD_INSTANCES: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MATERIAL_EDITING_LIBRARY_DUPLICATE_MATERIAL_EXPRESSION: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MATERIAL_EDITING_LIBRARY_DELETE_MATERIAL_EXPRESSION_IN_FUNCTION: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MATERIAL_EDITING_LIBRARY_DELETE_MATERIAL_EXPRESSION: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MATERIAL_EDITING_LIBRARY_DELETE_ALL_MATERIAL_EXPRESSIONS_IN_FUNCTION: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MATERIAL_EDITING_LIBRARY_DELETE_ALL_MATERIAL_EXPRESSIONS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MATERIAL_EDITING_LIBRARY_CREATE_MATERIAL_EXPRESSION_IN_FUNCTION: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MATERIAL_EDITING_LIBRARY_CREATE_MATERIAL_EXPRESSION: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MATERIAL_EDITING_LIBRARY_CONNECT_MATERIAL_PROPERTY: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MATERIAL_EDITING_LIBRARY_CONNECT_MATERIAL_EXPRESSIONS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MATERIAL_EDITING_LIBRARY_CLEAR_ALL_MATERIAL_INSTANCE_PARAMETERS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
pub fn initialize() {
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UMaterialEditingLibrary::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("UpdateMaterialInstance"),
            &raw mut U_MATERIAL_EDITING_LIBRARY_UPDATE_MATERIAL_INSTANCE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("UpdateMaterialFunction"),
            &raw mut U_MATERIAL_EDITING_LIBRARY_UPDATE_MATERIAL_FUNCTION,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetMaterialUsage"),
            &raw mut U_MATERIAL_EDITING_LIBRARY_SET_MATERIAL_USAGE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetMaterialInstanceVectorParameterValue"),
            &raw mut U_MATERIAL_EDITING_LIBRARY_SET_MATERIAL_INSTANCE_VECTOR_PARAMETER_VALUE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetMaterialInstanceTextureParameterValue"),
            &raw mut U_MATERIAL_EDITING_LIBRARY_SET_MATERIAL_INSTANCE_TEXTURE_PARAMETER_VALUE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetMaterialInstanceStaticSwitchParameterValue"),
            &raw mut U_MATERIAL_EDITING_LIBRARY_SET_MATERIAL_INSTANCE_STATIC_SWITCH_PARAMETER_VALUE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from(
                "SetMaterialInstanceSparseVolumeTextureParameterValue",
            ),
            &raw mut U_MATERIAL_EDITING_LIBRARY_SET_MATERIAL_INSTANCE_SPARSE_VOLUME_TEXTURE_PARAMETER_VALUE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetMaterialInstanceScalarParameterValue"),
            &raw mut U_MATERIAL_EDITING_LIBRARY_SET_MATERIAL_INSTANCE_SCALAR_PARAMETER_VALUE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from(
                "SetMaterialInstanceRuntimeVirtualTextureParameterValue",
            ),
            &raw mut U_MATERIAL_EDITING_LIBRARY_SET_MATERIAL_INSTANCE_RUNTIME_VIRTUAL_TEXTURE_PARAMETER_VALUE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetMaterialInstanceParent"),
            &raw mut U_MATERIAL_EDITING_LIBRARY_SET_MATERIAL_INSTANCE_PARENT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RenameMaterialParameterGroup"),
            &raw mut U_MATERIAL_EDITING_LIBRARY_RENAME_MATERIAL_PARAMETER_GROUP,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RenameMaterialFunctionParameterGroup"),
            &raw mut U_MATERIAL_EDITING_LIBRARY_RENAME_MATERIAL_FUNCTION_PARAMETER_GROUP,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RecompileMaterial"),
            &raw mut U_MATERIAL_EDITING_LIBRARY_RECOMPILE_MATERIAL,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("LayoutMaterialFunctionExpressions"),
            &raw mut U_MATERIAL_EDITING_LIBRARY_LAYOUT_MATERIAL_FUNCTION_EXPRESSIONS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("LayoutMaterialExpressions"),
            &raw mut U_MATERIAL_EDITING_LIBRARY_LAYOUT_MATERIAL_EXPRESSIONS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("HasMaterialUsage"),
            &raw mut U_MATERIAL_EDITING_LIBRARY_HAS_MATERIAL_USAGE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetVectorParameterSource"),
            &raw mut U_MATERIAL_EDITING_LIBRARY_GET_VECTOR_PARAMETER_SOURCE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetVectorParameterNames"),
            &raw mut U_MATERIAL_EDITING_LIBRARY_GET_VECTOR_PARAMETER_NAMES,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetUsedTextures"),
            &raw mut U_MATERIAL_EDITING_LIBRARY_GET_USED_TEXTURES,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetTextureParameterSource"),
            &raw mut U_MATERIAL_EDITING_LIBRARY_GET_TEXTURE_PARAMETER_SOURCE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetTextureParameterNames"),
            &raw mut U_MATERIAL_EDITING_LIBRARY_GET_TEXTURE_PARAMETER_NAMES,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetStatistics"),
            &raw mut U_MATERIAL_EDITING_LIBRARY_GET_STATISTICS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetStaticSwitchParameterSource"),
            &raw mut U_MATERIAL_EDITING_LIBRARY_GET_STATIC_SWITCH_PARAMETER_SOURCE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetStaticSwitchParameterNames"),
            &raw mut U_MATERIAL_EDITING_LIBRARY_GET_STATIC_SWITCH_PARAMETER_NAMES,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetScalarParameterSource"),
            &raw mut U_MATERIAL_EDITING_LIBRARY_GET_SCALAR_PARAMETER_SOURCE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetScalarParameterNames"),
            &raw mut U_MATERIAL_EDITING_LIBRARY_GET_SCALAR_PARAMETER_NAMES,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetNumMaterialExpressionsInFunction"),
            &raw mut U_MATERIAL_EDITING_LIBRARY_GET_NUM_MATERIAL_EXPRESSIONS_IN_FUNCTION,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetNumMaterialExpressions"),
            &raw mut U_MATERIAL_EDITING_LIBRARY_GET_NUM_MATERIAL_EXPRESSIONS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetNaniteOverrideMaterial"),
            &raw mut U_MATERIAL_EDITING_LIBRARY_GET_NANITE_OVERRIDE_MATERIAL,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetMaterialSelectedNodes"),
            &raw mut U_MATERIAL_EDITING_LIBRARY_GET_MATERIAL_SELECTED_NODES,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetMaterialPropertyInputNodeOutputName"),
            &raw mut U_MATERIAL_EDITING_LIBRARY_GET_MATERIAL_PROPERTY_INPUT_NODE_OUTPUT_NAME,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetMaterialPropertyInputNode"),
            &raw mut U_MATERIAL_EDITING_LIBRARY_GET_MATERIAL_PROPERTY_INPUT_NODE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetMaterialInstanceVectorParameterValue"),
            &raw mut U_MATERIAL_EDITING_LIBRARY_GET_MATERIAL_INSTANCE_VECTOR_PARAMETER_VALUE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetMaterialInstanceTextureParameterValue"),
            &raw mut U_MATERIAL_EDITING_LIBRARY_GET_MATERIAL_INSTANCE_TEXTURE_PARAMETER_VALUE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetMaterialInstanceStaticSwitchParameterValue"),
            &raw mut U_MATERIAL_EDITING_LIBRARY_GET_MATERIAL_INSTANCE_STATIC_SWITCH_PARAMETER_VALUE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from(
                "GetMaterialInstanceSparseVolumeTextureParameterValue",
            ),
            &raw mut U_MATERIAL_EDITING_LIBRARY_GET_MATERIAL_INSTANCE_SPARSE_VOLUME_TEXTURE_PARAMETER_VALUE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetMaterialInstanceScalarParameterValue"),
            &raw mut U_MATERIAL_EDITING_LIBRARY_GET_MATERIAL_INSTANCE_SCALAR_PARAMETER_VALUE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from(
                "GetMaterialInstanceRuntimeVirtualTextureParameterValue",
            ),
            &raw mut U_MATERIAL_EDITING_LIBRARY_GET_MATERIAL_INSTANCE_RUNTIME_VIRTUAL_TEXTURE_PARAMETER_VALUE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetMaterialExpressionNodePosition"),
            &raw mut U_MATERIAL_EDITING_LIBRARY_GET_MATERIAL_EXPRESSION_NODE_POSITION,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetMaterialExpressionInputTypes"),
            &raw mut U_MATERIAL_EDITING_LIBRARY_GET_MATERIAL_EXPRESSION_INPUT_TYPES,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetMaterialExpressionInputNames"),
            &raw mut U_MATERIAL_EDITING_LIBRARY_GET_MATERIAL_EXPRESSION_INPUT_NAMES,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetMaterialDefaultVectorParameterValue"),
            &raw mut U_MATERIAL_EDITING_LIBRARY_GET_MATERIAL_DEFAULT_VECTOR_PARAMETER_VALUE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetMaterialDefaultTextureParameterValue"),
            &raw mut U_MATERIAL_EDITING_LIBRARY_GET_MATERIAL_DEFAULT_TEXTURE_PARAMETER_VALUE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetMaterialDefaultStaticSwitchParameterValue"),
            &raw mut U_MATERIAL_EDITING_LIBRARY_GET_MATERIAL_DEFAULT_STATIC_SWITCH_PARAMETER_VALUE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetMaterialDefaultScalarParameterValue"),
            &raw mut U_MATERIAL_EDITING_LIBRARY_GET_MATERIAL_DEFAULT_SCALAR_PARAMETER_VALUE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetInputsForMaterialExpression"),
            &raw mut U_MATERIAL_EDITING_LIBRARY_GET_INPUTS_FOR_MATERIAL_EXPRESSION,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetInputNodeOutputNameForMaterialExpression"),
            &raw mut U_MATERIAL_EDITING_LIBRARY_GET_INPUT_NODE_OUTPUT_NAME_FOR_MATERIAL_EXPRESSION,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetChildInstances"),
            &raw mut U_MATERIAL_EDITING_LIBRARY_GET_CHILD_INSTANCES,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("DuplicateMaterialExpression"),
            &raw mut U_MATERIAL_EDITING_LIBRARY_DUPLICATE_MATERIAL_EXPRESSION,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("DeleteMaterialExpressionInFunction"),
            &raw mut U_MATERIAL_EDITING_LIBRARY_DELETE_MATERIAL_EXPRESSION_IN_FUNCTION,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("DeleteMaterialExpression"),
            &raw mut U_MATERIAL_EDITING_LIBRARY_DELETE_MATERIAL_EXPRESSION,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("DeleteAllMaterialExpressionsInFunction"),
            &raw mut U_MATERIAL_EDITING_LIBRARY_DELETE_ALL_MATERIAL_EXPRESSIONS_IN_FUNCTION,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("DeleteAllMaterialExpressions"),
            &raw mut U_MATERIAL_EDITING_LIBRARY_DELETE_ALL_MATERIAL_EXPRESSIONS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CreateMaterialExpressionInFunction"),
            &raw mut U_MATERIAL_EDITING_LIBRARY_CREATE_MATERIAL_EXPRESSION_IN_FUNCTION,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CreateMaterialExpression"),
            &raw mut U_MATERIAL_EDITING_LIBRARY_CREATE_MATERIAL_EXPRESSION,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ConnectMaterialProperty"),
            &raw mut U_MATERIAL_EDITING_LIBRARY_CONNECT_MATERIAL_PROPERTY,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ConnectMaterialExpressions"),
            &raw mut U_MATERIAL_EDITING_LIBRARY_CONNECT_MATERIAL_EXPRESSIONS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ClearAllMaterialInstanceParameters"),
            &raw mut U_MATERIAL_EDITING_LIBRARY_CLEAR_ALL_MATERIAL_INSTANCE_PARAMETERS,
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
