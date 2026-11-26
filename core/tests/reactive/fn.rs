use super::*;

#[test]
fn signal_memo_effect_and_batch_are_glitch_free() {
    reset_reactive_runtime();
    let count: Signal<i32> = Signal::new(1);
    let doubled: Memo<i32> = Memo::new(move || count.get() * 2);
    let quadrupled: Memo<i32> = Memo::new(move || doubled.get() * 2);
    let runs: Rc<Cell<usize>> = Rc::new(Cell::new(0));
    let runs_c: Rc<Cell<usize>> = runs.clone();
    let last_sum: Rc<Cell<i32>> = Rc::new(Cell::new(0));
    let sum_c: Rc<Cell<i32>> = last_sum.clone();
    let _effect: Effect = Effect::new(move || {
        runs_c.set(runs_c.get() + 1);
        sum_c.set(doubled.get() + quadrupled.get());
    });
    assert_eq!(runs.get(), 1);
    assert_eq!(last_sum.get(), 6);
    batch(|| {
        let _: bool = count.set(2);
        let _: bool = count.set(5);
    });
    assert_eq!(runs.get(), 2);
    assert_eq!(last_sum.get(), 30);
    assert_eq!(doubled.get(), 10);
    assert_eq!(quadrupled.get(), 20);
}

#[test]
fn memo_equality_short_circuits_downstream_effects() {
    reset_reactive_runtime();
    let value: Signal<i32> = Signal::new(2);
    let is_even: Memo<bool> = Memo::new(move || value.get() % 2 == 0);
    let runs: Rc<Cell<usize>> = Rc::new(Cell::new(0));
    let runs_c: Rc<Cell<usize>> = runs.clone();
    let _effect: Effect = Effect::new(move || {
        let _: bool = is_even.get();
        runs_c.set(runs_c.get() + 1);
    });
    assert_eq!(runs.get(), 1);
    let _: bool = value.set(4);
    assert!(is_even.get());
    let _: bool = value.set(5);
    assert!(!is_even.get());
    assert!(runs.get() >= 2);
}

#[test]
fn selector_notifies_only_previous_and_next_keys() {
    reset_reactive_runtime();
    let selected: Signal<usize> = Signal::new(1);
    let selector: Rc<Selector<usize>> = Rc::new(Selector::new(move || selected.get()));
    let hits_one: Rc<Cell<usize>> = Rc::new(Cell::new(0));
    let hits_two: Rc<Cell<usize>> = Rc::new(Cell::new(0));
    let hits_three: Rc<Cell<usize>> = Rc::new(Cell::new(0));
    let s1: Rc<Selector<usize>> = selector.clone();
    let h1: Rc<Cell<usize>> = hits_one.clone();
    let _e1: Effect = Effect::new(move || {
        let _: bool = s1.is_selected(1);
        h1.set(h1.get() + 1);
    });
    let s2: Rc<Selector<usize>> = selector.clone();
    let h2: Rc<Cell<usize>> = hits_two.clone();
    let _e2: Effect = Effect::new(move || {
        let _: bool = s2.is_selected(2);
        h2.set(h2.get() + 1);
    });
    let s3: Rc<Selector<usize>> = selector.clone();
    let h3: Rc<Cell<usize>> = hits_three.clone();
    let _e3: Effect = Effect::new(move || {
        let _: bool = s3.is_selected(3);
        h3.set(h3.get() + 1);
    });
    let _: bool = selected.set(2);
    assert_eq!(hits_one.get(), 2);
    assert_eq!(hits_two.get(), 2);
    assert_eq!(hits_three.get(), 1);
    selector.dispose();
}

#[test]
fn scope_disposes_owned_resources_and_runs_cleanups() {
    reset_reactive_runtime();
    let cleaned: Rc<Cell<bool>> = Rc::new(Cell::new(false));
    let cleaned_c: Rc<Cell<bool>> = cleaned.clone();
    let (scope, sig): (Scope, Signal<i32>) = Scope::run(|_: Scope| {
        on_cleanup(move || {
            cleaned_c.set(true);
        });
        Signal::new(42)
    });
    assert_eq!(sig.try_get(), Some(42));
    scope.dispose();
    assert!(cleaned.get());
    assert_eq!(sig.try_get(), None);
}

