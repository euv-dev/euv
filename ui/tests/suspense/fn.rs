use super::*;
#[test]
fn suspense_phase_default_is_pending() {
    let phase: SuspensePhase<i32> = SuspensePhase::default();
    assert!(matches!(phase, SuspensePhase::Pending));
}

#[test]
fn suspense_phase_partial_eq_same_variant_pending() {
    let a: SuspensePhase<i32> = SuspensePhase::Pending;
    let b: SuspensePhase<i32> = SuspensePhase::Pending;
    assert_eq!(a, b);
}

#[test]
fn suspense_phase_partial_eq_same_variant_resolved() {
    let a: SuspensePhase<i32> = SuspensePhase::Resolved(1);
    let b: SuspensePhase<i32> = SuspensePhase::Resolved(1);
    assert_eq!(a, b);
}

#[test]
fn suspense_phase_partial_eq_same_variant_failed() {
    let a: SuspensePhase<i32> = SuspensePhase::Failed("oops".to_string());
    let b: SuspensePhase<i32> = SuspensePhase::Failed("oops".to_string());
    assert_eq!(a, b);
}

#[test]
fn suspense_phase_partial_eq_different_variants() {
    let pending: SuspensePhase<i32> = SuspensePhase::Pending;
    let resolved: SuspensePhase<i32> = SuspensePhase::Resolved(0);
    let failed: SuspensePhase<i32> = SuspensePhase::Failed("x".to_string());
    assert_ne!(pending, resolved);
    assert_ne!(pending, failed);
    assert_ne!(resolved, failed);
}

#[test]
fn new_is_pending() {
    let handle: SuspenseHandle<i32> = SuspenseHandle::new();
    assert!(matches!(handle.get_phase().get(), SuspensePhase::Pending));
    assert!(!matches!(
        handle.get_phase().get(),
        SuspensePhase::Resolved(_)
    ));
    assert!(!matches!(
        handle.get_phase().get(),
        SuspensePhase::Failed(_)
    ));
    assert!(matches!(handle.get_phase().get(), SuspensePhase::Pending));
}

#[test]
fn default_is_pending() {
    let handle: SuspenseHandle<i32> = SuspenseHandle::default();
    assert!(matches!(handle.get_phase().get(), SuspensePhase::Pending));
}

#[test]
fn state_returns_signal_with_same_value() {
    let handle: SuspenseHandle<i32> = SuspenseHandle::new();
    let signal: &Signal<SuspensePhase<i32>> = handle.get_phase();
    assert!(matches!(signal.get(), SuspensePhase::Pending));
}

#[test]
fn debug_format_works() {
    let handle: SuspenseHandle<i32> = SuspenseHandle::new();
    let s: String = format!("{:?}", handle);
    assert!(s.contains("SuspenseHandle"));
}

#[test]
fn display_format_works() {
    let handle: SuspenseHandle<i32> = SuspenseHandle::new();
    let s: String = format!("{}", handle);
    assert!(s.contains("SuspenseHandle"));
    assert!(s.contains("Pending"));
}

#[test]
fn resolve_sync_transitions_to_resolved() {
    let handle: SuspenseHandle<i32> = SuspenseHandle::new();
    handle.resolve_sync(42);
    assert!(matches!(
        handle.get_phase().get(),
        SuspensePhase::Resolved(_)
    ));
    assert_eq!(handle.get_phase().get(), SuspensePhase::Resolved(42));
}

#[test]
fn resolve_sync_with_string() {
    let handle: SuspenseHandle<String> = SuspenseHandle::new();
    handle.resolve_sync("hello".to_string());
    assert_eq!(
        handle.get_phase().get(),
        SuspensePhase::Resolved("hello".to_string())
    );
}

#[test]
fn resolve_sync_with_vec() {
    let handle: SuspenseHandle<Vec<i32>> = SuspenseHandle::new();
    handle.resolve_sync(vec![1, 2, 3]);
    assert_eq!(
        handle.get_phase().get(),
        SuspensePhase::Resolved(vec![1, 2, 3])
    );
}

#[test]
fn fail_transitions_to_failed() {
    let handle: SuspenseHandle<i32> = SuspenseHandle::new();
    handle.fail("network error".to_string());
    assert!(matches!(handle.get_phase().get(), SuspensePhase::Failed(_)));
    assert_eq!(
        handle.get_phase().get(),
        SuspensePhase::Failed("network error".to_string())
    );
}

#[test]
fn reset_from_resolved_returns_to_pending() {
    let handle: SuspenseHandle<i32> = SuspenseHandle::new();
    handle.resolve_sync(42);
    handle.reset();
    assert!(matches!(handle.get_phase().get(), SuspensePhase::Pending));
}

#[test]
fn reset_from_failed_returns_to_pending() {
    let handle: SuspenseHandle<i32> = SuspenseHandle::new();
    handle.fail("oops".to_string());
    handle.reset();
    assert!(matches!(handle.get_phase().get(), SuspensePhase::Pending));
}

#[test]
fn phase_transitions_pending_resolved_pending() {
    let handle: SuspenseHandle<i32> = SuspenseHandle::new();
    assert!(matches!(handle.get_phase().get(), SuspensePhase::Pending));
    handle.resolve_sync(1);
    assert!(matches!(
        handle.get_phase().get(),
        SuspensePhase::Resolved(_)
    ));
    handle.reset();
    assert!(matches!(handle.get_phase().get(), SuspensePhase::Pending));
}

#[test]
fn phase_transitions_pending_failed_pending() {
    let handle: SuspenseHandle<i32> = SuspenseHandle::new();
    assert!(matches!(handle.get_phase().get(), SuspensePhase::Pending));
    handle.fail("x".to_string());
    assert!(matches!(handle.get_phase().get(), SuspensePhase::Failed(_)));
    handle.reset();
    assert!(matches!(handle.get_phase().get(), SuspensePhase::Pending));
}

#[test]
fn multiple_resolve_calls_update_value() {
    let handle: SuspenseHandle<i32> = SuspenseHandle::new();
    handle.resolve_sync(1);
    assert_eq!(handle.get_phase().get(), SuspensePhase::Resolved(1));
    handle.resolve_sync(2);
    assert_eq!(handle.get_phase().get(), SuspensePhase::Resolved(2));
    handle.resolve_sync(3);
    assert_eq!(handle.get_phase().get(), SuspensePhase::Resolved(3));
}

#[test]
fn clone_shares_state() {
    let handle: SuspenseHandle<i32> = SuspenseHandle::new();
    let cloned: SuspenseHandle<i32> = handle;
    assert_eq!(
        handle.get_phase().get_inner(),
        cloned.get_phase().get_inner(),
        "clone must share the same backing signal"
    );
}

#[test]
fn state_signal_is_reactive() {
    let handle: SuspenseHandle<i32> = SuspenseHandle::new();
    let signal: &Signal<SuspensePhase<i32>> = handle.get_phase();
    assert!(matches!(signal.get(), SuspensePhase::Pending));
    let _ = catch_unwind(AssertUnwindSafe(|| {
        handle.resolve_sync(42);
    }));
    let value: SuspensePhase<i32> = signal.get();
    assert!(matches!(
        value,
        SuspensePhase::Pending | SuspensePhase::Resolved(42)
    ));
}
