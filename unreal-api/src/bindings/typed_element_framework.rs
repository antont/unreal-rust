#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(8))]
pub struct FScriptTypedElementHandle {
    __padding_end: [u8; 8],
}
impl FScriptTypedElementHandle {}
#[repr(C, align(8))]
pub struct FScriptTypedElementListProxy {
    __padding_end: [u8; 16],
}
impl FScriptTypedElementListProxy {}
#[repr(C, align(8))]
pub struct UEditorDataStorageFactory {
    __padding_end: [u8; 48],
}
impl UEditorDataStorageFactory {}
#[repr(C, align(8))]
pub struct UTypedElementHandleLibrary {
    __padding_end: [u8; 48],
}
impl UTypedElementHandleLibrary {}
#[repr(C, align(8))]
pub struct UTypedElementListLibrary {
    __padding_end: [u8; 48],
}
impl UTypedElementListLibrary {}
pub struct ITypedElementCounterInterface {}
#[repr(C, align(8))]
pub struct UTypedElementCounterInterface {
    __padding_end: [u8; 48],
}
impl UTypedElementCounterInterface {}
#[repr(C, align(8))]
pub struct UTypedElementRegistry {
    __padding_end: [u8; 2408],
}
impl UTypedElementRegistry {}
pub struct ITestTypedElementInterfaceA {}
#[repr(C, align(8))]
pub struct UTestTypedElementInterfaceA {
    __padding_end: [u8; 48],
}
impl UTestTypedElementInterfaceA {}
pub struct ITestTypedElementInterfaceB {}
#[repr(C, align(8))]
pub struct UTestTypedElementInterfaceB {
    __padding_end: [u8; 48],
}
impl UTestTypedElementInterfaceB {}
pub struct ITestTypedElementInterfaceC {}
#[repr(C, align(8))]
pub struct UTestTypedElementInterfaceC {
    __padding_end: [u8; 48],
}
impl UTestTypedElementInterfaceC {}
#[repr(C, align(8))]
pub struct UTestTypedElementInterfaceA_ImplTyped {
    __padding_end: [u8; 56],
}
impl UTestTypedElementInterfaceA_ImplTyped {}
#[repr(C, align(8))]
pub struct UTestTypedElementInterfaceA_ImplUntyped {
    __padding_end: [u8; 56],
}
impl UTestTypedElementInterfaceA_ImplUntyped {}
#[repr(C, align(8))]
pub struct UTestTypedElementInterfaceBAndC_Typed {
    __padding_end: [u8; 64],
}
impl UTestTypedElementInterfaceBAndC_Typed {}
#[repr(C, align(8))]
pub struct UTest_PingPongBetweenPhaseFactory {
    __padding_end: [u8; 48],
}
impl UTest_PingPongBetweenPhaseFactory {}
#[repr(transparent)]
pub struct ESCCModification(pub i32);
impl ESCCModification {
    pub const CONFLICTED: ESCCModification = ESCCModification(0);
    pub const MODIFIED: ESCCModification = ESCCModification(1);
    pub const ADDED: ESCCModification = ESCCModification(2);
    pub const REMOVED: ESCCModification = ESCCModification(3);
}
