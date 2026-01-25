use std::ffi::c_void;

use glam::DVec3;

use crate::bindings::core_u_object::{FQuat, FTransform, FVector};

// TODO: This whole file is extremely handwavy and needs to be done properly by getting the correct
// layout from the reflection. Right now most of the layout is just guess work based on my machine

#[repr(C)]
pub struct FName {
    pub comparison_index: u32,
    pub number: u32,
    // TODO: This is wrong. This only exists in editor builds
    pub display_index: u32,
}

#[repr(transparent)]
pub struct UPtr<T> {
    pub ptr: *mut T,
}

#[repr(C)]
pub struct FWeakObject {
    pub object_index: i32,
    pub object_serial_number: i32,
}

#[repr(transparent)]
pub struct TWeakObjectPtr<T> {
    weak_object: FWeakObject,
    _marker: std::marker::PhantomData<T>,
}

#[repr(C, align(8))]
pub struct FScriptArray {
    pub data: *mut c_void,
    pub num: i32,
    pub max: i32,
}
#[repr(transparent)]
pub struct TArray<T> {
    script_array: FScriptArray,
    // TODO: Layout
    _marker: std::marker::PhantomData<T>,
}

// TODO: Generate
#[repr(C, align(8))]
pub struct FScriptSet {
    __opague: [u8; 80],
}

#[repr(transparent)]
pub struct TSet<T> {
    pub script_set: FScriptSet,
    _marker: std::marker::PhantomData<T>,
}

#[repr(C, align(8))]
pub struct FScriptMap {
    __opague: [u8; 80],
}

#[repr(transparent)]
pub struct TMap<Key, Value> {
    sparse_array: FScriptMap,
    _key_marker: std::marker::PhantomData<Key>,
    _value_marker: std::marker::PhantomData<Value>,
}

#[repr(C)]
pub struct FString {
    pub data: *mut u16,
    pub num: i32,
    pub max: i32,
}

// TODO: Layout
pub struct FField;

#[repr(C, align(8))]
pub struct FFieldPath {
    __opague: [u8; 48],
}

#[repr(transparent)]
pub struct TFieldPath<T> {
    field_path: FFieldPath,
    _marker: std::marker::PhantomData<T>,
}

// TODO: Layout
pub struct FMapProperty;
// TODO: Layout
pub struct FInt64Property;
pub struct FDoubleProperty;
pub struct FProperty;
pub struct FStructProperty;
pub struct FArrayProperty;
pub struct FMulticastDelegateProperty;

#[repr(C)]
pub struct TSubclassOf<T> {
    class: *mut c_void,
    _marker: std::marker::PhantomData<T>,
}

#[repr(C)]
pub struct TOptional<T> {
    pub is_set: bool,
    // TODO: this is bad and we might need maybeunit here
    pub data: T,
}

#[repr(C, align(8))]
pub struct FSoftObjectPtr {
    __opague: [u8; 48],
}

#[repr(transparent)]
pub struct TSoftObjectPtr<T> {
    soft_object_ptr: FSoftObjectPtr,
    _marker: std::marker::PhantomData<T>,
}

#[repr(transparent)]
pub struct TLazyObjectPtr<T> {
    // TODO: Layout
    _marker: std::marker::PhantomData<T>,
}

#[repr(transparent)]
pub struct FUtf8String {
    // TODO: Layout
}

#[repr(transparent)]
pub struct FAnsiString {
    // TODO: Layout
}

// TODO: Verify or make opague
#[repr(C, align(8))]
pub struct FScriptInterface {
    pub object: *mut c_void,
    pub interface: *mut c_void,
}

#[repr(transparent)]
pub struct TScriptInterface<T> {
    interface: FScriptInterface,
    _marker: std::marker::PhantomData<T>,
}

#[repr(C, align(16))]
pub struct StackAlloc<const N: usize> {
    stack: [u8; N],
}

impl<const N: usize> StackAlloc<N> {
    pub fn new() -> Self {
        Self { stack: [0; N] }
    }

    pub fn buffer_mut(&mut self) -> *mut c_void {
        self.stack.as_mut_ptr() as *mut c_void
    }
}

impl<const N: usize> Default for StackAlloc<N> {
    fn default() -> Self {
        Self::new()
    }
}

impl FVector {
    pub const ONE: FVector = FVector {
        x: 1.0,
        y: 1.0,
        z: 1.0,
    };
    pub const ZERO: FVector = FVector {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };
}

impl FQuat {
    pub const IDENTITY: FQuat = FQuat {
        x: 0.0,
        y: 0.0,
        z: 0.0,
        w: 1.0,
    };
}

impl Default for FQuat {
    fn default() -> Self {
        Self::IDENTITY
    }
}

impl From<glam::DQuat> for FQuat {
    fn from(value: glam::DQuat) -> Self {
        let arr = value.as_ref();
        FQuat {
            x: arr[0],
            y: arr[1],
            z: arr[2],
            w: arr[3],
        }
    }
}

impl From<DVec3> for FVector {
    fn from(value: DVec3) -> Self {
        Self {
            x: value.x,
            y: value.y,
            z: value.z,
        }
    }
}

impl FTransform {
    pub fn new(rotation: FQuat, translation: FVector, scale: FVector) -> Self {
        FTransform {
            translation,
            rotation,
            __padding_64: [0; 8],
            scale3_d: scale,
        }
    }
}

impl Default for FTransform {
    fn default() -> Self {
        FTransform::new(FQuat::IDENTITY, FVector::ZERO, FVector::ONE)
    }
}
