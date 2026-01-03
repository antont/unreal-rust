#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(8))]
pub struct FStringValuePair {
    pub key: FString,
    pub value: FString,
}
impl FStringValuePair {}
#[repr(C, align(8))]
pub struct UDataflowBlueprintLibrary {
    __padding_end: [u8; 48],
}
impl UDataflowBlueprintLibrary {}
pub struct IDataflowContentOwner {}
#[repr(C, align(8))]
pub struct UDataflowContentOwner {
    __padding_end: [u8; 48],
}
impl UDataflowContentOwner {}
#[repr(C, align(8))]
pub struct UDataflowContextObject {
    __padding_end: [u8; 128],
}
impl UDataflowContextObject {}
#[repr(C, align(8))]
pub struct UDataflowBaseContent {
    __padding_end: [u8; 208],
}
impl UDataflowBaseContent {}
#[repr(C, align(8))]
pub struct UDataflowSkeletalContent {
    __padding_end: [u8; 232],
}
impl UDataflowSkeletalContent {}
#[repr(C, align(16))]
pub struct UDataflowDebugDrawComponent {
    __padding_end: [u8; 1616],
}
impl UDataflowDebugDrawComponent {}
#[repr(C, align(8))]
pub struct UDataflowEdNode {
    __padding_end: [u8; 256],
}
impl UDataflowEdNode {}
pub struct IDataflowInstanceInterface {}
#[repr(C, align(8))]
pub struct UDataflowInstanceInterface {
    __padding_end: [u8; 48],
}
impl UDataflowInstanceInterface {}
#[repr(C, align(8))]
pub struct UDataflowMesh {
    __padding_end: [u8; 72],
}
impl UDataflowMesh {}
#[repr(C, align(16))]
pub struct UDataflow {
    __padding_end: [u8; 720],
}
impl UDataflow {}
#[repr(C, align(8))]
pub struct UDataflowSubGraph {
    __padding_end: [u8; 216],
}
impl UDataflowSubGraph {}
#[repr(transparent)]
pub struct EDataflowDebugDrawRenderType(pub u8);
impl EDataflowDebugDrawRenderType {
    pub const WIREFRAME: EDataflowDebugDrawRenderType = EDataflowDebugDrawRenderType(0);
    pub const SHADED: EDataflowDebugDrawRenderType = EDataflowDebugDrawRenderType(1);
}
#[repr(transparent)]
pub struct EDataflowSphereCoveringColorMethod(pub u8);
impl EDataflowSphereCoveringColorMethod {
    pub const SINGLE: EDataflowSphereCoveringColorMethod = EDataflowSphereCoveringColorMethod(
        0,
    );
    pub const COLOR_BY_RADIUS: EDataflowSphereCoveringColorMethod = EDataflowSphereCoveringColorMethod(
        1,
    );
    pub const RANDOM: EDataflowSphereCoveringColorMethod = EDataflowSphereCoveringColorMethod(
        2,
    );
}
#[repr(transparent)]
pub struct EDataflowType(pub u8);
impl EDataflowType {
    pub const CONSTRUCTION: EDataflowType = EDataflowType(0);
    pub const SIMULATION: EDataflowType = EDataflowType(1);
}
