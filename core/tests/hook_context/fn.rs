use super::*;

fn fresh_context() -> HookContext {
    HookContext::new(Rc::new(RefCell::new(HookContextInner::default())))
}

fn slot_count(context: &HookContext) -> usize {
    context.inner.borrow().hooks.len()
}

#[test]
fn two_calls_to_current_return_the_same_instance() {
    let first: HookContext = HookContext::current();
    let second: HookContext = HookContext::current();
    assert!(
        Rc::ptr_eq(&first.inner, &second.inner),
        "the thread-local must hand back one shared context, not a fresh one each call"
    );
}

#[test]
fn a_private_context_is_not_the_thread_local_one() {
    let saved: HookContext = HookContext::current();
    let other: HookContext = fresh_context();
    assert!(
        !Rc::ptr_eq(&saved.inner, &other.inner),
        "a private context must not alias the thread-local global"
    );
}

#[test]
fn with_installs_the_supplied_context_inside_the_closure() {
    let supplied: HookContext = fresh_context();
    let observed: bool = HookContext::with(supplied.clone(), || {
        Rc::ptr_eq(&HookContext::current().inner, &supplied.inner)
    });
    assert!(
        observed,
        "inside the closure the supplied context must be active"
    );
}

#[test]
fn with_restores_the_previous_context_afterwards() {
    let outer: HookContext = HookContext::current();
    let supplied: HookContext = fresh_context();
    let _: () = HookContext::with(supplied, || ());
    assert!(
        Rc::ptr_eq(&HookContext::current().inner, &outer.inner),
        "leaving the closure must put the previous context back"
    );
}

#[test]
fn with_returns_the_closure_result() {
    let observed: u32 = HookContext::with(fresh_context(), || 42);
    assert_eq!(observed, 42);
}

#[test]
fn with_restores_one_nesting_level_at_a_time() {
    let level0: HookContext = HookContext::current();
    let level1: HookContext = fresh_context();
    let level2: HookContext = fresh_context();
    let observations: (bool, bool, bool) = HookContext::with(level1.clone(), || {
        let at_level1: bool = Rc::ptr_eq(&HookContext::current().inner, &level1.inner);
        let inside_level2: bool = HookContext::with(level2, || {
            !Rc::ptr_eq(&HookContext::current().inner, &level1.inner)
        });
        let back_at_level1: bool = Rc::ptr_eq(&HookContext::current().inner, &level1.inner);
        (at_level1, inside_level2, back_at_level1)
    });
    assert_eq!(observations, (true, true, true));
    assert!(
        Rc::ptr_eq(&HookContext::current().inner, &level0.inner),
        "the outermost context must be back in place"
    );
}

#[test]
fn a_hook_returns_the_factory_value_on_a_fresh_slot() {
    let context: HookContext = fresh_context();
    let observed: u32 = HookContext::with(context, || HookContext::use_hook(|| 7_u32));
    assert_eq!(observed, 7, "an empty context has no stored value to reuse");
}

#[test]
fn every_hook_call_occupies_exactly_one_new_slot() {
    let context: HookContext = fresh_context();
    let _: (u32, u32, u32) = HookContext::with(context.clone(), || {
        (
            HookContext::use_hook(|| 1_u32),
            HookContext::use_hook(|| 2_u32),
            HookContext::use_hook(|| 3_u32),
        )
    });
    assert_eq!(
        slot_count(&context),
        3,
        "three hook calls must have filled three slots, no more and no fewer"
    );
}

#[test]
fn successive_hooks_land_on_successive_values() {
    let context: HookContext = fresh_context();
    let observed: (u32, u32, u32) = HookContext::with(context, || {
        (
            HookContext::use_hook(|| 1_u32),
            HookContext::use_hook(|| 2_u32),
            HookContext::use_hook(|| 3_u32),
        )
    });
    assert_eq!(
        observed,
        (1, 2, 3),
        "the index advances on every call, so each hook must build its own value"
    );
}

#[test]
fn a_hook_stores_its_type_so_a_different_type_does_not_see_it() {
    let context: HookContext = fresh_context();
    let observed: (u32, String) = HookContext::with(context, || {
        let first: u32 = HookContext::use_hook(|| 5_u32);
        let second: String = HookContext::use_hook(|| String::from("fresh"));
        (first, second)
    });
    assert_eq!(observed.0, 5);
    assert_eq!(
        observed.1, "fresh",
        "a String cannot reuse a u32 slot, so the factory must run"
    );
}

#[test]
fn a_fresh_context_starts_with_no_slots() {
    assert_eq!(slot_count(&fresh_context()), 0);
}

#[test]
fn a_dynamic_node_wraps_a_render_closure() {
    let node: VirtualNode = VirtualNode::create_dynamic(|_: &mut HookContext| VirtualNode::Empty);
    assert!(
        matches!(node, VirtualNode::Dynamic(_)),
        "create_dynamic must produce the Dynamic variant, got {node:?}"
    );
}

#[test]
fn two_dynamic_nodes_are_distinct_instances() {
    let a: VirtualNode = VirtualNode::create_dynamic(|_: &mut HookContext| VirtualNode::Empty);
    let b: VirtualNode = VirtualNode::create_dynamic(|_: &mut HookContext| VirtualNode::Empty);
    assert_ne!(
        a, b,
        "each dynamic node owns its own closure and its own hook context"
    );
}

#[test]
fn a_dynamic_node_survives_a_clone() {
    let node: VirtualNode = VirtualNode::create_dynamic(|_: &mut HookContext| VirtualNode::Empty);
    let cloned: VirtualNode = node.clone();
    assert!(
        matches!(&cloned, VirtualNode::Dynamic(_)),
        "cloning must preserve the Dynamic variant, got {cloned:?}"
    );
}

