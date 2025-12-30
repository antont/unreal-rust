#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::prelude::*;
#[repr(C, align(8))]
pub struct FEditableToolPaletteSettings {
    pub palette_command_names: TArray<FString>,
}
#[repr(C, align(16))]
pub struct FToolkitWidgetStyle {
    pub title_background_brush: FSlateBrush,
    pub tool_details_background_brush: FSlateBrush,
    pub title_foreground_color: FSlateColor,
    pub title_padding: FMargin,
    pub active_tool_title_border_padding: FMargin,
    pub tool_context_text_block_padding: FMargin,
    pub title_font: FSlateFontInfo,
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
