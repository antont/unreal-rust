#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
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
    pub profile_identifier: crate::bindings::gameplay_tags::FGameplayTag,
    pub profile_string_identifier: FString,
    pub user_id: crate::bindings::core_u_object::FPlatformUserId,
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
    pub default_key: crate::bindings::input_core::FKey,
    pub current_key: crate::bindings::input_core::FKey,
    pub hardware_device_id: crate::bindings::engine::FHardwareDeviceIdentifier,
    pub associated_input_action: UPtr<UInputAction>,
    pub associated_input_action_soft: TSoftObjectPtr<UInputAction>,
}
#[repr(C, align(8))]
pub struct FMapPlayerKeyArgs {
    pub mapping_name: FName,
    pub slot: EPlayerMappableKeySlot,
    pub new_key: crate::bindings::input_core::FKey,
    pub hardware_device_id: FName,
    pub profile_id: crate::bindings::gameplay_tags::FGameplayTag,
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
    pub key: crate::bindings::input_core::FKey,
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
    pub input_chord: crate::bindings::slate::FInputChord,
    pub input_key_event: crate::bindings::engine::EInputEvent,
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
    pub key_to_match: crate::bindings::input_core::FKey,
    pub slot_to_match: EPlayerMappableKeySlot,
    pub flags_49: u8,
    pub required_device_type: crate::bindings::engine::EHardwareDevicePrimaryType,
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
    pub platform_settings: crate::bindings::developer_settings::FPerPlatformSettings,
    pub user_settings_class: TSoftObjectPtr<crate::bindings::core_u_object::UClass>,
    pub default_player_mappable_key_profile_class: TSoftObjectPtr<
        crate::bindings::core_u_object::UClass,
    >,
    pub input_settings_save_slot_name: FString,
    pub default_world_input_class: TSoftObjectPtr<
        crate::bindings::core_u_object::UClass,
    >,
    pub flags_312: u8,
    pub default_mapping_context_input_mode_query: crate::bindings::gameplay_tags::FGameplayTagQuery,
    pub default_input_mode: crate::bindings::gameplay_tags::FGameplayTagContainer,
}
pub struct UEnhancedInputLibrary {}
pub struct UEnhancedInputPlatformData {
    pub mapping_context_redirects: TMap<
        UPtr<UInputMappingContext>,
        UPtr<UInputMappingContext>,
    >,
}
pub struct UEnhancedInputPlatformSettings {
    pub input_data: TArray<TSoftObjectPtr<crate::bindings::core_u_object::UClass>>,
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
    pub current_input_stack: TArray<
        TWeakObjectPtr<crate::bindings::engine::UInputComponent>,
    >,
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
    pub current_input_mode: crate::bindings::gameplay_tags::FGameplayTagContainer,
    pub keys_pressed_this_tick: TMap<
        crate::bindings::input_core::FKey,
        crate::bindings::core_u_object::FVector,
    >,
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
    pub input_mode_query_override: crate::bindings::gameplay_tags::FGameplayTagQuery,
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
    pub scalar: crate::bindings::core_u_object::FVector,
}
pub struct UInputModifierScaleByDeltaTime {}
pub struct UInputModifierNegate {
    pub b_x: bool,
    pub b_y: bool,
    pub b_z: bool,
}
pub struct UInputModifierSmooth {}
pub struct UInputModifierResponseCurveExponential {
    pub curve_exponent: crate::bindings::core_u_object::FVector,
}
pub struct UInputModifierResponseCurveUser {
    pub response_x: UPtr<crate::bindings::engine::UCurveFloat>,
    pub response_y: UPtr<crate::bindings::engine::UCurveFloat>,
    pub response_z: UPtr<crate::bindings::engine::UCurveFloat>,
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
    pub metadata: UPtr<crate::bindings::core_u_object::UObject>,
    pub contexts: TMap<UPtr<UInputMappingContext>, i32>,
}
pub struct UPlayerMappableKeySettings {
    pub metadata: UPtr<crate::bindings::core_u_object::UObject>,
    pub name: FName,
    pub display_name: FText,
    pub display_category: FText,
    pub supported_key_profiles: crate::bindings::gameplay_tags::FGameplayTagContainer,
    pub supported_key_profile_ids: TArray<FString>,
}
pub struct UEnhancedPlayerMappableKeyProfile {
    pub profile_identifier: crate::bindings::gameplay_tags::FGameplayTag,
    pub profile_identifier_string: FString,
    pub owning_user_id: crate::bindings::core_u_object::FPlatformUserId,
    pub display_name: FText,
    pub player_mapped_keys: TMap<FName, FKeyMappingRow>,
}
pub struct UEnhancedInputUserSettings {
    pub on_settings_changed: FEnhancedInputUserSettings_OnSettingsChanged,
    pub on_settings_applied: FEnhancedInputUserSettings_OnSettingsApplied,
    pub on_key_profile_changed: FEnhancedInputUserSettings_OnKeyProfileChanged,
    pub current_profile_identifier: crate::bindings::gameplay_tags::FGameplayTag,
    pub current_profile_identifier_string: FString,
    pub saved_key_profiles: TMap<
        crate::bindings::gameplay_tags::FGameplayTag,
        UPtr<UEnhancedPlayerMappableKeyProfile>,
    >,
    pub saved_key_profiles_map: TMap<FString, UPtr<UEnhancedPlayerMappableKeyProfile>>,
    pub owning_local_player: TWeakObjectPtr<crate::bindings::engine::ULocalPlayer>,
    pub registered_mapping_contexts: TSet<UPtr<UInputMappingContext>>,
}
pub struct FEnhancedInputLocalPlayerSubsystem_ControlMappingsRebuiltDelegate;
pub struct FEnhancedInputLocalPlayerSubsystem_OnMappingContextAdded;
pub struct FEnhancedInputLocalPlayerSubsystem_OnMappingContextRemoved;
pub struct FEnhancedInputLocalPlayerSubsystem_OnPostUserSettingsInitialized;
pub struct FEnhancedInputUserSettings_OnSettingsChanged;
pub struct FEnhancedInputUserSettings_OnSettingsApplied;
pub struct FEnhancedInputUserSettings_OnKeyProfileChanged;
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EPlayerMappableKeySlot(pub u8);
impl EPlayerMappableKeySlot {
    pub const FIRST: EPlayerMappableKeySlot = EPlayerMappableKeySlot(0);
    pub const SECOND: EPlayerMappableKeySlot = EPlayerMappableKeySlot(1);
    pub const THIRD: EPlayerMappableKeySlot = EPlayerMappableKeySlot(2);
    pub const FOURTH: EPlayerMappableKeySlot = EPlayerMappableKeySlot(3);
    pub const FIFTH: EPlayerMappableKeySlot = EPlayerMappableKeySlot(4);
    pub const SIXTH: EPlayerMappableKeySlot = EPlayerMappableKeySlot(5);
    pub const SEVENTH: EPlayerMappableKeySlot = EPlayerMappableKeySlot(6);
    pub const UNSPECIFIED: EPlayerMappableKeySlot = EPlayerMappableKeySlot(7);
    pub const MAX: EPlayerMappableKeySlot = EPlayerMappableKeySlot(8);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EMappingQueryIssue(pub u8);
impl EMappingQueryIssue {
    pub const NO_ISSUE: EMappingQueryIssue = EMappingQueryIssue(0);
    pub const RESERVED_BY_ACTION: EMappingQueryIssue = EMappingQueryIssue(1);
    pub const HIDES_EXISTING_MAPPING: EMappingQueryIssue = EMappingQueryIssue(2);
    pub const HIDDEN_BY_EXISTING_MAPPING: EMappingQueryIssue = EMappingQueryIssue(4);
    pub const COLLISION_WITH_MAPPING_IN_SAME_CONTEXT: EMappingQueryIssue = EMappingQueryIssue(
        8,
    );
    pub const FORCES_TYPE_PROMOTION: EMappingQueryIssue = EMappingQueryIssue(16);
    pub const FORCES_TYPE_DEMOTION: EMappingQueryIssue = EMappingQueryIssue(32);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EPlayerMappableKeySettingBehaviors(pub u8);
impl EPlayerMappableKeySettingBehaviors {
    pub const INHERIT_SETTINGS_FROM_ACTION: EPlayerMappableKeySettingBehaviors = EPlayerMappableKeySettingBehaviors(
        0,
    );
    pub const OVERRIDE_SETTINGS: EPlayerMappableKeySettingBehaviors = EPlayerMappableKeySettingBehaviors(
        1,
    );
    pub const IGNORE_SETTINGS: EPlayerMappableKeySettingBehaviors = EPlayerMappableKeySettingBehaviors(
        2,
    );
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ETriggerEvent(pub u8);
impl ETriggerEvent {
    pub const NONE: ETriggerEvent = ETriggerEvent(0);
    pub const TRIGGERED: ETriggerEvent = ETriggerEvent(1);
    pub const STARTED: ETriggerEvent = ETriggerEvent(2);
    pub const ONGOING: ETriggerEvent = ETriggerEvent(4);
    pub const CANCELED: ETriggerEvent = ETriggerEvent(8);
    pub const COMPLETED: ETriggerEvent = ETriggerEvent(16);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EInputActionValueType(pub u8);
impl EInputActionValueType {
    pub const BOOLEAN: EInputActionValueType = EInputActionValueType(0);
    pub const AXIS1_D: EInputActionValueType = EInputActionValueType(1);
    pub const AXIS2_D: EInputActionValueType = EInputActionValueType(2);
    pub const AXIS3_D: EInputActionValueType = EInputActionValueType(3);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EMappingQueryResult(pub u8);
impl EMappingQueryResult {
    pub const ERROR_ENHANCED_INPUT_NOT_ENABLED: EMappingQueryResult = EMappingQueryResult(
        0,
    );
    pub const ERROR_INPUT_CONTEXT_NOT_IN_ACTIVE_CONTEXTS: EMappingQueryResult = EMappingQueryResult(
        1,
    );
    pub const ERROR_INVALID_ACTION: EMappingQueryResult = EMappingQueryResult(2);
    pub const NOT_MAPPABLE: EMappingQueryResult = EMappingQueryResult(3);
    pub const MAPPING_AVAILABLE: EMappingQueryResult = EMappingQueryResult(4);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EInputMappingRebuildType(pub u8);
impl EInputMappingRebuildType {
    pub const NONE: EInputMappingRebuildType = EInputMappingRebuildType(0);
    pub const REBUILD: EInputMappingRebuildType = EInputMappingRebuildType(1);
    pub const REBUILD_WITH_FLUSH: EInputMappingRebuildType = EInputMappingRebuildType(2);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ETriggerType(pub u8);
impl ETriggerType {
    pub const EXPLICIT: ETriggerType = ETriggerType(0);
    pub const IMPLICIT: ETriggerType = ETriggerType(1);
    pub const BLOCKER: ETriggerType = ETriggerType(2);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ETriggerState(pub u8);
impl ETriggerState {
    pub const NONE: ETriggerState = ETriggerState(0);
    pub const ONGOING: ETriggerState = ETriggerState(1);
    pub const TRIGGERED: ETriggerState = ETriggerState(2);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EInputActionAccumulationBehavior(pub u8);
impl EInputActionAccumulationBehavior {
    pub const TAKE_HIGHEST_ABSOLUTE_VALUE: EInputActionAccumulationBehavior = EInputActionAccumulationBehavior(
        0,
    );
    pub const CUMULATIVE: EInputActionAccumulationBehavior = EInputActionAccumulationBehavior(
        1,
    );
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EMappingContextInputModeFilterOptions(pub u8);
impl EMappingContextInputModeFilterOptions {
    pub const USE_PROJECT_DEFAULT_QUERY: EMappingContextInputModeFilterOptions = EMappingContextInputModeFilterOptions(
        0,
    );
    pub const USE_CUSTOM_QUERY: EMappingContextInputModeFilterOptions = EMappingContextInputModeFilterOptions(
        1,
    );
    pub const DO_NOT_FILTER: EMappingContextInputModeFilterOptions = EMappingContextInputModeFilterOptions(
        2,
    );
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EMappingContextRegistrationTrackingMode(pub u8);
impl EMappingContextRegistrationTrackingMode {
    pub const UNTRACKED: EMappingContextRegistrationTrackingMode = EMappingContextRegistrationTrackingMode(
        0,
    );
    pub const COUNT_REGISTRATIONS: EMappingContextRegistrationTrackingMode = EMappingContextRegistrationTrackingMode(
        1,
    );
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ENormalizeInputSmoothingType(pub u8);
impl ENormalizeInputSmoothingType {
    pub const NONE: ENormalizeInputSmoothingType = ENormalizeInputSmoothingType(0);
    pub const LERP: ENormalizeInputSmoothingType = ENormalizeInputSmoothingType(1);
    pub const INTERP_TO: ENormalizeInputSmoothingType = ENormalizeInputSmoothingType(2);
    pub const INTERP_CONSTANT_TO: ENormalizeInputSmoothingType = ENormalizeInputSmoothingType(
        3,
    );
    pub const INTERP_CIRCULAR_IN: ENormalizeInputSmoothingType = ENormalizeInputSmoothingType(
        4,
    );
    pub const INTERP_CIRCULAR_OUT: ENormalizeInputSmoothingType = ENormalizeInputSmoothingType(
        5,
    );
    pub const INTERP_CIRCULAR_IN_OUT: ENormalizeInputSmoothingType = ENormalizeInputSmoothingType(
        6,
    );
    pub const INTERP_EASE_IN: ENormalizeInputSmoothingType = ENormalizeInputSmoothingType(
        7,
    );
    pub const INTERP_EASE_OUT: ENormalizeInputSmoothingType = ENormalizeInputSmoothingType(
        8,
    );
    pub const INTERP_EASE_IN_OUT: ENormalizeInputSmoothingType = ENormalizeInputSmoothingType(
        9,
    );
    pub const INTERP_EXPO_IN: ENormalizeInputSmoothingType = ENormalizeInputSmoothingType(
        10,
    );
    pub const INTERP_EXPO_OUT: ENormalizeInputSmoothingType = ENormalizeInputSmoothingType(
        11,
    );
    pub const INTERP_EXPO_IN_OUT: ENormalizeInputSmoothingType = ENormalizeInputSmoothingType(
        12,
    );
    pub const INTERP_SIN_IN: ENormalizeInputSmoothingType = ENormalizeInputSmoothingType(
        13,
    );
    pub const INTERP_SIN_OUT: ENormalizeInputSmoothingType = ENormalizeInputSmoothingType(
        14,
    );
    pub const INTERP_SIN_IN_OUT: ENormalizeInputSmoothingType = ENormalizeInputSmoothingType(
        15,
    );
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EDeadZoneType(pub u8);
impl EDeadZoneType {
    pub const AXIAL: EDeadZoneType = EDeadZoneType(0);
    pub const RADIAL: EDeadZoneType = EDeadZoneType(1);
    pub const UNSCALED_RADIAL: EDeadZoneType = EDeadZoneType(2);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EFOVScalingType(pub u8);
impl EFOVScalingType {
    pub const STANDARD: EFOVScalingType = EFOVScalingType(0);
    pub const UE4_BACK_COMPAT: EFOVScalingType = EFOVScalingType(1);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EInputAxisSwizzle(pub u8);
impl EInputAxisSwizzle {
    pub const YXZ: EInputAxisSwizzle = EInputAxisSwizzle(0);
    pub const ZYX: EInputAxisSwizzle = EInputAxisSwizzle(1);
    pub const XZY: EInputAxisSwizzle = EInputAxisSwizzle(2);
    pub const YZX: EInputAxisSwizzle = EInputAxisSwizzle(3);
    pub const ZXY: EInputAxisSwizzle = EInputAxisSwizzle(4);
}
