#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(8))]
pub struct FNiagaraVariableCommonReference {
    pub name: FName,
    pub underlying_type: UPtr<crate::bindings::core_u_object::UObject>,
}
#[repr(C, align(8))]
pub struct FNiagaraCompileHash {
    pub data_hash: TArray<u8>,
}
pub struct UNiagaraNotifyOnChanged {}
pub struct UNiagaraDataInterfaceBase {}
pub struct UNiagaraMergeable {
    pub merge_id: crate::bindings::core_u_object::FGuid,
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ENiagaraIterationSource(pub u8);
impl ENiagaraIterationSource {
    pub const PARTICLES: ENiagaraIterationSource = ENiagaraIterationSource(0);
    pub const DATA_INTERFACE: ENiagaraIterationSource = ENiagaraIterationSource(1);
    pub const DIRECT_SET: ENiagaraIterationSource = ENiagaraIterationSource(2);
}
