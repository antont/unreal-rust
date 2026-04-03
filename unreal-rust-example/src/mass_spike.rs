/// MassEntity-Bevy bridge spike: fragment type and processor logic.
///
/// BobFragment is a #[repr(C)] struct matching the C++ FBobFragment USTRUCT.
/// bob_movement_system operates on slices — callable from UE (via extern "C"),
/// Bevy systems, or plain #[test] without any engine dependency.

use std::ffi::c_void;

#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct BobFragment {
    pub position_x: f64,
    pub position_y: f64,
    pub position_z: f64,
    pub velocity_z: f64,
    pub time: f32,
    pub speed: f32,
}

pub fn bob_movement_system(_fragments: &mut [BobFragment], _dt: f32) {
    // TODO: implement
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bob_fragment_is_repr_c_and_default() {
        let frag = BobFragment::default();
        assert_eq!(frag.position_z, 0.0);
        assert_eq!(frag.speed, 0.0);
        assert_eq!(frag.time, 0.0);
    }

    #[test]
    fn bob_fragment_has_expected_size_and_alignment() {
        // Must match the C++ FBobFragment (4 doubles + 2 floats = 40 bytes)
        assert_eq!(std::mem::size_of::<BobFragment>(), 40);
        assert_eq!(std::mem::align_of::<BobFragment>(), 8);
    }

    #[test]
    fn bob_fragment_field_offsets() {
        assert_eq!(std::mem::offset_of!(BobFragment, position_x), 0);
        assert_eq!(std::mem::offset_of!(BobFragment, position_y), 8);
        assert_eq!(std::mem::offset_of!(BobFragment, position_z), 16);
        assert_eq!(std::mem::offset_of!(BobFragment, velocity_z), 24);
        assert_eq!(std::mem::offset_of!(BobFragment, time), 32);
        assert_eq!(std::mem::offset_of!(BobFragment, speed), 36);
    }

    #[test]
    fn bob_movement_advances_time() {
        let mut frags = [BobFragment { speed: 2.0, ..Default::default() }];
        bob_movement_system(&mut frags, 0.016);
        assert!((frags[0].time - 0.016).abs() < 1e-6);
    }

    #[test]
    fn bob_movement_changes_position_z() {
        let mut frags = [BobFragment { speed: 2.0, ..Default::default() }];
        bob_movement_system(&mut frags, 0.016);
        assert!(frags[0].position_z != 0.0, "position_z should change after one step");
    }

    #[test]
    fn bob_movement_preserves_xy() {
        let mut frags = [BobFragment {
            position_x: 100.0, position_y: 200.0, speed: 2.0, ..Default::default()
        }];
        bob_movement_system(&mut frags, 0.016);
        assert_eq!(frags[0].position_x, 100.0);
        assert_eq!(frags[0].position_y, 200.0);
    }

    #[test]
    fn bob_movement_zero_dt_no_change() {
        let mut frags = [BobFragment { speed: 2.0, position_z: 50.0, ..Default::default() }];
        bob_movement_system(&mut frags, 0.0);
        assert_eq!(frags[0].position_z, 50.0);
        assert_eq!(frags[0].time, 0.0);
    }

    #[test]
    fn bob_movement_different_speeds() {
        let mut frags = [
            BobFragment { speed: 1.0, ..Default::default() },
            BobFragment { speed: 5.0, ..Default::default() },
        ];
        for _ in 0..60 { bob_movement_system(&mut frags, 0.016); }
        assert!(
            (frags[0].position_z - frags[1].position_z).abs() > 0.01,
            "different speeds should produce different Z positions"
        );
    }

    #[test]
    fn bob_movement_oscillates() {
        let mut frag = BobFragment { speed: 2.0, ..Default::default() };
        let mut saw_positive = false;
        let mut saw_negative = false;
        let mut prev_z = 0.0;
        for _ in 0..200 {
            bob_movement_system(std::slice::from_mut(&mut frag), 0.016);
            let delta = frag.position_z - prev_z;
            if delta > 0.0 { saw_positive = true; }
            if delta < 0.0 { saw_negative = true; }
            prev_z = frag.position_z;
        }
        assert!(saw_positive && saw_negative, "bob should oscillate up and down");
    }

    #[test]
    fn bob_movement_batch_processes_all() {
        let mut frags: Vec<BobFragment> = (0..100)
            .map(|i| BobFragment { speed: 2.0, position_z: i as f64, ..Default::default() })
            .collect();
        bob_movement_system(&mut frags, 0.016);
        for (i, frag) in frags.iter().enumerate() {
            assert!(frag.time > 0.0, "entity {} should have time advanced", i);
        }
    }

    #[test]
    fn bob_movement_empty_slice_noop() {
        let mut frags: Vec<BobFragment> = vec![];
        bob_movement_system(&mut frags, 0.016); // should not panic
    }

    #[test]
    fn bob_movement_10k_entities_under_1ms() {
        let mut frags: Vec<BobFragment> = (0..10_000)
            .map(|i| BobFragment { speed: 2.0, position_z: i as f64, ..Default::default() })
            .collect();
        let start = std::time::Instant::now();
        for _ in 0..60 { bob_movement_system(&mut frags, 0.016); }
        let elapsed = start.elapsed();
        let per_frame = elapsed / 60;
        assert!(
            per_frame.as_micros() < 1000,
            "10k entities should process in < 1ms per frame, took {:?}", per_frame
        );
    }
}
