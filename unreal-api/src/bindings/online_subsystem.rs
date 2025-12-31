#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(8))]
pub struct FNamedInterface {
    pub interface_name: FName,
    pub interface_object: UPtr<crate::bindings::core_u_object::UObject>,
}
#[repr(C, align(8))]
pub struct FNamedInterfaceDef {
    pub interface_name: FName,
    pub interface_class_name: FString,
}
pub struct UNamedInterfaces {
    pub named_interfaces: TArray<FNamedInterface>,
    pub named_interface_defs: TArray<FNamedInterfaceDef>,
}
pub struct UTurnBasedMatchInterface {}
pub struct ITurnBasedMatchInterface {}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EMPMatchOutcome(pub u8);
impl EMPMatchOutcome {
    pub const NONE: EMPMatchOutcome = EMPMatchOutcome(0);
    pub const QUIT: EMPMatchOutcome = EMPMatchOutcome(1);
    pub const WON: EMPMatchOutcome = EMPMatchOutcome(2);
    pub const LOST: EMPMatchOutcome = EMPMatchOutcome(3);
    pub const TIED: EMPMatchOutcome = EMPMatchOutcome(4);
    pub const TIME_EXPIRED: EMPMatchOutcome = EMPMatchOutcome(5);
    pub const FIRST: EMPMatchOutcome = EMPMatchOutcome(6);
    pub const SECOND: EMPMatchOutcome = EMPMatchOutcome(7);
    pub const THIRD: EMPMatchOutcome = EMPMatchOutcome(8);
    pub const FOURTH: EMPMatchOutcome = EMPMatchOutcome(9);
}
