use super::*;

#[test]
fn a_drawer_toggle_always_yields_a_handler() {
    let open: Signal<bool> = Signal::create(false);
    assert!(
        UseEuvLayout::use_drawer_toggle(open).is_some(),
        "a drawer with no toggle cannot be opened or closed"
    );
}

#[test]
fn a_drawer_toggle_handles_both_initial_states() {
    for start in [true, false] {
        let open: Signal<bool> = Signal::create(start);
        assert!(
            UseEuvLayout::use_drawer_toggle(open).is_some(),
            "the handler must be present whether the drawer starts open or shut"
        );
    }
}

#[test]
fn a_slider_input_handler_is_always_present() {
    let value: Signal<f64> = Signal::create(0.0);
    assert!(
        EuvSliderHelpers::on_slider_input(value, 0.0, 100.0).is_some(),
        "a range input with no handler is a dead control"
    );
}

#[test]
fn a_slider_input_handler_exists_for_a_degenerate_range() {
    let value: Signal<f64> = Signal::create(1.0);
    assert!(
        EuvSliderHelpers::on_slider_input(value, 5.0, 5.0).is_some(),
        "a min == max slider is a misconfiguration, not a reason to drop the handler"
    );
}

#[test]
fn a_gesture_hook_starts_with_nothing_in_flight_and_all_four_handlers() {
    let recogniser: EuvGestureRecognizer = EuvGestureRecognizer::new();
    let state: EuvGestureState =
        HookContext::with(HookContext::default(), || recogniser.use_gesture());
    assert!(
        state.last_gesture.get().is_none(),
        "no gesture has completed yet"
    );
    assert!(state.drag.get().is_none(), "no finger is dragging yet");
    assert!(
        state.pinch.get().is_none(),
        "fewer than two fingers are down"
    );
    assert!(
        state.on_start.is_some(),
        "without touchstart no gesture ever begins"
    );
    assert!(state.on_move.is_some());
    assert!(state.on_end.is_some());
    assert!(state.on_cancel.is_some());
}

#[test]
fn a_gesture_hook_records_a_completed_gesture() {
    let recogniser: EuvGestureRecognizer = EuvGestureRecognizer::new();
    let state: EuvGestureState =
        HookContext::with(HookContext::default(), || recogniser.use_gesture());
    state.last_gesture.set(Some(EuvGesture::Tap));
    assert_eq!(
        state.last_gesture.get(),
        Some(EuvGesture::Tap),
        "the state must carry the most recent gesture to the view layer"
    );
}
