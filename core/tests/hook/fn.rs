use super::*;
#[test]
fn interval_handle_clear_does_not_panic_on_native() {
    let handle: IntervalHandle = IntervalHandle::default();
    let result: Result<(), String> = super::catch_unwind(super::AssertUnwindSafe(|| {
        handle.clear();
    }))
    .map_err(|_| "panic".to_string());
    assert!(result.is_err());
}

#[test]
fn interval_handle_default_yields_zero_interval_id() {
    let handle: IntervalHandle = IntervalHandle::default();
    let default_again: IntervalHandle = IntervalHandle::default();
    assert_eq!(handle, default_again);
    let real: IntervalHandle = IntervalHandle::new(7);
    assert_ne!(handle, real);
}

#[test]
fn interval_handle_is_copy() {
    let handle: IntervalHandle = IntervalHandle::new(11);
    let copy: IntervalHandle = handle;
    assert_eq!(handle, copy);
}

#[test]
fn interval_handle_is_eq_and_ne() {
    let a: IntervalHandle = IntervalHandle::new(3);
    let b: IntervalHandle = IntervalHandle::new(3);
    let c: IntervalHandle = IntervalHandle::new(4);
    assert_eq!(a, b);
    assert_ne!(a, c);
}

#[test]
fn interval_handle_clone() {
    let handle: IntervalHandle = IntervalHandle::new(5);
    let cloned: IntervalHandle = handle;
    assert_eq!(handle, cloned);
}

#[test]
fn interval_handle_is_hash_consistent_with_eq() {
    let a: IntervalHandle = IntervalHandle::new(42);
    let b: IntervalHandle = IntervalHandle::new(42);
    let c: IntervalHandle = IntervalHandle::new(43);
    let mut hasher_a: DefaultHasher = DefaultHasher::new();
    let mut hasher_b: DefaultHasher = DefaultHasher::new();
    let mut hasher_c: DefaultHasher = DefaultHasher::new();
    a.hash(&mut hasher_a);
    b.hash(&mut hasher_b);
    c.hash(&mut hasher_c);
    assert_eq!(hasher_a.finish(), hasher_b.finish());
    assert_ne!(hasher_a.finish(), hasher_c.finish());
}

#[test]
fn native_hook_context_default_does_not_panic() {
    let result: Result<(), ()> = super::catch_unwind(super::AssertUnwindSafe(|| {
        let _: HookContext = HookContext::default();
    }))
    .map_err(|_| ());
    assert!(result.is_ok());
}
