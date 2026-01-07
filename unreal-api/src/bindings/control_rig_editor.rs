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
    pub fn show_or_hide_controls(
        &self,
        in_guid: &crate::bindings::core_u_object::FGuid,
        b_show: bool,
        b_do_mirror: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<19>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig_editor::UAIE_SELECTION_SETS_SHOW_OR_HIDE_CONTROLS,
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
        unsafe {
            std::ptr::copy_nonoverlapping(&b_show, __buffer.add(16).cast::<bool>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_do_mirror,
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
                crate::bindings::control_rig_editor::UAIE_SELECTION_SETS_SHOW_OR_HIDE_CONTROLS,
                __buffer,
            )
        };
        unsafe { __buffer.add(18).cast::<bool>().read() }
    }
    pub fn show_all_controls(
        &self,
        in_guid: &crate::bindings::core_u_object::FGuid,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<17>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig_editor::UAIE_SELECTION_SETS_SHOW_ALL_CONTROLS,
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
                crate::bindings::control_rig_editor::UAIE_SELECTION_SETS_SHOW_ALL_CONTROLS,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<bool>().read() }
    }
    pub fn set_show_and_set_selected_only(&mut self, b_in_show_selected_only: bool) {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig_editor::UAIE_SELECTION_SETS_SET_SHOW_AND_SET_SELECTED_ONLY,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_in_show_selected_only,
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
                crate::bindings::control_rig_editor::UAIE_SELECTION_SETS_SET_SHOW_AND_SET_SELECTED_ONLY,
                __buffer,
            )
        };
    }
    pub fn set_item_row(
        &mut self,
        in_guid: &crate::bindings::core_u_object::FGuid,
        in_row: i32,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<21>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig_editor::UAIE_SELECTION_SETS_SET_ITEM_ROW,
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
        unsafe {
            std::ptr::copy_nonoverlapping(&in_row, __buffer.add(16).cast::<i32>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig_editor::UAIE_SELECTION_SETS_SET_ITEM_ROW,
                __buffer,
            )
        };
        unsafe { __buffer.add(20).cast::<bool>().read() }
    }
    pub fn set_item_color(
        &mut self,
        in_guid: &crate::bindings::core_u_object::FGuid,
        in_color: &crate::bindings::core_u_object::FLinearColor,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<33>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig_editor::UAIE_SELECTION_SETS_SET_ITEM_COLOR,
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
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_color,
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
                crate::bindings::control_rig_editor::UAIE_SELECTION_SETS_SET_ITEM_COLOR,
                __buffer,
            )
        };
        unsafe { __buffer.add(32).cast::<bool>().read() }
    }
    pub fn set_actor_as_active(
        &mut self,
        in_actor: UPtr<crate::bindings::engine::AActor>,
        b_set_active: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<10>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig_editor::UAIE_SELECTION_SETS_SET_ACTOR_AS_ACTIVE,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_actor,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::AActor>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_set_active,
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
                crate::bindings::control_rig_editor::UAIE_SELECTION_SETS_SET_ACTOR_AS_ACTIVE,
                __buffer,
            )
        };
        unsafe { __buffer.add(9).cast::<bool>().read() }
    }
    pub fn select_item(
        &self,
        in_guid: &crate::bindings::core_u_object::FGuid,
        b_do_mirror: bool,
        b_add: bool,
        b_toggle: bool,
        b_select: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<21>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig_editor::UAIE_SELECTION_SETS_SELECT_ITEM,
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
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_do_mirror,
                __buffer.add(16).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&b_add, __buffer.add(17).cast::<bool>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&b_toggle, __buffer.add(18).cast::<bool>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&b_select, __buffer.add(19).cast::<bool>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig_editor::UAIE_SELECTION_SETS_SELECT_ITEM,
                __buffer,
            )
        };
        unsafe { __buffer.add(20).cast::<bool>().read() }
    }
    pub fn rename_set_item(
        &mut self,
        in_guid: &crate::bindings::core_u_object::FGuid,
        new_name: &FText,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<33>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig_editor::UAIE_SELECTION_SETS_RENAME_SET_ITEM,
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
        unsafe {
            std::ptr::copy_nonoverlapping(new_name, __buffer.add(16).cast::<FText>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig_editor::UAIE_SELECTION_SETS_RENAME_SET_ITEM,
                __buffer,
            )
        };
        unsafe { __buffer.add(32).cast::<bool>().read() }
    }
    pub fn remove_selection_from_set_item(
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
                crate::bindings::control_rig_editor::UAIE_SELECTION_SETS_REMOVE_SELECTION_FROM_SET_ITEM,
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
                crate::bindings::control_rig_editor::UAIE_SELECTION_SETS_REMOVE_SELECTION_FROM_SET_ITEM,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<bool>().read() }
    }
    pub fn load_from_json_string(&mut self, json_string: FString) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<17>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig_editor::UAIE_SELECTION_SETS_LOAD_FROM_JSON_STRING,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &json_string,
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
                crate::bindings::control_rig_editor::UAIE_SELECTION_SETS_LOAD_FROM_JSON_STRING,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<bool>().read() }
    }
    pub fn load_from_json_file(
        &mut self,
        json_file_path: &crate::bindings::core_u_object::FFilePath,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<17>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig_editor::UAIE_SELECTION_SETS_LOAD_FROM_JSON_FILE,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                json_file_path,
                __buffer.add(0).cast::<crate::bindings::core_u_object::FFilePath>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig_editor::UAIE_SELECTION_SETS_LOAD_FROM_JSON_FILE,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<bool>().read() }
    }
    pub fn key_all(&self, in_guid: &crate::bindings::core_u_object::FGuid) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<17>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig_editor::UAIE_SELECTION_SETS_KEY_ALL,
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
                crate::bindings::control_rig_editor::UAIE_SELECTION_SETS_KEY_ALL,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<bool>().read() }
    }
    pub fn isolate_controls(
        &self,
        in_guid: &crate::bindings::core_u_object::FGuid,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<17>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig_editor::UAIE_SELECTION_SETS_ISOLATE_CONTROLS,
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
                crate::bindings::control_rig_editor::UAIE_SELECTION_SETS_ISOLATE_CONTROLS,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<bool>().read() }
    }
    pub fn is_multi_asset(
        &self,
        in_guid: &crate::bindings::core_u_object::FGuid,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<17>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig_editor::UAIE_SELECTION_SETS_IS_MULTI_ASSET,
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
                crate::bindings::control_rig_editor::UAIE_SELECTION_SETS_IS_MULTI_ASSET,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<bool>().read() }
    }
    pub fn get_show_and_set_selected_only(&self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig_editor::UAIE_SELECTION_SETS_GET_SHOW_AND_SET_SELECTED_ONLY,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig_editor::UAIE_SELECTION_SETS_GET_SHOW_AND_SET_SELECTED_ONLY,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn get_item_row(
        &self,
        in_guid: &crate::bindings::core_u_object::FGuid,
        out_row: &mut i32,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<21>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig_editor::UAIE_SELECTION_SETS_GET_ITEM_ROW,
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
        unsafe {
            std::ptr::copy_nonoverlapping(out_row, __buffer.add(16).cast::<i32>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig_editor::UAIE_SELECTION_SETS_GET_ITEM_ROW,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(16).cast::<i32>().swap(out_row);
        }
        unsafe { __buffer.add(20).cast::<bool>().read() }
    }
    pub fn get_item_name(
        &self,
        in_guid: &crate::bindings::core_u_object::FGuid,
        out_name: &mut FText,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<33>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig_editor::UAIE_SELECTION_SETS_GET_ITEM_NAME,
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
        unsafe {
            std::ptr::copy_nonoverlapping(out_name, __buffer.add(16).cast::<FText>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig_editor::UAIE_SELECTION_SETS_GET_ITEM_NAME,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(16).cast::<FText>().swap(out_name);
        }
        unsafe { __buffer.add(32).cast::<bool>().read() }
    }
    pub fn get_item_guids(
        &self,
        item_name: &FText,
        out_guids: &mut TArray<crate::bindings::core_u_object::FGuid>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig_editor::UAIE_SELECTION_SETS_GET_ITEM_GUIDS,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(item_name, __buffer.add(0).cast::<FText>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_guids,
                __buffer.add(16).cast::<TArray<crate::bindings::core_u_object::FGuid>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig_editor::UAIE_SELECTION_SETS_GET_ITEM_GUIDS,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(16)
                .cast::<TArray<crate::bindings::core_u_object::FGuid>>()
                .swap(out_guids);
        }
    }
    pub fn get_item_color(
        &self,
        in_guid: &crate::bindings::core_u_object::FGuid,
        out_color: &mut crate::bindings::core_u_object::FLinearColor,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<33>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig_editor::UAIE_SELECTION_SETS_GET_ITEM_COLOR,
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
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_color,
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
                crate::bindings::control_rig_editor::UAIE_SELECTION_SETS_GET_ITEM_COLOR,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(16)
                .cast::<crate::bindings::core_u_object::FLinearColor>()
                .swap(out_color);
        }
        unsafe { __buffer.add(32).cast::<bool>().read() }
    }
    pub fn get_all_actors(&self) -> TArray<UPtr<crate::bindings::engine::AActor>> {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig_editor::UAIE_SELECTION_SETS_GET_ALL_ACTORS,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig_editor::UAIE_SELECTION_SETS_GET_ALL_ACTORS,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(0)
                .cast::<TArray<UPtr<crate::bindings::engine::AActor>>>()
                .read()
        }
    }
    pub fn get_active_selection_sets(
        &mut self,
    ) -> TArray<crate::bindings::core_u_object::FGuid> {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig_editor::UAIE_SELECTION_SETS_GET_ACTIVE_SELECTION_SETS,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig_editor::UAIE_SELECTION_SETS_GET_ACTIVE_SELECTION_SETS,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(0)
                .cast::<TArray<crate::bindings::core_u_object::FGuid>>()
                .read()
        }
    }
    pub fn get_active_actors(&self) -> TArray<UPtr<crate::bindings::engine::AActor>> {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig_editor::UAIE_SELECTION_SETS_GET_ACTIVE_ACTORS,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig_editor::UAIE_SELECTION_SETS_GET_ACTIVE_ACTORS,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(0)
                .cast::<TArray<UPtr<crate::bindings::engine::AActor>>>()
                .read()
        }
    }
    pub fn export_as_json_string(&self, out_json_string: &mut FString) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<17>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig_editor::UAIE_SELECTION_SETS_EXPORT_AS_JSON_STRING,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_json_string,
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
                crate::bindings::control_rig_editor::UAIE_SELECTION_SETS_EXPORT_AS_JSON_STRING,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<FString>().swap(out_json_string);
        }
        unsafe { __buffer.add(16).cast::<bool>().read() }
    }
    pub fn export_as_json_file(
        &self,
        json_file_path: &crate::bindings::core_u_object::FFilePath,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<17>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig_editor::UAIE_SELECTION_SETS_EXPORT_AS_JSON_FILE,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                json_file_path,
                __buffer.add(0).cast::<crate::bindings::core_u_object::FFilePath>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig_editor::UAIE_SELECTION_SETS_EXPORT_AS_JSON_FILE,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<bool>().read() }
    }
    pub fn delete_set_item(
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
                crate::bindings::control_rig_editor::UAIE_SELECTION_SETS_DELETE_SET_ITEM,
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
                crate::bindings::control_rig_editor::UAIE_SELECTION_SETS_DELETE_SET_ITEM,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<bool>().read() }
    }
    pub fn create_set_item_from_selection(
        &mut self,
    ) -> crate::bindings::core_u_object::FGuid {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig_editor::UAIE_SELECTION_SETS_CREATE_SET_ITEM_FROM_SELECTION,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig_editor::UAIE_SELECTION_SETS_CREATE_SET_ITEM_FROM_SELECTION,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<crate::bindings::core_u_object::FGuid>().read() }
    }
    pub fn create_mirror(
        &mut self,
        in_guid: &crate::bindings::core_u_object::FGuid,
    ) -> crate::bindings::core_u_object::FGuid {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig_editor::UAIE_SELECTION_SETS_CREATE_MIRROR,
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
                crate::bindings::control_rig_editor::UAIE_SELECTION_SETS_CREATE_MIRROR,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(16).cast::<crate::bindings::core_u_object::FGuid>().read()
        }
    }
    pub fn add_selection_to_set_item(
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
                crate::bindings::control_rig_editor::UAIE_SELECTION_SETS_ADD_SELECTION_TO_SET_ITEM,
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
                crate::bindings::control_rig_editor::UAIE_SELECTION_SETS_ADD_SELECTION_TO_SET_ITEM,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<bool>().read() }
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
    pub fn setup_all_editor_menus() {
        let mut __stack = crate::core_data::StackAlloc::<0>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig_editor::U_CONTROL_RIG_BLUEPRINT_EDITOR_LIBRARY_SETUP_ALL_EDITOR_MENUS,
                __buffer,
            )
        };
        let __object_ptr = crate::bindings::control_rig_editor::UControlRigBlueprintEditorLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig_editor::U_CONTROL_RIG_BLUEPRINT_EDITOR_LIBRARY_SETUP_ALL_EDITOR_MENUS,
                __buffer,
            )
        };
    }
    pub fn set_preview_mesh(
        in_rig_blueprint: UPtr<
            crate::bindings::control_rig_developer::UControlRigBlueprint,
        >,
        preview_mesh: UPtr<crate::bindings::engine::USkeletalMesh>,
        b_mark_as_dirty: bool,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<17>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig_editor::U_CONTROL_RIG_BLUEPRINT_EDITOR_LIBRARY_SET_PREVIEW_MESH,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_rig_blueprint,
                __buffer
                    .add(0)
                    .cast::<
                        UPtr<
                            crate::bindings::control_rig_developer::UControlRigBlueprint,
                        >,
                    >(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &preview_mesh,
                __buffer.add(8).cast::<UPtr<crate::bindings::engine::USkeletalMesh>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_mark_as_dirty,
                __buffer.add(16).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::control_rig_editor::UControlRigBlueprintEditorLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig_editor::U_CONTROL_RIG_BLUEPRINT_EDITOR_LIBRARY_SET_PREVIEW_MESH,
                __buffer,
            )
        };
    }
    pub fn request_control_rig_init(
        in_rig_blueprint: UPtr<
            crate::bindings::control_rig_developer::UControlRigBlueprint,
        >,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig_editor::U_CONTROL_RIG_BLUEPRINT_EDITOR_LIBRARY_REQUEST_CONTROL_RIG_INIT,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_rig_blueprint,
                __buffer
                    .add(0)
                    .cast::<
                        UPtr<
                            crate::bindings::control_rig_developer::UControlRigBlueprint,
                        >,
                    >(),
                1,
            );
        }
        let __object_ptr = crate::bindings::control_rig_editor::UControlRigBlueprintEditorLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig_editor::U_CONTROL_RIG_BLUEPRINT_EDITOR_LIBRARY_REQUEST_CONTROL_RIG_INIT,
                __buffer,
            )
        };
    }
    pub fn get_preview_mesh(
        in_rig_blueprint: UPtr<
            crate::bindings::control_rig_developer::UControlRigBlueprint,
        >,
    ) -> UPtr<crate::bindings::engine::USkeletalMesh> {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig_editor::U_CONTROL_RIG_BLUEPRINT_EDITOR_LIBRARY_GET_PREVIEW_MESH,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_rig_blueprint,
                __buffer
                    .add(0)
                    .cast::<
                        UPtr<
                            crate::bindings::control_rig_developer::UControlRigBlueprint,
                        >,
                    >(),
                1,
            );
        }
        let __object_ptr = crate::bindings::control_rig_editor::UControlRigBlueprintEditorLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig_editor::U_CONTROL_RIG_BLUEPRINT_EDITOR_LIBRARY_GET_PREVIEW_MESH,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(8).cast::<UPtr<crate::bindings::engine::USkeletalMesh>>().read()
        }
    }
    pub fn get_hierarchy_controller(
        in_rig_blueprint: UPtr<
            crate::bindings::control_rig_developer::UControlRigBlueprint,
        >,
    ) -> UPtr<crate::bindings::control_rig::URigHierarchyController> {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig_editor::U_CONTROL_RIG_BLUEPRINT_EDITOR_LIBRARY_GET_HIERARCHY_CONTROLLER,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_rig_blueprint,
                __buffer
                    .add(0)
                    .cast::<
                        UPtr<
                            crate::bindings::control_rig_developer::UControlRigBlueprint,
                        >,
                    >(),
                1,
            );
        }
        let __object_ptr = crate::bindings::control_rig_editor::UControlRigBlueprintEditorLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig_editor::U_CONTROL_RIG_BLUEPRINT_EDITOR_LIBRARY_GET_HIERARCHY_CONTROLLER,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(8)
                .cast::<UPtr<crate::bindings::control_rig::URigHierarchyController>>()
                .read()
        }
    }
    pub fn get_hierarchy(
        in_rig_blueprint: UPtr<
            crate::bindings::control_rig_developer::UControlRigBlueprint,
        >,
    ) -> UPtr<crate::bindings::control_rig::URigHierarchy> {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig_editor::U_CONTROL_RIG_BLUEPRINT_EDITOR_LIBRARY_GET_HIERARCHY,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_rig_blueprint,
                __buffer
                    .add(0)
                    .cast::<
                        UPtr<
                            crate::bindings::control_rig_developer::UControlRigBlueprint,
                        >,
                    >(),
                1,
            );
        }
        let __object_ptr = crate::bindings::control_rig_editor::UControlRigBlueprintEditorLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig_editor::U_CONTROL_RIG_BLUEPRINT_EDITOR_LIBRARY_GET_HIERARCHY,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(8)
                .cast::<UPtr<crate::bindings::control_rig::URigHierarchy>>()
                .read()
        }
    }
    pub fn get_currently_open_rig_blueprints() -> TArray<
        UPtr<crate::bindings::control_rig_developer::UControlRigBlueprint>,
    > {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig_editor::U_CONTROL_RIG_BLUEPRINT_EDITOR_LIBRARY_GET_CURRENTLY_OPEN_RIG_BLUEPRINTS,
                __buffer,
            )
        };
        let __object_ptr = crate::bindings::control_rig_editor::UControlRigBlueprintEditorLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig_editor::U_CONTROL_RIG_BLUEPRINT_EDITOR_LIBRARY_GET_CURRENTLY_OPEN_RIG_BLUEPRINTS,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(0)
                .cast::<
                    TArray<
                        UPtr<
                            crate::bindings::control_rig_developer::UControlRigBlueprint,
                        >,
                    >,
                >()
                .read()
        }
    }
    pub fn get_available_rig_units() -> TArray<
        UPtr<crate::bindings::core_u_object::UStruct>,
    > {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig_editor::U_CONTROL_RIG_BLUEPRINT_EDITOR_LIBRARY_GET_AVAILABLE_RIG_UNITS,
                __buffer,
            )
        };
        let __object_ptr = crate::bindings::control_rig_editor::UControlRigBlueprintEditorLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig_editor::U_CONTROL_RIG_BLUEPRINT_EDITOR_LIBRARY_GET_AVAILABLE_RIG_UNITS,
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
    pub fn get_available_rig_modules() -> TArray<
        crate::bindings::control_rig::FRigModuleDescription,
    > {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig_editor::U_CONTROL_RIG_BLUEPRINT_EDITOR_LIBRARY_GET_AVAILABLE_RIG_MODULES,
                __buffer,
            )
        };
        let __object_ptr = crate::bindings::control_rig_editor::UControlRigBlueprintEditorLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig_editor::U_CONTROL_RIG_BLUEPRINT_EDITOR_LIBRARY_GET_AVAILABLE_RIG_MODULES,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(0)
                .cast::<TArray<crate::bindings::control_rig::FRigModuleDescription>>()
                .read()
        }
    }
    pub fn cast_to_control_rig_blueprint(
        object: UPtr<crate::bindings::core_u_object::UObject>,
        branches: &mut ECastToControlRigBlueprintCases,
        as_control_rig_blueprint: &mut UPtr<
            crate::bindings::control_rig_developer::UControlRigBlueprint,
        >,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig_editor::U_CONTROL_RIG_BLUEPRINT_EDITOR_LIBRARY_CAST_TO_CONTROL_RIG_BLUEPRINT,
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
                branches,
                __buffer.add(8).cast::<ECastToControlRigBlueprintCases>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                as_control_rig_blueprint,
                __buffer
                    .add(16)
                    .cast::<
                        UPtr<
                            crate::bindings::control_rig_developer::UControlRigBlueprint,
                        >,
                    >(),
                1,
            );
        }
        let __object_ptr = crate::bindings::control_rig_editor::UControlRigBlueprintEditorLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig_editor::U_CONTROL_RIG_BLUEPRINT_EDITOR_LIBRARY_CAST_TO_CONTROL_RIG_BLUEPRINT,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(8).cast::<ECastToControlRigBlueprintCases>().swap(branches);
        }
        unsafe {
            __buffer
                .add(16)
                .cast::<
                    UPtr<crate::bindings::control_rig_developer::UControlRigBlueprint>,
                >()
                .swap(as_control_rig_blueprint);
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
    pub fn create_new_control_rig_asset(
        in_desired_package_path: FString,
        b_modular_rig: bool,
    ) -> UPtr<crate::bindings::control_rig_developer::UControlRigBlueprint> {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig_editor::U_CONTROL_RIG_BLUEPRINT_FACTORY_CREATE_NEW_CONTROL_RIG_ASSET,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_desired_package_path,
                __buffer.add(0).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_modular_rig,
                __buffer.add(16).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::control_rig_editor::UControlRigBlueprintFactory::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig_editor::U_CONTROL_RIG_BLUEPRINT_FACTORY_CREATE_NEW_CONTROL_RIG_ASSET,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(24)
                .cast::<
                    UPtr<crate::bindings::control_rig_developer::UControlRigBlueprint>,
                >()
                .read()
        }
    }
    pub fn create_control_rig_from_skeletal_mesh_or_skeleton(
        in_selected_object: UPtr<crate::bindings::core_u_object::UObject>,
        b_modular_rig: bool,
    ) -> UPtr<crate::bindings::control_rig_developer::UControlRigBlueprint> {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig_editor::U_CONTROL_RIG_BLUEPRINT_FACTORY_CREATE_CONTROL_RIG_FROM_SKELETAL_MESH_OR_SKELETON,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_selected_object,
                __buffer.add(0).cast::<UPtr<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_modular_rig,
                __buffer.add(8).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::control_rig_editor::UControlRigBlueprintFactory::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig_editor::U_CONTROL_RIG_BLUEPRINT_FACTORY_CREATE_CONTROL_RIG_FROM_SKELETAL_MESH_OR_SKELETON,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(16)
                .cast::<
                    UPtr<crate::bindings::control_rig_developer::UControlRigBlueprint>,
                >()
                .read()
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
    pub fn tween_control_rig(
        level_sequence: UPtr<crate::bindings::level_sequence::ULevelSequence>,
        control_rig: UPtr<crate::bindings::control_rig::UControlRig>,
        tween_value: f32,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<21>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig_editor::U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_TWEEN_CONTROL_RIG,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &level_sequence,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::level_sequence::ULevelSequence>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &control_rig,
                __buffer
                    .add(8)
                    .cast::<UPtr<crate::bindings::control_rig::UControlRig>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &tween_value,
                __buffer.add(16).cast::<f32>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::control_rig_editor::UControlRigSequencerEditorLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig_editor::U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_TWEEN_CONTROL_RIG,
                __buffer,
            )
        };
        unsafe { __buffer.add(20).cast::<bool>().read() }
    }
    pub fn space_compensate(
        in_control_rig: UPtr<crate::bindings::control_rig::UControlRig>,
        in_time: crate::bindings::core_u_object::FFrameNumber,
        time_unit: crate::bindings::movie_scene::EMovieSceneTimeUnit,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<14>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig_editor::U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_SPACE_COMPENSATE,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_control_rig,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::control_rig::UControlRig>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_time,
                __buffer.add(8).cast::<crate::bindings::core_u_object::FFrameNumber>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &time_unit,
                __buffer
                    .add(12)
                    .cast::<crate::bindings::movie_scene::EMovieSceneTimeUnit>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::control_rig_editor::UControlRigSequencerEditorLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig_editor::U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_SPACE_COMPENSATE,
                __buffer,
            )
        };
        unsafe { __buffer.add(13).cast::<bool>().read() }
    }
    pub fn snap_control_rig(
        level_sequence: UPtr<crate::bindings::level_sequence::ULevelSequence>,
        start_frame: crate::bindings::core_u_object::FFrameNumber,
        end_frame: crate::bindings::core_u_object::FFrameNumber,
        children_to_snap: &FControlRigSnapperSelection,
        parent_to_snap: &FControlRigSnapperSelection,
        snap_settings: UPtr<UControlRigSnapSettings>,
        time_unit: crate::bindings::movie_scene::EMovieSceneTimeUnit,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<90>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig_editor::U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_SNAP_CONTROL_RIG,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &level_sequence,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::level_sequence::ULevelSequence>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &start_frame,
                __buffer.add(8).cast::<crate::bindings::core_u_object::FFrameNumber>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &end_frame,
                __buffer.add(12).cast::<crate::bindings::core_u_object::FFrameNumber>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                children_to_snap,
                __buffer.add(16).cast::<FControlRigSnapperSelection>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                parent_to_snap,
                __buffer.add(48).cast::<FControlRigSnapperSelection>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &snap_settings,
                __buffer.add(80).cast::<UPtr<UControlRigSnapSettings>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &time_unit,
                __buffer
                    .add(88)
                    .cast::<crate::bindings::movie_scene::EMovieSceneTimeUnit>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::control_rig_editor::UControlRigSequencerEditorLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig_editor::U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_SNAP_CONTROL_RIG,
                __buffer,
            )
        };
        unsafe { __buffer.add(89).cast::<bool>().read() }
    }
    pub fn smart_reduce(
        reduce_params: &mut crate::bindings::curve_editor::FSmartReduceParams,
        movie_scene_section: UPtr<crate::bindings::movie_scene::UMovieSceneSection>,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<25>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig_editor::U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_SMART_REDUCE,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                reduce_params,
                __buffer
                    .add(0)
                    .cast::<crate::bindings::curve_editor::FSmartReduceParams>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &movie_scene_section,
                __buffer
                    .add(16)
                    .cast::<UPtr<crate::bindings::movie_scene::UMovieSceneSection>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::control_rig_editor::UControlRigSequencerEditorLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig_editor::U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_SMART_REDUCE,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(0)
                .cast::<crate::bindings::curve_editor::FSmartReduceParams>()
                .swap(reduce_params);
        }
        unsafe { __buffer.add(24).cast::<bool>().read() }
    }
    pub fn show_all_controls(
        in_section: UPtr<crate::bindings::movie_scene::UMovieSceneSection>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig_editor::U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_SHOW_ALL_CONTROLS,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_section,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::movie_scene::UMovieSceneSection>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::control_rig_editor::UControlRigSequencerEditorLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig_editor::U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_SHOW_ALL_CONTROLS,
                __buffer,
            )
        };
    }
    pub fn set_local_control_rig_vector2_ds(
        level_sequence: UPtr<crate::bindings::level_sequence::ULevelSequence>,
        control_rig: UPtr<crate::bindings::control_rig::UControlRig>,
        control_name: FName,
        frames: &TArray<crate::bindings::core_u_object::FFrameNumber>,
        values: TArray<crate::bindings::core_u_object::FVector2D>,
        time_unit: crate::bindings::movie_scene::EMovieSceneTimeUnit,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<65>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig_editor::U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_SET_LOCAL_CONTROL_RIG_VECTOR2_DS,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &level_sequence,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::level_sequence::ULevelSequence>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &control_rig,
                __buffer
                    .add(8)
                    .cast::<UPtr<crate::bindings::control_rig::UControlRig>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &control_name,
                __buffer.add(16).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                frames,
                __buffer
                    .add(32)
                    .cast::<TArray<crate::bindings::core_u_object::FFrameNumber>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &values,
                __buffer
                    .add(48)
                    .cast::<TArray<crate::bindings::core_u_object::FVector2D>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &time_unit,
                __buffer
                    .add(64)
                    .cast::<crate::bindings::movie_scene::EMovieSceneTimeUnit>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::control_rig_editor::UControlRigSequencerEditorLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig_editor::U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_SET_LOCAL_CONTROL_RIG_VECTOR2_DS,
                __buffer,
            )
        };
    }
    pub fn set_local_control_rig_vector2_d(
        level_sequence: UPtr<crate::bindings::level_sequence::ULevelSequence>,
        control_rig: UPtr<crate::bindings::control_rig::UControlRig>,
        control_name: FName,
        frame: crate::bindings::core_u_object::FFrameNumber,
        value: crate::bindings::core_u_object::FVector2D,
        time_unit: crate::bindings::movie_scene::EMovieSceneTimeUnit,
        b_set_key: bool,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<50>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig_editor::U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_SET_LOCAL_CONTROL_RIG_VECTOR2_D,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &level_sequence,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::level_sequence::ULevelSequence>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &control_rig,
                __buffer
                    .add(8)
                    .cast::<UPtr<crate::bindings::control_rig::UControlRig>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &control_name,
                __buffer.add(16).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &frame,
                __buffer.add(28).cast::<crate::bindings::core_u_object::FFrameNumber>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &value,
                __buffer.add(32).cast::<crate::bindings::core_u_object::FVector2D>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &time_unit,
                __buffer
                    .add(48)
                    .cast::<crate::bindings::movie_scene::EMovieSceneTimeUnit>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_set_key,
                __buffer.add(49).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::control_rig_editor::UControlRigSequencerEditorLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig_editor::U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_SET_LOCAL_CONTROL_RIG_VECTOR2_D,
                __buffer,
            )
        };
    }
    pub fn set_local_control_rig_transforms(
        level_sequence: UPtr<crate::bindings::level_sequence::ULevelSequence>,
        control_rig: UPtr<crate::bindings::control_rig::UControlRig>,
        control_name: FName,
        frames: &TArray<crate::bindings::core_u_object::FFrameNumber>,
        values: TArray<crate::bindings::core_u_object::FTransform>,
        time_unit: crate::bindings::movie_scene::EMovieSceneTimeUnit,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<65>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig_editor::U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_SET_LOCAL_CONTROL_RIG_TRANSFORMS,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &level_sequence,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::level_sequence::ULevelSequence>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &control_rig,
                __buffer
                    .add(8)
                    .cast::<UPtr<crate::bindings::control_rig::UControlRig>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &control_name,
                __buffer.add(16).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                frames,
                __buffer
                    .add(32)
                    .cast::<TArray<crate::bindings::core_u_object::FFrameNumber>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &values,
                __buffer
                    .add(48)
                    .cast::<TArray<crate::bindings::core_u_object::FTransform>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &time_unit,
                __buffer
                    .add(64)
                    .cast::<crate::bindings::movie_scene::EMovieSceneTimeUnit>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::control_rig_editor::UControlRigSequencerEditorLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig_editor::U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_SET_LOCAL_CONTROL_RIG_TRANSFORMS,
                __buffer,
            )
        };
    }
    pub fn set_local_control_rig_transform_no_scales(
        level_sequence: UPtr<crate::bindings::level_sequence::ULevelSequence>,
        control_rig: UPtr<crate::bindings::control_rig::UControlRig>,
        control_name: FName,
        frames: &TArray<crate::bindings::core_u_object::FFrameNumber>,
        values: TArray<crate::bindings::animation_core::FTransformNoScale>,
        time_unit: crate::bindings::movie_scene::EMovieSceneTimeUnit,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<65>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig_editor::U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_SET_LOCAL_CONTROL_RIG_TRANSFORM_NO_SCALES,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &level_sequence,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::level_sequence::ULevelSequence>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &control_rig,
                __buffer
                    .add(8)
                    .cast::<UPtr<crate::bindings::control_rig::UControlRig>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &control_name,
                __buffer.add(16).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                frames,
                __buffer
                    .add(32)
                    .cast::<TArray<crate::bindings::core_u_object::FFrameNumber>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &values,
                __buffer
                    .add(48)
                    .cast::<
                        TArray<crate::bindings::animation_core::FTransformNoScale>,
                    >(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &time_unit,
                __buffer
                    .add(64)
                    .cast::<crate::bindings::movie_scene::EMovieSceneTimeUnit>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::control_rig_editor::UControlRigSequencerEditorLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig_editor::U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_SET_LOCAL_CONTROL_RIG_TRANSFORM_NO_SCALES,
                __buffer,
            )
        };
    }
    pub fn set_local_control_rig_transform_no_scale(
        level_sequence: UPtr<crate::bindings::level_sequence::ULevelSequence>,
        control_rig: UPtr<crate::bindings::control_rig::UControlRig>,
        control_name: FName,
        frame: crate::bindings::core_u_object::FFrameNumber,
        value: crate::bindings::animation_core::FTransformNoScale,
        time_unit: crate::bindings::movie_scene::EMovieSceneTimeUnit,
        b_set_key: bool,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<98>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig_editor::U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_SET_LOCAL_CONTROL_RIG_TRANSFORM_NO_SCALE,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &level_sequence,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::level_sequence::ULevelSequence>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &control_rig,
                __buffer
                    .add(8)
                    .cast::<UPtr<crate::bindings::control_rig::UControlRig>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &control_name,
                __buffer.add(16).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &frame,
                __buffer.add(28).cast::<crate::bindings::core_u_object::FFrameNumber>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &value,
                __buffer
                    .add(32)
                    .cast::<crate::bindings::animation_core::FTransformNoScale>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &time_unit,
                __buffer
                    .add(96)
                    .cast::<crate::bindings::movie_scene::EMovieSceneTimeUnit>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_set_key,
                __buffer.add(97).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::control_rig_editor::UControlRigSequencerEditorLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig_editor::U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_SET_LOCAL_CONTROL_RIG_TRANSFORM_NO_SCALE,
                __buffer,
            )
        };
    }
    pub fn set_local_control_rig_transform(
        level_sequence: UPtr<crate::bindings::level_sequence::ULevelSequence>,
        control_rig: UPtr<crate::bindings::control_rig::UControlRig>,
        control_name: FName,
        frame: crate::bindings::core_u_object::FFrameNumber,
        value: crate::bindings::core_u_object::FTransform,
        time_unit: crate::bindings::movie_scene::EMovieSceneTimeUnit,
        b_set_key: bool,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<130>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig_editor::U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_SET_LOCAL_CONTROL_RIG_TRANSFORM,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &level_sequence,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::level_sequence::ULevelSequence>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &control_rig,
                __buffer
                    .add(8)
                    .cast::<UPtr<crate::bindings::control_rig::UControlRig>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &control_name,
                __buffer.add(16).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &frame,
                __buffer.add(28).cast::<crate::bindings::core_u_object::FFrameNumber>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &value,
                __buffer.add(32).cast::<crate::bindings::core_u_object::FTransform>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &time_unit,
                __buffer
                    .add(128)
                    .cast::<crate::bindings::movie_scene::EMovieSceneTimeUnit>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_set_key,
                __buffer.add(129).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::control_rig_editor::UControlRigSequencerEditorLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig_editor::U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_SET_LOCAL_CONTROL_RIG_TRANSFORM,
                __buffer,
            )
        };
    }
    pub fn set_local_control_rig_scales(
        level_sequence: UPtr<crate::bindings::level_sequence::ULevelSequence>,
        control_rig: UPtr<crate::bindings::control_rig::UControlRig>,
        control_name: FName,
        frames: &TArray<crate::bindings::core_u_object::FFrameNumber>,
        values: TArray<crate::bindings::core_u_object::FVector>,
        time_unit: crate::bindings::movie_scene::EMovieSceneTimeUnit,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<65>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig_editor::U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_SET_LOCAL_CONTROL_RIG_SCALES,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &level_sequence,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::level_sequence::ULevelSequence>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &control_rig,
                __buffer
                    .add(8)
                    .cast::<UPtr<crate::bindings::control_rig::UControlRig>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &control_name,
                __buffer.add(16).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                frames,
                __buffer
                    .add(32)
                    .cast::<TArray<crate::bindings::core_u_object::FFrameNumber>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &values,
                __buffer
                    .add(48)
                    .cast::<TArray<crate::bindings::core_u_object::FVector>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &time_unit,
                __buffer
                    .add(64)
                    .cast::<crate::bindings::movie_scene::EMovieSceneTimeUnit>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::control_rig_editor::UControlRigSequencerEditorLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig_editor::U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_SET_LOCAL_CONTROL_RIG_SCALES,
                __buffer,
            )
        };
    }
    pub fn set_local_control_rig_scale(
        level_sequence: UPtr<crate::bindings::level_sequence::ULevelSequence>,
        control_rig: UPtr<crate::bindings::control_rig::UControlRig>,
        control_name: FName,
        frame: crate::bindings::core_u_object::FFrameNumber,
        value: crate::bindings::core_u_object::FVector,
        time_unit: crate::bindings::movie_scene::EMovieSceneTimeUnit,
        b_set_key: bool,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<58>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig_editor::U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_SET_LOCAL_CONTROL_RIG_SCALE,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &level_sequence,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::level_sequence::ULevelSequence>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &control_rig,
                __buffer
                    .add(8)
                    .cast::<UPtr<crate::bindings::control_rig::UControlRig>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &control_name,
                __buffer.add(16).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &frame,
                __buffer.add(28).cast::<crate::bindings::core_u_object::FFrameNumber>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &value,
                __buffer.add(32).cast::<crate::bindings::core_u_object::FVector>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &time_unit,
                __buffer
                    .add(56)
                    .cast::<crate::bindings::movie_scene::EMovieSceneTimeUnit>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_set_key,
                __buffer.add(57).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::control_rig_editor::UControlRigSequencerEditorLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig_editor::U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_SET_LOCAL_CONTROL_RIG_SCALE,
                __buffer,
            )
        };
    }
    pub fn set_local_control_rig_rotators(
        level_sequence: UPtr<crate::bindings::level_sequence::ULevelSequence>,
        control_rig: UPtr<crate::bindings::control_rig::UControlRig>,
        control_name: FName,
        frames: &TArray<crate::bindings::core_u_object::FFrameNumber>,
        values: TArray<crate::bindings::core_u_object::FRotator>,
        time_unit: crate::bindings::movie_scene::EMovieSceneTimeUnit,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<65>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig_editor::U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_SET_LOCAL_CONTROL_RIG_ROTATORS,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &level_sequence,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::level_sequence::ULevelSequence>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &control_rig,
                __buffer
                    .add(8)
                    .cast::<UPtr<crate::bindings::control_rig::UControlRig>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &control_name,
                __buffer.add(16).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                frames,
                __buffer
                    .add(32)
                    .cast::<TArray<crate::bindings::core_u_object::FFrameNumber>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &values,
                __buffer
                    .add(48)
                    .cast::<TArray<crate::bindings::core_u_object::FRotator>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &time_unit,
                __buffer
                    .add(64)
                    .cast::<crate::bindings::movie_scene::EMovieSceneTimeUnit>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::control_rig_editor::UControlRigSequencerEditorLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig_editor::U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_SET_LOCAL_CONTROL_RIG_ROTATORS,
                __buffer,
            )
        };
    }
    pub fn set_local_control_rig_rotator(
        level_sequence: UPtr<crate::bindings::level_sequence::ULevelSequence>,
        control_rig: UPtr<crate::bindings::control_rig::UControlRig>,
        control_name: FName,
        frame: crate::bindings::core_u_object::FFrameNumber,
        value: crate::bindings::core_u_object::FRotator,
        time_unit: crate::bindings::movie_scene::EMovieSceneTimeUnit,
        b_set_key: bool,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<58>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig_editor::U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_SET_LOCAL_CONTROL_RIG_ROTATOR,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &level_sequence,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::level_sequence::ULevelSequence>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &control_rig,
                __buffer
                    .add(8)
                    .cast::<UPtr<crate::bindings::control_rig::UControlRig>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &control_name,
                __buffer.add(16).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &frame,
                __buffer.add(28).cast::<crate::bindings::core_u_object::FFrameNumber>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &value,
                __buffer.add(32).cast::<crate::bindings::core_u_object::FRotator>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &time_unit,
                __buffer
                    .add(56)
                    .cast::<crate::bindings::movie_scene::EMovieSceneTimeUnit>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_set_key,
                __buffer.add(57).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::control_rig_editor::UControlRigSequencerEditorLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig_editor::U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_SET_LOCAL_CONTROL_RIG_ROTATOR,
                __buffer,
            )
        };
    }
    pub fn set_local_control_rig_positions(
        level_sequence: UPtr<crate::bindings::level_sequence::ULevelSequence>,
        control_rig: UPtr<crate::bindings::control_rig::UControlRig>,
        control_name: FName,
        frames: &TArray<crate::bindings::core_u_object::FFrameNumber>,
        values: TArray<crate::bindings::core_u_object::FVector>,
        time_unit: crate::bindings::movie_scene::EMovieSceneTimeUnit,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<65>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig_editor::U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_SET_LOCAL_CONTROL_RIG_POSITIONS,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &level_sequence,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::level_sequence::ULevelSequence>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &control_rig,
                __buffer
                    .add(8)
                    .cast::<UPtr<crate::bindings::control_rig::UControlRig>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &control_name,
                __buffer.add(16).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                frames,
                __buffer
                    .add(32)
                    .cast::<TArray<crate::bindings::core_u_object::FFrameNumber>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &values,
                __buffer
                    .add(48)
                    .cast::<TArray<crate::bindings::core_u_object::FVector>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &time_unit,
                __buffer
                    .add(64)
                    .cast::<crate::bindings::movie_scene::EMovieSceneTimeUnit>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::control_rig_editor::UControlRigSequencerEditorLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig_editor::U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_SET_LOCAL_CONTROL_RIG_POSITIONS,
                __buffer,
            )
        };
    }
    pub fn set_local_control_rig_position(
        level_sequence: UPtr<crate::bindings::level_sequence::ULevelSequence>,
        control_rig: UPtr<crate::bindings::control_rig::UControlRig>,
        control_name: FName,
        frame: crate::bindings::core_u_object::FFrameNumber,
        value: crate::bindings::core_u_object::FVector,
        time_unit: crate::bindings::movie_scene::EMovieSceneTimeUnit,
        b_set_key: bool,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<58>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig_editor::U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_SET_LOCAL_CONTROL_RIG_POSITION,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &level_sequence,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::level_sequence::ULevelSequence>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &control_rig,
                __buffer
                    .add(8)
                    .cast::<UPtr<crate::bindings::control_rig::UControlRig>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &control_name,
                __buffer.add(16).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &frame,
                __buffer.add(28).cast::<crate::bindings::core_u_object::FFrameNumber>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &value,
                __buffer.add(32).cast::<crate::bindings::core_u_object::FVector>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &time_unit,
                __buffer
                    .add(56)
                    .cast::<crate::bindings::movie_scene::EMovieSceneTimeUnit>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_set_key,
                __buffer.add(57).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::control_rig_editor::UControlRigSequencerEditorLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig_editor::U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_SET_LOCAL_CONTROL_RIG_POSITION,
                __buffer,
            )
        };
    }
    pub fn set_local_control_rig_ints(
        level_sequence: UPtr<crate::bindings::level_sequence::ULevelSequence>,
        control_rig: UPtr<crate::bindings::control_rig::UControlRig>,
        control_name: FName,
        frames: &TArray<crate::bindings::core_u_object::FFrameNumber>,
        values: TArray<i32>,
        time_unit: crate::bindings::movie_scene::EMovieSceneTimeUnit,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<65>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig_editor::U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_SET_LOCAL_CONTROL_RIG_INTS,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &level_sequence,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::level_sequence::ULevelSequence>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &control_rig,
                __buffer
                    .add(8)
                    .cast::<UPtr<crate::bindings::control_rig::UControlRig>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &control_name,
                __buffer.add(16).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                frames,
                __buffer
                    .add(32)
                    .cast::<TArray<crate::bindings::core_u_object::FFrameNumber>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &values,
                __buffer.add(48).cast::<TArray<i32>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &time_unit,
                __buffer
                    .add(64)
                    .cast::<crate::bindings::movie_scene::EMovieSceneTimeUnit>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::control_rig_editor::UControlRigSequencerEditorLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig_editor::U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_SET_LOCAL_CONTROL_RIG_INTS,
                __buffer,
            )
        };
    }
    pub fn set_local_control_rig_int(
        level_sequence: UPtr<crate::bindings::level_sequence::ULevelSequence>,
        control_rig: UPtr<crate::bindings::control_rig::UControlRig>,
        control_name: FName,
        frame: crate::bindings::core_u_object::FFrameNumber,
        value: i32,
        time_unit: crate::bindings::movie_scene::EMovieSceneTimeUnit,
        b_set_key: bool,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<38>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig_editor::U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_SET_LOCAL_CONTROL_RIG_INT,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &level_sequence,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::level_sequence::ULevelSequence>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &control_rig,
                __buffer
                    .add(8)
                    .cast::<UPtr<crate::bindings::control_rig::UControlRig>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &control_name,
                __buffer.add(16).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &frame,
                __buffer.add(28).cast::<crate::bindings::core_u_object::FFrameNumber>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&value, __buffer.add(32).cast::<i32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &time_unit,
                __buffer
                    .add(36)
                    .cast::<crate::bindings::movie_scene::EMovieSceneTimeUnit>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_set_key,
                __buffer.add(37).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::control_rig_editor::UControlRigSequencerEditorLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig_editor::U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_SET_LOCAL_CONTROL_RIG_INT,
                __buffer,
            )
        };
    }
    pub fn set_local_control_rig_floats(
        level_sequence: UPtr<crate::bindings::level_sequence::ULevelSequence>,
        control_rig: UPtr<crate::bindings::control_rig::UControlRig>,
        control_name: FName,
        frames: &TArray<crate::bindings::core_u_object::FFrameNumber>,
        values: TArray<f32>,
        time_unit: crate::bindings::movie_scene::EMovieSceneTimeUnit,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<65>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig_editor::U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_SET_LOCAL_CONTROL_RIG_FLOATS,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &level_sequence,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::level_sequence::ULevelSequence>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &control_rig,
                __buffer
                    .add(8)
                    .cast::<UPtr<crate::bindings::control_rig::UControlRig>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &control_name,
                __buffer.add(16).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                frames,
                __buffer
                    .add(32)
                    .cast::<TArray<crate::bindings::core_u_object::FFrameNumber>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &values,
                __buffer.add(48).cast::<TArray<f32>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &time_unit,
                __buffer
                    .add(64)
                    .cast::<crate::bindings::movie_scene::EMovieSceneTimeUnit>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::control_rig_editor::UControlRigSequencerEditorLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig_editor::U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_SET_LOCAL_CONTROL_RIG_FLOATS,
                __buffer,
            )
        };
    }
    pub fn set_local_control_rig_float(
        level_sequence: UPtr<crate::bindings::level_sequence::ULevelSequence>,
        control_rig: UPtr<crate::bindings::control_rig::UControlRig>,
        control_name: FName,
        frame: crate::bindings::core_u_object::FFrameNumber,
        value: f32,
        time_unit: crate::bindings::movie_scene::EMovieSceneTimeUnit,
        b_set_key: bool,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<38>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig_editor::U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_SET_LOCAL_CONTROL_RIG_FLOAT,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &level_sequence,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::level_sequence::ULevelSequence>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &control_rig,
                __buffer
                    .add(8)
                    .cast::<UPtr<crate::bindings::control_rig::UControlRig>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &control_name,
                __buffer.add(16).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &frame,
                __buffer.add(28).cast::<crate::bindings::core_u_object::FFrameNumber>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&value, __buffer.add(32).cast::<f32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &time_unit,
                __buffer
                    .add(36)
                    .cast::<crate::bindings::movie_scene::EMovieSceneTimeUnit>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_set_key,
                __buffer.add(37).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::control_rig_editor::UControlRigSequencerEditorLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig_editor::U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_SET_LOCAL_CONTROL_RIG_FLOAT,
                __buffer,
            )
        };
    }
    pub fn set_local_control_rig_euler_transforms(
        level_sequence: UPtr<crate::bindings::level_sequence::ULevelSequence>,
        control_rig: UPtr<crate::bindings::control_rig::UControlRig>,
        control_name: FName,
        frames: &TArray<crate::bindings::core_u_object::FFrameNumber>,
        values: TArray<crate::bindings::animation_core::FEulerTransform>,
        time_unit: crate::bindings::movie_scene::EMovieSceneTimeUnit,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<65>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig_editor::U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_SET_LOCAL_CONTROL_RIG_EULER_TRANSFORMS,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &level_sequence,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::level_sequence::ULevelSequence>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &control_rig,
                __buffer
                    .add(8)
                    .cast::<UPtr<crate::bindings::control_rig::UControlRig>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &control_name,
                __buffer.add(16).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                frames,
                __buffer
                    .add(32)
                    .cast::<TArray<crate::bindings::core_u_object::FFrameNumber>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &values,
                __buffer
                    .add(48)
                    .cast::<TArray<crate::bindings::animation_core::FEulerTransform>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &time_unit,
                __buffer
                    .add(64)
                    .cast::<crate::bindings::movie_scene::EMovieSceneTimeUnit>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::control_rig_editor::UControlRigSequencerEditorLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig_editor::U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_SET_LOCAL_CONTROL_RIG_EULER_TRANSFORMS,
                __buffer,
            )
        };
    }
    pub fn set_local_control_rig_euler_transform(
        level_sequence: UPtr<crate::bindings::level_sequence::ULevelSequence>,
        control_rig: UPtr<crate::bindings::control_rig::UControlRig>,
        control_name: FName,
        frame: crate::bindings::core_u_object::FFrameNumber,
        value: crate::bindings::animation_core::FEulerTransform,
        time_unit: crate::bindings::movie_scene::EMovieSceneTimeUnit,
        b_set_key: bool,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<106>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig_editor::U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_SET_LOCAL_CONTROL_RIG_EULER_TRANSFORM,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &level_sequence,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::level_sequence::ULevelSequence>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &control_rig,
                __buffer
                    .add(8)
                    .cast::<UPtr<crate::bindings::control_rig::UControlRig>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &control_name,
                __buffer.add(16).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &frame,
                __buffer.add(28).cast::<crate::bindings::core_u_object::FFrameNumber>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &value,
                __buffer
                    .add(32)
                    .cast::<crate::bindings::animation_core::FEulerTransform>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &time_unit,
                __buffer
                    .add(104)
                    .cast::<crate::bindings::movie_scene::EMovieSceneTimeUnit>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_set_key,
                __buffer.add(105).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::control_rig_editor::UControlRigSequencerEditorLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig_editor::U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_SET_LOCAL_CONTROL_RIG_EULER_TRANSFORM,
                __buffer,
            )
        };
    }
    pub fn set_local_control_rig_bools(
        level_sequence: UPtr<crate::bindings::level_sequence::ULevelSequence>,
        control_rig: UPtr<crate::bindings::control_rig::UControlRig>,
        control_name: FName,
        frames: &TArray<crate::bindings::core_u_object::FFrameNumber>,
        values: TArray<bool>,
        time_unit: crate::bindings::movie_scene::EMovieSceneTimeUnit,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<65>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig_editor::U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_SET_LOCAL_CONTROL_RIG_BOOLS,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &level_sequence,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::level_sequence::ULevelSequence>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &control_rig,
                __buffer
                    .add(8)
                    .cast::<UPtr<crate::bindings::control_rig::UControlRig>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &control_name,
                __buffer.add(16).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                frames,
                __buffer
                    .add(32)
                    .cast::<TArray<crate::bindings::core_u_object::FFrameNumber>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &values,
                __buffer.add(48).cast::<TArray<bool>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &time_unit,
                __buffer
                    .add(64)
                    .cast::<crate::bindings::movie_scene::EMovieSceneTimeUnit>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::control_rig_editor::UControlRigSequencerEditorLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig_editor::U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_SET_LOCAL_CONTROL_RIG_BOOLS,
                __buffer,
            )
        };
    }
    pub fn set_local_control_rig_bool(
        level_sequence: UPtr<crate::bindings::level_sequence::ULevelSequence>,
        control_rig: UPtr<crate::bindings::control_rig::UControlRig>,
        control_name: FName,
        frame: crate::bindings::core_u_object::FFrameNumber,
        value: bool,
        time_unit: crate::bindings::movie_scene::EMovieSceneTimeUnit,
        b_set_key: bool,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<35>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig_editor::U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_SET_LOCAL_CONTROL_RIG_BOOL,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &level_sequence,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::level_sequence::ULevelSequence>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &control_rig,
                __buffer
                    .add(8)
                    .cast::<UPtr<crate::bindings::control_rig::UControlRig>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &control_name,
                __buffer.add(16).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &frame,
                __buffer.add(28).cast::<crate::bindings::core_u_object::FFrameNumber>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&value, __buffer.add(32).cast::<bool>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &time_unit,
                __buffer
                    .add(33)
                    .cast::<crate::bindings::movie_scene::EMovieSceneTimeUnit>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_set_key,
                __buffer.add(34).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::control_rig_editor::UControlRigSequencerEditorLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig_editor::U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_SET_LOCAL_CONTROL_RIG_BOOL,
                __buffer,
            )
        };
    }
    pub fn set_interaction(b_is_interacting: bool) {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig_editor::U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_SET_INTERACTION,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_is_interacting,
                __buffer.add(0).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::control_rig_editor::UControlRigSequencerEditorLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig_editor::U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_SET_INTERACTION,
                __buffer,
            )
        };
    }
    pub fn set_controls_mask(
        in_section: UPtr<crate::bindings::movie_scene::UMovieSceneSection>,
        control_names: &TArray<FName>,
        b_visible: bool,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<25>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig_editor::U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_SET_CONTROLS_MASK,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_section,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::movie_scene::UMovieSceneSection>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                control_names,
                __buffer.add(8).cast::<TArray<FName>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_visible,
                __buffer.add(24).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::control_rig_editor::UControlRigSequencerEditorLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig_editor::U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_SET_CONTROLS_MASK,
                __buffer,
            )
        };
    }
    pub fn set_control_rig_world_transforms(
        level_sequence: UPtr<crate::bindings::level_sequence::ULevelSequence>,
        control_rig: UPtr<crate::bindings::control_rig::UControlRig>,
        control_name: FName,
        frames: &TArray<crate::bindings::core_u_object::FFrameNumber>,
        world_transforms: &TArray<crate::bindings::core_u_object::FTransform>,
        time_unit: crate::bindings::movie_scene::EMovieSceneTimeUnit,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<65>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig_editor::U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_SET_CONTROL_RIG_WORLD_TRANSFORMS,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &level_sequence,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::level_sequence::ULevelSequence>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &control_rig,
                __buffer
                    .add(8)
                    .cast::<UPtr<crate::bindings::control_rig::UControlRig>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &control_name,
                __buffer.add(16).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                frames,
                __buffer
                    .add(32)
                    .cast::<TArray<crate::bindings::core_u_object::FFrameNumber>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                world_transforms,
                __buffer
                    .add(48)
                    .cast::<TArray<crate::bindings::core_u_object::FTransform>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &time_unit,
                __buffer
                    .add(64)
                    .cast::<crate::bindings::movie_scene::EMovieSceneTimeUnit>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::control_rig_editor::UControlRigSequencerEditorLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig_editor::U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_SET_CONTROL_RIG_WORLD_TRANSFORMS,
                __buffer,
            )
        };
    }
    pub fn set_control_rig_world_transform(
        level_sequence: UPtr<crate::bindings::level_sequence::ULevelSequence>,
        control_rig: UPtr<crate::bindings::control_rig::UControlRig>,
        control_name: FName,
        frame: crate::bindings::core_u_object::FFrameNumber,
        world_transform: &crate::bindings::core_u_object::FTransform,
        time_unit: crate::bindings::movie_scene::EMovieSceneTimeUnit,
        b_set_key: bool,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<130>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig_editor::U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_SET_CONTROL_RIG_WORLD_TRANSFORM,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &level_sequence,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::level_sequence::ULevelSequence>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &control_rig,
                __buffer
                    .add(8)
                    .cast::<UPtr<crate::bindings::control_rig::UControlRig>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &control_name,
                __buffer.add(16).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &frame,
                __buffer.add(28).cast::<crate::bindings::core_u_object::FFrameNumber>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                world_transform,
                __buffer.add(32).cast::<crate::bindings::core_u_object::FTransform>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &time_unit,
                __buffer
                    .add(128)
                    .cast::<crate::bindings::movie_scene::EMovieSceneTimeUnit>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_set_key,
                __buffer.add(129).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::control_rig_editor::UControlRigSequencerEditorLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig_editor::U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_SET_CONTROL_RIG_WORLD_TRANSFORM,
                __buffer,
            )
        };
    }
    pub fn set_control_rig_space(
        in_sequence: UPtr<crate::bindings::level_sequence::ULevelSequence>,
        in_control_rig: UPtr<crate::bindings::control_rig::UControlRig>,
        in_control_name: FName,
        in_space_key: &crate::bindings::control_rig::FRigElementKey,
        in_time: crate::bindings::core_u_object::FFrameNumber,
        time_unit: crate::bindings::movie_scene::EMovieSceneTimeUnit,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<50>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig_editor::U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_SET_CONTROL_RIG_SPACE,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_sequence,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::level_sequence::ULevelSequence>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_control_rig,
                __buffer
                    .add(8)
                    .cast::<UPtr<crate::bindings::control_rig::UControlRig>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_control_name,
                __buffer.add(16).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_space_key,
                __buffer.add(28).cast::<crate::bindings::control_rig::FRigElementKey>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_time,
                __buffer.add(44).cast::<crate::bindings::core_u_object::FFrameNumber>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &time_unit,
                __buffer
                    .add(48)
                    .cast::<crate::bindings::movie_scene::EMovieSceneTimeUnit>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::control_rig_editor::UControlRigSequencerEditorLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig_editor::U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_SET_CONTROL_RIG_SPACE,
                __buffer,
            )
        };
        unsafe { __buffer.add(49).cast::<bool>().read() }
    }
    pub fn set_control_rig_priority_order(
        in_section: UPtr<crate::bindings::movie_scene::UMovieSceneTrack>,
        priority_order: i32,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<12>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig_editor::U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_SET_CONTROL_RIG_PRIORITY_ORDER,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_section,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::movie_scene::UMovieSceneTrack>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &priority_order,
                __buffer.add(8).cast::<i32>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::control_rig_editor::UControlRigSequencerEditorLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig_editor::U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_SET_CONTROL_RIG_PRIORITY_ORDER,
                __buffer,
            )
        };
    }
    pub fn set_control_rig_layered_mode(
        in_track: UPtr<
            crate::bindings::control_rig::UMovieSceneControlRigParameterTrack,
        >,
        b_set_is_layered: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<10>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig_editor::U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_SET_CONTROL_RIG_LAYERED_MODE,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_track,
                __buffer
                    .add(0)
                    .cast::<
                        UPtr<
                            crate::bindings::control_rig::UMovieSceneControlRigParameterTrack,
                        >,
                    >(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_set_is_layered,
                __buffer.add(8).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::control_rig_editor::UControlRigSequencerEditorLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig_editor::U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_SET_CONTROL_RIG_LAYERED_MODE,
                __buffer,
            )
        };
        unsafe { __buffer.add(9).cast::<bool>().read() }
    }
    pub fn set_control_rig_apply_mode(
        in_control_rig: UPtr<crate::bindings::control_rig::UControlRig>,
        in_apply_mode: crate::bindings::control_rig::EControlRigFKRigExecuteMode,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<10>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig_editor::U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_SET_CONTROL_RIG_APPLY_MODE,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_control_rig,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::control_rig::UControlRig>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_apply_mode,
                __buffer
                    .add(8)
                    .cast::<crate::bindings::control_rig::EControlRigFKRigExecuteMode>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::control_rig_editor::UControlRigSequencerEditorLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig_editor::U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_SET_CONTROL_RIG_APPLY_MODE,
                __buffer,
            )
        };
        unsafe { __buffer.add(9).cast::<bool>().read() }
    }
    pub fn set_constraint_active_key(
        in_constraint: UPtr<crate::bindings::constraints::UTickableConstraint>,
        b_active: bool,
        in_frame: crate::bindings::core_u_object::FFrameNumber,
        time_unit: crate::bindings::movie_scene::EMovieSceneTimeUnit,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<18>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig_editor::U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_SET_CONSTRAINT_ACTIVE_KEY,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_constraint,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::constraints::UTickableConstraint>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&b_active, __buffer.add(8).cast::<bool>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_frame,
                __buffer.add(12).cast::<crate::bindings::core_u_object::FFrameNumber>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &time_unit,
                __buffer
                    .add(16)
                    .cast::<crate::bindings::movie_scene::EMovieSceneTimeUnit>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::control_rig_editor::UControlRigSequencerEditorLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig_editor::U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_SET_CONSTRAINT_ACTIVE_KEY,
                __buffer,
            )
        };
        unsafe { __buffer.add(17).cast::<bool>().read() }
    }
    pub fn rename_control_rig_control_channels(
        in_sequence: UPtr<crate::bindings::level_sequence::ULevelSequence>,
        in_control_rig: UPtr<crate::bindings::control_rig::UControlRig>,
        in_old_control_names: &TArray<FName>,
        in_new_control_names: &TArray<FName>,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<49>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig_editor::U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_RENAME_CONTROL_RIG_CONTROL_CHANNELS,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_sequence,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::level_sequence::ULevelSequence>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_control_rig,
                __buffer
                    .add(8)
                    .cast::<UPtr<crate::bindings::control_rig::UControlRig>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_old_control_names,
                __buffer.add(16).cast::<TArray<FName>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_new_control_names,
                __buffer.add(32).cast::<TArray<FName>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::control_rig_editor::UControlRigSequencerEditorLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig_editor::U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_RENAME_CONTROL_RIG_CONTROL_CHANNELS,
                __buffer,
            )
        };
        unsafe { __buffer.add(48).cast::<bool>().read() }
    }
    pub fn move_control_rig_space(
        in_sequence: UPtr<crate::bindings::level_sequence::ULevelSequence>,
        in_control_rig: UPtr<crate::bindings::control_rig::UControlRig>,
        in_control_name: FName,
        in_time: crate::bindings::core_u_object::FFrameNumber,
        in_new_time: crate::bindings::core_u_object::FFrameNumber,
        time_unit: crate::bindings::movie_scene::EMovieSceneTimeUnit,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<38>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig_editor::U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_MOVE_CONTROL_RIG_SPACE,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_sequence,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::level_sequence::ULevelSequence>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_control_rig,
                __buffer
                    .add(8)
                    .cast::<UPtr<crate::bindings::control_rig::UControlRig>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_control_name,
                __buffer.add(16).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_time,
                __buffer.add(28).cast::<crate::bindings::core_u_object::FFrameNumber>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_new_time,
                __buffer.add(32).cast::<crate::bindings::core_u_object::FFrameNumber>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &time_unit,
                __buffer
                    .add(36)
                    .cast::<crate::bindings::movie_scene::EMovieSceneTimeUnit>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::control_rig_editor::UControlRigSequencerEditorLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig_editor::U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_MOVE_CONTROL_RIG_SPACE,
                __buffer,
            )
        };
        unsafe { __buffer.add(37).cast::<bool>().read() }
    }
    pub fn move_constraint_key(
        constraint: UPtr<crate::bindings::constraints::UTickableConstraint>,
        constraint_section: UPtr<crate::bindings::movie_scene::UMovieSceneSection>,
        in_time: crate::bindings::core_u_object::FFrameNumber,
        in_new_time: crate::bindings::core_u_object::FFrameNumber,
        time_unit: crate::bindings::movie_scene::EMovieSceneTimeUnit,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<26>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig_editor::U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_MOVE_CONSTRAINT_KEY,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &constraint,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::constraints::UTickableConstraint>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &constraint_section,
                __buffer
                    .add(8)
                    .cast::<UPtr<crate::bindings::movie_scene::UMovieSceneSection>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_time,
                __buffer.add(16).cast::<crate::bindings::core_u_object::FFrameNumber>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_new_time,
                __buffer.add(20).cast::<crate::bindings::core_u_object::FFrameNumber>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &time_unit,
                __buffer
                    .add(24)
                    .cast::<crate::bindings::movie_scene::EMovieSceneTimeUnit>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::control_rig_editor::UControlRigSequencerEditorLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig_editor::U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_MOVE_CONSTRAINT_KEY,
                __buffer,
            )
        };
        unsafe { __buffer.add(25).cast::<bool>().read() }
    }
    pub fn merge_anim_layers_with_settings(
        indices: &TArray<i32>,
        merge_anim_layer_settings: &FMergeAnimLayerSettings,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<33>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig_editor::U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_MERGE_ANIM_LAYERS_WITH_SETTINGS,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                indices,
                __buffer.add(0).cast::<TArray<i32>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                merge_anim_layer_settings,
                __buffer.add(16).cast::<FMergeAnimLayerSettings>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::control_rig_editor::UControlRigSequencerEditorLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig_editor::U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_MERGE_ANIM_LAYERS_WITH_SETTINGS,
                __buffer,
            )
        };
        unsafe { __buffer.add(32).cast::<bool>().read() }
    }
    pub fn merge_anim_layers(indices: &TArray<i32>) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<17>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig_editor::U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_MERGE_ANIM_LAYERS,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                indices,
                __buffer.add(0).cast::<TArray<i32>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::control_rig_editor::UControlRigSequencerEditorLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig_editor::U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_MERGE_ANIM_LAYERS,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<bool>().read() }
    }
    pub fn load_anim_sequence_into_control_rig_section_with_range(
        movie_scene_section: UPtr<crate::bindings::movie_scene::UMovieSceneSection>,
        anim_sequence: UPtr<crate::bindings::engine::UAnimSequence>,
        skel_mesh_comp: UPtr<crate::bindings::engine::USkeletalMeshComponent>,
        in_start_frame: crate::bindings::core_u_object::FFrameNumber,
        b_use_custom_anim_range: bool,
        anim_start_range: crate::bindings::core_u_object::FFrameNumber,
        anim_end_range: crate::bindings::core_u_object::FFrameNumber,
        time_unit: crate::bindings::movie_scene::EMovieSceneTimeUnit,
        b_key_reduce: bool,
        tolerance: f32,
        interpolation: crate::bindings::movie_scene::EMovieSceneKeyInterpolation,
        b_reset_controls: bool,
        b_onto_selected_controls: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<52>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig_editor::U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_LOAD_ANIM_SEQUENCE_INTO_CONTROL_RIG_SECTION_WITH_RANGE,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &movie_scene_section,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::movie_scene::UMovieSceneSection>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &anim_sequence,
                __buffer.add(8).cast::<UPtr<crate::bindings::engine::UAnimSequence>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &skel_mesh_comp,
                __buffer
                    .add(16)
                    .cast::<UPtr<crate::bindings::engine::USkeletalMeshComponent>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_start_frame,
                __buffer.add(24).cast::<crate::bindings::core_u_object::FFrameNumber>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_use_custom_anim_range,
                __buffer.add(28).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &anim_start_range,
                __buffer.add(32).cast::<crate::bindings::core_u_object::FFrameNumber>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &anim_end_range,
                __buffer.add(36).cast::<crate::bindings::core_u_object::FFrameNumber>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &time_unit,
                __buffer
                    .add(40)
                    .cast::<crate::bindings::movie_scene::EMovieSceneTimeUnit>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_key_reduce,
                __buffer.add(41).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&tolerance, __buffer.add(44).cast::<f32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &interpolation,
                __buffer
                    .add(48)
                    .cast::<crate::bindings::movie_scene::EMovieSceneKeyInterpolation>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_reset_controls,
                __buffer.add(49).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_onto_selected_controls,
                __buffer.add(50).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::control_rig_editor::UControlRigSequencerEditorLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig_editor::U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_LOAD_ANIM_SEQUENCE_INTO_CONTROL_RIG_SECTION_WITH_RANGE,
                __buffer,
            )
        };
        unsafe { __buffer.add(51).cast::<bool>().read() }
    }
    pub fn load_anim_sequence_into_control_rig_section(
        movie_scene_section: UPtr<crate::bindings::movie_scene::UMovieSceneSection>,
        anim_sequence: UPtr<crate::bindings::engine::UAnimSequence>,
        skel_mesh_comp: UPtr<crate::bindings::engine::USkeletalMeshComponent>,
        in_start_frame: crate::bindings::core_u_object::FFrameNumber,
        time_unit: crate::bindings::movie_scene::EMovieSceneTimeUnit,
        b_key_reduce: bool,
        tolerance: f32,
        interpolation: crate::bindings::movie_scene::EMovieSceneKeyInterpolation,
        b_reset_controls: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<39>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig_editor::U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_LOAD_ANIM_SEQUENCE_INTO_CONTROL_RIG_SECTION,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &movie_scene_section,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::movie_scene::UMovieSceneSection>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &anim_sequence,
                __buffer.add(8).cast::<UPtr<crate::bindings::engine::UAnimSequence>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &skel_mesh_comp,
                __buffer
                    .add(16)
                    .cast::<UPtr<crate::bindings::engine::USkeletalMeshComponent>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_start_frame,
                __buffer.add(24).cast::<crate::bindings::core_u_object::FFrameNumber>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &time_unit,
                __buffer
                    .add(28)
                    .cast::<crate::bindings::movie_scene::EMovieSceneTimeUnit>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_key_reduce,
                __buffer.add(29).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&tolerance, __buffer.add(32).cast::<f32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &interpolation,
                __buffer
                    .add(36)
                    .cast::<crate::bindings::movie_scene::EMovieSceneKeyInterpolation>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_reset_controls,
                __buffer.add(37).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::control_rig_editor::UControlRigSequencerEditorLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig_editor::U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_LOAD_ANIM_SEQUENCE_INTO_CONTROL_RIG_SECTION,
                __buffer,
            )
        };
        unsafe { __buffer.add(38).cast::<bool>().read() }
    }
    pub fn is_layered_control_rig(
        in_control_rig: UPtr<crate::bindings::control_rig::UControlRig>,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<9>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig_editor::U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_IS_LAYERED_CONTROL_RIG,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_control_rig,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::control_rig::UControlRig>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::control_rig_editor::UControlRigSequencerEditorLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig_editor::U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_IS_LAYERED_CONTROL_RIG,
                __buffer,
            )
        };
        unsafe { __buffer.add(8).cast::<bool>().read() }
    }
    pub fn is_fk_control_rig(
        in_control_rig: UPtr<crate::bindings::control_rig::UControlRig>,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<9>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig_editor::U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_IS_FK_CONTROL_RIG,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_control_rig,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::control_rig::UControlRig>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::control_rig_editor::UControlRigSequencerEditorLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig_editor::U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_IS_FK_CONTROL_RIG,
                __buffer,
            )
        };
        unsafe { __buffer.add(8).cast::<bool>().read() }
    }
    pub fn import_fbx_to_control_rig_track(
        world: UPtr<crate::bindings::engine::UWorld>,
        in_sequence: UPtr<crate::bindings::level_sequence::ULevelSequence>,
        in_track: UPtr<
            crate::bindings::control_rig::UMovieSceneControlRigParameterTrack,
        >,
        in_section: UPtr<
            crate::bindings::control_rig::UMovieSceneControlRigParameterSection,
        >,
        selected_control_rig_names: &TArray<FString>,
        import_fbx_control_rig_settings: UPtr<
            crate::bindings::movie_scene_tools::UMovieSceneUserImportFBXControlRigSettings,
        >,
        import_filename: FString,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<73>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig_editor::U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_IMPORT_FBX_TO_CONTROL_RIG_TRACK,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &world,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::UWorld>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_sequence,
                __buffer
                    .add(8)
                    .cast::<UPtr<crate::bindings::level_sequence::ULevelSequence>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_track,
                __buffer
                    .add(16)
                    .cast::<
                        UPtr<
                            crate::bindings::control_rig::UMovieSceneControlRigParameterTrack,
                        >,
                    >(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_section,
                __buffer
                    .add(24)
                    .cast::<
                        UPtr<
                            crate::bindings::control_rig::UMovieSceneControlRigParameterSection,
                        >,
                    >(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                selected_control_rig_names,
                __buffer.add(32).cast::<TArray<FString>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &import_fbx_control_rig_settings,
                __buffer
                    .add(48)
                    .cast::<
                        UPtr<
                            crate::bindings::movie_scene_tools::UMovieSceneUserImportFBXControlRigSettings,
                        >,
                    >(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &import_filename,
                __buffer.add(56).cast::<FString>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::control_rig_editor::UControlRigSequencerEditorLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig_editor::U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_IMPORT_FBX_TO_CONTROL_RIG_TRACK,
                __buffer,
            )
        };
        unsafe { __buffer.add(72).cast::<bool>().read() }
    }
    pub fn hide_all_controls(
        in_section: UPtr<crate::bindings::movie_scene::UMovieSceneSection>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig_editor::U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_HIDE_ALL_CONTROLS,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_section,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::movie_scene::UMovieSceneSection>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::control_rig_editor::UControlRigSequencerEditorLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig_editor::U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_HIDE_ALL_CONTROLS,
                __buffer,
            )
        };
    }
    pub fn get_world_space_reference_key() -> crate::bindings::control_rig::FRigElementKey {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig_editor::U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_GET_WORLD_SPACE_REFERENCE_KEY,
                __buffer,
            )
        };
        let __object_ptr = crate::bindings::control_rig_editor::UControlRigSequencerEditorLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig_editor::U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_GET_WORLD_SPACE_REFERENCE_KEY,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<crate::bindings::control_rig::FRigElementKey>().read()
        }
    }
    pub fn get_visible_control_rigs() -> TArray<
        UPtr<crate::bindings::control_rig::UControlRig>,
    > {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig_editor::U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_GET_VISIBLE_CONTROL_RIGS,
                __buffer,
            )
        };
        let __object_ptr = crate::bindings::control_rig_editor::UControlRigSequencerEditorLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig_editor::U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_GET_VISIBLE_CONTROL_RIGS,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(0)
                .cast::<TArray<UPtr<crate::bindings::control_rig::UControlRig>>>()
                .read()
        }
    }
    pub fn get_skeletal_mesh_component_world_transforms(
        level_sequence: UPtr<crate::bindings::level_sequence::ULevelSequence>,
        skeletal_mesh_component: UPtr<crate::bindings::engine::USkeletalMeshComponent>,
        frames: &TArray<crate::bindings::core_u_object::FFrameNumber>,
        time_unit: crate::bindings::movie_scene::EMovieSceneTimeUnit,
        reference_name: FName,
    ) -> TArray<crate::bindings::core_u_object::FTransform> {
        let mut __stack = crate::core_data::StackAlloc::<64>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig_editor::U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_GET_SKELETAL_MESH_COMPONENT_WORLD_TRANSFORMS,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &level_sequence,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::level_sequence::ULevelSequence>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &skeletal_mesh_component,
                __buffer
                    .add(8)
                    .cast::<UPtr<crate::bindings::engine::USkeletalMeshComponent>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                frames,
                __buffer
                    .add(16)
                    .cast::<TArray<crate::bindings::core_u_object::FFrameNumber>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &time_unit,
                __buffer
                    .add(32)
                    .cast::<crate::bindings::movie_scene::EMovieSceneTimeUnit>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &reference_name,
                __buffer.add(36).cast::<FName>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::control_rig_editor::UControlRigSequencerEditorLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig_editor::U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_GET_SKELETAL_MESH_COMPONENT_WORLD_TRANSFORMS,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(48)
                .cast::<TArray<crate::bindings::core_u_object::FTransform>>()
                .read()
        }
    }
    pub fn get_skeletal_mesh_component_world_transform(
        level_sequence: UPtr<crate::bindings::level_sequence::ULevelSequence>,
        skeletal_mesh_component: UPtr<crate::bindings::engine::USkeletalMeshComponent>,
        frame: crate::bindings::core_u_object::FFrameNumber,
        time_unit: crate::bindings::movie_scene::EMovieSceneTimeUnit,
        reference_name: FName,
    ) -> crate::bindings::core_u_object::FTransform {
        let mut __stack = crate::core_data::StackAlloc::<144>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig_editor::U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_GET_SKELETAL_MESH_COMPONENT_WORLD_TRANSFORM,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &level_sequence,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::level_sequence::ULevelSequence>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &skeletal_mesh_component,
                __buffer
                    .add(8)
                    .cast::<UPtr<crate::bindings::engine::USkeletalMeshComponent>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &frame,
                __buffer.add(16).cast::<crate::bindings::core_u_object::FFrameNumber>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &time_unit,
                __buffer
                    .add(20)
                    .cast::<crate::bindings::movie_scene::EMovieSceneTimeUnit>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &reference_name,
                __buffer.add(24).cast::<FName>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::control_rig_editor::UControlRigSequencerEditorLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig_editor::U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_GET_SKELETAL_MESH_COMPONENT_WORLD_TRANSFORM,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(48).cast::<crate::bindings::core_u_object::FTransform>().read()
        }
    }
    pub fn get_selection_sets(b_add_if_does_not_exist: bool) -> UPtr<UAIESelectionSets> {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig_editor::U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_GET_SELECTION_SETS,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_add_if_does_not_exist,
                __buffer.add(0).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::control_rig_editor::UControlRigSequencerEditorLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig_editor::U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_GET_SELECTION_SETS,
                __buffer,
            )
        };
        unsafe { __buffer.add(8).cast::<UPtr<UAIESelectionSets>>().read() }
    }
    pub fn get_local_control_rig_vector2_ds(
        level_sequence: UPtr<crate::bindings::level_sequence::ULevelSequence>,
        control_rig: UPtr<crate::bindings::control_rig::UControlRig>,
        control_name: FName,
        frames: &TArray<crate::bindings::core_u_object::FFrameNumber>,
        time_unit: crate::bindings::movie_scene::EMovieSceneTimeUnit,
    ) -> TArray<crate::bindings::core_u_object::FVector2D> {
        let mut __stack = crate::core_data::StackAlloc::<72>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig_editor::U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_GET_LOCAL_CONTROL_RIG_VECTOR2_DS,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &level_sequence,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::level_sequence::ULevelSequence>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &control_rig,
                __buffer
                    .add(8)
                    .cast::<UPtr<crate::bindings::control_rig::UControlRig>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &control_name,
                __buffer.add(16).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                frames,
                __buffer
                    .add(32)
                    .cast::<TArray<crate::bindings::core_u_object::FFrameNumber>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &time_unit,
                __buffer
                    .add(48)
                    .cast::<crate::bindings::movie_scene::EMovieSceneTimeUnit>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::control_rig_editor::UControlRigSequencerEditorLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig_editor::U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_GET_LOCAL_CONTROL_RIG_VECTOR2_DS,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(56)
                .cast::<TArray<crate::bindings::core_u_object::FVector2D>>()
                .read()
        }
    }
    pub fn get_local_control_rig_vector2_d(
        level_sequence: UPtr<crate::bindings::level_sequence::ULevelSequence>,
        control_rig: UPtr<crate::bindings::control_rig::UControlRig>,
        control_name: FName,
        frame: crate::bindings::core_u_object::FFrameNumber,
        time_unit: crate::bindings::movie_scene::EMovieSceneTimeUnit,
    ) -> crate::bindings::core_u_object::FVector2D {
        let mut __stack = crate::core_data::StackAlloc::<56>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig_editor::U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_GET_LOCAL_CONTROL_RIG_VECTOR2_D,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &level_sequence,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::level_sequence::ULevelSequence>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &control_rig,
                __buffer
                    .add(8)
                    .cast::<UPtr<crate::bindings::control_rig::UControlRig>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &control_name,
                __buffer.add(16).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &frame,
                __buffer.add(28).cast::<crate::bindings::core_u_object::FFrameNumber>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &time_unit,
                __buffer
                    .add(32)
                    .cast::<crate::bindings::movie_scene::EMovieSceneTimeUnit>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::control_rig_editor::UControlRigSequencerEditorLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig_editor::U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_GET_LOCAL_CONTROL_RIG_VECTOR2_D,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(40).cast::<crate::bindings::core_u_object::FVector2D>().read()
        }
    }
    pub fn get_local_control_rig_transforms(
        level_sequence: UPtr<crate::bindings::level_sequence::ULevelSequence>,
        control_rig: UPtr<crate::bindings::control_rig::UControlRig>,
        control_name: FName,
        frames: &TArray<crate::bindings::core_u_object::FFrameNumber>,
        time_unit: crate::bindings::movie_scene::EMovieSceneTimeUnit,
    ) -> TArray<crate::bindings::core_u_object::FTransform> {
        let mut __stack = crate::core_data::StackAlloc::<72>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig_editor::U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_GET_LOCAL_CONTROL_RIG_TRANSFORMS,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &level_sequence,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::level_sequence::ULevelSequence>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &control_rig,
                __buffer
                    .add(8)
                    .cast::<UPtr<crate::bindings::control_rig::UControlRig>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &control_name,
                __buffer.add(16).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                frames,
                __buffer
                    .add(32)
                    .cast::<TArray<crate::bindings::core_u_object::FFrameNumber>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &time_unit,
                __buffer
                    .add(48)
                    .cast::<crate::bindings::movie_scene::EMovieSceneTimeUnit>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::control_rig_editor::UControlRigSequencerEditorLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig_editor::U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_GET_LOCAL_CONTROL_RIG_TRANSFORMS,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(56)
                .cast::<TArray<crate::bindings::core_u_object::FTransform>>()
                .read()
        }
    }
    pub fn get_local_control_rig_transform_no_scales(
        level_sequence: UPtr<crate::bindings::level_sequence::ULevelSequence>,
        control_rig: UPtr<crate::bindings::control_rig::UControlRig>,
        control_name: FName,
        frames: &TArray<crate::bindings::core_u_object::FFrameNumber>,
        time_unit: crate::bindings::movie_scene::EMovieSceneTimeUnit,
    ) -> TArray<crate::bindings::animation_core::FTransformNoScale> {
        let mut __stack = crate::core_data::StackAlloc::<72>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig_editor::U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_GET_LOCAL_CONTROL_RIG_TRANSFORM_NO_SCALES,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &level_sequence,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::level_sequence::ULevelSequence>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &control_rig,
                __buffer
                    .add(8)
                    .cast::<UPtr<crate::bindings::control_rig::UControlRig>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &control_name,
                __buffer.add(16).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                frames,
                __buffer
                    .add(32)
                    .cast::<TArray<crate::bindings::core_u_object::FFrameNumber>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &time_unit,
                __buffer
                    .add(48)
                    .cast::<crate::bindings::movie_scene::EMovieSceneTimeUnit>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::control_rig_editor::UControlRigSequencerEditorLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig_editor::U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_GET_LOCAL_CONTROL_RIG_TRANSFORM_NO_SCALES,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(56)
                .cast::<TArray<crate::bindings::animation_core::FTransformNoScale>>()
                .read()
        }
    }
    pub fn get_local_control_rig_transform_no_scale(
        level_sequence: UPtr<crate::bindings::level_sequence::ULevelSequence>,
        control_rig: UPtr<crate::bindings::control_rig::UControlRig>,
        control_name: FName,
        frame: crate::bindings::core_u_object::FFrameNumber,
        time_unit: crate::bindings::movie_scene::EMovieSceneTimeUnit,
    ) -> crate::bindings::animation_core::FTransformNoScale {
        let mut __stack = crate::core_data::StackAlloc::<112>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig_editor::U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_GET_LOCAL_CONTROL_RIG_TRANSFORM_NO_SCALE,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &level_sequence,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::level_sequence::ULevelSequence>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &control_rig,
                __buffer
                    .add(8)
                    .cast::<UPtr<crate::bindings::control_rig::UControlRig>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &control_name,
                __buffer.add(16).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &frame,
                __buffer.add(28).cast::<crate::bindings::core_u_object::FFrameNumber>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &time_unit,
                __buffer
                    .add(32)
                    .cast::<crate::bindings::movie_scene::EMovieSceneTimeUnit>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::control_rig_editor::UControlRigSequencerEditorLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig_editor::U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_GET_LOCAL_CONTROL_RIG_TRANSFORM_NO_SCALE,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(48)
                .cast::<crate::bindings::animation_core::FTransformNoScale>()
                .read()
        }
    }
    pub fn get_local_control_rig_transform(
        level_sequence: UPtr<crate::bindings::level_sequence::ULevelSequence>,
        control_rig: UPtr<crate::bindings::control_rig::UControlRig>,
        control_name: FName,
        frame: crate::bindings::core_u_object::FFrameNumber,
        time_unit: crate::bindings::movie_scene::EMovieSceneTimeUnit,
    ) -> crate::bindings::core_u_object::FTransform {
        let mut __stack = crate::core_data::StackAlloc::<144>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig_editor::U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_GET_LOCAL_CONTROL_RIG_TRANSFORM,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &level_sequence,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::level_sequence::ULevelSequence>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &control_rig,
                __buffer
                    .add(8)
                    .cast::<UPtr<crate::bindings::control_rig::UControlRig>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &control_name,
                __buffer.add(16).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &frame,
                __buffer.add(28).cast::<crate::bindings::core_u_object::FFrameNumber>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &time_unit,
                __buffer
                    .add(32)
                    .cast::<crate::bindings::movie_scene::EMovieSceneTimeUnit>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::control_rig_editor::UControlRigSequencerEditorLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig_editor::U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_GET_LOCAL_CONTROL_RIG_TRANSFORM,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(48).cast::<crate::bindings::core_u_object::FTransform>().read()
        }
    }
    pub fn get_local_control_rig_scales(
        level_sequence: UPtr<crate::bindings::level_sequence::ULevelSequence>,
        control_rig: UPtr<crate::bindings::control_rig::UControlRig>,
        control_name: FName,
        frames: &TArray<crate::bindings::core_u_object::FFrameNumber>,
        time_unit: crate::bindings::movie_scene::EMovieSceneTimeUnit,
    ) -> TArray<crate::bindings::core_u_object::FVector> {
        let mut __stack = crate::core_data::StackAlloc::<72>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig_editor::U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_GET_LOCAL_CONTROL_RIG_SCALES,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &level_sequence,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::level_sequence::ULevelSequence>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &control_rig,
                __buffer
                    .add(8)
                    .cast::<UPtr<crate::bindings::control_rig::UControlRig>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &control_name,
                __buffer.add(16).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                frames,
                __buffer
                    .add(32)
                    .cast::<TArray<crate::bindings::core_u_object::FFrameNumber>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &time_unit,
                __buffer
                    .add(48)
                    .cast::<crate::bindings::movie_scene::EMovieSceneTimeUnit>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::control_rig_editor::UControlRigSequencerEditorLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig_editor::U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_GET_LOCAL_CONTROL_RIG_SCALES,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(56)
                .cast::<TArray<crate::bindings::core_u_object::FVector>>()
                .read()
        }
    }
    pub fn get_local_control_rig_scale(
        level_sequence: UPtr<crate::bindings::level_sequence::ULevelSequence>,
        control_rig: UPtr<crate::bindings::control_rig::UControlRig>,
        control_name: FName,
        frame: crate::bindings::core_u_object::FFrameNumber,
        time_unit: crate::bindings::movie_scene::EMovieSceneTimeUnit,
    ) -> crate::bindings::core_u_object::FVector {
        let mut __stack = crate::core_data::StackAlloc::<64>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig_editor::U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_GET_LOCAL_CONTROL_RIG_SCALE,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &level_sequence,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::level_sequence::ULevelSequence>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &control_rig,
                __buffer
                    .add(8)
                    .cast::<UPtr<crate::bindings::control_rig::UControlRig>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &control_name,
                __buffer.add(16).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &frame,
                __buffer.add(28).cast::<crate::bindings::core_u_object::FFrameNumber>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &time_unit,
                __buffer
                    .add(32)
                    .cast::<crate::bindings::movie_scene::EMovieSceneTimeUnit>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::control_rig_editor::UControlRigSequencerEditorLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig_editor::U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_GET_LOCAL_CONTROL_RIG_SCALE,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(40).cast::<crate::bindings::core_u_object::FVector>().read()
        }
    }
    pub fn get_local_control_rig_rotators(
        level_sequence: UPtr<crate::bindings::level_sequence::ULevelSequence>,
        control_rig: UPtr<crate::bindings::control_rig::UControlRig>,
        control_name: FName,
        frames: &TArray<crate::bindings::core_u_object::FFrameNumber>,
        time_unit: crate::bindings::movie_scene::EMovieSceneTimeUnit,
    ) -> TArray<crate::bindings::core_u_object::FRotator> {
        let mut __stack = crate::core_data::StackAlloc::<72>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig_editor::U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_GET_LOCAL_CONTROL_RIG_ROTATORS,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &level_sequence,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::level_sequence::ULevelSequence>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &control_rig,
                __buffer
                    .add(8)
                    .cast::<UPtr<crate::bindings::control_rig::UControlRig>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &control_name,
                __buffer.add(16).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                frames,
                __buffer
                    .add(32)
                    .cast::<TArray<crate::bindings::core_u_object::FFrameNumber>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &time_unit,
                __buffer
                    .add(48)
                    .cast::<crate::bindings::movie_scene::EMovieSceneTimeUnit>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::control_rig_editor::UControlRigSequencerEditorLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig_editor::U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_GET_LOCAL_CONTROL_RIG_ROTATORS,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(56)
                .cast::<TArray<crate::bindings::core_u_object::FRotator>>()
                .read()
        }
    }
    pub fn get_local_control_rig_rotator(
        level_sequence: UPtr<crate::bindings::level_sequence::ULevelSequence>,
        control_rig: UPtr<crate::bindings::control_rig::UControlRig>,
        control_name: FName,
        frame: crate::bindings::core_u_object::FFrameNumber,
        time_unit: crate::bindings::movie_scene::EMovieSceneTimeUnit,
    ) -> crate::bindings::core_u_object::FRotator {
        let mut __stack = crate::core_data::StackAlloc::<64>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig_editor::U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_GET_LOCAL_CONTROL_RIG_ROTATOR,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &level_sequence,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::level_sequence::ULevelSequence>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &control_rig,
                __buffer
                    .add(8)
                    .cast::<UPtr<crate::bindings::control_rig::UControlRig>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &control_name,
                __buffer.add(16).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &frame,
                __buffer.add(28).cast::<crate::bindings::core_u_object::FFrameNumber>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &time_unit,
                __buffer
                    .add(32)
                    .cast::<crate::bindings::movie_scene::EMovieSceneTimeUnit>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::control_rig_editor::UControlRigSequencerEditorLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig_editor::U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_GET_LOCAL_CONTROL_RIG_ROTATOR,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(40).cast::<crate::bindings::core_u_object::FRotator>().read()
        }
    }
    pub fn get_local_control_rig_positions(
        level_sequence: UPtr<crate::bindings::level_sequence::ULevelSequence>,
        control_rig: UPtr<crate::bindings::control_rig::UControlRig>,
        control_name: FName,
        frames: &TArray<crate::bindings::core_u_object::FFrameNumber>,
        time_unit: crate::bindings::movie_scene::EMovieSceneTimeUnit,
    ) -> TArray<crate::bindings::core_u_object::FVector> {
        let mut __stack = crate::core_data::StackAlloc::<72>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig_editor::U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_GET_LOCAL_CONTROL_RIG_POSITIONS,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &level_sequence,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::level_sequence::ULevelSequence>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &control_rig,
                __buffer
                    .add(8)
                    .cast::<UPtr<crate::bindings::control_rig::UControlRig>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &control_name,
                __buffer.add(16).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                frames,
                __buffer
                    .add(32)
                    .cast::<TArray<crate::bindings::core_u_object::FFrameNumber>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &time_unit,
                __buffer
                    .add(48)
                    .cast::<crate::bindings::movie_scene::EMovieSceneTimeUnit>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::control_rig_editor::UControlRigSequencerEditorLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig_editor::U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_GET_LOCAL_CONTROL_RIG_POSITIONS,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(56)
                .cast::<TArray<crate::bindings::core_u_object::FVector>>()
                .read()
        }
    }
    pub fn get_local_control_rig_position(
        level_sequence: UPtr<crate::bindings::level_sequence::ULevelSequence>,
        control_rig: UPtr<crate::bindings::control_rig::UControlRig>,
        control_name: FName,
        frame: crate::bindings::core_u_object::FFrameNumber,
        time_unit: crate::bindings::movie_scene::EMovieSceneTimeUnit,
    ) -> crate::bindings::core_u_object::FVector {
        let mut __stack = crate::core_data::StackAlloc::<64>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig_editor::U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_GET_LOCAL_CONTROL_RIG_POSITION,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &level_sequence,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::level_sequence::ULevelSequence>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &control_rig,
                __buffer
                    .add(8)
                    .cast::<UPtr<crate::bindings::control_rig::UControlRig>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &control_name,
                __buffer.add(16).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &frame,
                __buffer.add(28).cast::<crate::bindings::core_u_object::FFrameNumber>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &time_unit,
                __buffer
                    .add(32)
                    .cast::<crate::bindings::movie_scene::EMovieSceneTimeUnit>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::control_rig_editor::UControlRigSequencerEditorLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig_editor::U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_GET_LOCAL_CONTROL_RIG_POSITION,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(40).cast::<crate::bindings::core_u_object::FVector>().read()
        }
    }
    pub fn get_local_control_rig_ints(
        level_sequence: UPtr<crate::bindings::level_sequence::ULevelSequence>,
        control_rig: UPtr<crate::bindings::control_rig::UControlRig>,
        control_name: FName,
        frames: &TArray<crate::bindings::core_u_object::FFrameNumber>,
        time_unit: crate::bindings::movie_scene::EMovieSceneTimeUnit,
    ) -> TArray<i32> {
        let mut __stack = crate::core_data::StackAlloc::<72>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig_editor::U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_GET_LOCAL_CONTROL_RIG_INTS,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &level_sequence,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::level_sequence::ULevelSequence>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &control_rig,
                __buffer
                    .add(8)
                    .cast::<UPtr<crate::bindings::control_rig::UControlRig>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &control_name,
                __buffer.add(16).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                frames,
                __buffer
                    .add(32)
                    .cast::<TArray<crate::bindings::core_u_object::FFrameNumber>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &time_unit,
                __buffer
                    .add(48)
                    .cast::<crate::bindings::movie_scene::EMovieSceneTimeUnit>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::control_rig_editor::UControlRigSequencerEditorLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig_editor::U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_GET_LOCAL_CONTROL_RIG_INTS,
                __buffer,
            )
        };
        unsafe { __buffer.add(56).cast::<TArray<i32>>().read() }
    }
    pub fn get_local_control_rig_int(
        level_sequence: UPtr<crate::bindings::level_sequence::ULevelSequence>,
        control_rig: UPtr<crate::bindings::control_rig::UControlRig>,
        control_name: FName,
        frame: crate::bindings::core_u_object::FFrameNumber,
        time_unit: crate::bindings::movie_scene::EMovieSceneTimeUnit,
    ) -> i32 {
        let mut __stack = crate::core_data::StackAlloc::<40>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig_editor::U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_GET_LOCAL_CONTROL_RIG_INT,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &level_sequence,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::level_sequence::ULevelSequence>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &control_rig,
                __buffer
                    .add(8)
                    .cast::<UPtr<crate::bindings::control_rig::UControlRig>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &control_name,
                __buffer.add(16).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &frame,
                __buffer.add(28).cast::<crate::bindings::core_u_object::FFrameNumber>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &time_unit,
                __buffer
                    .add(32)
                    .cast::<crate::bindings::movie_scene::EMovieSceneTimeUnit>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::control_rig_editor::UControlRigSequencerEditorLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig_editor::U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_GET_LOCAL_CONTROL_RIG_INT,
                __buffer,
            )
        };
        unsafe { __buffer.add(36).cast::<i32>().read() }
    }
    pub fn get_local_control_rig_floats(
        level_sequence: UPtr<crate::bindings::level_sequence::ULevelSequence>,
        control_rig: UPtr<crate::bindings::control_rig::UControlRig>,
        control_name: FName,
        frames: &TArray<crate::bindings::core_u_object::FFrameNumber>,
        time_unit: crate::bindings::movie_scene::EMovieSceneTimeUnit,
    ) -> TArray<f32> {
        let mut __stack = crate::core_data::StackAlloc::<72>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig_editor::U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_GET_LOCAL_CONTROL_RIG_FLOATS,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &level_sequence,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::level_sequence::ULevelSequence>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &control_rig,
                __buffer
                    .add(8)
                    .cast::<UPtr<crate::bindings::control_rig::UControlRig>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &control_name,
                __buffer.add(16).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                frames,
                __buffer
                    .add(32)
                    .cast::<TArray<crate::bindings::core_u_object::FFrameNumber>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &time_unit,
                __buffer
                    .add(48)
                    .cast::<crate::bindings::movie_scene::EMovieSceneTimeUnit>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::control_rig_editor::UControlRigSequencerEditorLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig_editor::U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_GET_LOCAL_CONTROL_RIG_FLOATS,
                __buffer,
            )
        };
        unsafe { __buffer.add(56).cast::<TArray<f32>>().read() }
    }
    pub fn get_local_control_rig_float(
        level_sequence: UPtr<crate::bindings::level_sequence::ULevelSequence>,
        control_rig: UPtr<crate::bindings::control_rig::UControlRig>,
        control_name: FName,
        frame: crate::bindings::core_u_object::FFrameNumber,
        time_unit: crate::bindings::movie_scene::EMovieSceneTimeUnit,
    ) -> f32 {
        let mut __stack = crate::core_data::StackAlloc::<40>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig_editor::U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_GET_LOCAL_CONTROL_RIG_FLOAT,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &level_sequence,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::level_sequence::ULevelSequence>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &control_rig,
                __buffer
                    .add(8)
                    .cast::<UPtr<crate::bindings::control_rig::UControlRig>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &control_name,
                __buffer.add(16).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &frame,
                __buffer.add(28).cast::<crate::bindings::core_u_object::FFrameNumber>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &time_unit,
                __buffer
                    .add(32)
                    .cast::<crate::bindings::movie_scene::EMovieSceneTimeUnit>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::control_rig_editor::UControlRigSequencerEditorLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig_editor::U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_GET_LOCAL_CONTROL_RIG_FLOAT,
                __buffer,
            )
        };
        unsafe { __buffer.add(36).cast::<f32>().read() }
    }
    pub fn get_local_control_rig_euler_transforms(
        level_sequence: UPtr<crate::bindings::level_sequence::ULevelSequence>,
        control_rig: UPtr<crate::bindings::control_rig::UControlRig>,
        control_name: FName,
        frames: &TArray<crate::bindings::core_u_object::FFrameNumber>,
        time_unit: crate::bindings::movie_scene::EMovieSceneTimeUnit,
    ) -> TArray<crate::bindings::animation_core::FEulerTransform> {
        let mut __stack = crate::core_data::StackAlloc::<72>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig_editor::U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_GET_LOCAL_CONTROL_RIG_EULER_TRANSFORMS,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &level_sequence,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::level_sequence::ULevelSequence>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &control_rig,
                __buffer
                    .add(8)
                    .cast::<UPtr<crate::bindings::control_rig::UControlRig>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &control_name,
                __buffer.add(16).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                frames,
                __buffer
                    .add(32)
                    .cast::<TArray<crate::bindings::core_u_object::FFrameNumber>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &time_unit,
                __buffer
                    .add(48)
                    .cast::<crate::bindings::movie_scene::EMovieSceneTimeUnit>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::control_rig_editor::UControlRigSequencerEditorLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig_editor::U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_GET_LOCAL_CONTROL_RIG_EULER_TRANSFORMS,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(56)
                .cast::<TArray<crate::bindings::animation_core::FEulerTransform>>()
                .read()
        }
    }
    pub fn get_local_control_rig_euler_transform(
        level_sequence: UPtr<crate::bindings::level_sequence::ULevelSequence>,
        control_rig: UPtr<crate::bindings::control_rig::UControlRig>,
        control_name: FName,
        frame: crate::bindings::core_u_object::FFrameNumber,
        time_unit: crate::bindings::movie_scene::EMovieSceneTimeUnit,
    ) -> crate::bindings::animation_core::FEulerTransform {
        let mut __stack = crate::core_data::StackAlloc::<112>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig_editor::U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_GET_LOCAL_CONTROL_RIG_EULER_TRANSFORM,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &level_sequence,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::level_sequence::ULevelSequence>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &control_rig,
                __buffer
                    .add(8)
                    .cast::<UPtr<crate::bindings::control_rig::UControlRig>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &control_name,
                __buffer.add(16).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &frame,
                __buffer.add(28).cast::<crate::bindings::core_u_object::FFrameNumber>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &time_unit,
                __buffer
                    .add(32)
                    .cast::<crate::bindings::movie_scene::EMovieSceneTimeUnit>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::control_rig_editor::UControlRigSequencerEditorLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig_editor::U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_GET_LOCAL_CONTROL_RIG_EULER_TRANSFORM,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(40)
                .cast::<crate::bindings::animation_core::FEulerTransform>()
                .read()
        }
    }
    pub fn get_local_control_rig_bools(
        level_sequence: UPtr<crate::bindings::level_sequence::ULevelSequence>,
        control_rig: UPtr<crate::bindings::control_rig::UControlRig>,
        control_name: FName,
        frames: &TArray<crate::bindings::core_u_object::FFrameNumber>,
        time_unit: crate::bindings::movie_scene::EMovieSceneTimeUnit,
    ) -> TArray<bool> {
        let mut __stack = crate::core_data::StackAlloc::<72>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig_editor::U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_GET_LOCAL_CONTROL_RIG_BOOLS,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &level_sequence,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::level_sequence::ULevelSequence>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &control_rig,
                __buffer
                    .add(8)
                    .cast::<UPtr<crate::bindings::control_rig::UControlRig>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &control_name,
                __buffer.add(16).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                frames,
                __buffer
                    .add(32)
                    .cast::<TArray<crate::bindings::core_u_object::FFrameNumber>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &time_unit,
                __buffer
                    .add(48)
                    .cast::<crate::bindings::movie_scene::EMovieSceneTimeUnit>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::control_rig_editor::UControlRigSequencerEditorLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig_editor::U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_GET_LOCAL_CONTROL_RIG_BOOLS,
                __buffer,
            )
        };
        unsafe { __buffer.add(56).cast::<TArray<bool>>().read() }
    }
    pub fn get_local_control_rig_bool(
        level_sequence: UPtr<crate::bindings::level_sequence::ULevelSequence>,
        control_rig: UPtr<crate::bindings::control_rig::UControlRig>,
        control_name: FName,
        frame: crate::bindings::core_u_object::FFrameNumber,
        time_unit: crate::bindings::movie_scene::EMovieSceneTimeUnit,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<34>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig_editor::U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_GET_LOCAL_CONTROL_RIG_BOOL,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &level_sequence,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::level_sequence::ULevelSequence>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &control_rig,
                __buffer
                    .add(8)
                    .cast::<UPtr<crate::bindings::control_rig::UControlRig>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &control_name,
                __buffer.add(16).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &frame,
                __buffer.add(28).cast::<crate::bindings::core_u_object::FFrameNumber>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &time_unit,
                __buffer
                    .add(32)
                    .cast::<crate::bindings::movie_scene::EMovieSceneTimeUnit>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::control_rig_editor::UControlRigSequencerEditorLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig_editor::U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_GET_LOCAL_CONTROL_RIG_BOOL,
                __buffer,
            )
        };
        unsafe { __buffer.add(33).cast::<bool>().read() }
    }
    pub fn get_fk_control_rig_apply_mode(
        in_control_rig: UPtr<crate::bindings::control_rig::UControlRig>,
    ) -> crate::bindings::control_rig::EControlRigFKRigExecuteMode {
        let mut __stack = crate::core_data::StackAlloc::<9>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig_editor::U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_GET_FK_CONTROL_RIG_APPLY_MODE,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_control_rig,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::control_rig::UControlRig>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::control_rig_editor::UControlRigSequencerEditorLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig_editor::U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_GET_FK_CONTROL_RIG_APPLY_MODE,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(8)
                .cast::<crate::bindings::control_rig::EControlRigFKRigExecuteMode>()
                .read()
        }
    }
    pub fn get_default_parent_key() -> crate::bindings::control_rig::FRigElementKey {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig_editor::U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_GET_DEFAULT_PARENT_KEY,
                __buffer,
            )
        };
        let __object_ptr = crate::bindings::control_rig_editor::UControlRigSequencerEditorLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig_editor::U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_GET_DEFAULT_PARENT_KEY,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<crate::bindings::control_rig::FRigElementKey>().read()
        }
    }
    pub fn get_controls_mask(
        in_section: UPtr<crate::bindings::movie_scene::UMovieSceneSection>,
        control_name: FName,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<21>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig_editor::U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_GET_CONTROLS_MASK,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_section,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::movie_scene::UMovieSceneSection>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &control_name,
                __buffer.add(8).cast::<FName>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::control_rig_editor::UControlRigSequencerEditorLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig_editor::U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_GET_CONTROLS_MASK,
                __buffer,
            )
        };
        unsafe { __buffer.add(20).cast::<bool>().read() }
    }
    pub fn get_control_rig_world_transforms(
        level_sequence: UPtr<crate::bindings::level_sequence::ULevelSequence>,
        control_rig: UPtr<crate::bindings::control_rig::UControlRig>,
        control_name: FName,
        frames: &TArray<crate::bindings::core_u_object::FFrameNumber>,
        time_unit: crate::bindings::movie_scene::EMovieSceneTimeUnit,
    ) -> TArray<crate::bindings::core_u_object::FTransform> {
        let mut __stack = crate::core_data::StackAlloc::<72>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig_editor::U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_GET_CONTROL_RIG_WORLD_TRANSFORMS,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &level_sequence,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::level_sequence::ULevelSequence>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &control_rig,
                __buffer
                    .add(8)
                    .cast::<UPtr<crate::bindings::control_rig::UControlRig>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &control_name,
                __buffer.add(16).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                frames,
                __buffer
                    .add(32)
                    .cast::<TArray<crate::bindings::core_u_object::FFrameNumber>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &time_unit,
                __buffer
                    .add(48)
                    .cast::<crate::bindings::movie_scene::EMovieSceneTimeUnit>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::control_rig_editor::UControlRigSequencerEditorLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig_editor::U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_GET_CONTROL_RIG_WORLD_TRANSFORMS,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(56)
                .cast::<TArray<crate::bindings::core_u_object::FTransform>>()
                .read()
        }
    }
    pub fn get_control_rig_world_transform(
        level_sequence: UPtr<crate::bindings::level_sequence::ULevelSequence>,
        control_rig: UPtr<crate::bindings::control_rig::UControlRig>,
        control_name: FName,
        frame: crate::bindings::core_u_object::FFrameNumber,
        time_unit: crate::bindings::movie_scene::EMovieSceneTimeUnit,
    ) -> crate::bindings::core_u_object::FTransform {
        let mut __stack = crate::core_data::StackAlloc::<144>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig_editor::U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_GET_CONTROL_RIG_WORLD_TRANSFORM,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &level_sequence,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::level_sequence::ULevelSequence>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &control_rig,
                __buffer
                    .add(8)
                    .cast::<UPtr<crate::bindings::control_rig::UControlRig>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &control_name,
                __buffer.add(16).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &frame,
                __buffer.add(28).cast::<crate::bindings::core_u_object::FFrameNumber>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &time_unit,
                __buffer
                    .add(32)
                    .cast::<crate::bindings::movie_scene::EMovieSceneTimeUnit>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::control_rig_editor::UControlRigSequencerEditorLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig_editor::U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_GET_CONTROL_RIG_WORLD_TRANSFORM,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(48).cast::<crate::bindings::core_u_object::FTransform>().read()
        }
    }
    pub fn get_control_rigs(
        level_sequence: UPtr<crate::bindings::level_sequence::ULevelSequence>,
    ) -> TArray<FControlRigSequencerBindingProxy> {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig_editor::U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_GET_CONTROL_RIGS,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &level_sequence,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::level_sequence::ULevelSequence>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::control_rig_editor::UControlRigSequencerEditorLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig_editor::U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_GET_CONTROL_RIGS,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(8).cast::<TArray<FControlRigSequencerBindingProxy>>().read()
        }
    }
    pub fn get_control_rig_priority_order(
        in_section: UPtr<crate::bindings::movie_scene::UMovieSceneTrack>,
    ) -> i32 {
        let mut __stack = crate::core_data::StackAlloc::<12>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig_editor::U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_GET_CONTROL_RIG_PRIORITY_ORDER,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_section,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::movie_scene::UMovieSceneTrack>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::control_rig_editor::UControlRigSequencerEditorLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig_editor::U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_GET_CONTROL_RIG_PRIORITY_ORDER,
                __buffer,
            )
        };
        unsafe { __buffer.add(8).cast::<i32>().read() }
    }
    pub fn get_constraints_for_handle(
        in_world: UPtr<crate::bindings::engine::UWorld>,
        in_child: UPtr<crate::bindings::constraints::UTransformableHandle>,
    ) -> TArray<UPtr<crate::bindings::constraints::UTickableConstraint>> {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig_editor::U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_GET_CONSTRAINTS_FOR_HANDLE,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_world,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::UWorld>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_child,
                __buffer
                    .add(8)
                    .cast::<UPtr<crate::bindings::constraints::UTransformableHandle>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::control_rig_editor::UControlRigSequencerEditorLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig_editor::U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_GET_CONSTRAINTS_FOR_HANDLE,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(16)
                .cast::<
                    TArray<UPtr<crate::bindings::constraints::UTickableConstraint>>,
                >()
                .read()
        }
    }
    pub fn get_constraint_keys(
        in_constraint: UPtr<crate::bindings::constraints::UTickableConstraint>,
        constraint_section: UPtr<crate::bindings::movie_scene::UMovieSceneSection>,
        out_bools: &mut TArray<bool>,
        out_frames: &mut TArray<crate::bindings::core_u_object::FFrameNumber>,
        time_unit: crate::bindings::movie_scene::EMovieSceneTimeUnit,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<50>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig_editor::U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_GET_CONSTRAINT_KEYS,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_constraint,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::constraints::UTickableConstraint>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &constraint_section,
                __buffer
                    .add(8)
                    .cast::<UPtr<crate::bindings::movie_scene::UMovieSceneSection>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_bools,
                __buffer.add(16).cast::<TArray<bool>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_frames,
                __buffer
                    .add(32)
                    .cast::<TArray<crate::bindings::core_u_object::FFrameNumber>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &time_unit,
                __buffer
                    .add(48)
                    .cast::<crate::bindings::movie_scene::EMovieSceneTimeUnit>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::control_rig_editor::UControlRigSequencerEditorLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig_editor::U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_GET_CONSTRAINT_KEYS,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(16).cast::<TArray<bool>>().swap(out_bools);
        }
        unsafe {
            __buffer
                .add(32)
                .cast::<TArray<crate::bindings::core_u_object::FFrameNumber>>()
                .swap(out_frames);
        }
        unsafe { __buffer.add(49).cast::<bool>().read() }
    }
    pub fn get_anim_layers(b_add_if_does_not_exist: bool) -> TArray<UPtr<UAnimLayer>> {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig_editor::U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_GET_ANIM_LAYERS,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_add_if_does_not_exist,
                __buffer.add(0).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::control_rig_editor::UControlRigSequencerEditorLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig_editor::U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_GET_ANIM_LAYERS,
                __buffer,
            )
        };
        unsafe { __buffer.add(8).cast::<TArray<UPtr<UAnimLayer>>>().read() }
    }
    pub fn get_anim_layer_index(anim_layer: UPtr<UAnimLayer>) -> i32 {
        let mut __stack = crate::core_data::StackAlloc::<12>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig_editor::U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_GET_ANIM_LAYER_INDEX,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &anim_layer,
                __buffer.add(0).cast::<UPtr<UAnimLayer>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::control_rig_editor::UControlRigSequencerEditorLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig_editor::U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_GET_ANIM_LAYER_INDEX,
                __buffer,
            )
        };
        unsafe { __buffer.add(8).cast::<i32>().read() }
    }
    pub fn get_actor_world_transforms(
        level_sequence: UPtr<crate::bindings::level_sequence::ULevelSequence>,
        actor: UPtr<crate::bindings::engine::AActor>,
        frames: &TArray<crate::bindings::core_u_object::FFrameNumber>,
        time_unit: crate::bindings::movie_scene::EMovieSceneTimeUnit,
    ) -> TArray<crate::bindings::core_u_object::FTransform> {
        let mut __stack = crate::core_data::StackAlloc::<56>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig_editor::U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_GET_ACTOR_WORLD_TRANSFORMS,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &level_sequence,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::level_sequence::ULevelSequence>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &actor,
                __buffer.add(8).cast::<UPtr<crate::bindings::engine::AActor>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                frames,
                __buffer
                    .add(16)
                    .cast::<TArray<crate::bindings::core_u_object::FFrameNumber>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &time_unit,
                __buffer
                    .add(32)
                    .cast::<crate::bindings::movie_scene::EMovieSceneTimeUnit>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::control_rig_editor::UControlRigSequencerEditorLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig_editor::U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_GET_ACTOR_WORLD_TRANSFORMS,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(40)
                .cast::<TArray<crate::bindings::core_u_object::FTransform>>()
                .read()
        }
    }
    pub fn get_actor_world_transform(
        level_sequence: UPtr<crate::bindings::level_sequence::ULevelSequence>,
        actor: UPtr<crate::bindings::engine::AActor>,
        frame: crate::bindings::core_u_object::FFrameNumber,
        time_unit: crate::bindings::movie_scene::EMovieSceneTimeUnit,
    ) -> crate::bindings::core_u_object::FTransform {
        let mut __stack = crate::core_data::StackAlloc::<128>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig_editor::U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_GET_ACTOR_WORLD_TRANSFORM,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &level_sequence,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::level_sequence::ULevelSequence>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &actor,
                __buffer.add(8).cast::<UPtr<crate::bindings::engine::AActor>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &frame,
                __buffer.add(16).cast::<crate::bindings::core_u_object::FFrameNumber>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &time_unit,
                __buffer
                    .add(20)
                    .cast::<crate::bindings::movie_scene::EMovieSceneTimeUnit>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::control_rig_editor::UControlRigSequencerEditorLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig_editor::U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_GET_ACTOR_WORLD_TRANSFORM,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(32).cast::<crate::bindings::core_u_object::FTransform>().read()
        }
    }
    pub fn find_or_create_control_rig_track(
        world: UPtr<crate::bindings::engine::UWorld>,
        level_sequence: UPtr<crate::bindings::level_sequence::ULevelSequence>,
        control_rig_class: TSubclassOf<crate::bindings::core_u_object::UObject>,
        in_binding: &crate::bindings::movie_scene::FMovieSceneBindingProxy,
        b_is_layered_control_rig: bool,
    ) -> UPtr<crate::bindings::movie_scene::UMovieSceneTrack> {
        let mut __stack = crate::core_data::StackAlloc::<64>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig_editor::U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_FIND_OR_CREATE_CONTROL_RIG_TRACK,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &world,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::UWorld>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &level_sequence,
                __buffer
                    .add(8)
                    .cast::<UPtr<crate::bindings::level_sequence::ULevelSequence>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &control_rig_class,
                __buffer
                    .add(16)
                    .cast::<TSubclassOf<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_binding,
                __buffer
                    .add(24)
                    .cast::<crate::bindings::movie_scene::FMovieSceneBindingProxy>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_is_layered_control_rig,
                __buffer.add(48).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::control_rig_editor::UControlRigSequencerEditorLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig_editor::U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_FIND_OR_CREATE_CONTROL_RIG_TRACK,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(56)
                .cast::<UPtr<crate::bindings::movie_scene::UMovieSceneTrack>>()
                .read()
        }
    }
    pub fn find_or_create_control_rig_component_track(
        world: UPtr<crate::bindings::engine::UWorld>,
        level_sequence: UPtr<crate::bindings::level_sequence::ULevelSequence>,
        in_binding: &crate::bindings::movie_scene::FMovieSceneBindingProxy,
    ) -> TArray<UPtr<crate::bindings::movie_scene::UMovieSceneTrack>> {
        let mut __stack = crate::core_data::StackAlloc::<56>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig_editor::U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_FIND_OR_CREATE_CONTROL_RIG_COMPONENT_TRACK,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &world,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::UWorld>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &level_sequence,
                __buffer
                    .add(8)
                    .cast::<UPtr<crate::bindings::level_sequence::ULevelSequence>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_binding,
                __buffer
                    .add(16)
                    .cast::<crate::bindings::movie_scene::FMovieSceneBindingProxy>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::control_rig_editor::UControlRigSequencerEditorLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig_editor::U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_FIND_OR_CREATE_CONTROL_RIG_COMPONENT_TRACK,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(40)
                .cast::<TArray<UPtr<crate::bindings::movie_scene::UMovieSceneTrack>>>()
                .read()
        }
    }
    pub fn export_fbx_from_control_rig_section(
        sequence: UPtr<crate::bindings::level_sequence::ULevelSequence>,
        section: UPtr<
            crate::bindings::control_rig::UMovieSceneControlRigParameterSection,
        >,
        export_fbx_control_rig_settings: UPtr<
            crate::bindings::movie_scene_tools::UMovieSceneUserExportFBXControlRigSettings,
        >,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<25>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig_editor::U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_EXPORT_FBX_FROM_CONTROL_RIG_SECTION,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &sequence,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::level_sequence::ULevelSequence>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &section,
                __buffer
                    .add(8)
                    .cast::<
                        UPtr<
                            crate::bindings::control_rig::UMovieSceneControlRigParameterSection,
                        >,
                    >(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &export_fbx_control_rig_settings,
                __buffer
                    .add(16)
                    .cast::<
                        UPtr<
                            crate::bindings::movie_scene_tools::UMovieSceneUserExportFBXControlRigSettings,
                        >,
                    >(),
                1,
            );
        }
        let __object_ptr = crate::bindings::control_rig_editor::UControlRigSequencerEditorLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig_editor::U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_EXPORT_FBX_FROM_CONTROL_RIG_SECTION,
                __buffer,
            )
        };
        unsafe { __buffer.add(24).cast::<bool>().read() }
    }
    pub fn export_anim_sequence_from_sequencer(
        anim_sequence: UPtr<crate::bindings::engine::UAnimSequence>,
        export_option: UPtr<crate::bindings::unreal_ed::UAnimSeqExportOption>,
        binding: &crate::bindings::movie_scene::FMovieSceneBindingProxy,
        b_create_link: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<42>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig_editor::U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_EXPORT_ANIM_SEQUENCE_FROM_SEQUENCER,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &anim_sequence,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::UAnimSequence>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &export_option,
                __buffer
                    .add(8)
                    .cast::<UPtr<crate::bindings::unreal_ed::UAnimSeqExportOption>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                binding,
                __buffer
                    .add(16)
                    .cast::<crate::bindings::movie_scene::FMovieSceneBindingProxy>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_create_link,
                __buffer.add(40).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::control_rig_editor::UControlRigSequencerEditorLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig_editor::U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_EXPORT_ANIM_SEQUENCE_FROM_SEQUENCER,
                __buffer,
            )
        };
        unsafe { __buffer.add(41).cast::<bool>().read() }
    }
    pub fn duplicate_anim_layer(index: i32) -> i32 {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig_editor::U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_DUPLICATE_ANIM_LAYER,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&index, __buffer.add(0).cast::<i32>(), 1);
        }
        let __object_ptr = crate::bindings::control_rig_editor::UControlRigSequencerEditorLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig_editor::U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_DUPLICATE_ANIM_LAYER,
                __buffer,
            )
        };
        unsafe { __buffer.add(4).cast::<i32>().read() }
    }
    pub fn delete_control_rig_space(
        in_sequence: UPtr<crate::bindings::level_sequence::ULevelSequence>,
        in_control_rig: UPtr<crate::bindings::control_rig::UControlRig>,
        in_control_name: FName,
        in_time: crate::bindings::core_u_object::FFrameNumber,
        time_unit: crate::bindings::movie_scene::EMovieSceneTimeUnit,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<34>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig_editor::U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_DELETE_CONTROL_RIG_SPACE,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_sequence,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::level_sequence::ULevelSequence>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_control_rig,
                __buffer
                    .add(8)
                    .cast::<UPtr<crate::bindings::control_rig::UControlRig>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_control_name,
                __buffer.add(16).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_time,
                __buffer.add(28).cast::<crate::bindings::core_u_object::FFrameNumber>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &time_unit,
                __buffer
                    .add(32)
                    .cast::<crate::bindings::movie_scene::EMovieSceneTimeUnit>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::control_rig_editor::UControlRigSequencerEditorLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig_editor::U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_DELETE_CONTROL_RIG_SPACE,
                __buffer,
            )
        };
        unsafe { __buffer.add(33).cast::<bool>().read() }
    }
    pub fn delete_constraint_key(
        constraint: UPtr<crate::bindings::constraints::UTickableConstraint>,
        constraint_section: UPtr<crate::bindings::movie_scene::UMovieSceneSection>,
        in_time: crate::bindings::core_u_object::FFrameNumber,
        time_unit: crate::bindings::movie_scene::EMovieSceneTimeUnit,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<22>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig_editor::U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_DELETE_CONSTRAINT_KEY,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &constraint,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::constraints::UTickableConstraint>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &constraint_section,
                __buffer
                    .add(8)
                    .cast::<UPtr<crate::bindings::movie_scene::UMovieSceneSection>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_time,
                __buffer.add(16).cast::<crate::bindings::core_u_object::FFrameNumber>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &time_unit,
                __buffer
                    .add(20)
                    .cast::<crate::bindings::movie_scene::EMovieSceneTimeUnit>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::control_rig_editor::UControlRigSequencerEditorLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig_editor::U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_DELETE_CONSTRAINT_KEY,
                __buffer,
            )
        };
        unsafe { __buffer.add(21).cast::<bool>().read() }
    }
    pub fn delete_anim_layer(index: i32) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<5>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig_editor::U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_DELETE_ANIM_LAYER,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&index, __buffer.add(0).cast::<i32>(), 1);
        }
        let __object_ptr = crate::bindings::control_rig_editor::UControlRigSequencerEditorLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig_editor::U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_DELETE_ANIM_LAYER,
                __buffer,
            )
        };
        unsafe { __buffer.add(4).cast::<bool>().read() }
    }
    pub fn control_rig_copy_vector_parameter_curves_to_transform(
        in_track: UPtr<
            crate::bindings::control_rig::UMovieSceneControlRigParameterTrack,
        >,
        in_control_name: &FName,
        in_type: crate::bindings::control_rig::ERigControlType,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<22>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig_editor::U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_CONTROL_RIG_COPY_VECTOR_PARAMETER_CURVES_TO_TRANSFORM,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_track,
                __buffer
                    .add(0)
                    .cast::<
                        UPtr<
                            crate::bindings::control_rig::UMovieSceneControlRigParameterTrack,
                        >,
                    >(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_control_name,
                __buffer.add(8).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_type,
                __buffer.add(20).cast::<crate::bindings::control_rig::ERigControlType>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::control_rig_editor::UControlRigSequencerEditorLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig_editor::U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_CONTROL_RIG_COPY_VECTOR_PARAMETER_CURVES_TO_TRANSFORM,
                __buffer,
            )
        };
        unsafe { __buffer.add(21).cast::<bool>().read() }
    }
    pub fn compensate_all(
        in_constraint: UPtr<crate::bindings::constraints::UTickableConstraint>,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<9>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig_editor::U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_COMPENSATE_ALL,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_constraint,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::constraints::UTickableConstraint>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::control_rig_editor::UControlRigSequencerEditorLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig_editor::U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_COMPENSATE_ALL,
                __buffer,
            )
        };
        unsafe { __buffer.add(8).cast::<bool>().read() }
    }
    pub fn compensate(
        in_constraint: UPtr<crate::bindings::constraints::UTickableConstraint>,
        in_time: crate::bindings::core_u_object::FFrameNumber,
        time_unit: crate::bindings::movie_scene::EMovieSceneTimeUnit,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<14>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig_editor::U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_COMPENSATE,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_constraint,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::constraints::UTickableConstraint>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_time,
                __buffer.add(8).cast::<crate::bindings::core_u_object::FFrameNumber>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &time_unit,
                __buffer
                    .add(12)
                    .cast::<crate::bindings::movie_scene::EMovieSceneTimeUnit>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::control_rig_editor::UControlRigSequencerEditorLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig_editor::U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_COMPENSATE,
                __buffer,
            )
        };
        unsafe { __buffer.add(13).cast::<bool>().read() }
    }
    pub fn collapse_control_rig_anim_layers_with_settings(
        in_sequence: UPtr<crate::bindings::level_sequence::ULevelSequence>,
        in_track: UPtr<
            crate::bindings::control_rig::UMovieSceneControlRigParameterTrack,
        >,
        in_settings: &crate::bindings::movie_scene_tools::FBakingAnimationKeySettings,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<45>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig_editor::U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_COLLAPSE_CONTROL_RIG_ANIM_LAYERS_WITH_SETTINGS,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_sequence,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::level_sequence::ULevelSequence>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_track,
                __buffer
                    .add(8)
                    .cast::<
                        UPtr<
                            crate::bindings::control_rig::UMovieSceneControlRigParameterTrack,
                        >,
                    >(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_settings,
                __buffer
                    .add(16)
                    .cast::<
                        crate::bindings::movie_scene_tools::FBakingAnimationKeySettings,
                    >(),
                1,
            );
        }
        let __object_ptr = crate::bindings::control_rig_editor::UControlRigSequencerEditorLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig_editor::U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_COLLAPSE_CONTROL_RIG_ANIM_LAYERS_WITH_SETTINGS,
                __buffer,
            )
        };
        unsafe { __buffer.add(44).cast::<bool>().read() }
    }
    pub fn collapse_control_rig_anim_layers(
        in_sequence: UPtr<crate::bindings::level_sequence::ULevelSequence>,
        in_track: UPtr<
            crate::bindings::control_rig::UMovieSceneControlRigParameterTrack,
        >,
        b_key_reduce: bool,
        tolerance: f32,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<25>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig_editor::U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_COLLAPSE_CONTROL_RIG_ANIM_LAYERS,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_sequence,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::level_sequence::ULevelSequence>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_track,
                __buffer
                    .add(8)
                    .cast::<
                        UPtr<
                            crate::bindings::control_rig::UMovieSceneControlRigParameterTrack,
                        >,
                    >(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_key_reduce,
                __buffer.add(16).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&tolerance, __buffer.add(20).cast::<f32>(), 1);
        }
        let __object_ptr = crate::bindings::control_rig_editor::UControlRigSequencerEditorLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig_editor::U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_COLLAPSE_CONTROL_RIG_ANIM_LAYERS,
                __buffer,
            )
        };
        unsafe { __buffer.add(24).cast::<bool>().read() }
    }
    pub fn blend_values_on_selected(
        level_sequence: UPtr<crate::bindings::level_sequence::ULevelSequence>,
        blend_operation: EAnimToolBlendOperation,
        blend_value: f32,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<17>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig_editor::U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_BLEND_VALUES_ON_SELECTED,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &level_sequence,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::level_sequence::ULevelSequence>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &blend_operation,
                __buffer.add(8).cast::<EAnimToolBlendOperation>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &blend_value,
                __buffer.add(12).cast::<f32>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::control_rig_editor::UControlRigSequencerEditorLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig_editor::U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_BLEND_VALUES_ON_SELECTED,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<bool>().read() }
    }
    pub fn bake_to_control_rig(
        world: UPtr<crate::bindings::engine::UWorld>,
        level_sequence: UPtr<crate::bindings::level_sequence::ULevelSequence>,
        control_rig_class: TSubclassOf<crate::bindings::core_u_object::UObject>,
        export_options: UPtr<crate::bindings::unreal_ed::UAnimSeqExportOption>,
        b_reduce_keys: bool,
        tolerance: f32,
        binding: &crate::bindings::movie_scene::FMovieSceneBindingProxy,
        b_reset_controls: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<66>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig_editor::U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_BAKE_TO_CONTROL_RIG,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &world,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::UWorld>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &level_sequence,
                __buffer
                    .add(8)
                    .cast::<UPtr<crate::bindings::level_sequence::ULevelSequence>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &control_rig_class,
                __buffer
                    .add(16)
                    .cast::<TSubclassOf<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &export_options,
                __buffer
                    .add(24)
                    .cast::<UPtr<crate::bindings::unreal_ed::UAnimSeqExportOption>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_reduce_keys,
                __buffer.add(32).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&tolerance, __buffer.add(36).cast::<f32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                binding,
                __buffer
                    .add(40)
                    .cast::<crate::bindings::movie_scene::FMovieSceneBindingProxy>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_reset_controls,
                __buffer.add(64).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::control_rig_editor::UControlRigSequencerEditorLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig_editor::U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_BAKE_TO_CONTROL_RIG,
                __buffer,
            )
        };
        unsafe { __buffer.add(65).cast::<bool>().read() }
    }
    pub fn bake_control_rig_space(
        in_sequence: UPtr<crate::bindings::level_sequence::ULevelSequence>,
        in_control_rig: UPtr<crate::bindings::control_rig::UControlRig>,
        in_control_names: &TArray<FName>,
        in_settings: FRigSpacePickerBakeSettings,
        time_unit: crate::bindings::movie_scene::EMovieSceneTimeUnit,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<94>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig_editor::U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_BAKE_CONTROL_RIG_SPACE,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_sequence,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::level_sequence::ULevelSequence>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_control_rig,
                __buffer
                    .add(8)
                    .cast::<UPtr<crate::bindings::control_rig::UControlRig>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_control_names,
                __buffer.add(16).cast::<TArray<FName>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_settings,
                __buffer.add(32).cast::<FRigSpacePickerBakeSettings>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &time_unit,
                __buffer
                    .add(92)
                    .cast::<crate::bindings::movie_scene::EMovieSceneTimeUnit>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::control_rig_editor::UControlRigSequencerEditorLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig_editor::U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_BAKE_CONTROL_RIG_SPACE,
                __buffer,
            )
        };
        unsafe { __buffer.add(93).cast::<bool>().read() }
    }
    pub fn bake_constraints(
        world: UPtr<crate::bindings::engine::UWorld>,
        in_constraints: &mut TArray<
            UPtr<crate::bindings::constraints::UTickableConstraint>,
        >,
        in_settings: &crate::bindings::movie_scene_tools::FBakingAnimationKeySettings,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<53>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig_editor::U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_BAKE_CONSTRAINTS,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &world,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::UWorld>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_constraints,
                __buffer
                    .add(8)
                    .cast::<
                        TArray<UPtr<crate::bindings::constraints::UTickableConstraint>>,
                    >(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_settings,
                __buffer
                    .add(24)
                    .cast::<
                        crate::bindings::movie_scene_tools::FBakingAnimationKeySettings,
                    >(),
                1,
            );
        }
        let __object_ptr = crate::bindings::control_rig_editor::UControlRigSequencerEditorLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig_editor::U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_BAKE_CONSTRAINTS,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(8)
                .cast::<
                    TArray<UPtr<crate::bindings::constraints::UTickableConstraint>>,
                >()
                .swap(in_constraints);
        }
        unsafe { __buffer.add(52).cast::<bool>().read() }
    }
    pub fn bake_constraint(
        world: UPtr<crate::bindings::engine::UWorld>,
        constraint: UPtr<crate::bindings::constraints::UTickableConstraint>,
        frames: &TArray<crate::bindings::core_u_object::FFrameNumber>,
        time_unit: crate::bindings::movie_scene::EMovieSceneTimeUnit,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<34>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig_editor::U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_BAKE_CONSTRAINT,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &world,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::UWorld>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &constraint,
                __buffer
                    .add(8)
                    .cast::<UPtr<crate::bindings::constraints::UTickableConstraint>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                frames,
                __buffer
                    .add(16)
                    .cast::<TArray<crate::bindings::core_u_object::FFrameNumber>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &time_unit,
                __buffer
                    .add(32)
                    .cast::<crate::bindings::movie_scene::EMovieSceneTimeUnit>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::control_rig_editor::UControlRigSequencerEditorLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig_editor::U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_BAKE_CONSTRAINT,
                __buffer,
            )
        };
        unsafe { __buffer.add(33).cast::<bool>().read() }
    }
    pub fn add_constraint(
        world: UPtr<crate::bindings::engine::UWorld>,
        in_type: crate::bindings::animation_core::ETransformConstraintType,
        in_child: UPtr<crate::bindings::constraints::UTransformableHandle>,
        in_parent: UPtr<crate::bindings::constraints::UTransformableHandle>,
        b_maintain_offset: bool,
    ) -> UPtr<crate::bindings::constraints::UTickableConstraint> {
        let mut __stack = crate::core_data::StackAlloc::<48>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig_editor::U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_ADD_CONSTRAINT,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &world,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::UWorld>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_type,
                __buffer
                    .add(8)
                    .cast::<crate::bindings::animation_core::ETransformConstraintType>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_child,
                __buffer
                    .add(16)
                    .cast::<UPtr<crate::bindings::constraints::UTransformableHandle>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_parent,
                __buffer
                    .add(24)
                    .cast::<UPtr<crate::bindings::constraints::UTransformableHandle>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_maintain_offset,
                __buffer.add(32).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::control_rig_editor::UControlRigSequencerEditorLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig_editor::U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_ADD_CONSTRAINT,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(40)
                .cast::<UPtr<crate::bindings::constraints::UTickableConstraint>>()
                .read()
        }
    }
    pub fn add_anim_layer_from_selection() -> i32 {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig_editor::U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_ADD_ANIM_LAYER_FROM_SELECTION,
                __buffer,
            )
        };
        let __object_ptr = crate::bindings::control_rig_editor::UControlRigSequencerEditorLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig_editor::U_CONTROL_RIG_SEQUENCER_EDITOR_LIBRARY_ADD_ANIM_LAYER_FROM_SELECTION,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<i32>().read() }
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
    pub fn is_alt_down(&self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig_editor::U_CONTROL_RIG_CONTEXT_MENU_CONTEXT_IS_ALT_DOWN,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig_editor::U_CONTROL_RIG_CONTEXT_MENU_CONTEXT_IS_ALT_DOWN,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn get_rig_hierarchy_to_graph_drag_and_drop_context(
        &mut self,
    ) -> FControlRigRigHierarchyToGraphDragAndDropContext {
        let mut __stack = crate::core_data::StackAlloc::<40>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig_editor::U_CONTROL_RIG_CONTEXT_MENU_CONTEXT_GET_RIG_HIERARCHY_TO_GRAPH_DRAG_AND_DROP_CONTEXT,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig_editor::U_CONTROL_RIG_CONTEXT_MENU_CONTEXT_GET_RIG_HIERARCHY_TO_GRAPH_DRAG_AND_DROP_CONTEXT,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(0)
                .cast::<FControlRigRigHierarchyToGraphDragAndDropContext>()
                .read()
        }
    }
    pub fn get_rig_hierarchy_drag_and_drop_context(
        &mut self,
    ) -> FControlRigRigHierarchyDragAndDropContext {
        let mut __stack = crate::core_data::StackAlloc::<72>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig_editor::U_CONTROL_RIG_CONTEXT_MENU_CONTEXT_GET_RIG_HIERARCHY_DRAG_AND_DROP_CONTEXT,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig_editor::U_CONTROL_RIG_CONTEXT_MENU_CONTEXT_GET_RIG_HIERARCHY_DRAG_AND_DROP_CONTEXT,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<FControlRigRigHierarchyDragAndDropContext>().read()
        }
    }
    pub fn get_graph_node_context_menu_context(
        &mut self,
    ) -> FControlRigGraphNodeContextMenuContext {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig_editor::U_CONTROL_RIG_CONTEXT_MENU_CONTEXT_GET_GRAPH_NODE_CONTEXT_MENU_CONTEXT,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig_editor::U_CONTROL_RIG_CONTEXT_MENU_CONTEXT_GET_GRAPH_NODE_CONTEXT_MENU_CONTEXT,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<FControlRigGraphNodeContextMenuContext>().read()
        }
    }
    pub fn get_control_rig_blueprint(
        &self,
    ) -> UPtr<crate::bindings::control_rig_developer::UControlRigBlueprint> {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig_editor::U_CONTROL_RIG_CONTEXT_MENU_CONTEXT_GET_CONTROL_RIG_BLUEPRINT,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig_editor::U_CONTROL_RIG_CONTEXT_MENU_CONTEXT_GET_CONTROL_RIG_BLUEPRINT,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(0)
                .cast::<
                    UPtr<crate::bindings::control_rig_developer::UControlRigBlueprint>,
                >()
                .read()
        }
    }
    pub fn get_control_rig_asset_interface(
        &self,
    ) -> TScriptInterface<
        crate::bindings::control_rig_developer::UControlRigAssetInterface,
    > {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig_editor::U_CONTROL_RIG_CONTEXT_MENU_CONTEXT_GET_CONTROL_RIG_ASSET_INTERFACE,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig_editor::U_CONTROL_RIG_CONTEXT_MENU_CONTEXT_GET_CONTROL_RIG_ASSET_INTERFACE,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(0)
                .cast::<
                    TScriptInterface<
                        crate::bindings::control_rig_developer::UControlRigAssetInterface,
                    >,
                >()
                .read()
        }
    }
    pub fn get_control_rig(&self) -> UPtr<crate::bindings::control_rig::UControlRig> {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig_editor::U_CONTROL_RIG_CONTEXT_MENU_CONTEXT_GET_CONTROL_RIG,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig_editor::U_CONTROL_RIG_CONTEXT_MENU_CONTEXT_GET_CONTROL_RIG,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(0)
                .cast::<UPtr<crate::bindings::control_rig::UControlRig>>()
                .read()
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
    pub fn set_weight(&mut self, in_weight: f64) {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig_editor::U_ANIM_LAYER_SET_WEIGHT,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&in_weight, __buffer.add(0).cast::<f64>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig_editor::U_ANIM_LAYER_SET_WEIGHT,
                __buffer,
            )
        };
    }
    pub fn set_type(&mut self, layer_type: EAnimLayerType) {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig_editor::U_ANIM_LAYER_SET_TYPE,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &layer_type,
                __buffer.add(0).cast::<EAnimLayerType>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig_editor::U_ANIM_LAYER_SET_TYPE,
                __buffer,
            )
        };
    }
    pub fn set_selected(&mut self, b_in_selected: bool, b_clear_selection: bool) {
        let mut __stack = crate::core_data::StackAlloc::<2>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig_editor::U_ANIM_LAYER_SET_SELECTED,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_in_selected,
                __buffer.add(0).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_clear_selection,
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
                crate::bindings::control_rig_editor::U_ANIM_LAYER_SET_SELECTED,
                __buffer,
            )
        };
    }
    pub fn set_name(&mut self, in_name: &FText) {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig_editor::U_ANIM_LAYER_SET_NAME,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(in_name, __buffer.add(0).cast::<FText>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig_editor::U_ANIM_LAYER_SET_NAME,
                __buffer,
            )
        };
    }
    pub fn set_lock(&mut self, b_in_lock: bool) {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig_editor::U_ANIM_LAYER_SET_LOCK,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&b_in_lock, __buffer.add(0).cast::<bool>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig_editor::U_ANIM_LAYER_SET_LOCK,
                __buffer,
            )
        };
    }
    pub fn set_keyed(&mut self) {
        let mut __stack = crate::core_data::StackAlloc::<0>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig_editor::U_ANIM_LAYER_SET_KEYED,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig_editor::U_ANIM_LAYER_SET_KEYED,
                __buffer,
            )
        };
    }
    pub fn set_active(&mut self, b_in_active: bool) {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig_editor::U_ANIM_LAYER_SET_ACTIVE,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_in_active,
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
                crate::bindings::control_rig_editor::U_ANIM_LAYER_SET_ACTIVE,
                __buffer,
            )
        };
    }
    pub fn remove_selected_in_sequencer(&mut self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig_editor::U_ANIM_LAYER_REMOVE_SELECTED_IN_SEQUENCER,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig_editor::U_ANIM_LAYER_REMOVE_SELECTED_IN_SEQUENCER,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn get_weight(&self) -> f64 {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig_editor::U_ANIM_LAYER_GET_WEIGHT,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig_editor::U_ANIM_LAYER_GET_WEIGHT,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<f64>().read() }
    }
    pub fn get_type(&self) -> EAnimLayerType {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig_editor::U_ANIM_LAYER_GET_TYPE,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig_editor::U_ANIM_LAYER_GET_TYPE,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<EAnimLayerType>().read() }
    }
    pub fn get_selected(&self) -> crate::bindings::slate_core::ECheckBoxState {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig_editor::U_ANIM_LAYER_GET_SELECTED,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig_editor::U_ANIM_LAYER_GET_SELECTED,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<crate::bindings::slate_core::ECheckBoxState>().read()
        }
    }
    pub fn get_name(&self) -> FText {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig_editor::U_ANIM_LAYER_GET_NAME,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig_editor::U_ANIM_LAYER_GET_NAME,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<FText>().read() }
    }
    pub fn get_lock(&self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig_editor::U_ANIM_LAYER_GET_LOCK,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig_editor::U_ANIM_LAYER_GET_LOCK,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn get_keyed(&self) -> crate::bindings::slate_core::ECheckBoxState {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig_editor::U_ANIM_LAYER_GET_KEYED,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig_editor::U_ANIM_LAYER_GET_KEYED,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<crate::bindings::slate_core::ECheckBoxState>().read()
        }
    }
    pub fn get_active(&self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig_editor::U_ANIM_LAYER_GET_ACTIVE,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig_editor::U_ANIM_LAYER_GET_ACTIVE,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn add_selected_in_sequencer(&mut self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig_editor::U_ANIM_LAYER_ADD_SELECTED_IN_SEQUENCER,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig_editor::U_ANIM_LAYER_ADD_SELECTED_IN_SEQUENCER,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
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
