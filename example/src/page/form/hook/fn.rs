use super::*;

/// Creates form state signals wrapped in a `UseForm` struct.
///
/// # Returns
///
/// - `UseForm` - The form state.
pub(crate) fn use_form() -> UseForm {
    UseForm::default()
}

/// Validates the username field and updates the error signal.
///
/// # Arguments
///
/// - `UseForm` - The form state.
pub(crate) fn validate_form_username(state: UseForm) {
    let username_value: String = state.get_username().get();
    if username_value.trim().is_empty() {
        state
            .get_username_error()
            .set(FORM_USERNAME_REQUIRED_MESSAGE.to_string());
    } else {
        state.get_username_error().set(String::new());
    }
}

/// Validates the email field and updates the error signal.
///
/// # Arguments
///
/// - `UseForm` - The form state.
pub(crate) fn validate_form_email(state: UseForm) {
    let email_value: String = state.get_email().get();
    if email_value.trim().is_empty() {
        state
            .get_email_error()
            .set(FORM_EMAIL_REQUIRED_MESSAGE.to_string());
    } else if !email_value.contains('@') || !email_value.contains('.') {
        state
            .get_email_error()
            .set(FORM_EMAIL_INVALID_MESSAGE.to_string());
    } else {
        state.get_email_error().set(String::new());
    }
}

/// Validates the password field and updates the error signal.
///
/// # Arguments
///
/// - `UseForm` - The form state.
pub(crate) fn validate_form_password(state: UseForm) {
    let password_value: String = state.get_password().get();
    if password_value.is_empty() {
        state
            .get_password_error()
            .set(FORM_PASSWORD_REQUIRED_MESSAGE.to_string());
    } else if password_value.len() < 6 {
        state
            .get_password_error()
            .set(FORM_PASSWORD_TOO_SHORT_MESSAGE.to_string());
    } else {
        state.get_password_error().set(String::new());
    }
}

/// Validates the agree checkbox and updates the error signal.
///
/// # Arguments
///
/// - `UseForm` - The form state.
pub(crate) fn validate_form_agree(state: UseForm) {
    let agree_value: bool = state.get_agree().get();
    if !agree_value {
        state
            .get_agree_error()
            .set(FORM_AGREE_REQUIRED_MESSAGE.to_string());
    } else {
        state.get_agree_error().set(String::new());
    }
}

/// Validates all form fields and updates the error signals.
///
/// # Arguments
///
/// - `UseForm` - The form state.
pub(crate) fn validate_form_all(state: UseForm) {
    validate_form_username(state);
    validate_form_email(state);
    validate_form_password(state);
    validate_form_agree(state);
}

/// Creates an input event handler that updates the username and validates it.
///
/// # Arguments
///
/// - `UseForm` - The form state.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - An input handler.
pub(crate) fn form_on_input_username(state: UseForm) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |event: Event| {
        if let Some(target) = event.target()
            && let Ok(input) = target.clone().dyn_into::<HtmlInputElement>()
        {
            state.get_username().set(input.value());
        }
        validate_form_username(state);
    }))
}

/// Creates an input event handler that updates the email and validates it.
///
/// # Arguments
///
/// - `UseForm` - The form state.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - An input handler.
pub(crate) fn form_on_input_email(state: UseForm) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |event: Event| {
        if let Some(target) = event.target()
            && let Ok(input) = target.clone().dyn_into::<HtmlInputElement>()
        {
            state.get_email().set(input.value());
        }
        validate_form_email(state);
    }))
}

/// Creates an input event handler that updates the password and validates it.
///
/// # Arguments
///
/// - `UseForm` - The form state.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - An input handler.
pub(crate) fn form_on_input_password(state: UseForm) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |event: Event| {
        if let Some(target) = event.target()
            && let Ok(input) = target.clone().dyn_into::<HtmlInputElement>()
        {
            state.get_password().set(input.value());
        }
        validate_form_password(state);
    }))
}

/// Creates a click event handler that validates and submits the form.
///
/// # Arguments
///
/// - `UseForm` - The form state.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - A click handler for form submission.
pub(crate) fn form_on_submit(state: UseForm) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |_: Event| {
        validate_form_all(state);
        let username_error_value: String = state.get_username_error().get();
        let email_error_value: String = state.get_email_error().get();
        let password_error_value: String = state.get_password_error().get();
        let agree_error_value: String = state.get_agree_error().get();
        let mut validation_errors: Vec<String> = Vec::new();
        if !username_error_value.is_empty() {
            validation_errors.push(username_error_value);
        }
        if !email_error_value.is_empty() {
            validation_errors.push(email_error_value);
        }
        if !password_error_value.is_empty() {
            validation_errors.push(password_error_value);
        }
        if !agree_error_value.is_empty() {
            validation_errors.push(agree_error_value);
        }
        if validation_errors.is_empty() {
            state.get_errors().set(String::new());
            state.get_submitted().set(format!(
                "Submitted: username={}, email={}",
                state.get_username().get(),
                state.get_email().get()
            ));
        } else {
            state.get_errors().set(validation_errors.join("; "));
        }
    }))
}
