use super::*;

fn debounced(delay: u32) -> DebouncedValue<i32> {
    DebouncedValue::new(delay)
}

fn throttled(interval: u32) -> ThrottledValue<i32> {
    ThrottledValue::new(interval)
}

#[test]
fn a_fresh_debounced_value_holds_the_default() {
    let value: DebouncedValue<i32> = debounced(100);
    assert_eq!(value.get(), 0, "it starts at the type default");
    assert!(!value.is_pending(), "and nothing is waiting to commit");
}

#[test]
fn setting_a_debounced_value_does_not_change_it_yet() {
    let value: DebouncedValue<i32> = debounced(100);
    value.set(5, 0);
    assert_eq!(
        value.get(),
        0,
        "the point of debouncing is to not commit yet"
    );
    assert!(value.is_pending(), "the new value is parked as pending");
}

#[test]
fn a_debounced_value_commits_once_the_delay_elapses() {
    let value: DebouncedValue<i32> = debounced(100);
    value.set(5, 1_000);
    let committed: bool = value.tick(1_050);
    assert!(!committed, "50ms into a 100ms delay is too early");
    let late: bool = value.tick(1_100);
    assert!(late, "exactly at the delay the value commits");
    assert_eq!(
        value.get(),
        5,
        "and the parked value becomes the current one"
    );
    assert!(!value.is_pending(), "nothing is left pending");
}

#[test]
fn ticking_a_debounced_value_before_the_delay_changes_nothing() {
    let value: DebouncedValue<i32> = debounced(100);
    value.set(5, 0);
    let committed: bool = value.tick(99);
    assert!(!committed, "one millisecond short is still pending");
    assert_eq!(value.get(), 0, "so the current value is untouched");
    assert!(value.is_pending(), "and the pending value is still parked");
}

#[test]
fn only_the_most_recent_debounced_value_survives() {
    let value: DebouncedValue<i32> = debounced(100);
    value.set(1, 0);
    value.set(2, 10);
    value.set(3, 20);
    let too_early: bool = value.tick(100);
    assert!(
        !too_early,
        "the window restarts on every set, so t = 100 is only 80ms after the last one"
    );
    let committed: bool = value.tick(120);
    assert!(
        committed,
        "a full delay after the most recent set commits it"
    );
    assert_eq!(value.get(), 3, "and only the latest input is committed");
}

#[test]
fn cancelling_a_debounced_value_drops_the_pending_input() {
    let value: DebouncedValue<i32> = debounced(100);
    value.set(5, 0);
    value.cancel();
    assert!(!value.is_pending(), "cancelling clears the parked value");
    let committed: bool = value.tick(1_000);
    assert!(!committed, "so a later tick has nothing to commit");
    assert_eq!(value.get(), 0, "and the current value never moved");
}

#[test]
fn ticking_an_idle_debounced_value_reports_nothing_committed() {
    let value: DebouncedValue<i32> = debounced(100);
    let committed: bool = value.tick(10_000);
    assert!(!committed, "with nothing pending a tick is a no-op");
}

#[test]
fn a_zero_delay_debounced_value_commits_on_the_next_tick() {
    let value: DebouncedValue<i32> = debounced(0);
    value.set(7, 0);
    let committed: bool = value.tick(0);
    assert!(
        committed,
        "a zero delay has already elapsed by the next tick"
    );
    assert_eq!(value.get(), 7, "so the value lands immediately");
}

#[test]
fn a_fresh_throttled_value_holds_the_default() {
    let value: ThrottledValue<i32> = throttled(100);
    assert_eq!(value.get(), 0, "it starts at the type default");
    assert!(!value.is_throttling(), "and is not cooling down");
}

#[test]
fn the_first_throttled_write_commits_immediately() {
    let value: ThrottledValue<i32> = throttled(100);
    value.set(5, 0);
    assert_eq!(
        value.get(),
        5,
        "a throttle only delays the writes that follow the first one"
    );
    assert!(value.is_throttling(), "and opens a cooldown window");
}

#[test]
fn a_write_inside_the_cooldown_is_deferred() {
    let value: ThrottledValue<i32> = throttled(100);
    value.set(1, 0);
    value.set(2, 10);
    assert_eq!(value.get(), 1, "the second write does not land yet");
}

#[test]
fn a_deferred_write_lands_once_the_cooldown_expires() {
    let value: ThrottledValue<i32> = throttled(100);
    value.set(1, 0);
    value.set(2, 10);
    let committed: bool = value.tick(50);
    assert!(!committed, "the cooldown is still running");
    let later: bool = value.tick(100);
    assert!(later, "at the interval the deferred value lands");
    assert_eq!(value.get(), 2, "and it replaces the committed one");
    assert!(!value.is_throttling(), "the cooldown closes");
}

#[test]
fn only_the_last_write_inside_a_cooldown_survives() {
    let value: ThrottledValue<i32> = throttled(100);
    value.set(1, 0);
    value.set(2, 10);
    value.set(3, 20);
    let committed: bool = value.tick(100);
    assert!(committed, "the cooldown expires normally");
    assert_eq!(
        value.get(),
        3,
        "but only the most recent input is committed"
    );
}

#[test]
fn a_tick_that_finds_no_pending_write_closes_the_cooldown_without_committing() {
    let value: ThrottledValue<i32> = throttled(100);
    value.set(1, 0);
    let committed: bool = value.tick(100);
    assert!(
        !committed,
        "nothing was deferred, so the expiry has nothing to commit"
    );
    assert!(!value.is_throttling(), "and the cooldown closes regardless");
}

#[test]
fn cancelling_a_throttle_drops_the_deferred_write() {
    let value: ThrottledValue<i32> = throttled(100);
    value.set(1, 0);
    value.set(2, 10);
    value.cancel();
    assert!(!value.is_throttling(), "cancelling closes the cooldown");
    let committed: bool = value.tick(100);
    assert!(!committed, "and the deferred write is gone");
    assert_eq!(value.get(), 1, "so the committed value never moves");
}

#[test]
fn a_zero_interval_throttle_commits_every_write() {
    let value: ThrottledValue<i32> = throttled(0);
    value.set(1, 0);
    value.set(2, 1);
    assert_eq!(
        value.get(),
        2,
        "with no interval the cooldown branch is never taken, so every write lands"
    );
    assert!(!value.is_throttling(), "and no cooldown is ever opened");
}

#[test]
fn a_throttled_value_reports_its_state_through_display() {
    let value: ThrottledValue<i32> = throttled(100);
    let idle: String = format!("{value}");
    assert!(idle.starts_with("ThrottledValue("), "got {idle}");
    value.set(1, 0);
    let cooling: String = format!("{value}");
    assert!(
        cooling.contains("cooldown"),
        "the cooling state is spelled out, got {cooling}"
    );
}

#[test]
fn a_debounced_value_reports_its_state_through_display() {
    let value: DebouncedValue<i32> = debounced(100);
    let idle: String = format!("{value}");
    assert!(idle.starts_with("DebouncedValue("), "got {idle}");
    value.set(4, 0);
    let pending: String = format!("{value}");
    assert!(
        pending.contains("pending=4"),
        "the parked value is visible in the debug form, got {pending}"
    );
}
