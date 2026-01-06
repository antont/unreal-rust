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
    __padding_end: [u8; 4],
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
    __padding_end: [u8; 4],
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
