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

// ---------------------------------------------------------------------------
// Bevy scheduling resources
// ---------------------------------------------------------------------------

/// Delta time resource, updated each frame before the schedule runs.
#[derive(bevy_ecs::prelude::Resource, Default, Clone, Copy, Debug)]
pub struct MassDeltaTime(pub f32);

/// Per-fragment-type chunk storage (implementation detail).
/// Use `MassSystemChunks<S, T>` as the Bevy resource — the marker type `S`
/// ensures each system gets its own isolated storage even when multiple
/// systems use the same fragment type.
pub struct MassChunks<T: MassFragment> {
    primary_slices: Vec<unreal_ffi::MassFragmentSlice>,
    global_desc: Option<*const unreal_ffi::MassGlobalFragmentChunks>,
    _phantom: PhantomData<T>,
}

impl<T: MassFragment> bevy_ecs::prelude::Resource for MassChunks<T> {}

// Safety: chunk pointers are valid for the frame duration (C++ guarantees),
// and Bevy's scheduler serializes conflicting access via Res/ResMut.
unsafe impl<T: MassFragment> Send for MassChunks<T> {}
unsafe impl<T: MassFragment> Sync for MassChunks<T> {}

/// Per-system, per-fragment chunk storage as a Bevy resource.
/// The marker type `S` (a generated zero-sized struct per `#[mass_system]`)
/// ensures each system gets isolated chunk data, preventing cross-system
/// contamination when multiple systems use the same fragment type.
pub struct MassSystemChunks<S: 'static, T: MassFragment> {
    inner: MassChunks<T>,
    _marker: PhantomData<S>,
}

impl<S: 'static, T: MassFragment> bevy_ecs::prelude::Resource for MassSystemChunks<S, T> {}
unsafe impl<S: 'static, T: MassFragment> Send for MassSystemChunks<S, T> {}
unsafe impl<S: 'static, T: MassFragment> Sync for MassSystemChunks<S, T> {}

impl<S: 'static, T: MassFragment> MassSystemChunks<S, T> {
    pub fn new() -> Self {
        Self {
            inner: MassChunks::new(),
            _marker: PhantomData,
        }
    }
}

impl<S: 'static, T: MassFragment> std::ops::Deref for MassSystemChunks<S, T> {
    type Target = MassChunks<T>;
    fn deref(&self) -> &MassChunks<T> {
        &self.inner
    }
}

impl<S: 'static, T: MassFragment> std::ops::DerefMut for MassSystemChunks<S, T> {
    fn deref_mut(&mut self) -> &mut MassChunks<T> {
        &mut self.inner
    }
}

impl<T: MassFragment> MassChunks<T> {
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
        MassQueryRef::from_raw(slice.data as *const c_void, slice.count as usize)
    }

    /// Get a mutable query for primary chunk at index.
    ///
    /// # Safety
    /// Caller must ensure no other alias to the same chunk data exists.
    pub unsafe fn primary_chunk_mut(&mut self, index: usize) -> MassQueryMut<'_, T> {
        let slice = &self.primary_slices[index];
        MassQueryMut::from_raw(slice.data, slice.count as usize)
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
            .map(|desc| MassQueryAllMut::from_chunked(desc))
    }

    /// Get a read-only global query across all chunks.
    ///
    /// # Safety
    /// Caller must ensure the global descriptor pointer is still valid.
    pub unsafe fn global_query_ref(&self) -> Option<MassQueryAllRef<'_, T>> {
        self.global_desc
            .map(|desc| MassQueryAllRef::from_chunked(desc))
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
/// The World contains per-fragment `MassChunks<T>` resources and `MassDeltaTime`.
/// The Schedule contains the Bevy wrapper systems generated by `#[mass_system]`.
pub struct MassSchedule {
    world: bevy_ecs::world::World,
    schedule: bevy_ecs::schedule::Schedule,
}

