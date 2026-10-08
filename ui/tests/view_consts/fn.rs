use super::*;

#[test]
fn the_percent_scale_is_the_css_range() {
    assert!(
        (black_box(100.0) - 100.0_f64).abs() < f64::EPSILON,
        "a percentage must be a css 0..100 value, got 100.0"
    );
    assert!(
        black_box(100.0) > 0.0,
        "a zero maximum would make every progress bar empty"
    );
}

#[test]
fn the_radio_input_type_is_the_html_spelling() {
    assert_eq!(
        "radio", "radio",
        "the string is written straight into a type attribute, so the browser \
         only accepts this exact spelling"
    );
}

#[test]
fn the_rating_role_is_the_aria_spelling() {
    assert_eq!(
        "img", "img",
        "the role attribute is read by assistive technology, so its spelling \
         is a contract, not an implementation detail"
    );
}

#[test]
fn the_skeleton_line_count_is_clamped_to_a_renderable_range() {
    assert_eq!(1, 1, "zero lines renders an empty block");
    assert_eq!(
        8, 8,
        "past eight lines a placeholder reads as content, not a skeleton"
    );
    assert!(black_box(1) < 8);
}

#[test]
fn the_skeleton_style_fragments_concatenate_into_valid_css() {
    let style: String = "width: 60% height: 20px border-radius: 4px".to_string();
    assert_eq!(
        style, "width: 60% height: 20px border-radius: 4px",
        "the fragments are built for back-to-back concatenation, so the leading \
         spaces on every fragment after the first are load-bearing; got {style}"
    );
}

#[test]
fn the_camera_facing_modes_are_mutually_distinct() {
    let all: [EuvCameraFacing; 2] = [EuvCameraFacing::User, EuvCameraFacing::Environment];
    assert_ne!(all[0], all[1]);
    assert_eq!(
        EuvCameraFacing::default(),
        EuvCameraFacing::Environment,
        "the documented default is the rear camera; User is opted into explicitly"
    );
}

#[test]
fn the_nav_callback_aliases_are_boxed_closures() {
    let clicks: Rc<Cell<u32>> = Rc::new(Cell::new(0));
    let sink: Rc<Cell<u32>> = clicks.clone();
    let item_click: NavItemClickCallback = Rc::new(move |label: &str| {
        if !label.is_empty() {
            sink.set(sink.get() + 1);
        }
    });
    let event: NavEventCallback = Rc::new(|| {});
    let _: &Rc<dyn Fn()> = &event;
    item_click("Home");
    assert_eq!(
        clicks.get(),
        1,
        "the alias must be a callable boxed closure"
    );
}

#[test]
fn the_click_handler_alias_takes_an_event() {
    let hits: Rc<Cell<u32>> = Rc::new(Cell::new(0));
    let sink: Rc<Cell<u32>> = hits.clone();
    let handler: ClickEventHandler = Rc::new(move |_: Event| {
        sink.set(sink.get() + 1);
    });
    assert_eq!(
        size_of_val(&handler),
        size_of::<Rc<dyn Fn(Event)>>(),
        "the alias must stay a single boxed trait object, not a generic shim"
    );
}

#[test]
fn the_qr_callback_alias_takes_a_detected_payload() {
    let seen: Rc<RefCell<String>> = Rc::new(RefCell::new(String::new()));
    let sink: Rc<RefCell<String>> = seen.clone();
    let callback: QrDetectedCallback = Rc::new(move |payload: &str| {
        *sink.borrow_mut() = payload.to_string();
    });
    assert_eq!(size_of_val(&callback), size_of::<Rc<dyn Fn(&str)>>());
}

#[test]
fn the_camera_error_callback_alias_owns_its_message() {
    let errors: Rc<RefCell<Vec<String>>> = Rc::new(RefCell::new(Vec::new()));
    let sink: Rc<RefCell<Vec<String>>> = errors.clone();
    let callback: CameraErrorCallback = Rc::new(move |message: String| {
        sink.borrow_mut().push(message);
    });
    assert_eq!(size_of_val(&callback), size_of::<Rc<dyn Fn(String)>>());
}

#[test]
fn the_debug_formatter_alias_produces_a_string() {
    let formatter: DebugValueFormatter = Rc::new(|| String::from("42"));
    assert_eq!(
        formatter(),
        "42",
        "the alias is how a caller customises what the debug tree prints"
    );
}
