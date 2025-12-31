#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(8))]
pub struct FActorSequenceObjectReference {
    pub ty: EActorSequenceObjectReferenceType,
    pub actor_id: crate::bindings::core_u_object::FGuid,
    pub path_to_component: FString,
}
#[repr(C, align(8))]
pub struct FActorSequenceObjectReferences {
    pub array: TArray<FActorSequenceObjectReference>,
}
#[repr(C, align(8))]
pub struct FActorSequenceObjectReferenceMap {
    pub binding_ids: TArray<crate::bindings::core_u_object::FGuid>,
    pub references: TArray<FActorSequenceObjectReferences>,
}
pub struct UActorSequence {
    pub movie_scene: UPtr<crate::bindings::movie_scene::UMovieScene>,
    pub object_references: FActorSequenceObjectReferenceMap,
    pub b_has_been_initialized: bool,
}
pub struct UActorSequenceComponent {
    pub playback_settings: crate::bindings::movie_scene::FMovieSceneSequencePlaybackSettings,
    pub sequence: UPtr<UActorSequence>,
    pub sequence_player: UPtr<UActorSequencePlayer>,
}
pub struct UActorSequencePlayer {}
#[allow(non_camel_case_types)]
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