impl MassSchedule {
    pub fn new() -> Self {
        let mut world = bevy_ecs::world::World::new();
        world.insert_resource(MassDeltaTime::default());
        // Force single-threaded execution: MassChunks<T> holds raw pointers into
        // C++ Mass Entity chunk memory. Multi-threaded execution requires auditing
        // pointer safety and ensuring the spatial query callback is thread-safe.
        let mut schedule = bevy_ecs::schedule::Schedule::default();
        schedule.set_executor_kind(bevy_ecs::schedule::ExecutorKind::SingleThreaded);
        Self { world, schedule }
    }

    pub fn set_dt(&mut self, dt: f32) {
        self.world.resource_mut::<MassDeltaTime>().0 = dt;
    }

    pub fn run(&mut self) {
        self.schedule.run(&mut self.world);
    }

    pub fn world(&self) -> &bevy_ecs::world::World {
        &self.world
    }

    pub fn world_mut(&mut self) -> &mut bevy_ecs::world::World {
        &mut self.world
    }

    pub fn schedule_mut(&mut self) -> &mut bevy_ecs::schedule::Schedule {
        &mut self.schedule
    }
}

// ---------------------------------------------------------------------------
// Spatial query callback resource
// ---------------------------------------------------------------------------

/// Bevy resource holding the optional C++ spatial query callback.
/// Used by the Rust collision pre-pass system to detect food encounters.
pub struct MassSpatialQueryCallback {
    pub query_fn: Option<unreal_ffi::MassSpatialQueryFn>,
    pub pickup_radius: f32,
}

impl bevy_ecs::prelude::Resource for MassSpatialQueryCallback {}

// Safety: The callback function pointer is a plain `extern "C" fn` — no captured state.
unsafe impl Send for MassSpatialQueryCallback {}
unsafe impl Sync for MassSpatialQueryCallback {}

impl Default for MassSpatialQueryCallback {
    fn default() -> Self {
        Self {
            query_fn: None,
            pickup_radius: 15.0,
        }
    }
}

/// Trait implemented by `#[derive(MassFragment)]` on `#[repr(C)]` structs
/// that match a C++ MassEntity USTRUCT.
pub trait MassFragment: Sized + Copy + 'static {
    /// The C++ USTRUCT type name (e.g. "FGatherersMassAntFragment").
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
        "[f64; 3]" | "[f64 ; 3]" => "FVector".to_string(),
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

