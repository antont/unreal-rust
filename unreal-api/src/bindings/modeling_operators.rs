#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
pub struct URecomputeUVsToolProperties {
    pub b_enable_polygroup_support: bool,
    pub island_generation: ERecomputeUVsPropertiesIslandMode,
    pub unwrap_type: ERecomputeUVsPropertiesUnwrapType,
    pub auto_rotation: ERecomputeUVsToolOrientationMode,
    pub b_preserve_irregularity: bool,
    pub smoothing_steps: i32,
    pub smoothing_alpha: f32,
    pub merging_distortion_threshold: f32,
    pub merging_angle_threshold: f32,
    pub layout_type: ERecomputeUVsPropertiesLayoutType,
    pub texture_resolution: i32,
    pub normalize_scale: f32,
    pub b_enable_udim_layout: bool,
    pub b_udimcvar_enabled: bool,
}
pub struct UUVLayoutProperties {
    pub layout_type: EUVLayoutType,
    pub texture_resolution: i32,
    pub scale: f32,
    pub translation: crate::bindings::core_u_object::FVector2D,
    pub b_preserve_scale: bool,
    pub b_preserve_rotation: bool,
    pub b_allow_flips: bool,
    pub b_enable_udim_layout: bool,
    pub b_udimcvar_enabled: bool,
}
pub struct UGenerateCrossSectionOpFactory {}
pub struct URecomputeUVsOpFactory {
    pub settings: UPtr<URecomputeUVsToolProperties>,
}
pub struct UUVEditorTexelDensitySettings {
    pub texel_density_mode: ETexelDensityToolMode,
    pub target_world_units: f32,
    pub target_pixel_count: f32,
    pub texture_resolution: f32,
    pub b_enable_udim_layout: bool,
}
pub struct UUVTexelDensityOperatorFactory {
    pub settings: UPtr<UUVEditorTexelDensitySettings>,
}
pub struct UUVLayoutOperatorFactory {
    pub settings: UPtr<UUVLayoutProperties>,
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ERecomputeUVsPropertiesIslandMode(pub i32);
impl ERecomputeUVsPropertiesIslandMode {
    pub const POLY_GROUPS: ERecomputeUVsPropertiesIslandMode = ERecomputeUVsPropertiesIslandMode(
        0,
    );
    pub const EXISTING_U_VS: ERecomputeUVsPropertiesIslandMode = ERecomputeUVsPropertiesIslandMode(
        1,
    );
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ERecomputeUVsPropertiesUnwrapType(pub i32);
impl ERecomputeUVsPropertiesUnwrapType {
    pub const EXP_MAP: ERecomputeUVsPropertiesUnwrapType = ERecomputeUVsPropertiesUnwrapType(
        0,
    );
    pub const CONFORMAL: ERecomputeUVsPropertiesUnwrapType = ERecomputeUVsPropertiesUnwrapType(
        1,
    );
    pub const SPECTRAL_CONFORMAL: ERecomputeUVsPropertiesUnwrapType = ERecomputeUVsPropertiesUnwrapType(
        2,
    );
    pub const ISLAND_MERGING: ERecomputeUVsPropertiesUnwrapType = ERecomputeUVsPropertiesUnwrapType(
        3,
    );
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ERecomputeUVsToolOrientationMode(pub i32);
impl ERecomputeUVsToolOrientationMode {
    pub const NONE: ERecomputeUVsToolOrientationMode = ERecomputeUVsToolOrientationMode(
        0,
    );
    pub const MIN_BOUNDS: ERecomputeUVsToolOrientationMode = ERecomputeUVsToolOrientationMode(
        1,
    );
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ERecomputeUVsPropertiesLayoutType(pub i32);
impl ERecomputeUVsPropertiesLayoutType {
    pub const NONE: ERecomputeUVsPropertiesLayoutType = ERecomputeUVsPropertiesLayoutType(
        0,
    );
    pub const REPACK: ERecomputeUVsPropertiesLayoutType = ERecomputeUVsPropertiesLayoutType(
        1,
    );
    pub const NORMALIZE_TO_EXISTING_BOUNDS: ERecomputeUVsPropertiesLayoutType = ERecomputeUVsPropertiesLayoutType(
        2,
    );
    pub const NORMALIZE_TO_BOUNDS: ERecomputeUVsPropertiesLayoutType = ERecomputeUVsPropertiesLayoutType(
        3,
    );
    pub const NORMALIZE_TO_WORLD: ERecomputeUVsPropertiesLayoutType = ERecomputeUVsPropertiesLayoutType(
        4,
    );
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EUVLayoutType(pub i32);
impl EUVLayoutType {
    pub const TRANSFORM: EUVLayoutType = EUVLayoutType(0);
    pub const STACK: EUVLayoutType = EUVLayoutType(1);
    pub const REPACK: EUVLayoutType = EUVLayoutType(2);
    pub const NORMALIZE: EUVLayoutType = EUVLayoutType(3);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ETexelDensityToolMode(pub i32);
impl ETexelDensityToolMode {
    pub const APPLY_TO_ISLANDS: ETexelDensityToolMode = ETexelDensityToolMode(0);
    pub const APPLY_TO_WHOLE: ETexelDensityToolMode = ETexelDensityToolMode(1);
    pub const NORMALIZE: ETexelDensityToolMode = ETexelDensityToolMode(2);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ECSGOperation(pub u8);
impl ECSGOperation {
    pub const DIFFERENCE_AB: ECSGOperation = ECSGOperation(0);
    pub const DIFFERENCE_BA: ECSGOperation = ECSGOperation(1);
    pub const INTERSECT: ECSGOperation = ECSGOperation(2);
    pub const UNION: ECSGOperation = ECSGOperation(3);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ETrimOperation(pub u8);
impl ETrimOperation {
    pub const TRIM_A: ETrimOperation = ETrimOperation(0);
    pub const TRIM_B: ETrimOperation = ETrimOperation(1);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ETrimSide(pub u8);
impl ETrimSide {
    pub const REMOVE_INSIDE: ETrimSide = ETrimSide(0);
    pub const REMOVE_OUTSIDE: ETrimSide = ETrimSide(1);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EHoleFillOpFillType(pub u8);
impl EHoleFillOpFillType {
    pub const TRIANGLE_FAN: EHoleFillOpFillType = EHoleFillOpFillType(0);
    pub const POLYGON_EAR_CLIPPING: EHoleFillOpFillType = EHoleFillOpFillType(1);
    pub const PLANAR: EHoleFillOpFillType = EHoleFillOpFillType(2);
    pub const MINIMAL: EHoleFillOpFillType = EHoleFillOpFillType(3);
    pub const SMOOTH: EHoleFillOpFillType = EHoleFillOpFillType(4);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ERemeshSmoothingType(pub u8);
impl ERemeshSmoothingType {
    pub const UNIFORM: ERemeshSmoothingType = ERemeshSmoothingType(0);
    pub const COTANGENT: ERemeshSmoothingType = ERemeshSmoothingType(1);
    pub const MEAN_VALUE: ERemeshSmoothingType = ERemeshSmoothingType(2);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ERemeshType(pub u8);
impl ERemeshType {
    pub const STANDARD: ERemeshType = ERemeshType(0);
    pub const FULL_PASS: ERemeshType = ERemeshType(1);
    pub const NORMAL_FLOW: ERemeshType = ERemeshType(2);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EUVProjectionMethod(pub u8);
impl EUVProjectionMethod {
    pub const BOX: EUVProjectionMethod = EUVProjectionMethod(0);
    pub const CYLINDER: EUVProjectionMethod = EUVProjectionMethod(1);
    pub const PLANE: EUVProjectionMethod = EUVProjectionMethod(2);
    pub const EXP_MAP: EUVProjectionMethod = EUVProjectionMethod(3);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ENormalCalculationMethod(pub u8);
impl ENormalCalculationMethod {
    pub const AREA_WEIGHTED: ENormalCalculationMethod = ENormalCalculationMethod(0);
    pub const ANGLE_WEIGHTED: ENormalCalculationMethod = ENormalCalculationMethod(1);
    pub const AREA_ANGLE_WEIGHTING: ENormalCalculationMethod = ENormalCalculationMethod(
        2,
    );
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ESplitNormalMethod(pub u8);
impl ESplitNormalMethod {
    pub const USE_EXISTING_TOPOLOGY: ESplitNormalMethod = ESplitNormalMethod(0);
    pub const FACE_NORMAL_THRESHOLD: ESplitNormalMethod = ESplitNormalMethod(1);
    pub const FACE_GROUP_ID: ESplitNormalMethod = ESplitNormalMethod(2);
    pub const PER_TRIANGLE: ESplitNormalMethod = ESplitNormalMethod(3);
    pub const PER_VERTEX: ESplitNormalMethod = ESplitNormalMethod(4);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EFlattenCurveMethod(pub u8);
impl EFlattenCurveMethod {
    pub const DO_NOT_FLATTEN: EFlattenCurveMethod = EFlattenCurveMethod(0);
    pub const TO_BEST_FIT_PLANE: EFlattenCurveMethod = EFlattenCurveMethod(1);
    pub const ALONG_X: EFlattenCurveMethod = EFlattenCurveMethod(2);
    pub const ALONG_Y: EFlattenCurveMethod = EFlattenCurveMethod(3);
    pub const ALONG_Z: EFlattenCurveMethod = EFlattenCurveMethod(4);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ECombineCurvesMethod(pub u8);
impl ECombineCurvesMethod {
    pub const LEAVE_SEPARATE: ECombineCurvesMethod = ECombineCurvesMethod(0);
    pub const UNION: ECombineCurvesMethod = ECombineCurvesMethod(1);
    pub const INTERSECT: ECombineCurvesMethod = ECombineCurvesMethod(2);
    pub const DIFFERENCE: ECombineCurvesMethod = ECombineCurvesMethod(3);
    pub const EXCLUSIVE_OR: ECombineCurvesMethod = ECombineCurvesMethod(4);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EOffsetOpenCurvesMethod(pub u8);
impl EOffsetOpenCurvesMethod {
    pub const TREAT_AS_CLOSED: EOffsetOpenCurvesMethod = EOffsetOpenCurvesMethod(0);
    pub const OFFSET: EOffsetOpenCurvesMethod = EOffsetOpenCurvesMethod(1);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EOffsetClosedCurvesMethod(pub u8);
impl EOffsetClosedCurvesMethod {
    pub const DO_NOT_OFFSET: EOffsetClosedCurvesMethod = EOffsetClosedCurvesMethod(0);
    pub const OFFSET_OUTER_SIDE: EOffsetClosedCurvesMethod = EOffsetClosedCurvesMethod(
        1,
    );
    pub const OFFSET_BOTH_SIDES: EOffsetClosedCurvesMethod = EOffsetClosedCurvesMethod(
        2,
    );
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EOpenCurveEndShapes(pub u8);
impl EOpenCurveEndShapes {
    pub const SQUARE: EOpenCurveEndShapes = EOpenCurveEndShapes(0);
    pub const ROUND: EOpenCurveEndShapes = EOpenCurveEndShapes(1);
    pub const BUTT: EOpenCurveEndShapes = EOpenCurveEndShapes(2);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EOffsetJoinMethod(pub u8);
impl EOffsetJoinMethod {
    pub const SQUARE: EOffsetJoinMethod = EOffsetJoinMethod(0);
    pub const MITER: EOffsetJoinMethod = EOffsetJoinMethod(1);
    pub const ROUND: EOffsetJoinMethod = EOffsetJoinMethod(2);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EMorphologyOperation(pub u8);
impl EMorphologyOperation {
    pub const DILATE: EMorphologyOperation = EMorphologyOperation(0);
    pub const CONTRACT: EMorphologyOperation = EMorphologyOperation(1);
    pub const CLOSE: EMorphologyOperation = EMorphologyOperation(2);
    pub const OPEN: EMorphologyOperation = EMorphologyOperation(3);
}
