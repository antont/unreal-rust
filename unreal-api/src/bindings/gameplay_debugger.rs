#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(8))]
pub struct AGameplayDebuggerCategoryReplicator {
    __padding_end: [u8; 1328],
}
impl AGameplayDebuggerCategoryReplicator {}
#[repr(C, align(8))]
pub struct UGameplayDebuggerConfig {
    __padding_end: [u8; 712],
}
impl UGameplayDebuggerConfig {}
#[repr(C, align(8))]
pub struct UGameplayDebuggerUserSettings {
    __padding_end: [u8; 208],
}
impl UGameplayDebuggerUserSettings {}
#[repr(C, align(8))]
pub struct UGameplayDebuggerLocalController {
    __padding_end: [u8; 336],
}
impl UGameplayDebuggerLocalController {}
#[repr(C, align(8))]
pub struct AGameplayDebuggerPlayerManager {
    __padding_end: [u8; 1208],
}
impl AGameplayDebuggerPlayerManager {}
#[repr(C, align(16))]
pub struct UGameplayDebuggerRenderingComponent {
    __padding_end: [u8; 1728],
}
impl UGameplayDebuggerRenderingComponent {}
#[repr(transparent)]
pub struct EGameplayDebuggerShape(pub u8);
impl EGameplayDebuggerShape {
    pub const INVALID: EGameplayDebuggerShape = EGameplayDebuggerShape(0);
    pub const POINT: EGameplayDebuggerShape = EGameplayDebuggerShape(1);
    pub const SEGMENT: EGameplayDebuggerShape = EGameplayDebuggerShape(2);
    pub const BOX: EGameplayDebuggerShape = EGameplayDebuggerShape(3);
    pub const CONE: EGameplayDebuggerShape = EGameplayDebuggerShape(4);
    pub const CYLINDER: EGameplayDebuggerShape = EGameplayDebuggerShape(5);
    pub const CIRCLE: EGameplayDebuggerShape = EGameplayDebuggerShape(6);
    pub const RECTANGLE: EGameplayDebuggerShape = EGameplayDebuggerShape(7);
    pub const CAPSULE: EGameplayDebuggerShape = EGameplayDebuggerShape(8);
    pub const POLYGON: EGameplayDebuggerShape = EGameplayDebuggerShape(9);
    pub const POLYLINE: EGameplayDebuggerShape = EGameplayDebuggerShape(10);
    pub const ARROW: EGameplayDebuggerShape = EGameplayDebuggerShape(11);
}
#[repr(transparent)]
pub struct EGameplayDebuggerOverrideMode(pub u8);
impl EGameplayDebuggerOverrideMode {
    pub const ENABLE: EGameplayDebuggerOverrideMode = EGameplayDebuggerOverrideMode(0);
    pub const DISABLE: EGameplayDebuggerOverrideMode = EGameplayDebuggerOverrideMode(1);
    pub const USE_DEFAULT: EGameplayDebuggerOverrideMode = EGameplayDebuggerOverrideMode(
        2,
    );
}
