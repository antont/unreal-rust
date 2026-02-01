#![allow(clippy::all)]
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::new_without_default)]
#![allow(clippy::new_ret_no_self)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[doc(hidden)]
pub static mut __FUNCTION_PTRS: FunctionPtrs = FunctionPtrs::empty();
pub struct FunctionPtrs {
    pub u_radial_slider_set_value_tags: *mut crate::ffi::UFunctionOpague,
    pub u_radial_slider_set_value: *mut crate::ffi::UFunctionOpague,
    pub u_radial_slider_set_use_vertical_drag: *mut crate::ffi::UFunctionOpague,
    pub u_radial_slider_set_step_size: *mut crate::ffi::UFunctionOpague,
    pub u_radial_slider_set_slider_range: *mut crate::ffi::UFunctionOpague,
    pub u_radial_slider_set_slider_progress_color: *mut crate::ffi::UFunctionOpague,
    pub u_radial_slider_set_slider_handle_start_angle: *mut crate::ffi::UFunctionOpague,
    pub u_radial_slider_set_slider_handle_end_angle: *mut crate::ffi::UFunctionOpague,
    pub u_radial_slider_set_slider_handle_color: *mut crate::ffi::UFunctionOpague,
    pub u_radial_slider_set_slider_bar_color: *mut crate::ffi::UFunctionOpague,
    pub u_radial_slider_set_show_slider_handle: *mut crate::ffi::UFunctionOpague,
    pub u_radial_slider_set_show_slider_hand: *mut crate::ffi::UFunctionOpague,
    pub u_radial_slider_set_locked: *mut crate::ffi::UFunctionOpague,
    pub u_radial_slider_set_hand_start_end_ratio: *mut crate::ffi::UFunctionOpague,
    pub u_radial_slider_set_custom_default_value: *mut crate::ffi::UFunctionOpague,
    pub u_radial_slider_set_center_background_color: *mut crate::ffi::UFunctionOpague,
    pub u_radial_slider_set_angular_offset: *mut crate::ffi::UFunctionOpague,
    pub u_radial_slider_get_value: *mut crate::ffi::UFunctionOpague,
    pub u_radial_slider_get_normalized_slider_handle_position: *mut crate::ffi::UFunctionOpague,
    pub u_radial_slider_get_custom_default_value: *mut crate::ffi::UFunctionOpague,
}
impl FunctionPtrs {
    pub const fn empty() -> Self {
        Self {
            u_radial_slider_set_value_tags: std::ptr::null_mut(),
            u_radial_slider_set_value: std::ptr::null_mut(),
            u_radial_slider_set_use_vertical_drag: std::ptr::null_mut(),
            u_radial_slider_set_step_size: std::ptr::null_mut(),
            u_radial_slider_set_slider_range: std::ptr::null_mut(),
            u_radial_slider_set_slider_progress_color: std::ptr::null_mut(),
            u_radial_slider_set_slider_handle_start_angle: std::ptr::null_mut(),
            u_radial_slider_set_slider_handle_end_angle: std::ptr::null_mut(),
            u_radial_slider_set_slider_handle_color: std::ptr::null_mut(),
            u_radial_slider_set_slider_bar_color: std::ptr::null_mut(),
            u_radial_slider_set_show_slider_handle: std::ptr::null_mut(),
            u_radial_slider_set_show_slider_hand: std::ptr::null_mut(),
            u_radial_slider_set_locked: std::ptr::null_mut(),
            u_radial_slider_set_hand_start_end_ratio: std::ptr::null_mut(),
            u_radial_slider_set_custom_default_value: std::ptr::null_mut(),
            u_radial_slider_set_center_background_color: std::ptr::null_mut(),
            u_radial_slider_set_angular_offset: std::ptr::null_mut(),
            u_radial_slider_get_value: std::ptr::null_mut(),
            u_radial_slider_get_normalized_slider_handle_position: std::ptr::null_mut(),
            u_radial_slider_get_custom_default_value: std::ptr::null_mut(),
        }
    }
}
pub fn initialize() {
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = URadialSlider::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetValueTags"),
                &raw mut __FUNCTION_PTRS.u_radial_slider_set_value_tags,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetValue"),
                &raw mut __FUNCTION_PTRS.u_radial_slider_set_value,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetUseVerticalDrag"),
                &raw mut __FUNCTION_PTRS.u_radial_slider_set_use_vertical_drag,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetStepSize"),
                &raw mut __FUNCTION_PTRS.u_radial_slider_set_step_size,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetSliderRange"),
                &raw mut __FUNCTION_PTRS.u_radial_slider_set_slider_range,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetSliderProgressColor"),
                &raw mut __FUNCTION_PTRS.u_radial_slider_set_slider_progress_color,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetSliderHandleStartAngle"),
                &raw mut __FUNCTION_PTRS.u_radial_slider_set_slider_handle_start_angle,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetSliderHandleEndAngle"),
                &raw mut __FUNCTION_PTRS.u_radial_slider_set_slider_handle_end_angle,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetSliderHandleColor"),
                &raw mut __FUNCTION_PTRS.u_radial_slider_set_slider_handle_color,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetSliderBarColor"),
                &raw mut __FUNCTION_PTRS.u_radial_slider_set_slider_bar_color,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetShowSliderHandle"),
                &raw mut __FUNCTION_PTRS.u_radial_slider_set_show_slider_handle,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetShowSliderHand"),
                &raw mut __FUNCTION_PTRS.u_radial_slider_set_show_slider_hand,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetLocked"),
                &raw mut __FUNCTION_PTRS.u_radial_slider_set_locked,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetHandStartEndRatio"),
                &raw mut __FUNCTION_PTRS.u_radial_slider_set_hand_start_end_ratio,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetCustomDefaultValue"),
                &raw mut __FUNCTION_PTRS.u_radial_slider_set_custom_default_value,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetCenterBackgroundColor"),
                &raw mut __FUNCTION_PTRS.u_radial_slider_set_center_background_color,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetAngularOffset"),
                &raw mut __FUNCTION_PTRS.u_radial_slider_set_angular_offset,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetValue"),
                &raw mut __FUNCTION_PTRS.u_radial_slider_get_value,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetNormalizedSliderHandlePosition"),
                &raw mut __FUNCTION_PTRS
                    .u_radial_slider_get_normalized_slider_handle_position,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetCustomDefaultValue"),
                &raw mut __FUNCTION_PTRS.u_radial_slider_get_custom_default_value,
            );
        }
    }
}
#[repr(C, align(16))]
pub struct FColorGradingSpinBoxStyle {
    #[doc(hidden)]
    pub(crate) __padding_16: [u8; 16],
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
    pub(crate) __padding_696: [u8; 696],
    pub value: f32,
    #[doc(hidden)]
    pub(crate) __padding_744: [u8; 40],
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
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("URadialSlider")
            .copied()
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
                crate::bindings::advanced_widgets::__FUNCTION_PTRS
                    .u_radial_slider_set_value_tags,
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
                crate::bindings::advanced_widgets::__FUNCTION_PTRS
                    .u_radial_slider_set_value_tags,
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
                crate::bindings::advanced_widgets::__FUNCTION_PTRS
                    .u_radial_slider_set_value,
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
                crate::bindings::advanced_widgets::__FUNCTION_PTRS
                    .u_radial_slider_set_value,
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
                crate::bindings::advanced_widgets::__FUNCTION_PTRS
                    .u_radial_slider_set_use_vertical_drag,
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
                crate::bindings::advanced_widgets::__FUNCTION_PTRS
                    .u_radial_slider_set_use_vertical_drag,
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
                crate::bindings::advanced_widgets::__FUNCTION_PTRS
                    .u_radial_slider_set_step_size,
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
                crate::bindings::advanced_widgets::__FUNCTION_PTRS
                    .u_radial_slider_set_step_size,
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
                crate::bindings::advanced_widgets::__FUNCTION_PTRS
                    .u_radial_slider_set_slider_range,
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
                crate::bindings::advanced_widgets::__FUNCTION_PTRS
                    .u_radial_slider_set_slider_range,
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
                crate::bindings::advanced_widgets::__FUNCTION_PTRS
                    .u_radial_slider_set_slider_progress_color,
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
                crate::bindings::advanced_widgets::__FUNCTION_PTRS
                    .u_radial_slider_set_slider_progress_color,
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
                crate::bindings::advanced_widgets::__FUNCTION_PTRS
                    .u_radial_slider_set_slider_handle_start_angle,
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
                crate::bindings::advanced_widgets::__FUNCTION_PTRS
                    .u_radial_slider_set_slider_handle_start_angle,
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
                crate::bindings::advanced_widgets::__FUNCTION_PTRS
                    .u_radial_slider_set_slider_handle_end_angle,
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
                crate::bindings::advanced_widgets::__FUNCTION_PTRS
                    .u_radial_slider_set_slider_handle_end_angle,
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
                crate::bindings::advanced_widgets::__FUNCTION_PTRS
                    .u_radial_slider_set_slider_handle_color,
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
                crate::bindings::advanced_widgets::__FUNCTION_PTRS
                    .u_radial_slider_set_slider_handle_color,
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
                crate::bindings::advanced_widgets::__FUNCTION_PTRS
                    .u_radial_slider_set_slider_bar_color,
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
                crate::bindings::advanced_widgets::__FUNCTION_PTRS
                    .u_radial_slider_set_slider_bar_color,
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
                crate::bindings::advanced_widgets::__FUNCTION_PTRS
                    .u_radial_slider_set_show_slider_handle,
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
                crate::bindings::advanced_widgets::__FUNCTION_PTRS
                    .u_radial_slider_set_show_slider_handle,
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
                crate::bindings::advanced_widgets::__FUNCTION_PTRS
                    .u_radial_slider_set_show_slider_hand,
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
                crate::bindings::advanced_widgets::__FUNCTION_PTRS
                    .u_radial_slider_set_show_slider_hand,
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
                crate::bindings::advanced_widgets::__FUNCTION_PTRS
                    .u_radial_slider_set_locked,
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
                crate::bindings::advanced_widgets::__FUNCTION_PTRS
                    .u_radial_slider_set_locked,
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
                crate::bindings::advanced_widgets::__FUNCTION_PTRS
                    .u_radial_slider_set_hand_start_end_ratio,
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
                crate::bindings::advanced_widgets::__FUNCTION_PTRS
                    .u_radial_slider_set_hand_start_end_ratio,
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
                crate::bindings::advanced_widgets::__FUNCTION_PTRS
                    .u_radial_slider_set_custom_default_value,
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
                crate::bindings::advanced_widgets::__FUNCTION_PTRS
                    .u_radial_slider_set_custom_default_value,
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
                crate::bindings::advanced_widgets::__FUNCTION_PTRS
                    .u_radial_slider_set_center_background_color,
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
                crate::bindings::advanced_widgets::__FUNCTION_PTRS
                    .u_radial_slider_set_center_background_color,
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
                crate::bindings::advanced_widgets::__FUNCTION_PTRS
                    .u_radial_slider_set_angular_offset,
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
                crate::bindings::advanced_widgets::__FUNCTION_PTRS
                    .u_radial_slider_set_angular_offset,
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
                crate::bindings::advanced_widgets::__FUNCTION_PTRS
                    .u_radial_slider_get_value,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::advanced_widgets::__FUNCTION_PTRS
                    .u_radial_slider_get_value,
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
                crate::bindings::advanced_widgets::__FUNCTION_PTRS
                    .u_radial_slider_get_normalized_slider_handle_position,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::advanced_widgets::__FUNCTION_PTRS
                    .u_radial_slider_get_normalized_slider_handle_position,
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
                crate::bindings::advanced_widgets::__FUNCTION_PTRS
                    .u_radial_slider_get_custom_default_value,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::advanced_widgets::__FUNCTION_PTRS
                    .u_radial_slider_get_custom_default_value,
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
