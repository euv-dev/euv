use super::*;

/// Returns the city options for the given country code.
///
/// # Arguments
///
/// - `&str` - The country code.
///
/// # Returns
///
/// - `Vec<(String, String)>` - A list of (value, label) pairs for the cities.
pub(crate) fn get_cities_by_country(country: &str) -> Vec<(String, String)> {
    let empty_city: (String, String) = (String::new(), "-- Select City --".to_string());
    match country {
        "china" => vec![
            empty_city.clone(),
            ("beijing".to_string(), "Beijing".to_string()),
            ("shanghai".to_string(), "Shanghai".to_string()),
            ("guangzhou".to_string(), "Guangzhou".to_string()),
        ],
        "japan" => vec![
            empty_city.clone(),
            ("tokyo".to_string(), "Tokyo".to_string()),
            ("osaka".to_string(), "Osaka".to_string()),
            ("kyoto".to_string(), "Kyoto".to_string()),
        ],
        "usa" => vec![
            empty_city,
            ("new-york".to_string(), "New York".to_string()),
            ("los-angeles".to_string(), "Los Angeles".to_string()),
            ("chicago".to_string(), "Chicago".to_string()),
        ],
        _ => Vec::new(),
    }
}

/// Creates select demo state signals wrapped in a `UseSelect` struct.
///
/// # Returns
///
/// - `UseSelect` - The select state.
pub(crate) fn use_select() -> UseSelect {
    UseSelect::new(
        App::use_signal(|| "apple".to_string()),
        App::use_signal(String::new),
        App::use_signal(String::new),
        App::use_signal(Vec::new),
        App::use_signal(String::new),
        App::use_signal(String::new),
        App::use_signal(String::new),
    )
}

/// Validates the textarea content and updates the error signal.
///
/// # Arguments
///
/// - `UseSelect` - The select state.
pub(crate) fn validate_select_textarea(state: UseSelect) {
    let textarea_value: String = state.get_textarea_content().get();
    if textarea_value.trim().is_empty() {
        state
            .get_textarea_error()
            .set("Please enter some feedback.".to_string());
    } else if textarea_value.len() > 200 {
        state
            .get_textarea_error()
            .set("Feedback is too long (max 200 chars).".to_string());
    } else {
        state.get_textarea_error().set(String::new());
    }
}

/// Creates a change event handler that updates the country, resets the city,
/// and refreshes the city options list.
///
/// # Arguments
///
/// - `UseSelect` - The select state.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - A change handler for the country select.
pub(crate) fn select_on_country_change(state: UseSelect) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |event: Event| {
        if let Some(target) = event.target()
            && let Ok(country_select) = target.clone().dyn_into::<HtmlSelectElement>()
        {
            let country_value: String = country_select.value();
            state.get_selected_country().set(country_value.clone());
            state.get_selected_city().set(String::new());
            state
                .get_cities()
                .set(get_cities_by_country(&country_value));
        }
    }))
}

/// Creates an input event handler that updates the textarea content and validates it.
///
/// # Arguments
///
/// - `UseSelect` - The select state.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - An input handler.
pub(crate) fn select_on_input_textarea(state: UseSelect) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |event: Event| {
        if let Some(target) = event.target()
            && let Ok(textarea) = target.clone().dyn_into::<HtmlTextAreaElement>()
        {
            state.get_textarea_content().set(textarea.value());
        }
        validate_select_textarea(state);
    }))
}

/// Creates a click event handler that submits the textarea feedback.
///
/// # Arguments
///
/// - `UseSelect` - The select state.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - A click handler to submit feedback.
pub(crate) fn select_on_submit_feedback(state: UseSelect) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |_: Event| {
        validate_select_textarea(state);
        let textarea_error_value: String = state.get_textarea_error().get();
        if textarea_error_value.is_empty() {
            let content: String = state.get_textarea_content().get();
            state
                .get_feedback()
                .set(format!("Thank you for your feedback: \"{}\"", content));
            state.get_textarea_content().set(String::new());
        }
    }))
}
