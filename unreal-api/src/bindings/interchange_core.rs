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
    pub fn set_source_filename(
        &self,
        object: UPtr<crate::bindings::core_u_object::UObject>,
        source_filename: FString,
        source_index: i32,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<29>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_core::U_INTERCHANGE_FACTORY_BASE_SET_SOURCE_FILENAME,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &object,
                __buffer.add(0).cast::<UPtr<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &source_filename,
                __buffer.add(8).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &source_index,
                __buffer.add(24).cast::<i32>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_core::U_INTERCHANGE_FACTORY_BASE_SET_SOURCE_FILENAME,
                __buffer,
            )
        };
        unsafe { __buffer.add(28).cast::<bool>().read() }
    }
    pub fn get_source_filenames(
        &self,
        object: UPtr<crate::bindings::core_u_object::UObject>,
        out_source_filenames: &mut TArray<FString>,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<25>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_core::U_INTERCHANGE_FACTORY_BASE_GET_SOURCE_FILENAMES,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &object,
                __buffer.add(0).cast::<UPtr<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_source_filenames,
                __buffer.add(8).cast::<TArray<FString>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_core::U_INTERCHANGE_FACTORY_BASE_GET_SOURCE_FILENAMES,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(8).cast::<TArray<FString>>().swap(out_source_filenames);
        }
        unsafe { __buffer.add(24).cast::<bool>().read() }
    }
    pub fn get_factory_class(
        &self,
    ) -> TSubclassOf<crate::bindings::core_u_object::UObject> {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_core::U_INTERCHANGE_FACTORY_BASE_GET_FACTORY_CLASS,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_core::U_INTERCHANGE_FACTORY_BASE_GET_FACTORY_CLASS,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(0)
                .cast::<TSubclassOf<crate::bindings::core_u_object::UObject>>()
                .read()
        }
    }
    pub fn get_factory_asset_type(&mut self) -> EInterchangeFactoryAssetType {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_core::U_INTERCHANGE_FACTORY_BASE_GET_FACTORY_ASSET_TYPE,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_core::U_INTERCHANGE_FACTORY_BASE_GET_FACTORY_ASSET_TYPE,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<EInterchangeFactoryAssetType>().read() }
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
    pub fn support_reimport(&self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_core::U_INTERCHANGE_PIPELINE_BASE_SUPPORT_REIMPORT,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_core::U_INTERCHANGE_PIPELINE_BASE_SUPPORT_REIMPORT,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn scripted_set_reimport_source_index(
        &mut self,
        reimport_object_class: TSubclassOf<crate::bindings::core_u_object::UObject>,
        source_file_index: i32,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<12>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_core::U_INTERCHANGE_PIPELINE_BASE_SCRIPTED_SET_REIMPORT_SOURCE_INDEX,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &reimport_object_class,
                __buffer
                    .add(0)
                    .cast::<TSubclassOf<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &source_file_index,
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
                crate::bindings::interchange_core::U_INTERCHANGE_PIPELINE_BASE_SCRIPTED_SET_REIMPORT_SOURCE_INDEX,
                __buffer,
            )
        };
    }
    pub fn scripted_get_pipeline_display_name(&self) -> FString {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_core::U_INTERCHANGE_PIPELINE_BASE_SCRIPTED_GET_PIPELINE_DISPLAY_NAME,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_core::U_INTERCHANGE_PIPELINE_BASE_SCRIPTED_GET_PIPELINE_DISPLAY_NAME,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<FString>().read() }
    }
    pub fn scripted_execute_post_import_pipeline(
        &mut self,
        base_node_container: UPtr<UInterchangeBaseNodeContainer>,
        factory_node_key: FString,
        created_asset: UPtr<crate::bindings::core_u_object::UObject>,
        b_is_a_reimport: bool,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<33>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_core::U_INTERCHANGE_PIPELINE_BASE_SCRIPTED_EXECUTE_POST_IMPORT_PIPELINE,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &base_node_container,
                __buffer.add(0).cast::<UPtr<UInterchangeBaseNodeContainer>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &factory_node_key,
                __buffer.add(8).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &created_asset,
                __buffer.add(24).cast::<UPtr<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_is_a_reimport,
                __buffer.add(32).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_core::U_INTERCHANGE_PIPELINE_BASE_SCRIPTED_EXECUTE_POST_IMPORT_PIPELINE,
                __buffer,
            )
        };
    }
    pub fn scripted_execute_post_factory_pipeline(
        &mut self,
        base_node_container: UPtr<UInterchangeBaseNodeContainer>,
        factory_node_key: FString,
        created_asset: UPtr<crate::bindings::core_u_object::UObject>,
        b_is_a_reimport: bool,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<33>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_core::U_INTERCHANGE_PIPELINE_BASE_SCRIPTED_EXECUTE_POST_FACTORY_PIPELINE,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &base_node_container,
                __buffer.add(0).cast::<UPtr<UInterchangeBaseNodeContainer>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &factory_node_key,
                __buffer.add(8).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &created_asset,
                __buffer.add(24).cast::<UPtr<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_is_a_reimport,
                __buffer.add(32).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_core::U_INTERCHANGE_PIPELINE_BASE_SCRIPTED_EXECUTE_POST_FACTORY_PIPELINE,
                __buffer,
            )
        };
    }
    pub fn scripted_execute_post_broadcast_pipeline(
        &mut self,
        base_node_container: UPtr<UInterchangeBaseNodeContainer>,
        factory_node_key: FString,
        created_asset: UPtr<crate::bindings::core_u_object::UObject>,
        b_is_a_reimport: bool,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<33>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_core::U_INTERCHANGE_PIPELINE_BASE_SCRIPTED_EXECUTE_POST_BROADCAST_PIPELINE,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &base_node_container,
                __buffer.add(0).cast::<UPtr<UInterchangeBaseNodeContainer>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &factory_node_key,
                __buffer.add(8).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &created_asset,
                __buffer.add(24).cast::<UPtr<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_is_a_reimport,
                __buffer.add(32).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_core::U_INTERCHANGE_PIPELINE_BASE_SCRIPTED_EXECUTE_POST_BROADCAST_PIPELINE,
                __buffer,
            )
        };
    }
    pub fn scripted_execute_pipeline(
        &mut self,
        base_node_container: UPtr<UInterchangeBaseNodeContainer>,
        source_datas: &TArray<UPtr<UInterchangeSourceData>>,
        content_base_path: FString,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<40>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_core::U_INTERCHANGE_PIPELINE_BASE_SCRIPTED_EXECUTE_PIPELINE,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &base_node_container,
                __buffer.add(0).cast::<UPtr<UInterchangeBaseNodeContainer>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                source_datas,
                __buffer.add(8).cast::<TArray<UPtr<UInterchangeSourceData>>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &content_base_path,
                __buffer.add(24).cast::<FString>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_core::U_INTERCHANGE_PIPELINE_BASE_SCRIPTED_EXECUTE_PIPELINE,
                __buffer,
            )
        };
    }
    pub fn scripted_execute_export_pipeline(
        &mut self,
        base_node_container: UPtr<UInterchangeBaseNodeContainer>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_core::U_INTERCHANGE_PIPELINE_BASE_SCRIPTED_EXECUTE_EXPORT_PIPELINE,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &base_node_container,
                __buffer.add(0).cast::<UPtr<UInterchangeBaseNodeContainer>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_core::U_INTERCHANGE_PIPELINE_BASE_SCRIPTED_EXECUTE_EXPORT_PIPELINE,
                __buffer,
            )
        };
    }
    pub fn is_reimport_context(&mut self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_core::U_INTERCHANGE_PIPELINE_BASE_IS_REIMPORT_CONTEXT,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_core::U_INTERCHANGE_PIPELINE_BASE_IS_REIMPORT_CONTEXT,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn get_support_asset_classes(
        &self,
        pipeline_support_asset_classes: &mut TArray<
            TSubclassOf<crate::bindings::core_u_object::UObject>,
        >,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_core::U_INTERCHANGE_PIPELINE_BASE_GET_SUPPORT_ASSET_CLASSES,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                pipeline_support_asset_classes,
                __buffer
                    .add(0)
                    .cast::<
                        TArray<TSubclassOf<crate::bindings::core_u_object::UObject>>,
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
                crate::bindings::interchange_core::U_INTERCHANGE_PIPELINE_BASE_GET_SUPPORT_ASSET_CLASSES,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(0)
                .cast::<TArray<TSubclassOf<crate::bindings::core_u_object::UObject>>>()
                .swap(pipeline_support_asset_classes);
        }
    }
    pub fn find_or_add_property_states(
        &mut self,
        property_path: FName,
    ) -> FInterchangePipelinePropertyStates {
        let mut __stack = crate::core_data::StackAlloc::<17>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_core::U_INTERCHANGE_PIPELINE_BASE_FIND_OR_ADD_PROPERTY_STATES,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &property_path,
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
                crate::bindings::interchange_core::U_INTERCHANGE_PIPELINE_BASE_FIND_OR_ADD_PROPERTY_STATES,
                __buffer,
            )
        };
        unsafe { __buffer.add(12).cast::<FInterchangePipelinePropertyStates>().read() }
    }
    pub fn does_property_states_exist(&self, property_path: FName) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<13>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_core::U_INTERCHANGE_PIPELINE_BASE_DOES_PROPERTY_STATES_EXIST,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &property_path,
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
                crate::bindings::interchange_core::U_INTERCHANGE_PIPELINE_BASE_DOES_PROPERTY_STATES_EXIST,
                __buffer,
            )
        };
        unsafe { __buffer.add(12).cast::<bool>().read() }
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
    pub fn set_filename(&mut self, in_filename: FString) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<17>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_core::U_INTERCHANGE_SOURCE_DATA_SET_FILENAME,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_filename,
                __buffer.add(0).cast::<FString>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_core::U_INTERCHANGE_SOURCE_DATA_SET_FILENAME,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<bool>().read() }
    }
    pub fn set_context_object_by_tag(
        &self,
        tag: FString,
        object: UPtr<crate::bindings::core_u_object::UObject>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_core::U_INTERCHANGE_SOURCE_DATA_SET_CONTEXT_OBJECT_BY_TAG,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&tag, __buffer.add(0).cast::<FString>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &object,
                __buffer.add(16).cast::<UPtr<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_core::U_INTERCHANGE_SOURCE_DATA_SET_CONTEXT_OBJECT_BY_TAG,
                __buffer,
            )
        };
    }
    pub fn remove_all_context_objects(&self) {
        let mut __stack = crate::core_data::StackAlloc::<0>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_core::U_INTERCHANGE_SOURCE_DATA_REMOVE_ALL_CONTEXT_OBJECTS,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_core::U_INTERCHANGE_SOURCE_DATA_REMOVE_ALL_CONTEXT_OBJECTS,
                __buffer,
            )
        };
    }
    pub fn get_filename(&self) -> FString {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_core::U_INTERCHANGE_SOURCE_DATA_GET_FILENAME,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_core::U_INTERCHANGE_SOURCE_DATA_GET_FILENAME,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<FString>().read() }
    }
    pub fn get_context_object_by_tag(
        &self,
        tag: FString,
    ) -> UPtr<crate::bindings::core_u_object::UObject> {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_core::U_INTERCHANGE_SOURCE_DATA_GET_CONTEXT_OBJECT_BY_TAG,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&tag, __buffer.add(0).cast::<FString>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_core::U_INTERCHANGE_SOURCE_DATA_GET_CONTEXT_OBJECT_BY_TAG,
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
    pub fn get_all_context_object_tags(&self) -> TArray<FString> {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_core::U_INTERCHANGE_SOURCE_DATA_GET_ALL_CONTEXT_OBJECT_TAGS,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_core::U_INTERCHANGE_SOURCE_DATA_GET_ALL_CONTEXT_OBJECT_TAGS,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<TArray<FString>>().read() }
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
    pub fn save_settings(&mut self) {
        let mut __stack = crate::core_data::StackAlloc::<0>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_core::U_INTERCHANGE_TRANSLATOR_SETTINGS_SAVE_SETTINGS,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_core::U_INTERCHANGE_TRANSLATOR_SETTINGS_SAVE_SETTINGS,
                __buffer,
            )
        };
    }
    pub fn load_settings(&mut self) {
        let mut __stack = crate::core_data::StackAlloc::<0>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_core::U_INTERCHANGE_TRANSLATOR_SETTINGS_LOAD_SETTINGS,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_core::U_INTERCHANGE_TRANSLATOR_SETTINGS_LOAD_SETTINGS,
                __buffer,
            )
        };
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
    pub fn set_settings(
        &mut self,
        interchange_translator_settings: UPtr<UInterchangeTranslatorSettings>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_core::U_INTERCHANGE_TRANSLATOR_BASE_SET_SETTINGS,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &interchange_translator_settings,
                __buffer.add(0).cast::<UPtr<UInterchangeTranslatorSettings>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_core::U_INTERCHANGE_TRANSLATOR_BASE_SET_SETTINGS,
                __buffer,
            )
        };
    }
    pub fn get_translator_type(&self) -> EInterchangeTranslatorType {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_core::U_INTERCHANGE_TRANSLATOR_BASE_GET_TRANSLATOR_TYPE,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_core::U_INTERCHANGE_TRANSLATOR_BASE_GET_TRANSLATOR_TYPE,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<EInterchangeTranslatorType>().read() }
    }
    pub fn get_supported_formats(&self) -> TArray<FString> {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_core::U_INTERCHANGE_TRANSLATOR_BASE_GET_SUPPORTED_FORMATS,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_core::U_INTERCHANGE_TRANSLATOR_BASE_GET_SUPPORTED_FORMATS,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<TArray<FString>>().read() }
    }
    pub fn get_supported_asset_types(&self) -> EInterchangeTranslatorAssetType {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_core::U_INTERCHANGE_TRANSLATOR_BASE_GET_SUPPORTED_ASSET_TYPES,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_core::U_INTERCHANGE_TRANSLATOR_BASE_GET_SUPPORTED_ASSET_TYPES,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<EInterchangeTranslatorAssetType>().read() }
    }
    pub fn get_source_data(&self) -> UPtr<UInterchangeSourceData> {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_core::U_INTERCHANGE_TRANSLATOR_BASE_GET_SOURCE_DATA,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_core::U_INTERCHANGE_TRANSLATOR_BASE_GET_SOURCE_DATA,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<UPtr<UInterchangeSourceData>>().read() }
    }
    pub fn get_settings(&self) -> UPtr<UInterchangeTranslatorSettings> {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_core::U_INTERCHANGE_TRANSLATOR_BASE_GET_SETTINGS,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_core::U_INTERCHANGE_TRANSLATOR_BASE_GET_SETTINGS,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<UPtr<UInterchangeTranslatorSettings>>().read() }
    }
    pub fn can_import_source_data(
        &self,
        in_source_data: UPtr<UInterchangeSourceData>,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<9>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_core::U_INTERCHANGE_TRANSLATOR_BASE_CAN_IMPORT_SOURCE_DATA,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_source_data,
                __buffer.add(0).cast::<UPtr<UInterchangeSourceData>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_core::U_INTERCHANGE_TRANSLATOR_BASE_CAN_IMPORT_SOURCE_DATA,
                __buffer,
            )
        };
        unsafe { __buffer.add(8).cast::<bool>().read() }
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
    pub fn set_enabled(&mut self, b_is_enabled: bool) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<2>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_core::U_INTERCHANGE_BASE_NODE_SET_ENABLED,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_is_enabled,
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
                crate::bindings::interchange_core::U_INTERCHANGE_BASE_NODE_SET_ENABLED,
                __buffer,
            )
        };
        unsafe { __buffer.add(1).cast::<bool>().read() }
    }
    pub fn set_display_label(&mut self, in_display_label: FString) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<17>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_core::U_INTERCHANGE_BASE_NODE_SET_DISPLAY_LABEL,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_display_label,
                __buffer.add(0).cast::<FString>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_core::U_INTERCHANGE_BASE_NODE_SET_DISPLAY_LABEL,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<bool>().read() }
    }
    pub fn set_asset_name(&mut self, asset_name: FString) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<17>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_core::U_INTERCHANGE_BASE_NODE_SET_ASSET_NAME,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &asset_name,
                __buffer.add(0).cast::<FString>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_core::U_INTERCHANGE_BASE_NODE_SET_ASSET_NAME,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<bool>().read() }
    }
    pub fn remove_target_node_uid(&self, asset_uid: FString) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<17>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_core::U_INTERCHANGE_BASE_NODE_REMOVE_TARGET_NODE_UID,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &asset_uid,
                __buffer.add(0).cast::<FString>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_core::U_INTERCHANGE_BASE_NODE_REMOVE_TARGET_NODE_UID,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<bool>().read() }
    }
    pub fn remove_attribute(&mut self, node_attribute_key: FString) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<17>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_core::U_INTERCHANGE_BASE_NODE_REMOVE_ATTRIBUTE,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &node_attribute_key,
                __buffer.add(0).cast::<FString>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_core::U_INTERCHANGE_BASE_NODE_REMOVE_ATTRIBUTE,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<bool>().read() }
    }
    pub fn is_enabled(&self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_core::U_INTERCHANGE_BASE_NODE_IS_ENABLED,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_core::U_INTERCHANGE_BASE_NODE_IS_ENABLED,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn initialize_node(
        &mut self,
        unique_id: FString,
        display_label: FString,
        node_container_type: EInterchangeNodeContainerType,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<33>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_core::U_INTERCHANGE_BASE_NODE_INITIALIZE_NODE,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &unique_id,
                __buffer.add(0).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &display_label,
                __buffer.add(16).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &node_container_type,
                __buffer.add(32).cast::<EInterchangeNodeContainerType>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_core::U_INTERCHANGE_BASE_NODE_INITIALIZE_NODE,
                __buffer,
            )
        };
    }
    pub fn get_vector2_attribute(
        &self,
        node_attribute_key: FString,
        out_value: &mut crate::bindings::core_u_object::FVector2f,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<25>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_core::U_INTERCHANGE_BASE_NODE_GET_VECTOR2_ATTRIBUTE,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &node_attribute_key,
                __buffer.add(0).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_value,
                __buffer.add(16).cast::<crate::bindings::core_u_object::FVector2f>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_core::U_INTERCHANGE_BASE_NODE_GET_VECTOR2_ATTRIBUTE,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(16)
                .cast::<crate::bindings::core_u_object::FVector2f>()
                .swap(out_value);
        }
        unsafe { __buffer.add(24).cast::<bool>().read() }
    }
    pub fn get_unique_id(&self) -> FString {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_core::U_INTERCHANGE_BASE_NODE_GET_UNIQUE_ID,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_core::U_INTERCHANGE_BASE_NODE_GET_UNIQUE_ID,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<FString>().read() }
    }
    pub fn get_type_name(&self) -> FString {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_core::U_INTERCHANGE_BASE_NODE_GET_TYPE_NAME,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_core::U_INTERCHANGE_BASE_NODE_GET_TYPE_NAME,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<FString>().read() }
    }
    pub fn get_target_node_uids(&self, out_target_assets: &mut TArray<FString>) {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_core::U_INTERCHANGE_BASE_NODE_GET_TARGET_NODE_UIDS,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_target_assets,
                __buffer.add(0).cast::<TArray<FString>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_core::U_INTERCHANGE_BASE_NODE_GET_TARGET_NODE_UIDS,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<TArray<FString>>().swap(out_target_assets);
        }
    }
    pub fn get_target_node_count(&self) -> i32 {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_core::U_INTERCHANGE_BASE_NODE_GET_TARGET_NODE_COUNT,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_core::U_INTERCHANGE_BASE_NODE_GET_TARGET_NODE_COUNT,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<i32>().read() }
    }
    pub fn get_string_attribute(
        &self,
        node_attribute_key: FString,
        out_value: &mut FString,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<33>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_core::U_INTERCHANGE_BASE_NODE_GET_STRING_ATTRIBUTE,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &node_attribute_key,
                __buffer.add(0).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_value,
                __buffer.add(16).cast::<FString>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_core::U_INTERCHANGE_BASE_NODE_GET_STRING_ATTRIBUTE,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(16).cast::<FString>().swap(out_value);
        }
        unsafe { __buffer.add(32).cast::<bool>().read() }
    }
    pub fn get_parent_uid(&self) -> FString {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_core::U_INTERCHANGE_BASE_NODE_GET_PARENT_UID,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_core::U_INTERCHANGE_BASE_NODE_GET_PARENT_UID,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<FString>().read() }
    }
    pub fn get_node_container_type(&self) -> EInterchangeNodeContainerType {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_core::U_INTERCHANGE_BASE_NODE_GET_NODE_CONTAINER_TYPE,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_core::U_INTERCHANGE_BASE_NODE_GET_NODE_CONTAINER_TYPE,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<EInterchangeNodeContainerType>().read() }
    }
    pub fn get_namespace(&self, namespace: &mut FString) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<17>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_core::U_INTERCHANGE_BASE_NODE_GET_NAMESPACE,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                namespace,
                __buffer.add(0).cast::<FString>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_core::U_INTERCHANGE_BASE_NODE_GET_NAMESPACE,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<FString>().swap(namespace);
        }
        unsafe { __buffer.add(16).cast::<bool>().read() }
    }
    pub fn get_linear_color_attribute(
        &self,
        node_attribute_key: FString,
        out_value: &mut crate::bindings::core_u_object::FLinearColor,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<33>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_core::U_INTERCHANGE_BASE_NODE_GET_LINEAR_COLOR_ATTRIBUTE,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &node_attribute_key,
                __buffer.add(0).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_value,
                __buffer.add(16).cast::<crate::bindings::core_u_object::FLinearColor>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_core::U_INTERCHANGE_BASE_NODE_GET_LINEAR_COLOR_ATTRIBUTE,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(16)
                .cast::<crate::bindings::core_u_object::FLinearColor>()
                .swap(out_value);
        }
        unsafe { __buffer.add(32).cast::<bool>().read() }
    }
    pub fn get_int32_attribute(
        &self,
        node_attribute_key: FString,
        out_value: &mut i32,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<21>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_core::U_INTERCHANGE_BASE_NODE_GET_INT32_ATTRIBUTE,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &node_attribute_key,
                __buffer.add(0).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(out_value, __buffer.add(16).cast::<i32>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_core::U_INTERCHANGE_BASE_NODE_GET_INT32_ATTRIBUTE,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(16).cast::<i32>().swap(out_value);
        }
        unsafe { __buffer.add(20).cast::<bool>().read() }
    }
    pub fn get_icon_name(&self) -> FName {
        let mut __stack = crate::core_data::StackAlloc::<12>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_core::U_INTERCHANGE_BASE_NODE_GET_ICON_NAME,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_core::U_INTERCHANGE_BASE_NODE_GET_ICON_NAME,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<FName>().read() }
    }
    pub fn get_guid_attribute(
        &self,
        node_attribute_key: FString,
        out_value: &mut crate::bindings::core_u_object::FGuid,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<33>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_core::U_INTERCHANGE_BASE_NODE_GET_GUID_ATTRIBUTE,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &node_attribute_key,
                __buffer.add(0).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_value,
                __buffer.add(16).cast::<crate::bindings::core_u_object::FGuid>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_core::U_INTERCHANGE_BASE_NODE_GET_GUID_ATTRIBUTE,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(16)
                .cast::<crate::bindings::core_u_object::FGuid>()
                .swap(out_value);
        }
        unsafe { __buffer.add(32).cast::<bool>().read() }
    }
    pub fn get_float_attribute(
        &self,
        node_attribute_key: FString,
        out_value: &mut f32,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<21>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_core::U_INTERCHANGE_BASE_NODE_GET_FLOAT_ATTRIBUTE,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &node_attribute_key,
                __buffer.add(0).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(out_value, __buffer.add(16).cast::<f32>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_core::U_INTERCHANGE_BASE_NODE_GET_FLOAT_ATTRIBUTE,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(16).cast::<f32>().swap(out_value);
        }
        unsafe { __buffer.add(20).cast::<bool>().read() }
    }
    pub fn get_double_attribute(
        &self,
        node_attribute_key: FString,
        out_value: &mut f64,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<25>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_core::U_INTERCHANGE_BASE_NODE_GET_DOUBLE_ATTRIBUTE,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &node_attribute_key,
                __buffer.add(0).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(out_value, __buffer.add(16).cast::<f64>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_core::U_INTERCHANGE_BASE_NODE_GET_DOUBLE_ATTRIBUTE,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(16).cast::<f64>().swap(out_value);
        }
        unsafe { __buffer.add(24).cast::<bool>().read() }
    }
    pub fn get_display_label(&self) -> FString {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_core::U_INTERCHANGE_BASE_NODE_GET_DISPLAY_LABEL,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_core::U_INTERCHANGE_BASE_NODE_GET_DISPLAY_LABEL,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<FString>().read() }
    }
    pub fn get_desired_child_index(&self) -> i32 {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_core::U_INTERCHANGE_BASE_NODE_GET_DESIRED_CHILD_INDEX,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_core::U_INTERCHANGE_BASE_NODE_GET_DESIRED_CHILD_INDEX,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<i32>().read() }
    }
    pub fn get_boolean_attribute(
        &self,
        node_attribute_key: FString,
        out_value: &mut bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<18>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_core::U_INTERCHANGE_BASE_NODE_GET_BOOLEAN_ATTRIBUTE,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &node_attribute_key,
                __buffer.add(0).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(out_value, __buffer.add(16).cast::<bool>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_core::U_INTERCHANGE_BASE_NODE_GET_BOOLEAN_ATTRIBUTE,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(16).cast::<bool>().swap(out_value);
        }
        unsafe { __buffer.add(17).cast::<bool>().read() }
    }
    pub fn get_asset_name(&self) -> FString {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_core::U_INTERCHANGE_BASE_NODE_GET_ASSET_NAME,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_core::U_INTERCHANGE_BASE_NODE_GET_ASSET_NAME,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<FString>().read() }
    }
    pub fn add_vector2_attribute(
        &mut self,
        node_attribute_key: FString,
        value: &crate::bindings::core_u_object::FVector2f,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<25>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_core::U_INTERCHANGE_BASE_NODE_ADD_VECTOR2_ATTRIBUTE,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &node_attribute_key,
                __buffer.add(0).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                value,
                __buffer.add(16).cast::<crate::bindings::core_u_object::FVector2f>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_core::U_INTERCHANGE_BASE_NODE_ADD_VECTOR2_ATTRIBUTE,
                __buffer,
            )
        };
        unsafe { __buffer.add(24).cast::<bool>().read() }
    }
    pub fn add_target_node_uid(&self, asset_uid: FString) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<17>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_core::U_INTERCHANGE_BASE_NODE_ADD_TARGET_NODE_UID,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &asset_uid,
                __buffer.add(0).cast::<FString>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_core::U_INTERCHANGE_BASE_NODE_ADD_TARGET_NODE_UID,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<bool>().read() }
    }
    pub fn add_string_attribute(
        &mut self,
        node_attribute_key: FString,
        value: FString,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<33>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_core::U_INTERCHANGE_BASE_NODE_ADD_STRING_ATTRIBUTE,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &node_attribute_key,
                __buffer.add(0).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&value, __buffer.add(16).cast::<FString>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_core::U_INTERCHANGE_BASE_NODE_ADD_STRING_ATTRIBUTE,
                __buffer,
            )
        };
        unsafe { __buffer.add(32).cast::<bool>().read() }
    }
    pub fn add_linear_color_attribute(
        &mut self,
        node_attribute_key: FString,
        value: &crate::bindings::core_u_object::FLinearColor,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<33>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_core::U_INTERCHANGE_BASE_NODE_ADD_LINEAR_COLOR_ATTRIBUTE,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &node_attribute_key,
                __buffer.add(0).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                value,
                __buffer.add(16).cast::<crate::bindings::core_u_object::FLinearColor>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_core::U_INTERCHANGE_BASE_NODE_ADD_LINEAR_COLOR_ATTRIBUTE,
                __buffer,
            )
        };
        unsafe { __buffer.add(32).cast::<bool>().read() }
    }
    pub fn add_int32_attribute(
        &mut self,
        node_attribute_key: FString,
        value: &i32,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<21>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_core::U_INTERCHANGE_BASE_NODE_ADD_INT32_ATTRIBUTE,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &node_attribute_key,
                __buffer.add(0).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(value, __buffer.add(16).cast::<i32>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_core::U_INTERCHANGE_BASE_NODE_ADD_INT32_ATTRIBUTE,
                __buffer,
            )
        };
        unsafe { __buffer.add(20).cast::<bool>().read() }
    }
    pub fn add_guid_attribute(
        &mut self,
        node_attribute_key: FString,
        value: &crate::bindings::core_u_object::FGuid,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<33>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_core::U_INTERCHANGE_BASE_NODE_ADD_GUID_ATTRIBUTE,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &node_attribute_key,
                __buffer.add(0).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                value,
                __buffer.add(16).cast::<crate::bindings::core_u_object::FGuid>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_core::U_INTERCHANGE_BASE_NODE_ADD_GUID_ATTRIBUTE,
                __buffer,
            )
        };
        unsafe { __buffer.add(32).cast::<bool>().read() }
    }
    pub fn add_float_attribute(
        &mut self,
        node_attribute_key: FString,
        value: &f32,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<21>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_core::U_INTERCHANGE_BASE_NODE_ADD_FLOAT_ATTRIBUTE,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &node_attribute_key,
                __buffer.add(0).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(value, __buffer.add(16).cast::<f32>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_core::U_INTERCHANGE_BASE_NODE_ADD_FLOAT_ATTRIBUTE,
                __buffer,
            )
        };
        unsafe { __buffer.add(20).cast::<bool>().read() }
    }
    pub fn add_double_attribute(
        &mut self,
        node_attribute_key: FString,
        value: &f64,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<25>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_core::U_INTERCHANGE_BASE_NODE_ADD_DOUBLE_ATTRIBUTE,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &node_attribute_key,
                __buffer.add(0).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(value, __buffer.add(16).cast::<f64>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_core::U_INTERCHANGE_BASE_NODE_ADD_DOUBLE_ATTRIBUTE,
                __buffer,
            )
        };
        unsafe { __buffer.add(24).cast::<bool>().read() }
    }
    pub fn add_boolean_attribute(
        &mut self,
        node_attribute_key: FString,
        value: &bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<18>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_core::U_INTERCHANGE_BASE_NODE_ADD_BOOLEAN_ATTRIBUTE,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &node_attribute_key,
                __buffer.add(0).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(value, __buffer.add(16).cast::<bool>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_core::U_INTERCHANGE_BASE_NODE_ADD_BOOLEAN_ATTRIBUTE,
                __buffer,
            )
        };
        unsafe { __buffer.add(17).cast::<bool>().read() }
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
    pub fn set_node_parent_uid(
        &mut self,
        node_unique_id: FString,
        new_parent_node_uid: FString,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<33>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_core::U_INTERCHANGE_BASE_NODE_CONTAINER_SET_NODE_PARENT_UID,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &node_unique_id,
                __buffer.add(0).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &new_parent_node_uid,
                __buffer.add(16).cast::<FString>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_core::U_INTERCHANGE_BASE_NODE_CONTAINER_SET_NODE_PARENT_UID,
                __buffer,
            )
        };
        unsafe { __buffer.add(32).cast::<bool>().read() }
    }
    pub fn set_node_desired_child_index(
        &mut self,
        node_unique_id: FString,
        new_node_desired_child_index: &i32,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<21>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_core::U_INTERCHANGE_BASE_NODE_CONTAINER_SET_NODE_DESIRED_CHILD_INDEX,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &node_unique_id,
                __buffer.add(0).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                new_node_desired_child_index,
                __buffer.add(16).cast::<i32>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_core::U_INTERCHANGE_BASE_NODE_CONTAINER_SET_NODE_DESIRED_CHILD_INDEX,
                __buffer,
            )
        };
        unsafe { __buffer.add(20).cast::<bool>().read() }
    }
    pub fn set_namespace(
        &mut self,
        namespace: FString,
        target_class: TSubclassOf<crate::bindings::core_u_object::UObject>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_core::U_INTERCHANGE_BASE_NODE_CONTAINER_SET_NAMESPACE,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &namespace,
                __buffer.add(0).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &target_class,
                __buffer
                    .add(16)
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
                crate::bindings::interchange_core::U_INTERCHANGE_BASE_NODE_CONTAINER_SET_NAMESPACE,
                __buffer,
            )
        };
    }
    pub fn save_to_file(&mut self, filename: FString) {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_core::U_INTERCHANGE_BASE_NODE_CONTAINER_SAVE_TO_FILE,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &filename,
                __buffer.add(0).cast::<FString>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_core::U_INTERCHANGE_BASE_NODE_CONTAINER_SAVE_TO_FILE,
                __buffer,
            )
        };
    }
    pub fn reset_children_cache(&mut self) {
        let mut __stack = crate::core_data::StackAlloc::<0>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_core::U_INTERCHANGE_BASE_NODE_CONTAINER_RESET_CHILDREN_CACHE,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_core::U_INTERCHANGE_BASE_NODE_CONTAINER_RESET_CHILDREN_CACHE,
                __buffer,
            )
        };
    }
    pub fn reset(&mut self) {
        let mut __stack = crate::core_data::StackAlloc::<0>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_core::U_INTERCHANGE_BASE_NODE_CONTAINER_RESET,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_core::U_INTERCHANGE_BASE_NODE_CONTAINER_RESET,
                __buffer,
            )
        };
    }
    pub fn replace_node(
        &mut self,
        node_unique_id: FString,
        new_node: UPtr<UInterchangeFactoryBaseNode>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_core::U_INTERCHANGE_BASE_NODE_CONTAINER_REPLACE_NODE,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &node_unique_id,
                __buffer.add(0).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &new_node,
                __buffer.add(16).cast::<UPtr<UInterchangeFactoryBaseNode>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_core::U_INTERCHANGE_BASE_NODE_CONTAINER_REPLACE_NODE,
                __buffer,
            )
        };
    }
    pub fn remove_node(&mut self, node_unique_id: FString) {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_core::U_INTERCHANGE_BASE_NODE_CONTAINER_REMOVE_NODE,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &node_unique_id,
                __buffer.add(0).cast::<FString>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_core::U_INTERCHANGE_BASE_NODE_CONTAINER_REMOVE_NODE,
                __buffer,
            )
        };
    }
    pub fn load_from_file(&mut self, filename: FString) {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_core::U_INTERCHANGE_BASE_NODE_CONTAINER_LOAD_FROM_FILE,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &filename,
                __buffer.add(0).cast::<FString>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_core::U_INTERCHANGE_BASE_NODE_CONTAINER_LOAD_FROM_FILE,
                __buffer,
            )
        };
    }
    pub fn is_node_uid_valid(&self, node_unique_id: FString) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<17>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_core::U_INTERCHANGE_BASE_NODE_CONTAINER_IS_NODE_UID_VALID,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &node_unique_id,
                __buffer.add(0).cast::<FString>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_core::U_INTERCHANGE_BASE_NODE_CONTAINER_IS_NODE_UID_VALID,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<bool>().read() }
    }
    pub fn get_roots(&self, root_nodes: &mut TArray<FString>) {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_core::U_INTERCHANGE_BASE_NODE_CONTAINER_GET_ROOTS,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                root_nodes,
                __buffer.add(0).cast::<TArray<FString>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_core::U_INTERCHANGE_BASE_NODE_CONTAINER_GET_ROOTS,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<TArray<FString>>().swap(root_nodes);
        }
    }
    pub fn get_nodes(
        &self,
        class_node: TSubclassOf<crate::bindings::core_u_object::UObject>,
        out_nodes: &mut TArray<FString>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_core::U_INTERCHANGE_BASE_NODE_CONTAINER_GET_NODES,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &class_node,
                __buffer
                    .add(0)
                    .cast::<TSubclassOf<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_nodes,
                __buffer.add(8).cast::<TArray<FString>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_core::U_INTERCHANGE_BASE_NODE_CONTAINER_GET_NODES,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(8).cast::<TArray<FString>>().swap(out_nodes);
        }
    }
    pub fn get_node_children_uids(&self, node_unique_id: FString) -> TArray<FString> {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_core::U_INTERCHANGE_BASE_NODE_CONTAINER_GET_NODE_CHILDREN_UIDS,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &node_unique_id,
                __buffer.add(0).cast::<FString>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_core::U_INTERCHANGE_BASE_NODE_CONTAINER_GET_NODE_CHILDREN_UIDS,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<TArray<FString>>().read() }
    }
    pub fn get_node_children_count(&self, node_unique_id: FString) -> i32 {
        let mut __stack = crate::core_data::StackAlloc::<20>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_core::U_INTERCHANGE_BASE_NODE_CONTAINER_GET_NODE_CHILDREN_COUNT,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &node_unique_id,
                __buffer.add(0).cast::<FString>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_core::U_INTERCHANGE_BASE_NODE_CONTAINER_GET_NODE_CHILDREN_COUNT,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<i32>().read() }
    }
    pub fn get_node_children(
        &mut self,
        node_unique_id: FString,
        child_index: i32,
    ) -> UPtr<UInterchangeBaseNode> {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_core::U_INTERCHANGE_BASE_NODE_CONTAINER_GET_NODE_CHILDREN,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &node_unique_id,
                __buffer.add(0).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &child_index,
                __buffer.add(16).cast::<i32>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_core::U_INTERCHANGE_BASE_NODE_CONTAINER_GET_NODE_CHILDREN,
                __buffer,
            )
        };
        unsafe { __buffer.add(24).cast::<UPtr<UInterchangeBaseNode>>().read() }
    }
    pub fn get_node(&self, node_unique_id: FString) -> UPtr<UInterchangeBaseNode> {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_core::U_INTERCHANGE_BASE_NODE_CONTAINER_GET_NODE,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &node_unique_id,
                __buffer.add(0).cast::<FString>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_core::U_INTERCHANGE_BASE_NODE_CONTAINER_GET_NODE,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<UPtr<UInterchangeBaseNode>>().read() }
    }
    pub fn get_is_ancestor(
        &self,
        node_unique_id: FString,
        ancestor_uid: FString,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<33>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_core::U_INTERCHANGE_BASE_NODE_CONTAINER_GET_IS_ANCESTOR,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &node_unique_id,
                __buffer.add(0).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &ancestor_uid,
                __buffer.add(16).cast::<FString>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_core::U_INTERCHANGE_BASE_NODE_CONTAINER_GET_IS_ANCESTOR,
                __buffer,
            )
        };
        unsafe { __buffer.add(32).cast::<bool>().read() }
    }
    pub fn get_factory_node(
        &self,
        node_unique_id: FString,
    ) -> UPtr<UInterchangeFactoryBaseNode> {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_core::U_INTERCHANGE_BASE_NODE_CONTAINER_GET_FACTORY_NODE,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &node_unique_id,
                __buffer.add(0).cast::<FString>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_core::U_INTERCHANGE_BASE_NODE_CONTAINER_GET_FACTORY_NODE,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<UPtr<UInterchangeFactoryBaseNode>>().read() }
    }
    pub fn compute_children_cache(&mut self) {
        let mut __stack = crate::core_data::StackAlloc::<0>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_core::U_INTERCHANGE_BASE_NODE_CONTAINER_COMPUTE_CHILDREN_CACHE,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_core::U_INTERCHANGE_BASE_NODE_CONTAINER_COMPUTE_CHILDREN_CACHE,
                __buffer,
            )
        };
    }
    pub fn clear_node_parent_uid(&mut self, node_unique_id: FString) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<17>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_core::U_INTERCHANGE_BASE_NODE_CONTAINER_CLEAR_NODE_PARENT_UID,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &node_unique_id,
                __buffer.add(0).cast::<FString>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_core::U_INTERCHANGE_BASE_NODE_CONTAINER_CLEAR_NODE_PARENT_UID,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<bool>().read() }
    }
    pub fn add_node(&mut self, node: UPtr<UInterchangeBaseNode>) -> FString {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_core::U_INTERCHANGE_BASE_NODE_CONTAINER_ADD_NODE,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &node,
                __buffer.add(0).cast::<UPtr<UInterchangeBaseNode>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_core::U_INTERCHANGE_BASE_NODE_CONTAINER_ADD_NODE,
                __buffer,
            )
        };
        unsafe { __buffer.add(8).cast::<FString>().read() }
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
    pub fn unset_skip_node_import(&mut self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_core::U_INTERCHANGE_FACTORY_BASE_NODE_UNSET_SKIP_NODE_IMPORT,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_core::U_INTERCHANGE_FACTORY_BASE_NODE_UNSET_SKIP_NODE_IMPORT,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn unset_force_node_reimport(&mut self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_core::U_INTERCHANGE_FACTORY_BASE_NODE_UNSET_FORCE_NODE_REIMPORT,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_core::U_INTERCHANGE_FACTORY_BASE_NODE_UNSET_FORCE_NODE_REIMPORT,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn should_skip_node_import(&self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_core::U_INTERCHANGE_FACTORY_BASE_NODE_SHOULD_SKIP_NODE_IMPORT,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_core::U_INTERCHANGE_FACTORY_BASE_NODE_SHOULD_SKIP_NODE_IMPORT,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn should_force_node_reimport(&self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_core::U_INTERCHANGE_FACTORY_BASE_NODE_SHOULD_FORCE_NODE_REIMPORT,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_core::U_INTERCHANGE_FACTORY_BASE_NODE_SHOULD_FORCE_NODE_REIMPORT,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn set_skip_node_import(&mut self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_core::U_INTERCHANGE_FACTORY_BASE_NODE_SET_SKIP_NODE_IMPORT,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_core::U_INTERCHANGE_FACTORY_BASE_NODE_SET_SKIP_NODE_IMPORT,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn set_reimport_strategy_flags(
        &mut self,
        reimport_strategy_flags: &EReimportStrategyFlags,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<2>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_core::U_INTERCHANGE_FACTORY_BASE_NODE_SET_REIMPORT_STRATEGY_FLAGS,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                reimport_strategy_flags,
                __buffer.add(0).cast::<EReimportStrategyFlags>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_core::U_INTERCHANGE_FACTORY_BASE_NODE_SET_REIMPORT_STRATEGY_FLAGS,
                __buffer,
            )
        };
        unsafe { __buffer.add(1).cast::<bool>().read() }
    }
    pub fn set_force_node_reimport(&mut self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_core::U_INTERCHANGE_FACTORY_BASE_NODE_SET_FORCE_NODE_REIMPORT,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_core::U_INTERCHANGE_FACTORY_BASE_NODE_SET_FORCE_NODE_REIMPORT,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn set_custom_sub_path(&mut self, attribute_value: FString) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<17>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_core::U_INTERCHANGE_FACTORY_BASE_NODE_SET_CUSTOM_SUB_PATH,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &attribute_value,
                __buffer.add(0).cast::<FString>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_core::U_INTERCHANGE_FACTORY_BASE_NODE_SET_CUSTOM_SUB_PATH,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<bool>().read() }
    }
    pub fn set_custom_reference_object(
        &mut self,
        attribute_value: &crate::bindings::core_u_object::FSoftObjectPath,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<41>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_core::U_INTERCHANGE_FACTORY_BASE_NODE_SET_CUSTOM_REFERENCE_OBJECT,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                attribute_value,
                __buffer
                    .add(0)
                    .cast::<crate::bindings::core_u_object::FSoftObjectPath>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_core::U_INTERCHANGE_FACTORY_BASE_NODE_SET_CUSTOM_REFERENCE_OBJECT,
                __buffer,
            )
        };
        unsafe { __buffer.add(40).cast::<bool>().read() }
    }
    pub fn set_custom_level_uid(&mut self, attribute_value: FString) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<17>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_core::U_INTERCHANGE_FACTORY_BASE_NODE_SET_CUSTOM_LEVEL_UID,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &attribute_value,
                __buffer.add(0).cast::<FString>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_core::U_INTERCHANGE_FACTORY_BASE_NODE_SET_CUSTOM_LEVEL_UID,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<bool>().read() }
    }
    pub fn remove_factory_dependency_uid(&mut self, dependency_uid: FString) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<17>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_core::U_INTERCHANGE_FACTORY_BASE_NODE_REMOVE_FACTORY_DEPENDENCY_UID,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &dependency_uid,
                __buffer.add(0).cast::<FString>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_core::U_INTERCHANGE_FACTORY_BASE_NODE_REMOVE_FACTORY_DEPENDENCY_UID,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<bool>().read() }
    }
    pub fn is_runtime_import_allowed(&self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_core::U_INTERCHANGE_FACTORY_BASE_NODE_IS_RUNTIME_IMPORT_ALLOWED,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_core::U_INTERCHANGE_FACTORY_BASE_NODE_IS_RUNTIME_IMPORT_ALLOWED,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn get_reimport_strategy_flags(&self) -> EReimportStrategyFlags {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_core::U_INTERCHANGE_FACTORY_BASE_NODE_GET_REIMPORT_STRATEGY_FLAGS,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_core::U_INTERCHANGE_FACTORY_BASE_NODE_GET_REIMPORT_STRATEGY_FLAGS,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<EReimportStrategyFlags>().read() }
    }
    pub fn get_object_class(
        &self,
    ) -> TSubclassOf<crate::bindings::core_u_object::UObject> {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_core::U_INTERCHANGE_FACTORY_BASE_NODE_GET_OBJECT_CLASS,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_core::U_INTERCHANGE_FACTORY_BASE_NODE_GET_OBJECT_CLASS,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(0)
                .cast::<TSubclassOf<crate::bindings::core_u_object::UObject>>()
                .read()
        }
    }
    pub fn get_factory_dependency(&self, index: i32, out_dependency: &mut FString) {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_core::U_INTERCHANGE_FACTORY_BASE_NODE_GET_FACTORY_DEPENDENCY,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&index, __buffer.add(0).cast::<i32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_dependency,
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
                crate::bindings::interchange_core::U_INTERCHANGE_FACTORY_BASE_NODE_GET_FACTORY_DEPENDENCY,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(8).cast::<FString>().swap(out_dependency);
        }
    }
    pub fn get_factory_dependencies_count(&self) -> i32 {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_core::U_INTERCHANGE_FACTORY_BASE_NODE_GET_FACTORY_DEPENDENCIES_COUNT,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_core::U_INTERCHANGE_FACTORY_BASE_NODE_GET_FACTORY_DEPENDENCIES_COUNT,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<i32>().read() }
    }
    pub fn get_factory_dependencies(&self, out_dependencies: &mut TArray<FString>) {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_core::U_INTERCHANGE_FACTORY_BASE_NODE_GET_FACTORY_DEPENDENCIES,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_dependencies,
                __buffer.add(0).cast::<TArray<FString>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_core::U_INTERCHANGE_FACTORY_BASE_NODE_GET_FACTORY_DEPENDENCIES,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<TArray<FString>>().swap(out_dependencies);
        }
    }
    pub fn get_custom_sub_path(&self, attribute_value: &mut FString) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<17>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_core::U_INTERCHANGE_FACTORY_BASE_NODE_GET_CUSTOM_SUB_PATH,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                attribute_value,
                __buffer.add(0).cast::<FString>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_core::U_INTERCHANGE_FACTORY_BASE_NODE_GET_CUSTOM_SUB_PATH,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<FString>().swap(attribute_value);
        }
        unsafe { __buffer.add(16).cast::<bool>().read() }
    }
    pub fn get_custom_reference_object(
        &self,
        attribute_value: &mut crate::bindings::core_u_object::FSoftObjectPath,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<41>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_core::U_INTERCHANGE_FACTORY_BASE_NODE_GET_CUSTOM_REFERENCE_OBJECT,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                attribute_value,
                __buffer
                    .add(0)
                    .cast::<crate::bindings::core_u_object::FSoftObjectPath>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_core::U_INTERCHANGE_FACTORY_BASE_NODE_GET_CUSTOM_REFERENCE_OBJECT,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(0)
                .cast::<crate::bindings::core_u_object::FSoftObjectPath>()
                .swap(attribute_value);
        }
        unsafe { __buffer.add(40).cast::<bool>().read() }
    }
    pub fn get_custom_level_uid(&self, attribute_value: &mut FString) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<17>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_core::U_INTERCHANGE_FACTORY_BASE_NODE_GET_CUSTOM_LEVEL_UID,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                attribute_value,
                __buffer.add(0).cast::<FString>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_core::U_INTERCHANGE_FACTORY_BASE_NODE_GET_CUSTOM_LEVEL_UID,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<FString>().swap(attribute_value);
        }
        unsafe { __buffer.add(16).cast::<bool>().read() }
    }
    pub fn add_factory_dependency_uid(&mut self, dependency_uid: FString) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<17>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_core::U_INTERCHANGE_FACTORY_BASE_NODE_ADD_FACTORY_DEPENDENCY_UID,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &dependency_uid,
                __buffer.add(0).cast::<FString>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_core::U_INTERCHANGE_FACTORY_BASE_NODE_ADD_FACTORY_DEPENDENCY_UID,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<bool>().read() }
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
    pub fn set_extra_information(&mut self, name: FString, value: FString) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<33>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_core::U_INTERCHANGE_SOURCE_NODE_SET_EXTRA_INFORMATION,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&name, __buffer.add(0).cast::<FString>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&value, __buffer.add(16).cast::<FString>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_core::U_INTERCHANGE_SOURCE_NODE_SET_EXTRA_INFORMATION,
                __buffer,
            )
        };
        unsafe { __buffer.add(32).cast::<bool>().read() }
    }
    pub fn set_custom_use_legacy_skeletal_mesh_bake_transform(
        &mut self,
        attribute_value: &bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<2>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_core::U_INTERCHANGE_SOURCE_NODE_SET_CUSTOM_USE_LEGACY_SKELETAL_MESH_BAKE_TRANSFORM,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                attribute_value,
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
                crate::bindings::interchange_core::U_INTERCHANGE_SOURCE_NODE_SET_CUSTOM_USE_LEGACY_SKELETAL_MESH_BAKE_TRANSFORM,
                __buffer,
            )
        };
        unsafe { __buffer.add(1).cast::<bool>().read() }
    }
    pub fn set_custom_use_asset_type_sub_path_suffix(&mut self, suffix: &bool) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<2>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_core::U_INTERCHANGE_SOURCE_NODE_SET_CUSTOM_USE_ASSET_TYPE_SUB_PATH_SUFFIX,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(suffix, __buffer.add(0).cast::<bool>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_core::U_INTERCHANGE_SOURCE_NODE_SET_CUSTOM_USE_ASSET_TYPE_SUB_PATH_SUFFIX,
                __buffer,
            )
        };
        unsafe { __buffer.add(1).cast::<bool>().read() }
    }
    pub fn set_custom_sub_path_prefix(&mut self, prefix: FString) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<17>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_core::U_INTERCHANGE_SOURCE_NODE_SET_CUSTOM_SUB_PATH_PREFIX,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&prefix, __buffer.add(0).cast::<FString>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_core::U_INTERCHANGE_SOURCE_NODE_SET_CUSTOM_SUB_PATH_PREFIX,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<bool>().read() }
    }
    pub fn set_custom_source_timeline_start(&mut self, attribute_value: &f64) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<9>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_core::U_INTERCHANGE_SOURCE_NODE_SET_CUSTOM_SOURCE_TIMELINE_START,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                attribute_value,
                __buffer.add(0).cast::<f64>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_core::U_INTERCHANGE_SOURCE_NODE_SET_CUSTOM_SOURCE_TIMELINE_START,
                __buffer,
            )
        };
        unsafe { __buffer.add(8).cast::<bool>().read() }
    }
    pub fn set_custom_source_timeline_end(&mut self, attribute_value: &f64) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<9>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_core::U_INTERCHANGE_SOURCE_NODE_SET_CUSTOM_SOURCE_TIMELINE_END,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                attribute_value,
                __buffer.add(0).cast::<f64>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_core::U_INTERCHANGE_SOURCE_NODE_SET_CUSTOM_SOURCE_TIMELINE_END,
                __buffer,
            )
        };
        unsafe { __buffer.add(8).cast::<bool>().read() }
    }
    pub fn set_custom_source_frame_rate_numerator(
        &mut self,
        attribute_value: &i32,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<5>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_core::U_INTERCHANGE_SOURCE_NODE_SET_CUSTOM_SOURCE_FRAME_RATE_NUMERATOR,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                attribute_value,
                __buffer.add(0).cast::<i32>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_core::U_INTERCHANGE_SOURCE_NODE_SET_CUSTOM_SOURCE_FRAME_RATE_NUMERATOR,
                __buffer,
            )
        };
        unsafe { __buffer.add(4).cast::<bool>().read() }
    }
    pub fn set_custom_source_frame_rate_denominator(
        &mut self,
        attribute_value: &i32,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<5>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_core::U_INTERCHANGE_SOURCE_NODE_SET_CUSTOM_SOURCE_FRAME_RATE_DENOMINATOR,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                attribute_value,
                __buffer.add(0).cast::<i32>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_core::U_INTERCHANGE_SOURCE_NODE_SET_CUSTOM_SOURCE_FRAME_RATE_DENOMINATOR,
                __buffer,
            )
        };
        unsafe { __buffer.add(4).cast::<bool>().read() }
    }
    pub fn set_custom_skeletal_mesh_front_axis(&mut self, attribute_value: u8) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<2>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_core::U_INTERCHANGE_SOURCE_NODE_SET_CUSTOM_SKELETAL_MESH_FRONT_AXIS,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &attribute_value,
                __buffer.add(0).cast::<u8>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_core::U_INTERCHANGE_SOURCE_NODE_SET_CUSTOM_SKELETAL_MESH_FRONT_AXIS,
                __buffer,
            )
        };
        unsafe { __buffer.add(1).cast::<bool>().read() }
    }
    pub fn set_custom_reimport_strategy_flags(&mut self, strategy_flag: u8) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<2>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_core::U_INTERCHANGE_SOURCE_NODE_SET_CUSTOM_REIMPORT_STRATEGY_FLAGS,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &strategy_flag,
                __buffer.add(0).cast::<u8>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_core::U_INTERCHANGE_SOURCE_NODE_SET_CUSTOM_REIMPORT_STRATEGY_FLAGS,
                __buffer,
            )
        };
        unsafe { __buffer.add(1).cast::<bool>().read() }
    }
    pub fn set_custom_nanite_triangle_threshold(
        &mut self,
        min_num_triangles: &i64,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<9>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_core::U_INTERCHANGE_SOURCE_NODE_SET_CUSTOM_NANITE_TRIANGLE_THRESHOLD,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                min_num_triangles,
                __buffer.add(0).cast::<i64>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_core::U_INTERCHANGE_SOURCE_NODE_SET_CUSTOM_NANITE_TRIANGLE_THRESHOLD,
                __buffer,
            )
        };
        unsafe { __buffer.add(8).cast::<bool>().read() }
    }
    pub fn set_custom_import_unused_material(&mut self, attribute_value: &bool) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<2>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_core::U_INTERCHANGE_SOURCE_NODE_SET_CUSTOM_IMPORT_UNUSED_MATERIAL,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                attribute_value,
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
                crate::bindings::interchange_core::U_INTERCHANGE_SOURCE_NODE_SET_CUSTOM_IMPORT_UNUSED_MATERIAL,
                __buffer,
            )
        };
        unsafe { __buffer.add(1).cast::<bool>().read() }
    }
    pub fn set_custom_axis_conversion_inverse_transform(
        &mut self,
        axis_conversion_inverse_transform: &crate::bindings::core_u_object::FTransform,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<97>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_core::U_INTERCHANGE_SOURCE_NODE_SET_CUSTOM_AXIS_CONVERSION_INVERSE_TRANSFORM,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                axis_conversion_inverse_transform,
                __buffer.add(0).cast::<crate::bindings::core_u_object::FTransform>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_core::U_INTERCHANGE_SOURCE_NODE_SET_CUSTOM_AXIS_CONVERSION_INVERSE_TRANSFORM,
                __buffer,
            )
        };
        unsafe { __buffer.add(96).cast::<bool>().read() }
    }
    pub fn set_custom_animated_time_start(&mut self, attribute_value: &f64) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<9>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_core::U_INTERCHANGE_SOURCE_NODE_SET_CUSTOM_ANIMATED_TIME_START,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                attribute_value,
                __buffer.add(0).cast::<f64>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_core::U_INTERCHANGE_SOURCE_NODE_SET_CUSTOM_ANIMATED_TIME_START,
                __buffer,
            )
        };
        unsafe { __buffer.add(8).cast::<bool>().read() }
    }
    pub fn set_custom_animated_time_end(&mut self, attribute_value: &f64) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<9>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_core::U_INTERCHANGE_SOURCE_NODE_SET_CUSTOM_ANIMATED_TIME_END,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                attribute_value,
                __buffer.add(0).cast::<f64>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_core::U_INTERCHANGE_SOURCE_NODE_SET_CUSTOM_ANIMATED_TIME_END,
                __buffer,
            )
        };
        unsafe { __buffer.add(8).cast::<bool>().read() }
    }
    pub fn set_custom_allow_scene_root_as_joint(
        &mut self,
        b_allow_scene_root_as_joint: &bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<2>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_core::U_INTERCHANGE_SOURCE_NODE_SET_CUSTOM_ALLOW_SCENE_ROOT_AS_JOINT,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                b_allow_scene_root_as_joint,
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
                crate::bindings::interchange_core::U_INTERCHANGE_SOURCE_NODE_SET_CUSTOM_ALLOW_SCENE_ROOT_AS_JOINT,
                __buffer,
            )
        };
        unsafe { __buffer.add(1).cast::<bool>().read() }
    }
    pub fn remove_extra_information(&mut self, name: FString) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<17>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_core::U_INTERCHANGE_SOURCE_NODE_REMOVE_EXTRA_INFORMATION,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&name, __buffer.add(0).cast::<FString>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_core::U_INTERCHANGE_SOURCE_NODE_REMOVE_EXTRA_INFORMATION,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<bool>().read() }
    }
    pub fn initialize_source_node(
        &mut self,
        unique_id: FString,
        display_label: FString,
        node_container: UPtr<UInterchangeBaseNodeContainer>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<40>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_core::U_INTERCHANGE_SOURCE_NODE_INITIALIZE_SOURCE_NODE,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &unique_id,
                __buffer.add(0).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &display_label,
                __buffer.add(16).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &node_container,
                __buffer.add(32).cast::<UPtr<UInterchangeBaseNodeContainer>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_core::U_INTERCHANGE_SOURCE_NODE_INITIALIZE_SOURCE_NODE,
                __buffer,
            )
        };
    }
    pub fn get_unique_instance(
        node_container: UPtr<UInterchangeBaseNodeContainer>,
    ) -> UPtr<UInterchangeSourceNode> {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_core::U_INTERCHANGE_SOURCE_NODE_GET_UNIQUE_INSTANCE,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &node_container,
                __buffer.add(0).cast::<UPtr<UInterchangeBaseNodeContainer>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::interchange_core::UInterchangeSourceNode::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_core::U_INTERCHANGE_SOURCE_NODE_GET_UNIQUE_INSTANCE,
                __buffer,
            )
        };
        unsafe { __buffer.add(8).cast::<UPtr<UInterchangeSourceNode>>().read() }
    }
    pub fn get_extra_information(
        &self,
        out_extra_information: &mut TMap<FString, FString>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<80>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_core::U_INTERCHANGE_SOURCE_NODE_GET_EXTRA_INFORMATION,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_extra_information,
                __buffer.add(0).cast::<TMap<FString, FString>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_core::U_INTERCHANGE_SOURCE_NODE_GET_EXTRA_INFORMATION,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<TMap<FString, FString>>().swap(out_extra_information);
        }
    }
    pub fn get_custom_use_legacy_skeletal_mesh_bake_transform(
        &self,
        attribute_value: &mut bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<2>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_core::U_INTERCHANGE_SOURCE_NODE_GET_CUSTOM_USE_LEGACY_SKELETAL_MESH_BAKE_TRANSFORM,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                attribute_value,
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
                crate::bindings::interchange_core::U_INTERCHANGE_SOURCE_NODE_GET_CUSTOM_USE_LEGACY_SKELETAL_MESH_BAKE_TRANSFORM,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<bool>().swap(attribute_value);
        }
        unsafe { __buffer.add(1).cast::<bool>().read() }
    }
    pub fn get_custom_use_asset_type_sub_path_suffix(&self, suffix: &mut bool) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<2>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_core::U_INTERCHANGE_SOURCE_NODE_GET_CUSTOM_USE_ASSET_TYPE_SUB_PATH_SUFFIX,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(suffix, __buffer.add(0).cast::<bool>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_core::U_INTERCHANGE_SOURCE_NODE_GET_CUSTOM_USE_ASSET_TYPE_SUB_PATH_SUFFIX,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<bool>().swap(suffix);
        }
        unsafe { __buffer.add(1).cast::<bool>().read() }
    }
    pub fn get_custom_sub_path_prefix(&self, prefix: &mut FString) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<17>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_core::U_INTERCHANGE_SOURCE_NODE_GET_CUSTOM_SUB_PATH_PREFIX,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(prefix, __buffer.add(0).cast::<FString>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_core::U_INTERCHANGE_SOURCE_NODE_GET_CUSTOM_SUB_PATH_PREFIX,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<FString>().swap(prefix);
        }
        unsafe { __buffer.add(16).cast::<bool>().read() }
    }
    pub fn get_custom_source_timeline_start(&self, attribute_value: &mut f64) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<9>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_core::U_INTERCHANGE_SOURCE_NODE_GET_CUSTOM_SOURCE_TIMELINE_START,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                attribute_value,
                __buffer.add(0).cast::<f64>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_core::U_INTERCHANGE_SOURCE_NODE_GET_CUSTOM_SOURCE_TIMELINE_START,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<f64>().swap(attribute_value);
        }
        unsafe { __buffer.add(8).cast::<bool>().read() }
    }
    pub fn get_custom_source_timeline_end(&self, attribute_value: &mut f64) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<9>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_core::U_INTERCHANGE_SOURCE_NODE_GET_CUSTOM_SOURCE_TIMELINE_END,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                attribute_value,
                __buffer.add(0).cast::<f64>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_core::U_INTERCHANGE_SOURCE_NODE_GET_CUSTOM_SOURCE_TIMELINE_END,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<f64>().swap(attribute_value);
        }
        unsafe { __buffer.add(8).cast::<bool>().read() }
    }
    pub fn get_custom_source_frame_rate_numerator(
        &self,
        attribute_value: &mut i32,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<5>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_core::U_INTERCHANGE_SOURCE_NODE_GET_CUSTOM_SOURCE_FRAME_RATE_NUMERATOR,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                attribute_value,
                __buffer.add(0).cast::<i32>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_core::U_INTERCHANGE_SOURCE_NODE_GET_CUSTOM_SOURCE_FRAME_RATE_NUMERATOR,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<i32>().swap(attribute_value);
        }
        unsafe { __buffer.add(4).cast::<bool>().read() }
    }
    pub fn get_custom_source_frame_rate_denominator(
        &self,
        attribute_value: &mut i32,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<5>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_core::U_INTERCHANGE_SOURCE_NODE_GET_CUSTOM_SOURCE_FRAME_RATE_DENOMINATOR,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                attribute_value,
                __buffer.add(0).cast::<i32>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_core::U_INTERCHANGE_SOURCE_NODE_GET_CUSTOM_SOURCE_FRAME_RATE_DENOMINATOR,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<i32>().swap(attribute_value);
        }
        unsafe { __buffer.add(4).cast::<bool>().read() }
    }
    pub fn get_custom_skeletal_mesh_front_axis(&self, attribute_value: &mut u8) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<2>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_core::U_INTERCHANGE_SOURCE_NODE_GET_CUSTOM_SKELETAL_MESH_FRONT_AXIS,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                attribute_value,
                __buffer.add(0).cast::<u8>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_core::U_INTERCHANGE_SOURCE_NODE_GET_CUSTOM_SKELETAL_MESH_FRONT_AXIS,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<u8>().swap(attribute_value);
        }
        unsafe { __buffer.add(1).cast::<bool>().read() }
    }
    pub fn get_custom_reimport_strategy_flags(&self, strategy_flag: &mut u8) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<2>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_core::U_INTERCHANGE_SOURCE_NODE_GET_CUSTOM_REIMPORT_STRATEGY_FLAGS,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                strategy_flag,
                __buffer.add(0).cast::<u8>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_core::U_INTERCHANGE_SOURCE_NODE_GET_CUSTOM_REIMPORT_STRATEGY_FLAGS,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<u8>().swap(strategy_flag);
        }
        unsafe { __buffer.add(1).cast::<bool>().read() }
    }
    pub fn get_custom_nanite_triangle_threshold(
        &self,
        min_num_triangles: &mut i64,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<9>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_core::U_INTERCHANGE_SOURCE_NODE_GET_CUSTOM_NANITE_TRIANGLE_THRESHOLD,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                min_num_triangles,
                __buffer.add(0).cast::<i64>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_core::U_INTERCHANGE_SOURCE_NODE_GET_CUSTOM_NANITE_TRIANGLE_THRESHOLD,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<i64>().swap(min_num_triangles);
        }
        unsafe { __buffer.add(8).cast::<bool>().read() }
    }
    pub fn get_custom_import_unused_material(&self, attribute_value: &mut bool) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<2>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_core::U_INTERCHANGE_SOURCE_NODE_GET_CUSTOM_IMPORT_UNUSED_MATERIAL,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                attribute_value,
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
                crate::bindings::interchange_core::U_INTERCHANGE_SOURCE_NODE_GET_CUSTOM_IMPORT_UNUSED_MATERIAL,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<bool>().swap(attribute_value);
        }
        unsafe { __buffer.add(1).cast::<bool>().read() }
    }
    pub fn get_custom_axis_conversion_inverse_transform(
        &self,
        axis_conversion_inverse_transform: &mut crate::bindings::core_u_object::FTransform,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<97>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_core::U_INTERCHANGE_SOURCE_NODE_GET_CUSTOM_AXIS_CONVERSION_INVERSE_TRANSFORM,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                axis_conversion_inverse_transform,
                __buffer.add(0).cast::<crate::bindings::core_u_object::FTransform>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_core::U_INTERCHANGE_SOURCE_NODE_GET_CUSTOM_AXIS_CONVERSION_INVERSE_TRANSFORM,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(0)
                .cast::<crate::bindings::core_u_object::FTransform>()
                .swap(axis_conversion_inverse_transform);
        }
        unsafe { __buffer.add(96).cast::<bool>().read() }
    }
    pub fn get_custom_animated_time_start(&self, attribute_value: &mut f64) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<9>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_core::U_INTERCHANGE_SOURCE_NODE_GET_CUSTOM_ANIMATED_TIME_START,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                attribute_value,
                __buffer.add(0).cast::<f64>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_core::U_INTERCHANGE_SOURCE_NODE_GET_CUSTOM_ANIMATED_TIME_START,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<f64>().swap(attribute_value);
        }
        unsafe { __buffer.add(8).cast::<bool>().read() }
    }
    pub fn get_custom_animated_time_end(&self, attribute_value: &mut f64) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<9>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_core::U_INTERCHANGE_SOURCE_NODE_GET_CUSTOM_ANIMATED_TIME_END,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                attribute_value,
                __buffer.add(0).cast::<f64>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_core::U_INTERCHANGE_SOURCE_NODE_GET_CUSTOM_ANIMATED_TIME_END,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<f64>().swap(attribute_value);
        }
        unsafe { __buffer.add(8).cast::<bool>().read() }
    }
    pub fn get_custom_allow_scene_root_as_joint(
        &self,
        b_allow_scene_root_as_joint: &mut bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<2>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_core::U_INTERCHANGE_SOURCE_NODE_GET_CUSTOM_ALLOW_SCENE_ROOT_AS_JOINT,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                b_allow_scene_root_as_joint,
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
                crate::bindings::interchange_core::U_INTERCHANGE_SOURCE_NODE_GET_CUSTOM_ALLOW_SCENE_ROOT_AS_JOINT,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<bool>().swap(b_allow_scene_root_as_joint);
        }
        unsafe { __buffer.add(1).cast::<bool>().read() }
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
    pub fn remove_user_defined_attribute(
        interchange_node: UPtr<UInterchangeBaseNode>,
        user_defined_attribute_name: FString,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<25>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_core::U_INTERCHANGE_USER_DEFINED_ATTRIBUTES_API_REMOVE_USER_DEFINED_ATTRIBUTE,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &interchange_node,
                __buffer.add(0).cast::<UPtr<UInterchangeBaseNode>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &user_defined_attribute_name,
                __buffer.add(8).cast::<FString>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::interchange_core::UInterchangeUserDefinedAttributesAPI::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_core::U_INTERCHANGE_USER_DEFINED_ATTRIBUTES_API_REMOVE_USER_DEFINED_ATTRIBUTE,
                __buffer,
            )
        };
        unsafe { __buffer.add(24).cast::<bool>().read() }
    }
    pub fn get_user_defined_attribute_infos(
        interchange_node: UPtr<UInterchangeBaseNode>,
        user_defined_attribute_infos: &mut TArray<FInterchangeUserDefinedAttributeInfo>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_core::U_INTERCHANGE_USER_DEFINED_ATTRIBUTES_API_GET_USER_DEFINED_ATTRIBUTE_INFOS,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &interchange_node,
                __buffer.add(0).cast::<UPtr<UInterchangeBaseNode>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                user_defined_attribute_infos,
                __buffer.add(8).cast::<TArray<FInterchangeUserDefinedAttributeInfo>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::interchange_core::UInterchangeUserDefinedAttributesAPI::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_core::U_INTERCHANGE_USER_DEFINED_ATTRIBUTES_API_GET_USER_DEFINED_ATTRIBUTE_INFOS,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(8)
                .cast::<TArray<FInterchangeUserDefinedAttributeInfo>>()
                .swap(user_defined_attribute_infos);
        }
    }
    pub fn get_user_defined_attribute_int32(
        interchange_node: UPtr<UInterchangeBaseNode>,
        user_defined_attribute_name: FString,
        out_value: &mut i32,
        out_payload_key: &mut FString,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<49>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_core::U_INTERCHANGE_USER_DEFINED_ATTRIBUTES_API_GET_USER_DEFINED_ATTRIBUTE_INT32,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &interchange_node,
                __buffer.add(0).cast::<UPtr<UInterchangeBaseNode>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &user_defined_attribute_name,
                __buffer.add(8).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(out_value, __buffer.add(24).cast::<i32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_payload_key,
                __buffer.add(32).cast::<FString>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::interchange_core::UInterchangeUserDefinedAttributesAPI::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_core::U_INTERCHANGE_USER_DEFINED_ATTRIBUTES_API_GET_USER_DEFINED_ATTRIBUTE_INT32,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(24).cast::<i32>().swap(out_value);
        }
        unsafe {
            __buffer.add(32).cast::<FString>().swap(out_payload_key);
        }
        unsafe { __buffer.add(48).cast::<bool>().read() }
    }
    pub fn get_user_defined_attribute_f_string(
        interchange_node: UPtr<UInterchangeBaseNode>,
        user_defined_attribute_name: FString,
        out_value: &mut FString,
        out_payload_key: &mut FString,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<57>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_core::U_INTERCHANGE_USER_DEFINED_ATTRIBUTES_API_GET_USER_DEFINED_ATTRIBUTE_F_STRING,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &interchange_node,
                __buffer.add(0).cast::<UPtr<UInterchangeBaseNode>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &user_defined_attribute_name,
                __buffer.add(8).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_value,
                __buffer.add(24).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_payload_key,
                __buffer.add(40).cast::<FString>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::interchange_core::UInterchangeUserDefinedAttributesAPI::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_core::U_INTERCHANGE_USER_DEFINED_ATTRIBUTES_API_GET_USER_DEFINED_ATTRIBUTE_F_STRING,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(24).cast::<FString>().swap(out_value);
        }
        unsafe {
            __buffer.add(40).cast::<FString>().swap(out_payload_key);
        }
        unsafe { __buffer.add(56).cast::<bool>().read() }
    }
    pub fn get_user_defined_attribute_float(
        interchange_node: UPtr<UInterchangeBaseNode>,
        user_defined_attribute_name: FString,
        out_value: &mut f32,
        out_payload_key: &mut FString,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<49>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_core::U_INTERCHANGE_USER_DEFINED_ATTRIBUTES_API_GET_USER_DEFINED_ATTRIBUTE_FLOAT,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &interchange_node,
                __buffer.add(0).cast::<UPtr<UInterchangeBaseNode>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &user_defined_attribute_name,
                __buffer.add(8).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(out_value, __buffer.add(24).cast::<f32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_payload_key,
                __buffer.add(32).cast::<FString>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::interchange_core::UInterchangeUserDefinedAttributesAPI::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_core::U_INTERCHANGE_USER_DEFINED_ATTRIBUTES_API_GET_USER_DEFINED_ATTRIBUTE_FLOAT,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(24).cast::<f32>().swap(out_value);
        }
        unsafe {
            __buffer.add(32).cast::<FString>().swap(out_payload_key);
        }
        unsafe { __buffer.add(48).cast::<bool>().read() }
    }
    pub fn get_user_defined_attribute_double(
        interchange_node: UPtr<UInterchangeBaseNode>,
        user_defined_attribute_name: FString,
        out_value: &mut f64,
        out_payload_key: &mut FString,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<49>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_core::U_INTERCHANGE_USER_DEFINED_ATTRIBUTES_API_GET_USER_DEFINED_ATTRIBUTE_DOUBLE,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &interchange_node,
                __buffer.add(0).cast::<UPtr<UInterchangeBaseNode>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &user_defined_attribute_name,
                __buffer.add(8).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(out_value, __buffer.add(24).cast::<f64>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_payload_key,
                __buffer.add(32).cast::<FString>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::interchange_core::UInterchangeUserDefinedAttributesAPI::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_core::U_INTERCHANGE_USER_DEFINED_ATTRIBUTES_API_GET_USER_DEFINED_ATTRIBUTE_DOUBLE,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(24).cast::<f64>().swap(out_value);
        }
        unsafe {
            __buffer.add(32).cast::<FString>().swap(out_payload_key);
        }
        unsafe { __buffer.add(48).cast::<bool>().read() }
    }
    pub fn get_user_defined_attribute_boolean(
        interchange_node: UPtr<UInterchangeBaseNode>,
        user_defined_attribute_name: FString,
        out_value: &mut bool,
        out_payload_key: &mut FString,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<49>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_core::U_INTERCHANGE_USER_DEFINED_ATTRIBUTES_API_GET_USER_DEFINED_ATTRIBUTE_BOOLEAN,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &interchange_node,
                __buffer.add(0).cast::<UPtr<UInterchangeBaseNode>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &user_defined_attribute_name,
                __buffer.add(8).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(out_value, __buffer.add(24).cast::<bool>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_payload_key,
                __buffer.add(32).cast::<FString>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::interchange_core::UInterchangeUserDefinedAttributesAPI::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_core::U_INTERCHANGE_USER_DEFINED_ATTRIBUTES_API_GET_USER_DEFINED_ATTRIBUTE_BOOLEAN,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(24).cast::<bool>().swap(out_value);
        }
        unsafe {
            __buffer.add(32).cast::<FString>().swap(out_payload_key);
        }
        unsafe { __buffer.add(48).cast::<bool>().read() }
    }
    pub fn duplicate_all_user_defined_attribute(
        interchange_source_node: UPtr<UInterchangeBaseNode>,
        interchange_destination_node: UPtr<UInterchangeBaseNode>,
        b_add_source_node_name: bool,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<17>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_core::U_INTERCHANGE_USER_DEFINED_ATTRIBUTES_API_DUPLICATE_ALL_USER_DEFINED_ATTRIBUTE,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &interchange_source_node,
                __buffer.add(0).cast::<UPtr<UInterchangeBaseNode>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &interchange_destination_node,
                __buffer.add(8).cast::<UPtr<UInterchangeBaseNode>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_add_source_node_name,
                __buffer.add(16).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::interchange_core::UInterchangeUserDefinedAttributesAPI::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_core::U_INTERCHANGE_USER_DEFINED_ATTRIBUTES_API_DUPLICATE_ALL_USER_DEFINED_ATTRIBUTE,
                __buffer,
            )
        };
    }
    pub fn create_user_defined_attribute_int32(
        interchange_node: UPtr<UInterchangeBaseNode>,
        user_defined_attribute_name: FString,
        value: &i32,
        payload_key: FString,
        requires_delegate: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<50>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_core::U_INTERCHANGE_USER_DEFINED_ATTRIBUTES_API_CREATE_USER_DEFINED_ATTRIBUTE_INT32,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &interchange_node,
                __buffer.add(0).cast::<UPtr<UInterchangeBaseNode>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &user_defined_attribute_name,
                __buffer.add(8).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(value, __buffer.add(24).cast::<i32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &payload_key,
                __buffer.add(32).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &requires_delegate,
                __buffer.add(48).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::interchange_core::UInterchangeUserDefinedAttributesAPI::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_core::U_INTERCHANGE_USER_DEFINED_ATTRIBUTES_API_CREATE_USER_DEFINED_ATTRIBUTE_INT32,
                __buffer,
            )
        };
        unsafe { __buffer.add(49).cast::<bool>().read() }
    }
    pub fn create_user_defined_attribute_f_string(
        interchange_node: UPtr<UInterchangeBaseNode>,
        user_defined_attribute_name: FString,
        value: FString,
        payload_key: FString,
        requires_delegate: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<58>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_core::U_INTERCHANGE_USER_DEFINED_ATTRIBUTES_API_CREATE_USER_DEFINED_ATTRIBUTE_F_STRING,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &interchange_node,
                __buffer.add(0).cast::<UPtr<UInterchangeBaseNode>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &user_defined_attribute_name,
                __buffer.add(8).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&value, __buffer.add(24).cast::<FString>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &payload_key,
                __buffer.add(40).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &requires_delegate,
                __buffer.add(56).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::interchange_core::UInterchangeUserDefinedAttributesAPI::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_core::U_INTERCHANGE_USER_DEFINED_ATTRIBUTES_API_CREATE_USER_DEFINED_ATTRIBUTE_F_STRING,
                __buffer,
            )
        };
        unsafe { __buffer.add(57).cast::<bool>().read() }
    }
    pub fn create_user_defined_attribute_float(
        interchange_node: UPtr<UInterchangeBaseNode>,
        user_defined_attribute_name: FString,
        value: &f32,
        payload_key: FString,
        requires_delegate: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<50>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_core::U_INTERCHANGE_USER_DEFINED_ATTRIBUTES_API_CREATE_USER_DEFINED_ATTRIBUTE_FLOAT,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &interchange_node,
                __buffer.add(0).cast::<UPtr<UInterchangeBaseNode>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &user_defined_attribute_name,
                __buffer.add(8).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(value, __buffer.add(24).cast::<f32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &payload_key,
                __buffer.add(32).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &requires_delegate,
                __buffer.add(48).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::interchange_core::UInterchangeUserDefinedAttributesAPI::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_core::U_INTERCHANGE_USER_DEFINED_ATTRIBUTES_API_CREATE_USER_DEFINED_ATTRIBUTE_FLOAT,
                __buffer,
            )
        };
        unsafe { __buffer.add(49).cast::<bool>().read() }
    }
    pub fn create_user_defined_attribute_double(
        interchange_node: UPtr<UInterchangeBaseNode>,
        user_defined_attribute_name: FString,
        value: &f64,
        payload_key: FString,
        requires_delegate: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<50>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_core::U_INTERCHANGE_USER_DEFINED_ATTRIBUTES_API_CREATE_USER_DEFINED_ATTRIBUTE_DOUBLE,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &interchange_node,
                __buffer.add(0).cast::<UPtr<UInterchangeBaseNode>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &user_defined_attribute_name,
                __buffer.add(8).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(value, __buffer.add(24).cast::<f64>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &payload_key,
                __buffer.add(32).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &requires_delegate,
                __buffer.add(48).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::interchange_core::UInterchangeUserDefinedAttributesAPI::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_core::U_INTERCHANGE_USER_DEFINED_ATTRIBUTES_API_CREATE_USER_DEFINED_ATTRIBUTE_DOUBLE,
                __buffer,
            )
        };
        unsafe { __buffer.add(49).cast::<bool>().read() }
    }
    pub fn create_user_defined_attribute_boolean(
        interchange_node: UPtr<UInterchangeBaseNode>,
        user_defined_attribute_name: FString,
        value: &bool,
        payload_key: FString,
        requires_delegate: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<50>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_core::U_INTERCHANGE_USER_DEFINED_ATTRIBUTES_API_CREATE_USER_DEFINED_ATTRIBUTE_BOOLEAN,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &interchange_node,
                __buffer.add(0).cast::<UPtr<UInterchangeBaseNode>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &user_defined_attribute_name,
                __buffer.add(8).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(value, __buffer.add(24).cast::<bool>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &payload_key,
                __buffer.add(32).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &requires_delegate,
                __buffer.add(48).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::interchange_core::UInterchangeUserDefinedAttributesAPI::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_core::U_INTERCHANGE_USER_DEFINED_ATTRIBUTES_API_CREATE_USER_DEFINED_ATTRIBUTE_BOOLEAN,
                __buffer,
            )
        };
        unsafe { __buffer.add(49).cast::<bool>().read() }
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
