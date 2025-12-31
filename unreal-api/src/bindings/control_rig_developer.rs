#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
pub struct UAnimGraphNode_ControlRig {
    pub node: crate::bindings::control_rig::FAnimNode_ControlRig,
}
pub struct UControlRigAssetInterface {}
pub struct IControlRigAssetInterface {}
pub struct UControlRigBlueprint {
    pub modular_rig_settings: crate::bindings::control_rig::FModularRigSettings,
    pub hierarchy_settings: crate::bindings::control_rig::FRigHierarchySettings,
    pub rig_module_settings: crate::bindings::control_rig::FRigModuleSettings,
    pub custom_thumbnail: FString,
    pub module_reference_data: TArray<
        crate::bindings::control_rig::FModuleReferenceData,
    >,
    pub connection_map_deprecated: TMap<
        crate::bindings::control_rig::FRigElementKey,
        crate::bindings::control_rig::FRigElementKey,
    >,
    pub array_connection_map: TMap<
        crate::bindings::control_rig::FRigElementKey,
        crate::bindings::control_rig::FRigElementKeyCollection,
    >,
    pub public_functions_deprecated: TArray<
        crate::bindings::rig_vm_developer::FRigVMOldPublicFunctionData,
    >,
    pub gizmo_library_deprecated: TSoftObjectPtr<
        crate::bindings::control_rig::UControlRigShapeLibrary,
    >,
    pub shape_libraries: TArray<
        TSoftObjectPtr<crate::bindings::control_rig::UControlRigShapeLibrary>,
    >,
    pub statistics_deprecated: crate::bindings::rig_vm::FRigVMStatistics,
    pub draw_container: crate::bindings::rig_vm::FRigVMDrawContainer,
    pub influences: crate::bindings::control_rig::FRigInfluenceMapPerEvent,
    pub hierarchy_container_deprecated: crate::bindings::control_rig::FRigHierarchyContainer,
    pub hierarchy: UPtr<crate::bindings::control_rig::URigHierarchy>,
    pub modular_rig_model: crate::bindings::control_rig::FModularRigModel,
    pub control_rig_type: crate::bindings::control_rig::EControlRigType,
    pub item_type_display_name: FName,
    pub b_supports_inversion: bool,
    pub b_supports_controls: bool,
    pub preview_skeletal_mesh: TSoftObjectPtr<crate::bindings::engine::USkeletalMesh>,
    pub source_hierarchy_import: TSoftObjectPtr<crate::bindings::core_u_object::UObject>,
    pub source_curve_import: TSoftObjectPtr<crate::bindings::core_u_object::UObject>,
    pub b_exposes_animatable_controls: bool,
    pub b_allow_multiple_instances: bool,
    pub validator: UPtr<crate::bindings::control_rig::UControlRigValidator>,
    pub modules_recompilation_bracket: i32,
    pub debug_bone_radius: f32,
}
pub struct UControlRigSchema {}
pub struct UControlRigGraph {}
pub struct UControlRigGraphNode {}
pub struct UControlRigGraphSchema {}
