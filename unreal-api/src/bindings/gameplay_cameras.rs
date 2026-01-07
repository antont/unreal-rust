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
pub static mut U_CAMERA_RIG_INSTANCE_FUNCTIONS_IS_VALID: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_BLUEPRINT_CAMERA_DIRECTOR_EVALUATOR_RUN_CHILD_CAMERA_DIRECTOR: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_BLUEPRINT_CAMERA_DIRECTOR_EVALUATOR_RUN_CAMERA_DIRECTOR: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_BLUEPRINT_CAMERA_DIRECTOR_EVALUATOR_RESOLVE_CAMERA_RIG_PROXY: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_BLUEPRINT_CAMERA_DIRECTOR_EVALUATOR_REMOVE_CHILD_EVALUATION_CONTEXT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_BLUEPRINT_CAMERA_DIRECTOR_EVALUATOR_GET_INITIAL_CONTEXT_RESULT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_BLUEPRINT_CAMERA_DIRECTOR_EVALUATOR_GET_CONDITIONAL_CONTEXT_RESULT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_BLUEPRINT_CAMERA_DIRECTOR_EVALUATOR_FIND_EVALUATION_CONTEXT_OWNER_ACTOR: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_BLUEPRINT_CAMERA_DIRECTOR_EVALUATOR_DEACTIVATE_PERSISTENT_VISUAL_CAMERA_RIG: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_BLUEPRINT_CAMERA_DIRECTOR_EVALUATOR_DEACTIVATE_PERSISTENT_GLOBAL_CAMERA_RIG: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_BLUEPRINT_CAMERA_DIRECTOR_EVALUATOR_DEACTIVATE_PERSISTENT_BASE_CAMERA_RIG: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_BLUEPRINT_CAMERA_DIRECTOR_EVALUATOR_DEACTIVATE_CAMERA_DIRECTOR: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_BLUEPRINT_CAMERA_DIRECTOR_EVALUATOR_ADD_CHILD_EVALUATION_CONTEXT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_BLUEPRINT_CAMERA_DIRECTOR_EVALUATOR_ACTIVATE_PERSISTENT_VISUAL_CAMERA_RIG: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_BLUEPRINT_CAMERA_DIRECTOR_EVALUATOR_ACTIVATE_PERSISTENT_GLOBAL_CAMERA_RIG: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_BLUEPRINT_CAMERA_DIRECTOR_EVALUATOR_ACTIVATE_PERSISTENT_BASE_CAMERA_RIG: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_BLUEPRINT_CAMERA_DIRECTOR_EVALUATOR_ACTIVATE_CAMERA_RIG_VIA_PROXY: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_BLUEPRINT_CAMERA_DIRECTOR_EVALUATOR_ACTIVATE_CAMERA_RIG: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_BLUEPRINT_CAMERA_DIRECTOR_EVALUATOR_ACTIVATE_CAMERA_DIRECTOR: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_ACTIVATE_CAMERA_RIG_FUNCTIONS_ACTIVATE_PERSISTENT_VISUAL_CAMERA_RIG: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_ACTIVATE_CAMERA_RIG_FUNCTIONS_ACTIVATE_PERSISTENT_GLOBAL_CAMERA_RIG: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_ACTIVATE_CAMERA_RIG_FUNCTIONS_ACTIVATE_PERSISTENT_BASE_CAMERA_RIG: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_BLUEPRINT_CAMERA_EVALUATION_DATA_FUNCTION_LIBRARY_SET_DEFAULT_CAMERA_RIG_PARAMETERS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_BLUEPRINT_CAMERA_EVALUATION_DATA_FUNCTION_LIBRARY_SET_CAMERA_POSE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_BLUEPRINT_CAMERA_EVALUATION_DATA_FUNCTION_LIBRARY_MAKE_CAMERA_EVALUATION_DATA: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_BLUEPRINT_CAMERA_EVALUATION_DATA_FUNCTION_LIBRARY_GET_CAMERA_POSE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_BLUEPRINT_CAMERA_EVALUATION_DATA_FUNCTION_LIBRARY_BLEND_CAMERA_EVALUATION_DATA: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_BLUEPRINT_CAMERA_VARIABLE_TABLE_FUNCTION_LIBRARY_SET_VECTOR4_CAMERA_VARIABLE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_BLUEPRINT_CAMERA_VARIABLE_TABLE_FUNCTION_LIBRARY_SET_VECTOR3_CAMERA_VARIABLE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_BLUEPRINT_CAMERA_VARIABLE_TABLE_FUNCTION_LIBRARY_SET_VECTOR2_CAMERA_VARIABLE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_BLUEPRINT_CAMERA_VARIABLE_TABLE_FUNCTION_LIBRARY_SET_TRANSFORM_CAMERA_VARIABLE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_BLUEPRINT_CAMERA_VARIABLE_TABLE_FUNCTION_LIBRARY_SET_ROTATOR_CAMERA_VARIABLE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_BLUEPRINT_CAMERA_VARIABLE_TABLE_FUNCTION_LIBRARY_SET_INTEGER32_CAMERA_VARIABLE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_BLUEPRINT_CAMERA_VARIABLE_TABLE_FUNCTION_LIBRARY_SET_FLOAT_CAMERA_VARIABLE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_BLUEPRINT_CAMERA_VARIABLE_TABLE_FUNCTION_LIBRARY_SET_DOUBLE_CAMERA_VARIABLE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_BLUEPRINT_CAMERA_VARIABLE_TABLE_FUNCTION_LIBRARY_SET_BOOLEAN_CAMERA_VARIABLE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_BLUEPRINT_CAMERA_VARIABLE_TABLE_FUNCTION_LIBRARY_GET_VECTOR4_CAMERA_VARIABLE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_BLUEPRINT_CAMERA_VARIABLE_TABLE_FUNCTION_LIBRARY_GET_VECTOR3_CAMERA_VARIABLE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_BLUEPRINT_CAMERA_VARIABLE_TABLE_FUNCTION_LIBRARY_GET_VECTOR2_CAMERA_VARIABLE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_BLUEPRINT_CAMERA_VARIABLE_TABLE_FUNCTION_LIBRARY_GET_TRANSFORM_CAMERA_VARIABLE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_BLUEPRINT_CAMERA_VARIABLE_TABLE_FUNCTION_LIBRARY_GET_ROTATOR_CAMERA_VARIABLE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_BLUEPRINT_CAMERA_VARIABLE_TABLE_FUNCTION_LIBRARY_GET_INTEGER32_CAMERA_VARIABLE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_BLUEPRINT_CAMERA_VARIABLE_TABLE_FUNCTION_LIBRARY_GET_FLOAT_CAMERA_VARIABLE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_BLUEPRINT_CAMERA_VARIABLE_TABLE_FUNCTION_LIBRARY_GET_DOUBLE_CAMERA_VARIABLE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_BLUEPRINT_CAMERA_VARIABLE_TABLE_FUNCTION_LIBRARY_GET_BOOLEAN_CAMERA_VARIABLE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_BLUEPRINT_CAMERA_CONTEXT_DATA_TABLE_FUNCTION_LIBRARY_SET_STRUCT_DATA: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_BLUEPRINT_CAMERA_CONTEXT_DATA_TABLE_FUNCTION_LIBRARY_SET_STRING_DATA: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_BLUEPRINT_CAMERA_CONTEXT_DATA_TABLE_FUNCTION_LIBRARY_SET_OBJECT_DATA: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_BLUEPRINT_CAMERA_CONTEXT_DATA_TABLE_FUNCTION_LIBRARY_SET_NAME_DATA: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_BLUEPRINT_CAMERA_CONTEXT_DATA_TABLE_FUNCTION_LIBRARY_SET_ENUM_DATA: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_BLUEPRINT_CAMERA_CONTEXT_DATA_TABLE_FUNCTION_LIBRARY_SET_CLASS_DATA: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_BLUEPRINT_CAMERA_CONTEXT_DATA_TABLE_FUNCTION_LIBRARY_GET_STRUCT_DATA: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_BLUEPRINT_CAMERA_CONTEXT_DATA_TABLE_FUNCTION_LIBRARY_GET_STRING_DATA: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_BLUEPRINT_CAMERA_CONTEXT_DATA_TABLE_FUNCTION_LIBRARY_GET_OBJECT_DATA: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_BLUEPRINT_CAMERA_CONTEXT_DATA_TABLE_FUNCTION_LIBRARY_GET_NAME_DATA: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_BLUEPRINT_CAMERA_CONTEXT_DATA_TABLE_FUNCTION_LIBRARY_GET_ENUM_DATA: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_BLUEPRINT_CAMERA_CONTEXT_DATA_TABLE_FUNCTION_LIBRARY_GET_CLASS_DATA: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_BLUEPRINT_CAMERA_POSE_FUNCTION_LIBRARY_SET_TRANSFORM: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_BLUEPRINT_CAMERA_POSE_FUNCTION_LIBRARY_SET_TARGET_DISTANCE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_BLUEPRINT_CAMERA_POSE_FUNCTION_LIBRARY_SET_ROTATION: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_BLUEPRINT_CAMERA_POSE_FUNCTION_LIBRARY_SET_LOCATION: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_BLUEPRINT_CAMERA_POSE_FUNCTION_LIBRARY_SET_FOCAL_LENGTH: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_BLUEPRINT_CAMERA_POSE_FUNCTION_LIBRARY_SET_FIELD_OF_VIEW: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_BLUEPRINT_CAMERA_POSE_FUNCTION_LIBRARY_MAKE_CAMERA_POSE_FROM_CINE_CAMERA_COMPONENT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_BLUEPRINT_CAMERA_POSE_FUNCTION_LIBRARY_MAKE_CAMERA_POSE_FROM_CAMERA_COMPONENT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_BLUEPRINT_CAMERA_POSE_FUNCTION_LIBRARY_GET_TRANSFORM: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_BLUEPRINT_CAMERA_POSE_FUNCTION_LIBRARY_GET_TARGET_DISTANCE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_BLUEPRINT_CAMERA_POSE_FUNCTION_LIBRARY_GET_TARGET_AT_DISTANCE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_BLUEPRINT_CAMERA_POSE_FUNCTION_LIBRARY_GET_TARGET: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_BLUEPRINT_CAMERA_POSE_FUNCTION_LIBRARY_GET_SENSOR_ASPECT_RATIO: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_BLUEPRINT_CAMERA_POSE_FUNCTION_LIBRARY_GET_ROTATION: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_BLUEPRINT_CAMERA_POSE_FUNCTION_LIBRARY_GET_LOCATION: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_BLUEPRINT_CAMERA_POSE_FUNCTION_LIBRARY_GET_FOCAL_LENGTH: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_BLUEPRINT_CAMERA_POSE_FUNCTION_LIBRARY_GET_FIELD_OF_VIEW: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_BLUEPRINT_CAMERA_POSE_FUNCTION_LIBRARY_GET_EFFECTIVE_FIELD_OF_VIEW: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_BLUEPRINT_CAMERA_POSE_FUNCTION_LIBRARY_GET_AIM_RAY: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_BLUEPRINT_CAMERA_POSE_FUNCTION_LIBRARY_GET_AIM_DIR: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_CAMERA_RIG_PARAMETER_INTEROP_SET_CAMERA_PARAMETER: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_CAMERA_RIG_PARAMETER_INTEROP_GET_CAMERA_PARAMETER: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_CAMERA_RIG_PARAMETER_INTEROP_LIBRARY_MAKE_LITERAL_VECTOR3F: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_CAMERA_RIG_PARAMETER_INTEROP_LIBRARY_MAKE_LITERAL_VECTOR2_D: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_CAMERA_RIG_PARAMETER_INTEROP_LIBRARY_MAKE_LITERAL_VECTOR: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_CAMERA_RIG_PARAMETER_INTEROP_LIBRARY_MAKE_LITERAL_ROTATOR: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_CAMERA_RIG_PARAMETER_INTEROP_LIBRARY_MAKE_LITERAL_LINEAR_COLOR: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut A_GAMEPLAY_CAMERA_ACTOR_GET_CAMERA_COMPONENT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_GAMEPLAY_CAMERA_COMPONENT_BASE_GET_OUTPUT_CAMERA_COMPONENT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_GAMEPLAY_CAMERA_COMPONENT_BASE_GET_INITIAL_RESULT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_GAMEPLAY_CAMERA_COMPONENT_BASE_GET_EVALUATED_CAMERA_ROTATION: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_GAMEPLAY_CAMERA_COMPONENT_BASE_GET_CONDITIONAL_RESULT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_GAMEPLAY_CAMERA_COMPONENT_BASE_DEACTIVATE_CAMERA: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_GAMEPLAY_CAMERA_COMPONENT_BASE_ACTIVATE_PERSISTENT_VISUAL_CAMERA_RIG: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_GAMEPLAY_CAMERA_COMPONENT_BASE_ACTIVATE_PERSISTENT_GLOBAL_CAMERA_RIG: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_GAMEPLAY_CAMERA_COMPONENT_BASE_ACTIVATE_PERSISTENT_BASE_CAMERA_RIG: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_GAMEPLAY_CAMERA_COMPONENT_BASE_ACTIVATE_CAMERA_FOR_PLAYER_INDEX: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_GAMEPLAY_CAMERA_COMPONENT_BASE_ACTIVATE_CAMERA_FOR_PLAYER_CONTROLLER: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_GAMEPLAY_CAMERA_COMPONENT_NOTIFY_CHANGE_CAMERA_REFERENCE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_GAMEPLAY_CAMERA_PARAMETER_SETTER_COMPONENT_STOP_PARAMETER_SETTERS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_GAMEPLAY_CAMERA_PARAMETER_SETTER_COMPONENT_START_PARAMETER_SETTERS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_GAMEPLAY_CAMERA_PARAMETER_SETTER_COMPONENT_ON_ACTOR_END_OVERLAP: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_GAMEPLAY_CAMERA_PARAMETER_SETTER_COMPONENT_ON_ACTOR_BEGIN_OVERLAP: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut A_GAMEPLAY_CAMERA_RIG_ACTOR_GET_CAMERA_RIG_COMPONENT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_GAMEPLAY_CAMERA_RIG_COMPONENT_NOTIFY_CHANGE_CAMERA_RIG_REFERENCE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut A_GAMEPLAY_CAMERAS_PLAYER_CAMERA_MANAGER_STOP_CAMERA_SHAKE_ASSET: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut A_GAMEPLAY_CAMERAS_PLAYER_CAMERA_MANAGER_STOP_CAMERA_MODIFIER_RIG: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut A_GAMEPLAY_CAMERAS_PLAYER_CAMERA_MANAGER_STEAL_PLAYER_CONTROLLER: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut A_GAMEPLAY_CAMERAS_PLAYER_CAMERA_MANAGER_START_VISUAL_CAMERA_MODIFIER_RIG: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut A_GAMEPLAY_CAMERAS_PLAYER_CAMERA_MANAGER_START_GLOBAL_CAMERA_MODIFIER_RIG: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut A_GAMEPLAY_CAMERAS_PLAYER_CAMERA_MANAGER_START_CAMERA_SHAKE_ASSET: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut A_GAMEPLAY_CAMERAS_PLAYER_CAMERA_MANAGER_RELEASE_PLAYER_CONTROLLER: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut A_GAMEPLAY_CAMERAS_PLAYER_CAMERA_MANAGER_IS_CAMERA_SHAKE_ASSET_PLAYING: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut A_GAMEPLAY_CAMERAS_PLAYER_CAMERA_MANAGER_ACTIVATE_PERSISTENT_VISUAL_CAMERA_RIG: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut A_GAMEPLAY_CAMERAS_PLAYER_CAMERA_MANAGER_ACTIVATE_PERSISTENT_GLOBAL_CAMERA_RIG: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut A_GAMEPLAY_CAMERAS_PLAYER_CAMERA_MANAGER_ACTIVATE_PERSISTENT_BASE_CAMERA_RIG: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut A_GAMEPLAY_CAMERA_SYSTEM_ACTOR_GET_CAMERA_SYSTEM_COMPONENT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_GAMEPLAY_CAMERA_SYSTEM_COMPONENT_STOP_CAMERA_MODIFIER_RIG: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_GAMEPLAY_CAMERA_SYSTEM_COMPONENT_START_VISUAL_CAMERA_MODIFIER_RIG: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_GAMEPLAY_CAMERA_SYSTEM_COMPONENT_START_GLOBAL_CAMERA_MODIFIER_RIG: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_GAMEPLAY_CAMERA_SYSTEM_COMPONENT_IS_CAMERA_SYSTEM_ACTIVE_FOR_PLAY_CONTROLLER: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_GAMEPLAY_CAMERA_SYSTEM_COMPONENT_DEACTIVATE_CAMERA_SYSTEM: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_GAMEPLAY_CAMERA_SYSTEM_COMPONENT_ACTIVATE_CAMERA_SYSTEM_FOR_PLAYER_INDEX: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_GAMEPLAY_CAMERA_SYSTEM_COMPONENT_ACTIVATE_CAMERA_SYSTEM_FOR_PLAYER_CONTROLLER: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_GAMEPLAY_CONTROL_ROTATION_COMPONENT_DEACTIVATE_CONTROL_ROTATION_MANAGEMENT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_GAMEPLAY_CONTROL_ROTATION_COMPONENT_ACTIVATE_CONTROL_ROTATION_MANAGEMENT_FOR_PLAYER_INDEX: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_GAMEPLAY_CONTROL_ROTATION_COMPONENT_ACTIVATE_CONTROL_ROTATION_MANAGEMENT_FOR_PLAYER_CONTROLLER: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_BLUEPRINT_CAMERA_NODE_EVALUATOR_TICK_CAMERA_NODE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_BLUEPRINT_CAMERA_NODE_EVALUATOR_SET_DEFAULT_OWNING_CAMERA_RIG_PARAMETERS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_BLUEPRINT_CAMERA_NODE_EVALUATOR_SET_CURRENT_CAMERA_POSE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_BLUEPRINT_CAMERA_NODE_EVALUATOR_SET_CAMERA_POSE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_BLUEPRINT_CAMERA_NODE_EVALUATOR_INITIALIZE_CAMERA_NODE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_BLUEPRINT_CAMERA_NODE_EVALUATOR_GET_PLAYER_CONTROLLER: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_BLUEPRINT_CAMERA_NODE_EVALUATOR_GET_CURRENT_CAMERA_POSE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_BLUEPRINT_CAMERA_NODE_EVALUATOR_GET_CAMERA_POSE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_BLUEPRINT_CAMERA_NODE_EVALUATOR_FIND_EVALUATION_CONTEXT_OWNER_ACTOR: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
pub fn initialize() {
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UCameraRigInstanceFunctions::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("IsValid"),
            &raw mut U_CAMERA_RIG_INSTANCE_FUNCTIONS_IS_VALID,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UBlueprintCameraDirectorEvaluator::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RunChildCameraDirector"),
            &raw mut U_BLUEPRINT_CAMERA_DIRECTOR_EVALUATOR_RUN_CHILD_CAMERA_DIRECTOR,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RunCameraDirector"),
            &raw mut U_BLUEPRINT_CAMERA_DIRECTOR_EVALUATOR_RUN_CAMERA_DIRECTOR,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ResolveCameraRigProxy"),
            &raw mut U_BLUEPRINT_CAMERA_DIRECTOR_EVALUATOR_RESOLVE_CAMERA_RIG_PROXY,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RemoveChildEvaluationContext"),
            &raw mut U_BLUEPRINT_CAMERA_DIRECTOR_EVALUATOR_REMOVE_CHILD_EVALUATION_CONTEXT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetInitialContextResult"),
            &raw mut U_BLUEPRINT_CAMERA_DIRECTOR_EVALUATOR_GET_INITIAL_CONTEXT_RESULT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetConditionalContextResult"),
            &raw mut U_BLUEPRINT_CAMERA_DIRECTOR_EVALUATOR_GET_CONDITIONAL_CONTEXT_RESULT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("FindEvaluationContextOwnerActor"),
            &raw mut U_BLUEPRINT_CAMERA_DIRECTOR_EVALUATOR_FIND_EVALUATION_CONTEXT_OWNER_ACTOR,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("DeactivatePersistentVisualCameraRig"),
            &raw mut U_BLUEPRINT_CAMERA_DIRECTOR_EVALUATOR_DEACTIVATE_PERSISTENT_VISUAL_CAMERA_RIG,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("DeactivatePersistentGlobalCameraRig"),
            &raw mut U_BLUEPRINT_CAMERA_DIRECTOR_EVALUATOR_DEACTIVATE_PERSISTENT_GLOBAL_CAMERA_RIG,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("DeactivatePersistentBaseCameraRig"),
            &raw mut U_BLUEPRINT_CAMERA_DIRECTOR_EVALUATOR_DEACTIVATE_PERSISTENT_BASE_CAMERA_RIG,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("DeactivateCameraDirector"),
            &raw mut U_BLUEPRINT_CAMERA_DIRECTOR_EVALUATOR_DEACTIVATE_CAMERA_DIRECTOR,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AddChildEvaluationContext"),
            &raw mut U_BLUEPRINT_CAMERA_DIRECTOR_EVALUATOR_ADD_CHILD_EVALUATION_CONTEXT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ActivatePersistentVisualCameraRig"),
            &raw mut U_BLUEPRINT_CAMERA_DIRECTOR_EVALUATOR_ACTIVATE_PERSISTENT_VISUAL_CAMERA_RIG,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ActivatePersistentGlobalCameraRig"),
            &raw mut U_BLUEPRINT_CAMERA_DIRECTOR_EVALUATOR_ACTIVATE_PERSISTENT_GLOBAL_CAMERA_RIG,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ActivatePersistentBaseCameraRig"),
            &raw mut U_BLUEPRINT_CAMERA_DIRECTOR_EVALUATOR_ACTIVATE_PERSISTENT_BASE_CAMERA_RIG,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ActivateCameraRigViaProxy"),
            &raw mut U_BLUEPRINT_CAMERA_DIRECTOR_EVALUATOR_ACTIVATE_CAMERA_RIG_VIA_PROXY,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ActivateCameraRig"),
            &raw mut U_BLUEPRINT_CAMERA_DIRECTOR_EVALUATOR_ACTIVATE_CAMERA_RIG,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ActivateCameraDirector"),
            &raw mut U_BLUEPRINT_CAMERA_DIRECTOR_EVALUATOR_ACTIVATE_CAMERA_DIRECTOR,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UActivateCameraRigFunctions::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ActivatePersistentVisualCameraRig"),
            &raw mut U_ACTIVATE_CAMERA_RIG_FUNCTIONS_ACTIVATE_PERSISTENT_VISUAL_CAMERA_RIG,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ActivatePersistentGlobalCameraRig"),
            &raw mut U_ACTIVATE_CAMERA_RIG_FUNCTIONS_ACTIVATE_PERSISTENT_GLOBAL_CAMERA_RIG,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ActivatePersistentBaseCameraRig"),
            &raw mut U_ACTIVATE_CAMERA_RIG_FUNCTIONS_ACTIVATE_PERSISTENT_BASE_CAMERA_RIG,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UBlueprintCameraEvaluationDataFunctionLibrary::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetDefaultCameraRigParameters"),
            &raw mut U_BLUEPRINT_CAMERA_EVALUATION_DATA_FUNCTION_LIBRARY_SET_DEFAULT_CAMERA_RIG_PARAMETERS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetCameraPose"),
            &raw mut U_BLUEPRINT_CAMERA_EVALUATION_DATA_FUNCTION_LIBRARY_SET_CAMERA_POSE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("MakeCameraEvaluationData"),
            &raw mut U_BLUEPRINT_CAMERA_EVALUATION_DATA_FUNCTION_LIBRARY_MAKE_CAMERA_EVALUATION_DATA,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetCameraPose"),
            &raw mut U_BLUEPRINT_CAMERA_EVALUATION_DATA_FUNCTION_LIBRARY_GET_CAMERA_POSE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("BlendCameraEvaluationData"),
            &raw mut U_BLUEPRINT_CAMERA_EVALUATION_DATA_FUNCTION_LIBRARY_BLEND_CAMERA_EVALUATION_DATA,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UBlueprintCameraVariableTableFunctionLibrary::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetVector4CameraVariable"),
            &raw mut U_BLUEPRINT_CAMERA_VARIABLE_TABLE_FUNCTION_LIBRARY_SET_VECTOR4_CAMERA_VARIABLE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetVector3CameraVariable"),
            &raw mut U_BLUEPRINT_CAMERA_VARIABLE_TABLE_FUNCTION_LIBRARY_SET_VECTOR3_CAMERA_VARIABLE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetVector2CameraVariable"),
            &raw mut U_BLUEPRINT_CAMERA_VARIABLE_TABLE_FUNCTION_LIBRARY_SET_VECTOR2_CAMERA_VARIABLE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetTransformCameraVariable"),
            &raw mut U_BLUEPRINT_CAMERA_VARIABLE_TABLE_FUNCTION_LIBRARY_SET_TRANSFORM_CAMERA_VARIABLE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetRotatorCameraVariable"),
            &raw mut U_BLUEPRINT_CAMERA_VARIABLE_TABLE_FUNCTION_LIBRARY_SET_ROTATOR_CAMERA_VARIABLE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetInteger32CameraVariable"),
            &raw mut U_BLUEPRINT_CAMERA_VARIABLE_TABLE_FUNCTION_LIBRARY_SET_INTEGER32_CAMERA_VARIABLE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetFloatCameraVariable"),
            &raw mut U_BLUEPRINT_CAMERA_VARIABLE_TABLE_FUNCTION_LIBRARY_SET_FLOAT_CAMERA_VARIABLE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetDoubleCameraVariable"),
            &raw mut U_BLUEPRINT_CAMERA_VARIABLE_TABLE_FUNCTION_LIBRARY_SET_DOUBLE_CAMERA_VARIABLE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetBooleanCameraVariable"),
            &raw mut U_BLUEPRINT_CAMERA_VARIABLE_TABLE_FUNCTION_LIBRARY_SET_BOOLEAN_CAMERA_VARIABLE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetVector4CameraVariable"),
            &raw mut U_BLUEPRINT_CAMERA_VARIABLE_TABLE_FUNCTION_LIBRARY_GET_VECTOR4_CAMERA_VARIABLE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetVector3CameraVariable"),
            &raw mut U_BLUEPRINT_CAMERA_VARIABLE_TABLE_FUNCTION_LIBRARY_GET_VECTOR3_CAMERA_VARIABLE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetVector2CameraVariable"),
            &raw mut U_BLUEPRINT_CAMERA_VARIABLE_TABLE_FUNCTION_LIBRARY_GET_VECTOR2_CAMERA_VARIABLE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetTransformCameraVariable"),
            &raw mut U_BLUEPRINT_CAMERA_VARIABLE_TABLE_FUNCTION_LIBRARY_GET_TRANSFORM_CAMERA_VARIABLE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetRotatorCameraVariable"),
            &raw mut U_BLUEPRINT_CAMERA_VARIABLE_TABLE_FUNCTION_LIBRARY_GET_ROTATOR_CAMERA_VARIABLE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetInteger32CameraVariable"),
            &raw mut U_BLUEPRINT_CAMERA_VARIABLE_TABLE_FUNCTION_LIBRARY_GET_INTEGER32_CAMERA_VARIABLE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetFloatCameraVariable"),
            &raw mut U_BLUEPRINT_CAMERA_VARIABLE_TABLE_FUNCTION_LIBRARY_GET_FLOAT_CAMERA_VARIABLE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetDoubleCameraVariable"),
            &raw mut U_BLUEPRINT_CAMERA_VARIABLE_TABLE_FUNCTION_LIBRARY_GET_DOUBLE_CAMERA_VARIABLE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetBooleanCameraVariable"),
            &raw mut U_BLUEPRINT_CAMERA_VARIABLE_TABLE_FUNCTION_LIBRARY_GET_BOOLEAN_CAMERA_VARIABLE,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UBlueprintCameraContextDataTableFunctionLibrary::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetStructData"),
            &raw mut U_BLUEPRINT_CAMERA_CONTEXT_DATA_TABLE_FUNCTION_LIBRARY_SET_STRUCT_DATA,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetStringData"),
            &raw mut U_BLUEPRINT_CAMERA_CONTEXT_DATA_TABLE_FUNCTION_LIBRARY_SET_STRING_DATA,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetObjectData"),
            &raw mut U_BLUEPRINT_CAMERA_CONTEXT_DATA_TABLE_FUNCTION_LIBRARY_SET_OBJECT_DATA,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetNameData"),
            &raw mut U_BLUEPRINT_CAMERA_CONTEXT_DATA_TABLE_FUNCTION_LIBRARY_SET_NAME_DATA,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetEnumData"),
            &raw mut U_BLUEPRINT_CAMERA_CONTEXT_DATA_TABLE_FUNCTION_LIBRARY_SET_ENUM_DATA,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetClassData"),
            &raw mut U_BLUEPRINT_CAMERA_CONTEXT_DATA_TABLE_FUNCTION_LIBRARY_SET_CLASS_DATA,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetStructData"),
            &raw mut U_BLUEPRINT_CAMERA_CONTEXT_DATA_TABLE_FUNCTION_LIBRARY_GET_STRUCT_DATA,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetStringData"),
            &raw mut U_BLUEPRINT_CAMERA_CONTEXT_DATA_TABLE_FUNCTION_LIBRARY_GET_STRING_DATA,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetObjectData"),
            &raw mut U_BLUEPRINT_CAMERA_CONTEXT_DATA_TABLE_FUNCTION_LIBRARY_GET_OBJECT_DATA,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetNameData"),
            &raw mut U_BLUEPRINT_CAMERA_CONTEXT_DATA_TABLE_FUNCTION_LIBRARY_GET_NAME_DATA,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetEnumData"),
            &raw mut U_BLUEPRINT_CAMERA_CONTEXT_DATA_TABLE_FUNCTION_LIBRARY_GET_ENUM_DATA,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetClassData"),
            &raw mut U_BLUEPRINT_CAMERA_CONTEXT_DATA_TABLE_FUNCTION_LIBRARY_GET_CLASS_DATA,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UBlueprintCameraPoseFunctionLibrary::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetTransform"),
            &raw mut U_BLUEPRINT_CAMERA_POSE_FUNCTION_LIBRARY_SET_TRANSFORM,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetTargetDistance"),
            &raw mut U_BLUEPRINT_CAMERA_POSE_FUNCTION_LIBRARY_SET_TARGET_DISTANCE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetRotation"),
            &raw mut U_BLUEPRINT_CAMERA_POSE_FUNCTION_LIBRARY_SET_ROTATION,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetLocation"),
            &raw mut U_BLUEPRINT_CAMERA_POSE_FUNCTION_LIBRARY_SET_LOCATION,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetFocalLength"),
            &raw mut U_BLUEPRINT_CAMERA_POSE_FUNCTION_LIBRARY_SET_FOCAL_LENGTH,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetFieldOfView"),
            &raw mut U_BLUEPRINT_CAMERA_POSE_FUNCTION_LIBRARY_SET_FIELD_OF_VIEW,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("MakeCameraPoseFromCineCameraComponent"),
            &raw mut U_BLUEPRINT_CAMERA_POSE_FUNCTION_LIBRARY_MAKE_CAMERA_POSE_FROM_CINE_CAMERA_COMPONENT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("MakeCameraPoseFromCameraComponent"),
            &raw mut U_BLUEPRINT_CAMERA_POSE_FUNCTION_LIBRARY_MAKE_CAMERA_POSE_FROM_CAMERA_COMPONENT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetTransform"),
            &raw mut U_BLUEPRINT_CAMERA_POSE_FUNCTION_LIBRARY_GET_TRANSFORM,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetTargetDistance"),
            &raw mut U_BLUEPRINT_CAMERA_POSE_FUNCTION_LIBRARY_GET_TARGET_DISTANCE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetTargetAtDistance"),
            &raw mut U_BLUEPRINT_CAMERA_POSE_FUNCTION_LIBRARY_GET_TARGET_AT_DISTANCE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetTarget"),
            &raw mut U_BLUEPRINT_CAMERA_POSE_FUNCTION_LIBRARY_GET_TARGET,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetSensorAspectRatio"),
            &raw mut U_BLUEPRINT_CAMERA_POSE_FUNCTION_LIBRARY_GET_SENSOR_ASPECT_RATIO,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetRotation"),
            &raw mut U_BLUEPRINT_CAMERA_POSE_FUNCTION_LIBRARY_GET_ROTATION,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetLocation"),
            &raw mut U_BLUEPRINT_CAMERA_POSE_FUNCTION_LIBRARY_GET_LOCATION,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetFocalLength"),
            &raw mut U_BLUEPRINT_CAMERA_POSE_FUNCTION_LIBRARY_GET_FOCAL_LENGTH,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetFieldOfView"),
            &raw mut U_BLUEPRINT_CAMERA_POSE_FUNCTION_LIBRARY_GET_FIELD_OF_VIEW,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetEffectiveFieldOfView"),
            &raw mut U_BLUEPRINT_CAMERA_POSE_FUNCTION_LIBRARY_GET_EFFECTIVE_FIELD_OF_VIEW,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetAimRay"),
            &raw mut U_BLUEPRINT_CAMERA_POSE_FUNCTION_LIBRARY_GET_AIM_RAY,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetAimDir"),
            &raw mut U_BLUEPRINT_CAMERA_POSE_FUNCTION_LIBRARY_GET_AIM_DIR,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UCameraRigParameterInterop::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetCameraParameter"),
            &raw mut U_CAMERA_RIG_PARAMETER_INTEROP_SET_CAMERA_PARAMETER,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetCameraParameter"),
            &raw mut U_CAMERA_RIG_PARAMETER_INTEROP_GET_CAMERA_PARAMETER,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UCameraRigParameterInteropLibrary::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("MakeLiteralVector3f"),
            &raw mut U_CAMERA_RIG_PARAMETER_INTEROP_LIBRARY_MAKE_LITERAL_VECTOR3F,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("MakeLiteralVector2D"),
            &raw mut U_CAMERA_RIG_PARAMETER_INTEROP_LIBRARY_MAKE_LITERAL_VECTOR2_D,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("MakeLiteralVector"),
            &raw mut U_CAMERA_RIG_PARAMETER_INTEROP_LIBRARY_MAKE_LITERAL_VECTOR,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("MakeLiteralRotator"),
            &raw mut U_CAMERA_RIG_PARAMETER_INTEROP_LIBRARY_MAKE_LITERAL_ROTATOR,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("MakeLiteralLinearColor"),
            &raw mut U_CAMERA_RIG_PARAMETER_INTEROP_LIBRARY_MAKE_LITERAL_LINEAR_COLOR,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = AGameplayCameraActor::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetCameraComponent"),
            &raw mut A_GAMEPLAY_CAMERA_ACTOR_GET_CAMERA_COMPONENT,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UGameplayCameraComponentBase::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetOutputCameraComponent"),
            &raw mut U_GAMEPLAY_CAMERA_COMPONENT_BASE_GET_OUTPUT_CAMERA_COMPONENT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetInitialResult"),
            &raw mut U_GAMEPLAY_CAMERA_COMPONENT_BASE_GET_INITIAL_RESULT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetEvaluatedCameraRotation"),
            &raw mut U_GAMEPLAY_CAMERA_COMPONENT_BASE_GET_EVALUATED_CAMERA_ROTATION,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetConditionalResult"),
            &raw mut U_GAMEPLAY_CAMERA_COMPONENT_BASE_GET_CONDITIONAL_RESULT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("DeactivateCamera"),
            &raw mut U_GAMEPLAY_CAMERA_COMPONENT_BASE_DEACTIVATE_CAMERA,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ActivatePersistentVisualCameraRig"),
            &raw mut U_GAMEPLAY_CAMERA_COMPONENT_BASE_ACTIVATE_PERSISTENT_VISUAL_CAMERA_RIG,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ActivatePersistentGlobalCameraRig"),
            &raw mut U_GAMEPLAY_CAMERA_COMPONENT_BASE_ACTIVATE_PERSISTENT_GLOBAL_CAMERA_RIG,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ActivatePersistentBaseCameraRig"),
            &raw mut U_GAMEPLAY_CAMERA_COMPONENT_BASE_ACTIVATE_PERSISTENT_BASE_CAMERA_RIG,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ActivateCameraForPlayerIndex"),
            &raw mut U_GAMEPLAY_CAMERA_COMPONENT_BASE_ACTIVATE_CAMERA_FOR_PLAYER_INDEX,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ActivateCameraForPlayerController"),
            &raw mut U_GAMEPLAY_CAMERA_COMPONENT_BASE_ACTIVATE_CAMERA_FOR_PLAYER_CONTROLLER,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UGameplayCameraComponent::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("NotifyChangeCameraReference"),
            &raw mut U_GAMEPLAY_CAMERA_COMPONENT_NOTIFY_CHANGE_CAMERA_REFERENCE,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UGameplayCameraParameterSetterComponent::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("StopParameterSetters"),
            &raw mut U_GAMEPLAY_CAMERA_PARAMETER_SETTER_COMPONENT_STOP_PARAMETER_SETTERS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("StartParameterSetters"),
            &raw mut U_GAMEPLAY_CAMERA_PARAMETER_SETTER_COMPONENT_START_PARAMETER_SETTERS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("OnActorEndOverlap"),
            &raw mut U_GAMEPLAY_CAMERA_PARAMETER_SETTER_COMPONENT_ON_ACTOR_END_OVERLAP,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("OnActorBeginOverlap"),
            &raw mut U_GAMEPLAY_CAMERA_PARAMETER_SETTER_COMPONENT_ON_ACTOR_BEGIN_OVERLAP,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = AGameplayCameraRigActor::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetCameraRigComponent"),
            &raw mut A_GAMEPLAY_CAMERA_RIG_ACTOR_GET_CAMERA_RIG_COMPONENT,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UGameplayCameraRigComponent::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("NotifyChangeCameraRigReference"),
            &raw mut U_GAMEPLAY_CAMERA_RIG_COMPONENT_NOTIFY_CHANGE_CAMERA_RIG_REFERENCE,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = AGameplayCamerasPlayerCameraManager::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("StopCameraShakeAsset"),
            &raw mut A_GAMEPLAY_CAMERAS_PLAYER_CAMERA_MANAGER_STOP_CAMERA_SHAKE_ASSET,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("StopCameraModifierRig"),
            &raw mut A_GAMEPLAY_CAMERAS_PLAYER_CAMERA_MANAGER_STOP_CAMERA_MODIFIER_RIG,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("StealPlayerController"),
            &raw mut A_GAMEPLAY_CAMERAS_PLAYER_CAMERA_MANAGER_STEAL_PLAYER_CONTROLLER,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("StartVisualCameraModifierRig"),
            &raw mut A_GAMEPLAY_CAMERAS_PLAYER_CAMERA_MANAGER_START_VISUAL_CAMERA_MODIFIER_RIG,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("StartGlobalCameraModifierRig"),
            &raw mut A_GAMEPLAY_CAMERAS_PLAYER_CAMERA_MANAGER_START_GLOBAL_CAMERA_MODIFIER_RIG,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("StartCameraShakeAsset"),
            &raw mut A_GAMEPLAY_CAMERAS_PLAYER_CAMERA_MANAGER_START_CAMERA_SHAKE_ASSET,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ReleasePlayerController"),
            &raw mut A_GAMEPLAY_CAMERAS_PLAYER_CAMERA_MANAGER_RELEASE_PLAYER_CONTROLLER,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("IsCameraShakeAssetPlaying"),
            &raw mut A_GAMEPLAY_CAMERAS_PLAYER_CAMERA_MANAGER_IS_CAMERA_SHAKE_ASSET_PLAYING,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ActivatePersistentVisualCameraRig"),
            &raw mut A_GAMEPLAY_CAMERAS_PLAYER_CAMERA_MANAGER_ACTIVATE_PERSISTENT_VISUAL_CAMERA_RIG,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ActivatePersistentGlobalCameraRig"),
            &raw mut A_GAMEPLAY_CAMERAS_PLAYER_CAMERA_MANAGER_ACTIVATE_PERSISTENT_GLOBAL_CAMERA_RIG,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ActivatePersistentBaseCameraRig"),
            &raw mut A_GAMEPLAY_CAMERAS_PLAYER_CAMERA_MANAGER_ACTIVATE_PERSISTENT_BASE_CAMERA_RIG,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = AGameplayCameraSystemActor::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetCameraSystemComponent"),
            &raw mut A_GAMEPLAY_CAMERA_SYSTEM_ACTOR_GET_CAMERA_SYSTEM_COMPONENT,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UGameplayCameraSystemComponent::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("StopCameraModifierRig"),
            &raw mut U_GAMEPLAY_CAMERA_SYSTEM_COMPONENT_STOP_CAMERA_MODIFIER_RIG,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("StartVisualCameraModifierRig"),
            &raw mut U_GAMEPLAY_CAMERA_SYSTEM_COMPONENT_START_VISUAL_CAMERA_MODIFIER_RIG,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("StartGlobalCameraModifierRig"),
            &raw mut U_GAMEPLAY_CAMERA_SYSTEM_COMPONENT_START_GLOBAL_CAMERA_MODIFIER_RIG,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("IsCameraSystemActiveForPlayController"),
            &raw mut U_GAMEPLAY_CAMERA_SYSTEM_COMPONENT_IS_CAMERA_SYSTEM_ACTIVE_FOR_PLAY_CONTROLLER,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("DeactivateCameraSystem"),
            &raw mut U_GAMEPLAY_CAMERA_SYSTEM_COMPONENT_DEACTIVATE_CAMERA_SYSTEM,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ActivateCameraSystemForPlayerIndex"),
            &raw mut U_GAMEPLAY_CAMERA_SYSTEM_COMPONENT_ACTIVATE_CAMERA_SYSTEM_FOR_PLAYER_INDEX,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ActivateCameraSystemForPlayerController"),
            &raw mut U_GAMEPLAY_CAMERA_SYSTEM_COMPONENT_ACTIVATE_CAMERA_SYSTEM_FOR_PLAYER_CONTROLLER,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UGameplayControlRotationComponent::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("DeactivateControlRotationManagement"),
            &raw mut U_GAMEPLAY_CONTROL_ROTATION_COMPONENT_DEACTIVATE_CONTROL_ROTATION_MANAGEMENT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ActivateControlRotationManagementForPlayerIndex"),
            &raw mut U_GAMEPLAY_CONTROL_ROTATION_COMPONENT_ACTIVATE_CONTROL_ROTATION_MANAGEMENT_FOR_PLAYER_INDEX,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from(
                "ActivateControlRotationManagementForPlayerController",
            ),
            &raw mut U_GAMEPLAY_CONTROL_ROTATION_COMPONENT_ACTIVATE_CONTROL_ROTATION_MANAGEMENT_FOR_PLAYER_CONTROLLER,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UBlueprintCameraNodeEvaluator::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("TickCameraNode"),
            &raw mut U_BLUEPRINT_CAMERA_NODE_EVALUATOR_TICK_CAMERA_NODE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetDefaultOwningCameraRigParameters"),
            &raw mut U_BLUEPRINT_CAMERA_NODE_EVALUATOR_SET_DEFAULT_OWNING_CAMERA_RIG_PARAMETERS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetCurrentCameraPose"),
            &raw mut U_BLUEPRINT_CAMERA_NODE_EVALUATOR_SET_CURRENT_CAMERA_POSE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetCameraPose"),
            &raw mut U_BLUEPRINT_CAMERA_NODE_EVALUATOR_SET_CAMERA_POSE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("InitializeCameraNode"),
            &raw mut U_BLUEPRINT_CAMERA_NODE_EVALUATOR_INITIALIZE_CAMERA_NODE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetPlayerController"),
            &raw mut U_BLUEPRINT_CAMERA_NODE_EVALUATOR_GET_PLAYER_CONTROLLER,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetCurrentCameraPose"),
            &raw mut U_BLUEPRINT_CAMERA_NODE_EVALUATOR_GET_CURRENT_CAMERA_POSE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetCameraPose"),
            &raw mut U_BLUEPRINT_CAMERA_NODE_EVALUATOR_GET_CAMERA_POSE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("FindEvaluationContextOwnerActor"),
            &raw mut U_BLUEPRINT_CAMERA_NODE_EVALUATOR_FIND_EVALUATION_CONTEXT_OWNER_ACTOR,
        );
    }
}
#[repr(C, align(4))]
pub struct FCameraContextDataID {
    __padding_end: [u8; 4],
}
impl FCameraContextDataID {}
#[repr(C, align(4))]
pub struct FCameraRigInstanceID {
    __padding_end: [u8; 8],
}
impl FCameraRigInstanceID {}
#[repr(C, align(4))]
pub struct FCameraShakeInstanceID {
    __padding_end: [u8; 4],
}
impl FCameraShakeInstanceID {}
#[repr(C, align(8))]
pub struct FBaseCameraObjectReference {
    __padding_end: [u8; 56],
}
impl FBaseCameraObjectReference {}
#[repr(C, align(8))]
pub struct FCameraAssetReference {
    __padding_end: [u8; 56],
}
impl FCameraAssetReference {}
#[repr(C, align(8))]
pub struct FBooleanCameraParameter {
    pub value: bool,
    __padding_end: [u8; 15],
}
impl FBooleanCameraParameter {}
#[repr(C, align(8))]
pub struct FInteger32CameraParameter {
    pub value: i32,
    __padding_end: [u8; 12],
}
impl FInteger32CameraParameter {}
#[repr(C, align(8))]
pub struct FFloatCameraParameter {
    pub value: f32,
    __padding_end: [u8; 12],
}
impl FFloatCameraParameter {}
#[repr(C, align(8))]
pub struct FDoubleCameraParameter {
    pub value: f64,
    __padding_end: [u8; 16],
}
impl FDoubleCameraParameter {}
#[repr(C, align(8))]
pub struct FVector2fCameraParameter {
    pub value: crate::bindings::core_u_object::FVector2f,
    __padding_end: [u8; 16],
}
impl FVector2fCameraParameter {}
#[repr(C, align(8))]
pub struct FVector2dCameraParameter {
    pub value: crate::bindings::core_u_object::FVector2D,
    __padding_end: [u8; 16],
}
impl FVector2dCameraParameter {}
#[repr(C, align(8))]
pub struct FVector3fCameraParameter {
    pub value: crate::bindings::core_u_object::FVector3f,
    __padding_end: [u8; 12],
}
impl FVector3fCameraParameter {}
#[repr(C, align(8))]
pub struct FVector3dCameraParameter {
    pub value: crate::bindings::core_u_object::FVector,
    __padding_end: [u8; 16],
}
impl FVector3dCameraParameter {}
#[repr(C, align(16))]
pub struct FVector4fCameraParameter {
    pub value: crate::bindings::core_u_object::FVector4f,
    __padding_end: [u8; 16],
}
impl FVector4fCameraParameter {}
#[repr(C, align(16))]
pub struct FVector4dCameraParameter {
    pub value: crate::bindings::core_u_object::FVector4,
    __padding_end: [u8; 16],
}
impl FVector4dCameraParameter {}
#[repr(C, align(8))]
pub struct FRotator3fCameraParameter {
    pub value: crate::bindings::core_u_object::FRotator3f,
    __padding_end: [u8; 12],
}
impl FRotator3fCameraParameter {}
#[repr(C, align(8))]
pub struct FRotator3dCameraParameter {
    pub value: crate::bindings::core_u_object::FRotator,
    __padding_end: [u8; 16],
}
impl FRotator3dCameraParameter {}
#[repr(C, align(16))]
pub struct FTransform3fCameraParameter {
    pub value: crate::bindings::core_u_object::FTransform3f,
    __padding_end: [u8; 16],
}
impl FTransform3fCameraParameter {}
#[repr(C, align(16))]
pub struct FTransform3dCameraParameter {
    pub value: crate::bindings::core_u_object::FTransform,
    __padding_end: [u8; 16],
}
impl FTransform3dCameraParameter {}
#[repr(C, align(8))]
pub struct FCameraRigAssetReference {
    __padding_end: [u8; 304],
}
impl FCameraRigAssetReference {}
#[repr(C, align(8))]
pub struct FCameraShakeAssetReference {
    __padding_end: [u8; 64],
}
impl FCameraShakeAssetReference {}
#[repr(C, align(4))]
pub struct FCameraVariableSetterHandle {
    __padding_end: [u8; 8],
}
impl FCameraVariableSetterHandle {}
#[repr(C, align(8))]
pub struct FBlueprintCameraDirectorEvaluationParams {
    pub delta_time: f32,
    pub evaluation_context_owner: UPtr<crate::bindings::core_u_object::UObject>,
}
impl FBlueprintCameraDirectorEvaluationParams {}
#[repr(C, align(8))]
pub struct FBlueprintCameraDirectorActivateParams {
    pub evaluation_context_owner: UPtr<crate::bindings::core_u_object::UObject>,
}
impl FBlueprintCameraDirectorActivateParams {}
#[repr(C, align(8))]
pub struct FBlueprintCameraDirectorDeactivateParams {
    pub evaluation_context_owner: UPtr<crate::bindings::core_u_object::UObject>,
}
impl FBlueprintCameraDirectorDeactivateParams {}
#[repr(C, align(8))]
pub struct FCameraDirectorStateTreeEvaluationData {
    __padding_end: [u8; 32],
}
impl FCameraDirectorStateTreeEvaluationData {}
#[repr(C, align(8))]
pub struct FBlueprintCameraEvaluationDataRef {
    __padding_end: [u8; 24],
}
impl FBlueprintCameraEvaluationDataRef {}
#[repr(C, align(8))]
pub struct FBlueprintCameraPose {
    pub location: crate::bindings::core_u_object::FVector,
    pub rotation: crate::bindings::core_u_object::FRotator,
    pub target_distance: f64,
    pub field_of_view: f32,
    pub focal_length: f32,
    pub orthographic_width: f32,
    pub aperture: f32,
    pub shutter_speed: f32,
    pub focus_distance: f32,
    pub sensor_width: f32,
    pub sensor_height: f32,
    pub sensor_horizontal_offset: f32,
    pub sensor_vertical_offset: f32,
    pub iso: f32,
    pub squeeze_factor: f32,
    pub overscan: f32,
    pub diaphragm_blade_count: i32,
    pub near_clipping_plane: f32,
    pub far_clipping_plane: f32,
    #[doc(hidden)]
    __padding_124: [u8; 4],
    pub enable_physical_camera: bool,
    pub constrain_aspect_ratio: bool,
    #[doc(hidden)]
    __padding_127: [u8; 1],
    pub aspect_ratio_axis_constraint: crate::bindings::engine::EAspectRatioAxisConstraint,
    pub projection_mode: crate::bindings::engine::ECameraProjectionMode,
}
impl FBlueprintCameraPose {}
#[repr(C, align(8))]
pub struct FCameraActorAttachmentInfo {
    pub actor: UPtr<crate::bindings::engine::AActor>,
    pub socket_name: FName,
    pub bone_name: FName,
    pub weight: f32,
}
impl FCameraActorAttachmentInfo {}
#[repr(C, align(8))]
pub struct FCameraActorTargetInfo {
    pub actor: UPtr<crate::bindings::engine::AActor>,
    pub socket_name: FName,
    pub bone_name: FName,
    pub target_shape: ECameraTargetShape,
    pub target_size: f32,
    pub weight: f32,
}
impl FCameraActorTargetInfo {}
#[repr(C, align(8))]
pub struct FCameraFramingZone {
    pub left: f64,
    pub top: f64,
    pub right: f64,
    pub bottom: f64,
}
impl FCameraFramingZone {}
pub struct IHasCameraBuildStatus {}
#[repr(C, align(8))]
pub struct UHasCameraBuildStatus {
    __padding_end: [u8; 48],
}
impl UHasCameraBuildStatus {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UHasCameraBuildStatus")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UCameraRigInstanceFunctions {
    __padding_end: [u8; 48],
}
impl UCameraRigInstanceFunctions {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UCameraRigInstanceFunctions")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
    pub fn is_valid(instance_id: FCameraRigInstanceID) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<9>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_cameras::U_CAMERA_RIG_INSTANCE_FUNCTIONS_IS_VALID,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &instance_id,
                __buffer.add(0).cast::<FCameraRigInstanceID>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::gameplay_cameras::UCameraRigInstanceFunctions::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_cameras::U_CAMERA_RIG_INSTANCE_FUNCTIONS_IS_VALID,
                __buffer,
            )
        };
        unsafe { __buffer.add(8).cast::<bool>().read() }
    }
}
pub struct IAssetReferenceCameraNode {}
#[repr(C, align(8))]
pub struct UAssetReferenceCameraNode {
    __padding_end: [u8; 48],
}
impl UAssetReferenceCameraNode {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAssetReferenceCameraNode")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
pub struct IObjectTreeGraphObject {}
#[repr(C, align(8))]
pub struct UObjectTreeGraphObject {
    __padding_end: [u8; 48],
}
impl UObjectTreeGraphObject {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UObjectTreeGraphObject")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
pub struct IObjectTreeGraphRootObject {}
#[repr(C, align(8))]
pub struct UObjectTreeGraphRootObject {
    __padding_end: [u8; 48],
}
impl UObjectTreeGraphRootObject {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UObjectTreeGraphRootObject")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UBaseCameraObject {
    __padding_end: [u8; 176],
}
impl UBaseCameraObject {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBaseCameraObject")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UCameraNode {
    __padding_end: [u8; 104],
}
impl UCameraNode {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UCameraNode")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UBlendCameraNode {
    __padding_end: [u8; 104],
}
impl UBlendCameraNode {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBlendCameraNode")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UBlendStackCameraNode {
    __padding_end: [u8; 112],
}
impl UBlendStackCameraNode {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBlendStackCameraNode")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UBlendStackRootCameraNode {
    __padding_end: [u8; 120],
}
impl UBlendStackRootCameraNode {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBlendStackRootCameraNode")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UCameraAsset {
    __padding_end: [u8; 248],
}
impl UCameraAsset {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UCameraAsset")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UCameraDirector {
    __padding_end: [u8; 72],
}
impl UCameraDirector {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UCameraDirector")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UCameraObjectInterfaceParameterBase {
    __padding_end: [u8; 120],
}
impl UCameraObjectInterfaceParameterBase {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UCameraObjectInterfaceParameterBase")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UCameraObjectInterfaceBlendableParameter {
    __padding_end: [u8; 152],
}
impl UCameraObjectInterfaceBlendableParameter {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UCameraObjectInterfaceBlendableParameter")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UCameraObjectInterfaceDataParameter {
    __padding_end: [u8; 144],
}
impl UCameraObjectInterfaceDataParameter {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UCameraObjectInterfaceDataParameter")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UCameraRigAsset {
    __padding_end: [u8; 400],
}
impl UCameraRigAsset {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UCameraRigAsset")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UCombinedCameraRigsCameraNode {
    __padding_end: [u8; 120],
}
impl UCombinedCameraRigsCameraNode {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UCombinedCameraRigsCameraNode")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UCameraRigProxyAsset {
    __padding_end: [u8; 64],
}
impl UCameraRigProxyAsset {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UCameraRigProxyAsset")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UCameraRigProxyTable {
    __padding_end: [u8; 64],
}
impl UCameraRigProxyTable {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UCameraRigProxyTable")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UCameraRigTransitionCondition {
    __padding_end: [u8; 88],
}
impl UCameraRigTransitionCondition {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UCameraRigTransitionCondition")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UCameraRigTransition {
    __padding_end: [u8; 120],
}
impl UCameraRigTransition {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UCameraRigTransition")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UCameraShakeAsset {
    __padding_end: [u8; 288],
}
impl UCameraShakeAsset {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UCameraShakeAsset")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UCameraValueInterpolator {
    __padding_end: [u8; 48],
}
impl UCameraValueInterpolator {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UCameraValueInterpolator")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UPopValueInterpolator {
    __padding_end: [u8; 48],
}
impl UPopValueInterpolator {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPopValueInterpolator")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UCameraVariableAsset {
    __padding_end: [u8; 88],
}
impl UCameraVariableAsset {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UCameraVariableAsset")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UBooleanCameraVariable {
    __padding_end: [u8; 96],
}
impl UBooleanCameraVariable {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBooleanCameraVariable")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UInteger32CameraVariable {
    __padding_end: [u8; 96],
}
impl UInteger32CameraVariable {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UInteger32CameraVariable")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UFloatCameraVariable {
    __padding_end: [u8; 96],
}
impl UFloatCameraVariable {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UFloatCameraVariable")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UDoubleCameraVariable {
    __padding_end: [u8; 96],
}
impl UDoubleCameraVariable {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDoubleCameraVariable")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UVector2fCameraVariable {
    __padding_end: [u8; 96],
}
impl UVector2fCameraVariable {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UVector2fCameraVariable")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UVector2dCameraVariable {
    __padding_end: [u8; 104],
}
impl UVector2dCameraVariable {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UVector2dCameraVariable")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UVector3fCameraVariable {
    __padding_end: [u8; 104],
}
impl UVector3fCameraVariable {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UVector3fCameraVariable")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UVector3dCameraVariable {
    __padding_end: [u8; 112],
}
impl UVector3dCameraVariable {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UVector3dCameraVariable")
            .unwrap()
    }
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
pub struct UVector4fCameraVariable {
    __padding_end: [u8; 112],
}
impl UVector4fCameraVariable {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UVector4fCameraVariable")
            .unwrap()
    }
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
pub struct UVector4dCameraVariable {
    __padding_end: [u8; 128],
}
impl UVector4dCameraVariable {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UVector4dCameraVariable")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct URotator3fCameraVariable {
    __padding_end: [u8; 104],
}
impl URotator3fCameraVariable {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("URotator3fCameraVariable")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct URotator3dCameraVariable {
    __padding_end: [u8; 112],
}
impl URotator3dCameraVariable {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("URotator3dCameraVariable")
            .unwrap()
    }
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
pub struct UTransform3fCameraVariable {
    __padding_end: [u8; 144],
}
impl UTransform3fCameraVariable {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UTransform3fCameraVariable")
            .unwrap()
    }
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
pub struct UTransform3dCameraVariable {
    __padding_end: [u8; 192],
}
impl UTransform3dCameraVariable {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UTransform3dCameraVariable")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UCameraVariableCollection {
    __padding_end: [u8; 64],
}
impl UCameraVariableCollection {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UCameraVariableCollection")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct URootCameraNode {
    __padding_end: [u8; 104],
}
impl URootCameraNode {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("URootCameraNode")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UDefaultRootCameraNode {
    __padding_end: [u8; 136],
}
impl UDefaultRootCameraNode {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDefaultRootCameraNode")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
pub struct ICustomCameraNodeParameterProvider {}
#[repr(C, align(8))]
pub struct UCustomCameraNodeParameterProvider {
    __padding_end: [u8; 48],
}
impl UCustomCameraNodeParameterProvider {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UCustomCameraNodeParameterProvider")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UObjectTreeGraphComment {
    __padding_end: [u8; 104],
}
impl UObjectTreeGraphComment {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UObjectTreeGraphComment")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UShakeCameraNode {
    __padding_end: [u8; 104],
}
impl UShakeCameraNode {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UShakeCameraNode")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UBlueprintCameraDirectorEvaluator {
    __padding_end: [u8; 336],
}
impl UBlueprintCameraDirectorEvaluator {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBlueprintCameraDirectorEvaluator")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
    pub fn run_child_camera_director(
        &mut self,
        delta_time: f32,
        child_slot_name: FName,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<17>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_cameras::U_BLUEPRINT_CAMERA_DIRECTOR_EVALUATOR_RUN_CHILD_CAMERA_DIRECTOR,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&delta_time, __buffer.add(0).cast::<f32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &child_slot_name,
                __buffer.add(4).cast::<FName>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_cameras::U_BLUEPRINT_CAMERA_DIRECTOR_EVALUATOR_RUN_CHILD_CAMERA_DIRECTOR,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<bool>().read() }
    }
    pub fn run_camera_director(
        &mut self,
        delta_time: f32,
        evaluation_context_owner: UPtr<crate::bindings::core_u_object::UObject>,
        params: &FBlueprintCameraDirectorEvaluationParams,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_cameras::U_BLUEPRINT_CAMERA_DIRECTOR_EVALUATOR_RUN_CAMERA_DIRECTOR,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&delta_time, __buffer.add(0).cast::<f32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &evaluation_context_owner,
                __buffer.add(8).cast::<UPtr<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                params,
                __buffer.add(16).cast::<FBlueprintCameraDirectorEvaluationParams>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_cameras::U_BLUEPRINT_CAMERA_DIRECTOR_EVALUATOR_RUN_CAMERA_DIRECTOR,
                __buffer,
            )
        };
    }
    pub fn resolve_camera_rig_proxy(
        &self,
        camera_rig_proxy: UPtr<UCameraRigProxyAsset>,
    ) -> UPtr<UCameraRigAsset> {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_cameras::U_BLUEPRINT_CAMERA_DIRECTOR_EVALUATOR_RESOLVE_CAMERA_RIG_PROXY,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &camera_rig_proxy,
                __buffer.add(0).cast::<UPtr<UCameraRigProxyAsset>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_cameras::U_BLUEPRINT_CAMERA_DIRECTOR_EVALUATOR_RESOLVE_CAMERA_RIG_PROXY,
                __buffer,
            )
        };
        unsafe { __buffer.add(8).cast::<UPtr<UCameraRigAsset>>().read() }
    }
    pub fn remove_child_evaluation_context(
        &mut self,
        child_evaluation_context_owner: UPtr<crate::bindings::core_u_object::UObject>,
        child_slot_name: FName,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<21>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_cameras::U_BLUEPRINT_CAMERA_DIRECTOR_EVALUATOR_REMOVE_CHILD_EVALUATION_CONTEXT,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &child_evaluation_context_owner,
                __buffer.add(0).cast::<UPtr<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &child_slot_name,
                __buffer.add(8).cast::<FName>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_cameras::U_BLUEPRINT_CAMERA_DIRECTOR_EVALUATOR_REMOVE_CHILD_EVALUATION_CONTEXT,
                __buffer,
            )
        };
        unsafe { __buffer.add(20).cast::<bool>().read() }
    }
    pub fn get_initial_context_result(&self) -> FBlueprintCameraEvaluationDataRef {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_cameras::U_BLUEPRINT_CAMERA_DIRECTOR_EVALUATOR_GET_INITIAL_CONTEXT_RESULT,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_cameras::U_BLUEPRINT_CAMERA_DIRECTOR_EVALUATOR_GET_INITIAL_CONTEXT_RESULT,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<FBlueprintCameraEvaluationDataRef>().read() }
    }
    pub fn get_conditional_context_result(
        &self,
        condition: ECameraEvaluationDataCondition,
    ) -> FBlueprintCameraEvaluationDataRef {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_cameras::U_BLUEPRINT_CAMERA_DIRECTOR_EVALUATOR_GET_CONDITIONAL_CONTEXT_RESULT,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &condition,
                __buffer.add(0).cast::<ECameraEvaluationDataCondition>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_cameras::U_BLUEPRINT_CAMERA_DIRECTOR_EVALUATOR_GET_CONDITIONAL_CONTEXT_RESULT,
                __buffer,
            )
        };
        unsafe { __buffer.add(8).cast::<FBlueprintCameraEvaluationDataRef>().read() }
    }
    pub fn find_evaluation_context_owner_actor(
        &self,
        actor_class: TSubclassOf<crate::bindings::engine::AActor>,
    ) -> UPtr<crate::bindings::engine::AActor> {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_cameras::U_BLUEPRINT_CAMERA_DIRECTOR_EVALUATOR_FIND_EVALUATION_CONTEXT_OWNER_ACTOR,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &actor_class,
                __buffer.add(0).cast::<TSubclassOf<crate::bindings::engine::AActor>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_cameras::U_BLUEPRINT_CAMERA_DIRECTOR_EVALUATOR_FIND_EVALUATION_CONTEXT_OWNER_ACTOR,
                __buffer,
            )
        };
        unsafe { __buffer.add(8).cast::<UPtr<crate::bindings::engine::AActor>>().read() }
    }
    pub fn deactivate_persistent_visual_camera_rig(
        &mut self,
        camera_rig_prefab: UPtr<UCameraRigAsset>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_cameras::U_BLUEPRINT_CAMERA_DIRECTOR_EVALUATOR_DEACTIVATE_PERSISTENT_VISUAL_CAMERA_RIG,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &camera_rig_prefab,
                __buffer.add(0).cast::<UPtr<UCameraRigAsset>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_cameras::U_BLUEPRINT_CAMERA_DIRECTOR_EVALUATOR_DEACTIVATE_PERSISTENT_VISUAL_CAMERA_RIG,
                __buffer,
            )
        };
    }
    pub fn deactivate_persistent_global_camera_rig(
        &mut self,
        camera_rig_prefab: UPtr<UCameraRigAsset>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_cameras::U_BLUEPRINT_CAMERA_DIRECTOR_EVALUATOR_DEACTIVATE_PERSISTENT_GLOBAL_CAMERA_RIG,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &camera_rig_prefab,
                __buffer.add(0).cast::<UPtr<UCameraRigAsset>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_cameras::U_BLUEPRINT_CAMERA_DIRECTOR_EVALUATOR_DEACTIVATE_PERSISTENT_GLOBAL_CAMERA_RIG,
                __buffer,
            )
        };
    }
    pub fn deactivate_persistent_base_camera_rig(
        &mut self,
        camera_rig_prefab: UPtr<UCameraRigAsset>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_cameras::U_BLUEPRINT_CAMERA_DIRECTOR_EVALUATOR_DEACTIVATE_PERSISTENT_BASE_CAMERA_RIG,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &camera_rig_prefab,
                __buffer.add(0).cast::<UPtr<UCameraRigAsset>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_cameras::U_BLUEPRINT_CAMERA_DIRECTOR_EVALUATOR_DEACTIVATE_PERSISTENT_BASE_CAMERA_RIG,
                __buffer,
            )
        };
    }
    pub fn deactivate_camera_director(
        &mut self,
        evaluation_context_owner: UPtr<crate::bindings::core_u_object::UObject>,
        params: &FBlueprintCameraDirectorDeactivateParams,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_cameras::U_BLUEPRINT_CAMERA_DIRECTOR_EVALUATOR_DEACTIVATE_CAMERA_DIRECTOR,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &evaluation_context_owner,
                __buffer.add(0).cast::<UPtr<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                params,
                __buffer.add(8).cast::<FBlueprintCameraDirectorDeactivateParams>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_cameras::U_BLUEPRINT_CAMERA_DIRECTOR_EVALUATOR_DEACTIVATE_CAMERA_DIRECTOR,
                __buffer,
            )
        };
    }
    pub fn add_child_evaluation_context(
        &mut self,
        child_evaluation_context_owner: UPtr<crate::bindings::core_u_object::UObject>,
    ) -> FName {
        let mut __stack = crate::core_data::StackAlloc::<20>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_cameras::U_BLUEPRINT_CAMERA_DIRECTOR_EVALUATOR_ADD_CHILD_EVALUATION_CONTEXT,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &child_evaluation_context_owner,
                __buffer.add(0).cast::<UPtr<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_cameras::U_BLUEPRINT_CAMERA_DIRECTOR_EVALUATOR_ADD_CHILD_EVALUATION_CONTEXT,
                __buffer,
            )
        };
        unsafe { __buffer.add(8).cast::<FName>().read() }
    }
    pub fn activate_persistent_visual_camera_rig(
        &mut self,
        camera_rig_prefab: UPtr<UCameraRigAsset>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_cameras::U_BLUEPRINT_CAMERA_DIRECTOR_EVALUATOR_ACTIVATE_PERSISTENT_VISUAL_CAMERA_RIG,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &camera_rig_prefab,
                __buffer.add(0).cast::<UPtr<UCameraRigAsset>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_cameras::U_BLUEPRINT_CAMERA_DIRECTOR_EVALUATOR_ACTIVATE_PERSISTENT_VISUAL_CAMERA_RIG,
                __buffer,
            )
        };
    }
    pub fn activate_persistent_global_camera_rig(
        &mut self,
        camera_rig_prefab: UPtr<UCameraRigAsset>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_cameras::U_BLUEPRINT_CAMERA_DIRECTOR_EVALUATOR_ACTIVATE_PERSISTENT_GLOBAL_CAMERA_RIG,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &camera_rig_prefab,
                __buffer.add(0).cast::<UPtr<UCameraRigAsset>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_cameras::U_BLUEPRINT_CAMERA_DIRECTOR_EVALUATOR_ACTIVATE_PERSISTENT_GLOBAL_CAMERA_RIG,
                __buffer,
            )
        };
    }
    pub fn activate_persistent_base_camera_rig(
        &mut self,
        camera_rig_prefab: UPtr<UCameraRigAsset>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_cameras::U_BLUEPRINT_CAMERA_DIRECTOR_EVALUATOR_ACTIVATE_PERSISTENT_BASE_CAMERA_RIG,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &camera_rig_prefab,
                __buffer.add(0).cast::<UPtr<UCameraRigAsset>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_cameras::U_BLUEPRINT_CAMERA_DIRECTOR_EVALUATOR_ACTIVATE_PERSISTENT_BASE_CAMERA_RIG,
                __buffer,
            )
        };
    }
    pub fn activate_camera_rig_via_proxy(
        &mut self,
        camera_rig_proxy: UPtr<UCameraRigProxyAsset>,
        b_force_new_instance: bool,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<9>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_cameras::U_BLUEPRINT_CAMERA_DIRECTOR_EVALUATOR_ACTIVATE_CAMERA_RIG_VIA_PROXY,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &camera_rig_proxy,
                __buffer.add(0).cast::<UPtr<UCameraRigProxyAsset>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_force_new_instance,
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
                crate::bindings::gameplay_cameras::U_BLUEPRINT_CAMERA_DIRECTOR_EVALUATOR_ACTIVATE_CAMERA_RIG_VIA_PROXY,
                __buffer,
            )
        };
    }
    pub fn activate_camera_rig(
        &mut self,
        camera_rig: UPtr<UCameraRigAsset>,
        b_force_new_instance: bool,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<9>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_cameras::U_BLUEPRINT_CAMERA_DIRECTOR_EVALUATOR_ACTIVATE_CAMERA_RIG,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &camera_rig,
                __buffer.add(0).cast::<UPtr<UCameraRigAsset>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_force_new_instance,
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
                crate::bindings::gameplay_cameras::U_BLUEPRINT_CAMERA_DIRECTOR_EVALUATOR_ACTIVATE_CAMERA_RIG,
                __buffer,
            )
        };
    }
    pub fn activate_camera_director(
        &mut self,
        evaluation_context_owner: UPtr<crate::bindings::core_u_object::UObject>,
        params: &FBlueprintCameraDirectorActivateParams,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_cameras::U_BLUEPRINT_CAMERA_DIRECTOR_EVALUATOR_ACTIVATE_CAMERA_DIRECTOR,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &evaluation_context_owner,
                __buffer.add(0).cast::<UPtr<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                params,
                __buffer.add(8).cast::<FBlueprintCameraDirectorActivateParams>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_cameras::U_BLUEPRINT_CAMERA_DIRECTOR_EVALUATOR_ACTIVATE_CAMERA_DIRECTOR,
                __buffer,
            )
        };
    }
}
#[repr(C, align(8))]
pub struct UBlueprintCameraDirector {
    __padding_end: [u8; 80],
}
impl UBlueprintCameraDirector {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBlueprintCameraDirector")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UCameraDirectorStateTreeSchema {
    __padding_end: [u8; 64],
}
impl UCameraDirectorStateTreeSchema {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UCameraDirectorStateTreeSchema")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UPriorityQueueCameraDirector {
    __padding_end: [u8; 72],
}
impl UPriorityQueueCameraDirector {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPriorityQueueCameraDirector")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct USingleCameraDirector {
    __padding_end: [u8; 80],
}
impl USingleCameraDirector {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("USingleCameraDirector")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UStateTreeCameraDirector {
    __padding_end: [u8; 112],
}
impl UStateTreeCameraDirector {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UStateTreeCameraDirector")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UActivateCameraRigFunctions {
    __padding_end: [u8; 48],
}
impl UActivateCameraRigFunctions {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UActivateCameraRigFunctions")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
    pub fn activate_persistent_visual_camera_rig(
        world_context_object: UPtr<crate::bindings::core_u_object::UObject>,
        player_controller: UPtr<crate::bindings::engine::APlayerController>,
        camera_rig: UPtr<UCameraRigAsset>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_cameras::U_ACTIVATE_CAMERA_RIG_FUNCTIONS_ACTIVATE_PERSISTENT_VISUAL_CAMERA_RIG,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &world_context_object,
                __buffer.add(0).cast::<UPtr<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &player_controller,
                __buffer
                    .add(8)
                    .cast::<UPtr<crate::bindings::engine::APlayerController>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &camera_rig,
                __buffer.add(16).cast::<UPtr<UCameraRigAsset>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::gameplay_cameras::UActivateCameraRigFunctions::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_cameras::U_ACTIVATE_CAMERA_RIG_FUNCTIONS_ACTIVATE_PERSISTENT_VISUAL_CAMERA_RIG,
                __buffer,
            )
        };
    }
    pub fn activate_persistent_global_camera_rig(
        world_context_object: UPtr<crate::bindings::core_u_object::UObject>,
        player_controller: UPtr<crate::bindings::engine::APlayerController>,
        camera_rig: UPtr<UCameraRigAsset>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_cameras::U_ACTIVATE_CAMERA_RIG_FUNCTIONS_ACTIVATE_PERSISTENT_GLOBAL_CAMERA_RIG,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &world_context_object,
                __buffer.add(0).cast::<UPtr<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &player_controller,
                __buffer
                    .add(8)
                    .cast::<UPtr<crate::bindings::engine::APlayerController>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &camera_rig,
                __buffer.add(16).cast::<UPtr<UCameraRigAsset>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::gameplay_cameras::UActivateCameraRigFunctions::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_cameras::U_ACTIVATE_CAMERA_RIG_FUNCTIONS_ACTIVATE_PERSISTENT_GLOBAL_CAMERA_RIG,
                __buffer,
            )
        };
    }
    pub fn activate_persistent_base_camera_rig(
        world_context_object: UPtr<crate::bindings::core_u_object::UObject>,
        player_controller: UPtr<crate::bindings::engine::APlayerController>,
        camera_rig: UPtr<UCameraRigAsset>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_cameras::U_ACTIVATE_CAMERA_RIG_FUNCTIONS_ACTIVATE_PERSISTENT_BASE_CAMERA_RIG,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &world_context_object,
                __buffer.add(0).cast::<UPtr<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &player_controller,
                __buffer
                    .add(8)
                    .cast::<UPtr<crate::bindings::engine::APlayerController>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &camera_rig,
                __buffer.add(16).cast::<UPtr<UCameraRigAsset>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::gameplay_cameras::UActivateCameraRigFunctions::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_cameras::U_ACTIVATE_CAMERA_RIG_FUNCTIONS_ACTIVATE_PERSISTENT_BASE_CAMERA_RIG,
                __buffer,
            )
        };
    }
}
#[repr(C, align(8))]
pub struct UCameraComponentCameraNode {
    __padding_end: [u8; 104],
}
impl UCameraComponentCameraNode {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UCameraComponentCameraNode")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UCalcCameraActorCameraNode {
    __padding_end: [u8; 104],
}
impl UCalcCameraActorCameraNode {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UCalcCameraActorCameraNode")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UBlueprintCameraEvaluationDataFunctionLibrary {
    __padding_end: [u8; 48],
}
impl UBlueprintCameraEvaluationDataFunctionLibrary {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBlueprintCameraEvaluationDataFunctionLibrary")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
    pub fn set_default_camera_rig_parameters(
        camera_data: &FBlueprintCameraEvaluationDataRef,
        camera_rig: UPtr<UCameraRigAsset>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_cameras::U_BLUEPRINT_CAMERA_EVALUATION_DATA_FUNCTION_LIBRARY_SET_DEFAULT_CAMERA_RIG_PARAMETERS,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                camera_data,
                __buffer.add(0).cast::<FBlueprintCameraEvaluationDataRef>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &camera_rig,
                __buffer.add(24).cast::<UPtr<UCameraRigAsset>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::gameplay_cameras::UBlueprintCameraEvaluationDataFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_cameras::U_BLUEPRINT_CAMERA_EVALUATION_DATA_FUNCTION_LIBRARY_SET_DEFAULT_CAMERA_RIG_PARAMETERS,
                __buffer,
            )
        };
    }
    pub fn set_camera_pose(
        camera_data: &FBlueprintCameraEvaluationDataRef,
        camera_pose: &FBlueprintCameraPose,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<160>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_cameras::U_BLUEPRINT_CAMERA_EVALUATION_DATA_FUNCTION_LIBRARY_SET_CAMERA_POSE,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                camera_data,
                __buffer.add(0).cast::<FBlueprintCameraEvaluationDataRef>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                camera_pose,
                __buffer.add(24).cast::<FBlueprintCameraPose>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::gameplay_cameras::UBlueprintCameraEvaluationDataFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_cameras::U_BLUEPRINT_CAMERA_EVALUATION_DATA_FUNCTION_LIBRARY_SET_CAMERA_POSE,
                __buffer,
            )
        };
    }
    pub fn make_camera_evaluation_data() -> FBlueprintCameraEvaluationDataRef {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_cameras::U_BLUEPRINT_CAMERA_EVALUATION_DATA_FUNCTION_LIBRARY_MAKE_CAMERA_EVALUATION_DATA,
                __buffer,
            )
        };
        let __object_ptr = crate::bindings::gameplay_cameras::UBlueprintCameraEvaluationDataFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_cameras::U_BLUEPRINT_CAMERA_EVALUATION_DATA_FUNCTION_LIBRARY_MAKE_CAMERA_EVALUATION_DATA,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<FBlueprintCameraEvaluationDataRef>().read() }
    }
    pub fn get_camera_pose(
        camera_data: &FBlueprintCameraEvaluationDataRef,
    ) -> FBlueprintCameraPose {
        let mut __stack = crate::core_data::StackAlloc::<160>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_cameras::U_BLUEPRINT_CAMERA_EVALUATION_DATA_FUNCTION_LIBRARY_GET_CAMERA_POSE,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                camera_data,
                __buffer.add(0).cast::<FBlueprintCameraEvaluationDataRef>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::gameplay_cameras::UBlueprintCameraEvaluationDataFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_cameras::U_BLUEPRINT_CAMERA_EVALUATION_DATA_FUNCTION_LIBRARY_GET_CAMERA_POSE,
                __buffer,
            )
        };
        unsafe { __buffer.add(24).cast::<FBlueprintCameraPose>().read() }
    }
    pub fn blend_camera_evaluation_data(
        from_camera_data: &FBlueprintCameraEvaluationDataRef,
        to_camera_data: &FBlueprintCameraEvaluationDataRef,
        factor: f32,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<52>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_cameras::U_BLUEPRINT_CAMERA_EVALUATION_DATA_FUNCTION_LIBRARY_BLEND_CAMERA_EVALUATION_DATA,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                from_camera_data,
                __buffer.add(0).cast::<FBlueprintCameraEvaluationDataRef>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                to_camera_data,
                __buffer.add(24).cast::<FBlueprintCameraEvaluationDataRef>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&factor, __buffer.add(48).cast::<f32>(), 1);
        }
        let __object_ptr = crate::bindings::gameplay_cameras::UBlueprintCameraEvaluationDataFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_cameras::U_BLUEPRINT_CAMERA_EVALUATION_DATA_FUNCTION_LIBRARY_BLEND_CAMERA_EVALUATION_DATA,
                __buffer,
            )
        };
    }
}
#[repr(C, align(8))]
pub struct UBlueprintCameraVariableTableFunctionLibrary {
    __padding_end: [u8; 48],
}
impl UBlueprintCameraVariableTableFunctionLibrary {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBlueprintCameraVariableTableFunctionLibrary")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
    pub fn set_vector4_camera_variable(
        camera_data: &FBlueprintCameraEvaluationDataRef,
        variable: UPtr<UVector4dCameraVariable>,
        value: &crate::bindings::core_u_object::FVector4,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<64>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_cameras::U_BLUEPRINT_CAMERA_VARIABLE_TABLE_FUNCTION_LIBRARY_SET_VECTOR4_CAMERA_VARIABLE,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                camera_data,
                __buffer.add(0).cast::<FBlueprintCameraEvaluationDataRef>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &variable,
                __buffer.add(24).cast::<UPtr<UVector4dCameraVariable>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                value,
                __buffer.add(32).cast::<crate::bindings::core_u_object::FVector4>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::gameplay_cameras::UBlueprintCameraVariableTableFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_cameras::U_BLUEPRINT_CAMERA_VARIABLE_TABLE_FUNCTION_LIBRARY_SET_VECTOR4_CAMERA_VARIABLE,
                __buffer,
            )
        };
    }
    pub fn set_vector3_camera_variable(
        camera_data: &FBlueprintCameraEvaluationDataRef,
        variable: UPtr<UVector3dCameraVariable>,
        value: &crate::bindings::core_u_object::FVector,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<56>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_cameras::U_BLUEPRINT_CAMERA_VARIABLE_TABLE_FUNCTION_LIBRARY_SET_VECTOR3_CAMERA_VARIABLE,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                camera_data,
                __buffer.add(0).cast::<FBlueprintCameraEvaluationDataRef>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &variable,
                __buffer.add(24).cast::<UPtr<UVector3dCameraVariable>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                value,
                __buffer.add(32).cast::<crate::bindings::core_u_object::FVector>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::gameplay_cameras::UBlueprintCameraVariableTableFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_cameras::U_BLUEPRINT_CAMERA_VARIABLE_TABLE_FUNCTION_LIBRARY_SET_VECTOR3_CAMERA_VARIABLE,
                __buffer,
            )
        };
    }
    pub fn set_vector2_camera_variable(
        camera_data: &FBlueprintCameraEvaluationDataRef,
        variable: UPtr<UVector2dCameraVariable>,
        value: &crate::bindings::core_u_object::FVector2D,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<48>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_cameras::U_BLUEPRINT_CAMERA_VARIABLE_TABLE_FUNCTION_LIBRARY_SET_VECTOR2_CAMERA_VARIABLE,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                camera_data,
                __buffer.add(0).cast::<FBlueprintCameraEvaluationDataRef>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &variable,
                __buffer.add(24).cast::<UPtr<UVector2dCameraVariable>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                value,
                __buffer.add(32).cast::<crate::bindings::core_u_object::FVector2D>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::gameplay_cameras::UBlueprintCameraVariableTableFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_cameras::U_BLUEPRINT_CAMERA_VARIABLE_TABLE_FUNCTION_LIBRARY_SET_VECTOR2_CAMERA_VARIABLE,
                __buffer,
            )
        };
    }
    pub fn set_transform_camera_variable(
        camera_data: &FBlueprintCameraEvaluationDataRef,
        variable: UPtr<UTransform3dCameraVariable>,
        value: &crate::bindings::core_u_object::FTransform,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<128>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_cameras::U_BLUEPRINT_CAMERA_VARIABLE_TABLE_FUNCTION_LIBRARY_SET_TRANSFORM_CAMERA_VARIABLE,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                camera_data,
                __buffer.add(0).cast::<FBlueprintCameraEvaluationDataRef>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &variable,
                __buffer.add(24).cast::<UPtr<UTransform3dCameraVariable>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                value,
                __buffer.add(32).cast::<crate::bindings::core_u_object::FTransform>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::gameplay_cameras::UBlueprintCameraVariableTableFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_cameras::U_BLUEPRINT_CAMERA_VARIABLE_TABLE_FUNCTION_LIBRARY_SET_TRANSFORM_CAMERA_VARIABLE,
                __buffer,
            )
        };
    }
    pub fn set_rotator_camera_variable(
        camera_data: &FBlueprintCameraEvaluationDataRef,
        variable: UPtr<URotator3dCameraVariable>,
        value: &crate::bindings::core_u_object::FRotator,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<56>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_cameras::U_BLUEPRINT_CAMERA_VARIABLE_TABLE_FUNCTION_LIBRARY_SET_ROTATOR_CAMERA_VARIABLE,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                camera_data,
                __buffer.add(0).cast::<FBlueprintCameraEvaluationDataRef>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &variable,
                __buffer.add(24).cast::<UPtr<URotator3dCameraVariable>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                value,
                __buffer.add(32).cast::<crate::bindings::core_u_object::FRotator>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::gameplay_cameras::UBlueprintCameraVariableTableFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_cameras::U_BLUEPRINT_CAMERA_VARIABLE_TABLE_FUNCTION_LIBRARY_SET_ROTATOR_CAMERA_VARIABLE,
                __buffer,
            )
        };
    }
    pub fn set_integer32_camera_variable(
        camera_data: &FBlueprintCameraEvaluationDataRef,
        variable: UPtr<UInteger32CameraVariable>,
        value: i32,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<36>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_cameras::U_BLUEPRINT_CAMERA_VARIABLE_TABLE_FUNCTION_LIBRARY_SET_INTEGER32_CAMERA_VARIABLE,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                camera_data,
                __buffer.add(0).cast::<FBlueprintCameraEvaluationDataRef>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &variable,
                __buffer.add(24).cast::<UPtr<UInteger32CameraVariable>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&value, __buffer.add(32).cast::<i32>(), 1);
        }
        let __object_ptr = crate::bindings::gameplay_cameras::UBlueprintCameraVariableTableFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_cameras::U_BLUEPRINT_CAMERA_VARIABLE_TABLE_FUNCTION_LIBRARY_SET_INTEGER32_CAMERA_VARIABLE,
                __buffer,
            )
        };
    }
    pub fn set_float_camera_variable(
        camera_data: &FBlueprintCameraEvaluationDataRef,
        variable: UPtr<UFloatCameraVariable>,
        value: f32,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<36>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_cameras::U_BLUEPRINT_CAMERA_VARIABLE_TABLE_FUNCTION_LIBRARY_SET_FLOAT_CAMERA_VARIABLE,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                camera_data,
                __buffer.add(0).cast::<FBlueprintCameraEvaluationDataRef>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &variable,
                __buffer.add(24).cast::<UPtr<UFloatCameraVariable>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&value, __buffer.add(32).cast::<f32>(), 1);
        }
        let __object_ptr = crate::bindings::gameplay_cameras::UBlueprintCameraVariableTableFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_cameras::U_BLUEPRINT_CAMERA_VARIABLE_TABLE_FUNCTION_LIBRARY_SET_FLOAT_CAMERA_VARIABLE,
                __buffer,
            )
        };
    }
    pub fn set_double_camera_variable(
        camera_data: &FBlueprintCameraEvaluationDataRef,
        variable: UPtr<UDoubleCameraVariable>,
        value: f64,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<40>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_cameras::U_BLUEPRINT_CAMERA_VARIABLE_TABLE_FUNCTION_LIBRARY_SET_DOUBLE_CAMERA_VARIABLE,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                camera_data,
                __buffer.add(0).cast::<FBlueprintCameraEvaluationDataRef>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &variable,
                __buffer.add(24).cast::<UPtr<UDoubleCameraVariable>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&value, __buffer.add(32).cast::<f64>(), 1);
        }
        let __object_ptr = crate::bindings::gameplay_cameras::UBlueprintCameraVariableTableFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_cameras::U_BLUEPRINT_CAMERA_VARIABLE_TABLE_FUNCTION_LIBRARY_SET_DOUBLE_CAMERA_VARIABLE,
                __buffer,
            )
        };
    }
    pub fn set_boolean_camera_variable(
        camera_data: &FBlueprintCameraEvaluationDataRef,
        variable: UPtr<UBooleanCameraVariable>,
        value: bool,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<33>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_cameras::U_BLUEPRINT_CAMERA_VARIABLE_TABLE_FUNCTION_LIBRARY_SET_BOOLEAN_CAMERA_VARIABLE,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                camera_data,
                __buffer.add(0).cast::<FBlueprintCameraEvaluationDataRef>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &variable,
                __buffer.add(24).cast::<UPtr<UBooleanCameraVariable>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&value, __buffer.add(32).cast::<bool>(), 1);
        }
        let __object_ptr = crate::bindings::gameplay_cameras::UBlueprintCameraVariableTableFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_cameras::U_BLUEPRINT_CAMERA_VARIABLE_TABLE_FUNCTION_LIBRARY_SET_BOOLEAN_CAMERA_VARIABLE,
                __buffer,
            )
        };
    }
    pub fn get_vector4_camera_variable(
        camera_data: &FBlueprintCameraEvaluationDataRef,
        variable: UPtr<UVector4dCameraVariable>,
    ) -> crate::bindings::core_u_object::FVector4 {
        let mut __stack = crate::core_data::StackAlloc::<64>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_cameras::U_BLUEPRINT_CAMERA_VARIABLE_TABLE_FUNCTION_LIBRARY_GET_VECTOR4_CAMERA_VARIABLE,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                camera_data,
                __buffer.add(0).cast::<FBlueprintCameraEvaluationDataRef>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &variable,
                __buffer.add(24).cast::<UPtr<UVector4dCameraVariable>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::gameplay_cameras::UBlueprintCameraVariableTableFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_cameras::U_BLUEPRINT_CAMERA_VARIABLE_TABLE_FUNCTION_LIBRARY_GET_VECTOR4_CAMERA_VARIABLE,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(32).cast::<crate::bindings::core_u_object::FVector4>().read()
        }
    }
    pub fn get_vector3_camera_variable(
        camera_data: &FBlueprintCameraEvaluationDataRef,
        variable: UPtr<UVector3dCameraVariable>,
    ) -> crate::bindings::core_u_object::FVector {
        let mut __stack = crate::core_data::StackAlloc::<56>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_cameras::U_BLUEPRINT_CAMERA_VARIABLE_TABLE_FUNCTION_LIBRARY_GET_VECTOR3_CAMERA_VARIABLE,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                camera_data,
                __buffer.add(0).cast::<FBlueprintCameraEvaluationDataRef>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &variable,
                __buffer.add(24).cast::<UPtr<UVector3dCameraVariable>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::gameplay_cameras::UBlueprintCameraVariableTableFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_cameras::U_BLUEPRINT_CAMERA_VARIABLE_TABLE_FUNCTION_LIBRARY_GET_VECTOR3_CAMERA_VARIABLE,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(32).cast::<crate::bindings::core_u_object::FVector>().read()
        }
    }
    pub fn get_vector2_camera_variable(
        camera_data: &FBlueprintCameraEvaluationDataRef,
        variable: UPtr<UVector2dCameraVariable>,
    ) -> crate::bindings::core_u_object::FVector2D {
        let mut __stack = crate::core_data::StackAlloc::<48>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_cameras::U_BLUEPRINT_CAMERA_VARIABLE_TABLE_FUNCTION_LIBRARY_GET_VECTOR2_CAMERA_VARIABLE,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                camera_data,
                __buffer.add(0).cast::<FBlueprintCameraEvaluationDataRef>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &variable,
                __buffer.add(24).cast::<UPtr<UVector2dCameraVariable>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::gameplay_cameras::UBlueprintCameraVariableTableFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_cameras::U_BLUEPRINT_CAMERA_VARIABLE_TABLE_FUNCTION_LIBRARY_GET_VECTOR2_CAMERA_VARIABLE,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(32).cast::<crate::bindings::core_u_object::FVector2D>().read()
        }
    }
    pub fn get_transform_camera_variable(
        camera_data: &FBlueprintCameraEvaluationDataRef,
        variable: UPtr<UTransform3dCameraVariable>,
    ) -> crate::bindings::core_u_object::FTransform {
        let mut __stack = crate::core_data::StackAlloc::<128>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_cameras::U_BLUEPRINT_CAMERA_VARIABLE_TABLE_FUNCTION_LIBRARY_GET_TRANSFORM_CAMERA_VARIABLE,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                camera_data,
                __buffer.add(0).cast::<FBlueprintCameraEvaluationDataRef>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &variable,
                __buffer.add(24).cast::<UPtr<UTransform3dCameraVariable>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::gameplay_cameras::UBlueprintCameraVariableTableFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_cameras::U_BLUEPRINT_CAMERA_VARIABLE_TABLE_FUNCTION_LIBRARY_GET_TRANSFORM_CAMERA_VARIABLE,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(32).cast::<crate::bindings::core_u_object::FTransform>().read()
        }
    }
    pub fn get_rotator_camera_variable(
        camera_data: &FBlueprintCameraEvaluationDataRef,
        variable: UPtr<URotator3dCameraVariable>,
    ) -> crate::bindings::core_u_object::FRotator {
        let mut __stack = crate::core_data::StackAlloc::<56>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_cameras::U_BLUEPRINT_CAMERA_VARIABLE_TABLE_FUNCTION_LIBRARY_GET_ROTATOR_CAMERA_VARIABLE,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                camera_data,
                __buffer.add(0).cast::<FBlueprintCameraEvaluationDataRef>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &variable,
                __buffer.add(24).cast::<UPtr<URotator3dCameraVariable>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::gameplay_cameras::UBlueprintCameraVariableTableFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_cameras::U_BLUEPRINT_CAMERA_VARIABLE_TABLE_FUNCTION_LIBRARY_GET_ROTATOR_CAMERA_VARIABLE,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(32).cast::<crate::bindings::core_u_object::FRotator>().read()
        }
    }
    pub fn get_integer32_camera_variable(
        camera_data: &FBlueprintCameraEvaluationDataRef,
        variable: UPtr<UInteger32CameraVariable>,
    ) -> i32 {
        let mut __stack = crate::core_data::StackAlloc::<36>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_cameras::U_BLUEPRINT_CAMERA_VARIABLE_TABLE_FUNCTION_LIBRARY_GET_INTEGER32_CAMERA_VARIABLE,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                camera_data,
                __buffer.add(0).cast::<FBlueprintCameraEvaluationDataRef>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &variable,
                __buffer.add(24).cast::<UPtr<UInteger32CameraVariable>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::gameplay_cameras::UBlueprintCameraVariableTableFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_cameras::U_BLUEPRINT_CAMERA_VARIABLE_TABLE_FUNCTION_LIBRARY_GET_INTEGER32_CAMERA_VARIABLE,
                __buffer,
            )
        };
        unsafe { __buffer.add(32).cast::<i32>().read() }
    }
    pub fn get_float_camera_variable(
        camera_data: &FBlueprintCameraEvaluationDataRef,
        variable: UPtr<UFloatCameraVariable>,
    ) -> f32 {
        let mut __stack = crate::core_data::StackAlloc::<36>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_cameras::U_BLUEPRINT_CAMERA_VARIABLE_TABLE_FUNCTION_LIBRARY_GET_FLOAT_CAMERA_VARIABLE,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                camera_data,
                __buffer.add(0).cast::<FBlueprintCameraEvaluationDataRef>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &variable,
                __buffer.add(24).cast::<UPtr<UFloatCameraVariable>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::gameplay_cameras::UBlueprintCameraVariableTableFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_cameras::U_BLUEPRINT_CAMERA_VARIABLE_TABLE_FUNCTION_LIBRARY_GET_FLOAT_CAMERA_VARIABLE,
                __buffer,
            )
        };
        unsafe { __buffer.add(32).cast::<f32>().read() }
    }
    pub fn get_double_camera_variable(
        camera_data: &FBlueprintCameraEvaluationDataRef,
        variable: UPtr<UDoubleCameraVariable>,
    ) -> f64 {
        let mut __stack = crate::core_data::StackAlloc::<40>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_cameras::U_BLUEPRINT_CAMERA_VARIABLE_TABLE_FUNCTION_LIBRARY_GET_DOUBLE_CAMERA_VARIABLE,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                camera_data,
                __buffer.add(0).cast::<FBlueprintCameraEvaluationDataRef>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &variable,
                __buffer.add(24).cast::<UPtr<UDoubleCameraVariable>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::gameplay_cameras::UBlueprintCameraVariableTableFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_cameras::U_BLUEPRINT_CAMERA_VARIABLE_TABLE_FUNCTION_LIBRARY_GET_DOUBLE_CAMERA_VARIABLE,
                __buffer,
            )
        };
        unsafe { __buffer.add(32).cast::<f64>().read() }
    }
    pub fn get_boolean_camera_variable(
        camera_data: &FBlueprintCameraEvaluationDataRef,
        variable: UPtr<UBooleanCameraVariable>,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<33>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_cameras::U_BLUEPRINT_CAMERA_VARIABLE_TABLE_FUNCTION_LIBRARY_GET_BOOLEAN_CAMERA_VARIABLE,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                camera_data,
                __buffer.add(0).cast::<FBlueprintCameraEvaluationDataRef>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &variable,
                __buffer.add(24).cast::<UPtr<UBooleanCameraVariable>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::gameplay_cameras::UBlueprintCameraVariableTableFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_cameras::U_BLUEPRINT_CAMERA_VARIABLE_TABLE_FUNCTION_LIBRARY_GET_BOOLEAN_CAMERA_VARIABLE,
                __buffer,
            )
        };
        unsafe { __buffer.add(32).cast::<bool>().read() }
    }
}
#[repr(C, align(8))]
pub struct UBlueprintCameraContextDataTableFunctionLibrary {
    __padding_end: [u8; 48],
}
impl UBlueprintCameraContextDataTableFunctionLibrary {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBlueprintCameraContextDataTableFunctionLibrary")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
    pub fn set_struct_data(
        camera_data: &FBlueprintCameraEvaluationDataRef,
        data_id: FCameraContextDataID,
        data: &crate::bindings::core_u_object::FInstancedStruct,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<49>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_cameras::U_BLUEPRINT_CAMERA_CONTEXT_DATA_TABLE_FUNCTION_LIBRARY_SET_STRUCT_DATA,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                camera_data,
                __buffer.add(0).cast::<FBlueprintCameraEvaluationDataRef>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &data_id,
                __buffer.add(24).cast::<FCameraContextDataID>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                data,
                __buffer
                    .add(32)
                    .cast::<crate::bindings::core_u_object::FInstancedStruct>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::gameplay_cameras::UBlueprintCameraContextDataTableFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_cameras::U_BLUEPRINT_CAMERA_CONTEXT_DATA_TABLE_FUNCTION_LIBRARY_SET_STRUCT_DATA,
                __buffer,
            )
        };
        unsafe { __buffer.add(48).cast::<bool>().read() }
    }
    pub fn set_string_data(
        camera_data: &FBlueprintCameraEvaluationDataRef,
        data_id: FCameraContextDataID,
        data: FString,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<49>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_cameras::U_BLUEPRINT_CAMERA_CONTEXT_DATA_TABLE_FUNCTION_LIBRARY_SET_STRING_DATA,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                camera_data,
                __buffer.add(0).cast::<FBlueprintCameraEvaluationDataRef>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &data_id,
                __buffer.add(24).cast::<FCameraContextDataID>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&data, __buffer.add(32).cast::<FString>(), 1);
        }
        let __object_ptr = crate::bindings::gameplay_cameras::UBlueprintCameraContextDataTableFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_cameras::U_BLUEPRINT_CAMERA_CONTEXT_DATA_TABLE_FUNCTION_LIBRARY_SET_STRING_DATA,
                __buffer,
            )
        };
        unsafe { __buffer.add(48).cast::<bool>().read() }
    }
    pub fn set_object_data(
        camera_data: &FBlueprintCameraEvaluationDataRef,
        data_id: FCameraContextDataID,
        data: UPtr<crate::bindings::core_u_object::UObject>,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<41>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_cameras::U_BLUEPRINT_CAMERA_CONTEXT_DATA_TABLE_FUNCTION_LIBRARY_SET_OBJECT_DATA,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                camera_data,
                __buffer.add(0).cast::<FBlueprintCameraEvaluationDataRef>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &data_id,
                __buffer.add(24).cast::<FCameraContextDataID>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &data,
                __buffer.add(32).cast::<UPtr<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::gameplay_cameras::UBlueprintCameraContextDataTableFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_cameras::U_BLUEPRINT_CAMERA_CONTEXT_DATA_TABLE_FUNCTION_LIBRARY_SET_OBJECT_DATA,
                __buffer,
            )
        };
        unsafe { __buffer.add(40).cast::<bool>().read() }
    }
    pub fn set_name_data(
        camera_data: &FBlueprintCameraEvaluationDataRef,
        data_id: FCameraContextDataID,
        data: &FName,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<41>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_cameras::U_BLUEPRINT_CAMERA_CONTEXT_DATA_TABLE_FUNCTION_LIBRARY_SET_NAME_DATA,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                camera_data,
                __buffer.add(0).cast::<FBlueprintCameraEvaluationDataRef>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &data_id,
                __buffer.add(24).cast::<FCameraContextDataID>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(data, __buffer.add(28).cast::<FName>(), 1);
        }
        let __object_ptr = crate::bindings::gameplay_cameras::UBlueprintCameraContextDataTableFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_cameras::U_BLUEPRINT_CAMERA_CONTEXT_DATA_TABLE_FUNCTION_LIBRARY_SET_NAME_DATA,
                __buffer,
            )
        };
        unsafe { __buffer.add(40).cast::<bool>().read() }
    }
    pub fn set_enum_data(
        camera_data: &FBlueprintCameraEvaluationDataRef,
        data_id: FCameraContextDataID,
        enum_type: UPtr<crate::bindings::core_u_object::UEnum>,
        data: u8,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<42>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_cameras::U_BLUEPRINT_CAMERA_CONTEXT_DATA_TABLE_FUNCTION_LIBRARY_SET_ENUM_DATA,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                camera_data,
                __buffer.add(0).cast::<FBlueprintCameraEvaluationDataRef>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &data_id,
                __buffer.add(24).cast::<FCameraContextDataID>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &enum_type,
                __buffer.add(32).cast::<UPtr<crate::bindings::core_u_object::UEnum>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&data, __buffer.add(40).cast::<u8>(), 1);
        }
        let __object_ptr = crate::bindings::gameplay_cameras::UBlueprintCameraContextDataTableFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_cameras::U_BLUEPRINT_CAMERA_CONTEXT_DATA_TABLE_FUNCTION_LIBRARY_SET_ENUM_DATA,
                __buffer,
            )
        };
        unsafe { __buffer.add(41).cast::<bool>().read() }
    }
    pub fn set_class_data(
        camera_data: &FBlueprintCameraEvaluationDataRef,
        data_id: FCameraContextDataID,
        data: TSubclassOf<crate::bindings::core_u_object::UObject>,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<41>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_cameras::U_BLUEPRINT_CAMERA_CONTEXT_DATA_TABLE_FUNCTION_LIBRARY_SET_CLASS_DATA,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                camera_data,
                __buffer.add(0).cast::<FBlueprintCameraEvaluationDataRef>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &data_id,
                __buffer.add(24).cast::<FCameraContextDataID>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &data,
                __buffer
                    .add(32)
                    .cast::<TSubclassOf<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::gameplay_cameras::UBlueprintCameraContextDataTableFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_cameras::U_BLUEPRINT_CAMERA_CONTEXT_DATA_TABLE_FUNCTION_LIBRARY_SET_CLASS_DATA,
                __buffer,
            )
        };
        unsafe { __buffer.add(40).cast::<bool>().read() }
    }
    pub fn get_struct_data(
        camera_data: &FBlueprintCameraEvaluationDataRef,
        data_id: FCameraContextDataID,
        data_struct_type: UPtr<crate::bindings::core_u_object::UScriptStruct>,
    ) -> crate::bindings::core_u_object::FInstancedStruct {
        let mut __stack = crate::core_data::StackAlloc::<56>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_cameras::U_BLUEPRINT_CAMERA_CONTEXT_DATA_TABLE_FUNCTION_LIBRARY_GET_STRUCT_DATA,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                camera_data,
                __buffer.add(0).cast::<FBlueprintCameraEvaluationDataRef>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &data_id,
                __buffer.add(24).cast::<FCameraContextDataID>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &data_struct_type,
                __buffer
                    .add(32)
                    .cast::<UPtr<crate::bindings::core_u_object::UScriptStruct>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::gameplay_cameras::UBlueprintCameraContextDataTableFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_cameras::U_BLUEPRINT_CAMERA_CONTEXT_DATA_TABLE_FUNCTION_LIBRARY_GET_STRUCT_DATA,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(40)
                .cast::<crate::bindings::core_u_object::FInstancedStruct>()
                .read()
        }
    }
    pub fn get_string_data(
        camera_data: &FBlueprintCameraEvaluationDataRef,
        data_id: FCameraContextDataID,
    ) -> FString {
        let mut __stack = crate::core_data::StackAlloc::<48>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_cameras::U_BLUEPRINT_CAMERA_CONTEXT_DATA_TABLE_FUNCTION_LIBRARY_GET_STRING_DATA,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                camera_data,
                __buffer.add(0).cast::<FBlueprintCameraEvaluationDataRef>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &data_id,
                __buffer.add(24).cast::<FCameraContextDataID>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::gameplay_cameras::UBlueprintCameraContextDataTableFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_cameras::U_BLUEPRINT_CAMERA_CONTEXT_DATA_TABLE_FUNCTION_LIBRARY_GET_STRING_DATA,
                __buffer,
            )
        };
        unsafe { __buffer.add(32).cast::<FString>().read() }
    }
    pub fn get_object_data(
        camera_data: &FBlueprintCameraEvaluationDataRef,
        data_id: FCameraContextDataID,
    ) -> UPtr<crate::bindings::core_u_object::UObject> {
        let mut __stack = crate::core_data::StackAlloc::<40>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_cameras::U_BLUEPRINT_CAMERA_CONTEXT_DATA_TABLE_FUNCTION_LIBRARY_GET_OBJECT_DATA,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                camera_data,
                __buffer.add(0).cast::<FBlueprintCameraEvaluationDataRef>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &data_id,
                __buffer.add(24).cast::<FCameraContextDataID>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::gameplay_cameras::UBlueprintCameraContextDataTableFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_cameras::U_BLUEPRINT_CAMERA_CONTEXT_DATA_TABLE_FUNCTION_LIBRARY_GET_OBJECT_DATA,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(32)
                .cast::<UPtr<crate::bindings::core_u_object::UObject>>()
                .read()
        }
    }
    pub fn get_name_data(
        camera_data: &FBlueprintCameraEvaluationDataRef,
        data_id: FCameraContextDataID,
    ) -> FName {
        let mut __stack = crate::core_data::StackAlloc::<40>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_cameras::U_BLUEPRINT_CAMERA_CONTEXT_DATA_TABLE_FUNCTION_LIBRARY_GET_NAME_DATA,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                camera_data,
                __buffer.add(0).cast::<FBlueprintCameraEvaluationDataRef>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &data_id,
                __buffer.add(24).cast::<FCameraContextDataID>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::gameplay_cameras::UBlueprintCameraContextDataTableFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_cameras::U_BLUEPRINT_CAMERA_CONTEXT_DATA_TABLE_FUNCTION_LIBRARY_GET_NAME_DATA,
                __buffer,
            )
        };
        unsafe { __buffer.add(28).cast::<FName>().read() }
    }
    pub fn get_enum_data(
        camera_data: &FBlueprintCameraEvaluationDataRef,
        data_id: FCameraContextDataID,
        enum_type: UPtr<crate::bindings::core_u_object::UEnum>,
    ) -> u8 {
        let mut __stack = crate::core_data::StackAlloc::<41>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_cameras::U_BLUEPRINT_CAMERA_CONTEXT_DATA_TABLE_FUNCTION_LIBRARY_GET_ENUM_DATA,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                camera_data,
                __buffer.add(0).cast::<FBlueprintCameraEvaluationDataRef>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &data_id,
                __buffer.add(24).cast::<FCameraContextDataID>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &enum_type,
                __buffer.add(32).cast::<UPtr<crate::bindings::core_u_object::UEnum>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::gameplay_cameras::UBlueprintCameraContextDataTableFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_cameras::U_BLUEPRINT_CAMERA_CONTEXT_DATA_TABLE_FUNCTION_LIBRARY_GET_ENUM_DATA,
                __buffer,
            )
        };
        unsafe { __buffer.add(40).cast::<u8>().read() }
    }
    pub fn get_class_data(
        camera_data: &FBlueprintCameraEvaluationDataRef,
        data_id: FCameraContextDataID,
    ) -> TSubclassOf<crate::bindings::core_u_object::UObject> {
        let mut __stack = crate::core_data::StackAlloc::<40>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_cameras::U_BLUEPRINT_CAMERA_CONTEXT_DATA_TABLE_FUNCTION_LIBRARY_GET_CLASS_DATA,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                camera_data,
                __buffer.add(0).cast::<FBlueprintCameraEvaluationDataRef>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &data_id,
                __buffer.add(24).cast::<FCameraContextDataID>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::gameplay_cameras::UBlueprintCameraContextDataTableFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_cameras::U_BLUEPRINT_CAMERA_CONTEXT_DATA_TABLE_FUNCTION_LIBRARY_GET_CLASS_DATA,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(32)
                .cast::<TSubclassOf<crate::bindings::core_u_object::UObject>>()
                .read()
        }
    }
}
#[repr(C, align(8))]
pub struct UBlueprintCameraPoseFunctionLibrary {
    __padding_end: [u8; 48],
}
impl UBlueprintCameraPoseFunctionLibrary {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBlueprintCameraPoseFunctionLibrary")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
    pub fn set_transform(
        camera_pose: &FBlueprintCameraPose,
        transform: &crate::bindings::core_u_object::FTransform,
    ) -> FBlueprintCameraPose {
        let mut __stack = crate::core_data::StackAlloc::<376>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_cameras::U_BLUEPRINT_CAMERA_POSE_FUNCTION_LIBRARY_SET_TRANSFORM,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                camera_pose,
                __buffer.add(0).cast::<FBlueprintCameraPose>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                transform,
                __buffer.add(144).cast::<crate::bindings::core_u_object::FTransform>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::gameplay_cameras::UBlueprintCameraPoseFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_cameras::U_BLUEPRINT_CAMERA_POSE_FUNCTION_LIBRARY_SET_TRANSFORM,
                __buffer,
            )
        };
        unsafe { __buffer.add(240).cast::<FBlueprintCameraPose>().read() }
    }
    pub fn set_target_distance(
        camera_pose: &FBlueprintCameraPose,
        target_distance: f64,
    ) -> FBlueprintCameraPose {
        let mut __stack = crate::core_data::StackAlloc::<280>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_cameras::U_BLUEPRINT_CAMERA_POSE_FUNCTION_LIBRARY_SET_TARGET_DISTANCE,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                camera_pose,
                __buffer.add(0).cast::<FBlueprintCameraPose>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &target_distance,
                __buffer.add(136).cast::<f64>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::gameplay_cameras::UBlueprintCameraPoseFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_cameras::U_BLUEPRINT_CAMERA_POSE_FUNCTION_LIBRARY_SET_TARGET_DISTANCE,
                __buffer,
            )
        };
        unsafe { __buffer.add(144).cast::<FBlueprintCameraPose>().read() }
    }
    pub fn set_rotation(
        camera_pose: &FBlueprintCameraPose,
        rotation: &crate::bindings::core_u_object::FRotator,
    ) -> FBlueprintCameraPose {
        let mut __stack = crate::core_data::StackAlloc::<296>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_cameras::U_BLUEPRINT_CAMERA_POSE_FUNCTION_LIBRARY_SET_ROTATION,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                camera_pose,
                __buffer.add(0).cast::<FBlueprintCameraPose>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                rotation,
                __buffer.add(136).cast::<crate::bindings::core_u_object::FRotator>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::gameplay_cameras::UBlueprintCameraPoseFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_cameras::U_BLUEPRINT_CAMERA_POSE_FUNCTION_LIBRARY_SET_ROTATION,
                __buffer,
            )
        };
        unsafe { __buffer.add(160).cast::<FBlueprintCameraPose>().read() }
    }
    pub fn set_location(
        camera_pose: &FBlueprintCameraPose,
        location: &crate::bindings::core_u_object::FVector,
    ) -> FBlueprintCameraPose {
        let mut __stack = crate::core_data::StackAlloc::<296>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_cameras::U_BLUEPRINT_CAMERA_POSE_FUNCTION_LIBRARY_SET_LOCATION,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                camera_pose,
                __buffer.add(0).cast::<FBlueprintCameraPose>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                location,
                __buffer.add(136).cast::<crate::bindings::core_u_object::FVector>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::gameplay_cameras::UBlueprintCameraPoseFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_cameras::U_BLUEPRINT_CAMERA_POSE_FUNCTION_LIBRARY_SET_LOCATION,
                __buffer,
            )
        };
        unsafe { __buffer.add(160).cast::<FBlueprintCameraPose>().read() }
    }
    pub fn set_focal_length(
        camera_pose: &FBlueprintCameraPose,
        focal_length: f32,
    ) -> FBlueprintCameraPose {
        let mut __stack = crate::core_data::StackAlloc::<280>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_cameras::U_BLUEPRINT_CAMERA_POSE_FUNCTION_LIBRARY_SET_FOCAL_LENGTH,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                camera_pose,
                __buffer.add(0).cast::<FBlueprintCameraPose>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &focal_length,
                __buffer.add(136).cast::<f32>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::gameplay_cameras::UBlueprintCameraPoseFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_cameras::U_BLUEPRINT_CAMERA_POSE_FUNCTION_LIBRARY_SET_FOCAL_LENGTH,
                __buffer,
            )
        };
        unsafe { __buffer.add(144).cast::<FBlueprintCameraPose>().read() }
    }
    pub fn set_field_of_view(
        camera_pose: &FBlueprintCameraPose,
        field_of_view: f32,
    ) -> FBlueprintCameraPose {
        let mut __stack = crate::core_data::StackAlloc::<280>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_cameras::U_BLUEPRINT_CAMERA_POSE_FUNCTION_LIBRARY_SET_FIELD_OF_VIEW,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                camera_pose,
                __buffer.add(0).cast::<FBlueprintCameraPose>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &field_of_view,
                __buffer.add(136).cast::<f32>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::gameplay_cameras::UBlueprintCameraPoseFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_cameras::U_BLUEPRINT_CAMERA_POSE_FUNCTION_LIBRARY_SET_FIELD_OF_VIEW,
                __buffer,
            )
        };
        unsafe { __buffer.add(144).cast::<FBlueprintCameraPose>().read() }
    }
    pub fn make_camera_pose_from_cine_camera_component(
        camera_component: UPtr<crate::bindings::cinematic_camera::UCineCameraComponent>,
    ) -> FBlueprintCameraPose {
        let mut __stack = crate::core_data::StackAlloc::<144>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_cameras::U_BLUEPRINT_CAMERA_POSE_FUNCTION_LIBRARY_MAKE_CAMERA_POSE_FROM_CINE_CAMERA_COMPONENT,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &camera_component,
                __buffer
                    .add(0)
                    .cast::<
                        UPtr<crate::bindings::cinematic_camera::UCineCameraComponent>,
                    >(),
                1,
            );
        }
        let __object_ptr = crate::bindings::gameplay_cameras::UBlueprintCameraPoseFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_cameras::U_BLUEPRINT_CAMERA_POSE_FUNCTION_LIBRARY_MAKE_CAMERA_POSE_FROM_CINE_CAMERA_COMPONENT,
                __buffer,
            )
        };
        unsafe { __buffer.add(8).cast::<FBlueprintCameraPose>().read() }
    }
    pub fn make_camera_pose_from_camera_component(
        camera_component: UPtr<crate::bindings::engine::UCameraComponent>,
    ) -> FBlueprintCameraPose {
        let mut __stack = crate::core_data::StackAlloc::<144>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_cameras::U_BLUEPRINT_CAMERA_POSE_FUNCTION_LIBRARY_MAKE_CAMERA_POSE_FROM_CAMERA_COMPONENT,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &camera_component,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::engine::UCameraComponent>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::gameplay_cameras::UBlueprintCameraPoseFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_cameras::U_BLUEPRINT_CAMERA_POSE_FUNCTION_LIBRARY_MAKE_CAMERA_POSE_FROM_CAMERA_COMPONENT,
                __buffer,
            )
        };
        unsafe { __buffer.add(8).cast::<FBlueprintCameraPose>().read() }
    }
    pub fn get_transform(
        camera_pose: &FBlueprintCameraPose,
    ) -> crate::bindings::core_u_object::FTransform {
        let mut __stack = crate::core_data::StackAlloc::<240>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_cameras::U_BLUEPRINT_CAMERA_POSE_FUNCTION_LIBRARY_GET_TRANSFORM,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                camera_pose,
                __buffer.add(0).cast::<FBlueprintCameraPose>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::gameplay_cameras::UBlueprintCameraPoseFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_cameras::U_BLUEPRINT_CAMERA_POSE_FUNCTION_LIBRARY_GET_TRANSFORM,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(144).cast::<crate::bindings::core_u_object::FTransform>().read()
        }
    }
    pub fn get_target_distance(camera_pose: &FBlueprintCameraPose) -> f64 {
        let mut __stack = crate::core_data::StackAlloc::<144>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_cameras::U_BLUEPRINT_CAMERA_POSE_FUNCTION_LIBRARY_GET_TARGET_DISTANCE,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                camera_pose,
                __buffer.add(0).cast::<FBlueprintCameraPose>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::gameplay_cameras::UBlueprintCameraPoseFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_cameras::U_BLUEPRINT_CAMERA_POSE_FUNCTION_LIBRARY_GET_TARGET_DISTANCE,
                __buffer,
            )
        };
        unsafe { __buffer.add(136).cast::<f64>().read() }
    }
    pub fn get_target_at_distance(
        camera_pose: &FBlueprintCameraPose,
        target_distance: f64,
    ) -> crate::bindings::core_u_object::FVector {
        let mut __stack = crate::core_data::StackAlloc::<168>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_cameras::U_BLUEPRINT_CAMERA_POSE_FUNCTION_LIBRARY_GET_TARGET_AT_DISTANCE,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                camera_pose,
                __buffer.add(0).cast::<FBlueprintCameraPose>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &target_distance,
                __buffer.add(136).cast::<f64>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::gameplay_cameras::UBlueprintCameraPoseFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_cameras::U_BLUEPRINT_CAMERA_POSE_FUNCTION_LIBRARY_GET_TARGET_AT_DISTANCE,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(144).cast::<crate::bindings::core_u_object::FVector>().read()
        }
    }
    pub fn get_target(
        camera_pose: &FBlueprintCameraPose,
    ) -> crate::bindings::core_u_object::FVector {
        let mut __stack = crate::core_data::StackAlloc::<160>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_cameras::U_BLUEPRINT_CAMERA_POSE_FUNCTION_LIBRARY_GET_TARGET,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                camera_pose,
                __buffer.add(0).cast::<FBlueprintCameraPose>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::gameplay_cameras::UBlueprintCameraPoseFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_cameras::U_BLUEPRINT_CAMERA_POSE_FUNCTION_LIBRARY_GET_TARGET,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(136).cast::<crate::bindings::core_u_object::FVector>().read()
        }
    }
    pub fn get_sensor_aspect_ratio(camera_pose: &FBlueprintCameraPose) -> f64 {
        let mut __stack = crate::core_data::StackAlloc::<144>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_cameras::U_BLUEPRINT_CAMERA_POSE_FUNCTION_LIBRARY_GET_SENSOR_ASPECT_RATIO,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                camera_pose,
                __buffer.add(0).cast::<FBlueprintCameraPose>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::gameplay_cameras::UBlueprintCameraPoseFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_cameras::U_BLUEPRINT_CAMERA_POSE_FUNCTION_LIBRARY_GET_SENSOR_ASPECT_RATIO,
                __buffer,
            )
        };
        unsafe { __buffer.add(136).cast::<f64>().read() }
    }
    pub fn get_rotation(
        camera_pose: &FBlueprintCameraPose,
    ) -> crate::bindings::core_u_object::FRotator {
        let mut __stack = crate::core_data::StackAlloc::<160>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_cameras::U_BLUEPRINT_CAMERA_POSE_FUNCTION_LIBRARY_GET_ROTATION,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                camera_pose,
                __buffer.add(0).cast::<FBlueprintCameraPose>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::gameplay_cameras::UBlueprintCameraPoseFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_cameras::U_BLUEPRINT_CAMERA_POSE_FUNCTION_LIBRARY_GET_ROTATION,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(136).cast::<crate::bindings::core_u_object::FRotator>().read()
        }
    }
    pub fn get_location(
        camera_pose: &FBlueprintCameraPose,
    ) -> crate::bindings::core_u_object::FVector {
        let mut __stack = crate::core_data::StackAlloc::<160>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_cameras::U_BLUEPRINT_CAMERA_POSE_FUNCTION_LIBRARY_GET_LOCATION,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                camera_pose,
                __buffer.add(0).cast::<FBlueprintCameraPose>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::gameplay_cameras::UBlueprintCameraPoseFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_cameras::U_BLUEPRINT_CAMERA_POSE_FUNCTION_LIBRARY_GET_LOCATION,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(136).cast::<crate::bindings::core_u_object::FVector>().read()
        }
    }
    pub fn get_focal_length(camera_pose: &FBlueprintCameraPose) -> f64 {
        let mut __stack = crate::core_data::StackAlloc::<144>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_cameras::U_BLUEPRINT_CAMERA_POSE_FUNCTION_LIBRARY_GET_FOCAL_LENGTH,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                camera_pose,
                __buffer.add(0).cast::<FBlueprintCameraPose>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::gameplay_cameras::UBlueprintCameraPoseFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_cameras::U_BLUEPRINT_CAMERA_POSE_FUNCTION_LIBRARY_GET_FOCAL_LENGTH,
                __buffer,
            )
        };
        unsafe { __buffer.add(136).cast::<f64>().read() }
    }
    pub fn get_field_of_view(camera_pose: &FBlueprintCameraPose) -> f64 {
        let mut __stack = crate::core_data::StackAlloc::<144>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_cameras::U_BLUEPRINT_CAMERA_POSE_FUNCTION_LIBRARY_GET_FIELD_OF_VIEW,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                camera_pose,
                __buffer.add(0).cast::<FBlueprintCameraPose>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::gameplay_cameras::UBlueprintCameraPoseFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_cameras::U_BLUEPRINT_CAMERA_POSE_FUNCTION_LIBRARY_GET_FIELD_OF_VIEW,
                __buffer,
            )
        };
        unsafe { __buffer.add(136).cast::<f64>().read() }
    }
    pub fn get_effective_field_of_view(camera_pose: &FBlueprintCameraPose) -> f64 {
        let mut __stack = crate::core_data::StackAlloc::<144>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_cameras::U_BLUEPRINT_CAMERA_POSE_FUNCTION_LIBRARY_GET_EFFECTIVE_FIELD_OF_VIEW,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                camera_pose,
                __buffer.add(0).cast::<FBlueprintCameraPose>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::gameplay_cameras::UBlueprintCameraPoseFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_cameras::U_BLUEPRINT_CAMERA_POSE_FUNCTION_LIBRARY_GET_EFFECTIVE_FIELD_OF_VIEW,
                __buffer,
            )
        };
        unsafe { __buffer.add(136).cast::<f64>().read() }
    }
    pub fn get_aim_ray(
        camera_pose: &FBlueprintCameraPose,
    ) -> crate::bindings::core_u_object::FRay {
        let mut __stack = crate::core_data::StackAlloc::<184>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_cameras::U_BLUEPRINT_CAMERA_POSE_FUNCTION_LIBRARY_GET_AIM_RAY,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                camera_pose,
                __buffer.add(0).cast::<FBlueprintCameraPose>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::gameplay_cameras::UBlueprintCameraPoseFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_cameras::U_BLUEPRINT_CAMERA_POSE_FUNCTION_LIBRARY_GET_AIM_RAY,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(136).cast::<crate::bindings::core_u_object::FRay>().read()
        }
    }
    pub fn get_aim_dir(
        camera_pose: &FBlueprintCameraPose,
    ) -> crate::bindings::core_u_object::FVector {
        let mut __stack = crate::core_data::StackAlloc::<160>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_cameras::U_BLUEPRINT_CAMERA_POSE_FUNCTION_LIBRARY_GET_AIM_DIR,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                camera_pose,
                __buffer.add(0).cast::<FBlueprintCameraPose>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::gameplay_cameras::UBlueprintCameraPoseFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_cameras::U_BLUEPRINT_CAMERA_POSE_FUNCTION_LIBRARY_GET_AIM_DIR,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(136).cast::<crate::bindings::core_u_object::FVector>().read()
        }
    }
}
#[repr(C, align(8))]
pub struct UCameraRigParameterInterop {
    __padding_end: [u8; 48],
}
impl UCameraRigParameterInterop {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UCameraRigParameterInterop")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
    pub fn set_camera_parameter(
        camera_data: &FBlueprintCameraEvaluationDataRef,
        camera_rig: UPtr<UCameraRigAsset>,
        parameter_name: FName,
        new_value: &i32,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<48>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_cameras::U_CAMERA_RIG_PARAMETER_INTEROP_SET_CAMERA_PARAMETER,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                camera_data,
                __buffer.add(0).cast::<FBlueprintCameraEvaluationDataRef>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &camera_rig,
                __buffer.add(24).cast::<UPtr<UCameraRigAsset>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &parameter_name,
                __buffer.add(32).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(new_value, __buffer.add(44).cast::<i32>(), 1);
        }
        let __object_ptr = crate::bindings::gameplay_cameras::UCameraRigParameterInterop::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_cameras::U_CAMERA_RIG_PARAMETER_INTEROP_SET_CAMERA_PARAMETER,
                __buffer,
            )
        };
    }
    pub fn get_camera_parameter(
        camera_data: &FBlueprintCameraEvaluationDataRef,
        camera_rig: UPtr<UCameraRigAsset>,
        parameter_name: FName,
        return_value: &mut i32,
    ) -> i32 {
        let mut __stack = crate::core_data::StackAlloc::<48>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_cameras::U_CAMERA_RIG_PARAMETER_INTEROP_GET_CAMERA_PARAMETER,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                camera_data,
                __buffer.add(0).cast::<FBlueprintCameraEvaluationDataRef>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &camera_rig,
                __buffer.add(24).cast::<UPtr<UCameraRigAsset>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &parameter_name,
                __buffer.add(32).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                return_value,
                __buffer.add(44).cast::<i32>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::gameplay_cameras::UCameraRigParameterInterop::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_cameras::U_CAMERA_RIG_PARAMETER_INTEROP_GET_CAMERA_PARAMETER,
                __buffer,
            )
        };
        unsafe { __buffer.add(44).cast::<i32>().read() }
    }
}
#[repr(C, align(8))]
pub struct UCameraRigParameterInteropLibrary {
    __padding_end: [u8; 48],
}
impl UCameraRigParameterInteropLibrary {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UCameraRigParameterInteropLibrary")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
    pub fn make_literal_vector3f(
        value: crate::bindings::core_u_object::FVector3f,
    ) -> crate::bindings::core_u_object::FVector3f {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_cameras::U_CAMERA_RIG_PARAMETER_INTEROP_LIBRARY_MAKE_LITERAL_VECTOR3F,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &value,
                __buffer.add(0).cast::<crate::bindings::core_u_object::FVector3f>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::gameplay_cameras::UCameraRigParameterInteropLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_cameras::U_CAMERA_RIG_PARAMETER_INTEROP_LIBRARY_MAKE_LITERAL_VECTOR3F,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(12).cast::<crate::bindings::core_u_object::FVector3f>().read()
        }
    }
    pub fn make_literal_vector2_d(
        value: crate::bindings::core_u_object::FVector2D,
    ) -> crate::bindings::core_u_object::FVector2D {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_cameras::U_CAMERA_RIG_PARAMETER_INTEROP_LIBRARY_MAKE_LITERAL_VECTOR2_D,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &value,
                __buffer.add(0).cast::<crate::bindings::core_u_object::FVector2D>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::gameplay_cameras::UCameraRigParameterInteropLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_cameras::U_CAMERA_RIG_PARAMETER_INTEROP_LIBRARY_MAKE_LITERAL_VECTOR2_D,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(16).cast::<crate::bindings::core_u_object::FVector2D>().read()
        }
    }
    pub fn make_literal_vector(
        value: crate::bindings::core_u_object::FVector,
    ) -> crate::bindings::core_u_object::FVector {
        let mut __stack = crate::core_data::StackAlloc::<48>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_cameras::U_CAMERA_RIG_PARAMETER_INTEROP_LIBRARY_MAKE_LITERAL_VECTOR,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &value,
                __buffer.add(0).cast::<crate::bindings::core_u_object::FVector>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::gameplay_cameras::UCameraRigParameterInteropLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_cameras::U_CAMERA_RIG_PARAMETER_INTEROP_LIBRARY_MAKE_LITERAL_VECTOR,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(24).cast::<crate::bindings::core_u_object::FVector>().read()
        }
    }
    pub fn make_literal_rotator(
        value: crate::bindings::core_u_object::FRotator,
    ) -> crate::bindings::core_u_object::FRotator {
        let mut __stack = crate::core_data::StackAlloc::<48>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_cameras::U_CAMERA_RIG_PARAMETER_INTEROP_LIBRARY_MAKE_LITERAL_ROTATOR,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &value,
                __buffer.add(0).cast::<crate::bindings::core_u_object::FRotator>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::gameplay_cameras::UCameraRigParameterInteropLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_cameras::U_CAMERA_RIG_PARAMETER_INTEROP_LIBRARY_MAKE_LITERAL_ROTATOR,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(24).cast::<crate::bindings::core_u_object::FRotator>().read()
        }
    }
    pub fn make_literal_linear_color(
        value: crate::bindings::core_u_object::FLinearColor,
    ) -> crate::bindings::core_u_object::FLinearColor {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_cameras::U_CAMERA_RIG_PARAMETER_INTEROP_LIBRARY_MAKE_LITERAL_LINEAR_COLOR,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &value,
                __buffer.add(0).cast::<crate::bindings::core_u_object::FLinearColor>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::gameplay_cameras::UCameraRigParameterInteropLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_cameras::U_CAMERA_RIG_PARAMETER_INTEROP_LIBRARY_MAKE_LITERAL_LINEAR_COLOR,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(16)
                .cast::<crate::bindings::core_u_object::FLinearColor>()
                .read()
        }
    }
}
#[repr(C, align(8))]
pub struct UControllerGameplayCameraEvaluationComponent {
    __padding_end: [u8; 288],
}
impl UControllerGameplayCameraEvaluationComponent {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UControllerGameplayCameraEvaluationComponent")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct AGameplayCameraActorBase {
    __padding_end: [u8; 1136],
}
impl AGameplayCameraActorBase {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("AGameplayCameraActorBase")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct AGameplayCameraActor {
    #[doc(hidden)]
    __padding_1136: [u8; 1136],
    pub camera_component: UPtr<UGameplayCameraComponent>,
}
impl AGameplayCameraActor {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("AGameplayCameraActor")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
    pub fn get_camera_component(&self) -> UPtr<UGameplayCameraComponent> {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_cameras::A_GAMEPLAY_CAMERA_ACTOR_GET_CAMERA_COMPONENT,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_cameras::A_GAMEPLAY_CAMERA_ACTOR_GET_CAMERA_COMPONENT,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<UPtr<UGameplayCameraComponent>>().read() }
    }
}
#[repr(C, align(16))]
pub struct UGameplayCameraComponentBase {
    #[doc(hidden)]
    __padding_688: [u8; 688],
    pub auto_activate_for_player: crate::bindings::engine::EAutoReceiveInput,
    pub b_set_control_rotation_when_view_target: bool,
    __padding_end: [u8; 62],
}
impl UGameplayCameraComponentBase {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UGameplayCameraComponentBase")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
    pub fn get_output_camera_component(
        &self,
    ) -> UPtr<crate::bindings::cinematic_camera::UCineCameraComponent> {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_cameras::U_GAMEPLAY_CAMERA_COMPONENT_BASE_GET_OUTPUT_CAMERA_COMPONENT,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_cameras::U_GAMEPLAY_CAMERA_COMPONENT_BASE_GET_OUTPUT_CAMERA_COMPONENT,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(0)
                .cast::<UPtr<crate::bindings::cinematic_camera::UCineCameraComponent>>()
                .read()
        }
    }
    pub fn get_initial_result(&self) -> FBlueprintCameraEvaluationDataRef {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_cameras::U_GAMEPLAY_CAMERA_COMPONENT_BASE_GET_INITIAL_RESULT,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_cameras::U_GAMEPLAY_CAMERA_COMPONENT_BASE_GET_INITIAL_RESULT,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<FBlueprintCameraEvaluationDataRef>().read() }
    }
    pub fn get_evaluated_camera_rotation(
        &self,
    ) -> crate::bindings::core_u_object::FRotator {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_cameras::U_GAMEPLAY_CAMERA_COMPONENT_BASE_GET_EVALUATED_CAMERA_ROTATION,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_cameras::U_GAMEPLAY_CAMERA_COMPONENT_BASE_GET_EVALUATED_CAMERA_ROTATION,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<crate::bindings::core_u_object::FRotator>().read()
        }
    }
    pub fn get_conditional_result(
        &self,
        condition: ECameraEvaluationDataCondition,
    ) -> FBlueprintCameraEvaluationDataRef {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_cameras::U_GAMEPLAY_CAMERA_COMPONENT_BASE_GET_CONDITIONAL_RESULT,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &condition,
                __buffer.add(0).cast::<ECameraEvaluationDataCondition>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_cameras::U_GAMEPLAY_CAMERA_COMPONENT_BASE_GET_CONDITIONAL_RESULT,
                __buffer,
            )
        };
        unsafe { __buffer.add(8).cast::<FBlueprintCameraEvaluationDataRef>().read() }
    }
    pub fn deactivate_camera(&mut self, b_immediately: bool) {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_cameras::U_GAMEPLAY_CAMERA_COMPONENT_BASE_DEACTIVATE_CAMERA,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_immediately,
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
                crate::bindings::gameplay_cameras::U_GAMEPLAY_CAMERA_COMPONENT_BASE_DEACTIVATE_CAMERA,
                __buffer,
            )
        };
    }
    pub fn activate_persistent_visual_camera_rig(
        &mut self,
        camera_rig: UPtr<UCameraRigAsset>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_cameras::U_GAMEPLAY_CAMERA_COMPONENT_BASE_ACTIVATE_PERSISTENT_VISUAL_CAMERA_RIG,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &camera_rig,
                __buffer.add(0).cast::<UPtr<UCameraRigAsset>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_cameras::U_GAMEPLAY_CAMERA_COMPONENT_BASE_ACTIVATE_PERSISTENT_VISUAL_CAMERA_RIG,
                __buffer,
            )
        };
    }
    pub fn activate_persistent_global_camera_rig(
        &mut self,
        camera_rig: UPtr<UCameraRigAsset>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_cameras::U_GAMEPLAY_CAMERA_COMPONENT_BASE_ACTIVATE_PERSISTENT_GLOBAL_CAMERA_RIG,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &camera_rig,
                __buffer.add(0).cast::<UPtr<UCameraRigAsset>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_cameras::U_GAMEPLAY_CAMERA_COMPONENT_BASE_ACTIVATE_PERSISTENT_GLOBAL_CAMERA_RIG,
                __buffer,
            )
        };
    }
    pub fn activate_persistent_base_camera_rig(
        &mut self,
        camera_rig: UPtr<UCameraRigAsset>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_cameras::U_GAMEPLAY_CAMERA_COMPONENT_BASE_ACTIVATE_PERSISTENT_BASE_CAMERA_RIG,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &camera_rig,
                __buffer.add(0).cast::<UPtr<UCameraRigAsset>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_cameras::U_GAMEPLAY_CAMERA_COMPONENT_BASE_ACTIVATE_PERSISTENT_BASE_CAMERA_RIG,
                __buffer,
            )
        };
    }
    pub fn activate_camera_for_player_index(
        &mut self,
        player_index: i32,
        b_set_as_view_target: bool,
        activation_mode: EGameplayCameraComponentActivationMode,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<6>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_cameras::U_GAMEPLAY_CAMERA_COMPONENT_BASE_ACTIVATE_CAMERA_FOR_PLAYER_INDEX,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &player_index,
                __buffer.add(0).cast::<i32>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_set_as_view_target,
                __buffer.add(4).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &activation_mode,
                __buffer.add(5).cast::<EGameplayCameraComponentActivationMode>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_cameras::U_GAMEPLAY_CAMERA_COMPONENT_BASE_ACTIVATE_CAMERA_FOR_PLAYER_INDEX,
                __buffer,
            )
        };
    }
    pub fn activate_camera_for_player_controller(
        &mut self,
        player_controller: UPtr<crate::bindings::engine::APlayerController>,
        b_set_as_view_target: bool,
        activation_mode: EGameplayCameraComponentActivationMode,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<10>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_cameras::U_GAMEPLAY_CAMERA_COMPONENT_BASE_ACTIVATE_CAMERA_FOR_PLAYER_CONTROLLER,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &player_controller,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::engine::APlayerController>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_set_as_view_target,
                __buffer.add(8).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &activation_mode,
                __buffer.add(9).cast::<EGameplayCameraComponentActivationMode>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_cameras::U_GAMEPLAY_CAMERA_COMPONENT_BASE_ACTIVATE_CAMERA_FOR_PLAYER_CONTROLLER,
                __buffer,
            )
        };
    }
}
#[repr(C, align(16))]
pub struct UGameplayCameraComponent {
    #[doc(hidden)]
    __padding_744: [u8; 744],
    pub camera_reference: FCameraAssetReference,
    __padding_end: [u8; 32],
}
impl UGameplayCameraComponent {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UGameplayCameraComponent")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UGameplayCameraParameterSetterComponent {
    #[doc(hidden)]
    __padding_240: [u8; 240],
    pub camera_rig_reference: FCameraRigAssetReference,
    pub blend_in_time: f32,
    pub blend_out_time: f32,
    pub blend_type: ECameraVariableSetterBlendType,
    __padding_end: [u8; 23],
}
impl UGameplayCameraParameterSetterComponent {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UGameplayCameraParameterSetterComponent")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
    pub fn stop_parameter_setters(&mut self, b_immediately: bool) {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_cameras::U_GAMEPLAY_CAMERA_PARAMETER_SETTER_COMPONENT_STOP_PARAMETER_SETTERS,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_immediately,
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
                crate::bindings::gameplay_cameras::U_GAMEPLAY_CAMERA_PARAMETER_SETTER_COMPONENT_STOP_PARAMETER_SETTERS,
                __buffer,
            )
        };
    }
    pub fn start_parameter_setters(&mut self) {
        let mut __stack = crate::core_data::StackAlloc::<0>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_cameras::U_GAMEPLAY_CAMERA_PARAMETER_SETTER_COMPONENT_START_PARAMETER_SETTERS,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_cameras::U_GAMEPLAY_CAMERA_PARAMETER_SETTER_COMPONENT_START_PARAMETER_SETTERS,
                __buffer,
            )
        };
    }
}
#[repr(C, align(8))]
pub struct AGameplayCameraRigActor {
    #[doc(hidden)]
    __padding_1136: [u8; 1136],
    pub camera_rig_component: UPtr<UGameplayCameraRigComponent>,
}
impl AGameplayCameraRigActor {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("AGameplayCameraRigActor")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
    pub fn get_camera_rig_component(&self) -> UPtr<UGameplayCameraRigComponent> {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_cameras::A_GAMEPLAY_CAMERA_RIG_ACTOR_GET_CAMERA_RIG_COMPONENT,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_cameras::A_GAMEPLAY_CAMERA_RIG_ACTOR_GET_CAMERA_RIG_COMPONENT,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<UPtr<UGameplayCameraRigComponent>>().read() }
    }
}
#[repr(C, align(16))]
pub struct UGameplayCameraRigComponent {
    #[doc(hidden)]
    __padding_744: [u8; 744],
    pub camera_rig_reference: FCameraRigAssetReference,
    __padding_end: [u8; 40],
}
impl UGameplayCameraRigComponent {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UGameplayCameraRigComponent")
            .unwrap()
    }
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
pub struct AGameplayCamerasPlayerCameraManager {
    __padding_end: [u8; 11040],
}
impl AGameplayCamerasPlayerCameraManager {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("AGameplayCamerasPlayerCameraManager")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
    pub fn stop_camera_shake_asset(
        &mut self,
        in_instance_id: FCameraShakeInstanceID,
        b_immediately: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<6>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_cameras::A_GAMEPLAY_CAMERAS_PLAYER_CAMERA_MANAGER_STOP_CAMERA_SHAKE_ASSET,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_instance_id,
                __buffer.add(0).cast::<FCameraShakeInstanceID>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_immediately,
                __buffer.add(4).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_cameras::A_GAMEPLAY_CAMERAS_PLAYER_CAMERA_MANAGER_STOP_CAMERA_SHAKE_ASSET,
                __buffer,
            )
        };
        unsafe { __buffer.add(5).cast::<bool>().read() }
    }
    pub fn stop_camera_modifier_rig(
        &mut self,
        instance_id: FCameraRigInstanceID,
        b_immediately: bool,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<9>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_cameras::A_GAMEPLAY_CAMERAS_PLAYER_CAMERA_MANAGER_STOP_CAMERA_MODIFIER_RIG,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &instance_id,
                __buffer.add(0).cast::<FCameraRigInstanceID>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_immediately,
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
                crate::bindings::gameplay_cameras::A_GAMEPLAY_CAMERAS_PLAYER_CAMERA_MANAGER_STOP_CAMERA_MODIFIER_RIG,
                __buffer,
            )
        };
    }
    pub fn steal_player_controller(
        &mut self,
        player_controller: UPtr<crate::bindings::engine::APlayerController>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_cameras::A_GAMEPLAY_CAMERAS_PLAYER_CAMERA_MANAGER_STEAL_PLAYER_CONTROLLER,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &player_controller,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::engine::APlayerController>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_cameras::A_GAMEPLAY_CAMERAS_PLAYER_CAMERA_MANAGER_STEAL_PLAYER_CONTROLLER,
                __buffer,
            )
        };
    }
    pub fn start_visual_camera_modifier_rig(
        &mut self,
        camera_rig: UPtr<UCameraRigAsset>,
        order_key: i32,
    ) -> FCameraRigInstanceID {
        let mut __stack = crate::core_data::StackAlloc::<20>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_cameras::A_GAMEPLAY_CAMERAS_PLAYER_CAMERA_MANAGER_START_VISUAL_CAMERA_MODIFIER_RIG,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &camera_rig,
                __buffer.add(0).cast::<UPtr<UCameraRigAsset>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&order_key, __buffer.add(8).cast::<i32>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_cameras::A_GAMEPLAY_CAMERAS_PLAYER_CAMERA_MANAGER_START_VISUAL_CAMERA_MODIFIER_RIG,
                __buffer,
            )
        };
        unsafe { __buffer.add(12).cast::<FCameraRigInstanceID>().read() }
    }
    pub fn start_global_camera_modifier_rig(
        &mut self,
        camera_rig: UPtr<UCameraRigAsset>,
        order_key: i32,
    ) -> FCameraRigInstanceID {
        let mut __stack = crate::core_data::StackAlloc::<20>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_cameras::A_GAMEPLAY_CAMERAS_PLAYER_CAMERA_MANAGER_START_GLOBAL_CAMERA_MODIFIER_RIG,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &camera_rig,
                __buffer.add(0).cast::<UPtr<UCameraRigAsset>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&order_key, __buffer.add(8).cast::<i32>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_cameras::A_GAMEPLAY_CAMERAS_PLAYER_CAMERA_MANAGER_START_GLOBAL_CAMERA_MODIFIER_RIG,
                __buffer,
            )
        };
        unsafe { __buffer.add(12).cast::<FCameraRigInstanceID>().read() }
    }
    pub fn start_camera_shake_asset(
        &mut self,
        camera_shake: UPtr<UCameraShakeAsset>,
        shake_scale: f32,
        play_space: crate::bindings::engine::ECameraShakePlaySpace,
        user_play_space_rotation: crate::bindings::core_u_object::FRotator,
    ) -> FCameraShakeInstanceID {
        let mut __stack = crate::core_data::StackAlloc::<44>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_cameras::A_GAMEPLAY_CAMERAS_PLAYER_CAMERA_MANAGER_START_CAMERA_SHAKE_ASSET,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &camera_shake,
                __buffer.add(0).cast::<UPtr<UCameraShakeAsset>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &shake_scale,
                __buffer.add(8).cast::<f32>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &play_space,
                __buffer
                    .add(12)
                    .cast::<crate::bindings::engine::ECameraShakePlaySpace>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &user_play_space_rotation,
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
                crate::bindings::gameplay_cameras::A_GAMEPLAY_CAMERAS_PLAYER_CAMERA_MANAGER_START_CAMERA_SHAKE_ASSET,
                __buffer,
            )
        };
        unsafe { __buffer.add(40).cast::<FCameraShakeInstanceID>().read() }
    }
    pub fn release_player_controller(&mut self) {
        let mut __stack = crate::core_data::StackAlloc::<0>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_cameras::A_GAMEPLAY_CAMERAS_PLAYER_CAMERA_MANAGER_RELEASE_PLAYER_CONTROLLER,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_cameras::A_GAMEPLAY_CAMERAS_PLAYER_CAMERA_MANAGER_RELEASE_PLAYER_CONTROLLER,
                __buffer,
            )
        };
    }
    pub fn is_camera_shake_asset_playing(
        &self,
        in_instance_id: FCameraShakeInstanceID,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<5>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_cameras::A_GAMEPLAY_CAMERAS_PLAYER_CAMERA_MANAGER_IS_CAMERA_SHAKE_ASSET_PLAYING,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_instance_id,
                __buffer.add(0).cast::<FCameraShakeInstanceID>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_cameras::A_GAMEPLAY_CAMERAS_PLAYER_CAMERA_MANAGER_IS_CAMERA_SHAKE_ASSET_PLAYING,
                __buffer,
            )
        };
        unsafe { __buffer.add(4).cast::<bool>().read() }
    }
    pub fn activate_persistent_visual_camera_rig(
        &mut self,
        camera_rig: UPtr<UCameraRigAsset>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_cameras::A_GAMEPLAY_CAMERAS_PLAYER_CAMERA_MANAGER_ACTIVATE_PERSISTENT_VISUAL_CAMERA_RIG,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &camera_rig,
                __buffer.add(0).cast::<UPtr<UCameraRigAsset>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_cameras::A_GAMEPLAY_CAMERAS_PLAYER_CAMERA_MANAGER_ACTIVATE_PERSISTENT_VISUAL_CAMERA_RIG,
                __buffer,
            )
        };
    }
    pub fn activate_persistent_global_camera_rig(
        &mut self,
        camera_rig: UPtr<UCameraRigAsset>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_cameras::A_GAMEPLAY_CAMERAS_PLAYER_CAMERA_MANAGER_ACTIVATE_PERSISTENT_GLOBAL_CAMERA_RIG,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &camera_rig,
                __buffer.add(0).cast::<UPtr<UCameraRigAsset>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_cameras::A_GAMEPLAY_CAMERAS_PLAYER_CAMERA_MANAGER_ACTIVATE_PERSISTENT_GLOBAL_CAMERA_RIG,
                __buffer,
            )
        };
    }
    pub fn activate_persistent_base_camera_rig(
        &mut self,
        camera_rig: UPtr<UCameraRigAsset>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_cameras::A_GAMEPLAY_CAMERAS_PLAYER_CAMERA_MANAGER_ACTIVATE_PERSISTENT_BASE_CAMERA_RIG,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &camera_rig,
                __buffer.add(0).cast::<UPtr<UCameraRigAsset>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_cameras::A_GAMEPLAY_CAMERAS_PLAYER_CAMERA_MANAGER_ACTIVATE_PERSISTENT_BASE_CAMERA_RIG,
                __buffer,
            )
        };
    }
}
#[repr(C, align(8))]
pub struct AGameplayCameraSystemActor {
    #[doc(hidden)]
    __padding_1136: [u8; 1136],
    pub camera_system_component: UPtr<UGameplayCameraSystemComponent>,
}
impl AGameplayCameraSystemActor {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("AGameplayCameraSystemActor")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
    pub fn get_camera_system_component(&self) -> UPtr<UGameplayCameraSystemComponent> {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_cameras::A_GAMEPLAY_CAMERA_SYSTEM_ACTOR_GET_CAMERA_SYSTEM_COMPONENT,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_cameras::A_GAMEPLAY_CAMERA_SYSTEM_ACTOR_GET_CAMERA_SYSTEM_COMPONENT,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<UPtr<UGameplayCameraSystemComponent>>().read() }
    }
}
#[repr(C, align(16))]
pub struct UGameplayCameraSystemComponent {
    #[doc(hidden)]
    __padding_688: [u8; 688],
    pub auto_activate_for_player: crate::bindings::engine::EAutoReceiveInput,
    pub b_set_player_controller_rotation: bool,
}
impl UGameplayCameraSystemComponent {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UGameplayCameraSystemComponent")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
    pub fn stop_camera_modifier_rig(
        &mut self,
        instance_id: FCameraRigInstanceID,
        b_immediately: bool,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<9>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_cameras::U_GAMEPLAY_CAMERA_SYSTEM_COMPONENT_STOP_CAMERA_MODIFIER_RIG,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &instance_id,
                __buffer.add(0).cast::<FCameraRigInstanceID>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_immediately,
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
                crate::bindings::gameplay_cameras::U_GAMEPLAY_CAMERA_SYSTEM_COMPONENT_STOP_CAMERA_MODIFIER_RIG,
                __buffer,
            )
        };
    }
    pub fn start_visual_camera_modifier_rig(
        &mut self,
        camera_rig: UPtr<UCameraRigAsset>,
        order_key: i32,
    ) -> FCameraRigInstanceID {
        let mut __stack = crate::core_data::StackAlloc::<20>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_cameras::U_GAMEPLAY_CAMERA_SYSTEM_COMPONENT_START_VISUAL_CAMERA_MODIFIER_RIG,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &camera_rig,
                __buffer.add(0).cast::<UPtr<UCameraRigAsset>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&order_key, __buffer.add(8).cast::<i32>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_cameras::U_GAMEPLAY_CAMERA_SYSTEM_COMPONENT_START_VISUAL_CAMERA_MODIFIER_RIG,
                __buffer,
            )
        };
        unsafe { __buffer.add(12).cast::<FCameraRigInstanceID>().read() }
    }
    pub fn start_global_camera_modifier_rig(
        &mut self,
        camera_rig: UPtr<UCameraRigAsset>,
        order_key: i32,
    ) -> FCameraRigInstanceID {
        let mut __stack = crate::core_data::StackAlloc::<20>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_cameras::U_GAMEPLAY_CAMERA_SYSTEM_COMPONENT_START_GLOBAL_CAMERA_MODIFIER_RIG,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &camera_rig,
                __buffer.add(0).cast::<UPtr<UCameraRigAsset>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&order_key, __buffer.add(8).cast::<i32>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_cameras::U_GAMEPLAY_CAMERA_SYSTEM_COMPONENT_START_GLOBAL_CAMERA_MODIFIER_RIG,
                __buffer,
            )
        };
        unsafe { __buffer.add(12).cast::<FCameraRigInstanceID>().read() }
    }
    pub fn is_camera_system_active_for_play_controller(
        &self,
        player_controller: UPtr<crate::bindings::engine::APlayerController>,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<9>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_cameras::U_GAMEPLAY_CAMERA_SYSTEM_COMPONENT_IS_CAMERA_SYSTEM_ACTIVE_FOR_PLAY_CONTROLLER,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &player_controller,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::engine::APlayerController>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_cameras::U_GAMEPLAY_CAMERA_SYSTEM_COMPONENT_IS_CAMERA_SYSTEM_ACTIVE_FOR_PLAY_CONTROLLER,
                __buffer,
            )
        };
        unsafe { __buffer.add(8).cast::<bool>().read() }
    }
    pub fn deactivate_camera_system(
        &mut self,
        next_view_target: UPtr<crate::bindings::engine::AActor>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_cameras::U_GAMEPLAY_CAMERA_SYSTEM_COMPONENT_DEACTIVATE_CAMERA_SYSTEM,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &next_view_target,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::AActor>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_cameras::U_GAMEPLAY_CAMERA_SYSTEM_COMPONENT_DEACTIVATE_CAMERA_SYSTEM,
                __buffer,
            )
        };
    }
    pub fn activate_camera_system_for_player_index(&mut self, player_index: i32) {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_cameras::U_GAMEPLAY_CAMERA_SYSTEM_COMPONENT_ACTIVATE_CAMERA_SYSTEM_FOR_PLAYER_INDEX,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &player_index,
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
                crate::bindings::gameplay_cameras::U_GAMEPLAY_CAMERA_SYSTEM_COMPONENT_ACTIVATE_CAMERA_SYSTEM_FOR_PLAYER_INDEX,
                __buffer,
            )
        };
    }
    pub fn activate_camera_system_for_player_controller(
        &mut self,
        player_controller: UPtr<crate::bindings::engine::APlayerController>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_cameras::U_GAMEPLAY_CAMERA_SYSTEM_COMPONENT_ACTIVATE_CAMERA_SYSTEM_FOR_PLAYER_CONTROLLER,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &player_controller,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::engine::APlayerController>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_cameras::U_GAMEPLAY_CAMERA_SYSTEM_COMPONENT_ACTIVATE_CAMERA_SYSTEM_FOR_PLAYER_CONTROLLER,
                __buffer,
            )
        };
    }
}
#[repr(C, align(8))]
pub struct UGameplayControlRotationComponent {
    #[doc(hidden)]
    __padding_264: [u8; 264],
    pub auto_activate_for_player: crate::bindings::engine::EAutoReceiveInput,
    __padding_end: [u8; 47],
}
impl UGameplayControlRotationComponent {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UGameplayControlRotationComponent")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
    pub fn deactivate_control_rotation_management(&mut self) {
        let mut __stack = crate::core_data::StackAlloc::<0>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_cameras::U_GAMEPLAY_CONTROL_ROTATION_COMPONENT_DEACTIVATE_CONTROL_ROTATION_MANAGEMENT,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_cameras::U_GAMEPLAY_CONTROL_ROTATION_COMPONENT_DEACTIVATE_CONTROL_ROTATION_MANAGEMENT,
                __buffer,
            )
        };
    }
    pub fn activate_control_rotation_management_for_player_index(
        &mut self,
        player_index: i32,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_cameras::U_GAMEPLAY_CONTROL_ROTATION_COMPONENT_ACTIVATE_CONTROL_ROTATION_MANAGEMENT_FOR_PLAYER_INDEX,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &player_index,
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
                crate::bindings::gameplay_cameras::U_GAMEPLAY_CONTROL_ROTATION_COMPONENT_ACTIVATE_CONTROL_ROTATION_MANAGEMENT_FOR_PLAYER_INDEX,
                __buffer,
            )
        };
    }
    pub fn activate_control_rotation_management_for_player_controller(
        &mut self,
        player_controller: UPtr<crate::bindings::engine::APlayerController>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_cameras::U_GAMEPLAY_CONTROL_ROTATION_COMPONENT_ACTIVATE_CONTROL_ROTATION_MANAGEMENT_FOR_PLAYER_CONTROLLER,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &player_controller,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::engine::APlayerController>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_cameras::U_GAMEPLAY_CONTROL_ROTATION_COMPONENT_ACTIVATE_CONTROL_ROTATION_MANAGEMENT_FOR_PLAYER_CONTROLLER,
                __buffer,
            )
        };
    }
}
pub struct IGameplayCameraSystemHost {}
#[repr(C, align(8))]
pub struct UGameplayCameraSystemHost {
    __padding_end: [u8; 48],
}
impl UGameplayCameraSystemHost {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UGameplayCameraSystemHost")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct USimpleBlendCameraNode {
    __padding_end: [u8; 104],
}
impl USimpleBlendCameraNode {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("USimpleBlendCameraNode")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UViewTargetTransitionParamsBlendCameraNode {
    __padding_end: [u8; 120],
}
impl UViewTargetTransitionParamsBlendCameraNode {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UViewTargetTransitionParamsBlendCameraNode")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UGameplayCamerasSettings {
    __padding_end: [u8; 152],
}
impl UGameplayCamerasSettings {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UGameplayCamerasSettings")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UMovieSceneCameraFramingZonePropertySystem {
    __padding_end: [u8; 104],
}
impl UMovieSceneCameraFramingZonePropertySystem {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieSceneCameraFramingZonePropertySystem")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UMovieSceneCameraFramingZoneSection {
    __padding_end: [u8; 1616],
}
impl UMovieSceneCameraFramingZoneSection {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieSceneCameraFramingZoneSection")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UMovieSceneCameraFramingZoneTrack {
    __padding_end: [u8; 480],
}
impl UMovieSceneCameraFramingZoneTrack {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieSceneCameraFramingZoneTrack")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UAttachToActorCameraNode {
    __padding_end: [u8; 184],
}
impl UAttachToActorCameraNode {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAttachToActorCameraNode")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UAttachToActorGroupCameraNode {
    __padding_end: [u8; 128],
}
impl UAttachToActorGroupCameraNode {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAttachToActorGroupCameraNode")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UAttachToPlayerPawnCameraNode {
    #[doc(hidden)]
    __padding_136: [u8; 136],
    pub socket_name: FName,
    pub bone_name: FName,
}
impl UAttachToPlayerPawnCameraNode {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAttachToPlayerPawnCameraNode")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct USimpleFixedTimeBlendCameraNode {
    __padding_end: [u8; 120],
}
impl USimpleFixedTimeBlendCameraNode {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("USimpleFixedTimeBlendCameraNode")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct ULinearBlendCameraNode {
    __padding_end: [u8; 120],
}
impl ULinearBlendCameraNode {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("ULinearBlendCameraNode")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct ULocationRotationBlendCameraNode {
    __padding_end: [u8; 128],
}
impl ULocationRotationBlendCameraNode {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("ULocationRotationBlendCameraNode")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UOrbitBlendCameraNode {
    __padding_end: [u8; 112],
}
impl UOrbitBlendCameraNode {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UOrbitBlendCameraNode")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UPopBlendCameraNode {
    __padding_end: [u8; 104],
}
impl UPopBlendCameraNode {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPopBlendCameraNode")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct USmoothBlendCameraNode {
    __padding_end: [u8; 128],
}
impl USmoothBlendCameraNode {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("USmoothBlendCameraNode")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UCollisionPushCameraNode {
    __padding_end: [u8; 240],
}
impl UCollisionPushCameraNode {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UCollisionPushCameraNode")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UOcclusionMaterialCameraNode {
    __padding_end: [u8; 176],
}
impl UOcclusionMaterialCameraNode {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UOcclusionMaterialCameraNode")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UArrayCameraNode {
    __padding_end: [u8; 120],
}
impl UArrayCameraNode {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UArrayCameraNode")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UAutoFocusCameraNode {
    __padding_end: [u8; 136],
}
impl UAutoFocusCameraNode {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAutoFocusCameraNode")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UBodyParametersCameraNode {
    __padding_end: [u8; 136],
}
impl UBodyParametersCameraNode {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBodyParametersCameraNode")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UBoomArmCameraNode {
    __padding_end: [u8; 208],
}
impl UBoomArmCameraNode {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBoomArmCameraNode")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UCameraRigCameraNode {
    __padding_end: [u8; 424],
}
impl UCameraRigCameraNode {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UCameraRigCameraNode")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UClippingPlanesCameraNode {
    __padding_end: [u8; 152],
}
impl UClippingPlanesCameraNode {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UClippingPlanesCameraNode")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UDampenPositionCameraNode {
    __padding_end: [u8; 160],
}
impl UDampenPositionCameraNode {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDampenPositionCameraNode")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UDampenRotationCameraNode {
    __padding_end: [u8; 152],
}
impl UDampenRotationCameraNode {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDampenRotationCameraNode")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UFieldOfViewCameraNode {
    __padding_end: [u8; 120],
}
impl UFieldOfViewCameraNode {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UFieldOfViewCameraNode")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UFilmbackCameraNode {
    __padding_end: [u8; 224],
}
impl UFilmbackCameraNode {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UFilmbackCameraNode")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct ULensParametersCameraNode {
    __padding_end: [u8; 168],
}
impl ULensParametersCameraNode {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("ULensParametersCameraNode")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UOffsetCameraNode {
    __padding_end: [u8; 192],
}
impl UOffsetCameraNode {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UOffsetCameraNode")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UOrthographicCameraNode {
    __padding_end: [u8; 136],
}
impl UOrthographicCameraNode {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UOrthographicCameraNode")
            .unwrap()
    }
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
pub struct UPostProcessCameraNode {
    __padding_end: [u8; 2064],
}
impl UPostProcessCameraNode {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPostProcessCameraNode")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct USetLocationCameraNode {
    __padding_end: [u8; 152],
}
impl USetLocationCameraNode {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("USetLocationCameraNode")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct USetRotationCameraNode {
    __padding_end: [u8; 152],
}
impl USetRotationCameraNode {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("USetRotationCameraNode")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct USplineFieldOfViewCameraNode {
    __padding_end: [u8; 248],
}
impl USplineFieldOfViewCameraNode {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("USplineFieldOfViewCameraNode")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct USplineOffsetCameraNode {
    __padding_end: [u8; 896],
}
impl USplineOffsetCameraNode {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("USplineOffsetCameraNode")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct USplineOrbitCameraNode {
    __padding_end: [u8; 1288],
}
impl USplineOrbitCameraNode {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("USplineOrbitCameraNode")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UTargetRayCastCameraNode {
    __padding_end: [u8; 128],
}
impl UTargetRayCastCameraNode {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UTargetRayCastCameraNode")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UBaseFramingCameraNode {
    __padding_end: [u8; 424],
}
impl UBaseFramingCameraNode {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBaseFramingCameraNode")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UDollyFramingCameraNode {
    __padding_end: [u8; 456],
}
impl UDollyFramingCameraNode {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDollyFramingCameraNode")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UPanningFramingCameraNode {
    __padding_end: [u8; 456],
}
impl UPanningFramingCameraNode {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPanningFramingCameraNode")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UInput2DCameraNode {
    __padding_end: [u8; 104],
}
impl UInput2DCameraNode {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UInput2DCameraNode")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UAutoRotateInput2DCameraNode {
    __padding_end: [u8; 240],
}
impl UAutoRotateInput2DCameraNode {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAutoRotateInput2DCameraNode")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UInput1DCameraNode {
    __padding_end: [u8; 104],
}
impl UInput1DCameraNode {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UInput1DCameraNode")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UCameraRigInput1DSlot {
    __padding_end: [u8; 224],
}
impl UCameraRigInput1DSlot {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UCameraRigInput1DSlot")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UCameraRigInput2DSlot {
    __padding_end: [u8; 288],
}
impl UCameraRigInput2DSlot {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UCameraRigInput2DSlot")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UDrivenControlRotationCameraNode {
    __padding_end: [u8; 104],
}
impl UDrivenControlRotationCameraNode {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDrivenControlRotationCameraNode")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UInputAccumulator2DCameraNode {
    __padding_end: [u8; 296],
}
impl UInputAccumulator2DCameraNode {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UInputAccumulator2DCameraNode")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UInputAxisBinding2DCameraNode {
    __padding_end: [u8; 344],
}
impl UInputAxisBinding2DCameraNode {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UInputAxisBinding2DCameraNode")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct URawInputAxisBinding2DCameraNode {
    __padding_end: [u8; 152],
}
impl URawInputAxisBinding2DCameraNode {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("URawInputAxisBinding2DCameraNode")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UCameraShakeCameraNode {
    __padding_end: [u8; 184],
}
impl UCameraShakeCameraNode {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UCameraShakeCameraNode")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UCompositeShakeCameraNode {
    __padding_end: [u8; 120],
}
impl UCompositeShakeCameraNode {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UCompositeShakeCameraNode")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UEnvelopeShakeCameraNode {
    __padding_end: [u8; 160],
}
impl UEnvelopeShakeCameraNode {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEnvelopeShakeCameraNode")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UPerlinNoiseLocationShakeCameraNode {
    __padding_end: [u8; 176],
}
impl UPerlinNoiseLocationShakeCameraNode {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPerlinNoiseLocationShakeCameraNode")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UPerlinNoiseRotationShakeCameraNode {
    __padding_end: [u8; 176],
}
impl UPerlinNoiseRotationShakeCameraNode {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPerlinNoiseRotationShakeCameraNode")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UBlueprintCameraNodeEvaluator {
    #[doc(hidden)]
    __padding_48: [u8; 48],
    pub b_is_first_frame: bool,
    pub b_is_active_camera_rig: bool,
    pub evaluation_context_owner: UPtr<crate::bindings::core_u_object::UObject>,
    pub camera_data: FBlueprintCameraEvaluationDataRef,
    pub camera_pose: FBlueprintCameraPose,
    pub variable_table: FBlueprintCameraEvaluationDataRef,
    __padding_end: [u8; 32],
}
impl UBlueprintCameraNodeEvaluator {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBlueprintCameraNodeEvaluator")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
    pub fn set_default_owning_camera_rig_parameters(
        &self,
        target_camera_data: FBlueprintCameraEvaluationDataRef,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_cameras::U_BLUEPRINT_CAMERA_NODE_EVALUATOR_SET_DEFAULT_OWNING_CAMERA_RIG_PARAMETERS,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &target_camera_data,
                __buffer.add(0).cast::<FBlueprintCameraEvaluationDataRef>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_cameras::U_BLUEPRINT_CAMERA_NODE_EVALUATOR_SET_DEFAULT_OWNING_CAMERA_RIG_PARAMETERS,
                __buffer,
            )
        };
    }
    pub fn set_current_camera_pose(&mut self, camera_pose: &FBlueprintCameraPose) {
        let mut __stack = crate::core_data::StackAlloc::<136>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_cameras::U_BLUEPRINT_CAMERA_NODE_EVALUATOR_SET_CURRENT_CAMERA_POSE,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                camera_pose,
                __buffer.add(0).cast::<FBlueprintCameraPose>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_cameras::U_BLUEPRINT_CAMERA_NODE_EVALUATOR_SET_CURRENT_CAMERA_POSE,
                __buffer,
            )
        };
    }
    pub fn set_camera_pose(&mut self, in_camera_pose: &FBlueprintCameraPose) {
        let mut __stack = crate::core_data::StackAlloc::<136>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_cameras::U_BLUEPRINT_CAMERA_NODE_EVALUATOR_SET_CAMERA_POSE,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_camera_pose,
                __buffer.add(0).cast::<FBlueprintCameraPose>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_cameras::U_BLUEPRINT_CAMERA_NODE_EVALUATOR_SET_CAMERA_POSE,
                __buffer,
            )
        };
    }
    pub fn get_player_controller(
        &self,
    ) -> UPtr<crate::bindings::engine::APlayerController> {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_cameras::U_BLUEPRINT_CAMERA_NODE_EVALUATOR_GET_PLAYER_CONTROLLER,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_cameras::U_BLUEPRINT_CAMERA_NODE_EVALUATOR_GET_PLAYER_CONTROLLER,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(0)
                .cast::<UPtr<crate::bindings::engine::APlayerController>>()
                .read()
        }
    }
    pub fn get_current_camera_pose(&self) -> FBlueprintCameraPose {
        let mut __stack = crate::core_data::StackAlloc::<136>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_cameras::U_BLUEPRINT_CAMERA_NODE_EVALUATOR_GET_CURRENT_CAMERA_POSE,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_cameras::U_BLUEPRINT_CAMERA_NODE_EVALUATOR_GET_CURRENT_CAMERA_POSE,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<FBlueprintCameraPose>().read() }
    }
    pub fn get_camera_pose(&self) -> FBlueprintCameraPose {
        let mut __stack = crate::core_data::StackAlloc::<136>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_cameras::U_BLUEPRINT_CAMERA_NODE_EVALUATOR_GET_CAMERA_POSE,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_cameras::U_BLUEPRINT_CAMERA_NODE_EVALUATOR_GET_CAMERA_POSE,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<FBlueprintCameraPose>().read() }
    }
    pub fn find_evaluation_context_owner_actor(
        &self,
        actor_class: TSubclassOf<crate::bindings::engine::AActor>,
    ) -> UPtr<crate::bindings::engine::AActor> {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_cameras::U_BLUEPRINT_CAMERA_NODE_EVALUATOR_FIND_EVALUATION_CONTEXT_OWNER_ACTOR,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &actor_class,
                __buffer.add(0).cast::<TSubclassOf<crate::bindings::engine::AActor>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_cameras::U_BLUEPRINT_CAMERA_NODE_EVALUATOR_FIND_EVALUATION_CONTEXT_OWNER_ACTOR,
                __buffer,
            )
        };
        unsafe { __buffer.add(8).cast::<UPtr<crate::bindings::engine::AActor>>().read() }
    }
}
#[repr(C, align(8))]
pub struct UBlueprintCameraNode {
    __padding_end: [u8; 160],
}
impl UBlueprintCameraNode {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBlueprintCameraNode")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UCameraShakeServiceCameraNode {
    __padding_end: [u8; 104],
}
impl UCameraShakeServiceCameraNode {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UCameraShakeServiceCameraNode")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UUpdateTrackerCameraNode {
    __padding_end: [u8; 168],
}
impl UUpdateTrackerCameraNode {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UUpdateTrackerCameraNode")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UFixedTestCameraDirector {
    __padding_end: [u8; 104],
}
impl UFixedTestCameraDirector {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UFixedTestCameraDirector")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UIsCameraRigTransitionCondition {
    __padding_end: [u8; 104],
}
impl UIsCameraRigTransitionCondition {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UIsCameraRigTransitionCondition")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UGameplayTagTransitionCondition {
    __padding_end: [u8; 232],
}
impl UGameplayTagTransitionCondition {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UGameplayTagTransitionCondition")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UAccelerationDecelerationValueInterpolator {
    __padding_end: [u8; 64],
}
impl UAccelerationDecelerationValueInterpolator {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAccelerationDecelerationValueInterpolator")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UCriticalDamperValueInterpolator {
    __padding_end: [u8; 56],
}
impl UCriticalDamperValueInterpolator {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UCriticalDamperValueInterpolator")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UDoubleIIRValueInterpolator {
    __padding_end: [u8; 64],
}
impl UDoubleIIRValueInterpolator {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDoubleIIRValueInterpolator")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UIIRValueInterpolator {
    __padding_end: [u8; 56],
}
impl UIIRValueInterpolator {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UIIRValueInterpolator")
            .unwrap()
    }
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
pub struct ECameraObjectInterfaceParameterType(pub u8);
impl ECameraObjectInterfaceParameterType {
    pub const BLENDABLE: ECameraObjectInterfaceParameterType = ECameraObjectInterfaceParameterType(
        0,
    );
    pub const DATA: ECameraObjectInterfaceParameterType = ECameraObjectInterfaceParameterType(
        1,
    );
}
#[repr(transparent)]
pub struct ECameraVariableType(pub i32);
impl ECameraVariableType {
    pub const BOOLEAN: ECameraVariableType = ECameraVariableType(0);
    pub const INTEGER32: ECameraVariableType = ECameraVariableType(1);
    pub const FLOAT: ECameraVariableType = ECameraVariableType(2);
    pub const DOUBLE: ECameraVariableType = ECameraVariableType(3);
    pub const VECTOR2F: ECameraVariableType = ECameraVariableType(4);
    pub const VECTOR2D: ECameraVariableType = ECameraVariableType(5);
    pub const VECTOR3F: ECameraVariableType = ECameraVariableType(6);
    pub const VECTOR3D: ECameraVariableType = ECameraVariableType(7);
    pub const VECTOR4F: ECameraVariableType = ECameraVariableType(8);
    pub const VECTOR4D: ECameraVariableType = ECameraVariableType(9);
    pub const ROTATOR3F: ECameraVariableType = ECameraVariableType(10);
    pub const ROTATOR3D: ECameraVariableType = ECameraVariableType(11);
    pub const TRANSFORM3F: ECameraVariableType = ECameraVariableType(12);
    pub const TRANSFORM3D: ECameraVariableType = ECameraVariableType(13);
    pub const BLENDABLE_STRUCT: ECameraVariableType = ECameraVariableType(14);
}
#[repr(transparent)]
pub struct ECameraContextDataType(pub i32);
impl ECameraContextDataType {
    pub const NAME: ECameraContextDataType = ECameraContextDataType(0);
    pub const STRING: ECameraContextDataType = ECameraContextDataType(1);
    pub const ENUM: ECameraContextDataType = ECameraContextDataType(2);
    pub const STRUCT: ECameraContextDataType = ECameraContextDataType(3);
    pub const OBJECT: ECameraContextDataType = ECameraContextDataType(4);
    pub const CLASS: ECameraContextDataType = ECameraContextDataType(5);
    pub const COUNT: ECameraContextDataType = ECameraContextDataType(6);
}
#[repr(transparent)]
pub struct ECameraContextDataContainerType(pub i32);
impl ECameraContextDataContainerType {
    pub const NONE: ECameraContextDataContainerType = ECameraContextDataContainerType(0);
    pub const ARRAY: ECameraContextDataContainerType = ECameraContextDataContainerType(
        1,
    );
}
#[repr(transparent)]
pub struct ECameraRigLayer(pub u8);
impl ECameraRigLayer {
    pub const NONE: ECameraRigLayer = ECameraRigLayer(0);
    pub const BASE: ECameraRigLayer = ECameraRigLayer(1);
    pub const MAIN: ECameraRigLayer = ECameraRigLayer(2);
    pub const GLOBAL: ECameraRigLayer = ECameraRigLayer(3);
    pub const VISUAL: ECameraRigLayer = ECameraRigLayer(4);
}
#[repr(transparent)]
pub struct ECameraTargetShape(pub u8);
impl ECameraTargetShape {
    pub const POINT: ECameraTargetShape = ECameraTargetShape(0);
    pub const AUTOMATIC_BOUNDS: ECameraTargetShape = ECameraTargetShape(1);
    pub const MANUAL_BOUNDS: ECameraTargetShape = ECameraTargetShape(2);
}
#[repr(transparent)]
pub struct ECameraEvaluationDataCondition(pub u8);
impl ECameraEvaluationDataCondition {
    pub const ACTIVE_CAMERA_RIG: ECameraEvaluationDataCondition = ECameraEvaluationDataCondition(
        0,
    );
}
#[repr(transparent)]
pub struct EGameplayCameraComponentActivationMode(pub u8);
impl EGameplayCameraComponentActivationMode {
    pub const PUSH: EGameplayCameraComponentActivationMode = EGameplayCameraComponentActivationMode(
        0,
    );
    pub const PUSH_AND_INSERT: EGameplayCameraComponentActivationMode = EGameplayCameraComponentActivationMode(
        1,
    );
    pub const INSERT_OR_PUSH: EGameplayCameraComponentActivationMode = EGameplayCameraComponentActivationMode(
        2,
    );
}
#[repr(transparent)]
pub struct ECameraBlendStackType(pub i32);
impl ECameraBlendStackType {
    pub const ISOLATED_TRANSIENT: ECameraBlendStackType = ECameraBlendStackType(0);
    pub const ADDITIVE_PERSISTENT: ECameraBlendStackType = ECameraBlendStackType(1);
}
#[repr(transparent)]
pub struct ECameraBuildStatus(pub u8);
impl ECameraBuildStatus {
    pub const CLEAN: ECameraBuildStatus = ECameraBuildStatus(0);
    pub const CLEAN_WITH_WARNINGS: ECameraBuildStatus = ECameraBuildStatus(1);
    pub const WITH_ERRORS: ECameraBuildStatus = ECameraBuildStatus(2);
    pub const DIRTY: ECameraBuildStatus = ECameraBuildStatus(3);
}
#[repr(transparent)]
pub struct ECameraRigInitialOrientation(pub i32);
impl ECameraRigInitialOrientation {
    pub const NONE: ECameraRigInitialOrientation = ECameraRigInitialOrientation(0);
    pub const CONTEXT_YAW_PITCH: ECameraRigInitialOrientation = ECameraRigInitialOrientation(
        1,
    );
    pub const PREVIOUS_YAW_PITCH: ECameraRigInitialOrientation = ECameraRigInitialOrientation(
        2,
    );
    pub const PREVIOUS_ABSOLUTE_TARGET: ECameraRigInitialOrientation = ECameraRigInitialOrientation(
        3,
    );
    pub const PREVIOUS_RELATIVE_TARGET: ECameraRigInitialOrientation = ECameraRigInitialOrientation(
        4,
    );
}
#[repr(transparent)]
pub struct ECameraVariableSetterBlendType(pub u8);
impl ECameraVariableSetterBlendType {
    pub const NONE: ECameraVariableSetterBlendType = ECameraVariableSetterBlendType(0);
    pub const LINEAR: ECameraVariableSetterBlendType = ECameraVariableSetterBlendType(1);
    pub const SMOOTH_STEP: ECameraVariableSetterBlendType = ECameraVariableSetterBlendType(
        2,
    );
    pub const SMOOTHER_STEP: ECameraVariableSetterBlendType = ECameraVariableSetterBlendType(
        3,
    );
}
#[repr(transparent)]
pub struct EGameplayCamerasViewRotationMode(pub i32);
impl EGameplayCamerasViewRotationMode {
    pub const NONE: EGameplayCamerasViewRotationMode = EGameplayCamerasViewRotationMode(
        0,
    );
    pub const PREVIEW_UPDATE: EGameplayCamerasViewRotationMode = EGameplayCamerasViewRotationMode(
        1,
    );
}
#[repr(transparent)]
pub struct ESmoothCameraBlendType(pub i32);
impl ESmoothCameraBlendType {
    pub const SMOOTH_STEP: ESmoothCameraBlendType = ESmoothCameraBlendType(0);
    pub const SMOOTHER_STEP: ESmoothCameraBlendType = ESmoothCameraBlendType(1);
}
#[repr(transparent)]
pub struct ECollisionSafePosition(pub u8);
impl ECollisionSafePosition {
    pub const ACTIVE_CONTEXT: ECollisionSafePosition = ECollisionSafePosition(0);
    pub const OWNING_CONTEXT: ECollisionSafePosition = ECollisionSafePosition(1);
    pub const PIVOT: ECollisionSafePosition = ECollisionSafePosition(2);
    pub const PAWN: ECollisionSafePosition = ECollisionSafePosition(3);
}
#[repr(transparent)]
pub struct ECollisionSafePositionOffsetSpace(pub u8);
impl ECollisionSafePositionOffsetSpace {
    pub const ACTIVE_CONTEXT: ECollisionSafePositionOffsetSpace = ECollisionSafePositionOffsetSpace(
        0,
    );
    pub const OWNING_CONTEXT: ECollisionSafePositionOffsetSpace = ECollisionSafePositionOffsetSpace(
        1,
    );
    pub const PIVOT: ECollisionSafePositionOffsetSpace = ECollisionSafePositionOffsetSpace(
        2,
    );
    pub const CAMERA_POSE: ECollisionSafePositionOffsetSpace = ECollisionSafePositionOffsetSpace(
        3,
    );
    pub const PAWN: ECollisionSafePositionOffsetSpace = ECollisionSafePositionOffsetSpace(
        4,
    );
}
#[repr(transparent)]
pub struct ECameraNodeOriginPosition(pub u8);
impl ECameraNodeOriginPosition {
    pub const CAMERA_POSE: ECameraNodeOriginPosition = ECameraNodeOriginPosition(0);
    pub const ACTIVE_CONTEXT: ECameraNodeOriginPosition = ECameraNodeOriginPosition(1);
    pub const OWNING_CONTEXT: ECameraNodeOriginPosition = ECameraNodeOriginPosition(2);
    pub const PIVOT: ECameraNodeOriginPosition = ECameraNodeOriginPosition(3);
    pub const PAWN: ECameraNodeOriginPosition = ECameraNodeOriginPosition(4);
}
#[repr(transparent)]
pub struct ECameraNodeSpace(pub u8);
impl ECameraNodeSpace {
    pub const CAMERA_POSE: ECameraNodeSpace = ECameraNodeSpace(0);
    pub const ACTIVE_CONTEXT: ECameraNodeSpace = ECameraNodeSpace(1);
    pub const OWNING_CONTEXT: ECameraNodeSpace = ECameraNodeSpace(2);
    pub const PIVOT: ECameraNodeSpace = ECameraNodeSpace(3);
    pub const PAWN: ECameraNodeSpace = ECameraNodeSpace(4);
    pub const WORLD: ECameraNodeSpace = ECameraNodeSpace(5);
}
#[repr(transparent)]
pub struct ECameraAutoRotateDirection(pub i32);
impl ECameraAutoRotateDirection {
    pub const FACING: ECameraAutoRotateDirection = ECameraAutoRotateDirection(0);
    pub const MOVEMENT: ECameraAutoRotateDirection = ECameraAutoRotateDirection(1);
    pub const MOVEMENT_OR_FACING: ECameraAutoRotateDirection = ECameraAutoRotateDirection(
        2,
    );
}
#[repr(transparent)]
pub struct EBuiltInDoubleCameraVariable(pub i32);
impl EBuiltInDoubleCameraVariable {
    pub const NONE: EBuiltInDoubleCameraVariable = EBuiltInDoubleCameraVariable(0);
    pub const YAW: EBuiltInDoubleCameraVariable = EBuiltInDoubleCameraVariable(1);
    pub const PITCH: EBuiltInDoubleCameraVariable = EBuiltInDoubleCameraVariable(2);
    pub const ROLL: EBuiltInDoubleCameraVariable = EBuiltInDoubleCameraVariable(3);
    pub const ZOOM: EBuiltInDoubleCameraVariable = EBuiltInDoubleCameraVariable(4);
}
#[repr(transparent)]
pub struct EBuiltInVector2dCameraVariable(pub i32);
impl EBuiltInVector2dCameraVariable {
    pub const NONE: EBuiltInVector2dCameraVariable = EBuiltInVector2dCameraVariable(0);
    pub const YAW_PITCH: EBuiltInVector2dCameraVariable = EBuiltInVector2dCameraVariable(
        1,
    );
}
#[repr(transparent)]
pub struct ECameraShakeEvaluationMode(pub u8);
impl ECameraShakeEvaluationMode {
    pub const INLINE: ECameraShakeEvaluationMode = ECameraShakeEvaluationMode(0);
    pub const VISUAL_LAYER: ECameraShakeEvaluationMode = ECameraShakeEvaluationMode(1);
}
