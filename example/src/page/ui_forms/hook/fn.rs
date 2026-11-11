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
        toggle_values: App::use_signal(|| vec![String::from(FORM_TOGGLE_FIRST_VALUE)]),
        toggle_pressed: App::use_signal(|| false),
        accordion_open: App::use_signal(Vec::new),
        notes: App::use_signal(String::new),
        group_query: App::use_signal(String::new),
        field_disabled: App::use_signal(|| false),
        radio_first: App::use_signal(|| true),
        radio_second: App::use_signal(|| false),
        menubar_open: App::use_signal(|| false),
        menubar_edit_open: App::use_signal(|| false),
        menubar_parts_open: App::use_signal(|| false),
        command_query: App::use_signal(String::new),
        combobox_open: App::use_signal(|| false),
        combobox_value: App::use_signal(|| String::from(FORM_COMBOBOX_FIRST_VALUE)),
        sheet_open: App::use_signal(|| false),
        alert_open: App::use_signal(|| false),
        hover_open: App::use_signal(|| false),
        context_open: App::use_signal(|| false),
        otp_value: App::use_signal(String::new),
        carousel_active: App::use_signal(|| 0),
    }
}

/// Builds the handler that moves the radio selection.
///
/// # Arguments
///
/// - `FormDemoState` - The signals the page drives.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(&'static str)>>` - The select handler of the radio group.
pub(crate) fn ui_forms_on_radio_select(state: FormDemoState) -> Option<Rc<dyn Fn(&'static str)>> {
    Some(Rc::new(move |value: &'static str| {
        state.radio_first.set(value == FORM_RADIO_FIRST_VALUE);
        state.radio_second.set(value == FORM_RADIO_SECOND_VALUE);
    }))
}

/// Builds the handler that closes the menubar after a row is chosen.
///
/// # Arguments
///
/// - `FormDemoState` - The signals the page drives.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(&'static str)>>` - The select handler of the menubar.
pub(crate) fn ui_forms_on_menubar_select(state: FormDemoState) -> Option<Rc<dyn Fn(&'static str)>> {
    Some(Rc::new(move |value: &'static str| {
        state.menubar_open.set(false);
        Console::log(format!("{FORM_MENUBAR_LOG} {value}"));
    }))
}

/// Builds the callback that closes the second menu after selection.
///
/// # Arguments
///
/// - `FormDemoState` - A `FormDemoState` parameter.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(&'static str)>>` - The callback that closes the edit menu.
pub(crate) fn ui_forms_on_menubar_edit_select(
    state: FormDemoState,
) -> Option<Rc<dyn Fn(&'static str)>> {
    Some(Rc::new(move |value: &'static str| {
        state.menubar_edit_open.set(false);
        Console::log(format!("{FORM_MENUBAR_LOG} {value}"));
    }))
}

/// Builds the callback that reports a selection in the component-parts menubar.
///
/// # Arguments
///
/// - `FormDemoState` - The signals the page drives.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(&'static str)>>` - The menu-parts selection callback.
pub(crate) fn ui_forms_on_menubar_parts_select(
    state: FormDemoState,
) -> Option<Rc<dyn Fn(&'static str)>> {
    Some(Rc::new(move |value: &'static str| {
        state.menubar_parts_open.set(false);
        Console::log(format!("{FORM_MENUBAR_LOG} {value}"));
    }))
}

/// Builds the callback that updates the Combobox selection signal.
///
/// # Arguments
///
/// - `FormDemoState` - The signals the page drives.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(&'static str)>>` - The Combobox selection callback.
pub(crate) fn ui_forms_on_combobox_select(
    state: FormDemoState,
) -> Option<Rc<dyn Fn(&'static str)>> {
    Some(Rc::new(move |value: &'static str| {
        state.combobox_value.set(String::from(value));
    }))
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

/// Builds the handler that reports the open accordion sections.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Vec<String>)>>` - The change handler of the accordion.
pub(crate) fn ui_forms_on_accordion_change() -> Option<Rc<dyn Fn(Vec<String>)>> {
    Some(Rc::new(move |values: Vec<String>| {
        Console::log(FORM_ACCORDION_LOG.replace(FORM_LOG_VALUE_PLACEHOLDER, &values.join(", ")));
    }))
}
