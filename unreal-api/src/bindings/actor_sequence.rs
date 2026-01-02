#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(8))]
pub struct UActorSequence {
    __padding_end: [u8; 176],
}
impl UActorSequence {}
#[repr(C, align(8))]
pub struct UActorSequenceComponent {
    #[doc(hidden)]
    __padding_288: [u8; 288],
    pub sequence_player: UPtr<UActorSequencePlayer>,
}
impl UActorSequenceComponent {}
#[repr(C, align(8))]
pub struct UActorSequencePlayer {
    __padding_end: [u8; 1216],
}
impl UActorSequencePlayer {}
#[repr(transparent)]
pub struct EActorSequenceObjectReferenceType(pub u8);
impl EActorSequenceObjectReferenceType {
    pub const CONTEXT_ACTOR: EActorSequenceObjectReferenceType = EActorSequenceObjectReferenceType(
        0,
    );
    pub const EXTERNAL_ACTOR: EActorSequenceObjectReferenceType = EActorSequenceObjectReferenceType(
        1,
    );
    pub const COMPONENT: EActorSequenceObjectReferenceType = EActorSequenceObjectReferenceType(
        2,
    );
}
