#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(8))]
pub struct FMassProcessingContext_DEPRECATED {
    __padding_end: [u8; 32],
}
impl FMassProcessingContext_DEPRECATED {}
#[repr(C, align(8))]
pub struct UMassModuleSettings {
    __padding_end: [u8; 48],
}
impl UMassModuleSettings {}
#[repr(C, align(8))]
pub struct UMassEntitySettings {
    __padding_end: [u8; 544],
}
impl UMassEntitySettings {}
#[repr(C, align(8))]
pub struct UMassSubsystemBase {
    __padding_end: [u8; 72],
}
impl UMassSubsystemBase {}
#[repr(C, align(8))]
pub struct UMassEntitySubsystem {
    __padding_end: [u8; 88],
}
impl UMassEntitySubsystem {}
#[repr(C, align(8))]
pub struct UMassProcessor {
    __padding_end: [u8; 240],
}
impl UMassProcessor {}
#[repr(C, align(8))]
pub struct UMassObserverProcessor {
    __padding_end: [u8; 264],
}
impl UMassObserverProcessor {}
#[repr(C, align(8))]
pub struct UMassObserverRegistry {
    __padding_end: [u8; 1336],
}
impl UMassObserverRegistry {}
#[repr(C, align(8))]
pub struct UMassCompositeProcessor {
    __padding_end: [u8; 312],
}
impl UMassCompositeProcessor {}
#[repr(C, align(16))]
pub struct UMassRelationObserver {
    __padding_end: [u8; 1184],
}
impl UMassRelationObserver {}
#[repr(C, align(16))]
pub struct UMassRelationEntityCreation {
    __padding_end: [u8; 1184],
}
impl UMassRelationEntityCreation {}
#[repr(C, align(16))]
pub struct UMassRelationEntityGuardDog {
    __padding_end: [u8; 1184],
}
impl UMassRelationEntityGuardDog {}
#[repr(C, align(16))]
pub struct UMassRelationEntityDestruction {
    __padding_end: [u8; 1184],
}
impl UMassRelationEntityDestruction {}
#[repr(C, align(16))]
pub struct UMassRelationRoleDestruction {
    __padding_end: [u8; 1248],
}
impl UMassRelationRoleDestruction {}
#[repr(C, align(8))]
pub struct UMassSettings {
    __padding_end: [u8; 184],
}
impl UMassSettings {}
#[repr(C, align(8))]
pub struct UMassTickableSubsystemBase {
    __padding_end: [u8; 88],
}
impl UMassTickableSubsystemBase {}
#[repr(C, align(16))]
pub struct UMassChildOfRelationEntityCreation {
    __padding_end: [u8; 1184],
}
impl UMassChildOfRelationEntityCreation {}
#[repr(transparent)]
pub struct EMassProcessingPhase(pub u8);
impl EMassProcessingPhase {
    pub const PRE_PHYSICS: EMassProcessingPhase = EMassProcessingPhase(0);
    pub const START_PHYSICS: EMassProcessingPhase = EMassProcessingPhase(1);
    pub const DURING_PHYSICS: EMassProcessingPhase = EMassProcessingPhase(2);
    pub const END_PHYSICS: EMassProcessingPhase = EMassProcessingPhase(3);
    pub const POST_PHYSICS: EMassProcessingPhase = EMassProcessingPhase(4);
    pub const FRAME_END: EMassProcessingPhase = EMassProcessingPhase(5);
    pub const MAX: EMassProcessingPhase = EMassProcessingPhase(6);
}
#[repr(transparent)]
pub struct EActivationState(pub u8);
impl EActivationState {
    pub const INACTIVE: EActivationState = EActivationState(0);
    pub const ACTIVE: EActivationState = EActivationState(1);
    pub const ONE_SHOT: EActivationState = EActivationState(2);
}
