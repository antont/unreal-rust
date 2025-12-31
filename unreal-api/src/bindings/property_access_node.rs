#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
pub struct UK2Node_PropertyAccess {
    pub path: TArray<FString>,
    pub text_path: FText,
    pub resolved_pin_type: crate::bindings::engine::FEdGraphPinType,
    pub generated_property_name: FName,
    pub context_id: FName,
}
