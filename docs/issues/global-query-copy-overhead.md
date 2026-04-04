# Global query (MassQueryAll) copies data instead of writing directly

## Status: Resolved

Solved on `global-query-zero-copy` branch by caching direct pointers to Mass Entity chunk memory. Since ant and food entities are stable throughout the simulation (never added/removed), chunk layout is frozen and pointers are cached on first `Execute()`.

The Rust `MassQueryAllMut<T>` now wraps chunked descriptors (`MassGlobalFragmentChunks`) that point directly into Mass Entity memory. Iteration walks across chunks transparently. `get_mut(index)` provides O(1) indexed access.

See `docs/massentity-bridge-design.md` for the current architecture.
