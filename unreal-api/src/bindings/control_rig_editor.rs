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
pub static mut UAIE_SELECTION_SETS_SHOW_OR_HIDE_CONTROLS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut UAIE_SELECTION_SETS_SHOW_ALL_CONTROLS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut UAIE_SELECTION_SETS_SET_SHOW_AND_SET_SELECTED_ONLY: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut UAIE_SELECTION_SETS_SET_ITEM_ROW: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut UAIE_SELECTION_SETS_SET_ITEM_COLOR: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut UAIE_SELECTION_SETS_SET_ACTOR_AS_ACTIVE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut UAIE_SELECTION_SETS_SELECT_ITEM: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut UAIE_SELECTION_SETS_RENAME_SET_ITEM: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut UAIE_SELECTION_SETS_REMOVE_SELECTION_FROM_SET_ITEM: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut UAIE_SELECTION_SETS_LOAD_FROM_JSON_STRING: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut UAIE_SELECTION_SETS_LOAD_FROM_JSON_FILE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut UAIE_SELECTION_SETS_KEY_ALL: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut UAIE_SELECTION_SETS_ISOLATE_CONTROLS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut UAIE_SELECTION_SETS_IS_MULTI_ASSET: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut UAIE_SELECTION_SETS_GET_SHOW_AND_SET_SELECTED_ONLY: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut UAIE_SELECTION_SETS_GET_ITEM_ROW: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut UAIE_SELECTION_SETS_GET_ITEM_NAME: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut UAIE_SELECTION_SETS_GET_ITEM_GUIDS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut UAIE_SELECTION_SETS_GET_ITEM_COLOR: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut UAIE_SELECTION_SETS_GET_ALL_ACTORS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut UAIE_SELECTION_SETS_GET_ACTIVE_SELECTION_SETS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut UAIE_SELECTION_SETS_GET_ACTIVE_ACTORS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut UAIE_SELECTION_SETS_EXPORT_AS_JSON_STRING: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut UAIE_SELECTION_SETS_EXPORT_AS_JSON_FILE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut UAIE_SELECTION_SETS_DELETE_SET_ITEM: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut UAIE_SELECTION_SETS_CREATE_SET_ITEM_FROM_SELECTION: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut UAIE_SELECTION_SETS_CREATE_MIRROR: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut UAIE_SELECTION_SETS_ADD_SELECTION_TO_SET_ITEM: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_CONTROL_RIG_BLUEPRINT_EDITOR_LIBRARY_SETUP_ALL_EDITOR_MENUS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_CONTROL_RIG_BLUEPRINT_EDITOR_LIBRARY_SET_PREVIEW_MESH: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_CONTROL_RIG_BLUEPRINT_EDITOR_LIBRARY_REQUEST_CONTROL_RIG_INIT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_CONTROL_RIG_BLUEPRINT_EDITOR_LIBRARY_GET_PREVIEW_MESH: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_CONTROL_RIG_BLUEPRINT_EDITOR_LIBRARY_GET_HIERARCHY_CONTROLLER: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_CONTROL_RIG_BLUEPRINT_EDITOR_LIBRARY_GET_HIERARCHY: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_CONTROL_RIG_BLUEPRINT_EDITOR_LIBRARY_GET_CURRENTLY_OPEN_RIG_BLUEPRINTS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_CONTROL_RIG_BLUEPRINT_EDITOR_LIBRARY_GET_AVAILABLE_RIG_UNITS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_CONTROL_RIG_BLUEPRINT_EDITOR_LIBRARY_GET_AVAILABLE_RIG_MODULES: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_CONTROL_RIG_BLUEPRINT_EDITOR_LIBRARY_CAST_TO_CONTROL_RIG_BLUEPRINT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_CONTROL_RIG_BLUEPRINT_FACTORY_CREATE_NEW_CONTROL_RIG_ASSET: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_CONTROL_RIG_BLUEPRINT_FACTORY_CREATE_CONTROL_RIG_FROM_SKELETAL_MESH_OR_SKELETON: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_TWEEN_CONTROL_RIG: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_SPACE_COMPENSATE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_SNAP_CONTROL_RIG: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_SMART_REDUCE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_SHOW_ALL_CONTROLS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_SET_LOCAL_CONTROL_RIG_VECTOR2_DS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_SET_LOCAL_CONTROL_RIG_VECTOR2_D: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_SET_LOCAL_CONTROL_RIG_TRANSFORMS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_SET_LOCAL_CONTROL_RIG_TRANSFORM_NO_SCALES: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_SET_LOCAL_CONTROL_RIG_TRANSFORM_NO_SCALE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_SET_LOCAL_CONTROL_RIG_TRANSFORM: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_SET_LOCAL_CONTROL_RIG_SCALES: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_SET_LOCAL_CONTROL_RIG_SCALE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_SET_LOCAL_CONTROL_RIG_ROTATORS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_SET_LOCAL_CONTROL_RIG_ROTATOR: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_SET_LOCAL_CONTROL_RIG_POSITIONS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_SET_LOCAL_CONTROL_RIG_POSITION: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_SET_LOCAL_CONTROL_RIG_INTS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_SET_LOCAL_CONTROL_RIG_INT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_SET_LOCAL_CONTROL_RIG_FLOATS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_SET_LOCAL_CONTROL_RIG_FLOAT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_SET_LOCAL_CONTROL_RIG_EULER_TRANSFORMS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_SET_LOCAL_CONTROL_RIG_EULER_TRANSFORM: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_SET_LOCAL_CONTROL_RIG_BOOLS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_SET_LOCAL_CONTROL_RIG_BOOL: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_SET_INTERACTION: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_SET_CONTROLS_MASK: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_SET_CONTROL_RIG_WORLD_TRANSFORMS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_SET_CONTROL_RIG_WORLD_TRANSFORM: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_SET_CONTROL_RIG_SPACE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_SET_CONTROL_RIG_PRIORITY_ORDER: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_SET_CONTROL_RIG_LAYERED_MODE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_SET_CONTROL_RIG_APPLY_MODE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_SET_CONSTRAINT_ACTIVE_KEY: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_RENAME_CONTROL_RIG_CONTROL_CHANNELS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_MOVE_CONTROL_RIG_SPACE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_MOVE_CONSTRAINT_KEY: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_MERGE_ANIM_LAYERS_WITH_SETTINGS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_MERGE_ANIM_LAYERS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_LOAD_ANIM_SEQUENCE_INTO_CONTROL_RIG_SECTION_WITH_RANGE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_LOAD_ANIM_SEQUENCE_INTO_CONTROL_RIG_SECTION: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_IS_LAYERED_CONTROL_RIG: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_IS_FK_CONTROL_RIG: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_IMPORT_FBX_TO_CONTROL_RIG_TRACK: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_HIDE_ALL_CONTROLS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_GET_WORLD_SPACE_REFERENCE_KEY: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_GET_VISIBLE_CONTROL_RIGS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_GET_SKELETAL_MESH_COMPONENT_WORLD_TRANSFORMS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_GET_SKELETAL_MESH_COMPONENT_WORLD_TRANSFORM: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_GET_SELECTION_SETS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_GET_LOCAL_CONTROL_RIG_VECTOR2_DS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_GET_LOCAL_CONTROL_RIG_VECTOR2_D: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_GET_LOCAL_CONTROL_RIG_TRANSFORMS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_GET_LOCAL_CONTROL_RIG_TRANSFORM_NO_SCALES: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_GET_LOCAL_CONTROL_RIG_TRANSFORM_NO_SCALE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_GET_LOCAL_CONTROL_RIG_TRANSFORM: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_GET_LOCAL_CONTROL_RIG_SCALES: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_GET_LOCAL_CONTROL_RIG_SCALE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_GET_LOCAL_CONTROL_RIG_ROTATORS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_GET_LOCAL_CONTROL_RIG_ROTATOR: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_GET_LOCAL_CONTROL_RIG_POSITIONS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_GET_LOCAL_CONTROL_RIG_POSITION: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_GET_LOCAL_CONTROL_RIG_INTS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_GET_LOCAL_CONTROL_RIG_INT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_GET_LOCAL_CONTROL_RIG_FLOATS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_GET_LOCAL_CONTROL_RIG_FLOAT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_GET_LOCAL_CONTROL_RIG_EULER_TRANSFORMS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_GET_LOCAL_CONTROL_RIG_EULER_TRANSFORM: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_GET_LOCAL_CONTROL_RIG_BOOLS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_GET_LOCAL_CONTROL_RIG_BOOL: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_GET_FK_CONTROL_RIG_APPLY_MODE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_GET_DEFAULT_PARENT_KEY: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_GET_CONTROLS_MASK: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_GET_CONTROL_RIG_WORLD_TRANSFORMS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_GET_CONTROL_RIG_WORLD_TRANSFORM: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_GET_CONTROL_RIGS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_GET_CONTROL_RIG_PRIORITY_ORDER: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_GET_CONSTRAINTS_FOR_HANDLE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_GET_CONSTRAINT_KEYS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_GET_ANIM_LAYERS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_GET_ANIM_LAYER_INDEX: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_GET_ACTOR_WORLD_TRANSFORMS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_GET_ACTOR_WORLD_TRANSFORM: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_FIND_OR_CREATE_CONTROL_RIG_TRACK: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_FIND_OR_CREATE_CONTROL_RIG_COMPONENT_TRACK: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_EXPORT_FBX_FROM_CONTROL_RIG_SECTION: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_EXPORT_ANIM_SEQUENCE_FROM_SEQUENCER: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_DUPLICATE_ANIM_LAYER: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_DELETE_CONTROL_RIG_SPACE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_DELETE_CONSTRAINT_KEY: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_DELETE_ANIM_LAYER: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_CONTROL_RIG_COPY_VECTOR_PARAMETER_CURVES_TO_TRANSFORM: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_COMPENSATE_ALL: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_COMPENSATE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_COLLAPSE_CONTROL_RIG_ANIM_LAYERS_WITH_SETTINGS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_COLLAPSE_CONTROL_RIG_ANIM_LAYERS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_BLEND_VALUES_ON_SELECTED: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_BAKE_TO_CONTROL_RIG: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_BAKE_CONTROL_RIG_SPACE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_BAKE_CONSTRAINTS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_BAKE_CONSTRAINT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_ADD_CONSTRAINT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_ADD_ANIM_LAYER_FROM_SELECTION: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_CONTROL_RIG_EDIT_MODE_DELEGATE_HELPER_POST_POSE_UPDATE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_CONTROL_RIG_EDIT_MODE_DELEGATE_HELPER_ON_POSE_INITIALIZED: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_CONTROL_RIG_CONTEXT_MENU_CONTEXT_IS_ALT_DOWN: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_CONTROL_RIG_CONTEXT_MENU_CONTEXT_GET_RIG_HIERARCHY_TO_GRAPH_DRAG_AND_DROP_CONTEXT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_CONTROL_RIG_CONTEXT_MENU_CONTEXT_GET_RIG_HIERARCHY_DRAG_AND_DROP_CONTEXT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_CONTROL_RIG_CONTEXT_MENU_CONTEXT_GET_GRAPH_NODE_CONTEXT_MENU_CONTEXT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_CONTROL_RIG_CONTEXT_MENU_CONTEXT_GET_CONTROL_RIG_BLUEPRINT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_CONTROL_RIG_CONTEXT_MENU_CONTEXT_GET_CONTROL_RIG_ASSET_INTERFACE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_CONTROL_RIG_CONTEXT_MENU_CONTEXT_GET_CONTROL_RIG: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_ANIM_LAYER_SET_WEIGHT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_ANIM_LAYER_SET_TYPE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_ANIM_LAYER_SET_SELECTED: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_ANIM_LAYER_SET_NAME: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_ANIM_LAYER_SET_LOCK: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_ANIM_LAYER_SET_KEYED: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_ANIM_LAYER_SET_ACTIVE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_ANIM_LAYER_REMOVE_SELECTED_IN_SEQUENCER: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_ANIM_LAYER_GET_WEIGHT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_ANIM_LAYER_GET_TYPE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_ANIM_LAYER_GET_SELECTED: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_ANIM_LAYER_GET_NAME: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_ANIM_LAYER_GET_LOCK: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_ANIM_LAYER_GET_KEYED: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_ANIM_LAYER_GET_ACTIVE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_ANIM_LAYER_ADD_SELECTED_IN_SEQUENCER: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
pub fn initialize() {
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UAIESelectionSets::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ShowOrHideControls"),
            &raw mut UAIE_SELECTION_SETS_SHOW_OR_HIDE_CONTROLS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ShowAllControls"),
            &raw mut UAIE_SELECTION_SETS_SHOW_ALL_CONTROLS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetShowAndSetSelectedOnly"),
            &raw mut UAIE_SELECTION_SETS_SET_SHOW_AND_SET_SELECTED_ONLY,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetItemRow"),
            &raw mut UAIE_SELECTION_SETS_SET_ITEM_ROW,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetItemColor"),
            &raw mut UAIE_SELECTION_SETS_SET_ITEM_COLOR,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetActorAsActive"),
            &raw mut UAIE_SELECTION_SETS_SET_ACTOR_AS_ACTIVE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SelectItem"),
            &raw mut UAIE_SELECTION_SETS_SELECT_ITEM,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RenameSetItem"),
            &raw mut UAIE_SELECTION_SETS_RENAME_SET_ITEM,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RemoveSelectionFromSetItem"),
            &raw mut UAIE_SELECTION_SETS_REMOVE_SELECTION_FROM_SET_ITEM,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("LoadFromJsonString"),
            &raw mut UAIE_SELECTION_SETS_LOAD_FROM_JSON_STRING,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("LoadFromJsonFile"),
            &raw mut UAIE_SELECTION_SETS_LOAD_FROM_JSON_FILE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("KeyAll"),
            &raw mut UAIE_SELECTION_SETS_KEY_ALL,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("IsolateControls"),
            &raw mut UAIE_SELECTION_SETS_ISOLATE_CONTROLS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("IsMultiAsset"),
            &raw mut UAIE_SELECTION_SETS_IS_MULTI_ASSET,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetShowAndSetSelectedOnly"),
            &raw mut UAIE_SELECTION_SETS_GET_SHOW_AND_SET_SELECTED_ONLY,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetItemRow"),
            &raw mut UAIE_SELECTION_SETS_GET_ITEM_ROW,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetItemName"),
            &raw mut UAIE_SELECTION_SETS_GET_ITEM_NAME,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetItemGuids"),
            &raw mut UAIE_SELECTION_SETS_GET_ITEM_GUIDS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetItemColor"),
            &raw mut UAIE_SELECTION_SETS_GET_ITEM_COLOR,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetAllActors"),
            &raw mut UAIE_SELECTION_SETS_GET_ALL_ACTORS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetActiveSelectionSets"),
            &raw mut UAIE_SELECTION_SETS_GET_ACTIVE_SELECTION_SETS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetActiveActors"),
            &raw mut UAIE_SELECTION_SETS_GET_ACTIVE_ACTORS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ExportAsJsonString"),
            &raw mut UAIE_SELECTION_SETS_EXPORT_AS_JSON_STRING,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ExportAsJsonFile"),
            &raw mut UAIE_SELECTION_SETS_EXPORT_AS_JSON_FILE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("DeleteSetItem"),
            &raw mut UAIE_SELECTION_SETS_DELETE_SET_ITEM,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CreateSetItemFromSelection"),
            &raw mut UAIE_SELECTION_SETS_CREATE_SET_ITEM_FROM_SELECTION,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CreateMirror"),
            &raw mut UAIE_SELECTION_SETS_CREATE_MIRROR,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AddSelectionToSetItem"),
            &raw mut UAIE_SELECTION_SETS_ADD_SELECTION_TO_SET_ITEM,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UControlRigBlueprintEditorLibrary::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetupAllEditorMenus"),
            &raw mut U_CONTROL_RIG_BLUEPRINT_EDITOR_LIBRARY_SETUP_ALL_EDITOR_MENUS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetPreviewMesh"),
            &raw mut U_CONTROL_RIG_BLUEPRINT_EDITOR_LIBRARY_SET_PREVIEW_MESH,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RequestControlRigInit"),
            &raw mut U_CONTROL_RIG_BLUEPRINT_EDITOR_LIBRARY_REQUEST_CONTROL_RIG_INIT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetPreviewMesh"),
            &raw mut U_CONTROL_RIG_BLUEPRINT_EDITOR_LIBRARY_GET_PREVIEW_MESH,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetHierarchyController"),
            &raw mut U_CONTROL_RIG_BLUEPRINT_EDITOR_LIBRARY_GET_HIERARCHY_CONTROLLER,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetHierarchy"),
            &raw mut U_CONTROL_RIG_BLUEPRINT_EDITOR_LIBRARY_GET_HIERARCHY,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetCurrentlyOpenRigBlueprints"),
            &raw mut U_CONTROL_RIG_BLUEPRINT_EDITOR_LIBRARY_GET_CURRENTLY_OPEN_RIG_BLUEPRINTS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetAvailableRigUnits"),
            &raw mut U_CONTROL_RIG_BLUEPRINT_EDITOR_LIBRARY_GET_AVAILABLE_RIG_UNITS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetAvailableRigModules"),
            &raw mut U_CONTROL_RIG_BLUEPRINT_EDITOR_LIBRARY_GET_AVAILABLE_RIG_MODULES,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CastToControlRigBlueprint"),
            &raw mut U_CONTROL_RIG_BLUEPRINT_EDITOR_LIBRARY_CAST_TO_CONTROL_RIG_BLUEPRINT,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UControlRigBlueprintFactory::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CreateNewControlRigAsset"),
            &raw mut U_CONTROL_RIG_BLUEPRINT_FACTORY_CREATE_NEW_CONTROL_RIG_ASSET,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CreateControlRigFromSkeletalMeshOrSkeleton"),
            &raw mut U_CONTROL_RIG_BLUEPRINT_FACTORY_CREATE_CONTROL_RIG_FROM_SKELETAL_MESH_OR_SKELETON,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UControlRigSequencerEditorLibrary::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("TweenControlRig"),
            &raw mut U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_TWEEN_CONTROL_RIG,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SpaceCompensate"),
            &raw mut U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_SPACE_COMPENSATE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SnapControlRig"),
            &raw mut U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_SNAP_CONTROL_RIG,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SmartReduce"),
            &raw mut U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_SMART_REDUCE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ShowAllControls"),
            &raw mut U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_SHOW_ALL_CONTROLS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetLocalControlRigVector2Ds"),
            &raw mut U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_SET_LOCAL_CONTROL_RIG_VECTOR2_DS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetLocalControlRigVector2D"),
            &raw mut U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_SET_LOCAL_CONTROL_RIG_VECTOR2_D,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetLocalControlRigTransforms"),
            &raw mut U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_SET_LOCAL_CONTROL_RIG_TRANSFORMS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetLocalControlRigTransformNoScales"),
            &raw mut U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_SET_LOCAL_CONTROL_RIG_TRANSFORM_NO_SCALES,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetLocalControlRigTransformNoScale"),
            &raw mut U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_SET_LOCAL_CONTROL_RIG_TRANSFORM_NO_SCALE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetLocalControlRigTransform"),
            &raw mut U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_SET_LOCAL_CONTROL_RIG_TRANSFORM,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetLocalControlRigScales"),
            &raw mut U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_SET_LOCAL_CONTROL_RIG_SCALES,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetLocalControlRigScale"),
            &raw mut U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_SET_LOCAL_CONTROL_RIG_SCALE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetLocalControlRigRotators"),
            &raw mut U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_SET_LOCAL_CONTROL_RIG_ROTATORS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetLocalControlRigRotator"),
            &raw mut U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_SET_LOCAL_CONTROL_RIG_ROTATOR,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetLocalControlRigPositions"),
            &raw mut U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_SET_LOCAL_CONTROL_RIG_POSITIONS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetLocalControlRigPosition"),
            &raw mut U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_SET_LOCAL_CONTROL_RIG_POSITION,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetLocalControlRigInts"),
            &raw mut U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_SET_LOCAL_CONTROL_RIG_INTS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetLocalControlRigInt"),
            &raw mut U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_SET_LOCAL_CONTROL_RIG_INT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetLocalControlRigFloats"),
            &raw mut U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_SET_LOCAL_CONTROL_RIG_FLOATS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetLocalControlRigFloat"),
            &raw mut U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_SET_LOCAL_CONTROL_RIG_FLOAT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetLocalControlRigEulerTransforms"),
            &raw mut U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_SET_LOCAL_CONTROL_RIG_EULER_TRANSFORMS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetLocalControlRigEulerTransform"),
            &raw mut U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_SET_LOCAL_CONTROL_RIG_EULER_TRANSFORM,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetLocalControlRigBools"),
            &raw mut U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_SET_LOCAL_CONTROL_RIG_BOOLS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetLocalControlRigBool"),
            &raw mut U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_SET_LOCAL_CONTROL_RIG_BOOL,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetInteraction"),
            &raw mut U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_SET_INTERACTION,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetControlsMask"),
            &raw mut U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_SET_CONTROLS_MASK,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetControlRigWorldTransforms"),
            &raw mut U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_SET_CONTROL_RIG_WORLD_TRANSFORMS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetControlRigWorldTransform"),
            &raw mut U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_SET_CONTROL_RIG_WORLD_TRANSFORM,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetControlRigSpace"),
            &raw mut U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_SET_CONTROL_RIG_SPACE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetControlRigPriorityOrder"),
            &raw mut U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_SET_CONTROL_RIG_PRIORITY_ORDER,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetControlRigLayeredMode"),
            &raw mut U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_SET_CONTROL_RIG_LAYERED_MODE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetControlRigApplyMode"),
            &raw mut U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_SET_CONTROL_RIG_APPLY_MODE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetConstraintActiveKey"),
            &raw mut U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_SET_CONSTRAINT_ACTIVE_KEY,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RenameControlRigControlChannels"),
            &raw mut U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_RENAME_CONTROL_RIG_CONTROL_CHANNELS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("MoveControlRigSpace"),
            &raw mut U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_MOVE_CONTROL_RIG_SPACE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("MoveConstraintKey"),
            &raw mut U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_MOVE_CONSTRAINT_KEY,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("MergeAnimLayersWithSettings"),
            &raw mut U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_MERGE_ANIM_LAYERS_WITH_SETTINGS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("MergeAnimLayers"),
            &raw mut U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_MERGE_ANIM_LAYERS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("LoadAnimSequenceIntoControlRigSectionWithRange"),
            &raw mut U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_LOAD_ANIM_SEQUENCE_INTO_CONTROL_RIG_SECTION_WITH_RANGE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("LoadAnimSequenceIntoControlRigSection"),
            &raw mut U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_LOAD_ANIM_SEQUENCE_INTO_CONTROL_RIG_SECTION,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("IsLayeredControlRig"),
            &raw mut U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_IS_LAYERED_CONTROL_RIG,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("IsFKControlRig"),
            &raw mut U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_IS_FK_CONTROL_RIG,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ImportFBXToControlRigTrack"),
            &raw mut U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_IMPORT_FBX_TO_CONTROL_RIG_TRACK,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("HideAllControls"),
            &raw mut U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_HIDE_ALL_CONTROLS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetWorldSpaceReferenceKey"),
            &raw mut U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_GET_WORLD_SPACE_REFERENCE_KEY,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetVisibleControlRigs"),
            &raw mut U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_GET_VISIBLE_CONTROL_RIGS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetSkeletalMeshComponentWorldTransforms"),
            &raw mut U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_GET_SKELETAL_MESH_COMPONENT_WORLD_TRANSFORMS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetSkeletalMeshComponentWorldTransform"),
            &raw mut U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_GET_SKELETAL_MESH_COMPONENT_WORLD_TRANSFORM,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetSelectionSets"),
            &raw mut U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_GET_SELECTION_SETS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetLocalControlRigVector2Ds"),
            &raw mut U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_GET_LOCAL_CONTROL_RIG_VECTOR2_DS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetLocalControlRigVector2D"),
            &raw mut U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_GET_LOCAL_CONTROL_RIG_VECTOR2_D,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetLocalControlRigTransforms"),
            &raw mut U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_GET_LOCAL_CONTROL_RIG_TRANSFORMS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetLocalControlRigTransformNoScales"),
            &raw mut U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_GET_LOCAL_CONTROL_RIG_TRANSFORM_NO_SCALES,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetLocalControlRigTransformNoScale"),
            &raw mut U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_GET_LOCAL_CONTROL_RIG_TRANSFORM_NO_SCALE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetLocalControlRigTransform"),
            &raw mut U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_GET_LOCAL_CONTROL_RIG_TRANSFORM,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetLocalControlRigScales"),
            &raw mut U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_GET_LOCAL_CONTROL_RIG_SCALES,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetLocalControlRigScale"),
            &raw mut U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_GET_LOCAL_CONTROL_RIG_SCALE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetLocalControlRigRotators"),
            &raw mut U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_GET_LOCAL_CONTROL_RIG_ROTATORS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetLocalControlRigRotator"),
            &raw mut U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_GET_LOCAL_CONTROL_RIG_ROTATOR,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetLocalControlRigPositions"),
            &raw mut U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_GET_LOCAL_CONTROL_RIG_POSITIONS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetLocalControlRigPosition"),
            &raw mut U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_GET_LOCAL_CONTROL_RIG_POSITION,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetLocalControlRigInts"),
            &raw mut U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_GET_LOCAL_CONTROL_RIG_INTS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetLocalControlRigInt"),
            &raw mut U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_GET_LOCAL_CONTROL_RIG_INT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetLocalControlRigFloats"),
            &raw mut U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_GET_LOCAL_CONTROL_RIG_FLOATS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetLocalControlRigFloat"),
            &raw mut U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_GET_LOCAL_CONTROL_RIG_FLOAT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetLocalControlRigEulerTransforms"),
            &raw mut U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_GET_LOCAL_CONTROL_RIG_EULER_TRANSFORMS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetLocalControlRigEulerTransform"),
            &raw mut U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_GET_LOCAL_CONTROL_RIG_EULER_TRANSFORM,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetLocalControlRigBools"),
            &raw mut U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_GET_LOCAL_CONTROL_RIG_BOOLS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetLocalControlRigBool"),
            &raw mut U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_GET_LOCAL_CONTROL_RIG_BOOL,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetFKControlRigApplyMode"),
            &raw mut U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_GET_FK_CONTROL_RIG_APPLY_MODE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetDefaultParentKey"),
            &raw mut U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_GET_DEFAULT_PARENT_KEY,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetControlsMask"),
            &raw mut U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_GET_CONTROLS_MASK,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetControlRigWorldTransforms"),
            &raw mut U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_GET_CONTROL_RIG_WORLD_TRANSFORMS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetControlRigWorldTransform"),
            &raw mut U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_GET_CONTROL_RIG_WORLD_TRANSFORM,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetControlRigs"),
            &raw mut U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_GET_CONTROL_RIGS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetControlRigPriorityOrder"),
            &raw mut U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_GET_CONTROL_RIG_PRIORITY_ORDER,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetConstraintsForHandle"),
            &raw mut U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_GET_CONSTRAINTS_FOR_HANDLE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetConstraintKeys"),
            &raw mut U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_GET_CONSTRAINT_KEYS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetAnimLayers"),
            &raw mut U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_GET_ANIM_LAYERS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetAnimLayerIndex"),
            &raw mut U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_GET_ANIM_LAYER_INDEX,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetActorWorldTransforms"),
            &raw mut U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_GET_ACTOR_WORLD_TRANSFORMS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetActorWorldTransform"),
            &raw mut U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_GET_ACTOR_WORLD_TRANSFORM,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("FindOrCreateControlRigTrack"),
            &raw mut U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_FIND_OR_CREATE_CONTROL_RIG_TRACK,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("FindOrCreateControlRigComponentTrack"),
            &raw mut U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_FIND_OR_CREATE_CONTROL_RIG_COMPONENT_TRACK,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ExportFBXFromControlRigSection"),
            &raw mut U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_EXPORT_FBX_FROM_CONTROL_RIG_SECTION,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ExportAnimSequenceFromSequencer"),
            &raw mut U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_EXPORT_ANIM_SEQUENCE_FROM_SEQUENCER,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("DuplicateAnimLayer"),
            &raw mut U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_DUPLICATE_ANIM_LAYER,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("DeleteControlRigSpace"),
            &raw mut U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_DELETE_CONTROL_RIG_SPACE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("DeleteConstraintKey"),
            &raw mut U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_DELETE_CONSTRAINT_KEY,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("DeleteAnimLayer"),
            &raw mut U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_DELETE_ANIM_LAYER,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ControlRigCopyVectorParameterCurvesToTransform"),
            &raw mut U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_CONTROL_RIG_COPY_VECTOR_PARAMETER_CURVES_TO_TRANSFORM,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CompensateAll"),
            &raw mut U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_COMPENSATE_ALL,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("Compensate"),
            &raw mut U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_COMPENSATE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CollapseControlRigAnimLayersWithSettings"),
            &raw mut U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_COLLAPSE_CONTROL_RIG_ANIM_LAYERS_WITH_SETTINGS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CollapseControlRigAnimLayers"),
            &raw mut U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_COLLAPSE_CONTROL_RIG_ANIM_LAYERS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("BlendValuesOnSelected"),
            &raw mut U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_BLEND_VALUES_ON_SELECTED,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("BakeToControlRig"),
            &raw mut U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_BAKE_TO_CONTROL_RIG,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("BakeControlRigSpace"),
            &raw mut U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_BAKE_CONTROL_RIG_SPACE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("BakeConstraints"),
            &raw mut U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_BAKE_CONSTRAINTS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("BakeConstraint"),
            &raw mut U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_BAKE_CONSTRAINT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AddConstraint"),
            &raw mut U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_ADD_CONSTRAINT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AddAnimLayerFromSelection"),
            &raw mut U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_ADD_ANIM_LAYER_FROM_SELECTION,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UControlRigEditModeDelegateHelper::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("PostPoseUpdate"),
            &raw mut U_CONTROL_RIG_EDIT_MODE_DELEGATE_HELPER_POST_POSE_UPDATE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("OnPoseInitialized"),
            &raw mut U_CONTROL_RIG_EDIT_MODE_DELEGATE_HELPER_ON_POSE_INITIALIZED,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UControlRigContextMenuContext::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("IsAltDown"),
            &raw mut U_CONTROL_RIG_CONTEXT_MENU_CONTEXT_IS_ALT_DOWN,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetRigHierarchyToGraphDragAndDropContext"),
            &raw mut U_CONTROL_RIG_CONTEXT_MENU_CONTEXT_GET_RIG_HIERARCHY_TO_GRAPH_DRAG_AND_DROP_CONTEXT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetRigHierarchyDragAndDropContext"),
            &raw mut U_CONTROL_RIG_CONTEXT_MENU_CONTEXT_GET_RIG_HIERARCHY_DRAG_AND_DROP_CONTEXT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetGraphNodeContextMenuContext"),
            &raw mut U_CONTROL_RIG_CONTEXT_MENU_CONTEXT_GET_GRAPH_NODE_CONTEXT_MENU_CONTEXT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetControlRigBlueprint"),
            &raw mut U_CONTROL_RIG_CONTEXT_MENU_CONTEXT_GET_CONTROL_RIG_BLUEPRINT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetControlRigAssetInterface"),
            &raw mut U_CONTROL_RIG_CONTEXT_MENU_CONTEXT_GET_CONTROL_RIG_ASSET_INTERFACE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetControlRig"),
            &raw mut U_CONTROL_RIG_CONTEXT_MENU_CONTEXT_GET_CONTROL_RIG,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UAnimLayer::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetWeight"),
            &raw mut U_ANIM_LAYER_SET_WEIGHT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetType"),
            &raw mut U_ANIM_LAYER_SET_TYPE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetSelected"),
            &raw mut U_ANIM_LAYER_SET_SELECTED,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetName"),
            &raw mut U_ANIM_LAYER_SET_NAME,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetLock"),
            &raw mut U_ANIM_LAYER_SET_LOCK,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetKeyed"),
            &raw mut U_ANIM_LAYER_SET_KEYED,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetActive"),
            &raw mut U_ANIM_LAYER_SET_ACTIVE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RemoveSelectedInSequencer"),
            &raw mut U_ANIM_LAYER_REMOVE_SELECTED_IN_SEQUENCER,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetWeight"),
            &raw mut U_ANIM_LAYER_GET_WEIGHT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetType"),
            &raw mut U_ANIM_LAYER_GET_TYPE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetSelected"),
            &raw mut U_ANIM_LAYER_GET_SELECTED,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetName"),
            &raw mut U_ANIM_LAYER_GET_NAME,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetLock"),
            &raw mut U_ANIM_LAYER_GET_LOCK,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetKeyed"),
            &raw mut U_ANIM_LAYER_GET_KEYED,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetActive"),
            &raw mut U_ANIM_LAYER_GET_ACTIVE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AddSelectedInSequencer"),
            &raw mut U_ANIM_LAYER_ADD_SELECTED_IN_SEQUENCER,
        );
    }
}
#[repr(C, align(8))]
pub struct FMultiControlRigElementSelection {
    __padding_end: [u8; 32],
}
impl FMultiControlRigElementSelection {}
#[repr(C, align(8))]
pub struct FControlRigInteractionTransformContext {
    #[doc(hidden)]
    __padding_8: [u8; 8],
    pub drag: crate::bindings::core_u_object::FVector,
    #[doc(hidden)]
    __padding_40: [u8; 8],
    pub rot: crate::bindings::core_u_object::FRotator,
    #[doc(hidden)]
    __padding_72: [u8; 8],
    pub scale: crate::bindings::core_u_object::FVector,
    pub space: EControlRigInteractionTransformSpace,
    __padding_end: [u8; 4],
}
impl FControlRigInteractionTransformContext {}
#[repr(C, align(4))]
pub struct FRigSpacePickerBakeSettings {
    pub target_space: crate::bindings::control_rig::FRigElementKey,
    pub settings: crate::bindings::movie_scene_tools::FBakingAnimationKeySettings,
    __padding_end: [u8; 16],
}
impl FRigSpacePickerBakeSettings {}
#[repr(C, align(8))]
pub struct FAIESelectionSetItemName {
    pub name: FString,
    pub mirror_name: FString,
    pub ty: i32,
    pub owner_actor_name: FString,
}
impl FAIESelectionSetItemName {}
#[repr(C, align(4))]
pub struct FAIESelectionSetItemViewData {
    pub color: crate::bindings::core_u_object::FLinearColor,
    pub row: i32,
    __padding_end: [u8; 4],
}
impl FAIESelectionSetItemViewData {}
#[repr(C, align(8))]
pub struct FAIESelectionSetItem {
    #[doc(hidden)]
    __padding_16: [u8; 16],
    pub item_name: FText,
    pub names: TArray<FAIESelectionSetItemName>,
    pub view_data: FAIESelectionSetItemViewData,
    __padding_end: [u8; 32],
}
impl FAIESelectionSetItem {}
#[repr(C, align(1))]
pub struct FAnimDetailsBool {
    pub bool: bool,
}
impl FAnimDetailsBool {}
#[repr(C, align(8))]
pub struct FAnimDetailsEnum {
    __padding_end: [u8; 16],
}
impl FAnimDetailsEnum {}
#[repr(C, align(8))]
pub struct FAnimDetailsFloat {
    pub float: f64,
}
impl FAnimDetailsFloat {}
#[repr(C, align(8))]
pub struct FAnimDetailsInteger {
    pub integer: i64,
}
impl FAnimDetailsInteger {}
#[repr(C, align(8))]
pub struct FAnimDetailsLocation {
    pub lx: f64,
    pub ly: f64,
    pub lz: f64,
}
impl FAnimDetailsLocation {}
#[repr(C, align(8))]
pub struct FAnimDetailsRotation {
    pub rx: f64,
    pub ry: f64,
    pub rz: f64,
}
impl FAnimDetailsRotation {}
#[repr(C, align(8))]
pub struct FAnimDetailsScale {
    pub sx: f64,
    pub sy: f64,
    pub sz: f64,
}
impl FAnimDetailsScale {}
#[repr(C, align(8))]
pub struct FAnimDetailsVector2D {
    pub x: f64,
    pub y: f64,
}
impl FAnimDetailsVector2D {}
#[repr(C, align(8))]
pub struct FControlRigSequencerBindingProxy {
    pub proxy: crate::bindings::movie_scene::FMovieSceneBindingProxy,
    pub control_rig: UPtr<crate::bindings::control_rig::UControlRig>,
    pub track: UPtr<crate::bindings::control_rig::UMovieSceneControlRigParameterTrack>,
}
impl FControlRigSequencerBindingProxy {}
#[repr(C, align(8))]
pub struct FControlRigRigHierarchyDragAndDropContext {
    pub dragged_hierarchy_keys: TArray<crate::bindings::control_rig::FRigHierarchyKey>,
    pub target_hierarchy_key: crate::bindings::control_rig::FRigHierarchyKey,
    __padding_end: [u8; 4],
}
impl FControlRigRigHierarchyDragAndDropContext {}
#[repr(C, align(8))]
pub struct FControlRigGraphNodeContextMenuContext {
    pub graph: UPtr<crate::bindings::rig_vm_developer::URigVMGraph>,
    pub node: UPtr<crate::bindings::rig_vm_developer::URigVMNode>,
    pub pin: UPtr<crate::bindings::rig_vm_developer::URigVMPin>,
}
impl FControlRigGraphNodeContextMenuContext {}
#[repr(C, align(8))]
pub struct FControlRigRigHierarchyToGraphDragAndDropContext {
    pub dragged_hierarchy_keys: TArray<crate::bindings::control_rig::FRigHierarchyKey>,
    __padding_end: [u8; 24],
}
impl FControlRigRigHierarchyToGraphDragAndDropContext {}
#[repr(C, align(4))]
pub struct FMergeAnimLayerSettings {
    pub baking_key_settings: crate::bindings::movie_scene_tools::EBakingKeySettings,
    pub frame_increment: i32,
    pub b_reduce_keys: bool,
    pub tolerance_percentage: f32,
}
impl FMergeAnimLayerSettings {}
#[repr(C, align(4))]
pub struct FAnimLayerPropertyAndChannels {
    pub name: FName,
    __padding_end: [u8; 4],
}
impl FAnimLayerPropertyAndChannels {}
#[repr(C, align(8))]
pub struct FAnimLayerSelectionSet {
    pub bound_object: TWeakObjectPtr<crate::bindings::core_u_object::UObject>,
    pub names: TMap<FName, FAnimLayerPropertyAndChannels>,
}
impl FAnimLayerSelectionSet {}
#[repr(C, align(8))]
pub struct FAnimLayerSectionItem {
    pub anim_layer_set: FAnimLayerSelectionSet,
    pub section: TWeakObjectPtr<crate::bindings::movie_scene::UMovieSceneSection>,
}
impl FAnimLayerSectionItem {}
#[repr(C, align(8))]
pub struct FAnimLayerItem {
    pub section_items: TArray<FAnimLayerSectionItem>,
    __padding_end: [u8; 16],
}
impl FAnimLayerItem {}
#[repr(C, align(8))]
pub struct FAnimLayerState {
    pub b_keyed: crate::bindings::slate_core::ECheckBoxState,
    pub b_selected: crate::bindings::slate_core::ECheckBoxState,
    pub b_active: bool,
    pub b_lock: bool,
    pub name: FText,
    pub weight: f64,
    pub ty: i32,
    __padding_end: [u8; 4],
}
impl FAnimLayerState {}
#[repr(C, align(8))]
pub struct FAnimLayerControlRigObject {
    pub control_rig: TWeakObjectPtr<crate::bindings::control_rig::UControlRig>,
    pub control_names: TArray<FName>,
}
impl FAnimLayerControlRigObject {}
#[repr(C, align(4))]
pub struct FAnimLayerSceneObject {
    pub scene_object_or_component: TWeakObjectPtr<
        crate::bindings::core_u_object::UObject,
    >,
}
impl FAnimLayerSceneObject {}
#[repr(C, align(8))]
pub struct FAnimLayerObjects {
    pub control_rig_objects: TArray<FAnimLayerControlRigObject>,
    pub scene_objects: TArray<FAnimLayerSceneObject>,
}
impl FAnimLayerObjects {}
#[repr(C, align(8))]
pub struct FControlRigForWorldTransforms {
    pub control_rig: TWeakObjectPtr<crate::bindings::control_rig::UControlRig>,
    pub control_names: TArray<FName>,
}
impl FControlRigForWorldTransforms {}
#[repr(C, align(8))]
pub struct FControlRigSnapperSelection {
    pub actors: TArray<crate::bindings::movie_scene::FActorForWorldTransforms>,
    pub control_rigs: TArray<FControlRigForWorldTransforms>,
}
impl FControlRigSnapperSelection {}
#[repr(C, align(8))]
pub struct UAnimDetailsOptionsMenuContext {
    __padding_end: [u8; 64],
}
impl UAnimDetailsOptionsMenuContext {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAnimDetailsOptionsMenuContext")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UAnimDetailsSettings {
    __padding_end: [u8; 56],
}
impl UAnimDetailsSettings {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAnimDetailsSettings")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UAnimSequenceConverterFactory {
    __padding_end: [u8; 168],
}
impl UAnimSequenceConverterFactory {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAnimSequenceConverterFactory")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UConstraintCreationOptions {
    __padding_end: [u8; 56],
}
impl UConstraintCreationOptions {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UConstraintCreationOptions")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UAIESelectionSets {
    #[doc(hidden)]
    __padding_48: [u8; 48],
    pub selection_sets: TMap<
        crate::bindings::core_u_object::FGuid,
        FAIESelectionSetItem,
    >,
    __padding_end: [u8; 200],
}
impl UAIESelectionSets {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAIESelectionSets")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UAnimDetailsProxyManager {
    __padding_end: [u8; 336],
}
impl UAnimDetailsProxyManager {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAnimDetailsProxyManager")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UAnimDetailsSelection {
    __padding_end: [u8; 160],
}
impl UAnimDetailsSelection {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAnimDetailsSelection")
            .unwrap()
    }
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
pub struct UAnimDetailsProxyBase {
    __padding_end: [u8; 368],
}
impl UAnimDetailsProxyBase {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAnimDetailsProxyBase")
            .unwrap()
    }
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
pub struct UAnimDetailsProxyBool {
    #[doc(hidden)]
    __padding_360: [u8; 360],
    pub bool: FAnimDetailsBool,
    __padding_end: [u8; 7],
}
impl UAnimDetailsProxyBool {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAnimDetailsProxyBool")
            .unwrap()
    }
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
pub struct UAnimDetailsProxyEnum {
    #[doc(hidden)]
    __padding_360: [u8; 360],
    pub enum_: FAnimDetailsEnum,
    __padding_end: [u8; 8],
}
impl UAnimDetailsProxyEnum {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAnimDetailsProxyEnum")
            .unwrap()
    }
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
pub struct UAnimDetailsProxyFloat {
    #[doc(hidden)]
    __padding_360: [u8; 360],
    pub float: FAnimDetailsFloat,
}
impl UAnimDetailsProxyFloat {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAnimDetailsProxyFloat")
            .unwrap()
    }
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
pub struct UAnimDetailsProxyInteger {
    #[doc(hidden)]
    __padding_360: [u8; 360],
    pub integer: FAnimDetailsInteger,
}
impl UAnimDetailsProxyInteger {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAnimDetailsProxyInteger")
            .unwrap()
    }
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
pub struct UAnimDetailsProxyLocation {
    #[doc(hidden)]
    __padding_360: [u8; 360],
    pub location: FAnimDetailsLocation,
}
impl UAnimDetailsProxyLocation {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAnimDetailsProxyLocation")
            .unwrap()
    }
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
pub struct UAnimDetailsProxyRotation {
    #[doc(hidden)]
    __padding_360: [u8; 360],
    pub rotation: FAnimDetailsRotation,
}
impl UAnimDetailsProxyRotation {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAnimDetailsProxyRotation")
            .unwrap()
    }
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
pub struct UAnimDetailsProxyScale {
    #[doc(hidden)]
    __padding_360: [u8; 360],
    pub scale: FAnimDetailsScale,
}
impl UAnimDetailsProxyScale {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAnimDetailsProxyScale")
            .unwrap()
    }
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
pub struct UAnimDetailsProxyTransform {
    #[doc(hidden)]
    __padding_360: [u8; 360],
    pub location: FAnimDetailsLocation,
    pub rotation: FAnimDetailsRotation,
    pub scale: FAnimDetailsScale,
}
impl UAnimDetailsProxyTransform {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAnimDetailsProxyTransform")
            .unwrap()
    }
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
pub struct UAnimDetailsProxyVector2D {
    #[doc(hidden)]
    __padding_360: [u8; 360],
    pub vector2_d: FAnimDetailsVector2D,
    __padding_end: [u8; 8],
}
impl UAnimDetailsProxyVector2D {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAnimDetailsProxyVector2D")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UBakeToControlRigSettings {
    __padding_end: [u8; 72],
}
impl UBakeToControlRigSettings {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBakeToControlRigSettings")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UControlRigBlueprintEditorLibrary {
    __padding_end: [u8; 48],
}
impl UControlRigBlueprintEditorLibrary {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UControlRigBlueprintEditorLibrary")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UControlRigBlueprintFactory {
    __padding_end: [u8; 144],
}
impl UControlRigBlueprintFactory {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UControlRigBlueprintFactory")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UControlRigShapeLibraryFactory {
    __padding_end: [u8; 136],
}
impl UControlRigShapeLibraryFactory {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UControlRigShapeLibraryFactory")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UControlRigSequencerEditorLibrary {
    __padding_end: [u8; 48],
}
impl UControlRigSequencerEditorLibrary {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UControlRigSequencerEditorLibrary")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UControlRigThumbnailRenderer {
    __padding_end: [u8; 232],
}
impl UControlRigThumbnailRenderer {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UControlRigThumbnailRenderer")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UControlRigEditModeDelegateHelper {
    __padding_end: [u8; 72],
}
impl UControlRigEditModeDelegateHelper {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UControlRigEditModeDelegateHelper")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UControlRigEditModeSettings {
    __padding_end: [u8; 240],
}
impl UControlRigEditModeSettings {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UControlRigEditModeSettings")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UControlRigContextMenuContext {
    __padding_end: [u8; 232],
}
impl UControlRigContextMenuContext {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UControlRigContextMenuContext")
            .unwrap()
    }
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
pub struct UControlRigSkeletalMeshComponent {
    __padding_end: [u8; 5504],
}
impl UControlRigSkeletalMeshComponent {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UControlRigSkeletalMeshComponent")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UControlRigWrapperObject {
    __padding_end: [u8; 152],
}
impl UControlRigWrapperObject {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UControlRigWrapperObject")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct URigConnectorTargetsDetailWrapper {
    __padding_end: [u8; 88],
}
impl URigConnectorTargetsDetailWrapper {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("URigConnectorTargetsDetailWrapper")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct URigDependencyGraph {
    __padding_end: [u8; 848],
}
impl URigDependencyGraph {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("URigDependencyGraph")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct URigDependencyGraphNode {
    __padding_end: [u8; 456],
}
impl URigDependencyGraphNode {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("URigDependencyGraphNode")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct URigDependencyGraphSchema {
    __padding_end: [u8; 48],
}
impl URigDependencyGraphSchema {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("URigDependencyGraphSchema")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UAnimationAuthoringSettings {
    __padding_end: [u8; 112],
}
impl UAnimationAuthoringSettings {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAnimationAuthoringSettings")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UAnimLayerSequencerFilter {
    __padding_end: [u8; 48],
}
impl UAnimLayerSequencerFilter {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAnimLayerSequencerFilter")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UAnimLayerWeightProxy {
    #[doc(hidden)]
    __padding_48: [u8; 48],
    pub weight: f64,
}
impl UAnimLayerWeightProxy {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAnimLayerWeightProxy")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UAnimLayer {
    __padding_end: [u8; 184],
}
impl UAnimLayer {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAnimLayer")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UAnimLayers {
    #[doc(hidden)]
    __padding_48: [u8; 48],
    pub anim_layers: TArray<UPtr<UAnimLayer>>,
    __padding_end: [u8; 72],
}
impl UAnimLayers {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAnimLayers")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UControlRigTrackFilter {
    __padding_end: [u8; 48],
}
impl UControlRigTrackFilter {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UControlRigTrackFilter")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct ULoadAnimToControlRigSettings {
    __padding_end: [u8; 80],
}
impl ULoadAnimToControlRigSettings {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("ULoadAnimToControlRigSettings")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct USelectionSetsSettings {
    __padding_end: [u8; 120],
}
impl USelectionSetsSettings {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("USelectionSetsSettings")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UAssetDefinition_ControlRigPose {
    __padding_end: [u8; 72],
}
impl UAssetDefinition_ControlRigPose {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAssetDefinition_ControlRigPose")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UControlRigPoseThumbnailRenderer {
    __padding_end: [u8; 72],
}
impl UControlRigPoseThumbnailRenderer {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UControlRigPoseThumbnailRenderer")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UControlRigSnapSettings {
    #[doc(hidden)]
    __padding_48: [u8; 48],
    pub b_keep_offset: bool,
    pub b_snap_position: bool,
    pub b_snap_rotation: bool,
    pub b_snap_scale: bool,
    pub baking_key_settings: crate::bindings::movie_scene_tools::EBakingKeySettings,
    pub frame_increment: i32,
    pub b_reduce_keys: bool,
    pub tolerance: f32,
    __padding_end: [u8; 4],
}
impl UControlRigSnapSettings {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UControlRigSnapSettings")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UCreateControlPoseAssetRigSettings {
    __padding_end: [u8; 64],
}
impl UCreateControlPoseAssetRigSettings {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UCreateControlPoseAssetRigSettings")
            .unwrap()
    }
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
pub struct EControlRigInteractionTransformSpace(pub i32);
impl EControlRigInteractionTransformSpace {
    pub const WORLD: EControlRigInteractionTransformSpace = EControlRigInteractionTransformSpace(
        0,
    );
    pub const LOCAL: EControlRigInteractionTransformSpace = EControlRigInteractionTransformSpace(
        1,
    );
    pub const PARENT: EControlRigInteractionTransformSpace = EControlRigInteractionTransformSpace(
        2,
    );
    pub const EXPLICIT: EControlRigInteractionTransformSpace = EControlRigInteractionTransformSpace(
        3,
    );
}
#[repr(transparent)]
pub struct EControlRigConstrainTab(pub u8);
impl EControlRigConstrainTab {
    pub const SPACES: EControlRigConstrainTab = EControlRigConstrainTab(0);
    pub const CONSTRAINTS: EControlRigConstrainTab = EControlRigConstrainTab(1);
    pub const SNAPPER: EControlRigConstrainTab = EControlRigConstrainTab(2);
}
#[repr(transparent)]
pub struct ECastToControlRigBlueprintCases(pub u8);
impl ECastToControlRigBlueprintCases {
    pub const CAST_SUCCEEDED: ECastToControlRigBlueprintCases = ECastToControlRigBlueprintCases(
        0,
    );
    pub const CAST_FAILED: ECastToControlRigBlueprintCases = ECastToControlRigBlueprintCases(
        1,
    );
}
#[repr(transparent)]
pub struct EAnimToolBlendOperation(pub u8);
impl EAnimToolBlendOperation {
    pub const TWEEN: EAnimToolBlendOperation = EAnimToolBlendOperation(0);
    pub const BLEND_TO_NEIGHBOR: EAnimToolBlendOperation = EAnimToolBlendOperation(1);
    pub const PUSH_PULL: EAnimToolBlendOperation = EAnimToolBlendOperation(2);
    pub const BLEND_RELATIVE: EAnimToolBlendOperation = EAnimToolBlendOperation(3);
    pub const BLEND_TO_EASE: EAnimToolBlendOperation = EAnimToolBlendOperation(4);
    pub const SMOOTH_ROUGH: EAnimToolBlendOperation = EAnimToolBlendOperation(5);
}
#[repr(transparent)]
pub struct EAnimLayerType(pub u8);
impl EAnimLayerType {
    pub const BASE: EAnimLayerType = EAnimLayerType(0);
    pub const ADDITIVE: EAnimLayerType = EAnimLayerType(1);
    pub const OVERRIDE: EAnimLayerType = EAnimLayerType(2);
}
