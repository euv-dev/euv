use super::*;

fn counters() -> Counters {
    Counters {
        starts: Rc::new(Cell::new(0)),
        updates: Rc::new(Cell::new(0)),
        renders: Rc::new(Cell::new(0)),
        destroys: Rc::new(Cell::new(0)),
    }
}

pub fn component(name: &str, counters: &Counters) -> ComponentRc {
    let recorder: Recorder = Recorder::new(
        Rc::clone(&counters.starts),
        Rc::clone(&counters.updates),
        Rc::clone(&counters.renders),
        Rc::clone(&counters.destroys),
        name.to_string(),
    );
    Rc::new(EngineCell::new(recorder)) as ComponentRc
}

pub fn total(counters: &Counters) -> u32 {
    counters.starts.get()
        + counters.updates.get()
        + counters.renders.get()
        + counters.destroys.get()
}

#[test]
fn generate_id_hands_out_a_different_value_each_time() {
    let first: u64 = Entity::generate_id();
    let second: u64 = Entity::generate_id();
    assert_ne!(first, second, "ids are allocated from a monotonic counter");
}

#[test]
fn generate_id_never_repeats_across_a_long_run() {
    let mut seen: Vec<u64> = Vec::new();
    for _ in 0..64 {
        seen.push(Entity::generate_id());
    }
    let before: usize = seen.len();
    seen.sort_unstable();
    seen.dedup();
    assert_eq!(seen.len(), before, "no id is handed out twice");
}

#[test]
fn create_builds_an_active_entity_at_the_origin() {
    let entity: Entity = Entity::create("player");
    assert_eq!(entity.get_name(), "player", "the name is stored verbatim");
    assert!(entity.get_active(), "a fresh entity is active");
    assert!(
        entity.get_components().is_empty(),
        "a fresh entity has no components"
    );
}

#[test]
fn create_accepts_an_owned_string_name() {
    let entity: Entity = Entity::create(String::from("enemy"));
    assert_eq!(
        entity.get_name(),
        "enemy",
        "the generic name parameter accepts String"
    );
}

#[test]
fn create_at_places_the_entity_where_asked() {
    let entity: Entity = Entity::create_at(Vector2D::new(10.0, 20.0));
    let transform: Transform2D = entity.get_transform();
    let observed: Vector2D = transform.apply_to_point(Vector2D::zero());
    assert!(
        (observed.get_x() - 10.0).abs() < 1e-9 && (observed.get_y() - 20.0).abs() < 1e-9,
        "the origin maps to the requested position, got {observed:?}"
    );
}

#[test]
fn add_component_fires_on_start_immediately() {
    let counters: Counters = counters();
    let mut entity: Entity = Entity::create("host");
    entity.add_component(component("body", &counters));
    assert_eq!(
        counters.starts.get(),
        1,
        "on_start runs at attach time, not at update time"
    );
    assert_eq!(total(&counters), 1, "nothing else has run yet");
}

#[test]
fn add_component_appends_rather_than_replaces() {
    let counters: Counters = counters();
    let mut entity: Entity = Entity::create("host");
    entity.add_component(component("first", &counters));
    entity.add_component(component("second", &counters));
    assert_eq!(
        entity.get_components().len(),
        2,
        "both components are attached"
    );
}

#[test]
fn update_runs_every_component_once() {
    let counters: Counters = counters();
    let mut entity: Entity = Entity::create("host");
    entity.add_component(component("a", &counters));
    entity.add_component(component("b", &counters));
    let _: u32 = {
        let _: f64 = 0.016;
        0
    };
    entity.update(0.016);
    assert_eq!(
        counters.updates.get(),
        2,
        "one update tick reaches both components"
    );
}

#[test]
fn update_on_an_inactive_entity_reaches_nothing() {
    let counters: Counters = counters();
    let mut entity: Entity = Entity::create("host");
    entity.add_component(component("a", &counters));
    entity.set_active(false);
    entity.update(0.016);
    assert_eq!(
        counters.updates.get(),
        0,
        "a deactivated entity does not tick"
    );
}

#[test]
fn get_component_by_name_finds_an_attached_component() {
    let counters: Counters = counters();
    let mut entity: Entity = Entity::create("host");
    entity.add_component(component("body", &counters));
    let observed: Option<ComponentRc> = entity.get_component_by_name("body");
    assert!(observed.is_some(), "an attached component is found by name");
}

#[test]
fn get_component_by_name_is_none_for_an_unknown_name() {
    let counters: Counters = counters();
    let mut entity: Entity = Entity::create("host");
    entity.add_component(component("body", &counters));
    let observed: Option<ComponentRc> = entity.get_component_by_name("missing");
    assert!(observed.is_none(), "an unknown name matches nothing");
}

#[test]
fn remove_component_by_name_detaches_and_reports_on_destroy() {
    let counters: Counters = counters();
    let mut entity: Entity = Entity::create("host");
    entity.add_component(component("body", &counters));
    let removed: Option<ComponentRc> = entity.remove_component_by_name("body");
    assert!(removed.is_some(), "the component comes back to the caller");
    assert_eq!(counters.destroys.get(), 1, "on_destroy runs on removal");
    assert_eq!(entity.get_components().len(), 0, "the entity is left empty");
}

