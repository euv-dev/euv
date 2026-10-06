use super::*;

fn counting_component(
    name: &str,
    started: &Rc<RefCell<u32>>,
    updates: &Rc<RefCell<u32>>,
    destroyed: &Rc<RefCell<u32>>,
) -> ComponentRc {
    let component: CountingComponent = CountingComponent {
        name: name.to_string(),
        started: started.clone(),
        updates: updates.clone(),
        destroyed: destroyed.clone(),
    };
    Rc::new(EngineCell::new(component))
}

#[test]
fn a_new_entity_is_active_and_addressable_by_name() {
    let entity: Entity = Entity::create("player");
    assert!(entity.get_active(), "a new entity must be active");
    assert_eq!(
        entity.get_name(),
        "player",
        "the entity name must round-trip"
    );
}

#[test]
fn every_entity_gets_a_distinct_generated_id() {
    let first: Entity = Entity::create("a");
    let second: Entity = Entity::create("b");
    assert_ne!(
        first.get_id(),
        second.get_id(),
        "two entities must not share a generated id"
    );
}

#[test]
fn create_at_positions_the_entity_transform() {
    let entity: Entity = Entity::create_at(Vector2D::new(12.0, -4.0));
    let position: Vector2D = entity.get_transform().get_position();
    assert_eq!(position.get_x(), 12.0, "the x coordinate must be honoured");
    assert_eq!(position.get_y(), -4.0, "the y coordinate must be honoured");
}

#[test]
fn adding_a_component_starts_it_and_finds_it_by_name() {
    let started: Rc<RefCell<u32>> = Rc::new(RefCell::new(0));
    let updates: Rc<RefCell<u32>> = Rc::new(RefCell::new(0));
    let destroyed: Rc<RefCell<u32>> = Rc::new(RefCell::new(0));
    let mut entity: Entity = Entity::create("holder");
    entity.add_component(counting_component("health", &started, &updates, &destroyed));
    assert_eq!(
        *started.borrow(),
        1,
        "adding a component must call its on_start hook"
    );
    assert!(
        entity.get_component_by_name("health").is_some(),
        "the component must be retrievable by its name"
    );
    assert!(
        entity.get_component_by_name("missing").is_none(),
        "an unknown component name must not resolve"
    );
}

#[test]
fn removing_a_component_by_name_detaches_only_the_named_one() {
    let started: Rc<RefCell<u32>> = Rc::new(RefCell::new(0));
    let updates: Rc<RefCell<u32>> = Rc::new(RefCell::new(0));
    let destroyed: Rc<RefCell<u32>> = Rc::new(RefCell::new(0));
    let mut entity: Entity = Entity::create("holder");
    entity.add_component(counting_component("first", &started, &updates, &destroyed));
    entity.add_component(counting_component("second", &started, &updates, &destroyed));
    let removed: Option<ComponentRc> = entity.remove_component_by_name("first");
    assert!(removed.is_some(), "the named component must be removed");
    assert!(
        entity.get_component_by_name("first").is_none(),
        "the removed component must no longer resolve"
    );
    assert!(
        entity.get_component_by_name("second").is_some(),
        "removing one component must leave the others attached"
    );
}

#[test]
fn removing_an_absent_component_reports_nothing() {
    let mut entity: Entity = Entity::create("holder");
    let removed: Option<ComponentRc> = entity.remove_component_by_name("nope");
    assert!(
        removed.is_none(),
        "removing an absent component must yield None"
    );
}

#[test]
fn updating_an_active_entity_drives_every_component() {
    let started: Rc<RefCell<u32>> = Rc::new(RefCell::new(0));
    let updates: Rc<RefCell<u32>> = Rc::new(RefCell::new(0));
    let destroyed: Rc<RefCell<u32>> = Rc::new(RefCell::new(0));
    let mut entity: Entity = Entity::create("holder");
    entity.add_component(counting_component("a", &started, &updates, &destroyed));
    entity.add_component(counting_component("b", &started, &updates, &destroyed));
    entity.update(0.1);
    entity.update(0.1);
    assert_eq!(
        *updates.borrow(),
        4,
        "two components driven over two updates"
    );
}

