pub struct UClassRegistration {
    pub register_fn: fn(),
}

inventory::collect!(UClassRegistration);

pub fn register_all_uclasses() {
    for reg in inventory::iter::<UClassRegistration> {
        (reg.register_fn)();
    }
}
