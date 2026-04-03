use crate::fragments::{AntFragment, SimBounds};
use crate::movement::ant_movement_system;
use std::ffi::c_void;

/// FFI entry point for ant movement processor.
/// Called by C++ UGatherersAntMovementProcessor::Execute().
///
/// # Safety
/// `ants` must point to a valid array of `count` AntFragment structs (matching C++ layout).
/// `bounds_min` and `bounds_max` must each point to 3 f64 values, or be null if no bounds.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rust_mass_ant_movement(
    ants: *mut c_void,
    count: i32,
    dt: f32,
    bounds_min: *const f64,
    bounds_max: *const f64,
) {
    if ants.is_null() || count <= 0 {
        return;
    }

    let slice = unsafe { std::slice::from_raw_parts_mut(ants as *mut AntFragment, count as usize) };

    if bounds_min.is_null() || bounds_max.is_null() {
        // No bounds — move without boundary clamping (use very large bounds)
        let no_bounds = SimBounds {
            min: [f64::MIN / 2.0; 3],
            max: [f64::MAX / 2.0; 3],
        };
        ant_movement_system(slice, dt, &no_bounds);
    } else {
        let min = unsafe { std::slice::from_raw_parts(bounds_min, 3) };
        let max = unsafe { std::slice::from_raw_parts(bounds_max, 3) };
        let bounds = SimBounds {
            min: [min[0], min[1], min[2]],
            max: [max[0], max[1], max[2]],
        };
        ant_movement_system(slice, dt, &bounds);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::fragments::AntFragment;

    #[test]
    fn null_ptr_no_panic() {
        unsafe {
            rust_mass_ant_movement(std::ptr::null_mut(), 0, 0.016, std::ptr::null(), std::ptr::null());
        }
    }

    #[test]
    fn negative_count_no_panic() {
        unsafe {
            rust_mass_ant_movement(std::ptr::null_mut(), -1, 0.016, std::ptr::null(), std::ptr::null());
        }
    }

    #[test]
    fn updates_fragments() {
        let mut ants = [AntFragment {
            direction: [1.0, 0.0, 0.0],
            movement_speed: 100.0,
            ..Default::default()
        }];
        let bounds_min = [-500.0f64, -500.0, -100.0];
        let bounds_max = [500.0f64, 500.0, 100.0];
        unsafe {
            rust_mass_ant_movement(
                ants.as_mut_ptr() as *mut c_void,
                ants.len() as i32,
                0.1,
                bounds_min.as_ptr(),
                bounds_max.as_ptr(),
            );
        }
        assert!(
            (ants[0].position[0] - 10.0).abs() < 1e-6,
            "expected ~10.0, got {}",
            ants[0].position[0]
        );
    }

    #[test]
    fn matches_direct_call() {
        let base = AntFragment {
            position: [50.0, 50.0, 0.0],
            direction: [0.0, 1.0, 0.0],
            movement_speed: 200.0,
            pickup_cooldown_remaining_seconds: 0.5,
            ..Default::default()
        };

        // Direct call
        let mut direct = [base];
        let bounds = SimBounds {
            min: [-500.0, -500.0, -100.0],
            max: [500.0, 500.0, 100.0],
        };
        ant_movement_system(&mut direct, 0.05, &bounds);

        // FFI call
        let mut ffi = [base];
        let bounds_min = [-500.0f64, -500.0, -100.0];
        let bounds_max = [500.0f64, 500.0, 100.0];
        unsafe {
            rust_mass_ant_movement(
                ffi.as_mut_ptr() as *mut c_void,
                ffi.len() as i32,
                0.05,
                bounds_min.as_ptr(),
                bounds_max.as_ptr(),
            );
        }

        assert_eq!(direct[0].position, ffi[0].position);
        assert_eq!(direct[0].previous_position, ffi[0].previous_position);
        assert_eq!(
            direct[0].pickup_cooldown_remaining_seconds,
            ffi[0].pickup_cooldown_remaining_seconds
        );
    }

    #[test]
    fn null_bounds_still_moves() {
        let mut ants = [AntFragment {
            direction: [1.0, 0.0, 0.0],
            movement_speed: 100.0,
            ..Default::default()
        }];
        unsafe {
            rust_mass_ant_movement(
                ants.as_mut_ptr() as *mut c_void,
                ants.len() as i32,
                0.1,
                std::ptr::null(),
                std::ptr::null(),
            );
        }
        assert!(ants[0].position[0] > 0.0, "should have moved");
    }
}
