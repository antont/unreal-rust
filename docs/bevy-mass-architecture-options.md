# Bevy + Unreal Mass Entity: Architecture Options

## Problem

Systems authored with the `#[mass_system]` proc macro use custom types
(`MassQuery<&T>`) that don't match standard Bevy syntax. This ties application
code to Unreal and prevents running the same logic on pure Bevy for testing,
standalone services, or LLM-assisted code generation.

We want source-code compatibility: write standard Bevy systems that compile
against either pure Bevy or Unreal Mass Entity via a feature flag.

## Options Considered

### 1. Mirror Entities Per Frame

Copy C++ Mass Entity chunk data into real Bevy entities each frame. Systems use
genuine `Query<&mut T>`. After the schedule runs, copy modified components back
to C++ chunks.

**Pros:**
- Full Bevy compatibility — every Bevy feature works (filters, change detection, etc.)
- No custom types at all
- Simplest mental model

**Cons:**
- O(n) copy per frame per fragment type, both directions
- Entity creation/destruction must be synchronized between Mass Entity and Bevy
- Doubles memory for all fragment data during the frame

**Verdict:** Viable at small scale (hundreds of entities) but defeats the
zero-copy design goal. The copy cost grows with entity count and fragment size.

### 2. Facade Crate with Unified Query Wrapper (chosen)

A `bevy_mass` crate that provides a `Query<D>` type implementing Bevy's
iteration API. In pure-Bevy mode it wraps `bevy_ecs::system::Query`. In Unreal
mode it wraps `MassSystemChunks` and flattens chunk iteration.

**Pros:**
- Zero-copy in Unreal mode (pointers into C++ chunk memory)
- Standard Bevy syntax for the common patterns (iteration, Res/ResMut)
- Feature flag switches backends at compile time
- No Bevy fork required
- Incremental adoption — migrate systems one at a time

**Cons:**
- Only supports the subset of Bevy Query features that both backends can provide
- Advanced features (filters, change detection) require additional work per phase
- Custom `SystemParam` implementation is non-trivial

**Verdict:** Best balance of compatibility, performance, and maintainability.
Covers the 80% case (iteration, resources) with zero overhead.

### 3. Fork bevy_ecs with Pluggable Storage

Fork `bevy_ecs` and add a `StorageBackend` trait. The default implementation
uses Bevy's existing archetype tables. A `MassEntityStorage` implementation
reads from C++ chunk memory. Real `Query<&T>` works with external storage.

**Pros:**
- True `bevy_ecs::system::Query` — no wrapper types
- All Bevy features work automatically
- Could eventually merge upstream

**Cons:**
- Large fork to maintain against Bevy's rapid release cycle (~3 month cadence)
- Storage is Bevy's hottest code path — trait indirection risks performance
- Deep changes to archetype management, table storage, change detection
- Bevy's storage assumes it owns the memory (allocation, moves on archetype changes)

**Verdict:** High risk, high reward. Only justified if the facade proves
insufficient and there's willingness to maintain a Bevy fork long-term.

### 4. Upstream Bevy Proposal

Propose a `StorageBackend` trait to Bevy upstream, allowing external crates to
plug alternative storage implementations.

**Pros:**
- No fork to maintain
- Benefits the broader ecosystem (GPU-backed storage, database-backed ECS, etc.)
- Bevy team handles correctness and optimization

**Cons:**
- Uncertain acceptance — Bevy prioritizes performance and simplicity
- Must prove zero overhead for the default path (no trait dispatch on hot loops)
- Multi-month RFC and review process
- Bevy's storage internals are in active flux (relations, hooks, observers)

**Verdict:** Worth pursuing eventually, using the facade crate as evidence of
demand and a reference implementation. Not viable as a first step.

## Decision

**Option 2 (facade crate)** — start with `bevy_mass` providing a compatible
Query wrapper. This gives immediate value (standard Bevy syntax, dual-mode
compilation) without requiring Bevy modifications or maintaining a fork.

If the facade proves the concept and gains traction, it provides evidence and
design input for an eventual upstream proposal (Option 4).
