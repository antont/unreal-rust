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
pub static mut U_OBJECT_EXECUTE_UBERGRAPH: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
pub fn initialize() {
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UObject::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ExecuteUbergraph"),
            &raw mut U_OBJECT_EXECUTE_UBERGRAPH,
        );
    }
}
#[repr(C, align(4))]
pub struct FGuid {
    __padding_end: [u8; 16],
}
impl FGuid {}
#[repr(C, align(8))]
pub struct FDateTime {
    __padding_end: [u8; 8],
}
impl FDateTime {}
#[repr(C, align(8))]
pub struct FBox {
    pub min: FVector,
    pub max: FVector,
    pub is_valid: bool,
    __padding_end: [u8; 7],
}
impl FBox {}
#[repr(C, align(8))]
pub struct FVector {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}
impl FVector {}
#[repr(C, align(8))]
pub struct FBox2D {
    pub min: FVector2D,
    pub max: FVector2D,
    pub b_is_valid: bool,
    __padding_end: [u8; 7],
}
impl FBox2D {}
#[repr(C, align(8))]
pub struct FVector2D {
    pub x: f64,
    pub y: f64,
}
impl FVector2D {}
#[repr(C, align(4))]
pub struct FBox2f {
    __padding_end: [u8; 20],
}
impl FBox2f {}
#[repr(C, align(4))]
pub struct FVector2f {
    __padding_end: [u8; 8],
}
impl FVector2f {}
#[repr(C, align(4))]
pub struct FBox3f {
    __padding_end: [u8; 28],
}
impl FBox3f {}
#[repr(C, align(4))]
pub struct FVector3f {
    __padding_end: [u8; 12],
}
impl FVector3f {}
#[repr(C, align(8))]
pub struct FBoxSphereBounds {
    pub origin: FVector,
    pub box_extent: FVector,
    pub sphere_radius: f64,
}
impl FBoxSphereBounds {}
#[repr(C, align(4))]
pub struct FColor {
    pub b: u8,
    pub g: u8,
    pub r: u8,
    pub a: u8,
}
impl FColor {}
#[repr(C, align(8))]
pub struct FDoubleRange {
    pub lower_bound: FDoubleRangeBound,
    pub upper_bound: FDoubleRangeBound,
}
impl FDoubleRange {}
#[repr(C, align(8))]
pub struct FDoubleRangeBound {
    pub ty: ERangeBoundTypes,
    pub value: f64,
}
impl FDoubleRangeBound {}
#[repr(C, align(4))]
pub struct FFloatInterval {
    pub min: f32,
    pub max: f32,
}
impl FFloatInterval {}
#[repr(C, align(4))]
pub struct FFloatRange {
    pub lower_bound: FFloatRangeBound,
    pub upper_bound: FFloatRangeBound,
}
impl FFloatRange {}
#[repr(C, align(4))]
pub struct FFloatRangeBound {
    pub ty: ERangeBoundTypes,
    pub value: f32,
}
impl FFloatRangeBound {}
#[repr(C, align(4))]
pub struct FFrameNumber {
    pub value: i32,
}
impl FFrameNumber {}
#[repr(C, align(4))]
pub struct FFrameNumberRange {
    pub lower_bound: FFrameNumberRangeBound,
    pub upper_bound: FFrameNumberRangeBound,
}
impl FFrameNumberRange {}
#[repr(C, align(4))]
pub struct FFrameNumberRangeBound {
    pub ty: ERangeBoundTypes,
    pub value: FFrameNumber,
}
impl FFrameNumberRangeBound {}
#[repr(C, align(4))]
pub struct FFrameRate {
    pub numerator: i32,
    pub denominator: i32,
}
impl FFrameRate {}
#[repr(C, align(4))]
pub struct FFrameTime {
    pub frame_number: FFrameNumber,
    pub sub_frame: f32,
}
impl FFrameTime {}
#[repr(C, align(4))]
pub struct FInputDeviceId {
    __padding_end: [u8; 4],
}
impl FInputDeviceId {}
#[repr(C, align(4))]
pub struct FInt32Interval {
    pub min: i32,
    pub max: i32,
}
impl FInt32Interval {}
#[repr(C, align(4))]
pub struct FInt32Range {
    pub lower_bound: FInt32RangeBound,
    pub upper_bound: FInt32RangeBound,
}
impl FInt32Range {}
#[repr(C, align(4))]
pub struct FInt32RangeBound {
    pub ty: ERangeBoundTypes,
    pub value: i32,
}
impl FInt32RangeBound {}
#[repr(C, align(8))]
pub struct FInterpCurveFloat {
    pub points: TArray<FInterpCurvePointFloat>,
    pub b_is_looped: bool,
    pub loop_key_offset: f32,
}
impl FInterpCurveFloat {}
#[repr(C, align(4))]
pub struct FInterpCurvePointFloat {
    pub in_val: f32,
    pub out_val: f32,
    pub arrive_tangent: f32,
    pub leave_tangent: f32,
    pub interp_mode: EInterpCurveMode,
    __padding_end: [u8; 3],
}
impl FInterpCurvePointFloat {}
#[repr(C, align(8))]
pub struct FInterpCurveLinearColor {
    pub points: TArray<FInterpCurvePointLinearColor>,
    pub b_is_looped: bool,
    pub loop_key_offset: f32,
}
impl FInterpCurveLinearColor {}
#[repr(C, align(4))]
pub struct FInterpCurvePointLinearColor {
    pub in_val: f32,
    pub out_val: FLinearColor,
    pub arrive_tangent: FLinearColor,
    pub leave_tangent: FLinearColor,
    pub interp_mode: EInterpCurveMode,
    __padding_end: [u8; 3],
}
impl FInterpCurvePointLinearColor {}
#[repr(C, align(4))]
pub struct FLinearColor {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}
impl FLinearColor {}
#[repr(C, align(16))]
pub struct FInterpCurvePointQuat {
    pub in_val: f32,
    pub out_val: FQuat,
    pub arrive_tangent: FQuat,
    pub leave_tangent: FQuat,
    pub interp_mode: EInterpCurveMode,
    __padding_end: [u8; 15],
}
impl FInterpCurvePointQuat {}
#[repr(C, align(16))]
pub struct FQuat {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64,
}
impl FQuat {}
#[repr(C, align(8))]
pub struct FInterpCurvePointTwoVectors {
    pub in_val: f32,
    pub out_val: FTwoVectors,
    pub arrive_tangent: FTwoVectors,
    pub leave_tangent: FTwoVectors,
    pub interp_mode: EInterpCurveMode,
    __padding_end: [u8; 7],
}
impl FInterpCurvePointTwoVectors {}
#[repr(C, align(8))]
pub struct FTwoVectors {
    pub v1: FVector,
    pub v2: FVector,
}
impl FTwoVectors {}
#[repr(C, align(8))]
pub struct FInterpCurvePointVector {
    pub in_val: f32,
    pub out_val: FVector,
    pub arrive_tangent: FVector,
    pub leave_tangent: FVector,
    pub interp_mode: EInterpCurveMode,
    __padding_end: [u8; 7],
}
impl FInterpCurvePointVector {}
#[repr(C, align(8))]
pub struct FInterpCurvePointVector2D {
    pub in_val: f32,
    pub out_val: FVector2D,
    pub arrive_tangent: FVector2D,
    pub leave_tangent: FVector2D,
    pub interp_mode: EInterpCurveMode,
    __padding_end: [u8; 7],
}
impl FInterpCurvePointVector2D {}
#[repr(C, align(8))]
pub struct FInterpCurveQuat {
    pub points: TArray<FInterpCurvePointQuat>,
    pub b_is_looped: bool,
    pub loop_key_offset: f32,
}
impl FInterpCurveQuat {}
#[repr(C, align(8))]
pub struct FInterpCurveTwoVectors {
    pub points: TArray<FInterpCurvePointTwoVectors>,
    pub b_is_looped: bool,
    pub loop_key_offset: f32,
}
impl FInterpCurveTwoVectors {}
#[repr(C, align(8))]
pub struct FInterpCurveVector {
    pub points: TArray<FInterpCurvePointVector>,
    pub b_is_looped: bool,
    pub loop_key_offset: f32,
}
impl FInterpCurveVector {}
#[repr(C, align(8))]
pub struct FInterpCurveVector2D {
    pub points: TArray<FInterpCurvePointVector2D>,
    pub b_is_looped: bool,
    pub loop_key_offset: f32,
}
impl FInterpCurveVector2D {}
#[repr(C, align(4))]
pub struct FIntPoint {
    pub x: i32,
    pub y: i32,
}
impl FIntPoint {}
#[repr(C, align(4))]
pub struct FIntRect {
    pub min: FIntPoint,
    pub max: FIntPoint,
}
impl FIntRect {}
#[repr(C, align(4))]
pub struct FIntVector {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}
impl FIntVector {}
#[repr(C, align(4))]
pub struct FIntVector2 {
    pub x: i32,
    pub y: i32,
}
impl FIntVector2 {}
#[repr(C, align(4))]
pub struct FIntVector4 {
    pub x: i32,
    pub y: i32,
    pub z: i32,
    pub w: i32,
}
impl FIntVector4 {}
#[repr(C, align(16))]
pub struct FMatrix {
    pub x_plane: FPlane,
    pub y_plane: FPlane,
    pub z_plane: FPlane,
    pub w_plane: FPlane,
}
impl FMatrix {}
#[repr(C, align(16))]
pub struct FPlane {
    #[doc(hidden)]
    __padding_24: [u8; 24],
    pub w: f64,
}
impl FPlane {}
#[repr(C, align(16))]
pub struct FMatrix44f {
    __padding_end: [u8; 64],
}
impl FMatrix44f {}
#[repr(C, align(16))]
pub struct FPlane4f {
    __padding_end: [u8; 16],
}
impl FPlane4f {}
#[repr(C, align(4))]
pub struct FMusicalTime {
    __padding_end: [u8; 16],
}
impl FMusicalTime {}
#[repr(C, align(8))]
pub struct FOrientedBox {
    pub center: FVector,
    pub axis_x: FVector,
    pub axis_y: FVector,
    pub axis_z: FVector,
    pub extent_x: f64,
    pub extent_y: f64,
    pub extent_z: f64,
}
impl FOrientedBox {}
#[repr(C, align(8))]
pub struct FPackedRemoteObjectPathName {
    __padding_end: [u8; 32],
}
impl FPackedRemoteObjectPathName {}
#[repr(C, align(4))]
pub struct FPlatformInputDeviceState {
    pub owning_platform_user: FPlatformUserId,
    pub connection_state: EInputDeviceConnectionState,
    __padding_end: [u8; 3],
}
impl FPlatformInputDeviceState {}
#[repr(C, align(4))]
pub struct FPlatformUserId {
    __padding_end: [u8; 4],
}
impl FPlatformUserId {}
#[repr(C, align(8))]
pub struct FPolyglotTextData {
    pub category: ELocalizedTextSourceCategory,
    pub native_culture: FString,
    pub namespace: FString,
    pub key: FString,
    pub native_string: FString,
    pub localized_strings: TMap<FString, FString>,
    pub b_is_minimal_patch: bool,
    __padding_end: [u8; 23],
}
impl FPolyglotTextData {}
#[repr(C, align(4))]
pub struct FQualifiedFrameTime {
    pub time: FFrameTime,
    pub rate: FFrameRate,
}
impl FQualifiedFrameTime {}
#[repr(C, align(16))]
pub struct FQuat4f {
    __padding_end: [u8; 16],
}
impl FQuat4f {}
#[repr(C, align(4))]
pub struct FRandomStream {
    pub initial_seed: i32,
    __padding_end: [u8; 4],
}
impl FRandomStream {}
#[repr(C, align(8))]
pub struct FRay {
    pub origin: FVector,
    pub direction: FVector,
}
impl FRay {}
#[repr(C, align(8))]
pub struct FRemoteObjectBytes {
    __padding_end: [u8; 16],
}
impl FRemoteObjectBytes {}
#[repr(C, align(8))]
pub struct FRemoteObjectData {
    __padding_end: [u8; 64],
}
impl FRemoteObjectData {}
#[repr(C, align(8))]
pub struct FRemoteObjectTables {
    __padding_end: [u8; 32],
}
impl FRemoteObjectTables {}
#[repr(C, align(8))]
pub struct FRemoteObjectId {
    __padding_end: [u8; 8],
}
impl FRemoteObjectId {}
#[repr(C, align(8))]
pub struct FRemoteObjectPathName {
    __padding_end: [u8; 32],
}
impl FRemoteObjectPathName {}
#[repr(C, align(8))]
pub struct FRemoteObjectReference {
    __padding_end: [u8; 16],
}
impl FRemoteObjectReference {}
#[repr(C, align(4))]
pub struct FRemoteServerId {
    __padding_end: [u8; 4],
}
impl FRemoteServerId {}
#[repr(C, align(4))]
pub struct FRemoteTransactionId {
    __padding_end: [u8; 4],
}
impl FRemoteTransactionId {}
#[repr(C, align(8))]
pub struct FRemoteWorkPriority {
    __padding_end: [u8; 8],
}
impl FRemoteWorkPriority {}
#[repr(C, align(8))]
pub struct FRotator {
    pub pitch: f64,
    pub yaw: f64,
    pub roll: f64,
}
impl FRotator {}
#[repr(C, align(4))]
pub struct FRotator3f {
    __padding_end: [u8; 12],
}
impl FRotator3f {}
#[repr(C, align(8))]
pub struct FSphere {
    pub center: FVector,
    pub w: f64,
}
impl FSphere {}
#[repr(C, align(4))]
pub struct FTimecode {
    pub hours: i32,
    pub minutes: i32,
    pub seconds: i32,
    pub frames: i32,
    pub subframe: f32,
    pub b_drop_frame_format: bool,
    __padding_end: [u8; 3],
}
impl FTimecode {}
#[repr(C, align(8))]
pub struct FTimespan {
    __padding_end: [u8; 8],
}
impl FTimespan {}
#[repr(C, align(16))]
pub struct FTransform {
    pub rotation: FQuat,
    pub translation: FVector,
    #[doc(hidden)]
    __padding_64: [u8; 8],
    pub scale3_d: FVector,
    __padding_end: [u8; 8],
}
impl FTransform {}
#[repr(C, align(16))]
pub struct FTransform3f {
    __padding_end: [u8; 48],
}
impl FTransform3f {}
#[repr(C, align(16))]
pub struct FVector4 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64,
}
impl FVector4 {}
#[repr(C, align(16))]
pub struct FVector4f {
    __padding_end: [u8; 16],
}
impl FVector4f {}
#[repr(C, align(4))]
pub struct FTopLevelAssetPath {
    pub package_name: FName,
    pub asset_name: FName,
}
impl FTopLevelAssetPath {}
#[repr(C, align(8))]
pub struct FSoftObjectPath {
    __padding_end: [u8; 40],
}
impl FSoftObjectPath {}
#[repr(C, align(8))]
pub struct FARFilter {
    pub package_names: TArray<FName>,
    pub package_paths: TArray<FName>,
    #[doc(hidden)]
    __padding_48: [u8; 16],
    pub soft_object_paths: TArray<FSoftObjectPath>,
    pub class_names: TArray<FName>,
    pub class_paths: TArray<FTopLevelAssetPath>,
    #[doc(hidden)]
    __padding_176: [u8; 80],
    pub recursive_classes_exclusion_set: TSet<FName>,
    pub recursive_class_paths_exclusion_set: TSet<FTopLevelAssetPath>,
    pub b_recursive_paths: bool,
    pub b_recursive_classes: bool,
    pub b_include_only_on_disk_assets: bool,
    __padding_end: [u8; 13],
}
impl FARFilter {}
#[repr(C, align(8))]
pub struct FSoftClassPath {
    __padding_end: [u8; 40],
}
impl FSoftClassPath {}
#[repr(C, align(4))]
pub struct FPrimaryAssetId {
    pub primary_asset_type: FPrimaryAssetType,
    pub primary_asset_name: FName,
}
impl FPrimaryAssetId {}
#[repr(C, align(4))]
pub struct FPrimaryAssetType {
    pub name: FName,
}
impl FPrimaryAssetType {}
#[repr(C, align(8))]
pub struct FAssetData {
    #[doc(hidden)]
    __padding_12: [u8; 12],
    pub package_name: FName,
    pub package_path: FName,
    pub asset_name: FName,
    pub asset_class: FName,
    pub asset_class_path: FTopLevelAssetPath,
    __padding_end: [u8; 68],
}
impl FAssetData {}
#[repr(C, align(8))]
pub struct FInstancedStructBaseStructQueryParams {
    __padding_end: [u8; 16],
}
impl FInstancedStructBaseStructQueryParams {}
#[repr(C, align(8))]
pub struct FInstancedStruct {
    __padding_end: [u8; 16],
}
impl FInstancedStruct {}
#[repr(C, align(8))]
pub struct FPerPlatformInt {
    pub default: i32,
    __padding_end: [u8; 84],
}
impl FPerPlatformInt {}
#[repr(C, align(8))]
pub struct FPerPlatformFloat {
    pub default: f32,
    __padding_end: [u8; 84],
}
impl FPerPlatformFloat {}
#[repr(C, align(8))]
pub struct FPerPlatformFrameRate {
    __padding_end: [u8; 88],
}
impl FPerPlatformFrameRate {}
#[repr(C, align(8))]
pub struct FFilePath {
    pub file_path: FString,
}
impl FFilePath {}
#[repr(C, align(8))]
pub struct FDirectoryPath {
    pub path: FString,
}
impl FDirectoryPath {}
#[repr(C, align(8))]
pub struct FTemplateString {
    pub template: FString,
    pub resolved: FText,
}
impl FTemplateString {}
#[repr(C, align(8))]
pub struct UObject {
    __padding_end: [u8; 48],
}
impl UObject {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS.wait().name_to_ptr.get("UObject").unwrap()
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
pub struct UGCBarrier {
    __padding_end: [u8; 48],
}
impl UGCBarrier {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UGCBarrier")
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
pub struct UGCObjectReferencer {
    __padding_end: [u8; 128],
}
impl UGCObjectReferencer {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UGCObjectReferencer")
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
pub struct UObjectPtrTestClass {
    __padding_end: [u8; 48],
}
impl UObjectPtrTestClass {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UObjectPtrTestClass")
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
pub struct UObjectPtrAbstractTestClass {
    __padding_end: [u8; 48],
}
impl UObjectPtrAbstractTestClass {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UObjectPtrAbstractTestClass")
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
pub struct UObjectPtrAbstractDerivedTestClass {
    __padding_end: [u8; 48],
}
impl UObjectPtrAbstractDerivedTestClass {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UObjectPtrAbstractDerivedTestClass")
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
pub struct UObjectPtrTestClassWithRef {
    __padding_end: [u8; 88],
}
impl UObjectPtrTestClassWithRef {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UObjectPtrTestClassWithRef")
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
pub struct UObjectWithClassProperty {
    __padding_end: [u8; 72],
}
impl UObjectWithClassProperty {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UObjectWithClassProperty")
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
pub struct UObjectWithRawProperty {
    __padding_end: [u8; 64],
}
impl UObjectWithRawProperty {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UObjectWithRawProperty")
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
pub struct UObjectPtrDerrivedTestClass {
    __padding_end: [u8; 48],
}
impl UObjectPtrDerrivedTestClass {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UObjectPtrDerrivedTestClass")
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
pub struct UObjectPtrNotLazyTestClass {
    __padding_end: [u8; 48],
}
impl UObjectPtrNotLazyTestClass {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UObjectPtrNotLazyTestClass")
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
pub struct UObjectPtrStressTestClass {
    __padding_end: [u8; 112],
}
impl UObjectPtrStressTestClass {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UObjectPtrStressTestClass")
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
pub struct UMiddleClass {
    __padding_end: [u8; 56],
}
impl UMiddleClass {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMiddleClass")
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
pub struct UDerrivedClass {
    __padding_end: [u8; 64],
}
impl UDerrivedClass {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDerrivedClass")
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
pub struct UOptionalPropertyTestObject {
    __padding_end: [u8; 104],
}
impl UOptionalPropertyTestObject {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UOptionalPropertyTestObject")
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
pub struct UPropertyWrapper {
    __padding_end: [u8; 56],
}
impl UPropertyWrapper {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPropertyWrapper")
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
pub struct UMulticastDelegatePropertyWrapper {
    __padding_end: [u8; 56],
}
impl UMulticastDelegatePropertyWrapper {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMulticastDelegatePropertyWrapper")
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
pub struct UMulticastInlineDelegatePropertyWrapper {
    __padding_end: [u8; 56],
}
impl UMulticastInlineDelegatePropertyWrapper {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMulticastInlineDelegatePropertyWrapper")
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
pub struct UField {
    __padding_end: [u8; 56],
}
impl UField {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS.wait().name_to_ptr.get("UField").unwrap()
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
pub struct UProperty {
    __padding_end: [u8; 136],
}
impl UProperty {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UProperty")
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
pub struct UNumericProperty {
    __padding_end: [u8; 136],
}
impl UNumericProperty {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNumericProperty")
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
pub struct UByteProperty {
    __padding_end: [u8; 144],
}
impl UByteProperty {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UByteProperty")
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
pub struct UInt8Property {
    __padding_end: [u8; 136],
}
impl UInt8Property {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UInt8Property")
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
pub struct UInt16Property {
    __padding_end: [u8; 136],
}
impl UInt16Property {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UInt16Property")
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
pub struct UIntProperty {
    __padding_end: [u8; 136],
}
impl UIntProperty {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UIntProperty")
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
pub struct UInt64Property {
    __padding_end: [u8; 136],
}
impl UInt64Property {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UInt64Property")
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
pub struct UUInt16Property {
    __padding_end: [u8; 136],
}
impl UUInt16Property {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UUInt16Property")
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
pub struct UUInt32Property {
    __padding_end: [u8; 136],
}
impl UUInt32Property {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UUInt32Property")
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
pub struct UUInt64Property {
    __padding_end: [u8; 136],
}
impl UUInt64Property {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UUInt64Property")
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
pub struct UFloatProperty {
    __padding_end: [u8; 136],
}
impl UFloatProperty {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UFloatProperty")
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
pub struct UDoubleProperty {
    __padding_end: [u8; 136],
}
impl UDoubleProperty {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDoubleProperty")
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
pub struct UBoolProperty {
    __padding_end: [u8; 144],
}
impl UBoolProperty {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBoolProperty")
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
pub struct UObjectPropertyBase {
    __padding_end: [u8; 144],
}
impl UObjectPropertyBase {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UObjectPropertyBase")
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
pub struct UObjectProperty {
    __padding_end: [u8; 144],
}
impl UObjectProperty {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UObjectProperty")
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
pub struct UWeakObjectProperty {
    __padding_end: [u8; 144],
}
impl UWeakObjectProperty {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UWeakObjectProperty")
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
pub struct ULazyObjectProperty {
    __padding_end: [u8; 144],
}
impl ULazyObjectProperty {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("ULazyObjectProperty")
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
pub struct USoftObjectProperty {
    __padding_end: [u8; 144],
}
impl USoftObjectProperty {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("USoftObjectProperty")
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
pub struct UClassProperty {
    __padding_end: [u8; 152],
}
impl UClassProperty {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UClassProperty")
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
pub struct USoftClassProperty {
    __padding_end: [u8; 152],
}
impl USoftClassProperty {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("USoftClassProperty")
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
pub struct UInterfaceProperty {
    __padding_end: [u8; 144],
}
impl UInterfaceProperty {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UInterfaceProperty")
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
pub struct UNameProperty {
    __padding_end: [u8; 136],
}
impl UNameProperty {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNameProperty")
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
pub struct UStrProperty {
    __padding_end: [u8; 136],
}
impl UStrProperty {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UStrProperty")
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
pub struct UArrayProperty {
    __padding_end: [u8; 144],
}
impl UArrayProperty {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UArrayProperty")
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
pub struct UMapProperty {
    __padding_end: [u8; 176],
}
impl UMapProperty {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMapProperty")
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
pub struct USetProperty {
    __padding_end: [u8; 168],
}
impl USetProperty {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("USetProperty")
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
pub struct UStructProperty {
    __padding_end: [u8; 144],
}
impl UStructProperty {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UStructProperty")
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
pub struct UDelegateProperty {
    __padding_end: [u8; 144],
}
impl UDelegateProperty {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDelegateProperty")
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
pub struct UMulticastDelegateProperty {
    __padding_end: [u8; 144],
}
impl UMulticastDelegateProperty {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMulticastDelegateProperty")
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
pub struct UMulticastInlineDelegateProperty {
    __padding_end: [u8; 144],
}
impl UMulticastInlineDelegateProperty {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMulticastInlineDelegateProperty")
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
pub struct UMulticastSparseDelegateProperty {
    __padding_end: [u8; 144],
}
impl UMulticastSparseDelegateProperty {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMulticastSparseDelegateProperty")
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
pub struct UEnumProperty {
    __padding_end: [u8; 152],
}
impl UEnumProperty {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEnumProperty")
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
pub struct UTextProperty {
    __padding_end: [u8; 136],
}
impl UTextProperty {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UTextProperty")
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
pub struct UInterface {
    __padding_end: [u8; 48],
}
impl UInterface {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UInterface")
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
pub struct IEditorPathObjectInterface {}
#[repr(C, align(8))]
pub struct UEditorPathObjectInterface {
    __padding_end: [u8; 48],
}
impl UEditorPathObjectInterface {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEditorPathObjectInterface")
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
pub struct UTextBuffer {
    __padding_end: [u8; 88],
}
impl UTextBuffer {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UTextBuffer")
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
pub struct UPropertyBagMissingObject {
    __padding_end: [u8; 48],
}
impl UPropertyBagMissingObject {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPropertyBagMissingObject")
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
pub struct UStruct {
    __padding_end: [u8; 208],
}
impl UStruct {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS.wait().name_to_ptr.get("UStruct").unwrap()
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
pub struct UScriptStruct {
    __padding_end: [u8; 224],
}
impl UScriptStruct {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UScriptStruct")
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
pub struct UPropertyBag {
    __padding_end: [u8; 248],
}
impl UPropertyBag {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPropertyBag")
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
pub struct UUserDefinedStruct {
    __padding_end: [u8; 368],
}
impl UUserDefinedStruct {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UUserDefinedStruct")
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
pub struct UUserDefinedStructEditorDataBase {
    __padding_end: [u8; 48],
}
impl UUserDefinedStructEditorDataBase {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UUserDefinedStructEditorDataBase")
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
pub struct UFunction {
    __padding_end: [u8; 264],
}
impl UFunction {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UFunction")
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
pub struct UDelegateFunction {
    __padding_end: [u8; 264],
}
impl UDelegateFunction {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDelegateFunction")
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
pub struct USparseDelegateFunction {
    __padding_end: [u8; 288],
}
impl USparseDelegateFunction {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("USparseDelegateFunction")
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
pub struct UEnum {
    __padding_end: [u8; 120],
}
impl UEnum {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS.wait().name_to_ptr.get("UEnum").unwrap()
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
pub struct UPackage {
    __padding_end: [u8; 360],
}
impl UPackage {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS.wait().name_to_ptr.get("UPackage").unwrap()
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
pub struct UClass {
    __padding_end: [u8; 624],
}
impl UClass {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS.wait().name_to_ptr.get("UClass").unwrap()
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
pub struct UEnumCookedMetaData {
    __padding_end: [u8; 128],
}
impl UEnumCookedMetaData {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEnumCookedMetaData")
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
pub struct UStructCookedMetaData {
    __padding_end: [u8; 208],
}
impl UStructCookedMetaData {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UStructCookedMetaData")
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
pub struct UClassCookedMetaData {
    __padding_end: [u8; 288],
}
impl UClassCookedMetaData {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UClassCookedMetaData")
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
pub struct UPackageMap {
    __padding_end: [u8; 232],
}
impl UPackageMap {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPackageMap")
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
pub struct UObjectReachabilityStressData {
    __padding_end: [u8; 64],
}
impl UObjectReachabilityStressData {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UObjectReachabilityStressData")
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
pub struct UInstanceDataObjectClass {
    __padding_end: [u8; 640],
}
impl UInstanceDataObjectClass {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UInstanceDataObjectClass")
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
pub struct UInstanceDataObjectStruct {
    __padding_end: [u8; 256],
}
impl UInstanceDataObjectStruct {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UInstanceDataObjectStruct")
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
pub struct ULinkerPlaceholderClass {
    __padding_end: [u8; 1072],
}
impl ULinkerPlaceholderClass {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("ULinkerPlaceholderClass")
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
pub struct ULinkerPlaceholderExportObject {
    __padding_end: [u8; 256],
}
impl ULinkerPlaceholderExportObject {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("ULinkerPlaceholderExportObject")
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
pub struct ULinkerPlaceholderFunction {
    __padding_end: [u8; 712],
}
impl ULinkerPlaceholderFunction {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("ULinkerPlaceholderFunction")
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
pub struct UDEPRECATED_MetaData {
    __padding_end: [u8; 208],
}
impl UDEPRECATED_MetaData {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDEPRECATED_MetaData")
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
pub struct UObjectRedirector {
    __padding_end: [u8; 64],
}
impl UObjectRedirector {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UObjectRedirector")
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
pub struct UVerseClass {
    __padding_end: [u8; 1368],
}
impl UVerseClass {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UVerseClass")
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
pub struct UVerseEnum {
    __padding_end: [u8; 160],
}
impl UVerseEnum {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UVerseEnum")
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
pub struct UVerseFunction {
    __padding_end: [u8; 280],
}
impl UVerseFunction {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UVerseFunction")
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
pub struct UVerseStruct {
    __padding_end: [u8; 480],
}
impl UVerseStruct {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UVerseStruct")
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
pub struct UTestPropertyPathFunctionsClass {
    __padding_end: [u8; 2248],
}
impl UTestPropertyPathFunctionsClass {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UTestPropertyPathFunctionsClass")
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
pub struct UTestInstanceDataObjectClass {
    __padding_end: [u8; 136],
}
impl UTestInstanceDataObjectClass {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UTestInstanceDataObjectClass")
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
pub struct USubobjectInstancingTestObject {
    __padding_end: [u8; 56],
}
impl USubobjectInstancingTestObject {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("USubobjectInstancingTestObject")
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
pub struct USubobjectInstancingTestDerivedObject {
    __padding_end: [u8; 56],
}
impl USubobjectInstancingTestDerivedObject {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("USubobjectInstancingTestDerivedObject")
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
pub struct USubobjectInstancingTestDirectlyNestedObject {
    __padding_end: [u8; 80],
}
impl USubobjectInstancingTestDirectlyNestedObject {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("USubobjectInstancingTestDirectlyNestedObject")
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
pub struct USubobjectInstancingTestIndirectlyNestedObject {
    __padding_end: [u8; 64],
}
impl USubobjectInstancingTestIndirectlyNestedObject {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("USubobjectInstancingTestIndirectlyNestedObject")
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
pub struct USubobjectInstancingDefaultToInstancedTestObject {
    __padding_end: [u8; 56],
}
impl USubobjectInstancingDefaultToInstancedTestObject {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("USubobjectInstancingDefaultToInstancedTestObject")
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
pub struct USubobjectInstancingTestOuterObject {
    __padding_end: [u8; 592],
}
impl USubobjectInstancingTestOuterObject {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("USubobjectInstancingTestOuterObject")
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
pub struct USubobjectInstancingTestDerivedOuterObjectWithTypeOverride {
    __padding_end: [u8; 592],
}
impl USubobjectInstancingTestDerivedOuterObjectWithTypeOverride {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("USubobjectInstancingTestDerivedOuterObjectWithTypeOverride")
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
pub struct USubobjectInstancingTestDerivedOuterObjectWithDoNotCreateOverride {
    __padding_end: [u8; 592],
}
impl USubobjectInstancingTestDerivedOuterObjectWithDoNotCreateOverride {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("USubobjectInstancingTestDerivedOuterObjectWithDoNotCreateOverride")
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
pub struct UDynamicSubobjectInstancingTestClass {
    __padding_end: [u8; 624],
}
impl UDynamicSubobjectInstancingTestClass {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDynamicSubobjectInstancingTestClass")
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
pub struct EAutomationEventType(pub u8);
impl EAutomationEventType {
    pub const INFO: EAutomationEventType = EAutomationEventType(0);
    pub const WARNING: EAutomationEventType = EAutomationEventType(1);
    pub const ERROR: EAutomationEventType = EAutomationEventType(2);
}
#[repr(transparent)]
pub struct ERangeBoundTypes(pub u8);
impl ERangeBoundTypes {
    pub const EXCLUSIVE: ERangeBoundTypes = ERangeBoundTypes(0);
    pub const INCLUSIVE: ERangeBoundTypes = ERangeBoundTypes(1);
    pub const OPEN: ERangeBoundTypes = ERangeBoundTypes(2);
}
#[repr(transparent)]
pub struct EInterpCurveMode(pub u8);
impl EInterpCurveMode {
    pub const CIM_LINEAR: EInterpCurveMode = EInterpCurveMode(0);
    pub const CIM_CURVE_AUTO: EInterpCurveMode = EInterpCurveMode(1);
    pub const CIM_CONSTANT: EInterpCurveMode = EInterpCurveMode(2);
    pub const CIM_CURVE_USER: EInterpCurveMode = EInterpCurveMode(3);
    pub const CIM_CURVE_BREAK: EInterpCurveMode = EInterpCurveMode(4);
    pub const CIM_CURVE_AUTO_CLAMPED: EInterpCurveMode = EInterpCurveMode(5);
}
#[repr(transparent)]
pub struct EInputDeviceConnectionState(pub u8);
impl EInputDeviceConnectionState {
    pub const INVALID: EInputDeviceConnectionState = EInputDeviceConnectionState(0);
    pub const UNKNOWN: EInputDeviceConnectionState = EInputDeviceConnectionState(1);
    pub const DISCONNECTED: EInputDeviceConnectionState = EInputDeviceConnectionState(2);
    pub const CONNECTED: EInputDeviceConnectionState = EInputDeviceConnectionState(3);
}
#[repr(transparent)]
pub struct ELocalizedTextSourceCategory(pub u8);
impl ELocalizedTextSourceCategory {
    pub const GAME: ELocalizedTextSourceCategory = ELocalizedTextSourceCategory(0);
    pub const ENGINE: ELocalizedTextSourceCategory = ELocalizedTextSourceCategory(1);
    pub const EDITOR: ELocalizedTextSourceCategory = ELocalizedTextSourceCategory(2);
}
#[repr(transparent)]
pub struct EPropertyBagPropertyType(pub u8);
impl EPropertyBagPropertyType {
    pub const NONE: EPropertyBagPropertyType = EPropertyBagPropertyType(0);
    pub const BOOL: EPropertyBagPropertyType = EPropertyBagPropertyType(1);
    pub const BYTE: EPropertyBagPropertyType = EPropertyBagPropertyType(2);
    pub const INT32: EPropertyBagPropertyType = EPropertyBagPropertyType(3);
    pub const INT64: EPropertyBagPropertyType = EPropertyBagPropertyType(4);
    pub const FLOAT: EPropertyBagPropertyType = EPropertyBagPropertyType(5);
    pub const DOUBLE: EPropertyBagPropertyType = EPropertyBagPropertyType(6);
    pub const NAME: EPropertyBagPropertyType = EPropertyBagPropertyType(7);
    pub const STRING: EPropertyBagPropertyType = EPropertyBagPropertyType(8);
    pub const TEXT: EPropertyBagPropertyType = EPropertyBagPropertyType(9);
    pub const ENUM: EPropertyBagPropertyType = EPropertyBagPropertyType(10);
    pub const STRUCT: EPropertyBagPropertyType = EPropertyBagPropertyType(11);
    pub const OBJECT: EPropertyBagPropertyType = EPropertyBagPropertyType(12);
    pub const SOFT_OBJECT: EPropertyBagPropertyType = EPropertyBagPropertyType(13);
    pub const CLASS: EPropertyBagPropertyType = EPropertyBagPropertyType(14);
    pub const SOFT_CLASS: EPropertyBagPropertyType = EPropertyBagPropertyType(15);
    pub const U_INT32: EPropertyBagPropertyType = EPropertyBagPropertyType(16);
    pub const U_INT64: EPropertyBagPropertyType = EPropertyBagPropertyType(17);
    pub const COUNT: EPropertyBagPropertyType = EPropertyBagPropertyType(18);
}
#[repr(transparent)]
pub struct EOverriddenPropertyOperation(pub u8);
impl EOverriddenPropertyOperation {
    pub const NONE: EOverriddenPropertyOperation = EOverriddenPropertyOperation(0);
    pub const MODIFIED: EOverriddenPropertyOperation = EOverriddenPropertyOperation(1);
    pub const REPLACE: EOverriddenPropertyOperation = EOverriddenPropertyOperation(2);
    pub const ADD: EOverriddenPropertyOperation = EOverriddenPropertyOperation(3);
    pub const REMOVE: EOverriddenPropertyOperation = EOverriddenPropertyOperation(4);
    pub const SUB_OBJECTS_SHADOWING: EOverriddenPropertyOperation = EOverriddenPropertyOperation(
        5,
    );
}
#[repr(transparent)]
pub struct ETestInstanceDataObjectBird(pub u8);
impl ETestInstanceDataObjectBird {
    pub const TIDOB_NONE: ETestInstanceDataObjectBird = ETestInstanceDataObjectBird(0);
    pub const TIDOB_CARDINAL: ETestInstanceDataObjectBird = ETestInstanceDataObjectBird(
        1,
    );
    pub const TIDOB_CROW: ETestInstanceDataObjectBird = ETestInstanceDataObjectBird(2);
    pub const TIDOB_EAGLE: ETestInstanceDataObjectBird = ETestInstanceDataObjectBird(3);
    pub const TIDOB_HAWK: ETestInstanceDataObjectBird = ETestInstanceDataObjectBird(4);
    pub const TIDOB_OWL: ETestInstanceDataObjectBird = ETestInstanceDataObjectBird(5);
    pub const TIDOB_RAVEN: ETestInstanceDataObjectBird = ETestInstanceDataObjectBird(6);
}
#[repr(transparent)]
pub struct ETestInstanceDataObjectGrain(pub u8);
impl ETestInstanceDataObjectGrain {
    pub const NONE: ETestInstanceDataObjectGrain = ETestInstanceDataObjectGrain(0);
    pub const BARLEY: ETestInstanceDataObjectGrain = ETestInstanceDataObjectGrain(1);
    pub const CORN: ETestInstanceDataObjectGrain = ETestInstanceDataObjectGrain(2);
    pub const QUINOA: ETestInstanceDataObjectGrain = ETestInstanceDataObjectGrain(3);
    pub const RICE: ETestInstanceDataObjectGrain = ETestInstanceDataObjectGrain(4);
    pub const WHEAT: ETestInstanceDataObjectGrain = ETestInstanceDataObjectGrain(5);
}
#[repr(transparent)]
pub struct ETestInstanceDataObjectFruit(pub u8);
impl ETestInstanceDataObjectFruit {
    pub const NONE: ETestInstanceDataObjectFruit = ETestInstanceDataObjectFruit(0);
    pub const APPLE: ETestInstanceDataObjectFruit = ETestInstanceDataObjectFruit(1);
    pub const BANANA: ETestInstanceDataObjectFruit = ETestInstanceDataObjectFruit(2);
    pub const LEMON: ETestInstanceDataObjectFruit = ETestInstanceDataObjectFruit(3);
    pub const ORANGE: ETestInstanceDataObjectFruit = ETestInstanceDataObjectFruit(4);
}
#[repr(transparent)]
pub struct ETestInstanceDataObjectDirection(pub u16);
impl ETestInstanceDataObjectDirection {
    pub const NONE: ETestInstanceDataObjectDirection = ETestInstanceDataObjectDirection(
        0,
    );
    pub const NORTH: ETestInstanceDataObjectDirection = ETestInstanceDataObjectDirection(
        1,
    );
    pub const EAST: ETestInstanceDataObjectDirection = ETestInstanceDataObjectDirection(
        2,
    );
    pub const SOUTH: ETestInstanceDataObjectDirection = ETestInstanceDataObjectDirection(
        4,
    );
    pub const WEST: ETestInstanceDataObjectDirection = ETestInstanceDataObjectDirection(
        8,
    );
}
#[repr(transparent)]
pub struct ETestInstanceDataObjectFullFlags(pub u8);
impl ETestInstanceDataObjectFullFlags {
    pub const NONE: ETestInstanceDataObjectFullFlags = ETestInstanceDataObjectFullFlags(
        0,
    );
    pub const FLAG0: ETestInstanceDataObjectFullFlags = ETestInstanceDataObjectFullFlags(
        1,
    );
    pub const FLAG1: ETestInstanceDataObjectFullFlags = ETestInstanceDataObjectFullFlags(
        2,
    );
    pub const FLAG2: ETestInstanceDataObjectFullFlags = ETestInstanceDataObjectFullFlags(
        4,
    );
    pub const FLAG4: ETestInstanceDataObjectFullFlags = ETestInstanceDataObjectFullFlags(
        16,
    );
    pub const FLAG5: ETestInstanceDataObjectFullFlags = ETestInstanceDataObjectFullFlags(
        32,
    );
    pub const FLAG6: ETestInstanceDataObjectFullFlags = ETestInstanceDataObjectFullFlags(
        64,
    );
    pub const FLAG7: ETestInstanceDataObjectFullFlags = ETestInstanceDataObjectFullFlags(
        128,
    );
}
#[repr(transparent)]
pub struct ETestInstanceDataObjectGrainAlternate(pub u8);
impl ETestInstanceDataObjectGrainAlternate {
    pub const NONE: ETestInstanceDataObjectGrainAlternate = ETestInstanceDataObjectGrainAlternate(
        0,
    );
    pub const CORN: ETestInstanceDataObjectGrainAlternate = ETestInstanceDataObjectGrainAlternate(
        1,
    );
    pub const RICE: ETestInstanceDataObjectGrainAlternate = ETestInstanceDataObjectGrainAlternate(
        2,
    );
    pub const RYE: ETestInstanceDataObjectGrainAlternate = ETestInstanceDataObjectGrainAlternate(
        3,
    );
    pub const WHEAT: ETestInstanceDataObjectGrainAlternate = ETestInstanceDataObjectGrainAlternate(
        4,
    );
}
#[repr(transparent)]
pub struct ETestInstanceDataObjectFruitAlternate(pub u8);
impl ETestInstanceDataObjectFruitAlternate {
    pub const NONE: ETestInstanceDataObjectFruitAlternate = ETestInstanceDataObjectFruitAlternate(
        0,
    );
    pub const APPLE: ETestInstanceDataObjectFruitAlternate = ETestInstanceDataObjectFruitAlternate(
        1,
    );
    pub const CHERRY: ETestInstanceDataObjectFruitAlternate = ETestInstanceDataObjectFruitAlternate(
        2,
    );
    pub const ORANGE: ETestInstanceDataObjectFruitAlternate = ETestInstanceDataObjectFruitAlternate(
        3,
    );
    pub const PEAR: ETestInstanceDataObjectFruitAlternate = ETestInstanceDataObjectFruitAlternate(
        4,
    );
}
#[repr(transparent)]
pub struct ETestInstanceDataObjectDirectionAlternate(pub u16);
impl ETestInstanceDataObjectDirectionAlternate {
    pub const NONE: ETestInstanceDataObjectDirectionAlternate = ETestInstanceDataObjectDirectionAlternate(
        0,
    );
    pub const UP: ETestInstanceDataObjectDirectionAlternate = ETestInstanceDataObjectDirectionAlternate(
        1,
    );
    pub const DOWN: ETestInstanceDataObjectDirectionAlternate = ETestInstanceDataObjectDirectionAlternate(
        2,
    );
    pub const NORTH: ETestInstanceDataObjectDirectionAlternate = ETestInstanceDataObjectDirectionAlternate(
        4,
    );
    pub const EAST: ETestInstanceDataObjectDirectionAlternate = ETestInstanceDataObjectDirectionAlternate(
        8,
    );
    pub const SOUTH: ETestInstanceDataObjectDirectionAlternate = ETestInstanceDataObjectDirectionAlternate(
        16,
    );
    pub const WEST: ETestInstanceDataObjectDirectionAlternate = ETestInstanceDataObjectDirectionAlternate(
        32,
    );
}
#[repr(transparent)]
pub struct ETestInstanceDataObjectGrainAlternateEnumClass(pub u8);
impl ETestInstanceDataObjectGrainAlternateEnumClass {
    pub const NONE: ETestInstanceDataObjectGrainAlternateEnumClass = ETestInstanceDataObjectGrainAlternateEnumClass(
        0,
    );
    pub const CORN: ETestInstanceDataObjectGrainAlternateEnumClass = ETestInstanceDataObjectGrainAlternateEnumClass(
        1,
    );
    pub const RICE: ETestInstanceDataObjectGrainAlternateEnumClass = ETestInstanceDataObjectGrainAlternateEnumClass(
        2,
    );
    pub const RYE: ETestInstanceDataObjectGrainAlternateEnumClass = ETestInstanceDataObjectGrainAlternateEnumClass(
        3,
    );
    pub const WHEAT: ETestInstanceDataObjectGrainAlternateEnumClass = ETestInstanceDataObjectGrainAlternateEnumClass(
        4,
    );
}
#[repr(transparent)]
pub struct ETestInstanceDataObjectFruitAlternateNamespace(pub u8);
impl ETestInstanceDataObjectFruitAlternateNamespace {
    pub const NONE: ETestInstanceDataObjectFruitAlternateNamespace = ETestInstanceDataObjectFruitAlternateNamespace(
        0,
    );
    pub const APPLE: ETestInstanceDataObjectFruitAlternateNamespace = ETestInstanceDataObjectFruitAlternateNamespace(
        1,
    );
    pub const CHERRY: ETestInstanceDataObjectFruitAlternateNamespace = ETestInstanceDataObjectFruitAlternateNamespace(
        2,
    );
    pub const ORANGE: ETestInstanceDataObjectFruitAlternateNamespace = ETestInstanceDataObjectFruitAlternateNamespace(
        3,
    );
    pub const PEAR: ETestInstanceDataObjectFruitAlternateNamespace = ETestInstanceDataObjectFruitAlternateNamespace(
        4,
    );
}
#[repr(transparent)]
pub struct EAxis(pub u8);
impl EAxis {
    pub const NONE: EAxis = EAxis(0);
    pub const X: EAxis = EAxis(1);
    pub const Y: EAxis = EAxis(2);
    pub const Z: EAxis = EAxis(3);
}
#[repr(transparent)]
pub struct EPixelFormat(pub u8);
impl EPixelFormat {
    pub const PF_UNKNOWN: EPixelFormat = EPixelFormat(0);
    pub const PF_A32B32G32R32F: EPixelFormat = EPixelFormat(1);
    pub const PF_B8G8R8A8: EPixelFormat = EPixelFormat(2);
    pub const PF_G8: EPixelFormat = EPixelFormat(3);
    pub const PF_G16: EPixelFormat = EPixelFormat(4);
    pub const PF_DXT1: EPixelFormat = EPixelFormat(5);
    pub const PF_DXT3: EPixelFormat = EPixelFormat(6);
    pub const PF_DXT5: EPixelFormat = EPixelFormat(7);
    pub const PF_UYVY: EPixelFormat = EPixelFormat(8);
    pub const PF_FLOAT_RGB: EPixelFormat = EPixelFormat(9);
    pub const PF_FLOAT_RGBA: EPixelFormat = EPixelFormat(10);
    pub const PF_DEPTH_STENCIL: EPixelFormat = EPixelFormat(11);
    pub const PF_SHADOW_DEPTH: EPixelFormat = EPixelFormat(12);
    pub const PF_R32_FLOAT: EPixelFormat = EPixelFormat(13);
    pub const PF_G16R16: EPixelFormat = EPixelFormat(14);
    pub const PF_G16R16F: EPixelFormat = EPixelFormat(15);
    pub const PF_G16R16F_FILTER: EPixelFormat = EPixelFormat(16);
    pub const PF_G32R32F: EPixelFormat = EPixelFormat(17);
    pub const PF_A2B10G10R10: EPixelFormat = EPixelFormat(18);
    pub const PF_A16B16G16R16: EPixelFormat = EPixelFormat(19);
    pub const PF_D24: EPixelFormat = EPixelFormat(20);
    pub const PF_R16F: EPixelFormat = EPixelFormat(21);
    pub const PF_R16F_FILTER: EPixelFormat = EPixelFormat(22);
    pub const PF_BC5: EPixelFormat = EPixelFormat(23);
    pub const PF_V8U8: EPixelFormat = EPixelFormat(24);
    pub const PF_A1: EPixelFormat = EPixelFormat(25);
    pub const PF_FLOAT_R11G11B10: EPixelFormat = EPixelFormat(26);
    pub const PF_A8: EPixelFormat = EPixelFormat(27);
    pub const PF_R32_UINT: EPixelFormat = EPixelFormat(28);
    pub const PF_R32_SINT: EPixelFormat = EPixelFormat(29);
    pub const PF_PVRTC2: EPixelFormat = EPixelFormat(30);
    pub const PF_PVRTC4: EPixelFormat = EPixelFormat(31);
    pub const PF_R16_UINT: EPixelFormat = EPixelFormat(32);
    pub const PF_R16_SINT: EPixelFormat = EPixelFormat(33);
    pub const PF_R16G16B16A16_UINT: EPixelFormat = EPixelFormat(34);
    pub const PF_R16G16B16A16_SINT: EPixelFormat = EPixelFormat(35);
    pub const PF_R5G6B5_UNORM: EPixelFormat = EPixelFormat(36);
    pub const PF_R8G8B8A8: EPixelFormat = EPixelFormat(37);
    pub const PF_A8R8G8B8: EPixelFormat = EPixelFormat(38);
    pub const PF_BC4: EPixelFormat = EPixelFormat(39);
    pub const PF_R8G8: EPixelFormat = EPixelFormat(40);
    pub const PF_ATC_RGB: EPixelFormat = EPixelFormat(41);
    pub const PF_ATC_RGBA_E: EPixelFormat = EPixelFormat(42);
    pub const PF_ATC_RGBA_I: EPixelFormat = EPixelFormat(43);
    pub const PF_X24_G8: EPixelFormat = EPixelFormat(44);
    pub const PF_ETC1: EPixelFormat = EPixelFormat(45);
    pub const PF_ETC2_RGB: EPixelFormat = EPixelFormat(46);
    pub const PF_ETC2_RGBA: EPixelFormat = EPixelFormat(47);
    pub const PF_R32G32B32A32_UINT: EPixelFormat = EPixelFormat(48);
    pub const PF_R16G16_UINT: EPixelFormat = EPixelFormat(49);
    pub const PF_ASTC_4X4: EPixelFormat = EPixelFormat(50);
    pub const PF_ASTC_6X6: EPixelFormat = EPixelFormat(51);
    pub const PF_ASTC_8X8: EPixelFormat = EPixelFormat(52);
    pub const PF_ASTC_10X10: EPixelFormat = EPixelFormat(53);
    pub const PF_ASTC_12X12: EPixelFormat = EPixelFormat(54);
    pub const PF_BC6H: EPixelFormat = EPixelFormat(55);
    pub const PF_BC7: EPixelFormat = EPixelFormat(56);
    pub const PF_R8_UINT: EPixelFormat = EPixelFormat(57);
    pub const PF_L8: EPixelFormat = EPixelFormat(58);
    pub const PF_XGXR8: EPixelFormat = EPixelFormat(59);
    pub const PF_R8G8B8A8_UINT: EPixelFormat = EPixelFormat(60);
    pub const PF_R8G8B8A8_SNORM: EPixelFormat = EPixelFormat(61);
    pub const PF_R16G16B16A16_UNORM: EPixelFormat = EPixelFormat(62);
    pub const PF_R16G16B16A16_SNORM: EPixelFormat = EPixelFormat(63);
    pub const PF_PLATFORM_HDR_0: EPixelFormat = EPixelFormat(64);
    pub const PF_PLATFORM_HDR_1: EPixelFormat = EPixelFormat(65);
    pub const PF_PLATFORM_HDR_2: EPixelFormat = EPixelFormat(66);
    pub const PF_NV12: EPixelFormat = EPixelFormat(67);
    pub const PF_R32G32_UINT: EPixelFormat = EPixelFormat(68);
    pub const PF_ETC2_R11_EAC: EPixelFormat = EPixelFormat(69);
    pub const PF_ETC2_RG11_EAC: EPixelFormat = EPixelFormat(70);
    pub const PF_R8: EPixelFormat = EPixelFormat(71);
    pub const PF_B5G5R5A1_UNORM: EPixelFormat = EPixelFormat(72);
    pub const PF_ASTC_4X4_HDR: EPixelFormat = EPixelFormat(73);
    pub const PF_ASTC_6X6_HDR: EPixelFormat = EPixelFormat(74);
    pub const PF_ASTC_8X8_HDR: EPixelFormat = EPixelFormat(75);
    pub const PF_ASTC_10X10_HDR: EPixelFormat = EPixelFormat(76);
    pub const PF_ASTC_12X12_HDR: EPixelFormat = EPixelFormat(77);
    pub const PF_G16R16_SNORM: EPixelFormat = EPixelFormat(78);
    pub const PF_R8G8_UINT: EPixelFormat = EPixelFormat(79);
    pub const PF_R32G32B32_UINT: EPixelFormat = EPixelFormat(80);
    pub const PF_R32G32B32_SINT: EPixelFormat = EPixelFormat(81);
    pub const PF_R32G32B32F: EPixelFormat = EPixelFormat(82);
    pub const PF_R8_SINT: EPixelFormat = EPixelFormat(83);
    pub const PF_R64_UINT: EPixelFormat = EPixelFormat(84);
    pub const PF_R9G9B9EXP5: EPixelFormat = EPixelFormat(85);
    pub const PF_P010: EPixelFormat = EPixelFormat(86);
    pub const PF_ASTC_4X4_NORM_RG: EPixelFormat = EPixelFormat(87);
    pub const PF_ASTC_6X6_NORM_RG: EPixelFormat = EPixelFormat(88);
    pub const PF_ASTC_8X8_NORM_RG: EPixelFormat = EPixelFormat(89);
    pub const PF_ASTC_10X10_NORM_RG: EPixelFormat = EPixelFormat(90);
    pub const PF_ASTC_12X12_NORM_RG: EPixelFormat = EPixelFormat(91);
    pub const PF_R16G16_SINT: EPixelFormat = EPixelFormat(92);
}
#[repr(transparent)]
pub struct ELifetimeCondition(pub u8);
impl ELifetimeCondition {
    pub const COND_NONE: ELifetimeCondition = ELifetimeCondition(0);
    pub const COND_INITIAL_ONLY: ELifetimeCondition = ELifetimeCondition(1);
    pub const COND_OWNER_ONLY: ELifetimeCondition = ELifetimeCondition(2);
    pub const COND_SKIP_OWNER: ELifetimeCondition = ELifetimeCondition(3);
    pub const COND_SIMULATED_ONLY: ELifetimeCondition = ELifetimeCondition(4);
    pub const COND_AUTONOMOUS_ONLY: ELifetimeCondition = ELifetimeCondition(5);
    pub const COND_SIMULATED_OR_PHYSICS: ELifetimeCondition = ELifetimeCondition(6);
    pub const COND_INITIAL_OR_OWNER: ELifetimeCondition = ELifetimeCondition(7);
    pub const COND_CUSTOM: ELifetimeCondition = ELifetimeCondition(8);
    pub const COND_REPLAY_OR_OWNER: ELifetimeCondition = ELifetimeCondition(9);
    pub const COND_REPLAY_ONLY: ELifetimeCondition = ELifetimeCondition(10);
    pub const COND_SIMULATED_ONLY_NO_REPLAY: ELifetimeCondition = ELifetimeCondition(11);
    pub const COND_SIMULATED_OR_PHYSICS_NO_REPLAY: ELifetimeCondition = ELifetimeCondition(
        12,
    );
    pub const COND_SKIP_REPLAY: ELifetimeCondition = ELifetimeCondition(13);
    pub const COND_DYNAMIC: ELifetimeCondition = ELifetimeCondition(14);
    pub const COND_NEVER: ELifetimeCondition = ELifetimeCondition(15);
    pub const COND_NET_GROUP: ELifetimeCondition = ELifetimeCondition(16);
}
#[repr(transparent)]
pub struct EInputDeviceTriggerMask(pub u8);
impl EInputDeviceTriggerMask {
    pub const NONE: EInputDeviceTriggerMask = EInputDeviceTriggerMask(0);
    pub const LEFT: EInputDeviceTriggerMask = EInputDeviceTriggerMask(1);
    pub const RIGHT: EInputDeviceTriggerMask = EInputDeviceTriggerMask(2);
    pub const ALL: EInputDeviceTriggerMask = EInputDeviceTriggerMask(3);
}
#[repr(transparent)]
pub struct EUnit(pub u8);
impl EUnit {
    pub const MICROMETERS: EUnit = EUnit(0);
    pub const MILLIMETERS: EUnit = EUnit(1);
    pub const CENTIMETERS: EUnit = EUnit(2);
    pub const METERS: EUnit = EUnit(3);
    pub const KILOMETERS: EUnit = EUnit(4);
    pub const INCHES: EUnit = EUnit(5);
    pub const FEET: EUnit = EUnit(6);
    pub const YARDS: EUnit = EUnit(7);
    pub const MILES: EUnit = EUnit(8);
    pub const LIGHTYEARS: EUnit = EUnit(9);
    pub const DEGREES: EUnit = EUnit(10);
    pub const RADIANS: EUnit = EUnit(11);
    pub const CENTIMETERS_PER_SECOND: EUnit = EUnit(12);
    pub const METERS_PER_SECOND: EUnit = EUnit(13);
    pub const KILOMETERS_PER_HOUR: EUnit = EUnit(14);
    pub const MILES_PER_HOUR: EUnit = EUnit(15);
    pub const DEGREES_PER_SECOND: EUnit = EUnit(16);
    pub const RADIANS_PER_SECOND: EUnit = EUnit(17);
    pub const CENTIMETERS_PER_SECOND_SQUARED: EUnit = EUnit(18);
    pub const METERS_PER_SECOND_SQUARED: EUnit = EUnit(19);
    pub const CELSIUS: EUnit = EUnit(20);
    pub const FARENHEIT: EUnit = EUnit(21);
    pub const KELVIN: EUnit = EUnit(22);
    pub const MICROGRAMS: EUnit = EUnit(23);
    pub const MILLIGRAMS: EUnit = EUnit(24);
    pub const GRAMS: EUnit = EUnit(25);
    pub const KILOGRAMS: EUnit = EUnit(26);
    pub const METRIC_TONS: EUnit = EUnit(27);
    pub const OUNCES: EUnit = EUnit(28);
    pub const POUNDS: EUnit = EUnit(29);
    pub const STONES: EUnit = EUnit(30);
    pub const GRAMS_PER_CUBIC_CENTIMETER: EUnit = EUnit(31);
    pub const GRAMS_PER_CUBIC_METER: EUnit = EUnit(32);
    pub const KILOGRAMS_PER_CUBIC_CENTIMETER: EUnit = EUnit(33);
    pub const KILOGRAMS_PER_CUBIC_METER: EUnit = EUnit(34);
    pub const NEWTONS: EUnit = EUnit(35);
    pub const POUNDS_FORCE: EUnit = EUnit(36);
    pub const KILOGRAMS_FORCE: EUnit = EUnit(37);
    pub const KILOGRAM_CENTIMETERS_PER_SECOND_SQUARED: EUnit = EUnit(38);
    pub const NEWTON_METERS: EUnit = EUnit(39);
    pub const KILOGRAM_CENTIMETERS_SQUARED_PER_SECOND_SQUARED: EUnit = EUnit(40);
    pub const NEWTON_SECONDS: EUnit = EUnit(41);
    pub const KILOGRAM_CENTIMETERS: EUnit = EUnit(42);
    pub const KILOGRAM_METERS: EUnit = EUnit(43);
    pub const HERTZ: EUnit = EUnit(44);
    pub const KILOHERTZ: EUnit = EUnit(45);
    pub const MEGAHERTZ: EUnit = EUnit(46);
    pub const GIGAHERTZ: EUnit = EUnit(47);
    pub const REVOLUTIONS_PER_MINUTE: EUnit = EUnit(48);
    pub const BYTES: EUnit = EUnit(49);
    pub const KILOBYTES: EUnit = EUnit(50);
    pub const MEGABYTES: EUnit = EUnit(51);
    pub const GIGABYTES: EUnit = EUnit(52);
    pub const TERABYTES: EUnit = EUnit(53);
    pub const LUMENS: EUnit = EUnit(54);
    pub const CANDELA: EUnit = EUnit(55);
    pub const LUX: EUnit = EUnit(56);
    pub const CANDELA_PER_METER2: EUnit = EUnit(57);
    pub const EXPOSURE_VALUE: EUnit = EUnit(59);
    pub const NANOSECONDS: EUnit = EUnit(60);
    pub const MICROSECONDS: EUnit = EUnit(61);
    pub const MILLISECONDS: EUnit = EUnit(62);
    pub const SECONDS: EUnit = EUnit(63);
    pub const MINUTES: EUnit = EUnit(64);
    pub const HOURS: EUnit = EUnit(65);
    pub const DAYS: EUnit = EUnit(66);
    pub const MONTHS: EUnit = EUnit(67);
    pub const YEARS: EUnit = EUnit(68);
    pub const PIXELS_PER_INCH: EUnit = EUnit(69);
    pub const PERCENTAGE: EUnit = EUnit(70);
    pub const MULTIPLIER: EUnit = EUnit(71);
    pub const PASCALS: EUnit = EUnit(72);
    pub const KILO_PASCALS: EUnit = EUnit(73);
    pub const MEGA_PASCALS: EUnit = EUnit(74);
    pub const GIGA_PASCALS: EUnit = EUnit(75);
    pub const UNSPECIFIED: EUnit = EUnit(76);
}
#[repr(transparent)]
pub struct EDataValidationUsecase(pub u8);
impl EDataValidationUsecase {
    pub const NONE: EDataValidationUsecase = EDataValidationUsecase(0);
    pub const MANUAL: EDataValidationUsecase = EDataValidationUsecase(1);
    pub const COMMANDLET: EDataValidationUsecase = EDataValidationUsecase(2);
    pub const SAVE: EDataValidationUsecase = EDataValidationUsecase(3);
    pub const PRE_SUBMIT: EDataValidationUsecase = EDataValidationUsecase(4);
    pub const SCRIPT: EDataValidationUsecase = EDataValidationUsecase(5);
}
#[repr(transparent)]
pub struct EPropertyAccessChangeNotifyMode(pub u8);
impl EPropertyAccessChangeNotifyMode {
    pub const DEFAULT: EPropertyAccessChangeNotifyMode = EPropertyAccessChangeNotifyMode(
        0,
    );
    pub const NEVER: EPropertyAccessChangeNotifyMode = EPropertyAccessChangeNotifyMode(
        1,
    );
    pub const ALWAYS: EPropertyAccessChangeNotifyMode = EPropertyAccessChangeNotifyMode(
        2,
    );
}
#[repr(transparent)]
pub struct EMouseCursor(pub u8);
impl EMouseCursor {
    pub const NONE: EMouseCursor = EMouseCursor(0);
    pub const DEFAULT: EMouseCursor = EMouseCursor(1);
    pub const TEXT_EDIT_BEAM: EMouseCursor = EMouseCursor(2);
    pub const RESIZE_LEFT_RIGHT: EMouseCursor = EMouseCursor(3);
    pub const RESIZE_UP_DOWN: EMouseCursor = EMouseCursor(4);
    pub const RESIZE_SOUTH_EAST: EMouseCursor = EMouseCursor(5);
    pub const RESIZE_SOUTH_WEST: EMouseCursor = EMouseCursor(6);
    pub const CARDINAL_CROSS: EMouseCursor = EMouseCursor(7);
    pub const CROSSHAIRS: EMouseCursor = EMouseCursor(8);
    pub const HAND: EMouseCursor = EMouseCursor(9);
    pub const GRAB_HAND: EMouseCursor = EMouseCursor(10);
    pub const GRAB_HAND_CLOSED: EMouseCursor = EMouseCursor(11);
    pub const SLASHED_CIRCLE: EMouseCursor = EMouseCursor(12);
    pub const EYE_DROPPER: EMouseCursor = EMouseCursor(13);
    pub const CUSTOM: EMouseCursor = EMouseCursor(14);
}
#[repr(transparent)]
pub struct ESearchCase(pub u8);
impl ESearchCase {
    pub const CASE_SENSITIVE: ESearchCase = ESearchCase(0);
    pub const IGNORE_CASE: ESearchCase = ESearchCase(1);
}
#[repr(transparent)]
pub struct ESearchDir(pub u8);
impl ESearchDir {
    pub const FROM_START: ESearchDir = ESearchDir(0);
    pub const FROM_END: ESearchDir = ESearchDir(1);
}
#[repr(transparent)]
pub struct EDataValidationResult(pub u8);
impl EDataValidationResult {
    pub const INVALID: EDataValidationResult = EDataValidationResult(0);
    pub const VALID: EDataValidationResult = EDataValidationResult(1);
    pub const NOT_VALIDATED: EDataValidationResult = EDataValidationResult(2);
}
#[repr(transparent)]
pub struct EAppMsgType(pub u8);
impl EAppMsgType {
    pub const OK: EAppMsgType = EAppMsgType(0);
    pub const YES_NO: EAppMsgType = EAppMsgType(1);
    pub const OK_CANCEL: EAppMsgType = EAppMsgType(2);
    pub const YES_NO_CANCEL: EAppMsgType = EAppMsgType(3);
    pub const CANCEL_RETRY_CONTINUE: EAppMsgType = EAppMsgType(4);
    pub const YES_NO_YES_ALL_NO_ALL: EAppMsgType = EAppMsgType(5);
    pub const YES_NO_YES_ALL_NO_ALL_CANCEL: EAppMsgType = EAppMsgType(6);
    pub const YES_NO_YES_ALL: EAppMsgType = EAppMsgType(7);
}
#[repr(transparent)]
pub struct EAppReturnType(pub u8);
impl EAppReturnType {
    pub const NO: EAppReturnType = EAppReturnType(0);
    pub const YES: EAppReturnType = EAppReturnType(1);
    pub const YES_ALL: EAppReturnType = EAppReturnType(2);
    pub const NO_ALL: EAppReturnType = EAppReturnType(3);
    pub const CANCEL: EAppReturnType = EAppReturnType(4);
    pub const OK: EAppReturnType = EAppReturnType(5);
    pub const RETRY: EAppReturnType = EAppReturnType(6);
    pub const CONTINUE: EAppReturnType = EAppReturnType(7);
}
#[repr(transparent)]
pub struct EAppMsgCategory(pub u8);
impl EAppMsgCategory {
    pub const WARNING: EAppMsgCategory = EAppMsgCategory(0);
    pub const ERROR: EAppMsgCategory = EAppMsgCategory(1);
    pub const SUCCESS: EAppMsgCategory = EAppMsgCategory(2);
    pub const INFO: EAppMsgCategory = EAppMsgCategory(3);
}
#[repr(transparent)]
pub struct EUserDefinedStructureStatus(pub u8);
impl EUserDefinedStructureStatus {
    pub const UDSS_UP_TO_DATE: EUserDefinedStructureStatus = EUserDefinedStructureStatus(
        0,
    );
    pub const UDSS_DIRTY: EUserDefinedStructureStatus = EUserDefinedStructureStatus(1);
    pub const UDSS_ERROR: EUserDefinedStructureStatus = EUserDefinedStructureStatus(2);
    pub const UDSS_DUPLICATE: EUserDefinedStructureStatus = EUserDefinedStructureStatus(
        3,
    );
}
#[repr(transparent)]
pub struct EVerseEffectSet(pub u8);
impl EVerseEffectSet {
    pub const NONE: EVerseEffectSet = EVerseEffectSet(0);
    pub const SUSPENDS: EVerseEffectSet = EVerseEffectSet(1);
    pub const DECIDES: EVerseEffectSet = EVerseEffectSet(2);
    pub const DIVERGES: EVerseEffectSet = EVerseEffectSet(4);
    pub const READS: EVerseEffectSet = EVerseEffectSet(8);
    pub const WRITES: EVerseEffectSet = EVerseEffectSet(16);
    pub const ALLOCATES: EVerseEffectSet = EVerseEffectSet(32);
    pub const NO_ROLLBACK: EVerseEffectSet = EVerseEffectSet(64);
}
#[repr(transparent)]
pub struct EVerseEnumFlags(pub u32);
impl EVerseEnumFlags {
    pub const NONE: EVerseEnumFlags = EVerseEnumFlags(0);
    pub const NATIVE_BOUND: EVerseEnumFlags = EVerseEnumFlags(1);
    pub const UHT_NATIVE: EVerseEnumFlags = EVerseEnumFlags(2);
}
#[repr(transparent)]
pub struct EAxisList(pub u8);
impl EAxisList {
    pub const NONE: EAxisList = EAxisList(0);
    pub const X: EAxisList = EAxisList(1);
    pub const Y: EAxisList = EAxisList(2);
    pub const Z: EAxisList = EAxisList(4);
    pub const SCREEN: EAxisList = EAxisList(8);
    pub const XY: EAxisList = EAxisList(3);
    pub const XZ: EAxisList = EAxisList(5);
    pub const YZ: EAxisList = EAxisList(6);
    pub const XYZ: EAxisList = EAxisList(7);
    pub const ALL: EAxisList = EAxisList(15);
    pub const Z_ROTATION: EAxisList = EAxisList(6);
    pub const ROTATE2_D: EAxisList = EAxisList(8);
    pub const LEFT: EAxisList = EAxisList(16);
    pub const UP: EAxisList = EAxisList(32);
    pub const FORWARD: EAxisList = EAxisList(64);
    pub const LU: EAxisList = EAxisList(48);
    pub const LF: EAxisList = EAxisList(80);
    pub const UF: EAxisList = EAxisList(96);
    pub const LEFT_UP_FORWARD: EAxisList = EAxisList(112);
}
#[repr(transparent)]
pub struct EInputDeviceMappingPolicy(pub i32);
impl EInputDeviceMappingPolicy {
    pub const INVALID: EInputDeviceMappingPolicy = EInputDeviceMappingPolicy(-1);
    pub const USE_MANAGED_PLATFORM_LOGIN: EInputDeviceMappingPolicy = EInputDeviceMappingPolicy(
        0,
    );
    pub const PRIMARY_USER_SHARES_KEYBOARD_AND_FIRST_GAMEPAD: EInputDeviceMappingPolicy = EInputDeviceMappingPolicy(
        1,
    );
    pub const CREATE_UNIQUE_PLATFORM_USER_FOR_EACH_DEVICE: EInputDeviceMappingPolicy = EInputDeviceMappingPolicy(
        2,
    );
    pub const MAP_ALL_DEVICES_TO_PRIMARY_USER: EInputDeviceMappingPolicy = EInputDeviceMappingPolicy(
        3,
    );
}
#[repr(transparent)]
pub struct ELogTimes(pub u8);
impl ELogTimes {
    pub const NONE: ELogTimes = ELogTimes(0);
    pub const UTC: ELogTimes = ELogTimes(1);
    pub const SINCE_G_START_TIME: ELogTimes = ELogTimes(2);
    pub const LOCAL: ELogTimes = ELogTimes(3);
}
