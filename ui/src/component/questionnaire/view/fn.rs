use super::*;

/// Holds a questionnaire.
///
/// The questionnaire is a column of questions. The progress above them
/// says how far along the reader is, and the actions below them move
/// from one question to the next.
///
/// # Arguments
///
/// - `VirtualNode<EuvQuestionnaireProps>` - The props node carrying the
///   settings of the questionnaire.
///
/// # Returns
///
/// - `VirtualNode` - The euv questionnaire virtual DOM tree.
#[component]
pub fn euv_questionnaire(node: VirtualNode<EuvQuestionnaireProps>) -> VirtualNode {
    let EuvQuestionnaireProps {}: EuvQuestionnaireProps = node.try_get_props().unwrap_or_default();
    let children: VirtualNode = node.get_children().into();
    let base_class: &Css = c_euv_questionnaire();
    html! {
            div {
                "data-slot": "questionnaire"
                class: base_class

                children
            }
    }
}

/// Says how far along the reader is.
///
/// The progress is a number, so it is set in figures that line up in a
/// column while the count changes.
///
/// # Arguments
///
/// - `VirtualNode<EuvQuestionnaireProgressProps>` - The props node
///   carrying the settings of the progress.
///
/// # Returns
///
/// - `VirtualNode` - The euv questionnaire progress virtual DOM tree.
#[component]
pub fn euv_questionnaire_progress(node: VirtualNode<EuvQuestionnaireProgressProps>) -> VirtualNode {
    let EuvQuestionnaireProgressProps {}: EuvQuestionnaireProgressProps =
        node.try_get_props().unwrap_or_default();
    let children: VirtualNode = node.get_children().into();
    html! {
            div {
                "data-slot": "questionnaire-progress"
                class: c_euv_questionnaire_progress()
                children
            }
    }
}

/// Holds one question.
///
/// # Arguments
///
/// - `VirtualNode<EuvQuestionnaireItemProps>` - The props node carrying
///   the settings of the item.
///
/// # Returns
///
/// - `VirtualNode` - The euv questionnaire item virtual DOM tree.
#[component]
pub fn euv_questionnaire_item(node: VirtualNode<EuvQuestionnaireItemProps>) -> VirtualNode {
    let EuvQuestionnaireItemProps {}: EuvQuestionnaireItemProps =
        node.try_get_props().unwrap_or_default();
    let children: VirtualNode = node.get_children().into();
    html! {
        div {
            "data-slot": "questionnaire-item"
            class: c_euv_questionnaire_item()
            children
        }
    }
}

/// Holds the question itself.
///
/// # Arguments
///
/// - `VirtualNode<EuvQuestionnaireTitleProps>` - The props node carrying
///   the settings of the title.
///
/// # Returns
///
/// - `VirtualNode` - The euv questionnaire title virtual DOM tree.
#[component]
pub fn euv_questionnaire_title(node: VirtualNode<EuvQuestionnaireTitleProps>) -> VirtualNode {
    let EuvQuestionnaireTitleProps {}: EuvQuestionnaireTitleProps =
        node.try_get_props().unwrap_or_default();
    let children: VirtualNode = node.get_children().into();
    let base_class: &Css = c_euv_questionnaire_title();
    html! {
            div {
                "data-slot": "questionnaire-title"
                class: base_class

                children
            }
    }
}

/// Holds the line under the question.
///
/// # Arguments
///
/// - `VirtualNode<EuvQuestionnaireDescriptionProps>` - The props node
///   carrying the settings of the description.
///
/// # Returns
///
/// - `VirtualNode` - The euv questionnaire description virtual DOM tree.
#[component]
pub fn euv_questionnaire_description(
    node: VirtualNode<EuvQuestionnaireDescriptionProps>,
) -> VirtualNode {
    let EuvQuestionnaireDescriptionProps {}: EuvQuestionnaireDescriptionProps =
        node.try_get_props().unwrap_or_default();
    let children: VirtualNode = node.get_children().into();
    html! {
            div {
                "data-slot": "questionnaire-description"
                class: c_euv_questionnaire_description()
                children
            }
    }
}

