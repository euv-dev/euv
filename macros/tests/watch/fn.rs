#![allow(unreachable_code)]

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
fn watch_macro_runs_initial_body_under_hook_context() {
    let counter: &'static Cell<i32> = new_counter();
    let hook_context: HookContext = HookContext::current();
    let source: Signal<i32> = Signal::create(7);
    let body_panicked: bool = catch_unwind(AssertUnwindSafe(|| {
        HookContext::with(hook_context, || {
            watch!(source, |value: i32| {
                counter.set(counter.get() + 1);
                if value != 7 {
                    panic!("watch arg != source.get(): got {value}");
                }
                panic!("body ran for value={value}");
            });
        });
    }))
    .is_err();
    assert!(body_panicked, "watch body must have executed and panicked");
    assert_eq!(
        counter.get(),
        1,
        "watch body must run exactly once per hook context"
    );
    assert_eq!(source.get(), 7);
}

#[test]
fn watch_macro_supports_multiple_signal_sources() {
    let counter: &'static Cell<i32> = new_counter();
    let hook_context: HookContext = HookContext::current();
    let a: Signal<i32> = Signal::create(1);
    let b: Signal<i32> = Signal::create(2);
    let body_panicked: bool = catch_unwind(AssertUnwindSafe(|| {
        HookContext::with(hook_context, || {
            watch!(a, b, |x: i32, y: i32| {
                counter.set(counter.get() + 1);
                assert_eq!(x, 1, "first watch arg must equal a.get()");
                assert_eq!(y, 2, "second watch arg must equal b.get()");
                panic!("body ran for x={x},y={y}");
            });
        });
    }))
    .is_err();
    assert!(body_panicked);
    assert_eq!(counter.get(), 1);
    assert_eq!(a.get(), 1);
    assert_eq!(b.get(), 2);
}

#[test]
fn watch_macro_supports_anonymous_parameters() {
    let counter: &'static Cell<i32> = new_counter();
    let hook_context: HookContext = HookContext::current();
    let s: Signal<i32> = Signal::create(11);
    let body_panicked: bool = catch_unwind(AssertUnwindSafe(|| {
        HookContext::with(hook_context, || {
            watch!(s, |_: i32| {
                counter.set(counter.get() + 1);
                unreachable!("body ran with anonymous param");
                counter.set(counter.get());
            });
        });
    }))
    .is_err();
    assert!(body_panicked);
    assert_eq!(counter.get(), 1);
}

#[test]
fn watch_macro_expansion_evaluates_to_unit() {
    let _: () = {
        let hook_context: HookContext = HookContext::current();
        let s: Signal<i32> = Signal::create(0);
        let counter: &'static Cell<i32> = new_counter();
        let body_panicked: bool = catch_unwind(AssertUnwindSafe(|| {
            HookContext::with(hook_context, || {
                watch!(s, |v: i32| {
                    counter.set(counter.get() + 1);
                    let _: i32 = v;
                    panic!("unit body ran with v={v}");
                });
            });
        }))
        .is_err();
        assert!(body_panicked, "unit-returning body must execute");
        assert_eq!(
            counter.get(),
            1,
            "unit-returning body must run exactly once"
        );
    };
}

#[test]
fn watch_macro_does_not_move_signal_expression() {
    let counter: &'static Cell<i32> = new_counter();
    let hook_context: HookContext = HookContext::current();
    let source: Signal<i32> = Signal::create(13);
    let source_addr_before: usize = source.get_inner();
    let body_panicked: bool = catch_unwind(AssertUnwindSafe(|| {
        HookContext::with(hook_context, || {
            watch!(source, |v: i32| {
                counter.set(counter.get() + 1);
                assert_eq!(v, 13);
                panic!("body ran with v={v}");
            });
        });
    }))
    .is_err();
    assert!(body_panicked);
    assert_eq!(counter.get(), 1);
    assert_eq!(source.get_inner(), source_addr_before);
    assert_eq!(source.get(), 13);
}

#[test]
fn watch_macro_body_does_not_run_again_on_a_re_render() {
    let counter: &'static Cell<i32> = new_counter();
    let mut hook_context: HookContext = HookContext::current();
    let source: Signal<i32> = Signal::create(3);
    HookContext::with(hook_context.clone(), || {
        watch!(source, |value: i32| {
            counter.set(counter.get() + 1);
            assert_eq!(value, 3, "the body sees the watched signal's value");
        });
    });
    assert_eq!(counter.get(), 1, "the body runs once when the hook mounts");
    hook_context.reset_index();
    HookContext::with(hook_context.clone(), || {
        watch!(source, |value: i32| {
            counter.set(counter.get() + 1);
            assert_eq!(value, 3, "the body sees the watched signal's value");
        });
    });
    assert_eq!(
        counter.get(),
        1,
        "a re-render replays the hook list from the same hook context, so \
         the body must not run a second time: a component that acts on \
         every render would act twice, which is how a scroll-to-anchor \
         hook threw the reader back to the top of the page on a theme \
         switch"
    );
}

#[test]
fn watch_macro_body_runs_again_when_the_watched_signal_changes() {
    let counter: &'static Cell<i32> = new_counter();
    let hook_context: HookContext = HookContext::current();
    let source: Signal<i32> = Signal::create(3);
    HookContext::with(hook_context.clone(), || {
        watch!(source, |value: i32| {
            counter.set(counter.get() + 1);
            assert_eq!(
                value,
                source.get(),
                "the body sees the watched signal's value"
            );
        });
    });
    assert_eq!(counter.get(), 1, "the body runs once when the hook mounts");
    source.set(9);
    assert_eq!(
        counter.get(),
        2,
        "a changed signal must run the body again, or a component that \
         reacts to a route change would never fire"
    );
}
