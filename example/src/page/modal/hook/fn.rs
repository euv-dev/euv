use super::*;

/// Creates modal state signals wrapped in a `UseModal` struct.
///
/// # Returns
///
/// - `UseModal` - The modal state.
pub(crate) fn use_modal() -> UseModal {
    UseModal::default()
}

/// Opens a modal controlled by the given visibility signal and registers it
/// with the global back-gesture stack, supporting nested modals.
///
/// Sets the signal to `true` and pushes the modal (with a close callback and a
/// browser history entry) onto the global modal stack so that a system back
/// gesture dismisses the most recently opened modal instead of navigating to
/// the previous page. Opening a modal while another is already open simply
/// stacks on top, and back closes them newest-first. Use [`dismiss_modal`]
/// (not a bare `signal.set(false)`) to close it through the UI so the history
/// entry stays in sync.
///
/// # Convention
///
/// Any modal that should be dismissible by the system/browser back gesture
/// MUST be opened with [`open_modal`] and closed with [`dismiss_modal`] (or
/// the [`modal_dismiss_handler`] event handler). NEVER toggle the visibility
/// signal directly with `signal.set(true)` / `signal.set(false)` (or
/// `use_toggle`) for such modals: doing so bypasses the global modal stack and
/// browser history bookkeeping, which desynchronizes the history count and
/// breaks the "back closes the topmost modal" behavior (and nested modals).
///
/// # Arguments
///
/// - `Signal<bool>` - The visibility signal controlling the modal.
pub(crate) fn open_modal(visible: Signal<bool>) {
    visible.set(true);
    Router::modal_push(
        visible,
        Rc::new(move || {
            visible.set(false);
        }),
    );
}

/// Closes a modal that was opened via [`open_modal`] in response to a UI
/// action (close button, overlay click, confirm/cancel), keeping the browser
/// history in sync.
///
/// Removes the modal from the global stack and consumes the matching history
/// entry so a later back gesture behaves correctly. Sets the visibility signal
/// to `false` immediately to remove the modal from the DOM.
/// Do not call this from the back gesture path; the `popstate` handler invokes
/// the registered close callback directly in that case.
///
/// # Arguments
///
/// - `Signal<bool>` - The visibility signal controlling the modal.
pub(crate) fn dismiss_modal(visible: Signal<bool>) {
    Router::modal_close_via_ui(visible);
    visible.set(false);
}

/// Creates a click event handler that dismisses the modal and syncs the
/// browser history.
///
/// # Arguments
///
/// - `Signal<bool>` - The visibility signal controlling the modal.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - A click event handler that dismisses the modal.
pub(crate) fn modal_dismiss_handler(visible: Signal<bool>) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |_: Event| {
        if !visible.get() {
            return;
        }
        dismiss_modal(visible);
    }))
}

/// Validates the modal name field and updates the error signal.
///
/// # Arguments
///
/// - `UseModal` - The modal state.
pub(crate) fn validate_modal_name(state: UseModal) {
    let name_value: String = state.get_modal_name().get();
    if name_value.trim().is_empty() {
        state
            .get_name_error()
            .set(MODAL_ERROR_NAME_REQUIRED.to_string());
    } else {
        state.get_name_error().set(String::new());
    }
}

/// Validates the modal email field and updates the error signal.
///
/// # Arguments
///
/// - `UseModal` - The modal state.
pub(crate) fn validate_modal_email(state: UseModal) {
    let email_value: String = state.get_modal_email().get();
    if email_value.trim().is_empty() {
        state
            .get_email_error()
            .set(MODAL_ERROR_EMAIL_REQUIRED.to_string());
    } else if !email_value.contains('@') || !email_value.contains('.') {
        state
            .get_email_error()
            .set(MODAL_ERROR_EMAIL_INVALID.to_string());
    } else {
        state.get_email_error().set(String::new());
    }
}

/// Creates a click event handler that opens the basic modal.
///
/// # Arguments
///
/// - `UseModal` - The modal state.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - A click handler to open the basic modal.
pub(crate) fn modal_on_open_basic(state: UseModal) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |_: Event| {
        open_modal(state.get_show_basic());
    }))
}

/// Creates a click event handler that opens the confirm modal.
///
/// # Arguments
///
/// - `UseModal` - The modal state.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - A click handler to open the confirm modal.
pub(crate) fn modal_on_open_confirm(state: UseModal) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |_: Event| {
        state.get_confirm_result().set(String::new());
        open_modal(state.get_show_confirm());
    }))
}

/// Creates a click event handler that opens the form modal.
///
/// # Arguments
///
/// - `UseModal` - The modal state.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - A click handler to open the form modal.
pub(crate) fn modal_on_open_form(state: UseModal) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |_: Event| {
        state.get_modal_name().set(String::new());
        state.get_modal_email().set(String::new());
        state.get_modal_submitted().set(String::new());
        state.get_modal_error().set(String::new());
        state.get_name_error().set(String::new());
        state.get_email_error().set(String::new());
        open_modal(state.get_show_form());
    }))
}

