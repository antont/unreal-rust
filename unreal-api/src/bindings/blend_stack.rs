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
pub struct FunctionPtrs {
    pub u_blend_stack_anim_node_library_is_current_asset_looping: *mut crate::ffi::UFunctionOpague,
    pub u_blend_stack_anim_node_library_get_current_blend_stack_anim_is_active: *mut crate::ffi::UFunctionOpague,
    pub u_blend_stack_anim_node_library_get_current_blend_stack_anim_asset_time: *mut crate::ffi::UFunctionOpague,
    pub u_blend_stack_anim_node_library_get_current_blend_stack_anim_asset_mirror_table: *mut crate::ffi::UFunctionOpague,
    pub u_blend_stack_anim_node_library_get_current_blend_stack_anim_asset_mirrored: *mut crate::ffi::UFunctionOpague,
    pub u_blend_stack_anim_node_library_get_current_blend_stack_anim_asset: *mut crate::ffi::UFunctionOpague,
    pub u_blend_stack_anim_node_library_get_current_asset_time_remaining: *mut crate::ffi::UFunctionOpague,
    pub u_blend_stack_anim_node_library_get_current_asset_time: *mut crate::ffi::UFunctionOpague,
    pub u_blend_stack_anim_node_library_get_current_asset: *mut crate::ffi::UFunctionOpague,
    pub u_blend_stack_anim_node_library_force_blend_next_update: *mut crate::ffi::UFunctionOpague,
    pub u_blend_stack_anim_node_library_convert_to_blend_stack_node_pure: *mut crate::ffi::UFunctionOpague,
    pub u_blend_stack_anim_node_library_convert_to_blend_stack_node: *mut crate::ffi::UFunctionOpague,
    pub u_blend_stack_anim_node_library_blend_to_with_settings: *mut crate::ffi::UFunctionOpague,
    pub u_blend_stack_anim_node_library_blend_to: *mut crate::ffi::UFunctionOpague,
    pub u_blend_stack_input_anim_node_library_get_properties: *mut crate::ffi::UFunctionOpague,
    pub u_blend_stack_input_anim_node_library_convert_to_blend_stack_input_node_pure: *mut crate::ffi::UFunctionOpague,
    pub u_blend_stack_input_anim_node_library_convert_to_blend_stack_input_node: *mut crate::ffi::UFunctionOpague,
}
impl FunctionPtrs {
    pub const fn empty() -> Self {
        Self {
            u_blend_stack_anim_node_library_is_current_asset_looping: std::ptr::null_mut(),
            u_blend_stack_anim_node_library_get_current_blend_stack_anim_is_active: std::ptr::null_mut(),
            u_blend_stack_anim_node_library_get_current_blend_stack_anim_asset_time: std::ptr::null_mut(),
            u_blend_stack_anim_node_library_get_current_blend_stack_anim_asset_mirror_table: std::ptr::null_mut(),
            u_blend_stack_anim_node_library_get_current_blend_stack_anim_asset_mirrored: std::ptr::null_mut(),
            u_blend_stack_anim_node_library_get_current_blend_stack_anim_asset: std::ptr::null_mut(),
            u_blend_stack_anim_node_library_get_current_asset_time_remaining: std::ptr::null_mut(),
            u_blend_stack_anim_node_library_get_current_asset_time: std::ptr::null_mut(),
            u_blend_stack_anim_node_library_get_current_asset: std::ptr::null_mut(),
            u_blend_stack_anim_node_library_force_blend_next_update: std::ptr::null_mut(),
            u_blend_stack_anim_node_library_convert_to_blend_stack_node_pure: std::ptr::null_mut(),
            u_blend_stack_anim_node_library_convert_to_blend_stack_node: std::ptr::null_mut(),
            u_blend_stack_anim_node_library_blend_to_with_settings: std::ptr::null_mut(),
            u_blend_stack_anim_node_library_blend_to: std::ptr::null_mut(),
            u_blend_stack_input_anim_node_library_get_properties: std::ptr::null_mut(),
            u_blend_stack_input_anim_node_library_convert_to_blend_stack_input_node_pure: std::ptr::null_mut(),
            u_blend_stack_input_anim_node_library_convert_to_blend_stack_input_node: std::ptr::null_mut(),
        }
    }
}
pub fn initialize() {
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UBlendStackAnimNodeLibrary::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("IsCurrentAssetLooping"),
                &raw mut __FUNCTION_PTRS
                    .u_blend_stack_anim_node_library_is_current_asset_looping,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetCurrentBlendStackAnimIsActive"),
                &raw mut __FUNCTION_PTRS
                    .u_blend_stack_anim_node_library_get_current_blend_stack_anim_is_active,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetCurrentBlendStackAnimAssetTime"),
                &raw mut __FUNCTION_PTRS
                    .u_blend_stack_anim_node_library_get_current_blend_stack_anim_asset_time,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetCurrentBlendStackAnimAssetMirrorTable"),
                &raw mut __FUNCTION_PTRS
                    .u_blend_stack_anim_node_library_get_current_blend_stack_anim_asset_mirror_table,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetCurrentBlendStackAnimAssetMirrored"),
                &raw mut __FUNCTION_PTRS
                    .u_blend_stack_anim_node_library_get_current_blend_stack_anim_asset_mirrored,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetCurrentBlendStackAnimAsset"),
                &raw mut __FUNCTION_PTRS
                    .u_blend_stack_anim_node_library_get_current_blend_stack_anim_asset,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetCurrentAssetTimeRemaining"),
                &raw mut __FUNCTION_PTRS
                    .u_blend_stack_anim_node_library_get_current_asset_time_remaining,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetCurrentAssetTime"),
                &raw mut __FUNCTION_PTRS
                    .u_blend_stack_anim_node_library_get_current_asset_time,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetCurrentAsset"),
                &raw mut __FUNCTION_PTRS
                    .u_blend_stack_anim_node_library_get_current_asset,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ForceBlendNextUpdate"),
                &raw mut __FUNCTION_PTRS
                    .u_blend_stack_anim_node_library_force_blend_next_update,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ConvertToBlendStackNodePure"),
                &raw mut __FUNCTION_PTRS
                    .u_blend_stack_anim_node_library_convert_to_blend_stack_node_pure,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ConvertToBlendStackNode"),
                &raw mut __FUNCTION_PTRS
                    .u_blend_stack_anim_node_library_convert_to_blend_stack_node,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("BlendToWithSettings"),
                &raw mut __FUNCTION_PTRS
                    .u_blend_stack_anim_node_library_blend_to_with_settings,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("BlendTo"),
                &raw mut __FUNCTION_PTRS.u_blend_stack_anim_node_library_blend_to,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UBlendStackInputAnimNodeLibrary::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetProperties"),
                &raw mut __FUNCTION_PTRS
                    .u_blend_stack_input_anim_node_library_get_properties,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ConvertToBlendStackInputNodePure"),
                &raw mut __FUNCTION_PTRS
                    .u_blend_stack_input_anim_node_library_convert_to_blend_stack_input_node_pure,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ConvertToBlendStackInputNode"),
                &raw mut __FUNCTION_PTRS
                    .u_blend_stack_input_anim_node_library_convert_to_blend_stack_input_node,
            );
        }
    }
}
#[repr(C, align(8))]
pub struct FAnimNode_BlendStack_Standalone {
    #[doc(hidden)]
    pub(crate) __padding_248: [u8; 248],
    pub stitch_database: UPtr<crate::bindings::core_u_object::UObject>,
    pub stitch_blend_time: f32,
    pub stitch_blend_max_cost: f32,
    #[doc(hidden)]
    pub(crate) __padding_304: [u8; 40],
    pub notify_recency_time_out: f32,
    pub(crate) __padding_end: [u8; 12],
}
impl FAnimNode_BlendStack_Standalone {}
#[repr(C, align(8))]
pub struct FAnimNode_BlendStack {
    #[doc(hidden)]
    pub(crate) __padding_320: [u8; 320],
    pub animation_asset: UPtr<crate::bindings::engine::UAnimationAsset>,
    pub animation_time: f32,
    pub activation_delay_time: f32,
    pub b_loop: bool,
    pub b_mirrored: bool,
    pub wanted_play_rate: f32,
    pub blend_time: f32,
    pub max_animation_delta_time: f32,
    pub blend_profile: UPtr<crate::bindings::engine::UBlendProfile>,
    pub blend_option: crate::bindings::engine::EAlphaBlendOption,
    pub blend_parameters: crate::bindings::core_u_object::FVector,
    pub(crate) __padding_end: [u8; 72],
}
impl FAnimNode_BlendStack {}
#[repr(C, align(8))]
pub struct FAnimNode_BlendStackInput {
    pub(crate) __padding_end: [u8; 160],
}
impl FAnimNode_BlendStackInput {}
#[repr(C, align(8))]
pub struct FBlendStackAnimNodeReference {
    pub(crate) __padding_end: [u8; 16],
}
impl FBlendStackAnimNodeReference {}
#[repr(C, align(8))]
pub struct FBlendStackInputAnimNodeReference {
    pub(crate) __padding_end: [u8; 16],
}
impl FBlendStackInputAnimNodeReference {}
#[repr(C, align(8))]
pub struct UBlendStackAnimNodeLibrary {
    __padding_end: [u8; 48],
}
impl UBlendStackAnimNodeLibrary {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBlendStackAnimNodeLibrary")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBlendStackAnimNodeLibrary")
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
    pub fn is_current_asset_looping(
        blend_stack_node: &FBlendStackAnimNodeReference,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<17>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::blend_stack::__FUNCTION_PTRS
                    .u_blend_stack_anim_node_library_is_current_asset_looping,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                blend_stack_node,
                __buffer.add(0).cast::<FBlendStackAnimNodeReference>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::blend_stack::UBlendStackAnimNodeLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::blend_stack::__FUNCTION_PTRS
                    .u_blend_stack_anim_node_library_is_current_asset_looping,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<bool>().read() }
    }
    pub fn get_current_blend_stack_anim_is_active(
        node: &crate::bindings::engine::FAnimNodeReference,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<17>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::blend_stack::__FUNCTION_PTRS
                    .u_blend_stack_anim_node_library_get_current_blend_stack_anim_is_active,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                node,
                __buffer.add(0).cast::<crate::bindings::engine::FAnimNodeReference>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::blend_stack::UBlendStackAnimNodeLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::blend_stack::__FUNCTION_PTRS
                    .u_blend_stack_anim_node_library_get_current_blend_stack_anim_is_active,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<bool>().read() }
    }
    pub fn get_current_blend_stack_anim_asset_time(
        node: &crate::bindings::engine::FAnimNodeReference,
    ) -> f32 {
        let mut __stack = crate::core_data::StackAlloc::<20>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::blend_stack::__FUNCTION_PTRS
                    .u_blend_stack_anim_node_library_get_current_blend_stack_anim_asset_time,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                node,
                __buffer.add(0).cast::<crate::bindings::engine::FAnimNodeReference>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::blend_stack::UBlendStackAnimNodeLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::blend_stack::__FUNCTION_PTRS
                    .u_blend_stack_anim_node_library_get_current_blend_stack_anim_asset_time,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<f32>().read() }
    }
    pub fn get_current_blend_stack_anim_asset_mirror_table(
        node: &crate::bindings::engine::FAnimNodeReference,
    ) -> UPtr<crate::bindings::engine::UMirrorDataTable> {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::blend_stack::__FUNCTION_PTRS
                    .u_blend_stack_anim_node_library_get_current_blend_stack_anim_asset_mirror_table,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                node,
                __buffer.add(0).cast::<crate::bindings::engine::FAnimNodeReference>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::blend_stack::UBlendStackAnimNodeLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::blend_stack::__FUNCTION_PTRS
                    .u_blend_stack_anim_node_library_get_current_blend_stack_anim_asset_mirror_table,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(16)
                .cast::<UPtr<crate::bindings::engine::UMirrorDataTable>>()
                .read()
        }
    }
    pub fn get_current_blend_stack_anim_asset_mirrored(
        node: &crate::bindings::engine::FAnimNodeReference,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<17>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::blend_stack::__FUNCTION_PTRS
                    .u_blend_stack_anim_node_library_get_current_blend_stack_anim_asset_mirrored,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                node,
                __buffer.add(0).cast::<crate::bindings::engine::FAnimNodeReference>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::blend_stack::UBlendStackAnimNodeLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::blend_stack::__FUNCTION_PTRS
                    .u_blend_stack_anim_node_library_get_current_blend_stack_anim_asset_mirrored,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<bool>().read() }
    }
    pub fn get_current_blend_stack_anim_asset(
        node: &crate::bindings::engine::FAnimNodeReference,
    ) -> UPtr<crate::bindings::engine::UAnimationAsset> {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::blend_stack::__FUNCTION_PTRS
                    .u_blend_stack_anim_node_library_get_current_blend_stack_anim_asset,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                node,
                __buffer.add(0).cast::<crate::bindings::engine::FAnimNodeReference>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::blend_stack::UBlendStackAnimNodeLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::blend_stack::__FUNCTION_PTRS
                    .u_blend_stack_anim_node_library_get_current_blend_stack_anim_asset,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(16)
                .cast::<UPtr<crate::bindings::engine::UAnimationAsset>>()
                .read()
        }
    }
    pub fn get_current_asset_time_remaining(
        blend_stack_node: &FBlendStackAnimNodeReference,
    ) -> f32 {
        let mut __stack = crate::core_data::StackAlloc::<20>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::blend_stack::__FUNCTION_PTRS
                    .u_blend_stack_anim_node_library_get_current_asset_time_remaining,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                blend_stack_node,
                __buffer.add(0).cast::<FBlendStackAnimNodeReference>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::blend_stack::UBlendStackAnimNodeLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::blend_stack::__FUNCTION_PTRS
                    .u_blend_stack_anim_node_library_get_current_asset_time_remaining,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<f32>().read() }
    }
    pub fn get_current_asset_time(
        blend_stack_node: &FBlendStackAnimNodeReference,
    ) -> f32 {
        let mut __stack = crate::core_data::StackAlloc::<20>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::blend_stack::__FUNCTION_PTRS
                    .u_blend_stack_anim_node_library_get_current_asset_time,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                blend_stack_node,
                __buffer.add(0).cast::<FBlendStackAnimNodeReference>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::blend_stack::UBlendStackAnimNodeLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::blend_stack::__FUNCTION_PTRS
                    .u_blend_stack_anim_node_library_get_current_asset_time,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<f32>().read() }
    }
    pub fn get_current_asset(
        blend_stack_node: &FBlendStackAnimNodeReference,
    ) -> UPtr<crate::bindings::engine::UAnimationAsset> {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::blend_stack::__FUNCTION_PTRS
                    .u_blend_stack_anim_node_library_get_current_asset,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                blend_stack_node,
                __buffer.add(0).cast::<FBlendStackAnimNodeReference>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::blend_stack::UBlendStackAnimNodeLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::blend_stack::__FUNCTION_PTRS
                    .u_blend_stack_anim_node_library_get_current_asset,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(16)
                .cast::<UPtr<crate::bindings::engine::UAnimationAsset>>()
                .read()
        }
    }
    pub fn force_blend_next_update(blend_stack_node: &FBlendStackAnimNodeReference) {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::blend_stack::__FUNCTION_PTRS
                    .u_blend_stack_anim_node_library_force_blend_next_update,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                blend_stack_node,
                __buffer.add(0).cast::<FBlendStackAnimNodeReference>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::blend_stack::UBlendStackAnimNodeLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::blend_stack::__FUNCTION_PTRS
                    .u_blend_stack_anim_node_library_force_blend_next_update,
                __buffer,
            )
        };
    }
    pub fn convert_to_blend_stack_node_pure(
        node: &crate::bindings::engine::FAnimNodeReference,
        blend_stack_node: &mut FBlendStackAnimNodeReference,
        result: &mut bool,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<33>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::blend_stack::__FUNCTION_PTRS
                    .u_blend_stack_anim_node_library_convert_to_blend_stack_node_pure,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                node,
                __buffer.add(0).cast::<crate::bindings::engine::FAnimNodeReference>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                blend_stack_node,
                __buffer.add(16).cast::<FBlendStackAnimNodeReference>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(result, __buffer.add(32).cast::<bool>(), 1);
        }
        let __object_ptr = crate::bindings::blend_stack::UBlendStackAnimNodeLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::blend_stack::__FUNCTION_PTRS
                    .u_blend_stack_anim_node_library_convert_to_blend_stack_node_pure,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(16)
                .cast::<FBlendStackAnimNodeReference>()
                .swap(blend_stack_node);
        }
        unsafe {
            __buffer.add(32).cast::<bool>().swap(result);
        }
    }
    pub fn convert_to_blend_stack_node(
        node: &crate::bindings::engine::FAnimNodeReference,
        result: &mut crate::bindings::engine::EAnimNodeReferenceConversionResult,
    ) -> FBlendStackAnimNodeReference {
        let mut __stack = crate::core_data::StackAlloc::<40>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::blend_stack::__FUNCTION_PTRS
                    .u_blend_stack_anim_node_library_convert_to_blend_stack_node,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                node,
                __buffer.add(0).cast::<crate::bindings::engine::FAnimNodeReference>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                result,
                __buffer
                    .add(16)
                    .cast::<
                        crate::bindings::engine::EAnimNodeReferenceConversionResult,
                    >(),
                1,
            );
        }
        let __object_ptr = crate::bindings::blend_stack::UBlendStackAnimNodeLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::blend_stack::__FUNCTION_PTRS
                    .u_blend_stack_anim_node_library_convert_to_blend_stack_node,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(16)
                .cast::<crate::bindings::engine::EAnimNodeReferenceConversionResult>()
                .swap(result);
        }
        unsafe { __buffer.add(24).cast::<FBlendStackAnimNodeReference>().read() }
    }
    pub fn blend_to_with_settings(
        context: &crate::bindings::engine::FAnimUpdateContext,
        blend_stack_node: &FBlendStackAnimNodeReference,
        animation_asset: UPtr<crate::bindings::engine::UAnimationAsset>,
        animation_time: f32,
        b_loop: bool,
        b_mirrored: bool,
        blend_time: f32,
        blend_profile: UPtr<crate::bindings::engine::UBlendProfile>,
        blend_option: crate::bindings::engine::EAlphaBlendOption,
        b_inertial_blend: bool,
        blend_parameters: crate::bindings::core_u_object::FVector,
        wanted_play_rate: f32,
        activation_delay: f32,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<104>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::blend_stack::__FUNCTION_PTRS
                    .u_blend_stack_anim_node_library_blend_to_with_settings,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                context,
                __buffer.add(0).cast::<crate::bindings::engine::FAnimUpdateContext>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                blend_stack_node,
                __buffer.add(16).cast::<FBlendStackAnimNodeReference>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &animation_asset,
                __buffer
                    .add(32)
                    .cast::<UPtr<crate::bindings::engine::UAnimationAsset>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &animation_time,
                __buffer.add(40).cast::<f32>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&b_loop, __buffer.add(44).cast::<bool>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_mirrored,
                __buffer.add(45).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &blend_time,
                __buffer.add(48).cast::<f32>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &blend_profile,
                __buffer.add(56).cast::<UPtr<crate::bindings::engine::UBlendProfile>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &blend_option,
                __buffer.add(64).cast::<crate::bindings::engine::EAlphaBlendOption>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_inertial_blend,
                __buffer.add(65).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &blend_parameters,
                __buffer.add(72).cast::<crate::bindings::core_u_object::FVector>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &wanted_play_rate,
                __buffer.add(96).cast::<f32>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &activation_delay,
                __buffer.add(100).cast::<f32>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::blend_stack::UBlendStackAnimNodeLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::blend_stack::__FUNCTION_PTRS
                    .u_blend_stack_anim_node_library_blend_to_with_settings,
                __buffer,
            )
        };
        std::mem::forget(animation_asset);
        std::mem::forget(animation_time);
        std::mem::forget(b_loop);
        std::mem::forget(b_mirrored);
        std::mem::forget(blend_time);
        std::mem::forget(blend_profile);
        std::mem::forget(blend_option);
        std::mem::forget(b_inertial_blend);
        std::mem::forget(blend_parameters);
        std::mem::forget(wanted_play_rate);
        std::mem::forget(activation_delay);
    }
    pub fn blend_to(
        context: &crate::bindings::engine::FAnimUpdateContext,
        blend_stack_node: &FBlendStackAnimNodeReference,
        animation_asset: UPtr<crate::bindings::engine::UAnimationAsset>,
        animation_time: f32,
        b_loop: bool,
        b_mirrored: bool,
        blend_time: f32,
        blend_parameters: crate::bindings::core_u_object::FVector,
        wanted_play_rate: f32,
        activation_delay: f32,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<88>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::blend_stack::__FUNCTION_PTRS
                    .u_blend_stack_anim_node_library_blend_to,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                context,
                __buffer.add(0).cast::<crate::bindings::engine::FAnimUpdateContext>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                blend_stack_node,
                __buffer.add(16).cast::<FBlendStackAnimNodeReference>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &animation_asset,
                __buffer
                    .add(32)
                    .cast::<UPtr<crate::bindings::engine::UAnimationAsset>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &animation_time,
                __buffer.add(40).cast::<f32>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&b_loop, __buffer.add(44).cast::<bool>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_mirrored,
                __buffer.add(45).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &blend_time,
                __buffer.add(48).cast::<f32>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &blend_parameters,
                __buffer.add(56).cast::<crate::bindings::core_u_object::FVector>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &wanted_play_rate,
                __buffer.add(80).cast::<f32>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &activation_delay,
                __buffer.add(84).cast::<f32>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::blend_stack::UBlendStackAnimNodeLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::blend_stack::__FUNCTION_PTRS
                    .u_blend_stack_anim_node_library_blend_to,
                __buffer,
            )
        };
        std::mem::forget(animation_asset);
        std::mem::forget(animation_time);
        std::mem::forget(b_loop);
        std::mem::forget(b_mirrored);
        std::mem::forget(blend_time);
        std::mem::forget(blend_parameters);
        std::mem::forget(wanted_play_rate);
        std::mem::forget(activation_delay);
    }
}
#[repr(C, align(8))]
pub struct UBlendStackInputAnimNodeLibrary {
    __padding_end: [u8; 48],
}
impl UBlendStackInputAnimNodeLibrary {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBlendStackInputAnimNodeLibrary")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBlendStackInputAnimNodeLibrary")
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
    pub fn get_properties(
        blend_stack_input_node: &FBlendStackInputAnimNodeReference,
        animation_asset: &mut UPtr<crate::bindings::engine::UAnimationAsset>,
        accumulated_time: &mut f32,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<28>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::blend_stack::__FUNCTION_PTRS
                    .u_blend_stack_input_anim_node_library_get_properties,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                blend_stack_input_node,
                __buffer.add(0).cast::<FBlendStackInputAnimNodeReference>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                animation_asset,
                __buffer
                    .add(16)
                    .cast::<UPtr<crate::bindings::engine::UAnimationAsset>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                accumulated_time,
                __buffer.add(24).cast::<f32>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::blend_stack::UBlendStackInputAnimNodeLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::blend_stack::__FUNCTION_PTRS
                    .u_blend_stack_input_anim_node_library_get_properties,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(16)
                .cast::<UPtr<crate::bindings::engine::UAnimationAsset>>()
                .swap(animation_asset);
        }
        unsafe {
            __buffer.add(24).cast::<f32>().swap(accumulated_time);
        }
    }
    pub fn convert_to_blend_stack_input_node_pure(
        node: &crate::bindings::engine::FAnimNodeReference,
        blend_stack_input_node: &mut FBlendStackInputAnimNodeReference,
        result: &mut bool,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<33>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::blend_stack::__FUNCTION_PTRS
                    .u_blend_stack_input_anim_node_library_convert_to_blend_stack_input_node_pure,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                node,
                __buffer.add(0).cast::<crate::bindings::engine::FAnimNodeReference>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                blend_stack_input_node,
                __buffer.add(16).cast::<FBlendStackInputAnimNodeReference>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(result, __buffer.add(32).cast::<bool>(), 1);
        }
        let __object_ptr = crate::bindings::blend_stack::UBlendStackInputAnimNodeLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::blend_stack::__FUNCTION_PTRS
                    .u_blend_stack_input_anim_node_library_convert_to_blend_stack_input_node_pure,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(16)
                .cast::<FBlendStackInputAnimNodeReference>()
                .swap(blend_stack_input_node);
        }
        unsafe {
            __buffer.add(32).cast::<bool>().swap(result);
        }
    }
    pub fn convert_to_blend_stack_input_node(
        node: &crate::bindings::engine::FAnimNodeReference,
        result: &mut crate::bindings::engine::EAnimNodeReferenceConversionResult,
    ) -> FBlendStackInputAnimNodeReference {
        let mut __stack = crate::core_data::StackAlloc::<40>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::blend_stack::__FUNCTION_PTRS
                    .u_blend_stack_input_anim_node_library_convert_to_blend_stack_input_node,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                node,
                __buffer.add(0).cast::<crate::bindings::engine::FAnimNodeReference>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                result,
                __buffer
                    .add(16)
                    .cast::<
                        crate::bindings::engine::EAnimNodeReferenceConversionResult,
                    >(),
                1,
            );
        }
        let __object_ptr = crate::bindings::blend_stack::UBlendStackInputAnimNodeLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::blend_stack::__FUNCTION_PTRS
                    .u_blend_stack_input_anim_node_library_convert_to_blend_stack_input_node,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(16)
                .cast::<crate::bindings::engine::EAnimNodeReferenceConversionResult>()
                .swap(result);
        }
        unsafe { __buffer.add(24).cast::<FBlendStackInputAnimNodeReference>().read() }
    }
}
#[repr(transparent)]
pub struct EBlendStack_BlendspaceUpdateMode(pub u8);
impl EBlendStack_BlendspaceUpdateMode {
    pub const INITIAL_ONLY: EBlendStack_BlendspaceUpdateMode = EBlendStack_BlendspaceUpdateMode(
        0,
    );
    pub const UPDATE_ACTIVE_ONLY: EBlendStack_BlendspaceUpdateMode = EBlendStack_BlendspaceUpdateMode(
        1,
    );
    pub const UPDATE_ALL: EBlendStack_BlendspaceUpdateMode = EBlendStack_BlendspaceUpdateMode(
        2,
    );
}
