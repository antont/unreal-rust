#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
pub struct UEnhancedInputUserWidgetValidator {}
pub struct UInputActionEventNodeSpawner {
    pub weak_action_ptr: TWeakObjectPtr<crate::bindings::enhanced_input::UInputAction>,
}
pub struct UK2Node_EnhancedInputAction {
    pub input_action: UPtr<crate::bindings::enhanced_input::UInputAction>,
}
pub struct UK2Node_EnhancedInputActionEvent {
    pub input_action: UPtr<crate::bindings::enhanced_input::UInputAction>,
    pub trigger_event: crate::bindings::enhanced_input::ETriggerEvent,
}
pub struct UK2Node_GetInputActionValue {
    pub input_action: UPtr<crate::bindings::enhanced_input::UInputAction>,
}
pub struct UK2Node_InputActionValueAccessor {
    pub input_action: UPtr<crate::bindings::enhanced_input::UInputAction>,
}
pub struct UInputDebugKeyEventNodeSpawner {
    pub debug_key: crate::bindings::input_core::FKey,
}
pub struct UK2Node_InputDebugKey {
    pub input_key: crate::bindings::input_core::FKey,
    pub flags_232: u8,
}
pub struct UK2Node_InputDebugKeyEvent {
    pub input_chord: crate::bindings::slate::FInputChord,
    pub input_key_event: crate::bindings::engine::EInputEvent,
    pub b_execute_when_paused: bool,
}
