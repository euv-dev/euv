use super::*;

#[test]
fn seeded_pool_reports_three_available_zero_active_and_len_three() {
    let pool: ObjectPool<u32> = ObjectPool::new(vec![1, 2, 3]);
    assert_eq!(pool.available(), 3, "seeded pool must hold 3 free values");
    assert_eq!(pool.get_active(), 0, "a fresh pool has no checkouts");
    assert_eq!(pool.len(), 3, "len counts active plus free");
    assert!(!pool.is_empty(), "a seeded pool is not empty");
}

#[test]
fn acquire_returns_values_in_lifo_release_order() {
    let mut pool: ObjectPool<u32> = ObjectPool::new(vec![1, 2, 3]);
    let first: u32 = pool.acquire().unwrap();
    let second: u32 = pool.acquire().unwrap();
    let third: u32 = pool.acquire().unwrap();
    assert_eq!((first, second, third), (3, 2, 1), "checkout is LIFO");
    assert_eq!(pool.available(), 0, "free list drains after 3 acquires");
    assert_eq!(pool.get_active(), 3, "3 values are checked out");
    assert_eq!(pool.len(), 3, "len is stable across checkouts");
}

#[test]
fn acquire_on_empty_free_list_reports_none() {
    let mut pool: ObjectPool<u32> = ObjectPool::new(vec![1]);
    let taken: Option<u32> = pool.acquire();
    assert_eq!(taken, Some(1), "the only seeded value is served");
    let miss: Option<u32> = pool.acquire();
    assert_eq!(miss, None, "a 4th acquire must miss");
    assert_eq!(pool.get_active(), 1, "a miss must not count a checkout");
    assert_eq!(pool.len(), 1, "a miss must not grow the pool");
}

#[test]
fn acquire_with_calls_factory_only_when_free_list_is_empty() {
    let mut pool: ObjectPool<u32> = ObjectPool::new(vec![7]);
    let mut builds: u32 = 0;
    let recycled: u32 = pool.acquire_with(|| {
        builds += 1;
        99
    });
    assert_eq!(recycled, 7, "a pooled value beats the factory");
    assert_eq!(builds, 0, "the factory must not run on a hit");
    let built: u32 = pool.acquire_with(|| {
        builds += 1;
        99
    });
    assert_eq!(built, 99, "a miss returns the factory value");
    assert_eq!(builds, 1, "the factory runs exactly once on a miss");
    assert_eq!(pool.len(), 2, "a factory build grows the pool by one");
    assert_eq!(
        pool.get_active(),
        2,
        "a fresh build still counts as a checkout"
    );
}

#[test]
fn release_increases_available_and_leaves_len_unchanged() {
    let mut pool: ObjectPool<u32> = ObjectPool::new(vec![1, 2, 3]);
    let taken: u32 = pool.acquire().unwrap();
    let len_before: usize = pool.len();
    assert_eq!(pool.available(), 2, "one value is checked out");
    pool.release(taken);
    assert_eq!(
        pool.available(),
        3,
        "release returns the value to the free list"
    );
    assert_eq!(
        pool.len(),
        len_before,
        "len is total tracked values and is unchanged"
    );
    assert_eq!(pool.get_active(), 0, "release closes the checkout");
}

#[test]
fn acquire_after_release_returns_the_exact_same_value() {
    let mut pool: ObjectPool<u32> = ObjectPool::new(vec![42]);
    let taken: u32 = pool.acquire().unwrap();
    assert_eq!(taken, 42, "seeded value is served");
    pool.release(taken);
    let again: u32 = pool.acquire().unwrap();
    assert_eq!(again, 42, "the very same value comes back");
    assert_eq!(pool.len(), 1, "a round trip never grows the pool");
}

#[test]
fn round_trip_acquire_release_acquire_is_stable() {
    let mut pool: ObjectPool<u32> = ObjectPool::new(vec![5, 6]);
    for _ in 0..100 {
        let taken: u32 = pool.acquire().unwrap();
        pool.release(taken);
    }
    assert_eq!(
        pool.available(),
        2,
        "a balanced round trip restores the free list"
    );
    assert_eq!(
        pool.get_active(),
        0,
        "a balanced round trip leaves no checkouts"
    );
    assert_eq!(pool.len(), 2, "a balanced round trip keeps len constant");
}

