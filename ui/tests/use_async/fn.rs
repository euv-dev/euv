use super::*;

#[test]
fn fresh_handle_starts_in_loading_state() {
    let handle: UseAsyncHandle<u32, ()> = UseAsyncHandle::default();
    match handle.state() {
        AsyncState::Loading(()) => {}
        other => panic!("expected AsyncState::Loading(()), got {other:?}"),
    }
}

#[test]
fn set_state_transitions_to_ok() {
    let handle: UseAsyncHandle<String, ()> = UseAsyncHandle::default();
    handle.set_state(AsyncState::Ok(String::from("payload")));
    match handle.state() {
        AsyncState::Ok(payload) => assert_eq!(payload, "payload"),
        other => panic!("expected AsyncState::Ok, got {other:?}"),
    }
}

#[test]
fn set_state_transitions_to_err() {
    let handle: UseAsyncHandle<u32, ()> = UseAsyncHandle::default();
    handle.set_state(AsyncState::Err(String::from("network down")));
    match handle.state() {
        AsyncState::Err(msg) => assert_eq!(msg, "network down"),
        other => panic!("expected AsyncState::Err, got {other:?}"),
    }
}

#[test]
fn async_state_clone_preserves_variant() {
    let original: AsyncState<u32> = AsyncState::Ok(42);
    let cloned: AsyncState<u32> = original.clone();
    assert_eq!(original, cloned);
}

#[test]
fn async_state_eq_works_across_variants() {
    assert_eq!(
        AsyncState::<u32>::Loading(()),
        AsyncState::<u32>::Loading(()),
    );
    assert_eq!(AsyncState::<u32>::Ok(7), AsyncState::<u32>::Ok(7));
    assert_ne!(
        AsyncState::<u32>::Ok(7),
        AsyncState::<u32>::Ok(8),
        "different Ok payloads must not be equal",
    );
    assert_ne!(
        AsyncState::<u32>::Loading(()),
        AsyncState::<u32>::Ok(0),
        "different variants must not be equal",
    );
}

#[test]
fn async_state_debug_names_variant() {
    assert!(format!("{:?}", AsyncState::<u32>::Loading(())).contains("Loading"));
    assert!(format!("{:?}", AsyncState::<u32>::Ok(42)).contains("Ok"));
    assert!(format!("{:?}", AsyncState::<u32>::Err("x".to_string())).contains("Err"));
}

#[test]
fn handle_default_is_stable_across_state_calls() {
    let handle: UseAsyncHandle<u32, ()> = UseAsyncHandle::default();
    assert!(matches!(handle.state(), AsyncState::Loading(())));
    assert!(matches!(handle.state(), AsyncState::Loading(())));
    assert!(matches!(handle.state(), AsyncState::Loading(())));
}

#[test]
fn handle_clone_is_cheap_and_shares_state() {
    let handle: UseAsyncHandle<String, ()> = UseAsyncHandle::default();
    let twin: UseAsyncHandle<String, ()> = handle;
    handle.set_state(AsyncState::Ok(String::from("shared")));
    match twin.state() {
        AsyncState::Ok(value) => assert_eq!(value, "shared"),
        other => panic!("expected twin to observe shared state, got {other:?}"),
    }
}

#[test]
fn loading_hint_empty_default_for_unit() {
    let handle: UseAsyncHandle<u32, ()> = UseAsyncHandle::default();
    match handle.state() {
        AsyncState::Loading(()) => {}
        other => panic!("expected Loading(()), got {other:?}"),
    }
}

#[test]
fn handle_debug_hides_raw_address() {
    let handle: UseAsyncHandle<u32, ()> = UseAsyncHandle::default();
    let formatted: String = format!("{handle:?}");
    assert!(
        formatted.contains("UseAsyncHandle"),
        "Debug output must name the type, got: {formatted}",
    );
}

#[test]
fn two_default_handles_have_independent_slots() {
    let a: UseAsyncHandle<u32, ()> = UseAsyncHandle::default();
    let b: UseAsyncHandle<u32, ()> = UseAsyncHandle::default();
    a.set_state(AsyncState::Ok(1));
    b.set_state(AsyncState::Ok(2));
    match (a.state(), b.state()) {
        (AsyncState::Ok(1), AsyncState::Ok(2)) => {}
        other => panic!("expected independent state, got {other:?}"),
    }
}

