#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(8))]
pub struct FAudioInputDeviceInfo {
    pub device_name: FString,
    pub device_id: FString,
    pub input_channels: i32,
    pub preferred_sample_rate: i32,
    pub flags_40: u8,
    __padding_end: [u8; 7],
}
impl FAudioInputDeviceInfo {}
#[repr(C, align(4))]
pub struct FAudioCaptureDeviceInfo {
    pub device_name: FName,
    pub num_input_channels: i32,
    pub sample_rate: i32,
}
impl FAudioCaptureDeviceInfo {}
#[repr(C, align(8))]
pub struct UAudioCapture {
    __padding_end: [u8; 184],
}
impl UAudioCapture {}
#[repr(C, align(8))]
pub struct UAudioCaptureFunctionLibrary {
    __padding_end: [u8; 48],
}
impl UAudioCaptureFunctionLibrary {}
#[repr(C, align(8))]
pub struct UAudioCaptureBlueprintLibrary {
    __padding_end: [u8; 48],
}
impl UAudioCaptureBlueprintLibrary {}
#[repr(C, align(16))]
pub struct UAudioCaptureComponent {
    #[doc(hidden)]
    __padding_2384: [u8; 2384],
    pub jitter_latency_frames: i32,
    __padding_end: [u8; 188],
}
impl UAudioCaptureComponent {}
#[repr(C, align(8))]
pub struct FGetAvailableAudioInputDevices_OnObtainDevicesEvent {
    _opague: [u8; 32],
}
