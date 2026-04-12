# Reduce test coupling to generated facade struct names

Tests in `gatherers-bevy-mass/src/systems.rs` directly construct `__FQ_ant_food_decision_ants` — the macro-generated facade struct. This couples tests to internal codegen naming conventions.

## Problem

If the macro's naming scheme changes, all tests break even though the system logic is unchanged.

## Possible approaches

- Provide a public test helper / builder API for constructing facade structs
- Export a type alias from the macro so tests reference a stable name
- Restructure tests to test through the system entry point rather than constructing internals
