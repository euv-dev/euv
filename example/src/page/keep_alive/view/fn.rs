use super::*;

/// Cleans up the keep-alive timer interval when the page is unmounted.
///
/// # Arguments
///
/// - `Signal<Option<IntervalHandle>>` - The handle signal for the timer interval.
fn keep_alive_cleanup(handle: Signal<Option<IntervalHandle>>) {
    App::use_cleanup(move || {
        if let Some(h) = handle.get() {
            h.clear();
        }
    });
}

/// Renders the counter tab content for the keep-alive demo.
///
/// Maintains its own counter signal that persists when the tab is hidden
/// via CSS `display: none` instead of being destroyed.
///
/// # Returns
///
/// - `VirtualNode` - The counter tab virtual DOM tree.
fn counter_tab() -> VirtualNode {
    let count: Signal<i32> = App::use_signal(|| 0);
    html! {
        div {
            class: c_keep_alive_tab_panel()
            h4 {
                class: c_keep_alive_panel_title()
                "Counter"
            }
            p {
                class: c_keep_alive_demo_text()
                "This counter preserves its value when you switch tabs and come back."
            }
            div {
                class: c_keep_alive_counter_display()
                span {
                    class: c_keep_alive_counter_value()
                    count.get()
                }
            }
            div {
                class: c_keep_alive_counter_controls()
                euv_button {
                    variant: EuvButtonVariant::Primary
                    label: "-1"
                    onclick: keep_alive_counter_on_decrement(count)
                }
                euv_button {
                    variant: EuvButtonVariant::Primary
                    label: "+1"
                    onclick: keep_alive_counter_on_increment(count)
                }
                euv_button {
                    variant: EuvButtonVariant::Primary
                    label: "Reset"
                    onclick: keep_alive_counter_on_reset(count)
                }
            }
        }
    }
}

/// Renders the form tab content for the keep-alive demo.
///
/// Contains text inputs and a textarea whose values persist when the tab
/// is hidden via CSS `display: none` instead of being destroyed.
///
/// # Returns
///
/// - `VirtualNode` - The form tab virtual DOM tree.
fn form_tab() -> VirtualNode {
    let name: Signal<String> = App::use_signal(String::new);
    let email: Signal<String> = App::use_signal(String::new);
    let message: Signal<String> = App::use_signal(String::new);
    html! {
        div {
            class: c_keep_alive_tab_panel()
            h4 {
                class: c_keep_alive_panel_title()
                "Form"
            }
            p {
                class: c_keep_alive_demo_text()
                "Type something in the fields, switch tabs, and come back — your input is preserved."
            }
            div {
                class: c_keep_alive_form_group()
                label {
                    for: KEEP_ALIVE_NAME_ID
                    class: c_form_label()
                    "Name"
                }
                euv_input {
                    id: KEEP_ALIVE_NAME_ID
                    name: KEEP_ALIVE_NAME_NAME
                    input_type: KEEP_ALIVE_TEXT_TYPE
                    autocomplete: KEEP_ALIVE_AUTOCOMPLETE_NAME
                    placeholder: KEEP_ALIVE_NAME_PLACEHOLDER
                    value: name
                    oninput: UseEuvInput::on_input_value(name)
                }
            }
            div {
                class: c_keep_alive_form_group()
                label {
                    for: KEEP_ALIVE_EMAIL_ID
                    class: c_form_label()
                    "Email"
                }
                euv_input {
                    id: KEEP_ALIVE_EMAIL_ID
                    name: KEEP_ALIVE_EMAIL_NAME
                    input_type: KEEP_ALIVE_EMAIL_TYPE
                    autocomplete: KEEP_ALIVE_AUTOCOMPLETE_EMAIL
                    placeholder: KEEP_ALIVE_EMAIL_PLACEHOLDER
                    value: email
                    oninput: UseEuvInput::on_input_value(email)
                }
            }
            div {
                class: c_keep_alive_form_group()
                label {
                    for: KEEP_ALIVE_MESSAGE_ID
                    class: c_form_label()
                    "Message"
                }
                textarea {
                    id: KEEP_ALIVE_MESSAGE_ID
                    name: KEEP_ALIVE_MESSAGE_NAME
                    autocomplete: KEEP_ALIVE_AUTOCOMPLETE_OFF
                    placeholder: KEEP_ALIVE_MESSAGE_PLACEHOLDER
                    value: message.get()
                    class: c_textarea_input()
                    rows: KEEP_ALIVE_MESSAGE_ROWS
                    oninput: UseEuvInput::on_input_value(message)
                }
            }
            if { !name.get().is_empty() || !email.get().is_empty() || !message.get().is_empty() } {
                div {
                    class: c_keep_alive_form_preview()
                    p {
                        class: c_keep_alive_preview_label()
                        "Live Preview:"
                    }
                    p {
                        class: c_keep_alive_demo_text()
                        format!("Name: {} | Email: {} | Message: {}", name.get(), email.get(), message.get())
                    }
                }
            }
        }
    }
}

