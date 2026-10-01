use super::*;

fn new_counter() -> &'static Cell<i32> {
    static POOL: AtomicUsize = AtomicUsize::new(0);
    const CAP: usize = 16;
    static mut SLOTS: [Cell<i32>; CAP] = [const { Cell::new(0) }; CAP];
    let index: usize = POOL.fetch_add(1, Ordering::Relaxed) % CAP;
    unsafe {
        let slot: *mut Cell<i32> = &mut SLOTS[index];
        (*slot).set(0);
        &*slot
    }
}

#[test]
fn computed_macro_creates_signal_with_initial_value() {
    let hook_context: HookContext = HookContext::current();
    let source: Signal<i32> = Signal::create(3);
    let derived: Signal<i32> = HookContext::with(hook_context, || {
        computed!(source, |x: i32| -> i32 { x * 2 })
    });
    assert_eq!(derived.get(), 6);
}

#[test]
fn computed_macro_recomputes_on_source_change() {
    let hook_context: HookContext = HookContext::current();
    let source: Signal<i32> = Signal::create(3);
    let derived: Signal<i32> = HookContext::with(hook_context, || {
        computed!(source, |x: i32| -> i32 { x * 2 })
    });
    assert_eq!(derived.get(), 6, "initial value must be x * 2");
}

#[test]
fn computed_macro_supports_string_return() {
    let hook_context: HookContext = HookContext::current();
    let first: Signal<String> = Signal::create(String::from("hello"));
    let last: Signal<String> = Signal::create(String::from("world"));
    let full: Signal<String> = HookContext::with(hook_context, || {
        computed!(first, last, |f: String, l: String| -> String {
            format!("{f} {l}")
        })
    });
    assert_eq!(full.get(), "hello world");
}

#[test]
fn computed_macro_supports_anonymous_parameters() {
    let hook_context: HookContext = HookContext::current();
    let count: Signal<i32> = Signal::create(5);
    let derived: Signal<i32> = HookContext::with(hook_context, || {
        computed!(count, |_: i32| -> i32 { count.get() * 2 })
    });
    assert_eq!(derived.get(), 10);
}

#[test]
fn computed_macro_supports_multiple_inputs() {
    let hook_context: HookContext = HookContext::current();
    let a: Signal<i32> = Signal::create(2);
    let b: Signal<i32> = Signal::create(3);
    let sum: Signal<i32> = HookContext::with(hook_context, || {
        computed!(a, b, |x: i32, y: i32| -> i32 { x + y })
    });
    assert_eq!(sum.get(), 5);
}

#[test]
fn computed_macro_supports_unit_return() {
    let counter: &'static Cell<i32> = new_counter();
    let hook_context: HookContext = HookContext::current();
    let tick: Signal<u32> = Signal::create(0);
    let _: Signal<()> = HookContext::with(hook_context, || {
        computed!(tick, |_: u32| -> () {
            counter.set(counter.get() + 1);
        })
    });
    assert_eq!(
        counter.get(),
        1,
        "computed! body must run once for initial value"
    );
}

#[test]
fn computed_macro_body_can_use_let_statements() {
    let hook_context: HookContext = HookContext::current();
    let x: Signal<i32> = Signal::create(4);
    let derived: Signal<i32> = HookContext::with(hook_context, || {
        computed!(x, |v: i32| -> i32 {
            let doubled: i32 = v * 2;
            let plus_one: i32 = doubled + 1;
            plus_one
        })
    });
    assert_eq!(derived.get(), 9);
}
