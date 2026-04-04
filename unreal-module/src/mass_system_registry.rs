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

/// Stage number reserved for the collision pre-pass (between movement and food_decision).
/// Registered systems use even stages (0, 2, 4, ...), collision uses stage 1.
const COLLISION_STAGE: u32 = 1;

/// Build a MassSchedule from all Bevy-registered mass systems.
///
/// Stage layout (with 4 registered systems):
/// - Stage 0: first registered system (ant_movement)
/// - Stage 1: collision pre-pass (hand-written)
/// - Stage 2: second registered system (ant_food_decision)
/// - Stage 4: third registered system (ant_cooldown)
/// - Stage 6: fourth registered system (ant_boundary_reflect)
pub fn build_bevy_schedule() -> MassSchedule {
    let mut sched = MassSchedule::new();
    let regs: Vec<&MassBevySystemRegistration> =
        registered_bevy_mass_systems().into_iter().collect();

    // Assign stages: first system at 0, collision at 1, rest at 2, 4, 6, ...
    let mut stages: Vec<u32> = Vec::with_capacity(regs.len());
    for i in 0..regs.len() {
        if i == 0 {
            stages.push(0);
        } else {
            stages.push(COLLISION_STAGE + 1 + ((i - 1) as u32) * 2);
        }
    }

    // Build stage ordering chain
    let mut all_stages: Vec<u32> = stages.clone();
    if regs.len() > 1 {
        // Insert collision stage between first and second
        all_stages.insert(1, COLLISION_STAGE);
    }
    for i in 1..all_stages.len() {
        sched
            .schedule_mut()
            .configure_sets(MassSystemStage(all_stages[i]).after(MassSystemStage(all_stages[i - 1])));
    }

    // Init and add registered systems
    for (i, reg) in regs.iter().enumerate() {
        (reg.init_resources)(sched.world_mut());
        (reg.add_to_schedule)(sched.schedule_mut(), MassSystemStage(stages[i]));
    }

    // Add collision pre-pass system (needs AntFragment + AntEncounterFragment resources + spatial callback)
    sched.world_mut().insert_resource(MassSpatialQueryCallback::default());
    sched.schedule_mut().add_systems(
        gatherers_bevy_mass::systems::ant_collision_prepass_bevy
            .in_set(MassSystemStage(COLLISION_STAGE)),
    );

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

    // Collect registrations indexed by discovery order (matches system_index from C++)
    let regs: Vec<&MassBevySystemRegistration> =
        registered_bevy_mass_systems().into_iter().collect();

    // Clear all chunk resources
    for reg in &regs {
        (reg.clear_resources)(sched.world_mut());
    }

    // Populate from dispatch data — each batch's system_index maps to registration order
    let batches = unsafe {
        std::slice::from_raw_parts(data.systems, data.num_systems as usize)
    };
    for batch in batches {
        let idx = batch.system_index as usize;
        if let Some(reg) = regs.get(idx) {
            unsafe { (reg.populate_resources)(sched.world_mut(), batch) };
        }
    }

    sched.run();
}
