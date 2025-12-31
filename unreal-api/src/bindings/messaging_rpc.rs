#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(4))]
pub struct FMessageRpcCancel {
    pub call_id: crate::bindings::core_u_object::FGuid,
}
#[repr(C, align(8))]
pub struct FMessageRpcProgress {
    pub completion: f32,
    pub call_id: crate::bindings::core_u_object::FGuid,
    pub status_text: FString,
}
#[repr(C, align(4))]
pub struct FMessageRpcUnhandled {
    pub call_id: crate::bindings::core_u_object::FGuid,
}
#[repr(C, align(4))]
pub struct FRpcMessage {
    pub call_id: crate::bindings::core_u_object::FGuid,
}
