#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::prelude::*;
pub struct UEnhancedInputUserWidgetValidator {}
pub struct UInputActionEventNodeSpawner {
    pub weak_action_ptr: TWeakObjectPtr<UInputAction>,
}
pub struct UK2Node_EnhancedInputAction {
    pub input_action: UPtr<UInputAction>,
}
pub struct UK2Node_EnhancedInputActionEvent {
    pub input_action: UPtr<UInputAction>,
    pub trigger_event: ETriggerEvent,
}
pub struct UK2Node_GetInputActionValue {
    pub input_action: UPtr<UInputAction>,
}
pub struct UK2Node_InputActionValueAccessor {
    pub input_action: UPtr<UInputAction>,
}
pub struct UInputDebugKeyEventNodeSpawner {
    pub debug_key: FKey,
}
pub struct UK2Node_InputDebugKey {
    pub input_key: FKey,
    pub flags_232: u8,
}
pub struct UK2Node_InputDebugKeyEvent {
    pub input_chord: FInputChord,
    pub input_key_event: EInputEvent,
    pub b_execute_when_paused: bool,
}
