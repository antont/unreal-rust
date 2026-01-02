#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(8))]
pub struct UAssetDefinition_AudioBus {
    __padding_end: [u8; 72],
}
impl UAssetDefinition_AudioBus {}
#[repr(C, align(8))]
pub struct UAssetDefinition_DialogueVoice {
    __padding_end: [u8; 72],
}
impl UAssetDefinition_DialogueVoice {}
#[repr(C, align(8))]
pub struct UAssetDefinition_ReverbEffect {
    __padding_end: [u8; 72],
}
impl UAssetDefinition_ReverbEffect {}
#[repr(C, align(8))]
pub struct UAssetDefinition_SoundAttenuation {
    __padding_end: [u8; 72],
}
impl UAssetDefinition_SoundAttenuation {}
#[repr(C, align(8))]
pub struct UAssetDefinition_SoundConcurrency {
    __padding_end: [u8; 72],
}
impl UAssetDefinition_SoundConcurrency {}
#[repr(C, align(8))]
pub struct UAssetDefinition_SoundMix {
    __padding_end: [u8; 72],
}
impl UAssetDefinition_SoundMix {}
#[repr(C, align(8))]
pub struct UAssetDefinition_SoundAssetBase {
    __padding_end: [u8; 72],
}
impl UAssetDefinition_SoundAssetBase {}
#[repr(C, align(8))]
pub struct UAssetDefinition_SoundBase {
    __padding_end: [u8; 72],
}
impl UAssetDefinition_SoundBase {}
#[repr(C, align(8))]
pub struct UAssetDefinition_SoundSourceBus {
    __padding_end: [u8; 72],
}
impl UAssetDefinition_SoundSourceBus {}
#[repr(C, align(8))]
pub struct UAudioEditorPerProjectUserSettings {
    __padding_end: [u8; 112],
}
impl UAudioEditorPerProjectUserSettings {}
#[repr(C, align(8))]
pub struct UAssetDefinition_SoundCue {
    __padding_end: [u8; 72],
}
impl UAssetDefinition_SoundCue {}
#[repr(C, align(8))]
pub struct UAssetDefinition_SoundWave {
    __padding_end: [u8; 72],
}
impl UAssetDefinition_SoundWave {}
#[repr(C, align(8))]
pub struct UAudioEditorSettings {
    __padding_end: [u8; 184],
}
impl UAudioEditorSettings {}
#[repr(C, align(8))]
pub struct UAudioBusFactory {
    __padding_end: [u8; 136],
}
impl UAudioBusFactory {}
#[repr(C, align(8))]
pub struct UDialogueVoiceFactory {
    __padding_end: [u8; 136],
}
impl UDialogueVoiceFactory {}
#[repr(C, align(8))]
pub struct UDialogueWaveFactory {
    __padding_end: [u8; 176],
}
impl UDialogueWaveFactory {}
#[repr(C, align(8))]
pub struct USoundFactory {
    __padding_end: [u8; 176],
}
impl USoundFactory {}
#[repr(C, align(8))]
pub struct UReimportSoundFactory {
    __padding_end: [u8; 216],
}
impl UReimportSoundFactory {}
#[repr(C, align(8))]
pub struct UReverbEffectFactory {
    __padding_end: [u8; 136],
}
impl UReverbEffectFactory {}
#[repr(C, align(8))]
pub struct USoundAttenuationFactory {
    __padding_end: [u8; 136],
}
impl USoundAttenuationFactory {}
#[repr(C, align(8))]
pub struct USoundClassFactory {
    __padding_end: [u8; 136],
}
impl USoundClassFactory {}
#[repr(C, align(8))]
pub struct USoundConcurrencyFactory {
    __padding_end: [u8; 136],
}
impl USoundConcurrencyFactory {}
#[repr(C, align(8))]
pub struct USoundCueFactoryNew {
    __padding_end: [u8; 184],
}
impl USoundCueFactoryNew {}
#[repr(C, align(8))]
pub struct USoundMixFactory {
    __padding_end: [u8; 136],
}
impl USoundMixFactory {}
#[repr(C, align(8))]
pub struct USoundSourceBusFactory {
    __padding_end: [u8; 136],
}
impl USoundSourceBusFactory {}
#[repr(C, align(8))]
pub struct USoundSourceEffectFactory {
    __padding_end: [u8; 144],
}
impl USoundSourceEffectFactory {}
#[repr(C, align(8))]
pub struct USoundSourceEffectChainFactory {
    __padding_end: [u8; 136],
}
impl USoundSourceEffectChainFactory {}
#[repr(C, align(8))]
pub struct USoundSubmixEffectFactory {
    __padding_end: [u8; 144],
}
impl USoundSubmixEffectFactory {}
#[repr(C, align(8))]
pub struct USoundSubmixFactory {
    __padding_end: [u8; 136],
}
impl USoundSubmixFactory {}
#[repr(C, align(8))]
pub struct USoundfieldSubmixFactory {
    __padding_end: [u8; 136],
}
impl USoundfieldSubmixFactory {}
#[repr(C, align(8))]
pub struct UEndpointSubmixFactory {
    __padding_end: [u8; 136],
}
impl UEndpointSubmixFactory {}
#[repr(C, align(8))]
pub struct USoundfieldEndpointSubmixFactory {
    __padding_end: [u8; 136],
}
impl USoundfieldEndpointSubmixFactory {}
#[repr(C, align(8))]
pub struct USoundClassGraph {
    __padding_end: [u8; 200],
}
impl USoundClassGraph {}
#[repr(C, align(8))]
pub struct USoundClassGraphNode {
    __padding_end: [u8; 216],
}
impl USoundClassGraphNode {}
#[repr(C, align(8))]
pub struct USoundClassGraphSchema {
    __padding_end: [u8; 48],
}
impl USoundClassGraphSchema {}
#[repr(C, align(8))]
pub struct USoundEffectSourcePresetClassTemplate {
    __padding_end: [u8; 56],
}
impl USoundEffectSourcePresetClassTemplate {}
#[repr(C, align(8))]
pub struct USoundEffectSubmixPresetClassTemplate {
    __padding_end: [u8; 56],
}
impl USoundEffectSubmixPresetClassTemplate {}
#[repr(C, align(8))]
pub struct USynthComponentClassTemplate {
    __padding_end: [u8; 56],
}
impl USynthComponentClassTemplate {}
#[repr(C, align(8))]
pub struct USoundCueGraph {
    __padding_end: [u8; 192],
}
impl USoundCueGraph {}
#[repr(C, align(8))]
pub struct USoundCueGraphNode_Base {
    __padding_end: [u8; 192],
}
impl USoundCueGraphNode_Base {}
#[repr(C, align(8))]
pub struct USoundCueGraphNode {
    __padding_end: [u8; 200],
}
impl USoundCueGraphNode {}
#[repr(C, align(8))]
pub struct USoundCueGraphNode_Root {
    __padding_end: [u8; 192],
}
impl USoundCueGraphNode_Root {}
#[repr(C, align(8))]
pub struct USoundCueGraphSchema {
    __padding_end: [u8; 64],
}
impl USoundCueGraphSchema {}
#[repr(C, align(8))]
pub struct USoundSubmixGraph {
    __padding_end: [u8; 216],
}
impl USoundSubmixGraph {}
#[repr(C, align(8))]
pub struct USoundSubmixGraphNode {
    __padding_end: [u8; 224],
}
impl USoundSubmixGraphNode {}
#[repr(C, align(8))]
pub struct USoundSubmixGraphSchema {
    __padding_end: [u8; 48],
}
impl USoundSubmixGraphSchema {}
#[repr(transparent)]
pub struct EUseTemplateSoundWaveDuringAssetImport(pub u8);
impl EUseTemplateSoundWaveDuringAssetImport {
    pub const ALWAYS_PROMPT: EUseTemplateSoundWaveDuringAssetImport = EUseTemplateSoundWaveDuringAssetImport(
        0,
    );
    pub const ALWAYS_USE: EUseTemplateSoundWaveDuringAssetImport = EUseTemplateSoundWaveDuringAssetImport(
        1,
    );
    pub const NEVER_USE: EUseTemplateSoundWaveDuringAssetImport = EUseTemplateSoundWaveDuringAssetImport(
        2,
    );
}