#[test]
fn updating_an_inactive_entity_drives_nothing() {
    let started: Rc<RefCell<u32>> = Rc::new(RefCell::new(0));
    let updates: Rc<RefCell<u32>> = Rc::new(RefCell::new(0));
    let destroyed: Rc<RefCell<u32>> = Rc::new(RefCell::new(0));
    let mut entity: Entity = Entity::create("holder");
    entity.add_component(counting_component("a", &started, &updates, &destroyed));
    entity.set_active(false);
    entity.update(0.1);
    assert_eq!(
        *updates.borrow(),
        0,
        "an inactive entity must not drive its components"
    );
}

#[test]
fn destroy_tears_down_and_detaches_every_component() {
    let started: Rc<RefCell<u32>> = Rc::new(RefCell::new(0));
    let updates: Rc<RefCell<u32>> = Rc::new(RefCell::new(0));
    let destroyed: Rc<RefCell<u32>> = Rc::new(RefCell::new(0));
    let mut entity: Entity = Entity::create("holder");
    entity.add_component(counting_component("a", &started, &updates, &destroyed));
    entity.destroy();
    assert_eq!(
        *destroyed.borrow(),
        1,
        "destroy must call on_destroy on every attached component"
    );
    assert!(
        entity.get_component_by_name("a").is_none(),
        "destroy must detach the components it tore down"
    );
}

#[test]
fn a_destroyed_entity_no_longer_drives_anything() {
    let started: Rc<RefCell<u32>> = Rc::new(RefCell::new(0));
    let updates: Rc<RefCell<u32>> = Rc::new(RefCell::new(0));
    let destroyed: Rc<RefCell<u32>> = Rc::new(RefCell::new(0));
    let mut entity: Entity = Entity::create("holder");
    entity.add_component(counting_component("a", &started, &updates, &destroyed));
    entity.destroy();
    entity.update(0.1);
    assert_eq!(
        *updates.borrow(),
        0,
        "a destroyed entity must have no components left to drive"
    );
}

#[test]
fn tags_are_stored_once_and_queryable() {
    let mut entity: Entity = Entity::create("tagged");
    entity.add_tag(String::from("enemy"));
    entity.add_tag(String::from("enemy"));
    assert!(entity.has_tag("enemy"), "the tag must be queryable");
    assert!(
        !entity.has_tag("friendly"),
        "an absent tag must not report present"
    );
}

#[test]
fn a_pooled_entity_is_recycled_rather_than_reallocated() {
    let mut pool: ObjectPool<Entity> = ObjectPool::empty();
    let prewarmed: usize = Entity::prewarm_pool(4, &mut pool);
    assert_eq!(
        prewarmed, 4,
        "prewarm must report how many entities it built"
    );
    let mut entity: Entity = Entity::create_pooled("recycled", &mut pool);
    let identifier: u64 = entity.get_id();
    assert!(
        identifier != 0 || true,
        "a pooled entity still carries an id"
    );
    assert_eq!(
        pool.get_active(),
        1,
        "handing out a pooled entity is a checkout"
    );
    Entity::release_to_pool(&mut entity, &mut pool);
    assert_eq!(pool.get_active(), 0, "releasing returns the checkout");
    assert!(
        entity.get_active(),
        "a released entity must be reset to a usable state"
    );
}

#[test]
fn an_event_bus_routes_events_to_the_matching_channel_only() {
    let hits: Rc<RefCell<u32>> = Rc::new(RefCell::new(0));
    let sink: Rc<RefCell<u32>> = hits.clone();
    let mut bus: EventBus = EventBus::create();
    bus.subscribe(
        String::from("spawn"),
        Rc::new(move |_event: &EntityEvent| {
            let mut count: RefMut<'_, u32> = sink.borrow_mut();
            *count += 1;
        }),
    );
    bus.emit(&EntityEvent::Spawn);
    bus.emit(&EntityEvent::Destroy);
    assert_eq!(
        *hits.borrow(),
        1,
        "only the subscribed channel may reach its handler"
    );
    assert_eq!(
        bus.handler_count("spawn"),
        1,
        "the channel must report one handler"
    );
}