#[test]
fn removing_an_unknown_component_changes_nothing() {
    let counters: Counters = counters();
    let mut entity: Entity = Entity::create("host");
    entity.add_component(component("body", &counters));
    let removed: Option<ComponentRc> = entity.remove_component_by_name("missing");
    assert!(removed.is_none(), "there is nothing to remove");
    assert_eq!(
        entity.get_components().len(),
        1,
        "the real component survives"
    );
    assert_eq!(counters.destroys.get(), 0, "and its on_destroy did not run");
}

#[test]
fn remove_component_by_name_leaves_the_others_alone() {
    let counters: Counters = counters();
    let mut entity: Entity = Entity::create("host");
    entity.add_component(component("keep", &counters));
    entity.add_component(component("drop", &counters));
    let _: Option<ComponentRc> = entity.remove_component_by_name("drop");
    assert_eq!(entity.get_components().len(), 1, "only the named one goes");
    assert!(
        entity.get_component_by_name("keep").is_some(),
        "the survivor is still reachable by name"
    );
}

#[test]
fn destroy_runs_on_destroy_for_every_component_then_empties() {
    let counters: Counters = counters();
    let mut entity: Entity = Entity::create("host");
    entity.add_component(component("a", &counters));
    entity.add_component(component("b", &counters));
    entity.destroy();
    assert_eq!(counters.destroys.get(), 2, "both components are torn down");
    assert_eq!(entity.get_components().len(), 0, "and the list is emptied");
}

#[test]
fn a_destroyed_entity_ticks_nothing_further() {
    let counters: Counters = counters();
    let mut entity: Entity = Entity::create("host");
    entity.add_component(component("a", &counters));
    entity.destroy();
    entity.update(0.016);
    assert_eq!(
        counters.updates.get(),
        0,
        "a destroyed entity has no components left to tick"
    );
}

#[test]
fn add_tag_records_a_new_tag() {
    let mut entity: Entity = Entity::create("host");
    entity.add_tag(String::from("enemy"));
    assert!(entity.has_tag("enemy"), "a tag that was added is reported");
}

#[test]
fn adding_the_same_tag_twice_still_reports_one() {
    let mut entity: Entity = Entity::create("host");
    entity.add_tag(String::from("enemy"));
    entity.add_tag(String::from("enemy"));
    assert_eq!(entity.get_tags().len(), 1, "tags are a set, not a list");
}

#[test]
fn has_tag_is_false_for_a_tag_that_was_never_added() {
    let entity: Entity = Entity::create("host");
    assert!(!entity.has_tag("boss"), "an absent tag is reported absent");
}

#[test]
fn a_fresh_entity_has_no_tags() {
    let entity: Entity = Entity::create("host");
    assert!(
        entity.get_tags().is_empty(),
        "no tags are attached at construction"
    );
}

#[test]
fn subscribe_and_emit_deliver_to_the_handler() {
    let hits: Rc<Cell<u32>> = Rc::new(Cell::new(0));
    let counter: Rc<Cell<u32>> = Rc::clone(&hits);
    let mut bus: EventBus = EventBus::new();
    let handler: EventHandler = Rc::new(move |_: &EntityEvent| {
        counter.set(counter.get() + 1);
    });
    bus.subscribe(String::from("spawn"), handler);
    bus.emit(&EntityEvent::Spawn);
    assert_eq!(hits.get(), 1, "an emitted event reaches its handler");
}

#[test]
fn emitting_an_event_nobody_listens_to_is_harmless() {
    let bus: EventBus = EventBus::new();
    bus.emit(&EntityEvent::Destroy);
    assert_eq!(bus.handler_count("destroy"), 0, "no handler was registered");
}

#[test]
fn handler_count_reports_the_registered_handlers() {
    let hits: Rc<Cell<u32>> = Rc::new(Cell::new(0));
    let counter: Rc<Cell<u32>> = Rc::clone(&hits);
    let mut bus: EventBus = EventBus::new();
    let handler: EventHandler = Rc::new(move |_: &EntityEvent| {
        let _: u32 = counter.get();
    });
    bus.subscribe(String::from("spawn"), handler);
    assert_eq!(bus.handler_count("spawn"), 1, "one handler is registered");
}

#[test]
fn two_handlers_on_one_event_both_run() {
    let hits: Rc<Cell<u32>> = Rc::new(Cell::new(0));
    let mut bus: EventBus = EventBus::new();
    for _ in 0..2 {
        let counter: Rc<Cell<u32>> = Rc::clone(&hits);
        let handler: EventHandler = Rc::new(move |_: &EntityEvent| {
            counter.set(counter.get() + 1);
        });
        bus.subscribe(String::from("spawn"), handler);
    }
    bus.emit(&EntityEvent::Spawn);
    assert_eq!(hits.get(), 2, "both handlers on the same event run");
    assert_eq!(bus.handler_count("spawn"), 2, "and both stay registered");
}

