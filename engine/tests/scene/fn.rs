use super::*;

fn probe_scene(
    name: &str,
    entered: &Rc<RefCell<u32>>,
    exited: &Rc<RefCell<u32>>,
    updates: &Rc<RefCell<f64>>,
) -> SceneRc {
    let scene: ProbeScene = ProbeScene {
        name: name.to_string(),
        entered: entered.clone(),
        exited: exited.clone(),
        updates: updates.clone(),
    };
    SceneManager::create_scene(scene)
}

#[test]
fn a_new_manager_has_no_scenes_and_no_active_name() {
    let manager: SceneManager = SceneManager::default();
    assert!(!manager.has_scene("level"), "a fresh manager has no scenes");
    assert_eq!(
        manager.current_name(),
        None,
        "a fresh manager has no active scene"
    );
}

#[test]
fn registering_a_scene_makes_it_discoverable() {
    let entered: Rc<RefCell<u32>> = Rc::new(RefCell::new(0));
    let exited: Rc<RefCell<u32>> = Rc::new(RefCell::new(0));
    let updates: Rc<RefCell<f64>> = Rc::new(RefCell::new(0.0));
    let mut manager: SceneManager = SceneManager::default();
    manager.register(
        String::from("level"),
        probe_scene("level", &entered, &exited, &updates),
    );
    assert!(
        manager.has_scene("level"),
        "a registered scene must be discoverable"
    );
    assert!(
        !manager.has_scene("other"),
        "an unregistered name must not resolve"
    );
}

#[test]
fn unregistering_removes_a_scene() {
    let entered: Rc<RefCell<u32>> = Rc::new(RefCell::new(0));
    let exited: Rc<RefCell<u32>> = Rc::new(RefCell::new(0));
    let updates: Rc<RefCell<f64>> = Rc::new(RefCell::new(0.0));
    let mut manager: SceneManager = SceneManager::default();
    manager.register(
        String::from("level"),
        probe_scene("level", &entered, &exited, &updates),
    );
    manager.unregister("level");
    assert!(
        !manager.has_scene("level"),
        "an unregistered scene must no longer resolve"
    );
}

#[test]
fn switching_to_an_unknown_scene_reports_failure() {
    let mut manager: SceneManager = SceneManager::default();
    let switched: bool = manager.switch_to("missing");
    assert!(!switched, "switching to an unregistered scene must fail");
    assert_eq!(
        manager.current_name(),
        None,
        "a failed switch must not set the active name"
    );
}

#[test]
fn switching_to_a_registered_scene_enters_it_and_sets_the_active_name() {
    let entered: Rc<RefCell<u32>> = Rc::new(RefCell::new(0));
    let exited: Rc<RefCell<u32>> = Rc::new(RefCell::new(0));
    let updates: Rc<RefCell<f64>> = Rc::new(RefCell::new(0.0));
    let mut manager: SceneManager = SceneManager::default();
    manager.register(
        String::from("level"),
        probe_scene("level", &entered, &exited, &updates),
    );
    let switched: bool = manager.switch_to("level");
    assert!(switched, "switching to a registered scene must succeed");
    assert_eq!(
        manager.current_name(),
        Some("level"),
        "the active name must be set"
    );
    assert_eq!(
        *entered.borrow(),
        1,
        "entering a scene must fire its enter hook"
    );
}

#[test]
fn switching_away_exits_the_previous_scene() {
    let entered: Rc<RefCell<u32>> = Rc::new(RefCell::new(0));
    let exited: Rc<RefCell<u32>> = Rc::new(RefCell::new(0));
    let updates: Rc<RefCell<f64>> = Rc::new(RefCell::new(0.0));
    let mut manager: SceneManager = SceneManager::default();
    manager.register(
        String::from("first"),
        probe_scene("first", &entered, &exited, &updates),
    );
    manager.register(
        String::from("second"),
        probe_scene("second", &entered, &exited, &updates),
    );
    let _: bool = manager.switch_to("first");
    let _: bool = manager.switch_to("second");
    assert_eq!(
        *exited.borrow(),
        1,
        "leaving a scene must fire its exit hook once"
    );
    assert_eq!(
        manager.current_name(),
        Some("second"),
        "the active name must follow the switch"
    );
}

#[test]
fn a_requested_transition_is_only_applied_on_the_next_update() {
    let entered: Rc<RefCell<u32>> = Rc::new(RefCell::new(0));
    let exited: Rc<RefCell<u32>> = Rc::new(RefCell::new(0));
    let updates: Rc<RefCell<f64>> = Rc::new(RefCell::new(0.0));
    let mut manager: SceneManager = SceneManager::default();
    manager.register(
        String::from("level"),
        probe_scene("level", &entered, &exited, &updates),
    );
    manager.request_transition(String::from("level"));
    assert_eq!(
        manager.current_name(),
        None,
        "requesting a transition must not switch immediately"
    );
    manager.update(0.1);
    assert_eq!(
        manager.current_name(),
        Some("level"),
        "the pending transition must be applied by the next update"
    );
}

#[test]
fn a_pending_transition_is_consumed_only_once() {
    let entered: Rc<RefCell<u32>> = Rc::new(RefCell::new(0));
    let exited: Rc<RefCell<u32>> = Rc::new(RefCell::new(0));
    let updates: Rc<RefCell<f64>> = Rc::new(RefCell::new(0.0));
    let mut manager: SceneManager = SceneManager::default();
    manager.register(
        String::from("level"),
        probe_scene("level", &entered, &exited, &updates),
    );
    manager.request_transition(String::from("level"));
    manager.update(0.1);
    manager.update(0.1);
    assert_eq!(
        *entered.borrow(),
        1,
        "a consumed transition must not re-enter the scene on later updates"
    );
}

