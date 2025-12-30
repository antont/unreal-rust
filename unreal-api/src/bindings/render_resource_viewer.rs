#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::prelude::*;
#[repr(C, align(8))]
pub struct FRenderResourceViewerTreemapFilter {
    pub filter_string: FString,
    pub display_name: FString,
}
pub struct URenderResourceViewerSettings {
    pub filters: TArray<FRenderResourceViewerTreemapFilter>,
}