/// Renders the timer tab content for the keep-alive demo.
///
/// Maintains an auto-incrementing timer that continues running even when
/// the tab is hidden via CSS `display: none`, demonstrating that hooks
/// and intervals stay alive.
///
/// # Returns
///
/// - `VirtualNode` - The timer tab virtual DOM tree.
fn timer_tab() -> VirtualNode {
    let elapsed: Signal<i32> = App::use_signal(|| 0);
    let running: Signal<bool> = App::use_signal(|| false);
    let handle: Signal<Option<IntervalHandle>> = App::use_signal(|| None);
    keep_alive_cleanup(handle);
    watch!(running, |is_running: bool| {
        if is_running {
            let elapsed_signal: Signal<i32> = elapsed;
            let handle_signal: Signal<Option<IntervalHandle>> = handle;
            let new_handle: IntervalHandle = App::use_interval(1000, move || {
                let current: i32 = elapsed_signal.get();
                elapsed_signal.set(current + 1);
            });
            handle_signal.set(Some(new_handle));
        } else {
            let handle_opt: Option<IntervalHandle> = handle.get();
            if let Some(existing_handle) = handle_opt {
                existing_handle.clear();
            }
            handle.set(None);
        }
    });
    html! {
        div {
            class: c_keep_alive_tab_panel()
            h4 {
                class: c_keep_alive_panel_title()
                "Timer"
            }
            p {
                class: c_keep_alive_demo_text()
                "Start the timer, switch tabs, and come back — it keeps running in the background!"
            }
            div {
                class: c_keep_alive_counter_display()
                span {
                    class: c_keep_alive_counter_value()
                    format_time(elapsed.get())
                }
            }
            div {
                class: c_keep_alive_counter_controls()
                if { !running.get() } {
                    euv_button {
                        variant: EuvButtonVariant::Primary
                        label: "Start"
                        onclick: keep_alive_timer_on_start(running)
                    }
                } else {
                    euv_button {
                        variant: EuvButtonVariant::Primary
                        label: "Pause"
                        onclick: keep_alive_timer_on_pause(running, handle)
                    }
                }
                euv_button {
                    variant: EuvButtonVariant::Primary
                    label: "Reset"
                    onclick: keep_alive_timer_on_reset(elapsed, running, handle)
                }
            }
        }
    }
}

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
    format!("{minutes:02}:{seconds:02}")
}

/// Creates a click event handler that increments the counter signal.
///
/// # Arguments
///
/// - `Signal<i32>` - The counter signal to increment.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - A click handler that increments the counter.
pub(crate) fn keep_alive_counter_on_increment(count: Signal<i32>) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |_: Event| {
        let current: i32 = count.get();
        count.set(current + 1);
    }))
}

/// Creates a click event handler that decrements the counter signal.
///
/// # Arguments
///
/// - `Signal<i32>` - The counter signal to decrement.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - A click handler that decrements the counter.
pub(crate) fn keep_alive_counter_on_decrement(count: Signal<i32>) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |_: Event| {
        let current: i32 = count.get();
        count.set(current - 1);
    }))
}

/// Creates a click event handler that resets the counter signal to zero.
///
/// # Arguments
///
/// - `Signal<i32>` - The counter signal to reset.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - A click handler that resets the counter.
pub(crate) fn keep_alive_counter_on_reset(count: Signal<i32>) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |_: Event| {
        count.set(0);
    }))
}

