#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(8))]
pub struct UNiagaraNotifyOnChanged {
    __padding_end: [u8; 80],
}
impl UNiagaraNotifyOnChanged {}
#[repr(C, align(8))]
pub struct UNiagaraDataInterfaceBase {
    __padding_end: [u8; 80],
}
impl UNiagaraDataInterfaceBase {}
#[repr(C, align(8))]
pub struct UNiagaraMergeable {
    __padding_end: [u8; 96],
}
impl UNiagaraMergeable {}
#[repr(transparent)]
pub struct ENiagaraIterationSource(pub u8);
impl ENiagaraIterationSource {
    pub const PARTICLES: ENiagaraIterationSource = ENiagaraIterationSource(0);
    pub const DATA_INTERFACE: ENiagaraIterationSource = ENiagaraIterationSource(1);
    pub const DIRECT_SET: ENiagaraIterationSource = ENiagaraIterationSource(2);
}
