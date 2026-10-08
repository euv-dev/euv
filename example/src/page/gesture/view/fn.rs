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
                icon: GESTURE_HEADER_ICON
                title: GESTURE_HEADER_TITLE
                subtitle: GESTURE_HEADER_SUBTITLE
            }
            euv_card {
                title: GESTURE_PAD_CARD_TITLE
                div {
                    id: GESTURE_READOUT_CONTAINER_ID
                    class: c_event_touch_zone()
                    class: c_event_drag_zone()
                    style: GESTURE_PAD_STYLE
                    ontouchstart: state.on_start
                    ontouchmove: state.on_move
                    ontouchend: state.on_end
                    ontouchcancel: state.on_cancel
                    p {
                        class: c_demo_text()
                        GESTURE_PAD_PRIMARY_TEXT
                    }
                    p {
                        class: c_demo_text_muted()
                        GESTURE_PAD_SECONDARY_TEXT
                    }
                }
            }
            euv_card {
                title: GESTURE_READOUT_CARD_TITLE
                div {
                    class: c_info_row()
                    span {
                        class: c_info_label()
                        GESTURE_LABEL_LAST
                    }
                    span {
                        id: GESTURE_LAST_VALUE_ID
                        class: c_info_value()
                        last_gesture_text
                    }
                }
                div {
                    class: c_info_row()
                    span {
                        class: c_info_label()
                        GESTURE_LABEL_DRAG_DX
                    }
                    span {
                        id: GESTURE_DRAG_DX_VALUE_ID
                        class: c_info_value()
                        drag_dx_text
                    }
                }
                div {
                    class: c_info_row()
                    span {
                        class: c_info_label()
                        GESTURE_LABEL_DRAG_DY
                    }
                    span {
                        id: GESTURE_DRAG_DY_VALUE_ID
                        class: c_info_value()
                        drag_dy_text
                    }
                }
                div {
                    class: c_info_row()
                    span {
                        class: c_info_label()
                        GESTURE_LABEL_DRAG_TRAVEL
                    }
                    span {
                        id: GESTURE_DRAG_TRAVEL_VALUE_ID
                        class: c_info_value()
                        drag_travel_text
                    }
                }
                div {
                    class: c_info_row()
                    span {
                        class: c_info_label()
                        GESTURE_LABEL_PINCH_DISTANCE
                    }
                    span {
                        id: GESTURE_PINCH_DISTANCE_VALUE_ID
                        class: c_info_value()
                        pinch_distance_text
                    }
                }
                div {
                    class: c_info_row()
                    span {
                        class: c_info_label()
                        GESTURE_LABEL_PINCH_DELTA
                    }
                    span {
                        id: GESTURE_PINCH_DELTA_VALUE_ID
                        class: c_info_value()
                        pinch_delta_text
                    }
                }
                div {
                    class: c_info_row()
                    span {
                        class: c_info_label()
                        GESTURE_LABEL_PINCH_SCALE
                    }
                    span {
                        id: GESTURE_PINCH_SCALE_VALUE_ID
                        class: c_info_value()
                        pinch_scale_text
                    }
                }
            }
            euv_card {
                title: GESTURE_CONFIG_CARD_TITLE
                p {
                    class: c_game_description()
                    GESTURE_CONFIG_DESCRIPTION
                }
                euv_info {
                    label: GESTURE_LABEL_SWIPE_THRESHOLD
                    swipe_threshold_text
                }
                euv_info {
                    label: GESTURE_LABEL_TAP_SLOP
                    tap_slop_text
                }
                euv_info {
                    label: GESTURE_LABEL_LONG_PRESS
                    long_press_text
                }
                euv_info {
                    label: GESTURE_LABEL_PINCH_THRESHOLD
                    pinch_threshold_text
                }
            }
        }
    }
}
