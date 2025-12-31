#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(8))]
pub struct FDataflowBaseElement {}
#[repr(C, align(1))]
pub struct FDataflowAnyType {}
#[repr(C, align(1))]
pub struct FDataflowAllTypes {}
#[repr(C, align(1))]
pub struct FDataflowArrayTypes {}
#[repr(C, align(8))]
pub struct FDataflowNumericTypes {
    pub value: f64,
}
#[repr(C, align(16))]
pub struct FDataflowVectorTypes {
    pub value: crate::bindings::core_u_object::FVector4,
}
#[repr(C, align(8))]
pub struct FDataflowStringTypes {
    pub value: FString,
}
#[repr(C, align(1))]
pub struct FDataflowBoolTypes {
    pub value: bool,
}
#[repr(C, align(16))]
pub struct FDataflowTransformTypes {
    pub value: crate::bindings::core_u_object::FTransform,
}
#[repr(C, align(8))]
pub struct FDataflowStringConvertibleTypes {
    pub value: FString,
}
#[repr(C, align(8))]
pub struct FDataflowUObjectConvertibleTypes {
    pub value: UPtr<crate::bindings::core_u_object::UObject>,
}
#[repr(C, align(8))]
pub struct FDataflowSelectionTypes {
    pub value: FDataflowSelection,
}
#[repr(C, align(8))]
pub struct FDataflowSelection {}
#[repr(C, align(8))]
pub struct FDataflowVectorArrayTypes {
    pub value: TArray<crate::bindings::core_u_object::FVector4>,
}
#[repr(C, align(8))]
pub struct FDataflowNumericArrayTypes {
    pub value: TArray<f64>,
}
#[repr(C, align(8))]
pub struct FDataflowStringArrayTypes {
    pub value: TArray<FString>,
}
#[repr(C, align(8))]
pub struct FDataflowBoolArrayTypes {
    pub value: TArray<bool>,
}
#[repr(C, align(8))]
pub struct FDataflowTransformArrayTypes {
    pub value: TArray<crate::bindings::core_u_object::FTransform>,
}
#[repr(C, align(8))]
pub struct FDataflowRotationTypes {
    pub value: crate::bindings::core_u_object::FRotator,
}
#[repr(C, align(8))]
pub struct FDataflowConnection {}
#[repr(C, align(8))]
pub struct FDataflowNode {
    pub b_override_color: bool,
    pub override_color: crate::bindings::core_u_object::FLinearColor,
    pub b_active: bool,
    pub frozen_properties: crate::bindings::core_u_object::FInstancedPropertyBag,
    pub b_is_frozen: bool,
}
#[repr(C, align(8))]
pub struct FConvertNumericTypesDataflowNode {
    pub in_: FDataflowNumericTypes,
    pub out: FDataflowNumericTypes,
}
#[repr(C, align(16))]
pub struct FConvertVectorTypesDataflowNode {
    pub in_: FDataflowVectorTypes,
    pub out: FDataflowVectorTypes,
}
#[repr(C, align(8))]
pub struct FConvertStringTypesDataflowNode {
    pub in_: FDataflowStringTypes,
    pub out: FDataflowStringTypes,
}
#[repr(C, align(8))]
pub struct FConvertBoolTypesDataflowNode {
    pub in_: FDataflowBoolTypes,
    pub out: FDataflowBoolTypes,
}
#[repr(C, align(16))]
pub struct FConvertTransformTypesDataflowNode {
    pub in_: FDataflowTransformTypes,
    pub out: FDataflowTransformTypes,
}
#[repr(C, align(8))]
pub struct FConvertStringConvertibleTypesDataflowNode {
    pub in_: FDataflowStringConvertibleTypes,
    pub out: FDataflowStringConvertibleTypes,
}
#[repr(C, align(8))]
pub struct FConvertUObjectConvertibleTypesDataflowNode {
    pub in_: FDataflowUObjectConvertibleTypes,
    pub out: FDataflowUObjectConvertibleTypes,
}
#[repr(C, align(8))]
pub struct FConvertSelectionTypesDataflowNode {
    pub collection: crate::bindings::chaos::FManagedArrayCollection,
    pub in_: FDataflowSelectionTypes,
    pub b_all_elements_must_be_selected: bool,
    pub out: FDataflowSelectionTypes,
}
#[repr(C, align(8))]
pub struct FConvertSelectionTypesToIndexArrayDataflowNode {
    pub in_: FDataflowSelectionTypes,
    pub out: TArray<i32>,
}
#[repr(C, align(8))]
pub struct FConvertIndexToSelectionTypesDataflowNode {
    pub collection: crate::bindings::chaos::FManagedArrayCollection,
    pub in_: i32,
    pub out: FDataflowSelectionTypes,
}
#[repr(C, align(8))]
pub struct FConvertIndexArrayToSelectionTypesDataflowNode {
    pub collection: crate::bindings::chaos::FManagedArrayCollection,
    pub in_: TArray<i32>,
    pub out: FDataflowSelectionTypes,
}
#[repr(C, align(8))]
pub struct FConvertVectorArrayTypesDataflowNode {
    pub in_: FDataflowVectorArrayTypes,
    pub out: FDataflowVectorArrayTypes,
}
#[repr(C, align(8))]
pub struct FConvertNumericArrayTypesDataflowNode {
    pub in_: FDataflowNumericArrayTypes,
    pub out: FDataflowNumericArrayTypes,
}
#[repr(C, align(8))]
pub struct FConvertStringArrayTypesDataflowNode {
    pub in_: FDataflowStringArrayTypes,
    pub out: FDataflowStringArrayTypes,
}
#[repr(C, align(8))]
pub struct FConvertBoolArrayTypesDataflowNode {
    pub in_: FDataflowBoolArrayTypes,
    pub out: FDataflowBoolArrayTypes,
}
#[repr(C, align(8))]
pub struct FConvertTransformArrayTypesDataflowNode {
    pub in_: FDataflowTransformArrayTypes,
    pub out: FDataflowTransformArrayTypes,
}
#[repr(C, align(8))]
pub struct FConvertRotationDataflowNode {
    pub in_: FDataflowRotationTypes,
    pub out: FDataflowRotationTypes,
}
#[repr(C, align(8))]
pub struct FDataflowReRouteNode {
    pub value: FDataflowAnyType,
}
#[repr(C, align(8))]
pub struct FDataflowBranchNode {
    pub true_value: FDataflowAnyType,
    pub false_value: FDataflowAnyType,
    pub b_condition: bool,
    pub result: FDataflowAnyType,
}
#[repr(C, align(8))]
pub struct FDataflowSelectNode {
    pub inputs: TArray<FDataflowAnyType>,
    pub selected_index: i32,
    pub result: FDataflowAnyType,
}
#[repr(C, align(8))]
pub struct FDataflowPrintNode {
    pub value: FDataflowStringConvertibleTypes,
}
#[repr(C, align(8))]
pub struct FDataflowForceDependencyNode {
    pub value: FDataflowAnyType,
    pub dependent_value: FDataflowAnyType,
}
#[repr(C, align(8))]
pub struct FDataflowImage {}
#[repr(C, align(8))]
pub struct FDataflowImageFromColorNode {
    pub fill_color: crate::bindings::core_u_object::FLinearColor,
    pub resolution: EDataflowImageResolution,
    pub image: FDataflowImage,
}
#[repr(C, align(8))]
pub struct FDataflowImageSplitChannelsNode {
    pub image: FDataflowImage,
    pub red: FDataflowImage,
    pub green: FDataflowImage,
    pub blue: FDataflowImage,
    pub alpha: FDataflowImage,
}
#[repr(C, align(8))]
pub struct FDataflowImageCombineChannelsNode {
    pub red: FDataflowImage,
    pub green: FDataflowImage,
    pub blue: FDataflowImage,
    pub alpha: FDataflowImage,
    pub image: FDataflowImage,
    pub resolution_options: EDataflowImageCombineResolutionOption,
    pub resolution: EDataflowImageResolution,
}
#[repr(C, align(8))]
pub struct FDataflowInput {}
#[repr(C, align(8))]
pub struct FDataflowArrayInput {}
#[repr(C, align(8))]
pub struct FDataflowOutput {}
#[repr(C, align(8))]
pub struct FDataflowArrayOutput {}
#[repr(C, align(8))]
pub struct FDataflowMathOneInputOperatorNode {
    pub a: FDataflowNumericTypes,
    pub result: FDataflowNumericTypes,
}
#[repr(C, align(8))]
pub struct FDataflowMathTwoInputsOperatorNode {
    pub a: FDataflowNumericTypes,
    pub b: FDataflowNumericTypes,
    pub result: FDataflowNumericTypes,
}
#[repr(C, align(8))]
pub struct FDataflowMathAddNode {}
#[repr(C, align(8))]
pub struct FDataflowMathSubtractNode {}
#[repr(C, align(8))]
pub struct FDataflowMathMultiplyNode {}
#[repr(C, align(8))]
pub struct FDataflowMathDivideNode {
    pub fallback: FDataflowNumericTypes,
}
#[repr(C, align(8))]
pub struct FDataflowMathMinimumNode {}
#[repr(C, align(8))]
pub struct FDataflowMathMinimumNode_v2 {
    pub inputs: TArray<FDataflowNumericTypes>,
    pub result: FDataflowNumericTypes,
}
#[repr(C, align(8))]
pub struct FDataflowMathMaximumNode {}
#[repr(C, align(8))]
pub struct FDataflowMathMaximumNode_v2 {
    pub inputs: TArray<FDataflowNumericTypes>,
    pub result: FDataflowNumericTypes,
}
#[repr(C, align(8))]
pub struct FDataflowMathReciprocalNode {
    pub fallback: FDataflowNumericTypes,
}
#[repr(C, align(8))]
pub struct FDataflowMathSquareNode {}
#[repr(C, align(8))]
pub struct FDataflowMathCubeNode {}
#[repr(C, align(8))]
pub struct FDataflowMathSquareRootNode {}
#[repr(C, align(8))]
pub struct FDataflowMathInverseSquareRootNode {
    pub fallback: FDataflowNumericTypes,
}
#[repr(C, align(8))]
pub struct FDataflowMathNegateNode {}
#[repr(C, align(8))]
pub struct FDataflowMathAbsNode {}
#[repr(C, align(8))]
pub struct FDataflowMathFloorNode {}
#[repr(C, align(8))]
pub struct FDataflowMathCeilNode {}
#[repr(C, align(8))]
pub struct FDataflowMathRoundNode {}
#[repr(C, align(8))]
pub struct FDataflowMathTruncNode {}
#[repr(C, align(8))]
pub struct FDataflowMathFracNode {}
#[repr(C, align(8))]
pub struct FDataflowMathPowNode {}
#[repr(C, align(8))]
pub struct FDataflowMathLogXNode {
    pub base: FDataflowNumericTypes,
}
#[repr(C, align(8))]
pub struct FDataflowMathLogNode {}
#[repr(C, align(8))]
pub struct FDataflowMathExpNode {}
#[repr(C, align(8))]
pub struct FDataflowMathSignNode {}
#[repr(C, align(8))]
pub struct FDataflowMathOneMinusNode {}
#[repr(C, align(8))]
pub struct FDataflowMathConstantNode {
    pub constant: EDataflowMathConstantsEnum,
    pub result: FDataflowNumericTypes,
}
#[repr(C, align(8))]
pub struct FDataflowMathClampNode {
    pub min: FDataflowNumericTypes,
    pub max: FDataflowNumericTypes,
}
#[repr(C, align(8))]
pub struct FDataflowMathSinNode {}
#[repr(C, align(8))]
pub struct FDataflowMathCosNode {}
#[repr(C, align(8))]
pub struct FDataflowMathTanNode {}
#[repr(C, align(8))]
pub struct FDataflowMathArcSinNode {}
#[repr(C, align(8))]
pub struct FDataflowMathArcCosNode {}
#[repr(C, align(8))]
pub struct FDataflowMathArcTanNode {}
#[repr(C, align(8))]
pub struct FDataflowMathArcTan2Node {}
#[repr(C, align(8))]
pub struct FDataflowMathDegToRadNode {}
#[repr(C, align(8))]
pub struct FDataflowMathRadToDegNode {}
#[repr(C, align(1))]
pub struct FDataflowFreezeActions {}
#[repr(C, align(8))]
pub struct FDataflowOverrideNode {
    pub key: FName,
    pub default: FString,
    pub is_overriden: bool,
}
#[repr(C, align(8))]
pub struct FDataflowPath {}
#[repr(C, align(8))]
pub struct FDataflowTransformSelection {}
#[repr(C, align(8))]
pub struct FDataflowVertexSelection {}
#[repr(C, align(8))]
pub struct FDataflowFaceSelection {}
#[repr(C, align(8))]
pub struct FDataflowGeometrySelection {}
#[repr(C, align(8))]
pub struct FDataflowMaterialSelection {}
#[repr(C, align(8))]
pub struct FDataflowCurveSelection {}
#[repr(C, align(4))]
pub struct FNodeColors {
    pub node_title_color: crate::bindings::core_u_object::FLinearColor,
    pub node_body_tint_color: crate::bindings::core_u_object::FLinearColor,
}
#[repr(C, align(4))]
pub struct FPinSettings {
    pub pin_color: crate::bindings::core_u_object::FLinearColor,
    pub wire_thickness: f32,
}
#[repr(C, align(8))]
pub struct FTransformLevelColors {
    pub level_colors: TArray<crate::bindings::core_u_object::FLinearColor>,
    pub blank_color: crate::bindings::core_u_object::FLinearColor,
}
#[repr(C, align(8))]
pub struct FDataflowTerminalNode {}
#[repr(C, align(16))]
pub struct FDataflowVectorMakeVec2Node {
    pub x: FDataflowNumericTypes,
    pub y: FDataflowNumericTypes,
    pub vector2_d: FDataflowVectorTypes,
}
#[repr(C, align(16))]
pub struct FDataflowVectorMakeVec3Node {
    pub x: FDataflowNumericTypes,
    pub y: FDataflowNumericTypes,
    pub z: FDataflowNumericTypes,
    pub vector3_d: FDataflowVectorTypes,
}
#[repr(C, align(16))]
pub struct FDataflowVectorMakeVec4Node {
    pub x: FDataflowNumericTypes,
    pub y: FDataflowNumericTypes,
    pub z: FDataflowNumericTypes,
    pub w: FDataflowNumericTypes,
    pub vector4_d: FDataflowVectorTypes,
}
#[repr(C, align(16))]
pub struct FDataflowVectorBreakNode {
    pub v: FDataflowVectorTypes,
    pub x: FDataflowNumericTypes,
    pub y: FDataflowNumericTypes,
    pub z: FDataflowNumericTypes,
    pub w: FDataflowNumericTypes,
}
#[repr(C, align(16))]
pub struct FDataflowVectorAddNode {
    pub a: FDataflowVectorTypes,
    pub b: FDataflowVectorTypes,
    pub v: FDataflowVectorTypes,
}
#[repr(C, align(16))]
pub struct FDataflowVectorSubtractNode {
    pub a: FDataflowVectorTypes,
    pub b: FDataflowVectorTypes,
    pub v: FDataflowVectorTypes,
}
#[repr(C, align(16))]
pub struct FDataflowVectorMultiplyNode {
    pub a: FDataflowVectorTypes,
    pub b: FDataflowVectorTypes,
    pub v: FDataflowVectorTypes,
}
#[repr(C, align(16))]
pub struct FDataflowVectorDivideNode {
    pub a: FDataflowVectorTypes,
    pub b: FDataflowVectorTypes,
    pub fallback: FDataflowVectorTypes,
    pub v: FDataflowVectorTypes,
}
#[repr(C, align(16))]
pub struct FDataflowVectorDotProductNode {
    pub a: FDataflowVectorTypes,
    pub b: FDataflowVectorTypes,
    pub dot_product: FDataflowNumericTypes,
}
#[repr(C, align(16))]
pub struct FDataflowVectorLengthNode {
    pub v: FDataflowVectorTypes,
    pub length: FDataflowNumericTypes,
}
#[repr(C, align(16))]
pub struct FDataflowVectorSquaredLengthNode {
    pub v: FDataflowVectorTypes,
    pub squared_length: FDataflowNumericTypes,
}
#[repr(C, align(16))]
pub struct FDataflowVectorDistanceNode {
    pub a: FDataflowVectorTypes,
    pub b: FDataflowVectorTypes,
    pub distance: FDataflowNumericTypes,
}
#[repr(C, align(16))]
pub struct FDataflowVectorCrossProductNode {
    pub a: FDataflowVectorTypes,
    pub b: FDataflowVectorTypes,
    pub cross_product: FDataflowVectorTypes,
}
#[repr(C, align(16))]
pub struct FDataflowVectorScaleNode {
    pub v: FDataflowVectorTypes,
    pub scale: FDataflowNumericTypes,
    pub scaled: FDataflowVectorTypes,
}
#[repr(C, align(16))]
pub struct FDataflowVectorNormalize {
    pub v: FDataflowVectorTypes,
    pub normalized: FDataflowVectorTypes,
}
pub struct UDataflowGraphInterface {}
pub struct IDataflowGraphInterface {}
pub struct UDataflowSettings {
    pub node_colors_map: TMap<FName, FNodeColors>,
    pub pin_settings_map: TMap<FName, FPinSettings>,
    pub transform_level_colors: FTransformLevelColors,
}
#[allow(non_camel_case_types)]
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
#[allow(non_camel_case_types)]
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
#[allow(non_camel_case_types)]
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
