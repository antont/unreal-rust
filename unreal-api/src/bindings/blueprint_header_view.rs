#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
#![allow(clippy::non_camel_case_types)]
#![allow(clippy::new_without_default)]
#![allow(clippy::new_ret_no_self)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
pub fn initialize() {}
#[repr(C, align(8))]
pub struct UBlueprintHeaderViewSettings {
    __padding_end: [u8; 224],
}
impl UBlueprintHeaderViewSettings {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBlueprintHeaderViewSettings")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(transparent)]
pub struct EHeaderViewSortMethod(pub u8);
impl EHeaderViewSortMethod {
    pub const NONE: EHeaderViewSortMethod = EHeaderViewSortMethod(0);
    pub const SORT_BY_ACCESS_SPECIFIER: EHeaderViewSortMethod = EHeaderViewSortMethod(1);
    pub const SORT_FOR_OPTIMAL_PADDING: EHeaderViewSortMethod = EHeaderViewSortMethod(2);
}
