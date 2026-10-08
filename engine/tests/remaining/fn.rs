use super::*;

#[test]
fn the_time_source_reports_zero_off_wasm() {
    assert_eq!(
        SchedulerState::current_time(),
        0.0,
        "performance.now() does not exist off wasm, and the guard must report \
         0.0 rather than panicking or returning garbage"
    );
}

#[test]
fn the_time_source_is_monotonic_within_a_run() {
    let first: f64 = SchedulerState::current_time();
    let second: f64 = SchedulerState::current_time();
    assert!(second >= first, "time must never go backwards");
}

#[test]
fn the_three_body_types_are_mutually_distinct() {
    let all: [BodyType; 3] = [BodyType::Static, BodyType::Dynamic, BodyType::Kinematic];
    for i in 0..all.len() {
        for j in (i + 1)..all.len() {
            assert_ne!(all[i], all[j]);
        }
    }
}

#[test]
fn a_body_defaults_to_static_so_it_never_moves_unless_asked() {
    assert_eq!(
        BodyType::default(),
        BodyType::Static,
        "the inert body must be the default, not the simulated one"
    );
}

#[test]
fn a_ray_clones_itself_with_a_new_depth() {
    let ray: Ray = Ray::new(Vector3D::new(1.0, 2.0, 3.0), Vector3D::new(0.0, 0.0, 1.0));
    let deeper: Ray = ray.with_depth(3);
    let rendered: String = format!("{deeper:?}");
    assert!(
        rendered.contains("depth: 3"),
        "with_depth must replace the recursion budget, got: {rendered}"
    );
    assert!(
        rendered.contains("origin: Vector3D { x: 1.0, y: 2.0, z: 3.0 }"),
        "the clone must keep the original origin, got: {rendered}"
    );
    assert!(
        rendered.contains("direction: Vector3D { x: 0.0, y: 0.0, z: 1.0 }"),
        "the clone must keep the original direction, got: {rendered}"
    );
}

#[test]
fn deepening_a_ray_leaves_the_original_untouched() {
    let ray: Ray = Ray::new(Vector3D::zero(), Vector3D::new(1.0, 0.0, 0.0));
    let _: Ray = ray.with_depth(5);
    let rendered: String = format!("{ray:?}");
    assert!(
        !rendered.contains("depth: 5"),
        "with_depth returns a clone, it must not mutate in place, got: {rendered}"
    );
}

#[test]
fn a_ray_starts_at_a_known_recursion_depth() {
    let ray: Ray = Ray::new(Vector3D::zero(), Vector3D::new(1.0, 0.0, 0.0));
    let rendered: String = format!("{ray:?}");
    assert!(
        rendered.contains("depth: 0"),
        "a fresh ray should report the initial budget so a zero-depth bug is visible"
    );
}

#[test]
fn a_pool_exposes_its_free_list_for_bulk_seeding() {
    let mut pool: ObjectPool<u32> = ObjectPool::new(Vec::new());
    let free: &mut Vec<u32> = pool.get_mut_free();
    free.push(1);
    free.push(2);
    assert_eq!(pool.get_free().len(), 2);
    assert_eq!(pool.get_free(), &vec![1_u32, 2]);
}

#[test]
fn a_pool_reports_how_many_values_it_can_hand_out() {
    let mut pool: ObjectPool<u32> = ObjectPool::new(Vec::new());
    assert_eq!(pool.available(), 0);
    pool.get_mut_free().push(7);
    assert_eq!(
        pool.available(),
        1,
        "available is the free-list length, so seeding must be visible immediately"
    );
}

#[test]
fn a_seeded_pool_hands_out_exactly_what_was_put_back() {
    let mut pool: ObjectPool<u32> = ObjectPool::new(Vec::new());
    pool.get_mut_free().extend([1, 2, 3]);
    assert_eq!(
        pool.acquire(),
        Some(3),
        "the free list is most-recently-released first"
    );
    assert_eq!(pool.available(), 2);
    assert_eq!(pool.acquire(), Some(2));
    assert_eq!(pool.acquire(), Some(1));
    assert_eq!(pool.available(), 0);
}

#[test]
fn a_pool_with_an_exhausted_free_list_reports_none_rather_than_inventing_a_value() {
    let mut pool: ObjectPool<u32> = ObjectPool::new(vec![1]);
    assert_eq!(pool.acquire(), Some(1));
    assert_eq!(
        pool.acquire(),
        None,
        "an empty free list must read as None so the caller can decide whether to grow"
    );
}

#[test]
fn an_exhausted_pool_can_still_build_a_value_on_demand() {
    let mut pool: ObjectPool<u32> = ObjectPool::new(vec![1]);
    let _: Option<u32> = pool.acquire();
    let made: u32 = pool.acquire_with(|| 99);
    assert_eq!(
        made, 99,
        "acquire_with falls back to the factory once the pool runs dry"
    );
}

#[test]
fn releasing_a_value_puts_it_back_on_the_free_list() {
    let mut pool: ObjectPool<u32> = ObjectPool::new(vec![42]);
    let value: u32 = pool.acquire().expect("the seeded pool still has a value");
    assert_eq!(value, 42);
    assert_eq!(
        pool.available(),
        0,
        "checking a value out removes it from the free list"
    );
    pool.release(value);
    assert_eq!(
        pool.available(),
        1,
        "a released value is immediately reusable"
    );
}

