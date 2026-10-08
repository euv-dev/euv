use super::*;

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
#[cfg_attr(
    not(target_arch = "wasm32"),
    ignore = "reaches the no-DOM fallback only under wasm"
)]
fn a_router_without_a_window_reports_a_desktop_viewport() {
    let observed: bool = Router::is_mobile();
    assert!(
        !observed,
        "with no window there is no viewport to be narrow, so mobile must be false"
    );
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
#[cfg_attr(
    not(target_arch = "wasm32"),
    ignore = "reaches the no-DOM fallback only under wasm"
)]
fn a_router_without_a_window_has_an_empty_current_route() {
    let observed: String = Router::current_route();
    assert_eq!(
        observed, "",
        "a missing window yields no location hash, so the route is empty"
    );
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
#[cfg_attr(
    not(target_arch = "wasm32"),
    ignore = "reaches the no-DOM fallback only under wasm"
)]
fn a_missing_media_query_falls_back_to_the_light_theme() {
    let observed: String = ThemeState::detect_system_theme();
    assert_eq!(
        observed, "light",
        "no matchMedia means no dark preference can be proven, so light is the safe default"
    );
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
#[cfg_attr(
    not(target_arch = "wasm32"),
    ignore = "reaches the no-DOM fallback only under wasm"
)]
fn a_theme_change_hook_leaves_the_signal_alone_without_a_window() {
    let theme: Signal<String> = Signal::create(String::from("light"));
    ThemeState::use_system_theme_change(theme);
    assert_eq!(
        theme.get(),
        "light",
        "the hook must not write a value it could not have observed from a media query"
    );
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
#[cfg_attr(
    not(target_arch = "wasm32"),
    ignore = "reaches the no-DOM fallback only under wasm"
)]
fn a_local_storage_read_without_a_window_finds_nothing() {
    let observed: Option<String> = UseEuvBrowser::local_storage_get("any-key");
    assert_eq!(
        observed, None,
        "there is no storage without a window, so every key must read back as absent"
    );
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
#[cfg_attr(
    not(target_arch = "wasm32"),
    ignore = "reaches the no-DOM fallback only under wasm"
)]
fn a_local_storage_write_without_a_window_is_a_no_op() {
    UseEuvBrowser::local_storage_set("k", "v");
    let observed: Option<String> = UseEuvBrowser::local_storage_get("k");
    assert_eq!(
        observed, None,
        "a write that had nowhere to land must not later read back as present"
    );
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
#[cfg_attr(
    not(target_arch = "wasm32"),
    ignore = "reaches the no-DOM fallback only under wasm"
)]
fn a_browser_state_starts_with_every_input_empty() {
    let state: UseEuvBrowser = UseEuvBrowser::use_browser_state();
    assert!(
        state.local_key.get().is_empty() && state.local_value.get().is_empty(),
        "the local storage inputs must start blank, got {:?} / {:?}",
        state.local_key.get(),
        state.local_value.get()
    );
    assert!(
        state.session_key.get().is_empty() && state.session_value.get().is_empty(),
        "the session storage inputs must start blank, got {:?} / {:?}",
        state.session_key.get(),
        state.session_value.get()
    );
    assert!(
        state.user_agent.get().is_empty()
            && state.language.get().is_empty()
            && state.location_url.get().is_empty(),
        "nothing about the host can be known before a window exists"
    );
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
#[cfg_attr(
    not(target_arch = "wasm32"),
    ignore = "reaches the no-DOM fallback only under wasm"
)]
fn a_console_log_handler_is_built_even_without_a_window() {
    let input: Signal<String> = Signal::create(String::from("hello"));
    let handler: Option<Rc<dyn Fn(Event)>> = UseEuvBrowser::on_console_log(input);
    assert!(
        handler.is_some(),
        "building the handler touches no DOM, so it must be available without a window"
    );
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
#[cfg_attr(
    not(target_arch = "wasm32"),
    ignore = "reaches the no-DOM fallback only under wasm"
)]
fn a_storage_get_handler_reports_a_key_it_could_not_read() {
    let state: UseEuvBrowser = UseEuvBrowser::use_browser_state();
    state.local_key.set(String::from("missing"));
    let handler: Rc<dyn Fn(Event)> = UseEuvBrowser::on_local_storage_get(state)
        .expect("the handler is built without touching the dom");
    handler(Event::new("click").expect("a click event"));
    assert_eq!(
        state.local_result.get(),
        "Key 'missing' not found",
        "a key that reads back absent must say so rather than staying blank"
    );
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
#[cfg_attr(
    not(target_arch = "wasm32"),
    ignore = "reaches the no-DOM fallback only under wasm"
)]
fn a_storage_set_handler_reports_the_write_even_though_nothing_landed() {
    let state: UseEuvBrowser = UseEuvBrowser::use_browser_state();
    state.local_key.set(String::from("k"));
    state.local_value.set(String::from("v"));
    let handler: Rc<dyn Fn(Event)> = UseEuvBrowser::on_local_storage_set(state)
        .expect("the handler is built without touching the dom");
    handler(Event::new("click").expect("a click event"));
    assert_eq!(
        state.local_result.get(),
        "Set: k = v",
        "the result line reports the intent regardless of whether storage existed"
    );
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
#[cfg_attr(
    not(target_arch = "wasm32"),
    ignore = "reaches the no-DOM fallback only under wasm"
)]
fn a_storage_set_handler_skips_a_blank_key_entirely() {
    let state: UseEuvBrowser = UseEuvBrowser::use_browser_state();
    let handler: Rc<dyn Fn(Event)> = UseEuvBrowser::on_local_storage_set(state)
        .expect("the handler is built without touching the dom");
    handler(Event::new("click").expect("a click event"));
    assert!(
        state.local_result.get().is_empty(),
        "an empty key is not a write, so it must leave the result untouched"
    );
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
#[cfg_attr(
    not(target_arch = "wasm32"),
    ignore = "reaches the no-DOM fallback only under wasm"
)]
fn a_storage_remove_handler_reports_the_key_it_targeted() {
    let state: UseEuvBrowser = UseEuvBrowser::use_browser_state();
    state.local_key.set(String::from("gone"));
    let handler: Rc<dyn Fn(Event)> = UseEuvBrowser::on_local_storage_remove(state)
        .expect("the handler is built without touching the dom");
    handler(Event::new("click").expect("a click event"));
    assert_eq!(
        state.local_result.get(),
        "Removed key: gone",
        "the removal must name the key so the caller can tell what it removed"
    );
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
#[cfg_attr(
    not(target_arch = "wasm32"),
    ignore = "reaches the no-DOM fallback only under wasm"
)]
fn a_copy_handler_refuses_to_copy_nothing_before_it_reaches_the_dom() {
    let state: UseEuvBrowser = UseEuvBrowser::use_browser_state();
    let handler: Rc<dyn Fn(Event)> = UseEuvBrowser::on_clipboard_copy(state)
        .expect("the handler is built without touching the dom");
    handler(Event::new("click").expect("a click event"));
    assert_eq!(
        state.clipboard_result.get(),
        "Please enter text to copy",
        "an empty clipboard must be refused without ever touching navigator.clipboard"
    );
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
#[cfg_attr(
    not(target_arch = "wasm32"),
    ignore = "reaches the no-DOM fallback only under wasm"
)]
fn a_copy_handler_with_real_text_reports_nothing_without_a_window() {
    let state: UseEuvBrowser = UseEuvBrowser::use_browser_state();
    state.clipboard_text.set(String::from("some text"));
    let handler: Rc<dyn Fn(Event)> = UseEuvBrowser::on_clipboard_copy(state)
        .expect("the handler is built without touching the dom");
    handler(Event::new("click").expect("a click event"));
    assert!(
        state.clipboard_result.get().is_empty(),
        "with no clipboard to write to, the result must stay blank rather than claim success"
    );
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
#[cfg_attr(
    not(target_arch = "wasm32"),
    ignore = "reaches the no-DOM fallback only under wasm"
)]
fn a_paste_handler_without_a_window_leaves_the_result_blank() {
    let state: UseEuvBrowser = UseEuvBrowser::use_browser_state();
    let handler: Rc<dyn Fn(Event)> = UseEuvBrowser::on_clipboard_paste(state)
        .expect("the handler is built without touching the dom");
    handler(Event::new("click").expect("a click event"));
    assert!(
        state.clipboard_result.get().is_empty(),
        "no clipboard means no paste, and no paste must not read as a pasted message"
    );
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
#[cfg_attr(
    not(target_arch = "wasm32"),
    ignore = "reaches the no-DOM fallback only under wasm"
)]
fn a_window_size_refresh_without_a_window_reports_a_zero_viewport() {
    let state: UseEuvBrowser = UseEuvBrowser::use_browser_state();
    let handler: Rc<dyn Fn(Event)> = UseEuvBrowser::on_window_refresh_size(state)
        .expect("the handler is built without touching the dom");
    handler(Event::new("click").expect("a click event"));
    assert_eq!(
        state.window_size.get(),
        "0 x 0",
        "a missing viewport is zero by zero, and that is what the display must show"
    );
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
#[cfg_attr(
    not(target_arch = "wasm32"),
    ignore = "reaches the no-DOM fallback only under wasm"
)]
fn every_console_level_handler_is_built_without_a_window() {
    let input: Signal<String> = Signal::create(String::from("msg"));
    let log: Option<Rc<dyn Fn(Event)>> = UseEuvBrowser::on_console_log(input);
    let warn: Option<Rc<dyn Fn(Event)>> = UseEuvBrowser::on_console_warn(input);
    let error: Option<Rc<dyn Fn(Event)>> = UseEuvBrowser::on_console_error(input);
    assert!(
        log.is_some() && warn.is_some() && error.is_some(),
        "console levels are wired at mount time, not when a window appears"
    );
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
#[cfg_attr(
    not(target_arch = "wasm32"),
    ignore = "reaches the no-DOM fallback only under wasm"
)]
fn a_session_get_handler_reports_a_key_it_could_not_read() {
    let state: UseEuvBrowser = UseEuvBrowser::use_browser_state();
    state.session_key.set(String::from("absent"));
    let handler: Rc<dyn Fn(Event)> = UseEuvBrowser::on_session_storage_get(state)
        .expect("the handler is built without touching the dom");
    handler(Event::new("click").expect("a click event"));
    assert_eq!(
        state.session_result.get(),
        "Key 'absent' not found",
        "a session key that reads back absent must be named in the result"
    );
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
#[cfg_attr(
    not(target_arch = "wasm32"),
    ignore = "reaches the no-DOM fallback only under wasm"
)]
fn a_session_set_handler_reports_the_write_even_though_nothing_landed() {
    let state: UseEuvBrowser = UseEuvBrowser::use_browser_state();
    state.session_key.set(String::from("sk"));
    state.session_value.set(String::from("sv"));
    let handler: Rc<dyn Fn(Event)> = UseEuvBrowser::on_session_storage_set(state)
        .expect("the handler is built without touching the dom");
    handler(Event::new("click").expect("a click event"));
    assert_eq!(
        state.session_result.get(),
        "Set: sk = sv",
        "session writes report the intent the same way local writes do"
    );
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
#[cfg_attr(
    not(target_arch = "wasm32"),
    ignore = "reaches the no-DOM fallback only under wasm"
)]
fn a_session_set_handler_skips_a_blank_key_entirely() {
    let state: UseEuvBrowser = UseEuvBrowser::use_browser_state();
    let handler: Rc<dyn Fn(Event)> = UseEuvBrowser::on_session_storage_set(state)
        .expect("the handler is built without touching the dom");
    handler(Event::new("click").expect("a click event"));
    assert!(
        state.session_result.get().is_empty(),
        "an empty key is not a session write either"
    );
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
#[cfg_attr(
    not(target_arch = "wasm32"),
    ignore = "reaches the no-DOM fallback only under wasm"
)]
fn a_session_remove_handler_reports_the_key_it_targeted() {
    let state: UseEuvBrowser = UseEuvBrowser::use_browser_state();
    state.session_key.set(String::from("sk"));
    let handler: Rc<dyn Fn(Event)> = UseEuvBrowser::on_session_storage_remove(state)
        .expect("the handler is built without touching the dom");
    handler(Event::new("click").expect("a click event"));
    assert_eq!(
        state.session_result.get(),
        "Removed key: sk",
        "the session removal must name its key just like the local one"
    );
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
#[cfg_attr(
    not(target_arch = "wasm32"),
    ignore = "reaches the no-DOM fallback only under wasm"
)]
fn a_close_handler_marks_the_camera_shut_without_a_video_element() {
    let state: UseEuvCamera = UseEuvCamera::new(
        Signal::create(true),
        Signal::create(true),
        Signal::create(String::from("busy")),
        Signal::create(EuvCameraFacing::default()),
        Signal::create(String::from("qr")),
        Signal::create(None),
    );
    let handler: Rc<dyn Fn(Event)> =
        UseEuvCamera::on_close(state, None).expect("the handler is built without touching the dom");
    handler(Event::new("click").expect("a click event"));
    assert!(!state.camera_open.get(), "closing must clear the open flag");
    assert!(
        state.scan_result.get().is_empty(),
        "a shut camera has no scan result left to show"
    );
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
#[cfg_attr(
    not(target_arch = "wasm32"),
    ignore = "reaches the no-DOM fallback only under wasm"
)]
fn an_open_handler_reports_a_failure_and_releases_the_loading_flag() {
    let state: UseEuvCamera = UseEuvCamera::new(
        Signal::create(false),
        Signal::create(false),
        Signal::create(String::new()),
        Signal::create(EuvCameraFacing::default()),
        Signal::create(String::from("stale")),
        Signal::create(None),
    );
    let handler: Rc<dyn Fn(Event)> =
        UseEuvCamera::on_open(state, None).expect("the handler is built without touching the dom");
    handler(Event::new("click").expect("a click event"));
    assert!(
        !state.camera_loading.get(),
        "a failed open must release the loading flag, or the spinner never stops"
    );
    assert!(
        !state.camera_open.get(),
        "a failed open must not report the camera as open"
    );
    assert!(
        !state.error_message.get().is_empty(),
        "the caller has to learn why the camera did not start"
    );
    assert!(
        state.scan_result.get().is_empty(),
        "a fresh open attempt must clear the previous scan result"
    );
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
#[cfg_attr(
    not(target_arch = "wasm32"),
    ignore = "reaches the no-DOM fallback only under wasm"
)]
fn a_switch_handler_flips_the_facing_direction() {
    let state: UseEuvCamera = UseEuvCamera::new(
        Signal::create(true),
        Signal::create(false),
        Signal::create(String::new()),
        Signal::create(EuvCameraFacing::User),
        Signal::create(String::new()),
        Signal::create(None),
    );
    let handler: Rc<dyn Fn(Event)> = UseEuvCamera::on_switch(state, None)
        .expect("the handler is built without touching the dom");
    handler(Event::new("click").expect("a click event"));
    assert_eq!(
        state.facing.get(),
        EuvCameraFacing::Environment,
        "switching must land on the opposite of the direction it started in"
    );
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
#[cfg_attr(
    not(target_arch = "wasm32"),
    ignore = "reaches the no-DOM fallback only under wasm"
)]
fn a_second_switch_returns_to_the_original_facing_direction() {
    let state: UseEuvCamera = UseEuvCamera::new(
        Signal::create(false),
        Signal::create(false),
        Signal::create(String::new()),
        Signal::create(EuvCameraFacing::Environment),
        Signal::create(String::new()),
        Signal::create(None),
    );
    let handler: Rc<dyn Fn(Event)> = UseEuvCamera::on_switch(state, None)
        .expect("the handler is built without touching the dom");
    handler(Event::new("click").expect("a click event"));
    assert_eq!(
        state.facing.get(),
        EuvCameraFacing::User,
        "the flip is two-way, not a one-way latch"
    );
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
#[cfg_attr(not(target_arch = "wasm32"), ignore = "needs a wasm realm")]
fn a_registered_guard_can_be_removed_again() {
    let id: usize = Router::register_popstate_guard(Rc::new(|| true));
    assert!(
        Router::unregister_popstate_guard(id),
        "a guard that was just registered must still be there to remove"
    );
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
#[cfg_attr(not(target_arch = "wasm32"), ignore = "needs a wasm realm")]
fn unregistering_a_guard_twice_reports_that_nothing_matched_the_second_time() {
    let id: usize = Router::register_popstate_guard(Rc::new(|| true));
    assert!(Router::unregister_popstate_guard(id), "first removal takes");
    assert!(
        !Router::unregister_popstate_guard(id),
        "the guard is gone, so a second removal must say so rather than claim success"
    );
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
#[cfg_attr(not(target_arch = "wasm32"), ignore = "needs a wasm realm")]
fn removing_one_guard_leaves_the_others_registered() {
    let first: usize = Router::register_popstate_guard(Rc::new(|| true));
    let second: usize = Router::register_popstate_guard(Rc::new(|| true));
    let third: usize = Router::register_popstate_guard(Rc::new(|| true));
    assert!(
        Router::unregister_popstate_guard(second),
        "the middle one goes"
    );
    assert!(
        !Router::unregister_popstate_guard(second),
        "it is already gone"
    );
    assert!(
        Router::unregister_popstate_guard(first),
        "the first survives until asked"
    );
    assert!(Router::unregister_popstate_guard(third), "so does the last");
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
#[cfg_attr(not(target_arch = "wasm32"), ignore = "needs a wasm realm")]
fn every_registration_gets_its_own_id() {
    let ids: [usize; 3] = [
        Router::register_popstate_guard(Rc::new(|| true)),
        Router::register_popstate_guard(Rc::new(|| false)),
        Router::register_popstate_guard(Rc::new(|| true)),
    ];
    assert_ne!(ids[0], ids[1], "two registrations must not share an id");
    assert_ne!(ids[1], ids[2], "and the third must differ too");
    for id in ids {
        assert!(
            Router::unregister_popstate_guard(id),
            "each id must resolve to exactly its own guard"
        );
    }
}
#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
#[cfg_attr(
    not(target_arch = "wasm32"),
    ignore = "reaches the no-DOM fallback only under wasm"
)]
fn a_default_camera_config_scans_automatically_with_no_callbacks() {
    let config: EuvCameraConfig = EuvCameraConfig::default();
    assert!(
        config.auto_scan,
        "QR scanning is on by default, so a camera works without extra wiring"
    );
    assert!(
        config.on_qr_detected.is_none() && config.on_error.is_none(),
        "a default config must not carry callbacks the caller never supplied"
    );
    assert!(
        !config.video_selector.is_empty(),
        "the default selector must still point at something"
    );
    assert!(
        config.scan_interval_millis > 0,
        "a zero interval would busy-loop the scanner, got {}",
        config.scan_interval_millis
    );
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
#[cfg_attr(
    not(target_arch = "wasm32"),
    ignore = "reaches the no-DOM fallback only under wasm"
)]
fn a_container_lookup_without_a_document_reports_nothing_found() {
    let by_default: Option<Element> = UseVirtualList::try_get_container();
    assert_eq!(
        by_default, None,
        "no document means no container, and that is not an error"
    );
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
#[cfg_attr(
    not(target_arch = "wasm32"),
    ignore = "reaches the no-DOM fallback only under wasm"
)]
fn a_container_lookup_by_an_arbitrary_id_without_a_document_reports_nothing_found() {
    let observed: Option<Element> = UseVirtualList::try_get_container_by_id("anything");
    assert_eq!(
        observed, None,
        "an id cannot resolve without a document, whatever the id is"
    );
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
#[cfg_attr(
    not(target_arch = "wasm32"),
    ignore = "reaches the no-DOM fallback only under wasm"
)]
fn a_viewport_measurement_without_a_container_leaves_the_height_alone() {
    let state: UseVirtualList = UseVirtualList::new(Signal::create(0), Signal::create(321));
    state.update_viewport_height();
    assert_eq!(
        state.get_viewport_height().get(),
        321,
        "with no container to measure, the recorded height must be left as it was"
    );
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
#[cfg_attr(
    not(target_arch = "wasm32"),
    ignore = "reaches the no-DOM fallback only under wasm"
)]
fn a_container_lookup_accepts_any_id_type_that_borrows_as_str() {
    let owned: String = String::from("list");
    let from_owned: Option<Element> = UseVirtualList::try_get_container_by_id(&owned);
    let from_literal: Option<Element> = UseVirtualList::try_get_container_by_id("list");
    assert_eq!(
        from_owned, from_literal,
        "an owned String and a str literal name the same container, so both must miss alike"
    );
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
#[cfg_attr(
    not(target_arch = "wasm32"),
    ignore = "reaches the no-DOM fallback only under wasm"
)]
fn a_resize_hook_without_a_window_reports_the_desktop_breakpoint() {
    let observed: Signal<bool> = UseEuvLayout::use_resize();
    assert!(
        !observed.get(),
        "without a viewport there is nothing narrow, so the layout must stay in desktop mode"
    );
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
#[cfg_attr(
    not(target_arch = "wasm32"),
    ignore = "reaches the no-DOM fallback only under wasm"
)]
fn a_theme_state_without_a_media_query_starts_light() {
    let mobile: Signal<bool> = Signal::create(false);
    let state: ThemeState = ThemeState::use_theme_state(mobile);
    assert_eq!(
        state.get_theme().get(),
        "light",
        "no matchMedia means no dark preference was ever observed"
    );
    assert!(
        !state.get_root_class().get().is_empty(),
        "the root class must be derived even when the theme is the default"
    );
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
#[cfg_attr(
    not(target_arch = "wasm32"),
    ignore = "reaches the no-DOM fallback only under wasm"
)]
fn a_suspense_handle_starts_pending_and_survives_failure_and_reset() {
    let handle: SuspenseHandle<u32> = use_suspense::<u32>();
    let phase: SuspensePhase<u32> = handle.get_phase().get();
    assert!(
        matches!(phase, SuspensePhase::Pending),
        "a freshly mounted suspense has no data yet, so it must start pending"
    );
    handle.resolve_sync(7);
    assert!(
        matches!(handle.get_phase().get(), SuspensePhase::Resolved(7)),
        "resolve_sync must move the phase to the resolved value"
    );
    handle.fail(String::from("boom"));
    assert!(
        matches!(handle.get_phase().get(), SuspensePhase::Failed(ref m) if m == "boom"),
        "fail must replace the resolved value with the message"
    );
    handle.reset();
    assert!(
        matches!(handle.get_phase().get(), SuspensePhase::Pending),
        "reset must return the handle to its initial phase"
    );
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
#[cfg_attr(
    not(target_arch = "wasm32"),
    ignore = "reaches the no-DOM fallback only under wasm"
)]
fn the_router_history_helpers_all_short_circuit_without_a_window() {
    let route: Signal<String> = Signal::create(String::from("/"));
    Router::use_scroll_to_top(route);
    Router::use_hash_change(route);
    Router::use_overlay_history(Signal::create(false), Signal::create(false));
    Router::overlay_push_state();
    Router::overlay_back(None);
    Router::overlay_stack_close();
    Router::close_drawer_and_navigate(Signal::create(false), "/next");
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
#[cfg_attr(
    not(target_arch = "wasm32"),
    ignore = "reaches the no-DOM fallback only under wasm"
)]
fn a_modal_can_be_pushed_and_closed_again_without_a_window() {
    let visible: Signal<bool> = Signal::create(false);
    let closed: Rc<Cell<bool>> = Rc::new(Cell::new(false));
    let flag: Rc<Cell<bool>> = Rc::clone(&closed);
    Router::modal_push(visible, Rc::new(move || flag.set(true)));
    Router::modal_close_via_ui(visible);
    assert!(
        !closed.get(),
        "the closer belongs to the popstate path, which never runs without a window"
    );
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
#[cfg_attr(
    not(target_arch = "wasm32"),
    ignore = "reaches the no-DOM fallback only under wasm"
)]
fn the_safe_area_and_camera_hooks_unwind_cleanly_without_a_window() {
    UseEuvLayout::use_safe_area_fix();
    let camera: UseEuvCamera = UseEuvCamera::new(
        Signal::create(false),
        Signal::create(false),
        Signal::create(String::new()),
        Signal::create(EuvCameraFacing::default()),
        Signal::create(String::new()),
        Signal::create(None),
    );
    camera.cleanup(None);
    assert!(
        !camera.get_camera_open().get(),
        "cleanup must leave the camera closed whatever the environment"
    );
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
#[cfg_attr(
    not(target_arch = "wasm32"),
    ignore = "reaches the no-DOM fallback only under wasm"
)]
fn a_camera_state_starts_closed_and_idle() {
    let state: UseEuvCamera = UseEuvCamera::use_camera_state();
    assert!(
        !state.get_camera_open().get(),
        "a camera that was never opened must report closed"
    );
    assert!(
        !state.get_camera_loading().get(),
        "and must not still be waiting to open"
    );
    assert!(
        state.get_scan_result().get().is_empty(),
        "no scan has run, so there is nothing to report"
    );
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
#[cfg_attr(
    not(target_arch = "wasm32"),
    ignore = "reaches the no-DOM fallback only under wasm"
)]
fn link_and_external_handlers_are_built_without_a_window() {
    let external: NativeEventHandler = Router::external_link_handler("https://example.invalid");
    let internal: NativeEventHandler = Router::link_handler("/route");
    let _: &NativeEventHandler = &external;
    let _: &NativeEventHandler = &internal;
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
#[cfg_attr(
    not(target_arch = "wasm32"),
    ignore = "reaches the no-DOM fallback only under wasm"
)]
fn the_browser_opening_helpers_are_no_ops_without_a_window() {
    Router::open_system_browser("https://example.invalid");
    Router::use_scroll_drawer_to_active(Signal::create(false));
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
#[cfg_attr(
    not(target_arch = "wasm32"),
    ignore = "reaches the no-DOM fallback only under wasm"
)]
fn the_measurement_helpers_schedule_nothing_without_a_window() {
    UseVirtualList::schedule_measure(UseVirtualList::new(Signal::create(0), Signal::create(10)));
    UseEuvLayout::apply_cached_insets();
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
#[cfg_attr(
    not(target_arch = "wasm32"),
    ignore = "reaches the no-DOM fallback only under wasm"
)]

