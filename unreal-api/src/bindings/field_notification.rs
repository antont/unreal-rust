#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::prelude::*;
#[repr(C, align(4))]
pub struct FFieldNotificationId {
    pub field_name: FName,
}
pub struct UNotifyFieldValueChanged {}
pub struct INotifyFieldValueChanged {}
