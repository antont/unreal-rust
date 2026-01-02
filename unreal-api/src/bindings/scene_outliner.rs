#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(8))]
pub struct USceneOutlinerMenuContext {
    __padding_end: [u8; 88],
}
impl USceneOutlinerMenuContext {}
#[repr(C, align(8))]
pub struct UActorBrowserConfig {
    __padding_end: [u8; 128],
}
impl UActorBrowserConfig {}
#[repr(C, align(8))]
pub struct UOutlinerConfig {
    __padding_end: [u8; 128],
}
impl UOutlinerConfig {}
#[repr(transparent)]
pub struct EActorBrowsingFolderDoubleClickMethod(pub u8);
impl EActorBrowsingFolderDoubleClickMethod {
    pub const TOGGLE_EXPANSION: EActorBrowsingFolderDoubleClickMethod = EActorBrowsingFolderDoubleClickMethod(
        0,
    );
    pub const TOGGLE_CURRENT_FOLDER: EActorBrowsingFolderDoubleClickMethod = EActorBrowsingFolderDoubleClickMethod(
        1,
    );
}