/// Creates a click event handler that opens the first (outermost) nested modal layer.
///
/// # Arguments
///
/// - `UseModal` - The modal state.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - A click handler to open the first nested layer.
pub(crate) fn modal_on_open_nested_1(state: UseModal) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |_: Event| {
        open_modal(state.get_show_nested_1());
    }))
}

/// Creates a click event handler that opens the second nested modal layer
/// on top of the first.
///
/// # Arguments
///
/// - `UseModal` - The modal state.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - A click handler to open the second nested layer.
pub(crate) fn modal_on_open_nested_2(state: UseModal) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |_: Event| {
        open_modal(state.get_show_nested_2());
    }))
}

/// Creates a click event handler that opens the third (innermost) nested modal
/// layer on top of the second.
///
/// # Arguments
///
/// - `UseModal` - The modal state.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - A click handler to open the third nested layer.
pub(crate) fn modal_on_open_nested_3(state: UseModal) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |_: Event| {
        open_modal(state.get_show_nested_3());
    }))
}

/// Creates a click event handler that confirms the action and closes the confirm modal.
///
/// # Arguments
///
/// - `UseModal` - The modal state.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - A click handler to confirm the action.
pub(crate) fn modal_on_confirm(state: UseModal) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |_: Event| {
        if !state.get_show_confirm().get() {
            return;
        }
        state
            .get_confirm_result()
            .set(MODAL_STATUS_ACTION_CONFIRMED.to_string());
        dismiss_modal(state.get_show_confirm());
    }))
}

/// Creates a click event handler that cancels the confirm action and closes the confirm modal.
///
/// # Arguments
///
/// - `UseModal` - The modal state.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - A click handler to cancel the confirm action.
pub(crate) fn modal_on_cancel_confirm(state: UseModal) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |_: Event| {
        if !state.get_show_confirm().get() {
            return;
        }
        state
            .get_confirm_result()
            .set(MODAL_STATUS_ACTION_CANCELLED.to_string());
        dismiss_modal(state.get_show_confirm());
    }))
}

/// Creates an input event handler that updates the modal name and validates it.
///
/// # Arguments
///
/// - `UseModal` - The modal state.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - An input handler.
pub(crate) fn modal_on_input_name(state: UseModal) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |event: Event| {
        if let Some(target) = event.target()
            && let Ok(input) = target.clone().dyn_into::<HtmlInputElement>()
        {
            state.get_modal_name().set(input.value());
        }
        validate_modal_name(state);
    }))
}

/// Creates an input event handler that updates the modal email and validates it.
///
/// # Arguments
///
/// - `UseModal` - The modal state.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - An input handler.
pub(crate) fn modal_on_input_email(state: UseModal) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |event: Event| {
        if let Some(target) = event.target()
            && let Ok(input) = target.clone().dyn_into::<HtmlInputElement>()
        {
            state.get_modal_email().set(input.value());
        }
        validate_modal_email(state);
    }))
}

/// Creates a click event handler that submits the form modal.
///
/// # Arguments
///
/// - `UseModal` - The modal state.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - A click handler to submit the form modal.
pub(crate) fn modal_on_form_submit(state: UseModal) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |_: Event| {
        if !state.get_show_form().get() {
            return;
        }
        validate_modal_name(state);
        validate_modal_email(state);
        let name_error_value: String = state.get_name_error().get();
        let email_error_value: String = state.get_email_error().get();
        let mut validation_errors: Vec<String> = Vec::new();
        if !name_error_value.is_empty() {
            validation_errors.push(name_error_value);
        }
        if !email_error_value.is_empty() {
            validation_errors.push(email_error_value);
        }
        if validation_errors.is_empty() {
            state.get_modal_error().set(String::new());
            state.get_modal_submitted().set(format!(
                "Signed up: {} ({})",
                state.get_modal_name().get(),
                state.get_modal_email().get()
            ));
            dismiss_modal(state.get_show_form());
        } else {
            state.get_modal_error().set(validation_errors.join("; "));
        }
    }))
}

/// Creates a click event handler that cancels the form modal and shows a cancellation message.
///
/// # Arguments
///
/// - `UseModal` - The modal state.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - A click handler to cancel the form modal.
pub(crate) fn modal_on_cancel_form(state: UseModal) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |_: Event| {
        if !state.get_show_form().get() {
            return;
        }
        state
            .get_modal_submitted()
            .set(MODAL_STATUS_FORM_CANCELLED.to_string());
        dismiss_modal(state.get_show_form());
    }))
}
