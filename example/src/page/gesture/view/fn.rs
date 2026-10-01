use super::*;

/// A gesture recognition demo page driven by `EuvGestureRecognizer`.
///
/// The page mounts one recognizer with the default thresholds and wires its
/// four touch handlers onto a single demo pad. `last_gesture` reports the most
/// recently completed single-finger gesture, while `drag` and `pinch` stream
/// the live in-flight readings, so the readouts update continuously during a
/// move and settle once the finger lifts.
///
/// # Arguments
///
/// - `VirtualNode<PageGestureProps>` - The vdom node carrying the page
///   props.
///
/// # Returns
///
/// - `VirtualNode` - The gesture page virtual DOM tree.
#[component]
pub(crate) fn page_gesture(node: VirtualNode<PageGestureProps>) -> VirtualNode {
    let PageGestureProps: PageGestureProps = node.try_get_props().unwrap_or_default();
    let recognizer: EuvGestureRecognizer = EuvGestureRecognizer::new();
    let config: EuvGestureConfig = recognizer.get_config();
    let state: EuvGestureState = recognizer.use_gesture();
    let last_gesture_text: String = match state.get_last_gesture().get() {
        Some(gesture) => gesture.name().to_string(),
        None => GESTURE_PLACEHOLDER.to_string(),
    };
    let drag_now: Option<EuvDrag> = state.get_drag().get();
    let drag_dx_text: String = match drag_now {
        Some(drag) => format!("{:.1}", drag.get_delta_x()),
        None => GESTURE_PLACEHOLDER_NUMBER.to_string(),
    };
    let drag_dy_text: String = match drag_now {
        Some(drag) => format!("{:.1}", drag.get_delta_y()),
        None => GESTURE_PLACEHOLDER_NUMBER.to_string(),
    };
    let drag_travel_text: String = match drag_now {
        Some(drag) => format!("{:.1}", drag.get_travel()),
        None => GESTURE_PLACEHOLDER_NUMBER.to_string(),
    };
    let pinch_now: Option<EuvPinch> = state.get_pinch().get();
    let pinch_distance_text: String = match pinch_now {
        Some(pinch) => format!("{:.1}", pinch.get_distance()),
        None => GESTURE_PLACEHOLDER_NUMBER.to_string(),
    };
    let pinch_delta_text: String = match pinch_now {
        Some(pinch) => format!("{:.1}", pinch.get_delta()),
        None => GESTURE_PLACEHOLDER_NUMBER.to_string(),
    };
    let pinch_scale_text: String = match pinch_now {
        Some(pinch) => {
            let start: f64 = pinch.get_start_distance();
            let scale: f64 = if start > 0.0 {
                pinch.get_distance() / start
            } else {
                0.0
            };
            format!("{scale:.2}x")
        }
        None => GESTURE_PLACEHOLDER_NUMBER.to_string(),
    };
    let swipe_threshold_text: String =
        format!("{:.0}{}", config.get_swipe_threshold(), GESTURE_UNIT_PIXELS);
    let tap_slop_text: String = format!("{:.0}{}", config.get_tap_slop(), GESTURE_UNIT_PIXELS);
    let long_press_text: String = format!(
        "{:.0}{}",
        config.get_long_press_millis(),
        GESTURE_UNIT_MILLIS
    );
    let pinch_threshold_text: String = format!(
        "{:.2}{}",
        config.get_pinch_threshold() * 100.0,
        GESTURE_UNIT_RATIO
    );
    html! {
        div {
            class: c_page_container()
            euv_header {
                icon: "👆"
                title: "Gesture"
                subtitle: "EuvGestureRecognizer turns raw touch points into swipes, taps, long presses, and two-finger pinches. All four handlers mount on one pad, and the readouts below are driven straight off the recognizer signals."
            }
            euv_card {
                title: "Gesture Pad"
                div {
                    id: "gesture-readout"
                    class: c_event_touch_zone()
                    class: c_event_drag_zone()
                    style: "min-height: 340px; touch-action: none; user-select: none;"
                    ontouchstart: state.on_start
                    ontouchmove: state.on_move
                    ontouchend: state.on_end
                    ontouchcancel: state.on_cancel
                    p {
                        class: c_demo_text()
                        "Drag across this pad."
                    }
                    p {
                        class: c_demo_text_muted()
                        "Swipe past the threshold to resolve a direction, hold still for a long press, or use two fingers to pinch."
                    }
                }
            }
            euv_card {
                title: "Live Recognizer State"
                div {
                    class: c_info_row()
                    span {
                        class: c_info_label()
                        "Last gesture"
                    }
                    span {
                        id: "gesture-last-value"
                        class: c_info_value()
                        last_gesture_text
                    }
                }
                div {
                    class: c_info_row()
                    span {
                        class: c_info_label()
                        "Drag dx"
                    }
                    span {
                        id: "gesture-drag-dx-value"
                        class: c_info_value()
                        drag_dx_text
                    }
                }
                div {
                    class: c_info_row()
                    span {
                        class: c_info_label()
                        "Drag dy"
                    }
                    span {
                        id: "gesture-drag-dy-value"
                        class: c_info_value()
                        drag_dy_text
                    }
                }
                div {
                    class: c_info_row()
                    span {
                        class: c_info_label()
                        "Travel"
                    }
                    span {
                        id: "gesture-drag-travel-value"
                        class: c_info_value()
                        drag_travel_text
                    }
                }
                div {
                    class: c_info_row()
                    span {
                        class: c_info_label()
                        "Pinch distance"
                    }
                    span {
                        id: "gesture-pinch-distance-value"
                        class: c_info_value()
                        pinch_distance_text
                    }
                }
                div {
                    class: c_info_row()
                    span {
                        class: c_info_label()
                        "Pinch delta"
                    }
                    span {
                        id: "gesture-pinch-delta-value"
                        class: c_info_value()
                        pinch_delta_text
                    }
                }
                div {
                    class: c_info_row()
                    span {
                        class: c_info_label()
                        "Pinch scale"
                    }
                    span {
                        id: "gesture-pinch-scale-value"
                        class: c_info_value()
                        pinch_scale_text
                    }
                }
            }
            euv_card {
                title: "Recognizer Thresholds"
                p {
                    class: c_game_description()
                    "The pad mounts EuvGestureRecognizer::new(), so every reading below comes from the default threshold set. A 48px swipe threshold keeps fingertip jitter out of the directional gestures, a 10px tap slop separates a tap from a drag, a 500ms press turns a stationary touch into a long press, and a 1% pinch change filters out adjacent-move noise."
                }
                euv_info {
                    label: "Swipe"
                    swipe_threshold_text
                }
                euv_info {
                    label: "Tap slop"
                    tap_slop_text
                }
                euv_info {
                    label: "Long press"
                    long_press_text
                }
                euv_info {
                    label: "Pinch"
                    pinch_threshold_text
                }
            }
        }
    }
}
