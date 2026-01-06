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
pub static mut U_WIDGET_SET_VISIBILITY: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_WIDGET_SET_USER_FOCUS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_WIDGET_SET_TOOL_TIP_TEXT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_WIDGET_SET_TOOL_TIP: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_WIDGET_SET_RENDER_TRANSLATION: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_WIDGET_SET_RENDER_TRANSFORM_PIVOT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_WIDGET_SET_RENDER_TRANSFORM_ANGLE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_WIDGET_SET_RENDER_TRANSFORM: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_WIDGET_SET_RENDER_SHEAR: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_WIDGET_SET_RENDER_SCALE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_WIDGET_SET_RENDER_OPACITY: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_WIDGET_SET_NAVIGATION_RULE_EXPLICIT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_WIDGET_SET_NAVIGATION_RULE_CUSTOM_BOUNDARY: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_WIDGET_SET_NAVIGATION_RULE_CUSTOM: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_WIDGET_SET_NAVIGATION_RULE_BASE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_WIDGET_SET_NAVIGATION_RULE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_WIDGET_SET_KEYBOARD_FOCUS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_WIDGET_SET_IS_ENABLED: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_WIDGET_SET_FOCUS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_WIDGET_SET_CURSOR: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_WIDGET_SET_CLIPPING: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_WIDGET_SET_ALL_NAVIGATION_RULES: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_WIDGET_RESET_CURSOR: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_WIDGET_REMOVE_FROM_PARENT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_WIDGET_ON_REPLY_DELEGATE_SIGNATURE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_WIDGET_ON_POINTER_EVENT_DELEGATE_SIGNATURE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_WIDGET_K2_REMOVE_FIELD_VALUE_CHANGED_DELEGATE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_WIDGET_K2_BROADCAST_FIELD_VALUE_CHANGED: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_WIDGET_K2_ADD_FIELD_VALUE_CHANGED_DELEGATE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_WIDGET_IS_VISIBLE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_WIDGET_IS_RENDERED: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_WIDGET_IS_IN_VIEWPORT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_WIDGET_IS_HOVERED: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_WIDGET_INVALIDATE_LAYOUT_AND_VOLATILITY: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_WIDGET_HAS_USER_FOCUSED_DESCENDANTS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_WIDGET_HAS_USER_FOCUS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_WIDGET_HAS_MOUSE_CAPTURE_BY_USER: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_WIDGET_HAS_MOUSE_CAPTURE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_WIDGET_HAS_KEYBOARD_FOCUS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_WIDGET_HAS_FOCUSED_DESCENDANTS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_WIDGET_HAS_ANY_USER_FOCUS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_WIDGET_GET_WIDGET_DELEGATE_SIGNATURE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_WIDGET_GET_VISIBILITY: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_WIDGET_GET_TICK_SPACE_GEOMETRY: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_WIDGET_GET_TEXT_DELEGATE_SIGNATURE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_WIDGET_GET_SLATE_VISIBILITY_DELEGATE_SIGNATURE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_WIDGET_GET_SLATE_COLOR_DELEGATE_SIGNATURE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_WIDGET_GET_SLATE_BRUSH_DELEGATE_SIGNATURE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_WIDGET_GET_RENDER_TRANSFORM_ANGLE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_WIDGET_GET_RENDER_OPACITY: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_WIDGET_GET_PARENT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_WIDGET_GET_PAINT_SPACE_GEOMETRY: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_WIDGET_GET_OWNING_PLAYER: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_WIDGET_GET_OWNING_LOCAL_PLAYER: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_WIDGET_GET_MOUSE_CURSOR_DELEGATE_SIGNATURE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_WIDGET_GET_LINEAR_COLOR_DELEGATE_SIGNATURE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_WIDGET_GET_IS_ENABLED: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_WIDGET_GET_INT32_DELEGATE_SIGNATURE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_WIDGET_GET_GAME_INSTANCE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_WIDGET_GET_FLOAT_DELEGATE_SIGNATURE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_WIDGET_GET_DESIRED_SIZE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_WIDGET_GET_CLIPPING: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_WIDGET_GET_CHECK_BOX_STATE_DELEGATE_SIGNATURE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_WIDGET_GET_CACHED_GEOMETRY: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_WIDGET_GET_BOOL_DELEGATE_SIGNATURE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_WIDGET_GET_ACCESSIBLE_TEXT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_WIDGET_GET_ACCESSIBLE_SUMMARY_TEXT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_WIDGET_GENERATE_WIDGET_FOR_STRING_DELEGATE_SIGNATURE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_WIDGET_GENERATE_WIDGET_FOR_OBJECT_DELEGATE_SIGNATURE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_WIDGET_FORCE_VOLATILE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_WIDGET_FORCE_LAYOUT_PREPASS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_USER_WIDGET_UNREGISTER_INPUT_COMPONENT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_USER_WIDGET_UNBIND_FROM_ANIMATION_STARTED: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_USER_WIDGET_UNBIND_FROM_ANIMATION_FINISHED: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_USER_WIDGET_UNBIND_ALL_FROM_ANIMATION_STARTED: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_USER_WIDGET_UNBIND_ALL_FROM_ANIMATION_FINISHED: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_USER_WIDGET_TICK: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_USER_WIDGET_STOP_LISTENING_FOR_INPUT_ACTION: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_USER_WIDGET_STOP_LISTENING_FOR_ALL_INPUT_ACTIONS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_USER_WIDGET_STOP_ANIMATIONS_AND_LATENT_ACTIONS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_USER_WIDGET_STOP_ANIMATION: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_USER_WIDGET_STOP_ALL_ANIMATIONS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_USER_WIDGET_SET_POSITION_IN_VIEWPORT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_USER_WIDGET_SET_PLAYBACK_SPEED: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_USER_WIDGET_SET_PADDING: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_USER_WIDGET_SET_OWNING_PLAYER: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_USER_WIDGET_SET_NUM_LOOPS_TO_PLAY: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_USER_WIDGET_SET_INPUT_ACTION_PRIORITY: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_USER_WIDGET_SET_INPUT_ACTION_BLOCKING: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_USER_WIDGET_SET_FOREGROUND_COLOR: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_USER_WIDGET_SET_DESIRED_SIZE_IN_VIEWPORT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_USER_WIDGET_SET_DESIRED_FOCUS_WIDGET: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_USER_WIDGET_SET_COLOR_AND_OPACITY: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_USER_WIDGET_SET_ANIMATION_CURRENT_TIME: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_USER_WIDGET_SET_ANCHORS_IN_VIEWPORT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_USER_WIDGET_SET_ALIGNMENT_IN_VIEWPORT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_USER_WIDGET_REVERSE_ANIMATION: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_USER_WIDGET_REMOVE_FROM_VIEWPORT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_USER_WIDGET_REMOVE_EXTENSIONS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_USER_WIDGET_REMOVE_EXTENSION: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_USER_WIDGET_REGISTER_INPUT_COMPONENT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_USER_WIDGET_QUEUE_STOP_ANIMATION: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_USER_WIDGET_QUEUE_STOP_ALL_ANIMATIONS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_USER_WIDGET_QUEUE_PLAY_ANIMATION_TIME_RANGE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_USER_WIDGET_QUEUE_PLAY_ANIMATION_REVERSE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_USER_WIDGET_QUEUE_PLAY_ANIMATION_FORWARD: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_USER_WIDGET_QUEUE_PLAY_ANIMATION: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_USER_WIDGET_QUEUE_PAUSE_ANIMATION: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_USER_WIDGET_PRE_CONSTRUCT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_USER_WIDGET_PLAY_SOUND: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_USER_WIDGET_PLAY_ANIMATION_TIME_RANGE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_USER_WIDGET_PLAY_ANIMATION_REVERSE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_USER_WIDGET_PLAY_ANIMATION_FORWARD: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_USER_WIDGET_PLAY_ANIMATION: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_USER_WIDGET_PAUSE_ANIMATION: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_USER_WIDGET_ON_TOUCH_STARTED: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_USER_WIDGET_ON_TOUCH_MOVED: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_USER_WIDGET_ON_TOUCH_GESTURE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_USER_WIDGET_ON_TOUCH_FORCE_CHANGED: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_USER_WIDGET_ON_TOUCH_FIRST_MOVE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_USER_WIDGET_ON_TOUCH_ENDED: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_USER_WIDGET_ON_REMOVED_FROM_FOCUS_PATH: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_USER_WIDGET_ON_PREVIEW_MOUSE_BUTTON_DOWN: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_USER_WIDGET_ON_PREVIEW_KEY_DOWN: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_USER_WIDGET_ON_PAINT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_USER_WIDGET_ON_MOUSE_WHEEL: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_USER_WIDGET_ON_MOUSE_MOVE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_USER_WIDGET_ON_MOUSE_LEAVE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_USER_WIDGET_ON_MOUSE_ENTER: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_USER_WIDGET_ON_MOUSE_CAPTURE_LOST: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_USER_WIDGET_ON_MOUSE_BUTTON_UP: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_USER_WIDGET_ON_MOUSE_BUTTON_DOWN: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_USER_WIDGET_ON_MOUSE_BUTTON_DOUBLE_CLICK: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_USER_WIDGET_ON_MOTION_DETECTED: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_USER_WIDGET_ON_KEY_UP: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_USER_WIDGET_ON_KEY_DOWN: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_USER_WIDGET_ON_KEY_CHAR: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_USER_WIDGET_ON_INITIALIZED: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_USER_WIDGET_ON_FOCUS_RECEIVED: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_USER_WIDGET_ON_FOCUS_LOST: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_USER_WIDGET_ON_DROP: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_USER_WIDGET_ON_DRAG_OVER: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_USER_WIDGET_ON_DRAG_LEAVE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_USER_WIDGET_ON_DRAG_ENTER: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_USER_WIDGET_ON_DRAG_DETECTED: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_USER_WIDGET_ON_DRAG_CANCELLED: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_USER_WIDGET_ON_ANIMATION_STARTED: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_USER_WIDGET_ON_ANIMATION_FINISHED: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_USER_WIDGET_ON_ANALOG_VALUE_CHANGED: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_USER_WIDGET_ON_ADDED_TO_FOCUS_PATH: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_USER_WIDGET_LISTEN_FOR_INPUT_ACTION: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_USER_WIDGET_IS_PLAYING_ANIMATION: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_USER_WIDGET_IS_LISTENING_FOR_INPUT_ACTION: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_USER_WIDGET_IS_INTERACTABLE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_USER_WIDGET_IS_ANY_ANIMATION_PLAYING: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_USER_WIDGET_IS_ANIMATION_PLAYING_FORWARD: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_USER_WIDGET_IS_ANIMATION_PLAYING: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_USER_WIDGET_GET_OWNING_PLAYER_PAWN: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_USER_WIDGET_GET_OWNING_PLAYER_CAMERA_MANAGER: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_USER_WIDGET_GET_IS_VISIBLE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_USER_WIDGET_GET_EXTENSIONS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_USER_WIDGET_GET_EXTENSION: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_USER_WIDGET_GET_ANIMATION_CURRENT_TIME: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_USER_WIDGET_GET_ANCHORS_IN_VIEWPORT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_USER_WIDGET_GET_ALIGNMENT_IN_VIEWPORT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_USER_WIDGET_FLUSH_ANIMATIONS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_USER_WIDGET_DESTRUCT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_USER_WIDGET_CONSTRUCT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_USER_WIDGET_CANCEL_LATENT_ACTIONS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_USER_WIDGET_BIND_TO_ANIMATION_STARTED: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_USER_WIDGET_BIND_TO_ANIMATION_FINISHED: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_USER_WIDGET_BIND_TO_ANIMATION_EVENT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_USER_WIDGET_ADD_TO_VIEWPORT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_USER_WIDGET_ADD_TO_PLAYER_SCREEN: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_USER_WIDGET_ADD_EXTENSION: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_WIDGET_COMPONENT_SET_WINDOW_VISIBILITY: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_WIDGET_COMPONENT_SET_WINDOW_FOCUSABLE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_WIDGET_COMPONENT_SET_WIDGET_SPACE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_WIDGET_COMPONENT_SET_WIDGET: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_WIDGET_COMPONENT_SET_TWO_SIDED: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_WIDGET_COMPONENT_SET_TINT_COLOR_AND_OPACITY: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_WIDGET_COMPONENT_SET_TICK_WHEN_OFFSCREEN: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_WIDGET_COMPONENT_SET_TICK_MODE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_WIDGET_COMPONENT_SET_REDRAW_TIME: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_WIDGET_COMPONENT_SET_PIVOT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_WIDGET_COMPONENT_SET_OWNER_PLAYER: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_WIDGET_COMPONENT_SET_MANUALLY_REDRAW: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_WIDGET_COMPONENT_SET_GEOMETRY_MODE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_WIDGET_COMPONENT_SET_DRAW_SIZE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_WIDGET_COMPONENT_SET_DRAW_AT_DESIRED_SIZE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_WIDGET_COMPONENT_SET_CYLINDER_ARC_ANGLE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_WIDGET_COMPONENT_SET_BACKGROUND_COLOR: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_WIDGET_COMPONENT_REQUEST_RENDER_UPDATE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_WIDGET_COMPONENT_REQUEST_REDRAW: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_WIDGET_COMPONENT_IS_WIDGET_VISIBLE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_WIDGET_COMPONENT_GET_WINDOW_VISIBLILITY: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_WIDGET_COMPONENT_GET_WINDOW_FOCUSABLE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_WIDGET_COMPONENT_GET_WIDGET_SPACE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_WIDGET_COMPONENT_GET_WIDGET: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_WIDGET_COMPONENT_GET_USER_WIDGET_OBJECT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_WIDGET_COMPONENT_GET_TWO_SIDED: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_WIDGET_COMPONENT_GET_TICK_WHEN_OFFSCREEN: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_WIDGET_COMPONENT_GET_RENDER_TARGET: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_WIDGET_COMPONENT_GET_REDRAW_TIME: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_WIDGET_COMPONENT_GET_PIVOT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_WIDGET_COMPONENT_GET_OWNER_PLAYER: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_WIDGET_COMPONENT_GET_MATERIAL_INSTANCE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_WIDGET_COMPONENT_GET_MANUALLY_REDRAW: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_WIDGET_COMPONENT_GET_GEOMETRY_MODE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_WIDGET_COMPONENT_GET_DRAW_SIZE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_WIDGET_COMPONENT_GET_DRAW_AT_DESIRED_SIZE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_WIDGET_COMPONENT_GET_CYLINDER_ARC_ANGLE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_WIDGET_COMPONENT_GET_CURRENT_DRAW_SIZE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_PANEL_WIDGET_REMOVE_CHILD_AT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_PANEL_WIDGET_REMOVE_CHILD: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_PANEL_WIDGET_HAS_CHILD: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_PANEL_WIDGET_HAS_ANY_CHILDREN: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_PANEL_WIDGET_GET_CHILDREN_COUNT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_PANEL_WIDGET_GET_CHILD_INDEX: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_PANEL_WIDGET_GET_CHILD_AT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_PANEL_WIDGET_GET_ALL_CHILDREN: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_PANEL_WIDGET_CLEAR_CHILDREN: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_PANEL_WIDGET_ADD_CHILD: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_CONTENT_WIDGET_SET_CONTENT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_CONTENT_WIDGET_GET_CONTENT_SLOT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_CONTENT_WIDGET_GET_CONTENT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_BUTTON_SET_TOUCH_METHOD: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_BUTTON_SET_STYLE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_BUTTON_SET_PRESS_METHOD: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_BUTTON_SET_COLOR_AND_OPACITY: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_BUTTON_SET_CLICK_METHOD: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_BUTTON_SET_BACKGROUND_COLOR: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_BUTTON_SET_ALLOW_DRAG_DROP: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_BUTTON_IS_PRESSED: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_CHECK_BOX_SET_TOUCH_METHOD: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_CHECK_BOX_SET_PRESS_METHOD: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_CHECK_BOX_SET_IS_CHECKED: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_CHECK_BOX_SET_CLICK_METHOD: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_CHECK_BOX_SET_CHECKED_STATE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_CHECK_BOX_IS_PRESSED: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_CHECK_BOX_IS_CHECKED: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_CHECK_BOX_GET_CHECKED_STATE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_CIRCULAR_THROBBER_SET_RADIUS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_CIRCULAR_THROBBER_SET_PERIOD: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_CIRCULAR_THROBBER_SET_NUMBER_OF_PIECES: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_COMBO_BOX_KEY_SET_SELECTED_OPTION: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_COMBO_BOX_KEY_REMOVE_OPTION: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_COMBO_BOX_KEY_ON_SELECTION_CHANGED_EVENT_DELEGATE_SIGNATURE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_COMBO_BOX_KEY_ON_OPENING_EVENT_DELEGATE_SIGNATURE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_COMBO_BOX_KEY_IS_OPEN: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_COMBO_BOX_KEY_GET_SELECTED_OPTION: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_COMBO_BOX_KEY_GENERATE_WIDGET_EVENT_DELEGATE_SIGNATURE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_COMBO_BOX_KEY_CLEAR_SELECTION: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_COMBO_BOX_KEY_CLEAR_OPTIONS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_COMBO_BOX_KEY_ADD_OPTION: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_COMBO_BOX_STRING_SET_SELECTED_OPTION: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_COMBO_BOX_STRING_SET_SELECTED_INDEX: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_COMBO_BOX_STRING_REMOVE_OPTION: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_COMBO_BOX_STRING_REFRESH_OPTIONS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_COMBO_BOX_STRING_ON_SELECTION_CHANGED_EVENT_DELEGATE_SIGNATURE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_COMBO_BOX_STRING_ON_OPENING_EVENT_DELEGATE_SIGNATURE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_COMBO_BOX_STRING_IS_OPEN: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_COMBO_BOX_STRING_GET_SELECTED_OPTION: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_COMBO_BOX_STRING_GET_SELECTED_INDEX: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_COMBO_BOX_STRING_GET_OPTION_COUNT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_COMBO_BOX_STRING_GET_OPTION_AT_INDEX: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_COMBO_BOX_STRING_FIND_OPTION_INDEX: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_COMBO_BOX_STRING_CLEAR_SELECTION: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_COMBO_BOX_STRING_CLEAR_OPTIONS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_COMBO_BOX_STRING_ADD_OPTION: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_EDITABLE_TEXT_TOGGLE_VIRTUAL_KEYBOARD: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_EDITABLE_TEXT_SET_TEXT_OVERFLOW_POLICY: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_EDITABLE_TEXT_SET_TEXT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_EDITABLE_TEXT_SET_MINIMUM_DESIRED_WIDTH: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_EDITABLE_TEXT_SET_JUSTIFICATION: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_EDITABLE_TEXT_SET_IS_READ_ONLY: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_EDITABLE_TEXT_SET_IS_PASSWORD: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_EDITABLE_TEXT_SET_HINT_TEXT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_EDITABLE_TEXT_SET_FONT_OUTLINE_MATERIAL: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_EDITABLE_TEXT_SET_FONT_MATERIAL: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_EDITABLE_TEXT_SET_FONT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_EDITABLE_TEXT_ON_EDITABLE_TEXT_COMMITTED_EVENT_DELEGATE_SIGNATURE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_EDITABLE_TEXT_ON_EDITABLE_TEXT_CHANGED_EVENT_DELEGATE_SIGNATURE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_EDITABLE_TEXT_GET_TEXT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_EDITABLE_TEXT_GET_JUSTIFICATION: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_EDITABLE_TEXT_GET_HINT_TEXT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_EDITABLE_TEXT_GET_FONT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_EDITABLE_TEXT_BOX_SET_TEXT_OVERFLOW_POLICY: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_EDITABLE_TEXT_BOX_SET_TEXT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_EDITABLE_TEXT_BOX_SET_JUSTIFICATION: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_EDITABLE_TEXT_BOX_SET_IS_READ_ONLY: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_EDITABLE_TEXT_BOX_SET_IS_PASSWORD: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_EDITABLE_TEXT_BOX_SET_HINT_TEXT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_EDITABLE_TEXT_BOX_SET_FOREGROUND_COLOR: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_EDITABLE_TEXT_BOX_SET_ERROR: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_EDITABLE_TEXT_BOX_ON_EDITABLE_TEXT_BOX_COMMITTED_EVENT_DELEGATE_SIGNATURE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_EDITABLE_TEXT_BOX_ON_EDITABLE_TEXT_BOX_CHANGED_EVENT_DELEGATE_SIGNATURE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_EDITABLE_TEXT_BOX_HAS_ERROR: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_EDITABLE_TEXT_BOX_GET_TEXT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_EDITABLE_TEXT_BOX_CLEAR_ERROR: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_EXPANDABLE_AREA_SET_IS_EXPANDED_ANIMATED: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_EXPANDABLE_AREA_SET_IS_EXPANDED: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_EXPANDABLE_AREA_GET_IS_EXPANDED: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_INPUT_KEY_SELECTOR_SET_TEXT_BLOCK_VISIBILITY: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_INPUT_KEY_SELECTOR_SET_SELECTED_KEY: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_INPUT_KEY_SELECTOR_SET_NO_KEY_SPECIFIED_TEXT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_INPUT_KEY_SELECTOR_SET_KEY_SELECTION_TEXT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_INPUT_KEY_SELECTOR_SET_ESCAPE_KEYS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_INPUT_KEY_SELECTOR_SET_ALLOW_MODIFIER_KEYS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_INPUT_KEY_SELECTOR_SET_ALLOW_GAMEPAD_KEYS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_INPUT_KEY_SELECTOR_ON_KEY_SELECTED_DELEGATE_SIGNATURE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_INPUT_KEY_SELECTOR_ON_IS_SELECTING_KEY_CHANGED_DELEGATE_SIGNATURE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_INPUT_KEY_SELECTOR_GET_IS_SELECTING_KEY: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_LIST_VIEW_BASE_SET_WHEEL_SCROLL_MULTIPLIER: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_LIST_VIEW_BASE_SET_SCROLL_OFFSET: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_LIST_VIEW_BASE_SET_SCROLLBAR_VISIBILITY: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_LIST_VIEW_BASE_SET_IS_POINTER_SCROLLING_ENABLED: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_LIST_VIEW_BASE_SET_IS_GAMEPAD_SCROLLING_ENABLED: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_LIST_VIEW_BASE_SET_ALLOW_OVER_SCROLL: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_LIST_VIEW_BASE_SCROLL_TO_TOP: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_LIST_VIEW_BASE_SCROLL_TO_BOTTOM: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_LIST_VIEW_BASE_REQUEST_REFRESH: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_LIST_VIEW_BASE_REGENERATE_ALL_ENTRIES: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_LIST_VIEW_BASE_GET_SCROLL_OFFSET: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_LIST_VIEW_BASE_GET_IS_DRAGGING_LIST_ITEM: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_LIST_VIEW_BASE_GET_DISPLAYED_ENTRY_WIDGETS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_LIST_VIEW_BASE_END_INERTIAL_SCROLLING: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_LIST_VIEW_BASE_CREATE_DRAG_DROP_OPERATION: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_LIST_VIEW_BASE_CANCEL_LIST_VIEW_DRAG_DROP: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_LIST_VIEW_SET_SELECTION_MODE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_LIST_VIEW_SET_SELECTED_INDEX: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_LIST_VIEW_SET_SCROLL_INTO_VIEW_ALIGNMENT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_LIST_VIEW_SET_SCROLL_BAR_PADDING: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_LIST_VIEW_SCROLL_INDEX_INTO_VIEW: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_LIST_VIEW_REMOVE_ITEM: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_LIST_VIEW_ON_LIST_ITEM_OUTER_END_PLAYED: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_LIST_VIEW_ON_LIST_ITEM_END_PLAYED: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_LIST_VIEW_NAVIGATE_TO_INDEX: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_LIST_VIEW_IS_REFRESH_PENDING: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_LIST_VIEW_GET_VERTICAL_ENTRY_SPACING: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_LIST_VIEW_GET_SCROLL_BAR_PADDING: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_LIST_VIEW_GET_NUM_ITEMS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_LIST_VIEW_GET_LIST_ITEMS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_LIST_VIEW_GET_ITEM_AT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_LIST_VIEW_GET_INDEX_FOR_ITEM: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_LIST_VIEW_GET_HORIZONTAL_ENTRY_SPACING: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_LIST_VIEW_CLEAR_LIST_ITEMS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_LIST_VIEW_BP_SET_SELECTED_ITEM: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_LIST_VIEW_BP_SET_LIST_ITEMS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_LIST_VIEW_BP_SET_ITEM_SELECTION: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_LIST_VIEW_BP_SCROLL_ITEM_INTO_VIEW: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_LIST_VIEW_BP_NAVIGATE_TO_ITEM: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_LIST_VIEW_BP_IS_ITEM_VISIBLE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_LIST_VIEW_BP_GET_SELECTED_ITEMS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_LIST_VIEW_BP_GET_SELECTED_ITEM: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_LIST_VIEW_BP_GET_NUM_ITEMS_SELECTED: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_LIST_VIEW_BP_CLEAR_SELECTION: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_LIST_VIEW_BP_CANCEL_SCROLL_INTO_VIEW: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_LIST_VIEW_ADD_ITEM: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_TEXT_LAYOUT_WIDGET_SET_JUSTIFICATION: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MULTI_LINE_EDITABLE_TEXT_SET_WIDGET_STYLE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MULTI_LINE_EDITABLE_TEXT_SET_TEXT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MULTI_LINE_EDITABLE_TEXT_SET_IS_READ_ONLY: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MULTI_LINE_EDITABLE_TEXT_SET_HINT_TEXT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MULTI_LINE_EDITABLE_TEXT_SET_FONT_OUTLINE_MATERIAL: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MULTI_LINE_EDITABLE_TEXT_SET_FONT_MATERIAL: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MULTI_LINE_EDITABLE_TEXT_SET_FONT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MULTI_LINE_EDITABLE_TEXT_ON_MULTI_LINE_EDITABLE_TEXT_COMMITTED_EVENT_DELEGATE_SIGNATURE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MULTI_LINE_EDITABLE_TEXT_ON_MULTI_LINE_EDITABLE_TEXT_CHANGED_EVENT_DELEGATE_SIGNATURE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MULTI_LINE_EDITABLE_TEXT_GET_TEXT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MULTI_LINE_EDITABLE_TEXT_GET_HINT_TEXT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MULTI_LINE_EDITABLE_TEXT_GET_FONT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MULTI_LINE_EDITABLE_TEXT_BOX_SET_TEXT_STYLE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MULTI_LINE_EDITABLE_TEXT_BOX_SET_TEXT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MULTI_LINE_EDITABLE_TEXT_BOX_SET_IS_READ_ONLY: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MULTI_LINE_EDITABLE_TEXT_BOX_SET_HINT_TEXT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MULTI_LINE_EDITABLE_TEXT_BOX_SET_FOREGROUND_COLOR: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MULTI_LINE_EDITABLE_TEXT_BOX_SET_ERROR: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MULTI_LINE_EDITABLE_TEXT_BOX_ON_MULTI_LINE_EDITABLE_TEXT_BOX_COMMITTED_EVENT_DELEGATE_SIGNATURE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MULTI_LINE_EDITABLE_TEXT_BOX_ON_MULTI_LINE_EDITABLE_TEXT_BOX_CHANGED_EVENT_DELEGATE_SIGNATURE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MULTI_LINE_EDITABLE_TEXT_BOX_GET_TEXT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MULTI_LINE_EDITABLE_TEXT_BOX_GET_HINT_TEXT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_PROGRESS_BAR_SET_PERCENT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_PROGRESS_BAR_SET_IS_MARQUEE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_PROGRESS_BAR_SET_FILL_COLOR_AND_OPACITY: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_SCROLL_BAR_SET_STATE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_SCROLL_BOX_SET_WHEEL_SCROLL_MULTIPLIER: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_SCROLL_BOX_SET_SCROLL_WHEN_FOCUS_CHANGES: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_SCROLL_BOX_SET_SCROLL_OFFSET: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_SCROLL_BOX_SET_SCROLL_BAR_VISIBILITY: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_SCROLL_BOX_SET_SCROLLBAR_THICKNESS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_SCROLL_BOX_SET_SCROLLBAR_PADDING: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_SCROLL_BOX_SET_SCROLL_ANIMATION_INTERPOLATION_SPEED: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_SCROLL_BOX_SET_ORIENTATION: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_SCROLL_BOX_SET_NAVIGATION_DESTINATION: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_SCROLL_BOX_SET_IS_TOUCH_SCROLLING_ENABLED: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_SCROLL_BOX_SET_IS_FOCUSABLE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_SCROLL_BOX_SET_CONSUME_POINTER_INPUT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_SCROLL_BOX_SET_CONSUME_MOUSE_WHEEL: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_SCROLL_BOX_SET_ANIMATE_WHEEL_SCROLLING: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_SCROLL_BOX_SET_ANALOG_MOUSE_WHEEL_KEY: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_SCROLL_BOX_SET_ALWAYS_SHOW_SCROLLBAR: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_SCROLL_BOX_SET_ALLOW_OVERSCROLL: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_SCROLL_BOX_SCROLL_WIDGET_INTO_VIEW: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_SCROLL_BOX_SCROLL_TO_START: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_SCROLL_BOX_SCROLL_TO_END: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_SCROLL_BOX_GET_VIEW_OFFSET_FRACTION: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_SCROLL_BOX_GET_VIEW_FRACTION: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_SCROLL_BOX_GET_SCROLL_OFFSET_OF_END: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_SCROLL_BOX_GET_SCROLL_OFFSET: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_SCROLL_BOX_GET_OVERSCROLL_PERCENTAGE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_SCROLL_BOX_GET_OVERSCROLL_OFFSET: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_SCROLL_BOX_GET_IS_SCROLLING: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_SCROLL_BOX_GET_IS_FOCUSABLE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_SCROLL_BOX_GET_CONSUME_POINTER_INPUT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_SCROLL_BOX_GET_ANALOG_MOUSE_WHEEL_KEY: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_SCROLL_BOX_END_INERTIAL_SCROLLING: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_SLIDER_SET_VALUE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_SLIDER_SET_STEP_SIZE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_SLIDER_SET_SLIDER_HANDLE_COLOR: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_SLIDER_SET_SLIDER_BAR_COLOR: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_SLIDER_SET_MIN_VALUE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_SLIDER_SET_MAX_VALUE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_SLIDER_SET_LOCKED: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_SLIDER_SET_INDENT_HANDLE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_SLIDER_GET_VALUE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_SLIDER_GET_NORMALIZED_VALUE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_SPIN_BOX_SET_VALUE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_SPIN_BOX_SET_MIN_VALUE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_SPIN_BOX_SET_MIN_SLIDER_VALUE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_SPIN_BOX_SET_MIN_FRACTIONAL_DIGITS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_SPIN_BOX_SET_MAX_VALUE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_SPIN_BOX_SET_MAX_SLIDER_VALUE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_SPIN_BOX_SET_MAX_FRACTIONAL_DIGITS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_SPIN_BOX_SET_FOREGROUND_COLOR: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_SPIN_BOX_SET_DELTA: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_SPIN_BOX_SET_ALWAYS_USES_DELTA_SNAP: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_SPIN_BOX_ON_SPIN_BOX_VALUE_COMMITTED_EVENT_DELEGATE_SIGNATURE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_SPIN_BOX_ON_SPIN_BOX_VALUE_CHANGED_EVENT_DELEGATE_SIGNATURE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_SPIN_BOX_ON_SPIN_BOX_BEGIN_SLIDER_MOVEMENT_DELEGATE_SIGNATURE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_SPIN_BOX_GET_VALUE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_SPIN_BOX_GET_MIN_VALUE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_SPIN_BOX_GET_MIN_SLIDER_VALUE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_SPIN_BOX_GET_MIN_FRACTIONAL_DIGITS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_SPIN_BOX_GET_MAX_VALUE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_SPIN_BOX_GET_MAX_SLIDER_VALUE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_SPIN_BOX_GET_MAX_FRACTIONAL_DIGITS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_SPIN_BOX_GET_DELTA: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_SPIN_BOX_GET_ALWAYS_USES_DELTA_SNAP: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_SPIN_BOX_CLEAR_MIN_VALUE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_SPIN_BOX_CLEAR_MIN_SLIDER_VALUE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_SPIN_BOX_CLEAR_MAX_VALUE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_SPIN_BOX_CLEAR_MAX_SLIDER_VALUE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_THROBBER_SET_NUMBER_OF_PIECES: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_THROBBER_SET_ANIMATE_VERTICALLY: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_THROBBER_SET_ANIMATE_OPACITY: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_THROBBER_SET_ANIMATE_HORIZONTALLY: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_TREE_VIEW_SET_ITEM_EXPANSION: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_TREE_VIEW_EXPAND_ALL: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_TREE_VIEW_COLLAPSE_ALL: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_SLATE_ACCESSIBLE_WIDGET_DATA_GET_TEXT_DELEGATE_SIGNATURE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut UUMG_SEQUENCE_PLAYER_SET_USER_TAG: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut UUMG_SEQUENCE_PLAYER_GET_USER_TAG: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_WIDGET_ANIMATION_UNBIND_FROM_ANIMATION_STARTED: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_WIDGET_ANIMATION_UNBIND_FROM_ANIMATION_FINISHED: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_WIDGET_ANIMATION_UNBIND_ALL_FROM_ANIMATION_STARTED: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_WIDGET_ANIMATION_UNBIND_ALL_FROM_ANIMATION_FINISHED: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_WIDGET_ANIMATION_GET_START_TIME: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_WIDGET_ANIMATION_GET_END_TIME: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_WIDGET_ANIMATION_BIND_TO_ANIMATION_STARTED: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_WIDGET_ANIMATION_BIND_TO_ANIMATION_FINISHED: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_WIDGET_ANIMATION_HANDLE_FUNCTION_LIBRARY_SET_USER_TAG: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_WIDGET_ANIMATION_HANDLE_FUNCTION_LIBRARY_GET_USER_TAG: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_WIDGET_ANIMATION_PLAY_CALLBACK_PROXY_NEW_PLAY_ANIMATION_TIME_RANGE_PROXY_OBJECT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_WIDGET_ANIMATION_PLAY_CALLBACK_PROXY_NEW_PLAY_ANIMATION_PROXY_OBJECT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_WIDGET_ANIMATION_PLAY_CALLBACK_PROXY_CREATE_PLAY_ANIMATION_TIME_RANGE_PROXY_OBJECT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_WIDGET_ANIMATION_PLAY_CALLBACK_PROXY_CREATE_PLAY_ANIMATION_PROXY_OBJECT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_BOOL_BINDING_GET_VALUE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_BRUSH_BINDING_GET_VALUE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_CHECKED_STATE_BINDING_GET_VALUE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_COLOR_BINDING_GET_SLATE_VALUE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_COLOR_BINDING_GET_LINEAR_VALUE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_FLOAT_BINDING_GET_VALUE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_INT32_BINDING_GET_VALUE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MOUSE_CURSOR_BINDING_GET_VALUE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_TEXT_BINDING_GET_TEXT_VALUE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_TEXT_BINDING_GET_STRING_VALUE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_VISIBILITY_BINDING_GET_VALUE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_WIDGET_BINDING_GET_VALUE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_ASYNC_TASK_DOWNLOAD_IMAGE_DOWNLOAD_IMAGE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_GAME_VIEWPORT_SUBSYSTEM_SET_WIDGET_SLOT_POSITION: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_GAME_VIEWPORT_SUBSYSTEM_SET_WIDGET_SLOT_DESIRED_SIZE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_GAME_VIEWPORT_SUBSYSTEM_SET_WIDGET_SLOT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_GAME_VIEWPORT_SUBSYSTEM_REMOVE_WIDGET: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_GAME_VIEWPORT_SUBSYSTEM_IS_WIDGET_ADDED: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_GAME_VIEWPORT_SUBSYSTEM_GET_WIDGET_SLOT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_GAME_VIEWPORT_SUBSYSTEM_ADD_WIDGET_FOR_PLAYER: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_GAME_VIEWPORT_SUBSYSTEM_ADD_WIDGET: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_USER_LIST_ENTRY_BP_ON_UPDATE_ENTRY_DROP_INDICATOR: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_USER_LIST_ENTRY_BP_ON_ITEM_SELECTION_CHANGED: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_USER_LIST_ENTRY_BP_ON_ITEM_EXPANSION_CHANGED: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_USER_LIST_ENTRY_BP_ON_ENTRY_RELEASED: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_USER_LIST_ENTRY_BP_ON_ENTRY_DROPPED: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_USER_LIST_ENTRY_BP_ON_ENTRY_DRAG_OVER_CHANGED: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_USER_LIST_ENTRY_BP_ON_ENTRY_DRAGGED: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_USER_LIST_ENTRY_BP_ON_END_ENTRY_DROP_OPERATION: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_USER_LIST_ENTRY_LIBRARY_IS_LIST_ITEM_SELECTED: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_USER_LIST_ENTRY_LIBRARY_IS_LIST_ITEM_EXPANDED: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_USER_LIST_ENTRY_LIBRARY_GET_OWNING_LIST_VIEW: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_USER_OBJECT_LIST_ENTRY_ON_LIST_ITEM_OBJECT_SET: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_USER_OBJECT_LIST_ENTRY_LIBRARY_IS_LAST_WIDGET: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_USER_OBJECT_LIST_ENTRY_LIBRARY_IS_FIRST_WIDGET: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_USER_OBJECT_LIST_ENTRY_LIBRARY_GET_LIST_ITEM_OBJECT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_USER_OBJECT_LIST_ENTRY_LIBRARY_GET_LIST_ITEM_INDEX: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_BACKGROUND_BLUR_SET_VERTICAL_ALIGNMENT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_BACKGROUND_BLUR_SET_PADDING: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_BACKGROUND_BLUR_SET_LOW_QUALITY_FALLBACK_BRUSH: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_BACKGROUND_BLUR_SET_HORIZONTAL_ALIGNMENT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_BACKGROUND_BLUR_SET_CORNER_RADIUS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_BACKGROUND_BLUR_SET_BLUR_STRENGTH: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_BACKGROUND_BLUR_SET_BLUR_RADIUS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_BACKGROUND_BLUR_SET_APPLY_ALPHA_TO_BLUR: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_PANEL_SLOT_GET_CONTENT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_BACKGROUND_BLUR_SLOT_SET_VERTICAL_ALIGNMENT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_BACKGROUND_BLUR_SLOT_SET_PADDING: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_BACKGROUND_BLUR_SLOT_SET_HORIZONTAL_ALIGNMENT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_BORDER_SET_VERTICAL_ALIGNMENT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_BORDER_SET_SHOW_EFFECT_WHEN_DISABLED: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_BORDER_SET_PADDING: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_BORDER_SET_HORIZONTAL_ALIGNMENT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_BORDER_SET_DESIRED_SIZE_SCALE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_BORDER_SET_CONTENT_COLOR_AND_OPACITY: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_BORDER_SET_BRUSH_FROM_TEXTURE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_BORDER_SET_BRUSH_FROM_MATERIAL: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_BORDER_SET_BRUSH_FROM_ASSET: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_BORDER_SET_BRUSH_COLOR: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_BORDER_SET_BRUSH: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_BORDER_GET_DYNAMIC_MATERIAL: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_BORDER_SLOT_SET_VERTICAL_ALIGNMENT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_BORDER_SLOT_SET_PADDING: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_BORDER_SLOT_SET_HORIZONTAL_ALIGNMENT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_BUTTON_SLOT_SET_VERTICAL_ALIGNMENT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_BUTTON_SLOT_SET_PADDING: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_BUTTON_SLOT_SET_HORIZONTAL_ALIGNMENT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_CANVAS_PANEL_ADD_CHILD_TO_CANVAS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_CANVAS_PANEL_SLOT_SET_Z_ORDER: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_CANVAS_PANEL_SLOT_SET_SIZE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_CANVAS_PANEL_SLOT_SET_POSITION: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_CANVAS_PANEL_SLOT_SET_OFFSETS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_CANVAS_PANEL_SLOT_SET_MINIMUM: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_CANVAS_PANEL_SLOT_SET_MAXIMUM: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_CANVAS_PANEL_SLOT_SET_LAYOUT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_CANVAS_PANEL_SLOT_SET_AUTO_SIZE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_CANVAS_PANEL_SLOT_SET_ANCHORS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_CANVAS_PANEL_SLOT_SET_ALIGNMENT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_CANVAS_PANEL_SLOT_GET_Z_ORDER: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_CANVAS_PANEL_SLOT_GET_SIZE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_CANVAS_PANEL_SLOT_GET_POSITION: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_CANVAS_PANEL_SLOT_GET_OFFSETS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_CANVAS_PANEL_SLOT_GET_LAYOUT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_CANVAS_PANEL_SLOT_GET_AUTO_SIZE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_CANVAS_PANEL_SLOT_GET_ANCHORS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_CANVAS_PANEL_SLOT_GET_ALIGNMENT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_DYNAMIC_ENTRY_BOX_BASE_SET_RADIAL_SETTINGS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_DYNAMIC_ENTRY_BOX_BASE_SET_ENTRY_SPACING: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_DYNAMIC_ENTRY_BOX_BASE_GET_NUM_ENTRIES: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_DYNAMIC_ENTRY_BOX_BASE_GET_ALL_ENTRIES: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_DYNAMIC_ENTRY_BOX_RESET: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_DYNAMIC_ENTRY_BOX_REMOVE_ENTRY: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_DYNAMIC_ENTRY_BOX_BP_CREATE_ENTRY_OF_CLASS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_DYNAMIC_ENTRY_BOX_BP_CREATE_ENTRY: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_GRID_PANEL_SET_ROW_FILL: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_GRID_PANEL_SET_COLUMN_FILL: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_GRID_PANEL_CLEAR_FILL: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_GRID_PANEL_ADD_CHILD_TO_GRID: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_GRID_SLOT_SET_VERTICAL_ALIGNMENT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_GRID_SLOT_SET_ROW_SPAN: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_GRID_SLOT_SET_ROW: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_GRID_SLOT_SET_PADDING: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_GRID_SLOT_SET_NUDGE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_GRID_SLOT_SET_LAYER: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_GRID_SLOT_SET_HORIZONTAL_ALIGNMENT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_GRID_SLOT_SET_COLUMN_SPAN: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_GRID_SLOT_SET_COLUMN: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_HORIZONTAL_BOX_ADD_CHILD_TO_HORIZONTAL_BOX: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_HORIZONTAL_BOX_SLOT_SET_VERTICAL_ALIGNMENT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_HORIZONTAL_BOX_SLOT_SET_SIZE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_HORIZONTAL_BOX_SLOT_SET_PADDING: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_HORIZONTAL_BOX_SLOT_SET_HORIZONTAL_ALIGNMENT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_IMAGE_SET_OPACITY: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_IMAGE_SET_DESIRED_SIZE_OVERRIDE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_IMAGE_SET_COLOR_AND_OPACITY: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_IMAGE_SET_BRUSH_TINT_COLOR: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_IMAGE_SET_BRUSH_RESOURCE_OBJECT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_IMAGE_SET_BRUSH_FROM_TEXTURE_DYNAMIC: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_IMAGE_SET_BRUSH_FROM_TEXTURE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_IMAGE_SET_BRUSH_FROM_SOFT_TEXTURE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_IMAGE_SET_BRUSH_FROM_SOFT_MATERIAL: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_IMAGE_SET_BRUSH_FROM_MATERIAL: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_IMAGE_SET_BRUSH_FROM_ATLAS_INTERFACE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_IMAGE_SET_BRUSH_FROM_ASSET: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_IMAGE_SET_BRUSH: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_IMAGE_GET_DYNAMIC_MATERIAL: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_INVALIDATION_BOX_SET_CAN_CACHE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_INVALIDATION_BOX_INVALIDATE_CACHE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_INVALIDATION_BOX_GET_CAN_CACHE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MENU_ANCHOR_TOGGLE_OPEN: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MENU_ANCHOR_SHOULD_OPEN_DUE_TO_CLICK: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MENU_ANCHOR_SET_PLACEMENT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MENU_ANCHOR_OPEN: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MENU_ANCHOR_IS_OPEN: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MENU_ANCHOR_HAS_OPEN_SUB_MENUS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MENU_ANCHOR_GET_USER_WIDGET_DELEGATE_SIGNATURE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MENU_ANCHOR_GET_MENU_POSITION: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MENU_ANCHOR_FIT_IN_WINDOW: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MENU_ANCHOR_CLOSE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_OVERLAY_REPLACE_OVERLAY_CHILD_AT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_OVERLAY_ADD_CHILD_TO_OVERLAY: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_OVERLAY_SLOT_SET_VERTICAL_ALIGNMENT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_OVERLAY_SLOT_SET_PADDING: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_OVERLAY_SLOT_SET_HORIZONTAL_ALIGNMENT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RETAINER_BOX_SET_TEXTURE_PARAMETER: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RETAINER_BOX_SET_RETAIN_RENDERING: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RETAINER_BOX_SET_RENDERING_PHASE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RETAINER_BOX_SET_EFFECT_MATERIAL: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RETAINER_BOX_REQUEST_RENDER: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RETAINER_BOX_GET_EFFECT_MATERIAL: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RICH_TEXT_BLOCK_SET_TEXT_TRANSFORM_POLICY: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RICH_TEXT_BLOCK_SET_TEXT_STYLE_SET: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RICH_TEXT_BLOCK_SET_TEXT_OVERFLOW_POLICY: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RICH_TEXT_BLOCK_SET_TEXT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RICH_TEXT_BLOCK_SET_MIN_DESIRED_WIDTH: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RICH_TEXT_BLOCK_SET_DEFAULT_TEXT_STYLE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RICH_TEXT_BLOCK_SET_DEFAULT_STRIKE_BRUSH: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RICH_TEXT_BLOCK_SET_DEFAULT_SHADOW_OFFSET: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RICH_TEXT_BLOCK_SET_DEFAULT_SHADOW_COLOR_AND_OPACITY: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RICH_TEXT_BLOCK_SET_DEFAULT_MATERIAL: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RICH_TEXT_BLOCK_SET_DEFAULT_FONT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RICH_TEXT_BLOCK_SET_DEFAULT_COLOR_AND_OPACITY: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RICH_TEXT_BLOCK_SET_DECORATORS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RICH_TEXT_BLOCK_SET_AUTO_WRAP_TEXT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RICH_TEXT_BLOCK_REFRESH_TEXT_LAYOUT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RICH_TEXT_BLOCK_GET_TEXT_STYLE_SET: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RICH_TEXT_BLOCK_GET_TEXT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RICH_TEXT_BLOCK_GET_DEFAULT_DYNAMIC_MATERIAL: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RICH_TEXT_BLOCK_GET_DECORATOR_BY_CLASS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RICH_TEXT_BLOCK_CLEAR_ALL_DEFAULT_STYLE_OVERRIDES: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_SAFE_ZONE_SET_SIDES_TO_PAD: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_SCALE_BOX_SET_USER_SPECIFIED_SCALE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_SCALE_BOX_SET_STRETCH_DIRECTION: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_SCALE_BOX_SET_STRETCH: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_SCALE_BOX_SET_IGNORE_INHERITED_SCALE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_SCALE_BOX_SLOT_SET_VERTICAL_ALIGNMENT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_SCALE_BOX_SLOT_SET_PADDING: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_SCALE_BOX_SLOT_SET_HORIZONTAL_ALIGNMENT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_SCROLL_BOX_SLOT_SET_VERTICAL_ALIGNMENT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_SCROLL_BOX_SLOT_SET_PADDING: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_SCROLL_BOX_SLOT_SET_HORIZONTAL_ALIGNMENT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_SIZE_BOX_SET_WIDTH_OVERRIDE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_SIZE_BOX_SET_MIN_DESIRED_WIDTH: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_SIZE_BOX_SET_MIN_DESIRED_HEIGHT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_SIZE_BOX_SET_MIN_ASPECT_RATIO: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_SIZE_BOX_SET_MAX_DESIRED_WIDTH: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_SIZE_BOX_SET_MAX_DESIRED_HEIGHT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_SIZE_BOX_SET_MAX_ASPECT_RATIO: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_SIZE_BOX_SET_HEIGHT_OVERRIDE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_SIZE_BOX_CLEAR_WIDTH_OVERRIDE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_SIZE_BOX_CLEAR_MIN_DESIRED_WIDTH: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_SIZE_BOX_CLEAR_MIN_DESIRED_HEIGHT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_SIZE_BOX_CLEAR_MIN_ASPECT_RATIO: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_SIZE_BOX_CLEAR_MAX_DESIRED_WIDTH: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_SIZE_BOX_CLEAR_MAX_DESIRED_HEIGHT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_SIZE_BOX_CLEAR_MAX_ASPECT_RATIO: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_SIZE_BOX_CLEAR_HEIGHT_OVERRIDE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_SIZE_BOX_COMPONENT_CLEAR_WIDTH_OVERRIDE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_SIZE_BOX_COMPONENT_CLEAR_MIN_DESIRED_WIDTH: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_SIZE_BOX_COMPONENT_CLEAR_MIN_DESIRED_HEIGHT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_SIZE_BOX_COMPONENT_CLEAR_MIN_ASPECT_RATIO: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_SIZE_BOX_COMPONENT_CLEAR_MAX_DESIRED_WIDTH: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_SIZE_BOX_COMPONENT_CLEAR_MAX_DESIRED_HEIGHT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_SIZE_BOX_COMPONENT_CLEAR_MAX_ASPECT_RATIO: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_SIZE_BOX_COMPONENT_CLEAR_HEIGHT_OVERRIDE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_SIZE_BOX_SLOT_SET_VERTICAL_ALIGNMENT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_SIZE_BOX_SLOT_SET_PADDING: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_SIZE_BOX_SLOT_SET_HORIZONTAL_ALIGNMENT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_SPACER_SET_SIZE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_STACK_BOX_REPLACE_STACK_BOX_CHILD_AT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_STACK_BOX_ADD_CHILD_TO_STACK_BOX: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_TEXT_BLOCK_SET_TEXT_TRANSFORM_POLICY: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_TEXT_BLOCK_SET_TEXT_OVERFLOW_POLICY: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_TEXT_BLOCK_SET_TEXT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_TEXT_BLOCK_SET_STRIKE_BRUSH: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_TEXT_BLOCK_SET_SHADOW_OFFSET: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_TEXT_BLOCK_SET_SHADOW_COLOR_AND_OPACITY: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_TEXT_BLOCK_SET_OPACITY: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_TEXT_BLOCK_SET_MIN_DESIRED_WIDTH: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_TEXT_BLOCK_SET_FONT_OUTLINE_MATERIAL: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_TEXT_BLOCK_SET_FONT_MATERIAL: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_TEXT_BLOCK_SET_FONT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_TEXT_BLOCK_SET_COLOR_AND_OPACITY: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_TEXT_BLOCK_SET_AUTO_WRAP_TEXT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_TEXT_BLOCK_GET_TEXT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_TEXT_BLOCK_GET_DYNAMIC_OUTLINE_MATERIAL: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_TEXT_BLOCK_GET_DYNAMIC_FONT_MATERIAL: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_TILE_VIEW_SET_ENTRY_WIDTH: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_TILE_VIEW_SET_ENTRY_HEIGHT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_TILE_VIEW_IS_ALIGNED: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_TILE_VIEW_GET_ENTRY_WIDTH: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_TILE_VIEW_GET_ENTRY_HEIGHT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_UNIFORM_GRID_PANEL_SET_SLOT_PADDING: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_UNIFORM_GRID_PANEL_SET_MIN_DESIRED_SLOT_WIDTH: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_UNIFORM_GRID_PANEL_SET_MIN_DESIRED_SLOT_HEIGHT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_UNIFORM_GRID_PANEL_ADD_CHILD_TO_UNIFORM_GRID: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_UNIFORM_GRID_SLOT_SET_VERTICAL_ALIGNMENT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_UNIFORM_GRID_SLOT_SET_ROW: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_UNIFORM_GRID_SLOT_SET_HORIZONTAL_ALIGNMENT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_UNIFORM_GRID_SLOT_SET_COLUMN: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_VERTICAL_BOX_ADD_CHILD_TO_VERTICAL_BOX: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_VERTICAL_BOX_SLOT_SET_VERTICAL_ALIGNMENT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_VERTICAL_BOX_SLOT_SET_SIZE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_VERTICAL_BOX_SLOT_SET_PADDING: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_VERTICAL_BOX_SLOT_SET_HORIZONTAL_ALIGNMENT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_VIEWPORT_SPAWN: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_VIEWPORT_SET_VIEW_ROTATION: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_VIEWPORT_SET_VIEW_LOCATION: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_VIEWPORT_SET_SKY_INTENSITY: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_VIEWPORT_SET_SHOW_FLAG: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_VIEWPORT_SET_LIGHT_INTENSITY: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_VIEWPORT_SET_ENABLE_ADVANCED_FEATURES: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_VIEWPORT_GET_VIEW_ROTATION: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_VIEWPORT_GET_VIEW_PROJECTION_MATRIX: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_VIEWPORT_GET_VIEWPORT_WORLD: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_VIEWPORT_GET_VIEW_LOCATION: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_WIDGET_INTERACTION_COMPONENT_SET_FOCUS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_WIDGET_INTERACTION_COMPONENT_SET_CUSTOM_HIT_RESULT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_WIDGET_INTERACTION_COMPONENT_SEND_KEY_CHAR: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_WIDGET_INTERACTION_COMPONENT_SCROLL_WHEEL: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_WIDGET_INTERACTION_COMPONENT_RELEASE_POINTER_KEY: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_WIDGET_INTERACTION_COMPONENT_RELEASE_KEY: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_WIDGET_INTERACTION_COMPONENT_PRESS_POINTER_KEY: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_WIDGET_INTERACTION_COMPONENT_PRESS_KEY: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_WIDGET_INTERACTION_COMPONENT_PRESS_AND_RELEASE_KEY: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_WIDGET_INTERACTION_COMPONENT_IS_OVER_INTERACTABLE_WIDGET: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_WIDGET_INTERACTION_COMPONENT_IS_OVER_HIT_TEST_VISIBLE_WIDGET: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_WIDGET_INTERACTION_COMPONENT_IS_OVER_FOCUSABLE_WIDGET: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_WIDGET_INTERACTION_COMPONENT_GET_LAST_HIT_RESULT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_WIDGET_INTERACTION_COMPONENT_GET_HOVERED_WIDGET_COMPONENT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_WIDGET_INTERACTION_COMPONENT_GET2_D_HIT_LOCATION: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_WIDGET_SWITCHER_SET_ACTIVE_WIDGET_INDEX: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_WIDGET_SWITCHER_SET_ACTIVE_WIDGET: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_WIDGET_SWITCHER_GET_WIDGET_AT_INDEX: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_WIDGET_SWITCHER_GET_NUM_WIDGETS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_WIDGET_SWITCHER_GET_ACTIVE_WIDGET_INDEX: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_WIDGET_SWITCHER_GET_ACTIVE_WIDGET: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_WIDGET_SWITCHER_SLOT_SET_VERTICAL_ALIGNMENT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_WIDGET_SWITCHER_SLOT_SET_PADDING: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_WIDGET_SWITCHER_SLOT_SET_HORIZONTAL_ALIGNMENT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_WINDOW_TITLE_BAR_AREA_SET_VERTICAL_ALIGNMENT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_WINDOW_TITLE_BAR_AREA_SET_PADDING: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_WINDOW_TITLE_BAR_AREA_SET_HORIZONTAL_ALIGNMENT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_WINDOW_TITLE_BAR_AREA_SLOT_SET_VERTICAL_ALIGNMENT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_WINDOW_TITLE_BAR_AREA_SLOT_SET_PADDING: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_WINDOW_TITLE_BAR_AREA_SLOT_SET_HORIZONTAL_ALIGNMENT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_WRAP_BOX_SET_INNER_SLOT_PADDING: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_WRAP_BOX_SET_HORIZONTAL_ALIGNMENT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_WRAP_BOX_ADD_CHILD_TO_WRAP_BOX: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_WRAP_BOX_SLOT_SET_VERTICAL_ALIGNMENT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_WRAP_BOX_SLOT_SET_PADDING: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_WRAP_BOX_SLOT_SET_NEW_LINE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_WRAP_BOX_SLOT_SET_HORIZONTAL_ALIGNMENT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_WRAP_BOX_SLOT_SET_FILL_SPAN_WHEN_LESS_THAN: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_WRAP_BOX_SLOT_SET_FILL_EMPTY_SPACE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_DRAG_DROP_OPERATION_DROP: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_DRAG_DROP_OPERATION_DRAGGED: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_DRAG_DROP_OPERATION_DRAG_CANCELLED: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_SLATE_BLUEPRINT_LIBRARY_VECTOR_LOCAL_TO_ABSOLUTE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_SLATE_BLUEPRINT_LIBRARY_VECTOR_ABSOLUTE_TO_LOCAL: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_SLATE_BLUEPRINT_LIBRARY_TRANSFORM_VECTOR_LOCAL_TO_ABSOLUTE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_SLATE_BLUEPRINT_LIBRARY_TRANSFORM_VECTOR_ABSOLUTE_TO_LOCAL: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_SLATE_BLUEPRINT_LIBRARY_TRANSFORM_SCALAR_LOCAL_TO_ABSOLUTE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_SLATE_BLUEPRINT_LIBRARY_TRANSFORM_SCALAR_ABSOLUTE_TO_LOCAL: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_SLATE_BLUEPRINT_LIBRARY_SCREEN_TO_WIDGET_LOCAL: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_SLATE_BLUEPRINT_LIBRARY_SCREEN_TO_WIDGET_ABSOLUTE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_SLATE_BLUEPRINT_LIBRARY_SCREEN_TO_VIEWPORT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_SLATE_BLUEPRINT_LIBRARY_SCALAR_LOCAL_TO_ABSOLUTE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_SLATE_BLUEPRINT_LIBRARY_SCALAR_ABSOLUTE_TO_LOCAL: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_SLATE_BLUEPRINT_LIBRARY_LOCAL_TO_VIEWPORT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_SLATE_BLUEPRINT_LIBRARY_LOCAL_TO_ABSOLUTE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_SLATE_BLUEPRINT_LIBRARY_IS_UNDER_LOCATION: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_SLATE_BLUEPRINT_LIBRARY_GET_LOCAL_TOP_LEFT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_SLATE_BLUEPRINT_LIBRARY_GET_LOCAL_SIZE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_SLATE_BLUEPRINT_LIBRARY_GET_ABSOLUTE_SIZE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_SLATE_BLUEPRINT_LIBRARY_EQUAL_EQUAL_SLATE_BRUSH: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_SLATE_BLUEPRINT_LIBRARY_ABSOLUTE_TO_VIEWPORT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_SLATE_BLUEPRINT_LIBRARY_ABSOLUTE_TO_LOCAL: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_USER_WIDGET_FUNCTION_LIBRARY_GET_OUTER_USER_WIDGET: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_USER_WIDGET_FUNCTION_LIBRARY_CONV_UMG_SEQUENCE_PLAYER: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_WIDGET_BLUEPRINT_LIBRARY_UNLOCK_MOUSE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_WIDGET_BLUEPRINT_LIBRARY_UNHANDLED: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_WIDGET_BLUEPRINT_LIBRARY_SET_WINDOW_TITLE_BAR_STATE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_WIDGET_BLUEPRINT_LIBRARY_SET_WINDOW_TITLE_BAR_ON_CLOSE_CLICKED_DELEGATE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_WIDGET_BLUEPRINT_LIBRARY_SET_WINDOW_TITLE_BAR_CLOSE_BUTTON_ACTIVE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_WIDGET_BLUEPRINT_LIBRARY_SET_USER_FOCUS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_WIDGET_BLUEPRINT_LIBRARY_SET_MOUSE_POSITION: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_WIDGET_BLUEPRINT_LIBRARY_SET_INPUT_MODE_UI_ONLY_EX: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_WIDGET_BLUEPRINT_LIBRARY_SET_INPUT_MODE_GAME_ONLY: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_WIDGET_BLUEPRINT_LIBRARY_SET_INPUT_MODE_GAME_AND_UI_EX: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_WIDGET_BLUEPRINT_LIBRARY_SET_HARDWARE_CURSOR: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_WIDGET_BLUEPRINT_LIBRARY_SET_FOCUS_TO_GAME_VIEWPORT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_WIDGET_BLUEPRINT_LIBRARY_SET_COLOR_VISION_DEFICIENCY_TYPE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_WIDGET_BLUEPRINT_LIBRARY_SET_BRUSH_RESOURCE_TO_TEXTURE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_WIDGET_BLUEPRINT_LIBRARY_SET_BRUSH_RESOURCE_TO_MATERIAL: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_WIDGET_BLUEPRINT_LIBRARY_RESTORE_PREVIOUS_WINDOW_TITLE_BAR_STATE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_WIDGET_BLUEPRINT_LIBRARY_RELEASE_MOUSE_CAPTURE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_WIDGET_BLUEPRINT_LIBRARY_RELEASE_JOYSTICK_CAPTURE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_WIDGET_BLUEPRINT_LIBRARY_ON_GAME_WINDOW_CLOSE_BUTTON_CLICKED_DELEGATE_DELEGATE_SIGNATURE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_WIDGET_BLUEPRINT_LIBRARY_NO_RESOURCE_BRUSH: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_WIDGET_BLUEPRINT_LIBRARY_MAKE_BRUSH_FROM_TEXTURE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_WIDGET_BLUEPRINT_LIBRARY_MAKE_BRUSH_FROM_MATERIAL: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_WIDGET_BLUEPRINT_LIBRARY_MAKE_BRUSH_FROM_ASSET: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_WIDGET_BLUEPRINT_LIBRARY_LOCK_MOUSE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_WIDGET_BLUEPRINT_LIBRARY_IS_DRAG_DROPPING: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_WIDGET_BLUEPRINT_LIBRARY_HANDLED: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_WIDGET_BLUEPRINT_LIBRARY_GET_SAFE_ZONE_PADDING: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_WIDGET_BLUEPRINT_LIBRARY_GET_KEY_EVENT_FROM_ANALOG_INPUT_EVENT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_WIDGET_BLUEPRINT_LIBRARY_GET_INPUT_EVENT_FROM_POINTER_EVENT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_WIDGET_BLUEPRINT_LIBRARY_GET_INPUT_EVENT_FROM_NAVIGATION_EVENT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_WIDGET_BLUEPRINT_LIBRARY_GET_INPUT_EVENT_FROM_KEY_EVENT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_WIDGET_BLUEPRINT_LIBRARY_GET_INPUT_EVENT_FROM_CHARACTER_EVENT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_WIDGET_BLUEPRINT_LIBRARY_GET_DYNAMIC_MATERIAL: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_WIDGET_BLUEPRINT_LIBRARY_GET_DRAG_DROPPING_CONTENT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_WIDGET_BLUEPRINT_LIBRARY_GET_BRUSH_RESOURCE_AS_TEXTURE2_D: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_WIDGET_BLUEPRINT_LIBRARY_GET_BRUSH_RESOURCE_AS_MATERIAL: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_WIDGET_BLUEPRINT_LIBRARY_GET_BRUSH_RESOURCE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_WIDGET_BLUEPRINT_LIBRARY_GET_ALL_WIDGETS_WITH_INTERFACE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_WIDGET_BLUEPRINT_LIBRARY_GET_ALL_WIDGETS_OF_CLASS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_WIDGET_BLUEPRINT_LIBRARY_END_DRAG_DROP: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_WIDGET_BLUEPRINT_LIBRARY_DRAW_TEXT_FORMATTED: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_WIDGET_BLUEPRINT_LIBRARY_DRAW_TEXT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_WIDGET_BLUEPRINT_LIBRARY_DRAW_SPLINE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_WIDGET_BLUEPRINT_LIBRARY_DRAW_LINES: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_WIDGET_BLUEPRINT_LIBRARY_DRAW_LINE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_WIDGET_BLUEPRINT_LIBRARY_DRAW_BOX: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_WIDGET_BLUEPRINT_LIBRARY_DISMISS_ALL_MENUS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_WIDGET_BLUEPRINT_LIBRARY_DETECT_DRAG_IF_PRESSED: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_WIDGET_BLUEPRINT_LIBRARY_DETECT_DRAG: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_WIDGET_BLUEPRINT_LIBRARY_CREATE_DRAG_DROP_OPERATION: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_WIDGET_BLUEPRINT_LIBRARY_CREATE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_WIDGET_BLUEPRINT_LIBRARY_CLEAR_USER_FOCUS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_WIDGET_BLUEPRINT_LIBRARY_CAPTURE_MOUSE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_WIDGET_BLUEPRINT_LIBRARY_CAPTURE_JOYSTICK: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_WIDGET_BLUEPRINT_LIBRARY_CANCEL_DRAG_DROP: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_WIDGET_LAYOUT_LIBRARY_SLOT_AS_WRAP_BOX_SLOT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_WIDGET_LAYOUT_LIBRARY_SLOT_AS_WIDGET_SWITCHER_SLOT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_WIDGET_LAYOUT_LIBRARY_SLOT_AS_VERTICAL_BOX_SLOT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_WIDGET_LAYOUT_LIBRARY_SLOT_AS_UNIFORM_GRID_SLOT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_WIDGET_LAYOUT_LIBRARY_SLOT_AS_STACK_BOX_SLOT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_WIDGET_LAYOUT_LIBRARY_SLOT_AS_SIZE_BOX_SLOT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_WIDGET_LAYOUT_LIBRARY_SLOT_AS_SCROLL_BOX_SLOT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_WIDGET_LAYOUT_LIBRARY_SLOT_AS_SCALE_BOX_SLOT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_WIDGET_LAYOUT_LIBRARY_SLOT_AS_SAFE_BOX_SLOT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_WIDGET_LAYOUT_LIBRARY_SLOT_AS_OVERLAY_SLOT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_WIDGET_LAYOUT_LIBRARY_SLOT_AS_HORIZONTAL_BOX_SLOT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_WIDGET_LAYOUT_LIBRARY_SLOT_AS_GRID_SLOT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_WIDGET_LAYOUT_LIBRARY_SLOT_AS_CANVAS_SLOT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_WIDGET_LAYOUT_LIBRARY_SLOT_AS_BORDER_SLOT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_WIDGET_LAYOUT_LIBRARY_REMOVE_ALL_WIDGETS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_WIDGET_LAYOUT_LIBRARY_PROJECT_WORLD_LOCATION_TO_WIDGET_POSITION: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_WIDGET_LAYOUT_LIBRARY_GET_VIEWPORT_WIDGET_GEOMETRY: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_WIDGET_LAYOUT_LIBRARY_GET_VIEWPORT_SIZE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_WIDGET_LAYOUT_LIBRARY_GET_VIEWPORT_SCALE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_WIDGET_LAYOUT_LIBRARY_GET_PLAYER_SCREEN_WIDGET_GEOMETRY: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_WIDGET_LAYOUT_LIBRARY_GET_MOUSE_POSITION_SCALED_BY_DPI: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_WIDGET_LAYOUT_LIBRARY_GET_MOUSE_POSITION_ON_VIEWPORT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_WIDGET_LAYOUT_LIBRARY_GET_MOUSE_POSITION_ON_PLATFORM: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
pub fn initialize() {
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UWidget::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetVisibility"),
            &raw mut U_WIDGET_SET_VISIBILITY,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetUserFocus"),
            &raw mut U_WIDGET_SET_USER_FOCUS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetToolTipText"),
            &raw mut U_WIDGET_SET_TOOL_TIP_TEXT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetToolTip"),
            &raw mut U_WIDGET_SET_TOOL_TIP,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetRenderTranslation"),
            &raw mut U_WIDGET_SET_RENDER_TRANSLATION,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetRenderTransformPivot"),
            &raw mut U_WIDGET_SET_RENDER_TRANSFORM_PIVOT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetRenderTransformAngle"),
            &raw mut U_WIDGET_SET_RENDER_TRANSFORM_ANGLE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetRenderTransform"),
            &raw mut U_WIDGET_SET_RENDER_TRANSFORM,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetRenderShear"),
            &raw mut U_WIDGET_SET_RENDER_SHEAR,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetRenderScale"),
            &raw mut U_WIDGET_SET_RENDER_SCALE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetRenderOpacity"),
            &raw mut U_WIDGET_SET_RENDER_OPACITY,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetNavigationRuleExplicit"),
            &raw mut U_WIDGET_SET_NAVIGATION_RULE_EXPLICIT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetNavigationRuleCustomBoundary"),
            &raw mut U_WIDGET_SET_NAVIGATION_RULE_CUSTOM_BOUNDARY,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetNavigationRuleCustom"),
            &raw mut U_WIDGET_SET_NAVIGATION_RULE_CUSTOM,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetNavigationRuleBase"),
            &raw mut U_WIDGET_SET_NAVIGATION_RULE_BASE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetNavigationRule"),
            &raw mut U_WIDGET_SET_NAVIGATION_RULE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetKeyboardFocus"),
            &raw mut U_WIDGET_SET_KEYBOARD_FOCUS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetIsEnabled"),
            &raw mut U_WIDGET_SET_IS_ENABLED,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetFocus"),
            &raw mut U_WIDGET_SET_FOCUS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetCursor"),
            &raw mut U_WIDGET_SET_CURSOR,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetClipping"),
            &raw mut U_WIDGET_SET_CLIPPING,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetAllNavigationRules"),
            &raw mut U_WIDGET_SET_ALL_NAVIGATION_RULES,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ResetCursor"),
            &raw mut U_WIDGET_RESET_CURSOR,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RemoveFromParent"),
            &raw mut U_WIDGET_REMOVE_FROM_PARENT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("OnReply__DelegateSignature"),
            &raw mut U_WIDGET_ON_REPLY_DELEGATE_SIGNATURE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("OnPointerEvent__DelegateSignature"),
            &raw mut U_WIDGET_ON_POINTER_EVENT_DELEGATE_SIGNATURE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("K2_RemoveFieldValueChangedDelegate"),
            &raw mut U_WIDGET_K2_REMOVE_FIELD_VALUE_CHANGED_DELEGATE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("K2_BroadcastFieldValueChanged"),
            &raw mut U_WIDGET_K2_BROADCAST_FIELD_VALUE_CHANGED,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("K2_AddFieldValueChangedDelegate"),
            &raw mut U_WIDGET_K2_ADD_FIELD_VALUE_CHANGED_DELEGATE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("IsVisible"),
            &raw mut U_WIDGET_IS_VISIBLE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("IsRendered"),
            &raw mut U_WIDGET_IS_RENDERED,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("IsInViewport"),
            &raw mut U_WIDGET_IS_IN_VIEWPORT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("IsHovered"),
            &raw mut U_WIDGET_IS_HOVERED,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("InvalidateLayoutAndVolatility"),
            &raw mut U_WIDGET_INVALIDATE_LAYOUT_AND_VOLATILITY,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("HasUserFocusedDescendants"),
            &raw mut U_WIDGET_HAS_USER_FOCUSED_DESCENDANTS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("HasUserFocus"),
            &raw mut U_WIDGET_HAS_USER_FOCUS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("HasMouseCaptureByUser"),
            &raw mut U_WIDGET_HAS_MOUSE_CAPTURE_BY_USER,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("HasMouseCapture"),
            &raw mut U_WIDGET_HAS_MOUSE_CAPTURE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("HasKeyboardFocus"),
            &raw mut U_WIDGET_HAS_KEYBOARD_FOCUS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("HasFocusedDescendants"),
            &raw mut U_WIDGET_HAS_FOCUSED_DESCENDANTS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("HasAnyUserFocus"),
            &raw mut U_WIDGET_HAS_ANY_USER_FOCUS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetWidget__DelegateSignature"),
            &raw mut U_WIDGET_GET_WIDGET_DELEGATE_SIGNATURE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetVisibility"),
            &raw mut U_WIDGET_GET_VISIBILITY,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetTickSpaceGeometry"),
            &raw mut U_WIDGET_GET_TICK_SPACE_GEOMETRY,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetText__DelegateSignature"),
            &raw mut U_WIDGET_GET_TEXT_DELEGATE_SIGNATURE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetSlateVisibility__DelegateSignature"),
            &raw mut U_WIDGET_GET_SLATE_VISIBILITY_DELEGATE_SIGNATURE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetSlateColor__DelegateSignature"),
            &raw mut U_WIDGET_GET_SLATE_COLOR_DELEGATE_SIGNATURE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetSlateBrush__DelegateSignature"),
            &raw mut U_WIDGET_GET_SLATE_BRUSH_DELEGATE_SIGNATURE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetRenderTransformAngle"),
            &raw mut U_WIDGET_GET_RENDER_TRANSFORM_ANGLE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetRenderOpacity"),
            &raw mut U_WIDGET_GET_RENDER_OPACITY,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetParent"),
            &raw mut U_WIDGET_GET_PARENT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetPaintSpaceGeometry"),
            &raw mut U_WIDGET_GET_PAINT_SPACE_GEOMETRY,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetOwningPlayer"),
            &raw mut U_WIDGET_GET_OWNING_PLAYER,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetOwningLocalPlayer"),
            &raw mut U_WIDGET_GET_OWNING_LOCAL_PLAYER,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetMouseCursor__DelegateSignature"),
            &raw mut U_WIDGET_GET_MOUSE_CURSOR_DELEGATE_SIGNATURE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetLinearColor__DelegateSignature"),
            &raw mut U_WIDGET_GET_LINEAR_COLOR_DELEGATE_SIGNATURE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetIsEnabled"),
            &raw mut U_WIDGET_GET_IS_ENABLED,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetInt32__DelegateSignature"),
            &raw mut U_WIDGET_GET_INT32_DELEGATE_SIGNATURE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetGameInstance"),
            &raw mut U_WIDGET_GET_GAME_INSTANCE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetFloat__DelegateSignature"),
            &raw mut U_WIDGET_GET_FLOAT_DELEGATE_SIGNATURE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetDesiredSize"),
            &raw mut U_WIDGET_GET_DESIRED_SIZE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetClipping"),
            &raw mut U_WIDGET_GET_CLIPPING,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetCheckBoxState__DelegateSignature"),
            &raw mut U_WIDGET_GET_CHECK_BOX_STATE_DELEGATE_SIGNATURE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetCachedGeometry"),
            &raw mut U_WIDGET_GET_CACHED_GEOMETRY,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetBool__DelegateSignature"),
            &raw mut U_WIDGET_GET_BOOL_DELEGATE_SIGNATURE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetAccessibleText"),
            &raw mut U_WIDGET_GET_ACCESSIBLE_TEXT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetAccessibleSummaryText"),
            &raw mut U_WIDGET_GET_ACCESSIBLE_SUMMARY_TEXT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GenerateWidgetForString__DelegateSignature"),
            &raw mut U_WIDGET_GENERATE_WIDGET_FOR_STRING_DELEGATE_SIGNATURE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GenerateWidgetForObject__DelegateSignature"),
            &raw mut U_WIDGET_GENERATE_WIDGET_FOR_OBJECT_DELEGATE_SIGNATURE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ForceVolatile"),
            &raw mut U_WIDGET_FORCE_VOLATILE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ForceLayoutPrepass"),
            &raw mut U_WIDGET_FORCE_LAYOUT_PREPASS,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UUserWidget::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("UnregisterInputComponent"),
            &raw mut U_USER_WIDGET_UNREGISTER_INPUT_COMPONENT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("UnbindFromAnimationStarted"),
            &raw mut U_USER_WIDGET_UNBIND_FROM_ANIMATION_STARTED,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("UnbindFromAnimationFinished"),
            &raw mut U_USER_WIDGET_UNBIND_FROM_ANIMATION_FINISHED,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("UnbindAllFromAnimationStarted"),
            &raw mut U_USER_WIDGET_UNBIND_ALL_FROM_ANIMATION_STARTED,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("UnbindAllFromAnimationFinished"),
            &raw mut U_USER_WIDGET_UNBIND_ALL_FROM_ANIMATION_FINISHED,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("Tick"),
            &raw mut U_USER_WIDGET_TICK,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("StopListeningForInputAction"),
            &raw mut U_USER_WIDGET_STOP_LISTENING_FOR_INPUT_ACTION,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("StopListeningForAllInputActions"),
            &raw mut U_USER_WIDGET_STOP_LISTENING_FOR_ALL_INPUT_ACTIONS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("StopAnimationsAndLatentActions"),
            &raw mut U_USER_WIDGET_STOP_ANIMATIONS_AND_LATENT_ACTIONS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("StopAnimation"),
            &raw mut U_USER_WIDGET_STOP_ANIMATION,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("StopAllAnimations"),
            &raw mut U_USER_WIDGET_STOP_ALL_ANIMATIONS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetPositionInViewport"),
            &raw mut U_USER_WIDGET_SET_POSITION_IN_VIEWPORT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetPlaybackSpeed"),
            &raw mut U_USER_WIDGET_SET_PLAYBACK_SPEED,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetPadding"),
            &raw mut U_USER_WIDGET_SET_PADDING,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetOwningPlayer"),
            &raw mut U_USER_WIDGET_SET_OWNING_PLAYER,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetNumLoopsToPlay"),
            &raw mut U_USER_WIDGET_SET_NUM_LOOPS_TO_PLAY,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetInputActionPriority"),
            &raw mut U_USER_WIDGET_SET_INPUT_ACTION_PRIORITY,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetInputActionBlocking"),
            &raw mut U_USER_WIDGET_SET_INPUT_ACTION_BLOCKING,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetForegroundColor"),
            &raw mut U_USER_WIDGET_SET_FOREGROUND_COLOR,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetDesiredSizeInViewport"),
            &raw mut U_USER_WIDGET_SET_DESIRED_SIZE_IN_VIEWPORT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetDesiredFocusWidget"),
            &raw mut U_USER_WIDGET_SET_DESIRED_FOCUS_WIDGET,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetColorAndOpacity"),
            &raw mut U_USER_WIDGET_SET_COLOR_AND_OPACITY,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetAnimationCurrentTime"),
            &raw mut U_USER_WIDGET_SET_ANIMATION_CURRENT_TIME,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetAnchorsInViewport"),
            &raw mut U_USER_WIDGET_SET_ANCHORS_IN_VIEWPORT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetAlignmentInViewport"),
            &raw mut U_USER_WIDGET_SET_ALIGNMENT_IN_VIEWPORT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ReverseAnimation"),
            &raw mut U_USER_WIDGET_REVERSE_ANIMATION,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RemoveFromViewport"),
            &raw mut U_USER_WIDGET_REMOVE_FROM_VIEWPORT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RemoveExtensions"),
            &raw mut U_USER_WIDGET_REMOVE_EXTENSIONS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RemoveExtension"),
            &raw mut U_USER_WIDGET_REMOVE_EXTENSION,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RegisterInputComponent"),
            &raw mut U_USER_WIDGET_REGISTER_INPUT_COMPONENT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("QueueStopAnimation"),
            &raw mut U_USER_WIDGET_QUEUE_STOP_ANIMATION,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("QueueStopAllAnimations"),
            &raw mut U_USER_WIDGET_QUEUE_STOP_ALL_ANIMATIONS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("QueuePlayAnimationTimeRange"),
            &raw mut U_USER_WIDGET_QUEUE_PLAY_ANIMATION_TIME_RANGE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("QueuePlayAnimationReverse"),
            &raw mut U_USER_WIDGET_QUEUE_PLAY_ANIMATION_REVERSE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("QueuePlayAnimationForward"),
            &raw mut U_USER_WIDGET_QUEUE_PLAY_ANIMATION_FORWARD,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("QueuePlayAnimation"),
            &raw mut U_USER_WIDGET_QUEUE_PLAY_ANIMATION,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("QueuePauseAnimation"),
            &raw mut U_USER_WIDGET_QUEUE_PAUSE_ANIMATION,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("PreConstruct"),
            &raw mut U_USER_WIDGET_PRE_CONSTRUCT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("PlaySound"),
            &raw mut U_USER_WIDGET_PLAY_SOUND,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("PlayAnimationTimeRange"),
            &raw mut U_USER_WIDGET_PLAY_ANIMATION_TIME_RANGE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("PlayAnimationReverse"),
            &raw mut U_USER_WIDGET_PLAY_ANIMATION_REVERSE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("PlayAnimationForward"),
            &raw mut U_USER_WIDGET_PLAY_ANIMATION_FORWARD,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("PlayAnimation"),
            &raw mut U_USER_WIDGET_PLAY_ANIMATION,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("PauseAnimation"),
            &raw mut U_USER_WIDGET_PAUSE_ANIMATION,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("OnTouchStarted"),
            &raw mut U_USER_WIDGET_ON_TOUCH_STARTED,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("OnTouchMoved"),
            &raw mut U_USER_WIDGET_ON_TOUCH_MOVED,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("OnTouchGesture"),
            &raw mut U_USER_WIDGET_ON_TOUCH_GESTURE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("OnTouchForceChanged"),
            &raw mut U_USER_WIDGET_ON_TOUCH_FORCE_CHANGED,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("OnTouchFirstMove"),
            &raw mut U_USER_WIDGET_ON_TOUCH_FIRST_MOVE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("OnTouchEnded"),
            &raw mut U_USER_WIDGET_ON_TOUCH_ENDED,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("OnRemovedFromFocusPath"),
            &raw mut U_USER_WIDGET_ON_REMOVED_FROM_FOCUS_PATH,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("OnPreviewMouseButtonDown"),
            &raw mut U_USER_WIDGET_ON_PREVIEW_MOUSE_BUTTON_DOWN,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("OnPreviewKeyDown"),
            &raw mut U_USER_WIDGET_ON_PREVIEW_KEY_DOWN,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("OnPaint"),
            &raw mut U_USER_WIDGET_ON_PAINT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("OnMouseWheel"),
            &raw mut U_USER_WIDGET_ON_MOUSE_WHEEL,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("OnMouseMove"),
            &raw mut U_USER_WIDGET_ON_MOUSE_MOVE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("OnMouseLeave"),
            &raw mut U_USER_WIDGET_ON_MOUSE_LEAVE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("OnMouseEnter"),
            &raw mut U_USER_WIDGET_ON_MOUSE_ENTER,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("OnMouseCaptureLost"),
            &raw mut U_USER_WIDGET_ON_MOUSE_CAPTURE_LOST,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("OnMouseButtonUp"),
            &raw mut U_USER_WIDGET_ON_MOUSE_BUTTON_UP,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("OnMouseButtonDown"),
            &raw mut U_USER_WIDGET_ON_MOUSE_BUTTON_DOWN,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("OnMouseButtonDoubleClick"),
            &raw mut U_USER_WIDGET_ON_MOUSE_BUTTON_DOUBLE_CLICK,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("OnMotionDetected"),
            &raw mut U_USER_WIDGET_ON_MOTION_DETECTED,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("OnKeyUp"),
            &raw mut U_USER_WIDGET_ON_KEY_UP,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("OnKeyDown"),
            &raw mut U_USER_WIDGET_ON_KEY_DOWN,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("OnKeyChar"),
            &raw mut U_USER_WIDGET_ON_KEY_CHAR,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("OnInitialized"),
            &raw mut U_USER_WIDGET_ON_INITIALIZED,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("OnFocusReceived"),
            &raw mut U_USER_WIDGET_ON_FOCUS_RECEIVED,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("OnFocusLost"),
            &raw mut U_USER_WIDGET_ON_FOCUS_LOST,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("OnDrop"),
            &raw mut U_USER_WIDGET_ON_DROP,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("OnDragOver"),
            &raw mut U_USER_WIDGET_ON_DRAG_OVER,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("OnDragLeave"),
            &raw mut U_USER_WIDGET_ON_DRAG_LEAVE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("OnDragEnter"),
            &raw mut U_USER_WIDGET_ON_DRAG_ENTER,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("OnDragDetected"),
            &raw mut U_USER_WIDGET_ON_DRAG_DETECTED,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("OnDragCancelled"),
            &raw mut U_USER_WIDGET_ON_DRAG_CANCELLED,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("OnAnimationStarted"),
            &raw mut U_USER_WIDGET_ON_ANIMATION_STARTED,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("OnAnimationFinished"),
            &raw mut U_USER_WIDGET_ON_ANIMATION_FINISHED,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("OnAnalogValueChanged"),
            &raw mut U_USER_WIDGET_ON_ANALOG_VALUE_CHANGED,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("OnAddedToFocusPath"),
            &raw mut U_USER_WIDGET_ON_ADDED_TO_FOCUS_PATH,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ListenForInputAction"),
            &raw mut U_USER_WIDGET_LISTEN_FOR_INPUT_ACTION,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("IsPlayingAnimation"),
            &raw mut U_USER_WIDGET_IS_PLAYING_ANIMATION,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("IsListeningForInputAction"),
            &raw mut U_USER_WIDGET_IS_LISTENING_FOR_INPUT_ACTION,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("IsInteractable"),
            &raw mut U_USER_WIDGET_IS_INTERACTABLE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("IsAnyAnimationPlaying"),
            &raw mut U_USER_WIDGET_IS_ANY_ANIMATION_PLAYING,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("IsAnimationPlayingForward"),
            &raw mut U_USER_WIDGET_IS_ANIMATION_PLAYING_FORWARD,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("IsAnimationPlaying"),
            &raw mut U_USER_WIDGET_IS_ANIMATION_PLAYING,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetOwningPlayerPawn"),
            &raw mut U_USER_WIDGET_GET_OWNING_PLAYER_PAWN,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetOwningPlayerCameraManager"),
            &raw mut U_USER_WIDGET_GET_OWNING_PLAYER_CAMERA_MANAGER,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetIsVisible"),
            &raw mut U_USER_WIDGET_GET_IS_VISIBLE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetExtensions"),
            &raw mut U_USER_WIDGET_GET_EXTENSIONS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetExtension"),
            &raw mut U_USER_WIDGET_GET_EXTENSION,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetAnimationCurrentTime"),
            &raw mut U_USER_WIDGET_GET_ANIMATION_CURRENT_TIME,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetAnchorsInViewport"),
            &raw mut U_USER_WIDGET_GET_ANCHORS_IN_VIEWPORT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetAlignmentInViewport"),
            &raw mut U_USER_WIDGET_GET_ALIGNMENT_IN_VIEWPORT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("FlushAnimations"),
            &raw mut U_USER_WIDGET_FLUSH_ANIMATIONS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("Destruct"),
            &raw mut U_USER_WIDGET_DESTRUCT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("Construct"),
            &raw mut U_USER_WIDGET_CONSTRUCT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CancelLatentActions"),
            &raw mut U_USER_WIDGET_CANCEL_LATENT_ACTIONS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("BindToAnimationStarted"),
            &raw mut U_USER_WIDGET_BIND_TO_ANIMATION_STARTED,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("BindToAnimationFinished"),
            &raw mut U_USER_WIDGET_BIND_TO_ANIMATION_FINISHED,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("BindToAnimationEvent"),
            &raw mut U_USER_WIDGET_BIND_TO_ANIMATION_EVENT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AddToViewport"),
            &raw mut U_USER_WIDGET_ADD_TO_VIEWPORT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AddToPlayerScreen"),
            &raw mut U_USER_WIDGET_ADD_TO_PLAYER_SCREEN,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AddExtension"),
            &raw mut U_USER_WIDGET_ADD_EXTENSION,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UWidgetComponent::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetWindowVisibility"),
            &raw mut U_WIDGET_COMPONENT_SET_WINDOW_VISIBILITY,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetWindowFocusable"),
            &raw mut U_WIDGET_COMPONENT_SET_WINDOW_FOCUSABLE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetWidgetSpace"),
            &raw mut U_WIDGET_COMPONENT_SET_WIDGET_SPACE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetWidget"),
            &raw mut U_WIDGET_COMPONENT_SET_WIDGET,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetTwoSided"),
            &raw mut U_WIDGET_COMPONENT_SET_TWO_SIDED,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetTintColorAndOpacity"),
            &raw mut U_WIDGET_COMPONENT_SET_TINT_COLOR_AND_OPACITY,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetTickWhenOffscreen"),
            &raw mut U_WIDGET_COMPONENT_SET_TICK_WHEN_OFFSCREEN,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetTickMode"),
            &raw mut U_WIDGET_COMPONENT_SET_TICK_MODE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetRedrawTime"),
            &raw mut U_WIDGET_COMPONENT_SET_REDRAW_TIME,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetPivot"),
            &raw mut U_WIDGET_COMPONENT_SET_PIVOT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetOwnerPlayer"),
            &raw mut U_WIDGET_COMPONENT_SET_OWNER_PLAYER,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetManuallyRedraw"),
            &raw mut U_WIDGET_COMPONENT_SET_MANUALLY_REDRAW,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetGeometryMode"),
            &raw mut U_WIDGET_COMPONENT_SET_GEOMETRY_MODE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetDrawSize"),
            &raw mut U_WIDGET_COMPONENT_SET_DRAW_SIZE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetDrawAtDesiredSize"),
            &raw mut U_WIDGET_COMPONENT_SET_DRAW_AT_DESIRED_SIZE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetCylinderArcAngle"),
            &raw mut U_WIDGET_COMPONENT_SET_CYLINDER_ARC_ANGLE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetBackgroundColor"),
            &raw mut U_WIDGET_COMPONENT_SET_BACKGROUND_COLOR,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RequestRenderUpdate"),
            &raw mut U_WIDGET_COMPONENT_REQUEST_RENDER_UPDATE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RequestRedraw"),
            &raw mut U_WIDGET_COMPONENT_REQUEST_REDRAW,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("IsWidgetVisible"),
            &raw mut U_WIDGET_COMPONENT_IS_WIDGET_VISIBLE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetWindowVisiblility"),
            &raw mut U_WIDGET_COMPONENT_GET_WINDOW_VISIBLILITY,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetWindowFocusable"),
            &raw mut U_WIDGET_COMPONENT_GET_WINDOW_FOCUSABLE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetWidgetSpace"),
            &raw mut U_WIDGET_COMPONENT_GET_WIDGET_SPACE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetWidget"),
            &raw mut U_WIDGET_COMPONENT_GET_WIDGET,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetUserWidgetObject"),
            &raw mut U_WIDGET_COMPONENT_GET_USER_WIDGET_OBJECT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetTwoSided"),
            &raw mut U_WIDGET_COMPONENT_GET_TWO_SIDED,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetTickWhenOffscreen"),
            &raw mut U_WIDGET_COMPONENT_GET_TICK_WHEN_OFFSCREEN,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetRenderTarget"),
            &raw mut U_WIDGET_COMPONENT_GET_RENDER_TARGET,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetRedrawTime"),
            &raw mut U_WIDGET_COMPONENT_GET_REDRAW_TIME,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetPivot"),
            &raw mut U_WIDGET_COMPONENT_GET_PIVOT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetOwnerPlayer"),
            &raw mut U_WIDGET_COMPONENT_GET_OWNER_PLAYER,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetMaterialInstance"),
            &raw mut U_WIDGET_COMPONENT_GET_MATERIAL_INSTANCE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetManuallyRedraw"),
            &raw mut U_WIDGET_COMPONENT_GET_MANUALLY_REDRAW,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetGeometryMode"),
            &raw mut U_WIDGET_COMPONENT_GET_GEOMETRY_MODE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetDrawSize"),
            &raw mut U_WIDGET_COMPONENT_GET_DRAW_SIZE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetDrawAtDesiredSize"),
            &raw mut U_WIDGET_COMPONENT_GET_DRAW_AT_DESIRED_SIZE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetCylinderArcAngle"),
            &raw mut U_WIDGET_COMPONENT_GET_CYLINDER_ARC_ANGLE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetCurrentDrawSize"),
            &raw mut U_WIDGET_COMPONENT_GET_CURRENT_DRAW_SIZE,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UPanelWidget::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RemoveChildAt"),
            &raw mut U_PANEL_WIDGET_REMOVE_CHILD_AT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RemoveChild"),
            &raw mut U_PANEL_WIDGET_REMOVE_CHILD,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("HasChild"),
            &raw mut U_PANEL_WIDGET_HAS_CHILD,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("HasAnyChildren"),
            &raw mut U_PANEL_WIDGET_HAS_ANY_CHILDREN,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetChildrenCount"),
            &raw mut U_PANEL_WIDGET_GET_CHILDREN_COUNT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetChildIndex"),
            &raw mut U_PANEL_WIDGET_GET_CHILD_INDEX,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetChildAt"),
            &raw mut U_PANEL_WIDGET_GET_CHILD_AT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetAllChildren"),
            &raw mut U_PANEL_WIDGET_GET_ALL_CHILDREN,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ClearChildren"),
            &raw mut U_PANEL_WIDGET_CLEAR_CHILDREN,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AddChild"),
            &raw mut U_PANEL_WIDGET_ADD_CHILD,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UContentWidget::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetContent"),
            &raw mut U_CONTENT_WIDGET_SET_CONTENT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetContentSlot"),
            &raw mut U_CONTENT_WIDGET_GET_CONTENT_SLOT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetContent"),
            &raw mut U_CONTENT_WIDGET_GET_CONTENT,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UButton::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetTouchMethod"),
            &raw mut U_BUTTON_SET_TOUCH_METHOD,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetStyle"),
            &raw mut U_BUTTON_SET_STYLE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetPressMethod"),
            &raw mut U_BUTTON_SET_PRESS_METHOD,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetColorAndOpacity"),
            &raw mut U_BUTTON_SET_COLOR_AND_OPACITY,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetClickMethod"),
            &raw mut U_BUTTON_SET_CLICK_METHOD,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetBackgroundColor"),
            &raw mut U_BUTTON_SET_BACKGROUND_COLOR,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetAllowDragDrop"),
            &raw mut U_BUTTON_SET_ALLOW_DRAG_DROP,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("IsPressed"),
            &raw mut U_BUTTON_IS_PRESSED,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UCheckBox::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetTouchMethod"),
            &raw mut U_CHECK_BOX_SET_TOUCH_METHOD,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetPressMethod"),
            &raw mut U_CHECK_BOX_SET_PRESS_METHOD,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetIsChecked"),
            &raw mut U_CHECK_BOX_SET_IS_CHECKED,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetClickMethod"),
            &raw mut U_CHECK_BOX_SET_CLICK_METHOD,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetCheckedState"),
            &raw mut U_CHECK_BOX_SET_CHECKED_STATE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("IsPressed"),
            &raw mut U_CHECK_BOX_IS_PRESSED,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("IsChecked"),
            &raw mut U_CHECK_BOX_IS_CHECKED,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetCheckedState"),
            &raw mut U_CHECK_BOX_GET_CHECKED_STATE,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UCircularThrobber::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetRadius"),
            &raw mut U_CIRCULAR_THROBBER_SET_RADIUS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetPeriod"),
            &raw mut U_CIRCULAR_THROBBER_SET_PERIOD,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetNumberOfPieces"),
            &raw mut U_CIRCULAR_THROBBER_SET_NUMBER_OF_PIECES,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UComboBoxKey::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetSelectedOption"),
            &raw mut U_COMBO_BOX_KEY_SET_SELECTED_OPTION,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RemoveOption"),
            &raw mut U_COMBO_BOX_KEY_REMOVE_OPTION,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("OnSelectionChangedEvent__DelegateSignature"),
            &raw mut U_COMBO_BOX_KEY_ON_SELECTION_CHANGED_EVENT_DELEGATE_SIGNATURE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("OnOpeningEvent__DelegateSignature"),
            &raw mut U_COMBO_BOX_KEY_ON_OPENING_EVENT_DELEGATE_SIGNATURE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("IsOpen"),
            &raw mut U_COMBO_BOX_KEY_IS_OPEN,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetSelectedOption"),
            &raw mut U_COMBO_BOX_KEY_GET_SELECTED_OPTION,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GenerateWidgetEvent__DelegateSignature"),
            &raw mut U_COMBO_BOX_KEY_GENERATE_WIDGET_EVENT_DELEGATE_SIGNATURE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ClearSelection"),
            &raw mut U_COMBO_BOX_KEY_CLEAR_SELECTION,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ClearOptions"),
            &raw mut U_COMBO_BOX_KEY_CLEAR_OPTIONS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AddOption"),
            &raw mut U_COMBO_BOX_KEY_ADD_OPTION,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UComboBoxString::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetSelectedOption"),
            &raw mut U_COMBO_BOX_STRING_SET_SELECTED_OPTION,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetSelectedIndex"),
            &raw mut U_COMBO_BOX_STRING_SET_SELECTED_INDEX,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RemoveOption"),
            &raw mut U_COMBO_BOX_STRING_REMOVE_OPTION,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RefreshOptions"),
            &raw mut U_COMBO_BOX_STRING_REFRESH_OPTIONS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("OnSelectionChangedEvent__DelegateSignature"),
            &raw mut U_COMBO_BOX_STRING_ON_SELECTION_CHANGED_EVENT_DELEGATE_SIGNATURE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("OnOpeningEvent__DelegateSignature"),
            &raw mut U_COMBO_BOX_STRING_ON_OPENING_EVENT_DELEGATE_SIGNATURE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("IsOpen"),
            &raw mut U_COMBO_BOX_STRING_IS_OPEN,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetSelectedOption"),
            &raw mut U_COMBO_BOX_STRING_GET_SELECTED_OPTION,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetSelectedIndex"),
            &raw mut U_COMBO_BOX_STRING_GET_SELECTED_INDEX,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetOptionCount"),
            &raw mut U_COMBO_BOX_STRING_GET_OPTION_COUNT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetOptionAtIndex"),
            &raw mut U_COMBO_BOX_STRING_GET_OPTION_AT_INDEX,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("FindOptionIndex"),
            &raw mut U_COMBO_BOX_STRING_FIND_OPTION_INDEX,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ClearSelection"),
            &raw mut U_COMBO_BOX_STRING_CLEAR_SELECTION,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ClearOptions"),
            &raw mut U_COMBO_BOX_STRING_CLEAR_OPTIONS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AddOption"),
            &raw mut U_COMBO_BOX_STRING_ADD_OPTION,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UEditableText::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ToggleVirtualKeyboard"),
            &raw mut U_EDITABLE_TEXT_TOGGLE_VIRTUAL_KEYBOARD,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetTextOverflowPolicy"),
            &raw mut U_EDITABLE_TEXT_SET_TEXT_OVERFLOW_POLICY,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetText"),
            &raw mut U_EDITABLE_TEXT_SET_TEXT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetMinimumDesiredWidth"),
            &raw mut U_EDITABLE_TEXT_SET_MINIMUM_DESIRED_WIDTH,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetJustification"),
            &raw mut U_EDITABLE_TEXT_SET_JUSTIFICATION,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetIsReadOnly"),
            &raw mut U_EDITABLE_TEXT_SET_IS_READ_ONLY,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetIsPassword"),
            &raw mut U_EDITABLE_TEXT_SET_IS_PASSWORD,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetHintText"),
            &raw mut U_EDITABLE_TEXT_SET_HINT_TEXT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetFontOutlineMaterial"),
            &raw mut U_EDITABLE_TEXT_SET_FONT_OUTLINE_MATERIAL,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetFontMaterial"),
            &raw mut U_EDITABLE_TEXT_SET_FONT_MATERIAL,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetFont"),
            &raw mut U_EDITABLE_TEXT_SET_FONT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("OnEditableTextCommittedEvent__DelegateSignature"),
            &raw mut U_EDITABLE_TEXT_ON_EDITABLE_TEXT_COMMITTED_EVENT_DELEGATE_SIGNATURE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("OnEditableTextChangedEvent__DelegateSignature"),
            &raw mut U_EDITABLE_TEXT_ON_EDITABLE_TEXT_CHANGED_EVENT_DELEGATE_SIGNATURE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetText"),
            &raw mut U_EDITABLE_TEXT_GET_TEXT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetJustification"),
            &raw mut U_EDITABLE_TEXT_GET_JUSTIFICATION,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetHintText"),
            &raw mut U_EDITABLE_TEXT_GET_HINT_TEXT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetFont"),
            &raw mut U_EDITABLE_TEXT_GET_FONT,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UEditableTextBox::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetTextOverflowPolicy"),
            &raw mut U_EDITABLE_TEXT_BOX_SET_TEXT_OVERFLOW_POLICY,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetText"),
            &raw mut U_EDITABLE_TEXT_BOX_SET_TEXT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetJustification"),
            &raw mut U_EDITABLE_TEXT_BOX_SET_JUSTIFICATION,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetIsReadOnly"),
            &raw mut U_EDITABLE_TEXT_BOX_SET_IS_READ_ONLY,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetIsPassword"),
            &raw mut U_EDITABLE_TEXT_BOX_SET_IS_PASSWORD,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetHintText"),
            &raw mut U_EDITABLE_TEXT_BOX_SET_HINT_TEXT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetForegroundColor"),
            &raw mut U_EDITABLE_TEXT_BOX_SET_FOREGROUND_COLOR,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetError"),
            &raw mut U_EDITABLE_TEXT_BOX_SET_ERROR,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from(
                "OnEditableTextBoxCommittedEvent__DelegateSignature",
            ),
            &raw mut U_EDITABLE_TEXT_BOX_ON_EDITABLE_TEXT_BOX_COMMITTED_EVENT_DELEGATE_SIGNATURE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from(
                "OnEditableTextBoxChangedEvent__DelegateSignature",
            ),
            &raw mut U_EDITABLE_TEXT_BOX_ON_EDITABLE_TEXT_BOX_CHANGED_EVENT_DELEGATE_SIGNATURE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("HasError"),
            &raw mut U_EDITABLE_TEXT_BOX_HAS_ERROR,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetText"),
            &raw mut U_EDITABLE_TEXT_BOX_GET_TEXT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ClearError"),
            &raw mut U_EDITABLE_TEXT_BOX_CLEAR_ERROR,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UExpandableArea::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetIsExpanded_Animated"),
            &raw mut U_EXPANDABLE_AREA_SET_IS_EXPANDED_ANIMATED,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetIsExpanded"),
            &raw mut U_EXPANDABLE_AREA_SET_IS_EXPANDED,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetIsExpanded"),
            &raw mut U_EXPANDABLE_AREA_GET_IS_EXPANDED,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UInputKeySelector::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetTextBlockVisibility"),
            &raw mut U_INPUT_KEY_SELECTOR_SET_TEXT_BLOCK_VISIBILITY,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetSelectedKey"),
            &raw mut U_INPUT_KEY_SELECTOR_SET_SELECTED_KEY,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetNoKeySpecifiedText"),
            &raw mut U_INPUT_KEY_SELECTOR_SET_NO_KEY_SPECIFIED_TEXT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetKeySelectionText"),
            &raw mut U_INPUT_KEY_SELECTOR_SET_KEY_SELECTION_TEXT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetEscapeKeys"),
            &raw mut U_INPUT_KEY_SELECTOR_SET_ESCAPE_KEYS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetAllowModifierKeys"),
            &raw mut U_INPUT_KEY_SELECTOR_SET_ALLOW_MODIFIER_KEYS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetAllowGamepadKeys"),
            &raw mut U_INPUT_KEY_SELECTOR_SET_ALLOW_GAMEPAD_KEYS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("OnKeySelected__DelegateSignature"),
            &raw mut U_INPUT_KEY_SELECTOR_ON_KEY_SELECTED_DELEGATE_SIGNATURE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("OnIsSelectingKeyChanged__DelegateSignature"),
            &raw mut U_INPUT_KEY_SELECTOR_ON_IS_SELECTING_KEY_CHANGED_DELEGATE_SIGNATURE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetIsSelectingKey"),
            &raw mut U_INPUT_KEY_SELECTOR_GET_IS_SELECTING_KEY,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UListViewBase::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetWheelScrollMultiplier"),
            &raw mut U_LIST_VIEW_BASE_SET_WHEEL_SCROLL_MULTIPLIER,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetScrollOffset"),
            &raw mut U_LIST_VIEW_BASE_SET_SCROLL_OFFSET,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetScrollbarVisibility"),
            &raw mut U_LIST_VIEW_BASE_SET_SCROLLBAR_VISIBILITY,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetIsPointerScrollingEnabled"),
            &raw mut U_LIST_VIEW_BASE_SET_IS_POINTER_SCROLLING_ENABLED,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetIsGamepadScrollingEnabled"),
            &raw mut U_LIST_VIEW_BASE_SET_IS_GAMEPAD_SCROLLING_ENABLED,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetAllowOverScroll"),
            &raw mut U_LIST_VIEW_BASE_SET_ALLOW_OVER_SCROLL,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ScrollToTop"),
            &raw mut U_LIST_VIEW_BASE_SCROLL_TO_TOP,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ScrollToBottom"),
            &raw mut U_LIST_VIEW_BASE_SCROLL_TO_BOTTOM,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RequestRefresh"),
            &raw mut U_LIST_VIEW_BASE_REQUEST_REFRESH,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RegenerateAllEntries"),
            &raw mut U_LIST_VIEW_BASE_REGENERATE_ALL_ENTRIES,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetScrollOffset"),
            &raw mut U_LIST_VIEW_BASE_GET_SCROLL_OFFSET,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetIsDraggingListItem"),
            &raw mut U_LIST_VIEW_BASE_GET_IS_DRAGGING_LIST_ITEM,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetDisplayedEntryWidgets"),
            &raw mut U_LIST_VIEW_BASE_GET_DISPLAYED_ENTRY_WIDGETS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("EndInertialScrolling"),
            &raw mut U_LIST_VIEW_BASE_END_INERTIAL_SCROLLING,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CreateDragDropOperation"),
            &raw mut U_LIST_VIEW_BASE_CREATE_DRAG_DROP_OPERATION,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CancelListViewDragDrop"),
            &raw mut U_LIST_VIEW_BASE_CANCEL_LIST_VIEW_DRAG_DROP,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UListView::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetSelectionMode"),
            &raw mut U_LIST_VIEW_SET_SELECTION_MODE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetSelectedIndex"),
            &raw mut U_LIST_VIEW_SET_SELECTED_INDEX,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetScrollIntoViewAlignment"),
            &raw mut U_LIST_VIEW_SET_SCROLL_INTO_VIEW_ALIGNMENT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetScrollBarPadding"),
            &raw mut U_LIST_VIEW_SET_SCROLL_BAR_PADDING,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ScrollIndexIntoView"),
            &raw mut U_LIST_VIEW_SCROLL_INDEX_INTO_VIEW,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RemoveItem"),
            &raw mut U_LIST_VIEW_REMOVE_ITEM,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("OnListItemOuterEndPlayed"),
            &raw mut U_LIST_VIEW_ON_LIST_ITEM_OUTER_END_PLAYED,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("OnListItemEndPlayed"),
            &raw mut U_LIST_VIEW_ON_LIST_ITEM_END_PLAYED,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("NavigateToIndex"),
            &raw mut U_LIST_VIEW_NAVIGATE_TO_INDEX,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("IsRefreshPending"),
            &raw mut U_LIST_VIEW_IS_REFRESH_PENDING,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetVerticalEntrySpacing"),
            &raw mut U_LIST_VIEW_GET_VERTICAL_ENTRY_SPACING,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetScrollBarPadding"),
            &raw mut U_LIST_VIEW_GET_SCROLL_BAR_PADDING,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetNumItems"),
            &raw mut U_LIST_VIEW_GET_NUM_ITEMS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetListItems"),
            &raw mut U_LIST_VIEW_GET_LIST_ITEMS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetItemAt"),
            &raw mut U_LIST_VIEW_GET_ITEM_AT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetIndexForItem"),
            &raw mut U_LIST_VIEW_GET_INDEX_FOR_ITEM,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetHorizontalEntrySpacing"),
            &raw mut U_LIST_VIEW_GET_HORIZONTAL_ENTRY_SPACING,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ClearListItems"),
            &raw mut U_LIST_VIEW_CLEAR_LIST_ITEMS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("BP_SetSelectedItem"),
            &raw mut U_LIST_VIEW_BP_SET_SELECTED_ITEM,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("BP_SetListItems"),
            &raw mut U_LIST_VIEW_BP_SET_LIST_ITEMS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("BP_SetItemSelection"),
            &raw mut U_LIST_VIEW_BP_SET_ITEM_SELECTION,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("BP_ScrollItemIntoView"),
            &raw mut U_LIST_VIEW_BP_SCROLL_ITEM_INTO_VIEW,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("BP_NavigateToItem"),
            &raw mut U_LIST_VIEW_BP_NAVIGATE_TO_ITEM,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("BP_IsItemVisible"),
            &raw mut U_LIST_VIEW_BP_IS_ITEM_VISIBLE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("BP_GetSelectedItems"),
            &raw mut U_LIST_VIEW_BP_GET_SELECTED_ITEMS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("BP_GetSelectedItem"),
            &raw mut U_LIST_VIEW_BP_GET_SELECTED_ITEM,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("BP_GetNumItemsSelected"),
            &raw mut U_LIST_VIEW_BP_GET_NUM_ITEMS_SELECTED,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("BP_ClearSelection"),
            &raw mut U_LIST_VIEW_BP_CLEAR_SELECTION,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("BP_CancelScrollIntoView"),
            &raw mut U_LIST_VIEW_BP_CANCEL_SCROLL_INTO_VIEW,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AddItem"),
            &raw mut U_LIST_VIEW_ADD_ITEM,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UTextLayoutWidget::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetJustification"),
            &raw mut U_TEXT_LAYOUT_WIDGET_SET_JUSTIFICATION,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UMultiLineEditableText::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetWidgetStyle"),
            &raw mut U_MULTI_LINE_EDITABLE_TEXT_SET_WIDGET_STYLE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetText"),
            &raw mut U_MULTI_LINE_EDITABLE_TEXT_SET_TEXT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetIsReadOnly"),
            &raw mut U_MULTI_LINE_EDITABLE_TEXT_SET_IS_READ_ONLY,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetHintText"),
            &raw mut U_MULTI_LINE_EDITABLE_TEXT_SET_HINT_TEXT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetFontOutlineMaterial"),
            &raw mut U_MULTI_LINE_EDITABLE_TEXT_SET_FONT_OUTLINE_MATERIAL,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetFontMaterial"),
            &raw mut U_MULTI_LINE_EDITABLE_TEXT_SET_FONT_MATERIAL,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetFont"),
            &raw mut U_MULTI_LINE_EDITABLE_TEXT_SET_FONT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from(
                "OnMultiLineEditableTextCommittedEvent__DelegateSignature",
            ),
            &raw mut U_MULTI_LINE_EDITABLE_TEXT_ON_MULTI_LINE_EDITABLE_TEXT_COMMITTED_EVENT_DELEGATE_SIGNATURE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from(
                "OnMultiLineEditableTextChangedEvent__DelegateSignature",
            ),
            &raw mut U_MULTI_LINE_EDITABLE_TEXT_ON_MULTI_LINE_EDITABLE_TEXT_CHANGED_EVENT_DELEGATE_SIGNATURE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetText"),
            &raw mut U_MULTI_LINE_EDITABLE_TEXT_GET_TEXT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetHintText"),
            &raw mut U_MULTI_LINE_EDITABLE_TEXT_GET_HINT_TEXT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetFont"),
            &raw mut U_MULTI_LINE_EDITABLE_TEXT_GET_FONT,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UMultiLineEditableTextBox::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetTextStyle"),
            &raw mut U_MULTI_LINE_EDITABLE_TEXT_BOX_SET_TEXT_STYLE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetText"),
            &raw mut U_MULTI_LINE_EDITABLE_TEXT_BOX_SET_TEXT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetIsReadOnly"),
            &raw mut U_MULTI_LINE_EDITABLE_TEXT_BOX_SET_IS_READ_ONLY,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetHintText"),
            &raw mut U_MULTI_LINE_EDITABLE_TEXT_BOX_SET_HINT_TEXT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetForegroundColor"),
            &raw mut U_MULTI_LINE_EDITABLE_TEXT_BOX_SET_FOREGROUND_COLOR,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetError"),
            &raw mut U_MULTI_LINE_EDITABLE_TEXT_BOX_SET_ERROR,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from(
                "OnMultiLineEditableTextBoxCommittedEvent__DelegateSignature",
            ),
            &raw mut U_MULTI_LINE_EDITABLE_TEXT_BOX_ON_MULTI_LINE_EDITABLE_TEXT_BOX_COMMITTED_EVENT_DELEGATE_SIGNATURE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from(
                "OnMultiLineEditableTextBoxChangedEvent__DelegateSignature",
            ),
            &raw mut U_MULTI_LINE_EDITABLE_TEXT_BOX_ON_MULTI_LINE_EDITABLE_TEXT_BOX_CHANGED_EVENT_DELEGATE_SIGNATURE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetText"),
            &raw mut U_MULTI_LINE_EDITABLE_TEXT_BOX_GET_TEXT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetHintText"),
            &raw mut U_MULTI_LINE_EDITABLE_TEXT_BOX_GET_HINT_TEXT,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UProgressBar::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetPercent"),
            &raw mut U_PROGRESS_BAR_SET_PERCENT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetIsMarquee"),
            &raw mut U_PROGRESS_BAR_SET_IS_MARQUEE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetFillColorAndOpacity"),
            &raw mut U_PROGRESS_BAR_SET_FILL_COLOR_AND_OPACITY,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UScrollBar::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetState"),
            &raw mut U_SCROLL_BAR_SET_STATE,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UScrollBox::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetWheelScrollMultiplier"),
            &raw mut U_SCROLL_BOX_SET_WHEEL_SCROLL_MULTIPLIER,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetScrollWhenFocusChanges"),
            &raw mut U_SCROLL_BOX_SET_SCROLL_WHEN_FOCUS_CHANGES,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetScrollOffset"),
            &raw mut U_SCROLL_BOX_SET_SCROLL_OFFSET,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetScrollBarVisibility"),
            &raw mut U_SCROLL_BOX_SET_SCROLL_BAR_VISIBILITY,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetScrollbarThickness"),
            &raw mut U_SCROLL_BOX_SET_SCROLLBAR_THICKNESS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetScrollbarPadding"),
            &raw mut U_SCROLL_BOX_SET_SCROLLBAR_PADDING,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetScrollAnimationInterpolationSpeed"),
            &raw mut U_SCROLL_BOX_SET_SCROLL_ANIMATION_INTERPOLATION_SPEED,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetOrientation"),
            &raw mut U_SCROLL_BOX_SET_ORIENTATION,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetNavigationDestination"),
            &raw mut U_SCROLL_BOX_SET_NAVIGATION_DESTINATION,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetIsTouchScrollingEnabled"),
            &raw mut U_SCROLL_BOX_SET_IS_TOUCH_SCROLLING_ENABLED,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetIsFocusable"),
            &raw mut U_SCROLL_BOX_SET_IS_FOCUSABLE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetConsumePointerInput"),
            &raw mut U_SCROLL_BOX_SET_CONSUME_POINTER_INPUT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetConsumeMouseWheel"),
            &raw mut U_SCROLL_BOX_SET_CONSUME_MOUSE_WHEEL,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetAnimateWheelScrolling"),
            &raw mut U_SCROLL_BOX_SET_ANIMATE_WHEEL_SCROLLING,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetAnalogMouseWheelKey"),
            &raw mut U_SCROLL_BOX_SET_ANALOG_MOUSE_WHEEL_KEY,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetAlwaysShowScrollbar"),
            &raw mut U_SCROLL_BOX_SET_ALWAYS_SHOW_SCROLLBAR,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetAllowOverscroll"),
            &raw mut U_SCROLL_BOX_SET_ALLOW_OVERSCROLL,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ScrollWidgetIntoView"),
            &raw mut U_SCROLL_BOX_SCROLL_WIDGET_INTO_VIEW,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ScrollToStart"),
            &raw mut U_SCROLL_BOX_SCROLL_TO_START,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ScrollToEnd"),
            &raw mut U_SCROLL_BOX_SCROLL_TO_END,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetViewOffsetFraction"),
            &raw mut U_SCROLL_BOX_GET_VIEW_OFFSET_FRACTION,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetViewFraction"),
            &raw mut U_SCROLL_BOX_GET_VIEW_FRACTION,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetScrollOffsetOfEnd"),
            &raw mut U_SCROLL_BOX_GET_SCROLL_OFFSET_OF_END,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetScrollOffset"),
            &raw mut U_SCROLL_BOX_GET_SCROLL_OFFSET,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetOverscrollPercentage"),
            &raw mut U_SCROLL_BOX_GET_OVERSCROLL_PERCENTAGE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetOverscrollOffset"),
            &raw mut U_SCROLL_BOX_GET_OVERSCROLL_OFFSET,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetIsScrolling"),
            &raw mut U_SCROLL_BOX_GET_IS_SCROLLING,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetIsFocusable"),
            &raw mut U_SCROLL_BOX_GET_IS_FOCUSABLE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetConsumePointerInput"),
            &raw mut U_SCROLL_BOX_GET_CONSUME_POINTER_INPUT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetAnalogMouseWheelKey"),
            &raw mut U_SCROLL_BOX_GET_ANALOG_MOUSE_WHEEL_KEY,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("EndInertialScrolling"),
            &raw mut U_SCROLL_BOX_END_INERTIAL_SCROLLING,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = USlider::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetValue"),
            &raw mut U_SLIDER_SET_VALUE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetStepSize"),
            &raw mut U_SLIDER_SET_STEP_SIZE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetSliderHandleColor"),
            &raw mut U_SLIDER_SET_SLIDER_HANDLE_COLOR,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetSliderBarColor"),
            &raw mut U_SLIDER_SET_SLIDER_BAR_COLOR,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetMinValue"),
            &raw mut U_SLIDER_SET_MIN_VALUE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetMaxValue"),
            &raw mut U_SLIDER_SET_MAX_VALUE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetLocked"),
            &raw mut U_SLIDER_SET_LOCKED,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetIndentHandle"),
            &raw mut U_SLIDER_SET_INDENT_HANDLE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetValue"),
            &raw mut U_SLIDER_GET_VALUE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetNormalizedValue"),
            &raw mut U_SLIDER_GET_NORMALIZED_VALUE,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = USpinBox::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetValue"),
            &raw mut U_SPIN_BOX_SET_VALUE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetMinValue"),
            &raw mut U_SPIN_BOX_SET_MIN_VALUE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetMinSliderValue"),
            &raw mut U_SPIN_BOX_SET_MIN_SLIDER_VALUE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetMinFractionalDigits"),
            &raw mut U_SPIN_BOX_SET_MIN_FRACTIONAL_DIGITS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetMaxValue"),
            &raw mut U_SPIN_BOX_SET_MAX_VALUE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetMaxSliderValue"),
            &raw mut U_SPIN_BOX_SET_MAX_SLIDER_VALUE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetMaxFractionalDigits"),
            &raw mut U_SPIN_BOX_SET_MAX_FRACTIONAL_DIGITS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetForegroundColor"),
            &raw mut U_SPIN_BOX_SET_FOREGROUND_COLOR,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetDelta"),
            &raw mut U_SPIN_BOX_SET_DELTA,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetAlwaysUsesDeltaSnap"),
            &raw mut U_SPIN_BOX_SET_ALWAYS_USES_DELTA_SNAP,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("OnSpinBoxValueCommittedEvent__DelegateSignature"),
            &raw mut U_SPIN_BOX_ON_SPIN_BOX_VALUE_COMMITTED_EVENT_DELEGATE_SIGNATURE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("OnSpinBoxValueChangedEvent__DelegateSignature"),
            &raw mut U_SPIN_BOX_ON_SPIN_BOX_VALUE_CHANGED_EVENT_DELEGATE_SIGNATURE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("OnSpinBoxBeginSliderMovement__DelegateSignature"),
            &raw mut U_SPIN_BOX_ON_SPIN_BOX_BEGIN_SLIDER_MOVEMENT_DELEGATE_SIGNATURE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetValue"),
            &raw mut U_SPIN_BOX_GET_VALUE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetMinValue"),
            &raw mut U_SPIN_BOX_GET_MIN_VALUE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetMinSliderValue"),
            &raw mut U_SPIN_BOX_GET_MIN_SLIDER_VALUE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetMinFractionalDigits"),
            &raw mut U_SPIN_BOX_GET_MIN_FRACTIONAL_DIGITS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetMaxValue"),
            &raw mut U_SPIN_BOX_GET_MAX_VALUE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetMaxSliderValue"),
            &raw mut U_SPIN_BOX_GET_MAX_SLIDER_VALUE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetMaxFractionalDigits"),
            &raw mut U_SPIN_BOX_GET_MAX_FRACTIONAL_DIGITS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetDelta"),
            &raw mut U_SPIN_BOX_GET_DELTA,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetAlwaysUsesDeltaSnap"),
            &raw mut U_SPIN_BOX_GET_ALWAYS_USES_DELTA_SNAP,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ClearMinValue"),
            &raw mut U_SPIN_BOX_CLEAR_MIN_VALUE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ClearMinSliderValue"),
            &raw mut U_SPIN_BOX_CLEAR_MIN_SLIDER_VALUE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ClearMaxValue"),
            &raw mut U_SPIN_BOX_CLEAR_MAX_VALUE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ClearMaxSliderValue"),
            &raw mut U_SPIN_BOX_CLEAR_MAX_SLIDER_VALUE,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UThrobber::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetNumberOfPieces"),
            &raw mut U_THROBBER_SET_NUMBER_OF_PIECES,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetAnimateVertically"),
            &raw mut U_THROBBER_SET_ANIMATE_VERTICALLY,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetAnimateOpacity"),
            &raw mut U_THROBBER_SET_ANIMATE_OPACITY,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetAnimateHorizontally"),
            &raw mut U_THROBBER_SET_ANIMATE_HORIZONTALLY,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UTreeView::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetItemExpansion"),
            &raw mut U_TREE_VIEW_SET_ITEM_EXPANSION,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ExpandAll"),
            &raw mut U_TREE_VIEW_EXPAND_ALL,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CollapseAll"),
            &raw mut U_TREE_VIEW_COLLAPSE_ALL,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = USlateAccessibleWidgetData::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetText__DelegateSignature"),
            &raw mut U_SLATE_ACCESSIBLE_WIDGET_DATA_GET_TEXT_DELEGATE_SIGNATURE,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UUMGSequencePlayer::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetUserTag"),
            &raw mut UUMG_SEQUENCE_PLAYER_SET_USER_TAG,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetUserTag"),
            &raw mut UUMG_SEQUENCE_PLAYER_GET_USER_TAG,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UWidgetAnimation::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("UnbindFromAnimationStarted"),
            &raw mut U_WIDGET_ANIMATION_UNBIND_FROM_ANIMATION_STARTED,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("UnbindFromAnimationFinished"),
            &raw mut U_WIDGET_ANIMATION_UNBIND_FROM_ANIMATION_FINISHED,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("UnbindAllFromAnimationStarted"),
            &raw mut U_WIDGET_ANIMATION_UNBIND_ALL_FROM_ANIMATION_STARTED,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("UnbindAllFromAnimationFinished"),
            &raw mut U_WIDGET_ANIMATION_UNBIND_ALL_FROM_ANIMATION_FINISHED,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetStartTime"),
            &raw mut U_WIDGET_ANIMATION_GET_START_TIME,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetEndTime"),
            &raw mut U_WIDGET_ANIMATION_GET_END_TIME,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("BindToAnimationStarted"),
            &raw mut U_WIDGET_ANIMATION_BIND_TO_ANIMATION_STARTED,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("BindToAnimationFinished"),
            &raw mut U_WIDGET_ANIMATION_BIND_TO_ANIMATION_FINISHED,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UWidgetAnimationHandleFunctionLibrary::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetUserTag"),
            &raw mut U_WIDGET_ANIMATION_HANDLE_FUNCTION_LIBRARY_SET_USER_TAG,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetUserTag"),
            &raw mut U_WIDGET_ANIMATION_HANDLE_FUNCTION_LIBRARY_GET_USER_TAG,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UWidgetAnimationPlayCallbackProxy::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("NewPlayAnimationTimeRangeProxyObject"),
            &raw mut U_WIDGET_ANIMATION_PLAY_CALLBACK_PROXY_NEW_PLAY_ANIMATION_TIME_RANGE_PROXY_OBJECT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("NewPlayAnimationProxyObject"),
            &raw mut U_WIDGET_ANIMATION_PLAY_CALLBACK_PROXY_NEW_PLAY_ANIMATION_PROXY_OBJECT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CreatePlayAnimationTimeRangeProxyObject"),
            &raw mut U_WIDGET_ANIMATION_PLAY_CALLBACK_PROXY_CREATE_PLAY_ANIMATION_TIME_RANGE_PROXY_OBJECT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CreatePlayAnimationProxyObject"),
            &raw mut U_WIDGET_ANIMATION_PLAY_CALLBACK_PROXY_CREATE_PLAY_ANIMATION_PROXY_OBJECT,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UBoolBinding::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetValue"),
            &raw mut U_BOOL_BINDING_GET_VALUE,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UBrushBinding::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetValue"),
            &raw mut U_BRUSH_BINDING_GET_VALUE,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UCheckedStateBinding::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetValue"),
            &raw mut U_CHECKED_STATE_BINDING_GET_VALUE,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UColorBinding::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetSlateValue"),
            &raw mut U_COLOR_BINDING_GET_SLATE_VALUE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetLinearValue"),
            &raw mut U_COLOR_BINDING_GET_LINEAR_VALUE,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UFloatBinding::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetValue"),
            &raw mut U_FLOAT_BINDING_GET_VALUE,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UInt32Binding::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetValue"),
            &raw mut U_INT32_BINDING_GET_VALUE,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UMouseCursorBinding::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetValue"),
            &raw mut U_MOUSE_CURSOR_BINDING_GET_VALUE,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UTextBinding::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetTextValue"),
            &raw mut U_TEXT_BINDING_GET_TEXT_VALUE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetStringValue"),
            &raw mut U_TEXT_BINDING_GET_STRING_VALUE,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UVisibilityBinding::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetValue"),
            &raw mut U_VISIBILITY_BINDING_GET_VALUE,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UWidgetBinding::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetValue"),
            &raw mut U_WIDGET_BINDING_GET_VALUE,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UAsyncTaskDownloadImage::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("DownloadImage"),
            &raw mut U_ASYNC_TASK_DOWNLOAD_IMAGE_DOWNLOAD_IMAGE,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UGameViewportSubsystem::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetWidgetSlotPosition"),
            &raw mut U_GAME_VIEWPORT_SUBSYSTEM_SET_WIDGET_SLOT_POSITION,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetWidgetSlotDesiredSize"),
            &raw mut U_GAME_VIEWPORT_SUBSYSTEM_SET_WIDGET_SLOT_DESIRED_SIZE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetWidgetSlot"),
            &raw mut U_GAME_VIEWPORT_SUBSYSTEM_SET_WIDGET_SLOT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RemoveWidget"),
            &raw mut U_GAME_VIEWPORT_SUBSYSTEM_REMOVE_WIDGET,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("IsWidgetAdded"),
            &raw mut U_GAME_VIEWPORT_SUBSYSTEM_IS_WIDGET_ADDED,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetWidgetSlot"),
            &raw mut U_GAME_VIEWPORT_SUBSYSTEM_GET_WIDGET_SLOT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AddWidgetForPlayer"),
            &raw mut U_GAME_VIEWPORT_SUBSYSTEM_ADD_WIDGET_FOR_PLAYER,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AddWidget"),
            &raw mut U_GAME_VIEWPORT_SUBSYSTEM_ADD_WIDGET,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UUserListEntry::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("BP_OnUpdateEntryDropIndicator"),
            &raw mut U_USER_LIST_ENTRY_BP_ON_UPDATE_ENTRY_DROP_INDICATOR,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("BP_OnItemSelectionChanged"),
            &raw mut U_USER_LIST_ENTRY_BP_ON_ITEM_SELECTION_CHANGED,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("BP_OnItemExpansionChanged"),
            &raw mut U_USER_LIST_ENTRY_BP_ON_ITEM_EXPANSION_CHANGED,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("BP_OnEntryReleased"),
            &raw mut U_USER_LIST_ENTRY_BP_ON_ENTRY_RELEASED,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("BP_OnEntryDropped"),
            &raw mut U_USER_LIST_ENTRY_BP_ON_ENTRY_DROPPED,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("BP_OnEntryDragOverChanged"),
            &raw mut U_USER_LIST_ENTRY_BP_ON_ENTRY_DRAG_OVER_CHANGED,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("BP_OnEntryDragged"),
            &raw mut U_USER_LIST_ENTRY_BP_ON_ENTRY_DRAGGED,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("BP_OnEndEntryDropOperation"),
            &raw mut U_USER_LIST_ENTRY_BP_ON_END_ENTRY_DROP_OPERATION,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UUserListEntryLibrary::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("IsListItemSelected"),
            &raw mut U_USER_LIST_ENTRY_LIBRARY_IS_LIST_ITEM_SELECTED,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("IsListItemExpanded"),
            &raw mut U_USER_LIST_ENTRY_LIBRARY_IS_LIST_ITEM_EXPANDED,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetOwningListView"),
            &raw mut U_USER_LIST_ENTRY_LIBRARY_GET_OWNING_LIST_VIEW,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UUserObjectListEntry::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("OnListItemObjectSet"),
            &raw mut U_USER_OBJECT_LIST_ENTRY_ON_LIST_ITEM_OBJECT_SET,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UUserObjectListEntryLibrary::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("IsLastWidget"),
            &raw mut U_USER_OBJECT_LIST_ENTRY_LIBRARY_IS_LAST_WIDGET,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("IsFirstWidget"),
            &raw mut U_USER_OBJECT_LIST_ENTRY_LIBRARY_IS_FIRST_WIDGET,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetListItemObject"),
            &raw mut U_USER_OBJECT_LIST_ENTRY_LIBRARY_GET_LIST_ITEM_OBJECT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetListItemIndex"),
            &raw mut U_USER_OBJECT_LIST_ENTRY_LIBRARY_GET_LIST_ITEM_INDEX,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UBackgroundBlur::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetVerticalAlignment"),
            &raw mut U_BACKGROUND_BLUR_SET_VERTICAL_ALIGNMENT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetPadding"),
            &raw mut U_BACKGROUND_BLUR_SET_PADDING,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetLowQualityFallbackBrush"),
            &raw mut U_BACKGROUND_BLUR_SET_LOW_QUALITY_FALLBACK_BRUSH,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetHorizontalAlignment"),
            &raw mut U_BACKGROUND_BLUR_SET_HORIZONTAL_ALIGNMENT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetCornerRadius"),
            &raw mut U_BACKGROUND_BLUR_SET_CORNER_RADIUS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetBlurStrength"),
            &raw mut U_BACKGROUND_BLUR_SET_BLUR_STRENGTH,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetBlurRadius"),
            &raw mut U_BACKGROUND_BLUR_SET_BLUR_RADIUS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetApplyAlphaToBlur"),
            &raw mut U_BACKGROUND_BLUR_SET_APPLY_ALPHA_TO_BLUR,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UPanelSlot::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetContent"),
            &raw mut U_PANEL_SLOT_GET_CONTENT,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UBackgroundBlurSlot::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetVerticalAlignment"),
            &raw mut U_BACKGROUND_BLUR_SLOT_SET_VERTICAL_ALIGNMENT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetPadding"),
            &raw mut U_BACKGROUND_BLUR_SLOT_SET_PADDING,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetHorizontalAlignment"),
            &raw mut U_BACKGROUND_BLUR_SLOT_SET_HORIZONTAL_ALIGNMENT,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UBorder::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetVerticalAlignment"),
            &raw mut U_BORDER_SET_VERTICAL_ALIGNMENT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetShowEffectWhenDisabled"),
            &raw mut U_BORDER_SET_SHOW_EFFECT_WHEN_DISABLED,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetPadding"),
            &raw mut U_BORDER_SET_PADDING,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetHorizontalAlignment"),
            &raw mut U_BORDER_SET_HORIZONTAL_ALIGNMENT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetDesiredSizeScale"),
            &raw mut U_BORDER_SET_DESIRED_SIZE_SCALE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetContentColorAndOpacity"),
            &raw mut U_BORDER_SET_CONTENT_COLOR_AND_OPACITY,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetBrushFromTexture"),
            &raw mut U_BORDER_SET_BRUSH_FROM_TEXTURE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetBrushFromMaterial"),
            &raw mut U_BORDER_SET_BRUSH_FROM_MATERIAL,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetBrushFromAsset"),
            &raw mut U_BORDER_SET_BRUSH_FROM_ASSET,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetBrushColor"),
            &raw mut U_BORDER_SET_BRUSH_COLOR,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetBrush"),
            &raw mut U_BORDER_SET_BRUSH,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetDynamicMaterial"),
            &raw mut U_BORDER_GET_DYNAMIC_MATERIAL,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UBorderSlot::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetVerticalAlignment"),
            &raw mut U_BORDER_SLOT_SET_VERTICAL_ALIGNMENT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetPadding"),
            &raw mut U_BORDER_SLOT_SET_PADDING,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetHorizontalAlignment"),
            &raw mut U_BORDER_SLOT_SET_HORIZONTAL_ALIGNMENT,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UButtonSlot::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetVerticalAlignment"),
            &raw mut U_BUTTON_SLOT_SET_VERTICAL_ALIGNMENT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetPadding"),
            &raw mut U_BUTTON_SLOT_SET_PADDING,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetHorizontalAlignment"),
            &raw mut U_BUTTON_SLOT_SET_HORIZONTAL_ALIGNMENT,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UCanvasPanel::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AddChildToCanvas"),
            &raw mut U_CANVAS_PANEL_ADD_CHILD_TO_CANVAS,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UCanvasPanelSlot::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetZOrder"),
            &raw mut U_CANVAS_PANEL_SLOT_SET_Z_ORDER,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetSize"),
            &raw mut U_CANVAS_PANEL_SLOT_SET_SIZE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetPosition"),
            &raw mut U_CANVAS_PANEL_SLOT_SET_POSITION,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetOffsets"),
            &raw mut U_CANVAS_PANEL_SLOT_SET_OFFSETS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetMinimum"),
            &raw mut U_CANVAS_PANEL_SLOT_SET_MINIMUM,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetMaximum"),
            &raw mut U_CANVAS_PANEL_SLOT_SET_MAXIMUM,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetLayout"),
            &raw mut U_CANVAS_PANEL_SLOT_SET_LAYOUT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetAutoSize"),
            &raw mut U_CANVAS_PANEL_SLOT_SET_AUTO_SIZE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetAnchors"),
            &raw mut U_CANVAS_PANEL_SLOT_SET_ANCHORS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetAlignment"),
            &raw mut U_CANVAS_PANEL_SLOT_SET_ALIGNMENT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetZOrder"),
            &raw mut U_CANVAS_PANEL_SLOT_GET_Z_ORDER,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetSize"),
            &raw mut U_CANVAS_PANEL_SLOT_GET_SIZE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetPosition"),
            &raw mut U_CANVAS_PANEL_SLOT_GET_POSITION,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetOffsets"),
            &raw mut U_CANVAS_PANEL_SLOT_GET_OFFSETS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetLayout"),
            &raw mut U_CANVAS_PANEL_SLOT_GET_LAYOUT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetAutoSize"),
            &raw mut U_CANVAS_PANEL_SLOT_GET_AUTO_SIZE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetAnchors"),
            &raw mut U_CANVAS_PANEL_SLOT_GET_ANCHORS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetAlignment"),
            &raw mut U_CANVAS_PANEL_SLOT_GET_ALIGNMENT,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UDynamicEntryBoxBase::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetRadialSettings"),
            &raw mut U_DYNAMIC_ENTRY_BOX_BASE_SET_RADIAL_SETTINGS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetEntrySpacing"),
            &raw mut U_DYNAMIC_ENTRY_BOX_BASE_SET_ENTRY_SPACING,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetNumEntries"),
            &raw mut U_DYNAMIC_ENTRY_BOX_BASE_GET_NUM_ENTRIES,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetAllEntries"),
            &raw mut U_DYNAMIC_ENTRY_BOX_BASE_GET_ALL_ENTRIES,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UDynamicEntryBox::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("Reset"),
            &raw mut U_DYNAMIC_ENTRY_BOX_RESET,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RemoveEntry"),
            &raw mut U_DYNAMIC_ENTRY_BOX_REMOVE_ENTRY,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("BP_CreateEntryOfClass"),
            &raw mut U_DYNAMIC_ENTRY_BOX_BP_CREATE_ENTRY_OF_CLASS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("BP_CreateEntry"),
            &raw mut U_DYNAMIC_ENTRY_BOX_BP_CREATE_ENTRY,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UGridPanel::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetRowFill"),
            &raw mut U_GRID_PANEL_SET_ROW_FILL,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetColumnFill"),
            &raw mut U_GRID_PANEL_SET_COLUMN_FILL,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ClearFill"),
            &raw mut U_GRID_PANEL_CLEAR_FILL,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AddChildToGrid"),
            &raw mut U_GRID_PANEL_ADD_CHILD_TO_GRID,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UGridSlot::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetVerticalAlignment"),
            &raw mut U_GRID_SLOT_SET_VERTICAL_ALIGNMENT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetRowSpan"),
            &raw mut U_GRID_SLOT_SET_ROW_SPAN,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetRow"),
            &raw mut U_GRID_SLOT_SET_ROW,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetPadding"),
            &raw mut U_GRID_SLOT_SET_PADDING,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetNudge"),
            &raw mut U_GRID_SLOT_SET_NUDGE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetLayer"),
            &raw mut U_GRID_SLOT_SET_LAYER,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetHorizontalAlignment"),
            &raw mut U_GRID_SLOT_SET_HORIZONTAL_ALIGNMENT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetColumnSpan"),
            &raw mut U_GRID_SLOT_SET_COLUMN_SPAN,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetColumn"),
            &raw mut U_GRID_SLOT_SET_COLUMN,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UHorizontalBox::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AddChildToHorizontalBox"),
            &raw mut U_HORIZONTAL_BOX_ADD_CHILD_TO_HORIZONTAL_BOX,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UHorizontalBoxSlot::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetVerticalAlignment"),
            &raw mut U_HORIZONTAL_BOX_SLOT_SET_VERTICAL_ALIGNMENT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetSize"),
            &raw mut U_HORIZONTAL_BOX_SLOT_SET_SIZE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetPadding"),
            &raw mut U_HORIZONTAL_BOX_SLOT_SET_PADDING,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetHorizontalAlignment"),
            &raw mut U_HORIZONTAL_BOX_SLOT_SET_HORIZONTAL_ALIGNMENT,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UImage::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetOpacity"),
            &raw mut U_IMAGE_SET_OPACITY,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetDesiredSizeOverride"),
            &raw mut U_IMAGE_SET_DESIRED_SIZE_OVERRIDE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetColorAndOpacity"),
            &raw mut U_IMAGE_SET_COLOR_AND_OPACITY,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetBrushTintColor"),
            &raw mut U_IMAGE_SET_BRUSH_TINT_COLOR,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetBrushResourceObject"),
            &raw mut U_IMAGE_SET_BRUSH_RESOURCE_OBJECT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetBrushFromTextureDynamic"),
            &raw mut U_IMAGE_SET_BRUSH_FROM_TEXTURE_DYNAMIC,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetBrushFromTexture"),
            &raw mut U_IMAGE_SET_BRUSH_FROM_TEXTURE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetBrushFromSoftTexture"),
            &raw mut U_IMAGE_SET_BRUSH_FROM_SOFT_TEXTURE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetBrushFromSoftMaterial"),
            &raw mut U_IMAGE_SET_BRUSH_FROM_SOFT_MATERIAL,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetBrushFromMaterial"),
            &raw mut U_IMAGE_SET_BRUSH_FROM_MATERIAL,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetBrushFromAtlasInterface"),
            &raw mut U_IMAGE_SET_BRUSH_FROM_ATLAS_INTERFACE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetBrushFromAsset"),
            &raw mut U_IMAGE_SET_BRUSH_FROM_ASSET,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetBrush"),
            &raw mut U_IMAGE_SET_BRUSH,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetDynamicMaterial"),
            &raw mut U_IMAGE_GET_DYNAMIC_MATERIAL,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UInvalidationBox::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetCanCache"),
            &raw mut U_INVALIDATION_BOX_SET_CAN_CACHE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("InvalidateCache"),
            &raw mut U_INVALIDATION_BOX_INVALIDATE_CACHE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetCanCache"),
            &raw mut U_INVALIDATION_BOX_GET_CAN_CACHE,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UMenuAnchor::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ToggleOpen"),
            &raw mut U_MENU_ANCHOR_TOGGLE_OPEN,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ShouldOpenDueToClick"),
            &raw mut U_MENU_ANCHOR_SHOULD_OPEN_DUE_TO_CLICK,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetPlacement"),
            &raw mut U_MENU_ANCHOR_SET_PLACEMENT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("Open"),
            &raw mut U_MENU_ANCHOR_OPEN,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("IsOpen"),
            &raw mut U_MENU_ANCHOR_IS_OPEN,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("HasOpenSubMenus"),
            &raw mut U_MENU_ANCHOR_HAS_OPEN_SUB_MENUS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetUserWidget__DelegateSignature"),
            &raw mut U_MENU_ANCHOR_GET_USER_WIDGET_DELEGATE_SIGNATURE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetMenuPosition"),
            &raw mut U_MENU_ANCHOR_GET_MENU_POSITION,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("FitInWindow"),
            &raw mut U_MENU_ANCHOR_FIT_IN_WINDOW,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("Close"),
            &raw mut U_MENU_ANCHOR_CLOSE,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UOverlay::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ReplaceOverlayChildAt"),
            &raw mut U_OVERLAY_REPLACE_OVERLAY_CHILD_AT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AddChildToOverlay"),
            &raw mut U_OVERLAY_ADD_CHILD_TO_OVERLAY,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UOverlaySlot::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetVerticalAlignment"),
            &raw mut U_OVERLAY_SLOT_SET_VERTICAL_ALIGNMENT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetPadding"),
            &raw mut U_OVERLAY_SLOT_SET_PADDING,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetHorizontalAlignment"),
            &raw mut U_OVERLAY_SLOT_SET_HORIZONTAL_ALIGNMENT,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = URetainerBox::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetTextureParameter"),
            &raw mut U_RETAINER_BOX_SET_TEXTURE_PARAMETER,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetRetainRendering"),
            &raw mut U_RETAINER_BOX_SET_RETAIN_RENDERING,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetRenderingPhase"),
            &raw mut U_RETAINER_BOX_SET_RENDERING_PHASE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetEffectMaterial"),
            &raw mut U_RETAINER_BOX_SET_EFFECT_MATERIAL,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RequestRender"),
            &raw mut U_RETAINER_BOX_REQUEST_RENDER,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetEffectMaterial"),
            &raw mut U_RETAINER_BOX_GET_EFFECT_MATERIAL,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = URichTextBlock::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetTextTransformPolicy"),
            &raw mut U_RICH_TEXT_BLOCK_SET_TEXT_TRANSFORM_POLICY,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetTextStyleSet"),
            &raw mut U_RICH_TEXT_BLOCK_SET_TEXT_STYLE_SET,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetTextOverflowPolicy"),
            &raw mut U_RICH_TEXT_BLOCK_SET_TEXT_OVERFLOW_POLICY,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetText"),
            &raw mut U_RICH_TEXT_BLOCK_SET_TEXT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetMinDesiredWidth"),
            &raw mut U_RICH_TEXT_BLOCK_SET_MIN_DESIRED_WIDTH,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetDefaultTextStyle"),
            &raw mut U_RICH_TEXT_BLOCK_SET_DEFAULT_TEXT_STYLE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetDefaultStrikeBrush"),
            &raw mut U_RICH_TEXT_BLOCK_SET_DEFAULT_STRIKE_BRUSH,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetDefaultShadowOffset"),
            &raw mut U_RICH_TEXT_BLOCK_SET_DEFAULT_SHADOW_OFFSET,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetDefaultShadowColorAndOpacity"),
            &raw mut U_RICH_TEXT_BLOCK_SET_DEFAULT_SHADOW_COLOR_AND_OPACITY,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetDefaultMaterial"),
            &raw mut U_RICH_TEXT_BLOCK_SET_DEFAULT_MATERIAL,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetDefaultFont"),
            &raw mut U_RICH_TEXT_BLOCK_SET_DEFAULT_FONT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetDefaultColorAndOpacity"),
            &raw mut U_RICH_TEXT_BLOCK_SET_DEFAULT_COLOR_AND_OPACITY,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetDecorators"),
            &raw mut U_RICH_TEXT_BLOCK_SET_DECORATORS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetAutoWrapText"),
            &raw mut U_RICH_TEXT_BLOCK_SET_AUTO_WRAP_TEXT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RefreshTextLayout"),
            &raw mut U_RICH_TEXT_BLOCK_REFRESH_TEXT_LAYOUT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetTextStyleSet"),
            &raw mut U_RICH_TEXT_BLOCK_GET_TEXT_STYLE_SET,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetText"),
            &raw mut U_RICH_TEXT_BLOCK_GET_TEXT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetDefaultDynamicMaterial"),
            &raw mut U_RICH_TEXT_BLOCK_GET_DEFAULT_DYNAMIC_MATERIAL,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetDecoratorByClass"),
            &raw mut U_RICH_TEXT_BLOCK_GET_DECORATOR_BY_CLASS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ClearAllDefaultStyleOverrides"),
            &raw mut U_RICH_TEXT_BLOCK_CLEAR_ALL_DEFAULT_STYLE_OVERRIDES,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = USafeZone::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetSidesToPad"),
            &raw mut U_SAFE_ZONE_SET_SIDES_TO_PAD,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UScaleBox::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetUserSpecifiedScale"),
            &raw mut U_SCALE_BOX_SET_USER_SPECIFIED_SCALE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetStretchDirection"),
            &raw mut U_SCALE_BOX_SET_STRETCH_DIRECTION,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetStretch"),
            &raw mut U_SCALE_BOX_SET_STRETCH,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetIgnoreInheritedScale"),
            &raw mut U_SCALE_BOX_SET_IGNORE_INHERITED_SCALE,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UScaleBoxSlot::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetVerticalAlignment"),
            &raw mut U_SCALE_BOX_SLOT_SET_VERTICAL_ALIGNMENT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetPadding"),
            &raw mut U_SCALE_BOX_SLOT_SET_PADDING,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetHorizontalAlignment"),
            &raw mut U_SCALE_BOX_SLOT_SET_HORIZONTAL_ALIGNMENT,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UScrollBoxSlot::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetVerticalAlignment"),
            &raw mut U_SCROLL_BOX_SLOT_SET_VERTICAL_ALIGNMENT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetPadding"),
            &raw mut U_SCROLL_BOX_SLOT_SET_PADDING,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetHorizontalAlignment"),
            &raw mut U_SCROLL_BOX_SLOT_SET_HORIZONTAL_ALIGNMENT,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = USizeBox::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetWidthOverride"),
            &raw mut U_SIZE_BOX_SET_WIDTH_OVERRIDE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetMinDesiredWidth"),
            &raw mut U_SIZE_BOX_SET_MIN_DESIRED_WIDTH,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetMinDesiredHeight"),
            &raw mut U_SIZE_BOX_SET_MIN_DESIRED_HEIGHT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetMinAspectRatio"),
            &raw mut U_SIZE_BOX_SET_MIN_ASPECT_RATIO,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetMaxDesiredWidth"),
            &raw mut U_SIZE_BOX_SET_MAX_DESIRED_WIDTH,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetMaxDesiredHeight"),
            &raw mut U_SIZE_BOX_SET_MAX_DESIRED_HEIGHT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetMaxAspectRatio"),
            &raw mut U_SIZE_BOX_SET_MAX_ASPECT_RATIO,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetHeightOverride"),
            &raw mut U_SIZE_BOX_SET_HEIGHT_OVERRIDE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ClearWidthOverride"),
            &raw mut U_SIZE_BOX_CLEAR_WIDTH_OVERRIDE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ClearMinDesiredWidth"),
            &raw mut U_SIZE_BOX_CLEAR_MIN_DESIRED_WIDTH,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ClearMinDesiredHeight"),
            &raw mut U_SIZE_BOX_CLEAR_MIN_DESIRED_HEIGHT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ClearMinAspectRatio"),
            &raw mut U_SIZE_BOX_CLEAR_MIN_ASPECT_RATIO,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ClearMaxDesiredWidth"),
            &raw mut U_SIZE_BOX_CLEAR_MAX_DESIRED_WIDTH,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ClearMaxDesiredHeight"),
            &raw mut U_SIZE_BOX_CLEAR_MAX_DESIRED_HEIGHT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ClearMaxAspectRatio"),
            &raw mut U_SIZE_BOX_CLEAR_MAX_ASPECT_RATIO,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ClearHeightOverride"),
            &raw mut U_SIZE_BOX_CLEAR_HEIGHT_OVERRIDE,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = USizeBoxComponent::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ClearWidthOverride"),
            &raw mut U_SIZE_BOX_COMPONENT_CLEAR_WIDTH_OVERRIDE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ClearMinDesiredWidth"),
            &raw mut U_SIZE_BOX_COMPONENT_CLEAR_MIN_DESIRED_WIDTH,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ClearMinDesiredHeight"),
            &raw mut U_SIZE_BOX_COMPONENT_CLEAR_MIN_DESIRED_HEIGHT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ClearMinAspectRatio"),
            &raw mut U_SIZE_BOX_COMPONENT_CLEAR_MIN_ASPECT_RATIO,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ClearMaxDesiredWidth"),
            &raw mut U_SIZE_BOX_COMPONENT_CLEAR_MAX_DESIRED_WIDTH,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ClearMaxDesiredHeight"),
            &raw mut U_SIZE_BOX_COMPONENT_CLEAR_MAX_DESIRED_HEIGHT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ClearMaxAspectRatio"),
            &raw mut U_SIZE_BOX_COMPONENT_CLEAR_MAX_ASPECT_RATIO,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ClearHeightOverride"),
            &raw mut U_SIZE_BOX_COMPONENT_CLEAR_HEIGHT_OVERRIDE,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = USizeBoxSlot::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetVerticalAlignment"),
            &raw mut U_SIZE_BOX_SLOT_SET_VERTICAL_ALIGNMENT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetPadding"),
            &raw mut U_SIZE_BOX_SLOT_SET_PADDING,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetHorizontalAlignment"),
            &raw mut U_SIZE_BOX_SLOT_SET_HORIZONTAL_ALIGNMENT,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = USpacer::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetSize"),
            &raw mut U_SPACER_SET_SIZE,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UStackBox::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ReplaceStackBoxChildAt"),
            &raw mut U_STACK_BOX_REPLACE_STACK_BOX_CHILD_AT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AddChildToStackBox"),
            &raw mut U_STACK_BOX_ADD_CHILD_TO_STACK_BOX,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UTextBlock::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetTextTransformPolicy"),
            &raw mut U_TEXT_BLOCK_SET_TEXT_TRANSFORM_POLICY,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetTextOverflowPolicy"),
            &raw mut U_TEXT_BLOCK_SET_TEXT_OVERFLOW_POLICY,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetText"),
            &raw mut U_TEXT_BLOCK_SET_TEXT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetStrikeBrush"),
            &raw mut U_TEXT_BLOCK_SET_STRIKE_BRUSH,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetShadowOffset"),
            &raw mut U_TEXT_BLOCK_SET_SHADOW_OFFSET,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetShadowColorAndOpacity"),
            &raw mut U_TEXT_BLOCK_SET_SHADOW_COLOR_AND_OPACITY,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetOpacity"),
            &raw mut U_TEXT_BLOCK_SET_OPACITY,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetMinDesiredWidth"),
            &raw mut U_TEXT_BLOCK_SET_MIN_DESIRED_WIDTH,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetFontOutlineMaterial"),
            &raw mut U_TEXT_BLOCK_SET_FONT_OUTLINE_MATERIAL,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetFontMaterial"),
            &raw mut U_TEXT_BLOCK_SET_FONT_MATERIAL,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetFont"),
            &raw mut U_TEXT_BLOCK_SET_FONT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetColorAndOpacity"),
            &raw mut U_TEXT_BLOCK_SET_COLOR_AND_OPACITY,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetAutoWrapText"),
            &raw mut U_TEXT_BLOCK_SET_AUTO_WRAP_TEXT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetText"),
            &raw mut U_TEXT_BLOCK_GET_TEXT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetDynamicOutlineMaterial"),
            &raw mut U_TEXT_BLOCK_GET_DYNAMIC_OUTLINE_MATERIAL,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetDynamicFontMaterial"),
            &raw mut U_TEXT_BLOCK_GET_DYNAMIC_FONT_MATERIAL,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UTileView::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetEntryWidth"),
            &raw mut U_TILE_VIEW_SET_ENTRY_WIDTH,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetEntryHeight"),
            &raw mut U_TILE_VIEW_SET_ENTRY_HEIGHT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("IsAligned"),
            &raw mut U_TILE_VIEW_IS_ALIGNED,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetEntryWidth"),
            &raw mut U_TILE_VIEW_GET_ENTRY_WIDTH,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetEntryHeight"),
            &raw mut U_TILE_VIEW_GET_ENTRY_HEIGHT,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UUniformGridPanel::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetSlotPadding"),
            &raw mut U_UNIFORM_GRID_PANEL_SET_SLOT_PADDING,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetMinDesiredSlotWidth"),
            &raw mut U_UNIFORM_GRID_PANEL_SET_MIN_DESIRED_SLOT_WIDTH,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetMinDesiredSlotHeight"),
            &raw mut U_UNIFORM_GRID_PANEL_SET_MIN_DESIRED_SLOT_HEIGHT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AddChildToUniformGrid"),
            &raw mut U_UNIFORM_GRID_PANEL_ADD_CHILD_TO_UNIFORM_GRID,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UUniformGridSlot::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetVerticalAlignment"),
            &raw mut U_UNIFORM_GRID_SLOT_SET_VERTICAL_ALIGNMENT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetRow"),
            &raw mut U_UNIFORM_GRID_SLOT_SET_ROW,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetHorizontalAlignment"),
            &raw mut U_UNIFORM_GRID_SLOT_SET_HORIZONTAL_ALIGNMENT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetColumn"),
            &raw mut U_UNIFORM_GRID_SLOT_SET_COLUMN,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UVerticalBox::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AddChildToVerticalBox"),
            &raw mut U_VERTICAL_BOX_ADD_CHILD_TO_VERTICAL_BOX,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UVerticalBoxSlot::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetVerticalAlignment"),
            &raw mut U_VERTICAL_BOX_SLOT_SET_VERTICAL_ALIGNMENT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetSize"),
            &raw mut U_VERTICAL_BOX_SLOT_SET_SIZE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetPadding"),
            &raw mut U_VERTICAL_BOX_SLOT_SET_PADDING,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetHorizontalAlignment"),
            &raw mut U_VERTICAL_BOX_SLOT_SET_HORIZONTAL_ALIGNMENT,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UViewport::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("Spawn"),
            &raw mut U_VIEWPORT_SPAWN,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetViewRotation"),
            &raw mut U_VIEWPORT_SET_VIEW_ROTATION,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetViewLocation"),
            &raw mut U_VIEWPORT_SET_VIEW_LOCATION,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetSkyIntensity"),
            &raw mut U_VIEWPORT_SET_SKY_INTENSITY,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetShowFlag"),
            &raw mut U_VIEWPORT_SET_SHOW_FLAG,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetLightIntensity"),
            &raw mut U_VIEWPORT_SET_LIGHT_INTENSITY,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetEnableAdvancedFeatures"),
            &raw mut U_VIEWPORT_SET_ENABLE_ADVANCED_FEATURES,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetViewRotation"),
            &raw mut U_VIEWPORT_GET_VIEW_ROTATION,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetViewProjectionMatrix"),
            &raw mut U_VIEWPORT_GET_VIEW_PROJECTION_MATRIX,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetViewportWorld"),
            &raw mut U_VIEWPORT_GET_VIEWPORT_WORLD,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetViewLocation"),
            &raw mut U_VIEWPORT_GET_VIEW_LOCATION,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UWidgetInteractionComponent::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetFocus"),
            &raw mut U_WIDGET_INTERACTION_COMPONENT_SET_FOCUS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetCustomHitResult"),
            &raw mut U_WIDGET_INTERACTION_COMPONENT_SET_CUSTOM_HIT_RESULT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SendKeyChar"),
            &raw mut U_WIDGET_INTERACTION_COMPONENT_SEND_KEY_CHAR,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ScrollWheel"),
            &raw mut U_WIDGET_INTERACTION_COMPONENT_SCROLL_WHEEL,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ReleasePointerKey"),
            &raw mut U_WIDGET_INTERACTION_COMPONENT_RELEASE_POINTER_KEY,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ReleaseKey"),
            &raw mut U_WIDGET_INTERACTION_COMPONENT_RELEASE_KEY,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("PressPointerKey"),
            &raw mut U_WIDGET_INTERACTION_COMPONENT_PRESS_POINTER_KEY,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("PressKey"),
            &raw mut U_WIDGET_INTERACTION_COMPONENT_PRESS_KEY,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("PressAndReleaseKey"),
            &raw mut U_WIDGET_INTERACTION_COMPONENT_PRESS_AND_RELEASE_KEY,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("IsOverInteractableWidget"),
            &raw mut U_WIDGET_INTERACTION_COMPONENT_IS_OVER_INTERACTABLE_WIDGET,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("IsOverHitTestVisibleWidget"),
            &raw mut U_WIDGET_INTERACTION_COMPONENT_IS_OVER_HIT_TEST_VISIBLE_WIDGET,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("IsOverFocusableWidget"),
            &raw mut U_WIDGET_INTERACTION_COMPONENT_IS_OVER_FOCUSABLE_WIDGET,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetLastHitResult"),
            &raw mut U_WIDGET_INTERACTION_COMPONENT_GET_LAST_HIT_RESULT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetHoveredWidgetComponent"),
            &raw mut U_WIDGET_INTERACTION_COMPONENT_GET_HOVERED_WIDGET_COMPONENT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("Get2DHitLocation"),
            &raw mut U_WIDGET_INTERACTION_COMPONENT_GET2_D_HIT_LOCATION,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UWidgetSwitcher::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetActiveWidgetIndex"),
            &raw mut U_WIDGET_SWITCHER_SET_ACTIVE_WIDGET_INDEX,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetActiveWidget"),
            &raw mut U_WIDGET_SWITCHER_SET_ACTIVE_WIDGET,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetWidgetAtIndex"),
            &raw mut U_WIDGET_SWITCHER_GET_WIDGET_AT_INDEX,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetNumWidgets"),
            &raw mut U_WIDGET_SWITCHER_GET_NUM_WIDGETS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetActiveWidgetIndex"),
            &raw mut U_WIDGET_SWITCHER_GET_ACTIVE_WIDGET_INDEX,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetActiveWidget"),
            &raw mut U_WIDGET_SWITCHER_GET_ACTIVE_WIDGET,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UWidgetSwitcherSlot::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetVerticalAlignment"),
            &raw mut U_WIDGET_SWITCHER_SLOT_SET_VERTICAL_ALIGNMENT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetPadding"),
            &raw mut U_WIDGET_SWITCHER_SLOT_SET_PADDING,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetHorizontalAlignment"),
            &raw mut U_WIDGET_SWITCHER_SLOT_SET_HORIZONTAL_ALIGNMENT,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UWindowTitleBarArea::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetVerticalAlignment"),
            &raw mut U_WINDOW_TITLE_BAR_AREA_SET_VERTICAL_ALIGNMENT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetPadding"),
            &raw mut U_WINDOW_TITLE_BAR_AREA_SET_PADDING,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetHorizontalAlignment"),
            &raw mut U_WINDOW_TITLE_BAR_AREA_SET_HORIZONTAL_ALIGNMENT,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UWindowTitleBarAreaSlot::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetVerticalAlignment"),
            &raw mut U_WINDOW_TITLE_BAR_AREA_SLOT_SET_VERTICAL_ALIGNMENT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetPadding"),
            &raw mut U_WINDOW_TITLE_BAR_AREA_SLOT_SET_PADDING,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetHorizontalAlignment"),
            &raw mut U_WINDOW_TITLE_BAR_AREA_SLOT_SET_HORIZONTAL_ALIGNMENT,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UWrapBox::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetInnerSlotPadding"),
            &raw mut U_WRAP_BOX_SET_INNER_SLOT_PADDING,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetHorizontalAlignment"),
            &raw mut U_WRAP_BOX_SET_HORIZONTAL_ALIGNMENT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AddChildToWrapBox"),
            &raw mut U_WRAP_BOX_ADD_CHILD_TO_WRAP_BOX,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UWrapBoxSlot::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetVerticalAlignment"),
            &raw mut U_WRAP_BOX_SLOT_SET_VERTICAL_ALIGNMENT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetPadding"),
            &raw mut U_WRAP_BOX_SLOT_SET_PADDING,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetNewLine"),
            &raw mut U_WRAP_BOX_SLOT_SET_NEW_LINE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetHorizontalAlignment"),
            &raw mut U_WRAP_BOX_SLOT_SET_HORIZONTAL_ALIGNMENT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetFillSpanWhenLessThan"),
            &raw mut U_WRAP_BOX_SLOT_SET_FILL_SPAN_WHEN_LESS_THAN,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetFillEmptySpace"),
            &raw mut U_WRAP_BOX_SLOT_SET_FILL_EMPTY_SPACE,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UDragDropOperation::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("Drop"),
            &raw mut U_DRAG_DROP_OPERATION_DROP,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("Dragged"),
            &raw mut U_DRAG_DROP_OPERATION_DRAGGED,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("DragCancelled"),
            &raw mut U_DRAG_DROP_OPERATION_DRAG_CANCELLED,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = USlateBlueprintLibrary::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("Vector_LocalToAbsolute"),
            &raw mut U_SLATE_BLUEPRINT_LIBRARY_VECTOR_LOCAL_TO_ABSOLUTE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("Vector_AbsoluteToLocal"),
            &raw mut U_SLATE_BLUEPRINT_LIBRARY_VECTOR_ABSOLUTE_TO_LOCAL,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("TransformVectorLocalToAbsolute"),
            &raw mut U_SLATE_BLUEPRINT_LIBRARY_TRANSFORM_VECTOR_LOCAL_TO_ABSOLUTE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("TransformVectorAbsoluteToLocal"),
            &raw mut U_SLATE_BLUEPRINT_LIBRARY_TRANSFORM_VECTOR_ABSOLUTE_TO_LOCAL,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("TransformScalarLocalToAbsolute"),
            &raw mut U_SLATE_BLUEPRINT_LIBRARY_TRANSFORM_SCALAR_LOCAL_TO_ABSOLUTE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("TransformScalarAbsoluteToLocal"),
            &raw mut U_SLATE_BLUEPRINT_LIBRARY_TRANSFORM_SCALAR_ABSOLUTE_TO_LOCAL,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ScreenToWidgetLocal"),
            &raw mut U_SLATE_BLUEPRINT_LIBRARY_SCREEN_TO_WIDGET_LOCAL,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ScreenToWidgetAbsolute"),
            &raw mut U_SLATE_BLUEPRINT_LIBRARY_SCREEN_TO_WIDGET_ABSOLUTE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ScreenToViewport"),
            &raw mut U_SLATE_BLUEPRINT_LIBRARY_SCREEN_TO_VIEWPORT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("Scalar_LocalToAbsolute"),
            &raw mut U_SLATE_BLUEPRINT_LIBRARY_SCALAR_LOCAL_TO_ABSOLUTE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("Scalar_AbsoluteToLocal"),
            &raw mut U_SLATE_BLUEPRINT_LIBRARY_SCALAR_ABSOLUTE_TO_LOCAL,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("LocalToViewport"),
            &raw mut U_SLATE_BLUEPRINT_LIBRARY_LOCAL_TO_VIEWPORT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("LocalToAbsolute"),
            &raw mut U_SLATE_BLUEPRINT_LIBRARY_LOCAL_TO_ABSOLUTE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("IsUnderLocation"),
            &raw mut U_SLATE_BLUEPRINT_LIBRARY_IS_UNDER_LOCATION,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetLocalTopLeft"),
            &raw mut U_SLATE_BLUEPRINT_LIBRARY_GET_LOCAL_TOP_LEFT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetLocalSize"),
            &raw mut U_SLATE_BLUEPRINT_LIBRARY_GET_LOCAL_SIZE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetAbsoluteSize"),
            &raw mut U_SLATE_BLUEPRINT_LIBRARY_GET_ABSOLUTE_SIZE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("EqualEqual_SlateBrush"),
            &raw mut U_SLATE_BLUEPRINT_LIBRARY_EQUAL_EQUAL_SLATE_BRUSH,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AbsoluteToViewport"),
            &raw mut U_SLATE_BLUEPRINT_LIBRARY_ABSOLUTE_TO_VIEWPORT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AbsoluteToLocal"),
            &raw mut U_SLATE_BLUEPRINT_LIBRARY_ABSOLUTE_TO_LOCAL,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UUserWidgetFunctionLibrary::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetOuterUserWidget"),
            &raw mut U_USER_WIDGET_FUNCTION_LIBRARY_GET_OUTER_USER_WIDGET,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("Conv_UMGSequencePlayer"),
            &raw mut U_USER_WIDGET_FUNCTION_LIBRARY_CONV_UMG_SEQUENCE_PLAYER,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UWidgetBlueprintLibrary::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("UnlockMouse"),
            &raw mut U_WIDGET_BLUEPRINT_LIBRARY_UNLOCK_MOUSE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("Unhandled"),
            &raw mut U_WIDGET_BLUEPRINT_LIBRARY_UNHANDLED,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetWindowTitleBarState"),
            &raw mut U_WIDGET_BLUEPRINT_LIBRARY_SET_WINDOW_TITLE_BAR_STATE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetWindowTitleBarOnCloseClickedDelegate"),
            &raw mut U_WIDGET_BLUEPRINT_LIBRARY_SET_WINDOW_TITLE_BAR_ON_CLOSE_CLICKED_DELEGATE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetWindowTitleBarCloseButtonActive"),
            &raw mut U_WIDGET_BLUEPRINT_LIBRARY_SET_WINDOW_TITLE_BAR_CLOSE_BUTTON_ACTIVE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetUserFocus"),
            &raw mut U_WIDGET_BLUEPRINT_LIBRARY_SET_USER_FOCUS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetMousePosition"),
            &raw mut U_WIDGET_BLUEPRINT_LIBRARY_SET_MOUSE_POSITION,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetInputMode_UIOnlyEx"),
            &raw mut U_WIDGET_BLUEPRINT_LIBRARY_SET_INPUT_MODE_UI_ONLY_EX,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetInputMode_GameOnly"),
            &raw mut U_WIDGET_BLUEPRINT_LIBRARY_SET_INPUT_MODE_GAME_ONLY,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetInputMode_GameAndUIEx"),
            &raw mut U_WIDGET_BLUEPRINT_LIBRARY_SET_INPUT_MODE_GAME_AND_UI_EX,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetHardwareCursor"),
            &raw mut U_WIDGET_BLUEPRINT_LIBRARY_SET_HARDWARE_CURSOR,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetFocusToGameViewport"),
            &raw mut U_WIDGET_BLUEPRINT_LIBRARY_SET_FOCUS_TO_GAME_VIEWPORT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetColorVisionDeficiencyType"),
            &raw mut U_WIDGET_BLUEPRINT_LIBRARY_SET_COLOR_VISION_DEFICIENCY_TYPE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetBrushResourceToTexture"),
            &raw mut U_WIDGET_BLUEPRINT_LIBRARY_SET_BRUSH_RESOURCE_TO_TEXTURE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetBrushResourceToMaterial"),
            &raw mut U_WIDGET_BLUEPRINT_LIBRARY_SET_BRUSH_RESOURCE_TO_MATERIAL,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RestorePreviousWindowTitleBarState"),
            &raw mut U_WIDGET_BLUEPRINT_LIBRARY_RESTORE_PREVIOUS_WINDOW_TITLE_BAR_STATE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ReleaseMouseCapture"),
            &raw mut U_WIDGET_BLUEPRINT_LIBRARY_RELEASE_MOUSE_CAPTURE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ReleaseJoystickCapture"),
            &raw mut U_WIDGET_BLUEPRINT_LIBRARY_RELEASE_JOYSTICK_CAPTURE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from(
                "OnGameWindowCloseButtonClickedDelegate__DelegateSignature",
            ),
            &raw mut U_WIDGET_BLUEPRINT_LIBRARY_ON_GAME_WINDOW_CLOSE_BUTTON_CLICKED_DELEGATE_DELEGATE_SIGNATURE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("NoResourceBrush"),
            &raw mut U_WIDGET_BLUEPRINT_LIBRARY_NO_RESOURCE_BRUSH,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("MakeBrushFromTexture"),
            &raw mut U_WIDGET_BLUEPRINT_LIBRARY_MAKE_BRUSH_FROM_TEXTURE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("MakeBrushFromMaterial"),
            &raw mut U_WIDGET_BLUEPRINT_LIBRARY_MAKE_BRUSH_FROM_MATERIAL,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("MakeBrushFromAsset"),
            &raw mut U_WIDGET_BLUEPRINT_LIBRARY_MAKE_BRUSH_FROM_ASSET,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("LockMouse"),
            &raw mut U_WIDGET_BLUEPRINT_LIBRARY_LOCK_MOUSE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("IsDragDropping"),
            &raw mut U_WIDGET_BLUEPRINT_LIBRARY_IS_DRAG_DROPPING,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("Handled"),
            &raw mut U_WIDGET_BLUEPRINT_LIBRARY_HANDLED,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetSafeZonePadding"),
            &raw mut U_WIDGET_BLUEPRINT_LIBRARY_GET_SAFE_ZONE_PADDING,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetKeyEventFromAnalogInputEvent"),
            &raw mut U_WIDGET_BLUEPRINT_LIBRARY_GET_KEY_EVENT_FROM_ANALOG_INPUT_EVENT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetInputEventFromPointerEvent"),
            &raw mut U_WIDGET_BLUEPRINT_LIBRARY_GET_INPUT_EVENT_FROM_POINTER_EVENT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetInputEventFromNavigationEvent"),
            &raw mut U_WIDGET_BLUEPRINT_LIBRARY_GET_INPUT_EVENT_FROM_NAVIGATION_EVENT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetInputEventFromKeyEvent"),
            &raw mut U_WIDGET_BLUEPRINT_LIBRARY_GET_INPUT_EVENT_FROM_KEY_EVENT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetInputEventFromCharacterEvent"),
            &raw mut U_WIDGET_BLUEPRINT_LIBRARY_GET_INPUT_EVENT_FROM_CHARACTER_EVENT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetDynamicMaterial"),
            &raw mut U_WIDGET_BLUEPRINT_LIBRARY_GET_DYNAMIC_MATERIAL,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetDragDroppingContent"),
            &raw mut U_WIDGET_BLUEPRINT_LIBRARY_GET_DRAG_DROPPING_CONTENT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetBrushResourceAsTexture2D"),
            &raw mut U_WIDGET_BLUEPRINT_LIBRARY_GET_BRUSH_RESOURCE_AS_TEXTURE2_D,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetBrushResourceAsMaterial"),
            &raw mut U_WIDGET_BLUEPRINT_LIBRARY_GET_BRUSH_RESOURCE_AS_MATERIAL,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetBrushResource"),
            &raw mut U_WIDGET_BLUEPRINT_LIBRARY_GET_BRUSH_RESOURCE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetAllWidgetsWithInterface"),
            &raw mut U_WIDGET_BLUEPRINT_LIBRARY_GET_ALL_WIDGETS_WITH_INTERFACE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetAllWidgetsOfClass"),
            &raw mut U_WIDGET_BLUEPRINT_LIBRARY_GET_ALL_WIDGETS_OF_CLASS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("EndDragDrop"),
            &raw mut U_WIDGET_BLUEPRINT_LIBRARY_END_DRAG_DROP,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("DrawTextFormatted"),
            &raw mut U_WIDGET_BLUEPRINT_LIBRARY_DRAW_TEXT_FORMATTED,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("DrawText"),
            &raw mut U_WIDGET_BLUEPRINT_LIBRARY_DRAW_TEXT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("DrawSpline"),
            &raw mut U_WIDGET_BLUEPRINT_LIBRARY_DRAW_SPLINE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("DrawLines"),
            &raw mut U_WIDGET_BLUEPRINT_LIBRARY_DRAW_LINES,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("DrawLine"),
            &raw mut U_WIDGET_BLUEPRINT_LIBRARY_DRAW_LINE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("DrawBox"),
            &raw mut U_WIDGET_BLUEPRINT_LIBRARY_DRAW_BOX,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("DismissAllMenus"),
            &raw mut U_WIDGET_BLUEPRINT_LIBRARY_DISMISS_ALL_MENUS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("DetectDragIfPressed"),
            &raw mut U_WIDGET_BLUEPRINT_LIBRARY_DETECT_DRAG_IF_PRESSED,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("DetectDrag"),
            &raw mut U_WIDGET_BLUEPRINT_LIBRARY_DETECT_DRAG,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CreateDragDropOperation"),
            &raw mut U_WIDGET_BLUEPRINT_LIBRARY_CREATE_DRAG_DROP_OPERATION,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("Create"),
            &raw mut U_WIDGET_BLUEPRINT_LIBRARY_CREATE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ClearUserFocus"),
            &raw mut U_WIDGET_BLUEPRINT_LIBRARY_CLEAR_USER_FOCUS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CaptureMouse"),
            &raw mut U_WIDGET_BLUEPRINT_LIBRARY_CAPTURE_MOUSE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CaptureJoystick"),
            &raw mut U_WIDGET_BLUEPRINT_LIBRARY_CAPTURE_JOYSTICK,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CancelDragDrop"),
            &raw mut U_WIDGET_BLUEPRINT_LIBRARY_CANCEL_DRAG_DROP,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UWidgetLayoutLibrary::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SlotAsWrapBoxSlot"),
            &raw mut U_WIDGET_LAYOUT_LIBRARY_SLOT_AS_WRAP_BOX_SLOT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SlotAsWidgetSwitcherSlot"),
            &raw mut U_WIDGET_LAYOUT_LIBRARY_SLOT_AS_WIDGET_SWITCHER_SLOT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SlotAsVerticalBoxSlot"),
            &raw mut U_WIDGET_LAYOUT_LIBRARY_SLOT_AS_VERTICAL_BOX_SLOT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SlotAsUniformGridSlot"),
            &raw mut U_WIDGET_LAYOUT_LIBRARY_SLOT_AS_UNIFORM_GRID_SLOT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SlotAsStackBoxSlot"),
            &raw mut U_WIDGET_LAYOUT_LIBRARY_SLOT_AS_STACK_BOX_SLOT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SlotAsSizeBoxSlot"),
            &raw mut U_WIDGET_LAYOUT_LIBRARY_SLOT_AS_SIZE_BOX_SLOT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SlotAsScrollBoxSlot"),
            &raw mut U_WIDGET_LAYOUT_LIBRARY_SLOT_AS_SCROLL_BOX_SLOT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SlotAsScaleBoxSlot"),
            &raw mut U_WIDGET_LAYOUT_LIBRARY_SLOT_AS_SCALE_BOX_SLOT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SlotAsSafeBoxSlot"),
            &raw mut U_WIDGET_LAYOUT_LIBRARY_SLOT_AS_SAFE_BOX_SLOT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SlotAsOverlaySlot"),
            &raw mut U_WIDGET_LAYOUT_LIBRARY_SLOT_AS_OVERLAY_SLOT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SlotAsHorizontalBoxSlot"),
            &raw mut U_WIDGET_LAYOUT_LIBRARY_SLOT_AS_HORIZONTAL_BOX_SLOT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SlotAsGridSlot"),
            &raw mut U_WIDGET_LAYOUT_LIBRARY_SLOT_AS_GRID_SLOT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SlotAsCanvasSlot"),
            &raw mut U_WIDGET_LAYOUT_LIBRARY_SLOT_AS_CANVAS_SLOT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SlotAsBorderSlot"),
            &raw mut U_WIDGET_LAYOUT_LIBRARY_SLOT_AS_BORDER_SLOT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RemoveAllWidgets"),
            &raw mut U_WIDGET_LAYOUT_LIBRARY_REMOVE_ALL_WIDGETS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ProjectWorldLocationToWidgetPosition"),
            &raw mut U_WIDGET_LAYOUT_LIBRARY_PROJECT_WORLD_LOCATION_TO_WIDGET_POSITION,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetViewportWidgetGeometry"),
            &raw mut U_WIDGET_LAYOUT_LIBRARY_GET_VIEWPORT_WIDGET_GEOMETRY,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetViewportSize"),
            &raw mut U_WIDGET_LAYOUT_LIBRARY_GET_VIEWPORT_SIZE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetViewportScale"),
            &raw mut U_WIDGET_LAYOUT_LIBRARY_GET_VIEWPORT_SCALE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetPlayerScreenWidgetGeometry"),
            &raw mut U_WIDGET_LAYOUT_LIBRARY_GET_PLAYER_SCREEN_WIDGET_GEOMETRY,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetMousePositionScaledByDPI"),
            &raw mut U_WIDGET_LAYOUT_LIBRARY_GET_MOUSE_POSITION_SCALED_BY_DPI,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetMousePositionOnViewport"),
            &raw mut U_WIDGET_LAYOUT_LIBRARY_GET_MOUSE_POSITION_ON_VIEWPORT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetMousePositionOnPlatform"),
            &raw mut U_WIDGET_LAYOUT_LIBRARY_GET_MOUSE_POSITION_ON_PLATFORM,
        );
    }
}
#[repr(C, align(8))]
pub struct FEventReply {
    __padding_end: [u8; 192],
}
impl FEventReply {}
#[repr(C, align(8))]
pub struct FWidgetTransform {
    pub translation: crate::bindings::core_u_object::FVector2D,
    pub scale: crate::bindings::core_u_object::FVector2D,
    pub shear: crate::bindings::core_u_object::FVector2D,
    pub angle: f32,
    __padding_end: [u8; 4],
}
impl FWidgetTransform {}
#[repr(C, align(1))]
pub struct FShapedTextOptions {
    #[doc(hidden)]
    __padding_1: [u8; 1],
    pub text_shaping_method: crate::bindings::slate_core::ETextShapingMethod,
    pub text_flow_direction: crate::bindings::slate::ETextFlowDirection,
}
impl FShapedTextOptions {}
#[repr(C, align(8))]
pub struct FPaintContext {
    __padding_end: [u8; 48],
}
impl FPaintContext {}
#[repr(C, align(4))]
pub struct FRadialBoxSettings {
    __padding_end: [u8; 16],
}
impl FRadialBoxSettings {}
#[repr(C, align(4))]
pub struct FSlateChildSize {
    pub value: f32,
    pub size_rule: ESlateSizeRule,
    __padding_end: [u8; 3],
}
impl FSlateChildSize {}
#[repr(C, align(1))]
pub struct FWidgetEventField {
    __padding_end: [u8; 1],
}
impl FWidgetEventField {}
#[repr(C, align(8))]
pub struct FWidgetNavigationData {
    pub rule: crate::bindings::slate_core::EUINavigationRule,
    pub widget_to_focus: FName,
    __padding_end: [u8; 40],
}
impl FWidgetNavigationData {}
#[repr(C, align(8))]
pub struct FWidgetAnimationHandle {
    __padding_end: [u8; 16],
}
impl FWidgetAnimationHandle {}
#[repr(C, align(8))]
pub struct FGameViewportWidgetSlot {
    pub anchors: crate::bindings::slate::FAnchors,
    pub offsets: crate::bindings::slate_core::FMargin,
    pub alignment: crate::bindings::core_u_object::FVector2D,
    pub z_order: i32,
    pub b_auto_remove_on_world_removed: bool,
    __padding_end: [u8; 3],
}
impl FGameViewportWidgetSlot {}
#[repr(C, align(8))]
pub struct FAnchorData {
    pub offsets: crate::bindings::slate_core::FMargin,
    pub anchors: crate::bindings::slate::FAnchors,
    pub alignment: crate::bindings::core_u_object::FVector2D,
}
impl FAnchorData {}
#[repr(C, align(16))]
pub struct FRichTextStyleRow {
    __padding_end: [u8; 864],
}
impl FRichTextStyleRow {}
#[repr(C, align(16))]
pub struct FRichImageRow {
    __padding_end: [u8; 224],
}
impl FRichImageRow {}
#[repr(C, align(8))]
pub struct UVisual {
    __padding_end: [u8; 48],
}
impl UVisual {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS.wait().name_to_ptr.get("UVisual").unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UWidget {
    #[doc(hidden)]
    __padding_56: [u8; 56],
    pub slot: UPtr<UPanelSlot>,
    #[doc(hidden)]
    __padding_128: [u8; 64],
    pub tool_tip_text: FText,
    pub tool_tip_widget: UPtr<UWidget>,
    #[doc(hidden)]
    __padding_216: [u8; 64],
    pub render_transform: FWidgetTransform,
    pub render_transform_pivot: crate::bindings::core_u_object::FVector2D,
    pub flow_direction_preference: crate::bindings::slate_core::EFlowDirectionPreference,
    pub flags_289: u8,
    #[doc(hidden)]
    __padding_392: [u8; 102],
    pub flags_392: u8,
    pub cursor: crate::bindings::core_u_object::EMouseCursor,
    pub clipping: crate::bindings::slate_core::EWidgetClipping,
    pub visibility: ESlateVisibility,
    pub pixel_snapping: crate::bindings::slate_core::EWidgetPixelSnapping,
    pub render_opacity: f32,
    #[doc(hidden)]
    __padding_416: [u8; 8],
    pub navigation: UPtr<UWidgetNavigation>,
    __padding_end: [u8; 272],
}
impl UWidget {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS.wait().name_to_ptr.get("UWidget").unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
    pub fn verify_layout() {
        log::warn!(
            "{} = {} vs {}", "slot", std::mem::offset_of!(UWidget, slot), 56usize
        );
        log::warn!(
            "{} = {} vs {}", "tool_tip_text", std::mem::offset_of!(UWidget,
            tool_tip_text), 128usize
        );
        log::warn!(
            "{} = {} vs {}", "tool_tip_widget", std::mem::offset_of!(UWidget,
            tool_tip_widget), 144usize
        );
        log::warn!(
            "{} = {} vs {}", "render_transform", std::mem::offset_of!(UWidget,
            render_transform), 216usize
        );
        log::warn!(
            "{} = {} vs {}", "render_transform_pivot", std::mem::offset_of!(UWidget,
            render_transform_pivot), 272usize
        );
        log::warn!(
            "{} = {} vs {}", "flow_direction_preference", std::mem::offset_of!(UWidget,
            flow_direction_preference), 288usize
        );
        log::warn!(
            "{} = {} vs {}", "cursor", std::mem::offset_of!(UWidget, cursor), 393usize
        );
        log::warn!(
            "{} = {} vs {}", "clipping", std::mem::offset_of!(UWidget, clipping),
            394usize
        );
        log::warn!(
            "{} = {} vs {}", "visibility", std::mem::offset_of!(UWidget, visibility),
            395usize
        );
        log::warn!(
            "{} = {} vs {}", "pixel_snapping", std::mem::offset_of!(UWidget,
            pixel_snapping), 396usize
        );
        log::warn!(
            "{} = {} vs {}", "render_opacity", std::mem::offset_of!(UWidget,
            render_opacity), 400usize
        );
        log::warn!(
            "{} = {} vs {}", "navigation", std::mem::offset_of!(UWidget, navigation),
            416usize
        );
    }
}
#[repr(C, align(8))]
pub struct UUserWidget {
    #[doc(hidden)]
    __padding_768: [u8; 768],
    pub color_and_opacity: crate::bindings::core_u_object::FLinearColor,
    #[doc(hidden)]
    __padding_816: [u8; 32],
    pub foreground_color: crate::bindings::slate_core::FSlateColor,
    #[doc(hidden)]
    __padding_960: [u8; 124],
    pub padding: crate::bindings::slate_core::FMargin,
    pub priority: i32,
    pub flags_980: u8,
    #[doc(hidden)]
    __padding_1176: [u8; 195],
    pub tick_frequency: EWidgetTickFrequency,
    __padding_end: [u8; 111],
}
impl UUserWidget {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UUserWidget")
            .unwrap()
    }
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
pub struct UWidgetComponent {
    __padding_end: [u8; 2016],
}
impl UWidgetComponent {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UWidgetComponent")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UUserWidgetBlueprint {
    __padding_end: [u8; 1432],
}
impl UUserWidgetBlueprint {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UUserWidgetBlueprint")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UPanelWidget {
    __padding_end: [u8; 720],
}
impl UPanelWidget {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPanelWidget")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UContentWidget {
    __padding_end: [u8; 720],
}
impl UContentWidget {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UContentWidget")
            .unwrap()
    }
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
pub struct UButton {
    #[doc(hidden)]
    __padding_720: [u8; 720],
    pub widget_style: crate::bindings::slate_core::FButtonStyle,
    pub color_and_opacity: crate::bindings::core_u_object::FLinearColor,
    pub background_color: crate::bindings::core_u_object::FLinearColor,
    pub click_method: crate::bindings::slate_core::EButtonClickMethod,
    pub touch_method: crate::bindings::slate_core::EButtonTouchMethod,
    pub press_method: crate::bindings::slate_core::EButtonPressMethod,
    pub is_focusable: bool,
    #[doc(hidden)]
    __padding_2152: [u8; 308],
    pub b_allow_drag_drop: bool,
    __padding_end: [u8; 7],
}
impl UButton {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS.wait().name_to_ptr.get("UButton").unwrap()
    }
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
pub struct UCheckBox {
    #[doc(hidden)]
    __padding_720: [u8; 720],
    pub checked_state: crate::bindings::slate_core::ECheckBoxState,
    #[doc(hidden)]
    __padding_768: [u8; 32],
    pub widget_style: crate::bindings::slate_core::FCheckBoxStyle,
    pub horizontal_alignment: crate::bindings::slate_core::EHorizontalAlignment,
    pub click_method: crate::bindings::slate_core::EButtonClickMethod,
    pub touch_method: crate::bindings::slate_core::EButtonTouchMethod,
    pub press_method: crate::bindings::slate_core::EButtonPressMethod,
    pub is_focusable: bool,
    __padding_end: [u8; 59],
}
impl UCheckBox {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UCheckBox")
            .unwrap()
    }
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
pub struct UCircularThrobber {
    #[doc(hidden)]
    __padding_696: [u8; 696],
    pub number_of_pieces: i32,
    pub period: f32,
    pub radius: f32,
    pub image: crate::bindings::slate_core::FSlateBrush,
    __padding_end: [u8; 32],
}
impl UCircularThrobber {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UCircularThrobber")
            .unwrap()
    }
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
pub struct UComboBoxKey {
    #[doc(hidden)]
    __padding_736: [u8; 736],
    pub widget_style: crate::bindings::slate_core::FComboBoxStyle,
    pub item_style: crate::bindings::slate_core::FTableRowStyle,
    pub scroll_bar_style: crate::bindings::slate_core::FScrollBarStyle,
    pub foreground_color: crate::bindings::slate_core::FSlateColor,
    pub content_padding: crate::bindings::slate_core::FMargin,
    pub max_list_height: f32,
    pub b_has_down_arrow: bool,
    pub b_enable_gamepad_navigation_mode: bool,
    pub b_is_focusable: bool,
    __padding_end: [u8; 149],
}
impl UComboBoxKey {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UComboBoxKey")
            .unwrap()
    }
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
pub struct UComboBoxString {
    #[doc(hidden)]
    __padding_736: [u8; 736],
    pub widget_style: crate::bindings::slate_core::FComboBoxStyle,
    pub item_style: crate::bindings::slate_core::FTableRowStyle,
    pub scroll_bar_style: crate::bindings::slate_core::FScrollBarStyle,
    pub content_padding: crate::bindings::slate_core::FMargin,
    pub max_list_height: f32,
    pub has_down_arrow: bool,
    pub enable_gamepad_navigation_mode: bool,
    pub font: crate::bindings::slate_core::FSlateFontInfo,
    pub foreground_color: crate::bindings::slate_core::FSlateColor,
    pub b_is_focusable: bool,
    __padding_end: [u8; 163],
}
impl UComboBoxString {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UComboBoxString")
            .unwrap()
    }
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
pub struct UEditableText {
    #[doc(hidden)]
    __padding_696: [u8; 696],
    pub text: FText,
    #[doc(hidden)]
    __padding_744: [u8; 32],
    pub hint_text: FText,
    #[doc(hidden)]
    __padding_800: [u8; 32],
    pub widget_style: crate::bindings::slate_core::FEditableTextStyle,
    pub is_read_only: bool,
    pub is_password: bool,
    pub minimum_desired_width: f32,
    pub is_caret_moved_when_gain_focus: bool,
    pub select_all_text_when_focused: bool,
    pub revert_text_on_escape: bool,
    pub clear_keyboard_focus_on_commit: bool,
    pub select_all_text_on_commit: bool,
    #[doc(hidden)]
    __padding_1586: [u8; 5],
    pub justification: crate::bindings::slate::ETextJustify,
    pub overflow_policy: crate::bindings::slate_core::ETextOverflowPolicy,
    pub shaped_text_options: FShapedTextOptions,
    #[doc(hidden)]
    __padding_1688: [u8; 97],
    pub enable_integrated_keyboard: bool,
    __padding_end: [u8; 7],
}
impl UEditableText {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEditableText")
            .unwrap()
    }
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
pub struct UEditableTextBox {
    #[doc(hidden)]
    __padding_696: [u8; 696],
    pub text: FText,
    #[doc(hidden)]
    __padding_752: [u8; 32],
    pub widget_style: crate::bindings::slate_core::FEditableTextBoxStyle,
    pub hint_text: FText,
    #[doc(hidden)]
    __padding_4640: [u8; 32],
    pub is_read_only: bool,
    pub is_password: bool,
    pub minimum_desired_width: f32,
    pub is_caret_moved_when_gain_focus: bool,
    pub select_all_text_when_focused: bool,
    pub revert_text_on_escape: bool,
    pub clear_keyboard_focus_on_commit: bool,
    pub select_all_text_on_commit: bool,
    #[doc(hidden)]
    __padding_4658: [u8; 5],
    pub justification: crate::bindings::slate::ETextJustify,
    pub overflow_policy: crate::bindings::slate_core::ETextOverflowPolicy,
    pub shaped_text_options: FShapedTextOptions,
    __padding_end: [u8; 105],
}
impl UEditableTextBox {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEditableTextBox")
            .unwrap()
    }
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
pub struct UExpandableArea {
    #[doc(hidden)]
    __padding_1152: [u8; 1152],
    pub border_brush: crate::bindings::slate_core::FSlateBrush,
    pub border_color: crate::bindings::slate_core::FSlateColor,
    pub b_is_expanded: bool,
    pub max_height: f32,
    pub header_padding: crate::bindings::slate_core::FMargin,
    pub area_padding: crate::bindings::slate_core::FMargin,
    __padding_end: [u8; 68],
}
impl UExpandableArea {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UExpandableArea")
            .unwrap()
    }
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
pub struct UInputKeySelector {
    #[doc(hidden)]
    __padding_704: [u8; 704],
    pub widget_style: crate::bindings::slate_core::FButtonStyle,
    pub text_style: crate::bindings::slate_core::FTextBlockStyle,
    pub selected_key: crate::bindings::slate::FInputChord,
    pub margin: crate::bindings::slate_core::FMargin,
    pub key_selection_text: FText,
    pub no_key_specified_text: FText,
    pub b_allow_modifier_keys: bool,
    pub b_allow_gamepad_keys: bool,
    pub escape_keys: TArray<crate::bindings::input_core::FKey>,
    __padding_end: [u8; 64],
}
impl UInputKeySelector {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UInputKeySelector")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UListViewBase {
    #[doc(hidden)]
    __padding_760: [u8; 760],
    pub entry_widget_class: TSubclassOf<UUserWidget>,
    pub wheel_scroll_multiplier: f32,
    pub b_enable_scroll_animation: bool,
    pub scrolling_animation_interpolation_speed: f32,
    pub b_in_enable_touch_animated_scrolling: bool,
    #[doc(hidden)]
    __padding_792: [u8; 11],
    pub b_allow_dragging: bool,
    pub b_allow_drag_drop: bool,
    pub drag_drop_visual_pivot: EDragPivot,
    pub drag_drop_visual_offset: crate::bindings::core_u_object::FVector2D,
    pub drag_drop_visual_entry_class: TSubclassOf<UUserWidget>,
    pub drag_drop_operation_class: TSubclassOf<UDragDropOperation>,
    pub drag_visual_widget: UPtr<UWidget>,
    pub b_is_dragging: bool,
    pub b_select_item_on_navigation: bool,
    pub b_allow_keep_preselected_items: bool,
    __padding_end: [u8; 269],
}
impl UListViewBase {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UListViewBase")
            .unwrap()
    }
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
pub struct UListView {
    #[doc(hidden)]
    __padding_1616: [u8; 1616],
    pub widget_style: crate::bindings::slate_core::FTableViewStyle,
    pub scroll_bar_style: crate::bindings::slate_core::FScrollBarStyle,
    pub b_enable_shadow_brush: bool,
    pub shadow_brush_style: crate::bindings::slate_core::FScrollBoxStyle,
    pub orientation: crate::bindings::slate_core::EOrientation,
    pub selection_mode: crate::bindings::slate::ESelectionMode,
    pub consume_mouse_wheel: crate::bindings::slate_core::EConsumeMouseWheel,
    pub b_clear_selection_on_click: bool,
    pub b_is_focusable: bool,
    pub b_clear_scroll_velocity_on_selection: bool,
    pub b_return_focus_to_selection: bool,
    pub scroll_into_view_alignment: crate::bindings::slate::EScrollIntoViewAlignment,
    #[doc(hidden)]
    __padding_4680: [u8; 32],
    pub entry_spacing: f32,
    pub horizontal_entry_spacing: f32,
    pub vertical_entry_spacing: f32,
    pub scroll_bar_padding: crate::bindings::slate_core::FMargin,
    #[doc(hidden)]
    __padding_5048: [u8; 336],
    pub bp_on_is_item_selectable_or_navigable: FListView_BP_OnIsItemSelectableOrNavigable,
    __padding_end: [u8; 8],
}
impl UListView {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UListView")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UTextLayoutWidget {
    #[doc(hidden)]
    __padding_696: [u8; 696],
    pub shaped_text_options: FShapedTextOptions,
    pub justification: crate::bindings::slate::ETextJustify,
    pub wrapping_policy: crate::bindings::slate::ETextWrappingPolicy,
    pub flags_701: u8,
    pub apply_line_height_to_bottom_line: bool,
    pub wrap_text_at: f32,
    pub margin: crate::bindings::slate_core::FMargin,
    pub line_height_percentage: f32,
}
impl UTextLayoutWidget {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UTextLayoutWidget")
            .unwrap()
    }
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
pub struct UMultiLineEditableText {
    #[doc(hidden)]
    __padding_728: [u8; 728],
    pub text: FText,
    pub hint_text: FText,
    #[doc(hidden)]
    __padding_800: [u8; 32],
    pub widget_style: crate::bindings::slate_core::FTextBlockStyle,
    pub b_is_read_only: bool,
    pub select_all_text_when_focused: bool,
    #[doc(hidden)]
    __padding_1651: [u8; 1],
    pub revert_text_on_escape: bool,
    pub clear_keyboard_focus_on_commit: bool,
    __padding_end: [u8; 91],
}
impl UMultiLineEditableText {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMultiLineEditableText")
            .unwrap()
    }
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
pub struct UMultiLineEditableTextBox {
    #[doc(hidden)]
    __padding_728: [u8; 728],
    pub text: FText,
    pub hint_text: FText,
    #[doc(hidden)]
    __padding_800: [u8; 32],
    pub widget_style: crate::bindings::slate_core::FEditableTextBoxStyle,
    #[doc(hidden)]
    __padding_5488: [u8; 848],
    pub b_is_read_only: bool,
    __padding_end: [u8; 95],
}
impl UMultiLineEditableTextBox {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMultiLineEditableTextBox")
            .unwrap()
    }
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
pub struct UProgressBar {
    #[doc(hidden)]
    __padding_704: [u8; 704],
    pub widget_style: crate::bindings::slate_core::FProgressBarStyle,
    pub percent: f32,
    pub bar_fill_type: crate::bindings::slate::EProgressBarFillType,
    pub bar_fill_style: crate::bindings::slate::EProgressBarFillStyle,
    pub b_is_marquee: bool,
    pub border_padding: crate::bindings::core_u_object::FVector2D,
    #[doc(hidden)]
    __padding_1416: [u8; 32],
    pub fill_color_and_opacity: crate::bindings::core_u_object::FLinearColor,
    __padding_end: [u8; 72],
}
impl UProgressBar {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UProgressBar")
            .unwrap()
    }
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
pub struct UScrollBar {
    #[doc(hidden)]
    __padding_704: [u8; 704],
    pub widget_style: crate::bindings::slate_core::FScrollBarStyle,
    pub b_always_show_scrollbar: bool,
    pub b_always_show_scrollbar_track: bool,
    pub orientation: crate::bindings::slate_core::EOrientation,
    pub thickness: crate::bindings::core_u_object::FVector2D,
    pub padding: crate::bindings::slate_core::FMargin,
    __padding_end: [u8; 24],
}
impl UScrollBar {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UScrollBar")
            .unwrap()
    }
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
pub struct UScrollBox {
    #[doc(hidden)]
    __padding_720: [u8; 720],
    pub scroll_animation_interpolation_speed: f32,
    pub b_enable_touch_scrolling: bool,
    pub b_consume_pointer_input: bool,
    pub analog_mouse_wheel_key: crate::bindings::input_core::FKey,
    pub b_is_focusable: bool,
    pub widget_style: crate::bindings::slate_core::FScrollBoxStyle,
    pub widget_bar_style: crate::bindings::slate_core::FScrollBarStyle,
    pub orientation: crate::bindings::slate_core::EOrientation,
    pub scroll_bar_visibility: ESlateVisibility,
    pub consume_mouse_wheel: crate::bindings::slate_core::EConsumeMouseWheel,
    pub scrollbar_thickness: crate::bindings::core_u_object::FVector2D,
    pub scrollbar_padding: crate::bindings::slate_core::FMargin,
    pub always_show_scrollbar: bool,
    pub always_show_scrollbar_track: bool,
    pub allow_overscroll: bool,
    pub back_pad_scrolling: bool,
    pub front_pad_scrolling: bool,
    pub b_animate_wheel_scrolling: bool,
    pub navigation_destination: crate::bindings::slate::EDescendantScrollDestination,
    pub navigation_scroll_padding: f32,
    pub scroll_when_focus_changes: crate::bindings::slate::EScrollWhenFocusChanges,
    pub b_allow_right_click_drag_scrolling: bool,
    pub wheel_scroll_multiplier: f32,
    __padding_end: [u8; 148],
}
impl UScrollBox {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UScrollBox")
            .unwrap()
    }
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
pub struct USlider {
    #[doc(hidden)]
    __padding_696: [u8; 696],
    pub value: f32,
    #[doc(hidden)]
    __padding_736: [u8; 36],
    pub min_value: f32,
    pub max_value: f32,
    pub widget_style: crate::bindings::slate_core::FSliderStyle,
    pub orientation: crate::bindings::slate_core::EOrientation,
    pub slider_bar_color: crate::bindings::core_u_object::FLinearColor,
    pub slider_handle_color: crate::bindings::core_u_object::FLinearColor,
    pub indent_handle: bool,
    pub locked: bool,
    pub mouse_uses_step: bool,
    pub requires_controller_lock: bool,
    pub step_size: f32,
    pub is_focusable: bool,
    pub b_prevent_throttling: bool,
    __padding_end: [u8; 146],
}
impl USlider {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS.wait().name_to_ptr.get("USlider").unwrap()
    }
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
pub struct USpinBox {
    #[doc(hidden)]
    __padding_696: [u8; 696],
    pub value: f32,
    #[doc(hidden)]
    __padding_736: [u8; 32],
    pub widget_style: crate::bindings::slate_core::FSpinBoxStyle,
    pub min_fractional_digits: i32,
    pub max_fractional_digits: i32,
    pub b_always_uses_delta_snap: bool,
    pub b_enable_slider: bool,
    pub delta: f32,
    pub slider_exponent: f32,
    pub font: crate::bindings::slate_core::FSlateFontInfo,
    pub justification: crate::bindings::slate::ETextJustify,
    pub min_desired_width: f32,
    #[doc(hidden)]
    __padding_2417: [u8; 1],
    pub clear_keyboard_focus_on_commit: bool,
    pub select_all_text_on_commit: bool,
    pub foreground_color: crate::bindings::slate_core::FSlateColor,
    #[doc(hidden)]
    __padding_2540: [u8; 100],
    pub min_value: f32,
    pub max_value: f32,
    pub min_slider_value: f32,
    pub max_slider_value: f32,
    __padding_end: [u8; 36],
}
impl USpinBox {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS.wait().name_to_ptr.get("USpinBox").unwrap()
    }
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
pub struct UThrobber {
    #[doc(hidden)]
    __padding_696: [u8; 696],
    pub number_of_pieces: i32,
    pub b_animate_horizontally: bool,
    pub b_animate_vertically: bool,
    pub b_animate_opacity: bool,
    pub image: crate::bindings::slate_core::FSlateBrush,
    __padding_end: [u8; 16],
}
impl UThrobber {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UThrobber")
            .unwrap()
    }
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
pub struct UTreeView {
    __padding_end: [u8; 5216],
}
impl UTreeView {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UTreeView")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UListViewDesignerPreviewItem {
    __padding_end: [u8; 48],
}
impl UListViewDesignerPreviewItem {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UListViewDesignerPreviewItem")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct USlateAccessibleWidgetData {
    __padding_end: [u8; 152],
}
impl USlateAccessibleWidgetData {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("USlateAccessibleWidgetData")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UUserWidgetExtension {
    __padding_end: [u8; 48],
}
impl UUserWidgetExtension {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UUserWidgetExtension")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UWidgetBlueprintGeneratedClassExtension {
    __padding_end: [u8; 48],
}
impl UWidgetBlueprintGeneratedClassExtension {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UWidgetBlueprintGeneratedClassExtension")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UWidgetFieldNotificationExtension {
    __padding_end: [u8; 72],
}
impl UWidgetFieldNotificationExtension {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UWidgetFieldNotificationExtension")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UWidgetNavigation {
    #[doc(hidden)]
    __padding_48: [u8; 48],
    pub up: FWidgetNavigationData,
    pub down: FWidgetNavigationData,
    pub left: FWidgetNavigationData,
    pub right: FWidgetNavigationData,
    pub next: FWidgetNavigationData,
    pub previous: FWidgetNavigationData,
    pub routing_policy: crate::bindings::slate_core::EWidgetNavigationRoutingPolicy,
    pub navigation_method: crate::bindings::core_u_object::FInstancedStruct,
}
impl UWidgetNavigation {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UWidgetNavigation")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UMovieScene2DTransformPropertySystem {
    __padding_end: [u8; 104],
}
impl UMovieScene2DTransformPropertySystem {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieScene2DTransformPropertySystem")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UMovieScene2DTransformSection {
    __padding_end: [u8; 2504],
}
impl UMovieScene2DTransformSection {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieScene2DTransformSection")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UMovieScene2DTransformTrack {
    __padding_end: [u8; 480],
}
impl UMovieScene2DTransformTrack {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieScene2DTransformTrack")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UMovieSceneMarginPropertySystem {
    __padding_end: [u8; 104],
}
impl UMovieSceneMarginPropertySystem {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieSceneMarginPropertySystem")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UMovieSceneMarginSection {
    __padding_end: [u8; 1584],
}
impl UMovieSceneMarginSection {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieSceneMarginSection")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UMovieSceneMarginTrack {
    __padding_end: [u8; 480],
}
impl UMovieSceneMarginTrack {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieSceneMarginTrack")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UMovieSceneWidgetMaterialSystem {
    __padding_end: [u8; 536],
}
impl UMovieSceneWidgetMaterialSystem {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieSceneWidgetMaterialSystem")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UMovieSceneWidgetMaterialTrack {
    __padding_end: [u8; 456],
}
impl UMovieSceneWidgetMaterialTrack {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieSceneWidgetMaterialTrack")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UUMGSequencePlayer {
    __padding_end: [u8; 624],
}
impl UUMGSequencePlayer {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UUMGSequencePlayer")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UUMGSequenceTickManager {
    __padding_end: [u8; 216],
}
impl UUMGSequenceTickManager {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UUMGSequenceTickManager")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UWidgetAnimation {
    __padding_end: [u8; 176],
}
impl UWidgetAnimation {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UWidgetAnimation")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UWidgetAnimationDelegateBinding {
    __padding_end: [u8; 64],
}
impl UWidgetAnimationDelegateBinding {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UWidgetAnimationDelegateBinding")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UWidgetAnimationHandleFunctionLibrary {
    __padding_end: [u8; 48],
}
impl UWidgetAnimationHandleFunctionLibrary {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UWidgetAnimationHandleFunctionLibrary")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UWidgetAnimationPlayCallbackProxy {
    __padding_end: [u8; 96],
}
impl UWidgetAnimationPlayCallbackProxy {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UWidgetAnimationPlayCallbackProxy")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UPropertyBinding {
    __padding_end: [u8; 128],
}
impl UPropertyBinding {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPropertyBinding")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UBoolBinding {
    __padding_end: [u8; 128],
}
impl UBoolBinding {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBoolBinding")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UBrushBinding {
    __padding_end: [u8; 136],
}
impl UBrushBinding {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBrushBinding")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UCheckedStateBinding {
    __padding_end: [u8; 136],
}
impl UCheckedStateBinding {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UCheckedStateBinding")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UColorBinding {
    __padding_end: [u8; 136],
}
impl UColorBinding {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UColorBinding")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UFloatBinding {
    __padding_end: [u8; 128],
}
impl UFloatBinding {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UFloatBinding")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UInt32Binding {
    __padding_end: [u8; 128],
}
impl UInt32Binding {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UInt32Binding")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UMouseCursorBinding {
    __padding_end: [u8; 128],
}
impl UMouseCursorBinding {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMouseCursorBinding")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UWidgetBinaryStateRegistration {
    __padding_end: [u8; 48],
}
impl UWidgetBinaryStateRegistration {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UWidgetBinaryStateRegistration")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UWidgetHoveredStateRegistration {
    __padding_end: [u8; 48],
}
impl UWidgetHoveredStateRegistration {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UWidgetHoveredStateRegistration")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UWidgetPressedStateRegistration {
    __padding_end: [u8; 48],
}
impl UWidgetPressedStateRegistration {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UWidgetPressedStateRegistration")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UWidgetDisabledStateRegistration {
    __padding_end: [u8; 48],
}
impl UWidgetDisabledStateRegistration {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UWidgetDisabledStateRegistration")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UWidgetSelectedStateRegistration {
    __padding_end: [u8; 48],
}
impl UWidgetSelectedStateRegistration {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UWidgetSelectedStateRegistration")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UWidgetEnumStateRegistration {
    __padding_end: [u8; 48],
}
impl UWidgetEnumStateRegistration {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UWidgetEnumStateRegistration")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UWidgetStateSettings {
    __padding_end: [u8; 2136],
}
impl UWidgetStateSettings {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UWidgetStateSettings")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UTextBinding {
    __padding_end: [u8; 136],
}
impl UTextBinding {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UTextBinding")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UVisibilityBinding {
    __padding_end: [u8; 128],
}
impl UVisibilityBinding {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UVisibilityBinding")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UWidgetBinding {
    __padding_end: [u8; 128],
}
impl UWidgetBinding {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UWidgetBinding")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UAsyncTaskDownloadImage {
    __padding_end: [u8; 104],
}
impl UAsyncTaskDownloadImage {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAsyncTaskDownloadImage")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UGameViewportSubsystem {
    __padding_end: [u8; 200],
}
impl UGameViewportSubsystem {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UGameViewportSubsystem")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
pub struct IserListEntry {}
#[repr(C, align(8))]
pub struct UUserListEntry {
    __padding_end: [u8; 48],
}
impl UUserListEntry {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UUserListEntry")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UUserListEntryLibrary {
    __padding_end: [u8; 48],
}
impl UUserListEntryLibrary {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UUserListEntryLibrary")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
pub struct IserObjectListEntry {}
#[repr(C, align(8))]
pub struct UUserObjectListEntry {
    __padding_end: [u8; 48],
}
impl UUserObjectListEntry {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UUserObjectListEntry")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UUserObjectListEntryLibrary {
    __padding_end: [u8; 48],
}
impl UUserObjectListEntryLibrary {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UUserObjectListEntryLibrary")
            .unwrap()
    }
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
pub struct UBackgroundBlur {
    #[doc(hidden)]
    __padding_720: [u8; 720],
    pub padding: crate::bindings::slate_core::FMargin,
    pub horizontal_alignment: crate::bindings::slate_core::EHorizontalAlignment,
    pub vertical_alignment: crate::bindings::slate_core::EVerticalAlignment,
    pub b_apply_alpha_to_blur: bool,
    pub blur_strength: f32,
    #[doc(hidden)]
    __padding_748: [u8; 4],
    pub blur_radius: i32,
    pub corner_radius: crate::bindings::core_u_object::FVector4,
    pub low_quality_fallback_brush: crate::bindings::slate_core::FSlateBrush,
    __padding_end: [u8; 16],
}
impl UBackgroundBlur {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBackgroundBlur")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UPanelSlot {
    __padding_end: [u8; 64],
}
impl UPanelSlot {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPanelSlot")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UBackgroundBlurSlot {
    #[doc(hidden)]
    __padding_64: [u8; 64],
    pub padding: crate::bindings::slate_core::FMargin,
    pub horizontal_alignment: crate::bindings::slate_core::EHorizontalAlignment,
    pub vertical_alignment: crate::bindings::slate_core::EVerticalAlignment,
    __padding_end: [u8; 22],
}
impl UBackgroundBlurSlot {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBackgroundBlurSlot")
            .unwrap()
    }
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
pub struct UBorder {
    #[doc(hidden)]
    __padding_720: [u8; 720],
    pub horizontal_alignment: crate::bindings::slate_core::EHorizontalAlignment,
    pub vertical_alignment: crate::bindings::slate_core::EVerticalAlignment,
    pub flags_722: u8,
    pub content_color_and_opacity: crate::bindings::core_u_object::FLinearColor,
    #[doc(hidden)]
    __padding_776: [u8; 36],
    pub padding: crate::bindings::slate_core::FMargin,
    pub background: crate::bindings::slate_core::FSlateBrush,
    #[doc(hidden)]
    __padding_1040: [u8; 32],
    pub brush_color: crate::bindings::core_u_object::FLinearColor,
    #[doc(hidden)]
    __padding_1088: [u8; 32],
    pub desired_size_scale: crate::bindings::core_u_object::FVector2D,
    __padding_end: [u8; 176],
}
impl UBorder {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS.wait().name_to_ptr.get("UBorder").unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UBorderSlot {
    #[doc(hidden)]
    __padding_64: [u8; 64],
    pub padding: crate::bindings::slate_core::FMargin,
    pub horizontal_alignment: crate::bindings::slate_core::EHorizontalAlignment,
    pub vertical_alignment: crate::bindings::slate_core::EVerticalAlignment,
    __padding_end: [u8; 22],
}
impl UBorderSlot {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBorderSlot")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UButtonSlot {
    #[doc(hidden)]
    __padding_64: [u8; 64],
    pub padding: crate::bindings::slate_core::FMargin,
    pub horizontal_alignment: crate::bindings::slate_core::EHorizontalAlignment,
    pub vertical_alignment: crate::bindings::slate_core::EVerticalAlignment,
    __padding_end: [u8; 22],
}
impl UButtonSlot {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UButtonSlot")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UCanvasPanel {
    __padding_end: [u8; 736],
}
impl UCanvasPanel {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UCanvasPanel")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UCanvasPanelSlot {
    #[doc(hidden)]
    __padding_64: [u8; 64],
    pub layout_data: FAnchorData,
    pub b_auto_size: bool,
    pub z_order: i32,
    __padding_end: [u8; 152],
}
impl UCanvasPanelSlot {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UCanvasPanelSlot")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UWidgetCheckedStateRegistration {
    __padding_end: [u8; 48],
}
impl UWidgetCheckedStateRegistration {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UWidgetCheckedStateRegistration")
            .unwrap()
    }
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
pub struct UComboBox {
    #[doc(hidden)]
    __padding_704: [u8; 704],
    pub scroll_bar_style: crate::bindings::slate_core::FScrollBarStyle,
    pub items: TArray<UPtr<crate::bindings::core_u_object::UObject>>,
    #[doc(hidden)]
    __padding_2656: [u8; 32],
    pub b_is_focusable: bool,
    __padding_end: [u8; 31],
}
impl UComboBox {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UComboBox")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UDynamicEntryBoxBase {
    #[doc(hidden)]
    __padding_696: [u8; 696],
    pub entry_spacing: crate::bindings::core_u_object::FVector2D,
    pub spacing_pattern: TArray<crate::bindings::core_u_object::FVector2D>,
    pub entry_box_type: EDynamicBoxType,
    pub entry_size_rule: FSlateChildSize,
    pub entry_horizontal_alignment: crate::bindings::slate_core::EHorizontalAlignment,
    pub entry_vertical_alignment: crate::bindings::slate_core::EVerticalAlignment,
    pub max_element_size: i32,
    pub radial_box_settings: FRadialBoxSettings,
    __padding_end: [u8; 156],
}
impl UDynamicEntryBoxBase {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDynamicEntryBoxBase")
            .unwrap()
    }
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
pub struct UDynamicEntryBox {
    #[doc(hidden)]
    __padding_976: [u8; 976],
    pub entry_widget_class: TSubclassOf<UUserWidget>,
    __padding_end: [u8; 8],
}
impl UDynamicEntryBox {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDynamicEntryBox")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UGridPanel {
    #[doc(hidden)]
    __padding_720: [u8; 720],
    pub column_fill: TArray<f32>,
    pub row_fill: TArray<f32>,
    __padding_end: [u8; 16],
}
impl UGridPanel {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UGridPanel")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UGridSlot {
    #[doc(hidden)]
    __padding_64: [u8; 64],
    pub padding: crate::bindings::slate_core::FMargin,
    pub horizontal_alignment: crate::bindings::slate_core::EHorizontalAlignment,
    pub vertical_alignment: crate::bindings::slate_core::EVerticalAlignment,
    pub row: i32,
    pub row_span: i32,
    pub column: i32,
    pub column_span: i32,
    pub layer: i32,
    pub nudge: crate::bindings::core_u_object::FVector2D,
    __padding_end: [u8; 8],
}
impl UGridSlot {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UGridSlot")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UHorizontalBox {
    __padding_end: [u8; 736],
}
impl UHorizontalBox {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UHorizontalBox")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UHorizontalBoxSlot {
    #[doc(hidden)]
    __padding_72: [u8; 72],
    pub size: FSlateChildSize,
    pub padding: crate::bindings::slate_core::FMargin,
    pub horizontal_alignment: crate::bindings::slate_core::EHorizontalAlignment,
    pub vertical_alignment: crate::bindings::slate_core::EVerticalAlignment,
    __padding_end: [u8; 6],
}
impl UHorizontalBoxSlot {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UHorizontalBoxSlot")
            .unwrap()
    }
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
pub struct UImage {
    #[doc(hidden)]
    __padding_704: [u8; 704],
    pub brush: crate::bindings::slate_core::FSlateBrush,
    #[doc(hidden)]
    __padding_944: [u8; 32],
    pub color_and_opacity: crate::bindings::core_u_object::FLinearColor,
    #[doc(hidden)]
    __padding_992: [u8; 32],
    pub b_flip_for_right_to_left_flow_direction: bool,
    __padding_end: [u8; 143],
}
impl UImage {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS.wait().name_to_ptr.get("UImage").unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UInvalidationBox {
    #[doc(hidden)]
    __padding_720: [u8; 720],
    pub b_can_cache: bool,
    __padding_end: [u8; 23],
}
impl UInvalidationBox {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UInvalidationBox")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UMenuAnchor {
    #[doc(hidden)]
    __padding_720: [u8; 720],
    pub menu_class: TSubclassOf<UUserWidget>,
    #[doc(hidden)]
    __padding_792: [u8; 64],
    pub placement: crate::bindings::slate_core::EMenuPlacement,
    pub b_fit_in_window: bool,
    pub should_defer_painting_after_window_content: bool,
    pub use_application_menu_stack: bool,
    pub show_menu_background: bool,
    __padding_end: [u8; 43],
}
impl UMenuAnchor {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMenuAnchor")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UUIComponent {
    __padding_end: [u8; 96],
}
impl UUIComponent {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UUIComponent")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UMouseHoverComponent {
    #[doc(hidden)]
    __padding_96: [u8; 96],
    pub b_is_hovered: bool,
    __padding_end: [u8; 7],
}
impl UMouseHoverComponent {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMouseHoverComponent")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UNamedSlot {
    __padding_end: [u8; 760],
}
impl UNamedSlot {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNamedSlot")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
pub struct INamedSlotInterface {}
#[repr(C, align(8))]
pub struct UNamedSlotInterface {
    __padding_end: [u8; 48],
}
impl UNamedSlotInterface {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNamedSlotInterface")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UNativeWidgetHost {
    __padding_end: [u8; 712],
}
impl UNativeWidgetHost {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNativeWidgetHost")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UOverlay {
    __padding_end: [u8; 736],
}
impl UOverlay {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS.wait().name_to_ptr.get("UOverlay").unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UOverlaySlot {
    #[doc(hidden)]
    __padding_72: [u8; 72],
    pub padding: crate::bindings::slate_core::FMargin,
    pub horizontal_alignment: crate::bindings::slate_core::EHorizontalAlignment,
    pub vertical_alignment: crate::bindings::slate_core::EVerticalAlignment,
    __padding_end: [u8; 6],
}
impl UOverlaySlot {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UOverlaySlot")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct USlatePostBufferProcessorUpdater {
    __padding_end: [u8; 56],
}
impl USlatePostBufferProcessorUpdater {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("USlatePostBufferProcessorUpdater")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UPostBufferBlurUpdater {
    __padding_end: [u8; 64],
}
impl UPostBufferBlurUpdater {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPostBufferBlurUpdater")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UPostBufferUpdate {
    #[doc(hidden)]
    __padding_696: [u8; 696],
    pub b_update_only_paint_area: bool,
    pub b_perform_default_post_buffer_update: bool,
    __padding_end: [u8; 54],
}
impl UPostBufferUpdate {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPostBufferUpdate")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct URetainerBox {
    #[doc(hidden)]
    __padding_720: [u8; 720],
    pub b_retain_render: bool,
    pub render_on_invalidation: bool,
    pub render_on_phase: bool,
    pub phase: i32,
    pub phase_count: i32,
    pub effect_material: UPtr<crate::bindings::engine::UMaterialInterface>,
    pub texture_parameter: FName,
    pub b_show_effects_in_designer: bool,
    __padding_end: [u8; 19],
}
impl URetainerBox {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("URetainerBox")
            .unwrap()
    }
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
pub struct URichTextBlock {
    #[doc(hidden)]
    __padding_728: [u8; 728],
    pub text: FText,
    pub text_style_set: UPtr<crate::bindings::engine::UDataTable>,
    #[doc(hidden)]
    __padding_768: [u8; 16],
    pub default_text_style_override: crate::bindings::slate_core::FTextBlockStyle,
    pub min_desired_width: f32,
    #[doc(hidden)]
    __padding_1621: [u8; 1],
    pub text_transform_policy: crate::bindings::slate_core::ETextTransformPolicy,
    pub text_overflow_policy: crate::bindings::slate_core::ETextOverflowPolicy,
    __padding_end: [u8; 905],
}
impl URichTextBlock {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("URichTextBlock")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct URichTextBlockDecorator {
    __padding_end: [u8; 48],
}
impl URichTextBlockDecorator {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("URichTextBlockDecorator")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct URichTextBlockImageDecorator {
    __padding_end: [u8; 56],
}
impl URichTextBlockImageDecorator {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("URichTextBlockImageDecorator")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct USafeZone {
    #[doc(hidden)]
    __padding_720: [u8; 720],
    pub pad_left: bool,
    pub pad_right: bool,
    pub pad_top: bool,
    pub pad_bottom: bool,
    __padding_end: [u8; 52],
}
impl USafeZone {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("USafeZone")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct USafeZoneSlot {
    #[doc(hidden)]
    __padding_64: [u8; 64],
    pub b_is_title_safe: bool,
    pub safe_area_scale: crate::bindings::slate_core::FMargin,
    pub h_align: crate::bindings::slate_core::EHorizontalAlignment,
    pub v_align: crate::bindings::slate_core::EVerticalAlignment,
    pub padding: crate::bindings::slate_core::FMargin,
    __padding_end: [u8; 16],
}
impl USafeZoneSlot {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("USafeZoneSlot")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UScaleBox {
    #[doc(hidden)]
    __padding_720: [u8; 720],
    pub stretch: crate::bindings::slate::EStretch,
    pub stretch_direction: crate::bindings::slate::EStretchDirection,
    pub user_specified_scale: f32,
    pub ignore_inherited_scale: bool,
    __padding_end: [u8; 47],
}
impl UScaleBox {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UScaleBox")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UScaleBoxComponent {
    #[doc(hidden)]
    __padding_96: [u8; 96],
    pub horizontal_alignment: crate::bindings::slate_core::EHorizontalAlignment,
    pub vertical_alignment: crate::bindings::slate_core::EVerticalAlignment,
    pub stretch: crate::bindings::slate::EStretch,
    pub stretch_direction: crate::bindings::slate::EStretchDirection,
    pub user_specified_scale: f32,
    pub ignore_inherited_scale: bool,
    __padding_end: [u8; 23],
}
impl UScaleBoxComponent {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UScaleBoxComponent")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UScaleBoxSlot {
    #[doc(hidden)]
    __padding_80: [u8; 80],
    pub horizontal_alignment: crate::bindings::slate_core::EHorizontalAlignment,
    pub vertical_alignment: crate::bindings::slate_core::EVerticalAlignment,
    __padding_end: [u8; 22],
}
impl UScaleBoxSlot {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UScaleBoxSlot")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UScrollBoxSlot {
    #[doc(hidden)]
    __padding_64: [u8; 64],
    pub size: FSlateChildSize,
    pub padding: crate::bindings::slate_core::FMargin,
    pub horizontal_alignment: crate::bindings::slate_core::EHorizontalAlignment,
    pub vertical_alignment: crate::bindings::slate_core::EVerticalAlignment,
    __padding_end: [u8; 14],
}
impl UScrollBoxSlot {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UScrollBoxSlot")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct USizeBox {
    #[doc(hidden)]
    __padding_736: [u8; 736],
    pub width_override: f32,
    pub height_override: f32,
    pub min_desired_width: f32,
    pub min_desired_height: f32,
    pub max_desired_width: f32,
    pub max_desired_height: f32,
    pub min_aspect_ratio: f32,
    pub max_aspect_ratio: f32,
    __padding_end: [u8; 8],
}
impl USizeBox {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS.wait().name_to_ptr.get("USizeBox").unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct USizeBoxComponent {
    #[doc(hidden)]
    __padding_96: [u8; 96],
    pub padding: crate::bindings::slate_core::FMargin,
    pub horizontal_alignment: crate::bindings::slate_core::EHorizontalAlignment,
    pub vertical_alignment: crate::bindings::slate_core::EVerticalAlignment,
    pub width_override: f32,
    pub height_override: f32,
    pub min_desired_width: f32,
    pub min_desired_height: f32,
    pub max_desired_width: f32,
    pub max_desired_height: f32,
    pub min_aspect_ratio: f32,
    pub max_aspect_ratio: f32,
    __padding_end: [u8; 20],
}
impl USizeBoxComponent {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("USizeBoxComponent")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct USizeBoxSlot {
    #[doc(hidden)]
    __padding_64: [u8; 64],
    pub padding: crate::bindings::slate_core::FMargin,
    #[doc(hidden)]
    __padding_96: [u8; 16],
    pub horizontal_alignment: crate::bindings::slate_core::EHorizontalAlignment,
    pub vertical_alignment: crate::bindings::slate_core::EVerticalAlignment,
    __padding_end: [u8; 6],
}
impl USizeBoxSlot {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("USizeBoxSlot")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct USpacer {
    #[doc(hidden)]
    __padding_696: [u8; 696],
    pub size: crate::bindings::core_u_object::FVector2D,
    __padding_end: [u8; 16],
}
impl USpacer {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS.wait().name_to_ptr.get("USpacer").unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UStackBox {
    #[doc(hidden)]
    __padding_720: [u8; 720],
    pub orientation: crate::bindings::slate_core::EOrientation,
    __padding_end: [u8; 23],
}
impl UStackBox {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UStackBox")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UStackBoxSlot {
    #[doc(hidden)]
    __padding_64: [u8; 64],
    pub padding: crate::bindings::slate_core::FMargin,
    pub size: FSlateChildSize,
    pub horizontal_alignment: crate::bindings::slate_core::EHorizontalAlignment,
    pub vertical_alignment: crate::bindings::slate_core::EVerticalAlignment,
    __padding_end: [u8; 14],
}
impl UStackBoxSlot {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UStackBoxSlot")
            .unwrap()
    }
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
pub struct UTextBlock {
    #[doc(hidden)]
    __padding_728: [u8; 728],
    pub text: FText,
    #[doc(hidden)]
    __padding_776: [u8; 32],
    pub color_and_opacity: crate::bindings::slate_core::FSlateColor,
    #[doc(hidden)]
    __padding_832: [u8; 36],
    pub min_desired_width: f32,
    pub font: crate::bindings::slate_core::FSlateFontInfo,
    pub strike_brush: crate::bindings::slate_core::FSlateBrush,
    pub shadow_offset: crate::bindings::core_u_object::FVector2D,
    pub shadow_color_and_opacity: crate::bindings::core_u_object::FLinearColor,
    #[doc(hidden)]
    __padding_1232: [u8; 32],
    pub b_wrap_with_invalidation_panel: bool,
    pub text_transform_policy: crate::bindings::slate_core::ETextTransformPolicy,
    pub text_overflow_policy: crate::bindings::slate_core::ETextOverflowPolicy,
    pub b_simple_text_mode: bool,
    __padding_end: [u8; 76],
}
impl UTextBlock {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UTextBlock")
            .unwrap()
    }
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
pub struct UTileView {
    __padding_end: [u8; 5120],
}
impl UTileView {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UTileView")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UUniformGridPanel {
    #[doc(hidden)]
    __padding_720: [u8; 720],
    pub slot_padding: crate::bindings::slate_core::FMargin,
    pub min_desired_slot_width: f32,
    pub min_desired_slot_height: f32,
    __padding_end: [u8; 16],
}
impl UUniformGridPanel {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UUniformGridPanel")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UUniformGridSlot {
    #[doc(hidden)]
    __padding_64: [u8; 64],
    pub horizontal_alignment: crate::bindings::slate_core::EHorizontalAlignment,
    pub vertical_alignment: crate::bindings::slate_core::EVerticalAlignment,
    pub row: i32,
    pub column: i32,
    __padding_end: [u8; 12],
}
impl UUniformGridSlot {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UUniformGridSlot")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UVerticalBox {
    __padding_end: [u8; 736],
}
impl UVerticalBox {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UVerticalBox")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UVerticalBoxSlot {
    #[doc(hidden)]
    __padding_64: [u8; 64],
    pub size: FSlateChildSize,
    pub padding: crate::bindings::slate_core::FMargin,
    pub horizontal_alignment: crate::bindings::slate_core::EHorizontalAlignment,
    pub vertical_alignment: crate::bindings::slate_core::EVerticalAlignment,
    __padding_end: [u8; 14],
}
impl UVerticalBoxSlot {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UVerticalBoxSlot")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UViewport {
    #[doc(hidden)]
    __padding_720: [u8; 720],
    pub background_color: crate::bindings::core_u_object::FLinearColor,
    __padding_end: [u8; 80],
}
impl UViewport {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UViewport")
            .unwrap()
    }
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
pub struct UWidgetInteractionComponent {
    #[doc(hidden)]
    __padding_696: [u8; 696],
    pub virtual_user_index: i32,
    pub pointer_index: i32,
    pub trace_channel: crate::bindings::engine::ECollisionChannel,
    pub interaction_distance: f32,
    pub interaction_source: EWidgetInteractionSource,
    pub b_enable_hit_testing: bool,
    pub b_show_debug: bool,
    pub debug_sphere_line_thickness: f32,
    pub debug_line_thickness: f32,
    pub debug_color: crate::bindings::core_u_object::FLinearColor,
    __padding_end: [u8; 716],
}
impl UWidgetInteractionComponent {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UWidgetInteractionComponent")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UWidgetSwitcher {
    #[doc(hidden)]
    __padding_720: [u8; 720],
    pub active_widget_index: i32,
    __padding_end: [u8; 20],
}
impl UWidgetSwitcher {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UWidgetSwitcher")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UWidgetSwitcherSlot {
    #[doc(hidden)]
    __padding_72: [u8; 72],
    pub padding: crate::bindings::slate_core::FMargin,
    pub horizontal_alignment: crate::bindings::slate_core::EHorizontalAlignment,
    pub vertical_alignment: crate::bindings::slate_core::EVerticalAlignment,
    __padding_end: [u8; 6],
}
impl UWidgetSwitcherSlot {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UWidgetSwitcherSlot")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UWindowTitleBarArea {
    #[doc(hidden)]
    __padding_720: [u8; 720],
    pub b_window_buttons_enabled: bool,
    pub b_double_click_toggles_fullscreen: bool,
    __padding_end: [u8; 30],
}
impl UWindowTitleBarArea {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UWindowTitleBarArea")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UWindowTitleBarAreaSlot {
    #[doc(hidden)]
    __padding_64: [u8; 64],
    pub padding: crate::bindings::slate_core::FMargin,
    pub horizontal_alignment: crate::bindings::slate_core::EHorizontalAlignment,
    pub vertical_alignment: crate::bindings::slate_core::EVerticalAlignment,
    __padding_end: [u8; 22],
}
impl UWindowTitleBarAreaSlot {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UWindowTitleBarAreaSlot")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UWrapBox {
    #[doc(hidden)]
    __padding_720: [u8; 720],
    pub inner_slot_padding: crate::bindings::core_u_object::FVector2D,
    pub wrap_size: f32,
    pub b_explicit_wrap_size: bool,
    pub horizontal_alignment: crate::bindings::slate_core::EHorizontalAlignment,
    pub orientation: crate::bindings::slate_core::EOrientation,
    __padding_end: [u8; 17],
}
impl UWrapBox {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS.wait().name_to_ptr.get("UWrapBox").unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UWrapBoxSlot {
    #[doc(hidden)]
    __padding_64: [u8; 64],
    pub padding: crate::bindings::slate_core::FMargin,
    pub fill_span_when_less_than: f32,
    pub horizontal_alignment: crate::bindings::slate_core::EHorizontalAlignment,
    pub vertical_alignment: crate::bindings::slate_core::EVerticalAlignment,
    pub b_fill_empty_space: bool,
    pub b_force_new_line: bool,
    __padding_end: [u8; 8],
}
impl UWrapBoxSlot {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UWrapBoxSlot")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UDragDropOperation {
    #[doc(hidden)]
    __padding_48: [u8; 48],
    pub tag: FString,
    pub payload: UPtr<crate::bindings::core_u_object::UObject>,
    pub default_drag_visual: UPtr<UWidget>,
    pub pivot: EDragPivot,
    pub offset: crate::bindings::core_u_object::FVector2D,
    __padding_end: [u8; 72],
}
impl UDragDropOperation {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDragDropOperation")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UUIComponentContainer {
    __padding_end: [u8; 64],
}
impl UUIComponentContainer {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UUIComponentContainer")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UNavigationUIComponent {
    __padding_end: [u8; 184],
}
impl UNavigationUIComponent {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNavigationUIComponent")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UUIComponentUserWidgetExtension {
    __padding_end: [u8; 64],
}
impl UUIComponentUserWidgetExtension {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UUIComponentUserWidgetExtension")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UUIComponentWidgetBlueprintGeneratedClassExtension {
    __padding_end: [u8; 56],
}
impl UUIComponentWidgetBlueprintGeneratedClassExtension {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UUIComponentWidgetBlueprintGeneratedClassExtension")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct USlateBlueprintLibrary {
    __padding_end: [u8; 48],
}
impl USlateBlueprintLibrary {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("USlateBlueprintLibrary")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct USlateVectorArtData {
    __padding_end: [u8; 136],
}
impl USlateVectorArtData {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("USlateVectorArtData")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UUserWidgetFunctionLibrary {
    __padding_end: [u8; 48],
}
impl UUserWidgetFunctionLibrary {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UUserWidgetFunctionLibrary")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UWidgetBlueprintGeneratedClass {
    __padding_end: [u8; 1992],
}
impl UWidgetBlueprintGeneratedClass {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UWidgetBlueprintGeneratedClass")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UWidgetBlueprintLibrary {
    __padding_end: [u8; 48],
}
impl UWidgetBlueprintLibrary {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UWidgetBlueprintLibrary")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UWidgetLayoutLibrary {
    __padding_end: [u8; 48],
}
impl UWidgetLayoutLibrary {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UWidgetLayoutLibrary")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UWidgetTree {
    __padding_end: [u8; 160],
}
impl UWidgetTree {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UWidgetTree")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct FWidgetNavigationData_CustomDelegate {
    _opague: [u8; 32],
}
#[repr(C, align(8))]
pub struct FAnimationEventBinding_Delegate {
    _opague: [u8; 32],
}
#[repr(C, align(8))]
pub struct FK2_AddFieldValueChangedDelegate_Delegate {
    _opague: [u8; 32],
}
#[repr(C, align(8))]
pub struct FK2_RemoveFieldValueChangedDelegate_Delegate {
    _opague: [u8; 32],
}
#[repr(C, align(8))]
pub struct FSetNavigationRuleCustom_InCustomDelegate {
    _opague: [u8; 32],
}
#[repr(C, align(8))]
pub struct FSetNavigationRuleCustomBoundary_InCustomDelegate {
    _opague: [u8; 32],
}
#[repr(C, align(8))]
pub struct FSetWindowTitleBarOnCloseClickedDelegate_Delegate {
    _opague: [u8; 32],
}
#[repr(C, align(8))]
pub struct FBindToAnimationEvent_Delegate {
    _opague: [u8; 32],
}
#[repr(C, align(8))]
pub struct FBindToAnimationFinished_Delegate {
    _opague: [u8; 32],
}
#[repr(C, align(8))]
pub struct FBindToAnimationStarted_Delegate {
    _opague: [u8; 32],
}
#[repr(C, align(8))]
pub struct FListenForInputAction_Callback {
    _opague: [u8; 32],
}
#[repr(C, align(8))]
pub struct FUnbindFromAnimationFinished_Delegate {
    _opague: [u8; 32],
}
#[repr(C, align(8))]
pub struct FUnbindFromAnimationStarted_Delegate {
    _opague: [u8; 32],
}
#[repr(C, align(8))]
pub struct FWidget_bIsEnabledDelegate {
    _opague: [u8; 32],
}
#[repr(C, align(8))]
pub struct FWidget_ToolTipTextDelegate {
    _opague: [u8; 32],
}
#[repr(C, align(8))]
pub struct FWidget_ToolTipWidgetDelegate {
    _opague: [u8; 32],
}
#[repr(C, align(8))]
pub struct FWidget_VisibilityDelegate {
    _opague: [u8; 32],
}
#[repr(C, align(8))]
pub struct FWidget_AccessibleTextDelegate {
    _opague: [u8; 32],
}
#[repr(C, align(8))]
pub struct FWidget_AccessibleSummaryTextDelegate {
    _opague: [u8; 32],
}
#[repr(C, align(8))]
pub struct FUserWidget_ColorAndOpacityDelegate {
    _opague: [u8; 32],
}
#[repr(C, align(8))]
pub struct FUserWidget_ForegroundColorDelegate {
    _opague: [u8; 32],
}
#[repr(C, align(8))]
pub struct FUserWidget_OnVisibilityChanged {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FUserWidget_Delegate {
    _opague: [u8; 32],
}
#[repr(C, align(8))]
pub struct FButton_OnClicked {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FButton_OnPressed {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FButton_OnReleased {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FButton_OnHovered {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FButton_OnUnhovered {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FCheckBox_CheckedStateDelegate {
    _opague: [u8; 32],
}
#[repr(C, align(8))]
pub struct FCheckBox_OnCheckStateChanged {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FComboBoxKey_OnGenerateContentWidget {
    _opague: [u8; 32],
}
#[repr(C, align(8))]
pub struct FComboBoxKey_OnGenerateItemWidget {
    _opague: [u8; 32],
}
#[repr(C, align(8))]
pub struct FComboBoxKey_OnSelectionChanged {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FComboBoxKey_OnOpening {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FComboBoxString_OnGenerateWidgetEvent {
    _opague: [u8; 32],
}
#[repr(C, align(8))]
pub struct FComboBoxString_OnSelectionChanged {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FComboBoxString_OnOpening {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FEditableText_TextDelegate {
    _opague: [u8; 32],
}
#[repr(C, align(8))]
pub struct FEditableText_HintTextDelegate {
    _opague: [u8; 32],
}
#[repr(C, align(8))]
pub struct FEditableText_OnTextChanged {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FEditableText_OnTextCommitted {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FEditableTextBox_TextDelegate {
    _opague: [u8; 32],
}
#[repr(C, align(8))]
pub struct FEditableTextBox_HintTextDelegate {
    _opague: [u8; 32],
}
#[repr(C, align(8))]
pub struct FEditableTextBox_OnTextChanged {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FEditableTextBox_OnTextCommitted {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FExpandableArea_OnExpansionChanged {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FInputKeySelector_OnKeySelected {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FInputKeySelector_OnIsSelectingKeyChanged {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FListViewBase_BP_OnEntryGenerated {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FListViewBase_BP_OnEntriesGenerated {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FListViewBase_BP_OnEntryReleased {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FListView_BP_OnEntryInitialized {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FListView_BP_OnItemClicked {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FListView_BP_OnItemDoubleClicked {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FListView_BP_OnItemDragDetected {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FListView_BP_OnItemDragEnter {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FListView_BP_OnItemDragLeave {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FListView_BP_OnItemAcceptDrop {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FListView_BP_OnItemDragCancelled {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FListView_BP_OnListViewDraggingStateChanged {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FListView_BP_OnItemIsHoveredChanged {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FListView_BP_OnItemSelectionChanged {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FListView_BP_OnItemScrolledIntoView {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FListView_BP_OnListViewScrolled {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FListView_BP_OnListViewFinishedScrolling {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FListView_BP_OnIsItemSelectableOrNavigable {
    _opague: [u8; 32],
}
#[repr(C, align(8))]
pub struct FMultiLineEditableText_HintTextDelegate {
    _opague: [u8; 32],
}
#[repr(C, align(8))]
pub struct FMultiLineEditableText_OnTextChanged {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FMultiLineEditableText_OnTextCommitted {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FMultiLineEditableTextBox_HintTextDelegate {
    _opague: [u8; 32],
}
#[repr(C, align(8))]
pub struct FMultiLineEditableTextBox_OnTextChanged {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FMultiLineEditableTextBox_OnTextCommitted {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FProgressBar_PercentDelegate {
    _opague: [u8; 32],
}
#[repr(C, align(8))]
pub struct FProgressBar_FillColorAndOpacityDelegate {
    _opague: [u8; 32],
}
#[repr(C, align(8))]
pub struct FScrollBox_OnUserScrolled {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FScrollBox_OnScrollBarVisibilityChanged {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FScrollBox_OnFocusReceived {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FScrollBox_OnFocusLost {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FSlider_ValueDelegate {
    _opague: [u8; 32],
}
#[repr(C, align(8))]
pub struct FSlider_OnMouseCaptureBegin {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FSlider_OnMouseCaptureEnd {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FSlider_OnControllerCaptureBegin {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FSlider_OnControllerCaptureEnd {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FSlider_OnValueChanged {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FSpinBox_ValueDelegate {
    _opague: [u8; 32],
}
#[repr(C, align(8))]
pub struct FSpinBox_OnValueChanged {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FSpinBox_OnValueCommitted {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FSpinBox_OnBeginSliderMovement {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FSpinBox_OnEndSliderMovement {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FTreeView_BP_OnGetItemChildren {
    _opague: [u8; 32],
}
#[repr(C, align(8))]
pub struct FTreeView_BP_OnItemExpansionChanged {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FSlateAccessibleWidgetData_AccessibleTextDelegate {
    _opague: [u8; 32],
}
#[repr(C, align(8))]
pub struct FSlateAccessibleWidgetData_AccessibleSummaryTextDelegate {
    _opague: [u8; 32],
}
#[repr(C, align(8))]
pub struct FWidgetNavigation_CustomDelegate {
    _opague: [u8; 32],
}
#[repr(C, align(8))]
pub struct FWidgetAnimationPlayCallbackProxy_Finished {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FAsyncTaskDownloadImage_OnSuccess {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FAsyncTaskDownloadImage_OnFail {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FBorder_ContentColorAndOpacityDelegate {
    _opague: [u8; 32],
}
#[repr(C, align(8))]
pub struct FBorder_BackgroundDelegate {
    _opague: [u8; 32],
}
#[repr(C, align(8))]
pub struct FBorder_BrushColorDelegate {
    _opague: [u8; 32],
}
#[repr(C, align(8))]
pub struct FBorder_OnMouseButtonDownEvent {
    _opague: [u8; 32],
}
#[repr(C, align(8))]
pub struct FBorder_OnMouseButtonUpEvent {
    _opague: [u8; 32],
}
#[repr(C, align(8))]
pub struct FBorder_OnMouseMoveEvent {
    _opague: [u8; 32],
}
#[repr(C, align(8))]
pub struct FBorder_OnMouseDoubleClickEvent {
    _opague: [u8; 32],
}
#[repr(C, align(8))]
pub struct FComboBox_OnGenerateWidgetEvent {
    _opague: [u8; 32],
}
#[repr(C, align(8))]
pub struct FImage_BrushDelegate {
    _opague: [u8; 32],
}
#[repr(C, align(8))]
pub struct FImage_ColorAndOpacityDelegate {
    _opague: [u8; 32],
}
#[repr(C, align(8))]
pub struct FImage_OnMouseButtonDownEvent {
    _opague: [u8; 32],
}
#[repr(C, align(8))]
pub struct FMenuAnchor_OnGetMenuContentEvent {
    _opague: [u8; 32],
}
#[repr(C, align(8))]
pub struct FMenuAnchor_OnGetUserMenuContentEvent {
    _opague: [u8; 32],
}
#[repr(C, align(8))]
pub struct FMenuAnchor_OnMenuOpenChanged {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FTextBlock_TextDelegate {
    _opague: [u8; 32],
}
#[repr(C, align(8))]
pub struct FTextBlock_ColorAndOpacityDelegate {
    _opague: [u8; 32],
}
#[repr(C, align(8))]
pub struct FTextBlock_ShadowColorAndOpacityDelegate {
    _opague: [u8; 32],
}
#[repr(C, align(8))]
pub struct FWidgetInteractionComponent_OnHoveredWidgetChanged {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FDragDropOperation_OnDrop {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FDragDropOperation_OnDragCancelled {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FDragDropOperation_OnDragged {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FNavigationUIComponent_OnNavigationEnteredDelegate {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FNavigationUIComponent_OnNavigationExitedDelegate {
    _opague: [u8; 24],
}
#[repr(transparent)]
pub struct EBindingKind(pub u8);
impl EBindingKind {
    pub const FUNCTION: EBindingKind = EBindingKind(0);
    pub const PROPERTY: EBindingKind = EBindingKind(1);
}
#[repr(transparent)]
pub struct ESlateSizeRule(pub u8);
impl ESlateSizeRule {
    pub const AUTOMATIC: ESlateSizeRule = ESlateSizeRule(0);
    pub const FILL: ESlateSizeRule = ESlateSizeRule(1);
}
#[repr(transparent)]
pub struct EWidgetAnimationEvent(pub u8);
impl EWidgetAnimationEvent {
    pub const STARTED: EWidgetAnimationEvent = EWidgetAnimationEvent(0);
    pub const FINISHED: EWidgetAnimationEvent = EWidgetAnimationEvent(1);
}
#[repr(transparent)]
pub struct ESlateVisibility(pub u8);
impl ESlateVisibility {
    pub const VISIBLE: ESlateVisibility = ESlateVisibility(0);
    pub const COLLAPSED: ESlateVisibility = ESlateVisibility(1);
    pub const HIDDEN: ESlateVisibility = ESlateVisibility(2);
    pub const HIT_TEST_INVISIBLE: ESlateVisibility = ESlateVisibility(3);
    pub const SELF_HIT_TEST_INVISIBLE: ESlateVisibility = ESlateVisibility(4);
}
#[repr(transparent)]
pub struct EUMGSequencePlayMode(pub u8);
impl EUMGSequencePlayMode {
    pub const FORWARD: EUMGSequencePlayMode = EUMGSequencePlayMode(0);
    pub const REVERSE: EUMGSequencePlayMode = EUMGSequencePlayMode(1);
    pub const PING_PONG: EUMGSequencePlayMode = EUMGSequencePlayMode(2);
}
#[repr(transparent)]
pub struct EWidgetGeometryMode(pub u8);
impl EWidgetGeometryMode {
    pub const PLANE: EWidgetGeometryMode = EWidgetGeometryMode(0);
    pub const CYLINDER: EWidgetGeometryMode = EWidgetGeometryMode(1);
}
#[repr(transparent)]
pub struct EWidgetSpace(pub u8);
impl EWidgetSpace {
    pub const WORLD: EWidgetSpace = EWidgetSpace(0);
    pub const SCREEN: EWidgetSpace = EWidgetSpace(1);
}
#[repr(transparent)]
pub struct EWindowVisibility(pub u8);
impl EWindowVisibility {
    pub const VISIBLE: EWindowVisibility = EWindowVisibility(0);
    pub const SELF_HIT_TEST_INVISIBLE: EWindowVisibility = EWindowVisibility(1);
}
#[repr(transparent)]
pub struct ETickMode(pub u8);
impl ETickMode {
    pub const DISABLED: ETickMode = ETickMode(0);
    pub const ENABLED: ETickMode = ETickMode(1);
    pub const AUTOMATIC: ETickMode = ETickMode(2);
}
#[repr(transparent)]
pub struct EUMGItemDropZone(pub u8);
impl EUMGItemDropZone {
    pub const ABOVE_ITEM: EUMGItemDropZone = EUMGItemDropZone(0);
    pub const ONTO_ITEM: EUMGItemDropZone = EUMGItemDropZone(1);
    pub const BELOW_ITEM: EUMGItemDropZone = EUMGItemDropZone(2);
    pub const NONE: EUMGItemDropZone = EUMGItemDropZone(3);
}
#[repr(transparent)]
pub struct ESlateAccessibleBehavior(pub u8);
impl ESlateAccessibleBehavior {
    pub const NOT_ACCESSIBLE: ESlateAccessibleBehavior = ESlateAccessibleBehavior(0);
    pub const AUTO: ESlateAccessibleBehavior = ESlateAccessibleBehavior(1);
    pub const SUMMARY: ESlateAccessibleBehavior = ESlateAccessibleBehavior(2);
    pub const CUSTOM: ESlateAccessibleBehavior = ESlateAccessibleBehavior(3);
    pub const TOOL_TIP: ESlateAccessibleBehavior = ESlateAccessibleBehavior(4);
}
#[repr(transparent)]
pub struct EDesignPreviewSizeMode(pub u8);
impl EDesignPreviewSizeMode {
    pub const FILL_SCREEN: EDesignPreviewSizeMode = EDesignPreviewSizeMode(0);
    pub const CUSTOM: EDesignPreviewSizeMode = EDesignPreviewSizeMode(1);
    pub const CUSTOM_ON_SCREEN: EDesignPreviewSizeMode = EDesignPreviewSizeMode(2);
    pub const DESIRED: EDesignPreviewSizeMode = EDesignPreviewSizeMode(3);
    pub const DESIRED_ON_SCREEN: EDesignPreviewSizeMode = EDesignPreviewSizeMode(4);
}
#[repr(transparent)]
pub struct EWidgetTickFrequency(pub u8);
impl EWidgetTickFrequency {
    pub const NEVER: EWidgetTickFrequency = EWidgetTickFrequency(0);
    pub const AUTO: EWidgetTickFrequency = EWidgetTickFrequency(1);
}
#[repr(transparent)]
pub struct EWidgetTimingPolicy(pub u8);
impl EWidgetTimingPolicy {
    pub const REAL_TIME: EWidgetTimingPolicy = EWidgetTimingPolicy(0);
    pub const GAME_TIME: EWidgetTimingPolicy = EWidgetTimingPolicy(1);
}
#[repr(transparent)]
pub struct EWidgetBlendMode(pub u8);
impl EWidgetBlendMode {
    pub const OPAQUE: EWidgetBlendMode = EWidgetBlendMode(0);
    pub const MASKED: EWidgetBlendMode = EWidgetBlendMode(1);
    pub const TRANSPARENT: EWidgetBlendMode = EWidgetBlendMode(2);
}
#[repr(transparent)]
pub struct EVirtualKeyboardType(pub u8);
impl EVirtualKeyboardType {
    pub const DEFAULT: EVirtualKeyboardType = EVirtualKeyboardType(0);
    pub const NUMBER: EVirtualKeyboardType = EVirtualKeyboardType(1);
    pub const WEB: EVirtualKeyboardType = EVirtualKeyboardType(2);
    pub const EMAIL: EVirtualKeyboardType = EVirtualKeyboardType(3);
    pub const PASSWORD: EVirtualKeyboardType = EVirtualKeyboardType(4);
    pub const ALPHA_NUMERIC: EVirtualKeyboardType = EVirtualKeyboardType(5);
}
#[repr(transparent)]
pub struct EDragPivot(pub u8);
impl EDragPivot {
    pub const MOUSE_DOWN: EDragPivot = EDragPivot(0);
    pub const TOP_LEFT: EDragPivot = EDragPivot(1);
    pub const TOP_CENTER: EDragPivot = EDragPivot(2);
    pub const TOP_RIGHT: EDragPivot = EDragPivot(3);
    pub const CENTER_LEFT: EDragPivot = EDragPivot(4);
    pub const CENTER_CENTER: EDragPivot = EDragPivot(5);
    pub const CENTER_RIGHT: EDragPivot = EDragPivot(6);
    pub const BOTTOM_LEFT: EDragPivot = EDragPivot(7);
    pub const BOTTOM_CENTER: EDragPivot = EDragPivot(8);
    pub const BOTTOM_RIGHT: EDragPivot = EDragPivot(9);
}
#[repr(transparent)]
pub struct EDynamicBoxType(pub u8);
impl EDynamicBoxType {
    pub const HORIZONTAL: EDynamicBoxType = EDynamicBoxType(0);
    pub const VERTICAL: EDynamicBoxType = EDynamicBoxType(1);
    pub const WRAP: EDynamicBoxType = EDynamicBoxType(2);
    pub const VERTICAL_WRAP: EDynamicBoxType = EDynamicBoxType(3);
    pub const RADIAL: EDynamicBoxType = EDynamicBoxType(4);
    pub const OVERLAY: EDynamicBoxType = EDynamicBoxType(5);
}
#[repr(transparent)]
pub struct EWidgetInteractionSource(pub u8);
impl EWidgetInteractionSource {
    pub const WORLD: EWidgetInteractionSource = EWidgetInteractionSource(0);
    pub const MOUSE: EWidgetInteractionSource = EWidgetInteractionSource(1);
    pub const CENTER_SCREEN: EWidgetInteractionSource = EWidgetInteractionSource(2);
    pub const CUSTOM: EWidgetInteractionSource = EWidgetInteractionSource(3);
}
