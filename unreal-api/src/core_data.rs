use std::ffi::c_void;

use glam::DVec3;
use unreal_ffi::{FRustScriptArray, FRustString};

use crate::{
    bindings::core_u_object::{FQuat, FTransform, FVector, UClass},
    module::bindings,
};

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

impl<T> UPtr<T> {
    pub fn null() -> Self {
        Self {
            ptr: std::ptr::null_mut(),
        }
    }
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

#[repr(transparent)]
pub struct TArray<T> {
    script_array: FRustScriptArray,
    _marker: std::marker::PhantomData<T>,
}

impl<T> Drop for TArray<T> {
    fn drop(&mut self) {
        unsafe {
            (bindings().fscript_array_fns.dtor)(&mut self.script_array);
        }
    }
}

impl<T> TArray<T> {
    const ALIGN: u32 = std::mem::align_of::<T>() as u32;
    const SIZE: i32 = std::mem::size_of::<T>() as i32;

    pub fn new() -> Self {
        let mut script_array = FRustScriptArray::uninit();
        unsafe {
            (bindings().fscript_array_fns.ctor)(&mut script_array);
        }
        Self {
            script_array,
            _marker: std::marker::PhantomData,
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = &T> {
        // TODO: impl non bounds check access
        (0..self.num()).flat_map(|idx| self.get(idx))
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        if self.num() <= index {
            return None;
        }

        unsafe {
            let mut data_ptr: *mut c_void = std::ptr::null_mut();

            (bindings().fscript_array_fns.get_data)(
                &self.script_array as *const _ as *mut _,
                &mut data_ptr,
            );
            data_ptr.cast::<T>().add(index).as_ref()
        }
    }

    pub fn num(&self) -> usize {
        unsafe {
            let mut num = 0;
            (bindings().fscript_array_fns.num)(&self.script_array, &mut num);
            num as usize
        }
    }

    pub fn add(&mut self, value: T) {
        unsafe {
            let mut index = 0;
            (bindings().fscript_array_fns.add)(
                &mut self.script_array,
                1,
                Self::SIZE,
                Self::ALIGN,
                &mut index,
            );

            let mut data_ptr: *mut c_void = std::ptr::null_mut();
            (bindings().fscript_array_fns.get_data)(&mut self.script_array, &mut data_ptr);
            *data_ptr.cast::<T>().add(index as usize) = value;
        }
    }
}

impl<T> Default for TArray<T> {
    fn default() -> Self {
        Self::new()
    }
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

#[repr(transparent)]
pub struct FString {
    ffi: FRustString,
}

impl<'a> From<&'a str> for FString {
    fn from(value: &'a str) -> Self {
        unsafe {
            let mut rust_fstring = FRustString::uninit();
            (bindings().core_fns.new_fstring_from_utf8)(
                unreal_ffi::Utf8Str::from(value),
                &mut rust_fstring,
            );
            FString { ffi: rust_fstring }
        }
    }
}

impl Drop for FString {
    fn drop(&mut self) {
        unsafe {
            (bindings().core_fns.delete_fstring)(&mut self.ffi);
        }
    }
}

impl Clone for FString {
    fn clone(&self) -> Self {
        let mut cloned = FRustString::uninit();

        unsafe {
            (bindings().core_fns.copy_from_fstring)(&self.ffi, &mut cloned);
        }

        FString { ffi: cloned }
    }
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
    class: UPtr<UClass>,
    _marker: std::marker::PhantomData<T>,
}

impl<T> From<UPtr<UClass>> for TSubclassOf<T> {
    fn from(class: UPtr<UClass>) -> Self {
        TSubclassOf {
            class,
            _marker: std::marker::PhantomData,
        }
    }
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