/// Generate a C++ header with USTRUCT definitions and static_assert offset checks
/// for the given fragment registrations.
pub fn generate_cpp_fragments(fragments: &[&MassFragmentRegistration]) -> String {
    let mut out = String::new();
    out.push_str("#pragma once\n\n");
    out.push_str("#include \"CoreMinimal.h\"\n");
    out.push_str("#include \"MassEntityTypes.h\"\n");
    out.push_str("#include \"GeneratedFragments.generated.h\"\n\n");
    out.push_str("// Auto-generated from #[derive(MassFragment)] Rust structs.\n");
    out.push_str("// Do not edit manually.\n\n");

    for frag in fragments {
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

        for field in frag.fields {
            let (cpp_type, _is_padding) = rust_type_to_cpp(field.type_name, field.name);
            let cpp_name = if field.name.starts_with('_') {
                // Padding field: _foo_pad → _FooPad
                format!("_{}", snake_to_pascal(&field.name[1..]))
            } else if cpp_type == "bool" {
                // UE convention: prefix bools with 'b'
                format!("b{}", snake_to_pascal(field.name))
            } else {
                snake_to_pascal(field.name)
            };

            // Handle array types specially (PLACEHOLDER replaced with field name)
            if cpp_type.contains("PLACEHOLDER") {
                let cpp_type = cpp_type.replace("PLACEHOLDER", &cpp_name);
                out.push_str(&format!("\t{} = {{}};\n", cpp_type));
            } else {
                let default = if !field.default_value.is_empty() {
                    format!(" = {}", field.default_value)
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
        }

        out.push_str("};\n\n");

        // static_assert offset checks
        for field in frag.fields {
            let (cpp_type, _) = rust_type_to_cpp(field.type_name, field.name);
            let cpp_name = if field.name.starts_with('_') {
                format!("_{}", snake_to_pascal(&field.name[1..]))
            } else if cpp_type == "bool" {
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

// ---------------------------------------------------------------------------
// Primary query types (per-chunk)
// ---------------------------------------------------------------------------

/// Read-only view over fragment data in a chunk.
/// Generated by the macro for `MassQuery<&T>`.
pub struct MassQueryRef<'a, T: MassFragment> {
    data: &'a [T],
}

impl<'a, T: MassFragment> MassQueryRef<'a, T> {
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
}

impl<'b, 'a, T: MassFragment> IntoIterator for &'b MassQueryRef<'a, T> {
    type Item = &'b T;
    type IntoIter = std::slice::Iter<'b, T>;
    fn into_iter(self) -> Self::IntoIter {
        self.data.iter()
    }
}

/// Read-write view over fragment data in a chunk.
/// Generated by the macro for `MassQuery<&mut T>`.
pub struct MassQueryMut<'a, T: MassFragment> {
    data: &'a mut [T],
}

impl<'a, T: MassFragment> MassQueryMut<'a, T> {
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
}

impl<'b, 'a, T: MassFragment> IntoIterator for &'b MassQueryMut<'a, T> {
    type Item = &'b T;
    type IntoIter = std::slice::Iter<'b, T>;
    fn into_iter(self) -> Self::IntoIter {
        self.data.iter()
    }
}

impl<'b, 'a, T: MassFragment> IntoIterator for &'b mut MassQueryMut<'a, T> {
    type Item = &'b mut T;
    type IntoIter = std::slice::IterMut<'b, T>;
    fn into_iter(self) -> Self::IntoIter {
        self.data.iter_mut()
    }
}

// ---------------------------------------------------------------------------
// Global query types (cross-archetype, all matching entities, zero-copy)
// ---------------------------------------------------------------------------

/// Read-only view over ALL entities matching a fragment type, across all chunks.
/// Zero-copy: points directly into Mass Entity chunk memory.
/// Generated by the macro for `MassQueryAll<&T>`.
pub struct MassQueryAllRef<'a, T: MassFragment> {
    desc: &'a unreal_ffi::MassGlobalFragmentChunks,
    _phantom: std::marker::PhantomData<&'a T>,
}

impl<'a, T: MassFragment> MassQueryAllRef<'a, T> {
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
pub struct MassQueryAllMut<'a, T: MassFragment> {
    desc: &'a unreal_ffi::MassGlobalFragmentChunks,
    _phantom: std::marker::PhantomData<&'a mut T>,
}

impl<'a, T: MassFragment> MassQueryAllMut<'a, T> {
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

fn chunked_get<'a, T: MassFragment>(
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

fn chunked_get_mut<'a, T: MassFragment>(
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
pub struct MassQueryAllIter<'a, T: MassFragment> {
    desc: &'a unreal_ffi::MassGlobalFragmentChunks,
    chunk_idx: usize,
    offset: usize,
    _phantom: std::marker::PhantomData<&'a T>,
}

impl<'a, T: MassFragment> MassQueryAllIter<'a, T> {
    fn new(desc: &'a unreal_ffi::MassGlobalFragmentChunks) -> Self {
        Self {
            desc,
            chunk_idx: 0,
            offset: 0,
            _phantom: std::marker::PhantomData,
        }
    }
}

impl<'a, T: MassFragment> Iterator for MassQueryAllIter<'a, T> {
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
pub struct MassQueryAllIterMut<'a, T: MassFragment> {
    desc: &'a unreal_ffi::MassGlobalFragmentChunks,
    chunk_idx: usize,
    offset: usize,
    _phantom: std::marker::PhantomData<&'a mut T>,
}

impl<'a, T: MassFragment> MassQueryAllIterMut<'a, T> {
    fn new(desc: &'a unreal_ffi::MassGlobalFragmentChunks) -> Self {
        Self {
            desc,
            chunk_idx: 0,
            offset: 0,
            _phantom: std::marker::PhantomData,
        }
    }
}

impl<'a, T: MassFragment> Iterator for MassQueryAllIterMut<'a, T> {
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
}

/// Returns all registered mass systems (extern "C" dispatch path).
pub fn registered_mass_systems() -> inventory::iter<MassSystemRegistration> {
    inventory::iter::<MassSystemRegistration>
}

/// Registration entry for a Bevy-scheduled mass system, collected via inventory.
pub struct MassBevySystemRegistration {
    pub name: &'static str,
    /// Adds this system to a Bevy Schedule in the given stage.
    pub add_to_schedule: fn(&mut bevy_ecs::schedule::Schedule, MassSystemStage),
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
        let mut chunk_slice = unreal_ffi::MassGlobalChunkSlice {
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

        let mut query = unsafe { chunks.global_query_mut().unwrap() };
        assert_eq!(query.len(), 2);
        assert_eq!(query.get(0).unwrap().x, 10.0);
    }

    #[test]
    fn mass_schedule_new_has_dt() {
        let sched = MassSchedule::new();
        assert_eq!(sched.world().resource::<MassDeltaTime>().0, 0.0);
    }

    #[test]
    fn mass_schedule_set_dt() {
        let mut sched = MassSchedule::new();
        sched.set_dt(0.016);
        assert_eq!(sched.world().resource::<MassDeltaTime>().0, 0.016);
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
        sched.schedule_mut().add_systems(increment);
        sched.run();
        assert_eq!(sched.world().resource::<Counter>().0, 1);
        sched.run();
        assert_eq!(sched.world().resource::<Counter>().0, 2);
    }

    #[test]
    fn mass_delta_time_default() {
        let dt = MassDeltaTime::default();
        assert_eq!(dt.0, 0.0);
    }

    #[test]
    fn mass_delta_time_in_world() {
        let mut world = bevy_ecs::world::World::new();
        world.insert_resource(MassDeltaTime(0.016));
        assert_eq!(world.resource::<MassDeltaTime>().0, 0.016);
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
        static FIELDS: [MassFragmentFieldInfo; 3] = [
            MassFragmentFieldInfo { name: "position", type_name: "[f64 ; 3]", offset: 0, size: 24, default_value: "" },
            MassFragmentFieldInfo { name: "is_loose", type_name: "bool", offset: 24, size: 1, default_value: "true" },
            MassFragmentFieldInfo { name: "_pad", type_name: "[u8 ; 7]", offset: 25, size: 7, default_value: "" },
        ];
        let reg = MassFragmentRegistration {
            cpp_type_name: "FTestFood",
            rust_type_name: "FoodFragment",
            size: 32,
            align: 8,
            fields: &FIELDS,
            is_tag: false,
        };

        let output = generate_cpp_fragments(&[&reg]);
        assert!(output.contains("struct FTestFood : public FMassFragment"), "should have USTRUCT def");
        assert!(output.contains("FVector Position"), "should map [f64;3] to FVector");
        assert!(output.contains("bool bIsLoose = true"), "should map bool field with UE b prefix and custom default");
        assert!(output.contains("offsetof(FTestFood, Position) == 0"), "should have offset assert");
        assert!(output.contains("sizeof(FTestFood) == 32"), "should have size assert");
    }

    #[test]
    fn generate_cpp_padding_fields() {
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
        };

        let output = generate_cpp_fragments(&[&reg]);
        assert!(output.contains("_Pad"), "padding fields should get _ prefix");
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
        };

        let output = generate_cpp_fragments(&[&reg]);
        assert!(output.contains("struct FMyTag : public FMassTag"), "should use FMassTag base");
        assert!(output.contains("GENERATED_BODY()"), "should have GENERATED_BODY");
        assert!(!output.contains("offsetof"), "tags should have no offset asserts");
        assert!(!output.contains("sizeof"), "tags should have no size asserts");
    }
}
