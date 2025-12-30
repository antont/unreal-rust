#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::prelude::*;
#[repr(C, align(8))]
pub struct FAudioOutputDeviceInfo {
    pub name: FString,
    pub device_id: FString,
    pub num_channels: i32,
    pub sample_rate: i32,
    pub format: EAudioMixerStreamDataFormatType,
    pub output_channel_array: TArray<EAudioMixerChannelType>,
    pub flags_64: u8,
}
#[repr(C, align(8))]
pub struct FSwapAudioOutputResult {
    pub current_device_id: FString,
    pub requested_device_id: FString,
    pub result: ESwapAudioOutputDeviceResultState,
}
#[repr(C, align(4))]
pub struct FSubmixEffectDynamicProcessorFilterSettings {
    pub flags_0: u8,
    pub cutoff: f32,
    pub gain_db: f32,
}
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
    pub external_audio_bus: UPtr<UAudioBus>,
    pub external_submix: UPtr<USoundSubmix>,
    pub flags_56: u8,
    pub key_gain_db: f32,
    pub output_gain_db: f32,
    pub key_highshelf: FSubmixEffectDynamicProcessorFilterSettings,
    pub key_lowshelf: FSubmixEffectDynamicProcessorFilterSettings,
}
#[repr(C, align(4))]
pub struct FSubmixEffectEQBand {
    pub frequency: f32,
    pub bandwidth: f32,
    pub gain_db: f32,
    pub flags_12: u8,
}
#[repr(C, align(8))]
pub struct FSubmixEffectSubmixEQSettings {
    pub eq_bands: TArray<FSubmixEffectEQBand>,
}
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
}
pub struct UAudioBusSubsystem {}
pub struct UAudioDeviceNotificationSubsystem {
    pub default_capture_device_changed: FAudioDeviceNotificationSubsystem_DefaultCaptureDeviceChanged,
    pub default_render_device_changed: FAudioDeviceNotificationSubsystem_DefaultRenderDeviceChanged,
    pub device_added: FAudioDeviceNotificationSubsystem_DeviceAdded,
    pub device_removed: FAudioDeviceNotificationSubsystem_DeviceRemoved,
    pub device_state_changed: FAudioDeviceNotificationSubsystem_DeviceStateChanged,
    pub device_switched: FAudioDeviceNotificationSubsystem_DeviceSwitched,
}
pub struct UAudioMixerBlueprintLibrary {}
pub struct USynthSound {
    pub owning_synth_component: TWeakObjectPtr<USynthComponent>,
}
pub struct USynthComponent {
    pub flags_656: u8,
    pub flags_660: u8,
    pub attenuation_settings: UPtr<USoundAttenuation>,
    pub attenuation_overrides: FSoundAttenuationSettings,
    pub concurrency_settings_deprecated: UPtr<USoundConcurrency>,
    pub concurrency_set: TSet<UPtr<USoundConcurrency>>,
    pub modulation_routing: FSoundModulationDefaultRoutingSettings,
    pub sound_class: UPtr<USoundClass>,
    pub source_effect_chain: UPtr<USoundEffectSourcePresetChain>,
    pub sound_submix: UPtr<USoundSubmixBase>,
    pub sound_submix_sends: TArray<FSoundSubmixSendInfo>,
    pub bus_sends: TArray<FSoundSourceBusSendInfo>,
    pub pre_effect_bus_sends: TArray<FSoundSourceBusSendInfo>,
    pub flags_2248: u8,
    pub envelope_follower_attack_time: i32,
    pub envelope_follower_release_time: i32,
    pub on_audio_envelope_value: FSynthComponent_OnAudioEnvelopeValue,
    pub synth: UPtr<USynthSound>,
    pub audio_component: UPtr<UAudioComponent>,
}
pub struct USubmixEffectDynamicsProcessorPreset {
    pub settings: FSubmixEffectDynamicsProcessorSettings,
}
pub struct USubmixEffectSubmixEQPreset {
    pub settings: FSubmixEffectSubmixEQSettings,
}
pub struct USubmixEffectReverbPreset {
    pub settings: FSubmixEffectReverbSettings,
}
pub struct UAudioGenerator {}
pub struct UScrubbedSound {
    pub sound_wave_to_scrub: UPtr<USoundWave>,
}
pub struct UQuartzClockHandle {}
pub struct UQuartzSubsystem {}