#[test]
fn an_active_count_can_be_overridden_for_bulk_operations() {
    let mut pool: ObjectPool<u32> = ObjectPool::new(vec![1, 2, 3]);
    assert_eq!(pool.len(), 3);
    pool.set_active(0);
    assert_eq!(
        pool.get_active(),
        0,
        "the caller owns the count, so set_active must take effect verbatim"
    );
}

#[test]
fn a_uv_rect_holds_normalised_texture_coordinates() {
    let rect: UvRect = UvRect::new(0.0, 0.0, 0.5, 0.5);
    let rendered: String = format!("{rect:?}");
    assert!(
        rendered.contains("u0: 0.0") && rendered.contains("v1: 0.5"),
        "a uv rect must round-trip its four normalised edges, got: {rendered}"
    );
    let other: UvRect = UvRect::new(0.5, 0.5, 1.0, 1.0);
    assert_ne!(rect, other, "two different sub-rects must stay distinct");
}

#[test]
fn a_uv_rect_is_copy_so_it_can_be_returned_by_value() {
    let rect: UvRect = UvRect::new(0.1, 0.2, 0.3, 0.4);
    let copied: UvRect = rect;
    assert_eq!(copied, rect);
}

#[test]
fn nine_slice_insets_are_four_independent_edge_widths() {
    let insets: NineSliceInsets = NineSliceInsets::new(1.0, 2.0, 3.0, 4.0);
    let rendered: String = format!("{insets:?}");
    for edge in ["left", "top", "right", "bottom"] {
        assert!(
            rendered.contains(edge),
            "a nine-slice needs all four edge insets, {edge} missing from: {rendered}"
        );
    }
    assert_ne!(insets, NineSliceInsets::new(4.0, 3.0, 2.0, 1.0));
}

#[test]
fn a_tween_remembers_the_delay_it_was_given() {
    let mut tween: Tween<f64> = Tween::create(0.0, 1.0, 1.0);
    tween.set_delay(0.25);
    assert!(
        format!("{tween:?}").contains("delay: 0.25"),
        "set_delay must store the value verbatim, got: {tween:?}"
    );
}

#[test]
fn a_tween_delay_of_zero_is_still_a_delay() {
    let mut tween: Tween<f64> = Tween::create(0.0, 1.0, 1.0);
    tween.set_delay(0.5);
    tween.set_delay(0.0);
    assert!(
        format!("{tween:?}").contains("delay: 0.0"),
        "clearing the delay must be possible, got: {tween:?}"
    );
}

#[test]
fn a_component_handle_is_a_shared_cell_around_a_trait_object() {
    struct Probe;
    impl Lifecycle for Probe {
        fn on_update(&mut self, _: f64) {}
    }
    impl Component for Probe {
        fn on_start(&mut self) {}
        fn on_render(&self, _: &mut DrawList, _: &Transform2D) {}
        fn on_destroy(&mut self) {}
        fn name(&self) -> &str {
            "probe"
        }
    }
    let handle: ComponentRc = Rc::new(EngineCell::new(Probe));
    let cloned: ComponentRc = Rc::clone(&handle);
    assert!(
        Rc::ptr_eq(&handle, &cloned),
        "the handle is an std::rc::Rc alias"
    );
}

#[test]
fn an_entity_handle_is_a_shared_cell_around_an_entity() {
    let handle: EntityRc = Rc::new(EngineCell::new(Entity::create("probe")));
    let cloned: EntityRc = Rc::clone(&handle);
    assert!(
        Rc::ptr_eq(&handle, &cloned),
        "the handle is an std::rc::Rc alias"
    );
    assert_eq!(
        cloned.get().get_name(),
        "probe",
        "the handle must expose the very entity it was built from"
    );
}

#[test]
fn one_entity_event_can_have_several_handlers() {
    let seen: Rc<Cell<u32>> = Rc::new(Cell::new(0));
    let first: Rc<Cell<u32>> = seen.clone();
    let second: Rc<Cell<u32>> = seen.clone();
    let mut handlers: EventHandlers = EventHandlers::new();
    let entry: &mut Vec<EventHandler> = handlers.entry(String::from("hit")).or_default();
    entry.push(Rc::new(move |_: &EntityEvent| {
        first.set(first.get() + 1);
    }));
    entry.push(Rc::new(move |_: &EntityEvent| {
        second.set(second.get() + 10);
    }));
    assert_eq!(handlers.len(), 1, "one event name, one bucket");
    let bucket: &Vec<EventHandler> = handlers.get("hit").expect("the bucket was just created");
    for handler in bucket {
        handler(&EntityEvent::Collision {
            other_id: 1,
            normal: Vector2D::new(0.0, 1.0),
            depth: 0.5,
        });
    }
    assert_eq!(seen.get(), 11, "both handlers must run, in bucket order");
}

#[test]
fn a_handler_that_was_never_registered_has_no_bucket() {
    let handlers: EventHandlers = EventHandlers::new();
    assert!(handlers.is_empty());
    assert!(
        !handlers.contains_key("never-emitted"),
        "an unheard event must not have a bucket at all"
    );
}
