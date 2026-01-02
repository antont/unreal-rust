#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(8))]
pub struct UClassTemplate {
    __padding_end: [u8; 56],
}
impl UClassTemplate {}
#[repr(C, align(8))]
pub struct UPluginClassTemplate {
    __padding_end: [u8; 72],
}
impl UPluginClassTemplate {}
#[repr(C, align(8))]
pub struct UClassTemplateEditorSubsystem {
    __padding_end: [u8; 136],
}
impl UClassTemplateEditorSubsystem {}
#[repr(C, align(8))]
pub struct UTemplateProjectDefs {
    __padding_end: [u8; 320],
}
impl UTemplateProjectDefs {}
#[repr(C, align(8))]
pub struct UDefaultTemplateProjectDefs {
    __padding_end: [u8; 320],
}
impl UDefaultTemplateProjectDefs {}
#[repr(C, align(8))]
pub struct UTemplateCategories {
    __padding_end: [u8; 64],
}
impl UTemplateCategories {}
#[repr(transparent)]
pub struct ETemplateSetting(pub i32);
impl ETemplateSetting {
    pub const LANGUAGES: ETemplateSetting = ETemplateSetting(0);
    pub const HARDWARE_TARGET: ETemplateSetting = ETemplateSetting(1);
    pub const GRAPHICS_PRESET: ETemplateSetting = ETemplateSetting(2);
    pub const STARTER_CONTENT: ETemplateSetting = ETemplateSetting(3);
    pub const XR: ETemplateSetting = ETemplateSetting(4);
    pub const RAYTRACING_DEPRECATED: ETemplateSetting = ETemplateSetting(5);
    pub const VARIANTS: ETemplateSetting = ETemplateSetting(6);
    pub const ALL: ETemplateSetting = ETemplateSetting(7);
}
