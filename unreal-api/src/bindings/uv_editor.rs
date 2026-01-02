#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(8))]
pub struct UUVEditorInitializationContext {
    __padding_end: [u8; 72],
}
impl UUVEditorInitializationContext {}
#[repr(C, align(8))]
pub struct UUVEditorUnwrappedUXProperties {
    __padding_end: [u8; 208],
}
impl UUVEditorUnwrappedUXProperties {}
#[repr(C, align(8))]
pub struct UUVEditorLivePreviewUXProperties {
    __padding_end: [u8; 200],
}
impl UUVEditorLivePreviewUXProperties {}
#[repr(C, align(8))]
pub struct UUnsetUVsAction {
    __padding_end: [u8; 72],
}
impl UUnsetUVsAction {}
#[repr(C, align(8))]
pub struct UUVEditor {
    __padding_end: [u8; 96],
}
impl UUVEditor {}
#[repr(C, align(8))]
pub struct UUVEditor2DViewportContext {
    __padding_end: [u8; 192],
}
impl UUVEditor2DViewportContext {}
#[repr(C, align(8))]
pub struct UUVEditor3DViewportMode {
    __padding_end: [u8; 312],
}
impl UUVEditor3DViewportMode {}
#[repr(C, align(8))]
pub struct UUVEditorBackgroundPreviewProperties {
    __padding_end: [u8; 232],
}
impl UUVEditorBackgroundPreviewProperties {}
#[repr(C, align(8))]
pub struct UUVEditorBackgroundPreview {
    __padding_end: [u8; 360],
}
impl UUVEditorBackgroundPreview {}
#[repr(C, align(8))]
pub struct UUVEditorDistortionVisualizationProperties {
    __padding_end: [u8; 280],
}
impl UUVEditorDistortionVisualizationProperties {}
#[repr(C, align(8))]
pub struct UUVEditorDistortionVisualization {
    __padding_end: [u8; 104],
}
impl UUVEditorDistortionVisualization {}
#[repr(C, align(8))]
pub struct UUVEditorGridProperties {
    __padding_end: [u8; 192],
}
impl UUVEditorGridProperties {}
#[repr(C, align(8))]
pub struct UUVEditorUDIMProperties {
    __padding_end: [u8; 248],
}
impl UUVEditorUDIMProperties {}
#[repr(C, align(8))]
pub struct UUVEditorMode {
    __padding_end: [u8; 776],
}
impl UUVEditorMode {}
#[repr(C, align(8))]
pub struct UUVEditorUVChannelProperties {
    __padding_end: [u8; 264],
}
impl UUVEditorUVChannelProperties {}
#[repr(C, align(8))]
pub struct UUVEditorUISubsystem {
    __padding_end: [u8; 56],
}
impl UUVEditorUISubsystem {}
#[repr(C, align(8))]
pub struct UUVEditorSubsystem {
    __padding_end: [u8; 144],
}
impl UUVEditorSubsystem {}
#[repr(transparent)]
pub struct EUVEditorBackgroundSourceType(pub i32);
impl EUVEditorBackgroundSourceType {
    pub const CHECKERBOARD: EUVEditorBackgroundSourceType = EUVEditorBackgroundSourceType(
        0,
    );
    pub const TEXTURE: EUVEditorBackgroundSourceType = EUVEditorBackgroundSourceType(1);
    pub const MATERIAL: EUVEditorBackgroundSourceType = EUVEditorBackgroundSourceType(2);
}
#[repr(transparent)]
pub struct EDistortionMetric(pub u8);
impl EDistortionMetric {
    pub const REED_BETA: EDistortionMetric = EDistortionMetric(0);
    pub const SANDER_L2: EDistortionMetric = EDistortionMetric(1);
    pub const SANDER_L_INF: EDistortionMetric = EDistortionMetric(2);
    pub const TEXEL_DENSITY: EDistortionMetric = EDistortionMetric(3);
}
