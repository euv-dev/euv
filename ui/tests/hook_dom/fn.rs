use super::*;

fn with_context<F, R>(callback: F) -> R
where
    F: FnOnce() -> R,
{
    HookContext::with(HookContext::default(), callback)
}

#[test]
fn a_boolean_starts_at_the_value_the_caller_asked_for() {
    let handle: UseBoolean = with_context(|| use_boolean(true));
    assert!(
        handle.get_value().get(),
        "a boolean asked to start at true must start at true"
    );
    let other: UseBoolean = with_context(|| use_boolean(false));
    assert!(
        !other.get_value().get(),
        "a boolean asked to start at false must start at false"
    );
}

#[test]
fn a_boolean_can_be_set_and_flipped() {
    let handle: UseBoolean = with_context(|| use_boolean(false));
    handle.set_true();
    assert!(handle.get_value().get(), "set_true must raise the value");
    handle.set_false();
    assert!(!handle.get_value().get(), "set_false must drop the value");
    handle.toggle();
    assert!(handle.get_value().get(), "toggle must flip a false to true");
    handle.toggle();
    assert!(
        !handle.get_value().get(),
        "toggle must flip a true to false"
    );
}

#[test]
fn two_booleans_in_the_same_context_hold_independent_values() {
    let (first, second): (UseBoolean, UseBoolean) = with_context(|| {
        let first: UseBoolean = use_boolean(false);
        let second: UseBoolean = use_boolean(false);
        (first, second)
    });
    first.set_true();
    assert!(
        first.get_value().get(),
        "the first boolean must carry its own value"
    );
    assert!(
        !second.get_value().get(),
        "raising the first boolean must not raise the second"
    );
}

#[test]
fn a_boolean_at_the_same_slot_keeps_its_value_across_renders() {
    let mut context: HookContext = HookContext::default();
    let first: UseBoolean = HookContext::with(context.clone(), || use_boolean(false));
    first.set_true();
    context.reset_index();
    let second: UseBoolean = HookContext::with(context, || use_boolean(false));
    assert!(
        second.get_value().get(),
        "the second render at the same slot must see the value the first one left"
    );
}

#[test]
fn an_identifier_carries_its_prefix_and_is_unique() {
    let first: UseId = with_context(|| use_id("field"));
    let second: UseId = with_context(|| use_id("field"));
    let first_value: String = first.get_value().get();
    let second_value: String = second.get_value().get();
    assert!(
        first_value.starts_with("field-"),
        "an identifier must carry its prefix, got {first_value}"
    );
    assert_ne!(
        first_value, second_value,
        "two identifiers must not collide"
    );
}

#[test]
fn an_identifier_at_the_same_slot_is_the_same_identifier() {
    let mut context: HookContext = HookContext::default();
    let first: UseId = HookContext::with(context.clone(), || use_id("panel"));
    context.reset_index();
    let second: UseId = HookContext::with(context, || use_id("panel"));
    assert_eq!(
        first.get_value().get(),
        second.get_value().get(),
        "the identifier must be stable across renders"
    );
}

#[test]
fn a_stored_value_starts_at_the_fallback_when_the_key_is_absent() {
    let handle: UseStorage =
        with_context(|| use_storage("euv.test.absent", UseStorageKind::Local, "fallback"));
    assert_eq!(
        handle.current(),
        "fallback",
        "a key that is not in storage must read as the fallback"
    );
}

#[test]
fn a_stored_value_follows_a_write() {
    let handle: UseStorage =
        with_context(|| use_storage("euv.test.write", UseStorageKind::Local, "start"));
    handle.set_value("changed");
    assert_eq!(
        handle.current(),
        "changed",
        "a write must be visible through the signal"
    );
    handle.clear();
    assert_eq!(handle.current(), "", "clearing must empty the value");
}

#[test]
fn a_timer_starts_at_zero_ticks() {
    let handle: UseTimer = with_context(|| use_timer(1000));
    assert_eq!(
        handle.current_ticks(),
        0,
        "a timer must not have fired before its first tick"
    );
    handle.reset();
    assert_eq!(handle.current_ticks(), 0, "a reset must zero the count");
}

