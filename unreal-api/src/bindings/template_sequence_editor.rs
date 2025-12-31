#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
pub struct UAssetDefinition_TemplateSequence {}
pub struct UAssetDefinition_CameraAnimationSequence {}
pub struct UCameraAnimationSequenceFactoryNew {}
pub struct UTemplateSequenceFactoryNew {
    pub bound_actor_class: TSubclassOf<crate::bindings::core_u_object::UObject>,
}
pub struct UTemplateSequenceEditorSettings {
    pub b_show_outdated_assets_in_camera_animation_track_editor: bool,
    pub b_camera_initially_additive_to_viewport: bool,
}
pub struct UTemplateSequenceCameraPreviewSystem {}
