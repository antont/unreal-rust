#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(8))]
pub struct FSubobjectData {
    __padding_end: [u8; 88],
}
impl FSubobjectData {}
#[repr(C, align(8))]
pub struct FSubobjectDataHandle {
    __padding_end: [u8; 16],
}
impl FSubobjectDataHandle {}
#[repr(C, align(8))]
pub struct FAddNewSubobjectParams {
    pub parent_handle: FSubobjectDataHandle,
    pub new_class: TSubclassOf<crate::bindings::core_u_object::UObject>,
    #[doc(hidden)]
    __padding_32: [u8; 8],
    pub blueprint_context: UPtr<crate::bindings::engine::UBlueprint>,
    pub flags_40: u8,
    __padding_end: [u8; 7],
}
impl FAddNewSubobjectParams {}
#[repr(C, align(8))]
pub struct FReparentSubobjectParams {
    pub new_parent_handle: FSubobjectDataHandle,
    pub blueprint_context: UPtr<crate::bindings::engine::UBlueprint>,
    pub actor_preview_context: UPtr<crate::bindings::engine::AActor>,
}
impl FReparentSubobjectParams {}
#[repr(C, align(8))]
pub struct USubobjectDataBlueprintFunctionLibrary {
    __padding_end: [u8; 48],
}
impl USubobjectDataBlueprintFunctionLibrary {}
#[repr(C, align(8))]
pub struct USubobjectDataSubsystem {
    __padding_end: [u8; 136],
}
impl USubobjectDataSubsystem {}
