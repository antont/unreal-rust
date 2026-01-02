#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(16))]
pub struct UVisualLoggerRenderingComponent {
    __padding_end: [u8; 1616],
}
impl UVisualLoggerRenderingComponent {}
#[repr(C, align(8))]
pub struct ULogVisualizerSessionSettings {
    __padding_end: [u8; 88],
}
impl ULogVisualizerSessionSettings {}
#[repr(C, align(8))]
pub struct ULogVisualizerSettings {
    __padding_end: [u8; 200],
}
impl ULogVisualizerSettings {}
#[repr(C, align(8))]
pub struct AVisualLoggerCameraController {
    __padding_end: [u8; 2840],
}
impl AVisualLoggerCameraController {}
#[repr(C, align(8))]
pub struct AVisualLoggerHUD {
    __padding_end: [u8; 1440],
}
impl AVisualLoggerHUD {}
#[repr(C, align(8))]
pub struct AVisualLoggerRenderingActorBase {
    __padding_end: [u8; 1144],
}
impl AVisualLoggerRenderingActorBase {}
#[repr(C, align(8))]
pub struct AVisualLoggerRenderingActor {
    __padding_end: [u8; 1240],
}
impl AVisualLoggerRenderingActor {}
