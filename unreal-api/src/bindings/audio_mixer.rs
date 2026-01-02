#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(8))]
pub struct FAudioOutputDeviceInfo {
    pub name: FString,
    pub device_id: FString,
    pub num_channels: i32,
    pub sample_rate: i32,
    pub format: EAudioMixerStreamDataFormatType,
    pub output_channel_array: TArray<EAudioMixerChannelType>,
    pub flags_64: u8,
    __padding_end: [u8; 7],
}
impl FAudioOutputDeviceInfo {}
#[repr(C, align(8))]
pub struct FSwapAudioOutputResult {
    pub current_device_id: FString,
    pub requested_device_id: FString,
    pub result: ESwapAudioOutputDeviceResultState,
    __padding_end: [u8; 7],
}
impl FSwapAudioOutputResult {}
#[repr(C, align(4))]
pub struct FSubmixEffectDynamicProcessorFilterSettings {
    pub flags_0: u8,
    pub cutoff: f32,
    pub gain_db: f32,
}
impl FSubmixEffectDynamicProcessorFilterSettings {}
#[repr(C, align(8))]
pub struct FSubmixEffectDynamicsProcessorSettings {
    pub dynamics_processor_type: ESubmixEffectDynamicsProcessorType,
    pub peak_mode: ESubmixEffectDynamicsPeakMode,
    pub link_mode: ESubmixEffectDynamicsChannelLinkMode,
    pub input_gain_db: f32,
    pub threshold_db: f32,
    pub ratio: f32,
    pub knee_bandwidth_db: f32,
    pub look_ahead_msec: f32,
    pub attack_time_msec: f32,
    pub release_time_msec: f32,
    pub key_source: ESubmixEffectDynamicsKeySource,
    pub external_audio_bus: UPtr<crate::bindings::engine::UAudioBus>,
    pub external_submix: UPtr<crate::bindings::engine::USoundSubmix>,
    pub flags_56: u8,
    pub key_gain_db: f32,
    pub output_gain_db: f32,
    pub key_highshelf: FSubmixEffectDynamicProcessorFilterSettings,
    pub key_lowshelf: FSubmixEffectDynamicProcessorFilterSettings,
    __padding_end: [u8; 4],
}
impl FSubmixEffectDynamicsProcessorSettings {}
#[repr(C, align(4))]
pub struct FSubmixEffectEQBand {
    pub frequency: f32,
    pub bandwidth: f32,
    pub gain_db: f32,
    pub flags_12: u8,
    __padding_end: [u8; 3],
}
impl FSubmixEffectEQBand {}
#[repr(C, align(8))]
pub struct FSubmixEffectSubmixEQSettings {
    pub eq_bands: TArray<FSubmixEffectEQBand>,
}
impl FSubmixEffectSubmixEQSettings {}
#[repr(C, align(4))]
pub struct FSubmixEffectReverbSettings {
    pub b_bypass_early_reflections: bool,
    pub reflections_delay: f32,
    pub gain_hf: f32,
    pub reflections_gain: f32,
    pub b_bypass_late_reflections: bool,
    pub late_delay: f32,
    pub decay_time: f32,
    pub density: f32,
    pub diffusion: f32,
    pub air_absorption_gain_hf: f32,
    pub decay_hf_ratio: f32,
    pub late_gain: f32,
    pub gain: f32,
    pub wet_level: f32,
    pub dry_level: f32,
    pub b_bypass: bool,
    __padding_end: [u8; 3],
}
impl FSubmixEffectReverbSettings {}
#[repr(C, align(8))]
pub struct UAudioBusSubsystem {
    __padding_end: [u8; 272],
}
impl UAudioBusSubsystem {}
#[repr(C, align(8))]
pub struct UAudioDeviceNotificationSubsystem {
    __padding_end: [u8; 400],
}
impl UAudioDeviceNotificationSubsystem {}
#[repr(C, align(8))]
pub struct UAudioMixerBlueprintLibrary {
    __padding_end: [u8; 48],
}
impl UAudioMixerBlueprintLibrary {}
#[repr(C, align(16))]
pub struct USynthSound {
    __padding_end: [u8; 2192],
}
impl USynthSound {}
#[repr(C, align(16))]
pub struct USynthComponent {
    #[doc(hidden)]
    __padding_656: [u8; 656],
    pub flags_656: u8,
    #[doc(hidden)]
    __padding_660: [u8; 3],
    pub flags_660: u8,
    pub attenuation_settings: UPtr<crate::bindings::engine::USoundAttenuation>,
    pub attenuation_overrides: crate::bindings::engine::FSoundAttenuationSettings,
    #[doc(hidden)]
    __padding_1704: [u8; 8],
    pub concurrency_set: TSet<UPtr<crate::bindings::engine::USoundConcurrency>>,
    pub modulation_routing: crate::bindings::engine::FSoundModulationDefaultRoutingSettings,
    #[doc(hidden)]
    __padding_2200: [u8; 24],
    pub sound_submix_sends: TArray<crate::bindings::engine::FSoundSubmixSendInfo>,
    pub bus_sends: TArray<crate::bindings::engine::FSoundSourceBusSendInfo>,
    pub pre_effect_bus_sends: TArray<crate::bindings::engine::FSoundSourceBusSendInfo>,
    pub flags_2248: u8,
    pub envelope_follower_attack_time: i32,
    pub envelope_follower_release_time: i32,
    __padding_end: [u8; 124],
}
impl USynthComponent {}
#[repr(C, align(8))]
pub struct USubmixEffectDynamicsProcessorPreset {
    #[doc(hidden)]
    __padding_248: [u8; 248],
    pub settings: FSubmixEffectDynamicsProcessorSettings,
}
impl USubmixEffectDynamicsProcessorPreset {}
#[repr(C, align(8))]
pub struct USubmixEffectSubmixEQPreset {
    #[doc(hidden)]
    __padding_168: [u8; 168],
    pub settings: FSubmixEffectSubmixEQSettings,
}
impl USubmixEffectSubmixEQPreset {}
#[repr(C, align(8))]
pub struct USubmixEffectReverbPreset {
    #[doc(hidden)]
    __padding_216: [u8; 216],
    pub settings: FSubmixEffectReverbSettings,
}
impl USubmixEffectReverbPreset {}
#[repr(C, align(8))]
pub struct UAudioGenerator {
    __padding_end: [u8; 176],
}
impl UAudioGenerator {}
#[repr(C, align(8))]
pub struct UScrubbedSound {
    __padding_end: [u8; 2136],
}
impl UScrubbedSound {}
#[repr(C, align(8))]
pub struct UQuartzClockHandle {
    __padding_end: [u8; 688],
}
impl UQuartzClockHandle {}
#[repr(C, align(8))]
pub struct UQuartzSubsystem {
    __padding_end: [u8; 120],
}
impl UQuartzSubsystem {}
#[repr(C, align(8))]
pub struct FGetAvailableAudioOutputDevices_OnObtainDevicesEvent {
    _opague: [u8; 32],
}
#[repr(C, align(8))]
pub struct FGetCurrentAudioOutputDeviceName_OnObtainCurrentDeviceEvent {
    _opague: [u8; 32],
}
#[repr(C, align(8))]
pub struct FPrimeSoundForPlayback_OnLoadCompletion {
    _opague: [u8; 32],
}
#[repr(C, align(8))]
pub struct FSwapAudioOutputDevice_OnCompletedDeviceSwap {
    _opague: [u8; 32],
}
#[repr(C, align(8))]
pub struct FNotifyOnQuantizationBoundary_InDelegate {
    _opague: [u8; 32],
}
#[repr(C, align(8))]
pub struct FResetTransport_InDelegate {
    _opague: [u8; 32],
}
#[repr(C, align(8))]
pub struct FResetTransportQuantized_InDelegate {
    _opague: [u8; 32],
}
#[repr(C, align(8))]
pub struct FSetBeatsPerMinute_Delegate {
    _opague: [u8; 32],
}
#[repr(C, align(8))]
pub struct FSetMillisecondsPerTick_Delegate {
    _opague: [u8; 32],
}
#[repr(C, align(8))]
pub struct FSetSecondsPerTick_Delegate {
    _opague: [u8; 32],
}
#[repr(C, align(8))]
pub struct FSetThirtySecondNotesPerMinute_Delegate {
    _opague: [u8; 32],
}
#[repr(C, align(8))]
pub struct FSetTicksPerSecond_Delegate {
    _opague: [u8; 32],
}
#[repr(C, align(8))]
pub struct FStartOtherClock_InDelegate {
    _opague: [u8; 32],
}
#[repr(C, align(8))]
pub struct FSubscribeToAllQuantizationEvents_OnQuantizationEvent {
    _opague: [u8; 32],
}
#[repr(C, align(8))]
pub struct FSubscribeToQuantizationEvent_OnQuantizationEvent {
    _opague: [u8; 32],
}
#[repr(C, align(8))]
pub struct FAudioDeviceNotificationSubsystem_DefaultCaptureDeviceChanged {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FAudioDeviceNotificationSubsystem_DefaultRenderDeviceChanged {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FAudioDeviceNotificationSubsystem_DeviceAdded {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FAudioDeviceNotificationSubsystem_DeviceRemoved {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FAudioDeviceNotificationSubsystem_DeviceStateChanged {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FAudioDeviceNotificationSubsystem_DeviceSwitched {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FSynthComponent_OnAudioEnvelopeValue {
    _opague: [u8; 24],
}
#[repr(transparent)]
pub struct EAudioMixerStreamDataFormatType(pub u8);
impl EAudioMixerStreamDataFormatType {
    pub const UNKNOWN: EAudioMixerStreamDataFormatType = EAudioMixerStreamDataFormatType(
        0,
    );
    pub const FLOAT: EAudioMixerStreamDataFormatType = EAudioMixerStreamDataFormatType(
        1,
    );
    pub const INT16: EAudioMixerStreamDataFormatType = EAudioMixerStreamDataFormatType(
        2,
    );
    pub const UNSUPPORTED: EAudioMixerStreamDataFormatType = EAudioMixerStreamDataFormatType(
        3,
    );
}
#[repr(transparent)]
pub struct EAudioMixerChannelType(pub u8);
impl EAudioMixerChannelType {
    pub const FRONT_LEFT: EAudioMixerChannelType = EAudioMixerChannelType(0);
    pub const FRONT_RIGHT: EAudioMixerChannelType = EAudioMixerChannelType(1);
    pub const FRONT_CENTER: EAudioMixerChannelType = EAudioMixerChannelType(2);
    pub const LOW_FREQUENCY: EAudioMixerChannelType = EAudioMixerChannelType(3);
    pub const BACK_LEFT: EAudioMixerChannelType = EAudioMixerChannelType(4);
    pub const BACK_RIGHT: EAudioMixerChannelType = EAudioMixerChannelType(5);
    pub const FRONT_LEFT_OF_CENTER: EAudioMixerChannelType = EAudioMixerChannelType(6);
    pub const FRONT_RIGHT_OF_CENTER: EAudioMixerChannelType = EAudioMixerChannelType(7);
    pub const BACK_CENTER: EAudioMixerChannelType = EAudioMixerChannelType(8);
    pub const SIDE_LEFT: EAudioMixerChannelType = EAudioMixerChannelType(9);
    pub const SIDE_RIGHT: EAudioMixerChannelType = EAudioMixerChannelType(10);
    pub const TOP_CENTER: EAudioMixerChannelType = EAudioMixerChannelType(11);
    pub const TOP_FRONT_LEFT: EAudioMixerChannelType = EAudioMixerChannelType(12);
    pub const TOP_FRONT_CENTER: EAudioMixerChannelType = EAudioMixerChannelType(13);
    pub const TOP_FRONT_RIGHT: EAudioMixerChannelType = EAudioMixerChannelType(14);
    pub const TOP_BACK_LEFT: EAudioMixerChannelType = EAudioMixerChannelType(15);
    pub const TOP_BACK_CENTER: EAudioMixerChannelType = EAudioMixerChannelType(16);
    pub const TOP_BACK_RIGHT: EAudioMixerChannelType = EAudioMixerChannelType(17);
    pub const UNKNOWN: EAudioMixerChannelType = EAudioMixerChannelType(18);
    pub const CHANNEL_TYPE_COUNT: EAudioMixerChannelType = EAudioMixerChannelType(19);
    pub const DEFAULT_CHANNEL: EAudioMixerChannelType = EAudioMixerChannelType(0);
}
#[repr(transparent)]
pub struct ESwapAudioOutputDeviceResultState(pub u8);
impl ESwapAudioOutputDeviceResultState {
    pub const FAILURE: ESwapAudioOutputDeviceResultState = ESwapAudioOutputDeviceResultState(
        0,
    );
    pub const SUCCESS: ESwapAudioOutputDeviceResultState = ESwapAudioOutputDeviceResultState(
        1,
    );
    pub const NONE: ESwapAudioOutputDeviceResultState = ESwapAudioOutputDeviceResultState(
        2,
    );
}
#[repr(transparent)]
pub struct ESubmixEffectDynamicsProcessorType(pub u8);
impl ESubmixEffectDynamicsProcessorType {
    pub const COMPRESSOR: ESubmixEffectDynamicsProcessorType = ESubmixEffectDynamicsProcessorType(
        0,
    );
    pub const LIMITER: ESubmixEffectDynamicsProcessorType = ESubmixEffectDynamicsProcessorType(
        1,
    );
    pub const EXPANDER: ESubmixEffectDynamicsProcessorType = ESubmixEffectDynamicsProcessorType(
        2,
    );
    pub const GATE: ESubmixEffectDynamicsProcessorType = ESubmixEffectDynamicsProcessorType(
        3,
    );
    pub const UPWARDS_COMPRESSOR: ESubmixEffectDynamicsProcessorType = ESubmixEffectDynamicsProcessorType(
        4,
    );
    pub const COUNT: ESubmixEffectDynamicsProcessorType = ESubmixEffectDynamicsProcessorType(
        5,
    );
}
#[repr(transparent)]
pub struct ESubmixEffectDynamicsPeakMode(pub u8);
impl ESubmixEffectDynamicsPeakMode {
    pub const MEAN_SQUARED: ESubmixEffectDynamicsPeakMode = ESubmixEffectDynamicsPeakMode(
        0,
    );
    pub const ROOT_MEAN_SQUARED: ESubmixEffectDynamicsPeakMode = ESubmixEffectDynamicsPeakMode(
        1,
    );
    pub const PEAK: ESubmixEffectDynamicsPeakMode = ESubmixEffectDynamicsPeakMode(2);
    pub const COUNT: ESubmixEffectDynamicsPeakMode = ESubmixEffectDynamicsPeakMode(3);
}
#[repr(transparent)]
pub struct ESubmixEffectDynamicsChannelLinkMode(pub u8);
impl ESubmixEffectDynamicsChannelLinkMode {
    pub const DISABLED: ESubmixEffectDynamicsChannelLinkMode = ESubmixEffectDynamicsChannelLinkMode(
        0,
    );
    pub const AVERAGE: ESubmixEffectDynamicsChannelLinkMode = ESubmixEffectDynamicsChannelLinkMode(
        1,
    );
    pub const PEAK: ESubmixEffectDynamicsChannelLinkMode = ESubmixEffectDynamicsChannelLinkMode(
        2,
    );
    pub const COUNT: ESubmixEffectDynamicsChannelLinkMode = ESubmixEffectDynamicsChannelLinkMode(
        3,
    );
}
#[repr(transparent)]
pub struct ESubmixEffectDynamicsKeySource(pub u8);
impl ESubmixEffectDynamicsKeySource {
    pub const DEFAULT: ESubmixEffectDynamicsKeySource = ESubmixEffectDynamicsKeySource(
        0,
    );
    pub const AUDIO_BUS: ESubmixEffectDynamicsKeySource = ESubmixEffectDynamicsKeySource(
        1,
    );
    pub const SUBMIX: ESubmixEffectDynamicsKeySource = ESubmixEffectDynamicsKeySource(2);
    pub const COUNT: ESubmixEffectDynamicsKeySource = ESubmixEffectDynamicsKeySource(3);
}
#[repr(transparent)]
pub struct EMusicalNoteName(pub u8);
impl EMusicalNoteName {
    pub const C: EMusicalNoteName = EMusicalNoteName(0);
    pub const DB: EMusicalNoteName = EMusicalNoteName(1);
    pub const D: EMusicalNoteName = EMusicalNoteName(2);
    pub const EB: EMusicalNoteName = EMusicalNoteName(3);
    pub const E: EMusicalNoteName = EMusicalNoteName(4);
    pub const F: EMusicalNoteName = EMusicalNoteName(5);
    pub const GB: EMusicalNoteName = EMusicalNoteName(6);
    pub const G: EMusicalNoteName = EMusicalNoteName(7);
    pub const AB: EMusicalNoteName = EMusicalNoteName(8);
    pub const A: EMusicalNoteName = EMusicalNoteName(9);
    pub const BB: EMusicalNoteName = EMusicalNoteName(10);
    pub const B: EMusicalNoteName = EMusicalNoteName(11);
}
#[repr(transparent)]
pub struct EAudioDeviceChangedRole(pub u8);
impl EAudioDeviceChangedRole {
    pub const INVALID: EAudioDeviceChangedRole = EAudioDeviceChangedRole(0);
    pub const CONSOLE: EAudioDeviceChangedRole = EAudioDeviceChangedRole(1);
    pub const MULTIMEDIA: EAudioDeviceChangedRole = EAudioDeviceChangedRole(2);
    pub const COMMUNICATIONS: EAudioDeviceChangedRole = EAudioDeviceChangedRole(3);
    pub const COUNT: EAudioDeviceChangedRole = EAudioDeviceChangedRole(4);
}
#[repr(transparent)]
pub struct EAudioDeviceChangedState(pub u8);
impl EAudioDeviceChangedState {
    pub const INVALID: EAudioDeviceChangedState = EAudioDeviceChangedState(0);
    pub const ACTIVE: EAudioDeviceChangedState = EAudioDeviceChangedState(1);
    pub const DISABLED: EAudioDeviceChangedState = EAudioDeviceChangedState(2);
    pub const NOT_PRESENT: EAudioDeviceChangedState = EAudioDeviceChangedState(3);
    pub const UNPLUGGED: EAudioDeviceChangedState = EAudioDeviceChangedState(4);
    pub const COUNT: EAudioDeviceChangedState = EAudioDeviceChangedState(5);
}
