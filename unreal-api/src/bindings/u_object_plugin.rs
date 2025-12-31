#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(8))]
pub struct FMyPluginStruct {
    pub test_string: FString,
}
pub struct UMyPluginObject {
    pub my_struct: FMyPluginStruct,
}
