#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::prelude::*;
#[repr(C, align(8))]
pub struct FBindingTargets {
    pub started: UPtr<UInputBindingTarget>,
    pub ongoing: UPtr<UInputBindingTarget>,
    pub canceled: UPtr<UInputBindingTarget>,
    pub completed: UPtr<UInputBindingTarget>,
    pub triggered: UPtr<UInputBindingTarget>,
}
pub struct UEnhancedInputEditorProjectSettings {
    pub default_editor_input_class: TSoftObjectPtr<UClass>,
    pub default_mapping_contexts: TArray<FDefaultContextSetting>,
}
pub struct UEnhancedInputEditorSettings {
    pub flags_104: u8,
    pub visible_event_pins_by_default: u8,
}
pub struct UEnhancedInputEditorSubsystem {
    pub player_input: UPtr<UEnhancedPlayerInput>,
    pub current_input_stack: TArray<TWeakObjectPtr<UInputComponent>>,
    pub continuously_injected_inputs: TMap<UPtr<UInputAction>, FInjectedInput>,
}
pub struct UInputMappingContext_Factory {
    pub input_mapping_context_class: TSubclassOf<UInputMappingContext>,
}
pub struct UInputAction_Factory {
    pub input_action_class: TSubclassOf<UInputAction>,
}
pub struct UInputBindingTarget {}
pub struct UMockedEnhancedInputSubsystem {
    pub continuously_injected_inputs: TMap<UPtr<UInputAction>, FInjectedInput>,
}
pub struct UMockInputUserSettings {}
pub struct UControllablePlayer {
    pub player: UPtr<APlayerController>,
    pub binding_targets: TMap<FName, FBindingTargets>,
    pub input_context: TMap<FName, UPtr<UInputMappingContext>>,
    pub input_action: TMap<FName, UPtr<UInputAction>>,
    pub user_settings: UPtr<UEnhancedInputUserSettings>,
}
pub struct UTestMappableKeysAction {}
