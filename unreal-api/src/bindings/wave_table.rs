#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(8))]
pub struct FWaveTableData {
    __padding_end: [u8; 32],
}
impl FWaveTableData {}
#[repr(C, align(8))]
pub struct FWaveTableTransform {
    pub curve: EWaveTableCurve,
    pub scalar: f32,
    #[doc(hidden)]
    __padding_136: [u8; 128],
    pub curve_shared: UPtr<crate::bindings::engine::UCurveFloat>,
    __padding_end: [u8; 168],
}
impl FWaveTableTransform {}
#[repr(C, align(8))]
pub struct UWaveTableBank {
    __padding_end: [u8; 112],
}
impl UWaveTableBank {}
#[repr(transparent)]
pub struct EWaveTableBitDepth(pub u8);
impl EWaveTableBitDepth {
    pub const PCM_16: EWaveTableBitDepth = EWaveTableBitDepth(0);
    pub const IEEE_FLOAT: EWaveTableBitDepth = EWaveTableBitDepth(1);
    pub const COUNT: EWaveTableBitDepth = EWaveTableBitDepth(2);
}
#[repr(transparent)]
pub struct EWaveTableCurve(pub u8);
impl EWaveTableCurve {
    pub const LINEAR: EWaveTableCurve = EWaveTableCurve(0);
    pub const LINEAR_INV: EWaveTableCurve = EWaveTableCurve(1);
    pub const EXP: EWaveTableCurve = EWaveTableCurve(2);
    pub const EXP_INVERSE: EWaveTableCurve = EWaveTableCurve(3);
    pub const LOG: EWaveTableCurve = EWaveTableCurve(4);
    pub const SIN: EWaveTableCurve = EWaveTableCurve(5);
    pub const SIN_FULL: EWaveTableCurve = EWaveTableCurve(6);
    pub const S_CURVE: EWaveTableCurve = EWaveTableCurve(7);
    pub const SHARED: EWaveTableCurve = EWaveTableCurve(8);
    pub const CUSTOM: EWaveTableCurve = EWaveTableCurve(9);
    pub const FILE: EWaveTableCurve = EWaveTableCurve(10);
    pub const COUNT: EWaveTableCurve = EWaveTableCurve(11);
}
#[repr(transparent)]
pub struct EWaveTableSamplingMode(pub u8);
impl EWaveTableSamplingMode {
    pub const FIXED_SAMPLE_RATE: EWaveTableSamplingMode = EWaveTableSamplingMode(0);
    pub const FIXED_RESOLUTION: EWaveTableSamplingMode = EWaveTableSamplingMode(1);
    pub const COUNT: EWaveTableSamplingMode = EWaveTableSamplingMode(2);
}
#[repr(transparent)]
pub struct EWaveTableResolution(pub u8);
impl EWaveTableResolution {
    pub const NONE: EWaveTableResolution = EWaveTableResolution(0);
    pub const RES_8: EWaveTableResolution = EWaveTableResolution(3);
    pub const RES_16: EWaveTableResolution = EWaveTableResolution(4);
    pub const RES_32: EWaveTableResolution = EWaveTableResolution(5);
    pub const RES_64: EWaveTableResolution = EWaveTableResolution(6);
    pub const RES_128: EWaveTableResolution = EWaveTableResolution(7);
    pub const RES_256: EWaveTableResolution = EWaveTableResolution(8);
    pub const RES_512: EWaveTableResolution = EWaveTableResolution(9);
    pub const RES_1024: EWaveTableResolution = EWaveTableResolution(10);
    pub const RES_2048: EWaveTableResolution = EWaveTableResolution(11);
    pub const RES_4096: EWaveTableResolution = EWaveTableResolution(12);
    pub const MAXIMUM: EWaveTableResolution = EWaveTableResolution(13);
}
