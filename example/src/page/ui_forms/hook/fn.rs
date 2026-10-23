use super::*;

/// Builds the reactive state the form demo page drives.
///
/// # Returns
///
/// - `FormDemoState` - The signals the page drives.
pub(crate) fn use_form_demo_state() -> FormDemoState {
    FormDemoState {
        select_value: App::use_signal(|| String::from(FORM_DEFAULT_FRUIT)),
        checked_values: App::use_signal(|| vec![String::from(FORM_DEFAULT_TOPPING)]),
        segmented_value: App::use_signal(|| String::from(FORM_DEFAULT_VIEW)),
        number_value: App::use_signal(|| FORM_DEFAULT_QUANTITY),
        query: App::use_signal(String::new),
        date_value: App::use_signal(String::new),
    }
}

/// Builds the handler that reports the form's current state.
///
/// # Arguments
///
/// - `FormDemoState` - The signals the page drives.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn()>>` - The submit handler.
pub(crate) fn ui_forms_on_submit(state: FormDemoState) -> Option<Rc<dyn Fn()>> {
    Some(Rc::new(move || {
        let message: String = format!(
            "{FORM_SUBMIT_LOG}\n{}: {}\n{}: {}\n{}: {}\n{}: {}",
            FORM_SUMMARY_FRUIT,
            state.select_value.get(),
            FORM_SUMMARY_TOPPING,
            state.checked_values.get().join(", "),
            FORM_SUMMARY_VIEW,
            state.segmented_value.get(),
            FORM_SUMMARY_QUANTITY,
            state.number_value.get(),
        );
        Console::log(&message);
    }))
}

/// Builds the handler that reports the picked fruit.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(&'static str)>>` - The change handler of the select.
pub(crate) fn ui_forms_on_select() -> Option<Rc<dyn Fn(&'static str)>> {
    Some(Rc::new(move |picked: &'static str| {
        Console::log(FORM_SELECT_LOG.replace(FORM_LOG_VALUE_PLACEHOLDER, picked));
    }))
}

/// Builds the handler that reports the chosen view.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(&'static str)>>` - The change handler of the segmented
///   control.
pub(crate) fn ui_forms_on_segmented() -> Option<Rc<dyn Fn(&'static str)>> {
    Some(Rc::new(move |picked: &'static str| {
        Console::log(FORM_SEGMENTED_LOG.replace(FORM_LOG_VALUE_PLACEHOLDER, picked));
    }))
}
