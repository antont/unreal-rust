# Automate fragment padding for C++ layout compatibility

## Problem

Every `#[repr(C)]` fragment struct has manual `_pad` fields to match C++ sizeof/alignment:

```rust
pub struct Movement {
    pub direction: DVec3,
    pub movement_speed: f32,
    pub _pad: [u8; 4],  // manual
}
```

Authors must write `_pad: [0; 4]` at every construction site. The padding sizes are determined by manually checking C++ struct layout.

## Two categories of padding

1. **Trailing padding** (Movement, Carrying, FoodFragment) — padding at the end to match struct size. Can be solved with `#[repr(C, align(N))]`.
2. **Interior padding** (AntEncounterFragment._nearest_pad) — padding between fields for alignment. Cannot be solved with align alone.

## Possible approaches

- `#[repr(C, align(N))]` for trailing-only cases
- Proc macro on MassFragment derive that inserts padding fields automatically based on declared C++ size
- Make the facade layer handle mismatched sizes (pad/truncate at the boundary)

See detailed design discussion for the chosen approach.
