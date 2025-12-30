#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::prelude::*;
pub struct USkeletalMeshBackedDynamicMeshComponentProvider {}
pub struct ISkeletalMeshBackedDynamicMeshComponentProvider {}
pub struct USkeletalMeshBackedDynamicMeshComponent {}
pub struct USkeletalMeshEditingCache {
    pub host_actor: UPtr<AActor>,
    pub editing_mesh_component: UPtr<USkeletalMeshBackedDynamicMeshComponent>,
    pub preview_mesh: UPtr<UPreviewMesh>,
    pub skeletal_mesh_component: TWeakObjectPtr<USkeletalMeshComponent>,
    pub ref_skeleton_poser: UPtr<URefSkeletonPoser>,
    pub morph_target_overrides: TMap<FName, f32>,
}
pub struct USkeletalMeshEditorContextObject {
    pub skeleton_draw_mode: ESkeletonDrawMode,
}
pub struct USkeletalMeshGizmoWrapper {
    pub transform_gizmo: UPtr<UTransformGizmo>,
    pub transform_proxy: UPtr<USkeletonTransformProxy>,
}
pub struct USkeletalMeshGizmoContextObject {}
pub struct USkeletalMeshModelingToolsEditorMode {
    pub current_editing_cache: UPtr<USkeletalMeshEditingCache>,
    pub skeleton_reader: UPtr<USkeletonModifier>,
}
pub struct USkeletonFromStaticMeshFactory {
    pub static_mesh: UPtr<UStaticMesh>,
    pub root_position: FVector,
    pub position_reference: ERootBonePositionReference,
}
pub struct USkeletalMeshFromStaticMeshFactory {
    pub skeleton: UPtr<USkeleton>,
    pub static_mesh: UPtr<UStaticMesh>,
    pub bind_bone_name: FName,
}
pub struct UStaticMeshToSkeletalMeshConvertOptions {
    pub destination_path: FDirectoryPath,
    pub skeleton_import_option: EReferenceSkeletonImportOption,
    pub root_bone_placement: ERootBonePlacementOptions,
    pub skeleton: FSoftObjectPath,
    pub skeletal_mesh: FSoftObjectPath,
    pub binding_bone_name: FBoneReference,
    pub prefix_to_remove: FString,
    pub skeletal_mesh_prefix_to_add: FString,
    pub skeletal_mesh_suffix_to_add: FString,
    pub skeleton_prefix_to_add: FString,
    pub skeleton_suffix_to_add: FString,
}
pub struct USkeletalMeshBackedDynamicMeshComponentToolTarget {}
pub struct USkeletalMeshBackedDynamicMeshComponentToolTargetFactory {}
