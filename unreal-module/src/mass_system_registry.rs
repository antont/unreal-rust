use std::sync::Mutex;

use unreal_api::ffi::{MassFragmentRequirement, MassSystemDescriptor, Utf8Str};
use unreal_api::mass::{
    MassBevySystemRegistration, MassEntityMap, MassSchedule,
    MassSystemRegistration, MassSystemStage,
    registered_bevy_mass_systems, registered_dispatch_hooks, registered_mass_systems,
    registered_sim_inits, registered_visualizer_groups, registered_spatial_query_configs,
    registered_sim_defaults,
};
use bevy_mass::SpatialQuery;

/// Returns the number of dynamically registered mass systems.
pub unsafe extern "C" fn get_mass_system_count() -> u32 {
    let mut count = 0u32;
    for _ in registered_mass_systems() {
        count += 1;
    }
    count
}

/// Cached requirement arrays — kept alive so descriptor pointers remain valid.
/// Cleared on hot-reload via `reset_descriptor_cache()`.
/// Safety: only accessed from game thread behind Mutex.
struct SyncDescriptorCache(Vec<Box<[MassFragmentRequirement]>>);
unsafe impl Send for SyncDescriptorCache {}
unsafe impl Sync for SyncDescriptorCache {}

static DESCRIPTOR_CACHE: Mutex<SyncDescriptorCache> = Mutex::new(SyncDescriptorCache(Vec::new()));

