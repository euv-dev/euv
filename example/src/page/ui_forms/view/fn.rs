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
    let toggle_values: Signal<Vec<String>> = state.toggle_values;
    let toggle_pressed: Signal<bool> = state.toggle_pressed;
    let radio_first: Signal<bool> = state.radio_first;
    let radio_second: Signal<bool> = state.radio_second;
    let menubar_open: Signal<bool> = state.menubar_open;
    let on_radio_select: Option<Rc<dyn Fn(&'static str)>> = ui_forms_on_radio_select(state.clone());
    let on_menubar_select: Option<Rc<dyn Fn(&'static str)>> =
        ui_forms_on_menubar_select(state.clone());
    let command_query: Signal<String> = state.command_query;
    let combobox_open: Signal<bool> = state.combobox_open;
    let combobox_value: Signal<String> = state.combobox_value;
    let sheet_open: Signal<bool> = state.sheet_open;
    let alert_open: Signal<bool> = state.alert_open;
    let hover_open: Signal<bool> = state.hover_open;
    let context_open: Signal<bool> = state.context_open;
    let otp_value: Signal<String> = state.otp_value;
    let field_disabled: Signal<bool> = state.field_disabled;
    let notes: Signal<String> = state.notes;
    let group_query: Signal<String> = state.group_query;
    let accordion_open: Signal<Vec<String>> = state.accordion_open;
    let accordion_items: Vec<EuvAccordionItem> = vec![
        EuvAccordionItem {
            value: FORM_ACCORDION_FIRST_VALUE,
            trigger: FORM_ACCORDION_FIRST_TRIGGER,
            content: FORM_ACCORDION_FIRST_CONTENT,
            disabled: false,
        },
        EuvAccordionItem {
            value: FORM_ACCORDION_SECOND_VALUE,
            trigger: FORM_ACCORDION_SECOND_TRIGGER,
            content: FORM_ACCORDION_SECOND_CONTENT,
            disabled: false,
        },
        EuvAccordionItem {
            value: FORM_ACCORDION_THIRD_VALUE,
            trigger: FORM_ACCORDION_THIRD_TRIGGER,
            content: FORM_ACCORDION_THIRD_CONTENT,
            disabled: true,
        },
    ];
    let toggle_items: Vec<EuvToggleGroupItem> = vec![
        EuvToggleGroupItem {
            value: FORM_TOGGLE_FIRST_VALUE,
            label: FORM_TOGGLE_FIRST_LABEL,
            disabled: false,
        },
        EuvToggleGroupItem {
            value: FORM_TOGGLE_SECOND_VALUE,
            label: FORM_TOGGLE_SECOND_LABEL,
            disabled: false,
        },
        EuvToggleGroupItem {
            value: FORM_TOGGLE_THIRD_VALUE,
            label: FORM_TOGGLE_THIRD_LABEL,
            disabled: false,
        },
    ];
    let chart_config: Vec<(&'static str, &'static str)> = vec![
        (FORM_CHART_FIRST_SERIES, FORM_CHART_FIRST_COLOR),
        (FORM_CHART_SECOND_SERIES, FORM_CHART_SECOND_COLOR),
    ];
    let collapsible_open: Signal<bool> = App::use_signal(|| false);
    let trace_open: Signal<bool> = App::use_signal(|| false);
    let navigation_open: Signal<bool> = App::use_signal(|| false);
    let resizable_size: Signal<f64> = App::use_signal(|| 50.0);
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
            euv_card {
                title: FORM_OVERLAY_CARD_TITLE
                euv_command {
                    value: command_query
                    label: FORM_COMMAND_LABEL
                    euv_command_input {
                        value: command_query
                        placeholder: FORM_COMMAND_PLACEHOLDER
                        disabled: false
                    }
                    euv_command_list {
                        label: FORM_COMMAND_LIST_LABEL
                        euv_command_group {
                            heading: FORM_COMMAND_GROUP_HEADING
                            euv_command_item {
                                value: FORM_COMMAND_FIRST_VALUE
                                label: FORM_COMMAND_FIRST_LABEL
                                selected: false
                                disabled: false
                                on_select: on_menubar_select.clone()
                            }
                            euv_command_shortcut {
                                text: FORM_COMMAND_FIRST_SHORTCUT
                            }
                        }
                        euv_command_separator {
                            label: ""
                        }
                        euv_command_empty {
                            text: FORM_COMMAND_EMPTY_TEXT
                        }
                    }
                }
                euv_combobox {
                    open: combobox_open
                    value: combobox_value
                    label: FORM_COMBOBOX_LABEL
                    euv_combobox_trigger {
                        open: combobox_open
                        label: FORM_COMBOBOX_TRIGGER_LABEL
                        disabled: false
                    }
                    euv_combobox_input {
                        value: combobox_value
                        placeholder: FORM_COMBOBOX_PLACEHOLDER
                        disabled: false
                    }
                    euv_combobox_content {
                        open: combobox_open
                        euv_combobox_list {
                            label: FORM_COMBOBOX_LIST_LABEL
                            euv_combobox_item {
                                value: FORM_COMBOBOX_FIRST_VALUE
                                label: FORM_COMBOBOX_FIRST_LABEL
                                selected: true
                                disabled: false
                                on_select: on_menubar_select.clone()
                            }
                            euv_combobox_item {
                                value: FORM_COMBOBOX_SECOND_VALUE
                                label: FORM_COMBOBOX_SECOND_LABEL
                                selected: false
                                disabled: false
                                on_select: on_menubar_select.clone()
                            }
                        }
                    }
                }
                euv_sheet {
                    open: sheet_open
                    side: EuvSheetSide::Right
                    euv_sheet_trigger {
                        open: sheet_open
                        label: FORM_SHEET_TRIGGER_LABEL
                    }
                    euv_sheet_overlay {
                        open: sheet_open
                    }
                    euv_sheet_content {
                        open: sheet_open
                        side: EuvSheetSide::Right
                        euv_sheet_header {
                            label: FORM_SHEET_TITLE
                            euv_sheet_title {
                                text: FORM_SHEET_TITLE
                            }
                            euv_sheet_description {
                                text: FORM_SHEET_DESCRIPTION
                            }
                        }
                        euv_sheet_footer {
                            label: FORM_SHEET_TITLE
                            euv_sheet_close {
                                label: FORM_ALERT_CANCEL_LABEL
                            }
                        }
                    }
                }
                euv_alert_dialog {
                    open: alert_open
                    euv_alert_dialog_trigger {
                        open: alert_open
                        label: FORM_ALERT_TRIGGER_LABEL
                    }
                    euv_alert_dialog_overlay {
                        open: alert_open
                    }
                    euv_alert_dialog_content {
                        open: alert_open
                        euv_alert_dialog_header {
                            label: FORM_ALERT_TITLE
                            euv_alert_dialog_title {
                                text: FORM_ALERT_TITLE
                            }
                            euv_alert_dialog_description {
                                text: FORM_ALERT_DESCRIPTION
                            }
                        }
                        euv_alert_dialog_footer {
                            label: FORM_ALERT_TITLE
                            euv_alert_dialog_cancel {
                                label: FORM_ALERT_CANCEL_LABEL
                            }
                            euv_alert_dialog_action {
                                label: FORM_ALERT_ACTION_LABEL
                            }
                        }
                    }
                }
                euv_hover_card {
                    open: hover_open
                    euv_hover_card_trigger {
                        open: hover_open
                        label: FORM_HOVER_CARD_LABEL
                    }
                    euv_hover_card_content {
                        open: hover_open
                        euv_dialog_title {
                            text: FORM_HOVER_CARD_TITLE
                        }
                        p {
                            FORM_HOVER_CARD_BODY
                        }
                    }
                }
                euv_context_menu {
                    open: context_open
                    euv_context_menu_trigger {
                        open: context_open
                        label: FORM_CONTEXT_MENU_LABEL
                    }
                    euv_context_menu_content {
                        open: context_open
                        euv_context_menu_item {
                            value: FORM_CONTEXT_FIRST_VALUE
                            label: FORM_CONTEXT_FIRST_LABEL
                            disabled: false
                            on_select: on_menubar_select.clone()
                        }
                        euv_context_menu_separator {
                            label: ""
                        }
                        euv_context_menu_shortcut {
                            text: FORM_COMMAND_FIRST_SHORTCUT
                        }
                    }
                }
                euv_input_otp {
                    value: otp_value
                    length: 6
                    disabled: false
                    label: FORM_OTP_LABEL
                }
                euv_separator {
                    orientation: EuvSeparatorOrientation::Horizontal
                    decorative: true
                }
            }
            euv_card {
                title: FORM_MENU_CARD_TITLE
                div {
                    class: c_form_label()
                    FORM_RADIO_GROUP_LABEL
                }
                euv_radio_group {
                    label: FORM_RADIO_GROUP_LABEL
                    euv_radio_group_item {
                        label: FORM_RADIO_FIRST_LABEL
                        value: FORM_RADIO_FIRST_VALUE
                        checked: radio_first
                        on_select: on_radio_select.clone()
                    }
                    euv_radio_group_item {
                        label: FORM_RADIO_SECOND_LABEL
                        value: FORM_RADIO_SECOND_VALUE
                        checked: radio_second
                        on_select: on_radio_select.clone()
                    }
                }
                div {
                    class: c_euv_menu_panel()
                    role: "menu"
                    euv_dropdown_menu_label {
                        text: FORM_MENU_LABEL
                    }
                    euv_dropdown_menu_item {
                        label: FORM_MENU_FIRST_LABEL
                        shortcut: FORM_MENU_FIRST_SHORTCUT
                        value: FORM_MENU_FIRST_LABEL
                        disabled: false
                        on_select: on_menubar_select.clone()
                    }
                    euv_dropdown_menu_separator {
                        label: ""
                    }
                    euv_dropdown_menu_item {
                        label: FORM_MENU_SECOND_LABEL
                        shortcut: FORM_MENU_SECOND_SHORTCUT
                        value: FORM_MENU_SECOND_LABEL
                        disabled: false
                        on_select: on_menubar_select.clone()
                    }
                }
                euv_menubar {
                    label: FORM_MENUBAR_TRIGGER_LABEL
                    euv_menubar_menu {
                        open: menubar_open
                        euv_menubar_trigger {
                            label: FORM_MENUBAR_TRIGGER_LABEL
                            open: menubar_open
                        }
                        euv_menubar_content {
                            open: menubar_open
                            euv_menubar_item {
                                label: FORM_MENUBAR_ITEM_LABEL
                                value: FORM_MENUBAR_ITEM_VALUE
                                on_select: on_menubar_select.clone()
                            }
                        }
                    }
                }
                euv_toast {
                    variant: EuvToastVariant::Default
                    title: FORM_TOAST_TITLE
                    description: FORM_TOAST_DESCRIPTION
                    action_label: FORM_TOAST_ACTION_LABEL
                    dismissible: true
                }
            }
            euv_card {
                title: FORM_TOGGLE_CARD_TITLE
                p {
                    class: c_badge_hint()
                    FORM_TOGGLE_GROUP_LABEL
                }
                euv_toggle_group {
                    items: toggle_items
                    values: toggle_values
                    variant: EuvToggleVariant::Outline
                    size: EuvToggleSize::Default
                }
                euv_toggle {
                    pressed: toggle_pressed
                    variant: EuvToggleVariant::Default
                    size: EuvToggleSize::Default
                    label: FORM_TOGGLE_BOLD_LABEL
                }
                p {
                    class: c_badge_hint()
                    FORM_KBD_HINT
                }
                euv_kbd {
                    text: FORM_KBD_KEY
                }
            }
            euv_card {
                title: FORM_ACCORDION_CARD_TITLE
                euv_accordion {
                    items: accordion_items
                    open_values: accordion_open
                    accordion_type: EuvAccordionType::Single
                    collapsible: true
                    disabled: false
                    on_change: ui_forms_on_accordion_change()
                }
            }
            euv_card {
                title: FORM_FIELD_CARD_TITLE
                euv_label {
                    html_for: FORM_NOTES_FIELD_ID
                    text: FORM_NOTES_LABEL
                }
                euv_textarea {
                    id: FORM_NOTES_FIELD_ID
                    name: FORM_NOTES_FIELD_ID
                    placeholder: FORM_NOTES_PLACEHOLDER
                    value: notes
                    rows: 4
                    disabled: field_disabled
                }
                euv_label {
                    html_for: FORM_QUERY_FIELD_ID
                    text: FORM_QUERY_LABEL
                }
                euv_input_group {
                    euv_input_group_addon {
                        align: EuvInputGroupAlign::InlineStart
                        FORM_QUERY_ADDON
                    }
                    euv_input_group_input {
                        id: FORM_QUERY_FIELD_ID
                        placeholder: FORM_QUERY_PLACEHOLDER
                        value: group_query
                        disabled: field_disabled
                    }
                    euv_input_group_button {
                        size: EuvInputGroupButtonSize::Xs
                        label: FORM_QUERY_BUTTON_LABEL
                    }
                }
            }
            euv_card {
                title: FORM_BUTTON_GROUP_CARD_TITLE
                euv_button_group {
                    orientation: EuvButtonGroupOrientation::Horizontal
                    euv_button {
                        variant: EuvButtonVariant::Outline
                        label: FORM_BUTTON_GROUP_FIRST_LABEL
                    }
                    euv_button_group_separator {
                        orientation: EuvButtonGroupSeparatorOrientation::Vertical
                    }
                    euv_button {
                        variant: EuvButtonVariant::Outline
                        label: FORM_BUTTON_GROUP_SECOND_LABEL
                    }
                    euv_button_group_separator {
                        orientation: EuvButtonGroupSeparatorOrientation::Vertical
                    }
                    euv_button {
                        variant: EuvButtonVariant::Outline
                        label: FORM_BUTTON_GROUP_THIRD_LABEL
                    }
                }
                euv_button_group {
                    orientation: EuvButtonGroupOrientation::Vertical
                    euv_button_group_text {
                        text: FORM_BUTTON_GROUP_TEXT
                    }
                    euv_button {
                        variant: EuvButtonVariant::Outline
                        label: FORM_BUTTON_GROUP_FIRST_LABEL
                    }
                    euv_button {
                        variant: EuvButtonVariant::Outline
                        label: FORM_BUTTON_GROUP_SECOND_LABEL
                    }
                }
            }
            euv_card {
                title: FORM_CONTENT_CARD_TITLE
                euv_marker {
                    variant: EuvMarkerVariant::Default
                    euv_marker_content {
                        FORM_MARKER_LABEL
                    }
                }
                euv_marker {
                    variant: EuvMarkerVariant::Separator
                    euv_marker_content {
                        FORM_MARKER_SEPARATOR_LABEL
                    }
                }
                euv_marker {
                    variant: EuvMarkerVariant::Border
                    euv_marker_content {
                        FORM_MARKER_BORDER_LABEL
                    }
                }
                euv_native_select {
                    size: EuvNativeSelectSize::Default
                    name: FORM_SELECT_NAME
                    disabled: false
                    euv_native_select_option {
                        value: FORM_SELECT_FIRST_VALUE
                        label: FORM_SELECT_FIRST_LABEL
                    }
                    euv_native_select_option {
                        value: FORM_SELECT_SECOND_VALUE
                        label: FORM_SELECT_SECOND_LABEL
                    }
                    euv_native_select_option {
                        value: FORM_SELECT_THIRD_VALUE
                        label: FORM_SELECT_THIRD_LABEL
                    }
                }
                euv_attachment_group {
                    euv_attachment {
                        state: EuvAttachmentState::Done
                        size: EuvAttachmentSize::Default
                        orientation: EuvAttachmentOrientation::Horizontal
                        euv_attachment_media {
                            variant: EuvAttachmentMediaVariant::Icon
                            size: EuvAttachmentSize::Default
                        }
                        euv_attachment_content {
                            euv_attachment_title {
                                FORM_ATTACHMENT_DONE_TITLE
                            }
                            euv_attachment_description {
                                FORM_ATTACHMENT_DONE_DESCRIPTION
                            }
                        }
                    }
                    euv_attachment {
                        state: EuvAttachmentState::Uploading
                        size: EuvAttachmentSize::Default
                        orientation: EuvAttachmentOrientation::Horizontal
                        euv_attachment_media {
                            variant: EuvAttachmentMediaVariant::Icon
                            size: EuvAttachmentSize::Default
                        }
                        euv_attachment_content {
                            euv_attachment_title {
                                FORM_ATTACHMENT_UPLOADING_TITLE
                            }
                            euv_attachment_description {
                                FORM_ATTACHMENT_UPLOADING_DESCRIPTION
                            }
                        }
                    }
                    euv_attachment {
                        state: EuvAttachmentState::Error
                        size: EuvAttachmentSize::Default
                        orientation: EuvAttachmentOrientation::Horizontal
                        euv_attachment_media {
                            variant: EuvAttachmentMediaVariant::Icon
                            size: EuvAttachmentSize::Default
                        }
                        euv_attachment_content {
                            euv_attachment_title {
                                FORM_ATTACHMENT_ERROR_TITLE
                            }
                            euv_attachment_description {
                                FORM_ATTACHMENT_ERROR_DESCRIPTION
                            }
                        }
                    }
                }
                euv_chart {
                    id: "chart-usage"
                    config: chart_config
                    euv_chart_tooltip {
                        euv_chart_tooltip_label {
                            FORM_CHART_TOOLTIP_LABEL
                        }
                        euv_chart_tooltip_item {
                            name: FORM_CHART_FIRST_SERIES
                            value: FORM_CHART_FIRST_VALUE
                            color: FORM_CHART_FIRST_COLOR
                        }
                        euv_chart_tooltip_item {
                            name: FORM_CHART_SECOND_SERIES
                            value: FORM_CHART_SECOND_VALUE
                            color: FORM_CHART_SECOND_COLOR
                        }
                    }
                    euv_chart_legend {
                        euv_chart_legend_item {
                            label: FORM_CHART_FIRST_SERIES
                            color: FORM_CHART_FIRST_COLOR
                        }
                        euv_chart_legend_item {
                            label: FORM_CHART_SECOND_SERIES
                            color: FORM_CHART_SECOND_COLOR
                        }
                    }
                }
            }
            euv_card {
                title: FORM_CHAT_CARD_TITLE
                euv_collapsible {
                    open: collapsible_open
                    euv_collapsible_trigger {
                        open: collapsible_open
                        FORM_COLLAPSIBLE_TRIGGER
                    }
                    euv_collapsible_content {
                        open: collapsible_open
                        p {
                            FORM_COLLAPSIBLE_LABEL
                        }
                        p {
                            FORM_COLLAPSIBLE_BODY
                        }
                    }
                }
                euv_bubble_group {
                    euv_bubble {
                        variant: EuvBubbleVariant::Default
                        align: EuvBubbleAlign::Start
                        euv_bubble_content {
                            FORM_BUBBLE_FIRST
                        }
                        euv_bubble_reactions {
                            side: EuvBubbleSide::Bottom
                            align: EuvBubbleAlign::Start
                            FORM_BUBBLE_REACTION
                        }
                    }
                    euv_bubble {
                        variant: EuvBubbleVariant::Tinted
                        align: EuvBubbleAlign::End
                        euv_bubble_content {
                            FORM_BUBBLE_SECOND
                        }
                    }
                }
                euv_message_scroller {
                    euv_message_scroller_viewport {
                        euv_message_scroller_content {
                            euv_message_scroller_item {
                                scroll_anchor: false
                                p {
                                    FORM_BUBBLE_FIRST
                                }
                            }
                            euv_message_scroller_item {
                                scroll_anchor: true
                                p {
                                    FORM_BUBBLE_SECOND
                                }
                            }
                        }
                    }
                    euv_message_scroller_button {
                        direction: EuvMessageScrollerDirection::End
                    }
                }
                euv_trace {
                    euv_trace_line {
                        status: Some(EuvTraceStatus::Running)
                        euv_trace_title {
                            FORM_TRACE_TITLE
                        }
                        euv_trace_description {
                            FORM_TRACE_FIRST_DETAIL
                        }
                    }
                    euv_trace_line {
                        status: Some(EuvTraceStatus::Error)
                        euv_trace_title {
                            FORM_TRACE_TITLE
                        }
                        euv_trace_description {
                            FORM_TRACE_SECOND_DETAIL
                        }
                    }
                    euv_trace_line {
                        status: Some(EuvTraceStatus::Pending)
                        euv_trace_title {
                            FORM_TRACE_TITLE
                        }
                        euv_trace_description {
                            FORM_TRACE_THIRD_DETAIL
                        }
                    }
                    euv_trace_collapsible {
                        open: trace_open
                        euv_trace_collapsible_trigger {
                            open: trace_open
                            FORM_COLLAPSIBLE_TRIGGER
                        }
                        euv_trace_collapsible_content {
                            open: trace_open
                            p {
                                FORM_COLLAPSIBLE_BODY
                            }
                        }
                    }
                }
                euv_questionnaire {
                    euv_questionnaire_progress {
                        "1 / 3"
                    }
                    euv_questionnaire_item {
                        euv_questionnaire_title {
                            FORM_QUESTIONNAIRE_TITLE
                        }
                        euv_questionnaire_description {
                            FORM_QUESTIONNAIRE_DESCRIPTION
                        }
                        euv_questionnaire_choices {
                            choice_type: EuvQuestionnaireChoiceType::Checkbox
                            euv_questionnaire_choice {
                                choice_type: EuvQuestionnaireChoiceType::Checkbox
                                checked: true
                                disabled: false
                                euv_questionnaire_choice_indicator {
                                    choice_type: EuvQuestionnaireChoiceType::Checkbox
                                    checked: true
                                }
                                euv_questionnaire_choice_label {
                                    FORM_QUESTIONNAIRE_FIRST
                                }
                            }
                            euv_questionnaire_choice {
                                choice_type: EuvQuestionnaireChoiceType::Checkbox
                                checked: false
                                disabled: false
                                euv_questionnaire_choice_indicator {
                                    choice_type: EuvQuestionnaireChoiceType::Checkbox
                                    checked: false
                                }
                                euv_questionnaire_choice_label {
                                    FORM_QUESTIONNAIRE_SECOND
                                }
                            }
                        }
                    }
                    euv_questionnaire_actions {
                        euv_questionnaire_skip {
                            FORM_QUESTIONNAIRE_SKIP
                        }
                        euv_questionnaire_next {
                            FORM_QUESTIONNAIRE_NEXT
                        }
                    }
                }
                euv_code_block {
                    variant: EuvCodeBlockVariant::Default
                    size: EuvCodeBlockSize::Default
                    euv_code_block_header {
                        euv_code_block_title {
                            FORM_CODE_BLOCK_TITLE
                        }
                        euv_code_block_actions {
                            euv_code_block_copy_button {
                                value: FORM_CODE_BLOCK_FIRST_LINE
                            }
                        }
                    }
                    euv_code_block_content {
                        wrap: false
                        line_numbers: true
                        start_line: 1
                        euv_code_block_line {
                            variant: EuvCodeBlockLineVariant::Default
                            FORM_CODE_BLOCK_FIRST_LINE
                        }
                        euv_code_block_line {
                            variant: EuvCodeBlockLineVariant::Highlighted
                            FORM_CODE_BLOCK_SECOND_LINE
                        }
                    }
                }
                euv_navigation_menu {
                    open: navigation_open
                    euv_navigation_menu_list {
                        euv_navigation_menu_item {
                            euv_navigation_menu_trigger {
                                open: navigation_open
                                FORM_NAVIGATION_LABEL
                            }
                            euv_navigation_menu_content {
                                open: navigation_open
                                euv_navigation_menu_link {
                                    FORM_NAVIGATION_FIRST_LINK
                                }
                                euv_navigation_menu_link {
                                    FORM_NAVIGATION_SECOND_LINK
                                }
                            }
                        }
                    }
                }
                euv_resizable_panel_group {
                    direction: EuvResizableDirection::Horizontal
                    euv_resizable_panel {
                        size: resizable_size
                        p {
                            FORM_RESIZABLE_FIRST
                        }
                    }
                    euv_resizable_handle {
                        size: resizable_size
                        min: 20.0
                        max: 80.0
                        direction: EuvResizableDirection::Horizontal
                    }
                    euv_resizable_panel {
                        size: resizable_size
                        p {
                            FORM_RESIZABLE_SECOND
                        }
                    }
                }
            }
        }
    }
}
