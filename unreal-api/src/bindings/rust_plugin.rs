#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(8))]
pub struct FEntity {
    __padding_end: [u8; 8],
}
impl FEntity {}
#[repr(C, align(8))]
pub struct UAnimNotify_RustEvent {
    __padding_end: [u8; 176],
}
impl UAnimNotify_RustEvent {}
#[repr(C, align(8))]
pub struct UUuid {
    __padding_end: [u8; 64],
}
impl UUuid {}
#[repr(C, align(8))]
pub struct UEntityComponent {
    __padding_end: [u8; 328],
}
impl UEntityComponent {}
#[repr(C, align(8))]
pub struct UK2Node_GetComponentRust {
    __padding_end: [u8; 240],
}
impl UK2Node_GetComponentRust {}
#[repr(C, align(8))]
pub struct ARustActor {
    __padding_end: [u8; 1152],
}
impl ARustActor {}
#[repr(C, align(8))]
pub struct ARustBindingsActor {
    __padding_end: [u8; 1136],
}
impl ARustBindingsActor {}
#[repr(C, align(8))]
pub struct ARustGameModeBase {
    __padding_end: [u8; 1328],
}
impl ARustGameModeBase {}
#[repr(C, align(8))]
pub struct UUEdGraphSchema_Rust {
    __padding_end: [u8; 48],
}
impl UUEdGraphSchema_Rust {}
#[repr(C, align(8))]
pub struct URustReflectionLibrary {
    __padding_end: [u8; 48],
}
impl URustReflectionLibrary {}
