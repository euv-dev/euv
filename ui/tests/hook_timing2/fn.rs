use super::*;

fn debounced(delay_ms: u32) -> DebouncedValue<u32> {
    HookContext::with(HookContext::default(), || use_debounced_value(delay_ms))
}

fn throttled(interval_ms: u32) -> ThrottledValue<u32> {
    HookContext::with(HookContext::default(), || use_throttled_value(interval_ms))
}

#[test]
fn a_fresh_debounced_value_is_idle_with_the_default_value() {
    let value: DebouncedValue<u32> = debounced(100);
    assert_eq!(value.get(), 0, "a value nobody set reads as its Default");
    assert!(
        !value.is_pending(),
        "nothing has been written, so nothing is pending"
    );
    assert!(!value.tick(1_000), "ticking an idle value must do nothing");
}

#[test]
fn a_debounced_value_does_not_emit_before_its_delay_elapses() {
    let value: DebouncedValue<u32> = debounced(100);
    value.set(5, 1_000);
    assert!(value.is_pending(), "a write before the delay is pending");
    assert!(
        !value.tick(1_050),
        "half the delay has passed, so the value must still be withheld"
    );
    assert_eq!(value.get(), 0, "and the exposed value must not have moved");
}

#[test]
fn a_debounced_value_emits_once_the_delay_elapses() {
    let value: DebouncedValue<u32> = debounced(100);
    value.set(5, 1_000);
    assert!(value.tick(1_100), "the delay has exactly elapsed");
    assert_eq!(value.get(), 5, "the pending value must now be visible");
    assert!(!value.is_pending(), "emitting clears the pending state");
}

#[test]
fn a_debounced_value_keeps_only_the_latest_write() {
    let value: DebouncedValue<u32> = debounced(100);
    value.set(1, 0);
    value.set(2, 20);
    value.set(3, 40);
    assert!(value.tick(140), "the newest write is what eventually lands");
    assert_eq!(
        value.get(),
        3,
        "an intermediate keystroke must never overwrite the latest one"
    );
}

#[test]
fn cancelling_a_debounced_value_drops_the_pending_write() {
    let value: DebouncedValue<u32> = debounced(100);
    value.set(5, 0);
    value.cancel();
    assert!(!value.is_pending());
    assert!(
        !value.tick(1_000),
        "a cancelled write must never fire, however long you wait"
    );
    assert_eq!(value.get(), 0);
}

#[test]
fn a_fresh_throttled_value_is_idle() {
    let value: ThrottledValue<u32> = throttled(100);
    assert_eq!(value.get(), 0);
    assert!(!value.is_throttling(), "a throttle starts idle");
    assert!(!value.tick(1_000));
}

#[test]
fn a_throttled_value_passes_the_first_write_straight_through() {
    let value: ThrottledValue<u32> = throttled(100);
    value.set(5, 1_000);
    assert_eq!(
        value.get(),
        5,
        "throttling damps repeats, it must not delay the first one"
    );
    assert!(value.is_throttling(), "the first write opens the cooldown");
}

#[test]
fn a_write_inside_the_cooldown_is_withheld_then_committed() {
    let value: ThrottledValue<u32> = throttled(100);
    value.set(5, 1_000);
    value.set(9, 1_020);
    assert_eq!(value.get(), 5, "a write inside the window must be withheld");
    assert!(
        !value.tick(1_050),
        "50ms into a 100ms window is still inside"
    );
    assert!(
        value.tick(1_100),
        "once the window elapses the last withheld write is committed"
    );
    assert_eq!(value.get(), 9);
    assert!(
        !value.is_throttling(),
        "committing must return the throttle to idle"
    );
}

#[test]
fn a_write_inside_the_cooldown_is_dropped_when_nothing_else_arrives() {
    let value: ThrottledValue<u32> = throttled(100);
    value.set(5, 1_000);
    value.set(9, 1_020);
    value.cancel();
    assert!(
        !value.tick(1_200),
        "a cancelled throttle must commit nothing, however long you wait"
    );
    assert_eq!(value.get(), 5, "the last committed value stands");
}

#[test]
fn a_zero_interval_throttle_passes_everything_through() {
    let value: ThrottledValue<u32> = throttled(0);
    value.set(1, 0);
    value.set(2, 1);
    assert_eq!(
        value.get(),
        2,
        "a zero interval means no throttling at all, so nothing may be withheld"
    );
    assert!(!value.is_throttling());
}

#[test]
fn a_ticking_cooldown_with_nothing_pending_just_lapses() {
    let value: ThrottledValue<u32> = throttled(100);
    value.set(5, 1_000);
    assert!(
        !value.tick(1_200),
        "nothing was withheld, so the tick commits nothing"
    );
    assert!(
        !value.is_throttling(),
        "but the cooldown must still lapse, or the throttle jams forever"
    );
}

#[test]
fn an_error_boundary_reports_a_message_and_keeps_it() {
    let boundary: ErrorBoundary = ErrorBoundary::default();
    let returned: String = boundary.report_error("boom");
    assert_eq!(
        returned, "boom",
        "reporting must hand the message straight back"
    );
    assert!(
        format!("{boundary}").contains("boom"),
        "the boundary must carry the message, so the view can render it; got {boundary}"
    );
}

#[test]
fn a_reset_error_boundary_returns_to_healthy() {
    let boundary: ErrorBoundary = ErrorBoundary::default();
    assert!(
        format!("{boundary}").contains("Healthy"),
        "a fresh boundary is healthy"
    );
    boundary.report_error("boom");
    let caught: String = format!("{boundary}");
    assert!(
        !caught.contains("Healthy"),
        "reporting must move the boundary out of healthy, got: {caught}"
    );
    boundary.reset();
    assert!(
        format!("{boundary}").contains("Healthy"),
        "reset must return the boundary to the state a retry needs, got: {boundary}"
    );
}
