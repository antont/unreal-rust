# Global query (MassQueryAll) copies data instead of writing directly

## Status: Known limitation (stepping stone)

## Problem

`MassQueryAll<&mut T>` requires copying all matching fragment data into a contiguous temp buffer before passing it to Rust, then copying modifications back to the original chunk storage after Rust returns. This is because Mass Entity stores entities across multiple chunks that aren't contiguous in memory, but the Rust API presents them as a single flat `&mut [T]` slice.

Primary queries (`MassQuery`) don't have this problem — they pass chunk memory directly to Rust with zero copy.

## Impact

For small entity counts (e.g. 50-100 food entities) the overhead is negligible. At scale it becomes a problem: every frame copies `N * sizeof(T)` bytes twice (gather + write-back) for each mutable global fragment type.

## Possible solutions

- **Chunk-by-chunk iteration for globals**: Instead of gathering into one buffer, call Rust once per global chunk. The Rust API would need to handle non-contiguous data (e.g. a callback or iterator-of-slices pattern). Breaks the simple flat-slice ergonomics.

- **Stable archetype pointer access**: If we can guarantee a single archetype for a global query (e.g. all food entities share one archetype), we could pass chunk pointers directly. Requires knowing the archetype layout at query time.

- **Shared memory mapping**: Map the chunk data into a Rust-visible address range without copying. Complex and platform-specific.

- **Accept the copy for writes, zero-copy for reads**: `MassQueryAll<&T>` (read-only) could pass chunk pointers without gathering. Only mutable globals pay the copy cost. This is already partially implemented — read-only globals use `GetFragmentView()` — but the data is still copied into the contiguous buffer.

## Current implementation

See `RustMassDynamicProcessor.cpp` `Execute()` — the `bHasGlobalQueries` path with `GlobalBuffers`, `GlobalFragmentSources`, and the write-back loop.
