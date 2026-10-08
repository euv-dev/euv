use super::*;

/// Formats a duration in seconds into a MM:SS display string.
///
/// # Arguments
///
/// - `i32` - The total seconds to format.
///
/// # Returns
///
/// - `String` - The formatted time string in MM:SS format.
fn format_time(total_seconds: i32) -> String {
    let minutes: i32 = total_seconds / 60;
    let seconds: i32 = total_seconds % 60;
    format!("{:02}:{:02}", minutes, seconds)
}

/// A timer demo page with stopwatch and countdown features.
/// # Arguments
///
/// - `VirtualNode<PageTimerProps>` - The component props node.
///
/// # Returns
///
/// - `VirtualNode` - The timer demo page virtual DOM tree.
///
/// # Arguments
///
/// - `VirtualNode<PageTimerProps>` - The `node` argument.
///
#[component]
pub(crate) fn page_timer(node: VirtualNode<PageTimerProps>) -> VirtualNode {
    let PageTimerProps: PageTimerProps = node.try_get_props().unwrap_or_default();
    let stopwatch: UseStopwatch = use_stopwatch();
    let countdown: UseCountdown = use_countdown();
    App::use_cleanup(move || {
        if let Some(handle) = stopwatch.get_handle().get() {
            handle.clear();
        }
        if let Some(handle) = countdown.get_handle().get() {
            handle.clear();
        }
    });
    html! {
        div {
            class: c_page_container()
            euv_header {
                icon: "⏱️"
                title: "Timer"
                subtitle: "Interval-based stopwatch and countdown timer. Both use use_interval for precise timing and use_cleanup to clear intervals on unmount."
            }
            euv_card {
                title: "Stopwatch"
                div {
                    class: c_timer_display()
                    span {
                        class: c_timer_value()
                        {
                            format_time(stopwatch.get_seconds().get())
                        }
                    }
                }
                div {
                    class: c_timer_controls()
                    button {
                        class: c_euv_button_primary_md()
                        onclick: stopwatch_on_start(stopwatch)
                        if { !stopwatch.get_running().get() } {
                            "Start"
                        } else {
                            "Pause"
                        }
                    }
                    euv_button {
                        variant: EuvButtonVariant::Primary
                        label: "Reset"
                        onclick: stopwatch_on_reset(stopwatch)
                    }
                }
            }
            euv_card {
                title: "Countdown Timer"
                div {
                    class: c_euv_input_wrapper()
                    label {
                        for: "countdown-seconds"
                        class: c_form_label()
                        "Set seconds"
                    }
                    input {
                        id: "countdown-seconds"
                        name: "seconds"
                        type: "number"
                        autocomplete: "off"
                        min: "1"
                        max: "3600"
                        placeholder: "Enter seconds..."
                        value: countdown.get_input()
                        class: c_euv_input()
                        oninput: countdown_on_input(countdown)
                    }
                }
                div {
                    class: c_timer_display()
                    span {
                        class: c_timer_value()
                        {
                            format_time(countdown.get_remaining().get())
                        }
                    }
                }
                div {
                    class: c_timer_controls()
                    button {
                        class: c_euv_button_primary_md()
                        onclick: countdown_on_start(countdown)
                        if { !countdown.get_running().get() } {
                            "Start"
                        } else {
                            "Pause"
                        }
                    }
                    euv_button {
                        variant: EuvButtonVariant::Primary
                        label: "Reset"
                        onclick: countdown_on_reset(countdown)
                    }
                }
                if { countdown.get_remaining().get() == 0 && !countdown.get_running().get() } {
                    div {
                        class: c_timer_done()
                        "⏰ Time's up!"
                    }
                }
            }
        }
    }
}
