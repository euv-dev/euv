use super::*;

#[wasm_bindgen_test]
fn app_is_zero_sized_struct() {
    assert_eq!(size_of::<App>(), 0);
    assert_eq!(align_of::<App>(), 1);
}

#[wasm_bindgen_test]
fn app_default_clone_copy_equality() {
    let a: App = App;
    let b: App = a;
    let c: App = App;
    assert_eq!(a, b);
    assert_eq!(a, c);
}

#[wasm_bindgen_test]
fn app_debug_format_is_informative() {
    let app: App = App;
    let debug: String = format!("{:?}", app);
    assert!(
        debug.contains("App"),
        "Debug output should mention the type name: {debug}"
    );
}

#[wasm_bindgen_test]
fn app_hash_is_consistent() {
    let mut h1: DefaultHasher = DefaultHasher::new();
    let mut h2: DefaultHasher = DefaultHasher::new();
    App.hash(&mut h1);
    App.hash(&mut h2);
    assert_eq!(h1.finish(), h2.finish());
}

#[wasm_bindgen_test]
fn app_ord_matches_eq() {
    let a: App = App;
    let b: App = App;
    assert_eq!(a.cmp(&b), Ordering::Equal);
}

#[wasm_bindgen_test]
fn app_use_signal_creates_signal_with_initial_value() {
    let context: HookContext = HookContext::default();
    let result: Signal<i32> = HookContext::with(context, || App::use_signal(|| 42));
    assert_eq!(result.get(), 42);
}

#[wasm_bindgen_test]
fn app_use_signal_with_string_value() {
    let context: HookContext = HookContext::default();
    let value: Signal<String> =
        HookContext::with(context, || App::use_signal(|| String::from("hello")));
    assert_eq!(value.get(), "hello");
}

#[wasm_bindgen_test]
fn app_use_signal_caches_at_same_hook_index_across_renders() {
    let context: HookContext = HookContext::default();
    let stored: Signal<i32> = HookContext::with(context.clone(), || App::use_signal(|| 7));
    assert_eq!(stored.get(), 7);
    let _ = stored;
    let mut context: HookContext = context;
    context.reset_index();
    let second: Signal<i32> = HookContext::with(context, || App::use_signal(|| 999));
    assert_eq!(
        second.get(),
        7,
        "Re-rendering at the same hook index must yield the cached signal value"
    );
}

#[wasm_bindgen_test]
fn app_use_cleanup_runs_on_switch_arm() {
    let context: HookContext = HookContext::default();
    let cleanup_ran: Rc<Cell<bool>> = Rc::new(Cell::new(false));
    let cleanup_ran_for_test: Rc<Cell<bool>> = cleanup_ran.clone();
    HookContext::with(context.clone(), || {
        let cleanup_ran_inner: Rc<Cell<bool>> = cleanup_ran_for_test.clone();
        App::use_cleanup(move || {
            cleanup_ran_inner.set(true);
        });
    });
    assert!(
        !cleanup_ran.get(),
        "Cleanup must not run while hook context stays alive"
    );
    let mut context: HookContext = context;
    context.switch_arm(1);
    assert!(
        cleanup_ran.get(),
        "switch_arm must invoke cleanups registered via App::use_cleanup"
    );
}

#[wasm_bindgen_test]
fn app_use_node_ref_returns_distinct_refs_per_hook_index() {
    let context: HookContext = HookContext::default();
    let (a, b): (NodeRef<JsValue>, NodeRef<JsValue>) = HookContext::with(context, || {
        let first: NodeRef<JsValue> = App::use_node_ref();
        let second: NodeRef<JsValue> = App::use_node_ref();
        (first, second)
    });
    assert!(a.get().is_none());
    assert!(b.get().is_none());
    assert!(
        !std::ptr::eq(&a as *const _, &b as *const _),
        "Distinct hook indices must yield distinct ref handles"
    );
}

#[wasm_bindgen_test]
fn app_batch_returns_closure_value() {
    let value: i32 = App::batch(|| 21 + 21);
    assert_eq!(value, 42);
}

#[wasm_bindgen_test]
fn app_batch_runs_closure_exactly_once() {
    let count: Rc<Cell<u32>> = Rc::new(Cell::new(0));
    let count_for_closure: Rc<Cell<u32>> = count.clone();
    App::batch(move || {
        count_for_closure.set(count_for_closure.get() + 1);
    });
    assert_eq!(
        count.get(),
        1,
        "App::batch must invoke the closure exactly once"
    );
}

#[wasm_bindgen_test]
fn app_schedule_update_signature_pin() {
    let _f: fn(&[usize]) = App::schedule_update;
}

#[wasm_bindgen_test]
fn app_use_window_event_signature_pin() {
    let _: fn() = || {
        App::use_window_event("resize", || {});
    };
}
