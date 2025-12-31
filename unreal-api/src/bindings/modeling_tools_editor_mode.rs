#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(8))]
pub struct FModelingModeCustomSectionColor {
    pub section_name: FString,
    pub color: crate::bindings::core_u_object::FLinearColor,
}
#[repr(C, align(8))]
pub struct FModelingModeCustomToolColor {
    pub tool_name: FString,
    pub color: crate::bindings::core_u_object::FLinearColor,
}
#[repr(C, align(8))]
pub struct FModelingModeAssetCollectionSet {
    pub name: FString,
    pub collections: TArray<crate::bindings::engine::FCollectionReference>,
}
pub struct UModelingSelectionInteraction {
    pub click_or_drag_behavior: UPtr<
        crate::bindings::interactive_tools_framework::USingleClickOrDragInputBehavior,
    >,
    pub hover_behavior: UPtr<
        crate::bindings::interactive_tools_framework::UMouseHoverBehavior,
    >,
    pub behavior_set: UPtr<
        crate::bindings::interactive_tools_framework::UInputBehaviorSet,
    >,
    pub selection_manager: UPtr<
        crate::bindings::modeling_components::UGeometrySelectionManager,
    >,
    pub transform_proxy: UPtr<
        crate::bindings::interactive_tools_framework::UTransformProxy,
    >,
    pub transform_gizmo: UPtr<
        crate::bindings::interactive_tools_framework::UCombinedTransformGizmo,
    >,
    pub drag_alignment_interaction: UPtr<
        crate::bindings::modeling_components::UDragAlignmentInteraction,
    >,
    pub drag_alignment_toggle_behavior: UPtr<
        crate::bindings::interactive_tools_framework::UKeyAsModifierInputBehavior,
    >,
    pub rectangle_marquee_interaction: UPtr<
        crate::bindings::modeling_components::URectangleMarqueeInteraction,
    >,
    pub path_selection_interaction: UPtr<UPathSelectionInteraction>,
}
pub struct UPathSelectionInteraction {
    pub selection_interaction: TWeakObjectPtr<UModelingSelectionInteraction>,
}
pub struct UModelingModeEditableToolPaletteConfig {
    pub editable_tool_palettes: TMap<
        FName,
        crate::bindings::widget_registration::FEditableToolPaletteSettings,
    >,
}
pub struct UModelingToolsEditorMode {
    pub b_enable_volume_element_selection: bool,
    pub b_enable_static_mesh_element_selection: bool,
    pub scene_snapping_manager: UPtr<
        crate::bindings::modeling_components::UModelingSceneSnappingManager,
    >,
    pub selection_manager: UPtr<
        crate::bindings::modeling_components::UGeometrySelectionManager,
    >,
    pub selection_interaction: UPtr<UModelingSelectionInteraction>,
    pub modeling_mode_commands: TArray<
        UPtr<crate::bindings::interactive_tools_framework::UInteractiveCommand>,
    >,
}
pub struct UModelingToolsEditorModeSettings {
    pub b_enable_dynamic_mesh_actors: bool,
    pub asset_generation_location: EModelingModeAssetGenerationLocation,
    pub asset_generation_mode: EModelingModeAssetGenerationBehavior,
    pub default_mesh_object_type: EModelingModeDefaultMeshObjectType,
    pub auto_generated_asset_path: FString,
    pub b_store_unsaved_level_assets_in_top_level_game_folder: bool,
    pub b_use_per_user_autogen_subfolder: bool,
    pub autogen_subfolder_user_name_override: FString,
    pub b_append_random_string_to_name: bool,
    pub b_respect_level_editor_gizmo_mode: bool,
    pub b_enable_persistent_selections: bool,
    pub b_enable_mesh_selections: bool,
    pub restrictive_mode_auto_generated_asset_path: FString,
    pub b_enable_absolute_world_snapping: bool,
}
pub struct UModelingToolsModeCustomizationSettings {
    pub b_use_legacy_modeling_palette: bool,
    pub tool_section_order: TArray<FString>,
    pub section_colors: TArray<FModelingModeCustomSectionColor>,
    pub tool_colors: TArray<FModelingModeCustomToolColor>,
    pub brush_alpha_sets: TArray<FModelingModeAssetCollectionSet>,
    pub b_show_category_button_labels: bool,
    pub b_always_show_tool_buttons: bool,
    pub unselected_color: crate::bindings::core_u_object::FLinearColor,
    pub hover_over_selected_color: crate::bindings::core_u_object::FLinearColor,
    pub hover_over_unselected_color: crate::bindings::core_u_object::FLinearColor,
    pub geometry_selected_color: crate::bindings::core_u_object::FLinearColor,
    pub last_mesh_selection_drag_mode: i32,
    pub last_mesh_selection_local_frame_mode: i32,
    pub b_mesh_selection_hit_back_faces: bool,
    pub last_mesh_selection_element_type: i32,
    pub last_mesh_selection_topology_mode: i32,
    pub b_last_mesh_selection_volume_toggle: bool,
    pub b_last_mesh_selection_static_mesh_toggle: bool,
}
pub struct UModelingToolsHostCustomizationAPI {}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EModelingModeAssetGenerationLocation(pub i32);
impl EModelingModeAssetGenerationLocation {
    pub const AUTO_GENERATED_WORLD_RELATIVE_ASSET_PATH: EModelingModeAssetGenerationLocation = EModelingModeAssetGenerationLocation(
        0,
    );
    pub const AUTO_GENERATED_GLOBAL_ASSET_PATH: EModelingModeAssetGenerationLocation = EModelingModeAssetGenerationLocation(
        1,
    );
    pub const CURRENT_ASSET_BROWSER_PATH_IF_AVAILABLE: EModelingModeAssetGenerationLocation = EModelingModeAssetGenerationLocation(
        2,
    );
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EModelingModeAssetGenerationBehavior(pub i32);
impl EModelingModeAssetGenerationBehavior {
    pub const AUTO_GENERATE_AND_AUTOSAVE: EModelingModeAssetGenerationBehavior = EModelingModeAssetGenerationBehavior(
        0,
    );
    pub const AUTO_GENERATE_BUT_DO_NOT_AUTOSAVE: EModelingModeAssetGenerationBehavior = EModelingModeAssetGenerationBehavior(
        1,
    );
    pub const INTERACTIVE_PROMPT_TO_SAVE: EModelingModeAssetGenerationBehavior = EModelingModeAssetGenerationBehavior(
        2,
    );
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EModelingModeDefaultMeshObjectType(pub i32);
impl EModelingModeDefaultMeshObjectType {
    pub const STATIC_MESH_ASSET: EModelingModeDefaultMeshObjectType = EModelingModeDefaultMeshObjectType(
        0,
    );
    pub const VOLUME_ACTOR: EModelingModeDefaultMeshObjectType = EModelingModeDefaultMeshObjectType(
        1,
    );
    pub const DYNAMIC_MESH_ACTOR: EModelingModeDefaultMeshObjectType = EModelingModeDefaultMeshObjectType(
        2,
    );
}
