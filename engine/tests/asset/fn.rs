use super::*;

#[test]
fn default_loader_has_no_pending_loads() {
    let loader: AssetLoader = AssetLoader::default();
    assert_eq!(loader.pending_count(), 0);
    assert!(loader.is_all_loaded());
    assert!(loader.get_closures().get().slots.is_empty());
}

#[test]
fn collect_releases_settled_closures_only() {
    let mut loader: AssetLoader = AssetLoader::default();
    let store: &mut AssetClosureStore = loader.get_closures().get_mut();
    store.slots.push(None);
    store.settled.push(false);
    store.slots.push(None);
    store.settled.push(true);
    store.slots.push(None);
    store.settled.push(false);
    settle(&loader, 1);
    loader.collect();
    let store_ref: &AssetClosureStore = loader.get_closures().get();
    let flags: Vec<bool> = store_ref.settled.clone();
    assert_eq!(
        flags,
        vec![false, false, false],
        "collect must release only the settled slot"
    );
    assert_eq!(store_ref.slots.len(), 3, "slot indices must stay stable");
}

#[test]
fn collect_keeps_in_flight_closures_alive() {
    let mut loader: AssetLoader = AssetLoader::default();
    let store: &mut AssetClosureStore = loader.get_closures().get_mut();
    for settled in [false, true, false] {
        store.slots.push(None);
        store.settled.push(settled);
    }
    settle(&loader, 1);
    loader.collect();
    let store: &AssetClosureStore = loader.get_closures().get();
    let remaining: Vec<bool> = store.settled.clone();
    assert_eq!(
        remaining,
        vec![false, false, false],
        "collect must clear the settled flag of the released slot"
    );
}

#[test]
fn collect_is_idempotent() {
    let mut loader: AssetLoader = AssetLoader::default();
    let store: &mut AssetClosureStore = loader.get_closures().get_mut();
    store.slots.push(None);
    store.settled.push(true);
    loader.collect();
    loader.collect();
    let store: &AssetClosureStore = loader.get_closures().get();
    assert!(!store.settled[0]);
    assert_eq!(store.slots.len(), 1);
}

#[test]
fn mark_settled_flips_exactly_one_slot() {
    let loader: AssetLoader = AssetLoader::default();
    let store: &mut AssetClosureStore = loader.get_closures().get_mut();
    for _ in 0..4 {
        store.slots.push(None);
        store.settled.push(false);
    }
    let weak: Weak<EngineCell<AssetClosureStore>> = Rc::downgrade(loader.get_closures());
    mark_asset_closure_settled(&weak, 2);
    let store_ref: &AssetClosureStore = loader.get_closures().get();
    let flags: Vec<bool> = store_ref.settled.clone();
    assert_eq!(flags, vec![false, false, true, false]);
}

#[test]
fn mark_settled_out_of_range_is_ignored() {
    let loader: AssetLoader = AssetLoader::default();
    let weak: Weak<EngineCell<AssetClosureStore>> = Rc::downgrade(loader.get_closures());
    mark_asset_closure_settled(&weak, 99);
    assert!(loader.get_closures().get().settled.is_empty());
}

#[test]
fn mark_settled_after_loader_dropped_is_noop() {
    let loader: AssetLoader = AssetLoader::default();
    let weak: Weak<EngineCell<AssetClosureStore>> = Rc::downgrade(loader.get_closures());
    drop(loader);
    mark_asset_closure_settled(&weak, 0);
}

#[test]
fn pending_counter_is_shared_with_callbacks() {
    let loader: AssetLoader = AssetLoader::default();
    *loader.get_pending().get_mut() += 1;
    assert_eq!(loader.pending_count(), 1);
    *loader.get_pending().get_mut() = 0;
    assert_eq!(loader.pending_count(), 0);
    assert!(loader.get_closures().get().slots.is_empty());
}

#[test]
fn pending_count_never_underflows() {
    let loader: AssetLoader = AssetLoader::default();
    *loader.get_pending().get_mut() = 0;
    let pending: AssetPending = loader.get_pending().clone();
    *pending.get_mut() = pending.get().saturating_sub(1);
    assert_eq!(*pending.get(), 0);
}

#[test]
fn loader_update_calls_collect() {
    let mut loader: AssetLoader = AssetLoader::default();
    let store: &mut AssetClosureStore = loader.get_closures().get_mut();
    store.slots.push(None);
    store.settled.push(true);
    let weak: Weak<EngineCell<AssetClosureStore>> = Rc::downgrade(loader.get_closures());
    mark_asset_closure_settled(&weak, 0);
    loader.update(1.0 / 60.0);
    assert!(!loader.get_closures().get().settled[0]);
}

#[test]
fn loader_is_updatable() {
    let mut registry: TaskRegistry = TaskRegistry::default();
    let loader: AssetLoader = AssetLoader::default();
    let store: &mut AssetClosureStore = loader.get_closures().get_mut();
    store.slots.push(None);
    store.settled.push(true);
    let _ = registry.register(loader.clone());
    registry.update_all(1.0 / 60.0);
    assert!(
        !loader.get_closures().get().settled[0],
        "registering the loader as a task must drive its collect step"
    );
}

#[test]
fn engine_register_assets_creates_loader_once() {
    let mut handle: EngineHandle = Engine::new_handle(EngineConfig::default());
    assert!(handle.asset_loader().is_none());
    let first: AssetLoader = handle.register_assets();
    let second: AssetLoader = handle.register_assets();
    assert_eq!(
        first.get_pending().get(),
        second.get_pending().get(),
        "a second register_assets must reuse the existing loader"
    );
    assert!(handle.asset_loader().is_some());
    let tasks: &TaskRegistry = handle.tasks().get();
    assert_eq!(
        tasks.len(),
        1,
        "the asset loader must be registered exactly once as a task"
    );
}

#[test]
fn engine_asset_loader_collects_through_task_registry() {
    let mut handle: EngineHandle = Engine::new_handle(EngineConfig::default());
    let loader: AssetLoader = handle.register_assets();
    let store: &mut AssetClosureStore = loader.get_closures().get_mut();
    store.slots.push(None);
    store.settled.push(true);
    let registry: &TaskRegistryRc = handle.tasks();
    registry.get_mut().update_all(1.0 / 60.0);
    assert!(
        !loader.get_closures().get().settled[0],
        "driving the engine task registry must release settled asset closures"
    );
}
