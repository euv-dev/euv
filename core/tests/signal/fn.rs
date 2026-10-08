use super::*;

#[test]
fn signal_cell_set_then_get() {
    let cell: SignalCell<i32> = SignalCell::default();
    let signal: Signal<i32> = Signal::create(42);
    cell.set(signal);
    let stored: Signal<i32> = cell.loaded().expect("cell should be initialized");
    assert_eq!(stored.get(), 42);
}

#[test]
fn signal_cell_set_overwrites_via_none_then_set() {
    let cell: SignalCell<String> = SignalCell::default();
    let signal: Signal<String> = Signal::create(String::from("first"));
    cell.set(signal);
    let stored: Signal<String> = cell.loaded().expect("cell should be initialized");
    assert_eq!(stored.get(), "first");
}

#[test]
fn signal_cell_with_string_value() {
    let cell: SignalCell<String> = SignalCell::default();
    let signal: Signal<String> = Signal::create(String::from("hello"));
    cell.set(signal);
    let stored: Signal<String> = cell.loaded().expect("cell should be initialized");
    assert_eq!(stored.get(), "hello");
}

#[test]
fn signal_create_returns_handle() {
    let signal: Signal<i32> = Signal::create(7);
    let value: i32 = signal.get();
    assert_eq!(value, 7);
}

#[test]
fn signal_create_with_string() {
    let signal: Signal<String> = Signal::create(String::from("hi"));
    assert_eq!(signal.get(), "hi");
}

#[test]
fn signal_create_with_vec() {
    let signal: Signal<Vec<i32>> = Signal::create(vec![1, 2, 3]);
    assert_eq!(signal.get(), vec![1, 2, 3]);
}

#[test]
fn signal_copy_semantics_share_state() {
    let a: Signal<i32> = Signal::create(10);
    let b: Signal<i32> = a;
    assert_eq!(a.get(), 10);
    assert_eq!(b.get(), 10);
}

#[test]
fn signal_clone_via_copy_is_idempotent() {
    let signal: Signal<i32> = Signal::create(42);
    let c1: Signal<i32> = signal;
    let c2: Signal<i32> = signal;
    let c3: Signal<i32> = signal;
    assert_eq!(c1.get(), 42);
    assert_eq!(c2.get(), 42);
    assert_eq!(c3.get(), 42);
}

#[test]
fn fire_handle_new_yields_valid_handle() {
    let handle: FireHandle = FireHandle::new(|| {});
    assert_ne!(usize::from(handle), 0);
}

#[test]
fn fire_handle_from_closure() {
    let handle: FireHandle = FireHandle::from(|| {});
    assert_ne!(usize::from(handle), 0);
}

#[test]
fn fire_handle_is_copy() {
    let handle: FireHandle = FireHandle::from(|| {});
    let copy1: FireHandle = handle;
    let copy2: FireHandle = handle;
    let copy3: FireHandle = handle;
    assert_eq!(copy1, copy2);
    assert_eq!(copy2, copy3);
    assert_eq!(copy1, copy3);
}

#[test]
fn fire_handle_default_inner_is_zero() {
    let a: FireHandle = unsafe { zeroed() };
    let b: FireHandle = unsafe { zeroed() };
    assert_eq!(a, b);
    let mut h1: DefaultHasher = DefaultHasher::new();
    let mut h2: DefaultHasher = DefaultHasher::new();
    a.hash(&mut h1);
    b.hash(&mut h2);
    assert_eq!(h1.finish(), h2.finish());
}

#[test]
fn fire_handle_distinct_closures_have_distinct_addresses() {
    let a: FireHandle = FireHandle::from(|| {});
    let b: FireHandle = FireHandle::from(|| {});
    assert_ne!(a, b);
}

#[test]
fn fire_handle_fire_invokes_closure() {
    let counter: Rc<Cell<i32>> = Rc::new(Cell::new(0));
    let counter_for_closure: Rc<Cell<i32>> = counter.clone();
    let handle: FireHandle = FireHandle::new(move || {
        counter_for_closure.set(counter_for_closure.get() + 1);
    });
    unsafe {
        handle.fire();
    }
    assert_eq!(counter.get(), 1);
}

#[test]
fn fire_handle_fire_can_be_called_repeatedly() {
    let counter: Rc<Cell<i32>> = Rc::new(Cell::new(0));
    let counter_for_closure: Rc<Cell<i32>> = counter.clone();
    let handle: FireHandle = FireHandle::new(move || {
        counter_for_closure.set(counter_for_closure.get() + 1);
    });
    unsafe {
        handle.fire();
        handle.fire();
        handle.fire();
    }
    assert_eq!(counter.get(), 3);
}

#[test]
fn fire_handle_clone_via_copy_increments_underlying_counter() {
    let counter: Rc<Cell<i32>> = Rc::new(Cell::new(0));
    let counter_for_closure: Rc<Cell<i32>> = counter.clone();
    let handle: FireHandle = FireHandle::new(move || {
        counter_for_closure.set(counter_for_closure.get() + 1);
    });
    let copy: FireHandle = handle;
    unsafe {
        handle.fire();
    }
    assert_eq!(counter.get(), 1);
    unsafe {
        copy.fire();
    }
    assert_eq!(counter.get(), 2);
}

#[test]
fn native_signal_get_does_not_panic() {
    let result: Result<(), ()> = super::catch_unwind(super::AssertUnwindSafe(|| {
        let signal: Signal<i32> = Signal::create(11);
        let _: i32 = signal.get();
    }))
    .map_err(|_| ());
    assert!(result.is_ok());
}

