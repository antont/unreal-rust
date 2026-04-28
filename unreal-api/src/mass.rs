//! Bevy-like API layer for MassEntity.
//!
//! Provides `MassFragment` trait, query types with compile-time mut/immut
//! enforcement, and registration infrastructure for dynamic mass system dispatch.
//!
//! Query types:
//! - `MassQuery<&T>` / `MassQuery<&mut T>` — primary per-chunk query (user syntax)
//! - `MassQueryAll<&T>` / `MassQueryAll<&mut T>` — global cross-archetype query (user syntax)
//!
//! The proc macro maps these to concrete types:
//! - `MassQueryRef<T>` — read-only per-chunk
//! - `MassQueryMut<T>` — read-write per-chunk
//! - `MassQueryAllRef<T>` — read-only global
//! - `MassQueryAllMut<T>` — read-write global

use std::ffi::c_void;
use std::marker::PhantomData;
use std::sync::atomic::{AtomicU32, Ordering};

// ---------------------------------------------------------------------------
// Post-dispatch flags (set by Rust systems, read by C++ after dispatch)
// Safety: Relaxed ordering is correct — the Bevy schedule runs single-threaded
// (SingleThreadedExecutor) and both set/take happen on the UE game thread.
// ---------------------------------------------------------------------------

static DISPATCH_FLAGS: AtomicU32 = AtomicU32::new(0);

/// Set a dispatch flag (called from Rust systems during schedule execution).
pub fn set_dispatch_flag(flag: u32) {
    DISPATCH_FLAGS.fetch_or(flag, Ordering::Relaxed);
}

/// Take and clear all dispatch flags (called from mass_frame_dispatch after schedule runs).
pub fn take_dispatch_flags() -> u32 {
    DISPATCH_FLAGS.swap(0, Ordering::Relaxed)
}

// ---------------------------------------------------------------------------
// Message replay (chunk-loop safe MessageReader)
// ---------------------------------------------------------------------------

/// A per-chunk view over messages drained once by the outer Bevy wrapper.
///
/// Background: `#[mass_system]` calls the user function once per Mass Entity
/// chunk. A Bevy `MessageReader<T>` advances its internal cursor when `.read()`
/// is called; calling it on the first chunk drains the entire queue and
/// subsequent chunks observe no messages. The macro now pre-collects messages
/// into a `Vec<T>` once per frame (via the outer Bevy wrapper's real
/// `MessageReader`) and hands each chunk a fresh `MessageReplay` that iterates
/// the same buffer from the start. Same `.read()` API as `MessageReader`.
///
/// **`T: Clone` requirement**: the `#[mass_system]` macro expansion builds the
/// shared per-frame buffer via `reader.read().cloned().collect::<Vec<T>>()`
/// (see `unreal-api-derive/src/mass_system.rs`), so any message type used as
/// `MessageReader<T>` in a `#[mass_system]` parameter must implement `Clone`.
/// Standalone-Bevy uses of `MessageReader<T>` are unaffected. This is a
/// deliberate tradeoff: payloads are small, `Clone` is cheap, and pre-collecting
/// avoids per-chunk borrowed-slice lifetimes from the outer scheduler.
pub struct MessageReplay<'a, T: 'static> {
    messages: &'a [T],
    cursor: usize,
}

impl<'a, T: 'static> MessageReplay<'a, T> {
    /// Construct a replay view over the shared per-frame buffer.
    pub fn new(messages: &'a [T]) -> Self {
        Self { messages, cursor: 0 }
    }

    /// Iterate remaining messages — same semantics as `MessageReader::read`,
    /// but scoped to a single chunk iteration (cursor does not leak between
    /// chunks because each chunk gets a fresh `MessageReplay`).
    pub fn read(&mut self) -> std::slice::Iter<'a, T> {
        let iter = self.messages[self.cursor..].iter();
        self.cursor = self.messages.len();
        iter
    }

    /// Number of messages remaining in this replay view.
    pub fn len(&self) -> usize {
        self.messages.len() - self.cursor
    }

    /// Whether this replay view has any remaining messages.
    pub fn is_empty(&self) -> bool {
        self.cursor >= self.messages.len()
    }
}

// ---------------------------------------------------------------------------
// Bevy scheduling resources
// ---------------------------------------------------------------------------

// MassDeltaTime removed — use standard bevy_time::Time via TimePlugin instead.

/// Per-fragment-type chunk storage (implementation detail).
/// Use `MassSystemChunks<S, T>` as the Bevy resource — the marker type `S`
/// ensures each system gets its own isolated storage even when multiple
/// systems use the same fragment type.
pub struct MassChunks<T: Copy + 'static> {
    primary_slices: Vec<unreal_ffi::MassFragmentSlice>,
    global_desc: Option<*const unreal_ffi::MassGlobalFragmentChunks>,
    _phantom: PhantomData<T>,
}

impl<T: Copy + 'static> bevy_ecs::prelude::Resource for MassChunks<T> {}

// Safety: chunk pointers are valid for the frame duration (C++ guarantees),
// and Bevy's scheduler serializes conflicting access via Res/ResMut.
unsafe impl<T: Copy + 'static> Send for MassChunks<T> {}
unsafe impl<T: Copy + 'static> Sync for MassChunks<T> {}

/// Per-system, per-fragment chunk storage as a Bevy resource.
/// The marker type `S` (a generated zero-sized struct per `#[mass_system]`)
/// ensures each system gets isolated chunk data, preventing cross-system
/// contamination when multiple systems use the same fragment type.
pub struct MassSystemChunks<S: 'static, T: Copy + 'static> {
    inner: MassChunks<T>,
    _marker: PhantomData<S>,
}

impl<S: 'static, T: Copy + 'static> bevy_ecs::prelude::Resource for MassSystemChunks<S, T> {}
unsafe impl<S: 'static, T: Copy + 'static> Send for MassSystemChunks<S, T> {}
unsafe impl<S: 'static, T: Copy + 'static> Sync for MassSystemChunks<S, T> {}

impl<S: 'static, T: Copy + 'static> MassSystemChunks<S, T> {
    pub fn new() -> Self {
        Self {
            inner: MassChunks::new(),
            _marker: PhantomData,
        }
    }
}

impl<S: 'static, T: Copy + 'static> std::ops::Deref for MassSystemChunks<S, T> {
    type Target = MassChunks<T>;
    fn deref(&self) -> &MassChunks<T> {
        &self.inner
    }
}

impl<S: 'static, T: Copy + 'static> std::ops::DerefMut for MassSystemChunks<S, T> {
    fn deref_mut(&mut self) -> &mut MassChunks<T> {
        &mut self.inner
    }
}

impl<T: Copy + 'static> MassChunks<T> {
    pub fn new() -> Self {
        Self {
            primary_slices: Vec::new(),
            global_desc: None,
            _phantom: PhantomData,
        }
    }

    /// Add a primary chunk slice (one per archetype chunk).
    ///
    /// # Safety
    /// The slice data pointer must be valid for the frame duration.
    pub unsafe fn push_primary_slice(&mut self, slice: unreal_ffi::MassFragmentSlice) {
        self.primary_slices.push(slice);
    }

    /// Set the global chunked fragment descriptor.
    ///
    /// # Safety
    /// The descriptor and its chunk pointers must be valid for the frame duration.
    pub unsafe fn set_global(&mut self, desc: *const unreal_ffi::MassGlobalFragmentChunks) {
        self.global_desc = Some(desc);
    }

    pub fn clear(&mut self) {
        self.primary_slices.clear();
        self.global_desc = None;
    }

    pub fn primary_chunk_count(&self) -> usize {
        self.primary_slices.len()
    }

    /// Get a read-only query for primary chunk at index.
    ///
    /// # Safety
    /// Caller must ensure no mutable alias to the same chunk data exists.
    pub unsafe fn primary_chunk_ref(&self, index: usize) -> MassQueryRef<'_, T> {
        let slice = &self.primary_slices[index];
        unsafe { MassQueryRef::from_raw(slice.data as *const c_void, slice.count as usize) }
    }

    /// Get a mutable query for primary chunk at index.
    ///
    /// # Safety
    /// Caller must ensure no other alias to the same chunk data exists.
    pub unsafe fn primary_chunk_mut(&mut self, index: usize) -> MassQueryMut<'_, T> {
        let slice = &self.primary_slices[index];
        unsafe { MassQueryMut::from_raw(slice.data, slice.count as usize) }
    }

    pub fn global(&self) -> Option<*const unreal_ffi::MassGlobalFragmentChunks> {
        self.global_desc
    }

    /// Get a mutable global query across all chunks.
    ///
    /// # Safety
    /// Caller must ensure the global descriptor pointer is still valid.
    pub unsafe fn global_query_mut(&mut self) -> Option<MassQueryAllMut<'_, T>> {
        self.global_desc
            .map(|desc| unsafe { MassQueryAllMut::from_chunked(desc) })
    }

    /// Get a read-only global query across all chunks.
    ///
    /// # Safety
    /// Caller must ensure the global descriptor pointer is still valid.
    pub unsafe fn global_query_ref(&self) -> Option<MassQueryAllRef<'_, T>> {
        self.global_desc
            .map(|desc| unsafe { MassQueryAllRef::from_chunked(desc) })
    }
}

// ---------------------------------------------------------------------------
// System ordering
// ---------------------------------------------------------------------------

/// Stage marker for ordering Mass Entity systems in the Bevy schedule.
/// Systems in `MassSystemStage(i)` run after `MassSystemStage(i-1)`.
#[derive(bevy_ecs::prelude::SystemSet, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct MassSystemStage(pub u32);

/// Holds a Bevy World and Schedule for running Mass Entity systems.
/// The World contains per-fragment `MassChunks<T>` resources and `bevy_time::Time`.
/// The Schedule contains the Bevy wrapper systems generated by `#[mass_system]`.
pub struct MassSchedule {
    app: bevy_app::App,
}

impl MassSchedule {
    pub fn new() -> Self {
        let mut app = bevy_app::App::new();
        // Force single-threaded execution on Update: MassChunks<T> holds raw pointers
        // into C++ Mass Entity chunk memory. Multi-threaded execution requires auditing
        // pointer safety and ensuring the spatial query callback is thread-safe.
        // (Main schedule is already single-threaded via MainSchedulePlugin.)
        app.edit_schedule(bevy_app::Update, |schedule| {
            schedule.set_executor_kind(bevy_ecs::schedule::ExecutorKind::SingleThreaded);
        });
        app.add_plugins(bevy_time::TimePlugin);
        app.insert_resource(MassEntityMap::default());
        Self { app }
    }

    pub fn set_dt(&mut self, dt: f32) {
        *self.app.world_mut().resource_mut::<bevy_time::TimeUpdateStrategy>() =
            bevy_time::TimeUpdateStrategy::ManualDuration(
                core::time::Duration::from_secs_f32(dt),
            );
    }

    pub fn run(&mut self) {
        self.app.update();
    }

    pub fn app(&self) -> &bevy_app::App {
        &self.app
    }

    pub fn app_mut(&mut self) -> &mut bevy_app::App {
        &mut self.app
    }

    pub fn world(&self) -> &bevy_ecs::world::World {
        self.app.world()
    }

    pub fn world_mut(&mut self) -> &mut bevy_ecs::world::World {
        self.app.world_mut()
    }
}

// ---------------------------------------------------------------------------
// Shadow Bevy entity map
// ---------------------------------------------------------------------------

// ---------------------------------------------------------------------------
// Shadow-world accessor hooks (set by `unreal-module` at init time).
//
// `TestCtx::bevy_get` / `bevy_insert` / `bevy_entity` need to reach the global
// `MASS_SCHEDULE` that lives in `unreal-module`. Rather than pull that
// dependency up the graph, `unreal-module` registers function pointers here
// and `TestCtx` dispatches through them. Pure Rust — no FFI.
// ---------------------------------------------------------------------------

/// Visit the shadow Bevy world read-only. Returns `true` if the world is
/// available, else `false`. Set by `unreal-module::init_global_schedule`.
pub type ShadowWorldReadFn = fn(
    &mut dyn FnMut(&bevy_ecs::world::World, &MassEntityMap),
) -> bool;

/// Visit the shadow Bevy world mutably. Returns `true` if the world is
/// available, else `false`. Set by `unreal-module::init_global_schedule`.
pub type ShadowWorldWriteFn = fn(
    &mut dyn FnMut(&mut bevy_ecs::world::World, &MassEntityMap),
) -> bool;

static SHADOW_WORLD_READ: std::sync::OnceLock<ShadowWorldReadFn> = std::sync::OnceLock::new();
static SHADOW_WORLD_WRITE: std::sync::OnceLock<ShadowWorldWriteFn> = std::sync::OnceLock::new();

/// Register shadow-world accessors. Called once by `unreal-module` at init.
/// Subsequent calls are no-ops (OnceLock).
pub fn register_shadow_world_accessors(
    read_fn: ShadowWorldReadFn,
    write_fn: ShadowWorldWriteFn,
) {
    let _ = SHADOW_WORLD_READ.set(read_fn);
    let _ = SHADOW_WORLD_WRITE.set(write_fn);
}

pub(crate) fn shadow_world_read_fn() -> Option<ShadowWorldReadFn> {
    SHADOW_WORLD_READ.get().copied()
}

pub(crate) fn shadow_world_write_fn() -> Option<ShadowWorldWriteFn> {
    SHADOW_WORLD_WRITE.get().copied()
}

