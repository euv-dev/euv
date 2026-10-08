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

fn scene_rc(name: &str) -> SceneProbeHandles {
    let probe: Probe = Probe::new(name);
    let enters: Rc<Cell<u32>> = probe.enters();
    let exits: Rc<Cell<u32>> = probe.exits();
    let updates: Rc<Cell<u32>> = probe.updates();
    (SceneManager::create_scene(probe), enters, exits, updates)
}

fn manager() -> SceneManager {
    SceneManager::new()
}

#[test]
fn a_fresh_manager_has_no_current_scene() {
    let mgr: SceneManager = manager();
    assert_eq!(
        *mgr.try_get_current_scene_name(),
        None,
        "nothing is active until a switch happens"
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
fn switching_to_a_registered_scene_reports_success() {
    let mut mgr: SceneManager = manager();
    let (scene, _, _, _): SceneProbeHandles = scene_rc("menu");
    mgr.register(String::from("menu"), scene);
    let observed: bool = mgr.switch_to("menu");
    assert!(observed, "a registered name is a valid switch target");
}

#[test]
fn switching_to_an_unregistered_scene_reports_failure() {
    let mut mgr: SceneManager = manager();
    let observed: bool = mgr.switch_to("nowhere");
    assert!(!observed, "an unknown name cannot become active");
}

#[test]
fn a_failed_switch_leaves_the_previous_scene_active() {
    let mut mgr: SceneManager = manager();
    let (scene, _, _, _): SceneProbeHandles = scene_rc("menu");
    mgr.register(String::from("menu"), scene);
    let _: bool = mgr.switch_to("menu");
    let observed: bool = mgr.switch_to("nowhere");
    assert!(!observed, "the switch is refused");
    assert_eq!(
        *mgr.try_get_current_scene_name(),
        Some(String::from("menu")),
        "and the previous scene stays active"
    );
}

#[test]
fn entering_a_scene_fires_on_enter_once() {
    let mut mgr: SceneManager = manager();
    let (scene, enters, _, _): SceneProbeHandles = scene_rc("menu");
    mgr.register(String::from("menu"), scene);
    let _: bool = mgr.switch_to("menu");
    assert_eq!(enters.get(), 1, "on_enter runs on the first switch");
}

#[test]
fn switching_away_fires_on_exit_on_the_outgoing_scene() {
    let mut mgr: SceneManager = manager();
    let (first, first_enters, first_exits, _): SceneProbeHandles = scene_rc("menu");
    let (second, second_enters, _, _): SceneProbeHandles = scene_rc("game");
    mgr.register(String::from("menu"), first);
    mgr.register(String::from("game"), second);
    let _: bool = mgr.switch_to("menu");
    let _: bool = mgr.switch_to("game");
    assert_eq!(first_enters.get(), 1, "the first scene was entered once");
    assert_eq!(first_exits.get(), 1, "and exited when the second took over");
    assert_eq!(second_enters.get(), 1, "the second scene was entered once");
}

#[test]
fn switching_to_the_same_scene_twice_re_fires_both_hooks() {
    let mut mgr: SceneManager = manager();
    let (scene, enters, exits, _): SceneProbeHandles = scene_rc("menu");
    mgr.register(String::from("menu"), scene);
    let _: bool = mgr.switch_to("menu");
    let _: bool = mgr.switch_to("menu");
    assert_eq!(enters.get(), 2, "each switch re-enters the scene");
    assert_eq!(
        exits.get(),
        1,
        "and each switch exits the previous occupant"
    );
}

#[test]
fn unregister_removes_the_scene_so_it_can_no_longer_be_entered() {
    let mut mgr: SceneManager = manager();
    let (scene, _, _, _): SceneProbeHandles = scene_rc("menu");
    mgr.register(String::from("menu"), scene);
    mgr.unregister("menu");
    let observed: bool = mgr.switch_to("menu");
    assert!(!observed, "an unregistered name is no longer a target");
}

#[test]
fn unregistering_an_unknown_name_is_harmless() {
    let mut mgr: SceneManager = manager();
    let (scene, _, _, _): SceneProbeHandles = scene_rc("menu");
    mgr.register(String::from("menu"), scene);
    mgr.unregister("nowhere");
    let observed: bool = mgr.switch_to("menu");
    assert!(observed, "the real scene is untouched");
}

#[test]
fn registering_the_same_name_twice_replaces_the_scene() {
    let mut mgr: SceneManager = manager();
    let (first, first_enters, _, _): SceneProbeHandles = scene_rc("menu");
    let (second, second_enters, _, _): SceneProbeHandles = scene_rc("menu");
    mgr.register(String::from("menu"), first);
    mgr.register(String::from("menu"), second);
    let _: bool = mgr.switch_to("menu");
    assert_eq!(first_enters.get(), 0, "the replaced scene is never entered");
    assert_eq!(second_enters.get(), 1, "the replacement is the live one");
}

#[test]
fn a_requested_transition_does_not_take_effect_until_processed() {
    let mut mgr: SceneManager = manager();
    let (first, _, _, _): SceneProbeHandles = scene_rc("menu");
    let (second, second_enters, _, _): SceneProbeHandles = scene_rc("game");
    mgr.register(String::from("menu"), first);
    mgr.register(String::from("game"), second);
    let _: bool = mgr.switch_to("menu");
    mgr.request_transition(String::from("game"));
    assert_eq!(
        second_enters.get(),
        0,
        "requesting a transition only records the intent"
    );
    assert_eq!(
        *mgr.try_get_current_scene_name(),
        Some(String::from("menu")),
        "the old scene is still active"
    );
    mgr.process_pending_transition();
    assert_eq!(second_enters.get(), 1, "processing performs the switch");
    assert_eq!(
        *mgr.try_get_current_scene_name(),
        Some(String::from("game")),
        "and the new scene becomes active"
    );
}

#[test]
fn processing_with_nothing_pending_is_a_no_op() {
    let mut mgr: SceneManager = manager();
    let (scene, _, _, _): SceneProbeHandles = scene_rc("menu");
    mgr.register(String::from("menu"), scene);
    let _: bool = mgr.switch_to("menu");
    mgr.process_pending_transition();
    assert_eq!(
        *mgr.try_get_current_scene_name(),
        Some(String::from("menu")),
        "an empty queue changes nothing"
    );
}

#[test]
fn a_requested_transition_to_an_unknown_scene_is_dropped_quietly() {
    let mut mgr: SceneManager = manager();
    let (scene, _, _, _): SceneProbeHandles = scene_rc("menu");
    mgr.register(String::from("menu"), scene);
    let _: bool = mgr.switch_to("menu");
    mgr.request_transition(String::from("nowhere"));
    mgr.process_pending_transition();
    assert_eq!(
        *mgr.try_get_current_scene_name(),
        Some(String::from("menu")),
        "a transition to a missing scene leaves the current one alone"
    );
}

#[test]
fn the_requested_transition_is_consumed_so_it_cannot_replay() {
    let mut mgr: SceneManager = manager();
    let (scene, enters, _, _): SceneProbeHandles = scene_rc("menu");
    mgr.register(String::from("menu"), scene);
    mgr.request_transition(String::from("menu"));
    mgr.process_pending_transition();
    mgr.process_pending_transition();
    assert_eq!(
        enters.get(),
        1,
        "the pending slot is taken by the first process, so the second is a no-op"
    );
}

#[test]
fn update_with_no_active_scene_ticks_nothing() {
    let mut mgr: SceneManager = manager();
    let (scene, _, _, updates): SceneProbeHandles = scene_rc("menu");
    mgr.register(String::from("menu"), scene);
    mgr.update(0.016);
    assert_eq!(updates.get(), 0, "no active scene means no updates");
}

#[test]
fn update_ticks_only_the_active_scene() {
    let mut mgr: SceneManager = manager();
    let (first, _, _, first_updates): SceneProbeHandles = scene_rc("menu");
    let (second, _, _, second_updates): SceneProbeHandles = scene_rc("game");
    mgr.register(String::from("menu"), first);
    mgr.register(String::from("game"), second);
    let _: bool = mgr.switch_to("menu");
    mgr.update(0.016);
    assert_eq!(first_updates.get(), 1, "the active scene ticks");
    assert_eq!(second_updates.get(), 0, "the inactive one does not");
}

#[test]
fn update_applies_a_pending_transition_before_ticking() {
    let mut mgr: SceneManager = manager();
    let (first, _, _, first_updates): SceneProbeHandles = scene_rc("menu");
    let (second, second_enters, _, second_updates): SceneProbeHandles = scene_rc("game");
    mgr.register(String::from("menu"), first);
    mgr.register(String::from("game"), second);
    let _: bool = mgr.switch_to("menu");
    mgr.request_transition(String::from("game"));
    mgr.update(0.016);
    assert_eq!(
        second_enters.get(),
        1,
        "the transition lands in the same update"
    );
    assert_eq!(
        first_updates.get(),
        0,
        "the outgoing scene never gets this tick"
    );
    assert_eq!(
        second_updates.get(),
        1,
        "the incoming scene is the one ticked"
    );
}

#[test]
fn the_scene_reports_its_own_name() {
    let probe: Probe = Probe::new("level-3");
    assert_eq!(probe.name(), "level-3", "a scene knows its own name");
}

#[test]
fn a_fresh_manager_has_no_scenes_registered() {
    let mgr: SceneManager = manager();
    assert!(
        !mgr.has_scene("menu"),
        "nothing is registered on a new manager"
    );
}

#[test]
fn has_scene_reports_a_registered_name() {
    let mut mgr: SceneManager = manager();
    let (scene, _, _, _): SceneProbeHandles = scene_rc("menu");
    mgr.register(String::from("menu"), scene);
    assert!(
        mgr.has_scene("menu"),
        "a registered name is reported present"
    );
}

#[test]
fn has_scene_is_false_after_unregistering() {
    let mut mgr: SceneManager = manager();
    let (scene, _, _, _): SceneProbeHandles = scene_rc("menu");
    mgr.register(String::from("menu"), scene);
    mgr.unregister("menu");
    assert!(
        !mgr.has_scene("menu"),
        "an unregistered name is reported absent"
    );
}

#[test]
fn has_scene_does_not_claim_the_scene_is_active() {
    let mut mgr: SceneManager = manager();
    let (scene, _, _, _): SceneProbeHandles = scene_rc("menu");
    mgr.register(String::from("menu"), scene);
    assert!(mgr.has_scene("menu"), "the scene is registered");
    assert_eq!(
        mgr.current_name(),
        None,
        "but being registered is not the same as being the current scene"
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

type SceneCounters = (Rc<RefCell<u32>>, Rc<RefCell<u32>>, Rc<RefCell<f64>>);

fn scene_counters() -> SceneCounters {
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
    assert_eq!(
        *entered.borrow(),
        2,
        "the new scene is entered exactly once"
    );
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

#[test]
fn the_current_name_follows_a_successful_switch() {
    let mut mgr: SceneManager = manager();
    let (scene, _, _, _): SceneProbeHandles = scene_rc("menu");
    mgr.register(String::from("menu"), scene);
    let _: bool = mgr.switch_to("menu");
    assert_eq!(mgr.current_name(), Some("menu"), "the switch names it");
}

#[test]
fn a_failed_switch_leaves_the_current_name_untouched() {
    let mut mgr: SceneManager = manager();
    let (scene, _, _, _): SceneProbeHandles = scene_rc("menu");
    mgr.register(String::from("menu"), scene);
    let _: bool = mgr.switch_to("menu");
    let _: bool = mgr.switch_to("nowhere");
    assert_eq!(
        mgr.current_name(),
        Some("menu"),
        "a refused switch does not clear the active scene"
    );
}
