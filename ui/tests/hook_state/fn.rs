use super::*;

fn bounded() -> Counter {
    Counter::new(Some(0), Some(10), 1)
}

fn unbounded() -> Counter {
    Counter::new(None, None, 1)
}

#[test]
fn a_bounded_counter_starts_sitting_on_its_floor() {
    let counter: Counter = bounded();
    assert_eq!(counter.get(), 0, "a fresh counter sits at zero");
    assert!(
        counter.is_at_min(),
        "and a zero-valued counter with min 0 is already on its floor"
    );
    assert!(!counter.is_at_max(), "while it is nowhere near its ceiling");
}

#[test]
fn incrementing_moves_towards_the_ceiling() {
    let counter: Counter = bounded();
    counter.increment();
    assert_eq!(counter.get(), 1, "one increment lands on one");
}

#[test]
fn incrementing_uses_the_configured_step() {
    let counter: Counter = Counter::new(Some(0), Some(100), 5);
    counter.increment();
    assert_eq!(
        counter.get(),
        5,
        "the step size is honoured, not always one"
    );
}

#[test]
fn incrementing_stops_at_the_ceiling() {
    let counter: Counter = bounded();
    for _ in 0..20 {
        counter.increment();
    }
    assert_eq!(counter.get(), 10, "the ceiling clamps the value");
    assert!(counter.is_at_max(), "and the counter knows it is there");
}

#[test]
fn decrementing_stops_at_the_floor() {
    let counter: Counter = bounded();
    for _ in 0..20 {
        counter.decrement();
    }
    assert_eq!(counter.get(), 0, "the floor clamps the value");
    assert!(counter.is_at_min(), "and the counter knows it is there");
}

#[test]
fn an_unbounded_counter_has_no_ceiling_or_floor() {
    let counter: Counter = unbounded();
    for _ in 0..50 {
        counter.increment();
    }
    assert_eq!(counter.get(), 50, "with no max the value runs free");
    assert!(
        !counter.is_at_max(),
        "and it never reports being at a ceiling"
    );
    for _ in 0..80 {
        counter.decrement();
    }
    assert_eq!(counter.get(), -30, "nor is there a floor to stop at");
    assert!(!counter.is_at_min(), "so it is never at a floor either");
}

#[test]
fn set_clamps_into_the_configured_range() {
    let counter: Counter = bounded();
    counter.set(999);
    assert_eq!(
        counter.get(),
        10,
        "an overshoot is pulled down to the ceiling"
    );
    counter.set(-999);
    assert_eq!(counter.get(), 0, "an undershoot is pulled up to the floor");
}

#[test]
fn set_accepts_a_value_inside_the_range_untouched() {
    let counter: Counter = bounded();
    counter.set(4);
    assert_eq!(counter.get(), 4, "a value in range passes through");
}

#[test]
fn set_unchecked_bypasses_the_clamp() {
    let counter: Counter = bounded();
    counter.set_unchecked(999);
    assert_eq!(
        counter.get(),
        999,
        "set_unchecked is the deliberate escape hatch, so it must not clamp"
    );
    assert!(
        counter.is_at_max(),
        "and the ceiling predicate reads value >= max, so an overshot counter is at max"
    );
}

#[test]
fn incrementing_past_the_ceiling_stays_where_it_is() {
    let counter: Counter = bounded();
    for _ in 0..10 {
        counter.increment();
    }
    counter.increment();
    assert_eq!(
        counter.get(),
        10,
        "a further increment cannot escape the ceiling"
    );
}

#[test]
fn a_large_step_clamps_in_one_shoot() {
    let counter: Counter = Counter::new(Some(0), Some(10), 100);
    counter.increment();
    assert_eq!(
        counter.get(),
        10,
        "a step larger than the range clamps rather than wrapping"
    );
}

#[test]
fn a_toggle_starts_off_and_flips() {
    let value: Toggle = Toggle::new();
    assert!(!value.get(), "a fresh toggle is false");
    value.toggle();
    assert!(value.get(), "toggling turns it on");
    value.toggle();
    assert!(!value.get(), "toggling again turns it off");
}

#[test]
fn a_toggle_can_be_set_directly() {
    let value: Toggle = Toggle::new();
    value.set(true);
    assert!(value.get(), "set(true) turns it on");
    value.set(true);
    assert!(value.get(), "setting the same value again is idempotent");
    value.set_false();
    assert!(!value.get(), "set_false turns it off");
    value.set_true();
    assert!(value.get(), "set_true turns it back on");
}

#[test]
fn a_lazy_component_starts_pending() {
    let value: LazyComponent<i32> = LazyComponent::new(|| 7);
    assert_eq!(
        value.loaded(),
        None,
        "nothing is built until it is asked for"
    );
}

#[test]
fn prefetching_builds_the_value_without_reading_it() {
    let value: LazyComponent<i32> = LazyComponent::new(|| 7);
    value.prefetch();
    assert_eq!(value.loaded(), Some(7), "prefetch runs the factory");
}

#[test]
fn the_factory_runs_only_once() {
    let calls: Rc<Cell<u32>> = Rc::new(Cell::new(0));
    let counter: Rc<Cell<u32>> = Rc::clone(&calls);
    let value: LazyComponent<i32> = LazyComponent::new(move || {
        counter.set(counter.get() + 1);
        7
    });
    let _: Option<i32> = value.get();
    let _: Option<i32> = value.get();
    let _: Option<i32> = value.get();
    assert_eq!(calls.get(), 1, "the factory must not run again once loaded");
}

#[test]
fn a_lazy_component_hands_back_the_same_value_every_time() {
    let value: LazyComponent<i32> = LazyComponent::new(|| 7);
    let first: Option<i32> = value.get();
    let second: Option<i32> = value.get();
    assert_eq!(first, Some(7), "the first read builds it");
    assert_eq!(second, first, "and every later read sees the same value");
}

#[test]
fn prefetching_twice_does_not_rebuild() {
    let calls: Rc<Cell<u32>> = Rc::new(Cell::new(0));
    let counter: Rc<Cell<u32>> = Rc::clone(&calls);
    let value: LazyComponent<i32> = LazyComponent::new(move || {
        counter.set(counter.get() + 1);
        7
    });
    value.prefetch();
    value.prefetch();
    assert_eq!(calls.get(), 1, "the second prefetch is a no-op");
}

#[test]
fn reset_returns_a_lazy_component_to_pending() {
    let calls: Rc<Cell<u32>> = Rc::new(Cell::new(0));
    let counter: Rc<Cell<u32>> = Rc::clone(&calls);
    let value: LazyComponent<i32> = LazyComponent::new(move || {
        counter.set(counter.get() + 1);
        7
    });
    let _: Option<i32> = value.get();
    value.reset();
    assert_eq!(value.loaded(), None, "reset drops the loaded value");
    let _: Option<i32> = value.get();
    assert_eq!(calls.get(), 2, "so the next read rebuilds it");
}

#[test]
fn a_lazy_component_has_no_public_load_state_to_compare() {
    let value: LazyComponent<i32> = LazyComponent::new(|| 1);
    let observed: Option<i32> = value.get();
    assert_eq!(
        observed,
        Some(1),
        "the load state is only observable through get"
    );
}
