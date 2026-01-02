#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(8))]
pub struct FInputActionValue {
    __padding_end: [u8; 32],
}
impl FInputActionValue {}
#[repr(C, align(8))]
pub struct FPlayerMappableKeyProfileCreationArgs {
    pub profile_type: TSubclassOf<UEnhancedPlayerMappableKeyProfile>,
    #[doc(hidden)]
    __padding_24: [u8; 16],
    pub profile_string_identifier: FString,
    pub user_id: crate::bindings::core_u_object::FPlatformUserId,
    pub display_name: FText,
    pub flags_64: u8,
    __padding_end: [u8; 7],
}
impl FPlayerMappableKeyProfileCreationArgs {}
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
impl FPlayerKeyMapping {}
#[repr(C, align(8))]
pub struct FMapPlayerKeyArgs {
    pub mapping_name: FName,
    pub slot: EPlayerMappableKeySlot,
    pub new_key: crate::bindings::input_core::FKey,
    pub hardware_device_id: FName,
    #[doc(hidden)]
    __padding_72: [u8; 8],
    pub profile_id_string: FString,
    pub flags_88: u8,
    __padding_end: [u8; 7],
}
impl FMapPlayerKeyArgs {}
#[repr(C, align(8))]
pub struct FMappingQueryIssue {
    pub issue: EMappingQueryIssue,
    pub blocking_context: UPtr<UInputMappingContext>,
    pub blocking_action: UPtr<UInputAction>,
}
impl FMappingQueryIssue {}
#[repr(C, align(8))]
pub struct FEnhancedActionKeyMapping {
    pub triggers: TArray<UPtr<UInputTrigger>>,
    pub modifiers: TArray<UPtr<UInputModifier>>,
    pub action: UPtr<UInputAction>,
    pub key: crate::bindings::input_core::FKey,
    __padding_end: [u8; 16],
}
impl FEnhancedActionKeyMapping {}
#[repr(C, align(1))]
pub struct FModifyContextOptions {
    pub flags_0: u8,
}
impl FModifyContextOptions {}
#[repr(C, align(8))]
pub struct FInputActionInstance {
    pub source_action: UPtr<UInputAction>,
    #[doc(hidden)]
    __padding_19: [u8; 11],
    pub trigger_event: ETriggerEvent,
    pub last_triggered_world_time: f32,
    pub triggers: TArray<UPtr<UInputTrigger>>,
    pub modifiers: TArray<UPtr<UInputModifier>>,
    #[doc(hidden)]
    __padding_88: [u8; 32],
    pub elapsed_processed_time: f32,
    pub elapsed_triggered_time: f32,
}
impl FInputActionInstance {}
#[repr(C, align(8))]
pub struct FInputMappingContextMappingData {
    pub mappings: TArray<FEnhancedActionKeyMapping>,
}
impl FInputMappingContextMappingData {}
#[repr(C, align(8))]
pub struct FInputComboStepData {
    pub combo_step_action: UPtr<UInputAction>,
    pub combo_step_completion_states: u8,
    pub time_to_press_key: f32,
}
impl FInputComboStepData {}
#[repr(C, align(8))]
pub struct FInputCancelAction {
    pub cancel_action: UPtr<UInputAction>,
    pub cancellation_states: u8,
    __padding_end: [u8; 7],
}
impl FInputCancelAction {}
#[repr(C, align(8))]
pub struct FKeyMappingRow {
    pub mappings: TSet<FPlayerKeyMapping>,
}
impl FKeyMappingRow {}
#[repr(C, align(8))]
pub struct FPlayerMappableKeyQueryOptions {
    pub mapping_name: FName,
    pub key_to_match: crate::bindings::input_core::FKey,
    pub slot_to_match: EPlayerMappableKeySlot,
    pub flags_49: u8,
    pub required_device_type: crate::bindings::engine::EHardwareDevicePrimaryType,
    pub required_device_flags: i32,
}
impl FPlayerMappableKeyQueryOptions {}
#[repr(C, align(8))]
pub struct UEnhancedInputActionDelegateBinding {
    __padding_end: [u8; 64],
}
impl UEnhancedInputActionDelegateBinding {}
#[repr(C, align(8))]
pub struct UEnhancedInputActionValueBinding {
    __padding_end: [u8; 64],
}
impl UEnhancedInputActionValueBinding {}
#[repr(C, align(8))]
pub struct UEnhancedInputComponent {
    __padding_end: [u8; 432],
}
impl UEnhancedInputComponent {}
#[repr(C, align(8))]
pub struct UEnhancedInputDeveloperSettings {
    __padding_end: [u8; 424],
}
impl UEnhancedInputDeveloperSettings {}
#[repr(C, align(8))]
pub struct UEnhancedInputLibrary {
    __padding_end: [u8; 48],
}
impl UEnhancedInputLibrary {}
#[repr(C, align(8))]
pub struct UEnhancedInputPlatformData {
    #[doc(hidden)]
    __padding_48: [u8; 48],
    pub mapping_context_redirects: TMap<
        UPtr<UInputMappingContext>,
        UPtr<UInputMappingContext>,
    >,
}
impl UEnhancedInputPlatformData {}
#[repr(C, align(8))]
pub struct UEnhancedInputPlatformSettings {
    __padding_end: [u8; 120],
}
impl UEnhancedInputPlatformSettings {}
pub struct UEnhancedInputSubsystemInterface {}
pub struct IEnhancedInputSubsystemInterface {}
#[repr(C, align(8))]
pub struct UEnhancedInputLocalPlayerSubsystem {
    __padding_end: [u8; 576],
}
impl UEnhancedInputLocalPlayerSubsystem {}
#[repr(C, align(8))]
pub struct UEnhancedInputWorldSubsystem {
    __padding_end: [u8; 520],
}
impl UEnhancedInputWorldSubsystem {}
#[repr(C, align(8))]
pub struct UEnhancedPlayerInput {
    __padding_end: [u8; 2312],
}
impl UEnhancedPlayerInput {}
#[repr(C, align(8))]
pub struct UInputAction {
    #[doc(hidden)]
    __padding_120: [u8; 120],
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
impl UInputAction {}
#[repr(C, align(8))]
pub struct UInputDebugKeyDelegateBinding {
    __padding_end: [u8; 64],
}
impl UInputDebugKeyDelegateBinding {}
#[repr(C, align(8))]
pub struct UInputMappingContext {
    #[doc(hidden)]
    __padding_56: [u8; 56],
    pub mappings: TArray<FEnhancedActionKeyMapping>,
    pub default_key_mappings: FInputMappingContextMappingData,
    pub mapping_profile_overrides: TMap<FString, FInputMappingContextMappingData>,
    pub input_mode_filter_options: EMappingContextInputModeFilterOptions,
    pub input_mode_query_override: crate::bindings::gameplay_tags::FGameplayTagQuery,
    pub registration_tracking_mode: EMappingContextRegistrationTrackingMode,
    pub context_description: FText,
}
impl UInputMappingContext {}
#[repr(C, align(8))]
pub struct UInputModifier {
    __padding_end: [u8; 48],
}
impl UInputModifier {}
#[repr(C, align(8))]
pub struct UInputModifierSmoothDelta {
    #[doc(hidden)]
    __padding_48: [u8; 48],
    pub smoothing_method: ENormalizeInputSmoothingType,
    pub speed: f32,
    pub easing_exponent: f32,
    __padding_end: [u8; 52],
}
impl UInputModifierSmoothDelta {}
#[repr(C, align(8))]
pub struct UInputModifierDeadZone {
    #[doc(hidden)]
    __padding_48: [u8; 48],
    pub lower_threshold: f32,
    pub upper_threshold: f32,
    pub ty: EDeadZoneType,
    __padding_end: [u8; 7],
}
impl UInputModifierDeadZone {}
#[repr(C, align(8))]
pub struct UInputModifierScalar {
    #[doc(hidden)]
    __padding_48: [u8; 48],
    pub scalar: crate::bindings::core_u_object::FVector,
}
impl UInputModifierScalar {}
#[repr(C, align(8))]
pub struct UInputModifierScaleByDeltaTime {
    __padding_end: [u8; 48],
}
impl UInputModifierScaleByDeltaTime {}
#[repr(C, align(8))]
pub struct UInputModifierNegate {
    #[doc(hidden)]
    __padding_48: [u8; 48],
    pub b_x: bool,
    pub b_y: bool,
    pub b_z: bool,
    __padding_end: [u8; 5],
}
impl UInputModifierNegate {}
#[repr(C, align(8))]
pub struct UInputModifierSmooth {
    __padding_end: [u8; 96],
}
impl UInputModifierSmooth {}
#[repr(C, align(8))]
pub struct UInputModifierResponseCurveExponential {
    #[doc(hidden)]
    __padding_48: [u8; 48],
    pub curve_exponent: crate::bindings::core_u_object::FVector,
}
impl UInputModifierResponseCurveExponential {}
#[repr(C, align(8))]
pub struct UInputModifierResponseCurveUser {
    #[doc(hidden)]
    __padding_48: [u8; 48],
    pub response_x: UPtr<crate::bindings::engine::UCurveFloat>,
    pub response_y: UPtr<crate::bindings::engine::UCurveFloat>,
    pub response_z: UPtr<crate::bindings::engine::UCurveFloat>,
}
impl UInputModifierResponseCurveUser {}
#[repr(C, align(8))]
pub struct UInputModifierFOVScaling {
    #[doc(hidden)]
    __padding_48: [u8; 48],
    pub fov_scale: f32,
    pub fov_scaling_type: EFOVScalingType,
    __padding_end: [u8; 3],
}
impl UInputModifierFOVScaling {}
#[repr(C, align(8))]
pub struct UInputModifierToWorldSpace {
    __padding_end: [u8; 48],
}
impl UInputModifierToWorldSpace {}
#[repr(C, align(8))]
pub struct UInputModifierSwizzleAxis {
    #[doc(hidden)]
    __padding_48: [u8; 48],
    pub order: EInputAxisSwizzle,
    __padding_end: [u8; 7],
}
impl UInputModifierSwizzleAxis {}
#[repr(C, align(8))]
pub struct UInputTrigger {
    #[doc(hidden)]
    __padding_48: [u8; 48],
    pub actuation_threshold: f32,
    pub b_should_always_tick: bool,
    pub last_value: FInputActionValue,
}
impl UInputTrigger {}
#[repr(C, align(8))]
pub struct UInputTriggerTimedBase {
    #[doc(hidden)]
    __padding_88: [u8; 88],
    pub held_duration: f32,
    pub b_affected_by_time_dilation: bool,
    __padding_end: [u8; 3],
}
impl UInputTriggerTimedBase {}
#[repr(C, align(8))]
pub struct UInputTriggerDown {
    __padding_end: [u8; 88],
}
impl UInputTriggerDown {}
#[repr(C, align(8))]
pub struct UInputTriggerPressed {
    __padding_end: [u8; 88],
}
impl UInputTriggerPressed {}
#[repr(C, align(8))]
pub struct UInputTriggerReleased {
    __padding_end: [u8; 88],
}
impl UInputTriggerReleased {}
#[repr(C, align(8))]
pub struct UInputTriggerHold {
    #[doc(hidden)]
    __padding_100: [u8; 100],
    pub hold_time_threshold: f32,
    pub b_is_one_shot: bool,
    __padding_end: [u8; 7],
}
impl UInputTriggerHold {}
#[repr(C, align(8))]
pub struct UInputTriggerHoldAndRelease {
    #[doc(hidden)]
    __padding_96: [u8; 96],
    pub hold_time_threshold: f32,
    __padding_end: [u8; 4],
}
impl UInputTriggerHoldAndRelease {}
#[repr(C, align(8))]
pub struct UInputTriggerTap {
    #[doc(hidden)]
    __padding_96: [u8; 96],
    pub tap_release_time_threshold: f32,
    __padding_end: [u8; 4],
}
impl UInputTriggerTap {}
#[repr(C, align(8))]
pub struct UInputTriggerRepeatedTap {
    #[doc(hidden)]
    __padding_96: [u8; 96],
    pub repeat_delay: f64,
    #[doc(hidden)]
    __padding_112: [u8; 8],
    pub number_of_taps_which_trigger_repeat: i32,
    pub tap_release_time_threshold: f32,
    __padding_end: [u8; 8],
}
impl UInputTriggerRepeatedTap {}
#[repr(C, align(8))]
pub struct UInputTriggerPulse {
    #[doc(hidden)]
    __padding_100: [u8; 100],
    pub b_trigger_on_start: bool,
    pub interval: f32,
    pub trigger_limit: i32,
}
impl UInputTriggerPulse {}
#[repr(C, align(8))]
pub struct UInputTriggerChordAction {
    #[doc(hidden)]
    __padding_88: [u8; 88],
    pub chord_action: UPtr<UInputAction>,
}
impl UInputTriggerChordAction {}
#[repr(C, align(8))]
pub struct UInputTriggerChordBlocker {
    __padding_end: [u8; 96],
}
impl UInputTriggerChordBlocker {}
#[repr(C, align(8))]
pub struct UInputTriggerCombo {
    #[doc(hidden)]
    __padding_88: [u8; 88],
    pub current_combo_step_index: i32,
    pub current_time_between_combo_steps: f32,
    pub combo_actions: TArray<FInputComboStepData>,
    pub input_cancel_actions: TArray<FInputCancelAction>,
}
impl UInputTriggerCombo {}
#[repr(C, align(8))]
pub struct UPlayerMappableInputConfig {
    #[doc(hidden)]
    __padding_72: [u8; 72],
    pub config_name: FName,
    pub config_display_name: FText,
    pub b_is_deprecated: bool,
    pub metadata: UPtr<crate::bindings::core_u_object::UObject>,
    pub contexts: TMap<UPtr<UInputMappingContext>, i32>,
}
impl UPlayerMappableInputConfig {}
#[repr(C, align(8))]
pub struct UPlayerMappableKeySettings {
    #[doc(hidden)]
    __padding_48: [u8; 48],
    pub metadata: UPtr<crate::bindings::core_u_object::UObject>,
    pub name: FName,
    pub display_name: FText,
    pub display_category: FText,
    #[doc(hidden)]
    __padding_136: [u8; 32],
    pub supported_key_profile_ids: TArray<FString>,
}
impl UPlayerMappableKeySettings {}
#[repr(C, align(8))]
pub struct UEnhancedPlayerMappableKeyProfile {
    #[doc(hidden)]
    __padding_64: [u8; 64],
    pub profile_identifier_string: FString,
    pub owning_user_id: crate::bindings::core_u_object::FPlatformUserId,
    pub display_name: FText,
    pub player_mapped_keys: TMap<FName, FKeyMappingRow>,
}
impl UEnhancedPlayerMappableKeyProfile {}
#[repr(C, align(8))]
pub struct UEnhancedInputUserSettings {
    __padding_end: [u8; 432],
}
impl UEnhancedInputUserSettings {}
#[repr(transparent)]
pub struct FEnhancedInputLocalPlayerSubsystem_ControlMappingsRebuiltDelegate {
    _opague: u8,
}
#[repr(transparent)]
pub struct FEnhancedInputLocalPlayerSubsystem_OnMappingContextAdded {
    _opague: u8,
}
#[repr(transparent)]
pub struct FEnhancedInputLocalPlayerSubsystem_OnMappingContextRemoved {
    _opague: u8,
}
#[repr(transparent)]
pub struct FEnhancedInputLocalPlayerSubsystem_OnPostUserSettingsInitialized {
    _opague: u8,
}
#[repr(transparent)]
pub struct FEnhancedInputUserSettings_OnSettingsChanged {
    _opague: u8,
}
#[repr(transparent)]
pub struct FEnhancedInputUserSettings_OnSettingsApplied {
    _opague: u8,
}
#[repr(transparent)]
pub struct FEnhancedInputUserSettings_OnKeyProfileChanged {
    _opague: u8,
}
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
#[repr(transparent)]
pub struct EInputActionValueType(pub u8);
impl EInputActionValueType {
    pub const BOOLEAN: EInputActionValueType = EInputActionValueType(0);
    pub const AXIS1_D: EInputActionValueType = EInputActionValueType(1);
    pub const AXIS2_D: EInputActionValueType = EInputActionValueType(2);
    pub const AXIS3_D: EInputActionValueType = EInputActionValueType(3);
}
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
#[repr(transparent)]
pub struct EInputMappingRebuildType(pub u8);
impl EInputMappingRebuildType {
    pub const NONE: EInputMappingRebuildType = EInputMappingRebuildType(0);
    pub const REBUILD: EInputMappingRebuildType = EInputMappingRebuildType(1);
    pub const REBUILD_WITH_FLUSH: EInputMappingRebuildType = EInputMappingRebuildType(2);
}
#[repr(transparent)]
pub struct ETriggerType(pub u8);
impl ETriggerType {
    pub const EXPLICIT: ETriggerType = ETriggerType(0);
    pub const IMPLICIT: ETriggerType = ETriggerType(1);
    pub const BLOCKER: ETriggerType = ETriggerType(2);
}
#[repr(transparent)]
pub struct ETriggerState(pub u8);
impl ETriggerState {
    pub const NONE: ETriggerState = ETriggerState(0);
    pub const ONGOING: ETriggerState = ETriggerState(1);
    pub const TRIGGERED: ETriggerState = ETriggerState(2);
}
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
#[repr(transparent)]
pub struct EDeadZoneType(pub u8);
impl EDeadZoneType {
    pub const AXIAL: EDeadZoneType = EDeadZoneType(0);
    pub const RADIAL: EDeadZoneType = EDeadZoneType(1);
    pub const UNSCALED_RADIAL: EDeadZoneType = EDeadZoneType(2);
}
#[repr(transparent)]
pub struct EFOVScalingType(pub u8);
impl EFOVScalingType {
    pub const STANDARD: EFOVScalingType = EFOVScalingType(0);
    pub const UE4_BACK_COMPAT: EFOVScalingType = EFOVScalingType(1);
}
#[repr(transparent)]
pub struct EInputAxisSwizzle(pub u8);
impl EInputAxisSwizzle {
    pub const YXZ: EInputAxisSwizzle = EInputAxisSwizzle(0);
    pub const ZYX: EInputAxisSwizzle = EInputAxisSwizzle(1);
    pub const XZY: EInputAxisSwizzle = EInputAxisSwizzle(2);
    pub const YZX: EInputAxisSwizzle = EInputAxisSwizzle(3);
    pub const ZXY: EInputAxisSwizzle = EInputAxisSwizzle(4);
}