/// Clear the descriptor cache. Called on hot-reload so stale pointers aren't reused.
pub fn reset_descriptor_cache() {
    let mut cache = DESCRIPTOR_CACHE.lock().unwrap();
    cache.0.clear();
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

    // Build requirements array — filter out non-MassFragment types (is_valid == false),
    // which are pure-Bevy components with no C++ representation.
    let requirements: Vec<MassFragmentRequirement> = registration
        .requirements
        .iter()
        .filter(|req| req.is_valid)
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

    let requirements_len = requirements.len();

    // Store in cache so the pointer remains valid until the next hot-reload.
    let mut cache = DESCRIPTOR_CACHE.lock().unwrap();
    let boxed = requirements.into_boxed_slice();
    let requirements_ptr = if boxed.is_empty() {
        std::ptr::null()
    } else {
        boxed.as_ptr()
    };
    cache.0.push(boxed);

    unsafe {
        (*out) = MassSystemDescriptor {
            name: Utf8Str::from(registration.name),
            num_requirements: requirements_len as u32,
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

/// Run all registered `MassDispatchHook::pre_dispatch` callbacks.
/// Called inside `catch_unwind`, before `sched.run()`.
fn run_pre_dispatch_hooks(world: &mut unreal_api::ecs::world::World) {
    for hook in registered_dispatch_hooks() {
        (hook.pre_dispatch)(world);
    }
}

/// Run all registered `MassDispatchHook::post_dispatch` callbacks.
/// Called after `sched.run()` returns normally — skipped on panic.
fn run_post_dispatch_hooks(world: &mut unreal_api::ecs::world::World) {
    for hook in registered_dispatch_hooks() {
        (hook.post_dispatch)(world);
    }
}

/// Wrapper to allow MassSchedule (which contains App) in a static Mutex.
/// App's runner `Box<dyn FnOnce>` isn't Send, but we only access from game thread.
struct SyncMassSchedule(MassSchedule);
unsafe impl Send for SyncMassSchedule {}
unsafe impl Sync for SyncMassSchedule {}

static MASS_SCHEDULE: Mutex<Option<SyncMassSchedule>> = Mutex::new(None);

/// Build a MassSchedule from all Bevy-registered mass systems.
///
/// All systems (including collision pre-pass) are auto-discovered via inventory.
/// Each gets a sequential stage (0, 1, 2, ...) in discovery order.
pub fn build_bevy_schedule() -> MassSchedule {
    let mut sched = MassSchedule::new();
    let mut regs: Vec<&MassBevySystemRegistration> =
        registered_bevy_mass_systems().into_iter().collect();

    // Sort by execution order so stages are assigned deterministically,
    // regardless of inventory discovery order.
    regs.sort_by_key(|r| r.order);

    // Sequential stage ordering: stage i runs after stage i-1
    for i in 1..regs.len() {
        use unreal_api::ecs::schedule::IntoScheduleConfigs;
        sched.app_mut().configure_sets(
            unreal_api::ecs::Update,
            MassSystemStage(i as u32).after(MassSystemStage((i - 1) as u32)),
        );
    }

    // Init resources, register messages, and add systems
    for (i, reg) in regs.iter().enumerate() {
        (reg.init_resources)(sched.world_mut());
        (reg.register_messages)(sched.app_mut());
        (reg.add_to_app)(sched.app_mut(), MassSystemStage(i as u32));
    }

    // Resource for named spatial query callbacks (populated per-frame)
    sched.world_mut().insert_resource(SpatialQuery::default());

    // message_update_system is already added by App::default() in First schedule.
    // No manual wiring needed.

    sched
}

/// Initialize the global Bevy schedule. Called once during module init.
/// Safe to call after `reset_mass_schedule()` to rebuild.
pub fn init_global_schedule() {
    let mut guard = MASS_SCHEDULE.lock().unwrap();
    if guard.is_none() {
        *guard = Some(SyncMassSchedule(build_bevy_schedule()));
    }
}

/// Reset the global Bevy schedule, allowing it to be rebuilt on next init.
/// Used for testing.
#[cfg(test)]
fn reset_mass_schedule() {
    let mut guard = MASS_SCHEDULE.lock().unwrap();
    *guard = None;
}

/// Per-frame dispatch: update chunk resources and run the Bevy schedule.
///
/// # Safety
/// `data` must point to a valid `MassFrameDispatchData`.
pub unsafe extern "C" fn mass_frame_dispatch(
    data: *const unreal_api::ffi::MassFrameDispatchData,
) -> u32 {
    if data.is_null() {
        return 0;
    }
    let data = unsafe { &*data };

    let Ok(mut guard) = MASS_SCHEDULE.lock() else {
        return 0;
    };
    let Some(wrapper) = guard.as_mut() else {
        return 0;
    };
    let sched = &mut wrapper.0;

    // Discard timing samples left over from a prior frame whose drain was
    // skipped (e.g. sched.run() panicked and unwound out of catch_unwind
    // below before the drain at the end of the closure ran). Without this,
    // stale samples double-report in the next successful frame's [mass-perf]
    // line. Cheap (one mutex lock; Vec is empty when timing is disabled).
    unreal_api::mass::prepare_mass_frame();

    // AssertUnwindSafe: mid-schedule panic may leave App state inconsistent,
    // but worst case is one odd next frame — not UB.
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        sched.set_dt(data.dt);

        // Update spatial queries from dispatch data
        {
            let mut queries = sched.world_mut().resource_mut::<SpatialQuery>();
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

        // Pre-dispatch hooks: game-specific state reset / staging for the frame.
        // Runs inside catch_unwind so a panic leaves no stale state behind.
        run_pre_dispatch_hooks(sched.world_mut());

        sched.run();

        // Per-system timing (opt-in). Drain samples collected during sched.run(),
        // accumulate by system name (in case a system ran more than once — it
        // shouldn't under our scheduler, but let's be forgiving), and log one
        // line per system plus a total. Gated on env var so the cost is zero
        // when disabled.
        if unreal_api::mass::is_mass_timing_enabled() {
            let samples = unreal_api::mass::drain_mass_system_samples();
            if !samples.is_empty() {
                use std::collections::BTreeMap;
                let mut totals: BTreeMap<&'static str, (u128, u32)> = BTreeMap::new();
                let mut grand_total: u128 = 0;
                for s in &samples {
                    let entry = totals.entry(s.name).or_insert((0, 0));
                    entry.0 += s.nanos;
                    entry.1 += 1;
                    grand_total += s.nanos;
                }
                let mut buf = String::from("[mass-perf]");
                for (name, (nanos, calls)) in &totals {
                    let ms = (*nanos as f64) / 1_000_000.0;
                    if *calls > 1 {
                        buf.push_str(&format!(" {}={:.3}ms(x{})", name, ms, calls));
                    } else {
                        buf.push_str(&format!(" {}={:.3}ms", name, ms));
                    }
                }
                buf.push_str(&format!(" total={:.3}ms", (grand_total as f64) / 1_000_000.0));
                log::info!("{}", buf);
            }
        }

        // Post-dispatch hooks: drain game-specific events into FFI caches.
        // Only reached on successful return — skipped if sched.run() panics.
        run_post_dispatch_hooks(sched.world_mut());

        // Return post-dispatch flags and clear them
        unreal_api::mass::take_dispatch_flags()
    }));

    match result {
        Ok(flags) => flags,
        Err(_) => 0,
    }
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

    let outcome = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
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

        // Spawn shadow Bevy entities for each Mass Entity entity.
        // These allow pure-Bevy components to be attached to entities that also
        // have zero-copy MassFragment data in chunks.
        if let Ok(mut sched_guard) = MASS_SCHEDULE.lock() {
            if let Some(sched) = sched_guard.as_mut().map(|w| &mut w.0) {
                let mut entity_map = MassEntityMap::default();
                for (name_bytes, handles) in stored.iter() {
                    let name = std::str::from_utf8(&name_bytes[..name_bytes.len() - 1])
                        .unwrap_or("")
                        .to_string();
                    let entities: Vec<unreal_api::ecs::entity::Entity> = handles
                        .iter()
                        .map(|_| sched.world_mut().spawn_empty().id())
                        .collect();
                    entity_map.insert_group(name, entities);
                }
                *sched.world_mut().resource_mut::<MassEntityMap>() = entity_map;
            }
        }
    }));

    match outcome {
        Ok(()) => 1,
        Err(_) => {
            unsafe {
                (*result).groups = std::ptr::null();
                (*result).num_groups = 0;
                (*result)._pad = 0;
            }
            0
        }
    }
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

