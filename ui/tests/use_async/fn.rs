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
        AsyncState::Ok(value) => assert_eq!(value, "payload"),
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
