#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::prelude::*;
#[repr(C, align(1))]
pub struct FSubobjectDataSubsystemContextDataBase {}
#[repr(C, align(4))]
pub struct FSubobjectDataSubsystemContextData_SingleUObjectContextObject {
    pub object: TWeakObjectPtr<UObject>,
}
#[repr(C, align(8))]
pub struct FSubobjectDataSubsystemContextData_TedsRow {
    pub row_handle: FTedsRowHandle,
}
#[repr(C, align(8))]
pub struct FSubobjectData {
    pub weak_object_ptr: TWeakObjectPtr<UObject>,
    pub handle: FSubobjectDataHandle,
    pub parent_object_handle: FSubobjectDataHandle,
    pub children_handles: TArray<FSubobjectDataHandle>,
    pub context_data: FInstancedStruct,
}
#[repr(C, align(8))]
pub struct FSubobjectDataHandle {}
#[repr(C, align(8))]
pub struct FAddNewSubobjectParams {
    pub parent_handle: FSubobjectDataHandle,
    pub new_class: TSubclassOf<UObject>,
    pub blueprint_context: UPtr<UBlueprint>,
    pub flags_40: u8,
}
#[repr(C, align(8))]
pub struct FReparentSubobjectParams {
    pub new_parent_handle: FSubobjectDataHandle,
    pub blueprint_context: UPtr<UBlueprint>,
    pub actor_preview_context: UPtr<AActor>,
}
pub struct USubobjectDataBlueprintFunctionLibrary {}
pub struct USubobjectDataSubsystem {}
