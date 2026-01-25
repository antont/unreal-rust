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
    pub u_svg_distance_field_generator_generate_texture_from_svg_file: *mut crate::ffi::UFunctionOpague,
}
impl FunctionPtrs {
    pub const fn empty() -> Self {
        Self {
            u_svg_distance_field_generator_generate_texture_from_svg_file: std::ptr::null_mut(),
        }
    }
}
pub fn initialize() {
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = USvgDistanceFieldGenerator::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GenerateTextureFromSvgFile"),
            &raw mut __FUNCTION_PTRS
                .u_svg_distance_field_generator_generate_texture_from_svg_file,
        );
    }
}
#[repr(C, align(4))]
pub struct FSvgDistanceFieldConfiguration {
    pub distance_field_type: ESvgDistanceFieldType,
    pub base_distance_spread: f32,
    pub extra_outer_distance_spread: f32,
    pub extra_inner_distance_spread: f32,
    pub distance_spread_units: ESvgDistanceFieldUnits,
    pub output_width: i32,
    pub output_height: i32,
    pub scale_mode: ESvgDistanceFieldScaleMode,
    pub scale: f32,
    pub placement_mode: ESvgDistanceFieldPlacementMode,
    pub miter_limit: f32,
}
impl FSvgDistanceFieldConfiguration {}
#[repr(C, align(8))]
pub struct USvgDistanceFieldGenerator {
    __padding_end: [u8; 48],
}
impl USvgDistanceFieldGenerator {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("USvgDistanceFieldGenerator")
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
    pub fn generate_texture_from_svg_file(
        svg_file_path: FString,
        configuration: &FSvgDistanceFieldConfiguration,
    ) -> UPtr<crate::bindings::engine::UTexture2D> {
        let mut __stack = crate::core_data::StackAlloc::<72>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::svg_distance_field::__FUNCTION_PTRS
                    .u_svg_distance_field_generator_generate_texture_from_svg_file,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &svg_file_path,
                __buffer.add(0).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                configuration,
                __buffer.add(16).cast::<FSvgDistanceFieldConfiguration>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::svg_distance_field::USvgDistanceFieldGenerator::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::svg_distance_field::__FUNCTION_PTRS
                    .u_svg_distance_field_generator_generate_texture_from_svg_file,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(64).cast::<UPtr<crate::bindings::engine::UTexture2D>>().read()
        }
    }
}
#[repr(transparent)]
pub struct ESvgDistanceFieldType(pub u8);
impl ESvgDistanceFieldType {
    pub const SIMPLE: ESvgDistanceFieldType = ESvgDistanceFieldType(0);
    pub const PERPENDICULAR: ESvgDistanceFieldType = ESvgDistanceFieldType(1);
    pub const MULTI_CHANNEL_AND_SIMPLE: ESvgDistanceFieldType = ESvgDistanceFieldType(2);
}
#[repr(transparent)]
pub struct ESvgDistanceFieldUnits(pub u8);
impl ESvgDistanceFieldUnits {
    pub const SVG_UNITS: ESvgDistanceFieldUnits = ESvgDistanceFieldUnits(0);
    pub const OUTPUT_PIXELS: ESvgDistanceFieldUnits = ESvgDistanceFieldUnits(1);
    pub const PROPORTIONAL_TO_MAX_DIMENSION: ESvgDistanceFieldUnits = ESvgDistanceFieldUnits(
        2,
    );
}
#[repr(transparent)]
pub struct ESvgDistanceFieldScaleMode(pub u8);
impl ESvgDistanceFieldScaleMode {
    pub const EXPLICIT_SCALE: ESvgDistanceFieldScaleMode = ESvgDistanceFieldScaleMode(0);
    pub const FIT_CANVAS: ESvgDistanceFieldScaleMode = ESvgDistanceFieldScaleMode(1);
    pub const FIT_PADDED_CANVAS: ESvgDistanceFieldScaleMode = ESvgDistanceFieldScaleMode(
        2,
    );
    pub const FIT_BOUNDING_BOX: ESvgDistanceFieldScaleMode = ESvgDistanceFieldScaleMode(
        3,
    );
}
#[repr(transparent)]
pub struct ESvgDistanceFieldPlacementMode(pub u8);
impl ESvgDistanceFieldPlacementMode {
    pub const DO_NOT_TRANSLATE: ESvgDistanceFieldPlacementMode = ESvgDistanceFieldPlacementMode(
        0,
    );
    pub const PAD_WITH_OUTER_SPREAD: ESvgDistanceFieldPlacementMode = ESvgDistanceFieldPlacementMode(
        1,
    );
    pub const CENTER_CANVAS: ESvgDistanceFieldPlacementMode = ESvgDistanceFieldPlacementMode(
        2,
    );
    pub const CENTER_BOUNDING_BOX: ESvgDistanceFieldPlacementMode = ESvgDistanceFieldPlacementMode(
        3,
    );
}
