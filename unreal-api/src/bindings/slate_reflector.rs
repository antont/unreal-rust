#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(4))]
pub struct FWidgetSnapshotRequest {
    pub target_instance_id: crate::bindings::core_u_object::FGuid,
    pub snapshot_request_id: crate::bindings::core_u_object::FGuid,
}
#[repr(C, align(8))]
pub struct FWidgetSnapshotResponse {
    pub snapshot_request_id: crate::bindings::core_u_object::FGuid,
    pub snapshot_data: TArray<u8>,
}