#[test]
fn native_fire_handle_fire_does_not_panic() {
    let result: Result<(), ()> = super::catch_unwind(super::AssertUnwindSafe(|| {
        let handle: FireHandle = FireHandle::from(|| {});
        unsafe {
            handle.fire();
        }
    }))
    .map_err(|_| ());
    assert!(result.is_ok());
}

#[cfg(target_arch = "wasm32")]
#[test]
fn signal_set_persists_inner_state_across_writes() {
    let signal: Signal<i32> = Signal::create(0);
    signal.set(1);
    signal.set(2);
    signal.set(3);
    assert_eq!(signal.get(), 3);
    signal.set(4);
    signal.set(5);
    assert_eq!(signal.get(), 5);
}

#[test]
fn a_subscriber_is_notified_when_the_signal_is_set() {
    let signal: Signal<i32> = Signal::create(0);
    let hits: Rc<Cell<u32>> = Rc::new(Cell::new(0));
    let counter: Rc<Cell<u32>> = hits.clone();
    let _id: usize = signal.subscribe(move || {
        counter.set(counter.get() + 1);
    });
    signal.set(1);
    assert_eq!(hits.get(), 1, "one set means one notification");
    signal.set(2);
    assert_eq!(hits.get(), 2, "and a second set notifies again");
    assert_eq!(signal.get(), 2, "the value still lands");
}

#[test]
fn every_subscriber_is_notified_in_subscription_order() {
    let signal: Signal<i32> = Signal::create(0);
    let order: Rc<RefCell<Vec<usize>>> = Rc::new(RefCell::new(Vec::new()));
    for tag in 0..3usize {
        let sink: Rc<RefCell<Vec<usize>>> = order.clone();
        let _id: usize = signal.subscribe(move || {
            sink.borrow_mut().push(tag);
        });
    }
    signal.set(7);
    assert_eq!(*order.borrow(), vec![0, 1, 2]);
}

#[test]
fn each_subscription_gets_its_own_id() {
    let signal: Signal<i32> = Signal::create(0);
    let first: usize = signal.subscribe(|| {});
    let second: usize = signal.subscribe(|| {});
    assert_ne!(first, second, "an id identifies one specific listener");
}

#[test]
fn unsubscribing_stops_only_the_named_listener() {
    let signal: Signal<i32> = Signal::create(0);
    let kept_hits: Rc<Cell<u32>> = Rc::new(Cell::new(0));
    let dropped_hits: Rc<Cell<u32>> = Rc::new(Cell::new(0));
    let kept: Rc<Cell<u32>> = kept_hits.clone();
    let dropped: Rc<Cell<u32>> = dropped_hits.clone();
    let kept_id: usize = signal.subscribe(move || kept.set(kept.get() + 1));
    let dropped_id: usize = signal.subscribe(move || dropped.set(dropped.get() + 1));
    signal.set(1);
    assert_eq!((kept_hits.get(), dropped_hits.get()), (1, 1));
    signal.unsubscribe(dropped_id);
    signal.set(2);
    assert_eq!(kept_hits.get(), 2, "the surviving listener keeps firing");
    assert_eq!(dropped_hits.get(), 1, "the detached one stays silent");
    assert_ne!(kept_id, dropped_id);
}

#[test]
fn unsubscribing_an_unknown_id_changes_nothing() {
    let signal: Signal<i32> = Signal::create(0);
    let hits: Rc<Cell<u32>> = Rc::new(Cell::new(0));
    let counter: Rc<Cell<u32>> = hits.clone();
    let real_id: usize = signal.subscribe(move || counter.set(counter.get() + 1));
    signal.unsubscribe(real_id.wrapping_add(1000));
    signal.unsubscribe(usize::MAX);
    signal.set(1);
    assert_eq!(hits.get(), 1, "a bogus id must not evict a real listener");
}

#[test]
fn a_listener_that_unsubscribes_itself_from_inside_its_own_callback_never_fires_again() {
    let signal: Signal<i32> = Signal::create(0);
    let hits: Rc<Cell<u32>> = Rc::new(Cell::new(0));
    let counter: Rc<Cell<u32>> = hits.clone();
    let id: Rc<Cell<usize>> = Rc::new(Cell::new(usize::MAX));
    let id_cell: Rc<Cell<usize>> = id.clone();
    let _subscription: usize = signal.subscribe(move || {
        counter.set(counter.get() + 1);
        if id_cell.get() != usize::MAX {
            signal.unsubscribe(id_cell.get());
            id_cell.set(usize::MAX);
        }
    });
    id.set(_subscription);
    signal.set(1);
    assert_eq!(hits.get(), 1);
    signal.set(2);
    signal.set(3);
    assert_eq!(
        hits.get(),
        1,
        "removal during notification is deferred, and the deferred id must not \
         be resurrected into the live listener list"
    );
}

#[test]
fn a_signal_with_no_subscribers_still_stores_its_value() {
    let signal: Signal<i32> = Signal::create(5);
    signal.set(6);
    assert_eq!(signal.get(), 6);
}

#[test]
fn two_signals_keep_independent_subscriptions() {
    let left: Signal<i32> = Signal::create(0);
    let right: Signal<i32> = Signal::create(0);
    let left_hits: Rc<Cell<u32>> = Rc::new(Cell::new(0));
    let right_hits: Rc<Cell<u32>> = Rc::new(Cell::new(0));
    let left_counter: Rc<Cell<u32>> = left_hits.clone();
    let right_counter: Rc<Cell<u32>> = right_hits.clone();
    let _left_id: usize = left.subscribe(move || left_counter.set(left_counter.get() + 1));
    let _right_id: usize = right.subscribe(move || right_counter.set(right_counter.get() + 1));
    left.set(1);
    assert_eq!((left_hits.get(), right_hits.get()), (1, 0));
    right.set(1);
    assert_eq!((left_hits.get(), right_hits.get()), (1, 1));
}
