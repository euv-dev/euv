use super::*;

fn handle() -> EngineHandle {
    EngineHandle::new(Engine::default_config(), None, None, None, None)
}

#[test]
fn the_default_engine_config_is_the_default_config() {
    assert_eq!(
        Engine::default_config(),
        EngineConfig::default(),
        "default_config must be a plain default, not a second opinion about it"
    );
}

#[test]
fn a_handle_that_was_never_started_is_not_running() {
    assert!(
        !handle().is_running(),
        "the game loop is off until start is called"
    );
}

#[test]
fn a_handle_without_a_scheduler_can_still_be_stopped() {
    let engine: EngineHandle = handle();
    engine.stop();
    assert!(!engine.is_running(), "stopping an idle engine is a no-op");
}

#[test]
fn registering_a_task_returns_a_handle_that_unregisters_it() {
    let engine: EngineHandle = handle();
    let handle_id: TaskHandle = engine.register_task(Timer::create(1.0));
    assert!(
        engine.unregister_task(&handle_id),
        "the handle returned by register_task must remove that task"
    );
}

#[test]
fn unregistering_a_handle_twice_reports_false_the_second_time() {
    let engine: EngineHandle = handle();
    let handle_id: TaskHandle = engine.register_task(Timer::create(1.0));
    assert!(engine.unregister_task(&handle_id));
    assert!(
        !engine.unregister_task(&handle_id),
        "a removed handle is stale, so the second removal must report false"
    );
}

#[test]
fn removing_an_earlier_task_stales_every_later_handle() {
    let engine: EngineHandle = handle();
    let first: TaskHandle = engine.register_task(Timer::create(1.0));
    let second: TaskHandle = engine.register_task(Timer::create(1.0));
    assert!(engine.unregister_task(&first));
    assert!(
        !engine.unregister_task(&second),
        "a handle stores its insertion index, so removing an earlier task \
         shifts every later slot and stales the handle — this is the documented contract"
    );
}

#[test]
fn a_later_handle_outliving_an_earlier_removal_still_holds_a_task() {
    let engine: EngineHandle = handle();
    let first: TaskHandle = engine.register_task(Timer::create(1.0));
    let second: TaskHandle = engine.register_task(Timer::create(1.0));
    assert!(engine.unregister_task(&first));
    assert!(
        !engine.unregister_task(&second),
        "the stale removal must report false rather than silently dropping a survivor"
    );
    assert!(
        !engine.unregister_task(&second),
        "a second stale attempt must stay false instead of double-removing"
    );
}

#[test]
fn unregistering_the_most_recently_registered_task_always_succeeds() {
    let engine: EngineHandle = handle();
    let first: TaskHandle = engine.register_task(Timer::create(1.0));
    let second: TaskHandle = engine.register_task(Timer::create(1.0));
    let third: TaskHandle = engine.register_task(Timer::create(1.0));
    assert!(
        engine.unregister_task(&third),
        "the last handle is the one nothing registered before it can invalidate"
    );
    assert!(engine.unregister_task(&first));
    assert!(
        !engine.unregister_task(&second),
        "second sat after first, so removing first shifted it out from under its handle"
    );
}

#[test]
fn a_fresh_handle_registers_no_asset_loader() {
    let engine: EngineHandle = handle();
    assert!(
        engine.asset_loader().is_none(),
        "assets are opt-in, so nothing may be created before register_assets"
    );
}

#[test]
fn an_engine_with_no_tasks_reports_no_asset_loader_after_stop() {
    let engine: EngineHandle = handle();
    engine.stop();
    assert!(engine.asset_loader().is_none());
    assert!(!engine.is_running());
}

#[test]
fn a_task_registered_after_a_removal_is_still_removable() {
    let engine: EngineHandle = handle();
    let first: TaskHandle = engine.register_task(Timer::create(1.0));
    let removed: bool = engine.unregister_task(&first);
    assert!(removed);
    let late: TaskHandle = engine.register_task(Timer::create(1.0));
    assert!(
        engine.unregister_task(&late),
        "a handle returned after the list settled must be valid immediately"
    );
}
