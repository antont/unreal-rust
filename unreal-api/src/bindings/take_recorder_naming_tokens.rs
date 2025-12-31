#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
pub struct UTakeRecorderNamingTokens {
    pub take_meta_data: TWeakObjectPtr<crate::bindings::takes_core::UTakeMetaData>,
    pub context: UPtr<crate::bindings::takes_core::UTakeRecorderNamingTokensContext>,
}
