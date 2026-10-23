use super::*;

/// A demo page for the form half of the component library.
///
/// The page puts the input components on one screen: a select, a
/// checkbox group, a segmented control, a number input, an
/// auto-complete, a date picker, and the form wrapper that ties them
/// together. Every control owns its selection through a signal, so the
/// form's submit handler can read the whole state back.
///
/// # Arguments
///
/// - `VirtualNode<PageUiFormsProps>` - The page component node
///   carrying the page props.
///
/// # Returns
///
/// - `VirtualNode` - The form demo page virtual DOM tree.
#[component]
pub(crate) fn page_ui_forms(node: VirtualNode<PageUiFormsProps>) -> VirtualNode {
    let PageUiFormsProps: PageUiFormsProps = node.try_get_props().unwrap_or_default();
    let state: FormDemoState = use_form_demo_state();
    let fruit_options: Vec<EuvSelectOption> = vec![
        EuvSelectOption {
            value: FORM_FRUIT_FIRST_VALUE,
            label: FORM_FRUIT_FIRST_LABEL,
            group: "",
            disabled: false,
        },
        EuvSelectOption {
            value: FORM_FRUIT_SECOND_VALUE,
            label: FORM_FRUIT_SECOND_LABEL,
            group: "",
            disabled: false,
        },
        EuvSelectOption {
            value: FORM_FRUIT_THIRD_VALUE,
            label: FORM_FRUIT_THIRD_LABEL,
            group: "",
            disabled: false,
        },
    ];
    let topping_options: Vec<EuvCheckboxGroupOption> = vec![
        EuvCheckboxGroupOption {
            value: FORM_TOPPING_FIRST_VALUE,
            label: FORM_TOPPING_FIRST_LABEL,
            disabled: false,
        },
        EuvCheckboxGroupOption {
            value: FORM_TOPPING_SECOND_VALUE,
            label: FORM_TOPPING_SECOND_LABEL,
            disabled: false,
        },
        EuvCheckboxGroupOption {
            value: FORM_TOPPING_THIRD_VALUE,
            label: FORM_TOPPING_THIRD_LABEL,
            disabled: false,
        },
    ];
    let view_options: Vec<EuvSegmentedOption> = vec![
        EuvSegmentedOption {
            value: FORM_VIEW_FIRST_VALUE,
            label: FORM_VIEW_FIRST_LABEL,
            disabled: false,
        },
        EuvSegmentedOption {
            value: FORM_VIEW_SECOND_VALUE,
            label: FORM_VIEW_SECOND_LABEL,
            disabled: false,
        },
        EuvSegmentedOption {
            value: FORM_VIEW_THIRD_VALUE,
            label: FORM_VIEW_THIRD_LABEL,
            disabled: false,
        },
    ];
    let search_options: Vec<EuvAutoCompleteOption> = vec![
        EuvAutoCompleteOption {
            value: FORM_SEARCH_FIRST_VALUE,
            label: FORM_SEARCH_FIRST_LABEL,
        },
        EuvAutoCompleteOption {
            value: FORM_SEARCH_SECOND_VALUE,
            label: FORM_SEARCH_SECOND_LABEL,
        },
        EuvAutoCompleteOption {
            value: FORM_SEARCH_THIRD_VALUE,
            label: FORM_SEARCH_THIRD_LABEL,
        },
    ];
    html! {
        div {
            class: c_page_container()
            euv_header {
                icon: "🎛️"
                title: FORM_HEADER_TITLE
                subtitle: FORM_HEADER_SUBTITLE
            }
            euv_form {
                submit_label: FORM_SUBMIT_LABEL
                on_submit: ui_forms_on_submit(state)
                euv_card {
                    title: FORM_CHOICE_CARD_TITLE
                    euv_select {
                        value: state.select_value
                        options: fruit_options
                        placeholder: FORM_SELECT_PLACEHOLDER
                        label: FORM_SELECT_LABEL
                        on_change: ui_forms_on_select()
                    }
                    euv_checkbox_group {
                        options: topping_options
                        values: state.checked_values
                        label: FORM_TOPPING_LABEL
                    }
                    euv_segmented {
                        options: view_options
                        value: state.segmented_value
                        on_change: ui_forms_on_segmented()
                    }
                }
                euv_card {
                    title: FORM_INPUT_CARD_TITLE
                    euv_number_input {
                        value: state.number_value
                        min: Some(FORM_QUANTITY_MIN)
                        max: Some(FORM_QUANTITY_MAX)
                        step: FORM_QUANTITY_STEP
                        label: FORM_QUANTITY_LABEL
                    }
                    euv_auto_complete {
                        options: search_options
                        value: state.query
                        placeholder: FORM_SEARCH_PLACEHOLDER
                        label: FORM_SEARCH_LABEL
                    }
                    euv_date_picker {
                        value: state.date_value
                        placeholder: FORM_DATE_PLACEHOLDER
                        label: FORM_DATE_LABEL
                    }
                }
            }
        }
    }
}