#[test]
fn updating_drives_only_the_active_scene() {
    let entered: Rc<RefCell<u32>> = Rc::new(RefCell::new(0));
    let exited: Rc<RefCell<u32>> = Rc::new(RefCell::new(0));
    let active_updates: Rc<RefCell<f64>> = Rc::new(RefCell::new(0.0));
    let idle_updates: Rc<RefCell<f64>> = Rc::new(RefCell::new(0.0));
    let mut manager: SceneManager = SceneManager::default();
    manager.register(
        String::from("active"),
        probe_scene("active", &entered, &exited, &active_updates),
    );
    manager.register(
        String::from("idle"),
        probe_scene("idle", &entered, &exited, &idle_updates),
    );
    let _: bool = manager.switch_to("active");
    manager.update(0.5);
    manager.update(0.5);
    assert!(
        (*active_updates.borrow() - 1.0).abs() < 1e-9,
        "the active scene must receive the accumulated delta, got {}",
        *active_updates.borrow()
    );
    assert_eq!(
        *idle_updates.borrow(),
        0.0,
        "an inactive scene must not be updated"
    );
}

#[test]
fn updating_with_no_active_scene_is_a_no_op() {
    let manager: SceneManager = SceneManager::default();
    let mut manager: SceneManager = manager;
    manager.update(0.1);
    assert_eq!(
        manager.current_name(),
        None,
        "an update must not invent an active scene"
    );
}

#[test]
fn re_registering_a_name_replaces_the_previous_scene() {
    let entered: Rc<RefCell<u32>> = Rc::new(RefCell::new(0));
    let exited: Rc<RefCell<u32>> = Rc::new(RefCell::new(0));
    let updates: Rc<RefCell<f64>> = Rc::new(RefCell::new(0.0));
    let mut manager: SceneManager = SceneManager::default();
    manager.register(
        String::from("level"),
        probe_scene("level", &entered, &exited, &updates),
    );
    manager.register(
        String::from("level"),
        probe_scene("level", &entered, &exited, &updates),
    );
    let switched: bool = manager.switch_to("level");
    assert!(switched, "the replaced scene must still be reachable");
    assert_eq!(
        *entered.borrow(),
        1,
        "only the surviving scene must be entered"
    );
}

fn scene_counters() -> (Rc<RefCell<u32>>, Rc<RefCell<u32>>, Rc<RefCell<f64>>) {
    (
        Rc::new(RefCell::new(0)),
        Rc::new(RefCell::new(0)),
        Rc::new(RefCell::new(0.0)),
    )
}

#[test]
fn a_requested_scene_transition_only_happens_when_it_is_processed() {
    let (entered, exited, updates) = scene_counters();
    let mut manager: SceneManager = SceneManager::default();
    manager.register(
        String::from("title"),
        probe_scene("title", &entered, &exited, &updates),
    );
    manager.register(
        String::from("level-1"),
        probe_scene("level-1", &entered, &exited, &updates),
    );
    assert!(manager.switch_to("title"), "both scenes are registered");
    assert_eq!(*entered.borrow(), 1, "entering the first scene");
    manager.request_transition(String::from("level-1"));
    assert_eq!(
        manager.current_name(),
        Some("title"),
        "requesting must not switch on its own"
    );
    assert_eq!(*exited.borrow(), 0, "nor may it fire the exit hook");
    manager.process_pending_transition();
    assert_eq!(
        manager.current_name(),
        Some("level-1"),
        "processing applies the pending name"
    );
    assert_eq!(*exited.borrow(), 1, "the old scene is exited exactly once");
    assert_eq!(*entered.borrow(), 2, "the new scene is entered exactly once");
}

#[test]
fn processing_with_nothing_pending_leaves_the_current_scene_alone() {
    let (entered, exited, updates) = scene_counters();
    let mut manager: SceneManager = SceneManager::default();
    manager.register(
        String::from("title"),
        probe_scene("title", &entered, &exited, &updates),
    );
    manager.switch_to("title");
    manager.process_pending_transition();
    assert_eq!(
        manager.current_name(),
        Some("title"),
        "an empty queue is a no-op, not an error"
    );
    assert_eq!(*exited.borrow(), 0, "a no-op fires no hooks at all");
    manager.process_pending_transition();
    assert_eq!(
        manager.current_name(),
        Some("title"),
        "it stays a no-op when called twice"
    );
    assert_eq!(*entered.borrow(), 1, "the scene is still entered only once");
}

#[test]
fn a_pending_transition_to_an_unregistered_scene_is_dropped_without_disturbing_the_current_one() {
    let (entered, exited, updates) = scene_counters();
    let mut manager: SceneManager = SceneManager::default();
    manager.register(
        String::from("title"),
        probe_scene("title", &entered, &exited, &updates),
    );
    manager.switch_to("title");
    manager.request_transition(String::from("nowhere"));
    manager.process_pending_transition();
    assert_eq!(
        manager.current_name(),
        Some("title"),
        "switch_to refuses an unknown name, so the pending name is consumed and ignored"
    );
    assert_eq!(*exited.borrow(), 0, "the current scene is not exited");
}

#[test]
fn the_pending_name_is_consumed_even_when_the_switch_fails() {
    let (entered, exited, updates) = scene_counters();
    let mut manager: SceneManager = SceneManager::default();
    manager.register(
        String::from("title"),
        probe_scene("title", &entered, &exited, &updates),
    );
    manager.switch_to("title");
    manager.request_transition(String::from("nowhere"));
    manager.process_pending_transition();
    manager.register(
        String::from("nowhere"),
        probe_scene("nowhere", &entered, &exited, &updates),
    );
    manager.process_pending_transition();
    assert_eq!(
        manager.current_name(),
        Some("title"),
        "registering the scene afterwards must not revive the consumed request"
    );
}
