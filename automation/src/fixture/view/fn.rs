use super::*;

/// Renders real euv controls and stable browser-test observation points.
///
/// # Arguments
///
/// - `VirtualNode<FixturePageProps>` - Lifecycle-managed fixture component node.
///
/// # Returns
///
/// - `VirtualNode` - Mounted automation fixture.
#[component]
pub(crate) fn fixture_page(node: VirtualNode<FixturePageProps>) -> VirtualNode {
    let FixturePageProps: FixturePageProps = node.try_get_props().unwrap_or_default();
    let state: FixtureState = fixture_state();
    let theme: ThemeState = fixture_theme();
    let modal_class: Signal<String> = fixture_modal_class(state.get_modal());
    html! {
        div {
            class: theme.get_root_class()
            main {
                id: ROOT_ID
                class: c_automation_root()
                onkeydown: keyboard_handler(state.get_keys())
                h1 {
                    FIXTURE_TITLE
                }
                span {
                    id: READY_ID READY_TEXT
                }
                euv_card {
                    title: COUNTER_TITLE
                    div {
                        id: INCREMENT_ID euv_button {
                            label: INCREMENT_LABEL
                            onclick: increment_handler(state)
                        }
                    }
                    span {
                        id: COUNT_ID state.get_count()
                    }
                    span {
                        id: TRUSTED_ID state.get_trusted()
                    }
                }
                euv_card {
                    title: INPUT_TITLE
                    euv_input {
                        id: INPUT_ID
                        label: INPUT_LABEL
                        value: state.get_text() oninput: input_handler(state)
                    }
                    span {
                        id: TEXT_ID state.get_text()
                    }
                    euv_checkbox {
                        id: CHECKBOX_ID
                        label: CHECKBOX_LABEL
                        checked: state.get_checked()
                    }
                    span {
                        id: CHECKED_ID state.get_checked()
                    }
                    span {
                        id: KEYS_ID state.get_keys()
                    }
                }
                euv_card {
                    title: MODAL_TITLE
                    div {
                        id: OPEN_ID euv_button {
                            label: OPEN_LABEL
                            onclick: modal_handler(state.get_modal(), true)
                        }
                    }
                }
                div {
                    id: DIALOG_ID
                    class: modal_class
                    euv_modal {
                        title: MODAL_TITLE
                        onclick: modal_handler(state.get_modal(), false)
                        p {
                            id: DIALOG_CONTENT_ID DIALOG_TEXT
                        }
                    }
                }
                euv_card {
                    title: SCROLL_TITLE
                    div {
                        id: SCROLLER_ID
                        class: c_automation_scroller()
                        onscroll: scroll_handler(state.get_scroll())
                        div {
                            class: c_automation_content() p {
                                SCROLL_CONTENT
                            }
                        }
                    }
                    span {
                        id: SCROLL_ID state.get_scroll()
                    }
                }
            }
        }
    }
}
