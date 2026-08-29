use super::*;

/// Helper body of the `seed_debounced` free function.
///
/// # Arguments
///
/// - `&DebouncedValue<T>` - Shared reference to a `DebouncedValue<T>`.
/// - `T: Clone + PartialEq + Default + 'static` - A generic type parameter.
/// - `u32` - A 32-bit unsigned integer (`u32`).
/// - `u64` - The current time in milliseconds.
fn seed_debounced<T: Clone + PartialEq + Default + 'static>(
    debounced: &DebouncedValue<T>,
    initial: T,
    delay_ms: u32,
    now_ms: u64,
) {
    debounced.set(initial, now_ms);
    debounced.tick(now_ms + u64::from(delay_ms) + 1);
}

#[test]
fn debounced_value_starts_at_default() {
    let debounced: DebouncedValue<i32> = DebouncedValue::new(100);
    assert_eq!(debounced.get(), 0);
    assert!(!debounced.is_pending());
}

#[test]
fn debounced_value_set_marks_pending() {
    let debounced: DebouncedValue<i32> = DebouncedValue::new(100);
    debounced.set(5, 1_000);
    assert!(debounced.is_pending());
    assert_eq!(debounced.get(), 0);
}

#[test]
fn debounced_value_tick_before_delay_keeps_default() {
    let debounced: DebouncedValue<i32> = DebouncedValue::new(100);
    debounced.set(5, 1_000);
    let emitted: bool = debounced.tick(1_000);
    assert!(!emitted);
    assert!(debounced.is_pending());
    assert_eq!(debounced.get(), 0);
}

#[test]
fn debounced_value_tick_at_delay_emits_pending() {
    let debounced: DebouncedValue<i32> = DebouncedValue::new(100);
    debounced.set(5, 1_000);
    let emitted: bool = debounced.tick(1_100);
    assert!(emitted);
    assert!(!debounced.is_pending());
    assert_eq!(debounced.get(), 5);
}

#[test]
fn debounced_value_tick_past_delay_emits_pending() {
    let debounced: DebouncedValue<i32> = DebouncedValue::new(100);
    debounced.set(7, 1_000);
    let emitted: bool = debounced.tick(1_250);
    assert!(emitted);
    assert_eq!(debounced.get(), 7);
}

#[test]
fn debounced_value_rapid_sets_only_last_wins() {
    let debounced: DebouncedValue<i32> = DebouncedValue::new(100);
    debounced.set(1, 1_000);
    debounced.set(2, 1_010);
    debounced.set(3, 1_020);
    let emitted: bool = debounced.tick(1_150);
    assert!(emitted);
    assert_eq!(debounced.get(), 3);
}

#[test]
fn debounced_value_cancel_drops_pending() {
    let debounced: DebouncedValue<i32> = DebouncedValue::new(100);
    debounced.set(5, 1_000);
    debounced.cancel();
    assert!(!debounced.is_pending());
    assert_eq!(debounced.get(), 0);
}

#[test]
fn debounced_value_zero_delay_emits_immediately() {
    let debounced: DebouncedValue<i32> = DebouncedValue::new(0);
    debounced.set(5, 1_000);
    let emitted: bool = debounced.tick(1_000);
    assert!(emitted);
    assert_eq!(debounced.get(), 5);
}

#[test]
fn debounced_value_tick_when_idle_is_noop() {
    let debounced: DebouncedValue<i32> = DebouncedValue::new(100);
    let emitted: bool = debounced.tick(1_000);
    assert!(!emitted);
    assert_eq!(debounced.get(), 0);
}

#[test]
fn debounced_value_two_pending_cycles() {
    let debounced: DebouncedValue<i32> = DebouncedValue::new(50);
    debounced.set(1, 1_000);
    assert!(debounced.tick(1_060));
    assert_eq!(debounced.get(), 1);
    debounced.set(2, 1_070);
    assert!(!debounced.tick(1_080));
    assert!(debounced.tick(1_130));
    assert_eq!(debounced.get(), 2);
}

#[test]
fn debounced_value_clone_shares_state() {
    let original: DebouncedValue<i32> = DebouncedValue::new(100);
    let clone: DebouncedValue<i32> = original.clone();
    clone.set(9, 1_000);
    assert!(original.is_pending());
    assert!(original.tick(1_150));
    assert_eq!(clone.get(), 9);
}

#[test]
fn debounced_value_string_round_trip() {
    let debounced: DebouncedValue<String> = DebouncedValue::new(10);
    debounced.set(String::from("hello"), 1_000);
    assert!(debounced.tick(1_020));
    assert_eq!(debounced.get(), String::from("hello"));
}

#[test]
fn debounced_value_display_idle() {
    let debounced: DebouncedValue<i32> = DebouncedValue::new(100);
    let formatted: String = format!("{debounced}");
    assert_eq!(formatted, "DebouncedValue(0)");
}

#[test]
fn debounced_value_display_pending() {
    let debounced: DebouncedValue<i32> = DebouncedValue::new(100);
    debounced.set(99, 1_000);
    let formatted: String = format!("{debounced}");
    assert_eq!(formatted, "DebouncedValue(pending=99)");
}

#[test]
fn debounced_value_seed_helper_commits_immediately() {
    let debounced: DebouncedValue<i32> = DebouncedValue::new(10);
    seed_debounced(&debounced, 42, 10, 1_000);
    assert_eq!(debounced.get(), 42);
    assert!(!debounced.is_pending());
}
