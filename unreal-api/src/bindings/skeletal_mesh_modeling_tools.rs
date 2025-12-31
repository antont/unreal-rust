#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
pub struct USkeletalMeshBackedDynamicMeshComponentProvider {}
pub struct ISkeletalMeshBackedDynamicMeshComponentProvider {}
pub struct USkeletalMeshBackedDynamicMeshComponent {}
pub struct USkeletalMeshEditingCache {
    pub host_actor: UPtr<crate::bindings::engine::AActor>,
    pub editing_mesh_component: UPtr<USkeletalMeshBackedDynamicMeshComponent>,
    pub preview_mesh: UPtr<crate::bindings::modeling_components::UPreviewMesh>,
    pub skeletal_mesh_component: TWeakObjectPtr<
        crate::bindings::engine::USkeletalMeshComponent,
    >,
    pub ref_skeleton_poser: UPtr<
        crate::bindings::mesh_modeling_tools_editor_only::URefSkeletonPoser,
    >,
    pub morph_target_overrides: TMap<FName, f32>,
}
pub struct USkeletalMeshEditorContextObject {
    pub skeleton_draw_mode: crate::bindings::unreal_ed::ESkeletonDrawMode,
}
pub struct USkeletalMeshGizmoWrapper {
    pub transform_gizmo: UPtr<
        crate::bindings::editor_interactive_tools_framework::UTransformGizmo,
    >,
    pub transform_proxy: UPtr<
        crate::bindings::mesh_modeling_tools_editor_only::USkeletonTransformProxy,
    >,
}
pub struct USkeletalMeshGizmoContextObject {}
pub struct USkeletalMeshModelingToolsEditorMode {
    pub current_editing_cache: UPtr<USkeletalMeshEditingCache>,
    pub skeleton_reader: UPtr<
        crate::bindings::skeletal_mesh_modifiers::USkeletonModifier,
    >,
}
pub struct USkeletonFromStaticMeshFactory {
    pub static_mesh: UPtr<crate::bindings::engine::UStaticMesh>,
    pub root_position: crate::bindings::core_u_object::FVector,
    pub position_reference: ERootBonePositionReference,
}
pub struct USkeletalMeshFromStaticMeshFactory {
    pub skeleton: UPtr<crate::bindings::engine::USkeleton>,
    pub static_mesh: UPtr<crate::bindings::engine::UStaticMesh>,
    pub bind_bone_name: FName,
}
pub struct UStaticMeshToSkeletalMeshConvertOptions {
    pub destination_path: crate::bindings::core_u_object::FDirectoryPath,
    pub skeleton_import_option: EReferenceSkeletonImportOption,
    pub root_bone_placement: ERootBonePlacementOptions,
    pub skeleton: crate::bindings::core_u_object::FSoftObjectPath,
    pub skeletal_mesh: crate::bindings::core_u_object::FSoftObjectPath,
    pub binding_bone_name: crate::bindings::engine::FBoneReference,
    pub prefix_to_remove: FString,
    pub skeletal_mesh_prefix_to_add: FString,
    pub skeletal_mesh_suffix_to_add: FString,
    pub skeleton_prefix_to_add: FString,
    pub skeleton_suffix_to_add: FString,
}
pub struct USkeletalMeshBackedDynamicMeshComponentToolTarget {}
pub struct USkeletalMeshBackedDynamicMeshComponentToolTargetFactory {}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ERootBonePositionReference(pub i32);
impl ERootBonePositionReference {
    pub const RELATIVE: ERootBonePositionReference = ERootBonePositionReference(0);
    pub const ABSOLUTE: ERootBonePositionReference = ERootBonePositionReference(1);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EReferenceSkeletonImportOption(pub i32);
impl EReferenceSkeletonImportOption {
    pub const CREATE_NEW: EReferenceSkeletonImportOption = EReferenceSkeletonImportOption(
        0,
    );
    pub const USE_EXISTING_SKELETON: EReferenceSkeletonImportOption = EReferenceSkeletonImportOption(
        1,
    );
    pub const USE_EXISTING_SKELETAL_MESH: EReferenceSkeletonImportOption = EReferenceSkeletonImportOption(
        2,
    );
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ERootBonePlacementOptions(pub i32);
impl ERootBonePlacementOptions {
    pub const BOTTOM_CENTER: ERootBonePlacementOptions = ERootBonePlacementOptions(0);
    pub const CENTER: ERootBonePlacementOptions = ERootBonePlacementOptions(1);
    pub const ORIGIN: ERootBonePlacementOptions = ERootBonePlacementOptions(2);
}
