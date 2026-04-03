use unreal_api::ffi::{MassFragmentRequirement, MassSystemDescriptor, Utf8Str};
use unreal_api::mass::{MassSystemRegistration, registered_mass_systems};

/// Returns the number of dynamically registered mass systems.
pub unsafe extern "C" fn get_mass_system_count() -> u32 {
    let mut count = 0u32;
    for _ in registered_mass_systems() {
        count += 1;
    }
    count
}

/// Fills a MassSystemDescriptor for the system at `index`.
/// Returns 1 on success, 0 on failure (index out of range).
pub unsafe extern "C" fn get_mass_system_descriptor(
    index: u32,
    out: *mut MassSystemDescriptor,
) -> u32 {
    if out.is_null() {
        return 0;
    }

    let Some(registration) = registered_mass_systems().into_iter().nth(index as usize) else {
        return 0;
    };

    // Build requirements array — we need stable pointers, so use a static-lifetime
    // reference from the registration (requirements is &'static [MassSystemRequirement]).
    let requirements: Vec<MassFragmentRequirement> = registration
        .requirements
        .iter()
        .map(|req| MassFragmentRequirement {
            cpp_type_name: Utf8Str::from(req.cpp_type_name),
            size: req.size as u32,
            alignment: req.align as u32,
            access_mode: req.access_mode,
            is_tag: req.is_tag,
            query_scope: req.query_scope,
            _padding: 0,
        })
        .collect();

    // Leak the vec to get a stable pointer for C++ to read.
    // This is called once at init, so the leak is acceptable.
    let requirements_ptr = if requirements.is_empty() {
        std::ptr::null()
    } else {
        let boxed = requirements.into_boxed_slice();
        let ptr = boxed.as_ptr();
        std::mem::forget(boxed);
        ptr
    };

    unsafe {
        (*out) = MassSystemDescriptor {
            name: Utf8Str::from(registration.name),
            num_requirements: registration.requirements.len() as u32,
            requirements: requirements_ptr,
            execute_fn: registration.execute_fn,
        };
    }

    1
}
