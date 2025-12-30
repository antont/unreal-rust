#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::prelude::*;
pub struct UAnimGraphNode_ControlRig {
    pub node: FAnimNode_ControlRig,
}
pub struct UControlRigAssetInterface {}
pub struct IControlRigAssetInterface {}
pub struct UControlRigBlueprint {
    pub modular_rig_settings: FModularRigSettings,
    pub hierarchy_settings: FRigHierarchySettings,
    pub rig_module_settings: FRigModuleSettings,
    pub custom_thumbnail: FString,
    pub module_reference_data: TArray<FModuleReferenceData>,
    pub connection_map_deprecated: TMap<FRigElementKey, FRigElementKey>,
    pub array_connection_map: TMap<FRigElementKey, FRigElementKeyCollection>,
    pub public_functions_deprecated: TArray<FRigVMOldPublicFunctionData>,
    pub gizmo_library_deprecated: TSoftObjectPtr<UControlRigShapeLibrary>,
    pub shape_libraries: TArray<TSoftObjectPtr<UControlRigShapeLibrary>>,
    pub statistics_deprecated: FRigVMStatistics,
    pub draw_container: FRigVMDrawContainer,
    pub influences: FRigInfluenceMapPerEvent,
    pub hierarchy_container_deprecated: FRigHierarchyContainer,
    pub hierarchy: UPtr<URigHierarchy>,
    pub modular_rig_model: FModularRigModel,
    pub control_rig_type: EControlRigType,
    pub item_type_display_name: FName,
    pub b_supports_inversion: bool,
    pub b_supports_controls: bool,
    pub preview_skeletal_mesh: TSoftObjectPtr<USkeletalMesh>,
    pub source_hierarchy_import: TSoftObjectPtr<UObject>,
    pub source_curve_import: TSoftObjectPtr<UObject>,
    pub b_exposes_animatable_controls: bool,
    pub b_allow_multiple_instances: bool,
    pub validator: UPtr<UControlRigValidator>,
    pub modules_recompilation_bracket: i32,
    pub debug_bone_radius: f32,
}
pub struct UControlRigSchema {}
pub struct UControlRigGraph {}
pub struct UControlRigGraphNode {}
pub struct UControlRigGraphSchema {}