/// Maps Mass Entity groups to shadow Bevy entities.
///
/// Each named group (e.g., "ants", "food") has a `Vec<Entity>` indexed by
/// spawn order. Shadow entities live in the Bevy World alongside the
/// MassFragment chunk data, allowing pure-Bevy components (like Cooldown)
/// to be attached to entities that also have zero-copy chunk fragments.
#[derive(bevy_ecs::prelude::Resource, Default, Clone)]
pub struct MassEntityMap {
    groups: std::collections::HashMap<String, Vec<bevy_ecs::entity::Entity>>,
}

impl MassEntityMap {
    /// Look up a shadow Bevy entity by group name and index within the group.
    pub fn get(&self, group: &str, index: usize) -> Option<bevy_ecs::entity::Entity> {
        self.groups.get(group)?.get(index).copied()
    }

    /// Get all shadow entities for a named group.
    pub fn group(&self, group: &str) -> Option<&[bevy_ecs::entity::Entity]> {
        self.groups.get(group).map(|v| v.as_slice())
    }

    /// Register a group of shadow entities.
    pub fn insert_group(&mut self, name: String, entities: Vec<bevy_ecs::entity::Entity>) {
        self.groups.insert(name, entities);
    }

    /// Total number of shadow entities across all groups.
    pub fn total_count(&self) -> usize {
        self.groups.values().map(|v| v.len()).sum()
    }
}

// ---------------------------------------------------------------------------
// Spatial query resources
// ---------------------------------------------------------------------------

/// Bevy resource holding all named spatial query callbacks for the current frame.
/// Game systems access queries by name: `spatial.call("food_pickup", &prev, &cur)`.
pub struct MassSpatialQueries {
    queries: std::collections::HashMap<String, (unreal_ffi::MassSpatialQueryFn, f32)>,
}

impl bevy_ecs::prelude::Resource for MassSpatialQueries {}
unsafe impl Send for MassSpatialQueries {}
unsafe impl Sync for MassSpatialQueries {}

impl Default for MassSpatialQueries {
    fn default() -> Self {
        Self {
            queries: std::collections::HashMap::new(),
        }
    }
}

impl MassSpatialQueries {
    /// Clear all queries (called at start of each frame before population).
    pub fn clear(&mut self) {
        self.queries.clear();
    }

    /// Insert a named query callback + radius.
    pub fn insert(&mut self, name: String, query_fn: unreal_ffi::MassSpatialQueryFn, radius: f32) {
        self.queries.insert(name, (query_fn, radius));
    }

    /// Call a named spatial query. Returns None if query not registered this frame.
    pub fn call(
        &self,
        name: &str,
        previous_pos: &[f64; 3],
        current_pos: &[f64; 3],
    ) -> Option<unreal_ffi::MassSpatialQueryResult> {
        let (query_fn, radius) = self.queries.get(name)?;
        let mut result = unreal_ffi::MassSpatialQueryResult {
            entity_index: -1,
            _pad: 0,
            encounter_position: [0.0; 3],
            has_encounter: false,
            _result_pad: [0; 7],
        };
        let ok = unsafe {
            query_fn(
                previous_pos.as_ptr(),
                current_pos.as_ptr(),
                *radius,
                &mut result,
            )
        };
        if ok != 0 {
            Some(result)
        } else {
            None
        }
    }
}

/// Marker trait for types that live in UE Mass Entity chunk memory.
/// Implemented automatically by `#[component]` and `mass_fragment!`.
/// Used by `QueryBackend` specialization to dispatch queries to chunk
/// or Bevy entity storage at compile time.
pub trait ChunkBacked {}

/// Compile-time dispatch: is this component in chunk memory or Bevy entity storage?
/// Uses `min_specialization` — the compiler resolves `IS_CHUNK` at monomorphization
/// and eliminates the dead code path. Zero runtime cost.
pub trait QueryBackend {
    const IS_CHUNK: bool;
}

impl<T> QueryBackend for T {
    default const IS_CHUNK: bool = false;
}

impl<T: ChunkBacked> QueryBackend for T {
    const IS_CHUNK: bool = true;
}

/// Compile-time fragment detection: does this type have a C++ MassFragment representation?
/// Uses `min_specialization` — resolves at monomorphization, zero runtime cost.
/// Used by the `#[mass_system]` macro to generate requirement entries that compile
/// for both MassFragment types (with C++ registration) and pure-Bevy types (skipped).
pub trait MaybeFragment {
    const IS_FRAGMENT: bool;
    const CPP_TYPE_NAME_OR_EMPTY: &'static str;
}

impl<T> MaybeFragment for T {
    default const IS_FRAGMENT: bool = false;
    default const CPP_TYPE_NAME_OR_EMPTY: &'static str = "";
}

impl<T: MassFragment> MaybeFragment for T {
    const IS_FRAGMENT: bool = true;
    const CPP_TYPE_NAME_OR_EMPTY: &'static str = T::CPP_TYPE_NAME;
}

/// Trait implemented by `#[derive(MassFragment)]` on `#[repr(C)]` structs
/// that match a C++ MassEntity USTRUCT.
pub trait MassFragment: Sized + Copy + 'static {
    /// The C++ USTRUCT type name (e.g. "FGatherersFoodStateFragment").
    const CPP_TYPE_NAME: &'static str;
}

/// Describes one field in a MassFragment struct, used for C++ codegen.
pub struct MassFragmentFieldInfo {
    /// Rust field name (e.g., "carried_food_index").
    pub name: &'static str,
    /// Rust type as a string (e.g., "[f64; 3]", "i32", "bool").
    pub type_name: &'static str,
    /// Byte offset within the struct.
    pub offset: usize,
    /// Size of this field in bytes.
    pub size: usize,
    /// C++ default value expression (empty string = use type default).
    pub default_value: &'static str,
}

/// Compile-time registration entry for a MassFragment, collected via inventory.
pub struct MassFragmentRegistration {
    pub cpp_type_name: &'static str,
    pub rust_type_name: &'static str,
    pub size: usize,
    pub align: usize,
    /// Field metadata for C++ codegen. Empty slice if not available.
    pub fields: &'static [MassFragmentFieldInfo],
    /// If true, this is a FMassTag (no fields, no UPROPERTY).
    pub is_tag: bool,
    /// If true, this is an existing UE type (e.g. FTransformFragment).
    /// Codegen skips USTRUCT generation but still emits sizeof static_assert.
    pub existing: bool,
    /// Header to `#include` for existing types (e.g. "MassCommonFragments.h").
    /// Empty string means no extra include needed.
    pub include_header: &'static str,
    /// Write a default instance into the provided buffer (must be `size` bytes).
    /// Used by codegen to derive C++ defaults from `impl Default`.
    /// `None` for tags or fragments without `Default`.
    pub write_default: Option<fn(*mut u8)>,
}

inventory::collect!(MassFragmentRegistration);

/// Returns all registered MassFragment types.
pub fn registered_mass_fragments() -> inventory::iter<MassFragmentRegistration> {
    inventory::iter::<MassFragmentRegistration>
}

// ---------------------------------------------------------------------------
// C++ fragment codegen
// ---------------------------------------------------------------------------

/// Convert a Rust field type string to C++ type.
/// Returns (cpp_type, is_padding).
fn rust_type_to_cpp(type_name: &str, field_name: &str) -> (String, bool) {
    let is_padding = field_name.starts_with('_');
    // Normalize whitespace: the derive macro emits "[f64; 3]" (no space before ;)
    // but manual test data may use "[f64 ; 3]". Collapse to canonical form.
    let normalized: String = type_name.split_whitespace().collect::<Vec<_>>().join(" ");
    let cpp = match normalized.as_str() {
        "[f64; 3]" | "[f64 ; 3]" | "DVec3" => "FVector".to_string(),
        "f64" => "double".to_string(),
        "f32" => "float".to_string(),
        "i32" => "int32".to_string(),
        "u32" => "uint32".to_string(),
        "bool" => "bool".to_string(),
        s if s.starts_with("[u8;") || s.starts_with("[u8 ;") => {
            // "[u8; N]" or "[u8 ; N]" → "uint8[N]"
            let n = s.trim_start_matches("[u8")
                .trim_start_matches(" ;")
                .trim_start_matches(';')
                .trim()
                .trim_end_matches(']')
                .trim();
            format!("uint8 PLACEHOLDER[{}]", n)
        }
        other => format!("/* unknown: {} */", other),
    };
    (cpp, is_padding)
}

/// Convert snake_case Rust field name to PascalCase C++ name.
fn snake_to_pascal(name: &str) -> String {
    name.split('_')
        .filter(|s| !s.is_empty())
        .map(|s| {
            let mut c = s.chars();
            match c.next() {
                None => String::new(),
                Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
            }
        })
        .collect()
}

/// Format a field's C++ default from runtime default bytes.
/// Reads the field value from `buf` at the field's offset and formats it as a C++ literal.
/// Format a float ensuring it always has a decimal point (C++ requires this).
fn fmt_float(val: f32) -> String {
    let s = format!("{}", val);
    if s.contains('.') || s.contains('e') || s.contains('E') { s } else { format!("{}.0", s) }
}

/// Format a double ensuring it always has a decimal point.
fn fmt_double(val: f64) -> String {
    let s = format!("{}", val);
    if s.contains('.') || s.contains('e') || s.contains('E') { s } else { format!("{}.0", s) }
}

/// Format a field's C++ default from runtime default bytes.
/// Reads the field value from `buf` at the field's offset and formats it as a C++ literal.
fn format_runtime_default(buf: &[u8], field: &MassFragmentFieldInfo, cpp_type: &str) -> String {
    let bytes = &buf[field.offset..field.offset + field.size];
    match cpp_type {
        "FVector" => {
            // DVec3 = 3 × f64 = 24 bytes
            let x = f64::from_ne_bytes(bytes[0..8].try_into().unwrap());
            let y = f64::from_ne_bytes(bytes[8..16].try_into().unwrap());
            let z = f64::from_ne_bytes(bytes[16..24].try_into().unwrap());
            if x == 0.0 && y == 0.0 && z == 0.0 {
                " = FVector::ZeroVector".to_string()
            } else {
                format!(" = FVector({}, {}, {})", fmt_double(x), fmt_double(y), fmt_double(z))
            }
        }
        "float" => {
            let val = f32::from_ne_bytes(bytes[..4].try_into().unwrap());
            format!(" = {}f", fmt_float(val))
        }
        "double" => {
            let val = f64::from_ne_bytes(bytes[..8].try_into().unwrap());
            format!(" = {}", fmt_double(val))
        }
        "int32" => {
            let val = i32::from_ne_bytes(bytes[..4].try_into().unwrap());
            format!(" = {}", val)
        }
        "uint32" => {
            let val = u32::from_ne_bytes(bytes[..4].try_into().unwrap());
            format!(" = {}", val)
        }
        "bool" => {
            let val = bytes[0] != 0;
            format!(" = {}", val)
        }
        _ => String::new(),
    }
}

/// Generate a C++ header with USTRUCT definitions and static_assert offset checks
/// for the given fragment registrations.
pub fn generate_cpp_fragments(fragments: &[&MassFragmentRegistration], output_filename: &str) -> String {
    // Derive UHT generated header name from output filename (e.g. "Foo.gen.h" -> "Foo.gen.generated.h")
    let uht_include = output_filename.strip_suffix(".h").unwrap_or(output_filename);
    let mut out = String::new();
    out.push_str("#pragma once\n\n");
    out.push_str("#include \"CoreMinimal.h\"\n");
    out.push_str("#include \"MassEntityTypes.h\"\n");

    // Emit extra #includes needed by existing types (deduplicated).
    // Must appear before the .generated.h include (UHT requires it last).
    let mut extra_includes: Vec<&str> = fragments
        .iter()
        .filter(|f| f.existing && !f.include_header.is_empty())
        .map(|f| f.include_header)
        .collect();
    extra_includes.sort();
    extra_includes.dedup();
    for header in &extra_includes {
        out.push_str(&format!("#include \"{header}\"\n"));
    }

    out.push_str(&format!("#include \"{uht_include}.generated.h\"\n\n"));
    out.push_str("// Auto-generated from #[derive(MassFragment)] Rust structs.\n");
    out.push_str("// Do not edit manually.\n\n");

    for frag in fragments {
        // Existing UE types: skip USTRUCT generation, emit only sizeof verification
        if frag.existing {
            out.push_str(&format!(
                "// {} — existing UE type, layout verified:\n",
                frag.cpp_type_name
            ));
            // Tags are zero-sized in Rust but may have non-zero size in C++ (GENERATED_BODY).
            // Skip sizeof check for existing tags — their identity is all that matters.
            if !frag.is_tag {
                out.push_str(&format!(
                    "static_assert(sizeof({}) == {}, \"{} size must be {} for Rust interop\");\n",
                    frag.cpp_type_name, frag.size, frag.cpp_type_name, frag.size
                ));
            }
            out.push('\n');
            continue;
        }

        let base_class = if frag.is_tag { "FMassTag" } else { "FMassFragment" };

        // USTRUCT definition
        out.push_str("USTRUCT()\n");
        out.push_str(&format!("struct {} : public {}\n", frag.cpp_type_name, base_class));
        out.push_str("{\n");
        out.push_str("\tGENERATED_BODY()\n");

        if frag.is_tag {
            // Tags have no fields, just close the struct
            out.push_str("};\n\n");
            continue;
        }

        out.push('\n');

        // Get runtime default bytes (if available) for deriving C++ defaults
        // from the Rust `impl Default`.
        let default_bytes: Option<Vec<u8>> = frag.write_default.map(|write_fn| {
            let mut buf = vec![0u8; frag.size];
            write_fn(buf.as_mut_ptr());
            buf
        });

        // Emit fields, auto-inserting C++ padding for repr(C) alignment gaps.
        // Track current byte offset to detect gaps between fields.
        let mut cursor: usize = 0;

        for field in frag.fields {
            // Skip explicit _pad fields (legacy — repr(C) handles alignment)
            if field.name.starts_with('_') {
                continue;
            }

            // If there's a gap between cursor and this field's offset, emit padding
            if field.offset > cursor {
                let gap = field.offset - cursor;
                out.push_str(&format!("\tuint8 _Pad_{cursor}[{gap}] = {{}};\n"));
            }

            let (cpp_type, _) = rust_type_to_cpp(field.type_name, field.name);
            let cpp_name = if cpp_type == "bool" {
                format!("b{}", snake_to_pascal(field.name))
            } else {
                snake_to_pascal(field.name)
            };

            if cpp_type.contains("PLACEHOLDER") {
                let cpp_type = cpp_type.replace("PLACEHOLDER", &cpp_name);
                out.push_str(&format!("\t{} = {{}};\n", cpp_type));
            } else {
                // Priority: explicit #[mass(default = "...")] > runtime default > type zero
                let default = if !field.default_value.is_empty() {
                    format!(" = {}", field.default_value)
                } else if let Some(ref buf) = default_bytes {
                    format_runtime_default(buf, field, &cpp_type)
                } else {
                    match cpp_type.as_str() {
                        "FVector" => " = FVector::ZeroVector".to_string(),
                        "double" => " = 0.0".to_string(),
                        "float" => " = 0.0f".to_string(),
                        "int32" | "uint32" => " = 0".to_string(),
                        "bool" => " = false".to_string(),
                        _ => String::new(),
                    }
                };
                out.push_str(&format!("\t{} {}{default};\n", cpp_type, cpp_name));
            }

            cursor = field.offset + field.size;
        }

        // Trailing padding to match struct size
        if cursor < frag.size {
            let gap = frag.size - cursor;
            out.push_str(&format!("\tuint8 _Pad_{cursor}[{gap}] = {{}};\n"));
        }

        out.push_str("};\n\n");

        // static_assert offset checks (only for real fields, not auto-padding)
        for field in frag.fields {
            if field.name.starts_with('_') {
                continue;
            }
            let (cpp_type, _) = rust_type_to_cpp(field.type_name, field.name);
            let cpp_name = if cpp_type == "bool" {
                format!("b{}", snake_to_pascal(field.name))
            } else {
                snake_to_pascal(field.name)
            };
            if cpp_type.contains("PLACEHOLDER") {
                continue;
            }
            out.push_str(&format!(
                "static_assert(offsetof({}, {}) == {}, \"{} at offset {}\");\n",
                frag.cpp_type_name, cpp_name, field.offset, cpp_name, field.offset
            ));
        }
        out.push_str(&format!(
            "static_assert(sizeof({}) == {}, \"{} size must be {}\");\n\n",
            frag.cpp_type_name, frag.size, frag.cpp_type_name, frag.size
        ));
    }

    out
}