/// Holds the answers a question accepts.
///
/// The choices carry the kind of answer they take, so the indicator of
/// every choice beside them matches the kind of answer.
///
/// # Arguments
///
/// - `VirtualNode<EuvQuestionnaireChoicesProps>` - The props node
///   carrying the kind of answer the choices take.
///
/// # Returns
///
/// - `VirtualNode` - The euv questionnaire choices virtual DOM tree.
#[component]
pub fn euv_questionnaire_choices(node: VirtualNode<EuvQuestionnaireChoicesProps>) -> VirtualNode {
    let EuvQuestionnaireChoicesProps { choice_type }: EuvQuestionnaireChoicesProps =
        node.try_get_props().unwrap_or_default();
    let type_attr: &str = match choice_type {
        EuvQuestionnaireChoiceType::Radio => QUESTIONNAIRE_CHOICE_TYPE_RADIO,
        EuvQuestionnaireChoiceType::Checkbox => QUESTIONNAIRE_CHOICE_TYPE_CHECKBOX,
    };
    let children: VirtualNode = node.get_children().into();
    html! {
        div {
            "data-slot": "questionnaire-choices"
            "data-type": type_attr
            class: c_euv_questionnaire_choices()
            children
        }
    }
}

/// Holds one answer a question accepts.
///
/// The choice reports whether it is ticked and whether it refuses to be
/// picked.
///
/// # Arguments
///
/// - `VirtualNode<EuvQuestionnaireChoiceProps>` - The props node
///   carrying the kind of answer, the ticked state and the disabled
///   state of the choice.
///
/// # Returns
///
/// - `VirtualNode` - The euv questionnaire choice virtual DOM tree.
#[component]
pub fn euv_questionnaire_choice(node: VirtualNode<EuvQuestionnaireChoiceProps>) -> VirtualNode {
    let EuvQuestionnaireChoiceProps {
        choice_type,
        checked,
        disabled,
    }: EuvQuestionnaireChoiceProps = node.try_get_props().unwrap_or_default();
    let type_attr: &str = match choice_type {
        EuvQuestionnaireChoiceType::Radio => QUESTIONNAIRE_CHOICE_TYPE_RADIO,
        EuvQuestionnaireChoiceType::Checkbox => QUESTIONNAIRE_CHOICE_TYPE_CHECKBOX,
    };
    let children: VirtualNode = node.get_children().into();
    let base_class: &Css = c_euv_questionnaire_choice();
    html! {
            div {
                "data-slot": "questionnaire-choice"
                "data-type": type_attr
                "data-checked": if checked {
                    "true"
                } else {
                    "false"
                }
                "data-disabled": if disabled {
                    "true"
                } else {
                    "false"
                }
                class: base_class

                class: if disabled {
                    c_euv_questionnaire_choice_disabled()
                } else {
                    c_euv_questionnaire_choice_enabled()
                }
                input {
                    "data-slot": "questionnaire-choice-input"
                    type: if choice_type == EuvQuestionnaireChoiceType::Radio {
                        "radio"
                    } else {
                        "checkbox"
                    }
                    checked: checked
                    disabled: disabled
                    class: c_euv_questionnaire_choice_input()
                }
                children
            }
    }
}

/// Holds the label of an answer.
///
/// # Arguments
///
/// - `VirtualNode<EuvQuestionnaireChoiceLabelProps>` - The props node
///   carrying the settings of the label.
///
/// # Returns
///
/// - `VirtualNode` - The euv questionnaire choice label virtual DOM tree.
#[component]
pub fn euv_questionnaire_choice_label(
    node: VirtualNode<EuvQuestionnaireChoiceLabelProps>,
) -> VirtualNode {
    let EuvQuestionnaireChoiceLabelProps {}: EuvQuestionnaireChoiceLabelProps =
        node.try_get_props().unwrap_or_default();
    let children: VirtualNode = node.get_children().into();
    html! {
            span {
                "data-slot": "questionnaire-choice-label"
                class: c_euv_questionnaire_choice_label()
                children
            }
    }
}

/// Holds the line under the label of an answer.
///
/// # Arguments
///
/// - `VirtualNode<EuvQuestionnaireChoiceDescriptionProps>` - The props
///   node carrying the settings of the description.
///
/// # Returns
///
/// - `VirtualNode` - The euv questionnaire choice description virtual DOM
///   tree.
#[component]
pub fn euv_questionnaire_choice_description(
    node: VirtualNode<EuvQuestionnaireChoiceDescriptionProps>,
) -> VirtualNode {
    let EuvQuestionnaireChoiceDescriptionProps {}: EuvQuestionnaireChoiceDescriptionProps =
        node.try_get_props().unwrap_or_default();
    let children: VirtualNode = node.get_children().into();
    html! {
        span {
            "data-slot": "questionnaire-choice-description"
            class: c_euv_questionnaire_choice_description()
            children
        }
    }
}

