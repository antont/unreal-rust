#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(8))]
pub struct FEditableToolPaletteSettings {
    pub palette_command_names: TArray<FString>,
}
#[repr(C, align(16))]
pub struct FToolkitWidgetStyle {
    pub title_background_brush: crate::bindings::slate_core::FSlateBrush,
    pub tool_details_background_brush: crate::bindings::slate_core::FSlateBrush,
    pub title_foreground_color: crate::bindings::slate_core::FSlateColor,
    pub title_padding: crate::bindings::slate_core::FMargin,
    pub active_tool_title_border_padding: crate::bindings::slate_core::FMargin,
    pub tool_context_text_block_padding: crate::bindings::slate_core::FMargin,
    pub title_font: crate::bindings::slate_core::FSlateFontInfo,
}
#[repr(C, align(8))]
pub struct FPersistedNameArray {
    pub array_of_names_to_persist: TArray<FName>,
}
#[repr(C, align(1))]
pub struct FPersistedBool {
    pub persisted_bool: bool,
}
pub struct UBuilderPersistenceManager {
    pub saved_name_to_persisted_f_name_array_map: TMap<FString, FPersistedNameArray>,
    pub saved_name_to_persisted_bool_map: TMap<FString, FPersistedBool>,
}
