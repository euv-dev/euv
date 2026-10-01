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
                title: BROWSER_PAGE_TITLE
                subtitle: BROWSER_PAGE_SUBTITLE
            }
            euv_card {
                title: BROWSER_LOCAL_STORAGE_CARD_TITLE
                p {
                    class: c_demo_text()
                    BROWSER_LOCAL_STORAGE_CARD_DESC
                }
                div {
                    class: c_browser_api_row()
                    euv_field {
                        id: LOCAL_STORAGE_KEY_ID
                        name: LOCAL_STORAGE_KEY_NAME
                        label: "Key"
                        input_type: BROWSER_TEXT_TYPE
                        placeholder: LOCAL_STORAGE_KEY_PLACEHOLDER
                        autocomplete: BROWSER_AUTOCOMPLETE_OFF
                        value: state.get_local_key()
                        error: None
                    }
                    euv_field {
                        id: LOCAL_STORAGE_VALUE_ID
                        name: LOCAL_STORAGE_VALUE_NAME
                        label: BROWSER_VALUE_LABEL
                        input_type: BROWSER_TEXT_TYPE
                        placeholder: LOCAL_STORAGE_VALUE_PLACEHOLDER
                        autocomplete: BROWSER_AUTOCOMPLETE_OFF
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
                        label: BROWSER_REMOVE_BUTTON_LABEL
                        onclick: state.on_local_storage_remove()
                    }
                }
                div {
                    class: c_browser_result_box()
                    span {
                        class: c_browser_result_label()
                        BROWSER_RESULT_PREFIX
                    }
                    span {
                        class: c_browser_result_value()
                        state.get_local_result()
                    }
                }
            }
            euv_card {
                title: BROWSER_SESSION_STORAGE_CARD_TITLE
                p {
                    class: c_demo_text()
                    BROWSER_SESSION_STORAGE_CARD_DESC
                }
                div {
                    class: c_browser_api_row()
                    euv_field {
                        id: SESSION_STORAGE_KEY_ID
                        name: SESSION_STORAGE_KEY_NAME
                        label: "Key"
                        input_type: BROWSER_TEXT_TYPE
                        placeholder: SESSION_STORAGE_KEY_PLACEHOLDER
                        autocomplete: BROWSER_AUTOCOMPLETE_OFF
                        value: state.get_session_key()
                        error: None
                    }
                    euv_field {
                        id: SESSION_STORAGE_VALUE_ID
                        name: SESSION_STORAGE_VALUE_NAME
                        label: BROWSER_VALUE_LABEL
                        input_type: BROWSER_TEXT_TYPE
                        placeholder: SESSION_STORAGE_VALUE_PLACEHOLDER
                        autocomplete: BROWSER_AUTOCOMPLETE_OFF
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
                        label: BROWSER_REMOVE_BUTTON_LABEL
                        onclick: state.on_session_storage_remove()
                    }
                }
                div {
                    class: c_browser_result_box()
                    span {
                        class: c_browser_result_label()
                        BROWSER_RESULT_PREFIX
                    }
                    span {
                        class: c_browser_result_value()
                        state.get_session_result()
                    }
                }
            }
            euv_card {
                title: BROWSER_CLIPBOARD_CARD_TITLE
                p {
                    class: c_demo_text()
                    BROWSER_CLIPBOARD_CARD_DESC
                }
                euv_field {
                    id: CLIPBOARD_TEXT_ID
                    name: CLIPBOARD_TEXT_NAME
                    label: BROWSER_CLIPBOARD_TEXT_LABEL
                    input_type: BROWSER_TEXT_TYPE
                    placeholder: CLIPBOARD_TEXT_PLACEHOLDER
                    autocomplete: BROWSER_AUTOCOMPLETE_OFF
                    value: state.get_clipboard_text()
                    error: None
                }
                div {
                    class: c_browser_api_actions()
                    euv_button {
                        variant: EuvButtonVariant::Primary
                        label: BROWSER_COPY_BUTTON_LABEL
                        onclick: state.on_clipboard_copy()
                    }
                    euv_button {
                        variant: EuvButtonVariant::Primary
                        label: BROWSER_PASTE_BUTTON_LABEL
                        onclick: state.on_clipboard_paste()
                    }
                }
                div {
                    class: c_browser_result_box()
                    span {
                        class: c_browser_result_label()
                        BROWSER_RESULT_PREFIX
                    }
                    span {
                        class: c_browser_result_value()
                        state.get_clipboard_result()
                    }
                }
            }
            euv_card {
                title: BROWSER_WINDOW_CARD_TITLE
                p {
                    class: c_demo_text()
                    BROWSER_WINDOW_CARD_DESC
                }
                div {
                    class: c_button_controls()
                    euv_button {
                        variant: EuvButtonVariant::Primary
                        label: BROWSER_REFRESH_SIZE_BUTTON_LABEL
                        onclick: state.on_window_refresh_size()
                    }
                }
                div {
                    class: c_browser_info_grid()
                    div {
                        class: c_browser_info_item()
                        span {
                            class: c_browser_info_label()
                            BROWSER_INNER_SIZE_LABEL
                        }
                        span {
                            class: c_browser_info_value()
                            state.get_window_size()
                        }
                    }
                }
            }
            euv_card {
                title: BROWSER_NAVIGATOR_CARD_TITLE
                p {
                    class: c_demo_text()
                    BROWSER_NAVIGATOR_CARD_DESC
                }
                div {
                    class: c_browser_info_grid()
                    div {
                        class: c_browser_info_item()
                        span {
                            class: c_browser_info_label()
                            BROWSER_USER_AGENT_LABEL
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
                            BROWSER_LANGUAGE_LABEL
                        }
                        span {
                            class: c_browser_info_value()
                            state.get_language()
                        }
                    }
                }
            }
            euv_card {
                title: BROWSER_LOCATION_CARD_TITLE
                p {
                    class: c_demo_text()
                    BROWSER_LOCATION_CARD_DESC
                }
                div {
                    class: c_browser_info_grid()
                    div {
                        class: c_browser_info_item()
                        span {
                            class: c_browser_info_label()
                            BROWSER_HREF_LABEL
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
                            BROWSER_ORIGIN_LABEL
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
                            BROWSER_PATHNAME_LABEL
                        }
                        span {
                            class: c_browser_info_value()
                            state.get_location_pathname_val()
                        }
                    }
                }
            }
            euv_card {
                title: BROWSER_CONSOLE_CARD_TITLE
                p {
                    class: c_demo_text()
                    BROWSER_CONSOLE_CARD_DESC
                }
                euv_field {
                    id: CONSOLE_MESSAGE_ID
                    name: CONSOLE_MESSAGE_NAME
                    label: BROWSER_CONSOLE_MESSAGE_LABEL
                    input_type: BROWSER_TEXT_TYPE
                    placeholder: CONSOLE_MESSAGE_PLACEHOLDER
                    autocomplete: BROWSER_AUTOCOMPLETE_OFF
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
                        label: BROWSER_WARN_BUTTON_LABEL
                        onclick: UseEuvBrowser::on_console_warn(state.get_console_input())
                    }
                    euv_button {
                        variant: EuvButtonVariant::Primary
                        label: BROWSER_ERROR_BUTTON_LABEL
                        onclick: UseEuvBrowser::on_console_error(state.get_console_input())
                    }
                }
            }
        }
    }
}
