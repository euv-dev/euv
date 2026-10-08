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
                title: "Modal Dialog"
                subtitle: "Overlay dialogs with different content patterns: basic text, confirmation, form input, and nested stacking with back-gesture support."
            }
            euv_card {
                title: "Basic Modal"
                p {
                    class: c_demo_text()
                    "A simple modal with static text content. Click the close button or the overlay backdrop to dismiss."
                }
                div {
                    class: c_button_controls()
                    euv_button {
                        variant: EuvButtonVariant::Primary
                        label: "Open"
                        onclick: modal_on_open_basic(state)
                    }
                }
            }
            euv_card {
                title: "Confirm Modal"
                p {
                    class: c_demo_text()
                    "A modal that requires the user to confirm or cancel an action. The result is displayed below the button."
                }
                div {
                    class: c_button_controls()
                    euv_button {
                        variant: EuvButtonVariant::Primary
                        label: "Open"
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
                title: "Form Modal"
                p {
                    class: c_demo_text()
                    "A modal containing validated form inputs. Submission is blocked until all validation rules pass."
                }
                div {
                    class: c_button_controls()
                    euv_button {
                        variant: EuvButtonVariant::Primary
                        label: "Open"
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
                title: "Nested Modals"
                p {
                    class: c_demo_text()
                    "Three stacked modal layers. The system back gesture closes them one at a time, newest first."
                }
                div {
                    class: c_button_controls()
                    euv_button {
                        variant: EuvButtonVariant::Primary
                        label: "Open"
                        onclick: modal_on_open_nested_1(state)
                    }
                }
            }
            if { state.get_show_basic().get() } {
                euv_modal {
                    title: "Basic Modal"
                    onclick: modal_dismiss_handler(state.get_show_basic())
                    p {
                        class: c_demo_text()
                        "This is a basic modal dialog. Click the close button or the overlay to dismiss."
                    }
                    p {
                        class: c_demo_text_muted()
                        "Modals are useful for displaying focused content that requires user attention."
                    }
                }
            }
            if { state.get_show_confirm().get() } {
                euv_modal {
                    title: "Confirm Action"
                    onclick: modal_dismiss_handler(state.get_show_confirm())
                    p {
                        class: c_demo_text()
                        "Are you sure you want to proceed with this action?"
                    }
                    div {
                        class: c_modal_actions()
                        euv_button {
                            variant: EuvButtonVariant::Primary
                            label: "Confirm"
                            onclick: modal_on_confirm(state)
                        }
                        euv_button {
                            variant: EuvButtonVariant::Primary
                            label: "Cancel"
                            onclick: modal_on_cancel_confirm(state)
                        }
                    }
                }
            }
            if { state.get_show_form().get() } {
                euv_modal {
                    title: "Quick Sign Up"
                    onclick: modal_dismiss_handler(state.get_show_form())
                    euv_field {
                        id: "modal-name"
                        name: "name"
                        label: "Name"
                        input_type: "text"
                        placeholder: "Enter your name"
                        autocomplete: "name"
                        value: state.get_modal_name()
                        error: Some(state.get_name_error())
                        oninput: modal_on_input_name(state)
                    }
                    euv_field {
                        id: "modal-email"
                        name: "email"
                        label: "Email"
                        input_type: "email"
                        placeholder: "Enter your email"
                        autocomplete: "email"
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
                            label: "Submit"
                            onclick: modal_on_form_submit(state)
                        }
                        euv_button {
                            variant: EuvButtonVariant::Primary
                            label: "Cancel"
                            onclick: modal_on_cancel_form(state)
                        }
                    }
                }
            }
            if { state.get_show_nested_1().get() } {
                euv_modal {
                    title: "Nested Modal · Layer 1"
                    onclick: modal_dismiss_handler(state.get_show_nested_1())
                    p {
                        class: c_demo_text()
                        "This is the first layer. Open another modal on top of it."
                    }
                    p {
                        class: c_demo_text_muted()
                        "Use the system back gesture to close the topmost layer first."
                    }
                    div {
                        class: c_modal_actions()
                        euv_button {
                            variant: EuvButtonVariant::Primary
                            label: "Open Layer 2"
                            onclick: modal_on_open_nested_2(state)
                        }
                    }
                }
            }
            if { state.get_show_nested_2().get() } {
                euv_modal {
                    title: "Nested Modal · Layer 2"
                    onclick: modal_dismiss_handler(state.get_show_nested_2())
                    p {
                        class: c_demo_text()
                        "This is the second layer, stacked over layer 1."
                    }
                    div {
                        class: c_modal_actions()
                        euv_button {
                            variant: EuvButtonVariant::Primary
                            label: "Open Layer 3"
                            onclick: modal_on_open_nested_3(state)
                        }
                    }
                }
            }
            if { state.get_show_nested_3().get() } {
                euv_modal {
                    title: "Nested Modal · Layer 3"
                    onclick: modal_dismiss_handler(state.get_show_nested_3())
                    p {
                        class: c_demo_text()
                        "This is the third and innermost layer."
                    }
                    p {
                        class: c_demo_text_muted()
                        "Press back three times: layer 3, then 2, then 1 close in order before leaving the page."
                    }
                }
            }
        }
    }
}
