#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(8))]
pub struct FBindingTargets {
    pub started: UPtr<UInputBindingTarget>,
    pub ongoing: UPtr<UInputBindingTarget>,
    pub canceled: UPtr<UInputBindingTarget>,
    pub completed: UPtr<UInputBindingTarget>,
    pub triggered: UPtr<UInputBindingTarget>,
}
pub struct UEnhancedInputEditorProjectSettings {
    pub default_editor_input_class: TSoftObjectPtr<
        crate::bindings::core_u_object::UClass,
    >,
    pub default_mapping_contexts: TArray<
        crate::bindings::enhanced_input::FDefaultContextSetting,
    >,
}
pub struct UEnhancedInputEditorSettings {
    pub flags_104: u8,
    pub visible_event_pins_by_default: u8,
}
pub struct UEnhancedInputEditorSubsystem {
    pub player_input: UPtr<crate::bindings::enhanced_input::UEnhancedPlayerInput>,
    pub current_input_stack: TArray<
        TWeakObjectPtr<crate::bindings::engine::UInputComponent>,
    >,
    pub continuously_injected_inputs: TMap<
        UPtr<crate::bindings::enhanced_input::UInputAction>,
        crate::bindings::enhanced_input::FInjectedInput,
    >,
}
pub struct UInputMappingContext_Factory {
    pub input_mapping_context_class: TSubclassOf<
        crate::bindings::enhanced_input::UInputMappingContext,
    >,
}
pub struct UInputAction_Factory {
    pub input_action_class: TSubclassOf<crate::bindings::enhanced_input::UInputAction>,
}
pub struct UInputBindingTarget {}
pub struct UMockedEnhancedInputSubsystem {
    pub continuously_injected_inputs: TMap<
        UPtr<crate::bindings::enhanced_input::UInputAction>,
        crate::bindings::enhanced_input::FInjectedInput,
    >,
}
pub struct UMockInputUserSettings {}
pub struct UControllablePlayer {
    pub player: UPtr<crate::bindings::engine::APlayerController>,
    pub binding_targets: TMap<FName, FBindingTargets>,
    pub input_context: TMap<
        FName,
        UPtr<crate::bindings::enhanced_input::UInputMappingContext>,
    >,
    pub input_action: TMap<FName, UPtr<crate::bindings::enhanced_input::UInputAction>>,
    pub user_settings: UPtr<crate::bindings::enhanced_input::UEnhancedInputUserSettings>,
}
pub struct UTestMappableKeysAction {}
