#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::prelude::*;
#[repr(C, align(8))]
pub struct FGeometryCollectionTransferVertexScalarAttributeNode {
    pub collection: FManagedArrayCollection,
    pub from_collection: FManagedArrayCollection,
    pub attribute_key: FCollectionAttributeKey,
    pub bounding_volume_type: EDataflowTransferNodeBoundingVolume,
    pub sample_scale: EDataflowTransferNodeSampleScale,
    pub falloff: EDataflowTransferNodeFalloff,
    pub falloff_threshold: f32,
    pub edge_multiplier: f32,
    pub bound_multiplier: f32,
}
#[repr(C, align(8))]
pub struct FSetVertexColorInCollectionFromFloatArrayDataflowNode {
    pub collection: FManagedArrayCollection,
    pub float_array: TArray<f32>,
    pub scale: f32,
}
#[repr(C, align(8))]
pub struct FSetVertexColorInCollectionFromVertexSelectionDataflowNode {
    pub collection: FManagedArrayCollection,
    pub vertex_selection: FDataflowVertexSelection,
    pub selected_color: FLinearColor,
    pub non_selected_color: FLinearColor,
}
