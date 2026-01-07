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
                crate::bindings::material_editor::U_MATERIAL_EDITING_LIBRARY_UPDATE_MATERIAL_INSTANCE,
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
                crate::bindings::material_editor::U_MATERIAL_EDITING_LIBRARY_UPDATE_MATERIAL_INSTANCE,
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
                crate::bindings::material_editor::U_MATERIAL_EDITING_LIBRARY_UPDATE_MATERIAL_FUNCTION,
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
                crate::bindings::material_editor::U_MATERIAL_EDITING_LIBRARY_UPDATE_MATERIAL_FUNCTION,
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
                crate::bindings::material_editor::U_MATERIAL_EDITING_LIBRARY_SET_MATERIAL_USAGE,
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
                crate::bindings::material_editor::U_MATERIAL_EDITING_LIBRARY_SET_MATERIAL_USAGE,
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
                crate::bindings::material_editor::U_MATERIAL_EDITING_LIBRARY_SET_MATERIAL_INSTANCE_VECTOR_PARAMETER_VALUE,
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
                crate::bindings::material_editor::U_MATERIAL_EDITING_LIBRARY_SET_MATERIAL_INSTANCE_VECTOR_PARAMETER_VALUE,
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
                crate::bindings::material_editor::U_MATERIAL_EDITING_LIBRARY_SET_MATERIAL_INSTANCE_TEXTURE_PARAMETER_VALUE,
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
                crate::bindings::material_editor::U_MATERIAL_EDITING_LIBRARY_SET_MATERIAL_INSTANCE_TEXTURE_PARAMETER_VALUE,
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
                crate::bindings::material_editor::U_MATERIAL_EDITING_LIBRARY_SET_MATERIAL_INSTANCE_STATIC_SWITCH_PARAMETER_VALUE,
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
                crate::bindings::material_editor::U_MATERIAL_EDITING_LIBRARY_SET_MATERIAL_INSTANCE_STATIC_SWITCH_PARAMETER_VALUE,
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
                crate::bindings::material_editor::U_MATERIAL_EDITING_LIBRARY_SET_MATERIAL_INSTANCE_SPARSE_VOLUME_TEXTURE_PARAMETER_VALUE,
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
                crate::bindings::material_editor::U_MATERIAL_EDITING_LIBRARY_SET_MATERIAL_INSTANCE_SPARSE_VOLUME_TEXTURE_PARAMETER_VALUE,
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
                crate::bindings::material_editor::U_MATERIAL_EDITING_LIBRARY_SET_MATERIAL_INSTANCE_SCALAR_PARAMETER_VALUE,
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
                crate::bindings::material_editor::U_MATERIAL_EDITING_LIBRARY_SET_MATERIAL_INSTANCE_SCALAR_PARAMETER_VALUE,
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
                crate::bindings::material_editor::U_MATERIAL_EDITING_LIBRARY_SET_MATERIAL_INSTANCE_RUNTIME_VIRTUAL_TEXTURE_PARAMETER_VALUE,
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
                crate::bindings::material_editor::U_MATERIAL_EDITING_LIBRARY_SET_MATERIAL_INSTANCE_RUNTIME_VIRTUAL_TEXTURE_PARAMETER_VALUE,
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
                crate::bindings::material_editor::U_MATERIAL_EDITING_LIBRARY_SET_MATERIAL_INSTANCE_PARENT,
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
                crate::bindings::material_editor::U_MATERIAL_EDITING_LIBRARY_SET_MATERIAL_INSTANCE_PARENT,
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
                crate::bindings::material_editor::U_MATERIAL_EDITING_LIBRARY_RENAME_MATERIAL_PARAMETER_GROUP,
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
                crate::bindings::material_editor::U_MATERIAL_EDITING_LIBRARY_RENAME_MATERIAL_PARAMETER_GROUP,
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
                crate::bindings::material_editor::U_MATERIAL_EDITING_LIBRARY_RENAME_MATERIAL_FUNCTION_PARAMETER_GROUP,
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
                crate::bindings::material_editor::U_MATERIAL_EDITING_LIBRARY_RENAME_MATERIAL_FUNCTION_PARAMETER_GROUP,
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
                crate::bindings::material_editor::U_MATERIAL_EDITING_LIBRARY_RECOMPILE_MATERIAL,
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
                crate::bindings::material_editor::U_MATERIAL_EDITING_LIBRARY_RECOMPILE_MATERIAL,
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
                crate::bindings::material_editor::U_MATERIAL_EDITING_LIBRARY_LAYOUT_MATERIAL_FUNCTION_EXPRESSIONS,
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
                crate::bindings::material_editor::U_MATERIAL_EDITING_LIBRARY_LAYOUT_MATERIAL_FUNCTION_EXPRESSIONS,
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
                crate::bindings::material_editor::U_MATERIAL_EDITING_LIBRARY_LAYOUT_MATERIAL_EXPRESSIONS,
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
                crate::bindings::material_editor::U_MATERIAL_EDITING_LIBRARY_LAYOUT_MATERIAL_EXPRESSIONS,
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
                crate::bindings::material_editor::U_MATERIAL_EDITING_LIBRARY_HAS_MATERIAL_USAGE,
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
                crate::bindings::material_editor::U_MATERIAL_EDITING_LIBRARY_HAS_MATERIAL_USAGE,
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
                crate::bindings::material_editor::U_MATERIAL_EDITING_LIBRARY_GET_VECTOR_PARAMETER_SOURCE,
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
                crate::bindings::material_editor::U_MATERIAL_EDITING_LIBRARY_GET_VECTOR_PARAMETER_SOURCE,
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
                crate::bindings::material_editor::U_MATERIAL_EDITING_LIBRARY_GET_VECTOR_PARAMETER_NAMES,
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
                crate::bindings::material_editor::U_MATERIAL_EDITING_LIBRARY_GET_VECTOR_PARAMETER_NAMES,
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
                crate::bindings::material_editor::U_MATERIAL_EDITING_LIBRARY_GET_USED_TEXTURES,
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
                crate::bindings::material_editor::U_MATERIAL_EDITING_LIBRARY_GET_USED_TEXTURES,
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
                crate::bindings::material_editor::U_MATERIAL_EDITING_LIBRARY_GET_TEXTURE_PARAMETER_SOURCE,
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
                crate::bindings::material_editor::U_MATERIAL_EDITING_LIBRARY_GET_TEXTURE_PARAMETER_SOURCE,
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
                crate::bindings::material_editor::U_MATERIAL_EDITING_LIBRARY_GET_TEXTURE_PARAMETER_NAMES,
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
                crate::bindings::material_editor::U_MATERIAL_EDITING_LIBRARY_GET_TEXTURE_PARAMETER_NAMES,
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
                crate::bindings::material_editor::U_MATERIAL_EDITING_LIBRARY_GET_STATISTICS,
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
                crate::bindings::material_editor::U_MATERIAL_EDITING_LIBRARY_GET_STATISTICS,
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
                crate::bindings::material_editor::U_MATERIAL_EDITING_LIBRARY_GET_STATIC_SWITCH_PARAMETER_SOURCE,
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
                crate::bindings::material_editor::U_MATERIAL_EDITING_LIBRARY_GET_STATIC_SWITCH_PARAMETER_SOURCE,
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
                crate::bindings::material_editor::U_MATERIAL_EDITING_LIBRARY_GET_STATIC_SWITCH_PARAMETER_NAMES,
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
                crate::bindings::material_editor::U_MATERIAL_EDITING_LIBRARY_GET_STATIC_SWITCH_PARAMETER_NAMES,
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
                crate::bindings::material_editor::U_MATERIAL_EDITING_LIBRARY_GET_SCALAR_PARAMETER_SOURCE,
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
                crate::bindings::material_editor::U_MATERIAL_EDITING_LIBRARY_GET_SCALAR_PARAMETER_SOURCE,
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
                crate::bindings::material_editor::U_MATERIAL_EDITING_LIBRARY_GET_SCALAR_PARAMETER_NAMES,
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
                crate::bindings::material_editor::U_MATERIAL_EDITING_LIBRARY_GET_SCALAR_PARAMETER_NAMES,
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
                crate::bindings::material_editor::U_MATERIAL_EDITING_LIBRARY_GET_NUM_MATERIAL_EXPRESSIONS_IN_FUNCTION,
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
                crate::bindings::material_editor::U_MATERIAL_EDITING_LIBRARY_GET_NUM_MATERIAL_EXPRESSIONS_IN_FUNCTION,
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
                crate::bindings::material_editor::U_MATERIAL_EDITING_LIBRARY_GET_NUM_MATERIAL_EXPRESSIONS,
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
                crate::bindings::material_editor::U_MATERIAL_EDITING_LIBRARY_GET_NUM_MATERIAL_EXPRESSIONS,
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
                crate::bindings::material_editor::U_MATERIAL_EDITING_LIBRARY_GET_NANITE_OVERRIDE_MATERIAL,
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
                crate::bindings::material_editor::U_MATERIAL_EDITING_LIBRARY_GET_NANITE_OVERRIDE_MATERIAL,
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
                crate::bindings::material_editor::U_MATERIAL_EDITING_LIBRARY_GET_MATERIAL_SELECTED_NODES,
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
                crate::bindings::material_editor::U_MATERIAL_EDITING_LIBRARY_GET_MATERIAL_SELECTED_NODES,
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
                crate::bindings::material_editor::U_MATERIAL_EDITING_LIBRARY_GET_MATERIAL_PROPERTY_INPUT_NODE_OUTPUT_NAME,
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
                crate::bindings::material_editor::U_MATERIAL_EDITING_LIBRARY_GET_MATERIAL_PROPERTY_INPUT_NODE_OUTPUT_NAME,
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
                crate::bindings::material_editor::U_MATERIAL_EDITING_LIBRARY_GET_MATERIAL_PROPERTY_INPUT_NODE,
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
                crate::bindings::material_editor::U_MATERIAL_EDITING_LIBRARY_GET_MATERIAL_PROPERTY_INPUT_NODE,
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
                crate::bindings::material_editor::U_MATERIAL_EDITING_LIBRARY_GET_MATERIAL_INSTANCE_VECTOR_PARAMETER_VALUE,
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
                crate::bindings::material_editor::U_MATERIAL_EDITING_LIBRARY_GET_MATERIAL_INSTANCE_VECTOR_PARAMETER_VALUE,
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
                crate::bindings::material_editor::U_MATERIAL_EDITING_LIBRARY_GET_MATERIAL_INSTANCE_TEXTURE_PARAMETER_VALUE,
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
                crate::bindings::material_editor::U_MATERIAL_EDITING_LIBRARY_GET_MATERIAL_INSTANCE_TEXTURE_PARAMETER_VALUE,
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
                crate::bindings::material_editor::U_MATERIAL_EDITING_LIBRARY_GET_MATERIAL_INSTANCE_STATIC_SWITCH_PARAMETER_VALUE,
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
                crate::bindings::material_editor::U_MATERIAL_EDITING_LIBRARY_GET_MATERIAL_INSTANCE_STATIC_SWITCH_PARAMETER_VALUE,
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
                crate::bindings::material_editor::U_MATERIAL_EDITING_LIBRARY_GET_MATERIAL_INSTANCE_SPARSE_VOLUME_TEXTURE_PARAMETER_VALUE,
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
                crate::bindings::material_editor::U_MATERIAL_EDITING_LIBRARY_GET_MATERIAL_INSTANCE_SPARSE_VOLUME_TEXTURE_PARAMETER_VALUE,
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
                crate::bindings::material_editor::U_MATERIAL_EDITING_LIBRARY_GET_MATERIAL_INSTANCE_SCALAR_PARAMETER_VALUE,
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
                crate::bindings::material_editor::U_MATERIAL_EDITING_LIBRARY_GET_MATERIAL_INSTANCE_SCALAR_PARAMETER_VALUE,
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
                crate::bindings::material_editor::U_MATERIAL_EDITING_LIBRARY_GET_MATERIAL_INSTANCE_RUNTIME_VIRTUAL_TEXTURE_PARAMETER_VALUE,
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
                crate::bindings::material_editor::U_MATERIAL_EDITING_LIBRARY_GET_MATERIAL_INSTANCE_RUNTIME_VIRTUAL_TEXTURE_PARAMETER_VALUE,
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
                crate::bindings::material_editor::U_MATERIAL_EDITING_LIBRARY_GET_MATERIAL_EXPRESSION_NODE_POSITION,
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
                crate::bindings::material_editor::U_MATERIAL_EDITING_LIBRARY_GET_MATERIAL_EXPRESSION_NODE_POSITION,
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
                crate::bindings::material_editor::U_MATERIAL_EDITING_LIBRARY_GET_MATERIAL_EXPRESSION_INPUT_TYPES,
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
                crate::bindings::material_editor::U_MATERIAL_EDITING_LIBRARY_GET_MATERIAL_EXPRESSION_INPUT_TYPES,
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
                crate::bindings::material_editor::U_MATERIAL_EDITING_LIBRARY_GET_MATERIAL_EXPRESSION_INPUT_NAMES,
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
                crate::bindings::material_editor::U_MATERIAL_EDITING_LIBRARY_GET_MATERIAL_EXPRESSION_INPUT_NAMES,
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
                crate::bindings::material_editor::U_MATERIAL_EDITING_LIBRARY_GET_MATERIAL_DEFAULT_VECTOR_PARAMETER_VALUE,
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
                crate::bindings::material_editor::U_MATERIAL_EDITING_LIBRARY_GET_MATERIAL_DEFAULT_VECTOR_PARAMETER_VALUE,
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
                crate::bindings::material_editor::U_MATERIAL_EDITING_LIBRARY_GET_MATERIAL_DEFAULT_TEXTURE_PARAMETER_VALUE,
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
                crate::bindings::material_editor::U_MATERIAL_EDITING_LIBRARY_GET_MATERIAL_DEFAULT_TEXTURE_PARAMETER_VALUE,
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
                crate::bindings::material_editor::U_MATERIAL_EDITING_LIBRARY_GET_MATERIAL_DEFAULT_STATIC_SWITCH_PARAMETER_VALUE,
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
                crate::bindings::material_editor::U_MATERIAL_EDITING_LIBRARY_GET_MATERIAL_DEFAULT_STATIC_SWITCH_PARAMETER_VALUE,
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
                crate::bindings::material_editor::U_MATERIAL_EDITING_LIBRARY_GET_MATERIAL_DEFAULT_SCALAR_PARAMETER_VALUE,
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
                crate::bindings::material_editor::U_MATERIAL_EDITING_LIBRARY_GET_MATERIAL_DEFAULT_SCALAR_PARAMETER_VALUE,
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
                crate::bindings::material_editor::U_MATERIAL_EDITING_LIBRARY_GET_INPUTS_FOR_MATERIAL_EXPRESSION,
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
                crate::bindings::material_editor::U_MATERIAL_EDITING_LIBRARY_GET_INPUTS_FOR_MATERIAL_EXPRESSION,
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
                crate::bindings::material_editor::U_MATERIAL_EDITING_LIBRARY_GET_INPUT_NODE_OUTPUT_NAME_FOR_MATERIAL_EXPRESSION,
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
                crate::bindings::material_editor::U_MATERIAL_EDITING_LIBRARY_GET_INPUT_NODE_OUTPUT_NAME_FOR_MATERIAL_EXPRESSION,
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
                crate::bindings::material_editor::U_MATERIAL_EDITING_LIBRARY_GET_CHILD_INSTANCES,
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
                crate::bindings::material_editor::U_MATERIAL_EDITING_LIBRARY_GET_CHILD_INSTANCES,
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
                crate::bindings::material_editor::U_MATERIAL_EDITING_LIBRARY_DUPLICATE_MATERIAL_EXPRESSION,
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
                crate::bindings::material_editor::U_MATERIAL_EDITING_LIBRARY_DUPLICATE_MATERIAL_EXPRESSION,
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
                crate::bindings::material_editor::U_MATERIAL_EDITING_LIBRARY_DELETE_MATERIAL_EXPRESSION_IN_FUNCTION,
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
                crate::bindings::material_editor::U_MATERIAL_EDITING_LIBRARY_DELETE_MATERIAL_EXPRESSION_IN_FUNCTION,
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
                crate::bindings::material_editor::U_MATERIAL_EDITING_LIBRARY_DELETE_MATERIAL_EXPRESSION,
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
                crate::bindings::material_editor::U_MATERIAL_EDITING_LIBRARY_DELETE_MATERIAL_EXPRESSION,
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
                crate::bindings::material_editor::U_MATERIAL_EDITING_LIBRARY_DELETE_ALL_MATERIAL_EXPRESSIONS_IN_FUNCTION,
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
                crate::bindings::material_editor::U_MATERIAL_EDITING_LIBRARY_DELETE_ALL_MATERIAL_EXPRESSIONS_IN_FUNCTION,
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
                crate::bindings::material_editor::U_MATERIAL_EDITING_LIBRARY_DELETE_ALL_MATERIAL_EXPRESSIONS,
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
                crate::bindings::material_editor::U_MATERIAL_EDITING_LIBRARY_DELETE_ALL_MATERIAL_EXPRESSIONS,
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
                crate::bindings::material_editor::U_MATERIAL_EDITING_LIBRARY_CREATE_MATERIAL_EXPRESSION_IN_FUNCTION,
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
                crate::bindings::material_editor::U_MATERIAL_EDITING_LIBRARY_CREATE_MATERIAL_EXPRESSION_IN_FUNCTION,
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
                crate::bindings::material_editor::U_MATERIAL_EDITING_LIBRARY_CREATE_MATERIAL_EXPRESSION,
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
                crate::bindings::material_editor::U_MATERIAL_EDITING_LIBRARY_CREATE_MATERIAL_EXPRESSION,
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
                crate::bindings::material_editor::U_MATERIAL_EDITING_LIBRARY_CONNECT_MATERIAL_PROPERTY,
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
                crate::bindings::material_editor::U_MATERIAL_EDITING_LIBRARY_CONNECT_MATERIAL_PROPERTY,
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
                crate::bindings::material_editor::U_MATERIAL_EDITING_LIBRARY_CONNECT_MATERIAL_EXPRESSIONS,
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
                crate::bindings::material_editor::U_MATERIAL_EDITING_LIBRARY_CONNECT_MATERIAL_EXPRESSIONS,
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
                crate::bindings::material_editor::U_MATERIAL_EDITING_LIBRARY_CLEAR_ALL_MATERIAL_INSTANCE_PARAMETERS,
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
                crate::bindings::material_editor::U_MATERIAL_EDITING_LIBRARY_CLEAR_ALL_MATERIAL_INSTANCE_PARAMETERS,
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
