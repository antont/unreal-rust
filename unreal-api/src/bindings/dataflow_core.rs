#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
pub struct UDataflowGraphInterface {}
pub struct IDataflowGraphInterface {}
#[repr(C, align(8))]
pub struct UDataflowSettings {
    __padding_end: [u8; 360],
}
impl UDataflowSettings {}
#[repr(transparent)]
pub struct EDataflowImageResolution(pub i32);
impl EDataflowImageResolution {
    pub const RESOLUTION16: EDataflowImageResolution = EDataflowImageResolution(16);
    pub const RESOLUTION32: EDataflowImageResolution = EDataflowImageResolution(32);
    pub const RESOLUTION64: EDataflowImageResolution = EDataflowImageResolution(64);
    pub const RESOLUTION128: EDataflowImageResolution = EDataflowImageResolution(128);
    pub const RESOLUTION256: EDataflowImageResolution = EDataflowImageResolution(256);
    pub const RESOLUTION512: EDataflowImageResolution = EDataflowImageResolution(512);
    pub const RESOLUTION1024: EDataflowImageResolution = EDataflowImageResolution(1024);
    pub const RESOLUTION2048: EDataflowImageResolution = EDataflowImageResolution(2048);
    pub const RESOLUTION4096: EDataflowImageResolution = EDataflowImageResolution(4096);
    pub const RESOLUTION8192: EDataflowImageResolution = EDataflowImageResolution(8192);
}
#[repr(transparent)]
pub struct EDataflowImageCombineResolutionOption(pub i32);
impl EDataflowImageCombineResolutionOption {
    pub const LOWEST: EDataflowImageCombineResolutionOption = EDataflowImageCombineResolutionOption(
        0,
    );
    pub const HIGHEST: EDataflowImageCombineResolutionOption = EDataflowImageCombineResolutionOption(
        1,
    );
    pub const USER_DEFINED: EDataflowImageCombineResolutionOption = EDataflowImageCombineResolutionOption(
        2,
    );
}
#[repr(transparent)]
pub struct EDataflowMathConstantsEnum(pub u8);
impl EDataflowMathConstantsEnum {
    pub const DATAFLOW_MATH_CONSTANTS_PI: EDataflowMathConstantsEnum = EDataflowMathConstantsEnum(
        0,
    );
    pub const DATAFLOW_MATH_CONSTANTS_HALF_PI: EDataflowMathConstantsEnum = EDataflowMathConstantsEnum(
        1,
    );
    pub const DATAFLOW_MATH_CONSTANTS_TWO_PI: EDataflowMathConstantsEnum = EDataflowMathConstantsEnum(
        2,
    );
    pub const DATAFLOW_MATH_CONSTANTS_FOUR_PI: EDataflowMathConstantsEnum = EDataflowMathConstantsEnum(
        3,
    );
    pub const DATAFLOW_MATH_CONSTANTS_INV_PI: EDataflowMathConstantsEnum = EDataflowMathConstantsEnum(
        4,
    );
    pub const DATAFLOW_MATH_CONSTANTS_INV_TWO_PI: EDataflowMathConstantsEnum = EDataflowMathConstantsEnum(
        5,
    );
    pub const DATAFLOW_MATH_CONSTANTS_SQRT2: EDataflowMathConstantsEnum = EDataflowMathConstantsEnum(
        6,
    );
    pub const DATAFLOW_MATH_CONSTANTS_INV_SQRT2: EDataflowMathConstantsEnum = EDataflowMathConstantsEnum(
        7,
    );
    pub const DATAFLOW_MATH_CONSTANTS_SQRT3: EDataflowMathConstantsEnum = EDataflowMathConstantsEnum(
        8,
    );
    pub const DATAFLOW_MATH_CONSTANTS_INV_SQRT3: EDataflowMathConstantsEnum = EDataflowMathConstantsEnum(
        9,
    );
    pub const DATAFLOW_MATH_CONSTANTS_E: EDataflowMathConstantsEnum = EDataflowMathConstantsEnum(
        10,
    );
    pub const DATAFLOW_MATH_CONSTANTS_GAMMA: EDataflowMathConstantsEnum = EDataflowMathConstantsEnum(
        11,
    );
    pub const DATAFLOW_MATH_CONSTANTS_GOLDEN_RATIO: EDataflowMathConstantsEnum = EDataflowMathConstantsEnum(
        12,
    );
}
