# More Rust-authored UE tests

Expand test coverage using the Rust-authored test infrastructure (`TestCtx` + `inventory::submit!(MassTestRegistration)`).

## Current coverage

6 tests in `gatherers-bevy-mass/src/ue_tests.rs`:
- SpawnAndSimulate, BoundaryReflect, CooldownDecrement, FoodPickup, FoodDrop, Integration

## Candidates for new tests

- Hot-reload cycle (reset + reinit from Rust side)
- Multiple food pickups/drops in sequence
- Ant behavior with turn jitter (verify direction randomness)
- Large entity counts (stress test)
- Fragment write-back verification (write from Rust, read back, verify)
