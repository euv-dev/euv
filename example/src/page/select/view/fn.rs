use super::*;

/// A select demo page showcasing dropdown and cascading selections.
/// # Arguments
///
/// - `VirtualNode<PageSelectProps>` - The component props node.
///
/// # Returns
///
/// - `VirtualNode` - The select demo page virtual DOM tree.
///
/// # Arguments
///
/// - `VirtualNode<PageSelectProps>` - The `node` argument.
///
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
                        for: "select-fruit"
                        class: c_form_label()
                        "Choose a fruit"
                    }
                    select {
                        id: "select-fruit"
                        name: "fruit"
                        autocomplete: "off"
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
                        for: "select-country"
                        class: c_form_label()
                        "Country"
                    }
                    select {
                        id: "select-country"
                        name: "country"
                        autocomplete: "country"
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
                            for: "select-city"
                            class: c_form_label()
                            "City"
                        }
                        select {
                            id: "select-city"
                            name: "city"
                            autocomplete: "off"
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
                        for: "select-feedback"
                        class: c_form_label()
                        "Your feedback"
                    }
                    textarea {
                        id: "select-feedback"
                        name: "feedback"
                        autocomplete: "off"
                        class: if { state.get_textarea_error().get().is_empty() } {
                            c_textarea_input()
                        } else {
                            c_textarea_input_error()
                        }
                        placeholder: "Share your thoughts..."
                        value: state.get_textarea_content()
                        oninput: select_on_input_textarea(state)
                        rows: "4"
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
