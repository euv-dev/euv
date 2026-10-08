use super::*;

#[test]
fn subscribe_returns_a_usable_handle() {
    let signal: Signal<u32> = Signal::create(0);
    let id: usize = signal.subscribe(|| {});
    assert_ne!(id, usize::MAX, "a live signal hands out a real handle");
}

#[test]
fn each_subscription_gets_its_own_handle() {
    let signal: Signal<u32> = Signal::create(0);
    let first: usize = signal.subscribe(|| {});
    let second: usize = signal.subscribe(|| {});
    assert_ne!(
        first, second,
        "two subscriptions must be individually addressable"
    );
}

#[test]
fn a_listener_runs_when_the_value_changes() {
    let signal: Signal<u32> = Signal::create(0);
    let hits: Rc<Cell<u32>> = Rc::new(Cell::new(0));
    let counter: Rc<Cell<u32>> = Rc::clone(&hits);
    let _: usize = signal.subscribe(move || {
        counter.set(counter.get() + 1);
    });
    signal.set(1);
    assert_eq!(hits.get(), 1, "one change fires the listener once");
}

#[test]
fn a_listener_does_not_run_when_the_value_is_unchanged() {
    let signal: Signal<u32> = Signal::create(7);
    let hits: Rc<Cell<u32>> = Rc::new(Cell::new(0));
    let counter: Rc<Cell<u32>> = Rc::clone(&hits);
    let _: usize = signal.subscribe(move || {
        counter.set(counter.get() + 1);
    });
    signal.set(7);
    assert_eq!(
        hits.get(),
        0,
        "writing the same value is not a change, so nobody is notified"
    );
}

#[test]
fn a_listener_runs_once_per_actual_change() {
    let signal: Signal<u32> = Signal::create(0);
    let hits: Rc<Cell<u32>> = Rc::new(Cell::new(0));
    let counter: Rc<Cell<u32>> = Rc::clone(&hits);
    let _: usize = signal.subscribe(move || {
        counter.set(counter.get() + 1);
    });
    signal.set(1);
    signal.set(1);
    signal.set(2);
    assert_eq!(hits.get(), 2, "only the two real changes fire");
}

#[test]
fn every_listener_runs_on_a_change() {
    let signal: Signal<u32> = Signal::create(0);
    let first: Rc<Cell<u32>> = Rc::new(Cell::new(0));
    let second: Rc<Cell<u32>> = Rc::new(Cell::new(0));
    let a: Rc<Cell<u32>> = Rc::clone(&first);
    let b: Rc<Cell<u32>> = Rc::clone(&second);
    let _: usize = signal.subscribe(move || {
        a.set(a.get() + 1);
    });
    let _: usize = signal.subscribe(move || {
        b.set(b.get() + 1);
    });
    signal.set(1);
    assert_eq!(first.get(), 1, "the first listener ran");
    assert_eq!(second.get(), 1, "the second listener ran");
}

#[test]
fn a_listener_sees_the_new_value_when_it_runs() {
    let signal: Signal<u32> = Signal::create(0);
    let observed: Rc<Cell<u32>> = Rc::new(Cell::new(0));
    let slot: Rc<Cell<u32>> = Rc::clone(&observed);
    let handle: Signal<u32> = signal;
    let _: usize = signal.subscribe(move || {
        slot.set(handle.get());
    });
    signal.set(99);
    assert_eq!(
        observed.get(),
        99,
        "listeners run after the value is published, not before"
    );
}

#[test]
fn unsubscribe_detaches_the_named_listener() {
    let signal: Signal<u32> = Signal::create(0);
    let hits: Rc<Cell<u32>> = Rc::new(Cell::new(0));
    let counter: Rc<Cell<u32>> = Rc::clone(&hits);
    let id: usize = signal.subscribe(move || {
        counter.set(counter.get() + 1);
    });
    signal.set(1);
    signal.unsubscribe(id);
    signal.set(2);
    assert_eq!(
        hits.get(),
        1,
        "the detached listener must not see the later change"
    );
}

#[test]
fn unsubscribe_leaves_the_other_listeners_attached() {
    let signal: Signal<u32> = Signal::create(0);
    let first: Rc<Cell<u32>> = Rc::new(Cell::new(0));
    let second: Rc<Cell<u32>> = Rc::new(Cell::new(0));
    let a: Rc<Cell<u32>> = Rc::clone(&first);
    let b: Rc<Cell<u32>> = Rc::clone(&second);
    let id: usize = signal.subscribe(move || {
        a.set(a.get() + 1);
    });
    let _: usize = signal.subscribe(move || {
        b.set(b.get() + 1);
    });
    signal.unsubscribe(id);
    signal.set(1);
    assert_eq!(first.get(), 0, "the detached listener stayed quiet");
    assert_eq!(second.get(), 1, "the untouched listener still fires");
}

