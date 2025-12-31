#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(8))]
pub struct FAssetActionSupportCondition {
    pub filter: FString,
    pub failure_reason: FString,
    pub b_show_in_menu_if_filter_fails: bool,
}
#[repr(C, align(8))]
pub struct FBlutilityFunctionData {
    pub class: TSoftObjectPtr<crate::bindings::core_u_object::UClass>,
    pub name: FName,
    pub name_text: FText,
    pub category: FString,
    pub tooltip_text: FText,
}
pub struct UEditorFunctionLibrary {}
pub struct UEditorUtilityToolMenuEntry {
    pub b_run_editor_utility_on_startup: bool,
}
pub struct UEditorUtilityToolMenuSection {
    pub b_run_editor_utility_on_startup: bool,
}
pub struct UEditorUtilityExtension {}
pub struct IEditorUtilityExtension {}
pub struct UEditorUtilityObject {
    pub b_run_editor_utility_on_startup: bool,
}
pub struct UActorActionUtility {
    pub supported_classes: TArray<
        TSoftObjectPtr<crate::bindings::core_u_object::UClass>,
    >,
}
pub struct UAssetActionUtility {
    pub b_is_action_for_blueprints: bool,
    pub supported_classes: TArray<
        TSoftObjectPtr<crate::bindings::core_u_object::UClass>,
    >,
    pub supported_conditions: TArray<FAssetActionSupportCondition>,
}
pub struct UAsyncCaptureScene {
    pub complete: FAsyncCaptureScene_Complete,
    pub scene_capture: UPtr<crate::bindings::engine::ASceneCapture2D>,
    pub scene_capture_rt: UPtr<crate::bindings::engine::UTextureRenderTarget2D>,
}
pub struct UAsyncImageExport {
    pub complete: FAsyncImageExport_Complete,
    pub texture_to_export: UPtr<crate::bindings::engine::UTexture>,
    pub quality: i32,
    pub target_file: FString,
}
pub struct UAsyncRegisterAndExecuteTask {
    pub on_finished: FAsyncRegisterAndExecuteTask_OnFinished,
}
pub struct AEditorUtilityActor {
    pub editor_only_input_component: UPtr<crate::bindings::engine::UInputComponent>,
    pub b_receives_editor_input: bool,
}
pub struct UEditorUtilityActorComponent {}
pub struct UEditorUtilityBlueprint {}
pub struct UEditorUtilityBlueprintFactory {
    pub parent_class: TSubclassOf<crate::bindings::core_u_object::UObject>,
}
pub struct AEditorUtilityCamera {}
pub struct UEditorUtilityBlueprintAsyncActionBase {}
pub struct UAsyncEditorDelay {
    pub complete: FAsyncEditorDelay_Complete,
}
pub struct UAsyncEditorWaitForGameWorld {
    pub complete: FAsyncEditorWaitForGameWorld_Complete,
}
pub struct UAsyncEditorOpenMapAndFocusActor {
    pub complete: FAsyncEditorOpenMapAndFocusActor_Complete,
}
pub struct UEditorUtilityLibrary {}
pub struct UEditorUtilitySubsystem {
    pub loaded_u_is: TArray<crate::bindings::core_u_object::FSoftObjectPath>,
    pub startup_objects: TArray<crate::bindings::core_u_object::FSoftObjectPath>,
    pub on_begin_pie: FEditorUtilitySubsystem_OnBeginPIE,
    pub on_end_pie: FEditorUtilitySubsystem_OnEndPIE,
    pub object_instances: TMap<
        UPtr<crate::bindings::core_u_object::UObject>,
        UPtr<crate::bindings::core_u_object::UObject>,
    >,
    pub active_task_stack: TArray<UPtr<UEditorUtilityTask>>,
    pub referenced_objects: TSet<UPtr<crate::bindings::core_u_object::UObject>>,
}
pub struct UEditorUtilityTask {
    pub b_run_editor_utility_on_startup: bool,
    pub my_task_manager: UPtr<UEditorUtilitySubsystem>,
    pub my_parent_task: UPtr<UEditorUtilityTask>,
    pub b_cancel_requested: bool,
}
pub struct UEditorUtilityWidget {
    pub tab_display_name: FText,
    pub help_text: FString,
    pub b_always_reregister_with_windows_menu: bool,
    pub b_auto_run_default_action: bool,
    pub b_run_editor_utility_on_startup: bool,
}
pub struct UEditorUtilityWidgetBlueprint {
    pub b_is_enabled_in_pie: bool,
    pub b_is_enabled_in_debugging: bool,
    pub b_spawn_as_nomad_tab: bool,
    pub created_umg_widget: UPtr<UEditorUtilityWidget>,
}
pub struct UEditorUtilityWidgetBlueprintFactory {
    pub blueprint_type: crate::bindings::engine::EBlueprintType,
    pub parent_class: TSubclassOf<crate::bindings::umg::UUserWidget>,
}
pub struct UEditorUtilityButton {}
pub struct UEditorUtilityCheckBox {}
pub struct UEditorUtilityCircularThrobber {}
pub struct UEditorUtilityComboBoxKey {}
pub struct UEditorUtilityComboBoxString {}
pub struct UEditorUtilityEditableText {}
pub struct UEditorUtilityEditableTextBox {}
pub struct UEditorUtilityExpandableArea {}
pub struct UEditorUtilityInputKeySelector {}
pub struct UEditorUtilityListView {}
pub struct UEditorUtilityMultiLineEditableText {}
pub struct UEditorUtilityMultiLineEditableTextBox {}
pub struct UEditorUtilityProgressBar {}
pub struct UEditorUtilityScrollBar {}
pub struct UEditorUtilityScrollBox {}
pub struct UEditorUtilitySlider {}
pub struct UEditorUtilitySpinBox {}
pub struct UEditorUtilityThrobber {}
pub struct UEditorUtilityTreeView {}
pub struct UEditorUtilityWidgetProjectSettings {
    pub b_search_generated_classes_for_scripted_actions: bool,
}
pub struct UDEPRECATED_GlobalEditorUtilityBase {
    pub help_text: FString,
    pub b_dirtied_selection_set: bool,
    pub b_auto_run_default_action: bool,
    pub on_each_selected_actor: FGlobalEditorUtilityBase_OnEachSelectedActor,
    pub on_each_selected_asset: FGlobalEditorUtilityBase_OnEachSelectedAsset,
}
pub struct ADEPRECATED_PlacedEditorUtilityBase {
    pub help_text: FString,
}
pub struct UToolMenuWidget {
    pub menu_name: FString,
    pub menu_type: crate::bindings::slate::EMultiBoxType,
    pub full_menu_name: FName,
}
pub struct FAsyncCaptureScene_Complete;
pub struct FAsyncImageExport_Complete;
pub struct FAsyncRegisterAndExecuteTask_OnFinished;
pub struct FAsyncEditorDelay_Complete;
pub struct FAsyncEditorWaitForGameWorld_Complete;
pub struct FAsyncEditorOpenMapAndFocusActor_Complete;
pub struct FEditorUtilitySubsystem_OnBeginPIE;
pub struct FEditorUtilitySubsystem_OnEndPIE;
pub struct FGlobalEditorUtilityBase_OnEachSelectedActor;
pub struct FGlobalEditorUtilityBase_OnEachSelectedAsset;
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ECastToWidgetBlueprintCases(pub u8);
impl ECastToWidgetBlueprintCases {
    pub const CAST_SUCCEEDED: ECastToWidgetBlueprintCases = ECastToWidgetBlueprintCases(
        0,
    );
    pub const CAST_FAILED: ECastToWidgetBlueprintCases = ECastToWidgetBlueprintCases(1);
}
