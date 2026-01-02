use std::ffi::c_void;

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
pub struct WeakObject {
    pub object_index: i32,
    pub object_serial_number: i32,
}

#[repr(transparent)]
pub struct TWeakObjectPtr<T> {
    weak_object: WeakObject,
    _marker: std::marker::PhantomData<T>,
}

#[repr(C, align(8))]
pub struct ScriptArray {
    pub data: *mut c_void,
    pub num: i32,
    pub max: i32,
}
#[repr(transparent)]
pub struct TArray<T> {
    script_array: ScriptArray,
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
