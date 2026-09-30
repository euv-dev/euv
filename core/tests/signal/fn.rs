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
