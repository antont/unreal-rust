use std::ffi::c_void;

#[repr(C)]
pub struct FName {
    pub comparison_index: u32,
    pub number: u32,
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

#[repr(C)]
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
#[repr(C)]
pub struct FScriptSet {
    pub elements: *mut u8,
    pub num_elements: i32,
    pub max_elements: i32,
}

#[repr(transparent)]
pub struct TSet<T> {
    pub script_set: FScriptSet,
    _marker: std::marker::PhantomData<T>,
}

#[repr(C)]
pub struct FScriptSparseArray {
    pub data: *mut u8,
    pub num_elements: i32,
    pub max_elements: i32,
    pub free_indices: *mut i32,
}

#[repr(transparent)]
pub struct TMap<Key, Value> {
    sparse_array: FScriptSparseArray,
    _key_marker: std::marker::PhantomData<Key>,
    _value_marker: std::marker::PhantomData<Value>,
}

#[repr(C)]
pub struct FString {
    pub data: *mut u16,
    pub num: i32,
    pub max: i32,
}

#[repr(transparent)]
pub struct TFieldPath<T> {
    // TODO: Layout
    _marker: std::marker::PhantomData<T>,
}

// TODO: Layout
pub struct FMapProperty;
// TODO: Layout
pub struct FInt64Property;
pub struct FDoubleProperty;
pub struct FProperty;

#[repr(C)]
pub struct TSubclassOf<T> {
    class: *mut c_void,
    _marker: std::marker::PhantomData<T>,
}

#[repr(transparent)]
pub struct TOptional<T> {
    // TODO: Layout
    _marker: std::marker::PhantomData<T>,
}

#[repr(transparent)]
pub struct TSoftObjectPtr<T> {
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
