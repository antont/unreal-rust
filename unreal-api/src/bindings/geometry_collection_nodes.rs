#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(clippy::non_camel_case_types)]
#![allow(clippy::new_without_default)]
#![allow(clippy::new_ret_no_self)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(transparent)]
pub struct EFloatArrayToIntArrayFunctionEnum(pub u8);
impl EFloatArrayToIntArrayFunctionEnum {
    pub const DATAFLOW_FLOAT_TO_INT_FUNCTION_FLOOR: EFloatArrayToIntArrayFunctionEnum = EFloatArrayToIntArrayFunctionEnum(
        0,
    );
    pub const DATAFLOW_FLOAT_TO_INT_FUNCTION_CEIL: EFloatArrayToIntArrayFunctionEnum = EFloatArrayToIntArrayFunctionEnum(
        1,
    );
    pub const DATAFLOW_FLOAT_TO_INT_FUNCTION_ROUND: EFloatArrayToIntArrayFunctionEnum = EFloatArrayToIntArrayFunctionEnum(
        2,
    );
    pub const DATAFLOW_FLOAT_TO_INT_FUNCTION_TRUNCATE: EFloatArrayToIntArrayFunctionEnum = EFloatArrayToIntArrayFunctionEnum(
        3,
    );
    pub const DATAFLOW_FLOAT_TO_INT_NON_ZERO_TO_INDEX: EFloatArrayToIntArrayFunctionEnum = EFloatArrayToIntArrayFunctionEnum(
        4,
    );
    pub const DATAFLOW_FLOAT_TO_INT_ZERO_TO_INDEX: EFloatArrayToIntArrayFunctionEnum = EFloatArrayToIntArrayFunctionEnum(
        5,
    );
}
#[repr(transparent)]
pub struct ECompareOperation1Enum(pub u8);
impl ECompareOperation1Enum {
    pub const DATAFLOW_COMPARE_EQUAL: ECompareOperation1Enum = ECompareOperation1Enum(0);
    pub const DATAFLOW_COMPARE_SMALLER: ECompareOperation1Enum = ECompareOperation1Enum(
        1,
    );
    pub const DATAFLOW_COMPARE_SMALLER_OR_EQUAL: ECompareOperation1Enum = ECompareOperation1Enum(
        2,
    );
    pub const DATAFLOW_COMPARE_GREATER: ECompareOperation1Enum = ECompareOperation1Enum(
        3,
    );
    pub const DATAFLOW_COMPARE_GREATER_OR_EQUAL: ECompareOperation1Enum = ECompareOperation1Enum(
        4,
    );
}
#[repr(transparent)]
pub struct EStatisticsOperationEnum(pub u8);
impl EStatisticsOperationEnum {
    pub const DATAFLOW_E_STATISTICS_OPERATION_ENUM_MIN: EStatisticsOperationEnum = EStatisticsOperationEnum(
        0,
    );
    pub const DATAFLOW_E_STATISTICS_OPERATION_ENUM_MEAN: EStatisticsOperationEnum = EStatisticsOperationEnum(
        2,
    );
    pub const DATAFLOW_E_STATISTICS_OPERATION_ENUM_MEDIAN: EStatisticsOperationEnum = EStatisticsOperationEnum(
        3,
    );
    pub const DATAFLOW_E_STATISTICS_OPERATION_ENUM_MODE: EStatisticsOperationEnum = EStatisticsOperationEnum(
        4,
    );
    pub const DATAFLOW_E_STATISTICS_OPERATION_ENUM_SUM: EStatisticsOperationEnum = EStatisticsOperationEnum(
        5,
    );
}
#[repr(transparent)]
pub struct EClusterSizeMethodEnum(pub u8);
impl EClusterSizeMethodEnum {
    pub const DATAFLOW_CLUSTER_SIZE_METHOD_BY_NUMBER: EClusterSizeMethodEnum = EClusterSizeMethodEnum(
        0,
    );
    pub const DATAFLOW_CLUSTER_SIZE_METHOD_BY_FRACTION_OF_INPUT: EClusterSizeMethodEnum = EClusterSizeMethodEnum(
        1,
    );
    pub const DATAFLOW_CLUSTER_SIZE_METHOD_BY_SIZE: EClusterSizeMethodEnum = EClusterSizeMethodEnum(
        2,
    );
    pub const DATAFLOW_CLUSTER_SIZE_METHOD_BY_GRID: EClusterSizeMethodEnum = EClusterSizeMethodEnum(
        3,
    );
}
#[repr(transparent)]
pub struct EClusterNeighborSelectionMethodEnum(pub u8);
impl EClusterNeighborSelectionMethodEnum {
    pub const DATAFLOW_CLUSTER_NEIGHBOR_SELECTION_METHOD_LARGEST_NEIGHBOR: EClusterNeighborSelectionMethodEnum = EClusterNeighborSelectionMethodEnum(
        0,
    );
    pub const DATAFLOW_CLUSTER_NEIGHBOR_SELECTION_METHOD_NEAREST_CENTER: EClusterNeighborSelectionMethodEnum = EClusterNeighborSelectionMethodEnum(
        1,
    );
}
#[repr(transparent)]
pub struct EFloatToIntFunctionEnum(pub u8);
impl EFloatToIntFunctionEnum {
    pub const DATAFLOW_FLOAT_TO_INT_FUNCTION_FLOOR: EFloatToIntFunctionEnum = EFloatToIntFunctionEnum(
        0,
    );
    pub const DATAFLOW_FLOAT_TO_INT_FUNCTION_CEIL: EFloatToIntFunctionEnum = EFloatToIntFunctionEnum(
        1,
    );
    pub const DATAFLOW_FLOAT_TO_INT_FUNCTION_ROUND: EFloatToIntFunctionEnum = EFloatToIntFunctionEnum(
        2,
    );
    pub const DATAFLOW_FLOAT_TO_INT_FUNCTION_TRUNCATE: EFloatToIntFunctionEnum = EFloatToIntFunctionEnum(
        3,
    );
}
#[repr(transparent)]
pub struct EVisibiltyOptionsEnum(pub u8);
impl EVisibiltyOptionsEnum {
    pub const DATAFLOW_VISIBILITY_OPTIONS_VISIBLE: EVisibiltyOptionsEnum = EVisibiltyOptionsEnum(
        0,
    );
    pub const DATAFLOW_VISIBILITY_OPTIONS_INVISIBLE: EVisibiltyOptionsEnum = EVisibiltyOptionsEnum(
        1,
    );
}
#[repr(transparent)]
pub struct EDataflowFieldFalloffType(pub u8);
impl EDataflowFieldFalloffType {
    pub const DATAFLOW_FIELD_FALLOFF_TYPE_NONE: EDataflowFieldFalloffType = EDataflowFieldFalloffType(
        0,
    );
    pub const DATAFLOW_FIELD_FALLOFF_TYPE_LINEAR: EDataflowFieldFalloffType = EDataflowFieldFalloffType(
        1,
    );
    pub const DATAFLOW_FIELD_FALLOFF_TYPE_INVERSE: EDataflowFieldFalloffType = EDataflowFieldFalloffType(
        2,
    );
    pub const DATAFLOW_FIELD_FALLOFF_TYPE_SQUARED: EDataflowFieldFalloffType = EDataflowFieldFalloffType(
        3,
    );
    pub const DATAFLOW_FIELD_FALLOFF_TYPE_LOGARITHMIC: EDataflowFieldFalloffType = EDataflowFieldFalloffType(
        4,
    );
}
#[repr(transparent)]
pub struct EDataflowSetMaskConditionType(pub u8);
impl EDataflowSetMaskConditionType {
    pub const DATAFLOW_SET_MASK_CONDITION_TYPE_ALWAYS: EDataflowSetMaskConditionType = EDataflowSetMaskConditionType(
        0,
    );
    pub const DATAFLOW_SET_MASK_CONDITION_TYPE_IFF_NOT_INTERIOR: EDataflowSetMaskConditionType = EDataflowSetMaskConditionType(
        1,
    );
    pub const DATAFLOW_SET_MASK_CONDITION_TYPE_IFF_NOT_EXTERIOR: EDataflowSetMaskConditionType = EDataflowSetMaskConditionType(
        2,
    );
}
#[repr(transparent)]
pub struct EDataflowWaveFunctionType(pub u8);
impl EDataflowWaveFunctionType {
    pub const DATAFLOW_WAVE_FUNCTION_TYPE_COSINE: EDataflowWaveFunctionType = EDataflowWaveFunctionType(
        0,
    );
    pub const DATAFLOW_WAVE_FUNCTION_TYPE_GAUSSIAN: EDataflowWaveFunctionType = EDataflowWaveFunctionType(
        1,
    );
    pub const DATAFLOW_WAVE_FUNCTION_TYPE_FALLOFF: EDataflowWaveFunctionType = EDataflowWaveFunctionType(
        2,
    );
    pub const DATAFLOW_WAVE_FUNCTION_TYPE_DECAY: EDataflowWaveFunctionType = EDataflowWaveFunctionType(
        3,
    );
}
#[repr(transparent)]
pub struct EDataflowFloatFieldOperationType(pub u8);
impl EDataflowFloatFieldOperationType {
    pub const DATAFLOW_FLOAT_FIELD_OPERATION_TYPE_MULTIPLY: EDataflowFloatFieldOperationType = EDataflowFloatFieldOperationType(
        0,
    );
    pub const DATAFLOW_FLOAT_FIELD_FALLOFF_TYPE_DIVIDE: EDataflowFloatFieldOperationType = EDataflowFloatFieldOperationType(
        1,
    );
    pub const DATAFLOW_FLOAT_FIELD_FALLOFF_TYPE_ADD: EDataflowFloatFieldOperationType = EDataflowFloatFieldOperationType(
        2,
    );
    pub const DATAFLOW_FLOAT_FIELD_FALLOFF_TYPE_SUBSTRACT: EDataflowFloatFieldOperationType = EDataflowFloatFieldOperationType(
        3,
    );
    pub const DATAFLOW_FLOAT_FIELD_FALLOFF_TYPE_MIN: EDataflowFloatFieldOperationType = EDataflowFloatFieldOperationType(
        4,
    );
}
#[repr(transparent)]
pub struct EDataflowVectorFieldOperationType(pub u8);
impl EDataflowVectorFieldOperationType {
    pub const DATAFLOW_VECTOR_FIELD_OPERATION_TYPE_MULTIPLY: EDataflowVectorFieldOperationType = EDataflowVectorFieldOperationType(
        0,
    );
    pub const DATAFLOW_VECTOR_FIELD_FALLOFF_TYPE_DIVIDE: EDataflowVectorFieldOperationType = EDataflowVectorFieldOperationType(
        1,
    );
    pub const DATAFLOW_VECTOR_FIELD_FALLOFF_TYPE_ADD: EDataflowVectorFieldOperationType = EDataflowVectorFieldOperationType(
        2,
    );
    pub const DATAFLOW_VECTOR_FIELD_FALLOFF_TYPE_SUBSTRACT: EDataflowVectorFieldOperationType = EDataflowVectorFieldOperationType(
        3,
    );
    pub const DATAFLOW_VECTOR_FIELD_FALLOFF_TYPE_CROSS_PRODUCT: EDataflowVectorFieldOperationType = EDataflowVectorFieldOperationType(
        4,
    );
}
#[repr(transparent)]
pub struct EDataflowVisualizeFractureColoringType(pub u8);
impl EDataflowVisualizeFractureColoringType {
    pub const COLOR_BY_PARENT: EDataflowVisualizeFractureColoringType = EDataflowVisualizeFractureColoringType(
        0,
    );
    pub const COLOR_BY_LEVEL: EDataflowVisualizeFractureColoringType = EDataflowVisualizeFractureColoringType(
        1,
    );
    pub const COLOR_BY_CLUSTER: EDataflowVisualizeFractureColoringType = EDataflowVisualizeFractureColoringType(
        2,
    );
    pub const COLOR_BY_LEAF_LEVEL: EDataflowVisualizeFractureColoringType = EDataflowVisualizeFractureColoringType(
        3,
    );
    pub const COLOR_BY_LEAF: EDataflowVisualizeFractureColoringType = EDataflowVisualizeFractureColoringType(
        4,
    );
    pub const COLOR_BY_ATTR: EDataflowVisualizeFractureColoringType = EDataflowVisualizeFractureColoringType(
        5,
    );
}
#[repr(transparent)]
pub struct EDataflowSetFloatArrayMethod(pub u8);
impl EDataflowSetFloatArrayMethod {
    pub const RANDOM: EDataflowSetFloatArrayMethod = EDataflowSetFloatArrayMethod(0);
    pub const NOISE: EDataflowSetFloatArrayMethod = EDataflowSetFloatArrayMethod(1);
    pub const BY_BOUNDING_BOX: EDataflowSetFloatArrayMethod = EDataflowSetFloatArrayMethod(
        2,
    );
}
#[repr(transparent)]
pub struct EMakeBoxDataTypeEnum(pub u8);
impl EMakeBoxDataTypeEnum {
    pub const DATAFLOW_MAKE_BOX_DATA_TYPE_MIN_MAX: EMakeBoxDataTypeEnum = EMakeBoxDataTypeEnum(
        0,
    );
    pub const DATAFLOW_MAKE_BOX_DATA_TYPE_CENTER_SIZE: EMakeBoxDataTypeEnum = EMakeBoxDataTypeEnum(
        1,
    );
}
#[repr(transparent)]
pub struct EDataflowStairTypeEnum(pub u8);
impl EDataflowStairTypeEnum {
    pub const LINEAR: EDataflowStairTypeEnum = EDataflowStairTypeEnum(0);
    pub const FLOATING: EDataflowStairTypeEnum = EDataflowStairTypeEnum(1);
    pub const CURVED: EDataflowStairTypeEnum = EDataflowStairTypeEnum(2);
    pub const SPIRAL: EDataflowStairTypeEnum = EDataflowStairTypeEnum(3);
}
#[repr(transparent)]
pub struct ESetMaterialOperationTypeEnum(pub u8);
impl ESetMaterialOperationTypeEnum {
    pub const DATAFLOW_SET_MATERIAL_OPERATION_TYPE_ADD: ESetMaterialOperationTypeEnum = ESetMaterialOperationTypeEnum(
        0,
    );
    pub const DATAFLOW_SET_MATERIAL_OPERATION_TYPE_INSERT: ESetMaterialOperationTypeEnum = ESetMaterialOperationTypeEnum(
        1,
    );
}
#[repr(transparent)]
pub struct EMathConstantsEnum(pub u8);
impl EMathConstantsEnum {
    pub const DATAFLOW_MATH_CONSTANTS_PI: EMathConstantsEnum = EMathConstantsEnum(0);
    pub const DATAFLOW_MATH_CONSTANTS_HALF_PI: EMathConstantsEnum = EMathConstantsEnum(
        1,
    );
    pub const DATAFLOW_MATH_CONSTANTS_TWO_PI: EMathConstantsEnum = EMathConstantsEnum(2);
    pub const DATAFLOW_MATH_CONSTANTS_FOUR_PI: EMathConstantsEnum = EMathConstantsEnum(
        3,
    );
    pub const DATAFLOW_MATH_CONSTANTS_INV_PI: EMathConstantsEnum = EMathConstantsEnum(4);
    pub const DATAFLOW_MATH_CONSTANTS_INV_TWO_PI: EMathConstantsEnum = EMathConstantsEnum(
        5,
    );
    pub const DATAFLOW_MATH_CONSTANTS_SQRT2: EMathConstantsEnum = EMathConstantsEnum(6);
    pub const DATAFLOW_MATH_CONSTANTS_INV_SQRT2: EMathConstantsEnum = EMathConstantsEnum(
        7,
    );
    pub const DATAFLOW_MATH_CONSTANTS_SQRT3: EMathConstantsEnum = EMathConstantsEnum(8);
    pub const DATAFLOW_MATH_CONSTANTS_INV_SQRT3: EMathConstantsEnum = EMathConstantsEnum(
        9,
    );
    pub const DATAFLOW_FLOAT_TO_INT_FUNCTION_E: EMathConstantsEnum = EMathConstantsEnum(
        10,
    );
    pub const DATAFLOW_FLOAT_TO_INT_FUNCTION_GAMMA: EMathConstantsEnum = EMathConstantsEnum(
        11,
    );
    pub const DATAFLOW_FLOAT_TO_INT_FUNCTION_GOLDEN_RATIO: EMathConstantsEnum = EMathConstantsEnum(
        12,
    );
    pub const DATAFLOW_FLOAT_TO_INT_FUNCTION_ZERO_TOLERANCE: EMathConstantsEnum = EMathConstantsEnum(
        13,
    );
}
#[repr(transparent)]
pub struct EDataflowMeshSplitIslandsMethod(pub u8);
impl EDataflowMeshSplitIslandsMethod {
    pub const NO_SPLIT: EDataflowMeshSplitIslandsMethod = EDataflowMeshSplitIslandsMethod(
        0,
    );
    pub const BY_MESH_TOPOLOGY: EDataflowMeshSplitIslandsMethod = EDataflowMeshSplitIslandsMethod(
        1,
    );
    pub const BY_VERTEX_OVERLAP: EDataflowMeshSplitIslandsMethod = EDataflowMeshSplitIslandsMethod(
        2,
    );
}
#[repr(transparent)]
pub struct EBoxLengthMeasurementMethod(pub u8);
impl EBoxLengthMeasurementMethod {
    pub const X_AXIS: EBoxLengthMeasurementMethod = EBoxLengthMeasurementMethod(0);
    pub const Y_AXIS: EBoxLengthMeasurementMethod = EBoxLengthMeasurementMethod(1);
    pub const Z_AXIS: EBoxLengthMeasurementMethod = EBoxLengthMeasurementMethod(2);
    pub const SHORTEST_AXIS: EBoxLengthMeasurementMethod = EBoxLengthMeasurementMethod(
        3,
    );
    pub const LONGEST_AXIS: EBoxLengthMeasurementMethod = EBoxLengthMeasurementMethod(4);
    pub const DIAGONAL: EBoxLengthMeasurementMethod = EBoxLengthMeasurementMethod(5);
}
#[repr(transparent)]
pub struct ERotationOrderEnum(pub u8);
impl ERotationOrderEnum {
    pub const DATAFLOW_ROTATION_ORDER_XYZ: ERotationOrderEnum = ERotationOrderEnum(0);
    pub const DATAFLOW_ROTATION_ORDER_YZX: ERotationOrderEnum = ERotationOrderEnum(1);
    pub const DATAFLOW_ROTATION_ORDER_ZXY: ERotationOrderEnum = ERotationOrderEnum(2);
    pub const DATAFLOW_ROTATION_ORDER_XZY: ERotationOrderEnum = ERotationOrderEnum(3);
    pub const DATAFLOW_ROTATION_ORDER_YXZ: ERotationOrderEnum = ERotationOrderEnum(4);
    pub const DATAFLOW_ROTATION_ORDER_ZYX: ERotationOrderEnum = ERotationOrderEnum(5);
}
#[repr(transparent)]
pub struct ECompareOperationEnum(pub u8);
impl ECompareOperationEnum {
    pub const DATAFLOW_COMPARE_EQUAL: ECompareOperationEnum = ECompareOperationEnum(0);
    pub const DATAFLOW_COMPARE_SMALLER: ECompareOperationEnum = ECompareOperationEnum(1);
    pub const DATAFLOW_COMPARE_SMALLER_OR_EQUAL: ECompareOperationEnum = ECompareOperationEnum(
        2,
    );
    pub const DATAFLOW_COMPARE_GREATER: ECompareOperationEnum = ECompareOperationEnum(3);
    pub const DATAFLOW_COMPARE_GREATER_OR_EQUAL: ECompareOperationEnum = ECompareOperationEnum(
        4,
    );
    pub const DATAFLOW_COMPARE_NOT_EQUAL: ECompareOperationEnum = ECompareOperationEnum(
        5,
    );
}
#[repr(transparent)]
pub struct EBooleanOperationEnum(pub u8);
impl EBooleanOperationEnum {
    pub const DATAFLOW_AND: EBooleanOperationEnum = EBooleanOperationEnum(0);
    pub const DATAFLOW_OR: EBooleanOperationEnum = EBooleanOperationEnum(1);
    pub const DATAFLOW_NOT: EBooleanOperationEnum = EBooleanOperationEnum(2);
}
#[repr(transparent)]
pub struct EAnchorStateEnum(pub u8);
impl EAnchorStateEnum {
    pub const DATAFLOW_ANCHOR_STATE_ANCHORED: EAnchorStateEnum = EAnchorStateEnum(0);
    pub const DATAFLOW_ANCHOR_STATE_NOT_ANCHORED: EAnchorStateEnum = EAnchorStateEnum(1);
}
#[repr(transparent)]
pub struct EDataflowGeometryCollectionDynamicState(pub u8);
impl EDataflowGeometryCollectionDynamicState {
    pub const NONE: EDataflowGeometryCollectionDynamicState = EDataflowGeometryCollectionDynamicState(
        0,
    );
    pub const DYNAMIC: EDataflowGeometryCollectionDynamicState = EDataflowGeometryCollectionDynamicState(
        1,
    );
    pub const KINEMATIC: EDataflowGeometryCollectionDynamicState = EDataflowGeometryCollectionDynamicState(
        2,
    );
    pub const STATIC: EDataflowGeometryCollectionDynamicState = EDataflowGeometryCollectionDynamicState(
        3,
    );
}
#[repr(transparent)]
pub struct EProximityMethodEnum(pub u8);
impl EProximityMethodEnum {
    pub const DATAFLOW_PROXIMITY_METHOD_PRECISE: EProximityMethodEnum = EProximityMethodEnum(
        0,
    );
    pub const DATAFLOW_PROXIMITY_METHOD_CONVEX_HULL: EProximityMethodEnum = EProximityMethodEnum(
        1,
    );
}
#[repr(transparent)]
pub struct EProximityContactFilteringMethodEnum(pub u8);
impl EProximityContactFilteringMethodEnum {
    pub const DATAFLOW_PROXIMITY_CONTACT_FILTERING_METHOD_PROJECTED_BOUNDS_OVERLAP: EProximityContactFilteringMethodEnum = EProximityContactFilteringMethodEnum(
        0,
    );
    pub const DATAFLOW_PROXIMITY_CONTACT_FILTERING_METHOD_CONVEX_HULL_SHARP: EProximityContactFilteringMethodEnum = EProximityContactFilteringMethodEnum(
        1,
    );
    pub const DATAFLOW_PROXIMITY_CONTACT_FILTERING_METHOD_CONVEX_HULL_AREA: EProximityContactFilteringMethodEnum = EProximityContactFilteringMethodEnum(
        2,
    );
}
#[repr(transparent)]
pub struct EConnectionContactAreaMethodEnum(pub u8);
impl EConnectionContactAreaMethodEnum {
    pub const DATAFLOW_CONNECTION_CONTACT_AREA_METHOD_NONE: EConnectionContactAreaMethodEnum = EConnectionContactAreaMethodEnum(
        0,
    );
    pub const DATAFLOW_PROXIMITY_CONTACT_FILTERING_METHOD_CONVEX_HULL_AREA: EConnectionContactAreaMethodEnum = EConnectionContactAreaMethodEnum(
        1,
    );
}
#[repr(transparent)]
pub struct EStandardGroupNameEnum(pub u8);
impl EStandardGroupNameEnum {
    pub const DATAFLOW_E_STANDARD_GROUP_NAME_ENUM_TRANSFORM: EStandardGroupNameEnum = EStandardGroupNameEnum(
        0,
    );
    pub const DATAFLOW_E_STANDARD_GROUP_NAME_ENUM_GEOMETRY: EStandardGroupNameEnum = EStandardGroupNameEnum(
        1,
    );
    pub const DATAFLOW_E_STANDARD_GROUP_NAME_ENUM_FACES: EStandardGroupNameEnum = EStandardGroupNameEnum(
        2,
    );
    pub const DATAFLOW_E_STANDARD_GROUP_NAME_ENUM_VERTICES: EStandardGroupNameEnum = EStandardGroupNameEnum(
        3,
    );
    pub const DATAFLOW_E_STANDARD_GROUP_NAME_ENUM_MATERIAL: EStandardGroupNameEnum = EStandardGroupNameEnum(
        4,
    );
    pub const DATAFLOW_E_STANDARD_GROUP_NAME_ENUM_BREAKING: EStandardGroupNameEnum = EStandardGroupNameEnum(
        5,
    );
    pub const DATAFLOW_E_STANDARD_GROUP_NAME_ENUM_CUSTOM: EStandardGroupNameEnum = EStandardGroupNameEnum(
        6,
    );
}
#[repr(transparent)]
pub struct ECustomAttributeTypeEnum(pub u8);
impl ECustomAttributeTypeEnum {
    pub const DATAFLOW_CUSTOM_ATTRIBUTE_TYPE_U_INT8: ECustomAttributeTypeEnum = ECustomAttributeTypeEnum(
        0,
    );
    pub const DATAFLOW_CUSTOM_ATTRIBUTE_TYPE_INT32: ECustomAttributeTypeEnum = ECustomAttributeTypeEnum(
        1,
    );
    pub const DATAFLOW_CUSTOM_ATTRIBUTE_TYPE_FLOAT: ECustomAttributeTypeEnum = ECustomAttributeTypeEnum(
        2,
    );
    pub const DATAFLOW_CUSTOM_ATTRIBUTE_TYPE_DOUBLE: ECustomAttributeTypeEnum = ECustomAttributeTypeEnum(
        3,
    );
    pub const DATAFLOW_CUSTOM_ATTRIBUTE_TYPE_BOOL: ECustomAttributeTypeEnum = ECustomAttributeTypeEnum(
        4,
    );
    pub const DATAFLOW_CUSTOM_ATTRIBUTE_TYPE_STRING: ECustomAttributeTypeEnum = ECustomAttributeTypeEnum(
        5,
    );
    pub const DATAFLOW_CUSTOM_ATTRIBUTE_TYPE_VECTOR2F: ECustomAttributeTypeEnum = ECustomAttributeTypeEnum(
        6,
    );
    pub const DATAFLOW_CUSTOM_ATTRIBUTE_TYPE_VECTOR3F: ECustomAttributeTypeEnum = ECustomAttributeTypeEnum(
        7,
    );
    pub const DATAFLOW_CUSTOM_ATTRIBUTE_TYPE_VECTOR3D: ECustomAttributeTypeEnum = ECustomAttributeTypeEnum(
        8,
    );
    pub const DATAFLOW_CUSTOM_ATTRIBUTE_TYPE_VECTOR4F: ECustomAttributeTypeEnum = ECustomAttributeTypeEnum(
        9,
    );
    pub const DATAFLOW_CUSTOM_ATTRIBUTE_TYPE_LINEAR_COLOR: ECustomAttributeTypeEnum = ECustomAttributeTypeEnum(
        10,
    );
    pub const DATAFLOW_CUSTOM_ATTRIBUTE_TYPE_TRANSFORM: ECustomAttributeTypeEnum = ECustomAttributeTypeEnum(
        11,
    );
    pub const DATAFLOW_CUSTOM_ATTRIBUTE_TYPE_QUAT4F: ECustomAttributeTypeEnum = ECustomAttributeTypeEnum(
        12,
    );
    pub const DATAFLOW_CUSTOM_ATTRIBUTE_TYPE_BOX: ECustomAttributeTypeEnum = ECustomAttributeTypeEnum(
        13,
    );
    pub const DATAFLOW_CUSTOM_ATTRIBUTE_TYPE_GUID: ECustomAttributeTypeEnum = ECustomAttributeTypeEnum(
        14,
    );
    pub const DATAFLOW_CUSTOM_ATTRIBUTE_TYPE_INT32_SET: ECustomAttributeTypeEnum = ECustomAttributeTypeEnum(
        15,
    );
    pub const DATAFLOW_CUSTOM_ATTRIBUTE_TYPE_INT32_ARRAY: ECustomAttributeTypeEnum = ECustomAttributeTypeEnum(
        16,
    );
    pub const DATAFLOW_CUSTOM_ATTRIBUTE_TYPE_INT_VECTOR: ECustomAttributeTypeEnum = ECustomAttributeTypeEnum(
        17,
    );
    pub const DATAFLOW_CUSTOM_ATTRIBUTE_TYPE_INT_VECTOR2: ECustomAttributeTypeEnum = ECustomAttributeTypeEnum(
        18,
    );
    pub const DATAFLOW_CUSTOM_ATTRIBUTE_TYPE_INT_VECTOR4: ECustomAttributeTypeEnum = ECustomAttributeTypeEnum(
        19,
    );
    pub const DATAFLOW_CUSTOM_ATTRIBUTE_TYPE_INT_VECTOR2_ARRAY: ECustomAttributeTypeEnum = ECustomAttributeTypeEnum(
        20,
    );
    pub const DATAFLOW_CUSTOM_ATTRIBUTE_TYPE_FLOAT_ARRAY: ECustomAttributeTypeEnum = ECustomAttributeTypeEnum(
        21,
    );
    pub const DATAFLOW_CUSTOM_ATTRIBUTE_TYPE_VECTOR2F_ARRAY: ECustomAttributeTypeEnum = ECustomAttributeTypeEnum(
        22,
    );
    pub const DATAFLOW_CUSTOM_ATTRIBUTE_TYPE_F_VECTOR3F_ARRAY: ECustomAttributeTypeEnum = ECustomAttributeTypeEnum(
        23,
    );
}
#[repr(transparent)]
pub struct ESetOperationEnum(pub u8);
impl ESetOperationEnum {
    pub const DATAFLOW_SET_OPERATION_AND: ESetOperationEnum = ESetOperationEnum(0);
    pub const DATAFLOW_SET_OPERATION_OR: ESetOperationEnum = ESetOperationEnum(1);
    pub const DATAFLOW_SET_OPERATION_XOR: ESetOperationEnum = ESetOperationEnum(2);
    pub const DATAFLOW_SET_OPERATION_SUBTRACT: ESetOperationEnum = ESetOperationEnum(3);
}
#[repr(transparent)]
pub struct ERangeSettingEnum(pub u8);
impl ERangeSettingEnum {
    pub const DATAFLOW_RANGE_SETTING_INSIDE_RANGE: ERangeSettingEnum = ERangeSettingEnum(
        0,
    );
    pub const DATAFLOW_RANGE_SETTING_OUTSIDE_RANGE: ERangeSettingEnum = ERangeSettingEnum(
        1,
    );
}
#[repr(transparent)]
pub struct ESelectSubjectTypeEnum(pub u8);
impl ESelectSubjectTypeEnum {
    pub const DATAFLOW_SELECT_SUBJECT_TYPE_VERTICES: ESelectSubjectTypeEnum = ESelectSubjectTypeEnum(
        0,
    );
    pub const DATAFLOW_SELECT_SUBJECT_TYPE_BOUNDING_BOX: ESelectSubjectTypeEnum = ESelectSubjectTypeEnum(
        1,
    );
    pub const DATAFLOW_SELECT_SUBJECT_TYPE_CENTROID: ESelectSubjectTypeEnum = ESelectSubjectTypeEnum(
        2,
    );
}
#[repr(transparent)]
pub struct ESelectionByAttrGroup(pub u8);
impl ESelectionByAttrGroup {
    pub const VERTICES: ESelectionByAttrGroup = ESelectionByAttrGroup(0);
    pub const FACES: ESelectionByAttrGroup = ESelectionByAttrGroup(1);
    pub const TRANSFORM: ESelectionByAttrGroup = ESelectionByAttrGroup(2);
    pub const GEOMETRY: ESelectionByAttrGroup = ESelectionByAttrGroup(3);
    pub const MATERIAL: ESelectionByAttrGroup = ESelectionByAttrGroup(4);
    pub const CURVES: ESelectionByAttrGroup = ESelectionByAttrGroup(5);
}
#[repr(transparent)]
pub struct ESelectionByAttrOperation(pub u8);
impl ESelectionByAttrOperation {
    pub const EQUAL: ESelectionByAttrOperation = ESelectionByAttrOperation(0);
    pub const NOT_EQUAL: ESelectionByAttrOperation = ESelectionByAttrOperation(1);
    pub const GREATER: ESelectionByAttrOperation = ESelectionByAttrOperation(2);
    pub const GREATER_OR_EQUAL: ESelectionByAttrOperation = ESelectionByAttrOperation(3);
    pub const SMALLER: ESelectionByAttrOperation = ESelectionByAttrOperation(4);
    pub const SMALLER_OR_EQUAL: ESelectionByAttrOperation = ESelectionByAttrOperation(5);
    pub const MAXIMUM: ESelectionByAttrOperation = ESelectionByAttrOperation(6);
    pub const MINIMUM: ESelectionByAttrOperation = ESelectionByAttrOperation(7);
}
#[repr(transparent)]
pub struct EDataflowCollectionSelectionByNameMethod(pub u8);
impl EDataflowCollectionSelectionByNameMethod {
    pub const EXACT: EDataflowCollectionSelectionByNameMethod = EDataflowCollectionSelectionByNameMethod(
        0,
    );
    pub const STARTS_WITH: EDataflowCollectionSelectionByNameMethod = EDataflowCollectionSelectionByNameMethod(
        1,
    );
    pub const ENDS_WITH: EDataflowCollectionSelectionByNameMethod = EDataflowCollectionSelectionByNameMethod(
        2,
    );
    pub const CONTAINS: EDataflowCollectionSelectionByNameMethod = EDataflowCollectionSelectionByNameMethod(
        3,
    );
}
#[repr(transparent)]
pub struct ECollectionBakeTextureAttribute(pub i32);
impl ECollectionBakeTextureAttribute {
    pub const NONE: ECollectionBakeTextureAttribute = ECollectionBakeTextureAttribute(0);
    pub const DISTANCE_TO_EXTERNAL: ECollectionBakeTextureAttribute = ECollectionBakeTextureAttribute(
        1,
    );
    pub const AMBIENT_OCCLUSION: ECollectionBakeTextureAttribute = ECollectionBakeTextureAttribute(
        2,
    );
    pub const CURVATURE: ECollectionBakeTextureAttribute = ECollectionBakeTextureAttribute(
        3,
    );
    pub const NORMAL_X: ECollectionBakeTextureAttribute = ECollectionBakeTextureAttribute(
        4,
    );
    pub const NORMAL_Y: ECollectionBakeTextureAttribute = ECollectionBakeTextureAttribute(
        5,
    );
    pub const NORMAL_Z: ECollectionBakeTextureAttribute = ECollectionBakeTextureAttribute(
        6,
    );
    pub const POSITION_X: ECollectionBakeTextureAttribute = ECollectionBakeTextureAttribute(
        7,
    );
    pub const POSITION_Y: ECollectionBakeTextureAttribute = ECollectionBakeTextureAttribute(
        8,
    );
    pub const POSITION_Z: ECollectionBakeTextureAttribute = ECollectionBakeTextureAttribute(
        9,
    );
}
#[repr(transparent)]
pub struct EDataflowTransferVertexAttributeNodeTransferMethod(pub u8);
impl EDataflowTransferVertexAttributeNodeTransferMethod {
    pub const COMPONENT: EDataflowTransferVertexAttributeNodeTransferMethod = EDataflowTransferVertexAttributeNodeTransferMethod(
        0,
    );
    pub const GLOBAL: EDataflowTransferVertexAttributeNodeTransferMethod = EDataflowTransferVertexAttributeNodeTransferMethod(
        1,
    );
    pub const NONE: EDataflowTransferVertexAttributeNodeTransferMethod = EDataflowTransferVertexAttributeNodeTransferMethod(
        2,
    );
}
#[repr(transparent)]
pub struct EDataflowTransferVertexAttributeNodeBoundingVolume(pub u8);
impl EDataflowTransferVertexAttributeNodeBoundingVolume {
    pub const VERTEX: EDataflowTransferVertexAttributeNodeBoundingVolume = EDataflowTransferVertexAttributeNodeBoundingVolume(
        0,
    );
    pub const TRIANGLE: EDataflowTransferVertexAttributeNodeBoundingVolume = EDataflowTransferVertexAttributeNodeBoundingVolume(
        1,
    );
}
#[repr(transparent)]
pub struct EDataflowTransferVertexAttributeNodeSourceScale(pub u8);
impl EDataflowTransferVertexAttributeNodeSourceScale {
    pub const COMPONENT_EDGE: EDataflowTransferVertexAttributeNodeSourceScale = EDataflowTransferVertexAttributeNodeSourceScale(
        0,
    );
    pub const ASSET_EDGE: EDataflowTransferVertexAttributeNodeSourceScale = EDataflowTransferVertexAttributeNodeSourceScale(
        1,
    );
    pub const ASSET_BOUND: EDataflowTransferVertexAttributeNodeSourceScale = EDataflowTransferVertexAttributeNodeSourceScale(
        2,
    );
}
#[repr(transparent)]
pub struct EDataflowTransferVertexAttributeNodeFalloff(pub u8);
impl EDataflowTransferVertexAttributeNodeFalloff {
    pub const SQUARED: EDataflowTransferVertexAttributeNodeFalloff = EDataflowTransferVertexAttributeNodeFalloff(
        0,
    );
    pub const LINEAR: EDataflowTransferVertexAttributeNodeFalloff = EDataflowTransferVertexAttributeNodeFalloff(
        1,
    );
    pub const NONE: EDataflowTransferVertexAttributeNodeFalloff = EDataflowTransferVertexAttributeNodeFalloff(
        2,
    );
}
#[repr(transparent)]
pub struct ESetKinematicVertexSelectionKinematicValue(pub u8);
impl ESetKinematicVertexSelectionKinematicValue {
    pub const SET_KINEMATIC: ESetKinematicVertexSelectionKinematicValue = ESetKinematicVertexSelectionKinematicValue(
        0,
    );
    pub const SET_NON_KINEMATIC: ESetKinematicVertexSelectionKinematicValue = ESetKinematicVertexSelectionKinematicValue(
        1,
    );
}
#[repr(transparent)]
pub struct EConvexOverlapRemovalMethodEnum(pub u8);
impl EConvexOverlapRemovalMethodEnum {
    pub const DATAFLOW_E_CONVEX_OVERLAP_REMOVAL_METHOD_NONE: EConvexOverlapRemovalMethodEnum = EConvexOverlapRemovalMethodEnum(
        0,
    );
    pub const DATAFLOW_E_CONVEX_OVERLAP_REMOVAL_METHOD_ALL: EConvexOverlapRemovalMethodEnum = EConvexOverlapRemovalMethodEnum(
        1,
    );
    pub const DATAFLOW_E_CONVEX_OVERLAP_REMOVAL_METHOD_ONLY_CLUSTERS: EConvexOverlapRemovalMethodEnum = EConvexOverlapRemovalMethodEnum(
        2,
    );
    pub const DATAFLOW_E_CONVEX_OVERLAP_REMOVAL_METHOD_ONLY_CLUSTERS_VS_CLUSTERS: EConvexOverlapRemovalMethodEnum = EConvexOverlapRemovalMethodEnum(
        3,
    );
}
#[repr(transparent)]
pub struct ENegativeSpaceSampleMethodDataflowEnum(pub u8);
impl ENegativeSpaceSampleMethodDataflowEnum {
    pub const UNIFORM: ENegativeSpaceSampleMethodDataflowEnum = ENegativeSpaceSampleMethodDataflowEnum(
        0,
    );
    pub const VOXEL_SEARCH: ENegativeSpaceSampleMethodDataflowEnum = ENegativeSpaceSampleMethodDataflowEnum(
        1,
    );
    pub const NAVIGABLE_VOXEL_SEARCH: ENegativeSpaceSampleMethodDataflowEnum = ENegativeSpaceSampleMethodDataflowEnum(
        2,
    );
}
