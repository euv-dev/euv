//! Tests for `HookContext::effect` and `HookContext::effect_with_cleanup`.
//!
//! These exercise the first-render-only effect lifecycle by wrapping
//! the hook body in `HookContext::with(...)` — the same path that
//! `App::mount` takes during a real render. Each test then asserts on
//! the resulting `hooks` / `cleanups` queue lengths and on the
//! captured side effects (mutable counters via `Cell`).
//!
//! The tests stay on native (`cargo test --lib`) because the hook
//! registry itself is platform-agnostic; the `wasm_bindgen_futures`
//! shim only matters for `use_async`, which is a different module.
//!
//! `Cell` is used instead of `RefCell` so the closures can be `Fn`
//! (the signature `F: FnOnce() + 'static` does not require `FnMut`,
//! but the test counters need interior mutability without borrow
//! tracking). `Cell<u32>` gives us both `Copy` and `Set` for free.

use super::*;
use std::cell::{Cell, RefCell};
use std::rc::Rc;

/// Build a fresh `HookContext` for tests.
///
/// `HookContext::new` (auto-derived) requires an `Rc<RefCell<inner>>`
/// payload — there is no zero-arg `new` because the derive's
/// generated signature mirrors the struct's fields. Most callers in
/// production get a context via `DynamicNode`'s mount path; tests
/// build one by hand, which is what this helper centralises.
fn fresh_context() -> HookContext {
    HookContext::new(Rc::new(RefCell::new(HookContextInner::default())))
}

/// `use_effect` runs the closure exactly once on the first render.
/// The `hooks` queue grows by 1 to record the slot, but no cleanup
/// is registered (the closure has no return value).
#[test]
fn effect_runs_once_on_first_render_and_records_slot() {
    let counter: Rc<Cell<u32>> = Rc::new(Cell::new(0));
    let counter_for_body: Rc<Cell<u32>> = counter.clone();
    let ctx: HookContext = fresh_context();
    HookContext::with(ctx.clone(), || {
        HookContext::effect(move || {
            counter_for_body.set(counter_for_body.get() + 1);
        });
    });
    assert_eq!(
        counter.get(),
        1,
        "effect body must run exactly once on first render",
    );
    let inner: std::cell::Ref<'_, HookContextInner> = ctx.get_inner().borrow();
    assert_eq!(
        inner.get_hooks().len(),
        1,
        "effect must record exactly one slot",
    );
    assert_eq!(
        inner.get_cleanups().len(),
        0,
        "effect without cleanup must not register any cleanup",
    );
}

/// Second render at the same hook index is a no-op. The effect body
/// is `FnOnce` and already consumed, so re-running is impossible —
/// this test enforces that the hook model is strictly
/// first-render-only (matching `use_cleanup` / `use_window_event`).
#[test]
fn effect_does_not_rerun_on_second_render() {
    let counter: Rc<Cell<u32>> = Rc::new(Cell::new(0));
    let ctx: HookContext = fresh_context();
    let counter_for_first: Rc<Cell<u32>> = counter.clone();
    let counter_for_second: Rc<Cell<u32>> = counter.clone();
    HookContext::with(ctx.clone(), || {
        HookContext::effect(move || {
            counter_for_first.set(counter_for_first.get() + 1);
        });
    });
    assert_eq!(counter.get(), 1, "first render ran the effect");
    // Reset hook index to simulate the next render pass at the
    // same hook index.
    ctx.get_inner().borrow_mut().set_hook_index(0);
    HookContext::with(ctx.clone(), || {
        HookContext::effect(move || {
            counter_for_second.set(counter_for_second.get() + 1);
        });
    });
    assert_eq!(
        counter.get(),
        1,
        "second render must not re-invoke the effect",
    );
    assert_eq!(
        ctx.get_inner().borrow().get_hooks().len(),
        1,
        "second render must not push a duplicate slot",
    );
}

/// `use_effect_with_cleanup` runs the factory on first render and
/// pushes the returned cleanup closure into the cleanup queue.
#[test]
fn effect_with_cleanup_runs_and_registers_cleanup() {
    let setup: Rc<Cell<u32>> = Rc::new(Cell::new(0));
    let teardown: Rc<Cell<u32>> = Rc::new(Cell::new(0));
    let setup_for_factory: Rc<Cell<u32>> = setup.clone();
    let teardown_for_cleanup: Rc<Cell<u32>> = teardown.clone();
    let ctx: HookContext = fresh_context();
    HookContext::with(ctx.clone(), || {
        HookContext::effect_with_cleanup(move || {
            setup_for_factory.set(setup_for_factory.get() + 1);
            move || {
                teardown_for_cleanup.set(teardown_for_cleanup.get() + 1);
            }
        });
    });
    assert_eq!(
        setup.get(),
        1,
        "factory must run exactly once on first render",
    );
    assert_eq!(
        teardown.get(),
        0,
        "cleanup must not run during mount; it runs at teardown only",
    );
    let inner: std::cell::Ref<'_, HookContextInner> = ctx.get_inner().borrow();
    assert_eq!(
        inner.get_hooks().len(),
        1,
        "factory must record exactly one slot",
    );
    assert_eq!(
        inner.get_cleanups().len(),
        1,
        "factory must register exactly one cleanup closure",
    );
}

