use super::*;

/// Helper body of the `seed_throttled` free function.
///
/// # Arguments
///
/// - `&ThrottledValue<T>` - Shared reference to a `ThrottledValue<T>`.
/// - `T: Clone + PartialEq + Default + 'static` - A generic type parameter.
/// - `u64` - The current time in milliseconds.
fn seed_throttled<T: Clone + PartialEq + Default + 'static>(
    throttled: &ThrottledValue<T>,
    initial: T,
    now_ms: u64,
) {
    throttled.set(initial, now_ms);
}

#[test]
fn throttled_value_starts_at_default() {
    let throttled: ThrottledValue<i32> = ThrottledValue::new(100);
    assert_eq!(throttled.get(), 0);
    assert!(!throttled.is_throttling());
}

#[test]
fn throttled_value_set_when_idle_emits_immediately() {
    let throttled: ThrottledValue<i32> = ThrottledValue::new(100);
    throttled.set(5, 1_000);
    assert_eq!(throttled.get(), 5);
    assert!(throttled.is_throttling());
}

#[test]
fn throttled_value_set_during_cooldown_buffers_pending() {
    let throttled: ThrottledValue<i32> = ThrottledValue::new(100);
    throttled.set(5, 1_000);
    throttled.set(7, 1_010);
    assert_eq!(throttled.get(), 5);
}

#[test]
fn throttled_value_tick_during_cooldown_keeps_state() {
    let throttled: ThrottledValue<i32> = ThrottledValue::new(100);
    throttled.set(5, 1_000);
    let committed: bool = throttled.tick(1_020);
    assert!(!committed);
    assert!(throttled.is_throttling());
    assert_eq!(throttled.get(), 5);
}

#[test]
fn throttled_value_tick_at_interval_commits_pending() {
    let throttled: ThrottledValue<i32> = ThrottledValue::new(100);
    throttled.set(5, 1_000);
    throttled.set(7, 1_010);
    let committed: bool = throttled.tick(1_100);
    assert!(committed);
    assert_eq!(throttled.get(), 7);
    assert!(!throttled.is_throttling());
}

#[test]
fn throttled_value_tick_at_interval_with_no_pending_lapses_cooldown() {
    let throttled: ThrottledValue<i32> = ThrottledValue::new(100);
    throttled.set(5, 1_000);
    let committed: bool = throttled.tick(1_100);
    assert!(!committed);
    assert_eq!(throttled.get(), 5);
    assert!(!throttled.is_throttling());
}

#[test]
fn throttled_value_tick_after_interval_reopens_window() {
    let throttled: ThrottledValue<i32> = ThrottledValue::new(50);
    throttled.set(1, 1_000);
    throttled.tick(1_060);
    assert!(!throttled.is_throttling());
    throttled.set(2, 1_070);
    assert_eq!(throttled.get(), 2);
    assert!(throttled.is_throttling());
}

#[test]
fn throttled_value_multiple_buffered_sets_only_last_wins() {
    let throttled: ThrottledValue<i32> = ThrottledValue::new(100);
    throttled.set(1, 1_000);
    throttled.set(2, 1_010);
    throttled.set(3, 1_020);
    throttled.set(4, 1_030);
    let committed: bool = throttled.tick(1_110);
    assert!(committed);
    assert_eq!(throttled.get(), 4);
}

#[test]
fn throttled_value_cancel_drops_pending_and_cooldown() {
    let throttled: ThrottledValue<i32> = ThrottledValue::new(100);
    throttled.set(5, 1_000);
    throttled.set(7, 1_010);
    throttled.cancel();
    assert!(!throttled.is_throttling());
    assert_eq!(throttled.get(), 5);
    let committed: bool = throttled.tick(1_110);
    assert!(!committed);
    assert_eq!(throttled.get(), 5);
}

#[test]
fn throttled_value_zero_interval_emits_every_set() {
    let throttled: ThrottledValue<i32> = ThrottledValue::new(0);
    throttled.set(1, 1_000);
    throttled.set(2, 1_000);
    throttled.set(3, 1_000);
    assert_eq!(throttled.get(), 3);
    assert!(!throttled.is_throttling());
}

#[test]
fn throttled_value_tick_when_idle_is_noop() {
    let throttled: ThrottledValue<i32> = ThrottledValue::new(100);
    let committed: bool = throttled.tick(1_000);
    assert!(!committed);
    assert_eq!(throttled.get(), 0);
    assert!(!throttled.is_throttling());
}

#[test]
fn throttled_value_clone_shares_state() {
    let original: ThrottledValue<i32> = ThrottledValue::new(100);
    let clone: ThrottledValue<i32> = original.clone();
    clone.set(9, 1_000);
    assert_eq!(original.get(), 9);
    assert!(original.is_throttling());
}

#[test]
fn throttled_value_display_idle() {
    let throttled: ThrottledValue<i32> = ThrottledValue::new(100);
    let formatted: String = format!("{throttled}");
    assert_eq!(formatted, "ThrottledValue(0)");
}

#[test]
fn throttled_value_display_cooldown() {
    let throttled: ThrottledValue<i32> = ThrottledValue::new(100);
    throttled.set(99, 1_000);
    let formatted: String = format!("{throttled}");
    assert_eq!(formatted, "ThrottledValue(cooldown=99)");
}

#[test]
fn throttled_value_seed_helper_commits_immediately() {
    let throttled: ThrottledValue<i32> = ThrottledValue::new(100);
    seed_throttled(&throttled, 42, 1_000);
    assert_eq!(throttled.get(), 42);
    assert!(throttled.is_throttling());
}
