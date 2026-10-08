use super::*;

/// A dynamic tag demo page showcasing runtime tag switching.
///
/// Demonstrates the `{tag_expr} { content }` syntax for dynamic tags where
/// the tag name is determined at runtime based on a signal value. When the
/// signal changes, the dynamic tag automatically re-renders as either a
/// native HTML element or a user component.
/// # Arguments
///
/// - `VirtualNode<PageDynamicComponentProps>` - The component props node.
///
/// # Returns
///
/// - `VirtualNode` - The dynamic component demo page virtual DOM tree.
#[component]
pub(crate) fn page_dynamic_component(node: VirtualNode<PageDynamicComponentProps>) -> VirtualNode {
    let PageDynamicComponentProps: PageDynamicComponentProps =
        node.try_get_props().unwrap_or_default();
    let tag_name_opt: Signal<DynamicTagType> = App::use_signal(DynamicTagType::default);
    let content: Signal<String> = App::use_signal(|| "Hello, dynamic tag!".to_string());
    html! {
        div {
            class: c_page_container()
            euv_header {
                icon: "🏷️"
                title: "Dynamic Tag"
                subtitle: "Switch the element tag at runtime using the {tag} { content } syntax. Supports native HTML elements and user-defined components."
            }
            euv_card {
                title: "Tag Type Selection"
                p {
                    class: c_demo_text()
                    "Select a tag type below. Native HTML elements (div, span) and user components (euv_card, badge) are both supported."
                }
                div {
                    class: c_dynamic_component_tab_bar()
                    euv_button {
                        variant: if { tag_name_opt.get() == DynamicTagType::Div } {
                            EuvButtonVariant::Primary
                        } else {
                            EuvButtonVariant::Outline
                        }
                        label: DynamicTagType::Div.label()
                        onclick: tag_on_select(tag_name_opt, DynamicTagType::Div)
                    }
                    euv_button {
                        variant: if { tag_name_opt.get() == DynamicTagType::Span } {
                            EuvButtonVariant::Primary
                        } else {
                            EuvButtonVariant::Outline
                        }
                        label: DynamicTagType::Span.label()
                        onclick: tag_on_select(tag_name_opt, DynamicTagType::Span)
                    }
                    euv_button {
                        variant: if { tag_name_opt.get() == DynamicTagType::EuvCard } {
                            EuvButtonVariant::Primary
                        } else {
                            EuvButtonVariant::Outline
                        }
                        label: DynamicTagType::EuvCard.label()
                        onclick: tag_on_select(tag_name_opt, DynamicTagType::EuvCard)
                    }
                    euv_button {
                        variant: if { tag_name_opt.get() == DynamicTagType::Badge } {
                            EuvButtonVariant::Primary
                        } else {
                            EuvButtonVariant::Outline
                        }
                        label: DynamicTagType::Badge.label()
                        onclick: tag_on_select(tag_name_opt, DynamicTagType::Badge)
                    }
                }
            }
            euv_card {
                title: "Dynamic Tag Content"
                euv_input {
                    id: TAG_CONTENT_INPUT_ID
                    name: TAG_CONTENT_INPUT_ID
                    label: "Content Text"
                    placeholder: TAG_CONTENT_PLACEHOLDER
                    value: content
                    autocomplete: TAG_AUTOCOMPLETE_OFF
                    oninput: content_on_input(content)
                }
            }
            euv_card {
                title: "Result"
                p {
                    class: c_demo_text_muted()
                    format!("Current tag: {}", tag_name_opt.get())
                }
                div {
                    class: c_dynamic_component_panel()
                    {
                        tag_name_opt.get()
                    } {
                        title: "Dynamic euv_card"
                        onclick: badge_on_click("Dynamic Badge", LogLevel::Log)
                        {
                            content.get()
                        }
                    }
                }
            }
        }
    }
}