/// Creates a click event handler that starts the timer.
///
/// # Arguments
///
/// - `Signal<bool>` - The running signal to set to true.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - A click handler that starts the timer.
pub(crate) fn keep_alive_timer_on_start(running: Signal<bool>) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |_: Event| {
        running.set(true);
    }))
}

/// Creates a click event handler that pauses the timer.
///
/// # Arguments
///
/// - `Signal<bool>` - The running signal to set to false.
/// - `Signal<Option<IntervalHandle>>` - The handle signal for clearing the interval.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - A click handler that pauses the timer.
pub(crate) fn keep_alive_timer_on_pause(
    running: Signal<bool>,
    _handle: Signal<Option<IntervalHandle>>,
) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |_: Event| {
        running.set(false);
    }))
}

/// Creates a click event handler that resets the timer.
///
/// # Arguments
///
/// - `Signal<i32>` - The elapsed signal to reset to zero.
/// - `Signal<bool>` - The running signal to set to false.
/// - `Signal<Option<IntervalHandle>>` - The handle signal for clearing the interval.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - A click handler that resets the timer.
pub(crate) fn keep_alive_timer_on_reset(
    elapsed: Signal<i32>,
    running: Signal<bool>,
    _handle: Signal<Option<IntervalHandle>>,
) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |_: Event| {
        running.set(false);
        elapsed.set(0);
    }))
}

/// A keep-alive demo page demonstrating state preservation across tab switches.
///
/// Uses CSS `display: none` to hide inactive tab content instead of
/// destroying and recreating it, which preserves all hook state (signals,
/// intervals, form inputs) across tab switches.
///
/// # Returns
///
/// - `VirtualNode` - The keep-alive demo page virtual DOM tree.
#[component]
pub(crate) fn page_keep_alive(node: VirtualNode<PageKeepAliveProps>) -> VirtualNode {
    let PageKeepAliveProps: PageKeepAliveProps = node.try_get_props().unwrap_or_default();
    let tab: Signal<KeepAliveTab> = App::use_signal(KeepAliveTab::default);
    html! {
        div {
            class: c_page_container()
            euv_header {
                icon: "💚"
                title: "Keep-Alive"
                subtitle: "Keep component state alive across tab switches using CSS display toggling."
            }
            euv_card {
                title: "Tab Switching with State Preservation"
                div {
                    class: c_keep_alive_tab_bar()
                    div {
                        class: if { tab.get() == KeepAliveTab::Counter } {
                            c_tab_item_active()
                        } else {
                            c_tab_item_inactive()
                        }
                        onclick: keep_alive_tab_on_select(tab, KeepAliveTab::Counter)
                        KeepAliveTab::Counter.to_string()
                    }
                    div {
                        class: if { tab.get() == KeepAliveTab::Form } {
                            c_tab_item_active()
                        } else {
                            c_tab_item_inactive()
                        }
                        onclick: keep_alive_tab_on_select(tab, KeepAliveTab::Form)
                        KeepAliveTab::Form.to_string()
                    }
                    div {
                        class: if { tab.get() == KeepAliveTab::Timer } {
                            c_tab_item_active()
                        } else {
                            c_tab_item_inactive()
                        }
                        onclick: keep_alive_tab_on_select(tab, KeepAliveTab::Timer)
                        KeepAliveTab::Timer.to_string()
                    }
                }
                div {
                    class: if { tab.get() == KeepAliveTab::Counter } {
                        c_keep_alive_tab_visible()
                    } else {
                        c_keep_alive_tab_hidden()
                    }
                    counter_tab()
                }
                div {
                    class: if { tab.get() == KeepAliveTab::Form } {
                        c_keep_alive_tab_visible()
                    } else {
                        c_keep_alive_tab_hidden()
                    }
                    form_tab()
                }
                div {
                    class: if { tab.get() == KeepAliveTab::Timer } {
                        c_keep_alive_tab_visible()
                    } else {
                        c_keep_alive_tab_hidden()
                    }
                    timer_tab()
                }
            }
            euv_card {
                title: "How It Works"
                p {
                    class: c_keep_alive_demo_text()
                    KEEP_ALIVE_DESCRIPTION
                }
            }
        }
    }
}