#[test]
fn scope_freeze_and_resume_coalesce_cross_page_updates() {
    reset_reactive_runtime();
    let shared: Signal<i32> = Signal::new(0);
    let runs: Rc<Cell<usize>> = Rc::new(Cell::new(0));
    let runs_c: Rc<Cell<usize>> = runs.clone();
    let last: Rc<Cell<i32>> = Rc::new(Cell::new(-1));
    let last_c: Rc<Cell<i32>> = last.clone();
    let (scope, _): (Scope, Effect) = Scope::run(|_: Scope| {
        Effect::new(move || {
            runs_c.set(runs_c.get() + 1);
            last_c.set(shared.get());
        })
    });
    assert_eq!(runs.get(), 1);
    assert_eq!(last.get(), 0);
    scope.freeze();
    assert!(scope.is_frozen());
    for v in 1..=50 {
        let _: bool = shared.set(v);
    }
    assert_eq!(runs.get(), 1);
    assert_eq!(last.get(), 0);
    scope.resume();
    assert!(!scope.is_frozen());
    assert_eq!(runs.get(), 2);
    assert_eq!(last.get(), 50);
    scope.dispose();
}

#[test]
fn local_state_global_state_and_event_bus_respect_scope_freeze_and_cleanup() {
    reset_reactive_runtime();
    #[derive(Clone, Debug, PartialEq)]
    struct TestGlobalMetric(i32);
    #[derive(Clone, Debug, PartialEq)]
    struct TestCustomPing(u32);
    impl CustomEvent for TestCustomPing {}
    let g_sig: Signal<TestGlobalMetric> = GlobalState::provide(TestGlobalMetric(10));
    assert_eq!(
        GlobalState::get::<TestGlobalMetric>(),
        Some(TestGlobalMetric(10))
    );
    let global_seen: Rc<Cell<i32>> = Rc::new(Cell::new(0));
    let global_seen_c: Rc<Cell<i32>> = global_seen.clone();
    let event_seen: Rc<Cell<u32>> = Rc::new(Cell::new(0));
    let event_seen_c: Rc<Cell<u32>> = event_seen.clone();
    let local_seen: Rc<Cell<i32>> = Rc::new(Cell::new(0));
    let local_seen_c: Rc<Cell<i32>> = local_seen.clone();
    let (scope, local): (Scope, LocalState<i32>) = Scope::run(|_: Scope| {
        let local: LocalState<i32> = LocalState::new(1);
        let _: Effect = local.watch(move |_prev: &i32, next: &i32| {
            local_seen_c.set(*next);
        });
        let _: Option<Effect> = GlobalState::watch::<TestGlobalMetric, _>(
            move |_prev: &TestGlobalMetric, next: &TestGlobalMetric| {
                global_seen_c.set(next.0);
            },
        );
        let _: Effect = EventBus::on::<TestCustomPing, _>(move |ev: &TestCustomPing| {
            event_seen_c.set(ev.0);
        });
        local
    });
    let _: bool = local.set(7);
    assert_eq!(local_seen.get(), 7);
    assert!(GlobalState::set(TestGlobalMetric(25)));
    assert_eq!(global_seen.get(), 25);
    assert_eq!(g_sig.try_get_untracked(), Some(TestGlobalMetric(25)));
    EventBus::emit(TestCustomPing(42));
    assert_eq!(event_seen.get(), 42);
    scope.freeze();
    assert!(GlobalState::set(TestGlobalMetric(99)));
    EventBus::emit(TestCustomPing(100));
    assert_eq!(global_seen.get(), 25);
    assert_eq!(event_seen.get(), 42);
    scope.resume();
    assert_eq!(global_seen.get(), 99);
    assert_eq!(event_seen.get(), 100);
    scope.dispose();
    assert!(GlobalState::set(TestGlobalMetric(123)));
    EventBus::emit(TestCustomPing(200));
    assert_eq!(global_seen.get(), 99);
    assert_eq!(event_seen.get(), 100);
}
