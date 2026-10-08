use super::*;

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
