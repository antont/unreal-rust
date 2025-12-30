#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::prelude::*;
#[repr(C, align(4))]
pub struct FAssetPathColumn_Experimental {
    pub path: FName,
}
#[repr(C, align(1))]
pub struct FFolderTypeColumn_Experimental {}
#[repr(C, align(8))]
pub struct FAssetDataColumn_Experimental {
    pub asset_data: FAssetData,
}
#[repr(C, align(1))]
pub struct FAssetTag {}
#[repr(C, align(1))]
pub struct FPrivateAssetTag {}
#[repr(C, align(1))]
pub struct FEpicInternalAssetTag {}
#[repr(C, align(1))]
pub struct FPublicAssetTag {}
#[repr(C, align(4))]
pub struct FAssetNameColumn {
    pub name: FName,
}
#[repr(C, align(4))]
pub struct FAssetClassColumn {
    pub class_path: FTopLevelAssetPath,
}
#[repr(C, align(8))]
pub struct FDiskSizeColumn {
    pub disk_size: i64,
}
#[repr(C, align(8))]
pub struct FVersePathColumn {}
#[repr(C, align(1))]
pub struct FUpdatedPathTag {}
#[repr(C, align(1))]
pub struct FUpdatedAssetDataTag {}
#[repr(C, align(4))]
pub struct FVirtualPathColumn_Experimental {
    pub virtual_path: FName,
}
#[repr(C, align(8))]
pub struct FItemAttributeBaseColumn_Experimental {}
#[repr(C, align(8))]
pub struct FItemTextAttributeColumn_Experimental {
    pub value: FText,
}
#[repr(C, align(8))]
pub struct FItemStringAttributeColumn_Experimental {
    pub value: FString,
}
#[repr(C, align(1))]
pub struct FThumbnailSizeColumn_Experimental {}
#[repr(C, align(4))]
pub struct FSizeValueColumn_Experimental {
    pub size_value: f32,
}
#[repr(C, align(4))]
pub struct FWidgetPaddingColumn_Experimental {
    pub padding: FMargin,
}
#[repr(C, align(1))]
pub struct FThumbnailEditModeColumn_Experimental {
    pub is_edit_mode_toggled: bool,
}
#[repr(C, align(1))]
pub struct FTextOverflowPolicyColumn_Experimental {}
#[repr(C, align(1))]
pub struct FWidgetVisibilityColumn_Experimental {}
#[repr(C, align(8))]
pub struct FFontStyleColumn_Experimental {}
#[repr(C, align(8))]
pub struct FLocalWidgetTooltipColumn_Experimental {}
#[repr(C, align(8))]
pub struct FOnGetWidgetSlateBrushColumn_Experimental {}
#[repr(C, align(8))]
pub struct FOnGetWidgetColorAndOpacityColumn_Experimental {}
#[repr(C, align(8))]
pub struct FExternalWidgetOnClickedColumn_Experimental {}
#[repr(C, align(8))]
pub struct FDiskSizeWidgetConstructor {}
#[repr(C, align(8))]
pub struct FAssetDataItemTypeWidgetConstructor {}
#[repr(C, align(8))]
pub struct FAssetDataLabelWidgetConstructor {}
#[repr(C, align(8))]
pub struct FAssetThumbnailWidgetConstructor {}
#[repr(C, align(8))]
pub struct FAssetDataVirtualPathWidgetConstructor {}
#[repr(C, align(8))]
pub struct FAssetLabelWidgetConstructor {}
#[repr(C, align(8))]
pub struct FAssetNameWidgetConstructor {}
#[repr(C, align(8))]
pub struct FAssetPreviewAdvancedInfoWidgetConstructor {}
#[repr(C, align(8))]
pub struct FAssetPreviewBaseInfoWidgetConstructor {}
#[repr(C, align(8))]
pub struct FAssetPreviewHeaderWidgetConstructor {}
#[repr(C, align(8))]
pub struct FEditModeToggleHeaderWidgetConstructor {}
#[repr(C, align(8))]
pub struct FItemContextMenuWidgetConstructor {}
#[repr(C, align(8))]
pub struct FAssetPreviewThumbnailWidgetConstructor {}
#[repr(C, align(8))]
pub struct FContentBrowserListViewNameWidgetConstructor {}
#[repr(C, align(8))]
pub struct FDynamicAssetDataColumnBaseWidgetConstructor {}
#[repr(C, align(8))]
pub struct FStaticMeshTrianglesWidgetConstructor {}
pub struct UTedsAssetDataFactory {}
pub struct UDiskSizeWidgetFactory {}
pub struct UAssetDataItemTypeWidgetFactory {}
pub struct UAssetDataLabelWidgetFactory {}
pub struct UAssetThumbnailWidgetFactory {}
pub struct UAssetDataVirtualPathWidgetFactory {}
pub struct UAssetLabelWidgetFactory {}
pub struct UAssetNameWidgetFactory {}
pub struct UAssetPreviewGeneralWidgetRegistrationFactory {}
pub struct UAssetPreviewAdvancedInfoWidgetFactory {}
pub struct UAssetPreviewBaseInfoWidgetFactory {}
pub struct UAssetPreviewHeaderWidgetFactory {}
pub struct UEditModeToggleHeaderWidgetFactory {}
pub struct UItemContextMenuWidgetFactory {}
pub struct UAssetPreviewThumbnailWidgetFactory {}
pub struct UContentBrowserListViewNameWidgetFactory {}
pub struct UDynamicAssetDataColumnBaseWidgetFactory {}
pub struct UStaticMeshTrianglesWidgetFactory {}
