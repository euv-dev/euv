use super::*;

/// A canvas drawing board page component that shows a preview of the
/// drawing and a button to enter fullscreen drawing mode.
///
/// In normal mode, displays a card with the current drawing preview
/// (or a placeholder if no drawing exists) and a "Draw" button.
/// Clicking the button enters fullscreen mode with a portrait-oriented
/// canvas for drawing.
///
/// # Returns
///
/// - `VirtualNode` - The canvas drawing board page virtual DOM tree.
#[component]
pub(crate) fn page_canvas(node: VirtualNode<PageCanvasProps>) -> VirtualNode {
    let _page_canvas_props: PageCanvasProps = node.try_get_props().unwrap_or_default();
    let state: UseCanvas = use_canvas_state();
    use_fullscreen_popstate(state);
    let on_color_input = move |event: Event| {
        let new_color: String = Reflect::get(
            event.as_ref(),
            &JsValue::from_str(CANVAS_EVENT_PROPERTY_TARGET),
        )
        .ok()
        .and_then(|target: JsValue| {
            Reflect::get(&target, &JsValue::from_str(CANVAS_EVENT_PROPERTY_VALUE)).ok()
        })
        .and_then(|value: JsValue| value.as_string())
        .unwrap_or_default();
        state.get_stroke_color().set(new_color);
        save_stroke_color(&state.get_stroke_color().get());
    };
    let on_canvas_mouse_down = move |event: Event| {
        let (offset_x, offset_y): (f64, f64) = get_pointer_offset(&event);
        let (client_x, client_y): (f64, f64) = get_mouse_client(&event);
        let (mapped_x, mapped_y): (f64, f64) =
            map_rotated_offset(offset_x, offset_y, client_x, client_y, true);
        start_drawing(state, mapped_x, mapped_y);
    };
    let on_canvas_mouse_move = move |event: Event| {
        let (offset_x, offset_y): (f64, f64) = get_pointer_offset(&event);
        let (client_x, client_y): (f64, f64) = get_mouse_client(&event);
        let (mapped_x, mapped_y): (f64, f64) =
            map_rotated_offset(offset_x, offset_y, client_x, client_y, true);
        continue_drawing(state, mapped_x, mapped_y);
    };
    let on_canvas_mouse_up = move |_: Event| {
        stop_drawing(state);
    };
    let on_canvas_mouse_leave = move |_: Event| {
        stop_drawing(state);
    };
    let on_canvas_touch_start = move |event: Event| {
        prevent_event_default(&event);
        start_drawing_multi_touch(state, &event, true);
    };
    let on_canvas_touch_move = move |event: Event| {
        prevent_event_default(&event);
        continue_drawing_multi_touch(state, &event, true);
    };
    let on_canvas_touch_end = move |event: Event| {
        stop_drawing_multi_touch(state, &event);
    };
    let on_canvas_touch_cancel = move |event: Event| {
        stop_drawing_multi_touch(state, &event);
    };
    let initial_line_width: f64 = state.get_line_width().get();
    let initial_line_width_percent: i32 =
        ((initial_line_width - 1.0) / 29.0 * 100.0).clamp(0.0, 100.0) as i32;
    html! {
        div {
            class: c_page_container()
            euv_header {
                icon: "🎨"
                title: "Canvas"
                subtitle: "A freehand drawing board with color picker and line width control. Tap Draw to enter fullscreen canvas mode."
            }
            euv_card {
                title: "Drawing Board"
                div {
                    class: c_button_controls()
                    euv_button {
                        variant: EuvButtonVariant::Primary
                        label: CANVAS_DRAW_LABEL
                        onclick: canvas_on_draw(state)
                    }
                }
                div {
                    class: c_canvas_preview_container()
                    if { state.get_snapshot_data_url().get().is_empty() } {
                        div {
                            class: c_canvas_placeholder()
                            "No drawing yet. Tap Draw to start."
                        }
                    } else {
                        img {
                            class: c_canvas_preview_image()
                            src: state.get_snapshot_data_url().get()
                        }
                    }
                }
            }
        }
        if { state.get_fullscreen().get() } {
            div {
                id: CANVAS_CONTAINER_ID
                class: c_canvas_container_fullscreen()
                div {
                    class: c_canvas_fullscreen_toolbar()
                    div {
                        class: c_canvas_fullscreen_toolbar_row_top()
                        div {
                            class: c_canvas_fullscreen_toolbar_button()
                            euv_button {
                                variant: EuvButtonVariant::Primary
                                label: "Clear"
                                onclick: canvas_on_clear(state)
                            }
                        }
                        div {
                            class: c_canvas_fullscreen_toolbar_color_wrapper()
                            input {
                                type: "color"
                                class: c_canvas_color_input_fullscreen()
                                value: state.get_stroke_color().get()
                                oninput: on_color_input
                            }
                        }
                        div {
                            class: c_canvas_fullscreen_toolbar_button()
                            euv_button {
                                variant: EuvButtonVariant::Primary
                                label: CANVAS_FULLSCREEN_EXIT_LABEL
                                onclick: canvas_on_exit_fullscreen(state)
                            }
                        }
                    }
                    div {
                        class: c_canvas_fullscreen_toolbar_row_bottom()
                        input {
                            type: "range"
                            class: c_canvas_fullscreen_range_input()
                            min: CANVAS_MIN_LINE_WIDTH_ATTR
                            max: CANVAS_MAX_LINE_WIDTH_ATTR
                            step: CANVAS_LINE_WIDTH_STEP_ATTR
                            value: initial_line_width.to_string()
                            class: c_slider_value(&format!("{initial_line_width_percent}%"))
                            oninput: canvas_on_line_width_input(state)
                        }
                    }
                }
                div {
                    id: CANVAS_FULLSCREEN_WRAPPER_ID
                    class: c_canvas_drawing_fullscreen_wrapper()
                    canvas {
                        id: CANVAS_DRAWING_ID
                        class: c_canvas_drawing_fullscreen()
                        onmousedown: on_canvas_mouse_down
                        onmousemove: on_canvas_mouse_move
                        onmouseup: on_canvas_mouse_up
                        onmouseleave: on_canvas_mouse_leave
                        ontouchstart: on_canvas_touch_start
                        ontouchmove: on_canvas_touch_move
                        ontouchend: on_canvas_touch_end
                        ontouchcancel: on_canvas_touch_cancel
                    }
                }
            }
        }
    }
}
