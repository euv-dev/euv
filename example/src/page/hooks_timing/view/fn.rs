use super::*;

/// A page demonstrating the timing hooks
/// ([`DebouncedValue`], [`ThrottledValue`] and [`Previous`]).
///
/// The debounce / throttle state machines are driven by a single
/// `App::use_interval` ticker; timestamps come from
/// `performance.now()` because `std::time::Instant::now()` panics on
/// `wasm32-unknown-unknown`.
///
/// # Arguments
///
/// - `VirtualNode<PageHooksTimingProps>` - The page component node
///   carrying the page props.
///
/// # Returns
///
/// - `VirtualNode` - The timing hooks page virtual DOM tree.
#[component]
pub(crate) fn page_hooks_timing(node: VirtualNode<PageHooksTimingProps>) -> VirtualNode {
    let PageHooksTimingProps: PageHooksTimingProps = node.try_get_props().unwrap_or_default();
    let debounced: DebouncedValue<String> = use_debounced_value::<String>(TIMING_DEBOUNCE_MS);
    let throttled: ThrottledValue<String> = use_throttled_value::<String>(TIMING_THROTTLE_MS);
    let previous: Previous<String> = use_previous::<String>();
    let current: Signal<String> = App::use_signal(String::new);
    let live_debounce: Signal<String> = App::use_signal(String::new);
    let live_throttle: Signal<String> = App::use_signal(String::new);
    App::use_interval(TIMING_TICK_MS, {
        let debounced: DebouncedValue<String> = debounced;
        let throttled: ThrottledValue<String> = throttled;
        move || {
            let now_ms: u64 = timing_now_ms();
            debounced.tick(now_ms);
            throttled.tick(now_ms);
        }
    });
    let debounced_value: Signal<String> = debounced.get_value();
    let throttled_value: Signal<String> = throttled.get_value();
    html! {
        div {
            class: c_page_container()
            euv_header {
                icon: "⏲️"
                title: HOOKS_TIMING_PAGE_TITLE
                subtitle: HOOKS_TIMING_PAGE_SUBTITLE
            }
            euv_card {
                title: HOOKS_TIMING_ROW_DEBOUNCE_TITLE
                p {
                    HOOKS_TIMING_ROW_DEBOUNCE_BODY
                }
                div {
                    class: c_inline_input_row()
                    euv_input {
                        id: TIMING_DEBOUNCE_INPUT_ID
                        label: HOOKS_TIMING_INPUT_LABEL
                        placeholder: TIMING_INPUT_PLACEHOLDER
                        value: live_debounce
                        oninput: timing_debounce_on_input(live_debounce, debounced, current, previous)
                    }
                }
                if { !timing_signal_to_string(&debounced_value).is_empty() } {
                    div {
                        class: c_counter_value_row()
                        timing_signal_to_string(&debounced_value)
                    }
                }
            }
            euv_card {
                title: HOOKS_TIMING_ROW_THROTTLE_TITLE
                p {
                    HOOKS_TIMING_ROW_THROTTLE_BODY
                }
                div {
                    class: c_inline_input_row()
                    euv_input {
                        id: TIMING_THROTTLE_INPUT_ID
                        label: HOOKS_TIMING_INPUT_LABEL
                        placeholder: TIMING_INPUT_PLACEHOLDER
                        value: live_throttle
                        oninput: timing_throttle_on_input(live_throttle, throttled, current, previous)
                    }
                }
                if { !timing_signal_to_string(&throttled_value).is_empty() } {
                    div {
                        class: c_counter_value_row()
                        timing_signal_to_string(&throttled_value)
                    }
                }
            }
            euv_card {
                title: HOOKS_TIMING_ROW_PREVIOUS_TITLE
                p {
                    HOOKS_TIMING_ROW_PREVIOUS_BODY
                }
                div {
                    class: c_counter_row()
                    div {
                        HOOKS_TIMING_CURRENT_PREFIX
                        span {
                            class: c_counter_value()
                            current
                        }
                    }
                    div {
                        HOOKS_TIMING_PREVIOUS_PREFIX
                        span {
                            class: c_counter_value()
                            timing_previous_snapshot(previous)
                        }
                    }
                }
            }
        }
    }
}
