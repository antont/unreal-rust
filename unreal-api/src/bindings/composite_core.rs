#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(8))]
pub struct UCompositeCorePluginSettings {
    __padding_end: [u8; 240],
}
impl UCompositeCorePluginSettings {}
#[repr(C, align(8))]
pub struct UCompositeCoreSubsystem {
    __padding_end: [u8; 112],
}
impl UCompositeCoreSubsystem {}
#[repr(C, align(16))]
pub struct UHoldoutCompositeComponent {
    #[doc(hidden)]
    __padding_656: [u8; 656],
    pub b_is_enabled: bool,
    __padding_end: [u8; 15],
}
impl UHoldoutCompositeComponent {}
