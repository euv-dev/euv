use super::*;

/// A modal demo page showcasing different modal variations.
///
/// # Arguments
///
/// - `VirtualNode<PageModalProps>` - The page component node carrying the
///   page props.
///
/// # Returns
///
/// - `VirtualNode` - The modal demo page virtual DOM tree.
#[component]
pub(crate) fn page_modal(node: VirtualNode<PageModalProps>) -> VirtualNode {
    let PageModalProps: PageModalProps = node.try_get_props().unwrap_or_default();
    let state: UseModal = use_modal();
    html! {
        div {
            class: c_page_container()
            euv_header {
                icon: "💬"
                title: MODAL_PAGE_TITLE
                subtitle: MODAL_PAGE_SUBTITLE
            }
            euv_card {
                title: MODAL_BASIC_CARD_TITLE
                p {
                    class: c_demo_text()
                    MODAL_BASIC_CARD_DESC
                }
                div {
                    class: c_button_controls()
                    euv_button {
                        variant: EuvButtonVariant::Primary
                        label: MODAL_OPEN_BUTTON_LABEL
                        onclick: modal_on_open_basic(state)
                    }
                }
            }
            euv_card {
                title: MODAL_CONFIRM_CARD_TITLE
                p {
                    class: c_demo_text()
                    MODAL_CONFIRM_CARD_DESC
                }
                div {
                    class: c_button_controls()
                    euv_button {
                        variant: EuvButtonVariant::Primary
                        label: MODAL_OPEN_BUTTON_LABEL
                        onclick: modal_on_open_confirm(state)
                    }
                }
                if { !state.get_confirm_result().get().is_empty() } {
                    euv_alert {
                        variant: AlertVariant::Success
                        state.get_confirm_result()
                    }
                }
            }
            euv_card {
                title: MODAL_FORM_CARD_TITLE
                p {
                    class: c_demo_text()
                    MODAL_FORM_CARD_DESC
                }
                div {
                    class: c_button_controls()
                    euv_button {
                        variant: EuvButtonVariant::Primary
                        label: MODAL_OPEN_BUTTON_LABEL
                        onclick: modal_on_open_form(state)
                    }
                }
                if { !state.get_modal_submitted().get().is_empty() } {
                    euv_alert {
                        variant: AlertVariant::Success
                        state.get_modal_submitted()
                    }
                }
            }
            euv_card {
                title: MODAL_NESTED_CARD_TITLE
                p {
                    class: c_demo_text()
                    MODAL_NESTED_CARD_DESC
                }
                div {
                    class: c_button_controls()
                    euv_button {
                        variant: EuvButtonVariant::Primary
                        label: MODAL_OPEN_BUTTON_LABEL
                        onclick: modal_on_open_nested_1(state)
                    }
                }
            }
            if { state.get_show_basic().get() } {
                euv_modal {
                    title: MODAL_BASIC_CARD_TITLE
                    onclick: modal_dismiss_handler(state.get_show_basic())
                    p {
                        class: c_demo_text()
                        MODAL_BASIC_BODY_TEXT
                    }
                    p {
                        class: c_demo_text_muted()
                        MODAL_BASIC_BODY_HINT
                    }
                }
            }
            if { state.get_show_confirm().get() } {
                euv_modal {
                    title: MODAL_CONFIRM_ACTION_TITLE
                    onclick: modal_dismiss_handler(state.get_show_confirm())
                    p {
                        class: c_demo_text()
                        MODAL_CONFIRM_ACTION_TEXT
                    }
                    div {
                        class: c_modal_actions()
                        euv_button {
                            variant: EuvButtonVariant::Primary
                            label: MODAL_CONFIRM_BUTTON_LABEL
                            onclick: modal_on_confirm(state)
                        }
                        euv_button {
                            variant: EuvButtonVariant::Primary
                            label: MODAL_CANCEL_BUTTON_LABEL
                            onclick: modal_on_cancel_confirm(state)
                        }
                    }
                }
            }
            if { state.get_show_form().get() } {
                euv_modal {
                    title: MODAL_SIGN_UP_TITLE
                    onclick: modal_dismiss_handler(state.get_show_form())
                    euv_field {
                        id: MODAL_NAME_ID
                        name: MODAL_NAME_NAME
                        label: MODAL_SIGN_UP_NAME_LABEL
                        input_type: MODAL_TEXT_TYPE
                        placeholder: MODAL_NAME_PLACEHOLDER
                        autocomplete: MODAL_AUTOCOMPLETE_NAME
                        value: state.get_modal_name()
                        error: Some(state.get_name_error())
                        oninput: modal_on_input_name(state)
                    }
                    euv_field {
                        id: MODAL_EMAIL_ID
                        name: MODAL_EMAIL_NAME
                        label: MODAL_SIGN_UP_EMAIL_LABEL
                        input_type: MODAL_EMAIL_TYPE
                        placeholder: MODAL_EMAIL_PLACEHOLDER
                        autocomplete: MODAL_AUTOCOMPLETE_EMAIL
                        value: state.get_modal_email()
                        error: Some(state.get_email_error())
                        oninput: modal_on_input_email(state)
                    }
                    if { !state.get_modal_error().get().is_empty() } {
                        euv_alert {
                            variant: AlertVariant::Error
                            state.get_modal_error()
                        }
                    }
                    div {
                        class: c_modal_actions()
                        euv_button {
                            variant: EuvButtonVariant::Primary
                            label: MODAL_SUBMIT_BUTTON_LABEL
                            onclick: modal_on_form_submit(state)
                        }
                        euv_button {
                            variant: EuvButtonVariant::Primary
                            label: MODAL_CANCEL_BUTTON_LABEL
                            onclick: modal_on_cancel_form(state)
                        }
                    }
                }
            }
            if { state.get_show_nested_1().get() } {
                euv_modal {
                    title: MODAL_LAYER_1_TITLE
                    onclick: modal_dismiss_handler(state.get_show_nested_1())
                    p {
                        class: c_demo_text()
                        MODAL_LAYER_1_TEXT
                    }
                    p {
                        class: c_demo_text_muted()
                        MODAL_LAYER_1_HINT
                    }
                    div {
                        class: c_modal_actions()
                        euv_button {
                            variant: EuvButtonVariant::Primary
                            label: MODAL_OPEN_LAYER_2_BUTTON_LABEL
                            onclick: modal_on_open_nested_2(state)
                        }
                    }
                }
            }
            if { state.get_show_nested_2().get() } {
                euv_modal {
                    title: MODAL_LAYER_2_TITLE
                    onclick: modal_dismiss_handler(state.get_show_nested_2())
                    p {
                        class: c_demo_text()
                        MODAL_LAYER_2_TEXT
                    }
                    div {
                        class: c_modal_actions()
                        euv_button {
                            variant: EuvButtonVariant::Primary
                            label: MODAL_OPEN_LAYER_3_BUTTON_LABEL
                            onclick: modal_on_open_nested_3(state)
                        }
                    }
                }
            }
            if { state.get_show_nested_3().get() } {
                euv_modal {
                    title: MODAL_LAYER_3_TITLE
                    onclick: modal_dismiss_handler(state.get_show_nested_3())
                    p {
                        class: c_demo_text()
                        MODAL_LAYER_3_TEXT
                    }
                    p {
                        class: c_demo_text_muted()
                        MODAL_NESTED_HINT
                    }
                }
            }
        }
    }
}