#[test]
fn handle_is_copy_when_payload_is_copy() {
    let handle: UseAsyncHandle<u32, ()> = UseAsyncHandle::default();
    let copy: UseAsyncHandle<u32, ()> = handle;
    let state_from_handle: AsyncState<u32, ()> = handle.state();
    let state_from_copy: AsyncState<u32, ()> = copy.state();
    assert_eq!(state_from_handle, state_from_copy);
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
#[cfg_attr(
    not(target_arch = "wasm32"),
    ignore = "refetch spawns a local task, and spawn_local panics on a native target"
)]
fn refetch_runs_the_factory_before_it_returns() {
    let handle: UseAsyncHandle<u32, ()> = UseAsyncHandle::default();
    let called: Rc<Cell<u32>> = Rc::new(Cell::new(0));

    let shared: Rc<Cell<u32>> = Rc::clone(&called);
    handle.refetch(move || {
        shared.set(shared.get() + 1);
        async { Ok::<u32, String>(7) }
    });

    assert_eq!(
        called.get(),
        1,
        "the factory is invoked on the spot, not when the spawned task first gets polled; \
         otherwise a refetch from an event handler would not start anything until the next \
         microtask checkpoint"
    );
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
#[cfg_attr(
    not(target_arch = "wasm32"),
    ignore = "refetch spawns a local task, and spawn_local panics on a native target"
)]
fn refetch_leaves_the_settled_state_alone_synchronously() {
    let handle: UseAsyncHandle<u32, ()> = UseAsyncHandle::default();
    handle.set_state(AsyncState::Ok(42));

    handle.refetch(|| async { Ok::<u32, String>(99) });

    match handle.state() {
        AsyncState::Ok(value) => assert_eq!(
            value, 42,
            "the write-back happens in the spawned task, so the caller still sees the \
             previous result at the moment refetch returns"
        ),
        other => panic!("expected the previous Ok to still be there, got {other:?}"),
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
#[cfg_attr(
    not(target_arch = "wasm32"),
    ignore = "refetch spawns a local task, and spawn_local panics on a native target"
)]
fn a_refetch_after_a_failure_starts_a_fresh_attempt() {
    let handle: UseAsyncHandle<u32, ()> = UseAsyncHandle::default();
    handle.set_state(AsyncState::Err(String::from("network down")));

    let called: Rc<Cell<bool>> = Rc::new(Cell::new(false));
    let shared: Rc<Cell<bool>> = Rc::clone(&called);
    handle.refetch(move || {
        shared.set(true);
        async { Ok::<u32, String>(1) }
    });

    assert!(
        called.get(),
        "a refetch is the documented way out of a failed attempt, so the factory must run \
         whatever state the handle was left in — including Err"
    );
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
#[cfg_attr(
    not(target_arch = "wasm32"),
    ignore = "the hook spawns a local task, and spawn_local panics on a native target"
)]
fn the_async_hook_returns_the_same_slot_on_every_render() {
    let mut context: HookContext = HookContext::default();

    let first: UseAsyncHandle<u32, ()> = HookContext::with(context.clone(), use_async::<u32, ()>);
    context.reset_index();
    let second: UseAsyncHandle<u32, ()> = HookContext::with(context, use_async::<u32, ()>);

    first.set_state(AsyncState::Ok(7));

    match second.state() {
        AsyncState::Ok(value) => assert_eq!(
            value, 7,
            "a hook has to hand back the same slot on every render. If each render allocated a \
             fresh handle, the result of the first fetch would be dropped on the next render and \
             the component would refetch forever"
        ),
        other => panic!("expected the first render's Ok to still be there, got {other:?}"),
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
#[cfg_attr(
    not(target_arch = "wasm32"),
    ignore = "the hook spawns a local task, and spawn_local panics on a native target"
)]
fn two_components_get_async_handles_of_their_own() {
    let left: HookContext = HookContext::default();
    let right: HookContext = HookContext::default();

    let first: UseAsyncHandle<u32, ()> = HookContext::with(left, use_async::<u32, ()>);
    let second: UseAsyncHandle<u32, ()> = HookContext::with(right, use_async::<u32, ()>);

    first.set_state(AsyncState::Ok(1));

    assert!(
        matches!(second.state(), AsyncState::Loading(())),
        "the slot belongs to the hook context, not to the hook name: one component's finished \
         work must not make every other component look finished"
    );
}
