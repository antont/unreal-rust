#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
pub struct ULevelInstanceActorFactory {}
pub struct ULevelInstanceEditorBehaviorSource {
    pub input_behavior_set: UPtr<
        crate::bindings::interactive_tools_framework::UInputBehaviorSet,
    >,
}
pub struct ULevelInstanceEditorMode {
    pub mode_behavior_source: TScriptInterface<
        crate::bindings::interactive_tools_framework::IInputBehaviorSource,
    >,
}
pub struct ULevelInstanceEditorSettings {
    pub template_map_infos: TArray<crate::bindings::unreal_ed::FTemplateMapInfo>,
    pub level_instance_class_name: FString,
    pub b_enable_streaming: bool,
    pub b_is_edit_in_place_streaming_enabled: bool,
}
pub struct ULevelInstanceEditorPerProjectUserSettings {
    pub b_always_show_dialog: bool,
    pub pivot_type: crate::bindings::engine::ELevelInstancePivotType,
    pub b_is_sub_selection_enabled: bool,
    pub b_is_viewport_sub_selection_enabled: bool,
    pub b_keep_folders_during_break: bool,
}