/// Run fragment codegen: collect registered fragments, sort deterministically,
/// generate C++ header, write to output path (from argv[1] or default).
///
/// Game crates create a `[[bin]]` that links their lib (`use my_game as _`)
/// and calls this function. The link ensures inventory collects all registrations.
pub fn run_fragment_codegen(default_output: &std::path::Path) {
    let regs: Vec<&MassFragmentRegistration> = registered_mass_fragments().into_iter().collect();
    let mut tags: Vec<_> = regs.iter().copied().filter(|r| r.is_tag).collect();
    let mut fragments: Vec<_> = regs.iter().copied().filter(|r| !r.is_tag).collect();
    tags.sort_by_key(|r| r.cpp_type_name);
    fragments.sort_by_key(|r| r.cpp_type_name);
    let mut all = tags;
    all.extend(fragments);
    let out_path = std::env::args()
        .nth(1)
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|| default_output.to_path_buf());
    let filename = out_path.file_name().unwrap_or_default().to_str().unwrap_or("GeneratedFragments.h");
    let output = generate_cpp_fragments(&all, filename);
    if let Some(parent) = out_path.parent() {
        std::fs::create_dir_all(parent).ok();
    }
    std::fs::write(&out_path, &output).expect("Failed to write generated fragments");
    println!("Wrote {}", out_path.display());
}

// ---------------------------------------------------------------------------
// Simulation init registration
// ---------------------------------------------------------------------------

/// Game crates register a simulation init function that returns named entity groups.
pub struct MassSimInitRegistration {
    pub name: &'static str,
    pub init_fn: fn(
        params: &unreal_ffi::MassInitSimulationParams,
    ) -> Vec<(String, Vec<unreal_ffi::MassEntityHandle>)>,
}

inventory::collect!(MassSimInitRegistration);

pub fn registered_sim_inits() -> inventory::iter<MassSimInitRegistration> {
    inventory::iter::<MassSimInitRegistration>
}

// ---------------------------------------------------------------------------
// Visualizer group registration
// ---------------------------------------------------------------------------

/// Game crates register visual groups via inventory.
/// Each group maps an entity type to its position fragment and scale.
pub struct MassVisualizerGroupRegistration {
    pub name: &'static str,
    pub position_fragment_type: &'static str,
    pub position_offset: usize,
    pub scale: f32,
}

inventory::collect!(MassVisualizerGroupRegistration);

pub fn registered_visualizer_groups() -> inventory::iter<MassVisualizerGroupRegistration> {
    inventory::iter::<MassVisualizerGroupRegistration>
}

// ---------------------------------------------------------------------------
// Primary query types (per-chunk)
// ---------------------------------------------------------------------------

/// Read-only view over fragment data in a chunk.
/// Generated by the macro for `MassQuery<&T>`.
pub struct MassQueryRef<'a, T: Copy + 'static> {
    data: &'a [T],
}

impl<'a, T: Copy + 'static> MassQueryRef<'a, T> {
    /// # Safety
    /// `ptr` must point to a valid array of `count` elements of type `T`.
    pub unsafe fn from_raw(ptr: *const c_void, count: usize) -> Self {
        let data = unsafe { std::slice::from_raw_parts(ptr as *const T, count) };
        Self { data }
    }

    pub fn iter(&self) -> std::slice::Iter<'_, T> {
        self.data.iter()
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    pub fn as_slice(&self) -> &[T] {
        self.data
    }

    pub fn into_slice(self) -> &'a [T] {
        self.data
    }
}

impl<'b, 'a, T: Copy + 'static> IntoIterator for &'b MassQueryRef<'a, T> {
    type Item = &'b T;
    type IntoIter = std::slice::Iter<'b, T>;
    fn into_iter(self) -> Self::IntoIter {
        self.data.iter()
    }
}

/// Read-write view over fragment data in a chunk.
/// Generated by the macro for `MassQuery<&mut T>`.
pub struct MassQueryMut<'a, T: Copy + 'static> {
    data: &'a mut [T],
}

impl<'a, T: Copy + 'static> MassQueryMut<'a, T> {
    /// # Safety
    /// `ptr` must point to a valid array of `count` elements of type `T`.
    pub unsafe fn from_raw(ptr: *mut c_void, count: usize) -> Self {
        let data = unsafe { std::slice::from_raw_parts_mut(ptr as *mut T, count) };
        Self { data }
    }

    pub fn iter(&self) -> std::slice::Iter<'_, T> {
        self.data.iter()
    }

    pub fn iter_mut(&mut self) -> std::slice::IterMut<'_, T> {
        self.data.iter_mut()
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    pub fn as_slice(&self) -> &[T] {
        self.data
    }

    pub fn as_mut_slice(&mut self) -> &mut [T] {
        self.data
    }

    pub fn into_slice(self) -> &'a mut [T] {
        self.data
    }
}

impl<'b, 'a, T: Copy + 'static> IntoIterator for &'b MassQueryMut<'a, T> {
    type Item = &'b T;
    type IntoIter = std::slice::Iter<'b, T>;
    fn into_iter(self) -> Self::IntoIter {
        self.data.iter()
    }
}

impl<'b, 'a, T: Copy + 'static> IntoIterator for &'b mut MassQueryMut<'a, T> {
    type Item = &'b mut T;
    type IntoIter = std::slice::IterMut<'b, T>;
    fn into_iter(self) -> Self::IntoIter {
        self.data.iter_mut()
    }
}

// ---------------------------------------------------------------------------
// Dual-mode query types (chunk OR Bevy entity storage, selected at compile time)
// ---------------------------------------------------------------------------

/// Read-only dual-mode query: backed by either chunk memory or collected Bevy query results.
/// Used by `#[mass_system]` codegen for auto-detected query dispatch.
pub enum DualQueryRef<'a, T: 'static> {
    Chunk(&'a [T]),
    Bevy(Vec<*const T>, std::marker::PhantomData<&'a T>),
    Empty,
}

impl<'a, T: 'static> DualQueryRef<'a, T> {
    pub fn from_chunk(data: &'a [T]) -> Self {
        DualQueryRef::Chunk(data)
    }

    pub fn from_chunk_query(q: MassQueryRef<'a, T>) -> Self
    where T: MassFragment {
        DualQueryRef::Chunk(q.into_slice())
    }

    pub fn iter(&self) -> DualIterRef<'_, T> {
        self.into_iter()
    }

    pub fn len(&self) -> usize {
        match self {
            DualQueryRef::Chunk(s) => s.len(),
            DualQueryRef::Bevy(v, _) => v.len(),
            DualQueryRef::Empty => 0,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn reborrow(&self) -> DualQueryRef<'_, T> {
        match self {
            DualQueryRef::Chunk(s) => DualQueryRef::Chunk(s),
            DualQueryRef::Bevy(v, _) => DualQueryRef::Bevy(v.clone(), std::marker::PhantomData),
            DualQueryRef::Empty => DualQueryRef::Empty,
        }
    }
}

/// Iterator for `DualQueryRef` — yields `&T` from either chunk or Bevy storage.
pub enum DualIterRef<'a, T: 'static> {
    Chunk(std::slice::Iter<'a, T>),
    Bevy(std::slice::Iter<'a, *const T>, std::marker::PhantomData<&'a T>),
    Empty,
}

impl<'a, T: 'static> Iterator for DualIterRef<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        match self {
            DualIterRef::Chunk(iter) => iter.next(),
            DualIterRef::Bevy(iter, _) => iter.next().map(|ptr| unsafe { &**ptr }),
            DualIterRef::Empty => None,
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        match self {
            DualIterRef::Chunk(iter) => iter.size_hint(),
            DualIterRef::Bevy(iter, _) => iter.size_hint(),
            DualIterRef::Empty => (0, Some(0)),
        }
    }
}

impl<'a, T: 'static> ExactSizeIterator for DualIterRef<'a, T> {}

impl<'b, 'a, T: 'static> IntoIterator for &'b DualQueryRef<'a, T> {
    type Item = &'b T;
    type IntoIter = DualIterRef<'b, T>;
    fn into_iter(self) -> Self::IntoIter {
        match self {
            DualQueryRef::Chunk(s) => DualIterRef::Chunk(s.iter()),
            DualQueryRef::Bevy(v, _) => DualIterRef::Bevy(v.iter(), std::marker::PhantomData),
            DualQueryRef::Empty => DualIterRef::Empty,
        }
    }
}

/// Read-write dual-mode query: backed by either chunk memory or collected Bevy query results.
/// Used by `#[mass_system]` codegen for auto-detected query dispatch.
pub enum DualQueryMut<'a, T: 'static> {
    Chunk(&'a mut [T]),
    Bevy(Vec<*mut T>, std::marker::PhantomData<&'a mut T>),
    Empty,
}

impl<'a, T: 'static> DualQueryMut<'a, T> {
    pub fn from_chunk(data: &'a mut [T]) -> Self {
        DualQueryMut::Chunk(data)
    }

    pub fn from_chunk_query(q: MassQueryMut<'a, T>) -> Self
    where T: MassFragment {
        DualQueryMut::Chunk(q.into_slice())
    }

    pub fn iter(&self) -> DualIterRef2<'_, T> {
        self.into_iter()
    }

    pub fn iter_mut(&mut self) -> DualIterMut<'_, T> {
        self.into_iter()
    }