#[test]
fn a_media_query_reports_no_match_without_a_browser() {
    let handle: UseMediaQuery = with_context(|| use_media_query("(max-width: 767px)"));
    assert!(
        !handle.is_matching(),
        "a host without a browser cannot match a media query"
    );
    assert_eq!(
        handle.get_query(),
        "(max-width: 767px)",
        "the handle must remember the query it watches"
    );
}

#[test]
fn a_reduced_motion_handle_reports_no_preference_without_a_browser() {
    let handle: UseReducedMotion = with_context(use_reduced_motion);
    assert!(
        !handle.is_reduced(),
        "a host without a browser cannot express a motion preference"
    );
}

#[test]
fn an_online_status_handle_reports_online_without_a_browser() {
    let handle: UseOnlineStatus = with_context(use_online_status);
    assert!(
        handle.is_online(),
        "a host without a browser has no connection to lose"
    );
}

#[test]
fn a_click_outside_handle_starts_untriggered() {
    let handle: UseClickOutside = with_context(use_click_outside);
    assert!(
        !handle.is_triggered(),
        "no press has happened yet, so nothing is outside"
    );
    handle.reset();
    assert!(
        !handle.is_triggered(),
        "a reset must keep the handle untriggered"
    );
}

#[test]
fn a_hover_handle_starts_hovered_nowhere() {
    let handle: UseHover = with_context(use_hover);
    assert!(
        !handle.is_hovered(),
        "the pointer is nowhere near an element that does not exist"
    );
}

#[test]
fn a_resize_observer_reports_a_zero_size_without_a_browser() {
    let handle: UseResizeObserver = with_context(use_resize_observer);
    assert_eq!(
        handle.current_width(),
        0.0,
        "an element that was never observed has no width"
    );
    assert_eq!(
        handle.current_height(),
        0.0,
        "an element that was never observed has no height"
    );
}

#[test]
fn an_intersection_observer_reports_nothing_visible_without_a_browser() {
    let handle: UseIntersectionObserver = with_context(use_intersection_observer);
    assert!(
        !handle.is_visible(),
        "an element that was never observed is not visible"
    );
    assert_eq!(
        handle.current_ratio(),
        0.0,
        "an element that was never observed has no visible ratio"
    );
}

#[test]
fn a_keyboard_handle_starts_with_no_key_down() {
    let handle: UseKeyboard = with_context(use_keyboard);
    assert_eq!(
        handle.current_key(),
        "",
        "no key is down before the first keydown"
    );
    assert!(
        !handle.is_pressed("a"),
        "no key is down before the first keydown"
    );
}

#[test]
fn a_scroll_handle_starts_at_the_origin() {
    let handle: UseScroll = with_context(use_scroll);
    assert_eq!(handle.current_x(), 0.0, "the page starts at x = 0");
    assert_eq!(handle.current_y(), 0.0, "the page starts at y = 0");
}

#[test]
fn a_mounted_handle_starts_unmounted() {
    let handle: UseMounted = with_context(use_mounted);
    assert!(
        !handle.is_mounted(),
        "the flag is only raised after the DOM has been committed"
    );
}

#[test]
fn a_window_size_handle_starts_at_zero() {
    let handle: UseWindowSize = with_context(use_window_size);
    assert_eq!(handle.current_width(), 0, "there is no viewport yet");
    assert_eq!(handle.current_height(), 0, "there is no viewport yet");
}

#[test]
fn a_clipboard_write_is_refused_without_a_browser() {
    let handle: UseClipboard = with_context(use_clipboard);
    assert_eq!(
        handle.current_status(),
        UseClipboardStatus::Idle,
        "no write has been attempted yet"
    );
    assert!(
        !handle.copy("payload"),
        "a host without a browser cannot write to the clipboard"
    );
    assert_eq!(
        handle.current_status(),
        UseClipboardStatus::Unavailable,
        "a refused write must be reported as unavailable"
    );
}
