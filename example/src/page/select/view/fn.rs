use super::*;

/// A select demo page showcasing dropdown and cascading selections.
///
/// # Returns
///
/// - `VirtualNode` - The select demo page virtual DOM tree.
#[component]
pub(crate) fn page_select(node: VirtualNode<PageSelectProps>) -> VirtualNode {
    let PageSelectProps: PageSelectProps = node.try_get_props().unwrap_or_default();
    let state: UseSelect = use_select();
    html! {
        div {
            class: c_page_container()
            euv_header {
                icon: "📋"
                title: "Select & Textarea"
                subtitle: "Dropdown selection, cascading country-city selects, and textarea with character count validation."
            }
            euv_card {
                title: "Simple Select"
                div {
                    class: c_euv_input_wrapper()
                    label {
                        for: SELECT_FRUIT_ID
                        class: c_form_label()
                        "Choose a fruit"
                    }
                    select {
                        id: SELECT_FRUIT_ID
                        name: SELECT_FRUIT_NAME
                        autocomplete: SELECT_AUTOCOMPLETE_OFF
                        class: c_select_input()
                        value: state.get_selected_fruit()
                        onchange: UseEuvInput::on_change_value(state.get_selected_fruit())
                        option {
                            value: "apple"
                            "Apple"
                        }
                        option {
                            value: "banana"
                            "Banana"
                        }
                        option {
                            value: "cherry"
                            "Cherry"
                        }
                        option {
                            value: "durian"
                            "Durian"
                        }
                    }
                }
                p {
                    class: c_event_result()
                    "Selected: "
                    span {
                        class: c_event_highlight()
                        {
                            state.get_selected_fruit().get()
                        }
                    }
                }
            }
            euv_card {
                title: "Cascading Select"
                div {
                    class: c_euv_input_wrapper()
                    label {
                        for: SELECT_COUNTRY_ID
                        class: c_form_label()
                        "Country"
                    }
                    select {
                        id: SELECT_COUNTRY_ID
                        name: SELECT_COUNTRY_NAME
                        autocomplete: SELECT_AUTOCOMPLETE_COUNTRY
                        class: c_select_input()
                        onchange: select_on_country_change(state)
                        option {
                            value: ""
                            "-- Select Country --"
                        }
                        option {
                            value: "china"
                            "China"
                        }
                        option {
                            value: "japan"
                            "Japan"
                        }
                        option {
                            value: "usa"
                            "USA"
                        }
                    }
                }
                if { !state.get_selected_country().get().is_empty() } {
                    div {
                        class: c_euv_input_wrapper()
                        label {
                            for: SELECT_CITY_ID
                            class: c_form_label()
                            "City"
                        }
                        select {
                            id: SELECT_CITY_ID
                            name: SELECT_CITY_NAME
                            autocomplete: SELECT_AUTOCOMPLETE_OFF
                            class: c_select_input()
                            value: state.get_selected_city()
                            onchange: UseEuvInput::on_change_value(state.get_selected_city())
                            for (value, label) in { state.get_cities().get().iter() } {
                                option {
                                    value: value.clone()
                                    label.clone()
                                }
                            }
                        }
                    }
                }
                if { !state.get_selected_city().get().is_empty() } {
                    div {
                        class: c_success_box()
                        "You selected: "
                        span {
                            class: c_event_highlight()
                            state.get_selected_city().get()
                        }
                    }
                }
            }
            euv_card {
                title: "Textarea with Feedback"
                div {
                    class: c_euv_input_wrapper()
                    label {
                        for: SELECT_FEEDBACK_ID
                        class: c_form_label()
                        "Your feedback"
                    }
                    textarea {
                        id: SELECT_FEEDBACK_ID
                        name: SELECT_FEEDBACK_NAME
                        autocomplete: SELECT_AUTOCOMPLETE_OFF
                        class: if { state.get_textarea_error().get().is_empty() } {
                            c_textarea_input()
                        } else {
                            c_textarea_input_error()
                        }
                        placeholder: SELECT_FEEDBACK_PLACEHOLDER
                        value: state.get_textarea_content()
                        oninput: select_on_input_textarea(state)
                        rows: SELECT_FEEDBACK_ROWS
                    }
                    if { !state.get_textarea_error().get().is_empty() } {
                        p {
                            class: c_field_error_text()
                            state.get_textarea_error()
                        }
                    }
                }
                div {
                    class: c_textarea_counter()
                    span {
                        class: c_textarea_counter_text()
                        {
                            format!("{} / 200 characters", state.get_textarea_content().get().len())
                        }
                    }
                }
                div {
                    class: c_button_controls()
                    euv_button {
                        variant: EuvButtonVariant::Primary
                        label: "Submit"
                        onclick: select_on_submit_feedback(state)
                    }
                }
                if { !state.get_feedback().get().is_empty() } {
                    div {
                        class: c_success_box()
                        state.get_feedback()
                    }
                }
            }
        }
    }
}
