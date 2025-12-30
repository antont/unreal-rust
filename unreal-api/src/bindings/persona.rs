#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::prelude::*;
#[repr(C, align(8))]
pub struct FPhysicsAssetRenderSettings {
    pub center_of_mass_view_mode: EPhysicsAssetEditorCenterOfMassViewMode,
    pub collision_view_mode: EPhysicsAssetEditorCollisionViewMode,
    pub constraint_view_mode: EPhysicsAssetEditorConstraintViewMode,
    pub constraint_viewport_manipulation_flags: EConstraintTransformComponentFlags,
    pub constraint_transform_component_display_relative_to_default_flags: EConstraintTransformComponentFlags,
    pub constraint_draw_size: f32,
    pub physics_blend: f32,
    pub b_hide_kinematic_bodies: bool,
    pub b_hide_simulated_bodies: bool,
    pub b_hide_body_mass: bool,
    pub b_render_only_selected_constraints: bool,
    pub b_show_com_deprecated: bool,
    pub b_show_constraints_as_points: bool,
    pub b_highlight_overlaping_bodies: bool,
    pub b_draw_violated_limits: bool,
    pub b_hide_center_of_mass_for_kinematic_bodies: bool,
    pub bone_unselected_color: FColor,
    pub no_collision_color: FColor,
    pub com_render_color: FColor,
    pub com_render_size: f32,
    pub com_render_line_thickness: f32,
    pub com_render_mass_text_offset_screenspace: f32,
    pub influence_line_length: f32,
    pub bone_unselected_material: UPtr<UMaterialInterface>,
    pub bone_no_collision_material: UPtr<UMaterialInterface>,
    pub hidden_bodies: TArray<i32>,
    pub hidden_constraints: TArray<i32>,
}
#[repr(C, align(8))]
pub struct FAnimCurveMetadataEditorClipboardEntry {
    pub curve_name: FName,
    pub meta_data: FCurveMetaData,
}
#[repr(C, align(8))]
pub struct FAnimCurveMetadataEditorClipboard {
    pub entries: TArray<FAnimCurveMetadataEditorClipboardEntry>,
}
pub struct UAnimationEditorsAssetFamilyExtension {}
pub struct UAnimationSequenceBrowserContextMenuContext {
    pub selected_objects: TArray<TWeakObjectPtr<UObject>>,
}
pub struct UAnimNotifyPanelContextMenuContext {}
pub struct UAnimViewportContext {}
pub struct UAnimViewportToolBarToolMenuContext {}
pub struct UCachedAnalysisProperties {}
pub struct ULinearAnalysisPropertiesBase {
    pub bone_socket: FBoneSocketTarget,
    pub space: EAnalysisSpace,
    pub space_bone_socket: FBoneSocketTarget,
    pub start_time_fraction: f32,
    pub end_time_fraction: f32,
}
pub struct ULinearAnalysisProperties {
    pub function_axis: EAnalysisLinearAxis,
}
pub struct UEulerAnalysisProperties {
    pub function_axis: EAnalysisEulerAxis,
    pub bone_socket: FBoneSocketTarget,
    pub bone_facing_axis: EAnalysisLinearAxis,
    pub bone_right_axis: EAnalysisLinearAxis,
    pub euler_calculation_method: EEulerCalculationMethod,
    pub space: EAnalysisSpace,
    pub space_bone_socket: FBoneSocketTarget,
    pub character_facing_axis: EAnalysisLinearAxis,
    pub character_up_axis: EAnalysisLinearAxis,
    pub start_time_fraction: f32,
    pub end_time_fraction: f32,
}
pub struct UPersonaPreviewSceneDescription {
    pub preview_controller: TSubclassOf<UPersonaPreviewSceneController>,
    pub preview_controller_instance: UPtr<UPersonaPreviewSceneController>,
    pub preview_controller_instances: TArray<UPtr<UPersonaPreviewSceneController>>,
    pub preview_mesh: TSoftObjectPtr<USkeletalMesh>,
    pub preview_animation_blueprint: TSoftObjectPtr<UAnimBlueprint>,
    pub application_method: EPreviewAnimationBlueprintApplicationMethod,
    pub linked_anim_graph_tag: FName,
    pub additional_meshes: TSoftObjectPtr<UDataAsset>,
    pub default_additional_meshes: UPtr<UPreviewMeshCollection>,
}
pub struct UAnimAssetFindReplaceContext {}
pub struct UAnimAssetFindReplaceProcessor {}
pub struct UAnimAssetFindReplaceProcessor_StringBase {}
pub struct UAnimAssetFindReplaceCurves {}
pub struct UAnimAssetFindReplaceNotifies {}
pub struct UAnimAssetFindReplaceSyncMarkers {}
pub struct AAnimationEditorPreviewActor {}
pub struct UAnimCurveBaseCopyObject {
    pub curve_name: FName,
    pub curve_type: ERawCurveTrackTypes,
    pub channel: ETransformCurveChannel,
    pub axis: EVectorCurveChannel,
    pub origin_name: FName,
}
pub struct UFloatCurveCopyObject {
    pub curve: FFloatCurve,
}
pub struct UTransformCurveCopyObject {
    pub curve: FTransformCurve,
}
pub struct UVectorCurveCopyObject {
    pub curve: FVectorCurve,
}
pub struct UAnimTimelineClipboardContent {
    pub curves: TArray<UPtr<UAnimCurveBaseCopyObject>>,
}
pub struct UPersonaManagerContext {}
pub struct IPersonaManagerContext {}
pub struct UPersonaEditorModeManagerContext {}
pub struct ULODInfoUILayout {
    pub lod_info: FSkeletalMeshLODInfo,
}
pub struct UAnimationEditorsAssetFamilyExtension_SkeletonAsset {}
pub struct UAnimationEditorsAssetFamilyExtension_SkeletalMeshAsset {}
pub struct UAnimationEditorsAssetFamilyExtension_AnimationAsset {}
pub struct UAnimationEditorsAssetFamilyExtension_AnimBlueprintAsset {}
pub struct UAnimationEditorsAssetFamilyExtension_PhysicsAsset {}
pub struct UPersonaPreviewSceneController {}
pub struct UPersonaPreviewSceneAnimationController {
    pub animation: TSoftObjectPtr<UAnimationAsset>,
}
pub struct UPersonaPreviewSceneDefaultController {}
pub struct UPersonaPreviewSceneRefPoseController {
    pub b_reset_bone_transforms: bool,
}
pub struct UPersonaPreviewSceneSkelMeshInstanceController {
    pub active_preview_instance: TWeakObjectPtr<USkeletalMeshComponent>,
}
pub struct UPersonaToolMenuContext {}
pub struct UPhysicsAssetRenderUtilities {
    pub id_to_settings_map: TMap<u32, FPhysicsAssetRenderSettings>,
    pub bone_unselected_material: UPtr<UMaterialInterface>,
    pub bone_no_collision_material: UPtr<UMaterialInterface>,
}
pub struct USkinWeightImportOptions {
    pub profile_name: FString,
    pub file_path: FString,
    pub lod_index: i32,
}