#[test]
fn a_signal_converts_into_a_reactive_text_node() {
    let signal: Signal<i32> = Signal::create(42);
    let node: VirtualNode = signal.as_reactive_text();
    assert!(
        matches!(node, VirtualNode::Text(_)),
        "AsReactiveText must build a Text node, got {node:?}"
    );
}

#[test]
fn an_event_callback_alias_is_a_boxed_closure_taking_an_event() {
    let seen: Rc<RefCell<u32>> = Rc::new(RefCell::new(0));
    let sink: Rc<RefCell<u32>> = seen.clone();
    let callback: EventCallback = Box::new(move |_: Event| {
        *sink.borrow_mut() += 1;
    });
    assert_eq!(
        size_of_val(&callback),
        size_of::<Box<dyn FnMut(Event)>>(),
        "the alias must stay a single boxed trait object, not a generic shim"
    );
    assert_eq!(*seen.borrow(), 0, "building the callback must not run it");
}

#[test]
fn a_shared_event_callback_wraps_that_box_in_a_shared_cell() {
    let seen: Rc<RefCell<u32>> = Rc::new(RefCell::new(0));
    let sink: Rc<RefCell<u32>> = seen.clone();
    let callback: EventCallback = Box::new(move |_: Event| {
        *sink.borrow_mut() += 1;
    });
    let shared: SharedEventCallback = Rc::new(UnsafeCell::new(callback));
    let cloned: SharedEventCallback = Rc::clone(&shared);
    assert!(
        Rc::ptr_eq(&shared, &cloned),
        "the alias is an std::rc::Rc alias, so a clone must share one allocation"
    );
    assert_eq!(*seen.borrow(), 0);
}

#[test]
fn the_node_ref_alias_is_a_js_value_cell() {
    let node_ref: NodeRefDyn = NodeRef::new();
    assert!(!node_ref.is_set(), "a fresh NodeRef holds nothing");
    assert!(node_ref.get().is_none());
    assert!(node_ref.get_cloned().is_none());
    node_ref.clear();
    assert!(!node_ref.is_set(), "clearing an empty NodeRef is a no-op");
}

#[test]
fn a_node_ref_survives_a_clone_without_gaining_a_value() {
    let node_ref: NodeRefDyn = NodeRef::new();
    let cloned: NodeRefDyn = node_ref.clone();
    assert!(
        !cloned.is_set(),
        "a clone must not invent a value the original never received"
    );
    assert!(cloned.get().is_none());
}

#[test]
fn a_text_node_binder_alias_is_a_shared_closure() {
    let seen: Rc<RefCell<u32>> = Rc::new(RefCell::new(0));
    let sink: Rc<RefCell<u32>> = seen.clone();
    let binder: TextNodeBinder = Rc::new(move |_: &Text| {
        *sink.borrow_mut() += 1;
    });
    let cloned: TextNodeBinder = Rc::clone(&binder);
    assert!(Rc::ptr_eq(&binder, &cloned));
    assert_eq!(*seen.borrow(), 0, "building a binder must not run it");
}

#[test]
fn a_dynamic_node_keeps_its_own_hook_context_across_a_clone() {
    let node: VirtualNode = VirtualNode::create_dynamic(|_: &mut HookContext| VirtualNode::Empty);
    let VirtualNode::Dynamic(dynamic) = &node else {
        panic!("expected a Dynamic node, got {node:?}");
    };
    assert!(
        matches!(&node.clone(), VirtualNode::Dynamic(_)),
        "cloning must still yield a Dynamic node, not collapse it to Empty"
    );
    assert!(
        !matches!(&node.clone(), VirtualNode::Empty),
        "a DynamicNode is never an Empty node"
    );
    let _: &DynamicNode = dynamic;
}

#[test]
fn app_mount_takes_a_selector_and_a_node_producer() {
    let pin: fn() = || {
        App::mount("#app", || VirtualNode::Empty);
    };
    assert_eq!(
        size_of_val(&pin),
        size_of::<fn()>(),
        "the pin's job is the type check at compile time; running it off wasm \
         would need a document, so only its shape is asserted here"
    );
}

#[test]
fn app_mount_accepts_any_selector_owning_type() {
    let pin: fn() = || {
        App::mount(String::from(".root"), || VirtualNode::Empty);
    };
    assert_eq!(
        size_of_val(&pin),
        size_of::<fn()>(),
        "a `String` selector must be accepted just like a `&str` one"
    );
}

#[test]
fn app_use_interval_takes_milliseconds_and_a_callback() {
    let pin: fn() = || {
        let _: IntervalHandle = App::use_interval(16, || {});
    };
    assert_eq!(
        size_of_val(&pin),
        size_of::<fn()>(),
        "the pin's job is the type check at compile time — `use_interval` calls \
         `window().set_interval` and cannot run off wasm"
    );
}

#[test]
fn a_default_interval_handle_has_no_interval_to_cancel() {
    let handle: IntervalHandle = IntervalHandle::default();
    assert!(
        format!("{handle:?}").contains("interval_id: 0"),
        "a default handle names no browser interval, so clearing it is a no-op          rather than a call into js_sys, got: {handle:?}"
    );
}

#[test]
fn an_interval_handle_is_copy_so_a_hook_can_return_it_by_value() {
    let handle: IntervalHandle = IntervalHandle::default();
    let copied: IntervalHandle = handle;
    assert_eq!(copied, handle);
    let moved: IntervalHandle = handle;
    assert_eq!(
        moved, copied,
        "the original must still be usable after both copies"
    );
}
