//! Decision-outcome diagnostic counters.
//!
//! The shared `food_decision_system` calls these every tick regardless of
//! backend. In standalone Bevy builds they are no-ops (inert at LTO). In
//! Unreal-mode builds (`unreal_diag` feature — enabled transitively by
//! `gatherers-bevy-mass`'s dep), they accumulate into `AtomicU64` statics
//! that UE automation tests read via the FFI accessors in
//! `gatherers_bevy_mass::systems`.
//!
//! Note: `ants_iterated` historically counted every ant examined by the
//! processor, which under the old code path was one-per-chunk-tick. The
//! unified system does a hit-map lookup instead, so `ants_iterated` stays
//! at zero. The counter is preserved in the FFI struct (and logged by the
//! `PickupDensity` spec) — removing it would break the cross-language
//! struct layout. See `unreal-ffi::DecisionCounters`.

#[cfg(feature = "unreal_diag")]
mod real {
    use std::sync::atomic::{AtomicU64, Ordering};

    pub(super) static CALLS: AtomicU64 = AtomicU64::new(0);
    pub(super) static HITS_SEEN: AtomicU64 = AtomicU64::new(0);
    pub(super) static MATCHED: AtomicU64 = AtomicU64::new(0);
    pub(super) static PICKUPS: AtomicU64 = AtomicU64::new(0);
    pub(super) static DROPS: AtomicU64 = AtomicU64::new(0);
    pub(super) static NO_ACTIONS: AtomicU64 = AtomicU64::new(0);

    #[inline] pub fn bump(counter: &AtomicU64) { counter.fetch_add(1, Ordering::Relaxed); }
    #[inline] pub fn read(counter: &AtomicU64) -> u64 { counter.load(Ordering::Relaxed) }
    #[inline] pub fn zero(counter: &AtomicU64) { counter.store(0, Ordering::Relaxed); }
}

// ---------------------------------------------------------------------------
// Per-event hooks called from the shared decision system. Kept inline so the
// no-op path compiles to nothing.
// ---------------------------------------------------------------------------

#[inline]
pub fn decision_call() {
    #[cfg(feature = "unreal_diag")]
    real::bump(&real::CALLS);
}

#[inline]
pub fn decision_hit_seen() {
    #[cfg(feature = "unreal_diag")]
    real::bump(&real::HITS_SEEN);
}

#[inline]
pub fn decision_matched() {
    #[cfg(feature = "unreal_diag")]
    real::bump(&real::MATCHED);
}

#[inline]
pub fn decision_pickup() {
    #[cfg(feature = "unreal_diag")]
    real::bump(&real::PICKUPS);
}

#[inline]
pub fn decision_drop() {
    #[cfg(feature = "unreal_diag")]
    real::bump(&real::DROPS);
}

#[inline]
pub fn decision_no_action() {
    #[cfg(feature = "unreal_diag")]
    real::bump(&real::NO_ACTIONS);
}

// ---------------------------------------------------------------------------
// FFI-facing snapshot / reset. Only compiled under `unreal_diag` — the UE
// bridge crate wires these into `MassExternBinding`.
// ---------------------------------------------------------------------------

/// Snapshot all counters. Maps 1:1 onto the `DecisionCounters` FFI struct
/// but kept as a plain Rust type here so the sim crate has no FFI dep.
#[cfg(feature = "unreal_diag")]
#[derive(Default, Clone, Copy, Debug)]
pub struct DecisionSnapshot {
    pub calls: u64,
    pub hits_seen: u64,
    pub ants_seen: u64,
    pub matched: u64,
    pub pickups: u64,
    pub drops: u64,
    pub no_actions: u64,
}

#[cfg(feature = "unreal_diag")]
pub fn snapshot() -> DecisionSnapshot {
    DecisionSnapshot {
        calls: real::read(&real::CALLS),
        hits_seen: real::read(&real::HITS_SEEN),
        ants_seen: 0, // see module doc — unified system does not iterate all ants
        matched: real::read(&real::MATCHED),
        pickups: real::read(&real::PICKUPS),
        drops: real::read(&real::DROPS),
        no_actions: real::read(&real::NO_ACTIONS),
    }
}

#[cfg(feature = "unreal_diag")]
pub fn reset() {
    real::zero(&real::CALLS);
    real::zero(&real::HITS_SEEN);
    real::zero(&real::MATCHED);
    real::zero(&real::PICKUPS);
    real::zero(&real::DROPS);
    real::zero(&real::NO_ACTIONS);
}
