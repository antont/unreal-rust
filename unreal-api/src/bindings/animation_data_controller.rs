#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
pub struct UAnimDataController {
    pub model: TWeakObjectPtr<crate::bindings::engine::UAnimDataModel>,
    pub model_interface: TScriptInterface<crate::bindings::engine::IAnimationDataModel>,
}
