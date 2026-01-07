#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
#![allow(clippy::non_camel_case_types)]
#![allow(clippy::new_without_default)]
#![allow(clippy::new_ret_no_self)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[doc(hidden)]
pub static mut U_RADIAL_SLIDER_SET_VALUE_TAGS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RADIAL_SLIDER_SET_VALUE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RADIAL_SLIDER_SET_USE_VERTICAL_DRAG: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RADIAL_SLIDER_SET_STEP_SIZE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RADIAL_SLIDER_SET_SLIDER_RANGE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RADIAL_SLIDER_SET_SLIDER_PROGRESS_COLOR: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RADIAL_SLIDER_SET_SLIDER_HANDLE_START_ANGLE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RADIAL_SLIDER_SET_SLIDER_HANDLE_END_ANGLE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RADIAL_SLIDER_SET_SLIDER_HANDLE_COLOR: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RADIAL_SLIDER_SET_SLIDER_BAR_COLOR: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RADIAL_SLIDER_SET_SHOW_SLIDER_HANDLE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RADIAL_SLIDER_SET_SHOW_SLIDER_HAND: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RADIAL_SLIDER_SET_LOCKED: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RADIAL_SLIDER_SET_HAND_START_END_RATIO: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RADIAL_SLIDER_SET_CUSTOM_DEFAULT_VALUE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RADIAL_SLIDER_SET_CENTER_BACKGROUND_COLOR: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RADIAL_SLIDER_SET_ANGULAR_OFFSET: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RADIAL_SLIDER_GET_VALUE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RADIAL_SLIDER_GET_NORMALIZED_SLIDER_HANDLE_POSITION: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RADIAL_SLIDER_GET_CUSTOM_DEFAULT_VALUE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
pub fn initialize() {
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = URadialSlider::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetValueTags"),
            &raw mut U_RADIAL_SLIDER_SET_VALUE_TAGS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetValue"),
            &raw mut U_RADIAL_SLIDER_SET_VALUE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetUseVerticalDrag"),
            &raw mut U_RADIAL_SLIDER_SET_USE_VERTICAL_DRAG,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetStepSize"),
            &raw mut U_RADIAL_SLIDER_SET_STEP_SIZE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetSliderRange"),
            &raw mut U_RADIAL_SLIDER_SET_SLIDER_RANGE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetSliderProgressColor"),
            &raw mut U_RADIAL_SLIDER_SET_SLIDER_PROGRESS_COLOR,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetSliderHandleStartAngle"),
            &raw mut U_RADIAL_SLIDER_SET_SLIDER_HANDLE_START_ANGLE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetSliderHandleEndAngle"),
            &raw mut U_RADIAL_SLIDER_SET_SLIDER_HANDLE_END_ANGLE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetSliderHandleColor"),
            &raw mut U_RADIAL_SLIDER_SET_SLIDER_HANDLE_COLOR,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetSliderBarColor"),
            &raw mut U_RADIAL_SLIDER_SET_SLIDER_BAR_COLOR,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetShowSliderHandle"),
            &raw mut U_RADIAL_SLIDER_SET_SHOW_SLIDER_HANDLE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetShowSliderHand"),
            &raw mut U_RADIAL_SLIDER_SET_SHOW_SLIDER_HAND,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetLocked"),
            &raw mut U_RADIAL_SLIDER_SET_LOCKED,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetHandStartEndRatio"),
            &raw mut U_RADIAL_SLIDER_SET_HAND_START_END_RATIO,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetCustomDefaultValue"),
            &raw mut U_RADIAL_SLIDER_SET_CUSTOM_DEFAULT_VALUE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetCenterBackgroundColor"),
            &raw mut U_RADIAL_SLIDER_SET_CENTER_BACKGROUND_COLOR,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetAngularOffset"),
            &raw mut U_RADIAL_SLIDER_SET_ANGULAR_OFFSET,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetValue"),
            &raw mut U_RADIAL_SLIDER_GET_VALUE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetNormalizedSliderHandlePosition"),
            &raw mut U_RADIAL_SLIDER_GET_NORMALIZED_SLIDER_HANDLE_POSITION,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetCustomDefaultValue"),
            &raw mut U_RADIAL_SLIDER_GET_CUSTOM_DEFAULT_VALUE,
        );
    }
}
#[repr(C, align(16))]
pub struct FColorGradingSpinBoxStyle {
    #[doc(hidden)]
    __padding_16: [u8; 16],
    pub border_brush: crate::bindings::slate_core::FSlateBrush,
    pub active_border_brush: crate::bindings::slate_core::FSlateBrush,
    pub hovered_border_brush: crate::bindings::slate_core::FSlateBrush,
    pub selector_brush: crate::bindings::slate_core::FSlateBrush,
    pub selector_width: f32,
}
impl FColorGradingSpinBoxStyle {}
#[repr(C, align(16))]
pub struct URadialSlider {
    #[doc(hidden)]
    __padding_696: [u8; 696],
    pub value: f32,
    #[doc(hidden)]
    __padding_744: [u8; 40],
    pub slider_range: crate::bindings::engine::FRuntimeFloatCurve,
    pub value_tags: TArray<f32>,
    pub slider_handle_start_angle: f32,
    pub slider_handle_end_angle: f32,
    pub angular_offset: f32,
    pub hand_start_end_ratio: crate::bindings::core_u_object::FVector2D,
    pub widget_style: crate::bindings::slate_core::FSliderStyle,
    pub slider_bar_color: crate::bindings::core_u_object::FLinearColor,
    pub slider_progress_color: crate::bindings::core_u_object::FLinearColor,
    pub slider_handle_color: crate::bindings::core_u_object::FLinearColor,
    pub center_background_color: crate::bindings::core_u_object::FLinearColor,
    pub locked: bool,
    pub mouse_uses_step: bool,
    pub requires_controller_lock: bool,
    pub step_size: f32,
    pub is_focusable: bool,
    pub use_vertical_drag: bool,
    pub show_slider_handle: bool,
    pub show_slider_hand: bool,
    __padding_end: [u8; 148],
}
impl URadialSlider {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("URadialSlider")
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
    pub fn set_value_tags(&mut self, in_value_tags: &TArray<f32>) {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::advanced_widgets::U_RADIAL_SLIDER_SET_VALUE_TAGS,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_value_tags,
                __buffer.add(0).cast::<TArray<f32>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::advanced_widgets::U_RADIAL_SLIDER_SET_VALUE_TAGS,
                __buffer,
            )
        };
    }
    pub fn set_value(&mut self, in_value: f32) {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::advanced_widgets::U_RADIAL_SLIDER_SET_VALUE,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&in_value, __buffer.add(0).cast::<f32>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::advanced_widgets::U_RADIAL_SLIDER_SET_VALUE,
                __buffer,
            )
        };
    }
    pub fn set_use_vertical_drag(&mut self, in_use_vertical_drag: bool) {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::advanced_widgets::U_RADIAL_SLIDER_SET_USE_VERTICAL_DRAG,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_use_vertical_drag,
                __buffer.add(0).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::advanced_widgets::U_RADIAL_SLIDER_SET_USE_VERTICAL_DRAG,
                __buffer,
            )
        };
    }
    pub fn set_step_size(&mut self, in_value: f32) {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::advanced_widgets::U_RADIAL_SLIDER_SET_STEP_SIZE,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&in_value, __buffer.add(0).cast::<f32>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::advanced_widgets::U_RADIAL_SLIDER_SET_STEP_SIZE,
                __buffer,
            )
        };
    }
    pub fn set_slider_range(
        &mut self,
        in_slider_range: &crate::bindings::engine::FRuntimeFloatCurve,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<136>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::advanced_widgets::U_RADIAL_SLIDER_SET_SLIDER_RANGE,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_slider_range,
                __buffer.add(0).cast::<crate::bindings::engine::FRuntimeFloatCurve>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::advanced_widgets::U_RADIAL_SLIDER_SET_SLIDER_RANGE,
                __buffer,
            )
        };
    }
    pub fn set_slider_progress_color(
        &mut self,
        in_value: crate::bindings::core_u_object::FLinearColor,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::advanced_widgets::U_RADIAL_SLIDER_SET_SLIDER_PROGRESS_COLOR,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_value,
                __buffer.add(0).cast::<crate::bindings::core_u_object::FLinearColor>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::advanced_widgets::U_RADIAL_SLIDER_SET_SLIDER_PROGRESS_COLOR,
                __buffer,
            )
        };
    }
    pub fn set_slider_handle_start_angle(&mut self, in_value: f32) {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::advanced_widgets::U_RADIAL_SLIDER_SET_SLIDER_HANDLE_START_ANGLE,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&in_value, __buffer.add(0).cast::<f32>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::advanced_widgets::U_RADIAL_SLIDER_SET_SLIDER_HANDLE_START_ANGLE,
                __buffer,
            )
        };
    }
    pub fn set_slider_handle_end_angle(&mut self, in_value: f32) {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::advanced_widgets::U_RADIAL_SLIDER_SET_SLIDER_HANDLE_END_ANGLE,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&in_value, __buffer.add(0).cast::<f32>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::advanced_widgets::U_RADIAL_SLIDER_SET_SLIDER_HANDLE_END_ANGLE,
                __buffer,
            )
        };
    }
    pub fn set_slider_handle_color(
        &mut self,
        in_value: crate::bindings::core_u_object::FLinearColor,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::advanced_widgets::U_RADIAL_SLIDER_SET_SLIDER_HANDLE_COLOR,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_value,
                __buffer.add(0).cast::<crate::bindings::core_u_object::FLinearColor>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::advanced_widgets::U_RADIAL_SLIDER_SET_SLIDER_HANDLE_COLOR,
                __buffer,
            )
        };
    }
    pub fn set_slider_bar_color(
        &mut self,
        in_value: crate::bindings::core_u_object::FLinearColor,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::advanced_widgets::U_RADIAL_SLIDER_SET_SLIDER_BAR_COLOR,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_value,
                __buffer.add(0).cast::<crate::bindings::core_u_object::FLinearColor>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::advanced_widgets::U_RADIAL_SLIDER_SET_SLIDER_BAR_COLOR,
                __buffer,
            )
        };
    }
    pub fn set_show_slider_handle(&mut self, in_show_slider_handle: bool) {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::advanced_widgets::U_RADIAL_SLIDER_SET_SHOW_SLIDER_HANDLE,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_show_slider_handle,
                __buffer.add(0).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::advanced_widgets::U_RADIAL_SLIDER_SET_SHOW_SLIDER_HANDLE,
                __buffer,
            )
        };
    }
    pub fn set_show_slider_hand(&mut self, in_show_slider_hand: bool) {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::advanced_widgets::U_RADIAL_SLIDER_SET_SHOW_SLIDER_HAND,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_show_slider_hand,
                __buffer.add(0).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::advanced_widgets::U_RADIAL_SLIDER_SET_SHOW_SLIDER_HAND,
                __buffer,
            )
        };
    }
    pub fn set_locked(&mut self, in_value: bool) {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::advanced_widgets::U_RADIAL_SLIDER_SET_LOCKED,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&in_value, __buffer.add(0).cast::<bool>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::advanced_widgets::U_RADIAL_SLIDER_SET_LOCKED,
                __buffer,
            )
        };
    }
    pub fn set_hand_start_end_ratio(
        &mut self,
        in_value: crate::bindings::core_u_object::FVector2D,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::advanced_widgets::U_RADIAL_SLIDER_SET_HAND_START_END_RATIO,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_value,
                __buffer.add(0).cast::<crate::bindings::core_u_object::FVector2D>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::advanced_widgets::U_RADIAL_SLIDER_SET_HAND_START_END_RATIO,
                __buffer,
            )
        };
    }
    pub fn set_custom_default_value(&mut self, in_value: f32) {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::advanced_widgets::U_RADIAL_SLIDER_SET_CUSTOM_DEFAULT_VALUE,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&in_value, __buffer.add(0).cast::<f32>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::advanced_widgets::U_RADIAL_SLIDER_SET_CUSTOM_DEFAULT_VALUE,
                __buffer,
            )
        };
    }
    pub fn set_center_background_color(
        &mut self,
        in_value: crate::bindings::core_u_object::FLinearColor,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::advanced_widgets::U_RADIAL_SLIDER_SET_CENTER_BACKGROUND_COLOR,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_value,
                __buffer.add(0).cast::<crate::bindings::core_u_object::FLinearColor>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::advanced_widgets::U_RADIAL_SLIDER_SET_CENTER_BACKGROUND_COLOR,
                __buffer,
            )
        };
    }
    pub fn set_angular_offset(&mut self, in_value: f32) {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::advanced_widgets::U_RADIAL_SLIDER_SET_ANGULAR_OFFSET,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&in_value, __buffer.add(0).cast::<f32>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::advanced_widgets::U_RADIAL_SLIDER_SET_ANGULAR_OFFSET,
                __buffer,
            )
        };
    }
    pub fn get_value(&self) -> f32 {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::advanced_widgets::U_RADIAL_SLIDER_GET_VALUE,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::advanced_widgets::U_RADIAL_SLIDER_GET_VALUE,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<f32>().read() }
    }
    pub fn get_normalized_slider_handle_position(&self) -> f32 {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::advanced_widgets::U_RADIAL_SLIDER_GET_NORMALIZED_SLIDER_HANDLE_POSITION,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::advanced_widgets::U_RADIAL_SLIDER_GET_NORMALIZED_SLIDER_HANDLE_POSITION,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<f32>().read() }
    }
    pub fn get_custom_default_value(&self) -> f32 {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::advanced_widgets::U_RADIAL_SLIDER_GET_CUSTOM_DEFAULT_VALUE,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::advanced_widgets::U_RADIAL_SLIDER_GET_CUSTOM_DEFAULT_VALUE,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<f32>().read() }
    }
}
#[repr(C, align(8))]
pub struct FRadialSlider_ValueDelegate {
    _opague: [u8; 32],
}
#[repr(C, align(8))]
pub struct FRadialSlider_OnMouseCaptureBegin {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FRadialSlider_OnMouseCaptureEnd {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FRadialSlider_OnControllerCaptureBegin {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FRadialSlider_OnControllerCaptureEnd {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FRadialSlider_OnValueChanged {
    _opague: [u8; 24],
}