    pub fn len(&self) -> usize {
        match self {
            DualQueryMut::Chunk(s) => s.len(),
            DualQueryMut::Bevy(v, _) => v.len(),
            DualQueryMut::Empty => 0,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Reborrow for passing into multiple chunk iterations.
    /// Chunk variant reborrows the slice; Bevy variant clones the pointer vec.
    pub fn reborrow(&mut self) -> DualQueryMut<'_, T> {
        match self {
            DualQueryMut::Chunk(s) => DualQueryMut::Chunk(s),
            DualQueryMut::Bevy(v, _) => DualQueryMut::Bevy(v.clone(), std::marker::PhantomData),
            DualQueryMut::Empty => DualQueryMut::Empty,
        }
    }
}

/// Iterator for `DualQueryMut` — yields `&T` from either chunk or Bevy storage.
pub enum DualIterRef2<'a, T: 'static> {
    Chunk(std::slice::Iter<'a, T>),
    Bevy(std::slice::Iter<'a, *mut T>, std::marker::PhantomData<&'a T>),
    Empty,
}

impl<'a, T: 'static> Iterator for DualIterRef2<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        match self {
            DualIterRef2::Chunk(iter) => iter.next(),
            DualIterRef2::Bevy(iter, _) => iter.next().map(|ptr| unsafe { &**ptr }),
            DualIterRef2::Empty => None,
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        match self {
            DualIterRef2::Chunk(iter) => iter.size_hint(),
            DualIterRef2::Bevy(iter, _) => iter.size_hint(),
            DualIterRef2::Empty => (0, Some(0)),
        }
    }
}

impl<'a, T: 'static> ExactSizeIterator for DualIterRef2<'a, T> {}

/// Mutable iterator for `DualQueryMut` — yields `&mut T`.
pub enum DualIterMut<'a, T: 'static> {
    Chunk(std::slice::IterMut<'a, T>),
    Bevy(std::slice::Iter<'a, *mut T>, std::marker::PhantomData<&'a mut T>),
    Empty,
}

impl<'a, T: 'static> Iterator for DualIterMut<'a, T> {
    type Item = &'a mut T;
    fn next(&mut self) -> Option<Self::Item> {
        match self {
            DualIterMut::Chunk(iter) => iter.next(),
            DualIterMut::Bevy(iter, _) => iter.next().map(|ptr| unsafe { &mut **ptr }),
            DualIterMut::Empty => None,
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        match self {
            DualIterMut::Chunk(iter) => iter.size_hint(),
            DualIterMut::Bevy(iter, _) => iter.size_hint(),
            DualIterMut::Empty => (0, Some(0)),
        }
    }
}

impl<'a, T: 'static> ExactSizeIterator for DualIterMut<'a, T> {}

impl<'b, 'a, T: 'static> IntoIterator for &'b DualQueryMut<'a, T> {
    type Item = &'b T;
    type IntoIter = DualIterRef2<'b, T>;
    fn into_iter(self) -> Self::IntoIter {
        match self {
            DualQueryMut::Chunk(s) => DualIterRef2::Chunk(s.iter()),
            DualQueryMut::Bevy(v, _) => DualIterRef2::Bevy(v.iter(), std::marker::PhantomData),
            DualQueryMut::Empty => DualIterRef2::Empty,
        }
    }
}

impl<'b, 'a, T: 'static> IntoIterator for &'b mut DualQueryMut<'a, T> {
    type Item = &'b mut T;
    type IntoIter = DualIterMut<'b, T>;
    fn into_iter(self) -> Self::IntoIter {
        match self {
            DualQueryMut::Chunk(s) => DualIterMut::Chunk(s.iter_mut()),
            DualQueryMut::Bevy(v, _) => DualIterMut::Bevy(v.iter(), std::marker::PhantomData),
            DualQueryMut::Empty => DualIterMut::Empty,
        }
    }
}

// ---------------------------------------------------------------------------
// Global query types (cross-archetype, all matching entities, zero-copy)
// ---------------------------------------------------------------------------

/// Read-only view over ALL entities matching a fragment type, across all chunks.
/// Zero-copy: points directly into Mass Entity chunk memory.
/// Generated by the macro for `MassQueryAll<&T>`.
pub struct MassQueryAllRef<'a, T: Copy + 'static> {
    desc: &'a unreal_ffi::MassGlobalFragmentChunks,
    _phantom: std::marker::PhantomData<&'a T>,
}

impl<'a, T: Copy + 'static> MassQueryAllRef<'a, T> {
    const EMPTY_DESC: unreal_ffi::MassGlobalFragmentChunks = unreal_ffi::MassGlobalFragmentChunks {
        chunks: std::ptr::null(),
        num_chunks: 0,
        total_count: 0,
        stride: 0,
    };

    /// Create an empty global query (no entities match).
    pub fn empty() -> Self {
        Self {
            desc: &Self::EMPTY_DESC,
            _phantom: std::marker::PhantomData,
        }
    }

    /// Construct from a chunked descriptor (zero-copy path, used by generated code).
    ///
    /// # Safety
    /// `desc` must point to a valid `MassGlobalFragmentChunks` whose chunk data pointers
    /// are valid for `'a` and contain properly laid out `T` values.
    pub unsafe fn from_chunked(desc: *const unreal_ffi::MassGlobalFragmentChunks) -> Self {
        Self {
            desc: unsafe { &*desc },
            _phantom: std::marker::PhantomData,
        }
    }

    /// Construct from a contiguous slice (for tests).
    ///
    /// # Safety
    /// `ptr` must point to a valid array of `count` elements of type `T`.
    pub unsafe fn from_raw_single_chunk(
        ptr: *const c_void,
        count: usize,
        chunk_storage: &'a mut MassGlobalChunkStorage,
    ) -> Self {
        chunk_storage.init_single(ptr as *mut c_void, count, std::mem::size_of::<T>());
        Self {
            desc: &chunk_storage.desc,
            _phantom: std::marker::PhantomData,
        }
    }

    pub fn get(&self, global_index: usize) -> Option<&T> {
        chunked_get(self.desc, global_index)
    }

    pub fn iter(&self) -> MassQueryAllIter<'_, T> {
        MassQueryAllIter::new(self.desc)
    }

    pub fn len(&self) -> usize {
        self.desc.total_count as usize
    }

    pub fn is_empty(&self) -> bool {
        self.desc.total_count == 0
    }
}

/// Read-write view over ALL entities matching a fragment type, across all chunks.
/// Zero-copy: points directly into Mass Entity chunk memory, no write-back needed.
/// Generated by the macro for `MassQueryAll<&mut T>`.
pub struct MassQueryAllMut<'a, T: Copy + 'static> {
    desc: &'a unreal_ffi::MassGlobalFragmentChunks,
    _phantom: std::marker::PhantomData<&'a mut T>,
}

impl<'a, T: Copy + 'static> MassQueryAllMut<'a, T> {
    const EMPTY_DESC: unreal_ffi::MassGlobalFragmentChunks = unreal_ffi::MassGlobalFragmentChunks {
        chunks: std::ptr::null(),
        num_chunks: 0,
        total_count: 0,
        stride: 0,
    };

    /// Create an empty global query (no entities match).
    pub fn empty() -> Self {
        Self {
            desc: &Self::EMPTY_DESC,
            _phantom: std::marker::PhantomData,
        }
    }

    /// Construct from a chunked descriptor (zero-copy path, used by generated code).
    ///
    /// # Safety
    /// `desc` must point to a valid `MassGlobalFragmentChunks` whose chunk data pointers
    /// are valid for `'a` and contain properly laid out `T` values.
    pub unsafe fn from_chunked(desc: *const unreal_ffi::MassGlobalFragmentChunks) -> Self {
        Self {
            desc: unsafe { &*desc },
            _phantom: std::marker::PhantomData,
        }
    }

    /// Construct from a contiguous slice (for tests).
    ///
    /// # Safety
    /// `ptr` must point to a valid mutable array of `count` elements of type `T`.
    pub unsafe fn from_raw_single_chunk(
        ptr: *mut c_void,
        count: usize,
        chunk_storage: &'a mut MassGlobalChunkStorage,
    ) -> Self {
        chunk_storage.init_single(ptr, count, std::mem::size_of::<T>());
        Self {
            desc: &chunk_storage.desc,
            _phantom: std::marker::PhantomData,
        }
    }

    pub fn get(&self, global_index: usize) -> Option<&T> {
        chunked_get(self.desc, global_index)
    }

    pub fn get_mut(&mut self, global_index: usize) -> Option<&mut T> {
        chunked_get_mut(self.desc, global_index)
    }

    pub fn iter(&self) -> MassQueryAllIter<'_, T> {
        MassQueryAllIter::new(self.desc)
    }

    pub fn iter_mut(&mut self) -> MassQueryAllIterMut<'_, T> {
        MassQueryAllIterMut::new(self.desc)
    }

    pub fn len(&self) -> usize {
        self.desc.total_count as usize
    }

    pub fn is_empty(&self) -> bool {
        self.desc.total_count == 0
    }

    /// Diagnostic: returns (num_chunks, total_count, stride, first_chunk_data_addr, first_chunk_count).
    pub fn debug_info(&self) -> (u32, i32, u32, usize, i32) {
        let (first_addr, first_count) = if self.desc.num_chunks > 0 && !self.desc.chunks.is_null() {
            let c = unsafe { &*self.desc.chunks };
            (c.data as usize, c.count)
        } else {
            (0, 0)
        };
        (self.desc.num_chunks, self.desc.total_count, self.desc.stride, first_addr, first_count)
    }
}

// ---------------------------------------------------------------------------
// Chunked access helpers
// ---------------------------------------------------------------------------

/// Heap storage for a single-chunk descriptor, used in tests.
/// Keeps the `MassGlobalChunkSlice` and `MassGlobalFragmentChunks` alive.
pub struct MassGlobalChunkStorage {
    chunk: unreal_ffi::MassGlobalChunkSlice,
    desc: unreal_ffi::MassGlobalFragmentChunks,
}

impl MassGlobalChunkStorage {
    pub fn new() -> Self {
        Self {
            chunk: unreal_ffi::MassGlobalChunkSlice {
                data: std::ptr::null_mut(),
                count: 0,
                stride: 0,
            },
            desc: unreal_ffi::MassGlobalFragmentChunks {
                chunks: std::ptr::null(),
                num_chunks: 0,
                total_count: 0,
                stride: 0,
            },
        }
    }

    fn init_single(&mut self, ptr: *mut c_void, count: usize, stride: usize) {
        self.chunk = unreal_ffi::MassGlobalChunkSlice {
            data: ptr,
            count: count as i32,
            stride: stride as u32,
        };
        self.desc = unreal_ffi::MassGlobalFragmentChunks {
            chunks: &self.chunk as *const _,
            num_chunks: 1,
            total_count: count as i32,
            stride: stride as u32,
        };
    }
}

fn chunked_get<'a, T: Copy + 'static>(
    desc: &'a unreal_ffi::MassGlobalFragmentChunks,
    global_index: usize,
) -> Option<&'a T> {
    let mut remaining = global_index;
    for i in 0..desc.num_chunks as usize {
        let chunk = unsafe { &*desc.chunks.add(i) };
        let count = chunk.count as usize;
        if remaining < count {
            return Some(unsafe { &*(chunk.data as *const T).add(remaining) });
        }
        remaining -= count;
    }
    None
}

fn chunked_get_mut<'a, T: Copy + 'static>(
    desc: &'a unreal_ffi::MassGlobalFragmentChunks,
    global_index: usize,
) -> Option<&'a mut T> {
    let mut remaining = global_index;
    for i in 0..desc.num_chunks as usize {
        let chunk = unsafe { &*desc.chunks.add(i) };
        let count = chunk.count as usize;
        if remaining < count {
            return Some(unsafe { &mut *(chunk.data as *mut T).add(remaining) });
        }
        remaining -= count;
    }
    None
}

/// Read-only iterator across all chunks of a global query.
pub struct MassQueryAllIter<'a, T: Copy + 'static> {
    desc: &'a unreal_ffi::MassGlobalFragmentChunks,
    chunk_idx: usize,
    offset: usize,
    _phantom: std::marker::PhantomData<&'a T>,
}

impl<'a, T: Copy + 'static> MassQueryAllIter<'a, T> {
    fn new(desc: &'a unreal_ffi::MassGlobalFragmentChunks) -> Self {
        Self {
            desc,
            chunk_idx: 0,
            offset: 0,
            _phantom: std::marker::PhantomData,
        }
    }
}

impl<'a, T: Copy + 'static> Iterator for MassQueryAllIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if self.chunk_idx >= self.desc.num_chunks as usize {
                return None;
            }
            let chunk = unsafe { &*self.desc.chunks.add(self.chunk_idx) };
            if self.offset < chunk.count as usize {
                let item = unsafe { &*(chunk.data as *const T).add(self.offset) };
                self.offset += 1;
                return Some(item);
            }
            self.chunk_idx += 1;
            self.offset = 0;
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let total = self.desc.total_count as usize;
        (0, Some(total))
    }
}

/// Mutable iterator across all chunks of a global query.
pub struct MassQueryAllIterMut<'a, T: Copy + 'static> {
    desc: &'a unreal_ffi::MassGlobalFragmentChunks,
    chunk_idx: usize,
    offset: usize,
    _phantom: std::marker::PhantomData<&'a mut T>,
}

impl<'a, T: Copy + 'static> MassQueryAllIterMut<'a, T> {
    fn new(desc: &'a unreal_ffi::MassGlobalFragmentChunks) -> Self {
        Self {
            desc,
            chunk_idx: 0,
            offset: 0,
            _phantom: std::marker::PhantomData,
        }
    }
}

impl<'a, T: Copy + 'static> Iterator for MassQueryAllIterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if self.chunk_idx >= self.desc.num_chunks as usize {
                return None;
            }
            let chunk = unsafe { &*self.desc.chunks.add(self.chunk_idx) };
            if self.offset < chunk.count as usize {
                let item = unsafe { &mut *(chunk.data as *mut T).add(self.offset) };
                self.offset += 1;
                return Some(item);
            }
            self.chunk_idx += 1;
            self.offset = 0;
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let total = self.desc.total_count as usize;
        (0, Some(total))
    }
}

// ---------------------------------------------------------------------------
// Entity spawning
// ---------------------------------------------------------------------------

/// Builder for declaring an entity archetype and spawning entities of that type.
///
/// # Example
/// ```ignore
/// use gatherers_sim::fragments::*;
///
/// let handles = EntityArchetype::new("ant")
///     .fragment::<Position>()
///     .fragment::<Movement>()
///     .tag::<AntTag>()
///     .spawn(100, |index, writer| {
///         writer.set(&Position { position: [index as f64 * 10.0, 0.0, 50.0], ..Default::default() });
///         writer.set(&Movement { direction: [1.0, 0.0, 0.0], movement_speed: 100.0, ..Default::default() });
///     });
/// ```
pub struct EntityArchetype {
    pub name: &'static str,
    fragments: Vec<ArchetypeFragmentDesc>,
    tags: Vec<&'static str>,
}

struct ArchetypeFragmentDesc {
    cpp_type_name: &'static str,
    size: usize,
    align: usize,
}

