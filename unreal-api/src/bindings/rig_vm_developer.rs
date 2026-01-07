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
pub static mut U_RIG_VM_BLUEPRINT_SUSPEND_NOTIFICATIONS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_BLUEPRINT_SPLIT_ASSET_VARIANT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_BLUEPRINT_SET_AUTO_VM_RECOMPILE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_BLUEPRINT_REQUEST_RIG_VM_INIT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_BLUEPRINT_REQUEST_AUTO_VM_RECOMPILATION: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_BLUEPRINT_RENAME_MEMBER_VARIABLE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_BLUEPRINT_REMOVE_MODEL: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_BLUEPRINT_REMOVE_MEMBER_VARIABLE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_BLUEPRINT_RECOMPILE_VM_IF_REQUIRED: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_BLUEPRINT_RECOMPILE_VM: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_BLUEPRINT_JOIN_ASSET_VARIANT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_BLUEPRINT_GET_RIG_VM_HOST_CLASS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_BLUEPRINT_GET_OR_CREATE_LOCAL_FUNCTION_LIBRARY: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_BLUEPRINT_GET_OR_CREATE_CONTROLLER: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_BLUEPRINT_GET_MODEL: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_BLUEPRINT_GET_MEMBER_VARIABLES: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_BLUEPRINT_GET_MATCHING_VARIANTS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_BLUEPRINT_GET_LOCAL_FUNCTION_LIBRARY: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_BLUEPRINT_GET_FOCUSED_MODEL: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_BLUEPRINT_GET_DEFAULT_MODEL: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_BLUEPRINT_GET_DEBUGGED_RIG_VM_HOST: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_BLUEPRINT_GET_CONTROLLER_BY_NAME: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_BLUEPRINT_GET_CONTROLLER: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_BLUEPRINT_GET_AVAILABLE_RIG_VM_STRUCTS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_BLUEPRINT_GET_AUTO_VM_RECOMPILE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_BLUEPRINT_GET_ASSET_VARIANT_REF: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_BLUEPRINT_GET_ASSET_VARIANT_BP: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_BLUEPRINT_GET_ALL_MODELS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_BLUEPRINT_GENERATE_PYTHON_COMMANDS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_BLUEPRINT_CREATE_RIG_VM_HOST: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_BLUEPRINT_CHANGE_MEMBER_VARIABLE_TYPE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_BLUEPRINT_BULK_REMOVE_MEMBER_VARIABLES: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_BLUEPRINT_ADD_MODEL: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_BLUEPRINT_ADD_MEMBER_VARIABLE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_COMPILER_COMPILE_VM: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_COMPILER_COMPILE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_NODE_SET_IS_EXCLUDED_BY_EARLY_EXIT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_NODE_SET_HAS_EARLY_EXIT_MARKER: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_NODE_SET_HAS_BREAKPOINT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_NODE_SET_EXECUTION_IS_HALTED_AT_THIS_NODE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_NODE_IS_VISIBLE_IN_UI: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_NODE_IS_TRAIT_PIN: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_NODE_IS_SELECTED: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_NODE_IS_PURE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_NODE_IS_PIN_CATEGORY_EXPANDED: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_NODE_IS_MUTABLE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_NODE_IS_LOOP_NODE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_NODE_IS_LINKED_TO: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_NODE_IS_INPUT_AGGREGATE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_NODE_IS_INJECTED: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_NODE_IS_HIGHLIGHTED: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_NODE_IS_EXCLUDED_BY_EARLY_EXIT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_NODE_IS_EVENT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_NODE_IS_DEFINED_AS_VARYING: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_NODE_IS_DEFINED_AS_CONSTANT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_NODE_IS_CONTROL_FLOW_NODE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_NODE_IS_AGGREGATE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_NODE_HAS_PIN_OF_DIRECTION: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_NODE_HAS_OUTPUT_PIN: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_NODE_HAS_ORPHANED_PINS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_NODE_HAS_LAZY_PIN: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_NODE_HAS_IO_PIN: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_NODE_HAS_INPUT_PIN: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_NODE_HAS_EARLY_EXIT_MARKER: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_NODE_HAS_BREAKPOINT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_NODE_GET_TRAIT_PINS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_NODE_GET_TOOL_TIP_TEXT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_NODE_GET_SUPPORTED_WORKFLOWS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_NODE_GET_SUB_PIN_CATEGORIES: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_NODE_GET_SIZE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_NODE_GET_SECOND_AGGREGATE_PIN: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_NODE_GET_ROOT_GRAPH: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_NODE_GET_PREVIOUS_F_NAME: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_NODE_GET_POSITION: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_NODE_GET_PINS_FOR_CATEGORY: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_NODE_GET_PINS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_NODE_GET_PIN_CATEGORY_NAME: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_NODE_GET_PIN_CATEGORIES: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_NODE_GET_PARENT_PIN_CATEGORY: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_NODE_GET_PARENT_PIN_CATEGORIES: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_NODE_GET_ORPHANED_PINS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_NODE_GET_OPPOSITE_AGGREGATE_PIN: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_NODE_GET_NODE_TITLE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_NODE_GET_NODE_SUB_TITLE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_NODE_GET_NODE_PATH: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_NODE_GET_NODE_LAYOUT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_NODE_GET_NODE_INDEX: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_NODE_GET_NODE_COLOR: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_NODE_GET_NEXT_AGGREGATE_NAME: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_NODE_GET_LINKS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_NODE_GET_LINKED_TARGET_NODES: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_NODE_GET_LINKED_SOURCE_NODES: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_NODE_GET_INJECTION_INFO: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_NODE_GET_GRAPH_DEPTH: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_NODE_GET_GRAPH: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_NODE_GET_FIRST_AGGREGATE_PIN: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_NODE_GET_EVENT_NAME: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_NODE_GET_ALL_PINS_RECURSIVELY: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_NODE_GET_AGGREGATE_OUTPUTS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_NODE_GET_AGGREGATE_INPUTS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_NODE_FIND_ROOT_PIN_BY_NAME: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_NODE_FIND_PIN: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_NODE_FIND_FUNCTION_FOR_NODE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_NODE_FIND_EXECUTE_PIN: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_NODE_EXECUTION_IS_HALTED_AT_THIS_NODE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_NODE_CAN_ONLY_EXIST_ONCE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_NODE_CAN_BE_UPGRADED: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_TEMPLATE_NODE_IS_SINGLETON: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_TEMPLATE_NODE_IS_RESOLVED: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_TEMPLATE_NODE_IS_FULLY_UNRESOLVED: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_TEMPLATE_NODE_GET_SCRIPT_STRUCT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_TEMPLATE_NODE_GET_NOTATION: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_LIBRARY_NODE_GET_MATCHING_VARIANTS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_LIBRARY_NODE_GET_LIBRARY: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_LIBRARY_NODE_GET_FUNCTION_VARIANT_REF: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_LIBRARY_NODE_GET_FUNCTION_VARIANT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_LIBRARY_NODE_GET_CONTAINED_GRAPH: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut UDEPRECATED_RIG_VM_ARRAY_NODE_GET_OP_CODE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut UDEPRECATED_RIG_VM_ARRAY_NODE_GET_CPP_TYPE_OBJECT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut UDEPRECATED_RIG_VM_ARRAY_NODE_GET_CPP_TYPE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_COMMENT_NODE_GET_COMMENT_TEXT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_COMMENT_NODE_GET_COMMENT_FONT_SIZE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_COMMENT_NODE_GET_COMMENT_COLOR_BUBBLE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_COMMENT_NODE_GET_COMMENT_BUBBLE_VISIBLE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_ENUM_NODE_GET_ENUM: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_ENUM_NODE_GET_CPP_TYPE_OBJECT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_ENUM_NODE_GET_CPP_TYPE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_FUNCTION_REFERENCE_NODE_GET_REFERENCED_FUNCTION_HEADER_FOR_BLUEPRINT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_INVOKE_ENTRY_NODE_GET_ENTRY_NAME: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_PARAMETER_NODE_IS_INPUT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_PARAMETER_NODE_GET_PARAMETER_NAME: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_PARAMETER_NODE_GET_PARAMETER_DESCRIPTION: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_PARAMETER_NODE_GET_DEFAULT_VALUE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_PARAMETER_NODE_GET_CPP_TYPE_OBJECT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_PARAMETER_NODE_GET_CPP_TYPE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_UNIT_NODE_GET_STRUCT_DEFAULT_VALUE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_UNIT_NODE_GET_METHOD_NAME: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_VARIABLE_NODE_IS_LOCAL_VARIABLE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_VARIABLE_NODE_IS_INPUT_ARGUMENT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_VARIABLE_NODE_IS_GETTER: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_VARIABLE_NODE_IS_EXTERNAL_VARIABLE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_VARIABLE_NODE_GET_VARIABLE_NAME: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_VARIABLE_NODE_GET_VARIABLE_DESCRIPTION: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_VARIABLE_NODE_GET_DEFAULT_VALUE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_VARIABLE_NODE_GET_CPP_TYPE_OBJECT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_VARIABLE_NODE_GET_CPP_TYPE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_BUILD_DATA_SPLIT_VARIANT_FROM_SET: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_BUILD_DATA_JOIN_VARIANT_SET: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_BUILD_DATA_GET_VARIANT_REF_FOR_ASSET: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_BUILD_DATA_GET_USED_FUNCTION_IDENTIFIERS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_BUILD_DATA_GET_FUNCTION_IDENTIFIER_FOR_VARIANT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_BUILD_DATA_GET_ASSET_DATA_FOR_VARIANT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_BUILD_DATA_GET_ASSET_DATA_FOR_PATH: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_BUILD_DATA_GET_ALL_FUNCTION_IDENTIFIERS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_BUILD_DATA_GET: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_BUILD_DATA_GATHER_FUNCTION_VARIANT_REFS_FOR_ASSET: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_BUILD_DATA_GATHER_ALL_FUNCTION_VARIANT_REFS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_BUILD_DATA_GATHER_ALL_ASSET_VARIANT_REFS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_BUILD_DATA_FIND_FUNCTION_VARIANT_REFS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_BUILD_DATA_FIND_ASSET_VARIANT_REFS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_BUILD_DATA_CREATE_FUNCTION_VARIANT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_BUILD_DATA_CREATE_ASSET_VARIANT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_ACTION_STACK_UNDO: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_ACTION_STACK_REDO: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_ACTION_STACK_OPEN_UNDO_BRACKET: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_ACTION_STACK_CLOSE_UNDO_BRACKET: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_ACTION_STACK_CANCEL_UNDO_BRACKET: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_GRAPH_SET_SCHEMA_CLASS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_GRAPH_SET_DEFAULT_FUNCTION_LIBRARY: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_GRAPH_IS_TOP_LEVEL_GRAPH: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_GRAPH_IS_ROOT_GRAPH: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_GRAPH_IS_NODE_SELECTED: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_GRAPH_IS_NODE_HIGHLIGHTED: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_GRAPH_GET_VARIABLE_DESCRIPTIONS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_GRAPH_GET_SELECT_NODES: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_GRAPH_GET_SCHEMA_CLASS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_GRAPH_GET_SCHEMA: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_GRAPH_GET_ROOT_GRAPH: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_GRAPH_GET_RETURN_NODE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_GRAPH_GET_PARENT_GRAPH: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_GRAPH_GET_OUTPUT_ARGUMENTS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_GRAPH_GET_NODES: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_GRAPH_GET_NODE_PATH: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_GRAPH_GET_LOCAL_VARIABLES: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_GRAPH_GET_LINKS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_GRAPH_GET_INPUT_ARGUMENTS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_GRAPH_GET_GRAPH_NAME: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_GRAPH_GET_GRAPH_DEPTH: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_GRAPH_GET_EVENT_NAMES: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_GRAPH_GET_ENTRY_NODE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_GRAPH_GET_DEFAULT_FUNCTION_LIBRARY: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_GRAPH_GET_CONTAINED_GRAPHS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_GRAPH_FIND_PIN: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_GRAPH_FIND_NODE_BY_NAME: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_GRAPH_FIND_NODE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_GRAPH_FIND_LINK: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_GRAPH_CONTAINS_LINK: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_FUNCTION_LIBRARY_GET_REFERENCES_FOR_FUNCTION: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_FUNCTION_LIBRARY_GET_REFERENCE_PATHS_FOR_FUNCTION: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_FUNCTION_LIBRARY_GET_FUNCTIONS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_FUNCTION_LIBRARY_FIND_FUNCTION_FOR_NODE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_FUNCTION_LIBRARY_FIND_FUNCTION: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_LINK_GET_TARGET_PIN: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_LINK_GET_TARGET_NODE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_LINK_GET_SOURCE_PIN: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_LINK_GET_SOURCE_NODE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_LINK_GET_PIN_PATH_REPRESENTATION: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_LINK_GET_OPPOSITE_PIN: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_LINK_GET_LINK_INDEX: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_LINK_GET_GRAPH_DEPTH: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_LINK_GET_GRAPH: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_INJECTION_INFO_GET_PIN: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_INJECTION_INFO_GET_GRAPH: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_PIN_SHOULD_ONLY_SHOW_SUB_PINS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_PIN_SHOULD_HIDE_SUB_PINS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_PIN_REQUIRES_WATCH: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_PIN_IS_WILD_CARD: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_PIN_IS_VALID_DEFAULT_VALUE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_PIN_IS_U_OBJECT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_PIN_IS_TRAIT_PIN: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_PIN_IS_STRUCT_MEMBER: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_PIN_IS_STRUCT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_PIN_IS_STRING_TYPE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_PIN_IS_ROOT_PIN: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_PIN_IS_REFERENCE_COUNTED_CONTAINER: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_PIN_IS_LINKED_TO: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_PIN_IS_LAZY: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_PIN_IS_INTERFACE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_PIN_IS_FIXED_SIZE_ARRAY: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_PIN_IS_EXPANDED: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_PIN_IS_EXECUTE_CONTEXT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_PIN_IS_ENUM: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_PIN_IS_DYNAMIC_ARRAY: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_PIN_IS_DEFINED_AS_CONSTANT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_PIN_IS_ARRAY_ELEMENT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_PIN_IS_ARRAY: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_PIN_HAS_USER_PROVIDED_DEFAULT_VALUE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_PIN_HAS_ORIGINAL_DEFAULT_VALUE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_PIN_HAS_META_DATA: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_PIN_HAS_DEFAULT_VALUE_OVERRIDE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_PIN_GET_TOOL_TIP_TEXT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_PIN_GET_TARGET_LINKS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_PIN_GET_SUB_PINS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_PIN_GET_SUB_PIN_PATH: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_PIN_GET_SOURCE_LINKS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_PIN_GET_SEGMENT_PATH: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_PIN_GET_SCRIPT_STRUCT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_PIN_GET_ROOT_PIN: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_PIN_GET_PIN_PATH: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_PIN_GET_PIN_INDEX: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_PIN_GET_PIN_FOR_LINK: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_PIN_GET_PARENT_SCRIPT_STRUCT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_PIN_GET_PARENT_PIN: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_PIN_GET_ORIGINAL_PIN_FROM_INJECTED_NODE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_PIN_GET_ORIGINAL_DEFAULT_VALUE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_PIN_GET_NODE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_PIN_GET_META_DATA: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_PIN_GET_LINKS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_PIN_GET_LINKED_TARGET_PINS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_PIN_GET_LINKED_SOURCE_PINS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_PIN_GET_INDEX_IN_CATEGORY: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_PIN_GET_GRAPH: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_PIN_GET_ENUM: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_PIN_GET_DISPLAY_NAME: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_PIN_GET_DIRECTION: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_PIN_GET_DEFAULT_VALUE_TYPE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_PIN_GET_DEFAULT_VALUE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_PIN_GET_CUSTOM_WIDGET_NAME: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_PIN_GET_CPP_TYPE_OBJECT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_PIN_GET_CPP_TYPE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_PIN_GET_CATEGORY: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_PIN_GET_ARRAY_SIZE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_PIN_GET_ARRAY_ELEMENT_CPP_TYPE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_PIN_GET_ALL_SUB_PINS_RECURSIVELY: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_PIN_GET_ABSOLUTE_PIN_INDEX: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_PIN_FIND_SUB_PIN: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_PIN_FIND_LINK_FOR_PIN: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_PIN_CONTAINS_WILD_CARD_SUB_PIN: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_PIN_CAN_PROVIDE_DEFAULT_VALUE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_USER_WORKFLOW_REGISTRY_UNREGISTER_PROVIDER: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_USER_WORKFLOW_REGISTRY_REGISTER_PROVIDER: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_USER_WORKFLOW_REGISTRY_GET_WORKFLOWS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_USER_WORKFLOW_REGISTRY_GET: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_CONTROLLER_UPGRADE_NODES: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_CONTROLLER_UNRESOLVE_TEMPLATE_NODES: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_CONTROLLER_UNDO: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_CONTROLLER_UNBIND_PIN_FROM_VARIABLE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_CONTROLLER_SWAP_FUNCTION_REFERENCE_BY_NAME: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_CONTROLLER_SWAP_FUNCTION_REFERENCE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_CONTROLLER_SWAP_ALL_FUNCTION_REFERENCES: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_CONTROLLER_SPLIT_FUNCTION_VARIANT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_CONTROLLER_SET_UNIT_NODE_DEFAULTS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_CONTROLLER_SET_SCHEMA_CLASS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_CONTROLLER_SET_SCHEMA: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_CONTROLLER_SET_REMAPPED_VARIABLE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_CONTROLLER_SET_PIN_IS_WATCHED: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_CONTROLLER_SET_PIN_INDEX_IN_CATEGORY: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_CONTROLLER_SET_PIN_EXPANSION: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_CONTROLLER_SET_PIN_DISPLAY_NAME: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_CONTROLLER_SET_PIN_DEFAULT_VALUE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_CONTROLLER_SET_PIN_CATEGORY_INDEX: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_CONTROLLER_SET_PIN_CATEGORY_EXPANSION: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_CONTROLLER_SET_PIN_CATEGORY: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_CONTROLLER_SET_NODE_TITLE_BY_NAME: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_CONTROLLER_SET_NODE_TITLE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_CONTROLLER_SET_NODE_SIZE_BY_NAME: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_CONTROLLER_SET_NODE_SIZE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_CONTROLLER_SET_NODE_SELECTION: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_CONTROLLER_SET_NODE_POSITION_BY_NAME: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_CONTROLLER_SET_NODE_POSITION: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_CONTROLLER_SET_NODE_LAYOUT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_CONTROLLER_SET_NODE_KEYWORDS_BY_NAME: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_CONTROLLER_SET_NODE_KEYWORDS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_CONTROLLER_SET_NODE_DESCRIPTION_BY_NAME: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_CONTROLLER_SET_NODE_DESCRIPTION: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_CONTROLLER_SET_NODE_COLOR_BY_NAME: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_CONTROLLER_SET_NODE_COLOR: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_CONTROLLER_SET_NODE_CATEGORY_BY_NAME: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_CONTROLLER_SET_NODE_CATEGORY: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_CONTROLLER_SET_LOCAL_VARIABLE_TYPE_FROM_OBJECT_PATH: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_CONTROLLER_SET_LOCAL_VARIABLE_TYPE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_CONTROLLER_SET_LOCAL_VARIABLE_DEFAULT_VALUE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_CONTROLLER_SET_IS_RUNNING_UNIT_TEST: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_CONTROLLER_SET_GRAPH: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_CONTROLLER_SET_EXPOSED_PIN_INDEX: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_CONTROLLER_SET_COMMENT_TEXT_BY_NAME: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_CONTROLLER_SET_COMMENT_TEXT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_CONTROLLER_SET_ARRAY_PIN_SIZE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_CONTROLLER_SET_ACTION_STACK: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_CONTROLLER_SELECT_NODE_ISLANDS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_CONTROLLER_SELECT_NODE_BY_NAME: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_CONTROLLER_SELECT_NODE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_CONTROLLER_SELECT_LINKED_NODES: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_CONTROLLER_RESOLVE_WILD_CARD_PIN: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_CONTROLLER_RESET_PIN_DEFAULT_VALUE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_CONTROLLER_RESET_DEFAULT_VALUE_FOR_PINS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_CONTROLLER_RESET_DEFAULT_VALUE_FOR_ALL_PINS_ON_NODES: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_CONTROLLER_RESET_DEFAULT_VALUE_FOR_ALL_PINS_ON_NODE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_CONTROLLER_REPLACE_PARAMETER_NODE_WITH_VARIABLE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_CONTROLLER_RENAME_VARIABLE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_CONTROLLER_RENAME_PIN_CATEGORY: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_CONTROLLER_RENAME_PARAMETER: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_CONTROLLER_RENAME_NODE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_CONTROLLER_RENAME_LOCAL_VARIABLE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_CONTROLLER_RENAME_FUNCTION: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_CONTROLLER_RENAME_EXPOSED_PIN: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_CONTROLLER_REMOVE_TRAIT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_CONTROLLER_REMOVE_TAG_FROM_FUNCTION_VARIANT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_CONTROLLER_REMOVE_PIN_CATEGORY: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_CONTROLLER_REMOVE_NODES_BY_NAME: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_CONTROLLER_REMOVE_NODES: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_CONTROLLER_REMOVE_NODE_BY_NAME: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_CONTROLLER_REMOVE_NODE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_CONTROLLER_REMOVE_LOCAL_VARIABLE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_CONTROLLER_REMOVE_INJECTED_NODE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_CONTROLLER_REMOVE_FUNCTION_FROM_LIBRARY: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_CONTROLLER_REMOVE_EXPOSED_PIN: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_CONTROLLER_REMOVE_ARRAY_PIN: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_CONTROLLER_REMOVE_AGGREGATE_PIN: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_CONTROLLER_REFRESH_VARIABLE_NODE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_CONTROLLER_REDO: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_CONTROLLER_PUSH_GRAPH: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_CONTROLLER_PROMOTE_PIN_TO_VARIABLE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_CONTROLLER_PROMOTE_FUNCTION_REFERENCE_NODE_TO_COLLAPSE_NODE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_CONTROLLER_PROMOTE_COLLAPSE_NODE_TO_FUNCTION_REFERENCE_NODE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_CONTROLLER_POP_GRAPH: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_CONTROLLER_PERFORM_USER_WORKFLOW: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_CONTROLLER_OPEN_UNDO_BRACKET: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_CONTROLLER_MARK_FUNCTION_AS_PUBLIC: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_CONTROLLER_MAKE_VARIABLE_NODE_FROM_BINDING: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_CONTROLLER_MAKE_OPTIONS_FOR_WORKFLOW: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_CONTROLLER_MAKE_BINDINGS_FROM_VARIABLE_NODE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_CONTROLLER_LOCALIZE_FUNCTIONS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_CONTROLLER_LOCALIZE_FUNCTION_FROM_PATH: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_CONTROLLER_LOCALIZE_FUNCTION: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_CONTROLLER_JOIN_FUNCTION_VARIANT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_CONTROLLER_IS_TRANSACTING: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_CONTROLLER_IS_REPORTING_ENABLED: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_CONTROLLER_IS_FUNCTION_PUBLIC: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_CONTROLLER_INSERT_ARRAY_PIN: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_CONTROLLER_IMPORT_NODES_FROM_TEXT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_CONTROLLER_GET_UNIT_STRUCTS_FOR_TEMPLATE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_CONTROLLER_GET_TOP_LEVEL_GRAPH: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_CONTROLLER_GET_TEMPLATE_FOR_UNIT_STRUCT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_CONTROLLER_GET_SCHEMA: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_CONTROLLER_GET_REGISTERED_UNIT_STRUCTS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_CONTROLLER_GET_REGISTERED_TEMPLATES: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_CONTROLLER_GET_PIN_DEFAULT_VALUE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_CONTROLLER_GET_GRAPH: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_CONTROLLER_GET_CONTROLLER_FOR_GRAPH: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_CONTROLLER_GET_ACTION_STACK: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_CONTROLLER_GENERATE_PYTHON_COMMANDS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_CONTROLLER_FIND_VARIANTS_OF_FUNCTION: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_CONTROLLER_FIND_GRAPH_FUNCTION_IDENTIFIER: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_CONTROLLER_FIND_GRAPH_FUNCTION_HEADER_BY_NAME: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_CONTROLLER_FIND_GRAPH_FUNCTION_HEADER: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_CONTROLLER_EXPORT_SELECTED_NODES_TO_TEXT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_CONTROLLER_EXPORT_NODE_TO_TEXT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_CONTROLLER_EXPORT_NODES_TO_TEXT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_CONTROLLER_EXPAND_LIBRARY_NODE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_CONTROLLER_ENABLE_REPORTING: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_CONTROLLER_EJECT_NODE_FROM_PIN: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_CONTROLLER_DUPLICATE_ARRAY_PIN: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_CONTROLLER_CREATE_FUNCTION_VARIANT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_CONTROLLER_COLLAPSE_NODES: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_CONTROLLER_CLOSE_UNDO_BRACKET: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_CONTROLLER_CLEAR_PIN_CATEGORY: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_CONTROLLER_CLEAR_OVERRIDE_ON_PINS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_CONTROLLER_CLEAR_OVERRIDE_ON_PIN: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_CONTROLLER_CLEAR_OVERRIDE_ON_ALL_PINS_ON_NODES: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_CONTROLLER_CLEAR_OVERRIDE_ON_ALL_PINS_ON_NODE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_CONTROLLER_CLEAR_NODE_SELECTION: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_CONTROLLER_CLEAR_NODE_LAYOUT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_CONTROLLER_CLEAR_ARRAY_PIN: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_CONTROLLER_CHANGE_EXPOSED_PIN_TYPE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_CONTROLLER_CAN_IMPORT_NODES_FROM_TEXT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_CONTROLLER_CANCEL_UNDO_BRACKET: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_CONTROLLER_BREAK_LINK: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_CONTROLLER_BREAK_ALL_LINKS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_CONTROLLER_BIND_PIN_TO_VARIABLE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_CONTROLLER_ADD_VARIABLE_NODE_FROM_OBJECT_PATH: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_CONTROLLER_ADD_VARIABLE_NODE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_CONTROLLER_ADD_UNIT_NODE_WITH_DEFAULTS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_CONTROLLER_ADD_UNIT_NODE_FROM_STRUCT_PATH: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_CONTROLLER_ADD_UNIT_NODE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_CONTROLLER_ADD_TRAIT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_CONTROLLER_ADD_TEMPLATE_NODE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_CONTROLLER_ADD_TAG_TO_FUNCTION_VARIANT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_CONTROLLER_ADD_SELECT_NODE_FROM_STRUCT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_CONTROLLER_ADD_SELECT_NODE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_CONTROLLER_ADD_REROUTE_NODE_ON_PIN: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_CONTROLLER_ADD_REROUTE_NODE_ON_LINK_PATH: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_CONTROLLER_ADD_REROUTE_NODE_ON_LINK: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_CONTROLLER_ADD_PARAMETER_NODE_FROM_OBJECT_PATH: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_CONTROLLER_ADD_PARAMETER_NODE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_CONTROLLER_ADD_OVERRIDE_TO_PINS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_CONTROLLER_ADD_OVERRIDE_TO_PIN: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_CONTROLLER_ADD_OVERRIDE_TO_ALL_PINS_ON_NODES: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_CONTROLLER_ADD_OVERRIDE_TO_ALL_PINS_ON_NODE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_CONTROLLER_ADD_LOCAL_VARIABLE_FROM_OBJECT_PATH: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_CONTROLLER_ADD_LOCAL_VARIABLE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_CONTROLLER_ADD_LINK: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_CONTROLLER_ADD_INVOKE_ENTRY_NODE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_CONTROLLER_ADD_INJECTED_NODE_FROM_STRUCT_PATH: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_CONTROLLER_ADD_INJECTED_NODE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_CONTROLLER_ADD_IF_NODE_FROM_STRUCT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_CONTROLLER_ADD_IF_NODE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_CONTROLLER_ADD_FUNCTION_TO_LIBRARY: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_CONTROLLER_ADD_FUNCTION_REFERENCE_NODE_FROM_DESCRIPTION: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_CONTROLLER_ADD_FUNCTION_REFERENCE_NODE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_CONTROLLER_ADD_FREE_REROUTE_NODE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_CONTROLLER_ADD_EXTERNAL_FUNCTION_REFERENCE_NODE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_CONTROLLER_ADD_EXPOSED_PIN: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_CONTROLLER_ADD_ENUM_NODE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_CONTROLLER_ADD_EMPTY_PIN_CATEGORY: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_CONTROLLER_ADD_DEFAULT_TAG_TO_FUNCTION_VARIANT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_CONTROLLER_ADD_COMMENT_NODE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_CONTROLLER_ADD_BRANCH_NODE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_CONTROLLER_ADD_ARRAY_PIN: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_CONTROLLER_ADD_ARRAY_NODE_FROM_OBJECT_PATH: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_CONTROLLER_ADD_ARRAY_NODE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_CONTROLLER_ADD_AGGREGATE_PIN: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
pub fn initialize() {
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = URigVMBlueprint::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SuspendNotifications"),
            &raw mut U_RIG_VM_BLUEPRINT_SUSPEND_NOTIFICATIONS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SplitAssetVariant"),
            &raw mut U_RIG_VM_BLUEPRINT_SPLIT_ASSET_VARIANT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetAutoVMRecompile"),
            &raw mut U_RIG_VM_BLUEPRINT_SET_AUTO_VM_RECOMPILE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RequestRigVMInit"),
            &raw mut U_RIG_VM_BLUEPRINT_REQUEST_RIG_VM_INIT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RequestAutoVMRecompilation"),
            &raw mut U_RIG_VM_BLUEPRINT_REQUEST_AUTO_VM_RECOMPILATION,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RenameMemberVariable"),
            &raw mut U_RIG_VM_BLUEPRINT_RENAME_MEMBER_VARIABLE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RemoveModel"),
            &raw mut U_RIG_VM_BLUEPRINT_REMOVE_MODEL,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RemoveMemberVariable"),
            &raw mut U_RIG_VM_BLUEPRINT_REMOVE_MEMBER_VARIABLE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RecompileVMIfRequired"),
            &raw mut U_RIG_VM_BLUEPRINT_RECOMPILE_VM_IF_REQUIRED,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RecompileVM"),
            &raw mut U_RIG_VM_BLUEPRINT_RECOMPILE_VM,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("JoinAssetVariant"),
            &raw mut U_RIG_VM_BLUEPRINT_JOIN_ASSET_VARIANT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetRigVMHostClass"),
            &raw mut U_RIG_VM_BLUEPRINT_GET_RIG_VM_HOST_CLASS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetOrCreateLocalFunctionLibrary"),
            &raw mut U_RIG_VM_BLUEPRINT_GET_OR_CREATE_LOCAL_FUNCTION_LIBRARY,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetOrCreateController"),
            &raw mut U_RIG_VM_BLUEPRINT_GET_OR_CREATE_CONTROLLER,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetModel"),
            &raw mut U_RIG_VM_BLUEPRINT_GET_MODEL,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetMemberVariables"),
            &raw mut U_RIG_VM_BLUEPRINT_GET_MEMBER_VARIABLES,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetMatchingVariants"),
            &raw mut U_RIG_VM_BLUEPRINT_GET_MATCHING_VARIANTS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetLocalFunctionLibrary"),
            &raw mut U_RIG_VM_BLUEPRINT_GET_LOCAL_FUNCTION_LIBRARY,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetFocusedModel"),
            &raw mut U_RIG_VM_BLUEPRINT_GET_FOCUSED_MODEL,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetDefaultModel"),
            &raw mut U_RIG_VM_BLUEPRINT_GET_DEFAULT_MODEL,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetDebuggedRigVMHost"),
            &raw mut U_RIG_VM_BLUEPRINT_GET_DEBUGGED_RIG_VM_HOST,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetControllerByName"),
            &raw mut U_RIG_VM_BLUEPRINT_GET_CONTROLLER_BY_NAME,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetController"),
            &raw mut U_RIG_VM_BLUEPRINT_GET_CONTROLLER,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetAvailableRigVMStructs"),
            &raw mut U_RIG_VM_BLUEPRINT_GET_AVAILABLE_RIG_VM_STRUCTS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetAutoVMRecompile"),
            &raw mut U_RIG_VM_BLUEPRINT_GET_AUTO_VM_RECOMPILE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetAssetVariantRef"),
            &raw mut U_RIG_VM_BLUEPRINT_GET_ASSET_VARIANT_REF,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetAssetVariantBP"),
            &raw mut U_RIG_VM_BLUEPRINT_GET_ASSET_VARIANT_BP,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetAllModels"),
            &raw mut U_RIG_VM_BLUEPRINT_GET_ALL_MODELS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GeneratePythonCommands"),
            &raw mut U_RIG_VM_BLUEPRINT_GENERATE_PYTHON_COMMANDS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CreateRigVMHost"),
            &raw mut U_RIG_VM_BLUEPRINT_CREATE_RIG_VM_HOST,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ChangeMemberVariableType"),
            &raw mut U_RIG_VM_BLUEPRINT_CHANGE_MEMBER_VARIABLE_TYPE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("BulkRemoveMemberVariables"),
            &raw mut U_RIG_VM_BLUEPRINT_BULK_REMOVE_MEMBER_VARIABLES,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AddModel"),
            &raw mut U_RIG_VM_BLUEPRINT_ADD_MODEL,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AddMemberVariable"),
            &raw mut U_RIG_VM_BLUEPRINT_ADD_MEMBER_VARIABLE,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = URigVMCompiler::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CompileVM"),
            &raw mut U_RIG_VM_COMPILER_COMPILE_VM,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("Compile"),
            &raw mut U_RIG_VM_COMPILER_COMPILE,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = URigVMNode::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetIsExcludedByEarlyExit"),
            &raw mut U_RIG_VM_NODE_SET_IS_EXCLUDED_BY_EARLY_EXIT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetHasEarlyExitMarker"),
            &raw mut U_RIG_VM_NODE_SET_HAS_EARLY_EXIT_MARKER,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetHasBreakpoint"),
            &raw mut U_RIG_VM_NODE_SET_HAS_BREAKPOINT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetExecutionIsHaltedAtThisNode"),
            &raw mut U_RIG_VM_NODE_SET_EXECUTION_IS_HALTED_AT_THIS_NODE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("IsVisibleInUI"),
            &raw mut U_RIG_VM_NODE_IS_VISIBLE_IN_UI,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("IsTraitPin"),
            &raw mut U_RIG_VM_NODE_IS_TRAIT_PIN,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("IsSelected"),
            &raw mut U_RIG_VM_NODE_IS_SELECTED,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("IsPure"),
            &raw mut U_RIG_VM_NODE_IS_PURE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("IsPinCategoryExpanded"),
            &raw mut U_RIG_VM_NODE_IS_PIN_CATEGORY_EXPANDED,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("IsMutable"),
            &raw mut U_RIG_VM_NODE_IS_MUTABLE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("IsLoopNode"),
            &raw mut U_RIG_VM_NODE_IS_LOOP_NODE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("IsLinkedTo"),
            &raw mut U_RIG_VM_NODE_IS_LINKED_TO,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("IsInputAggregate"),
            &raw mut U_RIG_VM_NODE_IS_INPUT_AGGREGATE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("IsInjected"),
            &raw mut U_RIG_VM_NODE_IS_INJECTED,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("IsHighlighted"),
            &raw mut U_RIG_VM_NODE_IS_HIGHLIGHTED,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("IsExcludedByEarlyExit"),
            &raw mut U_RIG_VM_NODE_IS_EXCLUDED_BY_EARLY_EXIT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("IsEvent"),
            &raw mut U_RIG_VM_NODE_IS_EVENT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("IsDefinedAsVarying"),
            &raw mut U_RIG_VM_NODE_IS_DEFINED_AS_VARYING,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("IsDefinedAsConstant"),
            &raw mut U_RIG_VM_NODE_IS_DEFINED_AS_CONSTANT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("IsControlFlowNode"),
            &raw mut U_RIG_VM_NODE_IS_CONTROL_FLOW_NODE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("IsAggregate"),
            &raw mut U_RIG_VM_NODE_IS_AGGREGATE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("HasPinOfDirection"),
            &raw mut U_RIG_VM_NODE_HAS_PIN_OF_DIRECTION,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("HasOutputPin"),
            &raw mut U_RIG_VM_NODE_HAS_OUTPUT_PIN,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("HasOrphanedPins"),
            &raw mut U_RIG_VM_NODE_HAS_ORPHANED_PINS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("HasLazyPin"),
            &raw mut U_RIG_VM_NODE_HAS_LAZY_PIN,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("HasIOPin"),
            &raw mut U_RIG_VM_NODE_HAS_IO_PIN,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("HasInputPin"),
            &raw mut U_RIG_VM_NODE_HAS_INPUT_PIN,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("HasEarlyExitMarker"),
            &raw mut U_RIG_VM_NODE_HAS_EARLY_EXIT_MARKER,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("HasBreakpoint"),
            &raw mut U_RIG_VM_NODE_HAS_BREAKPOINT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetTraitPins"),
            &raw mut U_RIG_VM_NODE_GET_TRAIT_PINS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetToolTipText"),
            &raw mut U_RIG_VM_NODE_GET_TOOL_TIP_TEXT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetSupportedWorkflows"),
            &raw mut U_RIG_VM_NODE_GET_SUPPORTED_WORKFLOWS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetSubPinCategories"),
            &raw mut U_RIG_VM_NODE_GET_SUB_PIN_CATEGORIES,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetSize"),
            &raw mut U_RIG_VM_NODE_GET_SIZE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetSecondAggregatePin"),
            &raw mut U_RIG_VM_NODE_GET_SECOND_AGGREGATE_PIN,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetRootGraph"),
            &raw mut U_RIG_VM_NODE_GET_ROOT_GRAPH,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetPreviousFName"),
            &raw mut U_RIG_VM_NODE_GET_PREVIOUS_F_NAME,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetPosition"),
            &raw mut U_RIG_VM_NODE_GET_POSITION,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetPinsForCategory"),
            &raw mut U_RIG_VM_NODE_GET_PINS_FOR_CATEGORY,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetPins"),
            &raw mut U_RIG_VM_NODE_GET_PINS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetPinCategoryName"),
            &raw mut U_RIG_VM_NODE_GET_PIN_CATEGORY_NAME,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetPinCategories"),
            &raw mut U_RIG_VM_NODE_GET_PIN_CATEGORIES,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetParentPinCategory"),
            &raw mut U_RIG_VM_NODE_GET_PARENT_PIN_CATEGORY,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetParentPinCategories"),
            &raw mut U_RIG_VM_NODE_GET_PARENT_PIN_CATEGORIES,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetOrphanedPins"),
            &raw mut U_RIG_VM_NODE_GET_ORPHANED_PINS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetOppositeAggregatePin"),
            &raw mut U_RIG_VM_NODE_GET_OPPOSITE_AGGREGATE_PIN,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetNodeTitle"),
            &raw mut U_RIG_VM_NODE_GET_NODE_TITLE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetNodeSubTitle"),
            &raw mut U_RIG_VM_NODE_GET_NODE_SUB_TITLE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetNodePath"),
            &raw mut U_RIG_VM_NODE_GET_NODE_PATH,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetNodeLayout"),
            &raw mut U_RIG_VM_NODE_GET_NODE_LAYOUT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetNodeIndex"),
            &raw mut U_RIG_VM_NODE_GET_NODE_INDEX,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetNodeColor"),
            &raw mut U_RIG_VM_NODE_GET_NODE_COLOR,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetNextAggregateName"),
            &raw mut U_RIG_VM_NODE_GET_NEXT_AGGREGATE_NAME,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetLinks"),
            &raw mut U_RIG_VM_NODE_GET_LINKS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetLinkedTargetNodes"),
            &raw mut U_RIG_VM_NODE_GET_LINKED_TARGET_NODES,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetLinkedSourceNodes"),
            &raw mut U_RIG_VM_NODE_GET_LINKED_SOURCE_NODES,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetInjectionInfo"),
            &raw mut U_RIG_VM_NODE_GET_INJECTION_INFO,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetGraphDepth"),
            &raw mut U_RIG_VM_NODE_GET_GRAPH_DEPTH,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetGraph"),
            &raw mut U_RIG_VM_NODE_GET_GRAPH,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetFirstAggregatePin"),
            &raw mut U_RIG_VM_NODE_GET_FIRST_AGGREGATE_PIN,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetEventName"),
            &raw mut U_RIG_VM_NODE_GET_EVENT_NAME,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetAllPinsRecursively"),
            &raw mut U_RIG_VM_NODE_GET_ALL_PINS_RECURSIVELY,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetAggregateOutputs"),
            &raw mut U_RIG_VM_NODE_GET_AGGREGATE_OUTPUTS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetAggregateInputs"),
            &raw mut U_RIG_VM_NODE_GET_AGGREGATE_INPUTS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("FindRootPinByName"),
            &raw mut U_RIG_VM_NODE_FIND_ROOT_PIN_BY_NAME,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("FindPin"),
            &raw mut U_RIG_VM_NODE_FIND_PIN,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("FindFunctionForNode"),
            &raw mut U_RIG_VM_NODE_FIND_FUNCTION_FOR_NODE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("FindExecutePin"),
            &raw mut U_RIG_VM_NODE_FIND_EXECUTE_PIN,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ExecutionIsHaltedAtThisNode"),
            &raw mut U_RIG_VM_NODE_EXECUTION_IS_HALTED_AT_THIS_NODE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CanOnlyExistOnce"),
            &raw mut U_RIG_VM_NODE_CAN_ONLY_EXIST_ONCE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CanBeUpgraded"),
            &raw mut U_RIG_VM_NODE_CAN_BE_UPGRADED,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = URigVMTemplateNode::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("IsSingleton"),
            &raw mut U_RIG_VM_TEMPLATE_NODE_IS_SINGLETON,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("IsResolved"),
            &raw mut U_RIG_VM_TEMPLATE_NODE_IS_RESOLVED,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("IsFullyUnresolved"),
            &raw mut U_RIG_VM_TEMPLATE_NODE_IS_FULLY_UNRESOLVED,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetScriptStruct"),
            &raw mut U_RIG_VM_TEMPLATE_NODE_GET_SCRIPT_STRUCT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetNotation"),
            &raw mut U_RIG_VM_TEMPLATE_NODE_GET_NOTATION,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = URigVMLibraryNode::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetMatchingVariants"),
            &raw mut U_RIG_VM_LIBRARY_NODE_GET_MATCHING_VARIANTS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetLibrary"),
            &raw mut U_RIG_VM_LIBRARY_NODE_GET_LIBRARY,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetFunctionVariantRef"),
            &raw mut U_RIG_VM_LIBRARY_NODE_GET_FUNCTION_VARIANT_REF,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetFunctionVariant"),
            &raw mut U_RIG_VM_LIBRARY_NODE_GET_FUNCTION_VARIANT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetContainedGraph"),
            &raw mut U_RIG_VM_LIBRARY_NODE_GET_CONTAINED_GRAPH,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UDEPRECATED_RigVMArrayNode::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetOpCode"),
            &raw mut UDEPRECATED_RIG_VM_ARRAY_NODE_GET_OP_CODE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetCPPTypeObject"),
            &raw mut UDEPRECATED_RIG_VM_ARRAY_NODE_GET_CPP_TYPE_OBJECT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetCPPType"),
            &raw mut UDEPRECATED_RIG_VM_ARRAY_NODE_GET_CPP_TYPE,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = URigVMCommentNode::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetCommentText"),
            &raw mut U_RIG_VM_COMMENT_NODE_GET_COMMENT_TEXT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetCommentFontSize"),
            &raw mut U_RIG_VM_COMMENT_NODE_GET_COMMENT_FONT_SIZE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetCommentColorBubble"),
            &raw mut U_RIG_VM_COMMENT_NODE_GET_COMMENT_COLOR_BUBBLE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetCommentBubbleVisible"),
            &raw mut U_RIG_VM_COMMENT_NODE_GET_COMMENT_BUBBLE_VISIBLE,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = URigVMEnumNode::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetEnum"),
            &raw mut U_RIG_VM_ENUM_NODE_GET_ENUM,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetCPPTypeObject"),
            &raw mut U_RIG_VM_ENUM_NODE_GET_CPP_TYPE_OBJECT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetCPPType"),
            &raw mut U_RIG_VM_ENUM_NODE_GET_CPP_TYPE,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = URigVMFunctionReferenceNode::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetReferencedFunctionHeader_ForBlueprint"),
            &raw mut U_RIG_VM_FUNCTION_REFERENCE_NODE_GET_REFERENCED_FUNCTION_HEADER_FOR_BLUEPRINT,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = URigVMInvokeEntryNode::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetEntryName"),
            &raw mut U_RIG_VM_INVOKE_ENTRY_NODE_GET_ENTRY_NAME,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = URigVMParameterNode::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("IsInput"),
            &raw mut U_RIG_VM_PARAMETER_NODE_IS_INPUT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetParameterName"),
            &raw mut U_RIG_VM_PARAMETER_NODE_GET_PARAMETER_NAME,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetParameterDescription"),
            &raw mut U_RIG_VM_PARAMETER_NODE_GET_PARAMETER_DESCRIPTION,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetDefaultValue"),
            &raw mut U_RIG_VM_PARAMETER_NODE_GET_DEFAULT_VALUE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetCPPTypeObject"),
            &raw mut U_RIG_VM_PARAMETER_NODE_GET_CPP_TYPE_OBJECT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetCPPType"),
            &raw mut U_RIG_VM_PARAMETER_NODE_GET_CPP_TYPE,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = URigVMUnitNode::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetStructDefaultValue"),
            &raw mut U_RIG_VM_UNIT_NODE_GET_STRUCT_DEFAULT_VALUE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetMethodName"),
            &raw mut U_RIG_VM_UNIT_NODE_GET_METHOD_NAME,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = URigVMVariableNode::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("IsLocalVariable"),
            &raw mut U_RIG_VM_VARIABLE_NODE_IS_LOCAL_VARIABLE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("IsInputArgument"),
            &raw mut U_RIG_VM_VARIABLE_NODE_IS_INPUT_ARGUMENT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("IsGetter"),
            &raw mut U_RIG_VM_VARIABLE_NODE_IS_GETTER,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("IsExternalVariable"),
            &raw mut U_RIG_VM_VARIABLE_NODE_IS_EXTERNAL_VARIABLE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetVariableName"),
            &raw mut U_RIG_VM_VARIABLE_NODE_GET_VARIABLE_NAME,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetVariableDescription"),
            &raw mut U_RIG_VM_VARIABLE_NODE_GET_VARIABLE_DESCRIPTION,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetDefaultValue"),
            &raw mut U_RIG_VM_VARIABLE_NODE_GET_DEFAULT_VALUE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetCPPTypeObject"),
            &raw mut U_RIG_VM_VARIABLE_NODE_GET_CPP_TYPE_OBJECT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetCPPType"),
            &raw mut U_RIG_VM_VARIABLE_NODE_GET_CPP_TYPE,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = URigVMBuildData::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SplitVariantFromSet"),
            &raw mut U_RIG_VM_BUILD_DATA_SPLIT_VARIANT_FROM_SET,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("JoinVariantSet"),
            &raw mut U_RIG_VM_BUILD_DATA_JOIN_VARIANT_SET,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetVariantRefForAsset"),
            &raw mut U_RIG_VM_BUILD_DATA_GET_VARIANT_REF_FOR_ASSET,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetUsedFunctionIdentifiers"),
            &raw mut U_RIG_VM_BUILD_DATA_GET_USED_FUNCTION_IDENTIFIERS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetFunctionIdentifierForVariant"),
            &raw mut U_RIG_VM_BUILD_DATA_GET_FUNCTION_IDENTIFIER_FOR_VARIANT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetAssetDataForVariant"),
            &raw mut U_RIG_VM_BUILD_DATA_GET_ASSET_DATA_FOR_VARIANT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetAssetDataForPath"),
            &raw mut U_RIG_VM_BUILD_DATA_GET_ASSET_DATA_FOR_PATH,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetAllFunctionIdentifiers"),
            &raw mut U_RIG_VM_BUILD_DATA_GET_ALL_FUNCTION_IDENTIFIERS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("Get"),
            &raw mut U_RIG_VM_BUILD_DATA_GET,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GatherFunctionVariantRefsForAsset"),
            &raw mut U_RIG_VM_BUILD_DATA_GATHER_FUNCTION_VARIANT_REFS_FOR_ASSET,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GatherAllFunctionVariantRefs"),
            &raw mut U_RIG_VM_BUILD_DATA_GATHER_ALL_FUNCTION_VARIANT_REFS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GatherAllAssetVariantRefs"),
            &raw mut U_RIG_VM_BUILD_DATA_GATHER_ALL_ASSET_VARIANT_REFS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("FindFunctionVariantRefs"),
            &raw mut U_RIG_VM_BUILD_DATA_FIND_FUNCTION_VARIANT_REFS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("FindAssetVariantRefs"),
            &raw mut U_RIG_VM_BUILD_DATA_FIND_ASSET_VARIANT_REFS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CreateFunctionVariant"),
            &raw mut U_RIG_VM_BUILD_DATA_CREATE_FUNCTION_VARIANT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CreateAssetVariant"),
            &raw mut U_RIG_VM_BUILD_DATA_CREATE_ASSET_VARIANT,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = URigVMActionStack::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("Undo"),
            &raw mut U_RIG_VM_ACTION_STACK_UNDO,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("Redo"),
            &raw mut U_RIG_VM_ACTION_STACK_REDO,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("OpenUndoBracket"),
            &raw mut U_RIG_VM_ACTION_STACK_OPEN_UNDO_BRACKET,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CloseUndoBracket"),
            &raw mut U_RIG_VM_ACTION_STACK_CLOSE_UNDO_BRACKET,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CancelUndoBracket"),
            &raw mut U_RIG_VM_ACTION_STACK_CANCEL_UNDO_BRACKET,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = URigVMGraph::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetSchemaClass"),
            &raw mut U_RIG_VM_GRAPH_SET_SCHEMA_CLASS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetDefaultFunctionLibrary"),
            &raw mut U_RIG_VM_GRAPH_SET_DEFAULT_FUNCTION_LIBRARY,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("IsTopLevelGraph"),
            &raw mut U_RIG_VM_GRAPH_IS_TOP_LEVEL_GRAPH,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("IsRootGraph"),
            &raw mut U_RIG_VM_GRAPH_IS_ROOT_GRAPH,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("IsNodeSelected"),
            &raw mut U_RIG_VM_GRAPH_IS_NODE_SELECTED,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("IsNodeHighlighted"),
            &raw mut U_RIG_VM_GRAPH_IS_NODE_HIGHLIGHTED,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetVariableDescriptions"),
            &raw mut U_RIG_VM_GRAPH_GET_VARIABLE_DESCRIPTIONS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetSelectNodes"),
            &raw mut U_RIG_VM_GRAPH_GET_SELECT_NODES,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetSchemaClass"),
            &raw mut U_RIG_VM_GRAPH_GET_SCHEMA_CLASS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetSchema"),
            &raw mut U_RIG_VM_GRAPH_GET_SCHEMA,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetRootGraph"),
            &raw mut U_RIG_VM_GRAPH_GET_ROOT_GRAPH,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetReturnNode"),
            &raw mut U_RIG_VM_GRAPH_GET_RETURN_NODE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetParentGraph"),
            &raw mut U_RIG_VM_GRAPH_GET_PARENT_GRAPH,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetOutputArguments"),
            &raw mut U_RIG_VM_GRAPH_GET_OUTPUT_ARGUMENTS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetNodes"),
            &raw mut U_RIG_VM_GRAPH_GET_NODES,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetNodePath"),
            &raw mut U_RIG_VM_GRAPH_GET_NODE_PATH,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetLocalVariables"),
            &raw mut U_RIG_VM_GRAPH_GET_LOCAL_VARIABLES,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetLinks"),
            &raw mut U_RIG_VM_GRAPH_GET_LINKS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetInputArguments"),
            &raw mut U_RIG_VM_GRAPH_GET_INPUT_ARGUMENTS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetGraphName"),
            &raw mut U_RIG_VM_GRAPH_GET_GRAPH_NAME,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetGraphDepth"),
            &raw mut U_RIG_VM_GRAPH_GET_GRAPH_DEPTH,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetEventNames"),
            &raw mut U_RIG_VM_GRAPH_GET_EVENT_NAMES,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetEntryNode"),
            &raw mut U_RIG_VM_GRAPH_GET_ENTRY_NODE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetDefaultFunctionLibrary"),
            &raw mut U_RIG_VM_GRAPH_GET_DEFAULT_FUNCTION_LIBRARY,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetContainedGraphs"),
            &raw mut U_RIG_VM_GRAPH_GET_CONTAINED_GRAPHS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("FindPin"),
            &raw mut U_RIG_VM_GRAPH_FIND_PIN,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("FindNodeByName"),
            &raw mut U_RIG_VM_GRAPH_FIND_NODE_BY_NAME,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("FindNode"),
            &raw mut U_RIG_VM_GRAPH_FIND_NODE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("FindLink"),
            &raw mut U_RIG_VM_GRAPH_FIND_LINK,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ContainsLink"),
            &raw mut U_RIG_VM_GRAPH_CONTAINS_LINK,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = URigVMFunctionLibrary::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetReferencesForFunction"),
            &raw mut U_RIG_VM_FUNCTION_LIBRARY_GET_REFERENCES_FOR_FUNCTION,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetReferencePathsForFunction"),
            &raw mut U_RIG_VM_FUNCTION_LIBRARY_GET_REFERENCE_PATHS_FOR_FUNCTION,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetFunctions"),
            &raw mut U_RIG_VM_FUNCTION_LIBRARY_GET_FUNCTIONS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("FindFunctionForNode"),
            &raw mut U_RIG_VM_FUNCTION_LIBRARY_FIND_FUNCTION_FOR_NODE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("FindFunction"),
            &raw mut U_RIG_VM_FUNCTION_LIBRARY_FIND_FUNCTION,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = URigVMLink::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetTargetPin"),
            &raw mut U_RIG_VM_LINK_GET_TARGET_PIN,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetTargetNode"),
            &raw mut U_RIG_VM_LINK_GET_TARGET_NODE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetSourcePin"),
            &raw mut U_RIG_VM_LINK_GET_SOURCE_PIN,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetSourceNode"),
            &raw mut U_RIG_VM_LINK_GET_SOURCE_NODE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetPinPathRepresentation"),
            &raw mut U_RIG_VM_LINK_GET_PIN_PATH_REPRESENTATION,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetOppositePin"),
            &raw mut U_RIG_VM_LINK_GET_OPPOSITE_PIN,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetLinkIndex"),
            &raw mut U_RIG_VM_LINK_GET_LINK_INDEX,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetGraphDepth"),
            &raw mut U_RIG_VM_LINK_GET_GRAPH_DEPTH,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetGraph"),
            &raw mut U_RIG_VM_LINK_GET_GRAPH,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = URigVMInjectionInfo::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetPin"),
            &raw mut U_RIG_VM_INJECTION_INFO_GET_PIN,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetGraph"),
            &raw mut U_RIG_VM_INJECTION_INFO_GET_GRAPH,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = URigVMPin::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ShouldOnlyShowSubPins"),
            &raw mut U_RIG_VM_PIN_SHOULD_ONLY_SHOW_SUB_PINS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ShouldHideSubPins"),
            &raw mut U_RIG_VM_PIN_SHOULD_HIDE_SUB_PINS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RequiresWatch"),
            &raw mut U_RIG_VM_PIN_REQUIRES_WATCH,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("IsWildCard"),
            &raw mut U_RIG_VM_PIN_IS_WILD_CARD,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("IsValidDefaultValue"),
            &raw mut U_RIG_VM_PIN_IS_VALID_DEFAULT_VALUE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("IsUObject"),
            &raw mut U_RIG_VM_PIN_IS_U_OBJECT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("IsTraitPin"),
            &raw mut U_RIG_VM_PIN_IS_TRAIT_PIN,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("IsStructMember"),
            &raw mut U_RIG_VM_PIN_IS_STRUCT_MEMBER,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("IsStruct"),
            &raw mut U_RIG_VM_PIN_IS_STRUCT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("IsStringType"),
            &raw mut U_RIG_VM_PIN_IS_STRING_TYPE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("IsRootPin"),
            &raw mut U_RIG_VM_PIN_IS_ROOT_PIN,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("IsReferenceCountedContainer"),
            &raw mut U_RIG_VM_PIN_IS_REFERENCE_COUNTED_CONTAINER,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("IsLinkedTo"),
            &raw mut U_RIG_VM_PIN_IS_LINKED_TO,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("IsLazy"),
            &raw mut U_RIG_VM_PIN_IS_LAZY,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("IsInterface"),
            &raw mut U_RIG_VM_PIN_IS_INTERFACE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("IsFixedSizeArray"),
            &raw mut U_RIG_VM_PIN_IS_FIXED_SIZE_ARRAY,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("IsExpanded"),
            &raw mut U_RIG_VM_PIN_IS_EXPANDED,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("IsExecuteContext"),
            &raw mut U_RIG_VM_PIN_IS_EXECUTE_CONTEXT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("IsEnum"),
            &raw mut U_RIG_VM_PIN_IS_ENUM,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("IsDynamicArray"),
            &raw mut U_RIG_VM_PIN_IS_DYNAMIC_ARRAY,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("IsDefinedAsConstant"),
            &raw mut U_RIG_VM_PIN_IS_DEFINED_AS_CONSTANT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("IsArrayElement"),
            &raw mut U_RIG_VM_PIN_IS_ARRAY_ELEMENT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("IsArray"),
            &raw mut U_RIG_VM_PIN_IS_ARRAY,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("HasUserProvidedDefaultValue"),
            &raw mut U_RIG_VM_PIN_HAS_USER_PROVIDED_DEFAULT_VALUE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("HasOriginalDefaultValue"),
            &raw mut U_RIG_VM_PIN_HAS_ORIGINAL_DEFAULT_VALUE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("HasMetaData"),
            &raw mut U_RIG_VM_PIN_HAS_META_DATA,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("HasDefaultValueOverride"),
            &raw mut U_RIG_VM_PIN_HAS_DEFAULT_VALUE_OVERRIDE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetToolTipText"),
            &raw mut U_RIG_VM_PIN_GET_TOOL_TIP_TEXT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetTargetLinks"),
            &raw mut U_RIG_VM_PIN_GET_TARGET_LINKS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetSubPins"),
            &raw mut U_RIG_VM_PIN_GET_SUB_PINS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetSubPinPath"),
            &raw mut U_RIG_VM_PIN_GET_SUB_PIN_PATH,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetSourceLinks"),
            &raw mut U_RIG_VM_PIN_GET_SOURCE_LINKS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetSegmentPath"),
            &raw mut U_RIG_VM_PIN_GET_SEGMENT_PATH,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetScriptStruct"),
            &raw mut U_RIG_VM_PIN_GET_SCRIPT_STRUCT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetRootPin"),
            &raw mut U_RIG_VM_PIN_GET_ROOT_PIN,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetPinPath"),
            &raw mut U_RIG_VM_PIN_GET_PIN_PATH,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetPinIndex"),
            &raw mut U_RIG_VM_PIN_GET_PIN_INDEX,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetPinForLink"),
            &raw mut U_RIG_VM_PIN_GET_PIN_FOR_LINK,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetParentScriptStruct"),
            &raw mut U_RIG_VM_PIN_GET_PARENT_SCRIPT_STRUCT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetParentPin"),
            &raw mut U_RIG_VM_PIN_GET_PARENT_PIN,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetOriginalPinFromInjectedNode"),
            &raw mut U_RIG_VM_PIN_GET_ORIGINAL_PIN_FROM_INJECTED_NODE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetOriginalDefaultValue"),
            &raw mut U_RIG_VM_PIN_GET_ORIGINAL_DEFAULT_VALUE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetNode"),
            &raw mut U_RIG_VM_PIN_GET_NODE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetMetaData"),
            &raw mut U_RIG_VM_PIN_GET_META_DATA,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetLinks"),
            &raw mut U_RIG_VM_PIN_GET_LINKS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetLinkedTargetPins"),
            &raw mut U_RIG_VM_PIN_GET_LINKED_TARGET_PINS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetLinkedSourcePins"),
            &raw mut U_RIG_VM_PIN_GET_LINKED_SOURCE_PINS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetIndexInCategory"),
            &raw mut U_RIG_VM_PIN_GET_INDEX_IN_CATEGORY,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetGraph"),
            &raw mut U_RIG_VM_PIN_GET_GRAPH,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetEnum"),
            &raw mut U_RIG_VM_PIN_GET_ENUM,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetDisplayName"),
            &raw mut U_RIG_VM_PIN_GET_DISPLAY_NAME,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetDirection"),
            &raw mut U_RIG_VM_PIN_GET_DIRECTION,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetDefaultValueType"),
            &raw mut U_RIG_VM_PIN_GET_DEFAULT_VALUE_TYPE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetDefaultValue"),
            &raw mut U_RIG_VM_PIN_GET_DEFAULT_VALUE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetCustomWidgetName"),
            &raw mut U_RIG_VM_PIN_GET_CUSTOM_WIDGET_NAME,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetCPPTypeObject"),
            &raw mut U_RIG_VM_PIN_GET_CPP_TYPE_OBJECT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetCPPType"),
            &raw mut U_RIG_VM_PIN_GET_CPP_TYPE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetCategory"),
            &raw mut U_RIG_VM_PIN_GET_CATEGORY,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetArraySize"),
            &raw mut U_RIG_VM_PIN_GET_ARRAY_SIZE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetArrayElementCppType"),
            &raw mut U_RIG_VM_PIN_GET_ARRAY_ELEMENT_CPP_TYPE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetAllSubPinsRecursively"),
            &raw mut U_RIG_VM_PIN_GET_ALL_SUB_PINS_RECURSIVELY,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetAbsolutePinIndex"),
            &raw mut U_RIG_VM_PIN_GET_ABSOLUTE_PIN_INDEX,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("FindSubPin"),
            &raw mut U_RIG_VM_PIN_FIND_SUB_PIN,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("FindLinkForPin"),
            &raw mut U_RIG_VM_PIN_FIND_LINK_FOR_PIN,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ContainsWildCardSubPin"),
            &raw mut U_RIG_VM_PIN_CONTAINS_WILD_CARD_SUB_PIN,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CanProvideDefaultValue"),
            &raw mut U_RIG_VM_PIN_CAN_PROVIDE_DEFAULT_VALUE,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = URigVMUserWorkflowRegistry::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("UnregisterProvider"),
            &raw mut U_RIG_VM_USER_WORKFLOW_REGISTRY_UNREGISTER_PROVIDER,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RegisterProvider"),
            &raw mut U_RIG_VM_USER_WORKFLOW_REGISTRY_REGISTER_PROVIDER,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetWorkflows"),
            &raw mut U_RIG_VM_USER_WORKFLOW_REGISTRY_GET_WORKFLOWS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("Get"),
            &raw mut U_RIG_VM_USER_WORKFLOW_REGISTRY_GET,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = URigVMController::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("UpgradeNodes"),
            &raw mut U_RIG_VM_CONTROLLER_UPGRADE_NODES,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("UnresolveTemplateNodes"),
            &raw mut U_RIG_VM_CONTROLLER_UNRESOLVE_TEMPLATE_NODES,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("Undo"),
            &raw mut U_RIG_VM_CONTROLLER_UNDO,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("UnbindPinFromVariable"),
            &raw mut U_RIG_VM_CONTROLLER_UNBIND_PIN_FROM_VARIABLE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SwapFunctionReferenceByName"),
            &raw mut U_RIG_VM_CONTROLLER_SWAP_FUNCTION_REFERENCE_BY_NAME,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SwapFunctionReference"),
            &raw mut U_RIG_VM_CONTROLLER_SWAP_FUNCTION_REFERENCE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SwapAllFunctionReferences"),
            &raw mut U_RIG_VM_CONTROLLER_SWAP_ALL_FUNCTION_REFERENCES,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SplitFunctionVariant"),
            &raw mut U_RIG_VM_CONTROLLER_SPLIT_FUNCTION_VARIANT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetUnitNodeDefaults"),
            &raw mut U_RIG_VM_CONTROLLER_SET_UNIT_NODE_DEFAULTS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetSchemaClass"),
            &raw mut U_RIG_VM_CONTROLLER_SET_SCHEMA_CLASS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetSchema"),
            &raw mut U_RIG_VM_CONTROLLER_SET_SCHEMA,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetRemappedVariable"),
            &raw mut U_RIG_VM_CONTROLLER_SET_REMAPPED_VARIABLE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetPinIsWatched"),
            &raw mut U_RIG_VM_CONTROLLER_SET_PIN_IS_WATCHED,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetPinIndexInCategory"),
            &raw mut U_RIG_VM_CONTROLLER_SET_PIN_INDEX_IN_CATEGORY,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetPinExpansion"),
            &raw mut U_RIG_VM_CONTROLLER_SET_PIN_EXPANSION,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetPinDisplayName"),
            &raw mut U_RIG_VM_CONTROLLER_SET_PIN_DISPLAY_NAME,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetPinDefaultValue"),
            &raw mut U_RIG_VM_CONTROLLER_SET_PIN_DEFAULT_VALUE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetPinCategoryIndex"),
            &raw mut U_RIG_VM_CONTROLLER_SET_PIN_CATEGORY_INDEX,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetPinCategoryExpansion"),
            &raw mut U_RIG_VM_CONTROLLER_SET_PIN_CATEGORY_EXPANSION,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetPinCategory"),
            &raw mut U_RIG_VM_CONTROLLER_SET_PIN_CATEGORY,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetNodeTitleByName"),
            &raw mut U_RIG_VM_CONTROLLER_SET_NODE_TITLE_BY_NAME,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetNodeTitle"),
            &raw mut U_RIG_VM_CONTROLLER_SET_NODE_TITLE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetNodeSizeByName"),
            &raw mut U_RIG_VM_CONTROLLER_SET_NODE_SIZE_BY_NAME,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetNodeSize"),
            &raw mut U_RIG_VM_CONTROLLER_SET_NODE_SIZE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetNodeSelection"),
            &raw mut U_RIG_VM_CONTROLLER_SET_NODE_SELECTION,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetNodePositionByName"),
            &raw mut U_RIG_VM_CONTROLLER_SET_NODE_POSITION_BY_NAME,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetNodePosition"),
            &raw mut U_RIG_VM_CONTROLLER_SET_NODE_POSITION,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetNodeLayout"),
            &raw mut U_RIG_VM_CONTROLLER_SET_NODE_LAYOUT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetNodeKeywordsByName"),
            &raw mut U_RIG_VM_CONTROLLER_SET_NODE_KEYWORDS_BY_NAME,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetNodeKeywords"),
            &raw mut U_RIG_VM_CONTROLLER_SET_NODE_KEYWORDS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetNodeDescriptionByName"),
            &raw mut U_RIG_VM_CONTROLLER_SET_NODE_DESCRIPTION_BY_NAME,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetNodeDescription"),
            &raw mut U_RIG_VM_CONTROLLER_SET_NODE_DESCRIPTION,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetNodeColorByName"),
            &raw mut U_RIG_VM_CONTROLLER_SET_NODE_COLOR_BY_NAME,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetNodeColor"),
            &raw mut U_RIG_VM_CONTROLLER_SET_NODE_COLOR,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetNodeCategoryByName"),
            &raw mut U_RIG_VM_CONTROLLER_SET_NODE_CATEGORY_BY_NAME,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetNodeCategory"),
            &raw mut U_RIG_VM_CONTROLLER_SET_NODE_CATEGORY,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetLocalVariableTypeFromObjectPath"),
            &raw mut U_RIG_VM_CONTROLLER_SET_LOCAL_VARIABLE_TYPE_FROM_OBJECT_PATH,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetLocalVariableType"),
            &raw mut U_RIG_VM_CONTROLLER_SET_LOCAL_VARIABLE_TYPE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetLocalVariableDefaultValue"),
            &raw mut U_RIG_VM_CONTROLLER_SET_LOCAL_VARIABLE_DEFAULT_VALUE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetIsRunningUnitTest"),
            &raw mut U_RIG_VM_CONTROLLER_SET_IS_RUNNING_UNIT_TEST,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetGraph"),
            &raw mut U_RIG_VM_CONTROLLER_SET_GRAPH,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetExposedPinIndex"),
            &raw mut U_RIG_VM_CONTROLLER_SET_EXPOSED_PIN_INDEX,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetCommentTextByName"),
            &raw mut U_RIG_VM_CONTROLLER_SET_COMMENT_TEXT_BY_NAME,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetCommentText"),
            &raw mut U_RIG_VM_CONTROLLER_SET_COMMENT_TEXT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetArrayPinSize"),
            &raw mut U_RIG_VM_CONTROLLER_SET_ARRAY_PIN_SIZE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetActionStack"),
            &raw mut U_RIG_VM_CONTROLLER_SET_ACTION_STACK,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SelectNodeIslands"),
            &raw mut U_RIG_VM_CONTROLLER_SELECT_NODE_ISLANDS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SelectNodeByName"),
            &raw mut U_RIG_VM_CONTROLLER_SELECT_NODE_BY_NAME,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SelectNode"),
            &raw mut U_RIG_VM_CONTROLLER_SELECT_NODE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SelectLinkedNodes"),
            &raw mut U_RIG_VM_CONTROLLER_SELECT_LINKED_NODES,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ResolveWildCardPin"),
            &raw mut U_RIG_VM_CONTROLLER_RESOLVE_WILD_CARD_PIN,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ResetPinDefaultValue"),
            &raw mut U_RIG_VM_CONTROLLER_RESET_PIN_DEFAULT_VALUE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ResetDefaultValueForPins"),
            &raw mut U_RIG_VM_CONTROLLER_RESET_DEFAULT_VALUE_FOR_PINS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ResetDefaultValueForAllPinsOnNodes"),
            &raw mut U_RIG_VM_CONTROLLER_RESET_DEFAULT_VALUE_FOR_ALL_PINS_ON_NODES,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ResetDefaultValueForAllPinsOnNode"),
            &raw mut U_RIG_VM_CONTROLLER_RESET_DEFAULT_VALUE_FOR_ALL_PINS_ON_NODE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ReplaceParameterNodeWithVariable"),
            &raw mut U_RIG_VM_CONTROLLER_REPLACE_PARAMETER_NODE_WITH_VARIABLE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RenameVariable"),
            &raw mut U_RIG_VM_CONTROLLER_RENAME_VARIABLE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RenamePinCategory"),
            &raw mut U_RIG_VM_CONTROLLER_RENAME_PIN_CATEGORY,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RenameParameter"),
            &raw mut U_RIG_VM_CONTROLLER_RENAME_PARAMETER,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RenameNode"),
            &raw mut U_RIG_VM_CONTROLLER_RENAME_NODE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RenameLocalVariable"),
            &raw mut U_RIG_VM_CONTROLLER_RENAME_LOCAL_VARIABLE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RenameFunction"),
            &raw mut U_RIG_VM_CONTROLLER_RENAME_FUNCTION,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RenameExposedPin"),
            &raw mut U_RIG_VM_CONTROLLER_RENAME_EXPOSED_PIN,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RemoveTrait"),
            &raw mut U_RIG_VM_CONTROLLER_REMOVE_TRAIT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RemoveTagFromFunctionVariant"),
            &raw mut U_RIG_VM_CONTROLLER_REMOVE_TAG_FROM_FUNCTION_VARIANT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RemovePinCategory"),
            &raw mut U_RIG_VM_CONTROLLER_REMOVE_PIN_CATEGORY,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RemoveNodesByName"),
            &raw mut U_RIG_VM_CONTROLLER_REMOVE_NODES_BY_NAME,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RemoveNodes"),
            &raw mut U_RIG_VM_CONTROLLER_REMOVE_NODES,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RemoveNodeByName"),
            &raw mut U_RIG_VM_CONTROLLER_REMOVE_NODE_BY_NAME,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RemoveNode"),
            &raw mut U_RIG_VM_CONTROLLER_REMOVE_NODE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RemoveLocalVariable"),
            &raw mut U_RIG_VM_CONTROLLER_REMOVE_LOCAL_VARIABLE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RemoveInjectedNode"),
            &raw mut U_RIG_VM_CONTROLLER_REMOVE_INJECTED_NODE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RemoveFunctionFromLibrary"),
            &raw mut U_RIG_VM_CONTROLLER_REMOVE_FUNCTION_FROM_LIBRARY,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RemoveExposedPin"),
            &raw mut U_RIG_VM_CONTROLLER_REMOVE_EXPOSED_PIN,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RemoveArrayPin"),
            &raw mut U_RIG_VM_CONTROLLER_REMOVE_ARRAY_PIN,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RemoveAggregatePin"),
            &raw mut U_RIG_VM_CONTROLLER_REMOVE_AGGREGATE_PIN,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RefreshVariableNode"),
            &raw mut U_RIG_VM_CONTROLLER_REFRESH_VARIABLE_NODE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("Redo"),
            &raw mut U_RIG_VM_CONTROLLER_REDO,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("PushGraph"),
            &raw mut U_RIG_VM_CONTROLLER_PUSH_GRAPH,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("PromotePinToVariable"),
            &raw mut U_RIG_VM_CONTROLLER_PROMOTE_PIN_TO_VARIABLE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("PromoteFunctionReferenceNodeToCollapseNode"),
            &raw mut U_RIG_VM_CONTROLLER_PROMOTE_FUNCTION_REFERENCE_NODE_TO_COLLAPSE_NODE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("PromoteCollapseNodeToFunctionReferenceNode"),
            &raw mut U_RIG_VM_CONTROLLER_PROMOTE_COLLAPSE_NODE_TO_FUNCTION_REFERENCE_NODE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("PopGraph"),
            &raw mut U_RIG_VM_CONTROLLER_POP_GRAPH,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("PerformUserWorkflow"),
            &raw mut U_RIG_VM_CONTROLLER_PERFORM_USER_WORKFLOW,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("OpenUndoBracket"),
            &raw mut U_RIG_VM_CONTROLLER_OPEN_UNDO_BRACKET,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("MarkFunctionAsPublic"),
            &raw mut U_RIG_VM_CONTROLLER_MARK_FUNCTION_AS_PUBLIC,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("MakeVariableNodeFromBinding"),
            &raw mut U_RIG_VM_CONTROLLER_MAKE_VARIABLE_NODE_FROM_BINDING,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("MakeOptionsForWorkflow"),
            &raw mut U_RIG_VM_CONTROLLER_MAKE_OPTIONS_FOR_WORKFLOW,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("MakeBindingsFromVariableNode"),
            &raw mut U_RIG_VM_CONTROLLER_MAKE_BINDINGS_FROM_VARIABLE_NODE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("LocalizeFunctions"),
            &raw mut U_RIG_VM_CONTROLLER_LOCALIZE_FUNCTIONS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("LocalizeFunctionFromPath"),
            &raw mut U_RIG_VM_CONTROLLER_LOCALIZE_FUNCTION_FROM_PATH,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("LocalizeFunction"),
            &raw mut U_RIG_VM_CONTROLLER_LOCALIZE_FUNCTION,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("JoinFunctionVariant"),
            &raw mut U_RIG_VM_CONTROLLER_JOIN_FUNCTION_VARIANT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("IsTransacting"),
            &raw mut U_RIG_VM_CONTROLLER_IS_TRANSACTING,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("IsReportingEnabled"),
            &raw mut U_RIG_VM_CONTROLLER_IS_REPORTING_ENABLED,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("IsFunctionPublic"),
            &raw mut U_RIG_VM_CONTROLLER_IS_FUNCTION_PUBLIC,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("InsertArrayPin"),
            &raw mut U_RIG_VM_CONTROLLER_INSERT_ARRAY_PIN,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ImportNodesFromText"),
            &raw mut U_RIG_VM_CONTROLLER_IMPORT_NODES_FROM_TEXT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetUnitStructsForTemplate"),
            &raw mut U_RIG_VM_CONTROLLER_GET_UNIT_STRUCTS_FOR_TEMPLATE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetTopLevelGraph"),
            &raw mut U_RIG_VM_CONTROLLER_GET_TOP_LEVEL_GRAPH,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetTemplateForUnitStruct"),
            &raw mut U_RIG_VM_CONTROLLER_GET_TEMPLATE_FOR_UNIT_STRUCT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetSchema"),
            &raw mut U_RIG_VM_CONTROLLER_GET_SCHEMA,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetRegisteredUnitStructs"),
            &raw mut U_RIG_VM_CONTROLLER_GET_REGISTERED_UNIT_STRUCTS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetRegisteredTemplates"),
            &raw mut U_RIG_VM_CONTROLLER_GET_REGISTERED_TEMPLATES,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetPinDefaultValue"),
            &raw mut U_RIG_VM_CONTROLLER_GET_PIN_DEFAULT_VALUE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetGraph"),
            &raw mut U_RIG_VM_CONTROLLER_GET_GRAPH,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetControllerForGraph"),
            &raw mut U_RIG_VM_CONTROLLER_GET_CONTROLLER_FOR_GRAPH,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetActionStack"),
            &raw mut U_RIG_VM_CONTROLLER_GET_ACTION_STACK,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GeneratePythonCommands"),
            &raw mut U_RIG_VM_CONTROLLER_GENERATE_PYTHON_COMMANDS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("FindVariantsOfFunction"),
            &raw mut U_RIG_VM_CONTROLLER_FIND_VARIANTS_OF_FUNCTION,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("FindGraphFunctionIdentifier"),
            &raw mut U_RIG_VM_CONTROLLER_FIND_GRAPH_FUNCTION_IDENTIFIER,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("FindGraphFunctionHeaderByName"),
            &raw mut U_RIG_VM_CONTROLLER_FIND_GRAPH_FUNCTION_HEADER_BY_NAME,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("FindGraphFunctionHeader"),
            &raw mut U_RIG_VM_CONTROLLER_FIND_GRAPH_FUNCTION_HEADER,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ExportSelectedNodesToText"),
            &raw mut U_RIG_VM_CONTROLLER_EXPORT_SELECTED_NODES_TO_TEXT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ExportNodeToText"),
            &raw mut U_RIG_VM_CONTROLLER_EXPORT_NODE_TO_TEXT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ExportNodesToText"),
            &raw mut U_RIG_VM_CONTROLLER_EXPORT_NODES_TO_TEXT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ExpandLibraryNode"),
            &raw mut U_RIG_VM_CONTROLLER_EXPAND_LIBRARY_NODE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("EnableReporting"),
            &raw mut U_RIG_VM_CONTROLLER_ENABLE_REPORTING,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("EjectNodeFromPin"),
            &raw mut U_RIG_VM_CONTROLLER_EJECT_NODE_FROM_PIN,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("DuplicateArrayPin"),
            &raw mut U_RIG_VM_CONTROLLER_DUPLICATE_ARRAY_PIN,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CreateFunctionVariant"),
            &raw mut U_RIG_VM_CONTROLLER_CREATE_FUNCTION_VARIANT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CollapseNodes"),
            &raw mut U_RIG_VM_CONTROLLER_COLLAPSE_NODES,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CloseUndoBracket"),
            &raw mut U_RIG_VM_CONTROLLER_CLOSE_UNDO_BRACKET,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ClearPinCategory"),
            &raw mut U_RIG_VM_CONTROLLER_CLEAR_PIN_CATEGORY,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ClearOverrideOnPins"),
            &raw mut U_RIG_VM_CONTROLLER_CLEAR_OVERRIDE_ON_PINS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ClearOverrideOnPin"),
            &raw mut U_RIG_VM_CONTROLLER_CLEAR_OVERRIDE_ON_PIN,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ClearOverrideOnAllPinsOnNodes"),
            &raw mut U_RIG_VM_CONTROLLER_CLEAR_OVERRIDE_ON_ALL_PINS_ON_NODES,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ClearOverrideOnAllPinsOnNode"),
            &raw mut U_RIG_VM_CONTROLLER_CLEAR_OVERRIDE_ON_ALL_PINS_ON_NODE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ClearNodeSelection"),
            &raw mut U_RIG_VM_CONTROLLER_CLEAR_NODE_SELECTION,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ClearNodeLayout"),
            &raw mut U_RIG_VM_CONTROLLER_CLEAR_NODE_LAYOUT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ClearArrayPin"),
            &raw mut U_RIG_VM_CONTROLLER_CLEAR_ARRAY_PIN,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ChangeExposedPinType"),
            &raw mut U_RIG_VM_CONTROLLER_CHANGE_EXPOSED_PIN_TYPE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CanImportNodesFromText"),
            &raw mut U_RIG_VM_CONTROLLER_CAN_IMPORT_NODES_FROM_TEXT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CancelUndoBracket"),
            &raw mut U_RIG_VM_CONTROLLER_CANCEL_UNDO_BRACKET,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("BreakLink"),
            &raw mut U_RIG_VM_CONTROLLER_BREAK_LINK,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("BreakAllLinks"),
            &raw mut U_RIG_VM_CONTROLLER_BREAK_ALL_LINKS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("BindPinToVariable"),
            &raw mut U_RIG_VM_CONTROLLER_BIND_PIN_TO_VARIABLE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AddVariableNodeFromObjectPath"),
            &raw mut U_RIG_VM_CONTROLLER_ADD_VARIABLE_NODE_FROM_OBJECT_PATH,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AddVariableNode"),
            &raw mut U_RIG_VM_CONTROLLER_ADD_VARIABLE_NODE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AddUnitNodeWithDefaults"),
            &raw mut U_RIG_VM_CONTROLLER_ADD_UNIT_NODE_WITH_DEFAULTS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AddUnitNodeFromStructPath"),
            &raw mut U_RIG_VM_CONTROLLER_ADD_UNIT_NODE_FROM_STRUCT_PATH,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AddUnitNode"),
            &raw mut U_RIG_VM_CONTROLLER_ADD_UNIT_NODE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AddTrait"),
            &raw mut U_RIG_VM_CONTROLLER_ADD_TRAIT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AddTemplateNode"),
            &raw mut U_RIG_VM_CONTROLLER_ADD_TEMPLATE_NODE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AddTagToFunctionVariant"),
            &raw mut U_RIG_VM_CONTROLLER_ADD_TAG_TO_FUNCTION_VARIANT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AddSelectNodeFromStruct"),
            &raw mut U_RIG_VM_CONTROLLER_ADD_SELECT_NODE_FROM_STRUCT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AddSelectNode"),
            &raw mut U_RIG_VM_CONTROLLER_ADD_SELECT_NODE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AddRerouteNodeOnPin"),
            &raw mut U_RIG_VM_CONTROLLER_ADD_REROUTE_NODE_ON_PIN,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AddRerouteNodeOnLinkPath"),
            &raw mut U_RIG_VM_CONTROLLER_ADD_REROUTE_NODE_ON_LINK_PATH,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AddRerouteNodeOnLink"),
            &raw mut U_RIG_VM_CONTROLLER_ADD_REROUTE_NODE_ON_LINK,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AddParameterNodeFromObjectPath"),
            &raw mut U_RIG_VM_CONTROLLER_ADD_PARAMETER_NODE_FROM_OBJECT_PATH,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AddParameterNode"),
            &raw mut U_RIG_VM_CONTROLLER_ADD_PARAMETER_NODE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AddOverrideToPins"),
            &raw mut U_RIG_VM_CONTROLLER_ADD_OVERRIDE_TO_PINS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AddOverrideToPin"),
            &raw mut U_RIG_VM_CONTROLLER_ADD_OVERRIDE_TO_PIN,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AddOverrideToAllPinsOnNodes"),
            &raw mut U_RIG_VM_CONTROLLER_ADD_OVERRIDE_TO_ALL_PINS_ON_NODES,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AddOverrideToAllPinsOnNode"),
            &raw mut U_RIG_VM_CONTROLLER_ADD_OVERRIDE_TO_ALL_PINS_ON_NODE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AddLocalVariableFromObjectPath"),
            &raw mut U_RIG_VM_CONTROLLER_ADD_LOCAL_VARIABLE_FROM_OBJECT_PATH,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AddLocalVariable"),
            &raw mut U_RIG_VM_CONTROLLER_ADD_LOCAL_VARIABLE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AddLink"),
            &raw mut U_RIG_VM_CONTROLLER_ADD_LINK,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AddInvokeEntryNode"),
            &raw mut U_RIG_VM_CONTROLLER_ADD_INVOKE_ENTRY_NODE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AddInjectedNodeFromStructPath"),
            &raw mut U_RIG_VM_CONTROLLER_ADD_INJECTED_NODE_FROM_STRUCT_PATH,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AddInjectedNode"),
            &raw mut U_RIG_VM_CONTROLLER_ADD_INJECTED_NODE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AddIfNodeFromStruct"),
            &raw mut U_RIG_VM_CONTROLLER_ADD_IF_NODE_FROM_STRUCT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AddIfNode"),
            &raw mut U_RIG_VM_CONTROLLER_ADD_IF_NODE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AddFunctionToLibrary"),
            &raw mut U_RIG_VM_CONTROLLER_ADD_FUNCTION_TO_LIBRARY,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AddFunctionReferenceNodeFromDescription"),
            &raw mut U_RIG_VM_CONTROLLER_ADD_FUNCTION_REFERENCE_NODE_FROM_DESCRIPTION,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AddFunctionReferenceNode"),
            &raw mut U_RIG_VM_CONTROLLER_ADD_FUNCTION_REFERENCE_NODE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AddFreeRerouteNode"),
            &raw mut U_RIG_VM_CONTROLLER_ADD_FREE_REROUTE_NODE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AddExternalFunctionReferenceNode"),
            &raw mut U_RIG_VM_CONTROLLER_ADD_EXTERNAL_FUNCTION_REFERENCE_NODE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AddExposedPin"),
            &raw mut U_RIG_VM_CONTROLLER_ADD_EXPOSED_PIN,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AddEnumNode"),
            &raw mut U_RIG_VM_CONTROLLER_ADD_ENUM_NODE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AddEmptyPinCategory"),
            &raw mut U_RIG_VM_CONTROLLER_ADD_EMPTY_PIN_CATEGORY,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AddDefaultTagToFunctionVariant"),
            &raw mut U_RIG_VM_CONTROLLER_ADD_DEFAULT_TAG_TO_FUNCTION_VARIANT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AddCommentNode"),
            &raw mut U_RIG_VM_CONTROLLER_ADD_COMMENT_NODE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AddBranchNode"),
            &raw mut U_RIG_VM_CONTROLLER_ADD_BRANCH_NODE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AddArrayPin"),
            &raw mut U_RIG_VM_CONTROLLER_ADD_ARRAY_PIN,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AddArrayNodeFromObjectPath"),
            &raw mut U_RIG_VM_CONTROLLER_ADD_ARRAY_NODE_FROM_OBJECT_PATH,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AddArrayNode"),
            &raw mut U_RIG_VM_CONTROLLER_ADD_ARRAY_NODE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AddAggregatePin"),
            &raw mut U_RIG_VM_CONTROLLER_ADD_AGGREGATE_PIN,
        );
    }
}
#[repr(C, align(8))]
pub struct FRigVMGraphVariableDescription {
    pub name: FName,
    pub cpp_type: FString,
    pub cpp_type_object: UPtr<crate::bindings::core_u_object::UObject>,
    #[doc(hidden)]
    __padding_56: [u8; 16],
    pub default_value: FString,
    pub category: FText,
    pub tooltip: FText,
    pub b_exposed_on_spawn: bool,
    pub b_expose_to_cinematics: bool,
    pub b_public: bool,
    pub b_private: bool,
}
impl FRigVMGraphVariableDescription {}
#[repr(C, align(8))]
pub struct FRigVMParserASTSettings {
    #[doc(hidden)]
    __padding_3: [u8; 3],
    pub b_setup_traits: bool,
    __padding_end: [u8; 52],
}
impl FRigVMParserASTSettings {}
#[repr(C, align(8))]
pub struct FRigVMCompileSettings {
    pub surpress_info_messages: bool,
    pub surpress_warnings: bool,
    pub surpress_errors: bool,
    pub enable_pin_watches: bool,
    pub ast_settings: FRigVMParserASTSettings,
    #[doc(hidden)]
    __padding_66: [u8; 2],
    pub b_warn_about_deprecated_nodes: bool,
    pub b_warn_about_duplicate_events: bool,
}
impl FRigVMCompileSettings {}
#[repr(C, align(8))]
pub struct FRigVMGraphParameterDescription {
    pub name: FName,
    pub b_is_input: bool,
    pub cpp_type: FString,
    pub cpp_type_object: UPtr<crate::bindings::core_u_object::UObject>,
    pub default_value: FString,
}
impl FRigVMGraphParameterDescription {}
#[repr(C, align(8))]
pub struct FRigVMFunctionReferenceArray {
    __padding_end: [u8; 16],
}
impl FRigVMFunctionReferenceArray {}
#[repr(C, align(8))]
pub struct FRigStructScope {
    __padding_end: [u8; 16],
}
impl FRigStructScope {}
#[repr(C, align(8))]
pub struct URigVMEdGraph {
    __padding_end: [u8; 456],
}
impl URigVMEdGraph {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("URigVMEdGraph")
            .unwrap()
    }
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
pub struct URigVMEdGraphNode {
    __padding_end: [u8; 1152],
}
impl URigVMEdGraphNode {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("URigVMEdGraphNode")
            .unwrap()
    }
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
pub struct URigVMEdGraphSchema {
    __padding_end: [u8; 168],
}
impl URigVMEdGraphSchema {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("URigVMEdGraphSchema")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
pub struct IRigVMAssetInterface {}
#[repr(C, align(8))]
pub struct URigVMAssetInterface {
    __padding_end: [u8; 48],
}
impl URigVMAssetInterface {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("URigVMAssetInterface")
            .unwrap()
    }
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
pub struct URigVMBlueprint {
    #[doc(hidden)]
    __padding_2992: [u8; 2992],
    pub vm_compile_settings: FRigVMCompileSettings,
    __padding_end: [u8; 744],
}
impl URigVMBlueprint {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("URigVMBlueprint")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
    pub fn suspend_notifications(&mut self, b_suspend_notifs: bool) {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_BLUEPRINT_SUSPEND_NOTIFICATIONS,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_suspend_notifs,
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
                crate::bindings::rig_vm_developer::U_RIG_VM_BLUEPRINT_SUSPEND_NOTIFICATIONS,
                __buffer,
            )
        };
    }
    pub fn split_asset_variant(&mut self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_BLUEPRINT_SPLIT_ASSET_VARIANT,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_BLUEPRINT_SPLIT_ASSET_VARIANT,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn set_auto_vm_recompile(&mut self, b_auto_recompile: bool) {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_BLUEPRINT_SET_AUTO_VM_RECOMPILE,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_auto_recompile,
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
                crate::bindings::rig_vm_developer::U_RIG_VM_BLUEPRINT_SET_AUTO_VM_RECOMPILE,
                __buffer,
            )
        };
    }
    pub fn request_rig_vm_init(&mut self) {
        let mut __stack = crate::core_data::StackAlloc::<0>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_BLUEPRINT_REQUEST_RIG_VM_INIT,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_BLUEPRINT_REQUEST_RIG_VM_INIT,
                __buffer,
            )
        };
    }
    pub fn request_auto_vm_recompilation(&mut self) {
        let mut __stack = crate::core_data::StackAlloc::<0>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_BLUEPRINT_REQUEST_AUTO_VM_RECOMPILATION,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_BLUEPRINT_REQUEST_AUTO_VM_RECOMPILATION,
                __buffer,
            )
        };
    }
    pub fn rename_member_variable(
        &mut self,
        in_old_name: &FName,
        in_new_name: &FName,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<25>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_BLUEPRINT_RENAME_MEMBER_VARIABLE,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_old_name,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_new_name,
                __buffer.add(12).cast::<FName>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_BLUEPRINT_RENAME_MEMBER_VARIABLE,
                __buffer,
            )
        };
        unsafe { __buffer.add(24).cast::<bool>().read() }
    }
    pub fn remove_model(
        &mut self,
        in_name: FString,
        b_setup_undo_redo: bool,
        b_print_python_command: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<19>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_BLUEPRINT_REMOVE_MODEL,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_name,
                __buffer.add(0).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_setup_undo_redo,
                __buffer.add(16).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_print_python_command,
                __buffer.add(17).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_BLUEPRINT_REMOVE_MODEL,
                __buffer,
            )
        };
        unsafe { __buffer.add(18).cast::<bool>().read() }
    }
    pub fn remove_member_variable(&mut self, in_name: &FName) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<13>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_BLUEPRINT_REMOVE_MEMBER_VARIABLE,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(in_name, __buffer.add(0).cast::<FName>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_BLUEPRINT_REMOVE_MEMBER_VARIABLE,
                __buffer,
            )
        };
        unsafe { __buffer.add(12).cast::<bool>().read() }
    }
    pub fn recompile_vm_if_required(&mut self) {
        let mut __stack = crate::core_data::StackAlloc::<0>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_BLUEPRINT_RECOMPILE_VM_IF_REQUIRED,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_BLUEPRINT_RECOMPILE_VM_IF_REQUIRED,
                __buffer,
            )
        };
    }
    pub fn recompile_vm(&mut self) {
        let mut __stack = crate::core_data::StackAlloc::<0>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_BLUEPRINT_RECOMPILE_VM,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_BLUEPRINT_RECOMPILE_VM,
                __buffer,
            )
        };
    }
    pub fn join_asset_variant(
        &mut self,
        in_guid: &crate::bindings::core_u_object::FGuid,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<17>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_BLUEPRINT_JOIN_ASSET_VARIANT,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_guid,
                __buffer.add(0).cast::<crate::bindings::core_u_object::FGuid>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_BLUEPRINT_JOIN_ASSET_VARIANT,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<bool>().read() }
    }
    pub fn get_rig_vm_host_class(
        &self,
    ) -> TSubclassOf<crate::bindings::core_u_object::UObject> {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_BLUEPRINT_GET_RIG_VM_HOST_CLASS,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_BLUEPRINT_GET_RIG_VM_HOST_CLASS,
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
    pub fn get_or_create_local_function_library(
        &mut self,
        b_setup_undo_redo: bool,
    ) -> UPtr<URigVMFunctionLibrary> {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_BLUEPRINT_GET_OR_CREATE_LOCAL_FUNCTION_LIBRARY,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_setup_undo_redo,
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
                crate::bindings::rig_vm_developer::U_RIG_VM_BLUEPRINT_GET_OR_CREATE_LOCAL_FUNCTION_LIBRARY,
                __buffer,
            )
        };
        unsafe { __buffer.add(8).cast::<UPtr<URigVMFunctionLibrary>>().read() }
    }
    pub fn get_or_create_controller(
        &mut self,
        in_graph: UPtr<URigVMGraph>,
    ) -> UPtr<URigVMController> {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_BLUEPRINT_GET_OR_CREATE_CONTROLLER,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_graph,
                __buffer.add(0).cast::<UPtr<URigVMGraph>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_BLUEPRINT_GET_OR_CREATE_CONTROLLER,
                __buffer,
            )
        };
        unsafe { __buffer.add(8).cast::<UPtr<URigVMController>>().read() }
    }
    pub fn get_model(
        &self,
        in_ed_graph: UPtr<crate::bindings::engine::UEdGraph>,
    ) -> UPtr<URigVMGraph> {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_BLUEPRINT_GET_MODEL,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_ed_graph,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::UEdGraph>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_BLUEPRINT_GET_MODEL,
                __buffer,
            )
        };
        unsafe { __buffer.add(8).cast::<UPtr<URigVMGraph>>().read() }
    }
    pub fn get_member_variables(&self) -> TArray<FRigVMGraphVariableDescription> {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_BLUEPRINT_GET_MEMBER_VARIABLES,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_BLUEPRINT_GET_MEMBER_VARIABLES,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<TArray<FRigVMGraphVariableDescription>>().read()
        }
    }
    pub fn get_matching_variants(
        &self,
    ) -> TArray<crate::bindings::rig_vm::FRigVMVariantRef> {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_BLUEPRINT_GET_MATCHING_VARIANTS,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_BLUEPRINT_GET_MATCHING_VARIANTS,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(0)
                .cast::<TArray<crate::bindings::rig_vm::FRigVMVariantRef>>()
                .read()
        }
    }
    pub fn get_local_function_library(&self) -> UPtr<URigVMFunctionLibrary> {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_BLUEPRINT_GET_LOCAL_FUNCTION_LIBRARY,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_BLUEPRINT_GET_LOCAL_FUNCTION_LIBRARY,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<UPtr<URigVMFunctionLibrary>>().read() }
    }
    pub fn get_focused_model(&self) -> UPtr<URigVMGraph> {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_BLUEPRINT_GET_FOCUSED_MODEL,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_BLUEPRINT_GET_FOCUSED_MODEL,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<UPtr<URigVMGraph>>().read() }
    }
    pub fn get_default_model(&self) -> UPtr<URigVMGraph> {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_BLUEPRINT_GET_DEFAULT_MODEL,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_BLUEPRINT_GET_DEFAULT_MODEL,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<UPtr<URigVMGraph>>().read() }
    }
    pub fn get_debugged_rig_vm_host(
        &mut self,
    ) -> UPtr<crate::bindings::rig_vm::URigVMHost> {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_BLUEPRINT_GET_DEBUGGED_RIG_VM_HOST,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_BLUEPRINT_GET_DEBUGGED_RIG_VM_HOST,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<UPtr<crate::bindings::rig_vm::URigVMHost>>().read()
        }
    }
    pub fn get_controller_by_name(
        &self,
        in_graph_name: FString,
    ) -> UPtr<URigVMController> {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_BLUEPRINT_GET_CONTROLLER_BY_NAME,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_graph_name,
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
                crate::bindings::rig_vm_developer::U_RIG_VM_BLUEPRINT_GET_CONTROLLER_BY_NAME,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<UPtr<URigVMController>>().read() }
    }
    pub fn get_controller(&self, in_graph: UPtr<URigVMGraph>) -> UPtr<URigVMController> {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_BLUEPRINT_GET_CONTROLLER,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_graph,
                __buffer.add(0).cast::<UPtr<URigVMGraph>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_BLUEPRINT_GET_CONTROLLER,
                __buffer,
            )
        };
        unsafe { __buffer.add(8).cast::<UPtr<URigVMController>>().read() }
    }
    pub fn get_available_rig_vm_structs(
        &self,
    ) -> TArray<UPtr<crate::bindings::core_u_object::UStruct>> {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_BLUEPRINT_GET_AVAILABLE_RIG_VM_STRUCTS,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_BLUEPRINT_GET_AVAILABLE_RIG_VM_STRUCTS,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(0)
                .cast::<TArray<UPtr<crate::bindings::core_u_object::UStruct>>>()
                .read()
        }
    }
    pub fn get_auto_vm_recompile(&self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_BLUEPRINT_GET_AUTO_VM_RECOMPILE,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_BLUEPRINT_GET_AUTO_VM_RECOMPILE,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn get_asset_variant_ref(&self) -> crate::bindings::rig_vm::FRigVMVariantRef {
        let mut __stack = crate::core_data::StackAlloc::<72>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_BLUEPRINT_GET_ASSET_VARIANT_REF,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_BLUEPRINT_GET_ASSET_VARIANT_REF,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<crate::bindings::rig_vm::FRigVMVariantRef>().read()
        }
    }
    pub fn get_asset_variant(&self) -> crate::bindings::rig_vm::FRigVMVariant {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_BLUEPRINT_GET_ASSET_VARIANT_BP,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_BLUEPRINT_GET_ASSET_VARIANT_BP,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<crate::bindings::rig_vm::FRigVMVariant>().read()
        }
    }
    pub fn get_all_models(&self) -> TArray<UPtr<URigVMGraph>> {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_BLUEPRINT_GET_ALL_MODELS,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_BLUEPRINT_GET_ALL_MODELS,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<TArray<UPtr<URigVMGraph>>>().read() }
    }
    pub fn generate_python_commands(
        &mut self,
        in_new_blueprint_name: FString,
    ) -> TArray<FString> {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_BLUEPRINT_GENERATE_PYTHON_COMMANDS,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_new_blueprint_name,
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
                crate::bindings::rig_vm_developer::U_RIG_VM_BLUEPRINT_GENERATE_PYTHON_COMMANDS,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<TArray<FString>>().read() }
    }
    pub fn create_rig_vm_host(&mut self) -> UPtr<crate::bindings::rig_vm::URigVMHost> {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_BLUEPRINT_CREATE_RIG_VM_HOST,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_BLUEPRINT_CREATE_RIG_VM_HOST,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<UPtr<crate::bindings::rig_vm::URigVMHost>>().read()
        }
    }
    pub fn change_member_variable_type(
        &mut self,
        in_name: &FName,
        in_cpp_type: FString,
        b_is_public: bool,
        b_is_read_only: bool,
        in_default_value: FString,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<57>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_BLUEPRINT_CHANGE_MEMBER_VARIABLE_TYPE,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(in_name, __buffer.add(0).cast::<FName>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_cpp_type,
                __buffer.add(16).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_is_public,
                __buffer.add(32).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_is_read_only,
                __buffer.add(33).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_default_value,
                __buffer.add(40).cast::<FString>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_BLUEPRINT_CHANGE_MEMBER_VARIABLE_TYPE,
                __buffer,
            )
        };
        unsafe { __buffer.add(56).cast::<bool>().read() }
    }
    pub fn bulk_remove_member_variables(&mut self, in_names: &TArray<FName>) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<17>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_BLUEPRINT_BULK_REMOVE_MEMBER_VARIABLES,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_names,
                __buffer.add(0).cast::<TArray<FName>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_BLUEPRINT_BULK_REMOVE_MEMBER_VARIABLES,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<bool>().read() }
    }
    pub fn add_model(
        &mut self,
        in_name: FString,
        b_setup_undo_redo: bool,
        b_print_python_command: bool,
    ) -> UPtr<URigVMGraph> {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_BLUEPRINT_ADD_MODEL,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_name,
                __buffer.add(0).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_setup_undo_redo,
                __buffer.add(16).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_print_python_command,
                __buffer.add(17).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_BLUEPRINT_ADD_MODEL,
                __buffer,
            )
        };
        unsafe { __buffer.add(24).cast::<UPtr<URigVMGraph>>().read() }
    }
    pub fn add_member_variable(
        &mut self,
        in_name: &FName,
        in_cpp_type: FString,
        b_is_public: bool,
        b_is_read_only: bool,
        in_default_value: FString,
    ) -> FName {
        let mut __stack = crate::core_data::StackAlloc::<68>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_BLUEPRINT_ADD_MEMBER_VARIABLE,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(in_name, __buffer.add(0).cast::<FName>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_cpp_type,
                __buffer.add(16).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_is_public,
                __buffer.add(32).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_is_read_only,
                __buffer.add(33).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_default_value,
                __buffer.add(40).cast::<FString>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_BLUEPRINT_ADD_MEMBER_VARIABLE,
                __buffer,
            )
        };
        unsafe { __buffer.add(56).cast::<FName>().read() }
    }
}
#[repr(C, align(8))]
pub struct URigVMCompiler {
    __padding_end: [u8; 312],
}
impl URigVMCompiler {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("URigVMCompiler")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
    pub fn compile_vm(
        &mut self,
        in_graphs: TArray<UPtr<URigVMGraph>>,
        in_controller: UPtr<URigVMController>,
        out_vm: UPtr<crate::bindings::rig_vm::URigVM>,
        context: &mut crate::bindings::rig_vm::FRigVMExtendedExecuteContext,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<801>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_COMPILER_COMPILE_VM,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_graphs,
                __buffer.add(0).cast::<TArray<UPtr<URigVMGraph>>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_controller,
                __buffer.add(16).cast::<UPtr<URigVMController>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &out_vm,
                __buffer.add(24).cast::<UPtr<crate::bindings::rig_vm::URigVM>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                context,
                __buffer
                    .add(32)
                    .cast::<crate::bindings::rig_vm::FRigVMExtendedExecuteContext>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_COMPILER_COMPILE_VM,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(32)
                .cast::<crate::bindings::rig_vm::FRigVMExtendedExecuteContext>()
                .swap(context);
        }
        unsafe { __buffer.add(800).cast::<bool>().read() }
    }
    pub fn compile(
        &mut self,
        in_graphs: TArray<UPtr<URigVMGraph>>,
        in_controller: UPtr<URigVMController>,
        out_vm: UPtr<crate::bindings::rig_vm::URigVM>,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<33>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_COMPILER_COMPILE,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_graphs,
                __buffer.add(0).cast::<TArray<UPtr<URigVMGraph>>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_controller,
                __buffer.add(16).cast::<UPtr<URigVMController>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &out_vm,
                __buffer.add(24).cast::<UPtr<crate::bindings::rig_vm::URigVM>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_COMPILER_COMPILE,
                __buffer,
            )
        };
        unsafe { __buffer.add(32).cast::<bool>().read() }
    }
}
#[repr(C, align(8))]
pub struct URigVMNode {
    __padding_end: [u8; 544],
}
impl URigVMNode {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("URigVMNode")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
    pub fn set_is_excluded_by_early_exit(&mut self, b_is_excluded: bool) {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_NODE_SET_IS_EXCLUDED_BY_EARLY_EXIT,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_is_excluded,
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
                crate::bindings::rig_vm_developer::U_RIG_VM_NODE_SET_IS_EXCLUDED_BY_EARLY_EXIT,
                __buffer,
            )
        };
    }
    pub fn set_has_early_exit_marker(&mut self, b_value: bool) {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_NODE_SET_HAS_EARLY_EXIT_MARKER,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&b_value, __buffer.add(0).cast::<bool>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_NODE_SET_HAS_EARLY_EXIT_MARKER,
                __buffer,
            )
        };
    }
    pub fn set_has_breakpoint(&mut self, b_value: bool) {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_NODE_SET_HAS_BREAKPOINT,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&b_value, __buffer.add(0).cast::<bool>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_NODE_SET_HAS_BREAKPOINT,
                __buffer,
            )
        };
    }
    pub fn set_execution_is_halted_at_this_node(&mut self, b_value: bool) {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_NODE_SET_EXECUTION_IS_HALTED_AT_THIS_NODE,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&b_value, __buffer.add(0).cast::<bool>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_NODE_SET_EXECUTION_IS_HALTED_AT_THIS_NODE,
                __buffer,
            )
        };
    }
    pub fn is_visible_in_ui(&self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_NODE_IS_VISIBLE_IN_UI,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_NODE_IS_VISIBLE_IN_UI,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn is_trait_pin(&self, in_name: FName) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<13>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_NODE_IS_TRAIT_PIN,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&in_name, __buffer.add(0).cast::<FName>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_NODE_IS_TRAIT_PIN,
                __buffer,
            )
        };
        unsafe { __buffer.add(12).cast::<bool>().read() }
    }
    pub fn is_selected(&self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_NODE_IS_SELECTED,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_NODE_IS_SELECTED,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn is_pure(&self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_NODE_IS_PURE,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_NODE_IS_PURE,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn is_pin_category_expanded(&self, in_category: FString) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<17>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_NODE_IS_PIN_CATEGORY_EXPANDED,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_category,
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
                crate::bindings::rig_vm_developer::U_RIG_VM_NODE_IS_PIN_CATEGORY_EXPANDED,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<bool>().read() }
    }
    pub fn is_mutable(&self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_NODE_IS_MUTABLE,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_NODE_IS_MUTABLE,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn is_loop_node(&self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_NODE_IS_LOOP_NODE,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_NODE_IS_LOOP_NODE,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn is_linked_to(&self, in_node: UPtr<URigVMNode>) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<9>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_NODE_IS_LINKED_TO,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_node,
                __buffer.add(0).cast::<UPtr<URigVMNode>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_NODE_IS_LINKED_TO,
                __buffer,
            )
        };
        unsafe { __buffer.add(8).cast::<bool>().read() }
    }
    pub fn is_input_aggregate(&self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_NODE_IS_INPUT_AGGREGATE,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_NODE_IS_INPUT_AGGREGATE,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn is_injected(&self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_NODE_IS_INJECTED,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_NODE_IS_INJECTED,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn is_highlighted(&self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_NODE_IS_HIGHLIGHTED,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_NODE_IS_HIGHLIGHTED,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn is_excluded_by_early_exit(&self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_NODE_IS_EXCLUDED_BY_EARLY_EXIT,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_NODE_IS_EXCLUDED_BY_EARLY_EXIT,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn is_event(&self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_NODE_IS_EVENT,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_NODE_IS_EVENT,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn is_defined_as_varying(&self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_NODE_IS_DEFINED_AS_VARYING,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_NODE_IS_DEFINED_AS_VARYING,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn is_defined_as_constant(&self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_NODE_IS_DEFINED_AS_CONSTANT,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_NODE_IS_DEFINED_AS_CONSTANT,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn is_control_flow_node(&self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_NODE_IS_CONTROL_FLOW_NODE,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_NODE_IS_CONTROL_FLOW_NODE,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn is_aggregate(&self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_NODE_IS_AGGREGATE,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_NODE_IS_AGGREGATE,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn has_pin_of_direction(
        &self,
        in_direction: crate::bindings::rig_vm::ERigVMPinDirection,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<2>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_NODE_HAS_PIN_OF_DIRECTION,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_direction,
                __buffer.add(0).cast::<crate::bindings::rig_vm::ERigVMPinDirection>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_NODE_HAS_PIN_OF_DIRECTION,
                __buffer,
            )
        };
        unsafe { __buffer.add(1).cast::<bool>().read() }
    }
    pub fn has_output_pin(&self, b_include_io: bool) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<2>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_NODE_HAS_OUTPUT_PIN,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_include_io,
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
                crate::bindings::rig_vm_developer::U_RIG_VM_NODE_HAS_OUTPUT_PIN,
                __buffer,
            )
        };
        unsafe { __buffer.add(1).cast::<bool>().read() }
    }
    pub fn has_orphaned_pins(&self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_NODE_HAS_ORPHANED_PINS,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_NODE_HAS_ORPHANED_PINS,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn has_lazy_pin(&self, b_only_consider_pins_with_links: bool) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<2>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_NODE_HAS_LAZY_PIN,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_only_consider_pins_with_links,
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
                crate::bindings::rig_vm_developer::U_RIG_VM_NODE_HAS_LAZY_PIN,
                __buffer,
            )
        };
        unsafe { __buffer.add(1).cast::<bool>().read() }
    }
    pub fn has_io_pin(&self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_NODE_HAS_IO_PIN,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_NODE_HAS_IO_PIN,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn has_input_pin(&self, b_include_io: bool) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<2>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_NODE_HAS_INPUT_PIN,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_include_io,
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
                crate::bindings::rig_vm_developer::U_RIG_VM_NODE_HAS_INPUT_PIN,
                __buffer,
            )
        };
        unsafe { __buffer.add(1).cast::<bool>().read() }
    }
    pub fn has_early_exit_marker(&self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_NODE_HAS_EARLY_EXIT_MARKER,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_NODE_HAS_EARLY_EXIT_MARKER,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn has_breakpoint(&self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_NODE_HAS_BREAKPOINT,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_NODE_HAS_BREAKPOINT,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn get_trait_pins(&self) -> TArray<UPtr<URigVMPin>> {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_NODE_GET_TRAIT_PINS,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_NODE_GET_TRAIT_PINS,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<TArray<UPtr<URigVMPin>>>().read() }
    }
    pub fn get_tool_tip_text(&self) -> FText {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_NODE_GET_TOOL_TIP_TEXT,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_NODE_GET_TOOL_TIP_TEXT,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<FText>().read() }
    }
    pub fn get_supported_workflows(
        &self,
        in_type: crate::bindings::rig_vm::ERigVMUserWorkflowType,
        in_subject: UPtr<crate::bindings::core_u_object::UObject>,
    ) -> TArray<crate::bindings::rig_vm::FRigVMUserWorkflow> {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_NODE_GET_SUPPORTED_WORKFLOWS,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_type,
                __buffer
                    .add(0)
                    .cast::<crate::bindings::rig_vm::ERigVMUserWorkflowType>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_subject,
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
                crate::bindings::rig_vm_developer::U_RIG_VM_NODE_GET_SUPPORTED_WORKFLOWS,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(16)
                .cast::<TArray<crate::bindings::rig_vm::FRigVMUserWorkflow>>()
                .read()
        }
    }
    pub fn get_sub_pin_categories(
        &self,
        in_category: FString,
        b_only_existing: bool,
        b_recursive: bool,
    ) -> TArray<FString> {
        let mut __stack = crate::core_data::StackAlloc::<40>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_NODE_GET_SUB_PIN_CATEGORIES,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_category,
                __buffer.add(0).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_only_existing,
                __buffer.add(16).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_recursive,
                __buffer.add(17).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_NODE_GET_SUB_PIN_CATEGORIES,
                __buffer,
            )
        };
        unsafe { __buffer.add(24).cast::<TArray<FString>>().read() }
    }
    pub fn get_size(&self) -> crate::bindings::core_u_object::FVector2D {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_NODE_GET_SIZE,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_NODE_GET_SIZE,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<crate::bindings::core_u_object::FVector2D>().read()
        }
    }
    pub fn get_second_aggregate_pin(&self) -> UPtr<URigVMPin> {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_NODE_GET_SECOND_AGGREGATE_PIN,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_NODE_GET_SECOND_AGGREGATE_PIN,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<UPtr<URigVMPin>>().read() }
    }
    pub fn get_root_graph(&self) -> UPtr<URigVMGraph> {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_NODE_GET_ROOT_GRAPH,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_NODE_GET_ROOT_GRAPH,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<UPtr<URigVMGraph>>().read() }
    }
    pub fn get_previous_f_name(&self) -> FName {
        let mut __stack = crate::core_data::StackAlloc::<12>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_NODE_GET_PREVIOUS_F_NAME,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_NODE_GET_PREVIOUS_F_NAME,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<FName>().read() }
    }
    pub fn get_position(&self) -> crate::bindings::core_u_object::FVector2D {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_NODE_GET_POSITION,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_NODE_GET_POSITION,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<crate::bindings::core_u_object::FVector2D>().read()
        }
    }
    pub fn get_pins_for_category(
        &self,
        in_category: FString,
    ) -> TArray<UPtr<URigVMPin>> {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_NODE_GET_PINS_FOR_CATEGORY,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_category,
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
                crate::bindings::rig_vm_developer::U_RIG_VM_NODE_GET_PINS_FOR_CATEGORY,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<TArray<UPtr<URigVMPin>>>().read() }
    }
    pub fn get_pins(&self) -> TArray<UPtr<URigVMPin>> {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_NODE_GET_PINS,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_NODE_GET_PINS,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<TArray<UPtr<URigVMPin>>>().read() }
    }
    pub fn get_pin_category_name(&self, in_category: FString) -> FString {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_NODE_GET_PIN_CATEGORY_NAME,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_category,
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
                crate::bindings::rig_vm_developer::U_RIG_VM_NODE_GET_PIN_CATEGORY_NAME,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<FString>().read() }
    }
    pub fn get_pin_categories(&self) -> TArray<FString> {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_NODE_GET_PIN_CATEGORIES,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_NODE_GET_PIN_CATEGORIES,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<TArray<FString>>().read() }
    }
    pub fn get_parent_pin_category(
        &self,
        in_category: FString,
        b_only_existing: bool,
    ) -> FString {
        let mut __stack = crate::core_data::StackAlloc::<40>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_NODE_GET_PARENT_PIN_CATEGORY,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_category,
                __buffer.add(0).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_only_existing,
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
                crate::bindings::rig_vm_developer::U_RIG_VM_NODE_GET_PARENT_PIN_CATEGORY,
                __buffer,
            )
        };
        unsafe { __buffer.add(24).cast::<FString>().read() }
    }
    pub fn get_parent_pin_categories(
        &self,
        in_category: FString,
        b_only_existing: bool,
        b_include_self: bool,
    ) -> TArray<FString> {
        let mut __stack = crate::core_data::StackAlloc::<40>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_NODE_GET_PARENT_PIN_CATEGORIES,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_category,
                __buffer.add(0).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_only_existing,
                __buffer.add(16).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_include_self,
                __buffer.add(17).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_NODE_GET_PARENT_PIN_CATEGORIES,
                __buffer,
            )
        };
        unsafe { __buffer.add(24).cast::<TArray<FString>>().read() }
    }
    pub fn get_orphaned_pins(&self) -> TArray<UPtr<URigVMPin>> {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_NODE_GET_ORPHANED_PINS,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_NODE_GET_ORPHANED_PINS,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<TArray<UPtr<URigVMPin>>>().read() }
    }
    pub fn get_opposite_aggregate_pin(&self) -> UPtr<URigVMPin> {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_NODE_GET_OPPOSITE_AGGREGATE_PIN,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_NODE_GET_OPPOSITE_AGGREGATE_PIN,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<UPtr<URigVMPin>>().read() }
    }
    pub fn get_node_title(&self) -> FString {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_NODE_GET_NODE_TITLE,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_NODE_GET_NODE_TITLE,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<FString>().read() }
    }
    pub fn get_node_sub_title(&self) -> FString {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_NODE_GET_NODE_SUB_TITLE,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_NODE_GET_NODE_SUB_TITLE,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<FString>().read() }
    }
    pub fn get_node_path(&self, b_recursive: bool) -> FString {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_NODE_GET_NODE_PATH,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_recursive,
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
                crate::bindings::rig_vm_developer::U_RIG_VM_NODE_GET_NODE_PATH,
                __buffer,
            )
        };
        unsafe { __buffer.add(8).cast::<FString>().read() }
    }
    pub fn get_node_layout(
        &self,
        b_include_empty_categories: bool,
    ) -> crate::bindings::rig_vm::FRigVMNodeLayout {
        let mut __stack = crate::core_data::StackAlloc::<184>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_NODE_GET_NODE_LAYOUT,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_include_empty_categories,
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
                crate::bindings::rig_vm_developer::U_RIG_VM_NODE_GET_NODE_LAYOUT,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(8).cast::<crate::bindings::rig_vm::FRigVMNodeLayout>().read()
        }
    }
    pub fn get_node_index(&self) -> i32 {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_NODE_GET_NODE_INDEX,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_NODE_GET_NODE_INDEX,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<i32>().read() }
    }
    pub fn get_node_color(&self) -> crate::bindings::core_u_object::FLinearColor {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_NODE_GET_NODE_COLOR,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_NODE_GET_NODE_COLOR,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<crate::bindings::core_u_object::FLinearColor>().read()
        }
    }
    pub fn get_next_aggregate_name(&self, in_last_aggregate_pin_name: &FName) -> FName {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_NODE_GET_NEXT_AGGREGATE_NAME,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_last_aggregate_pin_name,
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
                crate::bindings::rig_vm_developer::U_RIG_VM_NODE_GET_NEXT_AGGREGATE_NAME,
                __buffer,
            )
        };
        unsafe { __buffer.add(12).cast::<FName>().read() }
    }
    pub fn get_links(&self) -> TArray<UPtr<URigVMLink>> {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_NODE_GET_LINKS,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_NODE_GET_LINKS,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<TArray<UPtr<URigVMLink>>>().read() }
    }
    pub fn get_linked_target_nodes(&self) -> TArray<UPtr<URigVMNode>> {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_NODE_GET_LINKED_TARGET_NODES,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_NODE_GET_LINKED_TARGET_NODES,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<TArray<UPtr<URigVMNode>>>().read() }
    }
    pub fn get_linked_source_nodes(&self) -> TArray<UPtr<URigVMNode>> {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_NODE_GET_LINKED_SOURCE_NODES,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_NODE_GET_LINKED_SOURCE_NODES,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<TArray<UPtr<URigVMNode>>>().read() }
    }
    pub fn get_injection_info(&self) -> UPtr<URigVMInjectionInfo> {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_NODE_GET_INJECTION_INFO,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_NODE_GET_INJECTION_INFO,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<UPtr<URigVMInjectionInfo>>().read() }
    }
    pub fn get_graph_depth(&self) -> i32 {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_NODE_GET_GRAPH_DEPTH,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_NODE_GET_GRAPH_DEPTH,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<i32>().read() }
    }
    pub fn get_graph(&self) -> UPtr<URigVMGraph> {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_NODE_GET_GRAPH,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_NODE_GET_GRAPH,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<UPtr<URigVMGraph>>().read() }
    }
    pub fn get_first_aggregate_pin(&self) -> UPtr<URigVMPin> {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_NODE_GET_FIRST_AGGREGATE_PIN,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_NODE_GET_FIRST_AGGREGATE_PIN,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<UPtr<URigVMPin>>().read() }
    }
    pub fn get_event_name(&self) -> FName {
        let mut __stack = crate::core_data::StackAlloc::<12>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_NODE_GET_EVENT_NAME,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_NODE_GET_EVENT_NAME,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<FName>().read() }
    }
    pub fn get_all_pins_recursively(&self) -> TArray<UPtr<URigVMPin>> {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_NODE_GET_ALL_PINS_RECURSIVELY,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_NODE_GET_ALL_PINS_RECURSIVELY,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<TArray<UPtr<URigVMPin>>>().read() }
    }
    pub fn get_aggregate_outputs(&self) -> TArray<UPtr<URigVMPin>> {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_NODE_GET_AGGREGATE_OUTPUTS,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_NODE_GET_AGGREGATE_OUTPUTS,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<TArray<UPtr<URigVMPin>>>().read() }
    }
    pub fn get_aggregate_inputs(&self) -> TArray<UPtr<URigVMPin>> {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_NODE_GET_AGGREGATE_INPUTS,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_NODE_GET_AGGREGATE_INPUTS,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<TArray<UPtr<URigVMPin>>>().read() }
    }
    pub fn find_root_pin_by_name(&self, in_pin_name: &FName) -> UPtr<URigVMPin> {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_NODE_FIND_ROOT_PIN_BY_NAME,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_pin_name,
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
                crate::bindings::rig_vm_developer::U_RIG_VM_NODE_FIND_ROOT_PIN_BY_NAME,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<UPtr<URigVMPin>>().read() }
    }
    pub fn find_pin(&self, in_pin_path: FString) -> UPtr<URigVMPin> {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_NODE_FIND_PIN,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_pin_path,
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
                crate::bindings::rig_vm_developer::U_RIG_VM_NODE_FIND_PIN,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<UPtr<URigVMPin>>().read() }
    }
    pub fn find_function_for_node(&self) -> UPtr<URigVMLibraryNode> {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_NODE_FIND_FUNCTION_FOR_NODE,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_NODE_FIND_FUNCTION_FOR_NODE,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<UPtr<URigVMLibraryNode>>().read() }
    }
    pub fn find_execute_pin(&self) -> UPtr<URigVMPin> {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_NODE_FIND_EXECUTE_PIN,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_NODE_FIND_EXECUTE_PIN,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<UPtr<URigVMPin>>().read() }
    }
    pub fn execution_is_halted_at_this_node(&self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_NODE_EXECUTION_IS_HALTED_AT_THIS_NODE,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_NODE_EXECUTION_IS_HALTED_AT_THIS_NODE,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn can_only_exist_once(&self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_NODE_CAN_ONLY_EXIST_ONCE,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_NODE_CAN_ONLY_EXIST_ONCE,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn can_be_upgraded(&self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_NODE_CAN_BE_UPGRADED,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_NODE_CAN_BE_UPGRADED,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
}
#[repr(C, align(8))]
pub struct URigVMTemplateNode {
    __padding_end: [u8; 632],
}
impl URigVMTemplateNode {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("URigVMTemplateNode")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
    pub fn is_singleton(&self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_TEMPLATE_NODE_IS_SINGLETON,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_TEMPLATE_NODE_IS_SINGLETON,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn is_resolved(&self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_TEMPLATE_NODE_IS_RESOLVED,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_TEMPLATE_NODE_IS_RESOLVED,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn is_fully_unresolved(&self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_TEMPLATE_NODE_IS_FULLY_UNRESOLVED,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_TEMPLATE_NODE_IS_FULLY_UNRESOLVED,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn get_script_struct(
        &self,
    ) -> UPtr<crate::bindings::core_u_object::UScriptStruct> {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_TEMPLATE_NODE_GET_SCRIPT_STRUCT,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_TEMPLATE_NODE_GET_SCRIPT_STRUCT,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(0)
                .cast::<UPtr<crate::bindings::core_u_object::UScriptStruct>>()
                .read()
        }
    }
    pub fn get_notation(&self) -> FName {
        let mut __stack = crate::core_data::StackAlloc::<12>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_TEMPLATE_NODE_GET_NOTATION,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_TEMPLATE_NODE_GET_NOTATION,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<FName>().read() }
    }
}
#[repr(C, align(8))]
pub struct URigVMLibraryNode {
    __padding_end: [u8; 632],
}
impl URigVMLibraryNode {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("URigVMLibraryNode")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
    pub fn get_matching_variants(
        &self,
    ) -> TArray<crate::bindings::rig_vm::FRigVMVariantRef> {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_LIBRARY_NODE_GET_MATCHING_VARIANTS,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_LIBRARY_NODE_GET_MATCHING_VARIANTS,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(0)
                .cast::<TArray<crate::bindings::rig_vm::FRigVMVariantRef>>()
                .read()
        }
    }
    pub fn get_library(&self) -> UPtr<URigVMFunctionLibrary> {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_LIBRARY_NODE_GET_LIBRARY,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_LIBRARY_NODE_GET_LIBRARY,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<UPtr<URigVMFunctionLibrary>>().read() }
    }
    pub fn get_function_variant_ref(&self) -> crate::bindings::rig_vm::FRigVMVariantRef {
        let mut __stack = crate::core_data::StackAlloc::<72>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_LIBRARY_NODE_GET_FUNCTION_VARIANT_REF,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_LIBRARY_NODE_GET_FUNCTION_VARIANT_REF,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<crate::bindings::rig_vm::FRigVMVariantRef>().read()
        }
    }
    pub fn get_function_variant(&self) -> crate::bindings::rig_vm::FRigVMVariant {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_LIBRARY_NODE_GET_FUNCTION_VARIANT,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_LIBRARY_NODE_GET_FUNCTION_VARIANT,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<crate::bindings::rig_vm::FRigVMVariant>().read()
        }
    }
    pub fn get_contained_graph(&self) -> UPtr<URigVMGraph> {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_LIBRARY_NODE_GET_CONTAINED_GRAPH,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_LIBRARY_NODE_GET_CONTAINED_GRAPH,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<UPtr<URigVMGraph>>().read() }
    }
}
#[repr(C, align(8))]
pub struct URigVMCollapseNode {
    __padding_end: [u8; 688],
}
impl URigVMCollapseNode {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("URigVMCollapseNode")
            .unwrap()
    }
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
pub struct URigVMAggregateNode {
    __padding_end: [u8; 704],
}
impl URigVMAggregateNode {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("URigVMAggregateNode")
            .unwrap()
    }
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
pub struct UDEPRECATED_RigVMArrayNode {
    __padding_end: [u8; 640],
}
impl UDEPRECATED_RigVMArrayNode {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDEPRECATED_RigVMArrayNode")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
    pub fn get_op_code(&self) -> crate::bindings::rig_vm::ERigVMOpCode {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::UDEPRECATED_RIG_VM_ARRAY_NODE_GET_OP_CODE,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::UDEPRECATED_RIG_VM_ARRAY_NODE_GET_OP_CODE,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<crate::bindings::rig_vm::ERigVMOpCode>().read() }
    }
    pub fn get_cpp_type_object(&self) -> UPtr<crate::bindings::core_u_object::UObject> {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::UDEPRECATED_RIG_VM_ARRAY_NODE_GET_CPP_TYPE_OBJECT,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::UDEPRECATED_RIG_VM_ARRAY_NODE_GET_CPP_TYPE_OBJECT,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(0)
                .cast::<UPtr<crate::bindings::core_u_object::UObject>>()
                .read()
        }
    }
    pub fn get_cpp_type(&self) -> FString {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::UDEPRECATED_RIG_VM_ARRAY_NODE_GET_CPP_TYPE,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::UDEPRECATED_RIG_VM_ARRAY_NODE_GET_CPP_TYPE,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<FString>().read() }
    }
}
#[repr(C, align(8))]
pub struct UDEPRECATED_RigVMBranchNode {
    __padding_end: [u8; 544],
}
impl UDEPRECATED_RigVMBranchNode {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDEPRECATED_RigVMBranchNode")
            .unwrap()
    }
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
pub struct URigVMCommentNode {
    __padding_end: [u8; 568],
}
impl URigVMCommentNode {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("URigVMCommentNode")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
    pub fn get_comment_text(&self) -> FString {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_COMMENT_NODE_GET_COMMENT_TEXT,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_COMMENT_NODE_GET_COMMENT_TEXT,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<FString>().read() }
    }
    pub fn get_comment_font_size(&self) -> i32 {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_COMMENT_NODE_GET_COMMENT_FONT_SIZE,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_COMMENT_NODE_GET_COMMENT_FONT_SIZE,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<i32>().read() }
    }
    pub fn get_comment_color_bubble(&self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_COMMENT_NODE_GET_COMMENT_COLOR_BUBBLE,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_COMMENT_NODE_GET_COMMENT_COLOR_BUBBLE,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn get_comment_bubble_visible(&self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_COMMENT_NODE_GET_COMMENT_BUBBLE_VISIBLE,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_COMMENT_NODE_GET_COMMENT_BUBBLE_VISIBLE,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
}
#[repr(C, align(8))]
pub struct URigVMDispatchNode {
    __padding_end: [u8; 720],
}
impl URigVMDispatchNode {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("URigVMDispatchNode")
            .unwrap()
    }
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
pub struct URigVMEnumNode {
    __padding_end: [u8; 544],
}
impl URigVMEnumNode {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("URigVMEnumNode")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
    pub fn get_enum(&self) -> UPtr<crate::bindings::core_u_object::UEnum> {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_ENUM_NODE_GET_ENUM,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_ENUM_NODE_GET_ENUM,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<UPtr<crate::bindings::core_u_object::UEnum>>().read()
        }
    }
    pub fn get_cpp_type_object(&self) -> UPtr<crate::bindings::core_u_object::UObject> {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_ENUM_NODE_GET_CPP_TYPE_OBJECT,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_ENUM_NODE_GET_CPP_TYPE_OBJECT,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(0)
                .cast::<UPtr<crate::bindings::core_u_object::UObject>>()
                .read()
        }
    }
    pub fn get_cpp_type(&self) -> FString {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_ENUM_NODE_GET_CPP_TYPE,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_ENUM_NODE_GET_CPP_TYPE,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<FString>().read() }
    }
}
#[repr(C, align(8))]
pub struct URigVMFunctionInterfaceNode {
    __padding_end: [u8; 632],
}
impl URigVMFunctionInterfaceNode {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("URigVMFunctionInterfaceNode")
            .unwrap()
    }
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
pub struct URigVMFunctionEntryNode {
    __padding_end: [u8; 632],
}
impl URigVMFunctionEntryNode {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("URigVMFunctionEntryNode")
            .unwrap()
    }
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
pub struct URigVMFunctionReferenceNode {
    __padding_end: [u8; 1288],
}
impl URigVMFunctionReferenceNode {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("URigVMFunctionReferenceNode")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
    pub fn get_referenced_function_header(
        &self,
    ) -> crate::bindings::rig_vm::FRigVMGraphFunctionHeader {
        let mut __stack = crate::core_data::StackAlloc::<528>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_FUNCTION_REFERENCE_NODE_GET_REFERENCED_FUNCTION_HEADER_FOR_BLUEPRINT,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_FUNCTION_REFERENCE_NODE_GET_REFERENCED_FUNCTION_HEADER_FOR_BLUEPRINT,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(0)
                .cast::<crate::bindings::rig_vm::FRigVMGraphFunctionHeader>()
                .read()
        }
    }
}
#[repr(C, align(8))]
pub struct URigVMFunctionReturnNode {
    __padding_end: [u8; 632],
}
impl URigVMFunctionReturnNode {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("URigVMFunctionReturnNode")
            .unwrap()
    }
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
pub struct UDEPRECATED_RigVMIfNode {
    __padding_end: [u8; 632],
}
impl UDEPRECATED_RigVMIfNode {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDEPRECATED_RigVMIfNode")
            .unwrap()
    }
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
pub struct URigVMInvokeEntryNode {
    __padding_end: [u8; 544],
}
impl URigVMInvokeEntryNode {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("URigVMInvokeEntryNode")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
    pub fn get_entry_name(&self) -> FName {
        let mut __stack = crate::core_data::StackAlloc::<12>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_INVOKE_ENTRY_NODE_GET_ENTRY_NAME,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_INVOKE_ENTRY_NODE_GET_ENTRY_NAME,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<FName>().read() }
    }
}
#[repr(C, align(8))]
pub struct URigVMParameterNode {
    __padding_end: [u8; 544],
}
impl URigVMParameterNode {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("URigVMParameterNode")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
    pub fn is_input(&self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_PARAMETER_NODE_IS_INPUT,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_PARAMETER_NODE_IS_INPUT,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn get_parameter_name(&self) -> FName {
        let mut __stack = crate::core_data::StackAlloc::<12>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_PARAMETER_NODE_GET_PARAMETER_NAME,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_PARAMETER_NODE_GET_PARAMETER_NAME,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<FName>().read() }
    }
    pub fn get_parameter_description(&self) -> FRigVMGraphParameterDescription {
        let mut __stack = crate::core_data::StackAlloc::<56>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_PARAMETER_NODE_GET_PARAMETER_DESCRIPTION,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_PARAMETER_NODE_GET_PARAMETER_DESCRIPTION,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<FRigVMGraphParameterDescription>().read() }
    }
    pub fn get_default_value(&self) -> FString {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_PARAMETER_NODE_GET_DEFAULT_VALUE,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_PARAMETER_NODE_GET_DEFAULT_VALUE,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<FString>().read() }
    }
    pub fn get_cpp_type_object(&self) -> UPtr<crate::bindings::core_u_object::UObject> {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_PARAMETER_NODE_GET_CPP_TYPE_OBJECT,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_PARAMETER_NODE_GET_CPP_TYPE_OBJECT,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(0)
                .cast::<UPtr<crate::bindings::core_u_object::UObject>>()
                .read()
        }
    }
    pub fn get_cpp_type(&self) -> FString {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_PARAMETER_NODE_GET_CPP_TYPE,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_PARAMETER_NODE_GET_CPP_TYPE,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<FString>().read() }
    }
}
#[repr(C, align(8))]
pub struct URigVMRerouteNode {
    __padding_end: [u8; 544],
}
impl URigVMRerouteNode {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("URigVMRerouteNode")
            .unwrap()
    }
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
pub struct UDEPRECATED_RigVMSelectNode {
    __padding_end: [u8; 632],
}
impl UDEPRECATED_RigVMSelectNode {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDEPRECATED_RigVMSelectNode")
            .unwrap()
    }
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
pub struct URigVMUnitNode {
    __padding_end: [u8; 680],
}
impl URigVMUnitNode {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("URigVMUnitNode")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
    pub fn get_struct_default_value(&self) -> FString {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_UNIT_NODE_GET_STRUCT_DEFAULT_VALUE,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_UNIT_NODE_GET_STRUCT_DEFAULT_VALUE,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<FString>().read() }
    }
    pub fn get_method_name(&self) -> FName {
        let mut __stack = crate::core_data::StackAlloc::<12>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_UNIT_NODE_GET_METHOD_NAME,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_UNIT_NODE_GET_METHOD_NAME,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<FName>().read() }
    }
}
#[repr(C, align(8))]
pub struct URigVMVariableNode {
    __padding_end: [u8; 544],
}
impl URigVMVariableNode {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("URigVMVariableNode")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
    pub fn is_local_variable(&self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_VARIABLE_NODE_IS_LOCAL_VARIABLE,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_VARIABLE_NODE_IS_LOCAL_VARIABLE,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn is_input_argument(&self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_VARIABLE_NODE_IS_INPUT_ARGUMENT,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_VARIABLE_NODE_IS_INPUT_ARGUMENT,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn is_getter(&self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_VARIABLE_NODE_IS_GETTER,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_VARIABLE_NODE_IS_GETTER,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn is_external_variable(&self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_VARIABLE_NODE_IS_EXTERNAL_VARIABLE,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_VARIABLE_NODE_IS_EXTERNAL_VARIABLE,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn get_variable_name(&self) -> FName {
        let mut __stack = crate::core_data::StackAlloc::<12>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_VARIABLE_NODE_GET_VARIABLE_NAME,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_VARIABLE_NODE_GET_VARIABLE_NAME,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<FName>().read() }
    }
    pub fn get_variable_description(&self) -> FRigVMGraphVariableDescription {
        let mut __stack = crate::core_data::StackAlloc::<112>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_VARIABLE_NODE_GET_VARIABLE_DESCRIPTION,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_VARIABLE_NODE_GET_VARIABLE_DESCRIPTION,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<FRigVMGraphVariableDescription>().read() }
    }
    pub fn get_default_value(&self) -> FString {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_VARIABLE_NODE_GET_DEFAULT_VALUE,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_VARIABLE_NODE_GET_DEFAULT_VALUE,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<FString>().read() }
    }
    pub fn get_cpp_type_object(&self) -> UPtr<crate::bindings::core_u_object::UObject> {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_VARIABLE_NODE_GET_CPP_TYPE_OBJECT,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_VARIABLE_NODE_GET_CPP_TYPE_OBJECT,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(0)
                .cast::<UPtr<crate::bindings::core_u_object::UObject>>()
                .read()
        }
    }
    pub fn get_cpp_type(&self) -> FString {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_VARIABLE_NODE_GET_CPP_TYPE,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_VARIABLE_NODE_GET_CPP_TYPE,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<FString>().read() }
    }
}
#[repr(C, align(8))]
pub struct URigVMBuildData {
    __padding_end: [u8; 216],
}
impl URigVMBuildData {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("URigVMBuildData")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
    pub fn split_variant_from_set(
        &mut self,
        in_variant_ref: &crate::bindings::rig_vm::FRigVMVariantRef,
    ) -> crate::bindings::rig_vm::FRigVMVariantRef {
        let mut __stack = crate::core_data::StackAlloc::<144>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_BUILD_DATA_SPLIT_VARIANT_FROM_SET,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_variant_ref,
                __buffer.add(0).cast::<crate::bindings::rig_vm::FRigVMVariantRef>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_BUILD_DATA_SPLIT_VARIANT_FROM_SET,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(72).cast::<crate::bindings::rig_vm::FRigVMVariantRef>().read()
        }
    }
    pub fn join_variant_set(
        &mut self,
        in_variant_ref: &crate::bindings::rig_vm::FRigVMVariantRef,
        in_guid: &crate::bindings::core_u_object::FGuid,
    ) -> crate::bindings::rig_vm::FRigVMVariantRef {
        let mut __stack = crate::core_data::StackAlloc::<160>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_BUILD_DATA_JOIN_VARIANT_SET,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_variant_ref,
                __buffer.add(0).cast::<crate::bindings::rig_vm::FRigVMVariantRef>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_guid,
                __buffer.add(72).cast::<crate::bindings::core_u_object::FGuid>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_BUILD_DATA_JOIN_VARIANT_SET,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(88).cast::<crate::bindings::rig_vm::FRigVMVariantRef>().read()
        }
    }
    pub fn get_variant_ref_for_asset(
        &self,
        in_asset_data: &crate::bindings::core_u_object::FAssetData,
    ) -> crate::bindings::rig_vm::FRigVMVariantRef {
        let mut __stack = crate::core_data::StackAlloc::<224>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_BUILD_DATA_GET_VARIANT_REF_FOR_ASSET,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_asset_data,
                __buffer.add(0).cast::<crate::bindings::core_u_object::FAssetData>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_BUILD_DATA_GET_VARIANT_REF_FOR_ASSET,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(152).cast::<crate::bindings::rig_vm::FRigVMVariantRef>().read()
        }
    }
    pub fn get_used_function_identifiers(
        &self,
        b_only_public: bool,
    ) -> TArray<crate::bindings::rig_vm::FRigVMGraphFunctionIdentifier> {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_BUILD_DATA_GET_USED_FUNCTION_IDENTIFIERS,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_only_public,
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
                crate::bindings::rig_vm_developer::U_RIG_VM_BUILD_DATA_GET_USED_FUNCTION_IDENTIFIERS,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(8)
                .cast::<TArray<crate::bindings::rig_vm::FRigVMGraphFunctionIdentifier>>()
                .read()
        }
    }
    pub fn get_function_identifier_for_variant(
        &self,
        in_variant_ref: &crate::bindings::rig_vm::FRigVMVariantRef,
    ) -> crate::bindings::rig_vm::FRigVMGraphFunctionIdentifier {
        let mut __stack = crate::core_data::StackAlloc::<168>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_BUILD_DATA_GET_FUNCTION_IDENTIFIER_FOR_VARIANT,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_variant_ref,
                __buffer.add(0).cast::<crate::bindings::rig_vm::FRigVMVariantRef>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_BUILD_DATA_GET_FUNCTION_IDENTIFIER_FOR_VARIANT,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(72)
                .cast::<crate::bindings::rig_vm::FRigVMGraphFunctionIdentifier>()
                .read()
        }
    }
    pub fn get_asset_data_for_variant(
        &self,
        in_variant_ref: &crate::bindings::rig_vm::FRigVMVariantRef,
    ) -> crate::bindings::core_u_object::FAssetData {
        let mut __stack = crate::core_data::StackAlloc::<224>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_BUILD_DATA_GET_ASSET_DATA_FOR_VARIANT,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_variant_ref,
                __buffer.add(0).cast::<crate::bindings::rig_vm::FRigVMVariantRef>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_BUILD_DATA_GET_ASSET_DATA_FOR_VARIANT,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(72).cast::<crate::bindings::core_u_object::FAssetData>().read()
        }
    }
    pub fn get_asset_data_for_path(
        &self,
        in_object_path: &crate::bindings::core_u_object::FSoftObjectPath,
    ) -> crate::bindings::core_u_object::FAssetData {
        let mut __stack = crate::core_data::StackAlloc::<192>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_BUILD_DATA_GET_ASSET_DATA_FOR_PATH,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_object_path,
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
                crate::bindings::rig_vm_developer::U_RIG_VM_BUILD_DATA_GET_ASSET_DATA_FOR_PATH,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(40).cast::<crate::bindings::core_u_object::FAssetData>().read()
        }
    }
    pub fn get_all_function_identifiers(
        &self,
        b_only_public: bool,
    ) -> TArray<crate::bindings::rig_vm::FRigVMGraphFunctionIdentifier> {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_BUILD_DATA_GET_ALL_FUNCTION_IDENTIFIERS,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_only_public,
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
                crate::bindings::rig_vm_developer::U_RIG_VM_BUILD_DATA_GET_ALL_FUNCTION_IDENTIFIERS,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(8)
                .cast::<TArray<crate::bindings::rig_vm::FRigVMGraphFunctionIdentifier>>()
                .read()
        }
    }
    pub fn get() -> UPtr<URigVMBuildData> {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_BUILD_DATA_GET,
                __buffer,
            )
        };
        let __object_ptr = crate::bindings::rig_vm_developer::URigVMBuildData::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_BUILD_DATA_GET,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<UPtr<URigVMBuildData>>().read() }
    }
    pub fn gather_function_variant_refs_for_asset(
        &self,
        in_asset_data: &crate::bindings::core_u_object::FAssetData,
    ) -> TArray<crate::bindings::rig_vm::FRigVMVariantRef> {
        let mut __stack = crate::core_data::StackAlloc::<168>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_BUILD_DATA_GATHER_FUNCTION_VARIANT_REFS_FOR_ASSET,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_asset_data,
                __buffer.add(0).cast::<crate::bindings::core_u_object::FAssetData>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_BUILD_DATA_GATHER_FUNCTION_VARIANT_REFS_FOR_ASSET,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(152)
                .cast::<TArray<crate::bindings::rig_vm::FRigVMVariantRef>>()
                .read()
        }
    }
    pub fn gather_all_function_variant_refs(
        &self,
    ) -> TArray<crate::bindings::rig_vm::FRigVMVariantRef> {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_BUILD_DATA_GATHER_ALL_FUNCTION_VARIANT_REFS,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_BUILD_DATA_GATHER_ALL_FUNCTION_VARIANT_REFS,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(0)
                .cast::<TArray<crate::bindings::rig_vm::FRigVMVariantRef>>()
                .read()
        }
    }
    pub fn gather_all_asset_variant_refs(
        &self,
    ) -> TArray<crate::bindings::rig_vm::FRigVMVariantRef> {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_BUILD_DATA_GATHER_ALL_ASSET_VARIANT_REFS,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_BUILD_DATA_GATHER_ALL_ASSET_VARIANT_REFS,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(0)
                .cast::<TArray<crate::bindings::rig_vm::FRigVMVariantRef>>()
                .read()
        }
    }
    pub fn find_function_variant_refs(
        &self,
        in_guid: &crate::bindings::core_u_object::FGuid,
    ) -> TArray<crate::bindings::rig_vm::FRigVMVariantRef> {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_BUILD_DATA_FIND_FUNCTION_VARIANT_REFS,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_guid,
                __buffer.add(0).cast::<crate::bindings::core_u_object::FGuid>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_BUILD_DATA_FIND_FUNCTION_VARIANT_REFS,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(16)
                .cast::<TArray<crate::bindings::rig_vm::FRigVMVariantRef>>()
                .read()
        }
    }
    pub fn find_asset_variant_refs(
        &self,
        in_guid: &crate::bindings::core_u_object::FGuid,
    ) -> TArray<crate::bindings::rig_vm::FRigVMVariantRef> {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_BUILD_DATA_FIND_ASSET_VARIANT_REFS,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_guid,
                __buffer.add(0).cast::<crate::bindings::core_u_object::FGuid>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_BUILD_DATA_FIND_ASSET_VARIANT_REFS,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(16)
                .cast::<TArray<crate::bindings::rig_vm::FRigVMVariantRef>>()
                .read()
        }
    }
    pub fn create_function_variant(
        &mut self,
        in_function_identifier: &crate::bindings::rig_vm::FRigVMGraphFunctionIdentifier,
        in_name: FName,
    ) -> crate::bindings::rig_vm::FRigVMVariantRef {
        let mut __stack = crate::core_data::StackAlloc::<184>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_BUILD_DATA_CREATE_FUNCTION_VARIANT,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_function_identifier,
                __buffer
                    .add(0)
                    .cast::<crate::bindings::rig_vm::FRigVMGraphFunctionIdentifier>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&in_name, __buffer.add(96).cast::<FName>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_BUILD_DATA_CREATE_FUNCTION_VARIANT,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(112).cast::<crate::bindings::rig_vm::FRigVMVariantRef>().read()
        }
    }
    pub fn create_asset_variant(
        &mut self,
        in_asset_data: &crate::bindings::core_u_object::FAssetData,
        in_name: FName,
    ) -> crate::bindings::rig_vm::FRigVMVariantRef {
        let mut __stack = crate::core_data::StackAlloc::<240>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_BUILD_DATA_CREATE_ASSET_VARIANT,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_asset_data,
                __buffer.add(0).cast::<crate::bindings::core_u_object::FAssetData>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_name,
                __buffer.add(152).cast::<FName>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_BUILD_DATA_CREATE_ASSET_VARIANT,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(168).cast::<crate::bindings::rig_vm::FRigVMVariantRef>().read()
        }
    }
}
pub struct IRigVMClientHost {}
#[repr(C, align(8))]
pub struct URigVMClientHost {
    __padding_end: [u8; 48],
}
impl URigVMClientHost {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("URigVMClientHost")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
pub struct IRigVMEditorSideObject {}
#[repr(C, align(8))]
pub struct URigVMEditorSideObject {
    __padding_end: [u8; 48],
}
impl URigVMEditorSideObject {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("URigVMEditorSideObject")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
pub struct IRigVMClientExternalModelHost {}
#[repr(C, align(8))]
pub struct URigVMClientExternalModelHost {
    __padding_end: [u8; 48],
}
impl URigVMClientExternalModelHost {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("URigVMClientExternalModelHost")
            .unwrap()
    }
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
pub struct URigVMActionStack {
    __padding_end: [u8; 152],
}
impl URigVMActionStack {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("URigVMActionStack")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
pub struct IRigVMExternalDependencyManager {}
#[repr(C, align(8))]
pub struct URigVMExternalDependencyManager {
    __padding_end: [u8; 48],
}
impl URigVMExternalDependencyManager {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("URigVMExternalDependencyManager")
            .unwrap()
    }
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
pub struct URigVMGraph {
    __padding_end: [u8; 296],
}
impl URigVMGraph {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("URigVMGraph")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
    pub fn set_schema_class(&mut self, in_schema_class: TSubclassOf<URigVMSchema>) {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_GRAPH_SET_SCHEMA_CLASS,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_schema_class,
                __buffer.add(0).cast::<TSubclassOf<URigVMSchema>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_GRAPH_SET_SCHEMA_CLASS,
                __buffer,
            )
        };
    }
    pub fn set_default_function_library(
        &mut self,
        in_function_library: UPtr<URigVMFunctionLibrary>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_GRAPH_SET_DEFAULT_FUNCTION_LIBRARY,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_function_library,
                __buffer.add(0).cast::<UPtr<URigVMFunctionLibrary>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_GRAPH_SET_DEFAULT_FUNCTION_LIBRARY,
                __buffer,
            )
        };
    }
    pub fn is_top_level_graph(&self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_GRAPH_IS_TOP_LEVEL_GRAPH,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_GRAPH_IS_TOP_LEVEL_GRAPH,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn is_root_graph(&self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_GRAPH_IS_ROOT_GRAPH,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_GRAPH_IS_ROOT_GRAPH,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn is_node_selected(&self, in_node_name: &FName) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<13>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_GRAPH_IS_NODE_SELECTED,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_node_name,
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
                crate::bindings::rig_vm_developer::U_RIG_VM_GRAPH_IS_NODE_SELECTED,
                __buffer,
            )
        };
        unsafe { __buffer.add(12).cast::<bool>().read() }
    }
    pub fn is_node_highlighted(&self, in_node_name: &FName) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<13>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_GRAPH_IS_NODE_HIGHLIGHTED,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_node_name,
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
                crate::bindings::rig_vm_developer::U_RIG_VM_GRAPH_IS_NODE_HIGHLIGHTED,
                __buffer,
            )
        };
        unsafe { __buffer.add(12).cast::<bool>().read() }
    }
    pub fn get_variable_descriptions(&self) -> TArray<FRigVMGraphVariableDescription> {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_GRAPH_GET_VARIABLE_DESCRIPTIONS,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_GRAPH_GET_VARIABLE_DESCRIPTIONS,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<TArray<FRigVMGraphVariableDescription>>().read()
        }
    }
    pub fn get_select_nodes(&self) -> TArray<FName> {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_GRAPH_GET_SELECT_NODES,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_GRAPH_GET_SELECT_NODES,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<TArray<FName>>().read() }
    }
    pub fn get_schema_class(&self) -> TSubclassOf<URigVMSchema> {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_GRAPH_GET_SCHEMA_CLASS,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_GRAPH_GET_SCHEMA_CLASS,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<TSubclassOf<URigVMSchema>>().read() }
    }
    pub fn get_schema(&self) -> UPtr<URigVMSchema> {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_GRAPH_GET_SCHEMA,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_GRAPH_GET_SCHEMA,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<UPtr<URigVMSchema>>().read() }
    }
    pub fn get_root_graph(&self) -> UPtr<URigVMGraph> {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_GRAPH_GET_ROOT_GRAPH,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_GRAPH_GET_ROOT_GRAPH,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<UPtr<URigVMGraph>>().read() }
    }
    pub fn get_return_node(&self) -> UPtr<URigVMFunctionReturnNode> {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_GRAPH_GET_RETURN_NODE,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_GRAPH_GET_RETURN_NODE,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<UPtr<URigVMFunctionReturnNode>>().read() }
    }
    pub fn get_parent_graph(&self) -> UPtr<URigVMGraph> {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_GRAPH_GET_PARENT_GRAPH,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_GRAPH_GET_PARENT_GRAPH,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<UPtr<URigVMGraph>>().read() }
    }
    pub fn get_output_arguments(&self) -> TArray<FRigVMGraphVariableDescription> {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_GRAPH_GET_OUTPUT_ARGUMENTS,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_GRAPH_GET_OUTPUT_ARGUMENTS,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<TArray<FRigVMGraphVariableDescription>>().read()
        }
    }
    pub fn get_nodes(&self) -> TArray<UPtr<URigVMNode>> {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_GRAPH_GET_NODES,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_GRAPH_GET_NODES,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<TArray<UPtr<URigVMNode>>>().read() }
    }
    pub fn get_node_path(&self) -> FString {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_GRAPH_GET_NODE_PATH,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_GRAPH_GET_NODE_PATH,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<FString>().read() }
    }
    pub fn get_local_variables(
        &self,
        b_include_input_arguments: bool,
    ) -> TArray<FRigVMGraphVariableDescription> {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_GRAPH_GET_LOCAL_VARIABLES,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_include_input_arguments,
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
                crate::bindings::rig_vm_developer::U_RIG_VM_GRAPH_GET_LOCAL_VARIABLES,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(8).cast::<TArray<FRigVMGraphVariableDescription>>().read()
        }
    }
    pub fn get_links(&self) -> TArray<UPtr<URigVMLink>> {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_GRAPH_GET_LINKS,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_GRAPH_GET_LINKS,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<TArray<UPtr<URigVMLink>>>().read() }
    }
    pub fn get_input_arguments(&self) -> TArray<FRigVMGraphVariableDescription> {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_GRAPH_GET_INPUT_ARGUMENTS,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_GRAPH_GET_INPUT_ARGUMENTS,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<TArray<FRigVMGraphVariableDescription>>().read()
        }
    }
    pub fn get_graph_name(&self) -> FString {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_GRAPH_GET_GRAPH_NAME,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_GRAPH_GET_GRAPH_NAME,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<FString>().read() }
    }
    pub fn get_graph_depth(&self) -> i32 {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_GRAPH_GET_GRAPH_DEPTH,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_GRAPH_GET_GRAPH_DEPTH,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<i32>().read() }
    }
    pub fn get_event_names(&self) -> TArray<FName> {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_GRAPH_GET_EVENT_NAMES,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_GRAPH_GET_EVENT_NAMES,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<TArray<FName>>().read() }
    }
    pub fn get_entry_node(&self) -> UPtr<URigVMFunctionEntryNode> {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_GRAPH_GET_ENTRY_NODE,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_GRAPH_GET_ENTRY_NODE,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<UPtr<URigVMFunctionEntryNode>>().read() }
    }
    pub fn get_default_function_library(&self) -> UPtr<URigVMFunctionLibrary> {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_GRAPH_GET_DEFAULT_FUNCTION_LIBRARY,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_GRAPH_GET_DEFAULT_FUNCTION_LIBRARY,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<UPtr<URigVMFunctionLibrary>>().read() }
    }
    pub fn get_contained_graphs(&self, b_recursive: bool) -> TArray<UPtr<URigVMGraph>> {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_GRAPH_GET_CONTAINED_GRAPHS,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_recursive,
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
                crate::bindings::rig_vm_developer::U_RIG_VM_GRAPH_GET_CONTAINED_GRAPHS,
                __buffer,
            )
        };
        unsafe { __buffer.add(8).cast::<TArray<UPtr<URigVMGraph>>>().read() }
    }
    pub fn find_pin(&self, in_pin_path: FString) -> UPtr<URigVMPin> {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_GRAPH_FIND_PIN,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_pin_path,
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
                crate::bindings::rig_vm_developer::U_RIG_VM_GRAPH_FIND_PIN,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<UPtr<URigVMPin>>().read() }
    }
    pub fn find_node_by_name(&self, in_node_name: &FName) -> UPtr<URigVMNode> {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_GRAPH_FIND_NODE_BY_NAME,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_node_name,
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
                crate::bindings::rig_vm_developer::U_RIG_VM_GRAPH_FIND_NODE_BY_NAME,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<UPtr<URigVMNode>>().read() }
    }
    pub fn find_node(&self, in_node_path: FString) -> UPtr<URigVMNode> {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_GRAPH_FIND_NODE,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_node_path,
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
                crate::bindings::rig_vm_developer::U_RIG_VM_GRAPH_FIND_NODE,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<UPtr<URigVMNode>>().read() }
    }
    pub fn find_link(
        &self,
        in_link_pin_path_representation: FString,
    ) -> UPtr<URigVMLink> {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_GRAPH_FIND_LINK,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_link_pin_path_representation,
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
                crate::bindings::rig_vm_developer::U_RIG_VM_GRAPH_FIND_LINK,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<UPtr<URigVMLink>>().read() }
    }
    pub fn contains_link(&self, in_pin_path_representation: FString) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<17>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_GRAPH_CONTAINS_LINK,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_pin_path_representation,
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
                crate::bindings::rig_vm_developer::U_RIG_VM_GRAPH_CONTAINS_LINK,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<bool>().read() }
    }
}
#[repr(C, align(8))]
pub struct URigVMFunctionLibrary {
    __padding_end: [u8; 576],
}
impl URigVMFunctionLibrary {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("URigVMFunctionLibrary")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
    pub fn get_references_for_function(
        &mut self,
        in_function_name: &FName,
    ) -> TArray<TSoftObjectPtr<URigVMFunctionReferenceNode>> {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_FUNCTION_LIBRARY_GET_REFERENCES_FOR_FUNCTION,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_function_name,
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
                crate::bindings::rig_vm_developer::U_RIG_VM_FUNCTION_LIBRARY_GET_REFERENCES_FOR_FUNCTION,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(16)
                .cast::<TArray<TSoftObjectPtr<URigVMFunctionReferenceNode>>>()
                .read()
        }
    }
    pub fn get_reference_paths_for_function(
        &mut self,
        in_function_name: &FName,
    ) -> TArray<FString> {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_FUNCTION_LIBRARY_GET_REFERENCE_PATHS_FOR_FUNCTION,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_function_name,
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
                crate::bindings::rig_vm_developer::U_RIG_VM_FUNCTION_LIBRARY_GET_REFERENCE_PATHS_FOR_FUNCTION,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<TArray<FString>>().read() }
    }
    pub fn get_functions(&self) -> TArray<UPtr<URigVMLibraryNode>> {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_FUNCTION_LIBRARY_GET_FUNCTIONS,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_FUNCTION_LIBRARY_GET_FUNCTIONS,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<TArray<UPtr<URigVMLibraryNode>>>().read() }
    }
    pub fn find_function_for_node(
        &self,
        in_node: UPtr<URigVMNode>,
    ) -> UPtr<URigVMLibraryNode> {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_FUNCTION_LIBRARY_FIND_FUNCTION_FOR_NODE,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_node,
                __buffer.add(0).cast::<UPtr<URigVMNode>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_FUNCTION_LIBRARY_FIND_FUNCTION_FOR_NODE,
                __buffer,
            )
        };
        unsafe { __buffer.add(8).cast::<UPtr<URigVMLibraryNode>>().read() }
    }
    pub fn find_function(&self, in_function_name: &FName) -> UPtr<URigVMLibraryNode> {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_FUNCTION_LIBRARY_FIND_FUNCTION,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_function_name,
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
                crate::bindings::rig_vm_developer::U_RIG_VM_FUNCTION_LIBRARY_FIND_FUNCTION,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<UPtr<URigVMLibraryNode>>().read() }
    }
}
#[repr(C, align(8))]
pub struct URigVMLink {
    __padding_end: [u8; 96],
}
impl URigVMLink {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("URigVMLink")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
    pub fn get_target_pin(&self) -> UPtr<URigVMPin> {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_LINK_GET_TARGET_PIN,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_LINK_GET_TARGET_PIN,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<UPtr<URigVMPin>>().read() }
    }
    pub fn get_target_node(&self) -> UPtr<URigVMNode> {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_LINK_GET_TARGET_NODE,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_LINK_GET_TARGET_NODE,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<UPtr<URigVMNode>>().read() }
    }
    pub fn get_source_pin(&self) -> UPtr<URigVMPin> {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_LINK_GET_SOURCE_PIN,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_LINK_GET_SOURCE_PIN,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<UPtr<URigVMPin>>().read() }
    }
    pub fn get_source_node(&self) -> UPtr<URigVMNode> {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_LINK_GET_SOURCE_NODE,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_LINK_GET_SOURCE_NODE,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<UPtr<URigVMNode>>().read() }
    }
    pub fn get_pin_path_representation(&self) -> FString {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_LINK_GET_PIN_PATH_REPRESENTATION,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_LINK_GET_PIN_PATH_REPRESENTATION,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<FString>().read() }
    }
    pub fn get_opposite_pin(&self, in_pin: UPtr<URigVMPin>) -> UPtr<URigVMPin> {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_LINK_GET_OPPOSITE_PIN,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_pin,
                __buffer.add(0).cast::<UPtr<URigVMPin>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_LINK_GET_OPPOSITE_PIN,
                __buffer,
            )
        };
        unsafe { __buffer.add(8).cast::<UPtr<URigVMPin>>().read() }
    }
    pub fn get_link_index(&self) -> i32 {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_LINK_GET_LINK_INDEX,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_LINK_GET_LINK_INDEX,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<i32>().read() }
    }
    pub fn get_graph_depth(&self) -> i32 {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_LINK_GET_GRAPH_DEPTH,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_LINK_GET_GRAPH_DEPTH,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<i32>().read() }
    }
    pub fn get_graph(&self) -> UPtr<URigVMGraph> {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_LINK_GET_GRAPH,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_LINK_GET_GRAPH,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<UPtr<URigVMGraph>>().read() }
    }
}
#[repr(C, align(8))]
pub struct URigVMInjectionInfo {
    __padding_end: [u8; 88],
}
impl URigVMInjectionInfo {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("URigVMInjectionInfo")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
    pub fn get_pin(&self) -> UPtr<URigVMPin> {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_INJECTION_INFO_GET_PIN,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_INJECTION_INFO_GET_PIN,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<UPtr<URigVMPin>>().read() }
    }
    pub fn get_graph(&self) -> UPtr<URigVMGraph> {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_INJECTION_INFO_GET_GRAPH,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_INJECTION_INFO_GET_GRAPH,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<UPtr<URigVMGraph>>().read() }
    }
}
#[repr(C, align(8))]
pub struct URigVMPin {
    __padding_end: [u8; 544],
}
impl URigVMPin {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("URigVMPin")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
    pub fn should_only_show_sub_pins(&self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_PIN_SHOULD_ONLY_SHOW_SUB_PINS,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_PIN_SHOULD_ONLY_SHOW_SUB_PINS,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn should_hide_sub_pins(&self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_PIN_SHOULD_HIDE_SUB_PINS,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_PIN_SHOULD_HIDE_SUB_PINS,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn requires_watch(&self, b_check_exposed_pin_chain: bool) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<2>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_PIN_REQUIRES_WATCH,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_check_exposed_pin_chain,
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
                crate::bindings::rig_vm_developer::U_RIG_VM_PIN_REQUIRES_WATCH,
                __buffer,
            )
        };
        unsafe { __buffer.add(1).cast::<bool>().read() }
    }
    pub fn is_wild_card(&self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_PIN_IS_WILD_CARD,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_PIN_IS_WILD_CARD,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn is_valid_default_value(&self, in_default_value: FString) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<17>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_PIN_IS_VALID_DEFAULT_VALUE,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_default_value,
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
                crate::bindings::rig_vm_developer::U_RIG_VM_PIN_IS_VALID_DEFAULT_VALUE,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<bool>().read() }
    }
    pub fn is_u_object(&self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_PIN_IS_U_OBJECT,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_PIN_IS_U_OBJECT,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn is_trait_pin(&self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_PIN_IS_TRAIT_PIN,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_PIN_IS_TRAIT_PIN,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn is_struct_member(&self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_PIN_IS_STRUCT_MEMBER,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_PIN_IS_STRUCT_MEMBER,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn is_struct(&self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_PIN_IS_STRUCT,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_PIN_IS_STRUCT,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn is_string_type(&self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_PIN_IS_STRING_TYPE,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_PIN_IS_STRING_TYPE,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn is_root_pin(&self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_PIN_IS_ROOT_PIN,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_PIN_IS_ROOT_PIN,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn is_reference_counted_container(&self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_PIN_IS_REFERENCE_COUNTED_CONTAINER,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_PIN_IS_REFERENCE_COUNTED_CONTAINER,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn is_linked_to(&self, in_pin: UPtr<URigVMPin>) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<9>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_PIN_IS_LINKED_TO,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_pin,
                __buffer.add(0).cast::<UPtr<URigVMPin>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_PIN_IS_LINKED_TO,
                __buffer,
            )
        };
        unsafe { __buffer.add(8).cast::<bool>().read() }
    }
    pub fn is_lazy(&self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_PIN_IS_LAZY,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_PIN_IS_LAZY,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn is_interface(&self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_PIN_IS_INTERFACE,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_PIN_IS_INTERFACE,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn is_fixed_size_array(&self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_PIN_IS_FIXED_SIZE_ARRAY,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_PIN_IS_FIXED_SIZE_ARRAY,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn is_expanded(&self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_PIN_IS_EXPANDED,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_PIN_IS_EXPANDED,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn is_execute_context(&self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_PIN_IS_EXECUTE_CONTEXT,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_PIN_IS_EXECUTE_CONTEXT,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn is_enum(&self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_PIN_IS_ENUM,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_PIN_IS_ENUM,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn is_dynamic_array(&self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_PIN_IS_DYNAMIC_ARRAY,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_PIN_IS_DYNAMIC_ARRAY,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn is_defined_as_constant(&self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_PIN_IS_DEFINED_AS_CONSTANT,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_PIN_IS_DEFINED_AS_CONSTANT,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn is_array_element(&self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_PIN_IS_ARRAY_ELEMENT,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_PIN_IS_ARRAY_ELEMENT,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn is_array(&self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_PIN_IS_ARRAY,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_PIN_IS_ARRAY,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn has_user_provided_default_value(&self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_PIN_HAS_USER_PROVIDED_DEFAULT_VALUE,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_PIN_HAS_USER_PROVIDED_DEFAULT_VALUE,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn has_original_default_value(&self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_PIN_HAS_ORIGINAL_DEFAULT_VALUE,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_PIN_HAS_ORIGINAL_DEFAULT_VALUE,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn has_meta_data(&self, in_key: FName) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<13>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_PIN_HAS_META_DATA,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&in_key, __buffer.add(0).cast::<FName>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_PIN_HAS_META_DATA,
                __buffer,
            )
        };
        unsafe { __buffer.add(12).cast::<bool>().read() }
    }
    pub fn has_default_value_override(&self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_PIN_HAS_DEFAULT_VALUE_OVERRIDE,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_PIN_HAS_DEFAULT_VALUE_OVERRIDE,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn get_tool_tip_text(&self) -> FText {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_PIN_GET_TOOL_TIP_TEXT,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_PIN_GET_TOOL_TIP_TEXT,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<FText>().read() }
    }
    pub fn get_target_links(&self, b_recursive: bool) -> TArray<UPtr<URigVMLink>> {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_PIN_GET_TARGET_LINKS,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_recursive,
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
                crate::bindings::rig_vm_developer::U_RIG_VM_PIN_GET_TARGET_LINKS,
                __buffer,
            )
        };
        unsafe { __buffer.add(8).cast::<TArray<UPtr<URigVMLink>>>().read() }
    }
    pub fn get_sub_pins(&self) -> TArray<UPtr<URigVMPin>> {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_PIN_GET_SUB_PINS,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_PIN_GET_SUB_PINS,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<TArray<UPtr<URigVMPin>>>().read() }
    }
    pub fn get_sub_pin_path(
        &self,
        in_parent_pin: UPtr<URigVMPin>,
        b_include_parent_pin_name: bool,
    ) -> FString {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_PIN_GET_SUB_PIN_PATH,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_parent_pin,
                __buffer.add(0).cast::<UPtr<URigVMPin>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_include_parent_pin_name,
                __buffer.add(8).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_PIN_GET_SUB_PIN_PATH,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<FString>().read() }
    }
    pub fn get_source_links(&self, b_recursive: bool) -> TArray<UPtr<URigVMLink>> {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_PIN_GET_SOURCE_LINKS,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_recursive,
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
                crate::bindings::rig_vm_developer::U_RIG_VM_PIN_GET_SOURCE_LINKS,
                __buffer,
            )
        };
        unsafe { __buffer.add(8).cast::<TArray<UPtr<URigVMLink>>>().read() }
    }
    pub fn get_segment_path(&self, b_include_root_pin: bool) -> FString {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_PIN_GET_SEGMENT_PATH,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_include_root_pin,
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
                crate::bindings::rig_vm_developer::U_RIG_VM_PIN_GET_SEGMENT_PATH,
                __buffer,
            )
        };
        unsafe { __buffer.add(8).cast::<FString>().read() }
    }
    pub fn get_script_struct(
        &self,
    ) -> UPtr<crate::bindings::core_u_object::UScriptStruct> {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_PIN_GET_SCRIPT_STRUCT,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_PIN_GET_SCRIPT_STRUCT,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(0)
                .cast::<UPtr<crate::bindings::core_u_object::UScriptStruct>>()
                .read()
        }
    }
    pub fn get_root_pin(&self) -> UPtr<URigVMPin> {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_PIN_GET_ROOT_PIN,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_PIN_GET_ROOT_PIN,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<UPtr<URigVMPin>>().read() }
    }
    pub fn get_pin_path(&self, b_use_node_path: bool) -> FString {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_PIN_GET_PIN_PATH,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_use_node_path,
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
                crate::bindings::rig_vm_developer::U_RIG_VM_PIN_GET_PIN_PATH,
                __buffer,
            )
        };
        unsafe { __buffer.add(8).cast::<FString>().read() }
    }
    pub fn get_pin_index(&self) -> i32 {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_PIN_GET_PIN_INDEX,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_PIN_GET_PIN_INDEX,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<i32>().read() }
    }
    pub fn get_pin_for_link(&self) -> UPtr<URigVMPin> {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_PIN_GET_PIN_FOR_LINK,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_PIN_GET_PIN_FOR_LINK,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<UPtr<URigVMPin>>().read() }
    }
    pub fn get_parent_script_struct(
        &self,
        fallback_node: UPtr<URigVMUnitNode>,
    ) -> UPtr<crate::bindings::core_u_object::UScriptStruct> {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_PIN_GET_PARENT_SCRIPT_STRUCT,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &fallback_node,
                __buffer.add(0).cast::<UPtr<URigVMUnitNode>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_PIN_GET_PARENT_SCRIPT_STRUCT,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(8)
                .cast::<UPtr<crate::bindings::core_u_object::UScriptStruct>>()
                .read()
        }
    }
    pub fn get_parent_pin(&self) -> UPtr<URigVMPin> {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_PIN_GET_PARENT_PIN,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_PIN_GET_PARENT_PIN,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<UPtr<URigVMPin>>().read() }
    }
    pub fn get_original_pin_from_injected_node(&self) -> UPtr<URigVMPin> {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_PIN_GET_ORIGINAL_PIN_FROM_INJECTED_NODE,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_PIN_GET_ORIGINAL_PIN_FROM_INJECTED_NODE,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<UPtr<URigVMPin>>().read() }
    }
    pub fn get_original_default_value(&self) -> FString {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_PIN_GET_ORIGINAL_DEFAULT_VALUE,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_PIN_GET_ORIGINAL_DEFAULT_VALUE,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<FString>().read() }
    }
    pub fn get_node(&self) -> UPtr<URigVMNode> {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_PIN_GET_NODE,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_PIN_GET_NODE,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<UPtr<URigVMNode>>().read() }
    }
    pub fn get_meta_data(&self, in_key: FName) -> FString {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_PIN_GET_META_DATA,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&in_key, __buffer.add(0).cast::<FName>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_PIN_GET_META_DATA,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<FString>().read() }
    }
    pub fn get_links(&self) -> TArray<UPtr<URigVMLink>> {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_PIN_GET_LINKS,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_PIN_GET_LINKS,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<TArray<UPtr<URigVMLink>>>().read() }
    }
    pub fn get_linked_target_pins(&self, b_recursive: bool) -> TArray<UPtr<URigVMPin>> {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_PIN_GET_LINKED_TARGET_PINS,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_recursive,
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
                crate::bindings::rig_vm_developer::U_RIG_VM_PIN_GET_LINKED_TARGET_PINS,
                __buffer,
            )
        };
        unsafe { __buffer.add(8).cast::<TArray<UPtr<URigVMPin>>>().read() }
    }
    pub fn get_linked_source_pins(&self, b_recursive: bool) -> TArray<UPtr<URigVMPin>> {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_PIN_GET_LINKED_SOURCE_PINS,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_recursive,
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
                crate::bindings::rig_vm_developer::U_RIG_VM_PIN_GET_LINKED_SOURCE_PINS,
                __buffer,
            )
        };
        unsafe { __buffer.add(8).cast::<TArray<UPtr<URigVMPin>>>().read() }
    }
    pub fn get_index_in_category(&self) -> i32 {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_PIN_GET_INDEX_IN_CATEGORY,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_PIN_GET_INDEX_IN_CATEGORY,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<i32>().read() }
    }
    pub fn get_graph(&self) -> UPtr<URigVMGraph> {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_PIN_GET_GRAPH,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_PIN_GET_GRAPH,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<UPtr<URigVMGraph>>().read() }
    }
    pub fn get_enum(&self) -> UPtr<crate::bindings::core_u_object::UEnum> {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_PIN_GET_ENUM,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_PIN_GET_ENUM,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<UPtr<crate::bindings::core_u_object::UEnum>>().read()
        }
    }
    pub fn get_display_name(&self) -> FName {
        let mut __stack = crate::core_data::StackAlloc::<12>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_PIN_GET_DISPLAY_NAME,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_PIN_GET_DISPLAY_NAME,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<FName>().read() }
    }
    pub fn get_direction(&self) -> crate::bindings::rig_vm::ERigVMPinDirection {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_PIN_GET_DIRECTION,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_PIN_GET_DIRECTION,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<crate::bindings::rig_vm::ERigVMPinDirection>().read()
        }
    }
    pub fn get_default_value_type(&self) -> ERigVMPinDefaultValueType {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_PIN_GET_DEFAULT_VALUE_TYPE,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_PIN_GET_DEFAULT_VALUE_TYPE,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<ERigVMPinDefaultValueType>().read() }
    }
    pub fn get_default_value(&self) -> FString {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_PIN_GET_DEFAULT_VALUE,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_PIN_GET_DEFAULT_VALUE,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<FString>().read() }
    }
    pub fn get_custom_widget_name(&self) -> FName {
        let mut __stack = crate::core_data::StackAlloc::<12>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_PIN_GET_CUSTOM_WIDGET_NAME,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_PIN_GET_CUSTOM_WIDGET_NAME,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<FName>().read() }
    }
    pub fn get_cpp_type_object(&self) -> UPtr<crate::bindings::core_u_object::UObject> {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_PIN_GET_CPP_TYPE_OBJECT,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_PIN_GET_CPP_TYPE_OBJECT,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(0)
                .cast::<UPtr<crate::bindings::core_u_object::UObject>>()
                .read()
        }
    }
    pub fn get_cpp_type(&self) -> FString {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_PIN_GET_CPP_TYPE,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_PIN_GET_CPP_TYPE,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<FString>().read() }
    }
    pub fn get_category(&self) -> FString {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_PIN_GET_CATEGORY,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_PIN_GET_CATEGORY,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<FString>().read() }
    }
    pub fn get_array_size(&self) -> i32 {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_PIN_GET_ARRAY_SIZE,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_PIN_GET_ARRAY_SIZE,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<i32>().read() }
    }
    pub fn get_array_element_cpp_type(&self) -> FString {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_PIN_GET_ARRAY_ELEMENT_CPP_TYPE,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_PIN_GET_ARRAY_ELEMENT_CPP_TYPE,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<FString>().read() }
    }
    pub fn get_all_sub_pins_recursively(&self) -> TArray<UPtr<URigVMPin>> {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_PIN_GET_ALL_SUB_PINS_RECURSIVELY,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_PIN_GET_ALL_SUB_PINS_RECURSIVELY,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<TArray<UPtr<URigVMPin>>>().read() }
    }
    pub fn get_absolute_pin_index(&self) -> i32 {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_PIN_GET_ABSOLUTE_PIN_INDEX,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_PIN_GET_ABSOLUTE_PIN_INDEX,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<i32>().read() }
    }
    pub fn find_sub_pin(&self, in_pin_path: FString) -> UPtr<URigVMPin> {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_PIN_FIND_SUB_PIN,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_pin_path,
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
                crate::bindings::rig_vm_developer::U_RIG_VM_PIN_FIND_SUB_PIN,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<UPtr<URigVMPin>>().read() }
    }
    pub fn find_link_for_pin(&self, in_other_pin: UPtr<URigVMPin>) -> UPtr<URigVMLink> {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_PIN_FIND_LINK_FOR_PIN,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_other_pin,
                __buffer.add(0).cast::<UPtr<URigVMPin>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_PIN_FIND_LINK_FOR_PIN,
                __buffer,
            )
        };
        unsafe { __buffer.add(8).cast::<UPtr<URigVMLink>>().read() }
    }
    pub fn contains_wild_card_sub_pin(&self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_PIN_CONTAINS_WILD_CARD_SUB_PIN,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_PIN_CONTAINS_WILD_CARD_SUB_PIN,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn can_provide_default_value(&self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_PIN_CAN_PROVIDE_DEFAULT_VALUE,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_PIN_CAN_PROVIDE_DEFAULT_VALUE,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
}
#[repr(C, align(8))]
pub struct URigVMSchema {
    __padding_end: [u8; 80],
}
impl URigVMSchema {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("URigVMSchema")
            .unwrap()
    }
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
pub struct URigVMUserWorkflowRegistry {
    __padding_end: [u8; 72],
}
impl URigVMUserWorkflowRegistry {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("URigVMUserWorkflowRegistry")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
    pub fn unregister_provider(&mut self, in_handle: i32) {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_USER_WORKFLOW_REGISTRY_UNREGISTER_PROVIDER,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&in_handle, __buffer.add(0).cast::<i32>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_USER_WORKFLOW_REGISTRY_UNREGISTER_PROVIDER,
                __buffer,
            )
        };
    }
    pub fn register_provider(
        &mut self,
        in_struct: UPtr<crate::bindings::core_u_object::UScriptStruct>,
        in_provider: FRegisterProvider_InProvider,
    ) -> i32 {
        let mut __stack = crate::core_data::StackAlloc::<44>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_USER_WORKFLOW_REGISTRY_REGISTER_PROVIDER,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_struct,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::core_u_object::UScriptStruct>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_provider,
                __buffer.add(8).cast::<FRegisterProvider_InProvider>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_USER_WORKFLOW_REGISTRY_REGISTER_PROVIDER,
                __buffer,
            )
        };
        unsafe { __buffer.add(40).cast::<i32>().read() }
    }
    pub fn get_workflows(
        &self,
        in_type: crate::bindings::rig_vm::ERigVMUserWorkflowType,
        in_struct: UPtr<crate::bindings::core_u_object::UScriptStruct>,
        in_subject: UPtr<crate::bindings::core_u_object::UObject>,
    ) -> TArray<crate::bindings::rig_vm::FRigVMUserWorkflow> {
        let mut __stack = crate::core_data::StackAlloc::<40>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_USER_WORKFLOW_REGISTRY_GET_WORKFLOWS,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_type,
                __buffer
                    .add(0)
                    .cast::<crate::bindings::rig_vm::ERigVMUserWorkflowType>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_struct,
                __buffer
                    .add(8)
                    .cast::<UPtr<crate::bindings::core_u_object::UScriptStruct>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_subject,
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
                crate::bindings::rig_vm_developer::U_RIG_VM_USER_WORKFLOW_REGISTRY_GET_WORKFLOWS,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(24)
                .cast::<TArray<crate::bindings::rig_vm::FRigVMUserWorkflow>>()
                .read()
        }
    }
    pub fn get() -> UPtr<URigVMUserWorkflowRegistry> {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_USER_WORKFLOW_REGISTRY_GET,
                __buffer,
            )
        };
        let __object_ptr = crate::bindings::rig_vm_developer::URigVMUserWorkflowRegistry::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_USER_WORKFLOW_REGISTRY_GET,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<UPtr<URigVMUserWorkflowRegistry>>().read() }
    }
}
#[repr(C, align(8))]
pub struct URigVMController {
    #[doc(hidden)]
    __padding_296: [u8; 296],
    pub modified_event_dynamic: FRigVMController_ModifiedEventDynamic,
    __padding_end: [u8; 408],
}
impl URigVMController {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("URigVMController")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
    pub fn upgrade_nodes(
        &mut self,
        in_node_names: &TArray<FName>,
        b_recursive: bool,
        b_setup_undo_redo: bool,
        b_print_python_command: bool,
    ) -> TArray<UPtr<URigVMNode>> {
        let mut __stack = crate::core_data::StackAlloc::<40>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_CONTROLLER_UPGRADE_NODES,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_node_names,
                __buffer.add(0).cast::<TArray<FName>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_recursive,
                __buffer.add(16).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_setup_undo_redo,
                __buffer.add(17).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_print_python_command,
                __buffer.add(18).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_CONTROLLER_UPGRADE_NODES,
                __buffer,
            )
        };
        unsafe { __buffer.add(24).cast::<TArray<UPtr<URigVMNode>>>().read() }
    }
    pub fn unresolve_template_nodes(
        &mut self,
        in_node_names: &TArray<FName>,
        b_setup_undo_redo: bool,
        b_print_python_command: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<19>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_CONTROLLER_UNRESOLVE_TEMPLATE_NODES,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_node_names,
                __buffer.add(0).cast::<TArray<FName>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_setup_undo_redo,
                __buffer.add(16).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_print_python_command,
                __buffer.add(17).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_CONTROLLER_UNRESOLVE_TEMPLATE_NODES,
                __buffer,
            )
        };
        unsafe { __buffer.add(18).cast::<bool>().read() }
    }
    pub fn undo(&mut self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_CONTROLLER_UNDO,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_CONTROLLER_UNDO,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn unbind_pin_from_variable(
        &mut self,
        in_pin_path: FString,
        b_setup_undo_redo: bool,
        b_print_python_command: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<19>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_CONTROLLER_UNBIND_PIN_FROM_VARIABLE,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_pin_path,
                __buffer.add(0).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_setup_undo_redo,
                __buffer.add(16).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_print_python_command,
                __buffer.add(17).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_CONTROLLER_UNBIND_PIN_FROM_VARIABLE,
                __buffer,
            )
        };
        unsafe { __buffer.add(18).cast::<bool>().read() }
    }
    pub fn swap_function_reference_by_name(
        &mut self,
        in_function_reference_node_name: &FName,
        in_new_function_identifier: &crate::bindings::rig_vm::FRigVMGraphFunctionIdentifier,
        b_setup_orphan_pins: bool,
        b_setup_undo_redo: bool,
        b_print_python_command: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<116>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_CONTROLLER_SWAP_FUNCTION_REFERENCE_BY_NAME,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_function_reference_node_name,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_new_function_identifier,
                __buffer
                    .add(16)
                    .cast::<crate::bindings::rig_vm::FRigVMGraphFunctionIdentifier>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_setup_orphan_pins,
                __buffer.add(112).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_setup_undo_redo,
                __buffer.add(113).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_print_python_command,
                __buffer.add(114).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_CONTROLLER_SWAP_FUNCTION_REFERENCE_BY_NAME,
                __buffer,
            )
        };
        unsafe { __buffer.add(115).cast::<bool>().read() }
    }
    pub fn swap_function_reference(
        &mut self,
        in_function_reference_node: UPtr<URigVMFunctionReferenceNode>,
        in_new_function_identifier: &crate::bindings::rig_vm::FRigVMGraphFunctionIdentifier,
        b_setup_orphan_pins: bool,
        b_setup_undo_redo: bool,
        b_print_python_command: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<108>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_CONTROLLER_SWAP_FUNCTION_REFERENCE,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_function_reference_node,
                __buffer.add(0).cast::<UPtr<URigVMFunctionReferenceNode>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_new_function_identifier,
                __buffer
                    .add(8)
                    .cast::<crate::bindings::rig_vm::FRigVMGraphFunctionIdentifier>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_setup_orphan_pins,
                __buffer.add(104).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_setup_undo_redo,
                __buffer.add(105).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_print_python_command,
                __buffer.add(106).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_CONTROLLER_SWAP_FUNCTION_REFERENCE,
                __buffer,
            )
        };
        unsafe { __buffer.add(107).cast::<bool>().read() }
    }
    pub fn swap_all_function_references(
        &mut self,
        in_old_function_identifier: &crate::bindings::rig_vm::FRigVMGraphFunctionIdentifier,
        in_new_function_identifier: &crate::bindings::rig_vm::FRigVMGraphFunctionIdentifier,
        b_setup_orphan_pins: bool,
        b_setup_undo_redo: bool,
        b_print_python_command: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<196>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_CONTROLLER_SWAP_ALL_FUNCTION_REFERENCES,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_old_function_identifier,
                __buffer
                    .add(0)
                    .cast::<crate::bindings::rig_vm::FRigVMGraphFunctionIdentifier>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_new_function_identifier,
                __buffer
                    .add(96)
                    .cast::<crate::bindings::rig_vm::FRigVMGraphFunctionIdentifier>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_setup_orphan_pins,
                __buffer.add(192).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_setup_undo_redo,
                __buffer.add(193).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_print_python_command,
                __buffer.add(194).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_CONTROLLER_SWAP_ALL_FUNCTION_REFERENCES,
                __buffer,
            )
        };
        unsafe { __buffer.add(195).cast::<bool>().read() }
    }
    pub fn split_function_variant(
        &mut self,
        in_function_name: &FName,
        b_setup_undo_redo: bool,
        b_print_python_command: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<15>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_CONTROLLER_SPLIT_FUNCTION_VARIANT,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_function_name,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_setup_undo_redo,
                __buffer.add(12).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_print_python_command,
                __buffer.add(13).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_CONTROLLER_SPLIT_FUNCTION_VARIANT,
                __buffer,
            )
        };
        unsafe { __buffer.add(14).cast::<bool>().read() }
    }
    pub fn set_unit_node_defaults(
        &mut self,
        in_node: UPtr<URigVMUnitNode>,
        in_defaults: FString,
        b_setup_undo_redo: bool,
        b_print_python_command: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<27>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_CONTROLLER_SET_UNIT_NODE_DEFAULTS,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_node,
                __buffer.add(0).cast::<UPtr<URigVMUnitNode>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_defaults,
                __buffer.add(8).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_setup_undo_redo,
                __buffer.add(24).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_print_python_command,
                __buffer.add(25).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_CONTROLLER_SET_UNIT_NODE_DEFAULTS,
                __buffer,
            )
        };
        unsafe { __buffer.add(26).cast::<bool>().read() }
    }
    pub fn set_schema_class(&mut self, in_schema_class: TSubclassOf<URigVMSchema>) {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_CONTROLLER_SET_SCHEMA_CLASS,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_schema_class,
                __buffer.add(0).cast::<TSubclassOf<URigVMSchema>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_CONTROLLER_SET_SCHEMA_CLASS,
                __buffer,
            )
        };
    }
    pub fn set_schema(&mut self, in_schema: UPtr<URigVMSchema>) {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_CONTROLLER_SET_SCHEMA,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_schema,
                __buffer.add(0).cast::<UPtr<URigVMSchema>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_CONTROLLER_SET_SCHEMA,
                __buffer,
            )
        };
    }
    pub fn set_remapped_variable(
        &mut self,
        in_function_ref_node: UPtr<URigVMFunctionReferenceNode>,
        in_inner_variable_name: &FName,
        in_outer_variable_name: &FName,
        b_setup_undo_redo: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<34>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_CONTROLLER_SET_REMAPPED_VARIABLE,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_function_ref_node,
                __buffer.add(0).cast::<UPtr<URigVMFunctionReferenceNode>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_inner_variable_name,
                __buffer.add(8).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_outer_variable_name,
                __buffer.add(20).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_setup_undo_redo,
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
                crate::bindings::rig_vm_developer::U_RIG_VM_CONTROLLER_SET_REMAPPED_VARIABLE,
                __buffer,
            )
        };
        unsafe { __buffer.add(33).cast::<bool>().read() }
    }
    pub fn set_pin_is_watched(
        &mut self,
        in_pin_path: FString,
        b_is_watched: bool,
        b_setup_undo_redo: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<19>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_CONTROLLER_SET_PIN_IS_WATCHED,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_pin_path,
                __buffer.add(0).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_is_watched,
                __buffer.add(16).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_setup_undo_redo,
                __buffer.add(17).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_CONTROLLER_SET_PIN_IS_WATCHED,
                __buffer,
            )
        };
        unsafe { __buffer.add(18).cast::<bool>().read() }
    }
    pub fn set_pin_index_in_category(
        &mut self,
        in_pin_path: FString,
        in_index_in_category: i32,
        b_setup_undo_redo: bool,
        b_print_python_command: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<23>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_CONTROLLER_SET_PIN_INDEX_IN_CATEGORY,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_pin_path,
                __buffer.add(0).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_index_in_category,
                __buffer.add(16).cast::<i32>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_setup_undo_redo,
                __buffer.add(20).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_print_python_command,
                __buffer.add(21).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_CONTROLLER_SET_PIN_INDEX_IN_CATEGORY,
                __buffer,
            )
        };
        unsafe { __buffer.add(22).cast::<bool>().read() }
    }
    pub fn set_pin_expansion(
        &mut self,
        in_pin_path: FString,
        b_is_expanded: bool,
        b_setup_undo_redo: bool,
        b_print_python_command: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<20>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_CONTROLLER_SET_PIN_EXPANSION,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_pin_path,
                __buffer.add(0).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_is_expanded,
                __buffer.add(16).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_setup_undo_redo,
                __buffer.add(17).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_print_python_command,
                __buffer.add(18).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_CONTROLLER_SET_PIN_EXPANSION,
                __buffer,
            )
        };
        unsafe { __buffer.add(19).cast::<bool>().read() }
    }
    pub fn set_pin_display_name(
        &mut self,
        in_pin_path: FString,
        in_display_name: FString,
        b_setup_undo_redo: bool,
        b_print_python_command: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<35>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_CONTROLLER_SET_PIN_DISPLAY_NAME,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_pin_path,
                __buffer.add(0).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_display_name,
                __buffer.add(16).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_setup_undo_redo,
                __buffer.add(32).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_print_python_command,
                __buffer.add(33).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_CONTROLLER_SET_PIN_DISPLAY_NAME,
                __buffer,
            )
        };
        unsafe { __buffer.add(34).cast::<bool>().read() }
    }
    pub fn set_pin_default_value(
        &mut self,
        in_pin_path: FString,
        in_default_value: FString,
        b_resize_arrays: bool,
        b_setup_undo_redo: bool,
        b_merge_undo_action: bool,
        b_print_python_command: bool,
        b_set_value_on_linked_pins: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<38>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_CONTROLLER_SET_PIN_DEFAULT_VALUE,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_pin_path,
                __buffer.add(0).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_default_value,
                __buffer.add(16).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_resize_arrays,
                __buffer.add(32).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_setup_undo_redo,
                __buffer.add(33).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_merge_undo_action,
                __buffer.add(34).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_print_python_command,
                __buffer.add(35).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_set_value_on_linked_pins,
                __buffer.add(36).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_CONTROLLER_SET_PIN_DEFAULT_VALUE,
                __buffer,
            )
        };
        unsafe { __buffer.add(37).cast::<bool>().read() }
    }
    pub fn set_pin_category_index(
        &mut self,
        in_node_name: &FName,
        in_pin_category: FString,
        in_new_index: i32,
        b_setup_undo_redo: bool,
        b_print_python_command: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<39>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_CONTROLLER_SET_PIN_CATEGORY_INDEX,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_node_name,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_pin_category,
                __buffer.add(16).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_new_index,
                __buffer.add(32).cast::<i32>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_setup_undo_redo,
                __buffer.add(36).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_print_python_command,
                __buffer.add(37).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_CONTROLLER_SET_PIN_CATEGORY_INDEX,
                __buffer,
            )
        };
        unsafe { __buffer.add(38).cast::<bool>().read() }
    }
    pub fn set_pin_category_expansion(
        &mut self,
        in_node_name: &FName,
        in_pin_category: FString,
        b_is_expanded: bool,
        b_setup_undo_redo: bool,
        b_print_python_command: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<36>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_CONTROLLER_SET_PIN_CATEGORY_EXPANSION,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_node_name,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_pin_category,
                __buffer.add(16).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_is_expanded,
                __buffer.add(32).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_setup_undo_redo,
                __buffer.add(33).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_print_python_command,
                __buffer.add(34).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_CONTROLLER_SET_PIN_CATEGORY_EXPANSION,
                __buffer,
            )
        };
        unsafe { __buffer.add(35).cast::<bool>().read() }
    }
    pub fn set_pin_category(
        &mut self,
        in_pin_path: FString,
        in_category: FString,
        b_setup_undo_redo: bool,
        b_print_python_command: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<35>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_CONTROLLER_SET_PIN_CATEGORY,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_pin_path,
                __buffer.add(0).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_category,
                __buffer.add(16).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_setup_undo_redo,
                __buffer.add(32).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_print_python_command,
                __buffer.add(33).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_CONTROLLER_SET_PIN_CATEGORY,
                __buffer,
            )
        };
        unsafe { __buffer.add(34).cast::<bool>().read() }
    }
    pub fn set_node_title_by_name(
        &mut self,
        in_node_name: &FName,
        in_node_title: FString,
        b_setup_undo_redo: bool,
        b_merge_undo_action: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<35>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_CONTROLLER_SET_NODE_TITLE_BY_NAME,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_node_name,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_node_title,
                __buffer.add(16).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_setup_undo_redo,
                __buffer.add(32).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_merge_undo_action,
                __buffer.add(33).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_CONTROLLER_SET_NODE_TITLE_BY_NAME,
                __buffer,
            )
        };
        unsafe { __buffer.add(34).cast::<bool>().read() }
    }
    pub fn set_node_title(
        &mut self,
        in_node: UPtr<URigVMNode>,
        in_node_title: FString,
        b_setup_undo_redo: bool,
        b_merge_undo_action: bool,
        b_print_python_command: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<28>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_CONTROLLER_SET_NODE_TITLE,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_node,
                __buffer.add(0).cast::<UPtr<URigVMNode>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_node_title,
                __buffer.add(8).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_setup_undo_redo,
                __buffer.add(24).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_merge_undo_action,
                __buffer.add(25).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_print_python_command,
                __buffer.add(26).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_CONTROLLER_SET_NODE_TITLE,
                __buffer,
            )
        };
        unsafe { __buffer.add(27).cast::<bool>().read() }
    }
    pub fn set_node_size_by_name(
        &mut self,
        in_node_name: &FName,
        in_size: &crate::bindings::core_u_object::FVector2D,
        b_setup_undo_redo: bool,
        b_merge_undo_action: bool,
        b_print_python_command: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<36>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_CONTROLLER_SET_NODE_SIZE_BY_NAME,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_node_name,
                __buffer.add(0).cast::<FName>(),
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
                &b_setup_undo_redo,
                __buffer.add(32).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_merge_undo_action,
                __buffer.add(33).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_print_python_command,
                __buffer.add(34).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_CONTROLLER_SET_NODE_SIZE_BY_NAME,
                __buffer,
            )
        };
        unsafe { __buffer.add(35).cast::<bool>().read() }
    }
    pub fn set_node_size(
        &mut self,
        in_node: UPtr<URigVMNode>,
        in_size: &crate::bindings::core_u_object::FVector2D,
        b_setup_undo_redo: bool,
        b_merge_undo_action: bool,
        b_print_python_command: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<28>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_CONTROLLER_SET_NODE_SIZE,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_node,
                __buffer.add(0).cast::<UPtr<URigVMNode>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_size,
                __buffer.add(8).cast::<crate::bindings::core_u_object::FVector2D>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_setup_undo_redo,
                __buffer.add(24).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_merge_undo_action,
                __buffer.add(25).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_print_python_command,
                __buffer.add(26).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_CONTROLLER_SET_NODE_SIZE,
                __buffer,
            )
        };
        unsafe { __buffer.add(27).cast::<bool>().read() }
    }
    pub fn set_node_selection(
        &mut self,
        in_node_names: &TArray<FName>,
        b_setup_undo_redo: bool,
        b_print_python_command: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<19>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_CONTROLLER_SET_NODE_SELECTION,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_node_names,
                __buffer.add(0).cast::<TArray<FName>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_setup_undo_redo,
                __buffer.add(16).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_print_python_command,
                __buffer.add(17).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_CONTROLLER_SET_NODE_SELECTION,
                __buffer,
            )
        };
        unsafe { __buffer.add(18).cast::<bool>().read() }
    }
    pub fn set_node_position_by_name(
        &mut self,
        in_node_name: &FName,
        in_position: &crate::bindings::core_u_object::FVector2D,
        b_setup_undo_redo: bool,
        b_merge_undo_action: bool,
        b_print_python_command: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<36>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_CONTROLLER_SET_NODE_POSITION_BY_NAME,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_node_name,
                __buffer.add(0).cast::<FName>(),
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
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_setup_undo_redo,
                __buffer.add(32).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_merge_undo_action,
                __buffer.add(33).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_print_python_command,
                __buffer.add(34).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_CONTROLLER_SET_NODE_POSITION_BY_NAME,
                __buffer,
            )
        };
        unsafe { __buffer.add(35).cast::<bool>().read() }
    }
    pub fn set_node_position(
        &mut self,
        in_node: UPtr<URigVMNode>,
        in_position: &crate::bindings::core_u_object::FVector2D,
        b_setup_undo_redo: bool,
        b_merge_undo_action: bool,
        b_print_python_command: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<28>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_CONTROLLER_SET_NODE_POSITION,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_node,
                __buffer.add(0).cast::<UPtr<URigVMNode>>(),
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
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_setup_undo_redo,
                __buffer.add(24).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_merge_undo_action,
                __buffer.add(25).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_print_python_command,
                __buffer.add(26).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_CONTROLLER_SET_NODE_POSITION,
                __buffer,
            )
        };
        unsafe { __buffer.add(27).cast::<bool>().read() }
    }
    pub fn set_node_layout(
        &mut self,
        in_node_name: &FName,
        in_layout: crate::bindings::rig_vm::FRigVMNodeLayout,
        b_setup_undo_redo: bool,
        b_print_python_command: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<195>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_CONTROLLER_SET_NODE_LAYOUT,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_node_name,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_layout,
                __buffer.add(16).cast::<crate::bindings::rig_vm::FRigVMNodeLayout>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_setup_undo_redo,
                __buffer.add(192).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_print_python_command,
                __buffer.add(193).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_CONTROLLER_SET_NODE_LAYOUT,
                __buffer,
            )
        };
        unsafe { __buffer.add(194).cast::<bool>().read() }
    }
    pub fn set_node_keywords_by_name(
        &mut self,
        in_node_name: &FName,
        in_keywords: FString,
        b_setup_undo_redo: bool,
        b_merge_undo_action: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<35>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_CONTROLLER_SET_NODE_KEYWORDS_BY_NAME,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_node_name,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_keywords,
                __buffer.add(16).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_setup_undo_redo,
                __buffer.add(32).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_merge_undo_action,
                __buffer.add(33).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_CONTROLLER_SET_NODE_KEYWORDS_BY_NAME,
                __buffer,
            )
        };
        unsafe { __buffer.add(34).cast::<bool>().read() }
    }
    pub fn set_node_keywords(
        &mut self,
        in_node: UPtr<URigVMCollapseNode>,
        in_keywords: FString,
        b_setup_undo_redo: bool,
        b_merge_undo_action: bool,
        b_print_python_command: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<28>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_CONTROLLER_SET_NODE_KEYWORDS,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_node,
                __buffer.add(0).cast::<UPtr<URigVMCollapseNode>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_keywords,
                __buffer.add(8).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_setup_undo_redo,
                __buffer.add(24).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_merge_undo_action,
                __buffer.add(25).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_print_python_command,
                __buffer.add(26).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_CONTROLLER_SET_NODE_KEYWORDS,
                __buffer,
            )
        };
        unsafe { __buffer.add(27).cast::<bool>().read() }
    }
    pub fn set_node_description_by_name(
        &mut self,
        in_node_name: &FName,
        in_description: FString,
        b_setup_undo_redo: bool,
        b_merge_undo_action: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<35>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_CONTROLLER_SET_NODE_DESCRIPTION_BY_NAME,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_node_name,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_description,
                __buffer.add(16).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_setup_undo_redo,
                __buffer.add(32).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_merge_undo_action,
                __buffer.add(33).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_CONTROLLER_SET_NODE_DESCRIPTION_BY_NAME,
                __buffer,
            )
        };
        unsafe { __buffer.add(34).cast::<bool>().read() }
    }
    pub fn set_node_description(
        &mut self,
        in_node: UPtr<URigVMCollapseNode>,
        in_description: FString,
        b_setup_undo_redo: bool,
        b_merge_undo_action: bool,
        b_print_python_command: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<28>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_CONTROLLER_SET_NODE_DESCRIPTION,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_node,
                __buffer.add(0).cast::<UPtr<URigVMCollapseNode>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_description,
                __buffer.add(8).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_setup_undo_redo,
                __buffer.add(24).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_merge_undo_action,
                __buffer.add(25).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_print_python_command,
                __buffer.add(26).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_CONTROLLER_SET_NODE_DESCRIPTION,
                __buffer,
            )
        };
        unsafe { __buffer.add(27).cast::<bool>().read() }
    }
    pub fn set_node_color_by_name(
        &mut self,
        in_node_name: &FName,
        in_color: &crate::bindings::core_u_object::FLinearColor,
        b_setup_undo_redo: bool,
        b_merge_undo_action: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<31>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_CONTROLLER_SET_NODE_COLOR_BY_NAME,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_node_name,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_color,
                __buffer.add(12).cast::<crate::bindings::core_u_object::FLinearColor>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_setup_undo_redo,
                __buffer.add(28).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_merge_undo_action,
                __buffer.add(29).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_CONTROLLER_SET_NODE_COLOR_BY_NAME,
                __buffer,
            )
        };
        unsafe { __buffer.add(30).cast::<bool>().read() }
    }
    pub fn set_node_color(
        &mut self,
        in_node: UPtr<URigVMNode>,
        in_color: &crate::bindings::core_u_object::FLinearColor,
        b_setup_undo_redo: bool,
        b_merge_undo_action: bool,
        b_print_python_command: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<28>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_CONTROLLER_SET_NODE_COLOR,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_node,
                __buffer.add(0).cast::<UPtr<URigVMNode>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_color,
                __buffer.add(8).cast::<crate::bindings::core_u_object::FLinearColor>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_setup_undo_redo,
                __buffer.add(24).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_merge_undo_action,
                __buffer.add(25).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_print_python_command,
                __buffer.add(26).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_CONTROLLER_SET_NODE_COLOR,
                __buffer,
            )
        };
        unsafe { __buffer.add(27).cast::<bool>().read() }
    }
    pub fn set_node_category_by_name(
        &mut self,
        in_node_name: &FName,
        in_category: FString,
        b_setup_undo_redo: bool,
        b_merge_undo_action: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<35>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_CONTROLLER_SET_NODE_CATEGORY_BY_NAME,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_node_name,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_category,
                __buffer.add(16).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_setup_undo_redo,
                __buffer.add(32).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_merge_undo_action,
                __buffer.add(33).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_CONTROLLER_SET_NODE_CATEGORY_BY_NAME,
                __buffer,
            )
        };
        unsafe { __buffer.add(34).cast::<bool>().read() }
    }
    pub fn set_node_category(
        &mut self,
        in_node: UPtr<URigVMCollapseNode>,
        in_category: FString,
        b_setup_undo_redo: bool,
        b_merge_undo_action: bool,
        b_print_python_command: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<28>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_CONTROLLER_SET_NODE_CATEGORY,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_node,
                __buffer.add(0).cast::<UPtr<URigVMCollapseNode>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_category,
                __buffer.add(8).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_setup_undo_redo,
                __buffer.add(24).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_merge_undo_action,
                __buffer.add(25).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_print_python_command,
                __buffer.add(26).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_CONTROLLER_SET_NODE_CATEGORY,
                __buffer,
            )
        };
        unsafe { __buffer.add(27).cast::<bool>().read() }
    }
    pub fn set_local_variable_type_from_object_path(
        &mut self,
        in_variable_name: &FName,
        in_cpp_type: FString,
        in_cpp_type_object_path: FString,
        b_setup_undo_redo: bool,
        b_print_python_command: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<51>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_CONTROLLER_SET_LOCAL_VARIABLE_TYPE_FROM_OBJECT_PATH,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_variable_name,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_cpp_type,
                __buffer.add(16).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_cpp_type_object_path,
                __buffer.add(32).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_setup_undo_redo,
                __buffer.add(48).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_print_python_command,
                __buffer.add(49).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_CONTROLLER_SET_LOCAL_VARIABLE_TYPE_FROM_OBJECT_PATH,
                __buffer,
            )
        };
        unsafe { __buffer.add(50).cast::<bool>().read() }
    }
    pub fn set_local_variable_type(
        &mut self,
        in_variable_name: &FName,
        in_cpp_type: FString,
        in_cpp_type_object: UPtr<crate::bindings::core_u_object::UObject>,
        b_setup_undo_redo: bool,
        b_print_python_command: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<43>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_CONTROLLER_SET_LOCAL_VARIABLE_TYPE,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_variable_name,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_cpp_type,
                __buffer.add(16).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_cpp_type_object,
                __buffer.add(32).cast::<UPtr<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_setup_undo_redo,
                __buffer.add(40).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_print_python_command,
                __buffer.add(41).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_CONTROLLER_SET_LOCAL_VARIABLE_TYPE,
                __buffer,
            )
        };
        unsafe { __buffer.add(42).cast::<bool>().read() }
    }
    pub fn set_local_variable_default_value(
        &mut self,
        in_variable_name: &FName,
        in_default_value: FString,
        b_setup_undo_redo: bool,
        b_print_python_command: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<35>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_CONTROLLER_SET_LOCAL_VARIABLE_DEFAULT_VALUE,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_variable_name,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_default_value,
                __buffer.add(16).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_setup_undo_redo,
                __buffer.add(32).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_print_python_command,
                __buffer.add(33).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_CONTROLLER_SET_LOCAL_VARIABLE_DEFAULT_VALUE,
                __buffer,
            )
        };
        unsafe { __buffer.add(34).cast::<bool>().read() }
    }
    pub fn set_is_running_unit_test(&mut self, b_is_running: bool) {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_CONTROLLER_SET_IS_RUNNING_UNIT_TEST,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_is_running,
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
                crate::bindings::rig_vm_developer::U_RIG_VM_CONTROLLER_SET_IS_RUNNING_UNIT_TEST,
                __buffer,
            )
        };
    }
    pub fn set_graph(&mut self, in_graph: UPtr<URigVMGraph>) {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_CONTROLLER_SET_GRAPH,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_graph,
                __buffer.add(0).cast::<UPtr<URigVMGraph>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_CONTROLLER_SET_GRAPH,
                __buffer,
            )
        };
    }
    pub fn set_exposed_pin_index(
        &mut self,
        in_pin_name: &FName,
        in_new_index: i32,
        b_setup_undo_redo: bool,
        b_print_python_command: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<19>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_CONTROLLER_SET_EXPOSED_PIN_INDEX,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_pin_name,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_new_index,
                __buffer.add(12).cast::<i32>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_setup_undo_redo,
                __buffer.add(16).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_print_python_command,
                __buffer.add(17).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_CONTROLLER_SET_EXPOSED_PIN_INDEX,
                __buffer,
            )
        };
        unsafe { __buffer.add(18).cast::<bool>().read() }
    }
    pub fn set_comment_text_by_name(
        &mut self,
        in_node_name: &FName,
        in_comment_text: FString,
        in_comment_font_size: &i32,
        b_in_comment_bubble_visible: &bool,
        b_in_comment_color_bubble: &bool,
        b_setup_undo_redo: bool,
        b_print_python_command: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<41>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_CONTROLLER_SET_COMMENT_TEXT_BY_NAME,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_node_name,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_comment_text,
                __buffer.add(16).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_comment_font_size,
                __buffer.add(32).cast::<i32>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                b_in_comment_bubble_visible,
                __buffer.add(36).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                b_in_comment_color_bubble,
                __buffer.add(37).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_setup_undo_redo,
                __buffer.add(38).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_print_python_command,
                __buffer.add(39).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_CONTROLLER_SET_COMMENT_TEXT_BY_NAME,
                __buffer,
            )
        };
        unsafe { __buffer.add(40).cast::<bool>().read() }
    }
    pub fn set_comment_text(
        &mut self,
        in_node: UPtr<URigVMNode>,
        in_comment_text: FString,
        in_comment_font_size: &i32,
        b_in_comment_bubble_visible: &bool,
        b_in_comment_color_bubble: &bool,
        b_setup_undo_redo: bool,
        b_print_python_command: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<33>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_CONTROLLER_SET_COMMENT_TEXT,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_node,
                __buffer.add(0).cast::<UPtr<URigVMNode>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_comment_text,
                __buffer.add(8).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_comment_font_size,
                __buffer.add(24).cast::<i32>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                b_in_comment_bubble_visible,
                __buffer.add(28).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                b_in_comment_color_bubble,
                __buffer.add(29).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_setup_undo_redo,
                __buffer.add(30).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_print_python_command,
                __buffer.add(31).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_CONTROLLER_SET_COMMENT_TEXT,
                __buffer,
            )
        };
        unsafe { __buffer.add(32).cast::<bool>().read() }
    }
    pub fn set_array_pin_size(
        &mut self,
        in_array_pin_path: FString,
        in_size: i32,
        in_default_value: FString,
        b_setup_undo_redo: bool,
        b_print_python_command: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<43>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_CONTROLLER_SET_ARRAY_PIN_SIZE,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_array_pin_path,
                __buffer.add(0).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&in_size, __buffer.add(16).cast::<i32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_default_value,
                __buffer.add(24).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_setup_undo_redo,
                __buffer.add(40).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_print_python_command,
                __buffer.add(41).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_CONTROLLER_SET_ARRAY_PIN_SIZE,
                __buffer,
            )
        };
        unsafe { __buffer.add(42).cast::<bool>().read() }
    }
    pub fn set_action_stack(&mut self, in_action_stack: UPtr<URigVMActionStack>) {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_CONTROLLER_SET_ACTION_STACK,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_action_stack,
                __buffer.add(0).cast::<UPtr<URigVMActionStack>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_CONTROLLER_SET_ACTION_STACK,
                __buffer,
            )
        };
    }
    pub fn select_node_islands(
        &mut self,
        in_node_names: &TArray<FName>,
        b_clear_selection: bool,
        b_setup_undo_redo: bool,
        b_print_python_command: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<20>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_CONTROLLER_SELECT_NODE_ISLANDS,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_node_names,
                __buffer.add(0).cast::<TArray<FName>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_clear_selection,
                __buffer.add(16).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_setup_undo_redo,
                __buffer.add(17).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_print_python_command,
                __buffer.add(18).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_CONTROLLER_SELECT_NODE_ISLANDS,
                __buffer,
            )
        };
        unsafe { __buffer.add(19).cast::<bool>().read() }
    }
    pub fn select_node_by_name(
        &mut self,
        in_node_name: &FName,
        b_select: bool,
        b_setup_undo_redo: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<15>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_CONTROLLER_SELECT_NODE_BY_NAME,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_node_name,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&b_select, __buffer.add(12).cast::<bool>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_setup_undo_redo,
                __buffer.add(13).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_CONTROLLER_SELECT_NODE_BY_NAME,
                __buffer,
            )
        };
        unsafe { __buffer.add(14).cast::<bool>().read() }
    }
    pub fn select_node(
        &mut self,
        in_node: UPtr<URigVMNode>,
        b_select: bool,
        b_setup_undo_redo: bool,
        b_print_python_command: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<12>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_CONTROLLER_SELECT_NODE,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_node,
                __buffer.add(0).cast::<UPtr<URigVMNode>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&b_select, __buffer.add(8).cast::<bool>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_setup_undo_redo,
                __buffer.add(9).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_print_python_command,
                __buffer.add(10).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_CONTROLLER_SELECT_NODE,
                __buffer,
            )
        };
        unsafe { __buffer.add(11).cast::<bool>().read() }
    }
    pub fn select_linked_nodes(
        &mut self,
        in_node_names: &TArray<FName>,
        b_select_source_nodes: bool,
        b_clear_selection: bool,
        b_setup_undo_redo: bool,
        b_print_python_command: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<21>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_CONTROLLER_SELECT_LINKED_NODES,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_node_names,
                __buffer.add(0).cast::<TArray<FName>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_select_source_nodes,
                __buffer.add(16).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_clear_selection,
                __buffer.add(17).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_setup_undo_redo,
                __buffer.add(18).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_print_python_command,
                __buffer.add(19).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_CONTROLLER_SELECT_LINKED_NODES,
                __buffer,
            )
        };
        unsafe { __buffer.add(20).cast::<bool>().read() }
    }
    pub fn resolve_wild_card_pin(
        &mut self,
        in_pin_path: FString,
        in_cpp_type: FString,
        in_cpp_type_object_path: &FName,
        b_setup_undo_redo: bool,
        b_print_python_command: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<47>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_CONTROLLER_RESOLVE_WILD_CARD_PIN,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_pin_path,
                __buffer.add(0).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_cpp_type,
                __buffer.add(16).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_cpp_type_object_path,
                __buffer.add(32).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_setup_undo_redo,
                __buffer.add(44).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_print_python_command,
                __buffer.add(45).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_CONTROLLER_RESOLVE_WILD_CARD_PIN,
                __buffer,
            )
        };
        unsafe { __buffer.add(46).cast::<bool>().read() }
    }
    pub fn reset_pin_default_value(
        &mut self,
        in_pin_path: FString,
        b_setup_undo_redo: bool,
        b_print_python_command: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<19>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_CONTROLLER_RESET_PIN_DEFAULT_VALUE,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_pin_path,
                __buffer.add(0).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_setup_undo_redo,
                __buffer.add(16).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_print_python_command,
                __buffer.add(17).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_CONTROLLER_RESET_PIN_DEFAULT_VALUE,
                __buffer,
            )
        };
        unsafe { __buffer.add(18).cast::<bool>().read() }
    }
    pub fn reset_default_value_for_pins(
        &mut self,
        in_pin_paths: &TArray<FString>,
        b_setup_undo: bool,
        b_print_python_command: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<19>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_CONTROLLER_RESET_DEFAULT_VALUE_FOR_PINS,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_pin_paths,
                __buffer.add(0).cast::<TArray<FString>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_setup_undo,
                __buffer.add(16).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_print_python_command,
                __buffer.add(17).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_CONTROLLER_RESET_DEFAULT_VALUE_FOR_PINS,
                __buffer,
            )
        };
        unsafe { __buffer.add(18).cast::<bool>().read() }
    }
    pub fn reset_default_value_for_all_pins_on_nodes(
        &mut self,
        in_node_names: &TArray<FName>,
        b_setup_undo: bool,
        b_print_python_command: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<19>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_CONTROLLER_RESET_DEFAULT_VALUE_FOR_ALL_PINS_ON_NODES,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_node_names,
                __buffer.add(0).cast::<TArray<FName>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_setup_undo,
                __buffer.add(16).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_print_python_command,
                __buffer.add(17).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_CONTROLLER_RESET_DEFAULT_VALUE_FOR_ALL_PINS_ON_NODES,
                __buffer,
            )
        };
        unsafe { __buffer.add(18).cast::<bool>().read() }
    }
    pub fn reset_default_value_for_all_pins_on_node(
        &mut self,
        in_node_name: &FName,
        b_setup_undo: bool,
        b_print_python_command: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<15>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_CONTROLLER_RESET_DEFAULT_VALUE_FOR_ALL_PINS_ON_NODE,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_node_name,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_setup_undo,
                __buffer.add(12).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_print_python_command,
                __buffer.add(13).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_CONTROLLER_RESET_DEFAULT_VALUE_FOR_ALL_PINS_ON_NODE,
                __buffer,
            )
        };
        unsafe { __buffer.add(14).cast::<bool>().read() }
    }
    pub fn replace_parameter_node_with_variable(
        &mut self,
        in_node_name: &FName,
        in_variable_name: &FName,
        in_cpp_type: FString,
        in_cpp_type_object: UPtr<crate::bindings::core_u_object::UObject>,
        b_setup_undo_redo: bool,
    ) -> UPtr<URigVMVariableNode> {
        let mut __stack = crate::core_data::StackAlloc::<64>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_CONTROLLER_REPLACE_PARAMETER_NODE_WITH_VARIABLE,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_node_name,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_variable_name,
                __buffer.add(12).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_cpp_type,
                __buffer.add(24).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_cpp_type_object,
                __buffer.add(40).cast::<UPtr<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_setup_undo_redo,
                __buffer.add(48).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_CONTROLLER_REPLACE_PARAMETER_NODE_WITH_VARIABLE,
                __buffer,
            )
        };
        unsafe { __buffer.add(56).cast::<UPtr<URigVMVariableNode>>().read() }
    }
    pub fn rename_variable(
        &mut self,
        in_old_name: &FName,
        in_new_name: &FName,
        b_setup_undo_redo: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<26>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_CONTROLLER_RENAME_VARIABLE,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_old_name,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_new_name,
                __buffer.add(12).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_setup_undo_redo,
                __buffer.add(24).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_CONTROLLER_RENAME_VARIABLE,
                __buffer,
            )
        };
        unsafe { __buffer.add(25).cast::<bool>().read() }
    }
    pub fn rename_pin_category(
        &mut self,
        in_node_name: &FName,
        in_old_pin_category: FString,
        in_new_pin_category: FString,
        b_setup_undo_redo: bool,
        b_print_python_command: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<51>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_CONTROLLER_RENAME_PIN_CATEGORY,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_node_name,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_old_pin_category,
                __buffer.add(16).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_new_pin_category,
                __buffer.add(32).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_setup_undo_redo,
                __buffer.add(48).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_print_python_command,
                __buffer.add(49).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_CONTROLLER_RENAME_PIN_CATEGORY,
                __buffer,
            )
        };
        unsafe { __buffer.add(50).cast::<bool>().read() }
    }
    pub fn rename_parameter(
        &mut self,
        in_old_name: &FName,
        in_new_name: &FName,
        b_setup_undo_redo: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<26>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_CONTROLLER_RENAME_PARAMETER,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_old_name,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_new_name,
                __buffer.add(12).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_setup_undo_redo,
                __buffer.add(24).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_CONTROLLER_RENAME_PARAMETER,
                __buffer,
            )
        };
        unsafe { __buffer.add(25).cast::<bool>().read() }
    }
    pub fn rename_node(
        &mut self,
        in_node: UPtr<URigVMNode>,
        in_new_name: &FName,
        b_setup_undo_redo: bool,
        b_print_python_command: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<23>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_CONTROLLER_RENAME_NODE,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_node,
                __buffer.add(0).cast::<UPtr<URigVMNode>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_new_name,
                __buffer.add(8).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_setup_undo_redo,
                __buffer.add(20).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_print_python_command,
                __buffer.add(21).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_CONTROLLER_RENAME_NODE,
                __buffer,
            )
        };
        unsafe { __buffer.add(22).cast::<bool>().read() }
    }
    pub fn rename_local_variable(
        &mut self,
        in_variable_name: &FName,
        in_new_variable_name: &FName,
        b_setup_undo_redo: bool,
        b_print_python_command: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<27>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_CONTROLLER_RENAME_LOCAL_VARIABLE,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_variable_name,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_new_variable_name,
                __buffer.add(12).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_setup_undo_redo,
                __buffer.add(24).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_print_python_command,
                __buffer.add(25).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_CONTROLLER_RENAME_LOCAL_VARIABLE,
                __buffer,
            )
        };
        unsafe { __buffer.add(26).cast::<bool>().read() }
    }
    pub fn rename_function(
        &mut self,
        in_old_function_name: &FName,
        in_new_function_name: &FName,
        b_setup_undo_redo: bool,
        b_print_python_command: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<27>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_CONTROLLER_RENAME_FUNCTION,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_old_function_name,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_new_function_name,
                __buffer.add(12).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_setup_undo_redo,
                __buffer.add(24).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_print_python_command,
                __buffer.add(25).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_CONTROLLER_RENAME_FUNCTION,
                __buffer,
            )
        };
        unsafe { __buffer.add(26).cast::<bool>().read() }
    }
    pub fn rename_exposed_pin(
        &mut self,
        in_old_pin_name: &FName,
        in_new_pin_name: &FName,
        b_setup_undo_redo: bool,
        b_print_python_command: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<27>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_CONTROLLER_RENAME_EXPOSED_PIN,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_old_pin_name,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_new_pin_name,
                __buffer.add(12).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_setup_undo_redo,
                __buffer.add(24).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_print_python_command,
                __buffer.add(25).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_CONTROLLER_RENAME_EXPOSED_PIN,
                __buffer,
            )
        };
        unsafe { __buffer.add(26).cast::<bool>().read() }
    }
    pub fn remove_trait(
        &mut self,
        in_node_name: &FName,
        in_trait_name: &FName,
        b_setup_undo_redo: bool,
        b_print_python_command: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<27>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_CONTROLLER_REMOVE_TRAIT,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_node_name,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_trait_name,
                __buffer.add(12).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_setup_undo_redo,
                __buffer.add(24).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_print_python_command,
                __buffer.add(25).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_CONTROLLER_REMOVE_TRAIT,
                __buffer,
            )
        };
        unsafe { __buffer.add(26).cast::<bool>().read() }
    }
    pub fn remove_tag_from_function_variant(
        &mut self,
        in_function_name: &FName,
        in_tag_name: &FName,
        b_setup_undo_redo: bool,
        b_print_python_command: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<27>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_CONTROLLER_REMOVE_TAG_FROM_FUNCTION_VARIANT,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_function_name,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_tag_name,
                __buffer.add(12).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_setup_undo_redo,
                __buffer.add(24).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_print_python_command,
                __buffer.add(25).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_CONTROLLER_REMOVE_TAG_FROM_FUNCTION_VARIANT,
                __buffer,
            )
        };
        unsafe { __buffer.add(26).cast::<bool>().read() }
    }
    pub fn remove_pin_category(
        &mut self,
        in_node_name: &FName,
        in_pin_category: FString,
        b_setup_undo_redo: bool,
        b_print_python_command: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<35>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_CONTROLLER_REMOVE_PIN_CATEGORY,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_node_name,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_pin_category,
                __buffer.add(16).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_setup_undo_redo,
                __buffer.add(32).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_print_python_command,
                __buffer.add(33).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_CONTROLLER_REMOVE_PIN_CATEGORY,
                __buffer,
            )
        };
        unsafe { __buffer.add(34).cast::<bool>().read() }
    }
    pub fn remove_nodes_by_name(
        &mut self,
        in_node_names: &TArray<FName>,
        b_setup_undo_redo: bool,
        b_print_python_command: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<19>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_CONTROLLER_REMOVE_NODES_BY_NAME,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_node_names,
                __buffer.add(0).cast::<TArray<FName>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_setup_undo_redo,
                __buffer.add(16).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_print_python_command,
                __buffer.add(17).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_CONTROLLER_REMOVE_NODES_BY_NAME,
                __buffer,
            )
        };
        unsafe { __buffer.add(18).cast::<bool>().read() }
    }
    pub fn remove_nodes(
        &mut self,
        in_nodes: TArray<UPtr<URigVMNode>>,
        b_setup_undo_redo: bool,
        b_print_python_command: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<19>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_CONTROLLER_REMOVE_NODES,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_nodes,
                __buffer.add(0).cast::<TArray<UPtr<URigVMNode>>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_setup_undo_redo,
                __buffer.add(16).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_print_python_command,
                __buffer.add(17).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_CONTROLLER_REMOVE_NODES,
                __buffer,
            )
        };
        unsafe { __buffer.add(18).cast::<bool>().read() }
    }
    pub fn remove_node_by_name(
        &mut self,
        in_node_name: &FName,
        b_setup_undo_redo: bool,
        b_print_python_command: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<15>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_CONTROLLER_REMOVE_NODE_BY_NAME,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_node_name,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_setup_undo_redo,
                __buffer.add(12).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_print_python_command,
                __buffer.add(13).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_CONTROLLER_REMOVE_NODE_BY_NAME,
                __buffer,
            )
        };
        unsafe { __buffer.add(14).cast::<bool>().read() }
    }
    pub fn remove_node(
        &mut self,
        in_node: UPtr<URigVMNode>,
        b_setup_undo_redo: bool,
        b_print_python_command: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<11>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_CONTROLLER_REMOVE_NODE,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_node,
                __buffer.add(0).cast::<UPtr<URigVMNode>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_setup_undo_redo,
                __buffer.add(8).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_print_python_command,
                __buffer.add(9).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_CONTROLLER_REMOVE_NODE,
                __buffer,
            )
        };
        unsafe { __buffer.add(10).cast::<bool>().read() }
    }
    pub fn remove_local_variable(
        &mut self,
        in_variable_name: &FName,
        b_setup_undo_redo: bool,
        b_print_python_command: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<15>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_CONTROLLER_REMOVE_LOCAL_VARIABLE,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_variable_name,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_setup_undo_redo,
                __buffer.add(12).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_print_python_command,
                __buffer.add(13).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_CONTROLLER_REMOVE_LOCAL_VARIABLE,
                __buffer,
            )
        };
        unsafe { __buffer.add(14).cast::<bool>().read() }
    }
    pub fn remove_injected_node(
        &mut self,
        in_pin_path: FString,
        b_as_input: bool,
        b_setup_undo_redo: bool,
        b_print_python_command: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<20>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_CONTROLLER_REMOVE_INJECTED_NODE,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_pin_path,
                __buffer.add(0).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_as_input,
                __buffer.add(16).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_setup_undo_redo,
                __buffer.add(17).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_print_python_command,
                __buffer.add(18).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_CONTROLLER_REMOVE_INJECTED_NODE,
                __buffer,
            )
        };
        unsafe { __buffer.add(19).cast::<bool>().read() }
    }
    pub fn remove_function_from_library(
        &mut self,
        in_function_name: &FName,
        b_setup_undo_redo: bool,
        b_print_python_command: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<15>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_CONTROLLER_REMOVE_FUNCTION_FROM_LIBRARY,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_function_name,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_setup_undo_redo,
                __buffer.add(12).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_print_python_command,
                __buffer.add(13).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_CONTROLLER_REMOVE_FUNCTION_FROM_LIBRARY,
                __buffer,
            )
        };
        unsafe { __buffer.add(14).cast::<bool>().read() }
    }
    pub fn remove_exposed_pin(
        &mut self,
        in_pin_name: &FName,
        b_setup_undo_redo: bool,
        b_print_python_command: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<15>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_CONTROLLER_REMOVE_EXPOSED_PIN,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_pin_name,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_setup_undo_redo,
                __buffer.add(12).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_print_python_command,
                __buffer.add(13).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_CONTROLLER_REMOVE_EXPOSED_PIN,
                __buffer,
            )
        };
        unsafe { __buffer.add(14).cast::<bool>().read() }
    }
    pub fn remove_array_pin(
        &mut self,
        in_array_element_pin_path: FString,
        b_setup_undo_redo: bool,
        b_print_python_command: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<19>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_CONTROLLER_REMOVE_ARRAY_PIN,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_array_element_pin_path,
                __buffer.add(0).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_setup_undo_redo,
                __buffer.add(16).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_print_python_command,
                __buffer.add(17).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_CONTROLLER_REMOVE_ARRAY_PIN,
                __buffer,
            )
        };
        unsafe { __buffer.add(18).cast::<bool>().read() }
    }
    pub fn remove_aggregate_pin(
        &mut self,
        in_pin_path: FString,
        b_setup_undo_redo: bool,
        b_print_python_command: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<19>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_CONTROLLER_REMOVE_AGGREGATE_PIN,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_pin_path,
                __buffer.add(0).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_setup_undo_redo,
                __buffer.add(16).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_print_python_command,
                __buffer.add(17).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_CONTROLLER_REMOVE_AGGREGATE_PIN,
                __buffer,
            )
        };
        unsafe { __buffer.add(18).cast::<bool>().read() }
    }
    pub fn refresh_variable_node(
        &mut self,
        in_node_name: &FName,
        in_variable_name: &FName,
        in_cpp_type: FString,
        in_cpp_type_object: UPtr<crate::bindings::core_u_object::UObject>,
        b_setup_undo_redo: bool,
        b_setup_orphan_pins: bool,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<50>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_CONTROLLER_REFRESH_VARIABLE_NODE,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_node_name,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_variable_name,
                __buffer.add(12).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_cpp_type,
                __buffer.add(24).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_cpp_type_object,
                __buffer.add(40).cast::<UPtr<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_setup_undo_redo,
                __buffer.add(48).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_setup_orphan_pins,
                __buffer.add(49).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_CONTROLLER_REFRESH_VARIABLE_NODE,
                __buffer,
            )
        };
    }
    pub fn redo(&mut self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_CONTROLLER_REDO,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_CONTROLLER_REDO,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn push_graph(
        &mut self,
        in_graph: UPtr<URigVMGraph>,
        b_setup_undo_redo: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<10>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_CONTROLLER_PUSH_GRAPH,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_graph,
                __buffer.add(0).cast::<UPtr<URigVMGraph>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_setup_undo_redo,
                __buffer.add(8).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_CONTROLLER_PUSH_GRAPH,
                __buffer,
            )
        };
        unsafe { __buffer.add(9).cast::<bool>().read() }
    }
    pub fn promote_pin_to_variable(
        &mut self,
        in_pin_path: FString,
        b_create_variable_node: bool,
        in_node_position: &crate::bindings::core_u_object::FVector2D,
        b_setup_undo_redo: bool,
        b_print_python_command: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<43>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_CONTROLLER_PROMOTE_PIN_TO_VARIABLE,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_pin_path,
                __buffer.add(0).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_create_variable_node,
                __buffer.add(16).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_node_position,
                __buffer.add(24).cast::<crate::bindings::core_u_object::FVector2D>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_setup_undo_redo,
                __buffer.add(40).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_print_python_command,
                __buffer.add(41).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_CONTROLLER_PROMOTE_PIN_TO_VARIABLE,
                __buffer,
            )
        };
        unsafe { __buffer.add(42).cast::<bool>().read() }
    }
    pub fn promote_function_reference_node_to_collapse_node(
        &mut self,
        in_node_name: &FName,
        b_setup_undo_redo: bool,
        b_print_python_command: bool,
        b_remove_function_definition: bool,
    ) -> FName {
        let mut __stack = crate::core_data::StackAlloc::<28>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_CONTROLLER_PROMOTE_FUNCTION_REFERENCE_NODE_TO_COLLAPSE_NODE,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_node_name,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_setup_undo_redo,
                __buffer.add(12).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_print_python_command,
                __buffer.add(13).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_remove_function_definition,
                __buffer.add(14).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_CONTROLLER_PROMOTE_FUNCTION_REFERENCE_NODE_TO_COLLAPSE_NODE,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<FName>().read() }
    }
    pub fn promote_collapse_node_to_function_reference_node(
        &mut self,
        in_node_name: &FName,
        b_setup_undo_redo: bool,
        b_print_python_command: bool,
        in_existing_function_definition_path: FString,
    ) -> FName {
        let mut __stack = crate::core_data::StackAlloc::<44>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_CONTROLLER_PROMOTE_COLLAPSE_NODE_TO_FUNCTION_REFERENCE_NODE,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_node_name,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_setup_undo_redo,
                __buffer.add(12).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_print_python_command,
                __buffer.add(13).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_existing_function_definition_path,
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
                crate::bindings::rig_vm_developer::U_RIG_VM_CONTROLLER_PROMOTE_COLLAPSE_NODE_TO_FUNCTION_REFERENCE_NODE,
                __buffer,
            )
        };
        unsafe { __buffer.add(32).cast::<FName>().read() }
    }
    pub fn pop_graph(&mut self, b_setup_undo_redo: bool) -> UPtr<URigVMGraph> {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_CONTROLLER_POP_GRAPH,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_setup_undo_redo,
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
                crate::bindings::rig_vm_developer::U_RIG_VM_CONTROLLER_POP_GRAPH,
                __buffer,
            )
        };
        unsafe { __buffer.add(8).cast::<UPtr<URigVMGraph>>().read() }
    }
    pub fn perform_user_workflow(
        &mut self,
        in_workflow: &crate::bindings::rig_vm::FRigVMUserWorkflow,
        in_options: UPtr<crate::bindings::rig_vm::URigVMUserWorkflowOptions>,
        b_setup_undo_redo: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<122>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_CONTROLLER_PERFORM_USER_WORKFLOW,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_workflow,
                __buffer.add(0).cast::<crate::bindings::rig_vm::FRigVMUserWorkflow>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_options,
                __buffer
                    .add(112)
                    .cast::<UPtr<crate::bindings::rig_vm::URigVMUserWorkflowOptions>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_setup_undo_redo,
                __buffer.add(120).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_CONTROLLER_PERFORM_USER_WORKFLOW,
                __buffer,
            )
        };
        unsafe { __buffer.add(121).cast::<bool>().read() }
    }
    pub fn open_undo_bracket(&mut self, in_title: FString) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<17>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_CONTROLLER_OPEN_UNDO_BRACKET,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_title,
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
                crate::bindings::rig_vm_developer::U_RIG_VM_CONTROLLER_OPEN_UNDO_BRACKET,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<bool>().read() }
    }
    pub fn mark_function_as_public(
        &mut self,
        in_function_name: &FName,
        b_in_is_public: bool,
        b_setup_undo_redo: bool,
        b_print_python_command: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_CONTROLLER_MARK_FUNCTION_AS_PUBLIC,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_function_name,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_in_is_public,
                __buffer.add(12).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_setup_undo_redo,
                __buffer.add(13).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_print_python_command,
                __buffer.add(14).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_CONTROLLER_MARK_FUNCTION_AS_PUBLIC,
                __buffer,
            )
        };
        unsafe { __buffer.add(15).cast::<bool>().read() }
    }
    pub fn make_variable_node_from_binding(
        &mut self,
        in_pin_path: FString,
        in_node_position: &crate::bindings::core_u_object::FVector2D,
        b_setup_undo_redo: bool,
        b_print_python_command: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<35>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_CONTROLLER_MAKE_VARIABLE_NODE_FROM_BINDING,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_pin_path,
                __buffer.add(0).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_node_position,
                __buffer.add(16).cast::<crate::bindings::core_u_object::FVector2D>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_setup_undo_redo,
                __buffer.add(32).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_print_python_command,
                __buffer.add(33).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_CONTROLLER_MAKE_VARIABLE_NODE_FROM_BINDING,
                __buffer,
            )
        };
        unsafe { __buffer.add(34).cast::<bool>().read() }
    }
    pub fn make_options_for_workflow(
        &mut self,
        in_subject: UPtr<crate::bindings::core_u_object::UObject>,
        in_workflow: &crate::bindings::rig_vm::FRigVMUserWorkflow,
    ) -> UPtr<crate::bindings::rig_vm::URigVMUserWorkflowOptions> {
        let mut __stack = crate::core_data::StackAlloc::<128>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_CONTROLLER_MAKE_OPTIONS_FOR_WORKFLOW,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_subject,
                __buffer.add(0).cast::<UPtr<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_workflow,
                __buffer.add(8).cast::<crate::bindings::rig_vm::FRigVMUserWorkflow>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_CONTROLLER_MAKE_OPTIONS_FOR_WORKFLOW,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(120)
                .cast::<UPtr<crate::bindings::rig_vm::URigVMUserWorkflowOptions>>()
                .read()
        }
    }
    pub fn make_bindings_from_variable_node(
        &mut self,
        in_node_name: &FName,
        b_setup_undo_redo: bool,
        b_print_python_command: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<15>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_CONTROLLER_MAKE_BINDINGS_FROM_VARIABLE_NODE,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_node_name,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_setup_undo_redo,
                __buffer.add(12).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_print_python_command,
                __buffer.add(13).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_CONTROLLER_MAKE_BINDINGS_FROM_VARIABLE_NODE,
                __buffer,
            )
        };
        unsafe { __buffer.add(14).cast::<bool>().read() }
    }
    pub fn localize_functions(
        &mut self,
        in_function_definitions: TArray<
            crate::bindings::rig_vm::FRigVMGraphFunctionIdentifier,
        >,
        b_localize_dependent_private_functions: bool,
        b_setup_undo_redo: bool,
        b_print_python_command: bool,
    ) -> TMap<
        crate::bindings::rig_vm::FRigVMGraphFunctionIdentifier,
        UPtr<URigVMLibraryNode>,
    > {
        let mut __stack = crate::core_data::StackAlloc::<104>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_CONTROLLER_LOCALIZE_FUNCTIONS,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_function_definitions,
                __buffer
                    .add(0)
                    .cast::<
                        TArray<crate::bindings::rig_vm::FRigVMGraphFunctionIdentifier>,
                    >(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_localize_dependent_private_functions,
                __buffer.add(16).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_setup_undo_redo,
                __buffer.add(17).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_print_python_command,
                __buffer.add(18).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_CONTROLLER_LOCALIZE_FUNCTIONS,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(24)
                .cast::<
                    TMap<
                        crate::bindings::rig_vm::FRigVMGraphFunctionIdentifier,
                        UPtr<URigVMLibraryNode>,
                    >,
                >()
                .read()
        }
    }
    pub fn localize_function_from_path(
        &mut self,
        in_host_path: FString,
        in_function_name: &FName,
        b_localize_dependent_private_functions: bool,
        b_setup_undo_redo: bool,
        b_print_python_command: bool,
    ) -> UPtr<URigVMLibraryNode> {
        let mut __stack = crate::core_data::StackAlloc::<40>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_CONTROLLER_LOCALIZE_FUNCTION_FROM_PATH,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_host_path,
                __buffer.add(0).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_function_name,
                __buffer.add(16).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_localize_dependent_private_functions,
                __buffer.add(28).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_setup_undo_redo,
                __buffer.add(29).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_print_python_command,
                __buffer.add(30).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_CONTROLLER_LOCALIZE_FUNCTION_FROM_PATH,
                __buffer,
            )
        };
        unsafe { __buffer.add(32).cast::<UPtr<URigVMLibraryNode>>().read() }
    }
    pub fn localize_function(
        &mut self,
        in_function_definition: &crate::bindings::rig_vm::FRigVMGraphFunctionIdentifier,
        b_localize_dependent_private_functions: bool,
        b_setup_undo_redo: bool,
        b_print_python_command: bool,
    ) -> UPtr<URigVMLibraryNode> {
        let mut __stack = crate::core_data::StackAlloc::<112>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_CONTROLLER_LOCALIZE_FUNCTION,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_function_definition,
                __buffer
                    .add(0)
                    .cast::<crate::bindings::rig_vm::FRigVMGraphFunctionIdentifier>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_localize_dependent_private_functions,
                __buffer.add(96).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_setup_undo_redo,
                __buffer.add(97).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_print_python_command,
                __buffer.add(98).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_CONTROLLER_LOCALIZE_FUNCTION,
                __buffer,
            )
        };
        unsafe { __buffer.add(104).cast::<UPtr<URigVMLibraryNode>>().read() }
    }
    pub fn join_function_variant(
        &mut self,
        in_function_name: &FName,
        in_guid: &crate::bindings::core_u_object::FGuid,
        b_setup_undo_redo: bool,
        b_print_python_command: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<31>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_CONTROLLER_JOIN_FUNCTION_VARIANT,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_function_name,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_guid,
                __buffer.add(12).cast::<crate::bindings::core_u_object::FGuid>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_setup_undo_redo,
                __buffer.add(28).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_print_python_command,
                __buffer.add(29).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_CONTROLLER_JOIN_FUNCTION_VARIANT,
                __buffer,
            )
        };
        unsafe { __buffer.add(30).cast::<bool>().read() }
    }
    pub fn is_transacting(&self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_CONTROLLER_IS_TRANSACTING,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_CONTROLLER_IS_TRANSACTING,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn is_reporting_enabled(&self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_CONTROLLER_IS_REPORTING_ENABLED,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_CONTROLLER_IS_REPORTING_ENABLED,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn is_function_public(&mut self, in_function_name: &FName) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<13>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_CONTROLLER_IS_FUNCTION_PUBLIC,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_function_name,
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
                crate::bindings::rig_vm_developer::U_RIG_VM_CONTROLLER_IS_FUNCTION_PUBLIC,
                __buffer,
            )
        };
        unsafe { __buffer.add(12).cast::<bool>().read() }
    }
    pub fn insert_array_pin(
        &mut self,
        in_array_pin_path: FString,
        in_index: i32,
        in_default_value: FString,
        b_setup_undo_redo: bool,
        b_print_python_command: bool,
    ) -> FString {
        let mut __stack = crate::core_data::StackAlloc::<64>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_CONTROLLER_INSERT_ARRAY_PIN,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_array_pin_path,
                __buffer.add(0).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&in_index, __buffer.add(16).cast::<i32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_default_value,
                __buffer.add(24).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_setup_undo_redo,
                __buffer.add(40).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_print_python_command,
                __buffer.add(41).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_CONTROLLER_INSERT_ARRAY_PIN,
                __buffer,
            )
        };
        unsafe { __buffer.add(48).cast::<FString>().read() }
    }
    pub fn import_nodes_from_text(
        &mut self,
        in_text: FString,
        b_setup_undo_redo: bool,
        b_print_python_commands: bool,
    ) -> TArray<FName> {
        let mut __stack = crate::core_data::StackAlloc::<40>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_CONTROLLER_IMPORT_NODES_FROM_TEXT,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_text,
                __buffer.add(0).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_setup_undo_redo,
                __buffer.add(16).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_print_python_commands,
                __buffer.add(17).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_CONTROLLER_IMPORT_NODES_FROM_TEXT,
                __buffer,
            )
        };
        unsafe { __buffer.add(24).cast::<TArray<FName>>().read() }
    }
    pub fn get_unit_structs_for_template(
        in_notation: &FName,
    ) -> TArray<UPtr<crate::bindings::core_u_object::UScriptStruct>> {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_CONTROLLER_GET_UNIT_STRUCTS_FOR_TEMPLATE,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_notation,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::rig_vm_developer::URigVMController::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_CONTROLLER_GET_UNIT_STRUCTS_FOR_TEMPLATE,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(16)
                .cast::<TArray<UPtr<crate::bindings::core_u_object::UScriptStruct>>>()
                .read()
        }
    }
    pub fn get_top_level_graph(&self) -> UPtr<URigVMGraph> {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_CONTROLLER_GET_TOP_LEVEL_GRAPH,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_CONTROLLER_GET_TOP_LEVEL_GRAPH,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<UPtr<URigVMGraph>>().read() }
    }
    pub fn get_template_for_unit_struct(
        in_function: UPtr<crate::bindings::core_u_object::UScriptStruct>,
        in_method_name: FString,
    ) -> FString {
        let mut __stack = crate::core_data::StackAlloc::<40>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_CONTROLLER_GET_TEMPLATE_FOR_UNIT_STRUCT,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_function,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::core_u_object::UScriptStruct>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_method_name,
                __buffer.add(8).cast::<FString>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::rig_vm_developer::URigVMController::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_CONTROLLER_GET_TEMPLATE_FOR_UNIT_STRUCT,
                __buffer,
            )
        };
        unsafe { __buffer.add(24).cast::<FString>().read() }
    }
    pub fn get_schema(&self) -> UPtr<URigVMSchema> {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_CONTROLLER_GET_SCHEMA,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_CONTROLLER_GET_SCHEMA,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<UPtr<URigVMSchema>>().read() }
    }
    pub fn get_registered_unit_structs() -> TArray<
        UPtr<crate::bindings::core_u_object::UScriptStruct>,
    > {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_CONTROLLER_GET_REGISTERED_UNIT_STRUCTS,
                __buffer,
            )
        };
        let __object_ptr = crate::bindings::rig_vm_developer::URigVMController::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_CONTROLLER_GET_REGISTERED_UNIT_STRUCTS,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(0)
                .cast::<TArray<UPtr<crate::bindings::core_u_object::UScriptStruct>>>()
                .read()
        }
    }
    pub fn get_registered_templates() -> TArray<FString> {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_CONTROLLER_GET_REGISTERED_TEMPLATES,
                __buffer,
            )
        };
        let __object_ptr = crate::bindings::rig_vm_developer::URigVMController::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_CONTROLLER_GET_REGISTERED_TEMPLATES,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<TArray<FString>>().read() }
    }
    pub fn get_pin_default_value(&mut self, in_pin_path: FString) -> FString {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_CONTROLLER_GET_PIN_DEFAULT_VALUE,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_pin_path,
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
                crate::bindings::rig_vm_developer::U_RIG_VM_CONTROLLER_GET_PIN_DEFAULT_VALUE,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<FString>().read() }
    }
    pub fn get_graph(&self) -> UPtr<URigVMGraph> {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_CONTROLLER_GET_GRAPH,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_CONTROLLER_GET_GRAPH,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<UPtr<URigVMGraph>>().read() }
    }
    pub fn get_controller_for_graph(
        &self,
        in_graph: UPtr<URigVMGraph>,
    ) -> UPtr<URigVMController> {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_CONTROLLER_GET_CONTROLLER_FOR_GRAPH,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_graph,
                __buffer.add(0).cast::<UPtr<URigVMGraph>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_CONTROLLER_GET_CONTROLLER_FOR_GRAPH,
                __buffer,
            )
        };
        unsafe { __buffer.add(8).cast::<UPtr<URigVMController>>().read() }
    }
    pub fn get_action_stack(&self) -> UPtr<URigVMActionStack> {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_CONTROLLER_GET_ACTION_STACK,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_CONTROLLER_GET_ACTION_STACK,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<UPtr<URigVMActionStack>>().read() }
    }
    pub fn generate_python_commands(&mut self) -> TArray<FString> {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_CONTROLLER_GENERATE_PYTHON_COMMANDS,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_CONTROLLER_GENERATE_PYTHON_COMMANDS,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<TArray<FString>>().read() }
    }
    pub fn find_variants_of_function(
        &mut self,
        in_function_name: &FName,
    ) -> TArray<crate::bindings::rig_vm::FRigVMVariantRef> {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_CONTROLLER_FIND_VARIANTS_OF_FUNCTION,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_function_name,
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
                crate::bindings::rig_vm_developer::U_RIG_VM_CONTROLLER_FIND_VARIANTS_OF_FUNCTION,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(16)
                .cast::<TArray<crate::bindings::rig_vm::FRigVMVariantRef>>()
                .read()
        }
    }
    pub fn find_graph_function_identifier(
        &self,
        in_host_path: FString,
        in_function_name: FName,
    ) -> crate::bindings::rig_vm::FRigVMGraphFunctionIdentifier {
        let mut __stack = crate::core_data::StackAlloc::<128>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_CONTROLLER_FIND_GRAPH_FUNCTION_IDENTIFIER,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_host_path,
                __buffer.add(0).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_function_name,
                __buffer.add(16).cast::<FName>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_CONTROLLER_FIND_GRAPH_FUNCTION_IDENTIFIER,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(32)
                .cast::<crate::bindings::rig_vm::FRigVMGraphFunctionIdentifier>()
                .read()
        }
    }
    pub fn find_graph_function_header_by_name(
        &self,
        in_host_path: FString,
        in_function_name: FName,
    ) -> crate::bindings::rig_vm::FRigVMGraphFunctionHeader {
        let mut __stack = crate::core_data::StackAlloc::<560>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_CONTROLLER_FIND_GRAPH_FUNCTION_HEADER_BY_NAME,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_host_path,
                __buffer.add(0).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_function_name,
                __buffer.add(16).cast::<FName>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_CONTROLLER_FIND_GRAPH_FUNCTION_HEADER_BY_NAME,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(32)
                .cast::<crate::bindings::rig_vm::FRigVMGraphFunctionHeader>()
                .read()
        }
    }
    pub fn find_graph_function_header(
        &self,
        in_function_identifier: crate::bindings::rig_vm::FRigVMGraphFunctionIdentifier,
    ) -> crate::bindings::rig_vm::FRigVMGraphFunctionHeader {
        let mut __stack = crate::core_data::StackAlloc::<624>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_CONTROLLER_FIND_GRAPH_FUNCTION_HEADER,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_function_identifier,
                __buffer
                    .add(0)
                    .cast::<crate::bindings::rig_vm::FRigVMGraphFunctionIdentifier>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_CONTROLLER_FIND_GRAPH_FUNCTION_HEADER,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(96)
                .cast::<crate::bindings::rig_vm::FRigVMGraphFunctionHeader>()
                .read()
        }
    }
    pub fn export_selected_nodes_to_text(
        &mut self,
        b_include_exterior_links: bool,
    ) -> FString {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_CONTROLLER_EXPORT_SELECTED_NODES_TO_TEXT,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_include_exterior_links,
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
                crate::bindings::rig_vm_developer::U_RIG_VM_CONTROLLER_EXPORT_SELECTED_NODES_TO_TEXT,
                __buffer,
            )
        };
        unsafe { __buffer.add(8).cast::<FString>().read() }
    }
    pub fn export_node_to_text(
        &mut self,
        in_node: UPtr<URigVMNode>,
        b_include_exterior_links: bool,
    ) -> FString {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_CONTROLLER_EXPORT_NODE_TO_TEXT,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_node,
                __buffer.add(0).cast::<UPtr<URigVMNode>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_include_exterior_links,
                __buffer.add(8).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_CONTROLLER_EXPORT_NODE_TO_TEXT,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<FString>().read() }
    }
    pub fn export_nodes_to_text(
        &mut self,
        in_node_names: &TArray<FName>,
        b_include_exterior_links: bool,
    ) -> FString {
        let mut __stack = crate::core_data::StackAlloc::<40>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_CONTROLLER_EXPORT_NODES_TO_TEXT,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_node_names,
                __buffer.add(0).cast::<TArray<FName>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_include_exterior_links,
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
                crate::bindings::rig_vm_developer::U_RIG_VM_CONTROLLER_EXPORT_NODES_TO_TEXT,
                __buffer,
            )
        };
        unsafe { __buffer.add(24).cast::<FString>().read() }
    }
    pub fn expand_library_node(
        &mut self,
        in_node_name: &FName,
        b_setup_undo_redo: bool,
        b_print_python_command: bool,
    ) -> TArray<UPtr<URigVMNode>> {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_CONTROLLER_EXPAND_LIBRARY_NODE,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_node_name,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_setup_undo_redo,
                __buffer.add(12).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_print_python_command,
                __buffer.add(13).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_CONTROLLER_EXPAND_LIBRARY_NODE,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<TArray<UPtr<URigVMNode>>>().read() }
    }
    pub fn enable_reporting(&mut self, b_enabled: bool) {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_CONTROLLER_ENABLE_REPORTING,
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
                crate::bindings::rig_vm_developer::U_RIG_VM_CONTROLLER_ENABLE_REPORTING,
                __buffer,
            )
        };
    }
    pub fn eject_node_from_pin(
        &mut self,
        in_pin_path: FString,
        b_setup_undo_redo: bool,
        b_print_python_command: bool,
    ) -> UPtr<URigVMNode> {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_CONTROLLER_EJECT_NODE_FROM_PIN,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_pin_path,
                __buffer.add(0).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_setup_undo_redo,
                __buffer.add(16).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_print_python_command,
                __buffer.add(17).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_CONTROLLER_EJECT_NODE_FROM_PIN,
                __buffer,
            )
        };
        unsafe { __buffer.add(24).cast::<UPtr<URigVMNode>>().read() }
    }
    pub fn duplicate_array_pin(
        &mut self,
        in_array_element_pin_path: FString,
        b_setup_undo_redo: bool,
        b_print_python_command: bool,
    ) -> FString {
        let mut __stack = crate::core_data::StackAlloc::<40>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_CONTROLLER_DUPLICATE_ARRAY_PIN,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_array_element_pin_path,
                __buffer.add(0).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_setup_undo_redo,
                __buffer.add(16).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_print_python_command,
                __buffer.add(17).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_CONTROLLER_DUPLICATE_ARRAY_PIN,
                __buffer,
            )
        };
        unsafe { __buffer.add(24).cast::<FString>().read() }
    }
    pub fn create_function_variant(
        &mut self,
        in_function_name: &FName,
        in_variant_name: &FName,
        b_setup_undo_redo: bool,
        b_print_python_command: bool,
    ) -> UPtr<URigVMLibraryNode> {
        let mut __stack = crate::core_data::StackAlloc::<40>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_CONTROLLER_CREATE_FUNCTION_VARIANT,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_function_name,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_variant_name,
                __buffer.add(12).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_setup_undo_redo,
                __buffer.add(24).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_print_python_command,
                __buffer.add(25).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_CONTROLLER_CREATE_FUNCTION_VARIANT,
                __buffer,
            )
        };
        unsafe { __buffer.add(32).cast::<UPtr<URigVMLibraryNode>>().read() }
    }
    pub fn collapse_nodes(
        &mut self,
        in_node_names: &TArray<FName>,
        in_collapse_node_name: FString,
        b_setup_undo_redo: bool,
        b_print_python_command: bool,
        b_is_aggregate: bool,
    ) -> UPtr<URigVMCollapseNode> {
        let mut __stack = crate::core_data::StackAlloc::<48>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_CONTROLLER_COLLAPSE_NODES,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_node_names,
                __buffer.add(0).cast::<TArray<FName>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_collapse_node_name,
                __buffer.add(16).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_setup_undo_redo,
                __buffer.add(32).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_print_python_command,
                __buffer.add(33).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_is_aggregate,
                __buffer.add(34).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_CONTROLLER_COLLAPSE_NODES,
                __buffer,
            )
        };
        unsafe { __buffer.add(40).cast::<UPtr<URigVMCollapseNode>>().read() }
    }
    pub fn close_undo_bracket(&mut self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_CONTROLLER_CLOSE_UNDO_BRACKET,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_CONTROLLER_CLOSE_UNDO_BRACKET,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn clear_pin_category(
        &mut self,
        in_pin_path: FString,
        b_setup_undo_redo: bool,
        b_print_python_command: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<19>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_CONTROLLER_CLEAR_PIN_CATEGORY,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_pin_path,
                __buffer.add(0).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_setup_undo_redo,
                __buffer.add(16).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_print_python_command,
                __buffer.add(17).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_CONTROLLER_CLEAR_PIN_CATEGORY,
                __buffer,
            )
        };
        unsafe { __buffer.add(18).cast::<bool>().read() }
    }
    pub fn clear_override_on_pins(
        &mut self,
        in_pin_paths: &TArray<FString>,
        b_setup_undo: bool,
        b_print_python_command: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<19>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_CONTROLLER_CLEAR_OVERRIDE_ON_PINS,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_pin_paths,
                __buffer.add(0).cast::<TArray<FString>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_setup_undo,
                __buffer.add(16).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_print_python_command,
                __buffer.add(17).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_CONTROLLER_CLEAR_OVERRIDE_ON_PINS,
                __buffer,
            )
        };
        unsafe { __buffer.add(18).cast::<bool>().read() }
    }
    pub fn clear_override_on_pin(
        &mut self,
        in_pin_path: FString,
        b_setup_undo: bool,
        b_print_python_command: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<19>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_CONTROLLER_CLEAR_OVERRIDE_ON_PIN,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_pin_path,
                __buffer.add(0).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_setup_undo,
                __buffer.add(16).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_print_python_command,
                __buffer.add(17).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_CONTROLLER_CLEAR_OVERRIDE_ON_PIN,
                __buffer,
            )
        };
        unsafe { __buffer.add(18).cast::<bool>().read() }
    }
    pub fn clear_override_on_all_pins_on_nodes(
        &mut self,
        in_node_names: &TArray<FName>,
        b_setup_undo: bool,
        b_print_python_command: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<19>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_CONTROLLER_CLEAR_OVERRIDE_ON_ALL_PINS_ON_NODES,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_node_names,
                __buffer.add(0).cast::<TArray<FName>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_setup_undo,
                __buffer.add(16).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_print_python_command,
                __buffer.add(17).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_CONTROLLER_CLEAR_OVERRIDE_ON_ALL_PINS_ON_NODES,
                __buffer,
            )
        };
        unsafe { __buffer.add(18).cast::<bool>().read() }
    }
    pub fn clear_override_on_all_pins_on_node(
        &mut self,
        in_node_name: &FName,
        b_setup_undo: bool,
        b_print_python_command: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<15>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_CONTROLLER_CLEAR_OVERRIDE_ON_ALL_PINS_ON_NODE,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_node_name,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_setup_undo,
                __buffer.add(12).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_print_python_command,
                __buffer.add(13).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_CONTROLLER_CLEAR_OVERRIDE_ON_ALL_PINS_ON_NODE,
                __buffer,
            )
        };
        unsafe { __buffer.add(14).cast::<bool>().read() }
    }
    pub fn clear_node_selection(
        &mut self,
        b_setup_undo_redo: bool,
        b_print_python_command: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<3>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_CONTROLLER_CLEAR_NODE_SELECTION,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_setup_undo_redo,
                __buffer.add(0).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_print_python_command,
                __buffer.add(1).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_CONTROLLER_CLEAR_NODE_SELECTION,
                __buffer,
            )
        };
        unsafe { __buffer.add(2).cast::<bool>().read() }
    }
    pub fn clear_node_layout(
        &mut self,
        in_node_name: &FName,
        b_setup_undo_redo: bool,
        b_print_python_command: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<15>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_CONTROLLER_CLEAR_NODE_LAYOUT,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_node_name,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_setup_undo_redo,
                __buffer.add(12).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_print_python_command,
                __buffer.add(13).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_CONTROLLER_CLEAR_NODE_LAYOUT,
                __buffer,
            )
        };
        unsafe { __buffer.add(14).cast::<bool>().read() }
    }
    pub fn clear_array_pin(
        &mut self,
        in_array_pin_path: FString,
        b_setup_undo_redo: bool,
        b_print_python_command: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<19>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_CONTROLLER_CLEAR_ARRAY_PIN,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_array_pin_path,
                __buffer.add(0).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_setup_undo_redo,
                __buffer.add(16).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_print_python_command,
                __buffer.add(17).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_CONTROLLER_CLEAR_ARRAY_PIN,
                __buffer,
            )
        };
        unsafe { __buffer.add(18).cast::<bool>().read() }
    }
    pub fn change_exposed_pin_type(
        &mut self,
        in_pin_name: &FName,
        in_cpp_type: FString,
        in_cpp_type_object_path: &FName,
        b_setup_undo_redo: &mut bool,
        b_setup_orphan_pins: bool,
        b_print_python_command: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<48>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_CONTROLLER_CHANGE_EXPOSED_PIN_TYPE,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_pin_name,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_cpp_type,
                __buffer.add(16).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_cpp_type_object_path,
                __buffer.add(32).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                b_setup_undo_redo,
                __buffer.add(44).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_setup_orphan_pins,
                __buffer.add(45).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_print_python_command,
                __buffer.add(46).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_CONTROLLER_CHANGE_EXPOSED_PIN_TYPE,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(44).cast::<bool>().swap(b_setup_undo_redo);
        }
        unsafe { __buffer.add(47).cast::<bool>().read() }
    }
    pub fn can_import_nodes_from_text(&mut self, in_text: FString) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<17>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_CONTROLLER_CAN_IMPORT_NODES_FROM_TEXT,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_text,
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
                crate::bindings::rig_vm_developer::U_RIG_VM_CONTROLLER_CAN_IMPORT_NODES_FROM_TEXT,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<bool>().read() }
    }
    pub fn cancel_undo_bracket(&mut self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_CONTROLLER_CANCEL_UNDO_BRACKET,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_CONTROLLER_CANCEL_UNDO_BRACKET,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn break_link(
        &mut self,
        in_output_pin_path: FString,
        in_input_pin_path: FString,
        b_setup_undo_redo: bool,
        b_print_python_command: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<35>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_CONTROLLER_BREAK_LINK,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_output_pin_path,
                __buffer.add(0).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_input_pin_path,
                __buffer.add(16).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_setup_undo_redo,
                __buffer.add(32).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_print_python_command,
                __buffer.add(33).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_CONTROLLER_BREAK_LINK,
                __buffer,
            )
        };
        unsafe { __buffer.add(34).cast::<bool>().read() }
    }
    pub fn break_all_links(
        &mut self,
        in_pin_path: FString,
        b_as_input: bool,
        b_setup_undo_redo: bool,
        b_print_python_command: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<20>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_CONTROLLER_BREAK_ALL_LINKS,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_pin_path,
                __buffer.add(0).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_as_input,
                __buffer.add(16).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_setup_undo_redo,
                __buffer.add(17).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_print_python_command,
                __buffer.add(18).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_CONTROLLER_BREAK_ALL_LINKS,
                __buffer,
            )
        };
        unsafe { __buffer.add(19).cast::<bool>().read() }
    }
    pub fn bind_pin_to_variable(
        &mut self,
        in_pin_path: FString,
        in_new_bound_variable_path: FString,
        b_setup_undo_redo: bool,
        b_print_python_command: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<35>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_CONTROLLER_BIND_PIN_TO_VARIABLE,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_pin_path,
                __buffer.add(0).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_new_bound_variable_path,
                __buffer.add(16).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_setup_undo_redo,
                __buffer.add(32).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_print_python_command,
                __buffer.add(33).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_CONTROLLER_BIND_PIN_TO_VARIABLE,
                __buffer,
            )
        };
        unsafe { __buffer.add(34).cast::<bool>().read() }
    }
    pub fn add_variable_node_from_object_path(
        &mut self,
        in_variable_name: &FName,
        in_cpp_type: FString,
        in_cpp_type_object_path: FString,
        b_is_getter: bool,
        in_default_value: FString,
        in_position: &crate::bindings::core_u_object::FVector2D,
        in_node_name: FString,
        b_setup_undo_redo: bool,
        b_print_python_command: bool,
    ) -> UPtr<URigVMVariableNode> {
        let mut __stack = crate::core_data::StackAlloc::<120>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_CONTROLLER_ADD_VARIABLE_NODE_FROM_OBJECT_PATH,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_variable_name,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_cpp_type,
                __buffer.add(16).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_cpp_type_object_path,
                __buffer.add(32).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_is_getter,
                __buffer.add(48).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_default_value,
                __buffer.add(56).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_position,
                __buffer.add(72).cast::<crate::bindings::core_u_object::FVector2D>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_node_name,
                __buffer.add(88).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_setup_undo_redo,
                __buffer.add(104).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_print_python_command,
                __buffer.add(105).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_CONTROLLER_ADD_VARIABLE_NODE_FROM_OBJECT_PATH,
                __buffer,
            )
        };
        unsafe { __buffer.add(112).cast::<UPtr<URigVMVariableNode>>().read() }
    }
    pub fn add_variable_node(
        &mut self,
        in_variable_name: &FName,
        in_cpp_type: FString,
        in_cpp_type_object: UPtr<crate::bindings::core_u_object::UObject>,
        b_is_getter: bool,
        in_default_value: FString,
        in_position: &crate::bindings::core_u_object::FVector2D,
        in_node_name: FString,
        b_setup_undo_redo: bool,
        b_print_python_command: bool,
    ) -> UPtr<URigVMVariableNode> {
        let mut __stack = crate::core_data::StackAlloc::<112>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_CONTROLLER_ADD_VARIABLE_NODE,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_variable_name,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_cpp_type,
                __buffer.add(16).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_cpp_type_object,
                __buffer.add(32).cast::<UPtr<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_is_getter,
                __buffer.add(40).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_default_value,
                __buffer.add(48).cast::<FString>(),
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
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_node_name,
                __buffer.add(80).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_setup_undo_redo,
                __buffer.add(96).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_print_python_command,
                __buffer.add(97).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_CONTROLLER_ADD_VARIABLE_NODE,
                __buffer,
            )
        };
        unsafe { __buffer.add(104).cast::<UPtr<URigVMVariableNode>>().read() }
    }
    pub fn add_unit_node_with_defaults(
        &mut self,
        in_script_struct: UPtr<crate::bindings::core_u_object::UScriptStruct>,
        in_defaults: FString,
        in_method_name: &FName,
        in_position: &crate::bindings::core_u_object::FVector2D,
        in_node_name: FString,
        b_setup_undo_redo: bool,
        b_print_python_command: bool,
    ) -> UPtr<URigVMUnitNode> {
        let mut __stack = crate::core_data::StackAlloc::<88>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_CONTROLLER_ADD_UNIT_NODE_WITH_DEFAULTS,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_script_struct,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::core_u_object::UScriptStruct>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_defaults,
                __buffer.add(8).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_method_name,
                __buffer.add(24).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_position,
                __buffer.add(40).cast::<crate::bindings::core_u_object::FVector2D>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_node_name,
                __buffer.add(56).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_setup_undo_redo,
                __buffer.add(72).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_print_python_command,
                __buffer.add(73).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_CONTROLLER_ADD_UNIT_NODE_WITH_DEFAULTS,
                __buffer,
            )
        };
        unsafe { __buffer.add(80).cast::<UPtr<URigVMUnitNode>>().read() }
    }
    pub fn add_unit_node_from_struct_path(
        &mut self,
        in_script_struct_path: FString,
        in_method_name: &FName,
        in_position: &crate::bindings::core_u_object::FVector2D,
        in_node_name: FString,
        b_setup_undo_redo: bool,
        b_print_python_command: bool,
    ) -> UPtr<URigVMUnitNode> {
        let mut __stack = crate::core_data::StackAlloc::<80>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_CONTROLLER_ADD_UNIT_NODE_FROM_STRUCT_PATH,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_script_struct_path,
                __buffer.add(0).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_method_name,
                __buffer.add(16).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_position,
                __buffer.add(32).cast::<crate::bindings::core_u_object::FVector2D>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_node_name,
                __buffer.add(48).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_setup_undo_redo,
                __buffer.add(64).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_print_python_command,
                __buffer.add(65).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_CONTROLLER_ADD_UNIT_NODE_FROM_STRUCT_PATH,
                __buffer,
            )
        };
        unsafe { __buffer.add(72).cast::<UPtr<URigVMUnitNode>>().read() }
    }
    pub fn add_unit_node(
        &mut self,
        in_script_struct: UPtr<crate::bindings::core_u_object::UScriptStruct>,
        in_method_name: &FName,
        in_position: &crate::bindings::core_u_object::FVector2D,
        in_node_name: FString,
        b_setup_undo_redo: bool,
        b_print_python_command: bool,
    ) -> UPtr<URigVMUnitNode> {
        let mut __stack = crate::core_data::StackAlloc::<72>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_CONTROLLER_ADD_UNIT_NODE,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_script_struct,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::core_u_object::UScriptStruct>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_method_name,
                __buffer.add(8).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_position,
                __buffer.add(24).cast::<crate::bindings::core_u_object::FVector2D>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_node_name,
                __buffer.add(40).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_setup_undo_redo,
                __buffer.add(56).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_print_python_command,
                __buffer.add(57).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_CONTROLLER_ADD_UNIT_NODE,
                __buffer,
            )
        };
        unsafe { __buffer.add(64).cast::<UPtr<URigVMUnitNode>>().read() }
    }
    pub fn add_trait(
        &mut self,
        in_node_name: &FName,
        in_trait_type_object_path: &FName,
        in_trait_name: &FName,
        in_default_value: FString,
        in_pin_index: i32,
        b_setup_undo_redo: bool,
        b_print_python_command: bool,
    ) -> FName {
        let mut __stack = crate::core_data::StackAlloc::<76>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_CONTROLLER_ADD_TRAIT,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_node_name,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_trait_type_object_path,
                __buffer.add(12).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_trait_name,
                __buffer.add(24).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_default_value,
                __buffer.add(40).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_pin_index,
                __buffer.add(56).cast::<i32>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_setup_undo_redo,
                __buffer.add(60).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_print_python_command,
                __buffer.add(61).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_CONTROLLER_ADD_TRAIT,
                __buffer,
            )
        };
        unsafe { __buffer.add(64).cast::<FName>().read() }
    }
    pub fn add_template_node(
        &mut self,
        in_notation: &FName,
        in_position: &crate::bindings::core_u_object::FVector2D,
        in_node_name: FString,
        b_setup_undo_redo: bool,
        b_print_python_command: bool,
    ) -> UPtr<URigVMTemplateNode> {
        let mut __stack = crate::core_data::StackAlloc::<64>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_CONTROLLER_ADD_TEMPLATE_NODE,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_notation,
                __buffer.add(0).cast::<FName>(),
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
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_node_name,
                __buffer.add(32).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_setup_undo_redo,
                __buffer.add(48).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_print_python_command,
                __buffer.add(49).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_CONTROLLER_ADD_TEMPLATE_NODE,
                __buffer,
            )
        };
        unsafe { __buffer.add(56).cast::<UPtr<URigVMTemplateNode>>().read() }
    }
    pub fn add_tag_to_function_variant(
        &mut self,
        in_function_name: &FName,
        in_tag: &crate::bindings::rig_vm::FRigVMTag,
        b_setup_undo_redo: bool,
        b_print_python_command: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<91>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_CONTROLLER_ADD_TAG_TO_FUNCTION_VARIANT,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_function_name,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_tag,
                __buffer.add(16).cast::<crate::bindings::rig_vm::FRigVMTag>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_setup_undo_redo,
                __buffer.add(88).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_print_python_command,
                __buffer.add(89).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_CONTROLLER_ADD_TAG_TO_FUNCTION_VARIANT,
                __buffer,
            )
        };
        unsafe { __buffer.add(90).cast::<bool>().read() }
    }
    pub fn add_select_node_from_struct(
        &mut self,
        in_script_struct: UPtr<crate::bindings::core_u_object::UScriptStruct>,
        in_position: &crate::bindings::core_u_object::FVector2D,
        in_node_name: FString,
        b_setup_undo_redo: bool,
    ) -> UPtr<URigVMNode> {
        let mut __stack = crate::core_data::StackAlloc::<56>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_CONTROLLER_ADD_SELECT_NODE_FROM_STRUCT,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_script_struct,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::core_u_object::UScriptStruct>>(),
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
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_node_name,
                __buffer.add(24).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_setup_undo_redo,
                __buffer.add(40).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_CONTROLLER_ADD_SELECT_NODE_FROM_STRUCT,
                __buffer,
            )
        };
        unsafe { __buffer.add(48).cast::<UPtr<URigVMNode>>().read() }
    }
    pub fn add_select_node(
        &mut self,
        in_cpp_type: FString,
        in_cpp_type_object_path: &FName,
        in_position: &crate::bindings::core_u_object::FVector2D,
        in_node_name: FString,
        b_setup_undo_redo: bool,
        b_print_python_command: bool,
    ) -> UPtr<URigVMNode> {
        let mut __stack = crate::core_data::StackAlloc::<80>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_CONTROLLER_ADD_SELECT_NODE,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_cpp_type,
                __buffer.add(0).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_cpp_type_object_path,
                __buffer.add(16).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_position,
                __buffer.add(32).cast::<crate::bindings::core_u_object::FVector2D>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_node_name,
                __buffer.add(48).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_setup_undo_redo,
                __buffer.add(64).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_print_python_command,
                __buffer.add(65).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_CONTROLLER_ADD_SELECT_NODE,
                __buffer,
            )
        };
        unsafe { __buffer.add(72).cast::<UPtr<URigVMNode>>().read() }
    }
    pub fn add_reroute_node_on_pin(
        &mut self,
        in_pin_path: FString,
        b_as_input: bool,
        in_position: &crate::bindings::core_u_object::FVector2D,
        in_node_name: FString,
        b_setup_undo_redo: bool,
        b_print_python_command: bool,
    ) -> UPtr<URigVMRerouteNode> {
        let mut __stack = crate::core_data::StackAlloc::<72>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_CONTROLLER_ADD_REROUTE_NODE_ON_PIN,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_pin_path,
                __buffer.add(0).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_as_input,
                __buffer.add(16).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_position,
                __buffer.add(24).cast::<crate::bindings::core_u_object::FVector2D>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_node_name,
                __buffer.add(40).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_setup_undo_redo,
                __buffer.add(56).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_print_python_command,
                __buffer.add(57).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_CONTROLLER_ADD_REROUTE_NODE_ON_PIN,
                __buffer,
            )
        };
        unsafe { __buffer.add(64).cast::<UPtr<URigVMRerouteNode>>().read() }
    }
    pub fn add_reroute_node_on_link_path(
        &mut self,
        in_link_pin_path_representation: FString,
        in_position: &crate::bindings::core_u_object::FVector2D,
        in_node_name: FString,
        b_setup_undo_redo: bool,
        b_print_python_command: bool,
    ) -> UPtr<URigVMRerouteNode> {
        let mut __stack = crate::core_data::StackAlloc::<64>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_CONTROLLER_ADD_REROUTE_NODE_ON_LINK_PATH,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_link_pin_path_representation,
                __buffer.add(0).cast::<FString>(),
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
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_node_name,
                __buffer.add(32).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_setup_undo_redo,
                __buffer.add(48).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_print_python_command,
                __buffer.add(49).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_CONTROLLER_ADD_REROUTE_NODE_ON_LINK_PATH,
                __buffer,
            )
        };
        unsafe { __buffer.add(56).cast::<UPtr<URigVMRerouteNode>>().read() }
    }
    pub fn add_reroute_node_on_link(
        &mut self,
        in_link: UPtr<URigVMLink>,
        in_position: &crate::bindings::core_u_object::FVector2D,
        in_node_name: FString,
        b_setup_undo_redo: bool,
        b_print_python_command: bool,
    ) -> UPtr<URigVMRerouteNode> {
        let mut __stack = crate::core_data::StackAlloc::<56>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_CONTROLLER_ADD_REROUTE_NODE_ON_LINK,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_link,
                __buffer.add(0).cast::<UPtr<URigVMLink>>(),
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
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_node_name,
                __buffer.add(24).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_setup_undo_redo,
                __buffer.add(40).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_print_python_command,
                __buffer.add(41).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_CONTROLLER_ADD_REROUTE_NODE_ON_LINK,
                __buffer,
            )
        };
        unsafe { __buffer.add(48).cast::<UPtr<URigVMRerouteNode>>().read() }
    }
    pub fn add_parameter_node_from_object_path(
        &mut self,
        in_parameter_name: &FName,
        in_cpp_type: FString,
        in_cpp_type_object_path: FString,
        b_is_input: bool,
        in_default_value: FString,
        in_position: &crate::bindings::core_u_object::FVector2D,
        in_node_name: FString,
        b_setup_undo_redo: bool,
        b_print_python_command: bool,
    ) -> UPtr<URigVMParameterNode> {
        let mut __stack = crate::core_data::StackAlloc::<120>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_CONTROLLER_ADD_PARAMETER_NODE_FROM_OBJECT_PATH,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_parameter_name,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_cpp_type,
                __buffer.add(16).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_cpp_type_object_path,
                __buffer.add(32).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_is_input,
                __buffer.add(48).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_default_value,
                __buffer.add(56).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_position,
                __buffer.add(72).cast::<crate::bindings::core_u_object::FVector2D>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_node_name,
                __buffer.add(88).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_setup_undo_redo,
                __buffer.add(104).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_print_python_command,
                __buffer.add(105).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_CONTROLLER_ADD_PARAMETER_NODE_FROM_OBJECT_PATH,
                __buffer,
            )
        };
        unsafe { __buffer.add(112).cast::<UPtr<URigVMParameterNode>>().read() }
    }
    pub fn add_parameter_node(
        &mut self,
        in_parameter_name: &FName,
        in_cpp_type: FString,
        in_cpp_type_object: UPtr<crate::bindings::core_u_object::UObject>,
        b_is_input: bool,
        in_default_value: FString,
        in_position: &crate::bindings::core_u_object::FVector2D,
        in_node_name: FString,
        b_setup_undo_redo: bool,
        b_print_python_command: bool,
    ) -> UPtr<URigVMParameterNode> {
        let mut __stack = crate::core_data::StackAlloc::<112>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_CONTROLLER_ADD_PARAMETER_NODE,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_parameter_name,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_cpp_type,
                __buffer.add(16).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_cpp_type_object,
                __buffer.add(32).cast::<UPtr<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_is_input,
                __buffer.add(40).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_default_value,
                __buffer.add(48).cast::<FString>(),
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
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_node_name,
                __buffer.add(80).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_setup_undo_redo,
                __buffer.add(96).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_print_python_command,
                __buffer.add(97).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_CONTROLLER_ADD_PARAMETER_NODE,
                __buffer,
            )
        };
        unsafe { __buffer.add(104).cast::<UPtr<URigVMParameterNode>>().read() }
    }
    pub fn add_override_to_pins(
        &mut self,
        in_pin_paths: &TArray<FString>,
        b_setup_undo: bool,
        b_print_python_command: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<19>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_CONTROLLER_ADD_OVERRIDE_TO_PINS,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_pin_paths,
                __buffer.add(0).cast::<TArray<FString>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_setup_undo,
                __buffer.add(16).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_print_python_command,
                __buffer.add(17).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_CONTROLLER_ADD_OVERRIDE_TO_PINS,
                __buffer,
            )
        };
        unsafe { __buffer.add(18).cast::<bool>().read() }
    }
    pub fn add_override_to_pin(
        &mut self,
        in_pin_path: FString,
        b_setup_undo: bool,
        b_print_python_command: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<19>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_CONTROLLER_ADD_OVERRIDE_TO_PIN,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_pin_path,
                __buffer.add(0).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_setup_undo,
                __buffer.add(16).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_print_python_command,
                __buffer.add(17).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_CONTROLLER_ADD_OVERRIDE_TO_PIN,
                __buffer,
            )
        };
        unsafe { __buffer.add(18).cast::<bool>().read() }
    }
    pub fn add_override_to_all_pins_on_nodes(
        &mut self,
        in_node_names: &TArray<FName>,
        b_setup_undo: bool,
        b_print_python_command: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<19>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_CONTROLLER_ADD_OVERRIDE_TO_ALL_PINS_ON_NODES,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_node_names,
                __buffer.add(0).cast::<TArray<FName>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_setup_undo,
                __buffer.add(16).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_print_python_command,
                __buffer.add(17).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_CONTROLLER_ADD_OVERRIDE_TO_ALL_PINS_ON_NODES,
                __buffer,
            )
        };
        unsafe { __buffer.add(18).cast::<bool>().read() }
    }
    pub fn add_override_to_all_pins_on_node(
        &mut self,
        in_node_name: &FName,
        b_setup_undo: bool,
        b_print_python_command: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<15>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_CONTROLLER_ADD_OVERRIDE_TO_ALL_PINS_ON_NODE,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_node_name,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_setup_undo,
                __buffer.add(12).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_print_python_command,
                __buffer.add(13).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_CONTROLLER_ADD_OVERRIDE_TO_ALL_PINS_ON_NODE,
                __buffer,
            )
        };
        unsafe { __buffer.add(14).cast::<bool>().read() }
    }
    pub fn add_local_variable_from_object_path(
        &mut self,
        in_variable_name: &FName,
        in_cpp_type: FString,
        in_cpp_type_object_path: FString,
        in_default_value: FString,
        b_setup_undo_redo: bool,
    ) -> FRigVMGraphVariableDescription {
        let mut __stack = crate::core_data::StackAlloc::<184>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_CONTROLLER_ADD_LOCAL_VARIABLE_FROM_OBJECT_PATH,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_variable_name,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_cpp_type,
                __buffer.add(16).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_cpp_type_object_path,
                __buffer.add(32).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_default_value,
                __buffer.add(48).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_setup_undo_redo,
                __buffer.add(64).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_CONTROLLER_ADD_LOCAL_VARIABLE_FROM_OBJECT_PATH,
                __buffer,
            )
        };
        unsafe { __buffer.add(72).cast::<FRigVMGraphVariableDescription>().read() }
    }
    pub fn add_local_variable(
        &mut self,
        in_variable_name: &FName,
        in_cpp_type: FString,
        in_cpp_type_object: UPtr<crate::bindings::core_u_object::UObject>,
        in_default_value: FString,
        b_setup_undo_redo: bool,
        b_print_python_command: bool,
    ) -> FRigVMGraphVariableDescription {
        let mut __stack = crate::core_data::StackAlloc::<176>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_CONTROLLER_ADD_LOCAL_VARIABLE,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_variable_name,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_cpp_type,
                __buffer.add(16).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_cpp_type_object,
                __buffer.add(32).cast::<UPtr<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_default_value,
                __buffer.add(40).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_setup_undo_redo,
                __buffer.add(56).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_print_python_command,
                __buffer.add(57).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_CONTROLLER_ADD_LOCAL_VARIABLE,
                __buffer,
            )
        };
        unsafe { __buffer.add(64).cast::<FRigVMGraphVariableDescription>().read() }
    }
    pub fn add_link(
        &mut self,
        in_output_pin_path: FString,
        in_input_pin_path: FString,
        b_setup_undo_redo: bool,
        b_print_python_command: bool,
        in_user_direction: crate::bindings::rig_vm::ERigVMPinDirection,
        b_create_cast_node: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<37>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_CONTROLLER_ADD_LINK,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_output_pin_path,
                __buffer.add(0).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_input_pin_path,
                __buffer.add(16).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_setup_undo_redo,
                __buffer.add(32).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_print_python_command,
                __buffer.add(33).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_user_direction,
                __buffer.add(34).cast::<crate::bindings::rig_vm::ERigVMPinDirection>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_create_cast_node,
                __buffer.add(35).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_CONTROLLER_ADD_LINK,
                __buffer,
            )
        };
        unsafe { __buffer.add(36).cast::<bool>().read() }
    }
    pub fn add_invoke_entry_node(
        &mut self,
        in_entry_name: &FName,
        in_position: &crate::bindings::core_u_object::FVector2D,
        in_node_name: FString,
        b_setup_undo_redo: bool,
        b_print_python_command: bool,
    ) -> UPtr<URigVMInvokeEntryNode> {
        let mut __stack = crate::core_data::StackAlloc::<64>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_CONTROLLER_ADD_INVOKE_ENTRY_NODE,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_entry_name,
                __buffer.add(0).cast::<FName>(),
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
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_node_name,
                __buffer.add(32).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_setup_undo_redo,
                __buffer.add(48).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_print_python_command,
                __buffer.add(49).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_CONTROLLER_ADD_INVOKE_ENTRY_NODE,
                __buffer,
            )
        };
        unsafe { __buffer.add(56).cast::<UPtr<URigVMInvokeEntryNode>>().read() }
    }
    pub fn add_injected_node_from_struct_path(
        &mut self,
        in_pin_path: FString,
        b_as_input: bool,
        in_script_struct_path: FString,
        in_method_name: &FName,
        in_input_pin_name: &FName,
        in_output_pin_name: &FName,
        in_node_name: FString,
        b_setup_undo_redo: bool,
    ) -> UPtr<URigVMInjectionInfo> {
        let mut __stack = crate::core_data::StackAlloc::<112>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_CONTROLLER_ADD_INJECTED_NODE_FROM_STRUCT_PATH,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_pin_path,
                __buffer.add(0).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_as_input,
                __buffer.add(16).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_script_struct_path,
                __buffer.add(24).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_method_name,
                __buffer.add(40).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_input_pin_name,
                __buffer.add(52).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_output_pin_name,
                __buffer.add(64).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_node_name,
                __buffer.add(80).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_setup_undo_redo,
                __buffer.add(96).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_CONTROLLER_ADD_INJECTED_NODE_FROM_STRUCT_PATH,
                __buffer,
            )
        };
        unsafe { __buffer.add(104).cast::<UPtr<URigVMInjectionInfo>>().read() }
    }
    pub fn add_injected_node(
        &mut self,
        in_pin_path: FString,
        b_as_input: bool,
        in_script_struct: UPtr<crate::bindings::core_u_object::UScriptStruct>,
        in_method_name: &FName,
        in_input_pin_name: &FName,
        in_output_pin_name: &FName,
        in_node_name: FString,
        b_setup_undo_redo: bool,
        b_print_python_command: bool,
    ) -> UPtr<URigVMInjectionInfo> {
        let mut __stack = crate::core_data::StackAlloc::<104>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_CONTROLLER_ADD_INJECTED_NODE,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_pin_path,
                __buffer.add(0).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_as_input,
                __buffer.add(16).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_script_struct,
                __buffer
                    .add(24)
                    .cast::<UPtr<crate::bindings::core_u_object::UScriptStruct>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_method_name,
                __buffer.add(32).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_input_pin_name,
                __buffer.add(44).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_output_pin_name,
                __buffer.add(56).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_node_name,
                __buffer.add(72).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_setup_undo_redo,
                __buffer.add(88).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_print_python_command,
                __buffer.add(89).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_CONTROLLER_ADD_INJECTED_NODE,
                __buffer,
            )
        };
        unsafe { __buffer.add(96).cast::<UPtr<URigVMInjectionInfo>>().read() }
    }
    pub fn add_if_node_from_struct(
        &mut self,
        in_script_struct: UPtr<crate::bindings::core_u_object::UScriptStruct>,
        in_position: &crate::bindings::core_u_object::FVector2D,
        in_node_name: FString,
        b_setup_undo_redo: bool,
    ) -> UPtr<URigVMNode> {
        let mut __stack = crate::core_data::StackAlloc::<56>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_CONTROLLER_ADD_IF_NODE_FROM_STRUCT,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_script_struct,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::core_u_object::UScriptStruct>>(),
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
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_node_name,
                __buffer.add(24).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_setup_undo_redo,
                __buffer.add(40).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_CONTROLLER_ADD_IF_NODE_FROM_STRUCT,
                __buffer,
            )
        };
        unsafe { __buffer.add(48).cast::<UPtr<URigVMNode>>().read() }
    }
    pub fn add_if_node(
        &mut self,
        in_cpp_type: FString,
        in_cpp_type_object_path: &FName,
        in_position: &crate::bindings::core_u_object::FVector2D,
        in_node_name: FString,
        b_setup_undo_redo: bool,
        b_print_python_command: bool,
    ) -> UPtr<URigVMNode> {
        let mut __stack = crate::core_data::StackAlloc::<80>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_CONTROLLER_ADD_IF_NODE,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_cpp_type,
                __buffer.add(0).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_cpp_type_object_path,
                __buffer.add(16).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_position,
                __buffer.add(32).cast::<crate::bindings::core_u_object::FVector2D>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_node_name,
                __buffer.add(48).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_setup_undo_redo,
                __buffer.add(64).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_print_python_command,
                __buffer.add(65).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_CONTROLLER_ADD_IF_NODE,
                __buffer,
            )
        };
        unsafe { __buffer.add(72).cast::<UPtr<URigVMNode>>().read() }
    }
    pub fn add_function_to_library(
        &mut self,
        in_function_name: &FName,
        b_mutable: bool,
        in_node_position: &crate::bindings::core_u_object::FVector2D,
        b_setup_undo_redo: bool,
        b_print_python_command: bool,
    ) -> UPtr<URigVMLibraryNode> {
        let mut __stack = crate::core_data::StackAlloc::<48>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_CONTROLLER_ADD_FUNCTION_TO_LIBRARY,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_function_name,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_mutable,
                __buffer.add(12).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_node_position,
                __buffer.add(16).cast::<crate::bindings::core_u_object::FVector2D>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_setup_undo_redo,
                __buffer.add(32).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_print_python_command,
                __buffer.add(33).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_CONTROLLER_ADD_FUNCTION_TO_LIBRARY,
                __buffer,
            )
        };
        unsafe { __buffer.add(40).cast::<UPtr<URigVMLibraryNode>>().read() }
    }
    pub fn add_function_reference_node_from_description(
        &mut self,
        in_function_definition: &crate::bindings::rig_vm::FRigVMGraphFunctionHeader,
        in_node_position: &crate::bindings::core_u_object::FVector2D,
        in_node_name: FString,
        b_setup_undo_redo: bool,
        b_print_python_command: bool,
    ) -> UPtr<URigVMFunctionReferenceNode> {
        let mut __stack = crate::core_data::StackAlloc::<576>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_CONTROLLER_ADD_FUNCTION_REFERENCE_NODE_FROM_DESCRIPTION,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_function_definition,
                __buffer
                    .add(0)
                    .cast::<crate::bindings::rig_vm::FRigVMGraphFunctionHeader>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_node_position,
                __buffer.add(528).cast::<crate::bindings::core_u_object::FVector2D>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_node_name,
                __buffer.add(544).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_setup_undo_redo,
                __buffer.add(560).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_print_python_command,
                __buffer.add(561).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_CONTROLLER_ADD_FUNCTION_REFERENCE_NODE_FROM_DESCRIPTION,
                __buffer,
            )
        };
        unsafe { __buffer.add(568).cast::<UPtr<URigVMFunctionReferenceNode>>().read() }
    }
    pub fn add_function_reference_node(
        &mut self,
        in_function_definition: UPtr<URigVMLibraryNode>,
        in_node_position: &crate::bindings::core_u_object::FVector2D,
        in_node_name: FString,
        b_setup_undo_redo: bool,
        b_print_python_command: bool,
    ) -> UPtr<URigVMFunctionReferenceNode> {
        let mut __stack = crate::core_data::StackAlloc::<56>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_CONTROLLER_ADD_FUNCTION_REFERENCE_NODE,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_function_definition,
                __buffer.add(0).cast::<UPtr<URigVMLibraryNode>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_node_position,
                __buffer.add(8).cast::<crate::bindings::core_u_object::FVector2D>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_node_name,
                __buffer.add(24).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_setup_undo_redo,
                __buffer.add(40).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_print_python_command,
                __buffer.add(41).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_CONTROLLER_ADD_FUNCTION_REFERENCE_NODE,
                __buffer,
            )
        };
        unsafe { __buffer.add(48).cast::<UPtr<URigVMFunctionReferenceNode>>().read() }
    }
    pub fn add_free_reroute_node(
        &mut self,
        in_cpp_type: FString,
        in_cpp_type_object_path: &FName,
        b_is_constant: bool,
        in_custom_widget_name: &FName,
        in_default_value: FString,
        in_position: &crate::bindings::core_u_object::FVector2D,
        in_node_name: FString,
        b_setup_undo_redo: bool,
    ) -> UPtr<URigVMRerouteNode> {
        let mut __stack = crate::core_data::StackAlloc::<112>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_CONTROLLER_ADD_FREE_REROUTE_NODE,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_cpp_type,
                __buffer.add(0).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_cpp_type_object_path,
                __buffer.add(16).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_is_constant,
                __buffer.add(28).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_custom_widget_name,
                __buffer.add(32).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_default_value,
                __buffer.add(48).cast::<FString>(),
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
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_node_name,
                __buffer.add(80).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_setup_undo_redo,
                __buffer.add(96).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_CONTROLLER_ADD_FREE_REROUTE_NODE,
                __buffer,
            )
        };
        unsafe { __buffer.add(104).cast::<UPtr<URigVMRerouteNode>>().read() }
    }
    pub fn add_external_function_reference_node(
        &mut self,
        in_host_path: FString,
        in_function_name: &FName,
        in_node_position: &crate::bindings::core_u_object::FVector2D,
        in_node_name: FString,
        b_setup_undo_redo: bool,
        b_print_python_command: bool,
    ) -> UPtr<URigVMFunctionReferenceNode> {
        let mut __stack = crate::core_data::StackAlloc::<80>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_CONTROLLER_ADD_EXTERNAL_FUNCTION_REFERENCE_NODE,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_host_path,
                __buffer.add(0).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_function_name,
                __buffer.add(16).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_node_position,
                __buffer.add(32).cast::<crate::bindings::core_u_object::FVector2D>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_node_name,
                __buffer.add(48).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_setup_undo_redo,
                __buffer.add(64).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_print_python_command,
                __buffer.add(65).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_CONTROLLER_ADD_EXTERNAL_FUNCTION_REFERENCE_NODE,
                __buffer,
            )
        };
        unsafe { __buffer.add(72).cast::<UPtr<URigVMFunctionReferenceNode>>().read() }
    }
    pub fn add_exposed_pin(
        &mut self,
        in_pin_name: &FName,
        in_direction: crate::bindings::rig_vm::ERigVMPinDirection,
        in_cpp_type: FString,
        in_cpp_type_object_path: &FName,
        in_default_value: FString,
        b_setup_undo_redo: bool,
        b_print_python_command: bool,
    ) -> FName {
        let mut __stack = crate::core_data::StackAlloc::<80>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_CONTROLLER_ADD_EXPOSED_PIN,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_pin_name,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_direction,
                __buffer.add(12).cast::<crate::bindings::rig_vm::ERigVMPinDirection>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_cpp_type,
                __buffer.add(16).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_cpp_type_object_path,
                __buffer.add(32).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_default_value,
                __buffer.add(48).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_setup_undo_redo,
                __buffer.add(64).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_print_python_command,
                __buffer.add(65).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_CONTROLLER_ADD_EXPOSED_PIN,
                __buffer,
            )
        };
        unsafe { __buffer.add(68).cast::<FName>().read() }
    }
    pub fn add_enum_node(
        &mut self,
        in_cpp_type_object_path: &FName,
        in_position: &crate::bindings::core_u_object::FVector2D,
        in_node_name: FString,
        b_setup_undo_redo: bool,
        b_print_python_command: bool,
    ) -> UPtr<URigVMEnumNode> {
        let mut __stack = crate::core_data::StackAlloc::<64>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_CONTROLLER_ADD_ENUM_NODE,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_cpp_type_object_path,
                __buffer.add(0).cast::<FName>(),
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
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_node_name,
                __buffer.add(32).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_setup_undo_redo,
                __buffer.add(48).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_print_python_command,
                __buffer.add(49).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_CONTROLLER_ADD_ENUM_NODE,
                __buffer,
            )
        };
        unsafe { __buffer.add(56).cast::<UPtr<URigVMEnumNode>>().read() }
    }
    pub fn add_empty_pin_category(
        &mut self,
        in_node_name: &FName,
        in_category: FString,
        b_setup_undo_redo: bool,
        b_print_python_command: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<35>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_CONTROLLER_ADD_EMPTY_PIN_CATEGORY,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_node_name,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_category,
                __buffer.add(16).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_setup_undo_redo,
                __buffer.add(32).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_print_python_command,
                __buffer.add(33).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_CONTROLLER_ADD_EMPTY_PIN_CATEGORY,
                __buffer,
            )
        };
        unsafe { __buffer.add(34).cast::<bool>().read() }
    }
    pub fn add_default_tag_to_function_variant(
        &mut self,
        in_function_name: &FName,
        in_tag_name: &FName,
        b_setup_undo_redo: bool,
        b_print_python_command: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<27>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_CONTROLLER_ADD_DEFAULT_TAG_TO_FUNCTION_VARIANT,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_function_name,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_tag_name,
                __buffer.add(12).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_setup_undo_redo,
                __buffer.add(24).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_print_python_command,
                __buffer.add(25).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_CONTROLLER_ADD_DEFAULT_TAG_TO_FUNCTION_VARIANT,
                __buffer,
            )
        };
        unsafe { __buffer.add(26).cast::<bool>().read() }
    }
    pub fn add_comment_node(
        &mut self,
        in_comment_text: FString,
        in_position: &crate::bindings::core_u_object::FVector2D,
        in_size: &crate::bindings::core_u_object::FVector2D,
        in_color: &crate::bindings::core_u_object::FLinearColor,
        in_node_name: FString,
        b_setup_undo_redo: bool,
        b_print_python_command: bool,
    ) -> UPtr<URigVMCommentNode> {
        let mut __stack = crate::core_data::StackAlloc::<96>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_CONTROLLER_ADD_COMMENT_NODE,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_comment_text,
                __buffer.add(0).cast::<FString>(),
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
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_size,
                __buffer.add(32).cast::<crate::bindings::core_u_object::FVector2D>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_color,
                __buffer.add(48).cast::<crate::bindings::core_u_object::FLinearColor>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_node_name,
                __buffer.add(64).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_setup_undo_redo,
                __buffer.add(80).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_print_python_command,
                __buffer.add(81).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_CONTROLLER_ADD_COMMENT_NODE,
                __buffer,
            )
        };
        unsafe { __buffer.add(88).cast::<UPtr<URigVMCommentNode>>().read() }
    }
    pub fn add_branch_node(
        &mut self,
        in_position: &crate::bindings::core_u_object::FVector2D,
        in_node_name: FString,
        b_setup_undo_redo: bool,
        b_print_python_command: bool,
    ) -> UPtr<URigVMNode> {
        let mut __stack = crate::core_data::StackAlloc::<48>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_CONTROLLER_ADD_BRANCH_NODE,
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
                &in_node_name,
                __buffer.add(16).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_setup_undo_redo,
                __buffer.add(32).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_print_python_command,
                __buffer.add(33).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_CONTROLLER_ADD_BRANCH_NODE,
                __buffer,
            )
        };
        unsafe { __buffer.add(40).cast::<UPtr<URigVMNode>>().read() }
    }
    pub fn add_array_pin(
        &mut self,
        in_array_pin_path: FString,
        in_default_value: FString,
        b_setup_undo_redo: bool,
        b_print_python_command: bool,
    ) -> FString {
        let mut __stack = crate::core_data::StackAlloc::<56>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_CONTROLLER_ADD_ARRAY_PIN,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_array_pin_path,
                __buffer.add(0).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_default_value,
                __buffer.add(16).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_setup_undo_redo,
                __buffer.add(32).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_print_python_command,
                __buffer.add(33).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_CONTROLLER_ADD_ARRAY_PIN,
                __buffer,
            )
        };
        unsafe { __buffer.add(40).cast::<FString>().read() }
    }
    pub fn add_array_node_from_object_path(
        &mut self,
        in_op_code: crate::bindings::rig_vm::ERigVMOpCode,
        in_cpp_type: FString,
        in_cpp_type_object_path: FString,
        in_position: &crate::bindings::core_u_object::FVector2D,
        in_node_name: FString,
        b_setup_undo_redo: bool,
        b_print_python_command: bool,
        b_is_patching: bool,
    ) -> UPtr<URigVMNode> {
        let mut __stack = crate::core_data::StackAlloc::<88>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_CONTROLLER_ADD_ARRAY_NODE_FROM_OBJECT_PATH,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_op_code,
                __buffer.add(0).cast::<crate::bindings::rig_vm::ERigVMOpCode>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_cpp_type,
                __buffer.add(8).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_cpp_type_object_path,
                __buffer.add(24).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_position,
                __buffer.add(40).cast::<crate::bindings::core_u_object::FVector2D>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_node_name,
                __buffer.add(56).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_setup_undo_redo,
                __buffer.add(72).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_print_python_command,
                __buffer.add(73).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_is_patching,
                __buffer.add(74).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_CONTROLLER_ADD_ARRAY_NODE_FROM_OBJECT_PATH,
                __buffer,
            )
        };
        unsafe { __buffer.add(80).cast::<UPtr<URigVMNode>>().read() }
    }
    pub fn add_array_node(
        &mut self,
        in_op_code: crate::bindings::rig_vm::ERigVMOpCode,
        in_cpp_type: FString,
        in_cpp_type_object: UPtr<crate::bindings::core_u_object::UObject>,
        in_position: &crate::bindings::core_u_object::FVector2D,
        in_node_name: FString,
        b_setup_undo_redo: bool,
        b_print_python_command: bool,
        b_is_patching: bool,
    ) -> UPtr<URigVMNode> {
        let mut __stack = crate::core_data::StackAlloc::<80>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_CONTROLLER_ADD_ARRAY_NODE,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_op_code,
                __buffer.add(0).cast::<crate::bindings::rig_vm::ERigVMOpCode>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_cpp_type,
                __buffer.add(8).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_cpp_type_object,
                __buffer.add(24).cast::<UPtr<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_position,
                __buffer.add(32).cast::<crate::bindings::core_u_object::FVector2D>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_node_name,
                __buffer.add(48).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_setup_undo_redo,
                __buffer.add(64).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_print_python_command,
                __buffer.add(65).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_is_patching,
                __buffer.add(66).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_CONTROLLER_ADD_ARRAY_NODE,
                __buffer,
            )
        };
        unsafe { __buffer.add(72).cast::<UPtr<URigVMNode>>().read() }
    }
    pub fn add_aggregate_pin(
        &mut self,
        in_node_name: FString,
        in_pin_name: FString,
        in_default_value: FString,
        b_setup_undo_redo: bool,
        b_print_python_command: bool,
    ) -> FString {
        let mut __stack = crate::core_data::StackAlloc::<72>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::U_RIG_VM_CONTROLLER_ADD_AGGREGATE_PIN,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_node_name,
                __buffer.add(0).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_pin_name,
                __buffer.add(16).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_default_value,
                __buffer.add(32).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_setup_undo_redo,
                __buffer.add(48).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_print_python_command,
                __buffer.add(49).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::U_RIG_VM_CONTROLLER_ADD_AGGREGATE_PIN,
                __buffer,
            )
        };
        unsafe { __buffer.add(56).cast::<FString>().read() }
    }
}
#[repr(C, align(8))]
pub struct URigVMControllerSettings {
    __padding_end: [u8; 136],
}
impl URigVMControllerSettings {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("URigVMControllerSettings")
            .unwrap()
    }
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
pub struct FRegisterProvider_InProvider {
    _opague: [u8; 32],
}
#[repr(C, align(8))]
pub struct FRigVMController_ModifiedEventDynamic {
    _opague: [u8; 24],
}
#[repr(transparent)]
pub struct ERigVMTagDisplayMode(pub u8);
impl ERigVMTagDisplayMode {
    pub const NONE: ERigVMTagDisplayMode = ERigVMTagDisplayMode(0);
    pub const ALL: ERigVMTagDisplayMode = ERigVMTagDisplayMode(1);
    pub const DEPRECATION_ONLY: ERigVMTagDisplayMode = ERigVMTagDisplayMode(2);
    pub const LAST: ERigVMTagDisplayMode = ERigVMTagDisplayMode(2);
}
#[repr(transparent)]
pub struct ERigVMPinDefaultValueType(pub u8);
impl ERigVMPinDefaultValueType {
    pub const AUTO_DETECT: ERigVMPinDefaultValueType = ERigVMPinDefaultValueType(0);
    pub const UNSET: ERigVMPinDefaultValueType = ERigVMPinDefaultValueType(1);
    pub const OVERRIDE: ERigVMPinDefaultValueType = ERigVMPinDefaultValueType(2);
    pub const KEEP_VALUE_TYPE: ERigVMPinDefaultValueType = ERigVMPinDefaultValueType(3);
}
#[repr(transparent)]
pub struct ERigVMNodeColorType(pub u8);
impl ERigVMNodeColorType {
    pub const FROM_METADATA: ERigVMNodeColorType = ERigVMNodeColorType(0);
    pub const USER_DEFINED: ERigVMNodeColorType = ERigVMNodeColorType(1);
}
#[repr(transparent)]
pub struct ERigVMGraphNotifType(pub u8);
impl ERigVMGraphNotifType {
    pub const GRAPH_CHANGED: ERigVMGraphNotifType = ERigVMGraphNotifType(0);
    pub const NODE_ADDED: ERigVMGraphNotifType = ERigVMGraphNotifType(1);
    pub const NODE_REMOVED: ERigVMGraphNotifType = ERigVMGraphNotifType(2);
    pub const NODE_SELECTED: ERigVMGraphNotifType = ERigVMGraphNotifType(3);
    pub const NODE_DESELECTED: ERigVMGraphNotifType = ERigVMGraphNotifType(4);
    pub const NODE_SELECTION_CHANGED: ERigVMGraphNotifType = ERigVMGraphNotifType(5);
    pub const NODE_POSITION_CHANGED: ERigVMGraphNotifType = ERigVMGraphNotifType(6);
    pub const NODE_SIZE_CHANGED: ERigVMGraphNotifType = ERigVMGraphNotifType(7);
    pub const NODE_TITLE_CHANGED: ERigVMGraphNotifType = ERigVMGraphNotifType(8);
    pub const NODE_COLOR_CHANGED: ERigVMGraphNotifType = ERigVMGraphNotifType(9);
    pub const PIN_ADDED: ERigVMGraphNotifType = ERigVMGraphNotifType(10);
    pub const PIN_REMOVED: ERigVMGraphNotifType = ERigVMGraphNotifType(11);
    pub const PIN_RENAMED: ERigVMGraphNotifType = ERigVMGraphNotifType(12);
    pub const PIN_EXPANSION_CHANGED: ERigVMGraphNotifType = ERigVMGraphNotifType(13);
    pub const PIN_WATCHED_CHANGED: ERigVMGraphNotifType = ERigVMGraphNotifType(14);
    pub const PIN_ARRAY_SIZE_CHANGED: ERigVMGraphNotifType = ERigVMGraphNotifType(15);
    pub const PIN_DEFAULT_VALUE_CHANGED: ERigVMGraphNotifType = ERigVMGraphNotifType(16);
    pub const PIN_DIRECTION_CHANGED: ERigVMGraphNotifType = ERigVMGraphNotifType(17);
    pub const PIN_TYPE_CHANGED: ERigVMGraphNotifType = ERigVMGraphNotifType(18);
    pub const PIN_INDEX_CHANGED: ERigVMGraphNotifType = ERigVMGraphNotifType(19);
    pub const LINK_ADDED: ERigVMGraphNotifType = ERigVMGraphNotifType(20);
    pub const LINK_REMOVED: ERigVMGraphNotifType = ERigVMGraphNotifType(21);
    pub const COMMENT_TEXT_CHANGED: ERigVMGraphNotifType = ERigVMGraphNotifType(22);
    pub const VARIABLE_ADDED: ERigVMGraphNotifType = ERigVMGraphNotifType(23);
    pub const VARIABLE_REMOVED: ERigVMGraphNotifType = ERigVMGraphNotifType(24);
    pub const VARIABLE_RENAMED: ERigVMGraphNotifType = ERigVMGraphNotifType(25);
    pub const INTERACTION_BRACKET_OPENED: ERigVMGraphNotifType = ERigVMGraphNotifType(
        26,
    );
    pub const INTERACTION_BRACKET_CLOSED: ERigVMGraphNotifType = ERigVMGraphNotifType(
        27,
    );
    pub const INTERACTION_BRACKET_CANCELED: ERigVMGraphNotifType = ERigVMGraphNotifType(
        28,
    );
    pub const PIN_BOUND_VARIABLE_CHANGED: ERigVMGraphNotifType = ERigVMGraphNotifType(
        29,
    );
    pub const NODE_RENAMED: ERigVMGraphNotifType = ERigVMGraphNotifType(30);
    pub const FUNCTION_RENAMED: ERigVMGraphNotifType = ERigVMGraphNotifType(31);
    pub const NODE_REFERENCE_CHANGED: ERigVMGraphNotifType = ERigVMGraphNotifType(32);
    pub const NODE_CATEGORY_CHANGED: ERigVMGraphNotifType = ERigVMGraphNotifType(33);
    pub const NODE_KEYWORDS_CHANGED: ERigVMGraphNotifType = ERigVMGraphNotifType(34);
    pub const NODE_DESCRIPTION_CHANGED: ERigVMGraphNotifType = ERigVMGraphNotifType(35);
    pub const VARIABLE_REMAPPING_CHANGED: ERigVMGraphNotifType = ERigVMGraphNotifType(
        36,
    );
    pub const LIBRARY_TEMPLATE_CHANGED: ERigVMGraphNotifType = ERigVMGraphNotifType(37);
    pub const FUNCTION_ACCESS_CHANGED: ERigVMGraphNotifType = ERigVMGraphNotifType(38);
    pub const VARIANT_TAGS_CHANGED: ERigVMGraphNotifType = ERigVMGraphNotifType(39);
    pub const PIN_DISPLAY_NAME_CHANGED: ERigVMGraphNotifType = ERigVMGraphNotifType(40);
    pub const PIN_CATEGORY_CHANGED: ERigVMGraphNotifType = ERigVMGraphNotifType(41);
    pub const PIN_CATEGORIES_CHANGED: ERigVMGraphNotifType = ERigVMGraphNotifType(42);
    pub const PIN_CATEGORY_EXPANSION_CHANGED: ERigVMGraphNotifType = ERigVMGraphNotifType(
        43,
    );
    pub const FUNCTION_VARIANT_GUID_CHANGED: ERigVMGraphNotifType = ERigVMGraphNotifType(
        44,
    );
    pub const NODE_EARLY_EXIT_CHANGED: ERigVMGraphNotifType = ERigVMGraphNotifType(45);
    pub const LOCAL_VARIABLE_DEFAULT_VALUE_CHANGED: ERigVMGraphNotifType = ERigVMGraphNotifType(
        46,
    );
    pub const LOCAL_VARIABLE_ADDED: ERigVMGraphNotifType = ERigVMGraphNotifType(47);
    pub const LOCAL_VARIABLE_REMOVED: ERigVMGraphNotifType = ERigVMGraphNotifType(48);
    pub const LOCAL_VARIABLE_RENAMED: ERigVMGraphNotifType = ERigVMGraphNotifType(49);
    pub const LOCAL_VARIABLE_TYPE_CHANGED: ERigVMGraphNotifType = ERigVMGraphNotifType(
        50,
    );
    pub const INVALID: ERigVMGraphNotifType = ERigVMGraphNotifType(51);
}
