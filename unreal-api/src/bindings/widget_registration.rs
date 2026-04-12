#![allow(clippy::all)]
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
#![allow(forgetting_copy_types)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::new_without_default)]
#![allow(clippy::new_ret_no_self)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[doc(hidden)]
pub static mut __FUNCTION_PTRS: FunctionPtrs = FunctionPtrs::empty();
pub struct FunctionPtrs {}
impl FunctionPtrs {
    pub const fn empty() -> Self {
        Self {}
    }
}
pub fn initialize() {}
#[repr(C, align(16))]
pub struct FToolkitWidgetStyle {
    #[doc(hidden)]
    pub(crate) __padding_16: [u8; 16],
    pub title_background_brush: crate::bindings::slate_core::FSlateBrush,
    pub tool_details_background_brush: crate::bindings::slate_core::FSlateBrush,
    pub title_foreground_color: crate::bindings::slate_core::FSlateColor,
    pub title_padding: crate::bindings::slate_core::FMargin,
    pub active_tool_title_border_padding: crate::bindings::slate_core::FMargin,
    pub tool_context_text_block_padding: crate::bindings::slate_core::FMargin,
    pub title_font: crate::bindings::slate_core::FSlateFontInfo,
}
impl FToolkitWidgetStyle {}
#[repr(C, align(8))]
pub struct UBuilderPersistenceManager {
    __padding_end: [u8; 208],
}
impl UBuilderPersistenceManager {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBuilderPersistenceManager")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBuilderPersistenceManager")
            .copied()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings()
                .core_fns
                .get_cdo_from_class)(class.ptr.cast(), &raw mut cdo);
            cdo
        }
    }
}
