#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(8))]
pub struct UBlueprintEditorToolMenuContext {
    __padding_end: [u8; 64],
}
impl UBlueprintEditorToolMenuContext {}
#[repr(C, align(8))]
pub struct USSCSEditorMenuContext {
    __padding_end: [u8; 88],
}
impl USSCSEditorMenuContext {}
#[repr(C, align(8))]
pub struct UBlueprintCompilerExtension {
    __padding_end: [u8; 48],
}
impl UBlueprintCompilerExtension {}
#[repr(C, align(8))]
pub struct UBlueprintPaletteFavorites {
    __padding_end: [u8; 128],
}
impl UBlueprintPaletteFavorites {}
#[repr(C, align(8))]
pub struct UJsonObjectGraphFunctionLibrary {
    __padding_end: [u8; 48],
}
impl UJsonObjectGraphFunctionLibrary {}
#[repr(C, align(8))]
pub struct USCSEditorExtensionContext {
    __padding_end: [u8; 80],
}
impl USCSEditorExtensionContext {}
#[repr(C, align(8))]
pub struct USubobjectEditorExtensionContext {
    __padding_end: [u8; 64],
}
impl USubobjectEditorExtensionContext {}
#[repr(transparent)]
pub struct EBPDependencyType(pub i32);
impl EBPDependencyType {
    pub const ASSET: EBPDependencyType = EBPDependencyType(0);
    pub const STRUCT: EBPDependencyType = EBPDependencyType(1);
    pub const CLASS: EBPDependencyType = EBPDependencyType(2);
}
#[repr(transparent)]
pub struct EFiBIndexAllPermission(pub i32);
impl EFiBIndexAllPermission {
    pub const NONE: EFiBIndexAllPermission = EFiBIndexAllPermission(0);
    pub const LOAD_ONLY: EFiBIndexAllPermission = EFiBIndexAllPermission(1);
    pub const CHECKOUT_AND_RESAVE: EFiBIndexAllPermission = EFiBIndexAllPermission(2);
}