/// Provides typed write access to one entity's fragment data during spawn.
pub struct EntityWriter<'a> {
    /// Per-fragment-type data buffers: (cpp_type_name, ptr, size)
    buffers: &'a mut [(&'static str, *mut u8, usize)],
    entity_index: usize,
}

impl<'a> EntityWriter<'a> {
    /// Write initial data for a fragment type. The fragment must be part of the archetype.
    /// Panics if the type doesn't match any fragment in the archetype.
    pub fn set<T: MassFragment + Copy>(&mut self, value: &T) {
        let size = std::mem::size_of::<T>();
        let name = T::CPP_TYPE_NAME;
        for &mut (frag_name, ptr, frag_size) in self.buffers.iter_mut() {
            if frag_name == name {
                assert_eq!(frag_size, size, "fragment size mismatch for {}", name);
                unsafe {
                    let dst = ptr.add(self.entity_index * frag_size);
                    std::ptr::copy_nonoverlapping(value as *const T as *const u8, dst, size);
                }
                return;
            }
        }
        panic!("EntityWriter::set: no fragment matching {} in archetype", name);
    }
}

impl EntityArchetype {
    pub fn new(name: &'static str) -> Self {
        Self {
            name,
            fragments: Vec::new(),
            tags: Vec::new(),
        }
    }

    /// Add a fragment type to this archetype.
    pub fn fragment<T: MassFragment + Default>(mut self) -> Self {
        self.fragments.push(ArchetypeFragmentDesc {
            cpp_type_name: T::CPP_TYPE_NAME,
            size: std::mem::size_of::<T>(),
            align: std::mem::align_of::<T>(),
        });
        self
    }

    /// Add a tag type to this archetype.
    pub fn tag<T: MassFragment>(mut self) -> Self {
        self.tags.push(T::CPP_TYPE_NAME);
        self
    }

    /// Spawn `count` entities. The `customizer` closure receives each entity's index
    /// and an `EntityWriter` for setting initial fragment values. Fragments not set
    /// by the customizer will use C++ default constructors.
    ///
    /// Returns the entity handles of the spawned entities.
    pub fn spawn(
        &self,
        count: u32,
        mut customizer: impl FnMut(u32, &mut EntityWriter),
    ) -> Vec<unreal_ffi::MassEntityHandle> {
        // Allocate per-fragment-type data buffers
        let mut buffers: Vec<Vec<u8>> = self
            .fragments
            .iter()
            .map(|f| {
                // Initialize with zeros (C++ will memcpy over default-constructed values)
                vec![0u8; f.size * count as usize]
            })
            .collect();

        // Let the customizer write per-entity data
        let mut buf_ptrs: Vec<(&'static str, *mut u8, usize)> = buffers
            .iter_mut()
            .zip(self.fragments.iter())
            .map(|(buf, desc)| (desc.cpp_type_name, buf.as_mut_ptr(), desc.size))
            .collect();

        for i in 0..count {
            let mut writer = EntityWriter {
                buffers: &mut buf_ptrs,
                entity_index: i as usize,
            };
            customizer(i, &mut writer);
        }

        // Build FFI request
        let ffi_fragments: Vec<unreal_ffi::SpawnFragmentData> = self
            .fragments
            .iter()
            .zip(buffers.iter())
            .map(|(desc, buf)| unreal_ffi::SpawnFragmentData {
                cpp_type_name: unreal_ffi::Utf8Str::from(desc.cpp_type_name),
                size: desc.size as u32,
                alignment: desc.align as u32,
                initial_data: buf.as_ptr() as *const c_void,
            })
            .collect();

        let ffi_tags: Vec<unreal_ffi::SpawnTagData> = self
            .tags
            .iter()
            .map(|&name| unreal_ffi::SpawnTagData {
                cpp_type_name: unreal_ffi::Utf8Str::from(name),
            })
            .collect();

        let request = unreal_ffi::SpawnEntityRequest {
            num_fragments: ffi_fragments.len() as u32,
            fragments: ffi_fragments.as_ptr(),
            num_tags: ffi_tags.len() as u32,
            tags: ffi_tags.as_ptr(),
            count,
            _padding: 0,
        };

        let mut handles = vec![unreal_ffi::MassEntityHandle::default(); count as usize];

        let spawn_fn = crate::module::bindings()
            .spawn_entities
            .expect("spawn_entities callback not set — is a Mass subsystem active?");

        let spawned = unsafe { spawn_fn(&request, handles.as_mut_ptr()) };
        handles.truncate(spawned as usize);
        handles
    }
}

// ---------------------------------------------------------------------------
// Backward compatibility: keep MassQuery as alias for generated code
// ---------------------------------------------------------------------------

/// Deprecated: use `MassQueryMut` or `MassQueryRef` directly.
/// Kept for backward compatibility with existing generated code.
pub type MassQuery<'a, T> = MassQueryMut<'a, T>;

// ---------------------------------------------------------------------------
// User-facing marker types (used in function signatures, never constructed)
// ---------------------------------------------------------------------------

/// Marker type for global cross-archetype queries in `#[mass_system]` signatures.
/// Users write `MassQueryAll<&T>` or `MassQueryAll<&mut T>`.
/// The proc macro maps these to `MassQueryAllRef<T>` or `MassQueryAllMut<T>`.
pub struct MassQueryAll<T> {
    _phantom: std::marker::PhantomData<T>,
}

// ---------------------------------------------------------------------------
// Registration
// ---------------------------------------------------------------------------

/// Registration entry for a dynamically registered mass system, collected via inventory.
pub struct MassSystemRegistration {
    pub name: &'static str,
    pub execute_fn: unsafe extern "C" fn(*const unreal_ffi::MassChunkData),
    pub requirements: &'static [MassSystemRequirement],
    /// Execution order within a pipeline. Lower values run first. Default is 0.
    pub order: u32,
}

inventory::collect!(MassSystemRegistration);

/// Describes one fragment requirement for a mass system.
pub struct MassSystemRequirement {
    pub cpp_type_name: &'static str,
    pub size: usize,
    pub align: usize,
    pub access_mode: u8,  // 0 = ReadOnly, 1 = ReadWrite
    pub is_tag: u8,       // 0 = fragment, 1 = tag
    pub query_scope: u8,  // 0 = primary (per-chunk), 1 = global (all)
    pub is_valid: bool,   // false for non-MassFragment types (filtered out before C++)
}

/// Returns all registered mass systems (extern "C" dispatch path).
pub fn registered_mass_systems() -> inventory::iter<MassSystemRegistration> {
    inventory::iter::<MassSystemRegistration>
}

/// Registration entry for a Bevy-scheduled mass system, collected via inventory.
pub struct MassBevySystemRegistration {
    pub name: &'static str,
    /// Execution order (lower runs first). Must match the C++ processor order.
    pub order: u32,
    /// Adds this system to the App's Update schedule in the given stage.
    pub add_to_app: fn(&mut bevy_app::App, MassSystemStage),
    /// Registers message types used by this system (MessageWriter<T>/MessageReader<T>).
    /// Called via `app.add_message::<T>()` which deduplicates internally.
    pub register_messages: fn(&mut bevy_app::App),
    /// Initializes the `MassChunks<T>` resources this system needs in the World.
    pub init_resources: fn(&mut bevy_ecs::world::World),
    /// Clears all `MassChunks<T>` resources this system uses.
    pub clear_resources: fn(&mut bevy_ecs::world::World),
    /// Populates `MassChunks<T>` resources from a C++ system chunk batch.
    ///
    /// # Safety
    /// The batch's chunk pointers must be valid for the frame duration.
    pub populate_resources:
        unsafe fn(&mut bevy_ecs::world::World, &unreal_ffi::MassSystemChunkBatch),
}

inventory::collect!(MassBevySystemRegistration);

/// Returns all registered Bevy-scheduled mass systems.
pub fn registered_bevy_mass_systems() -> inventory::iter<MassBevySystemRegistration> {
    inventory::iter::<MassBevySystemRegistration>
}

/// Plugin-declared system execution order.
///
/// Game code submits one of these per pipeline via `inventory::submit!`,
/// listing `#[mass_system]` function names in the order they should run.
/// The framework maps each name to a numeric `order` value with a fixed
/// stride so both the Bevy schedule and the C++ processor pipeline agree
/// on the sequence.
///
/// This is the Bevy-idiomatic replacement for sprinkling `order = N` on
/// every `#[mass_system]` attribute. Systems whose name is not listed
/// keep whatever `order` value they were declared with (defaulting to
/// `u32::MAX` sentinel, i.e. "run last").
pub struct MassScheduleOrder {
    /// Ordered list of `#[mass_system]` function names, first runs first.
    pub systems: &'static [&'static str],
}

inventory::collect!(MassScheduleOrder);

/// Returns all registered schedule-order declarations.
pub fn registered_schedule_orders() -> inventory::iter<MassScheduleOrder> {
    inventory::iter::<MassScheduleOrder>
}

/// Stride between consecutive schedule entries. Gaps let late-added
/// systems slot in without renumbering everything.
const MASS_SCHEDULE_STRIDE: u32 = 10;

/// Build `name → order` overrides from every registered `MassScheduleOrder`.
///
/// Multiple declarations are merged by appending — each declaration's
/// internal sequence is preserved, and names from later declarations
/// continue numbering past the tail of earlier ones. If the same name
/// appears more than once, the first occurrence wins.
pub fn resolved_schedule_orders() -> std::collections::HashMap<&'static str, u32> {
    let mut map = std::collections::HashMap::new();
    let mut next: u32 = MASS_SCHEDULE_STRIDE;
    for decl in registered_schedule_orders() {
        for name in decl.systems {
            if !map.contains_key(*name) {
                map.insert(*name, next);
                next = next.saturating_add(MASS_SCHEDULE_STRIDE);
            }
        }
    }
    map
}

/// Resolve the effective execution order for a system.
///
/// Explicit `order = N` on `#[mass_system]` still wins (for backward
/// compatibility and any edge cases). If the system was declared without
/// an order (sentinel `u32::MAX`), we consult the schedule override map.
/// Systems that appear in neither retain `u32::MAX`, which sorts them to
/// the end of the pipeline and makes the omission visible.
pub fn effective_order(
    name: &str,
    declared: u32,
    overrides: &std::collections::HashMap<&'static str, u32>,
) -> u32 {
    if declared != u32::MAX {
        return declared;
    }
    overrides.get(name).copied().unwrap_or(u32::MAX)
}

/// Pre/post-dispatch hooks for game-specific logic around `sched.run()`.
///
/// `pre_dispatch` runs inside `catch_unwind`, before `sched.run()`.
/// `post_dispatch` runs after `sched.run()` returns normally (skipped on panic).
/// Hook ordering among multiple registrations is unspecified; hooks must be order-independent.
///
/// Reference implementation: `gatherers-bevy-mass/src/systems.rs`.
pub struct MassDispatchHook {
    pub pre_dispatch: fn(&mut bevy_ecs::world::World),
    pub post_dispatch: fn(&mut bevy_ecs::world::World),
}

inventory::collect!(MassDispatchHook);

pub fn registered_dispatch_hooks() -> inventory::iter<MassDispatchHook> {
    inventory::iter::<MassDispatchHook>
}

/// Registration that lets `mass_init_simulation` populate an
/// `EntityIndex<Tag>` resource from a `MassEntityMap` group.
///
/// Emitted by `#[derive(MassFragment)]` when `#[mass(group = "...")]`
/// is present. One entry per tag type; the `populate_fn` is generated
/// to insert `EntityIndex::<Tag>` into the world with the supplied
/// entity slice.
pub struct MassEntityIndexRegistration {
    pub entity_group: &'static str,
    pub populate_fn: fn(&mut bevy_ecs::world::World, &[bevy_ecs::entity::Entity]),
}

inventory::collect!(MassEntityIndexRegistration);

pub fn registered_entity_index_populations()
    -> inventory::iter<MassEntityIndexRegistration> {
    inventory::iter::<MassEntityIndexRegistration>
}

/// Registration for spawn-time insertion of pure-Rust shadow components
/// onto every shadow `Entity` in an `entity_group`. Used for components
/// that don't live in chunk memory but need a default value per entity
/// (e.g. `Carrying`, `Cooldown`).
///
/// Game crates submit these via `inventory::submit!`. `mass_init_simulation`
/// iterates and calls `insert_fn` for each shadow Entity in the group.
pub struct MassShadowComponentDefault {
    pub entity_group: &'static str,
    pub insert_fn: fn(&mut bevy_ecs::world::World, bevy_ecs::entity::Entity),
}

inventory::collect!(MassShadowComponentDefault);

pub fn registered_shadow_component_defaults()
    -> inventory::iter<MassShadowComponentDefault> {
    inventory::iter::<MassShadowComponentDefault>
}

/// Registration for per-simulation-init world setup. Called from
/// `mass_init_simulation` with a mutable `World` and the raw
/// `MassInitSimulationParams`. Game crates use this to populate resources
/// whose value depends on init params (e.g. `SimBounds` reading
/// `params.bounds_min/max`). Runs after entity groups are built but before
/// any system runs.
pub struct MassSimInitHook {
    pub name: &'static str,
    pub hook_fn: fn(&mut bevy_ecs::world::World, &unreal_ffi::MassInitSimulationParams),
}

inventory::collect!(MassSimInitHook);

pub fn registered_sim_init_hooks() -> inventory::iter<MassSimInitHook> {
    inventory::iter::<MassSimInitHook>
}

// ---------------------------------------------------------------------------
// Per-system timing (opt-in)
// ---------------------------------------------------------------------------
//
// When `UNREAL_RUST_MASS_TIMING=1`, each `#[mass_system]` wrapper records the
// wall-clock time of its body into a per-frame accumulator. `mass_frame_dispatch`
// drains and logs the table after `sched.run()` returns.
//
// Overhead when disabled: one atomic load per system per frame. Enabled: adds
// two `Instant::now()` calls + a mutex lock per system per frame.

