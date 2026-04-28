//! Facade `QueryAll` type providing global index-based entity access.
//!
//! In Bevy mode: wraps an `EntityIndex<Tag>` resource + a `bevy_ecs::system::Query`
//! to provide `get(index)` / `get_mut(index)` by spawn-order index.
//!
//! In Unreal mode: marker type rewritten by `#[mass_system]` to chunk-based
//! `MassQueryAllRef`/`MassQueryAllMut` (zero-copy access into UE chunk memory).
//!
//! # Usage
//!
//! ```ignore
//! use bevy_mass::prelude::*;
//!
//! #[mass_system]
//! fn apply_food_mutations(
//!     foods: QueryAll<&mut FoodState, With<Food>>,
//! ) {
//!     if let Some(food) = foods.get_mut(index) {
//!         food.is_loose = false;
//!     }
//! }
//! ```

use bevy_ecs::prelude::*;
use std::marker::PhantomData;

/// Resource maintaining spawn-order entity indices for a component group.
///
/// In Bevy standalone mode, game code inserts this at spawn time:
/// ```ignore
/// commands.insert_resource(EntityIndex::<FoodTag>::new(food_entity_vec));
/// ```
///
/// In Unreal mode, this is unused — chunk memory provides direct index access.
#[derive(Resource)]
pub struct EntityIndex<Tag: 'static> {
    pub entities: Vec<Entity>,
    _phantom: PhantomData<Tag>,
}

impl<Tag: 'static> EntityIndex<Tag> {
    pub fn new(entities: Vec<Entity>) -> Self {
        Self {
            entities,
            _phantom: PhantomData,
        }
    }

    pub fn get(&self, index: usize) -> Option<Entity> {
        self.entities.get(index).copied()
    }

    pub fn len(&self) -> usize {
        self.entities.len()
    }

    pub fn is_empty(&self) -> bool {
        self.entities.is_empty()
    }
}

/// Bevy-mode wrapper providing `.get()` and `.get_mut()` by global index.
///
/// Constructed by the `#[mass_system]` macro from `EntityIndex<Tag>` + `Query<D, F>`.
/// Game code calls `foods.get_mut(index)` — the wrapper resolves the index to an
/// Entity via EntityIndex, then looks up the component via the Bevy Query.
pub struct QueryAllWrapper<'a, 'w, 's, D: bevy_ecs::query::QueryData, F: bevy_ecs::query::QueryFilter> {
    entities: &'a [Entity],
    query: &'a mut bevy_ecs::system::Query<'w, 's, D, F>,
}

impl<'a, 'w, 's, D: bevy_ecs::query::QueryData, F: bevy_ecs::query::QueryFilter>
    QueryAllWrapper<'a, 'w, 's, D, F>
{
    pub fn new(
        entities: &'a [Entity],
        query: &'a mut bevy_ecs::system::Query<'w, 's, D, F>,
    ) -> Self {
        Self { entities, query }
    }

    pub fn get_mut(
        &mut self,
        index: usize,
    ) -> Option<D::Item<'_, 's>> {
        let entity = *self.entities.get(index)?;
        self.query.get_mut(entity).ok()
    }

    pub fn get(
        &self,
        index: usize,
    ) -> Option<<D::ReadOnly as bevy_ecs::query::QueryData>::Item<'_, 's>> {
        let entity = *self.entities.get(index)?;
        self.query.get(entity).ok()
    }

    pub fn len(&self) -> usize {
        self.entities.len()
    }

    pub fn is_empty(&self) -> bool {
        self.entities.is_empty()
    }
}

/// Marker type for `QueryAll` in Unreal mode.
/// The `#[mass_system]` macro rewrites this to `MassQueryAllRef`/`MassQueryAllMut`.
/// In Bevy mode, this type is never used directly — the macro rewrites the function
/// to use `QueryAllWrapper` instead.
#[cfg(feature = "unreal")]
pub struct QueryAll<D, F = ()> {
    _phantom: PhantomData<(D, F)>,
}

#[cfg(all(test, not(feature = "unreal")))]
mod tests {
    use super::*;

    #[derive(Component, Clone, Copy, Debug)]
    struct TestFragment {
        pub value: i32,
    }

    #[derive(Component, Clone, Copy, Debug)]
    struct TestTag;

    #[test]
    fn entity_index_basic() {
        let mut world = World::new();
        let e1 = world.spawn(TestFragment { value: 10 }).id();
        let e2 = world.spawn(TestFragment { value: 20 }).id();
        let e3 = world.spawn(TestFragment { value: 30 }).id();

        let index = EntityIndex::<TestTag>::new(vec![e1, e2, e3]);
        assert_eq!(index.len(), 3);
        assert_eq!(index.get(0), Some(e1));
        assert_eq!(index.get(1), Some(e2));
        assert_eq!(index.get(2), Some(e3));
        assert_eq!(index.get(5), None);
    }

    #[test]
    fn query_all_wrapper_get_mut() {
        let mut world = World::new();
        let e1 = world.spawn((TestFragment { value: 10 }, TestTag)).id();
        let e2 = world.spawn((TestFragment { value: 20 }, TestTag)).id();

        let index = EntityIndex::<TestTag>::new(vec![e1, e2]);
        world.insert_resource(index);

        // Use a schedule to test through a real system
        fn test_system(
            index: Res<EntityIndex<TestTag>>,
            mut query: bevy_ecs::system::Query<&mut TestFragment, With<TestTag>>,
        ) {
            let mut wrapper = QueryAllWrapper::new(&index.entities, &mut query);
            assert_eq!(wrapper.len(), 2);

            // get_mut by index
            if let Some(mut frag) = wrapper.get_mut(0) {
                frag.value = 100;
            }
            if let Some(mut frag) = wrapper.get_mut(1) {
                frag.value = 200;
            }
            // out of bounds
            assert!(wrapper.get_mut(5).is_none());
        }

        let mut schedule = bevy_ecs::schedule::Schedule::default();
        schedule.add_systems(test_system);
        schedule.run(&mut world);

        // Verify mutations took effect
        let frag1 = world.get::<TestFragment>(e1).unwrap();
        assert_eq!(frag1.value, 100);
        let frag2 = world.get::<TestFragment>(e2).unwrap();
        assert_eq!(frag2.value, 200);
    }

    #[test]
    fn query_all_wrapper_get_readonly() {
        let mut world = World::new();
        let e1 = world.spawn((TestFragment { value: 42 }, TestTag)).id();

        let index = EntityIndex::<TestTag>::new(vec![e1]);
        world.insert_resource(index);

        fn test_system(
            index: Res<EntityIndex<TestTag>>,
            mut query: bevy_ecs::system::Query<&mut TestFragment, With<TestTag>>,
        ) {
            let wrapper = QueryAllWrapper::new(&index.entities, &mut query);
            let frag = wrapper.get(0).unwrap();
            assert_eq!(frag.value, 42);
            assert!(wrapper.get(1).is_none());
        }

        let mut schedule = bevy_ecs::schedule::Schedule::default();
        schedule.add_systems(test_system);
        schedule.run(&mut world);
    }
}
