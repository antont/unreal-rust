#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(8))]
pub struct FAudioPlatform {
    pub display_name: FString,
    pub module_name: FString,
}
#[repr(C, align(8))]
pub struct FSoundClassGraphSchemaAction_NewNode {}
#[repr(C, align(8))]
pub struct FSoundCueGraphSchemaAction_NewNode {
    pub sound_node_class: TSubclassOf<crate::bindings::engine::USoundNode>,
}
#[repr(C, align(8))]
pub struct FSoundCueGraphSchemaAction_NewFromSelected {}
#[repr(C, align(8))]
pub struct FSoundCueGraphSchemaAction_NewComment {}
#[repr(C, align(8))]
pub struct FSoundCueGraphSchemaAction_Paste {}
#[repr(C, align(8))]
pub struct FSoundSubmixGraphSchemaAction_NewNode {}
pub struct UAssetDefinition_AudioBus {}
pub struct UAssetDefinition_DialogueVoice {}
pub struct UAssetDefinition_ReverbEffect {}
pub struct UAssetDefinition_SoundAttenuation {}
pub struct UAssetDefinition_SoundConcurrency {}
pub struct UAssetDefinition_SoundMix {}
pub struct UAssetDefinition_SoundAssetBase {}
pub struct UAssetDefinition_SoundBase {}
pub struct UAssetDefinition_SoundSourceBus {}
pub struct UAudioEditorPerProjectUserSettings {
    pub use_template_sound_wave_during_asset_import: EUseTemplateSoundWaveDuringAssetImport,
}
pub struct UAssetDefinition_SoundCue {}
pub struct UAssetDefinition_SoundWave {}
pub struct UAudioEditorSettings {
    pub b_use_audio_attenuation: bool,
    pub b_pin_sound_cue_in_asset_menu: bool,
    pub b_pin_sound_cue_template_in_asset_menu: bool,
    pub b_pin_sound_attenuation_in_asset_menu: bool,
    pub b_pin_sound_concurrency_in_asset_menu: bool,
    pub audio_mixer_module_name: FString,
    pub b_use_system_device: bool,
    pub audio_output_device_id: FString,
    pub audio_mixer_platforms: TArray<FAudioPlatform>,
    pub b_use_submixes_for_preview_sound: bool,
    pub menu_position: FName,
}
pub struct UAudioBusFactory {}
pub struct UDialogueVoiceFactory {}
pub struct UDialogueWaveFactory {
    pub initial_sound_wave: UPtr<crate::bindings::engine::USoundWave>,
    pub initial_speaker_voice: UPtr<crate::bindings::engine::UDialogueVoice>,
    pub has_set_initial_target_voice: bool,
    pub initial_target_voices: TArray<UPtr<crate::bindings::engine::UDialogueVoice>>,
}
pub struct USoundFactory {
    pub flags_136: u8,
    pub cue_volume: f32,
    pub cue_package_suffix: FString,
}
pub struct UReimportSoundFactory {}
pub struct UReverbEffectFactory {}
pub struct USoundAttenuationFactory {}
pub struct USoundClassFactory {}
pub struct USoundConcurrencyFactory {}
pub struct USoundCueFactoryNew {
    pub initial_sound_wave: UPtr<crate::bindings::engine::USoundWave>,
    pub initial_sound_waves: TArray<TWeakObjectPtr<crate::bindings::engine::USoundWave>>,
    pub initial_dialogue_wave: UPtr<crate::bindings::engine::UDialogueWave>,
    pub initial_dialogue_waves: TArray<
        TWeakObjectPtr<crate::bindings::engine::UDialogueWave>,
    >,
}
pub struct USoundMixFactory {}
pub struct USoundSourceBusFactory {}
pub struct USoundSourceEffectFactory {
    pub sound_effect_sourcepreset_class: TSubclassOf<
        crate::bindings::engine::USoundEffectSourcePreset,
    >,
}
pub struct USoundSourceEffectChainFactory {}
pub struct USoundSubmixEffectFactory {
    pub sound_effect_submix_preset_class: TSubclassOf<
        crate::bindings::engine::USoundEffectSubmixPreset,
    >,
}
pub struct USoundSubmixFactory {}
pub struct USoundfieldSubmixFactory {}
pub struct UEndpointSubmixFactory {}
pub struct USoundfieldEndpointSubmixFactory {}
pub struct USoundClassGraph {}
pub struct USoundClassGraphNode {
    pub sound_class: UPtr<crate::bindings::engine::USoundClass>,
}
pub struct USoundClassGraphSchema {}
pub struct USoundEffectSourcePresetClassTemplate {}
pub struct USoundEffectSubmixPresetClassTemplate {}
pub struct USynthComponentClassTemplate {}
pub struct USoundCueGraph {}
pub struct USoundCueGraphNode_Base {}
pub struct USoundCueGraphNode {
    pub sound_node: UPtr<crate::bindings::engine::USoundNode>,
}
pub struct USoundCueGraphNode_Root {}
pub struct USoundCueGraphSchema {}
pub struct USoundSubmixGraph {
    pub root_sound_submix: UPtr<crate::bindings::engine::USoundSubmixBase>,
    pub stale_roots: TArray<UPtr<crate::bindings::engine::USoundSubmixBase>>,
}
pub struct USoundSubmixGraphNode {
    pub sound_submix: UPtr<crate::bindings::engine::USoundSubmixBase>,
    pub submix_node_user_widget: UPtr<crate::bindings::umg::UUserWidget>,
}
pub struct USoundSubmixGraphSchema {}
#[allow(non_camel_case_types)]
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