static MASS_TIMING_ENABLED: std::sync::atomic::AtomicBool =
    std::sync::atomic::AtomicBool::new(false);
static MASS_TIMING_INITIALIZED: std::sync::atomic::AtomicBool =
    std::sync::atomic::AtomicBool::new(false);

pub fn is_mass_timing_enabled() -> bool {
    use std::sync::atomic::Ordering;
    // Lazy one-shot init from env var. Avoids re-reading env every call.
    if !MASS_TIMING_INITIALIZED.load(Ordering::Relaxed) {
        let on = std::env::var("UNREAL_RUST_MASS_TIMING")
            .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
            .unwrap_or(false);
        MASS_TIMING_ENABLED.store(on, Ordering::Relaxed);
        MASS_TIMING_INITIALIZED.store(true, Ordering::Relaxed);
    }
    MASS_TIMING_ENABLED.load(Ordering::Relaxed)
}

/// Per-system cumulative sample recorded during a single frame.
pub struct MassSystemSample {
    pub name: &'static str,
    pub nanos: u128,
}

static MASS_TIMING_SAMPLES: std::sync::Mutex<Vec<MassSystemSample>> =
    std::sync::Mutex::new(Vec::new());

/// Record a timing sample for a single system invocation.
/// No-op unless `is_mass_timing_enabled()`.
pub fn record_mass_system_time(name: &'static str, nanos: u128) {
    if let Ok(mut samples) = MASS_TIMING_SAMPLES.lock() {
        samples.push(MassSystemSample { name, nanos });
    }
}

/// Drain all samples collected this frame. Called by `mass_frame_dispatch`.
pub fn drain_mass_system_samples() -> Vec<MassSystemSample> {
    match MASS_TIMING_SAMPLES.lock() {
        Ok(mut g) => std::mem::take(&mut *g),
        Err(_) => Vec::new(),
    }
}

/// Called once at the start of every `mass_frame_dispatch`, before any system
/// runs. Discards any timing samples left over from a prior frame that failed
/// to drain — e.g. when `catch_unwind` caught a panic from `sched.run()` so
/// the drain at the end of the closure never executed. Without this hook,
/// stale samples survive into the next successful frame's `[mass-perf]` line,
/// producing phantom duplicate entries and inflated totals.
///
/// Always drops (no enabled-flag check) — one mutex lock per frame, and the
/// Vec is empty when timing is disabled so the clear is a no-op. Keeping it
/// unconditional means the invariant holds even if timing gets toggled at
/// runtime mid-frame.
pub fn prepare_mass_frame() {
    if let Ok(mut samples) = MASS_TIMING_SAMPLES.lock() {
        samples.clear();
    }
}

/// Game-specific FFI function pointers discovered via inventory.
/// Framework folds over these to fill `Option` fields in `RustBindings`.
///
/// Add new game FFI functions as additional `Option<Fn>` fields here.
/// If this grows past ~3 fields, consider generalizing to an inventory-keyed dispatch map.
pub struct MassExternBinding {
    pub get_food_drop_events: Option<unreal_ffi::GetFoodDropEventsFn>,
    pub get_food_pickup_events: Option<unreal_ffi::GetFoodPickupEventsFn>,
    pub get_decision_counters: Option<unreal_ffi::GetDecisionCountersFn>,
    pub reset_decision_counters: Option<unreal_ffi::ResetDecisionCountersFn>,
}

inventory::collect!(MassExternBinding);

pub fn registered_extern_bindings() -> inventory::iter<MassExternBinding> {
    inventory::iter::<MassExternBinding>
}

// ---------------------------------------------------------------------------
// Spatial query config registration
// ---------------------------------------------------------------------------

/// Spatial query implementation type.
#[derive(Clone, Copy, Default, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MassSpatialQueryType {
    /// ISMC overlap queries (GetInstancesOverlappingSphere/Box).
    #[default]
    IsmcOverlap = 0,
    /// UE physics sweep (World->SweepMultiByChannel).
    PhysicsSweep = 1,
    /// UMassNavigationSubsystem hash grid (FNavigationObstacleHashGrid2D).
    GridHash = 2,
}

/// Game crates register spatial query configurations via inventory.
/// Each config tells the generic C++ subsystem how to perform spatial
/// queries for a particular entity group.
pub struct MassSpatialQueryConfigRegistration {
    /// Unique name for this spatial query (e.g. "food_pickup").
    pub query_name: &'static str,
    /// ISMC group name to search (e.g. "food").
    pub query_group: &'static str,
    /// Overlap sphere radius in Unreal units.
    pub radius: f32,
    /// Query implementation type (ISMC overlap or physics sweep).
    pub query_type: MassSpatialQueryType,
    /// Collision channel index for physics sweep (0 = ECC_GameTraceChannel1, etc.).
    /// Ignored for ISMC overlap queries.
    pub collision_channel_index: u8,
    /// C++ fragment type name that has the bool to filter on.
    pub filter_fragment_type: &'static str,
    /// Byte offset of the bool field within the filter fragment.
    pub filter_bool_offset: usize,
    /// Required value of the bool field.
    pub filter_bool_must_be: bool,
}

inventory::collect!(MassSpatialQueryConfigRegistration);

pub fn registered_spatial_query_configs() -> inventory::iter<MassSpatialQueryConfigRegistration> {
    inventory::iter::<MassSpatialQueryConfigRegistration>
}

// ---------------------------------------------------------------------------
// Simulation defaults registration
// ---------------------------------------------------------------------------

/// Game crates register default simulation parameters via inventory.
/// These provide defaults that the generic C++ activator actor can override
/// via UPROPERTY values in the editor.
pub struct MassSimDefaultsRegistration {
    pub name: &'static str,
    pub groups: &'static [(&'static str, i32)],
    pub bounds_min: [f64; 3],
    pub bounds_max: [f64; 3],
    pub random_seed: i32,
}

inventory::collect!(MassSimDefaultsRegistration);

pub fn registered_sim_defaults() -> inventory::iter<MassSimDefaultsRegistration> {
    inventory::iter::<MassSimDefaultsRegistration>
}

// ---------------------------------------------------------------------------
// Rust-authored UE integration tests
// ---------------------------------------------------------------------------

/// Game crates register UE integration tests via inventory.
/// Each test runs inside the UE editor with full Mass Entity + physics support.
/// Tests use standard Rust `assert!` macros — panics are caught and reported
/// as UE automation test failures.
pub struct MassTestRegistration {
    pub name: &'static str,
    pub test_fn: fn(&TestCtx),
}

inventory::collect!(MassTestRegistration);

pub fn registered_mass_tests() -> inventory::iter<MassTestRegistration> {
    inventory::iter::<MassTestRegistration>
}


/// Safe wrapper around C++ test callbacks.
/// Provides Rust-idiomatic access to UE subsystem operations during tests.
pub struct TestCtx {
    callbacks: *const unreal_ffi::MassTestCallbacks,
}

impl TestCtx {
    /// Create a TestCtx from raw FFI callbacks. Called by the test runner.
    ///
    /// # Safety
    /// `callbacks` must point to a valid `MassTestCallbacks` for the duration of the test.
    pub unsafe fn from_raw(callbacks: *const unreal_ffi::MassTestCallbacks) -> Self {
        Self { callbacks }
    }

    fn cb(&self) -> &unreal_ffi::MassTestCallbacks {
        unsafe { &*self.callbacks }
    }

    /// Initialize a simulation with named entity groups.
    pub fn init_sim(
        &self,
        groups: &[(&str, i32)],
        bounds_min: [f64; 3],
        bounds_max: [f64; 3],
        seed: i32,
    ) {
        let ffi_groups: Vec<unreal_ffi::MassEntityGroupDesc> = groups
            .iter()
            .map(|(name, count)| unreal_ffi::MassEntityGroupDesc {
                name: unreal_ffi::Utf8Str::from(*name),
                count: *count,
                _pad: 0,
            })
            .collect();

        let params = unreal_ffi::MassInitSimulationParams {
            groups: ffi_groups.as_ptr(),
            num_groups: ffi_groups.len() as u32,
            _pad0: 0,
            bounds_min,
            bounds_max,
            random_seed: seed,
            _pad1: 0,
        };

        let cb = self.cb();
        unsafe { (cb.init_sim)(cb.opaque, &params) };
    }

    /// Run simulation for `count` steps with the given delta time.
    pub fn step(&self, dt: f32, count: u32) {
        let cb = self.cb();
        unsafe { (cb.step_sim)(cb.opaque, dt, count) };
    }

    /// Reset simulation state (destroy all entities).
    pub fn reset(&self) {
        let cb = self.cb();
        unsafe { (cb.reset_sim)(cb.opaque) };
    }

    /// Trigger a subsystem tick (e.g. to fire auto-init).
    pub fn tick(&self, dt: f32) {
        let cb = self.cb();
        unsafe { (cb.tick)(cb.opaque, dt) };
    }

    /// Simulate a Rust hot-reload event.
    pub fn on_rust_reloaded(&self) {
        let cb = self.cb();
        unsafe { (cb.on_rust_reloaded)(cb.opaque) };
    }

    /// Get entity count for a named group.
    pub fn entity_count(&self, group: &str) -> i32 {
        let cb = self.cb();
        unsafe { (cb.entity_count)(cb.opaque, unreal_ffi::Utf8Str::from(group)) }
    }

    /// Check if a simulation is currently active.
    pub fn has_managed_sim(&self) -> bool {
        let cb = self.cb();
        unsafe { (cb.has_managed_sim)(cb.opaque) != 0 }
    }

    /// Check if a named spatial query is registered.
    pub fn has_spatial_query(&self, name: &str) -> bool {
        let cb = self.cb();
        unsafe { (cb.has_spatial_query)(cb.opaque, unreal_ffi::Utf8Str::from(name)) != 0 }
    }

    /// Read a fragment value for an entity in a group.
    /// Returns `None` if the entity or fragment doesn't exist.
    pub fn read<T: MassFragment>(&self, group: &str, index: u32) -> Option<T> {
        let mut value = std::mem::MaybeUninit::<T>::uninit();
        let cb = self.cb();
        let ok = unsafe {
            (cb.read_fragment)(
                cb.opaque,
                unreal_ffi::Utf8Str::from(group),
                index,
                unreal_ffi::Utf8Str::from(T::CPP_TYPE_NAME),
                value.as_mut_ptr() as *mut u8,
                std::mem::size_of::<T>() as u32,
            )
        };
        if ok == 1 {
            Some(unsafe { value.assume_init() })
        } else {
            None
        }
    }

    /// Write a fragment value for an entity in a group.
    /// Panics if the entity or fragment doesn't exist.
    pub fn write<T: MassFragment>(&self, group: &str, index: u32, value: &T) {
        let cb = self.cb();
        let ok = unsafe {
            (cb.write_fragment)(
                cb.opaque,
                unreal_ffi::Utf8Str::from(group),
                index,
                unreal_ffi::Utf8Str::from(T::CPP_TYPE_NAME),
                value as *const T as *const u8,
                std::mem::size_of::<T>() as u32,
            )
        };
        assert_eq!(ok, 1, "Failed to write fragment {} for {}/{}", T::CPP_TYPE_NAME, group, index);
    }

    // ---- Shadow-world (pure-Bevy component) accessors ---------------------
    //
    // These route through `register_shadow_world_accessors`, which
    // `unreal-module` sets at init. The shadow world is the Bevy `World`
    // inside `MASS_SCHEDULE` — same world the mass systems run against, so
    // insertions here are visible to the next dispatch.

    /// Resolve a shadow Bevy `Entity` for an entity in a group. Returns
    /// `Entity::PLACEHOLDER` if the group or index is unknown.
    pub fn bevy_entity(&self, group: &str, index: u32) -> bevy_ecs::entity::Entity {
        let Some(read) = shadow_world_read_fn() else {
            return bevy_ecs::entity::Entity::PLACEHOLDER;
        };
        let mut out = bevy_ecs::entity::Entity::PLACEHOLDER;
        let mut visit = |_world: &bevy_ecs::world::World, map: &MassEntityMap| {
            out = map
                .get(group, index as usize)
                .unwrap_or(bevy_ecs::entity::Entity::PLACEHOLDER);
        };
        read(&mut visit);
        out
    }

    /// Read a pure-Bevy component from the shadow entity at `(group, index)`.
    /// Returns `None` if the entity or component is missing.
    pub fn bevy_get<T: bevy_ecs::component::Component + Clone>(
        &self,
        group: &str,
        index: u32,
    ) -> Option<T> {
        let read = shadow_world_read_fn()?;
        let mut out: Option<T> = None;
        let mut visit = |world: &bevy_ecs::world::World, map: &MassEntityMap| {
            if let Some(entity) = map.get(group, index as usize) {
                out = world.get::<T>(entity).cloned();
            }
        };
        read(&mut visit);
        out
    }

    /// Insert or overwrite a pure-Bevy component on the shadow entity at
    /// `(group, index)`. Panics if the shadow world is unavailable or the
    /// entity index is out of range — a missing entity in a test is a bug,
    /// not a silent pass.
    pub fn bevy_insert<T: bevy_ecs::component::Component>(
        &self,
        group: &str,
        index: u32,
        value: T,
    ) {
        let write = shadow_world_write_fn()
            .expect("bevy_insert: shadow-world accessor not registered (unreal-module must init)");
        let mut value_opt: Option<T> = Some(value);
        let mut visit = |world: &mut bevy_ecs::world::World, map: &MassEntityMap| {
            let Some(entity) = map.get(group, index as usize) else {
                return;
            };
            if let Some(v) = value_opt.take() {
                world.entity_mut(entity).insert(v);
            }
        };
        let ok = write(&mut visit);
        assert!(ok, "bevy_insert: shadow world unavailable");
        assert!(
            value_opt.is_none(),
            "bevy_insert: shadow entity {}/{} not found",
            group, index
        );
    }

