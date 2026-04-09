use std::sync::OnceLock;

use unreal_api::ffi::{MassFragmentRequirement, MassSystemDescriptor, Utf8Str};
use unreal_api::mass::{
    MassBevySystemRegistration, MassDeltaTime, MassSchedule, MassSpatialQueryCallback,
    MassSystemRegistration, MassSystemStage,
    registered_bevy_mass_systems, registered_mass_systems,
};
use unreal_api::ecs::schedule::IntoScheduleConfigs;

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
            order: registration.order,
            requirements: requirements_ptr,
            execute_fn: registration.execute_fn,
        };
    }

    1
}

// ---------------------------------------------------------------------------
// Bevy-scheduled dispatch
// ---------------------------------------------------------------------------

static MASS_SCHEDULE: OnceLock<std::sync::Mutex<MassSchedule>> = OnceLock::new();

/// Build a MassSchedule from all Bevy-registered mass systems.
///
/// All systems (including collision pre-pass) are auto-discovered via inventory.
/// Each gets a sequential stage (0, 1, 2, ...) in discovery order.
pub fn build_bevy_schedule() -> MassSchedule {
    let mut sched = MassSchedule::new();
    let regs: Vec<&MassBevySystemRegistration> =
        registered_bevy_mass_systems().into_iter().collect();

    // Sequential stage ordering: stage i runs after stage i-1
    for i in 1..regs.len() {
        sched
            .schedule_mut()
            .configure_sets(MassSystemStage(i as u32).after(MassSystemStage((i - 1) as u32)));
    }

    // Init and add all registered systems
    for (i, reg) in regs.iter().enumerate() {
        (reg.init_resources)(sched.world_mut());
        (reg.add_to_schedule)(sched.schedule_mut(), MassSystemStage(i as u32));
    }

    // Resource for collision pre-pass spatial query callback (populated per-frame)
    sched.world_mut().insert_resource(MassSpatialQueryCallback::default());

    sched
}

/// Initialize the global Bevy schedule. Called once during module init.
pub fn init_global_schedule() {
    MASS_SCHEDULE.get_or_init(|| std::sync::Mutex::new(build_bevy_schedule()));
}

/// Per-frame dispatch: update chunk resources and run the Bevy schedule.
///
/// # Safety
/// `data` must point to a valid `MassFrameDispatchData`.
pub unsafe extern "C" fn mass_frame_dispatch(
    data: *const unreal_api::ffi::MassFrameDispatchData,
) {
    if data.is_null() {
        return;
    }
    let data = unsafe { &*data };

    let Some(mutex) = MASS_SCHEDULE.get() else {
        return;
    };
    let Ok(mut sched) = mutex.lock() else {
        return;
    };

    sched.set_dt(data.dt);

    // Update spatial query callback from dispatch data
    {
        let mut spatial = sched.world_mut().resource_mut::<MassSpatialQueryCallback>();
        spatial.query_fn = data.spatial_query_fn;
        spatial.pickup_radius = data.pickup_radius;
    }

    // Collect Bevy registrations (may include Bevy-only systems like collision prepass)
    let bevy_regs: Vec<&MassBevySystemRegistration> =
        registered_bevy_mass_systems().into_iter().collect();

    // Build name→bevy_index lookup so we can map C++ system_index → bevy registration
    let bevy_name_to_idx: std::collections::HashMap<&str, usize> = bevy_regs
        .iter()
        .enumerate()
        .map(|(i, reg)| (reg.name, i))
        .collect();

    // C++ registrations in discovery order — system_index i maps to cpp_regs[i].name
    let cpp_regs: Vec<&MassSystemRegistration> =
        registered_mass_systems().into_iter().collect();

    // Clear all chunk resources
    for reg in &bevy_regs {
        (reg.clear_resources)(sched.world_mut());
    }

    // Populate from dispatch data — map C++ system_index → name → Bevy registration
    let batches = unsafe {
        std::slice::from_raw_parts(data.systems, data.num_systems as usize)
    };
    for batch in batches {
        let cpp_idx = batch.system_index as usize;
        if let Some(cpp_reg) = cpp_regs.get(cpp_idx) {
            if let Some(&bevy_idx) = bevy_name_to_idx.get(cpp_reg.name) {
                unsafe { (bevy_regs[bevy_idx].populate_resources)(sched.world_mut(), batch) };
            }
        }
    }

    sched.run();
}
