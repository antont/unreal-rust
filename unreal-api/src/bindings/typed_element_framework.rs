#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(8))]
pub struct FScriptTypedElementHandle {}
#[repr(C, align(1))]
pub struct FEditorDataStorageTag {}
#[repr(C, align(1))]
pub struct FEditorDataStorageUiDecoratorWidgetFactoryTag {}
#[repr(C, align(8))]
pub struct FTedsRowHandle {
    pub row_handle: u64,
}
#[repr(C, align(1))]
pub struct FEditorDataStorageColumn {}
#[repr(C, align(4))]
pub struct FTypedElementUObjectColumn {}
#[repr(C, align(4))]
pub struct FTypedElementUObjectIdColumn {
    pub id: u32,
    pub serial_number: i32,
}
#[repr(C, align(8))]
pub struct FTypedElementExternalObjectColumn {}
#[repr(C, align(1))]
pub struct FTypedElementClassDefaultObjectTag {}
#[repr(C, align(1))]
pub struct FTypedElementActorTag {}
#[repr(C, align(4))]
pub struct FTypedElementWorldColumn {}
#[repr(C, align(1))]
pub struct FEditorDataStorageWorldTag {}
#[repr(C, align(4))]
pub struct FTypedElementLevelColumn {}
#[repr(C, align(1))]
pub struct FEditorDataStorageLevelTag {}
#[repr(C, align(1))]
pub struct FTypedElementPropertyBagPlaceholderTag {}
#[repr(C, align(1))]
pub struct FTypedElementLoosePropertyTag {}
#[repr(C, align(4))]
pub struct FTypedElementPropertyBagPlaceholderTypeInfoColumn {}
#[repr(C, align(4))]
pub struct FEditorDataStorageUObjectIdNameColumn {
    pub id_name: FName,
}
#[repr(C, align(1))]
pub struct FFolderTag {}
#[repr(C, align(8))]
pub struct FTedsTypedElementColumn {}
#[repr(C, align(8))]
pub struct FTableRowParentColumn {}
#[repr(C, align(8))]
pub struct FUnresolvedTableRowParentColumn {}
#[repr(C, align(4))]
pub struct FTypedElementIconOverrideColumn {
    pub icon_name: FName,
}
#[repr(C, align(8))]
pub struct FTypedElementLabelColumn {
    pub label: FString,
}
#[repr(C, align(8))]
pub struct FTypedElementLabelHashColumn {
    pub label_hash: u64,
}
#[repr(C, align(8))]
pub struct FScriptTypedElementListProxy {}
#[repr(C, align(1))]
pub struct FTypedElementSyncBackToWorldTag {}
#[repr(C, align(1))]
pub struct FTypedElementSyncFromWorldTag {}
#[repr(C, align(1))]
pub struct FTypedElementSyncFromWorldInteractiveTag {}
#[repr(C, align(8))]
pub struct FTypedElementRowReferenceColumn {}
#[repr(C, align(4))]
pub struct FNameColumn {
    pub name: FName,
}
#[repr(C, align(1))]
pub struct FFavoriteTag {}
#[repr(C, align(1))]
pub struct FObjectOverrideColumn {}
#[repr(C, align(8))]
pub struct FTypedElementPackageUnresolvedReference {}
#[repr(C, align(8))]
pub struct FTypedElementPackageReference {}
#[repr(C, align(1))]
pub struct FTypedElementPackageUpdatedTag {}
#[repr(C, align(8))]
pub struct FTypedElementPackagePathColumn {
    pub path: FString,
}
#[repr(C, align(8))]
pub struct FTypedElementPackageLoadedPathColumn {}
#[repr(C, align(8))]
pub struct FTypedElementPivotOffset {
    pub offset: crate::bindings::core_u_object::FVector,
}
#[repr(C, align(1))]
pub struct FSCCInChangelistTag {}
#[repr(C, align(1))]
pub struct FSCCStagedTag {}
#[repr(C, align(1))]
pub struct FSCCLockedTag {}
#[repr(C, align(1))]
pub struct FSCCExternallyEditedTag {}
#[repr(C, align(1))]
pub struct FSCCNotCurrentTag {}
#[repr(C, align(4))]
pub struct FSCCStatusColumn {
    pub modification: ESCCModification,
}
#[repr(C, align(4))]
pub struct FSCCRevisionId {
    pub id: u32,
}
#[repr(C, align(4))]
pub struct FSCCRevisionIdColumn {
    pub revision_id: FSCCRevisionId,
}
#[repr(C, align(4))]
pub struct FSCCExternalRevisionIdColumn {
    pub revision_id: FSCCRevisionId,
}
#[repr(C, align(8))]
pub struct FSCCUserInfo {
    pub name: FString,
}
#[repr(C, align(8))]
pub struct FSCCExternallyLockedColumn {
    pub locked_by: FSCCUserInfo,
}
#[repr(C, align(4))]
pub struct FTypedElementSelectionColumn {
    pub selection_set: FName,
}
#[repr(C, align(8))]
pub struct FTypedElementSlateWidgetReferenceColumn {}
#[repr(C, align(1))]
pub struct FTypedElementSlateWidgetReferenceDeletesRowTag {}
#[repr(C, align(8))]
pub struct FDisplayNameColumn {
    pub display_name: FText,
}
#[repr(C, align(8))]
pub struct FDescriptionColumn {
    pub description: FText,
}
#[repr(C, align(4))]
pub struct FSlateColorColumn {
    pub color: crate::bindings::slate_core::FSlateColor,
}
#[repr(C, align(1))]
pub struct FIsInEditingModeTag {}
#[repr(C, align(8))]
pub struct FWidgetPurposeReferenceColumn {}
#[repr(C, align(1))]
pub struct FTestColumnA {}
#[repr(C, align(1))]
pub struct FTestColumnB {}
#[repr(C, align(1))]
pub struct FTestColumnC {}
#[repr(C, align(1))]
pub struct FTestColumnD {}
#[repr(C, align(1))]
pub struct FTestColumnE {}
#[repr(C, align(1))]
pub struct FTestColumnF {}
#[repr(C, align(1))]
pub struct FTestColumnG {}
#[repr(C, align(1))]
pub struct FTestColumnDynamic {}
#[repr(C, align(4))]
pub struct FTestColumnInt {
    pub test_int: i32,
}
#[repr(C, align(8))]
pub struct FTestColumnString {
    pub test_string: FString,
}
#[repr(C, align(1))]
pub struct FTestTagColumnA {}
#[repr(C, align(1))]
pub struct FTestTagColumnB {}
#[repr(C, align(1))]
pub struct FTestTagColumnC {}
#[repr(C, align(1))]
pub struct FTestTagColumnD {}
#[repr(C, align(8))]
pub struct FTEDSProcessorTestsReferenceColumn {}
#[repr(C, align(1))]
pub struct FTEDSProcessorTests_PrimaryTag {}
#[repr(C, align(1))]
pub struct FTEDSProcessorTests_SecondaryTag {}
#[repr(C, align(1))]
pub struct FTEDSProcessorTests_Linked {}
#[repr(C, align(16))]
pub struct FTypedElementLocalTransformColumn {
    pub transform: crate::bindings::core_u_object::FTransform,
}
#[repr(C, align(4))]
pub struct FTypedElementClassTypeInfoColumn {}
#[repr(C, align(4))]
pub struct FTypedElementScriptStructTypeInfoColumn {}
#[repr(C, align(1))]
pub struct FHideRowFromUITag {}
#[repr(C, align(4))]
pub struct FTypedElementU32IntValueCacheColumn {
    pub value: u32,
}
#[repr(C, align(4))]
pub struct FTypedElementI32IntValueCacheColumn {
    pub value: i32,
}
#[repr(C, align(8))]
pub struct FTypedElementU64IntValueCacheColumn {
    pub value: u64,
}
#[repr(C, align(8))]
pub struct FTypedElementI64IntValueCacheColumn {
    pub value: i64,
}
#[repr(C, align(4))]
pub struct FTypedElementFloatValueCacheColumn {
    pub value: f32,
}
#[repr(C, align(1))]
pub struct FTypedElementViewportOutlineColorColumn {
    pub selection_outline_color_index: u8,
}
#[repr(C, align(4))]
pub struct FTypedElementViewportOverlayColorColumn {
    pub overlay_color: crate::bindings::core_u_object::FColor,
}
#[repr(C, align(1))]
pub struct FVisibleInEditorColumn {}
#[repr(C, align(8))]
pub struct FEditorDataStorageUrlColumn {
    pub url_string: FString,
}
#[repr(C, align(8))]
pub struct FEditorDataStorageWebImageColumn {
    pub url_string: FString,
    pub width: u16,
    pub height: u16,
}
#[repr(C, align(8))]
pub struct FWidgetPurposeColumn {}
#[repr(C, align(4))]
pub struct FWidgetPurposeNameColumn {
    pub namespace: FName,
    pub name: FName,
    pub frame: FName,
}
#[repr(C, align(8))]
pub struct FWidgetFactoryColumn {}
#[repr(C, align(4))]
pub struct FWidgetFactoryConstructorTypeInfoColumn {}
#[repr(C, align(4))]
pub struct FWidgetConstructorNameColumn {
    pub name: FName,
}
#[repr(C, align(8))]
pub struct FWidgetFactoryConditionsColumn {}
#[repr(C, align(8))]
pub struct FWidgetFactoryConstructorColumn {}
#[repr(C, align(8))]
pub struct FTedsWidgetConstructorBase {}
#[repr(C, align(8))]
pub struct FTedsDecoratorWidgetConstructor {}
#[repr(C, align(8))]
pub struct FTypedElementWidgetConstructor {}
#[repr(C, align(8))]
pub struct FSimpleWidgetConstructor {}
#[repr(C, align(8))]
pub struct FTest_PingPongPrePhys {
    pub value: u64,
}
#[repr(C, align(8))]
pub struct FTest_PingPongDurPhys {
    pub value: u64,
}
#[repr(C, align(8))]
pub struct FTest_PingPongPostPhys {
    pub value: u64,
}
pub struct UEditorDataStorageFactory {}
pub struct UTypedElementHandleLibrary {}
pub struct UTypedElementListLibrary {}
pub struct UTypedElementCounterInterface {}
pub struct ITypedElementCounterInterface {}
pub struct UTypedElementRegistry {}
pub struct UTestTypedElementInterfaceA {}
pub struct ITestTypedElementInterfaceA {}
pub struct UTestTypedElementInterfaceB {}
pub struct ITestTypedElementInterfaceB {}
pub struct UTestTypedElementInterfaceC {}
pub struct ITestTypedElementInterfaceC {}
pub struct UTestTypedElementInterfaceA_ImplTyped {}
pub struct UTestTypedElementInterfaceA_ImplUntyped {}
pub struct UTestTypedElementInterfaceBAndC_Typed {}
pub struct UTest_PingPongBetweenPhaseFactory {}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ESCCModification(pub i32);
impl ESCCModification {
    pub const CONFLICTED: ESCCModification = ESCCModification(0);
    pub const MODIFIED: ESCCModification = ESCCModification(1);
    pub const ADDED: ESCCModification = ESCCModification(2);
    pub const REMOVED: ESCCModification = ESCCModification(3);
}
