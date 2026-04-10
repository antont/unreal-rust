use std::sync::Mutex;

use unreal_api::ffi::{MassFragmentRequirement, MassSystemDescriptor, Utf8Str};
use unreal_api::mass::{
    MassBevySystemRegistration, MassDeltaTime, MassSchedule,
    MassSpatialQueries,
    MassSystemRegistration, MassSystemStage,
    registered_bevy_mass_systems, registered_mass_systems, registered_sim_inits,
    registered_visualizer_groups, registered_spatial_query_configs, registered_sim_defaults,
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

static MASS_SCHEDULE: Mutex<Option<MassSchedule>> = Mutex::new(None);

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

    // Resource for named spatial query callbacks (populated per-frame)
    sched.world_mut().insert_resource(MassSpatialQueries::default());

    sched
}

/// Initialize the global Bevy schedule. Called once during module init.
/// Safe to call after `reset_mass_schedule()` to rebuild.
pub fn init_global_schedule() {
    let mut guard = MASS_SCHEDULE.lock().unwrap();
    if guard.is_none() {
        *guard = Some(build_bevy_schedule());
    }
}

/// Reset the global Bevy schedule, allowing it to be rebuilt on next init.
/// Used for hot-reload and testing.
pub fn reset_mass_schedule() {
    let mut guard = MASS_SCHEDULE.lock().unwrap();
    *guard = None;
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

    let Ok(mut guard) = MASS_SCHEDULE.lock() else {
        return;
    };
    let Some(sched) = guard.as_mut() else {
        return;
    };

    sched.set_dt(data.dt);

    // Update spatial queries from dispatch data
    {
        let mut queries = sched.world_mut().resource_mut::<MassSpatialQueries>();
        queries.clear();
        if data.num_spatial_queries > 0 && !data.spatial_queries.is_null() {
            let slots = unsafe {
                std::slice::from_raw_parts(data.spatial_queries, data.num_spatial_queries as usize)
            };
            for slot in slots {
                let name = slot.name.as_str().to_string();
                queries.insert(name, slot.query_fn, slot.radius);
            }
        }
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

// ---------------------------------------------------------------------------
// Simulation init dispatch (named entity groups)
// ---------------------------------------------------------------------------

/// Wrapper for MassEntityGroupResult to allow it in a static Mutex.
/// Safety: only accessed from game thread behind Mutex.
struct SyncGroupResult(Vec<unreal_ffi::MassEntityGroupResult>);
unsafe impl Send for SyncGroupResult {}
unsafe impl Sync for SyncGroupResult {}

/// Stored results for init: group names (as null-terminated bytes) + handles.
static INIT_RESULT_GROUPS: Mutex<Vec<(Vec<u8>, Vec<unreal_ffi::MassEntityHandle>)>> =
    Mutex::new(Vec::new());
/// Stored descriptors pointing into INIT_RESULT_GROUPS. Rebuilt on each call.
static INIT_RESULT_DESCS: Mutex<SyncGroupResult> =
    Mutex::new(SyncGroupResult(Vec::new()));

/// Dispatch simulation init to the first registered init function.
///
/// # Safety
/// `params` and `result` must be valid pointers.
pub unsafe extern "C" fn mass_init_simulation(
    params: *const unreal_ffi::MassInitSimulationParams,
    result: *mut unreal_ffi::MassInitSimulationResult,
) -> u32 {
    if params.is_null() || result.is_null() {
        return 0;
    }
    let params = unsafe { &*params };
    let Some(reg) = registered_sim_inits().into_iter().next() else {
        return 0;
    };
    let groups = (reg.init_fn)(params);

    let mut stored = INIT_RESULT_GROUPS.lock().unwrap();
    *stored = groups
        .into_iter()
        .map(|(name, handles)| {
            let mut name_bytes = name.into_bytes();
            name_bytes.push(0); // null-terminate for C
            (name_bytes, handles)
        })
        .collect();

    // Build descriptor array pointing into stored data
    let mut descs = INIT_RESULT_DESCS.lock().unwrap();
    descs.0.clear();
    for (name_bytes, handles) in stored.iter() {
        descs.0.push(unreal_ffi::MassEntityGroupResult {
            name: unreal_ffi::Utf8Str {
                ptr: name_bytes.as_ptr() as *const i8,
                len: (name_bytes.len() - 1) as usize, // exclude null terminator
            },
            handles: handles.as_ptr(),
            count: handles.len() as u32,
            _pad: 0,
        });
    }

    unsafe {
        (*result).groups = descs.0.as_ptr();
        (*result).num_groups = descs.0.len() as u32;
        (*result)._pad = 0;
    }
    1
}

// ---------------------------------------------------------------------------
// Visualizer group descriptors
// ---------------------------------------------------------------------------

pub unsafe extern "C" fn get_visualizer_group_count() -> u32 {
    registered_visualizer_groups().into_iter().count() as u32
}

pub unsafe extern "C" fn get_visualizer_group_desc(
    index: u32,
    out: *mut unreal_ffi::MassVisualizerGroupDesc,
) -> u32 {
    if out.is_null() {
        return 0;
    }
    let Some(reg) = registered_visualizer_groups().into_iter().nth(index as usize) else {
        return 0;
    };
    unsafe {
        (*out) = unreal_ffi::MassVisualizerGroupDesc {
            name: unreal_ffi::Utf8Str::from(reg.name),
            position_fragment_type: unreal_ffi::Utf8Str::from(reg.position_fragment_type),
            position_offset: reg.position_offset as u32,
            scale: reg.scale,
        };
    }
    1
}

// ---------------------------------------------------------------------------
// Spatial query config descriptors
// ---------------------------------------------------------------------------

pub unsafe extern "C" fn get_spatial_query_config_count() -> u32 {
    registered_spatial_query_configs().into_iter().count() as u32
}

pub unsafe extern "C" fn get_spatial_query_config_desc(
    index: u32,
    out: *mut unreal_ffi::MassSpatialQueryConfigDesc,
) -> u32 {
    if out.is_null() {
        return 0;
    }
    let Some(reg) = registered_spatial_query_configs().into_iter().nth(index as usize) else {
        return 0;
    };
    unsafe {
        (*out) = unreal_ffi::MassSpatialQueryConfigDesc {
            query_name: unreal_ffi::Utf8Str::from(reg.query_name),
            query_group: unreal_ffi::Utf8Str::from(reg.query_group),
            radius: reg.radius,
            _pad0: 0,
            filter_fragment_type: unreal_ffi::Utf8Str::from(reg.filter_fragment_type),
            filter_bool_offset: reg.filter_bool_offset as u32,
            filter_bool_must_be: reg.filter_bool_must_be,
            query_type: reg.query_type as u8,
            collision_channel_index: reg.collision_channel_index,
            _pad1: 0,
        };
    }
    1
}

// ---------------------------------------------------------------------------
// Simulation defaults
// ---------------------------------------------------------------------------

/// Wrapper for MassEntityGroupDesc vec to allow it in a static Mutex.
/// Safety: only accessed from game thread behind Mutex.
struct SyncGroupDescs(Vec<unreal_ffi::MassEntityGroupDesc>);
unsafe impl Send for SyncGroupDescs {}
unsafe impl Sync for SyncGroupDescs {}

/// Stored group descs for sim defaults.
static SIM_DEFAULTS_GROUPS: Mutex<SyncGroupDescs> =
    Mutex::new(SyncGroupDescs(Vec::new()));

pub unsafe extern "C" fn get_sim_defaults(
    out: *mut unreal_ffi::MassSimDefaultsDesc,
) -> u32 {
    if out.is_null() {
        return 0;
    }
    let Some(reg) = registered_sim_defaults().into_iter().next() else {
        return 0;
    };

    // Build group descs from registration data
    let mut groups = SIM_DEFAULTS_GROUPS.lock().unwrap();
    groups.0.clear();
    for &(name, count) in reg.groups {
        groups.0.push(unreal_ffi::MassEntityGroupDesc {
            name: unreal_ffi::Utf8Str::from(name),
            count,
            _pad: 0,
        });
    }

    unsafe {
        (*out) = unreal_ffi::MassSimDefaultsDesc {
            groups: groups.0.as_ptr(),
            num_groups: groups.0.len() as u32,
            _pad: 0,
            bounds_min: reg.bounds_min,
            bounds_max: reg.bounds_max,
            random_seed: reg.random_seed,
            _pad2: 0,
        };
    }
    1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_init_and_reset_schedule() {
        // Start clean
        reset_mass_schedule();

        // Init should build a schedule
        init_global_schedule();
        {
            let guard = MASS_SCHEDULE.lock().unwrap();
            assert!(guard.is_some(), "Schedule should exist after init");
        }

        // Reset should clear it
        reset_mass_schedule();
        {
            let guard = MASS_SCHEDULE.lock().unwrap();
            assert!(guard.is_none(), "Schedule should be None after reset");
        }

        // Re-init should rebuild
        init_global_schedule();
        {
            let guard = MASS_SCHEDULE.lock().unwrap();
            assert!(guard.is_some(), "Schedule should exist after re-init");
        }

        // Clean up for other tests
        reset_mass_schedule();
    }

    #[test]
    fn test_mass_frame_dispatch_without_schedule() {
        // Ensure no schedule exists
        reset_mass_schedule();

        // Dispatch with no schedule should return gracefully (no panic)
        let data = unreal_ffi::MassFrameDispatchData {
            dt: 0.016,
            num_systems: 0,
            systems: std::ptr::null(),
            num_spatial_queries: 0,
            _pad: 0,
            spatial_queries: std::ptr::null(),
        };
        unsafe {
            mass_frame_dispatch(&data as *const _);
        }
        // If we get here without panic, the test passes
    }
}
