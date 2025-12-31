#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EVTProducerPriority(pub u8);
impl EVTProducerPriority {
    pub const LOWEST: EVTProducerPriority = EVTProducerPriority(0);
    pub const LOWER: EVTProducerPriority = EVTProducerPriority(1);
    pub const LOW: EVTProducerPriority = EVTProducerPriority(2);
    pub const BELOW_NORMAL: EVTProducerPriority = EVTProducerPriority(3);
    pub const NORMAL: EVTProducerPriority = EVTProducerPriority(4);
    pub const ABOVE_NORMAL: EVTProducerPriority = EVTProducerPriority(5);
    pub const HIGH: EVTProducerPriority = EVTProducerPriority(6);
    pub const HIGHEST: EVTProducerPriority = EVTProducerPriority(7);
    pub const COUNT: EVTProducerPriority = EVTProducerPriority(8);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EVTInvalidatePriority(pub u8);
impl EVTInvalidatePriority {
    pub const NORMAL: EVTInvalidatePriority = EVTInvalidatePriority(0);
    pub const HIGH: EVTInvalidatePriority = EVTInvalidatePriority(1);
    pub const COUNT: EVTInvalidatePriority = EVTInvalidatePriority(2);
}
