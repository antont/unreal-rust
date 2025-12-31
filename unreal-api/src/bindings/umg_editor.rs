#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(8))]
pub struct FAssetThumbnailWidgetSettings {
    pub b_force_generic_thumbnail: bool,
    pub b_allow_hint_text: bool,
    pub b_allow_real_time_on_hovered: bool,
    pub b_allow_asset_specific_thumbnail_overlay: bool,
    pub thumbnail_label: EThumbnailLabelType_BlueprintType,
    pub highlighted_text_delegate: FAssetThumbnailWidgetSettings_HighlightedTextDelegate,
    pub hint_color_and_opacity: crate::bindings::core_u_object::FLinearColor,
    pub b_override_asset_type_color: bool,
    pub asset_type_color_override: crate::bindings::core_u_object::FLinearColor,
    pub padding: crate::bindings::slate_core::FMargin,
    pub generic_thumbnail_size: i32,
    pub color_strip_orientation: EThumbnailColorStripOrientation_BlueprintType,
}
#[repr(C, align(4))]
pub struct FNavigationSimulationArguments {
    pub user_index: i32,
    pub navigation_genesis: crate::bindings::slate_core::ENavigationGenesis,
    pub ui_navigation: crate::bindings::slate_core::EUINavigation,
    pub b_override_ui_navigation: bool,
    pub b_show_preview: bool,
}
#[repr(C, align(8))]
pub struct FDebugResolution {
    pub width: i32,
    pub height: i32,
    pub description: FString,
    pub color: crate::bindings::core_u_object::FLinearColor,
}
#[repr(C, align(8))]
pub struct FWidgetCompilerOptions {
    pub b_allow_blueprint_tick: bool,
    pub b_allow_blueprint_paint: bool,
    pub property_binding_rule: EPropertyBindingPermissionLevel,
    pub rules: TArray<TSoftObjectPtr<crate::bindings::core_u_object::UClass>>,
}
#[repr(C, align(8))]
pub struct FDirectoryWidgetCompilerOptions {
    pub directory: crate::bindings::core_u_object::FDirectoryPath,
    pub ignored_widgets: TArray<TSoftObjectPtr<UWidgetBlueprint>>,
    pub options: FWidgetCompilerOptions,
}
#[repr(C, align(8))]
pub struct FEditorPropertyPathSegment {
    pub _struct: UPtr<crate::bindings::core_u_object::UStruct>,
    pub member_name: FName,
    pub member_guid: crate::bindings::core_u_object::FGuid,
    pub is_property: bool,
}
#[repr(C, align(8))]
pub struct FEditorPropertyPath {
    pub segments: TArray<FEditorPropertyPathSegment>,
}
#[repr(C, align(8))]
pub struct FDelegateEditorBinding {
    pub object_name: FString,
    pub property_name: FName,
    pub function_name: FName,
    pub source_property: FName,
    pub source_path: FEditorPropertyPath,
    pub member_guid: crate::bindings::core_u_object::FGuid,
    pub kind: crate::bindings::umg::EBindingKind,
}
pub struct UWidgetBlueprint {
    pub bindings: TArray<FDelegateEditorBinding>,
    pub animations: TArray<UPtr<crate::bindings::umg::UWidgetAnimation>>,
    pub widget_variable_name_to_guid_map: TMap<
        FName,
        crate::bindings::core_u_object::FGuid,
    >,
    pub palette_category: FString,
    pub b_can_call_initialized_without_player_context: bool,
    pub tick_frequency: crate::bindings::umg::EWidgetTickFrequency,
    pub tick_prediction: EWidgetCompileTimeTickPrediction,
    pub tick_prediction_reason: FString,
    pub property_bindings: i32,
    pub thumbnail_size_mode: EThumbnailPreviewSizeMode,
    pub thumbnail_custom_size: crate::bindings::core_u_object::FVector2D,
    pub thumbnail_image: UPtr<crate::bindings::engine::UTexture2D>,
}
pub struct UWidgetEditingProjectSettings {
    pub default_compiler_options: FWidgetCompilerOptions,
    pub directory_compiler_options: TArray<FDirectoryWidgetCompilerOptions>,
    pub b_show_widgets_from_engine_content: bool,
    pub b_show_widgets_from_developer_content: bool,
    pub categories_to_hide: TArray<FString>,
    pub widget_classes_to_hide: TArray<crate::bindings::core_u_object::FSoftClassPath>,
    pub b_use_widget_template_selector: bool,
    pub common_root_widget_classes: TArray<
        TSoftObjectPtr<crate::bindings::core_u_object::UClass>,
    >,
    pub default_root_widget: TSubclassOf<crate::bindings::umg::UPanelWidget>,
    pub b_use_editor_config_palette_filtering: bool,
    pub b_use_user_widget_parent_class_viewer_selector: bool,
    pub b_use_user_widget_parent_default_class_viewer_selector: bool,
    pub b_enable_make_variable: bool,
    pub b_enable_widget_animation_editor: bool,
    pub b_enable_palette_window: bool,
    pub b_enable_library_window: bool,
    pub b_enable_hierarchy_window: bool,
    pub b_enable_bind_widget_window: bool,
    pub b_enable_navigation_simulation_window: bool,
    pub b_can_call_initialized_without_player_context: bool,
    pub b_enable_ui_components_property: bool,
    pub favorite_widget_parent_classes: TArray<
        TSoftObjectPtr<crate::bindings::core_u_object::UClass>,
    >,
    pub debug_resolutions: TArray<FDebugResolution>,
    pub version: i32,
}
pub struct UUIComponentWidgetPair {
    pub widget_name: FName,
    pub component: UPtr<crate::bindings::umg::UUIComponent>,
}
pub struct UWidgetBlueprintToolMenuContext {}
pub struct UWidgetPaletteFavorites {
    pub favorites: TArray<FString>,
}
pub struct UAssetDefinition_WidgetBlueprint {}
pub struct UAssetDefinition_WidgetBlueprintGeneratedClass {}
pub struct UAssetThumbnailWidget {
    pub asset_to_show: crate::bindings::core_u_object::FAssetData,
    pub resolution: crate::bindings::core_u_object::FIntPoint,
    pub thumbnail_settings: FAssetThumbnailWidgetSettings,
}
pub struct UK2Node_WidgetAnimationEvent {
    pub action: crate::bindings::umg::EWidgetAnimationEvent,
    pub animation_property_name: FName,
    pub user_tag: FName,
    pub source_widget_blueprint: UPtr<UWidgetBlueprint>,
}
pub struct UK2Node_CreateDragDropOperation {}
pub struct UK2Node_CreateWidget {}
pub struct UK2Node_PlayAnimation {}
pub struct UK2Node_PlayAnimationTimeRange {}
pub struct UK2Node_PlayAnimation2 {}
pub struct UK2Node_PlayAnimationTimeRange2 {}
pub struct UUMGEditorProjectSettings {}
pub struct UWidgetDesignerSettings {
    pub flags_104: u8,
    pub grid_snap_size: i32,
    pub b_lock_to_panel_on_drag_by_default: bool,
    pub default_preview_resolution: crate::bindings::core_u_object::FUintVector2,
    pub b_show_outlines: bool,
    pub b_execute_pre_construct_event: bool,
    pub b_respect_locks: bool,
    pub create_on_compile: EDisplayOnCompile,
    pub dismiss_on_compile: EDisplayOnCompile,
    pub favorites: UPtr<UWidgetPaletteFavorites>,
}
pub struct USlateVectorArtDataFactory {}
pub struct UWidgetEditorModeUISubsystem {}
pub struct UWidgetBlueprintExtension {}
pub struct UUIComponentWidgetBlueprintExtension {
    pub component_container: UPtr<crate::bindings::umg::UUIComponentContainer>,
}
pub struct UWidgetSlotPair {
    pub widget_name: FName,
    pub slot_property_names: TArray<FName>,
    pub slot_property_values: TArray<FString>,
}
pub struct UWidgetBlueprintFactory {
    pub blueprint_type: crate::bindings::engine::EBlueprintType,
    pub parent_class: TSubclassOf<crate::bindings::umg::UUserWidget>,
    pub root_widget_class: TSubclassOf<crate::bindings::core_u_object::UObject>,
}
pub struct UWidgetBlueprintThumbnailRenderer {}
pub struct UWidgetCompilerRule {}
pub struct UWidgetGraphSchema {}
pub struct FAssetThumbnailWidgetSettings_HighlightedTextDelegate;
pub struct FSetThumbnailSettings_HighlightedTextDelegate;
pub struct FAssetThumbnailWidget_HighlightedTextDelegate;
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EThumbnailLabelType_BlueprintType(pub u8);
impl EThumbnailLabelType_BlueprintType {
    pub const CLASS_NAME: EThumbnailLabelType_BlueprintType = EThumbnailLabelType_BlueprintType(
        0,
    );
    pub const ASSET_NAME: EThumbnailLabelType_BlueprintType = EThumbnailLabelType_BlueprintType(
        1,
    );
    pub const NO_LABEL: EThumbnailLabelType_BlueprintType = EThumbnailLabelType_BlueprintType(
        2,
    );
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EThumbnailColorStripOrientation_BlueprintType(pub u8);
impl EThumbnailColorStripOrientation_BlueprintType {
    pub const HORIZONTAL_BOTTOM_EDGE: EThumbnailColorStripOrientation_BlueprintType = EThumbnailColorStripOrientation_BlueprintType(
        0,
    );
    pub const VERTICAL_RIGHT_EDGE: EThumbnailColorStripOrientation_BlueprintType = EThumbnailColorStripOrientation_BlueprintType(
        1,
    );
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EPropertyBindingPermissionLevel(pub u8);
impl EPropertyBindingPermissionLevel {
    pub const ALLOW: EPropertyBindingPermissionLevel = EPropertyBindingPermissionLevel(
        0,
    );
    pub const PREVENT: EPropertyBindingPermissionLevel = EPropertyBindingPermissionLevel(
        1,
    );
    pub const PREVENT_AND_WARN: EPropertyBindingPermissionLevel = EPropertyBindingPermissionLevel(
        2,
    );
    pub const PREVENT_AND_ERROR: EPropertyBindingPermissionLevel = EPropertyBindingPermissionLevel(
        3,
    );
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EWidgetCompileTimeTickPrediction(pub u8);
impl EWidgetCompileTimeTickPrediction {
    pub const WONT_TICK: EWidgetCompileTimeTickPrediction = EWidgetCompileTimeTickPrediction(
        0,
    );
    pub const ON_DEMAND: EWidgetCompileTimeTickPrediction = EWidgetCompileTimeTickPrediction(
        1,
    );
    pub const WILL_TICK: EWidgetCompileTimeTickPrediction = EWidgetCompileTimeTickPrediction(
        2,
    );
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EThumbnailPreviewSizeMode(pub u8);
impl EThumbnailPreviewSizeMode {
    pub const MATCH_DESIGNER_MODE: EThumbnailPreviewSizeMode = EThumbnailPreviewSizeMode(
        0,
    );
    pub const FILL_SCREEN: EThumbnailPreviewSizeMode = EThumbnailPreviewSizeMode(1);
    pub const CUSTOM: EThumbnailPreviewSizeMode = EThumbnailPreviewSizeMode(2);
    pub const DESIRED: EThumbnailPreviewSizeMode = EThumbnailPreviewSizeMode(3);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EDisplayOnCompile(pub u8);
impl EDisplayOnCompile {
    pub const DO_C_ERRORS_OR_WARNINGS: EDisplayOnCompile = EDisplayOnCompile(0);
    pub const DO_C_ERRORS_ONLY: EDisplayOnCompile = EDisplayOnCompile(1);
    pub const DO_C_WARNINGS_ONLY: EDisplayOnCompile = EDisplayOnCompile(2);
    pub const DO_C_NEVER: EDisplayOnCompile = EDisplayOnCompile(3);
}
