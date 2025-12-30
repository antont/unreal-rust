#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::prelude::*;
#[repr(C, align(8))]
pub struct FModelingModeCustomSectionColor {
    pub section_name: FString,
    pub color: FLinearColor,
}
#[repr(C, align(8))]
pub struct FModelingModeCustomToolColor {
    pub tool_name: FString,
    pub color: FLinearColor,
}
#[repr(C, align(8))]
pub struct FModelingModeAssetCollectionSet {
    pub name: FString,
    pub collections: TArray<FCollectionReference>,
}
pub struct UModelingSelectionInteraction {
    pub click_or_drag_behavior: UPtr<USingleClickOrDragInputBehavior>,
    pub hover_behavior: UPtr<UMouseHoverBehavior>,
    pub behavior_set: UPtr<UInputBehaviorSet>,
    pub selection_manager: UPtr<UGeometrySelectionManager>,
    pub transform_proxy: UPtr<UTransformProxy>,
    pub transform_gizmo: UPtr<UCombinedTransformGizmo>,
    pub drag_alignment_interaction: UPtr<UDragAlignmentInteraction>,
    pub drag_alignment_toggle_behavior: UPtr<UKeyAsModifierInputBehavior>,
    pub rectangle_marquee_interaction: UPtr<URectangleMarqueeInteraction>,
    pub path_selection_interaction: UPtr<UPathSelectionInteraction>,
}
pub struct UPathSelectionInteraction {
    pub selection_interaction: TWeakObjectPtr<UModelingSelectionInteraction>,
}
pub struct UModelingModeEditableToolPaletteConfig {
    pub editable_tool_palettes: TMap<FName, FEditableToolPaletteSettings>,
}
pub struct UModelingToolsEditorMode {
    pub b_enable_volume_element_selection: bool,
    pub b_enable_static_mesh_element_selection: bool,
    pub scene_snapping_manager: UPtr<UModelingSceneSnappingManager>,
    pub selection_manager: UPtr<UGeometrySelectionManager>,
    pub selection_interaction: UPtr<UModelingSelectionInteraction>,
    pub modeling_mode_commands: TArray<UPtr<UInteractiveCommand>>,
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
    pub unselected_color: FLinearColor,
    pub hover_over_selected_color: FLinearColor,
    pub hover_over_unselected_color: FLinearColor,
    pub geometry_selected_color: FLinearColor,
    pub last_mesh_selection_drag_mode: i32,
    pub last_mesh_selection_local_frame_mode: i32,
    pub b_mesh_selection_hit_back_faces: bool,
    pub last_mesh_selection_element_type: i32,
    pub last_mesh_selection_topology_mode: i32,
    pub b_last_mesh_selection_volume_toggle: bool,
    pub b_last_mesh_selection_static_mesh_toggle: bool,
}
pub struct UModelingToolsHostCustomizationAPI {}
