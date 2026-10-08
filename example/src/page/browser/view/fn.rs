use super::*;

/// A browser API demo page showcasing localStorage, sessionStorage,
/// clipboard, window, navigator, and location.
///
/// # Arguments
///
/// - `VirtualNode<PageBrowserProps>` - The page component node carrying the
///   page props.
///
/// # Returns
///
/// - `VirtualNode` - The browser API demo page virtual DOM tree.
#[component]
pub(crate) fn page_browser(node: VirtualNode<PageBrowserProps>) -> VirtualNode {
    let PageBrowserProps: PageBrowserProps = node.try_get_props().unwrap_or_default();
    let state: UseEuvBrowser = UseEuvBrowser::use_browser_state();
    html! {
        div {
            class: c_page_container()
            euv_header {
                icon: "🌐"
                title: "Browser APIs"
                subtitle: "Interact with browser storage, clipboard, window metrics, navigator info, location URL, and developer console — all through euv's typed hook APIs."
            }
            euv_card {
                title: "localStorage"
                p {
                    class: c_demo_text()
                    "Store, retrieve, and remove persistent key-value data. Data in localStorage survives page reloads and browser restarts."
                }
                div {
                    class: c_browser_api_row()
                    euv_field {
                        id: "local-storage-key"
                        name: "local_key"
                        label: "Key"
                        input_type: "text"
                        placeholder: "Storage key..."
                        autocomplete: "off"
                        value: state.get_local_key()
                        error: None
                    }
                    euv_field {
                        id: "local-storage-value"
                        name: "local_value"
                        label: "Value"
                        input_type: "text"
                        placeholder: "Storage value..."
                        autocomplete: "off"
                        value: state.get_local_value()
                        error: None
                    }
                }
                div {
                    class: c_browser_api_actions()
                    euv_button {
                        variant: EuvButtonVariant::Primary
                        label: "Set"
                        onclick: state.on_local_storage_set()
                    }
                    euv_button {
                        variant: EuvButtonVariant::Primary
                        label: "Get"
                        onclick: state.on_local_storage_get()
                    }
                    euv_button {
                        variant: EuvButtonVariant::Primary
                        label: "Remove"
                        onclick: state.on_local_storage_remove()
                    }
                }
                div {
                    class: c_browser_result_box()
                    span {
                        class: c_browser_result_label()
                        "Result: "
                    }
                    span {
                        class: c_browser_result_value()
                        state.get_local_result()
                    }
                }
            }
            euv_card {
                title: "sessionStorage"
                p {
                    class: c_demo_text()
                    "Store key-value data for the duration of the page session. Data is cleared when the tab or window is closed."
                }
                div {
                    class: c_browser_api_row()
                    euv_field {
                        id: "session-storage-key"
                        name: "session_key"
                        label: "Key"
                        input_type: "text"
                        placeholder: "Session key..."
                        autocomplete: "off"
                        value: state.get_session_key()
                        error: None
                    }
                    euv_field {
                        id: "session-storage-value"
                        name: "session_value"
                        label: "Value"
                        input_type: "text"
                        placeholder: "Session value..."
                        autocomplete: "off"
                        value: state.get_session_value()
                        error: None
                    }
                }
                div {
                    class: c_browser_api_actions()
                    euv_button {
                        variant: EuvButtonVariant::Primary
                        label: "Set"
                        onclick: state.on_session_storage_set()
                    }
                    euv_button {
                        variant: EuvButtonVariant::Primary
                        label: "Get"
                        onclick: state.on_session_storage_get()
                    }
                    euv_button {
                        variant: EuvButtonVariant::Primary
                        label: "Remove"
                        onclick: state.on_session_storage_remove()
                    }
                }
                div {
                    class: c_browser_result_box()
                    span {
                        class: c_browser_result_label()
                        "Result: "
                    }
                    span {
                        class: c_browser_result_value()
                        state.get_session_result()
                    }
                }
            }
            euv_card {
                title: "Clipboard API"
                p {
                    class: c_demo_text()
                    "Write text to the system clipboard or read the current clipboard contents. Requires a secure context (HTTPS or localhost)."
                }
                euv_field {
                    id: "clipboard-text"
                    name: "clipboard_text"
                    label: "Text to copy"
                    input_type: "text"
                    placeholder: "Enter text to copy..."
                    autocomplete: "off"
                    value: state.get_clipboard_text()
                    error: None
                }
                div {
                    class: c_browser_api_actions()
                    euv_button {
                        variant: EuvButtonVariant::Primary
                        label: "Copy"
                        onclick: state.on_clipboard_copy()
                    }
                    euv_button {
                        variant: EuvButtonVariant::Primary
                        label: "Paste"
                        onclick: state.on_clipboard_paste()
                    }
                }
                div {
                    class: c_browser_result_box()
                    span {
                        class: c_browser_result_label()
                        "Result: "
                    }
                    span {
                        class: c_browser_result_value()
                        state.get_clipboard_result()
                    }
                }
            }
            euv_card {
                title: "Window"
                p {
                    class: c_demo_text()
                    "Read the browser window's inner width and height in CSS pixels. Click Refresh Size after resizing the window."
                }
                div {
                    class: c_button_controls()
                    euv_button {
                        variant: EuvButtonVariant::Primary
                        label: "Refresh Size"
                        onclick: state.on_window_refresh_size()
                    }
                }
                div {
                    class: c_browser_info_grid()
                    div {
                        class: c_browser_info_item()
                        span {
                            class: c_browser_info_label()
                            "Inner Size"
                        }
                        span {
                            class: c_browser_info_value()
                            state.get_window_size()
                        }
                    }
                }
            }
            euv_card {
                title: "Navigator"
                p {
                    class: c_demo_text()
                    "Read the browser's User-Agent string and preferred language. Useful for analytics, feature detection, and localization."
                }
                div {
                    class: c_browser_info_grid()
                    div {
                        class: c_browser_info_item()
                        span {
                            class: c_browser_info_label()
                            "User Agent"
                        }
                        span {
                            class: c_browser_info_value()
                            state.get_user_agent()
                        }
                    }
                    div {
                        class: c_browser_info_item()
                        span {
                            class: c_browser_info_label()
                            "Language"
                        }
                        span {
                            class: c_browser_info_value()
                            state.get_language()
                        }
                    }
                }
            }
            euv_card {
                title: "Location"
                p {
                    class: c_demo_text()
                    "Read the current page's full URL components: href, origin, and pathname. All values are read-only and update automatically on navigation."
                }
                div {
                    class: c_browser_info_grid()
                    div {
                        class: c_browser_info_item()
                        span {
                            class: c_browser_info_label()
                            "Href"
                        }
                        span {
                            class: c_browser_info_value()
                            state.get_location_url()
                        }
                    }
                    div {
                        class: c_browser_info_item()
                        span {
                            class: c_browser_info_label()
                            "Origin"
                        }
                        span {
                            class: c_browser_info_value()
                            state.get_location_origin_val()
                        }
                    }
                    div {
                        class: c_browser_info_item()
                        span {
                            class: c_browser_info_label()
                            "Pathname"
                        }
                        span {
                            class: c_browser_info_value()
                            state.get_location_pathname_val()
                        }
                    }
                }
            }
            euv_card {
                title: "Console"
                p {
                    class: c_demo_text()
                    "Send log, warning, and error messages to the browser developer console. Open DevTools (F12) to see the output."
                }
                euv_field {
                    id: "console-message"
                    name: "console_message"
                    label: "Console message"
                    input_type: "text"
                    placeholder: "Type a message to log..."
                    autocomplete: "off"
                    value: state.get_console_input()
                    error: None
                }
                div {
                    class: c_browser_api_actions()
                    euv_button {
                        variant: EuvButtonVariant::Primary
                        label: "Log"
                        onclick: UseEuvBrowser::on_console_log(state.get_console_input())
                    }
                    euv_button {
                        variant: EuvButtonVariant::Primary
                        label: "Warn"
                        onclick: UseEuvBrowser::on_console_warn(state.get_console_input())
                    }
                    euv_button {
                        variant: EuvButtonVariant::Primary
                        label: "Error"
                        onclick: UseEuvBrowser::on_console_error(state.get_console_input())
                    }
                }
            }
        }
    }
}
