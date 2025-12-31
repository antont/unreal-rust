#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(8))]
pub struct FEditorDataHierarchyData_Template {
    pub parent: crate::bindings::typed_element_framework::FTedsRowHandle,
    pub children: TArray<crate::bindings::typed_element_framework::FTedsRowHandle>,
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
    pub memento_type: UPtr<crate::bindings::core_u_object::UScriptStruct>,
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
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EChunkMemorySize(pub i32);
impl EChunkMemorySize {
    pub const SIZE4_KB: EChunkMemorySize = EChunkMemorySize(4096);
    pub const SIZE8_KB: EChunkMemorySize = EChunkMemorySize(8192);
    pub const SIZE16_KB: EChunkMemorySize = EChunkMemorySize(16384);
    pub const SIZE32_KB: EChunkMemorySize = EChunkMemorySize(32768);
    pub const SIZE64_KB: EChunkMemorySize = EChunkMemorySize(65536);
    pub const SIZE128_KB: EChunkMemorySize = EChunkMemorySize(131072);
    pub const SIZE256_KB: EChunkMemorySize = EChunkMemorySize(262144);
    pub const SIZE512_KB: EChunkMemorySize = EChunkMemorySize(524288);
}
