#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(8))]
pub struct UWindowsTargetSettings {
    __padding_end: [u8; 272],
}
impl UWindowsTargetSettings {}
#[repr(transparent)]
pub struct EDefaultGraphicsRHI(pub u8);
impl EDefaultGraphicsRHI {
    pub const DEFAULT_GRAPHICS_RHI_DEFAULT: EDefaultGraphicsRHI = EDefaultGraphicsRHI(0);
    pub const DEFAULT_GRAPHICS_RHI_DX11: EDefaultGraphicsRHI = EDefaultGraphicsRHI(1);
    pub const DEFAULT_GRAPHICS_RHI_DX12: EDefaultGraphicsRHI = EDefaultGraphicsRHI(2);
    pub const DEFAULT_GRAPHICS_RHI_VULKAN: EDefaultGraphicsRHI = EDefaultGraphicsRHI(3);
}
#[repr(transparent)]
pub struct ECompilerVersion(pub u8);
impl ECompilerVersion {
    pub const DEFAULT: ECompilerVersion = ECompilerVersion(0);
    pub const VISUAL_STUDIO2015: ECompilerVersion = ECompilerVersion(1);
    pub const VISUAL_STUDIO2017: ECompilerVersion = ECompilerVersion(2);
    pub const VISUAL_STUDIO2019: ECompilerVersion = ECompilerVersion(3);
    pub const VISUAL_STUDIO2022: ECompilerVersion = ECompilerVersion(4);
    pub const VISUAL_STUDIO2026: ECompilerVersion = ECompilerVersion(5);
}