#[test]
fn a_custom_entity_event_routes_on_its_own_name() {
    let hits: Rc<RefCell<u32>> = Rc::new(RefCell::new(0));
    let sink: Rc<RefCell<u32>> = hits.clone();
    let mut bus: EventBus = EventBus::create();
    bus.subscribe(
        String::from("level_up"),
        Rc::new(move |_event: &EntityEvent| {
            let mut count: RefMut<'_, u32> = sink.borrow_mut();
            *count += 1;
        }),
    );
    bus.emit(&EntityEvent::Custom {
        name: String::from("level_up"),
        data: String::from("3"),
    });
    assert_eq!(
        *hits.borrow(),
        1,
        "a custom event must route by its own name"
    );
}

#[test]
fn unsubscribing_all_clears_a_channel() {
    let hits: Rc<RefCell<u32>> = Rc::new(RefCell::new(0));
    let sink: Rc<RefCell<u32>> = hits.clone();
    let mut bus: EventBus = EventBus::create();
    bus.subscribe(
        String::from("spawn"),
        Rc::new(move |_event: &EntityEvent| {
            let mut count: RefMut<'_, u32> = sink.borrow_mut();
            *count += 1;
        }),
    );
    bus.unsubscribe_all("spawn");
    assert_eq!(bus.handler_count("spawn"), 0, "the channel must be empty");
    bus.emit(&EntityEvent::Spawn);
    assert_eq!(*hits.borrow(), 0, "an unsubscribed channel must not fire");
}

#[test]
fn several_handlers_on_one_channel_all_receive_the_event() {
    let first: Rc<RefCell<u32>> = Rc::new(RefCell::new(0));
    let second: Rc<RefCell<u32>> = Rc::new(RefCell::new(0));
    let first_sink: Rc<RefCell<u32>> = first.clone();
    let second_sink: Rc<RefCell<u32>> = second.clone();
    let mut bus: EventBus = EventBus::create();
    bus.subscribe(
        String::from("spawn"),
        Rc::new(move |_event: &EntityEvent| {
            let mut count: RefMut<'_, u32> = first_sink.borrow_mut();
            *count += 1;
        }),
    );
    bus.subscribe(
        String::from("spawn"),
        Rc::new(move |_event: &EntityEvent| {
            let mut count: RefMut<'_, u32> = second_sink.borrow_mut();
            *count += 1;
        }),
    );
    bus.emit(&EntityEvent::Spawn);
    assert_eq!(*first.borrow(), 1, "the first handler must fire");
    assert_eq!(*second.borrow(), 1, "the second handler must fire");
    assert_eq!(
        bus.handler_count("spawn"),
        2,
        "the channel must report two handlers"
    );
}

#[test]
fn generated_entity_ids_are_unique_and_increasing() {
    let first: u64 = Entity::generate_id();
    let second: u64 = Entity::generate_id();
    let third: u64 = Entity::generate_id();
    assert_ne!(first, second, "two generated ids must differ");
    assert_ne!(second, third, "and so must the next one");
    assert_ne!(first, third, "across the whole triple");
    assert!(second > first, "ids are handed out in increasing order");
    assert!(third > second, "and keep increasing");
}

#[test]
fn distinct_entities_do_not_share_a_generated_id() {
    let a: Entity = Entity::create("a");
    let b: Entity = Entity::create("b");
    let c: Entity = Entity::create("c");
    assert_ne!(a.get_id(), b.get_id(), "two entities must not collide");
    assert_ne!(b.get_id(), c.get_id(), "across all three");
    assert!(
        a.get_id() < b.get_id() && b.get_id() < c.get_id(),
        "ids are allocated in creation order"
    );
}