/// Holds the mark that shows whether an answer is ticked.
///
/// The mark is a dot for the answer that takes one answer out of a list
/// and a tick for the answer that can be ticked on its own.
///
/// # Arguments
///
/// - `VirtualNode<EuvQuestionnaireChoiceIndicatorProps>` - The props node
///   carrying the kind of answer and the ticked state.
///
/// # Returns
///
/// - `VirtualNode` - The euv questionnaire choice indicator virtual DOM
///   tree.
#[component]
pub fn euv_questionnaire_choice_indicator(
    node: VirtualNode<EuvQuestionnaireChoiceIndicatorProps>,
) -> VirtualNode {
    let EuvQuestionnaireChoiceIndicatorProps {
        choice_type,
        checked,
    }: EuvQuestionnaireChoiceIndicatorProps = node.try_get_props().unwrap_or_default();
    let type_attr: &str = match choice_type {
        EuvQuestionnaireChoiceType::Radio => QUESTIONNAIRE_CHOICE_TYPE_RADIO,
        EuvQuestionnaireChoiceType::Checkbox => QUESTIONNAIRE_CHOICE_TYPE_CHECKBOX,
    };
    // The answer that takes one answer out of a list is marked with a
    // round mark, the one that can be ticked on its own with a square.
    let shape_class: fn() -> &'static Css = match choice_type {
        EuvQuestionnaireChoiceType::Radio => c_euv_questionnaire_choice_indicator_radio,
        EuvQuestionnaireChoiceType::Checkbox => c_euv_questionnaire_choice_indicator_checkbox,
    };
    let children: VirtualNode = node.get_children().into();
    html! {
        span {
            "data-slot": "questionnaire-choice-indicator"
            "data-type": type_attr
            "data-checked": if checked { "true" } else { "false" }
            "aria-hidden": "true"
            class: c_euv_questionnaire_choice_indicator()
            class: shape_class()
            class: if checked {
                c_euv_questionnaire_choice_indicator_checked()
            } else {
                c_euv_questionnaire_choice_indicator_unchecked()
            }
            span {
                "data-slot": "questionnaire-choice-indicator-dot"
                class: c_euv_questionnaire_choice_indicator_dot()
            }
            span {
                "data-slot": "questionnaire-choice-indicator-check"
                class: c_euv_questionnaire_choice_indicator_check()
            }
            children
        }
    }
}

/// Holds the free text answer a question accepts.
///
/// # Arguments
///
/// - `VirtualNode<EuvQuestionnaireInputProps>` - The props node carrying
///   the settings of the input.
///
/// # Returns
///
/// - `VirtualNode` - The euv questionnaire input virtual DOM tree.
#[component]
pub fn euv_questionnaire_input(node: VirtualNode<EuvQuestionnaireInputProps>) -> VirtualNode {
    let EuvQuestionnaireInputProps {}: EuvQuestionnaireInputProps =
        node.try_get_props().unwrap_or_default();
    let base_class: &Css = c_euv_questionnaire_input();
    html! {
        div {
            "data-slot": "questionnaire-input-wrapper"
            class: c_euv_questionnaire_input_wrapper()
            input {
                "data-slot": "questionnaire-input"
                type: "text"
                class: base_class

            }
        }
    }
}

/// Says that the answer is not accepted.
///
/// The error is announced as soon as it appears, so a reader who moves
/// on too early hears why the answer was refused.
///
/// # Arguments
///
/// - `VirtualNode<EuvQuestionnaireErrorProps>` - The props node carrying
///   the settings of the error.
///
/// # Returns
///
/// - `VirtualNode` - The euv questionnaire error virtual DOM tree.
#[component]
pub fn euv_questionnaire_error(node: VirtualNode<EuvQuestionnaireErrorProps>) -> VirtualNode {
    let EuvQuestionnaireErrorProps {}: EuvQuestionnaireErrorProps =
        node.try_get_props().unwrap_or_default();
    let children: VirtualNode = node.get_children().into();
    html! {
        div {
            "data-slot": "questionnaire-error"
            role: "alert"
            class: c_euv_questionnaire_error()
            children
        }
    }
}

/// Holds the buttons that move from one question to the next.
///
/// # Arguments
///
/// - `VirtualNode<EuvQuestionnaireActionsProps>` - The props node
///   carrying the settings of the actions.
///
/// # Returns
///
/// - `VirtualNode` - The euv questionnaire actions virtual DOM tree.
#[component]
pub fn euv_questionnaire_actions(node: VirtualNode<EuvQuestionnaireActionsProps>) -> VirtualNode {
    let EuvQuestionnaireActionsProps {}: EuvQuestionnaireActionsProps =
        node.try_get_props().unwrap_or_default();
    let children: VirtualNode = node.get_children().into();
    html! {
        div {
            "data-slot": "questionnaire-actions"
            class: c_euv_questionnaire_actions()
            children
        }
    }
}

