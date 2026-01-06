#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
#![allow(clippy::non_camel_case_types)]
#![allow(clippy::new_without_default)]
#![allow(clippy::new_ret_no_self)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
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
