#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(8))]
pub struct FMetaSoundOutput {
    __padding_end: [u8; 24],
}
impl FMetaSoundOutput {}
#[repr(C, align(8))]
pub struct FMetaSoundAssetDirectory {
    pub directory: crate::bindings::core_u_object::FDirectoryPath,
}
impl FMetaSoundAssetDirectory {}
#[repr(C, align(4))]
pub struct FMetaSoundBuilderNodeInputHandle {
    __padding_end: [u8; 32],
}
impl FMetaSoundBuilderNodeInputHandle {}
#[repr(C, align(4))]
pub struct FMetaSoundBuilderNodeOutputHandle {
    __padding_end: [u8; 32],
}
impl FMetaSoundBuilderNodeOutputHandle {}
#[repr(C, align(4))]
pub struct FMetaSoundNodeHandle {
    __padding_end: [u8; 16],
}
impl FMetaSoundNodeHandle {}
#[repr(C, align(8))]
pub struct FMetaSoundBuilderOptions {
    pub name: FName,
    pub b_force_unique_class_name: bool,
    __padding_end: [u8; 19],
}
impl FMetaSoundBuilderOptions {}
#[repr(C, align(8))]
pub struct UMetasoundEditorGraphBase {
    __padding_end: [u8; 192],
}
impl UMetasoundEditorGraphBase {}
#[repr(C, align(8))]
pub struct UMetaSoundPatch {
    __padding_end: [u8; 1824],
}
impl UMetaSoundPatch {}
#[repr(C, align(8))]
pub struct UMetaSoundAssetSubsystem {
    __padding_end: [u8; 56],
}
impl UMetaSoundAssetSubsystem {}
#[repr(C, align(8))]
pub struct UMetaSoundBuilderBase {
    __padding_end: [u8; 224],
}
impl UMetaSoundBuilderBase {}
#[repr(C, align(8))]
pub struct UMetaSoundPatchBuilder {
    __padding_end: [u8; 224],
}
impl UMetaSoundPatchBuilder {}
#[repr(C, align(8))]
pub struct UMetaSoundSourceBuilder {
    __padding_end: [u8; 264],
}
impl UMetaSoundSourceBuilder {}
#[repr(C, align(8))]
pub struct UMetaSoundBuilderSubsystem {
    __padding_end: [u8; 136],
}
impl UMetaSoundBuilderSubsystem {}
#[repr(C, align(8))]
pub struct UMetasoundFrontendLiteralBlueprintAccess {
    __padding_end: [u8; 48],
}
impl UMetasoundFrontendLiteralBlueprintAccess {}
#[repr(C, align(8))]
pub struct UMetasoundGeneratorHandle {
    __padding_end: [u8; 224],
}
impl UMetasoundGeneratorHandle {}
#[repr(C, align(8))]
pub struct UMetaSoundCacheSubsystem {
    __padding_end: [u8; 128],
}
impl UMetaSoundCacheSubsystem {}
#[repr(C, align(8))]
pub struct UMetasoundOutputBlueprintAccess {
    __padding_end: [u8; 48],
}
impl UMetasoundOutputBlueprintAccess {}
#[repr(C, align(8))]
pub struct UMetaSoundOutputSubsystem {
    __padding_end: [u8; 80],
}
impl UMetaSoundOutputSubsystem {}
#[repr(C, align(8))]
pub struct UMetaSoundQualityHelper {
    __padding_end: [u8; 48],
}
impl UMetaSoundQualityHelper {}
#[repr(C, align(8))]
pub struct UMetaSoundSettings {
    __padding_end: [u8; 664],
}
impl UMetaSoundSettings {}
#[repr(C, align(16))]
pub struct UMetaSoundSource {
    #[doc(hidden)]
    __padding_3880: [u8; 3880],
    pub output_format: EMetaSoundOutputAudioFormat,
    pub quality_setting: FName,
    #[doc(hidden)]
    __padding_3912: [u8; 16],
    pub block_rate_override: crate::bindings::core_u_object::FPerPlatformFloat,
    pub sample_rate_override: crate::bindings::core_u_object::FPerPlatformInt,
    __padding_end: [u8; 440],
}
impl UMetaSoundSource {}
#[repr(transparent)]
pub struct FAudition_OnCreateGenerator {
    _opague: u8,
}
#[repr(transparent)]
pub struct FWatchOutput_OnOutputValueChanged {
    _opague: u8,
}
#[repr(transparent)]
pub struct FUnwatchOutput_OnOutputValueChanged {
    _opague: u8,
}
#[repr(transparent)]
pub struct EMetaSoundBuilderResult(pub u8);
impl EMetaSoundBuilderResult {
    pub const SUCCEEDED: EMetaSoundBuilderResult = EMetaSoundBuilderResult(0);
    pub const FAILED: EMetaSoundBuilderResult = EMetaSoundBuilderResult(1);
}
#[repr(transparent)]
pub struct EMetaSoundOutputAudioFormat(pub u8);
impl EMetaSoundOutputAudioFormat {
    pub const MONO: EMetaSoundOutputAudioFormat = EMetaSoundOutputAudioFormat(0);
    pub const STEREO: EMetaSoundOutputAudioFormat = EMetaSoundOutputAudioFormat(1);
    pub const QUAD: EMetaSoundOutputAudioFormat = EMetaSoundOutputAudioFormat(2);
    pub const FIVE_DOT_ONE: EMetaSoundOutputAudioFormat = EMetaSoundOutputAudioFormat(3);
    pub const SEVEN_DOT_ONE: EMetaSoundOutputAudioFormat = EMetaSoundOutputAudioFormat(
        4,
    );
    pub const COUNT: EMetaSoundOutputAudioFormat = EMetaSoundOutputAudioFormat(5);
}