/// Goes back to the previous question.
///
/// # Arguments
///
/// - `VirtualNode<EuvQuestionnairePreviousProps>` - The props node
///   carrying the settings of the button.
///
/// # Returns
///
/// - `VirtualNode` - The euv questionnaire previous virtual DOM tree.
#[component]
pub fn euv_questionnaire_previous(node: VirtualNode<EuvQuestionnairePreviousProps>) -> VirtualNode {
    let EuvQuestionnairePreviousProps {}: EuvQuestionnairePreviousProps =
        node.try_get_props().unwrap_or_default();
    let children: VirtualNode = node.get_children().into();
    let base_class: &Css = c_euv_questionnaire_previous();
    html! {
        button {
            "data-slot": "questionnaire-previous"
            type: "button"
            class: base_class

            children
        }
    }
}

/// Leaves the question without answering it.
///
/// # Arguments
///
/// - `VirtualNode<EuvQuestionnaireSkipProps>` - The props node carrying
///   the settings of the button.
///
/// # Returns
///
/// - `VirtualNode` - The euv questionnaire skip virtual DOM tree.
#[component]
pub fn euv_questionnaire_skip(node: VirtualNode<EuvQuestionnaireSkipProps>) -> VirtualNode {
    let EuvQuestionnaireSkipProps {}: EuvQuestionnaireSkipProps =
        node.try_get_props().unwrap_or_default();
    let children: VirtualNode = node.get_children().into();
    let base_class: &Css = c_euv_questionnaire_skip();
    html! {
        button {
            "data-slot": "questionnaire-skip"
            type: "button"
            class: base_class

            children
        }
    }
}

/// Goes on to the next question.
///
/// # Arguments
///
/// - `VirtualNode<EuvQuestionnaireNextProps>` - The props node carrying
///   the settings of the button.
///
/// # Returns
///
/// - `VirtualNode` - The euv questionnaire next virtual DOM tree.
#[component]
pub fn euv_questionnaire_next(node: VirtualNode<EuvQuestionnaireNextProps>) -> VirtualNode {
    let EuvQuestionnaireNextProps {}: EuvQuestionnaireNextProps =
        node.try_get_props().unwrap_or_default();
    let children: VirtualNode = node.get_children().into();
    let base_class: &Css = c_euv_questionnaire_next();
    html! {
        button {
            "data-slot": "questionnaire-next"
            type: "button"
            class: base_class

            children
        }
    }
}

/// Sends the answers of the questionnaire.
///
/// # Arguments
///
/// - `VirtualNode<EuvQuestionnaireSubmitProps>` - The props node carrying
///   the settings of the button.
///
/// # Returns
///
/// - `VirtualNode` - The euv questionnaire submit virtual DOM tree.
#[component]
pub fn euv_questionnaire_submit(node: VirtualNode<EuvQuestionnaireSubmitProps>) -> VirtualNode {
    let EuvQuestionnaireSubmitProps {}: EuvQuestionnaireSubmitProps =
        node.try_get_props().unwrap_or_default();
    let children: VirtualNode = node.get_children().into();
    let base_class: &Css = c_euv_questionnaire_submit();
    html! {
        button {
            "data-slot": "questionnaire-submit"
            type: "button"
            class: base_class

            children
        }
    }
}

/// Shows the keys that answer a question.
///
/// The shortcut sits at the end of the answer row. It never takes the click,
/// so a reader who presses the keys still answers the question.
///
/// # Arguments
///
/// - `VirtualNode<EuvQuestionnaireChoiceShortcutProps>` - The props node
///   carrying the keys the shortcut shows.
///
/// # Returns
///
/// - `VirtualNode` - The euv questionnaire choice shortcut virtual DOM tree.
#[component]
pub fn euv_questionnaire_choice_shortcut(
    node: VirtualNode<EuvQuestionnaireChoiceShortcutProps>,
) -> VirtualNode {
    let EuvQuestionnaireChoiceShortcutProps { keys }: EuvQuestionnaireChoiceShortcutProps =
        node.try_get_props().unwrap_or_default();
    html! {
        span {
            "data-slot": "questionnaire-choice-shortcut"
            class: c_euv_questionnaire_choice_shortcut()
            {
                keys
            }
        }
    }
}
