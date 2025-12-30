#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::prelude::*;
#[repr(C, align(8))]
pub struct FEditorDataHierarchyData_Template {
    pub parent: FTedsRowHandle,
    pub children: TArray<FTedsRowHandle>,
}
#[repr(C, align(1))]
pub struct FEditorDataHierarchyParentTag_Template {}
#[repr(C, align(1))]
pub struct FEditorDataHierarchyChildTag_Template {}
#[repr(C, align(8))]
pub struct FEditorDataHierarchyUnresolvedParent_Template {}
#[repr(C, align(8))]
pub struct FTestDynamicColumn {}
#[repr(C, align(1))]
pub struct FTestDynamicTag {}
#[repr(C, align(1))]
pub struct FTedsSharedColumn {}
#[repr(C, align(1))]
pub struct FTypedElementMementoTag {}
#[repr(C, align(4))]
pub struct FTedsValueTagColumn {
    pub value: FName,
}
#[repr(C, align(16))]
pub struct FTypedElementQueryProcessorData {}
pub struct UEditorDataStorageSettings {
    pub chunk_memory_size: EChunkMemorySize,
    pub table_specific_chunk_memory_size: TMap<FName, EChunkMemorySize>,
}
pub struct UTedsMementoTranslatorBase {}
pub struct UTedsDefaultMementoTranslator {
    pub memento_type: UPtr<UScriptStruct>,
}
pub struct UTedsSelectionColumnMementoTranslator {}
pub struct UTedsObjectReinstancingManager {
    pub data_storage: UPtr<UEditorDataStorage>,
    pub data_storage_compatibility: UPtr<UEditorDataStorageCompatibility>,
}
pub struct UObjectNameDataStorageFactory {}
pub struct UObjectWorldDataStorageFactory {}
pub struct UTypedElementQueryProcessorCallbackAdapterProcessorBase {
    pub data: FTypedElementQueryProcessorData,
}
pub struct UTypedElementQueryProcessorCallbackAdapterProcessor {}
pub struct UTypedElementQueryProcessorCallbackAdapterProcessorWith1Subquery {}
pub struct UTypedElementQueryProcessorCallbackAdapterProcessorWith2Subqueries {}
pub struct UTypedElementQueryProcessorCallbackAdapterProcessorWith3Subqueries {}
pub struct UTypedElementQueryProcessorCallbackAdapterProcessorWith4Subqueries {}
pub struct UTypedElementQueryProcessorCallbackAdapterProcessorWith5Subqueries {}
pub struct UTypedElementQueryProcessorCallbackAdapterProcessorWith6Subqueries {}
pub struct UTypedElementQueryProcessorCallbackAdapterProcessorWith7Subqueries {}
pub struct UTypedElementQueryProcessorCallbackAdapterProcessorWith8Subqueries {}
pub struct UTypedElementQueryObserverCallbackAdapterProcessorBase {}
pub struct UTypedElementQueryObserverCallbackAdapterProcessor {}
pub struct UTypedElementQueryObserverCallbackAdapterProcessorWith1Subquery {}
pub struct UTypedElementQueryObserverCallbackAdapterProcessorWith2Subqueries {}
pub struct UTypedElementQueryObserverCallbackAdapterProcessorWith3Subqueries {}
pub struct UTypedElementQueryObserverCallbackAdapterProcessorWith4Subqueries {}
pub struct UTypedElementQueryObserverCallbackAdapterProcessorWith5Subqueries {}
pub struct UTypedElementQueryObserverCallbackAdapterProcessorWith6Subqueries {}
pub struct UTypedElementQueryObserverCallbackAdapterProcessorWith7Subqueries {}
pub struct UTypedElementQueryObserverCallbackAdapterProcessorWith8Subqueries {}
pub struct UDefaultPropertySorterFactory {}
pub struct UTypedElementHiearchyQueriesFactory {}
pub struct UTypedElementRemoveSyncToWorldTagFactory {}
pub struct UEditorDataStorage {}
pub struct UEditorDataStorageCompatibility {}
pub struct UEditorDataStorageUi {}