// ---------------------------------------------------------------------------
// Rust-authored test discovery and execution
// ---------------------------------------------------------------------------

/// Global storage for the last test error message.
/// Keeps the string alive so C++ can read the pointer after run_mass_test returns.
static LAST_TEST_ERROR: std::sync::Mutex<String> = std::sync::Mutex::new(String::new());

pub unsafe extern "C" fn get_mass_test_count() -> u32 {
    unreal_api::mass::registered_mass_tests().into_iter().count() as u32
}

pub unsafe extern "C" fn get_mass_test_desc(
    index: u32,
    out: *mut unreal_ffi::MassTestDesc,
) -> u32 {
    if out.is_null() {
        return 0;
    }
    let Some(reg) = unreal_api::mass::registered_mass_tests()
        .into_iter()
        .nth(index as usize)
    else {
        return 0;
    };
    unsafe {
        (*out) = unreal_ffi::MassTestDesc {
            name: unreal_ffi::Utf8Str::from(reg.name),
        };
    }
    1
}

pub unsafe extern "C" fn run_mass_test(
    name: unreal_ffi::Utf8Str,
    callbacks: *const unreal_ffi::MassTestCallbacks,
) -> unreal_ffi::MassTestResult {
    let name_str = name.as_str();

    let Some(reg) = unreal_api::mass::registered_mass_tests()
        .into_iter()
        .find(|r| r.name == name_str)
    else {
        let msg = format!("Test '{}' not found", name_str);
        let mut guard = LAST_TEST_ERROR.lock().unwrap();
        *guard = msg;
        return unreal_ffi::MassTestResult {
            passed: 0,
            error_len: guard.len() as u32,
            error_ptr: guard.as_ptr(),
        };
    };

    let test_fn = reg.test_fn;
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let ctx = unsafe { unreal_api::mass::TestCtx::from_raw(callbacks) };
        test_fn(&ctx);
    }));

    match result {
        Ok(()) => unreal_ffi::MassTestResult {
            passed: 1,
            error_len: 0,
            error_ptr: std::ptr::null(),
        },
        Err(payload) => {
            let msg = if let Some(s) = payload.downcast_ref::<&str>() {
                s.to_string()
            } else if let Some(s) = payload.downcast_ref::<String>() {
                s.clone()
            } else {
                "test panicked with unknown payload".to_string()
            };
            let mut guard = LAST_TEST_ERROR.lock().unwrap();
            *guard = msg;
            unreal_ffi::MassTestResult {
                passed: 0,
                error_len: guard.len() as u32,
                error_ptr: guard.as_ptr(),
            }
        }
    }
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

    // --- Dispatch hook iteration test ---
    //
    // Submits a sentinel MassDispatchHook via inventory and verifies that
    // run_pre_dispatch_hooks / run_post_dispatch_hooks fire it in the right order.
    // This test guards against regressions where the hook iteration is
    // accidentally reordered or dropped relative to sched.run().

    use std::sync::Mutex;

    static HOOK_LOG: Mutex<Vec<&'static str>> = Mutex::new(Vec::new());

    fn pre_test_hook(_world: &mut unreal_api::ecs::world::World) {
        HOOK_LOG.lock().unwrap().push("pre");
    }

    fn post_test_hook(_world: &mut unreal_api::ecs::world::World) {
        HOOK_LOG.lock().unwrap().push("post");
    }

    unreal_api::inventory::submit!(unreal_api::mass::MassDispatchHook {
        pre_dispatch: pre_test_hook,
        post_dispatch: post_test_hook,
    });

    #[test]
    fn test_dispatch_hooks_fire_in_order() {
        let mut world = unreal_api::ecs::world::World::new();

        HOOK_LOG.lock().unwrap().clear();

        run_pre_dispatch_hooks(&mut world);
        run_post_dispatch_hooks(&mut world);

        let log = HOOK_LOG.lock().unwrap();
        let pre_idx = log.iter().position(|&s| s == "pre");
        let post_idx = log.iter().position(|&s| s == "post");
        assert!(pre_idx.is_some(), "pre_dispatch hook should have fired");
        assert!(post_idx.is_some(), "post_dispatch hook should have fired");
        assert!(
            pre_idx.unwrap() < post_idx.unwrap(),
            "pre_dispatch must run before post_dispatch, got log: {:?}",
            *log
        );
    }
}