#[test]
fn handlers_are_keyed_per_event_name() {
    let hits: Rc<Cell<u32>> = Rc::new(Cell::new(0));
    let counter: Rc<Cell<u32>> = Rc::clone(&hits);
    let mut bus: EventBus = EventBus::new();
    let handler: EventHandler = Rc::new(move |_: &EntityEvent| {
        counter.set(counter.get() + 1);
    });
    bus.subscribe(String::from("spawn"), handler);
    bus.emit(&EntityEvent::Destroy);
    assert_eq!(
        hits.get(),
        0,
        "a different event does not reach this handler"
    );
}

#[test]
fn a_collision_event_reaches_a_collision_handler() {
    let hits: Rc<Cell<u32>> = Rc::new(Cell::new(0));
    let seen_depth: Rc<Cell<f64>> = Rc::new(Cell::new(-1.0));
    let counter: Rc<Cell<u32>> = Rc::clone(&hits);
    let depth_slot: Rc<Cell<f64>> = Rc::clone(&seen_depth);
    let mut bus: EventBus = EventBus::new();
    let handler: EventHandler = Rc::new(move |event: &EntityEvent| {
        counter.set(counter.get() + 1);
        if let EntityEvent::Collision { depth, .. } = event {
            depth_slot.set(*depth);
        }
    });
    bus.subscribe(String::from("collision"), handler);
    bus.emit(&EntityEvent::Collision {
        other_id: 7,
        normal: Vector2D::right(),
        depth: 2.5,
    });
    assert_eq!(hits.get(), 1, "the collision event is delivered");
    assert!(
        (seen_depth.get() - 2.5).abs() < 1e-9,
        "the payload reaches the handler intact, got {}",
        seen_depth.get()
    );
}

#[test]
fn unsubscribe_all_detaches_every_handler_for_one_event() {
    let hits: Rc<Cell<u32>> = Rc::new(Cell::new(0));
    let counter: Rc<Cell<u32>> = Rc::clone(&hits);
    let mut bus: EventBus = EventBus::new();
    let handler: EventHandler = Rc::new(move |_: &EntityEvent| {
        counter.set(counter.get() + 1);
    });
    bus.subscribe(String::from("spawn"), handler);
    bus.unsubscribe_all("spawn");
    bus.emit(&EntityEvent::Spawn);
    assert_eq!(hits.get(), 0, "a detached handler is never invoked again");
    assert_eq!(bus.handler_count("spawn"), 0, "and the registry is empty");
}

#[test]
fn unsubscribing_one_event_leaves_the_others_attached() {
    let hits: Rc<Cell<u32>> = Rc::new(Cell::new(0));
    let counter: Rc<Cell<u32>> = Rc::clone(&hits);
    let mut bus: EventBus = EventBus::new();
    let handler: EventHandler = Rc::new(move |_: &EntityEvent| {
        counter.set(counter.get() + 1);
    });
    bus.subscribe(String::from("spawn"), Rc::clone(&handler));
    bus.subscribe(String::from("destroy"), handler);
    bus.unsubscribe_all("spawn");
    bus.emit(&EntityEvent::Destroy);
    assert_eq!(hits.get(), 1, "the untouched event still delivers");
}

#[test]
fn a_fresh_event_bus_has_no_handlers() {
    let bus: EventBus = EventBus::new();
    assert_eq!(bus.handler_count("spawn"), 0, "a new bus is empty");
}

#[test]
fn rendering_an_active_entity_reaches_every_component() {
    let counters: Counters = counters();
    let mut entity: Entity = Entity::create("host");
    entity.add_component(component("a", &counters));
    entity.add_component(component("b", &counters));
    let mut list: DrawList = DrawList::create();

    entity.render(&mut list);

    assert_eq!(
        counters.renders.get(),
        2,
        "each attached component gets its render callback, in attachment order"
    );
}

#[test]
fn an_inactive_entity_renders_nothing_at_all() {
    let counters: Counters = counters();
    let mut entity: Entity = Entity::create("host");
    entity.add_component(component("a", &counters));
    let mut list: DrawList = DrawList::create();

    entity.set_active(false);
    entity.render(&mut list);

    assert_eq!(
        counters.renders.get(),
        0,
        "an entity switched off has to drop out of the frame entirely. Calling on_render anyway \
         would leave its draw commands in the list for the renderer to draw anyway, so the flag \
         would only hide the update half of the entity and not the visible half"
    );
    assert_eq!(
        list.len(),
        0,
        "and nothing reached the draw list, which is what the renderer actually consumes"
    );
}

#[test]
fn an_entity_with_no_components_renders_into_an_untouched_list() {
    let entity: Entity = Entity::create("empty");
    let mut list: DrawList = DrawList::create();
    let _: DrawList = DrawList::create();

    entity.render(&mut list);

    assert_eq!(
        list.len(),
        0,
        "an entity that has nothing to draw must not disturb commands other entities recorded"
    );
}