#[test]
fn prewarm_makes_exactly_count_values_available() {
    let mut pool: ObjectPool<u32> = ObjectPool::empty();
    let available: usize = pool.prewarm(4, || 0);
    assert_eq!(available, 4, "prewarm of 4 leaves 4 available");
    assert_eq!(pool.available(), 4, "available matches the prewarm result");
    assert_eq!(pool.len(), 4, "prewarm grows the tracked set by 4");
    assert_eq!(
        pool.get_active(),
        0,
        "prewarmed values are free, not checked out"
    );
}

#[test]
fn prewarm_on_a_populated_pool_adds_exactly_count() {
    let mut pool: ObjectPool<u32> = ObjectPool::new(vec![1, 2]);
    let available: usize = pool.prewarm(3, || 0);
    assert_eq!(available, 5, "prewarm adds to the existing free list");
    assert_eq!(pool.len(), 5, "the tracked set grows by exactly 3");
}

#[test]
fn prewarm_of_zero_changes_nothing() {
    let mut pool: ObjectPool<u32> = ObjectPool::new(vec![1]);
    let available: usize = pool.prewarm(0, || 0);
    assert_eq!(available, 1, "prewarming 0 is a no-op");
    assert_eq!(pool.len(), 1, "prewarming 0 does not grow the pool");
}

#[test]
fn len_is_stable_across_one_thousand_acquire_release_cycles() {
    let mut pool: ObjectPool<u32> = ObjectPool::empty();
    pool.prewarm(4, || 0);
    let high_water: usize = pool.len();
    for _ in 0..1000 {
        let taken: u32 = pool.acquire().unwrap();
        pool.release(taken);
    }
    assert_eq!(
        pool.len(),
        high_water,
        "recycling never reallocates the tracked set"
    );
    assert_eq!(
        pool.get_active(),
        0,
        "1000 balanced cycles leave no checkouts"
    );
    assert_eq!(
        pool.available(),
        4,
        "1000 balanced cycles restore the free list"
    );
}

#[test]
fn clear_discards_free_values_and_resets_the_checkout_count() {
    let mut pool: ObjectPool<u32> = ObjectPool::new(vec![1, 2, 3]);
    let taken: u32 = pool.acquire().unwrap();
    let discarded: usize = pool.clear();
    assert_eq!(
        discarded, 2,
        "clear reports only the free values it dropped"
    );
    assert_eq!(pool.available(), 0, "the free list is empty after clear");
    assert_eq!(pool.get_active(), 0, "clear resets the checkout count");
    assert!(pool.is_empty(), "the pool is empty after clear");
    pool.release(taken);
    assert_eq!(
        pool.available(),
        1,
        "a post-clear release repopulates the free list"
    );
}

#[test]
fn empty_pool_is_empty_and_serves_no_value() {
    let mut pool: ObjectPool<u32> = ObjectPool::empty();
    assert!(pool.is_empty(), "a default pool tracks nothing");
    assert_eq!(pool.len(), 0, "len is 0 on an empty pool");
    assert_eq!(pool.get_active(), 0, "an empty pool has no checkouts");
    let miss: Option<u32> = pool.acquire();
    assert_eq!(miss, None, "an empty pool serves no value");
}

#[test]
fn non_copy_values_survive_a_release_round_trip() {
    let mut pool: ObjectPool<String> = ObjectPool::new(vec![String::from("alpha")]);
    let taken: String = pool.acquire().unwrap();
    assert_eq!(taken, "alpha", "the seeded String is served whole");
    pool.release(taken);
    let again: String = pool.acquire().unwrap();
    assert_eq!(again, "alpha", "the same String value returns, not a copy");
    assert_eq!(pool.len(), 1, "a String round trip never grows the pool");
}