    /// Remove a pure-Bevy component from the shadow entity at `(group, index)`.
    /// No-op if the entity or component is missing.
    pub fn bevy_remove<T: bevy_ecs::component::Component>(&self, group: &str, index: u32) {
        let Some(write) = shadow_world_write_fn() else {
            return;
        };
        let mut visit = |world: &mut bevy_ecs::world::World, map: &MassEntityMap| {
            if let Some(entity) = map.get(group, index as usize) {
                world.entity_mut(entity).remove::<T>();
            }
        };
        let _ = write(&mut visit);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Clone, Copy)]
    #[repr(C)]
    struct TestFragment {
        x: f64,
        y: f64,
    }

    impl MassFragment for TestFragment {
        const CPP_TYPE_NAME: &'static str = "FTestFragment";
    }

    #[test]
    fn effective_order_prefers_explicit_order() {
        let mut map = std::collections::HashMap::new();
        map.insert("foo", 99);
        // Explicit `order = 42` on attr — not overridden by the schedule list.
        assert_eq!(effective_order("foo", 42, &map), 42);
    }

    #[test]
    fn effective_order_uses_override_when_sentinel() {
        let mut map = std::collections::HashMap::new();
        map.insert("foo", 20);
        // Declared as `u32::MAX` (no explicit order) — resolved via the map.
        assert_eq!(effective_order("foo", u32::MAX, &map), 20);
    }

    #[test]
    fn effective_order_unknown_name_stays_sentinel() {
        let map = std::collections::HashMap::new();
        // Neither explicit nor in the schedule: stays at the end.
        assert_eq!(effective_order("bar", u32::MAX, &map), u32::MAX);
    }

    #[test]
    fn mass_query_ref_from_slice() {
        let data = [
            TestFragment { x: 1.0, y: 2.0 },
            TestFragment { x: 3.0, y: 4.0 },
        ];
        let query = unsafe {
            MassQueryRef::<TestFragment>::from_raw(
                data.as_ptr() as *const c_void,
                data.len(),
            )
        };
        assert_eq!(query.len(), 2);
        assert!(!query.is_empty());
        let xs: Vec<f64> = query.iter().map(|f| f.x).collect();
        assert_eq!(xs, vec![1.0, 3.0]);
    }

    #[test]
    fn mass_query_mut_from_slice() {
        let mut data = [
            TestFragment { x: 1.0, y: 2.0 },
            TestFragment { x: 3.0, y: 4.0 },
        ];
        let mut query = unsafe {
            MassQueryMut::<TestFragment>::from_raw(
                data.as_mut_ptr() as *mut c_void,
                data.len(),
            )
        };
        for frag in query.iter_mut() {
            frag.x += 10.0;
        }
        assert_eq!(data[0].x, 11.0);
        assert_eq!(data[1].x, 13.0);
    }

    #[test]
    fn mass_query_all_ref_chunked() {
        let data = [
            TestFragment { x: 5.0, y: 6.0 },
            TestFragment { x: 7.0, y: 8.0 },
        ];
        let mut storage = MassGlobalChunkStorage::new();
        let query = unsafe {
            MassQueryAllRef::<TestFragment>::from_raw_single_chunk(
                data.as_ptr() as *const c_void,
                data.len(),
                &mut storage,
            )
        };
        assert_eq!(query.len(), 2);
        assert!(!query.is_empty());
        assert_eq!(query.get(0).unwrap().x, 5.0);
        assert_eq!(query.get(1).unwrap().x, 7.0);
        assert!(query.get(2).is_none());
        assert_eq!(query.iter().map(|f| f.x).collect::<Vec<_>>(), vec![5.0, 7.0]);
    }

    #[test]
    fn mass_query_all_mut_chunked() {
        let mut data = [
            TestFragment { x: 1.0, y: 2.0 },
            TestFragment { x: 3.0, y: 4.0 },
        ];
        let mut storage = MassGlobalChunkStorage::new();
        let mut query = unsafe {
            MassQueryAllMut::<TestFragment>::from_raw_single_chunk(
                data.as_mut_ptr() as *mut c_void,
                data.len(),
                &mut storage,
            )
        };
        assert_eq!(query.len(), 2);
        query.get_mut(0).unwrap().x = 99.0;
        assert_eq!(data[0].x, 99.0);
        // iter_mut
        for frag in query.iter_mut() {
            frag.y += 10.0;
        }
        assert_eq!(data[0].y, 12.0);
        assert_eq!(data[1].y, 14.0);
    }

    #[test]
    fn mass_query_all_multi_chunk() {
        let mut data1 = [TestFragment { x: 1.0, y: 2.0 }];
        let mut data2 = [
            TestFragment { x: 3.0, y: 4.0 },
            TestFragment { x: 5.0, y: 6.0 },
        ];
        let chunks = [
            unreal_ffi::MassGlobalChunkSlice {
                data: data1.as_mut_ptr() as *mut c_void,
                count: 1,
                stride: std::mem::size_of::<TestFragment>() as u32,
            },
            unreal_ffi::MassGlobalChunkSlice {
                data: data2.as_mut_ptr() as *mut c_void,
                count: 2,
                stride: std::mem::size_of::<TestFragment>() as u32,
            },
        ];
        let desc = unreal_ffi::MassGlobalFragmentChunks {
            chunks: chunks.as_ptr(),
            num_chunks: 2,
            total_count: 3,
            stride: std::mem::size_of::<TestFragment>() as u32,
        };
        let mut query = unsafe { MassQueryAllMut::<TestFragment>::from_chunked(&desc) };
        assert_eq!(query.len(), 3);
        // get across chunk boundary
        assert_eq!(query.get(0).unwrap().x, 1.0);
        assert_eq!(query.get(1).unwrap().x, 3.0);
        assert_eq!(query.get(2).unwrap().x, 5.0);
        assert!(query.get(3).is_none());
        // mutate in second chunk
        query.get_mut(2).unwrap().x = 99.0;
        assert_eq!(data2[1].x, 99.0);
        // iter collects all
        let xs: Vec<f64> = query.iter().map(|f| f.x).collect();
        assert_eq!(xs, vec![1.0, 3.0, 99.0]);
    }

    #[test]
    fn mass_fragment_trait() {
        assert_eq!(TestFragment::CPP_TYPE_NAME, "FTestFragment");
        assert_eq!(std::mem::size_of::<TestFragment>(), 16);
    }

    #[test]
    fn mass_chunks_empty() {
        let chunks = MassChunks::<TestFragment>::new();
        assert_eq!(chunks.primary_chunk_count(), 0);
        assert!(chunks.global().is_none());
    }

    #[test]
    fn mass_chunks_push_primary() {
        let mut data = [TestFragment { x: 1.0, y: 2.0 }, TestFragment { x: 3.0, y: 4.0 }];
        let slice = unreal_ffi::MassFragmentSlice {
            data: data.as_mut_ptr() as *mut c_void,
            count: 2,
            stride: std::mem::size_of::<TestFragment>() as u32,
        };
        let mut chunks = MassChunks::<TestFragment>::new();
        unsafe { chunks.push_primary_slice(slice); }
        assert_eq!(chunks.primary_chunk_count(), 1);

        let query = unsafe { chunks.primary_chunk_mut(0) };
        assert_eq!(query.len(), 2);
        assert_eq!(query.iter().next().unwrap().x, 1.0);
    }

    #[test]
    fn mass_chunks_clear() {
        let mut data = [TestFragment { x: 1.0, y: 2.0 }];
        let slice = unreal_ffi::MassFragmentSlice {
            data: data.as_mut_ptr() as *mut c_void,
            count: 1,
            stride: std::mem::size_of::<TestFragment>() as u32,
        };
        let mut chunks = MassChunks::<TestFragment>::new();
        unsafe { chunks.push_primary_slice(slice); }
        chunks.clear();
        assert_eq!(chunks.primary_chunk_count(), 0);
    }

    #[test]
    fn mass_chunks_in_world() {
        let mut world = bevy_ecs::world::World::new();
        world.insert_resource(MassChunks::<TestFragment>::new());
        let chunks = world.resource::<MassChunks<TestFragment>>();
        assert_eq!(chunks.primary_chunk_count(), 0);
    }

    #[test]
    fn mass_chunks_global() {
        let mut data = [TestFragment { x: 10.0, y: 20.0 }, TestFragment { x: 30.0, y: 40.0 }];
        let chunk_slice = unreal_ffi::MassGlobalChunkSlice {
            data: data.as_mut_ptr() as *mut c_void,
            count: 2,
            stride: std::mem::size_of::<TestFragment>() as u32,
        };
        let desc = unreal_ffi::MassGlobalFragmentChunks {
            chunks: &chunk_slice as *const _,
            num_chunks: 1,
            total_count: 2,
            stride: std::mem::size_of::<TestFragment>() as u32,
        };

        let mut chunks = MassChunks::<TestFragment>::new();
        unsafe { chunks.set_global(&desc); }
        assert!(chunks.global().is_some());

        let query = unsafe { chunks.global_query_mut().unwrap() };
        assert_eq!(query.len(), 2);
        assert_eq!(query.get(0).unwrap().x, 10.0);
    }

    #[test]
    fn mass_schedule_new_has_time() {
        let sched = MassSchedule::new();
        // TimePlugin registers the Time resource
        assert!(sched.world().contains_resource::<bevy_time::Time>());
    }

    #[test]
    fn mass_schedule_set_dt() {
        let mut sched = MassSchedule::new();
        sched.set_dt(0.016);
        // First run initializes the time origin (delta = 0, same as Bevy first frame)
        sched.run();
        let time = sched.world().resource::<bevy_time::Time>();
        assert_eq!(time.delta_secs(), 0.0, "first frame should have zero delta");

        // Second run produces the actual delta
        sched.set_dt(0.016);
        sched.run();
        let time = sched.world().resource::<bevy_time::Time>();
        assert!((time.delta_secs() - 0.016).abs() < 1e-6,
            "second frame delta_secs: {}", time.delta_secs());
    }

    #[test]
    fn mass_schedule_runs_system() {
        #[derive(bevy_ecs::prelude::Resource, Default)]
        struct Counter(u32);

        fn increment(mut counter: bevy_ecs::prelude::ResMut<Counter>) {
            counter.0 += 1;
        }

        let mut sched = MassSchedule::new();
        sched.world_mut().insert_resource(Counter(0));
        sched.app_mut().add_systems(bevy_app::Update, increment);
        sched.run();
        assert_eq!(sched.world().resource::<Counter>().0, 1);
        sched.run();
        assert_eq!(sched.world().resource::<Counter>().0, 2);
    }

    #[test]
    fn time_update_strategy_in_schedule() {
        let mut sched = MassSchedule::new();
        // First frame initializes time origin
        sched.set_dt(0.033);
        sched.run();

        // Second frame produces the actual delta
        sched.set_dt(0.033);
        sched.run();
        let time = sched.world().resource::<bevy_time::Time>();
        assert!((time.delta_secs() - 0.033).abs() < 1e-6,
            "delta_secs: {}", time.delta_secs());
    }

    #[test]
    fn mass_system_stage_ordering() {
        use bevy_ecs::prelude::*;
        use std::sync::{Arc, Mutex};

        #[derive(Resource)]
        struct Log(Arc<Mutex<Vec<&'static str>>>);

        fn system_a(log: Res<Log>) {
            log.0.lock().unwrap().push("a");
        }
        fn system_b(log: Res<Log>) {
            log.0.lock().unwrap().push("b");
        }

        let log = Arc::new(Mutex::new(Vec::new()));
        let mut world = bevy_ecs::world::World::new();
        world.insert_resource(Log(log.clone()));

        let mut schedule = bevy_ecs::schedule::Schedule::default();
        schedule.configure_sets(MassSystemStage(1).after(MassSystemStage(0)));
        schedule.add_systems(system_a.in_set(MassSystemStage(0)));
        schedule.add_systems(system_b.in_set(MassSystemStage(1)));
        schedule.run(&mut world);

        let order = log.lock().unwrap();
        assert_eq!(&*order, &["a", "b"]);
    }

    #[test]
    fn backward_compat_mass_query_alias() {
        // MassQuery is now an alias for MassQueryMut
        let mut data = [TestFragment { x: 1.0, y: 2.0 }];
        let mut query = unsafe {
            MassQuery::<TestFragment>::from_raw(
                data.as_mut_ptr() as *mut c_void,
                data.len(),
            )
        };
        query.iter_mut().for_each(|f| f.x += 1.0);
        assert_eq!(data[0].x, 2.0);
    }

    // -----------------------------------------------------------------------
    // C++ codegen tests
    // -----------------------------------------------------------------------

    #[test]
    fn snake_to_pascal_basic() {
        assert_eq!(snake_to_pascal("carried_food_index"), "CarriedFoodIndex");
        assert_eq!(snake_to_pascal("position"), "Position");
        assert_eq!(snake_to_pascal("is_loose"), "IsLoose");
    }

    #[test]
    fn rust_type_to_cpp_mapping() {
        assert_eq!(rust_type_to_cpp("[f64 ; 3]", "pos").0, "FVector");
        assert_eq!(rust_type_to_cpp("f32", "speed").0, "float");
        assert_eq!(rust_type_to_cpp("i32", "index").0, "int32");
        assert_eq!(rust_type_to_cpp("bool", "flag").0, "bool");
        assert!(rust_type_to_cpp("[u8 ; 7]", "_pad").0.contains("7"));
    }

    #[test]
    fn generate_cpp_simple_fragment() {
        // No _pad field — codegen should auto-emit trailing padding
        static FIELDS: [MassFragmentFieldInfo; 2] = [
            MassFragmentFieldInfo { name: "position", type_name: "[f64 ; 3]", offset: 0, size: 24, default_value: "" },
            MassFragmentFieldInfo { name: "is_loose", type_name: "bool", offset: 24, size: 1, default_value: "true" },
        ];
        let reg = MassFragmentRegistration {
            cpp_type_name: "FTestFood",
            rust_type_name: "FoodFragment",
            size: 32,
            align: 8,
            fields: &FIELDS,
            is_tag: false,
            existing: false,
            include_header: "",
            write_default: None,
        };

        let output = generate_cpp_fragments(&[&reg], "Test.h");
        assert!(output.contains("struct FTestFood : public FMassFragment"), "should have USTRUCT def");
        assert!(output.contains("FVector Position"), "should map [f64;3] to FVector");
        assert!(output.contains("bool bIsLoose = true"), "should map bool field with UE b prefix and custom default");
        assert!(output.contains("offsetof(FTestFood, Position) == 0"), "should have offset assert");
        assert!(output.contains("sizeof(FTestFood) == 32"), "should have size assert");
        // Auto-emitted trailing padding (7 bytes after bool at offset 25)
        assert!(output.contains("_Pad_25[7]"), "should auto-emit trailing padding: {}", output);
    }

    #[test]
    fn generate_cpp_auto_padding_interior() {
        // i32 at offset 0 (size 4), then DVec3 at offset 8 (size 24) — gap of 4 bytes
        static FIELDS: [MassFragmentFieldInfo; 2] = [
            MassFragmentFieldInfo { name: "index", type_name: "i32", offset: 0, size: 4, default_value: "" },
            MassFragmentFieldInfo { name: "position", type_name: "[f64 ; 3]", offset: 8, size: 24, default_value: "" },
        ];
        let reg = MassFragmentRegistration {
            cpp_type_name: "FTestInterior",
            rust_type_name: "InteriorPad",
            size: 32,
            align: 8,
            fields: &FIELDS,
            is_tag: false,
            existing: false,
            include_header: "",
            write_default: None,
        };

        let output = generate_cpp_fragments(&[&reg], "Test.h");
        // Interior padding between i32 and DVec3
        assert!(output.contains("_Pad_4[4]"), "should auto-emit interior padding: {}", output);
        assert!(output.contains("sizeof(FTestInterior) == 32"), "should have size assert");
    }

    #[test]
    fn generate_cpp_skips_legacy_pad_fields() {
        // Legacy _pad field should be skipped (not emitted as a named field)
        static FIELDS: [MassFragmentFieldInfo; 2] = [
            MassFragmentFieldInfo { name: "value", type_name: "i32", offset: 0, size: 4, default_value: "" },
            MassFragmentFieldInfo { name: "_pad", type_name: "[u8 ; 4]", offset: 4, size: 4, default_value: "" },
        ];
        let reg = MassFragmentRegistration {
            cpp_type_name: "FTestPad",
            rust_type_name: "PadFragment",
            size: 8,
            align: 4,
            fields: &FIELDS,
            is_tag: false,
            existing: false,
            include_header: "",
            write_default: None,
        };

        let output = generate_cpp_fragments(&[&reg], "Test.h");
        // The _pad field is skipped, but auto-padding fills the gap to size 8
        assert!(output.contains("_Pad_4[4]"), "should auto-emit trailing padding: {}", output);
        assert!(!output.contains("offsetof(FTestPad, _Pad)"), "should not emit offset assert for skipped _pad");
    }

    #[test]
    fn generate_cpp_tag() {
        let reg = MassFragmentRegistration {
            cpp_type_name: "FMyTag",
            rust_type_name: "MyTag",
            size: 0,
            align: 1,
            fields: &[],
            is_tag: true,
            existing: false,
            include_header: "",
            write_default: None,
        };

        let output = generate_cpp_fragments(&[&reg], "Test.h");
        assert!(output.contains("struct FMyTag : public FMassTag"), "should use FMassTag base");
        assert!(output.contains("GENERATED_BODY()"), "should have GENERATED_BODY");
        assert!(!output.contains("offsetof"), "tags should have no offset asserts");
        assert!(!output.contains("sizeof"), "tags should have no size asserts");
    }

    #[test]
    fn generate_cpp_existing_type() {
        let reg = MassFragmentRegistration {
            cpp_type_name: "FTransformFragment",
            rust_type_name: "Transform",
            size: 96,
            align: 16,
            fields: &[],
            is_tag: false,
            existing: true,
            include_header: "MassCommonFragments.h",
            write_default: None,
        };

        let output = generate_cpp_fragments(&[&reg], "Test.h");
        assert!(!output.contains("USTRUCT"), "existing type should not generate USTRUCT");
        assert!(!output.contains("GENERATED_BODY"), "existing type should not have GENERATED_BODY");
        assert!(output.contains("sizeof(FTransformFragment) == 96"), "should have sizeof assert");
        assert!(output.contains("#include \"MassCommonFragments.h\""), "should include header for existing type");
        assert!(output.contains("existing UE type"), "should have existing comment");
    }

    // -----------------------------------------------------------------------
    // MassSpatialQueries unit tests
    // -----------------------------------------------------------------------

    #[test]
    fn spatial_queries_call_returns_none_for_unregistered() {
        let queries = MassSpatialQueries::default();
        let prev = [0.0; 3];
        let cur = [1.0, 0.0, 0.0];
        assert!(queries.call("nonexistent", &prev, &cur).is_none());
    }

    #[test]
    fn spatial_queries_call_returns_result_for_registered() {
        unsafe extern "C" fn mock_query(
            _prev: *const f64,
            _curr: *const f64,
            _radius: f32,
            out: *mut unreal_ffi::MassSpatialQueryResult,
        ) -> u32 {
            unsafe {
                (*out).has_encounter = true;
                (*out).entity_index = 7;
                (*out).encounter_position = [10.0, 20.0, 30.0];
            }
            1
        }

        let mut queries = MassSpatialQueries::default();
        queries.insert("test_query".to_string(), mock_query, 25.0);

        let prev = [0.0; 3];
        let cur = [5.0, 0.0, 0.0];
        let result = queries.call("test_query", &prev, &cur);
        assert!(result.is_some());
        let r = result.unwrap();
        assert!(r.has_encounter);
        assert_eq!(r.entity_index, 7);
        assert_eq!(r.encounter_position, [10.0, 20.0, 30.0]);
    }

    #[test]
    fn spatial_queries_clear_removes_all() {
        unsafe extern "C" fn mock_query(
            _prev: *const f64, _curr: *const f64, _radius: f32,
            out: *mut unreal_ffi::MassSpatialQueryResult,
        ) -> u32 {
            unsafe { (*out).has_encounter = false; }
            1
        }

        let mut queries = MassSpatialQueries::default();
        queries.insert("q1".to_string(), mock_query, 10.0);
        assert!(queries.call("q1", &[0.0; 3], &[0.0; 3]).is_some());

        queries.clear();
        assert!(queries.call("q1", &[0.0; 3], &[0.0; 3]).is_none());
    }

    // --- min_specialization: ChunkBacked / QueryBackend ---

    #[repr(C)]
    #[derive(Clone, Copy)]
    struct ChunkType { value: f32 }
    impl ChunkBacked for ChunkType {}

    #[derive(Clone, Copy)]
    struct BevyOnlyType { value: f32 }

    #[test]
    fn chunk_backed_type_detected() {
        assert!(<ChunkType as QueryBackend>::IS_CHUNK);
    }

    #[test]
    fn bevy_only_type_not_chunk() {
        assert!(!<BevyOnlyType as QueryBackend>::IS_CHUNK);
    }

    #[test]
    fn const_if_eliminates_branch() {
        // Verify the pattern we'll use in generated code: const-if dispatch
        fn dispatch<T: QueryBackend>() -> &'static str {
            if <T as QueryBackend>::IS_CHUNK {
                "chunk"
            } else {
                "bevy"
            }
        }
        assert_eq!(dispatch::<ChunkType>(), "chunk");
        assert_eq!(dispatch::<BevyOnlyType>(), "bevy");
    }

    #[test]
    fn dual_query_mut_chunk_iteration() {
        let mut data = [1.0f32, 2.0, 3.0];
        let mut q = DualQueryMut::Chunk(&mut data);
        let vals: Vec<f32> = (&mut q).into_iter().map(|v| *v).collect();
        assert_eq!(vals, vec![1.0, 2.0, 3.0]);
    }

    #[test]
    fn dual_query_mut_chunk_mutation() {
        let mut data = [1.0f32, 2.0, 3.0];
        {
            let mut q = DualQueryMut::Chunk(&mut data);
            for v in &mut q {
                *v += 10.0;
            }
        }
        assert_eq!(data, [11.0, 12.0, 13.0]);
    }

    #[test]
    fn dual_query_mut_bevy_iteration() {
        let mut a = 10i32;
        let mut b = 20i32;
        let mut c = 30i32;
        let ptrs: Vec<*mut i32> = vec![&mut a as *mut _, &mut b as *mut _, &mut c as *mut _];
        let mut q = DualQueryMut::Bevy(ptrs, std::marker::PhantomData);
        let vals: Vec<i32> = (&mut q).into_iter().map(|v| *v).collect();
        assert_eq!(vals, vec![10, 20, 30]);
    }

    #[test]
    fn dual_query_mut_bevy_mutation() {
        let mut a = 1i32;
        let mut b = 2i32;
        let ptrs: Vec<*mut i32> = vec![&mut a as *mut _, &mut b as *mut _];
        {
            let mut q = DualQueryMut::Bevy(ptrs, std::marker::PhantomData);
            for v in &mut q {
                *v *= 100;
            }
        }
        assert_eq!(a, 100);
        assert_eq!(b, 200);
    }

    #[test]
    fn dual_query_ref_chunk_iteration() {
        let data = [4.0f32, 5.0, 6.0];
        let q = DualQueryRef::Chunk(&data);
        let vals: Vec<f32> = (&q).into_iter().copied().collect();
        assert_eq!(vals, vec![4.0, 5.0, 6.0]);
    }

    #[test]
    fn dual_query_empty_yields_nothing() {
        let mut q: DualQueryMut<'_, f32> = DualQueryMut::Empty;
        assert_eq!(q.len(), 0);
        assert!(q.is_empty());
        let vals: Vec<&mut f32> = (&mut q).into_iter().collect();
        assert!(vals.is_empty());
    }

    #[test]
    fn dual_query_mut_reborrow() {
        let mut data = [1i32, 2, 3];
        let mut q = DualQueryMut::Chunk(&mut data);
        {
            let mut rb = q.reborrow();
            for v in &mut rb {
                *v += 10;
            }
        }
        // Original data should be mutated via reborrow
        let vals: Vec<i32> = (&mut q).into_iter().map(|v| *v).collect();
        assert_eq!(vals, vec![11, 12, 13]);
    }

    // -----------------------------------------------------------------------
    // Finding 3: mass timing samples must not leak across panicked frames.
    //
    // `mass_frame_dispatch` records per-system timing samples during
    // `sched.run()`, then drains them at the end of the closure passed to
    // `catch_unwind`. If `sched.run()` panics, the closure unwinds out
    // *before* the drain runs, so any samples accumulated during the partial
    // frame stay in the global `MASS_TIMING_SAMPLES` Vec and are double-
    // reported on the next successful frame.
    //
    // `prepare_mass_frame()` is the fix point — called at the top of every
    // dispatch, before `catch_unwind`, it drains any residual samples so a
    // fresh frame starts from an empty table.
    // -----------------------------------------------------------------------

    // Serialize the timing-table tests — they all touch the shared global
    // MASS_TIMING_SAMPLES, and `cargo test` runs tests in parallel by default.
    // One shared mutex keeps them from stepping on each other's push/drain
    // sequences without requiring `--test-threads=1` on the command line.
    static TIMING_TEST_LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());

    #[test]
    fn timing_samples_do_not_leak_across_panic() {
        let _guard = TIMING_TEST_LOCK.lock().unwrap_or_else(|e| e.into_inner());

        // Start from a known-empty table — any residue from a prior test in
        // this process must not contaminate this one.
        let _ = drain_mass_system_samples();

        // --- Simulate a frame that panicked before its drain ran ---
        // In the real code path this is `record_mass_system_time(...)` calls
        // inside `sched.run()`, followed by a panic that unwinds out of the
        // `catch_unwind` closure without reaching the final drain.
        record_mass_system_time("system_from_panicked_frame", 1_000);
        // (no drain here — mirrors the unwind-skipped-drain case)

        // --- Start of a fresh frame ---
        // `mass_frame_dispatch` should call `prepare_mass_frame()` here,
        // before any timing data for the new frame is recorded.
        prepare_mass_frame();

        // --- New frame runs its systems ---
        record_mass_system_time("system_from_fresh_frame", 2_000);

        // --- End of the new frame: drain and inspect ---
        let samples = drain_mass_system_samples();
        let names: Vec<&'static str> = samples.iter().map(|s| s.name).collect();

        assert!(
            !names.contains(&"system_from_panicked_frame"),
            "stale sample from panicked frame leaked into next frame's drain: names={:?}",
            names
        );
        assert_eq!(
            names,
            vec!["system_from_fresh_frame"],
            "fresh-frame drain should contain only the samples recorded after prepare_mass_frame"
        );
    }
}
