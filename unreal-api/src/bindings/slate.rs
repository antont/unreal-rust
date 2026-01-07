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
pub struct FInputChord {
    pub key: crate::bindings::input_core::FKey,
    pub flags_32: u8,
}
impl FInputChord {}
#[repr(C, align(8))]
pub struct FAnchors {
    pub minimum: crate::bindings::core_u_object::FVector2D,
    pub maximum: crate::bindings::core_u_object::FVector2D,
}
impl FAnchors {}
#[repr(C, align(8))]
pub struct UToolMenuBase {
    __padding_end: [u8; 48],
}
impl UToolMenuBase {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UToolMenuBase")
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
#[repr(C, align(8))]
pub struct USlateSettings {
    __padding_end: [u8; 56],
}
impl USlateSettings {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("USlateSettings")
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
#[repr(C, align(16))]
pub struct UButtonWidgetStyle {
    #[doc(hidden)]
    __padding_64: [u8; 64],
    pub button_style: crate::bindings::slate_core::FButtonStyle,
}
impl UButtonWidgetStyle {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UButtonWidgetStyle")
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
#[repr(C, align(16))]
pub struct UCheckBoxWidgetStyle {
    __padding_end: [u8; 2896],
}
impl UCheckBoxWidgetStyle {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UCheckBoxWidgetStyle")
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
#[repr(C, align(16))]
pub struct UComboBoxWidgetStyle {
    __padding_end: [u8; 1824],
}
impl UComboBoxWidgetStyle {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UComboBoxWidgetStyle")
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
#[repr(C, align(16))]
pub struct UComboButtonWidgetStyle {
    __padding_end: [u8; 1680],
}
impl UComboButtonWidgetStyle {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UComboButtonWidgetStyle")
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
#[repr(C, align(16))]
pub struct UEditableTextBoxWidgetStyle {
    __padding_end: [u8; 3904],
}
impl UEditableTextBoxWidgetStyle {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEditableTextBoxWidgetStyle")
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
#[repr(C, align(16))]
pub struct UEditableTextWidgetStyle {
    __padding_end: [u8; 832],
}
impl UEditableTextWidgetStyle {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEditableTextWidgetStyle")
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
#[repr(C, align(16))]
pub struct UProgressWidgetStyle {
    #[doc(hidden)]
    __padding_64: [u8; 64],
    pub progress_bar_style: crate::bindings::slate_core::FProgressBarStyle,
}
impl UProgressWidgetStyle {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UProgressWidgetStyle")
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
#[repr(C, align(16))]
pub struct UScrollBarWidgetStyle {
    __padding_end: [u8; 1968],
}
impl UScrollBarWidgetStyle {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UScrollBarWidgetStyle")
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
#[repr(C, align(16))]
pub struct UScrollBoxWidgetStyle {
    __padding_end: [u8; 944],
}
impl UScrollBoxWidgetStyle {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UScrollBoxWidgetStyle")
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
#[repr(C, align(16))]
pub struct USpinBoxWidgetStyle {
    __padding_end: [u8; 1600],
}
impl USpinBoxWidgetStyle {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("USpinBoxWidgetStyle")
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
#[repr(C, align(16))]
pub struct UTextBlockWidgetStyle {
    __padding_end: [u8; 912],
}
impl UTextBlockWidgetStyle {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UTextBlockWidgetStyle")
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
pub struct ETextFlowDirection(pub u8);
impl ETextFlowDirection {
    pub const AUTO: ETextFlowDirection = ETextFlowDirection(0);
    pub const LEFT_TO_RIGHT: ETextFlowDirection = ETextFlowDirection(1);
    pub const RIGHT_TO_LEFT: ETextFlowDirection = ETextFlowDirection(2);
    pub const CULTURE: ETextFlowDirection = ETextFlowDirection(3);
}
#[repr(transparent)]
pub struct ECustomizedToolMenuVisibility(pub i32);
impl ECustomizedToolMenuVisibility {
    pub const NONE: ECustomizedToolMenuVisibility = ECustomizedToolMenuVisibility(0);
    pub const VISIBLE: ECustomizedToolMenuVisibility = ECustomizedToolMenuVisibility(1);
    pub const HIDDEN: ECustomizedToolMenuVisibility = ECustomizedToolMenuVisibility(2);
}
#[repr(transparent)]
pub struct EInputPreProcessorType(pub u8);
impl EInputPreProcessorType {
    pub const OVERLAY: EInputPreProcessorType = EInputPreProcessorType(0);
    pub const PRE_ENGINE: EInputPreProcessorType = EInputPreProcessorType(1);
    pub const ENGINE: EInputPreProcessorType = EInputPreProcessorType(2);
    pub const PRE_EDITOR: EInputPreProcessorType = EInputPreProcessorType(3);
    pub const EDITOR: EInputPreProcessorType = EInputPreProcessorType(4);
    pub const PRE_GAME: EInputPreProcessorType = EInputPreProcessorType(5);
    pub const GAME: EInputPreProcessorType = EInputPreProcessorType(6);
    pub const COUNT: EInputPreProcessorType = EInputPreProcessorType(7);
}
#[repr(transparent)]
pub struct EMultiBlockType(pub u8);
impl EMultiBlockType {
    pub const NONE: EMultiBlockType = EMultiBlockType(0);
    pub const BUTTON_ROW: EMultiBlockType = EMultiBlockType(1);
    pub const EDITABLE_TEXT: EMultiBlockType = EMultiBlockType(2);
    pub const HEADING: EMultiBlockType = EMultiBlockType(3);
    pub const MENU_ENTRY: EMultiBlockType = EMultiBlockType(4);
    pub const SEPARATOR: EMultiBlockType = EMultiBlockType(5);
    pub const TOOL_BAR_BUTTON: EMultiBlockType = EMultiBlockType(6);
    pub const TOOL_BAR_COMBO_BUTTON: EMultiBlockType = EMultiBlockType(7);
    pub const WIDGET: EMultiBlockType = EMultiBlockType(8);
}
#[repr(transparent)]
pub struct EUserInterfaceActionType(pub u8);
impl EUserInterfaceActionType {
    pub const NONE: EUserInterfaceActionType = EUserInterfaceActionType(0);
    pub const BUTTON: EUserInterfaceActionType = EUserInterfaceActionType(1);
    pub const TOGGLE_BUTTON: EUserInterfaceActionType = EUserInterfaceActionType(2);
    pub const RADIO_BUTTON: EUserInterfaceActionType = EUserInterfaceActionType(3);
    pub const CHECK: EUserInterfaceActionType = EUserInterfaceActionType(4);
    pub const COLLAPSED_BUTTON: EUserInterfaceActionType = EUserInterfaceActionType(5);
}
#[repr(transparent)]
pub struct ETextJustify(pub u8);
impl ETextJustify {
    pub const LEFT: ETextJustify = ETextJustify(0);
    pub const CENTER: ETextJustify = ETextJustify(1);
    pub const RIGHT: ETextJustify = ETextJustify(2);
    pub const INVARIANT_LEFT: ETextJustify = ETextJustify(3);
    pub const INVARIANT_RIGHT: ETextJustify = ETextJustify(4);
}
#[repr(transparent)]
pub struct EMultiBoxType(pub u8);
impl EMultiBoxType {
    pub const MENU_BAR: EMultiBoxType = EMultiBoxType(0);
    pub const TOOL_BAR: EMultiBoxType = EMultiBoxType(1);
    pub const VERTICAL_TOOL_BAR: EMultiBoxType = EMultiBoxType(2);
    pub const SLIM_HORIZONTAL_TOOL_BAR: EMultiBoxType = EMultiBoxType(3);
    pub const UNIFORM_TOOL_BAR: EMultiBoxType = EMultiBoxType(4);
    pub const MENU: EMultiBoxType = EMultiBoxType(5);
    pub const BUTTON_ROW: EMultiBoxType = EMultiBoxType(6);
    pub const SLIM_HORIZONTAL_UNIFORM_TOOL_BAR: EMultiBoxType = EMultiBoxType(7);
    pub const SLIM_WRAPPING_TOOL_BAR: EMultiBoxType = EMultiBoxType(8);
}
#[repr(transparent)]
pub struct EScrollIntoViewAlignment(pub u8);
impl EScrollIntoViewAlignment {
    pub const INTO_VIEW: EScrollIntoViewAlignment = EScrollIntoViewAlignment(0);
    pub const TOP_OR_LEFT: EScrollIntoViewAlignment = EScrollIntoViewAlignment(1);
    pub const CENTER_ALIGNED: EScrollIntoViewAlignment = EScrollIntoViewAlignment(2);
    pub const BOTTOM_OR_RIGHT: EScrollIntoViewAlignment = EScrollIntoViewAlignment(3);
}
#[repr(transparent)]
pub struct ESelectionMode(pub u8);
impl ESelectionMode {
    pub const NONE: ESelectionMode = ESelectionMode(0);
    pub const SINGLE: ESelectionMode = ESelectionMode(1);
    pub const SINGLE_TOGGLE: ESelectionMode = ESelectionMode(2);
    pub const MULTI: ESelectionMode = ESelectionMode(3);
}
#[repr(transparent)]
pub struct EDescendantScrollDestination(pub u8);
impl EDescendantScrollDestination {
    pub const INTO_VIEW: EDescendantScrollDestination = EDescendantScrollDestination(0);
    pub const TOP_OR_LEFT: EDescendantScrollDestination = EDescendantScrollDestination(
        1,
    );
    pub const CENTER: EDescendantScrollDestination = EDescendantScrollDestination(2);
    pub const BOTTOM_OR_RIGHT: EDescendantScrollDestination = EDescendantScrollDestination(
        3,
    );
}
#[repr(transparent)]
pub struct EScrollWhenFocusChanges(pub u8);
impl EScrollWhenFocusChanges {
    pub const NO_SCROLL: EScrollWhenFocusChanges = EScrollWhenFocusChanges(0);
    pub const INSTANT_SCROLL: EScrollWhenFocusChanges = EScrollWhenFocusChanges(1);
    pub const ANIMATED_SCROLL: EScrollWhenFocusChanges = EScrollWhenFocusChanges(2);
}
#[repr(transparent)]
pub struct EStretch(pub u8);
impl EStretch {
    pub const NONE: EStretch = EStretch(0);
    pub const FILL: EStretch = EStretch(1);
    pub const SCALE_TO_FIT: EStretch = EStretch(2);
    pub const SCALE_TO_FIT_X: EStretch = EStretch(3);
    pub const SCALE_TO_FIT_Y: EStretch = EStretch(4);
    pub const SCALE_TO_FILL: EStretch = EStretch(5);
    pub const SCALE_BY_SAFE_ZONE: EStretch = EStretch(6);
    pub const USER_SPECIFIED: EStretch = EStretch(7);
    pub const USER_SPECIFIED_WITH_CLIPPING: EStretch = EStretch(8);
}
#[repr(transparent)]
pub struct EStretchDirection(pub u8);
impl EStretchDirection {
    pub const BOTH: EStretchDirection = EStretchDirection(0);
    pub const DOWN_ONLY: EStretchDirection = EStretchDirection(1);
    pub const UP_ONLY: EStretchDirection = EStretchDirection(2);
}
#[repr(transparent)]
pub struct EVirtualKeyboardTrigger(pub u8);
impl EVirtualKeyboardTrigger {
    pub const ON_FOCUS_BY_POINTER: EVirtualKeyboardTrigger = EVirtualKeyboardTrigger(0);
    pub const ON_ALL_FOCUS_EVENTS: EVirtualKeyboardTrigger = EVirtualKeyboardTrigger(1);
}
#[repr(transparent)]
pub struct EVirtualKeyboardDismissAction(pub u8);
impl EVirtualKeyboardDismissAction {
    pub const TEXT_CHANGE_ON_DISMISS: EVirtualKeyboardDismissAction = EVirtualKeyboardDismissAction(
        0,
    );
    pub const TEXT_COMMIT_ON_ACCEPT: EVirtualKeyboardDismissAction = EVirtualKeyboardDismissAction(
        1,
    );
    pub const TEXT_COMMIT_ON_DISMISS: EVirtualKeyboardDismissAction = EVirtualKeyboardDismissAction(
        2,
    );
}
#[repr(transparent)]
pub struct ETextWrappingPolicy(pub u8);
impl ETextWrappingPolicy {
    pub const DEFAULT_WRAPPING: ETextWrappingPolicy = ETextWrappingPolicy(0);
    pub const ALLOW_PER_CHARACTER_WRAPPING: ETextWrappingPolicy = ETextWrappingPolicy(1);
}
#[repr(transparent)]
pub struct EProgressBarFillType(pub u8);
impl EProgressBarFillType {
    pub const LEFT_TO_RIGHT: EProgressBarFillType = EProgressBarFillType(0);
    pub const RIGHT_TO_LEFT: EProgressBarFillType = EProgressBarFillType(1);
    pub const FILL_FROM_CENTER: EProgressBarFillType = EProgressBarFillType(2);
    pub const FILL_FROM_CENTER_HORIZONTAL: EProgressBarFillType = EProgressBarFillType(
        3,
    );
    pub const FILL_FROM_CENTER_VERTICAL: EProgressBarFillType = EProgressBarFillType(4);
    pub const TOP_TO_BOTTOM: EProgressBarFillType = EProgressBarFillType(5);
    pub const BOTTOM_TO_TOP: EProgressBarFillType = EProgressBarFillType(6);
}
#[repr(transparent)]
pub struct EProgressBarFillStyle(pub u8);
impl EProgressBarFillStyle {
    pub const MASK: EProgressBarFillStyle = EProgressBarFillStyle(0);
    pub const SCALE: EProgressBarFillStyle = EProgressBarFillStyle(1);
}
#[repr(transparent)]
pub struct EListItemAlignment(pub u8);
impl EListItemAlignment {
    pub const EVENLY_DISTRIBUTED: EListItemAlignment = EListItemAlignment(0);
    pub const EVENLY_SIZE: EListItemAlignment = EListItemAlignment(1);
    pub const EVENLY_WIDE: EListItemAlignment = EListItemAlignment(2);
    pub const LEFT_ALIGNED: EListItemAlignment = EListItemAlignment(3);
    pub const RIGHT_ALIGNED: EListItemAlignment = EListItemAlignment(4);
    pub const CENTER_ALIGNED: EListItemAlignment = EListItemAlignment(5);
    pub const FILL: EListItemAlignment = EListItemAlignment(6);
}
