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
pub static mut UAI_ASYNC_TASK_BLUEPRINT_PROXY_ON_MOVE_COMPLETED: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut AAI_CONTROLLER_USE_BLACKBOARD: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut AAI_CONTROLLER_UNCLAIM_TASK_RESOURCE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut AAI_CONTROLLER_SET_PATH_FOLLOWING_COMPONENT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut AAI_CONTROLLER_SET_MOVE_BLOCK_DETECTION: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut AAI_CONTROLLER_RUN_BEHAVIOR_TREE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut AAI_CONTROLLER_ON_USING_BLACK_BOARD: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut AAI_CONTROLLER_ON_GAMEPLAY_TASK_RESOURCES_CLAIMED: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut AAI_CONTROLLER_MOVE_TO_LOCATION: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut AAI_CONTROLLER_MOVE_TO_ACTOR: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut AAI_CONTROLLER_K2_SET_FOCUS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut AAI_CONTROLLER_K2_SET_FOCAL_POINT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut AAI_CONTROLLER_K2_CLEAR_FOCUS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut AAI_CONTROLLER_HAS_PARTIAL_PATH: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut AAI_CONTROLLER_GET_PATH_FOLLOWING_COMPONENT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut AAI_CONTROLLER_GET_MOVE_STATUS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut AAI_CONTROLLER_GET_IMMEDIATE_MOVE_DESTINATION: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut AAI_CONTROLLER_GET_FOCUS_ACTOR: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut AAI_CONTROLLER_GET_FOCAL_POINT_ON_ACTOR: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut AAI_CONTROLLER_GET_FOCAL_POINT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut AAI_CONTROLLER_GET_AI_PERCEPTION_COMPONENT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut AAI_CONTROLLER_CLAIM_TASK_RESOURCE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut UAI_SYSTEM_AI_LOGGING_VERBOSE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut UAI_SYSTEM_AI_IGNORE_PLAYERS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_BRAIN_COMPONENT_STOP_LOGIC: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_BRAIN_COMPONENT_START_LOGIC: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_BRAIN_COMPONENT_RESTART_LOGIC: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_BRAIN_COMPONENT_IS_RUNNING: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_BRAIN_COMPONENT_IS_PAUSED: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_BEHAVIOR_TREE_COMPONENT_SET_DYNAMIC_SUBTREE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_BEHAVIOR_TREE_COMPONENT_GET_TAG_COOLDOWN_END_TIME: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_BEHAVIOR_TREE_COMPONENT_ADD_COOLDOWN_TAG_DURATION: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_BLACKBOARD_ASSET_PROVIDER_GET_BLACKBOARD_ASSET: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_BLACKBOARD_COMPONENT_SET_VALUE_AS_VECTOR: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_BLACKBOARD_COMPONENT_SET_VALUE_AS_STRING: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_BLACKBOARD_COMPONENT_SET_VALUE_AS_ROTATOR: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_BLACKBOARD_COMPONENT_SET_VALUE_AS_OBJECT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_BLACKBOARD_COMPONENT_SET_VALUE_AS_NAME: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_BLACKBOARD_COMPONENT_SET_VALUE_AS_INT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_BLACKBOARD_COMPONENT_SET_VALUE_AS_FLOAT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_BLACKBOARD_COMPONENT_SET_VALUE_AS_ENUM: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_BLACKBOARD_COMPONENT_SET_VALUE_AS_CLASS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_BLACKBOARD_COMPONENT_SET_VALUE_AS_BOOL: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_BLACKBOARD_COMPONENT_IS_VECTOR_VALUE_SET: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_BLACKBOARD_COMPONENT_GET_VALUE_AS_VECTOR: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_BLACKBOARD_COMPONENT_GET_VALUE_AS_STRING: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_BLACKBOARD_COMPONENT_GET_VALUE_AS_ROTATOR: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_BLACKBOARD_COMPONENT_GET_VALUE_AS_OBJECT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_BLACKBOARD_COMPONENT_GET_VALUE_AS_NAME: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_BLACKBOARD_COMPONENT_GET_VALUE_AS_INT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_BLACKBOARD_COMPONENT_GET_VALUE_AS_FLOAT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_BLACKBOARD_COMPONENT_GET_VALUE_AS_ENUM: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_BLACKBOARD_COMPONENT_GET_VALUE_AS_CLASS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_BLACKBOARD_COMPONENT_GET_VALUE_AS_BOOL: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_BLACKBOARD_COMPONENT_GET_ROTATION_FROM_ENTRY: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_BLACKBOARD_COMPONENT_GET_LOCATION_FROM_ENTRY: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_BLACKBOARD_COMPONENT_CLEAR_VALUE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut UBT_FUNCTION_LIBRARY_STOP_USING_EXTERNAL_EVENT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut UBT_FUNCTION_LIBRARY_START_USING_EXTERNAL_EVENT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut UBT_FUNCTION_LIBRARY_SET_BLACKBOARD_VALUE_AS_VECTOR: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut UBT_FUNCTION_LIBRARY_SET_BLACKBOARD_VALUE_AS_STRING: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut UBT_FUNCTION_LIBRARY_SET_BLACKBOARD_VALUE_AS_ROTATOR: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut UBT_FUNCTION_LIBRARY_SET_BLACKBOARD_VALUE_AS_OBJECT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut UBT_FUNCTION_LIBRARY_SET_BLACKBOARD_VALUE_AS_NAME: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut UBT_FUNCTION_LIBRARY_SET_BLACKBOARD_VALUE_AS_INT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut UBT_FUNCTION_LIBRARY_SET_BLACKBOARD_VALUE_AS_FLOAT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut UBT_FUNCTION_LIBRARY_SET_BLACKBOARD_VALUE_AS_ENUM: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut UBT_FUNCTION_LIBRARY_SET_BLACKBOARD_VALUE_AS_CLASS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut UBT_FUNCTION_LIBRARY_SET_BLACKBOARD_VALUE_AS_BOOL: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut UBT_FUNCTION_LIBRARY_GET_OWNERS_BLACKBOARD: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut UBT_FUNCTION_LIBRARY_GET_OWNER_COMPONENT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut UBT_FUNCTION_LIBRARY_GET_BLACKBOARD_VALUE_AS_VECTOR: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut UBT_FUNCTION_LIBRARY_GET_BLACKBOARD_VALUE_AS_STRING: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut UBT_FUNCTION_LIBRARY_GET_BLACKBOARD_VALUE_AS_ROTATOR: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut UBT_FUNCTION_LIBRARY_GET_BLACKBOARD_VALUE_AS_OBJECT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut UBT_FUNCTION_LIBRARY_GET_BLACKBOARD_VALUE_AS_NAME: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut UBT_FUNCTION_LIBRARY_GET_BLACKBOARD_VALUE_AS_INT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut UBT_FUNCTION_LIBRARY_GET_BLACKBOARD_VALUE_AS_FLOAT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut UBT_FUNCTION_LIBRARY_GET_BLACKBOARD_VALUE_AS_ENUM: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut UBT_FUNCTION_LIBRARY_GET_BLACKBOARD_VALUE_AS_CLASS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut UBT_FUNCTION_LIBRARY_GET_BLACKBOARD_VALUE_AS_BOOL: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut UBT_FUNCTION_LIBRARY_GET_BLACKBOARD_VALUE_AS_ACTOR: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut UBT_FUNCTION_LIBRARY_CLEAR_BLACKBOARD_VALUE_AS_VECTOR: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut UBT_FUNCTION_LIBRARY_CLEAR_BLACKBOARD_VALUE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut UBT_DECORATOR_BLUEPRINT_BASE_RECEIVE_TICK_AI: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut UBT_DECORATOR_BLUEPRINT_BASE_RECEIVE_TICK: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut UBT_DECORATOR_BLUEPRINT_BASE_RECEIVE_OBSERVER_DEACTIVATED_AI: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut UBT_DECORATOR_BLUEPRINT_BASE_RECEIVE_OBSERVER_DEACTIVATED: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut UBT_DECORATOR_BLUEPRINT_BASE_RECEIVE_OBSERVER_ACTIVATED_AI: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut UBT_DECORATOR_BLUEPRINT_BASE_RECEIVE_OBSERVER_ACTIVATED: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut UBT_DECORATOR_BLUEPRINT_BASE_RECEIVE_EXECUTION_START_AI: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut UBT_DECORATOR_BLUEPRINT_BASE_RECEIVE_EXECUTION_START: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut UBT_DECORATOR_BLUEPRINT_BASE_RECEIVE_EXECUTION_FINISH_AI: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut UBT_DECORATOR_BLUEPRINT_BASE_RECEIVE_EXECUTION_FINISH: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut UBT_DECORATOR_BLUEPRINT_BASE_PERFORM_CONDITION_CHECK_AI: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut UBT_DECORATOR_BLUEPRINT_BASE_PERFORM_CONDITION_CHECK: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut UBT_DECORATOR_BLUEPRINT_BASE_IS_DECORATOR_OBSERVER_ACTIVE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut UBT_DECORATOR_BLUEPRINT_BASE_IS_DECORATOR_EXECUTION_ACTIVE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut UBT_SERVICE_BLUEPRINT_BASE_RECEIVE_TICK_AI: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut UBT_SERVICE_BLUEPRINT_BASE_RECEIVE_TICK: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut UBT_SERVICE_BLUEPRINT_BASE_RECEIVE_SEARCH_START_AI: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut UBT_SERVICE_BLUEPRINT_BASE_RECEIVE_SEARCH_START: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut UBT_SERVICE_BLUEPRINT_BASE_RECEIVE_DEACTIVATION_AI: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut UBT_SERVICE_BLUEPRINT_BASE_RECEIVE_DEACTIVATION: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut UBT_SERVICE_BLUEPRINT_BASE_RECEIVE_ACTIVATION_AI: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut UBT_SERVICE_BLUEPRINT_BASE_RECEIVE_ACTIVATION: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut UBT_SERVICE_BLUEPRINT_BASE_IS_SERVICE_ACTIVE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut UBT_TASK_BLUEPRINT_BASE_SET_FINISH_ON_MESSAGE_WITH_ID: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut UBT_TASK_BLUEPRINT_BASE_SET_FINISH_ON_MESSAGE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut UBT_TASK_BLUEPRINT_BASE_RECEIVE_TICK_AI: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut UBT_TASK_BLUEPRINT_BASE_RECEIVE_TICK: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut UBT_TASK_BLUEPRINT_BASE_RECEIVE_EXECUTE_AI: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut UBT_TASK_BLUEPRINT_BASE_RECEIVE_EXECUTE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut UBT_TASK_BLUEPRINT_BASE_RECEIVE_ABORT_AI: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut UBT_TASK_BLUEPRINT_BASE_RECEIVE_ABORT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut UBT_TASK_BLUEPRINT_BASE_IS_TASK_EXECUTING: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut UBT_TASK_BLUEPRINT_BASE_IS_TASK_ABORTING: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut UBT_TASK_BLUEPRINT_BASE_FINISH_EXECUTE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut UBT_TASK_BLUEPRINT_BASE_FINISH_ABORT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_VALUE_OR_BB_KEY_BLUEPRINT_UTILITY_GET_VECTOR: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_VALUE_OR_BB_KEY_BLUEPRINT_UTILITY_GET_STRUCT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_VALUE_OR_BB_KEY_BLUEPRINT_UTILITY_GET_STRING: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_VALUE_OR_BB_KEY_BLUEPRINT_UTILITY_GET_ROTATOR: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_VALUE_OR_BB_KEY_BLUEPRINT_UTILITY_GET_OBJECT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_VALUE_OR_BB_KEY_BLUEPRINT_UTILITY_GET_NAME: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_VALUE_OR_BB_KEY_BLUEPRINT_UTILITY_GET_INT32: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_VALUE_OR_BB_KEY_BLUEPRINT_UTILITY_GET_FLOAT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_VALUE_OR_BB_KEY_BLUEPRINT_UTILITY_GET_ENUM: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_VALUE_OR_BB_KEY_BLUEPRINT_UTILITY_GET_CLASS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_VALUE_OR_BB_KEY_BLUEPRINT_UTILITY_GET_BOOL: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut UAI_BLUEPRINT_HELPER_LIBRARY_UNLOCK_AI_RESOURCES_WITH_ANIMATION: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut UAI_BLUEPRINT_HELPER_LIBRARY_SPAWN_AI_FROM_CLASS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut UAI_BLUEPRINT_HELPER_LIBRARY_SIMPLE_MOVE_TO_LOCATION: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut UAI_BLUEPRINT_HELPER_LIBRARY_SIMPLE_MOVE_TO_ACTOR: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut UAI_BLUEPRINT_HELPER_LIBRARY_SEND_AI_MESSAGE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut UAI_BLUEPRINT_HELPER_LIBRARY_LOCK_AI_RESOURCES_WITH_ANIMATION: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut UAI_BLUEPRINT_HELPER_LIBRARY_IS_VALID_AI_ROTATION: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut UAI_BLUEPRINT_HELPER_LIBRARY_IS_VALID_AI_LOCATION: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut UAI_BLUEPRINT_HELPER_LIBRARY_IS_VALID_AI_DIRECTION: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut UAI_BLUEPRINT_HELPER_LIBRARY_GET_NEXT_NAV_LINK_INDEX: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut UAI_BLUEPRINT_HELPER_LIBRARY_GET_CURRENT_PATH_POINTS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut UAI_BLUEPRINT_HELPER_LIBRARY_GET_CURRENT_PATH_INDEX: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut UAI_BLUEPRINT_HELPER_LIBRARY_GET_CURRENT_PATH: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut UAI_BLUEPRINT_HELPER_LIBRARY_GET_BLACKBOARD: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut UAI_BLUEPRINT_HELPER_LIBRARY_GET_AI_CONTROLLER: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut UAI_BLUEPRINT_HELPER_LIBRARY_CREATE_MOVE_TO_PROXY_OBJECT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_ENV_QUERY_CONTEXT_BLUEPRINT_BASE_PROVIDE_SINGLE_LOCATION: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_ENV_QUERY_CONTEXT_BLUEPRINT_BASE_PROVIDE_SINGLE_ACTOR: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_ENV_QUERY_CONTEXT_BLUEPRINT_BASE_PROVIDE_LOCATIONS_SET: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_ENV_QUERY_CONTEXT_BLUEPRINT_BASE_PROVIDE_ACTORS_SET: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_ENV_QUERY_INSTANCE_BLUEPRINT_WRAPPER_SET_NAMED_PARAM: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_ENV_QUERY_INSTANCE_BLUEPRINT_WRAPPER_GET_RESULTS_AS_LOCATIONS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_ENV_QUERY_INSTANCE_BLUEPRINT_WRAPPER_GET_RESULTS_AS_ACTORS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_ENV_QUERY_INSTANCE_BLUEPRINT_WRAPPER_GET_QUERY_RESULTS_AS_LOCATIONS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_ENV_QUERY_INSTANCE_BLUEPRINT_WRAPPER_GET_QUERY_RESULTS_AS_ACTORS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_ENV_QUERY_INSTANCE_BLUEPRINT_WRAPPER_GET_ITEM_SCORE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_ENV_QUERY_INSTANCE_BLUEPRINT_WRAPPER_EQS_QUERY_DONE_SIGNATURE_DELEGATE_SIGNATURE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_ENV_QUERY_MANAGER_RUN_EQS_QUERY: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_ENV_QUERY_GENERATOR_BLUEPRINT_BASE_GET_QUERIER: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_ENV_QUERY_GENERATOR_BLUEPRINT_BASE_DO_ITEM_GENERATION_FROM_ACTORS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_ENV_QUERY_GENERATOR_BLUEPRINT_BASE_DO_ITEM_GENERATION: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_ENV_QUERY_GENERATOR_BLUEPRINT_BASE_ADD_GENERATED_VECTOR: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_ENV_QUERY_GENERATOR_BLUEPRINT_BASE_ADD_GENERATED_ACTOR: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_PATH_FOLLOWING_COMPONENT_ON_NAV_DATA_REGISTERED: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_PATH_FOLLOWING_COMPONENT_ON_ACTOR_BUMP: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_PATH_FOLLOWING_COMPONENT_GET_PATH_DESTINATION: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_PATH_FOLLOWING_COMPONENT_GET_PATH_ACTION_TYPE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_CROWD_FOLLOWING_COMPONENT_SUSPEND_CROWD_STEERING: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_GENERATED_NAV_LINKS_PROXY_RECEIVE_SMART_LINK_REACHED: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut A_NAV_LINK_PROXY_SET_SMART_LINK_ENABLED: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut A_NAV_LINK_PROXY_RESUME_PATH_FOLLOWING: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut A_NAV_LINK_PROXY_RECEIVE_SMART_LINK_REACHED: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut A_NAV_LINK_PROXY_IS_SMART_LINK_ENABLED: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut A_NAV_LINK_PROXY_HAS_MOVING_AGENTS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut A_NAV_LINK_PROXY_COPY_END_POINTS_FROM_SIMPLE_LINK_TO_SMART_LINK: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_NAV_LOCAL_GRID_MANAGER_SET_LOCAL_NAVIGATION_GRID_DENSITY: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_NAV_LOCAL_GRID_MANAGER_REMOVE_LOCAL_NAVIGATION_GRID: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_NAV_LOCAL_GRID_MANAGER_FIND_LOCAL_NAVIGATION_GRID_PATH: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_NAV_LOCAL_GRID_MANAGER_ADD_LOCAL_NAVIGATION_GRID_FOR_POINTS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_NAV_LOCAL_GRID_MANAGER_ADD_LOCAL_NAVIGATION_GRID_FOR_POINT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_NAV_LOCAL_GRID_MANAGER_ADD_LOCAL_NAVIGATION_GRID_FOR_CAPSULE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_NAV_LOCAL_GRID_MANAGER_ADD_LOCAL_NAVIGATION_GRID_FOR_BOX: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut UAI_PERCEPTION_COMPONENT_SET_SENSE_ENABLED: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut UAI_PERCEPTION_COMPONENT_REQUEST_STIMULI_LISTENER_UPDATE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut UAI_PERCEPTION_COMPONENT_ON_OWNER_END_PLAY: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut UAI_PERCEPTION_COMPONENT_IS_SENSE_ENABLED: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut UAI_PERCEPTION_COMPONENT_GET_PERCEIVED_HOSTILE_ACTORS_BY_SENSE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut UAI_PERCEPTION_COMPONENT_GET_PERCEIVED_HOSTILE_ACTORS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut UAI_PERCEPTION_COMPONENT_GET_KNOWN_PERCEIVED_ACTORS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut UAI_PERCEPTION_COMPONENT_GET_CURRENTLY_PERCEIVED_ACTORS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut UAI_PERCEPTION_COMPONENT_GET_ACTORS_PERCEPTION: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut UAI_PERCEPTION_COMPONENT_FORGET_ALL: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut UAI_PERCEPTION_STIMULI_SOURCE_COMPONENT_UNREGISTER_FROM_SENSE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut UAI_PERCEPTION_STIMULI_SOURCE_COMPONENT_UNREGISTER_FROM_PERCEPTION_SYSTEM: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut UAI_PERCEPTION_STIMULI_SOURCE_COMPONENT_REGISTER_WITH_PERCEPTION_SYSTEM: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut UAI_PERCEPTION_STIMULI_SOURCE_COMPONENT_REGISTER_FOR_SENSE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut UAI_PERCEPTION_SYSTEM_REPORT_PERCEPTION_EVENT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut UAI_PERCEPTION_SYSTEM_REPORT_EVENT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut UAI_PERCEPTION_SYSTEM_REGISTER_PERCEPTION_STIMULI_SOURCE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut UAI_PERCEPTION_SYSTEM_ON_PERCEPTION_STIMULI_SOURCE_END_PLAY: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut UAI_PERCEPTION_SYSTEM_GET_SENSE_CLASS_FOR_STIMULUS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut UAI_SENSE_BLUEPRINT_ON_UPDATE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut UAI_SENSE_BLUEPRINT_ON_LISTENER_UPDATED: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut UAI_SENSE_BLUEPRINT_ON_LISTENER_UNREGISTERED: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut UAI_SENSE_BLUEPRINT_ON_LISTENER_REGISTERED: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut UAI_SENSE_BLUEPRINT_K2_ON_NEW_PAWN: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut UAI_SENSE_BLUEPRINT_GET_ALL_LISTENER_COMPONENTS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut UAI_SENSE_BLUEPRINT_GET_ALL_LISTENER_ACTORS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut UAI_SENSE_DAMAGE_REPORT_DAMAGE_EVENT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut UAI_SENSE_HEARING_REPORT_NOISE_EVENT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut UAI_SENSE_PREDICTION_REQUEST_PAWN_PREDICTION_EVENT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut UAI_SENSE_PREDICTION_REQUEST_CONTROLLER_PREDICTION_EVENT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut UAI_SENSE_TOUCH_REPORT_TOUCH_EVENT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_PAWN_SENSING_COMPONENT_SET_SENSING_UPDATES_ENABLED: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_PAWN_SENSING_COMPONENT_SET_SENSING_INTERVAL: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_PAWN_SENSING_COMPONENT_SET_PERIPHERAL_VISION_ANGLE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_PAWN_SENSING_COMPONENT_SEE_PAWN_DELEGATE_DELEGATE_SIGNATURE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_PAWN_SENSING_COMPONENT_HEAR_NOISE_DELEGATE_DELEGATE_SIGNATURE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_PAWN_SENSING_COMPONENT_GET_PERIPHERAL_VISION_COSINE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_PAWN_SENSING_COMPONENT_GET_PERIPHERAL_VISION_ANGLE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut UAI_TASK_MOVE_TO_AI_MOVE_TO: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut UAI_TASK_RUN_EQS_RUN_EQS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
pub fn initialize() {
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UAIAsyncTaskBlueprintProxy::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("OnMoveCompleted"),
            &raw mut UAI_ASYNC_TASK_BLUEPRINT_PROXY_ON_MOVE_COMPLETED,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = AAIController::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("UseBlackboard"),
            &raw mut AAI_CONTROLLER_USE_BLACKBOARD,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("UnclaimTaskResource"),
            &raw mut AAI_CONTROLLER_UNCLAIM_TASK_RESOURCE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetPathFollowingComponent"),
            &raw mut AAI_CONTROLLER_SET_PATH_FOLLOWING_COMPONENT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetMoveBlockDetection"),
            &raw mut AAI_CONTROLLER_SET_MOVE_BLOCK_DETECTION,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RunBehaviorTree"),
            &raw mut AAI_CONTROLLER_RUN_BEHAVIOR_TREE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("OnUsingBlackBoard"),
            &raw mut AAI_CONTROLLER_ON_USING_BLACK_BOARD,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("OnGameplayTaskResourcesClaimed"),
            &raw mut AAI_CONTROLLER_ON_GAMEPLAY_TASK_RESOURCES_CLAIMED,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("MoveToLocation"),
            &raw mut AAI_CONTROLLER_MOVE_TO_LOCATION,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("MoveToActor"),
            &raw mut AAI_CONTROLLER_MOVE_TO_ACTOR,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("K2_SetFocus"),
            &raw mut AAI_CONTROLLER_K2_SET_FOCUS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("K2_SetFocalPoint"),
            &raw mut AAI_CONTROLLER_K2_SET_FOCAL_POINT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("K2_ClearFocus"),
            &raw mut AAI_CONTROLLER_K2_CLEAR_FOCUS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("HasPartialPath"),
            &raw mut AAI_CONTROLLER_HAS_PARTIAL_PATH,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetPathFollowingComponent"),
            &raw mut AAI_CONTROLLER_GET_PATH_FOLLOWING_COMPONENT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetMoveStatus"),
            &raw mut AAI_CONTROLLER_GET_MOVE_STATUS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetImmediateMoveDestination"),
            &raw mut AAI_CONTROLLER_GET_IMMEDIATE_MOVE_DESTINATION,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetFocusActor"),
            &raw mut AAI_CONTROLLER_GET_FOCUS_ACTOR,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetFocalPointOnActor"),
            &raw mut AAI_CONTROLLER_GET_FOCAL_POINT_ON_ACTOR,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetFocalPoint"),
            &raw mut AAI_CONTROLLER_GET_FOCAL_POINT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetAIPerceptionComponent"),
            &raw mut AAI_CONTROLLER_GET_AI_PERCEPTION_COMPONENT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ClaimTaskResource"),
            &raw mut AAI_CONTROLLER_CLAIM_TASK_RESOURCE,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UAISystem::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AILoggingVerbose"),
            &raw mut UAI_SYSTEM_AI_LOGGING_VERBOSE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AIIgnorePlayers"),
            &raw mut UAI_SYSTEM_AI_IGNORE_PLAYERS,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UBrainComponent::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("StopLogic"),
            &raw mut U_BRAIN_COMPONENT_STOP_LOGIC,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("StartLogic"),
            &raw mut U_BRAIN_COMPONENT_START_LOGIC,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RestartLogic"),
            &raw mut U_BRAIN_COMPONENT_RESTART_LOGIC,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("IsRunning"),
            &raw mut U_BRAIN_COMPONENT_IS_RUNNING,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("IsPaused"),
            &raw mut U_BRAIN_COMPONENT_IS_PAUSED,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UBehaviorTreeComponent::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetDynamicSubtree"),
            &raw mut U_BEHAVIOR_TREE_COMPONENT_SET_DYNAMIC_SUBTREE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetTagCooldownEndTime"),
            &raw mut U_BEHAVIOR_TREE_COMPONENT_GET_TAG_COOLDOWN_END_TIME,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AddCooldownTagDuration"),
            &raw mut U_BEHAVIOR_TREE_COMPONENT_ADD_COOLDOWN_TAG_DURATION,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UBlackboardAssetProvider::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetBlackboardAsset"),
            &raw mut U_BLACKBOARD_ASSET_PROVIDER_GET_BLACKBOARD_ASSET,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UBlackboardComponent::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetValueAsVector"),
            &raw mut U_BLACKBOARD_COMPONENT_SET_VALUE_AS_VECTOR,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetValueAsString"),
            &raw mut U_BLACKBOARD_COMPONENT_SET_VALUE_AS_STRING,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetValueAsRotator"),
            &raw mut U_BLACKBOARD_COMPONENT_SET_VALUE_AS_ROTATOR,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetValueAsObject"),
            &raw mut U_BLACKBOARD_COMPONENT_SET_VALUE_AS_OBJECT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetValueAsName"),
            &raw mut U_BLACKBOARD_COMPONENT_SET_VALUE_AS_NAME,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetValueAsInt"),
            &raw mut U_BLACKBOARD_COMPONENT_SET_VALUE_AS_INT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetValueAsFloat"),
            &raw mut U_BLACKBOARD_COMPONENT_SET_VALUE_AS_FLOAT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetValueAsEnum"),
            &raw mut U_BLACKBOARD_COMPONENT_SET_VALUE_AS_ENUM,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetValueAsClass"),
            &raw mut U_BLACKBOARD_COMPONENT_SET_VALUE_AS_CLASS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetValueAsBool"),
            &raw mut U_BLACKBOARD_COMPONENT_SET_VALUE_AS_BOOL,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("IsVectorValueSet"),
            &raw mut U_BLACKBOARD_COMPONENT_IS_VECTOR_VALUE_SET,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetValueAsVector"),
            &raw mut U_BLACKBOARD_COMPONENT_GET_VALUE_AS_VECTOR,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetValueAsString"),
            &raw mut U_BLACKBOARD_COMPONENT_GET_VALUE_AS_STRING,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetValueAsRotator"),
            &raw mut U_BLACKBOARD_COMPONENT_GET_VALUE_AS_ROTATOR,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetValueAsObject"),
            &raw mut U_BLACKBOARD_COMPONENT_GET_VALUE_AS_OBJECT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetValueAsName"),
            &raw mut U_BLACKBOARD_COMPONENT_GET_VALUE_AS_NAME,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetValueAsInt"),
            &raw mut U_BLACKBOARD_COMPONENT_GET_VALUE_AS_INT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetValueAsFloat"),
            &raw mut U_BLACKBOARD_COMPONENT_GET_VALUE_AS_FLOAT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetValueAsEnum"),
            &raw mut U_BLACKBOARD_COMPONENT_GET_VALUE_AS_ENUM,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetValueAsClass"),
            &raw mut U_BLACKBOARD_COMPONENT_GET_VALUE_AS_CLASS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetValueAsBool"),
            &raw mut U_BLACKBOARD_COMPONENT_GET_VALUE_AS_BOOL,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetRotationFromEntry"),
            &raw mut U_BLACKBOARD_COMPONENT_GET_ROTATION_FROM_ENTRY,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetLocationFromEntry"),
            &raw mut U_BLACKBOARD_COMPONENT_GET_LOCATION_FROM_ENTRY,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ClearValue"),
            &raw mut U_BLACKBOARD_COMPONENT_CLEAR_VALUE,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UBTFunctionLibrary::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("StopUsingExternalEvent"),
            &raw mut UBT_FUNCTION_LIBRARY_STOP_USING_EXTERNAL_EVENT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("StartUsingExternalEvent"),
            &raw mut UBT_FUNCTION_LIBRARY_START_USING_EXTERNAL_EVENT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetBlackboardValueAsVector"),
            &raw mut UBT_FUNCTION_LIBRARY_SET_BLACKBOARD_VALUE_AS_VECTOR,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetBlackboardValueAsString"),
            &raw mut UBT_FUNCTION_LIBRARY_SET_BLACKBOARD_VALUE_AS_STRING,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetBlackboardValueAsRotator"),
            &raw mut UBT_FUNCTION_LIBRARY_SET_BLACKBOARD_VALUE_AS_ROTATOR,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetBlackboardValueAsObject"),
            &raw mut UBT_FUNCTION_LIBRARY_SET_BLACKBOARD_VALUE_AS_OBJECT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetBlackboardValueAsName"),
            &raw mut UBT_FUNCTION_LIBRARY_SET_BLACKBOARD_VALUE_AS_NAME,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetBlackboardValueAsInt"),
            &raw mut UBT_FUNCTION_LIBRARY_SET_BLACKBOARD_VALUE_AS_INT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetBlackboardValueAsFloat"),
            &raw mut UBT_FUNCTION_LIBRARY_SET_BLACKBOARD_VALUE_AS_FLOAT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetBlackboardValueAsEnum"),
            &raw mut UBT_FUNCTION_LIBRARY_SET_BLACKBOARD_VALUE_AS_ENUM,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetBlackboardValueAsClass"),
            &raw mut UBT_FUNCTION_LIBRARY_SET_BLACKBOARD_VALUE_AS_CLASS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetBlackboardValueAsBool"),
            &raw mut UBT_FUNCTION_LIBRARY_SET_BLACKBOARD_VALUE_AS_BOOL,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetOwnersBlackboard"),
            &raw mut UBT_FUNCTION_LIBRARY_GET_OWNERS_BLACKBOARD,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetOwnerComponent"),
            &raw mut UBT_FUNCTION_LIBRARY_GET_OWNER_COMPONENT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetBlackboardValueAsVector"),
            &raw mut UBT_FUNCTION_LIBRARY_GET_BLACKBOARD_VALUE_AS_VECTOR,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetBlackboardValueAsString"),
            &raw mut UBT_FUNCTION_LIBRARY_GET_BLACKBOARD_VALUE_AS_STRING,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetBlackboardValueAsRotator"),
            &raw mut UBT_FUNCTION_LIBRARY_GET_BLACKBOARD_VALUE_AS_ROTATOR,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetBlackboardValueAsObject"),
            &raw mut UBT_FUNCTION_LIBRARY_GET_BLACKBOARD_VALUE_AS_OBJECT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetBlackboardValueAsName"),
            &raw mut UBT_FUNCTION_LIBRARY_GET_BLACKBOARD_VALUE_AS_NAME,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetBlackboardValueAsInt"),
            &raw mut UBT_FUNCTION_LIBRARY_GET_BLACKBOARD_VALUE_AS_INT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetBlackboardValueAsFloat"),
            &raw mut UBT_FUNCTION_LIBRARY_GET_BLACKBOARD_VALUE_AS_FLOAT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetBlackboardValueAsEnum"),
            &raw mut UBT_FUNCTION_LIBRARY_GET_BLACKBOARD_VALUE_AS_ENUM,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetBlackboardValueAsClass"),
            &raw mut UBT_FUNCTION_LIBRARY_GET_BLACKBOARD_VALUE_AS_CLASS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetBlackboardValueAsBool"),
            &raw mut UBT_FUNCTION_LIBRARY_GET_BLACKBOARD_VALUE_AS_BOOL,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetBlackboardValueAsActor"),
            &raw mut UBT_FUNCTION_LIBRARY_GET_BLACKBOARD_VALUE_AS_ACTOR,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ClearBlackboardValueAsVector"),
            &raw mut UBT_FUNCTION_LIBRARY_CLEAR_BLACKBOARD_VALUE_AS_VECTOR,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ClearBlackboardValue"),
            &raw mut UBT_FUNCTION_LIBRARY_CLEAR_BLACKBOARD_VALUE,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UBTDecorator_BlueprintBase::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ReceiveTickAI"),
            &raw mut UBT_DECORATOR_BLUEPRINT_BASE_RECEIVE_TICK_AI,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ReceiveTick"),
            &raw mut UBT_DECORATOR_BLUEPRINT_BASE_RECEIVE_TICK,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ReceiveObserverDeactivatedAI"),
            &raw mut UBT_DECORATOR_BLUEPRINT_BASE_RECEIVE_OBSERVER_DEACTIVATED_AI,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ReceiveObserverDeactivated"),
            &raw mut UBT_DECORATOR_BLUEPRINT_BASE_RECEIVE_OBSERVER_DEACTIVATED,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ReceiveObserverActivatedAI"),
            &raw mut UBT_DECORATOR_BLUEPRINT_BASE_RECEIVE_OBSERVER_ACTIVATED_AI,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ReceiveObserverActivated"),
            &raw mut UBT_DECORATOR_BLUEPRINT_BASE_RECEIVE_OBSERVER_ACTIVATED,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ReceiveExecutionStartAI"),
            &raw mut UBT_DECORATOR_BLUEPRINT_BASE_RECEIVE_EXECUTION_START_AI,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ReceiveExecutionStart"),
            &raw mut UBT_DECORATOR_BLUEPRINT_BASE_RECEIVE_EXECUTION_START,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ReceiveExecutionFinishAI"),
            &raw mut UBT_DECORATOR_BLUEPRINT_BASE_RECEIVE_EXECUTION_FINISH_AI,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ReceiveExecutionFinish"),
            &raw mut UBT_DECORATOR_BLUEPRINT_BASE_RECEIVE_EXECUTION_FINISH,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("PerformConditionCheckAI"),
            &raw mut UBT_DECORATOR_BLUEPRINT_BASE_PERFORM_CONDITION_CHECK_AI,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("PerformConditionCheck"),
            &raw mut UBT_DECORATOR_BLUEPRINT_BASE_PERFORM_CONDITION_CHECK,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("IsDecoratorObserverActive"),
            &raw mut UBT_DECORATOR_BLUEPRINT_BASE_IS_DECORATOR_OBSERVER_ACTIVE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("IsDecoratorExecutionActive"),
            &raw mut UBT_DECORATOR_BLUEPRINT_BASE_IS_DECORATOR_EXECUTION_ACTIVE,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UBTService_BlueprintBase::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ReceiveTickAI"),
            &raw mut UBT_SERVICE_BLUEPRINT_BASE_RECEIVE_TICK_AI,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ReceiveTick"),
            &raw mut UBT_SERVICE_BLUEPRINT_BASE_RECEIVE_TICK,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ReceiveSearchStartAI"),
            &raw mut UBT_SERVICE_BLUEPRINT_BASE_RECEIVE_SEARCH_START_AI,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ReceiveSearchStart"),
            &raw mut UBT_SERVICE_BLUEPRINT_BASE_RECEIVE_SEARCH_START,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ReceiveDeactivationAI"),
            &raw mut UBT_SERVICE_BLUEPRINT_BASE_RECEIVE_DEACTIVATION_AI,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ReceiveDeactivation"),
            &raw mut UBT_SERVICE_BLUEPRINT_BASE_RECEIVE_DEACTIVATION,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ReceiveActivationAI"),
            &raw mut UBT_SERVICE_BLUEPRINT_BASE_RECEIVE_ACTIVATION_AI,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ReceiveActivation"),
            &raw mut UBT_SERVICE_BLUEPRINT_BASE_RECEIVE_ACTIVATION,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("IsServiceActive"),
            &raw mut UBT_SERVICE_BLUEPRINT_BASE_IS_SERVICE_ACTIVE,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UBTTask_BlueprintBase::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetFinishOnMessageWithId"),
            &raw mut UBT_TASK_BLUEPRINT_BASE_SET_FINISH_ON_MESSAGE_WITH_ID,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetFinishOnMessage"),
            &raw mut UBT_TASK_BLUEPRINT_BASE_SET_FINISH_ON_MESSAGE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ReceiveTickAI"),
            &raw mut UBT_TASK_BLUEPRINT_BASE_RECEIVE_TICK_AI,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ReceiveTick"),
            &raw mut UBT_TASK_BLUEPRINT_BASE_RECEIVE_TICK,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ReceiveExecuteAI"),
            &raw mut UBT_TASK_BLUEPRINT_BASE_RECEIVE_EXECUTE_AI,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ReceiveExecute"),
            &raw mut UBT_TASK_BLUEPRINT_BASE_RECEIVE_EXECUTE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ReceiveAbortAI"),
            &raw mut UBT_TASK_BLUEPRINT_BASE_RECEIVE_ABORT_AI,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ReceiveAbort"),
            &raw mut UBT_TASK_BLUEPRINT_BASE_RECEIVE_ABORT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("IsTaskExecuting"),
            &raw mut UBT_TASK_BLUEPRINT_BASE_IS_TASK_EXECUTING,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("IsTaskAborting"),
            &raw mut UBT_TASK_BLUEPRINT_BASE_IS_TASK_ABORTING,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("FinishExecute"),
            &raw mut UBT_TASK_BLUEPRINT_BASE_FINISH_EXECUTE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("FinishAbort"),
            &raw mut UBT_TASK_BLUEPRINT_BASE_FINISH_ABORT,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UValueOrBBKeyBlueprintUtility::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetVector"),
            &raw mut U_VALUE_OR_BB_KEY_BLUEPRINT_UTILITY_GET_VECTOR,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetStruct"),
            &raw mut U_VALUE_OR_BB_KEY_BLUEPRINT_UTILITY_GET_STRUCT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetString"),
            &raw mut U_VALUE_OR_BB_KEY_BLUEPRINT_UTILITY_GET_STRING,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetRotator"),
            &raw mut U_VALUE_OR_BB_KEY_BLUEPRINT_UTILITY_GET_ROTATOR,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetObject"),
            &raw mut U_VALUE_OR_BB_KEY_BLUEPRINT_UTILITY_GET_OBJECT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetName"),
            &raw mut U_VALUE_OR_BB_KEY_BLUEPRINT_UTILITY_GET_NAME,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetInt32"),
            &raw mut U_VALUE_OR_BB_KEY_BLUEPRINT_UTILITY_GET_INT32,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetFloat"),
            &raw mut U_VALUE_OR_BB_KEY_BLUEPRINT_UTILITY_GET_FLOAT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetEnum"),
            &raw mut U_VALUE_OR_BB_KEY_BLUEPRINT_UTILITY_GET_ENUM,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetClass"),
            &raw mut U_VALUE_OR_BB_KEY_BLUEPRINT_UTILITY_GET_CLASS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetBool"),
            &raw mut U_VALUE_OR_BB_KEY_BLUEPRINT_UTILITY_GET_BOOL,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UAIBlueprintHelperLibrary::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("UnlockAIResourcesWithAnimation"),
            &raw mut UAI_BLUEPRINT_HELPER_LIBRARY_UNLOCK_AI_RESOURCES_WITH_ANIMATION,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SpawnAIFromClass"),
            &raw mut UAI_BLUEPRINT_HELPER_LIBRARY_SPAWN_AI_FROM_CLASS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SimpleMoveToLocation"),
            &raw mut UAI_BLUEPRINT_HELPER_LIBRARY_SIMPLE_MOVE_TO_LOCATION,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SimpleMoveToActor"),
            &raw mut UAI_BLUEPRINT_HELPER_LIBRARY_SIMPLE_MOVE_TO_ACTOR,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SendAIMessage"),
            &raw mut UAI_BLUEPRINT_HELPER_LIBRARY_SEND_AI_MESSAGE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("LockAIResourcesWithAnimation"),
            &raw mut UAI_BLUEPRINT_HELPER_LIBRARY_LOCK_AI_RESOURCES_WITH_ANIMATION,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("IsValidAIRotation"),
            &raw mut UAI_BLUEPRINT_HELPER_LIBRARY_IS_VALID_AI_ROTATION,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("IsValidAILocation"),
            &raw mut UAI_BLUEPRINT_HELPER_LIBRARY_IS_VALID_AI_LOCATION,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("IsValidAIDirection"),
            &raw mut UAI_BLUEPRINT_HELPER_LIBRARY_IS_VALID_AI_DIRECTION,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetNextNavLinkIndex"),
            &raw mut UAI_BLUEPRINT_HELPER_LIBRARY_GET_NEXT_NAV_LINK_INDEX,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetCurrentPathPoints"),
            &raw mut UAI_BLUEPRINT_HELPER_LIBRARY_GET_CURRENT_PATH_POINTS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetCurrentPathIndex"),
            &raw mut UAI_BLUEPRINT_HELPER_LIBRARY_GET_CURRENT_PATH_INDEX,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetCurrentPath"),
            &raw mut UAI_BLUEPRINT_HELPER_LIBRARY_GET_CURRENT_PATH,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetBlackboard"),
            &raw mut UAI_BLUEPRINT_HELPER_LIBRARY_GET_BLACKBOARD,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetAIController"),
            &raw mut UAI_BLUEPRINT_HELPER_LIBRARY_GET_AI_CONTROLLER,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CreateMoveToProxyObject"),
            &raw mut UAI_BLUEPRINT_HELPER_LIBRARY_CREATE_MOVE_TO_PROXY_OBJECT,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UEnvQueryContext_BlueprintBase::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ProvideSingleLocation"),
            &raw mut U_ENV_QUERY_CONTEXT_BLUEPRINT_BASE_PROVIDE_SINGLE_LOCATION,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ProvideSingleActor"),
            &raw mut U_ENV_QUERY_CONTEXT_BLUEPRINT_BASE_PROVIDE_SINGLE_ACTOR,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ProvideLocationsSet"),
            &raw mut U_ENV_QUERY_CONTEXT_BLUEPRINT_BASE_PROVIDE_LOCATIONS_SET,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ProvideActorsSet"),
            &raw mut U_ENV_QUERY_CONTEXT_BLUEPRINT_BASE_PROVIDE_ACTORS_SET,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UEnvQueryInstanceBlueprintWrapper::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetNamedParam"),
            &raw mut U_ENV_QUERY_INSTANCE_BLUEPRINT_WRAPPER_SET_NAMED_PARAM,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetResultsAsLocations"),
            &raw mut U_ENV_QUERY_INSTANCE_BLUEPRINT_WRAPPER_GET_RESULTS_AS_LOCATIONS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetResultsAsActors"),
            &raw mut U_ENV_QUERY_INSTANCE_BLUEPRINT_WRAPPER_GET_RESULTS_AS_ACTORS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetQueryResultsAsLocations"),
            &raw mut U_ENV_QUERY_INSTANCE_BLUEPRINT_WRAPPER_GET_QUERY_RESULTS_AS_LOCATIONS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetQueryResultsAsActors"),
            &raw mut U_ENV_QUERY_INSTANCE_BLUEPRINT_WRAPPER_GET_QUERY_RESULTS_AS_ACTORS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetItemScore"),
            &raw mut U_ENV_QUERY_INSTANCE_BLUEPRINT_WRAPPER_GET_ITEM_SCORE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("EQSQueryDoneSignature__DelegateSignature"),
            &raw mut U_ENV_QUERY_INSTANCE_BLUEPRINT_WRAPPER_EQS_QUERY_DONE_SIGNATURE_DELEGATE_SIGNATURE,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UEnvQueryManager::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RunEQSQuery"),
            &raw mut U_ENV_QUERY_MANAGER_RUN_EQS_QUERY,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UEnvQueryGenerator_BlueprintBase::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetQuerier"),
            &raw mut U_ENV_QUERY_GENERATOR_BLUEPRINT_BASE_GET_QUERIER,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("DoItemGenerationFromActors"),
            &raw mut U_ENV_QUERY_GENERATOR_BLUEPRINT_BASE_DO_ITEM_GENERATION_FROM_ACTORS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("DoItemGeneration"),
            &raw mut U_ENV_QUERY_GENERATOR_BLUEPRINT_BASE_DO_ITEM_GENERATION,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AddGeneratedVector"),
            &raw mut U_ENV_QUERY_GENERATOR_BLUEPRINT_BASE_ADD_GENERATED_VECTOR,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AddGeneratedActor"),
            &raw mut U_ENV_QUERY_GENERATOR_BLUEPRINT_BASE_ADD_GENERATED_ACTOR,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UPathFollowingComponent::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("OnNavDataRegistered"),
            &raw mut U_PATH_FOLLOWING_COMPONENT_ON_NAV_DATA_REGISTERED,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("OnActorBump"),
            &raw mut U_PATH_FOLLOWING_COMPONENT_ON_ACTOR_BUMP,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetPathDestination"),
            &raw mut U_PATH_FOLLOWING_COMPONENT_GET_PATH_DESTINATION,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetPathActionType"),
            &raw mut U_PATH_FOLLOWING_COMPONENT_GET_PATH_ACTION_TYPE,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UCrowdFollowingComponent::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SuspendCrowdSteering"),
            &raw mut U_CROWD_FOLLOWING_COMPONENT_SUSPEND_CROWD_STEERING,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UGeneratedNavLinksProxy::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ReceiveSmartLinkReached"),
            &raw mut U_GENERATED_NAV_LINKS_PROXY_RECEIVE_SMART_LINK_REACHED,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = ANavLinkProxy::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetSmartLinkEnabled"),
            &raw mut A_NAV_LINK_PROXY_SET_SMART_LINK_ENABLED,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ResumePathFollowing"),
            &raw mut A_NAV_LINK_PROXY_RESUME_PATH_FOLLOWING,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ReceiveSmartLinkReached"),
            &raw mut A_NAV_LINK_PROXY_RECEIVE_SMART_LINK_REACHED,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("IsSmartLinkEnabled"),
            &raw mut A_NAV_LINK_PROXY_IS_SMART_LINK_ENABLED,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("HasMovingAgents"),
            &raw mut A_NAV_LINK_PROXY_HAS_MOVING_AGENTS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CopyEndPointsFromSimpleLinkToSmartLink"),
            &raw mut A_NAV_LINK_PROXY_COPY_END_POINTS_FROM_SIMPLE_LINK_TO_SMART_LINK,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UNavLocalGridManager::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetLocalNavigationGridDensity"),
            &raw mut U_NAV_LOCAL_GRID_MANAGER_SET_LOCAL_NAVIGATION_GRID_DENSITY,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RemoveLocalNavigationGrid"),
            &raw mut U_NAV_LOCAL_GRID_MANAGER_REMOVE_LOCAL_NAVIGATION_GRID,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("FindLocalNavigationGridPath"),
            &raw mut U_NAV_LOCAL_GRID_MANAGER_FIND_LOCAL_NAVIGATION_GRID_PATH,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AddLocalNavigationGridForPoints"),
            &raw mut U_NAV_LOCAL_GRID_MANAGER_ADD_LOCAL_NAVIGATION_GRID_FOR_POINTS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AddLocalNavigationGridForPoint"),
            &raw mut U_NAV_LOCAL_GRID_MANAGER_ADD_LOCAL_NAVIGATION_GRID_FOR_POINT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AddLocalNavigationGridForCapsule"),
            &raw mut U_NAV_LOCAL_GRID_MANAGER_ADD_LOCAL_NAVIGATION_GRID_FOR_CAPSULE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AddLocalNavigationGridForBox"),
            &raw mut U_NAV_LOCAL_GRID_MANAGER_ADD_LOCAL_NAVIGATION_GRID_FOR_BOX,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UAIPerceptionComponent::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetSenseEnabled"),
            &raw mut UAI_PERCEPTION_COMPONENT_SET_SENSE_ENABLED,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RequestStimuliListenerUpdate"),
            &raw mut UAI_PERCEPTION_COMPONENT_REQUEST_STIMULI_LISTENER_UPDATE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("OnOwnerEndPlay"),
            &raw mut UAI_PERCEPTION_COMPONENT_ON_OWNER_END_PLAY,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("IsSenseEnabled"),
            &raw mut UAI_PERCEPTION_COMPONENT_IS_SENSE_ENABLED,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetPerceivedHostileActorsBySense"),
            &raw mut UAI_PERCEPTION_COMPONENT_GET_PERCEIVED_HOSTILE_ACTORS_BY_SENSE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetPerceivedHostileActors"),
            &raw mut UAI_PERCEPTION_COMPONENT_GET_PERCEIVED_HOSTILE_ACTORS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetKnownPerceivedActors"),
            &raw mut UAI_PERCEPTION_COMPONENT_GET_KNOWN_PERCEIVED_ACTORS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetCurrentlyPerceivedActors"),
            &raw mut UAI_PERCEPTION_COMPONENT_GET_CURRENTLY_PERCEIVED_ACTORS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetActorsPerception"),
            &raw mut UAI_PERCEPTION_COMPONENT_GET_ACTORS_PERCEPTION,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ForgetAll"),
            &raw mut UAI_PERCEPTION_COMPONENT_FORGET_ALL,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UAIPerceptionStimuliSourceComponent::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("UnregisterFromSense"),
            &raw mut UAI_PERCEPTION_STIMULI_SOURCE_COMPONENT_UNREGISTER_FROM_SENSE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("UnregisterFromPerceptionSystem"),
            &raw mut UAI_PERCEPTION_STIMULI_SOURCE_COMPONENT_UNREGISTER_FROM_PERCEPTION_SYSTEM,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RegisterWithPerceptionSystem"),
            &raw mut UAI_PERCEPTION_STIMULI_SOURCE_COMPONENT_REGISTER_WITH_PERCEPTION_SYSTEM,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RegisterForSense"),
            &raw mut UAI_PERCEPTION_STIMULI_SOURCE_COMPONENT_REGISTER_FOR_SENSE,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UAIPerceptionSystem::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ReportPerceptionEvent"),
            &raw mut UAI_PERCEPTION_SYSTEM_REPORT_PERCEPTION_EVENT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ReportEvent"),
            &raw mut UAI_PERCEPTION_SYSTEM_REPORT_EVENT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RegisterPerceptionStimuliSource"),
            &raw mut UAI_PERCEPTION_SYSTEM_REGISTER_PERCEPTION_STIMULI_SOURCE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("OnPerceptionStimuliSourceEndPlay"),
            &raw mut UAI_PERCEPTION_SYSTEM_ON_PERCEPTION_STIMULI_SOURCE_END_PLAY,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetSenseClassForStimulus"),
            &raw mut UAI_PERCEPTION_SYSTEM_GET_SENSE_CLASS_FOR_STIMULUS,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UAISense_Blueprint::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("OnUpdate"),
            &raw mut UAI_SENSE_BLUEPRINT_ON_UPDATE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("OnListenerUpdated"),
            &raw mut UAI_SENSE_BLUEPRINT_ON_LISTENER_UPDATED,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("OnListenerUnregistered"),
            &raw mut UAI_SENSE_BLUEPRINT_ON_LISTENER_UNREGISTERED,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("OnListenerRegistered"),
            &raw mut UAI_SENSE_BLUEPRINT_ON_LISTENER_REGISTERED,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("K2_OnNewPawn"),
            &raw mut UAI_SENSE_BLUEPRINT_K2_ON_NEW_PAWN,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetAllListenerComponents"),
            &raw mut UAI_SENSE_BLUEPRINT_GET_ALL_LISTENER_COMPONENTS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetAllListenerActors"),
            &raw mut UAI_SENSE_BLUEPRINT_GET_ALL_LISTENER_ACTORS,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UAISense_Damage::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ReportDamageEvent"),
            &raw mut UAI_SENSE_DAMAGE_REPORT_DAMAGE_EVENT,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UAISense_Hearing::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ReportNoiseEvent"),
            &raw mut UAI_SENSE_HEARING_REPORT_NOISE_EVENT,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UAISense_Prediction::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RequestPawnPredictionEvent"),
            &raw mut UAI_SENSE_PREDICTION_REQUEST_PAWN_PREDICTION_EVENT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RequestControllerPredictionEvent"),
            &raw mut UAI_SENSE_PREDICTION_REQUEST_CONTROLLER_PREDICTION_EVENT,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UAISense_Touch::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ReportTouchEvent"),
            &raw mut UAI_SENSE_TOUCH_REPORT_TOUCH_EVENT,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UPawnSensingComponent::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetSensingUpdatesEnabled"),
            &raw mut U_PAWN_SENSING_COMPONENT_SET_SENSING_UPDATES_ENABLED,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetSensingInterval"),
            &raw mut U_PAWN_SENSING_COMPONENT_SET_SENSING_INTERVAL,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetPeripheralVisionAngle"),
            &raw mut U_PAWN_SENSING_COMPONENT_SET_PERIPHERAL_VISION_ANGLE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SeePawnDelegate__DelegateSignature"),
            &raw mut U_PAWN_SENSING_COMPONENT_SEE_PAWN_DELEGATE_DELEGATE_SIGNATURE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("HearNoiseDelegate__DelegateSignature"),
            &raw mut U_PAWN_SENSING_COMPONENT_HEAR_NOISE_DELEGATE_DELEGATE_SIGNATURE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetPeripheralVisionCosine"),
            &raw mut U_PAWN_SENSING_COMPONENT_GET_PERIPHERAL_VISION_COSINE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetPeripheralVisionAngle"),
            &raw mut U_PAWN_SENSING_COMPONENT_GET_PERIPHERAL_VISION_ANGLE,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UAITask_MoveTo::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AIMoveTo"),
            &raw mut UAI_TASK_MOVE_TO_AI_MOVE_TO,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UAITask_RunEQS::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RunEQS"),
            &raw mut UAI_TASK_RUN_EQS_RUN_EQS,
        );
    }
}
#[repr(C, align(8))]
pub struct FActorPerceptionUpdateInfo {
    pub target_id: i32,
    pub target: TWeakObjectPtr<crate::bindings::engine::AActor>,
    pub stimulus: FAIStimulus,
}
impl FActorPerceptionUpdateInfo {}
#[repr(C, align(8))]
pub struct FAIStimulus {
    pub age: f32,
    pub expiration_age: f32,
    pub strength: f32,
    pub stimulus_location: crate::bindings::core_u_object::FVector,
    pub receiver_location: crate::bindings::core_u_object::FVector,
    pub tag: FName,
    #[doc(hidden)]
    __padding_92: [u8; 16],
    pub flags_92: u8,
    __padding_end: [u8; 3],
}
impl FAIStimulus {}
#[repr(C, align(4))]
pub struct FAIRequestID {
    __padding_end: [u8; 4],
}
impl FAIRequestID {}
#[repr(C, align(1))]
pub struct FGenericTeamId {
    pub team_id: u8,
}
impl FGenericTeamId {}
#[repr(C, align(8))]
pub struct FBlackboardKeySelector {
    pub allowed_types: TArray<UPtr<UBlackboardKeyType>>,
    pub selected_key_name: FName,
    pub selected_key_type: TSubclassOf<UBlackboardKeyType>,
    pub selected_key_id: i32,
    pub flags_44: u8,
    __padding_end: [u8; 3],
}
impl FBlackboardKeySelector {}
#[repr(C, align(8))]
pub struct FValueOrBlackboardKeyBase {
    #[doc(hidden)]
    __padding_8: [u8; 8],
    pub key: FName,
    __padding_end: [u8; 4],
}
impl FValueOrBlackboardKeyBase {}
#[repr(C, align(8))]
pub struct FValueOrBBKey_Bool {
    __padding_end: [u8; 32],
}
impl FValueOrBBKey_Bool {}
#[repr(C, align(8))]
pub struct FValueOrBBKey_Class {
    __padding_end: [u8; 40],
}
impl FValueOrBBKey_Class {}
#[repr(C, align(8))]
pub struct FValueOrBBKey_Enum {
    __padding_end: [u8; 56],
}
impl FValueOrBBKey_Enum {}
#[repr(C, align(8))]
pub struct FValueOrBBKey_Float {
    __padding_end: [u8; 32],
}
impl FValueOrBBKey_Float {}
#[repr(C, align(8))]
pub struct FValueOrBBKey_Int32 {
    __padding_end: [u8; 32],
}
impl FValueOrBBKey_Int32 {}
#[repr(C, align(8))]
pub struct FValueOrBBKey_Name {
    __padding_end: [u8; 40],
}
impl FValueOrBBKey_Name {}
#[repr(C, align(8))]
pub struct FValueOrBBKey_String {
    __padding_end: [u8; 40],
}
impl FValueOrBBKey_String {}
#[repr(C, align(8))]
pub struct FValueOrBBKey_Object {
    __padding_end: [u8; 40],
}
impl FValueOrBBKey_Object {}
#[repr(C, align(8))]
pub struct FValueOrBBKey_Rotator {
    __padding_end: [u8; 48],
}
impl FValueOrBBKey_Rotator {}
#[repr(C, align(8))]
pub struct FValueOrBBKey_Vector {
    __padding_end: [u8; 48],
}
impl FValueOrBBKey_Vector {}
#[repr(C, align(8))]
pub struct FValueOrBBKey_Struct {
    __padding_end: [u8; 48],
}
impl FValueOrBBKey_Struct {}
#[repr(C, align(8))]
pub struct FEnvQueryResult {
    #[doc(hidden)]
    __padding_16: [u8; 16],
    pub item_type: TSubclassOf<UEnvQueryItemType>,
    #[doc(hidden)]
    __padding_44: [u8; 20],
    pub option_index: i32,
    pub query_id: i32,
    __padding_end: [u8; 12],
}
impl FEnvQueryResult {}
#[repr(C, align(8))]
pub struct FEnvQueryInstance {
    __padding_end: [u8; 448],
}
impl FEnvQueryInstance {}
#[repr(C, align(4))]
pub struct FEnvNamedValue {
    pub param_name: FName,
    pub param_type: EAIParamType,
    pub value: f32,
}
impl FEnvNamedValue {}
#[repr(C, align(8))]
pub struct FAIDynamicParam {
    pub param_name: FName,
    pub param_type: EAIParamType,
    pub flags_13: u8,
    pub value: f32,
    pub bb_key: FBlackboardKeySelector,
}
impl FAIDynamicParam {}
#[repr(C, align(8))]
pub struct FActorPerceptionBlueprintInfo {
    pub target: UPtr<crate::bindings::engine::AActor>,
    pub last_sensed_stimuli: TArray<FAIStimulus>,
    pub flags_24: u8,
    __padding_end: [u8; 7],
}
impl FActorPerceptionBlueprintInfo {}
#[repr(C, align(4))]
pub struct FAISenseAffiliationFilter {
    pub flags_0: u8,
    __padding_end: [u8; 3],
}
impl FAISenseAffiliationFilter {}
#[repr(C, align(8))]
pub struct FAIDamageEvent {
    pub amount: f32,
    pub location: crate::bindings::core_u_object::FVector,
    pub hit_location: crate::bindings::core_u_object::FVector,
    pub damaged_actor: UPtr<crate::bindings::engine::AActor>,
    pub instigator: UPtr<crate::bindings::engine::AActor>,
    pub tag: FName,
    __padding_end: [u8; 4],
}
impl FAIDamageEvent {}
#[repr(C, align(8))]
pub struct FAINoiseEvent {
    #[doc(hidden)]
    __padding_8: [u8; 8],
    pub noise_location: crate::bindings::core_u_object::FVector,
    pub loudness: f32,
    pub max_range: f32,
    pub instigator: UPtr<crate::bindings::engine::AActor>,
    pub tag: FName,
    __padding_end: [u8; 4],
}
impl FAINoiseEvent {}
#[repr(C, align(8))]
pub struct UAIAsyncTaskBlueprintProxy {
    __padding_end: [u8; 128],
}
impl UAIAsyncTaskBlueprintProxy {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAIAsyncTaskBlueprintProxy")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
pub struct IAIResourceInterface {}
#[repr(C, align(8))]
pub struct UAIResourceInterface {
    __padding_end: [u8; 48],
}
impl UAIResourceInterface {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAIResourceInterface")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UAISenseBlueprintListener {
    __padding_end: [u8; 368],
}
impl UAISenseBlueprintListener {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAISenseBlueprintListener")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UAISenseConfig {
    #[doc(hidden)]
    __padding_48: [u8; 48],
    pub debug_color: crate::bindings::core_u_object::FColor,
    pub max_age: f32,
    pub flags_56: u8,
    __padding_end: [u8; 23],
}
impl UAISenseConfig {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAISenseConfig")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UAISenseConfig_Blueprint {
    #[doc(hidden)]
    __padding_80: [u8; 80],
    pub implementation: TSubclassOf<UAISense_Blueprint>,
}
impl UAISenseConfig_Blueprint {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAISenseConfig_Blueprint")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UAISenseConfig_Hearing {
    #[doc(hidden)]
    __padding_80: [u8; 80],
    pub implementation: TSubclassOf<UAISense_Hearing>,
    pub hearing_range: f32,
    pub lo_s_hearing_range: f32,
    #[doc(hidden)]
    __padding_100: [u8; 4],
    pub detection_by_affiliation: FAISenseAffiliationFilter,
}
impl UAISenseConfig_Hearing {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAISenseConfig_Hearing")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UAISenseConfig_Prediction {
    __padding_end: [u8; 80],
}
impl UAISenseConfig_Prediction {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAISenseConfig_Prediction")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UAISenseConfig_Sight {
    #[doc(hidden)]
    __padding_80: [u8; 80],
    pub implementation: TSubclassOf<UAISense_Sight>,
    pub sight_radius: f32,
    pub lose_sight_radius: f32,
    pub peripheral_vision_angle_degrees: f32,
    pub detection_by_affiliation: FAISenseAffiliationFilter,
    pub auto_success_range_from_last_seen_location: f32,
    pub point_of_view_backward_offset: f32,
    pub near_clipping_radius: f32,
    __padding_end: [u8; 4],
}
impl UAISenseConfig_Sight {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAISenseConfig_Sight")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UAISenseConfig_Team {
    __padding_end: [u8; 80],
}
impl UAISenseConfig_Team {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAISenseConfig_Team")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UAISenseConfig_Touch {
    #[doc(hidden)]
    __padding_80: [u8; 80],
    pub detection_by_affiliation: FAISenseAffiliationFilter,
    __padding_end: [u8; 4],
}
impl UAISenseConfig_Touch {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAISenseConfig_Touch")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UAISenseEvent {
    __padding_end: [u8; 48],
}
impl UAISenseEvent {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAISenseEvent")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UAISenseEvent_Damage {
    #[doc(hidden)]
    __padding_48: [u8; 48],
    pub event: FAIDamageEvent,
}
impl UAISenseEvent_Damage {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAISenseEvent_Damage")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UAISenseEvent_Hearing {
    #[doc(hidden)]
    __padding_48: [u8; 48],
    pub event: FAINoiseEvent,
}
impl UAISenseEvent_Hearing {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAISenseEvent_Hearing")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
pub struct ICrowdAgentInterface {}
#[repr(C, align(8))]
pub struct UCrowdAgentInterface {
    __padding_end: [u8; 48],
}
impl UCrowdAgentInterface {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UCrowdAgentInterface")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
pub struct IEQSQueryResultSourceInterface {}
#[repr(C, align(8))]
pub struct UEQSQueryResultSourceInterface {
    __padding_end: [u8; 48],
}
impl UEQSQueryResultSourceInterface {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEQSQueryResultSourceInterface")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
pub struct IGenericTeamAgentInterface {}
#[repr(C, align(8))]
pub struct UGenericTeamAgentInterface {
    __padding_end: [u8; 48],
}
impl UGenericTeamAgentInterface {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UGenericTeamAgentInterface")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct AAIController {
    #[doc(hidden)]
    __padding_1376: [u8; 1376],
    pub flags_1376: u8,
    #[doc(hidden)]
    __padding_1392: [u8; 8],
    pub brain_component: UPtr<UBrainComponent>,
    #[doc(hidden)]
    __padding_1408: [u8; 8],
    pub blackboard: UPtr<UBlackboardComponent>,
    #[doc(hidden)]
    __padding_1424: [u8; 8],
    pub default_navigation_filter_class: TSubclassOf<
        crate::bindings::navigation_system::UNavigationQueryFilter,
    >,
    __padding_end: [u8; 32],
}
impl AAIController {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("AAIController")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UAIResource_Movement {
    __padding_end: [u8; 64],
}
impl UAIResource_Movement {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAIResource_Movement")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UAIResource_Logic {
    __padding_end: [u8; 64],
}
impl UAIResource_Logic {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAIResource_Logic")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UAISubsystem {
    __padding_end: [u8; 64],
}
impl UAISubsystem {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAISubsystem")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UAISystem {
    __padding_end: [u8; 472],
}
impl UAISystem {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAISystem")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UBehaviorTree {
    __padding_end: [u8; 136],
}
impl UBehaviorTree {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBehaviorTree")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UBrainComponent {
    __padding_end: [u8; 328],
}
impl UBrainComponent {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBrainComponent")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UBehaviorTreeComponent {
    #[doc(hidden)]
    __padding_848: [u8; 848],
    pub default_behavior_tree_asset: UPtr<UBehaviorTree>,
    __padding_end: [u8; 32],
}
impl UBehaviorTreeComponent {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBehaviorTreeComponent")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UBehaviorTreeManager {
    __padding_end: [u8; 88],
}
impl UBehaviorTreeManager {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBehaviorTreeManager")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UBehaviorTreeTypes {
    __padding_end: [u8; 48],
}
impl UBehaviorTreeTypes {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBehaviorTreeTypes")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
pub struct IBlackboardAssetProvider {}
#[repr(C, align(8))]
pub struct UBlackboardAssetProvider {
    __padding_end: [u8; 48],
}
impl UBlackboardAssetProvider {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBlackboardAssetProvider")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UBlackboardComponent {
    __padding_end: [u8; 504],
}
impl UBlackboardComponent {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBlackboardComponent")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UBlackboardData {
    __padding_end: [u8; 104],
}
impl UBlackboardData {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBlackboardData")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UBlackboardKeyType {
    __padding_end: [u8; 56],
}
impl UBlackboardKeyType {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBlackboardKeyType")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UBlackboardKeyType_Bool {
    __padding_end: [u8; 64],
}
impl UBlackboardKeyType_Bool {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBlackboardKeyType_Bool")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UBlackboardKeyType_Class {
    __padding_end: [u8; 72],
}
impl UBlackboardKeyType_Class {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBlackboardKeyType_Class")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UBlackboardKeyType_Enum {
    __padding_end: [u8; 88],
}
impl UBlackboardKeyType_Enum {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBlackboardKeyType_Enum")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UBlackboardKeyType_Float {
    __padding_end: [u8; 64],
}
impl UBlackboardKeyType_Float {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBlackboardKeyType_Float")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UBlackboardKeyType_Int {
    __padding_end: [u8; 64],
}
impl UBlackboardKeyType_Int {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBlackboardKeyType_Int")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UBlackboardKeyType_Name {
    __padding_end: [u8; 72],
}
impl UBlackboardKeyType_Name {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBlackboardKeyType_Name")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UBlackboardKeyType_NativeEnum {
    __padding_end: [u8; 80],
}
impl UBlackboardKeyType_NativeEnum {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBlackboardKeyType_NativeEnum")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UBlackboardKeyType_Object {
    __padding_end: [u8; 72],
}
impl UBlackboardKeyType_Object {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBlackboardKeyType_Object")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UBlackboardKeyType_Rotator {
    __padding_end: [u8; 88],
}
impl UBlackboardKeyType_Rotator {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBlackboardKeyType_Rotator")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UBlackboardKeyType_String {
    __padding_end: [u8; 88],
}
impl UBlackboardKeyType_String {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBlackboardKeyType_String")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UBlackboardKeyType_Struct {
    __padding_end: [u8; 88],
}
impl UBlackboardKeyType_Struct {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBlackboardKeyType_Struct")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UBlackboardKeyType_Vector {
    __padding_end: [u8; 88],
}
impl UBlackboardKeyType_Vector {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBlackboardKeyType_Vector")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UBTNode {
    __padding_end: [u8; 104],
}
impl UBTNode {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS.wait().name_to_ptr.get("UBTNode").unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UBTAuxiliaryNode {
    __padding_end: [u8; 112],
}
impl UBTAuxiliaryNode {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBTAuxiliaryNode")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UBTCompositeNode {
    __padding_end: [u8; 144],
}
impl UBTCompositeNode {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBTCompositeNode")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UBTDecorator {
    __padding_end: [u8; 120],
}
impl UBTDecorator {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBTDecorator")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UBTFunctionLibrary {
    __padding_end: [u8; 48],
}
impl UBTFunctionLibrary {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBTFunctionLibrary")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UBTService {
    __padding_end: [u8; 128],
}
impl UBTService {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBTService")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UBTTaskNode {
    __padding_end: [u8; 128],
}
impl UBTTaskNode {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBTTaskNode")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UBTComposite_Selector {
    __padding_end: [u8; 144],
}
impl UBTComposite_Selector {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBTComposite_Selector")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UBTComposite_Sequence {
    __padding_end: [u8; 144],
}
impl UBTComposite_Sequence {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBTComposite_Sequence")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UBTComposite_SimpleParallel {
    __padding_end: [u8; 152],
}
impl UBTComposite_SimpleParallel {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBTComposite_SimpleParallel")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UBTDecorator_BlackboardBase {
    __padding_end: [u8; 168],
}
impl UBTDecorator_BlackboardBase {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBTDecorator_BlackboardBase")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UBTDecorator_Blackboard {
    __padding_end: [u8; 216],
}
impl UBTDecorator_Blackboard {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBTDecorator_Blackboard")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UBTDecorator_BlueprintBase {
    #[doc(hidden)]
    __padding_168: [u8; 168],
    pub custom_description: FString,
    __padding_end: [u8; 8],
}
impl UBTDecorator_BlueprintBase {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBTDecorator_BlueprintBase")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UBTDecorator_CheckGameplayTagsOnActor {
    __padding_end: [u8; 224],
}
impl UBTDecorator_CheckGameplayTagsOnActor {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBTDecorator_CheckGameplayTagsOnActor")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UBTDecorator_CompareBBEntries {
    __padding_end: [u8; 224],
}
impl UBTDecorator_CompareBBEntries {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBTDecorator_CompareBBEntries")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UBTDecorator_ConditionalLoop {
    __padding_end: [u8; 216],
}
impl UBTDecorator_ConditionalLoop {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBTDecorator_ConditionalLoop")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UBTDecorator_ConeCheck {
    __padding_end: [u8; 296],
}
impl UBTDecorator_ConeCheck {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBTDecorator_ConeCheck")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UBTDecorator_Cooldown {
    __padding_end: [u8; 152],
}
impl UBTDecorator_Cooldown {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBTDecorator_Cooldown")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UBTDecorator_DoesPathExist {
    __padding_end: [u8; 320],
}
impl UBTDecorator_DoesPathExist {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBTDecorator_DoesPathExist")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UBTDecorator_ForceSuccess {
    __padding_end: [u8; 120],
}
impl UBTDecorator_ForceSuccess {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBTDecorator_ForceSuccess")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UBTDecorator_IsAtLocation {
    __padding_end: [u8; 312],
}
impl UBTDecorator_IsAtLocation {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBTDecorator_IsAtLocation")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UBTDecorator_IsBBEntryOfClass {
    __padding_end: [u8; 208],
}
impl UBTDecorator_IsBBEntryOfClass {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBTDecorator_IsBBEntryOfClass")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UBTDecorator_KeepInCone {
    __padding_end: [u8; 256],
}
impl UBTDecorator_KeepInCone {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBTDecorator_KeepInCone")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UBTDecorator_Loop {
    __padding_end: [u8; 192],
}
impl UBTDecorator_Loop {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBTDecorator_Loop")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UBTDecorator_LoopUntil {
    __padding_end: [u8; 176],
}
impl UBTDecorator_LoopUntil {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBTDecorator_LoopUntil")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UBTDecorator_ReachedMoveGoal {
    __padding_end: [u8; 120],
}
impl UBTDecorator_ReachedMoveGoal {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBTDecorator_ReachedMoveGoal")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UBTDecorator_SetTagCooldown {
    __padding_end: [u8; 200],
}
impl UBTDecorator_SetTagCooldown {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBTDecorator_SetTagCooldown")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UBTDecorator_TagCooldown {
    __padding_end: [u8; 232],
}
impl UBTDecorator_TagCooldown {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBTDecorator_TagCooldown")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UBTDecorator_TimeLimit {
    __padding_end: [u8; 152],
}
impl UBTDecorator_TimeLimit {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBTDecorator_TimeLimit")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UBTService_BlackboardBase {
    __padding_end: [u8; 176],
}
impl UBTService_BlackboardBase {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBTService_BlackboardBase")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UBTService_BlueprintBase {
    #[doc(hidden)]
    __padding_160: [u8; 160],
    pub custom_description: FString,
    __padding_end: [u8; 8],
}
impl UBTService_BlueprintBase {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBTService_BlueprintBase")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UBTService_DefaultFocus {
    __padding_end: [u8; 184],
}
impl UBTService_DefaultFocus {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBTService_DefaultFocus")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UBTService_RunEQS {
    __padding_end: [u8; 288],
}
impl UBTService_RunEQS {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBTService_RunEQS")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UBTTask_BlackboardBase {
    __padding_end: [u8; 176],
}
impl UBTTask_BlackboardBase {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBTTask_BlackboardBase")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UBTTask_BlueprintBase {
    #[doc(hidden)]
    __padding_176: [u8; 176],
    pub custom_description: FString,
    __padding_end: [u8; 8],
}
impl UBTTask_BlueprintBase {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBTTask_BlueprintBase")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UBTTask_FinishWithResult {
    __padding_end: [u8; 184],
}
impl UBTTask_FinishWithResult {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBTTask_FinishWithResult")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UBTTask_GameplayTaskBase {
    __padding_end: [u8; 160],
}
impl UBTTask_GameplayTaskBase {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBTTask_GameplayTaskBase")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UBTTask_MakeNoise {
    __padding_end: [u8; 160],
}
impl UBTTask_MakeNoise {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBTTask_MakeNoise")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UBTTask_MoveTo {
    __padding_end: [u8; 544],
}
impl UBTTask_MoveTo {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBTTask_MoveTo")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UBTTask_MoveDirectlyToward {
    __padding_end: [u8; 544],
}
impl UBTTask_MoveDirectlyToward {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBTTask_MoveDirectlyToward")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UBTTask_PlayAnimation {
    __padding_end: [u8; 280],
}
impl UBTTask_PlayAnimation {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBTTask_PlayAnimation")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UBTTask_PlaySound {
    __padding_end: [u8; 168],
}
impl UBTTask_PlaySound {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBTTask_PlaySound")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UBTTask_RotateToFaceBBEntry {
    __padding_end: [u8; 208],
}
impl UBTTask_RotateToFaceBBEntry {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBTTask_RotateToFaceBBEntry")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UBTTask_RunBehavior {
    __padding_end: [u8; 136],
}
impl UBTTask_RunBehavior {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBTTask_RunBehavior")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UBTTask_RunBehaviorDynamic {
    __padding_end: [u8; 160],
}
impl UBTTask_RunBehaviorDynamic {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBTTask_RunBehaviorDynamic")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UBTTask_RunEQSQuery {
    __padding_end: [u8; 296],
}
impl UBTTask_RunEQSQuery {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBTTask_RunEQSQuery")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UBTTask_SetKeyValueBool {
    __padding_end: [u8; 208],
}
impl UBTTask_SetKeyValueBool {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBTTask_SetKeyValueBool")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UBTTask_SetKeyValueClass {
    __padding_end: [u8; 224],
}
impl UBTTask_SetKeyValueClass {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBTTask_SetKeyValueClass")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UBTTask_SetKeyValueEnum {
    __padding_end: [u8; 240],
}
impl UBTTask_SetKeyValueEnum {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBTTask_SetKeyValueEnum")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UBTTask_SetKeyValueInt32 {
    __padding_end: [u8; 208],
}
impl UBTTask_SetKeyValueInt32 {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBTTask_SetKeyValueInt32")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UBTTask_SetKeyValueFloat {
    __padding_end: [u8; 208],
}
impl UBTTask_SetKeyValueFloat {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBTTask_SetKeyValueFloat")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UBTTask_SetKeyValueName {
    __padding_end: [u8; 216],
}
impl UBTTask_SetKeyValueName {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBTTask_SetKeyValueName")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UBTTask_SetKeyValueString {
    __padding_end: [u8; 216],
}
impl UBTTask_SetKeyValueString {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBTTask_SetKeyValueString")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UBTTask_SetKeyValueObject {
    __padding_end: [u8; 224],
}
impl UBTTask_SetKeyValueObject {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBTTask_SetKeyValueObject")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UBTTask_SetKeyValueRotator {
    __padding_end: [u8; 224],
}
impl UBTTask_SetKeyValueRotator {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBTTask_SetKeyValueRotator")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UBTTask_SetKeyValueStruct {
    __padding_end: [u8; 232],
}
impl UBTTask_SetKeyValueStruct {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBTTask_SetKeyValueStruct")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UBTTask_SetKeyValueVector {
    __padding_end: [u8; 224],
}
impl UBTTask_SetKeyValueVector {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBTTask_SetKeyValueVector")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UBTTask_SetTagCooldown {
    __padding_end: [u8; 208],
}
impl UBTTask_SetTagCooldown {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBTTask_SetTagCooldown")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UBTTask_Wait {
    __padding_end: [u8; 192],
}
impl UBTTask_Wait {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBTTask_Wait")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UBTTask_WaitBlackboardTime {
    __padding_end: [u8; 240],
}
impl UBTTask_WaitBlackboardTime {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBTTask_WaitBlackboardTime")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UValueOrBBKeyBlueprintUtility {
    __padding_end: [u8; 48],
}
impl UValueOrBBKeyBlueprintUtility {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UValueOrBBKeyBlueprintUtility")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UAIBlueprintHelperLibrary {
    __padding_end: [u8; 48],
}
impl UAIBlueprintHelperLibrary {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAIBlueprintHelperLibrary")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UAIDataProvider {
    __padding_end: [u8; 48],
}
impl UAIDataProvider {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAIDataProvider")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UAIDataProvider_QueryParams {
    __padding_end: [u8; 72],
}
impl UAIDataProvider_QueryParams {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAIDataProvider_QueryParams")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UAIDataProvider_Random {
    __padding_end: [u8; 88],
}
impl UAIDataProvider_Random {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAIDataProvider_Random")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct ADetourCrowdAIController {
    __padding_end: [u8; 1464],
}
impl ADetourCrowdAIController {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("ADetourCrowdAIController")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UEnvQueryContext {
    __padding_end: [u8; 48],
}
impl UEnvQueryContext {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEnvQueryContext")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UEnvQueryContext_BlueprintBase {
    __padding_end: [u8; 56],
}
impl UEnvQueryContext_BlueprintBase {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEnvQueryContext_BlueprintBase")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UEnvQueryContext_Item {
    __padding_end: [u8; 48],
}
impl UEnvQueryContext_Item {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEnvQueryContext_Item")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UEnvQueryContext_NavigationData {
    __padding_end: [u8; 112],
}
impl UEnvQueryContext_NavigationData {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEnvQueryContext_NavigationData")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UEnvQueryContext_Querier {
    __padding_end: [u8; 48],
}
impl UEnvQueryContext_Querier {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEnvQueryContext_Querier")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UEnvQuery {
    __padding_end: [u8; 96],
}
impl UEnvQuery {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEnvQuery")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UEnvQueryDebugHelpers {
    __padding_end: [u8; 48],
}
impl UEnvQueryDebugHelpers {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEnvQueryDebugHelpers")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UEnvQueryNode {
    __padding_end: [u8; 56],
}
impl UEnvQueryNode {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEnvQueryNode")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UEnvQueryGenerator {
    __padding_end: [u8; 88],
}
impl UEnvQueryGenerator {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEnvQueryGenerator")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UEnvQueryInstanceBlueprintWrapper {
    #[doc(hidden)]
    __padding_56: [u8; 56],
    pub query_id: i32,
    #[doc(hidden)]
    __padding_96: [u8; 32],
    pub item_type: TSubclassOf<UEnvQueryItemType>,
    pub option_index: i32,
    __padding_end: [u8; 36],
}
impl UEnvQueryInstanceBlueprintWrapper {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEnvQueryInstanceBlueprintWrapper")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UEnvQueryManager {
    __padding_end: [u8; 528],
}
impl UEnvQueryManager {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEnvQueryManager")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UEnvQueryOption {
    __padding_end: [u8; 72],
}
impl UEnvQueryOption {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEnvQueryOption")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UEnvQueryTest {
    __padding_end: [u8; 608],
}
impl UEnvQueryTest {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEnvQueryTest")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UEnvQueryTypes {
    __padding_end: [u8; 48],
}
impl UEnvQueryTypes {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEnvQueryTypes")
            .unwrap()
    }
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
pub struct UEQSRenderingComponent {
    __padding_end: [u8; 1792],
}
impl UEQSRenderingComponent {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEQSRenderingComponent")
            .unwrap()
    }
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
pub struct AEQSTestingPawn {
    __padding_end: [u8; 2288],
}
impl AEQSTestingPawn {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("AEQSTestingPawn")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UEnvQueryGenerator_ActorsOfClass {
    __padding_end: [u8; 232],
}
impl UEnvQueryGenerator_ActorsOfClass {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEnvQueryGenerator_ActorsOfClass")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UEnvQueryGenerator_BlueprintBase {
    __padding_end: [u8; 136],
}
impl UEnvQueryGenerator_BlueprintBase {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEnvQueryGenerator_BlueprintBase")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UEnvQueryGenerator_Composite {
    __padding_end: [u8; 120],
}
impl UEnvQueryGenerator_Composite {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEnvQueryGenerator_Composite")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UEnvQueryGenerator_ProjectedPoints {
    __padding_end: [u8; 160],
}
impl UEnvQueryGenerator_ProjectedPoints {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEnvQueryGenerator_ProjectedPoints")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UEnvQueryGenerator_Cone {
    __padding_end: [u8; 432],
}
impl UEnvQueryGenerator_Cone {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEnvQueryGenerator_Cone")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UEnvQueryGenerator_CurrentLocation {
    __padding_end: [u8; 168],
}
impl UEnvQueryGenerator_CurrentLocation {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEnvQueryGenerator_CurrentLocation")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UEnvQueryGenerator_Donut {
    __padding_end: [u8; 536],
}
impl UEnvQueryGenerator_Donut {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEnvQueryGenerator_Donut")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UEnvQueryGenerator_OnCircle {
    __padding_end: [u8; 672],
}
impl UEnvQueryGenerator_OnCircle {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEnvQueryGenerator_OnCircle")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UEnvQueryGenerator_SimpleGrid {
    __padding_end: [u8; 296],
}
impl UEnvQueryGenerator_SimpleGrid {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEnvQueryGenerator_SimpleGrid")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UEnvQueryGenerator_PathingGrid {
    __padding_end: [u8; 432],
}
impl UEnvQueryGenerator_PathingGrid {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEnvQueryGenerator_PathingGrid")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UEnvQueryGenerator_PerceivedActors {
    __padding_end: [u8; 184],
}
impl UEnvQueryGenerator_PerceivedActors {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEnvQueryGenerator_PerceivedActors")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UEnvQueryItemType {
    __padding_end: [u8; 56],
}
impl UEnvQueryItemType {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEnvQueryItemType")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UEnvQueryItemType_VectorBase {
    __padding_end: [u8; 56],
}
impl UEnvQueryItemType_VectorBase {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEnvQueryItemType_VectorBase")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UEnvQueryItemType_ActorBase {
    __padding_end: [u8; 56],
}
impl UEnvQueryItemType_ActorBase {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEnvQueryItemType_ActorBase")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UEnvQueryItemType_Actor {
    __padding_end: [u8; 56],
}
impl UEnvQueryItemType_Actor {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEnvQueryItemType_Actor")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UEnvQueryItemType_Direction {
    __padding_end: [u8; 56],
}
impl UEnvQueryItemType_Direction {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEnvQueryItemType_Direction")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UEnvQueryItemType_Point {
    __padding_end: [u8; 56],
}
impl UEnvQueryItemType_Point {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEnvQueryItemType_Point")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UEnvQueryTest_Distance {
    __padding_end: [u8; 624],
}
impl UEnvQueryTest_Distance {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEnvQueryTest_Distance")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UEnvQueryTest_Dot {
    __padding_end: [u8; 680],
}
impl UEnvQueryTest_Dot {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEnvQueryTest_Dot")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UEnvQueryTest_GameplayTags {
    __padding_end: [u8; 720],
}
impl UEnvQueryTest_GameplayTags {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEnvQueryTest_GameplayTags")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UEnvQueryTest_Overlap {
    __padding_end: [u8; 656],
}
impl UEnvQueryTest_Overlap {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEnvQueryTest_Overlap")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UEnvQueryTest_Pathfinding {
    __padding_end: [u8; 760],
}
impl UEnvQueryTest_Pathfinding {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEnvQueryTest_Pathfinding")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UEnvQueryTest_PathfindingBatch {
    __padding_end: [u8; 824],
}
impl UEnvQueryTest_PathfindingBatch {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEnvQueryTest_PathfindingBatch")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UEnvQueryTest_Project {
    __padding_end: [u8; 672],
}
impl UEnvQueryTest_Project {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEnvQueryTest_Project")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UEnvQueryTest_Random {
    __padding_end: [u8; 608],
}
impl UEnvQueryTest_Random {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEnvQueryTest_Random")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UEnvQueryTest_Trace {
    __padding_end: [u8; 872],
}
impl UEnvQueryTest_Trace {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEnvQueryTest_Trace")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UEnvQueryTest_Volume {
    __padding_end: [u8; 632],
}
impl UEnvQueryTest_Volume {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEnvQueryTest_Volume")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct AGridPathAIController {
    __padding_end: [u8; 1464],
}
impl AGridPathAIController {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("AGridPathAIController")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UAIHotSpotManager {
    __padding_end: [u8; 48],
}
impl UAIHotSpotManager {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAIHotSpotManager")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UPathFollowingComponent {
    __padding_end: [u8; 888],
}
impl UPathFollowingComponent {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPathFollowingComponent")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UCrowdFollowingComponent {
    __padding_end: [u8; 984],
}
impl UCrowdFollowingComponent {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UCrowdFollowingComponent")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UCrowdManager {
    __padding_end: [u8; 256],
}
impl UCrowdManager {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UCrowdManager")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UGeneratedNavLinksProxy {
    __padding_end: [u8; 96],
}
impl UGeneratedNavLinksProxy {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UGeneratedNavLinksProxy")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UGridPathFollowingComponent {
    __padding_end: [u8; 936],
}
impl UGridPathFollowingComponent {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UGridPathFollowingComponent")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UNavFilter_AIControllerDefault {
    __padding_end: [u8; 80],
}
impl UNavFilter_AIControllerDefault {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNavFilter_AIControllerDefault")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct ANavLinkProxy {
    #[doc(hidden)]
    __padding_1152: [u8; 1152],
    pub point_links: TArray<crate::bindings::engine::FNavigationLink>,
    __padding_end: [u8; 88],
}
impl ANavLinkProxy {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("ANavLinkProxy")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UNavLocalGridManager {
    __padding_end: [u8; 96],
}
impl UNavLocalGridManager {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNavLocalGridManager")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UPathFollowingManager {
    __padding_end: [u8; 48],
}
impl UPathFollowingManager {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPathFollowingManager")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UAIPerceptionComponent {
    __padding_end: [u8; 512],
}
impl UAIPerceptionComponent {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAIPerceptionComponent")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
pub struct IAIPerceptionListenerInterface {}
#[repr(C, align(8))]
pub struct UAIPerceptionListenerInterface {
    __padding_end: [u8; 48],
}
impl UAIPerceptionListenerInterface {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAIPerceptionListenerInterface")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UAIPerceptionStimuliSourceComponent {
    #[doc(hidden)]
    __padding_240: [u8; 240],
    pub flags_240: u8,
    pub register_as_source_for_senses: TArray<TSubclassOf<UAISense>>,
}
impl UAIPerceptionStimuliSourceComponent {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAIPerceptionStimuliSourceComponent")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UAIPerceptionSystem {
    __padding_end: [u8; 336],
}
impl UAIPerceptionSystem {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAIPerceptionSystem")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UAISense {
    #[doc(hidden)]
    __padding_48: [u8; 48],
    pub notify_type: EAISenseNotifyType,
    #[doc(hidden)]
    __padding_52: [u8; 3],
    pub flags_52: u8,
    __padding_end: [u8; 107],
}
impl UAISense {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS.wait().name_to_ptr.get("UAISense").unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UAISenseConfig_Damage {
    #[doc(hidden)]
    __padding_80: [u8; 80],
    pub implementation: TSubclassOf<UAISense_Damage>,
}
impl UAISenseConfig_Damage {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAISenseConfig_Damage")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UAISense_Blueprint {
    #[doc(hidden)]
    __padding_160: [u8; 160],
    pub listener_data_type: TSubclassOf<
        crate::bindings::core_u_object::UUserDefinedStruct,
    >,
    pub listener_container: TArray<UPtr<UAIPerceptionComponent>>,
    __padding_end: [u8; 16],
}
impl UAISense_Blueprint {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAISense_Blueprint")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UAISense_Damage {
    __padding_end: [u8; 176],
}
impl UAISense_Damage {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAISense_Damage")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UAISense_Hearing {
    __padding_end: [u8; 264],
}
impl UAISense_Hearing {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAISense_Hearing")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UAISense_Prediction {
    __padding_end: [u8; 176],
}
impl UAISense_Prediction {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAISense_Prediction")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UAISense_Sight {
    __padding_end: [u8; 488],
}
impl UAISense_Sight {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAISense_Sight")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UAISense_Team {
    __padding_end: [u8; 176],
}
impl UAISense_Team {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAISense_Team")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UAISense_Touch {
    __padding_end: [u8; 256],
}
impl UAISense_Touch {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAISense_Touch")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
pub struct IAISightTargetInterface {}
#[repr(C, align(8))]
pub struct UAISightTargetInterface {
    __padding_end: [u8; 48],
}
impl UAISightTargetInterface {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAISightTargetInterface")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UPawnSensingComponent {
    #[doc(hidden)]
    __padding_240: [u8; 240],
    pub hearing_threshold: f32,
    pub los_hearing_threshold: f32,
    pub sight_radius: f32,
    pub sensing_interval: f32,
    pub hearing_max_sound_age: f32,
    pub flags_260: u8,
    #[doc(hidden)]
    __padding_320: [u8; 56],
    pub peripheral_vision_angle: f32,
    __padding_end: [u8; 4],
}
impl UPawnSensingComponent {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPawnSensingComponent")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UAITask {
    #[doc(hidden)]
    __padding_128: [u8; 128],
    pub owner_controller: UPtr<AAIController>,
}
impl UAITask {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS.wait().name_to_ptr.get("UAITask").unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UAITask_LockLogic {
    __padding_end: [u8; 136],
}
impl UAITask_LockLogic {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAITask_LockLogic")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UAITask_MoveTo {
    __padding_end: [u8; 360],
}
impl UAITask_MoveTo {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAITask_MoveTo")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UAITask_RunEQS {
    __padding_end: [u8; 280],
}
impl UAITask_RunEQS {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAITask_RunEQS")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UVisualLoggerExtension {
    __padding_end: [u8; 48],
}
impl UVisualLoggerExtension {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UVisualLoggerExtension")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct FAIAsyncTaskBlueprintProxy_OnSuccess {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FAIAsyncTaskBlueprintProxy_OnFail {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FAIController_ReceiveMoveCompleted {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FEnvQueryInstanceBlueprintWrapper_OnQueryFinishedEvent {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FGeneratedNavLinksProxy_OnSmartLinkReached {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FNavLinkProxy_OnSmartLinkReached {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FAIPerceptionComponent_OnPerceptionUpdated {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FAIPerceptionComponent_OnTargetPerceptionForgotten {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FAIPerceptionComponent_OnTargetPerceptionUpdated {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FAIPerceptionComponent_OnTargetPerceptionInfoUpdated {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FPawnSensingComponent_OnSeePawn {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FPawnSensingComponent_OnHearNoise {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FAITask_MoveTo_OnRequestFailed {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FAITask_MoveTo_OnMoveFinished {
    _opague: [u8; 24],
}
#[repr(transparent)]
pub struct EBTDecoratorLogic(pub u8);
impl EBTDecoratorLogic {
    pub const INVALID: EBTDecoratorLogic = EBTDecoratorLogic(0);
    pub const TEST: EBTDecoratorLogic = EBTDecoratorLogic(1);
    pub const AND: EBTDecoratorLogic = EBTDecoratorLogic(2);
    pub const OR: EBTDecoratorLogic = EBTDecoratorLogic(3);
    pub const NOT: EBTDecoratorLogic = EBTDecoratorLogic(4);
}
#[repr(transparent)]
pub struct EAIParamType(pub u8);
impl EAIParamType {
    pub const FLOAT: EAIParamType = EAIParamType(0);
    pub const INT: EAIParamType = EAIParamType(1);
    pub const BOOL: EAIParamType = EAIParamType(2);
    pub const MAX: EAIParamType = EAIParamType(3);
}
#[repr(transparent)]
pub struct EEnvDirection(pub u8);
impl EEnvDirection {
    pub const TWO_POINTS: EEnvDirection = EEnvDirection(0);
    pub const ROTATION: EEnvDirection = EEnvDirection(1);
}
#[repr(transparent)]
pub struct EEnvTraceShape(pub u8);
impl EEnvTraceShape {
    pub const LINE: EEnvTraceShape = EEnvTraceShape(0);
    pub const BOX: EEnvTraceShape = EEnvTraceShape(1);
    pub const SPHERE: EEnvTraceShape = EEnvTraceShape(2);
    pub const CAPSULE: EEnvTraceShape = EEnvTraceShape(3);
}
#[repr(transparent)]
pub struct EEnvQueryTrace(pub u8);
impl EEnvQueryTrace {
    pub const NONE: EEnvQueryTrace = EEnvQueryTrace(0);
    pub const NAVIGATION: EEnvQueryTrace = EEnvQueryTrace(1);
    pub const GEOMETRY_BY_CHANNEL: EEnvQueryTrace = EEnvQueryTrace(2);
    pub const GEOMETRY_BY_PROFILE: EEnvQueryTrace = EEnvQueryTrace(3);
    pub const NAVIGATION_OVER_LEDGES: EEnvQueryTrace = EEnvQueryTrace(4);
}
#[repr(transparent)]
pub struct EEnvOverlapShape(pub u8);
impl EEnvOverlapShape {
    pub const BOX: EEnvOverlapShape = EEnvOverlapShape(0);
    pub const SPHERE: EEnvOverlapShape = EEnvOverlapShape(1);
    pub const CAPSULE: EEnvOverlapShape = EEnvOverlapShape(2);
}
#[repr(transparent)]
pub struct EEnvQueryRunMode(pub u8);
impl EEnvQueryRunMode {
    pub const SINGLE_RESULT: EEnvQueryRunMode = EEnvQueryRunMode(0);
    pub const RANDOM_BEST5_PCT: EEnvQueryRunMode = EEnvQueryRunMode(1);
    pub const RANDOM_BEST25_PCT: EEnvQueryRunMode = EEnvQueryRunMode(2);
    pub const ALL_MATCHING: EEnvQueryRunMode = EEnvQueryRunMode(3);
}
#[repr(transparent)]
pub struct EGenericAICheck(pub u8);
impl EGenericAICheck {
    pub const LESS: EGenericAICheck = EGenericAICheck(0);
    pub const LESS_OR_EQUAL: EGenericAICheck = EGenericAICheck(1);
    pub const EQUAL: EGenericAICheck = EGenericAICheck(2);
    pub const NOT_EQUAL: EGenericAICheck = EGenericAICheck(3);
    pub const GREATER_OR_EQUAL: EGenericAICheck = EGenericAICheck(4);
    pub const GREATER: EGenericAICheck = EGenericAICheck(5);
    pub const IS_TRUE: EGenericAICheck = EGenericAICheck(6);
    pub const MAX: EGenericAICheck = EGenericAICheck(7);
}
#[repr(transparent)]
pub struct EPathFollowingResult(pub u8);
impl EPathFollowingResult {
    pub const SUCCESS: EPathFollowingResult = EPathFollowingResult(0);
    pub const BLOCKED: EPathFollowingResult = EPathFollowingResult(1);
    pub const OFF_PATH: EPathFollowingResult = EPathFollowingResult(2);
    pub const ABORTED: EPathFollowingResult = EPathFollowingResult(3);
    pub const SKIPPED_DEPRECATED: EPathFollowingResult = EPathFollowingResult(4);
    pub const INVALID: EPathFollowingResult = EPathFollowingResult(5);
}
#[repr(transparent)]
pub struct EPathFollowingStatus(pub u8);
impl EPathFollowingStatus {
    pub const IDLE: EPathFollowingStatus = EPathFollowingStatus(0);
    pub const WAITING: EPathFollowingStatus = EPathFollowingStatus(1);
    pub const PAUSED: EPathFollowingStatus = EPathFollowingStatus(2);
    pub const MOVING: EPathFollowingStatus = EPathFollowingStatus(3);
}
#[repr(transparent)]
pub struct EPathFollowingRequestResult(pub u8);
impl EPathFollowingRequestResult {
    pub const FAILED: EPathFollowingRequestResult = EPathFollowingRequestResult(0);
    pub const ALREADY_AT_GOAL: EPathFollowingRequestResult = EPathFollowingRequestResult(
        1,
    );
    pub const REQUEST_SUCCESSFUL: EPathFollowingRequestResult = EPathFollowingRequestResult(
        2,
    );
}
#[repr(transparent)]
pub struct EBTNodeResult(pub u8);
impl EBTNodeResult {
    pub const SUCCEEDED: EBTNodeResult = EBTNodeResult(0);
    pub const FAILED: EBTNodeResult = EBTNodeResult(1);
    pub const ABORTED: EBTNodeResult = EBTNodeResult(2);
    pub const IN_PROGRESS: EBTNodeResult = EBTNodeResult(3);
}
#[repr(transparent)]
pub struct EPathFollowingAction(pub u8);
impl EPathFollowingAction {
    pub const ERROR: EPathFollowingAction = EPathFollowingAction(0);
    pub const NO_MOVE: EPathFollowingAction = EPathFollowingAction(1);
    pub const DIRECT_MOVE: EPathFollowingAction = EPathFollowingAction(2);
    pub const PARTIAL_PATH: EPathFollowingAction = EPathFollowingAction(3);
    pub const PATH_TO_GOAL: EPathFollowingAction = EPathFollowingAction(4);
}
#[repr(transparent)]
pub struct EAIOptionFlag(pub u8);
impl EAIOptionFlag {
    pub const DEFAULT: EAIOptionFlag = EAIOptionFlag(0);
    pub const ENABLE: EAIOptionFlag = EAIOptionFlag(1);
    pub const DISABLE: EAIOptionFlag = EAIOptionFlag(2);
    pub const MAX: EAIOptionFlag = EAIOptionFlag(3);
}
#[repr(transparent)]
pub struct EBTFlowAbortMode(pub u8);
impl EBTFlowAbortMode {
    pub const NONE: EBTFlowAbortMode = EBTFlowAbortMode(0);
    pub const LOWER_PRIORITY: EBTFlowAbortMode = EBTFlowAbortMode(1);
    pub const SELF: EBTFlowAbortMode = EBTFlowAbortMode(2);
    pub const BOTH: EBTFlowAbortMode = EBTFlowAbortMode(3);
}
#[repr(transparent)]
pub struct EBTParallelMode(pub u8);
impl EBTParallelMode {
    pub const ABORT_BACKGROUND: EBTParallelMode = EBTParallelMode(0);
    pub const WAIT_FOR_BACKGROUND: EBTParallelMode = EBTParallelMode(1);
}
#[repr(transparent)]
pub struct EBTBlackboardRestart(pub u8);
impl EBTBlackboardRestart {
    pub const VALUE_CHANGE: EBTBlackboardRestart = EBTBlackboardRestart(0);
    pub const RESULT_CHANGE: EBTBlackboardRestart = EBTBlackboardRestart(1);
}
#[repr(transparent)]
pub struct EBasicKeyOperation(pub u8);
impl EBasicKeyOperation {
    pub const SET: EBasicKeyOperation = EBasicKeyOperation(0);
    pub const NOT_SET: EBasicKeyOperation = EBasicKeyOperation(1);
}
#[repr(transparent)]
pub struct EArithmeticKeyOperation(pub u8);
impl EArithmeticKeyOperation {
    pub const EQUAL: EArithmeticKeyOperation = EArithmeticKeyOperation(0);
    pub const NOT_EQUAL: EArithmeticKeyOperation = EArithmeticKeyOperation(1);
    pub const LESS: EArithmeticKeyOperation = EArithmeticKeyOperation(2);
    pub const LESS_OR_EQUAL: EArithmeticKeyOperation = EArithmeticKeyOperation(3);
    pub const GREATER: EArithmeticKeyOperation = EArithmeticKeyOperation(4);
    pub const GREATER_OR_EQUAL: EArithmeticKeyOperation = EArithmeticKeyOperation(5);
}
#[repr(transparent)]
pub struct ETextKeyOperation(pub u8);
impl ETextKeyOperation {
    pub const EQUAL: ETextKeyOperation = ETextKeyOperation(0);
    pub const NOT_EQUAL: ETextKeyOperation = ETextKeyOperation(1);
    pub const CONTAIN: ETextKeyOperation = ETextKeyOperation(2);
    pub const NOT_CONTAIN: ETextKeyOperation = ETextKeyOperation(3);
}
#[repr(transparent)]
pub struct EBlackBoardEntryComparison(pub u8);
impl EBlackBoardEntryComparison {
    pub const EQUAL: EBlackBoardEntryComparison = EBlackBoardEntryComparison(0);
    pub const NOT_EQUAL: EBlackBoardEntryComparison = EBlackBoardEntryComparison(1);
}
#[repr(transparent)]
pub struct FAIDistanceType(pub u8);
impl FAIDistanceType {
    pub const DISTANCE3_D: FAIDistanceType = FAIDistanceType(0);
    pub const DISTANCE2_D: FAIDistanceType = FAIDistanceType(1);
    pub const DISTANCE_Z: FAIDistanceType = FAIDistanceType(2);
    pub const MAX: FAIDistanceType = FAIDistanceType(3);
}
#[repr(transparent)]
pub struct EEnvQueryResultNormalizationOption(pub u8);
impl EEnvQueryResultNormalizationOption {
    pub const DEFAULT: EEnvQueryResultNormalizationOption = EEnvQueryResultNormalizationOption(
        0,
    );
    pub const NORMALIZED: EEnvQueryResultNormalizationOption = EEnvQueryResultNormalizationOption(
        1,
    );
    pub const UNALTERED: EEnvQueryResultNormalizationOption = EEnvQueryResultNormalizationOption(
        2,
    );
}
#[repr(transparent)]
pub struct EEnvQueryStatus(pub u8);
impl EEnvQueryStatus {
    pub const PROCESSING: EEnvQueryStatus = EEnvQueryStatus(0);
    pub const SUCCESS: EEnvQueryStatus = EEnvQueryStatus(1);
    pub const FAILED: EEnvQueryStatus = EEnvQueryStatus(2);
    pub const ABORTED: EEnvQueryStatus = EEnvQueryStatus(3);
    pub const OWNER_LOST: EEnvQueryStatus = EEnvQueryStatus(4);
    pub const MISSING_PARAM: EEnvQueryStatus = EEnvQueryStatus(5);
}
#[repr(transparent)]
pub struct EEnvTestPurpose(pub u8);
impl EEnvTestPurpose {
    pub const FILTER: EEnvTestPurpose = EEnvTestPurpose(0);
    pub const SCORE: EEnvTestPurpose = EEnvTestPurpose(1);
    pub const FILTER_AND_SCORE: EEnvTestPurpose = EEnvTestPurpose(2);
}
#[repr(transparent)]
pub struct EEnvTestFilterOperator(pub u8);
impl EEnvTestFilterOperator {
    pub const ALL_PASS: EEnvTestFilterOperator = EEnvTestFilterOperator(0);
    pub const ANY_PASS: EEnvTestFilterOperator = EEnvTestFilterOperator(1);
}
#[repr(transparent)]
pub struct EEnvTestScoreOperator(pub u8);
impl EEnvTestScoreOperator {
    pub const AVERAGE_SCORE: EEnvTestScoreOperator = EEnvTestScoreOperator(0);
    pub const MIN_SCORE: EEnvTestScoreOperator = EEnvTestScoreOperator(1);
    pub const MAX_SCORE: EEnvTestScoreOperator = EEnvTestScoreOperator(2);
    pub const MULTIPLY: EEnvTestScoreOperator = EEnvTestScoreOperator(3);
}
#[repr(transparent)]
pub struct EEnvTestFilterType(pub u8);
impl EEnvTestFilterType {
    pub const MINIMUM: EEnvTestFilterType = EEnvTestFilterType(0);
    pub const MAXIMUM: EEnvTestFilterType = EEnvTestFilterType(1);
    pub const RANGE: EEnvTestFilterType = EEnvTestFilterType(2);
    pub const MATCH: EEnvTestFilterType = EEnvTestFilterType(3);
}
#[repr(transparent)]
pub struct EEnvTestScoreEquation(pub u8);
impl EEnvTestScoreEquation {
    pub const LINEAR: EEnvTestScoreEquation = EEnvTestScoreEquation(0);
    pub const SQUARE: EEnvTestScoreEquation = EEnvTestScoreEquation(1);
    pub const INVERSE_LINEAR: EEnvTestScoreEquation = EEnvTestScoreEquation(2);
    pub const SQUARE_ROOT: EEnvTestScoreEquation = EEnvTestScoreEquation(3);
    pub const CONSTANT: EEnvTestScoreEquation = EEnvTestScoreEquation(4);
}
#[repr(transparent)]
pub struct EEnvQueryTestClamping(pub u8);
impl EEnvQueryTestClamping {
    pub const NONE: EEnvQueryTestClamping = EEnvQueryTestClamping(0);
    pub const SPECIFIED_VALUE: EEnvQueryTestClamping = EEnvQueryTestClamping(1);
    pub const FILTER_THRESHOLD: EEnvQueryTestClamping = EEnvQueryTestClamping(2);
}
#[repr(transparent)]
pub struct EEQSNormalizationType(pub u8);
impl EEQSNormalizationType {
    pub const ABSOLUTE: EEQSNormalizationType = EEQSNormalizationType(0);
    pub const RELATIVE_TO_SCORES: EEQSNormalizationType = EEQSNormalizationType(1);
}
#[repr(transparent)]
pub struct EEnvQueryHightlightMode(pub u8);
impl EEnvQueryHightlightMode {
    pub const ALL: EEnvQueryHightlightMode = EEnvQueryHightlightMode(0);
    pub const BEST5_PCT: EEnvQueryHightlightMode = EEnvQueryHightlightMode(1);
    pub const BEST25_PCT: EEnvQueryHightlightMode = EEnvQueryHightlightMode(2);
}
#[repr(transparent)]
pub struct EPointOnCircleSpacingMethod(pub u8);
impl EPointOnCircleSpacingMethod {
    pub const BY_SPACE_BETWEEN: EPointOnCircleSpacingMethod = EPointOnCircleSpacingMethod(
        0,
    );
    pub const BY_NUMBER_OF_POINTS: EPointOnCircleSpacingMethod = EPointOnCircleSpacingMethod(
        1,
    );
}
#[repr(transparent)]
pub struct EEnvTestDistance(pub u8);
impl EEnvTestDistance {
    pub const DISTANCE3_D: EEnvTestDistance = EEnvTestDistance(0);
    pub const DISTANCE2_D: EEnvTestDistance = EEnvTestDistance(1);
    pub const DISTANCE_Z: EEnvTestDistance = EEnvTestDistance(2);
    pub const DISTANCE_ABSOLUTE_Z: EEnvTestDistance = EEnvTestDistance(3);
}
#[repr(transparent)]
pub struct EEnvTestDot(pub u8);
impl EEnvTestDot {
    pub const DOT3_D: EEnvTestDot = EEnvTestDot(0);
    pub const DOT2_D: EEnvTestDot = EEnvTestDot(1);
}
#[repr(transparent)]
pub struct EEnvTestPathfinding(pub u8);
impl EEnvTestPathfinding {
    pub const PATH_EXIST: EEnvTestPathfinding = EEnvTestPathfinding(0);
    pub const PATH_COST: EEnvTestPathfinding = EEnvTestPathfinding(1);
    pub const PATH_LENGTH: EEnvTestPathfinding = EEnvTestPathfinding(2);
}
#[repr(transparent)]
pub struct EAISenseNotifyType(pub u8);
impl EAISenseNotifyType {
    pub const ON_EVERY_PERCEPTION: EAISenseNotifyType = EAISenseNotifyType(0);
    pub const ON_PERCEPTION_CHANGE: EAISenseNotifyType = EAISenseNotifyType(1);
}
