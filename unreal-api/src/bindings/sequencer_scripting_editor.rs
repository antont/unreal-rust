#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::prelude::*;
#[repr(C, align(8))]
pub struct FSequencerChannelProxy {
    pub channel_name: FName,
    pub section: UPtr<UMovieSceneSection>,
}
#[repr(C, align(8))]
pub struct FSequencerBoundObjects {
    pub binding_proxy: FMovieSceneBindingProxy,
    pub bound_objects: TArray<UPtr<UObject>>,
}
#[repr(C, align(8))]
pub struct FSequencerQuickBindingResult {
    pub event_endpoint: UPtr<UK2Node_CustomEvent>,
    pub payload_names: TArray<FString>,
}
#[repr(C, align(8))]
pub struct FSequencerExportFBXParams {
    pub world: UPtr<UWorld>,
    pub sequence: UPtr<ULevelSequence>,
    pub root_sequence: UPtr<ULevelSequence>,
    pub bindings: TArray<FMovieSceneBindingProxy>,
    pub tracks: TArray<UPtr<UMovieSceneTrack>>,
    pub override_options: UPtr<UFbxExportOption>,
    pub fbx_file_name: FString,
}
pub struct USequencerCurveEditorObject {}
pub struct USequencerToolsFunctionLibrary {}
