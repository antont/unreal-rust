#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::prelude::*;
#[repr(C, align(8))]
pub struct FInputActionValue {}
#[repr(C, align(8))]
pub struct FInjectedInput {
    pub triggers: TArray<UPtr<UInputTrigger>>,
    pub modifiers: TArray<UPtr<UInputModifier>>,
}
#[repr(C, align(8))]
pub struct FPlayerMappableKeyProfileCreationArgs {
    pub profile_type: TSubclassOf<UEnhancedPlayerMappableKeyProfile>,
    pub profile_identifier: FGameplayTag,
    pub profile_string_identifier: FString,
    pub user_id: FPlatformUserId,
    pub display_name: FText,
    pub flags_64: u8,
}
#[repr(C, align(8))]
pub struct FPlayerKeyMapping {
    pub mapping_name: FName,
    pub display_name: FText,
    pub display_category: FText,
    pub slot: EPlayerMappableKeySlot,
    pub flags_49: u8,
    pub default_key: FKey,
    pub current_key: FKey,
    pub hardware_device_id: FHardwareDeviceIdentifier,
    pub associated_input_action: UPtr<UInputAction>,
    pub associated_input_action_soft: TSoftObjectPtr<UInputAction>,
}
#[repr(C, align(8))]
pub struct FMapPlayerKeyArgs {
    pub mapping_name: FName,
    pub slot: EPlayerMappableKeySlot,
    pub new_key: FKey,
    pub hardware_device_id: FName,
    pub profile_id: FGameplayTag,
    pub profile_id_string: FString,
    pub flags_88: u8,
}
#[repr(C, align(8))]
pub struct FMappingQueryIssue {
    pub issue: EMappingQueryIssue,
    pub blocking_context: UPtr<UInputMappingContext>,
    pub blocking_action: UPtr<UInputAction>,
}
#[repr(C, align(8))]
pub struct FEnhancedActionKeyMapping {
    pub triggers: TArray<UPtr<UInputTrigger>>,
    pub modifiers: TArray<UPtr<UInputModifier>>,
    pub action: UPtr<UInputAction>,
    pub key: FKey,
    pub flags_72: u8,
    pub setting_behavior: EPlayerMappableKeySettingBehaviors,
    pub player_mappable_key_settings: UPtr<UPlayerMappableKeySettings>,
}
#[repr(C, align(8))]
pub struct FBlueprintEnhancedInputActionBinding {
    pub input_action: UPtr<UInputAction>,
    pub trigger_event: ETriggerEvent,
    pub function_name_to_bind: FName,
}
#[repr(C, align(8))]
pub struct FDefaultContextSetting {
    pub input_mapping_context: TSoftObjectPtr<UInputMappingContext>,
    pub priority: i32,
    pub b_add_immediately: bool,
    pub b_register_with_user_settings: bool,
}
#[repr(C, align(1))]
pub struct FModifyContextOptions {
    pub flags_0: u8,
}
#[repr(C, align(8))]
pub struct FKeyConsumptionOptions {}
#[repr(C, align(8))]
pub struct FInjectedInputArray {
    pub injected: TArray<FInjectedInput>,
}
#[repr(C, align(4))]
pub struct FAppliedInputContextData {
    pub priority: i32,
    pub registration_count: i32,
}
#[repr(C, align(8))]
pub struct FInputActionInstance {
    pub source_action: UPtr<UInputAction>,
    pub trigger_event: ETriggerEvent,
    pub last_triggered_world_time: f32,
    pub triggers: TArray<UPtr<UInputTrigger>>,
    pub modifiers: TArray<UPtr<UInputModifier>>,
    pub elapsed_processed_time: f32,
    pub elapsed_triggered_time: f32,
}
#[repr(C, align(8))]
pub struct FBlueprintInputDebugKeyDelegateBinding {
    pub input_chord: FInputChord,
    pub input_key_event: EInputEvent,
    pub function_name_to_bind: FName,
    pub b_execute_when_paused: bool,
}
#[repr(C, align(8))]
pub struct FInputMappingContextMappingData {
    pub mappings: TArray<FEnhancedActionKeyMapping>,
}
#[repr(C, align(8))]
pub struct FInputComboStepData {
    pub combo_step_action: UPtr<UInputAction>,
    pub combo_step_completion_states: u8,
    pub time_to_press_key: f32,
}
#[repr(C, align(8))]
pub struct FInputCancelAction {
    pub cancel_action: UPtr<UInputAction>,
    pub cancellation_states: u8,
}
#[repr(C, align(8))]
pub struct FKeyMappingRow {
    pub mappings: TSet<FPlayerKeyMapping>,
}
#[repr(C, align(8))]
pub struct FPlayerMappableKeyQueryOptions {
    pub mapping_name: FName,
    pub key_to_match: FKey,
    pub slot_to_match: EPlayerMappableKeySlot,
    pub flags_49: u8,
    pub required_device_type: EHardwareDevicePrimaryType,
    pub required_device_flags: i32,
}
pub struct UEnhancedInputActionDelegateBinding {
    pub input_action_delegate_bindings: TArray<FBlueprintEnhancedInputActionBinding>,
}
pub struct UEnhancedInputActionValueBinding {
    pub input_action_value_bindings: TArray<FBlueprintEnhancedInputActionBinding>,
}
pub struct UEnhancedInputComponent {}
pub struct UEnhancedInputDeveloperSettings {
    pub default_mapping_contexts: TArray<FDefaultContextSetting>,
    pub default_world_subsystem_mapping_contexts: TArray<FDefaultContextSetting>,
    pub platform_settings: FPerPlatformSettings,
    pub user_settings_class: TSoftObjectPtr<UClass>,
    pub default_player_mappable_key_profile_class: TSoftObjectPtr<UClass>,
    pub input_settings_save_slot_name: FString,
    pub default_world_input_class: TSoftObjectPtr<UClass>,
    pub flags_312: u8,
    pub default_mapping_context_input_mode_query: FGameplayTagQuery,
    pub default_input_mode: FGameplayTagContainer,
}
pub struct UEnhancedInputLibrary {}
pub struct UEnhancedInputPlatformData {
    pub mapping_context_redirects: TMap<
        UPtr<UInputMappingContext>,
        UPtr<UInputMappingContext>,
    >,
}
pub struct UEnhancedInputPlatformSettings {
    pub input_data: TArray<TSoftObjectPtr<UClass>>,
    pub input_data_classes: TArray<TSubclassOf<UEnhancedInputPlatformData>>,
    pub b_should_log_mapping_context_redirects: bool,
}
pub struct UEnhancedInputSubsystemInterface {}
pub struct IEnhancedInputSubsystemInterface {}
pub struct UEnhancedInputLocalPlayerSubsystem {
    pub control_mappings_rebuilt_delegate: FEnhancedInputLocalPlayerSubsystem_ControlMappingsRebuiltDelegate,
    pub on_mapping_context_added: FEnhancedInputLocalPlayerSubsystem_OnMappingContextAdded,
    pub on_mapping_context_removed: FEnhancedInputLocalPlayerSubsystem_OnMappingContextRemoved,
    pub on_post_user_settings_initialized: FEnhancedInputLocalPlayerSubsystem_OnPostUserSettingsInitialized,
    pub user_settings: UPtr<UEnhancedInputUserSettings>,
    pub continuously_injected_inputs: TMap<UPtr<UInputAction>, FInjectedInput>,
}
pub struct UEnhancedInputWorldSubsystem {
    pub player_input: UPtr<UEnhancedPlayerInput>,
    pub current_input_stack: TArray<TWeakObjectPtr<UInputComponent>>,
    pub continuously_injected_inputs: TMap<UPtr<UInputAction>, FInjectedInput>,
}
pub struct UEnhancedPlayerInput {
    pub key_consumption_data: TMap<UPtr<UInputAction>, FKeyConsumptionOptions>,
    pub action_instance_data: TMap<UPtr<UInputAction>, FInputActionInstance>,
    pub consumed_input_actions: TSet<UPtr<UInputAction>>,
    pub applied_input_context_data: TMap<
        UPtr<UInputMappingContext>,
        FAppliedInputContextData,
    >,
    pub applied_input_contexts: TMap<UPtr<UInputMappingContext>, i32>,
    pub enhanced_action_mappings: TArray<FEnhancedActionKeyMapping>,
    pub current_input_mode: FGameplayTagContainer,
    pub keys_pressed_this_tick: TMap<FKey, FVector>,
    pub inputs_injected_this_tick: TMap<UPtr<UInputAction>, FInjectedInputArray>,
    pub last_injected_actions: TSet<UPtr<UInputAction>>,
}
pub struct UInputAction {
    pub action_description: FText,
    pub b_trigger_when_paused: bool,
    pub b_consume_input: bool,
    pub b_consumes_action_and_axis_mappings: bool,
    pub b_reserve_all_mappings: bool,
    pub trigger_events_that_consume_legacy_keys: i32,
    pub value_type: EInputActionValueType,
    pub accumulation_behavior: EInputActionAccumulationBehavior,
    pub triggers: TArray<UPtr<UInputTrigger>>,
    pub modifiers: TArray<UPtr<UInputModifier>>,
    pub player_mappable_key_settings: UPtr<UPlayerMappableKeySettings>,
}
pub struct UInputDebugKeyDelegateBinding {
    pub input_debug_key_delegate_bindings: TArray<
        FBlueprintInputDebugKeyDelegateBinding,
    >,
}
pub struct UInputMappingContext {
    pub mappings: TArray<FEnhancedActionKeyMapping>,
    pub default_key_mappings: FInputMappingContextMappingData,
    pub mapping_profile_overrides: TMap<FString, FInputMappingContextMappingData>,
    pub input_mode_filter_options: EMappingContextInputModeFilterOptions,
    pub input_mode_query_override: FGameplayTagQuery,
    pub registration_tracking_mode: EMappingContextRegistrationTrackingMode,
    pub context_description: FText,
}
pub struct UInputModifier {}
pub struct UInputModifierSmoothDelta {
    pub smoothing_method: ENormalizeInputSmoothingType,
    pub speed: f32,
    pub easing_exponent: f32,
}
pub struct UInputModifierDeadZone {
    pub lower_threshold: f32,
    pub upper_threshold: f32,
    pub ty: EDeadZoneType,
}
pub struct UInputModifierScalar {
    pub scalar: FVector,
}
pub struct UInputModifierScaleByDeltaTime {}
pub struct UInputModifierNegate {
    pub b_x: bool,
    pub b_y: bool,
    pub b_z: bool,
}
pub struct UInputModifierSmooth {}
pub struct UInputModifierResponseCurveExponential {
    pub curve_exponent: FVector,
}
pub struct UInputModifierResponseCurveUser {
    pub response_x: UPtr<UCurveFloat>,
    pub response_y: UPtr<UCurveFloat>,
    pub response_z: UPtr<UCurveFloat>,
}
pub struct UInputModifierFOVScaling {
    pub fov_scale: f32,
    pub fov_scaling_type: EFOVScalingType,
}
pub struct UInputModifierToWorldSpace {}
pub struct UInputModifierSwizzleAxis {
    pub order: EInputAxisSwizzle,
}
pub struct UInputTrigger {
    pub actuation_threshold: f32,
    pub b_should_always_tick: bool,
    pub last_value: FInputActionValue,
}
pub struct UInputTriggerTimedBase {
    pub held_duration: f32,
    pub b_affected_by_time_dilation: bool,
}
pub struct UInputTriggerDown {}
pub struct UInputTriggerPressed {}
pub struct UInputTriggerReleased {}
pub struct UInputTriggerHold {
    pub hold_time_threshold: f32,
    pub b_is_one_shot: bool,
}
pub struct UInputTriggerHoldAndRelease {
    pub hold_time_threshold: f32,
}
pub struct UInputTriggerTap {
    pub tap_release_time_threshold: f32,
}
pub struct UInputTriggerRepeatedTap {
    pub repeat_delay: f64,
    pub repeat_time: f64,
    pub number_of_taps_which_trigger_repeat: i32,
    pub tap_release_time_threshold: f32,
    pub number_of_taps_since_last_trigger: i32,
}
pub struct UInputTriggerPulse {
    pub b_trigger_on_start: bool,
    pub interval: f32,
    pub trigger_limit: i32,
}
pub struct UInputTriggerChordAction {
    pub chord_action: UPtr<UInputAction>,
}
pub struct UInputTriggerChordBlocker {}
pub struct UInputTriggerCombo {
    pub current_combo_step_index: i32,
    pub current_time_between_combo_steps: f32,
    pub combo_actions: TArray<FInputComboStepData>,
    pub input_cancel_actions: TArray<FInputCancelAction>,
}
pub struct UPlayerMappableInputConfig {
    pub config_name: FName,
    pub config_display_name: FText,
    pub b_is_deprecated: bool,
    pub metadata: UPtr<UObject>,
    pub contexts: TMap<UPtr<UInputMappingContext>, i32>,
}
pub struct UPlayerMappableKeySettings {
    pub metadata: UPtr<UObject>,
    pub name: FName,
    pub display_name: FText,
    pub display_category: FText,
    pub supported_key_profiles: FGameplayTagContainer,
    pub supported_key_profile_ids: TArray<FString>,
}
pub struct UEnhancedPlayerMappableKeyProfile {
    pub profile_identifier: FGameplayTag,
    pub profile_identifier_string: FString,
    pub owning_user_id: FPlatformUserId,
    pub display_name: FText,
    pub player_mapped_keys: TMap<FName, FKeyMappingRow>,
}
pub struct UEnhancedInputUserSettings {
    pub on_settings_changed: FEnhancedInputUserSettings_OnSettingsChanged,
    pub on_settings_applied: FEnhancedInputUserSettings_OnSettingsApplied,
    pub on_key_profile_changed: FEnhancedInputUserSettings_OnKeyProfileChanged,
    pub current_profile_identifier: FGameplayTag,
    pub current_profile_identifier_string: FString,
    pub saved_key_profiles: TMap<FGameplayTag, UPtr<UEnhancedPlayerMappableKeyProfile>>,
    pub saved_key_profiles_map: TMap<FString, UPtr<UEnhancedPlayerMappableKeyProfile>>,
    pub owning_local_player: TWeakObjectPtr<ULocalPlayer>,
    pub registered_mapping_contexts: TSet<UPtr<UInputMappingContext>>,
}