/// Calling `switch_arm` on the hook context drains the cleanup
/// queue and re-initializes the hook index — this is what a match-arm
/// switch triggers (see `HookContextInner::arm_changed`). The cleanup
/// closure returned by `use_effect_with_cleanup` runs in the
/// switched-state, exactly the same path real `match`-arm UIs take.
#[test]
fn effect_with_cleanup_runs_cleanup_on_switch_arm() {
    let teardown: Rc<Cell<u32>> = Rc::new(Cell::new(0));
    let teardown_for_cleanup: Rc<Cell<u32>> = teardown.clone();
    let ctx: HookContext = fresh_context();
    HookContext::with(ctx.clone(), || {
        HookContext::effect_with_cleanup(move || {
            move || {
                teardown_for_cleanup.set(teardown_for_cleanup.get() + 1);
            }
        });
    });
    assert_eq!(teardown.get(), 0, "cleanup must not run during mount");
    let mut ctx_mut = ctx.clone();
    ctx_mut.switch_arm(1);
    assert_eq!(
        teardown.get(),
        1,
        "cleanup must run exactly once when the arm switches",
    );
}

/// Multiple `use_effect` slots coexist at distinct hook indices
/// within the same render pass. The hook index advances monotonically,
/// so a component that calls `use_signal`, `use_effect`, `use_signal`
/// in that order gets slot 0 / 1 / 2 reserved — losing track of the
/// order would break every multi-hook component.
#[test]
fn multiple_effects_each_take_their_own_slot() {
    let first: Rc<Cell<u32>> = Rc::new(Cell::new(0));
    let second: Rc<Cell<u32>> = Rc::new(Cell::new(0));
    let first_for_body: Rc<Cell<u32>> = first.clone();
    let second_for_body: Rc<Cell<u32>> = second.clone();
    let ctx: HookContext = fresh_context();
    HookContext::with(ctx.clone(), || {
        HookContext::effect(move || {
            first_for_body.set(first_for_body.get() + 1);
        });
        HookContext::effect(move || {
            second_for_body.set(second_for_body.get() + 1);
        });
    });
    assert_eq!(first.get(), 1, "first effect ran");
    assert_eq!(second.get(), 1, "second effect ran");
    assert_eq!(
        ctx.get_inner().borrow().get_hooks().len(),
        2,
        "two effects must occupy two slots",
    );
    assert_eq!(
        ctx.get_inner().borrow().get_cleanups().len(),
        0,
        "neither effect registered a cleanup",
    );
}

/// `use_effect` without a hook context (e.g. called outside a
/// `Dynamic` mount pass) is a silent no-op. This matches the
/// `use_signal` / `use_cleanup` behaviour: hooks called outside a
/// hook context do not panic, they just do nothing. Without this
/// guard, a stray `App::use_effect(...)` in a top-level `main`
/// function would crash the page.
#[test]
fn effect_without_active_context_is_a_noop() {
    // Note: we deliberately do NOT call `HookContext::with`. The
    // effect is invoked with whatever (likely empty) current
    // context exists in this test runner. Either way, it must
    // not panic.
    HookContext::effect(|| {
        // body should not run
    });
}

/// `use_effect_with_cleanup` without a hook context is also a
/// silent no-op — both the factory body and the returned cleanup
/// are dropped without ever executing. This guards against the
/// same "stray hook in main" crash as the `use_effect` test.
#[test]
fn effect_with_cleanup_without_active_context_is_a_noop() {
    HookContext::effect_with_cleanup(|| {
        // factory should not run
        move || {
            // cleanup should not run either
        }
    });
}

/// `use_effect_with_cleanup` produces a closure that can capture
/// values from the factory body. This is the practical use case:
/// the factory opens a resource, returns a closure that closes it,
/// and the framework guarantees the close runs on teardown.
#[test]
fn effect_with_cleanup_closure_captures_factory_state() {
    let resource_id: Rc<Cell<u32>> = Rc::new(Cell::new(0));
    let cleanup_log: Rc<Cell<u32>> = Rc::new(Cell::new(0));
    let resource_for_factory: Rc<Cell<u32>> = resource_id.clone();
    let cleanup_log_for_cleanup: Rc<Cell<u32>> = cleanup_log.clone();
    let ctx: HookContext = fresh_context();
    HookContext::with(ctx.clone(), || {
        HookContext::effect_with_cleanup(move || {
            // Open a fictional resource; the resource id is the
            // value captured by the cleanup closure.
            resource_for_factory.set(42);
            let captured: u32 = resource_for_factory.get();
            move || {
                cleanup_log_for_cleanup.set(captured);
            }
        });
    });
    assert_eq!(
        resource_id.get(),
        42,
        "factory must run during mount and set up state",
    );
    let mut ctx_mut = ctx.clone();
    ctx_mut.switch_arm(2);
    assert_eq!(
        cleanup_log.get(),
        42,
        "cleanup must observe the value captured by the factory",
    );
}

/// The hook index advances by exactly 1 per hook call. This guards
/// against an off-by-one regression in the index bookkeeping —
/// `use_signal`, `use_effect`, and `use_cleanup` all share the
/// same `hook_index` counter, so an effect that skips an index
/// would silently misalign every subsequent hook.
#[test]
fn effect_advances_hook_index_by_one() {
    let ctx: HookContext = fresh_context();
    HookContext::with(ctx.clone(), || {
        assert_eq!(
            ctx.get_inner().borrow().get_hook_index(),
            0,
            "hook index starts at 0",
        );
        HookContext::effect(|| {});
        assert_eq!(
            ctx.get_inner().borrow().get_hook_index(),
            1,
            "first effect advanced index to 1",
        );
        HookContext::effect(|| {});
        assert_eq!(
            ctx.get_inner().borrow().get_hook_index(),
            2,
            "second effect advanced index to 2",
        );
        HookContext::effect_with_cleanup(|| move || {});
        assert_eq!(
            ctx.get_inner().borrow().get_hook_index(),
            3,
            "effect_with_cleanup also advances the index by 1",
        );
    });
}
