#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::prelude::*;
#[repr(C, align(8))]
pub struct FNiagaraVariableCommonReference {
    pub name: FName,
    pub underlying_type: UPtr<UObject>,
}
#[repr(C, align(8))]
pub struct FNiagaraCompileHash {
    pub data_hash: TArray<u8>,
}
pub struct UNiagaraNotifyOnChanged {}
pub struct UNiagaraDataInterfaceBase {}
pub struct UNiagaraMergeable {
    pub merge_id: FGuid,
}