#[test]
fn capacity_and_len_hold_after_peak_then_drain() {
    let mut pool: ObjectPool<u32> = ObjectPool::empty();
    let mut peak: Vec<u32> = Vec::new();
    for value in 0..8 {
        let built: u32 = pool.acquire_with(|| value);
        peak.push(built);
    }
    assert_eq!(pool.len(), 8, "8 concurrent checkouts track 8 values");
    assert_eq!(pool.available(), 0, "none are free at peak");
    for value in peak {
        pool.release(value);
    }
    assert_eq!(
        pool.len(),
        8,
        "draining back to zero keeps the high-water mark"
    );
    assert_eq!(pool.available(), 8, "every value returned to the free list");
    assert_eq!(pool.get_active(), 0, "all checkouts are closed");
}

#[test]
fn pooled_entity_creation_serves_a_new_entity_when_the_pool_is_empty() {
    let mut pool: ObjectPool<Entity> = ObjectPool::empty();
    let entity: Entity = Entity::create_pooled("first", &mut pool);
    assert_eq!(
        entity.get_name(),
        "first",
        "the pooled entity carries its name"
    );
    assert!(entity.get_active(), "a freshly pooled entity is active");
    assert_eq!(pool.get_active(), 1, "the checkout is tracked");
    assert_eq!(pool.len(), 1, "the pool grew to track the new entity");
}

#[test]
fn pooled_entity_creation_reuses_a_recycled_entity_with_a_fresh_id() {
    let mut pool: ObjectPool<Entity> = ObjectPool::empty();
    let mut first: Entity = Entity::create_pooled("first", &mut pool);
    let first_id: u64 = first.get_id();
    Entity::release_to_pool(&mut first, &mut pool);
    let second: Entity = Entity::create_pooled("second", &mut pool);
    assert_eq!(
        second.get_name(),
        "second",
        "the recycled entity takes the new name"
    );
    assert_ne!(
        second.get_id(),
        first_id,
        "a recycled entity must not reuse the previous identifier"
    );
    assert_eq!(pool.len(), 1, "the round trip never grows the entity pool");
}

#[test]
fn release_to_pool_clears_components_tags_and_transform() {
    let mut pool: ObjectPool<Entity> = ObjectPool::empty();
    let mut entity: Entity = Entity::create_pooled_at(Vector2D::new(3.0, 4.0), &mut pool);
    entity.add_tag(String::from("doomed"));
    Entity::release_to_pool(&mut entity, &mut pool);
    let recycled: Entity = pool.acquire().unwrap();
    assert_eq!(
        recycled.get_components().len(),
        0,
        "a recycled entity carries no components"
    );
    assert_eq!(
        recycled.get_tags().len(),
        0,
        "a recycled entity carries no tags"
    );
    assert_eq!(
        recycled.get_transform().get_position(),
        Vector2D::zero(),
        "a recycled entity is back at the identity position"
    );
    assert!(recycled.get_active(), "a recycled entity is active again");
}

#[test]
fn prewarm_pool_builds_exactly_count_entities() {
    let mut pool: ObjectPool<Entity> = ObjectPool::empty();
    let available: usize = Entity::prewarm_pool(3, &mut pool);
    assert_eq!(available, 3, "prewarm of 3 leaves 3 entities available");
    assert_eq!(pool.len(), 3, "the pool tracks 3 prewarmed entities");
    assert_eq!(
        pool.get_active(),
        0,
        "prewarmed entities are free, not checked out"
    );
}

#[test]
fn pooled_spawn_despawn_cycles_keep_the_entity_pool_size_flat() {
    let mut pool: ObjectPool<Entity> = ObjectPool::empty();
    Entity::prewarm_pool(2, &mut pool);
    let high_water: usize = pool.len();
    for index in 0..1000 {
        let mut entity: Entity = Entity::create_pooled(format!("e{index}"), &mut pool);
        entity.add_tag(String::from("temp"));
        Entity::release_to_pool(&mut entity, &mut pool);
    }
    assert_eq!(
        pool.len(),
        high_water,
        "1000 spawn/despawn cycles never grow the pool"
    );
    assert_eq!(
        pool.get_active(),
        0,
        "1000 cycles leave no outstanding checkouts"
    );
    assert_eq!(
        pool.available(),
        2,
        "1000 cycles return every entity to the free list"
    );
}

