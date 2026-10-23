use super::*;

/// Builds the submit handler of the form.
///
/// The handler cancels the browser's native submission — a form without an
/// action would otherwise navigate the page with a `GET` query — and then
/// notifies the caller.
///
/// # Arguments
///
/// - `Option<Rc<dyn Fn()>>` - The callback invoked on submit.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - The submit handler of the form.
fn on_form_submit(on_submit: Option<Rc<dyn Fn()>>) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |event: Event| {
        event.prevent_default();
        if let Some(callback) = &on_submit {
            callback();
        }
    }))
}

/// A form shell that wraps the caller's fields.
///
/// The component renders a `<form>` carrying the `c_euv_form` class and
/// wraps the children the caller passed in, so a page can drop
/// `euv_field`, `euv_input` or `euv_select` components straight into it and
/// get a consistent vertical rhythm without writing any markup.
///
/// When `submit_label` is not empty the form ends with a `c_euv_form_actions`
/// row holding a submit button built from `euv_button`, so the primary
/// action of the form looks like every other primary action in the app. The
/// submit handler cancels the browser's native submission, which keeps the
/// page from navigating with a `GET` query.
///
/// # Arguments
///
/// - `VirtualNode<EuvFormProps>` - The props node carrying the submit label
///   and the submit callback.
///
/// # Returns
///
/// - `VirtualNode` - The form element wrapping the caller's fields.
#[component]
pub fn euv_form(node: VirtualNode<EuvFormProps>) -> VirtualNode {
    let EuvFormProps {
        submit_label,
        on_submit,
    }: EuvFormProps = node.try_get_props().unwrap_or_default();
    let children: VirtualNode = node.get_children().into();
    html! {
        form {
            class: c_euv_form()
            onsubmit: on_form_submit(on_submit.clone())
            children
            if !submit_label.is_empty() {
                div {
                    class: c_euv_form_actions()
                    euv_button {
                        variant: EuvButtonVariant::Primary
                        label: submit_label
                    }
                }
            }
        }
    }
}
