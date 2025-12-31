#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(8))]
pub struct FHierarchyElementIdentity {
    pub guids: TArray<crate::bindings::core_u_object::FGuid>,
    pub names: TArray<FName>,
}
#[repr(C, align(1))]
pub struct FDataHierarchyElementMetaData {}
#[repr(C, align(4))]
pub struct FDataHierarchyElementMetaData_SectionAssociation {
    pub section: TWeakObjectPtr<UHierarchySection>,
}
#[repr(C, align(1))]
pub struct FHierarchyElementChangedPayload {}
#[repr(C, align(8))]
pub struct FHierarchyElementChangedPayload_AddedElement {}
#[repr(C, align(8))]
pub struct FHierarchyElementChangedPayload_DeletedElement {}
#[repr(C, align(8))]
pub struct FHierarchyElementChangedPayload_ElementPropertyChanged {}
pub struct UDataHierarchyEditorMenuContext {}
pub struct UHierarchyDataRefreshContext {}
pub struct UHierarchyElement {
    pub children: TArray<UPtr<UHierarchyElement>>,
    pub identity: FHierarchyElementIdentity,
    pub meta_data: TMap<
        UPtr<crate::bindings::core_u_object::UStruct>,
        crate::bindings::core_u_object::FInstancedStruct,
    >,
    pub guid_deprecated: crate::bindings::core_u_object::FGuid,
}
pub struct UHierarchySection {
    pub section: FName,
    pub tooltip: FText,
}
pub struct UHierarchyRoot {
    pub sections: TArray<UPtr<UHierarchySection>>,
}
pub struct UHierarchyItem {}
pub struct UHierarchyCategory {
    pub category: FName,
    pub tooltip: FText,
    pub section_deprecated: UPtr<UHierarchySection>,
}
pub struct UHierarchyMenuContext {}
pub struct UDataHierarchyViewModelBase {
    pub hierarchy_root: UPtr<UHierarchyRoot>,
    pub refresh_context: UPtr<UHierarchyDataRefreshContext>,
    pub b_is_initialized: bool,
    pub b_is_finalized: bool,
    pub all_section: UPtr<UHierarchySection>,
}
