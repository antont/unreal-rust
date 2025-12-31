#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(8))]
pub struct FGeometryCollectionTransferVertexScalarAttributeNode {
    pub collection: crate::bindings::chaos::FManagedArrayCollection,
    pub from_collection: crate::bindings::chaos::FManagedArrayCollection,
    pub attribute_key: crate::bindings::dataflow_engine_plugin::FCollectionAttributeKey,
    pub bounding_volume_type: EDataflowTransferNodeBoundingVolume,
    pub sample_scale: EDataflowTransferNodeSampleScale,
    pub falloff: EDataflowTransferNodeFalloff,
    pub falloff_threshold: f32,
    pub edge_multiplier: f32,
    pub bound_multiplier: f32,
}
#[repr(C, align(8))]
pub struct FSetVertexColorInCollectionFromFloatArrayDataflowNode {
    pub collection: crate::bindings::chaos::FManagedArrayCollection,
    pub float_array: TArray<f32>,
    pub scale: f32,
}
#[repr(C, align(8))]
pub struct FSetVertexColorInCollectionFromVertexSelectionDataflowNode {
    pub collection: crate::bindings::chaos::FManagedArrayCollection,
    pub vertex_selection: crate::bindings::dataflow_core::FDataflowVertexSelection,
    pub selected_color: crate::bindings::core_u_object::FLinearColor,
    pub non_selected_color: crate::bindings::core_u_object::FLinearColor,
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EDataflowTransferNodeBoundingVolume(pub u8);
impl EDataflowTransferNodeBoundingVolume {
    pub const DATAFLOW_TRANSFER_VERTEX: EDataflowTransferNodeBoundingVolume = EDataflowTransferNodeBoundingVolume(
        0,
    );
    pub const DATAFLOW_TRANSFER_TRIANGLE: EDataflowTransferNodeBoundingVolume = EDataflowTransferNodeBoundingVolume(
        1,
    );
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EDataflowTransferNodeSampleScale(pub u8);
impl EDataflowTransferNodeSampleScale {
    pub const DATAFLOW_TRANSFER_COMPONENT_EDGE: EDataflowTransferNodeSampleScale = EDataflowTransferNodeSampleScale(
        0,
    );
    pub const DATAFLOW_TRANSFER_ASSET_EDGE: EDataflowTransferNodeSampleScale = EDataflowTransferNodeSampleScale(
        1,
    );
    pub const DATAFLOW_TRANSFER_ASSET_BOUND: EDataflowTransferNodeSampleScale = EDataflowTransferNodeSampleScale(
        2,
    );
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EDataflowTransferNodeFalloff(pub u8);
impl EDataflowTransferNodeFalloff {
    pub const DATAFLOW_TRANSFER_SQUARED: EDataflowTransferNodeFalloff = EDataflowTransferNodeFalloff(
        0,
    );
    pub const DATAFLOW_TRANSFER_LINEAR: EDataflowTransferNodeFalloff = EDataflowTransferNodeFalloff(
        1,
    );
    pub const DATAFLOW_TRANSFER_NONE: EDataflowTransferNodeFalloff = EDataflowTransferNodeFalloff(
        2,
    );
}
