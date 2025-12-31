#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(8))]
pub struct FSequencerChannelProxy {
    pub channel_name: FName,
    pub section: UPtr<crate::bindings::movie_scene::UMovieSceneSection>,
}
#[repr(C, align(8))]
pub struct FSequencerBoundObjects {
    pub binding_proxy: crate::bindings::movie_scene::FMovieSceneBindingProxy,
    pub bound_objects: TArray<UPtr<crate::bindings::core_u_object::UObject>>,
}
#[repr(C, align(8))]
pub struct FSequencerQuickBindingResult {
    pub event_endpoint: UPtr<crate::bindings::blueprint_graph::UK2Node_CustomEvent>,
    pub payload_names: TArray<FString>,
}
#[repr(C, align(8))]
pub struct FSequencerExportFBXParams {
    pub world: UPtr<crate::bindings::engine::UWorld>,
    pub sequence: UPtr<crate::bindings::level_sequence::ULevelSequence>,
    pub root_sequence: UPtr<crate::bindings::level_sequence::ULevelSequence>,
    pub bindings: TArray<crate::bindings::movie_scene::FMovieSceneBindingProxy>,
    pub tracks: TArray<UPtr<crate::bindings::movie_scene::UMovieSceneTrack>>,
    pub override_options: UPtr<crate::bindings::unreal_ed::UFbxExportOption>,
    pub fbx_file_name: FString,
}
pub struct USequencerCurveEditorObject {}
pub struct USequencerToolsFunctionLibrary {}
pub struct FExportAnimSequenceWaitForDelegate_Delegate;
pub struct FRenderMovie_OnFinishedCallback;
