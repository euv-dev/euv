use super::*;

/// A page demonstrating the timing hooks
/// ([`DebouncedValue`], [`ThrottledValue`] and [`Previous`]).
///
/// The debounce / throttle state machines are driven by a single
/// `App::use_interval` ticker; timestamps come from
/// `performance.now()` because `std::time::Instant::now()` panics on
/// `wasm32-unknown-unknown`.
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
                title: "Hooks — Timing"
                subtitle: "DebouncedValue, ThrottledValue, and Previous side-by-side. Each row drives a Signal from a different rate-control policy."
            }
            euv_card {
                title: "Debounce (quiet period)"
                p {
                    "Type into the box to seed a pending value; after 300 ms of idle time the debounced signal commits the latest pending value."
                }
                div {
                    class: c_inline_input_row()
                    euv_input {
                        id: TIMING_DEBOUNCE_INPUT_ID
                        label: "Live input"
                        placeholder: TIMING_INPUT_PLACEHOLDER
                        value: live_debounce
                        oninput: timing_debounce_on_input(live_debounce, debounced, current, previous)
                    }
                    span {
                        class: c_counter_value()
                        timing_signal_to_string(&debounced_value)
                    }
                }
            }
            euv_card {
                title: "Throttle (max-once-per-window)"
                p {
                    "Type into the box to push pending values into the throttler. The committed value updates at most once every 250 ms."
                }
                div {
                    class: c_inline_input_row()
                    euv_input {
                        id: TIMING_THROTTLE_INPUT_ID
                        label: "Live input"
                        placeholder: TIMING_INPUT_PLACEHOLDER
                        value: live_throttle
                        oninput: timing_throttle_on_input(live_throttle, throttled, current, previous)
                    }
                    span {
                        class: c_counter_value()
                        timing_signal_to_string(&throttled_value)
                    }
                }
            }
            euv_card {
                title: "Previous (snapshot of last render)"
                p {
                    "Each render is preceded by `previous_step`, which records the current value and reports the snapshot from the previous render."
                }
                div {
                    class: c_counter_row()
                    div {
                        "current:"
                        span {
                            class: c_counter_value()
                            current
                        }
                    }
                    div {
                        "previous:"
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
