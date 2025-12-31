#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(1))]
pub struct FSwarmPingMessage {}
#[repr(C, align(8))]
pub struct FSwarmPongMessage {
    pub b_is_editor: bool,
    pub computer_name: FString,
}
#[repr(C, align(8))]
pub struct FSwarmInfoMessage {
    pub text_message: FString,
}
#[repr(C, align(8))]
pub struct FSwarmAlertMessage {
    pub job_guid: crate::bindings::core_u_object::FGuid,
    pub alert_level: u8,
    pub object_guid: crate::bindings::core_u_object::FGuid,
    pub type_id: i32,
    pub text_message: FString,
}
#[repr(C, align(4))]
pub struct FSwarmTimingMessage {
    pub state: u8,
    pub thread_num: i32,
}
#[repr(C, align(1))]
pub struct FSwarmTaskRequestMessage {}
#[repr(C, align(1))]
pub struct FSwarmTaskRequestReleaseMessage {}
#[repr(C, align(1))]
pub struct FSwarmTaskRequestReservationMessage {}
#[repr(C, align(8))]
pub struct FSwarmTaskRequestSpecificationMessage {
    pub task_guid: crate::bindings::core_u_object::FGuid,
    pub parameters: FString,
    pub flags: u8,
    pub cost: u32,
    pub dependencies: TArray<FString>,
}
#[repr(C, align(8))]
pub struct FSwarmJobStateMessage {
    pub guid: crate::bindings::core_u_object::FGuid,
    pub state: u8,
    pub message: FString,
    pub exit_code: i32,
    pub running_time: f64,
}
#[repr(C, align(8))]
pub struct FSwarmTaskStateMessage {
    pub guid: crate::bindings::core_u_object::FGuid,
    pub state: u8,
    pub message: FString,
    pub exit_code: i32,
    pub running_time: f64,
}
#[repr(C, align(1))]
pub struct FSwarmQuitMessage {}
