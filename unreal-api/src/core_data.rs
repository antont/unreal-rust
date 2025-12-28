use std::ffi::c_void;

#[repr(C)]
pub struct FName {
    pub comparison_index: u32,
    pub number: u32,
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

#[repr(transparent)]
pub struct TSet<T> {
    // TODO: Layout
    _marker: std::marker::PhantomData<T>,
}

#[repr(transparent)]
pub struct TMap<Key, Value> {
    // TODO: Layout
    _key_marker: std::marker::PhantomData<Key>,
    _value_marker: std::marker::PhantomData<Value>,
}
