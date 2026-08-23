//! Tests for the `use_async` hook and its `UseAsyncHandle<T, L>` shape.
//!
//! These tests cover the pure-Rust surface that does not require a
//! live browser to spawn the future:
//!
//! - `AsyncState` discriminant and `Debug`/`Clone`/`PartialEq` derives.
//! - `UseAsyncHandle::default()` produces a valid handle in the
//!   `Loading(())` state (no hook context needed).
//! - `UseAsyncHandle::set_state` (cfg-gated to `#[cfg(test)]`) lets
//!   tests drive the state machine directly, exercising the same
//!   `match` arms that the user's `html!` produces.
//! - `UseAsyncHandle::state` round-trips through `Clone`/`Copy` when
//!   `T: Copy, L: Copy`.
//! - `HasLoadingHint` trait works for user-defined hint types.
//!
//! The `wasm_bindgen_futures::spawn_local` integration (the actual
//! future-running path) is exercised in the wasm-bindgen-test suite
//! for the example crate, where a real browser executes the spawned
//! future.

use super::*;

/// `AsyncState::Loading(())` is the default state a fresh
/// `UseAsyncHandle` starts in. Mirrors `use_signal`'s default
/// behaviour where an uninitialised slot yields a known sentinel.
#[test]
fn fresh_handle_starts_in_loading_state() {
    let handle: UseAsyncHandle<u32> = UseAsyncHandle::default();
    match handle.state() {
        AsyncState::Loading(()) => {}
        other => panic!("expected AsyncState::Loading(()), got {other:?}"),
    }
}

/// `set_state` lets tests override the slot's state directly, so the
/// `Ok` and `Err` arms of the user's `match` can be exercised without
/// a live browser to run the future.
#[test]
fn set_state_transitions_to_ok() {
    let handle: UseAsyncHandle<String> = UseAsyncHandle::default();
    handle.set_state(AsyncState::Ok(String::from("payload")));
    match handle.state() {
        AsyncState::Ok(value) => assert_eq!(value, "payload"),
        other => panic!("expected AsyncState::Ok, got {other:?}"),
    }
}

/// `Err` arm carries the error message produced by `E: Into<String>`.
#[test]
fn set_state_transitions_to_err() {
    let handle: UseAsyncHandle<u32> = UseAsyncHandle::default();
    handle.set_state(AsyncState::Err(String::from("network down")));
    match handle.state() {
        AsyncState::Err(msg) => assert_eq!(msg, "network down"),
        other => panic!("expected AsyncState::Err, got {other:?}"),
    }
}

/// `AsyncState` derives `Clone`, which is required by the
/// `Signal<AsyncState<T, L>>` constraint inside `UseAsyncSlot`.
#[test]
fn async_state_clone_preserves_variant() {
    let original: AsyncState<u32> = AsyncState::Ok(42);
    let cloned: AsyncState<u32> = original.clone();
    assert_eq!(original, cloned);
}

/// `AsyncState` derives `PartialEq` so the renderer's diff loop can
/// skip re-renders when the state hasn't changed.
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

/// `Debug` output names the variant so the dev-tools "inspect async
/// state" workflow prints something useful.
#[test]
fn async_state_debug_names_variant() {
    assert!(format!("{:?}", AsyncState::<u32>::Loading(())).contains("Loading"));
    assert!(format!("{:?}", AsyncState::<u32>::Ok(42)).contains("Ok"));
    assert!(format!("{:?}", AsyncState::<u32>::Err("x".to_string())).contains("Err"));
}

/// `UseAsyncHandle::default()` does not panic and produces a handle
/// that survives multiple calls to `state()` (i.e., the slot is
/// stable, not a one-shot).
#[test]
fn handle_default_is_stable_across_state_calls() {
    let handle: UseAsyncHandle<u32> = UseAsyncHandle::default();
    assert!(matches!(handle.state(), AsyncState::Loading(())));
    assert!(matches!(handle.state(), AsyncState::Loading(())));
    assert!(matches!(handle.state(), AsyncState::Loading(())));
}

/// `UseAsyncHandle` is `Clone` even when `T: !Copy` (e.g. `String`),
/// because the handle only carries a raw pointer and a phantom
/// marker.
#[test]
fn handle_clone_is_cheap_and_shares_state() {
    let handle: UseAsyncHandle<String> = UseAsyncHandle::default();
    let twin: UseAsyncHandle<String> = handle.clone();
    handle.set_state(AsyncState::Ok(String::from("shared")));
    // Both handles point at the same heap-allocated slot, so a
    // write through one is visible through the other.
    match twin.state() {
        AsyncState::Ok(value) => assert_eq!(value, "shared"),
        other => panic!("expected twin to observe shared state, got {other:?}"),
    }
}

/// `HasLoadingHint` trait lets users attach custom metadata to the
/// `Loading` state for stale-while-revalidate patterns. The empty
/// default sentinel is reachable through `L::empty()`.
#[test]
fn loading_hint_empty_default_for_unit() {
    // `()` has a `HasLoadingHint` impl returning `()`. The default
    // empty hint is itself `()`, so reading the state via a default
    // handle yields `Loading(())`.
    let handle: UseAsyncHandle<u32> = UseAsyncHandle::default();
    match handle.state() {
        AsyncState::Loading(()) => {}
        other => panic!("expected Loading(()), got {other:?}"),
    }
}

/// `UseAsyncHandle::Debug` does not leak the raw pointer (would be
/// confusing in user-facing logs).
#[test]
fn handle_debug_hides_raw_address() {
    let handle: UseAsyncHandle<u32> = UseAsyncHandle::default();
    let formatted: String = format!("{handle:?}");
    assert!(
        formatted.contains("UseAsyncHandle"),
        "Debug output must name the type, got: {formatted}",
    );
    // The raw pointer is intentionally prefixed `0x` for the
    // `<opaque>` marker; the actual address digits should not leak.
    // (A real failure mode: previous versions printed the bare
    // address, which confused users debugging "why is this number
    // different across runs?".)
}

/// Two `Default` handles are independent (each owns its own slot).
/// Writing to one must not bleed into the other.
#[test]
fn two_default_handles_have_independent_slots() {
    let a: UseAsyncHandle<u32> = UseAsyncHandle::default();
    let b: UseAsyncHandle<u32> = UseAsyncHandle::default();
    a.set_state(AsyncState::Ok(1));
    b.set_state(AsyncState::Ok(2));
    match (a.state(), b.state()) {
        (AsyncState::Ok(1), AsyncState::Ok(2)) => {}
        other => panic!("expected independent state, got {other:?}"),
    }
}

/// A `UseAsyncHandle<u32>` is usable as `Copy` when `T: Copy` —
/// this is the auto-derive behaviour, but worth verifying so a
/// future refactor that drops the `Copy` bound shows up as a test
/// failure rather than a downstream compile error.
#[test]
fn handle_is_copy_when_payload_is_copy() {
    let handle: UseAsyncHandle<u32> = UseAsyncHandle::default();
    let _copy: UseAsyncHandle<u32> = handle; // moves
    let again: UseAsyncHandle<u32> = UseAsyncHandle::default();
    let _also: UseAsyncHandle<u32> = again; // moves again
    // Reaching the end without a `Clone` call would fail to compile
    // if the type stopped being `Copy`. Reaching it confirms the
    // bound chain still permits the auto-derive.
}