#[test]
fn create_pooled_at_places_a_recycled_entity_at_the_requested_position() {
    let mut pool: ObjectPool<Entity> = ObjectPool::empty();
    let mut stale: Entity = Entity::create_pooled_at(Vector2D::new(9.0, 9.0), &mut pool);
    Entity::release_to_pool(&mut stale, &mut pool);
    let placed: Entity = Entity::create_pooled_at(Vector2D::new(1.0, 2.0), &mut pool);
    assert_eq!(
        placed.get_transform().get_position(),
        Vector2D::new(1.0, 2.0),
        "the pooled entity sits at the requested position, not the recycled one"
    );
}

#[test]
fn two_live_pooled_entities_never_share_an_identifier() {
    let mut pool: ObjectPool<Entity> = ObjectPool::empty();
    let first: Entity = Entity::create_pooled("a", &mut pool);
    let second: Entity = Entity::create_pooled("b", &mut pool);
    assert_ne!(
        first.get_id(),
        second.get_id(),
        "two live pooled entities must not share an identifier"
    );
    assert_eq!(
        pool.len(),
        2,
        "an empty pool builds one entity per checkout"
    );
    assert_eq!(pool.get_active(), 2, "both checkouts are outstanding");
}

#[test]
fn the_free_list_is_readable_and_writable_through_its_accessors() {
    let mut pool: ObjectPool<u32> = ObjectPool::new(vec![10, 20, 30]);
    let free: &Vec<u32> = pool.get_free();
    assert_eq!(free.len(), 3, "the seeded values are on the free list");
    assert_eq!(*free, vec![10, 20, 30], "in seed order");
    let mutable: &mut Vec<u32> = pool.get_mut_free();
    mutable.push(40);
    assert_eq!(
        pool.available(),
        4,
        "a value pushed through the mutable view is available"
    );
    let taken: Option<u32> = pool.acquire();
    assert_eq!(
        taken,
        Some(40),
        "the free list is LIFO, so the last push comes out first"
    );
    assert_eq!(pool.available(), 3, "and the count drops by one");
}

#[test]
fn the_pool_debug_formatter_reports_counts_rather_than_values() {
    let mut pool: ObjectPool<String> =
        ObjectPool::new(vec![String::from("alpha"), String::from("beta")]);
    let _: Option<String> = pool.acquire();
    let rendered: String = format!("{pool:?}");
    assert!(
        rendered.contains("active") && rendered.contains("1"),
        "the debug form reports the active count, got {rendered}"
    );
    assert!(
        rendered.contains("available") && rendered.contains("1"),
        "and the available count, got {rendered}"
    );
    assert!(
        !rendered.contains("alpha") && !rendered.contains("beta"),
        "but never the values themselves, so a pool of handles stays printable: {rendered}"
    );
}

#[test]
fn a_pool_of_non_debug_payloads_still_prints() {
    let pool: ObjectPool<Vec<u8>> = ObjectPool::new(vec![vec![1, 2, 3]]);
    let rendered: String = format!("{pool:?}");
    assert!(
        rendered.contains("active") && rendered.contains("available"),
        "T is deliberately not required to be Debug, got {rendered}"
    );
}

#[test]
fn the_task_registry_handle_is_returned_by_registration() {
    let registry: TaskRegistryRc = Rc::new(EngineCell::new(TaskRegistry::default()));
    let handle: TaskHandle = SchedulerHandle::register_task(&registry, Timer::create(10.0));
    assert!(!registry.get().is_empty(), "registering must add the task");
    assert_eq!(registry.get().len(), 1, "and the count follows");
    assert!(
        registry.get_mut().unregister(&handle),
        "the handle we were handed unregisters that task"
    );
    assert!(registry.get().is_empty(), "and the registry is empty again");
}
