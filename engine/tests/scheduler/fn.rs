use super::*;

const EPSILON: f64 = 1e-9;

#[test]
fn registry_updates_registered_task_once_per_call() {
    let total: Rc<Cell<f64>> = Rc::new(Cell::new(0.0));
    let calls: Rc<Cell<u32>> = Rc::new(Cell::new(0));
    let mut registry: TaskRegistry = TaskRegistry::default();
    let _ = registry.register(DeltaRecorder::new(total.clone(), calls.clone()));
    registry.update_all(0.5);
    registry.update_all(0.25);
    assert_eq!(
        calls.get(),
        2,
        "task must be driven once per update_all call"
    );
    assert!(
        (total.get() - 0.75).abs() < EPSILON,
        "expected 0.75 accumulated delta, got {}",
        total.get()
    );
}

#[test]
fn registry_empty_is_noop() {
    let mut registry: TaskRegistry = TaskRegistry::default();
    assert!(registry.is_empty());
    registry.update_all(1.0);
    assert!(registry.is_empty());
    assert_eq!(registry.len(), 0);
}

#[test]
fn registry_unregister_stops_driving_task() {
    let total: Rc<Cell<f64>> = Rc::new(Cell::new(0.0));
    let calls: Rc<Cell<u32>> = Rc::new(Cell::new(0));
    let mut registry: TaskRegistry = TaskRegistry::default();
    let handle: TaskHandle = registry.register(DeltaRecorder::new(total.clone(), calls.clone()));
    registry.update_all(1.0);
    assert!(registry.unregister(&handle));
    registry.update_all(1.0);
    assert_eq!(calls.get(), 1, "unregistered task must stop being driven");
    assert!(
        (total.get() - 1.0).abs() < EPSILON,
        "only the pre-removal update may count, got {}",
        total.get()
    );
    assert!(registry.is_empty());
}

#[test]
fn registry_unregister_unknown_handle_returns_false() {
    let mut registry: TaskRegistry = TaskRegistry::default();
    let handle: TaskHandle = registry.register(DeltaRecorder::new(
        Rc::new(Cell::new(0.0)),
        Rc::new(Cell::new(0)),
    ));
    assert!(registry.unregister(&handle));
    assert!(
        !registry.unregister(&handle),
        "removing the same handle twice must report false"
    );
}

#[test]
fn registry_updates_tasks_in_registration_order() {
    let log: Rc<RefCell<Vec<u8>>> = Rc::new(RefCell::new(Vec::new()));
    let mut registry: TaskRegistry = TaskRegistry::default();
    for label in 0..3u8 {
        let _ = registry.register(LoggingTask {
            label,
            log: log.clone(),
        });
    }
    registry.update_all(0.1);
    let observed: Vec<u8> = log.borrow().clone();
    assert_eq!(
        observed,
        vec![0, 1, 2],
        "tasks must be driven in registration order"
    );
}

#[test]
fn registry_clear_removes_every_task() {
    let log: Rc<RefCell<Vec<u8>>> = Rc::new(RefCell::new(Vec::new()));
    let mut registry: TaskRegistry = TaskRegistry::default();
    let _ = registry.register(LoggingTask {
        label: 7,
        log: log.clone(),
    });
    registry.clear();
    registry.update_all(0.1);
    assert!(log.borrow().is_empty(), "cleared tasks must not be driven");
    assert_eq!(registry.len(), 0);
}

#[test]
fn timer_registered_as_task_actually_fires() {
    let fires: Rc<Cell<u32>> = Rc::new(Cell::new(0));
    let mut registry: TaskRegistry = TaskRegistry::default();
    let _ = registry.register(FireCounter::new(0.5, fires.clone()));
    for _ in 0..29 {
        registry.update_all(1.0 / 60.0);
    }
    assert_eq!(fires.get(), 0, "nothing fires before 0.5s elapses");
    for _ in 0..2 {
        registry.update_all(1.0 / 60.0);
    }
    assert_eq!(
        fires.get(),
        1,
        "a real Timer driven through the registry must fire exactly once at 0.5s"
    );
    for _ in 0..120 {
        registry.update_all(1.0 / 60.0);
    }
    assert_eq!(
        fires.get(),
        1,
        "a one-shot Timer must not re-fire after finishing"
    );
}

#[test]
fn tick_drives_tasks_once_per_fixed_step() {
    let total: Rc<Cell<f64>> = Rc::new(Cell::new(0.0));
    let calls: Rc<Cell<u32>> = Rc::new(Cell::new(0));
    let registry: TaskRegistryRc = Rc::new(EngineCell::new(TaskRegistry::default()));
    let _ = registry
        .get_mut()
        .register(DeltaRecorder::new(total.clone(), calls.clone()));
    let (handler, updates, _): (TickHandlerRc, Rc<Cell<u32>>, Rc<Cell<u32>>) =
        CountingHandler::spawn();
    let config: SchedulerConfig = SchedulerConfig::default();
    let mut state: SchedulerState = SchedulerState::default();
    state.tick(&config, &handler, Some(&registry), None);
    assert_eq!(
        calls.get() as usize,
        updates.get() as usize,
        "tasks must be driven exactly once per handler on_update call"
    );
    assert_eq!(calls.get(), state.get_update_count() as u32);
    assert_eq!(state.get_frame_count(), 1);
    assert!(
        (total.get() - config.get_fixed_timestep()).abs() < 1e-12,
        "task must receive the fixed timestep, got {}",
        total.get()
    );
}

#[test]
fn tick_without_registry_still_runs() {
    let (handler, updates, renders): (TickHandlerRc, Rc<Cell<u32>>, Rc<Cell<u32>>) =
        CountingHandler::spawn();
    let config: SchedulerConfig = SchedulerConfig::default();
    let mut state: SchedulerState = SchedulerState::new(-1.0);
    for _ in 0..4 {
        state.tick(&config, &handler, None, None);
    }
    assert_eq!(updates.get(), state.get_update_count() as u32);
    assert_eq!(renders.get(), 4);
}
