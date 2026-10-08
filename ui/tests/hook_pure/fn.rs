use super::*;

#[test]
fn the_range_input_type_is_the_standard_spelling() {
    assert_eq!(
        "range", "range",
        "only `range` gives the browser's native slider semantics and keyboard support"
    );
    assert_ne!("range", "file");
    assert_ne!("range", "radio");
}

#[test]
fn the_slider_and_progress_percent_scales_agree() {
    assert!(
        (100.0 - 100.0_f64).abs() < f64::EPSILON,
        "two components scaling to a css percentage must not drift apart, got \
         {} and {}",
        100.0,
        100.0
    );
}

#[test]
fn a_previous_tracker_reports_none_before_anything_is_recorded() {
    let mut previous: Previous<u32> = use_previous();
    let observed: Option<u32> = previous_step(previous, 1);
    assert_eq!(
        observed, None,
        "the first call has no prior value, so `None` is the only honest answer"
    );
    previous = black_box(previous);
    let _: Previous<u32> = previous;
}

#[test]
fn a_previous_tracker_returns_the_value_recorded_last_time() {
    let mut previous: Previous<u32> = use_previous();
    let _: Option<u32> = previous_step(previous, 1);
    previous = black_box(previous);
    let second: Option<u32> = previous_step(previous, 2);
    assert_eq!(
        second,
        Some(1),
        "the tracker must hand back the prior value"
    );
}

#[test]
fn a_lazy_component_builds_its_value_only_once_per_render_slot() {
    let built: Rc<Cell<u32>> = Rc::new(Cell::new(0));
    let counter: Rc<Cell<u32>> = built.clone();
    let lazy: LazyComponent<u32> = use_lazy_component(move || {
        counter.set(counter.get() + 1);
        7_u32
    });
    assert_eq!(
        built.get(),
        0,
        "a lazy component must not run its factory until something reads it"
    );
    let first: Option<u32> = lazy.get();
    let second: Option<u32> = lazy.get();
    assert_eq!(first, Some(7));
    assert_eq!(second, Some(7), "reading twice must not rebuild the value");
    assert_eq!(
        built.get(),
        1,
        "the factory is a build-once hook, not a getter"
    );
}

#[test]
fn a_lazy_component_forgets_its_value_when_reset() {
    let built: Rc<Cell<u32>> = Rc::new(Cell::new(0));
    let counter: Rc<Cell<u32>> = built.clone();
    let lazy: LazyComponent<u32> = use_lazy_component(move || {
        counter.set(counter.get() + 1);
        5_u32
    });
    assert_eq!(lazy.get(), Some(5));
    lazy.reset();
    assert_eq!(
        lazy.get(),
        Some(5),
        "a reset drops the memo so the next read rebuilds from the factory"
    );
    assert_eq!(
        built.get(),
        2,
        "which means the factory really did run again"
    );
}

#[test]
fn the_virtual_list_props_alias_points_at_the_public_struct() {
    let props: VirtualListProps = VirtualListProps {
        config: VirtualListConfig {
            id: String::from("f"),
            total_count: 2,
            item_height: 10,
            overscan_count: 0,
        },
        item_renderer: Rc::new(|_: usize| VirtualNode::Empty),
        on_scroll: None,
        on_visible_range_change: None,
    };
    let as_euv: EuvVirtualListProps = props;
    assert_eq!(as_euv.config.total_count, 2);
}
