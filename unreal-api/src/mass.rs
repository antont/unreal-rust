//! Bevy-like API layer for MassEntity.
//!
//! Provides `MassFragment` trait, `MassQuery` type, and registration
//! infrastructure for dynamic mass system dispatch.

use std::ffi::c_void;

/// Trait implemented by `#[derive(MassFragment)]` on `#[repr(C)]` structs
/// that match a C++ MassEntity USTRUCT.
pub trait MassFragment: Sized + Copy + 'static {
    /// The C++ USTRUCT type name (e.g. "FGatherersMassAntFragment").
    const CPP_TYPE_NAME: &'static str;
}

/// Compile-time registration entry for a MassFragment, collected via inventory.
pub struct MassFragmentRegistration {
    pub cpp_type_name: &'static str,
    pub rust_type_name: &'static str,
    pub size: usize,
    pub align: usize,
}

inventory::collect!(MassFragmentRegistration);

/// Returns all registered MassFragment types.
pub fn registered_mass_fragments() -> inventory::iter<MassFragmentRegistration> {
    inventory::iter::<MassFragmentRegistration>
}

/// A typed view over a contiguous slice of MassEntity fragment data in a chunk.
/// Constructed by the `#[mass_system]` macro from raw `MassChunkData`.
pub struct MassQuery<'a, T: MassFragment> {
    data: &'a mut [T],
}

impl<'a, T: MassFragment> MassQuery<'a, T> {
    /// Create a MassQuery from a raw pointer and count.
    ///
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

/// Registration entry for a dynamically registered mass system, collected via inventory.
pub struct MassSystemRegistration {
    pub name: &'static str,
    pub execute_fn: unsafe extern "C" fn(*const unreal_ffi::MassChunkData),
    pub requirements: &'static [MassSystemRequirement],
}

inventory::collect!(MassSystemRegistration);

/// Describes one fragment requirement for a mass system.
pub struct MassSystemRequirement {
    pub cpp_type_name: &'static str,
    pub size: usize,
    pub align: usize,
    pub access_mode: u8, // 0 = ReadOnly, 1 = ReadWrite
    pub is_tag: u8,      // 0 = fragment, 1 = tag
}

/// Returns all registered mass systems.
pub fn registered_mass_systems() -> inventory::iter<MassSystemRegistration> {
    inventory::iter::<MassSystemRegistration>
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
    fn mass_query_from_slice() {
        let mut data = [
            TestFragment { x: 1.0, y: 2.0 },
            TestFragment { x: 3.0, y: 4.0 },
        ];
        let query = unsafe {
            MassQuery::<TestFragment>::from_raw(
                data.as_mut_ptr() as *mut c_void,
                data.len(),
            )
        };
        assert_eq!(query.len(), 2);
        assert!(!query.is_empty());
    }

    #[test]
    fn mass_query_iter() {
        let mut data = [
            TestFragment { x: 1.0, y: 2.0 },
            TestFragment { x: 3.0, y: 4.0 },
        ];
        let query = unsafe {
            MassQuery::<TestFragment>::from_raw(
                data.as_mut_ptr() as *mut c_void,
                data.len(),
            )
        };
        let xs: Vec<f64> = query.iter().map(|f| f.x).collect();
        assert_eq!(xs, vec![1.0, 3.0]);
    }

    #[test]
    fn mass_query_iter_mut() {
        let mut data = [
            TestFragment { x: 1.0, y: 2.0 },
            TestFragment { x: 3.0, y: 4.0 },
        ];
        let mut query = unsafe {
            MassQuery::<TestFragment>::from_raw(
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
    fn mass_fragment_trait() {
        assert_eq!(TestFragment::CPP_TYPE_NAME, "FTestFragment");
        assert_eq!(std::mem::size_of::<TestFragment>(), 16);
    }
}
