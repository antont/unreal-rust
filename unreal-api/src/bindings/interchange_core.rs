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
pub static mut U_INTERCHANGE_FACTORY_BASE_SET_SOURCE_FILENAME: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_INTERCHANGE_FACTORY_BASE_GET_SOURCE_FILENAMES: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_INTERCHANGE_FACTORY_BASE_GET_FACTORY_CLASS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_INTERCHANGE_FACTORY_BASE_GET_FACTORY_ASSET_TYPE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_INTERCHANGE_PIPELINE_BASE_SUPPORT_REIMPORT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_INTERCHANGE_PIPELINE_BASE_SCRIPTED_SET_REIMPORT_SOURCE_INDEX: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_INTERCHANGE_PIPELINE_BASE_SCRIPTED_GET_PIPELINE_DISPLAY_NAME: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_INTERCHANGE_PIPELINE_BASE_SCRIPTED_EXECUTE_POST_IMPORT_PIPELINE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_INTERCHANGE_PIPELINE_BASE_SCRIPTED_EXECUTE_POST_FACTORY_PIPELINE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_INTERCHANGE_PIPELINE_BASE_SCRIPTED_EXECUTE_POST_BROADCAST_PIPELINE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_INTERCHANGE_PIPELINE_BASE_SCRIPTED_EXECUTE_PIPELINE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_INTERCHANGE_PIPELINE_BASE_SCRIPTED_EXECUTE_EXPORT_PIPELINE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_INTERCHANGE_PIPELINE_BASE_IS_REIMPORT_CONTEXT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_INTERCHANGE_PIPELINE_BASE_GET_SUPPORT_ASSET_CLASSES: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_INTERCHANGE_PIPELINE_BASE_FIND_OR_ADD_PROPERTY_STATES: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_INTERCHANGE_PIPELINE_BASE_DOES_PROPERTY_STATES_EXIST: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_INTERCHANGE_SOURCE_DATA_SET_FILENAME: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_INTERCHANGE_SOURCE_DATA_SET_CONTEXT_OBJECT_BY_TAG: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_INTERCHANGE_SOURCE_DATA_REMOVE_ALL_CONTEXT_OBJECTS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_INTERCHANGE_SOURCE_DATA_GET_FILENAME: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_INTERCHANGE_SOURCE_DATA_GET_CONTEXT_OBJECT_BY_TAG: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_INTERCHANGE_SOURCE_DATA_GET_ALL_CONTEXT_OBJECT_TAGS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_INTERCHANGE_TRANSLATOR_SETTINGS_SAVE_SETTINGS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_INTERCHANGE_TRANSLATOR_SETTINGS_LOAD_SETTINGS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_INTERCHANGE_TRANSLATOR_BASE_SET_SETTINGS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_INTERCHANGE_TRANSLATOR_BASE_GET_TRANSLATOR_TYPE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_INTERCHANGE_TRANSLATOR_BASE_GET_SUPPORTED_FORMATS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_INTERCHANGE_TRANSLATOR_BASE_GET_SUPPORTED_ASSET_TYPES: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_INTERCHANGE_TRANSLATOR_BASE_GET_SOURCE_DATA: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_INTERCHANGE_TRANSLATOR_BASE_GET_SETTINGS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_INTERCHANGE_TRANSLATOR_BASE_CAN_IMPORT_SOURCE_DATA: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_INTERCHANGE_BASE_NODE_SET_ENABLED: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_INTERCHANGE_BASE_NODE_SET_DISPLAY_LABEL: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_INTERCHANGE_BASE_NODE_SET_ASSET_NAME: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_INTERCHANGE_BASE_NODE_REMOVE_TARGET_NODE_UID: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_INTERCHANGE_BASE_NODE_REMOVE_ATTRIBUTE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_INTERCHANGE_BASE_NODE_IS_ENABLED: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_INTERCHANGE_BASE_NODE_INITIALIZE_NODE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_INTERCHANGE_BASE_NODE_GET_VECTOR2_ATTRIBUTE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_INTERCHANGE_BASE_NODE_GET_UNIQUE_ID: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_INTERCHANGE_BASE_NODE_GET_TYPE_NAME: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_INTERCHANGE_BASE_NODE_GET_TARGET_NODE_UIDS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_INTERCHANGE_BASE_NODE_GET_TARGET_NODE_COUNT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_INTERCHANGE_BASE_NODE_GET_STRING_ATTRIBUTE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_INTERCHANGE_BASE_NODE_GET_PARENT_UID: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_INTERCHANGE_BASE_NODE_GET_NODE_CONTAINER_TYPE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_INTERCHANGE_BASE_NODE_GET_NAMESPACE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_INTERCHANGE_BASE_NODE_GET_LINEAR_COLOR_ATTRIBUTE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_INTERCHANGE_BASE_NODE_GET_INT32_ATTRIBUTE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_INTERCHANGE_BASE_NODE_GET_ICON_NAME: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_INTERCHANGE_BASE_NODE_GET_GUID_ATTRIBUTE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_INTERCHANGE_BASE_NODE_GET_FLOAT_ATTRIBUTE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_INTERCHANGE_BASE_NODE_GET_DOUBLE_ATTRIBUTE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_INTERCHANGE_BASE_NODE_GET_DISPLAY_LABEL: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_INTERCHANGE_BASE_NODE_GET_DESIRED_CHILD_INDEX: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_INTERCHANGE_BASE_NODE_GET_BOOLEAN_ATTRIBUTE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_INTERCHANGE_BASE_NODE_GET_ASSET_NAME: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_INTERCHANGE_BASE_NODE_ADD_VECTOR2_ATTRIBUTE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_INTERCHANGE_BASE_NODE_ADD_TARGET_NODE_UID: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_INTERCHANGE_BASE_NODE_ADD_STRING_ATTRIBUTE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_INTERCHANGE_BASE_NODE_ADD_LINEAR_COLOR_ATTRIBUTE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_INTERCHANGE_BASE_NODE_ADD_INT32_ATTRIBUTE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_INTERCHANGE_BASE_NODE_ADD_GUID_ATTRIBUTE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_INTERCHANGE_BASE_NODE_ADD_FLOAT_ATTRIBUTE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_INTERCHANGE_BASE_NODE_ADD_DOUBLE_ATTRIBUTE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_INTERCHANGE_BASE_NODE_ADD_BOOLEAN_ATTRIBUTE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_INTERCHANGE_BASE_NODE_CONTAINER_SET_NODE_PARENT_UID: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_INTERCHANGE_BASE_NODE_CONTAINER_SET_NODE_DESIRED_CHILD_INDEX: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_INTERCHANGE_BASE_NODE_CONTAINER_SET_NAMESPACE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_INTERCHANGE_BASE_NODE_CONTAINER_SAVE_TO_FILE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_INTERCHANGE_BASE_NODE_CONTAINER_RESET_CHILDREN_CACHE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_INTERCHANGE_BASE_NODE_CONTAINER_RESET: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_INTERCHANGE_BASE_NODE_CONTAINER_REPLACE_NODE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_INTERCHANGE_BASE_NODE_CONTAINER_REMOVE_NODE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_INTERCHANGE_BASE_NODE_CONTAINER_LOAD_FROM_FILE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_INTERCHANGE_BASE_NODE_CONTAINER_IS_NODE_UID_VALID: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_INTERCHANGE_BASE_NODE_CONTAINER_GET_ROOTS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_INTERCHANGE_BASE_NODE_CONTAINER_GET_NODES: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_INTERCHANGE_BASE_NODE_CONTAINER_GET_NODE_CHILDREN_UIDS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_INTERCHANGE_BASE_NODE_CONTAINER_GET_NODE_CHILDREN_COUNT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_INTERCHANGE_BASE_NODE_CONTAINER_GET_NODE_CHILDREN: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_INTERCHANGE_BASE_NODE_CONTAINER_GET_NODE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_INTERCHANGE_BASE_NODE_CONTAINER_GET_IS_ANCESTOR: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_INTERCHANGE_BASE_NODE_CONTAINER_GET_FACTORY_NODE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_INTERCHANGE_BASE_NODE_CONTAINER_COMPUTE_CHILDREN_CACHE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_INTERCHANGE_BASE_NODE_CONTAINER_CLEAR_NODE_PARENT_UID: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_INTERCHANGE_BASE_NODE_CONTAINER_ADD_NODE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_INTERCHANGE_FACTORY_BASE_NODE_UNSET_SKIP_NODE_IMPORT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_INTERCHANGE_FACTORY_BASE_NODE_UNSET_FORCE_NODE_REIMPORT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_INTERCHANGE_FACTORY_BASE_NODE_SHOULD_SKIP_NODE_IMPORT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_INTERCHANGE_FACTORY_BASE_NODE_SHOULD_FORCE_NODE_REIMPORT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_INTERCHANGE_FACTORY_BASE_NODE_SET_SKIP_NODE_IMPORT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_INTERCHANGE_FACTORY_BASE_NODE_SET_REIMPORT_STRATEGY_FLAGS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_INTERCHANGE_FACTORY_BASE_NODE_SET_FORCE_NODE_REIMPORT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_INTERCHANGE_FACTORY_BASE_NODE_SET_CUSTOM_SUB_PATH: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_INTERCHANGE_FACTORY_BASE_NODE_SET_CUSTOM_REFERENCE_OBJECT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_INTERCHANGE_FACTORY_BASE_NODE_SET_CUSTOM_LEVEL_UID: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_INTERCHANGE_FACTORY_BASE_NODE_REMOVE_FACTORY_DEPENDENCY_UID: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_INTERCHANGE_FACTORY_BASE_NODE_IS_RUNTIME_IMPORT_ALLOWED: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_INTERCHANGE_FACTORY_BASE_NODE_GET_REIMPORT_STRATEGY_FLAGS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_INTERCHANGE_FACTORY_BASE_NODE_GET_OBJECT_CLASS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_INTERCHANGE_FACTORY_BASE_NODE_GET_FACTORY_DEPENDENCY: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_INTERCHANGE_FACTORY_BASE_NODE_GET_FACTORY_DEPENDENCIES_COUNT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_INTERCHANGE_FACTORY_BASE_NODE_GET_FACTORY_DEPENDENCIES: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_INTERCHANGE_FACTORY_BASE_NODE_GET_CUSTOM_SUB_PATH: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_INTERCHANGE_FACTORY_BASE_NODE_GET_CUSTOM_REFERENCE_OBJECT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_INTERCHANGE_FACTORY_BASE_NODE_GET_CUSTOM_LEVEL_UID: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_INTERCHANGE_FACTORY_BASE_NODE_ADD_FACTORY_DEPENDENCY_UID: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_INTERCHANGE_SOURCE_NODE_SET_EXTRA_INFORMATION: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_INTERCHANGE_SOURCE_NODE_SET_CUSTOM_USE_LEGACY_SKELETAL_MESH_BAKE_TRANSFORM: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_INTERCHANGE_SOURCE_NODE_SET_CUSTOM_USE_ASSET_TYPE_SUB_PATH_SUFFIX: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_INTERCHANGE_SOURCE_NODE_SET_CUSTOM_SUB_PATH_PREFIX: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_INTERCHANGE_SOURCE_NODE_SET_CUSTOM_SOURCE_TIMELINE_START: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_INTERCHANGE_SOURCE_NODE_SET_CUSTOM_SOURCE_TIMELINE_END: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_INTERCHANGE_SOURCE_NODE_SET_CUSTOM_SOURCE_FRAME_RATE_NUMERATOR: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_INTERCHANGE_SOURCE_NODE_SET_CUSTOM_SOURCE_FRAME_RATE_DENOMINATOR: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_INTERCHANGE_SOURCE_NODE_SET_CUSTOM_SKELETAL_MESH_FRONT_AXIS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_INTERCHANGE_SOURCE_NODE_SET_CUSTOM_REIMPORT_STRATEGY_FLAGS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_INTERCHANGE_SOURCE_NODE_SET_CUSTOM_NANITE_TRIANGLE_THRESHOLD: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_INTERCHANGE_SOURCE_NODE_SET_CUSTOM_IMPORT_UNUSED_MATERIAL: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_INTERCHANGE_SOURCE_NODE_SET_CUSTOM_AXIS_CONVERSION_INVERSE_TRANSFORM: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_INTERCHANGE_SOURCE_NODE_SET_CUSTOM_ANIMATED_TIME_START: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_INTERCHANGE_SOURCE_NODE_SET_CUSTOM_ANIMATED_TIME_END: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_INTERCHANGE_SOURCE_NODE_SET_CUSTOM_ALLOW_SCENE_ROOT_AS_JOINT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_INTERCHANGE_SOURCE_NODE_REMOVE_EXTRA_INFORMATION: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_INTERCHANGE_SOURCE_NODE_INITIALIZE_SOURCE_NODE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_INTERCHANGE_SOURCE_NODE_GET_UNIQUE_INSTANCE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_INTERCHANGE_SOURCE_NODE_GET_EXTRA_INFORMATION: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_INTERCHANGE_SOURCE_NODE_GET_CUSTOM_USE_LEGACY_SKELETAL_MESH_BAKE_TRANSFORM: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_INTERCHANGE_SOURCE_NODE_GET_CUSTOM_USE_ASSET_TYPE_SUB_PATH_SUFFIX: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_INTERCHANGE_SOURCE_NODE_GET_CUSTOM_SUB_PATH_PREFIX: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_INTERCHANGE_SOURCE_NODE_GET_CUSTOM_SOURCE_TIMELINE_START: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_INTERCHANGE_SOURCE_NODE_GET_CUSTOM_SOURCE_TIMELINE_END: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_INTERCHANGE_SOURCE_NODE_GET_CUSTOM_SOURCE_FRAME_RATE_NUMERATOR: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_INTERCHANGE_SOURCE_NODE_GET_CUSTOM_SOURCE_FRAME_RATE_DENOMINATOR: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_INTERCHANGE_SOURCE_NODE_GET_CUSTOM_SKELETAL_MESH_FRONT_AXIS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_INTERCHANGE_SOURCE_NODE_GET_CUSTOM_REIMPORT_STRATEGY_FLAGS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_INTERCHANGE_SOURCE_NODE_GET_CUSTOM_NANITE_TRIANGLE_THRESHOLD: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_INTERCHANGE_SOURCE_NODE_GET_CUSTOM_IMPORT_UNUSED_MATERIAL: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_INTERCHANGE_SOURCE_NODE_GET_CUSTOM_AXIS_CONVERSION_INVERSE_TRANSFORM: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_INTERCHANGE_SOURCE_NODE_GET_CUSTOM_ANIMATED_TIME_START: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_INTERCHANGE_SOURCE_NODE_GET_CUSTOM_ANIMATED_TIME_END: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_INTERCHANGE_SOURCE_NODE_GET_CUSTOM_ALLOW_SCENE_ROOT_AS_JOINT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_INTERCHANGE_USER_DEFINED_ATTRIBUTES_API_REMOVE_USER_DEFINED_ATTRIBUTE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_INTERCHANGE_USER_DEFINED_ATTRIBUTES_API_GET_USER_DEFINED_ATTRIBUTE_INFOS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_INTERCHANGE_USER_DEFINED_ATTRIBUTES_API_GET_USER_DEFINED_ATTRIBUTE_INT32: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_INTERCHANGE_USER_DEFINED_ATTRIBUTES_API_GET_USER_DEFINED_ATTRIBUTE_F_STRING: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_INTERCHANGE_USER_DEFINED_ATTRIBUTES_API_GET_USER_DEFINED_ATTRIBUTE_FLOAT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_INTERCHANGE_USER_DEFINED_ATTRIBUTES_API_GET_USER_DEFINED_ATTRIBUTE_DOUBLE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_INTERCHANGE_USER_DEFINED_ATTRIBUTES_API_GET_USER_DEFINED_ATTRIBUTE_BOOLEAN: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_INTERCHANGE_USER_DEFINED_ATTRIBUTES_API_DUPLICATE_ALL_USER_DEFINED_ATTRIBUTE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_INTERCHANGE_USER_DEFINED_ATTRIBUTES_API_CREATE_USER_DEFINED_ATTRIBUTE_INT32: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_INTERCHANGE_USER_DEFINED_ATTRIBUTES_API_CREATE_USER_DEFINED_ATTRIBUTE_F_STRING: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_INTERCHANGE_USER_DEFINED_ATTRIBUTES_API_CREATE_USER_DEFINED_ATTRIBUTE_FLOAT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_INTERCHANGE_USER_DEFINED_ATTRIBUTES_API_CREATE_USER_DEFINED_ATTRIBUTE_DOUBLE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_INTERCHANGE_USER_DEFINED_ATTRIBUTES_API_CREATE_USER_DEFINED_ATTRIBUTE_BOOLEAN: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
pub fn initialize() {
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UInterchangeFactoryBase::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetSourceFilename"),
            &raw mut U_INTERCHANGE_FACTORY_BASE_SET_SOURCE_FILENAME,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetSourceFilenames"),
            &raw mut U_INTERCHANGE_FACTORY_BASE_GET_SOURCE_FILENAMES,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetFactoryClass"),
            &raw mut U_INTERCHANGE_FACTORY_BASE_GET_FACTORY_CLASS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetFactoryAssetType"),
            &raw mut U_INTERCHANGE_FACTORY_BASE_GET_FACTORY_ASSET_TYPE,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UInterchangePipelineBase::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SupportReimport"),
            &raw mut U_INTERCHANGE_PIPELINE_BASE_SUPPORT_REIMPORT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ScriptedSetReimportSourceIndex"),
            &raw mut U_INTERCHANGE_PIPELINE_BASE_SCRIPTED_SET_REIMPORT_SOURCE_INDEX,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ScriptedGetPipelineDisplayName"),
            &raw mut U_INTERCHANGE_PIPELINE_BASE_SCRIPTED_GET_PIPELINE_DISPLAY_NAME,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ScriptedExecutePostImportPipeline"),
            &raw mut U_INTERCHANGE_PIPELINE_BASE_SCRIPTED_EXECUTE_POST_IMPORT_PIPELINE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ScriptedExecutePostFactoryPipeline"),
            &raw mut U_INTERCHANGE_PIPELINE_BASE_SCRIPTED_EXECUTE_POST_FACTORY_PIPELINE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ScriptedExecutePostBroadcastPipeline"),
            &raw mut U_INTERCHANGE_PIPELINE_BASE_SCRIPTED_EXECUTE_POST_BROADCAST_PIPELINE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ScriptedExecutePipeline"),
            &raw mut U_INTERCHANGE_PIPELINE_BASE_SCRIPTED_EXECUTE_PIPELINE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ScriptedExecuteExportPipeline"),
            &raw mut U_INTERCHANGE_PIPELINE_BASE_SCRIPTED_EXECUTE_EXPORT_PIPELINE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("IsReimportContext"),
            &raw mut U_INTERCHANGE_PIPELINE_BASE_IS_REIMPORT_CONTEXT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetSupportAssetClasses"),
            &raw mut U_INTERCHANGE_PIPELINE_BASE_GET_SUPPORT_ASSET_CLASSES,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("FindOrAddPropertyStates"),
            &raw mut U_INTERCHANGE_PIPELINE_BASE_FIND_OR_ADD_PROPERTY_STATES,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("DoesPropertyStatesExist"),
            &raw mut U_INTERCHANGE_PIPELINE_BASE_DOES_PROPERTY_STATES_EXIST,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UInterchangeSourceData::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetFilename"),
            &raw mut U_INTERCHANGE_SOURCE_DATA_SET_FILENAME,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetContextObjectByTag"),
            &raw mut U_INTERCHANGE_SOURCE_DATA_SET_CONTEXT_OBJECT_BY_TAG,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RemoveAllContextObjects"),
            &raw mut U_INTERCHANGE_SOURCE_DATA_REMOVE_ALL_CONTEXT_OBJECTS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetFilename"),
            &raw mut U_INTERCHANGE_SOURCE_DATA_GET_FILENAME,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetContextObjectByTag"),
            &raw mut U_INTERCHANGE_SOURCE_DATA_GET_CONTEXT_OBJECT_BY_TAG,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetAllContextObjectTags"),
            &raw mut U_INTERCHANGE_SOURCE_DATA_GET_ALL_CONTEXT_OBJECT_TAGS,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UInterchangeTranslatorSettings::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SaveSettings"),
            &raw mut U_INTERCHANGE_TRANSLATOR_SETTINGS_SAVE_SETTINGS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("LoadSettings"),
            &raw mut U_INTERCHANGE_TRANSLATOR_SETTINGS_LOAD_SETTINGS,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UInterchangeTranslatorBase::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetSettings"),
            &raw mut U_INTERCHANGE_TRANSLATOR_BASE_SET_SETTINGS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetTranslatorType"),
            &raw mut U_INTERCHANGE_TRANSLATOR_BASE_GET_TRANSLATOR_TYPE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetSupportedFormats"),
            &raw mut U_INTERCHANGE_TRANSLATOR_BASE_GET_SUPPORTED_FORMATS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetSupportedAssetTypes"),
            &raw mut U_INTERCHANGE_TRANSLATOR_BASE_GET_SUPPORTED_ASSET_TYPES,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetSourceData"),
            &raw mut U_INTERCHANGE_TRANSLATOR_BASE_GET_SOURCE_DATA,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetSettings"),
            &raw mut U_INTERCHANGE_TRANSLATOR_BASE_GET_SETTINGS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CanImportSourceData"),
            &raw mut U_INTERCHANGE_TRANSLATOR_BASE_CAN_IMPORT_SOURCE_DATA,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UInterchangeBaseNode::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetEnabled"),
            &raw mut U_INTERCHANGE_BASE_NODE_SET_ENABLED,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetDisplayLabel"),
            &raw mut U_INTERCHANGE_BASE_NODE_SET_DISPLAY_LABEL,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetAssetName"),
            &raw mut U_INTERCHANGE_BASE_NODE_SET_ASSET_NAME,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RemoveTargetNodeUid"),
            &raw mut U_INTERCHANGE_BASE_NODE_REMOVE_TARGET_NODE_UID,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RemoveAttribute"),
            &raw mut U_INTERCHANGE_BASE_NODE_REMOVE_ATTRIBUTE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("IsEnabled"),
            &raw mut U_INTERCHANGE_BASE_NODE_IS_ENABLED,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("InitializeNode"),
            &raw mut U_INTERCHANGE_BASE_NODE_INITIALIZE_NODE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetVector2Attribute"),
            &raw mut U_INTERCHANGE_BASE_NODE_GET_VECTOR2_ATTRIBUTE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetUniqueID"),
            &raw mut U_INTERCHANGE_BASE_NODE_GET_UNIQUE_ID,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetTypeName"),
            &raw mut U_INTERCHANGE_BASE_NODE_GET_TYPE_NAME,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetTargetNodeUids"),
            &raw mut U_INTERCHANGE_BASE_NODE_GET_TARGET_NODE_UIDS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetTargetNodeCount"),
            &raw mut U_INTERCHANGE_BASE_NODE_GET_TARGET_NODE_COUNT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetStringAttribute"),
            &raw mut U_INTERCHANGE_BASE_NODE_GET_STRING_ATTRIBUTE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetParentUid"),
            &raw mut U_INTERCHANGE_BASE_NODE_GET_PARENT_UID,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetNodeContainerType"),
            &raw mut U_INTERCHANGE_BASE_NODE_GET_NODE_CONTAINER_TYPE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetNamespace"),
            &raw mut U_INTERCHANGE_BASE_NODE_GET_NAMESPACE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetLinearColorAttribute"),
            &raw mut U_INTERCHANGE_BASE_NODE_GET_LINEAR_COLOR_ATTRIBUTE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetInt32Attribute"),
            &raw mut U_INTERCHANGE_BASE_NODE_GET_INT32_ATTRIBUTE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetIconName"),
            &raw mut U_INTERCHANGE_BASE_NODE_GET_ICON_NAME,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetGuidAttribute"),
            &raw mut U_INTERCHANGE_BASE_NODE_GET_GUID_ATTRIBUTE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetFloatAttribute"),
            &raw mut U_INTERCHANGE_BASE_NODE_GET_FLOAT_ATTRIBUTE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetDoubleAttribute"),
            &raw mut U_INTERCHANGE_BASE_NODE_GET_DOUBLE_ATTRIBUTE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetDisplayLabel"),
            &raw mut U_INTERCHANGE_BASE_NODE_GET_DISPLAY_LABEL,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetDesiredChildIndex"),
            &raw mut U_INTERCHANGE_BASE_NODE_GET_DESIRED_CHILD_INDEX,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetBooleanAttribute"),
            &raw mut U_INTERCHANGE_BASE_NODE_GET_BOOLEAN_ATTRIBUTE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetAssetName"),
            &raw mut U_INTERCHANGE_BASE_NODE_GET_ASSET_NAME,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AddVector2Attribute"),
            &raw mut U_INTERCHANGE_BASE_NODE_ADD_VECTOR2_ATTRIBUTE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AddTargetNodeUid"),
            &raw mut U_INTERCHANGE_BASE_NODE_ADD_TARGET_NODE_UID,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AddStringAttribute"),
            &raw mut U_INTERCHANGE_BASE_NODE_ADD_STRING_ATTRIBUTE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AddLinearColorAttribute"),
            &raw mut U_INTERCHANGE_BASE_NODE_ADD_LINEAR_COLOR_ATTRIBUTE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AddInt32Attribute"),
            &raw mut U_INTERCHANGE_BASE_NODE_ADD_INT32_ATTRIBUTE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AddGuidAttribute"),
            &raw mut U_INTERCHANGE_BASE_NODE_ADD_GUID_ATTRIBUTE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AddFloatAttribute"),
            &raw mut U_INTERCHANGE_BASE_NODE_ADD_FLOAT_ATTRIBUTE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AddDoubleAttribute"),
            &raw mut U_INTERCHANGE_BASE_NODE_ADD_DOUBLE_ATTRIBUTE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AddBooleanAttribute"),
            &raw mut U_INTERCHANGE_BASE_NODE_ADD_BOOLEAN_ATTRIBUTE,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UInterchangeBaseNodeContainer::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetNodeParentUid"),
            &raw mut U_INTERCHANGE_BASE_NODE_CONTAINER_SET_NODE_PARENT_UID,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetNodeDesiredChildIndex"),
            &raw mut U_INTERCHANGE_BASE_NODE_CONTAINER_SET_NODE_DESIRED_CHILD_INDEX,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetNamespace"),
            &raw mut U_INTERCHANGE_BASE_NODE_CONTAINER_SET_NAMESPACE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SaveToFile"),
            &raw mut U_INTERCHANGE_BASE_NODE_CONTAINER_SAVE_TO_FILE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ResetChildrenCache"),
            &raw mut U_INTERCHANGE_BASE_NODE_CONTAINER_RESET_CHILDREN_CACHE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("Reset"),
            &raw mut U_INTERCHANGE_BASE_NODE_CONTAINER_RESET,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ReplaceNode"),
            &raw mut U_INTERCHANGE_BASE_NODE_CONTAINER_REPLACE_NODE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RemoveNode"),
            &raw mut U_INTERCHANGE_BASE_NODE_CONTAINER_REMOVE_NODE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("LoadFromFile"),
            &raw mut U_INTERCHANGE_BASE_NODE_CONTAINER_LOAD_FROM_FILE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("IsNodeUidValid"),
            &raw mut U_INTERCHANGE_BASE_NODE_CONTAINER_IS_NODE_UID_VALID,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetRoots"),
            &raw mut U_INTERCHANGE_BASE_NODE_CONTAINER_GET_ROOTS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetNodes"),
            &raw mut U_INTERCHANGE_BASE_NODE_CONTAINER_GET_NODES,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetNodeChildrenUids"),
            &raw mut U_INTERCHANGE_BASE_NODE_CONTAINER_GET_NODE_CHILDREN_UIDS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetNodeChildrenCount"),
            &raw mut U_INTERCHANGE_BASE_NODE_CONTAINER_GET_NODE_CHILDREN_COUNT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetNodeChildren"),
            &raw mut U_INTERCHANGE_BASE_NODE_CONTAINER_GET_NODE_CHILDREN,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetNode"),
            &raw mut U_INTERCHANGE_BASE_NODE_CONTAINER_GET_NODE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetIsAncestor"),
            &raw mut U_INTERCHANGE_BASE_NODE_CONTAINER_GET_IS_ANCESTOR,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetFactoryNode"),
            &raw mut U_INTERCHANGE_BASE_NODE_CONTAINER_GET_FACTORY_NODE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ComputeChildrenCache"),
            &raw mut U_INTERCHANGE_BASE_NODE_CONTAINER_COMPUTE_CHILDREN_CACHE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ClearNodeParentUid"),
            &raw mut U_INTERCHANGE_BASE_NODE_CONTAINER_CLEAR_NODE_PARENT_UID,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AddNode"),
            &raw mut U_INTERCHANGE_BASE_NODE_CONTAINER_ADD_NODE,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UInterchangeFactoryBaseNode::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("UnsetSkipNodeImport"),
            &raw mut U_INTERCHANGE_FACTORY_BASE_NODE_UNSET_SKIP_NODE_IMPORT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("UnsetForceNodeReimport"),
            &raw mut U_INTERCHANGE_FACTORY_BASE_NODE_UNSET_FORCE_NODE_REIMPORT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ShouldSkipNodeImport"),
            &raw mut U_INTERCHANGE_FACTORY_BASE_NODE_SHOULD_SKIP_NODE_IMPORT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ShouldForceNodeReimport"),
            &raw mut U_INTERCHANGE_FACTORY_BASE_NODE_SHOULD_FORCE_NODE_REIMPORT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetSkipNodeImport"),
            &raw mut U_INTERCHANGE_FACTORY_BASE_NODE_SET_SKIP_NODE_IMPORT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetReimportStrategyFlags"),
            &raw mut U_INTERCHANGE_FACTORY_BASE_NODE_SET_REIMPORT_STRATEGY_FLAGS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetForceNodeReimport"),
            &raw mut U_INTERCHANGE_FACTORY_BASE_NODE_SET_FORCE_NODE_REIMPORT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetCustomSubPath"),
            &raw mut U_INTERCHANGE_FACTORY_BASE_NODE_SET_CUSTOM_SUB_PATH,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetCustomReferenceObject"),
            &raw mut U_INTERCHANGE_FACTORY_BASE_NODE_SET_CUSTOM_REFERENCE_OBJECT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetCustomLevelUid"),
            &raw mut U_INTERCHANGE_FACTORY_BASE_NODE_SET_CUSTOM_LEVEL_UID,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RemoveFactoryDependencyUid"),
            &raw mut U_INTERCHANGE_FACTORY_BASE_NODE_REMOVE_FACTORY_DEPENDENCY_UID,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("IsRuntimeImportAllowed"),
            &raw mut U_INTERCHANGE_FACTORY_BASE_NODE_IS_RUNTIME_IMPORT_ALLOWED,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetReimportStrategyFlags"),
            &raw mut U_INTERCHANGE_FACTORY_BASE_NODE_GET_REIMPORT_STRATEGY_FLAGS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetObjectClass"),
            &raw mut U_INTERCHANGE_FACTORY_BASE_NODE_GET_OBJECT_CLASS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetFactoryDependency"),
            &raw mut U_INTERCHANGE_FACTORY_BASE_NODE_GET_FACTORY_DEPENDENCY,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetFactoryDependenciesCount"),
            &raw mut U_INTERCHANGE_FACTORY_BASE_NODE_GET_FACTORY_DEPENDENCIES_COUNT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetFactoryDependencies"),
            &raw mut U_INTERCHANGE_FACTORY_BASE_NODE_GET_FACTORY_DEPENDENCIES,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetCustomSubPath"),
            &raw mut U_INTERCHANGE_FACTORY_BASE_NODE_GET_CUSTOM_SUB_PATH,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetCustomReferenceObject"),
            &raw mut U_INTERCHANGE_FACTORY_BASE_NODE_GET_CUSTOM_REFERENCE_OBJECT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetCustomLevelUid"),
            &raw mut U_INTERCHANGE_FACTORY_BASE_NODE_GET_CUSTOM_LEVEL_UID,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AddFactoryDependencyUid"),
            &raw mut U_INTERCHANGE_FACTORY_BASE_NODE_ADD_FACTORY_DEPENDENCY_UID,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UInterchangeSourceNode::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetExtraInformation"),
            &raw mut U_INTERCHANGE_SOURCE_NODE_SET_EXTRA_INFORMATION,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetCustomUseLegacySkeletalMeshBakeTransform"),
            &raw mut U_INTERCHANGE_SOURCE_NODE_SET_CUSTOM_USE_LEGACY_SKELETAL_MESH_BAKE_TRANSFORM,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetCustomUseAssetTypeSubPathSuffix"),
            &raw mut U_INTERCHANGE_SOURCE_NODE_SET_CUSTOM_USE_ASSET_TYPE_SUB_PATH_SUFFIX,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetCustomSubPathPrefix"),
            &raw mut U_INTERCHANGE_SOURCE_NODE_SET_CUSTOM_SUB_PATH_PREFIX,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetCustomSourceTimelineStart"),
            &raw mut U_INTERCHANGE_SOURCE_NODE_SET_CUSTOM_SOURCE_TIMELINE_START,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetCustomSourceTimelineEnd"),
            &raw mut U_INTERCHANGE_SOURCE_NODE_SET_CUSTOM_SOURCE_TIMELINE_END,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetCustomSourceFrameRateNumerator"),
            &raw mut U_INTERCHANGE_SOURCE_NODE_SET_CUSTOM_SOURCE_FRAME_RATE_NUMERATOR,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetCustomSourceFrameRateDenominator"),
            &raw mut U_INTERCHANGE_SOURCE_NODE_SET_CUSTOM_SOURCE_FRAME_RATE_DENOMINATOR,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetCustomSkeletalMeshFrontAxis"),
            &raw mut U_INTERCHANGE_SOURCE_NODE_SET_CUSTOM_SKELETAL_MESH_FRONT_AXIS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetCustomReimportStrategyFlags"),
            &raw mut U_INTERCHANGE_SOURCE_NODE_SET_CUSTOM_REIMPORT_STRATEGY_FLAGS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetCustomNaniteTriangleThreshold"),
            &raw mut U_INTERCHANGE_SOURCE_NODE_SET_CUSTOM_NANITE_TRIANGLE_THRESHOLD,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetCustomImportUnusedMaterial"),
            &raw mut U_INTERCHANGE_SOURCE_NODE_SET_CUSTOM_IMPORT_UNUSED_MATERIAL,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetCustomAxisConversionInverseTransform"),
            &raw mut U_INTERCHANGE_SOURCE_NODE_SET_CUSTOM_AXIS_CONVERSION_INVERSE_TRANSFORM,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetCustomAnimatedTimeStart"),
            &raw mut U_INTERCHANGE_SOURCE_NODE_SET_CUSTOM_ANIMATED_TIME_START,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetCustomAnimatedTimeEnd"),
            &raw mut U_INTERCHANGE_SOURCE_NODE_SET_CUSTOM_ANIMATED_TIME_END,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetCustomAllowSceneRootAsJoint"),
            &raw mut U_INTERCHANGE_SOURCE_NODE_SET_CUSTOM_ALLOW_SCENE_ROOT_AS_JOINT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RemoveExtraInformation"),
            &raw mut U_INTERCHANGE_SOURCE_NODE_REMOVE_EXTRA_INFORMATION,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("InitializeSourceNode"),
            &raw mut U_INTERCHANGE_SOURCE_NODE_INITIALIZE_SOURCE_NODE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetUniqueInstance"),
            &raw mut U_INTERCHANGE_SOURCE_NODE_GET_UNIQUE_INSTANCE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetExtraInformation"),
            &raw mut U_INTERCHANGE_SOURCE_NODE_GET_EXTRA_INFORMATION,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetCustomUseLegacySkeletalMeshBakeTransform"),
            &raw mut U_INTERCHANGE_SOURCE_NODE_GET_CUSTOM_USE_LEGACY_SKELETAL_MESH_BAKE_TRANSFORM,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetCustomUseAssetTypeSubPathSuffix"),
            &raw mut U_INTERCHANGE_SOURCE_NODE_GET_CUSTOM_USE_ASSET_TYPE_SUB_PATH_SUFFIX,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetCustomSubPathPrefix"),
            &raw mut U_INTERCHANGE_SOURCE_NODE_GET_CUSTOM_SUB_PATH_PREFIX,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetCustomSourceTimelineStart"),
            &raw mut U_INTERCHANGE_SOURCE_NODE_GET_CUSTOM_SOURCE_TIMELINE_START,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetCustomSourceTimelineEnd"),
            &raw mut U_INTERCHANGE_SOURCE_NODE_GET_CUSTOM_SOURCE_TIMELINE_END,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetCustomSourceFrameRateNumerator"),
            &raw mut U_INTERCHANGE_SOURCE_NODE_GET_CUSTOM_SOURCE_FRAME_RATE_NUMERATOR,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetCustomSourceFrameRateDenominator"),
            &raw mut U_INTERCHANGE_SOURCE_NODE_GET_CUSTOM_SOURCE_FRAME_RATE_DENOMINATOR,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetCustomSkeletalMeshFrontAxis"),
            &raw mut U_INTERCHANGE_SOURCE_NODE_GET_CUSTOM_SKELETAL_MESH_FRONT_AXIS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetCustomReimportStrategyFlags"),
            &raw mut U_INTERCHANGE_SOURCE_NODE_GET_CUSTOM_REIMPORT_STRATEGY_FLAGS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetCustomNaniteTriangleThreshold"),
            &raw mut U_INTERCHANGE_SOURCE_NODE_GET_CUSTOM_NANITE_TRIANGLE_THRESHOLD,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetCustomImportUnusedMaterial"),
            &raw mut U_INTERCHANGE_SOURCE_NODE_GET_CUSTOM_IMPORT_UNUSED_MATERIAL,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetCustomAxisConversionInverseTransform"),
            &raw mut U_INTERCHANGE_SOURCE_NODE_GET_CUSTOM_AXIS_CONVERSION_INVERSE_TRANSFORM,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetCustomAnimatedTimeStart"),
            &raw mut U_INTERCHANGE_SOURCE_NODE_GET_CUSTOM_ANIMATED_TIME_START,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetCustomAnimatedTimeEnd"),
            &raw mut U_INTERCHANGE_SOURCE_NODE_GET_CUSTOM_ANIMATED_TIME_END,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetCustomAllowSceneRootAsJoint"),
            &raw mut U_INTERCHANGE_SOURCE_NODE_GET_CUSTOM_ALLOW_SCENE_ROOT_AS_JOINT,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UInterchangeUserDefinedAttributesAPI::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RemoveUserDefinedAttribute"),
            &raw mut U_INTERCHANGE_USER_DEFINED_ATTRIBUTES_API_REMOVE_USER_DEFINED_ATTRIBUTE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetUserDefinedAttributeInfos"),
            &raw mut U_INTERCHANGE_USER_DEFINED_ATTRIBUTES_API_GET_USER_DEFINED_ATTRIBUTE_INFOS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetUserDefinedAttribute_Int32"),
            &raw mut U_INTERCHANGE_USER_DEFINED_ATTRIBUTES_API_GET_USER_DEFINED_ATTRIBUTE_INT32,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetUserDefinedAttribute_FString"),
            &raw mut U_INTERCHANGE_USER_DEFINED_ATTRIBUTES_API_GET_USER_DEFINED_ATTRIBUTE_F_STRING,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetUserDefinedAttribute_Float"),
            &raw mut U_INTERCHANGE_USER_DEFINED_ATTRIBUTES_API_GET_USER_DEFINED_ATTRIBUTE_FLOAT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetUserDefinedAttribute_Double"),
            &raw mut U_INTERCHANGE_USER_DEFINED_ATTRIBUTES_API_GET_USER_DEFINED_ATTRIBUTE_DOUBLE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetUserDefinedAttribute_Boolean"),
            &raw mut U_INTERCHANGE_USER_DEFINED_ATTRIBUTES_API_GET_USER_DEFINED_ATTRIBUTE_BOOLEAN,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("DuplicateAllUserDefinedAttribute"),
            &raw mut U_INTERCHANGE_USER_DEFINED_ATTRIBUTES_API_DUPLICATE_ALL_USER_DEFINED_ATTRIBUTE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CreateUserDefinedAttribute_Int32"),
            &raw mut U_INTERCHANGE_USER_DEFINED_ATTRIBUTES_API_CREATE_USER_DEFINED_ATTRIBUTE_INT32,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CreateUserDefinedAttribute_FString"),
            &raw mut U_INTERCHANGE_USER_DEFINED_ATTRIBUTES_API_CREATE_USER_DEFINED_ATTRIBUTE_F_STRING,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CreateUserDefinedAttribute_Float"),
            &raw mut U_INTERCHANGE_USER_DEFINED_ATTRIBUTES_API_CREATE_USER_DEFINED_ATTRIBUTE_FLOAT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CreateUserDefinedAttribute_Double"),
            &raw mut U_INTERCHANGE_USER_DEFINED_ATTRIBUTES_API_CREATE_USER_DEFINED_ATTRIBUTE_DOUBLE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CreateUserDefinedAttribute_Boolean"),
            &raw mut U_INTERCHANGE_USER_DEFINED_ATTRIBUTES_API_CREATE_USER_DEFINED_ATTRIBUTE_BOOLEAN,
        );
    }
}
#[repr(C, align(1))]
pub struct FInterchangePipelinePropertyStatePerContext {
    pub b_visible: bool,
}
impl FInterchangePipelinePropertyStatePerContext {}
#[repr(C, align(1))]
pub struct FInterchangePipelinePropertyStates {
    pub b_locked: bool,
    pub b_pre_dialog_reset: bool,
    pub basic_layout_states: FInterchangePipelinePropertyStatePerContext,
    pub import_states: FInterchangePipelinePropertyStatePerContext,
    pub reimport_states: FInterchangePipelinePropertyStatePerContext,
}
impl FInterchangePipelinePropertyStates {}
#[repr(C, align(8))]
pub struct FInterchangeUserDefinedAttributeInfo {
    pub name: FString,
    __padding_end: [u8; 32],
}
impl FInterchangeUserDefinedAttributeInfo {}
#[repr(C, align(8))]
pub struct UInterchangeFactoryBase {
    __padding_end: [u8; 56],
}
impl UInterchangeFactoryBase {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UInterchangeFactoryBase")
            .unwrap()
    }
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
pub struct UInterchangeWriterBase {
    __padding_end: [u8; 48],
}
impl UInterchangeWriterBase {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UInterchangeWriterBase")
            .unwrap()
    }
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
pub struct UInterchangePipelineBase {
    __padding_end: [u8; 344],
}
impl UInterchangePipelineBase {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UInterchangePipelineBase")
            .unwrap()
    }
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
pub struct UInterchangeResult {
    __padding_end: [u8; 120],
}
impl UInterchangeResult {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UInterchangeResult")
            .unwrap()
    }
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
pub struct UInterchangeResultSuccess {
    __padding_end: [u8; 120],
}
impl UInterchangeResultSuccess {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UInterchangeResultSuccess")
            .unwrap()
    }
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
pub struct UInterchangeResultWarning {
    __padding_end: [u8; 120],
}
impl UInterchangeResultWarning {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UInterchangeResultWarning")
            .unwrap()
    }
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
pub struct UInterchangeResultError {
    __padding_end: [u8; 120],
}
impl UInterchangeResultError {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UInterchangeResultError")
            .unwrap()
    }
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
pub struct UInterchangeResultWarning_Generic {
    __padding_end: [u8; 136],
}
impl UInterchangeResultWarning_Generic {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UInterchangeResultWarning_Generic")
            .unwrap()
    }
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
pub struct UInterchangeResultError_Generic {
    __padding_end: [u8; 136],
}
impl UInterchangeResultError_Generic {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UInterchangeResultError_Generic")
            .unwrap()
    }
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
pub struct UInterchangeResultError_ReimportFail {
    __padding_end: [u8; 120],
}
impl UInterchangeResultError_ReimportFail {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UInterchangeResultError_ReimportFail")
            .unwrap()
    }
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
pub struct UInterchangeResultDisplay_Generic {
    __padding_end: [u8; 136],
}
impl UInterchangeResultDisplay_Generic {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UInterchangeResultDisplay_Generic")
            .unwrap()
    }
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
pub struct UInterchangeResultsContainer {
    __padding_end: [u8; 104],
}
impl UInterchangeResultsContainer {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UInterchangeResultsContainer")
            .unwrap()
    }
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
pub struct UInterchangeSourceData {
    __padding_end: [u8; 168],
}
impl UInterchangeSourceData {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UInterchangeSourceData")
            .unwrap()
    }
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
pub struct UInterchangeTranslatorSettings {
    __padding_end: [u8; 48],
}
impl UInterchangeTranslatorSettings {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UInterchangeTranslatorSettings")
            .unwrap()
    }
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
pub struct UInterchangeTranslatorBase {
    __padding_end: [u8; 80],
}
impl UInterchangeTranslatorBase {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UInterchangeTranslatorBase")
            .unwrap()
    }
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
pub struct UInterchangeBaseNode {
    __padding_end: [u8; 112],
}
impl UInterchangeBaseNode {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UInterchangeBaseNode")
            .unwrap()
    }
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
pub struct UInterchangeBaseNodeContainer {
    __padding_end: [u8; 208],
}
impl UInterchangeBaseNodeContainer {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UInterchangeBaseNodeContainer")
            .unwrap()
    }
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
pub struct UInterchangeFactoryBaseNode {
    __padding_end: [u8; 464],
}
impl UInterchangeFactoryBaseNode {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UInterchangeFactoryBaseNode")
            .unwrap()
    }
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
pub struct UInterchangeSourceNode {
    __padding_end: [u8; 472],
}
impl UInterchangeSourceNode {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UInterchangeSourceNode")
            .unwrap()
    }
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
pub struct UInterchangeUserDefinedAttributesAPI {
    __padding_end: [u8; 48],
}
impl UInterchangeUserDefinedAttributesAPI {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UInterchangeUserDefinedAttributesAPI")
            .unwrap()
    }
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
pub struct EInterchangePipelineContext(pub u8);
impl EInterchangePipelineContext {
    pub const NONE: EInterchangePipelineContext = EInterchangePipelineContext(0);
    pub const ASSET_IMPORT: EInterchangePipelineContext = EInterchangePipelineContext(1);
    pub const ASSET_REIMPORT: EInterchangePipelineContext = EInterchangePipelineContext(
        2,
    );
    pub const SCENE_IMPORT: EInterchangePipelineContext = EInterchangePipelineContext(3);
    pub const SCENE_REIMPORT: EInterchangePipelineContext = EInterchangePipelineContext(
        4,
    );
    pub const ASSET_CUSTOM_LOD_IMPORT: EInterchangePipelineContext = EInterchangePipelineContext(
        5,
    );
    pub const ASSET_CUSTOM_LOD_REIMPORT: EInterchangePipelineContext = EInterchangePipelineContext(
        6,
    );
    pub const ASSET_ALTERNATE_SKINNING_IMPORT: EInterchangePipelineContext = EInterchangePipelineContext(
        7,
    );
    pub const ASSET_ALTERNATE_SKINNING_REIMPORT: EInterchangePipelineContext = EInterchangePipelineContext(
        8,
    );
    pub const ASSET_CUSTOM_MORPH_TARGET_IMPORT: EInterchangePipelineContext = EInterchangePipelineContext(
        9,
    );
    pub const ASSET_CUSTOM_MORPH_TARGET_RE_IMPORT: EInterchangePipelineContext = EInterchangePipelineContext(
        10,
    );
}
#[repr(transparent)]
pub struct EInterchangeTranslatorAssetType(pub u8);
impl EInterchangeTranslatorAssetType {
    pub const NONE: EInterchangeTranslatorAssetType = EInterchangeTranslatorAssetType(0);
    pub const TEXTURES: EInterchangeTranslatorAssetType = EInterchangeTranslatorAssetType(
        1,
    );
    pub const MATERIALS: EInterchangeTranslatorAssetType = EInterchangeTranslatorAssetType(
        2,
    );
    pub const MESHES: EInterchangeTranslatorAssetType = EInterchangeTranslatorAssetType(
        4,
    );
    pub const ANIMATIONS: EInterchangeTranslatorAssetType = EInterchangeTranslatorAssetType(
        8,
    );
    pub const SOUNDS: EInterchangeTranslatorAssetType = EInterchangeTranslatorAssetType(
        16,
    );
    pub const GROOMS: EInterchangeTranslatorAssetType = EInterchangeTranslatorAssetType(
        32,
    );
}
#[repr(transparent)]
pub struct EInterchangeFactoryAssetType(pub u8);
impl EInterchangeFactoryAssetType {
    pub const NONE: EInterchangeFactoryAssetType = EInterchangeFactoryAssetType(0);
    pub const TEXTURES: EInterchangeFactoryAssetType = EInterchangeFactoryAssetType(1);
    pub const MATERIALS: EInterchangeFactoryAssetType = EInterchangeFactoryAssetType(2);
    pub const MESHES: EInterchangeFactoryAssetType = EInterchangeFactoryAssetType(3);
    pub const ANIMATIONS: EInterchangeFactoryAssetType = EInterchangeFactoryAssetType(4);
    pub const PHYSICS: EInterchangeFactoryAssetType = EInterchangeFactoryAssetType(5);
    pub const GROOMS: EInterchangeFactoryAssetType = EInterchangeFactoryAssetType(6);
    pub const SOUNDS: EInterchangeFactoryAssetType = EInterchangeFactoryAssetType(7);
}
#[repr(transparent)]
pub struct EInterchangeTranslatorType(pub u8);
impl EInterchangeTranslatorType {
    pub const INVALID: EInterchangeTranslatorType = EInterchangeTranslatorType(0);
    pub const ASSETS: EInterchangeTranslatorType = EInterchangeTranslatorType(2);
    pub const ACTORS: EInterchangeTranslatorType = EInterchangeTranslatorType(4);
    pub const SCENES: EInterchangeTranslatorType = EInterchangeTranslatorType(6);
}
#[repr(transparent)]
pub struct EInterchangeNodeContainerType(pub u8);
impl EInterchangeNodeContainerType {
    pub const NONE: EInterchangeNodeContainerType = EInterchangeNodeContainerType(0);
    pub const TRANSLATED_SCENE: EInterchangeNodeContainerType = EInterchangeNodeContainerType(
        1,
    );
    pub const TRANSLATED_ASSET: EInterchangeNodeContainerType = EInterchangeNodeContainerType(
        2,
    );
    pub const FACTORY_DATA: EInterchangeNodeContainerType = EInterchangeNodeContainerType(
        3,
    );
}
#[repr(transparent)]
pub struct EReimportStrategyFlags(pub u8);
impl EReimportStrategyFlags {
    pub const APPLY_NO_PROPERTIES: EReimportStrategyFlags = EReimportStrategyFlags(0);
    pub const APPLY_PIPELINE_PROPERTIES: EReimportStrategyFlags = EReimportStrategyFlags(
        1,
    );
    pub const APPLY_EDITOR_CHANGED_PROPERTIES: EReimportStrategyFlags = EReimportStrategyFlags(
        2,
    );
}
#[repr(transparent)]
pub struct EInterchangeNodeUserInterfaceContext(pub u8);
impl EInterchangeNodeUserInterfaceContext {
    pub const NONE: EInterchangeNodeUserInterfaceContext = EInterchangeNodeUserInterfaceContext(
        0,
    );
    pub const PREVIEW: EInterchangeNodeUserInterfaceContext = EInterchangeNodeUserInterfaceContext(
        1,
    );
}