fn markdown_inlines_nest_to_any_depth() {
    const LEAF: EuvMdInline = EuvMdInline::Text("deep");
    const LEVEL_TWO: EuvMdInline = EuvMdInline::Strong(&[LEAF]);
    const LEVEL_THREE: EuvMdInline = EuvMdInline::Em(&[LEVEL_TWO]);
    const TOP: EuvMdInline = EuvMdInline::Del(&[LEVEL_THREE]);
    assert_eq!(
        format!("{TOP:?}"),
        "Del([Em([Strong([Text(\"deep\")])])])",
        "four levels of inline nesting must keep their exact shape"
    );
}

#[test]
fn markdown_wrappers_keep_different_emphasis_distinct() {
    assert_ne!(
        format!("{:?}", EuvMdInline::Strong(&[EuvMdInline::Text("x")])),
        format!("{:?}", EuvMdInline::Em(&[EuvMdInline::Text("x")])),
        "strong and em wrap the same children but are different renderings"
    );
    assert_ne!(
        format!("{:?}", EuvMdInline::Code("x")),
        format!("{:?}", EuvMdInline::Text("x")),
        "inline code is not the same as plain text with identical content"
    );
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
#[cfg_attr(
    not(target_arch = "wasm32"),
    ignore = "asserts the no-window degradation path of the CSS injector"
)]
fn injecting_global_css_without_a_window_is_a_no_op_rather_than_a_panic() {
    let global: Object = js_sys::global().unchecked_into();
    let window_probe: Result<JsValue, JsValue> =
        Reflect::get(&global, &JsValue::from_str("window"));
    let has_window: bool = window_probe.is_ok_and(|found: JsValue| found.is_object());

    inject_app_global_css();

    assert!(
        !has_window,
        "this test only proves the no-window branch; under a real window it must not run"
    );
}