#[test]
fn a_resubscribed_signal_can_be_listened_to_again() {
    let signal: Signal<u32> = Signal::create(0);
    let hits: Rc<Cell<u32>> = Rc::new(Cell::new(0));
    let counter: Rc<Cell<u32>> = Rc::clone(&hits);
    let id: usize = signal.subscribe(move || {
        counter.set(counter.get() + 1);
    });
    signal.unsubscribe(id);
    let counter_again: Rc<Cell<u32>> = Rc::clone(&hits);
    let _: usize = signal.subscribe(move || {
        counter_again.set(counter_again.get() + 1);
    });
    signal.set(1);
    assert_eq!(hits.get(), 1, "the fresh subscription takes over cleanly");
}

#[test]
fn unsubscribing_a_bogus_handle_is_a_no_op() {
    let signal: Signal<u32> = Signal::create(0);
    let hits: Rc<Cell<u32>> = Rc::new(Cell::new(0));
    let counter: Rc<Cell<u32>> = Rc::clone(&hits);
    let _: usize = signal.subscribe(move || {
        counter.set(counter.get() + 1);
    });
    signal.unsubscribe(12345);
    signal.set(1);
    assert_eq!(hits.get(), 1, "an unknown handle must not detach anything");
}

#[test]
fn unsubscribing_twice_is_safe() {
    let signal: Signal<u32> = Signal::create(0);
    let hits: Rc<Cell<u32>> = Rc::new(Cell::new(0));
    let counter: Rc<Cell<u32>> = Rc::clone(&hits);
    let id: usize = signal.subscribe(move || {
        counter.set(counter.get() + 1);
    });
    signal.unsubscribe(id);
    signal.unsubscribe(id);
    signal.set(1);
    assert_eq!(hits.get(), 0, "a repeated detach changes nothing");
}

#[test]
fn a_fresh_signal_cell_holds_no_signal() {
    let cell: SignalCell<u32> = SignalCell::none();
    let observed: Option<Signal<u32>> = cell.loaded();
    assert_eq!(observed, None, "none() starts out empty");
}

#[test]
fn the_first_write_into_a_none_cell_wins() {
    let cell: SignalCell<u32> = SignalCell::none();
    let first: Signal<u32> = Signal::create(1);
    let second: Signal<u32> = Signal::create(2);
    cell.set(first);
    cell.set(second);
    let observed: Option<Signal<u32>> = cell.loaded();
    assert_eq!(observed, Some(first), "the second write is dropped");
}

#[test]
fn a_none_cell_reads_back_the_signal_it_was_given() {
    let cell: SignalCell<u32> = SignalCell::none();
    let signal: Signal<u32> = Signal::create(42);
    cell.set(signal);
    let observed: Option<Signal<u32>> = cell.loaded();
    assert_eq!(observed, Some(signal), "the handle comes back out");
}

#[test]
fn fire_at_invokes_the_closure_behind_the_handle() {
    let hits: Rc<Cell<u32>> = Rc::new(Cell::new(0));
    let counter: Rc<Cell<u32>> = Rc::clone(&hits);
    let handle: FireHandle = FireHandle::new(move || {
        counter.set(counter.get() + 1);
    });
    let address: usize = usize::from(handle);
    unsafe { FireHandle::fire_at(address) };
    assert_eq!(hits.get(), 1, "fire_at dispatches the leaked closure");
}

#[test]
fn fire_at_can_be_called_repeatedly_on_one_handle() {
    let hits: Rc<Cell<u32>> = Rc::new(Cell::new(0));
    let counter: Rc<Cell<u32>> = Rc::clone(&hits);
    let handle: FireHandle = FireHandle::new(move || {
        counter.set(counter.get() + 1);
    });
    let address: usize = usize::from(handle);
    for _ in 0..3 {
        unsafe { FireHandle::fire_at(address) };
    }
    assert_eq!(hits.get(), 3, "the handle stays live across dispatches");
}

#[test]
fn fire_at_runs_the_closure_the_handle_was_built_from() {
    let hits: Rc<Cell<u32>> = Rc::new(Cell::new(0));
    let counter: Rc<Cell<u32>> = Rc::clone(&hits);
    let handle: FireHandle = FireHandle::from(move || {
        counter.set(counter.get() + 1);
    });
    let address: usize = usize::from(handle);
    unsafe { FireHandle::fire_at(address) };
    assert_eq!(
        hits.get(),
        1,
        "the From path and the new path produce the same kind of handle"
    );
}
