use super::*;

/// A form demo page with two-way binding and validation.
///
/// # Returns
///
/// - `VirtualNode` - The form demo page virtual DOM tree.
///
/// # Arguments
///
/// - `VirtualNode<PageFormProps>` - The `node` argument.
///
#[component]
pub(crate) fn page_form(node: VirtualNode<PageFormProps>) -> VirtualNode {
    let PageFormProps: PageFormProps = node.try_get_props().unwrap_or_default();
    let form: UseForm = use_form();
    html! {
        div {
            class: c_page_container()
            euv_header {
                icon: "📄"
                title: "Form Demo"
                subtitle: "A registration form with two-way Signal binding and real-time field validation."
            }
            euv_card {
                title: "Registration Form"
                euv_field {
                    id: "form-username"
                    name: "username"
                    label: "Username"
                    input_type: "text"
                    placeholder: "Enter username"
                    autocomplete: "username"
                    value: form.get_username()
                    error: Some(form.get_username_error())
                    oninput: form_on_input_username(form)
                }
                euv_field {
                    id: "form-email"
                    name: "email"
                    label: "Email"
                    input_type: "email"
                    placeholder: "Enter email"
                    autocomplete: "email"
                    value: form.get_email()
                    error: Some(form.get_email_error())
                    oninput: form_on_input_email(form)
                }
                euv_field {
                    id: "form-password"
                    name: "password"
                    label: "Password"
                    input_type: "password"
                    placeholder: "Enter password"
                    autocomplete: "new-password"
                    value: form.get_password()
                    error: Some(form.get_password_error())
                    oninput: form_on_input_password(form)
                }
                euv_checkbox {
                    id: "form-agree"
                    name: "agree"
                    autocomplete: "off"
                    checked: form.get_agree()
                    label: "I agree to the terms and conditions"
                }
                if { !form.get_agree_error().get().is_empty() } {
                    p {
                        class: c_field_error_text()
                        form.get_agree_error()
                    }
                }
                if { !form.get_errors().get().is_empty() } {
                    euv_alert {
                        variant: AlertVariant::Error
                        form.get_errors()
                    }
                }
                div {
                    class: c_button_controls()
                    euv_button {
                        variant: EuvButtonVariant::Primary
                        label: "Submit"
                        onclick: form_on_submit(form)
                    }
                }
                if { !form.get_submitted().get().is_empty() } {
                    euv_alert {
                        variant: AlertVariant::Success
                        form.get_submitted()
                    }
                }
            }
        }
    }
}
